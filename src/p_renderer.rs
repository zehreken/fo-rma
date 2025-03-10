use wgpu::Surface;
use winit::window::Window;

use crate::rendering_utils;

pub struct PRenderer<'a> {
    surface: Surface<'a>,
}

impl<'a> PRenderer<'a> {
    pub async fn new(window: &'a Window) -> Self {
        let size = window.inner_size();
        let (instance, surface) = rendering_utils::create_instance_and_surface(window);
        let adapter = rendering_utils::create_adapter(instance, &surface).await;

        Self { surface }
    }
}
