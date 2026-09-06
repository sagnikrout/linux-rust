//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/comedi/drivers/amplc_pc236.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// comedi/drivers/amplc_pc236.h
// Header for "amplc_pc236", "amplc_pci236" and "amplc_pc236_common".
//
// Copyright (C) 2002-2014 MEV Ltd. <https://www.mev.co.uk/>
//
// COMEDI - Linux Control and Measurement Device Interface
// Copyright (C) 2000 David A. Schleef <ds@schleef.org>
//

// Macro flag: #define AMPLC_PC236_H_INCLUDED

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pc236_board {
    pub name: *const c_char,
    pub enable): *mut *mut *mut void (intr_update_cb)(struct comedi_device dev, bool,
    pub dev): *mut *mut bool (intr_chk_clr_cb)(struct comedi_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pc236_private {
    pub /: *mut *mut unsigned long lcr_iobase; / PLX PCI9052 config registers in PCIBAR1,
    pub enable_irq: bool,
}
