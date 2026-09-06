//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/cdns3/core.h
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
// Cadence USBSS and USBSSP DRD Header File.
//
// Copyright (C) 2017-2018 NXP
// Copyright (C) 2018-2019 Cadence.
//
// Authors: Peter Chen <peter.chen@nxp.com>
// Pawel Laszczak <pawell@cadence.com>
//

//
// struct cdns_role_driver - host/gadget role driver
// @start: start this role
// @stop: stop this role
// @suspend: suspend callback for this role
// @resume: resume callback for this role
// @irq: irq handler for this role
// @name: role name string (host/gadget)
// @state: current state
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns_role_driver {
    pub cdns): *mut *mut int (start)(struct cdns,
    pub cdns): *mut *mut void (stop)(struct cdns,
    pub do_wakeup): *mut *mut *mut int (suspend)(struct cdns cdns, bool,
    pub lost_power): *mut *mut *mut int (resume)(struct cdns cdns, bool,
    pub name: *const c_char,
pub const CDNS_ROLE_STATE_INACTIVE: c_int = 0;
pub const CDNS_ROLE_STATE_ACTIVE: c_int = 1;
    pub state: c_int,
}

pub const CDNS_XHCI_RESOURCES_NUM: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns3_platform_data {
    pub wakeup): bool suspend, bool,
    pub quirks: c_ulong,

    pub /: *mut *mut u32 override_apb_timeout; / 0 = use default (e.g. for PCI),
}

//
// struct cdns - Representation of Cadence USB3 DRD controller.
// @dev: pointer to Cadence device struct
// @xhci_regs: pointer to base of xhci registers
// @xhci_res: the resource for xhci
// @dev_regs: pointer to base of dev registers
// @otg_res: the resource for otg
// @otg_v0_regs: pointer to base of v0 otg registers
// @otg_v1_regs: pointer to base of v1 otg registers
// @otg_cdnsp_regs: pointer to base of CDNSP otg registers
// @otg_regs: pointer to base of otg registers
// @otg_irq_regs: pointer to interrupt registers
// @otg_irq: irq number for otg controller
// @dev_irq: irq number for device controller
// @wakeup_irq: irq number for wakeup event, it is optional
// @roles: array of supported roles for this controller
// @role: current role
// @host_dev: the child host device pointer for cdns core
// @gadget_dev: the child gadget device pointer
// @usb2_phy: pointer to USB2 PHY
// @usb3_phy: pointer to USB3 PHY
// @mutex: the mutex for concurrent code at driver
// @dr_mode: supported mode of operation it can be only Host, only Device
// or OTG mode that allow to switch between Device and Host mode.
// This field based on firmware setting, kernel configuration
// and hardware configuration.
// @role_sw: pointer to role switch object.
// @in_lpm: indicate the controller is in low power mode
// @wakeup_pending: wakeup interrupt pending
// @pdata: platform data from glue layer
// @lock: spinlock structure
// @xhci_plat_data: xhci private data structure pointer
// @override_apb_timeout: hold value of APB timeout. For value 0 the default
// value in CHICKEN_BITS_3 will be preserved.
// @gadget_init: pointer to gadget initialization function
// @host_init: pointer to host initialization function
// @no_drd: DRD register block is inaccessible. The controller is hardwired to
// single role (host or device) or the logic for role switching is
// missing.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdns {
    pub dev: *mut device,
    pub xhci_regs: *mut void __iomem,
    pub xhci_res: [resource; CDNS_XHCI_RESOURCES_NUM],
    pub dev_regs: *mut cdns3_usb_regs __iomem,
    pub otg_res: resource,
    pub otg_v0_regs: *mut cdns3_otg_legacy_regs __iomem,
    pub otg_v1_regs: *mut cdns3_otg_regs __iomem,
    pub otg_cdnsp_regs: *mut cdnsp_otg_regs __iomem,
    pub otg_regs: *mut cdns_otg_common_regs __iomem,
    pub otg_irq_regs: *mut cdns_otg_irq_regs __iomem,
pub const CDNS3_CONTROLLER_V0: c_int = 0;
pub const CDNS3_CONTROLLER_V1: c_int = 1;
pub const CDNSP_CONTROLLER_V2: c_int = 2;
    pub version: u32,
    pub phyrst_a_enable: bool,
    pub otg_irq: c_int,
    pub dev_irq: c_int,
    pub wakeup_irq: c_int,
    pub 1]: *mut *mut cdns_role_driver roles[USB_ROLE_DEVICE +,
    pub role: usb_role,
    pub host_dev: *mut platform_device,
    pub gadget_dev: *mut c_void,
    pub usb2_phy: *mut phy,
    pub usb3_phy: *mut phy,
// mutext used in workqueue
    pub mutex: mutex,
    pub dr_mode: usb_dr_mode,
    pub role_sw: *mut usb_role_switch,
    pub in_lpm: bool,
    pub wakeup_pending: bool,
    pub pdata: *mut cdns3_platform_data,
    pub lock: spinlock_t,
    pub xhci_plat_data: *mut xhci_plat_priv,
    pub override_apb_timeout: u32,
    pub cdns): *mut *mut int (gadget_init)(struct cdns,
    pub cdns): *mut *mut int (host_init)(struct cdns,
    pub no_drd: bool,
}

extern "C" {
    pub fn cdns_hw_role_switch(cdns: *mut cdns) -> c_int;
}
extern "C" {
    pub fn cdns_init(cdns: *mut cdns) -> c_int;
}
extern "C" {
    pub fn cdns_remove(cdns: *mut cdns) -> c_int;
}
extern "C" {
    pub fn cdns_core_init_role(cdns: *mut cdns) -> c_int;
}

extern "C" {
    pub fn cdns_resume(cdns: *mut cdns) -> c_int;
}
extern "C" {
    pub fn cdns_suspend(cdns: *mut cdns) -> c_int;
}
extern "C" {
    pub fn cdns_set_active(cdns: *mut cdns, set_active: u8);
}

