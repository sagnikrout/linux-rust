//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hid/intel-thc-hid/intel-thc/intel-thc-wot.h
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
// Copyright (c) 2025 Intel Corporation

//
// struct thc_wot - THC Wake-on-Touch data structure
// @gpio_irq : GPIO interrupt IRQ number for wake-on-touch
// @gpio_irq_wakeable : Indicate GPIO IRQ workable or not
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct thc_wot {
    pub gpio_irq: c_int,
    pub gpio_irq_wakeable: bool,
}

extern "C" {
    pub fn thc_wot_config(thc_dev: *mut thc_device, gpio_map: *const acpi_gpio_mapping);
}
extern "C" {
    pub fn thc_wot_unconfig(thc_dev: *mut thc_device);
}
