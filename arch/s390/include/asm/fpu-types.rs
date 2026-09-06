//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/fpu-types.h
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
// FPU data structures
//
// Copyright IBM Corp. 2015
// Author(s): Hendrik Brueckner <brueckner@linux.vnet.ibm.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpu {
    pub fpc: u32,
    pub __aligned(8): __vector128 vxrs[__NUM_VXRS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kernel_fpu_hdr {
    pub mask: c_int,
    pub fpc: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kernel_fpu {
    pub hdr: kernel_fpu_hdr,
    pub __aligned(8): __vector128 vxrs[],
}

