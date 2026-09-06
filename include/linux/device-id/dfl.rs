//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/device-id/dfl.h
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

pub type kernel_ulong_t = c_ulong;

//
// DFL (Device Feature List)
//
// DFL defines a linked list of feature headers within the device MMIO space to
// provide an extensible way of adding features. Software can walk through these
// predefined data structures to enumerate features. It is now used in the FPGA.
// See Documentation/fpga/dfl.rst for more information.
//
// The dfl bus type is introduced to match the individual feature devices (dfl
// devices) for specific dfl drivers.
//
// struct dfl_device_id -  dfl device identifier
// @type: DFL FIU type of the device. See enum dfl_id_type.
// @feature_id: feature identifier local to its DFL FIU type.
// @driver_data: driver specific data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dfl_device_id {
    pub type: __u16,
    pub feature_id: __u16,
    pub driver_data: kernel_ulong_t,
}
