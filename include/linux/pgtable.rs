//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pgtable.h
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
// This defines the generic helper for accessing PMD page
// table page. Although platforms can still override this
// via their respective <asm/pgtable.h>.
//

//
// A page table page can be thought of an array like this: pXd_t[PTRS_PER_PxD]
//
// The pXx_index() functions return the index of the entry in the page
// table page which would control the given virtual address
//
// As these functions may be used by the same code for different levels of
// the page table folding, they are always available, regardless of
// CONFIG_PGTABLE_LEVELS value. For the folded levels they simply return 0
// because in such cases PTRS_PER_PxD equals 1.
//

// Must be a compile-time constant, so implement it as a macro

extern "C" {
    pub fn pte_offset_kernel(_arg: pmd, _arg: address) -> return;
}

extern "C" {
    pub fn pte_free_defer(mm: *mut mm_struct, pgtable: pgtable_t);
}
// Find an entry in the second-level page table..

extern "C" {
    pub fn pud_pgtable(pmd_index(address: *mut *mut pud) +) -> return;
}

extern "C" {
    pub fn p4d_pgtable(pud_index(address: *mut *mut p4d) +) -> return;
}

//
// a shortcut to get a pgd_t in a given mm
//

//
// a shortcut which implies the use of the kernel's pgd, instead
// of a process's
//

//
// In many cases it is known that a virtual address is mapped at PMD or PTE
// level, so instead of traversing all the page table levels, we can get a
// pointer to the PMD entry in user or kernel page table or translate a virtual
// address to the pointer in the PTE in the kernel page tables with simple
// helpers.
//
extern "C" {
    pub fn pmd_offset(_arg: pud_offset(p4d_offset(pgd_offset(mm, _arg: va), _arg: va), _arg: va), _arg: va) -> return;
}
extern "C" {
    pub fn pmd_offset(_arg: pud_offset(p4d_offset(pgd_offset_k(va), _arg: va), _arg: va), _arg: va) -> return;
}
extern "C" {
    pub fn pmd_none(pte_offset_kernel(pmd: *mut *mut pmd) ? NULL :, _arg: vaddr) -> return;
}

//
// A facility to provide lazy MMU batching.  This allows PTE updates and
// page invalidations to be delayed until a call to leave lazy MMU mode
// is issued.  Some architectures may benefit from doing this, and it is
// beneficial for both shadow and direct mode hypervisors, which may batch
// the PTE updates which happen during this window.  Note that using this
// interface requires that read hazards be removed from the code.  A read
// hazard could result in the direct mode hypervisor case, since the actual
// write to the page tables may not yet have taken place, so reads though
// a raw PTE pointer after it has been modified are not guaranteed to be
// up to date.
//
// In the general case, no lock is guaranteed to be held between entry and exit
// of the lazy mode. (In practice, for user PTE updates, the appropriate page
// table lock(s) are held, but for kernel PTE updates, no lock is held).
// The implementation must therefore assume preemption may be enabled upon
// entry to the mode and cpu migration is possible; it must take steps to be
// robust against this. An implementation may handle this by disabling
// preemption, as a consequence generic code may not sleep while the lazy MMU
// mode is active.
//
// The mode is disabled in interrupt context and calls to the lazy_mmu API have
// no effect.
//
// The lazy MMU mode is enabled for a given block of code using:
//
// lazy_mmu_mode_enable();
// <code>
// lazy_mmu_mode_disable();
//
// Nesting is permitted: <code> may itself use an enable()/disable() pair.
// A nested call to enable() has no functional effect; however disable() causes
// any batched architectural state to be flushed regardless of nesting. After a
// call to disable(), the caller can therefore rely on all previous page table
// modifications to have taken effect, but the lazy MMU mode may still be
// enabled.
//
// In certain cases, it may be desirable to temporarily pause the lazy MMU mode.
// This can be done using:
//
// lazy_mmu_mode_pause();
// <code>
// lazy_mmu_mode_resume();
//
// pause() ensures that the mode is exited regardless of the nesting level;
// resume() re-enters the mode at the same nesting level. Any call to the
// lazy_mmu_mode_* API between those two calls has no effect. In particular,
// this means that pause()/resume() pairs may nest.
//
// is_lazy_mmu_mode_active() can be used to check whether the lazy MMU mode is
// currently enabled.
//

