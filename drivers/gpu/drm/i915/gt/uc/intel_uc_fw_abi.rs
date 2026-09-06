//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/gt/uc/intel_uc_fw_abi.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2019 Intel Corporation
//

//
// DOC: Firmware Layout
//
// The GuC/HuC firmware layout looks like this::
//
// +======================================================================+
// |  Firmware blob                                                       |
// +===============+===============+============+============+============+
// |  CSS header   |     uCode     |  RSA key   |  modulus   |  exponent  |
// +===============+===============+============+============+============+
// <-header size->                 <---header size continued ----------->
// <--- size ----------------------------------------------------------->
// <-key size->
// <-mod size->
// <-exp size->
//
// The firmware may or may not have modulus key and exponent data. The header,
// uCode and RSA signature are must-have components that will be used by driver.
// Length of each components, which is all in dwords, can be found in header.
// In the case that modulus and exponent are not present in fw, a.k.a truncated
// image, the length value still appears in header.
//
// Driver will do some basic fw size validation based on the following rules:
//
// 1. Header, uCode and RSA are must-have components.
// 2. All firmware components, if they present, are in the sequence illustrated
// in the layout table above.
// 3. Length info of each component can be found in header, in dwords.
// 4. Modulus and exponent key are not required by driver. They may not appear
// in fw. So driver will load a truncated firmware in this case.
//
// Starting from DG2, the HuC is loaded by the GSC instead of i915. The GSC
// firmware performs all the required integrity checks, we just need to check
// the version. Note that the header for GSC-managed blobs is different from the
// CSS used for dma-loaded firmwares.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uc_css_header {
    pub module_type: u32,
//
// header_size includes all non-uCode bits, including css_header, rsa
// key, modulus key and exponent data.
//
    pub header_size_dw: u32,
    pub header_version: u32,
    pub module_id: u32,
    pub module_vendor: u32,
    pub date: u32,

    pub /: *mut *mut u32 size_dw; / uCode plus header_size_dw,
    pub key_size_dw: u32,
    pub modulus_size_dw: u32,
    pub exponent_size_dw: u32,
    pub time: u32,
    pub username: [c_char; 8],
    pub buildnumber: [c_char; 12],
    pub sw_version: u32,

    pub vf_version: u32,
    pub reserved0: [u32; 12],
    pub /: *mut *mut u32 private_data_size; / only applies to GuC,
    pub reserved1: u32,
}
