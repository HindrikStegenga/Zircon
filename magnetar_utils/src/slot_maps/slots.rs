use crate::handles::HandleType;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Slot<K>
where
    K: HandleType,
{
    pub index: K,
    pub reverse_slot: K,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VersionedSlot<K, G>
where
    K: HandleType,
    G: HandleType,
{
    pub index: K,
    pub generation: G,
    pub reverse_slot: K,
}
