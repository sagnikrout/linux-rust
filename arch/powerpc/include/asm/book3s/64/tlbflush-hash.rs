//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/book3s/64/tlbflush-hash.h
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
// TLB flushing for 64-bit hash-MMU CPUs
//

pub const PPC64_TLB_BATCH_NR: c_int = 192;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ppc64_tlb_batch {
    pub index: c_ulong,
    pub mm: *mut mm_struct,
    pub pte: [real_pte_t; PPC64_TLB_BATCH_NR],
    pub vpn: [c_ulong; PPC64_TLB_BATCH_NR],
    pub psize: c_uint,
    pub ssize: c_int,
}

extern "C" {
    pub fn __flush_tlb_pending(batch: *mut ppc64_tlb_batch);
}
//
// apply_to_page_range can call us this preempt enabled when
// operating on kernel page tables.
//
extern "C" {
    pub fn hash__tlbiel_all(action: c_uint);
}
extern "C" {
    pub fn flush_hash_range(number: c_ulong, local: c_int);
}
extern "C" {
    pub fn hash__tlb_flush(tlb: *mut mmu_gather);
}

// Private function for use by PCI IO mapping code
extern "C" {
    pub fn __flush_hash_table_range(start: c_ulong, end: c_ulong);
}
extern "C" {
    pub fn flush_hash_table_pmd_range(mm: *mut mm_struct, pmd: *mut pmd_t, addr: c_ulong);
}

