//! Automatically rewritten from C to Rust
//! Source: tools/sched_ext/scx_cpu0.bpf.c
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
// A CPU0 scheduler.
//
// This scheduler queues all tasks to a shared DSQ and only dispatches them on
// CPU0 in FIFO order. This is useful for testing bypass behavior when many
// tasks are concentrated on a single CPU. If the load balancer doesn't work,
// bypass mode can trigger task hangs or RCU stalls as the queue is long and
// there's only one CPU working on it.
//
// - Statistics tracking how many tasks are queued to local and CPU0 DSQs.
// - Termination notification for userspace.
//
// Copyright (c) 2025 Meta Platforms, Inc. and affiliates.
// Copyright (c) 2025 Tejun Heo <tj@kernel.org>
//

    char _license[] SEC("license") = "GPL";
    UEI_DEFINE(uei);
//
// We create a custom DSQ with ID 0 that we dispatch to and consume from on
// CPU0.
//
pub const DSQ_CPU0: c_int = 0;
    struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
    __uint(key_size, sizeof(u32));
    __uint(value_size, sizeof(u64));
    __uint(max_entries, 2);			/* [local, cpu0] */
    } stats SEC(".maps");
#[no_mangle]
unsafe extern "C" fn stat_inc(idx: u32) {
    static void stat_inc(u32 idx)
    {
    u64 *cnt_p = bpf_map_lookup_elem(&stats, &idx);
    if (cnt_p)
    (*cnt_p)++;
    }
#[no_mangle]
pub unsafe extern "C" fn BPF_STRUCT_OPS(_arg: cpu0_select_cpu, p: *mut task_struct, prev_cpu: i32, wake_flags: u64) -> i32 {
    s32 BPF_STRUCT_OPS(cpu0_select_cpu, struct task_struct *p, s32 prev_cpu, u64 wake_flags)
    {
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn BPF_STRUCT_OPS(_arg: cpu0_enqueue, p: *mut task_struct, enq_flags: u64) {
    void BPF_STRUCT_OPS(cpu0_enqueue, struct task_struct *p, u64 enq_flags)
    {
//
// select_cpu() always picks CPU0. If @p is not on CPU0, it can't run on
// CPU 0. Queue on whichever CPU it's currently only.
//
    if (scx_bpf_task_cpu(p) != 0) {
    stat_inc(0);	/* count local queueing */
    scx_bpf_dsq_insert(p, SCX_DSQ_LOCAL, SCX_SLICE_DFL, 0);
    return;
    }
    stat_inc(1);	/* count cpu0 queueing */
    scx_bpf_dsq_insert(p, DSQ_CPU0, SCX_SLICE_DFL, enq_flags);
    }
#[no_mangle]
pub unsafe extern "C" fn BPF_STRUCT_OPS(_arg: cpu0_dispatch, cpu: i32, prev: *mut task_struct) {
    void BPF_STRUCT_OPS(cpu0_dispatch, s32 cpu, struct task_struct *prev)
    {
    if (cpu == 0)
    scx_bpf_dsq_move_to_local(DSQ_CPU0, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn BPF_STRUCT_OPS_SLEEPABLE(_arg: cpu0_init) -> i32 {
    s32 BPF_STRUCT_OPS_SLEEPABLE(cpu0_init)
    {
    int ret;
    ret = scx_bpf_create_dsq(DSQ_CPU0, -1);
    if (ret) {
    scx_bpf_error("failed to create DSQ %d (%d)", DSQ_CPU0, ret);
    return ret;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn BPF_STRUCT_OPS(_arg: cpu0_exit, ei: *mut scx_exit_info) {
    void BPF_STRUCT_OPS(cpu0_exit, struct scx_exit_info *ei)
    {
    UEI_RECORD(uei, ei);
    }
    SCX_OPS_DEFINE(cpu0_ops,
    .select_cpu		= (void *)cpu0_select_cpu,
    .enqueue			= (void *)cpu0_enqueue,
    .dispatch		= (void *)cpu0_dispatch,
    .init			= (void *)cpu0_init,
    .exit			= (void *)cpu0_exit,
    .name			= "cpu0");
