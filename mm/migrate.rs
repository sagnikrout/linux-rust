//! Automatically rewritten from C to Rust
//! Source: mm/migrate.c
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
// Memory Migration functionality - linux/mm/migrate.c
//
// Copyright (C) 2006 Silicon Graphics, Inc., Christoph Lameter
//
// Page migration was first developed in the context of the memory hotplug
// project. The main authors of the migration code are:
//
// IWAMOTO Toshihiro <iwamoto@valinux.co.jp>
// Hirokazu Takahashi <taka@valinux.co.jp>
// Dave Hansen <haveblue@us.ibm.com>
// Christoph Lameter
//

pub static mut offline_movable_ops: *mut c_void = core::ptr::null_mut();
pub static mut zsmalloc_movable_ops: *mut c_void = core::ptr::null_mut();
#[no_mangle]
pub unsafe extern "C" fn set_movable_ops(ops: *const movable_operations, type: pagetype) -> c_int {
//
// We only allow for selected types and don't handle concurrent
// registration attempts yet.
//
    match (type) {
    PGTY_offline => {
    if (offline_movable_ops && ops) {
    return -EBUSY;
    }
    offline_movable_ops = ops;
    // break;
    }
    PGTY_zsmalloc => {
    if (zsmalloc_movable_ops && ops) {
    return -EBUSY;
    }
    zsmalloc_movable_ops = ops;
    // break;
    }
    _ => {
    return -EINVAL;
    }
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(set_movable_ops);
    static const struct movable_operations *page_movable_ops(page *page)
    {
    VM_WARN_ON_ONCE_PAGE(!page_has_movable_ops(page), page);
//
// If we enable page migration for a page of a certain type by marking
// it as movable, the page type must be sticky until the page gets freed
// back to the buddy.
//
    if (PageOffline(page)) {
// Only balloon page migration sets PageOffline pages movable.
    return offline_movable_ops;
    }
    if (PageZsmalloc(page)) {
    return zsmalloc_movable_ops;
    }
    return core::ptr::null_mut();
    }
//
// isolate_movable_ops_page - isolate a movable_ops page for migration
// @page: The page.
// @mode: The isolation mode.
//
// Try to isolate a movable_ops page for migration. Will fail if the page is
// not a movable_ops page, if the page is already isolated for migration
// or if the page was just was released by its owner.
//
// Once isolated, the page cannot get freed until it is either putback
// or migrated.
//
// Returns true if isolation succeeded, otherwise false.
//
#[no_mangle]
pub unsafe extern "C" fn isolate_movable_ops_page(page: *mut page, mode: isolate_mode_t) -> bool {
//
// TODO: these pages will not be folios in the future. All
// folio dependencies will have to be removed.
//
    let mut folio = folio_get_nontail_page(page);
pub static mut mops: *mut c_void = core::ptr::null_mut();
//
// Avoid burning cycles with pages that are yet under __free_pages(),
// or just got freed under us.
//
// In case we 'win' a race for a movable page being freed under us and
// raise its refcount preventing __free_pages() from doing its job
// the put_page() at the end of this block will take care of
// release this page, thus avoiding a nasty leakage.
//
    if (!folio) {
// goto;
    }
//
// Check for movable_ops pages before taking the page lock because
// we use non-atomic bitops on newly allocated page flags so
// unconditionally grabbing the lock ruins page's owner side.
//
// Note that once a page has movable_ops, it will stay that way
// until the page was freed.
//
    if (unlikely(!page_has_movable_ops(page))) {
// goto;
    }
//
// As movable pages are not isolated from LRU lists, concurrent
// compaction threads can race against page migration functions
// as well as race against the releasing a page.
//
// In order to avoid having an already isolated movable page
// being (wrongly) re-isolated while it is under migration,
// or to avoid attempting to isolate pages being released,
// lets be sure we have the page lock
// before proceeding with the movable page isolation steps.
//
    if (unlikely(!folio_trylock(folio))) {
// goto;
    }
    VM_WARN_ON_ONCE_PAGE(!page_has_movable_ops(page), page);
    if (PageMovableOpsIsolated(page)) {
// goto;
    }
    mops = page_movable_ops(page);
    if (WARN_ON_ONCE!(!mops)) {
// goto;
    }
    if (!mops.isolate_page(page, mode)) {
// goto;
    }
// Driver shouldn't use the isolated flag
    VM_WARN_ON_ONCE_PAGE(PageMovableOpsIsolated(page), page);
    SetPageMovableOpsIsolated(page);
    folio_unlock(folio);
    return true;
// label;
    folio_unlock(folio);
// label;
    folio_put(folio);
// label;
    return false;
    }
//
// putback_movable_ops_page - putback an isolated movable_ops page
// @page: The isolated page.
//
// Putback an isolated movable_ops page.
//
// After the page was putback, it might get freed instantly.
//
#[no_mangle]
unsafe extern "C" fn putback_movable_ops_page(page: *mut page) {
//
// TODO: these pages will not be folios in the future. All
// folio dependencies will have to be removed.
//
    let mut folio = page_folio(page);
    VM_WARN_ON_ONCE_PAGE(!page_has_movable_ops(page), page);
    VM_WARN_ON_ONCE_PAGE(!PageMovableOpsIsolated(page), page);
    folio_lock(folio);
    page_movable_ops(page).putback_page(page);
    ClearPageMovableOpsIsolated(page);
    folio_unlock(folio);
    folio_put(folio);
    }
//
// migrate_movable_ops_page - migrate an isolated movable_ops page
// @dst: The destination page.
// @src: The source page.
// @mode: The migration mode.
//
// Migrate an isolated movable_ops page.
//
// If the src page was already released by its owner, the src page is
// un-isolated (putback) and migration succeeds; the migration core will be the
// owner of both pages.
//
// If the src page was not released by its owner and the migration was
// successful, the owner of the src page and the dst page are swapped and
// the src page is un-isolated.
//
// If migration fails, the ownership stays unmodified and the src page
// remains isolated: migration may be retried later or the page can be putback.
//
// TODO: migration core will treat both pages as folios and lock them before
// this call to unlock them after this call. Further, the folio refcounts on
// src and dst are also released by migration core. These pages will not be
// folios in the future, so that must be reworked.
//
// Returns 0 on success, otherwise a negative error code.
//
#[no_mangle]
pub unsafe extern "C" fn migrate_movable_ops_page(dst: *mut page, src: *mut page, mode: migrate_mode) -> c_int {
    let mut rc = 0;
    VM_WARN_ON_ONCE_PAGE(!page_has_movable_ops(src), src);
    VM_WARN_ON_ONCE_PAGE(!PageMovableOpsIsolated(src), src);
    rc = page_movable_ops(src).migrate_page(dst, src, mode);
    if (!rc) {
    ClearPageMovableOpsIsolated(src);
    }
    return rc;
    }
//
// Put previously isolated pages back onto the appropriate lists
// from where they were once taken off for compaction/migration.
//
// This function shall be used whenever the isolated pageset has been
// built from lru, balloon, hugetlbfs page. See isolate_migratepages_range()
// and folio_isolate_hugetlb().
//
#[no_mangle]
pub unsafe extern "C" fn putback_movable_pages(l: *mut list_head) {
pub static mut folio: *mut c_void = core::ptr::null_mut();
pub static mut folio2: *mut c_void = core::ptr::null_mut();
    list_for_each_entry_safe(folio, folio2, l, lru) {
    if (unlikely(folio_test_hugetlb(folio))) {
    folio_putback_hugetlb(folio);
    continue;
    }
    list_del(&folio.lru);
    if (unlikely(page_has_movable_ops(&folio.page))) {
    putback_movable_ops_page(&folio.page);
    } else {
    node_stat_mod_folio(folio, NR_ISOLATED_ANON +
    folio_is_file_lru(folio), -folio_nr_pages(folio));
    folio_putback_lru(folio);
    }
    }
    }
// Must be called with an elevated refcount on the non-hugetlb folio
#[no_mangle]
pub unsafe extern "C" fn isolate_folio_to_list(folio: *mut folio, list: *mut list_head) -> bool {
    if (folio_test_hugetlb(folio)) {
    return folio_isolate_hugetlb(folio, list);
    }
    if (page_has_movable_ops(&folio.page)) {
    if (!isolate_movable_ops_page(&folio.page,
    ISOLATE_UNEVICTABLE)) {
    return false;
    }
    } else {
    if (!folio_isolate_lru(folio)) {
    return false;
    }
    node_stat_add_folio(folio, NR_ISOLATED_ANON +
    folio_is_file_lru(folio));
    }
    list_add(&folio.lru, list);
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn try_to_map_unused_to_zeropage(pvmw: *mut page_vma_mapped_walk, folio: *mut folio, old_pte: pte_t, idx: c_ulong) -> bool {
    let mut page = folio_page(folio, idx);
    let mut newpte;
    if (PageCompound(page) || PageHWPoison(page)) {
    return false;
    }
    VM_BUG_ON_PAGE(!PageAnon(page), page);
    VM_BUG_ON_PAGE(!PageLocked(page), page);
    VM_BUG_ON_PAGE(pte_present(old_pte), page);
    VM_WARN_ON_ONCE_FOLIO(folio_is_device_private(folio), folio);
    if (folio_test_mlocked(folio) || (pvmw.vma.vm_flags & VM_LOCKED) ||
    mm_forbids_zeropage(pvmw.vma.vm_mm)) {
    return false;
    }
//
// The pmd entry mapping the old thp was flushed and the pte mapping
// this subpage has been non present. If the subpage is only zero-filled
// then map it to the shared zeropage.
//
    if (!pages_identical(page, ZERO_PAGE(0))) {
    return false;
    }
    newpte = pte_mkspecial(pfn_pte(zero_pfn(pvmw.address),
    pvmw.vma.vm_page_prot));
    if (pte_swp_soft_dirty(old_pte)) {
    newpte = pte_mksoft_dirty(newpte);
    }
    if (pte_swp_uffd(old_pte)) {
    newpte = pte_mkuffd(newpte);
    }
// See remove_migration_pte(): restore PAGE_NONE for RWP
    if (pte_swp_uffd(old_pte) && userfaultfd_rwp(pvmw.vma)) {
    newpte = pte_modify(newpte, PAGE_NONE);
    }
    set_pte_at(pvmw.vma.vm_mm, pvmw.address, pvmw.pte, newpte);
    dec_mm_counter(pvmw.vma.vm_mm, mm_counter(folio));
    return true;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rmap_walk_arg {
    pub folio: *mut folio,
    pub map_unused_to_zeropage: bool,
}

//
// Restore a potential migration pte to a working pte entry
//
#[no_mangle]
pub unsafe extern "C" fn remove_migration_pte(folio: *mut folio, vma: *mut vm_area_struct, addr: c_ulong, arg: *mut c_void) -> bool {
    let mut rmap_walk_arg = arg;
pub static mut pvmw: usize = 0;
    while (page_vma_mapped_walk(&pvmw)) {
pub static mut rmap_flags: rmap_t = 0;
pub static mut idx: c_ulong = 0;
    let mut entry;
pub static mut new: *mut c_void = core::ptr::null_mut();
    let mut old_pte;
    let mut pte;

// PMD-mapped THP migration entry
    if (!pvmw.pte) {
    VM_BUG_ON_FOLIO(folio_test_hugetlb(folio) ||
    !folio_test_pmd_mappable(folio), folio);
    remove_migration_pmd(&pvmw, folio);
    continue;
    }

    if (folio_test_hugetlb(folio)) {
    old_pte = huge_ptep_get(vma.vm_mm, pvmw.address,
    pvmw.pte);
    }
    else {
    old_pte = ptep_get(pvmw.pte);
    }
    entry = softleaf_from_pte(old_pte);
    if (folio_test_large(folio) && !folio_test_hugetlb(folio)) {
    idx = softleaf_to_pfn(entry) - pvmw.pfn;
    }
    if (rmap_walk_arg.map_unused_to_zeropage &&
    try_to_map_unused_to_zeropage(&pvmw, folio, old_pte, idx)) {
    continue;
    }
    folio_get(folio);
    new = folio_page(folio, idx);
    pte = mk_pte(new, READ_ONCE(vma.vm_page_prot));
    if (!softleaf_is_migration_young(entry)) {
    pte = pte_mkold(pte);
    }
    if (folio_test_dirty(folio) && softleaf_is_migration_dirty(entry)) {
    pte = pte_mkdirty(pte);
    }
    if (pte_swp_soft_dirty(old_pte)) {
    pte = pte_mksoft_dirty(pte);
    }
    else {
    pte = pte_clear_soft_dirty(pte);
    }
    if (softleaf_is_migration_write(entry)) {
    pte = pte_mkwrite(pte, vma);
    }

    else if (pte_swp_uffd(old_pte)) {
    pte = pte_mkuffd(pte);
    }
// See do_swap_page(): restore PAGE_NONE for RWP
    if (pte_swp_uffd(old_pte) && userfaultfd_rwp(vma)) {
    pte = pte_modify(pte, PAGE_NONE);
    }
    if (folio_test_anon(folio) && !softleaf_is_migration_read(entry)) {
    rmap_flags |= RMAP_EXCLUSIVE;
    }
    if (unlikely(is_device_private_page(new))) {
    if (pte_write(pte)) {
    entry = make_writable_device_private_entry(
    page_to_pfn(new));
    }
    else {
    entry = make_readable_device_private_entry(
    page_to_pfn(new));
    }
    pte = softleaf_to_pte(entry);
    if (pte_swp_soft_dirty(old_pte)) {
    pte = pte_swp_mksoft_dirty(pte);
    }
    if (pte_swp_uffd(old_pte)) {
    pte = pte_swp_mkuffd(pte);
    }
    }

    if (folio_test_hugetlb(folio)) {
    let mut h = hstate_vma(vma);
pub static mut shift: c_uint = 0;
pub static mut psize: c_ulong = 0;
    pte = arch_make_huge_pte(pte, shift, vma.vm_flags);
    if (folio_test_anon(folio)) {
    hugetlb_add_anon_rmap(folio, vma, pvmw.address,
    rmap_flags);
    }
    else {
    hugetlb_add_file_rmap(folio);
    }
    set_huge_pte_at(vma.vm_mm, pvmw.address, pvmw.pte, pte,
    psize);
    } else {

    {
    }
    if (folio_test_anon(folio)) {
    folio_add_anon_rmap_pte(folio, new, vma,
    pvmw.address, rmap_flags);
    }
    else {
    folio_add_file_rmap_pte(folio, new, vma);
    }
    set_pte_at(vma.vm_mm, pvmw.address, pvmw.pte, pte);
    }
    if (READ_ONCE(vma.vm_flags) & VM_LOCKED) {
    mlock_drain_local();
    }
    trace_remove_migration_pte(pvmw.address, pte_val(pte),
    compound_order(new));
// No need to invalidate - it was non-present before
    update_mmu_cache(vma, pvmw.address, pvmw.pte);
    }
    return true;
    }
//
// Get rid of all migration entries and replace them by
// references to the indicated page.
//
#[no_mangle]
pub unsafe extern "C" fn remove_migration_ptes(src: *mut folio, dst: *mut folio, flags: ttu_flags) {
pub static mut rmap_walk_arg: usize = 0;
pub static mut rmap_walk_control: usize = 0;
    VM_BUG_ON_FOLIO((flags & TTU_USE_SHARED_ZEROPAGE) && (src != dst), src);
    if (flags & TTU_RMAP_LOCKED) {
    rmap_walk_locked(dst, &rwc);
    }
    else {
    rmap_walk(dst, &rwc);
    }
    }
//
// Something used the pte of a page under migration. We need to
// get to the page and wait until migration is finished.
// When we return from this function the fault will be retried.
//
#[no_mangle]
pub unsafe extern "C" fn migration_entry_wait(mm: *mut mm_struct, pmd: *mut pmd_t, address: c_ulong) {
pub static mut ptl: *mut c_void = core::ptr::null_mut();
pub static mut ptep: *mut c_void = core::ptr::null_mut();
    let mut pte;
    let mut entry;
    ptep = pte_offset_map_lock(mm, pmd, address, &ptl);
    if (!ptep) {
    return;
    }
    pte = ptep_get(ptep);
    pte_unmap(ptep);
    if (pte_none(pte) || pte_present(pte)) {
// goto;
    }
    entry = softleaf_from_pte(pte);
    if (!softleaf_is_migration(entry)) {
// goto;
    }
    softleaf_entry_wait_on_locked(entry, ptl);
    return;
// label;
    spin_unlock(ptl);
    }

//
// The vma read lock must be held upon entry. Holding that lock prevents either
// the pte or the ptl from being freed.
//
// This function will release the vma lock before returning.
//
#[no_mangle]
pub unsafe extern "C" fn migration_entry_wait_huge(vma: *mut vm_area_struct, addr: c_ulong, ptep: *mut pte_t) {
    let mut ptl = huge_pte_lockptr(hstate_vma(vma), vma.vm_mm, ptep);
    let mut entry;
    let mut pte;
    hugetlb_vma_assert_locked(vma);
    spin_lock(ptl);
    pte = huge_ptep_get(vma.vm_mm, addr, ptep);
    if (huge_pte_none(pte)) {
// goto;
    }
    entry = softleaf_from_pte(pte);
    if (softleaf_is_migration(entry)) {
//
// If migration entry existed, safe to release vma lock
// here because the pgtable page won't be freed without the
// pgtable lock released.  See comment right above pgtable
// lock release in softleaf_entry_wait_on_locked().
//
    hugetlb_vma_unlock_read(vma);
    softleaf_entry_wait_on_locked(entry, ptl);
    return;
    }
// label;
    spin_unlock(ptl);
    hugetlb_vma_unlock_read(vma);
    }

#[no_mangle]
pub unsafe extern "C" fn pmd_migration_entry_wait(mm: *mut mm_struct, pmd: *mut pmd_t) {
pub static mut ptl: *mut c_void = core::ptr::null_mut();
    ptl = pmd_lock(mm, pmd);
    if (!pmd_is_migration_entry(*pmd)) {
// goto;
    }
    softleaf_entry_wait_on_locked(softleaf_from_pmd(*pmd), ptl);
    return;
// label;
    spin_unlock(ptl);
    }

//
// Replace the folio in the mapping.
//
// The number of remaining references must be:
// 1 for anonymous folios without a mapping
// 2 for folios with a mapping
// 3 for folios with a mapping and the private flag set.
//
#[no_mangle]
pub unsafe extern "C" fn __folio_migrate_mapping(mapping: *mut address_space, newfolio: *mut folio, folio: *mut folio, expected_count: c_int) -> c_int {
    XA_STATE(xas, &mapping.i_pages, folio.index);
    let mut ci = core::ptr::null_mut();
    let mut oldzone = core::ptr::null_mut();
    let mut newzone = core::ptr::null_mut();
    let mut dirty = 0;
pub static mut nr: c_long = 0;
    if (!mapping) {
// Take off deferred split queue while frozen and memcg set
    if (folio_test_large(folio) &&
    folio_test_large_rmappable(folio)) {
    if (!folio_ref_freeze(folio, expected_count)) {
    return -EAGAIN;
    }
    folio_unqueue_deferred_split(folio);
    folio_ref_unfreeze(folio, expected_count);
    }
// No turning back from here
    newfolio.index = folio.index;
    newfolio.mapping = folio.mapping;
    if (folio_test_anon(folio) && folio_test_large(folio) &&
    !folio_test_hugetlb(folio)) {
    mod_mthp_stat(folio_order(folio), MTHP_STAT_NR_ANON, 1);
    }
    if (folio_test_swapbacked(folio)) {
    __folio_set_swapbacked(newfolio);
    }
    return 0;
    }
    oldzone = folio_zone(folio);
    newzone = folio_zone(newfolio);
    if (folio_test_swapcache(folio)) {
    ci = swap_cluster_get_and_lock_irq(folio);
    }
    else {
    xas_lock_irq(&xas);
    }
    if (!folio_ref_freeze(folio, expected_count)) {
    if (ci) {
    swap_cluster_unlock_irq(ci);
    }
    else {
    xas_unlock_irq(&xas);
    }
    return -EAGAIN;
    }
// Take off deferred split queue while frozen and memcg set
    folio_unqueue_deferred_split(folio);
//
// Now we know that no one else is looking at the folio:
// no turning back from here.
//
    newfolio.index = folio.index;
    newfolio.mapping = folio.mapping;
    if (folio_test_anon(folio) && folio_test_large(folio)) {
    mod_mthp_stat(folio_order(folio), MTHP_STAT_NR_ANON, 1);
    }
    folio_ref_add(newfolio, nr); /* add cache reference */
    if (folio_test_swapbacked(folio)) {
    __folio_set_swapbacked(newfolio);
    }
    if (folio_test_swapcache(folio)) {
    folio_set_swapcache(newfolio);
    newfolio.private = folio_get_private(folio);
    }
// Move dirty while folio refs frozen and newfolio not yet exposed
    dirty = folio_test_dirty(folio);
    if (dirty) {
    folio_clear_dirty(folio);
    folio_set_dirty(newfolio);
    }
    if (folio_test_swapcache(folio)) {
    __swap_cache_replace_folio(ci, folio, newfolio);
    }
    else {
    xas_store(&xas, newfolio);
    }
//
// Drop cache reference from old folio by unfreezing
// to one less reference.
// We know this isn't the last reference.
//
    folio_ref_unfreeze(folio, expected_count - nr);
// Leave irq disabled to prevent preemption while updating stats
    if (ci) {
    swap_cluster_unlock(ci);
    }
    else {
    xas_unlock(&xas);
    }
//
// If moved to a different zone then also account
// the folio for that zone. Other VM counters will be
// taken care of when we establish references to the
// new folio and drop references to the old folio.
//
// Note that anonymous folios are accounted for
// via NR_FILE_PAGES and NR_ANON_MAPPED if they
// are mapped to swap space.
//
    if (newzone != oldzone) {
    let mut old_lruvec = core::ptr::null_mut();
    let mut new_lruvec = core::ptr::null_mut();
pub static mut memcg: *mut c_void = core::ptr::null_mut();
    rcu_read_lock();
    memcg = folio_memcg(folio);
    old_lruvec = mem_cgroup_lruvec(memcg, oldzone.zone_pgdat);
    new_lruvec = mem_cgroup_lruvec(memcg, newzone.zone_pgdat);
    mod_lruvec_state(old_lruvec, NR_FILE_PAGES, -nr);
    mod_lruvec_state(new_lruvec, NR_FILE_PAGES, nr);
    if (folio_test_swapbacked(folio) && !folio_test_swapcache(folio)) {
    mod_lruvec_state(old_lruvec, NR_SHMEM, -nr);
    mod_lruvec_state(new_lruvec, NR_SHMEM, nr);
    if (folio_test_pmd_mappable(folio)) {
    mod_lruvec_state(old_lruvec, NR_SHMEM_THPS, -nr);
    mod_lruvec_state(new_lruvec, NR_SHMEM_THPS, nr);
    }
    }

    if (folio_test_swapcache(folio)) {
    mod_lruvec_state(old_lruvec, NR_SWAPCACHE, -nr);
    mod_lruvec_state(new_lruvec, NR_SWAPCACHE, nr);
    }

    if (dirty && mapping_can_writeback(mapping)) {
    mod_lruvec_state(old_lruvec, NR_FILE_DIRTY, -nr);
    __mod_zone_page_state(oldzone, NR_ZONE_WRITE_PENDING, -nr);
    mod_lruvec_state(new_lruvec, NR_FILE_DIRTY, nr);
    __mod_zone_page_state(newzone, NR_ZONE_WRITE_PENDING, nr);
    }
    rcu_read_unlock();
    }
    local_irq_enable();
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn folio_migrate_mapping(mapping: *mut address_space, newfolio: *mut folio, folio: *mut folio, extra_count: c_int) -> c_int {
pub static mut expected_count: c_int = 0;
    if (folio_ref_count(folio) != expected_count) {
    return -EAGAIN;
    }
    return __folio_migrate_mapping(mapping, newfolio, folio, expected_count);
    }
    EXPORT_SYMBOL(folio_migrate_mapping);
//
// The expected number of remaining references is the same as that
// of folio_migrate_mapping().
//
#[no_mangle]
pub unsafe extern "C" fn migrate_huge_page_move_mapping(mapping: *mut address_space, dst: *mut folio, src: *mut folio) -> c_int {
    XA_STATE(xas, &mapping.i_pages, src.index);
    int rc, expected_count = folio_expected_ref_count(src) + 1;
    if (folio_ref_count(src) != expected_count) {
    return -EAGAIN;
    }
    rc = folio_mc_copy(dst, src);
    if (unlikely(rc)) {
    return rc;
    }
    xas_lock_irq(&xas);
    if (!folio_ref_freeze(src, expected_count)) {
    xas_unlock_irq(&xas);
    return -EAGAIN;
    }
    dst.index = src.index;
    dst.mapping = src.mapping;
    folio_ref_add(dst, folio_nr_pages(dst));
    xas_store(&xas, dst);
    folio_ref_unfreeze(src, expected_count - folio_nr_pages(src));
    xas_unlock_irq(&xas);
    return 0;
    }
//
// Copy the flags and some other ancillary information
//
#[no_mangle]
pub unsafe extern "C" fn folio_migrate_flags(newfolio: *mut folio, folio: *mut folio) {
    let mut cpupid = 0;
    if (folio_test_referenced(folio)) {
    folio_set_referenced(newfolio);
    }
    if (folio_test_uptodate(folio)) {
    folio_mark_uptodate(newfolio);
    }
    if (folio_test_clear_active(folio)) {
    VM_BUG_ON_FOLIO(folio_test_unevictable(folio), folio);
    folio_set_active(newfolio);
    } else if (folio_test_clear_unevictable(folio)) {
    folio_set_unevictable(newfolio);
    }
    if (folio_test_workingset(folio)) {
    folio_set_workingset(newfolio);
    }
    if (folio_test_checked(folio)) {
    folio_set_checked(newfolio);
    }
//
// PG_anon_exclusive (-> PG_mappedtodisk) is always migrated via
// migration entries. We can still have PG_anon_exclusive set on an
// effectively unmapped and unreferenced first sub-pages of an
// anonymous THP: we can simply copy it here via PG_mappedtodisk.
//
    if (folio_test_mappedtodisk(folio)) {
    folio_set_mappedtodisk(newfolio);
    }
// Move dirty on pages not done by folio_migrate_mapping()
    if (folio_test_dirty(folio)) {
    folio_set_dirty(newfolio);
    }
    if (folio_test_young(folio)) {
    folio_set_young(newfolio);
    }
    if (folio_test_idle(folio)) {
    folio_set_idle(newfolio);
    }
    folio_migrate_refs(newfolio, folio);
//
// Copy NUMA information to the new page, to prevent over-eager
// future migrations of this same page.
//
    cpupid = folio_xchg_last_cpupid(folio, -1);
//
// For memory tiering mode, when migrate between slow and fast
// memory node, reset cpupid, because that is used to record
// page access time in slow memory node.
//
    if (sysctl_numa_balancing_mode & NUMA_BALANCING_MEMORY_TIERING) {
pub static mut f_toptier: bool = false;
pub static mut t_toptier: bool = false;
    if (f_toptier != t_toptier) {
    cpupid = -1;
    }
    }
    folio_xchg_last_cpupid(newfolio, cpupid);
    folio_migrate_ksm(newfolio, folio);
//
// Please do not reorder this without considering how mm/ksm.c's
// ksm_get_folio() depends upon ksm_migrate_page() and the
// swapcache flag.
//
    if (folio_test_swapcache(folio)) {
    folio_clear_swapcache(folio);
    }
    folio_clear_private(folio);
// page->private contains hugetlb specific flags
    if (!folio_test_hugetlb(folio)) {
    folio.private = core::ptr::null_mut();
    }
//
// If any waiters have accumulated on the new page then
// wake them up.
//
    if (folio_test_writeback(newfolio)) {
    folio_end_writeback(newfolio);
    }
//
// PG_readahead shares the same bit with PG_reclaim.  The above
// end_page_writeback() may clear PG_readahead mistakenly, so set the
// bit after that.
//
    if (folio_test_readahead(folio)) {
    folio_set_readahead(newfolio);
    }
    folio_copy_owner(newfolio, folio);
    pgalloc_tag_swap(newfolio, folio);
    mem_cgroup_migrate(folio, newfolio);
    }
    EXPORT_SYMBOL(folio_migrate_flags);
//
// Migration functions
//
#[no_mangle]
pub unsafe extern "C" fn __migrate_folio(mapping: *mut address_space, dst: *mut folio, src: *mut folio, src_private: *mut c_void, mode: migrate_mode) -> c_int {
    int rc, expected_count = folio_expected_ref_count(src) + 1;
// Check whether src does not have extra refs before we do more work
    if (folio_ref_count(src) != expected_count) {
    return -EAGAIN;
    }
    rc = folio_mc_copy(dst, src);
    if (unlikely(rc)) {
    return rc;
    }
    rc = __folio_migrate_mapping(mapping, dst, src, expected_count);
    if (rc) {
    return rc;
    }
    if (src_private) {
    folio_attach_private(dst, folio_detach_private(src));
    }
    folio_migrate_flags(dst, src);
    return 0;
    }
//
// migrate_folio() - Simple folio migration.
// @mapping: The address_space containing the folio.
// @dst: The folio to migrate the data to.
// @src: The folio containing the current data.
// @mode: How to migrate the folio.
//
// Common logic to directly migrate a single LRU folio suitable for
// folios that do not have private data.
//
// Folios are locked upon entry and exit.
//
#[no_mangle]
pub unsafe extern "C" fn migrate_folio(mapping: *mut address_space, dst: *mut folio, src: *mut folio, mode: migrate_mode) -> c_int {
    BUG_ON!(folio_test_writeback(src));	/* Writeback must be complete */
    return __migrate_folio(mapping, dst, src, core::ptr::null_mut(), mode);
    }
    EXPORT_SYMBOL(migrate_folio);

// Returns true if all buffers are successfully locked
#[no_mangle]
pub unsafe extern "C" fn buffer_migrate_lock_buffers(head: *mut buffer_head, mode: migrate_mode) -> bool {
    let mut bh = head;
pub static mut failed_bh: *mut c_void = core::ptr::null_mut();
    do {
    if (!trylock_buffer(bh)) {
    if (mode == MIGRATE_ASYNC) {
// goto;
    }
    if (mode == MIGRATE_SYNC_LIGHT && !buffer_uptodate(bh)) {
// goto;
    }
    lock_buffer(bh);
    }
    bh = bh.b_this_page;
    } while (bh != head);
    return true;
// label;
// We failed to lock the buffer and cannot stall.
    failed_bh = bh;
    bh = head;
    while (bh != failed_bh) {
    unlock_buffer(bh);
    bh = bh.b_this_page;
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn __buffer_migrate_folio(mapping: *mut address_space, dst: *mut folio, src: *mut folio, mode: migrate_mode, check_refs: bool) -> c_int {
    let mut bh = core::ptr::null_mut();
    let mut head = core::ptr::null_mut();
    let mut rc = 0;
    let mut expected_count = 0;
    head = folio_buffers(src);
    if (!head) {
    return migrate_folio(mapping, dst, src, mode);
    }
// Check whether page does not have extra refs before we do more work
    expected_count = folio_expected_ref_count(src) + 1;
    if (folio_ref_count(src) != expected_count) {
    return -EAGAIN;
    }
    if (!buffer_migrate_lock_buffers(head, mode)) {
    return -EAGAIN;
    }
    if (check_refs) {
    let mut busy = 0;
    let mut migrating = 0;
pub static mut invalidated: bool = false;
    migrating = test_and_set_bit_lock(BH_Migrate, &head.b_state);
    VM_WARN_ON_ONCE(migrating);
// label;
    busy = false;
    spin_lock(&mapping.i_private_lock);
    bh = head;
    do {
    if (atomic_read(&bh.b_count)) {
    busy = true;
    break;
    }
    bh = bh.b_this_page;
    } while (bh != head);
    spin_unlock(&mapping.i_private_lock);
    if (busy) {
    if (invalidated) {
    rc = -EAGAIN;
// goto;
    }
    invalidate_bh_lrus();
    invalidated = true;
// goto;
    }
    }
    rc = filemap_migrate_folio(mapping, dst, src, mode);
    if (rc) {
// goto;
    }
    bh = head;
    do {
    folio_set_bh(bh, dst, bh_offset(bh));
    bh = bh.b_this_page;
    } while (bh != head);
// label;
    if (check_refs) {
    clear_bit_unlock(BH_Migrate, &head.b_state);
    }
    bh = head;
    do {
    unlock_buffer(bh);
    bh = bh.b_this_page;
    } while (bh != head);
    return rc;
    }
//
// buffer_migrate_folio() - Migration function for folios with buffers.
// @mapping: The address space containing @src.
// @dst: The folio to migrate to.
// @src: The folio to migrate from.
// @mode: How to migrate the folio.
//
// This function can only be used if the underlying filesystem guarantees
// that no other references to @src exist. For example attached buffer
// heads are accessed only under the folio lock.  If your filesystem cannot
// provide this guarantee, buffer_migrate_folio_norefs() may be more
// appropriate.
//
// Return: 0 on success or a negative errno on failure.
//
#[no_mangle]
pub unsafe extern "C" fn buffer_migrate_folio(mapping: *mut address_space, dst: *mut folio, src: *mut folio, mode: migrate_mode) -> c_int {
    return __buffer_migrate_folio(mapping, dst, src, mode, false);
    }
    EXPORT_SYMBOL(buffer_migrate_folio);
//
// buffer_migrate_folio_norefs() - Migration function for folios with buffers.
// @mapping: The address space containing @src.
// @dst: The folio to migrate to.
// @src: The folio to migrate from.
// @mode: How to migrate the folio.
//
// Like buffer_migrate_folio() except that this variant is more careful
// and checks that there are also no buffer head references. This function
// is the right one for mappings where buffer heads are directly looked
// up and referenced (such as block device mappings).
//
// Return: 0 on success or a negative errno on failure.
//
#[no_mangle]
pub unsafe extern "C" fn buffer_migrate_folio_norefs(mapping: *mut address_space, dst: *mut folio, src: *mut folio, mode: migrate_mode) -> c_int {
    return __buffer_migrate_folio(mapping, dst, src, mode, true);
    }
    EXPORT_SYMBOL_GPL(buffer_migrate_folio_norefs);

#[no_mangle]
pub unsafe extern "C" fn filemap_migrate_folio(mapping: *mut address_space, dst: *mut folio, src: *mut folio, mode: migrate_mode) -> c_int {
    return __migrate_folio(mapping, dst, src, folio_get_private(src), mode);
    }
    EXPORT_SYMBOL_GPL(filemap_migrate_folio);
//
// Default handling if a filesystem does not provide a migration function.
//
#[no_mangle]
pub unsafe extern "C" fn fallback_migrate_folio(mapping: *mut address_space, dst: *mut folio, src: *mut folio, mode: migrate_mode) -> c_int {
    WARN_ONCE(mapping.a_ops.writepages,
    "%ps does not implement migrate_folio\n",
    mapping.a_ops);
    if (folio_test_dirty(src)) {
    return -EBUSY;
    }
//
// Filesystem may have private data at folio->private that we
// can't migrate automatically.
//
    if (!filemap_release_folio(src, GFP_KERNEL)) {
pub static mut mode: return = 0;
    }
    return migrate_folio(mapping, dst, src, mode);
    }
//
// Move a src folio to a newly allocated dst folio.
//
// The src and dst folios are locked and the src folios was unmapped from
// the page tables.
//
// On success, the src folio was replaced by the dst folio.
//
// Return value:
// < 0 - error code
// 0 - success
//
#[no_mangle]
pub unsafe extern "C" fn move_to_new_folio(dst: *mut folio, src: *mut folio, mode: migrate_mode) -> c_int {
    let mut mapping = folio_mapping(src);
pub static mut rc: c_int = 0;
    VM_BUG_ON_FOLIO(!folio_test_locked(src), src);
    VM_BUG_ON_FOLIO(!folio_test_locked(dst), dst);
    if (!mapping) {
    rc = migrate_folio(mapping, dst, src, mode);
    }

    else if (mapping_inaccessible(mapping)) {
    rc = -EOPNOTSUPP;
    }

    else if (mapping.a_ops.migrate_folio) {
//
// Most folios have a mapping and most filesystems
// provide a migrate_folio callback. Anonymous folios
// are part of swap space which also has its own
// migrate_folio callback. This is the most common path
// for page migration.
//
    rc = mapping.a_ops.migrate_folio(mapping, dst, src,
    mode);
    }
    else {
    rc = fallback_migrate_folio(mapping, dst, src, mode);
    }
    if (!rc) {
//
// For pagecache folios, src->mapping must be cleared before src
// is freed. Anonymous folios must stay anonymous until freed.
//
    if (!folio_test_anon(src)) {
    src.mapping = core::ptr::null_mut();
    }
    if (likely(!folio_is_zone_device(dst))) {
    flush_dcache_folio(dst);
    }
    }
    return rc;
    }
//
// To record some information during migration, we use the migrate_info
// field of struct folio of the newly allocated destination folio.
// This is safe because nobody is using it except us.
//
    enum {
    FOLIO_WAS_MAPPED = BIT(0),
    FOLIO_WAS_MLOCKED = BIT(1),
    FOLIO_OLD_STATES = FOLIO_WAS_MAPPED | FOLIO_WAS_MLOCKED,
    };
#[no_mangle]
pub unsafe extern "C" fn __migrate_folio_record(dst: *mut folio, old_folio_state: c_int, anon_vma: *mut anon_vma) {
    dst.migrate_info = (unsigned long)anon_vma | old_folio_state;
    }
#[no_mangle]
pub unsafe extern "C" fn __migrate_folio_extract(dst: *mut folio, old_folio_state: *mut c_int, anon_vmap: *mut *mut anon_vma) {
pub static mut info: c_ulong = 0;
// anon_vmap = (info & ~FOLIO_OLD_STATES);
// old_folio_state = info & FOLIO_OLD_STATES;
    dst.migrate_info = 0;
    }
// Restore the source folio to the original state upon failure
#[no_mangle]
pub unsafe extern "C" fn migrate_folio_undo_src(src: *mut folio, was_mapped: c_int, anon_vma: *mut anon_vma, locked: bool, ret: *mut list_head) {
    if (was_mapped) {
    remove_migration_ptes(src, src, 0);
    }
// Drop an anon_vma reference if we took one
    if (anon_vma) {
    put_anon_vma(anon_vma);
    }
    if (locked) {
    folio_unlock(src);
    }
    if (ret) {
    list_move_tail(&src.lru, ret);
    }
    }
// Restore the destination folio to the original state upon failure
#[no_mangle]
pub unsafe extern "C" fn migrate_folio_undo_dst(dst: *mut folio, locked: bool, put_new_folio: free_folio_t, private: c_ulong) {
    if (locked) {
    folio_unlock(dst);
    }
    if (put_new_folio) {
    put_new_folio(dst, private);
    }
    else {
    folio_put(dst);
    }
    }
// Cleanup src folio upon migration success
#[no_mangle]
pub unsafe extern "C" fn migrate_folio_done(src: *mut folio, reason: migrate_reason) {
    if (likely(!page_has_movable_ops(&src.page)) && reason != MR_DEMOTION) {
    mod_node_page_state(folio_pgdat(src), NR_ISOLATED_ANON +
    folio_is_file_lru(src), -folio_nr_pages(src));
    }
    if (reason != MR_MEMORY_FAILURE) {
// We release the page in page_handle_poison.
    folio_put(src);
    }
    }
// Obtain the lock on page, remove all ptes.
#[no_mangle]
pub unsafe extern "C" fn migrate_folio_unmap(get_new_folio: new_folio_t, put_new_folio: free_folio_t, private: c_ulong, src: *mut folio, dstp: *mut *mut folio, mode: migrate_mode, ret: *mut list_head) -> c_int {
pub static mut dst: *mut c_void = core::ptr::null_mut();
pub static mut rc: c_int = 0;
pub static mut old_folio_state: c_int = 0;
    let mut anon_vma = core::ptr::null_mut();
pub static mut locked: bool = false;
pub static mut dst_locked: bool = false;
    dst = get_new_folio(src, private);
    if (!dst) {
    return -ENOMEM;
    }
// dstp = dst;
    dst.migrate_info = 0;
    if (!folio_trylock(src)) {
    if (mode == MIGRATE_ASYNC) {
// goto;
    }
//
// It's not safe for direct compaction to call lock_page.
// For example, during page readahead pages are added locked
// to the LRU. Later, when the IO completes the pages are
// marked uptodate and unlocked. However, the queueing
// could be merging multiple pages for one bio (e.g.
// mpage_readahead). If an allocation happens for the
// second or third page, the process can end up locking
// the same page twice and deadlocking. Rather than
// trying to be clever about what pages can be locked,
// avoid the use of lock_page for direct compaction
// altogether.
//
    if (current.flags & PF_MEMALLOC) {
// goto;
    }
//
// In "light" mode, we can wait for transient locks (eg
// inserting a page into the page table), but it's not
// worth waiting for I/O.
//
    if (mode == MIGRATE_SYNC_LIGHT && !folio_test_uptodate(src)) {
// goto;
    }
    folio_lock(src);
    }
    locked = true;
    if (folio_test_mlocked(src)) {
    old_folio_state |= FOLIO_WAS_MLOCKED;
    }
    if (folio_test_writeback(src)) {
//
// Only in the case of a full synchronous migration is it
// necessary to wait for writeback. In the async case,
// the retry loop is too short and in the sync-light case,
// the overhead of stalling is too much
//
    match (mode) {
    MIGRATE_SYNC => {
    // break;
    }
    _ => {
    rc = -EBUSY;
// goto;
    }
    }
    folio_wait_writeback(src);
    }
//
// By try_to_migrate(), src->mapcount goes down to 0 here. In this case,
// we cannot notice that anon_vma is freed while we migrate a page.
// This get_anon_vma() delays freeing anon_vma pointer until the end
// of migration. File cache pages are no problem because of page_lock()
// File Caches may use write_page() or lock_page() in migration, then,
// just care Anon page here.
//
// Only folio_get_anon_vma() understands the subtleties of
// getting a hold on an anon_vma from outside one of its mms.
// But if we cannot get anon_vma, then we won't need it anyway,
// because that implies that the anon page is no longer mapped
// (and cannot be remapped so long as we hold the page lock).
//
    if (folio_test_anon(src) && !folio_test_ksm(src)) {
    anon_vma = folio_get_anon_vma(src);
    }
//
// Block others from accessing the new page when we get around to
// establishing additional references. We are usually the only one
// holding a reference to dst at this point. We used to have a BUG
// here if folio_trylock(dst) fails, but would like to allow for
// cases where there might be a race with the previous use of dst.
// This is much like races on refcount of oldpage: just don't BUG().
//
    if (unlikely(!folio_trylock(dst))) {
// goto;
    }
    dst_locked = true;
    if (unlikely(page_has_movable_ops(&src.page))) {
    __migrate_folio_record(dst, old_folio_state, anon_vma);
    return 0;
    }
//
// Corner case handling:
// 1. When a new swap-cache page is read into, it is added to the LRU
// and treated as swapcache but it has no rmap yet.
// Calling try_to_unmap() against a src->mapping==NULL page will
// trigger a BUG.  So handle it here.
// 2. An orphaned page (see truncate_cleanup_page) might have
// fs-private metadata. The page can be picked up due to memory
// offlining.  Everywhere else except page reclaim, the page is
// invisible to the vm, so the page can not be migrated.  So try to
// free the metadata, so the page can be freed.
//
    if (!src.mapping) {
    if (folio_test_private(src)) {
    try_to_free_buffers(src);
// goto;
    }
    } else if (folio_mapped(src)) {
// Establish migration ptes
    VM_BUG_ON_FOLIO(folio_test_anon(src) &&
    !folio_test_ksm(src) && !anon_vma, src);
    try_to_migrate(src, mode == MIGRATE_ASYNC ? TTU_BATCH_FLUSH : 0);
    old_folio_state |= FOLIO_WAS_MAPPED;
    }
    if (!folio_mapped(src)) {
    __migrate_folio_record(dst, old_folio_state, anon_vma);
    return 0;
    }
// label;
//
// A folio that has not been unmapped will be restored to
// right list unless we want to retry.
//
    if (rc == -EAGAIN) {
    ret = core::ptr::null_mut();
    }
    migrate_folio_undo_src(src, old_folio_state & FOLIO_WAS_MAPPED,
    anon_vma, locked, ret);
    migrate_folio_undo_dst(dst, dst_locked, put_new_folio, private);
    return rc;
    }
// Migrate the folio to the newly allocated folio in dst.
#[no_mangle]
pub unsafe extern "C" fn migrate_folio_move(put_new_folio: free_folio_t, private: c_ulong, src: *mut folio, dst: *mut folio, mode: migrate_mode, reason: migrate_reason, ret: *mut list_head) -> c_int {
    let mut rc = 0;
pub static mut old_folio_state: c_int = 0;
    let mut anon_vma = core::ptr::null_mut();
pub static mut src_deferred_split: bool = false;
pub static mut src_partially_mapped: bool = false;
pub static mut prev: *mut c_void = core::ptr::null_mut();
    __migrate_folio_extract(dst, &old_folio_state, &anon_vma);
    prev = dst.lru.prev;
    list_del(&dst.lru);
    if (unlikely(page_has_movable_ops(&src.page))) {
    rc = migrate_movable_ops_page(&dst.page, &src.page, mode);
    if (rc) {
// goto;
    }
// goto;
    }
    if (folio_order(src) > 1 &&
    !data_race(list_empty(&src._deferred_list))) {
    src_deferred_split = true;
    src_partially_mapped = folio_test_partially_mapped(src);
    }
    rc = move_to_new_folio(dst, src, mode);
    if (rc) {
// goto;
    }
//
// Requeue the destination folio on the deferred split queue if
// the source was on the queue.  The source is unqueued in
// __folio_migrate_mapping(), so we recorded the state from
// before move_to_new_folio().
//
    if (src_deferred_split) {
    deferred_split_folio(dst, src_partially_mapped);
    }
//
// When successful, push dst to LRU immediately: so that if it
// turns out to be an mlocked page, remove_migration_ptes() will
// automatically build up the correct dst->mlock_count for it.
//
// We would like to do something similar for the old page, when
// unsuccessful, and other cases when a page has been temporarily
// isolated from the unevictable LRU: but this case is the easiest.
//
    folio_add_lru(dst);
    if (old_folio_state & FOLIO_WAS_MLOCKED) {
    lru_add_drain();
    }
    if (old_folio_state & FOLIO_WAS_MAPPED) {
    remove_migration_ptes(src, dst, 0);
    }
// label;
    folio_unlock(dst);
    folio_set_owner_migrate_reason(dst, reason);
//
// If migration is successful, decrease refcount of dst,
// which will not free the page because new page owner increased
// refcounter.
//
    folio_put(dst);
//
// A folio that has been migrated has all references removed
// and will be freed.
//
    list_del(&src.lru);
// Drop an anon_vma reference if we took one
    if (anon_vma) {
    put_anon_vma(anon_vma);
    }
    folio_unlock(src);
    migrate_folio_done(src, reason);
    return rc;
// label;
//
// A folio that has not been migrated will be restored to
// right list unless we want to retry.
//
    if (rc == -EAGAIN) {
    list_add(&dst.lru, prev);
    __migrate_folio_record(dst, old_folio_state, anon_vma);
    return rc;
    }
    migrate_folio_undo_src(src, old_folio_state & FOLIO_WAS_MAPPED,
    anon_vma, true, ret);
    migrate_folio_undo_dst(dst, true, put_new_folio, private);
    return rc;
    }
//
// Counterpart of migrate_folio_unmap() and migrate_folio_move() for hugetlb
// folio migration.
//
// This function doesn't wait the completion of hugepage I/O
// because there is no race between I/O and migration for hugepage.
// Note that currently hugepage I/O occurs only in direct I/O
// where no lock is held and PG_writeback is irrelevant,
// and writeback status of all subpages are counted in the reference
// count of the head page (i.e. if all subpages of a 2MB hugepage are
// under direct I/O, the reference of the head page is 512 and a bit more.)
// This means that when we try to migrate hugepage whose subpages are
// doing direct I/O, some references remain after try_to_unmap() and
// hugepage migration fails without data corruption.
//
// There is also no race when direct I/O is issued on the page under migration,
// because then pte is replaced with migration swap entry and direct I/O code
// will wait in the page fault for migration to complete.
//
#[no_mangle]
pub unsafe extern "C" fn unmap_and_move_hugetlb_folio(get_new_folio: new_folio_t, put_new_folio: free_folio_t, private: c_ulong, src: *mut folio, force: c_int, mode: migrate_mode, reason: migrate_reason, ret: *mut list_head) -> c_int {
pub static mut dst: *mut c_void = core::ptr::null_mut();
pub static mut rc: c_int = 0;
pub static mut was_mapped: c_int = 0;
    let mut anon_vma = core::ptr::null_mut();
    let mut mapping = core::ptr::null_mut();
pub static mut ttu: ttu_flags = 0;
    if (folio_ref_count(src) == 1) {
// folio was freed from under us. So we are done.
    folio_putback_hugetlb(src);
    return 0;
    }
    dst = get_new_folio(src, private);
    if (!dst) {
    return -ENOMEM;
    }
    if (!folio_trylock(src)) {
    if (!force) {
// goto;
    }
    match (mode) {
    MIGRATE_SYNC => {
    // break;
    }
    _ => {
// goto;
    }
    }
    folio_lock(src);
    }
//
// Check for folios which are in the process of being freed.  Without
// folio_mapping() set, hugetlbfs specific move folio routine will not
// be called and we could leak usage counts for subpools.
//
    if (hugetlb_folio_subpool(src) && !folio_mapping(src)) {
    rc = -EBUSY;
// goto;
    }
    if (folio_test_anon(src)) {
    anon_vma = folio_get_anon_vma(src);
    }
    if (unlikely(!folio_trylock(dst))) {
// goto;
    }
    if (folio_mapped(src)) {
    if (!folio_test_anon(src)) {
//
// In shared mappings, try_to_unmap could potentially
// call huge_pmd_unshare.  Because of this, take
// semaphore in write mode here and set TTU_RMAP_LOCKED
// to let lower levels know we have taken the lock.
//
    mapping = hugetlb_folio_mapping_lock_write(src);
    if (unlikely(!mapping)) {
// goto;
    }
    ttu = TTU_RMAP_LOCKED;
    }
    try_to_migrate(src, ttu);
    was_mapped = 1;
    }
    if (!folio_mapped(src)) {
    rc = move_to_new_folio(dst, src, mode);
    }
    if (was_mapped) {
    remove_migration_ptes(src, !rc ? dst : src, ttu);
    }
    if (ttu & TTU_RMAP_LOCKED) {
    i_mmap_unlock_write(mapping);
    }
// label;
    folio_unlock(dst);
// label;
    if (anon_vma) {
    put_anon_vma(anon_vma);
    }
    if (!rc) {
    move_hugetlb_state(src, dst, reason);
    put_new_folio = core::ptr::null_mut();
    }
// label;
    folio_unlock(src);
// label;
    if (!rc) {
    folio_putback_hugetlb(src);
    }

    else if (rc != -EAGAIN) {
    list_move_tail(&src.lru, ret);
    }
//
// If migration was not successful and there's a freeing callback,
// return the folio to that special allocator. Otherwise, simply drop
// our additional reference.
//
    if (put_new_folio) {
    put_new_folio(dst, private);
    }
    else {
    folio_put(dst);
    }
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn try_split_folio(folio: *mut folio, split_folios: *mut list_head, mode: migrate_mode) -> c_int {
    let mut rc = 0;
    if (mode == MIGRATE_ASYNC) {
    if (!folio_trylock(folio)) {
    return -EAGAIN;
    }
    } else {
    folio_lock(folio);
    }
    rc = split_folio_to_list(folio, split_folios);
    folio_unlock(folio);
    if (!rc) {
    list_move_tail(&folio.lru, split_folios);
    }
    return rc;
    }

pub const NR_MAX_BATCHED_MIGRATION: c_int = 512;

pub const NR_MAX_MIGRATE_PAGES_RETRY: c_int = 10;
pub const NR_MAX_MIGRATE_ASYNC_RETRY: c_int = 3;

    (NR_MAX_MIGRATE_PAGES_RETRY - NR_MAX_MIGRATE_ASYNC_RETRY)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct migrate_pages_stats {
    pub in: *mut *mut int nr_succeeded; / Normal and large folios migrated successfully,,
    units of base pages */
    pub in: *mut *mut int nr_failed_pages; / Normal and large folios failed to be migrated,,
    units of base pages.  Untried folios aren't counted */
//     pub /: *mut *mut int nr_thp_succeeded; / THP migrated successfully,
//     pub /: *mut *mut int nr_thp_failed; / THP failed to be migrated,
//     pub /: *mut *mut int nr_thp_split; / THP split before migrating,
//     pub /: *mut *mut int nr_split; / Large folio (include THP) split before migrating,
}

//
// Returns the number of hugetlb folios that were not migrated, or an error code
// after NR_MAX_MIGRATE_PAGES_RETRY attempts or if no hugetlb folios are movable
// any more because the list has become empty or no retryable hugetlb folios
// exist any more. It is caller's responsibility to call putback_movable_pages()
// only if ret != 0.
//
#[no_mangle]
pub unsafe extern "C" fn migrate_hugetlbs(from: *mut list_head, get_new_folio: new_folio_t, put_new_folio: free_folio_t, private: c_ulong, mode: migrate_mode, reason: migrate_reason, stats: *mut migrate_pages_stats, ret_folios: *mut list_head) -> c_int {
pub static mut retry: c_int = 1;
pub static mut nr_failed: c_int = 0;
pub static mut nr_retry_pages: c_int = 0;
pub static mut pass: c_int = 0;
    let mut folio = core::ptr::null_mut();
    let mut folio2 = core::ptr::null_mut();
    let mut rc = 0;
    let mut nr_pages = 0;
    while (pass < NR_MAX_MIGRATE_PAGES_RETRY && retry) {
    retry = 0;
    nr_retry_pages = 0;
    list_for_each_entry_safe(folio, folio2, from, lru) {
    if (!folio_test_hugetlb(folio)) {
    continue;
    }
    nr_pages = folio_nr_pages(folio);
    cond_resched();
//
// Migratability of hugepages depends on architectures and
// their size.  This check is necessary because some callers
// of hugepage migration like soft offline and memory
// hotremove don't walk through page tables or check whether
// the hugepage is pmd-based or not before kicking migration.
//
    if (!hugepage_migration_supported(folio_hstate(folio))) {
    nr_failed += 1;
    stats.nr_failed_pages += nr_pages;
    list_move_tail(&folio.lru, ret_folios);
    continue;
    }
    rc = unmap_and_move_hugetlb_folio(get_new_folio,
    put_new_folio, private,
    folio, pass > 2, mode,
    reason, ret_folios);
//
// The rules are:
// 0: hugetlb folio will be put back
// -EAGAIN: stay on the from list
// -ENOMEM: stay on the from list
// Other errno: put on ret_folios list
//
    match (rc) {
    -ENOMEM => {
//
// When memory is low, don't bother to try to migrate
// other folios, just exit.
//
    stats.nr_failed_pages += nr_pages + nr_retry_pages;
    return -ENOMEM;
    }
    -EAGAIN => {
    retry += 1;
    nr_retry_pages += nr_pages;
    // break;
    }
    0 => {
    stats.nr_succeeded += nr_pages;
    // break;
    }
    _ => {
//
// Permanent failure (-EBUSY, etc.):
// unlike -EAGAIN case, the failed folio is
// removed from migration folio list and not
// retried in the next outer loop.
//
    nr_failed += 1;
    stats.nr_failed_pages += nr_pages;
    // break;
    }
    }
    }
    }
//
// nr_failed is number of hugetlb folios failed to be migrated.  After
// NR_MAX_MIGRATE_PAGES_RETRY attempts, give up and count retried hugetlb
// folios as failed.
//
    nr_failed += retry;
    stats.nr_failed_pages += nr_retry_pages;
    return nr_failed;
    }
#[no_mangle]
pub unsafe extern "C" fn migrate_folios_move(src_folios: *mut list_head, dst_folios: *mut list_head, put_new_folio: free_folio_t, private: c_ulong, mode: migrate_mode, reason: migrate_reason, ret_folios: *mut list_head, stats: *mut migrate_pages_stats, retry: *mut c_int, thp_retry: *mut c_int, nr_failed: *mut c_int, nr_retry_pages: *mut c_int) {
    let mut folio = core::ptr::null_mut();
    let mut folio2 = core::ptr::null_mut();
    let mut dst = core::ptr::null_mut();
    let mut dst2 = core::ptr::null_mut();
    let mut is_thp = 0;
    let mut nr_pages = 0;
    let mut rc = 0;
    dst = list_first_entry(dst_folios, folio, lru);
    dst2 = list_next_entry(dst, lru);
    list_for_each_entry_safe(folio, folio2, src_folios, lru) {
    is_thp = folio_test_large(folio) && folio_test_pmd_mappable(folio);
    nr_pages = folio_nr_pages(folio);
    cond_resched();
    rc = migrate_folio_move(put_new_folio, private,
    folio, dst, mode,
    reason, ret_folios);
//
// The rules are:
// 0: folio will be freed
// -EAGAIN: stay on the src_folios list
// Other errno: put on ret_folios list
//
    match (rc) {
    -EAGAIN => {
// retry += 1;
// thp_retry += is_thp;
// nr_retry_pages += nr_pages;
    // break;
    }
    0 => {
    stats.nr_succeeded += nr_pages;
    stats.nr_thp_succeeded += is_thp;
    // break;
    }
    _ => {
// nr_failed += 1;
    stats.nr_thp_failed += is_thp;
    stats.nr_failed_pages += nr_pages;
    // break;
    }
    }
    dst = dst2;
    dst2 = list_next_entry(dst, lru);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn migrate_folios_undo(src_folios: *mut list_head, dst_folios: *mut list_head, put_new_folio: free_folio_t, private: c_ulong, ret_folios: *mut list_head) {
    let mut folio = core::ptr::null_mut();
    let mut folio2 = core::ptr::null_mut();
    let mut dst = core::ptr::null_mut();
    let mut dst2 = core::ptr::null_mut();
    dst = list_first_entry(dst_folios, folio, lru);
    dst2 = list_next_entry(dst, lru);
    list_for_each_entry_safe(folio, folio2, src_folios, lru) {
pub static mut old_folio_state: c_int = 0;
    let mut anon_vma = core::ptr::null_mut();
    __migrate_folio_extract(dst, &old_folio_state, &anon_vma);
    migrate_folio_undo_src(folio, old_folio_state & FOLIO_WAS_MAPPED,
    anon_vma, true, ret_folios);
    list_del(&dst.lru);
    migrate_folio_undo_dst(dst, true, put_new_folio, private);
    dst = dst2;
    dst2 = list_next_entry(dst, lru);
    }
    }
//
// migrate_pages_batch() first unmaps folios in the from list as many as
// possible, then move the unmapped folios.
//
// We only batch migration if mode == MIGRATE_ASYNC to avoid to wait a
// lock or bit when we have locked more than one folio.  Which may cause
// deadlock (e.g., for loop device).  So, if mode != MIGRATE_ASYNC, the
// length of the from list must be <= 1.
//
#[no_mangle]
pub unsafe extern "C" fn migrate_pages_batch(from: *mut list_head, get_new_folio: new_folio_t, put_new_folio: free_folio_t, private: c_ulong, mode: migrate_mode, reason: migrate_reason, ret_folios: *mut list_head, split_folios: *mut list_head, stats: *mut migrate_pages_stats, nr_pass: c_int) -> c_int {
pub static mut retry: c_int = 1;
pub static mut thp_retry: c_int = 1;
pub static mut nr_failed: c_int = 0;
pub static mut nr_retry_pages: c_int = 0;
pub static mut pass: c_int = 0;
pub static mut is_thp: bool = false;
pub static mut is_large: bool = false;
    struct folio *folio, *folio2, *dst = core::ptr::null_mut();
    int rc, rc_saved = 0, nr_pages;
pub static mut unmap_folios: usize = 0;
pub static mut dst_folios: usize = 0;
pub static mut nosplit: bool = false;
    VM_WARN_ON_ONCE(mode != MIGRATE_ASYNC &&
    !list_empty(from) && !list_is_singular(from));
    while (pass < nr_pass && retry) {
    retry = 0;
    thp_retry = 0;
    nr_retry_pages = 0;
    list_for_each_entry_safe(folio, folio2, from, lru) {
    is_large = folio_test_large(folio);
    is_thp = folio_test_pmd_mappable(folio);
    nr_pages = folio_nr_pages(folio);
    cond_resched_tasks_rcu_qs();
//
// The rare folio on the deferred split list should
// be split now. It should not count as a failure:
// but increment nr_failed because, without doing so,
// migrate_pages() may report success with (split but
// unmigrated) pages still on its fromlist; whereas it
// always reports success when its fromlist is empty.
// stats->nr_thp_failed should be increased too,
// otherwise stats inconsistency will happen when
// migrate_pages_batch is called via migrate_pages()
// with MIGRATE_SYNC and MIGRATE_ASYNC.
//
// Only check it without removing it from the list.
// Since the folio can be on deferred_split_scan()
// local list and removing it can cause the local list
// corruption. Folio split process below can handle it
// with the help of folio_ref_freeze().
//
// nr_pages > 2 is needed to avoid checking order-1
// page cache folios. They exist, in contrast to
// non-existent order-1 anonymous folios, and do not
// use _deferred_list.
//
    if (nr_pages > 2 &&
    !list_empty(&folio._deferred_list) &&
    folio_test_partially_mapped(folio)) {
    if (!try_split_folio(folio, split_folios, mode)) {
    nr_failed += 1;
    stats.nr_thp_failed += is_thp;
    stats.nr_thp_split += is_thp;
    stats.nr_split += 1;
    continue;
    }
    }
//
// Large folio migration might be unsupported or
// the allocation might be failed so we should retry
// on the same folio with the large folio split
// to normal folios.
//
// Split folios are put in split_folios, and
// we will migrate them after the rest of the
// list is processed.
//
    if (!thp_migration_supported() && is_thp) {
    nr_failed += 1;
    stats.nr_thp_failed += 1;
    if (!try_split_folio(folio, split_folios, mode)) {
    stats.nr_thp_split += 1;
    stats.nr_split += 1;
    continue;
    }
    stats.nr_failed_pages += nr_pages;
    list_move_tail(&folio.lru, ret_folios);
    continue;
    }
//
// If we are holding the last folio reference, the folio
// was freed from under us, so just drop our reference.
//
    if (likely(!page_has_movable_ops(&folio.page)) &&
    folio_ref_count(folio) == 1) {
    folio_clear_active(folio);
    folio_clear_unevictable(folio);
    list_del(&folio.lru);
    migrate_folio_done(folio, reason);
    stats.nr_succeeded += nr_pages;
    stats.nr_thp_succeeded += is_thp;
    continue;
    }
    rc = migrate_folio_unmap(get_new_folio, put_new_folio,
    private, folio, &dst, mode, ret_folios);
//
// The rules are:
// 0: folio will be put on unmap_folios list,
// dst folio put on dst_folios list
// -EAGAIN: stay on the from list
// -ENOMEM: stay on the from list
// Other errno: put on ret_folios list
//
    match (rc) {
    -ENOMEM => {
//
// When memory is low, don't bother to try to migrate
// other folios, move unmapped folios, then exit.
//
    nr_failed += 1;
    stats.nr_thp_failed += is_thp;
// Large folio NUMA faulting doesn't split to retry.
    if (is_large && !nosplit) {
pub static mut ret: c_int = 0;
    if (!ret) {
    stats.nr_thp_split += is_thp;
    stats.nr_split += 1;
    // break;
    } else if (reason == MR_LONGTERM_PIN &&
    ret == -EAGAIN) {
//
// Try again to split large folio to
// mitigate the failure of longterm pinning.
//
    retry += 1;
    thp_retry += is_thp;
    nr_retry_pages += nr_pages;
// Undo duplicated failure counting.
    nr_failed -= 1;
    stats.nr_thp_failed -= is_thp;
    // break;
    }
    }
    stats.nr_failed_pages += nr_pages + nr_retry_pages;
// nr_failed isn't updated for not used
    stats.nr_thp_failed += thp_retry;
    rc_saved = rc;
    if (list_empty(&unmap_folios)) {
// goto;
    }
    else {
// goto;
    }
    }
    -EAGAIN => {
    retry += 1;
    thp_retry += is_thp;
    nr_retry_pages += nr_pages;
    // break;
    }
    0 => {
    list_move_tail(&folio.lru, &unmap_folios);
    list_add_tail(&dst.lru, &dst_folios);
    // break;
    }
    _ => {
//
// Permanent failure (-EBUSY, etc.):
// unlike -EAGAIN case, the failed folio is
// removed from migration folio list and not
// retried in the next outer loop.
//
    nr_failed += 1;
    stats.nr_thp_failed += is_thp;
    stats.nr_failed_pages += nr_pages;
    // break;
    }
    }
    }
    }
    nr_failed += retry;
    stats.nr_thp_failed += thp_retry;
    stats.nr_failed_pages += nr_retry_pages;
// label;
// Flush TLBs for all unmapped folios
    try_to_unmap_flush();
    retry = 1;
    while (pass < nr_pass && retry) {
    retry = 0;
    thp_retry = 0;
    nr_retry_pages = 0;
// Move the unmapped folios
    migrate_folios_move(&unmap_folios, &dst_folios,
    put_new_folio, private, mode, reason,
    ret_folios, stats, &retry, &thp_retry,
    &nr_failed, &nr_retry_pages);
    }
    nr_failed += retry;
    stats.nr_thp_failed += thp_retry;
    stats.nr_failed_pages += nr_retry_pages;
    rc = rc_saved ? : nr_failed;
// label;
// Cleanup remaining folios
    migrate_folios_undo(&unmap_folios, &dst_folios,
    put_new_folio, private, ret_folios);
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn migrate_pages_sync(from: *mut list_head, get_new_folio: new_folio_t, put_new_folio: free_folio_t, private: c_ulong, mode: migrate_mode, reason: migrate_reason, ret_folios: *mut list_head, split_folios: *mut list_head, stats: *mut migrate_pages_stats) -> c_int {
    int rc, nr_failed = 0;
pub static mut folios: usize = 0;
pub static mut astats: usize = 0;
    memset(&astats, 0, sizeof!(astats));
// Try to migrate in batch with MIGRATE_ASYNC mode firstly
    rc = migrate_pages_batch(from, get_new_folio, put_new_folio, private, MIGRATE_ASYNC,
    reason, &folios, split_folios, &astats,
    NR_MAX_MIGRATE_ASYNC_RETRY);
    stats.nr_succeeded += astats.nr_succeeded;
    stats.nr_thp_succeeded += astats.nr_thp_succeeded;
    stats.nr_thp_split += astats.nr_thp_split;
    stats.nr_split += astats.nr_split;
    if (rc < 0) {
    stats.nr_failed_pages += astats.nr_failed_pages;
    stats.nr_thp_failed += astats.nr_thp_failed;
    list_splice_tail(&folios, ret_folios);
    return rc;
    }
    stats.nr_thp_failed += astats.nr_thp_split;
//
// Do not count rc, as pages will be retried below.
// Count nr_split only, since it includes nr_thp_split.
//
    nr_failed += astats.nr_split;
//
// Fall back to migrate all failed folios one by one synchronously. All
// failed folios except split THPs will be retried, so their failure
// isn't counted
//
    list_splice_tail_init(&folios, from);
    while (!list_empty(from)) {
    list_move(from.next, &folios);
    rc = migrate_pages_batch(&folios, get_new_folio, put_new_folio,
    private, mode, reason, ret_folios,
    split_folios, stats, NR_MAX_MIGRATE_SYNC_RETRY);
    list_splice_tail_init(&folios, ret_folios);
    if (rc < 0) {
    return rc;
    }
    nr_failed += rc;
    }
    return nr_failed;
    }
//
// migrate_pages - migrate the folios specified in a list, to the free folios
// supplied as the target for the page migration
//
// @from:		The list of folios to be migrated.
// @get_new_folio:	The function used to allocate free folios to be used
// as the target of the folio migration.
// @put_new_folio:	The function used to free target folios if migration
// fails, or NULL if no special handling is necessary.
// @private:		Private data to be passed on to get_new_folio()
// @mode:		The migration mode that specifies the constraints for
// folio migration, if any.
// @reason:		The reason for folio migration.
// @ret_succeeded:	Set to the number of folios migrated successfully if
// the caller passes a non-NULL pointer.
//
// The function returns after NR_MAX_MIGRATE_PAGES_RETRY attempts or if no folios
// are movable any more because the list has become empty or no retryable folios
// exist any more. It is caller's responsibility to call putback_movable_pages()
// only if ret != 0.
//
// Returns the number of {normal folio, large folio, hugetlb} that were not
// migrated, or an error code. The number of large folio splits will be
// considered as the number of non-migrated large folio, no matter how many
// split folios of the large folio are migrated successfully.
//
#[no_mangle]
pub unsafe extern "C" fn migrate_pages(from: *mut list_head, get_new_folio: new_folio_t, put_new_folio: free_folio_t, private: c_ulong, mode: migrate_mode, reason: migrate_reason, ret_succeeded: *mut c_uint) -> c_int {
    let mut rc = 0;
    let mut rc_gather = 0;
    let mut nr_pages = 0;
    let mut folio = core::ptr::null_mut();
    let mut folio2 = core::ptr::null_mut();
pub static mut folios: usize = 0;
pub static mut ret_folios: usize = 0;
pub static mut split_folios: usize = 0;
pub static mut stats: usize = 0;
    trace_mm_migrate_pages_start(mode, reason);
    memset(&stats, 0, sizeof!(stats));
    rc_gather = migrate_hugetlbs(from, get_new_folio, put_new_folio, private,
    mode, reason, &stats, &ret_folios);
    if (rc_gather < 0) {
// goto;
    }
// label;
    nr_pages = 0;
    list_for_each_entry_safe(folio, folio2, from, lru) {
// Retried hugetlb folios will be kept in list
    if (folio_test_hugetlb(folio)) {
    list_move_tail(&folio.lru, &ret_folios);
    continue;
    }
    nr_pages += folio_nr_pages(folio);
    if (nr_pages >= NR_MAX_BATCHED_MIGRATION) {
    break;
    }
    }
    if (nr_pages >= NR_MAX_BATCHED_MIGRATION) {
    list_cut_before(&folios, from, &folio2.lru);
    }
    else {
    list_splice_init(from, &folios);
    }
    if (mode == MIGRATE_ASYNC) {
    rc = migrate_pages_batch(&folios, get_new_folio, put_new_folio,
    private, mode, reason, &ret_folios,
    &split_folios, &stats,
    NR_MAX_MIGRATE_PAGES_RETRY);
    }
    else {
    rc = migrate_pages_sync(&folios, get_new_folio, put_new_folio,
    private, mode, reason, &ret_folios,
    &split_folios, &stats);
    }
    list_splice_tail_init(&folios, &ret_folios);
    if (rc < 0) {
    rc_gather = rc;
    list_splice_tail(&split_folios, &ret_folios);
// goto;
    }
    if (!list_empty(&split_folios)) {
//
// Failure isn't counted since all split folios of a large folio
// is counted as 1 failure already.  And, we only try to migrate
// with minimal effort, force MIGRATE_ASYNC mode and retry once.
//
    migrate_pages_batch(&split_folios, get_new_folio,
    put_new_folio, private, MIGRATE_ASYNC, reason,
    &ret_folios, core::ptr::null_mut(), &stats, 1);
    list_splice_tail_init(&split_folios, &ret_folios);
    }
    rc_gather += rc;
    if (!list_empty(from)) {
// goto;
    }
// label;
//
// Put the permanent failure folio back to migration list, they
// will be put back to the right list by the caller.
//
    list_splice(&ret_folios, from);
//
// Return 0 in case all split folios of fail-to-migrate large folios
// are migrated successfully.
//
    if (list_empty(from)) {
    rc_gather = 0;
    }
    count_vm_events(PGMIGRATE_SUCCESS, stats.nr_succeeded);
    count_vm_events(PGMIGRATE_FAIL, stats.nr_failed_pages);
    count_vm_events(THP_MIGRATION_SUCCESS, stats.nr_thp_succeeded);
    count_vm_events(THP_MIGRATION_FAIL, stats.nr_thp_failed);
    count_vm_events(THP_MIGRATION_SPLIT, stats.nr_thp_split);
    trace_mm_migrate_pages(stats.nr_succeeded, stats.nr_failed_pages,
    stats.nr_thp_succeeded, stats.nr_thp_failed,
    stats.nr_thp_split, stats.nr_split, mode,
    reason);
    if (ret_succeeded) {
// ret_succeeded = stats.nr_succeeded;
    }
    return rc_gather;
    }
#[no_mangle]
pub unsafe extern "C" fn alloc_migration_target(src: *mut folio, private: c_ulong) -> *mut c_void {
pub static mut mtc: *mut c_void = core::ptr::null_mut();
    let mut gfp_mask;
pub static mut order: c_uint = 0;
    let mut nid = 0;
    enum zone_type zidx;
    mtc = private;
    gfp_mask = mtc.gfp_mask;
    nid = mtc.nid;
    if (nid == NUMA_NO_NODE) {
    nid = folio_nid(src);
    }
    if (folio_test_hugetlb(src)) {
    let mut h = folio_hstate(src);
    gfp_mask = htlb_modify_alloc_mask(h, gfp_mask);
    return alloc_hugetlb_folio_nodemask(h, nid,
    mtc.nmask, gfp_mask,
    htlb_allow_alloc_fallback(mtc.reason));
    }
    if (folio_test_large(src)) {
//
// clear __GFP_RECLAIM to make the migration callback
// consistent with regular THP allocations.
//
    gfp_mask &= ~__GFP_RECLAIM;
    gfp_mask |= GFP_TRANSHUGE;
    order = folio_order(src);
    }
    zidx = folio_zonenum(src);
    if (is_highmem_idx(zidx) || zidx == ZONE_MOVABLE) {
    gfp_mask |= __GFP_HIGHMEM;
    }
    return __folio_alloc(gfp_mask, order, nid, mtc.nmask);
    }

#[no_mangle]
unsafe extern "C" fn store_status(status: *mut int , start: c_int, value: c_int, nr: c_int) -> c_int {
    while (nr-- > 0) {
    if (put_user(value, status + start)) {
    return -EFAULT;
    }
    start += 1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn do_move_pages_to_node(pagelist: *mut list_head, node: c_int) -> c_int {
    let mut err = 0;
pub static mut migration_target_control: usize = 0;
    err = migrate_pages(pagelist, alloc_migration_target, core::ptr::null_mut(),
    (unsigned long)&mtc, MIGRATE_SYNC, MR_SYSCALL, core::ptr::null_mut());
    if (err) {
    putback_movable_pages(pagelist);
    }
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn __add_folio_for_migration(folio: *mut folio, node: c_int, pagelist: *mut list_head, migrate_all: bool) -> c_int {
    if (is_zero_folio(folio) || is_huge_zero_folio(folio)) {
    return -EFAULT;
    }
    if (folio_is_zone_device(folio)) {
    return -ENOENT;
    }
    if (folio_nid(folio) == node) {
    return 0;
    }
    if (folio_maybe_mapped_shared(folio) && !migrate_all) {
    return -EACCES;
    }
    if (folio_test_hugetlb(folio)) {
    if (folio_isolate_hugetlb(folio, pagelist)) {
    return 1;
    }
    } else if (folio_isolate_lru(folio)) {
    list_add_tail(&folio.lru, pagelist);
    node_stat_mod_folio(folio,
    NR_ISOLATED_ANON + folio_is_file_lru(folio),
    folio_nr_pages(folio));
    return 1;
    }
    return -EBUSY;
    }
//
// Resolves the given address to a struct folio, isolates it from the LRU and
// puts it to the given pagelist.
// Returns:
// errno - if the folio cannot be found/isolated
// 0 - when it doesn't have to be migrated because it is already on the
// target node
// 1 - when it has been queued
//
#[no_mangle]
pub unsafe extern "C" fn add_folio_for_migration(mm: *mut mm_struct, p: *mut c_void, node: c_int, pagelist: *mut list_head, migrate_all: bool) -> c_int {
pub static mut vma: *mut c_void = core::ptr::null_mut();
pub static mut fw: usize = 0;
pub static mut folio: *mut c_void = core::ptr::null_mut();
    let mut addr = 0;
pub static mut err: c_int = 0;
    mmap_read_lock(mm);
    addr = (unsigned long)untagged_addr_remote(mm, p);
    vma = vma_lookup(mm, addr);
    if (vma && vma_migratable(vma)) {
    folio = folio_walk_start(&fw, vma, addr, FW_ZEROPAGE);
    if (folio) {
    err = __add_folio_for_migration(folio, node, pagelist,
    migrate_all);
    folio_walk_end(&fw, vma);
    } else {
    err = -ENOENT;
    }
    }
    mmap_read_unlock(mm);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn move_pages_and_store_status(node: c_int, pagelist: *mut list_head, status: *mut c_int, start: c_int, i: c_int, nr_pages: c_ulong) -> c_int {
    let mut err = 0;
    if (list_empty(pagelist)) {
    return 0;
    }
    err = do_move_pages_to_node(pagelist, node);
    if (err) {
//
// Positive err means the number of failed
// pages to migrate.  Since we are going to
// abort and return the number of non-migrated
// pages, so need to include the rest of the
// nr_pages that have not been attempted as
// well.
//
    if (err > 0) {
    err += nr_pages - i;
    }
    return err;
    }
    return store_status(status, start, node, i - start);
    }
//
// Migrate an array of page address onto an array of nodes and fill
// the corresponding array of status.
//
#[no_mangle]
pub unsafe extern "C" fn do_pages_move(mm: *mut mm_struct, task_nodes: nodemask_t, nr_pages: c_ulong, pages: *mut *mut c_void, nodes: *mut c_int, status: *mut c_int, flags: c_int) -> c_int {
    let mut compat_pages = pages;
pub static mut current_node: c_int = 0;
pub static mut pagelist: usize = 0;
    let mut start = 0;
    let mut i = 0;
pub static mut err: c_int = 0;
    lru_cache_disable();
    while (i < nr_pages) {
    let mut p = core::ptr::null_mut();
    let mut node = 0;
    err = -EFAULT;
    if (in_compat_syscall()) {
    let mut cp;
    if (get_user(cp, compat_pages + i)) {
// goto;
    }
    p = compat_ptr(cp);
    } else {
    if (get_user(p, pages + i)) {
// goto;
    }
    }
    if (get_user(node, nodes + i)) {
// goto;
    }
    err = -ENODEV;
    if (node < 0 || node >= MAX_NUMNODES) {
// goto;
    }
    if (!node_state(node, N_MEMORY)) {
// goto;
    }
    err = -EACCES;
    if (!node_isset(node, task_nodes)) {
// goto;
    }
    if (current_node == NUMA_NO_NODE) {
    current_node = node;
    start = i;
    } else if (node != current_node) {
    err = move_pages_and_store_status(current_node,
    &pagelist, status, start, i, nr_pages);
    if (err) {
// goto;
    }
    start = i;
    current_node = node;
    }
//
// Errors in the page lookup or isolation are not fatal and we simply
// report them via status
//
    err = add_folio_for_migration(mm, p, current_node, &pagelist,
    flags & MPOL_MF_MOVE_ALL);
    if (err > 0) {
// The page is successfully queued for migration
    continue;
    }
//
// If the page is already on the target node (!err), store the
// node, otherwise, store the err.
//
    err = store_status(status, i, err ? : current_node, 1);
    if (err) {
// goto;
    }
    err = move_pages_and_store_status(current_node, &pagelist,
    status, start, i, nr_pages);
    if (err) {
// We have accounted for page i
    if (err > 0) {
    err -= 1;
    }
// goto;
    }
    current_node = NUMA_NO_NODE;
    }
// label;
// Make sure we do not overwrite the existing error
    err1 = move_pages_and_store_status(current_node, &pagelist,
    status, start, i, nr_pages);
    if (err >= 0) {
    err = err1;
    }
// label;
    lru_cache_enable();
    return err;
    }
//
// Determine the nodes of an array of pages and store it in an array of status.
//
#[no_mangle]
pub unsafe extern "C" fn do_pages_stat_array(mm: *mut mm_struct, nr_pages: c_ulong, pages: *mut *mut c_void, status: *mut c_int) {
    let mut i = 0;
    mmap_read_lock(mm);
    while (i < nr_pages) {
pub static mut addr: c_ulong = 0;
pub static mut vma: *mut c_void = core::ptr::null_mut();
pub static mut fw: usize = 0;
pub static mut folio: *mut c_void = core::ptr::null_mut();
pub static mut err: c_int = 0;
    vma = vma_lookup(mm, addr);
    if (!vma) {
// goto;
    }
    folio = folio_walk_start(&fw, vma, addr, FW_ZEROPAGE);
    if (folio) {
    if (is_zero_folio(folio) || is_huge_zero_folio(folio)) {
    err = -EFAULT;
    }

    else if (folio_is_zone_device(folio)) {
    err = -ENOENT;
    }
    else {
    err = folio_nid(folio);
    }
    folio_walk_end(&fw, vma);
    } else {
    err = -ENOENT;
    }
// label;
// status = err;
    pages += 1;
    status += 1;
    }
    mmap_read_unlock(mm);
    }
#[no_mangle]
pub unsafe extern "C" fn get_compat_pages_array(pages: *mut *mut c_void, chunk_offset: c_ulong, chunk_nr: c_ulong) -> c_int {
    let mut pages32 = pages;
    let mut p;
    let mut i = 0;
    while (i < chunk_nr) {
    if (get_user(p, pages32 + chunk_offset + i)) {
    return -EFAULT;
    }
    chunk_pages[i] = compat_ptr(p);
    }
    return 0;
    }
//
// Determine the nodes of a user array of pages and store it in
// a user array of status.
//
#[no_mangle]
pub unsafe extern "C" fn do_pages_stat(mm: *mut mm_struct, nr_pages: c_ulong, pages: *mut *mut c_void, status: *mut c_int) -> c_int {

    const void  *chunk_pages[DO_PAGES_STAT_CHUNK_NR];
    int chunk_status[DO_PAGES_STAT_CHUNK_NR];
pub static mut chunk_offset: c_ulong = 0;
    while (nr_pages) {
pub static mut chunk_nr: c_ulong = 0;
    if (in_compat_syscall()) {
    if (get_compat_pages_array(chunk_pages, pages,
    chunk_offset, chunk_nr)) {
    break;
    }
    } else {
    if (copy_from_user(chunk_pages, pages + chunk_offset,
#[no_mangle]
pub unsafe extern "C" fn sizeof!(_arg: *mut chunk_pages))) -> *mut chunk_nr {
    chunk_nr * sizeof!(*chunk_pages)))
    break;
    }
    do_pages_stat_array(mm, chunk_nr, chunk_pages, chunk_status);
    if (copy_to_user(status + chunk_offset, chunk_status,
#[no_mangle]
pub unsafe extern "C" fn sizeof!(_arg: *mut status))) -> *mut chunk_nr {
    chunk_nr * sizeof!(*status)))
    break;
    chunk_offset += chunk_nr;
    nr_pages -= chunk_nr;
    }
    return nr_pages ? -EFAULT : 0;
    }
#[no_mangle]
pub unsafe extern "C" fn find_mm_struct(pid: pid_t, mem_nodes: *mut nodemask_t) -> *mut c_void {
pub static mut task: *mut c_void = core::ptr::null_mut();
pub static mut mm: *mut c_void = core::ptr::null_mut();
//
// There is no need to check if current process has the right to modify
// the specified process when they are same.
//
    if (!pid) {
    mmget(current.mm);
// mem_nodes = cpuset_mems_allowed(current);
    return current.mm;
    }
    task = find_get_task_by_vpid(pid);
    if (!task) {
    return ERR_PTR(-ESRCH);
    }
    if (down_read_killable(&task.signal.exec_update_lock)) {
    mm = ERR_PTR(-EINTR);
// goto;
    }
//
// Check if this process has the right to modify the specified
// process. Use the regular "ptrace_may_access()" checks.
//
    if (!ptrace_may_access(task, PTRACE_MODE_READ_REALCREDS)) {
    mm = ERR_PTR(-EPERM);
// goto;
    }
    mm = ERR_PTR(security_task_movememory(task));
    if (IS_ERR(mm)) {
// goto;
    }
// mem_nodes = cpuset_mems_allowed(task);
    mm = get_task_mm(task);
// label;
    up_read(&task.signal.exec_update_lock);
// label;
    put_task_struct(task);
    if (!mm) {
    mm = ERR_PTR(-EINVAL);
    }
    return mm;
    }
//
// Move a list of pages in the address space of the currently executing
// process.
//
#[no_mangle]
pub unsafe extern "C" fn kernel_move_pages(pid: pid_t, nr_pages: c_ulong, pages: *mut *mut c_void, nodes: *mut c_int, status: *mut c_int, flags: c_int) -> c_int {
pub static mut mm: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    let mut task_nodes;
// Check flags
    if (flags & ~(MPOL_MF_MOVE|MPOL_MF_MOVE_ALL)) {
    return -EINVAL;
    }
    if ((flags & MPOL_MF_MOVE_ALL) && !capable(CAP_SYS_NICE)) {
    return -EPERM;
    }
    mm = find_mm_struct(pid, &task_nodes);
    if (IS_ERR(mm)) {
    return PTR_ERR(mm);
    }
    if (nodes) {
    err = do_pages_move(mm, task_nodes, nr_pages, pages,
    nodes, status, flags);
    }
    else {
    err = do_pages_stat(mm, nr_pages, pages, status);
    }
    mmput(mm);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn sys_move_pages(pid: usize, nr_pages: usize, pages: usize, nodes: usize, status: usize, flags: usize) -> c_long {
    return kernel_move_pages(pid, nr_pages, pages, nodes, status, flags);
    }

//
// Returns true if this is a safe migration target node for misplaced NUMA
// pages. Currently it only checks the watermarks which is crude.
//
#[no_mangle]
pub unsafe extern "C" fn migrate_balanced_pgdat(pgdat: *mut pglist_data, nr_migrate_pages: c_ulong) -> bool {
    let mut z = 0;
    while (z >= 0) {
    let mut zone = pgdat.node_zones + z;
    if (!managed_zone(zone)) {
    continue;
    }
// Avoid waking kswapd by allocating pages_to_migrate pages.
    if (!zone_watermark_ok(zone, 0,
    high_wmark_pages(zone) +
    nr_migrate_pages,
    ZONE_MOVABLE, ALLOC_CMA)) {
    continue;
    }
    return true;
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn alloc_misplaced_dst_folio(src: *mut folio, data: c_ulong) -> *mut c_void {
pub static mut nid: c_int = 0;
pub static mut order: c_int = 0;
pub static mut gfp: gfp_t = 0;
    if (order > 0) {
    gfp |= GFP_TRANSHUGE_LIGHT;
    }
    else {
    gfp |= GFP_HIGHUSER_MOVABLE | __GFP_NOMEMALLOC | __GFP_NORETRY |
    __GFP_NOWARN;
    gfp &= ~__GFP_RECLAIM;
    }
    return __folio_alloc_node(gfp, order, nid);
    }
//
// Prepare for calling migrate_misplaced_folio() by isolating the folio if
// permitted. Must be called with the PTL still held.
//
#[no_mangle]
pub unsafe extern "C" fn migrate_misplaced_folio_prepare(folio: *mut folio, vma: *mut vm_area_struct, node: c_int) -> c_int {
pub static mut nr_pages: c_int = 0;
    let mut pgdat = NODE_DATA(node);
    if (folio_is_file_lru(folio)) {
//
// Do not migrate file folios that are mapped in multiple
// processes with execute permissions as they are probably
// shared libraries.
//
// See folio_maybe_mapped_shared() on possible imprecision
// when we cannot easily detect if a folio is shared.
//
    if ((vma.vm_flags & VM_EXEC) && folio_maybe_mapped_shared(folio)) {
    return -EACCES;
    }
//
// Do not migrate dirty folios as not all filesystems can move
// dirty folios in MIGRATE_ASYNC mode which is a waste of
// cycles.
//
    if (folio_test_dirty(folio)) {
    return -EAGAIN;
    }
    }
// Avoid migrating to a node that is nearly full
    if (!migrate_balanced_pgdat(pgdat, nr_pages)) {
    let mut z = 0;
    if (!(sysctl_numa_balancing_mode & NUMA_BALANCING_MEMORY_TIERING)) {
    return -EAGAIN;
    }
    while (z >= 0) {
    if (managed_zone(pgdat.node_zones + z)) {
    break;
    }
    }
//
// If there are no managed zones, it should not proceed
// further.
//
    if (z < 0) {
    return -EAGAIN;
    }
    wakeup_kswapd(pgdat.node_zones + z, 0,
    folio_order(folio), ZONE_MOVABLE);
    return -EAGAIN;
    }
    if (!folio_isolate_lru(folio)) {
    return -EAGAIN;
    }
    node_stat_mod_folio(folio, NR_ISOLATED_ANON + folio_is_file_lru(folio),
    nr_pages);
    return 0;
    }
//
// Attempt to migrate a misplaced folio to the specified destination
// node. Caller is expected to have isolated the folio by calling
// migrate_misplaced_folio_prepare(), which will result in an
// elevated reference count on the folio. This function will un-isolate the
// folio, dereferencing the folio before returning.
//
#[no_mangle]
pub unsafe extern "C" fn migrate_misplaced_folio(folio: *mut folio, node: c_int) -> c_int {
    let mut pgdat = NODE_DATA(node);
    let mut nr_remaining = 0;
    let mut nr_succeeded = 0;
pub static mut migratepages: usize = 0;
    let mut memcg = get_mem_cgroup_from_folio(folio);
    let mut lruvec = mem_cgroup_lruvec(memcg, pgdat);
    list_add(&folio.lru, &migratepages);
    nr_remaining = migrate_pages(&migratepages, alloc_misplaced_dst_folio,
    core::ptr::null_mut(), node, MIGRATE_ASYNC,
    MR_NUMA_MISPLACED, &nr_succeeded);
    if (nr_remaining && !list_empty(&migratepages)) {
    putback_movable_pages(&migratepages);
    }
    if (nr_succeeded) {
    count_vm_numa_events(NUMA_PAGE_MIGRATE, nr_succeeded);
    count_memcg_events(memcg, NUMA_PAGE_MIGRATE, nr_succeeded);
    if ((sysctl_numa_balancing_mode & NUMA_BALANCING_MEMORY_TIERING)
    && !node_is_toptier(folio_nid(folio))
    && node_is_toptier(node)) {
    mod_lruvec_state(lruvec, PGPROMOTE_SUCCESS, nr_succeeded);
    }
    }
    mem_cgroup_put(memcg);
    BUG_ON!(!list_empty(&migratepages));
    return nr_remaining ? -EAGAIN : 0;
    }
}
}
