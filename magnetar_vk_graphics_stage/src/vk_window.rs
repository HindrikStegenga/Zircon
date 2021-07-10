use magnetar_engine::resource_system::Resource;

pub trait VkWindow {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
}

pub struct VkWindowRequestInfo {
    pub width: u32,
    pub height: u32,
    pub title: String,
}

impl Resource for Box<dyn VkWindow> {
    type ResourceRequestInfo = VkWindowRequestInfo;

    const IS_REMOVABLE: bool = true;
}