//
// lazy_mmu_mode_enable() - Enable the lazy MMU mode.
//
// Enters a new lazy MMU mode section; if the mode was not already enabled,
// enables it and calls arch_enter_lazy_mmu_mode().
//
// Must be paired with a call to lazy_mmu_mode_disable().
//
// Has no effect if called:
// - While paused - see lazy_mmu_mode_pause()
// - In interrupt context
//
// lazy_mmu_mode_disable() - Disable the lazy MMU mode.
//
// Exits the current lazy MMU mode section. If it is the outermost section,
// disables the mode and calls arch_leave_lazy_mmu_mode(). Otherwise (nested
// section), calls arch_flush_lazy_mmu_mode().
//
// Must match a call to lazy_mmu_mode_enable().
//
// Has no effect if called:
// - While paused - see lazy_mmu_mode_pause()
// - In interrupt context
//
// __task_lazy_mmu_mode_pause() - Pause the lazy MMU mode for a task.
// @tsk: The task to check.
//
// Pauses the lazy MMU mode of @tsk.
//
// This function only operates on the state saved in task_struct; to pause
// current lazy_mmu_mode_pause() should be used instead.
//
// This function is intended for architectures that implement the lazy MMU
// mode; it must not be called from generic code.
//
// lazy_mmu_mode_pause() - Pause the lazy MMU mode.
//
// Pauses the lazy MMU mode; if it is currently active, disables it and calls
// arch_leave_lazy_mmu_mode().
//
// Must be paired with a call to lazy_mmu_mode_resume(). Calls to the
// lazy_mmu_mode_* API have no effect until the matching resume() call.
//
// Has no effect if called:
// - While paused (inside another pause()/resume() pair)
// - In interrupt context
//
// __task_lazy_mmu_mode_resume() - Resume the lazy MMU mode for a task.
// @tsk: The task to check.
//
// Resumes the lazy MMU mode of @tsk.
//
// This function only operates on the state saved in task_struct; to resume
// current lazy_mmu_mode_resume() should be used instead.
//
// This function is intended for architectures that implement the lazy MMU
// mode; it must not be called from generic code.
//
// lazy_mmu_mode_resume() - Resume the lazy MMU mode.
//
// Resumes the lazy MMU mode; if it was active at the point where the matching
// call to lazy_mmu_mode_pause() was made, re-enables it and calls
// arch_enter_lazy_mmu_mode().
//
// Must match a call to lazy_mmu_mode_pause().
//
// Has no effect if called:
// - While paused (inside another pause()/resume() pair)
// - In interrupt context
//

//
// pte_batch_hint - Number of pages that can be added to batch without scanning.
// @ptep: Page table pointer for the entry.
// @pte: Page table entry.
//
// Some architectures know that a set of contiguous ptes all map the same
// contiguous memory with the same permissions. In this case, it can provide a
// hint to aid pte batching without the core code needing to scan every pte.
//
// An architecture implementation may ignore the PTE accessed state. Further,
// the dirty state must apply atomically to all the PTEs described by the hint.
//
// May be overridden by the architecture, else pte_batch_hint is always 1.
//

extern "C" {
    pub fn __pte(PFN_PTE_SHIFT): pte_val(pte) + (nr <<) -> return;
}

//
// set_ptes - Map consecutive pages to a contiguous range of addresses.
// @mm: Address space to map the pages into.
// @addr: Address to map the first page at.
// @ptep: Page table pointer for the first entry.
// @pte: Page table entry for the first page.
// @nr: Number of pages to map.
//
// When nr==1, initial state of pte may be present or not present, and new state
// may be present or not present. When nr>1, initial state of all ptes must be
// not present, and new state must be present.
//
// May be overridden by the architecture, or the architecture can define
// set_pte() and PFN_PTE_SHIFT.
//
// Context: The caller holds the page table lock.  The pages all belong
// to the same folio.  The PTEs are all in the same PMD.
//

extern "C" {
    pub fn READ_ONCE(_arg: *mut ptep) -> return;
}

extern "C" {
    pub fn READ_ONCE(_arg: *mut pmdp) -> return;
}

extern "C" {
    pub fn READ_ONCE(_arg: *mut pudp) -> return;
}

extern "C" {
    pub fn READ_ONCE(_arg: *mut p4dp) -> return;
}

extern "C" {
    pub fn READ_ONCE(_arg: *mut pgdp) -> return;
}

//
// Despite relevant to THP only, this API is called from generic rmap code
// under PageTransHuge(), hence needs a dummy implementation for !THP
//

//
// Return whether the accessed bit in non-leaf PMD entries is supported on the
// local CPU.
//
extern "C" {
    pub fn IS_ENABLED(_arg: CONFIG_ARCH_HAS_NONLEAF_PMD_YOUNG) -> return;
}

//
// Return whether the accessed bit is supported on the local CPU.
//
// This stub assumes accessing through an old PTE triggers a page fault.
// Architectures that automatically set the access bit should overwrite it.
//
extern "C" {
    pub fn IS_ENABLED(_arg: CONFIG_ARCH_HAS_HW_PTE_YOUNG) -> return;
}

//
// Returns preferred minimum folio order for executable file-backed memory. Must
// be in range [0, PMD_ORDER). Default to order-0.
//

