//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/kexec.h
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
// Copyright IBM Corp. 2005
//
// Author(s): Rolf Adelsberger <adelsberger@de.ibm.com>
//

//
// KEXEC_SOURCE_MEMORY_LIMIT maximum page get_free_page can return.
// I.e. Maximum page that is mapped directly into kernel memory,
// and kmap is not required.
//
// Maximum physical address we can use pages from

// Maximum address we can reach in physical address mode

// Maximum address we can use for the control pages
// Not more than 2GB

// Allocate control page with GFP_DMA

// Maximum address we can use for the crash control pages

// Allocate one page for the pdp and the second for the code
pub const KEXEC_CONTROL_PAGE_SIZE: c_int = 4096;
// Alignment of crashkernel memory

// The native architecture

// Allow kexec_file to load a segment to 0

// Provide a dummy definition to avoid build failures.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct s390_load_data {
// Pointer to the kernel buffer. Used to register cmdline etc..
    pub kernel_buf: *mut c_void,
// Load address of the kernel_buf.
    pub kernel_mem: c_ulong,
// Parmarea in the kernel buffer.
    pub parm: *mut parmarea,
// Total size of loaded segments in memory. Used as an offset.
    pub memsz: usize,
    pub report: *mut ipl_report,
}

extern "C" {
    pub fn s390_verify_sig(kernel: *const c_char, kernel_len: c_ulong) -> c_int;
}
// Macro flag: #define ARCH_HAS_KIMAGE_ARCH
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kimage_arch {
    pub ipl_buf: *mut c_void,
}

extern "C" {
    pub fn crash_free_reserved_phys_range(begin: c_ulong, end: c_ulong);
}

extern "C" {
    pub fn arch_kexec_protect_crashkres();
}

extern "C" {
    pub fn arch_kexec_unprotect_crashkres();
}

extern "C" {
    pub fn is_kdump_kernel() -> bool;
}

extern "C" {
    pub fn arch_kimage_file_post_load_cleanup(image: *mut kimage) -> c_int;
}

