//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/v4l2-device.h
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

//
// struct v4l2_device - main struct to for V4L2 device drivers
//
// @dev: pointer to struct device.
// @mdev: pointer to struct media_device, may be NULL.
// @subdevs: used to keep track of the registered subdevs
// @lock: lock this struct; can be used by the driver as well
// if this struct is embedded into a larger struct.
// @name: unique device name, by default the driver name + bus ID
// @notify: notify operation called by some sub-devices.
// @ctrl_handler: The control handler. May be %NULL.
// @prio: Device's priority state
// @ref: Keep track of the references to this struct.
// @release: Release function that is called when the ref count
// goes to 0.
//
// Each instance of a V4L2 device should create the v4l2_device struct,
// either stand-alone or embedded in a larger struct.
//
// It allows easy access to sub-devices (see v4l2-subdev.h) and provides
// basic V4L2 device-level support.
//
// .. note::
//
// #) @dev->driver_data points to this struct.
// #) @dev might be %NULL if there is no parent device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_device {
    pub dev: *mut device,
    pub mdev: *mut media_device,
    pub subdevs: list_head,
    pub lock: spinlock_t,
    pub name: [c_char; 36],
    pub arg): *mut unsigned int notification, void,
    pub ctrl_handler: *mut v4l2_ctrl_handler,
    pub prio: v4l2_prio_state,
    pub ref: kref,
    pub v4l2_dev): *mut *mut void (release)(struct v4l2_device,
}

//
// v4l2_device_get - gets a V4L2 device reference
//
// @v4l2_dev: pointer to struct &v4l2_device
//
// This is an ancillary routine meant to increment the usage for the
// struct &v4l2_device pointed by @v4l2_dev.
//
// v4l2_device_put - puts a V4L2 device reference
//
// @v4l2_dev: pointer to struct &v4l2_device
//
// This is an ancillary routine meant to decrement the usage for the
// struct &v4l2_device pointed by @v4l2_dev.
//
extern "C" {
    pub fn v4l2_device_put(v4l2_dev: *mut v4l2_device) -> c_int;
}
//
// v4l2_device_register - Initialize v4l2_dev and make @dev->driver_data
// point to @v4l2_dev.
//
// @dev: pointer to struct &device
// @v4l2_dev: pointer to struct &v4l2_device
//
// .. note::
// @dev may be %NULL in rare cases (ISA devices).
// In such case the caller must fill in the @v4l2_dev->name field
// before calling this function.
//
// v4l2_device_set_name - Optional function to initialize the
// name field of struct &v4l2_device
//
// @v4l2_dev: pointer to struct &v4l2_device
// @basename: base name for the device name
// @instance: pointer to a static atomic_t var with the instance usage for
// the device driver.
//
// v4l2_device_set_name() initializes the name field of struct &v4l2_device
// using the driver name and a driver-global atomic_t instance.
//
// This function will increment the instance counter and returns the
// instance value used in the name.
//
// Example:
//
// static atomic_t drv_instance = ATOMIC_INIT(0);
//
// ...
//
// instance = v4l2_device_set_name(&\ v4l2_dev, "foo", &\ drv_instance);
//
// The first time this is called the name field will be set to foo0 and
// this function returns 0. If the name ends with a digit (e.g. cx18),
// then the name will be set to cx18-0 since cx180 would look really odd.
//
// v4l2_device_disconnect - Change V4L2 device state to disconnected.
//
// @v4l2_dev: pointer to struct v4l2_device
//
// Should be called when the USB parent disconnects.
// Since the parent disappears, this ensures that @v4l2_dev doesn't have
// an invalid parent pointer.
//
// .. note:: This function sets @v4l2_dev->dev to NULL.
//
extern "C" {
    pub fn v4l2_device_disconnect(v4l2_dev: *mut v4l2_device);
}
//
// v4l2_device_unregister - Unregister all sub-devices and any other
// resources related to @v4l2_dev.
//
// @v4l2_dev: pointer to struct v4l2_device
//
extern "C" {
    pub fn v4l2_device_unregister(v4l2_dev: *mut v4l2_device);
}
//
// v4l2_device_register_subdev - Registers a subdev with a v4l2 device.
//
// @v4l2_dev: pointer to struct &v4l2_device
// @sd: pointer to &struct v4l2_subdev
//
// While registered, the subdev module is marked as in-use.
//
// An error is returned if the module is no longer loaded on any attempts
// to register it.
//

