//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/i3c/master/mipi-i3c-hci/hci.h
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


// SPDX-License-Identifier: BSD-3-Clause
//
// Copyright (c) 2020, MIPI Alliance, Inc.
//
// Author: Nicolas Pitre <npitre@baylibre.com>
//
// Common HCI stuff
//

// 32-bit word aware bit and mask macros

// Same for single bit macros (trailing _ to align with W*_MASK width)

// helper macro for HCI version check

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dat_words {
    pub w0: u32,
    pub w1: u32,
}

// Our main structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i3c_hci {
    pub master: i3c_master_controller,
    pub base_regs: *mut void __iomem,
    pub DAT_regs: *mut void __iomem,
    pub DCT_regs: *mut void __iomem,
    pub RHS_regs: *mut void __iomem,
    pub PIO_regs: *mut void __iomem,
    pub EXTCAPS_regs: *mut void __iomem,
    pub AUTOCMD_regs: *mut void __iomem,
    pub DEBUG_regs: *mut void __iomem,
    pub io: *const hci_io_ops,
    pub io_data: *mut c_void,
    pub cmd: *const hci_cmd_ops,
    pub lock: spinlock_t,
    pub control_mutex: mutex,
    pub next_cmd_tid: core::sync::atomic::AtomicI32,
    pub irq_inactive: bool,
    pub enqueue_blocked: bool,
    pub recovery_needed: bool,
    pub hj_init_done: bool,
    pub enqueue_wait_queue: wait_queue_head_t,
    pub caps: u32,
    pub quirks: c_uint,
    pub DAT_entries: c_uint,
    pub DAT_entry_size: c_uint,
    pub DAT_data: *mut c_void,
    pub DAT: *mut dat_words,
    pub ibi_devs: *mut i3c_dev_desc,
    pub DCT_entries: c_uint,
    pub DCT_entry_size: c_uint,
    pub version_major: u8,
    pub version_minor: u8,
    pub revision: u8,
    pub dyn_addr: u8,
    pub vendor_mipi_id: u32,
    pub vendor_version_id: u32,
    pub vendor_product_id: u32,
    pub vendor_data: *mut c_void,
}

//
// Structure to represent a master initiated transfer.
// The rnw, data and data_len fields must be initialized before calling any
// hci->cmd->*() method. The cmd method will initialize cmd_desc[] and
// possibly modify (clear) the data field. Then xfer->cmd_desc[0] can
// be augmented with CMD_0_ROC and/or CMD_0_TOC.
// The completion field needs to be initialized before queueing with
// hci->io->queue_xfer(), and requires CMD_0_ROC to be set.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_xfer {
    pub cmd_desc: [u32; 4],
    pub response: u32,
    pub rnw: bool,
    pub started: bool,
    pub data: *mut c_void,
    pub data_len: c_uint,
    pub cmd_tid: c_uint,
    pub completion: *mut completion,
    pub timeout: c_ulong,
    pub start_jiffies: c_ulong,
// PIO specific
    pub next_xfer: *mut hci_xfer,
    pub next_data: *mut hci_xfer,
    pub next_resp: *mut hci_xfer,
    pub data_left: c_uint,
    pub data_word_before_partial: u32,
}

// DMA specific
extern "C" {
    pub fn kzalloc_objs(hci_xfer: struct, _arg: n) -> return;
}
// This abstracts PIO vs DMA operations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_io_ops {
    pub hci): *mut *mut bool (irq_handler)(struct i3c_hci,
    pub n): *mut *mut *mut *mut int (queue_xfer)(struct i3c_hci hci, struct hci_xfer xfer, int,
    pub n): *mut *mut *mut *mut bool (dequeue_xfer)(struct i3c_hci hci, struct hci_xfer xfer, int,
    pub n): *mut *mut *mut *mut int (handle_error)(struct i3c_hci hci, struct hci_xfer xfer, int,
    pub req): *const i3c_ibi_setup,
    pub dev): *mut *mut *mut void (free_ibi)(struct i3c_hci hci, struct i3c_dev_desc,
    pub slot): *mut i3c_ibi_slot,
    pub hci): *mut *mut int (init)(struct i3c_hci,
    pub hci): *mut *mut void (cleanup)(struct i3c_hci,
    pub hci): *mut *mut void (suspend)(struct i3c_hci,
    pub hci): *mut *mut void (resume)(struct i3c_hci,
}

// Our per device master private data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i3c_hci_dev_data {
    pub dat_idx: c_int,
    pub ibi_data: *mut c_void,
}

// list of quirks

// global functions
extern "C" {
    pub fn mipi_i3c_hci_resume(hci: *mut i3c_hci);
}
extern "C" {
    pub fn mipi_i3c_hci_abort(hci: *mut i3c_hci);
}
extern "C" {
    pub fn mipi_i3c_hci_pio_reset(hci: *mut i3c_hci);
}
extern "C" {
    pub fn mipi_i3c_hci_pio_reset_all_queues(hci: *mut i3c_hci);
}
extern "C" {
    pub fn mipi_i3c_hci_dct_index_reset(hci: *mut i3c_hci);
}
extern "C" {
    pub fn amd_set_od_pp_timing(hci: *mut i3c_hci);
}
extern "C" {
    pub fn amd_set_resp_buf_thld(hci: *mut i3c_hci);
}
extern "C" {
    pub fn i3c_hci_sync_irq_inactive(hci: *mut i3c_hci);
}
extern "C" {
    pub fn i3c_hci_process_xfer(hci: *mut i3c_hci, xfer: *mut hci_xfer, n: c_int) -> c_int;
}
pub const DEFAULT_AUTOSUSPEND_DELAY_MS: c_int = 1000;
extern "C" {
    pub fn i3c_hci_rpm_suspend(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn i3c_hci_rpm_resume(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn i3c_hci_reset_and_restore(hci: *mut i3c_hci) -> c_int;
}
