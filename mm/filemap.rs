//! Automatically rewritten from C to Rust
//! Source: mm/filemap.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// linux/mm/filemap.c
//
// Copyright (C) 1994-1999  Linus Torvalds
//
// This file handles the generic file mmap semantics used by
// most "normal" filesystems (but you don't /have/ to use this:
// the NFS filesystem used to do this differently, for example)
//

// Macro flag: #define CREATE_TRACE_POINTS

//
// FIXME: remove all knowledge of the buffer layer from the core VM
//

//
// Shared mappings implemented 30.11.1994. It's not fully working yet,
// though.
//
// Shared mappings now work. 15.8.1995  Bruno.
//
// finished 'unifying' the page and buffer cache and SMP-threaded the
// page-cache, 21.05.1999, Ingo Molnar <mingo@redhat.com>
//
// SMP-threaded pagemap-LRU 1999, Andrea Arcangeli <andrea@suse.de>
//
// Lock ordering:
//
// ->i_mmap_rwsem		(truncate_pagecache)
// ->private_lock		(__free_pte->block_dirty_folio)
// ->swap_lock		(exclusive_swap_page, others)
// ->i_pages lock
//
// ->i_rwsem
// ->invalidate_lock		(acquired by fs in truncate path)
// ->i_mmap_rwsem		(truncate->unmap_mapping_range)
//
// ->mmap_lock
// ->i_mmap_rwsem
// ->page_table_lock or pte_lock	(various, mainly in memory.c)
// ->i_pages lock	(arch-dependent flush_dcache_mmap_lock)
//
// ->mmap_lock
// ->invalidate_lock		(filemap_fault)
// ->lock_page		(filemap_fault, access_process_vm)
//
// ->i_rwsem			(generic_perform_write)
// ->mmap_lock		(fault_in_readable->do_page_fault)
//
// bdi->wb.list_lock
// sb_lock			(fs/fs-writeback.c)
// ->i_pages lock		(__sync_single_inode)
//
// ->i_mmap_rwsem
// ->anon_vma.lock		(vma_merge)
//
// ->anon_vma.lock
// ->page_table_lock or pte_lock	(anon_vma_prepare and various)
//
// ->page_table_lock or pte_lock
// ->swap_lock		(try_to_unmap_one)
// ->private_lock		(try_to_unmap_one)
// ->i_pages lock		(try_to_unmap_one)
// ->lruvec->lru_lock	(follow_page_mask->mark_page_accessed)
// ->lruvec->lru_lock	(check_pte_range->folio_isolate_lru)
// ->private_lock		(folio_remove_rmap_pte->set_page_dirty)
// ->i_pages lock		(folio_remove_rmap_pte->set_page_dirty)
// bdi.wb->list_lock		(folio_remove_rmap_pte->set_page_dirty)
// ->inode->i_lock		(folio_remove_rmap_pte->set_page_dirty)
// bdi.wb->list_lock		(zap_pte_range->set_page_dirty)
// ->inode->i_lock		(zap_pte_range->set_page_dirty)
// ->private_lock		(zap_pte_range->block_dirty_folio)
//
#[no_mangle]
pub unsafe extern "C" fn page_cache_delete(mapping: *mut address_space, folio: *mut folio, shadow: *mut c_void) {
    XA_STATE(xas, &mapping.i_pages, folio.index);
pub static mut nr: c_long = 1;
    mapping_set_update(&xas, mapping);
    xas_set_order(&xas, folio.index, folio_order(folio));
    nr = folio_nr_pages(folio);
    VM_BUG_ON_FOLIO(!folio_test_locked(folio), folio);
    xas_store(&xas, shadow);
    xas_init_marks(&xas);
    folio.mapping = core::ptr::null_mut();
// Leave folio->index set: truncation lookup relies upon it
    mapping.nrpages -= nr;
    }
#[no_mangle]
pub unsafe extern "C" fn filemap_unaccount_folio(mapping: *mut address_space, folio: *mut folio) {
    let mut nr = 0;
    VM_BUG_ON_FOLIO(folio_mapped(folio), folio);
    if (!IS_ENABLED!(CONFIG_DEBUG_VM) && unlikely(folio_mapped(folio))) {
    pr_alert("BUG: Bad page cache in process %s  pfn:%05lx\n",
    current.comm, folio_pfn(folio));
    dump_page(&folio.page, "still mapped when deleted");
    dump_stack();
    add_taint(TAINT_BAD_PAGE, LOCKDEP_NOW_UNRELIABLE);
    if (mapping_exiting(mapping) && !folio_test_large(folio)) {
pub static mut mapcount: c_int = 0;
    if (folio_ref_count(folio) >= mapcount + 2) {
//
// All vmas have already been torn down, so it's
// a good bet that actually the page is unmapped
// and we'd rather not leak it: if we're wrong,
// another bad page check should catch it later.
//
    atomic_set(&folio._mapcount, -1);
    folio_ref_sub(folio, mapcount);
    }
    }
    }
// hugetlb folios do not participate in page cache accounting.
    if (folio_test_hugetlb(folio)) {
    return;
    }
    nr = folio_nr_pages(folio);
    lruvec_stat_mod_folio(folio, NR_FILE_PAGES, -nr);
    if (folio_test_swapbacked(folio)) {
    lruvec_stat_mod_folio(folio, NR_SHMEM, -nr);
    if (folio_test_pmd_mappable(folio)) {
    lruvec_stat_mod_folio(folio, NR_SHMEM_THPS, -nr);
    }
    } else if (folio_test_pmd_mappable(folio)) {
    lruvec_stat_mod_folio(folio, NR_FILE_THPS, -nr);
    }
    if (test_bit(AS_KERNEL_FILE, &folio.mapping.flags)) {
    mod_node_page_state(folio_pgdat(folio),
    NR_KERNEL_FILE_PAGES, -nr);
    }
//
// At this point folio must be either written or cleaned by
// truncate.  Dirty folio here signals a bug and loss of
// unwritten data - on ordinary filesystems.
//
// But it's harmless on in-memory filesystems like tmpfs; and can
// occur when a driver which did get_user_pages() sets page dirty
// before putting it, while the inode is being finally evicted.
//
// Below fixes dirty accounting after removing the folio entirely
// but leaves the dirty flag set: it has no effect for truncated
// folio and anyway will be cleared before returning folio to
// buddy allocator.
//
    if (WARN_ON_ONCE!(folio_test_dirty(folio) &&
    mapping_can_writeback(mapping))) {
    folio_account_cleaned(folio, inode_to_wb(mapping.host));
    }
    }
//
// Delete a page from the page cache and free it. Caller has to make
// sure the page is locked and that nobody else uses it - or that usage
// is safe.  The caller must hold the i_pages lock.
//
#[no_mangle]
pub unsafe extern "C" fn __filemap_remove_folio(folio: *mut folio, shadow: *mut c_void) {
    let mut mapping = folio.mapping;
    trace_mm_filemap_delete_from_page_cache(folio);
    filemap_unaccount_folio(mapping, folio);
    page_cache_delete(mapping, folio, shadow);
    }
#[no_mangle]
pub unsafe extern "C" fn filemap_free_folio(mapping: *mut address_space, folio: *mut folio) {
    void (*free_folio);
    free_folio = mapping.a_ops.free_folio;
    if (free_folio) {
    free_folio(folio);
    }
    folio_put_refs(folio, folio_nr_pages(folio));
    }
//
// filemap_remove_folio - Remove folio from page cache.
// @folio: The folio.
//
// This must be called only on folios that are locked and have been
// verified to be in the page cache.  It will never put the folio into
// the free list because the caller has a reference on the page.
//
#[no_mangle]
pub unsafe extern "C" fn filemap_remove_folio(folio: *mut folio) {
    let mut mapping = folio.mapping;
    BUG_ON!(!folio_test_locked(folio));
    spin_lock(&mapping.host.i_lock);
    xa_lock_irq(&mapping.i_pages);
    __filemap_remove_folio(folio, core::ptr::null_mut());
    xa_unlock_irq(&mapping.i_pages);
    if (mapping_shrinkable(mapping)) {
    inode_lru_list_add(mapping.host);
    }
    spin_unlock(&mapping.host.i_lock);
    filemap_free_folio(mapping, folio);
    }
//
// page_cache_delete_batch - delete several folios from page cache
// @mapping: the mapping to which folios belong
// @fbatch: batch of folios to delete
//
// The function walks over mapping->i_pages and removes folios passed in
// @fbatch from the mapping. The function expects @fbatch to be sorted
// by page index and is optimised for it to be dense.
// It tolerates holes in @fbatch (mapping entries at those indices are not
// modified).
//
// The function expects the i_pages lock to be held.
//
#[no_mangle]
pub unsafe extern "C" fn page_cache_delete_batch(mapping: *mut address_space, fbatch: *mut folio_batch) {
    XA_STATE(xas, &mapping.i_pages, fbatch.folios[0].index);
pub static mut total_pages: c_long = 0;
pub static mut i: c_int = 0;
pub static mut folio: *mut c_void = core::ptr::null_mut();
    mapping_set_update(&xas, mapping);
    xas_for_each(&xas, folio, ULONG_MAX) {
    if (i >= folio_batch_count(fbatch)) {
    break;
    }
// A swap/dax/shadow entry got inserted? Skip it.
    if (xa_is_value(folio)) {
    continue;
    }
//
// A page got inserted in our range? Skip it. We have our
// pages locked so they are protected from being removed.
// If we see a page whose index is higher than ours, it
// means our page has been removed, which shouldn't be
// possible because we're holding the PageLock.
//
    if (folio != fbatch.folios[i]) {
    VM_BUG_ON_FOLIO(folio.index >
    fbatch.folios[i].index, folio);
    continue;
    }
    WARN_ON_ONCE!(!folio_test_locked(folio));
    folio.mapping = core::ptr::null_mut();
// Leave folio->index set: truncation lookup relies on it
    i += 1;
    xas_store(&xas, core::ptr::null_mut());
    total_pages += folio_nr_pages(folio);
    }
    mapping.nrpages -= total_pages;
    }
#[no_mangle]
pub unsafe extern "C" fn delete_from_page_cache_batch(mapping: *mut address_space, fbatch: *mut folio_batch) {
    let mut i = 0;
    if (!folio_batch_count(fbatch)) {
    return;
    }
    spin_lock(&mapping.host.i_lock);
    xa_lock_irq(&mapping.i_pages);
    while (i < folio_batch_count(fbatch)) {
    let mut folio = fbatch.folios[i];
    trace_mm_filemap_delete_from_page_cache(folio);
    filemap_unaccount_folio(mapping, folio);
    }
    page_cache_delete_batch(mapping, fbatch);
    xa_unlock_irq(&mapping.i_pages);
    if (mapping_shrinkable(mapping)) {
    inode_lru_list_add(mapping.host);
    }
    spin_unlock(&mapping.host.i_lock);
    for (i = 0; i < folio_batch_count(fbatch); i++) {
    filemap_free_folio(mapping, fbatch.folios[i]);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn filemap_check_errors(mapping: *mut address_space) -> c_int {
pub static mut ret: c_int = 0;
// Check for outstanding write errors
    if (test_bit(AS_ENOSPC, &mapping.flags) &&
    test_and_clear_bit(AS_ENOSPC, &mapping.flags)) {
    ret = -ENOSPC;
    }
    if (test_bit(AS_EIO, &mapping.flags) &&
    test_and_clear_bit(AS_EIO, &mapping.flags)) {
    ret = -EIO;
    }
    return ret;
    }
    EXPORT_SYMBOL(filemap_check_errors);
#[no_mangle]
unsafe extern "C" fn filemap_check_and_keep_errors(mapping: *mut address_space) -> c_int {
// Check for outstanding write errors
    if (test_bit(AS_EIO, &mapping.flags)) {
    return -EIO;
    }
    if (test_bit(AS_ENOSPC, &mapping.flags)) {
    return -ENOSPC;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn filemap_writeback(mapping: *mut address_space, start: loff_t, end: loff_t, sync_mode: writeback_sync_modes, nr_to_write: *mut c_long) -> c_int {
pub static mut writeback_control: usize = 0;
    let mut ret = 0;
    if (!mapping_can_writeback(mapping) ||
    !mapping_tagged(mapping, PAGECACHE_TAG_DIRTY)) {
    return 0;
    }
    wbc_attach_fdatawrite_inode(&wbc, mapping.host);
    ret = do_writepages(mapping, &wbc);
    wbc_detach_inode(&wbc);
    if (!ret && nr_to_write) {
// nr_to_write = wbc.nr_to_write;
    }
    return ret;
    }
//
// filemap_fdatawrite_range - start writeback on mapping dirty pages in range
// @mapping:	address space structure to write
// @start:	offset in bytes where the range starts
// @end:	offset in bytes where the range ends (inclusive)
//
// Start writeback against all of a mapping's dirty pages that lie
// within the byte offsets <start, end> inclusive.
//
// This is a data integrity operation that waits upon dirty or in writeback
// pages.
//
// Return: %0 on success, negative error code otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn filemap_fdatawrite_range(mapping: *mut address_space, start: loff_t, end: loff_t) -> c_int {
    return filemap_writeback(mapping, start, end, WB_SYNC_ALL, core::ptr::null_mut());
    }
    EXPORT_SYMBOL(filemap_fdatawrite_range);
#[no_mangle]
pub unsafe extern "C" fn filemap_fdatawrite(mapping: *mut address_space) -> c_int {
    return filemap_fdatawrite_range(mapping, 0, LLONG_MAX);
    }
    EXPORT_SYMBOL(filemap_fdatawrite);
//
// filemap_flush_range - start writeback on a range
// @mapping:	target address_space
// @start:	index to start writeback on
// @end:	last (inclusive) index for writeback
//
// This is a non-integrity writeback helper, to start writing back folios
// for the indicated range.
//
// Return: %0 on success, negative error code otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn filemap_flush_range(mapping: *mut address_space, start: loff_t, end: loff_t) -> c_int {
    return filemap_writeback(mapping, start, end, WB_SYNC_NONE, core::ptr::null_mut());
    }
    EXPORT_SYMBOL_GPL(filemap_flush_range);
//
// filemap_flush - mostly a non-blocking flush
// @mapping:	target address_space
//
// This is a mostly non-blocking flush.  Not suitable for data-integrity
// purposes - I/O may not be started against all dirty pages.
//
// Return: %0 on success, negative error code otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn filemap_flush(mapping: *mut address_space) -> c_int {
    return filemap_flush_range(mapping, 0, LLONG_MAX);
    }
    EXPORT_SYMBOL(filemap_flush);
//
// Start writeback on @nr_to_write pages from @mapping.  No one but the existing
// btrfs caller should be using this.  Talk to linux-mm if you think adding a
// new caller is a good idea.
//
#[no_mangle]
pub unsafe extern "C" fn filemap_flush_nr(mapping: *mut address_space, nr_to_write: *mut c_long) -> c_int {
    return filemap_writeback(mapping, 0, LLONG_MAX, WB_SYNC_NONE,
    nr_to_write);
    }
    EXPORT_SYMBOL_FOR_MODULES(filemap_flush_nr, "btrfs");
//
// filemap_range_has_page - check if a page exists in range.
// @mapping:           address space within which to check
// @start_byte:        offset in bytes where the range starts
// @end_byte:          offset in bytes where the range ends (inclusive)
//
// Find at least one page in the range supplied, usually used to check if
// direct writing in this range will trigger a writeback.
//
// Return: %true if at least one page exists in the specified range,
// %false otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn filemap_range_has_page(mapping: *mut address_space, start_byte: loff_t, end_byte: loff_t) -> bool {
pub static mut folio: *mut c_void = core::ptr::null_mut();
    XA_STATE(xas, &mapping.i_pages, start_byte >> PAGE_SHIFT);
pub static mut max: pgoff_t = 0;
    if (end_byte < start_byte) {
    return false;
    }
    rcu_read_lock();
    for (;;) {
    folio = xas_find(&xas, max);
    if (xas_retry(&xas, folio)) {
    continue;
    }
// Shadow entries don't count
    if (xa_is_value(folio)) {
    continue;
    }
//
// We don't need to try to pin this page; we're about to
// release the RCU lock anyway.  It is enough to know that
// there was a page here recently.
//
    break;
    }
    rcu_read_unlock();
    return folio != core::ptr::null_mut();
    }
    EXPORT_SYMBOL(filemap_range_has_page);
#[no_mangle]
pub unsafe extern "C" fn __filemap_fdatawait_range(mapping: *mut address_space, start_byte: loff_t, end_byte: loff_t) {
pub static mut index: pgoff_t = 0;
pub static mut end: pgoff_t = 0;
pub static mut fbatch: usize = 0;
    let mut nr_folios: c_uint = 0;
    folio_batch_init(&fbatch);
    while (index <= end) {
    let mut i: c_uint = 0;
    nr_folios = filemap_get_folios_tag(mapping, &index, end,
    PAGECACHE_TAG_WRITEBACK, &fbatch);
    if (!nr_folios) {
    break;
    }
    while (i < nr_folios) {
    let mut folio = fbatch.folios[i];
    folio_wait_writeback(folio);
    }
    folio_batch_release(&fbatch);
    cond_resched();
    }
    }
//
// filemap_fdatawait_range - wait for writeback to complete
// @mapping:		address space structure to wait for
// @start_byte:		offset in bytes where the range starts
// @end_byte:		offset in bytes where the range ends (inclusive)
//
// Walk the list of under-writeback pages of the given address space
// in the given range and wait for all of them.  Check error status of
// the address space and return it.
//
// Since the error status of the address space is cleared by this function,
// callers are responsible for checking the return value and handling and/or
// reporting the error.
//
// Return: error status of the address space.
//
#[no_mangle]
pub unsafe extern "C" fn filemap_fdatawait_range(mapping: *mut address_space, start_byte: loff_t, end_byte: loff_t) -> c_int {
    __filemap_fdatawait_range(mapping, start_byte, end_byte);
    return filemap_check_errors(mapping);
    }
    EXPORT_SYMBOL(filemap_fdatawait_range);
//
// filemap_fdatawait_range_keep_errors - wait for writeback to complete
// @mapping:		address space structure to wait for
// @start_byte:		offset in bytes where the range starts
// @end_byte:		offset in bytes where the range ends (inclusive)
//
// Walk the list of under-writeback pages of the given address space in the
// given range and wait for all of them.  Unlike filemap_fdatawait_range(),
// this function does not clear error status of the address space.
//
// Use this function if callers don't handle errors themselves.  Expected
// call sites are system-wide / filesystem-wide data flushers: e.g. sync(2),
fsfreeze(8)
//
#[no_mangle]
pub unsafe extern "C" fn filemap_fdatawait_range_keep_errors(mapping: *mut address_space, start_byte: loff_t, end_byte: loff_t) -> c_int {
    __filemap_fdatawait_range(mapping, start_byte, end_byte);
    return filemap_check_and_keep_errors(mapping);
    }
    EXPORT_SYMBOL(filemap_fdatawait_range_keep_errors);
//
// file_fdatawait_range - wait for writeback to complete
// @file:		file pointing to address space structure to wait for
// @start_byte:		offset in bytes where the range starts
// @end_byte:		offset in bytes where the range ends (inclusive)
//
// Walk the list of under-writeback pages of the address space that file
// refers to, in the given range and wait for all of them.  Check error
// status of the address space vs. the file->f_wb_err cursor and return it.
//
// Since the error status of the file is advanced by this function,
// callers are responsible for checking the return value and handling and/or
// reporting the error.
//
// Return: error status of the address space vs. the file->f_wb_err cursor.
//
#[no_mangle]
pub unsafe extern "C" fn file_fdatawait_range(file: *mut file, start_byte: loff_t, end_byte: loff_t) -> c_int {
    let mut mapping = file.f_mapping;
    __filemap_fdatawait_range(mapping, start_byte, end_byte);
    return file_check_and_advance_wb_err(file);
    }
    EXPORT_SYMBOL(file_fdatawait_range);
