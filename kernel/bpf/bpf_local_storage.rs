//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/bpf_local_storage.c
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
// Copyright (c) 2019 Facebook

#[no_mangle]
pub unsafe extern "C" fn select_bucket(smap: *mut bpf_local_storage_map, local_storage: *mut bpf_local_storage) -> *mut c_void {
    return &smap.buckets[hash_ptr(local_storage, smap.bucket_log)];
    }
#[no_mangle]
unsafe extern "C" fn mem_charge(smap: *mut bpf_local_storage_map, owner: *mut c_void, size: u32) -> c_int {
    let mut map = &smap.map;
    if (!map.ops.map_local_storage_charge) {
    return 0;
    }
    return map.ops.map_local_storage_charge(smap, owner, size);
    }
#[no_mangle]
pub unsafe extern "C" fn mem_uncharge(smap: *mut bpf_local_storage_map, owner: *mut c_void, size: u32) {
    let mut map = &smap.map;
    if (map.ops.map_local_storage_uncharge) {
    map.ops.map_local_storage_uncharge(smap, owner, size);
    }
    }
    static struct bpf_local_storage  **
    owner_storage(bpf_local_storage_map *smap, void *owner)
    {
    let mut map = &smap.map;
    return map.ops.map_owner_storage_ptr(owner);
    }
#[no_mangle]
unsafe extern "C" fn selem_linked_to_storage_lockless(selem: *const bpf_local_storage_elem) -> bool {
    return !hlist_unhashed_lockless(&selem.snode);
    }
#[no_mangle]
unsafe extern "C" fn selem_linked_to_storage(selem: *const bpf_local_storage_elem) -> bool {
    return !hlist_unhashed(&selem.snode);
    }
