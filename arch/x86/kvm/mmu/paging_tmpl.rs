//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kvm/mmu/paging_tmpl.h
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
// Kernel-based Virtual Machine driver for Linux
//
// This module enables machines with Intel VT-x extensions to run virtual
// machines without emulation or binary translation.
//
// MMU support
//
// Copyright (C) 2006 Qumranet, Inc.
// Copyright 2010 Red Hat, Inc. and/or its affiliates.
//
// Authors:
// Yaniv Kamay  <yaniv@qumranet.com>
// Avi Kivity   <avi@qumranet.com>
//
// The MMU needs to be able to access/walk 32-bit and 64-bit guest page tables,
// as well as guest EPT tables, so the code in this file is compiled thrice,
// once per guest PTE type.  The per-type defines are #undef'd at the end.
//

pub const PT_LEVEL_BITS: c_int = 9;

pub const PT_MAX_FULL_LEVELS: c_int = 2;

pub const PT_LEVEL_BITS: c_int = 10;
pub const PT_MAX_FULL_LEVELS: c_int = 2;

pub const PT32_DIR_PSE36_SIZE: c_int = 4;
pub const PT32_DIR_PSE36_SHIFT: c_int = 13;

pub const PT_LEVEL_BITS: c_int = 9;
pub const PT_GUEST_DIRTY_SHIFT: c_int = 9;
pub const PT_GUEST_ACCESSED_SHIFT: c_int = 8;

// Common logic, but per-type values.  These also need to be undefined.

//
// The guest_walker structure emulates the behavior of the hardware page
// table walker.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guest_walker {
    pub level: c_int,
    pub max_level: unsigned,
    pub table_gfn: [gfn_t; PT_MAX_FULL_LEVELS],
    pub ptes: [pt_element_t; PT_MAX_FULL_LEVELS],
    pub prefetch_ptes: [pt_element_t; PTE_PREFETCH_NUM],
    pub pte_gpa: [gpa_t; PT_MAX_FULL_LEVELS],
    pub ptep_user: [*mut pt_element_t __user; PT_MAX_FULL_LEVELS],
    pub pte_writable: [bool; PT_MAX_FULL_LEVELS],
    pub pt_access: [c_uint; PT_MAX_FULL_LEVELS],
    pub pte_access: c_uint,
    pub gfn: gfn_t,
    pub fault: x86_exception,
}

// dirty bit is not supported, so no need to track it
// Allow write access to dirty gptes
// access &= mask;

//
// For EPT, an entry is present if any of bits 2:0 are set.
// With mode-based execute control, bit 10 also indicates presence.
//

extern "C" {
    pub fn __is_bad_mt_xwr(_arg: fmt, _arg: gpte) -> return;
}

// Prefetch only accessed entries (unless A/D bits are disabled).
//
// Set bits in ACC_*_MASK even if they might not be used in the
// actual checks.  For example, if EFER.NX is clear permission_fault()
// will ignore ACC_EXEC_MASK, and if MBEC is disabled it will
// ignore ACC_USER_EXEC_MASK.
//

//
// P is set here, so the page is always readable and W/U/!NX represent
// allowed accesses.
//

// dirty/accessed bits are not supported, so no need to update them

//
// If the slot is read-only, simply do not process the accessed
// and dirty bits.  This is the correct thing to do if the slot
// is ROM, and page tables in read-as-ROM/write-as-MMIO slots
// are only supported if the accessed and dirty bits are already
// set in the ROM (so that MMIO writes are never needed).
//
// Note that NPT does not allow this at all and faults, since
// it always wants nested page table entries for the guest
// page tables to be writable.  And EPT works but will simply
// overwrite the read-only memory to set the accessed and dirty
// bits.
//

//
// For EPT and PAE paging (both variants), bit 7 is either reserved at
// all level or indicates a huge page (ignoring CR3/EPTP).  In either
// case, bit 7 being set terminates the walk.
//

