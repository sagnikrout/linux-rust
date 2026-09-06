//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/usb/typec_altmode.h
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

pub const MODE_DISCOVERY_MAX: c_int = 6;

//
// struct typec_altmode - USB Type-C alternate mode device
// @dev: Driver model's view of this device
// @svid: Standard or Vendor ID (SVID) of the alternate mode
// @mode: Index of the Mode
// @vdo: VDO returned by Discover Modes USB PD command
// @active: Tells has the mode been entered or not
// @priority: Priority used by the automatic alternate mode selection process
// @mode_selection: Whether entry to this alternate mode is managed by the
// automatic alternate mode selection process or by the specific driver
// @desc: Optional human readable description of the mode
// @ops: Operations vector from the driver
// @cable_ops: Cable operations vector from the driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct typec_altmode {
    pub dev: device,
    pub svid: u16,
    pub mode: c_int,
    pub vdo: u32,
    pub active:1: c_uint,
    pub priority: u8,
    pub mode_selection: bool,
    pub desc: *mut c_char,
    pub ops: *const typec_altmode_ops,
    pub cable_ops: *const typec_cable_ops,
}

extern "C" {
    pub fn dev_get_drvdata(_arg: &altmode->dev) -> return;
}
//
// struct typec_altmode_ops - Alternate mode specific operations vector
// @enter: Operations to be executed with Enter Mode Command
// @exit: Operations to be executed with Exit Mode Command
// @attention: Callback for Attention Command
// @vdm: Callback for SVID specific commands
// @notify: Communication channel for platform and the alternate mode
// @activate: User callback for Enter/Exit Mode
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct typec_altmode_ops {
    pub vdo): *mut *mut *mut int (enter)(struct typec_altmode altmode, u32,
    pub altmode): *mut *mut int (exit)(struct typec_altmode,
    pub vdo): *mut *mut *mut void (attention)(struct typec_altmode altmode, u32,
    pub cnt): *const *const u32 vdo, int,
    pub data): *mut c_void,
    pub activate): *mut *mut *mut int (activate)(struct typec_altmode altmode, int,
}

extern "C" {
    pub fn typec_altmode_enter(altmode: *mut typec_altmode, vdo: *mut u32) -> c_int;
}
extern "C" {
    pub fn typec_altmode_exit(altmode: *mut typec_altmode) -> c_int;
}
extern "C" {
    pub fn typec_altmode_attention(altmode: *mut typec_altmode, vdo: u32) -> c_int;
}
//
// struct typec_cable_ops - Cable alternate mode operations vector
// @enter: Operations to be executed with Enter Mode Command
// @exit: Operations to be executed with Exit Mode Command
// @vdm: Callback for SVID specific commands
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct typec_cable_ops {
    pub vdo): *mut *mut *mut int (enter)(struct typec_altmode altmode, enum typec_plug_index sop, u32,
    pub sop): *mut *mut *mut int (exit)(struct typec_altmode altmode, enum typec_plug_index,
    pub cnt): *const *const u32 hdr, u32 vdo, int,
}

extern "C" {
    pub fn typec_cable_altmode_enter(altmode: *mut typec_altmode, sop: typec_plug_index, vdo: *mut u32) -> c_int;
}
extern "C" {
    pub fn typec_cable_altmode_exit(altmode: *mut typec_altmode, sop: typec_plug_index) -> c_int;
}
//
// typec_altmode_get_cable_svdm_version - Get negotiated SVDM version for cable plug
// @altmode: Handle to the alternate mode
//
extern "C" {
    pub fn typec_get_cable_svdm_version(_arg: typec_altmode2port(altmode)) -> return;
}
//
// These are the connector states (USB, Safe and Alt Mode) defined in USB Type-C
// Specification. SVID specific connector states are expected to follow and
// start from the value TYPEC_STATE_MODAL.
//
// For the muxes there is no difference between Accessory Modes and Alternate
// Modes, so the Accessory Modes are supplied with specific modal state values
// here. Unlike with Alternate Modes, where the mux will be linked with the
// alternate mode device, the mux for Accessory Modes will be linked with the
// port device instead.
//
// Port drivers can use TYPEC_MODE_AUDIO and TYPEC_MODE_DEBUG as the mode
// value for typec_set_mode() when accessory modes are supported.
//
// USB4 also requires that the pins on the connector are repurposed, just like
// Alternate Modes. USB4 mode is however not entered with the Enter Mode Command
// like the Alternate Modes are, but instead with a special Enter_USB Message.
// The Enter_USB Message can also be used for setting to connector to operate in
// USB 3.2 or in USB 2.0 mode instead of USB4.
//
// The Enter_USB specific "USB Modes" are also supplied here as special modal
// state values, just like the Accessory Modes.
//

