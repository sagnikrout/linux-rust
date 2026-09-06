//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/sunplus,sp7021-clkc.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
//
// Copyright (C) Sunplus Technology Co., Ltd.
// All rights reserved.
//
// gates
pub const CLK_RTC: c_int = 0;
pub const CLK_OTPRX: c_int = 1;
pub const CLK_NOC: c_int = 2;
pub const CLK_BR: c_int = 3;
pub const CLK_SPIFL: c_int = 4;
pub const CLK_PERI0: c_int = 5;
pub const CLK_PERI1: c_int = 6;
pub const CLK_STC0: c_int = 7;
pub const CLK_STC_AV0: c_int = 8;
pub const CLK_STC_AV1: c_int = 9;
pub const CLK_STC_AV2: c_int = 10;
pub const CLK_UA0: c_int = 11;
pub const CLK_UA1: c_int = 12;
pub const CLK_UA2: c_int = 13;
pub const CLK_UA3: c_int = 14;
pub const CLK_UA4: c_int = 15;
pub const CLK_HWUA: c_int = 16;
pub const CLK_DDC0: c_int = 17;
pub const CLK_UADMA: c_int = 18;
pub const CLK_CBDMA0: c_int = 19;
pub const CLK_CBDMA1: c_int = 20;
pub const CLK_SPI_COMBO_0: c_int = 21;
pub const CLK_SPI_COMBO_1: c_int = 22;
pub const CLK_SPI_COMBO_2: c_int = 23;
pub const CLK_SPI_COMBO_3: c_int = 24;
pub const CLK_AUD: c_int = 25;
pub const CLK_USBC0: c_int = 26;
pub const CLK_USBC1: c_int = 27;
pub const CLK_UPHY0: c_int = 28;
pub const CLK_UPHY1: c_int = 29;
pub const CLK_I2CM0: c_int = 30;
pub const CLK_I2CM1: c_int = 31;
pub const CLK_I2CM2: c_int = 32;
pub const CLK_I2CM3: c_int = 33;
pub const CLK_PMC: c_int = 34;
pub const CLK_CARD_CTL0: c_int = 35;
pub const CLK_CARD_CTL1: c_int = 36;
pub const CLK_CARD_CTL4: c_int = 37;
pub const CLK_BCH: c_int = 38;
pub const CLK_DDFCH: c_int = 39;
pub const CLK_CSIIW0: c_int = 40;
pub const CLK_CSIIW1: c_int = 41;
pub const CLK_MIPICSI0: c_int = 42;
pub const CLK_MIPICSI1: c_int = 43;
pub const CLK_HDMI_TX: c_int = 44;
pub const CLK_VPOST: c_int = 45;
pub const CLK_TGEN: c_int = 46;
pub const CLK_DMIX: c_int = 47;
pub const CLK_TCON: c_int = 48;
pub const CLK_GPIO: c_int = 49;
pub const CLK_MAILBOX: c_int = 50;
pub const CLK_SPIND: c_int = 51;
pub const CLK_I2C2CBUS: c_int = 52;
pub const CLK_SEC: c_int = 53;
pub const CLK_DVE: c_int = 54;
pub const CLK_GPOST0: c_int = 55;
pub const CLK_OSD0: c_int = 56;
pub const CLK_DISP_PWM: c_int = 57;
pub const CLK_UADBG: c_int = 58;
pub const CLK_FIO_CTL: c_int = 59;
pub const CLK_FPGA: c_int = 60;
pub const CLK_L2SW: c_int = 61;
pub const CLK_ICM: c_int = 62;
pub const CLK_AXI_GLOBAL: c_int = 63;
// plls
pub const PLL_A: c_int = 64;
pub const PLL_E: c_int = 65;
pub const PLL_E_2P5: c_int = 66;
pub const PLL_E_25: c_int = 67;
pub const PLL_E_112P5: c_int = 68;
pub const PLL_F: c_int = 69;
pub const PLL_TV: c_int = 70;
pub const PLL_TV_A: c_int = 71;
pub const PLL_SYS: c_int = 72;
pub const CLK_MAX: c_int = 73;
