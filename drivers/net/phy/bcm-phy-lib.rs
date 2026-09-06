//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/phy/bcm-phy-lib.h
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
// Copyright (C) 2015 Broadcom Corporation
//

// 28nm only register definitions

extern "C" {
    pub fn __bcm_phy_write_exp(phydev: *mut phy_device, reg: u16, val: u16) -> c_int;
}
extern "C" {
    pub fn __bcm_phy_read_exp(phydev: *mut phy_device, reg: u16) -> c_int;
}
extern "C" {
    pub fn __bcm_phy_modify_exp(phydev: *mut phy_device, reg: u16, mask: u16, set: u16) -> c_int;
}
extern "C" {
    pub fn bcm_phy_write_exp(phydev: *mut phy_device, reg: u16, val: u16) -> c_int;
}
extern "C" {
    pub fn bcm_phy_read_exp(phydev: *mut phy_device, reg: u16) -> c_int;
}
extern "C" {
    pub fn bcm_phy_modify_exp(phydev: *mut phy_device, reg: u16, mask: u16, set: u16) -> c_int;
}
extern "C" {
    pub fn bcm_phy_write_exp(_arg: phydev, MII_BCM54XX_EXP_SEL_ER: reg |, _arg: val) -> return;
}
extern "C" {
    pub fn bcm_phy_read_exp(_arg: phydev, MII_BCM54XX_EXP_SEL_ER: reg |) -> return;
}
extern "C" {
    pub fn bcm54xx_auxctl_write(phydev: *mut phy_device, regnum: u16, val: u16) -> c_int;
}
extern "C" {
    pub fn bcm54xx_auxctl_read(phydev: *mut phy_device, regnum: u16) -> c_int;
}
extern "C" {
    pub fn bcm_phy_read_shadow(phydev: *mut phy_device, shadow: u16) -> c_int;
}
extern "C" {
    pub fn __bcm_phy_write_rdb(phydev: *mut phy_device, rdb: u16, val: u16) -> c_int;
}
extern "C" {
    pub fn bcm_phy_write_rdb(phydev: *mut phy_device, rdb: u16, val: u16) -> c_int;
}
extern "C" {
    pub fn __bcm_phy_read_rdb(phydev: *mut phy_device, rdb: u16) -> c_int;
}
extern "C" {
    pub fn bcm_phy_read_rdb(phydev: *mut phy_device, rdb: u16) -> c_int;
}
extern "C" {
    pub fn bcm_phy_ack_intr(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn bcm_phy_config_intr(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn bcm_phy_handle_interrupt(phydev: *mut phy_device) -> irqreturn_t;
}
extern "C" {
    pub fn bcm_phy_enable_apd(phydev: *mut phy_device, dll_pwr_down: bool) -> c_int;
}
extern "C" {
    pub fn bcm_phy_set_eee(phydev: *mut phy_device, enable: bool) -> c_int;
}
extern "C" {
    pub fn bcm_phy_downshift_get(phydev: *mut phy_device, count: *mut u8) -> c_int;
}
extern "C" {
    pub fn bcm_phy_downshift_set(phydev: *mut phy_device, count: u8) -> c_int;
}
extern "C" {
    pub fn bcm_phy_get_sset_count(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn bcm_phy_get_strings(phydev: *mut phy_device, data: *mut u8);
}
extern "C" {
    pub fn bcm_phy_update_stats_shadow(phydev: *mut phy_device, shadow: *mut u64);
}
extern "C" {
    pub fn bcm_phy_r_rc_cal_reset(phydev: *mut phy_device);
}
extern "C" {
    pub fn bcm_phy_28nm_a0b0_afe_config_init(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn bcm_phy_enable_jumbo(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn bcm_phy_cable_test_start_rdb(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn bcm_phy_cable_test_start(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn bcm_phy_cable_test_get_status(phydev: *mut phy_device, finished: *mut bool) -> c_int;
}

extern "C" {
    pub fn bcm_ptp_config_init(phydev: *mut phy_device);
}
extern "C" {
    pub fn bcm_ptp_stop(priv: *mut bcm_ptp_private);
}

extern "C" {
    pub fn bcm_phy_set_wol(phydev: *mut phy_device, wol: *mut ethtool_wolinfo) -> c_int;
}
extern "C" {
    pub fn bcm_phy_get_wol(phydev: *mut phy_device, wol: *mut ethtool_wolinfo);
}
extern "C" {
    pub fn bcm_phy_wol_isr(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn bcm_setup_lre_master_slave(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn bcm_config_lre_aneg(phydev: *mut phy_device, changed: bool) -> c_int;
}
extern "C" {
    pub fn bcm_config_lre_advert(phydev: *mut phy_device) -> c_int;
}
