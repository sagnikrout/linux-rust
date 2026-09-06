//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/da9052/da9052.h
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
// da9052 declarations for DA9052 PMICs.
//
// Copyright(c) 2011 Dialog Semiconductor Ltd.
//
// Author: David Dajun Chen <dchen@diasemi.com>
//

// Common - HWMON Channel Definations
pub const DA9052_ADC_VDDOUT: c_int = 0;
pub const DA9052_ADC_ICH: c_int = 1;
pub const DA9052_ADC_TBAT: c_int = 2;
pub const DA9052_ADC_VBAT: c_int = 3;
pub const DA9052_ADC_IN4: c_int = 4;
pub const DA9052_ADC_IN5: c_int = 5;
pub const DA9052_ADC_IN6: c_int = 6;
pub const DA9052_ADC_TSI: c_int = 7;
pub const DA9052_ADC_TJUNC: c_int = 8;
pub const DA9052_ADC_VBBAT: c_int = 9;
// TSI channel has its own 4 channel mux
pub const DA9052_ADC_TSI_XP: c_int = 70;
pub const DA9052_ADC_TSI_XN: c_int = 71;
pub const DA9052_ADC_TSI_YP: c_int = 72;
pub const DA9052_ADC_TSI_YN: c_int = 73;
pub const DA9052_IRQ_DCIN: c_int = 0;
pub const DA9052_IRQ_VBUS: c_int = 1;
pub const DA9052_IRQ_DCINREM: c_int = 2;
pub const DA9052_IRQ_VBUSREM: c_int = 3;
pub const DA9052_IRQ_VDDLOW: c_int = 4;
pub const DA9052_IRQ_ALARM: c_int = 5;
pub const DA9052_IRQ_SEQRDY: c_int = 6;
pub const DA9052_IRQ_COMP1V2: c_int = 7;
pub const DA9052_IRQ_NONKEY: c_int = 8;
pub const DA9052_IRQ_IDFLOAT: c_int = 9;
pub const DA9052_IRQ_IDGND: c_int = 10;
pub const DA9052_IRQ_CHGEND: c_int = 11;
pub const DA9052_IRQ_TBAT: c_int = 12;
pub const DA9052_IRQ_ADC_EOM: c_int = 13;
pub const DA9052_IRQ_PENDOWN: c_int = 14;
pub const DA9052_IRQ_TSIREADY: c_int = 15;
pub const DA9052_IRQ_GPI0: c_int = 16;
pub const DA9052_IRQ_GPI1: c_int = 17;
pub const DA9052_IRQ_GPI2: c_int = 18;
pub const DA9052_IRQ_GPI3: c_int = 19;
pub const DA9052_IRQ_GPI4: c_int = 20;
pub const DA9052_IRQ_GPI5: c_int = 21;
pub const DA9052_IRQ_GPI6: c_int = 22;
pub const DA9052_IRQ_GPI7: c_int = 23;
pub const DA9052_IRQ_GPI8: c_int = 24;
pub const DA9052_IRQ_GPI9: c_int = 25;
pub const DA9052_IRQ_GPI10: c_int = 26;
pub const DA9052_IRQ_GPI11: c_int = 27;
pub const DA9052_IRQ_GPI12: c_int = 28;
pub const DA9052_IRQ_GPI13: c_int = 29;
pub const DA9052_IRQ_GPI14: c_int = 30;
pub const DA9052_IRQ_GPI15: c_int = 31;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum da9052_chip_id {
    DA9052,
    DA9053_AA,
    DA9053_BA,
    DA9053_BB,
    DA9053_BC,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct da9052 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub auxadc_lock: mutex,
    pub done: completion,
    pub irq_base: c_int,
    pub irq_data: *mut regmap_irq_chip_data,
    pub chip_id: u8,
    pub chip_irq: c_int,
    pub fault_log: c_int,
// SOC I/O transfer related fixes for DA9052/53
    pub reg): *mut *mut *mut int (fix_io) (struct da9052 da9052, unsigned char,
}

// ADC API
extern "C" {
    pub fn da9052_adc_manual_read(da9052: *mut da9052, channel: c_uchar) -> c_int;
}
extern "C" {
    pub fn da9052_adc_read_temp(da9052: *mut da9052) -> c_int;
}
// Device I/O API
extern "C" {
    pub fn da9052_device_init(da9052: *mut da9052, chip_id: u8) -> c_int;
}
extern "C" {
    pub fn da9052_device_exit(da9052: *mut da9052);
}
extern "C" {
    pub fn da9052_irq_init(da9052: *mut da9052) -> c_int;
}
extern "C" {
    pub fn da9052_irq_exit(da9052: *mut da9052) -> c_int;
}
extern "C" {
    pub fn da9052_free_irq(da9052: *mut da9052, irq: c_int, data: *mut c_void);
}
extern "C" {
    pub fn da9052_enable_irq(da9052: *mut da9052, irq: c_int) -> c_int;
}
extern "C" {
    pub fn da9052_disable_irq(da9052: *mut da9052, irq: c_int) -> c_int;
}
extern "C" {
    pub fn da9052_disable_irq_nosync(da9052: *mut da9052, irq: c_int) -> c_int;
}
