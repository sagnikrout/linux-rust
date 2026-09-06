//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netlink_diag.h
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
#[derive(Copy, Clone)]
pub struct netlink_diag_req {
    pub sdiag_family: __u8,
    pub sdiag_protocol: __u8,
    pub pad: __u16,
    pub ndiag_ino: __u32,
    pub ndiag_show: __u32,
    pub ndiag_cookie: [__u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netlink_diag_msg {
    pub ndiag_family: __u8,
    pub ndiag_type: __u8,
    pub ndiag_protocol: __u8,
    pub ndiag_state: __u8,
    pub ndiag_portid: __u32,
    pub ndiag_dst_portid: __u32,
    pub ndiag_dst_group: __u32,
    pub ndiag_ino: __u32,
    pub ndiag_cookie: [__u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netlink_diag_ring {
    pub ndr_block_size: __u32,
    pub ndr_block_nr: __u32,
    pub ndr_frame_size: __u32,
    pub ndr_frame_nr: __u32,
}

// NETLINK_DIAG_NONE, standard nl API requires this attribute!

pub const NDIAG_SHOW_MEMINFO: c_uint = 0x00000001 /* show memory info of a socket */;
pub const NDIAG_SHOW_GROUPS: c_uint = 0x00000002 /* show groups of a netlink socket */;
// deprecated since 4.6
pub const NDIAG_SHOW_RING_CFG: c_uint = 0x00000004 /* show ring configuration */;

pub const NDIAG_SHOW_FLAGS: c_uint = 0x00000008 /* show flags of a netlink socket */;
// flags
pub const NDIAG_FLAG_CB_RUNNING: c_uint = 0x00000001;
pub const NDIAG_FLAG_PKTINFO: c_uint = 0x00000002;
pub const NDIAG_FLAG_BROADCAST_ERROR: c_uint = 0x00000004;
pub const NDIAG_FLAG_NO_ENOBUFS: c_uint = 0x00000008;
pub const NDIAG_FLAG_LISTEN_ALL_NSID: c_uint = 0x00000010;
pub const NDIAG_FLAG_CAP_ACK: c_uint = 0x00000020;
