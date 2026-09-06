//! Automatically rewritten from C to Rust
//! Source: mm/readahead.c
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
// mm/readahead.c - address_space-level file readahead.
//
// Copyright (C) 2002, Linus Torvalds
//
// 09Apr2002	Andrew Morton
// Initial version.
//
// DOC: Readahead Overview
//
// Readahead is used to read content into the page cache before it is
// explicitly requested by the application.  Readahead only ever
// attempts to read folios that are not yet in the page cache.  If a
// folio is present but not up-to-date, readahead will not try to read
// it. In that case a simple ->read_folio() will be requested.
//
// Readahead is triggered when an application read request (whether a
// system call or a page fault) finds that the requested folio is not in
// the page cache, or that it is in the page cache and has the
// readahead flag set.  This flag indicates that the folio was read
// as part of a previous readahead request and now that it has been
// accessed, it is time for the next readahead.
//
// Each readahead request is partly synchronous read, and partly async
// readahead.  This is reflected in the struct file_ra_state which
// contains ->size being the total number of pages, and ->async_size
// which is the number of pages in the async section.  The readahead
// flag will be set on the first folio in this async section to trigger
// a subsequent readahead.  Once a series of sequential reads has been
// established, there should be no need for a synchronous component and
// all readahead request will be fully asynchronous.
//
// When either of the triggers causes a readahead, three numbers need
// to be determined: the start of the region to read, the size of the
// region, and the size of the async tail.
//
// The start of the region is simply the first page address at or after
// the accessed address, which is not currently populated in the page
// cache.  This is found with a simple search in the page cache.
//
// The size of the async tail is determined by subtracting the size that
// was explicitly requested from the determined request size, unless
// this would be less than zero - then zero is used.  NOTE THIS
// CALCULATION IS WRONG WHEN THE START OF THE REGION IS NOT THE ACCESSED
// PAGE.  ALSO THIS CALCULATION IS NOT USED CONSISTENTLY.
//
// The size of the region is normally determined from the size of the
// previous readahead which loaded the preceding pages.  This may be
// discovered from the struct file_ra_state for simple sequential reads,
// or from examining the state of the page cache when multiple
// sequential reads are interleaved.  Specifically: where the readahead
// was triggered by the readahead flag, the size of the previous
// readahead is assumed to be the number of pages from the triggering
// page to the start of the new readahead.  In these cases, the size of
// the previous readahead is scaled, often doubled, for the new
// readahead, though see get_next_ra_size() for details.
//
// If the size of the previous read cannot be determined, the number of
// preceding pages in the page cache is used to estimate the size of
// a previous read.  This estimate could easily be misled by random
// reads being coincidentally adjacent, so it is ignored unless it is
// larger than the current request, and it is not scaled up, unless it
// is at the start of file.
//
// In general readahead is accelerated at the start of the file, as
// reads from there are often sequential.  There are other minor
// adjustments to the readahead size in various special cases and these
// are best discovered by reading the code.
//
// The above calculation, based on the previous readahead size,
// determines the size of the readahead, to which any requested read
// size may be added.
//
// Readahead requests are sent to the filesystem using the ->readahead()
// address space operation, for which mpage_readahead() is a canonical
// implementation.  ->readahead() should normally initiate reads on all
// folios, but may fail to read any or all folios without causing an I/O
// error.  The page cache reading code will issue a ->read_folio() request
// for any folio which ->readahead() did not read, and only an error
// from this will be final.
//
// ->readahead() will generally call readahead_folio() repeatedly to get
// each folio from those prepared for readahead.  It may fail to read a
// folio by:
//
// * not calling readahead_folio() sufficiently many times, effectively
// ignoring some folios, as might be appropriate if the path to
// storage is congested.
//
// * failing to actually submit a read request for a given folio,
// possibly due to insufficient resources, or
//
// * getting an error during subsequent processing of a request.
//
// In the last two cases, the folio should be unlocked by the filesystem
// to indicate that the read attempt has failed.  In the first case the
// folio will be unlocked by the VFS.
//
// Those folios not in the final ``async_size`` of the request should be
// considered to be important and ->readahead() should not fail them due
// to congestion or temporary resource unavailability, but should wait
// for necessary resources (e.g.  memory or indexing information) to
// become available.  Folios in the final ``async_size`` may be
// considered less urgent and failure to read them is more acceptable.
// In this case it is best to use filemap_remove_folio() to remove the
// folios from the page cache as is automatically done for folios that
// were not fetched with readahead_folio().  This will allow a
// subsequent synchronous readahead request to try them again.  If they
// are left in the page cache, then they will be read individually using
// ->read_folio() which may be less efficient.
//

