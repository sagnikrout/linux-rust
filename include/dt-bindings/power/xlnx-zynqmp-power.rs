//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/power/xlnx-zynqmp-power.h
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
// Copyright (C) 2018 Xilinx, Inc.
//
pub const PD_RPU_0: c_int = 7;
pub const PD_RPU_1: c_int = 8;
pub const PD_R5_0_ATCM: c_int = 15;
pub const PD_R5_0_BTCM: c_int = 16;
pub const PD_R5_1_ATCM: c_int = 17;
pub const PD_R5_1_BTCM: c_int = 18;
pub const PD_USB_0: c_int = 22;
pub const PD_USB_1: c_int = 23;
pub const PD_TTC_0: c_int = 24;
pub const PD_TTC_1: c_int = 25;
pub const PD_TTC_2: c_int = 26;
pub const PD_TTC_3: c_int = 27;
pub const PD_SATA: c_int = 28;
pub const PD_ETH_0: c_int = 29;
pub const PD_ETH_1: c_int = 30;
pub const PD_ETH_2: c_int = 31;
pub const PD_ETH_3: c_int = 32;
pub const PD_UART_0: c_int = 33;
pub const PD_UART_1: c_int = 34;
pub const PD_SPI_0: c_int = 35;
pub const PD_SPI_1: c_int = 36;
pub const PD_I2C_0: c_int = 37;
pub const PD_I2C_1: c_int = 38;
pub const PD_SD_0: c_int = 39;
pub const PD_SD_1: c_int = 40;
pub const PD_DP: c_int = 41;
pub const PD_GDMA: c_int = 42;
pub const PD_ADMA: c_int = 43;
pub const PD_NAND: c_int = 44;
pub const PD_QSPI: c_int = 45;
pub const PD_GPIO: c_int = 46;
pub const PD_CAN_0: c_int = 47;
pub const PD_CAN_1: c_int = 48;
pub const PD_GPU: c_int = 58;
pub const PD_PCIE: c_int = 59;