//
// clear_young_dirty_ptes - Mark PTEs that map consecutive pages of the
// same folio as old/clean.
// @mm: Address space the pages are mapped into.
// @addr: Address the first page is mapped at.
// @ptep: Page table pointer for the first entry.
// @nr: Number of entries to mark old/clean.
// @flags: Flags to modify the PTE batch semantics.
//
// May be overridden by the architecture; otherwise, implemented by
// get_and_clear/modify/set for each pte in the range.
//
// Note that PTE bits in the PTE range besides the PFN can differ. For example,
// some PTEs might be write-protected.
//
// Context: The caller holds the page table lock.  The PTEs map consecutive
// pages that belong to the same folio.  The PTEs are all in the same PMD.
//

//
// No need for ptep_get_and_clear(): page table check doesn't care about
// any bits that could have been set by HW concurrently.
//

//
// For walking the pagetables without holding any locks.  Some architectures
// (eg x86-32 PAE) cannot load the entries atomically without using expensive
// instructions.  We are guaranteed that a PTE will only either go from not
// present to present, or present to not present -- it will not switch to a
// completely different present page without a TLB flush inbetween; which we
// are blocking by holding interrupts off.
//
// Setting ptes from not present to present goes:
//
// ptep->pte_high = h;
// smp_wmb();
// ptep->pte_low = l;
//
// And present to not present goes:
//
// ptep->pte_low = 0;
// smp_wmb();
// ptep->pte_high = 0;
//
// We must ensure here that the load of pte_low sees 'l' IFF pte_high sees 'h'.
// We load pte_high *after* loading pte_low, which ensures we don't see an older
// value of pte_high.  *Then* we recheck pte_low, which ensures that we haven't
// picked up a changed pte high. We might have gotten rubbish values from
// pte_low and pte_high, but we are guaranteed that pte_low will not have the
// present bit set *unless* it is 'l'. Because get_user_pages_fast() only
// operates on present ptes we're safe.
//

//
// We require that the PTE can be read atomically.
//

extern "C" {
    pub fn ptep_get(_arg: ptep) -> return;
}

extern "C" {
    pub fn pmdp_get(_arg: pmdp) -> return;
}

extern "C" {
    pub fn pmdp_huge_get_and_clear(_arg: vma->vm_mm, _arg: address, _arg: pmdp) -> return;
}

extern "C" {
    pub fn pudp_huge_get_and_clear(_arg: vma->vm_mm, _arg: address, _arg: pudp) -> return;
}

extern "C" {
    pub fn ptep_get_and_clear(_arg: mm, _arg: address, _arg: ptep) -> return;
}

//
// get_and_clear_full_ptes - Clear present PTEs that map consecutive pages of
// the same folio, collecting dirty/accessed bits.
// @mm: Address space the pages are mapped into.
// @addr: Address the first page is mapped at.
// @ptep: Page table pointer for the first entry.
// @nr: Number of entries to clear.
// @full: Whether we are clearing a full mm.
//
// May be overridden by the architecture; otherwise, implemented as a simple
// loop over ptep_get_and_clear_full(), merging dirty/accessed bits into the
// returned PTE.
//
// Note that PTE bits in the PTE range besides the PFN can differ. For example,
// some PTEs might be write-protected.
//
// Context: The caller holds the page table lock.  The PTEs map consecutive
// pages that belong to the same folio.  The PTEs are all in the same PMD.
//

//
// get_and_clear_ptes - Clear present PTEs that map consecutive pages of
// the same folio, collecting dirty/accessed bits.
// @mm: Address space the pages are mapped into.
// @addr: Address the first page is mapped at.
// @ptep: Page table pointer for the first entry.
// @nr: Number of entries to clear.
//
// Use this instead of get_and_clear_full_ptes() if it is known that we don't
// need to clear the full mm, which is mostly the case.
//
// Note that PTE bits in the PTE range besides the PFN can differ. For example,
// some PTEs might be write-protected.
//
// Context: The caller holds the page table lock.  The PTEs map consecutive
// pages that belong to the same folio.  The PTEs are all in the same PMD.
//
extern "C" {
    pub fn get_and_clear_full_ptes(_arg: mm, _arg: addr, _arg: ptep, _arg: nr, _arg: 0) -> return;
}

//
// clear_full_ptes - Clear present PTEs that map consecutive pages of the same
// folio.
// @mm: Address space the pages are mapped into.
// @addr: Address the first page is mapped at.
// @ptep: Page table pointer for the first entry.
// @nr: Number of entries to clear.
// @full: Whether we are clearing a full mm.
//
// May be overridden by the architecture; otherwise, implemented as a simple
// loop over ptep_get_and_clear_full().
//
// Note that PTE bits in the PTE range besides the PFN can differ. For example,
// some PTEs might be write-protected.
//
// Context: The caller holds the page table lock.  The PTEs map consecutive
// pages that belong to the same folio.  The PTEs are all in the same PMD.
//

