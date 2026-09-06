//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/netfilter/nf_conntrack_extend.h
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nf_ct_ext_id {
    NF_CT_EXT_HELPER,

    NF_CT_EXT_NAT,

    NF_CT_EXT_SEQADJ,
    NF_CT_EXT_ACCT,

    NF_CT_EXT_ECACHE,

    NF_CT_EXT_TSTAMP,

    NF_CT_EXT_TIMEOUT,

    NF_CT_EXT_LABELS,

    NF_CT_EXT_SYNPROXY,

    NF_CT_EXT_ACT_CT,

    NF_CT_EXT_NUM,
}

// Extensions: optional stuff which isn't permanently in struct.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nf_ct_ext {
    pub offset: [u8; NF_CT_EXT_NUM],
    pub len: u8,
    pub __aligned(8): char data[],
}

// Add this type, returns pointer to data or NULL.
