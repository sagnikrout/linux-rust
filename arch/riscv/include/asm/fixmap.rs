//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/fixmap.h
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
// Copyright (C) 2019 Western Digital Corporation or its affiliates.
//

//
// Here we define all the compile-time 'special' virtual addresses.
// The point is to have a constant address at compile time, but to
// set the physical address only in the boot process.
//
// These 'compile-time allocated' memory buffers are page-sized. Use
// set_fixmap(idx,phys) to associate physical memory with fixmap indices.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fixed_addresses {
    FIX_HOLE,
//
// The fdt fixmap mapping must be PMD aligned and will be mapped
// using PMD entries in fixmap_pmd in 64-bit and a PGD entry in 32-bit.
//
    FIX_FDT_END,
    FIX_FDT = FIX_FDT_END + FIX_FDT_SIZE / PAGE_SIZE - 1,

// Below fixmaps will be mapped using fixmap_pte
    FIX_PTE,
    FIX_PMD,
    FIX_PUD,
    FIX_P4D,
    FIX_TEXT_POKE1,
    FIX_TEXT_POKE0,
    FIX_EARLYCON_MEM_BASE,

    __end_of_permanent_fixed_addresses,
//
// Temporary boot-time mappings, used by early_ioremap(),
// before ioremap() is functional.
//

pub const FIX_BTMAPS_SLOTS: c_int = 7;

    FIX_BTMAP_END = __end_of_permanent_fixed_addresses,
    FIX_BTMAP_BEGIN = FIX_BTMAP_END + TOTAL_FIX_BTMAPS - 1,

    __end_of_fixed_addresses
}

