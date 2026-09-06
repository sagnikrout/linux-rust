//! Automatically rewritten from C to Rust
//! Source: mm/process_vm_access.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// linux/mm/process_vm_access.c
//
// Copyright (C) 2010-2011 Christopher Yeoh <cyeoh@au1.ibm.com>, IBM Corp.
//

//
// process_vm_rw_pages - read/write pages from task specified
// @pages: array of pointers to pages we want to copy
// @offset: offset in page to start copying from/to
// @len: number of bytes to copy
// @iter: where to copy to/from locally
// @vm_write: 0 means copy from, 1 means copy to
// Returns 0 on success, error code otherwise
//
#[no_mangle]
pub unsafe extern "C" fn process_vm_rw_pages(pages: *mut *mut page, offset: c_uint, len: size_t, iter: *mut iov_iter, vm_write: c_int) -> c_int {
// Do the copy for each page
    while (len && iov_iter_count(iter)) {
    let mut page = *pages += 1;
pub static mut copy: usize = 0;
    let mut copied = 0;
    if (copy > len) {
    copy = len;
    }
    if (vm_write) {
    copied = copy_page_from_iter(page, offset, copy, iter);
    }
    else {
    copied = copy_page_to_iter(page, offset, copy, iter);
    }
    len -= copied;
    if (copied < copy && iov_iter_count(iter)) {
    return -EFAULT;
    }
    offset = 0;
    }
    return 0;
    }
// Maximum number of pages kmalloc'd to hold struct page's during copy
pub const PVM_MAX_KMALLOC_PAGES: c_int = 2;
// Maximum number of pages that can be stored at a time

//
// process_vm_rw_single_vec - read/write pages from task specified
// @addr: start memory address of target process
// @len: size of area to copy to/from
// @iter: where to copy to/from locally
// @process_pages: pages area that can store at least
// nr_pages_to_copy struct page pointers
// @mm: mm for task
// @task: task to read/write from
// @vm_write: 0 means copy from, 1 means copy to
// Returns 0 on success or on failure error code
//
#[no_mangle]
pub unsafe extern "C" fn process_vm_rw_single_vec(addr: c_ulong, len: c_ulong, iter: *mut iov_iter, process_pages: *mut *mut page, mm: *mut mm_struct, task: *mut task_struct, vm_write: c_int) -> c_int {
pub static mut pa: c_ulong = 0;
pub static mut start_offset: c_ulong = 0;
    let mut nr_pages = 0;
pub static mut rc: isize = 0;
pub static mut flags: c_uint = 0;
// Work out address and page range required
    if (len == 0) {
    return 0;
    }
    nr_pages = (addr + len - 1) / PAGE_SIZE - addr / PAGE_SIZE + 1;
    if (vm_write) {
    flags |= FOLL_WRITE;
    }
    while (!rc && nr_pages && iov_iter_count(iter)) {
pub static mut pinned_pages: c_int = 0;
pub static mut locked: c_int = 1;
    let mut bytes = 0;
//
// Get the pages we're interested in.  We must
// access remotely because task/mm might not
// current/current->mm
//
    mmap_read_lock(mm);
    pinned_pages = pin_user_pages_remote(mm, pa, pinned_pages,
    flags, process_pages,
    &locked);
    if (locked) {
    mmap_read_unlock(mm);
    }
    if (pinned_pages <= 0) {
    return -EFAULT;
    }
    bytes = pinned_pages * PAGE_SIZE - start_offset;
    if (bytes > len) {
    bytes = len;
    }
    rc = process_vm_rw_pages(process_pages,
    start_offset, bytes, iter,
    vm_write);
    len -= bytes;
    start_offset = 0;
    nr_pages -= pinned_pages;
    pa += pinned_pages * PAGE_SIZE;
// If vm_write is set, the pages need to be made dirty:
    unpin_user_pages_dirty_lock(process_pages, pinned_pages,
    vm_write);
    }
    return rc;
    }
// Maximum number of entries for process pages array
    which lives on stack */
