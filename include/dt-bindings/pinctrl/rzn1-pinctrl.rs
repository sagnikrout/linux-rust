//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/pinctrl/rzn1-pinctrl.h
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
// Defines macros and constants for Renesas RZ/N1 pin controller pin
// muxing functions.
//

//
// Given the different levels of muxing on the SoC, it was decided to
// 'linearize' them into one numerical space. So mux level 1, 2 and the MDIO
// muxes are all represented by one single value.
//
// You can derive the hardware value pretty easily too, as
// 0...9   are Level 1
// 10...71 are Level 2. The Level 2 mux will be set to this
// value - RZN1_FUNC_L2_OFFSET, and the Level 1 mux will be
// set accordingly.
// 72...103 are for the 2 MDIO muxes.
//
pub const RZN1_FUNC_HIGHZ: c_int = 0;
pub const RZN1_FUNC_0L: c_int = 1;
pub const RZN1_FUNC_CLK_ETH_MII_RGMII_RMII: c_int = 2;
pub const RZN1_FUNC_CLK_ETH_NAND: c_int = 3;
pub const RZN1_FUNC_QSPI: c_int = 4;
pub const RZN1_FUNC_SDIO: c_int = 5;
pub const RZN1_FUNC_LCD: c_int = 6;
pub const RZN1_FUNC_LCD_E: c_int = 7;
pub const RZN1_FUNC_MSEBIM: c_int = 8;
pub const RZN1_FUNC_MSEBIS: c_int = 9;

// These are MDIO0 peripherals for the RZN1_FUNC_ETH_MDIO function

// These are MDIO0 peripherals for the RZN1_FUNC_ETH_MDIO_E1 function

// These are MDIO1 peripherals for the RZN1_FUNC_ETH_MDIO function

// These are MDIO1 peripherals for the RZN1_FUNC_ETH_MDIO_E1 function

