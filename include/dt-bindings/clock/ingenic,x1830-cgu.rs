//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/ingenic,x1830-cgu.h
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
// This header provides clock numbers for the ingenic,x1830-cgu DT binding.
//
// They are roughly ordered as:
// - external clocks
// - PLLs
// - muxes/dividers in the order they appear in the x1830 programmers manual
// - gates in order of their bit in the CLKGR* registers
//
pub const X1830_CLK_EXCLK: c_int = 0;
pub const X1830_CLK_RTCLK: c_int = 1;
pub const X1830_CLK_APLL: c_int = 2;
pub const X1830_CLK_MPLL: c_int = 3;
pub const X1830_CLK_EPLL: c_int = 4;
pub const X1830_CLK_VPLL: c_int = 5;
pub const X1830_CLK_OTGPHY: c_int = 6;
pub const X1830_CLK_SCLKA: c_int = 7;
pub const X1830_CLK_CPUMUX: c_int = 8;
pub const X1830_CLK_CPU: c_int = 9;
pub const X1830_CLK_L2CACHE: c_int = 10;
pub const X1830_CLK_AHB0: c_int = 11;
pub const X1830_CLK_AHB2PMUX: c_int = 12;
pub const X1830_CLK_AHB2: c_int = 13;
pub const X1830_CLK_PCLK: c_int = 14;
pub const X1830_CLK_DDR: c_int = 15;
pub const X1830_CLK_MAC: c_int = 16;
pub const X1830_CLK_LCD: c_int = 17;
pub const X1830_CLK_MSCMUX: c_int = 18;
pub const X1830_CLK_MSC0: c_int = 19;
pub const X1830_CLK_MSC1: c_int = 20;
pub const X1830_CLK_SSIPLL: c_int = 21;
pub const X1830_CLK_SSIPLL_DIV2: c_int = 22;
pub const X1830_CLK_SSIMUX: c_int = 23;
pub const X1830_CLK_EMC: c_int = 24;
pub const X1830_CLK_EFUSE: c_int = 25;
pub const X1830_CLK_OTG: c_int = 26;
pub const X1830_CLK_SSI0: c_int = 27;
pub const X1830_CLK_SMB0: c_int = 28;
pub const X1830_CLK_SMB1: c_int = 29;
pub const X1830_CLK_SMB2: c_int = 30;
pub const X1830_CLK_UART0: c_int = 31;
pub const X1830_CLK_UART1: c_int = 32;
pub const X1830_CLK_SSI1: c_int = 33;
pub const X1830_CLK_SFC: c_int = 34;
pub const X1830_CLK_PDMA: c_int = 35;
pub const X1830_CLK_TCU: c_int = 36;
pub const X1830_CLK_DTRNG: c_int = 37;
pub const X1830_CLK_OST: c_int = 38;
pub const X1830_CLK_EXCLK_DIV512: c_int = 39;
pub const X1830_CLK_RTC: c_int = 40;