extern "C" {
    pub fn typec_altmode_put_plug(plug: *mut typec_altmode);
}
//
// typec_altmode_get_orientation - Get cable plug orientation
// @altmode: Handle to the alternate mode
//
extern "C" {
    pub fn typec_get_orientation(_arg: typec_altmode2port(altmode)) -> return;
}
//
// typec_altmode_get_svdm_version - Get negotiated SVDM version
// @altmode: Handle to the alternate mode
//
extern "C" {
    pub fn typec_get_negotiated_svdm_version(_arg: typec_altmode2port(altmode)) -> return;
}
//
// typec_altmode_get_data_role - Get port data role
// @altmode: Handle to the alternate mode
//
// Alt Mode drivers should only issue Enter Mode through the port if they are
// the DFP.
//
extern "C" {
    pub fn typec_get_data_role(_arg: typec_altmode2port(altmode)) -> return;
}
//
// struct typec_altmode_driver - USB Type-C alternate mode device driver
// @id_table: Null terminated array of SVIDs
// @probe: Callback for device binding
// @remove: Callback for device unbinding
// @driver: Device driver model driver
//
// These drivers will be bind to the partner alternate mode devices. They will
// handle all SVID specific communication.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct typec_altmode_driver {
    pub id_table: *const typec_device_id,
    pub altmode): *mut *mut int (probe)(struct typec_altmode,
    pub altmode): *mut *mut void (remove)(struct typec_altmode,
    pub driver: device_driver,
}

//
// typec_altmode_register_driver - registers a USB Type-C alternate mode
// device driver
// @drv: pointer to struct typec_altmode_driver
//
// These drivers will be bind to the partner alternate mode devices. They will
// handle all SVID specific communication.
//

//
// typec_altmode_unregister_driver - unregisters a USB Type-C alternate mode
// device driver
// @drv: pointer to struct typec_altmode_driver
//
// These drivers will be bind to the partner alternate mode devices. They will
// handle all SVID specific communication.
//
extern "C" {
    pub fn typec_altmode_unregister_driver(drv: *mut typec_altmode_driver);
}

//
// typec_mode_selection_start - Start an alternate mode selection process
// @partner: Handle to the Type-C partner device
// @delay: Delay between mode entry/exit attempts, ms
// @timeout: Timeout for a mode entry attempt, ms
//
// This function initiates the process of attempting to enter an Alternate Mode
// supported by the connected Type-C partner.
// Returns 0 on success, or a negative error code on failure.
//
// typec_altmode_state_update - Report the current status of an Alternate Mode
// negotiation
// @partner: Handle to the Type-C partner device
// @svid: Standard or Vendor ID of the Alternate Mode. A value of 0 should be
// passed if no mode is currently active
// @result: Result of the entry operation. This should be 0 on success, or a
// negative error code if the negotiation failed
//
// This function should be called by an Alternate Mode driver to report the
// result of an asynchronous alternate mode entry request. It signals what the
// current active SVID is (or 0 if none) and the success or failure status of
// the last attempt.
//
// typec_mode_selection_delete - Delete an alternate mode selection instance
// @partner: Handle to the Type-C partner device.
//
// This function cancels a pending alternate mode selection request that was
// previously started with typec_mode_selection_start().
// This is typically called when the partner disconnects.
//
extern "C" {
    pub fn typec_mode_selection_delete(partner: *mut typec_partner);
}
