//! Automatically rewritten from C to Rust
//! Source: tools/perf/util/bpf_skel/kwork_top.bpf.c
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
// Copyright (c) 2022, Huawei

//
// This should be in sync with "util/kwork.h"
//
    enum kwork_class_type {
    KWORK_CLASS_IRQ,
    KWORK_CLASS_SOFTIRQ,
    KWORK_CLASS_WORKQUEUE,
    KWORK_CLASS_SCHED,
    KWORK_CLASS_MAX,
    };
pub const MAX_ENTRIES: c_int = 102400;
pub const MAX_NR_CPUS: c_int = 4096;
pub const PF_KTHREAD: c_uint = 0x00200000;
pub const MAX_COMMAND_LEN: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct time_data {
    pub timestamp: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct work_data {
    pub runtime: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_data {
    pub tgid: __u32,
    pub is_kthread: __u32,
    pub comm: [c_char; MAX_COMMAND_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct work_key {
    pub type: __u32,
    pub pid: __u32,
    pub task_p: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_key {
    pub pid: __u32,
    pub cpu: __u32,
}

    struct {
    __uint(type, BPF_MAP_TYPE_TASK_STORAGE);
    __uint(map_flags, BPF_F_NO_PREALLOC);
    __type(key, int);
    __type(value, struct time_data);
    } kwork_top_task_time SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_HASH);
    __uint(key_size, sizeof(struct work_key));
    __uint(value_size, sizeof(struct time_data));
    __uint(max_entries, MAX_ENTRIES);
    } kwork_top_irq_time SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(key_size, sizeof(struct task_key));
    __uint(value_size, sizeof(struct task_data));
    __uint(max_entries, MAX_ENTRIES);
    } kwork_top_tasks SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_HASH);
    __uint(key_size, sizeof(struct work_key));
    __uint(value_size, sizeof(struct work_data));
    __uint(max_entries, MAX_ENTRIES);
    } kwork_top_works SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(key_size, sizeof(u32));
    __uint(value_size, sizeof(u8));
    __uint(max_entries, MAX_NR_CPUS);
    } kwork_top_cpu_filter SEC(".maps");
    let mut enabled: c_int = 0;
    let mut has_cpu_filter: volatile int = 0;
    let mut from_timestamp: __u64 = 0;
    let mut to_timestamp: __u64 = 0;
