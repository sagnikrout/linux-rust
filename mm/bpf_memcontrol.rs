//! Automatically rewritten from C to Rust
//! Source: mm/bpf_memcontrol.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Memory Controller-related BPF kfuncs and auxiliary code
//
// Author: Roman Gushchin <roman.gushchin@linux.dev>
//

    __bpf_kfunc_start_defs();
//
// bpf_get_root_mem_cgroup - Returns a pointer to the root memory cgroup
//
// The function has KF_ACQUIRE semantics, even though the root memory
// cgroup is never destroyed after being created and doesn't require
// reference counting. And it's perfectly safe to pass it to
// bpf_put_mem_cgroup()
//
// Return: A pointer to the root memory cgroup.
//
    __bpf_kfunc struct mem_cgroup *bpf_get_root_mem_cgroup(void)
    {
    if (mem_cgroup_disabled())
    return core::ptr::null_mut();
// css_get() is not needed
    return root_mem_cgroup;
    }
//
// bpf_get_mem_cgroup - Get a reference to a memory cgroup
// @css: pointer to the css structure
//
// It's fine to pass a css which belongs to any cgroup controller,
// e.g. unified hierarchy's main css.
//
// Implements KF_ACQUIRE semantics.
//
// Return: A pointer to a mem_cgroup structure after bumping
// the corresponding css's reference counter.
//
    __bpf_kfunc struct mem_cgroup *
    bpf_get_mem_cgroup(struct cgroup_subsys_state *css)
    {
    struct mem_cgroup *memcg = core::ptr::null_mut();
    let mut rcu_unlock: bool = false;
    if (mem_cgroup_disabled() || !root_mem_cgroup)
    return core::ptr::null_mut();
    if (root_mem_cgroup.css.ss != css.ss) {
    struct cgroup *cgroup = css.cgroup;
    let mut ssid: c_int = root_mem_cgroup.css.ss.id;
    rcu_read_lock();
    rcu_unlock = true;
    css = rcu_dereference_raw(cgroup.subsys[ssid]);
    }
    if (css && css_tryget(css))
    memcg = container_of(css, struct mem_cgroup, css);
    if (rcu_unlock)
    rcu_read_unlock();
    return memcg;
    }
//
// bpf_put_mem_cgroup - Put a reference to a memory cgroup
// @memcg: memory cgroup to release
//
// Releases a previously acquired memcg reference.
// Implements KF_RELEASE semantics.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_put_mem_cgroup(memcg: *mut mem_cgroup) -> __bpf_kfunc void {
    __bpf_kfunc void bpf_put_mem_cgroup(struct mem_cgroup *memcg)
    {
    css_put(&memcg.css);
    }
//
// bpf_mem_cgroup_vm_events - Read memory cgroup's vm event counter
// @memcg: memory cgroup
// @event: event id
//
// Allows to read memory cgroup event counters.
//
// Return: The current value of the corresponding events counter.
//
    __bpf_kfunc unsigned long bpf_mem_cgroup_vm_events(struct mem_cgroup *memcg,
    enum vm_event_item event)
    {
    if (unlikely(!memcg_vm_event_item_valid(event)))
    return (unsigned long)-1;
    return memcg_events(memcg, event);
    }
//
// bpf_mem_cgroup_usage - Read memory cgroup's usage
// @memcg: memory cgroup
//
// Please, note that the root memory cgroup it special and is exempt
// from the memory accounting. The returned value is a sum of sub-cgroup's
// usages and it not reflecting the size of the root memory cgroup itself.
// If you need to get an approximation, you can use root level statistics:
// e.g. NR_FILE_PAGES + NR_ANON_MAPPED.
//
// Return: The current memory cgroup size in bytes.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_mem_cgroup_usage(memcg: *mut mem_cgroup) -> __bpf_kfunc unsigned long {
    __bpf_kfunc unsigned long bpf_mem_cgroup_usage(struct mem_cgroup *memcg)
    {
    return page_counter_read(&memcg.memory) * PAGE_SIZE;
    }
//
// bpf_mem_cgroup_memory_events - Read memory cgroup's memory event value
// @memcg: memory cgroup
// @event: memory event id
//
// Return: The current value of the memory event counter.
//
    __bpf_kfunc unsigned long bpf_mem_cgroup_memory_events(struct mem_cgroup *memcg,
    enum memcg_memory_event event)
    {
    if (unlikely(event >= MEMCG_NR_MEMORY_EVENTS))
    return (unsigned long)-1;
    return atomic_long_read(&memcg.memory_events[event]);
    }
//
// bpf_mem_cgroup_page_state - Read memory cgroup's page state counter
// @memcg: memory cgroup
// @idx: counter idx
//
// Allows to read memory cgroup statistics. The output is in bytes.
//
// Return: The value of the page state counter in bytes.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_mem_cgroup_page_state(memcg: *mut mem_cgroup, idx: c_int) -> __bpf_kfunc unsigned long {
    __bpf_kfunc unsigned long bpf_mem_cgroup_page_state(struct mem_cgroup *memcg, int idx)
    {
    if (unlikely(!memcg_stat_item_valid(idx)))
    return (unsigned long)-1;
    return memcg_page_state_output(memcg, idx);
    }
//
// bpf_mem_cgroup_flush_stats - Flush memory cgroup's statistics
// @memcg: memory cgroup
//
// Propagate memory cgroup's statistics up the cgroup tree.
//
#[no_mangle]
pub unsafe extern "C" fn bpf_mem_cgroup_flush_stats(memcg: *mut mem_cgroup) -> __bpf_kfunc void {
    __bpf_kfunc void bpf_mem_cgroup_flush_stats(struct mem_cgroup *memcg)
    {
    mem_cgroup_flush_stats(memcg);
    }
    __bpf_kfunc_end_defs();
    BTF_KFUNCS_START(bpf_memcontrol_kfuncs)
    BTF_ID_FLAGS(func, bpf_get_root_mem_cgroup, KF_ACQUIRE | KF_RET_NULL)
    BTF_ID_FLAGS(func, bpf_get_mem_cgroup, KF_ACQUIRE | KF_RET_NULL | KF_RCU)
    BTF_ID_FLAGS(func, bpf_put_mem_cgroup, KF_RELEASE)
    BTF_ID_FLAGS(func, bpf_mem_cgroup_vm_events)
    BTF_ID_FLAGS(func, bpf_mem_cgroup_memory_events)
    BTF_ID_FLAGS(func, bpf_mem_cgroup_usage)
    BTF_ID_FLAGS(func, bpf_mem_cgroup_page_state)
    BTF_ID_FLAGS(func, bpf_mem_cgroup_flush_stats, KF_SLEEPABLE)
    BTF_KFUNCS_END(bpf_memcontrol_kfuncs)
    static const struct btf_kfunc_id_set bpf_memcontrol_kfunc_set = {
    .owner          = THIS_MODULE,
    .set            = &bpf_memcontrol_kfuncs,
    };
#[no_mangle]
unsafe extern "C" fn bpf_memcontrol_init() -> int __init {
    static int __init bpf_memcontrol_init(void)
    {
    int err;
    err = register_btf_kfunc_id_set(BPF_PROG_TYPE_UNSPEC,
    &bpf_memcontrol_kfunc_set);
    if (err)
    pr_warn("error while registering bpf memcontrol kfuncs: %d", err);
    return err;
    }
    late_initcall(bpf_memcontrol_init);
