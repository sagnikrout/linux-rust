//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/kvm_pgtable.h
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
// Copyright (C) 2020 Google LLC
// Author: Will Deacon <will@kernel.org>
//

pub const KVM_PGTABLE_LAST_LEVEL: c_int = 3;
//
// The largest supported block sizes for KVM (no 52-bit PA support):
// - 4K (level 1):	1GB
// - 16K (level 2):	32MB
// - 64K (level 2):	512MB
//

pub const KVM_PGTABLE_MIN_BLOCK_LEVEL: c_int = 1;

pub const KVM_PGTABLE_MIN_BLOCK_LEVEL: c_int = 2;

pub type kvm_pte_t = u64;

pub const KVM_PTE_TYPE_BLOCK: c_int = 0;
pub const KVM_PTE_TYPE_PAGE: c_int = 1;
pub const KVM_PTE_TYPE_TABLE: c_int = 1;

pub const KVM_PTE_LEAF_ATTR_LO_S1_SH_IS: c_int = 3;

pub const KVM_PTE_LEAF_ATTR_LO_S2_SH_IS: c_int = 3;

// pKVM invalid pte encodings

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kvm_invalid_pte_type {
//
// Used to indicate a pte for which a 'break-before-make'
// sequence is in progress.
//
    KVM_INVALID_PTE_TYPE_LOCKED	= 1,

//
// pKVM has unmapped the page from the host due to a change of
// ownership.
//
    KVM_HOST_INVALID_PTE_TYPE_DONATION,

//
// The page has been forcefully reclaimed from the guest by the
// host.
//
    KVM_GUEST_INVALID_PTE_TYPE_POISONED,
}

extern "C" {
    pub fn __phys_to_pfn(_arg: kvm_pte_to_phys(pte)) -> return;
}
// Assumes KVM_PGTABLE_LAST_LEVEL is 3
extern "C" {
    pub fn ARM64_HW_PGTABLE_LEVEL_SHIFT(_arg: level) -> return;
}
extern "C" {
    pub fn BIT(_arg: kvm_granule_shift(level)) -> return;
}
//
// struct kvm_pgtable_mm_ops - Memory management callbacks.
// @zalloc_page:		Allocate a single zeroed memory page.
// The @arg parameter can be used by the walker
// to pass a memcache. The initial refcount of
// the page is 1.
// @zalloc_pages_exact:		Allocate an exact number of zeroed memory pages.
// The @size parameter is in bytes, and is rounded
// up to the next page boundary. The resulting
// allocation is physically contiguous.
// @free_pages_exact:		Free an exact number of memory pages previously
// allocated by zalloc_pages_exact.
// @free_unlinked_table:	Free an unlinked paging structure by unlinking and
// dropping references.
// @get_page:			Increment the refcount on a page.
// @put_page:			Decrement the refcount on a page. When the
// refcount reaches 0 the page is automatically
// freed.
// @page_count:			Return the refcount of a page.
// @phys_to_virt:		Convert a physical address into a virtual
// address	mapped in the current context.
// @virt_to_phys:		Convert a virtual address mapped in the current
// context into a physical address.
// @dcache_clean_inval_poc:	Clean and invalidate the data cache to the PoC
// for the	specified memory address range.
// @icache_inval_pou:		Invalidate the instruction cache to the PoU
// for the specified memory address range.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_pgtable_mm_ops {
    pub arg): *mut *mut *mut void (zalloc_page)(void,
    pub size): *mut *mut *mut void (zalloc_pages_exact)(size_t,
    pub size): *mut *mut *mut void (free_pages_exact)(void addr, size_t,
    pub level): *mut *mut *mut void (free_unlinked_table)(void addr, s8,
    pub addr): *mut *mut void (get_page)(void,
    pub addr): *mut *mut void (put_page)(void,
    pub addr): *mut *mut int (page_count)(void,
    pub phys): *mut *mut *mut void (phys_to_virt)(phys_addr_t,
    pub addr): *mut *mut phys_addr_t (virt_to_phys)(void,
    pub size): *mut *mut *mut void (dcache_clean_inval_poc)(void addr, size_t,
    pub size): *mut *mut *mut void (icache_inval_pou)(void addr, size_t,
}

