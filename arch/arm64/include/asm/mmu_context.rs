//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/mmu_context.h
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
// Based on arch/arm/include/asm/mmu_context.h
//
// Copyright (C) 1996 Russell King.
// Copyright (C) 2012 ARM Ltd.
//

//
// Set TTBR0 to reserved_pg_dir. No translations will be possible via TTBR0.
//
extern "C" {
    pub fn cpu_do_switch_mm(pgd_phys: phys_addr_t, mm: *mut mm_struct);
}
//
// Ensure TCR.T0SZ is set to the provided value.
//
// Remove the idmap from TTBR0_EL1 and install the pgd of the active mm.
//
// The idmap lives in the same VA range as userspace, but uses global entries
// and may use a different TCR_EL1.T0SZ. To avoid issues resulting from
// speculative TLB fetches, we must temporarily install the reserved page
// tables while we invalidate the TLBs and set up the correct TCR_EL1.T0SZ.
//
// If current is a not a user task, the mm covers the TTBR1_EL1 page tables,
// which should not be installed in TTBR0_EL1. In this case we can leave the
// reserved page tables in place.
//
// Load our new page tables. A strict BBM approach requires that we ensure that
// TLBs are free of any entries that may overlap with the global mappings we are
// about to install.
//
// For a real hibernate/resume/kexec cycle TTBR0 currently points to a zero
// page, but TLBs may contain stale ASID-tagged entries (e.g. for EFI runtime
// services), while for a userspace-driven test_resume cycle it points to
// userspace page tables (and we must point it at a zero page ourselves).
//
// We change T0SZ as part of installing the idmap. This is undone by
// cpu_uninstall_idmap() in __cpu_suspend_exit().
//
// avoid cpu_switch_mm() and its SW-PAN and CNP interactions
extern "C" {
    pub fn __cpu_replace_ttbr1(pgdp: *mut pgd_t, cnp: bool);
}
//
// Only for early TTBR1 replacement before cpucaps are finalized and
// before we've decided whether to use CNP.
//
// It would be nice to return ASIDs back to the allocator, but unfortunately
// that introduces a race with a generation rollover where we could erroneously
// free an ASID allocated in a future generation. We could workaround this by
// freeing the ASID from the context of the dying mm (e.g. in arch_exit_mmap),
// but we'd then need to make sure that we didn't dirty any TLBs afterwards.
// Setting a reserved TTBR0 or EPD0 would work, but it all gets ugly when you
// take CPU migration into account.
//
extern "C" {
    pub fn check_and_switch_context(mm: *mut mm_struct);
}

// pkey 0 is the default, so always reserve it.
// Duplicate the oldmm pkey state in mm:

//
// We don't actually care about the ttbr0 mapping, so point it at the
// zero page.
//
// init_mm.pgd does not contain any user mappings and it is always
// active for kernel addresses in TTBR1. Just set the reserved TTBR0.
//
// Update the saved TTBR0_EL1 of the scheduled-in task as the previous
// value may have not been initialised yet (activate_mm caller) or the
// ASID has changed since the last run (following the context switch
// of another thread of the same process).
//
extern "C" {
    pub fn system_32bit_el0_cpumask() -> return;
}
extern "C" {
    pub fn __task_cpu_possible_mask(_arg: p, _arg: cpu_possible_mask) -> return;
}

extern "C" {
    pub fn verify_cpu_asid_bits();
}
extern "C" {
    pub fn post_ttbr_update_workaround();
}
extern "C" {
    pub fn arm64_mm_context_get(mm: *mut mm_struct) -> c_ulong;
}
extern "C" {
    pub fn arm64_mm_context_put(mm: *mut mm_struct);
}

//
// Only enforce protection keys on the current process, because there is no
// user context to access POR_EL0 for another address space.
//
// allow access if the VMA is not one from this process
extern "C" {
    pub fn por_el0_allows_pkey(_arg: vma_pkey(vma), _arg: write, _arg: execute) -> return;
}

