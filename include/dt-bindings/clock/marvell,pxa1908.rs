//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/marvell,pxa1908.h
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


// SPDX-License-Identifier: GPL-2.0-only OR BSD-2-Clause
// plls
pub const PXA1908_CLK_CLK32: c_int = 1;
pub const PXA1908_CLK_VCTCXO: c_int = 2;
pub const PXA1908_CLK_PLL1_624: c_int = 3;
pub const PXA1908_CLK_PLL1_416: c_int = 4;
pub const PXA1908_CLK_PLL1_499: c_int = 5;
pub const PXA1908_CLK_PLL1_832: c_int = 6;
pub const PXA1908_CLK_PLL1_1248: c_int = 7;
pub const PXA1908_CLK_PLL1_D2: c_int = 8;
pub const PXA1908_CLK_PLL1_D4: c_int = 9;
pub const PXA1908_CLK_PLL1_D8: c_int = 10;
pub const PXA1908_CLK_PLL1_D16: c_int = 11;
pub const PXA1908_CLK_PLL1_D6: c_int = 12;
pub const PXA1908_CLK_PLL1_D12: c_int = 13;
pub const PXA1908_CLK_PLL1_D24: c_int = 14;
pub const PXA1908_CLK_PLL1_D48: c_int = 15;
pub const PXA1908_CLK_PLL1_D96: c_int = 16;
pub const PXA1908_CLK_PLL1_D13: c_int = 17;
pub const PXA1908_CLK_PLL1_32: c_int = 18;
pub const PXA1908_CLK_PLL1_208: c_int = 19;
pub const PXA1908_CLK_PLL1_117: c_int = 20;
pub const PXA1908_CLK_PLL1_416_GATE: c_int = 21;
pub const PXA1908_CLK_PLL1_624_GATE: c_int = 22;
pub const PXA1908_CLK_PLL1_832_GATE: c_int = 23;
pub const PXA1908_CLK_PLL1_1248_GATE: c_int = 24;
pub const PXA1908_CLK_PLL1_D2_GATE: c_int = 25;
pub const PXA1908_CLK_PLL1_499_EN: c_int = 26;
pub const PXA1908_CLK_PLL2VCO: c_int = 27;
pub const PXA1908_CLK_PLL2: c_int = 28;
pub const PXA1908_CLK_PLL2P: c_int = 29;
pub const PXA1908_CLK_PLL2VCODIV3: c_int = 30;
pub const PXA1908_CLK_PLL3VCO: c_int = 31;
pub const PXA1908_CLK_PLL3: c_int = 32;
pub const PXA1908_CLK_PLL3P: c_int = 33;
pub const PXA1908_CLK_PLL3VCODIV3: c_int = 34;
pub const PXA1908_CLK_PLL4VCO: c_int = 35;
pub const PXA1908_CLK_PLL4: c_int = 36;
pub const PXA1908_CLK_PLL4P: c_int = 37;
pub const PXA1908_CLK_PLL4VCODIV3: c_int = 38;
// apb (apbc) peripherals
pub const PXA1908_CLK_UART0: c_int = 1;
pub const PXA1908_CLK_UART1: c_int = 2;
pub const PXA1908_CLK_GPIO: c_int = 3;
pub const PXA1908_CLK_PWM0: c_int = 4;
pub const PXA1908_CLK_PWM1: c_int = 5;
pub const PXA1908_CLK_PWM2: c_int = 6;
pub const PXA1908_CLK_PWM3: c_int = 7;
pub const PXA1908_CLK_SSP0: c_int = 8;
pub const PXA1908_CLK_SSP1: c_int = 9;
pub const PXA1908_CLK_IPC_RST: c_int = 10;
pub const PXA1908_CLK_RTC: c_int = 11;
pub const PXA1908_CLK_TWSI0: c_int = 12;
pub const PXA1908_CLK_KPC: c_int = 13;
pub const PXA1908_CLK_SWJTAG: c_int = 14;
pub const PXA1908_CLK_SSP2: c_int = 15;
pub const PXA1908_CLK_TWSI1: c_int = 16;
pub const PXA1908_CLK_THERMAL: c_int = 17;
pub const PXA1908_CLK_TWSI3: c_int = 18;
// apb (apbcp) peripherals
pub const PXA1908_CLK_UART2: c_int = 1;
pub const PXA1908_CLK_TWSI2: c_int = 2;
pub const PXA1908_CLK_AICER: c_int = 3;
// axi (apmu) peripherals
pub const PXA1908_CLK_CCIC1: c_int = 1;
pub const PXA1908_CLK_ISP: c_int = 2;
pub const PXA1908_CLK_DSI1: c_int = 3;
pub const PXA1908_CLK_DISP1: c_int = 4;
pub const PXA1908_CLK_CCIC0: c_int = 5;
pub const PXA1908_CLK_SDH0: c_int = 6;
pub const PXA1908_CLK_SDH1: c_int = 7;
pub const PXA1908_CLK_USB: c_int = 8;
pub const PXA1908_CLK_NF: c_int = 9;
pub const PXA1908_CLK_CORE_DEBUG: c_int = 10;
pub const PXA1908_CLK_VPU: c_int = 11;
pub const PXA1908_CLK_GC: c_int = 12;
pub const PXA1908_CLK_SDH2: c_int = 13;
pub const PXA1908_CLK_GC2D: c_int = 14;
pub const PXA1908_CLK_TRACE: c_int = 15;
pub const PXA1908_CLK_DVC_DFC_DEBUG: c_int = 16;
