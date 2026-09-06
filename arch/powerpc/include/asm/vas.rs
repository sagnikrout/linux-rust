//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/vas.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright 2016-17 IBM Corp.
//

//
// Min and max FIFO sizes are based on Version 1.05 Section 3.1.4.25
// (Local FIFO Size Register) of the VAS workbook.
//

//
// Threshold Control Mode: Have paste operation fail if the number of
// requests in receive FIFO exceeds a threshold.
//
// NOTE: No special error code yet if paste is rejected because of these
// limits. So users can't distinguish between this and other errors.
//
pub const VAS_THRESH_DISABLED: c_int = 0;
pub const VAS_THRESH_FIFO_GT_HALF_FULL: c_int = 1;
pub const VAS_THRESH_FIFO_GT_QTR_FULL: c_int = 2;
pub const VAS_THRESH_FIFO_GT_EIGHTH_FULL: c_int = 3;
//
// VAS window Linux status bits
//
pub const VAS_WIN_ACTIVE: c_uint = 0x0	/* Used in platform independent */;
// vas mmap()
// Window is closed in the hypervisor due to lost credit
pub const VAS_WIN_NO_CRED_CLOSE: c_uint = 0x00000001;
// Window is closed due to migration
pub const VAS_WIN_MIGRATE_CLOSE: c_uint = 0x00000002;
//
// Get/Set bit fields
//

//
// Co-processor Engine type.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vas_cop_type {
    VAS_COP_TYPE_FAULT,
    VAS_COP_TYPE_842,
    VAS_COP_TYPE_842_HIPRI,
    VAS_COP_TYPE_GZIP,
    VAS_COP_TYPE_GZIP_HIPRI,
    VAS_COP_TYPE_FTW,
    VAS_COP_TYPE_MAX,
}

//
// User space VAS windows are opened by tasks and take references
// to pid and mm until windows are closed.
// Stores pid, mm, and tgid for each window.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vas_user_win_ref {
    pub /: *mut *mut *mut pid pid; / PID of owner,
    pub /: *mut *mut *mut pid tgid; / Thread group ID of owner,
    pub /: *mut *mut *mut mm_mm; / Linux process mm_struct,
    pub /: *mut *mut mutex mmap_mutex; / protects paste address mmap(),
// with DLPAR close/open windows
    pub /: *mut *mut *mut vm_area_vma; / Save VMA and used in DLPAR ops,
}

//
// Common VAS window struct on PowerNV and PowerVM
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vas_window {
    pub winid: u32,
    pub /: *mut *mut u32 wcreds_max; / Window credits,
    pub /: *mut *mut u32 status; / Window status used in OS,
    pub cop: vas_cop_type,
    pub task_ref: vas_user_win_ref,
    pub dbgname: *mut c_char,
    pub dbgdir: *mut dentry,
}

//
// User space window operations used for powernv and powerVM
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vas_user_win_ops {
    pub vas_cop_type): enum,
    pub ): *mut *mut u64 (paste_addr)(struct vas_window,
    pub ): *mut *mut int (close_win)(struct vas_window,
}

// Drop references to pid, tgid, and mm
//
// Even a process that has no foreign real address mapping can
// use an unpaired COPY instruction (to no real effect). Issue
// CP_ABORT to clear any pending COPY and prevent a covert
// channel.
//
// __switch_to() will issue CP_ABORT on future context switches
// if process / thread has any open VAS window (Use
// current->mm->context.vas_windows).
//
extern "C" {
    pub fn volatile(_arg: PPC_CP_ABORT) -> asm;
}
//
// Receive window attributes specified by the (in-kernel) owner of window.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vas_rx_win_attr {
    pub rx_fifo: u64,
    pub rx_fifo_size: c_int,
    pub wcreds_max: c_int,
    pub pin_win: bool,
    pub rej_no_credit: bool,
    pub tx_wcred_mode: bool,
    pub rx_wcred_mode: bool,
    pub tx_win_ord_mode: bool,
    pub rx_win_ord_mode: bool,
    pub data_stamp: bool,
    pub nx_win: bool,
    pub fault_win: bool,
    pub user_win: bool,
    pub notify_disable: bool,
    pub intr_disable: bool,
    pub notify_early: bool,
    pub lnotify_lpid: c_int,
    pub lnotify_pid: c_int,
    pub lnotify_tid: c_int,
    pub pswid: u32,
    pub tc_mode: c_int,
}

