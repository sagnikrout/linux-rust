//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/book3s/64/mmu.h
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
// Page size definition
//
// shift : is the "PAGE_SHIFT" value for that page size
// sllp  : is a bit mask with the value of SLB L || LP to be or'ed
// directly to a slbmte "vsid" value
// penc  : is the HPTE encoding mask for the "LP" field:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmu_psize_def {
    pub /: *mut *mut unsigned int shift; / number of bits,
    pub /: *mut *mut int penc[MMU_PAGE_COUNT]; / HPTE encoding,
    pub /: *mut *mut unsigned int tlbiel; / tlbiel supported for that page size,
    pub /: *mut *mut unsigned long avpnm; / bits to mask out in AVPN in the HPTE,
    pub /: *mut *mut unsigned long h_rpt_pgsize; / H_RPT_INVALIDATE page size encoding,
    pub /: *mut *mut unsigned long sllp; / SLB L||LP (exact mask to use in slbmte),
    pub /: *mut *mut unsigned long ap; / Ap encoding used by PowerISA 3.0,
}

// 64-bit classic hash table MMU

//
// ISA 3.0 partition and process table entry format
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prtb_entry {
    pub prtb0: __be64,
    pub prtb1: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct patb_entry {
    pub patb0: __be64,
    pub patb1: __be64,
}

// Bits in patb0 field

pub const RPDB_MASK: c_uint = 0x0fffffffffffff00UL;

pub const RPDS_MASK: c_uint = 0x1f		/* root page dir. size field */;
// Bits in patb1 field

pub const PRTS_MASK: c_uint = 0x1f		/* process table size field */;
pub const PRTB_MASK: c_uint = 0x0ffffffffffff000UL;
// Number of supported LPID bits
// Number of supported PID bits
// Base PID to allocate from

pub type mm_context_id_t = c_ulong;
// Maximum possible number of NPUs in a system.
pub const NV_MAX_NPUS: c_int = 8;
//
// We use id as the PIDR content for radix. On hash we can use
// more than one id. The extended ids are used when we start
// having address above 512TB. We allocate one extended id
// for each 512TB. The new id is then used with the 49 bit
// EA to build a new VA. We always use ESID_BITS_1T_MASK bits
// from EA and new context ids to build the new VAs.
//

// Number of bits in the mm_cpumask
// Number of users of the external (Nest) MMU
// Number of user space windows opened in process mm_context

//
// pagetable fragment support
//

//
// Each bit represents one protection key.
// bit set   -> key allocated
// bit unset -> key available for allocation
//

//
// The current system page and segment sizes
//

// MMU initialization
extern "C" {
    pub fn mmu_early_init_devtree();
}
extern "C" {
    pub fn hash__early_init_devtree();
}
extern "C" {
    pub fn radix__early_init_devtree();
}

extern "C" {
    pub fn pkey_early_init_devtree();
}

extern "C" {
    pub fn hash__early_init_mmu();
}
extern "C" {
    pub fn radix__early_init_mmu();
}
extern "C" {
    pub fn radix__early_init_mmu() -> return;
}
extern "C" {
    pub fn hash__early_init_mmu() -> return;
}
extern "C" {
    pub fn hash__early_init_mmu_secondary();
}
extern "C" {
    pub fn radix__early_init_mmu_secondary();
}
extern "C" {
    pub fn radix__early_init_mmu_secondary() -> return;
}
extern "C" {
    pub fn hash__early_init_mmu_secondary() -> return;
}
//
// Hash has more strict restrictions. At this point we don't
// know which translations we will pick. Hence go with hash
// restrictions.
//

extern "C" {
    pub fn radix_init_pseries() -> void __init;
}

extern "C" {
    pub fn cleanup_cpu_mmu_context();
}

// should never happen
extern "C" {
    pub fn get_vsid(_arg: context, _arg: ea, _arg: ssize) -> return;
}

