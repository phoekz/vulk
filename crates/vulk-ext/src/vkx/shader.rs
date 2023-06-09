use super::*;

#[derive(Clone, Copy, Debug)]
pub enum ShaderType {
    Task,
    Mesh,
    Fragment,
    Compute,
    Raygen,
    Miss,
    ClosestHit,
}

impl ShaderType {
    #[must_use]
    pub fn shader_stage(self) -> vk::ShaderStageFlagBits {
        match self {
            ShaderType::Task => vk::ShaderStageFlagBits::TaskEXT,
            ShaderType::Mesh => vk::ShaderStageFlagBits::MeshEXT,
            ShaderType::Fragment => vk::ShaderStageFlagBits::Fragment,
            ShaderType::Compute => vk::ShaderStageFlagBits::Compute,
            ShaderType::Raygen => vk::ShaderStageFlagBits::RaygenKHR,
            ShaderType::Miss => vk::ShaderStageFlagBits::MissKHR,
            ShaderType::ClosestHit => vk::ShaderStageFlagBits::ClosestHitKHR,
        }
    }

    #[must_use]
    pub fn next_shader_stage(self) -> Option<vk::ShaderStageFlagBits> {
        match self {
            ShaderType::Task => Some(vk::ShaderStageFlagBits::MeshEXT),
            ShaderType::Mesh => Some(vk::ShaderStageFlagBits::Fragment),
            ShaderType::Fragment
            | ShaderType::Compute
            | ShaderType::Raygen
            | ShaderType::Miss
            | ShaderType::ClosestHit => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ShaderBinary {
    ty: ShaderType,
    code: Vec<u8>,
    entry_point: CString,
}

impl ShaderBinary {
    #[must_use]
    pub fn shader_type(&self) -> ShaderType {
        self.ty
    }

    #[must_use]
    pub fn code_size(&self) -> usize {
        self.code.len()
    }

    #[must_use]
    pub fn p_code(&self) -> *const std::ffi::c_void {
        self.code.as_ptr().cast()
    }

    #[must_use]
    pub fn entry_point(&self) -> *const std::ffi::c_char {
        self.entry_point.as_ptr()
    }

    #[must_use]
    pub fn shader_module_create_info(&self) -> vk::ShaderModuleCreateInfo {
        vk::ShaderModuleCreateInfo {
            s_type: vk::StructureType::ShaderModuleCreateInfo,
            p_next: null(),
            flags: vk::ShaderModuleCreateFlags::empty(),
            code_size: self.code_size(),
            p_code: self.p_code().cast(),
        }
    }
}

pub struct ShaderCompiler {
    compiler: shaderc::Compiler,
    includes: HashMap<String, String>,
}

impl ShaderCompiler {
    pub fn new() -> Result<Self> {
        use shaderc::Compiler;
        let compiler = Compiler::new().context("Creating shader compiler")?;
        Ok(Self {
            compiler,
            includes: HashMap::new(),
        })
    }

    pub fn include(&mut self, source_name: impl AsRef<str>, content: impl AsRef<str>) {
        self.includes.insert(
            source_name.as_ref().to_string(),
            content.as_ref().to_string(),
        );
    }

    pub fn compile(
        &self,
        shader_type: ShaderType,
        input_file_name: impl AsRef<str>,
        entry_point_name: impl AsRef<str>,
        code: impl AsRef<str>,
    ) -> Result<ShaderBinary> {
        use shaderc::CompileOptions;
        use shaderc::OptimizationLevel;
        use shaderc::ResolvedInclude;
        use shaderc::SourceLanguage;
        use shaderc::SpirvVersion;
        use shaderc::TargetEnv;

        let shader_kind = match shader_type {
            ShaderType::Task => shaderc::ShaderKind::Task,
            ShaderType::Mesh => shaderc::ShaderKind::Mesh,
            ShaderType::Fragment => shaderc::ShaderKind::Fragment,
            ShaderType::Compute => shaderc::ShaderKind::Compute,
            ShaderType::Raygen => shaderc::ShaderKind::RayGeneration,
            ShaderType::Miss => shaderc::ShaderKind::Miss,
            ShaderType::ClosestHit => shaderc::ShaderKind::ClosestHit,
        };
        let mut options = CompileOptions::new().context("Creating shader compiler options")?;
        options.set_target_env(TargetEnv::Vulkan, vulk::REQUIRED_VULKAN_VERSION);
        options.set_optimization_level(OptimizationLevel::Performance);
        options.set_target_spirv(SpirvVersion::V1_6);
        options.set_source_language(SourceLanguage::GLSL);
        options.set_warnings_as_errors();
        options.set_include_callback(|source_name, _, _, _| -> shaderc::IncludeCallbackResult {
            let content = self.includes.get(source_name);
            if let Some(content) = content {
                Ok(ResolvedInclude {
                    resolved_name: source_name.to_owned(),
                    content: (*content).clone(),
                })
            } else {
                Err(format!("Unable to include source file {source_name}."))
            }
        });
        let shader = self.compiler.compile_into_spirv(
            code.as_ref(),
            shader_kind,
            input_file_name.as_ref(),
            entry_point_name.as_ref(),
            Some(&options),
        )?;
        if shader.get_num_warnings() > 0 {
            error!("{}", shader.get_warning_messages());
        }
        Ok(ShaderBinary {
            ty: shader_type,
            code: shader.as_binary_u8().to_owned(),
            entry_point: CString::new(entry_point_name.as_ref())?,
        })
    }
}

#[derive(Debug)]
pub struct ShaderCreateInfo<'a> {
    pub shader_binaries: &'a [ShaderBinary],
    pub set_layouts: &'a [vk::DescriptorSetLayout],
    pub push_constant_ranges: &'a [vk::PushConstantRange],
    pub specialization_info: Option<&'a vk::SpecializationInfo>,
}

#[derive(Debug)]
pub struct Shader {
    pub(super) stages: Vec<vk::ShaderStageFlagBits>,
    pub(super) shaders: Vec<vk::ShaderEXT>,
}

impl Shader {
    pub unsafe fn create(device: &Device, create_info: &ShaderCreateInfo<'_>) -> Result<Self> {
        let create_infos = create_info
            .shader_binaries
            .iter()
            .map(|binary| vk::ShaderCreateInfoEXT {
                s_type: vk::StructureType::ShaderCreateInfoEXT,
                p_next: null(),
                flags: vk::ShaderCreateFlagBitsEXT::LinkStageEXT.into(),
                stage: binary.ty.shader_stage(),
                next_stage: if let Some(next_stage) = binary.ty.next_shader_stage() {
                    next_stage.into()
                } else {
                    zeroed()
                },
                code_type: vk::ShaderCodeTypeEXT::SpirvEXT,
                code_size: binary.code_size(),
                p_code: binary.p_code(),
                p_name: binary.entry_point(),
                set_layout_count: create_info.set_layouts.len() as _,
                p_set_layouts: create_info.set_layouts.as_ptr(),
                push_constant_range_count: create_info.push_constant_ranges.len() as _,
                p_push_constant_ranges: create_info.push_constant_ranges.as_ptr(),
                p_specialization_info: if let Some(specialization_info) =
                    create_info.specialization_info
                {
                    specialization_info as *const _
                } else {
                    null()
                },
            })
            .collect::<Vec<_>>();
        let mut shaders = Vec::with_capacity(create_info.shader_binaries.len());
        device.create_shaders_ext(
            create_infos.len() as _,
            create_infos.as_ptr(),
            shaders.as_mut_ptr(),
        )?;
        shaders.set_len(create_info.shader_binaries.len());

        let stages = create_info
            .shader_binaries
            .iter()
            .map(|spirv| spirv.ty.shader_stage())
            .collect();

        Ok(Self { stages, shaders })
    }

    pub unsafe fn destroy(self, device: &Device) {
        for &shader in &self.shaders {
            device.destroy_shader_ext(shader);
        }
    }
}

impl Shader {
    pub unsafe fn bind(&self, device: &Device, cmd: vk::CommandBuffer) {
        device.cmd_bind_shaders_ext(
            cmd,
            self.stages.len() as _,
            self.stages.as_ptr(),
            self.shaders.as_ptr(),
        );
    }
}
