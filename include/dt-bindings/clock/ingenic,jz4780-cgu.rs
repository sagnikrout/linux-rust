//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/ingenic,jz4780-cgu.h
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
// This header provides clock numbers for the ingenic,jz4780-cgu DT binding.
//
// They are roughly ordered as:
// - external clocks
// - PLLs
// - muxes/dividers in the order they appear in the jz4780 programmers manual
// - gates in order of their bit in the CLKGR* registers
//
pub const JZ4780_CLK_EXCLK: c_int = 0;
pub const JZ4780_CLK_RTCLK: c_int = 1;
pub const JZ4780_CLK_APLL: c_int = 2;
pub const JZ4780_CLK_MPLL: c_int = 3;
pub const JZ4780_CLK_EPLL: c_int = 4;
pub const JZ4780_CLK_VPLL: c_int = 5;
pub const JZ4780_CLK_OTGPHY: c_int = 6;
pub const JZ4780_CLK_SCLKA: c_int = 7;
pub const JZ4780_CLK_CPUMUX: c_int = 8;
pub const JZ4780_CLK_CPU: c_int = 9;
pub const JZ4780_CLK_L2CACHE: c_int = 10;
pub const JZ4780_CLK_AHB0: c_int = 11;
pub const JZ4780_CLK_AHB2PMUX: c_int = 12;
pub const JZ4780_CLK_AHB2: c_int = 13;
pub const JZ4780_CLK_PCLK: c_int = 14;
pub const JZ4780_CLK_DDR: c_int = 15;
pub const JZ4780_CLK_VPU: c_int = 16;
pub const JZ4780_CLK_I2SPLL: c_int = 17;
pub const JZ4780_CLK_I2S: c_int = 18;
pub const JZ4780_CLK_LCD0PIXCLK: c_int = 19;
pub const JZ4780_CLK_LCD1PIXCLK: c_int = 20;
pub const JZ4780_CLK_MSCMUX: c_int = 21;
pub const JZ4780_CLK_MSC0: c_int = 22;
pub const JZ4780_CLK_MSC1: c_int = 23;
pub const JZ4780_CLK_MSC2: c_int = 24;
pub const JZ4780_CLK_UHC: c_int = 25;
pub const JZ4780_CLK_SSIPLL: c_int = 26;
pub const JZ4780_CLK_SSI: c_int = 27;
pub const JZ4780_CLK_CIMMCLK: c_int = 28;
pub const JZ4780_CLK_PCMPLL: c_int = 29;
pub const JZ4780_CLK_PCM: c_int = 30;
pub const JZ4780_CLK_GPU: c_int = 31;
pub const JZ4780_CLK_HDMI: c_int = 32;
pub const JZ4780_CLK_BCH: c_int = 33;
pub const JZ4780_CLK_NEMC: c_int = 34;
pub const JZ4780_CLK_OTG0: c_int = 35;
pub const JZ4780_CLK_SSI0: c_int = 36;
pub const JZ4780_CLK_SMB0: c_int = 37;
pub const JZ4780_CLK_SMB1: c_int = 38;
pub const JZ4780_CLK_SCC: c_int = 39;
pub const JZ4780_CLK_AIC: c_int = 40;
pub const JZ4780_CLK_TSSI0: c_int = 41;
pub const JZ4780_CLK_OWI: c_int = 42;
pub const JZ4780_CLK_KBC: c_int = 43;
pub const JZ4780_CLK_SADC: c_int = 44;
pub const JZ4780_CLK_UART0: c_int = 45;
pub const JZ4780_CLK_UART1: c_int = 46;
pub const JZ4780_CLK_UART2: c_int = 47;
pub const JZ4780_CLK_UART3: c_int = 48;
pub const JZ4780_CLK_SSI1: c_int = 49;
pub const JZ4780_CLK_SSI2: c_int = 50;
pub const JZ4780_CLK_PDMA: c_int = 51;
pub const JZ4780_CLK_GPS: c_int = 52;
pub const JZ4780_CLK_MAC: c_int = 53;
pub const JZ4780_CLK_SMB2: c_int = 54;
pub const JZ4780_CLK_CIM: c_int = 55;
pub const JZ4780_CLK_LCD: c_int = 56;
pub const JZ4780_CLK_TVE: c_int = 57;
pub const JZ4780_CLK_IPU: c_int = 58;
pub const JZ4780_CLK_DDR0: c_int = 59;
pub const JZ4780_CLK_DDR1: c_int = 60;
pub const JZ4780_CLK_SMB3: c_int = 61;
pub const JZ4780_CLK_TSSI1: c_int = 62;
pub const JZ4780_CLK_COMPRESS: c_int = 63;
pub const JZ4780_CLK_AIC1: c_int = 64;
pub const JZ4780_CLK_GPVLC: c_int = 65;
pub const JZ4780_CLK_OTG1: c_int = 66;
pub const JZ4780_CLK_UART4: c_int = 67;
pub const JZ4780_CLK_AHBMON: c_int = 68;
pub const JZ4780_CLK_SMB4: c_int = 69;
pub const JZ4780_CLK_DES: c_int = 70;
pub const JZ4780_CLK_X2D: c_int = 71;
pub const JZ4780_CLK_CORE1: c_int = 72;
pub const JZ4780_CLK_EXCLK_DIV512: c_int = 73;
pub const JZ4780_CLK_RTC: c_int = 74;
