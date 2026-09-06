//! Automatically rewritten from C to Rust
//! Source: mm/swap_state.c
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
// linux/mm/swap_state.c
//
// Copyright (C) 1991, 1992, 1993, 1994  Linus Torvalds
// Swap reorganised 29.12.95, Stephen Tweedie
//
// Rewritten to use page cache, (C) 1998 Stephen Tweedie
//

// Swap readahead cluster size, as a power of 2 pages.
    static int page_cluster;
pub static mut page_cluster_max: int = 31;
//
// swapper_space is a fiction, retained to simplify the path through
// vmscan's shrink_folio_list.
//
pub static mut address_space_operations: usize = 0;
    struct address_space swap_space  = {
    .a_ops = &swap_aops,
    };
pub static mut : bool enable_vma_readahead = true;
pub const SWAP_RA_ORDER_CEILING: c_int = 5;

    (((addr) & PAGE_MASK) |					
    (((win) << SWAP_RA_WIN_SHIFT) & SWAP_RA_WIN_MASK) |	
    ((hits) & SWAP_RA_HITS_MASK))
// Initial readahead hits is 4 to start up with a small window

    (atomic_long_read(&(vma).swap_readahead_info) ? : 4)
pub static mut swapin_readahead_hits: atomic_t = 0;
#[no_mangle]
pub unsafe extern "C" fn show_swap_cache_info() {
    printk("%lu pages in swap cache\n", total_swapcache_pages());
    printk("Free swap  = %ldkB\n", K(get_nr_swap_pages()));
    printk("Total swap = %lukB\n", K(total_swap_pages));
    }
//
// swap_cache_get_folio - Looks up a folio in the swap cache.
// @entry: swap entry used for the lookup.
//
// A found folio will be returned unlocked and with its refcount increased.
//
// Context: Caller must ensure @entry is valid and protect the swap device
// with reference count or locks.
// Return: Returns the found folio on success, NULL otherwise. The caller
// must lock and check if the folio still matches the swap entry before
// use (e.g., folio_matches_swap_entry).
//
#[no_mangle]
pub unsafe extern "C" fn swap_cache_get_folio(entry: swp_entry_t) -> *mut c_void {
    let mut swp_tb = 0;
pub static mut folio: *mut c_void = core::ptr::null_mut();
    for (;;) {
    swp_tb = swap_table_get(__swap_entry_to_cluster(entry),
    swp_cluster_offset(entry));
    if (!swp_tb_is_folio(swp_tb)) {
    return core::ptr::null_mut();
    }
    folio = swp_tb_to_folio(swp_tb);
    if (likely(folio_try_get(folio))) {
    return folio;
    }
    }
    return core::ptr::null_mut();
    }
//
// swap_cache_has_folio - Check if a swap slot has cache.
// @entry: swap entry indicating the slot.
//
// Context: Caller must ensure @entry is valid and protect the swap
// device with reference count or locks.
//
#[no_mangle]
pub unsafe extern "C" fn swap_cache_has_folio(entry: swp_entry_t) -> bool {
    let mut swp_tb = 0;
    swp_tb = swap_table_get(__swap_entry_to_cluster(entry),
    swp_cluster_offset(entry));
    return swp_tb_is_folio(swp_tb);
    }
//
// swap_cache_get_shadow - Looks up a shadow in the swap cache.
// @entry: swap entry used for the lookup.
//
// Context: Caller must ensure @entry is valid and protect the swap device
// with reference count or locks.
// Return: Returns either NULL or an XA_VALUE (shadow).
//
#[no_mangle]
pub unsafe extern "C" fn swap_cache_get_shadow(entry: swp_entry_t) -> *mut c_void {
    let mut swp_tb = 0;
    swp_tb = swap_table_get(__swap_entry_to_cluster(entry),
    swp_cluster_offset(entry));
    if (swp_tb_is_shadow(swp_tb)) {
    return swp_tb_to_shadow(swp_tb);
    }
    return core::ptr::null_mut();
    }
