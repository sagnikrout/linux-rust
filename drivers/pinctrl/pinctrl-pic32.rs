//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pinctrl/pinctrl-pic32.h
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
// PIC32 pinctrl driver
//
// Joshua Henderson, <joshua.henderson@microchip.com>
// Copyright (C) 2015 Microchip Technology Inc.  All rights reserved.
//
// PORT Registers
pub const ANSEL_REG: c_uint = 0x00;
pub const TRIS_REG: c_uint = 0x10;
pub const PORT_REG: c_uint = 0x20;
pub const LAT_REG: c_uint = 0x30;
pub const ODCU_REG: c_uint = 0x40;
pub const CNPU_REG: c_uint = 0x50;
pub const CNPD_REG: c_uint = 0x60;
pub const CNCON_REG: c_uint = 0x70;
pub const CNEN_REG: c_uint = 0x80;
pub const CNSTAT_REG: c_uint = 0x90;
pub const CNNE_REG: c_uint = 0xA0;
pub const CNF_REG: c_uint = 0xB0;
// Input PPS Registers
pub const INT1R: c_uint = 0x04;
pub const INT2R: c_uint = 0x08;
pub const INT3R: c_uint = 0x0C;
pub const INT4R: c_uint = 0x10;
pub const T2CKR: c_uint = 0x18;
pub const T3CKR: c_uint = 0x1C;
pub const T4CKR: c_uint = 0x20;
pub const T5CKR: c_uint = 0x24;
pub const T6CKR: c_uint = 0x28;
pub const T7CKR: c_uint = 0x2C;
pub const T8CKR: c_uint = 0x30;
pub const T9CKR: c_uint = 0x34;
pub const IC1R: c_uint = 0x38;
pub const IC2R: c_uint = 0x3C;
pub const IC3R: c_uint = 0x40;
pub const IC4R: c_uint = 0x44;
pub const IC5R: c_uint = 0x48;
pub const IC6R: c_uint = 0x4C;
pub const IC7R: c_uint = 0x50;
pub const IC8R: c_uint = 0x54;
pub const IC9R: c_uint = 0x58;
pub const OCFAR: c_uint = 0x60;
pub const U1RXR: c_uint = 0x68;
pub const U1CTSR: c_uint = 0x6C;
pub const U2RXR: c_uint = 0x70;
pub const U2CTSR: c_uint = 0x74;
pub const U3RXR: c_uint = 0x78;
pub const U3CTSR: c_uint = 0x7C;
pub const U4RXR: c_uint = 0x80;
pub const U4CTSR: c_uint = 0x84;
pub const U5RXR: c_uint = 0x88;
pub const U5CTSR: c_uint = 0x8C;
pub const U6RXR: c_uint = 0x90;
pub const U6CTSR: c_uint = 0x94;
pub const SDI1R: c_uint = 0x9C;
pub const SS1INR: c_uint = 0xA0;
pub const SDI2R: c_uint = 0xA8;
pub const SS2INR: c_uint = 0xAC;
pub const SDI3R: c_uint = 0xB4;
pub const SS3INR: c_uint = 0xB8;
pub const SDI4R: c_uint = 0xC0;
pub const SS4INR: c_uint = 0xC4;
pub const SDI5R: c_uint = 0xCC;
pub const SS5INR: c_uint = 0xD0;
pub const SDI6R: c_uint = 0xD8;
pub const SS6INR: c_uint = 0xDC;
pub const C1RXR: c_uint = 0xE0;
pub const C2RXR: c_uint = 0xE4;
pub const REFCLKI1R: c_uint = 0xE8;
pub const REFCLKI3R: c_uint = 0xF0;
pub const REFCLKI4R: c_uint = 0xF4;
// Output PPS Registers
pub const RPA14R: c_uint = 0x138;
pub const RPA15R: c_uint = 0x13C;
pub const RPB0R: c_uint = 0x140;
pub const RPB1R: c_uint = 0x144;
pub const RPB2R: c_uint = 0x148;
pub const RPB3R: c_uint = 0x14C;
pub const RPB5R: c_uint = 0x154;
pub const RPB6R: c_uint = 0x158;
pub const RPB7R: c_uint = 0x15C;
pub const RPB8R: c_uint = 0x160;
pub const RPB9R: c_uint = 0x164;
pub const RPB10R: c_uint = 0x168;
pub const RPB14R: c_uint = 0x178;
pub const RPB15R: c_uint = 0x17C;
pub const RPC1R: c_uint = 0x184;
pub const RPC2R: c_uint = 0x188;
pub const RPC3R: c_uint = 0x18C;
pub const RPC4R: c_uint = 0x190;
pub const RPC13R: c_uint = 0x1B4;
pub const RPC14R: c_uint = 0x1B8;
pub const RPD0R: c_uint = 0x1C0;
pub const RPD1R: c_uint = 0x1C4;
pub const RPD2R: c_uint = 0x1C8;
pub const RPD3R: c_uint = 0x1CC;
pub const RPD4R: c_uint = 0x1D0;
pub const RPD5R: c_uint = 0x1D4;
pub const RPD6R: c_uint = 0x1D8;
pub const RPD7R: c_uint = 0x1DC;
pub const RPD9R: c_uint = 0x1E4;
pub const RPD10R: c_uint = 0x1E8;
pub const RPD11R: c_uint = 0x1EC;
pub const RPD12R: c_uint = 0x1F0;
pub const RPD14R: c_uint = 0x1F8;
pub const RPD15R: c_uint = 0x1FC;
pub const RPE3R: c_uint = 0x20C;
pub const RPE5R: c_uint = 0x214;
pub const RPE8R: c_uint = 0x220;
pub const RPE9R: c_uint = 0x224;
pub const RPF0R: c_uint = 0x240;
pub const RPF1R: c_uint = 0x244;
pub const RPF2R: c_uint = 0x248;
pub const RPF3R: c_uint = 0x24C;
pub const RPF4R: c_uint = 0x250;
pub const RPF5R: c_uint = 0x254;
pub const RPF8R: c_uint = 0x260;
pub const RPF12R: c_uint = 0x270;
pub const RPF13R: c_uint = 0x274;
pub const RPG0R: c_uint = 0x280;
pub const RPG1R: c_uint = 0x284;
pub const RPG6R: c_uint = 0x298;
pub const RPG7R: c_uint = 0x29C;
pub const RPG8R: c_uint = 0x2A0;
pub const RPG9R: c_uint = 0x2A4;
