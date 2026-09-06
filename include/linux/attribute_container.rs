//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/attribute_container.h
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
// attribute_container.h - a generic container for all classes
//
// Copyright (c) 2005 - James Bottomley <James.Bottomley@steeleye.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct attribute_container {
    pub node: list_head,
    pub containers: klist,
    pub class: *mut class,
    pub grp: *const attribute_group,
    pub attrs: *mut device_attribute,
    pub ): *mut *mut *mut int (match)(struct attribute_container , struct device,
pub const ATTRIBUTE_CONTAINER_NO_CLASSDEVS: c_uint = 0x01;
    pub flags: c_ulong,
}

extern "C" {
    pub fn attribute_container_register(cont: *mut attribute_container);
}
extern "C" {
    pub fn attribute_container_unregister(cont: *mut attribute_container) -> int __must_check;
}
extern "C" {
    pub fn attribute_container_add_attrs(classdev: *mut device) -> c_int;
}
extern "C" {
    pub fn attribute_container_add_class_device(classdev: *mut device) -> c_int;
}
extern "C" {
    pub fn attribute_container_remove_attrs(classdev: *mut device);
}
extern "C" {
    pub fn attribute_container_class_device_del(classdev: *mut device);
}
