//! Automatically rewritten from C Header to Rust Module
//! Source: include/soc/canaan/k210-sysctl.h
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
// Copyright (C) 2019-20 Sean Anderson <seanga2@gmail.com>
// Copyright (c) 2020 Western Digital Corporation or its affiliates.
//
// Kendryte K210 SoC system controller registers offsets.
// Taken from Kendryte SDK (kendryte-standalone-sdk).
//
pub const K210_SYSCTL_GIT_ID: c_uint = 0x00 /* Git short commit id */;
pub const K210_SYSCTL_UART_BAUD: c_uint = 0x04 /* Default UARTHS baud rate */;
pub const K210_SYSCTL_PLL0: c_uint = 0x08 /* PLL0 controller */;
pub const K210_SYSCTL_PLL1: c_uint = 0x0C /* PLL1 controller */;
pub const K210_SYSCTL_PLL2: c_uint = 0x10 /* PLL2 controller */;
pub const K210_SYSCTL_PLL_LOCK: c_uint = 0x18 /* PLL lock tester */;
pub const K210_SYSCTL_ROM_ERROR: c_uint = 0x1C /* AXI ROM detector */;
pub const K210_SYSCTL_SEL0: c_uint = 0x20 /* Clock select controller 0 */;
pub const K210_SYSCTL_SEL1: c_uint = 0x24 /* Clock select controller 1 */;
pub const K210_SYSCTL_EN_CENT: c_uint = 0x28 /* Central clock enable */;
pub const K210_SYSCTL_EN_PERI: c_uint = 0x2C /* Peripheral clock enable */;
pub const K210_SYSCTL_SOFT_RESET: c_uint = 0x30 /* Soft reset ctrl */;
pub const K210_SYSCTL_PERI_RESET: c_uint = 0x34 /* Peripheral reset controller */;
pub const K210_SYSCTL_THR0: c_uint = 0x38 /* Clock threshold controller 0 */;
pub const K210_SYSCTL_THR1: c_uint = 0x3C /* Clock threshold controller 1 */;
pub const K210_SYSCTL_THR2: c_uint = 0x40 /* Clock threshold controller 2 */;
pub const K210_SYSCTL_THR3: c_uint = 0x44 /* Clock threshold controller 3 */;
pub const K210_SYSCTL_THR4: c_uint = 0x48 /* Clock threshold controller 4 */;
pub const K210_SYSCTL_THR5: c_uint = 0x4C /* Clock threshold controller 5 */;
pub const K210_SYSCTL_THR6: c_uint = 0x50 /* Clock threshold controller 6 */;
pub const K210_SYSCTL_MISC: c_uint = 0x54 /* Miscellaneous controller */;
pub const K210_SYSCTL_PERI: c_uint = 0x58 /* Peripheral controller */;
pub const K210_SYSCTL_SPI_SLEEP: c_uint = 0x5C /* SPI sleep controller */;
pub const K210_SYSCTL_RESET_STAT: c_uint = 0x60 /* Reset source status */;
pub const K210_SYSCTL_DMA_SEL0: c_uint = 0x64 /* DMA handshake selector 0 */;
pub const K210_SYSCTL_DMA_SEL1: c_uint = 0x68 /* DMA handshake selector 1 */;
pub const K210_SYSCTL_POWER_SEL: c_uint = 0x6C /* IO Power Mode Select controller */;
extern "C" {
    pub fn k210_clk_early_init(regs: *mut void __iomem);
}
