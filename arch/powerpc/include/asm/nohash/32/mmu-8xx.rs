//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/nohash/32/mmu-8xx.h
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
// PPC8xx support
//
// Control/status registers for the MPC8xx.
// A write operation to these registers causes serialized access.
// During software tablewalk, the registers used perform mask/shift-add
// operations when written/read.  A TLB entry is created when the Mx_RPN
// is written, and the contents of several registers are used to
// create the entry.
//

pub const MI_GPM: c_uint = 0x80000000	/* Set domain manager mode */;
pub const MI_PPM: c_uint = 0x40000000	/* Set subpage protection */;
pub const MI_CIDEF: c_uint = 0x20000000	/* Set cache inhibit when MMU dis */;
pub const MI_RSV4I: c_uint = 0x08000000	/* Reserve 4 TLB entries */;
pub const MI_PPCS: c_uint = 0x02000000	/* Use MI_RPN prob/priv state */;
pub const MI_IDXMASK: c_uint = 0x00001f00	/* TLB index to be loaded */;
// These are the Ks and Kp from the PowerPC books.  For proper operation,
// Ks = 0, Kp = 1.
//
pub const SPRN_MI_AP: c_int = 786;
pub const MI_Ks: c_uint = 0x80000000	/* Should not be set */;
pub const MI_Kp: c_uint = 0x40000000	/* Should always be set */;
//
// All pages' PP data bits are set to either 001 or 011 by copying _PAGE_EXEC
// into bit 21 in the ITLBmiss handler (bit 21 is the middle bit), which means
// respectively NA for All or X for Supervisor and no access for User.
// Then we use the APG to say whether accesses are according to Page rules or
// "all Supervisor" rules (Access to all)
// _PAGE_ACCESSED is also managed via APG. When _PAGE_ACCESSED is not set, say
// "all User" rules, that will lead to NA for all.
// Therefore, we define 4 APG groups. lsb is _PAGE_ACCESSED
// 0 => Kernel => 11 (all accesses performed according as user iaw page definition)
// 1 => Kernel+Accessed => 01 (all accesses performed according to page definition)
// 2 => User => 11 (all accesses performed according as user iaw page definition)
// 3 => User+Accessed => 10 (all accesses performed according to swaped page definition) for KUEP
// 4-15 => Not Used
//
pub const MI_APG_INIT: c_uint = 0xde000000;
// The effective page number register.  When read, contains the information
// about the last instruction TLB miss.  When MI_RPN is written, bits in
// this register are used to create the TLB entry.
//
pub const SPRN_MI_EPN: c_int = 787;
pub const MI_EPNMASK: c_uint = 0xfffff000	/* Effective page number for entry */;
pub const MI_EVALID: c_uint = 0x00000200	/* Entry is valid */;
pub const MI_ASIDMASK: c_uint = 0x0000000f	/* ASID match value */;
// Reset value is undefined
// A "level 1" or "segment" or whatever you want to call it register.
// For the instruction TLB, it contains bits that get loaded into the
// TLB entry when the MI_RPN is written.
//
pub const SPRN_MI_TWC: c_int = 789;
pub const MI_APG: c_uint = 0x000001e0	/* Access protection group (0) */;
pub const MI_GUARDED: c_uint = 0x00000010	/* Guarded storage */;
pub const MI_PSMASK: c_uint = 0x0000000c	/* Mask of page size bits */;
pub const MI_PS8MEG: c_uint = 0x0000000c	/* 8M page size */;
pub const MI_PS512K: c_uint = 0x00000004	/* 512K page size */;
pub const MI_PS4K_16K: c_uint = 0x00000000	/* 4K or 16K page size */;
pub const MI_SVALID: c_uint = 0x00000001	/* Segment entry is valid */;
// Reset value is undefined
// Real page number.  Defined by the pte.  Writing this register
// causes a TLB entry to be created for the instruction TLB, using
// additional information from the MI_EPN, and MI_TWC registers.
//
pub const SPRN_MI_RPN: c_int = 790;
pub const MI_SPS16K: c_uint = 0x00000008	/* Small page size (0 = 4k, 1 = 16k) */;
// Define an RPN value for mapping kernel memory to large virtual
// pages for boot initialization.  This has real page number of 0,
// large page size, shared page, cache enabled, and valid.
// Also mark all subpages valid and write access.
//
pub const MI_BOOTINIT: c_uint = 0x000001fd;

