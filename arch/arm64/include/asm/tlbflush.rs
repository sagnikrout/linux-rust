//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/tlbflush.h
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
// Based on arch/arm/include/asm/tlbflush.h
//
// Copyright (C) 1999-2003 Russell King
// Copyright (C) 2012 ARM Ltd.
//

//
// Raw TLBI operations.
//
// Where necessary, use the __tlbi() macro to avoid asm()
// boilerplate. Drivers and most kernel code should use the TLB
// management routines in preference to the macro below.
//
// The macro can be used as __tlbi(op) or __tlbi(op, arg), depending
// on whether a particular TLBI operation takes an argument or
// not. The macros handles invoking the asm with or without the
// register argument as appropriate.
//

// This macro creates a properly formatted VA operand for the TLBI

//
// Get translation granule of the system, which is decided by
// PAGE_SIZE.  Used by TTL.
// - 4KB	: 1
// - 16KB	: 2
// - 64KB	: 3
//
pub const TLBI_TTL_TG_4K: c_int = 1;
pub const TLBI_TTL_TG_16K: c_int = 2;
pub const TLBI_TTL_TG_64K: c_int = 3;

extern "C" {
    pub fn sme_do_dvmsync(mask: *const cpumask);
}

//
// Level-based TLBI operations.
//
// When ARMv8.4-TTL exists, TLBI operations take an additional hint for
// the level at which the invalidation must take place. If the level is
// wrong, no invalidation may take place. In the case where the level
// cannot be easily determined, the value TLBI_TTL_UNKNOWN will perform
// a non-hinted invalidation. Any provided level outside the hint range
// will also cause fall-back to non-hinted invalidation.
//
// For Stage-2 invalidation, use the level values provided to that effect
// in asm/stage2_pgtable.h.
//

extern "C" {
    pub fn void(arg: *mut *mut tlbi_op)(u64) -> typedef;
}
//
// This macro creates a properly formatted VA operand for the TLB RANGE. The
// value bit assignments are:
//
// +----------+------+-------+-------+-------+----------------------+
// |   ASID   |  TG  | SCALE |  NUM  |  TTL  |        BADDR         |
// +-----------------+-------+-------+-------+----------------------+
// |63      48|47  46|45   44|43   39|38   37|36                   0|
//
// The address range is determined by below formula: [BADDR, BADDR + (NUM + 1)
// 2^(5*SCALE + 1) * PAGESIZE)
//
// Note that the first argument, baddr, is pre-shifted; If LPA2 is in use, BADDR
// holds addr[52:16]. Else BADDR holds page number. See for example ARM DDI
// 0487J.a section C5.5.60 "TLBI VAE1IS, TLBI VAE1ISNXS, TLB Invalidate by VA,
// EL1, Inner Shareable".
//

// These macros are used by the TLBI RANGE feature.

//
// Generate 'num' values from -1 to 31 with -1 rejected by the
// __flush_tlb_range() loop below. Its return value is only
// significant for a maximum of MAX_TLBI_RANGE_PAGES pages. If
// 'pages' is more than that, you must iterate over the overall
// range.
//

