//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/packet_diag.h
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
pub struct packet_diag_req {
    pub sdiag_family: __u8,
    pub sdiag_protocol: __u8,
    pub pad: __u16,
    pub pdiag_ino: __u32,
    pub pdiag_show: __u32,
    pub pdiag_cookie: [__u32; 2],
}

pub const PACKET_SHOW_INFO: c_uint = 0x00000001 /* Basic packet_sk information */;
pub const PACKET_SHOW_MCLIST: c_uint = 0x00000002 /* A set of packet_diag_mclist-s */;
pub const PACKET_SHOW_RING_CFG: c_uint = 0x00000004 /* Rings configuration parameters */;
pub const PACKET_SHOW_FANOUT: c_uint = 0x00000008;
pub const PACKET_SHOW_MEMINFO: c_uint = 0x00000010;
pub const PACKET_SHOW_FILTER: c_uint = 0x00000020;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct packet_diag_msg {
    pub pdiag_family: __u8,
    pub pdiag_type: __u8,
    pub pdiag_num: __u16,
    pub pdiag_ino: __u32,
    pub pdiag_cookie: [__u32; 2],
}

// PACKET_DIAG_NONE, standard nl API requires this attribute!

#[repr(C)]
#[derive(Copy, Clone)]
pub struct packet_diag_info {
    pub pdi_index: __u32,
    pub pdi_version: __u32,
    pub pdi_reserve: __u32,
    pub pdi_copy_thresh: __u32,
    pub pdi_tstamp: __u32,
    pub pdi_flags: __u32,
pub const PDI_RUNNING: c_uint = 0x1;
pub const PDI_AUXDATA: c_uint = 0x2;
pub const PDI_ORIGDEV: c_uint = 0x4;
pub const PDI_VNETHDR: c_uint = 0x8;
pub const PDI_LOSS: c_uint = 0x10;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct packet_diag_mclist {
    pub pdmc_index: __u32,
    pub pdmc_count: __u32,
    pub pdmc_type: __u16,
    pub pdmc_alen: __u16,
    pub /: *mut *mut __u8 pdmc_addr[32]; / MAX_ADDR_LEN,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct packet_diag_ring {
    pub pdr_block_size: __u32,
    pub pdr_block_nr: __u32,
    pub pdr_frame_size: __u32,
    pub pdr_frame_nr: __u32,
    pub pdr_retire_tmo: __u32,
    pub pdr_sizeof_priv: __u32,
    pub pdr_features: __u32,
}
