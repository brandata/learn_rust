use web_sys::*;
use web_sys::WebGlRenderingContext as GL;

fn compile_shader(
    gl: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String {
     let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Error creating chader"))?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl.get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false) |
    {
        Ok(shader)    
    } else {
        Err(gl.get_shader_info_log(&shader)
        .unwrap_or_else(|| String::from("Unable to get this shader info log")))
    }
}