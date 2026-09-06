//! Automatically rewritten from C to Rust
//! Source: mm/rmap.c
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


// SPDX-License-Identifier: GPL-2.0
//
// mm/rmap.c - physical to virtual reverse mappings
//
// Copyright 2001, Rik van Riel <riel@conectiva.com.br>
//
// Simple, low overhead reverse mapping scheme.
// Please try to keep this thing as modular as possible.
//
// Provides methods for unmapping each kind of mapped page:
// the anon methods track anonymous pages, and
// the file methods track pages belonging to an inode.
//
// Original design by Rik van Riel <riel@conectiva.com.br> 2001
// File methods by Dave McCracken <dmccr@us.ibm.com> 2003, 2004
// Anonymous methods by Andrea Arcangeli <andrea@suse.de> 2004
// Contributions by Hugh Dickins 2003, 2004
//
// Lock ordering in mm:
//
// inode->i_rwsem	(while writing or truncating, not reading or faulting)
// mm->mmap_lock
// mapping->invalidate_lock (in filemap_fault)
// folio_lock
// hugetlbfs_i_mmap_rwsem_key (in huge_pmd_share, see hugetlbfs below)
// vma_start_write
// mapping->i_mmap_rwsem
// anon_vma->rwsem
// mm->page_table_lock or pte_lock
// swap_lock (in swap_duplicate, swap_info_get)
// mmlist_lock (in mmput, drain_mmlist and others)
// mapping->private_lock (in block_dirty_folio)
// i_pages lock (widely used)
// lruvec->lru_lock (in folio_lruvec_lock_irq)
// inode->i_lock (in set_page_dirty's __mark_inode_dirty)
// bdi.wb->list_lock (in set_page_dirty's __mark_inode_dirty)
// sb_lock (within inode_lock in fs/fs-writeback.c)
// i_pages lock (widely used, in set_page_dirty,
// in arch-dependent flush_dcache_mmap_lock,
within bdi.wb.list_lock in __sync_single_inode)
//
// anon_vma->rwsem,mapping->i_mmap_rwsem   (memory_failure, collect_procs_anon)
// ->tasklist_lock
// pte map lock
//
// hugetlbfs PageHuge() take locks in this order:
// hugetlb_fault_mutex (hugetlbfs specific page fault mutex)
// vma_lock (hugetlb specific lock for pmd_sharing)
// mapping->i_mmap_rwsem (also used for hugetlb pmd sharing)
// folio_lock
//

// Macro flag: #define CREATE_TRACE_POINTS

pub static mut anon_vma_cachep: *mut c_void = core::ptr::null_mut();
pub static mut anon_vma_chain_cachep: *mut c_void = core::ptr::null_mut();
#[no_mangle]
pub unsafe extern "C" fn anon_vma_alloc() -> *mut c_void {
pub static mut anon_vma: *mut c_void = core::ptr::null_mut();
    anon_vma = kmem_cache_alloc(anon_vma_cachep, GFP_KERNEL);
    if (anon_vma) {
    atomic_set(&anon_vma.refcount, 1);
    anon_vma.num_children = 0;
    anon_vma.num_active_vmas = 0;
    anon_vma.parent = anon_vma;
//
// Initialise the anon_vma root to point to itself. If called
// from fork, the root will be reset to the parents anon_vma.
//
    anon_vma.root = anon_vma;
    }
    return anon_vma;
    }
#[no_mangle]
pub unsafe extern "C" fn anon_vma_free(anon_vma: *mut anon_vma) {
    VM_BUG_ON(atomic_read(&anon_vma.refcount));
//
// Synchronize against folio_lock_anon_vma_read() such that
// we can safely hold the lock without the anon_vma getting
// freed.
//
// Relies on the full mb implied by the atomic_dec_and_test() from
// put_anon_vma() against the acquire barrier implied by
// down_read_trylock() from folio_lock_anon_vma_read(). This orders:
//
// folio_lock_anon_vma_read()	VS	put_anon_vma()
// down_read_trylock()		  atomic_dec_and_test()
// LOCK				  MB
// atomic_read()			  rwsem_is_locked()
//
// LOCK should suffice since the actual taking of the lock must
// happen _before_ what follows.
//
    might_sleep();
    if (rwsem_is_locked(&anon_vma.root.rwsem)) {
    anon_vma_lock_write(anon_vma);
    anon_vma_unlock_write(anon_vma);
    }
    kmem_cache_free(anon_vma_cachep, anon_vma);
    }
#[no_mangle]
pub unsafe extern "C" fn anon_vma_chain_alloc(gfp: gfp_t) -> *mut c_void {
    return kmem_cache_alloc(anon_vma_chain_cachep, gfp);
    }
#[no_mangle]
unsafe extern "C" fn anon_vma_chain_free(anon_vma_chain: *mut anon_vma_chain) {
    kmem_cache_free(anon_vma_chain_cachep, anon_vma_chain);
    }
#[no_mangle]
pub unsafe extern "C" fn anon_vma_chain_assign(vma: *mut vm_area_struct, avc: *mut anon_vma_chain, anon_vma: *mut anon_vma) {
    avc.vma = vma;
    avc.anon_vma = anon_vma;
    list_add(&avc.same_vma, &vma.anon_vma_chain);
    }
//
// __anon_vma_prepare - attach an anon_vma to a memory region
// @vma: the memory region in question
//
// This makes sure the memory mapping described by 'vma' has
// an 'anon_vma' attached to it, so that we can associate the
// anonymous pages mapped into it with that anon_vma.
//
// The common case will be that we already have one, which
// is handled inline by anon_vma_prepare(). But if
// not we either need to find an adjacent mapping that we
// can re-use the anon_vma from (very common when the only
// reason for splitting a vma has been mprotect()), or we
// allocate a new one.
//
// Anon-vma allocations are very subtle, because we may have
// optimistically looked up an anon_vma in folio_lock_anon_vma_read()
// and that may actually touch the rwsem even in the newly
// allocated vma (it depends on RCU to make sure that the
// anon_vma isn't actually destroyed).
//
// As a result, we need to do proper anon_vma locking even
// for the new allocation. At the same time, we do not want
// to do any locking for the common case of already having
// an anon_vma.
//
#[no_mangle]
pub unsafe extern "C" fn __anon_vma_prepare(vma: *mut vm_area_struct) -> c_int {
    let mut mm = vma.vm_mm;
    let mut anon_vma = core::ptr::null_mut();
    let mut allocated = core::ptr::null_mut();
pub static mut avc: *mut c_void = core::ptr::null_mut();
    mmap_assert_locked(mm);
    might_sleep();
    avc = anon_vma_chain_alloc(GFP_KERNEL);
    if (!avc) {
// goto;
    }
    anon_vma = find_mergeable_anon_vma(vma);
    allocated = core::ptr::null_mut();
    if (!anon_vma) {
    anon_vma = anon_vma_alloc();
    if (unlikely(!anon_vma)) {
// goto;
    }
    anon_vma.num_children += 1; /* self-parent link for new root */
    allocated = anon_vma;
    }
    anon_vma_lock_write(anon_vma);
// page_table_lock to protect against threads
    spin_lock(&mm.page_table_lock);
    if (likely(!vma.anon_vma)) {
    vma.anon_vma = anon_vma;
    anon_vma_chain_assign(vma, avc, anon_vma);
    anon_rmap_tree_insert(avc, anon_vma);
    anon_vma.num_active_vmas += 1;
    allocated = core::ptr::null_mut();
    avc = core::ptr::null_mut();
    }
    spin_unlock(&mm.page_table_lock);
    anon_vma_unlock_write(anon_vma);
    if (unlikely(allocated)) {
    put_anon_vma(allocated);
    }
    if (unlikely(avc)) {
    anon_vma_chain_free(avc);
    }
    return 0;
// label;
    anon_vma_chain_free(avc);
// label;
    return -ENOMEM;
    }
#[no_mangle]
pub unsafe extern "C" fn check_anon_vma_clone(dst: *mut vm_area_struct, src: *mut vm_area_struct, operation: vma_operation) {
// The write lock must be held.
    mmap_assert_write_locked(src.vm_mm);
// If not a fork then must be on same mm.
    VM_WARN_ON_ONCE(operation != VMA_OP_FORK && dst.vm_mm != src.vm_mm);
// If we have anything to do src->anon_vma must be provided.
    VM_WARN_ON_ONCE(!src.anon_vma && !list_empty(&src.anon_vma_chain));
    VM_WARN_ON_ONCE(!src.anon_vma && dst.anon_vma);
// We are establishing a new anon_vma_chain.
    VM_WARN_ON_ONCE(!list_empty(&dst.anon_vma_chain));
//
// On fork, dst->anon_vma is set NULL (temporarily). Otherwise, anon_vma
// must be the same across dst and src.
//
    VM_WARN_ON_ONCE(dst.anon_vma && dst.anon_vma != src.anon_vma);
//
// Essentially equivalent to above - if not a no-op, we should expect
// dst->anon_vma to be set for everything except a fork.
//
    VM_WARN_ON_ONCE(operation != VMA_OP_FORK && src.anon_vma &&
    !dst.anon_vma);
// For the anon_vma to be compatible, it can only be singular.
    VM_WARN_ON_ONCE(operation == VMA_OP_MERGE_UNFAULTED &&
    !list_is_singular(&src.anon_vma_chain));

// Only merging an unfaulted VMA leaves the destination attached.
    VM_WARN_ON_ONCE(operation != VMA_OP_MERGE_UNFAULTED &&
    vma_is_attached(dst));

    }
#[no_mangle]
pub unsafe extern "C" fn maybe_reuse_anon_vma(dst: *mut vm_area_struct, anon_vma: *mut anon_vma) {
// If already populated, nothing to do.
    if (dst.anon_vma) {
    return;
    }
//
// We reuse an anon_vma if any linking VMAs were unmapped and it has
// only a single child at most.
//
    if (anon_vma.num_active_vmas > 0) {
    return;
    }
    if (anon_vma.num_children > 1) {
    return;
    }
    dst.anon_vma = anon_vma;
    anon_vma.num_active_vmas += 1;
    }
// forward_decl: cleanup_partial_anon_vmas;
//
// anon_vma_clone - Establishes new anon_vma_chain objects in @dst linking to
// all of the anon_vma objects contained within @src anon_vma_chain's.
// @dst: The destination VMA with an empty anon_vma_chain.
// @src: The source VMA we wish to duplicate.
// @operation: The type of operation which resulted in the clone.
//
// This is the heart of the VMA side of the anon_vma implementation - we invoke
// this function whenever we need to set up a new VMA's anon_vma state.
//
// This is invoked for:
//
// - VMA Merge, but only when @dst is unfaulted and @src is faulted - meaning we
// clone @src into @dst.
// - VMA split.
// - VMA (m)remap.
// - Fork of faulted VMA.
//
// In all cases other than fork this is simply a duplication. Fork additionally
// adds a new active anon_vma.
//
// ONLY in the case of fork do we try to 'reuse' existing anon_vma's in an
// anon_vma hierarchy, reusing anon_vma's which have no VMA associated with them
// but do have a single child. This is to avoid waste of memory when repeatedly
// forking.
//
// Returns: 0 on success, -ENOMEM on failure.
//
#[no_mangle]
pub unsafe extern "C" fn anon_vma_clone(dst: *mut vm_area_struct, src: *mut vm_area_struct, operation: vma_operation) -> c_int {
    let mut avc = core::ptr::null_mut();
    let mut pavc = core::ptr::null_mut();
    let mut active_anon_vma = src.anon_vma;
    check_anon_vma_clone(dst, src, operation);
    if (!active_anon_vma) {
    return 0;
    }
//
// Allocate AVCs. We don't need an anon_vma lock for this as we
// are not updating the anon_vma rbtree nor are we changing
// anon_vma statistics.
//
// Either src, dst have the same mm for which we hold an exclusive mmap
// write lock, or we are forking and we hold it on src->vm_mm and dst is
// not yet accessible to other threads so there's no possibliity of the
// unlinked AVC's being observed yet.
//
    list_for_each_entry(pavc, &src.anon_vma_chain, same_vma) {
    avc = anon_vma_chain_alloc(GFP_KERNEL);
    if (!avc) {
// goto;
    }
    anon_vma_chain_assign(dst, avc, pavc.anon_vma);
    }
//
// Now link the anon_vma's back to the newly inserted AVCs.
// Note that all anon_vma's share the same root.
//
    anon_vma_lock_write(active_anon_vma);
    list_for_each_entry_reverse(avc, &dst.anon_vma_chain, same_vma) {
    let mut anon_vma = avc.anon_vma;
    anon_rmap_tree_insert(avc, anon_vma);
    if (operation == VMA_OP_FORK) {
    maybe_reuse_anon_vma(dst, anon_vma);
    }
    }
    if (operation != VMA_OP_FORK) {
    dst.anon_vma.num_active_vmas += 1;
    }
    anon_vma_unlock_write(active_anon_vma);
    return 0;
// label;
    cleanup_partial_anon_vmas(dst);
    return -ENOMEM;
    }
