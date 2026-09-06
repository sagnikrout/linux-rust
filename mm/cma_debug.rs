//! Automatically rewritten from C to Rust
//! Source: mm/cma_debug.c
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
// CMA DebugFS Interface
//
// Copyright (c) 2015 Sasha Levin <sasha.levin@oracle.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cma_mem {
    pub node: hlist_node,
    pub p: *mut page,
    pub n: c_ulong,
}

#[no_mangle]
unsafe extern "C" fn cma_debugfs_get(data: *mut c_void, val: *mut u64) -> c_int {
    let mut p = data;
// val = *p;
    return 0;
    }
    DEFINE_DEBUGFS_ATTRIBUTE(cma_debugfs_fops, cma_debugfs_get, core::ptr::null_mut(), "%llu\n");
#[no_mangle]
unsafe extern "C" fn cma_used_get(data: *mut c_void, val: *mut u64) -> c_int {
    let mut cma = data;
    spin_lock_irq(&cma.lock);
// val = cma->count - cma->available_count;
    spin_unlock_irq(&cma.lock);
    return 0;
    }
    DEFINE_DEBUGFS_ATTRIBUTE(cma_used_fops, cma_used_get, core::ptr::null_mut(), "%llu\n");
#[no_mangle]
unsafe extern "C" fn cma_maxchunk_get(data: *mut c_void, val: *mut u64) -> c_int {
    let mut cma = data;
pub static mut cmr: *mut c_void = core::ptr::null_mut();
pub static mut maxchunk: c_ulong = 0;
    unsigned long start, end;
    let mut bitmap_maxno = 0;
    let mut r = 0;
    spin_lock_irq(&cma.lock);
    while (r < cma.nranges) {
    cmr = &cma.ranges[r];
    bitmap_maxno = cma_bitmap_maxno(cma, cmr);
    for_each_clear_bitrange(start, end, cmr.bitmap, bitmap_maxno) {
    maxchunk = max(end - start, maxchunk);
    }
    }
    spin_unlock_irq(&cma.lock);
// val = (u64)maxchunk << cma->order_per_bit;
    return 0;
    }
    DEFINE_DEBUGFS_ATTRIBUTE(cma_maxchunk_fops, cma_maxchunk_get, core::ptr::null_mut(), "%llu\n");
