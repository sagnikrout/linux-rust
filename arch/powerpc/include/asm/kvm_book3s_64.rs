//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/kvm_book3s_64.h
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
// Copyright SUSE Linux Products GmbH 2010
//
// Authors: Alexander Graf <agraf@suse.de>
//

//
// Structure for a nested guest, that is, for a guest that is managed by
// one of our guests.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_nested_guest {
    pub /: *mut *mut *mut kvm l1_host; / L1 VM that owns this nested guest,
    pub /: *mut *mut int l1_lpid; / lpid L1 guest thinks this guest is,
    pub /: *mut *mut int shadow_lpid; / real lpid of this nested guest,
    pub /: *mut *mut *mut pgd_t shadow_pgtable; / our page table for this guest,
    pub /: *mut *mut u64 l1_gr_to_hr; / L1's addr of part'n-scoped table,
    pub /: *mut *mut u64 process_table; / process table entry for this guest,
    pub /: *mut *mut long refcnt; / number of pointers to this struct,
    pub /: *mut *mut mutex tlb_lock; / serialize page faults and tlbies,
    pub next: *mut kvm_nested_guest,
    pub need_tlb_flush: cpumask_t,
    pub prev_cpu: [c_short; NR_CPUS],
    pub /: *mut *mut u8 radix; / is this nested guest radix,
}

//
// We define a nested rmap entry as a single 64-bit quantity
// 0xFFF0000000000000	12-bit lpid field
// 0x000FFFFFFFFFF000	40-bit guest 4k page frame number
// 0x0000000000000001	1-bit  single entry flag
//
pub const RMAP_NESTED_LPID_MASK: c_uint = 0xFFF0000000000000UL;

pub const RMAP_NESTED_GPA_MASK: c_uint = 0x000FFFFFFFFFF000UL;
pub const RMAP_NESTED_IS_SINGLE_ENTRY: c_uint = 0x0000000000000001UL;
// Structure for a nested guest rmap entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rmap_nested {
    pub list: llist_node,
    pub rmap: u64,
}

//
// for_each_nest_rmap_safe - iterate over the list of nested rmap entries
// safe against removal of the list entry or NULL list
// @pos:	a (struct rmap_nested *) to use as a loop cursor
// @node:	pointer to the first entry
// NOTE: this can be NULL
// @rmapp:	an (unsigned long *) in which to return the rmap entries on each
// iteration
// NOTE: this must point to already allocated memory
//
// The nested_rmap is a llist of (struct rmap_nested) entries pointed to by the
// rmap entry in the memslot. The list is always terminated by a "single entry"
// stored in the list element of the final entry of the llist. If there is ONLY
// a single entry then this is itself in the rmap entry of the memslot, not a
// llist head pointer.
//
// Note that the iterator below assumes that a nested rmap entry is always
// non-zero.  This is true for our usage because the LPID field is always
// non-zero (zero is reserved for the host).
//
// This should be used to iterate over the list of rmap_nested entries with
// processing done on the u64 rmap value given by each iteration. This is safe
// against removal of list entries and it is always safe to call free on (pos).
//
// e.g.
// struct rmap_nested *cursor;
// struct llist_node *first;
// unsigned long rmap;
// for_each_nest_rmap_safe(cursor, first, &rmap) {
// do_something(rmap);
// free(cursor);
// }
//

extern "C" {
    pub fn kvmhv_put_nested(gp: *mut kvm_nested_guest);
}
extern "C" {
    pub fn kvmhv_nested_next_lpid(kvm: *mut kvm, lpid: c_int) -> c_int;
}
// Encoding of first parameter for H_TLB_INVALIDATE

// Power architecture requires HPT is at least 256kiB, at most 64TiB
pub const PPC_MIN_HPT_ORDER: c_int = 18;
pub const PPC_MAX_HPT_ORDER: c_int = 46;

extern "C" {
    pub fn kvmppc_msr_hard_disable_set_facilities(vcpu: *mut kvm_vcpu, msr: c_ulong) -> c_ulong;
}
extern "C" {
    pub fn kvmhv_vcpu_entry_p9(vcpu: *mut kvm_vcpu, time_limit: u64, lpcr: c_ulong, tb: *mut u64) -> c_int;
}

//
// Invalid HDSISR value which is used to indicate when HW has not set the reg.
// Used to work around an errata.
//
pub const HDSISR_CANARY: c_uint = 0x7fff;
//
// We use a lock bit in HPTE dword 0 to synchronize updates and
// accesses to each HPTE, and another bit to indicate non-present
// HPTEs.
//
pub const HPTE_V_HVLOCK: c_uint = 0x40UL;
pub const HPTE_V_ABSENT: c_uint = 0x20UL;
//
// We use this bit in the guest_rpte field of the revmap entry
// to indicate a modified HPTE.
//

// These bits are reserved in the guest view of the HPTE

