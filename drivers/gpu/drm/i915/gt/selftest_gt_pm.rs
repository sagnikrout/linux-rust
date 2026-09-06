//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/i915/gt/selftest_gt_pm.c
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2019 Intel Corporation
//

#[no_mangle]
unsafe extern "C" fn cmp_u64(A: *const c_void, B: *const c_void) -> c_int {
    static int cmp_u64(const void *A, const void *B)
    {
    const u64 *a = A, *b = B;
    if (*a < *b)
    return -1;
#[no_mangle]
pub unsafe extern "C" fn if(b: *mut *mut a >) -> else {
    else if (*a > *b)
    return 1;
    else
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cmp_u32(A: *const c_void, B: *const c_void) -> c_int {
    static int cmp_u32(const void *A, const void *B)
    {
    const u32 *a = A, *b = B;
    if (*a < *b)
    return -1;
#[no_mangle]
pub unsafe extern "C" fn if(b: *mut *mut a >) -> else {
    else if (*a > *b)
    return 1;
    else
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn read_timestamp(engine: *mut intel_engine_cs) -> u32 {
    static u32 read_timestamp(struct intel_engine_cs *engine)
    {
    struct drm_i915_private *i915 = engine.i915;
// On i965 the first read tends to give a stale value
    ENGINE_READ_FW(engine, RING_TIMESTAMP);
    if (GRAPHICS_VER(i915) == 5 || IS_G4X(i915))
    return ENGINE_READ_FW(engine, RING_TIMESTAMP_UDW);
    else
    return ENGINE_READ_FW(engine, RING_TIMESTAMP);
    }
    static void measure_clocks(struct intel_engine_cs *engine,
    u32 *out_cycles, ktime_t *out_dt)
    {
    ktime_t dt[5];
    u32 cycles[5];
    int i;
    for (i = 0; i < 5; i++) {
    local_irq_disable();
    cycles[i] = -read_timestamp(engine);
    dt[i] = ktime_get();
    udelay(1000);
    cycles[i] += read_timestamp(engine);
    dt[i] = ktime_sub(ktime_get(), dt[i]);
    local_irq_enable();
    }
// Use the median of both cycle/dt; close enough
    sort(cycles, 5, sizeof(*cycles), cmp_u32, core::ptr::null_mut());
// out_cycles = (cycles[1] + 2 * cycles[2] + cycles[3]) / 4;
    sort(dt, 5, sizeof(*dt), cmp_u64, core::ptr::null_mut());
// out_dt = div_u64(dt[1] + 2 * dt[2] + dt[3], 4);
    }
#[no_mangle]
unsafe extern "C" fn live_gt_clocks(arg: *mut c_void) -> c_int {
    static int live_gt_clocks(void *arg)
    {
    struct intel_gt *gt = arg;
    struct intel_engine_cs *engine;
    enum intel_engine_id id;
    intel_wakeref_t wakeref;
    let mut err: c_int = 0;
    if (!gt.clock_frequency) { /* unknown */
    pr_info("CS_TIMESTAMP frequency unknown\n");
    return 0;
    }
    if (GRAPHICS_VER(gt.i915) < 4) /* Any CS_TIMESTAMP? */
    return 0;
    wakeref = intel_gt_pm_get(gt);
    intel_uncore_forcewake_get(gt.uncore, FORCEWAKE_ALL);
    for_each_engine(engine, gt, id) {
    u32 cycles;
    u32 expected;
    u64 time;
    u64 dt;
    if (GRAPHICS_VER(engine.i915) < 7 && engine.id != RCS0)
    continue;
    measure_clocks(engine, &cycles, &dt);
    time = intel_gt_clock_interval_to_ns(engine.gt, cycles);
    expected = intel_gt_ns_to_clock_interval(engine.gt, dt);
    pr_info("%s: TIMESTAMP %d cycles [%lldns] in %lldns [%d cycles], using CS clock frequency of %uKHz\n",
    engine.name, cycles, time, dt, expected,
    engine.gt.clock_frequency / 1000);
    if (9 * time < 8 * dt || 8 * time > 9 * dt) {
    pr_err("%s: CS ticks did not match walltime!\n",
    engine.name);
    err = -EINVAL;
    break;
    }
    if (9 * expected < 8 * cycles || 8 * expected > 9 * cycles) {
    pr_err("%s: walltime did not match CS ticks!\n",
    engine.name);
    err = -EINVAL;
    break;
    }
    }
    intel_uncore_forcewake_put(gt.uncore, FORCEWAKE_ALL);
    intel_gt_pm_put(gt, wakeref);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn live_gt_resume(arg: *mut c_void) -> c_int {
    static int live_gt_resume(void *arg)
    {
    struct intel_gt *gt = arg;
    IGT_TIMEOUT(end_time);
    int err;
// Do several suspend/resume cycles to check we don't explode!
    do {
    intel_gt_suspend_prepare(gt);
    intel_gt_suspend_late(gt);
    if (gt.rc6.enabled) {
    pr_err("rc6 still enabled after suspend!\n");
    intel_gt_set_wedged_on_init(gt);
    err = -EINVAL;
    break;
    }
    err = intel_gt_resume(gt);
    if (err)
    break;
    if (gt.rc6.supported && !gt.rc6.enabled) {
    pr_err("rc6 not enabled upon resume!\n");
    intel_gt_set_wedged_on_init(gt);
    err = -EINVAL;
    break;
    }
    err = st_llc_verify(&gt.llc);
    if (err) {
    pr_err("llc state not restored upon resume!\n");
    intel_gt_set_wedged_on_init(gt);
    break;
    }
    } while (!__igt_timeout(end_time, core::ptr::null_mut()));
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn intel_gt_pm_live_selftests(i915: *mut drm_i915_private) -> c_int {
    int intel_gt_pm_live_selftests(struct drm_i915_private *i915)
    {
    static const struct i915_subtest tests[] = {
    SUBTEST(live_gt_clocks),
    SUBTEST(live_rc6_manual),
    SUBTEST(live_rps_clock_interval),
    SUBTEST(live_rps_control),
    SUBTEST(live_rps_frequency_cs),
    SUBTEST(live_rps_frequency_srm),
    SUBTEST(live_rps_power),
    SUBTEST(live_rps_interrupt),
    SUBTEST(live_rps_dynamic),
    SUBTEST(live_gt_resume),
    };
    if (intel_gt_is_wedged(to_gt(i915)))
    return 0;
    return intel_gt_live_subtests(tests, to_gt(i915));
    }
#[no_mangle]
pub unsafe extern "C" fn intel_gt_pm_late_selftests(i915: *mut drm_i915_private) -> c_int {
    int intel_gt_pm_late_selftests(struct drm_i915_private *i915)
    {
    static const struct i915_subtest tests[] = {
//
// These tests may leave the system in an undesirable state.
// They are intended to be run last in CI and the system
// rebooted afterwards.
//
    SUBTEST(live_rc6_ctx_wa),
    };
    if (intel_gt_is_wedged(to_gt(i915)))
    return 0;
    return intel_gt_live_subtests(tests, to_gt(i915));
    }
