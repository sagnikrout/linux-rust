//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/misc/mchp_pci1xxxx/mchp_pci1xxxx_gp.h
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
// Copyright (C) 2022 Microchip Technology Inc.

// Perform operations like variable length write, read and write with read back for OTP / EEPROM
// Perform bit mode write in OTP
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gp_aux_data_type {
    pub irq_num: c_int,
    pub region_start: resource_size_t,
    pub region_length: resource_size_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct auxiliary_device_wrapper {
    pub aux_dev: auxiliary_device,
    pub gp_aux_data: gp_aux_data_type,
}
