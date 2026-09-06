//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/cdns3/drd.h
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
// Cadence USB3 and USBSSP DRD header file.
//
// Copyright (C) 2018-2020 Cadence.
//
// Author: Pawel Laszczak <pawell@cadence.com>
//

// DRD register interface for version v1 of cdns3 driver.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns3_otg_regs {
    pub did: __le32,
    pub rid: __le32,
    pub capabilities: __le32,
    pub reserved1: __le32,
    pub cmd: __le32,
    pub sts: __le32,
    pub state: __le32,
    pub reserved2: __le32,
    pub ien: __le32,
    pub ivect: __le32,
    pub refclk: __le32,
    pub tmr: __le32,
    pub reserved3: [__le32; 4],
    pub simulate: __le32,
    pub override: __le32,
    pub susp_ctrl: __le32,
    pub phyrst_cfg: __le32,
    pub anasts: __le32,
    pub adp_ramp_time: __le32,
    pub ctrl1: __le32,
    pub ctrl2: __le32,
}

// DRD register interface for version v0 of cdns3 driver.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns3_otg_legacy_regs {
    pub cmd: __le32,
    pub sts: __le32,
    pub state: __le32,
    pub refclk: __le32,
    pub ien: __le32,
    pub ivect: __le32,
    pub reserved1: [__le32; 3],
    pub tmr: __le32,
    pub reserved2: [__le32; 2],
    pub version: __le32,
    pub capabilities: __le32,
    pub reserved3: [__le32; 2],
    pub simulate: __le32,
    pub reserved4: [__le32; 5],
    pub ctrl1: __le32,
}

// DRD register interface for cdnsp driver
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdnsp_otg_regs {
    pub did: __le32,
    pub rid: __le32,
    pub cfgs1: __le32,
    pub cfgs2: __le32,
    pub cmd: __le32,
    pub sts: __le32,
    pub state: __le32,
    pub ien: __le32,
    pub ivect: __le32,
    pub tmr: __le32,
    pub simulate: __le32,
    pub adpbc_sts: __le32,
    pub adp_ramp_time: __le32,
    pub adpbc_ctrl1: __le32,
    pub adpbc_ctrl2: __le32,
    pub override: __le32,
    pub vbusvalid_dbnc_cfg: __le32,
    pub sessvalid_dbnc_cfg: __le32,
    pub susp_timing_ctrl: __le32,
}

// CDNSP driver supports 0x000403xx Cadence USB controller family.

// CDNS3 driver supports 0x000402xx Cadence USB controller family.

//
// Common registers interface for both CDNS3 and CDNSP version of DRD.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns_otg_common_regs {
    pub cmd: __le32,
    pub sts: __le32,
    pub state: __le32,
}

//
// Interrupt related registers. This registers are mapped in different
// location for CDNSP controller.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns_otg_irq_regs {
    pub ien: __le32,
    pub ivect: __le32,
}

// CDNS_RID - bitmasks

// CDNS_VID - bitmasks

// OTGCMD - bitmasks
// "Request the bus for Device mode.

// Request the bus for Host mode

// Enable OTG mode.

// Disable OTG mode

// "Configure OTG as A-Device.

// "Configure OTG as A-Device.

// Drop the bus for Device mod	e.

// Drop the bus for Host mode

// Power Down USBSS-DEV - only for CDNS3.

// Power Down CDNSXHCI - only for CDNS3.

// OTGIEN - bitmasks
// ID change interrupt enable

// Vbusvalid fall detected interrupt enable.

// Vbusvalid fall detected interrupt enable

// OTGSTS - bitmasks
//
// Current value of the ID pin. It is only valid when idpullup in
// OTGCTRL1_TYPE register is set to '1'.
//

// Current value of the vbus_valid

// Current value of the b_sess_vld

// Device mode is active

// Host mode is active.

// OTG Controller not ready.

//
// Value of the strap pins for:
// CDNS3:
// 000 - no default configuration
// 010 - Controller initiall configured as Host
// 100 - Controller initially configured as Device
// CDNSP:
// 000 - No default configuration.
// 010 - Controller initiall configured as Host.
// 100 - Controller initially configured as Device.
//

pub const OTGSTS_STRAP_NO_DEFAULT_CFG: c_uint = 0x00;
pub const OTGSTS_STRAP_HOST_OTG: c_uint = 0x01;
pub const OTGSTS_STRAP_HOST: c_uint = 0x02;
pub const OTGSTS_STRAP_GADGET: c_uint = 0x04;
pub const OTGSTS_CDNSP_STRAP_HOST: c_uint = 0x01;
pub const OTGSTS_CDNSP_STRAP_GADGET: c_uint = 0x02;
// Host mode is turned on.

// "Device mode is turned on .

// OTGSTATE- bitmasks

pub const OTGSTATE_HOST_STATE_IDLE: c_uint = 0x0;
pub const OTGSTATE_HOST_STATE_VBUS_FALL: c_uint = 0x7;

// OTGREFCLK - bitmasks

// SUPS_CTRL - bitmasks

// OVERRIDE - bitmasks

// Only for CDNS3_CONTROLLER_V0 version

// Vbusvalid/Sesvalid override select.

// PHYRST_CFG - bitmasks

pub const CDNS3_ID_PERIPHERAL: c_int = 1;
pub const CDNS3_ID_HOST: c_int = 0;
extern "C" {
    pub fn cdns_is_host(cdns: *mut cdns) -> bool;
}
extern "C" {
    pub fn cdns_is_device(cdns: *mut cdns) -> bool;
}
extern "C" {
    pub fn cdns_get_id(cdns: *mut cdns) -> c_int;
}
extern "C" {
    pub fn cdns_get_vbus(cdns: *mut cdns) -> c_int;
}
extern "C" {
    pub fn cdns_clear_vbus(cdns: *mut cdns);
}
extern "C" {
    pub fn cdns_set_vbus(cdns: *mut cdns);
}
extern "C" {
    pub fn cdns_drd_init(cdns: *mut cdns) -> c_int;
}
extern "C" {
    pub fn cdns_drd_exit(cdns: *mut cdns) -> c_int;
}
extern "C" {
    pub fn cdns_drd_update_mode(cdns: *mut cdns) -> c_int;
}
extern "C" {
    pub fn cdns_drd_gadget_on(cdns: *mut cdns) -> c_int;
}
extern "C" {
    pub fn cdns_drd_gadget_off(cdns: *mut cdns);
}
extern "C" {
    pub fn cdns_drd_host_on(cdns: *mut cdns) -> c_int;
}
extern "C" {
    pub fn cdns_drd_host_off(cdns: *mut cdns);
}
extern "C" {
    pub fn cdns_power_is_lost(cdns: *mut cdns) -> bool;
}
