//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/twl6040.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// MFD driver for twl6040
//
// Authors:     Jorge Eduardo Candelaria <jorge.candelaria@ti.com>
// Misael Lopez Cruz <misael.lopez@ti.com>
//
// Copyright:   (C) 2011 Texas Instruments, Inc.
//

pub const TWL6040_REG_ASICID: c_uint = 0x01;
pub const TWL6040_REG_ASICREV: c_uint = 0x02;
pub const TWL6040_REG_INTID: c_uint = 0x03;
pub const TWL6040_REG_INTMR: c_uint = 0x04;
pub const TWL6040_REG_NCPCTL: c_uint = 0x05;
pub const TWL6040_REG_LDOCTL: c_uint = 0x06;
pub const TWL6040_REG_HPPLLCTL: c_uint = 0x07;
pub const TWL6040_REG_LPPLLCTL: c_uint = 0x08;
pub const TWL6040_REG_LPPLLDIV: c_uint = 0x09;
pub const TWL6040_REG_AMICBCTL: c_uint = 0x0A;
pub const TWL6040_REG_DMICBCTL: c_uint = 0x0B;
pub const TWL6040_REG_MICLCTL: c_uint = 0x0C;
pub const TWL6040_REG_MICRCTL: c_uint = 0x0D;
pub const TWL6040_REG_MICGAIN: c_uint = 0x0E;
pub const TWL6040_REG_LINEGAIN: c_uint = 0x0F;
pub const TWL6040_REG_HSLCTL: c_uint = 0x10;
pub const TWL6040_REG_HSRCTL: c_uint = 0x11;
pub const TWL6040_REG_HSGAIN: c_uint = 0x12;
pub const TWL6040_REG_EARCTL: c_uint = 0x13;
pub const TWL6040_REG_HFLCTL: c_uint = 0x14;
pub const TWL6040_REG_HFLGAIN: c_uint = 0x15;
pub const TWL6040_REG_HFRCTL: c_uint = 0x16;
pub const TWL6040_REG_HFRGAIN: c_uint = 0x17;
pub const TWL6040_REG_VIBCTLL: c_uint = 0x18;
pub const TWL6040_REG_VIBDATL: c_uint = 0x19;
pub const TWL6040_REG_VIBCTLR: c_uint = 0x1A;
pub const TWL6040_REG_VIBDATR: c_uint = 0x1B;
pub const TWL6040_REG_HKCTL1: c_uint = 0x1C;
pub const TWL6040_REG_HKCTL2: c_uint = 0x1D;
pub const TWL6040_REG_GPOCTL: c_uint = 0x1E;
pub const TWL6040_REG_ALB: c_uint = 0x1F;
pub const TWL6040_REG_DLB: c_uint = 0x20;
pub const TWL6040_REG_TRIM1: c_uint = 0x28;
pub const TWL6040_REG_TRIM2: c_uint = 0x29;
pub const TWL6040_REG_TRIM3: c_uint = 0x2A;
pub const TWL6040_REG_HSOTRIM: c_uint = 0x2B;
pub const TWL6040_REG_HFOTRIM: c_uint = 0x2C;
pub const TWL6040_REG_ACCCTL: c_uint = 0x2D;
pub const TWL6040_REG_STATUS: c_uint = 0x2E;
// INTID (0x03) fields
pub const TWL6040_THINT: c_uint = 0x01;
pub const TWL6040_PLUGINT: c_uint = 0x02;
pub const TWL6040_UNPLUGINT: c_uint = 0x04;
pub const TWL6040_HOOKINT: c_uint = 0x08;
pub const TWL6040_HFINT: c_uint = 0x10;
pub const TWL6040_VIBINT: c_uint = 0x20;
pub const TWL6040_READYINT: c_uint = 0x40;
// INTMR (0x04) fields
pub const TWL6040_THMSK: c_uint = 0x01;
pub const TWL6040_PLUGMSK: c_uint = 0x02;
pub const TWL6040_HOOKMSK: c_uint = 0x08;
pub const TWL6040_HFMSK: c_uint = 0x10;
pub const TWL6040_VIBMSK: c_uint = 0x20;
pub const TWL6040_READYMSK: c_uint = 0x40;
pub const TWL6040_ALLINT_MSK: c_uint = 0x7B;
// NCPCTL (0x05) fields
pub const TWL6040_NCPENA: c_uint = 0x01;
pub const TWL6040_NCPOPEN: c_uint = 0x40;
// LDOCTL (0x06) fields
pub const TWL6040_LSLDOENA: c_uint = 0x01;
pub const TWL6040_HSLDOENA: c_uint = 0x04;
pub const TWL6040_REFENA: c_uint = 0x40;
pub const TWL6040_OSCENA: c_uint = 0x80;
// HPPLLCTL (0x07) fields
pub const TWL6040_HPLLENA: c_uint = 0x01;
pub const TWL6040_HPLLRST: c_uint = 0x02;
pub const TWL6040_HPLLBP: c_uint = 0x04;
pub const TWL6040_HPLLSQRENA: c_uint = 0x08;

