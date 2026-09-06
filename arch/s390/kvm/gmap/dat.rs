//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/kvm/gmap/dat.h
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
// KVM guest address space mapping code
//
// Copyright IBM Corp. 2024, 2025
// Author(s): Claudio Imbrenda <imbrenda@linux.ibm.com>
//

//
// Base address and length must be sent at the start of each block, therefore
// it's cheaper to send some clean data, as long as it's less than the size of
// two longs.
//

// For consistency

// This fake table type is used for page table walks (both for normal page tables and vSIE)

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dat_walk_flags {
    DAT_WALK_USES_SKEYS	= 0x40,
    DAT_WALK_CONTINUE	= 0x20,
    DAT_WALK_IGN_HOLES	= 0x10,
    DAT_WALK_SPLIT		= 0x08,
    DAT_WALK_ALLOC		= 0x04,
    DAT_WALK_ANY		= 0x02,
    DAT_WALK_LEAF		= 0x01,
    DAT_WALK_DEFAULT	= 0
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pte {
    pub val: c_ulong,
    pub h: page_table_entry,
    pub /: *mut *mut unsigned long :56; / Hardware bits,
    pub /: *mut *mut unsigned long u : 1; / Page unused,
    pub /: *mut *mut unsigned long s : 1; / Special,
    pub /: *mut *mut unsigned long w : 1; / Writable,
    pub /: *mut *mut unsigned long r : 1; / Readable,
    pub /: *mut *mut unsigned long d : 1; / Dirty,
    pub /: *mut *mut unsigned long y : 1; / Young,
    pub /: *mut *mut unsigned long sd: 1; / Soft dirty,
    pub /: *mut *mut unsigned long pr: 1; / Present,
    pub s: },
    pub hwbytes: [c_uchar; 7],
    pub swbyte: c_uchar,
}

