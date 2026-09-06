//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/transport_class.h
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
// transport_class.h - a generic container for all transport classes
//
// Copyright (c) 2005 - James Bottomley <James.Bottomley@steeleye.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct transport_class {
    pub class: class,
    pub ): *mut device,
    pub ): *mut device,
    pub ): *mut device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct anon_transport_class {
    pub tclass: transport_class,
    pub container: attribute_container,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct transport_container {
    pub ac: attribute_container,
    pub statistics: *const attribute_group,
    pub encryption: *const attribute_group,
}

extern "C" {
    pub fn transport_remove_device(: *mut device);
}
extern "C" {
    pub fn transport_add_device(: *mut device) -> c_int;
}
extern "C" {
    pub fn transport_setup_device(: *mut device);
}
extern "C" {
    pub fn transport_configure_device(: *mut device);
}
extern "C" {
    pub fn transport_destroy_device(: *mut device);
}
extern "C" {
    pub fn transport_class_register(: *mut transport_class) -> c_int;
}
extern "C" {
    pub fn anon_transport_class_register(: *mut anon_transport_class);
}
extern "C" {
    pub fn transport_class_unregister(: *mut transport_class);
}
extern "C" {
    pub fn anon_transport_class_unregister(: *mut anon_transport_class);
}
