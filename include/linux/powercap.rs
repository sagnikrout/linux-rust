//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/powercap.h
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
// powercap.h: Data types and headers for sysfs power capping interface
// Copyright (c) 2013, Intel Corporation.
//

//
// A power cap class device can contain multiple powercap control_types.
// Each control_type can have multiple power zones, which can be independently
// controlled. Each power zone can have one or more constraints.
//
// struct powercap_control_type_ops - Define control type callbacks
// @set_enable:		Enable/Disable whole control type.
// Default is enabled. But this callback allows all zones
// to be in disable state and remove any applied power
// limits. If disabled power zone can only be monitored
// not controlled.
// @get_enable:		get Enable/Disable status.
// @release:		Callback to inform that last reference to this
// control type is closed. So it is safe to free data
// structure associated with this control type.
// This callback is mandatory if the client own memory
// for the control type.
//
// This structure defines control type callbacks to be implemented by client
// drivers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct powercap_control_type_ops {
    pub mode): *mut *mut *mut int (set_enable) (struct powercap_control_type , bool,
    pub mode): *mut *mut *mut int (get_enable) (struct powercap_control_type , bool,
    pub ): *mut *mut int (release) (struct powercap_control_type,
}

//
// struct powercap_control_type - Defines a powercap control_type
// @dev:		device for this control_type
// @idr:		idr to have unique id for its child
// @nr_zones:		counter for number of zones of this type
// @ops:		Pointer to callback struct
// @lock:		mutex for control type
// @allocated:		This is possible that client owns the memory
// used by this structure. In this case
// this flag is set to false by framework to
// prevent deallocation during release process.
// Otherwise this flag is set to true.
// @node:		linked-list node
//
// Defines powercap control_type. This acts as a container for power
// zones, which use same method to control power. E.g. RAPL, RAPL-PCI etc.
// All fields are private and should not be used by client drivers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct powercap_control_type {
    pub dev: device,
    pub idr: idr,
    pub nr_zones: c_int,
    pub ops: *const powercap_control_type_ops,
    pub lock: mutex,
    pub allocated: bool,
    pub node: list_head,
}

//
// struct powercap_zone_ops - Define power zone callbacks
// @get_max_energy_range_uj:	Get maximum range of energy counter in
// micro-joules.
// @get_energy_uj:		Get current energy counter in micro-joules.
// @reset_energy_uj:		Reset micro-joules energy counter.
// @get_max_power_range_uw:	Get maximum range of power counter in
// micro-watts.
// @get_power_uw:		Get current power counter in micro-watts.
// @set_enable:			Enable/Disable power zone controls.
// Default is enabled.
// @get_enable:			get Enable/Disable status.
// @release:			Callback to inform that last reference to this
// control type is closed. So it is safe to free
// data structure associated with this
// control type. Mandatory, if client driver owns
// the power_zone memory.
//
// This structure defines zone callbacks to be implemented by client drivers.
// Client drives can define both energy and power related callbacks. But at
// the least one type (either power or energy) is mandatory. Client drivers
// should handle mutual exclusion, if required in callbacks.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct powercap_zone_ops {
    pub ): *mut *mut *mut int (get_max_energy_range_uj) (struct powercap_zone , u64,
    pub ): *mut *mut *mut int (get_energy_uj) (struct powercap_zone , u64,
    pub ): *mut *mut int (reset_energy_uj) (struct powercap_zone,
    pub ): *mut *mut *mut int (get_max_power_range_uw) (struct powercap_zone , u64,
    pub ): *mut *mut *mut int (get_power_uw) (struct powercap_zone , u64,
    pub mode): *mut *mut *mut int (set_enable) (struct powercap_zone , bool,
    pub mode): *mut *mut *mut int (get_enable) (struct powercap_zone , bool,
    pub ): *mut *mut int (release) (struct powercap_zone,
}

pub const POWERCAP_ZONE_MAX_ATTRS: c_int = 6;
pub const POWERCAP_CONSTRAINTS_ATTRS: c_int = 8;
pub const MAX_CONSTRAINTS_PER_ZONE: c_int = 10;
//
// struct powercap_zone- Defines instance of a power cap zone
// @id:			Unique id
// @name:		Power zone name.
// @control_type_inst:	Control type instance for this zone.
// @ops:		Pointer to the zone operation structure.
// @dev:		Instance of a device.
// @const_id_cnt:	Number of constraint defined.
// @idr:		Instance to an idr entry for children zones.
// @parent_idr:		To remove reference from the parent idr.
// @private_data:	Private data pointer if any for this zone.
// @zone_dev_attrs:	Attributes associated with this device.
// @zone_attr_count:	Attribute count.
// @dev_zone_attr_group: Attribute group for attributes.
// @dev_attr_groups:	Attribute group store to register with device.
// @allocated:		This is possible that client owns the memory
// used by this structure. In this case
// this flag is set to false by framework to
// prevent deallocation during release process.
// Otherwise this flag is set to true.
// @constraints:	List of constraints for this zone.
//
// This defines a power zone instance. The fields of this structure are
// private, and should not be used by client drivers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct powercap_zone {
    pub id: c_int,
    pub name: *mut c_char,
    pub control_type_inst: *mut c_void,
    pub ops: *const powercap_zone_ops,
    pub dev: device,
    pub const_id_cnt: c_int,
    pub idr: idr,
    pub parent_idr: *mut idr,
    pub private_data: *mut c_void,
    pub zone_dev_attrs: *mut attribute,
    pub zone_attr_count: c_int,
    pub dev_zone_attr_group: attribute_group,
    pub /: *const *const *const attribute_group dev_attr_groups[2]; / 1 group + NULL,
    pub allocated: bool,
    pub constraints: *mut powercap_zone_constraint,
}

