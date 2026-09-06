//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/setup_data.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_setup_rom {
    pub data: setup_data,
    pub vendor: u16,
    pub devid: u16,
    pub pcilen: u64,
    pub segment: c_ulong,
    pub bus: c_ulong,
    pub device: c_ulong,
    pub function: c_ulong,
    pub romdata: [u8; ],
}

// kexec external ABI
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efi_setup_data {
    pub fw_vendor: u64,
    pub __unused: u64,
    pub tables: u64,
    pub smbios: u64,
    pub reserved: [u64; 8],
}

