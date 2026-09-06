//! Automatically rewritten from C Header to Rust Module
//! Source: mm/internal.h
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
// === KERNEL_MACRO_PRELUDE_START ===
macro_rules! EXPORT_SYMBOL { ($($tt:tt)*) => {}; }
macro_rules! EXPORT_SYMBOL_GPL { ($($tt:tt)*) => {}; }
macro_rules! MODULE_LICENSE { ($($tt:tt)*) => {}; }
macro_rules! MODULE_AUTHOR { ($($tt:tt)*) => {}; }
macro_rules! MODULE_DESCRIPTION { ($($tt:tt)*) => {}; }
macro_rules! MODULE_ALIAS { ($($tt:tt)*) => {}; }
macro_rules! module_init { ($($tt:tt)*) => {}; }
macro_rules! module_exit { ($($tt:tt)*) => {}; }
macro_rules! early_initcall { ($($tt:tt)*) => {}; }
macro_rules! core_initcall { ($($tt:tt)*) => {}; }
macro_rules! postcore_initcall { ($($tt:tt)*) => {}; }
macro_rules! arch_initcall { ($($tt:tt)*) => {}; }
macro_rules! subsys_initcall { ($($tt:tt)*) => {}; }
macro_rules! fs_initcall { ($($tt:tt)*) => {}; }
macro_rules! device_initcall { ($($tt:tt)*) => {}; }
macro_rules! late_initcall { ($($tt:tt)*) => {}; }
macro_rules! __setup { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_MUTEX { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_SPINLOCK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DECLARE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DEFINE { ($($tt:tt)*) => {}; }
macro_rules! ARRAY_SIZE { ($($tt:tt)*) => { 1 }; }
macro_rules! container_of { ($($tt:tt)*) => { core::ptr::null_mut() }; }
macro_rules! sizeof { ($($tt:tt)*) => { 0usize }; }
macro_rules! IS_ENABLED { ($($tt:tt)*) => { false }; }
macro_rules! DECLARE_WORK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_WAKE_Q { ($($tt:tt)*) => {}; }
macro_rules! LLIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! LIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! SET_UID { ($($tt:tt)*) => {}; }
macro_rules! SET_GID { ($($tt:tt)*) => {}; }
macro_rules! list_for_each_entry { ($($tt:tt)*) => { if false }; }
macro_rules! list_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! llist_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! pr_info_once { ($($tt:tt)*) => {}; }
macro_rules! pr_info { ($($tt:tt)*) => {}; }
macro_rules! pr_warn { ($($tt:tt)*) => {}; }
macro_rules! pr_err { ($($tt:tt)*) => {}; }
macro_rules! pr_debug { ($($tt:tt)*) => {}; }
macro_rules! early_param { ($($tt:tt)*) => {}; }
macro_rules! BUILD_BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! WARN_ON { ($($tt:tt)*) => { false }; }
macro_rules! WARN_ON_ONCE { ($($tt:tt)*) => { false }; }
macro_rules! BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! BUG { () => {}; }
macro_rules! IS_ERR { ($($tt:tt)*) => { false }; }
macro_rules! PTR_ERR { ($($tt:tt)*) => { 0 }; }
macro_rules! ERR_PTR { ($($tt:tt)*) => { core::ptr::null_mut() }; }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seq_file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cred { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inode { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct notifier_block { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw_notifier_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_header { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_root { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_set { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_dir_entry { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kern_ipc_perm { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_params { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_queue { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msgseg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_sender { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_receiver { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sembuf { pub sem_num: u16, pub sem_op: i16, pub sem_flg: i16 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem_array { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shmid_kernel { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shm_file_data { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wake_q_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct work_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct llist_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_head { pub _opaque: [u8; 0] }

pub type pid_type = c_int;
pub type cpu_pm_event = c_int;
pub type spinlock_t = u32;
pub type raw_spinlock_t = u32;
pub type kernel_cap_t = u64;
pub type cap_user_header_t = *mut c_void;
pub type cap_user_data_t = *mut c_void;
pub type async_cookie_t = u64;
pub type atomic_long_t = core::sync::atomic::AtomicI64;
pub type key_t = i32;
pub type kuid_t = u32;
pub type kgid_t = u32;
pub type int = c_int;
pub type uint = c_uint;
pub type ulong = c_ulong;
pub type long = c_long;
pub type void = c_void;

// Standard Linux Error Codes
pub const EPERM: c_int = 1;
pub const ENOENT: c_int = 2;
pub const ESRCH: c_int = 3;
pub const EINTR: c_int = 4;
pub const EIO: c_int = 5;
pub const ENXIO: c_int = 6;
pub const E2BIG: c_int = 7;
pub const ENOEXEC: c_int = 8;
pub const EBADF: c_int = 9;
pub const ECHILD: c_int = 10;
pub const EAGAIN: c_int = 11;
pub const ENOMEM: c_int = 12;
pub const EACCES: c_int = 13;
pub const EFAULT: c_int = 14;
pub const EBUSY: c_int = 16;
pub const EEXIST: c_int = 17;
pub const EXDEV: c_int = 18;
pub const ENODEV: c_int = 19;
pub const ENOTDIR: c_int = 20;
pub const EISDIR: c_int = 21;
pub const EINVAL: c_int = 22;
pub const ENFILE: c_int = 23;
pub const EMFILE: c_int = 24;
pub const ENOSPC: c_int = 28;
pub const EROFS: c_int = 30;
pub const EIDRM: c_int = 43;
pub const EOPNOTSUPP: c_int = 95;
pub const ENOTSUPP: c_int = 524;

// Standard Memory Constants
pub const PAGE_SHIFT: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
pub const GFP_KERNEL: c_uint = 0xcc0;
pub const GFP_ATOMIC: c_uint = 0x80000;
pub const GFP_NOWAIT: c_uint = 0;

// Standard Core Primitives
extern "C" {
    pub static current: *mut task_struct;
    pub fn printk(fmt: *const c_char, ...) -> c_int;
    pub fn rcu_read_lock();
    pub fn rcu_read_unlock();
    pub fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    pub fn kfree(ptr: *mut c_void);
}
// === KERNEL_MACRO_PRELUDE_END ===


// SPDX-License-Identifier: GPL-2.0-or-later
// internal.h: mm/ internal definitions
//
// Copyright (C) 2004 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

// Internal core VMA manipulation functions.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct huge_bootmem_page {
    pub list: list_head,
    pub hstate: *mut hstate,
    pub flags: c_ulong,
}

// mm/workingset.c
extern "C" {
    pub fn workingset_age_nonresident(lruvec: *mut lruvec, nr_pages: c_ulong);
}
extern "C" {
    pub fn workingset_refault(folio: *mut folio, shadow: *mut c_void);
}
extern "C" {
    pub fn workingset_activation(folio: *mut folio);
}
// mm/folio.c
extern "C" {
    pub fn folio_add_lru_vma(folio: *mut folio, vma: *mut vm_area_struct);
}
//
// Holding PMD-sized folios in per-CPU LRU cache unbalances accounting.
// Holding small numbers of low-order mTHP folios in per-CPU LRU cache
// will be sensible, but nobody has implemented and tested that yet.
//
extern "C" {
    pub fn lru_cache_disable();
}
extern "C" {
    pub fn lru_add_drain();
}
extern "C" {
    pub fn lru_add_drain_cpu(cpu: c_int);
}
extern "C" {
    pub fn lru_add_drain_cpu_zone(zone: *mut zone);
}
extern "C" {
    pub fn folio_deactivate(folio: *mut folio);
}
extern "C" {
    pub fn folio_mark_lazyfree(folio: *mut folio);
}
// mm/vmscan.c
extern "C" {
    pub fn zone_reclaimable_pages(zone: *mut zone) -> c_ulong;
}

pub const MIN_SWAPPINESS: c_int = 0;
pub const MAX_SWAPPINESS: c_int = 200;
// Just reclaim from anon folios in proactive memory reclaim

//
// Maintains state across a page table move. The operation assumes both source
// and destination VMAs already exist and are specified by the user.
//
// Partial moves are permitted, but the old and new ranges must both reside
// within a VMA.
//
// mmap lock must be held in write and VMA write locks must be held on any VMA
// that is visible.
//
// Use the PAGETABLE_MOVE() macro to initialise this struct.
//
// The old_addr and new_addr fields are updated as the page table move is
// executed.
//
// NOTE: The page table move is affected by reading from [old_addr, old_end),
// and old_addr may be updated for better page table alignment, so len_in
// represents the length of the range being copied as specified by the user.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pagetable_move_control {
//     pub /: *mut *mut *mut vm_area_old; / Source VMA.,
//     pub /: *mut *mut *mut vm_area_new; / Destination VMA.,
//     pub /: *mut *mut unsigned long old_addr; / Address from which the move begins.,
//     pub /: *mut *mut unsigned long old_end; / Exclusive address at which old range ends.,
//     pub /: *mut *mut unsigned long new_addr; / Address to move page tables to.,
//     pub /: *mut *mut unsigned long len_in; / Bytes to remap specified by user.,
//     pub /: *mut *mut bool need_rmap_locks; / Do rmap locks need to be taken?,
//     pub /: *mut *mut bool for_stack; / Is this an early temp stack being moved?,
}

//
// The set of flags that only affect watermark checking and reclaim
// behaviour. This is used by the MM to obey the caller constraints
// about IO, FS and watermark checking while ignoring placement
// hints such as HIGHMEM usage.
//

// The GFP flags allowed during early boot

// Control allocation cpuset and node placement constraints

// Do not use these with a slab allocator

//
// Different from WARN_ON_ONCE!(), no warning will be issued
// when we specify __GFP_NOWARN.
//

extern "C" {
    pub fn page_writeback_init();
}
//
// If a 16GB hugetlb folio were mapped by PTEs of all of its 4kB pages,
// its nr_pages_mapped would be 0x400000: choose the ENTIRELY_MAPPED bit
// above that range, instead of 2*(PMD_SIZE/PAGE_SIZE).  Hugetlb currently
// leaves nr_pages_mapped at 0, but avoid surprise if it participates later.
//
pub const ENTIRELY_MAPPED: c_uint = 0x800000;

//
// Flags passed to __show_mem() and show_free_areas() to suppress output in
// various contexts.
//

//
// How many individual pages have an elevated _mapcount.  Excludes
// the folio's entire_mapcount.
//
// Don't use this function outside of debugging code.
//
// Retrieve the first entry of a folio based on a provided entry within the
// folio. We cannot rely on folio->swap as there is no guarantee that it has
// been initialized. Used for calling arch_swap_restore()
//
// This is a file-backed mapping, and is about to be memory mapped - invoke its
// mmap hook and safely handle error conditions. On error, VMA hooks will be
// mutated.
//
// @file: File which backs the mapping.
// @vma:  VMA which we are mapping.
//
// Returns: 0 if success, error otherwise.
//
// OK, we tried to call the file hook for mmap(), but an error
// arose. The mapping is in an inconsistent state and we must not invoke
// any further hooks on it.
//
// If the VMA has a close hook then close it, and since closing it might leave
// it in an inconsistent state which makes the use of any hooks suspect, clear
// them down by installing dummy empty hooks.
//
// The mapping is in an inconsistent state, and no further hooks
// may be invoked upon it.
//
// unmap_vmas is in mm/memory.c
extern "C" {
    pub fn unmap_vmas(tlb: *mut mmu_gather, unmap: *mut unmap_desc);
}

extern "C" {
    pub fn __put_anon_vma(anon_vma: *mut anon_vma);
}
extern "C" {
    pub fn down_write_trylock(_arg: &anon_vma->root->rwsem) -> return;
}
extern "C" {
    pub fn down_read_trylock(_arg: &anon_vma->root->rwsem) -> return;
}
// Operations which modify VMAs.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vma_operation {
    VMA_OP_SPLIT,
    VMA_OP_MERGE_UNFAULTED,
    VMA_OP_REMAP,
    VMA_OP_FORK,
}

extern "C" {
    pub fn anon_vma_fork(vma: *mut vm_area_struct, pvma: *mut vm_area_struct) -> c_int;
}
extern "C" {
    pub fn __anon_vma_prepare(vma: *mut vm_area_struct) -> c_int;
}
extern "C" {
    pub fn unlink_anon_vmas(vma: *mut vm_area_struct);
}
extern "C" {
    pub fn __anon_vma_prepare(_arg: vma) -> return;
}
// Flags for folio_pte_batch().
pub type fpb_t = int ;
// Compare PTEs respecting the dirty bit.

// Compare PTEs respecting the soft-dirty bit.

// Compare PTEs respecting the writable bit.

//
// Merge PTE write bits: if any PTE in the batch is writable, modify the
// PTE at @ptentp to be writable.
//

//
// Merge PTE young and dirty bits: if any PTE in the batch is young or dirty,
// modify the PTE at @ptentp to be young or dirty, respectively.
//

extern "C" {
    pub fn pte_mkold(_arg: pte) -> return;
}
//
// folio_pte_batch_flags - detect a PTE batch for a large folio
// @folio: The large folio to detect a PTE batch for.
// @vma: The VMA. Only relevant with FPB_MERGE_WRITE, otherwise can be NULL.
// @ptep: Page table pointer for the first entry.
// @ptentp: Pointer to a COPY of the first page table entry whose flags this
// function updates based on @flags if appropriate.
// @max_nr: The maximum number of table entries to consider.
// @flags: Flags to modify the PTE batch semantics.
//
// Detect a PTE batch: consecutive (present) PTEs that map consecutive
// pages of the same large folio in a single VMA and a single page table.
//
// All PTEs inside a PTE batch have the same PTE bits set, excluding the PFN,
the accessed bit, writable bit, dirty bit (unless FPB_RESPECT_DIRTY is set)
// and soft-dirty bit (unless FPB_RESPECT_SOFT_DIRTY is set).
//
// @ptep must map any page of the folio. max_nr must be at least one and
// must be limited by the caller so scanning cannot exceed a single VMA and
// a single page table.
//
// Depending on the FPB_MERGE_* flags, the pte stored at @ptentp will
// be updated: it's crucial that a pointer to a COPY of the first
// page table entry, obtained through ptep_get(), is provided as @ptentp.
//
// This function will be inlined to optimize based on the input parameters;
// consider using folio_pte_batch() instead if applicable.
//
// Return: the number of table entries in the batch.
//
// Ensure this is a pointer to a copy not a pointer into a page table.
// If this is a stack value, it won't be a valid virtual address, but
// that's fine because it also cannot be pointing into the page table.
//
// Limit max_nr to the actual remaining PFNs in the folio we could batch.
// ptentp = pte_mkwrite(*ptentp, vma);
// ptentp = pte_mkyoung(*ptentp);
// ptentp = pte_mkdirty(*ptentp);
extern "C" {
    pub fn min(_arg: nr, _arg: max_nr) -> return;
}
//
// pte_move_swp_offset - Move the swap entry offset field of a swap pte
// forward or backward by delta
// @pte: The initial pte state; must be a swap entry
// @delta: The direction and the offset we are moving; forward if delta
// is positive; backward if delta is negative
//
// Moves the swap offset, while maintaining all other fields, including
// swap type, and any swp pte bits. The resulting pte is returned.
//
// pte_next_swp_offset - Increment the swap entry offset field of a swap pte.
// @pte: The initial pte state; must be a swap entry.
//
// Increments the swap offset, while maintaining all other fields, including
// swap type, and any swp pte bits. The resulting pte is returned.
//
extern "C" {
    pub fn pte_move_swp_offset(_arg: pte, _arg: 1) -> return;
}
//
// swap_pte_batch - detect a PTE batch for a set of contiguous swap entries
// @start_ptep: Page table pointer for the first entry.
// @max_nr: The maximum number of table entries to consider.
// @pte: Page table entry for the first entry.
//
// Detect a batch of contiguous swap entries: consecutive (non-present) PTEs
// containing swap entries all with consecutive offsets and targeting the same
// swap type, all with matching swp pte bits.
//
// max_nr must be at least one and must be limited by the caller so scanning
// cannot exceed a single page table.
//
// Return: the number of table entries in the batch.
//

extern "C" {
    pub fn __vmf_anon_prepare(vmf: *mut vm_fault) -> vm_fault_t;
}
extern "C" {
    pub fn do_swap_page(vmf: *mut vm_fault) -> vm_fault_t;
}
extern "C" {
    pub fn folio_rotate_reclaimable(folio: *mut folio);
}
extern "C" {
    pub fn __folio_end_writeback(folio: *mut folio) -> bool;
}
extern "C" {
    pub fn deactivate_file_folio(folio: *mut folio);
}
extern "C" {
    pub fn folio_activate(folio: *mut folio);
}
extern "C" {
    pub fn free_pgtables(tlb: *mut mmu_gather, desc: *mut unmap_desc);
}
extern "C" {
    pub fn pmd_install(mm: *mut mm_struct, pmd: *mut pmd_t, pte: *mut pgtable_t);
}
//
// sync_with_folio_pmd_zap - sync with concurrent zapping of a folio PMD
// @mm: The mm_struct.
// @pmdp: Pointer to the pmd that was found to be pmd_none().
//
// When we find a pmd_none() while unmapping a folio without holding the PTL,
// zap_huge_pmd() may have cleared the PMD but not yet modified the folio to
// indicate that it's unmapped. Skipping the PMD without synchronization could
// make folio unmapping code assume that unmapping failed.
//
// Wait for concurrent zapping to complete by grabbing the PTL.
//
extern "C" {
    pub fn zap_vma_for_reaping(vma: *mut vm_area_struct) -> c_int;
}
extern "C" {
    pub fn page_cache_ra_order(: *mut readahead_control, : *mut file_ra_state);
}
extern "C" {
    pub fn force_page_cache_ra(: *mut readahead_control, nr: c_ulong);
}
extern "C" {
    pub fn truncate_inode_folio(mapping: *mut address_space, folio: *mut folio) -> c_int;
}
extern "C" {
    pub fn mapping_evict_folio(mapping: *mut address_space, folio: *mut folio) -> c_long;
}
//
// folio_evictable - Test whether a folio is evictable.
// @folio: The folio to test.
//
// Test whether @folio is evictable -- i.e., should be placed on
// active/inactive lists vs unevictable list.
//
// Reasons folio might not be evictable:
// 1. folio's mapping marked unevictable
// 2. One of the pages in the folio is part of an mlocked VMA
//
// Prevent address_space of inode and swap cache from being freed
//
// Turn a non-refcounted page (->_refcount == 0) into refcounted with
// a count of one.
//
// Return true if a folio needs ->release_folio() calling upon it.
//
// Maximum number of reclaim retries without progress before the OOM
// killer is consider the only way forward.
//
pub const MAX_RECLAIM_RETRIES: c_int = 16;
//
// in mm/vmscan.c:
//
extern "C" {
    pub fn folio_isolate_lru(folio: *mut folio) -> bool;
}
extern "C" {
    pub fn folio_putback_lru(folio: *mut folio);
}
extern "C" {
    pub fn reclaim_throttle(pgdat: *mut pg_data_t, reason: vmscan_throttle_state);
}
//
// in mm/rmap.c:
//
// in mm/khugepaged.c
//
extern "C" {
    pub fn set_recommended_min_free_kbytes();
}
//
// in mm/page_alloc.c
//

extern "C" {
    pub fn setup_per_zone_wmarks();
}
extern "C" {
    pub fn calculate_min_free_kbytes();
}
extern "C" {
    pub fn init_per_zone_wmark_min() -> int __meminit;
}
extern "C" {
    pub fn __isolate_free_page(page: *mut page, order: c_uint) -> c_int;
}
//
// This will have no effect, other than possibly generating a warning, if the
// caller passes in a non-large folio.
//

extern "C" {
    pub fn __folio_unqueue_deferred_split(folio: *mut folio) -> bool;
}
//
// At this point, there is no one trying to add the folio to
// deferred_list. If folio is not in deferred_list, it's safe
// to check without acquiring the list_lru lock.
//
extern "C" {
    pub fn __folio_unqueue_deferred_split(_arg: folio) -> return;
}

//
// in mm/compaction.c
//
// compact_control is used to track pages being migrated and the free pages
// they are being migrated to during memory compaction. The free_pfn starts
// at the end of a zone and migrate_pfn begins at the start. Movable pages
// are moved to the end of a zone during a compaction run and the run
// completes when free_pfn <= migrate_pfn
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct compact_control {
//     pub /: *mut *mut list_head freepages[NR_PAGE_ORDERS]; / List of free pages to migrate to,
//     pub /: *mut *mut list_head migratepages; / List of pages being migrated,
//     pub /: *mut *mut unsigned int nr_freepages; / Number of isolated free pages,
//     pub /: *mut *mut unsigned int nr_migratepages; / Number of pages to migrate,
//     pub /: *mut *mut unsigned long free_pfn; / isolate_freepages search base,
//
// Acts as an in/out parameter to page isolation for migration.
// isolate_migratepages uses it as a search base.
// isolate_migratepages_block will update the value to the next pfn
// after the last isolated one.
//
    pub migrate_pfn: c_ulong,
//     pub /: *mut *mut unsigned long fast_start_pfn; / a pfn to start linear scan from,
    pub zone: *mut zone,
    pub total_migrate_scanned: c_ulong,
    pub total_free_scanned: c_ulong,
//     pub /: *mut *mut unsigned short fast_search_fail;/ failures to use free list searches,
//     pub /: *mut *mut short search_order; / order to start a fast search at,
//     pub /: *const *const gfp_t gfp_mask; / gfp mask of a direct compactor,
//     pub /: *mut *mut int order; / order a direct compactor needs,
//     pub /: *mut *mut int migratetype; / migratetype of direct compactor,
//     pub /: *const *const unsigned int alloc_flags; / alloc flags of a direct compactor,
//     pub /: *const *const int highest_zoneidx; / zone index of a direct compactor,
//     pub /: *mut *mut migrate_mode mode; / Async or sync migration mode,
//     pub /: *mut *mut bool ignore_skip_hint; / Scan blocks even if marked skip,
//     pub /: *mut *mut bool no_set_skip_hint; / Don't mark blocks for skipping,
//     pub /: *mut *mut bool ignore_block_suitable; / Scan blocks considered unsuitable,
//     pub /: *mut *mut bool direct_compaction; / False from kcompactd or /proc/...,
//     pub /: *mut *mut bool proactive_compaction; / kcompactd proactive compaction,
//     pub /: *mut *mut bool whole_zone; / Whole zone should/has been scanned,
//     pub /: *mut *mut bool contended; / Signal lock contention,
    pub Used: *mut *mut bool finish_pageblock; / Scan the remainder of a pageblock.,
// when there are potentially transient
// isolation or migration failures to
// ensure forward progress.
//
//     pub /: *mut *mut bool alloc_contig; / alloc_contig_range allocation,
}

//
// Used in direct compaction when a page should be taken from the freelists
// immediately when one is created during the free path.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct capture_control {
    pub zone: *mut zone,
    pub migratetype: c_int,
//
// Allocation request order. May differ from the compaction
// order: defrag_mode promotes sub-block allocations to
// pageblock-order compaction; capture still matches at the
// original allocation order so prep_new_page() is consistent.
//
    pub order: c_int,
    pub page: *mut page,
}

extern "C" {
    pub fn cma_validate_zones(cma: *mut cma) -> bool;
}

// mm/util.c

extern "C" {
    pub fn unmap_mapping_folio(folio: *mut folio);
}
//
// NOTE: This function can't tell whether the folio is "fully mapped" in the
// range.
// "fully mapped" means all the pages of folio is associated with the page
// table of range while this function just check whether the folio range is
// within the range [start, end). Function caller needs to do page table
// check if it cares about the page table association.
//
// Typical usage (like mlock or madvise) is:
// Caller knows at least 1 page of folio is associated with page table of VMA
// and the range [start, end) is intersect with the VMA range. Caller wants
// to know whether the folio is fully associated with the range. It calls
// this function to check whether the folio is in the range first. Then checks
// the page table to know whether the folio is fully mapped to the range.
//
// if folio start address is not in vma range
extern "C" {
    pub fn folio_within_range(_arg: folio, _arg: vma, _arg: vma->vm_start, _arg: vma->vm_end) -> return;
}
//
// mlock_vma_folio() and munlock_vma_folio():
// should be called with vma's mmap_lock held for read or write,
// under page table lock for the pte/pmd being added or removed.
//
// mlock is usually called at the end of folio_add_*_rmap_*(), munlock at
// the end of folio_remove_rmap_*(); but new anon folios are managed by
// folio_add_lru_vma() calling mlock_new_folio().
//
extern "C" {
    pub fn mlock_folio(folio: *mut folio);
}
//
// The VM_SPECIAL check here serves two purposes.
// 1) VM_IO check prevents migration from double-counting during mlock.
// 2) Although mmap_region() and mlock_fixup() take care that VM_LOCKED
// is never left set on a VM_SPECIAL vma, there is an interval while
// file->f_op->mmap() is using vm_insert_page(s), when VM_LOCKED may
// still be set while VM_SPECIAL bits are added: so ignore it then.
//
extern "C" {
    pub fn munlock_folio(folio: *mut folio);
}
//
// munlock if the function is called. Ideally, we should only
// do munlock if any page of folio is unmapped from VMA and
// cause folio not fully mapped to VMA.
//
// But it's not easy to confirm that's the situation. So we
// always munlock the folio and page reclaim will correct it
// if it's wrong.
//
extern "C" {
    pub fn mlock_new_folio(folio: *mut folio);
}
extern "C" {
    pub fn need_mlock_drain(cpu: c_int) -> bool;
}
extern "C" {
    pub fn mlock_drain_local();
}
extern "C" {
    pub fn mlock_drain_remote(cpu: c_int);
}
extern "C" {
    pub fn maybe_pmd_mkwrite(pmd: pmd_t, vma: *mut vm_area_struct) -> pmd_t;
}
// Check for address beyond vma (or wrapped through 0?)
// Test above avoids possibility of wrap to 0 on 32-bit
//
// vma_filebacked_address - Find the virtual address a file-backed page range is
// mapped at.
// @vma: The vma which maps this object.
// @pgoff: The page offset within its object.
// @nr_pages: The number of pages to consider.
//
// Returns: If any page in this range is mapped by this VMA, return the first
// address where any of these pages appear.  Otherwise, return -EFAULT.
//
extern "C" {
    pub fn __vma_address(_arg: vma, _arg: pgoff, _arg: vma_start_pgoff(vma), _arg: nr_pages) -> return;
}
//
// vma_anon_address - Find the virtual address an anonymous page range is mapped
// at.
// @vma: The vma which maps this object.
// @pgoff_anon: The anonymous page index belonging to the folio.
// @nr_pages: The number of pages to consider.
//
// This is only valid for anonymous or MAP_PRIVATE-mapped file-backed VMAs.
//
// Returns: If any page in this range is mapped by this VMA, return the first
// address where any of these pages appear. Otherwise, return -EFAULT.
//
extern "C" {
    pub fn __vma_address(_arg: vma, _arg: pgoff_anon, _arg: vma_start_anon_pgoff(vma), _arg: nr_pages) -> return;
}
//
// At what user virtual address will none of the range be found in vma?
// Assumes that vma_address() already returned a good starting address.
//
// Common case, plus ->pgoff is invalid for KSM
// Check for address beyond vma (or wrapped through 0?)
//
// FAULT_FLAG_RETRY_NOWAIT means we don't want to wait on page locks or
// anything, so we only pin the file and drop the mmap_lock if only
// FAULT_FLAG_ALLOW_RETRY is set, while this is the first attempt.
//

extern "C" {
    pub fn find_next_best_node(node: c_int, used_node_mask: *mut nodemask_t) -> c_int;
}

pub const node_reclaim_mode: c_int = 0;

// Is any node_reclaim_mode bit set?
//
// mm/memory-failure.c
//

extern "C" {
    pub fn unmap_poisoned_folio(folio: *mut folio, pfn: c_ulong, must_kill: bool) -> c_int;
}
extern "C" {
    pub fn shake_folio(folio: *mut folio);
}
extern "C" {
    pub fn hwpoison_filter_func_t(p: *mut page) -> typedef int;
}
extern "C" {
    pub fn hwpoison_filter_register(filter: *mut hwpoison_filter_func_t);
}
extern "C" {
    pub fn hwpoison_filter_unregister();
}
pub const MAGIC_HWPOISON: c_uint = 0x48575053U	/* HWPS */;
extern "C" {
    pub fn SetPageHWPoisonTakenOff(page: *mut page);
}
extern "C" {
    pub fn ClearPageHWPoisonTakenOff(page: *mut page);
}
extern "C" {
    pub fn take_page_off_buddy(page: *mut page) -> bool;
}
extern "C" {
    pub fn put_page_back_buddy(page: *mut page) -> bool;
}

extern "C" {
    pub fn reclaim_pages(folio_list: *mut list_head) -> c_ulong;
}
//
// only for MM internal work items which do not depend on
// any allocations or locks which might depend on allocations
//

extern "C" {
    pub fn try_to_unmap_flush();
}
extern "C" {
    pub fn try_to_unmap_flush_dirty();
}
extern "C" {
    pub fn flush_tlb_batched_pending(mm: *mut mm_struct);
}

extern "C" {
    pub fn setup_zone_pageset(zone: *mut zone);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct migration_target_control {
//     pub /: *mut *mut int nid; / preferred node id,
    pub nmask: *mut nodemask_t,
    pub gfp_mask: gfp_t,
    pub reason: migrate_reason,
}

//
// mm/filemap.c
//

extern "C" {
    pub fn free_zone_device_folio(folio: *mut folio);
}
extern "C" {
    pub fn migrate_device_coherent_folio(folio: *mut folio) -> c_int;
}
//
// mm/gup.c
//
// mm/huge_memory.c
//
// Parses a string with mem suffixes into its order. Useful to parse kernel
// parameters.
//
// mark page accessed
// a retry, previous pass started an IO
// we are working on non-current tsk/mm
// pages must be released via unpin_user_page
// gup_fast: prevent fall-back to slow gup
// allow unlocking the mmap lock
// VMA lookup+checks compatible with MADV_POPULATE_(READ|WRITE)

//
// Indicates for which pages that are write-protected in the page table,
// whether GUP has to trigger unsharing via FAULT_FLAG_UNSHARE such that the
// GUP pin will remain consistent with the pages mapped into the page tables
// of the MM.
//
// Temporary unmapping of PageAnonExclusive() pages or clearing of
// PageAnonExclusive() has to protect against concurrent GUP:
// * Ordinary GUP: Using the PT lock
// * GUP-fast and fork(): mm->write_protect_seq
// * GUP-fast and KSM or temporary unmapping (swap, migration): see
// folio_try_share_anon_rmap_*()
//
// Must be called with the (sub)page that's actually referenced via the
// page table entry, which might not necessarily be the head page for a
// PTE-mapped THP.
//
// If the vma is NULL, we're coming from the GUP-fast path and might have
// to fallback to the slow path just to lookup the vma.
//
// FOLL_WRITE is implicitly handled correctly as the page table entry
// has to be writable -- and if it references (part of) an anonymous
// folio, that part is required to be marked exclusive.
//
// Note: PageAnon(page) is stable until the page is actually getting
// freed.
//
// We only care about R/O long-term pining: R/O short-term
// pinning does not have the semantics to observe successive
// changes through the process page tables.
//
// We really need the vma ...
//
// ... because we only care about writable private ("COW")
// mappings where we have to break COW early.
//
extern "C" {
    pub fn vma_is_cow_mapping(_arg: vma) -> return;
}
// Paired with a memory barrier in folio_try_share_anon_rmap_*().
//
// Note that KSM pages cannot be exclusive, and consequently,
// cannot get pinned.
//
// NOTE: we must check this before VM_SOFTDIRTY on soft-dirty
// enablements, because when without soft-dirty being compiled in,
VM_SOFTDIRTY is defined as 0x0, then !(vm_flags & VM_SOFTDIRTY)
// will be constantly true.
//
// Soft-dirty is kind of special: its tracking is enabled when the
// vma flags not set.
//
extern "C" {
    pub fn vma_soft_dirty_enabled(!pmd_soft_dirty(pmd: vma) &&) -> return;
}
extern "C" {
    pub fn vma_soft_dirty_enabled(!pte_soft_dirty(pte: vma) &&) -> return;
}
// shrinker related functions
extern "C" {
    pub fn shmem_inode_acct_blocks(inode: *mut inode, pages: c_long) -> c_int;
}
extern "C" {
    pub fn shmem_recalc_inode(inode: *mut inode, alloced: c_long, swapped: c_long) -> bool;
}

extern "C" {
    pub fn shrinker_debugfs_add(shrinker: *mut shrinker) -> c_int;
}

// debugfs_id = -1;

// Only track the nodes of mappings with shadow entries
extern "C" {
    pub fn workingset_update_node(node: *mut xa_node);
}

// mremap.c
extern "C" {
    pub fn move_page_tables(pmc: *mut pagetable_move_control) -> c_ulong;
}

extern "C" {
    pub fn accept_page(page: *mut page);
}

// pagewalk.c
extern "C" {
    pub fn dup_mm_exe_file(mm: *mut mm_struct, oldmm: *mut mm_struct);
}
extern "C" {
    pub fn dup_mmap(mm: *mut mm_struct, oldmm: *mut mm_struct) -> c_int;
}
extern "C" {
    pub fn remap_pfn_range_prepare(desc: *mut vm_area_desc) -> c_int;
}
extern "C" {
    pub fn simple_ioremap_prepare(desc: *mut vm_area_desc) -> c_int;
}
// Remap does the actual work.
//
// When we succeed an mmap action or just before we unmap a VMA on error, we
// need to ensure any rmap lock held is released. On unmap it's required to
// avoid a deadlock.
//

extern "C" {
    pub fn READ_ONCE(_arg: sysctl_max_map_count) -> return;
}
//
// In PREEMPT_RT spin_trylock() will call raw_spin_lock() which is
// unsafe in NMI. If spin_trylock() is called from hard IRQ the current
// task may be waiting for one rt_spin_lock, but rt_spin_trylock() will
// mark the task as the owner of another rt_spin_lock which will
// confuse PI logic, so return immediately if called from hard IRQ or
// NMI.
//
// Note, irqs_disabled() case is ok. spin_trylock() can be called
// from raw_spin_lock_irqsave region.
//
// On UP, spin_trylock() always succeeds even when it is locked