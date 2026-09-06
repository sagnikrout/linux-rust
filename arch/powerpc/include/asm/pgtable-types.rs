//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/pgtable-types.h
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

// Macro flag: #define STRICT_MM_TYPECHECKS

// PTE level

pub type pte_t = pte_basic_t;

// PMD level

// 64 bit always use 4 level table.

// PGD level

// Page protection bits

//
// With hash config 64k pages additionally define a bigger "real PTE" type that
// gathers the "second half" part of the PTE for pseudo 64k pages
//

// See comment in switch_mm_irqs_off()
extern "C" {
    pub fn pte_val(__cmpxchg_u64(p: old) ==, _arg: pte_val(old), _arg: pte_val(new)) -> return;
}

