//! Automatically rewritten from C to Rust
//! Source: drivers/base/regmap/regcache.c
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
// Register cache access API
//
// Copyright 2011 Wolfson Microelectronics plc
//
// Author: Dimitris Papastamos <dp@opensource.wolfsonmicro.com>

    static const struct regcache_ops *cache_types[] = {
    &regcache_flat_sparse_ops,
    &regcache_rbtree_ops,
    &regcache_maple_ops,
    &regcache_flat_ops,
    };
#[no_mangle]
unsafe extern "C" fn regcache_defaults_cmp(a: *const c_void, b: *const c_void) -> c_int {
    static int regcache_defaults_cmp(const void *a, const void *b)
    {
    const struct reg_default *x = a;
    const struct reg_default *y = b;
    if (x.reg > y.reg)
    return 1;
#[no_mangle]
pub unsafe extern "C" fn if(y->reg: x->reg <) -> else {
    else if (x.reg < y.reg)
    return -1;
    else
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn regcache_sort_defaults(defaults: *mut reg_default, ndefaults: c_uint) {
    void regcache_sort_defaults(struct reg_default *defaults, unsigned int ndefaults)
    {
    sort(defaults, ndefaults, sizeof(*defaults),
    regcache_defaults_cmp, core::ptr::null_mut());
    }
    EXPORT_SYMBOL_GPL(regcache_sort_defaults);
#[no_mangle]
unsafe extern "C" fn regcache_count_cacheable_registers(map: *mut regmap) -> c_int {
    static int regcache_count_cacheable_registers(struct regmap *map)
    {
    unsigned int count;
// calculate the size of reg_defaults
    count = 0;
    for (unsigned int i = 0; i < map.num_reg_defaults_raw; i++)
    if (regmap_readable(map, i * map.reg_stride) &&
    !regmap_volatile(map, i * map.reg_stride))
    count++;
    return count;
    }
#[no_mangle]
unsafe extern "C" fn regcache_hw_init(map: *mut regmap) -> c_int {
    static int regcache_hw_init(struct regmap *map)
    {
    int ret;
    unsigned int reg, val;
    void *tmp_buf;
    if (!map.reg_defaults_raw) {
    let mut cache_bypass: bool = map.cache_bypass;
    dev_dbg(map.dev, "No cache defaults, reading back from HW\n");
// Bypass the cache access till data read from HW
    map.cache_bypass = true;
    tmp_buf = kmalloc(map.cache_size_raw, GFP_KERNEL);
    if (!tmp_buf)
    return -ENOMEM;
    ret = regmap_raw_read(map, 0, tmp_buf,
    map.cache_size_raw);
    map.cache_bypass = cache_bypass;
    if (ret == 0) {
    map.reg_defaults_raw = tmp_buf;
    map.cache_free = true;
    } else {
    kfree(tmp_buf);
    }
    }
// fill the reg_defaults
    for (unsigned int i = 0, j = 0; i < map.num_reg_defaults_raw; i++) {
    reg = i * map.reg_stride;
    if (!regmap_readable(map, reg))
    continue;
    if (regmap_volatile(map, reg))
    continue;
    if (map.reg_defaults_raw) {
    val = regcache_get_val(map, map.reg_defaults_raw, i);
    } else {
    let mut cache_bypass: bool = map.cache_bypass;
    map.cache_bypass = true;
    ret = regmap_read(map, reg, &val);
    map.cache_bypass = cache_bypass;
    if (ret != 0) {
    dev_err(map.dev, "Failed to read %x: %d\n",
    reg, ret);
    return ret;
    }
    }
    map.reg_defaults[j].reg = reg;
    map.reg_defaults[j].def = val;
    j++;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn regcache_hw_exit(map: *mut regmap) {
    static void regcache_hw_exit(struct regmap *map)
    {
    if (map.cache_free)
    kfree(map.reg_defaults_raw);
    }
#[no_mangle]
pub unsafe extern "C" fn regcache_init(map: *mut regmap, config: *const regmap_config) -> c_int {
    int regcache_init(struct regmap *map, const struct regmap_config *config)
    {
    let mut sort_defaults: bool = false;
    let mut reg_prev: c_uint = 0;
    let mut count: c_int = 0;
    int ret;
    int i;
    void *tmp_buf;
    if (map.cache_type == REGCACHE_NONE) {
    if (config.reg_defaults || config.num_reg_defaults_raw)
    dev_warn(map.dev,
    "No cache used with register defaults set!\n");
    map.cache_bypass = true;
    return 0;
    }
    if (config.reg_defaults && !config.num_reg_defaults) {
    dev_err(map.dev,
    "Register defaults are set without the number!\n");
    return -EINVAL;
    }
    if (config.num_reg_defaults && !config.reg_defaults) {
    dev_err(map.dev,
    "Register defaults number are set without the reg!\n");
    return -EINVAL;
    }
    for (i = 0; i < config.num_reg_defaults; i++) {
    if (config.reg_defaults[i].reg % map.reg_stride)
    return -EINVAL;
    if (reg_prev > config.reg_defaults[i].reg)
    sort_defaults = true;
    reg_prev = config.reg_defaults[i].reg;
    }
    for (i = 0; i < ARRAY_SIZE(cache_types); i++)
    if (cache_types[i].type == map.cache_type)
    break;
    if (i == ARRAY_SIZE(cache_types)) {
    dev_err(map.dev, "Could not match cache type: %d\n",
    map.cache_type);
    return -EINVAL;
    }
    map.num_reg_defaults = config.num_reg_defaults;
    map.num_reg_defaults_raw = config.num_reg_defaults_raw;
    map.reg_defaults_raw = config.reg_defaults_raw;
    map.cache_word_size = BITS_TO_BYTES(config.val_bits);
    map.cache_size_raw = map.cache_word_size * config.num_reg_defaults_raw;
    map.cache = core::ptr::null_mut();
    map.cache_ops = cache_types[i];
    if (!map.cache_ops.read ||
    !map.cache_ops.write ||
    !map.cache_ops.name)
    return -EINVAL;
// We still need to ensure that the reg_defaults
// won't vanish from under us.  We'll need to make
// a copy of it.
//
    if (config.reg_defaults) {
    tmp_buf = kmemdup_array(config.reg_defaults, map.num_reg_defaults,
    sizeof(*map.reg_defaults), GFP_KERNEL);
    if (!tmp_buf)
    return -ENOMEM;
// regcache_lookup_reg() bsearch()es this array
    if (sort_defaults) {
    dev_warn(map.dev,
    "Driver needs fixing: Unsorted reg_defaults, sorting the copy\n");
    regcache_sort_defaults(tmp_buf, map.num_reg_defaults);
    }
    map.reg_defaults = tmp_buf;
    } else if (map.num_reg_defaults_raw) {
    count = regcache_count_cacheable_registers(map);
    if (!count)
    map.cache_bypass = true;
// All registers are unreadable or volatile, so just bypass
    if (map.cache_bypass)
    return 0;
    map.num_reg_defaults = count;
    map.reg_defaults = kmalloc_objs(struct reg_default, count);
    if (!map.reg_defaults)
    return -ENOMEM;
    }
    if (!map.max_register_is_set && map.num_reg_defaults_raw) {
    map.max_register = (map.num_reg_defaults_raw  - 1) * map.reg_stride;
    map.max_register_is_set = true;
    }
    if (map.cache_ops.init) {
    dev_dbg(map.dev, "Initializing %s cache\n",
    map.cache_ops.name);
    map.lock(map.lock_arg);
    ret = map.cache_ops.init(map);
    map.unlock(map.lock_arg);
    if (ret)
    goto err_free_reg_defaults;
    }
//
// Some devices such as PMICs don't have cache defaults,
// we cope with this by reading back the HW registers and
// crafting the cache defaults by hand.
//
    if (count) {
    ret = regcache_hw_init(map);
    if (ret)
    goto err_exit;
    }
    if (map.cache_ops.populate &&
    (map.num_reg_defaults || map.reg_default_cb)) {
    dev_dbg(map.dev, "Populating %s cache\n", map.cache_ops.name);
    map.lock(map.lock_arg);
    ret = map.cache_ops.populate(map);
    map.unlock(map.lock_arg);
    if (ret)
    goto err_free;
    }
    return 0;
    err_free:
    regcache_hw_exit(map);
    err_exit:
    if (map.cache_ops.exit) {
    dev_dbg(map.dev, "Destroying %s cache\n", map.cache_ops.name);
    map.lock(map.lock_arg);
    map.cache_ops.exit(map);
    map.unlock(map.lock_arg);
    }
    err_free_reg_defaults:
    kfree(map.reg_defaults);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn regcache_exit(map: *mut regmap) {
    void regcache_exit(struct regmap *map)
    {
    if (map.cache_type == REGCACHE_NONE)
    return;
    BUG_ON(!map.cache_ops);
    regcache_hw_exit(map);
    if (map.cache_ops.exit) {
    dev_dbg(map.dev, "Destroying %s cache\n",
    map.cache_ops.name);
    map.lock(map.lock_arg);
    map.cache_ops.exit(map);
    map.unlock(map.lock_arg);
    }
    kfree(map.reg_defaults);
    }
//
// regcache_read - Fetch the value of a given register from the cache.
//
// @map: map to configure.
// @reg: The register index.
// @value: The value to be returned.
//
// Return a negative value on failure, 0 on success.
//
    int regcache_read(struct regmap *map,
    unsigned int reg, unsigned int *value)
    {
    int ret;
    if (map.cache_type == REGCACHE_NONE)
    return -EINVAL;
    BUG_ON(!map.cache_ops);
    if (!regmap_volatile(map, reg)) {
    ret = map.cache_ops.read(map, reg, value);
    if (ret == 0)
    trace_regmap_reg_read_cache(map, reg, *value);
    return ret;
    }
    return -EINVAL;
    }
//
// regcache_write - Set the value of a given register in the cache.
//
// @map: map to configure.
// @reg: The register index.
// @value: The new register value.
//
// Return a negative value on failure, 0 on success.
//
    int regcache_write(struct regmap *map,
    unsigned int reg, unsigned int value)
    {
    if (map.cache_type == REGCACHE_NONE)
    return 0;
    BUG_ON(!map.cache_ops);
    if (!regmap_volatile(map, reg))
    return map.cache_ops.write(map, reg, value);
    return 0;
    }
    bool regcache_reg_needs_sync(struct regmap *map, unsigned int reg,
    unsigned int val)
    {
    int ret;
    if (!regmap_writeable(map, reg))
    return false;
// If we don't know the chip just got reset, then sync everything.
    if (!map.no_sync_defaults)
    return true;
// Is this the hardware default?  If so skip.
    ret = regcache_lookup_reg(map, reg);
    if (ret >= 0 && val == map.reg_defaults[ret].def)
    return false;
    return true;
    }
    static int regcache_default_sync(struct regmap *map, unsigned int min,
    unsigned int max)
    {
    unsigned int reg;
    for (reg = min; reg <= max; reg += map.reg_stride) {
    unsigned int val;
    int ret;
    if (regmap_volatile(map, reg) ||
    !regmap_writeable(map, reg))
    continue;
    ret = regcache_read(map, reg, &val);
    if (ret == -ENOENT)
    continue;
    if (ret)
    return ret;
    if (!regcache_reg_needs_sync(map, reg, val))
    continue;
    map.cache_bypass = true;
    ret = _regmap_write(map, reg, val);
    map.cache_bypass = false;
    if (ret) {
    dev_err(map.dev, "Unable to sync register %#x. %d\n",
    reg, ret);
    return ret;
    }
    dev_dbg(map.dev, "Synced register %#x, value %#x\n", reg, val);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rbtree_all(key: *const c_void, node: *const rb_node) -> c_int {
    static int rbtree_all(const void *key, const struct rb_node *node)
    {
    return 0;
    }
//
// regcache_sync - Sync the register cache with the hardware.
//
// @map: map to configure.
//
// Any registers that should not be synced should be marked as
// volatile.  In general drivers can choose not to use the provided
// syncing functionality if they so require.
//
// This pushes cached changes made while cache_only (e.g. suspend) down
// to hardware. The caller must disable cache_only before calling this
// function.
//
// Return a negative value on failure, 0 on success.
//
#[no_mangle]
pub unsafe extern "C" fn regcache_sync(map: *mut regmap) -> c_int {
    int regcache_sync(struct regmap *map)
    {
    let mut sync_ret: c_int = 0;
    let mut selector_ret: c_int = 0;
    unsigned int i;
    const char *name;
    bool bypass;
    struct rb_node *node;
    if (WARN_ON(map.cache_type == REGCACHE_NONE))
    return -EINVAL;
    BUG_ON(!map.cache_ops);
    map.lock(map.lock_arg);
    if (WARN_ON(map.cache_only)) {
    map.unlock(map.lock_arg);
    return -EINVAL;
    }
// Remember the initial bypass state
    bypass = map.cache_bypass;
    dev_dbg(map.dev, "Syncing %s cache\n",
    map.cache_ops.name);
    name = map.cache_ops.name;
    trace_regcache_sync(map, name, "start");
    if (!map.cache_dirty)
    goto out;
// Apply any patch first
    map.cache_bypass = true;
    for (i = 0; i < map.patch_regs; i++) {
    sync_ret = _regmap_write(map, map.patch[i].reg, map.patch[i].def);
    if (sync_ret != 0) {
    dev_err(map.dev, "Failed to write %x = %x: %d\n",
    map.patch[i].reg, map.patch[i].def, sync_ret);
    goto out;
    }
    }
    map.cache_bypass = false;
    if (map.cache_ops.sync)
    sync_ret = map.cache_ops.sync(map, 0, map.max_register);
    else
    sync_ret = regcache_default_sync(map, 0, map.max_register);
    if (sync_ret == 0)
    map.cache_dirty = false;
    out:
// Restore the bypass state
    map.cache_bypass = bypass;
    map.no_sync_defaults = false;
//
// If we did any paging with cache bypassed and a cached
// paging register then the register and cache state might
// have gone out of sync, force writes of all the paging
// registers.
//
    rb_for_each(node, core::ptr::null_mut(), &map.range_tree, rbtree_all) {
    struct regmap_range_node *this =
    rb_entry(node, struct regmap_range_node, node);
// If there's nothing in the cache there's nothing to sync
    if (regcache_read(map, this.selector_reg, &i) != 0)
    continue;
    selector_ret = _regmap_write(map, this.selector_reg, i);
    if (selector_ret != 0) {
    map.cache_dirty = true;
    dev_err(map.dev, "Failed to write %x = %x: %d\n",
    this.selector_reg, i, selector_ret);
    break;
    }
    }
    map.unlock(map.lock_arg);
    regmap_async_complete(map);
    trace_regcache_sync(map, name, "stop");
    return sync_ret ? sync_ret : selector_ret;
    }
    EXPORT_SYMBOL_GPL(regcache_sync);
//
// regcache_sync_region - Sync part  of the register cache with the hardware.
//
// @map: map to sync.
// @min: first register to sync
// @max: last register to sync
//
// Write all non-default register values in the specified region to
// the hardware.
//
// Return a negative value on failure, 0 on success.
//
    int regcache_sync_region(struct regmap *map, unsigned int min,
    unsigned int max)
    {
    let mut ret: c_int = 0;
    const char *name;
    bool bypass;
    if (WARN_ON(map.cache_type == REGCACHE_NONE))
    return -EINVAL;
    BUG_ON(!map.cache_ops);
    map.lock(map.lock_arg);
    if (WARN_ON(map.cache_only)) {
    map.unlock(map.lock_arg);
    return -EINVAL;
    }
// Remember the initial bypass state
    bypass = map.cache_bypass;
    name = map.cache_ops.name;
    dev_dbg(map.dev, "Syncing %s cache from %#x-%#x\n", name, min, max);
    trace_regcache_sync(map, name, "start region");
    if (!map.cache_dirty)
    goto out;
    map.async = true;
    if (map.cache_ops.sync)
    ret = map.cache_ops.sync(map, min, max);
    else
    ret = regcache_default_sync(map, min, max);
    out:
// Restore the bypass state
    map.cache_bypass = bypass;
    map.async = false;
    map.no_sync_defaults = false;
    map.unlock(map.lock_arg);
    regmap_async_complete(map);
    trace_regcache_sync(map, name, "stop region");
    return ret;
    }
    EXPORT_SYMBOL_GPL(regcache_sync_region);
//
// regcache_drop_region - Discard part of the register cache
//
// @map: map to operate on
// @min: first register to discard
// @max: last register to discard
//
// Discard part of the register cache.
//
// Return a negative value on failure, 0 on success.
//
    int regcache_drop_region(struct regmap *map, unsigned int min,
    unsigned int max)
    {
    let mut ret: c_int = 0;
    if (!map.cache_ops || !map.cache_ops.drop)
    return -EINVAL;
    map.lock(map.lock_arg);
    trace_regcache_drop_region(map, min, max);
    ret = map.cache_ops.drop(map, min, max);
    map.unlock(map.lock_arg);
    return ret;
    }
    EXPORT_SYMBOL_GPL(regcache_drop_region);
//
// regcache_cache_only - Put a register map into cache only mode
//
// @map: map to configure
// @enable: flag if changes should be written to the hardware
//
// When a register map is marked as cache only writes to the register
// map API will only update the register cache, they will not cause
// any hardware changes.  This is useful for allowing portions of
// drivers to act as though the device were functioning as normal when
// it is disabled for power saving reasons.
//
#[no_mangle]
pub unsafe extern "C" fn regcache_cache_only(map: *mut regmap, enable: bool) {
    void regcache_cache_only(struct regmap *map, bool enable)
    {
    map.lock(map.lock_arg);
    WARN_ON(map.cache_type != REGCACHE_NONE &&
    map.cache_bypass && enable);
    map.cache_only = enable;
    trace_regmap_cache_only(map, enable);
    map.unlock(map.lock_arg);
    }
    EXPORT_SYMBOL_GPL(regcache_cache_only);
//
// regcache_mark_dirty - Indicate that HW registers were reset to default values
//
// @map: map to mark
//
// Inform regcache that the device has been powered down or reset, so that
// on resume, regcache_sync() knows to write out all non-default values
// stored in the cache.
//
// If this function is not called, regcache_sync() will assume that
// the hardware state still matches the cache state, modulo any writes that
// happened when cache_only was true.
//
#[no_mangle]
pub unsafe extern "C" fn regcache_mark_dirty(map: *mut regmap) {
    void regcache_mark_dirty(struct regmap *map)
    {
    map.lock(map.lock_arg);
    map.cache_dirty = true;
    map.no_sync_defaults = true;
    map.unlock(map.lock_arg);
    }
    EXPORT_SYMBOL_GPL(regcache_mark_dirty);
//
// regcache_cache_bypass - Put a register map into cache bypass mode
//
// @map: map to configure
// @enable: flag if changes should not be written to the cache
//
// When a register map is marked with the cache bypass option, writes
// to the register map API will only update the hardware and not
// the cache directly.  This is useful when syncing the cache back to
// the hardware.
//
#[no_mangle]
pub unsafe extern "C" fn regcache_cache_bypass(map: *mut regmap, enable: bool) {
    void regcache_cache_bypass(struct regmap *map, bool enable)
    {
    map.lock(map.lock_arg);
    WARN_ON(map.cache_only && enable);
    map.cache_bypass = enable;
    trace_regmap_cache_bypass(map, enable);
    map.unlock(map.lock_arg);
    }
    EXPORT_SYMBOL_GPL(regcache_cache_bypass);
//
// regcache_reg_cached - Check if a register is cached
//
// @map: map to check
// @reg: register to check
//
// Reports if a register is cached.
//
#[no_mangle]
pub unsafe extern "C" fn regcache_reg_cached(map: *mut regmap, reg: c_uint) -> bool {
    bool regcache_reg_cached(struct regmap *map, unsigned int reg)
    {
    unsigned int val;
    int ret;
    map.lock(map.lock_arg);
    ret = regcache_read(map, reg, &val);
    map.unlock(map.lock_arg);
    let mut ret: return = = 0;
    }
    EXPORT_SYMBOL_GPL(regcache_reg_cached);
    void regcache_set_val(struct regmap *map, void *base, unsigned int idx,
    unsigned int val)
    {
// Use device native format if possible
    if (map.format.format_val) {
    map.format.format_val(base + (map.cache_word_size * idx),
    val, 0);
    return;
    }
    switch (map.cache_word_size) {
    case 1: {
    u8 *cache = base;
    cache[idx] = val;
    break;
    }
    case 2: {
    u16 *cache = base;
    cache[idx] = val;
    break;
    }
    case 4: {
    u32 *cache = base;
    cache[idx] = val;
    break;
    }
    default:
    BUG();
    }
    }
    unsigned int regcache_get_val(struct regmap *map, const void *base,
    unsigned int idx)
    {
    if (!base)
    return -EINVAL;
// Use device native format if possible
    if (map.format.parse_val)
    return map.format.parse_val(regcache_get_val_addr(map, base,
    idx));
    switch (map.cache_word_size) {
    case 1: {
    const u8 *cache = base;
    return cache[idx];
    }
    case 2: {
    const u16 *cache = base;
    return cache[idx];
    }
    case 4: {
    const u32 *cache = base;
    return cache[idx];
    }
    default:
    BUG();
    }
// unreachable
    return -1;
    }
#[no_mangle]
pub unsafe extern "C" fn regcache_lookup_reg(map: *mut regmap, reg: c_uint) -> c_int {
    int regcache_lookup_reg(struct regmap *map, unsigned int reg)
    {
    struct reg_default key;
    struct reg_default *r;
    key.reg = reg;
    key.def = 0;
    r = bsearch(&key, map.reg_defaults, map.num_reg_defaults,
    sizeof(struct reg_default), regcache_defaults_cmp);
    if (r)
    return r - map.reg_defaults;
    else
    return -ENOENT;
    }
#[no_mangle]
unsafe extern "C" fn regcache_reg_present(cache_present: *mut c_ulong, idx: c_uint) -> bool {
    static bool regcache_reg_present(unsigned long *cache_present, unsigned int idx)
    {
    if (!cache_present)
    return true;
    return test_bit(idx, cache_present);
    }
#[no_mangle]
pub unsafe extern "C" fn regcache_sync_val(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int {
    int regcache_sync_val(struct regmap *map, unsigned int reg, unsigned int val)
    {
    int ret;
    if (!regcache_reg_needs_sync(map, reg, val))
    return 0;
    map.cache_bypass = true;
    ret = _regmap_write(map, reg, val);
    map.cache_bypass = false;
    if (ret != 0) {
    dev_err(map.dev, "Unable to sync register %#x. %d\n",
    reg, ret);
    return ret;
    }
    dev_dbg(map.dev, "Synced register %#x, value %#x\n",
    reg, val);
    return 0;
    }
    static int regcache_sync_block_single(struct regmap *map, void *block,
    unsigned long *cache_present,
    unsigned int block_base,
    unsigned int start, unsigned int end)
    {
    unsigned int i, regtmp, val;
    int ret;
    for (i = start; i < end; i++) {
    regtmp = block_base + (i * map.reg_stride);
    if (!regcache_reg_present(cache_present, i) ||
    !regmap_writeable(map, regtmp))
    continue;
    val = regcache_get_val(map, block, i);
    ret = regcache_sync_val(map, regtmp, val);
    if (ret != 0)
    return ret;
    }
    return 0;
    }
    static int regcache_sync_block_raw_flush(struct regmap *map, const void **data,
    unsigned int base, unsigned int cur)
    {
    let mut val_bytes: usize = map.format.val_bytes;
    int ret, count;
    if (*data == core::ptr::null_mut())
    return 0;
    count = (cur - base) / map.reg_stride;
    dev_dbg(map.dev, "Writing %zu bytes for %d registers from 0x%x-0x%x\n",
    count * val_bytes, count, base, cur - map.reg_stride);
    map.cache_bypass = true;
    ret = _regmap_raw_write(map, base, *data, count * val_bytes, false);
    if (ret)
    dev_err(map.dev, "Unable to sync registers %#x-%#x. %d\n",
    base, cur - map.reg_stride, ret);
    map.cache_bypass = false;
// data = NULL;
    return ret;
    }
    static int regcache_sync_block_raw(struct regmap *map, void *block,
    unsigned long *cache_present,
    unsigned int block_base, unsigned int start,
    unsigned int end)
    {
    let mut regtmp: c_uint = 0;
    let mut base: c_uint = 0;
    const void *data = core::ptr::null_mut();
    unsigned int val;
    int ret;
    for (unsigned int i = start; i < end; i++) {
    regtmp = block_base + (i * map.reg_stride);
    if (!regcache_reg_present(cache_present, i) ||
    !regmap_writeable(map, regtmp)) {
    ret = regcache_sync_block_raw_flush(map, &data,
    base, regtmp);
    if (ret != 0)
    return ret;
    continue;
    }
    val = regcache_get_val(map, block, i);
    if (!regcache_reg_needs_sync(map, regtmp, val)) {
    ret = regcache_sync_block_raw_flush(map, &data,
    base, regtmp);
    if (ret != 0)
    return ret;
    continue;
    }
    if (!data) {
    data = regcache_get_val_addr(map, block, i);
    base = regtmp;
    }
    }
    return regcache_sync_block_raw_flush(map, &data, base, regtmp +
    map.reg_stride);
    }
    int regcache_sync_block(struct regmap *map, void *block,
    unsigned long *cache_present,
    unsigned int block_base, unsigned int start,
    unsigned int end)
    {
    if (regmap_can_raw_write(map) && !map.use_single_write)
    return regcache_sync_block_raw(map, block, cache_present,
    block_base, start, end);
    else
    return regcache_sync_block_single(map, block, cache_present,
    block_base, start, end);
    }
