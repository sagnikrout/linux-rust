//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/nohash/32/pgalloc.h
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
// We don't have any real pmd's, and this code never triggers because
// the pgd will always be present..
//
// #define pmd_alloc_one(mm,address)       ({ BUG(); ((pmd_t *)2); })

// #define pgd_populate(mm, pmd, pte)      BUG()
// pmdp = __pmd((unsigned long)pte | _PMD_PRESENT);
// pmdp = __pmd(__pa(pte) | _PMD_PRESENT);
// pmdp = __pmd((unsigned long)pte_page | _PMD_PRESENT);
// pmdp = __pmd(__pa(pte_page) | _PMD_USER | _PMD_PRESENT);
