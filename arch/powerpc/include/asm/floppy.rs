//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/floppy.h
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


//
// Architecture specific parts of the Floppy driver
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file "COPYING" in the main directory of this archive
// for more details.
//
// Copyright (C) 1995
//

extern "C" {
    pub fn fd_request_dma() -> static int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fd_dma_ops {
    pub dmanr): *mut *mut void (_disable_dma)(unsigned int,
    pub dmanr): *mut *mut void (_free_dma)(unsigned int,
    pub dummy): *mut *mut int (_get_dma_residue)(unsigned int,
    pub io): *mut *mut *mut int (_dma_setup)(char addr, unsigned long size, int mode, int,
}

extern "C" {
    pub fn floppy_interrupt(_arg: irq, _arg: dev_id) -> return;
}
// lptr = inb_p(virtual_dma_port + FD_DATA);
// different from last time -- unmap prev
// remember this one as prev
extern "C" {
    pub fn request_dma(_arg: FLOPPY_DMA, _arg: "floppy") -> return;
}
//
// Again, the CMOS information not available
//
pub const FLOPPY0_TYPE: c_int = 6;
pub const FLOPPY1_TYPE: c_int = 0;

pub const N_DRIVE: c_int = 8;
// Macro flag: #define EXTRA_FLOPPY_PARAMS

