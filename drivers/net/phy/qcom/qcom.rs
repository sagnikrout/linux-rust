//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/phy/qcom/qcom.h
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
pub const AT803X_SPECIFIC_FUNCTION_CONTROL: c_uint = 0x10;

pub const AT803X_SFC_AUTOMATIC_CROSSOVER: c_uint = 0x3;
pub const AT803X_SFC_MANUAL_MDIX: c_uint = 0x1;
pub const AT803X_SFC_MANUAL_MDI: c_uint = 0x0;

pub const AT803X_SPECIFIC_STATUS: c_uint = 0x11;

pub const AT803X_SS_SPEED_1000: c_int = 2;
pub const AT803X_SS_SPEED_100: c_int = 1;
pub const AT803X_SS_SPEED_10: c_int = 0;

pub const QCA808X_SS_SPEED_2500: c_int = 4;
pub const AT803X_INTR_ENABLE: c_uint = 0x12;

pub const AT803X_INTR_STATUS: c_uint = 0x13;
pub const AT803X_SMART_SPEED: c_uint = 0x14;

pub const AT803X_CDT: c_uint = 0x16;

pub const AT803X_CDT_STATUS: c_uint = 0x1c;
pub const AT803X_CDT_STATUS_STAT_NORMAL: c_int = 0;
pub const AT803X_CDT_STATUS_STAT_SHORT: c_int = 1;
pub const AT803X_CDT_STATUS_STAT_OPEN: c_int = 2;
pub const AT803X_CDT_STATUS_STAT_FAIL: c_int = 3;

pub const QCA808X_MMD3_CDT_STATUS: c_uint = 0x8064;
pub const QCA808X_MMD3_CDT_DIAG_PAIR_A: c_uint = 0x8065;
pub const QCA808X_MMD3_CDT_DIAG_PAIR_B: c_uint = 0x8066;
pub const QCA808X_MMD3_CDT_DIAG_PAIR_C: c_uint = 0x8067;
pub const QCA808X_MMD3_CDT_DIAG_PAIR_D: c_uint = 0x8068;

// NORMAL are MDI with type set to 0

// Added for reference of existence but should be handled by wait_for_completion already

pub const QCA808X_MMD7_LED_GLOBAL: c_uint = 0x8073;

// Values are the same for both BLINK_1 and BLINK_2

// LED hw control pattern is the same for every LED

// Follow blink trigger even if duplex or speed condition doesn't match

// LED force ctrl is the same for every LED
// No documentation exist for this, not even internal one
// with NDA as QCOM gives only info about configuring
// hw control pattern rules and doesn't indicate any way
// to force the LED to specific mode.
// These define comes from reverse and testing and maybe
// lack of some info or some info are not entirely correct.
// For the basic LED control and hw control these finding
// are enough to support LED control in all the required APIs.
//
// On doing some comparison with implementation with qca807x,
// it was found that it's 1:1 equal to it and confirms all the
// reverse done. It was also found further specification with the
// force mode and the blink modes.
//

pub const AT803X_LOC_MAC_ADDR_0_15_OFFSET: c_uint = 0x804C;
pub const AT803X_LOC_MAC_ADDR_16_31_OFFSET: c_uint = 0x804B;
pub const AT803X_LOC_MAC_ADDR_32_47_OFFSET: c_uint = 0x804A;
pub const AT803X_PHY_MMD3_WOL_CTRL: c_uint = 0x8012;

pub const AT803X_DEBUG_ADDR: c_uint = 0x1D;
pub const AT803X_DEBUG_DATA: c_uint = 0x1E;
pub const AT803X_DEBUG_ANALOG_TEST_CTRL: c_uint = 0x00;

pub const AT803X_DEBUG_SYSTEM_CTRL_MODE: c_uint = 0x05;

pub const AT803X_DEBUG_REG_HIB_CTRL: c_uint = 0x0b;

pub const AT803X_DEFAULT_DOWNSHIFT: c_int = 5;
pub const AT803X_MIN_DOWNSHIFT: c_int = 2;
pub const AT803X_MAX_DOWNSHIFT: c_int = 9;
pub const QCA808X_MMD7_CNT_CTRL: c_uint = 0x8029;

pub const QCA808X_MMD7_CNT_RX_PKT_31_16: c_uint = 0x802a;
pub const QCA808X_MMD7_CNT_RX_PKT_15_0: c_uint = 0x802b;
pub const QCA808X_MMD7_CNT_RX_ERR_PKT: c_uint = 0x802c;
pub const QCA808X_MMD7_CNT_TX_PKT_31_16: c_uint = 0x802d;
pub const QCA808X_MMD7_CNT_TX_PKT_15_0: c_uint = 0x802e;
pub const QCA808X_MMD7_CNT_TX_ERR_PKT: c_uint = 0x802f;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stat_access_type {
    PHY,
    MMD
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct at803x_hw_stat {
    pub string: *const c_char,
    pub reg: u8,
    pub mask: u32,
    pub access_type: stat_access_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct at803x_ss_mask {
    pub speed_mask: u16,
    pub speed_shift: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qcom_phy_hw_stats {
    pub rx_pkts: u64,
    pub rx_err_pkts: u64,
    pub tx_pkts: u64,
    pub tx_err_pkts: u64,
}

extern "C" {
    pub fn at803x_debug_reg_read(phydev: *mut phy_device, reg: u16) -> c_int;
}
extern "C" {
    pub fn at803x_debug_reg_write(phydev: *mut phy_device, reg: u16, data: u16) -> c_int;
}
extern "C" {
    pub fn at803x_ack_interrupt(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn at803x_config_intr(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn at803x_handle_interrupt(phydev: *mut phy_device) -> irqreturn_t;
}
extern "C" {
    pub fn at803x_config_mdix(phydev: *mut phy_device, ctrl: u8) -> c_int;
}
extern "C" {
    pub fn at803x_prepare_config_aneg(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn at803x_read_status(phydev: *mut phy_device) -> c_int;
}
extern "C" {
    pub fn at803x_cdt_fault_length(dt: c_int) -> c_int;
}
extern "C" {
    pub fn at803x_cdt_start(phydev: *mut phy_device, cdt_start: u32) -> c_int;
}
extern "C" {
    pub fn qca808x_cable_test_get_status(phydev: *mut phy_device, finished: *mut bool) -> c_int;
}
extern "C" {
    pub fn qca808x_led_reg_hw_control_enable(phydev: *mut phy_device, reg: u16) -> c_int;
}
extern "C" {
    pub fn qca808x_led_reg_hw_control_status(phydev: *mut phy_device, reg: u16) -> bool;
}
extern "C" {
    pub fn qcom_phy_counter_config(phydev: *mut phy_device) -> c_int;
}
