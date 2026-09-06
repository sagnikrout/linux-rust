//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/nohash/32/pte-8xx.h
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

// Macro flag: #define _ASM_POWERPC_NOHASH_32_PTE_8xx_H

//
// The PowerPC MPC8xx uses a TLB with hardware assisted, software tablewalk.
// We also use the two level tables, but we can put the real bits in them
// needed for the TLB and tablewalk.  These definitions require Mx_CTR.PPM = 0,
// Mx_CTR.PPCS = 0, and MD_CTR.TWAM = 1.  The level 2 descriptor has
// additional page protection (when Mx_CTR.PPCS = 1) that allows TLB hit
// based upon user/super access.  The TLB does not have accessed nor write
// protect.  We assume that if the TLB get loaded with an entry it is
// accessed, and overload the changed bit for write protect.  We use
// two bits in the software pte that are supposed to be set to zero in
// the TLB entry (24 and 25) for these indicators.  Although the level 1
// descriptor contains the guarded and writethrough/copyback bits, we can
// set these at the page level since they get copied from the Mx_TWC
// register when the TLB entry is loaded.  We will use bit 27 for guard, since
// that is where it exists in the MD_TWC, and bit 26 for writethrough.
// These will get masked from the level 2 descriptor at TLB load time, and
// copied to the MD_TWC before it gets loaded.
// Large page sizes added.  We currently support two sizes, 4K and 8M.
// This also allows a TLB hander optimization because we can directly
// load the PMD into MD_TWC.  The 8M pages are only used for kernel
// mapping of well known areas.  The PMD (PGD) entries contain control
// flags in addition to the address, so care must be taken that the
// software no longer assumes these are only pointers.
//
// Definitions for 8xx embedded chips.
pub const _PAGE_PRESENT: c_uint = 0x0001	/* V: Page is valid */;
pub const _PAGE_NO_CACHE: c_uint = 0x0002	/* CI: cache inhibit */;
pub const _PAGE_SH: c_uint = 0x0004	/* SH: No ASID (context) compare */;
pub const _PAGE_SPS: c_uint = 0x0008	/* SPS: Small Page Size (1 if 16k, 512k or 8M)*/;
pub const _PAGE_DIRTY: c_uint = 0x0100	/* C: page changed */;
// These 4 software bits must be masked out when the L2 entry is loaded
// into the TLB.
//
pub const _PAGE_GUARDED: c_uint = 0x0010	/* Copied to L1 G entry in DTLB */;
pub const _PAGE_ACCESSED: c_uint = 0x0020	/* Copied to L1 APG 1 entry in I/DTLB */;
pub const _PAGE_EXEC: c_uint = 0x0040	/* Copied to PP (bit 21) in ITLB */;
pub const _PAGE_SPECIAL: c_uint = 0x0080	/* SW entry */;
pub const _PAGE_NA: c_uint = 0x0200	/* Supervisor NA, User no access */;
pub const _PAGE_RO: c_uint = 0x0600	/* Supervisor RO, User no access */;
pub const _PAGE_HUGE: c_uint = 0x0800	/* Copied to L1 PS bit 29 */;

pub const _PAGE_RW: c_int = 0;

// cache related flags non existing on 8xx
pub const _PAGE_COHERENT: c_int = 0;
pub const _PAGE_WRITETHRU: c_int = 0;

pub const _PMD_PRESENT: c_uint = 0x0001;

pub const _PMD_BAD: c_uint = 0x0f90;
pub const _PMD_PAGE_MASK: c_uint = 0x000c;
pub const _PMD_PAGE_8M: c_uint = 0x000c;
pub const _PMD_PAGE_512K: c_uint = 0x0004;
pub const _PMD_ACCESSED: c_uint = 0x0020	/* APG 1 */;
pub const _PMD_USER: c_uint = 0x0040	/* APG 2 */;
pub const _PTE_NONE_MASK: c_int = 0;

extern "C" {
    pub fn __pte(_PAGE_RO: pte_val(pte) |) -> return;
}

extern "C" {
    pub fn __pte(~_PAGE_RO: pte_val(pte) &) -> return;
}

extern "C" {
    pub fn __pte(_PAGE_HUGE: pte_val(pte) | _PAGE_SPS |) -> return;
}

//
// On the 8xx, the page tables are a bit special. For 16k pages, we have
// 4 identical entries. For 512k pages, we have 128 entries as if it was
// 4k pages, but they are flagged as 512k pages for the hardware.
// For 8M pages, we have 1024 entries as if it was 4M pages (PMD_SIZE)
// but they are flagged as 8M pages for the hardware.
// For 4k pages, we have a single entry in the table.
//
// entry++ = new;

