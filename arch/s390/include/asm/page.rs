//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/page.h
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
// S390 version
// Copyright IBM Corp. 1999, 2000
// Author(s): Hartmut Penner (hp@de.ibm.com)
//

// storage-protection override
pub const PAGE_SPO_ACC: c_int = 9;

pub const HPAGE_SHIFT: c_int = 20;

pub const HUGE_MAX_HSTATE: c_int = 2;
// Macro flag: #define ARCH_HAS_SETCLEAR_HUGE_PTE
// Macro flag: #define ARCH_HAS_HUGE_PTE_TYPE
// Macro flag: #define ARCH_HAS_PREPARE_HUGEPAGE
// Macro flag: #define ARCH_HAS_HUGEPAGE_CLEAR_FLUSH
// Macro flag: #define HAVE_ARCH_HUGETLB_UNMAPPED_AREA

extern "C" {
    pub fn __storage_key_init_range(start: c_ulong, end: c_ulong);
}

//
// copy_page uses the mvcl instruction with 0xb0 padding byte in order to
// bypass caches when copying a page. Especially when copying huge pages
// this keeps L1 and L2 data caches alive.
//

// Macro flag: #define STRICT_MM_TYPECHECKS

pub type pgprot_t = c_ulong;
pub type pte_t = c_ulong;
pub type pmd_t = c_ulong;
pub type pud_t = c_ulong;
pub type p4d_t = c_ulong;
pub type pgd_t = c_ulong;

extern "C" {
    pub fn volatile(%0: "iske, (addr): %1" : "=d" (skey) : "a") -> asm;
}
extern "C" {
    pub fn CC_TRANSFORM(_arg: cc) -> return;
}
extern "C" {
    pub fn split_pud_page(pudp: *mut pud_t, addr: c_ulong) -> c_int;
}
// Bits int the storage key
pub const _PAGE_CHANGED: c_uint = 0x02	/* HW changed bit		*/;
pub const _PAGE_REFERENCED: c_uint = 0x04	/* HW referenced bit		*/;
pub const _PAGE_FP_BIT: c_uint = 0x08	/* HW fetch protection bit	*/;
pub const _PAGE_ACC_BITS: c_uint = 0xf0	/* HW access control bits	*/;
extern "C" {
    pub fn arch_free_page(page: *mut page, order: c_int);
}
extern "C" {
    pub fn arch_alloc_page(page: *mut page, order: c_int);
}
// Macro flag: #define HAVE_ARCH_FREE_PAGE
// Macro flag: #define HAVE_ARCH_ALLOC_PAGE
extern "C" {
    pub fn arch_make_folio_accessible(folio: *mut folio) -> c_int;
}
// Macro flag: #define HAVE_ARCH_MAKE_FOLIO_ACCESSIBLE
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vm_layout {
    pub kaslr_offset: c_ulong,
    pub kaslr_offset_phys: c_ulong,
    pub identity_base: c_ulong,
    pub identity_size: c_ulong,
}

extern "C" {
    pub fn __phys_addr(x: c_ulong, is_31bit: bool) -> c_ulong;
}

extern "C" {
    pub fn __pa_nodebug(_arg: x) -> return;
}

extern "C" {
    pub fn __va(_arg: pfn_to_phys(pfn)) -> return;
}
extern "C" {
    pub fn phys_to_pfn(_arg: __pa(kaddr)) -> return;
}

pub const TEXT_OFFSET: c_uint = 0x100000;
