//! Automatically rewritten from C Header to Rust Module
//! Source: mm/vma.h
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
//
// vma.h
//
// Core VMA manipulation API implemented in vma.c, vma_init.c and vma_exec.c.
//
// Note that, in order for VMA logic to be userland testable, this header
// intentionally includes no dependencies.
//
// This is specifically scoped to mm-only. Users of this functionality (other
// than the core VMA implementation itself) should not include this header
// directly, but rather include internal.h.
//
// VMA lock generalization
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vma_prepare {
    pub vma: *mut vm_area_struct,
    pub adj_next: *mut vm_area_struct,
    pub file: *mut file,
    pub mapping: *mut address_space,
    pub anon_vma: *mut anon_vma,
    pub insert: *mut vm_area_struct,
    pub remove: *mut vm_area_struct,
    pub remove2: *mut vm_area_struct,
    pub :1: bool skip_vma_uprobe,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct unlink_vma_file_batch {
    pub count: c_int,
    pub vmas: [*mut vm_area_struct; 8],
}

//
// vma munmap operation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vma_munmap_struct {
    pub vmi: *mut vma_iterator,
//     pub /: *mut *mut *mut vm_area_vma; / The first vma to munmap,
//     pub /: *mut *mut *mut vm_area_prev; / vma before the munmap area,
//     pub /: *mut *mut *mut vm_area_next; / vma after the munmap area,
//     pub /: *mut *mut *mut list_head uf; / Userfaultfd list_head,
//     pub /: *mut *mut unsigned long start; / Aligned start addr (inclusive),
//     pub /: *mut *mut unsigned long end; / Aligned end addr (exclusive),
//     pub /: *mut *mut unsigned long unmap_start; / Unmap PTE start,
//     pub /: *mut *mut unsigned long unmap_end; / Unmap PTE end,
//     pub /: *mut *mut int vma_count; / Number of vmas that will be removed,
//     pub /: *mut *mut bool unlock; / Unlock after the munmap,
//     pub /: *mut *mut bool clear_ptes; / If there are outstanding PTE to be cleared,
// 2 byte hole
//     pub /: *mut *mut unsigned long nr_pages; / Number of pages being removed,
//     pub /: *mut *mut unsigned long locked_vm; / Number of locked pages,
//     pub /: *mut *mut unsigned long nr_accounted; / Number of VM_ACCOUNT pages,
    pub exec_vm: c_ulong,
    pub stack_vm: c_ulong,
    pub data_vm: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vma_merge_state {
    VMA_MERGE_START,
    VMA_MERGE_ERROR_NOMEM,
    VMA_MERGE_NOMERGE,
    VMA_MERGE_SUCCESS,
}

//
// Describes a VMA merge operation and is threaded throughout it.
//
// Any of the fields may be mutated by the merge operation, so no guarantees are
// made to the contents of this structure after a merge operation has completed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vma_merge_struct {
    pub mm: *mut mm_struct,
    pub vmi: *mut vma_iterator,
//
// Adjacent VMAs, any of which may be NULL if not present:
//
// |------|--------|------|
// | prev | middle | next |
// |------|--------|------|
//
// middle may not yet exist in the case of a proposed new VMA being
// merged, or it may be an existing VMA.
//
// next may be assigned by the caller.
//
    pub prev: *mut vm_area_struct,
    pub middle: *mut vm_area_struct,
    pub next: *mut vm_area_struct,
// This is the VMA we ultimately target to become the merged VMA.
    pub target: *mut vm_area_struct,
//
// Initially, the start, end, pgoff fields are provided by the caller
// and describe the proposed new VMA range, whether modifying an
// existing VMA (which will be 'middle'), or adding a new one.
//
// During the merge process these fields are updated to describe the new
// range _including those VMAs which will be merged_.
//
    pub start: c_ulong,
    pub end: c_ulong,
    pub pgoff: pgoff_t,
    pub anon_pgoff: pgoff_t,
// Temporary while VMA flags are being converted.
    pub vm_flags: vm_flags_t,
    pub vma_flags: vma_flags_t,
}

