//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/imagination/pvr_rogue_riscv.h
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
// Copyright (c) 2024 Imagination Technologies Ltd.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rogue_riscvfw_region {
    ROGUE_RISCV_REGION__RESERVED_0 = 0,
    ROGUE_RISCV_REGION__RESERVED_1,
    ROGUE_RISCV_REGION_SOCIF,
    ROGUE_RISCV_REGION__RESERVED_3,
    ROGUE_RISCV_REGION__RESERVED_4,
    ROGUE_RISCV_REGION_BOOTLDR_DATA,
    ROGUE_RISCV_REGION_SHARED_CACHED_DATA,
    ROGUE_RISCV_REGION__RESERVED_7,
    ROGUE_RISCV_REGION_COREMEM,
    ROGUE_RISCV_REGION__RESERVED_9,
    ROGUE_RISCV_REGION__RESERVED_A,
    ROGUE_RISCV_REGION__RESERVED_B,
    ROGUE_RISCV_REGION_BOOTLDR_CODE,
    ROGUE_RISCV_REGION_SHARED_UNCACHED_DATA,
    ROGUE_RISCV_REGION__RESERVED_E,
    ROGUE_RISCV_REGION__RESERVED_F,

    ROGUE_RISCV_REGION__COUNT,
}

