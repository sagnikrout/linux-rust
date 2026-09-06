//! Automatically rewritten from C to Rust
//! Source: mm/list_lru.c
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
// Copyright (c) 2013 Red Hat, Inc. and Parallels Inc. All rights reserved.
// Authors: David Chinner and Glauber Costa
//
// Generic LRU infrastructure
//

#[no_mangle]
pub unsafe extern "C" fn lock_list_lru(l: *mut list_lru_one, irq: bool, irq_flags: *mut c_ulong) {
    if (irq_flags) {
    spin_lock_irqsave(&l.lock, *irq_flags);
    }

    else if (irq) {
    spin_lock_irq(&l.lock);
    }
    else {
    spin_lock(&l.lock);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn unlock_list_lru(l: *mut list_lru_one, irq_off: bool, irq_flags: *mut c_ulong) {
    if (irq_flags) {
    spin_unlock_irqrestore(&l.lock, *irq_flags);
    }

    else if (irq_off) {
    spin_unlock_irq(&l.lock);
    }
    else {
    spin_unlock(&l.lock);
    }
    }

pub static mut memcg_list_lrus: usize = 0;
pub static mut list_lrus_mutex: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn list_lru_memcg_aware(lru: *mut list_lru) -> bool {
    return lru.memcg_aware;
    }
#[no_mangle]
unsafe extern "C" fn list_lru_register(lru: *mut list_lru) {
    if (!list_lru_memcg_aware(lru)) {
    return;
    }
    mutex_lock(&list_lrus_mutex);
    list_add(&lru.list, &memcg_list_lrus);
    mutex_unlock(&list_lrus_mutex);
    }
#[no_mangle]
unsafe extern "C" fn list_lru_unregister(lru: *mut list_lru) {
    if (!list_lru_memcg_aware(lru)) {
    return;
    }
    mutex_lock(&list_lrus_mutex);
    list_del(&lru.list);
    mutex_unlock(&list_lrus_mutex);
    }
#[no_mangle]
unsafe extern "C" fn lru_shrinker_id(lru: *mut list_lru) -> c_int {
    return lru.shrinker_id;
    }
#[no_mangle]
pub unsafe extern "C" fn list_lru_from_memcg_idx(lru: *mut list_lru, nid: c_int, idx: c_int) -> *mut c_void {
    if (list_lru_memcg_aware(lru) && idx >= 0) {
    let mut mlru = xa_load(&lru.xa, idx);
    return mlru ? &mlru.node[nid] : core::ptr::null_mut();
    }
    return &lru.node[nid].lru;
    }
#[no_mangle]
pub unsafe extern "C" fn lock_list_lru_of_memcg(lru: *mut list_lru, nid: c_int, memcg: *mut *mut mem_cgroup, irq: bool, irq_flags: *mut c_ulong, skip_empty: bool) -> *mut c_void {
pub static mut l: *mut c_void = core::ptr::null_mut();
    rcu_read_lock();
// label;
    l = list_lru_from_memcg_idx(lru, nid, memcg_kmem_id(*memcg));
    if (likely(l)) {
    lock_list_lru(l, irq, irq_flags);
    if (likely(READ_ONCE(l.nr_items) != LONG_MIN)) {
    rcu_read_unlock();
    return l;
    }
    unlock_list_lru(l, irq, irq_flags);
    }
//
// Caller may simply bail out if raced with reparenting or
// may iterate through the list_lru and expect empty slots.
//
    if (skip_empty) {
    rcu_read_unlock();
    return core::ptr::null_mut();
    }
    VM_WARN_ON(!css_is_dying(&(*memcg).css));
// memcg = parent_mem_cgroup(*memcg);
// goto;
    }

#[no_mangle]
unsafe extern "C" fn list_lru_register(lru: *mut list_lru) {
    }
#[no_mangle]
unsafe extern "C" fn list_lru_unregister(lru: *mut list_lru) {
    }
#[no_mangle]
unsafe extern "C" fn lru_shrinker_id(lru: *mut list_lru) -> c_int {
    return -1;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: list_lru_memcg_aware
pub unsafe extern "C" fn list_lru_memcg_aware_dup(lru: *mut list_lru) -> bool {
    return false;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: list_lru_from_memcg_idx
pub unsafe extern "C" fn list_lru_from_memcg_idx_dup(lru: *mut list_lru, nid: c_int, idx: c_int) -> *mut c_void {
    return &lru.node[nid].lru;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: lock_list_lru_of_memcg
pub unsafe extern "C" fn lock_list_lru_of_memcg_dup(lru: *mut list_lru, nid: c_int, memcg: *mut *mut mem_cgroup, irq: bool, irq_flags: *mut c_ulong, skip_empty: bool) -> *mut c_void {
    let mut l = &lru.node[nid].lru;
    lock_list_lru(l, irq, irq_flags);
    return l;
    }

#[no_mangle]
pub unsafe extern "C" fn list_lru_lock(lru: *mut list_lru, nid: c_int, memcg: *mut *mut mem_cgroup) -> *mut c_void {
    return lock_list_lru_of_memcg(lru, nid, memcg, /*irq=*/false,
// irq_flags=*/NULL, /*skip_empty=*/false);
    }
#[no_mangle]
pub unsafe extern "C" fn list_lru_unlock(l: *mut list_lru_one) {
    unlock_list_lru(l, /*irq_off=*/false, /*irq_flags=*/core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn list_lru_lock_irq(lru: *mut list_lru, nid: c_int, memcg: *mut *mut mem_cgroup) -> *mut c_void {
    return lock_list_lru_of_memcg(lru, nid, memcg, /*irq=*/true,
// irq_flags=*/NULL, /*skip_empty=*/false);
    }
#[no_mangle]
pub unsafe extern "C" fn list_lru_unlock_irq(l: *mut list_lru_one) {
    unlock_list_lru(l, /*irq_off=*/true, /*irq_flags=*/core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn list_lru_lock_irqsave(lru: *mut list_lru, nid: c_int, memcg: *mut *mut mem_cgroup, flags: *mut c_ulong) -> *mut c_void {
    return lock_list_lru_of_memcg(lru, nid, memcg, /*irq=*/true,
// irq_flags=*/flags, /*skip_empty=*/false);
    }
#[no_mangle]
pub unsafe extern "C" fn list_lru_unlock_irqrestore(l: *mut list_lru_one, flags: *mut c_ulong) {
    unlock_list_lru(l, /*irq_off=*/true, /*irq_flags=*/flags);
    }
#[no_mangle]
pub unsafe extern "C" fn __list_lru_add(lru: *mut list_lru, l: *mut list_lru_one, item: *mut list_head, nid: c_int, memcg: *mut mem_cgroup) -> bool {
    if (list_empty(item)) {
    list_add_tail(item, &l.list);
//
// Set shrinker bit on the memcg that owns the locked
// sublist - lock_list_lru_of_memcg() may have walked up
// past a dying memcg, and the bit must be set there.
//
    if (!l.nr_items++) {
    set_shrinker_bit(memcg, nid, lru_shrinker_id(lru));
    }
    atomic_long_inc(&lru.node[nid].nr_items);
    return true;
    }
    return false;
    }
    EXPORT_SYMBOL_GPL(list_lru_add);
#[no_mangle]
pub unsafe extern "C" fn __list_lru_del(lru: *mut list_lru, l: *mut list_lru_one, item: *mut list_head, nid: c_int) -> bool {
    if (!list_empty(item)) {
    list_del_init(item);
    l.nr_items -= 1;
    atomic_long_dec(&lru.node[nid].nr_items);
    return true;
    }
    return false;
    }
// The caller must ensure the memcg lifetime.
#[no_mangle]
pub unsafe extern "C" fn list_lru_add(lru: *mut list_lru, item: *mut list_head, nid: c_int, memcg: *mut mem_cgroup) -> bool {
pub static mut l: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    l = list_lru_lock(lru, nid, &memcg);
    ret = __list_lru_add(lru, l, item, nid, memcg);
    list_lru_unlock(l);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn list_lru_add_irq(lru: *mut list_lru, item: *mut list_head, nid: c_int, memcg: *mut mem_cgroup) -> bool {
pub static mut l: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    l = list_lru_lock_irq(lru, nid, &memcg);
    ret = __list_lru_add(lru, l, item, nid, memcg);
    list_lru_unlock_irq(l);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn list_lru_add_obj(lru: *mut list_lru, item: *mut list_head) -> bool {
    let mut ret = 0;
pub static mut nid: c_int = 0;
    if (list_lru_memcg_aware(lru)) {
    rcu_read_lock();
    ret = list_lru_add(lru, item, nid, mem_cgroup_from_virt(item));
    rcu_read_unlock();
    } else {
    ret = list_lru_add(lru, item, nid, core::ptr::null_mut());
    }
    return ret;
    }
    EXPORT_SYMBOL_GPL(list_lru_add_obj);
// The caller must ensure the memcg lifetime.
#[no_mangle]
pub unsafe extern "C" fn list_lru_del(lru: *mut list_lru, item: *mut list_head, nid: c_int, memcg: *mut mem_cgroup) -> bool {
pub static mut l: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    l = list_lru_lock(lru, nid, &memcg);
    ret = __list_lru_del(lru, l, item, nid);
    list_lru_unlock(l);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn list_lru_del_obj(lru: *mut list_lru, item: *mut list_head) -> bool {
    let mut ret = 0;
pub static mut nid: c_int = 0;
    if (list_lru_memcg_aware(lru)) {
    rcu_read_lock();
    ret = list_lru_del(lru, item, nid, mem_cgroup_from_virt(item));
    rcu_read_unlock();
    } else {
    ret = list_lru_del(lru, item, nid, core::ptr::null_mut());
    }
    return ret;
    }
    EXPORT_SYMBOL_GPL(list_lru_del_obj);
#[no_mangle]
pub unsafe extern "C" fn list_lru_isolate(list: *mut list_lru_one, item: *mut list_head) {
    list_del_init(item);
    list.nr_items -= 1;
    }
    EXPORT_SYMBOL_GPL(list_lru_isolate);
#[no_mangle]
pub unsafe extern "C" fn list_lru_isolate_move(list: *mut list_lru_one, item: *mut list_head, head: *mut list_head) {
    list_move(item, head);
    list.nr_items -= 1;
    }
    EXPORT_SYMBOL_GPL(list_lru_isolate_move);
#[no_mangle]
pub unsafe extern "C" fn list_lru_count_one(lru: *mut list_lru, nid: c_int, memcg: *mut mem_cgroup) -> c_ulong {
pub static mut l: *mut c_void = core::ptr::null_mut();
    let mut count = 0;
    rcu_read_lock();
    l = list_lru_from_memcg_idx(lru, nid, memcg_kmem_id(memcg));
    count = l ? READ_ONCE(l.nr_items) : 0;
    rcu_read_unlock();
    if (unlikely(count < 0)) {
    count = 0;
    }
    return count;
    }
    EXPORT_SYMBOL_GPL(list_lru_count_one);
#[no_mangle]
pub unsafe extern "C" fn list_lru_count_node(lru: *mut list_lru, nid: c_int) -> c_ulong {
pub static mut nlru: *mut c_void = core::ptr::null_mut();
    nlru = &lru.node[nid];
    return atomic_long_read(&nlru.nr_items);
    }
    EXPORT_SYMBOL_GPL(list_lru_count_node);
#[no_mangle]
pub unsafe extern "C" fn __list_lru_walk_one(lru: *mut list_lru, nid: c_int, memcg: *mut mem_cgroup, isolate: list_lru_walk_cb, cb_arg: *mut c_void, nr_to_walk: *mut c_ulong, irq_off: bool) -> c_ulong {
    let mut nlru = &lru.node[nid];
    let mut l = core::ptr::null_mut();
    let mut item = core::ptr::null_mut();
    let mut n = core::ptr::null_mut();
pub static mut isolated: c_ulong = 0;
// label;
    l = lock_list_lru_of_memcg(lru, nid, &memcg, /*irq=*/irq_off,
// irq_flags=*/NULL, /*skip_empty=*/true);
    if (!l) {
    return isolated;
    }
    list_for_each_safe(item, n, &l.list) {
    enum lru_status ret;
//
// decrement nr_to_walk first so that we don't livelock if we
// get stuck on large numbers of LRU_RETRY items
//
    if (!*nr_to_walk) {
    break;
    }
    --*nr_to_walk;
    ret = isolate(item, l, cb_arg);
    match (ret) {
//
// LRU_RETRY, LRU_REMOVED_RETRY and LRU_STOP will drop the lru
// lock. List traversal will have to restart from scratch.
//
    LRU_RETRY => {
// goto;
    }
    LRU_REMOVED_RETRY => {
    fallthrough;
    }
    LRU_REMOVED => {
    isolated += 1;
    atomic_long_dec(&nlru.nr_items);
    if (ret == LRU_REMOVED_RETRY) {
// goto;
    }
    // break;
    }
    LRU_ROTATE => {
    list_move_tail(item, &l.list);
    // break;
    }
    LRU_SKIP => {
    // break;
    }
    LRU_STOP => {
// goto;
    }
    _ => {
    BUG();
    }
    }
    }
    unlock_list_lru(l, irq_off, core::ptr::null_mut());
// label;
    return isolated;
    }
#[no_mangle]
pub unsafe extern "C" fn list_lru_walk_one(lru: *mut list_lru, nid: c_int, memcg: *mut mem_cgroup, isolate: list_lru_walk_cb, cb_arg: *mut c_void, nr_to_walk: *mut c_ulong) -> c_ulong {
    return __list_lru_walk_one(lru, nid, memcg, isolate,
    cb_arg, nr_to_walk, false);
    }
    EXPORT_SYMBOL_GPL(list_lru_walk_one);
#[no_mangle]
pub unsafe extern "C" fn list_lru_walk_one_irq(lru: *mut list_lru, nid: c_int, memcg: *mut mem_cgroup, isolate: list_lru_walk_cb, cb_arg: *mut c_void, nr_to_walk: *mut c_ulong) -> c_ulong {
    return __list_lru_walk_one(lru, nid, memcg, isolate,
    cb_arg, nr_to_walk, true);
    }
#[no_mangle]
pub unsafe extern "C" fn list_lru_walk_node(lru: *mut list_lru, nid: c_int, isolate: list_lru_walk_cb, cb_arg: *mut c_void, nr_to_walk: *mut c_ulong) -> c_ulong {
pub static mut isolated: c_long = 0;
    isolated += list_lru_walk_one(lru, nid, core::ptr::null_mut(), isolate, cb_arg,
    nr_to_walk);

    if (*nr_to_walk > 0 && list_lru_memcg_aware(lru)) {
pub static mut mlru: *mut c_void = core::ptr::null_mut();
pub static mut memcg: *mut c_void = core::ptr::null_mut();
    let mut index = 0;
    xa_for_each(&lru.xa, index, mlru) {
    rcu_read_lock();
    memcg = mem_cgroup_from_private_id(index);
    if (!mem_cgroup_tryget(memcg)) {
    rcu_read_unlock();
    continue;
    }
    rcu_read_unlock();
    isolated += __list_lru_walk_one(lru, nid, memcg,
    isolate, cb_arg,
    nr_to_walk, false);
    mem_cgroup_put(memcg);
    if (*nr_to_walk <= 0) {
    break;
    }
    }
    }

    return isolated;
    }
    EXPORT_SYMBOL_GPL(list_lru_walk_node);
#[no_mangle]
unsafe extern "C" fn init_one_lru(lru: *mut list_lru, l: *mut list_lru_one) {
    INIT_LIST_HEAD(&l.list);
    spin_lock_init(&l.lock);
    l.nr_items = 0;

    if (lru.key) {
    lockdep_set_class(&l.lock, lru.key);
    }

    }

#[no_mangle]
pub unsafe extern "C" fn memcg_init_list_lru_one(lru: *mut list_lru, gfp: gfp_t) -> *mut c_void {
    let mut nid = 0;
pub static mut mlru: *mut c_void = core::ptr::null_mut();
    mlru = kmalloc_flex(*mlru, node, nr_node_ids, gfp);
    if (!mlru) {
    return core::ptr::null_mut();
    }
    for_each_node(nid) {
    init_one_lru(lru, &mlru.node[nid]);
    }
    return mlru;
    }
#[no_mangle]
pub unsafe extern "C" fn memcg_init_list_lru(lru: *mut list_lru, memcg_aware: bool) {
    if (memcg_aware) {
    xa_init_flags(&lru.xa, XA_FLAGS_LOCK_IRQ);
    }
    lru.memcg_aware = memcg_aware;
    }
#[no_mangle]
unsafe extern "C" fn memcg_destroy_list_lru(lru: *mut list_lru) {
    XA_STATE(xas, &lru.xa, 0);
pub static mut mlru: *mut c_void = core::ptr::null_mut();
    if (!list_lru_memcg_aware(lru)) {
    return;
    }
    xas_lock_irq(&xas);
    xas_for_each(&xas, mlru, ULONG_MAX) {
    kfree(mlru);
    xas_store(&xas, core::ptr::null_mut());
    }
    xas_unlock_irq(&xas);
    }
#[no_mangle]
pub unsafe extern "C" fn memcg_reparent_list_lru_one(lru: *mut list_lru, nid: c_int, src: *mut list_lru_one, dst_memcg: *mut mem_cgroup) {
pub static mut dst_idx: c_int = 0;
pub static mut dst: *mut c_void = core::ptr::null_mut();
    spin_lock_irq(&src.lock);
    dst = list_lru_from_memcg_idx(lru, nid, dst_idx);
    spin_lock_nested(&dst.lock, SINGLE_DEPTH_NESTING);
    list_splice_init(&src.list, &dst.list);
    if (src.nr_items) {
    WARN_ON!(src.nr_items < 0);
    dst.nr_items += src.nr_items;
    set_shrinker_bit(dst_memcg, nid, lru_shrinker_id(lru));
    }
// Mark the list_lru_one dead
    src.nr_items = LONG_MIN;
    spin_unlock(&dst.lock);
    spin_unlock_irq(&src.lock);
    }
#[no_mangle]
pub unsafe extern "C" fn memcg_reparent_list_lrus(memcg: *mut mem_cgroup, parent: *mut mem_cgroup) {
pub static mut lru: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    mutex_lock(&list_lrus_mutex);
    list_for_each_entry(lru, &memcg_list_lrus, list) {
pub static mut mlru: *mut c_void = core::ptr::null_mut();
//
// css_is_dying() check in memcg_list_lru_alloc() avoids
// allocating a new mlru since CSS_DYING is already set for this
// memcg a rcu grace period ago.
//
    mlru = xa_load(&lru.xa, memcg.kmemcg_id);
    if (!mlru) {
    continue;
    }
//
// Reparent each per-node list and mark the child dead
// (LONG_MIN) before clearing xarray entry otherwise a
// concurrent list_lru_del() may corrupt the list if it arrives
// after xarray clear but before reparenting as
// lock_list_lru_of_memcg will acquire parent's lock while the
// item is still on child's list.
//
    for_each_node(i) {
    memcg_reparent_list_lru_one(lru, i, &mlru.node[i], parent);
    }
    xa_erase_irq(&lru.xa, memcg.kmemcg_id);
//
// Here all list_lrus corresponding to the cgroup are guaranteed
// to remain empty, we can safely free this lru, any further
// memcg_list_lru_alloc() call will simply bail out.
//
    kvfree_rcu(mlru, rcu);
    }
    mutex_unlock(&list_lrus_mutex);
    }
#[no_mangle]
pub unsafe extern "C" fn memcg_list_lru_allocated(memcg: *mut mem_cgroup, lru: *mut list_lru) -> bool {
pub static mut idx: c_int = 0;
    return idx < 0 || xa_load(&lru.xa, idx);
    }
#[no_mangle]
pub unsafe extern "C" fn __memcg_list_lru_alloc(memcg: *mut mem_cgroup, lru: *mut list_lru, gfp: gfp_t) -> c_int {
    let mut flags = 0;
    let mut mlru = core::ptr::null_mut();
    let mut pos = core::ptr::null_mut();
    let mut parent = core::ptr::null_mut();
    XA_STATE(xas, &lru.xa, 0);
    gfp &= GFP_RECLAIM_MASK;
//
// Because the list_lru can be reparented to the parent cgroup's
// list_lru, we should make sure that this cgroup and all its
// ancestors have allocated list_lru_memcg.
//
    do {
//
// Keep finding the farest parent that wasn't populated
// until found memcg itself.
//
    pos = memcg;
    parent = parent_mem_cgroup(pos);
    while (!memcg_list_lru_allocated(parent, lru)) {
    pos = parent;
    parent = parent_mem_cgroup(pos);
    }
    if (!mlru) {
    mlru = memcg_init_list_lru_one(lru, gfp);
    if (!mlru) {
    return -ENOMEM;
    }
    }
    xas_set(&xas, pos.kmemcg_id);
    do {
    xas_lock_irqsave(&xas, flags);
    if (!xas_load(&xas) && !css_is_dying(&pos.css)) {
    xas_store(&xas, mlru);
    if (!xas_error(&xas)) {
    mlru = core::ptr::null_mut();
    }
    }
    xas_unlock_irqrestore(&xas, flags);
    } while (xas_nomem(&xas, gfp));
    } while (pos != memcg && !css_is_dying(&pos.css));
    if (unlikely(mlru)) {
    kfree(mlru);
    }
    return xas_error(&xas);
    }
#[no_mangle]
pub unsafe extern "C" fn memcg_list_lru_alloc(memcg: *mut mem_cgroup, lru: *mut list_lru, gfp: gfp_t) -> c_int {
    if (!list_lru_memcg_aware(lru) || memcg_list_lru_allocated(memcg, lru)) {
    return 0;
    }
    return __memcg_list_lru_alloc(memcg, lru, gfp);
    }
#[no_mangle]
pub unsafe extern "C" fn folio_memcg_list_lru_alloc(folio: *mut folio, lru: *mut list_lru, gfp: gfp_t) -> c_int {
pub static mut memcg: *mut c_void = core::ptr::null_mut();
    let mut res = 0;
    if (!list_lru_memcg_aware(lru)) {
    return 0;
    }
// Fast path when list_lru heads already exist
    rcu_read_lock();
    memcg = folio_memcg(folio);
    res = memcg_list_lru_allocated(memcg, lru);
    rcu_read_unlock();
    if (likely(res)) {
    return 0;
    }
// Allocation may block, pin the memcg
    memcg = get_mem_cgroup_from_folio(folio);
    res = __memcg_list_lru_alloc(memcg, lru, gfp);
    mem_cgroup_put(memcg);
    return res;
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: memcg_init_list_lru
pub unsafe extern "C" fn memcg_init_list_lru_dup(lru: *mut list_lru, memcg_aware: bool) {
    }
#[no_mangle]
unsafe extern "C" fn memcg_destroy_list_lru(lru: *mut list_lru) {
    }

#[no_mangle]
pub unsafe extern "C" fn __list_lru_init(lru: *mut list_lru, memcg_aware: bool, shrinker: *mut shrinker) -> c_int {
    let mut i = 0;

    if (shrinker) {
    lru.shrinker_id = shrinker.id;
    }
    else {
    lru.shrinker_id = -1;
    }
    if (mem_cgroup_kmem_disabled()) {
    memcg_aware = false;
    }

    lru.node = kzalloc_objs(*lru.node, nr_node_ids);
    if (!lru.node) {
    return -ENOMEM;
    }
    for_each_node(i) {
    init_one_lru(lru, &lru.node[i].lru);
    }
    memcg_init_list_lru(lru, memcg_aware);
    list_lru_register(lru);
    return 0;
    }
    EXPORT_SYMBOL_GPL(__list_lru_init);
#[no_mangle]
pub unsafe extern "C" fn list_lru_destroy(lru: *mut list_lru) {
// Already destroyed or not yet initialized?
    if (!lru.node) {
    return;
    }
    list_lru_unregister(lru);
    memcg_destroy_list_lru(lru);
    kfree(lru.node);
    lru.node = core::ptr::null_mut();

    lru.shrinker_id = -1;

    }
    EXPORT_SYMBOL_GPL(list_lru_destroy);