// Macro flag: #define CREATE_TRACE_POINTS

//
// Initialise a struct file's readahead state.  Assumes that the caller has
// memset *ra to zero.
//
#[no_mangle]
pub unsafe extern "C" fn file_ra_state_init(ra: *mut file_ra_state, mapping: *mut address_space) {
    ra.ra_pages = inode_to_bdi(mapping.host).ra_pages;
    ra.prev_pos = -1;
    }
    EXPORT_SYMBOL_GPL(file_ra_state_init);
//
// read_pages() - Start IO for a contiguous range of allocated folios in the
// page cache.
// @rac: Readahead control.
//
// When read_pages() returns, it is guaranteed that all of the folios will have
// been processed or removed so that ``readahead_count(rac) == 0``. However,
// that does not imply that ``readahead_index(rac)`` will be updated to point
// to the end of the originally requested range because, for example, the
// filesystem may expand the range upwards.
//
#[no_mangle]
unsafe extern "C" fn read_pages(rac: *mut readahead_control) {
    let mut aops = rac.mapping.a_ops;
pub static mut folio: *mut c_void = core::ptr::null_mut();
pub static mut plug: usize = 0;
    if (!readahead_count(rac)) {
    return;
    }
    if (unlikely(rac._workingset)) {
    psi_memstall_enter(&rac._pflags);
    }
    blk_start_plug(&plug);
    if (aops.readahead) {
    aops.readahead(rac);
// Clean up the remaining folios.
    while ((folio = readahead_folio(rac)) != core::ptr::null_mut()) {
    folio_get(folio);
    filemap_remove_folio(folio);
    folio_unlock(folio);
    folio_put(folio);
    }
    } else {
    while ((folio = readahead_folio(rac)) != core::ptr::null_mut()) {
    aops.read_folio(rac.file, folio);
    }
    }
    blk_finish_plug(&plug);
    if (unlikely(rac._workingset)) {
    psi_memstall_leave(&rac._pflags);
    }
    rac._workingset = false;
    BUG_ON!(readahead_count(rac));
    }
#[no_mangle]
pub unsafe extern "C" fn ractl_alloc_folio(ractl: *mut readahead_control, gfp_mask: gfp_t, order: c_uint) -> *mut c_void {
pub static mut folio: *mut c_void = core::ptr::null_mut();
    folio = filemap_alloc_folio(gfp_mask, order, core::ptr::null_mut());
    if (folio && ractl.dropbehind) {
    __folio_set_dropbehind(folio);
    }
    return folio;
    }
