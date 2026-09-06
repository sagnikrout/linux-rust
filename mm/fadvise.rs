//! Automatically rewritten from C to Rust
//! Source: mm/fadvise.c
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
// mm/fadvise.c
//
// Copyright (C) 2002, Linus Torvalds
//
// 11Jan2003	Andrew Morton
// Initial version.
//

//
// POSIX_FADV_WILLNEED could set PG_Referenced, and POSIX_FADV_NOREUSE could
// deactivate the pages and clear PG_Referenced.
//
#[no_mangle]
pub unsafe extern "C" fn generic_fadvise(file: *mut file, offset: loff_t, len: loff_t, advice: c_int) -> c_int {
pub static mut inode: *mut c_void = core::ptr::null_mut();
pub static mut mapping: *mut c_void = core::ptr::null_mut();
pub static mut bdi: *mut c_void = core::ptr::null_mut();
    let mut endbyte = 0;			/* inclusive */
    let mut start_index;
    let mut end_index;
    let mut nrpages = 0;
    inode = file_inode(file);
    if (S_ISFIFO(inode.i_mode)) {
    return -ESPIPE;
    }
    mapping = file.f_mapping;
    if (!mapping || len < 0 || offset < 0) {
    return -EINVAL;
    }
    bdi = inode_to_bdi(mapping.host);
    if (IS_DAX(inode) || (bdi == &noop_backing_dev_info)) {
    match (advice) {
    POSIX_FADV_NORMAL => {
    }
    POSIX_FADV_RANDOM => {
    }
    POSIX_FADV_SEQUENTIAL => {
    }
    POSIX_FADV_WILLNEED => {
    }
    POSIX_FADV_NOREUSE => {
    }
    POSIX_FADV_DONTNEED => {
// no bad return value, but ignore advice
    // break;
    }
    _ => {
    return -EINVAL;
    }
    }
    return 0;
    }
//
// Careful about overflows. Len == 0 means "as much as possible".  Use
// unsigned math because signed overflows are undefined and UBSan
// complains.
//
    endbyte = (u64)offset + (u64)len;
    if (!len || endbyte < len) {
    endbyte = LLONG_MAX;
    }
    else {
    endbyte -= 1;		/* inclusive */
    }
    match (advice) {
    POSIX_FADV_NORMAL => {
    file.f_ra.ra_pages = bdi.ra_pages;
    spin_lock(&file.f_lock);
    file.f_mode &= ~(FMODE_RANDOM | FMODE_NOREUSE);
    spin_unlock(&file.f_lock);
    // break;
    }
    POSIX_FADV_RANDOM => {
    spin_lock(&file.f_lock);
    file.f_mode |= FMODE_RANDOM;
    spin_unlock(&file.f_lock);
    // break;
    }
    POSIX_FADV_SEQUENTIAL => {
    file.f_ra.ra_pages = bdi.ra_pages * 2;
    spin_lock(&file.f_lock);
    file.f_mode &= ~FMODE_RANDOM;
    spin_unlock(&file.f_lock);
    // break;
    }
    POSIX_FADV_WILLNEED => {
// First and last PARTIAL page!
    start_index = offset >> PAGE_SHIFT;
    end_index = endbyte >> PAGE_SHIFT;
// Careful about overflow on the "+1"
    nrpages = end_index - start_index + 1;
    if (!nrpages) {
    nrpages = ~0UL;
    }
    force_page_cache_readahead(mapping, file, start_index, nrpages);
    // break;
    }
    POSIX_FADV_NOREUSE => {
    spin_lock(&file.f_lock);
    file.f_mode |= FMODE_NOREUSE;
    spin_unlock(&file.f_lock);
    // break;
    }
    POSIX_FADV_DONTNEED => {
    filemap_flush_range(mapping, offset, endbyte);
//
// First and last FULL page! Partial pages are deliberately
// preserved on the expectation that it is better to preserve
// needed memory than to discard unneeded memory.
//
    start_index = (offset+(PAGE_SIZE-1)) >> PAGE_SHIFT;
    end_index = (endbyte >> PAGE_SHIFT);
//
// The page at end_index will be inclusively discarded according
// by invalidate_mapping_pages(), so subtracting 1 from
// end_index means we will skip the last page.  But if endbyte
// is page aligned or is at the end of file, we should not skip
// that page - discarding the last page is safe enough.
//
    if ((endbyte & ~PAGE_MASK) != ~PAGE_MASK &&
    endbyte != inode.i_size - 1) {
// First page is tricky as 0 - 1 = -1, but pgoff_t
// is unsigned, so the end_index >= start_index
// check below would be true and we'll discard the whole
// file cache which is not what was asked.
//
    if (end_index == 0) {
    // break;
    }
    end_index -= 1;
    }
    if (end_index >= start_index) {
pub static mut nr_failed: c_ulong = 0;
//
// It's common to FADV_DONTNEED right after
// the read or write that instantiates the
// pages, in which case there will be some
// sitting on the local LRU cache. Try to
// avoid the expensive remote drain and the
// second cache tree walk below by flushing
// them out right away.
//
    lru_add_drain();
    mapping_try_invalidate(mapping, start_index, end_index,
    &nr_failed);
//
// The failures may be due to the folio being
// in the LRU cache of a remote CPU. Drain all
// caches and try again.
//
    if (nr_failed) {
    lru_add_drain_all();
    invalidate_mapping_pages(mapping, start_index,
    end_index);
    }
    }
    // break;
    }
    _ => {
    return -EINVAL;
    }
    }
    return 0;
    }
    EXPORT_SYMBOL(generic_fadvise);
#[no_mangle]
pub unsafe extern "C" fn vfs_fadvise(file: *mut file, offset: loff_t, len: loff_t, advice: c_int) -> c_int {
    if (file.f_op.fadvise) {
    return file.f_op.fadvise(file, offset, len, advice);
    }
    return generic_fadvise(file, offset, len, advice);
    }
    EXPORT_SYMBOL(vfs_fadvise);

#[no_mangle]
pub unsafe extern "C" fn ksys_fadvise64_64(fd: c_int, offset: loff_t, len: loff_t, advice: c_int) -> c_int {
    CLASS(fd, f)(fd);
    if (fd_empty(f)) {
    return -EBADF;
    }
    return vfs_fadvise(fd_file(f), offset, len, advice);
    }
#[no_mangle]
pub unsafe extern "C" fn sys_fadvise64_64(fd: usize, offset: usize, len: usize, advice: usize) -> c_long {
    return ksys_fadvise64_64(fd, offset, len, advice);
    }

#[no_mangle]
pub unsafe extern "C" fn sys_fadvise64(fd: usize, offset: usize, len: usize, advice: usize) -> c_long {
    return ksys_fadvise64_64(fd, offset, len, advice);
    }

    COMPAT_SYSCALL_DEFINE6(fadvise64_64, int, fd, compat_arg_u64_dual(offset),
    compat_arg_u64_dual(len), int, advice)
    {
    return ksys_fadvise64_64(fd, compat_arg_u64_glue(offset),
    compat_arg_u64_glue(len), advice);
    }