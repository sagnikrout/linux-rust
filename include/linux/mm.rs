//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mm.h
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
    pub fn arch_mm_preinit();
}
extern "C" {
    pub fn mm_core_init_early();
}
extern "C" {
    pub fn mm_core_init();
}
extern "C" {
    pub fn init_mm_internals();
}
//
// Convert between pages and MB
// 20 is the shift for 1MB (2^20 = 1MB)
// PAGE_SHIFT is the shift for page size (e.g., 12 for 4KB pages)
// So (20 - PAGE_SHIFT) converts between pages and MB
//

pub const sysctl_legacy_va_layout: c_int = 0;

//
// To prevent common memory management code establishing
// a zero page mapping on a read fault.
// This macro should be defined within <asm/pgtable.h>.
// s390 does this to prevent multiplexing of hardware bits
// related to the physical page in case of virtualization.
//

//
// On some architectures it is expensive to call memset() for small sizes.
// If an architecture decides to implement their own version of
// mm_zero_struct_page they should wrap the defines below in a #ifndef and
// define their own version of this macro in <asm/pgtable.h>
//

// This function must be updated when the size of struct page grows above 96
// or reduces below 56. The idea that compiler optimizes out switch()
// statement, and only leaves move/store instructions. Also the compiler can
// combine write statements if they are both assignments and can be reordered,
// this can result in several of the writes here being dropped.
//

// Check that struct page is either 56, 64, 72, 80, 88 or 96 bytes

//
// Default maximum number of active map areas, this limits the number of vmas
// per mm struct. Users can overwrite this number by sysctl but there is a
// problem.
//
// When a program's coredump is generated as ELF format, a section is created
// per a vma. In ELF, the number of sections is represented in unsigned short.
// This means the number of sections should be smaller than 65535 at coredump.
// Because the kernel adds some informative sections to a image of program at
// generating coredump, we need some margin. The number of extra sections is
// 1-3 now and depends on arch. We use "5" as safe margin, here.
//
// ELF extended numbering allows more than 65535 sections, so 16-bit bound is
// not a hard limit any more. Although some userspace tools can be surprised by
// that.
//

extern "C" {
    pub fn page_range_contiguous(page: *const page, nr_pages: c_ulong) -> bool;
}

// to align the pointer to the (next) page boundary

// to align the pointer to the (prev) page boundary

// test whether an address (unsigned long or pointer) is aligned to PAGE_SIZE

//
// folio_page_idx - Return the number of a page in a folio.
// @folio: The folio.
// @page: The folio page.
//
// This function expects that the page is actually part of the folio.
// The returned number is relative to the start of the folio.
//
extern "C" {
    pub fn list_entry(_arg: (head)->prev, folio: struct, _arg: lru) -> return;
}
//
// Linux kernel virtual memory manager primitives.
// The idea being to have a "virtual" mm in the same way
// we have a virtual fs - giving a cleaner interface to the
// mm details, and allowing different kinds of memory mappings
// (from shared memory to executable loading to arbitrary
// mmap() functions).
//
extern "C" {
    pub fn vm_area_free(: *mut vm_area_struct);
}

extern "C" {
    pub fn kobjsize(objp: *const c_void) -> c_uint;
}

//
// vm_flags in vm_area_struct, see mm_types.h.
// When changing, update also include/trace/events/mmflags.h
//
pub const VM_NONE: c_uint = 0x00000000;
//
// typedef vma_flag_t - specifies an individual VMA flag by bit number.
//
// This value is made type safe by sparse to avoid passing invalid flag values
// around.
//
pub type vma_flag_t = int ;

// mprotect() hardcodes VM_MAYREAD >> 4 == VM_READ, and so for r/w/x bits.

// nommu: R/O MAP_PRIVATE mapping that might overlay a file mapping

// Page-ranges managed without "struct page", just pure PFN
// These bits are reused, we define specific uses below.
//
// This flag is used to connect VFIO to arch specific KVM code. It
// indicates that the memory under this VMA is safe for use with any
// non-cachable memory type inside KVM. Some VFIO devices, on some
// platforms, are thought to be unsafe and can cause machine crashes
// if KVM does not lock down the memory type.
//

// Flags that reuse flags above.

//
// VM_SHADOW_STACK should not be set with VM_SHARED because of lack of
// support core mm.
//
// These VMAs will get a single end guard page. This helps userspace
// protect itself from attacks. A single page is enough for current
// shadow stack archs (x86). See the comments near alloc_shstk() in
// arch/x86/kernel/shstk.c for more details on the guard size.
//

//
// arm64's Guarded Control Stack implements similar functionality and
// has similar constraints to shadow stacks.
//

// Despite the naming, these are FLAGS not bits.

//
// vma_flags_t masks for the userfaultfd VMA flags. The two high-bit modes are
// gated on the same configs as their VM_* flags above -- both of which imply
// 64BIT -- so an out-of-range bit is never fed to mk_vma_flags() on a build
// whose bitmap cannot hold it.
//

// Bits set in the VMA until the stack is in its final location

// Common data flag combinations

// Temporary until VMA flags conversion complete.

// VMA basic access permission flags

//
// Special vmas that are non-mergable, non-mlock()able.
//

//
// Physically remapped pages are special. Tell the
// rest of the world about it:
// IO tells people not to look at these pages
// (accesses can have side effects).
// PFNMAP tells the core MM that the base pages are just
// raw PFN mappings, and do not have a "struct page" associated
// with them.
// DONTEXPAND
// Disable vma merging and expanding with mremap().
// DONTDUMP
// Omit vma from core dump, even when VM_IO turned off.
//

// This mask prevents VMA from being scanned with khugepaged

// This mask defines which mm->def_flags a process can inherit its parent

// This mask represents all the VMA flag bits used by mlock

// These flags can be updated atomically via VMA/mmap read lock.

// Arch-specific flags to clear when updating VM flags on protection change

//
// Flags which should be 'sticky' on merge - that is, flags which, when one VMA
// possesses it but the other does not, the merged VMA should nonetheless have
// applied to it:
//
// VMA_SOFTDIRTY_BIT - if a VMA is marked soft-dirty, that is has not had its
// references cleared via /proc/$pid/clear_refs, any
// merged VMA should be considered soft-dirty also as it
// operates at a VMA granularity.
//
// VMA_MAYBE_GUARD_BIT - If a VMA may have guard regions in place it implies
// that mapped page tables may contain metadata not
// described by the VMA and thus any merged VMA may also
// contain this metadata, and thus we must make this flag
// sticky.
//

//
// VMA flags we ignore for the purposes of merge, i.e. one VMA possessing one
// of these flags and the other not does not preclude a merge.
//
// VMA_STICKY_FLAGS - When merging VMAs, VMA flags must match, unless they
// are 'sticky'. If any sticky flags exist in either VMA,
// we simply set all of them on the merged VMA.
//

//
// Flags which should result in page tables being copied on fork. These are
// flags which indicate that the VMA maps page tables which cannot be
// reconsistuted upon page fault, so necessitate page table copying upon fork.
//
// Note that these flags should be compared with the DESTINATION VMA not the
// source: VM_UFFD_WP and VM_UFFD_RWP may be cleared on the destination
// (dup_userfaultfd() -> userfaultfd_reset_ctx() when the parent context did
// not negotiate UFFD_FEATURE_EVENT_FORK), while all other flags propagate.
//
// VM_PFNMAP / VM_MIXEDMAP - These contain kernel-mapped data which cannot be
// reasonably reconstructed on page fault.
//
// VM_UFFD_WP - Encodes metadata about an installed uffd
// VM_UFFD_RWP  write- or read-write-protect handler, which
// cannot be reconstructed on page fault.
//
// We always copy pgtables when dst_vma has the
// uffd PTE bit in use even if it's file-backed
// (e.g. shmem). Because when the uffd bit is
// in use, the pgtable contains the protection
// information, that's something we can't
// retrieve from page cache, and skip copying
// will lose those info.
//
// VM_MAYBE_GUARD - Could contain page guard region markers which
// by design are a property of the page tables
// only and thus cannot be reconstructed on page
// fault.
//

//
// mapping from the currently active vm_flags protection bits (the
// low four bits) to a page protection mask..
//
// The default fault flags that should be used by most of the
// arch-specific page fault handlers.
//

//
// fault_flag_allow_retry_first - check ALLOW_RETRY the first time
// @flags: Fault flags.
//
// This is mostly used for places where we want to try to avoid taking
// the mmap_lock for too long a time when waiting for another condition
// to change, in which case we can try to be polite to release the
// mmap_lock in the first round to avoid potential starvation of other
// processes that would also want the mmap_lock.
//
// Return: true if the page fault allows retry and this is the first
// attempt of the fault handling; false otherwise.
//

//
// vm_fault is filled by the pagefault handler and passed to the vma's
// ->fault function. The vma's ->fault is responsible for returning a bitmask
// of VM_FAULT_xxx flags that give details about how the fault was handled.
//
// MM layer fills up gfp_mask for page allocations but fault handler might
// alter it if its implementation requires a different allocation context.
//
// pgoff should be used in favour of virtual_address, if possible.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vm_fault {
    pub /: *mut *mut *mut vm_area_vma; / Target VMA,
    pub /: *mut *mut gfp_t gfp_mask; / gfp mask to be used for allocations,
    pub /: *mut *mut pgoff_t pgoff; / Logical page offset based on vma,
    pub /: *mut *mut unsigned long address; / Faulting virtual address - masked,
    pub /: *mut *mut unsigned long real_address; / Faulting virtual address - unmasked,
}

// XXX: should really be 'const'
// the 'address'
//
// used by PMD fault only.
//
// page here, unless VM_FAULT_NOPAGE
// is set (which is also implied by
// VM_FAULT_ERROR).
//
// These three entries are valid only while holding ptl lock
// the 'address'. NULL if the page
// table hasn't been allocated.
//
// Protects pte page table if 'pte'
// is not NULL, otherwise pmd.
//
// vm_ops->map_pages() sets up a page
// table from atomic context.
// do_fault_around() pre-allocates
// page table to avoid allocation from
// atomic context.
//
// These are the virtual MM functions - opening of an area, closing and
// unmapping it (needed to keep files on disk up-to-date etc), pointer
// to the functions called when a no-page or a wp-page exception occurs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vm_operations_struct {
//
// @open: Called when a VMA is remapped, split or forked. Not called
// upon first mapping a VMA.
// Context: User context.  May sleep.  Caller holds mmap_lock.
//
    pub vma): *mut *mut void (open)(struct vm_area_struct,
//
// @close: Called when the VMA is being removed from the MM.
// Context: User context.  May sleep.  Caller holds mmap_lock.
//
    pub vma): *mut *mut void (close)(struct vm_area_struct,
//
// @mapped: Called when the VMA is first mapped in the MM. Not called if
// the new VMA is merged with an adjacent VMA.
//
// The @vm_private_data field is an output field allowing the user to
// modify vma->vm_private_data as necessary.
//
// ONLY valid if set from f_op->mmap_prepare. Will result in an error if
// set from f_op->mmap.
//
// Returns %0 on success, or an error otherwise. On error, the VMA will
// be unmapped.
//
// Context: User context.  May sleep.  Caller holds mmap_lock.
//
    pub vm_private_data): *const *const file file, void,
