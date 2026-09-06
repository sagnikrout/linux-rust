//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/boot/dts/renesas/rzg3s-smarc-switches.h
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
//
// On-board switches for the Renesas RZ/G3S SMARC Module and RZ SMARC Carrier II
// boards.
//
// Copyright (C) 2024 Renesas Electronics Corp.
//
// On-board switches' states:
// @SW_OFF: switch's state is OFF
// @SW_ON:  switch's state is ON
//
pub const SW_OFF: c_int = 0;
pub const SW_ON: c_int = 1;
//
// SW_CONFIG[x] switches' states:
// @SW_CONFIG2:
// SW_OFF - SD0 is connected to eMMC
// SW_ON  - SD0 is connected to uSD0 card
// @SW_CONFIG3:
// SW_OFF - SD2 is connected to SoC
// SW_ON  - SCIF1, SSI0, IRQ0, IRQ1 connected to SoC
//

//
// SW_OPT_MUX[x] switches' states:
// @SW_OPT_MUX4:
// SW_OFF - The SMARC SER0 signals are routed to M.2 Key E UART
// SW_ON  - The SMARC SER0 signals are routed to PMOD1
//

