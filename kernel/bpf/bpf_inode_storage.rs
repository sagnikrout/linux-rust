//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/bpf_inode_storage.c
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
// Copyright (c) 2019 Facebook
// Copyright 2020 Google LLC.
//

pub static mut inode_cache: usize = 0;
    static struct bpf_local_storage  **
    inode_storage_ptr(void *owner)
    {
    let mut inode = owner;
pub static mut bsb: *mut c_void = core::ptr::null_mut();
    bsb = bpf_inode(inode);
    if (!bsb) {
    return core::ptr::null_mut();
    }
    return &bsb.storage;
    }
#[no_mangle]
pub unsafe extern "C" fn inode_storage_lookup(inode: *mut inode, map: *mut bpf_map, cacheit_lockit: bool) -> *mut c_void {
pub static mut inode_storage: *mut c_void = core::ptr::null_mut();
pub static mut smap: *mut c_void = core::ptr::null_mut();
pub static mut bsb: *mut c_void = core::ptr::null_mut();
    bsb = bpf_inode(inode);
    if (!bsb) {
    return core::ptr::null_mut();
    }
    inode_storage =
    rcu_dereference_check(bsb.storage, bpf_rcu_lock_held());
    if (!inode_storage) {
    return core::ptr::null_mut();
    }
    smap = map;
    return bpf_local_storage_lookup(inode_storage, smap, cacheit_lockit);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_inode_storage_free(inode: *mut inode) {
pub static mut local_storage: *mut c_void = core::ptr::null_mut();
pub static mut bsb: *mut c_void = core::ptr::null_mut();
    bsb = bpf_inode(inode);
    if (!bsb) {
    return;
    }
    rcu_read_lock_dont_migrate();
    local_storage = rcu_dereference(bsb.storage);
    if (!local_storage) {
// goto;
    }
    bpf_local_storage_destroy(local_storage);
// label;
    rcu_read_unlock_migrate();
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_fd_inode_storage_lookup_elem(map: *mut bpf_map, key: *mut c_void) -> *mut c_void {
pub static mut sdata: *mut c_void = core::ptr::null_mut();
    CLASS(fd_raw, f)(*key);
    if (fd_empty(f)) {
    return ERR_PTR(-EBADF);
    }
    sdata = inode_storage_lookup(file_inode(fd_file(f)), map, true);
    return sdata ? sdata.data : core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_fd_inode_storage_update_elem(map: *mut bpf_map, key: *mut c_void, value: *mut c_void, map_flags: u64) -> c_long {
pub static mut sdata: *mut c_void = core::ptr::null_mut();
    CLASS(fd_raw, f)(*key);
    if (fd_empty(f)) {
    return -EBADF;
    }
    if (!inode_storage_ptr(file_inode(fd_file(f)))) {
    return -EBADF;
    }
    sdata = bpf_local_storage_update(file_inode(fd_file(f)),
    map,
    value, map_flags, false);
    return PTR_ERR_OR_ZERO(sdata);
    }
#[no_mangle]
unsafe extern "C" fn inode_storage_delete(inode: *mut inode, map: *mut bpf_map) -> c_int {
pub static mut sdata: *mut c_void = core::ptr::null_mut();
    sdata = inode_storage_lookup(inode, map, false);
    if (!sdata) {
    return -ENOENT;
    }
    return bpf_selem_unlink(SELEM(sdata));
    }
#[no_mangle]
unsafe extern "C" fn bpf_fd_inode_storage_delete_elem(map: *mut bpf_map, key: *mut c_void) -> c_long {
    CLASS(fd_raw, f)(*key);
    if (fd_empty(f)) {
    return -EBADF;
    }
    return inode_storage_delete(file_inode(fd_file(f)), map);
    }
    BPF_CALL_4(bpf_inode_storage_get, bpf_map *, map, inode *, inode,
    void *, value, u64, flags)
    {
pub static mut sdata: *mut c_void = core::ptr::null_mut();
    WARN_ON_ONCE!(!bpf_rcu_lock_held());
    if (flags & ~(BPF_LOCAL_STORAGE_GET_F_CREATE)) {
    return (unsigned long)core::ptr::null_mut();
    }
// explicitly check that the inode_storage_ptr is not
// NULL as inode_storage_lookup returns NULL in this case and
// bpf_local_storage_update expects the owner to have a
// valid storage pointer.
//
    if (!inode || !inode_storage_ptr(inode)) {
    return (unsigned long)core::ptr::null_mut();
    }
    sdata = inode_storage_lookup(inode, map, true);
    if (sdata) {
    return (unsigned long)sdata.data;
    }
// This helper must only called from where the inode is guaranteed
// to have a refcount and cannot be freed.
//
    if (flags & BPF_LOCAL_STORAGE_GET_F_CREATE) {
    sdata = bpf_local_storage_update(
    inode, map, value,
    BPF_NOEXIST, false);
    return IS_ERR(sdata) ? (unsigned long)core::ptr::null_mut() :
    (unsigned long)sdata.data;
    }
    return (unsigned long)core::ptr::null_mut();
    }
    BPF_CALL_2(bpf_inode_storage_delete, bpf_map *, map, inode *, inode)
    {
    WARN_ON_ONCE!(!bpf_rcu_lock_held());
    if (!inode) {
    return -EINVAL;
    }
// This helper must only called from where the inode is guaranteed
// to have a refcount and cannot be freed.
//
    return inode_storage_delete(inode, map);
    }
#[no_mangle]
pub unsafe extern "C" fn notsupp_get_next_key(map: *mut bpf_map, key: *mut c_void, next_key: *mut c_void) -> c_int {
    return -ENOTSUPP;
    }
#[no_mangle]
pub unsafe extern "C" fn inode_storage_map_alloc(attr: *mut union bpf_attr) -> *mut c_void {
//
// Do not allow allocation of BPF_MAP_TYPE_INODE_STORAGE if the BPF LSM
// was not initialized by the LSM framework at boot. Without proper
// initialization, the BPF inode security blob offset remains unprepared,
// causing bpf_inode() to calculate an invalid memory offset and corrupt
// inode->i_security.
//
    if (!bpf_lsm_initialized) {
    return ERR_PTR(-EOPNOTSUPP);
    }
    return bpf_local_storage_map_alloc(attr, &inode_cache);
    }
#[no_mangle]
unsafe extern "C" fn inode_storage_map_free(map: *mut bpf_map) {
    bpf_local_storage_map_free(map, &inode_cache);
    }
pub static mut bpf_map_ops: usize = 0;
    BTF_ID_LIST_SINGLE(bpf_inode_storage_btf_ids, struct, inode)
pub static mut bpf_func_proto: usize = 0;
pub static mut bpf_func_proto: usize = 0;