// Called any time before splitting to check if it's allowed
    pub addr): *mut *mut *mut int (may_split)(struct vm_area_struct vma, unsigned long,
    pub vma): *mut *mut int (mremap)(struct vm_area_struct,
//
// Called by mprotect() to make driver-specific permission
// checks before mprotect() is finalised.   The VMA must not
// be modified.  Returns 0 if mprotect() can proceed.
//
    pub newflags): unsigned long end, unsigned long,
    pub vmf): *mut *mut vm_fault_t (fault)(struct vm_fault,
    pub order): *mut *mut *mut vm_fault_t (huge_fault)(struct vm_fault vmf, unsigned int,
    pub end_pgoff): pgoff_t start_pgoff, pgoff_t,
    pub vma): *mut *mut unsigned long (pagesize)(struct vm_area_struct,
// notification that a previously read-only page is about to become
// writable, if an error is returned it will cause a SIGBUS
    pub vmf): *mut *mut vm_fault_t (page_mkwrite)(struct vm_fault,
// same as page_mkwrite when using VM_PFNMAP|VM_MIXEDMAP
    pub vmf): *mut *mut vm_fault_t (pfn_mkwrite)(struct vm_fault,
// called by access_process_vm when get_user_pages() fails, typically
// for use by special VMAs. See also generic_access_phys() for a generic
// implementation useful for any iomem mapping.
//
    pub write): *mut *mut void buf, int len, int,
// Called by the /proc/PID/maps code to ask the vma whether it
// has a special name.  Returning non-NULL will also cause this
// vma to be dumped unconditionally.
    pub vma): *const *const *const char (name)(struct vm_area_struct,

//
// set_policy() op must add a reference to any non-NULL @new mempolicy
// to hold the policy upon return.  Caller should pass NULL @new to
// remove a policy and fall back to surrounding context--i.e. do not
// install a MPOL_DEFAULT policy, nor the task or system default
// mempolicy.
//
    pub new): *mut *mut *mut int (set_policy)(struct vm_area_struct vma, struct mempolicy,
//
// get_policy() op must add reference [mpol_get()] to any policy at
// (vma,addr) marked as MPOL_SHARED.  The shared policy infrastructure
// in mm/mempolicy.c will do this automatically.
// get_policy() must NOT add a ref if the policy at (vma,addr) is not
// marked as MPOL_SHARED. vma policies are protected by the mmap_lock.
// If no [shared/vma] mempolicy exists at the addr, get_policy() op
// must return NULL--i.e., do not "fallback" to task or system default
// policy.
//
    pub ilx): *mut unsigned long addr, pgoff_t,

//
// Called by vm_normal_page() for special PTEs in @vma at @addr. This
// allows for returning a "normal" page from vm_normal_page() even
// though the PTE indicates that the "struct page" either does not exist
// or should not be touched: "special".
//
// Do not add new users: this really only works when a "normal" page
// was mapped, but then the PTE got changed to something weird (+
// marked special) that would not make pte_pfn() identify the originally
// inserted page.
//
    pub addr): c_ulong,

    pub uffd_ops: *const vm_uffd_ops,

}

//
// These must be here rather than mmap_lock.h as dependent on vm_fault type,
// declared in this header.
//

extern "C" {
    pub fn test_bit(_arg: flag, _arg: ACCESS_PRIVATE(&mm->flags, _arg: __mm_flags)) -> return;
}
extern "C" {
    pub fn test_and_set_bit(_arg: flag, _arg: ACCESS_PRIVATE(&mm->flags, _arg: __mm_flags)) -> return;
}
extern "C" {
    pub fn test_and_clear_bit(_arg: flag, _arg: ACCESS_PRIVATE(&mm->flags, _arg: __mm_flags)) -> return;
}
// Use when VMA is not part of the VMA tree and needs no locking
//
// Use when VMA is part of the VMA tree and modifications need coordination
// Note: vm_flags_reset and vm_flags_reset_once do not lock the vma and
// it should be locked explicitly beforehand.
//
// It is assumed only the first system word must be written once.
// The remainder can be copied normally.
//
// Use only if VMA is not part of the VMA tree or has no other users and
// therefore needs no locking.
//
// Use only when the order of set/clear operations is unimportant, otherwise
// use vm_flags_{set|clear} explicitly.
//
// Only specific flags are permitted
//
// Set VMA flag atomically. Requires only VMA/mmap read lock. Only specific
// valid flags are allowed to do this.
//
// Test for VMA flag atomically. Requires no locks. Only specific valid flags
// are allowed to do this.
//
// This is necessarily racey, so callers must ensure that serialisation is
// achieved through some other means, or that races are permissible.
//
extern "C" {
    pub fn test_bit(int)bit: (, _arg: &vma->vm_flags) -> return;
}
// Set an individual VMA flag in flags, non-atomically.
//
// Helper macro which bitwise-or combines the specified input flags into a
// vma_flags_t bitmap value. E.g.:
//
// vma_flags_t flags = mk_vma_flags(VMA_IO_BIT, VMA_PFNMAP_BIT,
// VMA_DONTEXPAND_BIT, VMA_DONTDUMP_BIT);
//
// The compiler cleverly optimises away all of the work and this ends up being
// equivalent to aggregating the values manually.
//

//
// Helper macro which acts like mk_vma_flags, only appending to a copy of the
// specified flags rather than establishing new flags. E.g.:
//
// vma_flags_t flags = append_vma_flags(VMA_STACK_DEFAULT_FLAGS, VMA_STACK_BIT,
// VMA_ACCOUNT_BIT);
//

// Calculates the number of set bits in the specified VMA flags.
extern "C" {
    pub fn bitmap_weight(_arg: bitmap, _arg: NUM_VMA_FLAG_BITS) -> return;
}
//
// Test whether a specific VMA flag is set, e.g.:
//
// if (vma_flags_test(flags, VMA_READ_BIT)) { ... }
//
extern "C" {
    pub fn test_bit(int)bit: (, _arg: bitmap) -> return;
}
//
// Obtain a set of VMA flags which contain the overlapping flags contained
// within flags and to_and.
//
// Obtain a set of VMA flags which contains the specified overlapping flags,
// e.g.:
//
// vma_flags_t read_flags = vma_flags_and(&flags, VMA_READ_BIT,
// VMA_MAY_READ_BIT);
//

// Test each of to_test flags in flags, non-atomically.
extern "C" {
    pub fn bitmap_intersects(_arg: bitmap_to_test, _arg: bitmap, _arg: NUM_VMA_FLAG_BITS) -> return;
}
//
// Test whether any specified VMA flag is set, e.g.:
//
// if (vma_flags_test_any(flags, VMA_READ_BIT, VMA_MAYREAD_BIT)) { ... }
//

// Test that ALL of the to_test flags are set, non-atomically.
extern "C" {
    pub fn bitmap_subset(_arg: bitmap_to_test, _arg: bitmap, _arg: NUM_VMA_FLAG_BITS) -> return;
}
//
// Test whether ALL specified VMA flags are set, e.g.:
//
// if (vma_flags_test_all(flags, VMA_READ_BIT, VMA_MAYREAD_BIT)) { ... }
//

//
// Helper to test that a flag mask of type vma_flags_t has a SINGLE flag set
// (returning false if flagmask has no flags set).
//
// This is defined to make the semantics clearer when testing an optionally
// defined VMA flags mask, e.g.:
//
// if (vma_flags_test_single_mask(&flags, VMA_DROPPABLE)) { ... }
//
// When VMA_DROPPABLE is defined if available, or set to EMPTY_VMA_FLAGS
// otherwise.
//
extern "C" {
    pub fn vma_flags_test_any_mask(_arg: flags, _arg: flagmask) -> return;
}
// Set each of the to_set flags in flags, non-atomically.
//
// Set all specified VMA flags, e.g.:
//
// vma_flags_set(&flags, VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT);
//

//
// Combine pre-computed vma_flags_t masks into one value, e.g.:
//
// vma_flags_t flags = mk_vma_flags_from_masks(VMA_UFFD_WP, VMA_UFFD_MINOR);
//
// Unlike mk_vma_flags(), which takes bit numbers, this takes whole masks --
// each of which may be EMPTY_VMA_FLAGS when its feature is unavailable -- so a
// bit that does not exist on the current build is never materialised.
//

// Clear all of the to-clear flags in flags, non-atomically.
//
// Clear all specified individual flags, e.g.:
//
// vma_flags_clear(&flags, VMA_READ_BIT, VMA_WRITE_BIT, VMA_EXEC_BIT);
//

//
// Obtain a VMA flags value containing those flags that are present in flags or
// flags_other but not in both.
//
// Determine if flags and flags_other have precisely the same flags set.
extern "C" {
    pub fn bitmap_equal(_arg: bitmap, _arg: bitmap_other, _arg: NUM_VMA_FLAG_BITS) -> return;
}
// Determine if flags and flags_other have precisely the same flags set.
extern "C" {
    pub fn bitmap_equal(_arg: bitmap, _arg: bitmap_other, _arg: NUM_VMA_FLAG_BITS) -> return;
}
//
// Helper macro to determine if only the specific flags are set, e.g.:
//
// if (vma_flags_same(&flags, VMA_WRITE_BIT) { ... }
//

//
// Test whether a specific flag in the VMA is set, e.g.:
//
// if (vma_test(vma, VMA_READ_BIT)) { ... }
//
extern "C" {
    pub fn vma_flags_test(_arg: &vma->flags, _arg: bit) -> return;
}
// Helper to test any VMA flags in a VMA .
extern "C" {
    pub fn vma_flags_test_any_mask(_arg: &vma->flags, _arg: flags) -> return;
}
//
// Helper macro for testing whether any VMA flags are set in a VMA,
// e.g.:
//
// if (vma_test_any(vma, VMA_IO_BIT, VMA_PFNMAP_BIT,
// VMA_DONTEXPAND_BIT, VMA_DONTDUMP_BIT)) { ... }
//

//
// Helper to test that ALL specified flags are set in a VMA.
//
// Note: appropriate locks must be held, this function does not acquire them for
// you.
//
extern "C" {
    pub fn vma_flags_test_all_mask(_arg: &vma->flags, _arg: flags) -> return;
}
//
// Helper macro for checking that ALL specified flags are set in a VMA, e.g.:
//
// if (vma_test_all(vma, VMA_READ_BIT, VMA_MAYREAD_BIT) { ... }
//

//
// Helper to test that a flag mask of type vma_flags_t has a SINGLE flag set
// (returning false if flagmask has no flags set).
//
// This is useful when a flag needs to be either defined or not depending upon
// kernel configuration, e.g.:
//
// if (vma_test_single_mask(vma, VMA_DROPPABLE)) { ... }
//
// When VMA_DROPPABLE is defined if available, or set to EMPTY_VMA_FLAGS
// otherwise.
//
extern "C" {
    pub fn vma_flags_test_single_mask(_arg: &vma->flags, _arg: flagmask) -> return;
}
//
// Helper to set all VMA flags in a VMA.
//
// Note: appropriate locks must be held, this function does not acquire them for
// you.
//
// Helper macro for specifying VMA flags in a VMA, e.g.:
//
// vma_set_flags(vma, VMA_IO_BIT, VMA_PFNMAP_BIT, VMA_DONTEXPAND_BIT,
// VMA_DONTDUMP_BIT);
//
// Note: appropriate locks must be held, this function does not acquire them for
// you.
//

