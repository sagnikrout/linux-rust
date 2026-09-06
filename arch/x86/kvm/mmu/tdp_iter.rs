//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kvm/mmu/tdp_iter.h
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
// TDP MMU SPTEs are RCU protected to allow paging structures (non-leaf SPTEs)
// to be zapped while holding mmu_lock for read, and to allow TLB flushes to be
// batched without having to collect the list of zapped SPs.  Flows that can
// remove SPs must service pending TLB flushes prior to dropping RCU protection.
//
extern "C" {
    pub fn READ_ONCE(_arg: *mut rcu_dereference(sptep)) -> return;
}
//
// WARNING!  mmu_lock must be held for write when using the "write atomic" or
// "clear bits atomic" APIs, otherwise KVM could overwrite the "wrong" old SPTE
// value, i.e. clobber an update from a different CPU.  The only exception is
// when KVM is freezing a leaf SPTE for removal, in which case KVM doesn't care
// about the exact old SPTE value (KVM will react to the actual old value).
//
extern "C" {
    pub fn xchg(_arg: rcu_dereference(sptep), _arg: new_spte) -> return;
}
//
// SPTEs must be modified atomically if they are shadow-present, leaf SPTEs,
// and have volatile bits (bits that can be set outside of mmu_lock) that
// must not be clobbered.
//
extern "C" {
    pub fn kvm_tdp_mmu_write_spte_atomic(_arg: sptep, _arg: new_spte) -> return;
}
extern "C" {
    pub fn tdp_mmu_clear_spte_bits_atomic(_arg: sptep, _arg: mask) -> return;
}
//
// A TDP iterator performs a pre-order walk over a TDP paging structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tdp_iter {
//
// The iterator will traverse the paging structure towards the mapping
// for this GFN.
//
    pub next_last_level_gfn: gfn_t,
//
// The next_last_level_gfn at the time when the thread last
// yielded. Only yielding when the next_last_level_gfn !=
// yielded_gfn helps ensure forward progress.
//
    pub yielded_gfn: gfn_t,
// Pointers to the page tables traversed to reach the current SPTE
    pub pt_path: [tdp_ptep_t; PT64_ROOT_MAX_LEVEL],
// A pointer to the current SPTE
    pub sptep: tdp_ptep_t,
// The lowest GFN (mask bits excluded) mapped by the current SPTE
    pub gfn: gfn_t,
// Mask applied to convert the GFN to the mapping GPA
    pub gfn_bits: gfn_t,
// The level of the root page given to the iterator
    pub root_level: c_int,
// The lowest level the iterator should traverse to
    pub min_level: c_int,
// The iterator's current level within the paging structure
    pub level: c_int,
// The address space ID, i.e. SMM vs. regular.
    pub as_id: c_int,
// A snapshot of the value at sptep
    pub old_spte: u64,
//
// Whether the iterator has a valid state. This will be false if the
// iterator walks off the end of the paging structure.
//
    pub valid: bool,
//
// True if KVM dropped mmu_lock and yielded in the middle of a walk, in
// which case tdp_iter_next() needs to restart the walk at the root
// level instead of advancing to the next entry.
//
    pub yielded: bool,
}

//
// Iterates over every SPTE mapping the GFN range [start, end) in a
// preorder traversal.
//

extern "C" {
    pub fn spte_to_child_pt(pte: u64, level: c_int) -> tdp_ptep_t;
}
extern "C" {
    pub fn tdp_iter_next(iter: *mut tdp_iter);
}
extern "C" {
    pub fn tdp_iter_restart(iter: *mut tdp_iter);
}
