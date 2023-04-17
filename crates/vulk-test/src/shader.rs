use super::*;

pub(crate) struct Compiler<'a> {
    compiler: shaderc::Compiler,
    options: shaderc::CompileOptions<'a>,
}

#[derive(Clone, Copy)]
pub(crate) enum ShaderType {
    Vertex,
    Fragment,
    Compute,
}

impl Compiler<'_> {
    pub(crate) fn new() -> Result<Self> {
        use shaderc::CompileOptions;
        use shaderc::OptimizationLevel;
        use shaderc::SourceLanguage;
        use shaderc::SpirvVersion;
        use shaderc::TargetEnv;
        let compiler = shaderc::Compiler::new().context("Creating shader compiler")?;
        let mut options = CompileOptions::new().context("Creating shader compiler options")?;
        options.set_target_env(TargetEnv::Vulkan, vk::make_api_version(0, 1, 3, 0));
        options.set_optimization_level(OptimizationLevel::Performance);
        options.set_target_spirv(SpirvVersion::V1_6);
        options.set_source_language(SourceLanguage::GLSL);
        options.set_warnings_as_errors();
        Ok(Self { compiler, options })
    }

    pub(crate) fn compile(
        &self,
        code: impl AsRef<str>,
        shader_type: ShaderType,
    ) -> Result<Vec<u8>> {
        let shader_kind = match shader_type {
            ShaderType::Vertex => shaderc::ShaderKind::Vertex,
            ShaderType::Fragment => shaderc::ShaderKind::Fragment,
            ShaderType::Compute => shaderc::ShaderKind::Compute,
        };
        let shader = self.compiler.compile_into_spirv(
            code.as_ref(),
            shader_kind,
            "unused",
            "main",
            Some(&self.options),
        )?;
        if shader.get_num_warnings() > 0 {
            warn!("{}", shader.get_warning_messages());
        }
        Ok(shader.as_binary_u8().to_owned())
    }
}
