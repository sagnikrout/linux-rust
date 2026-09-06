//! Automatically rewritten from C to Rust
//! Source: tools/perf/util/bpf_skel/lock_contention.bpf.c
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (c) 2022 Google

// for collect_lock_syms().  4096 was rejected by the verifier
pub const MAX_CPUS: c_int = 1024;
// for collect_zone_lock().  It should be more than the actual zones.
pub const MAX_ZONES: c_int = 10;
// for do_lock_delay().  Arbitrarily set to 1 million.

// lock contention flags from include/trace/events/lock.h

// callstack storage
    struct {
    __uint(type, BPF_MAP_TYPE_STACK_TRACE);
    __uint(key_size, sizeof(__u32));
    __uint(value_size, sizeof(__u64));
    __uint(max_entries, MAX_ENTRIES);
    } stacks SEC(".maps");
// buffer for owner stacktrace
    struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
    __uint(key_size, sizeof(__u32));
    __uint(value_size, sizeof(__u64));
    __uint(max_entries, 1);
    } stack_buf SEC(".maps");
// a map for tracing owner stacktrace to owner stack id
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(key_size, sizeof(__u64)); // owner stacktrace
    __uint(value_size, sizeof(__s32)); // owner stack id
    __uint(max_entries, 1);
    } owner_stacks SEC(".maps");
// a map for tracing lock address to owner data
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(key_size, sizeof(__u64)); // lock address
    __uint(value_size, sizeof(struct owner_tracing_data));
    __uint(max_entries, 1);
    } owner_data SEC(".maps");
// a map for contention_key (stores owner stack id) to contention data
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(key_size, sizeof(struct contention_key));
    __uint(value_size, sizeof(struct contention_data));
    __uint(max_entries, 1);
    } owner_stat SEC(".maps");
// maintain timestamp at the beginning of contention
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __type(key, int);
    __type(value, struct tstamp_data);
    __uint(max_entries, MAX_ENTRIES);
    } tstamp SEC(".maps");
// maintain per-CPU timestamp at the beginning of contention
    struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
    __uint(key_size, sizeof(__u32));
    __uint(value_size, sizeof(struct tstamp_data));
    __uint(max_entries, 1);
    } tstamp_cpu SEC(".maps");