// Helper to clear all VMA flags in a VMA.
//
// Helper macro for clearing VMA flags, e.g.:
//
// vma_clear_flags(vma, VMA_IO_BIT, VMA_PFNMAP_BIT, VMA_DONTEXPAND_BIT,
// VMA_DONTDUMP_BIT);
//

//
// Test whether a specific VMA flag is set in a VMA descriptor, e.g.:
//
// if (vma_desc_test(desc, VMA_READ_BIT)) { ... }
//
extern "C" {
    pub fn vma_flags_test(_arg: &desc->vma_flags, _arg: bit) -> return;
}
// Helper to test any VMA flags in a VMA descriptor.
extern "C" {
    pub fn vma_flags_test_any_mask(_arg: &desc->vma_flags, _arg: flags) -> return;
}
//
// Helper macro for testing whether any VMA flags are set in a VMA descriptor,
// e.g.:
//
// if (vma_desc_test_any(desc, VMA_IO_BIT, VMA_PFNMAP_BIT,
// VMA_DONTEXPAND_BIT, VMA_DONTDUMP_BIT)) { ... }
//

// Helper to test all VMA flags in a VMA descriptor.
extern "C" {
    pub fn vma_flags_test_all_mask(_arg: &desc->vma_flags, _arg: flags) -> return;
}
//
// Helper macro for testing whether ALL VMA flags are set in a VMA descriptor,
// e.g.:
//
// if (vma_desc_test_all(desc, VMA_READ_BIT, VMA_MAYREAD_BIT)) { ... }
//

// Helper to set all VMA flags in a VMA descriptor.
//
// Helper macro for specifying VMA flags for an input pointer to a struct
// vm_area_desc object describing a proposed VMA, e.g.:
//
// vma_desc_set_flags(desc, VMA_IO_BIT, VMA_PFNMAP_BIT, VMA_DONTEXPAND_BIT,
// VMA_DONTDUMP_BIT);
//

// Helper to clear all VMA flags in a VMA descriptor.
//
// Helper macro for clearing VMA flags for an input pointer to a struct
// vm_area_desc object describing a proposed VMA, e.g.:
//
// vma_desc_clear_flags(desc, VMA_IO_BIT, VMA_PFNMAP_BIT, VMA_DONTEXPAND_BIT,
// VMA_DONTDUMP_BIT);
//

//
// Indicate if the VMA is a heap for the given task; for
// /proc/PID/maps that is the heap of the main task.
//
// Indicate if the VMA is a stack for the given task; for
// /proc/PID/maps that is the stack of the main task.
//
// We make no effort to guess what a given thread considers to be
// its "stack".  It's not even well-defined for programs written
// languages like Go.
//
extern "C" {
    pub fn vma_flags_can_grow(_arg: &vma->flags) -> return;
}
extern "C" {
    pub fn vma_flags_test_all(_arg: flags, _arg: VMA_SHARED_BIT, _arg: VMA_MAYWRITE_BIT) -> return;
}
extern "C" {
    pub fn is_shared_maywrite(_arg: &vma->flags) -> return;
}
//
// vma_kernel_pagesize - Default page size granularity for this VMA.
// @vma: The user mapping.
//
// The kernel page size specifies in which granularity VMA modifications
// can be performed. Folios in this VMA will be aligned to, and at least
// the size of the number of bytes returned by this function.
//
// The default kernel page size is not affected by Transparent Huge Pages
// being in effect.
//
// Return: The default page size granularity for this VMA.
//
extern "C" {
    pub fn vma_mmu_pagesize(vma: *mut vm_area_struct) -> c_ulong;
}
extern "C" {
    pub fn mas_find(_arg: &vmi->mas, 1: max -) -> return;
}
//
// Uses mas_find() to get the first VMA when the iterator starts.
// Calling mas_next() could skip the first entry.
//
extern "C" {
    pub fn mas_find(_arg: &vmi->mas, _arg: ULONG_MAX) -> return;
}
extern "C" {
    pub fn mas_next_range(_arg: &vmi->mas, _arg: ULONG_MAX) -> return;
}
extern "C" {
    pub fn mas_prev(_arg: &vmi->mas, _arg: 0) -> return;
}
// Free any unused preallocations

// The MM code likes to work with exclusive end addresses

//
// The vma_is_shmem is not inline because it is used only by slow
// paths in userfault.
//
extern "C" {
    pub fn vma_is_shmem(vma: *const vm_area_struct) -> bool;
}
extern "C" {
    pub fn vma_is_anon_shmem(vma: *const vm_area_struct) -> bool;
}

extern "C" {
    pub fn vma_is_stack_for_current(vma: *const vm_area_struct) -> c_int;
}
// flush_tlb_range() takes a vma, not a mm, and can care about flags

extern "C" {
    pub fn prep_compound_page(page: *mut page, order: c_uint);
}

//
// compound_order() can be called without holding a reference, which means
// that niceties like page_folio() don't work.  These callers should be
// prepared to handle wild return values.  For example, PG_head may be
// set before the order is initialised, or this may be a tail page.
// See compaction.c for some good examples.
//
extern "C" {
    pub fn folio_large_order(_arg: folio) -> return;
}
//
// folio_order - The allocation order of a folio.
// @folio: The folio.
//
// A folio is composed of 2^order pages.  See get_order() for the definition
// of order.
//
// Return: The order of the folio.
//
extern "C" {
    pub fn folio_large_order(_arg: folio) -> return;
}
//
// folio_reset_order - Reset the folio order and derived _nr_pages
// @folio: The folio.
//
// Reset the order and derived _nr_pages to 0. Must only be used in the
// process of splitting large folios.
//

//
// Methods to modify the page usage count.
//
// What counts for a page usage:
// - cache mapping   (page->mapping)
// - private data    (page->private)
// - page mapped in a task's page tables, each mapping
// is counted separately
//
// Also, many kernel routines increase the page count before a critical
// routine so they can be sure the page doesn't go away from under them.
//
// Drop a ref, return true if the refcount fell to zero (the page has no users)
//
extern "C" {
    pub fn page_ref_dec_and_test(_arg: page) -> return;
}
extern "C" {
    pub fn put_page_testzero(_arg: &folio->page) -> return;
}
//
// Try to grab a ref unless the page has a refcount of zero, return false if
// that is the case.
// This can be called when MMU is off so it must not access
// any of the virtual mappings.
//
extern "C" {
    pub fn page_ref_add_unless_zero(_arg: page, _arg: 1) -> return;
}
extern "C" {
    pub fn page_is_ram(pfn: c_ulong) -> c_int;
}
// Support for virtually mapped pages
extern "C" {
    pub fn vmalloc_to_pfn(addr: *const c_void) -> c_ulong;
}
//
// Determine if an address is within the vmalloc range
//
// On nommu, vmalloc/vfree wrap through kmalloc/kfree directly, so there
// is no special casing required.
//

extern "C" {
    pub fn is_vmalloc_addr(x: *const c_void) -> bool;
}
extern "C" {
    pub fn is_vmalloc_or_module_addr(x: *const c_void) -> c_int;
}

//
// How many times the entire folio is mapped as a single unit (eg by a
// PMD or PUD entry).  This is probably not what you want, except for
// debugging purposes or implementation of other core folio_*() primitives.
//
// folio_mapcount() - Number of mappings of this folio.
// @folio: The folio.
//
// The folio mapcount corresponds to the number of present user page table
// entries that reference any part of a folio. Each such present user page
// table entry must be paired with exactly on folio reference.
//
// For ordindary folios, each user page table entry (PTE/PMD/PUD/...) counts
// exactly once.
//
// For hugetlb folios, each abstracted "hugetlb" user page table entry that
// references the entire folio counts exactly once, even when such special
// page table entries are comprised of multiple ordinary page table entries.
//
// Will report 0 for pages which cannot be mapped into userspace, such as
// slab, page tables and similar.
//
// Return: The number of times this folio is mapped.
//
extern "C" {
    pub fn folio_large_mapcount(_arg: folio) -> return;
}
//
// folio_mapped - Is this folio mapped into userspace?
// @folio: The folio.
//
// Return: True if any page in this folio is referenced by user page tables.
//
extern "C" {
    pub fn compound_head(_arg: page) -> return;
}
extern "C" {
    pub fn page_folio(_arg: page) -> return;
}
extern "C" {
    pub fn __folio_put(folio: *mut folio);
}
extern "C" {
    pub fn split_page(page: *mut page, order: c_uint);
}
extern "C" {
    pub fn folio_copy(dst: *mut folio, src: *mut folio);
}
extern "C" {
    pub fn folio_mc_copy(dst: *mut folio, src: *mut folio) -> c_int;
}
extern "C" {
    pub fn nr_free_buffer_pages() -> c_ulong;
}
// Returns the number of bytes in this potentially compound page.
// Returns the number of bits needed for the number of bytes in a page
//
// thp_order - Order of a transparent huge page.
// @page: Head page of a transparent huge page.
//
extern "C" {
    pub fn compound_order(_arg: page) -> return;
}
//
// thp_size - Size of a transparent huge page.
// @page: Head page of a transparent huge page.
//
// Return: Number of bytes in this page.
//

//
// Do pte_mkwrite, but only if the vma says VM_WRITE.  We do this when
// servicing faults for write access.  In the normal case, do always want
// pte_mkwrite.  But get_user_pages can cause write faults for mappings
// that do not have writing enabled, when used by access_process_vm.
//
extern "C" {
    pub fn do_set_pmd(vmf: *mut vm_fault, folio: *mut folio, page: *mut page) -> vm_fault_t;
}
extern "C" {
    pub fn finish_fault(vmf: *mut vm_fault) -> vm_fault_t;
}

//
// Multiple processes may "see" the same page. E.g. for untouched
// mappings of /dev/null, all processes see the same page full of
// zeroes, and text pages of executables and shared libraries have
// only one copy in memory, at most, normally.
//
// For the non-reserved pages, page_count(page) denotes a reference count.
// page_count() == 0 means the page is free. page->lru is then used for
// freelist management in the buddy allocator.
// page_count() > 0  means the page has been allocated.
//
// Pages are allocated by the slab allocator in order to provide memory
// to kmalloc and kmem_cache_alloc. In this case, the management of the
// page, and the fields in 'struct page' are the responsibility of mm/slab.c
// unless a particular usage is carefully commented. (the responsibility of
// freeing the kmalloc memory is the caller's, of course).
//
// A page may be used by anyone else who does a __get_free_page().
// In this case, page_count still tracks the references, and should only
// be used through the normal accessor functions. The top bits of page->flags
// and page->virtual store page management information, but all other fields
// are unused and could be used privately, carefully. The management of this
// page is the responsibility of the one who allocated it, and those who have
// subsequently been given references to it.
//
// The other pages (we may call them "pagecache pages") are completely
// managed by the Linux memory manager: I/O, buffers, swapping etc.
// The following discussion applies only to them.
//
// A pagecache page contains an opaque `private' member, which belongs to the
// page's address_space. Usually, this is the address of a circular list of
// the page's disk buffers. PG_private must be set to tell the VM to call
// into the filesystem to release these pages.
//
// A folio may belong to an inode's memory mapping. In this case,
// folio->mapping points to the inode, and folio->index is the file
// offset of the folio, in units of PAGE_SIZE.
//
// If pagecache pages are not associated with an inode, they are said to be
// anonymous pages. These may become associated with the swapcache, and in that
// case PG_swapcache is set, and page->private is an offset into the swapcache.
//
// In either case (swapcache or inode backed), the pagecache itself holds one
// reference to the page. Setting PG_private should also increment the
// refcount. The each user mapping also has a reference to the page.
//
// The pagecache pages are stored in a per-mapping radix tree, which is
// rooted at mapping->i_pages, and indexed by offset.
// Where 2.4 and early 2.6 kernels kept dirty/clean pages in per-address_space
// lists, we instead now tag pages as dirty/writeback in the radix tree.
//
// All pagecache pages may be subject to I/O:
// - inode pages may need to be read from disk,
// - inode pages which have been modified and are MAP_SHARED may need
// to be written back to the inode on disk,
// - anonymous pages (including MAP_PRIVATE file mappings) which have been
// modified may need to be swapped out to swap space and (later) to be read
// back into memory.
//
// 127: arbitrary random number, small enough to assemble well

