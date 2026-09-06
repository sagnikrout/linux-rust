//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/dsa/xrs700x/xrs700x.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xrs700x_info {
    pub id: c_uint,
    pub name: *const c_char,
    pub num_ports: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xrs700x_port {
    pub /: *mut *mut mutex mib_mutex; / protects mib_data,
    pub mib_data: *mut u64,
    pub stats64: rtnl_link_stats64,
    pub syncp: u64_stats_sync,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xrs700x {
    pub ds: *mut dsa_switch,
    pub dev: *mut device,
    pub priv: *mut c_void,
    pub regmap: *mut regmap,
    pub ps_forward: *mut regmap_field,
    pub ps_management: *mut regmap_field,
    pub ps_sel_speed: *mut regmap_field,
    pub ps_cur_speed: *mut regmap_field,
    pub mib_work: delayed_work,
    pub ports: *mut xrs700x_port,
}

extern "C" {
    pub fn xrs700x_switch_register(priv: *mut xrs700x) -> c_int;
}
extern "C" {
    pub fn xrs700x_switch_remove(priv: *mut xrs700x);
}
extern "C" {
    pub fn xrs700x_switch_shutdown(priv: *mut xrs700x);
}
