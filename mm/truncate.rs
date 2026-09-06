//! Automatically rewritten from C to Rust
//! Source: mm/truncate.c
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
// mm/truncate.c - code for taking down pages from address_spaces
//
// Copyright (C) 2002, Linus Torvalds
//
// 10Sep2002	Andrew Morton
// Initial version.
//

#[no_mangle]
pub unsafe extern "C" fn clear_shadow_entries(mapping: *mut address_space, start: c_ulong, max: c_ulong) {
    XA_STATE(xas, &mapping.i_pages, start);
pub static mut folio: *mut c_void = core::ptr::null_mut();
// Handled by shmem itself, or for DAX we do nothing.
    if (shmem_mapping(mapping) || dax_mapping(mapping)) {
    return;
    }
    xas_set_update(&xas, workingset_update_node);
    spin_lock(&mapping.host.i_lock);
    xas_lock_irq(&xas);
// Clear all shadow entries from start to max
    xas_for_each(&xas, folio, max) {
    if (xa_is_value(folio)) {
    xas_store(&xas, core::ptr::null_mut());
    }
    }
    xas_unlock_irq(&xas);
    if (mapping_shrinkable(mapping)) {
    inode_lru_list_add(mapping.host);
    }
    spin_unlock(&mapping.host.i_lock);
    }
//
// Unconditionally remove exceptional entries. Usually called from truncate
// path. Note that the folio_batch may be altered by this function by removing
// exceptional entries similar to what folio_batch_remove_exceptionals() does.
// Please note that indices[] has entries in ascending order as guaranteed by
// either find_get_entries() or find_lock_entries().
//
#[no_mangle]
pub unsafe extern "C" fn truncate_folio_batch_exceptionals(mapping: *mut address_space, fbatch: *mut folio_batch, indices: *mut pgoff_t) {
    XA_STATE(xas, &mapping.i_pages, indices[0]);
pub static mut nr: c_int = 0;
pub static mut folio: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    let mut j = 0;
// Handled by shmem itself
    if (shmem_mapping(mapping)) {
    return;
    }
    for (j = 0; j < nr; j++) {
    if (xa_is_value(fbatch.folios[j]))
    break;
    }
    if (j == nr) {
    return;
    }
    if (dax_mapping(mapping)) {
    while (i < nr) {
    if (xa_is_value(fbatch.folios[i])) {
//
// File systems should already have called
// dax_break_layout_entry() to remove all DAX
// entries while holding a lock to prevent
// establishing new entries. Therefore we
// shouldn't find any here.
//
    WARN_ON_ONCE!(1);
//
// Delete the mapping so truncate_pagecache()
// doesn't loop forever.
//
    dax_delete_mapping_entry(mapping, indices[i]);
    }
    }
// goto;
    }
    xas_set(&xas, indices[j]);
    xas_set_update(&xas, workingset_update_node);
    spin_lock(&mapping.host.i_lock);
    xas_lock_irq(&xas);
    xas_for_each(&xas, folio, indices[nr-1]) {
    if (xa_is_value(folio)) {
    xas_store(&xas, core::ptr::null_mut());
    }
    }
    xas_unlock_irq(&xas);
    if (mapping_shrinkable(mapping)) {
    inode_lru_list_add(mapping.host);
    }
    spin_unlock(&mapping.host.i_lock);
// label;
    folio_batch_remove_exceptionals(fbatch);
    }
//
// folio_invalidate - Invalidate part or all of a folio.
// @folio: The folio which is affected.
// @offset: start of the range to invalidate
// @length: length of the range to invalidate
//
// folio_invalidate() is called when all or part of the folio has become
// invalidated by a truncate operation.
//
// folio_invalidate() does not have to release all buffers, but it must
// ensure that no dirty buffer is left outside @offset and that no I/O
// is underway against any of the blocks which are outside the truncation
// point.  Because the caller is about to free (and possibly reuse) those
// blocks on-disk.
//
#[no_mangle]
pub unsafe extern "C" fn folio_invalidate(folio: *mut folio, offset: usize, length: usize) {
    let mut aops = folio.mapping.a_ops;
    if (aops.invalidate_folio) {
    aops.invalidate_folio(folio, offset, length);
    }
    }
    EXPORT_SYMBOL_GPL(folio_invalidate);