//
// v4l2_device_unregister_subdev - Unregisters a subdev with a v4l2 device.
//
// @sd: pointer to &struct v4l2_subdev
//
// .. note ::
//
// Can also be called if the subdev wasn't registered. In such
// case, it will do nothing.
//
extern "C" {
    pub fn v4l2_device_unregister_subdev(sd: *mut v4l2_subdev);
}
//
// __v4l2_device_register_subdev_nodes - Registers device nodes for
// all subdevs of the v4l2 device that are marked with the
// %V4L2_SUBDEV_FL_HAS_DEVNODE flag.
//
// @v4l2_dev: pointer to struct v4l2_device
// @read_only: subdevices read-only flag. True to register the subdevices
// device nodes in read-only mode, false to allow full access to the
// subdevice userspace API.
//
// v4l2_device_register_subdev_nodes - Registers subdevices device nodes with
// unrestricted access to the subdevice userspace operations
//
// Internally calls __v4l2_device_register_subdev_nodes(). See its documentation
// for more details.
//
// @v4l2_dev: pointer to struct v4l2_device
//

extern "C" {
    pub fn __v4l2_device_register_subdev_nodes(_arg: v4l2_dev, _arg: false) -> return;
}

//
// v4l2_device_register_ro_subdev_nodes - Registers subdevices device nodes
// in read-only mode
//
// Internally calls __v4l2_device_register_subdev_nodes(). See its documentation
// for more details.
//
// @v4l2_dev: pointer to struct v4l2_device
//

extern "C" {
    pub fn __v4l2_device_register_subdev_nodes(_arg: v4l2_dev, _arg: true) -> return;
}

//
// v4l2_subdev_notify - Sends a notification to v4l2_device.
//
// @sd: pointer to &struct v4l2_subdev
// @notification: type of notification. Please notice that the notification
// type is driver-specific.
// @arg: arguments for the notification. Those are specific to each
// notification type.
//
// v4l2_device_supports_requests - Test if requests are supported.
//
// @v4l2_dev: pointer to struct v4l2_device
//
// Helper macros to iterate over all subdevs.
//
// v4l2_device_for_each_subdev - Helper macro that interates over all
// sub-devices of a given &v4l2_device.
//
// @sd: pointer that will be filled by the macro with all
// &struct v4l2_subdev pointer used as an iterator by the loop.
// @v4l2_dev: &struct v4l2_device owning the sub-devices to iterate over.
//
// This macro iterates over all sub-devices owned by the @v4l2_dev device.
// It acts as a for loop iterator and executes the next statement with
// the @sd variable pointing to each sub-device in turn.
//

//
// __v4l2_device_call_subdevs_p - Calls the specified operation for
// all subdevs matching the condition.
//
// @v4l2_dev: &struct v4l2_device owning the sub-devices to iterate over.
// @sd: pointer that will be filled by the macro with all
// &struct v4l2_subdev pointer used as an iterator by the loop.
// @cond: condition to be match
// @o: name of the element at &struct v4l2_subdev_ops that contains @f.
// Each element there groups a set of operations functions.
// @f: operation function that will be called if @cond matches.
// The operation functions are defined in groups, according to
// each element at &struct v4l2_subdev_ops.
// @args: arguments for @f.
//
// Ignore any errors.
//
// Note: subdevs cannot be added or deleted while walking
// the subdevs list.
//

//
// __v4l2_device_call_subdevs - Calls the specified operation for
// all subdevs matching the condition.
//
// @v4l2_dev: &struct v4l2_device owning the sub-devices to iterate over.
// @cond: condition to be match
// @o: name of the element at &struct v4l2_subdev_ops that contains @f.
// Each element there groups a set of operations functions.
// @f: operation function that will be called if @cond matches.
// The operation functions are defined in groups, according to
// each element at &struct v4l2_subdev_ops.
// @args: arguments for @f.
//
// Ignore any errors.
//
// Note: subdevs cannot be added or deleted while walking
// the subdevs list.
//

//
// __v4l2_device_call_subdevs_until_err_p - Calls the specified operation for
// all subdevs matching the condition.
//
// @v4l2_dev: &struct v4l2_device owning the sub-devices to iterate over.
// @sd: pointer that will be filled by the macro with all
// &struct v4l2_subdev sub-devices associated with @v4l2_dev.
// @cond: condition to be match
// @o: name of the element at &struct v4l2_subdev_ops that contains @f.
// Each element there groups a set of operations functions.
// @f: operation function that will be called if @cond matches.
// The operation functions are defined in groups, according to
// each element at &struct v4l2_subdev_ops.
// @args: arguments for @f.
//
// Return:
//
// If the operation returns an error other than 0 or ``-ENOIOCTLCMD``
// for any subdevice, then abort and return with that error code, zero
// otherwise.
//
// Note: subdevs cannot be added or deleted while walking
// the subdevs list.
//

