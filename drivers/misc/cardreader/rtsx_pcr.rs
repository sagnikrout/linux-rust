//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/misc/cardreader/rtsx_pcr.h
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
// Driver for Realtek PCI-Express card reader
//
// Copyright(c) 2009-2013 Realtek Semiconductor Corp. All rights reserved.
//
// Author:
// Wei WANG <wei_wang@realsil.com.cn>
//

pub const MIN_DIV_N_PCR: c_int = 80;
pub const MAX_DIV_N_PCR: c_int = 208;
pub const RTS522A_PME_FORCE_CTL: c_uint = 0xFF78;
pub const RTS522A_AUTOLOAD_CFG1: c_uint = 0xFF7C;
pub const RTS522A_PM_CTRL3: c_uint = 0xFF7E;
pub const RTS524A_PME_FORCE_CTL: c_uint = 0xFF78;
pub const REG_EFUSE_BYPASS: c_uint = 0x08;
pub const REG_EFUSE_POR: c_uint = 0x04;
pub const REG_EFUSE_POWER_MASK: c_uint = 0x03;
pub const REG_EFUSE_POWERON: c_uint = 0x03;
pub const REG_EFUSE_POWEROFF: c_uint = 0x00;
pub const RTS5250_CLK_CFG3: c_uint = 0xFF79;
pub const RTS525A_CFG_MEM_PD: c_uint = 0xF0;
pub const RTS524A_AUTOLOAD_CFG1: c_uint = 0xFF7C;
pub const RTS524A_PM_CTRL3: c_uint = 0xFF7E;
pub const RTS525A_BIOS_CFG: c_uint = 0xFF2D;
pub const RTS525A_LOAD_BIOS_FLAG: c_uint = 0x01;
pub const RTS525A_CLEAR_BIOS_FLAG: c_uint = 0x00;
pub const RTS525A_EFUSE_CTL: c_uint = 0xFC32;
pub const REG_EFUSE_ENABLE: c_uint = 0x80;
pub const REG_EFUSE_MODE: c_uint = 0x40;
pub const RTS525A_EFUSE_ADD: c_uint = 0xFC33;
pub const REG_EFUSE_ADD_MASK: c_uint = 0x3F;
pub const RTS525A_EFUSE_DATA: c_uint = 0xFC35;
pub const LTR_ACTIVE_LATENCY_DEF: c_uint = 0x883C;
pub const LTR_IDLE_LATENCY_DEF: c_uint = 0x892C;
pub const LTR_L1OFF_LATENCY_DEF: c_uint = 0x9003;
pub const L1_SNOOZE_DELAY_DEF: c_int = 1;
pub const LTR_L1OFF_SSPWRGATE_5249_DEF: c_uint = 0xAF;
pub const LTR_L1OFF_SSPWRGATE_5250_DEF: c_uint = 0xFF;
pub const LTR_L1OFF_SNOOZE_SSPWRGATE_5249_DEF: c_uint = 0xAC;
pub const LTR_L1OFF_SNOOZE_SSPWRGATE_5250_DEF: c_uint = 0xF8;
pub const CMD_TIMEOUT_DEF: c_int = 100;
pub const MASK_8_BIT_DEF: c_uint = 0xFF;
pub const SSC_CLOCK_STABLE_WAIT: c_int = 130;
pub const RTS524A_OCP_THD_800: c_uint = 0x04;
pub const RTS525A_OCP_THD_800: c_uint = 0x05;
pub const RTS522A_OCP_THD_800: c_uint = 0x06;
extern "C" {
    pub fn __rtsx_pci_write_phy_register(pcr: *mut rtsx_pcr, addr: u8, val: u16) -> c_int;
}
extern "C" {
    pub fn __rtsx_pci_read_phy_register(pcr: *mut rtsx_pcr, addr: u8, val: *mut u16) -> c_int;
}
extern "C" {
    pub fn rts5209_init_params(pcr: *mut rtsx_pcr);
}
extern "C" {
    pub fn rts5229_init_params(pcr: *mut rtsx_pcr);
}
extern "C" {
    pub fn rtl8411_init_params(pcr: *mut rtsx_pcr);
}
extern "C" {
    pub fn rtl8402_init_params(pcr: *mut rtsx_pcr);
}
extern "C" {
    pub fn rts5227_init_params(pcr: *mut rtsx_pcr);
}
extern "C" {
    pub fn rts522a_init_params(pcr: *mut rtsx_pcr);
}
extern "C" {
    pub fn rts5249_init_params(pcr: *mut rtsx_pcr);
}
extern "C" {
    pub fn rts524a_init_params(pcr: *mut rtsx_pcr);
}
extern "C" {
    pub fn rts525a_init_params(pcr: *mut rtsx_pcr);
}
extern "C" {
    pub fn rtl8411b_init_params(pcr: *mut rtsx_pcr);
}
extern "C" {
    pub fn rts5260_init_params(pcr: *mut rtsx_pcr);
}
extern "C" {
    pub fn rts5261_init_params(pcr: *mut rtsx_pcr);
}
extern "C" {
    pub fn rts5228_init_params(pcr: *mut rtsx_pcr);
}
extern "C" {
    pub fn rts5264_init_params(pcr: *mut rtsx_pcr);
}

// generic operations
extern "C" {
    pub fn rtsx_gops_pm_reset(pcr: *mut rtsx_pcr) -> c_int;
}
extern "C" {
    pub fn rtsx_set_ltr_latency(pcr: *mut rtsx_pcr, latency: u32) -> c_int;
}
extern "C" {
    pub fn rtsx_set_l1off_sub(pcr: *mut rtsx_pcr, val: u8) -> c_int;
}
extern "C" {
    pub fn rtsx_pci_init_ocp(pcr: *mut rtsx_pcr);
}
extern "C" {
    pub fn rtsx_pci_disable_ocp(pcr: *mut rtsx_pcr);
}
extern "C" {
    pub fn rtsx_pci_enable_ocp(pcr: *mut rtsx_pcr);
}
extern "C" {
    pub fn rtsx_pci_get_ocpstat(pcr: *mut rtsx_pcr, val: *mut u8) -> c_int;
}
extern "C" {
    pub fn rtsx_pci_clear_ocpstat(pcr: *mut rtsx_pcr);
}
extern "C" {
    pub fn rtsx_pci_enable_oobs_polling(pcr: *mut rtsx_pcr);
}
extern "C" {
    pub fn rtsx_pci_disable_oobs_polling(pcr: *mut rtsx_pcr);
}