//
// filemap_fdatawait_keep_errors - wait for writeback without clearing errors
// @mapping: address space structure to wait for
//
// Walk the list of under-writeback pages of the given address space
// and wait for all of them.  Unlike filemap_fdatawait(), this function
// does not clear error status of the address space.
//
// Use this function if callers don't handle errors themselves.  Expected
// call sites are system-wide / filesystem-wide data flushers: e.g. sync(2),
fsfreeze(8)
//
// Return: error status of the address space.
//
#[no_mangle]
pub unsafe extern "C" fn filemap_fdatawait_keep_errors(mapping: *mut address_space) -> c_int {
    __filemap_fdatawait_range(mapping, 0, LLONG_MAX);
    return filemap_check_and_keep_errors(mapping);
    }
    EXPORT_SYMBOL(filemap_fdatawait_keep_errors);
// Returns true if writeback might be needed or already in progress.
#[no_mangle]
unsafe extern "C" fn mapping_needs_writeback(mapping: *mut address_space) -> bool {
    return mapping.nrpages;
    }
#[no_mangle]
pub unsafe extern "C" fn filemap_range_has_writeback(mapping: *mut address_space, start_byte: loff_t, end_byte: loff_t) -> bool {
    XA_STATE(xas, &mapping.i_pages, start_byte >> PAGE_SHIFT);
pub static mut max: pgoff_t = 0;
pub static mut folio: *mut c_void = core::ptr::null_mut();
    if (end_byte < start_byte) {
    return false;
    }
    rcu_read_lock();
    xas_for_each(&xas, folio, max) {
    if (xas_retry(&xas, folio)) {
    continue;
    }
    if (xa_is_value(folio)) {
    continue;
    }
    if (folio_test_dirty(folio) || folio_test_locked(folio) ||
    folio_test_writeback(folio)) {
    break;
    }
    }
    rcu_read_unlock();
    return folio != core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(filemap_range_has_writeback);
//
// filemap_write_and_wait_range - write out & wait on a file range
// @mapping:	the address_space for the pages
// @lstart:	offset in bytes where the range starts
// @lend:	offset in bytes where the range ends (inclusive)
//
// Write out and wait upon file offsets lstart->lend, inclusive.
//
// Note that @lend is inclusive (describes the last byte to be written) so
// that this function can be used to write to the very end-of-file (end = -1).
//
// Return: error status of the address space.
//
#[no_mangle]
pub unsafe extern "C" fn filemap_write_and_wait_range(mapping: *mut address_space, lstart: loff_t, lend: loff_t) -> c_int {
pub static mut err: c_int = 0;
    if (lend < lstart) {
    return 0;
    }
    if (mapping_needs_writeback(mapping)) {
    err = filemap_fdatawrite_range(mapping, lstart, lend);
//
// Even if the above returned error, the pages may be
// written partially (e.g. -ENOSPC), so we wait for it.
// But the -EIO is special case, it may indicate the worst
// thing (e.g. bug) happened, so we avoid waiting for it.
//
    if (err != -EIO) {
    __filemap_fdatawait_range(mapping, lstart, lend);
    }
    }
    err2 = filemap_check_errors(mapping);
    if (!err) {
    err = err2;
    }
    return err;
    }
    EXPORT_SYMBOL(filemap_write_and_wait_range);
#[no_mangle]
pub unsafe extern "C" fn __filemap_set_wb_err(mapping: *mut address_space, err: c_int) {
pub static mut eseq: errseq_t = 0;
    trace_filemap_set_wb_err(mapping, eseq);
    }
    EXPORT_SYMBOL(__filemap_set_wb_err);
//
// file_check_and_advance_wb_err - report wb error (if any) that was previously
// and advance wb_err to current one
// @file: file on which the error is being reported
//
// When userland calls fsync (or something like nfsd does the equivalent), we
// want to report any writeback errors that occurred since the last fsync (or
// since the file was opened if there haven't been any).
//
// Grab the wb_err from the mapping. If it matches what we have in the file,
// then just quickly return 0. The file is all caught up.
//
// If it doesn't match, then take the mapping value, set the "seen" flag in
// it and try to swap it into place. If it works, or another task beat us
// to it with the new value, then update the f_wb_err and return the error
// portion. The error at this point must be reported via proper channels
// (a'la fsync, or NFS COMMIT operation, etc.).
//
// While we handle mapping->wb_err with atomic operations, the f_wb_err
// value is protected by the f_lock since we must ensure that it reflects
// the latest value swapped in for this file descriptor.
//
// Return: %0 on success, negative error code otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn file_check_and_advance_wb_err(file: *mut file) -> c_int {
pub static mut err: c_int = 0;
pub static mut old: errseq_t = 0;
    let mut mapping = file.f_mapping;
// Locklessly handle the common case where nothing has changed
    if (errseq_check(&mapping.wb_err, old)) {
// Something changed, must use slow path
    spin_lock(&file.f_lock);
    old = file.f_wb_err;
    err = errseq_check_and_advance(&mapping.wb_err,
    &file.f_wb_err);
    trace_file_check_and_advance_wb_err(file, old);
    spin_unlock(&file.f_lock);
    }
//
// We're mostly using this function as a drop in replacement for
// filemap_check_errors. Clear AS_EIO/AS_ENOSPC to emulate the effect
// that the legacy code would have had on these flags.
//
    clear_bit(AS_EIO, &mapping.flags);
    clear_bit(AS_ENOSPC, &mapping.flags);
    return err;
    }
    EXPORT_SYMBOL(file_check_and_advance_wb_err);
//
// file_write_and_wait_range - write out & wait on a file range
// @file:	file pointing to address_space with pages
// @lstart:	offset in bytes where the range starts
// @lend:	offset in bytes where the range ends (inclusive)
//
// Write out and wait upon file offsets lstart->lend, inclusive.
//
// Note that @lend is inclusive (describes the last byte to be written) so
// that this function can be used to write to the very end-of-file (end = -1).
//
// After writing out and waiting on the data, we check and advance the
// f_wb_err cursor to the latest value, and return any errors detected there.
//
// Return: %0 on success, negative error code otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn file_write_and_wait_range(file: *mut file, lstart: loff_t, lend: loff_t) -> c_int {
pub static mut err: c_int = 0;
    let mut mapping = file.f_mapping;
    if (lend < lstart) {
    return 0;
    }
    if (mapping_needs_writeback(mapping)) {
    err = filemap_fdatawrite_range(mapping, lstart, lend);
// See comment of filemap_write_and_wait()
    if (err != -EIO) {
    __filemap_fdatawait_range(mapping, lstart, lend);
    }
    }
    err2 = file_check_and_advance_wb_err(file);
    if (!err) {
    err = err2;
    }
    return err;
    }
    EXPORT_SYMBOL(file_write_and_wait_range);
//
// replace_page_cache_folio - replace a pagecache folio with a new one
// @old:	folio to be replaced
// @new:	folio to replace with
//
// This function replaces a folio in the pagecache with a new one.  On
// success it acquires the pagecache reference for the new folio and
// drops it for the old folio.  Both the old and new folios must be
// locked.  This function does not add the new folio to the LRU, the
// caller must do that.
//
// The remove + add is atomic.  This function cannot fail.
//
#[no_mangle]
pub unsafe extern "C" fn replace_page_cache_folio(old: *mut folio, new: *mut folio) {
    let mut mapping = old.mapping;
    void (*free_folio) = mapping.a_ops.free_folio;
pub static mut offset: pgoff_t = 0;
    XA_STATE(xas, &mapping.i_pages, offset);
    VM_BUG_ON_FOLIO(!folio_test_locked(old), old);
    VM_BUG_ON_FOLIO(!folio_test_locked(new), new);
    VM_BUG_ON_FOLIO(new.mapping, new);
    folio_get(new);
    new.mapping = mapping;
    new.index = offset;
    mem_cgroup_replace_folio(old, new);
    xas_lock_irq(&xas);
    xas_store(&xas, new);
    old.mapping = core::ptr::null_mut();
// hugetlb pages do not participate in page cache accounting.
    if (!folio_test_hugetlb(old)) {
    lruvec_stat_sub_folio(old, NR_FILE_PAGES);
    }
    if (!folio_test_hugetlb(new)) {
    lruvec_stat_add_folio(new, NR_FILE_PAGES);
    }
    if (folio_test_swapbacked(old)) {
    lruvec_stat_sub_folio(old, NR_SHMEM);
    }
    if (folio_test_swapbacked(new)) {
    lruvec_stat_add_folio(new, NR_SHMEM);
    }
    xas_unlock_irq(&xas);
    if (free_folio) {
    free_folio(old);
    }
    folio_put(old);
    }
    EXPORT_SYMBOL_GPL(replace_page_cache_folio);
    noinline int __filemap_add_folio(address_space *mapping, folio *folio, pgoff_t index, gfp_t gfp, void **shadowp)
    {
    XA_STATE_ORDER(xas, &mapping.i_pages, index, folio_order(folio));
    let mut huge = 0;
    let mut nr = 0;
pub static mut forder: c_uint = 0;
    VM_BUG_ON_FOLIO(!folio_test_locked(folio), folio);
    VM_BUG_ON_FOLIO(folio_test_swapbacked(folio), folio);
    VM_BUG_ON_FOLIO(folio_order(folio) < mapping_min_folio_order(mapping),
    folio);
    mapping_set_update(&xas, mapping);
    VM_BUG_ON_FOLIO(index & (folio_nr_pages(folio) - 1), folio);
    huge = folio_test_hugetlb(folio);
    nr = folio_nr_pages(folio);
    gfp &= GFP_RECLAIM_MASK;
    folio_ref_add(folio, nr);
    folio.mapping = mapping;
    folio.index = xas.xa_index;
    for (;;) {
pub static mut order: c_int = 0;
    void *entry, *old = core::ptr::null_mut();
    xas_lock_irq(&xas);
    xas_for_each_conflict(&xas, entry) {
    old = entry;
    if (!xa_is_value(entry)) {
    xas_set_err(&xas, -EEXIST);
// goto;
    }
//
// If a larger entry exists,
// it will be the first and only entry iterated.
//
    if (order == -1) {
    order = xas_get_order(&xas);
    }
    }
    if (old) {
    if (order > 0 && order > forder) {
    let mut split_order = max(forder,
    xas_try_split_min_order(order));
// How to handle large swap entries?
    BUG_ON!(shmem_mapping(mapping));
    while (order > forder) {
    xas_set_order(&xas, index, split_order);
    xas_try_split(&xas, old, order);
    if (xas_error(&xas)) {
// goto;
    }
    order = split_order;
    split_order =
    max(xas_try_split_min_order(
    split_order),
    forder);
    }
    xas_reset(&xas);
    }
    if (shadowp) {
// shadowp = old;
    }
    }
    xas_store(&xas, folio);
    if (xas_error(&xas)) {
// goto;
    }
    mapping.nrpages += nr;
// hugetlb pages do not participate in page cache accounting
    if (!huge) {
    lruvec_stat_mod_folio(folio, NR_FILE_PAGES, nr);
    if (folio_test_pmd_mappable(folio)) {
    lruvec_stat_mod_folio(folio,
    NR_FILE_THPS, nr);
    }
    }
// label;
    xas_unlock_irq(&xas);
    if (!xas_nomem(&xas, gfp)) {
    break;
    }
//
// Lock has been dropped: start again with the original index
// and order (but now with the memory reserved by xas_nomem()).
//
    xas_set_order(&xas, index, forder);
    }
    if (xas_error(&xas)) {
// goto;
    }
    trace_mm_filemap_add_to_page_cache(folio);
    return 0;
// label;
    folio.mapping = core::ptr::null_mut();
// Leave folio->index set: truncation relies upon it
    folio_put_refs(folio, nr);
    return xas_error(&xas);
    }
    ALLOW_ERROR_INJECTION(__filemap_add_folio, ERRNO);
#[no_mangle]
pub unsafe extern "C" fn filemap_add_folio(mapping: *mut address_space, folio: *mut folio, index: pgoff_t, gfp: gfp_t) -> c_int {
    let mut shadow = core::ptr::null_mut();
    let mut ret = 0;
pub static mut tmp: *mut c_void = core::ptr::null_mut();
pub static mut kernel_file: bool = false;
    if (kernel_file) {
    tmp = set_active_memcg(root_mem_cgroup);
    }
    ret = mem_cgroup_charge(folio, core::ptr::null_mut(), gfp);
    if (kernel_file) {
    set_active_memcg(tmp);
    }
    if (ret) {
    return ret;
    }
    __folio_set_locked(folio);
    ret = __filemap_add_folio(mapping, folio, index, gfp, &shadow);
    if (unlikely(ret)) {
    mem_cgroup_uncharge(folio);
    __folio_clear_locked(folio);
    } else {
//
// The folio might have been evicted from cache only
// recently, in which case it should be activated like
// any other repeatedly accessed folio.
// The exception is folios getting rewritten; evicting other
// data from the working set, only to cache data that will
// get overwritten with something else, is a waste of memory.
//
    WARN_ON_ONCE!(folio_test_active(folio));
    if (!(gfp & __GFP_WRITE) && shadow) {
    workingset_refault(folio, shadow);
    }
    folio_add_lru(folio);
    if (kernel_file) {
    mod_node_page_state(folio_pgdat(folio),
    NR_KERNEL_FILE_PAGES,
    folio_nr_pages(folio));
    }
    }
    return ret;
    }
    EXPORT_SYMBOL_GPL(filemap_add_folio);

#[no_mangle]
pub unsafe extern "C" fn filemap_alloc_folio_noprof(gfp: gfp_t, order: c_uint, policy: *mut mempolicy) -> *mut c_void {
    let mut n = 0;
pub static mut folio: *mut c_void = core::ptr::null_mut();
    if (policy) {
    return folio_alloc_mpol_noprof(gfp, order, policy,
    NO_INTERLEAVE_INDEX, numa_node_id());
    }
    if (cpuset_do_page_mem_spread()) {
    let mut cpuset_mems_cookie = 0;
    do {
    cpuset_mems_cookie = read_mems_allowed_begin();
    n = cpuset_mem_spread_node();
    folio = __folio_alloc_node_noprof(gfp, order, n);
    } while (!folio && read_mems_allowed_retry(cpuset_mems_cookie));
    return folio;
    }
    return folio_alloc_noprof(gfp, order);
    }
    EXPORT_SYMBOL(filemap_alloc_folio_noprof);

//
// filemap_invalidate_lock_two - lock invalidate_lock for two mappings
//
// Lock exclusively invalidate_lock of any passed mapping that is not NULL.
//
// @mapping1: the first mapping to lock
// @mapping2: the second mapping to lock
//
#[no_mangle]
pub unsafe extern "C" fn filemap_invalidate_lock_two(mapping1: *mut address_space, mapping2: *mut address_space) {
    if (mapping1 > mapping2) {
    swap(mapping1, mapping2);
    }
    if (mapping1) {
    down_write(&mapping1.invalidate_lock);
    }
    if (mapping2 && mapping1 != mapping2) {
    down_write_nested(&mapping2.invalidate_lock, 1);
    }
    }
    EXPORT_SYMBOL(filemap_invalidate_lock_two);
//
// filemap_invalidate_unlock_two - unlock invalidate_lock for two mappings
//
// Unlock exclusive invalidate_lock of any passed mapping that is not NULL.
//
// @mapping1: the first mapping to unlock
// @mapping2: the second mapping to unlock
//
#[no_mangle]
pub unsafe extern "C" fn filemap_invalidate_unlock_two(mapping1: *mut address_space, mapping2: *mut address_space) {
    if (mapping1) {
    up_write(&mapping1.invalidate_lock);
    }
    if (mapping2 && mapping1 != mapping2) {
    up_write(&mapping2.invalidate_lock);
    }
    }
    EXPORT_SYMBOL(filemap_invalidate_unlock_two);
//
// In order to wait for pages to become available there must be
// waitqueues associated with pages. By using a hash table of
// waitqueues where the bucket discipline is to maintain all
// waiters on the same queue and wake all when any of the pages
// become available, and for the woken contexts to check to be
// sure the appropriate page became available, this saves space
// at a cost of "thundering herd" phenomena during rare hash
// collisions.
//
pub const PAGE_WAIT_TABLE_BITS: c_int = 8;

    static wait_queue_head_t folio_wait_table[PAGE_WAIT_TABLE_SIZE] __cacheline_aligned;
    static wait_queue_head_t *folio_waitqueue(folio *folio)
    {
    return &folio_wait_table[hash_ptr(folio, PAGE_WAIT_TABLE_BITS)];
    }
// How many times do we accept lock stealing from under a waiter?
pub static mut sysctl_page_lock_unfairness: int = 5;
pub static mut ctl_table: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn pagecache_init()  {
    let mut i = 0;
    for (i = 0; i < PAGE_WAIT_TABLE_SIZE; i++) {
    init_waitqueue_head(&folio_wait_table[i]);
    }
    page_writeback_init();
    register_sysctl_init("vm", filemap_sysctl_table);
    }
//
// The page wait code treats the "wait->flags" somewhat unusually, because
// we have multiple different kinds of waits, not just the usual "exclusive"
// one.
//
// We have:
//
// (a) no special bits set:
//
// We're just waiting for the bit to be released, and when a waker
// calls the wakeup function, we set WQ_FLAG_WOKEN and wake it up,
// and remove it from the wait queue.
//
// Simple and straightforward.
//
// (b) WQ_FLAG_EXCLUSIVE:
//
// The waiter is waiting to get the lock, and only one waiter should
// be woken up to avoid any thundering herd behavior. We'll set the
// WQ_FLAG_WOKEN bit, wake it up, and remove it from the wait queue.
//
// This is the traditional exclusive wait.
//
// (c) WQ_FLAG_EXCLUSIVE | WQ_FLAG_CUSTOM:
//
// The waiter is waiting to get the bit, and additionally wants the
// lock to be transferred to it for fair lock behavior. If the lock
// cannot be taken, we stop walking the wait queue without waking
// the waiter.
//
// This is the "fair lock handoff" case, and in addition to setting
// WQ_FLAG_WOKEN, we set WQ_FLAG_DONE to let the waiter easily see
// that it now has the lock.
//
#[no_mangle]
unsafe extern "C" fn wake_page_function(wait: *mut wait_queue_entry_t, mode: unsigned, sync: c_int, arg: *mut c_void) -> c_int {
    let mut flags = 0;
    let mut key = arg;
    let mut wait_page = container_of!(wait, wait_page_queue, wait);
    if (!wake_page_match(wait_page, key)) {
    return 0;
    }
//
// If it's a lock handoff wait, we get the bit for it, and
// stop walking (and do not wake it up) if we can't.
//
    flags = wait.flags;
    if (flags & WQ_FLAG_EXCLUSIVE) {
    if (test_bit(key.bit_nr, &key.folio.flags.f)) {
    return -1;
    }
    if (flags & WQ_FLAG_CUSTOM) {
    if (test_and_set_bit(key.bit_nr, &key.folio.flags.f)) {
    return -1;
    }
    flags |= WQ_FLAG_DONE;
    }
    }
//
// We are holding the wait-queue lock, but the waiter that
// is waiting for this will be checking the flags without
// any locking.
//
// So update the flags atomically, and wake up the waiter
// afterwards to avoid any races. This store-release pairs
// with the load-acquire in folio_wait_bit_common().
//
    smp_store_release(&wait.flags, flags | WQ_FLAG_WOKEN);
    wake_up_state(wait.private, mode);
//
// Ok, we have successfully done what we're waiting for,
// and we can unconditionally remove the wait entry.
//
// Note that this pairs with the "finish_wait()" in the
// waiter, and has to be the absolute last thing we do.
// After this list_del_init(&wait->entry) the wait entry
// might be de-allocated and the process might even have
// exited.
//
    list_del_init_careful(&wait.entry);
    return (flags & WQ_FLAG_EXCLUSIVE) != 0;
    }
#[no_mangle]
unsafe extern "C" fn folio_wake_bit(folio: *mut folio, bit_nr: c_int) {
    let mut q = folio_waitqueue(folio);
pub static mut key: usize = 0;
    let mut flags = 0;
    key.folio = folio;
    key.bit_nr = bit_nr;
    key.page_match = 0;
    spin_lock_irqsave(&q.lock, flags);
    __wake_up_locked_key(q, TASK_NORMAL, &key);
//
// It's possible to miss clearing waiters here, when we woke our page
// waiters, but the hashed waitqueue has waiters for other pages on it.
// That's okay, it's a rare case. The next waker will clear it.
//
// Note that, depending on the page pool (buddy, hugetlb, ZONE_DEVICE,
// other), the flag may be cleared in the course of freeing the page;
// but that is not required for correctness.
//
    if (!waitqueue_active(q) || !key.page_match) {
    folio_clear_waiters(folio);
    }
    spin_unlock_irqrestore(&q.lock, flags);
    }
