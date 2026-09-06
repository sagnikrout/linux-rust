//! Automatically rewritten from C to Rust
//! Source: mm/balloon.c
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
// Common interface for implementing a memory balloon, including support
// for migration of pages inflated in a memory balloon.
//
// Copyright (C) 2012, Red Hat, Inc.  Rafael Aquini <aquini@redhat.com>
//

//
// Lock protecting the balloon_dev_info of all devices. We don't really
// expect more than one device.
//
pub static mut balloon_pages_lock: usize = 0;
//
// balloon_page_insert - insert a page into the balloon's page list and make
// the page->private assignment accordingly.
// @balloon : pointer to balloon device
// @page    : page to be assigned as a 'balloon page'
//
// Caller must ensure the balloon_pages_lock is held.
//
#[no_mangle]
pub unsafe extern "C" fn balloon_page_insert(balloon: *mut balloon_dev_info, page: *mut page) {
    lockdep_assert_held(&balloon_pages_lock);
    __SetPageOffline(page);
    if (IS_ENABLED!(CONFIG_BALLOON_MIGRATION)) {
    SetPageMovableOps(page);
    set_page_private(page, (unsigned long)balloon);
    }
    list_add(&page.lru, &balloon.pages);
    }
//
// balloon_page_finalize - prepare a balloon page that was removed from the
// balloon list for release to the page allocator
// @page: page to be released to the page allocator
//
// Caller must ensure the balloon_pages_lock is held.
//
#[no_mangle]
unsafe extern "C" fn balloon_page_finalize(page: *mut page) {
    lockdep_assert_held(&balloon_pages_lock);
    if (IS_ENABLED!(CONFIG_BALLOON_MIGRATION)) {
    set_page_private(page, 0);
    }
// PageOffline is sticky until the page is freed to the buddy.
    }
#[no_mangle]
pub unsafe extern "C" fn balloon_page_enqueue_one(b_dev_info: *mut balloon_dev_info, page: *mut page) {
    balloon_page_insert(b_dev_info, page);
    if (b_dev_info.adjust_managed_page_count) {
    adjust_managed_page_count(page, -1);
    }
    __count_vm_event(BALLOON_INFLATE);
    inc_node_page_state(page, NR_BALLOON_PAGES);
    }
//
// balloon_page_list_enqueue() - inserts a list of pages into the balloon page
// list.
// @b_dev_info: balloon device descriptor where we will insert a new page to
// @pages: pages to enqueue - allocated using balloon_page_alloc.
//
// Driver must call this function to properly enqueue balloon pages before
// definitively removing them from the guest system.
//
// Return: number of pages that were enqueued.
//
#[no_mangle]
pub unsafe extern "C" fn balloon_page_list_enqueue(b_dev_info: *mut balloon_dev_info, pages: *mut list_head) -> size_t {
    let mut page = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();
    let mut flags = 0;
pub static mut n_pages: usize = 0;
    spin_lock_irqsave(&balloon_pages_lock, flags);
    list_for_each_entry_safe(page, tmp, pages, lru) {
    list_del(&page.lru);
    balloon_page_enqueue_one(b_dev_info, page);
    n_pages += 1;
    }
    spin_unlock_irqrestore(&balloon_pages_lock, flags);
    return n_pages;
    }
    EXPORT_SYMBOL_GPL(balloon_page_list_enqueue);