//
// clear_ptes - Clear present PTEs that map consecutive pages of the same folio.
// @mm: Address space the pages are mapped into.
// @addr: Address the first page is mapped at.
// @ptep: Page table pointer for the first entry.
// @nr: Number of entries to clear.
//
// Use this instead of clear_full_ptes() if it is known that we don't need to
// clear the full mm, which is mostly the case.
//
// Note that PTE bits in the PTE range besides the PFN can differ. For example,
// some PTEs might be write-protected.
//
// Context: The caller holds the page table lock.  The PTEs map consecutive
// pages that belong to the same folio.  The PTEs are all in the same PMD.
//
// If two threads concurrently fault at the same page, the thread that
// won the race updates the PTE and its local TLB/Cache. The other thread
// gives up, simply does nothing, and continues; on architectures where
// software can update TLB,  local TLB can be updated here to avoid next page
// fault. This function updates TLB only, do nothing with cache or others.
// It is the difference with function update_mmu_cache.
//

//
// clear_nonpresent_ptes - Clear multiple non-present PTEs which are
// consecutive in the pgtable.
// @mm: Address space the ptes represent.
// @addr: Address of the first pte.
// @ptep: Page table pointer for the first entry.
// @nr: Number of entries to clear.
//
// Context: The caller holds the page table lock.  The PTEs are all not present.
// The PTEs are all in the same PMD.
//

extern "C" {
    pub fn pte_mkwrite_novma(_arg: pte) -> return;
}

extern "C" {
    pub fn pmd_mkwrite_novma(_arg: pmd) -> return;
}

//
// ptep_try_set - atomically set an empty kernel PTE
// @ptep: page table entry
// @new_pte: value to install
//
// Atomically set *@ptep to @new_pte iff *@ptep is pte_none(). Return true on
// success, false if the slot was already populated or the arch has no
// implementation.
//
// For special kernel page tables only - never user page tables. The caller must
// prevent concurrent teardown of @ptep and must accept that other writers may
// race. Concurrent clearers must use ptep_get_and_clear() so racing accesses
// agree on the outcome.
//
// Architectures opt in by providing a cmpxchg-based override and defining
// ptep_try_set as an identity macro. The generic stub returns false, which is
// correct for callers that fall through to oops on failure.
//

//
// flush_tlb_before_set - invalidate a kernel PTE's TLB before re-setting it
// @addr: kernel virtual address whose PTE was just cleared
//
// Some architectures (e.g. arm64) do not allow a live page-table entry to be
// repointed at a different page in one step. The old entry must first be made
// invalid and its translation flushed from every TLB, and only then may the new
// entry be written.
//
// This is only for the lockless atomic kernel-PTE installers (ptep_try_set()).
// It must be callable with interrupts disabled.
//

//
// wrprotect_ptes - Write-protect PTEs that map consecutive pages of the same
// folio.
// @mm: Address space the pages are mapped into.
// @addr: Address the first page is mapped at.
// @ptep: Page table pointer for the first entry.
// @nr: Number of entries to write-protect.
//
// May be overridden by the architecture; otherwise, implemented as a simple
// loop over ptep_set_wrprotect().
//
// Note that PTE bits in the PTE range besides the PFN can differ. For example,
// some PTEs might be write-protected.
//
// Context: The caller holds the page table lock.  The PTEs map consecutive
// pages that belong to the same folio.  The PTEs are all in the same PMD.
//

//
// clear_flush_young_ptes - Mark PTEs that map consecutive pages of the same
// folio as old and flush the TLB.
// @vma: The virtual memory area the pages are mapped into.
// @addr: Address the first page is mapped at.
// @ptep: Page table pointer for the first entry.
// @nr: Number of entries to clear access bit.
//
// May be overridden by the architecture; otherwise, implemented as a simple
// loop over ptep_clear_flush_young().
//
// Note that PTE bits in the PTE range besides the PFN can differ. For example,
// some PTEs might be write-protected.
//
// Context: The caller holds the page table lock.  The PTEs map consecutive
// pages that belong to the same folio.  The PTEs are all in the same PMD.
//

//
// test_and_clear_young_ptes - Mark PTEs that map consecutive pages of the same
// folio as old
// @vma: The virtual memory area the pages are mapped into.
// @addr: Address the first page is mapped at.
// @ptep: Page table pointer for the first entry.
// @nr: Number of entries to clear access bit.
//
// May be overridden by the architecture; otherwise, implemented as a simple
// loop over ptep_test_and_clear_young().
//
// Note that PTE bits in the PTE range besides the PFN can differ. For example,
// some PTEs might be write-protected.
//
// Context: The caller holds the page table lock.  The PTEs map consecutive
// pages that belong to the same folio.  The PTEs are all in the same PMD.
//
// Returns: whether any PTE was young.
//

