//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/ultrarisc,dp1000-clk.h
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
pub const DP1000_CLK_SYSPLL: c_int = 0;
pub const DP1000_CLK_SYSPLL_DIV2: c_int = 1;
pub const DP1000_CLK_SUBSYS: c_int = 2;
pub const DP1000_CLK_GMAC: c_int = 3;
pub const DP1000_CLK_UART_ROOT: c_int = 4;
pub const DP1000_CLK_I2C_ROOT: c_int = 5;
pub const DP1000_CLK_SPI_ROOT: c_int = 6;
pub const DP1000_CLK_PCIE_DBI: c_int = 7;
pub const DP1000_CLK_PCIEX4_CORE: c_int = 8;
pub const DP1000_CLK_PCIEX16_CORE: c_int = 9;
pub const DP1000_CLK_PCIE_AUX: c_int = 10;
pub const DP1000_CLK_UART0: c_int = 11;
pub const DP1000_CLK_UART1: c_int = 12;
pub const DP1000_CLK_UART2: c_int = 13;
pub const DP1000_CLK_UART3: c_int = 14;
pub const DP1000_CLK_I2C0: c_int = 15;
pub const DP1000_CLK_I2C1: c_int = 16;
pub const DP1000_CLK_I2C2: c_int = 17;
pub const DP1000_CLK_I2C3: c_int = 18;
pub const DP1000_CLK_SPI0: c_int = 19;
pub const DP1000_CLK_SPI1: c_int = 20;
