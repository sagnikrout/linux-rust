//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/da9063/core.h
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
// Definitions for DA9063 MFD driver
//
// Copyright 2012 Dialog Semiconductor Ltd.
//
// Author: Michal Hajduk, Dialog Semiconductor
// Author: Krystian Garbaciak, Dialog Semiconductor
//

// DA9063 modules

pub const PMIC_CHIP_ID_DA9063: c_uint = 0x61;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum da9063_type {
    PMIC_TYPE_DA9063 = 0,
    PMIC_TYPE_DA9063L,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum da9063_variant_codes {
    PMIC_DA9063_AD = 0x3,
    PMIC_DA9063_BB = 0x5,
    PMIC_DA9063_CA = 0x6,
    PMIC_DA9063_DA = 0x7,
    PMIC_DA9063_EA = 0x8,
}

// Interrupts
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum da9063_irqs {
    DA9063_IRQ_ONKEY = 0,
    DA9063_IRQ_ALARM,
    DA9063_IRQ_TICK,
    DA9063_IRQ_ADC_RDY,
    DA9063_IRQ_SEQ_RDY,
    DA9063_IRQ_WAKE,
    DA9063_IRQ_TEMP,
    DA9063_IRQ_COMP_1V2,
    DA9063_IRQ_LDO_LIM,
    DA9063_IRQ_REG_UVOV,
    DA9063_IRQ_DVC_RDY,
    DA9063_IRQ_VDD_MON,
    DA9063_IRQ_WARN,
    DA9063_IRQ_GPI0,
    DA9063_IRQ_GPI1,
    DA9063_IRQ_GPI2,
    DA9063_IRQ_GPI3,
    DA9063_IRQ_GPI4,
    DA9063_IRQ_GPI5,
    DA9063_IRQ_GPI6,
    DA9063_IRQ_GPI7,
    DA9063_IRQ_GPI8,
    DA9063_IRQ_GPI9,
    DA9063_IRQ_GPI10,
    DA9063_IRQ_GPI11,
    DA9063_IRQ_GPI12,
    DA9063_IRQ_GPI13,
    DA9063_IRQ_GPI14,
    DA9063_IRQ_GPI15,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct da9063 {
// Device
    pub dev: *mut device,
    pub type: da9063_type,
    pub variant_code: c_uchar,
    pub flags: c_uint,
    pub use_sw_pm: bool,
// Control interface
    pub regmap: *mut regmap,
// Interrupts
    pub chip_irq: c_int,
    pub irq_base: c_uint,
    pub regmap_irq: *mut regmap_irq_chip_data,
}

extern "C" {
    pub fn da9063_device_init(da9063: *mut da9063, irq: c_uint) -> c_int;
}
extern "C" {
    pub fn da9063_irq_init(da9063: *mut da9063) -> c_int;
}
