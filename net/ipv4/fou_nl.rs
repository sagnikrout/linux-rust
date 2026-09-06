//! Automatically rewritten from C Header to Rust Module
//! Source: net/ipv4/fou_nl.h
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


// SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause)
// Do not edit directly, auto-generated from:
// Documentation/netlink/specs/fou.yaml
// YNL-GEN kernel header
// To regenerate run: tools/net/ynl/ynl-regen.sh

// Global operation policy for fou
// Ops table for fou
extern "C" {
    pub fn fou_nl_add_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn fou_nl_del_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn fou_nl_get_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int;
}
extern "C" {
    pub fn fou_nl_get_dumpit(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int;
}
