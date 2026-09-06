//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/intel_soc_pmic_bxtwc.h
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
// Header file for Intel Broxton Whiskey Cove PMIC
//
// Copyright (C) 2015 Intel Corporation. All rights reserved.
//
// BXT WC devices
pub const BXTWC_DEVICE1_ADDR: c_uint = 0x4E;
pub const BXTWC_DEVICE2_ADDR: c_uint = 0x4F;
pub const BXTWC_DEVICE3_ADDR: c_uint = 0x5E;
// device1 Registers
pub const BXTWC_CHIPID: c_uint = 0x4E00;
pub const BXTWC_CHIPVER: c_uint = 0x4E01;
pub const BXTWC_SCHGRIRQ0_ADDR: c_uint = 0x5E1A;
pub const BXTWC_CHGRCTRL0_ADDR: c_uint = 0x5E16;
pub const BXTWC_CHGRCTRL1_ADDR: c_uint = 0x5E17;
pub const BXTWC_CHGRCTRL2_ADDR: c_uint = 0x5E18;
pub const BXTWC_CHGRSTATUS_ADDR: c_uint = 0x5E19;
pub const BXTWC_THRMBATZONE_ADDR: c_uint = 0x4F22;
pub const BXTWC_USBPATH_ADDR: c_uint = 0x5E19;
pub const BXTWC_USBPHYCTRL_ADDR: c_uint = 0x5E07;
pub const BXTWC_USBIDCTRL_ADDR: c_uint = 0x5E05;
pub const BXTWC_USBIDEN_MASK: c_uint = 0x01;
pub const BXTWC_USBIDSTAT_ADDR: c_uint = 0x00FF;
pub const BXTWC_USBSRCDETSTATUS_ADDR: c_uint = 0x5E29;
pub const BXTWC_DBGUSBBC1_ADDR: c_uint = 0x5FE0;
pub const BXTWC_DBGUSBBC2_ADDR: c_uint = 0x5FE1;
pub const BXTWC_DBGUSBBCSTAT_ADDR: c_uint = 0x5FE2;
pub const BXTWC_WAKESRC_ADDR: c_uint = 0x4E22;
pub const BXTWC_WAKESRC2_ADDR: c_uint = 0x4EE5;
pub const BXTWC_CHRTTADDR_ADDR: c_uint = 0x5E22;
pub const BXTWC_CHRTTDATA_ADDR: c_uint = 0x5E23;
pub const BXTWC_STHRMIRQ0_ADDR: c_uint = 0x4F19;
pub const WC_MTHRMIRQ1_ADDR: c_uint = 0x4E12;
pub const WC_STHRMIRQ1_ADDR: c_uint = 0x4F1A;
pub const WC_STHRMIRQ2_ADDR: c_uint = 0x4F1B;
pub const BXTWC_THRMZN0H_ADDR: c_uint = 0x4F44;
pub const BXTWC_THRMZN0L_ADDR: c_uint = 0x4F45;
pub const BXTWC_THRMZN1H_ADDR: c_uint = 0x4F46;
pub const BXTWC_THRMZN1L_ADDR: c_uint = 0x4F47;
pub const BXTWC_THRMZN2H_ADDR: c_uint = 0x4F48;
pub const BXTWC_THRMZN2L_ADDR: c_uint = 0x4F49;
pub const BXTWC_THRMZN3H_ADDR: c_uint = 0x4F4A;
pub const BXTWC_THRMZN3L_ADDR: c_uint = 0x4F4B;
pub const BXTWC_THRMZN4H_ADDR: c_uint = 0x4F4C;
pub const BXTWC_THRMZN4L_ADDR: c_uint = 0x4F4D;
