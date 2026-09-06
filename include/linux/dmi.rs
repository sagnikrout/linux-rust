//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/dmi.h
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

// enum dmi_field is in mod_devicetable.h
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmi_device_type {
    DMI_DEV_TYPE_ANY = 0,
    DMI_DEV_TYPE_OTHER,
    DMI_DEV_TYPE_UNKNOWN,
    DMI_DEV_TYPE_VIDEO,
    DMI_DEV_TYPE_SCSI,
    DMI_DEV_TYPE_ETHERNET,
    DMI_DEV_TYPE_TOKENRING,
    DMI_DEV_TYPE_SOUND,
    DMI_DEV_TYPE_PATA,
    DMI_DEV_TYPE_SATA,
    DMI_DEV_TYPE_SAS,
    DMI_DEV_TYPE_IPMI = -1,
    DMI_DEV_TYPE_OEM_STRING = -2,
    DMI_DEV_TYPE_DEV_ONBOARD = -3,
    DMI_DEV_TYPE_DEV_SLOT = -4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmi_entry_type {
    DMI_ENTRY_BIOS = 0,
    DMI_ENTRY_SYSTEM,
    DMI_ENTRY_BASEBOARD,
    DMI_ENTRY_CHASSIS,
    DMI_ENTRY_PROCESSOR,
    DMI_ENTRY_MEM_CONTROLLER,
    DMI_ENTRY_MEM_MODULE,
    DMI_ENTRY_CACHE,
    DMI_ENTRY_PORT_CONNECTOR,
    DMI_ENTRY_SYSTEM_SLOT,
    DMI_ENTRY_ONBOARD_DEVICE,
    DMI_ENTRY_OEMSTRINGS,
    DMI_ENTRY_SYSCONF,
    DMI_ENTRY_BIOS_LANG,
    DMI_ENTRY_GROUP_ASSOC,
    DMI_ENTRY_SYSTEM_EVENT_LOG,
    DMI_ENTRY_PHYS_MEM_ARRAY,
    DMI_ENTRY_MEM_DEVICE,
    DMI_ENTRY_32_MEM_ERROR,
    DMI_ENTRY_MEM_ARRAY_MAPPED_ADDR,
    DMI_ENTRY_MEM_DEV_MAPPED_ADDR,
    DMI_ENTRY_BUILTIN_POINTING_DEV,
    DMI_ENTRY_PORTABLE_BATTERY,
    DMI_ENTRY_SYSTEM_RESET,
    DMI_ENTRY_HW_SECURITY,
    DMI_ENTRY_SYSTEM_POWER_CONTROLS,
    DMI_ENTRY_VOLTAGE_PROBE,
    DMI_ENTRY_COOLING_DEV,
    DMI_ENTRY_TEMP_PROBE,
    DMI_ENTRY_ELECTRICAL_CURRENT_PROBE,
    DMI_ENTRY_OOB_REMOTE_ACCESS,
    DMI_ENTRY_BIS_ENTRY,
    DMI_ENTRY_SYSTEM_BOOT,
    DMI_ENTRY_64_MEM_ERROR,
    DMI_ENTRY_MGMT_DEV,
    DMI_ENTRY_MGMT_DEV_COMPONENT,
    DMI_ENTRY_MGMT_DEV_THRES,
    DMI_ENTRY_MEM_CHANNEL,
    DMI_ENTRY_IPMI_DEV,
    DMI_ENTRY_SYS_POWER_SUPPLY,
    DMI_ENTRY_ADDITIONAL,
    DMI_ENTRY_ONBOARD_DEV_EXT,
    DMI_ENTRY_MGMT_CONTROLLER_HOST,
    DMI_ENTRY_TPM_DEVICE,
    DMI_ENTRY_PROCESSOR_ADDITIONAL,
    DMI_ENTRY_FIRMWARE_INVENTORY,
    DMI_ENTRY_STRING_PROPERTY,
    DMI_ENTRY_INACTIVE = 126,
    DMI_ENTRY_END_OF_TABLE = 127,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmi_header {
    pub type: u8,
    pub length: u8,
    pub handle: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmi_device {
    pub list: list_head,
    pub type: c_int,
    pub name: *const c_char,
    pub /: *mut *mut *mut void device_data; / Type specific data,
}

pub const DMI_A_INFO_ENT_MIN_SIZE: c_uint = 0x6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmi_a_info_entry {
    pub length: u8,
    pub handle: u16,
    pub offset: u8,
    pub str_num: u8,
    pub value: [u8; ],
    pub __packed: },
pub const DMI_A_INFO_MIN_SIZE: c_uint = 0xB;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmi_a_info {
    pub header: dmi_header,
    pub count: u8,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmi_dev_onboard {
    pub dev: dmi_device,
    pub instance: c_int,
    pub segment: c_int,
    pub bus: c_int,
    pub devfn: c_int,
}

extern "C" {
    pub fn dmi_check_system(list: *const dmi_system_id) -> c_int;
}
extern "C" {
    pub fn dmi_get_system_info(field: c_int) -> *const c_char;
}
extern "C" {
    pub fn dmi_setup();
}
extern "C" {
    pub fn dmi_get_date(field: c_int, yearp: *mut c_int, monthp: *mut c_int, dayp: *mut c_int) -> bool;
}
extern "C" {
    pub fn dmi_get_bios_year() -> c_int;
}
extern "C" {
    pub fn dmi_name_in_vendors(str: *const c_char) -> c_int;
}
extern "C" {
    pub fn dmi_name_in_serial(str: *const c_char) -> c_int;
}
extern "C" {
    pub fn dmi_match(f: dmi_field, str: *const c_char) -> bool;
}
extern "C" {
    pub fn dmi_memdev_name(handle: u16, bank: *const c_char, device: *const c_char);
}
extern "C" {
    pub fn dmi_memdev_size(handle: u16) -> u64;
}
extern "C" {
    pub fn dmi_memdev_type(handle: u16) -> u8;
}
extern "C" {
    pub fn dmi_memdev_handle(slot: c_int) -> u16;
}

// yearp = 0;
// monthp = 0;
// dayp = 0;
pub const dmi_available: c_int = 0;

