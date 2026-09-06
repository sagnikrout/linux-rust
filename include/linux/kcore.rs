//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/kcore.h
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
// /proc/kcore definitions
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kcore_type {
    KCORE_TEXT,
    KCORE_VMALLOC,
    KCORE_RAM,
    KCORE_VMEMMAP,
    KCORE_USER,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kcore_list {
    pub list: list_head,
    pub addr: c_ulong,
    pub size: usize,
    pub type: c_int,
}

extern "C" {
    pub fn kclist_add(: *mut kcore_list, : *mut c_void, _arg: usize, type: c_int) -> void __init;
}
extern "C" {
    pub fn register_mem_pfn_is_ram(pfn): *mut *mut int (fn)(unsigned long) -> int __init;
}

