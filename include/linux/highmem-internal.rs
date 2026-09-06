//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/highmem-internal.h
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
// Outside of CONFIG_HIGHMEM to support X86 32bit iomap_atomic() cruft.
//

extern "C" {
    pub fn kunmap_local_indexed(vaddr: *const c_void);
}
extern "C" {
    pub fn kmap_local_fork(tsk: *mut task_struct);
}
extern "C" {
    pub fn __kmap_local_sched_out();
}
extern "C" {
    pub fn __kmap_local_sched_in();
}

extern "C" {
    pub fn kunmap_high(page: *const page);
}
extern "C" {
    pub fn __kmap_flush_unused();
}
extern "C" {
    pub fn __kmap_to_page(_arg: addr) -> return;
}
extern "C" {
    pub fn __kmap_local_page_prot(_arg: page, _arg: kmap_prot) -> return;
}
extern "C" {
    pub fn page_address(_arg: page) -> return;
}
// If the page is in HighMem, it's not safe to kmap it.
extern "C" {
    pub fn __kmap_local_page_prot(_arg: page, _arg: prot) -> return;
}
extern "C" {
    pub fn __kmap_local_pfn_prot(_arg: pfn, _arg: kmap_prot) -> return;
}
extern "C" {
    pub fn __kmap_local_page_prot(_arg: page, _arg: prot) -> return;
}
extern "C" {
    pub fn kmap_atomic_prot(_arg: page, _arg: kmap_prot) -> return;
}
extern "C" {
    pub fn __kmap_local_pfn_prot(_arg: pfn, _arg: kmap_prot) -> return;
}
extern "C" {
    pub fn __nr_free_highpages() -> c_ulong;
}
extern "C" {
    pub fn __totalhigh_pages() -> c_ulong;
}
extern "C" {
    pub fn __nr_free_highpages() -> return;
}
extern "C" {
    pub fn __totalhigh_pages() -> return;
}

extern "C" {
    pub fn virt_to_page(_arg: addr) -> return;
}
extern "C" {
    pub fn page_address(_arg: page) -> return;
}

extern "C" {
    pub fn page_address(_arg: page) -> return;
}
extern "C" {
    pub fn page_address(_arg: page) -> return;
}
extern "C" {
    pub fn kmap_local_page(_arg: page) -> return;
}
extern "C" {
    pub fn kmap_local_page(_arg: pfn_to_page(pfn)) -> return;
}

extern "C" {
    pub fn page_address(_arg: page) -> return;
}
extern "C" {
    pub fn kmap_atomic(_arg: page) -> return;
}
extern "C" {
    pub fn kmap_atomic(_arg: pfn_to_page(pfn)) -> return;
}

//
// kunmap_atomic - Unmap the virtual address mapped by kmap_atomic() - deprecated!
// @__addr:       Virtual address to be unmapped
//
// Unmaps an address previously mapped by kmap_atomic() and re-enables
// pagefaults. Depending on PREEMPT_RT configuration, re-enables also
// migration and preemption. Users should not count on these side effects.
//
// Mappings should be unmapped in the reverse order that they were mapped.
// See kmap_local_page() for details on nesting.
//
// @__addr can be any address within the mapped page, so there is no need
// to subtract any offset that has been added. In contrast to kunmap(),
// this function takes the address returned from kmap_atomic(), not the
// page passed to it. The compiler will warn you if you pass the page.
//

//
// kunmap_local - Unmap a page mapped via kmap_local_page().
// @__addr: An address within the page mapped
//
// @__addr can be any address within the mapped page.  Commonly it is the
// address return from kmap_local_page(), but it can also include offsets.
//
// Unmapping should be done in the reverse order of the mapping.  See
// kmap_local_page() for details.
//

