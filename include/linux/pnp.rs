//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pnp.h
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
// Linux Plug and Play Support
// Copyright by Adam Belay <ambx1@neo.rr.com>
// Copyright (C) 2008 Hewlett-Packard Development Company, L.P.
// Bjorn Helgaas <bjorn.helgaas@hp.com>
//

pub const PNP_NAME_LEN: c_int = 50;
//
// Resource Management
//

extern "C" {
    pub fn resource_size(_arg: res) -> return;
}
extern "C" {
    pub fn pnp_resource_valid(_arg: pnp_get_resource(dev, _arg: IORESOURCE_IO, _arg: bar)) -> return;
}
extern "C" {
    pub fn pnp_resource_len(_arg: res) -> return;
}
extern "C" {
    pub fn pnp_resource_valid(_arg: pnp_get_resource(dev, _arg: IORESOURCE_MEM, _arg: bar)) -> return;
}
extern "C" {
    pub fn pnp_resource_len(_arg: res) -> return;
}
extern "C" {
    pub fn pnp_resource_valid(_arg: pnp_get_resource(dev, _arg: IORESOURCE_IRQ, _arg: bar)) -> return;
}
extern "C" {
    pub fn pnp_resource_valid(_arg: pnp_get_resource(dev, _arg: IORESOURCE_DMA, _arg: bar)) -> return;
}
//
// Device Management
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnp_card {
    pub /: *mut *mut device dev; / Driver Model device interface,
    pub /: *mut *mut unsigned char number; / used as an index, must be unique,
    pub /: *mut *mut list_head global_list; / node in global list of cards,
    pub /: *mut *mut list_head protocol_list; / node in protocol's list of cards,
    pub /: *mut *mut list_head devices; / devices attached to the card,
    pub protocol: *mut pnp_protocol,
    pub /: *mut *mut *mut pnp_id id; / contains supported EISA IDs,
    pub /: *mut *mut char name[PNP_NAME_LEN]; / contains a human-readable name,
    pub /: *mut *mut unsigned char pnpver; / Plug & Play version,
    pub /: *mut *mut unsigned char productver; / product version,
    pub /: *mut *mut unsigned int serial; / serial number,
    pub /: *mut *mut unsigned char checksum; / if zero - checksum passed,
    pub /: *mut *mut *mut proc_dir_entry procdir; / directory entry in /proc/bus/isapnp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnp_card_link {
    pub card: *mut pnp_card,
    pub driver: *mut pnp_card_driver,
    pub driver_data: *mut c_void,
    pub pm_state: pm_message_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnp_dev {
    pub /: *mut *mut device dev; / Driver Model device interface,
    pub dma_mask: u64,
    pub /: *mut *mut unsigned int number; / used as an index, must be unique,
    pub status: c_int,
    pub /: *mut *mut list_head global_list; / node in global list of devices,
    pub /: *mut *mut list_head protocol_list; / node in list of device's protocol,
    pub /: *mut *mut list_head card_list; / node in card's list of devices,
    pub /: *mut *mut list_head rdev_list; / node in cards list of requested devices,
    pub protocol: *mut pnp_protocol,
    pub /: *mut *mut *mut pnp_card card; / card the device is attached to, none if NULL,
    pub driver: *mut pnp_driver,
    pub card_link: *mut pnp_card_link,
    pub /: *mut *mut *mut pnp_id id; / supported EISA IDs,
    pub active: c_int,
    pub capabilities: c_int,
    pub num_dependent_sets: c_uint,
    pub resources: list_head,
    pub options: list_head,
    pub /: *mut *mut char name[PNP_NAME_LEN]; / contains a human-readable name,
    pub /: *mut *mut int flags; / used by protocols,
    pub /: *mut *mut *mut proc_dir_entry procent; / device entry in /proc/bus/isapnp,
    pub data: *mut c_void,
}

extern "C" {
    pub fn dev_get_drvdata(_arg: &pdev->dev) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnp_fixup {
    pub id: [c_char; 8],
    pub /: *mut *mut *mut *mut void (quirk_function) (struct pnp_dev dev); / fixup function,
}

// config parameters
pub const PNP_CONFIG_NORMAL: c_uint = 0x0001;
pub const PNP_CONFIG_FORCE: c_uint = 0x0002	/* disables validity checking */;
// capabilities
pub const PNP_READ: c_uint = 0x0001;
pub const PNP_WRITE: c_uint = 0x0002;
pub const PNP_DISABLE: c_uint = 0x0004;
pub const PNP_CONFIGURABLE: c_uint = 0x0008;
pub const PNP_REMOVABLE: c_uint = 0x0010;
pub const PNP_CONSOLE: c_uint = 0x0020;

pub const pnp_device_is_isapnp(dev): c_int = 0;

extern "C" {
    pub fn arch_pnpbios_disabled() -> bool;
}

pub const pnp_device_is_pnpbios(dev): c_int = 0;

pub const pnp_acpi_device(dev): c_int = 0;

// status
pub const PNP_READY: c_uint = 0x0000;
pub const PNP_ATTACHED: c_uint = 0x0001;
pub const PNP_BUSY: c_uint = 0x0002;
pub const PNP_FAULTY: c_uint = 0x0004;
// isapnp specific macros

//
// Driver Management
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnp_id {
    pub id: [c_char; PNP_ID_LEN],
    pub next: *mut pnp_id,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnp_driver {
    pub name: *const c_char,
    pub id_table: *const pnp_device_id,
    pub flags: c_uint,
    pub dev_id): *const *const *const int (probe) (struct pnp_dev dev, struct pnp_device_id,
    pub dev): *mut *mut void (remove) (struct pnp_dev,
    pub dev): *mut *mut void (shutdown) (struct pnp_dev,
    pub state): *mut *mut *mut int (suspend) (struct pnp_dev dev, pm_message_t,
    pub dev): *mut *mut int (resume) (struct pnp_dev,
    pub driver: device_driver,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnp_card_driver {
    pub global_list: list_head,
    pub name: *mut c_char,
    pub id_table: *const pnp_card_device_id,
    pub flags: c_uint,
    pub card_id): *const pnp_card_device_id,
    pub card): *mut *mut void (remove) (struct pnp_card_link,
    pub state): *mut *mut *mut int (suspend) (struct pnp_card_link card, pm_message_t,
    pub card): *mut *mut int (resume) (struct pnp_card_link,
    pub link: pnp_driver,
}

// pnp driver flags
pub const PNP_DRIVER_RES_DO_NOT_CHANGE: c_uint = 0x0001	/* do not change the state of the device */;
pub const PNP_DRIVER_RES_DISABLE: c_uint = 0x0003	/* ensure the device is disabled */;
//
// Protocol Management
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pnp_protocol {
    pub protocol_list: list_head,
    pub name: *mut c_char,
// resource control functions
    pub dev): *mut *mut int (get) (struct pnp_dev,
    pub dev): *mut *mut int (set) (struct pnp_dev,
    pub dev): *mut *mut int (disable) (struct pnp_dev,
// protocol specific suspend/resume
    pub dev): *mut *mut bool (can_wakeup) (struct pnp_dev,
    pub state): *mut *mut *mut int (suspend) (struct pnp_dev dev, pm_message_t,
    pub dev): *mut *mut int (resume) (struct pnp_dev,
// used by pnp layer only (look but don't touch)
    pub /: *mut *mut unsigned char number; / protocol number,
    pub /: *mut *mut device dev; / link to driver model,
    pub cards: list_head,
    pub devices: list_head,
}

// device management
extern "C" {
    pub fn pnp_device_attach(pnp_dev: *mut pnp_dev) -> c_int;
}
extern "C" {
    pub fn pnp_device_detach(pnp_dev: *mut pnp_dev);
}
// multidevice card support
extern "C" {
    pub fn pnp_release_card_device(dev: *mut pnp_dev);
}
extern "C" {
    pub fn pnp_register_card_driver(drv: *mut pnp_card_driver) -> c_int;
}
extern "C" {
    pub fn pnp_unregister_card_driver(drv: *mut pnp_card_driver);
}
// resource management
extern "C" {
    pub fn pnp_auto_config_dev(dev: *mut pnp_dev) -> c_int;
}
extern "C" {
    pub fn pnp_start_dev(dev: *mut pnp_dev) -> c_int;
}
extern "C" {
    pub fn pnp_stop_dev(dev: *mut pnp_dev) -> c_int;
}
extern "C" {
    pub fn pnp_activate_dev(dev: *mut pnp_dev) -> c_int;
}
extern "C" {
    pub fn pnp_disable_dev(dev: *mut pnp_dev) -> c_int;
}
extern "C" {
    pub fn pnp_range_reserved(start: resource_size_t, end: resource_size_t) -> c_int;
}
// protocol helpers
extern "C" {
    pub fn pnp_is_active(dev: *mut pnp_dev) -> c_int;
}
extern "C" {
    pub fn compare_pnp_id(pos: *mut pnp_id, id: *const c_char) -> c_int;
}
extern "C" {
    pub fn pnp_register_driver(drv: *mut pnp_driver) -> c_int;
}
extern "C" {
    pub fn pnp_unregister_driver(drv: *mut pnp_driver);
}
extern "C" {
    pub fn dev_is_pnp(dev: *const device) -> bool;
}

// device management
pub const pnp_platform_devices: c_int = 0;
// multidevice card support
// resource management
// protocol helpers

//
// module_pnp_driver() - Helper macro for registering a PnP driver
// @__pnp_driver: pnp_driver struct
//
// Helper macro for PnP drivers which do not do anything special in module
// init/exit. This eliminates a lot of boilerplate. Each module may only
// use this macro once, and calling it replaces module_init() and module_exit()
//

