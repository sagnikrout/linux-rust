//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/mmu_context.h
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
    pub fn cr4_update_pce(ignored: *mut c_void);
}

//
// ldt_structs can be allocated, used, and freed, but they are never
// modified while live.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ldt_struct {
//
// Xen requires page-aligned LDTs with special permissions.  This is
// needed to prevent us from installing evil descriptors such as
// call gates.  On native, we could merge the ldt_struct and LDT
// allocations, but it's not worth trying to optimize.
//
    pub entries: *mut desc_struct,
    pub nr_entries: c_uint,
//
// If PTI is in use, then the entries array is not mapped while we're
// in user mode.  The whole array will be aliased at the addressed
// given by ldt_slot_va(slot).  We use two slots so that we can allocate
// and map, and enable a new LDT without invalidating the mapping
// of an older, still-in-use LDT.
//
// slot will be -1 if this LDT doesn't have an alias mapping.
//
    pub slot: c_int,
}

//
// Used for LDT copy/destruction.
//
extern "C" {
    pub fn ldt_dup_context(oldmm: *mut mm_struct, mm: *mut mm_struct) -> c_int;
}
extern "C" {
    pub fn destroy_context_ldt(mm: *mut mm_struct);
}
extern "C" {
    pub fn ldt_arch_exit_mmap(mm: *mut mm_struct);
}

extern "C" {
    pub fn load_mm_ldt(mm: *mut mm_struct);
}
extern "C" {
    pub fn switch_ldt(prev: *mut mm_struct, next: *mut mm_struct);
}

//
// When switch_mm_irqs_off() is called for a kthread, it may race with
// LAM enablement. switch_mm_irqs_off() uses the LAM mask to do two
// things: populate CR3 and populate 'cpu_tlbstate.lam'. Make sure it
// reads a single value for both.
//
extern "C" {
    pub fn READ_ONCE(_arg: mm->context.lam_cr3_mask) -> return;
}

extern "C" {
    pub fn mm_init_global_asid(mm: *mut mm_struct);
}
extern "C" {
    pub fn mm_free_global_asid(mm: *mut mm_struct);
}
//
// Init a new mm.  Used on mm copies, like at fork()
// and on mm's that are brand-new, like at execve().
//

// pkey 0 is the default and allocated implicitly
// -1 means unallocated or invalid

// Duplicate the oldmm pkey state in mm:

extern "C" {
    pub fn ldt_dup_context(_arg: oldmm, _arg: mm) -> return;
}

extern "C" {
    pub fn test_bit(_arg: MM_CONTEXT_NOTRACK, _arg: &mm->context.flags) -> return;
}
//
// We only want to enforce protection keys on the current process
// because we effectively have no access to PKRU for other
// processes or any way to tell *which * PKRU in a threaded
// process we could use.
//
// So do not enforce things if the VMA is not from the current
// mm, or if we are in a kernel thread.
//
// pkeys never affect instruction fetches
// allow access if the VMA is not one from this process
extern "C" {
    pub fn __pkru_allows_pkey(_arg: vma_pkey(vma), _arg: write) -> return;
}
extern "C" {
    pub fn __get_current_cr3_fast() -> c_ulong;
}

extern "C" {
    pub fn unuse_temporary_mm(prev_mm: *mut mm_struct);
}
