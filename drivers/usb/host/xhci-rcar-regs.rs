//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/host/xhci-rcar-regs.h
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
// Register Offset
pub const RCAR_USB3_AXH_STA: c_uint = 0x104	/* AXI Host Control Status */;
pub const RCAR_USB3_INT_ENA: c_uint = 0x224	/* Interrupt Enable */;
pub const RCAR_USB3_DL_CTRL: c_uint = 0x250	/* FW Download Control & Status */;
pub const RCAR_USB3_FW_DATA0: c_uint = 0x258	/* FW Data0 */;
pub const RCAR_USB3_LCLK: c_uint = 0xa44	/* LCLK Select */;
pub const RCAR_USB3_CONF1: c_uint = 0xa48	/* USB3.0 Configuration1 */;
pub const RCAR_USB3_CONF2: c_uint = 0xa5c	/* USB3.0 Configuration2 */;
pub const RCAR_USB3_CONF3: c_uint = 0xaa8	/* USB3.0 Configuration3 */;
pub const RCAR_USB3_RX_POL: c_uint = 0xab0	/* USB3.0 RX Polarity */;
pub const RCAR_USB3_TX_POL: c_uint = 0xab8	/* USB3.0 TX Polarity */;
// Register Settings
// AXI Host Control Status
pub const RCAR_USB3_AXH_STA_B3_PLL_ACTIVE: c_uint = 0x00010000;
pub const RCAR_USB3_AXH_STA_B2_PLL_ACTIVE: c_uint = 0x00000001;

// Interrupt Enable
pub const RCAR_USB3_INT_XHC_ENA: c_uint = 0x00000001;
pub const RCAR_USB3_INT_PME_ENA: c_uint = 0x00000002;
pub const RCAR_USB3_INT_HSE_ENA: c_uint = 0x00000004;

// FW Download Control & Status
pub const RCAR_USB3_DL_CTRL_ENABLE: c_uint = 0x00000001;
pub const RCAR_USB3_DL_CTRL_FW_SUCCESS: c_uint = 0x00000010;
pub const RCAR_USB3_DL_CTRL_FW_SET_DATA0: c_uint = 0x00000100;
// LCLK Select
pub const RCAR_USB3_LCLK_ENA_VAL: c_uint = 0x01030001;
// USB3.0 Configuration
pub const RCAR_USB3_CONF1_VAL: c_uint = 0x00030204;
pub const RCAR_USB3_CONF2_VAL: c_uint = 0x00030300;
pub const RCAR_USB3_CONF3_VAL: c_uint = 0x13802007;
// USB3.0 Polarity

