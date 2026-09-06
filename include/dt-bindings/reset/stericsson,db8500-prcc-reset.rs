//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/stericsson,db8500-prcc-reset.h
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
pub const DB8500_PRCC_1: c_int = 1;
pub const DB8500_PRCC_2: c_int = 2;
pub const DB8500_PRCC_3: c_int = 3;
pub const DB8500_PRCC_6: c_int = 6;
// Reset lines on PRCC 1
pub const DB8500_PRCC_1_RESET_UART0: c_int = 0;
pub const DB8500_PRCC_1_RESET_UART1: c_int = 1;
pub const DB8500_PRCC_1_RESET_I2C1: c_int = 2;
pub const DB8500_PRCC_1_RESET_MSP0: c_int = 3;
pub const DB8500_PRCC_1_RESET_MSP1: c_int = 4;
pub const DB8500_PRCC_1_RESET_SDI0: c_int = 5;
pub const DB8500_PRCC_1_RESET_I2C2: c_int = 6;
pub const DB8500_PRCC_1_RESET_SPI3: c_int = 7;
pub const DB8500_PRCC_1_RESET_SLIMBUS0: c_int = 8;
pub const DB8500_PRCC_1_RESET_I2C4: c_int = 9;
pub const DB8500_PRCC_1_RESET_MSP3: c_int = 10;
pub const DB8500_PRCC_1_RESET_PER_MSP3: c_int = 11;
pub const DB8500_PRCC_1_RESET_PER_MSP1: c_int = 12;
pub const DB8500_PRCC_1_RESET_PER_MSP0: c_int = 13;
pub const DB8500_PRCC_1_RESET_PER_SLIMBUS: c_int = 14;
// Reset lines on PRCC 2
pub const DB8500_PRCC_2_RESET_I2C3: c_int = 0;
pub const DB8500_PRCC_2_RESET_PWL: c_int = 1;
pub const DB8500_PRCC_2_RESET_SDI4: c_int = 2;
pub const DB8500_PRCC_2_RESET_MSP2: c_int = 3;
pub const DB8500_PRCC_2_RESET_SDI1: c_int = 4;
pub const DB8500_PRCC_2_RESET_SDI3: c_int = 5;
pub const DB8500_PRCC_2_RESET_HSIRX: c_int = 6;
pub const DB8500_PRCC_2_RESET_HSITX: c_int = 7;
pub const DB8500_PRCC_1_RESET_PER_MSP2: c_int = 8;
// Reset lines on PRCC 3
pub const DB8500_PRCC_3_RESET_SSP0: c_int = 1;
pub const DB8500_PRCC_3_RESET_SSP1: c_int = 2;
pub const DB8500_PRCC_3_RESET_I2C0: c_int = 3;
pub const DB8500_PRCC_3_RESET_SDI2: c_int = 4;
pub const DB8500_PRCC_3_RESET_SKE: c_int = 5;
pub const DB8500_PRCC_3_RESET_UART2: c_int = 6;
pub const DB8500_PRCC_3_RESET_SDI5: c_int = 7;
// Reset lines on PRCC 6
pub const DB8500_PRCC_3_RESET_RNG: c_int = 0;