//
// 32-bit paging requires special handling because bit 7 is ignored if
// CR4.PSE=0, not reserved.  Clear bit 7 in the gpte if the level is
// greater than the last level for which bit 7 is the PAGE_SIZE bit.
//
// The RHS has bit 7 set iff level < (2 + PSE).  If it is clear, bit 7
// is not reserved and does not indicate a large page at this level,
// so clear PT_PAGE_SIZE_MASK in gpte if that is the case.
//

//
// PG_LEVEL_4K always terminates.  The RHS has bit 7 set
// iff level <= PG_LEVEL_4K, which for our purpose means
// level == PG_LEVEL_4K; set PT_PAGE_SIZE_MASK in gpte then.
//
// Fetch a guest pte for a guest virtual address, or for an L2's GPA.
//
// Note! Track the error_code that's common to legacy shadow paging
// and NPT shadow paging as a u16 to guard against unintentionally
// setting any of bits 63:16.  Architecturally, the #PF error code is
// 32 bits, and Intel CPUs don't support settings bits 31:16.
//

//
// FIXME: on Intel processors, loads of the PDPTE registers for PAE paging
// by the MOV to CR instruction are treated as reads and do not cause the
// processor to set the dirty flag in any EPT paging-structure entry.
//
// Queue a page fault for injection if this assertion fails, as callers
// assume that walker.fault contains sane info on a walk failure.  I.e.
// avoid making the situation worse by inducing even worse badness
// between when the assertion fails and when KVM kicks the vCPU out to
// userspace (because the VM is bugged).
//
// Inverting the NX it lets us AND it like other
// permission bits.
//
// Convert to ACC_*_MASK flags for struct guest_walker.

//
// On a write fault, fold the dirty bit into accessed_dirty.
// For modes without A/D bits support accessed_dirty will be
// always clear.
//

//
// Use PFERR_RSVD_MASK in error_code to tell if EPT
// misconfiguration requires to be injected. The detection is
// done by is_rsvd_bits_set() above.
//
// We set up the value of exit_qualification to inject:
// [2:0] - Derive from the access bits. The exit_qualification might be
// out of date if it is serving an EPT misconfiguration.
// [5:3] - Calculated by the page walk of the guest EPT page tables
// [7:8] - Derived from "fault stage" access bits
// [9:11] - Derived from [9:11] of real exit_qualification
//
// The other bits are set to 0.
//
// KVM doesn't emulate features that access GPAs directly, e.g.
// Intel Processor Trace.  Assume the GVA is always valid; when
// propagating faults from hardware, KVM will discard this info
// and use the EXIT_QUALIFICATION bits from the VMCS.
//
// Accesses to guest paging structures are either "reads" or
// "read+write" accesses, so consider them the latter if write_fault
// is true.
//
// Note, pte_access holds the raw RWX bits from the EPTE, not
// ACC_*_MASK flags!
//