//
// struct powercap_zone_constraint_ops - Define constraint callbacks
// @set_power_limit_uw:		Set power limit in micro-watts.
// @get_power_limit_uw:		Get power limit in micro-watts.
// @set_time_window_us:		Set time window in micro-seconds.
// @get_time_window_us:		Get time window in micro-seconds.
// @get_max_power_uw:		Get max power allowed in micro-watts.
// @get_min_power_uw:		Get min power allowed in micro-watts.
// @get_max_time_window_us:	Get max time window allowed in micro-seconds.
// @get_min_time_window_us:	Get min time window allowed in micro-seconds.
// @get_name:			Get the name of constraint
//
// This structure is used to define the constraint callbacks for the client
// drivers. The following callbacks are mandatory and can't be NULL:
// set_power_limit_uw
// get_power_limit_uw
// set_time_window_us
// get_time_window_us
// get_name
// Client drivers should handle mutual exclusion, if required in callbacks.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct powercap_zone_constraint_ops {
    pub u64): *mut *mut *mut int (set_power_limit_uw) (struct powercap_zone , int,,
    pub ): *mut *mut *mut int (get_power_limit_uw) (struct powercap_zone , int, u64,
    pub u64): *mut *mut *mut int (set_time_window_us) (struct powercap_zone , int,,
    pub ): *mut *mut *mut int (get_time_window_us) (struct powercap_zone , int, u64,
    pub ): *mut *mut *mut int (get_max_power_uw) (struct powercap_zone , int, u64,
    pub ): *mut *mut *mut int (get_min_power_uw) (struct powercap_zone , int, u64,
    pub ): *mut *mut *mut int (get_max_time_window_us) (struct powercap_zone , int, u64,
    pub ): *mut *mut *mut int (get_min_time_window_us) (struct powercap_zone , int, u64,
    pub int): *const *const *const *const char (get_name) (struct powercap_zone ,,
}

//
// struct powercap_zone_constraint- Defines instance of a constraint
// @id:			Instance Id of this constraint.
// @power_zone:		Pointer to the power zone for this constraint.
// @ops:		Pointer to the constraint callbacks.
//
// This defines a constraint instance.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct powercap_zone_constraint {
    pub id: c_int,
    pub power_zone: *mut powercap_zone,
    pub ops: *const powercap_zone_constraint_ops,
}

// For clients to get their device pointer, may be used for dev_dbgs

//
// powercap_set_zone_data() - Set private data for a zone
// @power_zone:	A pointer to the valid zone instance.
// @pdata:	A pointer to the user private data.
//
// Allows client drivers to associate some private data to zone instance.
//
// powercap_get_zone_data() - Get private data for a zone
// @power_zone:	A pointer to the valid zone instance.
//
// Allows client drivers to get private data associate with a zone,
// using call to powercap_set_zone_data.
//
// powercap_register_control_type() - Register a control_type with framework
// @control_type:	Pointer to client allocated memory for the control type
// structure storage. If this is NULL, powercap framework
// will allocate memory and own it.
// Advantage of this parameter is that client can embed
// this data in its data structures and allocate in a
// single call, preventing multiple allocations.
// @name:		The Name of this control_type, which will be shown
// in the sysfs Interface.
// @ops:			Callbacks for control type. This parameter is optional.
//
// Used to create a control_type with the power capping class. Here control_type
// can represent a type of technology, which can control a range of power zones.
// For example a control_type can be RAPL (Running Average Power Limit)
// Intel® 64 and IA-32 Processor Architectures. The name can be any string
// which must be unique, otherwise this function returns NULL.
// A pointer to the control_type instance is returned on success.
//
// powercap_unregister_control_type() - Unregister a control_type from framework
// @instance:	A pointer to the valid control_type instance.
//
// Used to unregister a control_type with the power capping class.
// All power zones registered under this control type have to be unregistered
// before calling this function, or it will fail with an error code.
//
extern "C" {
    pub fn powercap_unregister_control_type(instance: *mut powercap_control_type) -> c_int;
}
// Zone register/unregister API
//
// powercap_register_zone() - Register a power zone
// @power_zone:	Pointer to client allocated memory for the power zone structure
// storage. If this is NULL, powercap framework will allocate
// memory and own it. Advantage of this parameter is that client
// can embed this data in its data structures and allocate in a
// single call, preventing multiple allocations.
// @control_type: A control_type instance under which this zone operates.
// @name:	A name for this zone.
// @parent:	A pointer to the parent power zone instance if any or NULL
// @ops:		Pointer to zone operation callback structure.
// @nr_constraints: Number of constraints for this zone
// @const_ops:	Pointer to constraint callback structure
//
// Register a power zone under a given control type. A power zone must register
// a pointer to a structure representing zone callbacks.
// A power zone can be located under a parent power zone, in which case @parent
// should point to it.  Otherwise, if @parent is NULL, the new power zone will
// be located directly under the given control type
// For each power zone there may be a number of constraints that appear in the
// sysfs under that zone as attributes with unique numeric IDs.
// Returns pointer to the power_zone on success.
//
// powercap_unregister_zone() - Unregister a zone device
// @control_type:	A pointer to the valid instance of a control_type.
// @power_zone:	A pointer to the valid zone instance for a control_type
//
// Used to unregister a zone device for a control_type.  Caller should
// make sure that children for this zone are unregistered first.
//
