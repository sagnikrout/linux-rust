//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter/nfnetlink.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfnetlink_groups {
    NFNLGRP_NONE,

    NFNLGRP_CONNTRACK_NEW,

    NFNLGRP_CONNTRACK_UPDATE,

    NFNLGRP_CONNTRACK_DESTROY,

    NFNLGRP_CONNTRACK_EXP_NEW,

    NFNLGRP_CONNTRACK_EXP_UPDATE,

    NFNLGRP_CONNTRACK_EXP_DESTROY,

    NFNLGRP_NFTABLES,

    NFNLGRP_ACCT_QUOTA,

    NFNLGRP_NFTRACE,

    __NFNLGRP_MAX,
}

// General form of address family dependent message.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfgenmsg {
    pub /: *mut *mut __u8 nfgen_family; / AF_xxx,
    pub /: *mut *mut __u8 version; / nfnetlink version,
    pub /: *mut *mut __be16 res_id; / resource id,
}

pub const NFNETLINK_V0: c_int = 0;
// netfilter netlink message types are split in two pieces:
// 8 bit subsystem, 8bit operation.
//

// No enum here, otherwise __stringify() trick of MODULE_ALIAS_NFNL_SUBSYS()
// won't work anymore
pub const NFNL_SUBSYS_NONE: c_int = 0;
pub const NFNL_SUBSYS_CTNETLINK: c_int = 1;
pub const NFNL_SUBSYS_CTNETLINK_EXP: c_int = 2;
pub const NFNL_SUBSYS_QUEUE: c_int = 3;
pub const NFNL_SUBSYS_ULOG: c_int = 4;
pub const NFNL_SUBSYS_OSF: c_int = 5;
pub const NFNL_SUBSYS_IPSET: c_int = 6;
pub const NFNL_SUBSYS_ACCT: c_int = 7;
pub const NFNL_SUBSYS_CTNETLINK_TIMEOUT: c_int = 8;
pub const NFNL_SUBSYS_CTHELPER: c_int = 9;
pub const NFNL_SUBSYS_NFTABLES: c_int = 10;
pub const NFNL_SUBSYS_NFT_COMPAT: c_int = 11;
pub const NFNL_SUBSYS_HOOK: c_int = 12;
pub const NFNL_SUBSYS_COUNT: c_int = 13;
// Reserved control nfnetlink messages

//
// enum nfnl_batch_attributes - nfnetlink batch netlink attributes
//
// @NFNL_BATCH_GENID: generation ID for this changeset (NLA_U32)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfnl_batch_attributes {
    NFNL_BATCH_UNSPEC,
    NFNL_BATCH_GENID,
    __NFNL_BATCH_MAX
}

