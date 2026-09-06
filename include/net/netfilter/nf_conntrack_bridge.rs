//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/netfilter/nf_conntrack_bridge.h
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


// Macro flag: #define NF_CONNTRACK_BRIDGE_

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_ct_bridge_info {
    pub ops: *mut nf_hook_ops,
    pub ops_size: c_uint,
    pub me: *mut module,
}

extern "C" {
    pub fn nf_ct_bridge_register(info: *mut nf_ct_bridge_info);
}
extern "C" {
    pub fn nf_ct_bridge_unregister(info: *mut nf_ct_bridge_info);
}
