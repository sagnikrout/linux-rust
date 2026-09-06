//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/elx/efct/efct_driver.h
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
//
// Copyright (C) 2021 Broadcom. All Rights Reserved. The term
// “Broadcom” refers to Broadcom Inc. and/or its subsidiaries.
//

//
// OS specific includes
//

// EFCT_DEFAULT_FILTER-
// MRQ filter to segregate the IO flow.
//

// EFCT_OS_MAX_ISR_TIME_MSEC -
// maximum time driver code should spend in an interrupt
// or kernel thread context without yielding
//
pub const EFCT_OS_MAX_ISR_TIME_MSEC: c_int = 1000;
pub const EFCT_FC_MAX_SGL: c_int = 64;
pub const EFCT_FC_DIF_SEED: c_int = 0;
// Watermark
pub const EFCT_WATERMARK_HIGH_PCT: c_int = 90;
pub const EFCT_WATERMARK_LOW_PCT: c_int = 80;
pub const EFCT_IO_WATERMARK_PER_INITIATOR: c_int = 8;
pub const EFCT_PCI_MAX_REGS: c_int = 6;
pub const MAX_PCI_INTERRUPTS: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efct_intr_context {
    pub efct: *mut efct,
    pub index: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efct {
    pub pci: *mut pci_dev,
    pub reg: [*mut void __iomem; EFCT_PCI_MAX_REGS],
    pub n_msix_vec: u32,
    pub attached: bool,
    pub soft_wwn_enable: bool,
    pub efct_req_fw_upgrade: u8,
    pub intr_context: [efct_intr_context; MAX_PCI_INTERRUPTS],
    pub numa_node: u32,
    pub name: [c_char; EFC_NAME_LENGTH],
    pub instance_index: u32,
    pub list_entry: list_head,
    pub tgt_efct: efct_scsi_tgt,
    pub xport: *mut efct_xport,
    pub efcport: *mut efc,
    pub shost: *mut Scsi_Host,
    pub logmask: c_int,
    pub max_isr_time_msec: u32,
    pub desc: *const c_char,
    pub model: *const c_char,
    pub hw: efct_hw,
    pub rq_selection_policy: u32,
    pub filter_def: *mut c_char,
    pub topology: c_int,
// Look up for target node
    pub lookup: xarray,
//
// Target IO timer value:
// Zero: target command timeout disabled.
// Non-zero: Timeout value, in seconds, for target commands
//
    pub target_io_timer_sec: u32,
    pub speed: c_int,
    pub sess_debugfs_dir: *mut dentry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct efct_fw_write_result {
    pub done: completion,
    pub status: c_int,
    pub actual_xfer: u32,
    pub change_status: u32,
}
