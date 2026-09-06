//! Automatically rewritten from C to Rust
//! Source: arch/loongarch/kernel/cacheinfo.c
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
// LoongArch cacheinfo support
//
// Copyright (C) 2020-2022 Loongson Technology Corporation Limited
//

#[no_mangle]
pub unsafe extern "C" fn init_cache_level(cpu: c_uint) -> c_int {
    int init_cache_level(unsigned int cpu)
    {
    let mut cache_present: c_int = current_cpu_data.cache_leaves_present;
    struct cpu_cacheinfo *this_cpu_ci = get_cpu_cacheinfo(cpu);
    this_cpu_ci.num_levels =
    current_cpu_data.cache_leaves[cache_present - 1].level;
    this_cpu_ci.num_leaves = cache_present;
    return 0;
    }
    static inline bool cache_leaves_are_shared(struct cacheinfo *this_leaf,
    struct cacheinfo *sib_leaf)
    {
    return (!(*(unsigned char *)(this_leaf.priv) & CACHE_PRIVATE)
    && !(*(unsigned char *)(sib_leaf.priv) & CACHE_PRIVATE));
    }
#[no_mangle]
unsafe extern "C" fn cache_cpumap_setup(cpu: c_uint) {
    static void cache_cpumap_setup(unsigned int cpu)
    {
    unsigned int index;
    struct cacheinfo *this_leaf, *sib_leaf;
    struct cpu_cacheinfo *this_cpu_ci = get_cpu_cacheinfo(cpu);
    for (index = 0; index < this_cpu_ci.num_leaves; index++) {
    unsigned int i;
    this_leaf = this_cpu_ci.info_list + index;
// skip if shared_cpu_map is already populated
    if (!cpumask_empty(&this_leaf.shared_cpu_map))
    continue;
    cpumask_set_cpu(cpu, &this_leaf.shared_cpu_map);
    for_each_online_cpu(i) {
    struct cpu_cacheinfo *sib_cpu_ci = get_cpu_cacheinfo(i);
    if (i == cpu || !sib_cpu_ci.info_list ||
    (cpu_to_node(i) != cpu_to_node(cpu)))
    continue;
    sib_leaf = sib_cpu_ci.info_list + index;
// SMT cores share all caches
    if (cpus_are_siblings(i, cpu)) {
    cpumask_set_cpu(cpu, &sib_leaf.shared_cpu_map);
    cpumask_set_cpu(i, &this_leaf.shared_cpu_map);
    }
// Node's cores share shared caches
    if (cache_leaves_are_shared(this_leaf, sib_leaf)) {
    cpumask_set_cpu(cpu, &sib_leaf.shared_cpu_map);
    cpumask_set_cpu(i, &this_leaf.shared_cpu_map);
    }
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn populate_cache_leaves(cpu: c_uint) -> c_int {
    int populate_cache_leaves(unsigned int cpu)
    {
    int i, cache_present = current_cpu_data.cache_leaves_present;
    struct cpu_cacheinfo *this_cpu_ci = get_cpu_cacheinfo(cpu);
    struct cacheinfo *this_leaf = this_cpu_ci.info_list;
    struct cache_desc *cd, *cdesc = current_cpu_data.cache_leaves;
    for (i = 0; i < cache_present; i++) {
    cd = cdesc + i;
    this_leaf.type = cd.type;
    this_leaf.level = cd.level;
    this_leaf.coherency_line_size = cd.linesz;
    this_leaf.number_of_sets = cd.sets;
    this_leaf.ways_of_associativity = cd.ways;
    this_leaf.size = cd.linesz * cd.sets * cd.ways;
    this_leaf.priv = &cd.flags;
    this_leaf++;
    }
    cache_cpumap_setup(cpu);
    this_cpu_ci.cpu_map_populated = true;
    return 0;
    }
