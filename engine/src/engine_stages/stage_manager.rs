use crate::engine_stages::*;
use utils::as_any::AsAny;

macro_rules! impl_stage_manager {
    ($name:ident, $stage_any_trait:ident, $stage_trait:ident) => {
        pub struct $name<'a> {
            before: &'a mut [Box<dyn $stage_any_trait>],
            after: &'a mut [Box<dyn $stage_any_trait>],
        }

        impl<'a> $name<'a> {
            pub fn from_slices(
                before: &'a mut [Box<dyn $stage_any_trait>],
                after: &'a mut [Box<dyn $stage_any_trait>],
            ) -> Self {
                Self { before, after }
            }

            pub fn from_slice(slice: &'a mut [Box<dyn $stage_any_trait>]) -> Self {
                Self {
                    before: slice,
                    after: &mut [],
                }
            }

            pub fn get_stage<S: $stage_trait>(&'a mut self) -> Option<&'a mut S> {
                for elem in self.before.iter_mut() {
                    let any = elem.as_any_mut();
                    if let Some(item) = any.downcast_mut::<S>() {
                        return Some(item);
                    }
                }
                for elem in self.after.iter_mut() {
                    let any = elem.as_any_mut();
                    if let Some(item) = any.downcast_mut::<S>() {
                        return Some(item);
                    }
                }

                return None;
            }
        }
    };
}

impl_stage_manager!(RenderStageManager, AnyRenderStage, RenderStage);
impl_stage_manager!(UpdateStageManager, AnyUpdateStage, UpdateStage);
