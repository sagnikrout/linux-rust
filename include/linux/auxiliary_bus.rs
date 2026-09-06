//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/auxiliary_bus.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2019-2020 Intel Corporation
//
// Please see Documentation/driver-api/auxiliary_bus.rst for more information.
//

//
// DOC: DEVICE_LIFESPAN
//
// The registering driver is the entity that allocates memory for the
// auxiliary_device and registers it on the auxiliary bus.  It is important to
// note that, as opposed to the platform bus, the registering driver is wholly
// responsible for the management of the memory used for the device object.
//
// To be clear the memory for the auxiliary_device is freed in the release()
// callback defined by the registering driver.  The registering driver should
// only call auxiliary_device_delete() and then auxiliary_device_uninit() when
// it is done with the device.  The release() function is then automatically
// called if and when other code releases their reference to the devices.
//
// A parent object, defined in the shared header file, contains the
// auxiliary_device.  It also contains a pointer to the shared object(s), which
// also is defined in the shared header.  Both the parent object and the shared
// object(s) are allocated by the registering driver.  This layout allows the
// auxiliary_driver's registering module to perform a container_of() call to go
// from the pointer to the auxiliary_device, that is passed during the call to
// the auxiliary_driver's probe function, up to the parent object, and then
// have access to the shared object(s).
//
// The memory for the shared object(s) must have a lifespan equal to, or
// greater than, the lifespan of the memory for the auxiliary_device.  The
// auxiliary_driver should only consider that the shared object is valid as
// long as the auxiliary_device is still registered on the auxiliary bus.  It
// is up to the registering driver to manage (e.g. free or keep available) the
// memory for the shared object beyond the life of the auxiliary_device.
//
// The registering driver must unregister all auxiliary devices before its own
// driver.remove() is completed.  An easy way to ensure this is to use the
// devm_add_action_or_reset() call to register a function against the parent
// device which unregisters the auxiliary device object(s).
//
// Finally, any operations which operate on the auxiliary devices must continue
// to function (if only to return an error) after the registering driver
// unregisters the auxiliary device.
//
// struct auxiliary_device - auxiliary device object.
// @dev: Device,
// The release and parent fields of the device structure must be filled
// in
// @name: Match name found by the auxiliary device driver,
// @id: unique identitier if multiple devices of the same name are exported,
// @sysfs: embedded struct which hold all sysfs related fields,
// @sysfs.irqs: irqs xarray contains irq indices which are used by the device,
// @sysfs.lock: Synchronize irq sysfs creation,
// @sysfs.irq_dir_exists: whether "irqs" directory exists,
// @registration_data_rust: private data owned by the registering (parent)
// driver; valid for as long as the device is
// registered with the driver core,
//
// An auxiliary_device represents a part of its parent device's functionality.
// It is given a name that, combined with the registering drivers
// KBUILD_MODNAME, creates a match_name that is used for driver binding, and an
// id that combined with the match_name provide a unique name to register with
// the bus subsystem.  For example, a driver registering an auxiliary device is
// named 'foo_mod.ko' and the subdevice is named 'foo_dev'.  The match name is
// therefore 'foo_mod.foo_dev'.
//
// Registering an auxiliary_device is a three-step process.
//
// First, a 'struct auxiliary_device' needs to be defined or allocated for each
// sub-device desired.  The name, id, dev.release, and dev.parent fields of
// this structure must be filled in as follows.
//
// The 'name' field is to be given a name that is recognized by the auxiliary
// driver.  If two auxiliary_devices with the same match_name, eg
// "foo_mod.foo_dev", are registered onto the bus, they must have unique id
// values (e.g. "x" and "y") so that the registered devices names are
// "foo_mod.foo_dev.x" and "foo_mod.foo_dev.y".  If match_name + id are not
// unique, then the device_add fails and generates an error message.
//
// The auxiliary_device.dev.type.release or auxiliary_device.dev.release must
// be populated with a non-NULL pointer to successfully register the
// auxiliary_device.  This release call is where resources associated with the
// auxiliary device must be free'ed.  Because once the device is placed on the
// bus the parent driver can not tell what other code may have a reference to
// this data.
//
// The auxiliary_device.dev.parent should be set.  Typically to the registering
// drivers device.
//
// Second, call auxiliary_device_init(), which checks several aspects of the
// auxiliary_device struct and performs a device_initialize().  After this step
// completes, any error state must have a call to auxiliary_device_uninit() in
// its resolution path.
//
// The third and final step in registering an auxiliary_device is to perform a
// call to auxiliary_device_add(), which sets the name of the device and adds
// the device to the bus.
//
// .. code-block:: c
//
// #define MY_DEVICE_NAME "foo_dev"
//
// ...
//
// struct auxiliary_device *my_aux_dev = my_aux_dev_alloc(xxx);
//
// // Step 1:
// my_aux_dev->name = MY_DEVICE_NAME;
// my_aux_dev->id = my_unique_id_alloc(xxx);
// my_aux_dev->dev.release = my_aux_dev_release;
// my_aux_dev->dev.parent = my_dev;
//
// // Step 2:
// if (auxiliary_device_init(my_aux_dev))
// goto fail;
//
// // Step 3:
// if (auxiliary_device_add(my_aux_dev)) {
// auxiliary_device_uninit(my_aux_dev);
// goto fail;
// }
//
// ...
//
// Unregistering an auxiliary_device is a two-step process to mirror the
// register process.  First call auxiliary_device_delete(), then call
// auxiliary_device_uninit().
//
// .. code-block:: c
//
// auxiliary_device_delete(my_dev->my_aux_dev);
// auxiliary_device_uninit(my_dev->my_aux_dev);
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct auxiliary_device {
    pub dev: device,
    pub name: *const c_char,
    pub id: u32,
    pub irqs: xarray,
    pub /: *mut *mut mutex lock; / Synchronize irq sysfs creation,
    pub irq_dir_exists: bool,
    pub sysfs: },
    pub registration_data_rust: *mut c_void,
}