//
// balloon_page_list_dequeue() - removes pages from balloon's page list and
// returns a list of the pages.
// @b_dev_info: balloon device descriptor where we will grab a page from.
// @pages: pointer to the list of pages that would be returned to the caller.
// @n_req_pages: number of requested pages.
//
// Driver must call this function to properly de-allocate a previous enlisted
// balloon pages before definitively releasing it back to the guest system.
// This function tries to remove @n_req_pages from the ballooned pages and
// return them to the caller in the @pages list.
//
// Note that this function may fail to dequeue some pages even if the balloon
// isn't empty - since the page list can be temporarily empty due to compaction
// of isolated pages.
//
// Return: number of pages that were added to the @pages list.
//
#[no_mangle]
pub unsafe extern "C" fn balloon_page_list_dequeue(b_dev_info: *mut balloon_dev_info, pages: *mut list_head, n_req_pages: size_t) -> size_t {
    let mut page = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();
    let mut flags = 0;
pub static mut n_pages: usize = 0;
    spin_lock_irqsave(&balloon_pages_lock, flags);
    list_for_each_entry_safe(page, tmp, &b_dev_info.pages, lru) {
    if (n_pages == n_req_pages) {
    break;
    }
    list_del(&page.lru);
    if (b_dev_info.adjust_managed_page_count) {
    adjust_managed_page_count(page, 1);
    }
    balloon_page_finalize(page);
    __count_vm_event(BALLOON_DEFLATE);
    list_add(&page.lru, pages);
    dec_node_page_state(page, NR_BALLOON_PAGES);
    n_pages += 1;
    }
    spin_unlock_irqrestore(&balloon_pages_lock, flags);
    return n_pages;
    }
    EXPORT_SYMBOL_GPL(balloon_page_list_dequeue);
//
// balloon_page_alloc - allocates a new page for insertion into the balloon
// page list.
//
// Driver must call this function to properly allocate a new balloon page.
// Driver must call balloon_page_enqueue before definitively removing the page
// from the guest system.
//
// Return: page for the allocated page or NULL on allocation failure.
//
#[no_mangle]
pub unsafe extern "C" fn balloon_page_alloc() -> *mut c_void {
pub static mut gfp_flags: gfp_t = 0;
    if (IS_ENABLED!(CONFIG_BALLOON_MIGRATION)) {
    gfp_flags |= GFP_HIGHUSER_MOVABLE;
    }
    else {
    gfp_flags |= GFP_HIGHUSER;
    }
    return alloc_page(gfp_flags);
    }
    EXPORT_SYMBOL_GPL(balloon_page_alloc);
//
// balloon_page_enqueue - inserts a new page into the balloon page list.
//
// @b_dev_info: balloon device descriptor where we will insert a new page
// @page: new page to enqueue - allocated using balloon_page_alloc.
//
// Drivers must call this function to properly enqueue a new allocated balloon
// page before definitively removing the page from the guest system.
//
// Drivers must not enqueue pages while page->lru is still in
// use, and must not use page->lru until a page was unqueued again.
//
#[no_mangle]
pub unsafe extern "C" fn balloon_page_enqueue(b_dev_info: *mut balloon_dev_info, page: *mut page) {
    let mut flags = 0;
    spin_lock_irqsave(&balloon_pages_lock, flags);
    balloon_page_enqueue_one(b_dev_info, page);
    spin_unlock_irqrestore(&balloon_pages_lock, flags);
    }
    EXPORT_SYMBOL_GPL(balloon_page_enqueue);
//
// balloon_page_dequeue - removes a page from balloon's page list and returns
// its address to allow the driver to release the page.
// @b_dev_info: balloon device descriptor where we will grab a page from.
//
// Driver must call this function to properly dequeue a previously enqueued page
// before definitively releasing it back to the guest system.
//
// Caller must perform its own accounting to ensure that this
// function is called only if some pages are actually enqueued.
//
// Note that this function may fail to dequeue some pages even if there are
// some enqueued pages - since the page list can be temporarily empty due to
// the compaction of isolated pages.
//
// TODO: remove the caller accounting requirements, and allow caller to wait
// until all pages can be dequeued.
//
// Return: page for the dequeued page, or NULL if no page was dequeued.
//
#[no_mangle]
pub unsafe extern "C" fn balloon_page_dequeue(b_dev_info: *mut balloon_dev_info) -> *mut c_void {
    let mut flags = 0;
pub static mut pages: usize = 0;
    let mut n_pages = 0;
    n_pages = balloon_page_list_dequeue(b_dev_info, &pages, 1);
    if (n_pages != 1) {
//
// If we are unable to dequeue a balloon page because the page
// list is empty and there are no isolated pages, then something
// went out of track and some balloon pages are lost.
// BUG() here, otherwise the balloon driver may get stuck in
// an infinite loop while attempting to release all its pages.
//
    spin_lock_irqsave(&balloon_pages_lock, flags);
    if (unlikely(list_empty(&b_dev_info.pages) &&
    !b_dev_info.isolated_pages)) {
    BUG();
    }
    spin_unlock_irqrestore(&balloon_pages_lock, flags);
    return core::ptr::null_mut();
    }
    return list_first_entry(&pages, page, lru);
    }
    EXPORT_SYMBOL_GPL(balloon_page_dequeue);

