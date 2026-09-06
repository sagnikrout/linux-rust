//! Automatically rewritten from C to Rust
//! Source: kernel/cgroup/rdma.c
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
// RDMA resource limiting controller for cgroups.
//
// Used to allow a cgroup hierarchy to stop processes from consuming
// additional RDMA resources after a certain limit is reached.
//
// Copyright (C) 2016 Parav Pandit <pandit.parav@gmail.com>
//

    enum rdmacg_limit_tokens {
    RDMACG_DEVICE_INDEX,
    RDMACG_HCA_HANDLE_VAL,
    RDMACG_HCA_HANDLE_MAX,
    RDMACG_HCA_OBJECT_VAL,
    RDMACG_HCA_OBJECT_MAX,
    NR_RDMACG_LIMIT_TOKENS,
    };
    static const match_table_t rdmacg_limit_tokens = {
    { RDMACG_DEVICE_INDEX,		"index=%u"	},
    { RDMACG_HCA_HANDLE_VAL,	"hca_handle=%d"	},
    { RDMACG_HCA_HANDLE_MAX,	"hca_handle=max"	},
    { RDMACG_HCA_OBJECT_VAL,	"hca_object=%d"	},
    { RDMACG_HCA_OBJECT_MAX,	"hca_object=max"	},
    { NR_RDMACG_LIMIT_TOKENS,	core::ptr::null_mut()			},
    };
//
// Protects list of resource pools maintained on per cgroup basis
// and rdma device list.
//
pub static mut rdmacg_mutex: usize = 0;
pub static mut rdmacg_devices: usize = 0;
    enum rdmacg_file_type {
    RDMACG_RESOURCE_TYPE_MAX,
    RDMACG_RESOURCE_TYPE_STAT,
    RDMACG_RESOURCE_TYPE_PEAK,
    };
//
// resource table definition as to be seen by the user.
// Need to add entries to it when more resources are
// added/defined at IB verb/core layer.
//
    static char const *rdmacg_resource_names[] = {
    [RDMACG_RESOURCE_HCA_HANDLE]	= "hca_handle",
    [RDMACG_RESOURCE_HCA_OBJECT]	= "hca_object",
    };
// resource tracker for each resource of rdma cgroup
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdmacg_resource {
    pub max: c_int,
    pub usage: c_int,
    pub peak: c_int,
}

//
// resource pool object which represents per cgroup, per device
// resources. There are multiple instances of this object per cgroup,
// therefore it cannot be embedded within rdma_cgroup structure. It
// is maintained as list.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdmacg_resource_pool {
    pub device: *mut rdmacg_device,
    pub resources: [rdmacg_resource; RDMACG_RESOURCE_MAX],
    pub cg_node: list_head,
    pub dev_node: list_head,
// count active user tasks of this pool
    pub usage_sum: u64,
// total number counts which are set to max
    pub num_max_cnt: c_int,
// per-resource event counters
    pub events_max: [u64; RDMACG_RESOURCE_MAX],
    pub events_alloc_fail: [u64; RDMACG_RESOURCE_MAX],
    pub events_local_max: [u64; RDMACG_RESOURCE_MAX],
    pub events_local_alloc_fail: [u64; RDMACG_RESOURCE_MAX],
}

#[no_mangle]
pub unsafe extern "C" fn css_rdmacg(css: *mut cgroup_subsys_state) -> *mut c_void {
    return container_of!(css, rdma_cgroup, css);
    }
#[no_mangle]
pub unsafe extern "C" fn parent_rdmacg(cg: *mut rdma_cgroup) -> *mut c_void {
    return css_rdmacg(cg.css.parent);
    }
#[no_mangle]
pub unsafe extern "C" fn get_current_rdmacg() -> *mut c_void {
    return css_rdmacg(task_get_css(current, rdma_cgrp_id));
    }
