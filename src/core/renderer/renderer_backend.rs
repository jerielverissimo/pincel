pub struct PlatformState;

pub enum RendererBackendType {
    RendererBackendTypeVulkan,
    RendererBackendTypeOpengl,
}

pub struct RendererBackend {
    plat_state: PlatformState,
    frame_number: u64,

    initialize:
        fn(backend: &RendererBackend, application_name: &str, plat_state: &PlatformState) -> bool,
    shutdown: fn(backend: &RendererBackend),
    resized: fn(backend: &RendererBackend, width: u16, height: u16),
    begin_frame: fn(backend: &RendererBackend, delta_time: f32) -> bool,
    end_frame: fn(backend: &RendererBackend, delta_time: f32) -> bool,
}

pub struct RenderPacket {
    delta_time: f32,
}
