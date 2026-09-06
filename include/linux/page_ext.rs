//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/page_ext.h
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
// struct page_ext_operations - per page_ext client operations
// @offset: Offset to the client's data within page_ext. Offset is returned to
// the client by page_ext_init.
// @size: The size of the client data within page_ext.
// @need: Function that returns true if client requires page_ext.
// @init: (optional) Called to initialize client once page_exts are allocated.
// @need_shared_flags: True when client is using shared page_ext->flags
// field.
//
// Each Page Extension client must define page_ext_operations in
// page_ext_ops array.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct page_ext_operations {
    pub offset: usize,
    pub size: usize,
    pub (*need)(void): *mut bool,
    pub (*init)(void): *mut c_void,
    pub need_shared_flags: bool,
}

//
// The page_ext_flags users must set need_shared_flags to true.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum page_ext_flags {
    PAGE_EXT_OWNER,
    PAGE_EXT_OWNER_ALLOCATED,

    PAGE_EXT_YOUNG,
    PAGE_EXT_IDLE,

}

//
// Page Extension can be considered as an extended mem_map.
// A page_ext page is associated with every page descriptor. The
// page_ext helps us add more information about the page.
// All page_ext are allocated at boot or memory hotplug event,
// then the page_ext for pfn always exists.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct page_ext {
    pub flags: c_ulong,
}

extern "C" {
    pub fn page_ext_init();
}
//
// page_ext is allocated per memory section. Once we cross a
// memory section, we have to fetch the new pointer.
//

extern "C" {
    pub fn page_ext_init_flatmem();
}
extern "C" {
    pub fn page_ext_init_flatmem_late();
}

extern "C" {
    pub fn page_ext_put(page_ext: *mut page_ext);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct page_ext_iter {
    pub index: c_ulong,
    pub start_pfn: c_ulong,
    pub page_ext: *mut page_ext,
}

//
// page_ext_iter_begin() - Prepare for iterating through page extensions.
// @iter: page extension iterator.
// @pfn: PFN of the page we're interested in.
// @count: maximum number of page extensions to return.
//
// Must be called with RCU read lock taken.
//
// Return: NULL if no page_ext exists for this page.
//
// page_ext_iter_next() - Get next page extension
// @iter: page extension iterator.
// @count: maximum number of page extensions to return.
//
// Must be called with RCU read lock taken.
//
// Return: NULL if no next page_ext exists.
//
// page_ext_iter_get() - Get current page extension
// @iter: page extension iterator.
//
// Return: NULL if no page_ext exists for this iterator.
//
// for_each_page_ext(): iterate through page_ext objects.
// @__page: the page we're interested in
// @__pgcount: how many pages to iterate through
// @__page_ext: struct page_ext pointer where the current page_ext
// object is returned
// @__iter: struct page_ext_iter object (defined in the stack)
//
// IMPORTANT: must be called with RCU read lock taken.
//

