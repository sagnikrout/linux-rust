//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/ddbridge/ddbridge-regs.h
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


// SPDX-License-Identifier: GPL-2.0
//
// ddbridge-regs.h: Digital Devices PCIe bridge driver
//
// Copyright (C) 2010-2017 Digital Devices GmbH
//
// -------------------------------------------------------------------------
// SPI Controller
pub const SPI_CONTROL: c_uint = 0x10;
pub const SPI_DATA: c_uint = 0x14;
// -------------------------------------------------------------------------
// GPIO
pub const GPIO_OUTPUT: c_uint = 0x20;
pub const GPIO_INPUT: c_uint = 0x24;
pub const GPIO_DIRECTION: c_uint = 0x28;
// -------------------------------------------------------------------------
pub const BOARD_CONTROL: c_uint = 0x30;
// -------------------------------------------------------------------------
// Interrupt controller
// How many MSI's are available depends on HW (Min 2 max 8)
// How many are usable also depends on Host platform
//

// Temperature Monitor ( 2x LM75A @ 0x90,0x92 I2c )

// SHORT Temperature in Celsius x 256

// -------------------------------------------------------------------------
// I2C Master Controller

// -------------------------------------------------------------------------
// DMA  Controller

// -------------------------------------------------------------------------
// DMA  Buffer

// -------------------------------------------------------------------------
// CI Interface (only CI-Bridge)

// -------------------------------------------------------------------------
// LNB commands (mxl5xx / Max S8)

pub const LNB_CMD_NOP: c_int = 0;
pub const LNB_CMD_INIT: c_int = 1;
pub const LNB_CMD_LOW: c_int = 3;
pub const LNB_CMD_HIGH: c_int = 4;
pub const LNB_CMD_OFF: c_int = 5;
pub const LNB_CMD_DISEQC: c_int = 6;

