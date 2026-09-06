//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/i8259.h
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

// i8259A PIC registers
pub const PIC_MASTER_CMD: c_uint = 0x20;
pub const PIC_MASTER_IMR: c_uint = 0x21;

pub const PIC_SLAVE_CMD: c_uint = 0xa0;
pub const PIC_SLAVE_IMR: c_uint = 0xa1;
pub const PIC_ELCR1: c_uint = 0x4d0;
pub const PIC_ELCR2: c_uint = 0x4d1;
// i8259A PIC related value
pub const PIC_CASCADE_IR: c_int = 2;
pub const MASTER_ICW4_DEFAULT: c_uint = 0x01;
pub const SLAVE_ICW4_DEFAULT: c_uint = 0x01;
pub const PIC_ICW4_AEOI: c_int = 2;
// the PIC may need a careful delay on some platforms, hence specific calls
//
// delay for some accesses to PIC on motherboard or in chipset
// must be at least one microsecond, so be safe here:
//
// delay for some accesses to PIC on motherboard or in chipset
// must be at least one microsecond, so be safe here:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct legacy_pic {
    pub nr_legacy_irqs: c_int,
    pub chip: *mut irq_chip,
    pub irq): *mut *mut void (mask)(unsigned int,
    pub irq): *mut *mut void (unmask)(unsigned int,
    pub (*mask_all)(void): *mut c_void,
    pub (*restore_mask)(void): *mut c_void,
    pub auto_eoi): *mut *mut void (init)(int,
    pub (*probe)(void): *mut c_int,
    pub irq): *mut *mut int (irq_pending)(unsigned int,
    pub irq): *mut *mut void (make_irq)(unsigned int,
}

extern "C" {
    pub fn legacy_pic_pcat_compat();
}