#[no_mangle]
pub unsafe extern "C" fn set_resource_limit(rpool: *mut rdmacg_resource_pool, index: c_int, new_max: c_int) {
    if (new_max == S32_MAX) {
    if (rpool.resources[index].max != S32_MAX) {
    rpool.num_max_cnt += 1;
    }
    } else {
    if (rpool.resources[index].max == S32_MAX) {
    rpool.num_max_cnt -= 1;
    }
    }
    rpool.resources[index].max = new_max;
    }
#[no_mangle]
unsafe extern "C" fn set_all_resource_max_limit(rpool: *mut rdmacg_resource_pool) {
    let mut i = 0;
    for (i = 0; i < RDMACG_RESOURCE_MAX; i++) {
    set_resource_limit(rpool, i, S32_MAX);
    }
    }
#[no_mangle]
unsafe extern "C" fn free_cg_rpool_locked(rpool: *mut rdmacg_resource_pool) {
    lockdep_assert_held(&rdmacg_mutex);
    list_del(&rpool.cg_node);
    list_del(&rpool.dev_node);
    kfree(rpool);
    }
#[no_mangle]
unsafe extern "C" fn rpool_has_persistent_state(rpool: *mut rdmacg_resource_pool) -> bool {
    let mut i = 0;
//
// Keep the rpool alive if any peak value is non-zero,
// so that rdma.peak persists as a historical high-
// watermark even after all resources are freed.
//
    while (i < RDMACG_RESOURCE_MAX) {
    if (rpool.resources[i].peak ||
    rpool.events_max[i] ||
    rpool.events_local_max[i] ||
    rpool.events_alloc_fail[i] ||
    rpool.events_local_alloc_fail[i]) {
    return true;
    }
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn find_cg_rpool_locked(cg: *mut rdma_cgroup, device: *mut rdmacg_device) -> *mut c_void {
pub static mut pool: *mut c_void = core::ptr::null_mut();
    lockdep_assert_held(&rdmacg_mutex);
    list_for_each_entry(pool, &cg.rpools, cg_node) {
    if (pool.device == device)
    return pool;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn get_cg_rpool_locked(cg: *mut rdma_cgroup, device: *mut rdmacg_device) -> *mut c_void {
pub static mut rpool: *mut c_void = core::ptr::null_mut();
    rpool = find_cg_rpool_locked(cg, device);
    if (rpool) {
    return rpool;
    }
    rpool = kzalloc_obj(*rpool);
    if (!rpool) {
    return ERR_PTR(-ENOMEM);
    }
    rpool.device = device;
    set_all_resource_max_limit(rpool);
    INIT_LIST_HEAD(&rpool.cg_node);
    INIT_LIST_HEAD(&rpool.dev_node);
    list_add_tail(&rpool.cg_node, &cg.rpools);
    list_add_tail(&rpool.dev_node, &device.rpools);
    return rpool;
    }
//
// uncharge_cg_locked - uncharge resource for rdma cgroup
// @cg: pointer to cg to uncharge and all parents in hierarchy
// @device: pointer to rdmacg device
// @index: index of the resource to uncharge in cg (resource pool)
//
// It also frees the resource pool which was created as part of
// charging operation when there are no resources attached to
// resource pool.
//
#[no_mangle]
pub unsafe extern "C" fn uncharge_cg_locked(cg: *mut rdma_cgroup, device: *mut rdmacg_device, index: rdmacg_resource_type) {
pub static mut rpool: *mut c_void = core::ptr::null_mut();
    rpool = find_cg_rpool_locked(cg, device);
//
// rpool cannot be null at this stage. Let kernel operate in case
// if there a bug in IB stack or rdma controller, instead of crashing
// the system.
//
    if (unlikely(!rpool)) {
    pr_warn!("Invalid device %p or rdma cgroup %p\n", device, cg);
    return;
    }
    rpool.resources[index].usage -= 1;
//
// A negative count (or overflow) is invalid,
// it indicates a bug in the rdma controller.
//
    WARN_ON_ONCE!(rpool.resources[index].usage < 0);
    rpool.usage_sum -= 1;
    if (rpool.usage_sum == 0 &&
    rpool.num_max_cnt == RDMACG_RESOURCE_MAX) {
    if (!rpool_has_persistent_state(rpool)) {
//
// No user of the rpool and all entries are set to max, so
// safe to delete this rpool.
//
    free_cg_rpool_locked(rpool);
    }
    }
    }
//
// rdmacg_event_locked - fire event when resource allocation exceeds limit
// @cg: requesting cgroup
// @over_cg: cgroup whose limit was exceeded
// @device: rdma device
// @index: resource type index
//
// Must be called under rdmacg_mutex. Updates event counters in the
// resource pools of @cg and @over_cg, propagates hierarchical max
// events from @over_cg (including itself) upward, and notifies
// userspace via cgroup_file_notify().
//
#[no_mangle]
pub unsafe extern "C" fn rdmacg_event_locked(cg: *mut rdma_cgroup, over_cg: *mut rdma_cgroup, device: *mut rdmacg_device, index: rdmacg_resource_type) {
pub static mut rpool: *mut c_void = core::ptr::null_mut();
pub static mut p: *mut c_void = core::ptr::null_mut();
    lockdep_assert_held(&rdmacg_mutex);
// Increment local alloc_fail in requesting cgroup
    rpool = find_cg_rpool_locked(cg, device);
    if (rpool) {
    rpool.events_local_alloc_fail[index]++;
    cgroup_file_notify(&cg.events_local_file);
    }
// Increment local max in the over-limit cgroup
    rpool = find_cg_rpool_locked(over_cg, device);
    if (rpool) {
    rpool.events_local_max[index]++;
    cgroup_file_notify(&over_cg.events_local_file);
    }
// Propagate hierarchical max events upward
    for (p = over_cg; parent_rdmacg(p); p = parent_rdmacg(p)) {
    rpool = get_cg_rpool_locked(p, device);
    if (!IS_ERR(rpool)) {
    rpool.events_max[index]++;
    cgroup_file_notify(&p.events_file);
    }
    }
// Propagate hierarchical alloc_fail from requesting cgroup upward
    for (p = cg; parent_rdmacg(p); p = parent_rdmacg(p)) {
    rpool = get_cg_rpool_locked(p, device);
    if (!IS_ERR(rpool)) {
    rpool.events_alloc_fail[index]++;
    cgroup_file_notify(&p.events_file);
    }
    }
    }
//
// rdmacg_uncharge_hierarchy - hierarchically uncharge rdma resource count
// @cg: pointer to cg to uncharge and all parents in hierarchy
// @device: pointer to rdmacg device
// @stop_cg: while traversing hirerchy, when meet with stop_cg cgroup
// stop uncharging
// @index: index of the resource to uncharge in cg in given resource pool
//
#[no_mangle]
pub unsafe extern "C" fn rdmacg_uncharge_hierarchy(cg: *mut rdma_cgroup, device: *mut rdmacg_device, stop_cg: *mut rdma_cgroup, index: rdmacg_resource_type) {
pub static mut p: *mut c_void = core::ptr::null_mut();
    mutex_lock(&rdmacg_mutex);
    for (p = cg; p != stop_cg; p = parent_rdmacg(p)) {
    uncharge_cg_locked(p, device, index);
    }
    mutex_unlock(&rdmacg_mutex);
    css_put(&cg.css);
    }
//
// rdmacg_uncharge - hierarchically uncharge rdma resource count
// @cg: pointer to cg to uncharge and all parents in hierarchy
// @device: pointer to rdmacg device
// @index: index of the resource to uncharge in cgroup in given resource pool
//
#[no_mangle]
pub unsafe extern "C" fn rdmacg_uncharge(cg: *mut rdma_cgroup, device: *mut rdmacg_device, index: rdmacg_resource_type) {
    if (index >= RDMACG_RESOURCE_MAX) {
    return;
    }
    rdmacg_uncharge_hierarchy(cg, device, core::ptr::null_mut(), index);
    }
    EXPORT_SYMBOL(rdmacg_uncharge);
//
// rdmacg_try_charge - hierarchically try to charge the rdma resource
// @rdmacg: pointer to rdma cgroup which will own this resource
// @device: pointer to rdmacg device
// @index: index of the resource to charge in cgroup (resource pool)
//
// This function follows charging resource in hierarchical way.
// It will fail if the charge would cause the new value to exceed the
// hierarchical limit.
// Returns 0 if the charge succeeded, otherwise -EAGAIN, -ENOMEM or -EINVAL.
// Returns pointer to rdmacg for this resource when charging is successful.
//
// Charger needs to account resources on two criteria.
// (a) per cgroup & (b) per device resource usage.
// Per cgroup resource usage ensures that tasks of cgroup doesn't cross
// the configured limits. Per device provides granular configuration
// in multi device usage. It allocates resource pool in the hierarchy
// for each parent it come across for first resource. Later on resource
// pool will be available. Therefore it will be much faster thereon
// to charge/uncharge.
//
#[no_mangle]
pub unsafe extern "C" fn rdmacg_try_charge(rdmacg: *mut *mut rdma_cgroup, device: *mut rdmacg_device, index: rdmacg_resource_type) -> c_int {
    let mut cg = core::ptr::null_mut();
    let mut p = core::ptr::null_mut();
pub static mut rpool: *mut c_void = core::ptr::null_mut();
    let mut new = 0;
pub static mut ret: c_int = 0;
    if (index >= RDMACG_RESOURCE_MAX) {
    return -EINVAL;
    }
//
// hold on to css, as cgroup can be removed but resource
// accounting happens on css.
//
    cg = get_current_rdmacg();
    mutex_lock(&rdmacg_mutex);
    for (p = cg; p; p = parent_rdmacg(p)) {
    rpool = get_cg_rpool_locked(p, device);
    if (IS_ERR(rpool)) {
    ret = PTR_ERR(rpool);
// goto;
    } else {
    new = (s64)rpool.resources[index].usage + 1;
    if (new > rpool.resources[index].max) {
    ret = -EAGAIN;
// goto;
    } else {
    rpool.resources[index].usage = new;
    rpool.usage_sum += 1;
    }
    }
    }
// Update peak only after all charges succeed
    for (p = cg; p; p = parent_rdmacg(p)) {
    rpool = find_cg_rpool_locked(p, device);
    if (rpool && rpool.resources[index].usage > rpool.resources[index].peak) {
    rpool.resources[index].peak = rpool.resources[index].usage;
    }
    }
    mutex_unlock(&rdmacg_mutex);
// rdmacg = cg;
    return 0;
// label;
    if (ret == -EAGAIN) {
    rdmacg_event_locked(cg, p, device, index);
    }
    mutex_unlock(&rdmacg_mutex);
    rdmacg_uncharge_hierarchy(cg, device, p, index);
    return ret;
    }
    EXPORT_SYMBOL(rdmacg_try_charge);
//
// rdmacg_register_device - register rdmacg device to rdma controller.
// @device: pointer to rdmacg device whose resources need to be accounted.
//
// If IB stack wish a device to participate in rdma cgroup resource
// tracking, it must invoke this API to register with rdma cgroup before
// any user space application can start using the RDMA resources.
//
#[no_mangle]
pub unsafe extern "C" fn rdmacg_register_device(device: *mut rdmacg_device) {
    INIT_LIST_HEAD(&device.dev_node);
    INIT_LIST_HEAD(&device.rpools);
    mutex_lock(&rdmacg_mutex);
    list_add_tail(&device.dev_node, &rdmacg_devices);
    mutex_unlock(&rdmacg_mutex);
    }
    EXPORT_SYMBOL(rdmacg_register_device);
//
// rdmacg_unregister_device - unregister rdmacg device from rdma controller.
// @device: pointer to rdmacg device which was previously registered with rdma
// controller using rdmacg_register_device().
//
// IB stack must invoke this after all the resources of the IB device
// are destroyed and after ensuring that no more resources will be created
// when this API is invoked.
//
#[no_mangle]
pub unsafe extern "C" fn rdmacg_unregister_device(device: *mut rdmacg_device) {
    let mut rpool = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();
//
// Synchronize with any active resource settings,
// usage query happening via configfs.
//
    mutex_lock(&rdmacg_mutex);
    list_del_init(&device.dev_node);
//
// Now that this device is off the cgroup list, its safe to free
// all the rpool resources.
//
    list_for_each_entry_safe(rpool, tmp, &device.rpools, dev_node) {
    free_cg_rpool_locked(rpool);
    }
    mutex_unlock(&rdmacg_mutex);
    }
    EXPORT_SYMBOL(rdmacg_unregister_device);
#[no_mangle]
pub unsafe extern "C" fn rdmacg_get_device_locked(name: *mut c_char, has_index: bool, index: u32) -> *mut c_void {
    let mut match = core::ptr::null_mut();
pub static mut device: *mut c_void = core::ptr::null_mut();
    lockdep_assert_held(&rdmacg_mutex);
    list_for_each_entry(device, &rdmacg_devices, dev_node) {
    if (strcmp(name, device.name)) {
    continue;
    }
    if (has_index) {
    if (device.index == index) {
    return device;
    }
    continue;
    }
    if (match) {
    return ERR_PTR(-ENOTUNIQ);
    }
    match = device;
    }
    return match ?: ERR_PTR(-ENODEV);
    }
#[no_mangle]
pub unsafe extern "C" fn rdmacg_device_name_unique_locked(device: *mut rdmacg_device) -> bool {
pub static mut other: *mut c_void = core::ptr::null_mut();
    lockdep_assert_held(&rdmacg_mutex);
    list_for_each_entry(other, &rdmacg_devices, dev_node) {
    if (other != device && !strcmp(other.name, device.name))
    return false;
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn rdmacg_print_device_key(sf: *mut seq_file, device: *mut rdmacg_device) {
    seq_puts(sf, device.name);
    if (!rdmacg_device_name_unique_locked(device)) {
    seq_printf(sf, " index=%u", device.index);
    }
    seq_putc(sf, ' ');
    }
#[no_mangle]
pub unsafe extern "C" fn rdmacg_resource_set_max(of: *mut kernfs_open_file, buf: *mut c_char, nbytes: size_t, off: loff_t) -> ssize_t {
    let mut cg = css_rdmacg(of_css(of));
pub static mut dev_name: *mut c_void = core::ptr::null_mut();
pub static mut rpool: *mut c_void = core::ptr::null_mut();
pub static mut device: *mut c_void = core::ptr::null_mut();
    let mut options = strstrip(buf);
pub static mut p: *mut c_void = core::ptr::null_mut();
pub static mut new_limits: *mut c_void = core::ptr::null_mut();
pub static mut enables: c_ulong = 0;
pub static mut dev_index: u32 = 0;
pub static mut has_index: bool = false;
pub static mut i: c_int = 0;
// extract the device name first
    dev_name = strsep(&options, " ");
    if (!dev_name) {
    ret = -EINVAL;
// goto;
    }
    new_limits = kzalloc_objs(int, RDMACG_RESOURCE_MAX);
    if (!new_limits) {
    ret = -ENOMEM;
// goto;
    }
// parse the optional device index and resource limit tokens
    while ((p = strsep(&options, " \t\n"))) {
    substring_t args[MAX_OPT_ARGS];
    let mut tok = 0;
    let mut intval = 0;
    if (!*p) {
    continue;
    }
    tok = match_token(p, rdmacg_limit_tokens, args);
    match (tok) {
    RDMACG_DEVICE_INDEX => {
    if (has_index || match_uint(&args[0], &dev_index)) {
    ret = -EINVAL;
// goto;
    }
    has_index = true;
    // break;
    }
    RDMACG_HCA_HANDLE_VAL => {
    if (match_int(&args[0], &intval) || intval < 0) {
    ret = -EINVAL;
// goto;
    }
    new_limits[RDMACG_RESOURCE_HCA_HANDLE] = intval;
    enables |= BIT(RDMACG_RESOURCE_HCA_HANDLE);
    // break;
    }
    RDMACG_HCA_HANDLE_MAX => {
    new_limits[RDMACG_RESOURCE_HCA_HANDLE] = S32_MAX;
    enables |= BIT(RDMACG_RESOURCE_HCA_HANDLE);
    // break;
    }
    RDMACG_HCA_OBJECT_VAL => {
    if (match_int(&args[0], &intval) || intval < 0) {
    ret = -EINVAL;
// goto;
    }
    new_limits[RDMACG_RESOURCE_HCA_OBJECT] = intval;
    enables |= BIT(RDMACG_RESOURCE_HCA_OBJECT);
    // break;
    }
    RDMACG_HCA_OBJECT_MAX => {
    new_limits[RDMACG_RESOURCE_HCA_OBJECT] = S32_MAX;
    enables |= BIT(RDMACG_RESOURCE_HCA_OBJECT);
    // break;
    }
    _ => {
    ret = -EINVAL;
// goto;
    }
    }
    }
// acquire lock to synchronize with hot plug devices
    mutex_lock(&rdmacg_mutex);
    device = rdmacg_get_device_locked(dev_name, has_index, dev_index);
    if (IS_ERR(device)) {
    ret = PTR_ERR(device);
// goto;
    }
    rpool = get_cg_rpool_locked(cg, device);
    if (IS_ERR(rpool)) {
    ret = PTR_ERR(rpool);
// goto;
    }
// now set the new limits of the rpool
    for_each_set_bit(i, &enables, RDMACG_RESOURCE_MAX) {
    set_resource_limit(rpool, i, new_limits[i]);
    }
    if (rpool.usage_sum == 0 &&
    rpool.num_max_cnt == RDMACG_RESOURCE_MAX) {
    if (!rpool_has_persistent_state(rpool)) {
//
// No user of the rpool and all entries are set to max, so
// safe to delete this rpool.
//
    free_cg_rpool_locked(rpool);
    }
    }
// label;
    mutex_unlock(&rdmacg_mutex);
// label;
    kfree(new_limits);
// label;
    return ret ?: nbytes;
    }
#[no_mangle]
pub unsafe extern "C" fn print_rpool_values(sf: *mut seq_file, rpool: *mut rdmacg_resource_pool) {
    enum rdmacg_file_type sf_type;
    let mut i = 0;
    let mut value = 0;
    sf_type = seq_cft(sf).private;
    while (i < RDMACG_RESOURCE_MAX) {
    seq_puts(sf, rdmacg_resource_names[i]);
    seq_putc(sf, '=');
    if (sf_type == RDMACG_RESOURCE_TYPE_MAX) {
    if (rpool) {
    value = rpool.resources[i].max;
    }
    else {
    value = S32_MAX;
    }
    } else if (sf_type == RDMACG_RESOURCE_TYPE_PEAK) {
    value = rpool ? rpool.resources[i].peak : 0;
    } else {
    if (rpool) {
    value = rpool.resources[i].usage;
    }
    else {
    value = 0;
    }
    }
    if (value == S32_MAX) {
    seq_puts(sf, RDMACG_MAX_STR);
    }
    else {
    seq_printf(sf, "%d", value);
    }
    seq_putc(sf, ' ');
    }
    }
#[no_mangle]
unsafe extern "C" fn rdmacg_resource_read(sf: *mut seq_file, v: *mut c_void) -> c_int {
pub static mut device: *mut c_void = core::ptr::null_mut();
pub static mut rpool: *mut c_void = core::ptr::null_mut();
    let mut cg = css_rdmacg(seq_css(sf));
    mutex_lock(&rdmacg_mutex);
    list_for_each_entry(device, &rdmacg_devices, dev_node) {
    rdmacg_print_device_key(sf, device);
    rpool = find_cg_rpool_locked(cg, device);
    print_rpool_values(sf, rpool);
    seq_putc(sf, '\n');
    }
    mutex_unlock(&rdmacg_mutex);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rdmacg_events_show(sf: *mut seq_file, v: *mut c_void) -> c_int {
    let mut cg = css_rdmacg(seq_css(sf));
pub static mut rpool: *mut c_void = core::ptr::null_mut();
pub static mut device: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    mutex_lock(&rdmacg_mutex);
    list_for_each_entry(device, &rdmacg_devices, dev_node) {
    rpool = find_cg_rpool_locked(cg, device);
    rdmacg_print_device_key(sf, device);
    while (i < RDMACG_RESOURCE_MAX) {
    seq_printf(sf, "%s.max=%llu %s.alloc_fail=%llu",
    rdmacg_resource_names[i],
    rpool ? rpool.events_max[i] : 0ULL,
    rdmacg_resource_names[i],
    rpool ? rpool.events_alloc_fail[i] : 0ULL);
    if (i < RDMACG_RESOURCE_MAX - 1) {
    seq_putc(sf, ' ');
    }
    }
    seq_putc(sf, '\n');
    }
    mutex_unlock(&rdmacg_mutex);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rdmacg_events_local_show(sf: *mut seq_file, v: *mut c_void) -> c_int {
    let mut cg = css_rdmacg(seq_css(sf));
pub static mut rpool: *mut c_void = core::ptr::null_mut();
pub static mut device: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    mutex_lock(&rdmacg_mutex);
    list_for_each_entry(device, &rdmacg_devices, dev_node) {
    rpool = find_cg_rpool_locked(cg, device);
    rdmacg_print_device_key(sf, device);
    while (i < RDMACG_RESOURCE_MAX) {
    seq_printf(sf, "%s.max=%llu %s.alloc_fail=%llu",
    rdmacg_resource_names[i],
    rpool ? rpool.events_local_max[i] : 0ULL,
    rdmacg_resource_names[i],
    rpool ? rpool.events_local_alloc_fail[i] : 0ULL);
    if (i < RDMACG_RESOURCE_MAX - 1) {
    seq_putc(sf, ' ');
    }
    }
    seq_putc(sf, '\n');
    }
    mutex_unlock(&rdmacg_mutex);
    return 0;
    }
pub static mut cftype: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn rdmacg_css_alloc(parent: *mut cgroup_subsys_state) -> *mut c_void {
pub static mut cg: *mut c_void = core::ptr::null_mut();
    cg = kzalloc_obj(*cg);
    if (!cg) {
    return ERR_PTR(-ENOMEM);
    }
    INIT_LIST_HEAD(&cg.rpools);
    return &cg.css;
    }
#[no_mangle]
unsafe extern "C" fn rdmacg_css_free(css: *mut cgroup_subsys_state) {
    let mut cg = css_rdmacg(css);
    let mut rpool = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();
// Clean up rpools kept alive by non-zero peak values
    mutex_lock(&rdmacg_mutex);
    list_for_each_entry_safe(rpool, tmp, &cg.rpools, cg_node) {
    free_cg_rpool_locked(rpool);
    }
    mutex_unlock(&rdmacg_mutex);
    kfree(cg);
    }
//
// rdmacg_css_offline - cgroup css_offline callback
// @css: css of interest
//
// This function is called when @css is about to go away and responsible
// for shooting down all rdmacg associated with @css. As part of that it
// marks all the resource pool entries to max value, so that when resources are
// uncharged, associated resource pool can be freed as well.
//
#[no_mangle]
unsafe extern "C" fn rdmacg_css_offline(css: *mut cgroup_subsys_state) {
    let mut cg = css_rdmacg(css);
pub static mut rpool: *mut c_void = core::ptr::null_mut();
    mutex_lock(&rdmacg_mutex);
    list_for_each_entry(rpool, &cg.rpools, cg_node) {
    set_all_resource_max_limit(rpool);
    }
    mutex_unlock(&rdmacg_mutex);
    }
pub static mut cgroup_subsys: usize = 0;