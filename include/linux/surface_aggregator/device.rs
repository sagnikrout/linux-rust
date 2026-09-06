//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/surface_aggregator/device.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Surface System Aggregator Module (SSAM) bus and client-device subsystem.
//
// Main interface for the surface-aggregator bus, surface-aggregator client
// devices, and respective drivers building on top of the SSAM controller.
// Provides support for non-platform/non-ACPI SSAM clients via dedicated
// subsystem.
//
// Copyright (C) 2019-2021 Maximilian Luz <luzmaximilian@gmail.com>
//

// -- Surface System Aggregator Module bus. ---------------------------------
//
// enum ssam_device_domain - SAM device domain.
// @SSAM_DOMAIN_VIRTUAL:   Virtual device.
// @SSAM_DOMAIN_SERIALHUB: Physical device connected via Surface Serial Hub.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ssam_device_domain {
    SSAM_DOMAIN_VIRTUAL   = 0x00,
    SSAM_DOMAIN_SERIALHUB = 0x01,
}

//
// enum ssam_virtual_tc - Target categories for the virtual SAM domain.
// @SSAM_VIRTUAL_TC_HUB: Device hub category.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ssam_virtual_tc {
    SSAM_VIRTUAL_TC_HUB = 0x00,
}

//
// struct ssam_device_uid - Unique identifier for SSAM device.
// @domain:   Domain of the device.
// @category: Target category of the device.
// @target:   Target ID of the device.
// @instance: Instance ID of the device.
// @function: Sub-function of the device. This field can be used to split a
// single SAM device into multiple virtual subdevices to separate
// different functionality of that device and allow one driver per
// such functionality.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssam_device_uid {
    pub domain: u8,
    pub category: u8,
    pub target: u8,
    pub instance: u8,
    pub function: u8,
}

//
// Special values for device matching.
//
// These values are intended to be used with SSAM_DEVICE(), SSAM_VDEV(), and
// SSAM_SDEV() exclusively. Specifically, they are used to initialize the
// match_flags member of the device ID structure. Do not use them directly
// with struct ssam_device_id or struct ssam_device_uid.
//
pub const SSAM_SSH_TID_ANY: c_uint = 0xffff;
pub const SSAM_SSH_IID_ANY: c_uint = 0xffff;
pub const SSAM_SSH_FUN_ANY: c_uint = 0xffff;
//
// SSAM_DEVICE() - Initialize a &struct ssam_device_id with the given
// parameters.
// @d:   Domain of the device.
// @cat: Target category of the device.
// @tid: Target ID of the device.
// @iid: Instance ID of the device.
// @fun: Sub-function of the device.
//
// Initializes a &struct ssam_device_id with the given parameters. See &struct
// ssam_device_uid for details regarding the parameters. The special values
// %SSAM_SSH_TID_ANY, %SSAM_SSH_IID_ANY, and %SSAM_SSH_FUN_ANY can be used to specify that
// matching should ignore target ID, instance ID, and/or sub-function,
// respectively. This macro initializes the ``match_flags`` field based on the
// given parameters.
//
// Note: The parameters @d and @cat must be valid &u8 values, the parameters
// @tid, @iid, and @fun must be either valid &u8 values or %SSAM_SSH_TID_ANY,
// %SSAM_SSH_IID_ANY, or %SSAM_SSH_FUN_ANY, respectively. Other non-&u8 values are not
// allowed.
//

//
// SSAM_VDEV() - Initialize a &struct ssam_device_id as virtual device with
// the given parameters.
// @cat: Target category of the device.
// @tid: Target ID of the device.
// @iid: Instance ID of the device.
// @fun: Sub-function of the device.
//
// Initializes a &struct ssam_device_id with the given parameters in the
// virtual domain. See &struct ssam_device_uid for details regarding the
// parameters. The special values %SSAM_SSH_TID_ANY, %SSAM_SSH_IID_ANY, and
// %SSAM_SSH_FUN_ANY can be used to specify that matching should ignore target ID,
// instance ID, and/or sub-function, respectively. This macro initializes the
// ``match_flags`` field based on the given parameters.
//
// Note: The parameter @cat must be a valid &u8 value, the parameters @tid,
// @iid, and @fun must be either valid &u8 values or %SSAM_SSH_TID_ANY,
// %SSAM_SSH_IID_ANY, or %SSAM_SSH_FUN_ANY, respectively. Other non-&u8 values are not
// allowed.
//

