//! Automatically rewritten from C to Rust
//! Source: drivers/md/dm-ps-io-affinity.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2020 Oracle Corporation
//
// Module Author: Mike Christie
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct path_info {
    pub path: *mut dm_path,
    pub cpumask: cpumask_var_t,
    pub refcount: refcount_t,
    pub failed: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct selector {
    pub path_map: *mut path_info,
    pub path_mask: cpumask_var_t,
    pub map_misses: core::sync::atomic::AtomicI32,
}

#[no_mangle]
unsafe extern "C" fn ioa_free_path(s: *mut selector, cpu: c_uint) {
    static void ioa_free_path(struct selector *s, unsigned int cpu)
    {
    struct path_info *pi = s.path_map[cpu];
    if (!pi)
    return;
    if (refcount_dec_and_test(&pi.refcount)) {
    cpumask_clear_cpu(cpu, s.path_mask);
    free_cpumask_var(pi.cpumask);
    kfree(pi);
    s.path_map[cpu] = core::ptr::null_mut();
    }
    }
    static int ioa_add_path(struct path_selector *ps, struct dm_path *path,
    int argc, char **argv, char **error)
    {
    struct selector *s = ps.context;
    struct path_info *pi = core::ptr::null_mut();
    unsigned int cpu;
    int ret;
    if (argc != 1) {
// error = "io-affinity ps: invalid number of arguments";
    return -EINVAL;
    }
    pi = kzalloc_obj(*pi);
    if (!pi) {
// error = "io-affinity ps: Error allocating path context";
    return -ENOMEM;
    }
    pi.path = path;
    path.pscontext = pi;
    refcount_set(&pi.refcount, 1);
    if (!zalloc_cpumask_var(&pi.cpumask, GFP_KERNEL)) {
// error = "io-affinity ps: Error allocating cpumask context";
    ret = -ENOMEM;
    goto free_pi;
    }
    ret = cpumask_parse(argv[0], pi.cpumask);
    if (ret) {
// error = "io-affinity ps: invalid cpumask";
    ret = -EINVAL;
    goto free_mask;
    }
    for_each_cpu(cpu, pi.cpumask) {
    if (cpu >= nr_cpu_ids) {
    DMWARN_LIMIT("Ignoring mapping for CPU %u. Max CPU is %u",
    cpu, nr_cpu_ids);
    break;
    }
    if (s.path_map[cpu]) {
    DMWARN("CPU mapping for %u exists. Ignoring.", cpu);
    continue;
    }
    cpumask_set_cpu(cpu, s.path_mask);
    s.path_map[cpu] = pi;
    refcount_inc(&pi.refcount);
    }
    if (refcount_dec_and_test(&pi.refcount)) {
// error = "io-affinity ps: No new/valid CPU mapping found";
    ret = -EINVAL;
    goto free_mask;
    }
    return 0;
    free_mask:
    free_cpumask_var(pi.cpumask);
    free_pi:
    kfree(pi);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ioa_create(ps: *mut path_selector, argc: c_uint, argv: *mut c_char) -> c_int {
    static int ioa_create(struct path_selector *ps, unsigned int argc, char **argv)
    {
    struct selector *s;
    s = kmalloc_obj(*s);
    if (!s)
    return -ENOMEM;
    s.path_map = kzalloc_objs(struct path_info *, nr_cpu_ids);
    if (!s.path_map)
    goto free_selector;
    if (!zalloc_cpumask_var(&s.path_mask, GFP_KERNEL))
    goto free_map;
    atomic_set(&s.map_misses, 0);
    ps.context = s;
    return 0;
    free_map:
    kfree(s.path_map);
    free_selector:
    kfree(s);
    return -ENOMEM;
    }
#[no_mangle]
unsafe extern "C" fn ioa_destroy(ps: *mut path_selector) {
    static void ioa_destroy(struct path_selector *ps)
    {
    struct selector *s = ps.context;
    unsigned int cpu;
    for_each_cpu(cpu, s.path_mask)
    ioa_free_path(s, cpu);
    free_cpumask_var(s.path_mask);
    kfree(s.path_map);
    kfree(s);
    ps.context = core::ptr::null_mut();
    }
    static int ioa_status(struct path_selector *ps, struct dm_path *path,
    status_type_t type, char *result, unsigned int maxlen)
    {
    struct selector *s = ps.context;
    struct path_info *pi;
    let mut sz: c_int = 0;
    if (!path) {
    DMEMIT("0 ");
    return sz;
    }
    switch (type) {
    case STATUSTYPE_INFO:
    DMEMIT("%d ", atomic_read(&s.map_misses));
    break;
    case STATUSTYPE_TABLE:
    pi = path.pscontext;
    DMEMIT("%*pb ", cpumask_pr_args(pi.cpumask));
    break;
    case STATUSTYPE_IMA:
// result = '\0';
    break;
    }
    return sz;
    }
#[no_mangle]
unsafe extern "C" fn ioa_fail_path(ps: *mut path_selector, p: *mut dm_path) {
    static void ioa_fail_path(struct path_selector *ps, struct dm_path *p)
    {
    struct path_info *pi = p.pscontext;
    pi.failed = true;
    }
#[no_mangle]
unsafe extern "C" fn ioa_reinstate_path(ps: *mut path_selector, p: *mut dm_path) -> c_int {
    static int ioa_reinstate_path(struct path_selector *ps, struct dm_path *p)
    {
    struct path_info *pi = p.pscontext;
    pi.failed = false;
    return 0;
    }
    static struct dm_path *ioa_select_path(struct path_selector *ps,
    size_t nr_bytes)
    {
    unsigned int cpu, node;
    struct selector *s = ps.context;
    const struct cpumask *cpumask;
    struct path_info *pi;
    int i;
    cpu = get_cpu();
    pi = s.path_map[cpu];
    if (pi && !pi.failed)
    goto done;
//
// Perf is not optimal, but we at least try the local node then just
// try not to fail.
//
    if (!pi)
    atomic_inc(&s.map_misses);
    node = cpu_to_node(cpu);
    cpumask = cpumask_of_node(node);
    for_each_cpu(i, cpumask) {
    pi = s.path_map[i];
    if (pi && !pi.failed)
    goto done;
    }
    for_each_cpu(i, s.path_mask) {
    pi = s.path_map[i];
    if (pi && !pi.failed)
    goto done;
    }
    pi = core::ptr::null_mut();
    done:
    put_cpu();
    return pi ? pi.path : core::ptr::null_mut();
    }
    static struct path_selector_type ioa_ps = {
    .name		= "io-affinity",
    .module		= THIS_MODULE,
    .table_args	= 1,
    .info_args	= 1,
    .create		= ioa_create,
    .destroy	= ioa_destroy,
    .status		= ioa_status,
    .add_path	= ioa_add_path,
    .fail_path	= ioa_fail_path,
    .reinstate_path	= ioa_reinstate_path,
    .select_path	= ioa_select_path,
    };
#[no_mangle]
unsafe extern "C" fn dm_ioa_init() -> int __init {
    static int __init dm_ioa_init(void)
    {
    let mut ret: c_int = dm_register_path_selector(&ioa_ps);
    if (ret < 0)
    DMERR("register failed %d", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dm_ioa_exit() -> void __exit {
    static void __exit dm_ioa_exit(void)
    {
    dm_unregister_path_selector(&ioa_ps);
    }
    module_init(dm_ioa_init);
    module_exit(dm_ioa_exit);
    MODULE_DESCRIPTION(DM_NAME " multipath path selector that selects paths based on the CPU IO is being executed on");
    MODULE_AUTHOR("Mike Christie <michael.christie@oracle.com>");
    MODULE_LICENSE("GPL");