//
// enum kvm_pgtable_stage2_flags - Stage-2 page-table flags.
// @KVM_PGTABLE_S2_IDMAP:	Only use identity mappings.
// @KVM_PGTABLE_S2_AS_S1:	Final memory attributes are that of Stage-1.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kvm_pgtable_stage2_flags {
    KVM_PGTABLE_S2_IDMAP			= BIT(0),
    KVM_PGTABLE_S2_AS_S1			= BIT(1),
}

//
// enum kvm_pgtable_prot - Page-table permissions and attributes.
// @KVM_PGTABLE_PROT_UX:	Unprivileged execute permission.
// @KVM_PGTABLE_PROT_PX:	Privileged execute permission.
// @KVM_PGTABLE_PROT_X:		Privileged and unprivileged execute permission.
// @KVM_PGTABLE_PROT_W:		Write permission.
// @KVM_PGTABLE_PROT_R:		Read permission.
// @KVM_PGTABLE_PROT_DEVICE:	Device attributes.
// @KVM_PGTABLE_PROT_NORMAL_NC:	Normal noncacheable attributes.
// @KVM_PGTABLE_PROT_SW0:	Software bit 0.
// @KVM_PGTABLE_PROT_SW1:	Software bit 1.
// @KVM_PGTABLE_PROT_SW2:	Software bit 2.
// @KVM_PGTABLE_PROT_SW3:	Software bit 3.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kvm_pgtable_prot {
    KVM_PGTABLE_PROT_PX			= BIT(0),
    KVM_PGTABLE_PROT_UX			= BIT(1),
    KVM_PGTABLE_PROT_X			= KVM_PGTABLE_PROT_PX	|
    KVM_PGTABLE_PROT_UX,
    KVM_PGTABLE_PROT_W			= BIT(2),
    KVM_PGTABLE_PROT_R			= BIT(3),

    KVM_PGTABLE_PROT_DEVICE			= BIT(4),
    KVM_PGTABLE_PROT_NORMAL_NC		= BIT(5),

    KVM_PGTABLE_PROT_SW0			= BIT(55),
    KVM_PGTABLE_PROT_SW1			= BIT(56),
    KVM_PGTABLE_PROT_SW2			= BIT(57),
    KVM_PGTABLE_PROT_SW3			= BIT(58),
}

//
// enum kvm_pgtable_walk_flags - Flags to control a depth-first page-table walk.
// @KVM_PGTABLE_WALK_LEAF:		Visit leaf entries, including invalid
// entries.
// @KVM_PGTABLE_WALK_TABLE_PRE:		Visit table entries before their
// children.
// @KVM_PGTABLE_WALK_TABLE_POST:	Visit table entries after their
// children.
// @KVM_PGTABLE_WALK_SHARED:		Indicates the page-tables may be shared
// with other software walkers.
// @KVM_PGTABLE_WALK_IGNORE_EAGAIN:	Don't terminate the walk early if
// the walker returns -EAGAIN.
// @KVM_PGTABLE_WALK_SKIP_BBM_TLBI:	Visit and update table entries
// without Break-before-make's
// TLB invalidation.
// @KVM_PGTABLE_WALK_SKIP_CMO:		Visit and update table entries
// without Cache maintenance
// operations required.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kvm_pgtable_walk_flags {
    KVM_PGTABLE_WALK_LEAF			= BIT(0),
    KVM_PGTABLE_WALK_TABLE_PRE		= BIT(1),
    KVM_PGTABLE_WALK_TABLE_POST		= BIT(2),
    KVM_PGTABLE_WALK_SHARED			= BIT(3),
    KVM_PGTABLE_WALK_IGNORE_EAGAIN		= BIT(4),
    KVM_PGTABLE_WALK_SKIP_BBM_TLBI		= BIT(5),
    KVM_PGTABLE_WALK_SKIP_CMO		= BIT(6),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_pgtable_visit_ctx {
    pub ptep: *mut kvm_pte_t,
    pub old: kvm_pte_t,
    pub arg: *mut c_void,
    pub mm_ops: *mut kvm_pgtable_mm_ops,
    pub start: u64,
    pub addr: u64,
    pub end: u64,
    pub level: i8,
    pub flags: kvm_pgtable_walk_flags,
}

