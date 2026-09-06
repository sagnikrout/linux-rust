//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/mpc512x-clock.h
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
// This header provides constants for MPC512x clock specs in DT bindings.
//

// Macro flag: #define _DT_BINDINGS_CLOCK_MPC512x_CLOCK_H
pub const MPC512x_CLK_DUMMY: c_int = 0;
pub const MPC512x_CLK_REF: c_int = 1;
pub const MPC512x_CLK_SYS: c_int = 2;
pub const MPC512x_CLK_DIU: c_int = 3;
pub const MPC512x_CLK_VIU: c_int = 4;
pub const MPC512x_CLK_CSB: c_int = 5;
pub const MPC512x_CLK_E300: c_int = 6;
pub const MPC512x_CLK_IPS: c_int = 7;
pub const MPC512x_CLK_FEC: c_int = 8;
pub const MPC512x_CLK_SATA: c_int = 9;
pub const MPC512x_CLK_PATA: c_int = 10;
pub const MPC512x_CLK_NFC: c_int = 11;
pub const MPC512x_CLK_LPC: c_int = 12;
pub const MPC512x_CLK_MBX_BUS: c_int = 13;
pub const MPC512x_CLK_MBX: c_int = 14;
pub const MPC512x_CLK_MBX_3D: c_int = 15;
pub const MPC512x_CLK_AXE: c_int = 16;
pub const MPC512x_CLK_USB1: c_int = 17;
pub const MPC512x_CLK_USB2: c_int = 18;
pub const MPC512x_CLK_I2C: c_int = 19;
pub const MPC512x_CLK_MSCAN0_MCLK: c_int = 20;
pub const MPC512x_CLK_MSCAN1_MCLK: c_int = 21;
pub const MPC512x_CLK_MSCAN2_MCLK: c_int = 22;
pub const MPC512x_CLK_MSCAN3_MCLK: c_int = 23;
pub const MPC512x_CLK_BDLC: c_int = 24;
pub const MPC512x_CLK_SDHC: c_int = 25;
pub const MPC512x_CLK_PCI: c_int = 26;
pub const MPC512x_CLK_PSC_MCLK_IN: c_int = 27;
pub const MPC512x_CLK_SPDIF_TX: c_int = 28;
pub const MPC512x_CLK_SPDIF_RX: c_int = 29;
pub const MPC512x_CLK_SPDIF_MCLK: c_int = 30;
pub const MPC512x_CLK_SPDIF: c_int = 31;
pub const MPC512x_CLK_AC97: c_int = 32;
pub const MPC512x_CLK_PSC0_MCLK: c_int = 33;
pub const MPC512x_CLK_PSC1_MCLK: c_int = 34;
pub const MPC512x_CLK_PSC2_MCLK: c_int = 35;
pub const MPC512x_CLK_PSC3_MCLK: c_int = 36;
pub const MPC512x_CLK_PSC4_MCLK: c_int = 37;
pub const MPC512x_CLK_PSC5_MCLK: c_int = 38;
pub const MPC512x_CLK_PSC6_MCLK: c_int = 39;
pub const MPC512x_CLK_PSC7_MCLK: c_int = 40;
pub const MPC512x_CLK_PSC8_MCLK: c_int = 41;
pub const MPC512x_CLK_PSC9_MCLK: c_int = 42;
pub const MPC512x_CLK_PSC10_MCLK: c_int = 43;
pub const MPC512x_CLK_PSC11_MCLK: c_int = 44;
pub const MPC512x_CLK_PSC_FIFO: c_int = 45;
pub const MPC512x_CLK_PSC0: c_int = 46;
pub const MPC512x_CLK_PSC1: c_int = 47;
pub const MPC512x_CLK_PSC2: c_int = 48;
pub const MPC512x_CLK_PSC3: c_int = 49;
pub const MPC512x_CLK_PSC4: c_int = 50;
pub const MPC512x_CLK_PSC5: c_int = 51;
pub const MPC512x_CLK_PSC6: c_int = 52;
pub const MPC512x_CLK_PSC7: c_int = 53;
pub const MPC512x_CLK_PSC8: c_int = 54;
pub const MPC512x_CLK_PSC9: c_int = 55;
pub const MPC512x_CLK_PSC10: c_int = 56;
pub const MPC512x_CLK_PSC11: c_int = 57;
pub const MPC512x_CLK_SDHC2: c_int = 58;
pub const MPC512x_CLK_FEC2: c_int = 59;
pub const MPC512x_CLK_OUT0_CLK: c_int = 60;
pub const MPC512x_CLK_OUT1_CLK: c_int = 61;
pub const MPC512x_CLK_OUT2_CLK: c_int = 62;
pub const MPC512x_CLK_OUT3_CLK: c_int = 63;
pub const MPC512x_CLK_CAN_CLK_IN: c_int = 64;
pub const MPC512x_CLK_LAST_PUBLIC: c_int = 64;
