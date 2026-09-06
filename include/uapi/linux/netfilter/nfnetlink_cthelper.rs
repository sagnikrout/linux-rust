//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter/nfnetlink_cthelper.h
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
pub const NFCT_HELPER_STATUS_DISABLED: c_int = 0;
pub const NFCT_HELPER_STATUS_ENABLED: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfnl_cthelper_msg_types {
    NFNL_MSG_CTHELPER_NEW,
    NFNL_MSG_CTHELPER_GET,
    NFNL_MSG_CTHELPER_DEL,
    NFNL_MSG_CTHELPER_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfnl_cthelper_type {
    NFCTH_UNSPEC,
    NFCTH_NAME,
    NFCTH_TUPLE,
    NFCTH_QUEUE_NUM,
    NFCTH_POLICY,
    NFCTH_PRIV_DATA_LEN,
    NFCTH_STATUS,
    __NFCTH_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfnl_cthelper_policy_type {
    NFCTH_POLICY_SET_UNSPEC,
    NFCTH_POLICY_SET_NUM,
    NFCTH_POLICY_SET,
    NFCTH_POLICY_SET1	= NFCTH_POLICY_SET,
    NFCTH_POLICY_SET2,
    NFCTH_POLICY_SET3,
    NFCTH_POLICY_SET4,
    __NFCTH_POLICY_SET_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfnl_cthelper_pol_type {
    NFCTH_POLICY_UNSPEC,
    NFCTH_POLICY_NAME,
    NFCTH_POLICY_EXPECT_MAX,
    NFCTH_POLICY_EXPECT_TIMEOUT,
    __NFCTH_POLICY_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfnl_cthelper_tuple_type {
    NFCTH_TUPLE_UNSPEC,
    NFCTH_TUPLE_L3PROTONUM,
    NFCTH_TUPLE_L4PROTONUM,
    __NFCTH_TUPLE_MAX,
}

