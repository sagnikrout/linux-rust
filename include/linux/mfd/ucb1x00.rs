//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/ucb1x00.h
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
// linux/include/mfd/ucb1x00.h
//
// Copyright (C) 2001 Russell King, All Rights Reserved.
//

pub const UCB_IO_DATA: c_uint = 0x00;
pub const UCB_IO_DIR: c_uint = 0x01;

pub const UCB_IE_RIS: c_uint = 0x02;
pub const UCB_IE_FAL: c_uint = 0x03;
pub const UCB_IE_STATUS: c_uint = 0x04;
pub const UCB_IE_CLEAR: c_uint = 0x04;

pub const UCB_IRQ_TSPX: c_int = 12;
pub const UCB_TC_A: c_uint = 0x05;

pub const UCB_TC_B: c_uint = 0x06;

pub const UCB_AC_A: c_uint = 0x07;
pub const UCB_AC_B: c_uint = 0x08;

pub const UCB_TS_CR: c_uint = 0x09;

pub const UCB_ADC_CR: c_uint = 0x0a;

pub const UCB_ADC_DATA: c_uint = 0x0b;

pub const UCB_ID: c_uint = 0x0c;
pub const UCB_ID_1200: c_uint = 0x1004;
pub const UCB_ID_1300: c_uint = 0x1005;
pub const UCB_ID_TC35143: c_uint = 0x9712;
pub const UCB_MODE: c_uint = 0x0d;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ucb1x00_reset {
    UCB_RST_PROBE,
    UCB_RST_RESUME,
    UCB_RST_SUSPEND,
    UCB_RST_REMOVE,
    UCB_RST_PROBE_FAIL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucb1x00_plat_data {
    pub ucb1x00_reset): *mut *mut void (reset)(enum,
    pub irq_base: unsigned,
    pub gpio_base: c_int,
    pub can_wakeup: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucb1x00 {
    pub irq_lock: raw_spinlock_t,
    pub mcp: *mut mcp,
    pub irq: c_uint,
    pub irq_base: c_int,
    pub adc_mutex: mutex,
    pub io_lock: spinlock_t,
    pub id: u16,
    pub io_dir: u16,
    pub io_out: u16,
    pub adc_cr: u16,
    pub irq_fal_enbl: u16,
    pub irq_ris_enbl: u16,
    pub irq_mask: u16,
    pub irq_wake: u16,
    pub dev: device,
    pub node: list_head,
    pub devs: list_head,
    pub gpio: gpio_chip,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucb1x00_dev {
    pub dev_node: list_head,
    pub drv_node: list_head,
    pub ucb: *mut ucb1x00,
    pub drv: *mut ucb1x00_driver,
    pub priv: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucb1x00_driver {
    pub node: list_head,
    pub devs: list_head,
    pub dev): *mut *mut int (add)(struct ucb1x00_dev,
    pub dev): *mut *mut void (remove)(struct ucb1x00_dev,
    pub dev): *mut *mut int (suspend)(struct ucb1x00_dev,
    pub dev): *mut *mut int (resume)(struct ucb1x00_dev,
}

extern "C" {
    pub fn ucb1x00_register_driver(: *mut ucb1x00_driver) -> c_int;
}
extern "C" {
    pub fn ucb1x00_unregister_driver(: *mut ucb1x00_driver);
}
//
// ucb1x00_clkrate - return the UCB1x00 SIB clock rate
// @ucb: UCB1x00 structure describing chip
//
// Return the SIB clock rate in Hz.
//
extern "C" {
    pub fn mcp_get_sclk_rate(_arg: ucb->mcp) -> return;
}
//
// ucb1x00_enable - enable the UCB1x00 SIB clock
// @ucb: UCB1x00 structure describing chip
//
// Enable the SIB clock.  This can be called multiple times.
//
// ucb1x00_disable - disable the UCB1x00 SIB clock
// @ucb: UCB1x00 structure describing chip
//
// Disable the SIB clock.  The SIB clock will only be disabled
// when the number of ucb1x00_enable calls match the number of
// ucb1x00_disable calls.
//
// ucb1x00_reg_write - write a UCB1x00 register
// @ucb: UCB1x00 structure describing chip
// @reg: UCB1x00 4-bit register index to write
// @val: UCB1x00 16-bit value to write
//
// Write the UCB1x00 register @reg with value @val.  The SIB
// clock must be running for this function to return.
//
// ucb1x00_reg_read - read a UCB1x00 register
// @ucb: UCB1x00 structure describing chip
// @reg: UCB1x00 4-bit register index to write
//
// Read the UCB1x00 register @reg and return its value.  The SIB
// clock must be running for this function to return.
//
extern "C" {
    pub fn mcp_reg_read(_arg: ucb->mcp, _arg: reg) -> return;
}
//
// ucb1x00_set_audio_divisor -
// @ucb: UCB1x00 structure describing chip
// @div: SIB clock divisor
//
// ucb1x00_set_telecom_divisor -
// @ucb: UCB1x00 structure describing chip
// @div: SIB clock divisor
//
extern "C" {
    pub fn ucb1x00_io_set_dir(ucb: *mut ucb1x00, int: unsigned, int: unsigned);
}
extern "C" {
    pub fn ucb1x00_io_write(ucb: *mut ucb1x00, int: unsigned, int: unsigned);
}
extern "C" {
    pub fn ucb1x00_io_read(ucb: *mut ucb1x00) -> c_uint;
}

extern "C" {
    pub fn ucb1x00_adc_read(ucb: *mut ucb1x00, adc_channel: c_int, sync: c_int) -> c_uint;
}
extern "C" {
    pub fn ucb1x00_adc_enable(ucb: *mut ucb1x00);
}
extern "C" {
    pub fn ucb1x00_adc_disable(ucb: *mut ucb1x00);
}
