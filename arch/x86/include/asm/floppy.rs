//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/floppy.h
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

//
// The DMA channel used by the floppy controller cannot access data at
// addresses >= 16MB
//
// Went back to the 1MB limit, as some people had problems with the floppy
// driver otherwise. It doesn't matter much for performance anyway, as most
// floppy accesses go through the track buffer.
//

// Macro flag: #define FLOPPY_CAN_FALLBACK_ON_NODMA

extern "C" {
    pub fn floppy_interrupt(_arg: irq, _arg: dev_id) -> return;
}

// lptr = fd_inb(virtual_dma_port, FD_DATA);

extern "C" {
    pub fn __get_dma_pages(_arg: GFP_KERNEL|__GFP_NORETRY, _arg: get_order(size)) -> return;
}

// actual, physical DMA
extern "C" {
    pub fn long(size: *mut *mut _dma_mem_alloc)(unsigned long) -> unsigned;
}
//
// Floppy types are stored in the rtc's CMOS RAM and so rtc_lock
// is needed to prevent corrupted CMOS RAM in case "insmod floppy"
// coincides with another rtc CMOS user.		Paul G.
//

pub const N_FDC: c_int = 2;
pub const N_DRIVE: c_int = 8;
// Macro flag: #define EXTRA_FLOPPY_PARAMS