//
// We load/store in native endian, but the HTAB is in big endian. If
// we byte swap all data we apply on the PTE we're implicitly correct
// again.
//
extern "C" {
    pub fn volatile("memory": PPC_RELEASE_BARRIER "" : : :) -> asm;
}
// Without barrier
//
// These functions encode knowledge of the POWER7/8/9 hardware
// interpretations of the HPTE LP (large page size) field.
//
// Ignore the top 14 bits of va
// v have top two bits covering segment size, hence move
// by 16 bits, Also clear the lower HPTE_V_AVPN_SHIFT (7) bits.
// AVA field in v also have the lower 23 bits ignored.
// For base page size 4K we need 14 .. 65 bits (so need to
// collect extra 11 bits)
// For others we need 14..14+i
//
// This covers 14..54 bits of va
//
// AVA in v had cleared lower 23 bits. We need to derive
// that from pteg index
//
// get the vpn bits from va_low using reverse of hashing.
// In v we have va with 23 bits dropped and then left shifted
// HPTE_V_AVPN_SHIFT (7) bits. Now to find vsid we need
// right shift it with (SID_SHIFT - (23 - 7))
//
// remaining bits of AVA/LP fields
// Also contain the rr bits of LP
//
// Now clear not needed LP bits based on actual psize
//
// AVAL field 58..77 - base_page_shift bits of va
// we have space for 58..64 bits, Missing bits should
// be zero filled. +1 is to take care of L bit shift
//
// This sets both bits of the B field in the PTE. 0b1x values are
// reserved, but those will have been filtered by kvmppc_do_h_enter.
//
// Handle SAO
//
// if host is mapped cache inhibited, make sure hptel also have
// cache inhibited.
//
// If it's present and writable, atomically set dirty and referenced bits and
// return the PTE, otherwise return 0.
//
// Make sure we don't reload from ptep
//
// wait until H_PAGE_BUSY is clear then set it atomically
//
// If pte is not present return None
extern "C" {
    pub fn __pte(_arg: 0) -> return;
}
//
// This works for 4k, 64k and 16M pages on POWER7,
// and 4k and 16M pages on PPC970.
//

//
// Note modification of an HPTE; set the HPTE modified bit
// if anyone is interested.
//
// Like kvm_memslots(), but for use in real mode when we can't do
// any RCU stuff (since the secondary threads are offline from the
// kernel's point of view), and we can't print anything.
// Thus we use rcu_dereference_raw() rather than rcu_dereference_check().
//
extern "C" {
    pub fn rcu_dereference_raw_check(_arg: kvm->memslots[0]) -> return;
}
extern "C" {
    pub fn kvmppc_mmu_debugfs_init(kvm: *mut kvm);
}
extern "C" {
    pub fn kvmhv_radix_debugfs_init(kvm: *mut kvm);
}
extern "C" {
    pub fn kvmhv_rm_send_ipi(cpu: c_int);
}
// HPTEs are 2**4 bytes long
// 128 (2**7) bytes in each HPTEG
// Set bits in a dirty bitmap, which is in LE format

extern "C" {
    pub fn kvmhv_nestedv2_vcpu_create(vcpu: *mut kvm_vcpu, io: *mut kvmhv_nestedv2_io) -> c_int;
}
extern "C" {
    pub fn kvmhv_nestedv2_vcpu_free(vcpu: *mut kvm_vcpu, io: *mut kvmhv_nestedv2_io);
}
extern "C" {
    pub fn kvmhv_nestedv2_flush_vcpu(vcpu: *mut kvm_vcpu, time_limit: u64) -> c_int;
}
extern "C" {
    pub fn kvmhv_nestedv2_set_ptbl_entry(lpid: c_ulong, dw0: u64, dw1: u64) -> c_int;
}
extern "C" {
    pub fn kvmhv_nestedv2_parse_output(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvmhv_nestedv2_set_vpa(vcpu: *mut kvm_vcpu, vpa: c_ulong) -> c_int;
}
extern "C" {
    pub fn kvmhv_counters_tracepoint_regfunc() -> c_int;
}
extern "C" {
    pub fn kvmhv_counters_tracepoint_unregfunc();
}
extern "C" {
    pub fn kvmhv_get_l2_counters_status() -> c_int;
}
extern "C" {
    pub fn kvmhv_set_l2_counters_status(cpu: c_int, status: bool);
}
extern "C" {
    pub fn kvmhv_get_l1_to_l2_cs_time() -> u64;
}
extern "C" {
    pub fn kvmhv_get_l2_to_l1_cs_time() -> u64;
}
extern "C" {
    pub fn kvmhv_get_l2_runtime_agg() -> u64;
}
extern "C" {
    pub fn kvmhv_get_l1_to_l2_cs_time_vcpu() -> u64;
}
extern "C" {
    pub fn kvmhv_get_l2_to_l1_cs_time_vcpu() -> u64;
}
extern "C" {
    pub fn kvmhv_get_l2_runtime_agg_vcpu() -> u64;
}