//
// folio_get - Increment the reference count on a folio.
// @folio: The folio.
//
// Context: May be called in any context, as long as you know that
// you have a refcount on the folio.  If you do not already have one,
// folio_try_get() may be the right interface for you to use.
//
// folio_put - Decrement the reference count on a folio.
// @folio: The folio.
//
// If the folio's reference count reaches zero, the memory will be
// released back to the page allocator and may be used by another
// allocation immediately.  Do not access the memory or the struct folio
// after calling folio_put() unless you can be sure that it wasn't the
// last reference.
//
// Context: May be called in process or interrupt context, but not in NMI
// context.  May be called while holding a spinlock.
//
// folio_put_refs - Reduce the reference count on a folio.
// @folio: The folio.
// @refs: The amount to subtract from the folio's reference count.
//
// If the folio's reference count reaches zero, the memory will be
// released back to the page allocator and may be used by another
// allocation immediately.  Do not access the memory or the struct folio
// after calling folio_put_refs() unless you can be sure that these weren't
// the last references.
//
// Context: May be called in process or interrupt context, but not in NMI
// context.  May be called while holding a spinlock.
//
extern "C" {
    pub fn folios_put_refs(folios: *mut folio_batch, refs: *mut c_uint);
}
//
// union release_pages_arg - an array of pages or folios
//
// release_pages() releases a simple array of multiple pages, and
// accepts various different forms of said page array: either
// a regular old boring array of pages, an array of folios, or
// an array of encoded page pointers.
//
// The transparent union syntax for this kind of "any of these
// argument types" is all kinds of ugly, so look away.
//
extern "C" {
    pub fn release_pages(_arg: release_pages_arg, nr: c_int);
}
//
// folios_put - Decrement the reference count on an array of folios.
// @folios: The folios.
//
// Like folio_put(), but for a batch of folios.  This is more efficient
// than writing the loop yourself as it will optimise the locks which need
// to be taken if the folios are freed.  The folios batch is returned
// empty and ready to be reused for another batch; there is no need to
// reinitialise it.
//
// Context: May be called in process or interrupt context, but not in NMI
// context.  May be called while holding a spinlock.
//
// GUP_PIN_COUNTING_BIAS, and the associated functions that use it, overload
// the page's refcount so that two separate items are tracked: the original page
// reference count, and also a new count of how many pin_user_pages() calls were
// made against the page. ("gup-pinned" is another term for the latter).
//
// With this scheme, pin_user_pages() becomes special: such pages are marked as
// distinct from normal pages. As such, the unpin_user_page() call (and its
// variants) must be used in order to release gup-pinned pages.
//
// Choice of value:
//
// By making GUP_PIN_COUNTING_BIAS a power of two, debugging of page reference
// counts with respect to pin_user_pages() and unpin_user_page() becomes
// simpler, due to the fact that adding an even power of two to the page
// refcount has the effect of using only the upper N bits, for the code that
// counts up using the bias value. This means that the lower bits are left for
// the exclusive use of the original code that increments and decrements by one
// (or at least, by much smaller values than the bias value).
//
// Of course, once the lower bits overflow into the upper bits (and this is
// OK, because subtraction recovers the original values), then visual inspection
// no longer suffices to directly view the separate counts. However, for normal
// applications that don't have huge page reference counts, this won't be an
// issue.
//
// Locking: the lockless algorithm described in folio_try_get_rcu()
// provides safe operation for get_user_pages(), folio_mkclean() and
// other calls that race to set up page table entries.
//

extern "C" {
    pub fn unpin_user_page(page: *mut page);
}
extern "C" {
    pub fn unpin_folio(folio: *mut folio);
}
extern "C" {
    pub fn unpin_user_pages(pages: *mut page, npages: c_ulong);
}
extern "C" {
    pub fn unpin_user_folio(folio: *mut folio, npages: c_ulong);
}
extern "C" {
    pub fn unpin_folios(folios: *mut folio, nfolios: c_ulong);
}
//
// vma_flags_is_cow_mapping() - Do these VMA flags imply a CoW mapping?
// @flags: The VMA flags to check.
//
// Mappings which could be CoW'd (subject to Copy-On-Write faults) are
// described as CoW mappings.
//
// All mappings backed by anonymous folios (all anonymous mappings and most
// MAP_PRIVATE-file backed ranges) are CoW mappings.
//
// All other mappings (including all MAP_SHARED mappings) are non-CoW.
//
// The criteria are !VMA_SHARED_BIT, VMA_MAYWRITE_BIT.
//
// VMA_MAYWRITE_BIT is checked instead of VMA_WRITE_BIT to account for both
// future mprotect() calls which can render a read-only mapping writable, and
// GUP with FOLL_FORCE (e.g. ptrace) which can CoW a read-only mapping.
//
// - No anonymous mapping can ever clear VMA_MAYWRITE_BIT.
//
// - Writes to anonymous mappings do not immediately result in CoW faults but
// may do so after the process is forked or if a read is followed by a
// write.
//
// - Writes to MAP_PRIVATE file-backed mappings result in CoW faults and may
// do so again after fork.
//
// - MAP_SHARED mappings of a file opened read-only are transformed into
// VMA_MAYSHARE_BIT, !VMA_SHARED_BIT, !VMA_MAYWRITE_BIT mappings, so remain
// non-CoW.
//
// - Drivers may clear VMA_MAYWRITE_BIT but do so at mmap() time and cannot
// mark themselves anonymous. Having cleared this flag it is not valid for
// them to leave the VMA_WRITE_BIT flag set.
//
// As a consequence, the anonymous reverse mapping only tracks CoW mappings.
//
// Returns: true if the flags indicate a CoW mapping, otherwise false.
//
// vma_is_cow_mapping() - Is this VMA a CoW mapping?
// @vma: The VMA to check.
//
// See vma_flags_is_cow_mapping() for details.
//
// Returns: true if the VMA is a CoW mapping, otherwise false.
//
extern "C" {
    pub fn vma_flags_is_cow_mapping(_arg: &vma->flags) -> return;
}
//
// vma_desc_is_cow_mapping() - Is this VMA descriptor a CoW mapping?
// @desc: The VMA descriptor to check.
//
// See vma_flags_is_cow_mapping() for details.
//
// Returns: true if the VMA descriptor describes a CoW mapping, otherwise
// false.
//
extern "C" {
    pub fn vma_flags_is_cow_mapping(_arg: &desc->vma_flags) -> return;
}

//
// NOMMU shared mappings are ordinary MAP_SHARED mappings and selected
// R/O MAP_PRIVATE file mappings that are an effective R/O overlay of
// a file mapping. R/O MAP_PRIVATE mappings might still modify
// underlying memory if ptrace is active, so this is only possible if
// ptrace does not apply. Note that there is no mprotect() to upgrade
// write permissions later.
//
extern "C" {
    pub fn vma_flags_test_any(_arg: flags, _arg: VMA_MAYSHARE_BIT, _arg: VMA_MAYOVERLAY_BIT) -> return;
}

// Macro flag: #define SECTION_IN_PAGE_FLAGS

//
// The identification function is mainly used by the buddy allocator for
// determining if two pages could be buddies. We are not really identifying
// the zone since we could be using the section number id if we do not have
// node id available in page flags.
// We only guarantee that it will return the same value for two combinable
// pages in a zone.
//

extern "C" {
    pub fn memdesc_nid(mdf: *const memdesc_flags_t) -> c_int;
}

extern "C" {
    pub fn memdesc_nid(_arg: &(PF_POISONED_CHECK(page)->flags)) -> return;
}
extern "C" {
    pub fn memdesc_nid(_arg: &folio->flags) -> return;
}

// page access time bits needs to hold at least 4 seconds
pub const PAGE_ACCESS_TIME_MIN_BITS: c_int = 12;

pub const PAGE_ACCESS_TIME_BUCKETS: c_int = 0;

extern "C" {
    pub fn cpu_to_node(_arg: cpupid_to_cpu(cpupid)) -> return;
}
extern "C" {
    pub fn cpupid_to_pid(LAST__PID_MASK: cpupid) == (-1 &) -> return;
}
extern "C" {
    pub fn cpupid_to_cpu(LAST__CPU_MASK: cpupid) == (-1 &) -> return;
}

extern "C" {
    pub fn xchg(_arg: &folio->_last_cpupid, LAST_CPUPID_MASK: cpupid &) -> return;
}

extern "C" {
    pub fn folio_xchg_last_cpupid(folio: *mut folio, cpupid: c_int) -> c_int;
}

extern "C" {
    pub fn folio_use_access_time(folio: *mut folio) -> bool;
}

//
// KASAN per-page tags are stored xor'ed with 0xff. This allows to avoid
// setting tags for all pages to native kernel tag value 0xff, as the default
// value 0x00 maps to 0xff.
//

extern "C" {
    pub fn NODE_DATA(_arg: page_to_nid(page)) -> return;
}
extern "C" {
    pub fn NODE_DATA(_arg: folio_nid(folio)) -> return;
}

//
// folio_pfn - Return the Page Frame Number of a folio.
// @folio: The folio.
//
// A folio may contain multiple pages.  The pages have consecutive
// Page Frame Numbers.
//
// Return: The Page Frame Number of the first page in the folio.
//
extern "C" {
    pub fn page_to_pfn(_arg: &folio->page) -> return;
}
extern "C" {
    pub fn page_folio(_arg: pfn_to_page(pfn)) -> return;
}

extern "C" {
    pub fn pfn_pte(_arg: page_to_pfn(page), _arg: pgprot) -> return;
}
//
// folio_mk_pte - Create a PTE for this folio
// @folio: The folio to create a PTE for
// @pgprot: The page protection bits to use
//
// Create a page table entry for the first page of this folio.
// This is suitable for passing to set_ptes().
//
// Return: A page table entry suitable for mapping this folio.
//
extern "C" {
    pub fn pfn_pte(_arg: folio_pfn(folio), _arg: pgprot) -> return;
}

//
// folio_mk_pmd - Create a PMD for this folio
// @folio: The folio to create a PMD for
// @pgprot: The page protection bits to use
//
// Create a page table entry for the first page of this folio.
// This is suitable for passing to set_pmd_at().
//
// Return: A page table entry suitable for mapping this folio.
//
extern "C" {
    pub fn pmd_mkhuge(_arg: pfn_pmd(folio_pfn(folio), _arg: pgprot)) -> return;
}

