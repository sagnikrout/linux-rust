//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/vma/include/dup.h
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


// SPDX-License-Identifier: GPL-2.0+

// Forward declarations to avoid header cycle.
extern "C" {
    pub fn vma_start_write(vma: *mut vm_area_struct);
}
extern "C" {
    pub fn rlimit(limit: c_uint) -> c_ulong;
}
pub const MMF_HAS_MDWE: c_int = 28;

//
// Define the task command name length as enum, then it can be visible to
// BPF programs.
//
// PARTIALLY implemented types.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mm_struct {
    pub mm_mt: maple_tree,
    pub /: *mut *mut int map_count; / number of VMAs,
    pub /: *mut *mut unsigned long total_vm; / Total pages mapped,
    pub /: *mut *mut unsigned long locked_vm; / Pages that have PG_mlocked set,
    pub /: *mut *mut unsigned long data_vm; / VM_WRITE & ~VM_SHARED & ~VM_STACK,
    pub /: *mut *mut unsigned long exec_vm; / VM_EXEC & ~VM_WRITE & ~VM_STACK,
    pub /: *mut *mut unsigned long stack_vm; / VM_STACK,
    pub def_flags: vm_flags_t,
    pub def_vma_flags: vma_flags_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct address_space {
    pub i_mmap: rb_root_cached,
    pub flags: c_ulong,
    pub i_mmap_writable: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct file_operations {
    pub ): *mut *mut *mut int (mmap)(struct file , struct vm_area_struct,
    pub ): *mut *mut int (mmap_prepare)(struct vm_area_desc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct file {
    pub f_mapping: *mut address_space,
    pub f_op: *const file_operations,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct anon_vma_chain {
    pub anon_vma: *mut anon_vma,
    pub same_vma: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_struct {
    pub comm: [c_char; TASK_COMM_LEN],
    pub pid: pid_t,
    pub mm: *mut mm_struct,
// Used for emulating ABI behavior of previous Linux versions:
    pub personality: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kref {
    pub refcount: refcount_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct anon_vma_name {
    pub kref: kref,
// The name needs to be at the end because it is dynamically sized.
    pub name: [c_char; ],
}

//
// Contains declarations that are DUPLICATED from kernel source in order to
// faciliate userland VMA testing.
//
// These must be kept in sync with kernel source.
//
pub const VMA_LOCK_OFFSET: c_uint = 0x40000000;
pub const VM_NONE: c_uint = 0x00000000;
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

// Bits set in the VMA until the stack is in its final location

// Common data flag combinations

// Temporary until VMA flags conversion complete.

// VMA basic access permission flags

//
// Special vmas that are non-mergable, non-mlock()able.
//

// This mask represents all the VMA flag bits used by mlock

pub const CAP_IPC_LOCK: c_int = 14;

// The MM code likes to work with exclusive end addresses

pub const AS_MM_ALL_LOCKS: c_int = 2;

//
// Flags for bug emulation.
//
// These occupy the top three bytes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vma_iterator {
    pub mas: ma_state,
}

extern "C" {
    pub fn bitmap_empty(_arg: bitmap, _arg: NUM_VMA_FLAG_BITS) -> return;
}
// What action should be taken after an .mmap_prepare call is complete?
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mmap_action_type {
    MMAP_NOTHING,		/* Mapping is complete, no further action. */
    MMAP_REMAP_PFN,		/* Remap PFN range. */
    MMAP_IO_REMAP_PFN,	/* I/O remap PFN range. */
    MMAP_SIMPLE_IO_REMAP,	/* I/O remap with guardrails. */
    MMAP_MAP_KERNEL_PAGES,	/* Map kernel page range from an array. */
}

//
// Describes an action an mmap_prepare hook can instruct to be taken to complete
// the mapping of a VMA. Specified in vm_area_desc.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmap_action {
    pub start: c_ulong,
    pub start_pfn: c_ulong,
    pub size: c_ulong,
    pub pgprot: pgprot_t,
    pub remap: },
    pub start_phys_addr: phys_addr_t,
    pub size: c_ulong,
    pub simple_ioremap: },
    pub start: c_ulong,
    pub pages: *mut page,
    pub nr_pages: c_ulong,
    pub pgoff: pgoff_t,
    pub map_kernel: },
}

//
// If non-zero, replace errors that arise from mmap actions with this
// value instead. Only valid error codes may be specified.
//
// This should be set in rare instances where the operation required
// that the rmap should not be able to access the VMA until
// completely set up.
//
// Operations which modify VMAs.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vma_operation {
    VMA_OP_SPLIT,
    VMA_OP_MERGE_UNFAULTED,
    VMA_OP_REMAP,
    VMA_OP_FORK,
}

