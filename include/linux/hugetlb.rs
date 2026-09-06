//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/hugetlb.h
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

extern "C" {
    pub fn free_huge_folio(folio: *mut folio);
}

//
// For HugeTLB page, there are more metadata to save in the struct page. But
// the head struct page cannot meet our needs, so we have to abuse other tail
// struct page to store the metadata.
//
pub const __NR_USED_SUBPAGE: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hugepage_subpool {
    pub lock: spinlock_t,
    pub count: c_long,
    pub /: *mut *mut long max_hpages; / Maximum huge pages or -1 if no maximum.,
    pub /: *mut *mut long used_hpages; / Used count against maximum, includes,
// both allocated and reserved pages.
    pub hstate: *mut hstate,
    pub /: *mut *mut long min_hpages; / Minimum huge pages or -1 if no minimum.,
    pub /: *mut *mut long rsv_hpages; / Pages reserved against global pool to,
// satisfy minimum size.
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct resv_map {
    pub refs: kref,
    pub lock: spinlock_t,
    pub regions: list_head,
    pub adds_in_progress: c_long,
    pub region_cache: list_head,
    pub region_cache_count: c_long,
    pub rw_sema: rw_semaphore,

//
// On private mappings, the counter to uncharge reservations is stored
// here. If these fields are 0, then either the mapping is shared, or
// cgroup accounting is disabled for this resv_map.
//
    pub reservation_counter: *mut page_counter,
    pub pages_per_hpage: c_ulong,
    pub css: *mut cgroup_subsys_state,

}

//
// Region tracking -- allows tracking of reservations and instantiated pages
// across the pages in a mapping.
//
// The region data structures are embedded into a resv_map and protected
// by a resv_map's lock.  The set of regions within the resv_map represent
// reservations for huge pages, or huge pages that have already been
// instantiated within the map.  The from and to elements are huge page
// indices into the associated mapping.  from indicates the starting index
// of the region.  to represents the first index past the end of  the region.
//
// For example, a file region structure with from == 0 and to == 4 represents
// four huge pages in a mapping.  It is important to note that the to element
// represents the first element past the end of the region. This is used in
// arithmetic as 4(to) - 0(from) = 4 huge pages in the region.
//
// Interval notation of the form [from, to) will be used to indicate that
// the endpoint from is inclusive and to is exclusive.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct file_region {
    pub link: list_head,
    pub from: c_long,
    pub to: c_long,

//
// On shared mappings, each reserved region appears as a struct
// file_region in resv_map. These fields hold the info needed to
// uncharge each reservation.
//
    pub reservation_counter: *mut page_counter,
    pub css: *mut cgroup_subsys_state,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hugetlb_vma_lock {
    pub refs: kref,
    pub rw_sema: rw_semaphore,
    pub vma: *mut vm_area_struct,
}

extern "C" {
    pub fn resv_map_release(ref: *mut kref);
}

extern "C" {
    pub fn hugepage_put_subpool(spool: *mut hugepage_subpool);
}
extern "C" {
    pub fn hugetlb_dup_vma_private(vma: *mut vm_area_struct);
}
extern "C" {
    pub fn clear_vma_resv_huge_pages(vma: *mut vm_area_struct);
}
extern "C" {
    pub fn hugetlb_report_meminfo(: *mut seq_file);
}
extern "C" {
    pub fn hugetlb_report_node_meminfo(buf: *mut c_char, len: c_int, nid: c_int) -> c_int;
}
extern "C" {
    pub fn hugetlb_show_meminfo_node(nid: c_int);
}
extern "C" {
    pub fn hugetlb_total_pages() -> c_ulong;
}

extern "C" {
    pub fn folio_isolate_hugetlb(folio: *mut folio, list: *mut list_head) -> bool;
}
extern "C" {
    pub fn get_hwpoison_hugetlb_folio(folio: *mut folio, hugetlb: *mut bool, unpoison: bool) -> c_int;
}
extern "C" {
    pub fn folio_putback_hugetlb(folio: *mut folio);
}
extern "C" {
    pub fn hugetlb_fix_reserve_counts(inode: *mut inode);
}
extern "C" {
    pub fn hugetlb_fault_mutex_hash(mapping: *mut address_space, idx: pgoff_t) -> u32;
}
extern "C" {
    pub fn hugetlb_bootmem_struct_page_init();
}
extern "C" {
    pub fn hugetlb_bootmem_alloc();
}
extern "C" {
    pub fn hugetlb_bootmem_set_nodes();
}
// arch callbacks

//
// pte_offset_huge() and pte_alloc_huge() are helpers for those architectures
// which may go down to the lowest PTE level in their huge_pte_offset() and
// huge_pte_alloc(): to avoid reliance on pte_offset_map() without pte_unmap().
//
extern "C" {
    pub fn pte_offset_kernel(_arg: pmd, _arg: address) -> return;
}
extern "C" {
    pub fn pte_alloc(_arg: mm, pte_offset_huge(pmd: pmd) ? NULL :, _arg: address) -> return;
}

//
// huge_pte_offset(): Walk the hugetlb pgtable until the last level PTE.
// Returns the pte_t* if found, or NULL if the address is not mapped.
//
// IMPORTANT: we should normally not directly call this function, instead
// this is only a common interface to implement arch-specific
// walker. Please use hugetlb_walk() instead, because that will attempt to
// verify the locking for you.
//
// Since this function will walk all the pgtable pages (including not only
// high-level pgtable page, but also PUD entry that can be unshared
// concurrently for VM_SHARED), the caller of this function should be
// responsible of its thread safety.  One can follow this rule:
//
// (1) For private mappings: pmd unsharing is not possible, so holding the
// mmap_lock for either read or write is sufficient. Most callers
// already hold the mmap_lock, so normally, no special action is
// required.
//
// (2) For shared mappings: pmd unsharing is possible (so the PUD-ranged
// pgtable page can go away from under us!  It can be done by a pmd
// unshare with a follow up munmap() on the other process), then we
// need either:
//
// (2.1) hugetlb vma lock read or write held, to make sure pmd unshare
// won't happen upon the range (it also makes sure the pte_t we
// read is the right and stable one), or,
//
// (2.2) hugetlb mapping i_mmap_rwsem lock held read or write, to make
// sure even if unshare happened the racy unmap() will wait until
// i_mmap_rwsem is released.
//
// Option (2.1) is the safest, which guarantees pte stability from pmd
// sharing pov, until the vma lock released.  Option (2.2) doesn't protect
// a concurrent pmd unshare, but it makes sure the pgtable page is safe to
// access.
//
extern "C" {
    pub fn hugetlb_mask_last_page(h: *mut hstate) -> c_ulong;
}
extern "C" {
    pub fn huge_pmd_unshare_flush(tlb: *mut mmu_gather, vma: *mut vm_area_struct);
}
extern "C" {
    pub fn hugetlb_vma_lock_read(vma: *mut vm_area_struct);
}
extern "C" {
    pub fn hugetlb_vma_unlock_read(vma: *mut vm_area_struct);
}
extern "C" {
    pub fn hugetlb_vma_lock_write(vma: *mut vm_area_struct);
}
extern "C" {
    pub fn hugetlb_vma_unlock_write(vma: *mut vm_area_struct);
}
extern "C" {
    pub fn hugetlb_vma_trylock_write(vma: *mut vm_area_struct) -> c_int;
}
extern "C" {
    pub fn hugetlb_vma_assert_locked(vma: *mut vm_area_struct);
}
extern "C" {
    pub fn hugetlb_vma_lock_release(kref: *mut kref);
}
extern "C" {
    pub fn hugetlb_unshare_all_pmds(vma: *mut vm_area_struct);
}
extern "C" {
    pub fn fixup_hugetlb_reservations(vma: *mut vm_area_struct);
}
extern "C" {
    pub fn hugetlb_split(vma: *mut vm_area_struct, addr: c_ulong);
}
extern "C" {
    pub fn arch_hugetlb_cma_order() -> c_uint;
}

//
// The file will be used as an shm file so shmfs accounting rules
// apply
//
// The file is being created on the internal vfs mount and shmfs
// accounting rules do not apply
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hugetlbfs_sb_info {
    pub /: *mut *mut long max_inodes; / inodes allowed,
    pub /: *mut *mut long free_inodes; / inodes free,
    pub stat_lock: spinlock_t,
    pub hstate: *mut hstate,
    pub spool: *mut hugepage_subpool,
    pub uid: kuid_t,
    pub gid: kgid_t,
    pub mode: umode_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hugetlbfs_inode_info {
    pub vfs_inode: inode,
    pub resv_map: *mut resv_map,
    pub seals: c_uint,
}

extern "C" {
    pub fn container_of(_arg: inode, hugetlbfs_inode_info: struct, _arg: vfs_inode) -> return;
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENOSYS) -> return;
}

//
// huegtlb page specific state flags.  These flags are located in page.private
// of the hugetlb head page.  Functions created via the below macros should be
// used to manipulate these flags.
//
// HPG_restore_reserve - Set when a hugetlb page consumes a reservation at
// allocation time.  Cleared when page is fully instantiated.  Free
// routine checks flag to restore a reservation on error paths.
// Synchronization:  Examined or modified by code that knows it has
// the only reference to page.  i.e. After allocation but before use
// or when the page is being freed.
// HPG_migratable  - Set after a newly allocated page is added to the page
// cache and/or page tables.  Indicates the page is a candidate for
// migration.
// Synchronization:  Initially set after new page allocation with no
// locking.  When examined and modified during migration processing
// (isolate, migrate, putback) the hugetlb_lock is held.
// HPG_temporary - Set on a page that is temporarily allocated from the buddy
// allocator.  Typically used for migration target pages when no pages
// are available in the pool.  The hugetlb free page path will
// immediately free pages with this flag set to the buddy allocator.
// Synchronization: Can be set after huge page allocation from buddy when
// code knows it has only reference.  All other examinations and
// modifications require hugetlb_lock.
// HPG_freed - Set when page is on the free lists.
// Synchronization: hugetlb_lock held for examination and modification.
// HPG_vmemmap_optimized - Set when the vmemmap pages of the page are freed.
// HPG_raw_hwp_unreliable - Set when the hugetlb page has a hwpoison sub-page
// that is not tracked by raw_hwp_page list.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hugetlb_page_flags {
    HPG_restore_reserve = 0,
    HPG_migratable,
    HPG_temporary,
    HPG_freed,
    HPG_vmemmap_optimized,
    HPG_raw_hwp_unreliable,
    HPG_cma,
    __NR_HPAGEFLAGS,
}

