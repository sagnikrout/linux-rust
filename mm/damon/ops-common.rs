//! Automatically rewritten from C Header to Rust Module
//! Source: mm/damon/ops-common.h
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
// Common Code for Data Access Monitoring
//

extern "C" {
    pub fn damon_ptep_mkold(pte: *mut pte_t, vma: *mut vm_area_struct, addr: c_ulong);
}
extern "C" {
    pub fn damon_pmdp_mkold(pmd: *mut pmd_t, vma: *mut vm_area_struct, addr: c_ulong);
}
extern "C" {
    pub fn damon_folio_mkold(folio: *mut folio);
}
extern "C" {
    pub fn damon_folio_young(folio: *mut folio) -> bool;
}
extern "C" {
    pub fn damos_folio_filter_match(filter: *mut damos_filter, folio: *mut folio) -> bool;
}
extern "C" {
    pub fn damon_migrate_pages(folio_list: *mut list_head, target_nid: c_int) -> c_ulong;
}
extern "C" {
    pub fn damos_ops_has_filter(s: *mut damos) -> bool;
}
