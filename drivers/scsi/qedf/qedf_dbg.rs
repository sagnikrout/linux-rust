//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/qedf/qedf_dbg.h
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
// QLogic FCoE Offload Driver
// Copyright (c) 2016-2018 Cavium Inc.
//

// Debug print level definitions
pub const QEDF_LOG_DEFAULT: c_uint = 0x1		/* Set default logging mask */;
pub const QEDF_LOG_INFO: c_uint = 0x2		/*;
// Informational logs,
// MAC address, WWPN, WWNN
//
pub const QEDF_LOG_DISC: c_uint = 0x4		/* Init, discovery, rport */;
pub const QEDF_LOG_LL2: c_uint = 0x8		/* LL2, VLAN logs */;
pub const QEDF_LOG_CONN: c_uint = 0x10		/* Connection setup, cleanup */;
pub const QEDF_LOG_EVT: c_uint = 0x20		/* Events, link, mtu */;
pub const QEDF_LOG_TIMER: c_uint = 0x40		/* Timer events */;
pub const QEDF_LOG_MP_REQ: c_uint = 0x80		/* Middle Path (MP) logs */;
pub const QEDF_LOG_SCSI_TM: c_uint = 0x100		/* SCSI Aborts, Task Mgmt */;
pub const QEDF_LOG_UNSOL: c_uint = 0x200		/* unsolicited event logs */;
pub const QEDF_LOG_IO: c_uint = 0x400		/* scsi cmd, completion */;
pub const QEDF_LOG_MQ: c_uint = 0x800		/* Multi Queue logs */;
pub const QEDF_LOG_BSG: c_uint = 0x1000		/* BSG logs */;
pub const QEDF_LOG_DEBUGFS: c_uint = 0x2000		/* debugFS logs */;
pub const QEDF_LOG_LPORT: c_uint = 0x4000		/* lport logs */;
pub const QEDF_LOG_ELS: c_uint = 0x8000		/* ELS logs */;
pub const QEDF_LOG_NPIV: c_uint = 0x10000		/* NPIV logs */;
pub const QEDF_LOG_SESS: c_uint = 0x20000		/* Connection setup, cleanup */;
pub const QEDF_LOG_TID: c_uint = 0x80000         /*;
// FW TID context acquire
// free
//
pub const QEDF_TRACK_TID: c_uint = 0x100000        /*;
// Track TID state. To be
// enabled only at module load
// and not run-time.
//
pub const QEDF_TRACK_CMD_LIST: c_uint = 0x300000        /*;
// Track active cmd list nodes,
// done with reference to TID,
// hence TRACK_TID also enabled.
//
pub const QEDF_LOG_NOTICE: c_uint = 0x40000000	/* Notice logs */;
pub const QEDF_LOG_WARN: c_uint = 0x80000000	/* Warning logs */;

// Debug context structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedf_dbg_ctx {
    pub host_no: c_uint,
    pub pdev: *mut pci_dev,

    pub bdf_dentry: *mut dentry,

}

// GRC Dump related defines
pub const QEDF_UEVENT_CODE_GRCDUMP: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sysfs_bin_attrs {
    pub name: *mut c_char,
    pub attr: *const bin_attribute,
}

extern "C" {
    pub fn qedf_alloc_grc_dump_buf(buf: *mut u8, len: u32) -> c_int;
}
extern "C" {
    pub fn qedf_free_grc_dump_buf(buf: *mut u8);
}
extern "C" {
    pub fn qedf_uevent_emit(shost: *mut Scsi_Host, code: u32, msg: *mut c_char);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedf_debugfs_ops {
    pub name: *mut c_char,
    pub qedf_funcs: *mut qedf_list_of_funcs,
}

// DebugFS related code
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qedf_list_of_funcs {
    pub oper_str: *mut c_char,
    pub qedf): *mut *mut ssize_t (oper_func)(struct qedf_dbg_ctx,
}

// Used for debugfs sequential files

extern "C" {
    pub fn qedf_dbg_host_exit(qedf: *mut qedf_dbg_ctx);
}
extern "C" {
    pub fn qedf_dbg_init(drv_name: *mut c_char);
}
extern "C" {
    pub fn qedf_dbg_exit();
}

