//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/microcode.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_signature {
    pub sig: c_uint,
    pub pf: c_uint,
    pub rev: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucode_cpu_info {
    pub cpu_sig: cpu_signature,
    pub mc: *mut c_void,
}

extern "C" {
    pub fn load_ucode_bsp();
}
extern "C" {
    pub fn load_ucode_ap();
}
extern "C" {
    pub fn microcode_bsp_resume();
}
extern "C" {
    pub fn microcode_loader_disabled() -> bool __init;
}

// Intel specific microcode defines. Public for IFS
#[repr(C)]
#[derive(Copy, Clone)]
pub struct microcode_header_intel {
    pub hdrver: c_uint,
    pub rev: c_uint,
    pub date: c_uint,
    pub sig: c_uint,
    pub cksum: c_uint,
    pub ldrver: c_uint,
    pub pf: c_uint,
    pub datasize: c_uint,
    pub totalsize: c_uint,
    pub metasize: c_uint,
    pub min_req_ver: c_uint,
    pub reserved: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct microcode_intel {
    pub hdr: microcode_header_intel,
    pub bits: [c_uint; ],
}

pub const MC_HEADER_TYPE_MICROCODE: c_int = 1;
pub const MC_HEADER_TYPE_IFS: c_int = 2;
// As documented in the SDM: Do a CPUID 1 here
// get the current revision from MSR 0x8B

extern "C" {
    pub fn microcode_nmi_handler() -> bool;
}
extern "C" {
    pub fn microcode_offline_nmi_handler();
}

extern "C" {
    pub fn static_branch_unlikely(_arg: &microcode_nmi_handler_enable) -> return;
}