extern "C" {
    pub fn kvm_mmu_prefetch_sptes(_arg: vcpu, _arg: gfn, _arg: spte, _arg: 1, _arg: pte_access) -> return;
}
//
// If addresses are being invalidated, skip prefetching to avoid
// accidentally prefetching those addresses.
//
extern "C" {
    pub fn __direct_pte_prefetch(_arg: vcpu, _arg: sp, _arg: sptep) -> return;
}
//
// Fetch a shadow pte for a specific level in the paging hierarchy.
// If the guest tries to write a write-protected page, we need to
// emulate this operation, return 1 to indicate this case.
//
// Verify that the top-level gpte is still there.  Since the page
// is a root page, it is either write protected (and cannot be
// changed from now on) or it is invalid (in which case, we don't
// really care if it changes underneath us after this point).
//
// Load a new root and retry the faulting instruction in the extremely
// unlikely scenario that the guest root gfn became visible between
// loading a dummy root and handling the resulting page fault, e.g. if
// userspace create a memslot in the interim.
//
// Synchronize the new page before linking it, as the CPU (KVM)
// is architecturally disallowed from inserting non-present
// entries into the TLB, i.e. the guest isn't required to flush
// the TLB when changing the gPTE from non-present to present.
//
// For PG_LEVEL_4K, kvm_mmu_find_shadow_page() has already
// synchronized the page via kvm_sync_page().
//
// For higher level pages, which cannot be unsync themselves
// but can have unsync children, synchronize via the slower
// mmu_sync_children().  If KVM needs to drop mmu_lock due to
// contention or to reschedule, instruct the caller to retry
// the #PF (mmu_sync_children() ensures forward progress will
// be made).
//
// Verify that the gpte in the page, which is now either
// write-protected or unsync, wasn't modified between the fault
// and acquiring mmu_lock.  This needs to be done even when
// reusing an existing shadow page to ensure the information
// gathered by the walker matches the information stored in the
// shadow page (which could have been modified by a different
// vCPU even if the page was already linked).  Holding mmu_lock
// prevents the shadow page from changing after this point.
//
// Adjust the hugepage size _after_ resolving indirect shadow pages.
// KVM doesn't support mapping hugepages into the guest for gfns that
// are being shadowed by KVM, i.e. allocating a new shadow page may
// affect the allowed hugepage size.
//
// We cannot overwrite existing page tables with an NX
// large page, as the leaf could be executable.
//
// Page fault handler.  There are several causes for a page fault:
// - there is no shadow pte for the guest pte
// - write access through a shadow pte marked read only so that we can set
// the dirty bit
// - write access to a shadow pte marked read only so we can update the page
// dirty bitmap, when userspace requests it
// - mmio access; in this case we will never install a present shadow pte
// - normal guest page fault due to the guest pte marked not present, not
// writable, or not executable
//
// Returns: 1 if we need to emulate the instruction, 0 otherwise, or
// a negative value on error.
//
// Look up the guest pte for the faulting address.
// If PFEC.RSVD is set, this is a shadow page fault.
// The bit needs to be cleared before walking guest page tables.
//
// The page is not mapped by the guest.  Let the guest handle it.
//

//
// Treat the guest PTE protections as writable, supervisor-only if this
// is a supervisor write fault and CR0.WP=0 (supervisor accesses ignore
// PTE.W if CR0.WP=0).  Don't change the access type for emulated MMIO,
// otherwise KVM will cache incorrect access information in the SPTE.
//
// If we converted a user page to a kernel page,
// so that the kernel can write to it when cr0.wp=0,
// then we should prevent the kernel from executing it
// if SMEP is enabled.
//

extern "C" {
    pub fn gfn_to_gpa(sizeof(pt_element_t: *mut *mut sp->gfn) + offset) -> return;
}
// Note, @addr is a GPA when gva_to_gpa() translates an L2 GPA to an L1 GPA.

// A 64-bit GVA should be impossible on 32-bit KVM.

// exception = walker.fault;
//
// Using the information in sp->shadowed_translation (kvm_mmu_page_get_gfn()) is
// safe because SPTEs are protected by mmu_notifiers and memslot generations, so
// the pfn for a given gfn can't change unless all SPTEs pointing to the gfn are
// nuked first.
//
// Returns
// < 0: failed to sync spte
// 0: the spte is synced and no tlb flushing is required
// > 0: the spte is synced and tlb flushing is required
//
// Drop the SPTE if the new protections result in no effective
// "present" bit or if the gfn is changing.  The former case
// only affects EPT with execute-only support with pte_access==0;
// all other paging modes will create a read-only SPTE if
// pte_access is zero.
//
// Do nothing if the permissions are unchanged.  The existing SPTE is
// still, and prefetch_invalid_gpte() has verified that the A/D bits
// are set in the "new" gPTE, i.e. there is no danger of missing an A/D
// update due to A/D bits being set in the SPTE but not the gPTE.
//
// Update the shadowed access bits in case they changed.
//
// There is no need to mark the pfn dirty, as the new protections must
// be a subset of the old protections, i.e. synchronizing a SPTE cannot
// change the SPTE from read-only to writable.
//
extern "C" {
    pub fn mmu_spte_update(_arg: sptep, _arg: spte) -> return;
}