// actual lock contention statistics
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(key_size, sizeof(struct contention_key));
    __uint(value_size, sizeof(struct contention_data));
    __uint(max_entries, MAX_ENTRIES);
    } lock_stat SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(key_size, sizeof(__u32));
    __uint(value_size, sizeof(struct contention_task_data));
    __uint(max_entries, MAX_ENTRIES);
    } task_data SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(key_size, sizeof(__u64));
    __uint(value_size, sizeof(__u32));
    __uint(max_entries, MAX_ENTRIES);
    } lock_syms SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(key_size, sizeof(__u32));
    __uint(value_size, sizeof(__u8));
    __uint(max_entries, 1);
    } cpu_filter SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(key_size, sizeof(__u32));
    __uint(value_size, sizeof(__u8));
    __uint(max_entries, 1);
    } task_filter SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(key_size, sizeof(__u32));
    __uint(value_size, sizeof(__u8));
    __uint(max_entries, 1);
    } type_filter SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(key_size, sizeof(__u64));
    __uint(value_size, sizeof(__u8));
    __uint(max_entries, 1);
    } addr_filter SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(key_size, sizeof(__u64));
    __uint(value_size, sizeof(__u8));
    __uint(max_entries, 1);
    } cgroup_filter SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(key_size, sizeof(long));
    __uint(value_size, sizeof(__u8));
    __uint(max_entries, 1);
    } slab_filter SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(key_size, sizeof(long));
    __uint(value_size, sizeof(struct slab_cache_data));
    __uint(max_entries, 1);
    } slab_caches SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(key_size, sizeof(__u64));
    __uint(value_size, sizeof(__u64));
    __uint(max_entries, 1);
    } lock_delays SEC(".maps");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rw_semaphore___old {
    pub owner: *mut task_struct,
    pub __attribute__((preserve_access_index)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rw_semaphore___new {
    pub owner: atomic_long_t,
    pub __attribute__((preserve_access_index)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mm_struct___old {
    pub mmap_sem: rw_semaphore,
    pub __attribute__((preserve_access_index)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mm_struct___new {
    pub mmap_lock: rw_semaphore,
    pub __attribute__((preserve_access_index)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cas_ctx {
    pub data: *mut contention_data,
    pub duration: u64,
    pub max_done: c_int,
    pub min_done: c_int,
}

    extern struct kmem_cache *bpf_get_kmem_cache(u64 addr) __ksym __weak;
// control flags
    const volatile int has_cpu;
    const volatile int has_task;
    const volatile int has_type;
    const volatile int has_addr;
    const volatile int has_cgroup;
    const volatile int has_slab;
    const volatile int has_mmap_lock;
    const volatile int needs_callstack;
    const volatile int stack_skip;
    const volatile int lock_owner;
    const volatile int use_cgroup_v2;
    const volatile int max_stack;
    const volatile int lock_delay;
// determine the key of lock stat
    const volatile int aggr_mode;
    int enabled;
    let mut perf_subsys_id: c_int = -1;
    __u64 end_ts;
    __u32 slab_cache_id;
// error stat
    int task_fail;
    int stack_fail;
    int time_fail;
    int data_fail;
    int task_map_full;
    int data_map_full;
    struct task_struct *bpf_task_from_pid(s32 pid) __ksym __weak;
    void bpf_task_release(struct task_struct *p) __ksym __weak;
    static inline __u32 check_lock_type(__u64 lock, __u32 flags);
#[no_mangle]
pub unsafe extern "C" fn get_current_cgroup_id() -> __u64 {
    static inline __u64 get_current_cgroup_id(void)
    {
    struct task_struct *task;
    struct cgroup *cgrp;
    if (use_cgroup_v2)
    return bpf_get_current_cgroup_id();
    task = bpf_get_current_task_btf();
    if (perf_subsys_id == -1) {

    perf_subsys_id = bpf_core_enum_value(enum cgroup_subsys_id,
    perf_event_cgrp_id);

    perf_subsys_id = perf_event_cgrp_id;

    }
    cgrp = BPF_CORE_READ(task, cgroups, subsys[perf_subsys_id], cgroup);
    return BPF_CORE_READ(cgrp, kn, id);
    }
#[no_mangle]
pub unsafe extern "C" fn can_record(ctx: *mut u64) -> c_int {
    static inline int can_record(u64 *ctx)
    {
    let mut is_addr_ok: bool = false;
    if (has_cpu) {
    let mut cpu: __u32 = bpf_get_smp_processor_id();
    __u8 *ok;
    ok = bpf_map_lookup_elem(&cpu_filter, &cpu);
    if (!ok)
    return 0;
    }
    if (has_task) {
    __u8 *ok;
    let mut pid: __u32 = bpf_get_current_pid_tgid();
    ok = bpf_map_lookup_elem(&task_filter, &pid);
    if (!ok)
    return 0;
    }
    if (has_type) {
    __u8 *ok;
    let mut flags: __u32 = (__u32)ctx[1];
    ok = bpf_map_lookup_elem(&type_filter, &flags);
    if (!ok)
    return 0;
    }
    if (has_addr) {
    __u8 *ok;
    let mut addr: __u64 = ctx[0];
    ok = bpf_map_lookup_elem(&addr_filter, &addr);
    if (!ok && !has_slab && !has_mmap_lock)
    return 0;
    is_addr_ok = !!ok;
    }
    if (has_cgroup) {
    __u8 *ok;
    let mut cgrp: __u64 = get_current_cgroup_id();
    ok = bpf_map_lookup_elem(&cgroup_filter, &cgrp);
    if (!ok)
    return 0;
    }
    if (is_addr_ok)
    return 1;
// slab and mmap_lock are part of the addr_filter
    if (has_slab && bpf_get_kmem_cache) {
    __u8 *ok;
    let mut addr: __u64 = ctx[0];
    long kmem_cache_addr;
    kmem_cache_addr = (long)bpf_get_kmem_cache(addr);
    ok = bpf_map_lookup_elem(&slab_filter, &kmem_cache_addr);
    if (ok)
    return 1;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !has_mmap_lock) -> else {
    else if (!has_mmap_lock)
    return 0;
    }
    if (has_mmap_lock) {
    let mut lock: __u64 = ctx[0];
    let mut flag: __u32 = ctx[1];
    if (check_lock_type(lock, flag) != LCD_F_MMAP_LOCK)
    return 0;
    }
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn update_task_data(task: *mut task_struct) -> c_int {
    static inline int update_task_data(struct task_struct *task)
    {
    struct contention_task_data *p;
    int pid, err;
    err = bpf_core_read(&pid, sizeof(pid), &task.pid);
    if (err)
    return -1;
    p = bpf_map_lookup_elem(&task_data, &pid);
    if (p == core::ptr::null_mut() && !task_map_full) {
    let mut data: contention_task_data = {};
    BPF_CORE_READ_STR_INTO(&data.comm, task, comm);
    if (bpf_map_update_elem(&task_data, &pid, &data, BPF_NOEXIST) == -E2BIG)
    task_map_full = 1;
    }
    return 0;
    }

    static inline struct task_struct *get_lock_owner(__u64 lock, __u32 flags)
    {
    struct task_struct *task;
    let mut owner: __u64 = 0;
    if (flags & LCB_F_MUTEX) {
    struct mutex *mutex = (void *)lock;
    owner = BPF_CORE_READ(mutex, owner.counter);
    } else if (flags == LCB_F_READ || flags == LCB_F_WRITE) {
//
// Support for the BPF_TYPE_MATCHES argument to the
// __builtin_preserve_type_info builtin was added at some point during
// development of clang 15 and it's what is needed for
// bpf_core_type_matches.
//

    if (bpf_core_type_matches(struct rw_semaphore___old)) {
    struct rw_semaphore___old *rwsem = (void *)lock;
    owner = (unsigned long)BPF_CORE_READ(rwsem, owner);
    } else if (bpf_core_type_matches(struct rw_semaphore___new)) {
    struct rw_semaphore___new *rwsem = (void *)lock;
    owner = BPF_CORE_READ(rwsem, owner.counter);
    }

// assume new struct
    struct rw_semaphore *rwsem = (void *)lock;
    owner = BPF_CORE_READ(rwsem, owner.counter);

    }
    if (!owner)
    return core::ptr::null_mut();
    task = (void *)(owner & ~7UL);
    return task;
    }
#[no_mangle]
pub unsafe extern "C" fn check_lock_type(lock: __u64, flags: __u32) -> __u32 {
    static inline __u32 check_lock_type(__u64 lock, __u32 flags)
    {
    struct task_struct *curr;
    struct mm_struct___old *mm_old;
    struct mm_struct___new *mm_new;
    struct sighand_struct *sighand;
    switch (flags) {
    case LCB_F_READ:  /* rwsem */
    case LCB_F_WRITE:
    curr = bpf_get_current_task_btf();
    if (curr.mm == core::ptr::null_mut())
    break;
    mm_new = (void *)curr.mm;
    if (bpf_core_field_exists(mm_new.mmap_lock)) {
    if (&mm_new.mmap_lock == (void *)lock)
    return LCD_F_MMAP_LOCK;
    break;
    }
    mm_old = (void *)curr.mm;
    if (bpf_core_field_exists(mm_old.mmap_sem)) {
    if (&mm_old.mmap_sem == (void *)lock)
    return LCD_F_MMAP_LOCK;
    }
    break;
    case LCB_F_SPIN:  /* spinlock */
    curr = bpf_get_current_task_btf();
    sighand = curr.sighand;
    if (sighand && &sighand.siglock == (void *)lock)
    return LCD_F_SIGHAND_LOCK;
    break;
    default:
    break;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn delay_callback(idx: __u64, arg: *mut c_void) -> c_long {
    static inline long delay_callback(__u64 idx, void *arg)
    {
    let mut target: __u64 = *(__u64 *)arg;
    if (target <= bpf_ktime_get_ns())
    return 1;
// just to kill time
    (void)bpf_get_prandom_u32();
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn do_lock_delay(duration: __u64) {
    static inline void do_lock_delay(__u64 duration)
    {
    let mut target: __u64 = bpf_ktime_get_ns() + duration;
    bpf_loop(MAX_LOOP, delay_callback, &target, /*flags=*/0);
    }
#[no_mangle]
pub unsafe extern "C" fn check_lock_delay(lock: __u64) {
    static inline void check_lock_delay(__u64 lock)
    {
    __u64 *delay;
    delay = bpf_map_lookup_elem(&lock_delays, &lock);
    if (delay)
    do_lock_delay(*delay);
    }
    static inline struct tstamp_data *get_tstamp_elem(__u32 flags)
    {
    __u32 pid;
    struct tstamp_data *pelem;
// Use per-cpu array map for spinlock and rwlock
    if ((flags & (LCB_F_SPIN | LCB_F_MUTEX)) == LCB_F_SPIN) {
    let mut idx: __u32 = 0;
    pelem = bpf_map_lookup_elem(&tstamp_cpu, &idx);
// Do not update the element for nested locks
    if (pelem && pelem.lock)
    pelem = core::ptr::null_mut();
    return pelem;
    }
    pid = bpf_get_current_pid_tgid();
    pelem = bpf_map_lookup_elem(&tstamp, &pid);
// Do not update the element for nested locks
    if (pelem && pelem.lock)
    return core::ptr::null_mut();
    if (pelem == core::ptr::null_mut()) {
    let mut zero: tstamp_data = {};
    if (bpf_map_update_elem(&tstamp, &pid, &zero, BPF_NOEXIST) < 0) {
    __sync_fetch_and_add(&task_fail, 1);
    return core::ptr::null_mut();
    }
    pelem = bpf_map_lookup_elem(&tstamp, &pid);
    if (pelem == core::ptr::null_mut()) {
    __sync_fetch_and_add(&task_fail, 1);
    return core::ptr::null_mut();
    }
    }
    return pelem;
    }
#[no_mangle]
pub unsafe extern "C" fn get_owner_stack_id(stacktrace: *mut u64) -> i32 {
    static inline s32 get_owner_stack_id(u64 *stacktrace)
    {
    s32 *id, new_id;
    let mut id_gen: static s64 = 1;
    id = bpf_map_lookup_elem(&owner_stacks, stacktrace);
    if (id)
    return *id;
    new_id = (s32)__sync_fetch_and_add(&id_gen, 1);
    bpf_map_update_elem(&owner_stacks, stacktrace, &new_id, BPF_NOEXIST);
    id = bpf_map_lookup_elem(&owner_stacks, stacktrace);
    if (id)
    return *id;
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn cas_min_max_cb(idx: u64, arg: *mut c_void) -> c_long {
    static long cas_min_max_cb(u64 idx, void *arg)
    {
    struct cas_ctx *ctx = arg;
    if (!ctx.max_done) {
    let mut old_max: u64 = ctx.data.max_time;
    if (old_max >= ctx.duration) {
    ctx.max_done = 1;
    } else {
    u64 r = __sync_val_compare_and_swap(
    &ctx.data.max_time, old_max, ctx.duration);
    if (r == old_max)
    ctx.max_done = 1;
    }
    }
    if (!ctx.min_done) {
    let mut old_min: u64 = ctx.data.min_time;
    if (old_min <= ctx.duration) {
    ctx.min_done = 1;
    } else {
    u64 r = __sync_val_compare_and_swap(
    &ctx.data.min_time, old_min, ctx.duration);
    if (r == old_min)
    ctx.min_done = 1;
    }
    }
    return (ctx.max_done && ctx.min_done) ? 1 : 0;
    }
#[no_mangle]
pub unsafe extern "C" fn update_contention_data(data: *mut contention_data, duration: u64, count: u32) {
    static inline void update_contention_data(struct contention_data *data, u64 duration, u32 count)
    {
    __sync_fetch_and_add(&data.total_time, duration);
    __sync_fetch_and_add(&data.count, count);
    struct cas_ctx ctx = {
    .data     = data,
    .duration = duration,
    .max_done = 0,
    .min_done = 0,
    };
    bpf_loop(64, cas_min_max_cb, &ctx, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn update_owner_stat(id: u32, duration: u64, flags: u32) {
    static inline void update_owner_stat(u32 id, u64 duration, u32 flags)
    {
    struct contention_key key = {
    .stack_id = id,
    .pid = 0,
    .lock_addr_or_cgroup = 0,
    };
    struct contention_data *data = bpf_map_lookup_elem(&owner_stat, &key);
    if (!data) {
    struct contention_data first = {
    .total_time = duration,
    .max_time = duration,
    .min_time = duration,
    .count = 1,
    .flags = flags,
    };
    bpf_map_update_elem(&owner_stat, &key, &first, BPF_NOEXIST);
    } else {
    update_contention_data(data, duration, 1);
    }
    }
    SEC("tp_btf/contention_begin")
#[no_mangle]
pub unsafe extern "C" fn contention_begin(ctx: *mut u64) -> c_int {
    int contention_begin(u64 *ctx)
    {
    struct tstamp_data *pelem;
    if (!enabled || !can_record(ctx))
    return 0;
    pelem = get_tstamp_elem(ctx[1]);
    if (pelem == core::ptr::null_mut())
    return 0;
    pelem.timestamp = bpf_ktime_get_ns();
    pelem.lock = (__u64)ctx[0];
    pelem.flags = (__u32)ctx[1];
    if (aggr_mode == LOCK_AGGR_CGROUP)
    pelem.cgroup_id = get_current_cgroup_id();
    if (needs_callstack) {
    let mut i: u32 = 0;
    let mut id: u32 = 0;
    int owner_pid;
    u64 *buf;
    struct task_struct *task;
    struct owner_tracing_data *otdata;
    if (!lock_owner)
    goto skip_owner;
    task = get_lock_owner(pelem.lock, pelem.flags);
    if (!task)
    goto skip_owner;
    owner_pid = BPF_CORE_READ(task, pid);
    buf = bpf_map_lookup_elem(&stack_buf, &i);
    if (!buf)
    goto skip_owner;
    for (i = 0; i < max_stack; i++)
    buf[i] = 0x0;
    if (!bpf_task_from_pid)
    goto skip_owner;
    task = bpf_task_from_pid(owner_pid);
    if (!task)
    goto skip_owner;
    bpf_get_task_stack(task, buf, max_stack * sizeof(unsigned long), 0);
    bpf_task_release(task);
    otdata = bpf_map_lookup_elem(&owner_data, &pelem.lock);
    id = get_owner_stack_id(buf);
//
// Contention just happens, or corner case `lock` is owned by process not
// `owner_pid`. For the corner case we treat it as unexpected internal error and
// just ignore the precvious tracing record.
//
    if (!otdata || otdata.pid != owner_pid) {
    struct owner_tracing_data first = {
    .pid = owner_pid,
    .timestamp = pelem.timestamp,
    .count = 1,
    .stack_id = id,
    };
    bpf_map_update_elem(&owner_data, &pelem.lock, &first, BPF_ANY);
    }
// Contention is ongoing and new waiter joins
    else {
    __sync_fetch_and_add(&otdata.count, 1);
//
// The owner is the same, but stacktrace might be changed. In this case we
// store/update `owner_stat` based on current owner stack id.
//
    if (id != otdata.stack_id) {
    update_owner_stat(id, pelem.timestamp - otdata.timestamp,
    pelem.flags);
    otdata.timestamp = pelem.timestamp;
    otdata.stack_id = id;
    }
    }
    skip_owner:
    pelem.stack_id = bpf_get_stackid(ctx, &stacks,
    BPF_F_FAST_STACK_CMP | stack_skip);
    if (pelem.stack_id < 0)
    __sync_fetch_and_add(&stack_fail, 1);
    } else if (aggr_mode == LOCK_AGGR_TASK) {
    struct task_struct *task;
    if (lock_owner) {
    task = get_lock_owner(pelem.lock, pelem.flags);
// The flags is not used anymore.  Pass the owner pid.
    if (task)
    pelem.flags = BPF_CORE_READ(task, pid);
    else
    pelem.flags = -1U;
    } else {
    task = bpf_get_current_task_btf();
    }
    if (task) {
    if (update_task_data(task) < 0 && lock_owner)
    pelem.flags = -1U;
    }
    }
    return 0;
    }
    SEC("tp_btf/contention_end")
#[no_mangle]
pub unsafe extern "C" fn contention_end(ctx: *mut u64) -> c_int {
    int contention_end(u64 *ctx)
    {
    let mut pid: __u32 = 0, idx = 0;
    struct tstamp_data *pelem;
    let mut key: contention_key = {};
    struct contention_data *data;
    __u64 timestamp;
    __u64 duration;
    let mut need_delete: bool = false;
    if (!enabled)
    return 0;
//
// For spinlock and rwlock, it needs to get the timestamp for the
// per-cpu map.  However, contention_end does not have the flags
// so it cannot know whether it reads percpu or hash map.
//
// Try per-cpu map first and check if there's active contention.
// If it is, do not read hash map because it cannot go to sleeping
// locks before releasing the spinning locks.
//
    pelem = bpf_map_lookup_elem(&tstamp_cpu, &idx);
    if (pelem && pelem.lock) {
    if (pelem.lock != ctx[0])
    return 0;
    } else {
    pid = bpf_get_current_pid_tgid();
    pelem = bpf_map_lookup_elem(&tstamp, &pid);
    if (!pelem || pelem.lock != ctx[0])
    return 0;
    need_delete = true;
    }
    timestamp = bpf_ktime_get_ns();
    duration = timestamp - pelem.timestamp;
    if ((__s64)duration < 0) {
    __sync_fetch_and_add(&time_fail, 1);
    goto out;
    }
    if (needs_callstack && lock_owner) {
    struct owner_tracing_data *otdata = bpf_map_lookup_elem(&owner_data, &pelem.lock);
    if (!otdata)
    goto skip_owner;
// Update `owner_stat`
    update_owner_stat(otdata.stack_id, timestamp - otdata.timestamp, pelem.flags);
// No contention is occurring, delete `lock` entry in `owner_data`
    if (otdata.count <= 1)
    bpf_map_delete_elem(&owner_data, &pelem.lock);
//
// Contention is still ongoing, with a new owner (current task). `owner_data`
// should be updated accordingly.
//
    else {
    let mut i: u32 = 0;
    let mut ret: i32 = (s32)ctx[1];
    u64 *buf;
    otdata.timestamp = timestamp;
    __sync_fetch_and_add(&otdata.count, -1);
    buf = bpf_map_lookup_elem(&stack_buf, &i);
    if (!buf)
    goto skip_owner;
    for (i = 0; i < (u32)max_stack; i++)
    buf[i] = 0x0;
//
// `ret` has the return code of the lock function.
// If `ret` is negative, the current task terminates lock waiting without
// acquiring it. Owner is not changed, but we still need to update the owner
// stack.
//
    if (ret < 0) {
    let mut id: i32 = 0;
    struct task_struct *task;
    if (!bpf_task_from_pid)
    goto skip_owner;
    task = bpf_task_from_pid(otdata.pid);
    if (!task)
    goto skip_owner;
    bpf_get_task_stack(task, buf,
    max_stack * sizeof(unsigned long), 0);
    bpf_task_release(task);
    id = get_owner_stack_id(buf);
//
// If owner stack is changed, update owner stack id for this lock.
//
    if (id != otdata.stack_id)
    otdata.stack_id = id;
    }
//
// Otherwise, update tracing data with the current task, which is the new
// owner.
//
    else {
    otdata.pid = pid;
//
// We don't want to retrieve callstack here, since it is where the
// current task acquires the lock and provides no additional
// information. We simply assign -1 to invalidate it.
//
    otdata.stack_id = -1;
    }
    }
    }
    skip_owner:
    switch (aggr_mode) {
    case LOCK_AGGR_CALLER:
    key.stack_id = pelem.stack_id;
    break;
    case LOCK_AGGR_TASK:
    if (lock_owner)
    key.pid = pelem.flags;
    else {
    if (!need_delete)
    pid = bpf_get_current_pid_tgid();
    key.pid = pid;
    }
    if (needs_callstack)
    key.stack_id = pelem.stack_id;
    break;
    case LOCK_AGGR_ADDR:
    key.lock_addr_or_cgroup = pelem.lock;
    if (needs_callstack)
    key.stack_id = pelem.stack_id;
    break;
    case LOCK_AGGR_CGROUP:
    key.lock_addr_or_cgroup = pelem.cgroup_id;
    break;
    default:
// should not happen
    return 0;
    }
    data = bpf_map_lookup_elem(&lock_stat, &key);
    if (!data) {
    if (data_map_full) {
    __sync_fetch_and_add(&data_fail, 1);
    goto out;
    }
    struct contention_data first = {
    .total_time = duration,
    .max_time = duration,
    .min_time = duration,
    .count = 1,
    .flags = pelem.flags,
    };
    int err;
    if (aggr_mode == LOCK_AGGR_ADDR) {
    first.flags |= check_lock_type(pelem.lock,
    pelem.flags & LCB_F_TYPE_MASK);
// Check if it's from a slab object
    if (bpf_get_kmem_cache) {
    struct kmem_cache *s;
    struct slab_cache_data *d;
    s = bpf_get_kmem_cache(pelem.lock);
    if (s != core::ptr::null_mut()) {
//
// Save the ID of the slab cache in the flags
// (instead of full address) to reduce the
// space in the contention_data.
//
    d = bpf_map_lookup_elem(&slab_caches, &s);
    if (d != core::ptr::null_mut())
    first.flags |= d.id;
    }
    }
    }
    err = bpf_map_update_elem(&lock_stat, &key, &first, BPF_NOEXIST);
    if (err < 0) {
    if (err == -EEXIST) {
// it lost the race, try to get it again
    data = bpf_map_lookup_elem(&lock_stat, &key);
    if (data != core::ptr::null_mut())
    goto found;
    }
    if (err == -E2BIG)
    data_map_full = 1;
    __sync_fetch_and_add(&data_fail, 1);
    }
    goto out;
    }
    found:
    update_contention_data(data, duration, 1);
    out:
    if (lock_delay)
    check_lock_delay(pelem.lock);
    pelem.lock = 0;
    if (need_delete)
    bpf_map_delete_elem(&tstamp, &pid);
    return 0;
    }
    extern struct rq runqueues __ksym;
    const volatile __u64 contig_page_data_addr;
    const volatile __u64 node_data_addr;
    const volatile int nr_nodes;
    const volatile int sizeof_zone;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rq___old {
    pub lock: raw_spinlock_t,
    pub __attribute__((preserve_access_index)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rq___new {
    pub __lock: raw_spinlock_t,
    pub __attribute__((preserve_access_index)): },
#[no_mangle]
unsafe extern "C" fn collect_zone_lock() {
    static void collect_zone_lock(void)
    {
    pub zone_off: __u64 nr_zones,,
    pub lock_off: __u64 lock_addr,,
    pub LOCK_CLASS_ZONE_LOCK: __u32 lock_flag =,
    pub node_zones): zone_off = offsetof(struct pglist_data,,
    pub lock): lock_off = offsetof(struct zone,,
    if (contig_page_data_addr) {
    pub contig_page_data: *mut pglist_data,
    pub )(long)contig_page_data_addr: *mut contig_page_data = (void,
    pub nr_zones): nr_zones = BPF_CORE_READ(contig_page_data,,
    pub {: for (int i = 0; i < MAX_ZONES; i++),
    pub zone_addr: __u64,
    if (i >= nr_zones)
    pub zone_off: *mut *mut zone_addr = contig_page_data_addr + (sizeof_zone  i) +,
    pub lock_off: lock_addr = zone_addr +,
    pub BPF_ANY): bpf_map_update_elem(&lock_syms, &lock_addr, &lock_flag,,
    }
    } else if (nr_nodes > 0) {
    pub )(long)node_data_addr: *mut *mut *mut pglist_data node_data = (void,
    pub {: for (int i = 0; i < nr_nodes; i++),
    pub NULL: *mut *mut pglist_data pgdat =,
    pub err: c_int,
    pub &node_data[i]): err = bpf_core_read(&pgdat, sizeof(pgdat),,
    if (err < 0 || pgdat == core::ptr::null_mut())
    pub nr_zones): nr_zones = BPF_CORE_READ(pgdat,,
    pub {: for (int k = 0; k < MAX_ZONES; k++),
    pub zone_addr: __u64,
    if (k >= nr_zones)
    pub zone_off: *mut *mut *mut zone_addr = (__u64)(void )pgdat + (sizeof_zone  k) +,
    pub lock_off: lock_addr = zone_addr +,
    pub BPF_ANY): bpf_map_update_elem(&lock_syms, &lock_addr, &lock_flag,,
    }
    }
    }
    }
    SEC("raw_tp/bpf_test_finish")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: collect_lock_syms) -> c_int {
    int BPF_PROG(collect_lock_syms)
    {
    pub lock_off: __u64 lock_addr,,
    pub lock_flag: __u32,
    if (bpf_core_field_exists(struct rq___new, __lock))
    pub __lock): lock_off = offsetof(struct rq___new,,
    else
    pub lock): lock_off = offsetof(struct rq___old,,
    pub {: for (int i = 0; i < MAX_CPUS; i++),
    pub i): *mut *mut rq rq = bpf_per_cpu_ptr(&runqueues,,
    if (rq == core::ptr::null_mut())
    pub lock_off: *mut *mut lock_addr = (__u64)(void )rq +,
    pub LOCK_CLASS_RQLOCK: lock_flag =,
    pub BPF_ANY): bpf_map_update_elem(&lock_syms, &lock_addr, &lock_flag,,
    }
    pub 0: return,
    }
    SEC("raw_tp/bpf_test_finish")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: end_timestamp) -> c_int {
    int BPF_PROG(end_timestamp)
    {
    pub bpf_ktime_get_ns(): end_ts =,
    pub 0: return,
    }
//
// bpf_iter__kmem_cache added recently so old kernels don't have it in the
// vmlinux.h.  But we cannot add it here since it will cause a compiler error
// due to redefinition of the struct on later kernels.
//
// So it uses a CO-RE trick to access the member only if it has the type.
// This will support both old and new kernels without compiler errors.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter__kmem_cache___new {
    pub s: *mut kmem_cache,
    pub __attribute__((preserve_access_index)): },
    SEC("iter/kmem_cache")
#[no_mangle]
pub unsafe extern "C" fn slab_cache_iter(ctx: *mut c_void) -> c_int {
    int slab_cache_iter(void *ctx)
    {
    pub NULL: *mut *mut kmem_cache s =,
    pub d: slab_cache_data,
    pub nameptr: *const c_char,
    if (bpf_core_type_exists(struct bpf_iter__kmem_cache)) {
    pub ctx: *mut *mut bpf_iter__kmem_cache___new iter =,
    pub iter->s: s =,
    }
    if (s == core::ptr::null_mut())
    pub 0: return,
    pub s->name: nameptr =,
    pub nameptr): bpf_probe_read_kernel_str(d.name, sizeof(d.name),,
    pub LCB_F_SLAB_ID_SHIFT: d.id = ++slab_cache_id <<,
    if (d.id >= LCB_F_SLAB_ID_END)
    pub 0: return,
    pub BPF_NOEXIST): bpf_map_update_elem(&slab_caches, &s, &d,,
    pub 0: return,
    }
    pub BSD/GPL": char LICENSE[] SEC("license") = "Dual,
