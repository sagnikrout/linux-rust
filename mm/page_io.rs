//! Automatically rewritten from C to Rust
//! Source: mm/page_io.c
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
// linux/mm/page_io.c
//
// Copyright (C) 1991, 1992, 1993, 1994  Linus Torvalds
//
// Swap reorganised 29.12.95,
// Asynchronous swapping added 30.12.95. Stephen Tweedie
// Removed race in async swapping. 14.4.1996. Bruno Haible
// Add swap of shared pages through the page cache. 20.2.1998. Stephen Tweedie
// Always use brw_page, life becomes simpler. 12 May 1998 Eric Biederman
//

#[no_mangle]
pub unsafe extern "C" fn generic_swapfile_activate(sis: *mut swap_info_struct, swap_file: *mut file, span: *mut sector_t) -> c_int {
    let mut mapping = swap_file.f_mapping;
    let mut inode = mapping.host;
    let mut blocks_per_page: c_uint = 0;
    let mut page_no = 0;
    let mut blkbits: c_uint = 0;
    let mut probe_block;
    let mut last_block;
pub static mut lowest_block: sector_t = 0;
pub static mut highest_block: sector_t = 0;
pub static mut nr_extents: c_int = 0;
    let mut ret = 0;
    blkbits = inode.i_blkbits;
    blocks_per_page = PAGE_SIZE >> blkbits;
//
// Map all the blocks into the extent tree.  This code doesn't try
// to be very smart.
//
    probe_block = 0;
    page_no = 0;
    last_block = i_size_read(inode) >> blkbits;
    while ((probe_block + blocks_per_page) <= last_block &&
    page_no < sis.max) {
    let mut block_in_page: c_uint = 0;
    let mut first_block;
    cond_resched();
    first_block = probe_block;
    ret = bmap(inode, &first_block);
    if (ret || !first_block) {
// goto;
    }
//
// It must be PAGE_SIZE aligned on-disk
//
    if (first_block & (blocks_per_page - 1)) {
    probe_block += 1;
// goto;
    }
    while (block_in_page < blocks_per_page) {
    let mut block;
    block = probe_block + block_in_page;
    ret = bmap(inode, &block);
    if (ret || !block) {
// goto;
    }
    if (block != first_block + block_in_page) {
// Discontiguity
    probe_block += 1;
// goto;
    }
    }
    first_block >>= (PAGE_SHIFT - blkbits);
    if (page_no) {	/* exclude the header page */ {
    if (first_block < lowest_block)
    lowest_block = first_block;
    }
    if (first_block > highest_block) {
    highest_block = first_block;
    }
    }
//
// We found a PAGE_SIZE-length, PAGE_SIZE-aligned run of blocks
//
    ret = add_swap_extent(sis, page_no, 1, first_block);
    if (ret < 0) {
// goto;
    }
    nr_extents += ret;
    page_no += 1;
    probe_block += blocks_per_page;
// label;
    continue;
    }
    ret = nr_extents;
// span = 1 + highest_block - lowest_block;
    if (page_no == 0) {
    page_no = 1;	/* force Empty message */
    }
    sis.max = page_no;
    sis.pages = page_no - 1;
// label;
    return ret;
// label;
    pr_err!("swapon: swapfile has holes\n");
    ret = -EINVAL;
// goto;
    }
