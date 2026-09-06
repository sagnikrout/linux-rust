//! Automatically rewritten from C to Rust
//! Source: arch/riscv/kernel/cacheinfo.c
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
// Copyright (C) 2017 SiFive
//

    static struct riscv_cacheinfo_ops *rv_cache_ops;
#[no_mangle]
pub unsafe extern "C" fn riscv_set_cacheinfo_ops(ops: *mut riscv_cacheinfo_ops) {
    void riscv_set_cacheinfo_ops(struct riscv_cacheinfo_ops *ops)
    {
    rv_cache_ops = ops;
    }
    EXPORT_SYMBOL_GPL(riscv_set_cacheinfo_ops);
    const struct attribute_group *
    cache_get_priv_group(struct cacheinfo *this_leaf)
    {
    if (rv_cache_ops && rv_cache_ops.get_priv_group)
    return rv_cache_ops.get_priv_group(this_leaf);
    return core::ptr::null_mut();
    }
    static struct cacheinfo *get_cacheinfo(u32 level, enum cache_type type)
    {
//
// Using raw_smp_processor_id() elides a preemptability check, but this
// is really indicative of a larger problem: the cacheinfo UABI assumes
// that cores have a homonogenous view of the cache hierarchy.  That
// happens to be the case for the current set of RISC-V systems, but
// likely won't be true in general.  Since there's no way to provide
// correct information for these systems via the current UABI we're
// just eliding the check for now.
//
    struct cpu_cacheinfo *this_cpu_ci = get_cpu_cacheinfo(raw_smp_processor_id());
    struct cacheinfo *this_leaf;
    int index;
    for (index = 0; index < this_cpu_ci.num_leaves; index++) {
    this_leaf = this_cpu_ci.info_list + index;
    if (this_leaf.level == level && this_leaf.type == type)
    return this_leaf;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn get_cache_size(level: u32, type: enum cache_type) -> uintptr_t {
    uintptr_t get_cache_size(u32 level, enum cache_type type)
    {
    struct cacheinfo *this_leaf = get_cacheinfo(level, type);
    return this_leaf ? this_leaf.size : 0;
    }
#[no_mangle]
pub unsafe extern "C" fn get_cache_geometry(level: u32, type: enum cache_type) -> uintptr_t {
    uintptr_t get_cache_geometry(u32 level, enum cache_type type)
    {
    struct cacheinfo *this_leaf = get_cacheinfo(level, type);
    return this_leaf ? (this_leaf.ways_of_associativity << 16 |
    this_leaf.coherency_line_size) :
    0;
    }
    static void ci_leaf_init(struct cacheinfo *this_leaf,
    enum cache_type type, unsigned int level)
    {
    this_leaf.level = level;
    this_leaf.type = type;
    }
#[no_mangle]
pub unsafe extern "C" fn init_cache_level(cpu: c_uint) -> c_int {
    int init_cache_level(unsigned int cpu)
    {
    return init_of_cache_level(cpu);
    }
#[no_mangle]
pub unsafe extern "C" fn populate_cache_leaves(cpu: c_uint) -> c_int {
    int populate_cache_leaves(unsigned int cpu)
    {
    struct cpu_cacheinfo *this_cpu_ci = get_cpu_cacheinfo(cpu);
    struct cacheinfo *this_leaf = this_cpu_ci.info_list;
    struct device_node *np, *prev;
    let mut levels: c_int = 1, level = 1;
    if (!acpi_disabled) {
    int ret, fw_levels, split_levels;
    ret = acpi_get_cache_info(cpu, &fw_levels, &split_levels);
    if (ret)
    return ret;
    BUG_ON((split_levels > fw_levels) ||
    (split_levels + fw_levels > this_cpu_ci.num_leaves));
    for (; level <= this_cpu_ci.num_levels; level++) {
    if (level <= split_levels) {
    ci_leaf_init(this_leaf++, CACHE_TYPE_DATA, level);
    ci_leaf_init(this_leaf++, CACHE_TYPE_INST, level);
    } else {
    ci_leaf_init(this_leaf++, CACHE_TYPE_UNIFIED, level);
    }
    }
    return 0;
    }
    np = of_cpu_device_node_get(cpu);
    if (!np)
    return -ENOENT;
    if (of_property_present(np, "cache-size"))
    ci_leaf_init(this_leaf++, CACHE_TYPE_UNIFIED, level);
    if (of_property_present(np, "i-cache-size"))
    ci_leaf_init(this_leaf++, CACHE_TYPE_INST, level);
    if (of_property_present(np, "d-cache-size"))
    ci_leaf_init(this_leaf++, CACHE_TYPE_DATA, level);
    prev = np;
    while ((np = of_find_next_cache_node(np))) {
    of_node_put(prev);
    prev = np;
    if (!of_device_is_compatible(np, "cache"))
    break;
    if (of_property_read_u32(np, "cache-level", &level))
    break;
    if (level <= levels)
    break;
    if (of_property_present(np, "cache-size"))
    ci_leaf_init(this_leaf++, CACHE_TYPE_UNIFIED, level);
    if (of_property_present(np, "i-cache-size"))
    ci_leaf_init(this_leaf++, CACHE_TYPE_INST, level);
    if (of_property_present(np, "d-cache-size"))
    ci_leaf_init(this_leaf++, CACHE_TYPE_DATA, level);
    levels = level;
    }
    of_node_put(prev);
    return 0;
    }
