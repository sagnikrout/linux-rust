//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/platform/x86/dell/dell-wmi-sysman/dell-wmi-sysman.h
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
// Definitions for kernel modules using Dell WMI System Management Driver
//
// Copyright (c) 2020 Dell Inc.
//

pub const MAX_BUFF: c_int = 512;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct enumeration_data {
    pub attr_name_kobj: *mut kobject,
    pub display_name_language_code: [c_char; MAX_BUFF],
    pub dell_value_modifier: [c_char; MAX_BUFF],
    pub possible_values: [c_char; MAX_BUFF],
    pub attribute_name: [c_char; MAX_BUFF],
    pub default_value: [c_char; MAX_BUFF],
    pub dell_modifier: [c_char; MAX_BUFF],
    pub display_name: [c_char; MAX_BUFF],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct integer_data {
    pub attr_name_kobj: *mut kobject,
    pub display_name_language_code: [c_char; MAX_BUFF],
    pub attribute_name: [c_char; MAX_BUFF],
    pub dell_modifier: [c_char; MAX_BUFF],
    pub display_name: [c_char; MAX_BUFF],
    pub scalar_increment: c_int,
    pub default_value: c_int,
    pub min_value: c_int,
    pub max_value: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct str_data {
    pub attr_name_kobj: *mut kobject,
    pub display_name_language_code: [c_char; MAX_BUFF],
    pub attribute_name: [c_char; MAX_BUFF],
    pub display_name: [c_char; MAX_BUFF],
    pub default_value: [c_char; MAX_BUFF],
    pub dell_modifier: [c_char; MAX_BUFF],
    pub min_length: c_int,
    pub max_length: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct po_data {
    pub attr_name_kobj: *mut kobject,
    pub attribute_name: [c_char; MAX_BUFF],
    pub min_password_length: c_int,
    pub max_password_length: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_sysman_priv {
    pub current_admin_password: [c_char; MAX_BUFF],
    pub current_system_password: [c_char; MAX_BUFF],
    pub password_attr_wdev: *mut wmi_device,
    pub bios_attr_wdev: *mut wmi_device,
    pub authentication_dir_kset: *mut kset,
    pub main_dir_kset: *mut kset,
    pub class_dev: *mut device,
    pub enumeration_data: *mut enumeration_data,
    pub enumeration_instances_count: c_int,
    pub integer_data: *mut integer_data,
    pub integer_instances_count: c_int,
    pub str_data: *mut str_data,
    pub str_instances_count: c_int,
    pub po_data: *mut po_data,
    pub po_instances_count: c_int,
    pub pending_changes: bool,
    pub mutex: mutex,
}

// global structure used by multiple WMI interfaces
pub const ENUM_MIN_ELEMENTS: c_int = 8;
pub const INT_MIN_ELEMENTS: c_int = 9;
pub const STR_MIN_ELEMENTS: c_int = 8;
pub const PO_MIN_ELEMENTS: c_int = 4;

// p = '\0';							\

extern "C" {
    pub fn get_instance_count(guid_string: *const c_char) -> c_int;
}
extern "C" {
    pub fn strlcpy_attr(dest: *mut c_char, src: *mut c_char);
}
extern "C" {
    pub fn alloc_enum_data() -> c_int;
}
extern "C" {
    pub fn exit_enum_attributes();
}
extern "C" {
    pub fn alloc_int_data() -> c_int;
}
extern "C" {
    pub fn exit_int_attributes();
}
extern "C" {
    pub fn populate_str_data(str_obj: *mut acpi_object, instance_id: c_int, attr_name_kobj: *mut kobject) -> c_int;
}
extern "C" {
    pub fn alloc_str_data() -> c_int;
}
extern "C" {
    pub fn exit_str_attributes();
}
extern "C" {
    pub fn populate_po_data(po_obj: *mut acpi_object, instance_id: c_int, attr_name_kobj: *mut kobject) -> c_int;
}
extern "C" {
    pub fn alloc_po_data() -> c_int;
}
extern "C" {
    pub fn exit_po_attributes();
}
extern "C" {
    pub fn set_attribute(a_name: *const c_char, a_value: *const c_char) -> c_int;
}
extern "C" {
    pub fn set_bios_defaults(defType: u8) -> c_int;
}
extern "C" {
    pub fn exit_bios_attr_set_interface();
}
extern "C" {
    pub fn init_bios_attr_set_interface() -> c_int;
}
extern "C" {
    pub fn map_wmi_error(error_code: c_int) -> c_int;
}
extern "C" {
    pub fn calculate_string_buffer(str: *const c_char) -> usize;
}
extern "C" {
    pub fn calculate_security_buffer(authentication: *const c_char) -> usize;
}
extern "C" {
    pub fn populate_security_buffer(buffer: *mut u8, authentication: *const c_char);
}
extern "C" {
    pub fn populate_string_buffer(buffer: *mut u8, buffer_len: usize, str: *const c_char) -> isize;
}
extern "C" {
    pub fn set_new_password(password_type: *const c_char, new: *const c_char) -> c_int;
}
extern "C" {
    pub fn init_bios_attr_pass_interface() -> c_int;
}
extern "C" {
    pub fn exit_bios_attr_pass_interface();
}
