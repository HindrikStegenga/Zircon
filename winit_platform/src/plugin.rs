use engine::{engine_stages::*, *};
use std::any::TypeId;
use utils::as_any::*;
use winit::{event::*, event_loop::EventLoop};

use crate::WinitPlatformInterface;

pub(crate) struct WinitPlatformPluginContainer<P: WinitPlatformPlugin> {
    item: P,
}

impl<P: WinitPlatformPlugin> WinitPlatformPluginContainer<P> {
    pub fn new(item: P) -> Self {
        Self { item }
    }
}

pub(crate) trait AnyWinitPlatformPlugin {
    fn plugin_type_id(&self) -> TypeId;
    fn plugin_as_any(&mut self) -> &mut dyn std::any::Any;
    fn pre_run(&mut self, event_loop: EventLoop<()>);
    fn systems_will_init(
        &mut self,
        platform_interface: &mut WinitPlatformInterface,
        input: &mut PlatformInitInput,
    ) -> EngineUpdateResult;
    fn systems_did_init(
        &mut self,
        platform_interface: &mut WinitPlatformInterface,
        input: &mut PlatformInitInput,
    ) -> EngineUpdateResult;
    fn process_event(&mut self, event: &Event<()>) -> bool;
}

impl<P> AnyWinitPlatformPlugin for WinitPlatformPluginContainer<P>
where
    P: WinitPlatformPlugin,
{
    fn plugin_type_id(&self) -> TypeId {
        TypeId::of::<P>()
    }

    fn plugin_as_any(&mut self) -> &mut dyn std::any::Any {
        self.item.as_any_mut()
    }

    fn pre_run(&mut self, event_loop: EventLoop<()>) {
        self.item.pre_run(event_loop)
    }

    fn systems_will_init(
        &mut self,
        platform_interface: &mut WinitPlatformInterface,
        input: &mut PlatformInitInput,
    ) -> EngineUpdateResult {
        self.item.systems_will_init(platform_interface, input)
    }

    fn systems_did_init(
        &mut self,
        platform_interface: &mut WinitPlatformInterface,
        input: &mut PlatformInitInput,
    ) -> EngineUpdateResult {
        self.item.systems_did_init(platform_interface, input)
    }

    fn process_event(&mut self, event: &Event<()>) -> bool {
        self.item.process_event(event)
    }
}

pub trait WinitPlatformPlugin: 'static {
    fn pre_run(&mut self, event_loop: EventLoop<()>);
    fn systems_will_init(
        &mut self,
        platform_interface: &mut WinitPlatformInterface,
        input: &mut PlatformInitInput,
    ) -> EngineUpdateResult;
    fn systems_did_init(
        &mut self,
        platform_interface: &mut WinitPlatformInterface,
        input: &mut PlatformInitInput,
    ) -> EngineUpdateResult;
    fn process_event(&mut self, event: &Event<()>) -> bool;
}
