//! Automatically rewritten from C Header to Rust Module
//! Source: arch/loongarch/include/asm/highmem.h
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
// highmem.h: virtual kernel memory mappings for high memory
//
// Used in CONFIG_HIGHMEM systems for memory pages which
// are not addressable by direct kernel virtual addresses.
//
// Copyright (C) 2025 Loongson Technology Corporation Limited
//

// Macro flag: #define ARCH_HAS_KMAP_FLUSH_TLB
extern "C" {
    pub fn kmap_flush_tlb(addr: c_ulong);
}

//
// Right now we initialize only a single pte table. It can be extended
// easily, subsequent pte tables have to be allocated in one physical
// chunk of RAM.
//
pub const LAST_PKMAP: c_int = 1024;