//
// Describes a VMA that is about to be mmap()'ed. Drivers may choose to
// manipulate mutable fields which will cause those fields to be updated in the
// resultant VMA.
//
// Helper functions are not required for manipulating any field.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vm_area_desc {
// Immutable state.
    pub mm: *mut mm_struct,
    pub /: *mut *mut *mut file file; / May vary from vm_file in stacked callers.,
    pub start: c_ulong,
    pub end: c_ulong,
// Mutable fields. Populated with initial state.
    pub pgoff: pgoff_t,
    pub vm_file: *mut file,
    pub vma_flags: vma_flags_t,
    pub page_prot: pgprot_t,
// Write-only fields.
    pub vm_ops: *const vm_operations_struct,
    pub private_data: *mut c_void,
// Take further action?
    pub action: mmap_action,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vm_area_struct {
// The first cache line has the info for VMA tree walking.
// VMA covers [vm_start; vm_end) addresses within mm
    pub vm_start: c_ulong,
    pub vm_end: c_ulong,
}

//
// Flags, see mm.h.
// To modify use vm_flags_{init|reset|set|clear|mod} functions.
//

//
// Can only be written (using WRITE_ONCE()) while holding both:
// - mmap_lock (in write mode)
// - vm_refcnt bit at VMA_LOCK_OFFSET is set
// Can be read reliably while holding one of:
// - mmap_lock (in read or write mode)
// - vm_refcnt bit at VMA_LOCK_OFFSET is set or vm_refcnt > 1
// Can be read unreliably (using READ_ONCE()) for pessimistic bailout
// while holding nothing (except RCU to keep the VMA struct allocated).
//
// This sequence counter is explicitly allowed to overflow; sequence
// counter reuse can only lead to occasional unnecessary use of the
// slowpath.
//

//
// A file's MAP_PRIVATE vma can be in both i_mmap tree and anon_vma
// list, after a COW of one of the file pages.	A MAP_SHARED vma
// can only be in the i_mmap tree.  An anonymous MAP_PRIVATE, stack
// or brk vma (with NULL file) can only be in an anon_vma list.
//
// page_table_lock
// Function pointers to deal with this struct.
// Information about our backing store:

// Unstable RCU readers are allowed to read this.

//
// For areas with an address space and backing store,
// linkage into the address_space->i_mmap interval tree.
//

//
// For private and shared anonymous mappings, a pointer to a null
// terminated string containing the name given to the vma, or NULL if
// unnamed. Serialized by mmap_lock. Use anon_vma_name to access.
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pagetable_move_control {
    pub /: *mut *mut *mut vm_area_old; / Source VMA.,
    pub /: *mut *mut *mut vm_area_new; / Destination VMA.,
    pub /: *mut *mut unsigned long old_addr; / Address from which the move begins.,
    pub /: *mut *mut unsigned long old_end; / Exclusive address at which old range ends.,
    pub /: *mut *mut unsigned long new_addr; / Address to move page tables to.,
    pub /: *mut *mut unsigned long len_in; / Bytes to remap specified by user.,
    pub /: *mut *mut bool need_rmap_locks; / Do rmap locks need to be taken?,
    pub /: *mut *mut bool for_stack; / Is this an early temp stack being moved?,
}

