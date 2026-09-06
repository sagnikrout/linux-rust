//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/asm/kvm_ipi.h
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
// Copyright (C) 2024 Loongson Technology Corporation Limited
//

pub const LARCH_INT_IPI: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct loongarch_ipi {
    pub lock: spinlock_t,
    pub kvm: *mut kvm,
    pub device: kvm_io_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipi_state {
    pub lock: spinlock_t,
    pub status: u32,
    pub en: u32,
    pub set: u32,
    pub clear: u32,
    pub buf: [u64; 4],
}

pub const IOCSR_IPI_BASE: c_uint = 0x1000;
pub const IOCSR_IPI_SIZE: c_uint = 0x160;
pub const IOCSR_IPI_STATUS: c_uint = 0x000;
pub const IOCSR_IPI_EN: c_uint = 0x004;
pub const IOCSR_IPI_SET: c_uint = 0x008;
pub const IOCSR_IPI_CLEAR: c_uint = 0x00c;
pub const IOCSR_IPI_BUF_20: c_uint = 0x020;
pub const IOCSR_IPI_BUF_28: c_uint = 0x028;
pub const IOCSR_IPI_BUF_30: c_uint = 0x030;
pub const IOCSR_IPI_BUF_38: c_uint = 0x038;
pub const IOCSR_IPI_SEND: c_uint = 0x040;
pub const IOCSR_MAIL_SEND: c_uint = 0x048;
pub const IOCSR_ANY_SEND: c_uint = 0x158;
extern "C" {
    pub fn kvm_loongarch_register_ipi_device() -> c_int;
}
extern "C" {
    pub fn kvm_loongarch_unregister_ipi_device();
}
