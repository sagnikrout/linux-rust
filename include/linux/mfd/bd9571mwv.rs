//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/bd9571mwv.h
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
// ROHM BD9571MWV-M and BD9574MWF-M driver
//
// Copyright (C) 2017 Marek Vasut <marek.vasut+renesas@gmail.com>
// Copyright (C) 2020 Renesas Electronics Corporation
//
// Based on the TPS65086 driver
//

// List of registers for BD9571MWV and BD9574MWF
pub const BD9571MWV_VENDOR_CODE: c_uint = 0x00;
pub const BD9571MWV_VENDOR_CODE_VAL: c_uint = 0xdb;
pub const BD9571MWV_PRODUCT_CODE: c_uint = 0x01;
pub const BD9571MWV_PRODUCT_CODE_BD9571MWV: c_uint = 0x60;
pub const BD9571MWV_PRODUCT_CODE_BD9574MWF: c_uint = 0x74;
pub const BD9571MWV_PRODUCT_REVISION: c_uint = 0x02;
pub const BD9571MWV_I2C_FUSA_MODE: c_uint = 0x10;
pub const BD9571MWV_I2C_MD2_E1_BIT_1: c_uint = 0x11;
pub const BD9571MWV_I2C_MD2_E1_BIT_2: c_uint = 0x12;
pub const BD9571MWV_BKUP_MODE_CNT: c_uint = 0x20;

pub const BD9571MWV_BKUP_MODE_STATUS: c_uint = 0x21;
pub const BD9571MWV_BKUP_RECOVERY_CNT: c_uint = 0x22;
pub const BD9571MWV_BKUP_CTRL_TIM_CNT: c_uint = 0x23;
pub const BD9571MWV_WAITBKUP_WDT_CNT: c_uint = 0x24;
pub const BD9571MWV_128H_TIM_CNT: c_uint = 0x26;
pub const BD9571MWV_QLLM_CNT: c_uint = 0x27;
pub const BD9571MWV_AVS_SET_MONI: c_uint = 0x31;
pub const BD9571MWV_AVS_SET_MONI_MASK: c_uint = 0x3;

pub const BD9571MWV_VD18_VID: c_uint = 0x42;
pub const BD9571MWV_VD25_VID: c_uint = 0x43;
pub const BD9571MWV_VD33_VID: c_uint = 0x44;
pub const BD9571MWV_DVFS_VINIT: c_uint = 0x50;
pub const BD9574MWF_VD09_VINIT: c_uint = 0x51;
pub const BD9571MWV_DVFS_SETVMAX: c_uint = 0x52;
pub const BD9571MWV_DVFS_BOOSTVID: c_uint = 0x53;
pub const BD9571MWV_DVFS_SETVID: c_uint = 0x54;
pub const BD9571MWV_DVFS_MONIVDAC: c_uint = 0x55;
pub const BD9571MWV_DVFS_PGD_CNT: c_uint = 0x56;
pub const BD9571MWV_GPIO_DIR: c_uint = 0x60;
pub const BD9571MWV_GPIO_OUT: c_uint = 0x61;
pub const BD9571MWV_GPIO_IN: c_uint = 0x62;
pub const BD9571MWV_GPIO_DEB: c_uint = 0x63;
pub const BD9571MWV_GPIO_INT_SET: c_uint = 0x64;
pub const BD9571MWV_GPIO_INT: c_uint = 0x65;
pub const BD9571MWV_GPIO_INTMASK: c_uint = 0x66;
pub const BD9574MWF_GPIO_MUX: c_uint = 0x67;

pub const BD9571MWV_PMIC_INTERNAL_STATUS: c_uint = 0x80;
pub const BD9571MWV_PROT_ERROR_STATUS0: c_uint = 0x81;
pub const BD9571MWV_PROT_ERROR_STATUS1: c_uint = 0x82;
pub const BD9571MWV_PROT_ERROR_STATUS2: c_uint = 0x83;
pub const BD9571MWV_PROT_ERROR_STATUS3: c_uint = 0x84;
pub const BD9571MWV_PROT_ERROR_STATUS4: c_uint = 0x85;
pub const BD9574MWF_PROT_ERROR_STATUS5: c_uint = 0x86;
pub const BD9574MWF_SYSTEM_ERROR_STATUS: c_uint = 0x87;
pub const BD9571MWV_INT_INTREQ: c_uint = 0x90;

pub const BD9571MWV_INT_INTMASK: c_uint = 0x91;
pub const BD9574MWF_SSCG_CNT: c_uint = 0xA0;
pub const BD9574MWF_POFFB_MRB: c_uint = 0xA1;
pub const BD9574MWF_SMRB_WR_PROT: c_uint = 0xA2;
pub const BD9574MWF_SMRB_ASSERT: c_uint = 0xA3;
pub const BD9574MWF_SMRB_STATUS: c_uint = 0xA4;
pub const BD9571MWV_ACCESS_KEY: c_uint = 0xff;
// Define the BD9571MWV IRQ numbers
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bd9571mwv_irqs {
    BD9571MWV_IRQ_MD1,
    BD9571MWV_IRQ_MD2_E1,
    BD9571MWV_IRQ_MD2_E2,
    BD9571MWV_IRQ_PROT_ERR,
    BD9571MWV_IRQ_GP,
    BD9571MWV_IRQ_128H_OF,	/* BKUP_HOLD on BD9574MWF */
    BD9571MWV_IRQ_WDT_OF,
    BD9571MWV_IRQ_BKUP_TRG,
}
