//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/gtp.h
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
pub enum gtp_genl_cmds {
    GTP_CMD_NEWPDP,
    GTP_CMD_DELPDP,
    GTP_CMD_GETPDP,
    GTP_CMD_ECHOREQ,

    GTP_CMD_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gtp_version {
    GTP_V0 = 0,
    GTP_V1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gtp_attrs {
    GTPA_UNSPEC = 0,
    GTPA_LINK,
    GTPA_VERSION,
    GTPA_TID,	/* for GTPv0 only */
    GTPA_PEER_ADDRESS,	/* Remote GSN peer, either SGSN or GGSN */

    GTPA_MS_ADDRESS,
    GTPA_FLOW,
    GTPA_NET_NS_FD,
    GTPA_I_TEI,	/* for GTPv1 only */
    GTPA_O_TEI,	/* for GTPv1 only */
    GTPA_PAD,
    GTPA_PEER_ADDR6,
    GTPA_MS_ADDR6,
    GTPA_FAMILY,
    __GTPA_MAX,
}

