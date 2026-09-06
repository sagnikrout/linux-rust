//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/backlight.h
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
// Backlight Lowlevel Control Abstraction
//
// Copyright (C) 2003,2004 Hewlett-Packard Company
//

//
// enum backlight_update_reason - what method was used to update backlight
//
// A driver indicates the method (reason) used for updating the backlight
// when calling backlight_force_update().
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum backlight_update_reason {
//
// @BACKLIGHT_UPDATE_HOTKEY: The backlight was updated using a hot-key.
//
    BACKLIGHT_UPDATE_HOTKEY,

//
// @BACKLIGHT_UPDATE_SYSFS: The backlight was updated using sysfs.
//
    BACKLIGHT_UPDATE_SYSFS,
}

//
// enum backlight_type - the type of backlight control
//
// The type of interface used to control the backlight.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum backlight_type {
//
// @BACKLIGHT_RAW:
//
// The backlight is controlled using hardware registers.
//
    BACKLIGHT_RAW = 1,

//
// @BACKLIGHT_PLATFORM:
//
// The backlight is controlled using a platform-specific interface.
//
    BACKLIGHT_PLATFORM,

//
// @BACKLIGHT_FIRMWARE:
//
// The backlight is controlled using a standard firmware interface.
//
    BACKLIGHT_FIRMWARE,

//
// @BACKLIGHT_TYPE_MAX: Number of entries.
//
    BACKLIGHT_TYPE_MAX,
}

// enum backlight_scale - the type of scale used for brightness values
//
// The type of scale used for brightness values.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum backlight_scale {
//
// @BACKLIGHT_SCALE_UNKNOWN: The scale is unknown.
//
    BACKLIGHT_SCALE_UNKNOWN = 0,

//
// @BACKLIGHT_SCALE_LINEAR: The scale is linear.
//
// The linear scale will increase brightness the same for each step.
//
    BACKLIGHT_SCALE_LINEAR,

//
// @BACKLIGHT_SCALE_NON_LINEAR: The scale is not linear.
//
// This is often used when the brightness values tries to adjust to
// the relative perception of the eye demanding a non-linear scale.
//
    BACKLIGHT_SCALE_NON_LINEAR,
}

//
// struct backlight_ops - backlight operations
//
// The backlight operations are specified when the backlight device is registered.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct backlight_ops {
//
// @options: Configure how operations are called from the core.
//
// The options parameter is used to adjust the behaviour of the core.
// Set BL_CORE_SUSPENDRESUME to get the update_status() operation called
// upon suspend and resume.
//
    pub options: c_uint,

//
// @update_status: Operation called when properties have changed.
//
// Notify the backlight driver some property has changed.
// The update_status operation is protected by the update_lock.
//
// The backlight driver is expected to use backlight_is_blank()
// to check if the display is blanked and set brightness accordingly.
// update_status() is called when any of the properties has changed.
//
// RETURNS:
//
// 0 on success, negative error code if any failure occurred.
//
    pub ): *mut *mut int (update_status)(struct backlight_device,
//
// @get_brightness: Return the current backlight brightness.
//
// The driver may implement this as a readback from the HW.
// This operation is optional and if not present then the current
// brightness property value is used.
//
// RETURNS:
//
// A brightness value which is 0 or a positive number.
// On failure a negative error code is returned.
//
    pub ): *mut *mut int (get_brightness)(struct backlight_device,
//
// @controls_device: Check against the display device
//
// Check if the backlight controls the given display device. This
// operation is optional and if not implemented it is assumed that
// the display is always the one controlled by the backlight.
//
// RETURNS:
//
// If display_dev is NULL or display_dev matches the device controlled by
// the backlight, return true. Otherwise return false.
//
    pub display_dev): *mut *mut *mut bool (controls_device)(struct backlight_device bd, struct device,
}

//
// struct backlight_properties - backlight properties
//
// This structure defines all the properties of a backlight.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct backlight_properties {
//
// @brightness: The current brightness requested by the user.
//
// The backlight core makes sure the range is (0 to max_brightness)
// when the brightness is set via the sysfs attribute:
// /sys/class/backlight/<backlight>/brightness.
//
// This value can be set in the backlight_properties passed
// to devm_backlight_device_register() to set a default brightness
// value.
//
    pub brightness: c_int,