//
// struct kvm_pgtable_walker - Hook into a page-table walk.
// @cb:		Callback function to invoke during the walk.
// @arg:	Argument passed to the callback function.
// @flags:	Bitwise-OR of flags to identify the entry types on which to
// invoke the callback function.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_pgtable_walker {
    pub cb: kvm_pgtable_visitor_fn_t,
    pub arg: *const *const c_void,
    pub flags: kvm_pgtable_walk_flags,
}

//
// RCU cannot be used in a non-kernel context such as the hyp. As such, page
// table walkers used in hyp do not call into RCU and instead use other
// synchronization mechanisms (such as a spinlock).
//

//
// Due to the lack of RCU (or a similar protection scheme), only
// non-shared table walkers are allowed in the hypervisor.
//

extern "C" {
    pub fn rcu_dereference_check(_arg: pteref, KVM_PGTABLE_WALK_SHARED): !(walker->flags &) -> return;
}
extern "C" {
    pub fn rcu_dereference_raw(_arg: pteref) -> return;
}
extern "C" {
    pub fn rcu_read_lock_held() -> return;
}

//
// struct kvm_pgtable - KVM page-table.
// @ia_bits:		Maximum input address size, in bits.
// @start_level:	Level at which the page-table walk starts.
// @pgd:		Pointer to the first top-level entry of the page-table.
// @mm_ops:		Memory management callbacks.
// @mmu:		Stage-2 KVM MMU struct. Unused for stage-1 page-tables.
// @flags:		Stage-2 page-table flags.
// @force_pte_cb:	Function that returns true if page level mappings must
// be used instead of block mappings.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_pgtable {
    pub pkvm_mappings: rb_root_cached,
    pub ia_bits: u32,
    pub start_level: i8,
    pub pgd: kvm_pteref_t,
    pub mm_ops: *mut kvm_pgtable_mm_ops,
// Stage-2 only
    pub flags: kvm_pgtable_stage2_flags,
    pub force_pte_cb: kvm_pgtable_force_pte_cb_t,
}

