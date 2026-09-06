//! Automatically rewritten from C to Rust
//! Source: mm/page_table_check.c
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
// Copyright (c) 2021, Google LLC.
// Pasha Tatashin <pasha.tatashin@soleen.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct page_table_check {
    pub anon_map_count: core::sync::atomic::AtomicI32,
    pub file_map_count: core::sync::atomic::AtomicI32,
}

    static bool __page_table_check_enabled __initdata =
    IS_ENABLED!(CONFIG_PAGE_TABLE_CHECK_ENFORCED);
pub static mut page_table_check_disabled: usize = 0;
    EXPORT_SYMBOL(page_table_check_disabled);
#[no_mangle]
unsafe extern "C" fn early_page_table_check_param(buf: *mut c_char) -> c_int {
    return kstrtobool(buf, &__page_table_check_enabled);
    }
    early_param!("page_table_check", early_page_table_check_param);
#[no_mangle]
unsafe extern "C" fn need_page_table_check() -> bool __init {
    return __page_table_check_enabled;
    }
#[no_mangle]
unsafe extern "C" fn init_page_table_check()  {
    if (!__page_table_check_enabled) {
    return;
    }
    static_branch_disable(&page_table_check_disabled);
    }
pub static mut page_ext_operations: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn get_page_table_check(page_ext: *mut page_ext) -> *mut c_void {
    BUG_ON!(!page_ext);
    return page_ext_data(page_ext, &page_table_check_ops);
    }
//
// An entry is removed from the page table, decrement the counters for that page
// verify that it is of correct type and counters do not become negative.
//
#[no_mangle]
unsafe extern "C" fn page_table_check_clear(pfn: c_ulong, pgcnt: c_ulong) {
pub static mut iter: usize = 0;
pub static mut page_ext: *mut c_void = core::ptr::null_mut();
pub static mut page: *mut c_void = core::ptr::null_mut();
    let mut anon = 0;
    if (!pfn_valid(pfn)) {
    return;
    }
    page = pfn_to_page(pfn);
    BUG_ON!(PageSlab(page));
    anon = PageAnon(page);
    rcu_read_lock();
    for_each_page_ext(page, pgcnt, page_ext, iter) {
    let mut ptc = get_page_table_check(page_ext);
    if (anon) {
    BUG_ON!(atomic_read(&ptc.file_map_count));
    BUG_ON!(atomic_dec_return(&ptc.anon_map_count) < 0);
    } else {
    BUG_ON!(atomic_read(&ptc.anon_map_count));
    BUG_ON!(atomic_dec_return(&ptc.file_map_count) < 0);
    }
    }
    rcu_read_unlock();
    }
//
// A new entry is added to the page table, increment the counters for that page
// verify that it is of correct type and is not being mapped with a different
// type to a different process.
//
#[no_mangle]
pub unsafe extern "C" fn page_table_check_set(pfn: c_ulong, pgcnt: c_ulong, rw: bool) {
pub static mut iter: usize = 0;
pub static mut page_ext: *mut c_void = core::ptr::null_mut();
pub static mut page: *mut c_void = core::ptr::null_mut();
    let mut anon = 0;
    if (!pfn_valid(pfn)) {
    return;
    }
    page = pfn_to_page(pfn);
    BUG_ON!(PageSlab(page));
    anon = PageAnon(page);
    rcu_read_lock();
    for_each_page_ext(page, pgcnt, page_ext, iter) {
    let mut ptc = get_page_table_check(page_ext);
    if (anon) {
    BUG_ON!(atomic_read(&ptc.file_map_count));
    BUG_ON!(atomic_inc_return(&ptc.anon_map_count) > 1 && rw);
    } else {
    BUG_ON!(atomic_read(&ptc.anon_map_count));
    BUG_ON!(atomic_inc_return(&ptc.file_map_count) < 0);
    }
    }
    rcu_read_unlock();
    }
//
// page is on free list, or is being allocated, verify that counters are zeroes
// crash if they are not.
//
#[no_mangle]
pub unsafe extern "C" fn __page_table_check_zero(page: *mut page, order: c_uint) {
pub static mut iter: usize = 0;
pub static mut page_ext: *mut c_void = core::ptr::null_mut();
    BUG_ON!(PageSlab(page));
    rcu_read_lock();
    for_each_page_ext(page, 1 << order, page_ext, iter) {
    let mut ptc = get_page_table_check(page_ext);
    BUG_ON!(atomic_read(&ptc.anon_map_count));
    BUG_ON!(atomic_read(&ptc.file_map_count));
    }
    rcu_read_unlock();
    }
#[no_mangle]
pub unsafe extern "C" fn __page_table_check_pte_clear(mm: *mut mm_struct, addr: c_ulong, pte: pte_t) {
    if (&init_mm == mm) {
    return;
    }
    if (pte_user_accessible_page(mm, addr, pte) && !pte_special(pte)) {
    page_table_check_clear(pte_pfn(pte), PAGE_SIZE >> PAGE_SHIFT);
    }
    }
    EXPORT_SYMBOL(__page_table_check_pte_clear);
#[no_mangle]
pub unsafe extern "C" fn page_table_check_huge_zero_pmd(pmd: pmd_t) -> bool {
pub static mut pfn: c_ulong = 0;
    if (!pfn_valid(pfn)) {
    return false;
    }
    return is_huge_zero_folio(page_folio(pfn_to_page(pfn)));
    }
