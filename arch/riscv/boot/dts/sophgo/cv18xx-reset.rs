//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/boot/dts/sophgo/cv18xx-reset.h
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


// SPDX-License-Identifier: (GPL-2.0 OR MIT)
//
// Copyright (C) 2025 Inochi Amaoto <inochiama@outlook.com>
//
pub const RST_DDR: c_int = 2;
pub const RST_H264C: c_int = 3;
pub const RST_JPEG: c_int = 4;
pub const RST_H265C: c_int = 5;
pub const RST_VIPSYS: c_int = 6;
pub const RST_TDMA: c_int = 7;
pub const RST_TPU: c_int = 8;
pub const RST_TPUSYS: c_int = 9;
pub const RST_USB: c_int = 11;
pub const RST_ETH0: c_int = 12;
pub const RST_ETH1: c_int = 13;
pub const RST_NAND: c_int = 14;
pub const RST_EMMC: c_int = 15;
pub const RST_SD0: c_int = 16;
pub const RST_SDMA: c_int = 18;
pub const RST_I2S0: c_int = 19;
pub const RST_I2S1: c_int = 20;
pub const RST_I2S2: c_int = 21;
pub const RST_I2S3: c_int = 22;
pub const RST_UART0: c_int = 23;
pub const RST_UART1: c_int = 24;
pub const RST_UART2: c_int = 25;
pub const RST_UART3: c_int = 26;
pub const RST_I2C0: c_int = 27;
pub const RST_I2C1: c_int = 28;
pub const RST_I2C2: c_int = 29;
pub const RST_I2C3: c_int = 30;
pub const RST_I2C4: c_int = 31;
pub const RST_PWM0: c_int = 32;
pub const RST_PWM1: c_int = 33;
pub const RST_PWM2: c_int = 34;
pub const RST_PWM3: c_int = 35;
pub const RST_SPI0: c_int = 40;
pub const RST_SPI1: c_int = 41;
pub const RST_SPI2: c_int = 42;
pub const RST_SPI3: c_int = 43;
pub const RST_GPIO0: c_int = 44;
pub const RST_GPIO1: c_int = 45;
pub const RST_GPIO2: c_int = 46;
pub const RST_EFUSE: c_int = 47;
pub const RST_WDT: c_int = 48;
pub const RST_AHB_ROM: c_int = 49;
pub const RST_SPIC: c_int = 50;
pub const RST_TEMPSEN: c_int = 51;
pub const RST_SARADC: c_int = 52;
pub const RST_COMBO_PHY0: c_int = 58;
pub const RST_SPI_NAND: c_int = 61;
pub const RST_SE: c_int = 62;
pub const RST_UART4: c_int = 74;
pub const RST_GPIO3: c_int = 75;
pub const RST_SYSTEM: c_int = 76;
pub const RST_TIMER: c_int = 77;
pub const RST_TIMER0: c_int = 78;
pub const RST_TIMER1: c_int = 79;
pub const RST_TIMER2: c_int = 80;
pub const RST_TIMER3: c_int = 81;
pub const RST_TIMER4: c_int = 82;
pub const RST_TIMER5: c_int = 83;
pub const RST_TIMER6: c_int = 84;
pub const RST_TIMER7: c_int = 85;
pub const RST_WGN0: c_int = 86;
pub const RST_WGN1: c_int = 87;
pub const RST_WGN2: c_int = 88;
pub const RST_KEYSCAN: c_int = 89;
pub const RST_AUDDAC: c_int = 91;
pub const RST_AUDDAC_APB: c_int = 92;
pub const RST_AUDADC: c_int = 93;
pub const RST_VCSYS: c_int = 95;
pub const RST_ETHPHY: c_int = 96;
pub const RST_ETHPHY_APB: c_int = 97;
pub const RST_AUDSRC: c_int = 98;
pub const RST_VIP_CAM0: c_int = 99;
pub const RST_WDT1: c_int = 100;
pub const RST_WDT2: c_int = 101;
pub const RST_AUTOCLEAR_CPUCORE0: c_int = 256;
pub const RST_AUTOCLEAR_CPUCORE1: c_int = 257;
pub const RST_AUTOCLEAR_CPUCORE2: c_int = 258;
pub const RST_AUTOCLEAR_CPUCORE3: c_int = 259;
pub const RST_AUTOCLEAR_CPUSYS0: c_int = 260;
pub const RST_AUTOCLEAR_CPUSYS1: c_int = 261;
pub const RST_AUTOCLEAR_CPUSYS2: c_int = 262;
pub const RST_CPUCORE0: c_int = 288;
pub const RST_CPUCORE1: c_int = 289;
pub const RST_CPUCORE2: c_int = 290;
pub const RST_CPUCORE3: c_int = 291;
pub const RST_CPUSYS0: c_int = 292;
pub const RST_CPUSYS1: c_int = 293;
pub const RST_CPUSYS2: c_int = 294;
