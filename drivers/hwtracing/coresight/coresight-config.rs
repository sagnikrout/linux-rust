//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hwtracing/coresight/coresight-config.h
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
// Copyright (c) 2020 Linaro Limited, All rights reserved.
// Author: Mike Leach <mike.leach@linaro.org>
//

// CoreSight Configuration Management - component and system wide configuration
//
// Register type flags for register value descriptor:
// describe how the value is interpreted, and handled.
//
pub const CS_CFG_REG_TYPE_STD: c_uint = 0x80	/* reg is standard reg */;
pub const CS_CFG_REG_TYPE_RESOURCE: c_uint = 0x40	/* reg is a resource */;
pub const CS_CFG_REG_TYPE_VAL_PARAM: c_uint = 0x08	/* reg value uses param */;
pub const CS_CFG_REG_TYPE_VAL_MASK: c_uint = 0x04	/* reg value bit masked */;
pub const CS_CFG_REG_TYPE_VAL_64BIT: c_uint = 0x02	/* reg value 64 bit */;
pub const CS_CFG_REG_TYPE_VAL_SAVE: c_uint = 0x01	/* reg value save on disable */;
//
// flags defining what device class a feature will match to when processing a
// system configuration - used by config data and devices.
//
pub const CS_CFG_MATCH_CLASS_SRC_ALL: c_uint = 0x0001	/* match any source */;
pub const CS_CFG_MATCH_CLASS_SRC_ETM4: c_uint = 0x0002	/* match any ETMv4 device */;
// flags defining device instance matching - used in config match desc data.
pub const CS_CFG_MATCH_INST_ANY: c_uint = 0x80000000 /* any instance of a class */;
//
// Limit number of presets in a configuration
// This is related to the number of bits (4) we use to select the preset on
// the perf command line. Preset 0 is always none selected.
// See PMU_FORMAT_ATTR(preset, "config:0-3") in coresight-etm-perf.c
//
pub const CS_CFG_CONFIG_PRESET_MAX: c_int = 15;
//
// Parameter descriptor for a device feature.
//
// @name:  Name of parameter.
// @value: Initial or default value.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cscfg_parameter_desc {
    pub name: *const c_char,
    pub value: u64,
}

//
// Representation of register value and a descriptor of register usage.
//
// Used as a descriptor in the feature descriptors.
// Used as a value in when in a feature loading into a csdev.
//
// Supports full 64 bit register value, or 32 bit value with optional mask
// value.
//
// @type:	define register usage and interpretation.
// @offset:	the address offset for register in the hardware device (per device specification).
// @hw_info:	optional hardware device type specific information. (ETM / CTI specific etc)
// @val64:	64 bit value.
// @val32:	32 bit value.
// @mask32:	32 bit mask when using 32 bit value to access device register - if mask type.
// @param_idx:	parameter index value into parameter array if param type.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cscfg_regval_desc {
    pub type:8: u32,
    pub offset:12: u32,
    pub hw_info:12: u32,
}

//
// Device feature descriptor - combination of registers and parameters to
// program a device to implement a specific complex function.
//
// @name:	 feature name.
// @description: brief description of the feature.
// @item:	 List entry.
// @match_flags: matching information if loading into a device
// @nr_params:   number of parameters used.
// @params_desc: array of parameters used.
// @nr_regs:	 number of registers used.
// @regs_desc:	 array of registers used.
// @load_owner:	 handle to load owner for dynamic load and unload of features.
// @fs_group:	 reference to configfs group for dynamic unload.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cscfg_feature_desc {
    pub name: *const c_char,
    pub description: *const c_char,
    pub item: list_head,
    pub match_flags: u32,
    pub nr_params: c_int,
    pub params_desc: *mut cscfg_parameter_desc,
    pub nr_regs: c_int,
    pub regs_desc: *mut cscfg_regval_desc,
    pub load_owner: *mut c_void,
    pub fs_group: *mut config_group,
}

