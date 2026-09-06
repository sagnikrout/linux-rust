//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/stage2_pgtable.h
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
// Copyright (C) 2016 - ARM Ltd
//
// stage2 page table helpers
//

//
// The hardware supports concatenation of up to 16 tables at stage2 entry
// level and we use the feature whenever possible, which means we resolve 4
// additional bits of address at the entry level.
//
// This implies, the total number of page table levels required for
// IPA_SHIFT at stage2 expected by the hardware can be calculated using
// the same logic used for the (non-collapsable) stage1 page tables but for
// (IPA_SHIFT - 4).
//

//
// kvm_mmmu_cache_min_pages() is the number of pages required to install
// a stage-2 translation. We pre-allocate the entry level page table at
// the VM creation.
//

