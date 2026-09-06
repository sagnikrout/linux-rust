//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/kvm/gmap/gmap.h
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
// Copyright IBM Corp. 2007, 2016, 2025
// Author(s): Martin Schwidefsky <schwidefsky@de.ibm.com>
// Claudio Imbrenda <imbrenda@linux.ibm.com>
//

//
// enum gmap_flags - Flags of a gmap.
//
// @GMAP_FLAG_SHADOW: The gmap is a vsie shadow gmap.
// @GMAP_FLAG_OWNS_PAGETABLES: The gmap owns all dat levels; normally 1, is 0
// only for ucontrol per-cpu gmaps, since they
// share the page tables with the main gmap.
// @GMAP_FLAG_IS_UCONTROL: The gmap is ucontrol (main gmap or per-cpu gmap).
// @GMAP_FLAG_ALLOW_HPAGE_1M: 1M hugepages are allowed for this gmap,
// independently of the page size used by userspace.
// @GMAP_FLAG_ALLOW_HPAGE_2G: 2G hugepages are allowed for this gmap,
// independently of the page size used by userspace.
// @GMAP_FLAG_PFAULT_ENABLED: Pfault is enabled for the gmap.
// @GMAP_FLAG_USES_SKEYS: If the guest uses storage keys.
// @GMAP_FLAG_USES_CMM: Whether the guest uses CMMA.
// @GMAP_FLAG_EXPORT_ON_UNMAP: Whether to export guest pages when unmapping.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gmap_flags {
    GMAP_FLAG_SHADOW = 0,
    GMAP_FLAG_OWNS_PAGETABLES,
    GMAP_FLAG_IS_UCONTROL,
    GMAP_FLAG_ALLOW_HPAGE_1M,
    GMAP_FLAG_ALLOW_HPAGE_2G,
    GMAP_FLAG_PFAULT_ENABLED,
    GMAP_FLAG_USES_SKEYS,
    GMAP_FLAG_USES_CMM,
    GMAP_FLAG_EXPORT_ON_UNMAP,
}

//
// struct gmap_struct - Guest address space.
//
// @flags: GMAP_FLAG_* flags.
// @edat_level: The edat level of this shadow gmap.
// @kvm: The vm.
// @asce: The ASCE used by this gmap.
// @list: List head used in children gmaps for the children gmap list.
// @children_lock: Protects children and scb_users.
// @children: List of child gmaps of this gmap.
// @scb_users: List of vsie_scb that use this shadow gmap.
// @parent: Parent gmap of a child gmap.
// @guest_asce: Original ASCE of this shadow gmap.
// @host_to_rmap_lock: Protects host_to_rmap.
// @host_to_rmap: Radix tree mapping host addresses to guest addresses.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gmap {
    pub flags: c_ulong,
    pub edat_level: c_uchar,
    pub invalidated: bool,
    pub kvm: *mut kvm,
    pub asce: asce,
    pub list: list_head,
    pub /: *mut *mut spinlock_t children_lock; / Protects: children, scb_users,
    pub children: list_head,
    pub scb_users: list_head,
    pub parent: *mut gmap,
    pub guest_asce: asce,
    pub /: *mut *mut spinlock_t host_to_rmap_lock; / Protects host_to_rmap,
    pub host_to_rmap: radix_tree_root,
    pub refcount: refcount_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gmap_cache {
    pub list: list_head,
    pub gmap: *mut gmap,
}

extern "C" {
    pub fn s390_replace_asce(gmap: *mut gmap) -> c_int;
}
extern "C" {
    pub fn gmap_age_gfn(gmap: *mut gmap, start: gfn_t, end: gfn_t) -> bool;
}
extern "C" {
    pub fn gmap_unmap_gfn_range(gmap: *mut gmap, slot: *mut kvm_memory_slot, start: gfn_t, end: gfn_t) -> bool;
}
extern "C" {
    pub fn gmap_try_fixup_minor(gmap: *mut gmap, fault: *mut guest_fault) -> c_int;
}
extern "C" {
    pub fn gmap_remove_child(child: *mut gmap);
}
extern "C" {
    pub fn gmap_dispose(gmap: *mut gmap);
}
extern "C" {
    pub fn gmap_sync_dirty_log(gmap: *mut gmap, start: gfn_t, end: gfn_t);
}
extern "C" {
    pub fn gmap_set_limit(gmap: *mut gmap, limit: gfn_t) -> c_int;
}
extern "C" {
    pub fn gmap_ucas_translate(mc: *mut kvm_s390_mmu_cache, gmap: *mut gmap, gaddr: *mut gpa_t) -> c_int;
}
extern "C" {
    pub fn gmap_ucas_map(gmap: *mut gmap, p_gfn: gfn_t, c_gfn: gfn_t, count: c_ulong) -> c_int;
}
extern "C" {
    pub fn gmap_ucas_unmap(gmap: *mut gmap, c_gfn: gfn_t, count: c_ulong);
}

