//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/ti_am335x_tscadc.h
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
// TI Touch Screen / ADC MFD driver
//
// Copyright (C) 2012 Texas Instruments Incorporated - https://www.ti.com
//

pub const REG_RAWIRQSTATUS: c_uint = 0x024;
pub const REG_IRQSTATUS: c_uint = 0x028;
pub const REG_IRQENABLE: c_uint = 0x02C;
pub const REG_IRQCLR: c_uint = 0x030;
pub const REG_IRQWAKEUP: c_uint = 0x034;
pub const REG_DMAENABLE_SET: c_uint = 0x038;
pub const REG_DMAENABLE_CLEAR: c_uint = 0x03c;
pub const REG_CTRL: c_uint = 0x040;
pub const REG_ADCFSM: c_uint = 0x044;
pub const REG_CLKDIV: c_uint = 0x04C;
pub const REG_SE: c_uint = 0x054;
pub const REG_IDLECONFIG: c_uint = 0x058;
pub const REG_CHARGECONFIG: c_uint = 0x05C;
pub const REG_CHARGEDELAY: c_uint = 0x060;

pub const REG_FIFO0CNT: c_uint = 0xE4;
pub const REG_FIFO0THR: c_uint = 0xE8;
pub const REG_FIFO1CNT: c_uint = 0xF0;
pub const REG_FIFO1THR: c_uint = 0xF4;
pub const REG_DMA1REQ: c_uint = 0xF8;
pub const REG_FIFO0: c_uint = 0x100;
pub const REG_FIFO1: c_uint = 0x200;
// Register Bitfields
// IRQ wakeup enable

// IRQ enable

// Step Configuration

// Delay register

// Charge Config

// Charge delay

// Control register

// Control registers bitfields  for MAGADC IP

// FIFO READ Register

// DMA ENABLE/CLEAR Register

// Sequencer Status

pub const CHARGE_STEP: c_uint = 0x11;

pub const TOTAL_STEPS: c_int = 16;
pub const TOTAL_CHANNELS: c_int = 8;
pub const FIFO1_THRESHOLD: c_int = 19;
//
// time in us for processing a single channel, calculated as follows:
//
// max num cycles = open delay + (sample delay + conv time) * averaging
//
// max num cycles: 262143 + (255 + 13) * 16 = 266431
//
// clock frequency: 26MHz / 8 = 3.25MHz
// clock period: 1 / 3.25MHz = 308ns
//
// max processing time: 266431 * 308ns = 83ms(approx)
//

pub const TSCADC_CELLS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_tscadc_data {
    pub adc_feature_name: *mut c_char,
    pub adc_feature_compatible: *mut c_char,
    pub secondary_feature_name: *mut c_char,
    pub secondary_feature_compatible: *mut c_char,
    pub target_clk_rate: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ti_tscadc_dev {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub tscadc_base: *mut void __iomem,
    pub tscadc_phys_base: phys_addr_t,
    pub data: *const ti_tscadc_data,
    pub irq: c_int,
    pub cells: [mfd_cell; TSCADC_CELLS],
    pub ctrl: u32,
    pub reg_se_cache: u32,
    pub adc_waiting: bool,
    pub adc_in_use: bool,
    pub reg_se_wait: wait_queue_head_t,
    pub reg_lock: spinlock_t,
    pub clk_div: c_uint,
// tsc device
    pub tsc: *mut titsc,
// adc device
    pub adc: *mut adc_device,
}

extern "C" {
    pub fn am335x_tsc_se_set_cache(tsadc: *mut ti_tscadc_dev, val: u32);
}
extern "C" {
    pub fn am335x_tsc_se_set_once(tsadc: *mut ti_tscadc_dev, val: u32);
}
extern "C" {
    pub fn am335x_tsc_se_clr(tsadc: *mut ti_tscadc_dev, val: u32);
}
extern "C" {
    pub fn am335x_tsc_se_adc_done(tsadc: *mut ti_tscadc_dev);
}
