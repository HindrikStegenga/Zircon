#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum EngineUpdateResult {
    Ok = 0,
    Stop = 1,
    Restart = 2,
}
