//! Automatically rewritten from C to Rust
//! Source: mm/mincore.c
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
// linux/mm/mincore.c
//
// Copyright (C) 1994-2006  Linus Torvalds
//
// The mincore() system call.
//

#[no_mangle]
pub unsafe extern "C" fn mincore_hugetlb(pte: *mut pte_t, hmask: c_ulong, addr: c_ulong, end: c_ulong, walk: *mut mm_walk) -> c_int {

pub static mut nr: c_ulong = 0;
    let mut resident = 0;
pub static mut ptl: *mut c_void = core::ptr::null_mut();
    let mut ptep;
    ptl = huge_pte_lock(hstate_vma(walk.vma), walk.mm, pte);
    ptep = huge_ptep_get(walk.mm, addr, pte);
    resident = !huge_pte_none(ptep) && !pte_is_marker(ptep);
    memset(walk.private, resident, nr);
    walk.private += nr;
    spin_unlock(ptl);

    BUG();

    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mincore_swap(entry: swp_entry_t, shmem: bool) -> c_uchar {
pub static mut si: *mut c_void = core::ptr::null_mut();
    let mut folio = core::ptr::null_mut();
pub static mut present: c_uchar = 0;
//
// Shmem mapping may contain swapin error entries, which are
// absent. Page table may contain migration or hwpoison
// entries which are always uptodate.
//
    if (!softleaf_is_swap(entry)) {
    return !shmem;
    }
    if (!IS_ENABLED!(CONFIG_SWAP)) {
    WARN_ON!(1);
    return 0;
    }
//
// Shmem mapping lookup is lockless, so we need to grab the swap
// device. mincore page table walk locks the PTL, and the swap
// device is stable, avoid touching the si for better performance.
//
    if (shmem) {
    si = get_swap_device(entry);
    if (!si) {
    return 0;
    }
    }
    folio = swap_cache_get_folio(entry);
    if (shmem) {
    put_swap_device(si);
    }
    if (folio) {
    present = folio_test_uptodate(folio);
    folio_put(folio);
    }
    return present;
    }
//
// Later we can get more picky about what "in core" means precisely.
// For now, simply check to see if the page is in the page cache,
// and is up to date; i.e. that no page-in operation would be required
// at this time if an application were to map and access this page.
//
#[no_mangle]
unsafe extern "C" fn mincore_page(mapping: *mut address_space, index: pgoff_t) -> c_uchar {
    let mut present = 0;
pub static mut folio: *mut c_void = core::ptr::null_mut();
//
// When tmpfs swaps out a page from a file, any process mapping that
// file will not get a swp_entry_t in its pte, but rather it is like
// any other file mapping (ie. marked !present and faulted in with
// tmpfs's .fault). So swapped out tmpfs mappings are tested here.
//
    folio = filemap_get_entry(mapping, index);
    if (!folio) {
    return 0;
    }
    if (xa_is_value(folio)) {
    if (!shmem_mapping(mapping)) {
    return 0;
    }
    return mincore_swap(radix_to_swp_entry(folio), true);
    }
    present = folio_test_uptodate(folio);
    folio_put(folio);
    return present;
    }
#[no_mangle]
pub unsafe extern "C" fn __mincore_unmapped_range(addr: c_ulong, end: c_ulong, vma: *mut vm_area_struct, vec: *mut c_uchar) -> c_int {
pub static mut nr: c_ulong = 0;
    let mut i = 0;
    if (vma.vm_file) {
    let mut pgoff;
    pgoff = linear_page_index(vma, addr);
    for (i = 0; i < nr; i++, pgoff++) {
    vec[i] = mincore_page(vma.vm_file.f_mapping, pgoff);
    }
    } else {
    for (i = 0; i < nr; i++) {
    vec[i] = 0;
    }
    }
    return nr;
    }
#[no_mangle]
pub unsafe extern "C" fn mincore_unmapped_range(addr: c_ulong, end: c_ulong, depth: __always_unused int, walk: *mut mm_walk) -> c_int {
    walk.private += __mincore_unmapped_range(addr, end,
    walk.vma, walk.private);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mincore_pud_entry(pudp: *mut pud_t, addr: c_ulong, end: c_ulong, walk: *mut mm_walk) -> c_int {
    if (pud_is_huge(pudp_get(pudp))) {
pub static mut nr: c_ulong = 0;
    memset(walk.private, 1, nr);
    walk.private += nr;
    walk.action = ACTION_CONTINUE;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mincore_pte_range(pmd: *mut pmd_t, addr: c_ulong, end: c_ulong, walk: *mut mm_walk) -> c_int {
pub static mut ptl: *mut c_void = core::ptr::null_mut();
    let mut vma = walk.vma;
pub static mut ptep: *mut c_void = core::ptr::null_mut();
    let mut vec = walk.private;
pub static mut nr: c_int = 0;
    let mut step = 0;
    let mut i = 0;
    ptl = pmd_trans_huge_lock(pmd, vma);
    if (ptl) {
    memset(vec, 1, nr);
    spin_unlock(ptl);
// goto;
    }
    ptep = pte_offset_map_lock(walk.mm, pmd, addr, &ptl);
    if (!ptep) {
    walk.action = ACTION_AGAIN;
    return 0;
    }
    while (addr != end) {
pub static mut pte: pte_t = 0;
    step = 1;
// We need to do cache lookup too for markers
    if (pte_none(pte) || pte_is_marker(pte)) {
    __mincore_unmapped_range(addr, addr + PAGE_SIZE,
    vma, vec);
    }
if true {
pub static mut batch: c_uint = 0;
    if (batch > 1) {
pub static mut max_nr: c_uint = 0;
    step = min_t(unsigned int, batch, max_nr);
    }
    for (i = 0; i < step; i++) {
    vec[i] = 1;
    }
    } else { /* pte is a swap entry */
pub static mut entry: softleaf_t = 0;
// vec = mincore_swap(entry, false);
    }
    vec += step;
    }
    pte_unmap_unlock(ptep - 1, ptl);
// label;
    walk.private += nr;
    cond_resched();
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn can_do_mincore(vma: *mut vm_area_struct) -> bool {
    if (vma_is_anonymous(vma)) {
    return true;
    }
    if (!vma.vm_file) {
    return false;
    }
//
// Reveal pagecache information only for non-anonymous mappings that
// correspond to the files the calling process could (if tried) open
// for writing; otherwise we'd be including shared non-exclusive
// mappings, which opens a side channel.
//
    return file_owner_or_capable(vma.vm_file) ||
    file_permission(vma.vm_file, MAY_WRITE) == 0;
    }
pub static mut mm_walk_ops: usize = 0;
//
// Do a chunk of "sys_mincore()". We've already checked
// all the arguments, we hold the mmap semaphore: we should
// just return the amount of info we're asked for.
//
#[no_mangle]
unsafe extern "C" fn do_mincore(addr: c_ulong, pages: c_ulong, vec: *mut c_uchar) -> c_long {
pub static mut vma: *mut c_void = core::ptr::null_mut();
    let mut end = 0;
    let mut err = 0;
    vma = vma_lookup(current.mm, addr);
    if (!vma) {
    return -ENOMEM;
    }
    end = min(vma.vm_end, addr + (pages << PAGE_SHIFT));
    if (!can_do_mincore(vma)) {
pub static mut pages: c_ulong = 0;
    memset(vec, 1, pages);
    return pages;
    }
    err = walk_page_range_vma(vma, addr, end, &mincore_walk_ops, vec);
    if (err < 0) {
    return err;
    }
    return (end - addr) >> PAGE_SHIFT;
    }
//
// The mincore(2) system call.
//
// mincore() returns the memory residency status of the pages in the
// current process's address space specified by [addr, addr + len).
// The status is returned in a vector of bytes.  The least significant
// bit of each byte is 1 if the referenced page is in memory, otherwise
// it is zero.
//
// Because the status of a page can change after mincore() checks it
// but before it returns to the application, the returned vector may
// contain stale information.  Only locked pages are guaranteed to
// remain in memory.
//
// return values:
// zero    - success
// -EFAULT - vec points to an illegal address
// -EINVAL - addr is not a multiple of PAGE_SIZE
// -ENOMEM - Addresses in the range [addr, addr + len] are
// invalid for the address space of this process, or
// specify one or more pages which are not currently
// mapped
// -EAGAIN - A kernel resource was temporarily unavailable.
//
#[no_mangle]
pub unsafe extern "C" fn sys_mincore(start: usize, len: usize, vec: usize) -> c_long {
    let mut retval = 0;
    let mut pages = 0;
pub static mut tmp: *mut c_void = core::ptr::null_mut();
    start = untagged_addr(start);
// Check the start address: needs to be page-aligned..
    if (unlikely(start & ~PAGE_MASK)) {
    return -EINVAL;
    }
// ..and we need to be passed a valid user-space range
    if (!access_ok( start, len)) {
    return -ENOMEM;
    }
// This also avoids any overflows on PAGE_ALIGN
    pages = len >> PAGE_SHIFT;
    pages += (offset_in_page(len)) != 0;
    if (!access_ok(vec, pages)) {
    return -EFAULT;
    }
    tmp = kmalloc(PAGE_SIZE, GFP_KERNEL);
    if (!tmp) {
    return -EAGAIN;
    }
    retval = 0;
    while (pages) {
//
// Do at most PAGE_SIZE entries per iteration, due to
// the temporary buffer size.
//
    mmap_read_lock(current.mm);
    retval = do_mincore(start, min(pages, PAGE_SIZE), tmp);
    mmap_read_unlock(current.mm);
    if (retval <= 0) {
    break;
    }
    if (copy_to_user(vec, tmp, retval)) {
    retval = -EFAULT;
    break;
    }
    pages -= retval;
    vec += retval;
    start += retval << PAGE_SHIFT;
    retval = 0;
    }
    kfree(tmp);
    return retval;
    }