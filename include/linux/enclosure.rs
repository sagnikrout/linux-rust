//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/enclosure.h
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
// Enclosure Services
//
// Copyright (C) 2008 James Bottomley <James.Bottomley@HansenPartnership.com>
//
// -----------------------------------------------------------------------------
//
// -----------------------------------------------------------------------------
//

// A few generic types ... taken from ses-2
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum enclosure_component_type {
    ENCLOSURE_COMPONENT_DEVICE = 0x01,
    ENCLOSURE_COMPONENT_CONTROLLER_ELECTRONICS = 0x07,
    ENCLOSURE_COMPONENT_SCSI_TARGET_PORT = 0x14,
    ENCLOSURE_COMPONENT_SCSI_INITIATOR_PORT = 0x15,
    ENCLOSURE_COMPONENT_ARRAY_DEVICE = 0x17,
    ENCLOSURE_COMPONENT_SAS_EXPANDER = 0x18,
}

// ses-2 common element status
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum enclosure_status {
    ENCLOSURE_STATUS_UNSUPPORTED = 0,
    ENCLOSURE_STATUS_OK,
    ENCLOSURE_STATUS_CRITICAL,
    ENCLOSURE_STATUS_NON_CRITICAL,
    ENCLOSURE_STATUS_UNRECOVERABLE,
    ENCLOSURE_STATUS_NOT_INSTALLED,
    ENCLOSURE_STATUS_UNKNOWN,
    ENCLOSURE_STATUS_UNAVAILABLE,
// last element for counting purposes
    ENCLOSURE_STATUS_MAX
}

// SFF-8485 activity light settings
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum enclosure_component_setting {
    ENCLOSURE_SETTING_DISABLED = 0,
    ENCLOSURE_SETTING_ENABLED = 1,
    ENCLOSURE_SETTING_BLINK_A_ON_OFF = 2,
    ENCLOSURE_SETTING_BLINK_A_OFF_ON = 3,
    ENCLOSURE_SETTING_BLINK_B_ON_OFF = 6,
    ENCLOSURE_SETTING_BLINK_B_OFF_ON = 7,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct enclosure_component_callbacks {
    pub ): *mut enclosure_component,
    pub enclosure_status): enum,
    pub ): *mut enclosure_component,
    pub enclosure_component_setting): enum,
    pub ): *mut enclosure_component,
    pub enclosure_component_setting): enum,
    pub ): *mut enclosure_component,
    pub enclosure_component_setting): enum,
    pub ): *mut enclosure_component,
    pub buf): *mut *mut *mut int (show_id)(struct enclosure_device , char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct enclosure_component {
    pub scratch: *mut c_void,
    pub cdev: device,
    pub dev: *mut device,
    pub type: enclosure_component_type,
    pub number: c_int,
    pub fault: c_int,
    pub active: c_int,
    pub locate: c_int,
    pub slot: c_int,
    pub status: enclosure_status,
    pub power_status: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct enclosure_device {
    pub scratch: *mut c_void,
    pub node: list_head,
    pub edev: device,
    pub cb: *mut enclosure_component_callbacks,
    pub components: c_int,
    pub component: [enclosure_component; ],
}

extern "C" {
    pub fn container_of(_arg: dev, enclosure_device: struct, _arg: edev) -> return;
}
extern "C" {
    pub fn container_of(_arg: dev, enclosure_component: struct, _arg: cdev) -> return;
}
extern "C" {
    pub fn enclosure_unregister(: *mut enclosure_device);
}
extern "C" {
    pub fn enclosure_component_register(: *mut enclosure_component) -> c_int;
}
extern "C" {
    pub fn enclosure_remove_device(: *mut enclosure_device, : *mut device) -> c_int;
}
