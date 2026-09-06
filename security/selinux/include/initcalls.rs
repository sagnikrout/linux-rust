//! Automatically rewritten from C Header to Rust Module
//! Source: security/selinux/include/initcalls.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// SELinux initcalls
//
extern "C" {
    pub fn init_sel_fs() -> c_int;
}
extern "C" {
    pub fn sel_netport_init() -> c_int;
}
extern "C" {
    pub fn sel_netnode_init() -> c_int;
}
extern "C" {
    pub fn sel_netif_init() -> c_int;
}
extern "C" {
    pub fn sel_netlink_init() -> c_int;
}
extern "C" {
    pub fn sel_ib_pkey_init() -> c_int;
}
extern "C" {
    pub fn selinux_nf_ip_init() -> c_int;
}
extern "C" {
    pub fn selinux_initcall() -> c_int;
}