//
// page_cache_ra_unbounded - Start unchecked readahead.
// @ractl: Readahead control.
// @nr_to_read: The number of pages to read.
// @lookahead_size: Where to start the next readahead.
//
// This function is for filesystems to call when they want to start
// readahead beyond a file's stated i_size.  This is almost certainly
// not the function you want to call.  Use page_cache_async_readahead()
// or page_cache_sync_readahead() instead.
//
// Context: File is referenced by caller, and ractl->mapping->invalidate_lock
// must be held by the caller at least in shared mode.  Mutexes may be held by
// caller.  May sleep, but will not reenter filesystem to reclaim memory.
//
#[no_mangle]
pub unsafe extern "C" fn page_cache_ra_unbounded(ractl: *mut readahead_control, nr_to_read: c_ulong, lookahead_size: c_ulong) {
    let mut mapping = ractl.mapping;
pub static mut index: c_ulong = 0;
pub static mut gfp_mask: gfp_t = 0;
pub static mut mark: c_ulong = 0;
pub static mut min_nrpages: c_uint = 0;
//
// Partway through the readahead operation, we will have added
// locked pages to the page cache, but will not yet have submitted
// them for I/O.  Adding another page may need to allocate memory,
// which can trigger memory reclaim.  Telling the VM we're in
// the middle of a filesystem operation will cause it to not
// touch file-backed pages, preventing a deadlock.  Most (all?)
// filesystems already specify __GFP_NOFS in their mapping's
// gfp_mask, but let's be explicit here.
//
pub static mut nofs: c_uint = 0;
    lockdep_assert_held(&mapping.invalidate_lock);
    trace_page_cache_ra_unbounded(mapping.host, index, nr_to_read,
    lookahead_size);
    index = mapping_align_index(mapping, index);
//
// As iterator `i` is aligned to min_nrpages, round_up the
// difference between nr_to_read and lookahead_size to mark the
// index that only has lookahead or "async_region" to set the
// readahead flag.
//
    if (lookahead_size <= nr_to_read) {
    let mut ra_folio_index = 0;
    ra_folio_index = round_up(readahead_index(ractl) +
    nr_to_read - lookahead_size,
    min_nrpages);
    mark = ra_folio_index - index;
    }
    nr_to_read += readahead_index(ractl) - index;
    ractl._index = index;
//
// Preallocate as many pages as we will need.
//
    while (i < nr_to_read) {
    let mut folio = xa_load(&mapping.i_pages, index + i);
    let mut ret = 0;
    if (folio && !xa_is_value(folio)) {
//
// Page already present?  Kick off the current batch
// of contiguous pages before continuing with the
// next batch.  This page may be the one we would
// have intended to mark as Readahead, but we don't
// have a stable reference to this page, and it's
// not worth getting one just for that.
//
    read_pages(ractl);
    ractl._index += min_nrpages;
    i = ractl._index - index;
    continue;
    }
    folio = ractl_alloc_folio(ractl, gfp_mask,
    mapping_min_folio_order(mapping));
    if (!folio) {
    break;
    }
    ret = filemap_add_folio(mapping, folio, index + i, gfp_mask);
    if (ret < 0) {
    folio_put(folio);
    if (ret == -ENOMEM) {
    break;
    }
    read_pages(ractl);
    ractl._index += min_nrpages;
    i = ractl._index - index;
    continue;
    }
    if (i == mark) {
    folio_set_readahead(folio);
    }
    ractl._workingset |= folio_test_workingset(folio);
    ractl._nr_pages += min_nrpages;
    i += min_nrpages;
    }
//
// Now start the IO.  We ignore I/O errors - if the folio is not
// uptodate then the caller will launch read_folio again, and
// will then handle the error.
//
    read_pages(ractl);
    memalloc_nofs_restore(nofs);
    }
    EXPORT_SYMBOL_GPL(page_cache_ra_unbounded);
//
// do_page_cache_ra() actually reads a chunk of disk.  It allocates
// the pages first, then submits them for I/O. This avoids the very bad
// behaviour which would occur if page allocations are causing VM writeback.
// We really don't want to intermingle reads and writes like that.
//
#[no_mangle]
pub unsafe extern "C" fn do_page_cache_ra(ractl: *mut readahead_control, nr_to_read: c_ulong, lookahead_size: c_ulong) {
    let mut mapping = ractl.mapping;
pub static mut index: c_ulong = 0;
pub static mut isize: loff_t = 0;
    let mut end_index;	/* The last page we want to read */
    if (isize == 0) {
    return;
    }
    end_index = (isize - 1) >> PAGE_SHIFT;
    if (index > end_index) {
    return;
    }
// Don't read past the page containing the last byte of the file
    if (nr_to_read > end_index - index) {
    nr_to_read = end_index - index + 1;
// We've reached the end, so don't set a readahead marker.
    lookahead_size = 0;
    }
    filemap_invalidate_lock_shared(mapping);
    page_cache_ra_unbounded(ractl, nr_to_read, lookahead_size);
    filemap_invalidate_unlock_shared(mapping);
    }