extern "C" {
    pub fn gmap_enable_skeys(gmap: *mut gmap) -> c_int;
}

extern "C" {
    pub fn gmap_pv_destroy_range(gmap: *mut gmap, start: gfn_t, end: gfn_t, interruptible: bool) -> c_int;
}
extern "C" {
    pub fn _gmap_handle_vsie_unshadow_event(parent: *mut gmap, gfn: gfn_t);
}
extern "C" {
    pub fn gmap_split_huge_pages(gmap: *mut gmap);
}
extern "C" {
    pub fn test_bit(_arg: GMAP_FLAG_USES_SKEYS, _arg: &gmap->flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: GMAP_FLAG_USES_CMM, _arg: &gmap->flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: GMAP_FLAG_PFAULT_ENABLED, _arg: &gmap->flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: GMAP_FLAG_IS_UCONTROL, _arg: &gmap->flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: GMAP_FLAG_SHADOW, _arg: &gmap->flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: GMAP_FLAG_OWNS_PAGETABLES, _arg: &gmap->flags) -> return;
}

extern "C" {
    pub fn _gmap_unmap_prefix(gmap: *mut gmap, gfn: gfn_t, end: gfn_t, hint: bool) -> bool;
}

extern "C" {
    pub fn _gmap_unmap_prefix(_arg: gmap, _arg: gfn, _arg: end, _arg: true) -> return;
}
extern "C" {
    pub fn _gmap_unmap_prefix(_arg: gmap, _arg: gfn, _arg: end, _arg: false) -> return;
}
//
// pte_needs_unshadow() -- Check if the pte operations triggers unshadowing.
// @oldpte: the previous value for the guest pte.
// @newpte: the new pte being set.
// @pgste: the pgste for the pte entry.
//
// If the pgste.vsie_notif bit is not set, return false: the page is not
// involved in vsie and thus should not trigger an unshadow operation.
//
// If the pgste.vsie_gmem bit is set, this pte represents shadowed guest
// memory. The access rights on g3's memory should be synchronized with g1's
// and g2's. Therefore unshadowing is triggered if the new and old pte
// differ in protection, or if the new pte is invalid.
//
// If the pgste.vsie_gmem bit is not set, this pte maps the g2 dat tables
// for g3. If the entry becomes writable or absent, it becomes impossible to
// guarantee that the shadow mapping will match g2's mapping. In that case,
// trigger an unshadow event.
//
// Return: true if an unshadow event should be triggered, otherwise false.
//

extern "C" {
    pub fn _gmap_set_cmma_all(gmap: *mut gmap, dirty: bool);
}

extern "C" {
    pub fn __dat_ptep_xchg(_arg: ptep, _arg: pgste, _arg: newpte, _arg: gfn, _arg: gmap->asce, _arg: uses_skeys(gmap)) -> return;
}
extern "C" {
    pub fn _gmap_ptep_xchg(_arg: gmap, _arg: ptep, _arg: newpte, _arg: pgste, _arg: gfn, _arg: true) -> return;
}
//
// crste_needs_unshadow() -- Check if the crste operations triggers unshadowing.
// @oldcrste: the previous value for the crste.
// @newcrste: the new value for the crste.
//
// If the old crste did not have the vsie_notif bit set, return false: the
// page is not involved in vsie and thus should not trigger an unshadow
// operation. Conversely, if the bit is set, it can only be g3 memory, since
// dat tables are never mapped using large pages.
//
// Similar to the pgste.vsie_gmem case of pte_needs_unshadow(), if the
// protection bit is changing or the new page is invalid, trigger an
// unshadow event. Also trigger an unshadow event if the new crste does not
// have the vsie_notif bit set.
//
// Return: true if an unshadow event should be triggered, otherwise false.
//
// Return false even if the swap was successful, as it only
// indicates that the best effort clearing of the vsie_notif
// bit was successful. The caller will have to try again
// regardless, since the desired value has not been set.
// This pointless check is needed to silence a potential
// __must_check warning.
//
extern "C" {
    pub fn dat_crstep_xchg_atomic(_arg: crstep, _arg: oldcrste, _arg: newcrste, _arg: gfn, _arg: gmap->asce) -> return;
}
extern "C" {
    pub fn _gmap_crstep_xchg_atomic(_arg: gmap, _arg: crstep, _arg: oldcrste, _arg: newcrste, _arg: gfn, _arg: true) -> return;
}
//
// gmap_is_shadow_valid() - check if a shadow guest address space matches the
// given properties and is still valid.
// @sg: Pointer to the shadow guest address space structure.
// @asce: ASCE for which the shadow table is requested.
// @edat_level: Edat level to be used for the shadow translation.
//
// Return: true if the gmap shadow is still valid and matches the given
// properties and the caller can continue using it; false otherwise, the
// caller has to request a new shadow gmap in this case.
//
