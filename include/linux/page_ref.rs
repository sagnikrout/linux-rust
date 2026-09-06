//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/page_ref.h
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
// Ideally we would want to use the trace_<tracepoint>_enabled() helper
// functions. But due to include header file issues, that is not
// feasible. Instead we have to open code the static key functions.
//
// See trace_##name##_enabled(void) in include/linux/tracepoint.h
//

extern "C" {
    pub fn __page_ref_set(page: *mut page, v: c_int);
}
extern "C" {
    pub fn __page_ref_mod(page: *mut page, v: c_int);
}
extern "C" {
    pub fn __page_ref_mod_and_test(page: *mut page, v: c_int, ret: c_int);
}
extern "C" {
    pub fn __page_ref_mod_and_return(page: *mut page, v: c_int, ret: c_int);
}
extern "C" {
    pub fn __page_ref_mod_unless(page: *mut page, v: c_int, u: c_int);
}
extern "C" {
    pub fn __page_ref_freeze(page: *mut page, v: c_int, ret: c_int);
}
extern "C" {
    pub fn __page_ref_unfreeze(page: *mut page, v: c_int);
}

extern "C" {
    pub fn atomic_read(_arg: &page->_refcount) -> return;
}
//
// folio_ref_count - The reference count on this folio.
// @folio: The folio.
//
// Folios contain a reference count.  When that reference count reaches
// zero, the folio is referred to as frozen.  At this point, it will
// usually be returned to the memory allocator, but some parts of the
// kernel freeze folios in order to perform unusual operations on them
// such as splitting or migration.
//
// The refcount is usually incremented by calls to folio_get() and
// decremented by calls to folio_put().  Some typical users of the
// folio refcount:
//
// - Each reference from a page table
// - The page cache
// - Filesystem private data
// - The LRU list
// - Pipes
// - Direct IO which references this page in the process address space
//
// The reference count has three components: expected, temporary and
// spurious.  The expected reference count of a folio is that which
// we would logically expect it to be from just reading the code.
// Temporary refcounts are gained by threads which need a temporary
// reference to make sure the folio isn't reallocated while they use it.
// Spurious refcounts are gained by threads which, thanks to RCU walks
// of the page tables or file cache, find a stale pointer to a folio.
// These threads will drop the refcount after discoveering the pointer
// is stale, but it can surprise other users to see the spurious refcount
// on a freshly allocated folio (eg they may see a refcount of 2 instead
// of 1).
//
// Return: The number of references to this folio.
//
extern "C" {
    pub fn page_ref_count(_arg: &folio->page) -> return;
}
extern "C" {
    pub fn folio_ref_count(_arg: page_folio(page)) -> return;
}
//
// Setup the page count before being freed into the page allocator for
// the first time (boot or memory hotplug)
//
extern "C" {
    pub fn page_ref_sub_and_test(_arg: &folio->page, _arg: nr) -> return;
}
extern "C" {
    pub fn page_ref_inc_return(_arg: &folio->page) -> return;
}
extern "C" {
    pub fn page_ref_dec_and_test(_arg: &folio->page) -> return;
}
extern "C" {
    pub fn page_ref_dec_return(_arg: &folio->page) -> return;
}
extern "C" {
    pub fn page_ref_add_unless_zero(_arg: &folio->page, _arg: nr) -> return;
}
//
// folio_try_get - Attempt to increase the refcount on a folio.
// @folio: The folio.
//
// If you do not already have a reference to a folio, you can attempt to
// get one using this function.  It may fail if, for example, the folio
// has been freed since you found a pointer to it, or it is frozen for
// the purposes of splitting or migration.
//
// Return: True if the reference count was successfully incremented.
//
extern "C" {
    pub fn folio_ref_add_unless_zero(_arg: folio, _arg: 1) -> return;
}
extern "C" {
    pub fn folio_ref_add_unless_zero(_arg: folio, _arg: count) -> return;
}
extern "C" {
    pub fn page_ref_freeze(_arg: &folio->page, _arg: count) -> return;
}
