//! Automatically rewritten from C Header to Rust Module
//! Source: include/xen/xen-ops.h
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
    pub fn per_cpu(_arg: xen_vcpu_id, _arg: cpu) -> return;
}

extern "C" {
    pub fn xen_arch_pre_suspend();
}
extern "C" {
    pub fn xen_arch_post_suspend(suspend_cancelled: c_int);
}
extern "C" {
    pub fn xen_timer_resume();
}
extern "C" {
    pub fn xen_arch_resume();
}
extern "C" {
    pub fn xen_arch_suspend();
}
extern "C" {
    pub fn xen_reboot(reason: c_int);
}
extern "C" {
    pub fn xen_resume_notifier_register(nb: *mut notifier_block);
}
extern "C" {
    pub fn xen_vcpu_stolen(vcpu: c_int) -> bool;
}
extern "C" {
    pub fn xen_setup_runstate_info(cpu: c_int);
}
extern "C" {
    pub fn xen_time_setup_guest();
}
extern "C" {
    pub fn xen_manage_runstate_time(action: c_int);
}
extern "C" {
    pub fn xen_steal_clock(cpu: c_int) -> u64;
}
extern "C" {
    pub fn xen_setup_shutdown_event() -> c_int;
}

//
// xen_remap_domain_gfn_array() - map an array of foreign frames by gfn
// @vma:     VMA to map the pages into
// @addr:    Address at which to map the pages
// @gfn:     Array of GFNs to map
// @nr:      Number entries in the GFN array
// @err_ptr: Returns per-GFN error status.
// @prot:    page protection mask
// @domid:   Domain owning the pages
// @pages:   Array of pages if this domain has an auto-translated physmap
//
// @gfn and @err_ptr may point to the same buffer, the GFNs will be
// overwritten by the error codes after they are mapped.
//
// Returns the number of successfully mapped frames, or a -ve error
// code.
//
// We BUG_ON because it's a programmer error to pass a NULL err_ptr,
// and the consequences later is quite hard to detect what the actual
// cause of "wrong memory was mapped in".
//
// xen_remap_domain_mfn_array() - map an array of foreign frames by mfn
// @vma:     VMA to map the pages into
// @addr:    Address at which to map the pages
// @mfn:     Array of MFNs to map
// @nr:      Number entries in the MFN array
// @err_ptr: Returns per-MFN error status.
// @prot:    page protection mask
// @domid:   Domain owning the pages
//
// @mfn and @err_ptr may point to the same buffer, the MFNs will be
// overwritten by the error codes after they are mapped.
//
// Returns the number of successfully mapped frames, or a -ve error
// code.
//
// xen_remap_domain_gfn_range() - map a range of foreign frames
// @vma:     VMA to map the pages into
// @addr:    Address at which to map the pages
// @gfn:     First GFN to map.
// @nr:      Number frames to map
// @prot:    page protection mask
// @domid:   Domain owning the pages
// @pages:   Array of pages if this domain has an auto-translated physmap
//
// Returns the number of successfully mapped frames, or a -ve error
// code.
//
extern "C" {
    pub fn xen_remap_pfn(_arg: vma, _arg: addr, _arg: &gfn, _arg: nr, _arg: NULL, _arg: prot, _arg: domid, _arg: false) -> return;
}
extern "C" {
    pub fn xen_running_on_version_or_later(major: c_uint, minor: c_uint) -> bool;
}
extern "C" {
    pub fn xen_efi_runtime_setup();
}

extern "C" {
    pub fn xen_virtio_restricted_mem_acc(dev: *mut virtio_device) -> bool;
}

