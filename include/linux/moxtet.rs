//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/moxtet.h
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
// Turris Mox module configuration bus driver
//
// Copyright (C) 2019 Marek Behún <kabel@kernel.org>
//

pub const TURRIS_MOX_MAX_MODULES: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum turris_mox_cpu_module_id {
    TURRIS_MOX_CPU_ID_EMMC	= 0x00,
    TURRIS_MOX_CPU_ID_SD	= 0x10,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum turris_mox_module_id {
    TURRIS_MOX_MODULE_FIRST		= 0x01,

    TURRIS_MOX_MODULE_SFP		= 0x01,
    TURRIS_MOX_MODULE_PCI		= 0x02,
    TURRIS_MOX_MODULE_TOPAZ		= 0x03,
    TURRIS_MOX_MODULE_PERIDOT	= 0x04,
    TURRIS_MOX_MODULE_USB3		= 0x05,
    TURRIS_MOX_MODULE_PCI_BRIDGE	= 0x06,

    TURRIS_MOX_MODULE_LAST		= 0x06,
}

pub const MOXTET_NIRQS: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct moxtet {
    pub dev: *mut device,
    pub lock: mutex,
    pub modules: [u8; TURRIS_MOX_MAX_MODULES],
    pub count: c_int,
    pub tx: [u8; TURRIS_MOX_MAX_MODULES],
    pub dev_irq: c_int,
    pub domain: *mut irq_domain,
    pub chip: irq_chip,
    pub exists: unsigned long masked,,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct moxtet_irqpos {
    pub idx: u8,
    pub bit: u8,
    pub position: [}; MOXTET_NIRQS],
    pub irq: },

    pub debugfs_root: *mut dentry,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct moxtet_driver {
    pub id_table: *const turris_mox_module_id,
    pub driver: device_driver,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct moxtet_device {
    pub dev: device,
    pub moxtet: *mut moxtet,
    pub id: turris_mox_module_id,
    pub idx: c_uint,
}

extern "C" {
    pub fn moxtet_device_read(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn moxtet_device_write(dev: *mut device, val: u8) -> c_int;
}
extern "C" {
    pub fn moxtet_device_written(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn container_of(_arg: dev, moxtet_device: struct, _arg: dev) -> return;
}
