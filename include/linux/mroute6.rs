//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mroute6.h
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
    pub fn ip6_mroute_setsockopt(: *mut sock, _arg: c_int, _arg: sockptr_t, int: unsigned) -> c_int;
}
extern "C" {
    pub fn ip6_mroute_getsockopt(: *mut sock, _arg: c_int, _arg: sockptr_t, _arg: sockptr_t) -> c_int;
}
extern "C" {
    pub fn ip6_mr_input(skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn ip6mr_compat_ioctl(sk: *mut sock, cmd: c_uint, arg: *mut void __user) -> c_int;
}
extern "C" {
    pub fn ip6_mr_init() -> c_int;
}
extern "C" {
    pub fn ip6_mr_output(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn ip6_mr_cleanup();
}
extern "C" {
    pub fn ip6mr_ioctl(sk: *mut sock, cmd: c_int, arg: *mut c_void) -> c_int;
}

extern "C" {
    pub fn ip6_output(_arg: net, _arg: sk, _arg: skb) -> return;
}

extern "C" {
    pub fn ip6mr_rule_default(rule: *const fib_rule) -> bool;
}

pub const VIFF_STATIC: c_uint = 0x8000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mfc6_cache_cmp_arg {
    pub mf6c_mcastgrp: in6_addr,
    pub mf6c_origin: in6_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mfc6_cache {
    pub _c: mr_mfc,
    pub mf6c_mcastgrp: in6_addr,
    pub mf6c_origin: in6_addr,
}

extern "C" {
    pub fn mroute6_is_socket(net: *mut net, skb: *mut sk_buff) -> bool;
}
extern "C" {
    pub fn ip6mr_sk_done(sk: *mut sock) -> c_int;
}
// These userspace buffers will be consumed by ip6mr_ioctl()

