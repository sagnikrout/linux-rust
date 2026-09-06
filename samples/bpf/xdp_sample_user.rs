//! Automatically rewritten from C Header to Rust Module
//! Source: samples/bpf/xdp_sample_user.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stats_mask {
    _SAMPLE_REDIRECT_MAP         = 1U << 0,
    SAMPLE_RX_CNT                = 1U << 1,
    SAMPLE_REDIRECT_ERR_CNT      = 1U << 2,
    SAMPLE_CPUMAP_ENQUEUE_CNT    = 1U << 3,
    SAMPLE_CPUMAP_KTHREAD_CNT    = 1U << 4,
    SAMPLE_EXCEPTION_CNT         = 1U << 5,
    SAMPLE_DEVMAP_XMIT_CNT       = 1U << 6,
    SAMPLE_REDIRECT_CNT          = 1U << 7,
    SAMPLE_REDIRECT_MAP_CNT      = SAMPLE_REDIRECT_CNT | _SAMPLE_REDIRECT_MAP,
    SAMPLE_REDIRECT_ERR_MAP_CNT  = SAMPLE_REDIRECT_ERR_CNT | _SAMPLE_REDIRECT_MAP,
    SAMPLE_DEVMAP_XMIT_CNT_MULTI = 1U << 8,
    SAMPLE_SKIP_HEADING	     = 1U << 9,
}

// Exit return codes
pub const EXIT_OK: c_int = 0;
pub const EXIT_FAIL: c_int = 1;
pub const EXIT_FAIL_OPTION: c_int = 2;
pub const EXIT_FAIL_XDP: c_int = 3;
pub const EXIT_FAIL_BPF: c_int = 4;
pub const EXIT_FAIL_MEM: c_int = 5;
extern "C" {
    pub fn sample_setup_maps(maps: *mut bpf_map) -> c_int;
}
extern "C" {
    pub fn __sample_init(mask: c_int) -> c_int;
}
extern "C" {
    pub fn sample_exit(status: c_int);
}
extern "C" {
    pub fn sample_run(interval: c_int, ): *mut *mut void (post_cb)(void, ctx: *mut c_void) -> c_int;
}
extern "C" {
    pub fn sample_switch_mode();
}
extern "C" {
    pub fn get_mac_addr(ifindex: c_int, mac_addr: *mut c_void) -> c_int;
}

