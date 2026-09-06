//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/da9062/core.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2015-2017  Dialog Semiconductor
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum da9062_compatible_types {
    COMPAT_TYPE_DA9061 = 1,
    COMPAT_TYPE_DA9062,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum da9061_irqs {
// IRQ A
    DA9061_IRQ_ONKEY,
    DA9061_IRQ_WDG_WARN,
    DA9061_IRQ_SEQ_RDY,
// IRQ B
    DA9061_IRQ_TEMP,
    DA9061_IRQ_LDO_LIM,
    DA9061_IRQ_DVC_RDY,
    DA9061_IRQ_VDD_WARN,
// IRQ C
    DA9061_IRQ_GPI0,
    DA9061_IRQ_GPI1,
    DA9061_IRQ_GPI2,
    DA9061_IRQ_GPI3,
    DA9061_IRQ_GPI4,

    DA9061_NUM_IRQ,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum da9062_irqs {
// IRQ A
    DA9062_IRQ_ONKEY,
    DA9062_IRQ_ALARM,
    DA9062_IRQ_TICK,
    DA9062_IRQ_WDG_WARN,
    DA9062_IRQ_SEQ_RDY,
// IRQ B
    DA9062_IRQ_TEMP,
    DA9062_IRQ_LDO_LIM,
    DA9062_IRQ_DVC_RDY,
    DA9062_IRQ_VDD_WARN,
// IRQ C
    DA9062_IRQ_GPI0,
    DA9062_IRQ_GPI1,
    DA9062_IRQ_GPI2,
    DA9062_IRQ_GPI3,
    DA9062_IRQ_GPI4,

    DA9062_NUM_IRQ,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct da9062 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub regmap_irq: *mut regmap_irq_chip_data,
    pub chip_type: da9062_compatible_types,
}
