use super::*;

pub(crate) struct Compiler {
    compiler: shaderc::Compiler,
    includes: HashMap<String, String>,
}

impl Compiler {
    pub(crate) fn new() -> Result<Self> {
        use shaderc::Compiler;
        let compiler = Compiler::new().context("Creating shader compiler")?;
        Ok(Self {
            compiler,
            includes: HashMap::new(),
        })
    }

    pub(crate) fn include(&mut self, source_name: impl AsRef<str>, content: impl AsRef<str>) {
        self.includes.insert(
            source_name.as_ref().to_string(),
            content.as_ref().to_string(),
        );
    }

    pub(crate) fn compile(&self, shader_type: ShaderType, code: impl AsRef<str>) -> Result<SpirV> {
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
        };
        let mut options = CompileOptions::new().context("Creating shader compiler options")?;
        options.set_target_env(TargetEnv::Vulkan, vk::make_api_version(0, 1, 3, 0));
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
            "unused",
            "main",
            Some(&options),
        )?;
        if shader.get_num_warnings() > 0 {
            warn!("{}", shader.get_warning_messages());
        }
        Ok(SpirV {
            ty: shader_type,
            code: shader.as_binary_u8().to_owned(),
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ShaderType {
    Task,
    Mesh,
    Fragment,
    Compute,
}

impl ShaderType {
    pub fn shader_stage(self) -> vk::ShaderStageFlagBits {
        match self {
            ShaderType::Task => vk::ShaderStageFlagBits::TASK_EXT,
            ShaderType::Mesh => vk::ShaderStageFlagBits::MESH_EXT,
            ShaderType::Fragment => vk::ShaderStageFlagBits::FRAGMENT,
            ShaderType::Compute => vk::ShaderStageFlagBits::COMPUTE,
        }
    }

    pub fn next_shader_stage(self) -> vk::ShaderStageFlagBits {
        match self {
            ShaderType::Task => vk::ShaderStageFlagBits::MESH_EXT,
            ShaderType::Mesh => vk::ShaderStageFlagBits::FRAGMENT,
            ShaderType::Fragment | ShaderType::Compute => vk::ShaderStageFlagBits::empty(),
        }
    }
}

pub struct SpirV {
    pub ty: ShaderType,
    pub code: Vec<u8>,
}

pub struct Shader {
    stages: Vec<vk::ShaderStageFlagBits>,
    shaders: Vec<vk::ShaderEXT>,
}

impl Shader {
    pub unsafe fn create(
        Gpu { device, .. }: &Gpu,
        spirvs: &[SpirV],
        set_layouts: &[vk::DescriptorSetLayout],
        push_constant_ranges: &[vk::PushConstantRange],
        specialization_info: Option<&vk::SpecializationInfo>,
    ) -> Result<Self> {
        let create_infos = spirvs
            .iter()
            .map(|spirv| vk::ShaderCreateInfoEXT {
                s_type: vk::StructureType::ShaderCreateInfoEXT,
                p_next: null(),
                flags: vk::ShaderCreateFlagsEXT::LINK_STAGE_EXT,
                stage: spirv.ty.shader_stage(),
                next_stage: spirv.ty.next_shader_stage(),
                code_type: vk::ShaderCodeTypeEXT::SpirvEXT,
                code_size: spirv.code.len(),
                p_code: spirv.code.as_ptr().cast(),
                p_name: b"main\0".as_ptr().cast(),
                set_layout_count: set_layouts.len() as _,
                p_set_layouts: set_layouts.as_ptr(),
                push_constant_range_count: push_constant_ranges.len() as _,
                p_push_constant_ranges: push_constant_ranges.as_ptr(),
                p_specialization_info: if let Some(specialization_info) = specialization_info {
                    specialization_info as *const _
                } else {
                    null()
                },
            })
            .collect::<Vec<_>>();
        let mut shaders = Vec::with_capacity(spirvs.len());
        device.create_shaders_ext(
            create_infos.len() as _,
            create_infos.as_ptr(),
            shaders.as_mut_ptr(),
        )?;
        shaders.set_len(spirvs.len());

        let stages = spirvs.iter().map(|spirv| spirv.ty.shader_stage()).collect();

        Ok(Self { stages, shaders })
    }

    pub unsafe fn destroy(&self, Gpu { device, .. }: &Gpu) {
        for &shader in &self.shaders {
            device.destroy_shader_ext(shader);
        }
    }

    pub unsafe fn bind(&self, Gpu { device, .. }: &Gpu, cmd: vk::CommandBuffer) {
        device.cmd_bind_shaders_ext(
            cmd,
            self.stages.len() as _,
            self.stages.as_ptr(),
            self.shaders.as_ptr(),
        );
    }
}
