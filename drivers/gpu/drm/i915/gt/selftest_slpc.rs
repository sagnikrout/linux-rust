//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/i915/gt/selftest_slpc.c
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
// Copyright © 2021 Intel Corporation
//
pub const NUM_STEPS: c_int = 5;
pub const H2G_DELAY: c_int = 50000;

    GEN9_FREQ_SCALER)
    enum test_type {
    VARY_MIN,
    VARY_MAX,
    MAX_GRANTED,
    SLPC_POWER,
    TILE_INTERACTION,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slpc_thread {
    pub worker: *mut kthread_worker,
    pub work: kthread_work,
    pub gt: *mut intel_gt,
    pub result: c_int,
}

#[no_mangle]
unsafe extern "C" fn slpc_set_min_freq(slpc: *mut intel_guc_slpc, freq: u32) -> c_int {
    static int slpc_set_min_freq(struct intel_guc_slpc *slpc, u32 freq)
    {
    int ret;
    ret = intel_guc_slpc_set_min_freq(slpc, freq);
    if (ret)
    pr_err("Could not set min frequency to [%u]\n", freq);
    else /* Delay to ensure h2g completes */
    delay_for_h2g();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn slpc_set_max_freq(slpc: *mut intel_guc_slpc, freq: u32) -> c_int {
    static int slpc_set_max_freq(struct intel_guc_slpc *slpc, u32 freq)
    {
    int ret;
    ret = intel_guc_slpc_set_max_freq(slpc, freq);
    if (ret)
    pr_err("Could not set maximum frequency [%u]\n",
    freq);
    else /* Delay to ensure h2g completes */
    delay_for_h2g();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn slpc_set_freq(gt: *mut intel_gt, freq: u32) -> c_int {
    static int slpc_set_freq(struct intel_gt *gt, u32 freq)
    {
    int err;
    struct intel_guc_slpc *slpc = &gt_to_guc(gt).slpc;
    err = slpc_set_max_freq(slpc, freq);
    if (err) {
    pr_err("Unable to update max freq");
    return err;
    }
    err = slpc_set_min_freq(slpc, freq);
    if (err) {
    pr_err("Unable to update min freq");
    return err;
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn slpc_restore_freq(slpc: *mut intel_guc_slpc, min: u32, max: u32) -> c_int {
    static int slpc_restore_freq(struct intel_guc_slpc *slpc, u32 min, u32 max)
    {
    int err;
    err = slpc_set_max_freq(slpc, max);
    if (err) {
    pr_err("Unable to restore max freq");
    return err;
    }
    err = slpc_set_min_freq(slpc, min);
    if (err) {
    pr_err("Unable to restore min freq");
    return err;
    }
    err = intel_guc_slpc_set_ignore_eff_freq(slpc, false);
    if (err) {
    pr_err("Unable to restore efficient freq");
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn slpc_measure_power(rps: *mut intel_rps, freq: *mut c_int) -> u64 {
    static u64 slpc_measure_power(struct intel_rps *rps, int *freq)
    {
    u64 x[5];
    int i;
    for (i = 0; i < 5; i++)
    x[i] = __measure_power(5);
// freq = (*freq + intel_rps_read_actual_frequency(rps)) / 2;
// A simple triangle filter for better result stability
    sort(x, 5, sizeof(*x), cmp_u64, core::ptr::null_mut());
    return div_u64(x[1] + 2 * x[2] + x[3], 4);
    }
#[no_mangle]
unsafe extern "C" fn measure_power_at_freq(gt: *mut intel_gt, freq: *mut c_int, power: *mut u64) -> u64 {
    static u64 measure_power_at_freq(struct intel_gt *gt, int *freq, u64 *power)
    {
    let mut err: c_int = 0;
    err = slpc_set_freq(gt, *freq);
    if (err)
    return err;
// freq = intel_rps_read_actual_frequency(&gt->rps);
// power = slpc_measure_power(&gt->rps, freq);
    return err;
    }
    static int vary_max_freq(struct intel_guc_slpc *slpc, struct intel_rps *rps,
    u32 *max_act_freq)
    {
    u32 step, max_freq, req_freq;
    u32 act_freq;
    let mut err: c_int = 0;
// Go from max to min in 5 steps
    step = (slpc.rp0_freq - slpc.min_freq) / NUM_STEPS;
// max_act_freq = slpc->min_freq;
    for (max_freq = slpc.rp0_freq; max_freq > slpc.min_freq;
    max_freq -= step) {
    err = slpc_set_max_freq(slpc, max_freq);
    if (err)
    break;
    req_freq = intel_rps_read_punit_req_frequency(rps);
// GuC requests freq in multiples of 50/3 MHz
    if (req_freq > (max_freq + FREQUENCY_REQ_UNIT)) {
    pr_err("SWReq is %d, should be at most %d\n", req_freq,
    max_freq + FREQUENCY_REQ_UNIT);
    err = -EINVAL;
    }
    act_freq =  intel_rps_read_actual_frequency(rps);
    if (act_freq > *max_act_freq)
// max_act_freq = act_freq;
    if (err)
    break;
    }
    return err;
    }
    static int vary_min_freq(struct intel_guc_slpc *slpc, struct intel_rps *rps,
    u32 *max_act_freq)
    {
    u32 step, min_freq, req_freq;
    u32 act_freq;
    let mut err: c_int = 0;
// Go from min to max in 5 steps
    step = (slpc.rp0_freq - slpc.min_freq) / NUM_STEPS;
// max_act_freq = slpc->min_freq;
    for (min_freq = slpc.min_freq; min_freq < slpc.rp0_freq;
    min_freq += step) {
    err = slpc_set_min_freq(slpc, min_freq);
    if (err)
    break;
    req_freq = intel_rps_read_punit_req_frequency(rps);
// GuC requests freq in multiples of 50/3 MHz
    if (req_freq < (min_freq - FREQUENCY_REQ_UNIT)) {
    pr_err("SWReq is %d, should be at least %d\n", req_freq,
    min_freq - FREQUENCY_REQ_UNIT);
    err = -EINVAL;
    }
    act_freq =  intel_rps_read_actual_frequency(rps);
    if (act_freq > *max_act_freq)
// max_act_freq = act_freq;
    if (err)
    break;
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn slpc_power(gt: *mut intel_gt, engine: *mut intel_engine_cs) -> c_int {
    static int slpc_power(struct intel_gt *gt, struct intel_engine_cs *engine)
    {
    struct intel_guc_slpc *slpc = &gt_to_guc(gt).slpc;
    struct {
    u64 power;
    int freq;
    } min, max;
    let mut err: c_int = 0;
//
// Our fundamental assumption is that running at lower frequency
// actually saves power. Let's see if our RAPL measurement supports
// that theory.
//
    if (!librapl_supported(gt.i915))
    return 0;
    min.freq = slpc.min_freq;
    err = measure_power_at_freq(gt, &min.freq, &min.power);
    if (err)
    return err;
    max.freq = slpc.rp0_freq;
    err = measure_power_at_freq(gt, &max.freq, &max.power);
    if (err)
    return err;
    pr_info("%s: min:%llumW @ %uMHz, max:%llumW @ %uMHz\n",
    engine.name,
    min.power, min.freq,
    max.power, max.freq);
    if (10 * min.freq >= 9 * max.freq) {
    pr_notice("Could not control frequency, ran at [%uMHz, %uMhz]\n",
    min.freq, max.freq);
    }
    if (11 * min.power > 10 * max.power) {
    pr_err("%s: did not conserve power when setting lower frequency!\n",
    engine.name);
    err = -EINVAL;
    }
// Restore min/max frequencies
    slpc_set_max_freq(slpc, slpc.rp0_freq);
    slpc_set_min_freq(slpc, slpc.min_freq);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn max_granted_freq(slpc: *mut intel_guc_slpc, rps: *mut intel_rps, max_act_freq: *mut u32) -> c_int {
    static int max_granted_freq(struct intel_guc_slpc *slpc, struct intel_rps *rps, u32 *max_act_freq)
    {
    struct intel_gt *gt = rps_to_gt(rps);
    u32 perf_limit_reasons;
    let mut err: c_int = 0;
    err = slpc_set_min_freq(slpc, slpc.rp0_freq);
    if (err)
    return err;
// max_act_freq =  intel_rps_read_actual_frequency(rps);
    if (*max_act_freq != slpc.rp0_freq) {
// Check if there was some throttling by pcode
    perf_limit_reasons = intel_uncore_read(gt.uncore,
    intel_gt_perf_limit_reasons_reg(gt));
// If not, this is an error
    if (!(perf_limit_reasons & GT0_PERF_LIMIT_REASONS_MASK)) {
    pr_err("Pcode did not grant max freq\n");
    err = -EINVAL;
    } else {
    pr_info("Pcode throttled frequency 0x%x\n", perf_limit_reasons);
    }
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn run_test(gt: *mut intel_gt, test_type: c_int) -> c_int {
    static int run_test(struct intel_gt *gt, int test_type)
    {
    struct intel_guc_slpc *slpc = &gt_to_guc(gt).slpc;
    struct intel_rps *rps = &gt.rps;
    struct intel_engine_cs *engine;
    enum intel_engine_id id;
    intel_wakeref_t wakeref;
    struct igt_spinner spin;
    u32 slpc_min_freq, slpc_max_freq;
    let mut err: c_int = 0;
    if (!intel_uc_uses_guc_slpc(&gt.uc))
    return 0;
    if (slpc.min_freq == slpc.rp0_freq) {
    pr_err("Min/Max are fused to the same value\n");
    return -EINVAL;
    }
    if (igt_spinner_init(&spin, gt))
    return -ENOMEM;
    if (intel_guc_slpc_get_max_freq(slpc, &slpc_max_freq)) {
    pr_err("Could not get SLPC max freq\n");
    return -EIO;
    }
    if (intel_guc_slpc_get_min_freq(slpc, &slpc_min_freq)) {
    pr_err("Could not get SLPC min freq\n");
    return -EIO;
    }
//
// Set min frequency to RPn so that we can test the whole
// range of RPn-RP0.
//
    err = slpc_set_min_freq(slpc, slpc.min_freq);
    if (err) {
    pr_err("Unable to update min freq!");
    return err;
    }
//
// Turn off efficient frequency so RPn/RP0 ranges are obeyed.
//
    err = intel_guc_slpc_set_ignore_eff_freq(slpc, true);
    if (err) {
    pr_err("Unable to turn off efficient freq!");
    return err;
    }
    intel_gt_pm_wait_for_idle(gt);
    wakeref = intel_gt_pm_get(gt);
    for_each_engine(engine, gt, id) {
    struct i915_request *rq;
    u32 max_act_freq;
    if (!intel_engine_can_store_dword(engine))
    continue;
    st_engine_heartbeat_disable(engine);
    rq = igt_spinner_create_request(&spin,
    engine.kernel_context,
    MI_NOOP);
    if (IS_ERR(rq)) {
    err = PTR_ERR(rq);
    st_engine_heartbeat_enable(engine);
    break;
    }
    i915_request_add(rq);
    if (!igt_wait_for_spinner(&spin, rq)) {
    pr_err("%s: Spinner did not start\n",
    engine.name);
    igt_spinner_end(&spin);
    st_engine_heartbeat_enable(engine);
    intel_gt_set_wedged(engine.gt);
    err = -EIO;
    break;
    }
    switch (test_type) {
    case VARY_MIN:
    err = vary_min_freq(slpc, rps, &max_act_freq);
    break;
    case VARY_MAX:
    err = vary_max_freq(slpc, rps, &max_act_freq);
    break;
    case MAX_GRANTED:
    case TILE_INTERACTION:
// Media engines have a different RP0
    if (gt.type != GT_MEDIA && (engine.class == VIDEO_DECODE_CLASS ||
    engine.class == VIDEO_ENHANCEMENT_CLASS)) {
    igt_spinner_end(&spin);
    st_engine_heartbeat_enable(engine);
    err = 0;
    continue;
    }
    err = max_granted_freq(slpc, rps, &max_act_freq);
    break;
    case SLPC_POWER:
    err = slpc_power(gt, engine);
    break;
    }
    if (test_type != SLPC_POWER) {
    pr_info("Max actual frequency for %s was %d\n",
    engine.name, max_act_freq);
// Actual frequency should rise above min
    if (max_act_freq <= slpc.min_freq) {
    pr_err("Actual freq did not rise above min\n");
    pr_err("Perf Limit Reasons: 0x%x\n",
    intel_uncore_read(gt.uncore,
    intel_gt_perf_limit_reasons_reg(gt)));
    err = -EINVAL;
    }
    }
    igt_spinner_end(&spin);
    st_engine_heartbeat_enable(engine);
    if (err)
    break;
    }
// Restore min/max/efficient frequencies
    err = slpc_restore_freq(slpc, slpc_min_freq, slpc_max_freq);
    if (igt_flush_test(gt.i915))
    err = -EIO;
    intel_gt_pm_put(gt, wakeref);
    igt_spinner_fini(&spin);
    intel_gt_pm_wait_for_idle(gt);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn live_slpc_vary_min(arg: *mut c_void) -> c_int {
    static int live_slpc_vary_min(void *arg)
    {
    struct drm_i915_private *i915 = arg;
    struct intel_gt *gt;
    unsigned int i;
    int ret;
    for_each_gt(gt, i915, i) {
    ret = run_test(gt, VARY_MIN);
    if (ret)
    return ret;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn live_slpc_vary_max(arg: *mut c_void) -> c_int {
    static int live_slpc_vary_max(void *arg)
    {
    struct drm_i915_private *i915 = arg;
    struct intel_gt *gt;
    unsigned int i;
    int ret;
    for_each_gt(gt, i915, i) {
    ret = run_test(gt, VARY_MAX);
    if (ret)
    return ret;
    }
    return ret;
    }
// check if pcode can grant RP0
#[no_mangle]
unsafe extern "C" fn live_slpc_max_granted(arg: *mut c_void) -> c_int {
    static int live_slpc_max_granted(void *arg)
    {
    struct drm_i915_private *i915 = arg;
    struct intel_gt *gt;
    unsigned int i;
    int ret;
    for_each_gt(gt, i915, i) {
    ret = run_test(gt, MAX_GRANTED);
    if (ret)
    return ret;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn live_slpc_power(arg: *mut c_void) -> c_int {
    static int live_slpc_power(void *arg)
    {
    struct drm_i915_private *i915 = arg;
    struct intel_gt *gt;
    unsigned int i;
    int ret;
    for_each_gt(gt, i915, i) {
    ret = run_test(gt, SLPC_POWER);
    if (ret)
    return ret;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn slpc_spinner_thread(work: *mut kthread_work) {
    static void slpc_spinner_thread(struct kthread_work *work)
    {
    struct slpc_thread *thread = container_of(work, typeof(*thread), work);
    thread.result = run_test(thread.gt, TILE_INTERACTION);
    }
#[no_mangle]
unsafe extern "C" fn live_slpc_tile_interaction(arg: *mut c_void) -> c_int {
    static int live_slpc_tile_interaction(void *arg)
    {
    struct drm_i915_private *i915 = arg;
    struct intel_gt *gt;
    struct slpc_thread *threads;
    let mut i: c_int = 0, ret = 0;
    threads = kzalloc_objs(*threads, I915_MAX_GT);
    if (!threads)
    return -ENOMEM;
    for_each_gt(gt, i915, i) {
    threads[i].worker = kthread_run_worker(0, "igt/slpc_parallel:%d", gt.info.id);
    if (IS_ERR(threads[i].worker)) {
    ret = PTR_ERR(threads[i].worker);
    break;
    }
    threads[i].gt = gt;
    kthread_init_work(&threads[i].work, slpc_spinner_thread);
    kthread_queue_work(threads[i].worker, &threads[i].work);
    }
    for_each_gt(gt, i915, i) {
    int status;
    if (IS_ERR_OR_NULL(threads[i].worker))
    continue;
    kthread_flush_work(&threads[i].work);
    status = READ_ONCE(threads[i].result);
    if (status && !ret) {
    pr_err("%s GT %d failed ", __func__, gt.info.id);
    ret = status;
    }
    kthread_destroy_worker(threads[i].worker);
    }
    kfree(threads);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn intel_slpc_live_selftests(i915: *mut drm_i915_private) -> c_int {
    int intel_slpc_live_selftests(struct drm_i915_private *i915)
    {
    static const struct i915_subtest tests[] = {
    SUBTEST(live_slpc_vary_max),
    SUBTEST(live_slpc_vary_min),
    SUBTEST(live_slpc_max_granted),
    SUBTEST(live_slpc_power),
    SUBTEST(live_slpc_tile_interaction),
    };
    struct intel_gt *gt;
    unsigned int i;
    for_each_gt(gt, i915, i) {
    if (intel_gt_is_wedged(gt))
    return 0;
    }
    return i915_live_subtests(tests, i915);
    }
