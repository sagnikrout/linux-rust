//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/clp.h
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
// CLP common request & response block size

// Call Logical Processor - Command Code
pub const CLP_SLPC: c_uint = 0x0001;
pub const CLP_LPS_BASE: c_int = 0;
pub const CLP_LPS_PCI: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clp_req_hdr {
    pub len: u16,
    pub cmd: u16,
    pub 4: u32 fmt :,
    pub 28: u32 reserved1 :,
    pub reserved2: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clp_rsp_hdr {
    pub len: u16,
    pub rsp: u16,
    pub 4: u32 fmt :,
    pub 28: u32 reserved1 :,
    pub reserved2: u64,
    pub __packed: },
// CLP Response Codes
pub const CLP_RC_OK: c_uint = 0x0010	/* Command request successfully */;
pub const CLP_RC_CMD: c_uint = 0x0020	/* Command code not recognized */;
pub const CLP_RC_PERM: c_uint = 0x0030	/* Command not authorized */;
pub const CLP_RC_FMT: c_uint = 0x0040	/* Invalid command request format */;
pub const CLP_RC_LEN: c_uint = 0x0050	/* Invalid command request length */;
pub const CLP_RC_8K: c_uint = 0x0060	/* Command requires 8K LPCB */;
pub const CLP_RC_RESNOT0: c_uint = 0x0070	/* Reserved field not zero */;
pub const CLP_RC_NODATA: c_uint = 0x0080	/* No data available */;
pub const CLP_RC_FC_UNKNOWN: c_uint = 0x0100	/* Function code not recognized */;
// Store logical-processor characteristics request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clp_req_slpc {
    pub hdr: clp_req_hdr,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clp_rsp_slpc {
    pub hdr: clp_rsp_hdr,
    pub reserved2: [u32; 4],
    pub lpif: [u32; 8],
    pub reserved3: [u32; 8],
    pub lpic: [u32; 8],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clp_req_rsp_slpc {
    pub request: clp_req_slpc,
    pub response: clp_rsp_slpc,
    pub __packed: },
