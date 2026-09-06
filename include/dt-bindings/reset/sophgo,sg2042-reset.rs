//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/sophgo,sg2042-reset.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-2-Clause
//
// Copyright (C) 2023 Sophgo Technology Inc. All rights reserved.
//
pub const RST_MAIN_AP: c_int = 0;
pub const RST_RISCV_CPU: c_int = 1;
pub const RST_RISCV_LOW_SPEED_LOGIC: c_int = 2;
pub const RST_RISCV_CMN: c_int = 3;
pub const RST_HSDMA: c_int = 4;
pub const RST_SYSDMA: c_int = 5;
pub const RST_EFUSE0: c_int = 6;
pub const RST_EFUSE1: c_int = 7;
pub const RST_RTC: c_int = 8;
pub const RST_TIMER: c_int = 9;
pub const RST_WDT: c_int = 10;
pub const RST_AHB_ROM0: c_int = 11;
pub const RST_AHB_ROM1: c_int = 12;
pub const RST_I2C0: c_int = 13;
pub const RST_I2C1: c_int = 14;
pub const RST_I2C2: c_int = 15;
pub const RST_I2C3: c_int = 16;
pub const RST_GPIO0: c_int = 17;
pub const RST_GPIO1: c_int = 18;
pub const RST_GPIO2: c_int = 19;
pub const RST_PWM: c_int = 20;
pub const RST_AXI_SRAM0: c_int = 21;
pub const RST_AXI_SRAM1: c_int = 22;
pub const RST_SF0: c_int = 23;
pub const RST_SF1: c_int = 24;
pub const RST_LPC: c_int = 25;
pub const RST_ETH0: c_int = 26;
pub const RST_EMMC: c_int = 27;
pub const RST_SD: c_int = 28;
pub const RST_UART0: c_int = 29;
pub const RST_UART1: c_int = 30;
pub const RST_UART2: c_int = 31;
pub const RST_UART3: c_int = 32;
pub const RST_SPI0: c_int = 33;
pub const RST_SPI1: c_int = 34;
pub const RST_DBG_I2C: c_int = 35;
pub const RST_PCIE0: c_int = 36;
pub const RST_PCIE1: c_int = 37;
pub const RST_DDR0: c_int = 38;
pub const RST_DDR1: c_int = 39;
pub const RST_DDR2: c_int = 40;
pub const RST_DDR3: c_int = 41;
pub const RST_FAU0: c_int = 42;
pub const RST_FAU1: c_int = 43;
pub const RST_FAU2: c_int = 44;
pub const RST_RXU0: c_int = 45;
pub const RST_RXU1: c_int = 46;
pub const RST_RXU2: c_int = 47;
pub const RST_RXU3: c_int = 48;
pub const RST_RXU4: c_int = 49;
pub const RST_RXU5: c_int = 50;
pub const RST_RXU6: c_int = 51;
pub const RST_RXU7: c_int = 52;
pub const RST_RXU8: c_int = 53;
pub const RST_RXU9: c_int = 54;
pub const RST_RXU10: c_int = 55;
pub const RST_RXU11: c_int = 56;
pub const RST_RXU12: c_int = 57;
pub const RST_RXU13: c_int = 58;
pub const RST_RXU14: c_int = 59;
pub const RST_RXU15: c_int = 60;
pub const RST_RXU16: c_int = 61;
pub const RST_RXU17: c_int = 62;
pub const RST_RXU18: c_int = 63;
pub const RST_RXU19: c_int = 64;
pub const RST_RXU20: c_int = 65;
pub const RST_RXU21: c_int = 66;
pub const RST_RXU22: c_int = 67;
pub const RST_RXU23: c_int = 68;
pub const RST_RXU24: c_int = 69;
pub const RST_RXU25: c_int = 70;
pub const RST_RXU26: c_int = 71;
pub const RST_RXU27: c_int = 72;
pub const RST_RXU28: c_int = 73;
pub const RST_RXU29: c_int = 74;
pub const RST_RXU30: c_int = 75;
pub const RST_RXU31: c_int = 76;