//
// SSAM_SDEV() - Initialize a &struct ssam_device_id as physical SSH device
// with the given parameters.
// @cat: Target category of the device.
// @tid: Target ID of the device.
// @iid: Instance ID of the device.
// @fun: Sub-function of the device.
//
// Initializes a &struct ssam_device_id with the given parameters in the SSH
// domain. See &struct ssam_device_uid for details regarding the parameters.
// The special values %SSAM_SSH_TID_ANY, %SSAM_SSH_IID_ANY, and
// %SSAM_SSH_FUN_ANY can be used to specify that matching should ignore target
// ID, instance ID, and/or sub-function, respectively. This macro initializes
// the ``match_flags`` field based on the given parameters.
//
// Note: The parameter @cat must be a valid &u8 value, the parameters @tid,
// @iid, and @fun must be either valid &u8 values or %SSAM_SSH_TID_ANY,
// %SSAM_SSH_IID_ANY, or %SSAM_SSH_FUN_ANY, respectively. Other non-&u8 values
// are not allowed.
//

//
// enum ssam_device_flags - Flags for SSAM client devices.
// @SSAM_DEVICE_HOT_REMOVED_BIT:
// The device has been hot-removed. Further communication with it may time
// out and should be avoided.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ssam_device_flags {
    SSAM_DEVICE_HOT_REMOVED_BIT = 0,
}

//
// struct ssam_device - SSAM client device.
// @dev:   Driver model representation of the device.
// @ctrl:  SSAM controller managing this device.
// @uid:   UID identifying the device.
// @flags: Device state flags, see &enum ssam_device_flags.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssam_device {
    pub dev: device,
    pub ctrl: *mut ssam_controller,
    pub uid: ssam_device_uid,
    pub flags: c_ulong,
}

//
// struct ssam_device_driver - SSAM client device driver.
// @driver:      Base driver model structure.
// @match_table: Match table specifying which devices the driver should bind to.
// @probe:       Called when the driver is being bound to a device.
// @remove:      Called when the driver is being unbound from the device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssam_device_driver {
    pub driver: device_driver,
    pub match_table: *const ssam_device_id,
    pub sdev): *mut *mut int (probe)(struct ssam_device,
    pub sdev): *mut *mut void (remove)(struct ssam_device,
}

//
// is_ssam_device() - Check if the given device is a SSAM client device.
// @d: The device to test the type of.
//
// Return: Returns %true if the specified device is of type &struct
// ssam_device, i.e. the device type points to %ssam_device_type, and %false
// otherwise.
//

//
// to_ssam_device() - Casts the given device to a SSAM client device.
// @d: The device to cast.
//
// Casts the given &struct device to a &struct ssam_device. The caller has to
// ensure that the given device is actually enclosed in a &struct ssam_device,
// e.g. by calling is_ssam_device().
//
// Return: Returns a pointer to the &struct ssam_device wrapping the given
// device @d.
//

//
// to_ssam_device_driver() - Casts the given device driver to a SSAM client
// device driver.
// @d: The driver to cast.
//
// Casts the given &struct device_driver to a &struct ssam_device_driver. The
// caller has to ensure that the given driver is actually enclosed in a
// &struct ssam_device_driver.
//
// Return: Returns the pointer to the &struct ssam_device_driver wrapping the
// given device driver @d.
//

extern "C" {
    pub fn ssam_device_add(sdev: *mut ssam_device) -> c_int;
}
extern "C" {
    pub fn ssam_device_remove(sdev: *mut ssam_device);
}
//
// ssam_device_mark_hot_removed() - Mark the given device as hot-removed.
// @sdev: The device to mark as hot-removed.
//
// Mark the device as having been hot-removed. This signals drivers using the
// device that communication with the device should be avoided and may lead to
// timeouts.
//
// ssam_device_is_hot_removed() - Check if the given device has been
// hot-removed.
// @sdev: The device to check.
//
// Checks if the given device has been marked as hot-removed. See
// ssam_device_mark_hot_removed() for more details.
//
// Return: Returns ``true`` if the device has been marked as hot-removed.
//
extern "C" {
    pub fn test_bit(_arg: SSAM_DEVICE_HOT_REMOVED_BIT, _arg: &sdev->flags) -> return;
}
//
// ssam_device_get() - Increment reference count of SSAM client device.
// @sdev: The device to increment the reference count of.
//
// Increments the reference count of the given SSAM client device by
// incrementing the reference count of the enclosed &struct device via
// get_device().
//
// See ssam_device_put() for the counter-part of this function.
//
// Return: Returns the device provided as input.
//
// ssam_device_put() - Decrement reference count of SSAM client device.
// @sdev: The device to decrement the reference count of.
//
// Decrements the reference count of the given SSAM client device by
// decrementing the reference count of the enclosed &struct device via
// put_device().
//
// See ssam_device_get() for the counter-part of this function.
//
// ssam_device_get_drvdata() - Get driver-data of SSAM client device.
// @sdev: The device to get the driver-data from.
//
// Return: Returns the driver-data of the given device, previously set via
// ssam_device_set_drvdata().
//
extern "C" {
    pub fn dev_get_drvdata(_arg: &sdev->dev) -> return;
}
//
// ssam_device_set_drvdata() - Set driver-data of SSAM client device.
// @sdev: The device to set the driver-data of.
// @data: The data to set the device's driver-data pointer to.
//
extern "C" {
    pub fn __ssam_device_driver_register(d: *mut ssam_device_driver, o: *mut module) -> c_int;
}
extern "C" {
    pub fn ssam_device_driver_unregister(d: *mut ssam_device_driver);
}
//
// ssam_device_driver_register() - Register a SSAM client device driver.
// @drv: The driver to register.
//