#[no_mangle]
unsafe extern "C" fn is_folio_zero_filled(folio: *mut folio) -> bool {
    let mut pos = 0;
    let mut last_pos = 0;
pub static mut data: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    last_pos = PAGE_SIZE / sizeof!(*data) - 1;
    while (i < folio_nr_pages(folio)) {
    data = kmap_local_folio(folio, i * PAGE_SIZE);
//
// Check last word first, incase the page is zero-filled at
// the start and has non-zero data at the end, which is common
// in real-world workloads.
//
    if (data[last_pos]) {
    kunmap_local(data);
    return false;
    }
    while (pos < last_pos) {
    if (data[pos]) {
    kunmap_local(data);
    return false;
    }
    }
    kunmap_local(data);
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn swap_zeromap_folio_set(folio: *mut folio) {
    let mut objcg = get_obj_cgroup_from_folio(folio);
pub static mut nr_pages: c_int = 0;
pub static mut ci: *mut c_void = core::ptr::null_mut();
    let mut entry;
    let mut i = 0;
    VM_WARN_ON_ONCE_FOLIO(!folio_test_swapcache(folio), folio);
    VM_WARN_ON_ONCE_FOLIO(!folio_test_locked(folio), folio);
    ci = swap_cluster_get_and_lock(folio);
    while (i < folio_nr_pages(folio)) {
    entry = page_swap_entry(folio_page(folio, i));
    __swap_table_set_zero(ci, swp_cluster_offset(entry));
    }
    swap_cluster_unlock(ci);
    count_vm_events(SWPOUT_ZERO, nr_pages);
    if (objcg) {
    count_objcg_events(objcg, SWPOUT_ZERO, nr_pages);
    obj_cgroup_put(objcg);
    }
    }
#[no_mangle]
unsafe extern "C" fn swap_zeromap_folio_clear(folio: *mut folio) {
pub static mut ci: *mut c_void = core::ptr::null_mut();
    let mut entry;
    let mut i = 0;
    VM_WARN_ON_ONCE_FOLIO(!folio_test_swapcache(folio), folio);
    VM_WARN_ON_ONCE_FOLIO(!folio_test_locked(folio), folio);
    ci = swap_cluster_get_and_lock(folio);
    while (i < folio_nr_pages(folio)) {
    entry = page_swap_entry(folio_page(folio, i));
    __swap_table_clear_zero(ci, swp_cluster_offset(entry));
    }
    swap_cluster_unlock(ci);
    }
//
// We may have stale swap cache pages in memory: notice
// them here and get rid of the unnecessary final write.
//
#[no_mangle]
pub unsafe extern "C" fn swap_writeout(ctx: *mut swap_io_ctx, folio: *mut folio) -> c_int {
pub static mut ret: c_int = 0;
    if (folio_free_swap(folio)) {
// goto;
    }
//
// Arch code may have to preserve more data than just the page
// contents, e.g. memory tags.
//
    ret = arch_prepare_to_swap(folio);
    if (ret) {
    folio_mark_dirty(folio);
// goto;
    }
//
// Use the swap table zero mark to avoid doing IO for zero-filled
// pages. The zero mark is protected by the cluster lock, which is
// acquired internally by swap_zeromap_folio_set/clear.
//
    if (is_folio_zero_filled(folio)) {
    swap_zeromap_folio_set(folio);
// goto;
    }
//
// Clear bits this folio occupies in the zeromap to prevent zero data
// being read in from any previous zero writes that occupied the same
// swap entries.
//
    swap_zeromap_folio_clear(folio);
    if (zswap_store(folio)) {
    count_mthp_stat(folio_order(folio), MTHP_STAT_ZSWPOUT);
// goto;
    }
    rcu_read_lock();
    if (!mem_cgroup_zswap_writeback_enabled(folio_memcg(folio))) {
    rcu_read_unlock();
    folio_mark_dirty(folio);
    return AOP_WRITEPAGE_ACTIVATE;
    }
    rcu_read_unlock();
    __swap_writepage(ctx, folio);
    return 0;
// label;
    folio_unlock(folio);
    return ret;
    }

#[no_mangle]
pub unsafe extern "C" fn folio_memcg_blkg_css(folio: *mut folio) -> *mut c_void {
    return cgroup_e_css(folio_memcg(folio).css.cgroup, &io_cgrp_subsys);
    }
#[no_mangle]
unsafe extern "C" fn folio_blkg_can_merge(folio: *mut folio, prev_folio: *mut folio) -> bool {
pub static mut can_merge: bool = true;
    if (folio_memcg_charged(folio) != folio_memcg_charged(prev_folio)) {
    return false;
    }
    if (folio_memcg_charged(folio)) {
    rcu_read_lock();
    if (folio_memcg_blkg_css(folio) !=
    folio_memcg_blkg_css(prev_folio)) {
    can_merge = false;
    }
    rcu_read_unlock();
    }
    return can_merge;
    }
#[no_mangle]
unsafe extern "C" fn bio_associate_blkg_from_page(bio: *mut bio, folio: *mut folio) {
pub static mut css: *mut c_void = core::ptr::null_mut();
    if (!folio_memcg_charged(folio)) {
    return;
    }
    rcu_read_lock();
    css = folio_memcg_blkg_css(folio);
    if (css && !css_tryget(css)) {
    css = core::ptr::null_mut();
    }
    rcu_read_unlock();
    bio_associate_blkg_from_css(bio, css);
    if (css) {
    css_put(css);
    }
    }

#[no_mangle]
unsafe extern "C" fn folio_blkg_can_merge(folio: *mut folio, prev_folio: *mut folio) -> bool {
    return true;
    }

pub static mut sio_pool: *mut c_void = core::ptr::null_mut();
#[no_mangle]
pub unsafe extern "C" fn sio_pool_init() -> c_int {
    if (!sio_pool) {
    let mut pool = mempool_create_kmalloc_pool(
    SWAP_CLUSTER_MAX, sizeof!(swap_iocb));
    if (cmpxchg(&sio_pool, core::ptr::null_mut(), pool)) {
    mempool_destroy(pool);
    }
    }
    if (!sio_pool) {
    return -ENOMEM;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn swap_can_merge(ctx: *mut swap_io_ctx, folio: *mut folio, rw: c_int) -> bool {
    let mut sis = __swap_entry_to_info(folio.swap);
    let mut last_bv = &ctx.sio.bvecs[ctx.sio.nr_bvecs - 1];
    let mut prev_folio = bvec_folio(last_bv);
pub static mut prev_folio_size: usize = 0;
    if (ctx.sis != sis) {
    return false;
    }
    return sis.ops.can_merge(folio, prev_folio, prev_folio_size, rw);
    }
#[no_mangle]
unsafe extern "C" fn swap_add_folio(ctx: *mut swap_io_ctx, folio: *mut folio, rw: c_int) {
    let mut sis = __swap_entry_to_info(folio.swap);
    let mut sio = ctx.sio;
    if (sio && !swap_can_merge(ctx, folio, rw)) {
    if (rw == WRITE) {
    swap_write_submit(ctx);
    }
    else {
    swap_read_submit(ctx);
    }
    sio = ctx.sio;
    }
    if (!sio) {
    ctx.sis = sis;
    ctx.sio = sio = mempool_alloc(sio_pool, GFP_NOIO);
    sio.nr_bvecs = 0;
    sio.len = 0;
    }
    bvec_set_folio(&sio.bvecs[sio.nr_bvecs], folio, folio_size(folio), 0);
    sio.len += folio_size(folio);
//
// Write out the iocb if we filled it, or if the device is synchronous.
//
// The latter is to work around expectations in the classic LRU code
// which make synchronous clearing of the folio writeback flag in the
// reclaim path beneficial.
//
    if (++sio.nr_bvecs == ARRAY_SIZE!(sio.bvecs) ||
    (rw == WRITE && (sis.flags & SWP_SYNCHRONOUS_IO))) {
    if (rw == WRITE) {
    swap_write_submit(ctx);
    }
    else {
    swap_read_submit(ctx);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __swap_writepage(ctx: *mut swap_io_ctx, folio: *mut folio) {
    VM_BUG_ON_FOLIO(!folio_test_swapcache(folio), folio);

    if (unlikely(folio_test_pmd_mappable(folio))) {
    count_memcg_folio_events(folio, THP_SWPOUT, 1);
    count_vm_event(THP_SWPOUT);
    }

    count_mthp_stat(folio_order(folio), MTHP_STAT_SWPOUT);
    count_memcg_folio_events(folio, PSWPOUT, folio_nr_pages(folio));
    count_vm_events(PSWPOUT, folio_nr_pages(folio));
    folio_start_writeback(folio);
    folio_unlock(folio);
    swap_add_folio(ctx, folio, WRITE);
    }
//
// Return the count of contiguous swap entries that share the same
// zeromap status as the starting entry. If is_zerop is not NULL,
// it will return the zeromap status of the starting entry.
//
// Context: Caller must ensure the cluster containing the entries
// that will be checked won't be freed.
//
#[no_mangle]
pub unsafe extern "C" fn swap_zeromap_batch(entry: swp_entry_t, max_nr: c_int, is_zerop: *mut bool) -> c_int {
    let mut i = 0;
    let mut is_zero = 0;
pub static mut ci_start: c_uint = 0;
    let mut ci = __swap_entry_to_cluster(entry);
    VM_WARN_ON_ONCE(ci_start + max_nr > SWAPFILE_CLUSTER);
    rcu_read_lock();
    is_zero = __swap_table_test_zero(ci, ci_start);
    for (i = 1; i < max_nr; i++) {
    if (is_zero != __swap_table_test_zero(ci, ci_start + i))
    break;
    }
    rcu_read_unlock();
    if (is_zerop) {
// is_zerop = is_zero;
    }
    return i;
    }
#[no_mangle]
unsafe extern "C" fn swap_read_folio_zeromap(folio: *mut folio) -> bool {
pub static mut nr_pages: c_int = 0;
pub static mut objcg: *mut c_void = core::ptr::null_mut();
    let mut is_zeromap = 0;
    VM_WARN_ON_ONCE_FOLIO(!folio_test_locked(folio), folio);
//
// Swapping in a large folio that is partially in the zeromap is not
// currently handled. Return true without marking the folio uptodate so
// that an IO error is emitted (e.g. do_swap_page() will sigbus).
// Folio lock stabilizes the cluster and map, so the check is safe.
//
    if (WARN_ON_ONCE!(swap_zeromap_batch(folio.swap, nr_pages,
    &is_zeromap) != nr_pages)) {
    return true;
    }
    if (!is_zeromap) {
    return false;
    }
    objcg = get_obj_cgroup_from_folio(folio);
    count_vm_events(SWPIN_ZERO, nr_pages);
    if (objcg) {
    count_objcg_events(objcg, SWPIN_ZERO, nr_pages);
    obj_cgroup_put(objcg);
    }
    folio_zero_range(folio, 0, folio_size(folio));
    folio_mark_uptodate(folio);
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn swap_read_folio(ctx: *mut swap_io_ctx, folio: *mut folio) {
    let mut sis = __swap_entry_to_info(folio.swap);
pub static mut synchronous: bool = false;
pub static mut workingset: bool = false;
    let mut pflags = 0;
    let mut in_thrashing = 0;
    VM_BUG_ON_FOLIO(!folio_test_swapcache(folio) && !synchronous, folio);
    VM_BUG_ON_FOLIO(!folio_test_locked(folio), folio);
    VM_BUG_ON_FOLIO(folio_test_uptodate(folio), folio);
//
// Count submission time as memory stall and delay. When the device
// is congested, or the submitting cgroup IO-throttled, submission
// can be a significant part of overall IO time.
//
    if (workingset) {
    delayacct_thrashing_start(&in_thrashing);
    psi_memstall_enter(&pflags);
    }
    delayacct_swapin_start();
    if (swap_read_folio_zeromap(folio)) {
    folio_unlock(folio);
// goto;
    }
    if (zswap_load(folio) != -ENOENT) {
// goto;
    }
// We have to read from slower devices. Increase zswap protection.
    zswap_folio_swapin(folio);
    swap_add_folio(ctx, folio, READ);
// label;
    if (workingset) {
    delayacct_thrashing_end(&in_thrashing);
    psi_memstall_leave(&pflags);
    }
    delayacct_swapin_end();
    }
#[no_mangle]
unsafe extern "C" fn swap_write_end(sio: *mut swap_iocb, failed: bool) {
    let mut p = 0;
    while (p < sio.nr_bvecs) {
    let mut page = sio.bvecs[p].bv_page;
    if (failed) {
    set_page_dirty(page);
    ClearPageReclaim(page);
    }
    end_page_writeback(page);
    }
    mempool_free(sio, sio_pool);
    }
#[no_mangle]
unsafe extern "C" fn swap_fs_write_complete(iocb: *mut kiocb, ret: c_long) {
    let mut sio = container_of!(iocb, swap_iocb, iocb);
pub static mut failed: bool = false;
    if (failed) {
    let mut page = sio.bvecs[0].bv_page;
//
// In the case of swap-over-nfs, this can be a temporary failure
// if the system has limited memory for allocating transmit
// buffers.  Mark the page dirty and avoid
// folio_rotate_reclaimable but rate-limit the messages.
//
    pr_err_ratelimited("Write error %ld on dio swapfile (%llu)\n",
    ret, swap_dev_pos(page_swap_entry(page)));
    }
    swap_write_end(sio, failed);
    }
#[no_mangle]
unsafe extern "C" fn end_swap_bio_write(bio: *mut bio) {
    let mut sio = container_of!(bio, swap_iocb, bio);
pub static mut failed: bool = false;
    if (failed) {
    pr_alert_ratelimited("Write-error on swap-device (%u:%u:%llu)\n",
    MAJOR(bio_dev(bio)), MINOR(bio_dev(bio)),
    (unsigned long long)bio.bi_iter.bi_sector);
    }
    bio_uninit(bio);
    swap_write_end(sio, failed);
    }
#[no_mangle]
unsafe extern "C" fn swap_read_end(sio: *mut swap_iocb, failed: bool) {
    let mut p = 0;
    while (p < sio.nr_bvecs) {
    let mut folio = bvec_folio(&sio.bvecs[p]);
    if (!failed) {
    count_mthp_stat(folio_order(folio), MTHP_STAT_SWPIN);
    count_memcg_folio_events(folio, PSWPIN,
    folio_nr_pages(folio));
    folio_mark_uptodate(folio);
    }
    folio_unlock(folio);
    }
    if (!failed) {
    count_vm_events(PSWPIN, sio.len >> PAGE_SHIFT);
    }
    mempool_free(sio, sio_pool);
    }
#[no_mangle]
unsafe extern "C" fn swap_fs_read_complete(iocb: *mut kiocb, ret: c_long) {
    let mut sio = container_of!(iocb, swap_iocb, iocb);
pub static mut failed: bool = false;
    if (failed) {
    pr_alert_ratelimited("Read-error on swap-device\n");
    }
    swap_read_end(sio, failed);
    }
#[no_mangle]
unsafe extern "C" fn swap_bio_read_end_io(bio: *mut bio) {
    let mut sio = container_of!(bio, swap_iocb, bio);
pub static mut failed: bool = false;
    if (failed) {
    pr_alert_ratelimited("Read-error on swap-device (%u:%u:%llu)\n",
    MAJOR(bio_dev(bio)), MINOR(bio_dev(bio)),
    (unsigned long long)bio.bi_iter.bi_sector);
    }
    bio_uninit(bio);
    swap_read_end(sio, failed);
    }
#[no_mangle]
unsafe extern "C" fn swap_bdev_submit_write(ctx: *mut swap_io_ctx) {
    let mut sio = ctx.sio;
    let mut bio = &sio.bio;
    bio_init(bio, ctx.sis.bdev, sio.bvecs, ARRAY_SIZE!(sio.bvecs),
    REQ_OP_WRITE | REQ_SWAP);
    bio.bi_iter.bi_size = sio.len;
    bio.bi_iter.bi_sector = swap_folio_sector(bio_first_folio_all(bio));
    bio_associate_blkg_from_page(bio, bio_first_folio_all(bio));
    if (ctx.sis.flags & SWP_SYNCHRONOUS_IO) {
    submit_bio_wait(bio);
    end_swap_bio_write(bio);
    } else {
    bio.bi_end_io = end_swap_bio_write;
    submit_bio(bio);
    }
    }
#[no_mangle]
unsafe extern "C" fn swap_bdev_submit_read(ctx: *mut swap_io_ctx) {
    let mut sio = ctx.sio;
    let mut bio = &sio.bio;
    bio_init(bio, ctx.sis.bdev, sio.bvecs, ARRAY_SIZE!(sio.bvecs),
    REQ_OP_READ);
    bio.bi_iter.bi_size = sio.len;
    bio.bi_iter.bi_sector = swap_folio_sector(bio_first_folio_all(bio));
    if (ctx.sis.flags & SWP_SYNCHRONOUS_IO) {
//
// Keep this task valid during swap readpage because the oom
// killer may attempt to access it in the page fault retry
// time check.
//
    get_task_struct(current);
    submit_bio_wait(bio);
    swap_bio_read_end_io(bio);
    put_task_struct(current);
    } else {
    bio.bi_end_io = swap_bio_read_end_io;
    submit_bio(bio);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn swap_bdev_can_merge(folio: *mut folio, prev_folio: *mut folio, prev_folio_size: size_t, rw: c_int) -> bool {
    if (swap_folio_sector(folio) !=
    swap_folio_sector(prev_folio) + (prev_folio_size >> SECTOR_SHIFT)) {
    return false;
    }
    if (rw == WRITE && !folio_blkg_can_merge(folio, prev_folio)) {
    return false;
    }
    return true;
    }
pub static mut swap_ops: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn swap_fs_prepare_rw(ctx: *mut swap_io_ctx, rw: c_int, iter: *mut iov_iter) {
    let mut sio = ctx.sio;
    init_sync_kiocb(&sio.iocb, ctx.sis.swap_file);
    sio.iocb.ki_pos = swap_dev_pos(bvec_folio(&sio.bvecs[0]).swap);
    if (rw == WRITE) {
    sio.iocb.ki_complete = swap_fs_write_complete;
    }
    else {
    sio.iocb.ki_complete = swap_fs_read_complete;
    }
    iov_iter_bvec(iter, rw == WRITE ? ITER_SOURCE : ITER_DEST,
    sio.bvecs, sio.nr_bvecs, sio.len);
    }
    EXPORT_SYMBOL_GPL(swap_fs_prepare_rw);
#[no_mangle]
pub unsafe extern "C" fn swap_fs_can_merge(folio: *mut folio, prev_folio: *mut folio, prev_folio_size: size_t, rw: c_int) -> bool {
    return swap_dev_pos(folio.swap) ==
    swap_dev_pos(prev_folio.swap) + prev_folio_size;
    }
    EXPORT_SYMBOL_GPL(swap_fs_can_merge);
#[no_mangle]
pub unsafe extern "C" fn swap_fs_activate(sis: *mut swap_info_struct, ops: *const swap_ops) -> c_int {
    sis.ops = ops;
    return add_swap_extent(sis, 0, sis.max, 0);
    }
    EXPORT_SYMBOL_GPL(swap_fs_activate);
#[no_mangle]
pub unsafe extern "C" fn swap_write_submit(ctx: *mut swap_io_ctx) {
    if (!ctx.sio) {
    return;
    }
    count_vm_events(NRSWPOUT, 1);
    ctx.sis.ops.submit_write(ctx);
    ctx.sio = core::ptr::null_mut();
    ctx.sis = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn swap_read_submit(ctx: *mut swap_io_ctx) {
    if (!ctx.sio) {
    return;
    }
    count_vm_events(NRSWPIN, 1);
    ctx.sis.ops.submit_read(ctx);
    ctx.sio = core::ptr::null_mut();
    ctx.sis = core::ptr::null_mut();
    }