//
// __swap_cache_add_check - Check if a range is suitable for adding a folio.
// @ci: The locked swap cluster
// @targ_entry: The target swap entry to check, will be rounded down by @nr
// @nr: Number of slots to check, must be a power of 2
// @shadowp: Returns the shadow value if one exists in the range
// @memcg_id: Returns the memory cgroup id, NULL to ignore cgroup check
//
// Check if all slots covered by given range have a swap count >= 1.
// Retrieves the shadow if there is one. If @memcg_id is not NULL, also
// checks if all slots belong to the same cgroup and return the cgroup
// private id.
//
// Context: Caller must lock the cluster.
// Return: 0 if success, error code if failed.
//
#[no_mangle]
pub unsafe extern "C" fn __swap_cache_add_check(ci: *mut swap_cluster_info, targ_entry: swp_entry_t, nr: c_ulong, shadowp: *mut *mut c_void, memcg_id: *mut c_ushort) -> c_int {
    let mut ci_off = 0;
    let mut ci_end = 0;
    let mut old_tb = 0;
    let mut is_zero = 0;
    lockdep_assert_held(&ci.lock);
//
// If the target slot is not swapped out or already cached, return
// -ENOENT or -EEXIST. If the batch is not suitable, could be a
// race with concurrent free or cache add, return -EBUSY.
//
    if (unlikely(!ci.table)) {
    return -ENOENT;
    }
    ci_off = swp_cluster_offset(targ_entry);
    old_tb = __swap_table_get(ci, ci_off);
    if (swp_tb_is_folio(old_tb)) {
    return -EEXIST;
    }
    if (!__swp_tb_get_count(old_tb)) {
    return -ENOENT;
    }
    if (shadowp && swp_tb_is_shadow(old_tb)) {
// shadowp = swp_tb_to_shadow(old_tb);
    }
    if (memcg_id) {
// memcg_id = __swap_cgroup_get(ci, ci_off);
    }
    if (nr == 1) {
    return 0;
    }
    is_zero = __swap_table_test_zero(ci, ci_off);
    ci_off = round_down(ci_off, nr);
    ci_end = ci_off + nr;
    do {
    old_tb = __swap_table_get(ci, ci_off);
    if (unlikely(swp_tb_is_folio(old_tb) ||
    !__swp_tb_get_count(old_tb) ||
    is_zero != __swap_table_test_zero(ci, ci_off) ||
    (memcg_id && *memcg_id != __swap_cgroup_get(ci, ci_off)))) {
    return -EBUSY;
    }
    } while (++ci_off < ci_end);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn __swap_cache_do_add_folio(ci: *mut swap_cluster_info, folio: *mut folio, entry: swp_entry_t) {
pub static mut ci_off: c_uint = 0;
pub static mut nr_pages: c_ulong = 0;
pub static mut pfn: c_ulong = 0;
    let mut old_tb = 0;
    VM_WARN_ON_ONCE_FOLIO(!folio_test_locked(folio), folio);
    VM_WARN_ON_ONCE_FOLIO(folio_test_swapcache(folio), folio);
    VM_WARN_ON_ONCE_FOLIO(!folio_test_swapbacked(folio), folio);
    ci_end = ci_off + nr_pages;
    do {
    old_tb = __swap_table_get(ci, ci_off);
    VM_WARN_ON_ONCE(swp_tb_is_folio(old_tb));
    __swap_table_set(ci, ci_off, pfn_to_swp_tb(pfn, __swp_tb_get_flags(old_tb)));
    } while (++ci_off < ci_end);
    folio_ref_add(folio, nr_pages);
    folio_set_swapcache(folio);
    folio.swap = entry;
    }
//
// __swap_cache_add_folio - Add a folio to the swap cache and update stats.
// @ci: The locked swap cluster.
// @folio: The folio to be added.
// @entry: The swap entry corresponding to the folio.
//
// Unconditionally add a folio to the swap cache. The caller must ensure
// all slots are usable and have no conflicts. This assigns entry to
// @folio->swap, increases folio refcount by the number of pages, and
// updates swap cache stats.
//
// Context: Caller must ensure the folio is locked and lock the cluster
// that holds the entries.
//
#[no_mangle]
pub unsafe extern "C" fn __swap_cache_add_folio(ci: *mut swap_cluster_info, folio: *mut folio, entry: swp_entry_t) {
pub static mut nr_pages: c_ulong = 0;
    __swap_cache_do_add_folio(ci, folio, entry);
    node_stat_mod_folio(folio, NR_FILE_PAGES, nr_pages);
    lruvec_stat_mod_folio(folio, NR_SWAPCACHE, nr_pages);
    }
#[no_mangle]
pub unsafe extern "C" fn __swap_cache_do_del_folio(ci: *mut swap_cluster_info, folio: *mut folio, entry: swp_entry_t, shadow: *mut c_void) {
    let mut old_tb = 0;
pub static mut si: *mut c_void = core::ptr::null_mut();
    let mut ci_start = 0;
    let mut ci_off = 0;
    let mut ci_end = 0;
pub static mut folio_swapped: bool = false;
pub static mut nr_pages: c_ulong = 0;
    VM_WARN_ON_ONCE(__swap_entry_to_cluster(entry) != ci);
    VM_WARN_ON_ONCE_FOLIO(!folio_test_locked(folio), folio);
    VM_WARN_ON_ONCE_FOLIO(!folio_test_swapcache(folio), folio);
    VM_WARN_ON_ONCE_FOLIO(folio_test_writeback(folio), folio);
    si = __swap_entry_to_info(entry);
    ci_start = swp_cluster_offset(entry);
    ci_end = ci_start + nr_pages;
    ci_off = ci_start;
    do {
    old_tb = __swap_table_get(ci, ci_off);
    WARN_ON_ONCE!(!swp_tb_is_folio(old_tb) ||
    swp_tb_to_folio(old_tb) != folio);
    if (__swp_tb_get_count(old_tb)) {
    folio_swapped = true;
    }
    else {
    need_free = true;
    }
// If shadow is NULL, we set an empty shadow.
    __swap_table_set(ci, ci_off, shadow_to_swp_tb(shadow,
    __swp_tb_get_flags(old_tb)));
    } while (++ci_off < ci_end);
    folio.swap.val = 0;
    folio_clear_swapcache(folio);
    if (!folio_swapped) {
    __swap_cluster_free_entries(si, ci, ci_start, nr_pages);
    } else if (need_free) {
    ci_off = ci_start;
    do {
    if (!__swp_tb_get_count(__swap_table_get(ci, ci_off))) {
    __swap_cluster_free_entries(si, ci, ci_off, 1);
    }
    } while (++ci_off < ci_end);
    }
    }
//
// __swap_cache_del_folio - Removes a folio from the swap cache.
// @ci: The locked swap cluster.
// @folio: The folio.
// @entry: The first swap entry that the folio corresponds to.
// @shadow: shadow value to be filled in the swap cache.
//
// Removes a folio from the swap cache and fills a shadow in place.
// This won't put the folio's refcount. The caller has to do that.
//
// Context: Caller must ensure the folio is locked and in the swap cache
// using the index of @entry, and lock the cluster that holds the entries.
//
#[no_mangle]
pub unsafe extern "C" fn __swap_cache_del_folio(ci: *mut swap_cluster_info, folio: *mut folio, entry: swp_entry_t, shadow: *mut c_void) {
pub static mut nr_pages: c_ulong = 0;
    __swap_cache_do_del_folio(ci, folio, entry, shadow);
    node_stat_mod_folio(folio, NR_FILE_PAGES, -nr_pages);
    lruvec_stat_mod_folio(folio, NR_SWAPCACHE, -nr_pages);
    }
//
// swap_cache_del_folio - Removes a folio from the swap cache.
// @folio: The folio.
//
// Same as __swap_cache_del_folio, but handles lock and refcount. The
// caller must ensure the folio is either clean or has a swap count
// equal to zero, or it may cause data loss.
//
// Context: Caller must ensure the folio is locked and in the swap cache.
//
#[no_mangle]
pub unsafe extern "C" fn swap_cache_del_folio(folio: *mut folio) {
pub static mut ci: *mut c_void = core::ptr::null_mut();
pub static mut entry: swp_entry_t = 0;
    ci = swap_cluster_lock(__swap_entry_to_info(entry), swp_offset(entry));
    __swap_cache_del_folio(ci, folio, entry, core::ptr::null_mut());
    swap_cluster_unlock(ci);
    folio_ref_sub(folio, folio_nr_pages(folio));
    }
//
// __swap_cache_replace_folio - Replace a folio in the swap cache.
// @ci: The locked swap cluster.
// @old: The old folio to be replaced.
// @new: The new folio.
//
// Replace an existing folio in the swap cache with a new folio. The
// caller is responsible for setting up the new folio's flag and swap
// entries. Replacement will take the new folio's swap entry value as
// the starting offset to override all slots covered by the new folio.
//
// Context: Caller must ensure both folios are locked, and lock the
// cluster that holds the old folio to be replaced.
//
#[no_mangle]
pub unsafe extern "C" fn __swap_cache_replace_folio(ci: *mut swap_cluster_info, old: *mut folio, new: *mut folio) {
pub static mut entry: swp_entry_t = 0;
pub static mut nr_pages: c_ulong = 0;
pub static mut ci_off: c_uint = 0;
pub static mut ci_end: c_uint = 0;
pub static mut pfn: c_ulong = 0;
    let mut old_tb = 0;
    VM_WARN_ON_ONCE(!folio_test_swapcache(old) || !folio_test_swapcache(new));
    VM_WARN_ON_ONCE(!folio_test_locked(old) || !folio_test_locked(new));
    VM_WARN_ON_ONCE(!entry.val);
// Swap cache still stores N entries instead of a high-order entry
    do {
    old_tb = __swap_table_get(ci, ci_off);
    WARN_ON_ONCE!(!swp_tb_is_folio(old_tb) || swp_tb_to_folio(old_tb) != old);
    __swap_table_set(ci, ci_off, pfn_to_swp_tb(pfn, __swp_tb_get_flags(old_tb)));
    } while (++ci_off < ci_end);
//
// If the old folio is partially replaced (e.g., splitting a large
// folio, the old folio is shrunk, and new split sub folios replace
// the shrunk part), ensure the new folio doesn't overlap it.
//
    if (IS_ENABLED!(CONFIG_DEBUG_VM) &&
    folio_order(old) != folio_order(new)) {
    ci_off = swp_cluster_offset(old.swap);
    ci_end = ci_off + folio_nr_pages(old);
    while (ci_off++ < ci_end) {
    WARN_ON_ONCE!(swp_tb_to_folio(__swap_table_get(ci, ci_off)) != old);
    }
    }
    }
//
// Try to allocate a folio of given order in the swap cache.
//
// This helper resolves the potential races of swap allocation
// and prepares a folio to be used for swap IO. May return following
// value:
//
// -ENOMEM / -EBUSY: Order is too large or in conflict with sub slot,
// caller should shrink the order and retry
// -ENOENT / -EEXIST: Target swap entry is unavailable or cached, the caller
// should abort or try to use the cached folio instead
//
#[no_mangle]
pub unsafe extern "C" fn __swap_cache_alloc(ci: *mut swap_cluster_info, targ_entry: swp_entry_t, gfp: gfp_t, order: c_uint, vmf: *mut vm_fault, mpol: *mut mempolicy, ilx: pgoff_t) -> *mut c_void {
    let mut err = 0;
    let mut entry;
pub static mut folio: *mut c_void = core::ptr::null_mut();
    let mut shadow = core::ptr::null_mut();
    let mut memcg_id = 0;
    unsigned long address, nr_pages = 1UL << order;
    let mut vma = vmf ? vmf.vma : core::ptr::null_mut();
    VM_WARN_ON_ONCE(nr_pages > SWAPFILE_CLUSTER);
    entry.val = round_down(targ_entry.val, nr_pages);
// Check if the slot and range are available, skip allocation if not
    spin_lock(&ci.lock);
    err = __swap_cache_add_check(ci, targ_entry, nr_pages, core::ptr::null_mut(), core::ptr::null_mut());
    spin_unlock(&ci.lock);
    if (unlikely(err)) {
    return ERR_PTR(err);
    }
//
// Limit THP gfp. The limitation is a no-op for typical
// GFP_HIGHUSER_MOVABLE but matters for shmem.
//
    if (order) {
    gfp = thp_shmem_limit_gfp_mask(vma_thp_gfp_mask(vma), gfp);
    }
    if (mpol || !vmf) {
    folio = folio_alloc_mpol(gfp, order, mpol, ilx, numa_node_id());
    } else {
    address = round_down(vmf.address, PAGE_SIZE << order);
    folio = vma_alloc_folio(gfp, order, vmf.vma, address);
    }
    if (unlikely(!folio)) {
    return ERR_PTR(-ENOMEM);
    }
// Double check the range is still not in conflict
    spin_lock(&ci.lock);
    err = __swap_cache_add_check(ci, targ_entry, nr_pages, &shadow, &memcg_id);
    if (unlikely(err)) {
    spin_unlock(&ci.lock);
    folio_put(folio);
    return ERR_PTR(err);
    }
    __folio_set_locked(folio);
    __folio_set_swapbacked(folio);
    __swap_cache_do_add_folio(ci, folio, entry);
    spin_unlock(&ci.lock);
    if (mem_cgroup_swapin_charge_folio(folio, memcg_id,
    vmf ? vmf.vma.vm_mm : core::ptr::null_mut(), gfp)) {
    spin_lock(&ci.lock);
    __swap_cache_do_del_folio(ci, folio, entry, shadow);
    spin_unlock(&ci.lock);
    folio_unlock(folio);
// nr_pages refs from swap cache, 1 from allocation
    folio_put_refs(folio, nr_pages + 1);
    count_mthp_stat(order, MTHP_STAT_SWPIN_FALLBACK_CHARGE);
    return ERR_PTR(-ENOMEM);
    }
    if (order > 1 && folio_memcg_alloc_deferred(folio)) {
    spin_lock(&ci.lock);
    __swap_cache_do_del_folio(ci, folio, entry, shadow);
    spin_unlock(&ci.lock);
    folio_unlock(folio);
// nr_pages refs from swap cache, 1 from allocation
    folio_put_refs(folio, nr_pages + 1);
    return ERR_PTR(-ENOMEM);
    }
// memsw uncharges swap when folio is added to swap cache
    memcg1_swapin(folio);
    if (shadow) {
    workingset_refault(folio, shadow);
    }
    node_stat_mod_folio(folio, NR_FILE_PAGES, nr_pages);
    lruvec_stat_mod_folio(folio, NR_SWAPCACHE, nr_pages);
// Caller will initiate read into locked new_folio
    folio_add_lru(folio);
    return folio;
    }
//
// swap_cache_alloc_folio - Allocate folio for swapped out slot in swap cache.
// @targ_entry: swap entry indicating the target slot
// @gfp: memory allocation flags
// @orders: allocation orders, must be non zero
// @vmf: fault information
// @mpol: NUMA memory allocation policy to be applied
// @ilx: NUMA interleave index, for use only when MPOL_INTERLEAVE
//
// Allocate a folio in the swap cache for one swap slot, typically before
// doing IO (e.g. swap in or zswap writeback). The swap slot indicated by
// @targ_entry must have a non-zero swap count (swapped out).
//
// Context: Caller must protect the swap device with reference count or locks.
// Return: Returns the folio if allocation succeeded and folio is in the swap
// cache. Returns error code if failed due to race, OOM or invalid arguments.
//
#[no_mangle]
pub unsafe extern "C" fn swap_cache_alloc_folio(targ_entry: swp_entry_t, gfp: gfp_t, orders: c_ulong, vmf: *mut vm_fault, mpol: *mut mempolicy, ilx: pgoff_t) -> *mut c_void {
    let mut order = 0;
    let mut err = 0;
pub static mut ret: *mut c_void = core::ptr::null_mut();
pub static mut ci: *mut c_void = core::ptr::null_mut();
    ci = __swap_entry_to_cluster(targ_entry);
    order = highest_order(orders);
// orders must be non-zero, and must not exceed cluster size.
    if (WARN_ON_ONCE!(!orders || (1UL << order) > SWAPFILE_CLUSTER)) {
    return ERR_PTR(-EINVAL);
    }
    do {
    ret = __swap_cache_alloc(ci, targ_entry, gfp, order,
    vmf, mpol, ilx);
    if (!IS_ERR(ret)) {
    break;
    }
    err = PTR_ERR(ret);
    if (!order || (err && err != -EBUSY && err != -ENOMEM)) {
    break;
    }
    count_mthp_stat(order, MTHP_STAT_SWPIN_FALLBACK);
    order = next_order(&orders, order);
    } while (orders);
    return ret;
    }
//
// If we are the only user, then try to free up the swap cache.
//
// Its ok to check the swapcache flag without the folio lock
// here because we are going to recheck again inside
// folio_free_swap() _with_ the lock.
// - Marcelo
//
#[no_mangle]
pub unsafe extern "C" fn free_swap_cache(folio: *mut folio) {
    if (folio_test_swapcache(folio) && !folio_mapped(folio) &&
    folio_trylock(folio)) {
    folio_free_swap(folio);
    folio_unlock(folio);
    }
    }
//
// Freeing a folio and also freeing any swap cache associated with
// this folio if it is the last user.
//
#[no_mangle]
pub unsafe extern "C" fn free_folio_and_swap_cache(folio: *mut folio) {
    free_swap_cache(folio);
    if (!is_huge_zero_folio(folio)) {
    folio_put(folio);
    }
    }
//
// Passed an array of pages, drop them all from swapcache and then release
// them.  They are removed from the LRU and freed if this is their last use.
//
#[no_mangle]
pub unsafe extern "C" fn free_pages_and_swap_cache(pages: *mut encoded_page, nr: c_int) {
pub static mut folios: usize = 0;
    unsigned int refs[FOLIO_BATCH_SIZE];
    folio_batch_init(&folios);
    while (i < nr) {
    let mut folio = page_folio(encoded_page_ptr(pages[i]));
    free_swap_cache(folio);
    refs[folios.nr] = 1;
    if (unlikely(encoded_page_flags(pages[i]) &
    ENCODED_PAGE_BIT_NR_PAGES_NEXT)) {
    refs[folios.nr] = encoded_nr_pages(pages[++i]);
    }
    if (folio_batch_add(&folios, folio) == 0) {
    folios_put_refs(&folios, refs);
    }
    }
    if (folios.nr) {
    folios_put_refs(&folios, refs);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn swap_use_vma_readahead() -> bool {
    return READ_ONCE(enable_vma_readahead) && !atomic_read(&nr_rotate_swap);
    }
//
// swap_update_readahead - Update the readahead statistics of VMA or globally.
// @folio: the swap cache folio that just got hit.
// @vma: the VMA that should be updated, could be NULL for global update.
// @addr: the addr that triggered the swapin, ignored if @vma is NULL.
//
#[no_mangle]
pub unsafe extern "C" fn swap_update_readahead(folio: *mut folio, vma: *mut vm_area_struct, addr: c_ulong) {
    bool readahead, vma_ra = swap_use_vma_readahead();
//
// At the moment, we don't support PG_readahead for anon THP
// so let's bail out rather than confusing the readahead stat.
//
    if (unlikely(folio_test_large(folio))) {
    return;
    }
    readahead = folio_test_clear_readahead(folio);
    if (vma && vma_ra) {
    let mut ra_val = 0;
    let mut win = 0;
    let mut hits = 0;
    ra_val = GET_SWAP_RA_VAL(vma);
    win = SWAP_RA_WIN(ra_val);
    hits = SWAP_RA_HITS(ra_val);
    if (readahead) {
    hits = min_t(int, hits + 1, SWAP_RA_HITS_MAX);
    }
    atomic_long_set(&vma.swap_readahead_info,
    SWAP_RA_VAL(addr, win, hits));
    }
    if (readahead) {
    count_vm_event(SWAP_RA_HIT);
    if (!vma || !vma_ra) {
    atomic_inc(&swapin_readahead_hits);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn swap_cache_read_folio(ctx: *mut swap_io_ctx, entry: swp_entry_t, gfp: gfp_t, mpol: *mut mempolicy, ilx: pgoff_t, readahead: bool) -> *mut c_void {
pub static mut folio: *mut c_void = core::ptr::null_mut();
    do {
    folio = swap_cache_get_folio(entry);
    if (folio) {
    return folio;
    }
    folio = swap_cache_alloc_folio(entry, gfp, BIT(0), core::ptr::null_mut(), mpol, ilx);
    } while (PTR_ERR(folio) == -EEXIST);
    if (IS_ERR_OR_NULL(folio)) {
    return core::ptr::null_mut();
    }
    swap_read_folio(ctx, folio);
    if (readahead) {
    folio_set_readahead(folio);
    count_vm_event(SWAP_RA);
    }
    return folio;
    }
//
// swapin_sync - swap-in one or multiple entries skipping readahead.
// @entry: swap entry indicating the target slot
// @gfp: memory allocation flags
// @orders: allocation orders
// @vmf: fault information
// @mpol: NUMA memory allocation policy to be applied
// @ilx: NUMA interleave index, for use only when MPOL_INTERLEAVE
//
// This allocates a folio suitable for given @orders, or returns the
// existing folio in the swap cache for @entry. This initiates the IO, too,
// if needed. @entry is rounded down if @orders allow large allocation.
//
// Context: Caller must ensure @entry is valid and pin the swap device with refcount.
// Return: Returns the folio on success, error code if failed.
//
#[no_mangle]
pub unsafe extern "C" fn swapin_sync(entry: swp_entry_t, gfp: gfp_t, orders: c_ulong, vmf: *mut vm_fault, mpol: *mut mempolicy, ilx: pgoff_t) -> *mut c_void {
pub static mut ctx: swap_io_ctx = 0;
pub static mut folio: *mut c_void = core::ptr::null_mut();
    do {
    folio = swap_cache_get_folio(entry);
    if (folio) {
    return folio;
    }
    folio = swap_cache_alloc_folio(entry, gfp, orders, vmf, mpol, ilx);
    } while (PTR_ERR(folio) == -EEXIST);
    if (IS_ERR(folio)) {
    return folio;
    }
    swap_read_folio(&ctx, folio);
    swap_read_submit(&ctx);
    return folio;
    }
//
// Locate a page of swap in physical memory, reserving swap cache space
// and reading the disk if it is not already cached.
// A failure return means that either the page allocation failed or that
// the swap entry is no longer in use.
//
#[no_mangle]
pub unsafe extern "C" fn read_swap_cache_async(ctx: *mut swap_io_ctx, entry: swp_entry_t, gfp_mask: gfp_t, vma: *mut vm_area_struct, addr: c_ulong) -> *mut c_void {
pub static mut si: *mut c_void = core::ptr::null_mut();
pub static mut mpol: *mut c_void = core::ptr::null_mut();
    let mut ilx;
pub static mut folio: *mut c_void = core::ptr::null_mut();
    si = get_swap_device(entry);
    if (!si) {
    return core::ptr::null_mut();
    }
    mpol = get_vma_policy(vma, addr, 0, &ilx);
    folio = swap_cache_read_folio(ctx, entry, gfp_mask, mpol, ilx, false);
    mpol_cond_put(mpol);
    put_swap_device(si);
    return folio;
    }
#[no_mangle]
pub unsafe extern "C" fn swap_cache_read_folio_sync(entry: swp_entry_t, gfp: gfp_t, mpol: *mut mempolicy, ilx: pgoff_t) -> *mut c_void {
pub static mut ctx: swap_io_ctx = 0;
pub static mut folio: *mut c_void = core::ptr::null_mut();
    folio = swap_cache_read_folio(&ctx, entry, gfp, mpol, ilx, false);
    swap_read_submit(&ctx);
    return folio;
    }
#[no_mangle]
pub unsafe extern "C" fn __swapin_nr_pages(prev_offset: c_ulong, offset: c_ulong, hits: c_int, max_pages: c_int, prev_win: c_int) -> c_uint {
    let mut pages = 0;
    let mut last_ra = 0;
//
// This heuristic has been found to work well on both sequential and
// random loads, swapping to hard disk or to SSD: please don't ask
// what the "+ 2" means, it just happens to work well, that's all.
//
    pages = hits + 2;
    if (pages == 2) {
//
// We can have no readahead hits to judge by: but must not get
// stuck here forever, so check for an adjacent offset instead
// (and don't even bother to check whether swap type is same).
//
    if (offset != prev_offset + 1 && offset != prev_offset - 1) {
    pages = 1;
    }
    } else {
pub static mut roundup: c_uint = 4;
    while (roundup < pages) {
    roundup <<= 1;
    }
    pages = roundup;
    }
    if (pages > max_pages) {
    pages = max_pages;
    }
// Don't shrink readahead too fast
    last_ra = prev_win / 2;
    if (pages < last_ra) {
    pages = last_ra;
    }
    return pages;
    }
#[no_mangle]
unsafe extern "C" fn swapin_nr_pages(offset: c_ulong) -> c_ulong {
    static unsigned long prev_offset;
    let mut hits = 0;
    let mut pages = 0;
    let mut max_pages = 0;
    static atomic_t last_readahead_pages;
    max_pages = 1 << READ_ONCE(page_cluster);
    if (max_pages <= 1) {
    return 1;
    }
    hits = atomic_xchg(&swapin_readahead_hits, 0);
    pages = __swapin_nr_pages(READ_ONCE(prev_offset), offset, hits,
    max_pages,
    atomic_read(&last_readahead_pages));
    if (!hits) {
    WRITE_ONCE(prev_offset, offset);
    }
    atomic_set(&last_readahead_pages, pages);
    return pages;
    }
//
// swap_cluster_readahead - swap in pages in hope we need them soon
// @entry: swap entry of this memory
// @gfp_mask: memory allocation flags
// @mpol: NUMA memory allocation policy to be applied
// @ilx: NUMA interleave index, for use only when MPOL_INTERLEAVE
//
// Returns the struct folio for entry and addr, after queueing swapin.
//
// Primitive swap readahead code. We simply read an aligned block of
// (1 << page_cluster) entries in the swap area. This method is chosen
// because it doesn't cost us any seek time.  We also make sure to queue
// the 'original' request together with the readahead ones...
//
// Note: it is intentional that the same NUMA policy and interleave index
// are used for every page of the readahead: neighbouring pages on swap
// are fairly likely to have been swapped out from the same node.
//
#[no_mangle]
pub unsafe extern "C" fn swap_cluster_readahead(entry: swp_entry_t, gfp_mask: gfp_t, mpol: *mut mempolicy, ilx: pgoff_t) -> *mut c_void {
pub static mut folio: *mut c_void = core::ptr::null_mut();
pub static mut entry_offset: c_ulong = 0;
pub static mut offset: c_ulong = 0;
    unsigned long start_offset, end_offset;
    let mut mask = 0;
    let mut si = __swap_entry_to_info(entry);
pub static mut ctx: swap_io_ctx = 0;
pub static mut plug: usize = 0;
    let mut ra_entry;
    mask = swapin_nr_pages(offset) - 1;
    if (!mask) {
// goto;
    }
// Read a page_cluster sized and aligned cluster around offset.
    start_offset = offset & ~mask;
    end_offset = offset | mask;
    if (!start_offset)	/* First page is swap header. */ {
    start_offset += 1;
    }
    if (end_offset >= si.max) {
    end_offset = si.max - 1;
    }
    blk_start_plug(&plug);
    while (offset <= end_offset ) {
// Ok, do the async read-ahead now
    ra_entry = swp_entry(swp_type(entry), offset);
    folio = swap_cache_read_folio(&ctx, ra_entry, gfp_mask, mpol,
    ilx, offset != entry_offset);
    if (!folio) {
    continue;
    }
    folio_put(folio);
    }
    blk_finish_plug(&plug);
    swap_read_submit(&ctx);
// label;
    return swap_cache_read_folio_sync(entry, gfp_mask, mpol, ilx);
    }
#[no_mangle]
pub unsafe extern "C" fn swap_vma_ra_win(vmf: *mut vm_fault, start: *mut c_ulong, end: *mut c_ulong) -> c_int {
    let mut vma = vmf.vma;
    let mut ra_val = 0;
    unsigned long faddr, prev_faddr, left, right;
    let mut max_win = 0;
    let mut hits = 0;
    let mut prev_win = 0;
    let mut win = 0;
    max_win = 1 << min(READ_ONCE(page_cluster), SWAP_RA_ORDER_CEILING);
    if (max_win == 1) {
    return 1;
    }
    faddr = vmf.address;
    ra_val = GET_SWAP_RA_VAL(vma);
    prev_faddr = SWAP_RA_ADDR(ra_val);
    prev_win = SWAP_RA_WIN(ra_val);
    hits = SWAP_RA_HITS(ra_val);
    win = __swapin_nr_pages(PFN_DOWN(prev_faddr), PFN_DOWN(faddr), hits,
    max_win, prev_win);
    atomic_long_set(&vma.swap_readahead_info, SWAP_RA_VAL(faddr, win, 0));
    if (win == 1) {
    return 1;
    }
    if (faddr == prev_faddr + PAGE_SIZE) {
    left = faddr;
    }

    else if (prev_faddr == faddr + PAGE_SIZE) {
    left = faddr - (win << PAGE_SHIFT) + PAGE_SIZE;
    }
    else {
    left = faddr - (((win - 1) / 2) << PAGE_SHIFT);
    }
    right = left + (win << PAGE_SHIFT);
    if ((long)left < 0) {
    left = 0;
    }
// start = max3(left, vma->vm_start, faddr & PMD_MASK);
// end = min3(right, vma->vm_end, (faddr & PMD_MASK) + PMD_SIZE);
    return win;
    }
//
// swap_vma_readahead - swap in pages in hope we need them soon
// @targ_entry: swap entry of the targeted memory
// @gfp_mask: memory allocation flags
// @mpol: NUMA memory allocation policy to be applied
// @targ_ilx: NUMA interleave index, for use only when MPOL_INTERLEAVE
// @vmf: fault information
//
// Returns the struct folio for entry and addr, after queueing swapin.
//
// Primitive swap readahead code. We simply read in a few pages whose
// virtual addresses are around the fault address in the same vma.
//
// Caller must hold read mmap_lock if vmf->vma is not NULL.
//
#[no_mangle]
pub unsafe extern "C" fn swap_vma_readahead(targ_entry: swp_entry_t, gfp_mask: gfp_t, mpol: *mut mempolicy, targ_ilx: pgoff_t, vmf: *mut vm_fault) -> *mut c_void {
pub static mut ctx: swap_io_ctx = 0;
pub static mut plug: usize = 0;
pub static mut folio: *mut c_void = core::ptr::null_mut();
    let mut pte = core::ptr::null_mut(), pentry;
    let mut win = 0;
    unsigned long start, end, addr;
pub static mut ilx: pgoff_t = 0;
    win = swap_vma_ra_win(vmf, &start, &end);
    if (win == 1) {
// goto;
    }
    ilx = targ_ilx - PFN_DOWN(vmf.address - start);
    blk_start_plug(&plug);
    while (addr < end) {
    let mut si = core::ptr::null_mut();
    let mut entry;
    if (!pte++) {
    pte = pte_offset_map(vmf.pmd, addr);
    if (!pte) {
    break;
    }
    }
    pentry = ptep_get_lockless(pte);
    entry = softleaf_from_pte(pentry);
    if (!softleaf_is_swap(entry)) {
    continue;
    }
    pte_unmap(pte);
    pte = core::ptr::null_mut();
//
// Readahead entry may come from a device that we are not
// holding a reference to, try to grab a reference, or skip.
//
    if (swp_type(entry) != swp_type(targ_entry)) {
    si = get_swap_device(entry);
    if (!si) {
    continue;
    }
    }
    folio = swap_cache_read_folio(&ctx, entry, gfp_mask, mpol, ilx,
    addr != vmf.address);
    if (si) {
    put_swap_device(si);
    }
    if (!folio) {
    continue;
    }
    folio_put(folio);
    }
    if (pte) {
    pte_unmap(pte);
    }
    blk_finish_plug(&plug);
    swap_read_submit(&ctx);
// label;
// The folio was likely read above, so no need for plugging here
    return swap_cache_read_folio_sync(targ_entry, gfp_mask, mpol, targ_ilx);
    }
//
// swapin_readahead - swap in pages in hope we need them soon
// @entry: swap entry of this memory
// @gfp_mask: memory allocation flags
// @vmf: fault information
//
// Returns the struct folio for entry and addr, after queueing swapin.
//
// It's a main entry function for swap readahead. By the configuration,
it will read ahead blocks by cluster-based(ie, physical disk based)
// or vma-based(ie, virtual address based on faulty address) readahead.
//
#[no_mangle]
pub unsafe extern "C" fn swapin_readahead(entry: swp_entry_t, gfp_mask: gfp_t, vmf: *mut vm_fault) -> *mut c_void {
pub static mut mpol: *mut c_void = core::ptr::null_mut();
    let mut ilx;
pub static mut folio: *mut c_void = core::ptr::null_mut();
    mpol = get_vma_policy(vmf.vma, vmf.address, 0, &ilx);
    folio = swap_use_vma_readahead() ?
    swap_vma_readahead(entry, gfp_mask, mpol, ilx, vmf) :
    swap_cluster_readahead(entry, gfp_mask, mpol, ilx);
    mpol_cond_put(mpol);
    return folio;
    }
pub static mut ctl_table: usize = 0;
#[no_mangle]
unsafe extern "C" fn swap_readahead_setup()  {
pub static mut megs: c_ulong = 0;
// Use a smaller cluster for small-memory machines
    if (megs < 16) {
    page_cluster = 2;
    }
    else {
    page_cluster = 3;
    }
//
// Right now other parts of the system means that we
// _really_ don't want to cluster much more
//
    register_sysctl_init("vm", swap_readahead_sysctl_table);
    }

#[no_mangle]
pub unsafe extern "C" fn vma_ra_enabled_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t {
    return sysfs_emit(buf, "%s\n", str_true_false(enable_vma_readahead));
    }
#[no_mangle]
pub unsafe extern "C" fn vma_ra_enabled_store(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char, count: size_t) -> ssize_t {
    let mut ret = 0;
    ret = kstrtobool(buf, &enable_vma_readahead);
    if (ret) {
    return ret;
    }
    return count;
    }
pub static mut vma_ra_enabled_attr: kobj_attribute = 0;
    static struct attribute *swap_attrs[] = {
    &vma_ra_enabled_attr.attr,
    core::ptr::null_mut(),
    };
pub static mut attribute_group: usize = 0;
#[no_mangle]
unsafe extern "C" fn swap_sysfs_init() -> c_int {
    let mut err = 0;
pub static mut swap_kobj: *mut c_void = core::ptr::null_mut();
    swap_kobj = kobject_create_and_add("swap", mm_kobj);
    if (!swap_kobj) {
    pr_err!("failed to create swap kobject\n");
    return -ENOMEM;
    }
    err = sysfs_create_group(swap_kobj, &swap_attr_group);
    if (err) {
    pr_err!("failed to register swap group\n");
// goto;
    }
// Swap cache writeback is LRU based, no tags for it
    mapping_set_no_writeback_tags(&swap_space);
    return 0;
// label;
    kobject_put(swap_kobj);
    return err;
    }

#[no_mangle]
unsafe extern "C" fn swap_sysfs_init() -> c_int {
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn swap_init() -> c_int {
    swap_readahead_setup();
    return swap_sysfs_init();
    }
    subsys_initcall!(swap_init);