//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/histb-clock.h
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
// clocks provided by core CRG
pub const HISTB_OSC_CLK: c_int = 0;
pub const HISTB_APB_CLK: c_int = 1;
pub const HISTB_AHB_CLK: c_int = 2;
pub const HISTB_UART1_CLK: c_int = 3;
pub const HISTB_UART2_CLK: c_int = 4;
pub const HISTB_UART3_CLK: c_int = 5;
pub const HISTB_I2C0_CLK: c_int = 6;
pub const HISTB_I2C1_CLK: c_int = 7;
pub const HISTB_I2C2_CLK: c_int = 8;
pub const HISTB_I2C3_CLK: c_int = 9;
pub const HISTB_I2C4_CLK: c_int = 10;
pub const HISTB_I2C5_CLK: c_int = 11;
pub const HISTB_SPI0_CLK: c_int = 12;
pub const HISTB_SPI1_CLK: c_int = 13;
pub const HISTB_SPI2_CLK: c_int = 14;
pub const HISTB_SCI_CLK: c_int = 15;
pub const HISTB_FMC_CLK: c_int = 16;
pub const HISTB_MMC_BIU_CLK: c_int = 17;
pub const HISTB_MMC_CIU_CLK: c_int = 18;
pub const HISTB_MMC_DRV_CLK: c_int = 19;
pub const HISTB_MMC_SAMPLE_CLK: c_int = 20;
pub const HISTB_SDIO0_BIU_CLK: c_int = 21;
pub const HISTB_SDIO0_CIU_CLK: c_int = 22;
pub const HISTB_SDIO0_DRV_CLK: c_int = 23;
pub const HISTB_SDIO0_SAMPLE_CLK: c_int = 24;
pub const HISTB_PCIE_AUX_CLK: c_int = 25;
pub const HISTB_PCIE_PIPE_CLK: c_int = 26;
pub const HISTB_PCIE_SYS_CLK: c_int = 27;
pub const HISTB_PCIE_BUS_CLK: c_int = 28;
pub const HISTB_ETH0_MAC_CLK: c_int = 29;
pub const HISTB_ETH0_MACIF_CLK: c_int = 30;
pub const HISTB_ETH1_MAC_CLK: c_int = 31;
pub const HISTB_ETH1_MACIF_CLK: c_int = 32;
pub const HISTB_COMBPHY1_CLK: c_int = 33;
pub const HISTB_USB2_BUS_CLK: c_int = 34;
pub const HISTB_USB2_PHY_CLK: c_int = 35;
pub const HISTB_USB2_UTMI_CLK: c_int = 36;
pub const HISTB_USB2_12M_CLK: c_int = 37;
pub const HISTB_USB2_48M_CLK: c_int = 38;
pub const HISTB_USB2_OTG_UTMI_CLK: c_int = 39;
pub const HISTB_USB2_PHY1_REF_CLK: c_int = 40;
pub const HISTB_USB2_PHY2_REF_CLK: c_int = 41;
pub const HISTB_COMBPHY0_CLK: c_int = 42;
pub const HISTB_USB3_BUS_CLK: c_int = 43;
pub const HISTB_USB3_UTMI_CLK: c_int = 44;
pub const HISTB_USB3_PIPE_CLK: c_int = 45;
pub const HISTB_USB3_SUSPEND_CLK: c_int = 46;
pub const HISTB_USB3_BUS_CLK1: c_int = 47;
pub const HISTB_USB3_UTMI_CLK1: c_int = 48;
pub const HISTB_USB3_PIPE_CLK1: c_int = 49;
pub const HISTB_USB3_SUSPEND_CLK1: c_int = 50;
// clocks provided by mcu CRG
pub const HISTB_MCE_CLK: c_int = 1;
pub const HISTB_IR_CLK: c_int = 2;
pub const HISTB_TIMER01_CLK: c_int = 3;
pub const HISTB_LEDC_CLK: c_int = 4;
pub const HISTB_UART0_CLK: c_int = 5;
pub const HISTB_LSADC_CLK: c_int = 6;