//
// folio_mk_pud - Create a PUD for this folio
// @folio: The folio to create a PUD for
// @pgprot: The page protection bits to use
//
// Create a page table entry for the first page of this folio.
// This is suitable for passing to set_pud_at().
//
// Return: A page table entry suitable for mapping this folio.
//
extern "C" {
    pub fn pud_mkhuge(_arg: pfn_pud(folio_pfn(folio), _arg: pgprot)) -> return;
}

extern "C" {
    pub fn folio_test_large(_arg: folio) -> return;
}
//
// folio_maybe_dma_pinned - Report if a folio may be pinned for DMA.
// @folio: The folio.
//
// This function checks if a folio has been pinned via a call to
// a function in the pin_user_pages() family.
//
// For small folios, the return value is partially fuzzy: false is not fuzzy,
// because it means "definitely not pinned for DMA", but true means "probably
// pinned for DMA, but possibly a false positive due to having at least
// GUP_PIN_COUNTING_BIAS worth of normal folio references".
//
// False positives are OK, because: a) it's unlikely for a folio to
// get that many refcounts, and b) all the callers of this routine are
// expected to be able to deal gracefully with a false positive.
//
// For most large folios, the result will be exactly correct. That's because
// we have more tracking data available: the _pincount field is used
// instead of the GUP_PIN_COUNTING_BIAS scheme.
//
// For more information, please see Documentation/core-api/pin_user_pages.rst.
//
// Return: True, if it is likely that the folio has been "dma-pinned".
// False, if the folio is definitely not dma-pinned.
//
// folio_ref_count() is signed. If that refcount overflows, then
// folio_ref_count() returns a negative value, and callers will avoid
// further incrementing the refcount.
//
// Here, for that overflow case, use the sign bit to count a little
// bit higher via unsigned math, and thus still get an accurate result.
//
// This should most likely only be called during fork() to see whether we
// should break the cow immediately for an anon page on the src mm.
//
// The caller has to hold the PT lock and the vma->vm_mm->->write_protect_seq.
//
extern "C" {
    pub fn folio_maybe_dma_pinned(_arg: folio) -> return;
}
//
// is_zero_page - Query if a page is a zero page
// @page: The page to query
//
// This returns true if @page is one of the permanent zero pages.
//
extern "C" {
    pub fn is_zero_pfn(_arg: page_to_pfn(page)) -> return;
}
//
// is_zero_folio - Query if a folio is a zero page
// @folio: The folio to query
//
// This returns true if @folio is one of the permanent zero pages.
//
extern "C" {
    pub fn is_zero_page(_arg: &folio->page) -> return;
}
// MIGRATE_CMA and ZONE_MOVABLE do not allow pin folios

// The zero page can be "pinned" but gets special handling.
// Coherent device memory must always allow eviction.
//
// Filesystems can only tolerate transient delays to truncate and
// hole-punch operations
//
// Otherwise, non-movable zone folios can be pinned.

//
// folio_nr_pages - The number of pages in the folio.
// @folio: The folio.
//
// Return: A positive power of two.
//
extern "C" {
    pub fn folio_large_nr_pages(_arg: folio) -> return;
}
//
// compound_nr() returns the number of pages in this potentially compound
// page.  compound_nr() can be called on a tail page, and is defined to
// return 1 in that case.
//
extern "C" {
    pub fn folio_large_nr_pages(_arg: folio) -> return;
}
//
// folio_next - Move to the next physical folio.
// @folio: The folio we're currently operating on.
//
// If you have physically contiguous memory which may span more than
// one folio (eg a &struct bio_vec), use this function to move from one
// folio to the next.  Do not use it if the memory is only virtually
// contiguous as the folios are almost certainly not adjacent to each
// other.  This is the folio equivalent to writing ``page++``.
//
// Context: We assume that the folios are refcounted and/or locked at a
// higher level and do not adjust the reference counts.
// Return: The next struct folio.
//
// folio_shift - The size of the memory described by this folio.
// @folio: The folio.
//
// A folio represents a number of bytes which is a power-of-two in size.
// This function tells you which power-of-two the folio is.  See also
// folio_size() and folio_order().
//
// Context: The caller should have a reference on the folio to prevent
// it from being split.  It is not necessary for the folio to be locked.
// Return: The base-2 logarithm of the size of this folio.
//
// folio_size - The number of bytes in a folio.
// @folio: The folio.
//
// Context: The caller should have a reference on the folio to prevent
// it from being split.  It is not necessary for the folio to be locked.
// Return: The number of bytes in this folio.
//
// folio_maybe_mapped_shared - Whether the folio is mapped into the page
// tables of more than one MM
// @folio: The folio.
//
// This function checks if the folio maybe currently mapped into more than one
// MM ("maybe mapped shared"), or if the folio is certainly mapped into a single
// MM ("mapped exclusively").
//
// For KSM folios, this function also returns "mapped shared" when a folio is
// mapped multiple times into the same MM, because the individual page mappings
// are independent.
//
// For small anonymous folios and anonymous hugetlb folios, the return
// value will be exactly correct: non-KSM folios can only be mapped at most once
// into an MM, and they cannot be partially mapped. KSM folios are
// considered shared even if mapped multiple times into the same MM.
//
// For other folios, the result can be fuzzy:
// #. For partially-mappable large folios (THP), the return value can wrongly
// indicate "mapped shared" (false positive) if a folio was mapped by
// more than two MMs at one point in time.
// #. For pagecache folios (including hugetlb), the return value can wrongly
// indicate "mapped shared" (false positive) when two VMAs in the same MM
// cover the same file range.
//
// Further, this function only considers current page table mappings that
// are tracked using the folio mapcount(s).
//
// This function does not consider:
// #. If the folio might get mapped in the (near) future (e.g., swapcache,
// pagecache, temporary unmapping for migration).
// #. If the folio is mapped differently (VM_PFNMAP).
// #. If hugetlb page table sharing applies. Callers might want to check
// hugetlb_pmd_shared().
//
// Return: Whether the folio is estimated to be mapped into more than one MM.
//
// Only partially-mappable folios require more care.
//
// vm_insert_page() without CONFIG_TRANSPARENT_HUGEPAGE ...
// simply assume "mapped shared", nobody should really care
// about this for arbitrary kernel allocations.
//
// A single mapping implies "mapped exclusively", even if the
// folio flag says something different: it's easier to handle this
// case here instead of on the RMAP hot path.
//
extern "C" {
    pub fn test_bit(_arg: FOLIO_MM_IDS_SHARED_BITNUM, _arg: &folio->_mm_ids) -> return;
}
//
// folio_expected_ref_count - calculate the expected folio refcount
// @folio: the folio
//
// Calculate the expected folio refcount, taking references from the pagecache,
// swapcache, PG_private and page table mappings into account. Useful in
// combination with folio_ref_count() to detect unexpected references (e.g.,
// GUP or other temporary references).
//
// Does currently not consider references from the LRU cache. If the folio
// was isolated from the LRU (which is the case during migration or split),
// the LRU cache does not apply.
//
// Calling this function on an unmapped folio -- !folio_mapped() -- that is
// locked will return a stable result.
//
// Calling this function on a mapped folio will not result in a stable result,
// because nothing stops additional page table mappings from coming (e.g.,
// fork()) or going (e.g., munmap()).
//
// Calling this function without the folio lock will also not result in a
// stable result: for example, the folio might get dropped from the swapcache
// concurrently.
//
// However, even when called without the folio lock or on a mapped folio,
// this function can be used to detect unexpected references early (for example,
// if it makes sense to even lock the folio and unmap it).
//
// The caller must add any reference (e.g., from folio_try_get()) it might be
// holding itself to the result.
//
// Returns: the expected folio refcount.
//
// One reference per page from the swapcache.
// One reference per page from the pagecache.
// One reference from PG_private.
// One reference per page table mapping.

//
// Some inline functions in vmstat.h depend on page_zone()
//

// Macro flag: #define HASHED_PAGE_VIRTUAL

extern "C" {
    pub fn set_page_address(page: *mut page, virtual: *mut c_void);
}
extern "C" {
    pub fn page_address_init();
}

extern "C" {
    pub fn page_to_virt(_arg: page) -> return;
}

extern "C" {
    pub fn page_address(_arg: &folio->page) -> return;
}
//
// Return true only if the page has been allocated with
// ALLOC_NO_WATERMARKS and the low watermark was not
// met implying that the system is under some pressure.
//
// lru.next has bit 1 set if the page is allocated from the
// pfmemalloc reserves.  Callers may simply overwrite it if
// they do not need to preserve that information.
//
// Return true only if the folio has been allocated with
// ALLOC_NO_WATERMARKS and the low watermark was not
// met implying that the system is under some pressure.
//
// lru.next has bit 1 set if the page is allocated from the
// pfmemalloc reserves.  Callers may simply overwrite it if
// they do not need to preserve that information.
//
// Only to be called by the page allocator on a freshly allocated
// page.
//
// Can be called by the pagefault handler when it gets a VM_FAULT_OOM.
//
extern "C" {
    pub fn pagefault_out_of_memory();
}

//
// Parameter block passed down to zap_pte_range in exceptional cases.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zap_details {
    pub /: *mut *mut *mut folio single_folio; / Locked folio to be unmapped,
    pub /: *mut *mut bool skip_cows; / Do not zap COWed private pages,
    pub /: *mut *mut bool reclaim_pt; / Need reclaim page tables?,
    pub /: *mut *mut bool reaping; / Reaping, do not block.,
    pub /: *mut *mut zap_flags_t zap_flags; / Extra flags for zapping,
}

//
// Whether to drop the pte markers, for example, the uffd-wp information for
// file-backed memory.  This should only be specified when we will completely
// drop the page in the mm, either by truncation or unmapping of the vma.  By
// default, the flag is not set.
//

// Set in unmap_vmas() to indicate a final unmap call.  Only used by hugetlb

extern "C" {
    pub fn can_do_mlock() -> bool;
}

extern "C" {
    pub fn user_shm_lock(_arg: usize, : *mut ucounts) -> c_int;
}
extern "C" {
    pub fn user_shm_unlock(_arg: usize, : *mut ucounts);
}
//
// zap_vma - zap all page table entries in a vma
// @vma: The vma to zap.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct follow_pfnmap_args {
//
// Inputs:
// @vma: Pointer to @vm_area_struct struct
// @address: the virtual address to walk
//
    pub vma: *mut vm_area_struct,
    pub address: c_ulong,
//
// Internals:
//
// The caller shouldn't touch any of these.
//
    pub lock: *mut spinlock_t,
    pub ptep: *mut pte_t,
//
// Outputs:
//
// @pfn: the PFN of the address
// @addr_mask: address mask covering pfn
// @pgprot: the pgprot_t of the mapping
// @writable: whether the mapping is writable
// @special: whether the mapping is a special mapping (real PFN maps)
//
    pub pfn: c_ulong,
    pub addr_mask: c_ulong,
    pub pgprot: pgprot_t,
    pub writable: bool,
    pub special: bool,
}

extern "C" {
    pub fn follow_pfnmap_start(args: *mut follow_pfnmap_args) -> c_int;
}
extern "C" {
    pub fn follow_pfnmap_end(args: *mut follow_pfnmap_args);
}
extern "C" {
    pub fn truncate_pagecache(inode: *mut inode, new: loff_t);
}
extern "C" {
    pub fn truncate_setsize(inode: *mut inode, newsize: loff_t);
}
extern "C" {
    pub fn pagecache_isize_extended(inode: *mut inode, from: loff_t, to: loff_t);
}
extern "C" {
    pub fn truncate_pagecache_range(inode: *mut inode, offset: loff_t, end: loff_t);
}

