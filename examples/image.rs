use blinds::traits::*;
use blinds::*;
use golem::{Context, GolemError};
use golem::attribute::{Attribute, ShaderType};
use golem::input::{Color, Uniforms, Vec2, Vec4, VertexBuilder, vec2, rgba};
use golem::program::{Attribute, ShaderDescription};

async fn app(window: Window, ctx: glow::Context, mut events: EventStream) -> Result<(), GolemError> {
    let mut ctx = Context::from_glow(ctx);

    let vertices = [
        // Position         Color
        -0.5, -0.5,         1.0, 0.0, 0.0, 1.0,
        0.5, -0.5,          0.0, 1.0, 0.0, 1.0,
        0.0, 0.5,           0.0, 0.0, 1.0, 1.0
    ];

    let shader = ctx.new_shader(ShaderDescription {
        vertex_input: &[
            Attribute::Vector(2, "vert_position"),
            Attribute::Vector(4, "vert_color"),
        ],
        fragment_input: &[ Attribute::Vector(4, "frag_color") ],
        uniforms: &[],
        vertex_shader: r#" void main() {
            gl_Position = vec4(vert_position, 0, 1);
            frag_color = vert_color;
        }"#,
        fragment_shader:
        r#" void main() {
            gl_FragColor = frag_color;
        }"#
    })?;

    let mut vb = ctx.new_vertex_buffer();
    let mut eb = ctx.new_element_buffer();
    vb.send_data(0, &vertices);
    eb.send_data(0, &[0, 1, 2]);

    let uniforms = Uniforms::new();
 
    while let Some(_) = events.next().await {
        ctx.clear(rgba(0.0, 0.0, 0.0, 0.0));
        ctx.draw(&shader, &vb, &eb, &uniforms, &[0..3]);
        window.present();
    }

    Ok(())
}

fn main() {
    blinds::run_gl(Settings::default(), |window, gfx, events| async move {
        app(window, gfx, events).await.unwrap()
    });
}
