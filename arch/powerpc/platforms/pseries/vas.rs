//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/platforms/pseries/vas.h
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
// Copyright 2020-21 IBM Corp.
//

//
// VAS window modify flags
//

pub const VAS_WIN_ACTIVE: c_uint = 0x0;
pub const VAS_WIN_CLOSED: c_uint = 0x1;
pub const VAS_WIN_INACTIVE: c_uint = 0x2	/* Inactive due to HW failure */;
// Process of being modified, deallocated, or quiesced
pub const VAS_WIN_MOD_IN_PROCESS: c_uint = 0x3;
pub const VAS_COPY_PASTE_USER_MODE: c_uint = 0x00000001;
pub const VAS_COP_OP_USER_MODE: c_uint = 0x00000010;
pub const VAS_GZIP_QOS_CAPABILITIES: c_uint = 0x56516F73477A6970;
pub const VAS_GZIP_DEFAULT_CAPABILITIES: c_uint = 0x56446566477A6970;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vas_migrate_action {
    VAS_SUSPEND,
    VAS_RESUME,
}

//
// Co-processor feature - GZIP QoS windows or GZIP default windows
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vas_cop_feat_type {
    VAS_GZIP_QOS_FEAT_TYPE,
    VAS_GZIP_DEF_FEAT_TYPE,
    VAS_MAX_FEAT_TYPE,
}

//
// Use to get feature specific capabilities from the
// hypervisor.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_vas_cop_feat_caps {
    pub descriptor: __be64,
    pub /: *mut *mut u8 win_type; / Default or QoS type,
    pub user_mode: u8,
    pub max_lpar_creds: __be16,
    pub max_win_creds: __be16,
    pub reserved: __be16,
    pub /: *mut *mut __be16 def_lpar_creds; / Used for default capabilities,
}

//
// Feature specific (QoS or default) capabilities.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vas_cop_feat_caps {
    pub descriptor: u64,
    pub /: *mut *mut u8 win_type; / Default or QoS type,
    pub /: *mut *mut u8 user_mode; / User mode copy/paste or COP HCALL,
    pub /: *mut *mut u16 max_lpar_creds; / Max credits available in LPAR,
// Max credits can be assigned per window
    pub max_win_creds: u16,
    pub /: *mut *mut u16 reserved; / Used for QoS credit type,
    pub /: *mut *mut u16 def_lpar_creds; / Used for default credit type,
}

// Total LPAR available credits. Can be different from max LPAR
// credits due to DLPAR operation
//
// Feature (QoS or Default) specific to store capabilities and
// the list of open windows.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vas_caps {
    pub caps: vas_cop_feat_caps,
    pub /: *mut *mut list_head list; / List of open windows,
    pub /: *mut *mut int nr_open_wins_progress; / Number of open windows in,
// progress. Used in migration
    pub /: *mut *mut int nr_close_wins; / closed windows in the hypervisor for DLPAR,
    pub /: *mut *mut int nr_open_windows; / Number of successful open windows,
    pub /: *mut *mut u8 feat; / Feature type,
}

//
// To get window information from the hypervisor.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_vas_win_lpar {
    pub version: __be16,
    pub win_type: u8,
    pub status: u8,
    pub /: *mut *mut __be16 credits; / No of credits assigned to this window,
    pub reserved: __be16,
    pub /: *mut *mut __be32 pid; / LPAR Process ID,
    pub /: *mut *mut __be32 tid; / LPAR Thread ID,
    pub /: *mut *mut __be64 win_addr; / Paste address,
    pub /: *mut *mut __be32 interrupt; / Interrupt when NX request completes,
    pub /: *mut *mut __be32 fault; / Interrupt when NX sees fault,
// Associativity Domain Identifiers as returned in
// H_HOME_NODE_ASSOCIATIVITY
    pub domain: [__be64; 6],
    pub /: *mut *mut __be64 win_util; / Number of bytes processed,
    pub __aligned(0x1000): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pseries_vas_window {
    pub vas_win: vas_window,
    pub /: *mut *mut u64 win_addr; / Physical paste address,
    pub /: *mut *mut u8 win_type; / QoS or Default window,
    pub /: *mut *mut u32 complete_irq; / Completion interrupt,
    pub /: *mut *mut u32 fault_irq; / Fault interrupt,
    pub /: *mut *mut u64 domain[6]; / Associativity domain Ids,
// this window is allocated
    pub util: u64,
    pub /: *mut *mut u32 pid; / PID associated with this window,
// List of windows opened which is used for LPM
    pub win_list: list_head,
    pub flags: u64,
    pub name: *mut c_char,
    pub fault_virq: c_int,
    pub /: *mut *mut atomic_t pending_faults; / Number of pending faults,
}

extern "C" {
    pub fn sysfs_add_vas_caps(caps: *mut vas_cop_feat_caps) -> c_int;
}
extern "C" {
    pub fn vas_reconfig_capabilties(type: u8, new_nr_creds: c_int) -> c_int;
}
extern "C" {
    pub fn sysfs_pseries_vas_init(vas_caps: *mut vas_all_caps) -> int __init;
}

extern "C" {
    pub fn vas_migration_handler(action: c_int) -> c_int;
}
extern "C" {
    pub fn pseries_vas_dlpar_cpu() -> c_int;
}

