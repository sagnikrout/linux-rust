//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/tun_vnet.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
// High bits in flags field are unused.
pub const TUN_VNET_LE: c_uint = 0x80000000;
pub const TUN_VNET_BE: c_uint = 0x40000000;

extern "C" {
    pub fn __virtio16_to_cpu(_arg: tun_vnet_is_little_endian(flags), _arg: val) -> return;
}
extern "C" {
    pub fn __cpu_to_virtio16(_arg: tun_vnet_is_little_endian(flags), _arg: val) -> return;
}
// vnet_hdr_sz = s;
extern "C" {
    pub fn tun_get_vnet_be(_arg: *mut flags, _arg: sp) -> return;
}
extern "C" {
    pub fn tun_set_vnet_be(_arg: flags, _arg: sp) -> return;
}
extern "C" {
    pub fn sizeof(virtio_net_hdr: struct) -> return;
}
extern "C" {
    pub fn __tun_vnet_hdr_get(_arg: sz, _arg: flags, _arg: 0, _arg: from, _arg: hdr) -> return;
}
extern "C" {
    pub fn __tun_vnet_hdr_put(_arg: sz, _arg: 0, _arg: iter, _arg: hdr) -> return;
}
extern "C" {
    pub fn virtio_net_hdr_to_skb(_arg: skb, _arg: hdr, _arg: tun_vnet_is_little_endian(flags)) -> return;
}
//
// Tun is not aware of the negotiated guest features, guess them from the
// virtio net hdr size
//