//
// Configuration descriptor - describes selectable system configuration.
//
// A configuration describes device features in use, and may provide preset
// values for the parameters in those features.
//
// A single set of presets is the sum of the parameters declared by
// all the features in use - this value is @nr_total_params.
//
// @name:		name of the configuration - used for selection.
// @description:	description of the purpose of the configuration.
// @item:		list entry.
// @nr_feat_refs:	Number of features used in this configuration.
// @feat_ref_names:	references to features used in this configuration.
// @nr_presets:		Number of sets of presets supplied by this configuration.
// @nr_total_params:	Sum of all parameters declared by used features
// @presets:		Array of preset values.
// @event_ea:		Extended attribute for perf event value
// @active_cnt:		ref count for activate on this configuration.
// @load_owner:		handle to load owner for dynamic load and unload of configs.
// @fs_group:		reference to configfs group for dynamic unload.
// @available:		config can be activated - multi-stage load sets true on completion.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cscfg_config_desc {
    pub name: *const c_char,
    pub description: *const c_char,
    pub item: list_head,
    pub nr_feat_refs: c_int,
    pub feat_ref_names: *const c_char,
    pub nr_presets: c_int,
    pub nr_total_params: c_int,
    pub /: *const *const *const *const u64 presets; / nr_presets  nr_total_params,
    pub event_ea: *mut dev_ext_attribute,
    pub active_cnt: core::sync::atomic::AtomicI32,
    pub load_owner: *mut c_void,
    pub fs_group: *mut config_group,
    pub available: bool,
}

//
// config register instance - part of a loaded feature.
// maps register values to csdev driver structures
//
// @reg_desc:		value to use when setting feature on device / store for
// readback of volatile values.
// @driver_regval:	pointer to internal driver element used to set the value
// in hardware.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cscfg_regval_csdev {
    pub reg_desc: cscfg_regval_desc,
    pub driver_regval: *mut c_void,
}

//
// config parameter instance - part of a loaded feature.
//
// @feat_csdev:		parent feature
// @reg_csdev:		register value updated by this parameter.
// @current_value:	current value of parameter - may be set by user via
// sysfs, or modified during device operation.
// @val64:		true if 64 bit value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cscfg_parameter_csdev {
    pub feat_csdev: *mut cscfg_feature_csdev,
    pub reg_csdev: *mut cscfg_regval_csdev,
    pub current_value: u64,
    pub val64: bool,
}

//
// Feature instance loaded into a CoreSight device.
//
// When a feature is loaded into a specific device, then this structure holds
// the connections between the register / parameter values used and the
// internal data structures that are written when the feature is enabled.
//
// Since applying a feature modifies internal data structures in the device,
// then we have a reference to the device spinlock to protect access to these
// structures (@drv_spinlock).
//
// @feat_desc:		pointer to the static descriptor for this feature.
// @csdev:		parent CoreSight device instance.
// @node:		list entry into feature list for this device.
// @drv_spinlock:	device spinlock for access to driver register data.
// @nr_params:		number of parameters.
// @params_csdev:	current parameter values on this device
// @nr_regs:		number of registers to be programmed.
// @regs_csdev:		Programming details for the registers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cscfg_feature_csdev {
    pub feat_desc: *const cscfg_feature_desc,
    pub csdev: *mut coresight_device,
    pub node: list_head,
    pub drv_spinlock: *mut raw_spinlock_t,
    pub nr_params: c_int,
    pub params_csdev: *mut cscfg_parameter_csdev,
    pub nr_regs: c_int,
    pub regs_csdev: *mut cscfg_regval_csdev,
}

//
// Configuration instance when loaded into a CoreSight device.
//
// The instance contains references to loaded features on this device that are
// used by the configuration.
//
// @config_desc:reference to the descriptor for this configuration
// @csdev:	parent coresight device for this configuration instance.
// @enabled:	true if configuration is enabled on this device.
// @node:	list entry within the coresight device
// @nr_feat:	Number of features on this device that are used in the
// configuration.
// @feats_csdev:references to the device features to enable.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cscfg_config_csdev {
    pub config_desc: *mut cscfg_config_desc,
    pub csdev: *mut coresight_device,
    pub enabled: bool,
    pub node: list_head,
    pub nr_feat: c_int,
    pub feats_csdev: [*mut cscfg_feature_csdev; ],
}

//
// Coresight device operations.
//
// Registered coresight devices provide these operations to manage feature
// instances compatible with the device hardware and drivers
//
// @load_feat:	Pass a feature descriptor into the device and create the
// loaded feature instance (struct cscfg_feature_csdev).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cscfg_csdev_feat_ops {
    pub feat_csdev): *mut cscfg_feature_csdev,
}

// coresight config helper functions
// enable / disable config on a device - called with appropriate locks set.
extern "C" {
    pub fn cscfg_csdev_enable_config(config_csdev: *mut cscfg_config_csdev, preset: c_int) -> c_int;
}
extern "C" {
    pub fn cscfg_csdev_disable_config(config_csdev: *mut cscfg_config_csdev);
}
// reset a feature to default values
extern "C" {
    pub fn cscfg_reset_feat(feat_csdev: *mut cscfg_feature_csdev);
}
