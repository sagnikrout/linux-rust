//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/mobileye,eyeq5-clk.h
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
// Copyright (C) 2024 Mobileye Vision Technologies Ltd.
//
pub const EQ5C_PLL_CPU: c_int = 0;
pub const EQ5C_PLL_VMP: c_int = 1;
pub const EQ5C_PLL_PMA: c_int = 2;
pub const EQ5C_PLL_VDI: c_int = 3;
pub const EQ5C_PLL_DDR0: c_int = 4;
pub const EQ5C_PLL_PCI: c_int = 5;
pub const EQ5C_PLL_PER: c_int = 6;
pub const EQ5C_PLL_PMAC: c_int = 7;
pub const EQ5C_PLL_MPC: c_int = 8;
pub const EQ5C_PLL_DDR1: c_int = 9;
pub const EQ5C_DIV_OSPI: c_int = 10;
// EQ5C_PLL_CPU children
pub const EQ5C_CPU_CORE0: c_int = 11;
pub const EQ5C_CPU_CORE1: c_int = 12;
pub const EQ5C_CPU_CORE2: c_int = 13;
pub const EQ5C_CPU_CORE3: c_int = 14;
// EQ5C_PLL_PER children
pub const EQ5C_PER_OCC: c_int = 15;
pub const EQ5C_PER_UART: c_int = 16;
pub const EQ5C_PER_SPI: c_int = 17;
pub const EQ5C_PER_I2C: c_int = 18;
pub const EQ5C_PER_GPIO: c_int = 19;
pub const EQ5C_PER_EMMC: c_int = 20;
pub const EQ5C_PER_OCC_PCI: c_int = 21;
pub const EQ6LC_PLL_DDR: c_int = 0;
pub const EQ6LC_PLL_CPU: c_int = 1;
pub const EQ6LC_PLL_PER: c_int = 2;
pub const EQ6LC_PLL_VDI: c_int = 3;
pub const EQ6HC_CENTRAL_PLL_CPU: c_int = 0;
pub const EQ6HC_CENTRAL_CPU_OCC: c_int = 1;
pub const EQ6HC_WEST_PLL_PER: c_int = 0;
pub const EQ6HC_WEST_PER_OCC: c_int = 1;
pub const EQ6HC_WEST_PER_UART: c_int = 2;
pub const EQ6HC_SOUTH_PLL_VDI: c_int = 0;
pub const EQ6HC_SOUTH_PLL_PCIE: c_int = 1;
pub const EQ6HC_SOUTH_PLL_PER: c_int = 2;
pub const EQ6HC_SOUTH_PLL_ISP: c_int = 3;
pub const EQ6HC_SOUTH_DIV_EMMC: c_int = 4;
pub const EQ6HC_SOUTH_DIV_OSPI_REF: c_int = 5;
pub const EQ6HC_SOUTH_DIV_OSPI_SYS: c_int = 6;
pub const EQ6HC_SOUTH_DIV_TSU: c_int = 7;
pub const EQ6HC_ACC_PLL_XNN: c_int = 0;
pub const EQ6HC_ACC_PLL_VMP: c_int = 1;
pub const EQ6HC_ACC_PLL_PMA: c_int = 2;
pub const EQ6HC_ACC_PLL_MPC: c_int = 3;
pub const EQ6HC_ACC_PLL_NOC: c_int = 4;
