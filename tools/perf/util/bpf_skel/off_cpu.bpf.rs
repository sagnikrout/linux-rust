//! Automatically rewritten from C to Rust
//! Source: tools/perf/util/bpf_skel/off_cpu.bpf.c
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

// task->flags for off-cpu analysis
pub const PF_KTHREAD: c_uint = 0x00200000  /* I am a kernel thread */;
// task->state for off-cpu analysis
pub const TASK_INTERRUPTIBLE: c_uint = 0x0001;
pub const TASK_UNINTERRUPTIBLE: c_uint = 0x0002;
// create a new thread
pub const CLONE_THREAD: c_uint = 0x10000;
pub const MAX_STACKS: c_int = 32;
pub const MAX_ENTRIES: c_int = 102400;
pub const MAX_CPUS: c_int = 4096;
pub const MAX_OFFCPU_LEN: c_int = 37;
// We have a 'struct stack' in vmlinux.h when building with GEN_VMLINUX_H=1
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __stack {
    pub array: [u64; MAX_STACKS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstamp_data {
    pub stack_id: __u32,
    pub state: __u32,
    pub timestamp: __u64,
    pub stack: __stack,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct offcpu_key {
    pub pid: __u32,
    pub tgid: __u32,
    pub stack_id: __u32,
    pub state: __u32,
    pub cgroup_id: __u64,
}

    struct {
    __uint(type, BPF_MAP_TYPE_STACK_TRACE);
    __uint(key_size, sizeof(__u32));
    __uint(value_size, MAX_STACKS * sizeof(__u64));
    __uint(max_entries, MAX_ENTRIES);
    } stacks SEC(".maps");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct offcpu_data {
    pub array: [u64; MAX_OFFCPU_LEN],
}

    struct {
    __uint(type, BPF_MAP_TYPE_PERF_EVENT_ARRAY);
    __uint(key_size, sizeof(int));
    __uint(value_size, sizeof(int));
    __uint(max_entries, MAX_CPUS);
    } offcpu_output SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
    __uint(key_size, sizeof(__u32));
    __uint(value_size, sizeof(struct offcpu_data));
    __uint(max_entries, 1);
    } offcpu_payload SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_TASK_STORAGE);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    __type(key, int);
    __type(value, struct tstamp_data);
    } tstamp SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(key_size, sizeof(struct offcpu_key));
    __uint(value_size, sizeof(__u64));
    __uint(max_entries, MAX_ENTRIES);
    } off_cpu SEC(".maps");
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
    __uint(key_size, sizeof(__u64));
    __uint(value_size, sizeof(__u8));
    __uint(max_entries, 1);
    } cgroup_filter SEC(".maps");
// new kernel task_struct definition
#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_struct___new {
    pub __state: c_long,
    pub __attribute__((preserve_access_index)): },
// old kernel task_struct definition
#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_struct___old {
    pub state: c_long,
    pub __attribute__((preserve_access_index)): },
    pub 0: int enabled =,
    pub 0: volatile int has_cpu =,
    pub 0: volatile int has_task =,
    pub 0: volatile int has_cgroup =,
    pub 0: volatile int uses_tgid =,
    pub false: volatile bool has_prev_state =,
    pub false: volatile bool needs_cgroup =,
    pub false: volatile bool uses_cgroup_v1 =,
    pub -1: int perf_subsys_id =,
    pub offcpu_thresh_ns: __u64,