#[no_mangle]
unsafe extern "C" fn cpu_is_filtered(cpu: __u32) -> __always_inline int {
    static __always_inline int cpu_is_filtered(__u32 cpu)
    {
    __u8 *cpu_val;
    if (has_cpu_filter) {
    cpu_val = bpf_map_lookup_elem(&kwork_top_cpu_filter, &cpu);
    if (!cpu_val)
    return 1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn update_task_info(task: *mut task_struct, cpu: __u32) -> __always_inline void {
    static __always_inline void update_task_info(struct task_struct *task, __u32 cpu)
    {
    struct task_key key = {
    .pid = task.pid,
    .cpu = cpu,
    };
    if (!bpf_map_lookup_elem(&kwork_top_tasks, &key)) {
    struct task_data data = {
    .tgid = task.tgid,
    .is_kthread = task.flags & PF_KTHREAD ? 1 : 0,
    };
    BPF_CORE_READ_STR_INTO(&data.comm, task, comm);
    bpf_map_update_elem(&kwork_top_tasks, &key, &data, BPF_ANY);
    }
    }
#[no_mangle]
unsafe extern "C" fn update_work(key: *mut work_key, delta: __u64) -> __always_inline void {
    static __always_inline void update_work(struct work_key *key, __u64 delta)
    {
    struct work_data *data;
    data = bpf_map_lookup_elem(&kwork_top_works, key);
    if (data) {
    data.runtime += delta;
    } else {
    struct work_data new_data = {
    .runtime = delta,
    };
    bpf_map_update_elem(&kwork_top_works, key, &new_data, BPF_ANY);
    }
    }
#[no_mangle]
unsafe extern "C" fn on_sched_out(task: *mut task_struct, ts: __u64, cpu: __u32) {
    static void on_sched_out(struct task_struct *task, __u64 ts, __u32 cpu)
    {
    __u64 delta;
    struct time_data *pelem;
    pelem = bpf_task_storage_get(&kwork_top_task_time, task, core::ptr::null_mut(), 0);
    if (pelem)
    delta = ts - pelem.timestamp;
    else
    delta = ts - from_timestamp;
    struct work_key key = {
    .type = KWORK_CLASS_SCHED,
    .pid = task.pid,
    .task_p = (__u64)task,
    };
    update_work(&key, delta);
    update_task_info(task, cpu);
    }
#[no_mangle]
unsafe extern "C" fn on_sched_in(task: *mut task_struct, ts: __u64) {
    static void on_sched_in(struct task_struct *task, __u64 ts)
    {
    struct time_data *pelem;
    pelem = bpf_task_storage_get(&kwork_top_task_time, task, core::ptr::null_mut(),
    BPF_LOCAL_STORAGE_GET_F_CREATE);
    if (pelem)
    pelem.timestamp = ts;
    }
    SEC("tp_btf/sched_switch")
#[no_mangle]
pub unsafe extern "C" fn on_switch(ctx: *mut u64) -> c_int {
    int on_switch(u64 *ctx)
    {
    struct task_struct *prev, *next;
    prev = (struct task_struct *)ctx[1];
    next = (struct task_struct *)ctx[2];
    if (!enabled)
    return 0;
    let mut cpu: __u32 = bpf_get_smp_processor_id();
    if (cpu_is_filtered(cpu))
    return 0;
    let mut ts: __u64 = bpf_ktime_get_ns();
    on_sched_out(prev, ts, cpu);
    on_sched_in(next, ts);
    return 0;
    }
    SEC("tp_btf/irq_handler_entry")
#[no_mangle]
pub unsafe extern "C" fn on_irq_handler_entry(cxt: *mut u64) -> c_int {
    int on_irq_handler_entry(u64 *cxt)
    {
    struct task_struct *task;
    if (!enabled)
    return 0;
    let mut cpu: __u32 = bpf_get_smp_processor_id();
    if (cpu_is_filtered(cpu))
    return 0;
    let mut ts: __u64 = bpf_ktime_get_ns();
    task = (struct task_struct *)bpf_get_current_task();
    if (!task)
    return 0;
    struct work_key key = {
    .type = KWORK_CLASS_IRQ,
    .pid = BPF_CORE_READ(task, pid),
    .task_p = (__u64)task,
    };
    struct time_data data = {
    .timestamp = ts,
    };
    bpf_map_update_elem(&kwork_top_irq_time, &key, &data, BPF_ANY);
    return 0;
    }
    SEC("tp_btf/irq_handler_exit")
#[no_mangle]
pub unsafe extern "C" fn on_irq_handler_exit(cxt: *mut u64) -> c_int {
    int on_irq_handler_exit(u64 *cxt)
    {
    __u64 delta;
    struct task_struct *task;
    struct time_data *pelem;
    if (!enabled)
    return 0;
    let mut cpu: __u32 = bpf_get_smp_processor_id();
    if (cpu_is_filtered(cpu))
    return 0;
    let mut ts: __u64 = bpf_ktime_get_ns();
    task = (struct task_struct *)bpf_get_current_task();
    if (!task)
    return 0;
    struct work_key key = {
    .type = KWORK_CLASS_IRQ,
    .pid = BPF_CORE_READ(task, pid),
    .task_p = (__u64)task,
    };
    pelem = bpf_map_lookup_elem(&kwork_top_irq_time, &key);
    if (pelem && pelem.timestamp != 0)
    delta = ts - pelem.timestamp;
    else
    delta = ts - from_timestamp;
    update_work(&key, delta);
    return 0;
    }
    SEC("tp_btf/softirq_entry")
#[no_mangle]
pub unsafe extern "C" fn on_softirq_entry(cxt: *mut u64) -> c_int {
    int on_softirq_entry(u64 *cxt)
    {
    struct task_struct *task;
    if (!enabled)
    return 0;
    let mut cpu: __u32 = bpf_get_smp_processor_id();
    if (cpu_is_filtered(cpu))
    return 0;
    let mut ts: __u64 = bpf_ktime_get_ns();
    task = (struct task_struct *)bpf_get_current_task();
    if (!task)
    return 0;
    struct work_key key = {
    .type = KWORK_CLASS_SOFTIRQ,
    .pid = BPF_CORE_READ(task, pid),
    .task_p = (__u64)task,
    };
    struct time_data data = {
    .timestamp = ts,
    };
    bpf_map_update_elem(&kwork_top_irq_time, &key, &data, BPF_ANY);
    return 0;
    }
    SEC("tp_btf/softirq_exit")
#[no_mangle]
pub unsafe extern "C" fn on_softirq_exit(cxt: *mut u64) -> c_int {
    int on_softirq_exit(u64 *cxt)
    {
    __u64 delta;
    struct task_struct *task;
    struct time_data *pelem;
    if (!enabled)
    return 0;
    let mut cpu: __u32 = bpf_get_smp_processor_id();
    if (cpu_is_filtered(cpu))
    return 0;
    let mut ts: __u64 = bpf_ktime_get_ns();
    task = (struct task_struct *)bpf_get_current_task();
    if (!task)
    return 0;
    struct work_key key = {
    .type = KWORK_CLASS_SOFTIRQ,
    .pid = BPF_CORE_READ(task, pid),
    .task_p = (__u64)task,
    };
    pelem = bpf_map_lookup_elem(&kwork_top_irq_time, &key);
    if (pelem)
    delta = ts - pelem.timestamp;
    else
    delta = ts - from_timestamp;
    update_work(&key, delta);
    return 0;
    }
    char LICENSE[] SEC("license") = "Dual BSD/GPL";