//
// module_ssam_device_driver() - Helper macro for SSAM device driver
// registration.
// @drv: The driver managed by this module.
//
// Helper macro to register a SSAM device driver via module_init() and
// module_exit(). This macro may only be used once per module and replaces the
// aforementioned definitions.
//

// -- Helpers for controller and hub devices. -------------------------------

extern "C" {
    pub fn ssam_remove_clients(dev: *mut device);
}

//
// ssam_register_clients() - Register all client devices defined under the
// given parent device.
// @dev: The parent device under which clients should be registered.
// @ctrl: The controller with which client should be registered.
//
// Register all clients that have via firmware nodes been defined as children
// of the given (parent) device. The respective child firmware nodes will be
// associated with the correspondingly created child devices.
//
// The given controller will be used to instantiate the new devices. See
// ssam_device_add() for details.
//
// Return: Returns zero on success, nonzero on failure.
//
extern "C" {
    pub fn __ssam_register_clients(_arg: dev, _arg: ctrl, _arg: dev_fwnode(dev)) -> return;
}
//
// ssam_device_register_clients() - Register all client devices defined under
// the given SSAM parent device.
// @sdev: The parent device under which clients should be registered.
//
// Register all clients that have via firmware nodes been defined as children
// of the given (parent) device. The respective child firmware nodes will be
// associated with the correspondingly created child devices.
//
// The controller used by the parent device will be used to instantiate the new
// devices. See ssam_device_add() for details.
//
// Return: Returns zero on success, nonzero on failure.
//
extern "C" {
    pub fn ssam_register_clients(_arg: &sdev->dev, _arg: sdev->ctrl) -> return;
}
// -- Helpers for client-device requests. -----------------------------------
//
// SSAM_DEFINE_SYNC_REQUEST_CL_N() - Define synchronous client-device SAM
// request function with neither argument nor return value.
// @name: Name of the generated function.
// @spec: Specification (&struct ssam_request_spec_md) defining the request.
//
// Defines a function executing the synchronous SAM request specified by
// @spec, with the request having neither argument nor return value. Device
// specifying parameters are not hard-coded, but instead are provided via the
// client device, specifically its UID, supplied when calling this function.
// The generated function takes care of setting up the request struct, buffer
// allocation, as well as execution of the request itself, returning once the
// request has been fully completed. The required transport buffer will be
// allocated on the stack.
//
// The generated function is defined as ``static int name(struct ssam_device
// *sdev)``, returning the status of the request, which is zero on success and
// negative on failure. The ``sdev`` parameter specifies both the target
// device of the request and by association the controller via which the
// request is sent.
//
// Refer to ssam_request_do_sync_onstack() for more details on the behavior of
// the generated function.
//

//
// SSAM_DEFINE_SYNC_REQUEST_CL_W() - Define synchronous client-device SAM
// request function with argument.
// @name:  Name of the generated function.
// @atype: Type of the request's argument.
// @spec:  Specification (&struct ssam_request_spec_md) defining the request.
//
// Defines a function executing the synchronous SAM request specified by
// @spec, with the request taking an argument of type @atype and having no
// return value. Device specifying parameters are not hard-coded, but instead
// are provided via the client device, specifically its UID, supplied when
// calling this function. The generated function takes care of setting up the
// request struct, buffer allocation, as well as execution of the request
// itself, returning once the request has been fully completed. The required
// transport buffer will be allocated on the stack.
//
// The generated function is defined as ``static int name(struct ssam_device
// *sdev, const atype *arg)``, returning the status of the request, which is
// zero on success and negative on failure. The ``sdev`` parameter specifies
// both the target device of the request and by association the controller via
// which the request is sent. The request's argument is specified via the
// ``arg`` pointer.
//
// Refer to ssam_request_do_sync_onstack() for more details on the behavior of
// the generated function.
//

