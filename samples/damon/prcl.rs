//! Automatically rewritten from C to Rust
//! Source: samples/damon/prcl.c
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
// proactive reclamation: monitor access pattern of a given process, find
// regions that seems not accessed, and proactively page out the regions.
//

    static int target_pid __read_mostly;
    module_param(target_pid, int, 0600);
    static int damon_sample_prcl_enable_store(
    const char *val, const struct kernel_param *kp);
    static const struct kernel_param_ops enabled_param_ops = {
    .set = damon_sample_prcl_enable_store,
    .get = param_get_bool,
    };
    static bool enabled __read_mostly;
    module_param_cb(enabled, &enabled_param_ops, &enabled, 0600);
    MODULE_PARM_DESC(enabled, "Enable or disable DAMON_SAMPLE_PRCL");
    static struct damon_ctx *ctx;
    static struct pid *target_pidp;
#[no_mangle]
unsafe extern "C" fn damon_sample_prcl_repeat_call_fn(data: *mut c_void) -> c_int {
    static int damon_sample_prcl_repeat_call_fn(void *data)
    {
    struct damon_ctx *c = data;
    struct damon_target *t;
    damon_for_each_target(t, c) {
    struct damon_region *r;
    let mut wss: c_ulong = 0;
    damon_for_each_region(r, t) {
    if (r.nr_accesses > 0)
    wss += r.ar.end - r.ar.start;
    }
    pr_info("wss: %lu\n", wss);
    }
    return 0;
    }
    static struct damon_call_control repeat_call_control = {
    .fn = damon_sample_prcl_repeat_call_fn,
    .repeat = true,
    };
#[no_mangle]
unsafe extern "C" fn damon_sample_prcl_start() -> c_int {
    static int damon_sample_prcl_start(void)
    {
    struct damon_target *target;
    struct damos *scheme;
    int err;
    pr_info("start\n");
    ctx = damon_new_ctx();
    if (!ctx)
    return -ENOMEM;
    if (damon_select_ops(ctx, DAMON_OPS_VADDR)) {
    damon_destroy_ctx(ctx);
    return -EINVAL;
    }
    target = damon_new_target();
    if (!target) {
    damon_destroy_ctx(ctx);
    return -ENOMEM;
    }
    damon_add_target(ctx, target);
    target_pidp = find_get_pid(target_pid);
    if (!target_pidp) {
    damon_destroy_ctx(ctx);
    return -EINVAL;
    }
    target.pid = target_pidp;
    scheme = damon_new_scheme(
    &(struct damos_access_pattern) {
    .min_sz_region = PAGE_SIZE,
    .max_sz_region = ULONG_MAX,
    .min_nr_accesses = 0,
    .max_nr_accesses = 0,
    .min_age_region = 50,
    .max_age_region = UINT_MAX},
    DAMOS_PAGEOUT,
    0,
    &(struct damos_quota){},
    &(struct damos_watermarks){},
    NUMA_NO_NODE);
    if (!scheme) {
    damon_destroy_ctx(ctx);
    return -ENOMEM;
    }
    damon_set_schemes(ctx, &scheme, 1);
    err = damon_start(&ctx, 1, true);
    if (err) {
    damon_destroy_ctx(ctx);
    return err;
    }
    repeat_call_control.data = ctx;
    err = damon_call(ctx, &repeat_call_control);
    if (err)
    damon_destroy_ctx(ctx);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn damon_sample_prcl_stop() {
    static void damon_sample_prcl_stop(void)
    {
    pr_info("stop\n");
    if (ctx) {
    damon_stop(&ctx, 1);
    damon_destroy_ctx(ctx);
    }
    }
    static int damon_sample_prcl_enable_store(
    const char *val, const struct kernel_param *kp)
    {
    let mut is_enabled: bool = enabled;
    int err;
    err = kstrtobool(val, &enabled);
    if (err)
    return err;
    if (enabled == is_enabled)
    return 0;
    if (!damon_initialized())
    return 0;
    if (enabled) {
    err = damon_sample_prcl_start();
    if (err)
    enabled = false;
    return err;
    }
    damon_sample_prcl_stop();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn damon_sample_prcl_init() -> int __init {
    static int __init damon_sample_prcl_init(void)
    {
    let mut err: c_int = 0;
    if (!damon_initialized()) {
    if (enabled)
    enabled = false;
    return -ENOMEM;
    }
    if (enabled) {
    err = damon_sample_prcl_start();
    if (err)
    enabled = false;
    }
    return 0;
    }
    module_init(damon_sample_prcl_init);
