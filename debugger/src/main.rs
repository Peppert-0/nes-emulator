mod bitmap;
mod renderer;
mod sdl;

fn main() -> Result<(), sdl::ContextError> {
    let mut context = sdl::Context::new()?;
    context.main_loop()?;
    Ok(())
}
