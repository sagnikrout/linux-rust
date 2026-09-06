//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/char/tpm/tpm_tis_spi.h
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
// Copyright (C) 2015 Infineon Technologies AG
// Copyright (C) 2016 STMicroelectronics SAS
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpm_tis_spi_phy {
    pub priv: tpm_tis_data,
    pub spi_device: *mut spi_device,
    pub xfer): *mut spi_transfer,
    pub ready: completion,
    pub wake_after: c_ulong,
    pub iobuf: *mut u8,
}

extern "C" {
    pub fn container_of(_arg: data, tpm_tis_spi_phy: struct, _arg: priv) -> return;
}

extern "C" {
    pub fn cr50_spi_probe(spi: *mut spi_device) -> c_int;
}

extern "C" {
    pub fn tpm_tis_spi_resume(dev: *mut device) -> c_int;
}

