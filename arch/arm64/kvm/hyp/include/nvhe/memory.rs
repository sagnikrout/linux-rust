//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/kvm/hyp/include/nvhe/memory.h
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

//
// Bits 0-1 are used to encode the memory ownership state of each page from the
// point of view of a pKVM "component" (host, hyp, guest, ... see enum
// pkvm_component_id):
// 00: The page is owned and exclusively accessible by the component;
// 01: The page is owned and accessible by the component, but is also
// accessible by another component;
// 10: The page is accessible but not owned by the component;
// The storage of this state depends on the component: either in the
// hyp_vmemmap for the host and hyp states or in PTE software bits for guests.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pkvm_page_state {
    PKVM_PAGE_OWNED			= 0ULL,
    PKVM_PAGE_SHARED_OWNED		= BIT(0),
    PKVM_PAGE_SHARED_BORROWED	= BIT(1),

//
// 'Meta-states' are not stored directly in PTE SW bits for guest
// states, but inferred from the context (e.g. invalid PTE entries).
// For the host and hyp, meta-states are stored directly in the
// struct hyp_page.
//
    PKVM_NOPAGE			= BIT(0) | BIT(1),

//
// 'Meta-states' which aren't encoded directly in the PTE's SW bits (or
// the hyp_vmemmap entry for the host)
//
    PKVM_POISON			= BIT(2),
}

extern "C" {
    pub fn FIELD_GET(_arg: PKVM_PAGE_STATE_PROT_MASK, _arg: prot) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hyp_page {
    pub refcount: u16,
    pub order: u8,
// Host state. Guarded by the host stage-2 lock.
    pub 4: unsigned __host_state :,
//
// Complement of the hyp state. Guarded by the hyp stage-1 lock. We use
// the complement so that the initial 0 in __hyp_state_comp (due to the
// entire vmemmap starting off zeroed) encodes PKVM_NOPAGE.
//
    pub 4: unsigned __hyp_state_comp :,
    pub host_share_guest_count: u32,
}

extern "C" {
    pub fn __hyp_va(_arg: phys) -> return;
}
extern "C" {
    pub fn __hyp_pa(_arg: addr) -> return;
}

//
// Refcounting for 'struct hyp_page'.
// hyp_pool::lock must be held if atomic access to the refcount is required.
//