//
// Window attributes specified by the in-kernel owner of a send window.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vas_tx_win_attr {
    pub cop: vas_cop_type,
    pub wcreds_max: c_int,
    pub lpid: c_int,
    pub /: *mut *mut int pidr; / hardware PID (from SPRN_PID),
    pub pswid: c_int,
    pub rsvd_txbuf_count: c_int,
    pub tc_mode: c_int,
    pub user_win: bool,
    pub pin_win: bool,
    pub rej_no_credit: bool,
    pub rsvd_txbuf_enable: bool,
    pub tx_wcred_mode: bool,
    pub rx_wcred_mode: bool,
    pub tx_win_ord_mode: bool,
    pub rx_win_ord_mode: bool,
}

//
// Helper to map a chip id to VAS id.
// For POWER9, this is a 1:1 mapping. In the future this maybe a 1:N
// mapping in which case, we will need to update this helper.
//
// Return the VAS id or -1 if no matching vasid is found.
//
extern "C" {
    pub fn chip_to_vas_id(chipid: c_int) -> c_int;
}
//
// Helper to initialize receive window attributes to defaults for an
// NX window.
//
extern "C" {
    pub fn vas_init_rx_win_attr(rxattr: *mut vas_rx_win_attr, cop: vas_cop_type);
}
//
// Open a VAS receive window for the instance of VAS identified by @vasid
// Use @attr to initialize the attributes of the window.
//
// Return a handle to the window or ERR_PTR() on error.
//
// Helper to initialize send window attributes to defaults for an NX window.
//
// Open a VAS send window for the instance of VAS identified by @vasid
// and the co-processor type @cop. Use @attr to initialize attributes
// of the window.
//
// Note: The instance of VAS must already have an open receive window for
// the coprocessor type @cop.
//
// Return a handle to the send window or ERR_PTR() on error.
//
// Close the send or receive window identified by @win. For receive windows
// return -EAGAIN if there are active send windows attached to this receive
// window.
//
extern "C" {
    pub fn vas_win_close(win: *mut vas_window) -> c_int;
}
//
// Copy the co-processor request block (CRB) @crb into the local L2 cache.
//
extern "C" {
    pub fn vas_copy_crb(crb: *mut c_void, offset: c_int) -> c_int;
}
//
// Paste a previously copied CRB (see vas_copy_crb()) from the L2 cache to
// the hardware address associated with the window @win. @re is expected
// assumed to be true for NX windows.
//
extern "C" {
    pub fn vas_paste_crb(win: *mut vas_window, offset: c_int, re: bool) -> c_int;
}
extern "C" {
    pub fn vas_unregister_api_powernv();
}

// VAS Capabilities
pub const VAS_GZIP_QOS_FEAT: c_uint = 0x1;
pub const VAS_GZIP_DEF_FEAT: c_uint = 0x2;

// NX Capabilities
pub const VAS_NX_GZIP_FEAT: c_uint = 0x1;

//
// These structs are used to retrieve overall VAS capabilities that
// the hypervisor provides.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_vas_all_caps {
    pub descriptor: __be64,
    pub feat_type: __be64,
    pub __aligned(0x1000): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vas_all_caps {
    pub descriptor: u64,
    pub feat_type: u64,
}

extern "C" {
    pub fn h_query_vas_capabilities(hcall: u64, query_type: u8, result: u64) -> c_int;
}
extern "C" {
    pub fn vas_unregister_api_pseries();
}

//
// Register / unregister coprocessor type to VAS API which will be exported
// to user space. Applications can use this API to open / close window
// which can be used to send / receive requests directly to cooprcessor.
//
// Only NX GZIP coprocessor type is supported now, but this API can be
// used for others in future.
//
extern "C" {
    pub fn vas_unregister_coproc_api();
}
extern "C" {
    pub fn get_vas_user_win_ref(task_ref: *mut vas_user_win_ref) -> c_int;
}
extern "C" {
    pub fn vas_dump_crb(crb: *mut coprocessor_request_block);
}
