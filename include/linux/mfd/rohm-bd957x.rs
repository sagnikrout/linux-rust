//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/rohm-bd957x.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (C) 2021 ROHM Semiconductors
//
// The BD9576 has own IRQ 'blocks' for:
// - I2C/thermal,
// - Over voltage protection
// - Short-circuit protection
// - Over current protection
// - Over voltage detection
// - Under voltage detection
// - Under voltage protection
// - 'system interrupt'.
//
// Each of the blocks have a status register giving more accurate IRQ source
// information - for example which of the regulators have over-voltage.
//
// On top of this, there is "main IRQ" status register where each bit indicates
// which of sub-blocks have active IRQs. Fine. That would fit regmap-irq main
// status handling. Except that:
// - Only some sub-IRQs can be masked.
// - The IRQ informs us about fault-condition, not when fault state changes.
// The IRQ line it is kept asserted until the detected condition is acked
// AND cleared in HW. This is annoying for IRQs like the one informing high
// temperature because if IRQ is not disabled it keeps the CPU in IRQ
// handling loop.
//
// For now we do just use the main-IRQ register as source for our IRQ
// information and bind the regmap-irq to this. We leave fine-grained sub-IRQ
// register handling to handlers in sub-devices. The regulator driver shall
// read which regulators are source for problem - or if the detected error is
// regulator temperature error. The sub-drivers do also handle masking of "sub-
// IRQs" if this is supported/needed.
//
// To overcome the problem with HW keeping IRQ asserted we do call
// disable_irq_nosync() from sub-device handler and add a delayed work to
// re-enable IRQ roughly 1 second later. This should keep our CPU out of
// busy-loop.
//
pub const IRQS_SILENT_MS: c_int = 1000;
pub const BD957X_REG_SMRB_ASSERT: c_uint = 0x15;
pub const BD957X_REG_PMIC_INTERNAL_STAT: c_uint = 0x20;
pub const BD957X_REG_INT_THERM_STAT: c_uint = 0x23;
pub const BD957X_REG_INT_THERM_MASK: c_uint = 0x24;
pub const BD957X_REG_INT_OVP_STAT: c_uint = 0x25;
pub const BD957X_REG_INT_SCP_STAT: c_uint = 0x26;
pub const BD957X_REG_INT_OCP_STAT: c_uint = 0x27;
pub const BD957X_REG_INT_OVD_STAT: c_uint = 0x28;
pub const BD957X_REG_INT_UVD_STAT: c_uint = 0x29;
pub const BD957X_REG_INT_UVP_STAT: c_uint = 0x2a;
pub const BD957X_REG_INT_SYS_STAT: c_uint = 0x2b;
pub const BD957X_REG_INT_SYS_MASK: c_uint = 0x2c;
pub const BD957X_REG_INT_MAIN_STAT: c_uint = 0x30;
pub const BD957X_REG_INT_MAIN_MASK: c_uint = 0x31;
pub const UVD_IRQ_VALID_MASK: c_uint = 0x6F;
pub const OVD_IRQ_VALID_MASK: c_uint = 0x2F;

pub const BD957X_MASK_INT_ALL: c_uint = 0xff;
pub const BD957X_REG_WDT_CONF: c_uint = 0x16;
pub const BD957X_REG_POW_TRIGGER1: c_uint = 0x41;
pub const BD957X_REG_POW_TRIGGER2: c_uint = 0x42;
pub const BD957X_REG_POW_TRIGGER3: c_uint = 0x43;
pub const BD957X_REG_POW_TRIGGER4: c_uint = 0x44;
pub const BD957X_REG_POW_TRIGGERL1: c_uint = 0x45;
pub const BD957X_REG_POW_TRIGGERS1: c_uint = 0x46;
pub const BD957X_REGULATOR_EN_MASK: c_uint = 0xff;
pub const BD957X_REGULATOR_DIS_VAL: c_uint = 0xff;
pub const BD957X_VSEL_REG_MASK: c_uint = 0xff;
pub const BD957X_MASK_VOUT1_TUNE: c_uint = 0x87;
pub const BD957X_MASK_VOUT2_TUNE: c_uint = 0x87;
pub const BD957X_MASK_VOUT3_TUNE: c_uint = 0x1f;
pub const BD957X_MASK_VOUT4_TUNE: c_uint = 0x1f;
pub const BD957X_MASK_VOUTL1_TUNE: c_uint = 0x87;
pub const BD957X_REG_VOUT1_TUNE: c_uint = 0x50;
pub const BD957X_REG_VOUT2_TUNE: c_uint = 0x53;
pub const BD957X_REG_VOUT3_TUNE: c_uint = 0x56;
pub const BD957X_REG_VOUT4_TUNE: c_uint = 0x59;
pub const BD957X_REG_VOUTL1_TUNE: c_uint = 0x5c;
pub const BD9576_REG_VOUT1_OVD: c_uint = 0x51;
pub const BD9576_REG_VOUT1_UVD: c_uint = 0x52;
pub const BD9576_REG_VOUT2_OVD: c_uint = 0x54;
pub const BD9576_REG_VOUT2_UVD: c_uint = 0x55;
pub const BD9576_REG_VOUT3_OVD: c_uint = 0x57;
pub const BD9576_REG_VOUT3_UVD: c_uint = 0x58;
pub const BD9576_REG_VOUT4_OVD: c_uint = 0x5a;
pub const BD9576_REG_VOUT4_UVD: c_uint = 0x5b;
pub const BD9576_REG_VOUTL1_OVD: c_uint = 0x5d;
pub const BD9576_REG_VOUTL1_UVD: c_uint = 0x5e;
pub const BD9576_MASK_XVD: c_uint = 0x7f;
pub const BD9576_REG_VOUT1S_OCW: c_uint = 0x5f;
pub const BD9576_REG_VOUT1S_OCP: c_uint = 0x60;
pub const BD9576_MASK_VOUT1S_OCW: c_uint = 0x3f;
pub const BD9576_MASK_VOUT1S_OCP: c_uint = 0x3f;
pub const BD957X_MAX_REGISTER: c_uint = 0x61;