//
// SSAM_DEFINE_SYNC_REQUEST_CL_R() - Define synchronous client-device SAM
// request function with return value.
// @name:  Name of the generated function.
// @rtype: Type of the request's return value.
// @spec:  Specification (&struct ssam_request_spec_md) defining the request.
//
// Defines a function executing the synchronous SAM request specified by
// @spec, with the request taking no argument but having a return value of
// type @rtype. Device specifying parameters are not hard-coded, but instead
// are provided via the client device, specifically its UID, supplied when
// calling this function. The generated function takes care of setting up the
// request struct, buffer allocation, as well as execution of the request
// itself, returning once the request has been fully completed. The required
// transport buffer will be allocated on the stack.
//
// The generated function is defined as ``static int name(struct ssam_device
// *sdev, rtype *ret)``, returning the status of the request, which is zero on
// success and negative on failure. The ``sdev`` parameter specifies both the
// target device of the request and by association the controller via which
// the request is sent. The request's return value is written to the memory
// pointed to by the ``ret`` parameter.
//
// Refer to ssam_request_do_sync_onstack() for more details on the behavior of
// the generated function.
//

//
// SSAM_DEFINE_SYNC_REQUEST_CL_WR() - Define synchronous client-device SAM
// request function with argument and return value.
// @name:  Name of the generated function.
// @atype: Type of the request's argument.
// @rtype: Type of the request's return value.
// @spec:  Specification (&struct ssam_request_spec_md) defining the request.
//
// Defines a function executing the synchronous SAM request specified by @spec,
// with the request taking an argument of type @atype and having a return value
// of type @rtype. Device specifying parameters are not hard-coded, but instead
// are provided via the client device, specifically its UID, supplied when
// calling this function. The generated function takes care of setting up the
// request struct, buffer allocation, as well as execution of the request
// itself, returning once the request has been fully completed. The required
// transport buffer will be allocated on the stack.
//
// The generated function is defined as ``static int name(struct ssam_device
// *sdev, const atype *arg, rtype *ret)``, returning the status of the request,
// which is zero on success and negative on failure. The ``sdev`` parameter
// specifies both the target device of the request and by association the
// controller via which the request is sent. The request's argument is
// specified via the ``arg`` pointer. The request's return value is written to
// the memory pointed to by the ``ret`` parameter.
//
// Refer to ssam_request_do_sync_onstack() for more details on the behavior of
// the generated function.
//

// -- Helpers for client-device notifiers. ----------------------------------
//
// ssam_device_notifier_register() - Register an event notifier for the
// specified client device.
// @sdev: The device the notifier should be registered on.
// @n:    The event notifier to register.
//
// Register an event notifier. Increment the usage counter of the associated
// SAM event if the notifier is not marked as an observer. If the event is not
// marked as an observer and is currently not enabled, it will be enabled
// during this call. If the notifier is marked as an observer, no attempt will
// be made at enabling any event and no reference count will be modified.
//
// Notifiers marked as observers do not need to be associated with one specific
// event, i.e. as long as no event matching is performed, only the event target
// category needs to be set.
//
// Return: Returns zero on success, %-ENOSPC if there have already been
// %INT_MAX notifiers for the event ID/type associated with the notifier block
// registered, %-ENOMEM if the corresponding event entry could not be
// allocated, %-ENODEV if the device is marked as hot-removed. If this is the
// first time that a notifier block is registered for the specific associated
// event, returns the status of the event-enable EC-command.
//
// Note that this check does not provide any guarantees whatsoever as
// hot-removal could happen at any point and we can't protect against
// it. Nevertheless, if we can detect hot-removal, bail early to avoid
// communication timeouts.
//
extern "C" {
    pub fn ssam_notifier_register(_arg: sdev->ctrl, _arg: n) -> return;
}
//
// ssam_device_notifier_unregister() - Unregister an event notifier for the
// specified client device.
// @sdev: The device the notifier has been registered on.
// @n:    The event notifier to unregister.
//
// Unregister an event notifier. Decrement the usage counter of the associated
// SAM event if the notifier is not marked as an observer. If the usage counter
// reaches zero, the event will be disabled.
//
// In case the device has been marked as hot-removed, the event will not be
// disabled on the EC, as in those cases any attempt at doing so may time out.
//
// Return: Returns zero on success, %-ENOENT if the given notifier block has
// not been registered on the controller. If the given notifier block was the
// last one associated with its specific event, returns the status of the
// event-disable EC-command.
//