// should never happen if there's no MMU

//
// Retrieves a single page alongside its VMA. Does not support FOLL_NOWAIT.
//
extern "C" {
    pub fn ERR_PTR(_arg: -EINVAL) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: got) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EINVAL) -> return;
}
// vmap = vma;
extern "C" {
    pub fn folio_add_pins(folio: *mut folio, pins: c_uint) -> c_int;
}
extern "C" {
    pub fn folio_add_pin(folio: *mut folio);
}
extern "C" {
    pub fn account_locked_vm(mm: *mut mm_struct, pages: c_ulong, inc: bool) -> c_int;
}
extern "C" {
    pub fn folio_mark_dirty(folio: *mut folio) -> bool;
}
extern "C" {
    pub fn folio_mark_dirty_lock(folio: *mut folio) -> bool;
}
extern "C" {
    pub fn set_page_dirty(page: *mut page) -> bool;
}
extern "C" {
    pub fn set_page_dirty_lock(page: *mut page) -> c_int;
}
extern "C" {
    pub fn get_cmdline(task: *mut task_struct, buffer: *mut c_char, buflen: c_int) -> c_int;
}
//
// Flags used by change_protection().  For now we make it a bitmap so
// that we can pass in multiple flags just like parameters.  However
// for now all the callers are only use one of the flags at the same
// time.
//
// Whether we should manually check if we can map individual PTEs writable,
// because something (e.g., COW, uffd-wp) blocks that from happening for all
// PTEs automatically in a writable mapping.
//

// Whether this protection change is for NUMA hints

// Whether this change is for write protecting

// Whether this change is for uffd RWP

//
// doesn't attempt to fault and will return short.
//
// per-process(per-mm_struct) statistics.
//
extern "C" {
    pub fn percpu_counter_read_positive(_arg: &mm->rss_stat[member]) -> return;
}
extern "C" {
    pub fn percpu_counter_sum_positive(_arg: &mm->rss_stat[member]) -> return;
}
extern "C" {
    pub fn mm_trace_rss_stat(mm: *mut mm_struct, member: c_int);
}
// Optimized variant when folio is already known not to be anon
extern "C" {
    pub fn mm_counter_file(_arg: folio) -> return;
}
extern "C" {
    pub fn max(_arg: mm->hiwater_rss, _arg: get_mm_rss(mm)) -> return;
}
extern "C" {
    pub fn max(_arg: mm->hiwater_vm, _arg: mm->total_vm) -> return;
}
// maxrss = hiwater_rss;

extern "C" {
    pub fn __p4d_alloc(mm: *mut mm_struct, pgd: *mut pgd_t, address: c_ulong) -> c_int;
}

extern "C" {
    pub fn __pud_alloc(mm: *mut mm_struct, p4d: *mut p4d_t, address: c_ulong) -> c_int;
}

extern "C" {
    pub fn __pmd_alloc(mm: *mut mm_struct, pud: *mut pud_t, address: c_ulong) -> c_int;
}

extern "C" {
    pub fn atomic_long_read(_arg: &mm->pgtables_bytes) -> return;
}

