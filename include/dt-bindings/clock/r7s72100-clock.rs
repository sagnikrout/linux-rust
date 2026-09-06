//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/r7s72100-clock.h
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
// Copyright (C) 2014 Renesas Solutions Corp.
// Copyright (C) 2014 Wolfram Sang, Sang Engineering <wsa@sang-engineering.com>
//
pub const R7S72100_CLK_PLL: c_int = 0;
pub const R7S72100_CLK_I: c_int = 1;
pub const R7S72100_CLK_G: c_int = 2;
// MSTP2
pub const R7S72100_CLK_CORESIGHT: c_int = 0;
// MSTP3
pub const R7S72100_CLK_IEBUS: c_int = 7;
pub const R7S72100_CLK_IRDA: c_int = 6;
pub const R7S72100_CLK_LIN0: c_int = 5;
pub const R7S72100_CLK_LIN1: c_int = 4;
pub const R7S72100_CLK_MTU2: c_int = 3;
pub const R7S72100_CLK_CAN: c_int = 2;
pub const R7S72100_CLK_ADCPWR: c_int = 1;
pub const R7S72100_CLK_PWM: c_int = 0;
// MSTP4
pub const R7S72100_CLK_SCIF0: c_int = 7;
pub const R7S72100_CLK_SCIF1: c_int = 6;
pub const R7S72100_CLK_SCIF2: c_int = 5;
pub const R7S72100_CLK_SCIF3: c_int = 4;
pub const R7S72100_CLK_SCIF4: c_int = 3;
pub const R7S72100_CLK_SCIF5: c_int = 2;
pub const R7S72100_CLK_SCIF6: c_int = 1;
pub const R7S72100_CLK_SCIF7: c_int = 0;
// MSTP5
pub const R7S72100_CLK_SCI0: c_int = 7;
pub const R7S72100_CLK_SCI1: c_int = 6;
pub const R7S72100_CLK_SG0: c_int = 5;
pub const R7S72100_CLK_SG1: c_int = 4;
pub const R7S72100_CLK_SG2: c_int = 3;
pub const R7S72100_CLK_SG3: c_int = 2;
pub const R7S72100_CLK_OSTM0: c_int = 1;
pub const R7S72100_CLK_OSTM1: c_int = 0;
// MSTP6
pub const R7S72100_CLK_ADC: c_int = 7;
pub const R7S72100_CLK_CEU: c_int = 6;
pub const R7S72100_CLK_DOC0: c_int = 5;
pub const R7S72100_CLK_DOC1: c_int = 4;
pub const R7S72100_CLK_DRC0: c_int = 3;
pub const R7S72100_CLK_DRC1: c_int = 2;
pub const R7S72100_CLK_JCU: c_int = 1;
pub const R7S72100_CLK_RTC: c_int = 0;
// MSTP7
pub const R7S72100_CLK_VDEC0: c_int = 7;
pub const R7S72100_CLK_VDEC1: c_int = 6;
pub const R7S72100_CLK_ETHER: c_int = 4;
pub const R7S72100_CLK_NAND: c_int = 3;
pub const R7S72100_CLK_USB0: c_int = 1;
pub const R7S72100_CLK_USB1: c_int = 0;
// MSTP8
pub const R7S72100_CLK_IMR0: c_int = 7;
pub const R7S72100_CLK_IMR1: c_int = 6;
pub const R7S72100_CLK_IMRDISP: c_int = 5;
pub const R7S72100_CLK_MMCIF: c_int = 4;
pub const R7S72100_CLK_MLB: c_int = 3;
pub const R7S72100_CLK_ETHAVB: c_int = 2;
pub const R7S72100_CLK_SCUX: c_int = 1;
// MSTP9
pub const R7S72100_CLK_I2C0: c_int = 7;
pub const R7S72100_CLK_I2C1: c_int = 6;
pub const R7S72100_CLK_I2C2: c_int = 5;
pub const R7S72100_CLK_I2C3: c_int = 4;
pub const R7S72100_CLK_SPIBSC0: c_int = 3;
pub const R7S72100_CLK_SPIBSC1: c_int = 2;

pub const R7S72100_CLK_VDC51: c_int = 0;
// MSTP10
pub const R7S72100_CLK_SPI0: c_int = 7;
pub const R7S72100_CLK_SPI1: c_int = 6;
pub const R7S72100_CLK_SPI2: c_int = 5;
pub const R7S72100_CLK_SPI3: c_int = 4;
pub const R7S72100_CLK_SPI4: c_int = 3;
pub const R7S72100_CLK_CDROM: c_int = 2;
pub const R7S72100_CLK_SPDIF: c_int = 1;
pub const R7S72100_CLK_RGPVG2: c_int = 0;
// MSTP11
pub const R7S72100_CLK_SSI0: c_int = 5;
pub const R7S72100_CLK_SSI1: c_int = 4;
pub const R7S72100_CLK_SSI2: c_int = 3;
pub const R7S72100_CLK_SSI3: c_int = 2;
pub const R7S72100_CLK_SSI4: c_int = 1;
pub const R7S72100_CLK_SSI5: c_int = 0;
// MSTP12
pub const R7S72100_CLK_SDHI00: c_int = 3;
pub const R7S72100_CLK_SDHI01: c_int = 2;
pub const R7S72100_CLK_SDHI10: c_int = 1;
pub const R7S72100_CLK_SDHI11: c_int = 0;
// MSTP13
pub const R7S72100_CLK_PIX1: c_int = 2;
pub const R7S72100_CLK_PIX0: c_int = 1;