// Soft dirty, needed as macro for atomic operations on ptes
pub const _PAGE_SD: c_uint = 0x002;
// Needed as macro to perform atomic operations
pub const PGSTE_PCL_BIT: c_uint = 0x0080000000000000UL	/* PCL lock, HW bit */;
pub const PGSTE_CMMA_D_BIT: c_uint = 0x0000000000008000UL	/* CMMA dirty soft-bit */;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pgste_gps_usage {
    PGSTE_GPS_USAGE_STABLE = 0,
    PGSTE_GPS_USAGE_UNUSED,
    PGSTE_GPS_USAGE_POT_VOLATILE,
    PGSTE_GPS_USAGE_VOLATILE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pgste {
    pub val: c_ulong,
    pub 4: unsigned long acc :,
    pub 1: unsigned long fp :,
    pub 3: unsigned long :,
    pub 1: unsigned long pcl :,
    pub 1: unsigned long hr :,
    pub 1: unsigned long hc :,
    pub 2: unsigned long :,
    pub 1: unsigned long gr :,
    pub 1: unsigned long gc :,
    pub 1: unsigned long :,
    pub /: *mut *mut unsigned long :16; / val16,
    pub 1: unsigned long zero :,
    pub 1: unsigned long nodat :,
    pub 4: unsigned long :,
    pub 2: unsigned long usage :,
    pub 8: unsigned long :,
    pub /: *mut *mut unsigned long cmma_d : 1; / Dirty flag for CMMA bits,
    pub /: *mut *mut unsigned long prefix_notif : 1; / Guest prefix invalidation notification,
    pub /: *mut *mut unsigned long vsie_notif : 1; / Referenced in a shadow table,
    pub /: *mut *mut unsigned long vsie_gmem : 1; / Contains nested guest memory,
    pub 4: unsigned long :,
    pub 8: unsigned long :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pmd {
    pub val: c_ulong,
    pub h: segment_table_entry,
    pub /: *mut *mut unsigned long :44; / HW,
    pub /: *mut *mut unsigned long : 3; / Unused,
    pub /: *mut *mut unsigned long : 1; / HW,
    pub /: *mut *mut unsigned long s : 1; / Special,
    pub /: *mut *mut unsigned long w : 1; / Writable soft-bit,
    pub /: *mut *mut unsigned long r : 1; / Readable soft-bit,
    pub /: *mut *mut unsigned long d : 1; / Dirty,
    pub /: *mut *mut unsigned long y : 1; / Young,
    pub /: *mut *mut unsigned long : 3; / HW,
    pub /: *mut *mut unsigned long prefix_notif : 1; / Guest prefix invalidation notification,
    pub /: *mut *mut unsigned long vsie_notif : 1; / Referenced in a shadow table,
    pub /: *mut *mut unsigned long : 4; / HW,
    pub /: *mut *mut unsigned long sd : 1; / Soft-Dirty,
    pub /: *mut *mut unsigned long pr : 1; / Present,
    pub fc1: },
    pub s: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pud {
    pub val: c_ulong,
    pub h: region3_table_entry,
    pub /: *mut *mut unsigned long :33; / HW,
    pub /: *mut *mut unsigned long :14; / Unused,
    pub /: *mut *mut unsigned long : 1; / HW,
    pub /: *mut *mut unsigned long s : 1; / Special,
    pub /: *mut *mut unsigned long w : 1; / Writable soft-bit,
    pub /: *mut *mut unsigned long r : 1; / Readable soft-bit,
    pub /: *mut *mut unsigned long d : 1; / Dirty,
    pub /: *mut *mut unsigned long y : 1; / Young,
    pub /: *mut *mut unsigned long : 3; / HW,
    pub /: *mut *mut unsigned long prefix_notif : 1; / Guest prefix invalidation notification,
    pub /: *mut *mut unsigned long vsie_notif : 1; / Referenced in a shadow table,
    pub /: *mut *mut unsigned long : 4; / HW,
    pub /: *mut *mut unsigned long sd : 1; / Soft-Dirty,
    pub /: *mut *mut unsigned long pr : 1; / Present,
    pub fc1: },
    pub s: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union p4d {
    pub val: c_ulong,
    pub h: region2_table_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union pgd {
    pub val: c_ulong,
    pub h: region1_table_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union crste {
    pub val: c_ulong,
    pub :52: c_ulong,
    pub 1: unsigned long :,
    pub 1: unsigned long fc:,
    pub 1: unsigned long p :,
    pub 1: unsigned long :,
    pub 2: unsigned long :,
    pub 1: unsigned long i :,
    pub 1: unsigned long :,
    pub 2: unsigned long tt:,
    pub 2: unsigned long :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union skey {
    pub skey: c_uchar,
    pub :4: unsigned char acc,
    pub :1: unsigned char fp,
    pub :1: unsigned char r,
    pub :1: unsigned char c,
    pub zero:1: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct segment_table {
    pub pmds: [pmd; _CRST_ENTRIES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct region3_table {
    pub puds: [pud; _CRST_ENTRIES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct region2_table {
    pub p4ds: [p4d; _CRST_ENTRIES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct region1_table {
    pub pgds: [pgd; _CRST_ENTRIES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crst_table {
    pub crstes: [crste; _CRST_ENTRIES],
    pub segment: segment_table,
    pub region3: region3_table,
    pub region2: region2_table,
    pub region1: region1_table,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct page_table {
    pub ptes: [pte; _PAGE_ENTRIES],
    pub pgstes: [pgste; _PAGE_ENTRIES],
}

extern "C" {
    pub fn long(crste: *mut *mut dat_walk_op)(union crste, gfn: gfn_t, next: gfn_t, w: *mut dat_walk) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dat_walk_ops {
    pub crste_ops: [dat_walk_op; 4],
    pub pmd_entry: dat_walk_op,
    pub pud_entry: dat_walk_op,
    pub p4d_entry: dat_walk_op,
    pub pgd_entry: dat_walk_op,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dat_walk {
    pub ops: *const dat_walk_ops,
    pub last: *mut crste,
    pub last_pte: *mut pte,
    pub asce: asce,
    pub start: gfn_t,
    pub end: gfn_t,
    pub flags: c_int,
    pub priv: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptval_param {
    pub 6: unsigned char offset :,
    pub 2: unsigned char len :,
}

//
// _pte() - Useful constructor for union pte
// @pfn: the pfn this pte should point to.
// @writable: whether the pte should be writable.
// @dirty: whether the pte should be dirty.
// @special: whether the pte should be marked as special
//
// The pte is also marked as young and present. If the pte is marked as dirty,
// it gets marked as soft-dirty too. If the pte is not dirty, the hardware
// protect bit is set (independently of the write softbit); this way proper
// dirty tracking can be performed.
//
// Return: a union pte value.
//
// _crste() - Useful constructor for union crste with FC=1
// @pfn: the pfn this pte should point to.
// @tt: the table type
// @writable: whether the pte should be writable.
// @dirty: whether the pte should be dirty.
//
// The crste is also marked as young and present. If the crste is marked as
// dirty, it gets marked as soft-dirty too. If the crste is not dirty, the
// hardware protect bit is set (independently of the write softbit); this way
// proper dirty tracking can be performed.
//
// Return: a union crste value.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union essa_state {
    pub val: c_uchar,
    pub 2: unsigned char :,
    pub 1: unsigned char nodat :,
    pub 1: unsigned char exception :,
    pub 2: unsigned char usage :,
    pub 2: unsigned char content :,
}

//
// struct vsie_rmap - reverse mapping for shadow page table entries
// @next: pointer to next rmap in the list
// @r_gfn: virtual rmap address in the shadow guest address space
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsie_rmap {
    pub next: *mut vsie_rmap,
    pub val: c_ulong,
    pub 8: long level:,
    pub 4: unsigned long :,
    pub r_gfn:52: c_ulong,
}

pub const KVM_S390_MMU_CACHE_N_CRSTS: c_int = 6;
pub const KVM_S390_MMU_CACHE_N_PTS: c_int = 2;
pub const KVM_S390_MMU_CACHE_N_RMAPS: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_mmu_cache {
    pub crsts: [*mut c_void; KVM_S390_MMU_CACHE_N_CRSTS],
    pub pts: [*mut c_void; KVM_S390_MMU_CACHE_N_PTS],
    pub rmaps: [*mut c_void; KVM_S390_MMU_CACHE_N_RMAPS],
    pub n_crsts: short int,
    pub n_pts: short int,
    pub n_rmaps: short int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct guest_fault {
    pub /: *mut *mut gfn_t gfn; / Guest frame,
    pub /: *mut *mut kvm_pfn_t pfn; / Host PFN,
    pub /: *mut *mut *mut page page; / Host page,
    pub /: *mut *mut *mut pte ptep; / Used to resolve the fault, or NULL,
    pub /: *mut *mut *mut crste crstep; / Used to resolve the fault, or NULL,
    pub /: *mut *mut bool writable; / Mapping is writable,
    pub /: *mut *mut bool write_attempt; / Write access attempted,
    pub /: *mut *mut bool attempt_pfault; / Attempt a pfault first,
    pub /: *mut *mut bool valid; / This entry contains valid data,
    pub /: *mut *mut bool crste_region3; / Whether crstep refers to a region3 entry,
    pub f): *mut *mut void (callback)(struct guest_fault,
    pub priv: *mut c_void,
}

//
// 0	1	2	3	4	5	6	7
// +-------+-------+-------+-------+-------+-------+-------+-------+
// 0	|				|	    PGT_ADDR		|
// 8	|	 VMADDR		|					|
// 16	|								|
// 24	|								|
//

extern "C" {
    pub fn dat_crstep_xchg(crstep: *mut crste, new: crste, gfn: gfn_t, asce: asce);
}
extern "C" {
    pub fn dat_free_level(table: *mut crst_table, owns_ptes: bool);
}
extern "C" {
    pub fn dat_set_asce_limit(mc: *mut kvm_s390_mmu_cache, asce: *mut asce, newtype: c_int) -> c_int;
}

extern "C" {
    pub fn dat_get_storage_key(asce: asce, gfn: gfn_t, skey: *mut skey) -> c_int;
}
extern "C" {
    pub fn dat_reset_reference_bit(asce: asce, gfn: gfn_t, skey: *mut skey) -> c_int;
}
extern "C" {
    pub fn dat_reset_skeys(asce: asce, start: gfn_t) -> c_long;
}

extern "C" {
    pub fn dat_get_ptval(table: *mut page_table, param: ptval_param) -> c_ulong;
}
extern "C" {
    pub fn dat_set_ptval(table: *mut page_table, param: ptval_param, val: c_ulong);
}

extern "C" {
    pub fn dat_set_prefix_notif_bit(asce: asce, gfn: gfn_t) -> c_int;
}

extern "C" {
    pub fn dat_test_age_gfn(asce: asce, start: gfn_t, end: gfn_t) -> bool;
}

extern "C" {
    pub fn dat_perform_essa(asce: asce, gfn: gfn_t, orc: c_int, state: *mut essa_state, dirty: *mut bool) -> c_int;
}
extern "C" {
    pub fn dat_reset_cmma(asce: asce, start_gfn: gfn_t) -> c_long;
}
extern "C" {
    pub fn dat_peek_cmma(start: gfn_t, asce: asce, count: *mut c_uint, values: *mut u8) -> c_int;
}
extern "C" {
    pub fn dat_get_cmma(asce: asce, start: *mut gfn_t, count: *mut c_uint, values: *mut u8, rem: *mut core::sync::atomic::AtomicI64) -> c_int;
}

extern "C" {
    pub fn kvm_s390_mmu_cache_topup(mc: *mut kvm_s390_mmu_cache) -> c_int;
}

extern "C" {
    pub fn kzalloc_obj(vsie_rmap: struct, _arg: GFP_KVM_S390_MMU_CACHE) -> return;
}
extern "C" {
    pub fn crdte(_arg: old.val, _arg: new.val, _arg: table, _arg: dtt, _arg: gfn_to_gpa(gfn), _arg: asce.val) -> return;
}
//
// idte_crste() - invalidate a crste entry using idte
// @crstep: pointer to the crste to be invalidated
// @gfn: a gfn mapped by the crste
// @opt: options for the idte instruction
// @asce: the asce
// @local: whether the operation is cpu-local
//
// flush without guest asce
// flush with guest asce

//
// crste_origin_large() - Return the large frame origin of a large crste
// @crste: The crste whose origin is to be returned. Should be either a
// region-3 table entry or a segment table entry, in both cases with
// FC set to 1 (large pages).
//
// Return: The origin of the large frame pointed to by @crste, or -1 if the
// crste was not large (wrong table type, or FC==0)
//
extern "C" {
    pub fn pmd_origin_large(_arg: crste.pmd) -> return;
}
extern "C" {
    pub fn pud_origin_large(_arg: crste.pud) -> return;
}

extern "C" {
    pub fn pud_origin_large(~_REGION3_MASK: pud) | (gfn_to_gpa(gfn) &) -> return;
}
extern "C" {
    pub fn pmd_origin_large(~_SEGMENT_MASK: pmd) | (gfn_to_gpa(gfn) &) -> return;
}
extern "C" {
    pub fn large_pmd_to_phys(_arg: crste.pmd, _arg: gfn) -> return;
}
extern "C" {
    pub fn large_pud_to_phys(_arg: crste.pud, _arg: gfn) -> return;
}
extern "C" {
    pub fn cspg(_arg: &crstep->val, _arg: old.val, _arg: new.val) -> return;
}
extern "C" {
    pub fn phys_to_virt(_arg: crste_origin(pmd)) -> return;
}
extern "C" {
    pub fn phys_to_virt(_arg: crste_origin(pud)) -> return;
}
extern "C" {
    pub fn phys_to_virt(_arg: crste_origin(p4d)) -> return;
}
extern "C" {
    pub fn phys_to_virt(_arg: crste_origin(pgd)) -> return;
}
extern "C" {
    pub fn phys_to_virt(_arg: crste_origin(crste.pud)) -> return;
}

extern "C" {
    pub fn phys_to_virt(_ASCE_ORIGIN: asce.val &) -> return;
}
// res = old_pgste;

extern "C" {
    pub fn dat_crstep_xchg_atomic(_arg: _CRSTEP(pmdp), _arg: _CRSTE(old), _arg: _CRSTE(new), _arg: gfn, _arg: asce) -> return;
}
extern "C" {
    pub fn dat_crstep_xchg_atomic(_arg: _CRSTEP(pudp), _arg: _CRSTE(old), _arg: _CRSTE(new), _arg: gfn, _arg: asce) -> return;
}
extern "C" {
    pub fn dat_set_slot(_arg: mc, _arg: asce, _arg: start, npages: start +, _arg: _DAT_TOKEN_PIC, _arg: PGM_ADDRESSING) -> return;
}
extern "C" {
    pub fn dat_set_slot(_arg: mc, _arg: asce, _arg: start, npages: start +, _arg: _DAT_TOKEN_NONE, _arg: 0) -> return;
}
