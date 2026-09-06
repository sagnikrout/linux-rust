//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/mediatek/mt76/mt76x2/mcu.h
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


// SPDX-License-Identifier: BSD-3-Clause-Clear
//
// Copyright (C) 2016 Felix Fietkau <nbd@nbd.name>
//

// Macro flag: #define __MT76x2_MCU_H

// Register definitions
pub const MT_MCU_CPU_CTL: c_uint = 0x0704;
pub const MT_MCU_CLOCK_CTL: c_uint = 0x0708;
pub const MT_MCU_PCIE_REMAP_BASE1: c_uint = 0x0740;
pub const MT_MCU_PCIE_REMAP_BASE2: c_uint = 0x0744;
pub const MT_MCU_PCIE_REMAP_BASE3: c_uint = 0x0748;
pub const MT_MCU_ROM_PATCH_OFFSET: c_uint = 0x80000;
pub const MT_MCU_ROM_PATCH_ADDR: c_uint = 0x90000;
pub const MT_MCU_ILM_OFFSET: c_uint = 0x80000;
pub const MT_MCU_DLM_OFFSET: c_uint = 0x100000;
pub const MT_MCU_DLM_ADDR: c_uint = 0x90000;
pub const MT_MCU_DLM_ADDR_E3: c_uint = 0x90800;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mcu_calibration {
    MCU_CAL_R = 1,
    MCU_CAL_TEMP_SENSOR,
    MCU_CAL_RXDCOC,
    MCU_CAL_RC,
    MCU_CAL_SX_LOGEN,
    MCU_CAL_LC,
    MCU_CAL_TX_LOFT,
    MCU_CAL_TXIQ,
    MCU_CAL_TSSI,
    MCU_CAL_TSSI_COMP,
    MCU_CAL_DPD,
    MCU_CAL_RXIQC_FI,
    MCU_CAL_RXIQC_FD,
    MCU_CAL_PWRON,
    MCU_CAL_TX_SHAPING,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mt76x2_mcu_cr_mode {
    MT_RF_CR,
    MT_BBP_CR,
    MT_RF_BBP_CR,
    MT_HL_TEMP_CR_UPDATE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt76x2_tssi_comp {
    pub pa_mode: u8,
    pub cal_mode: u8,
    pub pad: u16,
    pub slope0: u8,
    pub slope1: u8,
    pub offset0: u8,
    pub offset1: u8,
    pub __aligned(4): } __packed,
    pub tssi_data): *mut mt76x2_tssi_comp,
    pub force): bool,
