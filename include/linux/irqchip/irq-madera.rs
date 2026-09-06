//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/irqchip/irq-madera.h
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
// Interrupt support for Cirrus Logic Madera codecs
//
// Copyright (C) 2016-2018 Cirrus Logic, Inc. and
// Cirrus Logic International Semiconductor Ltd.
//

pub const MADERA_IRQ_FLL1_LOCK: c_int = 0;
pub const MADERA_IRQ_FLL2_LOCK: c_int = 1;
pub const MADERA_IRQ_FLL3_LOCK: c_int = 2;
pub const MADERA_IRQ_FLLAO_LOCK: c_int = 3;
pub const MADERA_IRQ_CLK_SYS_ERR: c_int = 4;
pub const MADERA_IRQ_CLK_ASYNC_ERR: c_int = 5;
pub const MADERA_IRQ_CLK_DSP_ERR: c_int = 6;
pub const MADERA_IRQ_HPDET: c_int = 7;
pub const MADERA_IRQ_MICDET1: c_int = 8;
pub const MADERA_IRQ_MICDET2: c_int = 9;
pub const MADERA_IRQ_JD1_RISE: c_int = 10;
pub const MADERA_IRQ_JD1_FALL: c_int = 11;
pub const MADERA_IRQ_JD2_RISE: c_int = 12;
pub const MADERA_IRQ_JD2_FALL: c_int = 13;
pub const MADERA_IRQ_MICD_CLAMP_RISE: c_int = 14;
pub const MADERA_IRQ_MICD_CLAMP_FALL: c_int = 15;
pub const MADERA_IRQ_DRC2_SIG_DET: c_int = 16;
pub const MADERA_IRQ_DRC1_SIG_DET: c_int = 17;
pub const MADERA_IRQ_ASRC1_IN1_LOCK: c_int = 18;
pub const MADERA_IRQ_ASRC1_IN2_LOCK: c_int = 19;
pub const MADERA_IRQ_ASRC2_IN1_LOCK: c_int = 20;
pub const MADERA_IRQ_ASRC2_IN2_LOCK: c_int = 21;
pub const MADERA_IRQ_DSP_IRQ1: c_int = 22;
pub const MADERA_IRQ_DSP_IRQ2: c_int = 23;
pub const MADERA_IRQ_DSP_IRQ3: c_int = 24;
pub const MADERA_IRQ_DSP_IRQ4: c_int = 25;
pub const MADERA_IRQ_DSP_IRQ5: c_int = 26;
pub const MADERA_IRQ_DSP_IRQ6: c_int = 27;
pub const MADERA_IRQ_DSP_IRQ7: c_int = 28;
pub const MADERA_IRQ_DSP_IRQ8: c_int = 29;
pub const MADERA_IRQ_DSP_IRQ9: c_int = 30;
pub const MADERA_IRQ_DSP_IRQ10: c_int = 31;
pub const MADERA_IRQ_DSP_IRQ11: c_int = 32;
pub const MADERA_IRQ_DSP_IRQ12: c_int = 33;
pub const MADERA_IRQ_DSP_IRQ13: c_int = 34;
pub const MADERA_IRQ_DSP_IRQ14: c_int = 35;
pub const MADERA_IRQ_DSP_IRQ15: c_int = 36;
pub const MADERA_IRQ_DSP_IRQ16: c_int = 37;
pub const MADERA_IRQ_HP1L_SC: c_int = 38;
pub const MADERA_IRQ_HP1R_SC: c_int = 39;
pub const MADERA_IRQ_HP2L_SC: c_int = 40;
pub const MADERA_IRQ_HP2R_SC: c_int = 41;
pub const MADERA_IRQ_HP3L_SC: c_int = 42;
pub const MADERA_IRQ_HP3R_SC: c_int = 43;
pub const MADERA_IRQ_SPKOUTL_SC: c_int = 44;
pub const MADERA_IRQ_SPKOUTR_SC: c_int = 45;
pub const MADERA_IRQ_HP1L_ENABLE_DONE: c_int = 46;
pub const MADERA_IRQ_HP1R_ENABLE_DONE: c_int = 47;
pub const MADERA_IRQ_HP2L_ENABLE_DONE: c_int = 48;
pub const MADERA_IRQ_HP2R_ENABLE_DONE: c_int = 49;
pub const MADERA_IRQ_HP3L_ENABLE_DONE: c_int = 50;
pub const MADERA_IRQ_HP3R_ENABLE_DONE: c_int = 51;
pub const MADERA_IRQ_SPKOUTL_ENABLE_DONE: c_int = 52;
pub const MADERA_IRQ_SPKOUTR_ENABLE_DONE: c_int = 53;
pub const MADERA_IRQ_SPK_SHUTDOWN: c_int = 54;
pub const MADERA_IRQ_SPK_OVERHEAT: c_int = 55;
pub const MADERA_IRQ_SPK_OVERHEAT_WARN: c_int = 56;
pub const MADERA_IRQ_GPIO1: c_int = 57;
pub const MADERA_IRQ_GPIO2: c_int = 58;
pub const MADERA_IRQ_GPIO3: c_int = 59;
pub const MADERA_IRQ_GPIO4: c_int = 60;
pub const MADERA_IRQ_GPIO5: c_int = 61;
pub const MADERA_IRQ_GPIO6: c_int = 62;
pub const MADERA_IRQ_GPIO7: c_int = 63;
pub const MADERA_IRQ_GPIO8: c_int = 64;
pub const MADERA_IRQ_DSP1_BUS_ERR: c_int = 65;
pub const MADERA_IRQ_DSP2_BUS_ERR: c_int = 66;
pub const MADERA_IRQ_DSP3_BUS_ERR: c_int = 67;
pub const MADERA_IRQ_DSP4_BUS_ERR: c_int = 68;
pub const MADERA_IRQ_DSP5_BUS_ERR: c_int = 69;
pub const MADERA_IRQ_DSP6_BUS_ERR: c_int = 70;
pub const MADERA_IRQ_DSP7_BUS_ERR: c_int = 71;
pub const MADERA_NUM_IRQ: c_int = 72;
//
// These wrapper functions are for use by other child drivers of the
// same parent MFD.
//
extern "C" {
    pub fn regmap_irq_get_virq(_arg: madera->irq_data, _arg: irq) -> return;
}
extern "C" {
    pub fn irq_set_irq_wake(_arg: irq, _arg: on) -> return;
}
