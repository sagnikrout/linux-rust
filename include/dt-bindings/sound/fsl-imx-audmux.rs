//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/sound/fsl-imx-audmux.h
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
pub const MX27_AUDMUX_HPCR1_SSI0: c_int = 0;
pub const MX27_AUDMUX_HPCR2_SSI1: c_int = 1;
pub const MX27_AUDMUX_HPCR3_SSI_PINS_4: c_int = 2;
pub const MX27_AUDMUX_PPCR1_SSI_PINS_1: c_int = 3;
pub const MX27_AUDMUX_PPCR2_SSI_PINS_2: c_int = 4;
pub const MX27_AUDMUX_PPCR3_SSI_PINS_3: c_int = 5;
pub const MX31_AUDMUX_PORT1_SSI0: c_int = 0;
pub const MX31_AUDMUX_PORT2_SSI1: c_int = 1;
pub const MX31_AUDMUX_PORT3_SSI_PINS_3: c_int = 2;
pub const MX31_AUDMUX_PORT4_SSI_PINS_4: c_int = 3;
pub const MX31_AUDMUX_PORT5_SSI_PINS_5: c_int = 4;
pub const MX31_AUDMUX_PORT6_SSI_PINS_6: c_int = 5;
pub const MX31_AUDMUX_PORT7_SSI_PINS_7: c_int = 6;
pub const MX51_AUDMUX_PORT1_SSI0: c_int = 0;
pub const MX51_AUDMUX_PORT2_SSI1: c_int = 1;
pub const MX51_AUDMUX_PORT3: c_int = 2;
pub const MX51_AUDMUX_PORT4: c_int = 3;
pub const MX51_AUDMUX_PORT5: c_int = 4;
pub const MX51_AUDMUX_PORT6: c_int = 5;
pub const MX51_AUDMUX_PORT7: c_int = 6;
//
// TFCSEL/RFCSEL (i.MX27) or TFSEL/TCSEL/RFSEL/RCSEL (i.MX31/51/53/6Q)
// can be sourced from Rx/Tx.
//
pub const IMX_AUDMUX_RXFS: c_uint = 0x8;
pub const IMX_AUDMUX_RXCLK: c_uint = 0x8;
// Register definitions for the i.MX21/27 Digital Audio Multiplexer

// Register definitions for the i.MX25/31/35/51 Digital Audio Multiplexer

