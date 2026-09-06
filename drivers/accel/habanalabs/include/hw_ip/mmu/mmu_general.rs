//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accel/habanalabs/include/hw_ip/mmu/mmu_general.h
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
// Copyright 2016-2020 HabanaLabs, Ltd.
// All Rights Reserved.
//
pub const PAGE_SHIFT_4KB: c_int = 12;
pub const PAGE_SHIFT_64KB: c_int = 16;
pub const PAGE_SHIFT_2MB: c_int = 21;
pub const PAGE_SHIFT_16MB: c_int = 24;
pub const PAGE_SHIFT_64MB: c_int = 26;
pub const PAGE_SHIFT_1GB: c_int = 30;

pub const PAGE_PRESENT_MASK: c_uint = 0x0000000000001ull;
pub const SWAP_OUT_MASK: c_uint = 0x0000000000004ull;
pub const LAST_MASK: c_uint = 0x0000000000800ull;
pub const FLAGS_MASK: c_uint = 0x0000000000FFFull;
pub const MMU_ARCH_3_HOPS: c_int = 3;
pub const MMU_ARCH_4_HOPS: c_int = 4;
pub const MMU_ARCH_5_HOPS: c_int = 5;
pub const MMU_ARCH_6_HOPS: c_int = 6;

// definitions for HOP with 512 PTE entries
pub const HOP_PTE_ENTRIES_512: c_int = 512;

pub const MMU_HOP0_PA43_12_SHIFT: c_int = 12;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mmu_hop_num {
    MMU_HOP0,
    MMU_HOP1,
    MMU_HOP2,
    MMU_HOP3,
    MMU_HOP4,
    MMU_HOP5,
    MMU_HOP_MAX,
}