//
// Attach vma to its own anon_vma, as well as to the anon_vmas that
// the corresponding VMA in the parent process is attached to.
// Returns 0 on success, non-zero on failure.
//
#[no_mangle]
pub unsafe extern "C" fn anon_vma_fork(vma: *mut vm_area_struct, pvma: *mut vm_area_struct) -> c_int {
pub static mut avc: *mut c_void = core::ptr::null_mut();
pub static mut anon_vma: *mut c_void = core::ptr::null_mut();
    let mut rc = 0;
// Don't bother if the parent process has no anon_vma here.
    if (!pvma.anon_vma) {
    return 0;
    }
// Drop inherited anon_vma, we'll reuse existing or allocate new.
    vma.anon_vma = core::ptr::null_mut();
    anon_vma = anon_vma_alloc();
    if (!anon_vma) {
    return -ENOMEM;
    }
    avc = anon_vma_chain_alloc(GFP_KERNEL);
    if (!avc) {
    put_anon_vma(anon_vma);
    return -ENOMEM;
    }
//
// First, attach the new VMA to the parent VMA's anon_vmas,
// so rmap can find non-COWed pages in child processes.
//
    rc = anon_vma_clone(vma, pvma, VMA_OP_FORK);
// An error arose or an existing anon_vma was reused, all done then.
    if (rc || vma.anon_vma) {
    put_anon_vma(anon_vma);
    anon_vma_chain_free(avc);
    return rc;
    }
//
// OK no reuse, so add our own anon_vma.
//
// Since it is not linked anywhere we can safely manipulate anon_vma
// fields without a lock.
//
    anon_vma.num_active_vmas = 1;
//
// The root anon_vma's rwsem is the lock actually used when we
// lock any of the anon_vmas in this anon_vma tree.
//
    anon_vma.root = pvma.anon_vma.root;
    anon_vma.parent = pvma.anon_vma;
//
// With refcounts, an anon_vma can stay around longer than the
// process it belongs to. The root anon_vma needs to be pinned until
// this anon_vma is freed, because the lock lives in the root.
//
    get_anon_vma(anon_vma.root);
// Mark this anon_vma as the one where our new (COWed) pages go.
    vma.anon_vma = anon_vma;
    anon_vma_chain_assign(vma, avc, anon_vma);
// Now let rmap see it.
    anon_vma_lock_write(anon_vma);
    anon_rmap_tree_insert(avc, anon_vma);
    anon_vma.parent.num_children += 1;
    anon_vma_unlock_write(anon_vma);
    return 0;
    }
//
// In the unfortunate case of anon_vma_clone() failing to allocate memory we
// have to clean things up.
//
// Since we allocate anon_vma_chain's before we insert them into the interval
// trees, we simply have to free up the AVC's and remove the entries from the
// VMA's anon_vma_chain.
//
#[no_mangle]
unsafe extern "C" fn cleanup_partial_anon_vmas(vma: *mut vm_area_struct) {
    let mut avc = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
    list_for_each_entry_safe(avc, next, &vma.anon_vma_chain, same_vma) {
    list_del(&avc.same_vma);
    anon_vma_chain_free(avc);
    }
//
// The anon_vma assigned to this VMA is no longer valid, as we were not
// able to correctly clone AVC state. Avoid inconsistent anon_vma tree
// state by resetting.
//
    vma.anon_vma = core::ptr::null_mut();
    }
//
// unlink_anon_vmas() - remove all links between a VMA and anon_vma's, freeing
// anon_vma_chain objects.
// @vma: The VMA whose links to anon_vma objects is to be severed.
//
// As part of the process anon_vma_chain's are freed,
// anon_vma->num_children,num_active_vmas is updated as required and, if the
// relevant anon_vma references no further VMAs, its reference count is
// decremented.
//
#[no_mangle]
pub unsafe extern "C" fn unlink_anon_vmas(vma: *mut vm_area_struct) {
    let mut avc = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
    let mut active_anon_vma = vma.anon_vma;
// Always hold mmap lock, read-lock on unmap possibly.
    mmap_assert_locked(vma.vm_mm);
// Unfaulted is a no-op.
    if (!active_anon_vma) {
    VM_WARN_ON_ONCE(!list_empty(&vma.anon_vma_chain));
    return;
    }
    anon_vma_lock_write(active_anon_vma);
//
// Unlink each anon_vma chained to the VMA.  This list is ordered
// from newest to oldest, ensuring the root anon_vma gets freed last.
//
    list_for_each_entry_safe(avc, next, &vma.anon_vma_chain, same_vma) {
    let mut anon_vma = avc.anon_vma;
    anon_rmap_tree_remove(avc, anon_vma);
//
// Leave empty anon_vmas on the list - we'll need
// to free them outside the lock.
//
    if (RB_EMPTY_ROOT(&anon_vma.rb_root.rb_root)) {
    anon_vma.parent.num_children -= 1;
    continue;
    }
    list_del(&avc.same_vma);
    anon_vma_chain_free(avc);
    }
    active_anon_vma.num_active_vmas -= 1;
//
// vma would still be needed after unlink, and anon_vma will be prepared
// when handle fault.
//
    vma.anon_vma = core::ptr::null_mut();
    anon_vma_unlock_write(active_anon_vma);
//
// Iterate the list once more, it now only contains empty and unlinked
// anon_vmas, destroy them. Could not do before due to __put_anon_vma()
// needing to write-acquire the anon_vma->root->rwsem.
//
    list_for_each_entry_safe(avc, next, &vma.anon_vma_chain, same_vma) {
    let mut anon_vma = avc.anon_vma;
    VM_WARN_ON(anon_vma.num_children);
    VM_WARN_ON(anon_vma.num_active_vmas);
    put_anon_vma(anon_vma);
    list_del(&avc.same_vma);
    anon_vma_chain_free(avc);
    }
    }
#[no_mangle]
unsafe extern "C" fn anon_vma_ctor(data: *mut c_void) {
    let mut anon_vma = data;
    init_rwsem(&anon_vma.rwsem);
    atomic_set(&anon_vma.refcount, 0);
    anon_vma.rb_root = RB_ROOT_CACHED;
    }
#[no_mangle]
pub unsafe extern "C" fn anon_vma_init()  {
    anon_vma_cachep = kmem_cache_create("anon_vma", sizeof!(anon_vma),
    0, SLAB_TYPESAFE_BY_RCU|SLAB_PANIC|SLAB_ACCOUNT,
    anon_vma_ctor);
    anon_vma_chain_cachep = KMEM_CACHE(anon_vma_chain,
    SLAB_PANIC|SLAB_ACCOUNT);
    }
//
// Getting a lock on a stable anon_vma from a page off the LRU is tricky!
//
// Since there is no serialization what so ever against folio_remove_rmap_*()
// the best this function can do is return a refcount increased anon_vma
// that might have been relevant to this page.
//
// The page might have been remapped to a different anon_vma or the anon_vma
// returned may already be freed (and even reused).
//
// In case it was remapped to a different anon_vma, the new anon_vma will be a
// child of the old anon_vma, and the anon_vma lifetime rules will therefore
// ensure that any anon_vma obtained from the page will still be valid for as
// long as we observe folio_mapped() [ hence all those folio_mapped() tests ].
//
// All users of this function must be very careful when walking the anon_vma
// chain and verify that the page in question is indeed mapped in it
// [ something equivalent to page_mapped_in_vma() ].
//
// Since anon_vma's slab is SLAB_TYPESAFE_BY_RCU and we know from
// folio_remove_rmap_*() that the anon_vma pointer from page->mapping is valid
// if there is a mapcount, we can dereference the anon_vma after observing
// those.
//
// NOTE: the caller should hold folio lock when calling this.
//
#[no_mangle]
pub unsafe extern "C" fn folio_get_anon_vma(folio: *mut folio) -> *mut c_void {
    let mut anon_vma = core::ptr::null_mut();
    let mut anon_mapping = 0;
    VM_WARN_ON_FOLIO(!folio_test_locked(folio), folio);
    rcu_read_lock();
    anon_mapping = (unsigned long)READ_ONCE(folio.mapping);
    if ((anon_mapping & FOLIO_MAPPING_FLAGS) != FOLIO_MAPPING_ANON) {
// goto;
    }
    if (!folio_mapped(folio)) {
// goto;
    }
    anon_vma =  (anon_mapping - FOLIO_MAPPING_ANON);
    if (!atomic_inc_not_zero(&anon_vma.refcount)) {
    anon_vma = core::ptr::null_mut();
// goto;
    }
//
// If this folio is still mapped, then its anon_vma cannot have been
// freed.  But if it has been unmapped, we have no security against the
// anon_vma structure being freed and reused (for another anon_vma:
// SLAB_TYPESAFE_BY_RCU guarantees that - so the atomic_inc_not_zero()
// above cannot corrupt).
//
    if (!folio_mapped(folio)) {
    rcu_read_unlock();
    put_anon_vma(anon_vma);
    return core::ptr::null_mut();
    }
// label;
    rcu_read_unlock();
    return anon_vma;
    }
//
// Similar to folio_get_anon_vma() except it locks the anon_vma.
//
// Its a little more complex as it tries to keep the fast path to a single
// atomic op -- the trylock. If we fail the trylock, we fall back to getting a
// reference like with folio_get_anon_vma() and then block on the mutex
// on !rwc->try_lock case.
//
#[no_mangle]
pub unsafe extern "C" fn folio_lock_anon_vma_read(folio: *mut folio, rwc: *mut rmap_walk_control) -> *mut c_void {
    let mut anon_vma = core::ptr::null_mut();
pub static mut root_anon_vma: *mut c_void = core::ptr::null_mut();
    let mut anon_mapping = 0;
    VM_WARN_ON_FOLIO(!folio_test_locked(folio), folio);
    rcu_read_lock();
    anon_mapping = (unsigned long)READ_ONCE(folio.mapping);
    if ((anon_mapping & FOLIO_MAPPING_FLAGS) != FOLIO_MAPPING_ANON) {
// goto;
    }
    if (!folio_mapped(folio)) {
// goto;
    }
    anon_vma =  (anon_mapping - FOLIO_MAPPING_ANON);
    root_anon_vma = READ_ONCE(anon_vma.root);
    if (down_read_trylock(&root_anon_vma.rwsem)) {
//
// If the folio is still mapped, then this anon_vma is still
// its anon_vma, and holding the mutex ensures that it will
// not go away, see anon_vma_free().
//
    if (!folio_mapped(folio)) {
    up_read(&root_anon_vma.rwsem);
    anon_vma = core::ptr::null_mut();
    }
// goto;
    }
    if (rwc && rwc.try_lock) {
    anon_vma = core::ptr::null_mut();
    rwc.contended = true;
// goto;
    }
// trylock failed, we got to sleep
    if (!atomic_inc_not_zero(&anon_vma.refcount)) {
    anon_vma = core::ptr::null_mut();
// goto;
    }
    if (!folio_mapped(folio)) {
    rcu_read_unlock();
    put_anon_vma(anon_vma);
    return core::ptr::null_mut();
    }
// we pinned the anon_vma, its safe to sleep
    rcu_read_unlock();
    anon_vma_lock_read(anon_vma);
    if (atomic_dec_and_test(&anon_vma.refcount)) {
//
// Oops, we held the last refcount, release the lock
// and bail -- can't simply use put_anon_vma() because
// we'll deadlock on the anon_vma_lock_write() recursion.
//
    anon_vma_unlock_read(anon_vma);
    __put_anon_vma(anon_vma);
    anon_vma = core::ptr::null_mut();
    }
    return anon_vma;
// label;
    rcu_read_unlock();
    return anon_vma;
    }

//
// Flush TLB entries for recently unmapped pages from remote CPUs. It is
// important if a PTE was dirty when it was unmapped that it's flushed
// before any IO is initiated on the page to prevent lost writes. Similarly,
// it must be flushed before freeing to prevent data leakage.
//
#[no_mangle]
pub unsafe extern "C" fn try_to_unmap_flush() {
    let mut tlb_ubc = &current.tlb_ubc;
    if (!tlb_ubc.flush_required) {
    return;
    }
    arch_tlbbatch_flush(&tlb_ubc.arch);
    tlb_ubc.flush_required = false;
    tlb_ubc.writable = false;
    }
// Flush iff there are potentially writable TLB entries that can race with IO
#[no_mangle]
pub unsafe extern "C" fn try_to_unmap_flush_dirty() {
    let mut tlb_ubc = &current.tlb_ubc;
    if (tlb_ubc.writable) {
    try_to_unmap_flush();
    }
    }
//
// Bits 0-14 of mm->tlb_flush_batched record pending generations.
// Bits 16-30 of mm->tlb_flush_batched bit record flushed generations.
//
pub const TLB_FLUSH_BATCH_FLUSHED_SHIFT: c_int = 16;

    ((1 << (TLB_FLUSH_BATCH_FLUSHED_SHIFT - 1)) - 1)

    (TLB_FLUSH_BATCH_PENDING_MASK / 2)
#[no_mangle]
pub unsafe extern "C" fn set_tlb_ubc_flush_pending(mm: *mut mm_struct, pteval: pte_t, start: c_ulong, end: c_ulong) {
    let mut tlb_ubc = &current.tlb_ubc;
    let mut batch = 0;
pub static mut writable: bool = false;
    if (!pte_accessible(mm, pteval)) {
    return;
    }
    arch_tlbbatch_add_pending(&tlb_ubc.arch, mm, start, end);
    tlb_ubc.flush_required = true;
//
// Ensure compiler does not re-order the setting of tlb_flush_batched
// before the PTE is cleared.
//
    barrier();
    batch = atomic_read(&mm.tlb_flush_batched);
// label;
    if ((batch & TLB_FLUSH_BATCH_PENDING_MASK) > TLB_FLUSH_BATCH_PENDING_LARGE) {
//
// Prevent `pending' from catching up with `flushed' because of
// overflow.  Reset `pending' and `flushed' to be 1 and 0 if
// `pending' becomes large.
//
    if (!atomic_try_cmpxchg(&mm.tlb_flush_batched, &batch, 1)) {
// goto;
    }
    } else {
    atomic_inc(&mm.tlb_flush_batched);
    }
//
// If the PTE was dirty then it's best to assume it's writable. The
// caller must use try_to_unmap_flush_dirty() or try_to_unmap_flush()
// before the page is queued for IO.
//
    if (writable) {
    tlb_ubc.writable = true;
    }
    }
//
// Returns true if the TLB flush should be deferred to the end of a batch of
// unmap operations to reduce IPIs.
//
#[no_mangle]
unsafe extern "C" fn should_defer_flush(mm: *mut mm_struct, flags: ttu_flags) -> bool {
    if (!(flags & TTU_BATCH_FLUSH)) {
    return false;
    }
    return arch_tlbbatch_should_defer(mm);
    }
