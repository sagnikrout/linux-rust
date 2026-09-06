//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/uapi/asm/hwprobe.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Copyright 2023-2024 Rivos, Inc
//

//
// Interface for probing hardware capabilities from userspace, see
// Documentation/arch/riscv/hwprobe.rst for more information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct riscv_hwprobe {
    pub key: __s64,
    pub value: __u64,
}

pub const RISCV_HWPROBE_KEY_MVENDORID: c_int = 0;
pub const RISCV_HWPROBE_KEY_MARCHID: c_int = 1;
pub const RISCV_HWPROBE_KEY_MIMPID: c_int = 2;
pub const RISCV_HWPROBE_KEY_BASE_BEHAVIOR: c_int = 3;

pub const RISCV_HWPROBE_KEY_IMA_EXT_0: c_int = 4;

pub const RISCV_HWPROBE_KEY_CPUPERF_0: c_int = 5;
pub const RISCV_HWPROBE_MISALIGNED_UNKNOWN: c_int = 0;
pub const RISCV_HWPROBE_MISALIGNED_EMULATED: c_int = 1;
pub const RISCV_HWPROBE_MISALIGNED_SLOW: c_int = 2;
pub const RISCV_HWPROBE_MISALIGNED_FAST: c_int = 3;
pub const RISCV_HWPROBE_MISALIGNED_UNSUPPORTED: c_int = 4;
pub const RISCV_HWPROBE_MISALIGNED_MASK: c_int = 7;
pub const RISCV_HWPROBE_KEY_ZICBOZ_BLOCK_SIZE: c_int = 6;
pub const RISCV_HWPROBE_KEY_HIGHEST_VIRT_ADDRESS: c_int = 7;
pub const RISCV_HWPROBE_KEY_TIME_CSR_FREQ: c_int = 8;
pub const RISCV_HWPROBE_KEY_MISALIGNED_SCALAR_PERF: c_int = 9;
pub const RISCV_HWPROBE_MISALIGNED_SCALAR_UNKNOWN: c_int = 0;
pub const RISCV_HWPROBE_MISALIGNED_SCALAR_EMULATED: c_int = 1;
pub const RISCV_HWPROBE_MISALIGNED_SCALAR_SLOW: c_int = 2;
pub const RISCV_HWPROBE_MISALIGNED_SCALAR_FAST: c_int = 3;
pub const RISCV_HWPROBE_MISALIGNED_SCALAR_UNSUPPORTED: c_int = 4;
pub const RISCV_HWPROBE_KEY_MISALIGNED_VECTOR_PERF: c_int = 10;
pub const RISCV_HWPROBE_MISALIGNED_VECTOR_UNKNOWN: c_int = 0;
pub const RISCV_HWPROBE_MISALIGNED_VECTOR_SLOW: c_int = 2;
pub const RISCV_HWPROBE_MISALIGNED_VECTOR_FAST: c_int = 3;
pub const RISCV_HWPROBE_MISALIGNED_VECTOR_UNSUPPORTED: c_int = 4;
pub const RISCV_HWPROBE_KEY_VENDOR_EXT_THEAD_0: c_int = 11;
pub const RISCV_HWPROBE_KEY_ZICBOM_BLOCK_SIZE: c_int = 12;
pub const RISCV_HWPROBE_KEY_VENDOR_EXT_SIFIVE_0: c_int = 13;
pub const RISCV_HWPROBE_KEY_VENDOR_EXT_MIPS_0: c_int = 14;
pub const RISCV_HWPROBE_KEY_ZICBOP_BLOCK_SIZE: c_int = 15;
pub const RISCV_HWPROBE_KEY_IMA_EXT_1: c_int = 16;

// Increase RISCV_HWPROBE_MAX_KEY when adding items.
// Flags

