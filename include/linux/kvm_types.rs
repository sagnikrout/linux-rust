//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/kvm_types.h
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


// SPDX-License-Identifier: GPL-2.0-only

// Macro flag: #define EXPORT_SYMBOL_FOR_KVM_INTERNAL(symbol)
//
// Allow architectures to provide a custom EXPORT_SYMBOL_FOR_KVM, but only
// if there are no submodules, e.g. to allow suppressing exports if KVM=m, but
// kvm.ko won't actually be built (due to lack of at least one submodule).
//

// Macro flag: #define EXPORT_SYMBOL_FOR_KVM(symbol)

//
// Address types:
//
// gva - guest virtual address
// gpa - guest physical address
// gfn - guest frame number
// hva - host virtual address
// hpa - host physical address
// hfn - host frame number
//
pub type gva_t = c_ulong;
pub type gpa_t = u64;
pub type gfn_t = u64;

pub type hva_t = c_ulong;
pub type hpa_t = u64;
pub type hfn_t = u64;
pub type kvm_pfn_t = hfn_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfn_to_hva_cache {
    pub generation: u64,
    pub gpa: gpa_t,
    pub hva: c_ulong,
    pub len: c_ulong,
    pub memslot: *mut kvm_memory_slot,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gfn_to_pfn_cache {
    pub generation: u64,
    pub gpa: gpa_t,
    pub uhva: c_ulong,
    pub memslot: *mut kvm_memory_slot,
    pub kvm: *mut kvm,
    pub list: list_head,
    pub lock: rwlock_t,
    pub refresh_lock: mutex,
    pub khva: *mut c_void,
    pub pfn: kvm_pfn_t,
    pub active: bool,
    pub valid: bool,
}

//
// Memory caches are used to preallocate memory ahead of various MMU flows,
// e.g. page fault handlers.  Gracefully handling allocation failures deep in
// MMU flows is problematic, as is triggering reclaim, I/O, etc... while
// holding MMU locks.  Note, these caches act more like prefetch buffers than
// classical caches, i.e. objects are not returned to the cache on being freed.
//
// The @capacity field and @objects array are lazily initialized when the cache
// is topped up (__kvm_mmu_topup_memory_cache()).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_mmu_memory_cache {
    pub gfp_zero: gfp_t,
    pub gfp_custom: gfp_t,
    pub init_value: u64,
    pub kmem_cache: *mut kmem_cache,
    pub capacity: c_int,
    pub nobjs: c_int,
    pub objects: *mut c_void,
}

pub const HALT_POLL_HIST_COUNT: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_vm_stat_generic {
    pub remote_tlb_flush: u64,
    pub remote_tlb_flush_requests: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_vcpu_stat_generic {
    pub halt_successful_poll: u64,
    pub halt_attempted_poll: u64,
    pub halt_poll_invalid: u64,
    pub halt_wakeup: u64,
    pub halt_poll_success_ns: u64,
    pub halt_poll_fail_ns: u64,
    pub halt_wait_ns: u64,
    pub halt_poll_success_hist: [u64; HALT_POLL_HIST_COUNT],
    pub halt_poll_fail_hist: [u64; HALT_POLL_HIST_COUNT],
    pub halt_wait_hist: [u64; HALT_POLL_HIST_COUNT],
    pub blocking: u64,
}

pub const KVM_STATS_NAME_SIZE: c_int = 48;

