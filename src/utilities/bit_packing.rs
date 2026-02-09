// unsigned

pub fn pack_u32_u64(l: u32, r: u32) -> u64 {
    ((l as u64) << 32) | (r as u64)
}

pub fn unpack_u64_u32(lr: u64) -> (u32, u32) {
    ((lr >> 32) as u32, (lr & 0xFFFF_FFFF) as u32)
}

pub fn pack_u64_u128(l: u64, r: u64) -> u128 {
    ((l as u128) << 64) | (r as u128)
}

pub fn unpack_u128_u64(lr: u128) -> (u64, u64) {
    ((lr >> 64) as u64, (lr & 0xFFFF_FFFF_FFFF_FFFF) as u64)
}

// signed

pub fn pack_i32_i64(x: i32, y: i32) -> i64 {
    (((x as u32) as u64) << 32 | (y as u32) as u64) as i64
}

pub fn unpack_i64_i32(v: i64) -> (i32, i32) {
    let u = v as u64;
    ((u >> 32) as u32 as i32, u as u32 as i32)
}