//
// On some architectures hardware does not set page access bit when accessing
// memory page, it is responsibility of software setting this bit. It brings
// out extra page fault penalty to track page access bit. For optimization page
// access bit can be set during all page fault flow on these arches.
// To be differentiate with macro pte_mkyoung, this macro is used on platforms
// where software maintains page access bit.
//

extern "C" {
    pub fn pgtable_trans_huge_withdraw(mm: *mut mm_struct, pmdp: *mut pmd_t) -> pgtable_t;
}

//
// This is an implementation of pmdp_establish() that is only suitable for an
// architecture that doesn't have hardware dirty/accessed bits. In this case we
// can't race with CPU which sets these bits and non-atomic approach is fine.
//

//
// pmdp_invalidate_ad() invalidates the PMD while changing a transparent
// hugepage mapping in the page tables. This function is similar to
// pmdp_invalidate(), but should only be used if the access and dirty bits would
// not be cleared by the software in the new PMD value. The function ensures
// that hardware changes of the access and dirty bits updates would not be lost.
//
// Doing so can allow in certain architectures to avoid a TLB flush in most
// cases. Yet, another TLB flush might be necessary later if the PMD update
// itself requires such flush (e.g., if protection was set to be stricter). Yet,
// even when a TLB flush is needed because of the update, the caller may be able
// to batch these TLB flushing operations, so fewer TLB flush operations are
// needed.
//

extern "C" {
    pub fn pte_val(pte_val(pte_b: pte_a) ==) -> return;
}

//
// Some architectures provide facilities to virtualization guests
// so that they can flag allocated pages as unused. This allows the
// host to transparently reclaim unused pages. This function returns
// whether the pte's page is unused.
//

extern "C" {
    pub fn pmd_val(pmd_val(pmd_b: pmd_a) ==) -> return;
}

extern "C" {
    pub fn pud_val(pud_val(pud_b: pud_a) ==) -> return;
}

extern "C" {
    pub fn p4d_val(p4d_val(p4d_b: p4d_a) ==) -> return;
}

extern "C" {
    pub fn pgd_val(pgd_val(pgd_b: pgd_a) ==) -> return;
}

//
// Some architectures support metadata associated with a page. When a
// page is being swapped out, this metadata must be saved so it can be
// restored when the page is swapped back in. SPARC M7 and newer
// processors support an ADI (Application Data Integrity) tag for the
// page as metadata for the page. arch_do_swap_page() can restore this
// metadata when a page is swapped back in.
//

//
// Some architectures support metadata associated with a page. When a
// page is being swapped out, this metadata must be saved so it can be
// restored when the page is swapped back in. SPARC M7 and newer
// processors support an ADI (Application Data Integrity) tag for the
// page as metadata for the page. arch_unmap_one() can save this
// metadata on a swap-out of a page.
//

//
// Allow architectures to preserve additional metadata associated with
// swapped-out pages. The corresponding __HAVE_ARCH_SWAP_* macros and function
// prototypes must be defined in the arch-specific asm/pgtable.h file.
//

//
// When walking page tables, get the address of the next boundary,
// or the end address of the range if that comes earlier.  Although no
// vma end wraps to 0, rounded up __boundary may wrap to 0 throughout.
//

//
// When walking page tables, we usually want to skip any p?d_none entries;
// and any p?d_bad entries - reporting the error before resetting to none.
// Do the tests inline, but report and clear the bad entry in mm/memory.c.
//
extern "C" {
    pub fn pgd_clear_bad(: *mut pgd_t);
}
extern "C" {
    pub fn p4d_clear_bad(: *mut p4d_t);
}

extern "C" {
    pub fn pud_clear_bad(: *mut pud_t);
}

extern "C" {
    pub fn pmd_clear_bad(: *mut pmd_t);
}
//
// Get the current pte state, but zero it out to make it
// non-present, preventing the hardware from asynchronously
// updating it.
//
extern "C" {
    pub fn ptep_get_and_clear(_arg: vma->vm_mm, _arg: addr, _arg: ptep) -> return;
}
//
// The pte is non-present, so there's no hardware state to
// preserve.
//
// Start a pte protection read-modify-write transaction, which
// protects against asynchronous hardware modifications to the pte.
// The intention is not to prevent the hardware from making pte
// updates, but to prevent any updates it may make from being lost.
//
// This does not protect against other software modifications of the
// pte; the appropriate pte lock must be held over the transaction.
//
// Note that this interface is intended to be batchable, meaning that
// ptep_modify_prot_commit may not actually update the pte, but merely
// queue the update to be done at some later time.  The update must be
// actually committed before the pte lock is released, however.
//
extern "C" {
    pub fn __ptep_modify_prot_start(_arg: vma, _arg: addr, _arg: ptep) -> return;
}
//
// Commit an update to a pte, leaving any hardware-controlled bits in
// the PTE unmodified. The pte returned from ptep_modify_prot_start() may
// additionally have young and/or dirty bits set where previously they were not,
// so the updated pte may have these additional changes.
//