//
// Complete broadcast TLB maintenance issued by the host which invalidates
// stage 1 information in the host's own translation regime.
//
// Complete broadcast TLB maintenance issued by hyp code which invalidates
// stage 1 translation information in any translation regime.
//
// TLB Invalidation
// ================
//
// This header file implements the low-level TLB invalidation routines
// (sometimes referred to as "flushing" in the kernel) for arm64.
//
// Every invalidation operation uses the following template:
//
// DSB ISHST	// Ensure prior page-table updates have completed
// TLBI ...	// Invalidate the TLB
// DSB ISH		// Ensure the TLB invalidation has completed
// if (invalidated kernel mappings)
// ISB	// Discard any instructions fetched from the old mapping
//
// The following functions form part of the "core" TLB invalidation API,
// as documented in Documentation/core-api/cachetlb.rst:
//
// flush_tlb_all()
// Invalidate the entire TLB (kernel + user) on all CPUs
//
// flush_tlb_mm(mm)
// Invalidate an entire user address space on all CPUs.
// The 'mm' argument identifies the ASID to invalidate.
//
// flush_tlb_range(vma, start, end)
// Invalidate the virtual-address range '[start, end)' on all
// CPUs for the user address space corresponding to 'vma->mm'.
// Note that this operation also invalidates any walk-cache
// entries associated with translations for the specified address
// range.
//
// flush_tlb_kernel_range(start, end)
// Same as flush_tlb_range(..., start, end), but applies to
// kernel mappings rather than a particular user address space.
// Whilst not explicitly documented, this function is used when
// unmapping pages from vmalloc/io space.
//
// flush_tlb_page(vma, addr)
// Equivalent to __flush_tlb_page(..., flags=TLBF_NONE)
//
// Next, we have some undocumented invalidation routines that you probably
// don't want to call unless you know what you're doing:
//
// local_flush_tlb_all()
// Same as flush_tlb_all(), but only applies to the calling CPU.
//
// __flush_tlb_kernel_pgtable(addr)
// Invalidate a single kernel mapping for address 'addr' on all
// CPUs, ensuring that any walk-cache entries associated with the
// translation are also invalidated.
//
// __flush_tlb_range(vma, start, end, stride, tlb_level, flags)
// Invalidate the virtual-address range '[start, end)' on all
// CPUs for the user address space corresponding to 'vma->mm'.
// The invalidation operations are issued at a granularity
// determined by 'stride'. tlb_level is the level at
// which the invalidation must take place. If the level is wrong,
// no invalidation may take place. In the case where the level
// cannot be easily determined, the value TLBI_TTL_UNKNOWN will
// perform a non-hinted invalidation. flags may be TLBF_NONE (0) or
// any combination of TLBF_NOWALKCACHE (elide eviction of walk
// cache entries), TLBF_NONOTIFY (don't call mmu notifiers),
// TLBF_NOSYNC (don't issue trailing dsb) and TLBF_NOBROADCAST
// (only perform the invalidation for the local cpu).
//
// __flush_tlb_page(vma, addr, flags)
// Invalidate a single user mapping for address 'addr' in the
// address space corresponding to 'vma->mm'.  Note that this
// operation only invalidates a single level 3 page-table entry
// and therefore does not affect any walk-caches. flags may contain
// any combination of TLBF_NONOTIFY (don't call mmu notifiers),
// TLBF_NOSYNC (don't issue trailing dsb) and TLBF_NOBROADCAST
// (only perform the invalidation for the local cpu).
//
// Finally, take a look at asm/tlb.h to see how tlb_flush() is implemented
// on top of these routines, since that is our interface to the mmu_gather
// API as used by munmap() and friends.
//
// To support TLB batched flush for multiple pages unmapping, we only send
// the TLBI for each page in arch_tlbbatch_add_pending() and wait for the
// completion at the end in arch_tlbbatch_flush(). Since we've already issued
// TLBI for each page so only a DSB is needed to synchronise its effect on the
// other CPUs.
//
// This will save the time waiting on DSB comparing issuing a TLBI;DSB sequence
// for each page.
//
// This is meant to avoid soft lock-ups on large TLB flushing ranges and not
// necessarily a performance improvement.
//

//
// __flush_tlb_range_op - Perform TLBI operation upon a range
//
// @lop:	TLBI level operation to perform
// @rop:	TLBI range operation to perform
// @start:	The start address of the range
// @pages:	Range as the number of pages from 'start'
// @stride:	Flush granularity
// @asid:	The ASID of the task (0 for IPA instructions)
// @level:	Translation Table level hint, if known
// @lpa2:	If 'true', the lpa2 scheme is used as set out below
//
// When the CPU does not support TLB range operations, flush the TLB
// entries one by one at the granularity of 'stride'. If the TLB
// range ops are supported, then:
//
// 1. If FEAT_LPA2 is in use, the start address of a range operation must be
// 64KB aligned, so flush pages one by one until the alignment is reached
// using the non-range operations. This step is skipped if LPA2 is not in
// use.
//
// 2. The minimum range granularity is decided by 'scale', so multiple range
// TLBI operations may be required. Start from scale = 3, flush the largest
// possible number of pages ((num+1)*2^(5*scale+1)) that fit into the
// requested range, then decrement scale and continue until one or zero pages
// are left. We must start from highest scale to ensure 64KB start alignment
// is maintained in the LPA2 case.
//
// 3. If there is 1 page remaining, flush it through non-range operations. Range
// operations can only span an even number of pages. We save this for last to
// ensure 64KB start alignment is maintained for the LPA2 case.
//

//
// Assume that the worst case number of DVM ops required to flush a
// given range on a system that supports tlb-range is 20 (4 scales, 1
// final page, 15 for alignment on LPA2 systems), which is much smaller
// than MAX_DVM_OPS.
//
pub type tlbf_t = unsigned ;
// No special behaviour.

// Invalidate tlb entries only, leaving the page table walk cache intact.

// Skip the trailing dsb after issuing tlbi.

// Suppress tlb notifier callbacks for this flush operation.

// Perform the tlbi locally without broadcasting to other CPUs.

// Combination unused
//
// We cannot use leaf-only invalidation here, since we may be invalidating
// table entries as part of collapsing hugepages or moving page tables.
// Set the tlb_level to TLBI_TTL_UNKNOWN because we can not get enough
// information here.
//
// Used to invalidate the TLB (walk caches) corresponding to intermediate page
// table levels (pgd/pud/pmd).
//
// invalid to valid transition requires no flush
// Transition in the SW bits requires no flush
extern "C" {
    pub fn __pte_flags_need_flush(_arg: pte_val(oldpte), _arg: pte_val(newpte)) -> return;
}

extern "C" {
    pub fn __pte_flags_need_flush(_arg: pmd_val(oldpmd), _arg: pmd_val(newpmd)) -> return;
}

