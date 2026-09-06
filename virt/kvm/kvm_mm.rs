//! Automatically rewritten from C Header to Rust Module
//! Source: virt/kvm/kvm_mm.h
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
pub const __KVM_MM_H__: c_int = 1;

//
// Architectures can choose whether to use an rwlock or spinlock
// for the mmu_lock.  These macros, for use in common code
// only, avoids using #ifdefs in places that must deal with
// multiple architectures.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_follow_pfn {
    pub slot: *const kvm_memory_slot,
    pub gfn: gfn_t,
    pub hva: c_ulong,
// FOLL_* flags modifying lookup behavior, e.g. FOLL_WRITE.
    pub flags: c_uint,
//
// Pin the page (effectively FOLL_PIN, which is an mm/ internal flag).
// The page *must* be pinned if KVM will write to the page via a kernel
// mapping, e.g. via kmap(), mremap(), etc.
//
    pub pin: bool,
//
// If non-NULL, try to get a writable mapping even for a read fault.
// Set to true if a writable mapping was obtained.
//
    pub map_writable: *mut bool,
//
// Optional output.  Set to a valid "struct page" if the returned pfn
// is for a refcounted or pinned struct page, NULL if the returned pfn
// has no struct page or if the struct page is not being refcounted
// (e.g. tail pages of non-compound higher order allocations from
// IO/PFNMAP mappings).
//
    pub refcounted_page: *mut page,
}

extern "C" {
    pub fn hva_to_pfn(kfp: *mut kvm_follow_pfn) -> kvm_pfn_t;
}