extern "C" {
    pub fn __pgprot(pgprot_val(newprot): pgprot_val(oldprot) |) -> return;
}
extern "C" {
    pub fn __pgprot(_arg: vm_flags) -> return;
}
extern "C" {
    pub fn test_bit(_arg: flag, _arg: ACCESS_PRIVATE(&mm->flags, _arg: __mm_flags)) -> return;
}
//
// Copy value to the first system word of VMA flags, non-atomically.
//
// IMPORTANT: This does not overwrite bytes past the first system word. The
// caller must account for this.
//
// Copy value to the first system word of VMA flags ONCE, non-atomically.
//
// IMPORTANT: This does not overwrite bytes past the first system word. The
// caller must account for this.
//
// Update the first system word of VMA flags setting bits, non-atomically.
// bitmap |= value;
// Update the first system word of VMA flags clearing bits, non-atomically.
// bitmap &= ~value;
//
// Helper function which converts a vma_flags_t value to a legacy vm_flags_t
// value. This is only valid if the input flags value can be expressed in a
// system word.
//
// Will be removed once the conversion to VMA flags is complete.
//
// Helper function which converts a legacy vm_flags_t value to a vma_flags_t
// value.
//
// Will be removed once the conversion to VMA flags is complete.
//
// Use when VMA is not part of the VMA tree and needs no locking
//
// Use when VMA is part of the VMA tree and modifications need coordination
// Note: vm_flags_reset and vm_flags_reset_once do not lock the vma and
// it should be locked explicitly beforehand.
//
// It is assumed only the first system word must be written once.
// The remainder can be copied normally.

extern "C" {
    pub fn bitmap_weight(_arg: bitmap, _arg: NUM_VMA_FLAG_BITS) -> return;
}
extern "C" {
    pub fn test_bit(int)bit: (, _arg: bitmap) -> return;
}

extern "C" {
    pub fn bitmap_intersects(_arg: bitmap_to_test, _arg: bitmap, _arg: NUM_VMA_FLAG_BITS) -> return;
}

extern "C" {
    pub fn bitmap_subset(_arg: bitmap_to_test, _arg: bitmap, _arg: NUM_VMA_FLAG_BITS) -> return;
}

extern "C" {
    pub fn vma_flags_test_any_mask(_arg: flags, _arg: flagmask) -> return;
}

extern "C" {
    pub fn bitmap_equal(_arg: bitmap, _arg: bitmap_other, _arg: NUM_VMA_FLAG_BITS) -> return;
}
extern "C" {
    pub fn bitmap_equal(_arg: bitmap, _arg: bitmap_other, _arg: NUM_VMA_FLAG_BITS) -> return;
}

extern "C" {
    pub fn vma_flags_test(_arg: &vma->flags, _arg: bit) -> return;
}
extern "C" {
    pub fn vma_flags_test_any_mask(_arg: &vma->flags, _arg: flags) -> return;
}

extern "C" {
    pub fn vma_flags_test_all_mask(_arg: &vma->flags, _arg: flags) -> return;
}

extern "C" {
    pub fn vma_flags_test_single_mask(_arg: &vma->flags, _arg: flagmask) -> return;
}

extern "C" {
    pub fn vma_flags_test(_arg: &desc->vma_flags, _arg: bit) -> return;
}
extern "C" {
    pub fn vma_flags_test_any_mask(_arg: &desc->vma_flags, _arg: flags) -> return;
}

extern "C" {
    pub fn vma_flags_test_all_mask(_arg: &desc->vma_flags, _arg: flags) -> return;
}