#[no_mangle]
unsafe extern "C" fn selem_linked_to_map(selem: *const bpf_local_storage_elem) -> bool {
    return !hlist_unhashed(&selem.map_node);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_selem_alloc(smap: *mut bpf_local_storage_map, owner: *mut c_void, value: *mut c_void, swap_uptrs: bool) -> *mut c_void {
pub static mut selem: *mut c_void = core::ptr::null_mut();
    if (mem_charge(smap, owner, smap.elem_size)) {
    return core::ptr::null_mut();
    }
    selem = bpf_map_kmalloc_nolock(&smap.map, smap.elem_size,
    __GFP_ZERO, NUMA_NO_NODE);
    if (selem) {
    RCU_INIT_POINTER(SDATA(selem).smap, smap);
    atomic_set(&selem.state, 0);
    if (value) {
// No need to call check_and_init_map_value as memory is zero init
    copy_map_value(&smap.map, SDATA(selem).data, value);
    if (swap_uptrs) {
    bpf_obj_swap_uptrs(smap.map.record, SDATA(selem).data, value);
    }
    }
    return selem;
    }
    mem_uncharge(smap, owner, smap.elem_size);
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn bpf_local_storage_free_trace_rcu(rcu: *mut rcu_head) {
pub static mut local_storage: *mut c_void = core::ptr::null_mut();
//
// RCU Tasks Trace grace period implies RCU grace period, do
// kfree() directly.
//
    local_storage = container_of!(rcu, bpf_local_storage, rcu);
    kfree(local_storage);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_local_storage_free(local_storage: *mut bpf_local_storage, reuse_now: bool) {
    if (!local_storage) {
    return;
    }
    if (reuse_now) {
    kfree_rcu(local_storage, rcu);
    return;
    }
    call_rcu_tasks_trace(&local_storage.rcu,
    bpf_local_storage_free_trace_rcu);
    }
#[no_mangle]
unsafe extern "C" fn bpf_selem_free_trace_rcu(rcu: *mut rcu_head) {
pub static mut selem: *mut c_void = core::ptr::null_mut();
pub static mut smap: *mut c_void = core::ptr::null_mut();
    selem = container_of!(rcu, bpf_local_storage_elem, rcu);
// The bpf_local_storage_map_free will wait for rcu_barrier
    smap = rcu_dereference_check(SDATA(selem).smap, 1);
    if (smap) {
    bpf_obj_free_fields(smap.map.record, SDATA(selem).data);
    }
//
// RCU Tasks Trace grace period implies RCU grace period, do
// kfree() directly.
//
    kfree(selem);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_selem_free(selem: *mut bpf_local_storage_elem, reuse_now: bool) {
pub static mut smap: *mut c_void = core::ptr::null_mut();
    smap = rcu_dereference_check(SDATA(selem).smap, 1);
    if (reuse_now) {
    if (smap) {
    bpf_obj_free_fields(smap.map.record, SDATA(selem).data);
    }
    kfree_rcu(selem, rcu);
    return;
    }
    call_rcu_tasks_trace(&selem.rcu, bpf_selem_free_trace_rcu);
    }
#[no_mangle]
unsafe extern "C" fn bpf_selem_free_list(list: *mut hlist_head, reuse_now: bool) {
pub static mut selem: *mut c_void = core::ptr::null_mut();
pub static mut n: *mut c_void = core::ptr::null_mut();
// The "_safe" iteration is needed.
// The loop is not removing the selem from the list
// but bpf_selem_free will use the selem->rcu_head
// which is union-ized with the selem->free_node.
//
    hlist_for_each_entry_safe(selem, n, list, free_node)
    bpf_selem_free(selem, reuse_now);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_selem_unlink_storage_nolock_misc(selem: *mut bpf_local_storage_elem, smap: *mut bpf_local_storage_map, local_storage: *mut bpf_local_storage, free_local_storage: bool, pin_owner: bool) {
    let mut owner = local_storage.owner;
pub static mut uncharge: u32 = 0;
    if (rcu_access_pointer(local_storage.cache[smap.cache_idx]) ==
    SDATA(selem)) {
    RCU_INIT_POINTER(local_storage.cache[smap.cache_idx], core::ptr::null_mut());
    }
    if (pin_owner && !refcount_inc_not_zero(&local_storage.owner_refcnt)) {
    return;
    }
    uncharge += free_local_storage ? sizeof!(*local_storage) : 0;
    mem_uncharge(smap, local_storage.owner, uncharge);
    local_storage.mem_charge -= uncharge;
    if (free_local_storage) {
    local_storage.owner = core::ptr::null_mut();
// After this RCU_INIT, owner may be freed and cannot be used
    RCU_INIT_POINTER(*owner_storage(smap, owner), core::ptr::null_mut());
    }
    if (pin_owner) {
    refcount_dec(&local_storage.owner_refcnt);
    }
    }
// local_storage->lock must be held and selem->local_storage == local_storage.
// The caller must ensure selem->smap is still valid to be
// dereferenced for its smap->elem_size and smap->cache_idx.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_selem_unlink_storage_nolock(local_storage: *mut bpf_local_storage, selem: *mut bpf_local_storage_elem, free_selem_list: *mut hlist_head) -> bool {
pub static mut smap: *mut c_void = core::ptr::null_mut();
    let mut free_local_storage = 0;
    smap = rcu_dereference_check(SDATA(selem).smap, bpf_rcu_lock_held());
    free_local_storage = hlist_is_singular_node(&selem.snode,
    &local_storage.list);
    bpf_selem_unlink_storage_nolock_misc(selem, smap, local_storage,
    free_local_storage, false);
    hlist_del_init_rcu(&selem.snode);
    hlist_add_head(&selem.free_node, free_selem_list);
    return free_local_storage;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_selem_link_storage_nolock(local_storage: *mut bpf_local_storage, selem: *mut bpf_local_storage_elem) {
pub static mut smap: *mut c_void = core::ptr::null_mut();
    smap = rcu_dereference_check(SDATA(selem).smap, bpf_rcu_lock_held());
    local_storage.mem_charge += smap.elem_size;
    RCU_INIT_POINTER(selem.local_storage, local_storage);
    hlist_add_head_rcu(&selem.snode, &local_storage.list);
    }
#[no_mangle]
unsafe extern "C" fn bpf_selem_unlink_map(selem: *mut bpf_local_storage_elem) -> c_int {
pub static mut local_storage: *mut c_void = core::ptr::null_mut();
pub static mut smap: *mut c_void = core::ptr::null_mut();
pub static mut b: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    let mut err = 0;
    local_storage = rcu_dereference_check(selem.local_storage,
    bpf_rcu_lock_held());
    smap = rcu_dereference_check(SDATA(selem).smap, bpf_rcu_lock_held());
    b = select_bucket(smap, local_storage);
    err = raw_res_spin_lock_irqsave(&b.lock, flags);
    if (err) {
    return err;
    }
    hlist_del_init_rcu(&selem.map_node);
    raw_res_spin_unlock_irqrestore(&b.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bpf_selem_unlink_map_nolock(selem: *mut bpf_local_storage_elem) {
    hlist_del_init_rcu(&selem.map_node);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_selem_link_map(smap: *mut bpf_local_storage_map, local_storage: *mut bpf_local_storage, selem: *mut bpf_local_storage_elem) -> c_int {
pub static mut b: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    let mut err = 0;
    b = select_bucket(smap, local_storage);
    err = raw_res_spin_lock_irqsave(&b.lock, flags);
    if (err) {
    return err;
    }
    hlist_add_head_rcu(&selem.map_node, &b.list);
    raw_res_spin_unlock_irqrestore(&b.lock, flags);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_selem_link_map_nolock(b: *mut bpf_local_storage_map_bucket, selem: *mut bpf_local_storage_elem) {
    hlist_add_head_rcu(&selem.map_node, &b.list);
    }
//
// Unlink an selem from map and local storage with lock held.
// This is the common path used by local storages to delete an selem.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_selem_unlink(selem: *mut bpf_local_storage_elem) -> c_int {
pub static mut local_storage: *mut c_void = core::ptr::null_mut();
pub static mut free_local_storage: bool = false;
    HLIST_HEAD(selem_free_list);
    let mut flags = 0;
    let mut err = 0;
    if (in_nmi()) {
    return -EOPNOTSUPP;
    }
    if (unlikely(!selem_linked_to_storage_lockless(selem))) {
// selem has already been unlinked from sk
    return 0;
    }
    local_storage = rcu_dereference_check(selem.local_storage,
    bpf_rcu_lock_held());
    err = raw_res_spin_lock_irqsave(&local_storage.lock, flags);
    if (err) {
    return err;
    }
    if (likely(selem_linked_to_storage(selem))) {
// Always unlink from map before unlinking from local_storage
// because selem will be freed after successfully unlinked from
// the local_storage.
//
    err = bpf_selem_unlink_map(selem);
    if (err) {
// goto;
    }
    free_local_storage = bpf_selem_unlink_storage_nolock(
    local_storage, selem, &selem_free_list);
    }
// label;
    raw_res_spin_unlock_irqrestore(&local_storage.lock, flags);
    bpf_selem_free_list(&selem_free_list, false);
    if (free_local_storage) {
    bpf_local_storage_free(local_storage, false);
    }
    return err;
    }
//
// Unlink an selem from map and local storage with lockless fallback if callers
// are racing or rqspinlock returns error. It should only be called by
// bpf_local_storage_destroy() or bpf_local_storage_map_free().
//
#[no_mangle]
pub unsafe extern "C" fn bpf_selem_unlink_nofail(selem: *mut bpf_local_storage_elem, b: *mut bpf_local_storage_map_bucket) {
pub static mut in_map_free: bool = false;
pub static mut local_storage: *mut c_void = core::ptr::null_mut();
pub static mut smap: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    int err, unlink = 0;
    local_storage = rcu_dereference_check(selem.local_storage, bpf_rcu_lock_held());
    smap = rcu_dereference_check(SDATA(selem).smap, bpf_rcu_lock_held());
    if (smap) {
    b = b ? : select_bucket(smap, local_storage);
    err = raw_res_spin_lock_irqsave(&b.lock, flags);
    if (!err) {
//
// Call bpf_obj_free_fields() under b->lock to make sure it is done
// exactly once for an selem. Safe to free special fields immediately
// as no BPF program should be referencing the selem.
//
    if (likely(selem_linked_to_map(selem))) {
    hlist_del_init_rcu(&selem.map_node);
    bpf_obj_free_fields(smap.map.record, SDATA(selem).data);
    unlink += 1;
    }
    raw_res_spin_unlock_irqrestore(&b.lock, flags);
    }
//
// Highly unlikely scenario: resource leak
//
// When map_free(selem1), destroy(selem1) and destroy(selem2) are racing
// and both selem belong to the same bucket, if destroy(selem2) acquired
// b->lock and block for too long, neither map_free(selem1) and
// destroy(selem1) will be able to free the special field associated
// with selem1 as raw_res_spin_lock_irqsave() returns -ETIMEDOUT.
//
    WARN_ON_ONCE!(err && in_map_free);
    if (!err || in_map_free) {
    RCU_INIT_POINTER(SDATA(selem).smap, core::ptr::null_mut());
    }
    }
    if (local_storage) {
    err = raw_res_spin_lock_irqsave(&local_storage.lock, flags);
    if (!err) {
    if (likely(selem_linked_to_storage(selem))) {
    free_storage = hlist_is_singular_node(&selem.snode,
    &local_storage.list);
//
// Okay to skip clearing owner_storage and storage->owner in
// destroy() since the owner is going away. No user or bpf
// programs should be able to reference it.
//
    if (smap && in_map_free) {
    bpf_selem_unlink_storage_nolock_misc(
    selem, smap, local_storage,
    free_storage, true);
    }
    hlist_del_init_rcu(&selem.snode);
    unlink += 1;
    }
    raw_res_spin_unlock_irqrestore(&local_storage.lock, flags);
    }
//
// Highly unlikely scenario: memory leak
//
// When destroy() fails to acqurire local_storage->lock and initializes
// selem->local_storage to NULL before any racing map_free() sees the same
// selem, no one will free the local storage.
//
    WARN_ON_ONCE!(err && !in_map_free);
    if (!err || !in_map_free) {
    RCU_INIT_POINTER(selem.local_storage, core::ptr::null_mut());
    }
    }
    if (unlink != 2) {
    atomic_or(in_map_free ? SELEM_MAP_UNLINKED : SELEM_STORAGE_UNLINKED, &selem.state);
    }
//
// Normally, an selem can be unlinked under local_storage->lock and b->lock, and
// then freed after an RCU grace period. However, if destroy() and map_free() are
// racing or rqspinlock returns errors in unlikely situations (unlink != 2), free
// the selem only after both map_free() and destroy() see the selem.
//
    if (unlink == 2 ||
    atomic_cmpxchg(&selem.state, SELEM_UNLINKED, SELEM_TOFREE) == SELEM_UNLINKED) {
    bpf_selem_free(selem, true);
    }
    if (free_storage) {
    bpf_local_storage_free(local_storage, true);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __bpf_local_storage_insert_cache(local_storage: *mut bpf_local_storage, smap: *mut bpf_local_storage_map, selem: *mut bpf_local_storage_elem) {
    let mut flags = 0;
    let mut err = 0;
// spinlock is needed to avoid racing with the
// parallel delete.  Otherwise, publishing an already
// deleted sdata to the cache will become a use-after-free
// problem in the next bpf_local_storage_lookup().
//
    err = raw_res_spin_lock_irqsave(&local_storage.lock, flags);
    if (err) {
    return;
    }
    if (selem_linked_to_storage(selem)) {
    rcu_assign_pointer(local_storage.cache[smap.cache_idx], SDATA(selem));
    }
    raw_res_spin_unlock_irqrestore(&local_storage.lock, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn check_flags(old_sdata: *mut bpf_local_storage_data, map_flags: u64) -> c_int {
    if (old_sdata && (map_flags & ~BPF_F_LOCK) == BPF_NOEXIST) {
// elem already exists
    return -EEXIST;
    }
    if (!old_sdata && (map_flags & ~BPF_F_LOCK) == BPF_EXIST) {
// elem doesn't exist, cannot update it
    return -ENOENT;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_local_storage_alloc(owner: *mut c_void, smap: *mut bpf_local_storage_map, first_selem: *mut bpf_local_storage_elem) -> c_int {
    let mut prev_storage = core::ptr::null_mut();
    let mut storage = core::ptr::null_mut();
pub static mut owner_storage_ptr: *mut c_void = core::ptr::null_mut();
pub static mut b: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    let mut err = 0;
    err = mem_charge(smap, owner, sizeof!(*storage));
    if (err) {
    return err;
    }
    storage = bpf_map_kmalloc_nolock(&smap.map, sizeof!(*storage),
    __GFP_ZERO, NUMA_NO_NODE);
    if (!storage) {
    err = -ENOMEM;
// goto;
    }
    INIT_HLIST_HEAD(&storage.list);
    raw_res_spin_lock_init(&storage.lock);
    storage.owner = owner;
    storage.mem_charge = sizeof!(*storage);
    refcount_set(&storage.owner_refcnt, 1);
    bpf_selem_link_storage_nolock(storage, first_selem);
    b = select_bucket(smap, storage);
    err = raw_res_spin_lock_irqsave(&b.lock, flags);
    if (err) {
// goto;
    }
    bpf_selem_link_map_nolock(b, first_selem);
    owner_storage_ptr =
    owner_storage(smap, owner);
// Publish storage to the owner.
// Instead of using any lock of the kernel object (i.e. owner),
// cmpxchg will work with any kernel object regardless what
// the running context is, bh, irq...etc.
//
// From now on, the owner->storage pointer (e.g. sk->sk_bpf_storage)
// is protected by the storage->lock.  Hence, when freeing
// the owner->storage, the storage->lock must be held before
// setting owner->storage ptr to NULL.
//
    prev_storage = cmpxchg(owner_storage_ptr, core::ptr::null_mut(), storage);
    if (unlikely(prev_storage)) {
    bpf_selem_unlink_map_nolock(first_selem);
    raw_res_spin_unlock_irqrestore(&b.lock, flags);
    err = -EAGAIN;
// goto;
    }
    raw_res_spin_unlock_irqrestore(&b.lock, flags);
    return 0;
// label;
    bpf_local_storage_free(storage, true);
    mem_uncharge(smap, owner, sizeof!(*storage));
    return err;
    }
// sk cannot be going away because it is linking new elem
// to sk->sk_bpf_storage. (i.e. sk->sk_refcnt cannot be 0).
// Otherwise, it will become a leak (and other memory issues
// during map destruction).
//
#[no_mangle]
pub unsafe extern "C" fn bpf_local_storage_update(owner: *mut c_void, smap: *mut bpf_local_storage_map, value: *mut c_void, map_flags: u64, swap_uptrs: bool) -> *mut c_void {
    let mut old_sdata = core::ptr::null_mut();
    struct bpf_local_storage_elem *alloc_selem, *selem = core::ptr::null_mut();
pub static mut local_storage: *mut c_void = core::ptr::null_mut();
pub static mut b: *mut c_void = core::ptr::null_mut();
    HLIST_HEAD(old_selem_free_list);
    unsigned long flags, b_flags;
    let mut err = 0;
// BPF_EXIST and BPF_NOEXIST cannot be both set
    if (unlikely((map_flags & ~BPF_F_LOCK) > BPF_EXIST) ||
// BPF_F_LOCK can only be used in a value with spin_lock
    unlikely((map_flags & BPF_F_LOCK) &&
    !btf_record_has_field(smap.map.record, BPF_SPIN_LOCK))) {
    return ERR_PTR(-EINVAL);
    }
    local_storage = rcu_dereference_check(*owner_storage(smap, owner),
    bpf_rcu_lock_held());
    if (!local_storage || hlist_empty(&local_storage.list)) {
// Very first elem for the owner
    err = check_flags(core::ptr::null_mut(), map_flags);
    if (err) {
    return ERR_PTR(err);
    }
    selem = bpf_selem_alloc(smap, owner, value, swap_uptrs);
    if (!selem) {
    return ERR_PTR(-ENOMEM);
    }
    err = bpf_local_storage_alloc(owner, smap, selem);
    if (err) {
    bpf_selem_free(selem, true);
    mem_uncharge(smap, owner, smap.elem_size);
    return ERR_PTR(err);
    }
    return SDATA(selem);
    }
    if ((map_flags & BPF_F_LOCK) && !(map_flags & BPF_NOEXIST)) {
// Hoping to find an old_sdata to do inline update
// such that it can avoid taking the local_storage->lock
// and changing the lists.
//
    old_sdata =
    bpf_local_storage_lookup(local_storage, smap, false);
    err = check_flags(old_sdata, map_flags);
    if (err) {
    return ERR_PTR(err);
    }
    if (old_sdata && selem_linked_to_storage_lockless(SELEM(old_sdata))) {
    copy_map_value_locked(&smap.map, old_sdata.data,
    value, false);
    return old_sdata;
    }
    }
// A lookup has just been done before and concluded a new selem is
// needed. The chance of an unnecessary alloc is unlikely.
//
    alloc_selem = selem = bpf_selem_alloc(smap, owner, value, swap_uptrs);
    if (!alloc_selem) {
    return ERR_PTR(-ENOMEM);
    }
    err = raw_res_spin_lock_irqsave(&local_storage.lock, flags);
    if (err) {
// goto;
    }
// Recheck local_storage->list under local_storage->lock
    if (unlikely(hlist_empty(&local_storage.list))) {
// A parallel del is happening and local_storage is going
// away.  It has just been checked before, so very
// unlikely.  Return instead of retry to keep things
// simple.
//
    err = -EAGAIN;
// goto;
    }
    old_sdata = bpf_local_storage_lookup(local_storage, smap, false);
    err = check_flags(old_sdata, map_flags);
    if (err) {
// goto;
    }
    if (old_sdata && (map_flags & BPF_F_LOCK)) {
    copy_map_value_locked(&smap.map, old_sdata.data, value,
    false);
    selem = SELEM(old_sdata);
// goto;
    }
    b = select_bucket(smap, local_storage);
    err = raw_res_spin_lock_irqsave(&b.lock, b_flags);
    if (err) {
// goto;
    }
    alloc_selem = core::ptr::null_mut();
// First, link the new selem to the map
    bpf_selem_link_map_nolock(b, selem);
// Second, link (and publish) the new selem to local_storage
    bpf_selem_link_storage_nolock(local_storage, selem);
// Third, remove old selem, SELEM(old_sdata)
    if (old_sdata) {
    bpf_selem_unlink_map_nolock(SELEM(old_sdata));
    bpf_selem_unlink_storage_nolock(local_storage, SELEM(old_sdata),
    &old_selem_free_list);
    }
    raw_res_spin_unlock_irqrestore(&b.lock, b_flags);
// label;
    raw_res_spin_unlock_irqrestore(&local_storage.lock, flags);
// label;
    bpf_selem_free_list(&old_selem_free_list, false);
    if (alloc_selem) {
    mem_uncharge(smap, owner, smap.elem_size);
    bpf_selem_free(alloc_selem, true);
    }
    return err ? ERR_PTR(err) : SDATA(selem);
    }
#[no_mangle]
unsafe extern "C" fn bpf_local_storage_cache_idx_get(cache: *mut bpf_local_storage_cache) -> u16 {
pub static mut min_usage: u64 = 0;
    u16 i, res = 0;
    spin_lock(&cache.idx_lock);
    while (i < BPF_LOCAL_STORAGE_CACHE_SIZE) {
    if (cache.idx_usage_counts[i] < min_usage) {
    min_usage = cache.idx_usage_counts[i];
    res = i;
// Found a free cache_idx
    if (!min_usage) {
    break;
    }
    }
    }
    cache.idx_usage_counts[res]++;
    spin_unlock(&cache.idx_lock);
    return res;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_local_storage_cache_idx_free(cache: *mut bpf_local_storage_cache, idx: u16) {
    spin_lock(&cache.idx_lock);
    cache.idx_usage_counts[idx]--;
    spin_unlock(&cache.idx_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_local_storage_map_alloc_check(attr: *mut union bpf_attr) -> c_int {
    if (attr.map_flags & ~BPF_LOCAL_STORAGE_CREATE_FLAG_MASK ||
    !(attr.map_flags & BPF_F_NO_PREALLOC) ||
    attr.max_entries ||
    attr.key_size != sizeof!(int) || !attr.value_size ||
// Enforce BTF for userspace sk dumping
    !attr.btf_key_type_id || !attr.btf_value_type_id) {
    return -EINVAL;
    }
    if (attr.value_size > BPF_LOCAL_STORAGE_MAX_VALUE_SIZE) {
    return -E2BIG;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_local_storage_map_check_btf(map: *mut bpf_map, btf: *mut btf, key_type: *mut btf_type, value_type: *mut btf_type) -> c_int {
    if (!btf_type_is_i32(key_type)) {
    return -EINVAL;
    }
    return 0;
    }
//
// Destroy local storage when the owner is going away. Caller must uncharge memory
// if memory charging is used.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_local_storage_destroy(local_storage: *mut bpf_local_storage) -> u32 {
pub static mut selem: *mut c_void = core::ptr::null_mut();
// Neither the bpf_prog nor the bpf_map's syscall
// could be modifying the local_storage->list now.
// Thus, no elem can be added to or deleted from the
// local_storage->list by the bpf_prog or by the bpf_map's syscall.
//
// It is racing with bpf_local_storage_map_free() alone
// when unlinking elem from the local_storage->list and
// the map's bucket->list.
//
    hlist_for_each_entry_rcu(selem, &local_storage.list, snode)
    bpf_selem_unlink_nofail(selem, core::ptr::null_mut());
    if (!refcount_dec_and_test(&local_storage.owner_refcnt)) {
    while (refcount_read(&local_storage.owner_refcnt)) {
    cpu_relax();
    }
//
// Paired with refcount_dec() in bpf_selem_unlink_nofail()
// to make sure destroy() sees the correct local_storage->mem_charge.
//
    smp_mb();
    }
    return local_storage.mem_charge;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_local_storage_map_mem_usage(map: *const bpf_map) -> u64 {
    let mut smap = map;
pub static mut usage: u64 = 0;
// The dynamically callocated selems are not counted currently.
    usage += sizeof!(*smap.buckets) * (1ULL << smap.bucket_log);
    return usage;
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_local_storage_map_alloc(attr: *mut union bpf_attr, cache: *mut bpf_local_storage_cache) -> *mut c_void {
pub static mut smap: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    let mut nbuckets = 0;
    let mut err = 0;
    smap = bpf_map_area_alloc(sizeof!(*smap), NUMA_NO_NODE);
    if (!smap) {
    return ERR_PTR(-ENOMEM);
    }
    bpf_map_init_from_attr(&smap.map, attr);
    nbuckets = roundup_pow_of_two(num_possible_cpus());
// Use at least 2 buckets, select_bucket() is undefined behavior with 1 bucket
    nbuckets = max_t(u32, 2, nbuckets);
    smap.bucket_log = ilog2(nbuckets);
    smap.buckets = bpf_map_kvcalloc(&smap.map, nbuckets,
    sizeof!(*smap.buckets), GFP_USER | __GFP_NOWARN);
    if (!smap.buckets) {
    err = -ENOMEM;
// goto;
    }
    while (i < nbuckets) {
    INIT_HLIST_HEAD(&smap.buckets[i].list);
    raw_res_spin_lock_init(&smap.buckets[i].lock);
    }
    smap.elem_size = offsetof(bpf_local_storage_elem,
    sdata.data[attr.value_size]);
    smap.cache_idx = bpf_local_storage_cache_idx_get(cache);
    return &smap.map;
// label;
    kvfree(smap.buckets);
    bpf_map_area_free(smap);
    return ERR_PTR(err);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_local_storage_map_free(map: *mut bpf_map, cache: *mut bpf_local_storage_cache) {
pub static mut b: *mut c_void = core::ptr::null_mut();
pub static mut selem: *mut c_void = core::ptr::null_mut();
pub static mut smap: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    smap = map;
    bpf_local_storage_cache_idx_free(cache, smap.cache_idx);
// Note that this map might be concurrently cloned from
// bpf_sk_storage_clone. Wait for any existing bpf_sk_storage_clone
// RCU read section to finish before proceeding. New RCU
// read sections should be prevented via bpf_map_inc_not_zero.
//
    synchronize_rcu();
// bpf prog and the userspace can no longer access this map
// now.  No new selem (of this map) can be added
// to the owner->storage or to the map bucket's list.
//
// The elem of this map can be cleaned up here
// or when the storage is freed e.g.
// by bpf_sk_storage_free() during __sk_destruct().
//
    while (i < (1U << smap.bucket_log)) {
    b = &smap.buckets[i];
    rcu_read_lock();
// No one is adding to b->list now
// label;
    hlist_for_each_entry_rcu(selem, &b.list, map_node) {
    bpf_selem_unlink_nofail(selem, b);
    if (need_resched()) {
    cond_resched_rcu();
// goto;
    }
    }
    rcu_read_unlock();
    }
// While freeing the storage we may still need to access the map.
//
// e.g. when bpf_sk_storage_free() has unlinked selem from the map
// which then made the above while((selem = ...)) loop
// exit immediately.
//
// However, while freeing the storage one still needs to access the
// smap->elem_size to do the uncharging in
// bpf_selem_unlink_storage_nolock().
//
// Hence, wait another rcu grace period for the storage to be freed.
//
    synchronize_rcu();
// smap remains in use regardless of kmalloc_nolock, so wait unconditionally.
    rcu_barrier_tasks_trace();
    rcu_barrier();
    kvfree(smap.buckets);
    bpf_map_area_free(smap);
    }