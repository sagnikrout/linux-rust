//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/ehv_pic.h
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
// EHV_PIC private definitions and structure.
//
// Copyright 2008-2010 Freescale Semiconductor, Inc.
//
// This file is licensed under the terms of the GNU General Public License
// version 2.  This program is licensed "as is" without any warranty of any
// kind, whether express or implied.
//

pub const NR_EHV_PIC_INTS: c_int = 1024;

pub const EHV_PIC_VECPRI_POLARITY_NEGATIVE: c_int = 0;
pub const EHV_PIC_VECPRI_POLARITY_POSITIVE: c_int = 1;
pub const EHV_PIC_VECPRI_SENSE_EDGE: c_int = 0;
pub const EHV_PIC_VECPRI_SENSE_LEVEL: c_uint = 0x2;
pub const EHV_PIC_VECPRI_POLARITY_MASK: c_uint = 0x1;
pub const EHV_PIC_VECPRI_SENSE_MASK: c_uint = 0x2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ehv_pic {
// The remapper for this EHV_PIC
    pub irqhost: *mut irq_domain,
// The "linux" controller struct
    pub hc_irq: irq_chip,
// core int flag
    pub coreint_flag: c_int,
}

extern "C" {
    pub fn ehv_pic_init();
}
extern "C" {
    pub fn ehv_pic_get_irq() -> c_uint;
}
