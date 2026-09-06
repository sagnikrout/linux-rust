//! Automatically rewritten from C Header to Rust Module
//! Source: include/asm-generic/hugetlb.h
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

extern "C" {
    pub fn pte_write(_arg: pte) -> return;
}
extern "C" {
    pub fn pte_dirty(_arg: pte) -> return;
}
extern "C" {
    pub fn pte_mkwrite_novma(_arg: pte) -> return;
}
extern "C" {
    pub fn pte_wrprotect(_arg: pte) -> return;
}

extern "C" {
    pub fn pte_mkdirty(_arg: pte) -> return;
}
extern "C" {
    pub fn pte_modify(_arg: pte, _arg: newprot) -> return;
}
extern "C" {
    pub fn huge_pte_wrprotect(_arg: pte_mkuffd(pte)) -> return;
}

extern "C" {
    pub fn pte_clear_uffd(_arg: pte) -> return;
}

extern "C" {
    pub fn pte_uffd(_arg: pte) -> return;
}

extern "C" {
    pub fn ptep_get_and_clear(_arg: mm, _arg: addr, _arg: ptep) -> return;
}

extern "C" {
    pub fn ptep_clear_flush(_arg: vma, _arg: addr, _arg: ptep) -> return;
}

extern "C" {
    pub fn pte_none(_arg: pte) -> return;
}

extern "C" {
    pub fn ptep_set_access_flags(_arg: vma, _arg: addr, _arg: ptep, _arg: pte, _arg: dirty) -> return;
}

extern "C" {
    pub fn ptep_get(_arg: ptep) -> return;
}

extern "C" {
    pub fn IS_ENABLED(_arg: CONFIG_ARCH_HAS_GIGANTIC_PAGE) -> return;
}