pub const PVM_MAX_PP_ARRAY_COUNT: c_int = 16;
//
// process_vm_rw_core - core of reading/writing pages from task specified
// @pid: PID of process to read/write from/to
// @iter: where to copy to/from locally
// @rvec: iovec array specifying where to copy to/from in the other process
// @riovcnt: size of rvec array
// @flags: currently unused
// @vm_write: 0 if reading from other process, 1 if writing to other process
//
// Returns the number of bytes read/written or error code. May
// return less bytes than expected if an error occurs during the copying
// process.
//
#[no_mangle]
pub unsafe extern "C" fn process_vm_rw_core(pid: pid_t, iter: *mut iov_iter, rvec: *mut iovec, riovcnt: c_ulong, flags: c_ulong, vm_write: c_int) -> ssize_t {
pub static mut task: *mut c_void = core::ptr::null_mut();
    struct page *pp_stack[PVM_MAX_PP_ARRAY_COUNT];
    let mut process_pages = pp_stack;
pub static mut mm: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
pub static mut rc: isize = 0;
pub static mut nr_pages: c_ulong = 0;
    let mut nr_pages_iov = 0;
    let mut iov_len = 0;
pub static mut total_len: usize = 0;
//
// Work out how many pages of struct pages we're going to need
// when eventually calling get_user_pages
//
    while (i < riovcnt) {
    iov_len = rvec[i].iov_len;
    if (iov_len > 0) {
    nr_pages_iov = ((unsigned long)rvec[i].iov_base
    + iov_len - 1)
    / PAGE_SIZE - (unsigned long)rvec[i].iov_base
    / PAGE_SIZE + 1;
    nr_pages = max(nr_pages, nr_pages_iov);
    }
    }
    if (nr_pages == 0) {
    return 0;
    }
    if (nr_pages > PVM_MAX_PP_ARRAY_COUNT) {
// For reliability don't try to kmalloc more than
    2 pages worth */
    process_pages = kmalloc(min_t(size_t, PVM_MAX_KMALLOC_PAGES * PAGE_SIZE,
    sizeof!*nr_pages),
    GFP_KERNEL);
    if (!process_pages) {
    return -ENOMEM;
    }
    }
// Get process information
    task = find_get_task_by_vpid(pid);
    if (!task) {
    rc = -ESRCH;
// goto;
    }
    mm = mm_access(task, PTRACE_MODE_ATTACH_REALCREDS);
    if (IS_ERR(mm)) {
    rc = PTR_ERR(mm);
//
// Explicitly map EACCES to EPERM as EPERM is a more
// appropriate error code for process_vw_readv/writev
//
    if (rc == -EACCES) {
    rc = -EPERM;
    }
// goto;
    }
    for (i = 0; i < riovcnt && iov_iter_count(iter) && !rc; i++) {
    rc = process_vm_rw_single_vec(
    (unsigned long)rvec[i].iov_base, rvec[i].iov_len,
    iter, process_pages, mm, task, vm_write);
    }
// copied = space before - space after
    total_len -= iov_iter_count(iter);
// If we have managed to copy any data at all then
    we return the number of bytes copied. Otherwise
    we return the error code */
    if (total_len) {
    rc = total_len;
    }
    mmput(mm);
// label;
    put_task_struct(task);
// label;
    if (process_pages != pp_stack) {
    kfree(process_pages);
    }
    return rc;
    }
//
// process_vm_rw - check iovecs before calling core routine
// @pid: PID of process to read/write from/to
// @lvec: iovec array specifying where to copy to/from locally
// @liovcnt: size of lvec array
// @rvec: iovec array specifying where to copy to/from in the other process
// @riovcnt: size of rvec array
// @flags: currently unused
// @vm_write: 0 if reading from other process, 1 if writing to other process
//
// Returns the number of bytes read/written or error code. May
// return less bytes than expected if an error occurs during the copying
// process.
//
#[no_mangle]
pub unsafe extern "C" fn process_vm_rw(pid: pid_t, lvec: *mut iovec, liovcnt: c_ulong, rvec: *mut iovec, riovcnt: c_ulong, flags: c_ulong, vm_write: c_int) -> ssize_t {
    struct iovec iovstack_l[UIO_FASTIOV];
    struct iovec iovstack_r[UIO_FASTIOV];
    let mut iov_l = iovstack_l;
pub static mut iov_r: *mut c_void = core::ptr::null_mut();
pub static mut iter: usize = 0;
    let mut rc = 0;
pub static mut dir: c_int = 0;
    if (flags != 0) {
    return -EINVAL;
    }
// Check iovecs
    rc = import_iovec(dir, lvec, liovcnt, UIO_FASTIOV, &iov_l, &iter);
    if (rc < 0) {
    return rc;
    }
    if (!iov_iter_count(&iter)) {
// goto;
    }
    iov_r = iovec_from_user(rvec, riovcnt, UIO_FASTIOV, iovstack_r,
    in_compat_syscall());
    if (IS_ERR(iov_r)) {
    rc = PTR_ERR(iov_r);
// goto;
    }
    rc = process_vm_rw_core(pid, &iter, iov_r, riovcnt, flags, vm_write);
    if (iov_r != iovstack_r) {
    kfree(iov_r);
    }
// label;
    kfree(iov_l);
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn sys_process_vm_readv(pid: usize, lvec: usize, liovcnt: usize, rvec: usize, riovcnt: usize, flags: usize) -> c_long {
    return process_vm_rw(pid, lvec, liovcnt, rvec, riovcnt, flags, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn sys_process_vm_writev(pid: usize, lvec: usize, liovcnt: usize, rvec: usize, riovcnt: usize, flags: usize) -> c_long {
    return process_vm_rw(pid, lvec, liovcnt, rvec, riovcnt, flags, 1);
    }