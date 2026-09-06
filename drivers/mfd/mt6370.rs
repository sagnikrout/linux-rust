//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mfd/mt6370.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2022 Richtek Technology Corp.
//
// Author: ChiYuan Huang <cy_huang@richtek.com>
//
// IRQ definitions
pub const MT6370_IRQ_DIRCHGON: c_int = 0;
pub const MT6370_IRQ_CHG_TREG: c_int = 4;
pub const MT6370_IRQ_CHG_AICR: c_int = 5;
pub const MT6370_IRQ_CHG_MIVR: c_int = 6;
pub const MT6370_IRQ_PWR_RDY: c_int = 7;
pub const MT6370_IRQ_FL_CHG_VINOVP: c_int = 11;
pub const MT6370_IRQ_CHG_VSYSUV: c_int = 12;
pub const MT6370_IRQ_CHG_VSYSOV: c_int = 13;
pub const MT6370_IRQ_CHG_VBATOV: c_int = 14;
pub const MT6370_IRQ_CHG_VINOVPCHG: c_int = 15;
pub const MT6370_IRQ_TS_BAT_COLD: c_int = 20;
pub const MT6370_IRQ_TS_BAT_COOL: c_int = 21;
pub const MT6370_IRQ_TS_BAT_WARM: c_int = 22;
pub const MT6370_IRQ_TS_BAT_HOT: c_int = 23;
pub const MT6370_IRQ_TS_STATC: c_int = 24;
pub const MT6370_IRQ_CHG_FAULT: c_int = 25;
pub const MT6370_IRQ_CHG_STATC: c_int = 26;
pub const MT6370_IRQ_CHG_TMR: c_int = 27;
pub const MT6370_IRQ_CHG_BATABS: c_int = 28;
pub const MT6370_IRQ_CHG_ADPBAD: c_int = 29;
pub const MT6370_IRQ_CHG_RVP: c_int = 30;
pub const MT6370_IRQ_TSHUTDOWN: c_int = 31;
pub const MT6370_IRQ_CHG_IINMEAS: c_int = 32;
pub const MT6370_IRQ_CHG_ICCMEAS: c_int = 33;
pub const MT6370_IRQ_CHGDET_DONE: c_int = 34;
pub const MT6370_IRQ_WDTMR: c_int = 35;
pub const MT6370_IRQ_SSFINISH: c_int = 36;
pub const MT6370_IRQ_CHG_RECHG: c_int = 37;
pub const MT6370_IRQ_CHG_TERM: c_int = 38;
pub const MT6370_IRQ_CHG_IEOC: c_int = 39;
pub const MT6370_IRQ_ADC_DONE: c_int = 40;
pub const MT6370_IRQ_PUMPX_DONE: c_int = 41;
pub const MT6370_IRQ_BST_BATUV: c_int = 45;
pub const MT6370_IRQ_BST_MIDOV: c_int = 46;
pub const MT6370_IRQ_BST_OLP: c_int = 47;
pub const MT6370_IRQ_ATTACH: c_int = 48;
pub const MT6370_IRQ_DETACH: c_int = 49;
pub const MT6370_IRQ_HVDCP_STPDONE: c_int = 51;
pub const MT6370_IRQ_HVDCP_VBUSDET_DONE: c_int = 52;
pub const MT6370_IRQ_HVDCP_DET: c_int = 53;
pub const MT6370_IRQ_CHGDET: c_int = 54;
pub const MT6370_IRQ_DCDT: c_int = 55;
pub const MT6370_IRQ_DIRCHG_VGOK: c_int = 59;
pub const MT6370_IRQ_DIRCHG_WDTMR: c_int = 60;
pub const MT6370_IRQ_DIRCHG_UC: c_int = 61;
pub const MT6370_IRQ_DIRCHG_OC: c_int = 62;
pub const MT6370_IRQ_DIRCHG_OV: c_int = 63;
pub const MT6370_IRQ_OVPCTRL_SWON: c_int = 67;
pub const MT6370_IRQ_OVPCTRL_UVP_D: c_int = 68;
pub const MT6370_IRQ_OVPCTRL_UVP: c_int = 69;
pub const MT6370_IRQ_OVPCTRL_OVP_D: c_int = 70;
pub const MT6370_IRQ_OVPCTRL_OVP: c_int = 71;
pub const MT6370_IRQ_FLED_STRBPIN: c_int = 72;
pub const MT6370_IRQ_FLED_TORPIN: c_int = 73;
pub const MT6370_IRQ_FLED_TX: c_int = 74;
pub const MT6370_IRQ_FLED_LVF: c_int = 75;
pub const MT6370_IRQ_FLED2_SHORT: c_int = 78;
pub const MT6370_IRQ_FLED1_SHORT: c_int = 79;
pub const MT6370_IRQ_FLED2_STRB: c_int = 80;
pub const MT6370_IRQ_FLED1_STRB: c_int = 81;
pub const MT6370_IRQ_FLED2_STRB_TO: c_int = 82;
pub const MT6370_IRQ_FLED1_STRB_TO: c_int = 83;
pub const MT6370_IRQ_FLED2_TOR: c_int = 84;
pub const MT6370_IRQ_FLED1_TOR: c_int = 85;
pub const MT6370_IRQ_OTP: c_int = 93;
pub const MT6370_IRQ_VDDA_OVP: c_int = 94;
pub const MT6370_IRQ_VDDA_UV: c_int = 95;
pub const MT6370_IRQ_LDO_OC: c_int = 103;
pub const MT6370_IRQ_BLED_OCP: c_int = 118;
pub const MT6370_IRQ_BLED_OVP: c_int = 119;
pub const MT6370_IRQ_DSV_VNEG_OCP: c_int = 123;
pub const MT6370_IRQ_DSV_VPOS_OCP: c_int = 124;
pub const MT6370_IRQ_DSV_BST_OCP: c_int = 125;
pub const MT6370_IRQ_DSV_VNEG_SCP: c_int = 126;
pub const MT6370_IRQ_DSV_VPOS_SCP: c_int = 127;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt6370_info {
    pub i2c: [*mut i2c_client; MT6370_MAX_I2C],
    pub irq_data: *mut regmap_irq_chip_data,
}