//
// A choice of three behaviors for folio_wait_bit_common():
//
    enum behavior {
    EXCLUSIVE,	// Hold ref to page and take the bit when woken, like
// __folio_lock() waiting on then setting PG_locked.
//
    SHARED,		// Hold ref to page and check the bit when woken, like
// folio_wait_writeback() waiting on PG_writeback.
//
    DROP,		// Drop ref to page before wait, no check when woken,
// like folio_put_wait_locked() on PG_locked.
//
    };
//
// Attempt to check (or get) the folio flag, and mark us done
// if successful.
//
#[no_mangle]
pub unsafe extern "C" fn folio_trylock_flag(folio: *mut folio, bit_nr: c_int, wait: *mut wait_queue_entry) -> bool {
    if (wait.flags & WQ_FLAG_EXCLUSIVE) {
    if (test_and_set_bit(bit_nr, &folio.flags.f)) {
    return false;
    }
    } else if (test_bit(bit_nr, &folio.flags.f)) {
    return false;
    }
    wait.flags |= WQ_FLAG_WOKEN | WQ_FLAG_DONE;
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn folio_wait_bit_common(folio: *mut folio, bit_nr: c_int, state: c_int, behavior: behavior) -> c_int {
    let mut q = folio_waitqueue(folio);
pub static mut unfairness: c_int = 0;
pub static mut wait_page: usize = 0;
    let mut wait = &wait_page.wait;
pub static mut thrashing: bool = false;
    let mut pflags = 0;
    let mut in_thrashing = 0;
    if (bit_nr == PG_locked &&
    !folio_test_uptodate(folio) && folio_test_workingset(folio)) {
    delayacct_thrashing_start(&in_thrashing);
    psi_memstall_enter(&pflags);
    thrashing = true;
    }
    init_wait(wait);
    wait.func = wake_page_function;
    wait_page.folio = folio;
    wait_page.bit_nr = bit_nr;
// label;
    wait.flags = 0;
    if (behavior == EXCLUSIVE) {
    wait.flags = WQ_FLAG_EXCLUSIVE;
    if (--unfairness < 0) {
    wait.flags |= WQ_FLAG_CUSTOM;
    }
    }
//
// Do one last check whether we can get the
// page bit synchronously.
//
// Do the folio_set_waiters() marking before that
// to let any waker we _just_ missed know they
// need to wake us up (otherwise they'll never
// even go to the slow case that looks at the
// page queue), and add ourselves to the wait
// queue if we need to sleep.
//
// This part needs to be done under the queue
// lock to avoid races.
//
    spin_lock_irq(&q.lock);
    folio_set_waiters(folio);
    if (!folio_trylock_flag(folio, bit_nr, wait)) {
    __add_wait_queue_entry_tail(q, wait);
    }
    spin_unlock_irq(&q.lock);
//
// From now on, all the logic will be based on
// the WQ_FLAG_WOKEN and WQ_FLAG_DONE flag, to
// see whether the page bit testing has already
// been done by the wake function.
//
// We can drop our reference to the folio.
//
    if (behavior == DROP) {
    folio_put(folio);
    }
//
// Note that until the "finish_wait()", or until
// we see the WQ_FLAG_WOKEN flag, we need to
// be very careful with the 'wait->flags', because
// we may race with a waker that sets them.
//
    for (;;) {
    let mut flags = 0;
    set_current_state(state);
// Loop until we've been woken or interrupted
    flags = smp_load_acquire(&wait.flags);
    if (!(flags & WQ_FLAG_WOKEN)) {
    if (signal_pending_state(state, current)) {
    break;
    }
    io_schedule();
    continue;
    }
// If we were non-exclusive, we're done
    if (behavior != EXCLUSIVE) {
    break;
    }
// If the waker got the lock for us, we're done
    if (flags & WQ_FLAG_DONE) {
    break;
    }
//
// Otherwise, if we're getting the lock, we need to
// try to get it ourselves.
//
// And if that fails, we'll have to retry this all.
//
    if (unlikely(test_and_set_bit(bit_nr, folio_flags(folio, 0)))) {
// goto;
    }
    wait.flags |= WQ_FLAG_DONE;
    break;
    }
//
// If a signal happened, this 'finish_wait()' may remove the last
// waiter from the wait-queues, but the folio waiters bit will remain
// set. That's ok. The next wakeup will take care of it, and trying
// to do it here would be difficult and prone to races.
//
    finish_wait(q, wait);
    if (thrashing) {
    delayacct_thrashing_end(&in_thrashing);
    psi_memstall_leave(&pflags);
    }
//
// NOTE! The wait->flags weren't stable until we've done the
// 'finish_wait()', and we could have exited the loop above due
// to a signal, and had a wakeup event happen after the signal
// test but before the 'finish_wait()'.
//
// So only after the finish_wait() can we reliably determine
// if we got woken up or not, so we can now figure out the final
// return value based on that state without races.
//
// Also note that WQ_FLAG_WOKEN is sufficient for a non-exclusive
// waiter, but an exclusive one requires WQ_FLAG_DONE.
//
    if (behavior == EXCLUSIVE) {
    return wait.flags & WQ_FLAG_DONE ? 0 : -EINTR;
    }
    return wait.flags & WQ_FLAG_WOKEN ? 0 : -EINTR;
    }

//
// softleaf_entry_wait_on_locked - Wait for a migration entry or
// device_private entry to be removed.
// @entry: migration or device_private swap entry.
// @ptl: already locked ptl. This function will drop the lock.
//
// Wait for a migration entry referencing the given page, or device_private
// entry referencing a dvice_private page to be unlocked. This is
// equivalent to folio_put_wait_locked(folio, TASK_UNINTERRUPTIBLE) except
// this can be called without taking a reference on the page. Instead this
// should be called while holding the ptl for @entry referencing
// the page.
//
// Returns after unlocking the ptl.
//
// This follows the same logic as folio_wait_bit_common() so see the comments
// there.
//
#[no_mangle]
pub unsafe extern "C" fn softleaf_entry_wait_on_locked(entry: softleaf_t, ptl: *mut spinlock_t) {
pub static mut wait_page: usize = 0;
    let mut wait = &wait_page.wait;
pub static mut thrashing: bool = false;
    let mut pflags = 0;
    let mut in_thrashing = 0;
pub static mut q: *mut c_void = core::ptr::null_mut();
    let mut folio = softleaf_to_folio(entry);
    q = folio_waitqueue(folio);
    if (!folio_test_uptodate(folio) && folio_test_workingset(folio)) {
    delayacct_thrashing_start(&in_thrashing);
    psi_memstall_enter(&pflags);
    thrashing = true;
    }
    init_wait(wait);
    wait.func = wake_page_function;
    wait_page.folio = folio;
    wait_page.bit_nr = PG_locked;
    wait.flags = 0;
    spin_lock_irq(&q.lock);
    folio_set_waiters(folio);
    if (!folio_trylock_flag(folio, PG_locked, wait)) {
    __add_wait_queue_entry_tail(q, wait);
    }
    spin_unlock_irq(&q.lock);
//
// If a migration entry exists for the page the migration path must hold
// a valid reference to the page, and it must take the ptl to remove the
// migration entry. So the page is valid until the ptl is dropped.
// Similarly any path attempting to drop the last reference to a
// device-private page needs to grab the ptl to remove the device-private
// entry.
//
    spin_unlock(ptl);
    for (;;) {
    let mut flags = 0;
    set_current_state(TASK_UNINTERRUPTIBLE);
// Loop until we've been woken or interrupted
    flags = smp_load_acquire(&wait.flags);
    if (!(flags & WQ_FLAG_WOKEN)) {
    if (signal_pending_state(TASK_UNINTERRUPTIBLE, current)) {
    break;
    }
    io_schedule();
    continue;
    }
    break;
    }
    finish_wait(q, wait);
    if (thrashing) {
    delayacct_thrashing_end(&in_thrashing);
    psi_memstall_leave(&pflags);
    }
    }

#[no_mangle]
pub unsafe extern "C" fn folio_wait_bit(folio: *mut folio, bit_nr: c_int) {
    folio_wait_bit_common(folio, bit_nr, TASK_UNINTERRUPTIBLE, SHARED);
    }
    EXPORT_SYMBOL(folio_wait_bit);
#[no_mangle]
pub unsafe extern "C" fn folio_wait_bit_killable(folio: *mut folio, bit_nr: c_int) -> c_int {
    return folio_wait_bit_common(folio, bit_nr, TASK_KILLABLE, SHARED);
    }
    EXPORT_SYMBOL(folio_wait_bit_killable);
//
// folio_put_wait_locked - Drop a reference and wait for it to be unlocked
// @folio: The folio to wait for.
// @state: The sleep state (TASK_KILLABLE, TASK_UNINTERRUPTIBLE, etc).
//
// The caller should hold a reference on @folio.  They expect the page to
// become unlocked relatively soon, but do not wish to hold up migration
// (for example) by holding the reference while waiting for the folio to
// come unlocked.  After this function returns, the caller should not
// dereference @folio.
//
// Return: 0 if the folio was unlocked or -EINTR if interrupted by a signal.
//
#[no_mangle]
unsafe extern "C" fn folio_put_wait_locked(folio: *mut folio, state: c_int) -> c_int {
    return folio_wait_bit_common(folio, PG_locked, state, DROP);
    }
//
// folio_unlock - Unlock a locked folio.
// @folio: The folio.
//
// Unlocks the folio and wakes up any thread sleeping on the page lock.
//
// Context: May be called from interrupt or process context.  May not be
// called from NMI context.
//
#[no_mangle]
pub unsafe extern "C" fn folio_unlock(folio: *mut folio) {
// Bit 7 allows x86 to check the byte's sign bit
    BUILD_BUG_ON!(PG_waiters != 7);
    BUILD_BUG_ON!(PG_locked > 7);
    VM_BUG_ON_FOLIO(!folio_test_locked(folio), folio);
    if (folio_xor_flags_has_waiters(folio, 1 << PG_locked)) {
    folio_wake_bit(folio, PG_locked);
    }
    }
    EXPORT_SYMBOL(folio_unlock);
//
// folio_end_read - End read on a folio.
// @folio: The folio.
// @success: True if all reads completed successfully.
//
// When all reads against a folio have completed, filesystems should
// call this function to let the pagecache know that no more reads
// are outstanding.  This will unlock the folio and wake up any thread
// sleeping on the lock.  The folio will also be marked uptodate if all
// reads succeeded.
//
// Context: May be called from interrupt or process context.  May not be
// called from NMI context.
//
#[no_mangle]
pub unsafe extern "C" fn folio_end_read(folio: *mut folio, success: bool) {
pub static mut mask: c_ulong = 0;
// Must be in bottom byte for x86 to work
    BUILD_BUG_ON!(PG_uptodate > 7);
    VM_BUG_ON_FOLIO(!folio_test_locked(folio), folio);
    VM_BUG_ON_FOLIO(success && folio_test_uptodate(folio), folio);
    if (likely(success)) {
    mask |= 1 << PG_uptodate;
    }
    if (folio_xor_flags_has_waiters(folio, mask)) {
    folio_wake_bit(folio, PG_locked);
    }
    }
    EXPORT_SYMBOL(folio_end_read);
//
// folio_end_private_2 - Clear PG_private_2 and wake any waiters.
// @folio: The folio.
//
// Clear the PG_private_2 bit on a folio and wake up any sleepers waiting for
// it.  The folio reference held for PG_private_2 being set is released.
//
// This is, for example, used when a netfs folio is being written to a local
// disk cache, thereby allowing writes to the cache for the same folio to be
// serialised.
//
#[no_mangle]
pub unsafe extern "C" fn folio_end_private_2(folio: *mut folio) {
    VM_BUG_ON_FOLIO(!folio_test_private_2(folio), folio);
    clear_bit_unlock(PG_private_2, folio_flags(folio, 0));
    folio_wake_bit(folio, PG_private_2);
    folio_put(folio);
    }
    EXPORT_SYMBOL(folio_end_private_2);
//
// folio_wait_private_2 - Wait for PG_private_2 to be cleared on a folio.
// @folio: The folio to wait on.
//
// Wait for PG_private_2 to be cleared on a folio.
//
#[no_mangle]
pub unsafe extern "C" fn folio_wait_private_2(folio: *mut folio) {
    while (folio_test_private_2(folio)) {
    folio_wait_bit(folio, PG_private_2);
    }
    }
    EXPORT_SYMBOL(folio_wait_private_2);
//
// folio_wait_private_2_killable - Wait for PG_private_2 to be cleared on a folio.
// @folio: The folio to wait on.
//
// Wait for PG_private_2 to be cleared on a folio or until a fatal signal is
// received by the calling task.
//
// Return:
// - 0 if successful.
// - -EINTR if a fatal signal was encountered.
//
#[no_mangle]
pub unsafe extern "C" fn folio_wait_private_2_killable(folio: *mut folio) -> c_int {
pub static mut ret: c_int = 0;
    while (folio_test_private_2(folio)) {
    ret = folio_wait_bit_killable(folio, PG_private_2);
    if (ret < 0) {
    break;
    }
    }
    return ret;
    }
    EXPORT_SYMBOL(folio_wait_private_2_killable);
#[no_mangle]
unsafe extern "C" fn filemap_end_dropbehind(folio: *mut folio) {
    let mut mapping = folio.mapping;
    VM_BUG_ON_FOLIO(!folio_test_locked(folio), folio);
    if (folio_test_writeback(folio) || folio_test_dirty(folio)) {
    return;
    }
    if (!folio_test_clear_dropbehind(folio)) {
    return;
    }
    if (mapping) {
    folio_unmap_invalidate(mapping, folio, 0);
    }
    }
//
// If folio was marked as dropbehind, then pages should be dropped when writeback
// completes. Do that now. If we fail, it's likely because of a big folio -
// just reset dropbehind for that case and latter completions should invalidate.
//
#[no_mangle]
pub unsafe extern "C" fn folio_end_dropbehind(folio: *mut folio) {
    if (!folio_test_dropbehind(folio)) {
    return;
    }
//
// Hitting !in_task() should not happen off RWF_DONTCACHE writeback,
// but can happen if normal writeback just happens to find dirty folios
// that were created as part of uncached writeback, and that writeback
// would otherwise not need non-IRQ handling. Just skip the
// invalidation in that case.
//
    if (in_task() && folio_trylock(folio)) {
    filemap_end_dropbehind(folio);
    folio_unlock(folio);
    }
    }
    EXPORT_SYMBOL_GPL(folio_end_dropbehind);
//
// folio_end_writeback_no_dropbehind - End writeback against a folio.
// @folio: The folio.
//
// The folio must actually be under writeback.
// This call is intended for filesystems that need to defer dropbehind.
//
// Context: May be called from process or interrupt context.
//
#[no_mangle]
pub unsafe extern "C" fn folio_end_writeback_no_dropbehind(folio: *mut folio) {
    VM_BUG_ON_FOLIO(!folio_test_writeback(folio), folio);
//
// folio_test_clear_reclaim() could be used here but it is an
// atomic operation and overkill in this particular case. Failing
// to shuffle a folio marked for immediate reclaim is too mild
// a gain to justify taking an atomic operation penalty at the
// end of every folio writeback.
//
    if (folio_test_reclaim(folio)) {
    folio_clear_reclaim(folio);
    folio_rotate_reclaimable(folio);
    }
    if (__folio_end_writeback(folio)) {
    folio_wake_bit(folio, PG_writeback);
    }
    acct_reclaim_writeback(folio);
    }
    EXPORT_SYMBOL_GPL(folio_end_writeback_no_dropbehind);
//
// folio_end_writeback - End writeback against a folio.
// @folio: The folio.
//
// The folio must actually be under writeback.
//
// Context: May be called from process or interrupt context.
//
#[no_mangle]
pub unsafe extern "C" fn folio_end_writeback(folio: *mut folio) {
    VM_BUG_ON_FOLIO(!folio_test_writeback(folio), folio);
//
// Writeback does not hold a folio reference of its own, relying
// on truncation to wait for the clearing of PG_writeback.
// But here we must make sure that the folio is not freed and
// reused before the folio_wake_bit().
//
    folio_get(folio);
    folio_end_writeback_no_dropbehind(folio);
    folio_end_dropbehind(folio);
    folio_put(folio);
    }
    EXPORT_SYMBOL(folio_end_writeback);
//
// __folio_lock - Get a lock on the folio, assuming we need to sleep to get it.
// @folio: The folio to lock
//
#[no_mangle]
pub unsafe extern "C" fn __folio_lock(folio: *mut folio) {
    folio_wait_bit_common(folio, PG_locked, TASK_UNINTERRUPTIBLE,
    EXCLUSIVE);
    }
    EXPORT_SYMBOL(__folio_lock);
#[no_mangle]
pub unsafe extern "C" fn __folio_lock_killable(folio: *mut folio) -> c_int {
    return folio_wait_bit_common(folio, PG_locked, TASK_KILLABLE,
    EXCLUSIVE);
    }
    EXPORT_SYMBOL_GPL(__folio_lock_killable);
#[no_mangle]
unsafe extern "C" fn __folio_lock_async(folio: *mut folio, wait: *mut wait_page_queue) -> c_int {
    let mut q = folio_waitqueue(folio);
    let mut ret = 0;
    wait.folio = folio;
    wait.bit_nr = PG_locked;
    spin_lock_irq(&q.lock);
    __add_wait_queue_entry_tail(q, &wait.wait);
    folio_set_waiters(folio);
    ret = !folio_trylock(folio);
//
// If we were successful now, we know we're still on the
// waitqueue as we're still under the lock. This means it's
// safe to remove and return success, we know the callback
// isn't going to trigger.
//
    if (!ret) {
    __remove_wait_queue(q, &wait.wait);
    }
    else {
    ret = -EIOCBQUEUED;
    }
    spin_unlock_irq(&q.lock);
    return ret;
    }
//
// Return values:
// 0 - folio is locked.
// non-zero - folio is not locked.
// mmap_lock or per-VMA lock has been released (mmap_read_unlock() or
// vma_end_read()), unless flags had both FAULT_FLAG_ALLOW_RETRY and
// FAULT_FLAG_RETRY_NOWAIT set, in which case the lock is still held.
//
// If neither ALLOW_RETRY nor KILLABLE are set, will always return 0
// with the folio locked and the mmap_lock/per-VMA lock is left unperturbed.
//
#[no_mangle]
pub unsafe extern "C" fn __folio_lock_or_retry(folio: *mut folio, vmf: *mut vm_fault) -> vm_fault_t {
pub static mut flags: c_uint = 0;
    if (fault_flag_allow_retry_first(flags)) {
//
// CAUTION! In this case, mmap_lock/per-VMA lock is not
// released even though returning VM_FAULT_RETRY.
//
    if (flags & FAULT_FLAG_RETRY_NOWAIT) {
    return VM_FAULT_RETRY;
    }
    release_fault_lock(vmf);
    if (flags & FAULT_FLAG_KILLABLE) {
    folio_wait_locked_killable(folio);
    }
    else {
    folio_wait_locked(folio);
    }
    return VM_FAULT_RETRY;
    }
    if (flags & FAULT_FLAG_KILLABLE) {
    let mut ret = 0;
    ret = __folio_lock_killable(folio);
    if (ret) {
    release_fault_lock(vmf);
    return VM_FAULT_RETRY;
    }
    } else {
    __folio_lock(folio);
    }
    return 0;
    }
//
// page_cache_next_miss() - Find the next gap in the page cache.
// @mapping: Mapping.
// @index: Index.
// @max_scan: Maximum range to search.
//
// Search the range [index, min(index + max_scan - 1, ULONG_MAX)] for the
// gap with the lowest index.
//
// This function may be called under the rcu_read_lock.  However, this will
// not atomically search a snapshot of the cache at a single point in time.
// For example, if a gap is created at index 5, then subsequently a gap is
// created at index 10, page_cache_next_miss covering both indices may
// return 10 if called under the rcu_read_lock.
//
// Return: The index of the gap if found, otherwise an index outside the
// range specified (in which case 'return - index >= max_scan' will be true).
// In the rare case of index wrap-around, 0 will be returned.
//
    pgoff_t page_cache_next_miss(address_space *mapping,
    pgoff_t index, unsigned long max_scan)
    {
    XA_STATE(xas, &mapping.i_pages, index);
    while (max_scan--) {
    let mut entry = xas_next(&xas);
    if (!entry || xa_is_value(entry)) {
    return xas.xa_index;
    }
    if (xas.xa_index == 0) {
    return 0;
    }
    }
// Return end of the range + 1 when no hole is found
    return xas.xa_index + 1;
    }
    EXPORT_SYMBOL(page_cache_next_miss);
//
// page_cache_prev_miss() - Find the previous gap in the page cache.
// @mapping: Mapping.
// @index: Index.
// @max_scan: Maximum range to search.
//
// Search the range [max(index - max_scan + 1, 0), index] for the
// gap with the highest index.
//
// This function may be called under the rcu_read_lock.  However, this will
// not atomically search a snapshot of the cache at a single point in time.
// For example, if a gap is created at index 10, then subsequently a gap is
// created at index 5, page_cache_prev_miss() covering both indices may
// return 5 if called under the rcu_read_lock.
//
// Return: The index of the gap if found, otherwise an index outside the
// range specified (in which case 'index - return >= max_scan' will be true).
// In the rare case of wrap-around, ULONG_MAX will be returned.
//
    pgoff_t page_cache_prev_miss(address_space *mapping,
    pgoff_t index, unsigned long max_scan)
    {
    XA_STATE(xas, &mapping.i_pages, index);
    while (max_scan--) {
    let mut entry = xas_prev(&xas);
    if (!entry || xa_is_value(entry)) {
    return xas.xa_index;
    }
    if (xas.xa_index == ULONG_MAX) {
    return ULONG_MAX;
    }
    }
// Return start of the range - 1 when no hole is found
    return xas.xa_index - 1;
    }
    EXPORT_SYMBOL(page_cache_prev_miss);
//
// Lockless page cache protocol:
// On the lookup side:
// 1. Load the folio from i_pages
// 2. Increment the refcount if it's not zero
// 3. If the folio is not found by xas_reload(), put the refcount and retry
//
// On the removal side:
// A. Freeze the page (by zeroing the refcount if nobody else has a reference)
// B. Remove the page from i_pages
// C. Return the page to the page allocator
//
// This means that any page may have its reference count temporarily
// increased by a speculative page cache (or GUP-fast) lookup as it can
// be allocated by another user before the RCU grace period expires.
// Because the refcount temporarily acquired here may end up being the
// last refcount on the page, any page allocation must be freeable by
// folio_put().
//
// filemap_get_entry - Get a page cache entry.
// @mapping: the address_space to search
// @index: The page cache index.
//
// Looks up the page cache entry at @mapping & @index.  If it is a folio,
// it is returned with an increased refcount.  If it is a shadow entry
// of a previously evicted folio, or a swap entry from shmem/tmpfs,
// it is returned without further action.
//
// Return: The folio, swap or shadow entry, %NULL if nothing is found.
//
#[no_mangle]
pub unsafe extern "C" fn filemap_get_entry(mapping: *mut address_space, index: pgoff_t) -> *mut c_void {
    XA_STATE(xas, &mapping.i_pages, index);
pub static mut folio: *mut c_void = core::ptr::null_mut();
    rcu_read_lock();
// label;
    xas_reset(&xas);
    folio = xas_load(&xas);
    if (xas_retry(&xas, folio)) {
// goto;
    }
//
// A shadow entry of a recently evicted page, or a swap entry from
// shmem/tmpfs.  Return it without attempting to raise page count.
//
    if (!folio || xa_is_value(folio)) {
// goto;
    }
    if (!folio_try_get(folio)) {
// goto;
    }
    if (unlikely(folio != xas_reload(&xas))) {
    folio_put(folio);
// goto;
    }
// label;
    rcu_read_unlock();
    return folio;
    }
//
// __filemap_get_folio_mpol - Find and get a reference to a folio.
// @mapping: The address_space to search.
// @index: The page index.
// @fgp_flags: %FGP flags modify how the folio is returned.
// @gfp: Memory allocation flags to use if %FGP_CREAT is specified.
// @policy: NUMA memory allocation policy to follow.
//
// Looks up the page cache entry at @mapping & @index.
//
// If %FGP_LOCK or %FGP_CREAT are specified then the function may sleep even
// if the %GFP flags specified for %FGP_CREAT are atomic.
//
// If this function returns a folio, it is returned with an increased refcount.
//
// Return: The found folio or an ERR_PTR() otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn __filemap_get_folio_mpol(mapping: *mut address_space, index: pgoff_t, fgp_flags: fgf_t, gfp: gfp_t, policy: *mut mempolicy) -> *mut c_void {
pub static mut folio: *mut c_void = core::ptr::null_mut();
// label;
    folio = filemap_get_entry(mapping, index);
    if (xa_is_value(folio)) {
    folio = core::ptr::null_mut();
    }
    if (!folio) {
// goto;
    }
    if (fgp_flags & FGP_LOCK) {
    if (fgp_flags & FGP_NOWAIT) {
    if (!folio_trylock(folio)) {
    folio_put(folio);
    return ERR_PTR(-EAGAIN);
    }
    } else {
    folio_lock(folio);
    }
// Has the page been truncated?
    if (unlikely(folio.mapping != mapping)) {
    folio_unlock(folio);
    folio_put(folio);
// goto;
    }
    VM_BUG_ON_FOLIO(!folio_contains(folio, index), folio);
    }
    if (fgp_flags & FGP_ACCESSED) {
    folio_mark_accessed(folio);
    }
if true {
// Clear idle flag for buffer write
    if (folio_test_idle(folio)) {
    folio_clear_idle(folio);
    }
    }
    if (fgp_flags & FGP_STABLE) {
    folio_wait_stable(folio);
    }
// label;
    if (!folio && (fgp_flags & FGP_CREAT)) {
pub static mut min_order: c_uint = 0;
pub static mut order: c_uint = 0;
    let mut err = 0;
    index = mapping_align_index(mapping, index);
    if ((fgp_flags & FGP_WRITE) && mapping_can_writeback(mapping)) {
    gfp |= __GFP_WRITE;
    }
    if (fgp_flags & FGP_NOFS) {
    gfp &= ~__GFP_FS;
    }
    if (fgp_flags & FGP_NOWAIT) {
    gfp &= ~GFP_KERNEL;
    gfp |= GFP_NOWAIT;
    }
    if (WARN_ON_ONCE!(!(fgp_flags & (FGP_LOCK | FGP_FOR_MMAP)))) {
    fgp_flags |= FGP_LOCK;
    }
    if (order > mapping_max_folio_order(mapping)) {
    order = mapping_max_folio_order(mapping);
    }
// If we're not aligned, allocate a smaller folio
    if (index & ((1UL << order) - 1)) {
    order = __ffs(index);
    }
    do {
pub static mut alloc_gfp: gfp_t = 0;
    err = -ENOMEM;
    if (order > min_order) {
    alloc_gfp |= __GFP_NORETRY | __GFP_NOWARN;
    }
    folio = filemap_alloc_folio(alloc_gfp, order, policy);
    if (!folio) {
    continue;
    }
// Init accessed so avoid atomic mark_page_accessed later
    if (fgp_flags & FGP_ACCESSED) {
    __folio_set_referenced(folio);
    }
    if (fgp_flags & FGP_DONTCACHE) {
    __folio_set_dropbehind(folio);
    }
    err = filemap_add_folio(mapping, folio, index, gfp);
    if (!err) {
    break;
    }
    folio_put(folio);
    folio = core::ptr::null_mut();
    } while (order-- > min_order);
    if (err == -EEXIST) {
// goto;
    }
    if (err) {
//
// When NOWAIT I/O fails to allocate folios this could
// be due to a nonblocking memory allocation and not
// because the system actually is out of memory.
// Return -EAGAIN so that there caller retries in a
// blocking fashion instead of propagating -ENOMEM
// to the application.
//
    if ((fgp_flags & FGP_NOWAIT) && err == -ENOMEM) {
    err = -EAGAIN;
    }
    return ERR_PTR(err);
    }
//
// filemap_add_folio locks the page, and for mmap
// we expect an unlocked page.
//
    if (folio && (fgp_flags & FGP_FOR_MMAP)) {
    folio_unlock(folio);
    }
    }
    if (!folio) {
    return ERR_PTR(-ENOENT);
    }
// not an uncached lookup, clear uncached if set
    if (!(fgp_flags & FGP_DONTCACHE) && folio_test_clear_dropbehind(folio)) {
    if (folio_test_dirty(folio) &&
    mapping_can_writeback(mapping)) {
    let mut inode = mapping.host;
pub static mut wb: *mut c_void = core::ptr::null_mut();
pub static mut cookie: wb_lock_cookie = 0;
pub static mut nr: c_long = 0;
    wb = unlocked_inode_to_wb_begin(inode, &cookie);
    wb_stat_mod(wb, WB_DONTCACHE_DIRTY, -nr);
    unlocked_inode_to_wb_end(inode, &cookie);
    }
    }
    return folio;
    }
    EXPORT_SYMBOL(__filemap_get_folio_mpol);