//
// Chunk the readahead into 2 megabyte units, so that we don't pin too much
// memory at once.
//
#[no_mangle]
pub unsafe extern "C" fn force_page_cache_ra(ractl: *mut readahead_control, nr_to_read: c_ulong) {
    let mut mapping = ractl.mapping;
    let mut ra = ractl.ra;
    let mut bdi = inode_to_bdi(mapping.host);
    let mut max_pages = 0;
    if (unlikely(!mapping.a_ops.read_folio && !mapping.a_ops.readahead)) {
    return;
    }
//
// If the request exceeds the readahead window, allow the read to
// be up to the optimal hardware IO size
//
    max_pages = max_t(unsigned long, bdi.io_pages, ra.ra_pages);
    nr_to_read = min_t(unsigned long, nr_to_read, max_pages);
    while (nr_to_read) {
pub static mut this_chunk: c_ulong = 0;
    if (this_chunk > nr_to_read) {
    this_chunk = nr_to_read;
    }
    do_page_cache_ra(ractl, this_chunk, 0);
    nr_to_read -= this_chunk;
    }
    }
//
// Set the initial window size, round to next power of 2 and square
// for small size, x 4 for medium, and x 2 for large
// for 128k (32 page) max ra
// 1-2 page = 16k, 3-4 page 32k, 5-8 page = 64k, > 8 page = 128k initial
//
#[no_mangle]
unsafe extern "C" fn get_init_ra_size(size: c_ulong, max: c_ulong) -> c_ulong {
pub static mut newsize: c_ulong = 0;
    if (newsize <= max / 32) {
    newsize = newsize * 4;
    }

    else if (newsize <= max / 4) {
    newsize = newsize * 2;
    }
    else {
    newsize = max;
    }
    return newsize;
    }
//
// Get the previous window size, ramp it up, and
// return it as the new window size.
//
#[no_mangle]
pub unsafe extern "C" fn get_next_ra_size(ra: *mut file_ra_state, max: c_ulong) -> c_ulong {
pub static mut cur: c_ulong = 0;
    if (cur < max / 16) {
    return 4 * cur;
    }
    if (cur <= max / 2) {
    return 2 * cur;
    }
    return max;
    }
