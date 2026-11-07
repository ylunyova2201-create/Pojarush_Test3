//! Android display scaling hook; native touch coordinates remain in surface pixels.
#[derive(Clone, Copy, Debug, Default)]
pub struct RenderScale;
pub fn set_scale(_scale: RenderScale) {}
pub fn apply(_window: &winit::window::Window, _renderer: &mut horizon_client::gfx::Renderer) -> Result<(), String> { Ok(()) }
