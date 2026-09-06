//! Automatically rewritten from C to Rust
//! Source: mm/damon/stat.c
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
// Shows data access monitoring results in simple metrics.
//

    static int damon_stat_enabled_store(
    const char *val, const struct kernel_param *kp);
    static int damon_stat_enabled_load(char *buffer,
    const struct kernel_param *kp);
    static const struct kernel_param_ops enabled_param_ops = {
    .set = damon_stat_enabled_store,
    .get = damon_stat_enabled_load,
    };
    static bool enabled __read_mostly = IS_ENABLED(
    CONFIG_DAMON_STAT_ENABLED_DEFAULT);
    module_param_cb(enabled, &enabled_param_ops, core::ptr::null_mut(), 0600);
    MODULE_PARM_DESC(enabled, "Enable of disable DAMON_STAT");
    static unsigned long estimated_memory_bandwidth __read_mostly;
    module_param(estimated_memory_bandwidth, ulong, 0400);
    MODULE_PARM_DESC(estimated_memory_bandwidth,
    "Estimated memory bandwidth usage in bytes per second");
    static long memory_idle_ms_percentiles[101] = {0,};
    module_param_array(memory_idle_ms_percentiles, long, core::ptr::null_mut(), 0400);
    MODULE_PARM_DESC(memory_idle_ms_percentiles,
    "Memory idle time percentiles in milliseconds");
    static unsigned long aggr_interval_us;
    module_param(aggr_interval_us, ulong, 0400);
    MODULE_PARM_DESC(aggr_interval_us,
    "Current tuned aggregation interval in microseconds");
    static struct damon_ctx *damon_stat_context;
    static unsigned long damon_stat_last_refresh_jiffies;