// If copied from (i.e. mremap()'d) the VMA from which we are copying.
// Flags which callers can use to modify merge behaviour:
//
// If we can expand, simply do so. We know there is nothing to merge to
// the right. Does not reset state upon failure to merge. The VMA
// iterator is assumed to be positioned at the previous VMA, rather than
// at the gap.
//
// If a merge is possible, but an OOM error occurs, give up and don't
// execute the merge, returning NULL.
//
// If set, skip uprobe_mmap upon merged vma.
//
// Internal flags set during merge process:
//
// Internal flag indicating the merge increases vmg->middle->vm_start
// (and thereby, vmg->prev->vm_end).
//
// Internal flag indicating the merge decreases vmg->next->vm_start
// (and thereby, vmg->middle->vm_end).
//
// Internal flag used during the merge operation to indicate we will
// remove vmg->middle.
//
// Internal flag used during the merge operation to indicate we will
// remove vmg->next.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct unmap_desc {
//     pub /: *mut *mut *mut ma_state mas; / the maple state point to the first vma,
//     pub /: *mut *mut *mut vm_area_first; / The first vma,
//     pub /: *mut *mut unsigned long pg_start; / The first pagetable address to free (floor),
//     pub /: *mut *mut unsigned long pg_end; / The last pagetable address to free (ceiling),
//     pub /: *mut *mut unsigned long vma_start; / The min vma address,
//     pub /: *mut *mut unsigned long vma_end; / The max vma address,
//     pub /: *mut *mut unsigned long tree_end; / Maximum for the vma tree search,
//     pub /: *mut *mut unsigned long tree_reset; / Where to reset the vma tree walk,
//     pub /: *mut *mut bool mm_wr_locked; / If the mmap write lock is held,
}

//
// unmap_all_init() - Initialize unmap_desc to remove all vmas, point the
// pg_start and pg_end to a safe location.
//
// unmap_pgtable_init() - Initialize unmap_desc to remove all page tables within
// the user range.
//
// ARM can have mappings outside of vmas.
// See: e2cdef8c847b4 ("[PATCH] freepgt: free_pgtables from FIRST_USER_ADDRESS")
//
// ARM LPAE uses page table mappings beyond the USER_PGTABLES_CEILING
// See: CONFIG_ARM_LPAE in arch/arm/include/asm/pgtable.h
//

extern "C" {
    pub fn vmg_start_pgoff(vmg_pages(vmg: vmg) +) -> return;
}
// nommu doesn't set a virtual pgoff for anon VMAs.
//
// File-backed VMAs have arbitrary page offset (either page offset into
// file or for pfnmap the PFN of the start of the range or drivers may
// set arbitrary page offset).
//
// MAP_PRIVATE-/dev/zero is anon, non-NULL vm_file, but has file pgoff.
// If faulted in, could have been remapped.
// OK this is really an anon VMA - expect virtual page offset.
extern "C" {
    pub fn vmg_start_anon_pgoff(vmg_pages(vmg: vmg) +) -> return;
}

extern "C" {
    pub fn validate_mm(mm: *mut mm_struct);
}

