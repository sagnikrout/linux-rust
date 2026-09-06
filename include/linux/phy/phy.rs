//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/phy/phy.h
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
//
// phy.h -- generic phy header file
//
// Copyright (C) 2013 Texas Instruments Incorporated - http://www.ti.com
//
// Author: Kishon Vijay Abraham I <kishon@ti.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum phy_mode {
    PHY_MODE_INVALID,
    PHY_MODE_USB_HOST,
    PHY_MODE_USB_HOST_LS,
    PHY_MODE_USB_HOST_FS,
    PHY_MODE_USB_HOST_HS,
    PHY_MODE_USB_HOST_SS,
    PHY_MODE_USB_DEVICE,
    PHY_MODE_USB_DEVICE_LS,
    PHY_MODE_USB_DEVICE_FS,
    PHY_MODE_USB_DEVICE_HS,
    PHY_MODE_USB_DEVICE_SS,
    PHY_MODE_USB_OTG,
    PHY_MODE_UFS_HS_A,
    PHY_MODE_UFS_HS_B,
    PHY_MODE_PCIE,
    PHY_MODE_ETHERNET,
    PHY_MODE_MIPI_DPHY,
    PHY_MODE_SATA,
    PHY_MODE_LVDS,
    PHY_MODE_DP,
    PHY_MODE_HDMI,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum phy_media {
    PHY_MEDIA_DEFAULT,
    PHY_MEDIA_SR,
    PHY_MEDIA_DAC,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum phy_ufs_state {
    PHY_UFS_HIBERN8_ENTER,
    PHY_UFS_HIBERN8_EXIT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union phy_notify {
    pub ufs_state: phy_ufs_state,
}

//
// union phy_configure_opts - Opaque generic phy configuration
//
// @mipi_dphy:	Configuration set applicable for phys supporting
// the MIPI_DPHY phy mode.
// @dp:		Configuration set applicable for phys supporting
// the DisplayPort protocol.
// @lvds:	Configuration set applicable for phys supporting
// the LVDS phy mode.
// @hdmi:	Configuration set applicable for phys supporting
// the HDMI phy mode.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union phy_configure_opts {
    pub mipi_dphy: phy_configure_opts_mipi_dphy,
    pub dp: phy_configure_opts_dp,
    pub lvds: phy_configure_opts_lvds,
    pub hdmi: phy_configure_opts_hdmi,
}

//
// struct phy_ops - set of function pointers for performing phy operations
// @init: operation to be performed for initializing phy
// @exit: operation to be performed while exiting
// @power_on: powering on the phy
// @power_off: powering off the phy
// @set_mode: set the mode of the phy
// @set_media: set the media type of the phy (optional)
// @set_speed: set the speed of the phy (optional)
// @reset: resetting the phy
// @calibrate: calibrate the phy
// @notify_phystate: notify and configure the phy for a particular state
// @release: ops to be performed while the consumer relinquishes the PHY
// @owner: the module owner containing the ops
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_ops {
    pub phy): *mut *mut int (init)(struct phy,
    pub phy): *mut *mut int (exit)(struct phy,
    pub phy): *mut *mut int (power_on)(struct phy,
    pub phy): *mut *mut int (power_off)(struct phy,
    pub submode): *mut *mut *mut int (set_mode)(struct phy phy, enum phy_mode mode, int,
    pub media): *mut *mut *mut int (set_media)(struct phy phy, enum phy_media,
    pub speed): *mut *mut *mut int (set_speed)(struct phy phy, int,
//
// @configure:
//
// Optional.
//
// Used to change the PHY parameters. phy_init() must have
// been called on the phy.
//
// Returns: 0 if successful, an negative error code otherwise
//
    pub opts): *mut *mut *mut int (configure)(struct phy phy, union phy_configure_opts,
//
// @validate:
//
// Optional.
//
// Used to check that the current set of parameters can be
// handled by the phy. Implementations are free to tune the
// parameters passed as arguments if needed by some
// implementation detail or constraints. It must not change
// any actual configuration of the PHY, so calling it as many
// times as deemed fit by the consumer must have no side
// effect.
//
// Returns: 0 if the configuration can be applied, an negative
// error code otherwise
//
    pub opts): *mut phy_configure_opts,
    pub phy): *mut *mut int (reset)(struct phy,
    pub phy): *mut *mut int (calibrate)(struct phy,
// notify phy connect status change
    pub port): *mut *mut *mut int (connect)(struct phy phy, int,
    pub port): *mut *mut *mut int (disconnect)(struct phy phy, int,
    pub state): *mut *mut *mut int (notify_phystate)(struct phy phy, union phy_notify,
    pub phy): *mut *mut void (release)(struct phy,
    pub owner: *mut module,
}

//
// struct phy_attrs - represents phy attributes
// @bus_width: Data path width implemented by PHY
// @max_link_rate: Maximum link rate supported by PHY (units to be decided by producer and consumer)
// @mode: PHY mode
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_attrs {
    pub bus_width: u32,
    pub max_link_rate: u32,
    pub mode: phy_mode,
}