//
// modify_prot_start_ptes - Start a pte protection read-modify-write transaction
// over a batch of ptes, which protects against asynchronous hardware
// modifications to the ptes. The intention is not to prevent the hardware from
// making pte updates, but to prevent any updates it may make from being lost.
// Please see the comment above ptep_modify_prot_start() for full description.
//
// @vma: The virtual memory area the pages are mapped into.
// @addr: Address the first page is mapped at.
// @ptep: Page table pointer for the first entry.
// @nr: Number of entries.
//
// May be overridden by the architecture; otherwise, implemented as a simple
// loop over ptep_modify_prot_start(), collecting the a/d bits from each pte
// in the batch.
//
// Note that PTE bits in the PTE batch besides the PFN can differ.
//
// Context: The caller holds the page table lock.  The PTEs map consecutive
// pages that belong to the same folio. All other PTE bits must be identical for
// all PTEs in the batch except for young and dirty bits.  The PTEs are all in
// the same PMD.
//

//
// modify_prot_commit_ptes - Commit an update to a batch of ptes, leaving any
// hardware-controlled bits in the PTE unmodified.
//
// @vma: The virtual memory area the pages are mapped into.
// @addr: Address the first page is mapped at.
// @ptep: Page table pointer for the first entry.
// @old_pte: Old page table entry (for the first entry) which is now cleared.
// @pte: New page table entry to be set.
// @nr: Number of entries.
//
// May be overridden by the architecture; otherwise, implemented as a simple
// loop over ptep_modify_prot_commit().
//
// Context: The caller holds the page table lock. The PTEs are all in the same
// PMD. On exit, the set ptes in the batch map the same folio. The ptes set by
// ptep_modify_prot_start() may additionally have young and/or dirty bits set
// where previously they were not, so the updated ptes may have these
// additional changes.
//

// Advance PFN only, set same prot

//
// Architectures can set this mask to a combination of PGTBL_P?D_MODIFIED values
// and let generic vmalloc, ioremap and page table update code know when
// arch_sync_kernel_mappings() needs to be called.
//

pub const ARCH_PAGE_TABLE_SYNC_MASK: c_int = 0;

//
// There is no default implementation for arch_sync_kernel_mappings(). It is
// relied upon the compiler to optimize calls out if ARCH_PAGE_TABLE_SYNC_MASK
// is 0.
//
extern "C" {
    pub fn arch_sync_kernel_mappings(start: c_ulong, end: c_ulong);
}

//
// On almost all architectures and configurations, 0 can be used as the
// upper ceiling to free_pgtables(): on many architectures it has the same
// effect as using TASK_SIZE.  However, there is one configuration which
// must impose a more careful limit, to avoid freeing kernel pgtables.
//

//
// This defines the first usable user address. Platforms
// can override its value with custom FIRST_USER_ADDRESS
// defined in their respective <asm/pgtable.h>.
//

//
// No-op macros that just return the current protection value. Defined here
// because these macros can be used even if CONFIG_MMU is not defined.
//

//
// A facility to provide batching of the reload of page tables and
// other process state with the actual context switch code for
// paravirtualized guests.  By convention, only one of the batched
// update (lazy) modes (CPU, MMU) should be active at any given time,
// entry should never be nested, and entry and exits should always be
// paired.  This is for sanity of maintaining and reasoning about the
// kernel code.  In this case, the exit (end of the context switch) is
// in architecture-specific code, and so doesn't need a generic
// definition.
//

//
// Some platforms can customize the PTE soft-dirty bit making it unavailable
// even if the architecture provides the resource.
// Adding this API allows architectures to add their own checks for the
// devices on which the kernel is running.
// Note: When overriding it, please make sure the CONFIG_MEM_SOFT_DIRTY
// is part of this macro.
//

//
// Interfaces that can be used by architecture code to keep track of
// memory type of pfn mappings specified by the remap_pfn_range,
// vmf_insert_pfn.
//

