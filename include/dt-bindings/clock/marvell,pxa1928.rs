//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/marvell,pxa1928.h
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
// Clock ID values here correspond to the control register offset/4.
//
// apb peripherals
pub const PXA1928_CLK_RTC: c_uint = 0x00;
pub const PXA1928_CLK_TWSI0: c_uint = 0x01;
pub const PXA1928_CLK_TWSI1: c_uint = 0x02;
pub const PXA1928_CLK_TWSI2: c_uint = 0x03;
pub const PXA1928_CLK_TWSI3: c_uint = 0x04;
pub const PXA1928_CLK_OWIRE: c_uint = 0x05;
pub const PXA1928_CLK_KPC: c_uint = 0x06;
pub const PXA1928_CLK_TB_ROTARY: c_uint = 0x07;
pub const PXA1928_CLK_SW_JTAG: c_uint = 0x08;
pub const PXA1928_CLK_TIMER1: c_uint = 0x09;
pub const PXA1928_CLK_UART0: c_uint = 0x0b;
pub const PXA1928_CLK_UART1: c_uint = 0x0c;
pub const PXA1928_CLK_UART2: c_uint = 0x0d;
pub const PXA1928_CLK_GPIO: c_uint = 0x0e;
pub const PXA1928_CLK_PWM0: c_uint = 0x0f;
pub const PXA1928_CLK_PWM1: c_uint = 0x10;
pub const PXA1928_CLK_PWM2: c_uint = 0x11;
pub const PXA1928_CLK_PWM3: c_uint = 0x12;
pub const PXA1928_CLK_SSP0: c_uint = 0x13;
pub const PXA1928_CLK_SSP1: c_uint = 0x14;
pub const PXA1928_CLK_SSP2: c_uint = 0x15;
pub const PXA1928_CLK_TWSI4: c_uint = 0x1f;
pub const PXA1928_CLK_TWSI5: c_uint = 0x20;
pub const PXA1928_CLK_UART3: c_uint = 0x22;
pub const PXA1928_CLK_THSENS_GLOB: c_uint = 0x24;
pub const PXA1928_CLK_THSENS_CPU: c_uint = 0x26;
pub const PXA1928_CLK_THSENS_VPU: c_uint = 0x27;
pub const PXA1928_CLK_THSENS_GC: c_uint = 0x28;
// axi peripherals
pub const PXA1928_CLK_SDH0: c_uint = 0x15;
pub const PXA1928_CLK_SDH1: c_uint = 0x16;
pub const PXA1928_CLK_USB: c_uint = 0x17;
pub const PXA1928_CLK_NAND: c_uint = 0x18;
pub const PXA1928_CLK_DMA: c_uint = 0x19;
pub const PXA1928_CLK_SDH2: c_uint = 0x3a;
pub const PXA1928_CLK_SDH3: c_uint = 0x3b;
pub const PXA1928_CLK_HSIC: c_uint = 0x3e;
pub const PXA1928_CLK_SDH4: c_uint = 0x57;
pub const PXA1928_CLK_GC3D: c_uint = 0x5d;
pub const PXA1928_CLK_GC2D: c_uint = 0x5f;
