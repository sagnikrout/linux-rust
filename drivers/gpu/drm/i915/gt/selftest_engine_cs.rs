//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/i915/gt/selftest_engine_cs.c
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
// Copyright © 2018 Intel Corporation
//

pub const COUNT: c_int = 5;
#[no_mangle]
unsafe extern "C" fn cmp_u32(A: *const c_void, B: *const c_void) -> c_int {
    static int cmp_u32(const void *A, const void *B)
    {
    const u32 *a = A, *b = B;
    return *a - *b;
    }
#[no_mangle]
unsafe extern "C" fn perf_begin(gt: *mut intel_gt) -> intel_wakeref_t {
    static intel_wakeref_t perf_begin(struct intel_gt *gt)
    {
    let mut wakeref: intel_wakeref_t = intel_gt_pm_get(gt);
// Boost gpufreq to max [waitboost] and keep it fixed
    atomic_inc(&gt.rps.num_waiters);
    queue_work(gt.i915.unordered_wq, &gt.rps.work);
    flush_work(&gt.rps.work);
    return wakeref;
    }
#[no_mangle]
unsafe extern "C" fn perf_end(gt: *mut intel_gt, wakeref: intel_wakeref_t) -> c_int {
    static int perf_end(struct intel_gt *gt, intel_wakeref_t wakeref)
    {
    atomic_dec(&gt.rps.num_waiters);
    intel_gt_pm_put(gt, wakeref);
    return igt_flush_test(gt.i915);
    }
#[no_mangle]
unsafe extern "C" fn timestamp_reg(engine: *mut intel_engine_cs) -> i915_reg_t {
    static i915_reg_t timestamp_reg(struct intel_engine_cs *engine)
    {
    struct drm_i915_private *i915 = engine.i915;
    if (GRAPHICS_VER(i915) == 5 || IS_G4X(i915))
    return RING_TIMESTAMP_UDW(engine.mmio_base);
    else
    return RING_TIMESTAMP(engine.mmio_base);
    }
#[no_mangle]
unsafe extern "C" fn write_timestamp(rq: *mut i915_request, slot: c_int) -> c_int {
    static int write_timestamp(struct i915_request *rq, int slot)
    {
    struct intel_timeline *tl =
    rcu_dereference_protected(rq.timeline,
    !i915_request_signaled(rq));
    u32 cmd;
    u32 *cs;
    cs = intel_ring_begin(rq, 4);
    if (IS_ERR(cs))
    return PTR_ERR(cs);
    cmd = MI_STORE_REGISTER_MEM | MI_USE_GGTT;
    if (GRAPHICS_VER(rq.i915) >= 8)
    cmd++;
// cs++ = cmd;
// cs++ = i915_mmio_reg_offset(timestamp_reg(rq->engine));
// cs++ = tl->hwsp_offset + slot * sizeof(u32);
// cs++ = 0;
    intel_ring_advance(rq, cs);
    return 0;
    }
    static struct i915_vma *create_empty_batch(struct intel_context *ce)
    {
    struct drm_i915_gem_object *obj;
    struct i915_vma *vma;
    u32 *cs;
    int err;
    obj = i915_gem_object_create_internal(ce.engine.i915, PAGE_SIZE);
    if (IS_ERR(obj))
    return ERR_CAST(obj);
    cs = i915_gem_object_pin_map_unlocked(obj, I915_MAP_WB);
    if (IS_ERR(cs)) {
    err = PTR_ERR(cs);
    goto err_put;
    }
    cs[0] = MI_BATCH_BUFFER_END;
    i915_gem_object_flush_map(obj);
    vma = i915_vma_instance(obj, ce.vm, core::ptr::null_mut());
    if (IS_ERR(vma)) {
    err = PTR_ERR(vma);
    goto err_unpin;
    }
    err = i915_vma_pin(vma, 0, 0, PIN_USER);
    if (err)
    goto err_unpin;
    i915_gem_object_unpin_map(obj);
    return vma;
    err_unpin:
    i915_gem_object_unpin_map(obj);
    err_put:
    i915_gem_object_put(obj);
    return ERR_PTR(err);
    }
#[no_mangle]
unsafe extern "C" fn trifilter(a: *mut u32) -> u32 {
    static u32 trifilter(u32 *a)
    {
    u64 sum;
    sort(a, COUNT, sizeof(*a), cmp_u32, core::ptr::null_mut());
    sum = mul_u32_u32(a[2], 2);
    sum += a[1];
    sum += a[3];
    return sum >> 2;
    }
#[no_mangle]
unsafe extern "C" fn perf_mi_bb_start(arg: *mut c_void) -> c_int {
    static int perf_mi_bb_start(void *arg)
    {
    struct intel_gt *gt = arg;
    struct intel_engine_cs *engine;
    enum intel_engine_id id;
    intel_wakeref_t wakeref;
    let mut err: c_int = 0;
    if (GRAPHICS_VER(gt.i915) < 4) /* Any CS_TIMESTAMP? */
    return 0;
    wakeref = perf_begin(gt);
    for_each_engine(engine, gt, id) {
    struct intel_context *ce = engine.kernel_context;
    struct i915_vma *batch;
    u32 cycles[COUNT];
    int i;
    if (GRAPHICS_VER(engine.i915) < 7 && engine.id != RCS0)
    continue;
    intel_engine_pm_get(engine);
    batch = create_empty_batch(ce);
    if (IS_ERR(batch)) {
    err = PTR_ERR(batch);
    intel_engine_pm_put(engine);
    break;
    }
    err = i915_vma_sync(batch);
    if (err) {
    intel_engine_pm_put(engine);
    i915_vma_put(batch);
    break;
    }
    for (i = 0; i < ARRAY_SIZE(cycles); i++) {
    struct i915_request *rq;
    rq = i915_request_create(ce);
    if (IS_ERR(rq)) {
    err = PTR_ERR(rq);
    break;
    }
    err = write_timestamp(rq, 2);
    if (err)
    goto out;
    err = rq.engine.emit_bb_start(rq,
    i915_vma_offset(batch), 8,
    0);
    if (err)
    goto out;
    err = write_timestamp(rq, 3);
    if (err)
    goto out;
    out:
    i915_request_get(rq);
    i915_request_add(rq);
    if (i915_request_wait(rq, 0, HZ / 5) < 0)
    err = -EIO;
    i915_request_put(rq);
    if (err)
    break;
    cycles[i] = rq.hwsp_seqno[3] - rq.hwsp_seqno[2];
    }
    i915_vma_put(batch);
    intel_engine_pm_put(engine);
    if (err)
    break;
    pr_info("%s: MI_BB_START cycles: %u\n",
    engine.name, trifilter(cycles));
    }
    if (perf_end(gt, wakeref))
    err = -EIO;
    return err;
    }
    static struct i915_vma *create_nop_batch(struct intel_context *ce)
    {
    struct drm_i915_gem_object *obj;
    struct i915_vma *vma;
    u32 *cs;
    int err;
    obj = i915_gem_object_create_internal(ce.engine.i915, SZ_64K);
    if (IS_ERR(obj))
    return ERR_CAST(obj);
    cs = i915_gem_object_pin_map_unlocked(obj, I915_MAP_WB);
    if (IS_ERR(cs)) {
    err = PTR_ERR(cs);
    goto err_put;
    }
    memset(cs, 0, SZ_64K);
    cs[SZ_64K / sizeof(*cs) - 1] = MI_BATCH_BUFFER_END;
    i915_gem_object_flush_map(obj);
    vma = i915_vma_instance(obj, ce.vm, core::ptr::null_mut());
    if (IS_ERR(vma)) {
    err = PTR_ERR(vma);
    goto err_unpin;
    }
    err = i915_vma_pin(vma, 0, 0, PIN_USER);
    if (err)
    goto err_unpin;
    i915_gem_object_unpin_map(obj);
    return vma;
    err_unpin:
    i915_gem_object_unpin_map(obj);
    err_put:
    i915_gem_object_put(obj);
    return ERR_PTR(err);
    }
#[no_mangle]
unsafe extern "C" fn perf_mi_noop(arg: *mut c_void) -> c_int {
    static int perf_mi_noop(void *arg)
    {
    struct intel_gt *gt = arg;
    struct intel_engine_cs *engine;
    enum intel_engine_id id;
    intel_wakeref_t wakeref;
    let mut err: c_int = 0;
    if (GRAPHICS_VER(gt.i915) < 4) /* Any CS_TIMESTAMP? */
    return 0;
    wakeref = perf_begin(gt);
    for_each_engine(engine, gt, id) {
    struct intel_context *ce = engine.kernel_context;
    struct i915_vma *base, *nop;
    u32 cycles[COUNT];
    int i;
    if (GRAPHICS_VER(engine.i915) < 7 && engine.id != RCS0)
    continue;
    intel_engine_pm_get(engine);
    base = create_empty_batch(ce);
    if (IS_ERR(base)) {
    err = PTR_ERR(base);
    intel_engine_pm_put(engine);
    break;
    }
    err = i915_vma_sync(base);
    if (err) {
    i915_vma_put(base);
    intel_engine_pm_put(engine);
    break;
    }
    nop = create_nop_batch(ce);
    if (IS_ERR(nop)) {
    err = PTR_ERR(nop);
    i915_vma_put(base);
    intel_engine_pm_put(engine);
    break;
    }
    err = i915_vma_sync(nop);
    if (err) {
    i915_vma_put(nop);
    i915_vma_put(base);
    intel_engine_pm_put(engine);
    break;
    }
    for (i = 0; i < ARRAY_SIZE(cycles); i++) {
    struct i915_request *rq;
    rq = i915_request_create(ce);
    if (IS_ERR(rq)) {
    err = PTR_ERR(rq);
    break;
    }
    err = write_timestamp(rq, 2);
    if (err)
    goto out;
    err = rq.engine.emit_bb_start(rq,
    i915_vma_offset(base), 8,
    0);
    if (err)
    goto out;
    err = write_timestamp(rq, 3);
    if (err)
    goto out;
    err = rq.engine.emit_bb_start(rq,
    i915_vma_offset(nop),
    i915_vma_size(nop),
    0);
    if (err)
    goto out;
    err = write_timestamp(rq, 4);
    if (err)
    goto out;
    out:
    i915_request_get(rq);
    i915_request_add(rq);
    if (i915_request_wait(rq, 0, HZ / 5) < 0)
    err = -EIO;
    i915_request_put(rq);
    if (err)
    break;
    cycles[i] =
    (rq.hwsp_seqno[4] - rq.hwsp_seqno[3]) -
    (rq.hwsp_seqno[3] - rq.hwsp_seqno[2]);
    }
    i915_vma_put(nop);
    i915_vma_put(base);
    intel_engine_pm_put(engine);
    if (err)
    break;
    pr_info("%s: 16K MI_NOOP cycles: %u\n",
    engine.name, trifilter(cycles));
    }
    if (perf_end(gt, wakeref))
    err = -EIO;
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn intel_engine_cs_perf_selftests(i915: *mut drm_i915_private) -> c_int {
    int intel_engine_cs_perf_selftests(struct drm_i915_private *i915)
    {
    static const struct i915_subtest tests[] = {
    SUBTEST(perf_mi_bb_start),
    SUBTEST(perf_mi_noop),
    };
    if (intel_gt_is_wedged(to_gt(i915)))
    return 0;
    return intel_gt_live_subtests(tests, to_gt(i915));
    }
#[no_mangle]
unsafe extern "C" fn intel_mmio_bases_check(arg: *mut c_void) -> c_int {
    static int intel_mmio_bases_check(void *arg)
    {
    int i, j;
    for (i = 0; i < ARRAY_SIZE(intel_engines); i++) {
    const struct engine_info *info = &intel_engines[i];
    let mut prev: u8 = U8_MAX;
    for (j = 0; j < MAX_MMIO_BASES; j++) {
    let mut ver: u8 = info.mmio_bases[j].graphics_ver;
    let mut base: u32 = info.mmio_bases[j].base;
    if (ver >= prev) {
    pr_err("%s(%s, class:%d, instance:%d): mmio base for graphics ver %u is before the one for ver %u\n",
    __func__,
    intel_engine_class_repr(info.class),
    info.class, info.instance,
    prev, ver);
    return -EINVAL;
    }
    if (ver == 0)
    break;
    if (!base) {
    pr_err("%s(%s, class:%d, instance:%d): invalid mmio base (%x) for graphics ver %u at entry %u\n",
    __func__,
    intel_engine_class_repr(info.class),
    info.class, info.instance,
    base, ver, j);
    return -EINVAL;
    }
    prev = ver;
    }
    pr_debug("%s: min graphics version supported for %s%d is %u\n",
    __func__,
    intel_engine_class_repr(info.class),
    info.instance,
    prev);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn intel_engine_cs_mock_selftests() -> c_int {
    int intel_engine_cs_mock_selftests(void)
    {
    static const struct i915_subtest tests[] = {
    SUBTEST(intel_mmio_bases_check),
    };
    return i915_subtests(tests, core::ptr::null_mut());
    }
