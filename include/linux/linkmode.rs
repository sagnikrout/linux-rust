//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/linkmode.h
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


extern "C" {
    pub fn bitmap_empty(_arg: src, _arg: __ETHTOOL_LINK_MODE_MASK_NBITS) -> return;
}
extern "C" {
    pub fn bitmap_andnot(_arg: dst, _arg: src1, _arg: src2, _arg: __ETHTOOL_LINK_MODE_MASK_NBITS) -> return;
}

extern "C" {
    pub fn bitmap_equal(_arg: src1, _arg: src2, _arg: __ETHTOOL_LINK_MODE_MASK_NBITS) -> return;
}
extern "C" {
    pub fn bitmap_intersects(_arg: src1, _arg: src2, _arg: __ETHTOOL_LINK_MODE_MASK_NBITS) -> return;
}
extern "C" {
    pub fn bitmap_subset(_arg: src1, _arg: src2, _arg: __ETHTOOL_LINK_MODE_MASK_NBITS) -> return;
}
extern "C" {
    pub fn linkmode_set_pause(advertisement: *mut c_ulong, tx: bool, rx: bool);
}