//
// pfnmap_setup_cachemode - setup the cachemode in the pgprot for a pfn range
// @pfn: the start of the pfn range
// @size: the size of the pfn range in bytes
// @prot: the pgprot to modify
//
// Lookup the cachemode for the pfn range starting at @pfn with the size
// @size and store it in @prot, leaving other data in @prot unchanged.
//
// This allows for a hardware implementation to have fine-grained control of
// memory cache behavior at page level granularity. Without a hardware
// implementation, this function does nothing.
//
// Currently there is only one implementation for this - x86 Page Attribute
// Table (PAT). See Documentation/arch/x86/pat.rst for more details.
//
// This function can fail if the pfn range spans pfns that require differing
// cachemodes. If the pfn range was previously verified to have a single
// cachemode, it is sufficient to query only a single pfn. The assumption is
// that this is the case for drivers using the vmf_insert_pfn*() interface.
//
// Returns 0 on success and -EINVAL on error.
//
// pfnmap_track - track a pfn range
// @pfn: the start of the pfn range
// @size: the size of the pfn range in bytes
// @prot: the pgprot to track
//
// Requested the pfn range to be 'tracked' by a hardware implementation and
// setup the cachemode in @prot similar to pfnmap_setup_cachemode().
//
// This allows for fine-grained control of memory cache behaviour at page
// level granularity. Tracking memory this way is persisted across VMA splits
// (VMA merging does not apply for VM_PFNMAP).
//
// Currently, there is only one implementation for this - x86 Page Attribute
// Table (PAT). See Documentation/arch/x86/pat.rst for more details.
//
// Returns 0 on success and -EINVAL on error.
//
extern "C" {
    pub fn pfnmap_track(pfn: c_ulong, size: c_ulong, prot: *mut pgprot_t) -> c_int;
}
//
// pfnmap_untrack - untrack a pfn range
// @pfn: the start of the pfn range
// @size: the size of the pfn range in bytes
//
// Untrack a pfn range previously tracked through pfnmap_track().
//
extern "C" {
    pub fn pfnmap_untrack(pfn: c_ulong, size: c_ulong);
}

//
// pfnmap_setup_cachemode_pfn - setup the cachemode in the pgprot for a pfn
// @pfn: the pfn
// @prot: the pgprot to modify
//
// Lookup the cachemode for @pfn and store it in @prot, leaving other
// data in @prot unchanged.
//
// See pfnmap_setup_cachemode() for details.
//
// ZERO_PAGE() is global shared page(s) that is always zero. It is used for
// zero-mapped memory areas, CoW etc.
//
// On architectures that __HAVE_COLOR_ZERO_PAGE there are several such pages
// for different ranges in the virtual address space.
//
// zero_page_pfn identifies the first (or the only) pfn for these pages.
//
// For architectures that don't __HAVE_COLOR_ZERO_PAGE the zero page lives in
// empty_zero_page in BSS.
//
extern "C" {
    pub fn arch_setup_zero_pages();
}

//
// In an inaccessible (PROT_NONE) VMA, pte_protnone() may indicate "yes". It
// is perfectly valid to indicate "no" in that case, which is why our
// default implementation defaults to "always no".
//
// In an accessible VMA, pte_protnone() reliably indicates a present
// PROT_NONE page protection. Today the kernel uses such PTEs for two
// purposes: NUMA hinting faults, and userfaultfd RWP tracking on
// VM_UFFD_RWP VMAs. The two are distinguished by the uffd PTE bit and
// the VMA flag; see include/linux/userfaultfd_k.h.
//
// So, to reliably identify PROT_NONE PTEs that require kernel handling,
// looking at the VMA accessibility (and the uffd bit on RWP VMAs) is
// sufficient.
//
// Architectures without CONFIG_ARCH_HAS_PTE_PROTNONE get the always-zero
// stubs below; PAGE_NONE references that survive to runtime fire the
// BUILD_BUG() fallback, since callers should have folded such paths to
// dead code via IS_ENABLED(CONFIG_ARCH_HAS_PTE_PROTNONE).
//

extern "C" {
    pub fn p4d_set_huge(p4d: *mut p4d_t, addr: phys_addr_t, prot: pgprot_t) -> c_int;
}
extern "C" {
    pub fn p4d_clear_huge(p4d: *mut p4d_t);
}

extern "C" {
    pub fn pud_set_huge(pud: *mut pud_t, addr: phys_addr_t, prot: pgprot_t) -> c_int;
}
extern "C" {
    pub fn pmd_set_huge(pmd: *mut pmd_t, addr: phys_addr_t, prot: pgprot_t) -> c_int;
}
extern "C" {
    pub fn pud_clear_huge(pud: *mut pud_t) -> c_int;
}
extern "C" {
    pub fn pmd_clear_huge(pmd: *mut pmd_t) -> c_int;
}
extern "C" {
    pub fn p4d_free_pud_page(p4d: *mut p4d_t, addr: c_ulong) -> c_int;
}
extern "C" {
    pub fn pud_free_pmd_page(pud: *mut pud_t, addr: c_ulong) -> c_int;
}
extern "C" {
    pub fn pmd_free_pte_page(pmd: *mut pmd_t, addr: c_ulong) -> c_int;
}

