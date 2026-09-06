//! Automatically rewritten from C to Rust
//! Source: kernel/liveupdate/kexec_handover_debugfs.c
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
// kexec_handover_debugfs.c - kexec handover debugfs interfaces
// Copyright (C) 2023 Alexander Graf <graf@amazon.com>
// Copyright (C) 2025 Microsoft Corporation, Mike Rapoport <rppt@kernel.org>
// Copyright (C) 2025 Google LLC, Changyuan Lyu <changyuanl@google.com>
// Copyright (C) 2025 Google LLC, Pasha Tatashin <pasha.tatashin@soleen.com>
//

pub static mut debugfs_root: *mut c_void = core::ptr::null_mut();
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fdt_debugfs {
    pub list: list_head,
    pub wrapper: debugfs_blob_wrapper,
    pub file: *mut dentry,
}

#[no_mangle]
pub unsafe extern "C" fn __kho_debugfs_blob_add(list: *mut list_head, dir: *mut dentry, name: *mut c_char, blob: *mut c_void, size: size_t) -> c_int {
pub static mut f: *mut c_void = core::ptr::null_mut();
pub static mut file: *mut c_void = core::ptr::null_mut();
    f = kmalloc_obj(*f);
    if (!f) {
    return -ENOMEM;
    }
    f.wrapper.data = blob;
    f.wrapper.size = size;
    file = debugfs_create_blob(name, 0400, dir, &f.wrapper);
    if (IS_ERR(file)) {
    kfree(f);
    return PTR_ERR(file);
    }
    f.file = file;
    list_add(&f.list, list);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn kho_debugfs_blob_add(dbg: *mut kho_debugfs, name: *mut c_char, blob: *mut c_void, size: size_t, root: bool) -> c_int {
pub static mut dir: *mut c_void = core::ptr::null_mut();
    if (root) {
    dir = dbg.dir;
    }
    else {
    dir = dbg.sub_fdt_dir;
    }
    return __kho_debugfs_blob_add(&dbg.fdt_list, dir, name, blob, size);
    }
#[no_mangle]
pub unsafe extern "C" fn kho_debugfs_blob_remove(dbg: *mut kho_debugfs, blob: *mut c_void) {
pub static mut ff: *mut c_void = core::ptr::null_mut();
    list_for_each_entry(ff, &dbg.fdt_list, list) {
    if (ff.wrapper.data == blob) {
    debugfs_remove(ff.file);
    list_del(&ff.list);
    kfree(ff);
    break;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn scratch_phys_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    for (int i = 0; i < kho_scratch_cnt; i++) {
    seq_printf(m, "0x%llx\n", kho_scratch[i].addr);
    }
    return 0;
    }
pub static mut scratch_phys: usize = 0;
#[no_mangle]
unsafe extern "C" fn scratch_len_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    for (int i = 0; i < kho_scratch_cnt; i++) {
    seq_printf(m, "0x%llx\n", kho_scratch[i].size);
    }
    return 0;
    }
pub static mut scratch_len: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn kho_in_debugfs_init(dbg: *mut kho_debugfs, fdt: *const c_void) -> __init void {
    let mut dir = core::ptr::null_mut();
    let mut sub_fdt_dir = core::ptr::null_mut();
    let mut err = 0;
    let mut child = 0;
    INIT_LIST_HEAD(&dbg.fdt_list);
    dir = debugfs_create_dir("in", debugfs_root);
    if (IS_ERR(dir)) {
    err = PTR_ERR(dir);
// goto;
    }
    sub_fdt_dir = debugfs_create_dir("sub_fdts", dir);
    if (IS_ERR(sub_fdt_dir)) {
    err = PTR_ERR(sub_fdt_dir);
// goto;
    }
    err = __kho_debugfs_blob_add(&dbg.fdt_list, dir, "fdt", fdt,
    fdt_totalsize(fdt));
    if (err) {
// goto;
    }
    fdt_for_each_subnode(child, fdt, 0) {
pub static mut len: c_int = 0;
    let mut name = fdt_get_name(fdt, child, core::ptr::null_mut());
pub static mut blob_phys: *mut c_void = core::ptr::null_mut();
pub static mut blob_size: *mut c_void = core::ptr::null_mut();
pub static mut blob: *mut c_void = core::ptr::null_mut();
    blob_phys = fdt_getprop(fdt, child,
    KHO_SUB_TREE_PROP_NAME, &len);
    if (!blob_phys) {
    continue;
    }
    if (len != sizeof!(*blob_phys)) {
    pr_warn!("node %s prop %s has invalid length: %d\n",
    name, KHO_SUB_TREE_PROP_NAME, len);
    continue;
    }
    blob_size = fdt_getprop(fdt, child,
    KHO_SUB_TREE_SIZE_PROP_NAME, &len);
    if (!blob_size || len != sizeof!(*blob_size)) {
    pr_warn!("node %s missing or invalid %s property\n",
    name, KHO_SUB_TREE_SIZE_PROP_NAME);
    continue;
    }
    blob = phys_to_virt(*blob_phys);
    err = __kho_debugfs_blob_add(&dbg.fdt_list, sub_fdt_dir, name,
    blob, *blob_size);
    if (err) {
    pr_warn!("failed to add blob %s to debugfs: %pe\n",
    name, ERR_PTR(err));
    continue;
    }
    }
    dbg.dir = dir;
    dbg.sub_fdt_dir = sub_fdt_dir;
    return;
// label;
    debugfs_remove_recursive(dir);
// label;
//
// Failure to create /sys/kernel/debug/kho/in does not prevent
// reviving state from KHO and setting up KHO for the next
// kexec.
//
    if (err) {
    pr_err!("failed exposing handover FDT in debugfs: %pe\n",
    ERR_PTR(err));
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kho_out_debugfs_init(dbg: *mut kho_debugfs) -> __init int {
    let mut dir = core::ptr::null_mut();
    let mut f = core::ptr::null_mut();
    let mut sub_fdt_dir = core::ptr::null_mut();
    INIT_LIST_HEAD(&dbg.fdt_list);
    dir = debugfs_create_dir("out", debugfs_root);
    if (IS_ERR(dir)) {
    return -ENOMEM;
    }
    sub_fdt_dir = debugfs_create_dir("sub_fdts", dir);
    if (IS_ERR(sub_fdt_dir)) {
// goto;
    }
    f = debugfs_create_file("scratch_phys", 0400, dir, core::ptr::null_mut(),
    &scratch_phys_fops);
    if (IS_ERR(f)) {
// goto;
    }
    f = debugfs_create_file("scratch_len", 0400, dir, core::ptr::null_mut(),
    &scratch_len_fops);
    if (IS_ERR(f)) {
// goto;
    }
    dbg.dir = dir;
    dbg.sub_fdt_dir = sub_fdt_dir;
    return 0;
// label;
    debugfs_remove_recursive(dir);
    return -ENOENT;
    }
#[no_mangle]
pub unsafe extern "C" fn kho_debugfs_init() -> __init int {
    debugfs_root = debugfs_create_dir("kho", core::ptr::null_mut());
    if (IS_ERR(debugfs_root)) {
    return -ENOENT;
    }
    return 0;
    }