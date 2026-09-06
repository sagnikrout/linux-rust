//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/arp.h
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


// SPDX-License-Identifier: GPL-2.0
// linux/net/inet/arp.h

extern "C" {
    pub fn ___neigh_lookup_noref(_arg: &arp_tbl, _arg: neigh_key_eq32, _arg: arp_hashfn, _arg: &key, _arg: dev) -> return;
}

extern "C" {
    pub fn arp_init();
}
extern "C" {
    pub fn arp_ioctl(net: *mut net, cmd: c_uint, arg: *mut void __user) -> c_int;
}
extern "C" {
    pub fn arp_mc_map(addr: __be32, haddr: *mut u8, dev: *mut net_device, dir: c_int) -> c_int;
}
extern "C" {
    pub fn arp_ifdown(dev: *mut net_device);
}
extern "C" {
    pub fn arp_invalidate(dev: *mut net_device, ip: __be32, force: bool) -> c_int;
}
extern "C" {
    pub fn arp_xmit(skb: *mut sk_buff);
}
