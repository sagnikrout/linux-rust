//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/asm/mmu_context.h
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
// Switch a MMU context.
//
// Copyright (C) 2020-2022 Loongson Technology Corporation Limited
//

//
// All unused by hardware upper bits will be considered
// as a software asid extension.
//

// Normal, classic get_new_mmu_context
// need_flush = true;	/* start new asid cycle
//
// Initialize the context related info for a new mm_struct
// instance.
//
// Check if our ASID is of an older version and thus invalid
//
// Mark current->active_mm as not "active" anymore.
// We don't want to mislead possible IPI tlb flush routines.
//

//
// Destroy context related info for an mm_struct that is about
// to be put to rest.
//

//
// If mm is currently active, we can't really drop it.
// Instead, we will get a new one for it.
//
// Will get a new context next time
