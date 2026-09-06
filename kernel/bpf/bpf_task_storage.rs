//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/bpf_task_storage.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2020 Facebook
// Copyright 2020 Google LLC.
//

    DEFINE_BPF_STORAGE_CACHE(task_cache);
    static struct bpf_local_storage __rcu **task_storage_ptr(void *owner)
    {
    struct task_struct *task = owner;
    return &task.bpf_storage;
    }
    static struct bpf_local_storage_data *
    task_storage_lookup(struct task_struct *task, struct bpf_map *map,
    bool cacheit_lockit)
    {
    struct bpf_local_storage *task_storage;
    struct bpf_local_storage_map *smap;
    task_storage =
    rcu_dereference_check(task.bpf_storage, bpf_rcu_lock_held());
    if (!task_storage)
    return core::ptr::null_mut();
    smap = (struct bpf_local_storage_map *)map;
    return bpf_local_storage_lookup(task_storage, smap, cacheit_lockit);
    }
#[no_mangle]
pub unsafe extern "C" fn bpf_task_storage_free(task: *mut task_struct) {
    void bpf_task_storage_free(struct task_struct *task)
    {
    struct bpf_local_storage *local_storage;
    rcu_read_lock();
    local_storage = rcu_dereference(task.bpf_storage);
    if (!local_storage)
    goto out;
    bpf_local_storage_destroy(local_storage);
    out:
    rcu_read_unlock();
    }
    static void *bpf_pid_task_storage_lookup_elem(struct bpf_map *map, void *key)
    {
    struct bpf_local_storage_data *sdata;
    struct task_struct *task;
    unsigned int f_flags;
    struct pid *pid;
    int fd, err;
    fd = *(int *)key;
    pid = pidfd_get_pid(fd, &f_flags);
    if (IS_ERR(pid))
    return ERR_CAST(pid);
// We should be in an RCU read side critical section, it should be safe
// to call pid_task.
//
    WARN_ON_ONCE(!rcu_read_lock_held());
    task = pid_task(pid, PIDTYPE_PID);
    if (!task) {
    err = -ENOENT;
    goto out;
    }
    sdata = task_storage_lookup(task, map, true);
    put_pid(pid);
    return sdata ? sdata.data : core::ptr::null_mut();
    out:
    put_pid(pid);
    return ERR_PTR(err);
    }
    static long bpf_pid_task_storage_update_elem(struct bpf_map *map, void *key,
    void *value, u64 map_flags)
    {
    struct bpf_local_storage_data *sdata;
    struct task_struct *task;
    unsigned int f_flags;
    struct pid *pid;
    int fd, err;
    if ((map_flags & BPF_F_LOCK) && btf_record_has_field(map.record, BPF_UPTR))
    return -EOPNOTSUPP;
    fd = *(int *)key;
    pid = pidfd_get_pid(fd, &f_flags);
    if (IS_ERR(pid))
    return PTR_ERR(pid);
// We should be in an RCU read side critical section, it should be safe
// to call pid_task.
//
    WARN_ON_ONCE(!rcu_read_lock_held());
    task = pid_task(pid, PIDTYPE_PID);
    if (!task) {
    err = -ENOENT;
    goto out;
    }
    sdata = bpf_local_storage_update(
    task, (struct bpf_local_storage_map *)map, value, map_flags,
    true);
    err = PTR_ERR_OR_ZERO(sdata);
    out:
    put_pid(pid);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn task_storage_delete(task: *mut task_struct, map: *mut bpf_map) -> c_int {
    static int task_storage_delete(struct task_struct *task, struct bpf_map *map)
    {
    struct bpf_local_storage_data *sdata;
    sdata = task_storage_lookup(task, map, false);
    if (!sdata)
    return -ENOENT;
    return bpf_selem_unlink(SELEM(sdata));
    }
#[no_mangle]
unsafe extern "C" fn bpf_pid_task_storage_delete_elem(map: *mut bpf_map, key: *mut c_void) -> c_long {
    static long bpf_pid_task_storage_delete_elem(struct bpf_map *map, void *key)
    {
    struct task_struct *task;
    unsigned int f_flags;
    struct pid *pid;
    int fd, err;
    fd = *(int *)key;
    pid = pidfd_get_pid(fd, &f_flags);
    if (IS_ERR(pid))
    return PTR_ERR(pid);
// We should be in an RCU read side critical section, it should be safe
// to call pid_task.
//
    WARN_ON_ONCE(!rcu_read_lock_held());
    task = pid_task(pid, PIDTYPE_PID);
    if (!task) {
    err = -ENOENT;
    goto out;
    }
    err = task_storage_delete(task, map);
    out:
    put_pid(pid);
    return err;
    }
    BPF_CALL_4(bpf_task_storage_get, struct bpf_map *, map, struct task_struct *,
    task, void *, value, u64, flags)
    {
    struct bpf_local_storage_data *sdata;
    WARN_ON_ONCE(!bpf_rcu_lock_held());
    if (flags & ~BPF_LOCAL_STORAGE_GET_F_CREATE || !task)
    return (unsigned long)core::ptr::null_mut();
    sdata = task_storage_lookup(task, map, true);
    if (sdata)
    return (unsigned long)sdata.data;
// only allocate new storage, when the task is refcounted
    if (refcount_read(&task.usage) &&
    (flags & BPF_LOCAL_STORAGE_GET_F_CREATE)) {
    sdata = bpf_local_storage_update(
    task, (struct bpf_local_storage_map *)map, value,
    BPF_NOEXIST, false);
    return IS_ERR(sdata) ? (unsigned long)core::ptr::null_mut() : (unsigned long)sdata.data;
    }
    return (unsigned long)core::ptr::null_mut();
    }
    BPF_CALL_2(bpf_task_storage_delete, struct bpf_map *, map, struct task_struct *,
    task)
    {
    WARN_ON_ONCE(!bpf_rcu_lock_held());
    if (!task)
    return -EINVAL;
// This helper must only be called from places where the lifetime of the task
// is guaranteed. Either by being refcounted or by being protected
// by an RCU read-side critical section.
//
    return task_storage_delete(task, map);
    }
#[no_mangle]
unsafe extern "C" fn notsupp_get_next_key(map: *mut bpf_map, key: *mut c_void, next_key: *mut c_void) -> c_int {
    static int notsupp_get_next_key(struct bpf_map *map, void *key, void *next_key)
    {
    return -ENOTSUPP;
    }
    static struct bpf_map *task_storage_map_alloc(union bpf_attr *attr)
    {
    return bpf_local_storage_map_alloc(attr, &task_cache);
    }
#[no_mangle]
unsafe extern "C" fn task_storage_map_free(map: *mut bpf_map) {
    static void task_storage_map_free(struct bpf_map *map)
    {
    bpf_local_storage_map_free(map, &task_cache);
    }
    BTF_ID_LIST_GLOBAL_SINGLE(bpf_local_storage_map_btf_id, struct, bpf_local_storage_map)
    const struct bpf_map_ops task_storage_map_ops = {
    .map_meta_equal = bpf_map_meta_equal,
    .map_alloc_check = bpf_local_storage_map_alloc_check,
    .map_alloc = task_storage_map_alloc,
    .map_free = task_storage_map_free,
    .map_get_next_key = notsupp_get_next_key,
    .map_lookup_elem = bpf_pid_task_storage_lookup_elem,
    .map_update_elem = bpf_pid_task_storage_update_elem,
    .map_delete_elem = bpf_pid_task_storage_delete_elem,
    .map_check_btf = bpf_local_storage_map_check_btf,
    .map_mem_usage = bpf_local_storage_map_mem_usage,
    .map_btf_id = &bpf_local_storage_map_btf_id[0],
    .map_owner_storage_ptr = task_storage_ptr,
    };
    const struct bpf_func_proto bpf_task_storage_get_proto = {
    .func = bpf_task_storage_get,
    .gpl_only = false,
    .ret_type = RET_PTR_TO_MAP_VALUE_OR_NULL,
    .arg1_type = ARG_CONST_MAP_PTR,
    .arg2_type = ARG_PTR_TO_BTF_ID_OR_NULL,
    .arg2_btf_id = &btf_tracing_ids[BTF_TRACING_TYPE_TASK],
    .arg3_type = ARG_PTR_TO_MAP_VALUE_OR_NULL,
    .arg4_type = ARG_ANYTHING,
    };
    const struct bpf_func_proto bpf_task_storage_delete_proto = {
    .func = bpf_task_storage_delete,
    .gpl_only = false,
    .ret_type = RET_INTEGER,
    .arg1_type = ARG_CONST_MAP_PTR,
    .arg2_type = ARG_PTR_TO_BTF_ID_OR_NULL,
    .arg2_btf_id = &btf_tracing_ids[BTF_TRACING_TYPE_TASK],
    };
