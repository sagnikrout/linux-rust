//! Automatically rewritten from C to Rust
//! Source: drivers/xen/time.c
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
// Xen stolen ticks accounting.
//

// runstate info updated by Xen
    static DEFINE_PER_CPU(struct vcpu_runstate_info, xen_runstate);
    static DEFINE_PER_CPU(u64[4], old_runstate_time);
// return an consistent snapshot of 64-bit time/counter value
#[no_mangle]
unsafe extern "C" fn get64(p: *const u64) -> u64 {
    static u64 get64(const u64 *p)
    {
    u64 ret;
    if (BITS_PER_LONG < 64) {
    u32 *p32 = (u32 *)p;
    u32 h, l, h2;
//
// Read high then low, and then make sure high is
// still the same; this will only loop if low wraps
// and carries into high.
// XXX some clean way to make this endian-proof?
//
    do {
    h = READ_ONCE(p32[1]);
    l = READ_ONCE(p32[0]);
    h2 = READ_ONCE(p32[1]);
    } while(h2 != h);
    ret = (((u64)h) << 32) | l;
    } else
    ret = READ_ONCE(*p);
    return ret;
    }
    static void xen_get_runstate_snapshot_cpu_delta(
    struct vcpu_runstate_info *res, unsigned int cpu)
    {
    u64 state_time;
    struct vcpu_runstate_info *state;
    BUG_ON(preemptible());
    state = per_cpu_ptr(&xen_runstate, cpu);
    do {
    state_time = get64(&state.state_entry_time);
    rmb();	/* Hypervisor might update data. */
// res = __READ_ONCE(*state);
    rmb();	/* Hypervisor might update data. */
    } while (get64(&state.state_entry_time) != state_time ||
    (state_time & XEN_RUNSTATE_UPDATE));
    }
    static void xen_get_runstate_snapshot_cpu(struct vcpu_runstate_info *res,
    unsigned int cpu)
    {
    int i;
    xen_get_runstate_snapshot_cpu_delta(res, cpu);
    for (i = 0; i < 4; i++)
    res.time[i] += per_cpu(old_runstate_time, cpu)[i];
    }
#[no_mangle]
pub unsafe extern "C" fn xen_manage_runstate_time(action: c_int) {
    void xen_manage_runstate_time(int action)
    {
    static struct vcpu_runstate_info *runstate_delta;
    struct vcpu_runstate_info state;
    int cpu, i;
    switch (action) {
    case -1: /* backup runstate time before suspend */
    if (unlikely(runstate_delta))
    pr_warn_once("%s: memory leak as runstate_delta is not core::ptr::null_mut()\n",
    __func__);
    runstate_delta = kmalloc_objs(*runstate_delta,
    num_possible_cpus(), GFP_ATOMIC);
    if (unlikely(!runstate_delta)) {
    pr_warn("%s: failed to allocate runstate_delta\n",
    __func__);
    return;
    }
    for_each_possible_cpu(cpu) {
    xen_get_runstate_snapshot_cpu_delta(&state, cpu);
    memcpy(runstate_delta[cpu].time, state.time,
    sizeof(runstate_delta[cpu].time));
    }
    break;
    case 0: /* backup runstate time after resume */
    if (unlikely(!runstate_delta)) {
    pr_warn("%s: cannot accumulate runstate time as runstate_delta is core::ptr::null_mut()\n",
    __func__);
    return;
    }
    for_each_possible_cpu(cpu) {
    for (i = 0; i < 4; i++)
    per_cpu(old_runstate_time, cpu)[i] +=
    runstate_delta[cpu].time[i];
    }
    break;
    default: /* do not accumulate runstate time for checkpointing */
    break;
    }
    if (action != -1 && runstate_delta) {
    kfree(runstate_delta);
    runstate_delta = core::ptr::null_mut();
    }
    }
// return true when a vcpu could run but has no real cpu to run on
#[no_mangle]
pub unsafe extern "C" fn xen_vcpu_stolen(vcpu: c_int) -> bool {
    bool xen_vcpu_stolen(int vcpu)
    {
    return per_cpu(xen_runstate, vcpu).state == RUNSTATE_runnable;
    }
#[no_mangle]
pub unsafe extern "C" fn xen_steal_clock(cpu: c_int) -> u64 {
    u64 xen_steal_clock(int cpu)
    {
    struct vcpu_runstate_info state;
    xen_get_runstate_snapshot_cpu(&state, cpu);
    return state.time[RUNSTATE_runnable] + state.time[RUNSTATE_offline];
    }
#[no_mangle]
pub unsafe extern "C" fn xen_setup_runstate_info(cpu: c_int) {
    void xen_setup_runstate_info(int cpu)
    {
    struct vcpu_register_runstate_memory_area area;
    area.addr.v = &per_cpu(xen_runstate, cpu);
    if (HYPERVISOR_vcpu_op(VCPUOP_register_runstate_memory_area,
    xen_vcpu_nr(cpu), &area))
    BUG();
    }
#[no_mangle]
pub unsafe extern "C" fn xen_time_setup_guest() -> void __init {
    void __init xen_time_setup_guest(void)
    {
    bool xen_runstate_remote;
    xen_runstate_remote = !HYPERVISOR_vm_assist(VMASST_CMD_enable,
    VMASST_TYPE_runstate_update_flag);
    static_call_update(pv_steal_clock, xen_steal_clock);
    static_key_slow_inc(&paravirt_steal_enabled);
    if (xen_runstate_remote)
    static_key_slow_inc(&paravirt_steal_rq_enabled);
    }
