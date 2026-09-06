//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mtd/nand/raw/ingenic/ingenic_ecc.h
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
// struct ingenic_ecc_params - ECC parameters
// @size: data bytes per ECC step.
// @bytes: ECC bytes per step.
// @strength: number of correctable bits per ECC step.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ingenic_ecc_params {
    pub size: c_int,
    pub bytes: c_int,
    pub strength: c_int,
}

extern "C" {
    pub fn ingenic_ecc_release(ecc: *mut ingenic_ecc);
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ingenic_ecc_ops {
    pub ecc): *mut *mut void (disable)(struct ingenic_ecc,
    pub ecc_code): *const *const u8 buf, u8,
    pub ecc_code): *mut *mut u8 buf, u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ingenic_ecc {
    pub dev: *mut device,
    pub ops: *const ingenic_ecc_ops,
    pub base: *mut void __iomem,
    pub clk: *mut clk,
    pub lock: mutex,
}

extern "C" {
    pub fn ingenic_ecc_probe(pdev: *mut platform_device) -> c_int;
}
