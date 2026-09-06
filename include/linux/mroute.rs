//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mroute.h
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

extern "C" {
    pub fn ip_mroute_setsockopt(: *mut sock, _arg: c_int, _arg: sockptr_t, int: unsigned) -> c_int;
}
extern "C" {
    pub fn ip_mroute_getsockopt(: *mut sock, _arg: c_int, _arg: sockptr_t, _arg: sockptr_t) -> c_int;
}
extern "C" {
    pub fn ipmr_ioctl(sk: *mut sock, cmd: c_int, arg: *mut c_void) -> c_int;
}
extern "C" {
    pub fn ipmr_compat_ioctl(sk: *mut sock, cmd: c_uint, arg: *mut void __user) -> c_int;
}
extern "C" {
    pub fn ip_mr_init() -> c_int;
}
extern "C" {
    pub fn ipmr_rule_default(rule: *const fib_rule) -> bool;
}
extern "C" {
    pub fn ipmr_sk_ioctl(sk: *mut sock, cmd: c_uint, arg: *mut void __user) -> c_int;
}

pub const VIFF_STATIC: c_uint = 0x8000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mfc_cache_cmp_arg {
    pub mfc_mcastgrp: __be32,
    pub mfc_origin: __be32,
}

//
// struct mfc_cache - multicast routing entries
// @_c: Common multicast routing information; has to be first [for casting]
// @mfc_mcastgrp: destination multicast group address
// @mfc_origin: source address
// @cmparg: used for rhashtable comparisons
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mfc_cache {
    pub _c: mr_mfc,
    pub mfc_mcastgrp: __be32,
    pub mfc_origin: __be32,
}