//
// __v4l2_device_call_subdevs_until_err - Calls the specified operation for
// all subdevs matching the condition.
//
// @v4l2_dev: &struct v4l2_device owning the sub-devices to iterate over.
// @cond: condition to be match
// @o: name of the element at &struct v4l2_subdev_ops that contains @f.
// Each element there groups a set of operations functions.
// @f: operation function that will be called if @cond matches.
// The operation functions are defined in groups, according to
// each element at &struct v4l2_subdev_ops.
// @args: arguments for @f.
//
// Return:
//
// If the operation returns an error other than 0 or ``-ENOIOCTLCMD``
// for any subdevice, then abort and return with that error code,
// zero otherwise.
//
// Note: subdevs cannot be added or deleted while walking
// the subdevs list.
//

//
// v4l2_device_call_all - Calls the specified operation for
// all subdevs matching the &v4l2_subdev.grp_id, as assigned
// by the bridge driver.
//
// @v4l2_dev: &struct v4l2_device owning the sub-devices to iterate over.
// @grpid: &struct v4l2_subdev->grp_id group ID to match.
// Use 0 to match them all.
// @o: name of the element at &struct v4l2_subdev_ops that contains @f.
// Each element there groups a set of operations functions.
// @f: operation function that will be called if @cond matches.
// The operation functions are defined in groups, according to
// each element at &struct v4l2_subdev_ops.
// @args: arguments for @f.
//
// Ignore any errors.
//
// Note: subdevs cannot be added or deleted while walking
// the subdevs list.
//

//
// v4l2_device_call_until_err - Calls the specified operation for
// all subdevs matching the &v4l2_subdev.grp_id, as assigned
// by the bridge driver, until an error occurs.
//
// @v4l2_dev: &struct v4l2_device owning the sub-devices to iterate over.
// @grpid: &struct v4l2_subdev->grp_id group ID to match.
// Use 0 to match them all.
// @o: name of the element at &struct v4l2_subdev_ops that contains @f.
// Each element there groups a set of operations functions.
// @f: operation function that will be called if @cond matches.
// The operation functions are defined in groups, according to
// each element at &struct v4l2_subdev_ops.
// @args: arguments for @f.
//
// Return:
//
// If the operation returns an error other than 0 or ``-ENOIOCTLCMD``
// for any subdevice, then abort and return with that error code,
// zero otherwise.
//
// Note: subdevs cannot be added or deleted while walking
// the subdevs list.
//

//
// v4l2_device_mask_call_all - Calls the specified operation for
// all subdevices where a group ID matches a specified bitmask.
//
// @v4l2_dev: &struct v4l2_device owning the sub-devices to iterate over.
// @grpmsk: bitmask to be checked against &struct v4l2_subdev->grp_id
// group ID to be matched. Use 0 to match them all.
// @o: name of the element at &struct v4l2_subdev_ops that contains @f.
// Each element there groups a set of operations functions.
// @f: operation function that will be called if @cond matches.
// The operation functions are defined in groups, according to
// each element at &struct v4l2_subdev_ops.
// @args: arguments for @f.
//
// Ignore any errors.
//
// Note: subdevs cannot be added or deleted while walking
// the subdevs list.
//

//
// v4l2_device_mask_call_until_err - Calls the specified operation for
// all subdevices where a group ID matches a specified bitmask.
//
// @v4l2_dev: &struct v4l2_device owning the sub-devices to iterate over.
// @grpmsk: bitmask to be checked against &struct v4l2_subdev->grp_id
// group ID to be matched. Use 0 to match them all.
// @o: name of the element at &struct v4l2_subdev_ops that contains @f.
// Each element there groups a set of operations functions.
// @f: operation function that will be called if @cond matches.
// The operation functions are defined in groups, according to
// each element at &struct v4l2_subdev_ops.
// @args: arguments for @f.
//
// Return:
//
// If the operation returns an error other than 0 or ``-ENOIOCTLCMD``
// for any subdevice, then abort and return with that error code,
// zero otherwise.
//
// Note: subdevs cannot be added or deleted while walking
// the subdevs list.
//

//
// v4l2_device_has_op - checks if any subdev with matching grpid has a
// given ops.
//
// @v4l2_dev: &struct v4l2_device owning the sub-devices to iterate over.
// @grpid: &struct v4l2_subdev->grp_id group ID to match.
// Use 0 to match them all.
// @o: name of the element at &struct v4l2_subdev_ops that contains @f.
// Each element there groups a set of operations functions.
// @f: operation function that will be called if @cond matches.
// The operation functions are defined in groups, according to
// each element at &struct v4l2_subdev_ops.
//

//
// v4l2_device_mask_has_op - checks if any subdev with matching group
// mask has a given ops.
//
// @v4l2_dev: &struct v4l2_device owning the sub-devices to iterate over.
// @grpmsk: bitmask to be checked against &struct v4l2_subdev->grp_id
// group ID to be matched. Use 0 to match them all.
// @o: name of the element at &struct v4l2_subdev_ops that contains @f.
// Each element there groups a set of operations functions.
// @f: operation function that will be called if @cond matches.
// The operation functions are defined in groups, according to
// each element at &struct v4l2_subdev_ops.
//