//
// ARCHes with special requirements for evicting THP backing TLB entries can
// implement this. Otherwise also, it can help optimize normal TLB flush in
// THP regime. Stock flush_tlb_range() typically has optimization to nuke the
// entire TLB if flush span is greater than a threshold, which will
// likely be true for a single huge page. Thus a single THP flush will
// invalidate the entire TLB which is not desirable.
// e.g. see arch/arc: flush_pmd_tlb_range
//

extern "C" {
    pub fn pgtable_cache_init() -> void __init;
}

//
// Architecture PAGE_KERNEL_* fallbacks
//
// Some architectures don't define certain PAGE_KERNEL_* flags. This is either
// because they really don't support them, or the port needs to be updated to
// reflect the required functionality. Below are a set of relatively safe
// fallbacks, as best effort, which we can count on in lieu of the architectures
// not defining them on their own yet.
//

//
// Page Table Modification bits for pgtbl_mod_mask.
//
// These are used by the p?d_alloc_track*() and p*d_populate_kernel()
// functions in the generic vmalloc, ioremap and page table update code
// to track at which page-table levels entries have been modified.
// Based on that the code can better decide when page table changes need
// to be synchronized to other page-tables in the system.
//
pub const __PGTBL_PGD_MODIFIED: c_int = 0;
pub const __PGTBL_P4D_MODIFIED: c_int = 1;
pub const __PGTBL_PUD_MODIFIED: c_int = 2;
pub const __PGTBL_PMD_MODIFIED: c_int = 3;
pub const __PGTBL_PTE_MODIFIED: c_int = 4;

// Page-Table Modification Mask
pub type pgtbl_mod_mask = c_uint;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pgtable_level {
    PGTABLE_LEVEL_PTE = 0,
    PGTABLE_LEVEL_PMD,
    PGTABLE_LEVEL_PUD,
    PGTABLE_LEVEL_P4D,
    PGTABLE_LEVEL_PGD,
}

//
// ZSMALLOC needs to know the highest PFN on 32-bit architectures
// with physical address space extension, but falls back to
// BITS_PER_LONG otherwise.
//

pub const MAX_POSSIBLE_PHYSMEM_BITS: c_int = 32;

//
// On some architectures it depends on the mm if the p4d/pud or pmd
// layer of the page table hierarchy is folded or not.
//

//
// pXd_leaf() is the API to check whether a pgtable entry is a huge page
// mapping.  It should work globally across all archs, without any
// dependency on CONFIG_* options.  For architectures that do not support
// huge mappings on specific levels, below fallbacks will be used.
//
// A leaf pgtable entry should always imply the following:
//
// - It is a "present" entry.  IOW, before using this API, please check it
// with pXd_present() first. NOTE: it may not always mean the "present
// bit" is set.  For example, PROT_NONE entries are always "present".
//
// - It should _never_ be a swap entry of any type.  Above "present" check
// should have guarded this, but let's be crystal clear on this.
//
// - It should contain a huge PFN, which points to a huge page larger than
// PAGE_SIZE of the platform.  The PFN format isn't important here.
//
// - It should cover all kinds of huge mappings (i.e. pXd_trans_huge()
// or hugetlb mappings).
//

//
// We always define pmd_pfn for all archs as it's used in lots of generic
// code.  Now it happens too for pud_pfn (and can happen for larger
// mappings too in the future; we're not there yet).  Instead of defining
// it for all archs (like pmd_pfn), provide a fallback.
//
// Note that returning 0 here means any arch that didn't define this can
// get severely wrong when it hits a real pud leaf.  It's arch's
// responsibility to properly define it when a huge pud is possible.
//

pub const pud_pfn(x): c_int = 0;

//
// Some architectures have MMUs that are configurable or selectable at boot
// time. These lead to variable PTRS_PER_x. For statically allocated arrays it
// helps to have a static maximum value.
//

// description of effects of mapping type and prot in current implementation.
// this is due to the limited x86 page protection hardware.  The expected
// behavior is in parens:
//
// map_type	prot
// PROT_NONE	PROT_READ	PROT_WRITE	PROT_EXEC
// MAP_SHARED	r: (no) no	r: (yes) yes	r: (no) yes	r: (no) yes
// w: (no) no	w: (no) no	w: (yes) yes	w: (no) no
// x: (no) no	x: (no) yes	x: (no) yes	x: (yes) yes
//
// MAP_PRIVATE	r: (no) no	r: (yes) yes	r: (no) yes	r: (no) yes
// w: (no) no	w: (no) no	w: (copy) copy	w: (no) no
// x: (no) no	x: (no) yes	x: (no) yes	x: (yes) yes
//
// On arm64, PROT_EXEC has the following behaviour for both MAP_SHARED and
// MAP_PRIVATE (with Enhanced PAN supported):
// r: (no) no
// w: (no) no
// x: (yes) yes
//

