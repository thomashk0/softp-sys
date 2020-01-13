/* automatically generated by rust-bindgen */

pub type __int32_t = ::std::os::raw::c_int;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type __int64_t = ::std::os::raw::c_long;
pub type __uint64_t = ::std::os::raw::c_ulong;
pub type int128_t = i128;
pub type uint128_t = u128;
pub const RoundingModeEnum_RM_RNE: RoundingModeEnum = 0;
pub const RoundingModeEnum_RM_RTZ: RoundingModeEnum = 1;
pub const RoundingModeEnum_RM_RDN: RoundingModeEnum = 2;
pub const RoundingModeEnum_RM_RUP: RoundingModeEnum = 3;
pub const RoundingModeEnum_RM_RMM: RoundingModeEnum = 4;
pub type RoundingModeEnum = u32;
pub type sfloat32 = u32;
pub type sfloat64 = u64;
pub type sfloat128 = uint128_t;
extern "C" {
    pub fn add_sf32(a: sfloat32, b: sfloat32, rm: RoundingModeEnum, pfflags: *mut u32) -> sfloat32;
}
extern "C" {
    pub fn sub_sf32(a: sfloat32, b: sfloat32, rm: RoundingModeEnum, pfflags: *mut u32) -> sfloat32;
}
extern "C" {
    pub fn mul_sf32(a: sfloat32, b: sfloat32, rm: RoundingModeEnum, pfflags: *mut u32) -> sfloat32;
}
extern "C" {
    pub fn div_sf32(a: sfloat32, b: sfloat32, rm: RoundingModeEnum, pfflags: *mut u32) -> sfloat32;
}
extern "C" {
    pub fn sqrt_sf32(a: sfloat32, rm: RoundingModeEnum, pfflags: *mut u32) -> sfloat32;
}
extern "C" {
    pub fn fma_sf32(
        a: sfloat32,
        b: sfloat32,
        c: sfloat32,
        rm: RoundingModeEnum,
        pfflags: *mut u32,
    ) -> sfloat32;
}
extern "C" {
    pub fn min_sf32(a: sfloat32, b: sfloat32, pfflags: *mut u32) -> sfloat32;
}
extern "C" {
    pub fn max_sf32(a: sfloat32, b: sfloat32, pfflags: *mut u32) -> sfloat32;
}
extern "C" {
    pub fn eq_quiet_sf32(a: sfloat32, b: sfloat32, pfflags: *mut u32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn le_sf32(a: sfloat32, b: sfloat32, pfflags: *mut u32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lt_sf32(a: sfloat32, b: sfloat32, pfflags: *mut u32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fclass_sf32(a: sfloat32) -> u32;
}
extern "C" {
    pub fn cvt_sf32_sf64(a: sfloat32, pfflags: *mut u32) -> sfloat64;
}
extern "C" {
    pub fn cvt_sf64_sf32(a: sfloat64, rm: RoundingModeEnum, pfflags: *mut u32) -> sfloat32;
}
extern "C" {
    pub fn cvt_sf32_i32(a: sfloat32, rm: RoundingModeEnum, pfflags: *mut u32) -> i32;
}
extern "C" {
    pub fn cvt_sf32_u32(a: sfloat32, rm: RoundingModeEnum, pfflags: *mut u32) -> u32;
}
extern "C" {
    pub fn cvt_sf32_i64(a: sfloat32, rm: RoundingModeEnum, pfflags: *mut u32) -> i64;
}
extern "C" {
    pub fn cvt_sf32_u64(a: sfloat32, rm: RoundingModeEnum, pfflags: *mut u32) -> u64;
}
extern "C" {
    pub fn cvt_sf32_i128(a: sfloat32, rm: RoundingModeEnum, pfflags: *mut u32) -> int128_t;
}
extern "C" {
    pub fn cvt_sf32_u128(a: sfloat32, rm: RoundingModeEnum, pfflags: *mut u32) -> uint128_t;
}
extern "C" {
    pub fn cvt_i32_sf32(a: i32, rm: RoundingModeEnum, pfflags: *mut u32) -> sfloat32;
}
extern "C" {
    pub fn cvt_u32_sf32(a: u32, rm: RoundingModeEnum, pfflags: *mut u32) -> sfloat32;
}
extern "C" {
    pub fn cvt_i64_sf32(a: i64, rm: RoundingModeEnum, pfflags: *mut u32) -> sfloat32;
}
extern "C" {
    pub fn cvt_u64_sf32(a: u64, rm: RoundingModeEnum, pfflags: *mut u32) -> sfloat32;
}
extern "C" {
    pub fn cvt_i128_sf32(a: int128_t, rm: RoundingModeEnum, pfflags: *mut u32) -> sfloat32;
}
extern "C" {
    pub fn cvt_u128_sf32(a: uint128_t, rm: RoundingModeEnum, pfflags: *mut u32) -> sfloat32;
}
extern "C" {
    pub fn add_sf64(a: sfloat64, b: sfloat64, rm: RoundingModeEnum, pfflags: *mut u32) -> sfloat64;
}
extern "C" {
    pub fn sub_sf64(a: sfloat64, b: sfloat64, rm: RoundingModeEnum, pfflags: *mut u32) -> sfloat64;
}
extern "C" {
    pub fn mul_sf64(a: sfloat64, b: sfloat64, rm: RoundingModeEnum, pfflags: *mut u32) -> sfloat64;
}
extern "C" {
    pub fn div_sf64(a: sfloat64, b: sfloat64, rm: RoundingModeEnum, pfflags: *mut u32) -> sfloat64;
}
extern "C" {
    pub fn sqrt_sf64(a: sfloat64, rm: RoundingModeEnum, pfflags: *mut u32) -> sfloat64;
}
extern "C" {
    pub fn fma_sf64(
        a: sfloat64,
        b: sfloat64,
        c: sfloat64,
        rm: RoundingModeEnum,
        pfflags: *mut u32,
    ) -> sfloat64;
}
extern "C" {
    pub fn min_sf64(a: sfloat64, b: sfloat64, pfflags: *mut u32) -> sfloat64;
}
extern "C" {
    pub fn max_sf64(a: sfloat64, b: sfloat64, pfflags: *mut u32) -> sfloat64;
}
extern "C" {
    pub fn eq_quiet_sf64(a: sfloat64, b: sfloat64, pfflags: *mut u32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn le_sf64(a: sfloat64, b: sfloat64, pfflags: *mut u32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lt_sf64(a: sfloat64, b: sfloat64, pfflags: *mut u32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fclass_sf64(a: sfloat64) -> u32;
}
extern "C" {
    pub fn cvt_sf64_i32(a: sfloat64, rm: RoundingModeEnum, pfflags: *mut u32) -> i32;
}
extern "C" {
    pub fn cvt_sf64_u32(a: sfloat64, rm: RoundingModeEnum, pfflags: *mut u32) -> u32;
}
extern "C" {
    pub fn cvt_sf64_i64(a: sfloat64, rm: RoundingModeEnum, pfflags: *mut u32) -> i64;
}
extern "C" {
    pub fn cvt_sf64_u64(a: sfloat64, rm: RoundingModeEnum, pfflags: *mut u32) -> u64;
}
extern "C" {
    pub fn cvt_sf64_i128(a: sfloat64, rm: RoundingModeEnum, pfflags: *mut u32) -> int128_t;
}
extern "C" {
    pub fn cvt_sf64_u128(a: sfloat64, rm: RoundingModeEnum, pfflags: *mut u32) -> uint128_t;
}
extern "C" {
    pub fn cvt_i32_sf64(a: i32, rm: RoundingModeEnum, pfflags: *mut u32) -> sfloat64;
}
extern "C" {
    pub fn cvt_u32_sf64(a: u32, rm: RoundingModeEnum, pfflags: *mut u32) -> sfloat64;
}
extern "C" {
    pub fn cvt_i64_sf64(a: i64, rm: RoundingModeEnum, pfflags: *mut u32) -> sfloat64;
}
extern "C" {
    pub fn cvt_u64_sf64(a: u64, rm: RoundingModeEnum, pfflags: *mut u32) -> sfloat64;
}
extern "C" {
    pub fn cvt_i128_sf64(a: int128_t, rm: RoundingModeEnum, pfflags: *mut u32) -> sfloat64;
}
extern "C" {
    pub fn cvt_u128_sf64(a: uint128_t, rm: RoundingModeEnum, pfflags: *mut u32) -> sfloat64;
}
extern "C" {
    pub fn cvt_sf32_sf128(a: sfloat32, pfflags: *mut u32) -> sfloat128;
}
extern "C" {
    pub fn cvt_sf128_sf32(a: sfloat128, rm: RoundingModeEnum, pfflags: *mut u32) -> sfloat32;
}
extern "C" {
    pub fn cvt_sf64_sf128(a: sfloat64, pfflags: *mut u32) -> sfloat128;
}
extern "C" {
    pub fn cvt_sf128_sf64(a: sfloat128, rm: RoundingModeEnum, pfflags: *mut u32) -> sfloat64;
}
extern "C" {
    pub fn cvt_sf128_i32(a: sfloat128, rm: RoundingModeEnum, pfflags: *mut u32) -> i32;
}
extern "C" {
    pub fn cvt_sf128_u32(a: sfloat128, rm: RoundingModeEnum, pfflags: *mut u32) -> u32;
}
extern "C" {
    pub fn cvt_sf128_i64(a: sfloat128, rm: RoundingModeEnum, pfflags: *mut u32) -> i64;
}
extern "C" {
    pub fn cvt_sf128_u64(a: sfloat128, rm: RoundingModeEnum, pfflags: *mut u32) -> u64;
}
extern "C" {
    pub fn cvt_sf128_i128(a: sfloat128, rm: RoundingModeEnum, pfflags: *mut u32) -> int128_t;
}
extern "C" {
    pub fn cvt_sf128_u128(a: sfloat128, rm: RoundingModeEnum, pfflags: *mut u32) -> uint128_t;
}
extern "C" {
    pub fn cvt_i32_sf128(a: i32, rm: RoundingModeEnum, pfflags: *mut u32) -> sfloat128;
}
extern "C" {
    pub fn cvt_u32_sf128(a: u32, rm: RoundingModeEnum, pfflags: *mut u32) -> sfloat128;
}
extern "C" {
    pub fn cvt_i64_sf128(a: i64, rm: RoundingModeEnum, pfflags: *mut u32) -> sfloat128;
}
extern "C" {
    pub fn cvt_u64_sf128(a: u64, rm: RoundingModeEnum, pfflags: *mut u32) -> sfloat128;
}
extern "C" {
    pub fn cvt_i128_sf128(a: int128_t, rm: RoundingModeEnum, pfflags: *mut u32) -> sfloat128;
}
extern "C" {
    pub fn cvt_u128_sf128(a: uint128_t, rm: RoundingModeEnum, pfflags: *mut u32) -> sfloat128;
}