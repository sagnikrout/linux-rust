//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ptdump.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptdump_range {
    pub start: c_ulong,
    pub end: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptdump_state {
    pub pte): *mut *mut *mut void (note_page_pte)(struct ptdump_state st, unsigned long addr, pte_t,
    pub pmd): *mut *mut *mut void (note_page_pmd)(struct ptdump_state st, unsigned long addr, pmd_t,
    pub pud): *mut *mut *mut void (note_page_pud)(struct ptdump_state st, unsigned long addr, pud_t,
    pub p4d): *mut *mut *mut void (note_page_p4d)(struct ptdump_state st, unsigned long addr, p4d_t,
    pub pgd): *mut *mut *mut void (note_page_pgd)(struct ptdump_state st, unsigned long addr, pgd_t,
    pub st): *mut *mut void (note_page_flush)(struct ptdump_state,
    pub pte): *mut *mut *mut void (effective_prot_pte)(struct ptdump_state st, pte_t,
    pub pmd): *mut *mut *mut void (effective_prot_pmd)(struct ptdump_state st, pmd_t,
    pub pud): *mut *mut *mut void (effective_prot_pud)(struct ptdump_state st, pud_t,
    pub p4d): *mut *mut *mut void (effective_prot_p4d)(struct ptdump_state st, p4d_t,
    pub pgd): *mut *mut *mut void (effective_prot_pgd)(struct ptdump_state st, pgd_t,
    pub range: *const ptdump_range,
}

extern "C" {
    pub fn ptdump_walk_pgd(st: *mut ptdump_state, mm: *mut mm_struct, pgd: *mut pgd_t);
}
extern "C" {
    pub fn ptdump_check_wx() -> bool;
}
