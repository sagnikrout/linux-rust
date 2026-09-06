//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/pinctrl/starfive,jh7110-pinctrl.h
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
//
// Copyright (C) 2022 Emil Renner Berthing <kernel@esmil.dk>
// Copyright (C) 2022 StarFive Technology Co., Ltd.
//
// sys_iomux pins
pub const PAD_GPIO0: c_int = 0;
pub const PAD_GPIO1: c_int = 1;
pub const PAD_GPIO2: c_int = 2;
pub const PAD_GPIO3: c_int = 3;
pub const PAD_GPIO4: c_int = 4;
pub const PAD_GPIO5: c_int = 5;
pub const PAD_GPIO6: c_int = 6;
pub const PAD_GPIO7: c_int = 7;
pub const PAD_GPIO8: c_int = 8;
pub const PAD_GPIO9: c_int = 9;
pub const PAD_GPIO10: c_int = 10;
pub const PAD_GPIO11: c_int = 11;
pub const PAD_GPIO12: c_int = 12;
pub const PAD_GPIO13: c_int = 13;
pub const PAD_GPIO14: c_int = 14;
pub const PAD_GPIO15: c_int = 15;
pub const PAD_GPIO16: c_int = 16;
pub const PAD_GPIO17: c_int = 17;
pub const PAD_GPIO18: c_int = 18;
pub const PAD_GPIO19: c_int = 19;
pub const PAD_GPIO20: c_int = 20;
pub const PAD_GPIO21: c_int = 21;
pub const PAD_GPIO22: c_int = 22;
pub const PAD_GPIO23: c_int = 23;
pub const PAD_GPIO24: c_int = 24;
pub const PAD_GPIO25: c_int = 25;
pub const PAD_GPIO26: c_int = 26;
pub const PAD_GPIO27: c_int = 27;
pub const PAD_GPIO28: c_int = 28;
pub const PAD_GPIO29: c_int = 29;
pub const PAD_GPIO30: c_int = 30;
pub const PAD_GPIO31: c_int = 31;
pub const PAD_GPIO32: c_int = 32;
pub const PAD_GPIO33: c_int = 33;
pub const PAD_GPIO34: c_int = 34;
pub const PAD_GPIO35: c_int = 35;
pub const PAD_GPIO36: c_int = 36;
pub const PAD_GPIO37: c_int = 37;
pub const PAD_GPIO38: c_int = 38;
pub const PAD_GPIO39: c_int = 39;
pub const PAD_GPIO40: c_int = 40;
pub const PAD_GPIO41: c_int = 41;
pub const PAD_GPIO42: c_int = 42;
pub const PAD_GPIO43: c_int = 43;
pub const PAD_GPIO44: c_int = 44;
pub const PAD_GPIO45: c_int = 45;
pub const PAD_GPIO46: c_int = 46;
pub const PAD_GPIO47: c_int = 47;
pub const PAD_GPIO48: c_int = 48;
pub const PAD_GPIO49: c_int = 49;
pub const PAD_GPIO50: c_int = 50;
pub const PAD_GPIO51: c_int = 51;
pub const PAD_GPIO52: c_int = 52;
pub const PAD_GPIO53: c_int = 53;
pub const PAD_GPIO54: c_int = 54;
pub const PAD_GPIO55: c_int = 55;
pub const PAD_GPIO56: c_int = 56;
pub const PAD_GPIO57: c_int = 57;
pub const PAD_GPIO58: c_int = 58;
pub const PAD_GPIO59: c_int = 59;
pub const PAD_GPIO60: c_int = 60;
pub const PAD_GPIO61: c_int = 61;
pub const PAD_GPIO62: c_int = 62;
pub const PAD_GPIO63: c_int = 63;
pub const PAD_SD0_CLK: c_int = 64;
pub const PAD_SD0_CMD: c_int = 65;
pub const PAD_SD0_DATA0: c_int = 66;
pub const PAD_SD0_DATA1: c_int = 67;
pub const PAD_SD0_DATA2: c_int = 68;
pub const PAD_SD0_DATA3: c_int = 69;
pub const PAD_SD0_DATA4: c_int = 70;
pub const PAD_SD0_DATA5: c_int = 71;
pub const PAD_SD0_DATA6: c_int = 72;
pub const PAD_SD0_DATA7: c_int = 73;
pub const PAD_SD0_STRB: c_int = 74;
pub const PAD_GMAC1_MDC: c_int = 75;
pub const PAD_GMAC1_MDIO: c_int = 76;
pub const PAD_GMAC1_RXD0: c_int = 77;
pub const PAD_GMAC1_RXD1: c_int = 78;
pub const PAD_GMAC1_RXD2: c_int = 79;
pub const PAD_GMAC1_RXD3: c_int = 80;
pub const PAD_GMAC1_RXDV: c_int = 81;
pub const PAD_GMAC1_RXC: c_int = 82;
pub const PAD_GMAC1_TXD0: c_int = 83;
pub const PAD_GMAC1_TXD1: c_int = 84;
pub const PAD_GMAC1_TXD2: c_int = 85;
pub const PAD_GMAC1_TXD3: c_int = 86;
pub const PAD_GMAC1_TXEN: c_int = 87;
pub const PAD_GMAC1_TXC: c_int = 88;
pub const PAD_QSPI_SCLK: c_int = 89;
pub const PAD_QSPI_CS0: c_int = 90;
pub const PAD_QSPI_DATA0: c_int = 91;
pub const PAD_QSPI_DATA1: c_int = 92;
pub const PAD_QSPI_DATA2: c_int = 93;
pub const PAD_QSPI_DATA3: c_int = 94;
// aon_iomux pins
pub const PAD_TESTEN: c_int = 0;
pub const PAD_RGPIO0: c_int = 1;
pub const PAD_RGPIO1: c_int = 2;
pub const PAD_RGPIO2: c_int = 3;
pub const PAD_RGPIO3: c_int = 4;
pub const PAD_RSTN: c_int = 5;
pub const PAD_GMAC0_MDC: c_int = 6;
pub const PAD_GMAC0_MDIO: c_int = 7;
pub const PAD_GMAC0_RXD0: c_int = 8;
pub const PAD_GMAC0_RXD1: c_int = 9;
pub const PAD_GMAC0_RXD2: c_int = 10;
pub const PAD_GMAC0_RXD3: c_int = 11;
pub const PAD_GMAC0_RXDV: c_int = 12;
pub const PAD_GMAC0_RXC: c_int = 13;
pub const PAD_GMAC0_TXD0: c_int = 14;
pub const PAD_GMAC0_TXD1: c_int = 15;
pub const PAD_GMAC0_TXD2: c_int = 16;
pub const PAD_GMAC0_TXD3: c_int = 17;
pub const PAD_GMAC0_TXEN: c_int = 18;
pub const PAD_GMAC0_TXC: c_int = 19;
pub const GPOUT_LOW: c_int = 0;
pub const GPOUT_HIGH: c_int = 1;
pub const GPOEN_ENABLE: c_int = 0;
pub const GPOEN_DISABLE: c_int = 1;
pub const GPI_NONE: c_int = 255;
