//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/ipl.h
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
// s390 (re)ipl support
//
// Copyright IBM Corp. 2007
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipl_parameter_block {
    pub hdr: ipl_pl_hdr,
    pub pb0_hdr: ipl_pb_hdr,
    pub common: ipl_pb0_common,
    pub fcp: ipl_pb0_fcp,
    pub ccw: ipl_pb0_ccw,
    pub eckd: ipl_pb0_eckd,
    pub nvme: ipl_pb0_nvme,
    pub ipl_pl_hdr)]: char raw[PAGE_SIZE - sizeof(struct,
}

pub const NSS_NAME_SIZE: c_int = 8;

extern "C" {
    pub fn save_area_alloc(is_boot_cpu: bool) -> *mut save_area  __init;
}
extern "C" {
    pub fn save_area_boot_cpu() -> *mut save_area  __init;
}
extern "C" {
    pub fn save_area_add_regs(: *mut save_area, regs: *mut c_void) -> void __init;
}
extern "C" {
    pub fn save_area_add_vxrs(: *mut save_area, vxrs: *mut __vector128) -> void __init;
}
extern "C" {
    pub fn s390_reset_system();
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipl_type {
    IPL_TYPE_UNKNOWN	= 1,
    IPL_TYPE_CCW		= 2,
    IPL_TYPE_FCP		= 4,
    IPL_TYPE_FCP_DUMP	= 8,
    IPL_TYPE_NSS		= 16,
    IPL_TYPE_NVME		= 32,
    IPL_TYPE_NVME_DUMP	= 64,
    IPL_TYPE_ECKD		= 128,
    IPL_TYPE_ECKD_DUMP	= 256,
}

extern "C" {
    pub fn setup_ipl();
}
extern "C" {
    pub fn set_os_info_reipl_block();
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipl_report {
    pub ipib: *mut ipl_parameter_block,
    pub components: list_head,
    pub certificates: list_head,
    pub size: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipl_report_component {
    pub list: list_head,
    pub entry: ipl_rb_component_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipl_report_certificate {
    pub list: list_head,
    pub entry: ipl_rb_certificate_entry,
    pub key: *mut c_void,
}

extern "C" {
    pub fn ipl_report_free(report: *mut ipl_report) -> c_int;
}
//
// DIAG 308 support
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum diag308_subcode {
    DIAG308_CLEAR_RESET = 0,
    DIAG308_LOAD_NORMAL_RESET = 1,
    DIAG308_REL_HSA = 2,
    DIAG308_LOAD_CLEAR = 3,
    DIAG308_LOAD_NORMAL_DUMP = 4,
    DIAG308_SET = 5,
    DIAG308_STORE = 6,
    DIAG308_LOAD_NORMAL = 7,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum diag308_subcode_flags {
    DIAG308_FLAG_EI = 1UL << 16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum diag308_rc {
    DIAG308_RC_OK		= 0x0001,
    DIAG308_RC_NOCONFIG	= 0x0102,
}

extern "C" {
    pub fn diag308(subcode: c_ulong, addr: *mut c_void) -> c_int;
}
extern "C" {
    pub fn store_status(): *mut *mut void (fn)(void, data: *mut c_void);
}
extern "C" {
    pub fn lgr_info_log();
}
