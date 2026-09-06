//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/gpio-omap.h
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
// OMAP GPIO handling defines and functions
//
// Copyright (C) 2003-2005 Nokia Corporation
//
// Written by Juha Yrjölä <juha.yrjola@nokia.com>
//

pub const OMAP1_MPUIO_BASE: c_uint = 0xfffb5000;
//
// These are the omap15xx/16xx offsets. The omap7xx offset are
// OMAP_MPUIO_ / 2 offsets below.
//
pub const OMAP_MPUIO_INPUT_LATCH: c_uint = 0x00;
pub const OMAP_MPUIO_OUTPUT: c_uint = 0x04;
pub const OMAP_MPUIO_IO_CNTL: c_uint = 0x08;
pub const OMAP_MPUIO_KBR_LATCH: c_uint = 0x10;
pub const OMAP_MPUIO_KBC: c_uint = 0x14;
pub const OMAP_MPUIO_GPIO_EVENT_MODE: c_uint = 0x18;
pub const OMAP_MPUIO_GPIO_INT_EDGE: c_uint = 0x1c;
pub const OMAP_MPUIO_KBD_INT: c_uint = 0x20;
pub const OMAP_MPUIO_GPIO_INT: c_uint = 0x24;
pub const OMAP_MPUIO_KBD_MASKIT: c_uint = 0x28;
pub const OMAP_MPUIO_GPIO_MASKIT: c_uint = 0x2c;
pub const OMAP_MPUIO_GPIO_DEBOUNCING: c_uint = 0x30;
pub const OMAP_MPUIO_LATCH: c_uint = 0x34;
pub const OMAP34XX_NR_GPIOS: c_int = 6;
//
// OMAP1510 GPIO registers
//
pub const OMAP1510_GPIO_DATA_INPUT: c_uint = 0x00;
pub const OMAP1510_GPIO_DATA_OUTPUT: c_uint = 0x04;
pub const OMAP1510_GPIO_DIR_CONTROL: c_uint = 0x08;
pub const OMAP1510_GPIO_INT_CONTROL: c_uint = 0x0c;
pub const OMAP1510_GPIO_INT_MASK: c_uint = 0x10;
pub const OMAP1510_GPIO_INT_STATUS: c_uint = 0x14;
pub const OMAP1510_GPIO_PIN_CONTROL: c_uint = 0x18;
pub const OMAP1510_IH_GPIO_BASE: c_int = 64;
//
// OMAP1610 specific GPIO registers
//
pub const OMAP1610_GPIO_REVISION: c_uint = 0x0000;
pub const OMAP1610_GPIO_SYSCONFIG: c_uint = 0x0010;
pub const OMAP1610_GPIO_SYSSTATUS: c_uint = 0x0014;
pub const OMAP1610_GPIO_IRQSTATUS1: c_uint = 0x0018;
pub const OMAP1610_GPIO_IRQENABLE1: c_uint = 0x001c;
pub const OMAP1610_GPIO_WAKEUPENABLE: c_uint = 0x0028;
pub const OMAP1610_GPIO_DATAIN: c_uint = 0x002c;
pub const OMAP1610_GPIO_DATAOUT: c_uint = 0x0030;
pub const OMAP1610_GPIO_DIRECTION: c_uint = 0x0034;
pub const OMAP1610_GPIO_EDGE_CTRL1: c_uint = 0x0038;
pub const OMAP1610_GPIO_EDGE_CTRL2: c_uint = 0x003c;
pub const OMAP1610_GPIO_CLEAR_IRQENABLE1: c_uint = 0x009c;
pub const OMAP1610_GPIO_CLEAR_WAKEUPENA: c_uint = 0x00a8;
pub const OMAP1610_GPIO_CLEAR_DATAOUT: c_uint = 0x00b0;
pub const OMAP1610_GPIO_SET_IRQENABLE1: c_uint = 0x00dc;
pub const OMAP1610_GPIO_SET_WAKEUPENA: c_uint = 0x00e8;
pub const OMAP1610_GPIO_SET_DATAOUT: c_uint = 0x00f0;
//
// OMAP7XX specific GPIO registers
//
pub const OMAP7XX_GPIO_DATA_INPUT: c_uint = 0x00;
pub const OMAP7XX_GPIO_DATA_OUTPUT: c_uint = 0x04;
pub const OMAP7XX_GPIO_DIR_CONTROL: c_uint = 0x08;
pub const OMAP7XX_GPIO_INT_CONTROL: c_uint = 0x0c;
pub const OMAP7XX_GPIO_INT_MASK: c_uint = 0x10;
pub const OMAP7XX_GPIO_INT_STATUS: c_uint = 0x14;
//
// omap2+ specific GPIO registers
//
pub const OMAP24XX_GPIO_REVISION: c_uint = 0x0000;
pub const OMAP24XX_GPIO_SYSCONFIG: c_uint = 0x0010;
pub const OMAP24XX_GPIO_IRQSTATUS1: c_uint = 0x0018;
pub const OMAP24XX_GPIO_IRQSTATUS2: c_uint = 0x0028;
pub const OMAP24XX_GPIO_IRQENABLE2: c_uint = 0x002c;
pub const OMAP24XX_GPIO_IRQENABLE1: c_uint = 0x001c;
pub const OMAP24XX_GPIO_WAKE_EN: c_uint = 0x0020;
pub const OMAP24XX_GPIO_CTRL: c_uint = 0x0030;
pub const OMAP24XX_GPIO_OE: c_uint = 0x0034;
pub const OMAP24XX_GPIO_DATAIN: c_uint = 0x0038;
pub const OMAP24XX_GPIO_DATAOUT: c_uint = 0x003c;
pub const OMAP24XX_GPIO_LEVELDETECT0: c_uint = 0x0040;
pub const OMAP24XX_GPIO_LEVELDETECT1: c_uint = 0x0044;
pub const OMAP24XX_GPIO_RISINGDETECT: c_uint = 0x0048;
pub const OMAP24XX_GPIO_FALLINGDETECT: c_uint = 0x004c;
pub const OMAP24XX_GPIO_DEBOUNCE_EN: c_uint = 0x0050;
pub const OMAP24XX_GPIO_DEBOUNCE_VAL: c_uint = 0x0054;
pub const OMAP24XX_GPIO_CLEARIRQENABLE1: c_uint = 0x0060;
pub const OMAP24XX_GPIO_SETIRQENABLE1: c_uint = 0x0064;
pub const OMAP24XX_GPIO_CLEARWKUENA: c_uint = 0x0080;
pub const OMAP24XX_GPIO_SETWKUENA: c_uint = 0x0084;
pub const OMAP24XX_GPIO_CLEARDATAOUT: c_uint = 0x0090;
pub const OMAP24XX_GPIO_SETDATAOUT: c_uint = 0x0094;
pub const OMAP4_GPIO_REVISION: c_uint = 0x0000;
pub const OMAP4_GPIO_SYSCONFIG: c_uint = 0x0010;
pub const OMAP4_GPIO_EOI: c_uint = 0x0020;
pub const OMAP4_GPIO_IRQSTATUSRAW0: c_uint = 0x0024;
pub const OMAP4_GPIO_IRQSTATUSRAW1: c_uint = 0x0028;
pub const OMAP4_GPIO_IRQSTATUS0: c_uint = 0x002c;
pub const OMAP4_GPIO_IRQSTATUS1: c_uint = 0x0030;
pub const OMAP4_GPIO_IRQSTATUSSET0: c_uint = 0x0034;
pub const OMAP4_GPIO_IRQSTATUSSET1: c_uint = 0x0038;
pub const OMAP4_GPIO_IRQSTATUSCLR0: c_uint = 0x003c;
pub const OMAP4_GPIO_IRQSTATUSCLR1: c_uint = 0x0040;
pub const OMAP4_GPIO_IRQWAKEN0: c_uint = 0x0044;
pub const OMAP4_GPIO_IRQWAKEN1: c_uint = 0x0048;
pub const OMAP4_GPIO_IRQENABLE1: c_uint = 0x011c;
pub const OMAP4_GPIO_WAKE_EN: c_uint = 0x0120;
pub const OMAP4_GPIO_IRQSTATUS2: c_uint = 0x0128;
pub const OMAP4_GPIO_IRQENABLE2: c_uint = 0x012c;
pub const OMAP4_GPIO_CTRL: c_uint = 0x0130;
pub const OMAP4_GPIO_OE: c_uint = 0x0134;
pub const OMAP4_GPIO_DATAIN: c_uint = 0x0138;
pub const OMAP4_GPIO_DATAOUT: c_uint = 0x013c;
pub const OMAP4_GPIO_LEVELDETECT0: c_uint = 0x0140;
pub const OMAP4_GPIO_LEVELDETECT1: c_uint = 0x0144;
pub const OMAP4_GPIO_RISINGDETECT: c_uint = 0x0148;
pub const OMAP4_GPIO_FALLINGDETECT: c_uint = 0x014c;
pub const OMAP4_GPIO_DEBOUNCENABLE: c_uint = 0x0150;
pub const OMAP4_GPIO_DEBOUNCINGTIME: c_uint = 0x0154;
pub const OMAP4_GPIO_CLEARIRQENABLE1: c_uint = 0x0160;
pub const OMAP4_GPIO_SETIRQENABLE1: c_uint = 0x0164;
pub const OMAP4_GPIO_CLEARWKUENA: c_uint = 0x0180;
pub const OMAP4_GPIO_SETWKUENA: c_uint = 0x0184;
pub const OMAP4_GPIO_CLEARDATAOUT: c_uint = 0x0190;
pub const OMAP4_GPIO_SETDATAOUT: c_uint = 0x0194;
pub const OMAP_MAX_GPIO_LINES: c_int = 192;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_gpio_reg_offs {
    pub revision: u16,
    pub sysconfig: u16,
    pub direction: u16,
    pub datain: u16,
    pub dataout: u16,
    pub set_dataout: u16,
    pub clr_dataout: u16,
    pub irqstatus: u16,
    pub irqstatus2: u16,
    pub irqstatus_raw0: u16,
    pub irqstatus_raw1: u16,
    pub irqenable: u16,
    pub irqenable2: u16,
    pub set_irqenable: u16,
    pub clr_irqenable: u16,
    pub debounce: u16,
    pub debounce_en: u16,
    pub ctrl: u16,
    pub wkup_en: u16,
    pub leveldetect0: u16,
    pub leveldetect1: u16,
    pub risingdetect: u16,
    pub fallingdetect: u16,
    pub irqctrl: u16,
    pub edgectrl1: u16,
    pub edgectrl2: u16,
    pub pinctrl: u16,
    pub irqenable_inv: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct omap_gpio_platform_data {
    pub bank_type: c_int,
    pub /: *mut *mut int bank_width; / GPIO bank width,
    pub /: *mut *mut int bank_stride; / Only needed for omap1 MPUIO,
    pub /: *mut *mut bool dbck_flag; / dbck required or not - True for OMAP3&4,
    pub /: *mut *mut bool loses_context; / whether the bank would ever lose context,
    pub /: *mut *mut bool is_mpuio; / whether the bank is of type MPUIO,
    pub non_wakeup_gpios: u32,
    pub regs: *const omap_gpio_reg_offs,
// Return context loss count due to PM states changing
    pub dev): *mut *mut int (get_context_loss_count)(struct device,
}

