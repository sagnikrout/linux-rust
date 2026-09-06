//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/gspca/stk1135.h
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
// STK1135 registers
//
// Copyright (c) 2013 Ondrej Zary
//
pub const STK1135_REG_GCTRL: c_uint = 0x000	/* GPIO control */;
pub const STK1135_REG_ICTRL: c_uint = 0x004	/* Interrupt control */;
pub const STK1135_REG_IDATA: c_uint = 0x008	/* Interrupt data */;
pub const STK1135_REG_RMCTL: c_uint = 0x00c	/* Remote wakeup control */;
pub const STK1135_REG_POSVA: c_uint = 0x010	/* Power-on strapping data */;
pub const STK1135_REG_SENSO: c_uint = 0x018	/* Sensor select options */;
pub const STK1135_REG_PLLFD: c_uint = 0x01c	/* PLL frequency divider */;
pub const STK1135_REG_SCTRL: c_uint = 0x100	/* Sensor control register */;
pub const STK1135_REG_DCTRL: c_uint = 0x104	/* Decimation control register */;
pub const STK1135_REG_CISPO: c_uint = 0x110	/* Capture image starting position */;
pub const STK1135_REG_CIEPO: c_uint = 0x114	/* Capture image ending position */;
pub const STK1135_REG_TCTRL: c_uint = 0x120	/* Test data control */;
pub const STK1135_REG_SICTL: c_uint = 0x200	/* Serial interface control register */;
pub const STK1135_REG_SBUSW: c_uint = 0x204	/* Serial bus write */;
pub const STK1135_REG_SBUSR: c_uint = 0x208	/* Serial bus read */;
pub const STK1135_REG_SCSI: c_uint = 0x20c	/* Software control serial interface */;
pub const STK1135_REG_GSBWP: c_uint = 0x210	/* General serial bus write port */;
pub const STK1135_REG_GSBRP: c_uint = 0x214	/* General serial bus read port */;
pub const STK1135_REG_ASIC: c_uint = 0x2fc	/* Alternate serial interface control */;
pub const STK1135_REG_TMGEN: c_uint = 0x300	/* Timing generator */;
pub const STK1135_REG_TCP1: c_uint = 0x350	/* Timing control parameter 1 */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stk1135_pkt_header {
    pub flags: u8,
    pub seq: u8,
    pub gpio: __le16,
    pub __packed: },

pub const STK1135_HDR_SEQ_MASK: c_uint = 0x3f;
