//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/test_tcp_hdr_options.h
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
// Copyright (c) 2020 Facebook
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_test_option {
    pub flags: __u8,
    pub max_delack_ms: __u8,
    pub rand: __u8,
    pub __attribute__((packed)): },
}

// Store in bpf_sk_storage
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdr_stg {
    pub active: bool,
    pub /: *mut *mut bool resend_syn; / active side only,
    pub /: *mut *mut bool syncookie; / passive side only,
    pub /: *mut *mut bool fastopen; / passive side only,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct linum_err {
    pub linum: c_uint,
    pub err: c_int,
}

pub const TCPHDR_FIN: c_uint = 0x01;
pub const TCPHDR_SYN: c_uint = 0x02;
pub const TCPHDR_RST: c_uint = 0x04;
pub const TCPHDR_PSH: c_uint = 0x08;
pub const TCPHDR_ACK: c_uint = 0x10;
pub const TCPHDR_URG: c_uint = 0x20;
pub const TCPHDR_ECE: c_uint = 0x40;
pub const TCPHDR_CWR: c_uint = 0x80;

pub const TCPOPT_EOL: c_int = 0;
pub const TCPOPT_NOP: c_int = 1;
pub const TCPOPT_MSS: c_int = 2;
pub const TCPOPT_WINDOW: c_int = 3;
pub const TCPOPT_EXP: c_int = 254;
pub const TCP_BPF_EXPOPT_BASE_LEN: c_int = 4;
pub const MAX_TCP_HDR_LEN: c_int = 60;
pub const MAX_TCP_OPTION_SPACE: c_int = 40;

pub const CG_OK: c_int = 1;
pub const CG_ERR: c_int = 0;

pub const SOL_TCP: c_int = 6;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_exprm_opt {
    pub kind: __u8,
    pub len: __u8,
    pub magic: __u16,
    pub data: [__u8; 4],
    pub data32: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_opt {
    pub kind: __u8,
    pub len: __u8,
    pub data: [__u8; 4],
    pub data32: __u32,
}

