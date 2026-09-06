//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/init.h
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
pub struct x86_mapping_info {
    pub /: *mut *mut *mut *mut *mut void (alloc_pgt_page)(void ); / allocate buf for page table,
    pub /: *mut *mut *mut *mut *mut void (free_pgt_page)(void , void ); / free buf for page table,
    pub /: *mut *mut *mut void context; / context for alloc_pgt_page,
    pub /: *mut *mut unsigned long page_flag; / page flag for PMD or PUD entry,
    pub /: *mut *mut unsigned long offset; / ident mapping offset,
    pub /: *mut *mut bool direct_gbpages; / PUD level 1GB page support,
    pub /: *mut *mut unsigned long kernpg_flag; / kernel pagetable flag override,
}

extern "C" {
    pub fn kernel_ident_mapping_free(info: *mut x86_mapping_info, pgd: *mut pgd_t);
}
