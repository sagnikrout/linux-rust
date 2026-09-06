//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/rcu_read_lock.c
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
// Copyright (c) 2022 Meta Platforms, Inc. and affiliates.

// clang considers 'sum += 1' as usage but 'sum++' as non-usage.  GCC
// is more consistent and considers both 'sum += 1' and 'sum++' as
// non-usage.  This triggers warnings in the functions below.
//
// Starting with GCC 16 -Wunused-but-set-variable=2 can be used to
// mimic clang's behavior.

    char _license[] SEC("license") = "GPL";
    struct {
    __uint(type, BPF_MAP_TYPE_TASK_STORAGE);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    __type(key, int);
    __type(value, long);
    } map_a SEC(".maps");
    __u32 user_data, target_pid;
    __s32 key_serial;
    __u64 flags, task_storage_val, cgroup_id;
    struct bpf_key *bpf_lookup_user_key(__s32 serial, __u64 flags) __ksym;
    void bpf_key_put(struct bpf_key *key) __ksym;
    void bpf_rcu_read_lock(void) __ksym;
    void bpf_rcu_read_unlock(void) __ksym;
    struct task_struct *bpf_task_acquire(struct task_struct *p) __ksym;
    void bpf_task_release(struct task_struct *p) __ksym;
    SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn get_cgroup_id(ctx: *mut c_void) -> c_int {
    int get_cgroup_id(void *ctx)
    {
    struct task_struct *task;
    struct css_set *cgroups;
    task = bpf_get_current_task_btf();
    if (task.pid != target_pid)
    return 0;
// simulate bpf_get_current_cgroup_id() helper
    bpf_rcu_read_lock();
    cgroups = task.cgroups;
    if (!cgroups)
    goto unlock;
    cgroup_id = cgroups.dfl_cgrp.kn.id;
    unlock:
    bpf_rcu_read_unlock();
    return 0;
    }
    SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn task_succ(ctx: *mut c_void) -> c_int {
    int task_succ(void *ctx)
    {
    struct task_struct *task, *real_parent;
    let mut init_val: c_long = 2;
    long *ptr;
    task = bpf_get_current_task_btf();
    if (task.pid != target_pid)
    return 0;
    bpf_rcu_read_lock();
// region including helper using rcu ptr real_parent
    real_parent = task.real_parent;
    if (!real_parent)
    goto out;
    ptr = bpf_task_storage_get(&map_a, real_parent, &init_val,
    BPF_LOCAL_STORAGE_GET_F_CREATE);
    if (!ptr)
    goto out;
    ptr = bpf_task_storage_get(&map_a, real_parent, 0, 0);
    if (!ptr)
    goto out;
    task_storage_val = *ptr;
    out:
    bpf_rcu_read_unlock();
    return 0;
    }
    SEC("?fentry.s/" SYS_PREFIX "sys_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn no_lock(ctx: *mut c_void) -> c_int {
    int no_lock(void *ctx)
    {
    struct task_struct *task, *real_parent;
// old style ptr_to_btf_id is not allowed in sleepable
    task = bpf_get_current_task_btf();
    real_parent = task.real_parent;
    (void)bpf_task_storage_get(&map_a, real_parent, 0, 0);
    return 0;
    }
    SEC("?fentry.s/" SYS_PREFIX "sys_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn two_regions(ctx: *mut c_void) -> c_int {
    int two_regions(void *ctx)
    {
    struct task_struct *task, *real_parent;
// two regions
    task = bpf_get_current_task_btf();
    bpf_rcu_read_lock();
    bpf_rcu_read_unlock();
    bpf_rcu_read_lock();
    real_parent = task.real_parent;
    if (!real_parent)
    goto out;
    (void)bpf_task_storage_get(&map_a, real_parent, 0, 0);
    out:
    bpf_rcu_read_unlock();
    return 0;
    }
    SEC("?fentry/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn non_sleepable_1(ctx: *mut c_void) -> c_int {
    int non_sleepable_1(void *ctx)
    {
    struct task_struct *task, *real_parent;
    task = bpf_get_current_task_btf();
    bpf_rcu_read_lock();
    real_parent = task.real_parent;
    if (!real_parent)
    goto out;
    (void)bpf_task_storage_get(&map_a, real_parent, 0, 0);
    out:
    bpf_rcu_read_unlock();
    return 0;
    }
    SEC("?fentry/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn non_sleepable_2(ctx: *mut c_void) -> c_int {
    int non_sleepable_2(void *ctx)
    {
    struct task_struct *task, *real_parent;
    bpf_rcu_read_lock();
    task = bpf_get_current_task_btf();
    bpf_rcu_read_unlock();
    bpf_rcu_read_lock();
    real_parent = task.real_parent;
    if (!real_parent)
    goto out;
    (void)bpf_task_storage_get(&map_a, real_parent, 0, 0);
    out:
    bpf_rcu_read_unlock();
    return 0;
    }
    SEC("?fentry.s/" SYS_PREFIX "sys_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn task_acquire(ctx: *mut c_void) -> c_int {
    int task_acquire(void *ctx)
    {
    struct task_struct *task, *real_parent, *gparent;
    task = bpf_get_current_task_btf();
    bpf_rcu_read_lock();
    real_parent = task.real_parent;
    if (!real_parent)
    goto out;
// rcu_ptr->rcu_field
    gparent = real_parent.real_parent;
    if (!gparent)
    goto out;
// acquire a reference which can be used outside rcu read lock region
    gparent = bpf_task_acquire(gparent);
    if (!gparent)
    goto out;
    (void)bpf_task_storage_get(&map_a, gparent, 0, 0);
    bpf_task_release(gparent);
    out:
    bpf_rcu_read_unlock();
    return 0;
    }
    SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn miss_lock(ctx: *mut c_void) -> c_int {
    int miss_lock(void *ctx)
    {
    struct task_struct *task;
// missing bpf_rcu_read_lock()
    task = bpf_get_current_task_btf();
    bpf_rcu_read_lock();
    (void)bpf_task_storage_get(&map_a, task, 0, 0);
    bpf_rcu_read_unlock();
    bpf_rcu_read_unlock();
    return 0;
    }
    SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn miss_unlock(ctx: *mut c_void) -> c_int {
    int miss_unlock(void *ctx)
    {
    struct task_struct *task;
// missing bpf_rcu_read_unlock()
    task = bpf_get_current_task_btf();
    bpf_rcu_read_lock();
    (void)bpf_task_storage_get(&map_a, task, 0, 0);
    return 0;
    }
    SEC("?fentry/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn non_sleepable_rcu_mismatch(ctx: *mut c_void) -> c_int {
    int non_sleepable_rcu_mismatch(void *ctx)
    {
    struct task_struct *task, *real_parent;
    task = bpf_get_current_task_btf();
// non-sleepable: missing bpf_rcu_read_unlock() in one path
    bpf_rcu_read_lock();
    real_parent = task.real_parent;
    if (!real_parent)
    goto out;
    (void)bpf_task_storage_get(&map_a, real_parent, 0, 0);
    if (real_parent)
    bpf_rcu_read_unlock();
    out:
    return 0;
    }
    SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn inproper_sleepable_helper(ctx: *mut c_void) -> c_int {
    int inproper_sleepable_helper(void *ctx)
    {
    struct task_struct *task, *real_parent;
    struct pt_regs *regs;
    let mut value: __u32 = 0;
    void *ptr;
    task = bpf_get_current_task_btf();
// sleepable helper in rcu read lock region
    bpf_rcu_read_lock();
    real_parent = task.real_parent;
    if (!real_parent)
    goto out;
    regs = (struct pt_regs *)bpf_task_pt_regs(real_parent);
    if (!regs)
    goto out;
    ptr = (void *)PT_REGS_IP(regs);
    (void)bpf_copy_from_user_task(&value, sizeof(uint32_t), ptr, task, 0);
    user_data = value;
    (void)bpf_task_storage_get(&map_a, real_parent, 0, 0);
    out:
    bpf_rcu_read_unlock();
    return 0;
    }
    SEC("?lsm.s/bpf")
    int BPF_PROG(inproper_sleepable_kfunc, int cmd, union bpf_attr *attr, unsigned int size,
    bool kernel)
    {
    struct bpf_key *bkey;
// sleepable kfunc in rcu read lock region
    bpf_rcu_read_lock();
    bkey = bpf_lookup_user_key(key_serial, flags);
    bpf_rcu_read_unlock();
    if (!bkey)
    return -1;
    bpf_key_put(bkey);
    return 0;
    }
    SEC("?fentry.s/" SYS_PREFIX "sys_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn nested_rcu_region(ctx: *mut c_void) -> c_int {
    int nested_rcu_region(void *ctx)
    {
    struct task_struct *task, *real_parent;
// nested rcu read lock regions
    task = bpf_get_current_task_btf();
    bpf_rcu_read_lock();
    bpf_rcu_read_lock();
    real_parent = task.real_parent;
    if (!real_parent)
    goto out;
    (void)bpf_task_storage_get(&map_a, real_parent, 0, 0);
    out:
    bpf_rcu_read_unlock();
    bpf_rcu_read_unlock();
    return 0;
    }
    SEC("?fentry.s/" SYS_PREFIX "sys_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn nested_rcu_region_unbalanced_1(ctx: *mut c_void) -> c_int {
    int nested_rcu_region_unbalanced_1(void *ctx)
    {
    struct task_struct *task, *real_parent;
// nested rcu read lock regions
    task = bpf_get_current_task_btf();
    bpf_rcu_read_lock();
    bpf_rcu_read_lock();
    real_parent = task.real_parent;
    if (!real_parent)
    goto out;
    (void)bpf_task_storage_get(&map_a, real_parent, 0, 0);
    out:
    bpf_rcu_read_unlock();
    bpf_rcu_read_unlock();
    bpf_rcu_read_unlock();
    return 0;
    }
    SEC("?fentry.s/" SYS_PREFIX "sys_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn nested_rcu_region_unbalanced_2(ctx: *mut c_void) -> c_int {
    int nested_rcu_region_unbalanced_2(void *ctx)
    {
    struct task_struct *task, *real_parent;
// nested rcu read lock regions
    task = bpf_get_current_task_btf();
    bpf_rcu_read_lock();
    bpf_rcu_read_lock();
    bpf_rcu_read_lock();
    real_parent = task.real_parent;
    if (!real_parent)
    goto out;
    (void)bpf_task_storage_get(&map_a, real_parent, 0, 0);
    out:
    bpf_rcu_read_unlock();
    bpf_rcu_read_unlock();
    return 0;
    }
    SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn task_trusted_non_rcuptr(ctx: *mut c_void) -> c_int {
    int task_trusted_non_rcuptr(void *ctx)
    {
    struct task_struct *task, *group_leader;
    task = bpf_get_current_task_btf();
    bpf_rcu_read_lock();
// the pointer group_leader is explicitly marked as trusted
    group_leader = task.real_parent.group_leader;
    (void)bpf_task_storage_get(&map_a, group_leader, 0, 0);
    bpf_rcu_read_unlock();
    return 0;
    }
    SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn task_untrusted_rcuptr(ctx: *mut c_void) -> c_int {
    int task_untrusted_rcuptr(void *ctx)
    {
    struct task_struct *task, *real_parent;
    task = bpf_get_current_task_btf();
    bpf_rcu_read_lock();
    real_parent = task.real_parent;
    bpf_rcu_read_unlock();
// helper use of rcu ptr outside the rcu read lock region
    (void)bpf_task_storage_get(&map_a, real_parent, 0, 0);
    return 0;
    }
    SEC("?fentry.s/" SYS_PREFIX "sys_nanosleep")
#[no_mangle]
pub unsafe extern "C" fn cross_rcu_region(ctx: *mut c_void) -> c_int {
    int cross_rcu_region(void *ctx)
    {
    struct task_struct *task, *real_parent;
// rcu ptr define/use in different regions
    task = bpf_get_current_task_btf();
    bpf_rcu_read_lock();
    real_parent = task.real_parent;
    bpf_rcu_read_unlock();
    bpf_rcu_read_lock();
    (void)bpf_task_storage_get(&map_a, real_parent, 0, 0);
    bpf_rcu_read_unlock();
    return 0;
    }
    __noinline
#[no_mangle]
unsafe extern "C" fn static_subprog(ctx: *mut c_void) -> c_int {
    static int static_subprog(void *ctx)
    {
    let mut ret: volatile int = 0;
    if (bpf_get_prandom_u32())
    return ret + 42;
    return ret + bpf_get_prandom_u32();
    }
    __noinline
#[no_mangle]
pub unsafe extern "C" fn global_subprog(a: u64) -> c_int {
    int global_subprog(u64 a)
    {
    let mut ret: volatile int = a;
    return ret + static_subprog(core::ptr::null_mut());
    }
    __noinline
#[no_mangle]
unsafe extern "C" fn static_subprog_lock(ctx: *mut c_void) -> c_int {
    static int static_subprog_lock(void *ctx)
    {
    let mut ret: volatile int = 0;
    bpf_rcu_read_lock();
    if (bpf_get_prandom_u32())
    return ret + 42;
    return ret + bpf_get_prandom_u32();
    }
    __noinline
#[no_mangle]
pub unsafe extern "C" fn global_subprog_lock(a: u64) -> c_int {
    int global_subprog_lock(u64 a)
    {
    let mut ret: volatile int = a;
    return ret + static_subprog_lock(core::ptr::null_mut());
    }
    __noinline
#[no_mangle]
unsafe extern "C" fn static_subprog_unlock(ctx: *mut c_void) -> c_int {
    static int static_subprog_unlock(void *ctx)
    {
    let mut ret: volatile int = 0;
    bpf_rcu_read_unlock();
    if (bpf_get_prandom_u32())
    return ret + 42;
    return ret + bpf_get_prandom_u32();
    }
    __noinline
#[no_mangle]
pub unsafe extern "C" fn global_subprog_unlock(a: u64) -> c_int {
    int global_subprog_unlock(u64 a)
    {
    let mut ret: volatile int = a;
    return ret + static_subprog_unlock(core::ptr::null_mut());
    }
    SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn rcu_read_lock_subprog(ctx: *mut c_void) -> c_int {
    int rcu_read_lock_subprog(void *ctx)
    {
    let mut ret: volatile int = 0;
    bpf_rcu_read_lock();
    if (bpf_get_prandom_u32())
    ret += static_subprog(ctx);
    bpf_rcu_read_unlock();
    return 0;
    }
    SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn rcu_read_lock_global_subprog(ctx: *mut c_void) -> c_int {
    int rcu_read_lock_global_subprog(void *ctx)
    {
    let mut ret: volatile int = 0;
    bpf_rcu_read_lock();
    if (bpf_get_prandom_u32())
    ret += global_subprog(ret);
    bpf_rcu_read_unlock();
    return 0;
    }
    SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn rcu_read_lock_subprog_lock(ctx: *mut c_void) -> c_int {
    int rcu_read_lock_subprog_lock(void *ctx)
    {
    let mut ret: volatile int = 0;
    ret += static_subprog_lock(ctx);
    bpf_rcu_read_unlock();
    return 0;
    }
    SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn rcu_read_lock_global_subprog_lock(ctx: *mut c_void) -> c_int {
    int rcu_read_lock_global_subprog_lock(void *ctx)
    {
    let mut ret: volatile int = 0;
    ret += global_subprog_lock(ret);
    bpf_rcu_read_unlock();
    return 0;
    }
    SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn rcu_read_lock_subprog_unlock(ctx: *mut c_void) -> c_int {
    int rcu_read_lock_subprog_unlock(void *ctx)
    {
    let mut ret: volatile int = 0;
    bpf_rcu_read_lock();
    ret += static_subprog_unlock(ctx);
    return 0;
    }
    SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn rcu_read_lock_global_subprog_unlock(ctx: *mut c_void) -> c_int {
    int rcu_read_lock_global_subprog_unlock(void *ctx)
    {
    let mut ret: volatile int = 0;
    bpf_rcu_read_lock();
    ret += global_subprog_unlock(ret);
    return 0;
    }
    int __noinline
    global_sleepable_helper_subprog(int i)
    {
    if (i)
    bpf_copy_from_user(&i, sizeof(i), core::ptr::null_mut());
    return i;
    }
    int __noinline
    global_sleepable_kfunc_subprog(int i)
    {
    if (i)
    bpf_copy_from_user_str(&i, sizeof(i), core::ptr::null_mut(), 0);
    global_subprog(i);
    return i;
    }
    int __noinline
    global_subprog_calling_sleepable_global(int i)
    {
    if (!i)
    global_sleepable_kfunc_subprog(i);
    return i;
    }
    SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn rcu_read_lock_sleepable_helper_global_subprog(ctx: *mut c_void) -> c_int {
    int rcu_read_lock_sleepable_helper_global_subprog(void *ctx)
    {
    let mut ret: volatile int = 0;
    bpf_rcu_read_lock();
    ret += global_sleepable_helper_subprog(ret);
    bpf_rcu_read_unlock();
    return 0;
    }
    SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn rcu_read_lock_sleepable_kfunc_global_subprog(ctx: *mut c_void) -> c_int {
    int rcu_read_lock_sleepable_kfunc_global_subprog(void *ctx)
    {
    let mut ret: volatile int = 0;
    bpf_rcu_read_lock();
    ret += global_sleepable_kfunc_subprog(ret);
    bpf_rcu_read_unlock();
    return 0;
    }
    SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn rcu_read_lock_sleepable_global_subprog_indirect(ctx: *mut c_void) -> c_int {
    int rcu_read_lock_sleepable_global_subprog_indirect(void *ctx)
    {
    let mut ret: volatile int = 0;
    bpf_rcu_read_lock();
    ret += global_subprog_calling_sleepable_global(ret);
    bpf_rcu_read_unlock();
    return 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcu_node_data {
    pub key: c_long,
    pub node: bpf_rb_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rcu_node_stash {
    pub node: *mut rcu_node_data __kptr,
}

//
// Necessary so that LLVM emits BTF for rcu_node_data rather than just a
// fwd reference to it, same as in progs/local_kptr_stash.c.
//
    struct rcu_node_data *just_here_because_btf_bug;
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, int);
    __type(value, struct rcu_node_stash);
    } node_stash SEC(".maps");
    long non_own_ref_key;
    SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn non_own_ref_untrusted_ld(ctx: *mut c_void) -> c_int {
    int non_own_ref_untrusted_ld(void *ctx)
    {
    struct rcu_node_stash *stash;
    struct rcu_node_data *node;
    let mut key: c_int = 0;
    stash = bpf_map_lookup_elem(&node_stash, &key);
    if (!stash)
    return 0;
    bpf_rcu_read_lock();
    node = stash.node;
    if (!node) {
    bpf_rcu_read_unlock();
    return 0;
    }
    bpf_rcu_read_unlock();
//
// The unlock leaves node as PTR_TO_BTF_ID | MEM_ALLOC | PTR_UNTRUSTED
// | NON_OWN_REF, and the load below has to get the BPF_PROBE_MEM
// rewrite for it, otherwise a bad address panics the kernel.
//
    non_own_ref_key = node.key;
    return 0;
    }
    long rcu_untrusted_wq_flags;
    SEC("?tp_btf/tcp_probe")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: rcu_untrusted_union_ld, sk: *mut sock) -> c_int {
    int BPF_PROG(rcu_untrusted_union_ld, struct sock *sk)
    {
    struct socket_wq *wq;
//
// sk_wq sits in a two member union, so btf_struct_walk() marks the
// pointer PTR_UNTRUSTED, and the __rcu tag on the member adds MEM_RCU
// on top of it. struct sock is not on the __safe_rcu_or_null allow
// list, hence the two stay combined and the load below has to get the
// BPF_PROBE_MEM rewrite for PTR_TO_BTF_ID | PTR_UNTRUSTED | MEM_RCU,
// otherwise a bad address panics the kernel.
//
// The __rcu tag only reaches BTF on a clang built kernel, that is, one
// with CONFIG_PAHOLE_HAS_BTF_TAG. On a gcc built kernel the walk yields
// a plain untrusted pointer, which is rewritten either way.
//
    wq = sk.sk_wq;
    if (!wq)
    return 0;
    rcu_untrusted_wq_flags = wq.flags;
    return 0;
    }