//
// Reclaim unmaps pages under the PTL but do not flush the TLB prior to
// releasing the PTL if TLB flushes are batched. It's possible for a parallel
// operation such as mprotect or munmap to race between reclaim unmapping
// the page and flushing the page. If this race occurs, it potentially allows
// access to data via a stale TLB entry. Tracking all mm's that have TLB
// batching in flight would be expensive during reclaim so instead track
// whether TLB batching occurred in the past and if so then do a flush here
// if required. This will cost one additional flush per reclaim cycle paid
// by the first operation at risk such as mprotect and mumap.
//
// This must be called under the PTL so that an access to tlb_flush_batched
// that is potentially a "reclaim vs mprotect/munmap/etc" race will synchronise
// via the PTL.
//
#[no_mangle]
pub unsafe extern "C" fn flush_tlb_batched_pending(mm: *mut mm_struct) {
pub static mut batch: c_int = 0;
pub static mut pending: c_int = 0;
pub static mut flushed: c_int = 0;
    if (pending != flushed) {
    flush_tlb_mm(mm);
//
// If the new TLB flushing is pending during flushing, leave
// mm->tlb_flush_batched as is, to avoid losing flushing.
//
    atomic_cmpxchg(&mm.tlb_flush_batched, batch,
    pending | (pending << TLB_FLUSH_BATCH_FLUSHED_SHIFT));
    }
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: set_tlb_ubc_flush_pending
pub unsafe extern "C" fn set_tlb_ubc_flush_pending_dup(mm: *mut mm_struct, pteval: pte_t, start: c_ulong, end: c_ulong) {
    }
#[no_mangle]
unsafe extern "C" fn should_defer_flush(mm: *mut mm_struct, flags: ttu_flags) -> bool {
    return false;
    }

//
// page_address_in_vma - The virtual address of a page in this VMA.
// @folio: The folio containing the page.
// @page: The page within the folio.
// @vma: The VMA we need to know the address in.
//
// Calculates the user virtual address of this page in the specified VMA.
// It is the caller's responsibility to check the page is actually
// within the VMA.  There may not currently be a PTE pointing at this
// page, but if a page fault occurs at this address, this is the page
// which will be accessed.
//
// Context: Caller should hold a reference to the folio.  Caller should
// hold a lock (eg the i_mmap_lock or the mmap_lock) which keeps the
// VMA from being altered.
//
// Return: The virtual address corresponding to this page in the VMA.
//
#[no_mangle]
pub unsafe extern "C" fn page_address_in_vma(folio: *mut folio, page: *mut page, vma: *mut vm_area_struct) -> c_ulong {
    if (folio_test_anon(folio)) {
    let mut anon_vma = folio_anon_vma(folio);
//
// Note: swapoff's unuse_vma() is more efficient with this
// check, and needs it to match anon_vma when KSM is active.
//
    if (!vma.anon_vma || !anon_vma ||
    vma.anon_vma.root != anon_vma.root) {
    return -EFAULT;
    }
// KSM folios don't reach here because of the !anon_vma check
    return vma_anon_address(vma, page_pgoff(folio, page), 1);
    } else if (!vma.vm_file) {
    return -EFAULT;
    } else if (vma.vm_file.f_mapping != folio.mapping) {
    return -EFAULT;
    }
    return vma_filebacked_address(vma, page_pgoff(folio, page), 1);
    }
//
// Returns the actual pmd_t* where we expect 'address' to be mapped from, or
// NULL if it doesn't exist.  No guarantees / checks on what the pmd_t
// represents.
//
    pmd_t *mm_find_pmd(mm_struct *mm, unsigned long address)
    {
pub static mut pgd: *mut c_void = core::ptr::null_mut();
pub static mut p4d: *mut c_void = core::ptr::null_mut();
pub static mut pud: *mut c_void = core::ptr::null_mut();
    let mut pmd = core::ptr::null_mut();
    pgd = pgd_offset(mm, address);
    if (!pgd_present(*pgd)) {
// goto;
    }
    p4d = p4d_offset(pgd, address);
    if (!p4d_present(*p4d)) {
// goto;
    }
    pud = pud_offset(p4d, address);
    if (!pud_present(*pud)) {
// goto;
    }
    pmd = pmd_offset(pud, address);
// label;
    return pmd;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct folio_referenced_arg {
    pub mapcount: c_int,
    pub referenced: c_int,
    pub vma_flags: vma_flags_t,
    pub memcg: *mut mem_cgroup,
}

//
// arg: folio_referenced_arg will be passed
//
#[no_mangle]
pub unsafe extern "C" fn folio_referenced_one(folio: *mut folio, vma: *mut vm_area_struct, address: c_ulong, arg: *mut c_void) -> bool {
    let mut pra = arg;
pub static mut pvmw: usize = 0;
pub static mut ptes: c_int = 0;
    let mut nr = 0;
    while (page_vma_mapped_walk(&pvmw)) {
    address = pvmw.address;
    nr = 1;
    if (vma_test(vma, VMA_LOCKED_BIT)) {
    ptes += 1;
    pra.mapcount -= 1;
// Only mlock fully mapped pages
    if (pvmw.pte && ptes != pvmw.nr_pages) {
    continue;
    }
//
// All PTEs must be protected by page table lock in
// order to mlock the page.
//
// If page table boundary has been cross, current ptl
// only protect part of ptes.
//
    if (pvmw.flags & PVMW_PGTABLE_CROSSED) {
    continue;
    }
// Restore the mlock which got missed
    mlock_vma_folio(folio, vma);
    page_vma_mapped_walk_done(&pvmw);
    vma_flags_set(&pra.vma_flags, VMA_LOCKED_BIT);
    return false; /* To break the loop */
    }
//
// Skip the non-shared swapbacked folio mapped solely by
// the exiting or OOM-reaped process. This avoids redundant
// swap-out followed by an immediate unmap.
//
    if ((!atomic_read(&vma.vm_mm.mm_users) ||
    check_stable_address_space(vma.vm_mm)) &&
    folio_test_anon(folio) && folio_test_swapbacked(folio) &&
    !folio_maybe_mapped_shared(folio)) {
    pra.referenced = -1;
    page_vma_mapped_walk_done(&pvmw);
    return false;
    }
    if (pvmw.pte && folio_test_large(folio)) {
pub static mut end_addr: c_ulong = 0;
pub static mut max_nr: c_uint = 0;
pub static mut pteval: pte_t = 0;
    nr = folio_pte_batch(folio, pvmw.pte, pteval, max_nr);
    }
//
// When LRU is switching, we don’t know where the surrounding folios
// are. —they could be on active/inactive lists or on MGLRU. So the
// simplest approach is to disable this look-around optimization.
//
    if (lru_gen_enabled() && !lru_gen_switching() && pvmw.pte) {
    if (lru_gen_look_around(&pvmw, nr)) {
    referenced += 1;
    }
    } else if (pvmw.pte) {
    if (clear_flush_young_ptes_notify(vma, address, pvmw.pte, nr)) {
    referenced += 1;
    }
    } else if (IS_ENABLED!(CONFIG_TRANSPARENT_HUGEPAGE)) {
    if (pmdp_clear_flush_young_notify(vma, address,
    pvmw.pmd)) {
    referenced += 1;
    }
    } else {
// unexpected pmd-mapped folio?
    WARN_ON_ONCE!(1);
    }
    ptes += nr;
    pra.mapcount -= nr;
//
// If we are sure that we batched the entire folio,
// we can just optimize and stop right here.
//
    if (ptes == pvmw.nr_pages) {
    page_vma_mapped_walk_done(&pvmw);
    break;
    }
// Skip the batched PTEs
    pvmw.pte += nr - 1;
    pvmw.address += (nr - 1) * PAGE_SIZE;
    }
    if (referenced) {
    folio_clear_idle(folio);
    }
    if (folio_test_clear_young(folio)) {
    referenced += 1;
    }
    if (referenced) {
pub static mut vma_flags: vma_flags_t = 0;
    pra.referenced += 1;
    vma_flags_clear(&vma_flags, VMA_LOCKED_BIT);
    vma_flags_set_mask(&pra.vma_flags, vma_flags);
    }
    if (!pra.mapcount) {
    return false; /* To break the loop */
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn invalid_folio_referenced_vma(vma: *mut vm_area_struct, arg: *mut c_void) -> bool {
    let mut pra = arg;
    let mut memcg = pra.memcg;
//
// Ignore references from this mapping if it has no recency. If the
// folio has been used in another mapping, we will catch it; if this
// other mapping is already gone, the unmap path will have set the
// referenced flag or activated the folio in zap_pte_range().
//
    if (!vma_has_recency(vma)) {
    return true;
    }
//
// If we are reclaiming on behalf of a cgroup, skip counting on behalf
// of references from different cgroups.
//
    if (memcg && !mm_match_cgroup(vma.vm_mm, memcg)) {
    return true;
    }
    return false;
    }
//
// folio_referenced() - Test if the folio was referenced.
// @folio: The folio to test.
// @is_locked: Caller holds lock on the folio.
// @memcg: target memory cgroup
// @vma_flags: A combination of all the vma->flags which referenced the folio.
//
// Quick test_and_clear_referenced for all mappings of a folio,
//
// Return: The number of mappings which referenced the folio. Return -1 if
// the function bailed out due to rmap lock contention.
//
#[no_mangle]
pub unsafe extern "C" fn folio_referenced(folio: *mut folio, is_locked: c_int, memcg: *mut mem_cgroup, vma_flags: *mut vma_flags_t) -> c_int {
pub static mut we_locked: bool = false;
pub static mut folio_referenced_arg: usize = 0;
pub static mut rmap_walk_control: usize = 0;
    VM_WARN_ON_ONCE_FOLIO(folio_is_zone_device(folio), folio);
    vma_flags_clear_all(vma_flags);
    if (!pra.mapcount) {
    return 0;
    }
    if (!folio_raw_mapping(folio)) {
    return 0;
    }
    if (!is_locked) {
    we_locked = folio_trylock(folio);
    if (!we_locked) {
    return 1;
    }
    }
    rmap_walk(folio, &rwc);
    vma_flags_set_mask(vma_flags, pra.vma_flags);
    if (we_locked) {
    folio_unlock(folio);
    }
    return rwc.contended ? -1 : pra.referenced;
    }
#[no_mangle]
unsafe extern "C" fn page_vma_mkclean_one(pvmw: *mut page_vma_mapped_walk) -> c_int {
pub static mut cleaned: c_int = 0;
    let mut vma = pvmw.vma;
pub static mut range: usize = 0;
pub static mut address: c_ulong = 0;
//
// We have to assume the worse case ie pmd for invalidation. Note that
// the folio can not be freed from this function.
//
    mmu_notifier_range_init(&range, MMU_NOTIFY_PROTECTION_PAGE, 0,
    vma.vm_mm, address, vma_address_end(pvmw));
    mmu_notifier_invalidate_range_start(&range);
    while (page_vma_mapped_walk(pvmw)) {
pub static mut ret: c_int = 0;
    address = pvmw.address;
    if (pvmw.pte) {
    let mut pte = pvmw.pte;
pub static mut entry: pte_t = 0;
//
// PFN swap PTEs, such as device-exclusive ones, that
// actually map pages are clean and not writable from a
// CPU perspective. The MMU notifier takes care of any
// device aspects.
//
    if (!pte_present(entry)) {
    continue;
    }
    if (!pte_dirty(entry) && !pte_write(entry)) {
    continue;
    }
    flush_cache_page(vma, address, pte_pfn(entry));
    entry = ptep_clear_flush(vma, address, pte);
    entry = pte_wrprotect(entry);
    entry = pte_mkclean(entry);
    set_pte_at(vma.vm_mm, address, pte, entry);
    ret = 1;
    } else {

    let mut pmd = pvmw.pmd;
pub static mut entry: pmd_t = 0;
//
// Please see the comment above (!pte_present).
// A non present PMD is not writable from a CPU
// perspective.
//
    if (!pmd_present(entry)) {
    continue;
    }
    if (!pmd_dirty(entry) && !pmd_write(entry)) {
    continue;
    }
    flush_cache_range(vma, address,
    address + HPAGE_PMD_SIZE);
    entry = pmdp_invalidate(vma, address, pmd);
    entry = pmd_wrprotect(entry);
    entry = pmd_mkclean(entry);
    set_pmd_at(vma.vm_mm, address, pmd, entry);
    ret = 1;

// unexpected pmd-mapped folio?
    WARN_ON_ONCE!(1);

    }
    if (ret) {
    cleaned += 1;
    }
    }
    mmu_notifier_invalidate_range_end(&range);
    return cleaned;
    }
#[no_mangle]
pub unsafe extern "C" fn page_mkclean_one(folio: *mut folio, vma: *mut vm_area_struct, address: c_ulong, arg: *mut c_void) -> bool {
pub static mut pvmw: usize = 0;
    let mut cleaned = arg;
// cleaned += page_vma_mkclean_one(&pvmw);
    return true;
    }
#[no_mangle]
unsafe extern "C" fn invalid_mkclean_vma(vma: *mut vm_area_struct, arg: *mut c_void) -> bool {
    if (vma.vm_flags & VM_SHARED) {
    return false;
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn folio_mkclean(folio: *mut folio) -> c_int {
pub static mut cleaned: c_int = 0;
pub static mut mapping: *mut c_void = core::ptr::null_mut();
pub static mut rmap_walk_control: usize = 0;
    BUG_ON!(!folio_test_locked(folio));
    if (!folio_mapped(folio)) {
    return 0;
    }
    mapping = folio_mapping(folio);
    if (!mapping) {
    return 0;
    }
    rmap_walk(folio, &rwc);
    return cleaned;
    }
    EXPORT_SYMBOL_GPL(folio_mkclean);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wrprotect_file_state {
    pub cleaned: c_int,
    pub pgoff: pgoff_t,
    pub pfn: c_ulong,
    pub nr_pages: c_ulong,
}

#[no_mangle]
pub unsafe extern "C" fn mapping_wrprotect_range_one(folio: *mut folio, vma: *mut vm_area_struct, address: c_ulong, arg: *mut c_void) -> bool {
    let mut state = arg;
pub static mut page_vma_mapped_walk: usize = 0;
    state.cleaned += page_vma_mkclean_one(&pvmw);
    return true;
    }
// forward_decl: __rmap_walk_file;
//
// mapping_wrprotect_range() - Write-protect all mappings in a specified range.
//
// @mapping:	The mapping whose reverse mapping should be traversed.
// @pgoff:	The page offset at which @pfn is mapped within @mapping.
// @pfn:	The PFN of the page mapped in @mapping at @pgoff.
// @nr_pages:	The number of physically contiguous base pages spanned.
//
// Traverses the reverse mapping, finding all VMAs which contain a shared
// mapping of the pages in the specified range in @mapping, and write-protects
// them (that is, updates the page tables to mark the mappings read-only such
// that a write protection fault arises when the mappings are written to).
//
// The @pfn value need not refer to a folio, but rather can reference a kernel
// allocation which is mapped into userland. We therefore do not require that
// the page maps to a folio with a valid mapping or index field, rather the
// caller specifies these in @mapping and @pgoff.
//
// Return: the number of write-protected PTEs, or an error.
//
#[no_mangle]
pub unsafe extern "C" fn mapping_wrprotect_range(mapping: *mut address_space, pgoff: pgoff_t, pfn: c_ulong, nr_pages: c_ulong) -> c_int {
pub static mut wrprotect_file_state: usize = 0;
pub static mut rmap_walk_control: usize = 0;
    if (!mapping) {
    return 0;
    }
    __rmap_walk_file(/* folio = */core::ptr::null_mut(), mapping, pgoff, nr_pages, &rwc,
// locked = */false);
    return state.cleaned;
    }
    EXPORT_SYMBOL_GPL(mapping_wrprotect_range);
//
// pfn_mkclean_range - Cleans the PTEs (including PMDs) mapped with range of
// [@pfn, @pfn + @nr_pages) at the specific offset (@pgoff)
// within the @vma of shared mappings. And since clean PTEs
// should also be readonly, write protects them too.
// @pfn: start pfn.
// @nr_pages: number of physically contiguous pages srarting with @pfn.
// @pgoff: page offset that the @pfn mapped with.
// @vma: vma that @pfn mapped within.
//
// Returns the number of cleaned PTEs (including PMDs).
//
#[no_mangle]
pub unsafe extern "C" fn pfn_mkclean_range(pfn: c_ulong, nr_pages: c_ulong, pgoff: pgoff_t, vma: *mut vm_area_struct) -> c_int {
pub static mut page_vma_mapped_walk: usize = 0;
    if (invalid_mkclean_vma(vma, core::ptr::null_mut())) {
    return 0;
    }
    pvmw.address = vma_filebacked_address(vma, pgoff, nr_pages);
    VM_BUG_ON_VMA(pvmw.address == -EFAULT, vma);
    return page_vma_mkclean_one(&pvmw);
    }
#[no_mangle]
unsafe extern "C" fn __folio_mod_stat(folio: *mut folio, nr: c_int, nr_pmdmapped: c_int) {
    let mut idx = 0;
    if (nr) {
    idx = folio_test_anon(folio) ? NR_ANON_MAPPED : NR_FILE_MAPPED;
    lruvec_stat_mod_folio(folio, idx, nr);
    }
    if (nr_pmdmapped) {
    if (folio_test_anon(folio)) {
    idx = NR_ANON_THPS;
    lruvec_stat_mod_folio(folio, idx, nr_pmdmapped);
    } else {
// NR_*_PMDMAPPED are not maintained per-memcg
    idx = folio_test_swapbacked(folio) ?
    NR_SHMEM_PMDMAPPED : NR_FILE_PMDMAPPED;
    __mod_node_page_state(folio_pgdat(folio), idx,
    nr_pmdmapped);
    }
    }
    }
    static __always_inline void __folio_add_rmap(folio *folio, page *page, int nr_pages, vm_area_struct *vma,
    enum pgtable_level level)
    {
    let mut mapped = &folio._nr_pages_mapped;
pub static mut orig_nr_pages: c_int = 0;
pub static mut first: c_int = 0;
    __folio_rmap_sanity_checks(folio, page, nr_pages, level);
    match (level) {
    PGTABLE_LEVEL_PTE => {
    if (!folio_test_large(folio)) {
    nr = atomic_inc_and_test(&folio._mapcount);
    // break;
    }
    if (IS_ENABLED!(CONFIG_NO_PAGE_MAPCOUNT)) {
    nr = folio_add_return_large_mapcount(folio, orig_nr_pages, vma);
    if (nr == orig_nr_pages) {
// Was completely unmapped.
    nr = folio_large_nr_pages(folio);
    }
    else {
    nr = 0;
    }
    // break;
    }
    do {
    first += atomic_inc_and_test(&page._mapcount);
    } while (page++, --nr_pages > 0);
    if (first &&
    atomic_add_return_relaxed(first, mapped) < ENTIRELY_MAPPED) {
    nr = first;
    }
    folio_add_large_mapcount(folio, orig_nr_pages, vma);
    // break;
    }
    PGTABLE_LEVEL_PMD => {
    }
    PGTABLE_LEVEL_PUD => {
    first = atomic_inc_and_test(&folio._entire_mapcount);
    if (IS_ENABLED!(CONFIG_NO_PAGE_MAPCOUNT)) {
    if (level == PGTABLE_LEVEL_PMD && first) {
    nr_pmdmapped = folio_large_nr_pages(folio);
    }
    nr = folio_inc_return_large_mapcount(folio, vma);
    if (nr == 1) {
// Was completely unmapped.
    nr = folio_large_nr_pages(folio);
    }
    else {
    nr = 0;
    }
    // break;
    }
    if (first) {
    nr = atomic_add_return_relaxed(ENTIRELY_MAPPED, mapped);
    if (likely(nr < ENTIRELY_MAPPED + ENTIRELY_MAPPED)) {
    nr_pages = folio_large_nr_pages(folio);
//
// We only track PMD mappings of PMD-sized
// folios separately.
//
    if (level == PGTABLE_LEVEL_PMD) {
    nr_pmdmapped = nr_pages;
    }
    nr = nr_pages - (nr & FOLIO_PAGES_MAPPED);
// Raced ahead of a remove and another add?
    if (unlikely(nr < 0)) {
    nr = 0;
    }
    } else {
// Raced ahead of a remove of ENTIRELY_MAPPED
    nr = 0;
    }
    }
    folio_inc_large_mapcount(folio, vma);
    // break;
    }
    _ => {
    BUILD_BUG();
    }
    }
    __folio_mod_stat(folio, nr, nr_pmdmapped);
    }
//
// folio_move_anon_rmap - move a folio to our anon_vma
// @folio:	The folio to move to our anon_vma
// @vma:	The vma the folio belongs to
//
// When a folio belongs exclusively to one process after a COW event,
// that folio can be moved into the anon_vma that belongs to just that
// process, so the rmap code will not search the parent or sibling processes.
//
#[no_mangle]
pub unsafe extern "C" fn folio_move_anon_rmap(folio: *mut folio, vma: *mut vm_area_struct) {
    let mut anon_vma = vma.anon_vma;
    VM_BUG_ON_FOLIO(!folio_test_locked(folio), folio);
    VM_BUG_ON_VMA(!anon_vma, vma);
    anon_vma += FOLIO_MAPPING_ANON;
//
// Ensure that anon_vma and the FOLIO_MAPPING_ANON bit are written
// simultaneously, so a concurrent reader (eg folio_referenced()'s
// folio_test_anon()) will not see one without the other.
//
    WRITE_ONCE(folio.mapping, anon_vma);
    }
//
// __folio_set_anon - set up a new anonymous rmap for a folio
// @folio:	The folio to set up the new anonymous rmap for.
// @vma:	VM area to add the folio to.
// @address:	User virtual address of the mapping
// @exclusive:	Whether the folio is exclusive to the process.
//
#[no_mangle]
pub unsafe extern "C" fn __folio_set_anon(folio: *mut folio, vma: *mut vm_area_struct, address: c_ulong, exclusive: bool) {
    let mut anon_vma = vma.anon_vma;
    BUG_ON!(!anon_vma);
//
// If the folio isn't exclusive to this vma, we must use the _oldest_
// possible anon_vma for the folio mapping!
//
    if (!exclusive) {
    anon_vma = anon_vma.root;
    }
//
// page_idle does a lockless/optimistic rmap scan on folio->mapping.
// Make sure the compiler doesn't split the stores of anon_vma and
// the FOLIO_MAPPING_ANON type identifier, otherwise the rmap code
// could mistake the mapping for a struct address_space and crash.
//
    anon_vma =  anon_vma + FOLIO_MAPPING_ANON;
    WRITE_ONCE(folio.mapping,  anon_vma);
    folio.index = linear_anon_page_index(vma, address);
    }
//
// __page_check_anon_rmap - sanity check anonymous rmap addition
// @folio:	The folio containing @page.
// @page:	the page to check the mapping of
// @vma:	the vm area in which the mapping is added
// @address:	the user virtual address mapped
//
#[no_mangle]
pub unsafe extern "C" fn __page_check_anon_rmap(folio: *mut folio, page: *mut page, vma: *mut vm_area_struct, address: c_ulong) {
//
// The page's anon-rmap details (mapping and index) are guaranteed to
// be set up correctly at this point.
//
// We have exclusion against folio_add_anon_rmap_*() because the caller
// always holds the page locked.
//
// We have exclusion against folio_add_new_anon_rmap because those pages
// are initially only visible via the pagetables, and the pte is locked
// over the call to folio_add_new_anon_rmap.
//
    VM_BUG_ON_FOLIO(folio_anon_vma(folio).root != vma.anon_vma.root,
    folio);
    VM_BUG_ON_PAGE(page_pgoff(folio, page) !=
    linear_anon_page_index(vma, address), page);
    }
    static __always_inline void __folio_add_anon_rmap(folio *folio, page *page, int nr_pages, vm_area_struct *vma,
    unsigned long address, rmap_t flags, enum pgtable_level level)
    {
    let mut i = 0;
    VM_WARN_ON_FOLIO(!folio_test_anon(folio), folio);
    __folio_add_rmap(folio, page, nr_pages, vma, level);
    if (likely(!folio_test_ksm(folio))) {
    __page_check_anon_rmap(folio, page, vma, address);
    }
    if (flags & RMAP_EXCLUSIVE) {
    match (level) {
    PGTABLE_LEVEL_PTE => {
    for (i = 0; i < nr_pages; i++) {
    SetPageAnonExclusive(page + i);
    }
    // break;
    }
    PGTABLE_LEVEL_PMD => {
    SetPageAnonExclusive(page);
    // break;
    }
    PGTABLE_LEVEL_PUD => {
//
// Keep the compiler happy, we don't support anonymous
// PUD mappings.
//
    WARN_ON_ONCE!(1);
    // break;
    }
    _ => {
    BUILD_BUG();
    }
    }
    }
    VM_WARN_ON_FOLIO(!folio_test_large(folio) && PageAnonExclusive(page) &&
    atomic_read(&folio._mapcount) > 0, folio);
    while (i < nr_pages) {
    let mut cur_page = page + i;
    VM_WARN_ON_FOLIO(folio_test_large(folio) &&
    folio_entire_mapcount(folio) > 1 &&
    PageAnonExclusive(cur_page), folio);
    if (IS_ENABLED!(CONFIG_NO_PAGE_MAPCOUNT)) {
    continue;
    }
//
// While PTE-mapping a THP we have a PMD and a PTE
// mapping.
//
    VM_WARN_ON_FOLIO(atomic_read(&cur_page._mapcount) > 0 &&
    PageAnonExclusive(cur_page), folio);
    }
//
// Only mlock it if the folio is fully mapped to the VMA.
//
// Partially mapped folios can be split on reclaim and part outside
// of mlocked VMA can be evicted or freed.
//
    if (folio_nr_pages(folio) == nr_pages) {
    mlock_vma_folio(folio, vma);
    }
    }
//
// folio_add_anon_rmap_ptes - add PTE mappings to a page range of an anon folio
// @folio:	The folio to add the mappings to
// @page:	The first page to add
// @nr_pages:	The number of pages which will be mapped
// @vma:	The vm area in which the mappings are added
// @address:	The user virtual address of the first page to map
// @flags:	The rmap flags
//
// The page range of folio is defined by [first_page, first_page + nr_pages)
//
// The caller needs to hold the page table lock, and the page must be locked in
// the anon_vma case: to serialize mapping,index checking after setting,
// and to ensure that an anon folio is not being upgraded racily to a KSM folio
// (but KSM folios are never downgraded).
//
#[no_mangle]
pub unsafe extern "C" fn folio_add_anon_rmap_ptes(folio: *mut folio, page: *mut page, nr_pages: c_int, vma: *mut vm_area_struct, address: c_ulong, flags: rmap_t) {
    __folio_add_anon_rmap(folio, page, nr_pages, vma, address, flags,
    PGTABLE_LEVEL_PTE);
    }
//
// folio_add_anon_rmap_pmd - add a PMD mapping to a page range of an anon folio
// @folio:	The folio to add the mapping to
// @page:	The first page to add
// @vma:	The vm area in which the mapping is added
// @address:	The user virtual address of the first page to map
// @flags:	The rmap flags
//
// The page range of folio is defined by [first_page, first_page + HPAGE_PMD_NR)
//
// The caller needs to hold the page table lock, and the page must be locked in
// the anon_vma case: to serialize mapping,index checking after setting.
//
#[no_mangle]
pub unsafe extern "C" fn folio_add_anon_rmap_pmd(folio: *mut folio, page: *mut page, vma: *mut vm_area_struct, address: c_ulong, flags: rmap_t) {

    __folio_add_anon_rmap(folio, page, HPAGE_PMD_NR, vma, address, flags,
    PGTABLE_LEVEL_PMD);

    WARN_ON_ONCE!(true);

    }
//
// folio_add_new_anon_rmap - Add mapping to a new anonymous folio.
// @folio:	The folio to add the mapping to.
// @vma:	the vm area in which the mapping is added
// @address:	the user virtual address mapped
// @flags:	The rmap flags
//
// Like folio_add_anon_rmap_*() but must only be called on *new* folios.
// This means the inc-and-test can be bypassed.
// The folio doesn't necessarily need to be locked while it's exclusive
// unless two threads map it concurrently. However, the folio must be
// locked if it's shared.
//
// If the folio is pmd-mappable, it is accounted as a THP.
//
#[no_mangle]
pub unsafe extern "C" fn folio_add_new_anon_rmap(folio: *mut folio, vma: *mut vm_area_struct, address: c_ulong, flags: rmap_t) {
pub static mut exclusive: bool = false;
pub static mut nr: c_int = 0;
    VM_WARN_ON_FOLIO(folio_test_hugetlb(folio), folio);
    VM_WARN_ON_FOLIO(!exclusive && !folio_test_locked(folio), folio);
//
// VM_DROPPABLE mappings don't swap; instead they're just dropped when
// under memory pressure.
//
    if (!folio_test_swapbacked(folio) && !(vma.vm_flags & VM_DROPPABLE)) {
    __folio_set_swapbacked(folio);
    }
    __folio_set_anon(folio, vma, address, exclusive);
    if (likely(!folio_test_large(folio))) {
// increment count (starts at -1)
    atomic_set(&folio._mapcount, 0);
    if (exclusive) {
    SetPageAnonExclusive(&folio.page);
    }
    } else if (!folio_test_pmd_mappable(folio)) {
    let mut i = 0;
    nr = folio_large_nr_pages(folio);
    while (i < nr) {
    let mut page = folio_page(folio, i);
    if (IS_ENABLED!(CONFIG_PAGE_MAPCOUNT)) {
// increment count (starts at -1)
    atomic_set(&page._mapcount, 0);
    }
    if (exclusive) {
    SetPageAnonExclusive(page);
    }
    }
    folio_set_large_mapcount(folio, nr, vma);
    if (IS_ENABLED!(CONFIG_PAGE_MAPCOUNT)) {
    atomic_set(&folio._nr_pages_mapped, nr);
    }
    } else {
    nr = folio_large_nr_pages(folio);
// increment count (starts at -1)
    atomic_set(&folio._entire_mapcount, 0);
    folio_set_large_mapcount(folio, 1, vma);
    if (IS_ENABLED!(CONFIG_PAGE_MAPCOUNT)) {
    atomic_set(&folio._nr_pages_mapped, ENTIRELY_MAPPED);
    }
    if (exclusive) {
    SetPageAnonExclusive(&folio.page);
    }
    nr_pmdmapped = nr;
    }
    VM_WARN_ON_ONCE(address < vma.vm_start ||
    address + (nr << PAGE_SHIFT) > vma.vm_end);
    __folio_mod_stat(folio, nr, nr_pmdmapped);
    mod_mthp_stat(folio_order(folio), MTHP_STAT_NR_ANON, 1);
    }
    static __always_inline void __folio_add_file_rmap(folio *folio, page *page, int nr_pages, vm_area_struct *vma,
    enum pgtable_level level)
    {
    VM_WARN_ON_FOLIO(folio_test_anon(folio), folio);
    __folio_add_rmap(folio, page, nr_pages, vma, level);
//
// Only mlock it if the folio is fully mapped to the VMA.
//
// Partially mapped folios can be split on reclaim and part outside
// of mlocked VMA can be evicted or freed.
//
    if (folio_nr_pages(folio) == nr_pages) {
    mlock_vma_folio(folio, vma);
    }
    }
//
// folio_add_file_rmap_ptes - add PTE mappings to a page range of a folio
// @folio:	The folio to add the mappings to
// @page:	The first page to add
// @nr_pages:	The number of pages that will be mapped using PTEs
// @vma:	The vm area in which the mappings are added
//
// The page range of the folio is defined by [page, page + nr_pages)
//
// The caller needs to hold the page table lock.
//
#[no_mangle]
pub unsafe extern "C" fn folio_add_file_rmap_ptes(folio: *mut folio, page: *mut page, nr_pages: c_int, vma: *mut vm_area_struct) {
    __folio_add_file_rmap(folio, page, nr_pages, vma, PGTABLE_LEVEL_PTE);
    }
//
// folio_add_file_rmap_pmd - add a PMD mapping to a page range of a folio
// @folio:	The folio to add the mapping to
// @page:	The first page to add
// @vma:	The vm area in which the mapping is added
//
// The page range of the folio is defined by [page, page + HPAGE_PMD_NR)
//
// The caller needs to hold the page table lock.
//
#[no_mangle]
pub unsafe extern "C" fn folio_add_file_rmap_pmd(folio: *mut folio, page: *mut page, vma: *mut vm_area_struct) {

    __folio_add_file_rmap(folio, page, HPAGE_PMD_NR, vma, PGTABLE_LEVEL_PMD);

    WARN_ON_ONCE!(true);

    }
//
// folio_add_file_rmap_pud - add a PUD mapping to a page range of a folio
// @folio:	The folio to add the mapping to
// @page:	The first page to add
// @vma:	The vm area in which the mapping is added
//
// The page range of the folio is defined by [page, page + HPAGE_PUD_NR)
//
// The caller needs to hold the page table lock.
//
#[no_mangle]
pub unsafe extern "C" fn folio_add_file_rmap_pud(folio: *mut folio, page: *mut page, vma: *mut vm_area_struct) {

    defined(CONFIG_HAVE_ARCH_TRANSPARENT_HUGEPAGE_PUD)
    __folio_add_file_rmap(folio, page, HPAGE_PUD_NR, vma, PGTABLE_LEVEL_PUD);

    WARN_ON_ONCE!(true);

    }
    static __always_inline void __folio_remove_rmap(folio *folio, page *page, int nr_pages, vm_area_struct *vma,
    enum pgtable_level level)
    {
    let mut mapped = &folio._nr_pages_mapped;
pub static mut last: c_int = 0;
pub static mut partially_mapped: bool = false;
    __folio_rmap_sanity_checks(folio, page, nr_pages, level);
    match (level) {
    PGTABLE_LEVEL_PTE => {
    if (!folio_test_large(folio)) {
    nr = atomic_add_negative(-1, &folio._mapcount);
    // break;
    }
    if (IS_ENABLED!(CONFIG_NO_PAGE_MAPCOUNT)) {
    nr = folio_sub_return_large_mapcount(folio, nr_pages, vma);
    if (!nr) {
// Now completely unmapped.
    nr = folio_large_nr_pages(folio);
    } else {
    partially_mapped = nr < folio_large_nr_pages(folio) &&
    !folio_entire_mapcount(folio);
    nr = 0;
    }
    // break;
    }
    folio_sub_large_mapcount(folio, nr_pages, vma);
    do {
    last += atomic_add_negative(-1, &page._mapcount);
    } while (page++, --nr_pages > 0);
    if (last &&
    atomic_sub_return_relaxed(last, mapped) < ENTIRELY_MAPPED) {
    nr = last;
    }
    partially_mapped = nr && atomic_read(mapped);
    // break;
    }
    PGTABLE_LEVEL_PMD => {
    }
    PGTABLE_LEVEL_PUD => {
    if (IS_ENABLED!(CONFIG_NO_PAGE_MAPCOUNT)) {
    last = atomic_add_negative(-1, &folio._entire_mapcount);
    if (level == PGTABLE_LEVEL_PMD && last) {
    nr_pmdmapped = folio_large_nr_pages(folio);
    }
    nr = folio_dec_return_large_mapcount(folio, vma);
    if (!nr) {
// Now completely unmapped.
    nr = folio_large_nr_pages(folio);
    } else {
    partially_mapped = last &&
    nr < folio_large_nr_pages(folio);
    nr = 0;
    }
    // break;
    }
    folio_dec_large_mapcount(folio, vma);
    last = atomic_add_negative(-1, &folio._entire_mapcount);
    if (last) {
    nr = atomic_sub_return_relaxed(ENTIRELY_MAPPED, mapped);
    if (likely(nr < ENTIRELY_MAPPED)) {
    nr_pages = folio_large_nr_pages(folio);
    if (level == PGTABLE_LEVEL_PMD) {
    nr_pmdmapped = nr_pages;
    }
    nr = nr_pages - nr;
// Raced ahead of another remove and an add?
    if (unlikely(nr < 0)) {
    nr = 0;
    }
    } else {
// An add of ENTIRELY_MAPPED raced ahead
    nr = 0;
    }
    }
    partially_mapped = nr && nr < nr_pmdmapped;
    // break;
    }
    _ => {
    BUILD_BUG();
    }
    }
//
// Queue anon large folio for deferred split if at least one page of
// the folio is unmapped and at least one page is still mapped.
//
// Check partially_mapped first to ensure it is a large folio.
//
// Device private folios do not support deferred splitting and
// shrinker based scanning of the folios to free.
//
    if (partially_mapped && folio_test_anon(folio) &&
    !folio_test_partially_mapped(folio) &&
    !folio_is_device_private(folio)) {
    deferred_split_folio(folio, true);
    }
    __folio_mod_stat(folio, -nr, -nr_pmdmapped);
//
// It would be tidy to reset folio_test_anon mapping when fully
// unmapped, but that might overwrite a racing folio_add_anon_rmap_*()
// which increments mapcount after us but sets mapping before us:
// so leave the reset to free_pages_prepare, and remember that
// it's only reliable while mapped.
//
    munlock_vma_folio(folio, vma);
    }
//
// folio_remove_rmap_ptes - remove PTE mappings from a page range of a folio
// @folio:	The folio to remove the mappings from
// @page:	The first page to remove
// @nr_pages:	The number of pages that will be removed from the mapping
// @vma:	The vm area from which the mappings are removed
//
// The page range of the folio is defined by [page, page + nr_pages)
//
// The caller needs to hold the page table lock.
//
#[no_mangle]
pub unsafe extern "C" fn folio_remove_rmap_ptes(folio: *mut folio, page: *mut page, nr_pages: c_int, vma: *mut vm_area_struct) {
    __folio_remove_rmap(folio, page, nr_pages, vma, PGTABLE_LEVEL_PTE);
    }
//
// folio_remove_rmap_pmd - remove a PMD mapping from a page range of a folio
// @folio:	The folio to remove the mapping from
// @page:	The first page to remove
// @vma:	The vm area from which the mapping is removed
//
// The page range of the folio is defined by [page, page + HPAGE_PMD_NR)
//
// The caller needs to hold the page table lock.
//
#[no_mangle]
pub unsafe extern "C" fn folio_remove_rmap_pmd(folio: *mut folio, page: *mut page, vma: *mut vm_area_struct) {

    __folio_remove_rmap(folio, page, HPAGE_PMD_NR, vma, PGTABLE_LEVEL_PMD);

    WARN_ON_ONCE!(true);

    }
//
// folio_remove_rmap_pud - remove a PUD mapping from a page range of a folio
// @folio:	The folio to remove the mapping from
// @page:	The first page to remove
// @vma:	The vm area from which the mapping is removed
//
// The page range of the folio is defined by [page, page + HPAGE_PUD_NR)
//
// The caller needs to hold the page table lock.
//
#[no_mangle]
pub unsafe extern "C" fn folio_remove_rmap_pud(folio: *mut folio, page: *mut page, vma: *mut vm_area_struct) {

    defined(CONFIG_HAVE_ARCH_TRANSPARENT_HUGEPAGE_PUD)
    __folio_remove_rmap(folio, page, HPAGE_PUD_NR, vma, PGTABLE_LEVEL_PUD);

    WARN_ON_ONCE!(true);

    }
#[no_mangle]
pub unsafe extern "C" fn folio_unmap_pte_batch(folio: *mut folio, pvmw: *mut page_vma_mapped_walk, flags: ttu_flags, pte: pte_t) -> c_uint {
    unsigned long end_addr, addr = pvmw.address;
    let mut vma = pvmw.vma;
    let mut max_nr = 0;
    if (flags & TTU_HWPOISON) {
    return 1;
    }
    if (!folio_test_large(folio)) {
    return 1;
    }
// We may only batch within a single VMA and a single page table.
    end_addr = pmd_addr_end(addr, vma.vm_end);
    max_nr = (end_addr - addr) >> PAGE_SHIFT;
// We only support lazyfree or file folios batching for now ...
    if (folio_test_anon(folio) && folio_test_swapbacked(folio)) {
    return 1;
    }
    if (pte_unused(pte)) {
    return 1;
    }
//
// If unmap fails, we need to restore the ptes. To avoid accidentally
// upgrading write permissions for ptes that were not originally
// writable, and to avoid losing the soft-dirty bit, use the
// appropriate FPB flags.
//
    return folio_pte_batch_flags(folio, vma, pvmw.pte, &pte, max_nr,
    FPB_RESPECT_WRITE | FPB_RESPECT_SOFT_DIRTY);
    }
#[no_mangle]
pub unsafe extern "C" fn try_to_unmap_poisoned_hugetlb_one(folio: *mut folio, vma: *mut vm_area_struct, address: c_ulong, arg: *mut c_void) -> bool {
pub static mut pvmw: usize = 0;
pub static mut hsz: c_ulong = 0;
pub static mut flags: ttu_flags = 0;
    let mut mm = vma.vm_mm;
pub static mut range: usize = 0;
pub static mut ret: bool = true;
    let mut pteval;
//
// The try_to_unmap() is only passed a hugetlb folio in the case
// where the hugetlb folio is poisoned.
//
    VM_WARN_ON_ONCE_FOLIO(!folio_test_hwpoison(folio), folio);
    VM_WARN_ON_ONCE(!(flags & TTU_HWPOISON));
    range.end = vma_address_end(&pvmw);
    mmu_notifier_range_init(&range, MMU_NOTIFY_CLEAR, 0, vma.vm_mm,
    address, range.end);
    adjust_range_if_pmd_sharing_possible(vma, &range.start, &range.end);
    mmu_notifier_invalidate_range_start(&range);
// There is only a single mapping in a VMA.
    if (!page_vma_mapped_walk(&pvmw)) {
// goto;
    }
    VM_WARN_ON_ONCE(address != pvmw.address);
    pteval = huge_ptep_get(mm, address, pvmw.pte);
    VM_WARN_ON_ONCE(!pte_present(pteval));
    VM_WARN_ON_ONCE(pte_pfn(pteval) != folio_pfn(folio));
//
// huge_pmd_unshare may unmap an entire PMD page. There is no way of
// knowing exactly which PMDs may be cached for this mm, so we must
// flush them all. start/end were already adjusted above to cover this
// range.
//
    flush_cache_range(vma, range.start, range.end);
//
// To call huge_pmd_unshare, i_mmap_rwsem must be held in write mode.
// Caller needs to explicitly do this outside rmap routines.
//
// We also must hold hugetlb vma_lock in write mode. Lock order dictates
// acquiring vma_lock BEFORE i_mmap_rwsem. We can only try lock here and
// fail if unsuccessful.
//
    if (!folio_test_anon(folio)) {
pub static mut tlb: usize = 0;
    VM_WARN_ON_ONCE(!(flags & TTU_RMAP_LOCKED));
    if (!hugetlb_vma_trylock_write(vma)) {
    ret = false;
// goto;
    }
    tlb_gather_mmu_vma(&tlb, vma);
    if (huge_pmd_unshare(&tlb, vma, address, pvmw.pte)) {
    hugetlb_vma_unlock_write(vma);
    huge_pmd_unshare_flush(&tlb, vma);
    tlb_finish_mmu(&tlb);
//
// The PMD table was unmapped, consequently unmapping
// the folio.
//
// goto;
    }
    hugetlb_vma_unlock_write(vma);
    tlb_finish_mmu(&tlb);
    }
    pteval = huge_ptep_clear_flush(vma, address, pvmw.pte);
    if (huge_pte_dirty(pteval)) {
    folio_mark_dirty(folio);
    }
    pteval = swp_entry_to_pte(make_hwpoison_entry(folio_page(folio, 0)));
    hugetlb_count_sub(folio_nr_pages(folio), mm);
    set_huge_pte_at(mm, address, pvmw.pte, pteval, hsz);
    hugetlb_remove_rmap(folio);
    folio_put_refs(folio, 1);
// label;
    page_vma_mapped_walk_done(&pvmw);
// label;
    mmu_notifier_invalidate_range_end(&range);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn ttu_anon_lazyfree_folio(vma: *mut vm_area_struct, folio: *mut folio, nr_pages: c_ulong) -> bool {
    let mut ref_count = 0;
    let mut map_count = 0;
//
// Synchronize with gup_pte_range():
// - clear PTE; barrier; read refcount
// - inc refcount; barrier; read PTE
//
    smp_mb();
    ref_count = folio_ref_count(folio);
    map_count = folio_mapcount(folio);
//
// Order reads for page refcount and dirty flag
// (see comments in __remove_mapping()).
//
    smp_rmb();
    if (folio_test_dirty(folio) && !(vma.vm_flags & VM_DROPPABLE)) {
//
// redirtied either using the page table or a previously
// obtained GUP reference.
//
    folio_set_swapbacked(folio);
    return false;
    }
//
// Additional references could be due to GUP or speculative lookups.
// GUP users must mark the folio dirty if there was a modification.
// This folio cannot be reclaimed right now either way, so act just
// like nothing happened. We'll come back here later and detect if the
// folio was dirtied when the additional reference is gone.
//
    if (ref_count != 1 + map_count) {
    return false;
    }
    add_mm_counter(vma.vm_mm, MM_ANONPAGES, -nr_pages);
    return true;
    }
    static pte_t swp_pte_prepare(swp_entry_t entry, pte_t old_pte,
    bool anon_exclusive)
    {
pub static mut swp_pte: pte_t = 0;
    if (anon_exclusive) {
    swp_pte = pte_swp_mkexclusive(swp_pte);
    }
    if (likely(pte_present(old_pte))) {
    if (pte_soft_dirty(old_pte)) {
    swp_pte = pte_swp_mksoft_dirty(swp_pte);
    }
    if (pte_uffd(old_pte)) {
    swp_pte = pte_swp_mkuffd(swp_pte);
    }
    } else {
// Device-exclusive entry
    if (pte_swp_soft_dirty(old_pte)) {
    swp_pte = pte_swp_mksoft_dirty(swp_pte);
    }
    if (pte_swp_uffd(old_pte)) {
    swp_pte = pte_swp_mkuffd(swp_pte);
    }
    }
    return swp_pte;
    }
#[no_mangle]
pub unsafe extern "C" fn ttu_anon_swapbacked_folio(vma: *mut vm_area_struct, folio: *mut folio, page: *mut page, address: c_ulong, ptep: *mut pte_t, pteval: pte_t) -> bool {
    let mut anon_exclusive = folio_test_anon(folio) &&
    PageAnonExclusive(page);
pub static mut entry: swp_entry_t = 0;
    let mut mm = vma.vm_mm;
    if (folio_dup_swap(folio, page) < 0) {
    return false;
    }
//
// arch_unmap_one() is expected to be a NOP on
// architectures where we could have PFN swap PTEs,
// so we'll not check/care.
//
    if (arch_unmap_one(mm, vma, address, pteval) < 0) {
    folio_put_swap(folio, page);
    return false;
    }
// See folio_try_share_anon_rmap(): clear PTE first.
    if (anon_exclusive && folio_try_share_anon_rmap_pte(folio, page)) {
    folio_put_swap(folio, page);
    return false;
    }
    mm_prepare_for_swap_entries(mm);
    dec_mm_counter(mm, MM_ANONPAGES);
    inc_mm_counter(mm, MM_SWAPENTS);
    set_pte_at(mm, address, ptep,
    swp_pte_prepare(entry, pteval, anon_exclusive));
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn ttu_anon_folio(vma: *mut vm_area_struct, folio: *mut folio, page: *mut page, address: c_ulong, ptep: *mut pte_t, pteval: pte_t, nr_pages: c_ulong) -> bool {
//
// Store the swap location in the pte.
// See handle_pte_fault() ...
//
    if (WARN_ON_ONCE!(folio_test_swapbacked(folio) !=
    folio_test_swapcache(folio))) {
    return false;
    }
    if (!folio_test_swapbacked(folio)) {
    return ttu_anon_lazyfree_folio(vma, folio, nr_pages);
    }
// nr_pages > 1 not supported yet
    return ttu_anon_swapbacked_folio(vma, folio, page, address, ptep,
    pteval);
    }
//
// @arg: ttu_flags will be passed to this argument
//
#[no_mangle]
pub unsafe extern "C" fn try_to_unmap_one(folio: *mut folio, vma: *mut vm_area_struct, address: c_ulong, arg: *mut c_void) -> bool {
    let mut mm = vma.vm_mm;
pub static mut pvmw: usize = 0;
pub static mut ret: bool = true;
    let mut pteval;
pub static mut page: *mut c_void = core::ptr::null_mut();
pub static mut range: usize = 0;
pub static mut flags: ttu_flags = 0;
pub static mut nr_pages: c_ulong = 0;
    let mut pfn = 0;
pub static mut ptes: c_int = 0;
//
// When racing against e.g. zap_pte_range() on another cpu,
// in between its ptep_get_and_clear_full() and folio_remove_rmap_*(),
// try_to_unmap() may return before folio_mapped() has become false,
// if page table locking is skipped: use TTU_SYNC to wait for that.
//
    if (flags & TTU_SYNC) {
    pvmw.flags = PVMW_SYNC;
    }
//
// For THP, we have to assume the worse case ie pmd for invalidation.
//
// Note that the folio can not be freed in this function as call of
// try_to_unmap() must hold a reference on the folio.
//
    range.end = vma_address_end(&pvmw);
    mmu_notifier_range_init(&range, MMU_NOTIFY_CLEAR, 0, vma.vm_mm,
    address, range.end);
    mmu_notifier_invalidate_range_start(&range);
    while (page_vma_mapped_walk(&pvmw)) {
    nr_pages = 1;
//
// If the folio is in an mlock()d vma, we must not swap it out.
//
    if (!(flags & TTU_IGNORE_MLOCK) &&
    (vma.vm_flags & VM_LOCKED)) {
    ptes += 1;
//
// Set 'ret' to indicate the page cannot be unmapped.
//
// Do not jump to walk_abort immediately as additional
// iteration might be required to detect fully mapped
// folio an mlock it.
//
    ret = false;
// Only mlock fully mapped pages
    if (pvmw.pte && ptes != pvmw.nr_pages) {
    continue;
    }
//
// All PTEs must be protected by page table lock in
// order to mlock the page.
//
// If page table boundary has been cross, current ptl
// only protect part of ptes.
//
    if (pvmw.flags & PVMW_PGTABLE_CROSSED) {
// goto;
    }
// Restore the mlock which got missed
    mlock_vma_folio(folio, vma);
// goto;
    }
    if (!pvmw.pte) {
    if (folio_test_lazyfree(folio)) {
    if (unmap_huge_pmd_locked(vma, pvmw.address, pvmw.pmd, folio)) {
// goto;
    }
//
// unmap_huge_pmd_locked has either already marked
// the folio as swap-backed or decided to retain it
// due to GUP or speculative references.
//
// goto;
    }
    if (flags & TTU_SPLIT_HUGE_PMD) {
//
// We temporarily have to drop the PTL and
// restart so we can process the PTE-mapped THP.
//
    split_huge_pmd_locked(vma, pvmw.address,
    pvmw.pmd, false);
    flags &= ~TTU_SPLIT_HUGE_PMD;
    page_vma_mapped_walk_restart(&pvmw);
    continue;
    }
    }
// Unexpected PMD-mapped THP?
    VM_BUG_ON_FOLIO(!pvmw.pte, folio);
    address = pvmw.address;
    if (folio_test_hugetlb(folio)) {
    pteval = huge_ptep_get(mm, address, pvmw.pte);
    } else {
    pteval = ptep_get(pvmw.pte);
    }
    if (likely(pte_present(pteval))) {
    pfn = pte_pfn(pteval);
    } else {
//
// Handle PFN swap PTEs, such as device-exclusive ones,
// that actually map pages.
//
pub static mut entry: softleaf_t = 0;
    pfn = softleaf_to_pfn(entry);
    }
    page = folio_page(folio, pfn - folio_pfn(folio));
    if (likely(pte_present(pteval))) {
    nr_pages = folio_unmap_pte_batch(folio, &pvmw, flags, pteval);
    end_addr = address + nr_pages * PAGE_SIZE;
    flush_cache_range(vma, address, end_addr);
// Nuke the page table entry.
    pteval = get_and_clear_ptes(mm, address, pvmw.pte, nr_pages);
//
// We clear the PTE but do not flush so potentially
// a remote CPU could still be writing to the folio.
// If the entry was previously clean then the
// architecture must guarantee that a clear->dirty
// transition on a cached TLB entry is written through
// and traps if the PTE is unmapped.
//
    if (should_defer_flush(mm, flags)) {
    set_tlb_ubc_flush_pending(mm, pteval, address, end_addr);
    }
    else {
    flush_tlb_range(vma, address, end_addr);
    }
    if (pte_dirty(pteval)) {
    folio_mark_dirty(folio);
    }
    } else {
    pte_clear(mm, address, pvmw.pte);
    }
//
// Now the pte is cleared. If this pte was uffd-wp armed,
// we may want to replace a none pte with a marker pte if
// it's file-backed, so we don't lose the tracking info.
//
    cond_install_uffd_wp_ptes(vma, address, pvmw.pte, pteval,
    nr_pages);
// Update high watermark before we lower rss
    update_hiwater_rss(mm);
// unmap_poisoned_folio() only refs order-0 folios
    if (folio_test_hwpoison(folio) && (flags & TTU_HWPOISON)) {
    pteval = swp_entry_to_pte(make_hwpoison_entry(page));
    dec_mm_counter(mm, mm_counter(folio));
    set_pte_at(mm, address, pvmw.pte, pteval);
    } else if (likely(pte_present(pteval)) && pte_unused(pteval) &&
    !userfaultfd_armed(vma)) {
//
// The guest indicated that the page content is of no
// interest anymore. Simply discard the pte, vmscan
// will take care of the rest.
// A future reference will then fault in a new zero
// page. When userfaultfd is active, we must not drop
// this page though, as its main user (postcopy
// migration) will not expect userfaults on already
// copied pages.
//
    dec_mm_counter(mm, mm_counter(folio));
    } else if (folio_test_anon(folio)) {
    if (!ttu_anon_folio(vma, folio, page, address,
    pvmw.pte, pteval, nr_pages)) {
    set_ptes(mm, address, pvmw.pte, pteval, nr_pages);
// goto;
    }
// goto;
    } else {
//
// This is a locked file-backed folio,
// so it cannot be removed from the page
// cache and replaced by a new folio before
// mmu_notifier_invalidate_range_end, so no
// concurrent thread might update its page table
// to point at a new folio while a device is
// still using this folio.
//
// See Documentation/mm/mmu_notifier.rst
//
    add_mm_counter(mm, mm_counter_file(folio), -nr_pages);
    }
// label;
    folio_remove_rmap_ptes(folio, page, nr_pages, vma);
    if (vma.vm_flags & VM_LOCKED) {
    mlock_drain_local();
    }
    folio_put_refs(folio, nr_pages);
//
// If we are sure that we batched the entire folio and cleared
// all PTEs, we can just optimize and stop right here.
//
    if (nr_pages == folio_nr_pages(folio)) {
// goto;
    }
    continue;
// label;
    ret = false;
// label;
    page_vma_mapped_walk_done(&pvmw);
    break;
    }
    mmu_notifier_invalidate_range_end(&range);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn invalid_migration_vma(vma: *mut vm_area_struct, arg: *mut c_void) -> bool {
    return vma_is_temporary_stack(vma);
    }
#[no_mangle]
unsafe extern "C" fn folio_not_mapped(folio: *mut folio) -> c_int {
    return !folio_mapped(folio);
    }
//
// try_to_unmap - Try to remove all page table mappings to a folio.
// @folio: The folio to unmap.
// @flags: action and flags
//
// Tries to remove all the page table entries which are mapping this
// folio.  It is the caller's responsibility to check if the folio is
// still mapped if needed (use TTU_SYNC to prevent accounting races).
//
// Context: Caller must hold the folio lock.
//
#[no_mangle]
pub unsafe extern "C" fn try_to_unmap(folio: *mut folio, flags: ttu_flags) {
pub static mut rmap_walk_control: usize = 0;
    if (flags & TTU_RMAP_LOCKED) {
    rmap_walk_locked(folio, &rwc);
    }
    else {
    rmap_walk(folio, &rwc);
    }
    }
//
// @arg: ttu_flags will be passed to this argument.
//
// If TTU_SPLIT_HUGE_PMD is specified any PMD mappings will be split into PTEs
// containing migration entries.
//
#[no_mangle]
pub unsafe extern "C" fn try_to_migrate_one(folio: *mut folio, vma: *mut vm_area_struct, address: c_ulong, arg: *mut c_void) -> bool {
    let mut mm = vma.vm_mm;
pub static mut pvmw: usize = 0;
    bool anon_exclusive, writable, ret = true;
    let mut pteval;
pub static mut subpage: *mut c_void = core::ptr::null_mut();
pub static mut range: usize = 0;
pub static mut flags: ttu_flags = 0;
    let mut pfn = 0;
pub static mut hsz: c_ulong = 0;
//
// When racing against e.g. zap_pte_range() on another cpu,
// in between its ptep_get_and_clear_full() and folio_remove_rmap_*(),
// try_to_migrate() may return before folio_mapped() has become false,
// if page table locking is skipped: use TTU_SYNC to wait for that.
//
    if (flags & TTU_SYNC) {
    pvmw.flags = PVMW_SYNC;
    }
//
// For THP, we have to assume the worse case ie pmd for invalidation.
// For hugetlb, it could be much worse if we need to do pud
// invalidation in the case of pmd sharing.
//
// Note that the page can not be free in this function as call of
// try_to_unmap() must hold a reference on the page.
//
    range.end = vma_address_end(&pvmw);
    mmu_notifier_range_init(&range, MMU_NOTIFY_CLEAR, 0, vma.vm_mm,
    address, range.end);
    if (folio_test_hugetlb(folio)) {
//
// If sharing is possible, start and end will be adjusted
// accordingly.
//
    adjust_range_if_pmd_sharing_possible(vma, &range.start,
    &range.end);
// We need the huge page size for set_huge_pte_at()
    hsz = huge_page_size(hstate_vma(vma));
    }
    mmu_notifier_invalidate_range_start(&range);
    while (page_vma_mapped_walk(&pvmw)) {
// PMD-mapped THP migration entry
    if (!pvmw.pte) {
    __maybe_unused unsigned long pfn;
    __maybe_unused pmd_t pmdval;
    if (flags & TTU_SPLIT_HUGE_PMD) {
//
// split_huge_pmd_locked() might leave the
// folio mapped through PTEs. Retry the walk
// so we can detect this scenario and properly
// abort the walk.
//
    split_huge_pmd_locked(vma, pvmw.address,
    pvmw.pmd, true);
    flags &= ~TTU_SPLIT_HUGE_PMD;
    page_vma_mapped_walk_restart(&pvmw);
    continue;
    }

    pmdval = pmdp_get(pvmw.pmd);
    if (likely(pmd_present(pmdval))) {
    pfn = pmd_pfn(pmdval);
    }
    else {
    pfn = softleaf_to_pfn(softleaf_from_pmd(pmdval));
    }
    subpage = folio_page(folio, pfn - folio_pfn(folio));
    VM_BUG_ON_FOLIO(folio_test_hugetlb(folio) ||
    !folio_test_pmd_mappable(folio), folio);
    if (set_pmd_migration_entry(&pvmw, subpage)) {
    ret = false;
    page_vma_mapped_walk_done(&pvmw);
    break;
    }
    continue;

    }
// Unexpected PMD-mapped THP?
    VM_BUG_ON_FOLIO(!pvmw.pte, folio);
    address = pvmw.address;
    if (folio_test_hugetlb(folio)) {
    pteval = huge_ptep_get(mm, address, pvmw.pte);
    }
    else {
    pteval = ptep_get(pvmw.pte);
    }
    if (likely(pte_present(pteval))) {
    pfn = pte_pfn(pteval);
    } else {
//
// Handle PFN swap PTEs, such as device-exclusive ones,
// that actually map pages.
//
pub static mut entry: softleaf_t = 0;
    pfn = softleaf_to_pfn(entry);
    VM_WARN_ON_FOLIO(folio_test_hugetlb(folio), folio);
    }
    subpage = folio_page(folio, pfn - folio_pfn(folio));
    anon_exclusive = folio_test_anon(folio) &&
    PageAnonExclusive(subpage);
    if (folio_test_hugetlb(folio)) {
pub static mut anon: bool = false;
//
// huge_pmd_unshare may unmap an entire PMD page.
// There is no way of knowing exactly which PMDs may
// be cached for this mm, so we must flush them all.
// start/end were already adjusted above to cover this
// range.
//
    flush_cache_range(vma, range.start, range.end);
//
// To call huge_pmd_unshare, i_mmap_rwsem must be
// held in write mode.  Caller needs to explicitly
// do this outside rmap routines.
//
// We also must hold hugetlb vma_lock in write mode.
// Lock order dictates acquiring vma_lock BEFORE
// i_mmap_rwsem.  We can only try lock here and
// fail if unsuccessful.
//
    if (!anon) {
pub static mut tlb: usize = 0;
    VM_BUG_ON(!(flags & TTU_RMAP_LOCKED));
    if (!hugetlb_vma_trylock_write(vma)) {
    page_vma_mapped_walk_done(&pvmw);
    ret = false;
    break;
    }
    tlb_gather_mmu_vma(&tlb, vma);
    if (huge_pmd_unshare(&tlb, vma, address, pvmw.pte)) {
    hugetlb_vma_unlock_write(vma);
    huge_pmd_unshare_flush(&tlb, vma);
    tlb_finish_mmu(&tlb);
//
// The PMD table was unmapped,
// consequently unmapping the folio.
//
    page_vma_mapped_walk_done(&pvmw);
    break;
    }
    hugetlb_vma_unlock_write(vma);
    tlb_finish_mmu(&tlb);
    }
// Nuke the hugetlb page table entry
    pteval = huge_ptep_clear_flush(vma, address, pvmw.pte);
    if (pte_dirty(pteval)) {
    folio_mark_dirty(folio);
    }
    writable = pte_write(pteval);
    } else if (likely(pte_present(pteval))) {
    flush_cache_page(vma, address, pfn);
// Nuke the page table entry.
    if (should_defer_flush(mm, flags)) {
//
// We clear the PTE but do not flush so potentially
// a remote CPU could still be writing to the folio.
// If the entry was previously clean then the
// architecture must guarantee that a clear->dirty
// transition on a cached TLB entry is written through
// and traps if the PTE is unmapped.
//
    pteval = ptep_get_and_clear(mm, address, pvmw.pte);
    set_tlb_ubc_flush_pending(mm, pteval, address, address + PAGE_SIZE);
    } else {
    pteval = ptep_clear_flush(vma, address, pvmw.pte);
    }
    if (pte_dirty(pteval)) {
    folio_mark_dirty(folio);
    }
    writable = pte_write(pteval);
    } else {
pub static mut entry: softleaf_t = 0;
    pte_clear(mm, address, pvmw.pte);
    writable = softleaf_is_device_private_write(entry);
    }
    VM_WARN_ON_FOLIO(writable && folio_test_anon(folio) &&
    !anon_exclusive, folio);
// Update high watermark before we lower rss
    update_hiwater_rss(mm);
    if (PageHWPoison(subpage)) {
    VM_WARN_ON_FOLIO(folio_is_device_private(folio), folio);
    pteval = swp_entry_to_pte(make_hwpoison_entry(subpage));
    if (folio_test_hugetlb(folio)) {
    hugetlb_count_sub(folio_nr_pages(folio), mm);
    set_huge_pte_at(mm, address, pvmw.pte, pteval,
    hsz);
    } else {
    dec_mm_counter(mm, mm_counter(folio));
    set_pte_at(mm, address, pvmw.pte, pteval);
    }
    } else if (likely(pte_present(pteval)) && pte_unused(pteval) &&
    !userfaultfd_armed(vma)) {
//
// The guest indicated that the page content is of no
// interest anymore. Simply discard the pte, vmscan
// will take care of the rest.
// A future reference will then fault in a new zero
// page. When userfaultfd is active, we must not drop
// this page though, as its main user (postcopy
// migration) will not expect userfaults on already
// copied pages.
//
    dec_mm_counter(mm, mm_counter(folio));
    } else {
    let mut entry;
    let mut swp_pte;
//
// arch_unmap_one() is expected to be a NOP on
// architectures where we could have PFN swap PTEs,
// so we'll not check/care.
//
    if (arch_unmap_one(mm, vma, address, pteval) < 0) {
    if (folio_test_hugetlb(folio)) {
    set_huge_pte_at(mm, address, pvmw.pte,
    pteval, hsz);
    }
    else {
    set_pte_at(mm, address, pvmw.pte, pteval);
    }
    ret = false;
    page_vma_mapped_walk_done(&pvmw);
    break;
    }
// See folio_try_share_anon_rmap_pte(): clear PTE first.
    if (folio_test_hugetlb(folio)) {
    if (anon_exclusive &&
    hugetlb_try_share_anon_rmap(folio)) {
    set_huge_pte_at(mm, address, pvmw.pte,
    pteval, hsz);
    ret = false;
    page_vma_mapped_walk_done(&pvmw);
    break;
    }
    } else if (anon_exclusive &&
    folio_try_share_anon_rmap_pte(folio, subpage)) {
    set_pte_at(mm, address, pvmw.pte, pteval);
    ret = false;
    page_vma_mapped_walk_done(&pvmw);
    break;
    }
//
// Store the pfn of the page in a special migration
// pte. do_swap_page() will wait until the migration
// pte is removed and then restart fault handling.
//
    if (writable) {
    entry = make_writable_migration_entry(
    page_to_pfn(subpage));
    }

    else if (anon_exclusive) {
    entry = make_readable_exclusive_migration_entry(
    page_to_pfn(subpage));
    }
    else {
    entry = make_readable_migration_entry(
    page_to_pfn(subpage));
    }
    if (likely(pte_present(pteval))) {
    if (pte_young(pteval)) {
    entry = make_migration_entry_young(entry);
    }
    if (pte_dirty(pteval)) {
    entry = make_migration_entry_dirty(entry);
    }
    swp_pte = swp_entry_to_pte(entry);
    if (pte_soft_dirty(pteval)) {
    swp_pte = pte_swp_mksoft_dirty(swp_pte);
    }
    if (pte_uffd(pteval)) {
    swp_pte = pte_swp_mkuffd(swp_pte);
    }
    } else {
    swp_pte = swp_entry_to_pte(entry);
    if (pte_swp_soft_dirty(pteval)) {
    swp_pte = pte_swp_mksoft_dirty(swp_pte);
    }
    if (pte_swp_uffd(pteval)) {
    swp_pte = pte_swp_mkuffd(swp_pte);
    }
    }
    if (folio_test_hugetlb(folio)) {
    set_huge_pte_at(mm, address, pvmw.pte, swp_pte,
    hsz);
    }
    else {
    set_pte_at(mm, address, pvmw.pte, swp_pte);
    }
    trace_set_migration_pte(address, pte_val(swp_pte),
    folio_order(folio));
//
// No need to invalidate here it will synchronize on
// against the special swap migration pte.
//
    }
    if (unlikely(folio_test_hugetlb(folio))) {
    hugetlb_remove_rmap(folio);
    }
    else {
    folio_remove_rmap_pte(folio, subpage, vma);
    }
    if (vma.vm_flags & VM_LOCKED) {
    mlock_drain_local();
    }
    folio_put(folio);
    }
    mmu_notifier_invalidate_range_end(&range);
    return ret;
    }
//
// try_to_migrate - try to replace all page table mappings with swap entries
// @folio: the folio to replace page table entries for
// @flags: action and flags
//
// Tries to remove all the page table entries which are mapping this folio and
// replace them with special swap entries. Caller must hold the folio lock.
//
#[no_mangle]
pub unsafe extern "C" fn try_to_migrate(folio: *mut folio, flags: ttu_flags) {
pub static mut rmap_walk_control: usize = 0;
//
// Migration always ignores mlock and only supports TTU_RMAP_LOCKED and
// TTU_SPLIT_HUGE_PMD, TTU_SYNC, and TTU_BATCH_FLUSH flags.
//
    if (WARN_ON_ONCE!(flags & ~(TTU_RMAP_LOCKED | TTU_SPLIT_HUGE_PMD |
    TTU_SYNC | TTU_BATCH_FLUSH))) {
    return;
    }
    if (folio_is_zone_device(folio) &&
    (!folio_is_device_private(folio) && !folio_is_device_coherent(folio))) {
    return;
    }
//
// During exec, a temporary VMA is setup and later moved.
// The VMA is moved under the anon_vma lock but not the
// page tables leading to a race where migration cannot
// find the migration ptes. Rather than increasing the
// locking requirements of exec(), migration skips
// temporary VMAs until after exec() completes.
//
    if (!folio_test_ksm(folio) && folio_test_anon(folio)) {
    rwc.invalid_vma = invalid_migration_vma;
    }
    if (flags & TTU_RMAP_LOCKED) {
    rmap_walk_locked(folio, &rwc);
    }
    else {
    rmap_walk(folio, &rwc);
    }
    }

//
// make_device_exclusive() - Mark a page for exclusive use by a device
// @mm: mm_struct of associated target process
// @addr: the virtual address to mark for exclusive device access
// @owner: passed to MMU_NOTIFY_EXCLUSIVE range notifier to allow filtering
// @foliop: folio pointer will be stored here on success.
//
// This function looks up the page mapped at the given address, grabs a
// folio reference, locks the folio and replaces the PTE with special
// device-exclusive PFN swap entry, preventing access through the process
// page tables. The function will return with the folio locked and referenced.
//
// On fault, the device-exclusive entries are replaced with the original PTE
// under folio lock, after calling MMU notifiers.
//
// Only anonymous non-hugetlb folios are supported and the VMA must have
// write permissions such that we can fault in the anonymous page writable
// in order to mark it exclusive. The caller must hold the mmap_lock in read
// mode.
//
// A driver using this to program access from a device must use a mmu notifier
// critical section to hold a device specific lock during programming. Once
// programming is complete it should drop the folio lock and reference after
// which point CPU access to the page will revoke the exclusive access.
//
// Notes:
// #. This function always operates on individual PTEs mapping individual
// pages. PMD-sized THPs are first remapped to be mapped by PTEs before
// the conversion happens on a single PTE corresponding to @addr.
// #. While concurrent access through the process page tables is prevented,
// concurrent access through other page references (e.g., earlier GUP
// invocation) is not handled and not supported.
// #. device-exclusive entries are considered "clean" and "old" by core-mm.
// Device drivers must update the folio state when informed by MMU
// notifiers.
//
// Returns: pointer to mapped page on success, otherwise a negative error.
//
#[no_mangle]
pub unsafe extern "C" fn make_device_exclusive(mm: *mut mm_struct, addr: c_ulong, owner: *mut c_void, foliop: *mut *mut folio) -> *mut c_void {
pub static mut range: usize = 0;
    let mut folio = core::ptr::null_mut();
    let mut fw_folio = core::ptr::null_mut();
pub static mut vma: *mut c_void = core::ptr::null_mut();
pub static mut fw: usize = 0;
pub static mut page: *mut c_void = core::ptr::null_mut();
    let mut entry;
    let mut swp_pte;
    let mut ret = 0;
    mmap_assert_locked(mm);
    addr = PAGE_ALIGN_DOWN(addr);
//
// Fault in the page writable and try to lock it; note that if the
// address would already be marked for exclusive use by a device,
// the GUP call would undo that first by triggering a fault.
//
// If any other device would already map this page exclusively, the
// fault will trigger a conversion to an ordinary
// (non-device-exclusive) PTE and issue a MMU_NOTIFY_EXCLUSIVE.
//
// label;
    page = get_user_page_vma_remote(mm, addr,
    FOLL_GET | FOLL_WRITE | FOLL_SPLIT_PMD,
    &vma);
    if (IS_ERR(page)) {
    return page;
    }
    folio = page_folio(page);
    if (!folio_test_anon(folio) || folio_test_hugetlb(folio)) {
    folio_put(folio);
    return ERR_PTR(-EOPNOTSUPP);
    }
    ret = folio_lock_killable(folio);
    if (ret) {
    folio_put(folio);
    return ERR_PTR(ret);
    }
//
// Inform secondary MMUs that we are going to convert this PTE to
// device-exclusive, such that they unmap it now. Note that the
// caller must filter this event out to prevent livelocks.
//
    mmu_notifier_range_init_owner(&range, MMU_NOTIFY_EXCLUSIVE, 0,
    mm, addr, addr + PAGE_SIZE, owner);
    mmu_notifier_invalidate_range_start(&range);
//
// Let's do a second walk and make sure we still find the same page
// mapped writable. Note that any page of an anonymous folio can
// only be mapped writable using exactly one PTE ("exclusive"), so
// there cannot be other mappings.
//
    fw_folio = folio_walk_start(&fw, vma, addr, 0);
    if (fw_folio != folio || fw.page != page ||
    fw.level != FW_LEVEL_PTE || !pte_write(fw.pte)) {
    if (fw_folio) {
    folio_walk_end(&fw, vma);
    }
    mmu_notifier_invalidate_range_end(&range);
    folio_unlock(folio);
    folio_put(folio);
// goto;
    }
// Nuke the page table entry so we get the uptodate dirty bit.
    flush_cache_page(vma, addr, page_to_pfn(page));
    fw.pte = ptep_clear_flush(vma, addr, fw.ptep);
// Set the dirty flag on the folio now the PTE is gone.
    if (pte_dirty(fw.pte)) {
    folio_mark_dirty(folio);
    }
//
// Store the pfn of the page in a special device-exclusive PFN swap PTE.
// do_swap_page() will trigger the conversion back while holding the
// folio lock.
//
    entry = make_device_exclusive_entry(page_to_pfn(page));
    swp_pte = swp_entry_to_pte(entry);
    if (pte_soft_dirty(fw.pte)) {
    swp_pte = pte_swp_mksoft_dirty(swp_pte);
    }
// The pte is writable, uffd-wp does not apply.
    set_pte_at(mm, addr, fw.ptep, swp_pte);
    folio_walk_end(&fw, vma);
    mmu_notifier_invalidate_range_end(&range);
// foliop = folio;
    return page;
    }
    EXPORT_SYMBOL_GPL(make_device_exclusive);

#[no_mangle]
pub unsafe extern "C" fn __put_anon_vma(anon_vma: *mut anon_vma) {
    let mut root = anon_vma.root;
    anon_vma_free(anon_vma);
    if (root != anon_vma && atomic_dec_and_test(&root.refcount)) {
    anon_vma_free(root);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn rmap_walk_anon_lock(folio: *mut folio, rwc: *mut rmap_walk_control) -> *mut c_void {
pub static mut anon_vma: *mut c_void = core::ptr::null_mut();
    if (rwc.anon_lock) {
    return rwc.anon_lock(folio, rwc);
    }
//
// Note: remove_migration_ptes() cannot use folio_lock_anon_vma_read()
// because that depends on folio_mapped(); but not all its usages
// are holding mmap_lock. Users without mmap_lock are required to
// take a reference count to prevent the anon_vma disappearing
//
    anon_vma = folio_anon_vma(folio);
    if (!anon_vma) {
    return core::ptr::null_mut();
    }
    if (anon_vma_trylock_read(anon_vma)) {
// goto;
    }
    if (rwc.try_lock) {
    anon_vma = core::ptr::null_mut();
    rwc.contended = true;
// goto;
    }
    anon_vma_lock_read(anon_vma);
// label;
    return anon_vma;
    }
//
// rmap_walk_anon - do something to anonymous page using the object-based
// rmap method
// @folio: the folio to be handled
// @rwc: control variable according to each walk type
// @locked: caller holds relevant rmap lock
//
// Find all the mappings of a folio using the mapping pointer and the vma
// chains contained in the anon_vma struct it points to.
//
#[no_mangle]
pub unsafe extern "C" fn rmap_walk_anon(folio: *mut folio, rwc: *mut rmap_walk_control, locked: bool) {
pub static mut anon_vma: *mut c_void = core::ptr::null_mut();
    pgoff_t pgoff_start, pgoff_end;
pub static mut avc: *mut c_void = core::ptr::null_mut();
//
// The folio lock ensures that folio->mapping can't be changed under us
// to an anon_vma with different root.
//
    VM_WARN_ON_FOLIO(!folio_test_locked(folio), folio);
    if (locked) {
    anon_vma = folio_anon_vma(folio);
// anon_vma disappear under us?
    VM_BUG_ON_FOLIO(!anon_vma, folio);
    } else {
    anon_vma = rmap_walk_anon_lock(folio, rwc);
    }
    if (!anon_vma) {
    return;
    }
    pgoff_start = folio_pgoff(folio);
    pgoff_end = pgoff_start + folio_nr_pages(folio) - 1;
    anon_rmap_tree_foreach(avc, anon_vma, pgoff_start, pgoff_end) {
    let mut vma = avc.vma;
    let mut address = vma_anon_address(vma, pgoff_start,
    folio_nr_pages(folio));
    VM_WARN_ON_ONCE_VMA(address == -EFAULT, vma);
    cond_resched();
    if (rwc.invalid_vma && rwc.invalid_vma(vma, rwc.arg)) {
    continue;
    }
    if (!rwc.rmap_one(folio, vma, address, rwc.arg)) {
    break;
    }
    if (rwc.done && rwc.done(folio)) {
    break;
    }
    }
    if (!locked) {
    anon_vma_unlock_read(anon_vma);
    }
    }
//
// __rmap_walk_file() - Traverse the reverse mapping for a file-backed mapping
// of a page mapped within a specified page cache object at a specified offset.
//
// @folio: 		Either the folio whose mappings to traverse, or if NULL,
// the callbacks specified in @rwc will be configured such
// as to be able to look up mappings correctly.
// @mapping: 		The page cache object whose mapping VMAs we intend to
// traverse. If @folio is non-NULL, this should be equal to
// folio_mapping(folio).
// @pgoff_start:	The offset within @mapping of the page which we are
// looking up. If @folio is non-NULL, this should be equal
// to folio_pgoff(folio).
// @nr_pages:		The number of pages mapped by the mapping. If @folio is
// non-NULL, this should be equal to folio_nr_pages(folio).
// @rwc:		The reverse mapping walk control object describing how
// the traversal should proceed.
// @locked:		Is the @mapping already locked? If not, we acquire the
// lock.
//
#[no_mangle]
pub unsafe extern "C" fn __rmap_walk_file(folio: *mut folio, mapping: *mut address_space, pgoff_start: pgoff_t, nr_pages: c_ulong, rwc: *mut rmap_walk_control, locked: bool) {
pub static mut pgoff_end: pgoff_t = 0;
pub static mut vma: *mut c_void = core::ptr::null_mut();
    VM_WARN_ON_FOLIO(folio && mapping != folio_mapping(folio), folio);
    VM_WARN_ON_FOLIO(folio && pgoff_start != folio_pgoff(folio), folio);
    VM_WARN_ON_FOLIO(folio && nr_pages != folio_nr_pages(folio), folio);
    if (!locked) {
    if (i_mmap_trylock_read(mapping)) {
// goto;
    }
    if (rwc.try_lock) {
    rwc.contended = true;
    return;
    }
    i_mmap_lock_read(mapping);
    }
// label;
    mapping_rmap_tree_foreach(vma, mapping, pgoff_start, pgoff_end) {
    let mut address = vma_filebacked_address(vma, pgoff_start,
    nr_pages);
    VM_BUG_ON_VMA(address == -EFAULT, vma);
    cond_resched();
    if (rwc.invalid_vma && rwc.invalid_vma(vma, rwc.arg)) {
    continue;
    }
    if (!rwc.rmap_one(folio, vma, address, rwc.arg)) {
// goto;
    }
    if (rwc.done && rwc.done(folio)) {
// goto;
    }
    }
// label;
    if (!locked) {
    i_mmap_unlock_read(mapping);
    }
    }
//
// rmap_walk_file - do something to file page using the object-based rmap method
// @folio: the folio to be handled
// @rwc: control variable according to each walk type
// @locked: caller holds relevant rmap lock
//
// Find all the mappings of a folio using the mapping pointer and the vma chains
// contained in the address_space struct it points to.
//
#[no_mangle]
pub unsafe extern "C" fn rmap_walk_file(folio: *mut folio, rwc: *mut rmap_walk_control, locked: bool) {
//
// The folio lock not only makes sure that folio->mapping cannot
// suddenly be NULLified by truncation, it makes sure that the structure
// at mapping cannot be freed and reused yet, so we can safely take
// mapping->i_mmap_rwsem.
//
    VM_BUG_ON_FOLIO(!folio_test_locked(folio), folio);
    if (!folio.mapping) {
    return;
    }
    __rmap_walk_file(folio, folio.mapping, folio.index,
    folio_nr_pages(folio), rwc, locked);
    }
#[no_mangle]
pub unsafe extern "C" fn rmap_walk(folio: *mut folio, rwc: *mut rmap_walk_control) {
    if (unlikely(folio_test_ksm(folio))) {
    rmap_walk_ksm(folio, rwc);
    }

    else if (folio_test_anon(folio)) {
    rmap_walk_anon(folio, rwc, false);
    }
    else {
    rmap_walk_file(folio, rwc, false);
    }
    }
// Like rmap_walk, but caller holds relevant rmap lock
#[no_mangle]
pub unsafe extern "C" fn rmap_walk_locked(folio: *mut folio, rwc: *mut rmap_walk_control) {
// no ksm support for now
    VM_BUG_ON_FOLIO(folio_test_ksm(folio), folio);
    if (folio_test_anon(folio)) {
    rmap_walk_anon(folio, rwc, true);
    }
    else {
    rmap_walk_file(folio, rwc, true);
    }
    }

//
// The following two functions are for anonymous (private mapped) hugepages.
// Unlike common anonymous pages, anonymous hugepages have no accounting code
// and no lru code, because we handle hugepages differently from common pages.
//
#[no_mangle]
pub unsafe extern "C" fn hugetlb_add_anon_rmap(folio: *mut folio, vma: *mut vm_area_struct, address: c_ulong, flags: rmap_t) {
    VM_WARN_ON_FOLIO(!folio_test_hugetlb(folio), folio);
    VM_WARN_ON_FOLIO(!folio_test_anon(folio), folio);
    atomic_inc(&folio._entire_mapcount);
    atomic_inc(&folio._large_mapcount);
    if (flags & RMAP_EXCLUSIVE) {
    SetPageAnonExclusive(&folio.page);
    }
    VM_WARN_ON_FOLIO(folio_entire_mapcount(folio) > 1 &&
    PageAnonExclusive(&folio.page), folio);
    }
#[no_mangle]
pub unsafe extern "C" fn hugetlb_add_new_anon_rmap(folio: *mut folio, vma: *mut vm_area_struct, address: c_ulong) {
    VM_WARN_ON_FOLIO(!folio_test_hugetlb(folio), folio);
    BUG_ON!(address < vma.vm_start || address >= vma.vm_end);
// increment count (starts at -1)
    atomic_set(&folio._entire_mapcount, 0);
    atomic_set(&folio._large_mapcount, 0);
    folio_clear_hugetlb_restore_reserve(folio);
    __folio_set_anon(folio, vma, address, true);
    SetPageAnonExclusive(&folio.page);
    }