//
// Old kernel used to call it task_struct->state and now it's '__state'.
// Use BPF CO-RE "ignored suffix rule" to deal with it like below:
//
// https://nakryiko.com/posts/bpf-core-reference-guide/#handling-incompatible-field-and-type-changes
//
#[no_mangle]
pub unsafe extern "C" fn get_task_state(t: *mut task_struct) -> c_int {
    static inline int get_task_state(struct task_struct *t)
    {
// recast pointer to capture new type for compiler
    pub )t: *mut *mut task_struct___new t_new = (void,
    if (bpf_core_field_exists(t_new.__state)) {
    pub __state): return BPF_CORE_READ(t_new,,
    } else {
// recast pointer to capture old type for compiler
    pub )t: *mut *mut task_struct___old t_old = (void,
    pub state): return BPF_CORE_READ(t_old,,
    }
    }
#[no_mangle]
pub unsafe extern "C" fn get_cgroup_id(t: *mut task_struct) -> __u64 {
    static inline __u64 get_cgroup_id(struct task_struct *t)
    {
    pub cgrp: *mut cgroup,
    if (!uses_cgroup_v1)
    pub id): return BPF_CORE_READ(t, cgroups, dfl_cgrp, kn,,
    if (perf_subsys_id == -1) {

    perf_subsys_id = bpf_core_enum_value(enum cgroup_subsys_id,

    pub perf_event_cgrp_id: perf_subsys_id =,

    }
    pub cgroup): cgrp = BPF_CORE_READ(t, cgroups, subsys[perf_subsys_id],,
    pub id): return BPF_CORE_READ(cgrp, kn,,
    }
#[no_mangle]
pub unsafe extern "C" fn can_record(t: *mut task_struct, state: c_int) -> c_int {
    static inline int can_record(struct task_struct *t, int state)
    {
// kernel threads don't have user stack
    if (t.flags & PF_KTHREAD)
    pub 0: return,
    if (state != TASK_INTERRUPTIBLE &&
    state != TASK_UNINTERRUPTIBLE)
    pub 0: return,
    if (has_cpu) {
    pub bpf_get_smp_processor_id(): __u32 cpu =,
    pub ok: *mut __u8,
    pub &cpu): ok = bpf_map_lookup_elem(&cpu_filter,,
    if (!ok)
    pub 0: return,
    }
    if (has_task) {
    pub ok: *mut __u8,
    pub pid: __u32,
    if (uses_tgid)
    pub t->tgid: pid =,
    else
    pub t->pid: pid =,
    pub &pid): ok = bpf_map_lookup_elem(&task_filter,,
    if (!ok)
    pub 0: return,
    }
    if (has_cgroup) {
    pub ok: *mut __u8,
    pub get_cgroup_id(t): __u64 cgrp_id =,
    pub &cgrp_id): ok = bpf_map_lookup_elem(&cgroup_filter,,
    if (!ok)
    pub 0: return,
    }
    pub 1: return,
    }
#[no_mangle]
pub unsafe extern "C" fn copy_stack(from: *mut __stack, to: *mut offcpu_data, n: c_int) -> c_int {
    static inline int copy_stack(struct __stack *from, struct offcpu_data *to, int n)
    {
    pub 0: int len =,
    pub ++len): for (int i = 0; i < MAX_STACKS && from->array[i]; ++i,,
    pub from->array[i]: to->array[n + 2 + i] =,
    pub len: return,
    }
//
// off_cpu_dump - dump off-cpu samples to ring buffer
// @data: payload for dumping off-cpu samples
// @key: off-cpu data
// @stack: stack trace of the task before being scheduled out
//
// If the threshold of off-cpu time is reached, acquire tid, period, callchain, and cgroup id
// information of the task, and dump it as a raw sample to perf ring buffer
//
    static int off_cpu_dump(void *ctx, struct offcpu_data *data, struct offcpu_key *key,
    struct __stack *stack, __u64 delta)
    {
    pub 0: int n = 0, len =,
    pub key->pid: data->array[n++] = (u64)key->tgid << 32 |,
    pub delta: data->array[n++] =,
// data->array[n] is callchain->nr (updated later)
    pub PERF_CONTEXT_USER: data->array[n + 1] =,
    pub 0: data->array[n + 2] =,
    pub n): len = copy_stack(stack, data,,
// update length of callchain
    pub 1: data->array[n] = len +,
    pub 2: n += len +,
    pub key->cgroup_id: data->array[n++] =,
    pub sizeof(u64)): *mut *mut return bpf_perf_event_output(ctx, &offcpu_output, BPF_F_CURRENT_CPU, data, n,
    }
    static int off_cpu_stat(u64 *ctx, struct task_struct *prev,
    struct task_struct *next, int state)
    {
    pub ts: __u64,
    pub stack_id: __u32,
    pub pelem: *mut tstamp_data,
    pub bpf_ktime_get_ns(): ts =,
    if (!can_record(prev, state))
    pub next: goto,
    stack_id = bpf_get_stackid(ctx, &stacks,
    pub BPF_F_USER_STACK): BPF_F_FAST_STACK_CMP |,
    pelem = bpf_task_storage_get(&tstamp, prev, core::ptr::null_mut(),
    if (!pelem)
    pub next: goto,
    pub ts: pelem->timestamp =,
    pub state: pelem->state =,
    pub stack_id: pelem->stack_id =,
//
// If stacks are successfully collected by bpf_get_stackid(), collect them once more
// in task_storage for direct off-cpu sample dumping
//
    if (stack_id > 0 && bpf_get_stack(ctx, &pelem.stack, MAX_STACKS * sizeof(u64), BPF_F_USER_STACK)) {
//
// This empty if block is used to avoid 'result unused warning' from bpf_get_stack().
// If the collection fails, continue with the logic for the next task.
//
    }
    next:
    pub 0): pelem = bpf_task_storage_get(&tstamp, next, NULL,,
    if (pelem && pelem.timestamp) {
    struct offcpu_key key = {
    .pid = next.pid,
    .tgid = next.tgid,
    .stack_id = pelem.stack_id,
    .state = pelem.state,
    .cgroup_id = needs_cgroup ? get_cgroup_id(next) : 0,
}

    let mut delta: __u64 = ts - pelem.timestamp;
    __u64 *total;
    if (delta >= offcpu_thresh_ns) {
    let mut zero: c_int = 0;
    struct offcpu_data *data = bpf_map_lookup_elem(&offcpu_payload, &zero);
    if (data)
    off_cpu_dump(ctx, data, &key, &pelem.stack, delta);
    } else {
    total = bpf_map_lookup_elem(&off_cpu, &key);
    if (total)
// total += delta;
    else
    bpf_map_update_elem(&off_cpu, &key, &delta, BPF_ANY);
    }
// prevent to reuse the timestamp later
    pelem.timestamp = 0;
    }
    return 0;
    }
    SEC("tp_btf/task_newtask")
#[no_mangle]
pub unsafe extern "C" fn on_newtask(ctx: *mut u64) -> c_int {
    int on_newtask(u64 *ctx)
    {
    struct task_struct *task;
    u64 clone_flags;
    u32 pid;
    let mut val: u8 = 1;
    if (!uses_tgid)
    return 0;
    task = (struct task_struct *)bpf_get_current_task();
    pid = BPF_CORE_READ(task, tgid);
    if (!bpf_map_lookup_elem(&task_filter, &pid))
    return 0;
    task = (struct task_struct *)ctx[0];
    clone_flags = ctx[1];
    pid = task.tgid;
    if (!(clone_flags & CLONE_THREAD))
    bpf_map_update_elem(&task_filter, &pid, &val, BPF_NOEXIST);
    return 0;
    }
    SEC("tp_btf/sched_switch")
#[no_mangle]
pub unsafe extern "C" fn on_switch(ctx: *mut u64) -> c_int {
    int on_switch(u64 *ctx)
    {
    struct task_struct *prev, *next;
    int prev_state;
    if (!enabled)
    return 0;
    prev = (struct task_struct *)ctx[1];
    next = (struct task_struct *)ctx[2];
    if (has_prev_state)
    prev_state = (int)ctx[3];
    else
    prev_state = get_task_state(prev);
    return off_cpu_stat(ctx, prev, next, prev_state & 0xff);
    }
    char LICENSE[] SEC("license") = "Dual BSD/GPL";
