use crate::sdl::ContextError;

mod sdl;
mod renderer;
mod bitmap;

fn main() -> Result<(), ContextError> {
    let mut context = sdl::Context::new()?;
    context.main_loop()?;
    Ok(())
}