extern "C" {
    pub fn __pte_alloc(mm: *mut mm_struct, pmd: *mut pmd_t) -> c_int;
}
extern "C" {
    pub fn __pte_alloc_kernel(pmd: *mut pmd_t) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pt_flags {
    PT_kernel = PG_referenced,
    PT_reserved = PG_reserved,
// High bits are used for zone/node/section
}

extern "C" {
    pub fn page_ptdesc(_arg: virt_to_page(x)) -> return;
}
//
// ptdesc_address - Virtual address of page table.
// @pt: Page table descriptor.
//
// Return: The first byte of the page table described by @pt.
//
extern "C" {
    pub fn folio_address(_arg: ptdesc_folio(pt)) -> return;
}
extern "C" {
    pub fn test_bit(_arg: PT_reserved, _arg: &pt->pt_flags.f) -> return;
}
//
// ptdesc_set_kernel - Mark a ptdesc used to map the kernel
// @ptdesc: The ptdesc to be marked
//
// Kernel page tables often need special handling. Set a flag so that
// the handling code knows this ptdesc will not be used for userspace.
//
// ptdesc_clear_kernel - Mark a ptdesc as no longer used to map the kernel
// @ptdesc: The ptdesc to be unmarked
//
// Use when the ptdesc is no longer used to map the kernel and no longer
// needs special handling.
//
// Note: the 'PG_referenced' bit does not strictly need to be
// cleared before freeing the page. But this is nice for
// symmetry.
//
// ptdesc_test_kernel - Check if a ptdesc is used to map the kernel
// @ptdesc: The ptdesc being tested
//
// Call to tell if the ptdesc used to map the kernel.
//
extern "C" {
    pub fn test_bit(_arg: PT_kernel, _arg: &ptdesc->pt_flags.f) -> return;
}
//
// pagetable_alloc - Allocate pagetables
// @gfp:    GFP flags
// @order:  desired pagetable order
//
// pagetable_alloc allocates memory for page tables as well as a page table
// descriptor to describe that memory.
//
// Return: The ptdesc describing the allocated page tables.
//
extern "C" {
    pub fn page_ptdesc(_arg: page) -> return;
}

extern "C" {
    pub fn pagetable_free_kernel(pt: *mut ptdesc);
}

//
// pagetable_free - Free pagetables
// @pt:	The page table descriptor
//
// pagetable_free frees the memory of all page tables described by a page
// table descriptor and the memory for the descriptor itself.
//

extern "C" {
    pub fn ptlock_cache_init() -> void __init;
}
extern "C" {
    pub fn ptlock_alloc(ptdesc: *mut ptdesc) -> bool;
}
extern "C" {
    pub fn ptlock_free(ptdesc: *mut ptdesc);
}

extern "C" {
    pub fn ptlock_ptr(_arg: *mut page_ptdesc(pmd_page(pmd))) -> return;
}
extern "C" {
    pub fn ptlock_ptr(_arg: virt_to_ptdesc(pte)) -> return;
}
//
// prep_new_page() initialize page->private (and therefore page->ptl)
// with 0. Make sure nobody took it in use in between.
//
// It can happen if arch try to use slab for page table allocation:
// slab code uses page->slab_cache, which share storage with page->ptl.
//

//
// We use mm->page_table_lock to guard all pagetable pages of the mm.
//

extern "C" {
    pub fn __pte_offset_map(_arg: pmd, _arg: addr, _arg: NULL) -> return;
}

extern "C" {
    pub fn virt_to_page(mask): *mut *mut (void )((unsigned long) pmd &) -> return;
}
extern "C" {
    pub fn page_ptdesc(_arg: pmd_pgtable_page(pmd)) -> return;
}
extern "C" {
    pub fn ptlock_ptr(_arg: pmd_ptdesc(pmd)) -> return;
}

extern "C" {
    pub fn ptlock_init(_arg: ptdesc) -> return;
}

//
// No scalability reason to split PUD locks yet, but follow the same pattern
// as the PMD locks to make it easier if we decide to.  The VM should not be
// considered ready to switch to split PUD locks yet; there may be places
// which need to be converted from page_table_lock.
//
extern "C" {
    pub fn pagecache_init() -> void __init;
}
extern "C" {
    pub fn free_initmem();
}
//
// Free reserved pages within range [PAGE_ALIGN(start), end & PAGE_MASK)
// into the buddy system. The freed pages will be poisoned with pattern
// "poison" if it's within range [0, UCHAR_MAX].
// Return pages freed into the buddy system.
//
extern "C" {
    pub fn adjust_managed_page_count(page: *mut page, count: c_long);
}
extern "C" {
    pub fn free_reserved_pages(page: *mut page, order: c_uint);
}
//
// Default method to free all the __init memory into the buddy system.
// The freed pages will be poisoned with pattern "poison" if it's within
// range [0, UCHAR_MAX].
// Return pages freed into the buddy system.
//
// FIXME: Using memblock node mappings, an architecture may initialise its
// zones, allocate the backing mem_map and account for memory holes in an
// architecture independent manner.
//
// An architecture is expected to register range of page frames backed by
// physical memory with memblock_add[_node]() before calling
// free_area_init() passing in the PFN each zone ends at. At a basic
// usage, an architecture is expected to do something like
//
// unsigned long max_zone_pfns[MAX_NR_ZONES] = {max_dma, max_normal_pfn,
// max_highmem_pfn};
// for_each_valid_physical_page_range()
// memblock_add_node(base, size, nid, MEMBLOCK_NONE)
// free_area_init(max_zone_pfns);
//
extern "C" {
    pub fn arch_zone_limits_init(max_zone_pfn: *mut c_ulong);
}
extern "C" {
    pub fn node_map_pfn_alignment() -> c_ulong;
}

// please see mm/page_alloc.c
extern "C" {
    pub fn early_pfn_to_nid(pfn: c_ulong) -> int __meminit;
}

extern "C" {
    pub fn mem_init();
}
extern "C" {
    pub fn mmap_init() -> void __init;
}
extern "C" {
    pub fn __show_mem(flags: c_uint, nodemask: *const nodemask_t, max_zone_idx: c_int);
}
extern "C" {
    pub fn si_mem_available() -> c_long;
}
extern "C" {
    pub fn si_meminfo(val: *mut *mut sysinfo);
}
extern "C" {
    pub fn si_meminfo_node(val: *mut sysinfo, nid: c_int);
}
extern "C" {
    pub fn warn_alloc(gfp_mask: gfp_t, nodemask: *const nodemask_t, fmt: *const c_char, ...);
}
extern "C" {
    pub fn setup_per_cpu_pageset();
}
// nommu.c
extern "C" {
    pub fn nommu_shrink_inode_mappings(: *mut inode, _arg: usize, _arg: usize) -> c_int;
}
// interval_tree.c

extern "C" {
    pub fn anon_rmap_tree_verify(avc: *mut anon_vma_chain);
}

// mmap.c
extern "C" {
    pub fn __vm_enough_memory(mm: *const mm_struct, pages: c_long, cap_sys_admin: c_int) -> c_int;
}
extern "C" {
    pub fn exit_mmap(: *mut mm_struct);
}
extern "C" {
    pub fn mm_take_all_locks(mm: *mut mm_struct) -> c_int;
}
extern "C" {
    pub fn mm_drop_all_locks(mm: *mut mm_struct);
}
extern "C" {
    pub fn set_mm_exe_file(mm: *mut mm_struct, new_exe_file: *mut file) -> c_int;
}
extern "C" {
    pub fn replace_mm_exe_file(mm: *mut mm_struct, new_exe_file: *mut file) -> c_int;
}
extern "C" {
    pub fn vm_stat_account(: *mut mm_struct, _arg: vm_flags_t, npages: c_long);
}
extern "C" {
    pub fn randomize_stack_top(stack_top: c_ulong) -> c_ulong;
}
extern "C" {
    pub fn randomize_page(start: c_ulong, range: c_ulong) -> c_ulong;
}
extern "C" {
    pub fn do_madvise(mm: *mut mm_struct, start: c_ulong, len_in: usize, behavior: c_int) -> c_int;
}

// Ignore errors

// This takes the mm semaphore itself
extern "C" {
    pub fn vm_brk_flags(addr: c_ulong, request: c_ulong, is_exec: bool) -> int __must_check;
}
extern "C" {
    pub fn vm_munmap(start: c_ulong, len: usize) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vm_unmapped_area_info {
pub const VM_UNMAPPED_AREA_TOPDOWN: c_int = 1;
    pub flags: c_ulong,
    pub length: c_ulong,
    pub low_limit: c_ulong,
    pub high_limit: c_ulong,
    pub align_mask: c_ulong,
    pub align_offset: c_ulong,
    pub start_gap: c_ulong,
}

extern "C" {
    pub fn vm_unmapped_area(info: *mut vm_unmapped_area_info) -> c_ulong;
}
// truncate.c
extern "C" {
    pub fn truncate_inode_pages(mapping: *mut address_space, lstart: loff_t);
}
extern "C" {
    pub fn truncate_inode_pages_final(mapping: *mut address_space);
}
// generic vm_area_ops exported for stackable file systems
extern "C" {
    pub fn filemap_fault(vmf: *mut vm_fault) -> vm_fault_t;
}
extern "C" {
    pub fn filemap_page_mkwrite(vmf: *mut vm_fault) -> vm_fault_t;
}
// Generic expand stack which grows the stack according to GROWS{UP,DOWN}
extern "C" {
    pub fn expand_stack_locked(vma: *mut vm_area_struct, address: c_ulong) -> c_int;
}
// Look up the first VMA which satisfies  addr < vm_end,  NULL if none.
extern "C" {
    pub fn find_vma(mm: *mut *mut mm_struct, addr: c_ulong) -> *mut vm_area_struct;
}
//
// Look up the first VMA which intersects the interval [start_addr, end_addr)
// NULL if none.  Assume start_addr < end_addr.
//
// vma_lookup() - Find a VMA at a specific address
// @mm: The process address space.
// @addr: The user address.
//
// Return: The vm_area_struct at the given address, %NULL otherwise.
//
extern "C" {
    pub fn mtree_load(_arg: &mm->mm_mt, _arg: addr) -> return;
}
// See reasoning around the VM_SHADOW_STACK definition
//
// vma_start_pgoff() - Get the page offset of the start of @vma
// @vma: The VMA whose page offset is required.
//
// If the VMA is file-backed, this is the page offset into the file.
//
// If @vma is anonymous, this is the virtual page offset of the start of the
// VMA - if unfaulted, then vma->vm_start >> PAGE_SHIFT, if faulted then the
// virtual page offset at the time of first fault.
//
// If @vma is a MAP_PRIVATE file-backed mapping, then this returns the
// page offset within the file.
//
// Edge cases: nommu does not abide by these, MAP_PRIVATE-/dev/zero satisfies
// vma_is_anonymous() but has file-backed page offset, and MAP_PRIVATE-pfnmap
// regions have their page offset set to the first PFN in the range.
//
// Returns: The page offset of the start of @vma.
//
// vma_end_pgoff() - Get the page offset of the exclusive end of @vma
// @vma: The VMA whose end page offset is required.
//
// This returns the exclusive end page offset of @vma, which is useful for
// expressing page offset ranges.
//
// See the description of vma_start_pgoff() for a description of VMA page
// offsets.
//
// Returns: The exclusive end page offset of @vma.
//
extern "C" {
    pub fn vma_start_pgoff(vma_pages(vma: vma) +) -> return;
}
//
// vma_last_pgoff() - Get the page offset of the last page in @vma
// @vma: The VMA whose last page offset is required.
//
// This returns the last page offset contained within @vma.
//
// See the description of vma_start_pgoff() for a description of VMA page
// offsets.
//
// Returns: The last page offset of @vma.
//
// vma_start_anon_pgoff() - Get the anonymous page offset of the start of @vma
// @vma: The VMA whose anonymous page offset is required.
//
// If unfaulted, then this is vma->vm_start >> PAGE_SHIFT, if faulted then the
// anonymous page offset at the time of first fault.
//
// If the VMA is anonymous, this returns the same value as vma_start_pgoff().
//
// This value is used for tracking MAP_PRIVATE file-backed mappings by their
// anonymous page offset.
//
// Returns: The anonymous page offset of the start of @vma.
//

//
// vma_end_anon_pgoff() - Get the anonymous page offset of the exclusive end of
// @vma.
// @vma: The VMA whose end anonymous page offset is required.
//
// This returns the anonymous exclusive end page offset of @vma, which is useful
// for expressing page offset ranges.
//
// See the description of vma_start_anon_pgoff() for a description of VMA
// anonymous page offsets.
//
// Returns: The exclusive end anonymous page offset of @vma.
//
extern "C" {
    pub fn vma_start_anon_pgoff(vma_pages(vma: vma) +) -> return;
}
//
// vma_last_anon_pgoff() - Get the anonymous page offset of the last page in
// @vma.
// @vma: The VMA whose last anonymous page offset is required.
//
// See the description of vma_start_anon_pgoff() for a description of VMA
// anonymous page offsets.
//
// Returns: The last anonymous page offset of @vma.
//
// mmap_action_remap - helper for mmap_prepare hook to specify that a pure PFN
// remap is required.
// @desc: The VMA descriptor for the VMA requiring remap.
// @start: The virtual address to start the remap from, must be within the VMA.
// @start_pfn: The first PFN in the range to remap.
// @size: The size of the range to remap, in bytes, at most spanning to the end
// of the VMA.
//
// [start, start + size) must be within the VMA.
//
// mmap_action_remap_full - helper for mmap_prepare hook to specify that the
// entirety of a VMA should be PFN remapped.
// @desc: The VMA descriptor for the VMA requiring remap.
// @start_pfn: The first PFN in the range to remap.
//
// mmap_action_ioremap - helper for mmap_prepare hook to specify that a pure PFN
// I/O remap is required.
// @desc: The VMA descriptor for the VMA requiring remap.
// @start: The virtual address to start the remap from, must be within the VMA.
// @start_pfn: The first PFN in the range to remap.
// @size: The size of the range to remap, in bytes, at most spanning to the end
// of the VMA.
//
// mmap_action_ioremap_full - helper for mmap_prepare hook to specify that the
// entirety of a VMA should be PFN I/O remapped.
// @desc: The VMA descriptor for the VMA requiring remap.
// @start_pfn: The first PFN in the range to remap.
//
// mmap_action_simple_ioremap - helper for mmap_prepare hook to specify that the
// physical range in [start_phys_addr, start_phys_addr + size) should be I/O
// remapped.
// @desc: The VMA descriptor for the VMA requiring remap.
// @start_phys_addr: Start of the physical memory to be mapped.
// @size: Size of the area to map.
//
// NOTE: Some drivers might want to tweak desc->page_prot for purposes of
// write-combine or similar.
//
// mmap_action_map_kernel_pages - helper for mmap_prepare hook to specify that
// @num kernel pages contained in the @pages array should be mapped to userland
// starting at virtual address @start.
// @desc: The VMA descriptor for the VMA requiring kernel pags to be mapped.
// @start: The virtual address from which to map them.
// @pages: An array of struct page pointers describing the memory to map.
// @nr_pages: The number of entries in the @pages aray.
//
// mmap_action_map_kernel_pages_full - helper for mmap_prepare hook to specify that
// kernel pages contained in the @pages array should be mapped to userland
// from @desc->start to @desc->end.
// @desc: The VMA descriptor for the VMA requiring kernel pags to be mapped.
// @pages: An array of struct page pointers describing the memory to map.
//
// The caller must ensure that @pages contains sufficient entries to cover the
// entire range described by @desc.
//
extern "C" {
    pub fn mmap_action_prepare(desc: *mut vm_area_desc) -> c_int;
}
// Look up the first VMA which exactly match the interval vm_start ... vm_end
//
// range_is_subset - Is the specified inner range a subset of the outer range?
// @outer_start: The start of the outer range.
// @outer_end: The exclusive end of the outer range.
// @inner_start: The start of the inner range.
// @inner_end: The exclusive end of the inner range.
//
// Returns: %true if [inner_start, inner_end) is a subset of [outer_start,
// outer_end), otherwise %false.
//
// range_in_vma - is the specified [@start, @end) range a subset of the VMA?
// @vma: The VMA against which we want to check [@start, @end).
// @start: The start of the range we wish to check.
// @end: The exclusive end of the range we wish to check.
//
// Returns: %true if [@start, @end) is a subset of [@vma->vm_start,
// @vma->vm_end), %false otherwise.
//
extern "C" {
    pub fn range_is_subset(_arg: vma->vm_start, _arg: vma->vm_end, _arg: start, _arg: end) -> return;
}
//
// range_in_vma_desc - is the specified [@start, @end) range a subset of the VMA
// described by @desc, a VMA descriptor?
// @desc: The VMA descriptor against which we want to check [@start, @end).
// @start: The start of the range we wish to check.
// @end: The exclusive end of the range we wish to check.
//
// Returns: %true if [@start, @end) is a subset of [@desc->start, @desc->end),
// %false otherwise.
//
extern "C" {
    pub fn range_is_subset(_arg: desc->start, _arg: desc->end, _arg: start, _arg: end) -> return;
}

extern "C" {
    pub fn vm_get_page_prot(vm_flags: vm_flags_t) -> pgprot_t;
}
extern "C" {
    pub fn vm_get_page_prot(_arg: vm_flags) -> return;
}
extern "C" {
    pub fn vma_flags_to_page_prot(_arg: vma->flags) -> return;
}
extern "C" {
    pub fn vma_set_page_prot(vma: *mut vm_area_struct);
}

extern "C" {
    pub fn __pgprot(_arg: 0) -> return;
}
extern "C" {
    pub fn __pgprot(_arg: 0) -> return;
}
extern "C" {
    pub fn __pgprot(_arg: 0) -> return;
}

extern "C" {
    pub fn vma_set_file(vma: *mut vm_area_struct, file: *mut file);
}

extern "C" {
    pub fn vm_insert_page(: *mut vm_area_struct, addr: c_ulong, : *mut page) -> c_int;
}
extern "C" {
    pub fn map_kernel_pages_prepare(desc: *mut vm_area_desc) -> c_int;
}
extern "C" {
    pub fn vm_iomap_memory(vma: *mut vm_area_struct, start: phys_addr_t, len: c_ulong) -> c_int;
}

extern "C" {
    pub fn remap_pfn_range(_arg: vma, _arg: addr, _arg: pfn, _arg: size, _arg: prot) -> return;
}
//
// Convert errno to return value for ->page_mkwrite() calls.
//
// This should eventually be merged with vmf_error() above, but will need a
// careful audit of all vmf_error() callers.
//
// -ENOSPC, -EDQUOT, -EIO ...
//
// Indicates whether GUP can follow a PROT_NONE mapped page, or whether
// a (NUMA hinting or userfaultfd RWP) fault is required.
//
// VM_UFFD_RWP uses protnone as an access-tracking marker, not for
// NUMA hinting. GUP must always take a fault so the access is
// delivered to userfaultfd, regardless of FOLL_HONOR_NUMA_FAULT.
//
// Only do so while the VMA is accessible. If it has been made
// inaccessible (e.g. mprotect(PROT_NONE)), fall through to the guard
// below: forcing a fault there would loop, as handle_mm_fault() makes
// no progress on protnone in an inaccessible VMA, and the access is
// denied regardless of RWP anyway.
//
// If callers don't want to honor NUMA hinting faults, no need to
// determine if we would actually have to trigger a NUMA hinting fault.
//
// NUMA hinting faults don't apply in inaccessible (PROT_NONE) VMAs.
//
// Requiring a fault here even for inaccessible VMAs would mean that
// FOLL_FORCE cannot make any progress, because handle_mm_fault()
// refuses to process NUMA hinting faults in inaccessible VMAs.
//
extern "C" {
    pub fn int(pte: *mut *mut pte_fn_t)(pte_t, addr: c_ulong, data: *mut c_void) -> typedef;
}

extern "C" {
    pub fn __kernel_poison_pages(page: *mut page, numpages: c_int);
}
extern "C" {
    pub fn __kernel_unpoison_pages(page: *mut page, numpages: c_int);
}
//
// For use in fast paths after init_mem_debugging() has run, or when a
// false negative result is not harmful when called too early.
//
extern "C" {
    pub fn static_branch_unlikely(_arg: &_page_poisoning_enabled) -> return;
}

//
// For use in fast paths after mem_debugging_and_hardening_init() has run,
// or when a false negative result is not harmful when called too early.
//
extern "C" {
    pub fn static_branch_unlikely(_arg: &_debug_pagealloc_enabled) -> return;
}
//
// To support DEBUG_PAGEALLOC architecture must ensure that
// __kernel_map_pages() never fails
//
extern "C" {
    pub fn __kernel_map_pages(page: *mut page, numpages: c_int, enable: c_int);
}

extern "C" {
    pub fn static_branch_unlikely(_arg: &_debug_guardpage_enabled) -> return;
}
extern "C" {
    pub fn PageGuard(_arg: page) -> return;
}
extern "C" {
    pub fn __set_page_guard(zone: *mut zone, page: *mut page, order: c_uint) -> bool;
}
extern "C" {
    pub fn __set_page_guard(_arg: zone, _arg: page, _arg: order) -> return;
}
extern "C" {
    pub fn __clear_page_guard(zone: *mut zone, page: *mut page, order: c_uint);
}

//
// clear_pages() - clear a page range for kernel-internal use.
// @addr: start address
// @npages: number of pages
//
// Use clear_user_pages() instead when clearing a page range to be
// mapped to user space.
//
// Does absolutely no exception handling.
//
// Note that even though the clearing operation is preemptible, clear_pages()
// does not (and on architectures where it reduces to a few long-running
// instructions, might not be able to) call cond_resched() to check if
// rescheduling is required.
//
// When running under preemptible models this is not a problem. Under
// cooperatively scheduled models, however, the caller is expected to
// limit @npages to no more than PROCESS_PAGES_NON_PREEMPT_BATCH.
//

//
// The architecture defines clear_pages(), and we assume that it is
// generally "fast". So choose a batch size large enough to allow the processor
// headroom for optimizing the operation and yet small enough that we see
// reasonable preemption latency for when this optimization is not possible
// (ex. slow microarchitectures, memory bandwidth saturation.)
//
// With a value of 32MB and assuming a memory bandwidth of ~10GBps, this should
// result in worst case preemption latency of around 3ms when clearing pages.
//
// (See comment above clear_pages() for why preemption latency is a concern
// here.)
//

//
// The architecture does not provide a clear_pages() implementation. Assume
// that clear_page() -- which clear_pages() will fallback to -- is relatively
// slow and choose a small value for PROCESS_PAGES_NON_PREEMPT_BATCH.
//
pub const PROCESS_PAGES_NON_PREEMPT_BATCH: c_int = 1;

extern "C" {
    pub fn in_gate_area_no_mm(addr: c_ulong) -> c_int;
}
extern "C" {
    pub fn in_gate_area(mm: *mut mm_struct, addr: c_ulong) -> c_int;
}

extern "C" {
    pub fn process_shares_mm(p: *const task_struct, mm: *const mm_struct) -> bool;
}
extern "C" {
    pub fn drop_slab();
}

pub const randomize_va_space: c_int = 0;

extern "C" {
    pub fn arch_vma_name(vma: *mut vm_area_struct) -> *const c_char;
}

extern "C" {
    pub fn print_vma_addr(prefix: *mut c_char, rip: c_ulong);
}

extern "C" {
    pub fn section_map_size() -> c_ulong;
}
extern "C" {
    pub fn vmemmap_verify(: *mut pte_t, _arg: c_int, long: unsigned, long: unsigned);
}
extern "C" {
    pub fn vmemmap_populate_print_last();
}

// number of pfns from base where pfn_to_page() is valid

pub const VMEMMAP_RESERVE_NR: c_int = 2;

//
// For vmemmap optimization with DAX we need minimum 2 vmemmap
// pages. See layout diagram in Documentation/mm/vmemmap_dedup.rst
//
// If we don't have an architecture override, use the generic rule
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mf_flags {
    MF_COUNT_INCREASED = 1 << 0,
    MF_ACTION_REQUIRED = 1 << 1,
    MF_MUST_KILL = 1 << 2,
    MF_SOFT_OFFLINE = 1 << 3,
    MF_UNPOISON = 1 << 4,
    MF_SW_SIMULATED = 1 << 5,
    MF_NO_RETRY = 1 << 6,
    MF_MEM_PRE_REMOVE = 1 << 7,
}

extern "C" {
    pub fn memory_failure(pfn: c_ulong, flags: c_int) -> c_int;
}
extern "C" {
    pub fn unpoison_memory(pfn: c_ulong) -> c_int;
}
extern "C" {
    pub fn soft_offline_page(pfn: c_ulong, flags: c_int) -> c_int;
}

//
// Sysfs entries for memory failure handling statistics.
//
extern "C" {
    pub fn memory_failure_queue(pfn: c_ulong, flags: c_int);
}
extern "C" {
    pub fn num_poisoned_pages_inc(pfn: c_ulong);
}
extern "C" {
    pub fn num_poisoned_pages_sub(pfn: c_ulong, i: c_long);
}

extern "C" {
    pub fn memblk_nr_poison_inc(pfn: c_ulong);
}
extern "C" {
    pub fn memblk_nr_poison_sub(pfn: c_ulong, i: c_long);
}

//
// Error handlers for various types of pages.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mf_result {
    MF_IGNORED,	/* Error: cannot be handled */
    MF_FAILED,	/* Error: handling failed */
    MF_DELAYED,	/* Will be handled later */
    MF_RECOVERED,	/* Successfully recovered */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mf_action_page_type {
    MF_MSG_KERNEL,
    MF_MSG_KERNEL_HIGH_ORDER,
    MF_MSG_DIFFERENT_COMPOUND,
    MF_MSG_HUGE,
    MF_MSG_FREE_HUGE,
    MF_MSG_GET_HWPOISON,
    MF_MSG_UNMAP_FAILED,
    MF_MSG_DIRTY_SWAPCACHE,
    MF_MSG_CLEAN_SWAPCACHE,
    MF_MSG_DIRTY_MLOCKED_LRU,
    MF_MSG_CLEAN_MLOCKED_LRU,
    MF_MSG_DIRTY_UNEVICTABLE_LRU,
    MF_MSG_CLEAN_UNEVICTABLE_LRU,
    MF_MSG_DIRTY_LRU,
    MF_MSG_CLEAN_LRU,
    MF_MSG_TRUNCATED_LRU,
    MF_MSG_BUDDY,
    MF_MSG_DAX,
    MF_MSG_UNSPLIT_THP,
    MF_MSG_ALREADY_POISONED,
    MF_MSG_PFN_MAP,
    MF_MSG_UNKNOWN,
}

extern "C" {
    pub fn folio_zero_user(folio: *mut folio, addr_hint: c_ulong);
}

extern "C" {
    pub fn setup_nr_node_ids() -> void __init;
}

extern "C" {
    pub fn memcmp_pages(page1: *mut page, page2: *mut page) -> c_int;
}

extern "C" {
    pub fn range_contains_unaccepted_memory(start: phys_addr_t, size: c_ulong) -> bool;
}
extern "C" {
    pub fn accept_memory(start: phys_addr_t, size: c_ulong);
}

extern "C" {
    pub fn range_contains_unaccepted_memory(PAGE_SHIFT: pfn <<, _arg: PAGE_SIZE) -> return;
}
extern "C" {
    pub fn vma_pgtable_walk_begin(vma: *mut vm_area_struct);
}
extern "C" {
    pub fn vma_pgtable_walk_end(vma: *mut vm_area_struct);
}
extern "C" {
    pub fn reserve_mem_find_by_name(name: *const c_char, start: *mut phys_addr_t, size: *mut phys_addr_t) -> c_int;
}
extern "C" {
    pub fn reserve_mem_release_by_name(name: *const c_char) -> c_int;
}

extern "C" {
    pub fn mseal_mmap_page_zero();
}

//
// user_alloc_needs_zeroing checks if a user folio from page allocator needs to
// be zeroed or not.
//
// for user folios, arch with cache aliasing requires cache flush and
// arc changes folio->flags to make icache coherent with dcache, so
// always return false to make caller use
// clear_user_page()/clear_user_highpage().
//
extern "C" {
    pub fn arch_get_shadow_stack_status(t: *mut task_struct, status: *mut unsigned long __user) -> c_int;
}
extern "C" {
    pub fn arch_set_shadow_stack_status(t: *mut task_struct, status: c_ulong) -> c_int;
}
extern "C" {
    pub fn arch_lock_shadow_stack_status(t: *mut task_struct, status: c_ulong) -> c_int;
}
//
// DMA mapping IDs for page_pool
//
// When DMA-mapping a page, page_pool allocates an ID (from an xarray) and
// stashes it in the upper bits of page->pp_magic. We always want to be able to
// unambiguously identify page pool pages (using page_pool_page_is_pp()). Non-PP
// pages can have arbitrary kernel pointers stored in the same field as pp_magic
// (since it overlaps with page->lru.next), so we must ensure that we cannot
// mistake a valid kernel pointer with any of the values we write into this
// field.
//
// On architectures that set POISON_POINTER_DELTA, this is already ensured,
// since this value becomes part of PP_SIGNATURE; meaning we can just use the
// space between the PP_SIGNATURE value (without POISON_POINTER_DELTA), and the
// lowest bits of POISON_POINTER_DELTA. On arches where POISON_POINTER_DELTA is
// 0, we use the lowest bit of PAGE_OFFSET as the boundary if that value is
// known at compile-time.
//
// If the value of PAGE_OFFSET is not known at compile time, or if it is too
// small to leave at least 8 bits available above PP_SIGNATURE, we define the
// number of bits to be 0, which turns off the DMA index tracking altogether
// (see page_pool_register_dma_index()).
//

// PP_SIGNATURE includes POISON_POINTER_DELTA, so limit the size of the DMA
// index to not overlap with that if set
//

// Use the lowest bit of PAGE_OFFSET if there's at least 8 bits available; see above

// Mask used for checking in page_pool_page_is_pp() below. page->pp_magic is
// OR'ed with PP_SIGNATURE after the allocation in order to preserve bit 0 for
// the head page of compound page and bit 1 for pfmemalloc page, as well as the
// bits used for the DMA index. page_is_pfmemalloc() is checked in
// __page_pool_put_page() to avoid recycling the pfmemalloc page.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct page_snapshot {
    pub folio_snapshot: folio,
    pub page_snapshot: page,
    pub pfn: c_ulong,
    pub idx: c_ulong,
    pub flags: c_ulong,
}

extern "C" {
    pub fn snapshot_page(ps: *mut page_snapshot, page: *const page);
}
