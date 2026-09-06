//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/mmu_context.h
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
// Most if the context management is out of line
//

extern "C" {
    pub fn init_new_context(tsk: *mut task_struct, mm: *mut mm_struct) -> c_int;
}

extern "C" {
    pub fn destroy_context(mm: *mut mm_struct);
}

extern "C" {
    pub fn mm_iommu_preregistered(mm: *mut mm_struct) -> bool;
}
extern "C" {
    pub fn mm_iommu_init(mm: *mut mm_struct);
}
extern "C" {
    pub fn mm_iommu_mapped_inc(mem: *mut mm_iommu_table_group_mem_t) -> c_long;
}
extern "C" {
    pub fn mm_iommu_mapped_dec(mem: *mut mm_iommu_table_group_mem_t);
}

extern "C" {
    pub fn switch_slb(tsk: *mut task_struct, mm: *mut mm_struct);
}

extern "C" {
    pub fn radix__switch_mmu_context(_arg: prev, _arg: next) -> return;
}
extern "C" {
    pub fn switch_slb(_arg: tsk, _arg: next) -> return;
}
extern "C" {
    pub fn hash__alloc_context_id() -> c_int;
}
extern "C" {
    pub fn hash__reserve_context_id(id: c_int) -> void __init;
}
extern "C" {
    pub fn __destroy_context(context_id: c_int);
}

extern "C" {
    pub fn __init_new_context() -> c_ulong;
}
extern "C" {
    pub fn __destroy_context(context_id: c_ulong);
}
extern "C" {
    pub fn mmu_context_init();
}
// non book3s_64 should never find this called

//
// If any copro is in use, increment the active CPU count
// in order to force TLB invalidations to be global as to
// propagate to the Nest MMU.
//
// When removing the last copro, we need to broadcast a global
// flush of the full mm, as the next TLBI may be local and the
// nMMU and/or PSL need to be cleaned up.
//
// Both the 'copros' and 'active_cpus' counts are looked at in
// radix__flush_all_mm() to determine the scope (local/global)
// of the TLBIs, so we need to flush first before decrementing
// 'copros'. If this API is used by several callers for the
// same context, it can lead to over-flushing. It's hopefully
// not common enough to be a problem.
//
// Skip on hash, as we don't know how to do the proper flush
// for the time being. Invalidations will remain global if
// used on hash. Note that we can't drop 'copros' either, as
// it could make some invalidations local with no flush
// in-between.
//
// Detect imbalance between add and remove
//
// vas_windows counter shows number of open windows in the mm
// context. During context switch, use this counter to clear the
// foreign real address mapping (CP_ABORT) for the thread / process
// that intend to use COPY/PASTE. When a process closes all windows,
// disable CP_ABORT which is expensive to run.
//
// For user context, register a copro so that TLBIs are seen by the
// nest MMU. mm_context_add/remove_vas_window() are used only for user
// space windows.
//
// Detect imbalance between add and remove

//
// After we have set current->mm to a new value, this activates
// the context for the new mm so we see the new mappings.
//

// We don't currently use enter_lazy_tlb() for anything

// 64-bit Book3E keeps track of current PGD in the PACA

extern "C" {
    pub fn arch_exit_mmap(mm: *mut mm_struct);
}

extern "C" {
    pub fn arch_dup_pkeys(oldmm: *mut mm_struct, mm: *mut mm_struct);
}

// by default, allow everything
// Macro flag: #define pkey_mm_init(mm)