//
// If truncate cannot remove the fs-private metadata from the page, the page
// becomes orphaned.  It will be left on the LRU and may even be mapped into
// user pagetables if we're racing with filemap_fault().
//
// We need to bail out if page->mapping is no longer equal to the original
// mapping.  This happens a) when the VM reclaimed the page while we waited on
// its lock, b) when a concurrent invalidate_mapping_pages got there first and
// c) when tmpfs swizzles a page between a tmpfs inode and swapper_space.
//
#[no_mangle]
unsafe extern "C" fn truncate_cleanup_folio(folio: *mut folio) {
    if (folio_mapped(folio)) {
    unmap_mapping_folio(folio);
    }
    if (folio_needs_release(folio)) {
    folio_invalidate(folio, 0, folio_size(folio));
    }
//
// Some filesystems seem to re-dirty the page even after
// the VM has canceled the dirty bit (eg ext3 journaling).
// Hence dirty accounting check is placed after invalidation.
//
    folio_cancel_dirty(folio);
    }
#[no_mangle]
pub unsafe extern "C" fn truncate_inode_folio(mapping: *mut address_space, folio: *mut folio) -> c_int {
    if (folio.mapping != mapping) {
    return -EIO;
    }
    truncate_cleanup_folio(folio);
    filemap_remove_folio(folio);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn folio_split_or_unmap(folio: *mut folio, split_at: *mut page, min_order: c_ulong) -> c_int {
    enum ttu_flags ttu_flags =
    TTU_SYNC |
    TTU_SPLIT_HUGE_PMD |
    TTU_IGNORE_MLOCK;
    let mut ret = 0;
    ret = folio_split(folio, min_order, split_at, core::ptr::null_mut());
//
// If the split fails, unmap the folio, so it will be refaulted
// with PTEs to respect SIGBUS semantics.
//
// Make an exception for shmem/tmpfs that for long time
// intentionally mapped with PMDs across i_size.
//
    if (ret && !shmem_mapping(folio.mapping)) {
    try_to_unmap(folio, ttu_flags);
    WARN_ON!(folio_mapped(folio));
    }
    return ret;
    }
//
// Handle partial folios.  The folio may be entirely within the
// range if a split has raced with us.  If not, we zero the part of the
// folio that's within the [start, end] range, and then split the folio if
// it's large.  split_page_range() will discard pages which now lie beyond
// i_size, and we rely on the caller to discard pages which lie within a
// newly created hole.
//
// Returns false if splitting failed so the caller can avoid
// discarding the entire folio which is stubbornly unsplit.
//
#[no_mangle]
pub unsafe extern "C" fn truncate_inode_partial_folio(folio: *mut folio, start: loff_t, end: loff_t) -> bool {
pub static mut pos: loff_t = 0;
pub static mut size: usize = 0;
    let mut offset = 0;
    let mut length = 0;
    let mut split_at = core::ptr::null_mut();
    let mut split_at2 = core::ptr::null_mut();
    let mut min_order = 0;
    if (pos < start) {
    offset = start - pos;
    }
    else {
    offset = 0;
    }
    if (pos + size <= (u64)end) {
    length = size - offset;
    }
    else {
    length = end + 1 - pos - offset;
    }
    folio_wait_writeback(folio);
    if (length == size) {
    truncate_inode_folio(folio.mapping, folio);
    return true;
    }
//
// We may be zeroing pages we're about to discard, but it avoids
// doing a complex calculation here, and then doing the zeroing
// anyway if the page split fails.
//
    if (!mapping_inaccessible(folio.mapping)) {
    folio_zero_range(folio, offset, length);
    }
    if (folio_needs_release(folio)) {
    folio_invalidate(folio, offset, length);
    }
    if (!folio_test_large(folio)) {
    return true;
    }
    min_order = mapping_min_folio_order(folio.mapping);
    split_at = folio_page(folio, PAGE_ALIGN_DOWN(offset) / PAGE_SIZE);
    if (!folio_split_or_unmap(folio, split_at, min_order)) {
//
// try to split at offset + length to make sure folios within
// the range can be dropped, especially to avoid memory waste
// for shmem truncate
//
pub static mut folio2: *mut c_void = core::ptr::null_mut();
    if (offset + length == size) {
// goto;
    }
    split_at2 = folio_page(folio,
    PAGE_ALIGN_DOWN(offset + length) / PAGE_SIZE);
    folio2 = page_folio(split_at2);
    if (!folio_try_get(folio2)) {
// goto;
    }
    if (!folio_test_large(folio2)) {
// goto;
    }
    if (!folio_trylock(folio2)) {
// goto;
    }
// make sure folio2 is large and does not change its mapping
    if (folio_test_large(folio2) &&
    folio2.mapping == folio.mapping) {
    folio_split_or_unmap(folio2, split_at2, min_order);
    }
    folio_unlock(folio2);
// label;
    folio_put(folio2);
// label;
    return true;
    }
    if (folio_test_dirty(folio)) {
    return false;
    }
    truncate_inode_folio(folio.mapping, folio);
    return true;
    }
//
// Used to get rid of pages on hardware memory corruption.
//
#[no_mangle]
pub unsafe extern "C" fn generic_error_remove_folio(mapping: *mut address_space, folio: *mut folio) -> c_int {
    if (!mapping) {
    return -EINVAL;
    }
//
// Only punch for normal data pages for now.
// Handling other types like directories would need more auditing.
//
    if (!S_ISREG(mapping.host.i_mode)) {
    return -EIO;
    }
    return truncate_inode_folio(mapping, folio);
    }
    EXPORT_SYMBOL(generic_error_remove_folio);
//
// mapping_evict_folio() - Remove an unused folio from the page-cache.
// @mapping: The mapping this folio belongs to.
// @folio: The folio to remove.
//
// Safely remove one folio from the page cache.
// It only drops clean, unused folios.
//
// Context: Folio must be locked.
// Return: The number of pages successfully removed.
//
#[no_mangle]
pub unsafe extern "C" fn mapping_evict_folio(mapping: *mut address_space, folio: *mut folio) -> c_long {
// The page may have been truncated before it was locked
    if (!mapping) {
    return 0;
    }
    if (folio_test_dirty(folio) || folio_test_writeback(folio)) {
    return 0;
    }
// The refcount will be elevated if any page in the folio is mapped
    if (folio_ref_count(folio) >
    folio_nr_pages(folio) + folio_has_private(folio) + 1) {
    return 0;
    }
    if (!filemap_release_folio(folio, 0)) {
    return 0;
    }
    return remove_mapping(mapping, folio);
    }
//
// truncate_inode_pages_range - truncate range of pages specified by start & end byte offsets
// @mapping: mapping to truncate
// @lstart: offset from which to truncate
// @lend: offset to which to truncate (inclusive)
//
// Truncate the page cache, removing the pages that are between
// specified offsets (and zeroing out partial pages
// if lstart or lend + 1 is not page aligned).
//
// Truncate takes two passes - the first pass is nonblocking.  It will not
// block on page locks and it will not block on writeback.  The second pass
// will wait.  This is to prevent as much IO as possible in the affected region.
// The first pass will remove most pages, so the search cost of the second pass
// is low.
//
// We pass down the cache-hot hint to the page freeing code.  Even if the
// mapping is large, it is probably the case that the final pages are the most
// recently touched, and freeing happens in ascending file offset order.
//
// Note that since ->invalidate_folio() accepts range to invalidate
// truncate_inode_pages_range is able to handle cases where lend + 1 is not
// page aligned properly.
//
#[no_mangle]
pub unsafe extern "C" fn truncate_inode_pages_range(mapping: *mut address_space, lstart: loff_t, lend: uoff_t) {
    let mut start;		/* inclusive */
    let mut end;		/* exclusive */
pub static mut fbatch: usize = 0;
    pgoff_t		indices[FOLIO_BATCH_SIZE];
    let mut index;
    let mut i = 0;
pub static mut folio: *mut c_void = core::ptr::null_mut();
    let mut same_folio = 0;
    if (mapping_empty(mapping)) {
    return;
    }
//
// 'start' and 'end' always covers the range of pages to be fully
// truncated. Partial pages are covered with 'partial_start' at the
// start of the range and 'partial_end' at the end of the range.
// Note that 'end' is exclusive while 'lend' is inclusive.
//
    start = (lstart + PAGE_SIZE - 1) >> PAGE_SHIFT;
    if (lend == -1) {
//
// lend == -1 indicates end-of-file so we have to set 'end'
// to the highest possible pgoff_t and since the type is
// unsigned we're using -1.
//
    end = -1;
    }
    else {
    end = (lend + 1) >> PAGE_SHIFT;
    }
    folio_batch_init(&fbatch);
    index = start;
    while (index < end && find_lock_entries(mapping, &index, end - 1,
    &fbatch, indices)) {
    truncate_folio_batch_exceptionals(mapping, &fbatch, indices);
    for (i = 0; i < folio_batch_count(&fbatch); i++) {
    truncate_cleanup_folio(fbatch.folios[i]);
    }
    delete_from_page_cache_batch(mapping, &fbatch);
    for (i = 0; i < folio_batch_count(&fbatch); i++) {
    folio_unlock(fbatch.folios[i]);
    }
    folio_batch_release(&fbatch);
    cond_resched();
    }
    same_folio = (lstart >> PAGE_SHIFT) == (lend >> PAGE_SHIFT);
    folio = __filemap_get_folio(mapping, lstart >> PAGE_SHIFT, FGP_LOCK, 0);
    if (!IS_ERR(folio)) {
    same_folio = lend < folio_next_pos(folio);
    if (!truncate_inode_partial_folio(folio, lstart, lend)) {
    start = folio_next_index(folio);
    if (same_folio) {
    end = folio.index;
    }
    }
    folio_unlock(folio);
    folio_put(folio);
    folio = core::ptr::null_mut();
    }
    if (!same_folio) {
    folio = __filemap_get_folio(mapping, lend >> PAGE_SHIFT,
    FGP_LOCK, 0);
    if (!IS_ERR(folio)) {
    if (!truncate_inode_partial_folio(folio, lstart, lend)) {
    end = folio.index;
    }
    folio_unlock(folio);
    folio_put(folio);
    }
    }
    index = start;
    while (index < end) {
    cond_resched();
    if (!find_get_entries(mapping, &index, end - 1, &fbatch,
    indices)) {
// If all gone from start onwards, we're done
    if (index == start) {
    break;
    }
// Otherwise restart to make sure all gone
    index = start;
    continue;
    }
    while (i < folio_batch_count(&fbatch)) {
    let mut folio = fbatch.folios[i];
// We rely upon deletion not changing folio->index
    if (xa_is_value(folio)) {
    continue;
    }
    folio_lock(folio);
    VM_BUG_ON_FOLIO(!folio_contains(folio, indices[i]), folio);
    folio_wait_writeback(folio);
    truncate_inode_folio(mapping, folio);
    folio_unlock(folio);
    }
    truncate_folio_batch_exceptionals(mapping, &fbatch, indices);
    folio_batch_release(&fbatch);
    }
    }
    EXPORT_SYMBOL(truncate_inode_pages_range);
//
// truncate_inode_pages - truncate *all* the pages from an offset
// @mapping: mapping to truncate
// @lstart: offset from which to truncate
//
// Called under (and serialised by) inode->i_rwsem and
// mapping->invalidate_lock.
//
// Note: When this function returns, there can be a page in the process of
// deletion (inside __filemap_remove_folio()) in the specified range.  Thus
// mapping->nrpages can be non-zero when this function returns even after
// truncation of the whole mapping.
//
#[no_mangle]
pub unsafe extern "C" fn truncate_inode_pages(mapping: *mut address_space, lstart: loff_t) {
    truncate_inode_pages_range(mapping, lstart, (loff_t)-1);
    }
    EXPORT_SYMBOL(truncate_inode_pages);
//
// truncate_inode_pages_final - truncate *all* pages before inode dies
// @mapping: mapping to truncate
//
// Called under (and serialized by) inode->i_rwsem.
//
// Filesystems have to use this in the .evict_inode path to inform the
// VM that this is the final truncate and the inode is going away.
//
#[no_mangle]
pub unsafe extern "C" fn truncate_inode_pages_final(mapping: *mut address_space) {
//
// Page reclaim can not participate in regular inode lifetime
// management (can't call iput()) and thus can race with the
// inode teardown.  Tell it when the address space is exiting,
// so that it does not install eviction information after the
// final truncate has begun.
//
    mapping_set_exiting(mapping);
    if (!mapping_empty(mapping)) {
//
// As truncation uses a lockless tree lookup, cycle
// the tree lock to make sure any ongoing tree
// modification that does not see AS_EXITING is
// completed before starting the final truncate.
//
    xa_lock_irq(&mapping.i_pages);
    xa_unlock_irq(&mapping.i_pages);
    }
    truncate_inode_pages(mapping, 0);
    }
    EXPORT_SYMBOL(truncate_inode_pages_final);
//
// mapping_try_invalidate - Invalidate all the evictable folios of one inode
// @mapping: the address_space which holds the folios to invalidate
// @start: the offset 'from' which to invalidate
// @end: the offset 'to' which to invalidate (inclusive)
// @nr_failed: How many folio invalidations failed
//
// This function is similar to invalidate_mapping_pages(), except that it
// returns the number of folios which could not be evicted in @nr_failed.
//
#[no_mangle]
pub unsafe extern "C" fn mapping_try_invalidate(mapping: *mut address_space, start: pgoff_t, end: pgoff_t, nr_failed: *mut c_ulong) -> c_ulong {
    pgoff_t indices[FOLIO_BATCH_SIZE];
pub static mut fbatch: usize = 0;
pub static mut index: pgoff_t = 0;
    let mut ret = 0;
pub static mut count: c_ulong = 0;
    let mut i = 0;
    folio_batch_init(&fbatch);
    while (find_lock_entries(mapping, &index, end, &fbatch, indices)) {
pub static mut xa_has_values: bool = false;
pub static mut nr: c_int = 0;
    while (i < nr) {
    let mut folio = fbatch.folios[i];
// We rely upon deletion not changing folio->index
    if (xa_is_value(folio)) {
    xa_has_values = true;
    count += 1;
    continue;
    }
    ret = mapping_evict_folio(mapping, folio);
    folio_unlock(folio);
//
// Invalidation is a hint that the folio is no longer
// of interest and try to speed up its reclaim.
//
    if (!ret) {
    deactivate_file_folio(folio);
// Likely in the lru cache of a remote CPU
    if (nr_failed) {
    (*nr_failed)++;
    }
    }
    count += ret;
    }
    if (xa_has_values) {
    clear_shadow_entries(mapping, indices[0], indices[nr-1]);
    }
    folio_batch_remove_exceptionals(&fbatch);
    folio_batch_release(&fbatch);
    cond_resched();
    }
    return count;
    }
//
// invalidate_mapping_pages - Invalidate all clean, unlocked cache of one inode
// @mapping: the address_space which holds the cache to invalidate
// @start: the offset 'from' which to invalidate
// @end: the offset 'to' which to invalidate (inclusive)
//
// This function removes pages that are clean, unmapped and unlocked,
// as well as shadow entries. It will not block on IO activity.
//
// If you want to remove all the pages of one inode, regardless of
// their use and writeback state, use truncate_inode_pages().
//
// Return: The number of indices that had their contents invalidated
//
#[no_mangle]
pub unsafe extern "C" fn invalidate_mapping_pages(mapping: *mut address_space, start: pgoff_t, end: pgoff_t) -> c_ulong {
    return mapping_try_invalidate(mapping, start, end, core::ptr::null_mut());
    }
    EXPORT_SYMBOL(invalidate_mapping_pages);
#[no_mangle]
unsafe extern "C" fn folio_launder(mapping: *mut address_space, folio: *mut folio) -> c_int {
    if (!folio_test_dirty(folio)) {
    return 0;
    }
    if (folio.mapping != mapping || mapping.a_ops.launder_folio == core::ptr::null_mut()) {
    return 0;
    }
    return mapping.a_ops.launder_folio(folio);
    }
//
// This is like mapping_evict_folio(), except it ignores the folio's
// refcount.  We do this because invalidate_inode_pages2() needs stronger
// invalidation guarantees, and cannot afford to leave folios behind because
// shrink_folio_list() has a temp ref on them, or because they're transiently
// sitting in the folio_add_lru() caches.
//
#[no_mangle]
pub unsafe extern "C" fn folio_unmap_invalidate(mapping: *mut address_space, folio: *mut folio, gfp: gfp_t) -> c_int {
    void (*free_folio);
    let mut ret = 0;
    VM_BUG_ON_FOLIO(!folio_test_locked(folio), folio);
    if (folio_mapped(folio)) {
    unmap_mapping_folio(folio);
    }
    BUG_ON!(folio_mapped(folio));
    ret = folio_launder(mapping, folio);
    if (ret) {
    return ret;
    }
    if (folio.mapping != mapping) {
    return -EBUSY;
    }
    if (!filemap_release_folio(folio, gfp)) {
    return -EBUSY;
    }
    spin_lock(&mapping.host.i_lock);
    xa_lock_irq(&mapping.i_pages);
    if (folio_test_dirty(folio)) {
// goto;
    }
    BUG_ON!(folio_has_private(folio));
    __filemap_remove_folio(folio, core::ptr::null_mut());
    xa_unlock_irq(&mapping.i_pages);
    if (mapping_shrinkable(mapping)) {
    inode_lru_list_add(mapping.host);
    }
    free_folio = mapping.a_ops.free_folio;
    spin_unlock(&mapping.host.i_lock);
    if (free_folio) {
    free_folio(folio);
    }
    folio_put_refs(folio, folio_nr_pages(folio));
    return 1;
// label;
    xa_unlock_irq(&mapping.i_pages);
    spin_unlock(&mapping.host.i_lock);
    return -EBUSY;
    }
//
// invalidate_inode_pages2_range - remove range of pages from an address_space
// @mapping: the address_space
// @start: the page offset 'from' which to invalidate
// @end: the page offset 'to' which to invalidate (inclusive)
//
// Any pages which are found to be mapped into pagetables are unmapped prior to
// invalidation.
//
// Return: -EBUSY if any pages could not be invalidated.
//
#[no_mangle]
pub unsafe extern "C" fn invalidate_inode_pages2_range(mapping: *mut address_space, start: pgoff_t, end: pgoff_t) -> c_int {
    pgoff_t indices[FOLIO_BATCH_SIZE];
pub static mut fbatch: usize = 0;
    let mut index;
    let mut i = 0;
pub static mut ret: c_int = 0;
pub static mut ret2: c_int = 0;
pub static mut did_range_unmap: c_int = 0;
    if (mapping_empty(mapping)) {
    return 0;
    }
    folio_batch_init(&fbatch);
    index = start;
    while (find_get_entries(mapping, &index, end, &fbatch, indices)) {
pub static mut xa_has_values: bool = false;
pub static mut nr: c_int = 0;
    while (i < nr) {
    let mut folio = fbatch.folios[i];
// We rely upon deletion not changing folio->index
    if (xa_is_value(folio)) {
    xa_has_values = true;
    if (dax_mapping(mapping) &&
    !dax_invalidate_mapping_entry_sync(mapping, indices[i])) {
    ret = -EBUSY;
    }
    continue;
    }
    if (!did_range_unmap && folio_mapped(folio)) {
//
// If folio is mapped, before taking its lock,
// zap the rest of the file in one hit.
//
    unmap_mapping_pages(mapping, indices[i],
    (1 + end - indices[i]), false);
    did_range_unmap = 1;
    }
    folio_lock(folio);
    if (unlikely(folio.mapping != mapping)) {
    folio_unlock(folio);
    continue;
    }
    VM_BUG_ON_FOLIO(!folio_contains(folio, indices[i]), folio);
    folio_wait_writeback(folio);
    ret2 = folio_unmap_invalidate(mapping, folio, GFP_KERNEL);
    if (ret2 < 0) {
    ret = ret2;
    }
    folio_unlock(folio);
    }
    if (xa_has_values) {
    clear_shadow_entries(mapping, indices[0], indices[nr-1]);
    }
    folio_batch_remove_exceptionals(&fbatch);
    folio_batch_release(&fbatch);
    cond_resched();
    }
//
// For DAX we invalidate page tables after invalidating page cache.  We
// could invalidate page tables while invalidating each entry however
// that would be expensive. And doing range unmapping before doesn't
// work as we have no cheap way to find whether page cache entry didn't
// get remapped later.
//
    if (dax_mapping(mapping)) {
    unmap_mapping_pages(mapping, start, end - start + 1, false);
    }
    return ret;
    }
    EXPORT_SYMBOL_GPL(invalidate_inode_pages2_range);
//
// invalidate_inode_pages2 - remove all pages from an address_space
// @mapping: the address_space
//
// Any pages which are found to be mapped into pagetables are unmapped prior to
// invalidation.
//
// Return: -EBUSY if any pages could not be invalidated.
//
#[no_mangle]
pub unsafe extern "C" fn invalidate_inode_pages2(mapping: *mut address_space) -> c_int {
    return invalidate_inode_pages2_range(mapping, 0, -1);
    }
    EXPORT_SYMBOL_GPL(invalidate_inode_pages2);
//
// truncate_pagecache - unmap and remove pagecache that has been truncated
// @inode: inode
// @newsize: new file size
//
// inode's new i_size must already be written before truncate_pagecache
// is called.
//
// This function should typically be called before the filesystem
// releases resources associated with the freed range (eg. deallocates
// blocks). This way, pagecache will always stay logically coherent
// with on-disk format, and the filesystem would not have to deal with
// situations such as writepage being called for a page that has already
// had its underlying blocks deallocated.
//
#[no_mangle]
pub unsafe extern "C" fn truncate_pagecache(inode: *mut inode, newsize: loff_t) {
    let mut mapping = inode.i_mapping;
pub static mut holebegin: loff_t = 0;
//
// unmap_mapping_range is called twice, first simply for
// efficiency so that truncate_inode_pages does fewer
// single-page unmaps.  However after this first call, and
// before truncate_inode_pages finishes, it is possible for
// private pages to be COWed, which remain after
// truncate_inode_pages finishes, hence the second
// unmap_mapping_range call must be made for correctness.
//
    unmap_mapping_range(mapping, holebegin, 0, 1);
    truncate_inode_pages(mapping, newsize);
    unmap_mapping_range(mapping, holebegin, 0, 1);
    }
    EXPORT_SYMBOL(truncate_pagecache);
//
// truncate_setsize - update inode and pagecache for a new file size
// @inode: inode
// @newsize: new file size
//
// truncate_setsize updates i_size and performs pagecache truncation (if
// necessary) to @newsize. It will be typically be called from the filesystem's
// setattr function when ATTR_SIZE is passed in.
//
// Must be called with a lock serializing truncates and writes (generally
// i_rwsem but e.g. xfs uses a different lock) and before all filesystem
// specific block truncation has been performed.
//
#[no_mangle]
pub unsafe extern "C" fn truncate_setsize(inode: *mut inode, newsize: loff_t) {
pub static mut oldsize: loff_t = 0;
    i_size_write(inode, newsize);
    if (newsize > oldsize) {
    pagecache_isize_extended(inode, oldsize, newsize);
    }
    truncate_pagecache(inode, newsize);
    }
    EXPORT_SYMBOL(truncate_setsize);
//
// pagecache_isize_extended - update pagecache after extension of i_size
// @inode:	inode for which i_size was extended
// @from:	original inode size
// @to:		new inode size
//
// Handle extension of inode size either caused by extending truncate or
// by write starting after current i_size.  We mark the page straddling
// current i_size RO so that page_mkwrite() is called on the first
// write access to the page.  The filesystem will update its per-block
// information before user writes to the page via mmap after the i_size
// has been changed.
//
// The function must be called after i_size is updated so that page fault
// coming after we unlock the folio will already see the new i_size.
// The function must be called while we still hold i_rwsem - this not only
// makes sure i_size is stable but also that userspace cannot observe new
// i_size value before we are prepared to store mmap writes at new inode size.
//
#[no_mangle]
pub unsafe extern "C" fn pagecache_isize_extended(inode: *mut inode, from: loff_t, to: loff_t) {
pub static mut bsize: c_int = 0;
    let mut rounded_from = 0;
pub static mut folio: *mut c_void = core::ptr::null_mut();
    WARN_ON!(to > inode.i_size);
    if (from >= to || bsize >= PAGE_SIZE) {
    return;
    }
// Page straddling @from will not have any hole block created?
    rounded_from = round_up(from, bsize);
    if (to <= rounded_from || !(rounded_from & (PAGE_SIZE - 1))) {
    return;
    }
    folio = filemap_lock_folio(inode.i_mapping, from / PAGE_SIZE);
// Folio not cached? Nothing to do
    if (IS_ERR(folio)) {
    return;
    }
//
// See folio_clear_dirty_for_io() for details why folio_mark_dirty()
// is needed.
//
    if (folio_mkclean(folio)) {
    folio_mark_dirty(folio);
    }
//
// The post-eof range of the folio must be zeroed before it is exposed
// to the file. Writeback normally does this, but since i_size has been
// increased we handle it here.
//
    if (folio_test_dirty(folio)) {
    let mut offset = 0;
    let mut end = 0;
    offset = from - folio_pos(folio);
    end = min_t(unsigned int, to - folio_pos(folio),
    folio_size(folio));
    folio_zero_segment(folio, offset, end);
    }
    folio_unlock(folio);
    folio_put(folio);
    }
    EXPORT_SYMBOL(pagecache_isize_extended);
//
// truncate_pagecache_range - unmap and remove pagecache that is hole-punched
// @inode: inode
// @lstart: offset of beginning of hole
// @lend: offset of last byte of hole
//
// This function should typically be called before the filesystem
// releases resources associated with the freed range (eg. deallocates
// blocks). This way, pagecache will always stay logically coherent
// with on-disk format, and the filesystem would not have to deal with
// situations such as writepage being called for a page that has already
// had its underlying blocks deallocated.
//
#[no_mangle]
pub unsafe extern "C" fn truncate_pagecache_range(inode: *mut inode, lstart: loff_t, lend: loff_t) {
    let mut mapping = inode.i_mapping;
pub static mut unmap_start: loff_t = 0;
pub static mut unmap_end: loff_t = 0;
//
// This rounding is currently just for example: unmap_mapping_range
// expands its hole outwards, whereas we want it to contract the hole
// inwards.  However, existing callers of truncate_pagecache_range are
// doing their own page rounding first.  Note that unmap_mapping_range
// allows holelen 0 for all, and we allow lend -1 for end of file.
//
// Unlike in truncate_pagecache, unmap_mapping_range is called only
// once (before truncating pagecache), and without "even_cows" flag:
// hole-punching should not remove private COWed pages from the hole.
//
    if ((u64)unmap_end > (u64)unmap_start) {
    unmap_mapping_range(mapping, unmap_start,
    1 + unmap_end - unmap_start, 0);
    }
    truncate_inode_pages_range(mapping, lstart, lend);
    }
    EXPORT_SYMBOL(truncate_pagecache_range);