use anyhow::Result;
use raw_window_handle::HasRawWindowHandle;

#[cfg(feature = "vulkan")]
use crate::vulkan::VulkanRenderer;

pub enum Backend {
    Vulkan,
}

pub trait Render {
    fn render(
        &mut self,
        dimensions: &[u32; 2],
    ) -> Result<()>;
}

impl dyn Render {
    pub fn create_backend(
        backend: &Backend,
        window_handle: &impl HasRawWindowHandle,
        dimensions: &[u32; 2],
    ) -> Result<impl Render> {
        match backend {
            Backend::Vulkan => VulkanRenderer::new(window_handle, dimensions),
        }
    }
}
