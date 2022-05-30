use crate::engine_stages::*;

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

            pub fn get_stage<S: $stage_trait>(&mut self) -> Option<&mut S> {
                for elem in self.before.iter_mut() {
                    if let Some(item) =
                        $stage_any_trait::stage_as_any_mut(elem.as_mut()).downcast_mut::<S>()
                    {
                        return Some(item);
                    }
                }
                for elem in self.after.iter_mut() {
                    if let Some(item) =
                        $stage_any_trait::stage_as_any_mut(elem.as_mut()).downcast_mut::<S>()
                    {
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
