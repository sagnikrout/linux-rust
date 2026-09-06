//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/cix,sky1-system-control.h
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
// Author: Jerry Zhu <jerry.zhu@cixtech.com>
// func reset for sky1 fch
pub const SW_I3C0_RST_FUNC_G_N: c_int = 0;
pub const SW_I3C0_RST_FUNC_I_N: c_int = 1;
pub const SW_I3C1_RST_FUNC_G_N: c_int = 2;
pub const SW_I3C1_RST_FUNC_I_N: c_int = 3;
pub const SW_UART0_RST_FUNC_N: c_int = 4;
pub const SW_UART1_RST_FUNC_N: c_int = 5;
pub const SW_UART2_RST_FUNC_N: c_int = 6;
pub const SW_UART3_RST_FUNC_N: c_int = 7;
pub const SW_TIMER_RST_FUNC_N: c_int = 8;
// apb reset for sky1 fch
pub const SW_I3C0_RST_APB_N: c_int = 9;
pub const SW_I3C1_RST_APB_N: c_int = 10;
pub const SW_DMA_RST_AXI_N: c_int = 11;
pub const SW_UART0_RST_APB_N: c_int = 12;
pub const SW_UART1_RST_APB_N: c_int = 13;
pub const SW_UART2_RST_APB_N: c_int = 14;
pub const SW_UART3_RST_APB_N: c_int = 15;
pub const SW_SPI0_RST_APB_N: c_int = 16;
pub const SW_SPI1_RST_APB_N: c_int = 17;
pub const SW_I2C0_RST_APB_N: c_int = 18;
pub const SW_I2C1_RST_APB_N: c_int = 19;
pub const SW_I2C2_RST_APB_N: c_int = 20;
pub const SW_I2C3_RST_APB_N: c_int = 21;
pub const SW_I2C4_RST_APB_N: c_int = 22;
pub const SW_I2C5_RST_APB_N: c_int = 23;
pub const SW_I2C6_RST_APB_N: c_int = 24;
pub const SW_I2C7_RST_APB_N: c_int = 25;
pub const SW_GPIO_RST_APB_N: c_int = 26;
// fch rst for xspi
pub const SW_XSPI_REG_RST_N: c_int = 27;
pub const SW_XSPI_SYS_RST_N: c_int = 28;
