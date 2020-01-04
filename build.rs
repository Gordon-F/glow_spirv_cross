#![cfg_attr(
    not(any(feature = "gl", feature = "webgl")),
    allow(dead_code, unused_extern_crates, unused_imports)
)]

use glsl_to_spirv::ShaderType;
use spirv_cross::{glsl, spirv};
use std::error::Error;
use std::io::Read;

#[cfg(any(feature = "gl", feature = "webgl"))]
fn main() -> Result<(), Box<dyn Error>> {
    let shader_gen_path = "assets/shaders/generated/";

    std::fs::create_dir_all(shader_gen_path)?;

    for entry in std::fs::read_dir("src/shaders")? {
        let entry = entry?;

        if entry.file_type()?.is_file() {
            let in_path = entry.path();

            let shader_type =
                in_path
                    .extension()
                    .and_then(|ext| match ext.to_string_lossy().as_ref() {
                        "vert" => Some(ShaderType::Vertex),
                        "frag" => Some(ShaderType::Fragment),
                        _ => None,
                    });

            if let Some(shader_type) = shader_type {
                let source = std::fs::read_to_string(&in_path)?;
                let file_name = in_path.file_name().unwrap().to_string_lossy();

                let mut compiled_file = glsl_to_spirv::compile(&source, shader_type)?;
                let mut compiled_bytes = Vec::new();
                compiled_file.read_to_end(&mut compiled_bytes)?;

                let out_path = format!("{}{}.spv", shader_gen_path, file_name);

                std::fs::write(&out_path, &compiled_bytes)?;

                let module = spirv::Module::from_words(words_from_bytes(&compiled_bytes));

                let glsl_compiler_opt = glsl::CompilerOptions {
                    #[cfg(feature = "webgl")]
                    version: glsl::Version::V3_00Es,
                    #[cfg(feature = "gl")]
                    version: glsl::Version::V4_10,
                    vertex: glsl::CompilerVertexOptions::default(),
                };

                let mut ast = spirv::Ast::<glsl::Target>::parse(&module).unwrap();
                ast.set_compiler_options(&glsl_compiler_opt).unwrap();

                let shader = ast.compile().unwrap();

                #[cfg(feature = "webgl")]
                let shader_prefix = "_es";
                #[cfg(feature = "gl")]
                let shader_prefix = "";

                let out_path = format!("{}{}{}.glsl", shader_gen_path, file_name, shader_prefix);

                std::fs::write(&out_path, &shader)?;
            }
        }
    }

    Ok(())
}

#[cfg(not(any(feature = "gl", feature = "webgl")))]
fn main() {
    println!("You need to specify graphics api feature [gl, webgl]");
}

#[allow(clippy::cast_ptr_alignment)]
fn words_from_bytes(buf: &[u8]) -> &[u32] {
    unsafe {
        std::slice::from_raw_parts(
            buf.as_ptr() as *const u32,
            buf.len() / std::mem::size_of::<u32>(),
        )
    }
}
