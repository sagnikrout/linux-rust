//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/mmu.h
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

// Uprobes on this MM assume 32-bit code
pub const MM_CONTEXT_UPROBE_IA32: c_int = 0;
// vsyscall page is accessible on this MM
pub const MM_CONTEXT_HAS_VSYSCALL: c_int = 1;
// Do not allow changing LAM mode
pub const MM_CONTEXT_LOCK_LAM: c_int = 2;
// Allow LAM and SVA coexisting
pub const MM_CONTEXT_FORCE_TAGGED_SVA: c_int = 3;
// Tracks mm_cpumask
pub const MM_CONTEXT_NOTRACK: c_int = 4;
//
// x86 has arch-specific MMU state beyond what lives in mm_struct.
//
// ctx_id uniquely identifies this mm_struct.  A ctx_id will never
// be reused, and zero is not a valid ctx_id.
//
// Any code that needs to do any sort of TLB flushing for this
// mm will first make its changes to the page tables, then
// increment tlb_gen, then flush.  This lets the low-level
// flushing code keep track of what needs flushing.
//
// This is not used on Xen PV.
//

// Active LAM mode:  X86_CR3_LAM_U48 or X86_CR3_LAM_U57 or 0 (disabled)
// Significant bits of the virtual address. Excludes tag bits.

//
// One bit per protection key says whether userspace can
// use it or not.  protected by mmap_lock.
//

//
// The global ASID will be a non-zero value when the process has
// the same ASID across all CPUs, allowing it to make use of
// hardware-assisted remote TLB invalidation like AMD INVLPGB.
//
// The process is transitioning to a new global ASID number.

extern "C" {
    pub fn leave_mm();
}

