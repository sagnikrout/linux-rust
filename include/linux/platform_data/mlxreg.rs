//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/mlxreg.h
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


// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
//
// Copyright (C) 2017-2020 Mellanox Technologies Ltd.
//
pub const MLXREG_CORE_LABEL_MAX_SIZE: c_int = 32;

//
// enum mlxreg_wdt_type - type of HW watchdog
//
// @MLX_WDT_TYPE1: HW watchdog implementation in old systems.
// @MLX_WDT_TYPE2: All new systems have TYPE2 HW watchdog.
// @MLX_WDT_TYPE3: HW watchdog that can exist on all systems with new CPLD.
// TYPE3 is selected by WD capability bit.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlxreg_wdt_type {
    MLX_WDT_TYPE1,
    MLX_WDT_TYPE2,
    MLX_WDT_TYPE3,
}

//
// enum mlxreg_hotplug_kind - kind of hotplug entry
//
// @MLXREG_HOTPLUG_DEVICE_NA: do not care;
// @MLXREG_HOTPLUG_LC_PRESENT: entry for line card presence in/out events;
// @MLXREG_HOTPLUG_LC_VERIFIED: entry for line card verification status events
// coming after line card security signature validation;
// @MLXREG_HOTPLUG_LC_POWERED: entry for line card power on/off events;
// @MLXREG_HOTPLUG_LC_SYNCED: entry for line card synchronization events, coming
// after hardware-firmware synchronization handshake;
// @MLXREG_HOTPLUG_LC_READY: entry for line card ready events, indicating line card
// PHYs ready / unready state;
// @MLXREG_HOTPLUG_LC_ACTIVE: entry for line card active events, indicating firmware
// availability / unavailability for the ports on line card;
// @MLXREG_HOTPLUG_LC_THERMAL: entry for line card thermal shutdown events, positive
// event indicates that system should power off the line
// card for which this event has been received;
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlxreg_hotplug_kind {
    MLXREG_HOTPLUG_DEVICE_NA = 0,
    MLXREG_HOTPLUG_LC_PRESENT = 1,
    MLXREG_HOTPLUG_LC_VERIFIED = 2,
    MLXREG_HOTPLUG_LC_POWERED = 3,
    MLXREG_HOTPLUG_LC_SYNCED = 4,
    MLXREG_HOTPLUG_LC_READY = 5,
    MLXREG_HOTPLUG_LC_ACTIVE = 6,
    MLXREG_HOTPLUG_LC_THERMAL = 7,
}

//
// enum mlxreg_hotplug_device_action - hotplug device action required for
// driver's connectivity
//
// @MLXREG_HOTPLUG_DEVICE_DEFAULT_ACTION: probe device for 'on' event, remove
// for 'off' event;
// @MLXREG_HOTPLUG_DEVICE_PLATFORM_ACTION: probe platform device for 'on'
// event, remove for 'off' event;
// @MLXREG_HOTPLUG_DEVICE_NO_ACTION: no connectivity action is required;
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlxreg_hotplug_device_action {
    MLXREG_HOTPLUG_DEVICE_DEFAULT_ACTION = 0,
    MLXREG_HOTPLUG_DEVICE_PLATFORM_ACTION = 1,
    MLXREG_HOTPLUG_DEVICE_NO_ACTION = 2,
}

//
// struct mlxreg_core_hotplug_notifier - hotplug notifier block:
//
// @identity: notifier identity name;
// @handle: user handle to be passed by user handler function;
// @user_handler: user handler function associated with the event;
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxreg_core_hotplug_notifier {
    pub identity: [c_char; MLXREG_CORE_LABEL_MAX_SIZE],
    pub handle: *mut c_void,
    pub action): *mut *mut *mut int (user_handler)(void handle, enum mlxreg_hotplug_kind kind, u8,
}

//
// struct mlxreg_hotplug_device - I2C device data:
//
// @adapter: I2C device adapter;
// @client: I2C device client;
// @brdinfo: device board information;
// @nr: I2C device adapter number, to which device is to be attached;
// @pdev: platform device, if device is instantiated as a platform device;
// @action: action to be performed upon event receiving;
// @handle: user handle to be passed by user handler function;
// @user_handler: user handler function associated with the event;
// @notifier: pointer to event notifier block;
//
// Structure represents I2C hotplug device static data (board topology) and
// dynamic data (related kernel objects handles).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxreg_hotplug_device {
    pub adapter: *mut i2c_adapter,
    pub client: *mut i2c_client,
    pub brdinfo: *mut i2c_board_info,
    pub nr: c_int,
    pub pdev: *mut platform_device,
    pub action: mlxreg_hotplug_device_action,
    pub handle: *mut c_void,
    pub action): *mut *mut *mut int (user_handler)(void handle, enum mlxreg_hotplug_kind kind, u8,
    pub notifier: *mut mlxreg_core_hotplug_notifier,
}

