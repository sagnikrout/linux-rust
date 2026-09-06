//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pagewalk.h
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

// Locking requirement during a page walk.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum page_walk_lock {
// mmap_lock should be locked for read to stabilize the vma tree
    PGWALK_RDLOCK = 0,
// vma will be write-locked during the walk
    PGWALK_WRLOCK = 1,
// vma is expected to be already write-locked during the walk
    PGWALK_WRLOCK_VERIFY = 2,
// vma is expected to be already read-locked during the walk
    PGWALK_VMA_RDLOCK_VERIFY = 3,
}

//
// struct mm_walk_ops - callbacks for walk_page_range
// @pgd_entry:		if set, called for each non-empty PGD (top-level) entry
// @p4d_entry:		if set, called for each non-empty P4D entry
// @pud_entry:		if set, called for each non-empty PUD entry
// @pmd_entry:		if set, called for each non-empty PMD entry
// this handler is required to be able to handle
// pmd_trans_huge() pmds.  They may simply choose to
// split_huge_page() instead of handling it explicitly.
// @pte_entry:		if set, called for each PTE (lowest-level) entry
// including empty ones, except if @install_pte is set.
// If @install_pte is set, @pte_entry is called only for
// existing PTEs.
// @pte_hole:		if set, called for each hole at all levels,
// depth is -1 if not known, 0:PGD, 1:P4D, 2:PUD, 3:PMD.
// Any folded depths (where PTRS_PER_P?D is equal to 1)
// are skipped. If @install_pte is specified, this will
// not trigger for any populated ranges.
// @hugetlb_entry:	if set, called for each hugetlb entry. This hook
// function is called with the vma lock held, in order to
// protect against a concurrent freeing of the pte_t* or
// the ptl. In some cases, the hook function needs to drop
// and retake the vma lock in order to avoid deadlocks
// while calling other functions. In such cases the hook
// function must either refrain from accessing the pte or
// ptl after dropping the vma lock, or else revalidate
// those items after re-acquiring the vma lock and before
// accessing them.
// @test_walk:		caller specific callback function to determine whether
// we walk over the current vma or not. Returning 0 means
// "do page table walk over the current vma", returning
// a negative value means "abort current page table walk
// right now" and returning 1 means "skip the current vma"
// Note that this callback is not called when the caller
// passes in a single VMA as for walk_page_vma().
// @pre_vma:            if set, called before starting walk on a non-null vma.
// @post_vma:           if set, called after a walk on a non-null vma, provided
// that @pre_vma and the vma walk succeeded.
// @install_pte:        if set, missing page table entries are installed and
// thus all levels are always walked in the specified
// range. This callback is then invoked at the PTE level
// (having split any THP pages prior), providing the PTE to
// install. If allocations fail, the walk is aborted. This
// operation is only available for userland memory. Not
// usable for hugetlb ranges.
//
// p?d_entry callbacks are called even if those levels are folded on a
// particular architecture/configuration.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mm_walk_ops {
    pub walk): *mut unsigned long next, struct mm_walk,
    pub walk): *mut unsigned long next, struct mm_walk,
    pub walk): *mut unsigned long next, struct mm_walk,
    pub walk): *mut unsigned long next, struct mm_walk,
    pub walk): *mut unsigned long next, struct mm_walk,
    pub walk): *mut int depth, struct mm_walk,
    pub walk): *mut mm_walk,
    pub walk): *mut mm_walk,
    pub walk): *mut mm_walk,
    pub walk): *mut *mut void (post_vma)(struct mm_walk,
    pub walk): *mut *mut pte_t ptep, struct mm_walk,
    pub walk_lock: page_walk_lock,
}

//
// Action for pud_entry / pmd_entry callbacks.
// ACTION_SUBTREE is the default
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum page_walk_action {
// Descend to next level, splitting huge pages if needed and possible
    ACTION_SUBTREE = 0,
// Continue to next entry at this level (ignoring any subtree)
    ACTION_CONTINUE = 1,
// Call again for this entry
    ACTION_AGAIN = 2
}

//
// struct mm_walk - walk_page_range data
// @ops:	operation to call during the walk
// @mm:		mm_struct representing the target process of page table walk
// @pgd:	pointer to PGD; only valid with no_vma (otherwise set to NULL)
// @vma:	vma currently walked (NULL if walking outside vmas)
// @action:	next action to perform (see enum page_walk_action)
// @no_vma:	walk ignoring vmas (vma will always be NULL)
// @private:	private data for callbacks' usage
//
// (see the comment on walk_page_range() for more details)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mm_walk {
    pub ops: *const mm_walk_ops,
    pub mm: *mut mm_struct,
    pub pgd: *mut pgd_t,
    pub vma: *mut vm_area_struct,
    pub action: page_walk_action,
    pub no_vma: bool,
    pub private: *mut c_void,
}

pub type folio_walk_flags_t = int ;
// Walk shared zeropages (small + huge) as well.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum folio_walk_level {
    FW_LEVEL_PTE,
    FW_LEVEL_PMD,
    FW_LEVEL_PUD,
}

//
// struct folio_walk - folio_walk_start() / folio_walk_end() data
// @page:	exact folio page referenced (if applicable)
// @level:	page table level identifying the entry type
// @pte:	pointer to the page table entry (FW_LEVEL_PTE).
// @pmd:	pointer to the page table entry (FW_LEVEL_PMD).
// @pud:	pointer to the page table entry (FW_LEVEL_PUD).
// @ptl:	pointer to the page table lock.
//
// (see folio_walk_start() documentation for more details)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct folio_walk {
// public
    pub page: *mut page,
    pub level: folio_walk_level,
    pub ptep: *mut pte_t,
    pub pudp: *mut pud_t,
    pub pmdp: *mut pmd_t,
}

// private

