//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/canaan,k230-rst.h
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
// Copyright (C) 2023-2024 Canaan Bright Sight Co., Ltd
// Copyright (C) 2024-2025 Junhui Liu <junhui.liu@pigmoral.tech>
//
pub const RST_CPU0: c_int = 0;
pub const RST_CPU1: c_int = 1;
pub const RST_CPU0_FLUSH: c_int = 2;
pub const RST_CPU1_FLUSH: c_int = 3;
pub const RST_AI: c_int = 4;
pub const RST_VPU: c_int = 5;
pub const RST_HISYS: c_int = 6;
pub const RST_HISYS_AHB: c_int = 7;
pub const RST_SDIO0: c_int = 8;
pub const RST_SDIO1: c_int = 9;
pub const RST_SDIO_AXI: c_int = 10;
pub const RST_USB0: c_int = 11;
pub const RST_USB1: c_int = 12;
pub const RST_USB0_AHB: c_int = 13;
pub const RST_USB1_AHB: c_int = 14;
pub const RST_SPI0: c_int = 15;
pub const RST_SPI1: c_int = 16;
pub const RST_SPI2: c_int = 17;
pub const RST_SEC: c_int = 18;
pub const RST_PDMA: c_int = 19;
pub const RST_SDMA: c_int = 20;
pub const RST_DECOMPRESS: c_int = 21;
pub const RST_SRAM: c_int = 22;
pub const RST_SHRM_AXIM: c_int = 23;
pub const RST_SHRM_AXIS: c_int = 24;
pub const RST_NONAI2D: c_int = 25;
pub const RST_MCTL: c_int = 26;
pub const RST_ISP: c_int = 27;
pub const RST_ISP_DW: c_int = 28;
pub const RST_DPU: c_int = 29;
pub const RST_DISP: c_int = 30;
pub const RST_GPU: c_int = 31;
pub const RST_AUDIO: c_int = 32;
pub const RST_TIMER0: c_int = 33;
pub const RST_TIMER1: c_int = 34;
pub const RST_TIMER2: c_int = 35;
pub const RST_TIMER3: c_int = 36;
pub const RST_TIMER4: c_int = 37;
pub const RST_TIMER5: c_int = 38;
pub const RST_TIMER_APB: c_int = 39;
pub const RST_HDI: c_int = 40;
pub const RST_WDT0: c_int = 41;
pub const RST_WDT1: c_int = 42;
pub const RST_WDT0_APB: c_int = 43;
pub const RST_WDT1_APB: c_int = 44;
pub const RST_TS_APB: c_int = 45;
pub const RST_MAILBOX: c_int = 46;
pub const RST_STC: c_int = 47;
pub const RST_PMU: c_int = 48;
pub const RST_LOSYS_APB: c_int = 49;
pub const RST_UART0: c_int = 50;
pub const RST_UART1: c_int = 51;
pub const RST_UART2: c_int = 52;
pub const RST_UART3: c_int = 53;
pub const RST_UART4: c_int = 54;
pub const RST_I2C0: c_int = 55;
pub const RST_I2C1: c_int = 56;
pub const RST_I2C2: c_int = 57;
pub const RST_I2C3: c_int = 58;
pub const RST_I2C4: c_int = 59;
pub const RST_JAMLINK0_APB: c_int = 60;
pub const RST_JAMLINK1_APB: c_int = 61;
pub const RST_JAMLINK2_APB: c_int = 62;
pub const RST_JAMLINK3_APB: c_int = 63;
pub const RST_CODEC_APB: c_int = 64;
pub const RST_GPIO_DB: c_int = 65;
pub const RST_GPIO_APB: c_int = 66;
pub const RST_ADC: c_int = 67;
pub const RST_ADC_APB: c_int = 68;
pub const RST_PWM_APB: c_int = 69;
pub const RST_SHRM_APB: c_int = 70;
pub const RST_CSI0: c_int = 71;
pub const RST_CSI1: c_int = 72;
pub const RST_CSI2: c_int = 73;
pub const RST_CSI_DPHY: c_int = 74;
pub const RST_ISP_AHB: c_int = 75;
pub const RST_M0: c_int = 76;
pub const RST_M1: c_int = 77;
pub const RST_M2: c_int = 78;
pub const RST_SPI2AXI: c_int = 79;
