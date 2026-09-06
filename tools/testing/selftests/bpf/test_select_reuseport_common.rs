//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/test_select_reuseport_common.h
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
// Copyright (c) 2018 Facebook

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum result {
    DROP_ERR_INNER_MAP,
    DROP_ERR_SKB_DATA,
    DROP_ERR_SK_SELECT_REUSEPORT,
    DROP_MISC,
    PASS,
    PASS_ERR_SK_SELECT_REUSEPORT,
    NR_RESULTS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd {
    pub reuseport_index: __u32,
    pub pass_on_failure: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct data_check {
    pub ip_protocol: __u32,
    pub skb_addrs: [__u32; 8],
    pub skb_ports: [__u16; 2],
    pub eth_protocol: __u16,
    pub bind_inany: __u8,
    pub equal_check_end: [__u8; 0],
    pub len: __u32,
    pub hash: __u32,
}
