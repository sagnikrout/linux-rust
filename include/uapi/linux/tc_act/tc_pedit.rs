//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/tc_act/tc_pedit.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

// TCA_PEDIT_KEY_EX_HDR_TYPE_NETWROK is a special case for legacy users. It
// means no specific header type - offset is relative to the network layer
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pedit_header_type {
    TCA_PEDIT_KEY_EX_HDR_TYPE_NETWORK = 0,
    TCA_PEDIT_KEY_EX_HDR_TYPE_ETH = 1,
    TCA_PEDIT_KEY_EX_HDR_TYPE_IP4 = 2,
    TCA_PEDIT_KEY_EX_HDR_TYPE_IP6 = 3,
    TCA_PEDIT_KEY_EX_HDR_TYPE_TCP = 4,
    TCA_PEDIT_KEY_EX_HDR_TYPE_UDP = 5,
    __PEDIT_HDR_TYPE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pedit_cmd {
    TCA_PEDIT_KEY_EX_CMD_SET = 0,
    TCA_PEDIT_KEY_EX_CMD_ADD = 1,
    __PEDIT_CMD_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_pedit_key {
    pub /: *mut *mut __u32 mask; / AND,
    pub /: *mut *mut __u32 val; /XOR,
    pub /: *mut *mut __u32 off; /offset,
    pub at: __u32,
    pub offmask: __u32,
    pub shift: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_pedit_sel {
    pub nkeys: c_uchar,
    pub flags: c_uchar,
    pub __counted_by(nkeys): tc_pedit_key keys[],
}