#[no_mangle]
pub unsafe extern "C" fn find_get_entry(xas: *mut xa_state, max: pgoff_t, mark: xa_mark_t) -> *mut c_void {
pub static mut folio: *mut c_void = core::ptr::null_mut();
// label;
    if (mark == XA_PRESENT) {
    folio = xas_find(xas, max);
    }
    else {
    folio = xas_find_marked(xas, max, mark);
    }
    if (xas_retry(xas, folio)) {
// goto;
    }
//
// A shadow entry of a recently evicted page, a swap
// entry from shmem/tmpfs or a DAX entry.  Return it
// without attempting to raise page count.
//
    if (!folio || xa_is_value(folio)) {
    return folio;
    }
    if (!folio_try_get(folio)) {
// goto;
    }
    if (unlikely(folio != xas_reload(xas))) {
    folio_put(folio);
// goto;
    }
    return folio;
// label;
    xas_reset(xas);
// goto;
    }
//
// find_get_entries - gang pagecache lookup
// @mapping:	The address_space to search
// @start:	The starting page cache index
// @end:	The final page index (inclusive).
// @fbatch:	Where the resulting entries are placed.
// @indices:	The cache indices corresponding to the entries in @entries
//
// find_get_entries() will search for and return a batch of entries in
// the mapping.  The entries are placed in @fbatch.  find_get_entries()
// takes a reference on any actual folios it returns.
//
// The entries have ascending indexes.  The indices may not be consecutive
// due to not-present entries or large folios.
//
// Any shadow entries of evicted folios, or swap entries from
// shmem/tmpfs, are included in the returned array.
//
// Return: The number of entries which were found.
//
#[no_mangle]
pub unsafe extern "C" fn find_get_entries(mapping: *mut address_space, start: *mut pgoff_t, end: pgoff_t, fbatch: *mut folio_batch, indices: *mut pgoff_t) -> c_uint {
    XA_STATE(xas, &mapping.i_pages, *start);
pub static mut folio: *mut c_void = core::ptr::null_mut();
    rcu_read_lock();
    while ((folio = find_get_entry(&xas, end, XA_PRESENT)) != core::ptr::null_mut()) {
    indices[fbatch.nr] = xas.xa_index;
    if (!folio_batch_add(fbatch, folio)) {
    break;
    }
    }
    if (folio_batch_count(fbatch)) {
    let mut nr = 0;
pub static mut idx: c_int = 0;
    folio = fbatch.folios[idx];
    if (!xa_is_value(folio)) {
    nr = folio_nr_pages(folio);
    }
    else {
    nr = 1 << xa_get_order(&mapping.i_pages, indices[idx]);
    }
// start = round_down(indices[idx] + nr, nr);
    }
    rcu_read_unlock();
    return folio_batch_count(fbatch);
    }
//
// find_lock_entries - Find a batch of pagecache entries.
// @mapping:	The address_space to search.
// @start:	The starting page cache index.
// @end:	The final page index (inclusive).
// @fbatch:	Where the resulting entries are placed.
// @indices:	The cache indices of the entries in @fbatch.
//
// find_lock_entries() will return a batch of entries from @mapping.
// Swap, shadow and DAX entries are included.  Folios are returned
// locked and with an incremented refcount.  Folios which are locked
// by somebody else or under writeback are skipped.  Folios which are
// partially outside the range are not returned.
//
// The entries have ascending indexes.  The indices may not be consecutive
// due to not-present entries, large folios, folios which could not be
// locked or folios under writeback.
//
// Return: The number of entries which were found.
//
#[no_mangle]
pub unsafe extern "C" fn find_lock_entries(mapping: *mut address_space, start: *mut pgoff_t, end: pgoff_t, fbatch: *mut folio_batch, indices: *mut pgoff_t) -> c_uint {
    XA_STATE(xas, &mapping.i_pages, *start);
pub static mut folio: *mut c_void = core::ptr::null_mut();
    rcu_read_lock();
    while ((folio = find_get_entry(&xas, end, XA_PRESENT))) {
    let mut base = 0;
    let mut nr = 0;
    if (!xa_is_value(folio)) {
    nr = folio_nr_pages(folio);
    base = folio.index;
// Omit large folio which begins before the start
    if (base < *start) {
// goto;
    }
// Omit large folio which extends beyond the end
    if (base + nr - 1 > end) {
// goto;
    }
    if (!folio_trylock(folio)) {
// goto;
    }
    if (folio.mapping != mapping ||
    folio_test_writeback(folio)) {
// goto;
    }
    VM_BUG_ON_FOLIO(!folio_contains(folio, xas.xa_index),
    folio);
    } else {
    nr = 1 << xas_get_order(&xas);
    base = xas.xa_index & ~(nr - 1);
// Omit order>0 value which begins before the start
    if (base < *start) {
    continue;
    }
// Omit order>0 value which extends beyond the end
    if (base + nr - 1 > end) {
    break;
    }
    }
// Update start now so that last update is correct on return
// start = base + nr;
    indices[fbatch.nr] = xas.xa_index;
    if (!folio_batch_add(fbatch, folio)) {
    break;
    }
    continue;
// label;
    folio_unlock(folio);
// label;
    folio_put(folio);
    }
    rcu_read_unlock();
    return folio_batch_count(fbatch);
    }
//
// filemap_get_folios - Get a batch of folios
// @mapping:	The address_space to search
// @start:	The starting page index
// @end:	The final page index (inclusive)
// @fbatch:	The batch to fill.
//
// Search for and return a batch of folios in the mapping starting at
// index @start and up to index @end (inclusive).  The folios are returned
// in @fbatch with an elevated reference count.
//
// Return: The number of folios which were found.
// We also update @start to index the next folio for the traversal.
//
#[no_mangle]
pub unsafe extern "C" fn filemap_get_folios(mapping: *mut address_space, start: *mut pgoff_t, end: pgoff_t, fbatch: *mut folio_batch) -> c_uint {
    return filemap_get_folios_tag(mapping, start, end, XA_PRESENT, fbatch);
    }
    EXPORT_SYMBOL(filemap_get_folios);
//
// filemap_get_folios_contig - Get a batch of contiguous folios
// @mapping:	The address_space to search
// @start:	The starting page index
// @end:	The final page index (inclusive)
// @fbatch:	The batch to fill
//
// filemap_get_folios_contig() works exactly like filemap_get_folios(),
// except the returned folios are guaranteed to be contiguous. This may
// not return all contiguous folios if the batch gets filled up.
//
// Return: The number of folios found.
// Also update @start to be positioned for traversal of the next folio.
//
#[no_mangle]
pub unsafe extern "C" fn filemap_get_folios_contig(mapping: *mut address_space, start: *mut pgoff_t, end: pgoff_t, fbatch: *mut folio_batch) -> c_uint {
    XA_STATE(xas, &mapping.i_pages, *start);
    let mut nr = 0;
pub static mut folio: *mut c_void = core::ptr::null_mut();
    if (*start > end) {
    return 0;
    }
    rcu_read_lock();
    for (folio = xas_load(&xas); folio; folio = xas_next(&xas)) {
    if (xas_retry(&xas, folio)) {
    continue;
    }
//
// If the entry has been swapped out, we can stop looking.
// No current caller is looking for DAX entries.
//
    if (xa_is_value(folio)) {
    break;
    }
// If we landed in the middle of a THP, continue at its end.
    if (xa_is_sibling(folio)) {
    break;
    }
    if (!folio_try_get(folio)) {
// goto;
    }
    if (unlikely(folio != xas_reload(&xas))) {
// goto;
    }
    if (!folio_batch_add(fbatch, folio)) {
    break;
    }
    xas_advance(&xas, folio_next_index(folio) - 1);
    if (xas.xa_index >= end) {
    break;
    }
    continue;
// label;
    folio_put(folio);
// label;
    xas_reset(&xas);
    }
    rcu_read_unlock();
    nr = folio_batch_count(fbatch);
    if (nr) {
    folio = fbatch.folios[nr - 1];
// start = folio_next_index(folio);
    }
    return nr;
    }
    EXPORT_SYMBOL(filemap_get_folios_contig);
//
// filemap_get_folios_tag - Get a batch of folios matching @tag
// @mapping:    The address_space to search
// @start:      The starting page index
// @end:        The final page index (inclusive)
// @tag:        The tag index
// @fbatch:     The batch to fill
//
// The first folio may start before @start; if it does, it will contain
// @start.  The final folio may extend beyond @end; if it does, it will
// contain @end.  The folios have ascending indices.  There may be gaps
// between the folios if there are indices which have no folio in the
// page cache.  If folios are added to or removed from the page cache
// while this is running, they may or may not be found by this call.
// Only returns folios that are tagged with @tag.
//
// Return: The number of folios found.
// Also update @start to index the next folio for traversal.
//
#[no_mangle]
pub unsafe extern "C" fn filemap_get_folios_tag(mapping: *mut address_space, start: *mut pgoff_t, end: pgoff_t, tag: xa_mark_t, fbatch: *mut folio_batch) -> c_uint {
    XA_STATE(xas, &mapping.i_pages, *start);
pub static mut folio: *mut c_void = core::ptr::null_mut();
    rcu_read_lock();
    while ((folio = find_get_entry(&xas, end, tag)) != core::ptr::null_mut()) {
//
// Shadow entries should never be tagged, but this iteration
// is lockless so there is a window for page reclaim to evict
// a page we saw tagged. Skip over it.
//
    if (xa_is_value(folio)) {
    continue;
    }
    if (!folio_batch_add(fbatch, folio)) {
// start = folio_next_index(folio);
// goto;
    }
    }
//
// We come here when there is no page beyond @end. We take care to not
// overflow the index @start as it confuses some of the callers. This
// breaks the iteration when there is a page at index -1 but that is
// already broke anyway.
//
    if (end == (pgoff_t)-1) {
// start = (pgoff_t)-1;
    }
    else {
// start = end + 1;
    }
// label;
    rcu_read_unlock();
    return folio_batch_count(fbatch);
    }
    EXPORT_SYMBOL(filemap_get_folios_tag);