//
// struct auxiliary_driver - Definition of an auxiliary bus driver
// @probe: Called when a matching device is added to the bus.
// @remove: Called when device is removed from the bus.
// @shutdown: Called at shut-down time to quiesce the device.
// @suspend: Called to put the device to sleep mode. Usually to a power state.
// @resume: Called to bring a device from sleep mode.
// @name: Driver name.
// @driver: Core driver structure.
// @id_table: Table of devices this driver should match on the bus.
//
// Auxiliary drivers follow the standard driver model convention, where
// discovery/enumeration is handled by the core, and drivers provide probe()
// and remove() methods. They support power management and shutdown
// notifications using the standard conventions.
//
// Auxiliary drivers register themselves with the bus by calling
// auxiliary_driver_register(). The id_table contains the match_names of
// auxiliary devices that a driver can bind with.
//
// .. code-block:: c
//
// static const struct auxiliary_device_id my_auxiliary_id_table[] = {
// { .name = "foo_mod.foo_dev" },
// {},
// };
//
// MODULE_DEVICE_TABLE(auxiliary, my_auxiliary_id_table);
//
// struct auxiliary_driver my_drv = {
// .name = "myauxiliarydrv",
// .id_table = my_auxiliary_id_table,
// .probe = my_drv_probe,
// .remove = my_drv_remove
// };
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct auxiliary_driver {
    pub id): *const *const *const int (probe)(struct auxiliary_device auxdev, struct auxiliary_device_id,
    pub auxdev): *mut *mut void (remove)(struct auxiliary_device,
    pub auxdev): *mut *mut void (shutdown)(struct auxiliary_device,
    pub state): *mut *mut *mut int (suspend)(struct auxiliary_device auxdev, pm_message_t,
    pub auxdev): *mut *mut int (resume)(struct auxiliary_device,
    pub name: *const c_char,
    pub driver: device_driver,
    pub id_table: *const auxiliary_device_id,
}

extern "C" {
    pub fn dev_get_drvdata(_arg: &auxdev->dev) -> return;
}
extern "C" {
    pub fn container_of(_arg: dev, auxiliary_device: struct, _arg: dev) -> return;
}
extern "C" {
    pub fn container_of(_arg: drv, auxiliary_driver: struct, _arg: driver) -> return;
}
extern "C" {
    pub fn auxiliary_device_init(auxdev: *mut auxiliary_device) -> c_int;
}
extern "C" {
    pub fn __auxiliary_device_add(auxdev: *mut auxiliary_device, modname: *const c_char) -> c_int;
}

extern "C" {
    pub fn auxiliary_device_sysfs_irq_add(auxdev: *mut auxiliary_device, irq: c_int) -> c_int;
}

extern "C" {
    pub fn auxiliary_driver_unregister(auxdrv: *mut auxiliary_driver);
}
extern "C" {
    pub fn auxiliary_device_destroy(auxdev: *mut c_void);
}

extern "C" {
    pub fn dev_is_auxiliary(dev: *mut device) -> bool;
}
//
// module_auxiliary_driver() - Helper macro for registering an auxiliary driver
// @__auxiliary_driver: auxiliary driver struct
//
// Helper macro for auxiliary drivers which do not do anything special in
// module init/exit. This eliminates a lot of boilerplate. Each module may only
// use this macro once, and calling it replaces module_init() and module_exit()
//
// .. code-block:: c
//
// module_auxiliary_driver(my_drv);
//