pub const TWL6040_MCLK_MSK: c_uint = 0x60;
// LPPLLCTL (0x08) fields
pub const TWL6040_LPLLENA: c_uint = 0x01;
pub const TWL6040_LPLLRST: c_uint = 0x02;
pub const TWL6040_LPLLSEL: c_uint = 0x04;
pub const TWL6040_LPLLFIN: c_uint = 0x08;
pub const TWL6040_HPLLSEL: c_uint = 0x10;
// HSLCTL/R (0x10/0x11) fields

// HFLCTL/R (0x14/0x16) fields

// VIBCTLL/R (0x18/0x1A) fields

// VIBDATL/R (0x19/0x1B) fields
pub const TWL6040_VIBDAT_MAX: c_uint = 0x64;
// GPOCTL (0x1E) fields
pub const TWL6040_GPO1: c_uint = 0x01;
pub const TWL6040_GPO2: c_uint = 0x02;
pub const TWL6040_GPO3: c_uint = 0x04;
// ACCCTL (0x2D) fields
pub const TWL6040_I2CSEL: c_uint = 0x01;
pub const TWL6040_RESETSPLIT: c_uint = 0x04;
pub const TWL6040_INTCLRMODE: c_uint = 0x08;

// STATUS (0x2E) fields
pub const TWL6040_PLUGCOMP: c_uint = 0x02;
pub const TWL6040_VIBLOCDET: c_uint = 0x10;
pub const TWL6040_VIBROCDET: c_uint = 0x20;
pub const TWL6040_TSHUTDET: c_uint = 0x40;
pub const TWL6040_CELLS: c_int = 4;
pub const TWL6040_REV_ES1_0: c_uint = 0x00;
pub const TWL6040_REV_ES1_1: c_uint = 0x01 /* Rev ES1.1 and ES1.2 */;
pub const TWL6040_REV_ES1_3: c_uint = 0x02;
pub const TWL6041_REV_ES2_0: c_uint = 0x10;
pub const TWL6040_IRQ_TH: c_int = 0;
pub const TWL6040_IRQ_PLUG: c_int = 1;
pub const TWL6040_IRQ_HOOK: c_int = 2;
pub const TWL6040_IRQ_HF: c_int = 3;
pub const TWL6040_IRQ_VIB: c_int = 4;
pub const TWL6040_IRQ_READY: c_int = 5;
// PLL selection
pub const TWL6040_SYSCLK_SEL_LPPLL: c_int = 0;
pub const TWL6040_SYSCLK_SEL_HPPLL: c_int = 1;
pub const TWL6040_GPO_MAX: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct twl6040 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub irq_data: *mut regmap_irq_chip_data,
    pub /: *mut *mut regulator_bulk_data supplies[2]; / supplies for vio, v2v1,
    pub clk32k: *mut clk,
    pub mclk: *mut clk,
    pub mutex: mutex,
    pub irq_mutex: mutex,
    pub cells: [mfd_cell; TWL6040_CELLS],
    pub ready: completion,
    pub audpwron: *mut gpio_desc,
    pub power_count: c_int,
    pub rev: c_int,
// PLL configuration
    pub pll: c_int,
    pub sysclk_rate: c_uint,
    pub mclk_rate: c_uint,
    pub irq: c_uint,
    pub irq_ready: c_uint,
    pub irq_th: c_uint,
}

extern "C" {
    pub fn twl6040_reg_read(twl6040: *mut twl6040, reg: c_uint) -> c_int;
}
extern "C" {
    pub fn twl6040_power(twl6040: *mut twl6040, on: c_int) -> c_int;
}
extern "C" {
    pub fn twl6040_get_pll(twl6040: *mut twl6040) -> c_int;
}
extern "C" {
    pub fn twl6040_get_sysclk(twl6040: *mut twl6040) -> c_uint;
}
// Get the combined status of the vibra control register
extern "C" {
    pub fn twl6040_get_vibralr_status(twl6040: *mut twl6040) -> c_int;
}