//
// struct phy - represents the phy device
// @dev: phy device
// @id: id of the phy device
// @ops: function pointers for performing phy operations
// @mutex: mutex to protect phy_ops
// @lockdep_key: lockdep information for this mutex
// @init_count: used to protect when the PHY is used by multiple consumers
// @power_count: used to protect when the PHY is used by multiple consumers
// @attrs: used to specify PHY specific attributes
// @pwr: power regulator associated with the phy
// @debugfs: debugfs directory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy {
    pub dev: device,
    pub id: c_int,
    pub ops: *const phy_ops,
    pub mutex: mutex,
    pub lockdep_key: lock_class_key,
    pub init_count: c_int,
    pub power_count: c_int,
    pub attrs: phy_attrs,
    pub pwr: *mut regulator,
    pub debugfs: *mut dentry,
}

//
// struct phy_provider - represents the phy provider
// @dev: phy provider device
// @children: can be used to override the default (dev->of_node) child node
// @owner: the module owner having of_xlate
// @list: to maintain a linked list of PHY providers
// @of_xlate: function pointer to obtain phy instance from phy pointer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_provider {
    pub dev: *mut device,
    pub children: *mut device_node,
    pub owner: *mut module,
    pub list: list_head,
    pub args): *const of_phandle_args,
}

//
// struct phy_lookup - PHY association in list of phys managed by the phy driver
// @node: list node
// @dev_id: the device of the association
// @con_id: connection ID string on device
// @phy: the phy of the association
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct phy_lookup {
    pub node: list_head,
    pub dev_id: *const c_char,
    pub con_id: *const c_char,
    pub phy: *mut phy,
}

extern "C" {
    pub fn dev_get_drvdata(_arg: &phy->dev) -> return;
}

extern "C" {
    pub fn phy_pm_runtime_get(phy: *mut phy) -> c_int;
}
extern "C" {
    pub fn phy_pm_runtime_get_sync(phy: *mut phy) -> c_int;
}
extern "C" {
    pub fn phy_pm_runtime_put(phy: *mut phy);
}
extern "C" {
    pub fn phy_pm_runtime_put_sync(phy: *mut phy) -> c_int;
}
extern "C" {
    pub fn phy_init(phy: *mut phy) -> c_int;
}
extern "C" {
    pub fn phy_exit(phy: *mut phy) -> c_int;
}
extern "C" {
    pub fn phy_power_on(phy: *mut phy) -> c_int;
}
extern "C" {
    pub fn phy_power_off(phy: *mut phy) -> c_int;
}
extern "C" {
    pub fn phy_set_mode_ext(phy: *mut phy, mode: phy_mode, submode: c_int) -> c_int;
}

extern "C" {
    pub fn phy_set_media(phy: *mut phy, media: phy_media) -> c_int;
}
extern "C" {
    pub fn phy_set_speed(phy: *mut phy, speed: c_int) -> c_int;
}
extern "C" {
    pub fn phy_configure(phy: *mut phy, opts: *mut phy_configure_opts) -> c_int;
}
extern "C" {
    pub fn phy_reset(phy: *mut phy) -> c_int;
}
extern "C" {
    pub fn phy_calibrate(phy: *mut phy) -> c_int;
}
extern "C" {
    pub fn phy_notify_connect(phy: *mut phy, port: c_int) -> c_int;
}
extern "C" {
    pub fn phy_notify_disconnect(phy: *mut phy, port: c_int) -> c_int;
}
extern "C" {
    pub fn phy_notify_state(phy: *mut phy, state: phy_notify) -> c_int;
}
extern "C" {
    pub fn of_phy_put(phy: *mut phy);
}
extern "C" {
    pub fn phy_put(dev: *mut device, phy: *mut phy);
}
extern "C" {
    pub fn devm_phy_put(dev: *mut device, phy: *mut phy);
}
extern "C" {
    pub fn phy_destroy(phy: *mut phy);
}
extern "C" {
    pub fn devm_phy_destroy(dev: *mut device, phy: *mut phy);
}
extern "C" {
    pub fn of_phy_provider_unregister(phy_provider: *mut phy_provider);
}
extern "C" {
    pub fn phy_create_lookup(phy: *mut phy, con_id: *const c_char, dev_id: *const c_char) -> c_int;
}
extern "C" {
    pub fn phy_remove_lookup(phy: *mut phy, con_id: *const c_char, dev_id: *const c_char);
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENOSYS) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENOSYS) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENOSYS) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENOSYS) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENOSYS) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENOSYS) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENOSYS) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENOSYS) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENOSYS) -> return;
}
// dev, struct device_node *children, struct module *owner,
extern "C" {
    pub fn ERR_PTR(_arg: -ENOSYS) -> return;
}

