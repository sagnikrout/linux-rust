//! Automatically rewritten from C to Rust
//! Source: drivers/base/regmap/regcache-flat.c
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
// Register cache access API - flat caching support
//
// Copyright 2012 Wolfson Microelectronics plc
//
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>

    static inline unsigned int regcache_flat_get_index(const struct regmap *map,
    unsigned int reg)
    {
    return regcache_get_index_by_order(map, reg);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct regcache_flat_data {
    pub valid: *mut c_ulong,
    pub data: [c_uint; ],
}

#[no_mangle]
unsafe extern "C" fn regcache_flat_init(map: *mut regmap) -> c_int {
    static int regcache_flat_init(struct regmap *map)
    {
    unsigned int cache_size;
    struct regcache_flat_data *cache;
    if (!map || map.reg_stride_order < 0 || !map.max_register_is_set)
    return -EINVAL;
    cache_size = regcache_flat_get_index(map, map.max_register) + 1;
    cache = kzalloc_flex(*cache, data, cache_size, map.alloc_flags);
    if (!cache)
    return -ENOMEM;
    cache.valid = bitmap_zalloc(cache_size, map.alloc_flags);
    if (!cache.valid)
    goto err_free;
    map.cache = cache;
    return 0;
    err_free:
    kfree(cache);
    return -ENOMEM;
    }
#[no_mangle]
unsafe extern "C" fn regcache_flat_exit(map: *mut regmap) {
    static void regcache_flat_exit(struct regmap *map)
    {
    struct regcache_flat_data *cache = map.cache;
    if (cache)
    bitmap_free(cache.valid);
    kfree(cache);
    map.cache = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn regcache_flat_populate(map: *mut regmap) -> c_int {
    static int regcache_flat_populate(struct regmap *map)
    {
    struct regcache_flat_data *cache = map.cache;
    unsigned int i;
    for (i = 0; i < map.num_reg_defaults; i++) {
    let mut reg: c_uint = map.reg_defaults[i].reg;
    let mut index: c_uint = regcache_flat_get_index(map, reg);
    cache.data[index] = map.reg_defaults[i].def;
    __set_bit(index, cache.valid);
    }
    if (map.reg_default_cb) {
    dev_dbg(map.dev,
    "Populating regcache_flat using reg_default_cb callback\n");
    for (i = 0; i <= map.max_register; i += map.reg_stride) {
    let mut index: c_uint = regcache_flat_get_index(map, i);
    unsigned int value;
    if (test_bit(index, cache.valid))
    continue;
    if (map.reg_default_cb(map.dev, i, &value))
    continue;
    cache.data[index] = value;
    __set_bit(index, cache.valid);
    }
    }
    return 0;
    }
    static int regcache_flat_read(struct regmap *map,
    unsigned int reg, unsigned int *value)
    {
    struct regcache_flat_data *cache = map.cache;
    let mut index: c_uint = regcache_flat_get_index(map, reg);
// legacy behavior: ignore validity, but warn the user
    if (unlikely(!test_bit(index, cache.valid)))
    dev_warn_once(map.dev,
    "using zero-initialized flat cache, this may cause unexpected behavior");
// value = cache->data[index];
    return 0;
    }
    static int regcache_flat_sparse_read(struct regmap *map,
    unsigned int reg, unsigned int *value)
    {
    struct regcache_flat_data *cache = map.cache;
    let mut index: c_uint = regcache_flat_get_index(map, reg);
    if (unlikely(!test_bit(index, cache.valid)))
    return -ENOENT;
// value = cache->data[index];
    return 0;
    }
    static int regcache_flat_write(struct regmap *map, unsigned int reg,
    unsigned int value)
    {
    struct regcache_flat_data *cache = map.cache;
    let mut index: c_uint = regcache_flat_get_index(map, reg);
    cache.data[index] = value;
    __set_bit(index, cache.valid);
    return 0;
    }
    static int regcache_flat_drop(struct regmap *map, unsigned int min,
    unsigned int max)
    {
    struct regcache_flat_data *cache = map.cache;
    let mut bitmap_min: c_uint = regcache_flat_get_index(map, min);
    let mut bitmap_max: c_uint = regcache_flat_get_index(map, max);
    bitmap_clear(cache.valid, bitmap_min, bitmap_max + 1 - bitmap_min);
    return 0;
    }
    struct regcache_ops regcache_flat_ops = {
    .type = REGCACHE_FLAT,
    .name = "flat",
    .init = regcache_flat_init,
    .exit = regcache_flat_exit,
    .populate = regcache_flat_populate,
    .read = regcache_flat_read,
    .write = regcache_flat_write,
    };
    struct regcache_ops regcache_flat_sparse_ops = {
    .type = REGCACHE_FLAT_S,
    .name = "flat-sparse",
    .init = regcache_flat_init,
    .exit = regcache_flat_exit,
    .populate = regcache_flat_populate,
    .read = regcache_flat_sparse_read,
    .write = regcache_flat_write,
    .drop = regcache_flat_drop,
    };
