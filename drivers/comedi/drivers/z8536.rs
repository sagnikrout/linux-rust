//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/comedi/drivers/z8536.h
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
// Z8536 CIO Internal registers
//
// Master Interrupt Control register
pub const Z8536_INT_CTRL_REG: c_uint = 0x00;

// Master Configuration Control register
pub const Z8536_CFG_CTRL_REG: c_uint = 0x01;

// Interrupt Vector registers
pub const Z8536_PA_INT_VECT_REG: c_uint = 0x02;
pub const Z8536_PB_INT_VECT_REG: c_uint = 0x03;
pub const Z8536_CT_INT_VECT_REG: c_uint = 0x04;
pub const Z8536_CURR_INT_VECT_REG: c_uint = 0x1f;
// Port A/B & Counter/Timer 1/2/3 Command and Status registers
pub const Z8536_PA_CMDSTAT_REG: c_uint = 0x08;
pub const Z8536_PB_CMDSTAT_REG: c_uint = 0x09;
pub const Z8536_CT1_CMDSTAT_REG: c_uint = 0x0a;
pub const Z8536_CT2_CMDSTAT_REG: c_uint = 0x0b;
pub const Z8536_CT3_CMDSTAT_REG: c_uint = 0x0c;

// Port Data registers
pub const Z8536_PA_DATA_REG: c_uint = 0x0d;
pub const Z8536_PB_DATA_REG: c_uint = 0x0e;
pub const Z8536_PC_DATA_REG: c_uint = 0x0f;
// Counter/Timer 1/2/3 Current Count registers
pub const Z8536_CT1_VAL_MSB_REG: c_uint = 0x10;
pub const Z8536_CT1_VAL_LSB_REG: c_uint = 0x11;
pub const Z8536_CT2_VAL_MSB_REG: c_uint = 0x12;
pub const Z8536_CT2_VAL_LSB_REG: c_uint = 0x13;
pub const Z8536_CT3_VAL_MSB_REG: c_uint = 0x14;
pub const Z8536_CT3_VAL_LSB_REG: c_uint = 0x15;

// Counter/Timer 1/2/3 Time Constant registers
pub const Z8536_CT1_RELOAD_MSB_REG: c_uint = 0x16;
pub const Z8536_CT1_RELOAD_LSB_REG: c_uint = 0x17;
pub const Z8536_CT2_RELOAD_MSB_REG: c_uint = 0x18;
pub const Z8536_CT2_RELOAD_LSB_REG: c_uint = 0x19;
pub const Z8536_CT3_RELOAD_MSB_REG: c_uint = 0x1a;
pub const Z8536_CT3_RELOAD_LSB_REG: c_uint = 0x1b;

// Counter/Timer 1/2/3 Mode Specification registers
pub const Z8536_CT1_MODE_REG: c_uint = 0x1c;
pub const Z8536_CT2_MODE_REG: c_uint = 0x1d;
pub const Z8536_CT3_MODE_REG: c_uint = 0x1e;

// Port A/B Mode Specification registers
pub const Z8536_PA_MODE_REG: c_uint = 0x20;
pub const Z8536_PB_MODE_REG: c_uint = 0x28;

// Port A/B Handshake Specification registers
pub const Z8536_PA_HANDSHAKE_REG: c_uint = 0x21;
pub const Z8536_PB_HANDSHAKE_REG: c_uint = 0x29;

//
// Port A/B/C Data Path Polarity registers
//
// 0 = Non-Inverting
// 1 = Inverting
//
pub const Z8536_PA_DPP_REG: c_uint = 0x22;
pub const Z8536_PB_DPP_REG: c_uint = 0x2a;
pub const Z8536_PC_DPP_REG: c_uint = 0x05;
//
// Port A/B/C Data Direction registers
//
// 0 = Output bit
// 1 = Input bit
//
pub const Z8536_PA_DD_REG: c_uint = 0x23;
pub const Z8536_PB_DD_REG: c_uint = 0x2b;
pub const Z8536_PC_DD_REG: c_uint = 0x06;
//
// Port A/B/C Special I/O Control registers
//
// 0 = Normal Input or Output
// 1 = Output with open drain or Input with 1's catcher
//
pub const Z8536_PA_SIO_REG: c_uint = 0x24;
pub const Z8536_PB_SIO_REG: c_uint = 0x2c;
pub const Z8536_PC_SIO_REG: c_uint = 0x07;
//
// Port A/B Pattern Polarity/Transition/Mask registers
//
// PM PT PP  Pattern Specification
// -- -- --  -------------------------------------
// 0  0  x  Bit masked off
// 0  1  x  Any transition
// 1  0  0  Zero (low-level)
// 1  0  1  One (high-level)
// 1  1  0  One-to-zero transition (falling-edge)
// 1  1  1  Zero-to-one transition (rising-edge)
//
pub const Z8536_PA_PP_REG: c_uint = 0x25;
pub const Z8536_PB_PP_REG: c_uint = 0x2d;
pub const Z8536_PA_PT_REG: c_uint = 0x26;
pub const Z8536_PB_PT_REG: c_uint = 0x2e;
pub const Z8536_PA_PM_REG: c_uint = 0x27;
pub const Z8536_PB_PM_REG: c_uint = 0x2f;
