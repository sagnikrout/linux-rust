//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/tc3589x.h
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
// Copyright (C) ST-Ericsson SA 2010
//

// Macro flag: #define __LINUX_MFD_TC3589x_H
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tx3589x_block {
    TC3589x_BLOCK_GPIO        = 1 << 0,
    TC3589x_BLOCK_KEYPAD      = 1 << 1,
}

// Keyboard Configuration Registers
pub const TC3589x_KBDSETTLE_REG: c_uint = 0x01;
pub const TC3589x_KBDBOUNCE: c_uint = 0x02;
pub const TC3589x_KBDSIZE: c_uint = 0x03;
pub const TC3589x_KBCFG_LSB: c_uint = 0x04;
pub const TC3589x_KBCFG_MSB: c_uint = 0x05;
pub const TC3589x_KBDIC: c_uint = 0x08;
pub const TC3589x_KBDMSK: c_uint = 0x09;
pub const TC3589x_EVTCODE_FIFO: c_uint = 0x10;
pub const TC3589x_KBDMFS: c_uint = 0x8F;
pub const TC3589x_IRQST: c_uint = 0x91;
pub const TC3589x_MANFCODE_MAGIC: c_uint = 0x03;
pub const TC3589x_MANFCODE: c_uint = 0x80;
pub const TC3589x_VERSION: c_uint = 0x81;
pub const TC3589x_IOCFG: c_uint = 0xA7;
pub const TC3589x_CLKMODE: c_uint = 0x88;
pub const TC3589x_CLKCFG: c_uint = 0x89;
pub const TC3589x_CLKEN: c_uint = 0x8A;
pub const TC3589x_RSTCTRL: c_uint = 0x82;
pub const TC3589x_EXTRSTN: c_uint = 0x83;
pub const TC3589x_RSTINTCLR: c_uint = 0x84;
// Pull up/down configuration registers
pub const TC3589x_IOCFG: c_uint = 0xA7;
pub const TC3589x_IOPULLCFG0_LSB: c_uint = 0xAA;
pub const TC3589x_IOPULLCFG0_MSB: c_uint = 0xAB;
pub const TC3589x_IOPULLCFG1_LSB: c_uint = 0xAC;
pub const TC3589x_IOPULLCFG1_MSB: c_uint = 0xAD;
pub const TC3589x_IOPULLCFG2_LSB: c_uint = 0xAE;
pub const TC3589x_GPIOIS0: c_uint = 0xC9;
pub const TC3589x_GPIOIS1: c_uint = 0xCA;
pub const TC3589x_GPIOIS2: c_uint = 0xCB;
pub const TC3589x_GPIOIBE0: c_uint = 0xCC;
pub const TC3589x_GPIOIBE1: c_uint = 0xCD;
pub const TC3589x_GPIOIBE2: c_uint = 0xCE;
pub const TC3589x_GPIOIEV0: c_uint = 0xCF;
pub const TC3589x_GPIOIEV1: c_uint = 0xD0;
pub const TC3589x_GPIOIEV2: c_uint = 0xD1;
pub const TC3589x_GPIOIE0: c_uint = 0xD2;
pub const TC3589x_GPIOIE1: c_uint = 0xD3;
pub const TC3589x_GPIOIE2: c_uint = 0xD4;
pub const TC3589x_GPIORIS0: c_uint = 0xD6;
pub const TC3589x_GPIORIS1: c_uint = 0xD7;
pub const TC3589x_GPIORIS2: c_uint = 0xD8;
pub const TC3589x_GPIOMIS0: c_uint = 0xD9;
pub const TC3589x_GPIOMIS1: c_uint = 0xDA;
pub const TC3589x_GPIOMIS2: c_uint = 0xDB;
pub const TC3589x_GPIOIC0: c_uint = 0xDC;
pub const TC3589x_GPIOIC1: c_uint = 0xDD;
pub const TC3589x_GPIOIC2: c_uint = 0xDE;
pub const TC3589x_GPIODATA0: c_uint = 0xC0;
pub const TC3589x_GPIOMASK0: c_uint = 0xc1;
pub const TC3589x_GPIODATA1: c_uint = 0xC2;
pub const TC3589x_GPIOMASK1: c_uint = 0xc3;
pub const TC3589x_GPIODATA2: c_uint = 0xC4;
pub const TC3589x_GPIOMASK2: c_uint = 0xC5;
pub const TC3589x_GPIODIR0: c_uint = 0xC6;
pub const TC3589x_GPIODIR1: c_uint = 0xC7;
pub const TC3589x_GPIODIR2: c_uint = 0xC8;
pub const TC3589x_GPIOSYNC0: c_uint = 0xE6;
pub const TC3589x_GPIOSYNC1: c_uint = 0xE7;
pub const TC3589x_GPIOSYNC2: c_uint = 0xE8;
pub const TC3589x_GPIOWAKE0: c_uint = 0xE9;
pub const TC3589x_GPIOWAKE1: c_uint = 0xEA;
pub const TC3589x_GPIOWAKE2: c_uint = 0xEB;
pub const TC3589x_GPIOODM0: c_uint = 0xE0;
pub const TC3589x_GPIOODE0: c_uint = 0xE1;
pub const TC3589x_GPIOODM1: c_uint = 0xE2;
pub const TC3589x_GPIOODE1: c_uint = 0xE3;
pub const TC3589x_GPIOODM2: c_uint = 0xE4;
pub const TC3589x_GPIOODE2: c_uint = 0xE5;
pub const TC3589x_DIRECT0: c_uint = 0xEC;
pub const TC3589x_DKBDMSK: c_uint = 0xF3;
pub const TC3589x_INT_GPIIRQ: c_int = 0;
pub const TC3589x_INT_TI0IRQ: c_int = 1;
pub const TC3589x_INT_TI1IRQ: c_int = 2;
pub const TC3589x_INT_TI2IRQ: c_int = 3;
pub const TC3589x_INT_ROTIRQ: c_int = 5;
pub const TC3589x_INT_KBDIRQ: c_int = 6;
pub const TC3589x_INT_PORIRQ: c_int = 7;
pub const TC3589x_NR_INTERNAL_IRQS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc3589x {
    pub lock: mutex,
    pub dev: *mut device,
    pub i2c: *mut i2c_client,
    pub domain: *mut irq_domain,
    pub irq_base: c_int,
    pub num_gpio: c_int,
    pub pdata: *mut tc3589x_platform_data,
}

extern "C" {
    pub fn tc3589x_reg_write(tc3589x: *mut tc3589x, reg: u8, data: u8) -> c_int;
}
extern "C" {
    pub fn tc3589x_reg_read(tc3589x: *mut tc3589x, reg: u8) -> c_int;
}
extern "C" {
    pub fn tc3589x_set_bits(tc3589x: *mut tc3589x, reg: u8, mask: u8, val: u8) -> c_int;
}
//
// Keypad related platform specific constants
// These values may be modified for fine tuning
//
pub const TC_KPD_ROWS: c_uint = 0x8;
pub const TC_KPD_COLUMNS: c_uint = 0x8;
pub const TC_KPD_DEBOUNCE_PERIOD: c_uint = 0xA3;
pub const TC_KPD_SETTLE_TIME: c_uint = 0xA3;
//
// struct tc3589x_platform_data - TC3589x platform data
// @block: bitmask of blocks to enable (use TC3589x_BLOCK_*)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc3589x_platform_data {
    pub block: c_uint,
}