//
// kvm_pgtable_hyp_init() - Initialise a hypervisor stage-1 page-table.
// @pgt:	Uninitialised page-table structure to initialise.
// @va_bits:	Maximum virtual address bits.
// @mm_ops:	Memory management callbacks.
//
// Return: 0 on success, negative error code on failure.
//
// kvm_pgtable_hyp_destroy() - Destroy an unused hypervisor stage-1 page-table.
// @pgt:	Page-table structure initialised by kvm_pgtable_hyp_init().
//
// The page-table is assumed to be unreachable by any hardware walkers prior
// to freeing and therefore no TLB invalidation is performed.
//
extern "C" {
    pub fn kvm_pgtable_hyp_destroy(pgt: *mut kvm_pgtable);
}
//
// kvm_pgtable_hyp_map() - Install a mapping in a hypervisor stage-1 page-table.
// @pgt:	Page-table structure initialised by kvm_pgtable_hyp_init().
// @addr:	Virtual address at which to place the mapping.
// @size:	Size of the mapping.
// @phys:	Physical address of the memory to map.
// @prot:	Permissions and attributes for the mapping.
//
// The offset of @addr within a page is ignored, @size is rounded-up to
// the next page boundary and @phys is rounded-down to the previous page
// boundary.
//
// If device attributes are not explicitly requested in @prot, then the
// mapping will be normal, cacheable. Attempts to install a new mapping
// for a virtual address that is already mapped will be rejected with an
// error and a WARN().
//
// Return: 0 on success, negative error code on failure.
//
// kvm_pgtable_hyp_unmap() - Remove a mapping from a hypervisor stage-1 page-table.
// @pgt:	Page-table structure initialised by kvm_pgtable_hyp_init().
// @addr:	Virtual address from which to remove the mapping.
// @size:	Size of the mapping.
//
// The offset of @addr within a page is ignored, @size is rounded-up to
// the next page boundary and @phys is rounded-down to the previous page
// boundary.
//
// TLB invalidation is performed for each page-table entry cleared during the
// unmapping operation and the reference count for the page-table page
// containing the cleared entry is decremented, with unreferenced pages being
// freed. The unmapping operation will stop early if it encounters either an
// invalid page-table entry or a valid block mapping which maps beyond the range
// being unmapped.
//
// Return: Number of bytes unmapped, which may be 0.
//
extern "C" {
    pub fn kvm_pgtable_hyp_unmap(pgt: *mut kvm_pgtable, addr: u64, size: u64) -> u64;
}
//
// kvm_get_vtcr() - Helper to construct VTCR_EL2
// @mmfr0:	Sanitized value of SYS_ID_AA64MMFR0_EL1 register.
// @mmfr1:	Sanitized value of SYS_ID_AA64MMFR1_EL1 register.
// @phys_shfit:	Value to set in VTCR_EL2.T0SZ.
//
// The VTCR value is common across all the physical CPUs on the system.
// We use system wide sanitised values to fill in different fields,
// except for Hardware Management of Access Flags. HA Flag is set
// unconditionally on all CPUs, as it is safe to run with or without
// the feature and the bit is RES0 on CPUs that don't support it.
//
// Return: VTCR_EL2 value
//
extern "C" {
    pub fn kvm_get_vtcr(mmfr0: u64, mmfr1: u64, phys_shift: u32) -> u64;
}
//
// kvm_pgtable_stage2_pgd_size() - Helper to compute size of a stage-2 PGD
// @vtcr:	Content of the VTCR register.
//
// Return: the size (in bytes) of the stage-2 PGD
//
extern "C" {
    pub fn kvm_pgtable_stage2_pgd_size(vtcr: u64) -> usize;
}
//
// __kvm_pgtable_stage2_init() - Initialise a guest stage-2 page-table.
// @pgt:	Uninitialised page-table structure to initialise.
// @mmu:	S2 MMU context for this S2 translation
// @mm_ops:	Memory management callbacks.
// @flags:	Stage-2 configuration flags.
// @force_pte_cb: Function that returns true if page level mappings must
// be used instead of block mappings.
//
// Return: 0 on success, negative error code on failure.
//
extern "C" {
    pub fn __kvm_pgtable_stage2_init(_arg: pgt, _arg: mmu, _arg: mm_ops, _arg: 0, _arg: NULL) -> return;
}
//
// kvm_pgtable_stage2_destroy() - Destroy an unused guest stage-2 page-table.
// @pgt:	Page-table structure initialised by kvm_pgtable_stage2_init*().
//
// The page-table is assumed to be unreachable by any hardware walkers prior
// to freeing and therefore no TLB invalidation is performed.
//
extern "C" {
    pub fn kvm_pgtable_stage2_destroy(pgt: *mut kvm_pgtable);
}
//
// kvm_pgtable_stage2_destroy_range() - Destroy the unlinked range of addresses.
// @pgt:	Page-table structure initialised by kvm_pgtable_stage2_init*().
// @addr:      Intermediate physical address at which to place the mapping.
// @size:      Size of the mapping.
//
// The page-table is assumed to be unreachable by any hardware walkers prior
// to freeing and therefore no TLB invalidation is performed.
//
// kvm_pgtable_stage2_destroy_pgd() - Destroy the PGD of guest stage-2 page-table.
// @pgt:       Page-table structure initialised by kvm_pgtable_stage2_init*().
//
// It is assumed that the rest of the page-table is freed before this operation.
//
extern "C" {
    pub fn kvm_pgtable_stage2_destroy_pgd(pgt: *mut kvm_pgtable);
}
//
// kvm_pgtable_stage2_free_unlinked() - Free an unlinked stage-2 paging structure.
// @mm_ops:	Memory management callbacks.
// @pgtable:	Unlinked stage-2 paging structure to be freed.
// @level:	Level of the stage-2 paging structure to be freed.
//
// The page-table is assumed to be unreachable by any hardware walkers prior to
// freeing and therefore no TLB invalidation is performed.
//
extern "C" {
    pub fn kvm_pgtable_stage2_free_unlinked(mm_ops: *mut kvm_pgtable_mm_ops, pgtable: *mut c_void, level: i8);
}
//
// kvm_pgtable_stage2_create_unlinked() - Create an unlinked stage-2 paging structure.
// @pgt:	Page-table structure initialised by kvm_pgtable_stage2_init*().
// @phys:	Physical address of the memory to map.
// @level:	Starting level of the stage-2 paging structure to be created.
// @prot:	Permissions and attributes for the mapping.
// @mc:		Cache of pre-allocated and zeroed memory from which to allocate
// page-table pages.
// @force_pte:  Force mappings to PAGE_SIZE granularity.
//
// Returns an unlinked page-table tree.  This new page-table tree is
// not reachable (i.e., it is unlinked) from the root pgd and it's
// therefore unreachableby the hardware page-table walker. No TLB
// invalidation or CMOs are performed.
//
// If device attributes are not explicitly requested in @prot, then the
// mapping will be normal, cacheable.
//
// Return: The fully populated (unlinked) stage-2 paging structure, or
// an ERR_PTR(error) on failure.
//
// kvm_pgtable_stage2_map() - Install a mapping in a guest stage-2 page-table.
// @pgt:	Page-table structure initialised by kvm_pgtable_stage2_init*().
// @addr:	Intermediate physical address at which to place the mapping.
// @size:	Size of the mapping.
// @phys:	Physical address of the memory to map.
// @prot:	Permissions and attributes for the mapping.
// @mc:		Cache of pre-allocated and zeroed memory from which to allocate
// page-table pages.
// @flags:	Flags to control the page-table walk (ex. a shared walk)
//
// The offset of @addr within a page is ignored, @size is rounded-up to
// the next page boundary and @phys is rounded-down to the previous page
// boundary.
//
// If device attributes are not explicitly requested in @prot, then the
// mapping will be normal, cacheable.
//
// Note that the update of a valid leaf PTE in this function will be aborted,
// if it's trying to recreate the exact same mapping or only change the access
// permissions. Instead, the vCPU will exit one more time from guest if still
// needed and then go through the path of relaxing permissions.
//
// Note that this function will both coalesce existing table entries and split
// existing block mappings, relying on page-faults to fault back areas outside
// of the new mapping lazily.
//
// Return: 0 on success, negative error code on failure.
//
// kvm_pgtable_stage2_annotate() - Unmap and annotate pages in the IPA space
// to track ownership (and more).
// @pgt:	Page-table structure initialised by kvm_pgtable_stage2_init*().
// @addr:	Base intermediate physical address to annotate.
// @size:	Size of the annotated range.
// @mc:		Cache of pre-allocated and zeroed memory from which to allocate
// page-table pages.
// @type:	The type of the annotation, determining its meaning and format.
// @annotation:	A 59-bit value that will be stored in the page tables.
// @annotation[0] and @annotation[63:60] must be 0.
// @annotation[59:1] is stored in the page tables, along
// with @type.
//
// By default, all page-tables are owned by identifier 0. This function can be
// used to mark portions of the IPA space as owned by other entities. When a
// stage 2 is used with identity-mappings, these annotations allow to use the
// page-table data structure as a simple rmap.
//
// Return: 0 on success, negative error code on failure.
//
// kvm_pgtable_stage2_unmap() - Remove a mapping from a guest stage-2 page-table.
// @pgt:	Page-table structure initialised by kvm_pgtable_stage2_init*().
// @addr:	Intermediate physical address from which to remove the mapping.
// @size:	Size of the mapping.
//
// The offset of @addr within a page is ignored and @size is rounded-up to
// the next page boundary.
//
// TLB invalidation is performed for each page-table entry cleared during the
// unmapping operation and the reference count for the page-table page
// containing the cleared entry is decremented, with unreferenced pages being
// freed. Unmapping a cacheable page will ensure that it is clean to the PoC if
// FWB is not supported by the CPU.
//
// Return: 0 on success, negative error code on failure.
//
extern "C" {
    pub fn kvm_pgtable_stage2_unmap(pgt: *mut kvm_pgtable, addr: u64, size: u64) -> c_int;
}
//
// kvm_pgtable_stage2_wrprotect() - Write-protect guest stage-2 address range
// without TLB invalidation.
// @pgt:	Page-table structure initialised by kvm_pgtable_stage2_init*().
// @addr:	Intermediate physical address from which to write-protect,
// @size:	Size of the range.
//
// The offset of @addr within a page is ignored and @size is rounded-up to
// the next page boundary.
//
// Note that it is the caller's responsibility to invalidate the TLB after
// calling this function to ensure that the updated permissions are visible
// to the CPUs.
//
// Return: 0 on success, negative error code on failure.
//
extern "C" {
    pub fn kvm_pgtable_stage2_wrprotect(pgt: *mut kvm_pgtable, addr: u64, size: u64) -> c_int;
}
//
// kvm_pgtable_stage2_mkyoung() - Set the access flag in a page-table entry.
// @pgt:	Page-table structure initialised by kvm_pgtable_stage2_init*().
// @addr:	Intermediate physical address to identify the page-table entry.
// @flags:	Flags to control the page-table walk (ex. a shared walk)
//
// The offset of @addr within a page is ignored.
//
// If there is a valid, leaf page-table entry used to translate @addr, then
// set the access flag in that entry.
//
// kvm_pgtable_stage2_test_clear_young() - Test and optionally clear the access
// flag in a page-table entry.
// @pgt:	Page-table structure initialised by kvm_pgtable_stage2_init*().
// @addr:	Intermediate physical address to identify the page-table entry.
// @size:	Size of the address range to visit.
// @mkold:	True if the access flag should be cleared.
//
// The offset of @addr within a page is ignored.
//
// Tests and conditionally clears the access flag for every valid, leaf
// page-table entry used to translate the range [@addr, @addr + @size).
//
// Note that it is the caller's responsibility to invalidate the TLB after
// calling this function to ensure that the updated permissions are visible
// to the CPUs.
//
// Return: True if any of the visited PTEs had the access flag set.
//
// kvm_pgtable_stage2_relax_perms() - Relax the permissions enforced by a
// page-table entry.
// @pgt:	Page-table structure initialised by kvm_pgtable_stage2_init*().
// @addr:	Intermediate physical address to identify the page-table entry.
// @prot:	Additional permissions to grant for the mapping.
// @flags:	Flags to control the page-table walk (ex. a shared walk)
//
// The offset of @addr within a page is ignored.
//
// If there is a valid, leaf page-table entry used to translate @addr, then
// relax the permissions in that entry according to the read, write and
// execute permissions specified by @prot. No permissions are removed, and
// TLB invalidation is performed after updating the entry. Software bits cannot
// be set or cleared using kvm_pgtable_stage2_relax_perms().
//
// Return: 0 on success, negative error code on failure.
//
// kvm_pgtable_stage2_flush_range() - Clean and invalidate data cache to Point
// of Coherency for guest stage-2 address
// range.
// @pgt:	Page-table structure initialised by kvm_pgtable_stage2_init*().
// @addr:	Intermediate physical address from which to flush.
// @size:	Size of the range.
//
// The offset of @addr within a page is ignored and @size is rounded-up to
// the next page boundary.
//
// Return: 0 on success, negative error code on failure.
//
extern "C" {
    pub fn kvm_pgtable_stage2_flush(pgt: *mut kvm_pgtable, addr: u64, size: u64) -> c_int;
}
//
// kvm_pgtable_stage2_split() - Split a range of huge pages into leaf PTEs pointing
// to PAGE_SIZE guest pages.
// @pgt:	 Page-table structure initialised by kvm_pgtable_stage2_init().
// @addr:	 Intermediate physical address from which to split.
// @size:	 Size of the range.
// @mc:		 Cache of pre-allocated and zeroed memory from which to allocate
// page-table pages.
//
// The function tries to split any level 1 or 2 entry that overlaps
// with the input range (given by @addr and @size).
//
// Return: 0 on success, negative error code on failure. Note that
// kvm_pgtable_stage2_split() is best effort: it tries to break as many
// blocks in the input range as allowed by @mc_capacity.
//
// kvm_pgtable_walk() - Walk a page-table.
// @pgt:	Page-table structure initialised by kvm_pgtable_*_init().
// @addr:	Input address for the start of the walk.
// @size:	Size of the range to walk.
// @walker:	Walker callback description.
//
// The offset of @addr within a page is ignored and @size is rounded-up to
// the next page boundary.
//
// The walker will walk the page-table entries corresponding to the input
// address range specified, visiting entries according to the walker flags.
// Invalid entries are treated as leaf entries. The visited page table entry is
// reloaded after invoking the walker callback, allowing the walker to descend
// into a newly installed table.
//
// Returning a negative error code from the walker callback function will
// terminate the walk immediately with the same error code.
//
// Return: 0 on success, negative error code on failure.
//
// kvm_pgtable_get_leaf() - Walk a page-table and retrieve the leaf entry
// with its level.
// @pgt:	Page-table structure initialised by kvm_pgtable_*_init()
// or a similar initialiser.
// @addr:	Input address for the start of the walk.
// @ptep:	Pointer to storage for the retrieved PTE.
// @level:	Pointer to storage for the level of the retrieved PTE.
//
// The offset of @addr within a page is ignored.
//
// The walker will walk the page-table entries corresponding to the input
// address specified, retrieving the leaf corresponding to this address.
// Invalid entries are treated as leaf entries.
//
// Return: 0 on success, negative error code on failure.
//
// kvm_pgtable_stage2_pte_prot() - Retrieve the protection attributes of a
// stage-2 Page-Table Entry.
// @pte:	Page-table entry
//
// Return: protection attributes of the page-table entry in the enum
// kvm_pgtable_prot format.
//
extern "C" {
    pub fn kvm_pgtable_stage2_pte_prot(pte: kvm_pte_t) -> kvm_pgtable_prot;
}
//
// kvm_pgtable_hyp_pte_prot() - Retrieve the protection attributes of a stage-1
// Page-Table Entry.
// @pte:	Page-table entry
//
// Return: protection attributes of the page-table entry in the enum
// kvm_pgtable_prot format.
//
extern "C" {
    pub fn kvm_pgtable_hyp_pte_prot(pte: kvm_pte_t) -> kvm_pgtable_prot;
}
//
// kvm_tlb_flush_vmid_range() - Invalidate/flush a range of TLB entries
//
// @mmu:	Stage-2 KVM MMU struct
// @addr:	The base Intermediate physical address from which to invalidate
// @size:	Size of the range from the base to invalidate
//
