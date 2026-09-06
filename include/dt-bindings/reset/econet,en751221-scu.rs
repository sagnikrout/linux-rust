//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/econet,en751221-scu.h
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
pub const EN751221_XPON_PHY_RST: c_int = 0;
pub const EN751221_PCM1_ZSI_ISI_RST: c_int = 1;
pub const EN751221_FE_QDMA1_RST: c_int = 2;
pub const EN751221_FE_QDMA2_RST: c_int = 3;
pub const EN751221_FE_UNZIP_RST: c_int = 4;
pub const EN751221_PCM2_RST: c_int = 5;
pub const EN751221_PTM_MAC_RST: c_int = 6;
pub const EN751221_CRYPTO_RST: c_int = 7;
pub const EN751221_SAR_RST: c_int = 8;
pub const EN751221_TIMER_RST: c_int = 9;
pub const EN751221_INTC_RST: c_int = 10;
pub const EN751221_BONDING_RST: c_int = 11;
pub const EN751221_PCM1_RST: c_int = 12;
pub const EN751221_UART_RST: c_int = 13;
pub const EN751221_GPIO_RST: c_int = 14;
pub const EN751221_GDMA_RST: c_int = 15;
pub const EN751221_I2C_MASTER_RST: c_int = 16;
pub const EN751221_PCM2_ZSI_ISI_RST: c_int = 17;
pub const EN751221_SFC_RST: c_int = 18;
pub const EN751221_UART2_RST: c_int = 19;
pub const EN751221_GDMP_RST: c_int = 20;
pub const EN751221_FE_RST: c_int = 21;
pub const EN751221_USB_HOST_P0_RST: c_int = 22;
pub const EN751221_GSW_RST: c_int = 23;
pub const EN751221_SFC2_PCM_RST: c_int = 24;
pub const EN751221_PCIE0_RST: c_int = 25;
pub const EN751221_PCIE1_RST: c_int = 26;
pub const EN751221_CPU_TIMER_RST: c_int = 27;
pub const EN751221_PCIE_HB_RST: c_int = 28;
pub const EN751221_SIMIF_RST: c_int = 29;
pub const EN751221_XPON_MAC_RST: c_int = 30;
pub const EN751221_GFAST_RST: c_int = 31;
pub const EN751221_CPU_TIMER2_RST: c_int = 32;
pub const EN751221_UART3_RST: c_int = 33;
pub const EN751221_UART4_RST: c_int = 34;
pub const EN751221_UART5_RST: c_int = 35;
pub const EN751221_I2C2_RST: c_int = 36;
pub const EN751221_XSI_MAC_RST: c_int = 37;
pub const EN751221_XSI_PHY_RST: c_int = 38;
pub const EN751221_DMT_RST: c_int = 39;
pub const EN751221_USB_PHY_P0_RST: c_int = 40;
pub const EN751221_USB_PHY_P1_RST: c_int = 41;