//
// Macros to create test, set and clear function definitions for
// hugetlb specific page flags.
//

//
// Create functions associated with hugetlb page flags
//

pub const HSTATE_NAME_LEN: c_int = 32;
// Defines one hugetlb page size
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hstate {
    pub resize_lock: mutex,
    pub resize_key: lock_class_key,
    pub next_nid_to_alloc: c_int,
    pub next_nid_to_free: c_int,
    pub order: c_uint,
    pub demote_order: c_uint,
    pub mask: c_ulong,
    pub max_huge_pages: c_ulong,
    pub nr_huge_pages: c_ulong,
    pub free_huge_pages: c_ulong,
    pub resv_huge_pages: c_ulong,
    pub surplus_huge_pages: c_ulong,
    pub nr_overcommit_huge_pages: c_ulong,
    pub hugepage_activelist: list_head,
    pub hugepage_freelists: [list_head; MAX_NUMNODES],
    pub max_huge_pages_node: [c_uint; MAX_NUMNODES],
    pub nr_huge_pages_node: [c_uint; MAX_NUMNODES],
    pub free_huge_pages_node: [c_uint; MAX_NUMNODES],
    pub surplus_huge_pages_node: [c_uint; MAX_NUMNODES],
    pub name: [c_char; HSTATE_NAME_LEN],
}

