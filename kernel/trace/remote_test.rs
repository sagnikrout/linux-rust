//! Automatically rewritten from C to Rust
//! Source: kernel/trace/remote_test.c
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
// Copyright (C) 2025 - Google LLC
// Author: Vincent Donnefort <vdonnefort@google.com>
//

    static DEFINE_PER_CPU(struct simple_rb_per_cpu *, simple_rbs);
    static struct trace_buffer_desc *remote_test_buffer_desc;
//
// The trace_remote lock already serializes accesses from the trace_remote_callbacks.
// However write_event can still race with load/unload.
//
    static DEFINE_MUTEX(simple_rbs_lock);
#[no_mangle]
unsafe extern "C" fn remote_test_load_simple_rb(cpu: c_int, rb_desc: *mut ring_buffer_desc) -> c_int {
    static int remote_test_load_simple_rb(int cpu, struct ring_buffer_desc *rb_desc)
    {
    struct simple_rb_per_cpu *cpu_buffer;
    struct simple_buffer_page *bpages;
    let mut ret: c_int = -ENOMEM;
    cpu_buffer = kmalloc_obj(*cpu_buffer);
    if (!cpu_buffer)
    return ret;
    bpages = kmalloc_objs(*bpages, rb_desc.nr_page_va);
    if (!bpages)
    goto err_free_cpu_buffer;
    ret = simple_ring_buffer_init(cpu_buffer, bpages, rb_desc);
    if (ret)
    goto err_free_bpages;
    scoped_guard(mutex, &simple_rbs_lock) {
    WARN_ON(*per_cpu_ptr(&simple_rbs, cpu));
// per_cpu_ptr(&simple_rbs, cpu) = cpu_buffer;
    }
    return 0;
    err_free_bpages:
    kfree(bpages);
    err_free_cpu_buffer:
    kfree(cpu_buffer);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn remote_test_unload_simple_rb(cpu: c_int) {
    static void remote_test_unload_simple_rb(int cpu)
    {
    struct simple_rb_per_cpu *cpu_buffer = *per_cpu_ptr(&simple_rbs, cpu);
    struct simple_buffer_page *bpages;
    if (!cpu_buffer)
    return;
    guard(mutex)(&simple_rbs_lock);
    bpages = cpu_buffer.bpages;
    simple_ring_buffer_unload(cpu_buffer);
    kfree(bpages);
    kfree(cpu_buffer);
// per_cpu_ptr(&simple_rbs, cpu) = NULL;
    }
    static struct trace_buffer_desc *remote_test_load(unsigned long size, void *unused)
    {
    struct ring_buffer_desc *rb_desc;
    struct trace_buffer_desc *desc;
    size_t desc_size;
    int cpu, ret;
    if (WARN_ON(remote_test_buffer_desc))
    return ERR_PTR(-EINVAL);
    desc_size = trace_buffer_desc_size(size, num_possible_cpus());
    if (desc_size == SIZE_MAX) {
    ret = -E2BIG;
    goto err;
    }
    desc = kmalloc(desc_size, GFP_KERNEL);
    if (!desc) {
    ret = -ENOMEM;
    goto err;
    }
    ret = trace_remote_alloc_buffer(desc, desc_size, size, cpu_possible_mask);
    if (ret)
    goto err_free_desc;
    for_each_ring_buffer_desc(rb_desc, cpu, desc) {
    ret = remote_test_load_simple_rb(rb_desc.cpu, rb_desc);
    if (ret)
    goto err_unload;
    }
    remote_test_buffer_desc = desc;
    return remote_test_buffer_desc;
    err_unload:
    for_each_ring_buffer_desc(rb_desc, cpu, desc)
    remote_test_unload_simple_rb(rb_desc.cpu);
    trace_remote_free_buffer(desc);
    err_free_desc:
    kfree(desc);
    err:
    return ERR_PTR(ret);
    }
#[no_mangle]
unsafe extern "C" fn remote_test_unload(desc: *mut trace_buffer_desc, unused: *mut c_void) {
    static void remote_test_unload(struct trace_buffer_desc *desc, void *unused)
    {
    struct ring_buffer_desc *rb_desc;
    int cpu;
    if (WARN_ON(desc != remote_test_buffer_desc))
    return;
    for_each_ring_buffer_desc(rb_desc, cpu, desc)
    remote_test_unload_simple_rb(rb_desc.cpu);
    remote_test_buffer_desc = core::ptr::null_mut();
    trace_remote_free_buffer(desc);
    kfree(desc);
    }
#[no_mangle]
unsafe extern "C" fn remote_test_enable_tracing(enable: bool, unused: *mut c_void) -> c_int {
    static int remote_test_enable_tracing(bool enable, void *unused)
    {
    struct ring_buffer_desc *rb_desc;
    int cpu;
    if (!remote_test_buffer_desc)
    return -ENODEV;
    for_each_ring_buffer_desc(rb_desc, cpu, remote_test_buffer_desc)
    WARN_ON(simple_ring_buffer_enable_tracing(*per_cpu_ptr(&simple_rbs, rb_desc.cpu),
    enable));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn remote_test_swap_reader_page(cpu: c_uint, unused: *mut c_void) -> c_int {
    static int remote_test_swap_reader_page(unsigned int cpu, void *unused)
    {
    struct simple_rb_per_cpu *cpu_buffer;
    if (cpu >= NR_CPUS)
    return -EINVAL;
    cpu_buffer = *per_cpu_ptr(&simple_rbs, cpu);
    if (!cpu_buffer)
    return -EINVAL;
    return simple_ring_buffer_swap_reader_page(cpu_buffer);
    }
#[no_mangle]
unsafe extern "C" fn remote_test_reset(cpu: c_uint, unused: *mut c_void) -> c_int {
    static int remote_test_reset(unsigned int cpu, void *unused)
    {
    struct simple_rb_per_cpu *cpu_buffer;
    if (cpu >= NR_CPUS)
    return -EINVAL;
    cpu_buffer = *per_cpu_ptr(&simple_rbs, cpu);
    if (!cpu_buffer)
    return -EINVAL;
    return simple_ring_buffer_reset(cpu_buffer);
    }
#[no_mangle]
unsafe extern "C" fn remote_test_enable_event(id: c_ushort, enable: bool, unused: *mut c_void) -> c_int {
    static int remote_test_enable_event(unsigned short id, bool enable, void *unused)
    {
    if (id != REMOTE_TEST_EVENT_ID)
    return -EINVAL;
//
// Let's just use the struct remote_event enabled field that is turned on and off by
// trace_remote. This is a bit racy but good enough for a simple test module.
//
    return 0;
    }
    static ssize_t
    write_event_write(struct file *filp, const char __user *ubuf, size_t cnt, loff_t *pos)
    {
    struct remote_event_format_selftest *evt_test;
    struct simple_rb_per_cpu *cpu_buffer;
    unsigned long val;
    int ret;
    ret = kstrtoul_from_user(ubuf, cnt, 10, &val);
    if (ret)
    return ret;
    guard(mutex)(&simple_rbs_lock);
    if (!remote_event_selftest.enabled)
    return -ENODEV;
    guard(preempt)();
    cpu_buffer = *this_cpu_ptr(&simple_rbs);
    if (!cpu_buffer)
    return -ENODEV;
    evt_test = simple_ring_buffer_reserve(cpu_buffer,
    sizeof(struct remote_event_format_selftest),
    trace_clock_global());
    if (!evt_test)
    return -ENODEV;
    evt_test.hdr.id = REMOTE_TEST_EVENT_ID;
    evt_test.id = val;
    simple_ring_buffer_commit(cpu_buffer);
    return cnt;
    }
    static const struct file_operations write_event_fops = {
    .write	= write_event_write,
    };
#[no_mangle]
unsafe extern "C" fn remote_test_init_tracefs(d: *mut dentry, unused: *mut c_void) -> c_int {
    static int remote_test_init_tracefs(struct dentry *d, void *unused)
    {
    return tracefs_create_file("write_event", 0200, d, core::ptr::null_mut(), &write_event_fops) ?
    0 : -ENOMEM;
    }
    static struct trace_remote_callbacks trace_remote_callbacks = {
    .init			= remote_test_init_tracefs,
    .load_trace_buffer	= remote_test_load,
    .unload_trace_buffer	= remote_test_unload,
    .enable_tracing		= remote_test_enable_tracing,
    .swap_reader_page	= remote_test_swap_reader_page,
    .reset			= remote_test_reset,
    .enable_event		= remote_test_enable_event,
    };
#[no_mangle]
unsafe extern "C" fn remote_test_init() -> int __init {
    static int __init remote_test_init(void)
    {
    return trace_remote_register("test", &trace_remote_callbacks, core::ptr::null_mut(),
    &remote_event_selftest, 1);
    }
    module_init(remote_test_init);
    MODULE_DESCRIPTION("Test module for the trace remote interface");
    MODULE_AUTHOR("Vincent Donnefort");
    MODULE_LICENSE("GPL");