#[no_mangle]
pub unsafe extern "C" fn __page_table_check_pmd_clear(mm: *mut mm_struct, addr: c_ulong, pmd: pmd_t) {
    if (&init_mm == mm) {
    return;
    }
    if (pmd_user_accessible_page(mm, addr, pmd) &&
    !page_table_check_huge_zero_pmd(pmd)) {
    page_table_check_clear(pmd_pfn(pmd), PMD_SIZE >> PAGE_SHIFT);
    }
    }
    EXPORT_SYMBOL(__page_table_check_pmd_clear);
#[no_mangle]
pub unsafe extern "C" fn __page_table_check_pud_clear(mm: *mut mm_struct, addr: c_ulong, pud: pud_t) {
    if (&init_mm == mm) {
    return;
    }
    if (pud_user_accessible_page(mm, addr, pud)) {
    page_table_check_clear(pud_pfn(pud), PUD_SIZE >> PAGE_SHIFT);
    }
    }
    EXPORT_SYMBOL(__page_table_check_pud_clear);
// Whether the swap entry cached writable information
#[no_mangle]
pub unsafe extern "C" fn softleaf_cached_writable(entry: softleaf_t) -> bool {
    return softleaf_is_device_private_write(entry) ||
    softleaf_is_migration_write(entry);
    }
#[no_mangle]
unsafe extern "C" fn page_table_check_pte_flags(pte: pte_t) {
    if (pte_present(pte)) {
    WARN_ON_ONCE!(pte_uffd(pte) && pte_write(pte));
    } else if (pte_swp_uffd(pte)) {
pub static mut entry: softleaf_t = 0;
    WARN_ON_ONCE!(softleaf_cached_writable(entry));
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __page_table_check_ptes_set(mm: *mut mm_struct, addr: c_ulong, ptep: *mut pte_t, pte: pte_t, nr: c_uint) {
    let mut i = 0;
    if (&init_mm == mm) {
    return;
    }
    page_table_check_pte_flags(pte);
    for (i = 0; i < nr; i++) {
    __page_table_check_pte_clear(mm, addr + PAGE_SIZE * i, ptep_get(ptep + i));
    }
    if (pte_user_accessible_page(mm, addr, pte) && !pte_special(pte)) {
    page_table_check_set(pte_pfn(pte), nr, pte_write(pte));
    }
    }
    EXPORT_SYMBOL(__page_table_check_ptes_set);
#[no_mangle]
pub unsafe extern "C" fn page_table_check_pmd_flags(pmd: pmd_t) {
    if (pmd_present(pmd)) {
    if (pmd_uffd(pmd)) {
    WARN_ON_ONCE!(pmd_write(pmd));
    }
    } else if (pmd_swp_uffd(pmd)) {
pub static mut entry: softleaf_t = 0;
    WARN_ON_ONCE!(softleaf_cached_writable(entry));
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __page_table_check_pmds_set(mm: *mut mm_struct, addr: c_ulong, pmdp: *mut pmd_t, pmd: pmd_t, nr: c_uint) {
pub static mut stride: c_ulong = 0;
    let mut i = 0;
    if (&init_mm == mm) {
    return;
    }
    page_table_check_pmd_flags(pmd);
    for (i = 0; i < nr; i++) {
    __page_table_check_pmd_clear(mm, addr + PMD_SIZE * i, *(pmdp + i));
    }
    if (pmd_user_accessible_page(mm, addr, pmd) &&
    !page_table_check_huge_zero_pmd(pmd)) {
    page_table_check_set(pmd_pfn(pmd), stride * nr, pmd_write(pmd));
    }
    }
    EXPORT_SYMBOL(__page_table_check_pmds_set);
#[no_mangle]
pub unsafe extern "C" fn __page_table_check_puds_set(mm: *mut mm_struct, addr: c_ulong, pudp: *mut pud_t, pud: pud_t, nr: c_uint) {
pub static mut stride: c_ulong = 0;
    let mut i = 0;
    if (&init_mm == mm) {
    return;
    }
    for (i = 0; i < nr; i++) {
    __page_table_check_pud_clear(mm, addr + PUD_SIZE * i, *(pudp + i));
    }
    if (pud_user_accessible_page(mm, addr, pud)) {
    page_table_check_set(pud_pfn(pud), stride * nr, pud_write(pud));
    }
    }
    EXPORT_SYMBOL(__page_table_check_puds_set);
#[no_mangle]
pub unsafe extern "C" fn __page_table_check_pte_clear_range(mm: *mut mm_struct, addr: c_ulong, pmd: pmd_t) {
    if (&init_mm == mm) {
    return;
    }
    if (!pmd_bad(pmd) && !pmd_leaf(pmd)) {
    let mut ptep = pte_offset_map(&pmd, addr);
    let mut i = 0;
    if (WARN_ON!(!ptep)) {
    return;
    }
    while (i < PTRS_PER_PTE) {
    __page_table_check_pte_clear(mm, addr, ptep_get(ptep));
    addr += PAGE_SIZE;
    ptep += 1;
    }
    pte_unmap(ptep - PTRS_PER_PTE);
    }
    }