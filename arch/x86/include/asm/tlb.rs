//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/tlb.h
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
    pub fn tlb_flush(tlb: *mut mmu_gather);
}

extern "C" {
    pub fn volatile("memory": "invlpg (%0)" ::"r" (addr) :) -> asm;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum addr_stride {
    PTE_STRIDE = 0,
    PMD_STRIDE = 1
}

//
// INVLPGB can be targeted by virtual address, PCID, ASID, or any combination
// of the three. For example:
// - FLAG_VA | FLAG_INCLUDE_GLOBAL: invalidate all TLB entries at the address
// - FLAG_PCID:			    invalidate all TLB entries matching the PCID
//
// The first is used to invalidate (kernel) mappings at a particular
// address across all processes.
//
// The latter invalidates all TLB entries matching a PCID.
//

// The implied mode when all bits are clear:

//
// INVLPGB does broadcast TLB invalidation across all the CPUs in the system.
//
// The INVLPGB instruction is weakly ordered, and a batch of invalidations can
// be done in a parallel fashion.
//
// The instruction takes the number of extra pages to invalidate, beyond the
// first page, while __invlpgb gets the more human readable number of pages to
// invalidate.
//
// The bits in rax[0:2] determine respectively which components of the address
// (VA, PCID, ASID) get compared when flushing. If neither bits are set, *any
// address in the specified range matches.
//
// Since it is desired to only flush TLB entries for the ASID that is executing
// the instruction (a host/hypervisor or a guest), the ASID valid bit should
// always be set. On a host/hypervisor, the hardware will use the ASID value
// specified in EDX[15:0] (which should be 0). On a guest, the hardware will
// use the actual ASID value of the guest.
//
// TLBSYNC is used to ensure that pending INVLPGB invalidations initiated from
// this CPU have completed.
//
// The low bits in rax are for flags. Verify addr is clean.
// INVLPGB; supported in binutils >= 2.36.
extern "C" {
    pub fn volatile(0x0f: ".byte, _arg: 0x01, (rax): 0xfe" :: "a", (ecx): "c", (edx): "d") -> asm;
}
//
// TLBSYNC waits for INVLPGB instructions originating on the same CPU
// to have completed. Print a warning if the task has been migrated,
// and might not be waiting on all the INVLPGBs issued during this TLB
// invalidation sequence.
//
// TLBSYNC: supported in binutils >= 0.36.
extern "C" {
    pub fn volatile(0x0f: ".byte, _arg: 0x01, "memory": 0xff" :::) -> asm;
}

// Some compilers (I'm looking at you clang!) simply can't do DCE

// Flush all mappings for a given PCID, not including globals.
// Flush all mappings, including globals, for all PCIDs.
//
// TLBSYNC at the end needs to make sure all flushes done on the
// current CPU have been executed system-wide. Therefore, make
// sure nothing gets migrated in-between but disable preemption
// as it is cheaper.
//
// Flush addr, including globals, for all PCIDs.
// Flush all mappings for all PCIDs except globals.
