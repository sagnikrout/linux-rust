//! Automatically rewritten from C Header to Rust Module
//! Source: include/xen/xen.h
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xen_domain_type {
    XEN_NATIVE,		/* running on bare hardware    */
    XEN_PV_DOMAIN,		/* running in a PV domain      */
    XEN_HVM_DOMAIN,		/* running in a Xen hvm domain */
}

pub const xen_pvh: c_int = 0;

pub const xen_pv_domain(): c_int = 0;

pub const xen_pv_pci_possible: c_int = 0;

extern "C" {
    pub fn xen_prepare_pvh();
}
extern "C" {
    pub fn xen_pv_evtchn_do_upcall(regs: *mut pt_regs);
}

extern "C" {
    pub fn xen_alloc_unpopulated_pages(nr_pages: c_uint, pages: *mut page) -> c_int;
}
extern "C" {
    pub fn xen_free_unpopulated_pages(nr_pages: c_uint, pages: *mut page);
}

extern "C" {
    pub fn arch_xen_unpopulated_init(res: *mut resource) -> c_int;
}

extern "C" {
    pub fn xen_alloc_ballooned_pages(_arg: nr_pages, _arg: pages) -> return;
}

extern "C" {
    pub fn xen_processor_present(acpi_id: u32) -> bool __init;
}

