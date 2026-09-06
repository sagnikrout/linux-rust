//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/hi3516cv300-clock.h
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
// Copyright (c) 2016 HiSilicon Technologies Co., Ltd.
//
// hi3516CV300 core CRG
pub const HI3516CV300_APB_CLK: c_int = 0;
pub const HI3516CV300_UART0_CLK: c_int = 1;
pub const HI3516CV300_UART1_CLK: c_int = 2;
pub const HI3516CV300_UART2_CLK: c_int = 3;
pub const HI3516CV300_SPI0_CLK: c_int = 4;
pub const HI3516CV300_SPI1_CLK: c_int = 5;
pub const HI3516CV300_FMC_CLK: c_int = 6;
pub const HI3516CV300_MMC0_CLK: c_int = 7;
pub const HI3516CV300_MMC1_CLK: c_int = 8;
pub const HI3516CV300_MMC2_CLK: c_int = 9;
pub const HI3516CV300_MMC3_CLK: c_int = 10;
pub const HI3516CV300_ETH_CLK: c_int = 11;
pub const HI3516CV300_ETH_MACIF_CLK: c_int = 12;
pub const HI3516CV300_DMAC_CLK: c_int = 13;
pub const HI3516CV300_PWM_CLK: c_int = 14;
pub const HI3516CV300_USB2_BUS_CLK: c_int = 15;
pub const HI3516CV300_USB2_OHCI48M_CLK: c_int = 16;
pub const HI3516CV300_USB2_OHCI12M_CLK: c_int = 17;
pub const HI3516CV300_USB2_OTG_UTMI_CLK: c_int = 18;
pub const HI3516CV300_USB2_HST_PHY_CLK: c_int = 19;
pub const HI3516CV300_USB2_UTMI0_CLK: c_int = 20;
pub const HI3516CV300_USB2_PHY_CLK: c_int = 21;
// hi3516CV300 sysctrl CRG
pub const HI3516CV300_WDT_CLK: c_int = 1;