pub const MD_GPM: c_uint = 0x80000000	/* Set domain manager mode */;
pub const MD_PPM: c_uint = 0x40000000	/* Set subpage protection */;
pub const MD_CIDEF: c_uint = 0x20000000	/* Set cache inhibit when MMU dis */;
pub const MD_WTDEF: c_uint = 0x10000000	/* Set writethrough when MMU dis */;
pub const MD_RSV4I: c_uint = 0x08000000	/* Reserve 4 TLB entries */;
pub const MD_TWAM: c_uint = 0x04000000	/* Use 4K page hardware assist */;
pub const MD_PPCS: c_uint = 0x02000000	/* Use MI_RPN prob/priv state */;
pub const MD_IDXMASK: c_uint = 0x00001f00	/* TLB index to be loaded */;

pub const MC_ASIDMASK: c_uint = 0x0000000f	/* Bits used for ASID value */;
// These are the Ks and Kp from the PowerPC books.  For proper operation,
// Ks = 0, Kp = 1.
//
pub const SPRN_MD_AP: c_int = 794;
pub const MD_Ks: c_uint = 0x80000000	/* Should not be set */;
pub const MD_Kp: c_uint = 0x40000000	/* Should always be set */;
// See explanation above at the definition of MI_APG_INIT
pub const MD_APG_INIT: c_uint = 0xdc000000;
pub const MD_APG_KUAP: c_uint = 0xde000000;
// The effective page number register.  When read, contains the information
// about the last instruction TLB miss.  When MD_RPN is written, bits in
// this register are used to create the TLB entry.
//
pub const SPRN_MD_EPN: c_int = 795;
pub const MD_EPNMASK: c_uint = 0xfffff000	/* Effective page number for entry */;
pub const MD_EVALID: c_uint = 0x00000200	/* Entry is valid */;
pub const MD_ASIDMASK: c_uint = 0x0000000f	/* ASID match value */;
// Reset value is undefined
// The pointer to the base address of the first level page table.
// During a software tablewalk, reading this register provides the address
// of the entry associated with MD_EPN.
//
pub const SPRN_M_TWB: c_int = 796;
pub const M_L1TB: c_uint = 0xfffff000	/* Level 1 table base address */;
pub const M_L1INDX: c_uint = 0x00000ffc	/* Level 1 index, when read */;
// Reset value is undefined
// A "level 1" or "segment" or whatever you want to call it register.
// For the data TLB, it contains bits that get loaded into the TLB entry
// when the MD_RPN is written.  It is also provides the hardware assist
// for finding the PTE address during software tablewalk.
//
pub const SPRN_MD_TWC: c_int = 797;
pub const MD_L2TB: c_uint = 0xfffff000	/* Level 2 table base address */;
pub const MD_L2INDX: c_uint = 0xfffffe00	/* Level 2 index (*pte), when read */;
pub const MD_APG: c_uint = 0x000001e0	/* Access protection group (0) */;
pub const MD_GUARDED: c_uint = 0x00000010	/* Guarded storage */;
pub const MD_PSMASK: c_uint = 0x0000000c	/* Mask of page size bits */;
pub const MD_PS8MEG: c_uint = 0x0000000c	/* 8M page size */;
pub const MD_PS512K: c_uint = 0x00000004	/* 512K page size */;
pub const MD_PS4K_16K: c_uint = 0x00000000	/* 4K or 16K page size */;
pub const MD_WT: c_uint = 0x00000002	/* Use writethrough page attribute */;
pub const MD_SVALID: c_uint = 0x00000001	/* Segment entry is valid */;
// Reset value is undefined
// Real page number.  Defined by the pte.  Writing this register
// causes a TLB entry to be created for the data TLB, using
// additional information from the MD_EPN, and MD_TWC registers.
//
pub const SPRN_MD_RPN: c_int = 798;
pub const MD_SPS16K: c_uint = 0x00000008	/* Small page size (0 = 4k, 1 = 16k) */;
// This is a temporary storage register that could be used to save
// a processor working register during a tablewalk.
//
pub const SPRN_M_TW: c_int = 799;

pub const PTE_FRAG_NR: c_int = 4;
pub const PTE_FRAG_SIZE_SHIFT: c_int = 12;

extern "C" {
    pub fn mmu_pin_tlb(top: c_ulong, readonly: bool);
}

//
// Page size definitions for 8xx
//
// shift : is the "PAGE_SHIFT" value for that page size
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmu_psize_def {
    pub /: *mut *mut unsigned int shift; / number of bits,
}

// patch sites