//
// @max_brightness: The maximum brightness value.
//
// This value must be set in the backlight_properties passed to
// devm_backlight_device_register() and shall not be modified by the
// driver after registration.
//
    pub max_brightness: c_int,
//
// @power: The current power mode.
//
// User space can configure the power mode using the sysfs
// attribute: /sys/class/backlight/<backlight>/bl_power
// When the power property is updated update_status() is called.
//
// The possible values are: (0: full on, 4: full off), see
// BACKLIGHT_POWER constants.
//
// When the backlight device is enabled, @power is set to
// BACKLIGHT_POWER_ON. When the backlight device is disabled,
// @power is set to BACKLIGHT_POWER_OFF.
//
    pub power: c_int,

//
// @type: The type of backlight supported.
//
// The backlight type allows userspace to make appropriate
// policy decisions based on the backlight type.
//
// This value must be set in the backlight_properties
// passed to devm_backlight_device_register().
//
    pub type: backlight_type,
//
// @state: The state of the backlight core.
//
// The state is a bitmask. BL_CORE_FBBLANK is set when the display
// is expected to be blank. BL_CORE_SUSPENDED is set when the
// driver is suspended.
//
// backlight drivers are expected to use backlight_is_blank()
// in their update_status() operation rather than reading the
// state property.
//
// The state is maintained by the core and drivers may not modify it.
//
    pub state: c_uint,

//
// @scale: The type of the brightness scale.
//
    pub scale: backlight_scale,
}

//
// struct backlight_device - backlight device data
//
// This structure holds all data required by a backlight device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct backlight_device {
//
// @props: Backlight properties
//
    pub props: backlight_properties,
//
// @update_lock: The lock used when calling the update_status() operation.
//
// update_lock is an internal backlight lock that serialise access
// to the update_status() operation. The backlight core holds the update_lock
// when calling the update_status() operation. The update_lock shall not
// be used by backlight drivers.
//
    pub update_lock: mutex,
//
// @ops_lock: The lock used around everything related to backlight_ops.
//
// ops_lock is an internal backlight lock that protects the ops pointer
// and is used around all accesses to ops and when the operations are
// invoked. The ops_lock shall not be used by backlight drivers.
//
    pub ops_lock: mutex,
//
// @ops: Pointer to the backlight operations.
//
// If ops is NULL, the driver that registered this device has been unloaded,
// and if class_get_devdata() points to something in the body of that driver,
// it is also invalid.
//
    pub ops: *const backlight_ops,
//
// @entry: List entry of all registered backlight devices
//
    pub entry: list_head,
//
// @dev: Parent device.
//
    pub dev: device,
//
// @use_count: The number of unblanked displays.
//
    pub use_count: c_int,
}

//
// backlight_update_status - force an update of the backlight device status
// @bd: the backlight device
//
// backlight_enable - Enable backlight
// @bd: the backlight device to enable
//
extern "C" {
    pub fn backlight_update_status(_arg: bd) -> return;
}
//
// backlight_disable - Disable backlight
// @bd: the backlight device to disable
//
extern "C" {
    pub fn backlight_update_status(_arg: bd) -> return;
}
//
// backlight_is_blank - Return true if display is expected to be blank
// @bd: the backlight device
//
// Display is expected to be blank if any of these is true::
//
// 1) if power in not UNBLANK
// 2) if state indicate BLANK or SUSPENDED
//
// Returns true if display is expected to be blank, false otherwise.
//
// backlight_get_brightness - Returns the current brightness value
// @bd: the backlight device
//
// Returns the current brightness value, taking in consideration the current
// state. If backlight_is_blank() returns true then return 0 as brightness
// otherwise return the current brightness property value.
//
// Backlight drivers are expected to use this function in their update_status()
// operation to get the brightness value.
//
extern "C" {
    pub fn backlight_device_unregister(bd: *mut backlight_device);
}

//
// bl_get_data - access devdata
// @bl_dev: pointer to backlight device
//
// When a backlight device is registered the driver has the possibility
// to supply a void * devdata. bl_get_data() return a pointer to the
// devdata.
//
// RETURNS:
//
// pointer to devdata stored while registering the backlight device.
//
extern "C" {
    pub fn dev_get_drvdata(_arg: &bl_dev->dev) -> return;
}

