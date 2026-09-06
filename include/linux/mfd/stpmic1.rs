//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/stpmic1.h
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
// Copyright (C) STMicroelectronics 2018 - All Rights Reserved
// Author: Philippe Peurichard <philippe.peurichard@st.com>,
// Pascal Paillet <p.paillet@st.com> for STMicroelectronics.
//
pub const TURN_ON_SR: c_uint = 0x1;
pub const TURN_OFF_SR: c_uint = 0x2;
pub const ICC_LDO_TURN_OFF_SR: c_uint = 0x3;
pub const ICC_BUCK_TURN_OFF_SR: c_uint = 0x4;
pub const RREQ_STATE_SR: c_uint = 0x5;
pub const VERSION_SR: c_uint = 0x6;
pub const MAIN_CR: c_uint = 0x10;
pub const PADS_PULL_CR: c_uint = 0x11;
pub const BUCKS_PD_CR: c_uint = 0x12;
pub const LDO14_PD_CR: c_uint = 0x13;
pub const LDO56_VREF_PD_CR: c_uint = 0x14;
pub const VBUS_DET_VIN_CR: c_uint = 0x15;
pub const PKEY_TURNOFF_CR: c_uint = 0x16;
pub const BUCKS_MASK_RANK_CR: c_uint = 0x17;
pub const BUCKS_MASK_RESET_CR: c_uint = 0x18;
pub const LDOS_MASK_RANK_CR: c_uint = 0x19;
pub const LDOS_MASK_RESET_CR: c_uint = 0x1A;
pub const WCHDG_CR: c_uint = 0x1B;
pub const WCHDG_TIMER_CR: c_uint = 0x1C;
pub const BUCKS_ICCTO_CR: c_uint = 0x1D;
pub const LDOS_ICCTO_CR: c_uint = 0x1E;
pub const BUCK1_ACTIVE_CR: c_uint = 0x20;
pub const BUCK2_ACTIVE_CR: c_uint = 0x21;
pub const BUCK3_ACTIVE_CR: c_uint = 0x22;
pub const BUCK4_ACTIVE_CR: c_uint = 0x23;
pub const VREF_DDR_ACTIVE_CR: c_uint = 0x24;
pub const LDO1_ACTIVE_CR: c_uint = 0x25;
pub const LDO2_ACTIVE_CR: c_uint = 0x26;
pub const LDO3_ACTIVE_CR: c_uint = 0x27;
pub const LDO4_ACTIVE_CR: c_uint = 0x28;
pub const LDO5_ACTIVE_CR: c_uint = 0x29;
pub const LDO6_ACTIVE_CR: c_uint = 0x2A;
pub const BUCK1_STDBY_CR: c_uint = 0x30;
pub const BUCK2_STDBY_CR: c_uint = 0x31;
pub const BUCK3_STDBY_CR: c_uint = 0x32;
pub const BUCK4_STDBY_CR: c_uint = 0x33;
pub const VREF_DDR_STDBY_CR: c_uint = 0x34;
pub const LDO1_STDBY_CR: c_uint = 0x35;
pub const LDO2_STDBY_CR: c_uint = 0x36;
pub const LDO3_STDBY_CR: c_uint = 0x37;
pub const LDO4_STDBY_CR: c_uint = 0x38;
pub const LDO5_STDBY_CR: c_uint = 0x39;
pub const LDO6_STDBY_CR: c_uint = 0x3A;
pub const BST_SW_CR: c_uint = 0x40;
pub const INT_PENDING_R1: c_uint = 0x50;
pub const INT_PENDING_R2: c_uint = 0x51;
pub const INT_PENDING_R3: c_uint = 0x52;
pub const INT_PENDING_R4: c_uint = 0x53;
pub const INT_DBG_LATCH_R1: c_uint = 0x60;
pub const INT_DBG_LATCH_R2: c_uint = 0x61;
pub const INT_DBG_LATCH_R3: c_uint = 0x62;
pub const INT_DBG_LATCH_R4: c_uint = 0x63;
pub const INT_CLEAR_R1: c_uint = 0x70;
pub const INT_CLEAR_R2: c_uint = 0x71;
pub const INT_CLEAR_R3: c_uint = 0x72;
pub const INT_CLEAR_R4: c_uint = 0x73;
pub const INT_MASK_R1: c_uint = 0x80;
pub const INT_MASK_R2: c_uint = 0x81;
pub const INT_MASK_R3: c_uint = 0x82;
pub const INT_MASK_R4: c_uint = 0x83;
pub const INT_SET_MASK_R1: c_uint = 0x90;
pub const INT_SET_MASK_R2: c_uint = 0x91;
pub const INT_SET_MASK_R3: c_uint = 0x92;
pub const INT_SET_MASK_R4: c_uint = 0x93;
pub const INT_CLEAR_MASK_R1: c_uint = 0xA0;
pub const INT_CLEAR_MASK_R2: c_uint = 0xA1;
pub const INT_CLEAR_MASK_R3: c_uint = 0xA2;
pub const INT_CLEAR_MASK_R4: c_uint = 0xA3;
pub const INT_SRC_R1: c_uint = 0xB0;
pub const INT_SRC_R2: c_uint = 0xB1;
pub const INT_SRC_R3: c_uint = 0xB2;
pub const INT_SRC_R4: c_uint = 0xB3;

pub const STPMIC1_PMIC_NUM_IRQ_REGS: c_int = 4;
pub const TURN_OFF_SR_ICC_EVENT: c_uint = 0x08;

pub const LDO_BUCK_VOLTAGE_SHIFT: c_int = 2;

pub const BUCK_HPLP_SHIFT: c_int = 1;

// Main PMIC Control Register
// MAIN_CR
// Address : 0x10
//

// Main PMIC PADS Control Register
// PADS_PULL_CR
// Address : 0x11
//

// Main PMIC VINLOW Control Register
// VBUS_DET_VIN_CRC DMSC
// Address : 0x15
//

// USB Control Register
// Address : 0x40
//

// PKEY_TURNOFF_CR
// Address : 0x16
//

//
// struct stpmic1 - stpmic1 master device for sub-drivers
// @dev: master device of the chip (can be used to access platform data)
// @irq: main IRQ number
// @regmap_irq_chip_data: irq chip data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stpmic1 {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub irq: c_int,
    pub irq_data: *mut regmap_irq_chip_data,
}