extern "C" {
    pub fn vma_expand(vmg: *mut vma_merge_struct) -> __must_check int;
}
//
// Temporary helper function for stacked mmap handlers which specify
// f_op->mmap() but which might have an underlying file system which implements
// f_op->mmap_prepare().
//
// Since we're invoking .mmap_prepare() despite having a partially
// established VMA, we must take care to handle setting fields
// correctly.
//
// Mutable fields. Populated with initial state.
// User-defined fields.
extern "C" {
    pub fn remove_vma(vma: *mut vm_area_struct);
}
extern "C" {
    pub fn unmap_region(unmap: *mut unmap_desc);
}
//
// vma_modify_flags() - Perform any necessary split/merge in preparation for
// setting VMA flags to *@vm_flags in the range @start to @end contained within
// @vma.
// @vmi: Valid VMA iterator positioned at @vma.
// @prev: The VMA immediately prior to @vma or NULL if @vma is the first.
// @vma: The VMA containing the range @start to @end to be updated.
// @start: The start of the range to update. May be offset within @vma.
// @end: The exclusive end of the range to update, may be offset within @vma.
// @vma_flags_ptr: A pointer to the VMA flags that the @start to @end range is
// about to be set to. On merge, this will be updated to include sticky flags.
//
// IMPORTANT: The actual modification being requested here is NOT applied,
// rather the VMA is perhaps split, perhaps merged to accommodate the change,
// and the caller is expected to perform the actual modification.
//
// In order to account for sticky VMA flags, the @vma_flags_ptr parameter points
// to the requested flags which are then updated so the caller, should they
// overwrite any existing flags, correctly retains these.
//
// Returns: A VMA which contains the range @start to @end ready to have its
// flags altered to *@vma_flags.
//
// vma_modify_name() - Perform any necessary split/merge in preparation for
// setting anonymous VMA name to @new_name in the range @start to @end contained
// within @vma.
// @vmi: Valid VMA iterator positioned at @vma.
// @prev: The VMA immediately prior to @vma or NULL if @vma is the first.
// @vma: The VMA containing the range @start to @end to be updated.
// @start: The start of the range to update. May be offset within @vma.
// @end: The exclusive end of the range to update, may be offset within @vma.
// @new_name: The anonymous VMA name that the @start to @end range is about to
// be set to.
//
// IMPORTANT: The actual modification being requested here is NOT applied,
// rather the VMA is perhaps split, perhaps merged to accommodate the change,
// and the caller is expected to perform the actual modification.
//
// Returns: A VMA which contains the range @start to @end ready to have its
// anonymous VMA name changed to @new_name.
//
// vma_modify_policy() - Perform any necessary split/merge in preparation for
// setting NUMA policy to @new_pol in the range @start to @end contained
// within @vma.
// @vmi: Valid VMA iterator positioned at @vma.
// @prev: The VMA immediately prior to @vma or NULL if @vma is the first.
// @vma: The VMA containing the range @start to @end to be updated.
// @start: The start of the range to update. May be offset within @vma.
// @end: The exclusive end of the range to update, may be offset within @vma.
// @new_pol: The NUMA policy that the @start to @end range is about to be set
// to.
//
// IMPORTANT: The actual modification being requested here is NOT applied,
// rather the VMA is perhaps split, perhaps merged to accommodate the change,
// and the caller is expected to perform the actual modification.
//
// Returns: A VMA which contains the range @start to @end ready to have its
// NUMA policy changed to @new_pol.
//
// vma_modify_flags_uffd() - Perform any necessary split/merge in preparation for
// setting VMA flags to @vm_flags and UFFD context to @new_ctx in the range
// @start to @end contained within @vma.
// @vmi: Valid VMA iterator positioned at @vma.
// @prev: The VMA immediately prior to @vma or NULL if @vma is the first.
// @vma: The VMA containing the range @start to @end to be updated.
// @start: The start of the range to update. May be offset within @vma.
// @end: The exclusive end of the range to update, may be offset within @vma.
// @vma_flags: The VMA flags that the @start to @end range is about to be set to.
// @new_ctx: The userfaultfd context that the @start to @end range is about to
// be set to.
// @give_up_on_oom: If an out of memory condition occurs on merge, simply give
// up on it and treat the merge as best-effort.
//
// IMPORTANT: The actual modification being requested here is NOT applied,
// rather the VMA is perhaps split, perhaps merged to accommodate the change,
// and the caller is expected to perform the actual modification.
//
// Returns: A VMA which contains the range @start to @end ready to have its VMA
// flags changed to @vma_flags and its userfaultfd context changed to @new_ctx.
//
extern "C" {
    pub fn unlink_file_vma_batch_init(vb: *mut unlink_vma_file_batch);
}
extern "C" {
    pub fn unlink_file_vma_batch_final(vb: *mut unlink_vma_file_batch);
}
extern "C" {
    pub fn vma_needs_dirty_tracking(vma: *mut vm_area_struct) -> bool;
}
extern "C" {
    pub fn vma_wants_writenotify(vma: *mut vm_area_struct, vm_page_prot: pgprot_t) -> bool;
}
extern "C" {
    pub fn mm_take_all_locks(mm: *mut mm_struct) -> c_int;
}
extern "C" {
    pub fn mm_drop_all_locks(mm: *mut mm_struct);
}
extern "C" {
    pub fn unmapped_area(info: *mut vm_unmapped_area_info) -> c_ulong;
}
extern "C" {
    pub fn unmapped_area_topdown(info: *mut vm_unmapped_area_info) -> c_ulong;
}
//
// We want to check manually if we can change individual PTEs writable
// if we can't do that automatically for all PTEs in a mapping. For
// private mappings, that's always the case when we have write
// permissions as we properly have to handle COW.
//
extern "C" {
    pub fn vma_wants_writenotify(_arg: vma, _arg: vma->vm_page_prot) -> return;
}

