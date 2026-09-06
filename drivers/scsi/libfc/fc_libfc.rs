//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/libfc/fc_libfc.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2009 Intel Corporation. All rights reserved.
//
// Maintained at www.Open-FCoE.org
//
pub const FC_LIBFC_LOGGING: c_uint = 0x01 /* General logging, not categorized */;
pub const FC_LPORT_LOGGING: c_uint = 0x02 /* lport layer logging */;
pub const FC_DISC_LOGGING: c_uint = 0x04 /* discovery layer logging */;
pub const FC_RPORT_LOGGING: c_uint = 0x08 /* rport layer logging */;
pub const FC_FCP_LOGGING: c_uint = 0x10 /* I/O path logging */;
pub const FC_EM_LOGGING: c_uint = 0x20 /* Exchange Manager logging */;
pub const FC_EXCH_LOGGING: c_uint = 0x40 /* Exchange/Sequence logging */;
pub const FC_SCSI_LOGGING: c_uint = 0x80 /* SCSI logging (mostly error handling) */;

//
// FC-4 Providers.
//
// Set up direct-data placement for this I/O request
//
extern "C" {
    pub fn fc_fcp_ddp_setup(fsp: *mut fc_fcp_pkt, xid: u16);
}
extern "C" {
    pub fn fc_fcp_ddp_done(fsp: *mut fc_fcp_pkt);
}
//
// Module setup functions
//
extern "C" {
    pub fn fc_setup_exch_mgr() -> c_int;
}
extern "C" {
    pub fn fc_destroy_exch_mgr();
}
extern "C" {
    pub fn fc_setup_rport() -> c_int;
}
extern "C" {
    pub fn fc_destroy_rport();
}
extern "C" {
    pub fn fc_setup_fcp() -> c_int;
}
extern "C" {
    pub fn fc_destroy_fcp();
}
//
// Internal libfc functions
//
extern "C" {
    pub fn fc_fc4_add_lport(: *mut fc_lport);
}
extern "C" {
    pub fn fc_fc4_del_lport(: *mut fc_lport);
}
extern "C" {
    pub fn fc_fc4_conf_lport_params(: *mut fc_lport, fc_fh_type: enum);
}
//
// Copies a buffer into an sg list
//
