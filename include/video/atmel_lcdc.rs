//! Automatically rewritten from C Header to Rust Module
//! Source: include/video/atmel_lcdc.h
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
// Header file for AT91/AT32 LCD Controller
//
// Data structure and register user interface
//
// Copyright (C) 2007 Atmel Corporation
//

// Way LCD wires are connected to the chip:
// Some Atmel chips use BGR color mode (instead of standard RGB)
// A swapped wiring onboard can bring to RGB mode.
//
pub const ATMEL_LCDC_WIRING_BGR: c_int = 0;
pub const ATMEL_LCDC_WIRING_RGB: c_int = 1;
// LCD Controller info data structure, stored in device platform_data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_lcdfb_pdata {
    pub guard_time: c_uint,
    pub lcdcon_is_backlight: bool,
    pub lcdcon_pol_negative: bool,
    pub default_bpp: u8,
    pub lcd_wiring_mode: u8,
    pub default_lcdcon2: c_uint,
    pub default_dmacon: c_uint,
    pub on): *mut *mut *mut void (atmel_lcdfb_power_control)(struct atmel_lcdfb_pdata pdata, int,
    pub default_monspecs: *mut fb_monspecs,
    pub pwr_gpios: list_head,
}

pub const ATMEL_LCDC_DMABADDR1: c_uint = 0x00;
pub const ATMEL_LCDC_DMABADDR2: c_uint = 0x04;
pub const ATMEL_LCDC_DMAFRMPT1: c_uint = 0x08;
pub const ATMEL_LCDC_DMAFRMPT2: c_uint = 0x0c;
pub const ATMEL_LCDC_DMAFRMADD1: c_uint = 0x10;
pub const ATMEL_LCDC_DMAFRMADD2: c_uint = 0x14;
pub const ATMEL_LCDC_DMAFRMCFG: c_uint = 0x18;

pub const ATMEL_LCDC_BLENGTH_OFFSET: c_int = 24;

pub const ATMEL_LCDC_DMACON: c_uint = 0x1c;

pub const ATMEL_LCDC_DMA2DCFG: c_uint = 0x20;
pub const ATMEL_LCDC_ADDRINC_OFFSET: c_int = 0;

pub const ATMEL_LCDC_PIXELOFF_OFFSET: c_int = 24;

pub const ATMEL_LCDC_LCDCON1: c_uint = 0x0800;

pub const ATMEL_LCDC_CLKVAL_OFFSET: c_int = 12;

pub const ATMEL_LCDC_LCDCON2: c_uint = 0x0804;

pub const ATMEL_LCDC_TIM1: c_uint = 0x0808;

pub const ATMEL_LCDC_VBP_OFFSET: c_int = 8;

pub const ATMEL_LCDC_VPW_OFFSET: c_int = 16;

pub const ATMEL_LCDC_VHDLY_OFFSET: c_int = 24;

pub const ATMEL_LCDC_TIM2: c_uint = 0x080c;

pub const ATMEL_LCDC_HPW_OFFSET: c_int = 8;

pub const ATMEL_LCDC_HFP_OFFSET: c_int = 21;

pub const ATMEL_LCDC_LCDFRMCFG: c_uint = 0x0810;

pub const ATMEL_LCDC_HOZVAL_OFFSET: c_int = 21;

pub const ATMEL_LCDC_FIFO: c_uint = 0x0814;

pub const ATMEL_LCDC_MVAL: c_uint = 0x0818;
pub const ATMEL_LCDC_DP1_2: c_uint = 0x081c;
pub const ATMEL_LCDC_DP4_7: c_uint = 0x0820;
pub const ATMEL_LCDC_DP3_5: c_uint = 0x0824;
pub const ATMEL_LCDC_DP2_3: c_uint = 0x0828;
pub const ATMEL_LCDC_DP5_7: c_uint = 0x082c;
pub const ATMEL_LCDC_DP3_4: c_uint = 0x0830;
pub const ATMEL_LCDC_DP4_5: c_uint = 0x0834;
pub const ATMEL_LCDC_DP6_7: c_uint = 0x0838;

pub const ATMEL_LCDC_PWRCON: c_uint = 0x083c;

pub const ATMEL_LCDC_GUARDT_OFFSET: c_int = 1;

pub const ATMEL_LCDC_CONTRAST_CTR: c_uint = 0x0840;

pub const ATMEL_LCDC_CONTRAST_VAL: c_uint = 0x0844;

pub const ATMEL_LCDC_IER: c_uint = 0x0848;
pub const ATMEL_LCDC_IDR: c_uint = 0x084c;
pub const ATMEL_LCDC_IMR: c_uint = 0x0850;
pub const ATMEL_LCDC_ISR: c_uint = 0x0854;
pub const ATMEL_LCDC_ICR: c_uint = 0x0858;

