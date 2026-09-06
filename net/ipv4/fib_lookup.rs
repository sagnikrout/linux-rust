//! Automatically rewritten from C Header to Rust Module
//! Source: net/ipv4/fib_lookup.h
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
pub struct fib_alias {
    pub fa_list: hlist_node,
    pub fa_info: *mut fib_info,
    pub fa_dscp: dscp_t,
    pub fa_type: u8,
    pub fa_state: u8,
    pub fa_slen: u8,
    pub tb_id: u32,
    pub fa_default: i16,
    pub offload: u8,
    pub trap: u8,
    pub offload_failed: u8,
    pub rcu: rcu_head,
}

pub const FA_S_ACCESSED: c_uint = 0x01;
// Don't write on fa_state unless needed, to keep it shared on all cpus
// Exported by fib_semantics.c
extern "C" {
    pub fn fib_release_info(: *mut fib_info);
}
extern "C" {
    pub fn fib_metrics_match(cfg: *mut fib_config, fi: *mut fib_info) -> bool;
}
extern "C" {
    pub fn fib_nlmsg_size(fi: *mut fib_info) -> usize;
}
// we used to play games with refcounts, but we now use RCU
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fib_prop {
    pub error: c_int,
    pub scope: u8,
}
