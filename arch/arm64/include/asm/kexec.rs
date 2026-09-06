//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/kexec.h
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
// kexec for arm64
//
// Copyright (C) Linaro.
// Copyright (C) Huawei Futurewei Technologies.
//
// Maximum physical address we can use pages from

// Maximum address we can reach in physical address mode

// Maximum address we can use for the control code buffer

pub const KEXEC_CONTROL_PAGE_SIZE: c_int = 4096;

//
// crash_setup_regs() - save registers for the panic kernel
//
// @newregs: registers are saved here
// @oldregs: registers to be saved (may be %NULL)
//
// pc

extern "C" {
    pub fn crash_is_nosave(pfn: c_ulong) -> bool;
}
extern "C" {
    pub fn crash_prepare_suspend();
}
extern "C" {
    pub fn crash_post_resume();
}
extern "C" {
    pub fn crash_free_reserved_phys_range(begin: c_ulong, end: c_ulong);
}

extern "C" {
    pub fn machine_kexec_post_load(image: *mut kimage) -> c_int;
}

// Macro flag: #define ARCH_HAS_KIMAGE_ARCH
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kimage_arch {
    pub dtb: *mut c_void,
    pub dtb_mem: phys_addr_t,
    pub kern_reloc: phys_addr_t,
    pub el2_vectors: phys_addr_t,
    pub ttbr0: phys_addr_t,
    pub ttbr1: phys_addr_t,
    pub zero_page: phys_addr_t,
    pub phys_offset: c_ulong,
    pub t0sz: c_ulong,
}

extern "C" {
    pub fn arch_kimage_file_post_load_cleanup(image: *mut kimage) -> c_int;
}