#[no_mangle]
unsafe extern "C" fn damon_stat_set_estimated_memory_bandwidth(c: *mut damon_ctx) {
    static void damon_stat_set_estimated_memory_bandwidth(struct damon_ctx *c)
    {
    struct damon_target *t;
    struct damon_region *r;
    let mut access_bytes: c_ulong = 0;
    damon_for_each_target(t, c) {
    damon_for_each_region(r, t)
    access_bytes += (r.ar.end - r.ar.start) *
    r.nr_accesses;
    }
    estimated_memory_bandwidth = access_bytes * USEC_PER_MSEC *
    MSEC_PER_SEC / c.attrs.aggr_interval;
    }
#[no_mangle]
unsafe extern "C" fn damon_stat_idletime(r: *const damon_region) -> c_int {
    static int damon_stat_idletime(const struct damon_region *r)
    {
    if (r.nr_accesses)
    return -1 * (r.age + 1);
    return r.age + 1;
    }
#[no_mangle]
unsafe extern "C" fn damon_stat_cmp_regions(a: *const c_void, b: *const c_void) -> c_int {
    static int damon_stat_cmp_regions(const void *a, const void *b)
    {
    const struct damon_region *ra = *(const struct damon_region **)a;
    const struct damon_region *rb = *(const struct damon_region **)b;
    return damon_stat_idletime(ra) - damon_stat_idletime(rb);
    }
    static int damon_stat_sort_regions(struct damon_ctx *c,
    struct damon_region ***sorted_ptr, int *nr_regions_ptr,
    unsigned long *total_sz_ptr)
    {
    struct damon_target *t;
    struct damon_region *r;
    struct damon_region **region_pointers;
    let mut nr_regions: c_uint = 0;
    let mut total_sz: c_ulong = 0;
    damon_for_each_target(t, c) {
// there is only one target
    region_pointers = kmalloc_objs(*region_pointers,
    damon_nr_regions(t));
    if (!region_pointers)
    return -ENOMEM;
    damon_for_each_region(r, t) {
    region_pointers[nr_regions++] = r;
    total_sz += r.ar.end - r.ar.start;
    }
    }
    sort(region_pointers, nr_regions, sizeof(*region_pointers),
    damon_stat_cmp_regions, core::ptr::null_mut());
// sorted_ptr = region_pointers;
// nr_regions_ptr = nr_regions;
// total_sz_ptr = total_sz;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn damon_stat_set_idletime_percentiles(c: *mut damon_ctx) {
    static void damon_stat_set_idletime_percentiles(struct damon_ctx *c)
    {
    struct damon_region **sorted_regions, *region;
    int nr_regions;
    unsigned long total_sz, accounted_bytes = 0;
    int err, i, next_percentile = 0;
    err = damon_stat_sort_regions(c, &sorted_regions, &nr_regions,
    &total_sz);
    if (err)
    return;
    for (i = 0; i < nr_regions; i++) {
    region = sorted_regions[i];
    accounted_bytes += region.ar.end - region.ar.start;
    while (next_percentile <= accounted_bytes * 100 / total_sz)
    memory_idle_ms_percentiles[next_percentile++] =
    damon_stat_idletime(region) *
    (long)c.attrs.aggr_interval / USEC_PER_MSEC;
    }
    kfree(sorted_regions);
    }
#[no_mangle]
unsafe extern "C" fn damon_stat_damon_call_fn(data: *mut c_void) -> c_int {
    static int damon_stat_damon_call_fn(void *data)
    {
    struct damon_ctx *c = data;
// avoid unnecessarily frequent stat update
    if (time_before_eq(jiffies, damon_stat_last_refresh_jiffies +
    secs_to_jiffies(5)))
    return 0;
    damon_stat_last_refresh_jiffies = jiffies;
    aggr_interval_us = c.attrs.aggr_interval;
    damon_stat_set_estimated_memory_bandwidth(c);
    damon_stat_set_idletime_percentiles(c);
    return 0;
    }
    static struct damon_ctx *damon_stat_build_ctx(void)
    {
    struct damon_ctx *ctx;
    struct damon_attrs attrs;
    struct damon_target *target;
    let mut start: c_ulong = 0, end = 0;
    ctx = damon_new_ctx();
    if (!ctx)
    return core::ptr::null_mut();
    attrs = (struct damon_attrs) {
    .sample_interval = 5 * USEC_PER_MSEC,
    .aggr_interval = 100 * USEC_PER_MSEC,
    .ops_update_interval = 60 * USEC_PER_MSEC * MSEC_PER_SEC,
    .min_nr_regions = 10,
    .max_nr_regions = 1000,
    };
//
// auto-tune sampling and aggregation interval aiming 4% DAMON-observed
// accesses ratio, keeping sampling interval in [5ms, 10s] range.
//
    attrs.intervals_goal = (struct damon_intervals_goal) {
    .access_bp = 400, .aggrs = 3,
    .min_sample_us = 5000, .max_sample_us = 10000000,
    };
    if (damon_set_attrs(ctx, &attrs))
    goto free_out;
    if (damon_select_ops(ctx, DAMON_OPS_PADDR))
    goto free_out;
    target = damon_new_target();
    if (!target)
    goto free_out;
    damon_add_target(ctx, target);
    if (damon_set_region_system_rams_default(target, &start, &end,
    ctx.addr_unit, ctx.min_region_sz))
    goto free_out;
    return ctx;
    free_out:
    damon_destroy_ctx(ctx);
    return core::ptr::null_mut();
    }
    static struct damon_call_control call_control = {
    .fn = damon_stat_damon_call_fn,
    .repeat = true,
    };
#[no_mangle]
unsafe extern "C" fn damon_stat_start() -> c_int {
    static int damon_stat_start(void)
    {
    int err;
    if (damon_stat_context) {
    if (damon_is_running(damon_stat_context))
    return -EAGAIN;
    damon_destroy_ctx(damon_stat_context);
    }
    damon_stat_context = damon_stat_build_ctx();
    if (!damon_stat_context)
    return -ENOMEM;
    err = damon_start(&damon_stat_context, 1, true);
    if (err) {
    damon_destroy_ctx(damon_stat_context);
    damon_stat_context = core::ptr::null_mut();
    return err;
    }
    damon_stat_last_refresh_jiffies = jiffies;
    call_control.data = damon_stat_context;
    return damon_call(damon_stat_context, &call_control);
    }
#[no_mangle]
unsafe extern "C" fn damon_stat_stop() {
    static void damon_stat_stop(void)
    {
    damon_stop(&damon_stat_context, 1);
    damon_destroy_ctx(damon_stat_context);
    damon_stat_context = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn damon_stat_enabled() -> bool {
    static bool damon_stat_enabled(void)
    {
    if (!damon_stat_context)
    return false;
    return damon_is_running(damon_stat_context);
    }
    static int damon_stat_enabled_store(
    const char *val, const struct kernel_param *kp)
    {
    int err;
    err = kstrtobool(val, &enabled);
    if (err)
    return err;
    if (damon_stat_enabled() == enabled)
    return 0;
    if (!damon_initialized())
//
// probably called from command line parsing (parse_args()).
// Cannot call damon_new_ctx().  Let damon_stat_init() handle.
//
    return 0;
    if (enabled)
    return damon_stat_start();
    damon_stat_stop();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn damon_stat_enabled_load(buffer: *mut c_char, kp: *const kernel_param) -> c_int {
    static int damon_stat_enabled_load(char *buffer, const struct kernel_param *kp)
    {
    return sprintf(buffer, "%c\n", damon_stat_enabled() ? 'Y' : 'N');
    }
    static int damon_stat_kdamond_pid_store(
    const char *val, const struct kernel_param *kp)
    {
//
// kdamond_pid is read-only, but kernel command line could write it.
// Do nothing here.
//
    return 0;
    }
    static int damon_stat_kdamond_pid_load(
    char *buffer, const struct kernel_param *kp)
    {
    int pid;
    if (!damon_stat_context) {
    pid = -1;
    } else {
    pid = damon_kdamond_pid(damon_stat_context);
    if (pid < 1)
    pid = -1;
    }
    return sprintf(buffer, "%d\n", pid);
    }
    static const struct kernel_param_ops kdamond_pid_param_ops = {
    .set = damon_stat_kdamond_pid_store,
    .get = damon_stat_kdamond_pid_load,
    };
//
// PID of the DAMON thread
//
// If DAMON_STAT is enabled, this becomes the PID of the worker thread.
// Else, -1.
//
    module_param_cb(kdamond_pid, &kdamond_pid_param_ops, core::ptr::null_mut(), 0400);
    MODULE_PARM_DESC(kdamond_pid, "pid of the kdamond");
#[no_mangle]
unsafe extern "C" fn damon_stat_init() -> int __init {
    static int __init damon_stat_init(void)
    {
    let mut err: c_int = 0;
    if (!damon_initialized()) {
    err = -ENOMEM;
    goto out;
    }
// probably set via command line
    if (enabled)
    err = damon_stat_start();
    out:
    if (err && enabled)
    enabled = false;
    return err;
    }
    module_init(damon_stat_init);