#[no_mangle]
pub unsafe extern "C" fn balloon_page_device(page: *mut page) -> *mut c_void {
    return page_private(page);
    }
#[no_mangle]
unsafe extern "C" fn balloon_page_isolate(page: *mut page, mode: isolate_mode_t) -> bool {
pub static mut b_dev_info: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    spin_lock_irqsave(&balloon_pages_lock, flags);
    b_dev_info = balloon_page_device(page);
    if (!b_dev_info) {
//
// The page already got deflated and removed from the
// balloon list.
//
    spin_unlock_irqrestore(&balloon_pages_lock, flags);
    return false;
    }
    list_del(&page.lru);
    b_dev_info.isolated_pages += 1;
    spin_unlock_irqrestore(&balloon_pages_lock, flags);
    return true;
    }
#[no_mangle]
unsafe extern "C" fn balloon_page_putback(page: *mut page) {
    let mut b_dev_info = balloon_page_device(page);
    let mut flags = 0;
//
// When we isolated the page, the page was still inflated in a balloon
// device. As isolated balloon pages cannot get deflated, we still have
// a balloon device here.
//
    if (WARN_ON_ONCE!(!b_dev_info)) {
    return;
    }
    spin_lock_irqsave(&balloon_pages_lock, flags);
    list_add(&page.lru, &b_dev_info.pages);
    b_dev_info.isolated_pages -= 1;
    spin_unlock_irqrestore(&balloon_pages_lock, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn balloon_page_migrate(newpage: *mut page, page: *mut page, mode: migrate_mode) -> c_int {
    let mut b_dev_info = balloon_page_device(page);
    let mut flags = 0;
    let mut rc = 0;
//
// When we isolated the page, the page was still inflated in a balloon
// device. As isolated balloon pages cannot get deflated, we still have
// a balloon device here.
//
    if (WARN_ON_ONCE!(!b_dev_info)) {
    return -EAGAIN;
    }
    rc = b_dev_info.migratepage(b_dev_info, newpage, page, mode);
    if (rc < 0 && rc != -ENOENT) {
    return rc;
    }
    spin_lock_irqsave(&balloon_pages_lock, flags);
    if (!rc) {
// Insert the new page into the balloon list.
    get_page(newpage);
    balloon_page_insert(b_dev_info, newpage);
    __count_vm_event(BALLOON_MIGRATE);
    if (b_dev_info.adjust_managed_page_count &&
    page_zone(page) != page_zone(newpage)) {
//
// When we migrate a page to a different zone we
// have to fixup the count of both involved zones.
//
    adjust_managed_page_count(page, 1);
    adjust_managed_page_count(newpage, -1);
    }
    } else {
// Old page was deflated but new page not inflated.
    __count_vm_event(BALLOON_DEFLATE);
    if (b_dev_info.adjust_managed_page_count) {
    adjust_managed_page_count(page, 1);
    }
    }
    b_dev_info.isolated_pages -= 1;
// Free the now-deflated page we isolated in balloon_page_isolate().
    balloon_page_finalize(page);
    spin_unlock_irqrestore(&balloon_pages_lock, flags);
    put_page(page);
    return 0;
    }
pub static mut movable_operations: usize = 0;
#[no_mangle]
unsafe extern "C" fn balloon_init() -> c_int {
    return set_movable_ops(&balloon_mops, PGTY_offline);
    }
    core_initcall!(balloon_init);