#[no_mangle]
unsafe extern "C" fn cma_add_to_cma_mem_list(cma: *mut cma, mem: *mut cma_mem) {
    spin_lock(&cma.mem_head_lock);
    hlist_add_head(&mem.node, &cma.mem_head);
    spin_unlock(&cma.mem_head_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn cma_get_entry_from_list(cma: *mut cma) -> *mut c_void {
    let mut mem = core::ptr::null_mut();
    spin_lock(&cma.mem_head_lock);
    if (!hlist_empty(&cma.mem_head)) {
    mem = hlist_entry(cma.mem_head.first, cma_mem, node);
    hlist_del_init(&mem.node);
    }
    spin_unlock(&cma.mem_head_lock);
    return mem;
    }
#[no_mangle]
unsafe extern "C" fn cma_free_mem(cma: *mut cma, count: c_int) -> c_int {
    let mut mem = core::ptr::null_mut();
    while (count) {
    mem = cma_get_entry_from_list(cma);
    if (mem == core::ptr::null_mut()) {
    return 0;
    }
    if (mem.n <= count) {
    cma_release(cma, mem.p, mem.n);
    count -= mem.n;
    kfree(mem);
    } else if (cma.order_per_bit == 0) {
    cma_release(cma, mem.p, count);
    mem.p += count;
    mem.n -= count;
    count = 0;
    cma_add_to_cma_mem_list(cma, mem);
    } else {
    pr_debug!("cma: cannot release partial block when order_per_bit != 0\n");
    cma_add_to_cma_mem_list(cma, mem);
    break;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cma_free_write(data: *mut c_void, val: u64) -> c_int {
pub static mut pages: c_int = 0;
    let mut cma = data;
    return cma_free_mem(cma, pages);
    }
    DEFINE_DEBUGFS_ATTRIBUTE(cma_free_fops, core::ptr::null_mut(), cma_free_write, "%llu\n");
#[no_mangle]
unsafe extern "C" fn cma_alloc_mem(cma: *mut cma, count: c_int) -> c_int {
pub static mut mem: *mut c_void = core::ptr::null_mut();
pub static mut p: *mut c_void = core::ptr::null_mut();
    mem = kzalloc_obj(*mem);
    if (!mem) {
    return -ENOMEM;
    }
    p = cma_alloc(cma, count, 0, false);
    if (!p) {
    kfree(mem);
    return -ENOMEM;
    }
    mem.p = p;
    mem.n = count;
    cma_add_to_cma_mem_list(cma, mem);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cma_alloc_write(data: *mut c_void, val: u64) -> c_int {
pub static mut pages: c_int = 0;
    let mut cma = data;
    return cma_alloc_mem(cma, pages);
    }
    DEFINE_DEBUGFS_ATTRIBUTE(cma_alloc_fops, core::ptr::null_mut(), cma_alloc_write, "%llu\n");
#[no_mangle]
unsafe extern "C" fn cma_debugfs_add_one(cma: *mut cma, root_dentry: *mut dentry) {
    let mut tmp = core::ptr::null_mut();
    let mut dir = core::ptr::null_mut();
    let mut rangedir = core::ptr::null_mut();
    let mut r = 0;
    char rdirname[12];
pub static mut cmr: *mut c_void = core::ptr::null_mut();
    tmp = debugfs_create_dir(cma.name, root_dentry);
    debugfs_create_file("alloc", 0200, tmp, cma, &cma_alloc_fops);
    debugfs_create_file("free", 0200, tmp, cma, &cma_free_fops);
    debugfs_create_file("count", 0444, tmp, &cma.count, &cma_debugfs_fops);
    debugfs_create_file("order_per_bit", 0444, tmp,
    &cma.order_per_bit, &cma_debugfs_fops);
    debugfs_create_file("used", 0444, tmp, cma, &cma_used_fops);
    debugfs_create_file("maxchunk", 0444, tmp, cma, &cma_maxchunk_fops);
    rangedir = debugfs_create_dir("ranges", tmp);
    while (r < cma.nranges) {
    cmr = &cma.ranges[r];
    snprintf(rdirname, sizeof!(rdirname), "%d", r);
    dir = debugfs_create_dir(rdirname, rangedir);
    debugfs_create_file("base_pfn", 0444, dir,
    &cmr.base_pfn, &cma_debugfs_fops);
    cmr.dfs_bitmap.array = cmr.bitmap;
    cmr.dfs_bitmap.n_elements =
    DIV_ROUND_UP(cma_bitmap_maxno(cma, cmr),
    BITS_PER_BYTE * sizeof!(u32));
    debugfs_create_u32_array("bitmap", 0444, dir,
    &cmr.dfs_bitmap);
    }
//
// Backward compatible symlinks to range 0 for base_pfn and bitmap.
//
    debugfs_create_symlink("base_pfn", tmp, "ranges/0/base_pfn");
    debugfs_create_symlink("bitmap", tmp, "ranges/0/bitmap");
    }
#[no_mangle]
unsafe extern "C" fn cma_debugfs_init() -> c_int {
pub static mut cma_debugfs_root: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    cma_debugfs_root = debugfs_create_dir("cma", core::ptr::null_mut());
    for (i = 0; i < cma_area_count; i++) {
    if (test_bit(CMA_ACTIVATED, &cma_areas[i].flags))
    cma_debugfs_add_one(&cma_areas[i], cma_debugfs_root);
    }
    return 0;
    }
    late_initcall!(cma_debugfs_init);