//
// struct mlxreg_core_data - attributes control data:
//
// @label: attribute label;
// @reg: attribute register;
// @mask: attribute access mask;
// @bit: attribute effective bit;
// @capability: attribute capability register;
// @reg_prsnt: attribute presence register;
// @reg_sync: attribute synch register;
// @reg_pwr: attribute power register;
// @reg_ena: attribute enable register;
// @mode: access mode;
// @np: pointer to node platform associated with attribute;
// @hpdev: hotplug device data;
// @notifier: pointer to event notifier block;
// @health_cntr: dynamic device health indication counter;
// @attached: true if device has been attached after good health indication;
// @regnum: number of registers occupied by multi-register attribute;
// @slot: slot number, at which device is located;
// @secured: if set indicates that entry access is secured;
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxreg_core_data {
    pub label: [c_char; MLXREG_CORE_LABEL_MAX_SIZE],
    pub reg: u32,
    pub mask: u32,
    pub bit: u32,
    pub capability: u32,
    pub reg_prsnt: u32,
    pub reg_sync: u32,
    pub reg_pwr: u32,
    pub reg_ena: u32,
    pub mode: umode_t,
    pub np: *mut device_node,
    pub hpdev: mlxreg_hotplug_device,
    pub notifier: *mut mlxreg_core_hotplug_notifier,
    pub health_cntr: u32,
    pub attached: bool,
    pub regnum: u8,
    pub slot: u8,
    pub secured: u8,
}

//
// struct mlxreg_core_item - same type components controlled by the driver:
//
// @data: component data;
// @kind: kind of hotplug attribute;
// @aggr_mask: group aggregation mask;
// @reg: group interrupt status register;
// @mask: group interrupt mask;
// @capability: group capability register;
// @cache: last status value for elements fro the same group;
// @count: number of available elements in the group;
// @ind: element's index inside the group;
// @inversed: if 0: 0 for signal status is OK, if 1 - 1 is OK;
// @health: true if device has health indication, false in other case;
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxreg_core_item {
    pub data: *mut mlxreg_core_data,
    pub kind: mlxreg_hotplug_kind,
    pub aggr_mask: u32,
    pub reg: u32,
    pub mask: u32,
    pub capability: u32,
    pub cache: u32,
    pub count: u8,
    pub ind: u8,
    pub inversed: u8,
    pub health: u8,
}

//
// struct mlxreg_core_platform_data - platform data:
//
// @data: instance private data;
// @regmap: register map of parent device;
// @counter: number of instances;
// @features: supported features of device;
// @version: implementation version;
// @identity: device identity name;
// @capability: device capability register;
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxreg_core_platform_data {
    pub data: *mut mlxreg_core_data,
    pub regmap: *mut c_void,
    pub counter: c_int,
    pub features: u32,
    pub version: u32,
    pub identity: [c_char; MLXREG_CORE_LABEL_MAX_SIZE],
    pub capability: u32,
}

//
// struct mlxreg_core_hotplug_platform_data - hotplug platform data:
//
// @items: same type components with the hotplug capability;
// @irq: platform interrupt number;
// @regmap: register map of parent device;
// @count: number of the components with the hotplug capability;
// @cell: location of top aggregation interrupt register;
// @mask: top aggregation interrupt common mask;
// @cell_low: location of low aggregation interrupt register;
// @mask_low: low aggregation interrupt common mask;
// @deferred_nr: I2C adapter number must be exist prior probing execution;
// @shift_nr: I2C adapter numbers must be incremented by this value;
// @addr: mapped resource address;
// @handle: handle to be passed by callback;
// @completion_notify: callback to notify when platform driver probing is done;
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxreg_core_hotplug_platform_data {
    pub items: *mut mlxreg_core_item,
    pub irq: c_int,
    pub regmap: *mut c_void,
    pub count: c_int,
    pub cell: u32,
    pub mask: u32,
    pub cell_low: u32,
    pub mask_low: u32,
    pub deferred_nr: c_int,
    pub shift_nr: c_int,
    pub addr: *mut void __iomem,
    pub handle: *mut c_void,
    pub id): *mut *mut *mut int (completion_notify)(void handle, int,
}