//
// filemap_get_folios_dirty - Get a batch of dirty folios
// @mapping:	The address_space to search
// @start:	The starting folio index
// @end:	The final folio index (inclusive)
// @fbatch:	The batch to fill
//
// filemap_get_folios_dirty() works exactly like filemap_get_folios(), except
// the returned folios are presumed to be dirty or undergoing writeback. Dirty
// state is presumed because we don't block on folio lock nor want to miss
// folios. Callers that need to can recheck state upon locking the folio.
//
// This may not return all dirty folios if the batch gets filled up.
//
// Return: The number of folios found.
// Also update @start to be positioned for traversal of the next folio.
//
#[no_mangle]
pub unsafe extern "C" fn filemap_get_folios_dirty(mapping: *mut address_space, start: *mut pgoff_t, end: pgoff_t, fbatch: *mut folio_batch) -> c_uint {
    XA_STATE(xas, &mapping.i_pages, *start);
pub static mut folio: *mut c_void = core::ptr::null_mut();
    rcu_read_lock();
    while ((folio = find_get_entry(&xas, end, XA_PRESENT)) != core::ptr::null_mut()) {
    if (xa_is_value(folio)) {
    continue;
    }
    if (folio_trylock(folio)) {
    let mut clean = !folio_test_dirty(folio) &&
    !folio_test_writeback(folio);
    folio_unlock(folio);
    if (clean) {
    folio_put(folio);
    continue;
    }
    }
    if (!folio_batch_add(fbatch, folio)) {
// start = folio_next_index(folio);
// goto;
    }
    }
//
// We come here when there is no folio beyond @end. We take care to not
// overflow the index @start as it confuses some of the callers. This
// breaks the iteration when there is a folio at index -1 but that is
// already broke anyway.
//
    if (end == (pgoff_t)-1) {
// start = (pgoff_t)-1;
    }
    else {
// start = end + 1;
    }
// label;
    rcu_read_unlock();
    return folio_batch_count(fbatch);
    }
//
// CD/DVDs are error prone. When a medium error occurs, the driver may fail
// a _large_ part of the i/o request. Imagine the worst scenario:
//
// ---R__________________________________________B__________
// ^ reading here                             ^ bad block(assume 4k)
//
// read(R) => miss => readahead(R...B) => media error => frustrating retries
// => failing the whole request => read(R) => read(R+1) =>
// readahead(R+1...B+1) => bang => read(R+2) => read(R+3) =>
// readahead(R+3...B+2) => bang => read(R+3) => read(R+4) =>
// readahead(R+4...B+3) => bang => read(R+4) => read(R+5) => ......
//
// It is going insane. Fix it by quickly scaling down the readahead size.
//
#[no_mangle]
unsafe extern "C" fn shrink_readahead_size_eio(ra: *mut file_ra_state) {
    ra.ra_pages /= 4;
    }
//
// filemap_get_read_batch - Get a batch of folios for read
//
// Get a batch of folios which represent a contiguous range of bytes in
// the file.  No exceptional entries will be returned.  If @index is in
// the middle of a folio, the entire folio will be returned.  The last
// folio in the batch may have the readahead flag set or the uptodate flag
// clear so that the caller can take the appropriate action.
//
#[no_mangle]
pub unsafe extern "C" fn filemap_get_read_batch(mapping: *mut address_space, index: pgoff_t, max: pgoff_t, fbatch: *mut folio_batch) {
    XA_STATE(xas, &mapping.i_pages, index);
pub static mut folio: *mut c_void = core::ptr::null_mut();
    if (index > max) {
    return;
    }
    rcu_read_lock();
    for (folio = xas_load(&xas); folio; folio = xas_next(&xas)) {
    if (xas_retry(&xas, folio)) {
    continue;
    }
    if (xa_is_value(folio)) {
    break;
    }
    if (xa_is_sibling(folio)) {
    break;
    }
    if (!folio_try_get(folio)) {
// goto;
    }
    if (unlikely(folio != xas_reload(&xas))) {
// goto;
    }
    if (!folio_batch_add(fbatch, folio)) {
    break;
    }
    if (!folio_test_uptodate(folio)) {
    break;
    }
    if (folio_test_readahead(folio)) {
    break;
    }
    xas_advance(&xas, folio_next_index(folio) - 1);
    if (xas.xa_index >= max) {
    break;
    }
    continue;
// label;
    folio_put(folio);
// label;
    xas_reset(&xas);
    }
    rcu_read_unlock();
    }
#[no_mangle]
pub unsafe extern "C" fn filemap_read_folio(file: *mut file, filler: filler_t, folio: *mut folio) -> c_int {
pub static mut workingset: bool = false;
    let mut pflags = 0;
    let mut error = 0;
// Start the actual read. The read will unlock the page.
    if (unlikely(workingset)) {
    psi_memstall_enter(&pflags);
    }
    error = filler(file, folio);
    if (unlikely(workingset)) {
    psi_memstall_leave(&pflags);
    }
    if (error) {
    return error;
    }
    error = folio_wait_locked_killable(folio);
    if (error) {
    return error;
    }
    if (folio_test_uptodate(folio)) {
    return 0;
    }
    if (file) {
    shrink_readahead_size_eio(&file.f_ra);
    }
    return -EIO;
    }
#[no_mangle]
pub unsafe extern "C" fn filemap_range_uptodate(mapping: *mut address_space, pos: loff_t, count: size_t, folio: *mut folio, need_uptodate: bool) -> bool {
    if (folio_test_uptodate(folio)) {
    return true;
    }
// pipes can't handle partially uptodate pages
    if (need_uptodate) {
    return false;
    }
    if (!mapping.a_ops.is_partially_uptodate) {
    return false;
    }
    if (mapping.host.i_blkbits >= folio_shift(folio)) {
    return false;
    }
    if (folio_pos(folio) > pos) {
    count -= folio_pos(folio) - pos;
    pos = 0;
    } else {
    pos -= folio_pos(folio);
    }
    if (pos == 0 && count >= folio_size(folio)) {
    return false;
    }
    return mapping.a_ops.is_partially_uptodate(folio, pos, count);
    }
#[no_mangle]
pub unsafe extern "C" fn filemap_update_page(iocb: *mut kiocb, mapping: *mut address_space, count: size_t, folio: *mut folio, need_uptodate: bool) -> c_int {
    let mut error = 0;
    if (iocb.ki_flags & IOCB_NOWAIT) {
    if (!filemap_invalidate_trylock_shared(mapping)) {
    return -EAGAIN;
    }
    } else {
    filemap_invalidate_lock_shared(mapping);
    }
    if (!folio_trylock(folio)) {
    error = -EAGAIN;
    if (iocb.ki_flags & (IOCB_NOWAIT | IOCB_NOIO)) {
// goto;
    }
    if (!(iocb.ki_flags & IOCB_WAITQ)) {
    filemap_invalidate_unlock_shared(mapping);
//
// This is where we usually end up waiting for a
// previously submitted readahead to finish.
//
    folio_put_wait_locked(folio, TASK_KILLABLE);
    return AOP_TRUNCATED_PAGE;
    }
    error = __folio_lock_async(folio, iocb.ki_waitq);
    if (error) {
// goto;
    }
    }
    error = AOP_TRUNCATED_PAGE;
    if (!folio.mapping) {
// goto;
    }
    error = 0;
    if (filemap_range_uptodate(mapping, iocb.ki_pos, count, folio,
    need_uptodate)) {
// goto;
    }
    error = -EAGAIN;
    if (iocb.ki_flags & (IOCB_NOIO | IOCB_NOWAIT | IOCB_WAITQ)) {
// goto;
    }
    error = filemap_read_folio(iocb.ki_filp, mapping.a_ops.read_folio,
    folio);
// goto;
// label;
    folio_unlock(folio);
// label;
    filemap_invalidate_unlock_shared(mapping);
    if (error == AOP_TRUNCATED_PAGE) {
    folio_put(folio);
    }
    return error;
    }
#[no_mangle]
unsafe extern "C" fn filemap_create_folio(iocb: *mut kiocb, fbatch: *mut folio_batch) -> c_int {
    let mut mapping = iocb.ki_filp.f_mapping;
pub static mut folio: *mut c_void = core::ptr::null_mut();
    let mut error = 0;
pub static mut min_order: c_uint = 0;
    let mut index;
    if (iocb.ki_flags & (IOCB_NOWAIT | IOCB_WAITQ)) {
    return -EAGAIN;
    }
    folio = filemap_alloc_folio(mapping_gfp_mask(mapping), min_order, core::ptr::null_mut());
    if (!folio) {
    return -ENOMEM;
    }
    if (iocb.ki_flags & IOCB_DONTCACHE) {
    __folio_set_dropbehind(folio);
    }
//
// Protect against truncate / hole punch. Grabbing invalidate_lock
// here assures we cannot instantiate and bring uptodate new
// pagecache folios after evicting page cache during truncate
// and before actually freeing blocks.	Note that we could
// release invalidate_lock after inserting the folio into
// the page cache as the locked folio would then be enough to
// synchronize with hole punching. But there are code paths
// such as filemap_update_page() filling in partially uptodate
// pages or ->readahead() that need to hold invalidate_lock
// while mapping blocks for IO so let's hold the lock here as
// well to keep locking rules simple.
//
    filemap_invalidate_lock_shared(mapping);
    index = (iocb.ki_pos >> (PAGE_SHIFT + min_order)) << min_order;
    error = filemap_add_folio(mapping, folio, index,
    mapping_gfp_constraint(mapping, GFP_KERNEL));
    if (error == -EEXIST) {
    error = AOP_TRUNCATED_PAGE;
    }
    if (error) {
// goto;
    }
    error = filemap_read_folio(iocb.ki_filp, mapping.a_ops.read_folio,
    folio);
    if (error) {
// goto;
    }
    filemap_invalidate_unlock_shared(mapping);
    folio_batch_add(fbatch, folio);
    return 0;
// label;
    filemap_invalidate_unlock_shared(mapping);
    folio_put(folio);
    return error;
    }