pub const HUGE_BOOTMEM_HVO: c_uint = 0x0001;
pub const HUGE_BOOTMEM_ZONES_VALID: c_uint = 0x0002;
pub const HUGE_BOOTMEM_CMA: c_uint = 0x0004;
extern "C" {
    pub fn isolate_or_dissolve_huge_folio(folio: *mut folio, list: *mut list_head) -> c_int;
}
extern "C" {
    pub fn replace_free_hugepage_folios(start_pfn: c_ulong, end_pfn: c_ulong) -> c_int;
}
extern "C" {
    pub fn wait_for_freed_hugetlb_folios();
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mempolicy_interpreted {
    pub nid: c_int,
    pub nodemask: *mut nodemask_t,
    pub mode: mempolicy_mode,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hugetlb_alloc_flag {
    HUGETLB_ALLOC_CHARGE_CGROUP_RSVD_BIT = 0,
    HUGETLB_ALLOC_USE_GLOBAL_RESERVATIONS_BIT,
}

// arch callback
extern "C" {
    pub fn __alloc_bootmem_huge_page(h: *mut hstate, nid: c_int) -> *mut void __init;
}
extern "C" {
    pub fn arch_alloc_bootmem_huge_page(h: *mut hstate, nid: c_int) -> *mut void __init;
}
extern "C" {
    pub fn hugetlb_node_alloc_supported() -> bool __init;
}
extern "C" {
    pub fn hugetlb_add_hstate(order: unsigned) -> void __init;
}
extern "C" {
    pub fn arch_hugetlb_valid_size(size: c_ulong) -> bool __init;
}

pub const HUGE_MAX_HSTATE: c_int = 1;

extern "C" {
    pub fn hstate_inode(_arg: file_inode(f)) -> return;
}
extern "C" {
    pub fn size_to_hstate(page_size_log: 1UL <<) -> return;
}
extern "C" {
    pub fn hstate_file(_arg: vma->vm_file) -> return;
}
//
// hugetlb_linear_page_index() - linear_page_index() but in hugetlb
// page size granularity.
// @vma: the hugetlb VMA
// @address: the virtual address within the VMA
//
// Return: the page offset within the mapping in huge page units.
//
extern "C" {
    pub fn linear_page_index(_arg: vma, huge_page_order(h: address) >>) -> return;
}
extern "C" {
    pub fn order_is_gigantic(_arg: huge_page_order(h)) -> return;
}
extern "C" {
    pub fn filemap_lock_folio(_arg: mapping, huge_page_order(h): idx <<) -> return;
}

extern "C" {
    pub fn pte_mkhuge(_arg: entry) -> return;
}

//
// Some architectures do their own bootmem allocation, so they can't use
// early CMA allocation.
//

extern "C" {
    pub fn size_to_hstate(_arg: folio_size(folio)) -> return;
}
extern "C" {
    pub fn dissolve_free_hugetlb_folio(folio: *mut folio) -> c_int;
}

extern "C" {
    pub fn folio_clear_hugetlb_hwpoison(folio: *mut folio);
}

extern "C" {
    pub fn arch_hugetlb_migration_supported(_arg: h) -> return;
}
//
// Movability check is different as compared to migration check.
// It determines whether or not a huge page should be placed on
// movable zone or not. Movability of any huge page should be
// required only if huge page size is supported for migration.
// There won't be any reason for the huge page to be movable if
// it is not migratable to start with. Also the size of the huge
// page should be large enough to be placed under a movable zone
// and still feasible enough to be migratable. Just the presence
// in movable zone does not make the migration feasible.
//
// So even though large huge page sizes like the gigantic ones
// are migratable they should not be movable because its not
// feasible to migrate them from movable zone.
//
// Movability of hugepages depends on migration support.
// Some callers might want to enforce node
//
// Note: the memory offline, memory failure and migration syscalls will
// be allowed to fallback to other nodes due to lack of a better chioce,
// that might break the per-node hugetlb pool. While other cases will
// set the __GFP_THISNODE to avoid breaking the per-node hugetlb pool.
//
// hugetlb must use the exact same PT locks as core-mm page table
// walkers would. When modifying a PTE table, hugetlb must take the
// PTE PT lock, when modifying a PMD table, hugetlb must take the PMD
// PT lock etc.
//
// The expectation is that any hugetlb folio smaller than a PMD is
// always mapped into a single PTE table and that any hugetlb folio
// smaller than a PUD (but at least as big as a PMD) is always mapped
// into a single PMD table.
//
// If that does not hold for an architecture, then that architecture
// must disable split PT locks such that all *_lockptr() functions
// will give us the same result: the per-MM PT lock.
//
// Note that with e.g., CONFIG_PGTABLE_LEVELS=2 where
// PGDIR_SIZE==P4D_SIZE==PUD_SIZE==PMD_SIZE, we'd use pud_lockptr()
// and core-mm would use pmd_lockptr(). However, in such configurations
// split PMD locks are disabled -- they don't make sense on a single
// PGDIR page table -- and the end result is the same.
//
extern "C" {
    pub fn pud_lockptr(_arg: mm, pte: *mut *mut (pud_t )) -> return;
}
extern "C" {
    pub fn pmd_lockptr(_arg: mm, pte: *mut *mut (pmd_t )) -> return;
}
// pte_alloc_huge() only applies with !CONFIG_HIGHPTE
extern "C" {
    pub fn ptep_lockptr(_arg: mm, _arg: pte) -> return;
}

//
// Some platform decide whether they support huge pages at boot
// time. Some of them, such as powerpc, set HPAGE_SHIFT to 0
// when there is no such support
//

extern "C" {
    pub fn hugetlb_report_usage(m: *mut seq_file, mm: *mut mm_struct);
}

extern "C" {
    pub fn huge_ptep_get_and_clear(_arg: vma->vm_mm, _arg: addr, _arg: ptep, _arg: psize) -> return;
}

extern "C" {
    pub fn hugetlb_register_node(node: *mut node);
}
extern "C" {
    pub fn hugetlb_unregister_node(node: *mut node);
}

//
// Check if a given raw @page in a hugepage is HWPOISON.
//
extern "C" {
    pub fn is_raw_hwpoison_page_in_hugepage(page: *mut page) -> bool;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hstate {
    pub 0: return,
    pub NULL: return,
    pub NULL: return,
    pub -ENOMEM: return,
    pub 0: return,
    pub NULL: return,
    pub NULL: return,
    pub NULL: return,
    pub NULL: return,
    pub NULL: return,
    pub NULL: return,
    pub NULL: return,
    pub NULL: return,
    pub NULL: return,
    pub PAGE_SIZE: return,
    pub PAGE_MASK: return,
    pub 0: return,
    pub PAGE_SHIFT: return,
    pub false: return,
    pub 1: return,
    pub 0: return,
    pub 0: return,
    pub 0: return,
    pub 0: return,
    pub false: return,
    pub false: return,
    pub 0: return,
    pub 0: return,
    pub false: return,
    pub &mm->page_table_lock: return,
    pub ptep): *mut *mut pte_t huge_ptep_get(struct mm_struct mm, unsigned long addr, pte_t,
    pub pte): unsigned long huge_pte_dirty(pte_t,

    pub ptep_get(ptep): return,

    pub ptep: *mut return,

    pub false: return,

    pub ptl: *mut spinlock_t,
    pub pte): ptl = huge_pte_lockptr(h, mm,,
    pub ptl: return,

    pub hugetlb_cma_reserve(void): extern void __init,

    pub ptdesc_pmd_is_shared(virt_to_ptdesc(pte)): return,

    pub false: return,

    pub addr): *mut *mut bool want_pmd_share(struct vm_area_struct vma, unsigned long,
//
// ARCHes with special requirements for evicting HUGETLB backing TLB entries can
// implement this.
//

    pub vma->vm_private_data: return (vma->vm_flags & VM_MAYSHARE) &&,
    pub vma): *mut bool __vma_private_lock(struct vm_area_struct,
//
// Safe version of huge_pte_offset() to check the locks.  See comments
// above huge_pte_offset().
//

    pub vma->vm_private_data: *mut *mut hugetlb_vma_lock vma_lock =,
//
// If pmd sharing possible, locking needed to safely walk the
// hugetlb pgtables.  More information can be found at the comment
// above huge_pte_offset() in the same file.
//
// NOTE: lockdep_is_held() is only defined with CONFIG_LOCKDEP.
//

    pub sz): return huge_pte_offset(vma->vm_mm, addr,,