//
// On-demand readahead design.
//
// The fields in struct file_ra_state represent the most-recently-executed
// readahead attempt:
//
// |<----- async_size ---------|
// |------------------- size -------------------->|
// |==================#===========================|
// ^start             ^page marked with PG_readahead
//
// To overlap application thinking time and disk I/O time, we do
// `readahead pipelining': Do not wait until the application consumed all
// readahead pages and stalled on the missing page at readahead_index;
// Instead, submit an asynchronous readahead I/O as soon as there are
// only async_size pages left in the readahead window. Normally async_size
// will be equal to size, for maximum pipelining.
//
// In interleaved sequential reads, concurrent streams on the same fd can
// be invalidating each other's readahead state. So we flag the new readahead
// page at (start+size-async_size) with PG_readahead, and use it as readahead
// indicator. The flag won't be set on already cached pages, to avoid the
// readahead-for-nothing fuss, saving pointless page cache lookups.
//
// prev_pos tracks the last visited byte in the _previous_ read request.
// It should be maintained by the caller, and will be used for detecting
// small random reads. Note that the readahead algorithm checks loosely
// for sequential patterns. Hence interleaved reads might be served as
// sequential ones.
//
// There is a special-case: if the first page which the application tries to
// read happens to be the first page of the file, it is assumed that a linear
// read is about to happen and the window is immediately set to the initial size
// based on I/O request size and the max_readahead.
//
// The code ramps up the readahead size aggressively at first, but slow down as
// it approaches max_readahead.
//
#[no_mangle]
pub unsafe extern "C" fn ra_alloc_folio(ractl: *mut readahead_control, index: pgoff_t, mark: pgoff_t, order: c_uint, gfp: gfp_t) -> c_int {
    let mut err = 0;
    let mut folio = ractl_alloc_folio(ractl, gfp, order);
    if (!folio) {
    return -ENOMEM;
    }
    mark = round_down(mark, 1UL << order);
    if (index == mark) {
    folio_set_readahead(folio);
    }
    err = filemap_add_folio(ractl.mapping, folio, index, gfp);
    if (err) {
    folio_put(folio);
    return err;
    }
    ractl._nr_pages += 1UL << order;
    ractl._workingset |= folio_test_workingset(folio);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn page_cache_ra_order(ractl: *mut readahead_control, ra: *mut file_ra_state) {
    let mut mapping = ractl.mapping;
pub static mut start: pgoff_t = 0;
pub static mut index: pgoff_t = 0;
pub static mut min_order: c_uint = 0;
pub static mut limit: pgoff_t = 0;
    let mut mark;
    let mut nofs = 0;
pub static mut err: c_int = 0;
pub static mut gfp: gfp_t = 0;
pub static mut new_order: c_uint = 0;
    trace_page_cache_ra_order(mapping.host, start, ra);
    if (!mapping_large_folio_support(mapping)) {
    ra.order = 0;
// goto;
    }
    if (limit > index + ra.size - 1) {
    limit = index + ra.size - 1;
    mark = index + ra.size - ra.async_size;
    } else {
// We've reached the end, so don't set a readahead marker.
    mark = ULONG_MAX;
    }
    new_order = min(mapping_max_folio_order(mapping), new_order);
    new_order = min_t(unsigned int, new_order, ilog2(ra.size));
    new_order = max(new_order, min_order);
    ra.order = new_order;
// See comment in page_cache_ra_unbounded()
    nofs = memalloc_nofs_save();
    filemap_invalidate_lock_shared(mapping);
//
// If the new_order is greater than min_order and index is
// already aligned to new_order, then this will be noop as index
// aligned to new_order should also be aligned to min_order.
//
    ractl._index = mapping_align_index(mapping, index);
    index = readahead_index(ractl);
    while (index <= limit) {
pub static mut order: c_uint = 0;
// Align with smaller pages if needed
    if (index & ((1UL << order) - 1)) {
    order = __ffs(index);
    }
// Don't allocate pages past EOF
    while (order > min_order && index + (1UL << order) - 1 > limit) {
    order -= 1;
    }
    err = ra_alloc_folio(ractl, index, mark, order, gfp);
    if (err) {
    break;
    }
    index += 1UL << order;
    }
    read_pages(ractl);
    filemap_invalidate_unlock_shared(mapping);
    memalloc_nofs_restore(nofs);
//
// If there were already pages in the page cache, then we may have
// left some gaps.  Let the regular readahead code take care of this
// situation below.
//
    if (!err) {
    return;
    }
// label;
//
// ->readahead() may have updated readahead window size so we have to
// check there's still something to read.
//
    if (ra.size > index - start) {
    do_page_cache_ra(ractl, ra.size - (index - start),
    ra.async_size);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn ractl_max_pages(ractl: *mut readahead_control, req_size: c_ulong) -> c_ulong {
    let mut bdi = inode_to_bdi(ractl.mapping.host);
pub static mut max_pages: c_ulong = 0;
//
// If the request exceeds the readahead window, allow the read to
// be up to the optimal hardware IO size
//
    if (req_size > max_pages && bdi.io_pages > max_pages) {
    max_pages = min(req_size, bdi.io_pages);
    }
    return max_pages;
    }
#[no_mangle]
pub unsafe extern "C" fn page_cache_sync_ra(ractl: *mut readahead_control, req_count: c_ulong) {
pub static mut index: pgoff_t = 0;
pub static mut do_forced_ra: bool = false;
    let mut ra = ractl.ra;
    unsigned long max_pages, contig_count;
    pgoff_t prev_index, miss;
    trace_page_cache_sync_ra(ractl.mapping.host, index, ra, req_count);
//
// Even if readahead is disabled, issue this request as readahead
// as we'll need it to satisfy the requested range. The forced
// readahead will do the right thing and limit the read to just the
// requested range, which we'll set to 1 page for this case.
//
    if (!ra.ra_pages || blk_cgroup_congested()) {
    if (!ractl.file) {
    return;
    }
    req_count = 1;
    do_forced_ra = true;
    }
// be dumb
    if (do_forced_ra) {
    force_page_cache_ra(ractl, req_count);
    return;
    }
    max_pages = ractl_max_pages(ractl, req_count);
    prev_index = (unsigned long long)ra.prev_pos >> PAGE_SHIFT;
//
// A start of file, oversized read, or sequential cache miss:
// trivial case: (index - prev_index) == 1
// unaligned reads: (index - prev_index) == 0
//
    if (!index || req_count > max_pages || index - prev_index <= 1UL) {
    ra.start = index;
    ra.size = get_init_ra_size(req_count, max_pages);
    ra.async_size = ra.size > req_count ? ra.size - req_count :
    ra.size >> 1;
// goto;
    }
//
// Query the page cache and look for the traces(cached history pages)
// that a sequential stream would leave behind.
//
    rcu_read_lock();
    miss = page_cache_prev_miss(ractl.mapping, index - 1, max_pages);
    rcu_read_unlock();
    contig_count = index - miss - 1;
//
// Standalone, small random read. Read as is, and do not pollute the
// readahead state.
//
    if (contig_count <= req_count) {
    do_page_cache_ra(ractl, req_count, 0);
    return;
    }
//
// File cached from the beginning:
// it is a strong indication of long-run stream (or whole-file-read)
//
    if (miss == ULONG_MAX) {
    contig_count *= 2;
    }
    ra.start = index;
    ra.size = min(contig_count + req_count, max_pages);
    ra.async_size = 1;
// label;
    ra.order = 0;
    ractl._index = ra.start;
    page_cache_ra_order(ractl, ra);
    }
    EXPORT_SYMBOL_GPL(page_cache_sync_ra);
#[no_mangle]
pub unsafe extern "C" fn page_cache_async_ra(ractl: *mut readahead_control, folio: *mut folio, req_count: c_ulong) {
    let mut max_pages = 0;
    let mut ra = ractl.ra;
pub static mut index: pgoff_t = 0;
    pgoff_t expected, start, end, aligned_end, align;
// no readahead
    if (!ra.ra_pages) {
    return;
    }
//
// Same bit is used for PG_readahead and PG_reclaim.
//
    if (folio_test_writeback(folio)) {
    return;
    }
    trace_page_cache_async_ra(ractl.mapping.host, index, ra, req_count);
    folio_clear_readahead(folio);
    if (blk_cgroup_congested()) {
    return;
    }
    max_pages = ractl_max_pages(ractl, req_count);
//
// It's the expected callback index, assume sequential access.
// Ramp up sizes, and push forward the readahead window.
//
    expected = round_down(ra.start + ra.size - ra.async_size,
    folio_nr_pages(folio));
    if (index == expected) {
    ra.start += ra.size;
//
// In the case of MADV_HUGEPAGE, the actual size might exceed
// the readahead window.
//
    ra.size = max(ra.size, get_next_ra_size(ra, max_pages));
// goto;
    }
//
// Hit a marked folio without valid readahead state.
// E.g. interleaved reads.
// Query the pagecache for async_size, which normally equals to
// readahead size. Ramp it up and use it as the new readahead size.
//
    rcu_read_lock();
    start = page_cache_next_miss(ractl.mapping, index + 1, max_pages);
    rcu_read_unlock();
    if (!start || start - index > max_pages) {
    return;
    }
    ra.start = start;
    ra.size = start - index;	/* old async_size */
    ra.size += req_count;
    ra.size = get_next_ra_size(ra, max_pages);
// label;
    ra.order += 2;
    align = 1UL << min(ra.order, ffs(max_pages) - 1);
    end = ra.start + ra.size;
    aligned_end = round_down(end, align);
    if (aligned_end > ra.start) {
    ra.size -= end - aligned_end;
    }
    ra.async_size = ra.size;
    ractl._index = ra.start;
    page_cache_ra_order(ractl, ra);
    }
    EXPORT_SYMBOL_GPL(page_cache_async_ra);
#[no_mangle]
pub unsafe extern "C" fn ksys_readahead(fd: c_int, offset: loff_t, count: usize) -> isize {
pub static mut file: *mut c_void = core::ptr::null_mut();
pub static mut inode: *mut c_void = core::ptr::null_mut();
    CLASS(fd, f)(fd);
    if (fd_empty(f)) {
    return -EBADF;
    }
    file = fd_file(f);
    if (!(file.f_mode & FMODE_READ)) {
    return -EBADF;
    }
//
// The readahead() syscall is intended to run only on files
// that can execute readahead. If readahead is not possible
// on this file, then we must return -EINVAL.
//
    if (!file.f_mapping) {
    return -EINVAL;
    }
    if (!file.f_mapping.a_ops) {
    return -EINVAL;
    }
    inode = file_inode(file);
    if (!S_ISREG(inode.i_mode) && !S_ISBLK(inode.i_mode)) {
    return -EINVAL;
    }
    if (IS_ANON_FILE(inode)) {
    return -EINVAL;
    }
    return vfs_fadvise(fd_file(f), offset, count, POSIX_FADV_WILLNEED);
    }
#[no_mangle]
pub unsafe extern "C" fn sys_readahead(fd: usize, offset: usize, count: usize) -> c_long {
    return ksys_readahead(fd, offset, count);
    }

    COMPAT_SYSCALL_DEFINE4(readahead, int, fd, compat_arg_u64_dual(offset), size_t, count)
    {
    return ksys_readahead(fd, compat_arg_u64_glue(offset), count);
    }

//
// readahead_expand - Expand a readahead request
// @ractl: The request to be expanded
// @new_start: The revised start
// @new_len: The revised size of the request
//
// Attempt to expand a readahead request outwards from the current size to the
// specified size by inserting locked pages before and after the current window
// to increase the size to the new window.  This may involve the insertion of
// THPs, in which case the window may get expanded even beyond what was
// requested.
//
// The algorithm will stop if it encounters a conflicting page already in the
// pagecache and leave a smaller expansion than requested.
//
// The caller must check for this by examining the revised @ractl object for a
// different expansion than was requested.
//
#[no_mangle]
pub unsafe extern "C" fn readahead_expand(ractl: *mut readahead_control, new_start: loff_t, new_len: size_t) {
    let mut mapping = ractl.mapping;
    let mut ra = ractl.ra;
    pgoff_t new_index, new_nr_pages;
pub static mut gfp_mask: gfp_t = 0;
pub static mut min_nrpages: c_ulong = 0;
pub static mut min_order: c_uint = 0;
    new_index = new_start / PAGE_SIZE;
//
// Readahead code should have aligned the ractl->_index to
// min_nrpages before calling readahead aops.
//
    VM_BUG_ON(!IS_ALIGNED(ractl._index, min_nrpages));
// Expand the leading edge downwards
    while (ractl._index > new_index) {
pub static mut index: c_ulong = 0;
    let mut folio = xa_load(&mapping.i_pages, index);
    if (folio && !xa_is_value(folio)) {
    return; /* Folio apparently present */
    }
    folio = ractl_alloc_folio(ractl, gfp_mask, min_order);
    if (!folio) {
    return;
    }
    index = mapping_align_index(mapping, index);
    if (filemap_add_folio(mapping, folio, index, gfp_mask) < 0) {
    folio_put(folio);
    return;
    }
    if (unlikely(folio_test_workingset(folio)) &&
    !ractl._workingset) {
    ractl._workingset = true;
    psi_memstall_enter(&ractl._pflags);
    }
    ractl._nr_pages += min_nrpages;
    ractl._index = folio.index;
    }
    new_len += new_start - readahead_pos(ractl);
    new_nr_pages = DIV_ROUND_UP(new_len, PAGE_SIZE);
// Expand the trailing edge upwards
    while (ractl._nr_pages < new_nr_pages) {
pub static mut index: c_ulong = 0;
    let mut folio = xa_load(&mapping.i_pages, index);
    if (folio && !xa_is_value(folio)) {
    return; /* Folio apparently present */
    }
    folio = ractl_alloc_folio(ractl, gfp_mask, min_order);
    if (!folio) {
    return;
    }
    index = mapping_align_index(mapping, index);
    if (filemap_add_folio(mapping, folio, index, gfp_mask) < 0) {
    folio_put(folio);
    return;
    }
    if (unlikely(folio_test_workingset(folio)) &&
    !ractl._workingset) {
    ractl._workingset = true;
    psi_memstall_enter(&ractl._pflags);
    }
    ractl._nr_pages += min_nrpages;
    if (ra) {
    ra.size += min_nrpages;
    ra.async_size += min_nrpages;
    }
    }
    }
    EXPORT_SYMBOL(readahead_expand);