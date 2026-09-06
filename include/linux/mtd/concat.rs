//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mtd/concat.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// MTD device concatenation layer definitions
//
// Copyright © 2002      Robert Kaiser <rkaiser@sysgo.de>
//
// Our storage structure:
// Subdev points to an array of pointers to struct mtd_info objects
// which is allocated along with this structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtd_concat {
    pub mtd: mtd_info,
    pub num_subdev: c_int,
    pub subdev: [*mut mtd_info; ],
}

extern "C" {
    pub fn mtd_concat_destroy(mtd: *mut mtd_info);
}
//
// mtd_virt_concat_node_create - Create a component for concatenation
//
// Returns a positive number representing the no. of devices found for
// concatenation, or a negative error code.
//
// List all the devices for concatenations found in DT and create a
// component for concatenation.
//
extern "C" {
    pub fn mtd_virt_concat_node_create() -> c_int;
}
//
// mtd_virt_concat_add - add mtd_info object to the list of subdevices for concatenation
// @mtd: pointer to new MTD device info structure
//
// Returns true if the mtd_info object is added successfully else returns false.
//
// The mtd_info object is added to the list of subdevices for concatenation.
// It returns true if a match is found, and false if all subdevices have
// already been added or if the mtd_info object does not match any of the
// intended MTD devices.
//
extern "C" {
    pub fn mtd_virt_concat_add(mtd: *mut mtd_info) -> bool;
}
//
// mtd_virt_concat_create_join - Create and register the concatenated MTD device
//
// Returns 0 on succes, or a negative error code.
//
// Creates and registers the concatenated MTD device
//
extern "C" {
    pub fn mtd_virt_concat_create_join() -> c_int;
}
//
// mtd_virt_concat_destroy - Remove the concat that includes a specific mtd device
// as one of its components.
// @mtd: pointer to MTD device info structure.
//
// Returns 0 on succes, or a negative error code.
//
// If the mtd_info object is part of a concatenated device, all other MTD devices
// within that concat are registered individually. The concatenated device is then
// removed, along with its concatenation component.
//
extern "C" {
    pub fn mtd_virt_concat_destroy(mtd: *mut mtd_info) -> c_int;
}
extern "C" {
    pub fn mtd_virt_concat_destroy_joins();
}
extern "C" {
    pub fn mtd_virt_concat_destroy_items();
}