extern "C" {
    pub fn vma_flags_test_all(_arg: flags, _arg: VMA_SHARED_BIT, _arg: VMA_MAYWRITE_BIT) -> return;
}
extern "C" {
    pub fn is_shared_maywrite(_arg: &vma->flags) -> return;
}
extern "C" {
    pub fn vma_flags_is_cow_mapping(_arg: &vma->flags) -> return;
}
//
// Uses mas_find() to get the first VMA when the iterator starts.
// Calling mas_next() could skip the first entry.
//
extern "C" {
    pub fn mas_find(_arg: &vmi->mas, _arg: ULONG_MAX) -> return;
}
extern "C" {
    pub fn refcount_read(_arg: &vma->vm_refcnt) -> return;
}
//
// WARNING: to avoid racing with vma_mark_attached()/vma_mark_detached(), these
// assertions should be made either under mmap_write_lock or when the object
// has been isolated under mmap_write_lock, ensuring no competing writers.
//
extern "C" {
    pub fn vma_assert_write_locked(: *mut vm_area_struct);
}
// We are the only writer, so no need to use vma_refcount_put().
//
// Reader must have temporarily raised vm_refcnt but it will
// drop it without using the vma since vma is write-locked.
//
// These are defined in vma.h, but sadly vm_stat_account() is referenced by
// kernel/fork.c, so we have to these broadly available there, and temporarily
// define them here to resolve the dependency cycle.
//

extern "C" {
    pub fn mas_find(_arg: &vmi->mas, 1: max -) -> return;
}
// Declared in vma.h.
// Default.
extern "C" {
    pub fn vma_start_pgoff(vma_pages(vma: vma) +) -> return;
}

extern "C" {
    pub fn vma_start_anon_pgoff(vma_pages(vma: vma) +) -> return;
}
// Perform any preparatory tasks for mmap action.
// Update the VMA from the descriptor.
// Complete any specified mmap actions.
extern "C" {
    pub fn mmap_action_complete(_arg: vma, _arg: &desc->action, _arg: *mut *mut /is_compat=/true) -> return;
}
// being invoked from .mmmap means we don't have to enforce this.
extern "C" {
    pub fn __compat_vma_mmap(_arg: &desc, _arg: vma) -> return;
}
extern "C" {
    pub fn mmap_assert_locked(: *mut mm_struct);
}
extern "C" {
    pub fn mt_find(_arg: &mm->mm_mt, _arg: &index, 1: end_addr -) -> return;
}
extern "C" {
    pub fn mtree_load(_arg: &mm->mm_mt, _arg: addr) -> return;
}
extern "C" {
    pub fn mas_prev(_arg: &vmi->mas, _arg: 0) -> return;
}
// Defined in vma.h, so temporarily define here to avoid circular dependency.

// pprev = vma_prev(&vmi);

extern "C" {
    pub fn mas_next_range(_arg: &vmi->mas, _arg: ULONG_MAX) -> return;
}
extern "C" {
    pub fn vma_wants_writenotify(vma: *mut vm_area_struct, vm_page_prot: pgprot_t) -> bool;
}
// Update vma->vm_page_prot to reflect vma->vm_flags.
// testing: we inline vm_pgprot_modify() to avoid clash with vma.h.
// remove_protection_ptes reads vma->vm_page_prot without mmap_lock
// See reasoning around the VM_SHADOW_STACK definition
// Did the driver provide valid mmap hook configuration?
// Hooks are mutually exclusive.
extern "C" {
    pub fn compat_vma_mmap(_arg: file, _arg: vma) -> return;
}
// Changing an anonymous vma with this is illegal
extern "C" {
    pub fn READ_ONCE(_arg: sysctl_max_map_count) -> return;
}

extern "C" {
    pub fn vm_get_page_prot(_arg: vm_flags) -> return;
}
extern "C" {
    pub fn vma_flags_to_page_prot(_arg: vma->flags) -> return;
}
// Account for MAP_PRIVATE-/dev/zero which is only semi-anonymous.
