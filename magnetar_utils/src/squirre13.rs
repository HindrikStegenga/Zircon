pub const fn squirre13(input: i32) -> u32 {
    const BIT_NOISE1: u32 = 0xB5297A4D;
    const BIT_NOISE2: u32 = 0x68E31DA4;
    const BIT_NOISE3: u32 = 0x1B56C4E9;

    let mut mangled: u32 = u32::from_le_bytes(input.to_le_bytes());
    mangled = mangled.wrapping_mul(BIT_NOISE1);
    mangled ^= mangled >> 8;
    mangled = mangled.wrapping_add(BIT_NOISE2);
    mangled ^= mangled << 8;
    mangled = mangled.wrapping_add(BIT_NOISE3);
    mangled ^= mangled >> 8;

    mangled
}

pub const fn squirre13_seeded(input: i32, seed: u32) -> u32 {
    const BIT_NOISE1: u32 = 0xB5297A4D;
    const BIT_NOISE2: u32 = 0x68E31DA4;
    const BIT_NOISE3: u32 = 0x1B56C4E9;

    let mut mangled: u32 = u32::from_le_bytes(input.to_le_bytes());
    mangled = mangled.wrapping_mul(BIT_NOISE1);
    mangled += seed;
    mangled ^= mangled >> 8;
    mangled = mangled.wrapping_add(BIT_NOISE2);
    mangled ^= mangled << 8;
    mangled = mangled.wrapping_add(BIT_NOISE3);
    mangled ^= mangled >> 8;

    mangled
}
