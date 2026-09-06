//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kernel/cacheinfo.c
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
// ARM64 cacheinfo support
//
// Copyright (C) 2015 ARM Ltd.
// All Rights Reserved
//

#[no_mangle]
pub unsafe extern "C" fn cache_line_size() -> c_int {
    int cache_line_size(void)
    {
    if (coherency_max_size != 0)
    return coherency_max_size;
    return cache_line_size_of_cpu();
    }
    EXPORT_SYMBOL_GPL(cache_line_size);
#[no_mangle]
pub unsafe extern "C" fn get_cache_type(level: c_int) -> enum cache_type {
    static inline enum cache_type get_cache_type(int level)
    {
    u64 clidr;
    if (level > MAX_CACHE_LEVEL)
    return CACHE_TYPE_NOCACHE;
    clidr = read_sysreg(clidr_el1);
    return CLIDR_CTYPE(clidr, level);
    }
    static void ci_leaf_init(struct cacheinfo *this_leaf,
    enum cache_type type, unsigned int level)
    {
    this_leaf.level = level;
    this_leaf.type = type;
    }
#[no_mangle]
unsafe extern "C" fn detect_cache_level(level_p: *mut c_uint, leaves_p: *mut c_uint) {
    static void detect_cache_level(unsigned int *level_p, unsigned int *leaves_p)
    {
    unsigned int ctype, level, leaves;
    for (level = 1, leaves = 0; level <= MAX_CACHE_LEVEL; level++) {
    ctype = get_cache_type(level);
    if (ctype == CACHE_TYPE_NOCACHE) {
    level--;
    break;
    }
// Separate instruction and data caches
    leaves += (ctype == CACHE_TYPE_SEPARATE) ? 2 : 1;
    }
// level_p = level;
// leaves_p = leaves;
    }
#[no_mangle]
pub unsafe extern "C" fn early_cache_level(cpu: c_uint) -> c_int {
    int early_cache_level(unsigned int cpu)
    {
    struct cpu_cacheinfo *this_cpu_ci = get_cpu_cacheinfo(cpu);
    detect_cache_level(&this_cpu_ci.num_levels, &this_cpu_ci.num_leaves);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn init_cache_level(cpu: c_uint) -> c_int {
    int init_cache_level(unsigned int cpu)
    {
    unsigned int level, leaves;
    int fw_level, ret;
    struct cpu_cacheinfo *this_cpu_ci = get_cpu_cacheinfo(cpu);
    detect_cache_level(&level, &leaves);
    if (acpi_disabled) {
    fw_level = of_find_last_cache_level(cpu);
    } else {
    ret = acpi_get_cache_info(cpu, &fw_level, core::ptr::null_mut());
    if (ret < 0)
    fw_level = 0;
    }
    if (level < fw_level) {
//
// some external caches not specified in CLIDR_EL1
// the information may be available in the device tree
// only unified external caches are considered here
//
    leaves += (fw_level - level);
    level = fw_level;
    }
    this_cpu_ci.num_levels = level;
    this_cpu_ci.num_leaves = leaves;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn populate_cache_leaves(cpu: c_uint) -> c_int {
    int populate_cache_leaves(unsigned int cpu)
    {
    unsigned int level, idx;
    enum cache_type type;
    struct cpu_cacheinfo *this_cpu_ci = get_cpu_cacheinfo(cpu);
    struct cacheinfo *infos = this_cpu_ci.info_list;
    for (idx = 0, level = 1; level <= this_cpu_ci.num_levels &&
    idx < this_cpu_ci.num_leaves; level++) {
    type = get_cache_type(level);
    if (type == CACHE_TYPE_SEPARATE) {
    if (idx + 1 >= this_cpu_ci.num_leaves)
    break;
    ci_leaf_init(&infos[idx++], CACHE_TYPE_DATA, level);
    ci_leaf_init(&infos[idx++], CACHE_TYPE_INST, level);
    } else {
    ci_leaf_init(&infos[idx++], type, level);
    }
    }
    return 0;
    }
