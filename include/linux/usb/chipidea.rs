//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/chipidea.h
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
// Platform data for the chipidea USB dual role controller
//

//
// struct ci_hdrc_cable - structure for external connector cable state tracking
// @connected: true if cable is connected, false otherwise
// @changed: set to true when extcon event happen
// @enabled: set to true if we've enabled the vbus or id interrupt
// @edev: device which generate events
// @ci: driver state of the chipidea device
// @nb: hold event notification callback
// @conn: used for notification registration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ci_hdrc_cable {
    pub connected: bool,
    pub changed: bool,
    pub enabled: bool,
    pub edev: *mut extcon_dev,
    pub ci: *mut ci_hdrc,
    pub nb: notifier_block,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ci_hdrc_platform_data {
    pub name: *const c_char,
// offset of the capability registers
    pub capoffset: uintptr_t,
    pub power_budget: unsigned,
    pub phy: *mut phy,
// old usb_phy interface
    pub usb_phy: *mut usb_phy,
    pub phy_mode: usb_phy_interface,
    pub flags: c_ulong,

//
// Only set it when DCCPARAMS.DC==1 and DCCPARAMS.HC==1,
// but otg is not supported (no register otgsc).
//

    pub dr_mode: usb_dr_mode,
pub const CI_HDRC_CONTROLLER_RESET_EVENT: c_int = 0;
pub const CI_HDRC_CONTROLLER_STOPPED_EVENT: c_int = 1;
pub const CI_HDRC_IMX_HSIC_ACTIVE_EVENT: c_int = 2;
pub const CI_HDRC_IMX_HSIC_SUSPEND_EVENT: c_int = 3;
pub const CI_HDRC_CONTROLLER_VBUS_EVENT: c_int = 4;
pub const CI_HDRC_CONTROLLER_PULLUP_EVENT: c_int = 5;
    pub event): *mut *mut *mut int (notify_event) (struct ci_hdrc ci, unsigned,
    pub reg_vbus: *mut regulator,
    pub ci_otg_caps: usb_otg_caps,
    pub tpl_support: bool,
// interrupt threshold setting
    pub itc_setting: u32,
    pub ahb_burst_config: u32,
    pub tx_burst_size: u32,
    pub rx_burst_size: u32,
// VBUS and ID signal state tracking, using extcon framework
    pub vbus_extcon: ci_hdrc_cable,
    pub id_extcon: ci_hdrc_cable,
    pub phy_clkgate_delay_us: u32,
// pins
    pub pctl: *mut pinctrl,
    pub pins_default: *mut pinctrl_state,
    pub pins_host: *mut pinctrl_state,
    pub pins_device: *mut pinctrl_state,
// platform-specific hooks
    pub flags): *mut *mut bool done, unsigned long,
    pub enable): *mut *mut *mut void (enter_lpm)(struct ci_hdrc ci, bool,
}

// Default offset of capability registers
pub const DEF_CAPOFFSET: c_uint = 0x100;
// Add ci hdrc device
// Remove ci hdrc device
extern "C" {
    pub fn ci_hdrc_remove_device(pdev: *mut platform_device);
}
// Get current available role
extern "C" {
    pub fn ci_hdrc_query_available_role(pdev: *mut platform_device) -> usb_dr_mode;
}
