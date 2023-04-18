use super::*;

pub(crate) struct Compiler {
    compiler: shaderc::Compiler,
    includes: HashMap<String, String>,
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum ShaderType {
    Task,
    Mesh,
    Fragment,
    Compute,
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

    pub(crate) fn compile(
        &self,
        code: impl AsRef<str>,
        shader_type: ShaderType,
    ) -> Result<Vec<u8>> {
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
        Ok(shader.as_binary_u8().to_owned())
    }
}