extern "C" {
    pub fn pgprot_modify(_arg: oldprot, _arg: prot) -> return;
}

extern "C" {
    pub fn mas_prev(_arg: &vmi->mas, _arg: min) -> return;
}
//
// These three helpers classifies VMAs for virtual memory accounting.
//
// Executable code area - executable, not writable, not stack
//
// Stack area (including shadow stacks)
//
// VM_GROWSUP / VM_GROWSDOWN VMAs are always private anonymous:
// do_mmap() forbids all other combinations.
//
// Data area - private, writable, not stack
//
extern "C" {
    pub fn mas_prev_range(_arg: &vmi->mas, _arg: min) -> return;
}
extern "C" {
    pub fn mas_next_range(_arg: &vmi->mas, _arg: max) -> return;
}
extern "C" {
    pub fn mas_empty_area(_arg: &vmi->mas, _arg: min, 1: max -, _arg: size) -> return;
}
extern "C" {
    pub fn mas_empty_area_rev(_arg: &vmi->mas, _arg: min, 1: max -, _arg: size) -> return;
}
//
// VMA Iterator functions shared between nommu and mmap
//
extern "C" {
    pub fn mas_preallocate(_arg: &vmi->mas, _arg: vma, _arg: GFP_KERNEL) -> return;
}
extern "C" {
    pub fn mas_walk(_arg: &vmi->mas) -> return;
}
// Store a VMA with preallocated memory

extern "C" {
    pub fn mas_prev_range(_arg: &vmi->mas, _arg: 0) -> return;
}
//
// Retrieve the next VMA and rewind the iterator to end of the previous VMA, or
// if no previous VMA, to index 0.
//
// Consider the case where no previous VMA exists. We advance to the
// next VMA, skipping any gap, then rewind to the start of the range.
//
// If we were to unconditionally advance to the next range we'd wind up
// at the next VMA again, so we check to ensure there is a previous VMA
// to skip over.
//
// pprev = prev;

extern "C" {
    pub fn expand_upwards(vma: *mut vm_area_struct, address: c_ulong) -> c_int;
}

extern "C" {
    pub fn expand_downwards(vma: *mut vm_area_struct, address: c_ulong) -> c_int;
}
extern "C" {
    pub fn __vm_munmap(start: c_ulong, len: usize, unlock: bool) -> c_int;
}
extern "C" {
    pub fn insert_vm_struct(mm: *mut mm_struct, vma: *mut vm_area_struct) -> c_int;
}
// vma_init.h, shared between CONFIG_MMU and nommu.
extern "C" {
    pub fn vma_state_init() ;
}
extern "C" {
    pub fn vm_area_free(vma: *mut vm_area_struct);
}
// vma_exec.c

extern "C" {
    pub fn relocate_vma_down(vma: *mut vm_area_struct, shift: c_ulong) -> c_int;
}

//
// Denies creating a writable executable mapping or gaining executable permissions.
//
// This denies the following:
//
// a)	mmap(PROT_WRITE | PROT_EXEC)
//
// b)	mmap(PROT_WRITE)
// mprotect(PROT_EXEC)
//
// c)	mmap(PROT_WRITE)
// mprotect(PROT_READ)
// mprotect(PROT_EXEC)
//
// But allows the following:
//
// d)	mmap(PROT_READ | PROT_EXEC)
// mmap(PROT_READ | PROT_EXEC | PROT_BTI)
//
// This is only applicable if the user has set the Memory-Deny-Write-Execute
// (MDWE) protection mask for the current process.
//
// @old specifies the VMA flags the VMA originally possessed, and @new the ones
// we propose to set.
//
// Return: false if proposed change is OK, true if not ok and should be denied.
//
// If MDWE is disabled, we have nothing to deny.
// If the new VMA is not executable, we have nothing to deny.
// Under MDWE we do not accept newly writably executable VMAs...
// ...nor previously non-executable VMAs becoming executable.