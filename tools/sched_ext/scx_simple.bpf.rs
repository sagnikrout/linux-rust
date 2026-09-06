//! Automatically rewritten from C to Rust
//! Source: tools/sched_ext/scx_simple.bpf.c
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
// A simple scheduler.
//
// By default, it operates as a simple global weighted vtime scheduler and can
// be switched to FIFO scheduling. It also demonstrates the following niceties.
//
// - Statistics tracking how many tasks are queued to local and global dsq's.
// - Termination notification for userspace.
//
// While very simple, this scheduler should work reasonably well on CPUs with a
// uniform L3 cache topology. While preemption is not implemented, the fact that
// the scheduling queue is shared across all CPUs means that whatever is at the
// front of the queue is likely to be executed fairly quickly given enough
// number of CPUs. The FIFO scheduling mode may be beneficial to some workloads
// but comes with the usual problems with FIFO scheduling where saturating
// threads can easily drown out interactive ones.
//
// Copyright (c) 2022 Meta Platforms, Inc. and affiliates.
// Copyright (c) 2022 Tejun Heo <tj@kernel.org>
// Copyright (c) 2022 David Vernet <dvernet@meta.com>
//

    char _license[] SEC("license") = "GPL";
    const volatile bool fifo_sched;
    static u64 vtime_now;
    UEI_DEFINE(uei);
//
// Built-in DSQs such as SCX_DSQ_GLOBAL cannot be used as priority queues
// (meaning, cannot be dispatched to with scx_bpf_dsq_insert_vtime()). We
// therefore create a separate DSQ with ID 0 that we dispatch to and consume
// from. If scx_simple only supported global FIFO scheduling, then we could just
// use SCX_DSQ_GLOBAL.
//
pub const SHARED_DSQ: c_int = 0;
    struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
    __uint(key_size, sizeof(u32));
    __uint(value_size, sizeof(u64));
    __uint(max_entries, 2);			/* [local, global] */
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
pub unsafe extern "C" fn BPF_STRUCT_OPS(_arg: simple_select_cpu, p: *mut task_struct, prev_cpu: i32, wake_flags: u64) -> i32 {
    s32 BPF_STRUCT_OPS(simple_select_cpu, struct task_struct *p, s32 prev_cpu, u64 wake_flags)
    {
    let mut is_idle: bool = false;
    s32 cpu;
    cpu = scx_bpf_select_cpu_dfl(p, prev_cpu, wake_flags, &is_idle);
    if (is_idle) {
    stat_inc(0);	/* count local queueing */
    scx_bpf_dsq_insert(p, SCX_DSQ_LOCAL, SCX_SLICE_DFL, 0);
    }
    return cpu;
    }
#[no_mangle]
pub unsafe extern "C" fn BPF_STRUCT_OPS(_arg: simple_enqueue, p: *mut task_struct, enq_flags: u64) {
    void BPF_STRUCT_OPS(simple_enqueue, struct task_struct *p, u64 enq_flags)
    {
    stat_inc(1);	/* count global queueing */
    if (fifo_sched) {
    scx_bpf_dsq_insert(p, SHARED_DSQ, SCX_SLICE_DFL, enq_flags);
    } else {
    let mut vtime: u64 = p.scx.dsq_vtime;
//
// Limit the amount of budget that an idling task can accumulate
// to one slice.
//
    if (time_before(vtime, vtime_now - SCX_SLICE_DFL))
    vtime = vtime_now - SCX_SLICE_DFL;
    scx_bpf_dsq_insert_vtime(p, SHARED_DSQ, SCX_SLICE_DFL, vtime,
    enq_flags);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn BPF_STRUCT_OPS(_arg: simple_dispatch, cpu: i32, prev: *mut task_struct) {
    void BPF_STRUCT_OPS(simple_dispatch, s32 cpu, struct task_struct *prev)
    {
    scx_bpf_dsq_move_to_local(SHARED_DSQ, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn BPF_STRUCT_OPS(_arg: simple_running, p: *mut task_struct) {
    void BPF_STRUCT_OPS(simple_running, struct task_struct *p)
    {
    if (fifo_sched)
    return;
//
// Global vtime always progresses forward as tasks start executing. The
// test and update can be performed concurrently from multiple CPUs and
// thus racy. Any error should be contained and temporary. Let's just
// live with it.
//
    if (time_before(vtime_now, p.scx.dsq_vtime))
    vtime_now = p.scx.dsq_vtime;
    }
#[no_mangle]
pub unsafe extern "C" fn BPF_STRUCT_OPS(_arg: simple_stopping, p: *mut task_struct, runnable: bool) {
    void BPF_STRUCT_OPS(simple_stopping, struct task_struct *p, bool runnable)
    {
    if (fifo_sched)
    return;
//
// Scale the execution time by the inverse of the weight and charge.
//
// Note that the default yield implementation yields by setting
// @p->scx.slice to zero and the following would treat the yielding task
// as if it has consumed all its slice. If this penalizes yielding tasks
// too much, determine the execution time by taking explicit timestamps
// instead of depending on @p->scx.slice.
//
    let mut delta: u64 = scale_by_task_weight_inverse(p, SCX_SLICE_DFL - p.scx.slice);
    scx_bpf_task_set_dsq_vtime(p, p.scx.dsq_vtime + delta);
    }
#[no_mangle]
pub unsafe extern "C" fn BPF_STRUCT_OPS(_arg: simple_enable, p: *mut task_struct) {
    void BPF_STRUCT_OPS(simple_enable, struct task_struct *p)
    {
    scx_bpf_task_set_dsq_vtime(p, vtime_now);
    }
#[no_mangle]
pub unsafe extern "C" fn BPF_STRUCT_OPS_SLEEPABLE(_arg: simple_init) -> i32 {
    s32 BPF_STRUCT_OPS_SLEEPABLE(simple_init)
    {
    int ret;
    ret = scx_bpf_create_dsq(SHARED_DSQ, -1);
    if (ret) {
    scx_bpf_error("failed to create DSQ %d (%d)", SHARED_DSQ, ret);
    return ret;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn BPF_STRUCT_OPS(_arg: simple_exit, ei: *mut scx_exit_info) {
    void BPF_STRUCT_OPS(simple_exit, struct scx_exit_info *ei)
    {
    UEI_RECORD(uei, ei);
    }
    SCX_OPS_DEFINE(simple_ops,
    .select_cpu		= (void *)simple_select_cpu,
    .enqueue			= (void *)simple_enqueue,
    .dispatch		= (void *)simple_dispatch,
    .running			= (void *)simple_running,
    .stopping		= (void *)simple_stopping,
    .enable			= (void *)simple_enable,
    .init			= (void *)simple_init,
    .exit			= (void *)simple_exit,
    .name			= "simple");
