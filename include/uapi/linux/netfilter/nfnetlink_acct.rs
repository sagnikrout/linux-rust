//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter/nfnetlink_acct.h
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

pub const NFACCT_NAME_MAX: c_int = 32;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfnl_acct_msg_types {
    NFNL_MSG_ACCT_NEW,
    NFNL_MSG_ACCT_GET,
    NFNL_MSG_ACCT_GET_CTRZERO,
    NFNL_MSG_ACCT_DEL,
    NFNL_MSG_ACCT_OVERQUOTA,
    NFNL_MSG_ACCT_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfnl_acct_flags {
    NFACCT_F_QUOTA_PKTS	= (1 << 0),
    NFACCT_F_QUOTA_BYTES	= (1 << 1),
    NFACCT_F_OVERQUOTA	= (1 << 2), /* can't be set from userspace */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfnl_acct_type {
    NFACCT_UNSPEC,
    NFACCT_NAME,
    NFACCT_PKTS,
    NFACCT_BYTES,
    NFACCT_USE,
    NFACCT_FLAGS,
    NFACCT_QUOTA,
    NFACCT_FILTER,
    NFACCT_PAD,
    __NFACCT_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfnl_attr_filter_type {
    NFACCT_FILTER_UNSPEC,
    NFACCT_FILTER_MASK,
    NFACCT_FILTER_VALUE,
    __NFACCT_FILTER_MAX
}

