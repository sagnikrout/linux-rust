//! Automatically rewritten from C to Rust
//! Source: arch/s390/kernel/cache.c
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
// Extract CPU cache information and expose them via sysfs.
//
// Copyright IBM Corp. 2012
//

    enum {
    CACHE_SCOPE_NOTEXISTS,
    CACHE_SCOPE_PRIVATE,
    CACHE_SCOPE_SHARED,
    CACHE_SCOPE_RESERVED,
    };
    enum {
    CTYPE_SEPARATE,
    CTYPE_DATA,
    CTYPE_INSTRUCTION,
    CTYPE_UNIFIED,
    };
    enum {
    EXTRACT_TOPOLOGY,
    EXTRACT_LINE_SIZE,
    EXTRACT_SIZE,
    EXTRACT_ASSOCIATIVITY,
    };
    enum {
    CACHE_TI_UNIFIED = 0,
    CACHE_TI_DATA = 0,
    CACHE_TI_INSTRUCTION,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cache_info {
    pub 4: unsigned char :,
    pub 2: unsigned char scope :,
    pub 2: unsigned char type :,
}

pub const CACHE_MAX_LEVEL: c_int = 8;
    union cache_topology {
    struct cache_info ci[CACHE_MAX_LEVEL];
    unsigned long raw;
    };
    static const char * const cache_type_string[] = {
    "",
    "Instruction",
    "Data",
    "",
    "Unified",
    };
    static const enum cache_type cache_type_map[] = {
    [CTYPE_SEPARATE] = CACHE_TYPE_SEPARATE,
    [CTYPE_DATA] = CACHE_TYPE_DATA,
    [CTYPE_INSTRUCTION] = CACHE_TYPE_INST,
    [CTYPE_UNIFIED] = CACHE_TYPE_UNIFIED,
    };
#[no_mangle]
pub unsafe extern "C" fn show_cacheinfo(m: *mut seq_file) {
    void show_cacheinfo(struct seq_file *m)
    {
    struct cpu_cacheinfo *this_cpu_ci;
    struct cacheinfo *cache;
    int idx;
    this_cpu_ci = get_cpu_cacheinfo(cpumask_any(cpu_online_mask));
    for (idx = 0; idx < this_cpu_ci.num_leaves; idx++) {
    cache = this_cpu_ci.info_list + idx;
    seq_printf(m, "cache%-11d: ", idx);
    seq_printf(m, "level=%d ", cache.level);
    seq_printf(m, "type=%s ", cache_type_string[cache.type]);
    seq_printf(m, "scope=%s ",
    cache.disable_sysfs ? "Shared" : "Private");
    seq_printf(m, "size=%dK ", cache.size >> 10);
    seq_printf(m, "line_size=%u ", cache.coherency_line_size);
    seq_printf(m, "associativity=%d", cache.ways_of_associativity);
    seq_puts(m, "\n");
    }
    }
#[no_mangle]
pub unsafe extern "C" fn get_cache_type(ci: *mut cache_info, level: c_int) -> enum cache_type {
    static inline enum cache_type get_cache_type(struct cache_info *ci, int level)
    {
    if (level >= CACHE_MAX_LEVEL)
    return CACHE_TYPE_NOCACHE;
    ci += level;
    if (ci.scope != CACHE_SCOPE_SHARED && ci.scope != CACHE_SCOPE_PRIVATE)
    return CACHE_TYPE_NOCACHE;
    return cache_type_map[ci.type];
    }
#[no_mangle]
pub unsafe extern "C" fn ecag(ai: c_int, li: c_int, ti: c_int) -> c_ulong {
    static inline unsigned long ecag(int ai, int li, int ti)
    {
    return __ecag(ECAG_CACHE_ATTRIBUTE, ai << 4 | li << 1 | ti);
    }
    static void ci_leaf_init(struct cacheinfo *this_leaf, int private,
    enum cache_type type, unsigned int level, int cpu)
    {
    int ti, num_sets;
    if (type == CACHE_TYPE_INST)
    ti = CACHE_TI_INSTRUCTION;
    else
    ti = CACHE_TI_UNIFIED;
    this_leaf.level = level + 1;
    this_leaf.type = type;
    this_leaf.coherency_line_size = ecag(EXTRACT_LINE_SIZE, level, ti);
    this_leaf.ways_of_associativity = ecag(EXTRACT_ASSOCIATIVITY, level, ti);
    this_leaf.size = ecag(EXTRACT_SIZE, level, ti);
    num_sets = this_leaf.size / this_leaf.coherency_line_size;
    num_sets /= this_leaf.ways_of_associativity;
    this_leaf.number_of_sets = num_sets;
    cpumask_set_cpu(cpu, &this_leaf.shared_cpu_map);
    if (!private)
    this_leaf.disable_sysfs = true;
    }
#[no_mangle]
pub unsafe extern "C" fn init_cache_level(cpu: c_uint) -> c_int {
    int init_cache_level(unsigned int cpu)
    {
    struct cpu_cacheinfo *this_cpu_ci = get_cpu_cacheinfo(cpu);
    let mut level: c_uint = 0, leaves = 0;
    union cache_topology ct;
    enum cache_type ctype;
    if (!this_cpu_ci)
    return -EINVAL;
    ct.raw = ecag(EXTRACT_TOPOLOGY, 0, 0);
    do {
    ctype = get_cache_type(&ct.ci[0], level);
    if (ctype == CACHE_TYPE_NOCACHE)
    break;
// Separate instruction and data caches
    leaves += (ctype == CACHE_TYPE_SEPARATE) ? 2 : 1;
    } while (++level < CACHE_MAX_LEVEL);
    this_cpu_ci.num_levels = level;
    this_cpu_ci.num_leaves = leaves;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn populate_cache_leaves(cpu: c_uint) -> c_int {
    int populate_cache_leaves(unsigned int cpu)
    {
    struct cpu_cacheinfo *this_cpu_ci = get_cpu_cacheinfo(cpu);
    struct cacheinfo *this_leaf = this_cpu_ci.info_list;
    unsigned int level, idx, pvt;
    union cache_topology ct;
    enum cache_type ctype;
    ct.raw = ecag(EXTRACT_TOPOLOGY, 0, 0);
    for (idx = 0, level = 0; level < this_cpu_ci.num_levels &&
    idx < this_cpu_ci.num_leaves; idx++, level++) {
    if (!this_leaf)
    return -EINVAL;
    pvt = (ct.ci[level].scope == CACHE_SCOPE_PRIVATE) ? 1 : 0;
    ctype = get_cache_type(&ct.ci[0], level);
    if (ctype == CACHE_TYPE_SEPARATE) {
    ci_leaf_init(this_leaf++, pvt, CACHE_TYPE_DATA, level, cpu);
    ci_leaf_init(this_leaf++, pvt, CACHE_TYPE_INST, level, cpu);
    } else {
    ci_leaf_init(this_leaf++, pvt, ctype, level, cpu);
    }
    }
    this_cpu_ci.cpu_map_populated = true;
    return 0;
    }
