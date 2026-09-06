//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/test_tcpbpf.h
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
pub struct tcpbpf_globals {
    pub event_map: __u32,
    pub total_retrans: __u32,
    pub data_segs_in: __u32,
    pub data_segs_out: __u32,
    pub bad_cb_test_rv: __u32,
    pub good_cb_test_rv: __u32,
    pub bytes_received: __u64,
    pub bytes_acked: __u64,
    pub num_listen: __u32,
    pub num_close_events: __u32,
    pub tcp_save_syn: __u32,
    pub tcp_saved_syn: __u32,
    pub window_clamp_client: __u32,
    pub window_clamp_server: __u32,
}