#[no_mangle]
pub unsafe extern "C" fn filemap_readahead(iocb: *mut kiocb, file: *mut file, mapping: *mut address_space, folio: *mut folio, last_index: pgoff_t) -> c_int {
pub static mut ractl: usize = 0;
    if (iocb.ki_flags & IOCB_NOIO) {
    return -EAGAIN;
    }
    if (iocb.ki_flags & IOCB_DONTCACHE) {
    ractl.dropbehind = 1;
    }
    page_cache_async_ra(&ractl, folio, last_index - folio.index);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn filemap_get_pages(iocb: *mut kiocb, count: size_t, fbatch: *mut folio_batch, need_uptodate: bool) -> c_int {
    let mut filp = iocb.ki_filp;
    let mut mapping = filp.f_mapping;
pub static mut index: pgoff_t = 0;
    let mut last_index;
pub static mut folio: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
pub static mut err: c_int = 0;
// "last_index" is the index of the folio beyond the end of the read
    last_index = round_up(iocb.ki_pos + count,
    mapping_min_folio_nrbytes(mapping)) >> PAGE_SHIFT;
// label;
    if (fatal_signal_pending(current)) {
    return -EINTR;
    }
    filemap_get_read_batch(mapping, index, last_index - 1, fbatch);
    if (!folio_batch_count(fbatch)) {
pub static mut ractl: usize = 0;
    if (iocb.ki_flags & IOCB_NOIO) {
    return -EAGAIN;
    }
    if (iocb.ki_flags & IOCB_NOWAIT) {
    flags = memalloc_noio_save();
    }
    if (iocb.ki_flags & IOCB_DONTCACHE) {
    ractl.dropbehind = 1;
    }
    page_cache_sync_ra(&ractl, last_index - index);
    if (iocb.ki_flags & IOCB_NOWAIT) {
    memalloc_noio_restore(flags);
    }
    filemap_get_read_batch(mapping, index, last_index - 1, fbatch);
    }
    if (!folio_batch_count(fbatch)) {
    err = filemap_create_folio(iocb, fbatch);
    if (err == AOP_TRUNCATED_PAGE) {
// goto;
    }
    return err;
    }
    folio = fbatch.folios[folio_batch_count(fbatch) - 1];
    if (folio_test_readahead(folio)) {
    err = filemap_readahead(iocb, filp, mapping, folio, last_index);
    if (err) {
// goto;
    }
    }
    if (!folio_test_uptodate(folio)) {
    if (folio_batch_count(fbatch) > 1) {
    err = -EAGAIN;
// goto;
    }
    err = filemap_update_page(iocb, mapping, count, folio,
    need_uptodate);
    if (err) {
// goto;
    }
    }
    trace_mm_filemap_get_pages(mapping, index, last_index - 1);
    return 0;
// label;
    if (err < 0) {
    folio_put(folio);
    }
    if (likely(--fbatch.nr)) {
    return 0;
    }
    if (err == AOP_TRUNCATED_PAGE) {
// goto;
    }
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn pos_same_folio(pos1: loff_t, pos2: loff_t, folio: *mut folio) -> bool {
pub static mut shift: c_uint = 0;
    return (pos1 >> shift == pos2 >> shift);
    }
#[no_mangle]
unsafe extern "C" fn filemap_end_dropbehind_read(folio: *mut folio) {
    if (!folio_test_dropbehind(folio)) {
    return;
    }
    if (folio_test_writeback(folio) || folio_test_dirty(folio)) {
    return;
    }
    if (folio_trylock(folio)) {
    filemap_end_dropbehind(folio);
    folio_unlock(folio);
    }
    }
//
// filemap_read - Read data from the page cache.
// @iocb: The iocb to read.
// @iter: Destination for the data.
// @already_read: Number of bytes already read by the caller.
//
// Copies data from the page cache.  If the data is not currently present,
// uses the readahead and read_folio address_space operations to fetch it.
//
// Return: Total number of bytes copied, including those already read by
// the caller.  If an error happens before any bytes are copied, returns
// a negative error number.
//
#[no_mangle]
pub unsafe extern "C" fn filemap_read(iocb: *mut kiocb, iter: *mut iov_iter, already_read: ssize_t) -> ssize_t {
    let mut filp = iocb.ki_filp;
    let mut ra = &filp.f_ra;
    let mut mapping = filp.f_mapping;
    let mut inode = mapping.host;
pub static mut fbatch: usize = 0;
    int i, error = 0;
    let mut writably_mapped = 0;
    loff_t isize, end_offset;
pub static mut last_pos: loff_t = 0;
    if (unlikely(iocb.ki_pos < 0)) {
    return -EINVAL;
    }
    if (unlikely(iocb.ki_pos >= inode.i_sb.s_maxbytes)) {
    return 0;
    }
    if (unlikely(!iov_iter_count(iter))) {
    return 0;
    }
    iov_iter_truncate(iter, inode.i_sb.s_maxbytes - iocb.ki_pos);
    folio_batch_init(&fbatch);
    do {
    cond_resched();
//
// If we've already successfully copied some data, then we
// can no longer safely return -EIOCBQUEUED. Hence mark
// an async read NOWAIT at that point.
//
    if ((iocb.ki_flags & IOCB_WAITQ) && already_read) {
    iocb.ki_flags |= IOCB_NOWAIT;
    }
    if (unlikely(iocb.ki_pos >= i_size_read(inode))) {
    break;
    }
    error = filemap_get_pages(iocb, iter.count, &fbatch, false);
    if (error < 0) {
    break;
    }
//
// i_size must be checked after we know the pages are Uptodate.
//
// Checking i_size after the check allows us to calculate
// the correct value for "nr", which means the zero-filled
// part of the page is not copied back to userspace (unless
// another truncate extends the file - this is desired though).
//
    isize = i_size_read(inode);
    if (unlikely(iocb.ki_pos >= isize)) {
// goto;
    }
    end_offset = min_t(loff_t, isize, iocb.ki_pos + iter.count);
//
// Once we start copying data, we don't want to be touching any
// cachelines that might be contended:
//
    writably_mapped = mapping_writably_mapped(mapping);
//
// When a read accesses the same folio several times, only
// mark it as accessed the first time.
//
    if (!pos_same_folio(iocb.ki_pos, last_pos - 1,
    fbatch.folios[0])) {
    folio_mark_accessed(fbatch.folios[0]);
    }
    while (i < folio_batch_count(&fbatch)) {
    let mut folio = fbatch.folios[i];
pub static mut fsize: usize = 0;
pub static mut offset: usize = 0;
    size_t bytes = min_t(loff_t, end_offset - iocb.ki_pos,
    fsize - offset);
    let mut copied = 0;
    if (end_offset < folio_pos(folio)) {
    break;
    }
    if (i > 0) {
    folio_mark_accessed(folio);
    }
//
// If users can be writing to this folio using arbitrary
// virtual addresses, take care of potential aliasing
// before reading the folio on the kernel side.
//
    if (writably_mapped) {
    flush_dcache_folio(folio);
    }
    copied = copy_folio_to_iter(folio, offset, bytes, iter);
    already_read += copied;
    iocb.ki_pos += copied;
    last_pos = iocb.ki_pos;
    if (copied < bytes) {
    error = -EFAULT;
    break;
    }
    }
// label;
    while (i < folio_batch_count(&fbatch)) {
    let mut folio = fbatch.folios[i];
    filemap_end_dropbehind_read(folio);
    folio_put(folio);
    }
    folio_batch_init(&fbatch);
    } while (iov_iter_count(iter) && iocb.ki_pos < isize && !error);
    file_accessed(filp);
    ra.prev_pos = last_pos;
    return already_read ? already_read : error;
    }
    EXPORT_SYMBOL_GPL(filemap_read);
#[no_mangle]
pub unsafe extern "C" fn kiocb_write_and_wait(iocb: *mut kiocb, count: usize) -> c_int {
    let mut mapping = iocb.ki_filp.f_mapping;
pub static mut pos: loff_t = 0;
pub static mut end: loff_t = 0;
    if (iocb.ki_flags & IOCB_NOWAIT) {
    if (filemap_range_needs_writeback(mapping, pos, end)) {
    return -EAGAIN;
    }
    return 0;
    }
    return filemap_write_and_wait_range(mapping, pos, end);
    }
    EXPORT_SYMBOL_GPL(kiocb_write_and_wait);
#[no_mangle]
pub unsafe extern "C" fn filemap_invalidate_pages(mapping: *mut address_space, pos: loff_t, end: loff_t, nowait: bool) -> c_int {
    let mut ret = 0;
    if (nowait) {
// we could block if there are any pages in the range
    if (filemap_range_has_page(mapping, pos, end)) {
    return -EAGAIN;
    }
    } else {
    ret = filemap_write_and_wait_range(mapping, pos, end);
    if (ret) {
    return ret;
    }
    }
//
// After a write we want buffered reads to be sure to go to disk to get
// the new data.  We invalidate clean cached page from the region we're
// about to write.  We do this *before* the write so that we can return
// without clobbering -EIOCBQUEUED from ->direct_IO().
//
    return invalidate_inode_pages2_range(mapping, pos >> PAGE_SHIFT,
    end >> PAGE_SHIFT);
    }
#[no_mangle]
pub unsafe extern "C" fn kiocb_invalidate_pages(iocb: *mut kiocb, count: usize) -> c_int {
    let mut mapping = iocb.ki_filp.f_mapping;
    return filemap_invalidate_pages(mapping, iocb.ki_pos,
    iocb.ki_pos + count - 1,
    iocb.ki_flags & IOCB_NOWAIT);
    }
    EXPORT_SYMBOL_GPL(kiocb_invalidate_pages);
//
// generic_file_read_iter - generic filesystem read routine
// @iocb:	kernel I/O control block
// @iter:	destination for the data read
//
// This is the "read_iter()" routine for all filesystems
// that can use the page cache directly.
//
// The IOCB_NOWAIT flag in iocb->ki_flags indicates that -EAGAIN shall
// be returned when no data can be read without waiting for I/O requests
// to complete; it doesn't prevent readahead.
//
// The IOCB_NOIO flag in iocb->ki_flags indicates that no new I/O
// requests shall be made for the read or for readahead.  When no data
// can be read, -EAGAIN shall be returned.  When readahead would be
// triggered, a partial, possibly empty read shall be returned.
//
// Return:
// * number of bytes copied, even for partial reads
// * negative error code (or 0 if IOCB_NOIO) if nothing was read
//
#[no_mangle]
pub unsafe extern "C" fn generic_file_read_iter(iocb: *mut kiocb, iter: *mut iov_iter) -> ssize_t {
pub static mut count: usize = 0;
pub static mut retval: isize = 0;
    if (!count) {
    return 0; /* skip atime */
    }
    if (iocb.ki_flags & IOCB_DIRECT) {
    let mut file = iocb.ki_filp;
    let mut mapping = file.f_mapping;
    let mut inode = mapping.host;
    retval = kiocb_write_and_wait(iocb, count);
    if (retval < 0) {
    return retval;
    }
    file_accessed(file);
    retval = mapping.a_ops.direct_IO(iocb, iter);
    if (retval >= 0) {
    iocb.ki_pos += retval;
    count -= retval;
    }
    if (retval != -EIOCBQUEUED) {
    iov_iter_revert(iter, count - iov_iter_count(iter));
    }
//
// Btrfs can have a short DIO read if we encounter
// compressed extents, so if there was an error, or if
// we've already read everything we wanted to, or if
// there was a short read because we hit EOF, go ahead
// and return.  Otherwise fallthrough to buffered io for
// the rest of the read.  Buffered reads will not work for
// DAX files, so don't bother trying.
//
    if (retval < 0 || !count || IS_DAX(inode)) {
    return retval;
    }
    if (iocb.ki_pos >= i_size_read(inode)) {
    return retval;
    }
    }
    return filemap_read(iocb, iter, retval);
    }
    EXPORT_SYMBOL(generic_file_read_iter);
//
// Splice subpages from a folio into a pipe.
//
#[no_mangle]
pub unsafe extern "C" fn splice_folio_into_pipe(pipe: *mut pipe_inode_info, folio: *mut folio, fpos: loff_t, size: size_t) -> size_t {
pub static mut page: *mut c_void = core::ptr::null_mut();
pub static mut spliced: usize = 0;
    page = folio_page(folio, offset / PAGE_SIZE);
    size = min(size, folio_size(folio) - offset);
    offset %= PAGE_SIZE;
    while (spliced < size && !pipe_is_full(pipe)) {
    let mut buf = pipe_head_buf(pipe);
pub static mut part: usize = 0;
// buf = (pipe_buffer) {
    .ops	= &page_cache_pipe_buf_ops,
    .page	= page,
    .offset	= offset,
    .len	= part,
    };
    folio_get(folio);
    pipe.head += 1;
    page += 1;
    spliced += part;
    offset = 0;
    }
    return spliced;
    }
//
// filemap_splice_read -  Splice data from a file's pagecache into a pipe
// @in: The file to read from
// @ppos: Pointer to the file position to read from
// @pipe: The pipe to splice into
// @len: The amount to splice
// @flags: The SPLICE_F_* flags
//
// This function gets folios from a file's pagecache and splices them into the
// pipe.  Readahead will be called as necessary to fill more folios.  This may
// be used for blockdevs also.
//
// Return: On success, the number of bytes read will be returned and *@ppos
// will be updated if appropriate; 0 will be returned if there is no more data
// to be read; -EAGAIN will be returned if the pipe had no space, and some
// other negative error code will be returned on error.  A short read may occur
// if the pipe has insufficient space, we reach the end of the data or we hit a
// hole.
//
#[no_mangle]
pub unsafe extern "C" fn filemap_splice_read(in: *mut file, ppos: *mut loff_t, pipe: *mut pipe_inode_info, len: size_t, flags: c_uint) -> ssize_t {
pub static mut fbatch: usize = 0;
pub static mut iocb: usize = 0;
pub static mut total_spliced: usize = 0;
    loff_t isize, end_offset;
    let mut writably_mapped = 0;
    int i, error = 0;
    if (unlikely(*ppos >= in.f_mapping.host.i_sb.s_maxbytes)) {
    return 0;
    }
    init_sync_kiocb(&iocb, in);
    iocb.ki_pos = *ppos;
// Work out how much data we can actually add into the pipe
    used = pipe_buf_usage(pipe);
    npages = max_t(ssize_t, pipe.max_usage - used, 0);
    len = min_t(size_t, len, npages * PAGE_SIZE);
    folio_batch_init(&fbatch);
    do {
    cond_resched();
    if (*ppos >= i_size_read(in.f_mapping.host)) {
    break;
    }
    iocb.ki_pos = *ppos;
    error = filemap_get_pages(&iocb, len, &fbatch, true);
    if (error < 0) {
    break;
    }
//
// i_size must be checked after we know the pages are Uptodate.
//
// Checking i_size after the check allows us to calculate
// the correct value for "nr", which means the zero-filled
// part of the page is not copied back to userspace (unless
// another truncate extends the file - this is desired though).
//
    isize = i_size_read(in.f_mapping.host);
    if (unlikely(*ppos >= isize)) {
    break;
    }
    end_offset = min_t(loff_t, isize, *ppos + len);
//
// Once we start copying data, we don't want to be touching any
// cachelines that might be contended:
//
    writably_mapped = mapping_writably_mapped(in.f_mapping);
    while (i < folio_batch_count(&fbatch)) {
    let mut folio = fbatch.folios[i];
    let mut n = 0;
    if (folio_pos(folio) >= end_offset) {
// goto;
    }
    folio_mark_accessed(folio);
//
// If users can be writing to this folio using arbitrary
// virtual addresses, take care of potential aliasing
// before reading the folio on the kernel side.
//
    if (writably_mapped) {
    flush_dcache_folio(folio);
    }
    n = min_t(loff_t, len, isize - *ppos);
    n = splice_folio_into_pipe(pipe, folio, *ppos, n);
    if (!n) {
// goto;
    }
    len -= n;
    total_spliced += n;
// ppos += n;
    in.f_ra.prev_pos = *ppos;
    if (pipe_is_full(pipe)) {
// goto;
    }
    }
    folio_batch_release(&fbatch);
    } while (len);
// label;
    folio_batch_release(&fbatch);
    file_accessed(in);
    return total_spliced ? total_spliced : error;
    }
    EXPORT_SYMBOL(filemap_splice_read);
#[no_mangle]
pub unsafe extern "C" fn folio_seek_hole_data(xas: *mut xa_state, mapping: *mut address_space, folio: *mut folio, start: loff_t, end: loff_t, seek_data: bool) -> loff_t {
    let mut ops = mapping.a_ops;
    size_t offset, bsz = i_blocksize(mapping.host);
    if (xa_is_value(folio) || folio_test_uptodate(folio)) {
    return seek_data ? start : end;
    }
    if (!ops.is_partially_uptodate) {
    return seek_data ? end : start;
    }
    xas_pause(xas);
    rcu_read_unlock();
    folio_lock(folio);
    if (unlikely(folio.mapping != mapping)) {
// goto;
    }
    offset = offset_in_folio(folio, start) & ~(bsz - 1);
    do {
    if (ops.is_partially_uptodate(folio, offset, bsz) ==
    seek_data) {
    break;
    }
    start = (start + bsz) & ~((u64)bsz - 1);
    offset += bsz;
    } while (offset < folio_size(folio));
// label;
    folio_unlock(folio);
    rcu_read_lock();
    return start;
    }
#[no_mangle]
pub unsafe extern "C" fn seek_folio_size(xas: *mut xa_state, folio: *mut folio) -> usize {
    if (xa_is_value(folio)) {
    return PAGE_SIZE << xas_get_order(xas);
    }
    return folio_size(folio);
    }
//
// mapping_seek_hole_data - Seek for SEEK_DATA / SEEK_HOLE in the page cache.
// @mapping: Address space to search.
// @start: First byte to consider.
// @end: Limit of search (exclusive).
// @whence: Either SEEK_HOLE or SEEK_DATA.
//
// If the page cache knows which blocks contain holes and which blocks
// contain data, your filesystem can use this function to implement
// SEEK_HOLE and SEEK_DATA.  This is useful for filesystems which are
// entirely memory-based such as tmpfs, and filesystems which support
// unwritten extents.
//
// Return: The requested offset on success, or -ENXIO if @whence specifies
// SEEK_DATA and there is no data after @start.  There is an implicit hole
// after @end - 1, so SEEK_HOLE returns @end if all the bytes between @start
// and @end contain data.
//
#[no_mangle]
pub unsafe extern "C" fn mapping_seek_hole_data(mapping: *mut address_space, start: loff_t, end: loff_t, whence: c_int) -> loff_t {
    XA_STATE(xas, &mapping.i_pages, start >> PAGE_SHIFT);
pub static mut max: pgoff_t = 0;
pub static mut seek_data: bool = false;
pub static mut folio: *mut c_void = core::ptr::null_mut();
    if (end <= start) {
    return -ENXIO;
    }
    rcu_read_lock();
    while ((folio = find_get_entry(&xas, max, XA_PRESENT))) {
pub static mut pos: loff_t = 0;
    let mut seek_size = 0;
    let mut next = 0;
    if (start < pos) {
    if (!seek_data) {
// goto;
    }
    start = pos;
    }
    seek_size = seek_folio_size(&xas, folio);
    next = round_up((u64)pos + 1, seek_size);
    if (next > (u64)end) {
    pos = end;
    }
    else {
    pos = next;
    }
    start = folio_seek_hole_data(&xas, mapping, folio, start, pos,
    seek_data);
    if (start < pos) {
// goto;
    }
    if (start >= end) {
    break;
    }
    if (seek_size > PAGE_SIZE) {
    xas_set(&xas, pos >> PAGE_SHIFT);
    }
    if (!xa_is_value(folio)) {
    folio_put(folio);
    }
    }
    if (seek_data) {
    start = -ENXIO;
    }
// label;
    rcu_read_unlock();
    if (folio && !xa_is_value(folio)) {
    folio_put(folio);
    }
    if (start > end) {
    return end;
    }
    return start;
    }

//
// lock_folio_maybe_drop_mmap - lock the page, possibly dropping the mmap_lock
// @vmf - the vm_fault for this fault.
// @folio - the folio to lock.
// @fpin - the pointer to the file we may pin (or is already pinned).
//
// This works similar to lock_folio_or_retry in that it can drop the
// mmap_lock.  It differs in that it actually returns the folio locked
// if it returns 1 and 0 if it couldn't lock the folio.  If we did have
// to drop the mmap_lock then fpin will point to the pinned file and
// needs to be fput()'ed at a later point.
//
#[no_mangle]
pub unsafe extern "C" fn lock_folio_maybe_drop_mmap(vmf: *mut vm_fault, folio: *mut folio, fpin: *mut *mut file) -> c_int {
    if (folio_trylock(folio)) {
    return 1;
    }
//
// NOTE! This will make us return with VM_FAULT_RETRY, but with
// the fault lock still held. That's how FAULT_FLAG_RETRY_NOWAIT
// is supposed to work. We have way too many special cases..
//
    if (vmf.flags & FAULT_FLAG_RETRY_NOWAIT) {
    return 0;
    }
// fpin = maybe_unlock_mmap_for_io(vmf, *fpin);
    if (vmf.flags & FAULT_FLAG_KILLABLE) {
    if (__folio_lock_killable(folio)) {
//
// We didn't have the right flags to drop the
// fault lock, but all fault_handlers only check
// for fatal signals if we return VM_FAULT_RETRY,
// so we need to drop the fault lock here and
// return 0 if we don't have a fpin.
//
    if (*fpin == core::ptr::null_mut()) {
    release_fault_lock(vmf);
    }
    return 0;
    }
    } else {
    __folio_lock(folio);
    }
    return 1;
    }
//
// Synchronous readahead happens when we don't even find a page in the page
// cache at all.  We don't want to perform IO under the mmap sem, so if we have
// to drop the mmap sem we return the file that was pinned in order for us to do
// that.  If we didn't pin a file then we return NULL.  The file that is
// returned needs to be fput()'ed when we're done with it.
//
#[no_mangle]
pub unsafe extern "C" fn do_sync_mmap_readahead(vmf: *mut vm_fault) -> *mut c_void {
    let mut file = vmf.vma.vm_file;
    let mut ra = &file.f_ra;
    let mut mapping = file.f_mapping;
pub static mut ractl: usize = 0;
    let mut fpin = core::ptr::null_mut();
pub static mut vm_flags: vm_flags_t = 0;
pub static mut force_thp_readahead: bool = false;
pub static mut thp_order: c_uint = 0;
    let mut mmap_miss = 0;
// Use the readahead code, even if readahead is disabled
    if (IS_ENABLED!(CONFIG_TRANSPARENT_HUGEPAGE) && (vm_flags & VM_HUGEPAGE)) {
//
// Cap max THP order at 2MB: this is the common PMD-sized
// hugepage size, and it avoids memory pressure from very
// large forced readahead when mapping_max_folio_order() is
// high (for example, 128MB with 64K base pages on arm64).
//
    if (mapping_large_folio_support(mapping)) {
    force_thp_readahead = true;
    thp_order = min_t(unsigned int,
    mapping_max_folio_order(mapping),
    get_order(SZ_2M));
    }
    }
    if (!force_thp_readahead) {
//
// If we don't want any read-ahead, don't bother.
// VM_EXEC case below is already intended for random access.
//
    if ((vm_flags & (VM_RAND_READ | VM_EXEC)) == VM_RAND_READ) {
    return fpin;
    }
    if (!ra.ra_pages) {
    return fpin;
    }
    if (vm_flags & VM_SEQ_READ) {
    fpin = maybe_unlock_mmap_for_io(vmf, fpin);
    page_cache_sync_ra(&ractl, ra.ra_pages);
    return fpin;
    }
    }
    if (!(vm_flags & (VM_SEQ_READ | VM_EXEC))) {
// Avoid banging the cache line if not needed
    mmap_miss = READ_ONCE(ra.mmap_miss);
    if (mmap_miss < MMAP_LOTSAMISS * 10) {
    WRITE_ONCE(ra.mmap_miss, ++mmap_miss);
    }
//
// Do we miss much more than hit in this file? If so,
// stop bothering with read-ahead. It will only hurt.
//
    if (mmap_miss > MMAP_LOTSAMISS) {
    return fpin;
    }
    }
    if (force_thp_readahead) {
pub static mut folio_nr_pages: c_ulong = 0;
    fpin = maybe_unlock_mmap_for_io(vmf, fpin);
    ractl._index &= ~(folio_nr_pages - 1);
    ra.size = folio_nr_pages;
//
// Fetch two folios so we get the chance to actually
// readahead, unless we've been told not to.
//
    if (!(vm_flags & VM_RAND_READ)) {
    ra.size *= 2;
    }
    ra.async_size = folio_nr_pages;
    ra.order = thp_order;
    page_cache_ra_order(&ractl, ra);
    return fpin;
    }
    if (vm_flags & VM_EXEC) {
//
// Allow arch to request a preferred minimum folio order for
// executable memory. This can often be beneficial to
// performance if (e.g.) arm64 can contpte-map the folio.
// Executable memory rarely benefits from readahead, due to its
// random access nature, so set async_size to 0.
//
// Limit to the boundaries of the VMA to avoid reading in any
// pad that might exist between sections, which would be a waste
// of memory.
//
    let mut vma = vmf.vma;
pub static mut start: c_ulong = 0;
pub static mut end: c_ulong = 0;
    let mut ra_end = 0;
    ra.order = exec_folio_order();
    ra.start = round_down(vmf.pgoff, 1UL << ra.order);
    ra.start = max(ra.start, start);
    ra_end = round_up(ra.start + ra.ra_pages, 1UL << ra.order);
    ra_end = min(ra_end, end);
    ra.size = ra_end - ra.start;
    ra.async_size = 0;
    } else {
//
// mmap read-around
//
    ra.start = max_t(long, 0, vmf.pgoff - ra.ra_pages / 2);
    ra.size = ra.ra_pages;
    ra.async_size = ra.ra_pages / 4;
    ra.order = 0;
    }
    fpin = maybe_unlock_mmap_for_io(vmf, fpin);
    ractl._index = ra.start;
    page_cache_ra_order(&ractl, ra);
    return fpin;
    }
//
// Asynchronous readahead happens when we find the page and PG_readahead,
// so we want to possibly extend the readahead further.  We return the file that
// was pinned if we have to drop the mmap_lock in order to do IO.
//
#[no_mangle]
pub unsafe extern "C" fn do_async_mmap_readahead(vmf: *mut vm_fault, folio: *mut folio) -> *mut c_void {
    let mut file = vmf.vma.vm_file;
    let mut ra = &file.f_ra;
pub static mut ractl: usize = 0;
    let mut fpin = core::ptr::null_mut();
    let mut mmap_miss = 0;
// If we don't want any read-ahead, don't bother
    if (vmf.vma.vm_flags & VM_RAND_READ || !ra.ra_pages) {
    return fpin;
    }
//
// If the folio is locked, we're likely racing against another fault.
// Don't touch the mmap_miss counter to avoid decreasing it multiple
// times for a single folio and break the balance with mmap_miss
// increase in do_sync_mmap_readahead().
//
// VM_SEQ_READ and VM_EXEC mappings skip the mmap_miss increment in
// do_sync_mmap_readahead(), so skip the decrement here as well to
// keep the counter symmetric.
//
    if (likely(!folio_test_locked(folio)) &&
    !(vmf.vma.vm_flags & (VM_SEQ_READ | VM_EXEC))) {
    mmap_miss = READ_ONCE(ra.mmap_miss);
    if (mmap_miss) {
    WRITE_ONCE(ra.mmap_miss, --mmap_miss);
    }
    }
    if (folio_test_readahead(folio)) {
    fpin = maybe_unlock_mmap_for_io(vmf, fpin);
    page_cache_async_ra(&ractl, folio, ra.ra_pages);
    }
    return fpin;
    }
#[no_mangle]
unsafe extern "C" fn filemap_fault_recheck_pte_none(vmf: *mut vm_fault) -> vm_fault_t {
    let mut vma = vmf.vma;
pub static mut ret: vm_fault_t = 0;
pub static mut ptep: *mut c_void = core::ptr::null_mut();
//
// We might have COW'ed a pagecache folio and might now have an mlocked
// anon folio mapped. The original pagecache folio is not mlocked and
// might have been evicted. During a read+clear/modify/write update of
// the PTE, such as done in do_numa_page()/change_pte_range(), we
// temporarily clear the PTE under PT lock and might detect it here as
// "none" when not holding the PT lock.
//
// Not rechecking the PTE under PT lock could result in an unexpected
// major fault in an mlock'ed region. Recheck only for this special
// scenario while holding the PT lock, to not degrade non-mlocked
// scenarios. Recheck the PTE without PT lock firstly, thereby reducing
// the number of times we hold PT lock.
//
    if (!(vma.vm_flags & VM_LOCKED)) {
    return 0;
    }
    if (!(vmf.flags & FAULT_FLAG_ORIG_PTE_VALID)) {
    return 0;
    }
    ptep = pte_offset_map_ro_nolock(vma.vm_mm, vmf.pmd, vmf.address,
    &vmf.ptl);
    if (unlikely(!ptep)) {
    return VM_FAULT_NOPAGE;
    }
    if (unlikely(!pte_none(ptep_get_lockless(ptep)))) {
    ret = VM_FAULT_NOPAGE;
    } else {
    spin_lock(vmf.ptl);
    if (unlikely(!pte_none(ptep_get(ptep)))) {
    ret = VM_FAULT_NOPAGE;
    }
    spin_unlock(vmf.ptl);
    }
    pte_unmap(ptep);
    return ret;
    }
//
// filemap_fault - read in file data for page fault handling
// @vmf: vm_fault containing details of the fault
//
// filemap_fault() is invoked via the vma operations vector for a
// mapped memory region to read in file data during a page fault.
//
// The goto's are kind of ugly, but this streamlines the normal case of having
// it in the page cache, and handles the special cases reasonably without
// having a lot of duplicated code.
//
// vma->vm_mm->mmap_lock must be held on entry.
//
// If our return value has VM_FAULT_RETRY set, it's because the mmap_lock
// may be dropped before doing I/O or by lock_folio_maybe_drop_mmap().
//
// If our return value does not have VM_FAULT_RETRY set, the mmap_lock
// has not been released.
//
// We never return with VM_FAULT_RETRY and a bit from VM_FAULT_ERROR set.
//
// Return: bitwise-OR of %VM_FAULT_ codes.
//
#[no_mangle]
pub unsafe extern "C" fn filemap_fault(vmf: *mut vm_fault) -> vm_fault_t {
    let mut error = 0;
    let mut file = vmf.vma.vm_file;
    let mut fpin = core::ptr::null_mut();
    let mut mapping = file.f_mapping;
    let mut inode = mapping.host;
    pgoff_t max_idx, index = vmf.pgoff;
pub static mut folio: *mut c_void = core::ptr::null_mut();
pub static mut ret: vm_fault_t = 0;
pub static mut mapping_locked: bool = false;
    max_idx = DIV_ROUND_UP(i_size_read(inode), PAGE_SIZE);
    if (unlikely(index >= max_idx)) {
    return VM_FAULT_SIGBUS;
    }
    trace_mm_filemap_fault(mapping, index);
//
// Do we have something in the page cache already?
//
    folio = filemap_get_folio(mapping, index);
    if (likely(!IS_ERR(folio))) {
//
// We found the page, so try async readahead before waiting for
// the lock.
//
    if (!(vmf.flags & FAULT_FLAG_TRIED)) {
    fpin = do_async_mmap_readahead(vmf, folio);
    }
    if (unlikely(!folio_test_uptodate(folio))) {
    filemap_invalidate_lock_shared(mapping);
    mapping_locked = true;
    }
    } else {
    ret = filemap_fault_recheck_pte_none(vmf);
    if (unlikely(ret)) {
    return ret;
    }
// No page in the page cache at all
    count_vm_event(PGMAJFAULT);
    count_memcg_event_mm(vmf.vma.vm_mm, PGMAJFAULT);
    ret = VM_FAULT_MAJOR;
    fpin = do_sync_mmap_readahead(vmf);
// label;
//
// See comment in filemap_create_folio() why we need
// invalidate_lock
//
    if (!mapping_locked) {
    filemap_invalidate_lock_shared(mapping);
    mapping_locked = true;
    }
    folio = __filemap_get_folio(mapping, index,
    FGP_CREAT|FGP_FOR_MMAP,
    vmf.gfp_mask);
    if (IS_ERR(folio)) {
    if (fpin) {
// goto;
    }
    filemap_invalidate_unlock_shared(mapping);
    return VM_FAULT_OOM;
    }
    }
    if (!lock_folio_maybe_drop_mmap(vmf, folio, &fpin)) {
// goto;
    }
// Did it get truncated?
    if (unlikely(folio.mapping != mapping)) {
    folio_unlock(folio);
    folio_put(folio);
// goto;
    }
    VM_BUG_ON_FOLIO(!folio_contains(folio, index), folio);
//
// We have a locked folio in the page cache, now we need to check
// that it's up-to-date. If not, it is going to be due to an error,
// or because readahead was otherwise unable to retrieve it.
//
    if (unlikely(!folio_test_uptodate(folio))) {
//
// If the invalidate lock is not held, the folio was in cache
// and uptodate and now it is not. Strange but possible since we
// didn't hold the page lock all the time. Let's drop
// everything, get the invalidate lock and try again.
//
    if (!mapping_locked) {
    folio_unlock(folio);
    folio_put(folio);
// goto;
    }
//
// OK, the folio is really not uptodate. This can be because the
// VMA has the VM_RAND_READ flag set, or because an error
// arose. Let's read it in directly.
//
// goto;
    }
//
// We've made it this far and we had to drop our mmap_lock, now is the
// time to return to the upper layer and have it re-find the vma and
// redo the fault.
//
    if (fpin) {
    folio_unlock(folio);
// goto;
    }
    if (mapping_locked) {
    filemap_invalidate_unlock_shared(mapping);
    }
//
// Found the page and have a reference on it.
// We must recheck i_size under page lock.
//
    max_idx = DIV_ROUND_UP(i_size_read(inode), PAGE_SIZE);
    if (unlikely(index >= max_idx)) {
    folio_unlock(folio);
    folio_put(folio);
    return VM_FAULT_SIGBUS;
    }
    vmf.page = folio_file_page(folio, index);
    return ret | VM_FAULT_LOCKED;
// label;
//
// Umm, take care of errors if the page isn't up-to-date.
// Try to re-read it _once_. We do this synchronously,
// because there really aren't any performance issues here
// and we need to check for errors.
//
    fpin = maybe_unlock_mmap_for_io(vmf, fpin);
    error = filemap_read_folio(file, mapping.a_ops.read_folio, folio);
    if (fpin) {
// goto;
    }
    folio_put(folio);
    if (!error || error == AOP_TRUNCATED_PAGE) {
// goto;
    }
    filemap_invalidate_unlock_shared(mapping);
    return VM_FAULT_SIGBUS;
// label;
//
// We dropped the mmap_lock, we need to return to the fault handler to
// re-find the vma and come back and find our hopefully still populated
// page.
//
    if (!IS_ERR(folio)) {
    folio_put(folio);
    }
    if (mapping_locked) {
    filemap_invalidate_unlock_shared(mapping);
    }
    if (fpin) {
    fput(fpin);
    }
    return ret | VM_FAULT_RETRY;
    }
    EXPORT_SYMBOL(filemap_fault);
#[no_mangle]
pub unsafe extern "C" fn filemap_map_pmd(vmf: *mut vm_fault, folio: *mut folio, start: pgoff_t) -> bool {
    let mut mm = vmf.vma.vm_mm;
// Huge page is mapped? No need to proceed.
    if (pmd_trans_huge(*vmf.pmd)) {
    folio_unlock(folio);
    folio_put(folio);
    return true;
    }
    if (pmd_none(*vmf.pmd) && folio_test_pmd_mappable(folio)) {
    let mut page = folio_file_page(folio, start);
pub static mut ret: vm_fault_t = 0;
    if (!ret) {
// The page is mapped successfully, reference consumed.
    folio_unlock(folio);
    return true;
    }
    }
    if (pmd_none(*vmf.pmd) && vmf.prealloc_pte) {
    pmd_install(mm, vmf.pmd, &vmf.prealloc_pte);
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn next_uptodate_folio(xas: *mut xa_state, mapping: *mut address_space, end_pgoff: pgoff_t) -> *mut c_void {
    let mut folio = xas_next_entry(xas, end_pgoff);
    let mut max_idx = 0;
    do {
    if (!folio) {
    return core::ptr::null_mut();
    }
    if (xas_retry(xas, folio)) {
    continue;
    }
    if (xa_is_value(folio)) {
    continue;
    }
    if (!folio_try_get(folio)) {
    continue;
    }
    if (folio_test_locked(folio)) {
// goto;
    }
// Has the page moved or been split?
    if (unlikely(folio != xas_reload(xas))) {
// goto;
    }
    if (!folio_test_uptodate(folio) || folio_test_readahead(folio)) {
// goto;
    }
    if (!folio_trylock(folio)) {
// goto;
    }
    if (folio.mapping != mapping) {
// goto;
    }
    if (!folio_test_uptodate(folio)) {
// goto;
    }
    max_idx = DIV_ROUND_UP(i_size_read(mapping.host), PAGE_SIZE);
    if (xas.xa_index >= max_idx) {
// goto;
    }
    return folio;
// label;
    folio_unlock(folio);
// label;
    folio_put(folio);
    } while ((folio = xas_next_entry(xas, end_pgoff)) != core::ptr::null_mut());
    return core::ptr::null_mut();
    }
//
// Map page range [start_page, start_page + nr_pages) of folio.
// start_page is gotten from start by folio_page(folio, start)
//
    static vm_fault_t filemap_map_folio_range(vm_fault *vmf, folio *folio, unsigned long start,
    unsigned long addr, unsigned int nr_pages,
    unsigned long *rss, pgoff_t file_end)
    {
    let mut mapping = folio.mapping;
pub static mut ref_from_caller: c_uint = 1;
pub static mut ret: vm_fault_t = 0;
    let mut page = folio_page(folio, start);
pub static mut count: c_uint = 0;
    let mut old_ptep = vmf.pte;
    let mut addr0 = 0;
//
// Map the large folio fully where possible:
//
// - The folio is fully within size of the file or belong
// to shmem/tmpfs;
// - The folio doesn't cross VMA boundary;
// - The folio doesn't cross page table boundary;
//
    addr0 = addr - start * PAGE_SIZE;
    if ((file_end >= folio_next_index(folio) || shmem_mapping(mapping)) &&
    folio_within_vma(folio, vmf.vma) &&
    (addr0 & PMD_MASK) == ((addr0 + folio_size(folio) - 1) & PMD_MASK)) {
    vmf.pte -= start;
    page -= start;
    addr = addr0;
    nr_pages = folio_nr_pages(folio);
    }
    do {
    if (PageHWPoison(page + count)) {
// goto;
    }
//
// NOTE: If there're PTE markers, we'll leave them to be
// handled in the specific fault path, and it'll prohibit the
// fault-around logic.
//
    if (!pte_none(ptep_get(&vmf.pte[count]))) {
// goto;
    }
    count += 1;
    continue;
// label;
    if (count) {
    set_pte_range(vmf, folio, page, count, addr);
// rss += count;
    folio_ref_add(folio, count - ref_from_caller);
    ref_from_caller = 0;
    if (in_range(vmf.address, addr, count * PAGE_SIZE)) {
    ret = VM_FAULT_NOPAGE;
    }
    }
    count += 1;
    page += count;
    vmf.pte += count;
    addr += count * PAGE_SIZE;
    count = 0;
    } while (--nr_pages > 0);
    if (count) {
    set_pte_range(vmf, folio, page, count, addr);
// rss += count;
    folio_ref_add(folio, count - ref_from_caller);
    ref_from_caller = 0;
    if (in_range(vmf.address, addr, count * PAGE_SIZE)) {
    ret = VM_FAULT_NOPAGE;
    }
    }
    vmf.pte = old_ptep;
    if (ref_from_caller) {
// Locked folios cannot get truncated.
    folio_ref_dec(folio);
    }
    return ret;
    }
    static vm_fault_t filemap_map_order0_folio(vm_fault *vmf, folio *folio, unsigned long addr,
    unsigned long *rss)
    {
pub static mut ret: vm_fault_t = 0;
    let mut page = &folio.page;
    if (PageHWPoison(page)) {
// goto;
    }
//
// NOTE: If there're PTE markers, we'll leave them to be
// handled in the specific fault path, and it'll prohibit
// the fault-around logic.
//
    if (!pte_none(ptep_get(vmf.pte))) {
// goto;
    }
    if (vmf.address == addr) {
    ret = VM_FAULT_NOPAGE;
    }
    set_pte_range(vmf, folio, page, 1, addr);
    (*rss)++;
    return ret;
// label;
// Locked folios cannot get truncated.
    folio_ref_dec(folio);
    return ret;
    }
    vm_fault_t filemap_map_pages(vm_fault *vmf,
    pgoff_t start_pgoff, pgoff_t end_pgoff)
    {
    let mut vma = vmf.vma;
    let mut file = vma.vm_file;
    let mut mapping = file.f_mapping;
    pgoff_t file_end, last_pgoff = start_pgoff;
    let mut addr = 0;
    XA_STATE(xas, &mapping.i_pages, start_pgoff);
pub static mut folio: *mut c_void = core::ptr::null_mut();
pub static mut ret: vm_fault_t = 0;
pub static mut rss: c_ulong = 0;
pub static mut nr_pages: c_uint = 0;
//
// Recalculate end_pgoff based on file_end before calling
// next_uptodate_folio() to avoid races with concurrent
// truncation.
//
    file_end = DIV_ROUND_UP(i_size_read(mapping.host), PAGE_SIZE) - 1;
    end_pgoff = min(end_pgoff, file_end);
    rcu_read_lock();
    folio = next_uptodate_folio(&xas, mapping, end_pgoff);
    if (!folio) {
// goto;
    }
//
// Do not allow to map with PMD across i_size to preserve
// SIGBUS semantics.
//
// Make an exception for shmem/tmpfs that for long time
// intentionally mapped with PMDs across i_size.
//
    if ((file_end >= folio_next_index(folio) || shmem_mapping(mapping)) &&
    filemap_map_pmd(vmf, folio, start_pgoff)) {
    ret = VM_FAULT_NOPAGE;
// goto;
    }
    addr = vma.vm_start +
    ((start_pgoff - vma_start_pgoff(vma)) << PAGE_SHIFT);
    vmf.pte = pte_offset_map_lock(vma.vm_mm, vmf.pmd, addr, &vmf.ptl);
    if (!vmf.pte) {
    folio_unlock(folio);
    folio_put(folio);
// goto;
    }
    folio_type = mm_counter_file(folio);
    do {
    let mut end = 0;
    let mut map_ret;
    addr += (xas.xa_index - last_pgoff) << PAGE_SHIFT;
    vmf.pte += xas.xa_index - last_pgoff;
    last_pgoff = xas.xa_index;
    end = folio_next_index(folio) - 1;
    nr_pages = min(end, end_pgoff) - xas.xa_index + 1;
    if (!folio_test_large(folio)) {
    map_ret = filemap_map_order0_folio(vmf, folio, addr,
    &rss);
    } else {
pub static mut start: c_ulong = 0;
    map_ret = filemap_map_folio_range(vmf, folio, start,
    addr, nr_pages, &rss,
    file_end);
    }
    ret |= map_ret;
//
// If there are too many folios that are recently evicted
// in a file, they will probably continue to be evicted.
// In such situation, read-ahead is only a waste of IO.
// Don't decrease mmap_miss in this scenario to make sure
// we can stop read-ahead.
//
// VM_SEQ_READ and VM_EXEC mappings skip the mmap_miss
// increment in do_sync_mmap_readahead(), so skip the
// decrement here as well to keep the counter symmetric.
//
    if ((map_ret & VM_FAULT_NOPAGE) &&
    !(vmf.flags & FAULT_FLAG_TRIED) &&
    !folio_test_workingset(folio) &&
    !(vma.vm_flags & (VM_SEQ_READ | VM_EXEC))) {
    let mut mmap_miss = 0;
    mmap_miss = READ_ONCE(file.f_ra.mmap_miss);
    if (mmap_miss) {
    WRITE_ONCE(file.f_ra.mmap_miss,
    mmap_miss - 1);
    }
    }
    folio_unlock(folio);
    } while ((folio = next_uptodate_folio(&xas, mapping, end_pgoff)) != core::ptr::null_mut());
    add_mm_counter(vma.vm_mm, folio_type, rss);
    pte_unmap_unlock(vmf.pte, vmf.ptl);
    trace_mm_filemap_map_pages(mapping, start_pgoff, end_pgoff);
// label;
    rcu_read_unlock();
    return ret;
    }
    EXPORT_SYMBOL(filemap_map_pages);
#[no_mangle]
pub unsafe extern "C" fn filemap_page_mkwrite(vmf: *mut vm_fault) -> vm_fault_t {
    let mut mapping = vmf.vma.vm_file.f_mapping;
    let mut folio = page_folio(vmf.page);
pub static mut ret: vm_fault_t = 0;
    sb_start_pagefault(mapping.host.i_sb);
    file_update_time(vmf.vma.vm_file);
    folio_lock(folio);
    if (folio.mapping != mapping) {
    folio_unlock(folio);
    ret = VM_FAULT_NOPAGE;
// goto;
    }
//
// We mark the folio dirty already here so that when freeze is in
// progress, we are guaranteed that writeback during freezing will
// see the dirty folio and writeprotect it again.
//
    folio_mark_dirty(folio);
    folio_wait_stable(folio);
// label;
    sb_end_pagefault(mapping.host.i_sb);
    return ret;
    }
pub static mut vm_operations_struct: usize = 0;
// This is used for a general mmap of a disk file
#[no_mangle]
pub unsafe extern "C" fn generic_file_mmap(file: *mut file, vma: *mut vm_area_struct) -> c_int {
    let mut mapping = file.f_mapping;
    if (!mapping.a_ops.read_folio) {
    return -ENOEXEC;
    }
    file_accessed(file);
    vma.vm_ops = &generic_file_vm_ops;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn generic_file_mmap_prepare(desc: *mut vm_area_desc) -> c_int {
    let mut file = desc.file;
    let mut mapping = file.f_mapping;
    if (!mapping.a_ops.read_folio) {
    return -ENOEXEC;
    }
    file_accessed(file);
    desc.vm_ops = &generic_file_vm_ops;
    return 0;
    }
//
// This is for filesystems which do not implement ->writepage.
//
#[no_mangle]
pub unsafe extern "C" fn generic_file_readonly_mmap(file: *mut file, vma: *mut vm_area_struct) -> c_int {
    if (vma_is_shared_maywrite(vma)) {
    return -EINVAL;
    }
    return generic_file_mmap(file, vma);
    }
#[no_mangle]
pub unsafe extern "C" fn generic_file_readonly_mmap_prepare(desc: *mut vm_area_desc) -> c_int {
    if (is_shared_maywrite(&desc.vma_flags)) {
    return -EINVAL;
    }
    return generic_file_mmap_prepare(desc);
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: filemap_page_mkwrite
pub unsafe extern "C" fn filemap_page_mkwrite_dup(vmf: *mut vm_fault) -> vm_fault_t {
    return VM_FAULT_SIGBUS;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: generic_file_mmap
pub unsafe extern "C" fn generic_file_mmap_dup(file: *mut file, vma: *mut vm_area_struct) -> c_int {
    return -ENOSYS;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: generic_file_mmap_prepare
pub unsafe extern "C" fn generic_file_mmap_prepare_dup(desc: *mut vm_area_desc) -> c_int {
    return -ENOSYS;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: generic_file_readonly_mmap
pub unsafe extern "C" fn generic_file_readonly_mmap_dup(file: *mut file, vma: *mut vm_area_struct) -> c_int {
    return -ENOSYS;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: generic_file_readonly_mmap_prepare
pub unsafe extern "C" fn generic_file_readonly_mmap_prepare_dup(desc: *mut vm_area_desc) -> c_int {
    return -ENOSYS;
    }

    EXPORT_SYMBOL(filemap_page_mkwrite);
    EXPORT_SYMBOL(generic_file_mmap);
    EXPORT_SYMBOL(generic_file_mmap_prepare);
    EXPORT_SYMBOL(generic_file_readonly_mmap);
    EXPORT_SYMBOL(generic_file_readonly_mmap_prepare);
#[no_mangle]
pub unsafe extern "C" fn do_read_cache_folio(mapping: *mut address_space, index: pgoff_t, filler: filler_t, file: *mut file, gfp: gfp_t) -> *mut c_void {
pub static mut folio: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    if (!filler) {
    filler = mapping.a_ops.read_folio;
    }
// label;
    folio = filemap_get_folio(mapping, index);
    if (IS_ERR(folio)) {
    folio = filemap_alloc_folio(gfp, mapping_min_folio_order(mapping), core::ptr::null_mut());
    if (!folio) {
    return ERR_PTR(-ENOMEM);
    }
    index = mapping_align_index(mapping, index);
    err = filemap_add_folio(mapping, folio, index, gfp);
    if (unlikely(err)) {
    folio_put(folio);
    if (err == -EEXIST) {
// goto;
    }
// Presumably ENOMEM for xarray node
    return ERR_PTR(err);
    }
// goto;
    }
    if (folio_test_uptodate(folio)) {
// goto;
    }
    if (!folio_trylock(folio)) {
    folio_put_wait_locked(folio, TASK_UNINTERRUPTIBLE);
// goto;
    }
// Folio was truncated from mapping
    if (!folio.mapping) {
    folio_unlock(folio);
    folio_put(folio);
// goto;
    }
// Someone else locked and filled the page in a very small window
    if (folio_test_uptodate(folio)) {
    folio_unlock(folio);
// goto;
    }
// label;
    err = filemap_read_folio(file, filler, folio);
    if (err) {
    folio_put(folio);
    if (err == AOP_TRUNCATED_PAGE) {
// goto;
    }
    return ERR_PTR(err);
    }
// label;
    folio_mark_accessed(folio);
    return folio;
    }
//
// read_cache_folio - Read into page cache, fill it if needed.
// @mapping: The address_space to read from.
// @index: The index to read.
// @filler: Function to perform the read, or NULL to use aops->read_folio().
// @file: Passed to filler function, may be NULL if not required.
//
// Read one page into the page cache.  If it succeeds, the folio returned
// will contain @index, but it may not be the first page of the folio.
//
// If the filler function returns an error, it will be returned to the
// caller.
//
// Context: May sleep.  Expects mapping->invalidate_lock to be held.
// Return: An uptodate folio on success, ERR_PTR() on failure.
//
#[no_mangle]
pub unsafe extern "C" fn read_cache_folio(mapping: *mut address_space, index: pgoff_t, filler: filler_t, file: *mut file) -> *mut c_void {
    return do_read_cache_folio(mapping, index, filler, file,
    mapping_gfp_mask(mapping));
    }
    EXPORT_SYMBOL(read_cache_folio);
//
// mapping_read_folio_gfp - Read into page cache, using specified allocation flags.
// @mapping:	The address_space for the folio.
// @index:	The index that the allocated folio will contain.
// @gfp:	The page allocator flags to use if allocating.
//
// This is the same as "read_cache_folio(mapping, index, NULL, NULL)", but with
// any new memory allocations done using the specified allocation flags.
//
// The most likely error from this function is EIO, but ENOMEM is
// possible and so is EINTR.  If ->read_folio returns another error,
// that will be returned to the caller.
//
// The function expects mapping->invalidate_lock to be already held.
//
// Return: Uptodate folio on success, ERR_PTR() on failure.
//
#[no_mangle]
pub unsafe extern "C" fn mapping_read_folio_gfp(mapping: *mut address_space, index: pgoff_t, gfp: gfp_t) -> *mut c_void {
    return do_read_cache_folio(mapping, index, core::ptr::null_mut(), core::ptr::null_mut(), gfp);
    }
    EXPORT_SYMBOL(mapping_read_folio_gfp);
#[no_mangle]
pub unsafe extern "C" fn do_read_cache_page(mapping: *mut address_space, index: pgoff_t, filler: *mut filler_t, file: *mut file, gfp: gfp_t) -> *mut c_void {
pub static mut folio: *mut c_void = core::ptr::null_mut();
    folio = do_read_cache_folio(mapping, index, filler, file, gfp);
    if (IS_ERR(folio)) {
    return &folio.page;
    }
    return folio_file_page(folio, index);
    }
#[no_mangle]
pub unsafe extern "C" fn read_cache_page(mapping: *mut address_space, index: pgoff_t, filler: *mut filler_t, file: *mut file) -> *mut c_void {
    return do_read_cache_page(mapping, index, filler, file,
    mapping_gfp_mask(mapping));
    }
    EXPORT_SYMBOL(read_cache_page);
//
// read_cache_page_gfp - read into page cache, using specified page allocation flags.
// @mapping:	the page's address_space
// @index:	the page index
// @gfp:	the page allocator flags to use if allocating
//
// This is the same as "read_mapping_page(mapping, index, NULL)", but with
// any new page allocations done using the specified allocation flags.
//
// If the page does not get brought uptodate, return -EIO.
//
// The function expects mapping->invalidate_lock to be already held.
//
// Return: up to date page on success, ERR_PTR() on failure.
//
#[no_mangle]
pub unsafe extern "C" fn read_cache_page_gfp(mapping: *mut address_space, index: pgoff_t, gfp: gfp_t) -> *mut c_void {
    return do_read_cache_page(mapping, index, core::ptr::null_mut(), core::ptr::null_mut(), gfp);
    }
    EXPORT_SYMBOL(read_cache_page_gfp);
//
// Warn about a page cache invalidation failure during a direct I/O write.
//
#[no_mangle]
unsafe extern "C" fn dio_warn_stale_pagecache(filp: *mut file) {
pub static mut _rs: usize = 0;
    char pathname[128];
pub static mut path: *mut c_void = core::ptr::null_mut();
    errseq_set(&filp.f_mapping.wb_err, -EIO);
    if (__ratelimit(&_rs)) {
    path = file_path(filp, pathname, sizeof!(pathname));
    if (IS_ERR(path)) {
    path = "(unknown)";
    }
    pr_crit("Page cache invalidation failure on direct I/O.  Possible data corruption due to collision with buffered I/O!\n");
    pr_crit("File: %s PID: %d Comm: %.20s\n", path, current.pid,
    current.comm);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kiocb_invalidate_post_direct_write(iocb: *mut kiocb, count: usize) {
    let mut mapping = iocb.ki_filp.f_mapping;
    if (mapping.nrpages &&
    invalidate_inode_pages2_range(mapping,
    iocb.ki_pos >> PAGE_SHIFT,
    (iocb.ki_pos + count - 1) >> PAGE_SHIFT)) {
    dio_warn_stale_pagecache(iocb.ki_filp);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn generic_file_direct_write(iocb: *mut kiocb, from: *mut iov_iter) -> ssize_t {
    let mut mapping = iocb.ki_filp.f_mapping;
pub static mut write_len: usize = 0;
    let mut written = 0;
//
// If a page can not be invalidated, return 0 to fall back
// to buffered write.
//
    written = kiocb_invalidate_pages(iocb, write_len);
    if (written) {
    if (written == -EBUSY) {
    return 0;
    }
    return written;
    }
    written = mapping.a_ops.direct_IO(iocb, from);
//
// Finally, try again to invalidate clean pages which might have been
// cached by non-direct readahead, or faulted in by get_user_pages()
// if the source of the write was an mmap'ed region of the file
// we're writing.  Either one is a pretty crazy thing to do,
// so we don't support it 100%.  If this invalidation
// fails, tough, the write still worked...
//
// Most of the time we do not need this since dio_complete() will do
// the invalidation for us. However there are some file systems that
// do not end up with dio_complete() being called, so let's not break
// them by removing it completely.
//
// Noticeable example is a blkdev_direct_IO().
//
// Skip invalidation for async writes or if mapping has no pages.
//
    if (written > 0) {
    let mut inode = mapping.host;
pub static mut pos: loff_t = 0;
    kiocb_invalidate_post_direct_write(iocb, written);
    pos += written;
    write_len -= written;
    if (pos > i_size_read(inode) && !S_ISBLK(inode.i_mode)) {
    i_size_write(inode, pos);
    mark_inode_dirty(inode);
    }
    iocb.ki_pos = pos;
    }
    if (written != -EIOCBQUEUED) {
    iov_iter_revert(from, write_len - iov_iter_count(from));
    }
    return written;
    }
    EXPORT_SYMBOL(generic_file_direct_write);
#[no_mangle]
pub unsafe extern "C" fn generic_perform_write(iocb: *mut kiocb, i: *mut iov_iter) -> isize {
    let mut file = iocb.ki_filp;
pub static mut pos: loff_t = 0;
    let mut mapping = file.f_mapping;
    let mut a_ops = mapping.a_ops;
pub static mut chunk: usize = 0;
pub static mut status: c_long = 0;
pub static mut written: isize = 0;
    do {
pub static mut folio: *mut c_void = core::ptr::null_mut();
    let mut offset = 0;		/* Offset into folio */
    let mut bytes = 0;		/* Bytes to write to folio */
    let mut copied = 0;		/* Bytes copied from user */
    let mut fsdata = core::ptr::null_mut();
    bytes = iov_iter_count(i);
// label;
    offset = pos & (chunk - 1);
    bytes = min(chunk - offset, bytes);
    balance_dirty_pages_ratelimited(mapping);
    if (fatal_signal_pending(current)) {
    status = -EINTR;
    break;
    }
    status = a_ops.write_begin(iocb, mapping, pos, bytes,
    &folio, &fsdata);
    if (unlikely(status < 0)) {
    break;
    }
    offset = offset_in_folio(folio, pos);
    if (bytes > folio_size(folio) - offset) {
    bytes = folio_size(folio) - offset;
    }
    if (mapping_writably_mapped(mapping)) {
    flush_dcache_folio(folio);
    }
//
// Faults here on mmap()s can recurse into arbitrary
// filesystem code. Lots of locks are held that can
// deadlock. Use an atomic copy to avoid deadlocking
// in page fault handling.
//
    copied = copy_folio_from_iter_atomic(folio, offset, bytes, i);
    flush_dcache_folio(folio);
    status = a_ops.write_end(iocb, mapping, pos, bytes, copied,
    folio, fsdata);
    if (unlikely(status != copied)) {
    iov_iter_revert(i, copied - max(status, 0L));
    if (unlikely(status < 0)) {
    break;
    }
    }
    cond_resched();
    if (unlikely(status == 0)) {
//
// A short copy made ->write_end() reject the
// thing entirely.  Might be memory poisoning
// halfway through, might be a race with munmap,
// might be severe memory pressure.
//
    if (chunk > PAGE_SIZE) {
    chunk /= 2;
    }
    if (copied) {
    bytes = copied;
// goto;
    }
//
// 'folio' is now unlocked and faults on it can be
// handled. Ensure forward progress by trying to
// fault it in now.
//
    if (fault_in_iov_iter_readable(i, bytes) == bytes) {
    status = -EFAULT;
    break;
    }
    } else {
    pos += status;
    written += status;
    }
    } while (iov_iter_count(i));
    if (!written) {
    return status;
    }
    iocb.ki_pos += written;
    return written;
    }
    EXPORT_SYMBOL(generic_perform_write);
//
// __generic_file_write_iter - write data to a file
// @iocb:	IO state structure (file, offset, etc.)
// @from:	iov_iter with data to write
//
// This function does all the work needed for actually writing data to a
// file. It does all basic checks, removes SUID from the file, updates
// modification times and calls proper subroutines depending on whether we
// do direct IO or a standard buffered write.
//
// It expects i_rwsem to be grabbed unless we work on a block device or similar
// object which does not need locking at all.
//
// This function does *not* take care of syncing data in case of O_SYNC write.
// A caller has to handle it. This is mainly due to the fact that we want to
// avoid syncing under i_rwsem.
//
// Return:
// * number of bytes written, even for truncated writes
// * negative error code if no data has been written at all
//
#[no_mangle]
pub unsafe extern "C" fn __generic_file_write_iter(iocb: *mut kiocb, from: *mut iov_iter) -> isize {
    let mut file = iocb.ki_filp;
    let mut mapping = file.f_mapping;
    let mut inode = mapping.host;
    let mut ret = 0;
    ret = file_remove_privs(file);
    if (ret) {
    return ret;
    }
    ret = file_update_time(file);
    if (ret) {
    return ret;
    }
    if (iocb.ki_flags & IOCB_DIRECT) {
    ret = generic_file_direct_write(iocb, from);
//
// If the write stopped short of completing, fall back to
// buffered writes.  Some filesystems do this for writes to
// holes, for example.  For DAX files, a buffered write will
// not succeed (even if it did, DAX does not handle dirty
// page-cache pages correctly).
//
    if (ret < 0 || !iov_iter_count(from) || IS_DAX(inode)) {
    return ret;
    }
    return direct_write_fallback(iocb, from, ret,
    generic_perform_write(iocb, from));
    }
    return generic_perform_write(iocb, from);
    }
    EXPORT_SYMBOL(__generic_file_write_iter);
//
// generic_file_write_iter - write data to a file
// @iocb:	IO state structure
// @from:	iov_iter with data to write
//
// This is a wrapper around __generic_file_write_iter() to be used by most
// filesystems. It takes care of syncing the file in case of O_SYNC file
// and acquires i_rwsem as needed.
// Return:
// * negative error code if no data has been written at all of
// vfs_fsync_range() failed for a synchronous write
// * number of bytes written, even for truncated writes
//
#[no_mangle]
pub unsafe extern "C" fn generic_file_write_iter(iocb: *mut kiocb, from: *mut iov_iter) -> isize {
    let mut file = iocb.ki_filp;
    let mut inode = file.f_mapping.host;
    let mut ret = 0;
    inode_lock(inode);
    ret = generic_write_checks(iocb, from);
    if (ret > 0) {
    ret = __generic_file_write_iter(iocb, from);
    }
    inode_unlock(inode);
    if (ret > 0) {
    ret = generic_write_sync(iocb, ret);
    }
    return ret;
    }
    EXPORT_SYMBOL(generic_file_write_iter);
//
// filemap_release_folio() - Release fs-specific metadata on a folio.
// @folio: The folio which the kernel is trying to free.
// @gfp: Memory allocation flags (and I/O mode).
//
// The address_space is trying to release any data attached to a folio
// (presumably at folio->private).
//
// This will also be called if the private_2 flag is set on a page,
// indicating that the folio has other metadata associated with it.
//
// The @gfp argument specifies whether I/O may be performed to release
// this page (__GFP_IO), and whether the call may block
// (__GFP_RECLAIM & __GFP_FS).
//
// Return: %true if the release was successful, otherwise %false.
//
#[no_mangle]
pub unsafe extern "C" fn filemap_release_folio(folio: *mut folio, gfp: gfp_t) -> bool {
pub static mut mapping: *mut address_space  const = core::ptr::null_mut();
    BUG_ON!(!folio_test_locked(folio));
    if (!folio_needs_release(folio)) {
    return true;
    }
    if (folio_test_writeback(folio)) {
    return false;
    }
    if (mapping && mapping.a_ops.release_folio) {
    return mapping.a_ops.release_folio(folio, gfp);
    }
    return try_to_free_buffers(folio);
    }
    EXPORT_SYMBOL(filemap_release_folio);
//
// filemap_invalidate_inode - Invalidate/forcibly write back a range of an inode's pagecache
// @inode: The inode to flush
// @flush: Set to write back rather than simply invalidate.
// @start: First byte to in range.
// @end: Last byte in range (inclusive), or LLONG_MAX for everything from start
// onwards.
//
// Invalidate all the folios on an inode that contribute to the specified
// range, possibly writing them back first.  Whilst the operation is
// undertaken, the invalidate lock is held to prevent new folios from being
// installed.
//
#[no_mangle]
pub unsafe extern "C" fn filemap_invalidate_inode(inode: *mut inode, flush: bool, start: loff_t, end: loff_t) -> c_int {
    let mut mapping = inode.i_mapping;
pub static mut first: pgoff_t = 0;
pub static mut last: pgoff_t = 0;
pub static mut nr: pgoff_t = 0;
    if (!mapping || !mapping.nrpages || end < start) {
// goto;
    }
// Prevent new folios from being added to the inode.
    filemap_invalidate_lock(mapping);
    if (!mapping.nrpages) {
// goto;
    }
    unmap_mapping_pages(mapping, first, nr, false);
// Write back the data if we're asked to.
    if (flush) {
    filemap_fdatawrite_range(mapping, start, end);
    }
// Wait for writeback to complete on all folios and discard.
    invalidate_inode_pages2_range(mapping, start / PAGE_SIZE, end / PAGE_SIZE);
// label;
    filemap_invalidate_unlock(mapping);
// label;
    return filemap_check_errors(mapping);
    }
    EXPORT_SYMBOL_GPL(filemap_invalidate_inode);

//
// filemap_cachestat() - compute the page cache statistics of a mapping
// @mapping:	The mapping to compute the statistics for.
// @first_index:	The starting page cache index.
// @last_index:	The final page index (inclusive).
// @cs:	the cachestat struct to write the result to.
//
// This will query the page cache statistics of a mapping in the
// page range of [first_index, last_index] (inclusive). The statistics
// queried include: number of dirty pages, number of pages marked for
// writeback, and the number of (recently) evicted pages.
//
#[no_mangle]
pub unsafe extern "C" fn filemap_cachestat(mapping: *mut address_space, first_index: pgoff_t, last_index: pgoff_t, cs: *mut cachestat) {
    XA_STATE(xas, &mapping.i_pages, first_index);
pub static mut folio: *mut c_void = core::ptr::null_mut();
// Flush stats (and potentially sleep) outside the RCU read section.
    mem_cgroup_flush_stats_ratelimited(core::ptr::null_mut());
    rcu_read_lock();
    xas_for_each(&xas, folio, last_index) {
    let mut order = 0;
    let mut nr_pages = 0;
    pgoff_t folio_first_index, folio_last_index;
//
// Don't deref the folio. It is not pinned, and might
// get freed (and reused) underneath us.
//
// We *could* pin it, but that would be expensive for
// what should be a fast and lightweight syscall.
//
// Instead, derive all information of interest from
// the rcu-protected xarray.
//
    if (xas_retry(&xas, folio)) {
    continue;
    }
    order = xas_get_order(&xas);
    nr_pages = 1 << order;
    folio_first_index = round_down(xas.xa_index, 1 << order);
    folio_last_index = folio_first_index + nr_pages - 1;
// Folios might straddle the range boundaries, only count covered pages
    if (folio_first_index < first_index) {
    nr_pages -= first_index - folio_first_index;
    }
    if (folio_last_index > last_index) {
    nr_pages -= folio_last_index - last_index;
    }
    if (xa_is_value(folio)) {
// page is evicted
    let mut shadow = folio;
    let mut workingset = 0; /* not used */
    cs.nr_evicted += nr_pages;

    if (shmem_mapping(mapping)) {
// shmem file - in swap cache
pub static mut swp: swp_entry_t = 0;
// swapin error results in poisoned entry
    if (!softleaf_is_swap(swp)) {
// goto;
    }
//
// Getting a swap entry from the shmem
// inode means we beat
// shmem_unuse(). rcu_read_lock()
// ensures swapoff waits for us before
// freeing the swapper space. However,
// we can race with swapping and
// invalidation, so there might not be
// a shadow in the swapcache (yet).
//
    shadow = swap_cache_get_shadow(swp);
    if (!shadow) {
// goto;
    }
    }

    if (workingset_test_recent(shadow, true, &workingset, false)) {
    cs.nr_recently_evicted += nr_pages;
    }
// goto;
    }
// page is in cache
    cs.nr_cache += nr_pages;
    if (xas_get_mark(&xas, PAGECACHE_TAG_DIRTY)) {
    cs.nr_dirty += nr_pages;
    }
    if (xas_get_mark(&xas, PAGECACHE_TAG_WRITEBACK)) {
    cs.nr_writeback += nr_pages;
    }
// label;
    if (need_resched()) {
    xas_pause(&xas);
    cond_resched_rcu();
    }
    }
    rcu_read_unlock();
    }
//
// See mincore: reveal pagecache information only for files
// that the calling process has write access to, or could (if
// tried) open for writing.
//
#[no_mangle]
pub unsafe extern "C" fn can_do_cachestat(f: *mut file) -> bool {
    if (f.f_mode & FMODE_WRITE) {
    return true;
    }
    if (file_owner_or_capable(f)) {
    return true;
    }
    return file_permission(f, MAY_WRITE) == 0;
    }
//
// The cachestat(2) system call.
//
// cachestat() returns the page cache statistics of a file in the
// bytes range specified by `off` and `len`: number of cached pages,
// number of dirty pages, number of pages marked for writeback,
// number of evicted pages, and number of recently evicted pages.
//
// An evicted page is a page that is previously in the page cache
// but has been evicted since. A page is recently evicted if its last
// eviction was recent enough that its reentry to the cache would
// indicate that it is actively being used by the system, and that
// there is memory pressure on the system.
//
// `off` and `len` must be non-negative integers. If `len` > 0,
// the queried range is [`off`, `off` + `len`]. If `len` == 0,
// we will query in the range from `off` to the end of the file.
//
// The `flags` argument is unused for now, but is included for future
// extensibility. User should pass 0 (i.e no flag specified).
//
// Currently, hugetlbfs is not supported.
//
// Because the status of a page can change after cachestat() checks it
// but before it returns to the application, the returned values may
// contain stale information.
//
// return values:
// zero        - success
// -EFAULT     - cstat or cstat_range points to an illegal address
// -EINVAL     - invalid flags
// -EBADF      - invalid file descriptor
// -EOPNOTSUPP - file descriptor is of a hugetlbfs file
//
#[no_mangle]
pub unsafe extern "C" fn sys_cachestat(fd: usize, cstat_range: usize, cstat: usize, flags: usize) -> c_long {
    CLASS(fd, f)(fd);
pub static mut mapping: *mut c_void = core::ptr::null_mut();
pub static mut csr: usize = 0;
pub static mut cs: usize = 0;
    pgoff_t first_index, last_index;
    if (fd_empty(f)) {
    return -EBADF;
    }
    if (copy_from_user(&csr, cstat_range,
    sizeof!(cachestat_range))) {
    return -EFAULT;
    }
// hugetlbfs is not supported
    if (is_file_hugepages(fd_file(f))) {
    return -EOPNOTSUPP;
    }
    if (!can_do_cachestat(fd_file(f))) {
    return -EPERM;
    }
    if (flags != 0) {
    return -EINVAL;
    }
    first_index = csr.off >> PAGE_SHIFT;
    last_index =
    csr.len == 0 ? ULONG_MAX : (csr.off + csr.len - 1) >> PAGE_SHIFT;
    memset(&cs, 0, sizeof!(cachestat));
    mapping = fd_file(f).f_mapping;
    filemap_cachestat(mapping, first_index, last_index, &cs);
    if (copy_to_user(cstat, &cs, sizeof!(cachestat))) {
    return -EFAULT;
    }
    return 0;
    }