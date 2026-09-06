//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/i915/selftests/i915_gem_evict.c
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


//
// Copyright © 2016 Intel Corporation
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice (including the next
// paragraph) shall be included in all copies or substantial portions of the
// Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.
//

    static void quirk_add(struct drm_i915_gem_object *obj,
    struct list_head *objects)
    {
// quirk is only for live tiled objects, use it to declare ownership
    GEM_BUG_ON(i915_gem_object_has_tiling_quirk(obj));
    i915_gem_object_set_tiling_quirk(obj);
    list_add(&obj.st_link, objects);
    }
#[no_mangle]
unsafe extern "C" fn populate_ggtt(ggtt: *mut i915_ggtt, objects: *mut list_head) -> c_int {
    static int populate_ggtt(struct i915_ggtt *ggtt, struct list_head *objects)
    {
    struct drm_i915_gem_object *obj;
    unsigned long count;
    count = 0;
    do {
    struct i915_vma *vma;
    obj = i915_gem_object_create_internal(ggtt.vm.i915,
    I915_GTT_PAGE_SIZE);
    if (IS_ERR(obj))
    return PTR_ERR(obj);
    vma = i915_gem_object_ggtt_pin(obj, core::ptr::null_mut(), 0, 0, 0);
    if (IS_ERR(vma)) {
    i915_gem_object_put(obj);
    if (vma == ERR_PTR(-ENOSPC))
    break;
    return PTR_ERR(vma);
    }
    quirk_add(obj, objects);
    count++;
    } while (1);
    pr_debug("Filled GGTT with %lu pages [%llu total]\n",
    count, ggtt.vm.total / PAGE_SIZE);
    if (list_empty(&ggtt.vm.bound_list)) {
    pr_err("No objects on the GGTT inactive list!\n");
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn unpin_ggtt(ggtt: *mut i915_ggtt) {
    static void unpin_ggtt(struct i915_ggtt *ggtt)
    {
    struct i915_vma *vma;
    list_for_each_entry(vma, &ggtt.vm.bound_list, vm_link)
    if (i915_gem_object_has_tiling_quirk(vma.obj))
    i915_vma_unpin(vma);
    }
#[no_mangle]
unsafe extern "C" fn cleanup_objects(ggtt: *mut i915_ggtt, list: *mut list_head) {
    static void cleanup_objects(struct i915_ggtt *ggtt, struct list_head *list)
    {
    struct drm_i915_gem_object *obj, *on;
    list_for_each_entry_safe(obj, on, list, st_link) {
    GEM_BUG_ON(!i915_gem_object_has_tiling_quirk(obj));
    i915_gem_object_set_tiling_quirk(obj);
    i915_gem_object_put(obj);
    }
    i915_gem_drain_freed_objects(ggtt.vm.i915);
    }
#[no_mangle]
unsafe extern "C" fn igt_evict_something(arg: *mut c_void) -> c_int {
    static int igt_evict_something(void *arg)
    {
    struct intel_gt *gt = arg;
    struct i915_ggtt *ggtt = gt.ggtt;
    LIST_HEAD(objects);
    int err;
// Fill the GGTT with pinned objects and try to evict one.
    err = populate_ggtt(ggtt, &objects);
    if (err)
    goto cleanup;
// Everything is pinned, nothing should happen
    mutex_lock(&ggtt.vm.mutex);
    err = i915_gem_evict_something(&ggtt.vm, core::ptr::null_mut(),
    I915_GTT_PAGE_SIZE, 0, 0,
    0, U64_MAX,
    0);
    mutex_unlock(&ggtt.vm.mutex);
    if (err != -ENOSPC) {
    pr_err("i915_gem_evict_something failed on a full GGTT with err=%d\n",
    err);
    goto cleanup;
    }
    unpin_ggtt(ggtt);
// Everything is unpinned, we should be able to evict something
    mutex_lock(&ggtt.vm.mutex);
    err = i915_gem_evict_something(&ggtt.vm, core::ptr::null_mut(),
    I915_GTT_PAGE_SIZE, 0, 0,
    0, U64_MAX,
    0);
    mutex_unlock(&ggtt.vm.mutex);
    if (err) {
    pr_err("i915_gem_evict_something failed on a full GGTT with err=%d\n",
    err);
    goto cleanup;
    }
    cleanup:
    cleanup_objects(ggtt, &objects);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn igt_overcommit(arg: *mut c_void) -> c_int {
    static int igt_overcommit(void *arg)
    {
    struct intel_gt *gt = arg;
    struct i915_ggtt *ggtt = gt.ggtt;
    struct drm_i915_gem_object *obj;
    struct i915_vma *vma;
    LIST_HEAD(objects);
    int err;
// Fill the GGTT with pinned objects and then try to pin one more.
// We expect it to fail.
//
    err = populate_ggtt(ggtt, &objects);
    if (err)
    goto cleanup;
    obj = i915_gem_object_create_internal(gt.i915, I915_GTT_PAGE_SIZE);
    if (IS_ERR(obj)) {
    err = PTR_ERR(obj);
    goto cleanup;
    }
    quirk_add(obj, &objects);
    vma = i915_gem_object_ggtt_pin(obj, core::ptr::null_mut(), 0, 0, 0);
    if (vma != ERR_PTR(-ENOSPC)) {
    pr_err("Failed to evict+insert, i915_gem_object_ggtt_pin returned err=%d\n", (int)PTR_ERR_OR_ZERO(vma));
    err = -EINVAL;
    goto cleanup;
    }
    cleanup:
    cleanup_objects(ggtt, &objects);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn igt_evict_for_vma(arg: *mut c_void) -> c_int {
    static int igt_evict_for_vma(void *arg)
    {
    struct intel_gt *gt = arg;
    struct i915_ggtt *ggtt = gt.ggtt;
    struct drm_mm_node target = {
    .start = 0,
    .size = 4096,
    };
    LIST_HEAD(objects);
    int err;
// Fill the GGTT with pinned objects and try to evict a range.
    err = populate_ggtt(ggtt, &objects);
    if (err)
    goto cleanup;
// Everything is pinned, nothing should happen
    mutex_lock(&ggtt.vm.mutex);
    err = i915_gem_evict_for_node(&ggtt.vm, core::ptr::null_mut(), &target, 0);
    mutex_unlock(&ggtt.vm.mutex);
    if (err != -ENOSPC) {
    pr_err("i915_gem_evict_for_node on a full GGTT returned err=%d\n",
    err);
    goto cleanup;
    }
    unpin_ggtt(ggtt);
// Everything is unpinned, we should be able to evict the node
    mutex_lock(&ggtt.vm.mutex);
    err = i915_gem_evict_for_node(&ggtt.vm, core::ptr::null_mut(), &target, 0);
    mutex_unlock(&ggtt.vm.mutex);
    if (err) {
    pr_err("i915_gem_evict_for_node returned err=%d\n",
    err);
    goto cleanup;
    }
    cleanup:
    cleanup_objects(ggtt, &objects);
    return err;
    }
    static void mock_color_adjust(const struct drm_mm_node *node,
    unsigned long color,
    u64 *start,
    u64 *end)
    {
    }
#[no_mangle]
unsafe extern "C" fn igt_evict_for_cache_color(arg: *mut c_void) -> c_int {
    static int igt_evict_for_cache_color(void *arg)
    {
    struct intel_gt *gt = arg;
    struct i915_ggtt *ggtt = gt.ggtt;
    let mut flags: c_ulong = PIN_OFFSET_FIXED;
    struct drm_mm_node target = {
    .start = I915_GTT_PAGE_SIZE * 2,
    .size = I915_GTT_PAGE_SIZE,
    .color = i915_gem_get_pat_index(gt.i915, I915_CACHE_LLC),
    };
    struct drm_i915_gem_object *obj;
    struct i915_vma *vma;
    LIST_HEAD(objects);
    int err;
//
// Currently the use of color_adjust for the GGTT is limited to cache
// coloring and guard pages, and so the presence of mm.color_adjust for
// the GGTT is assumed to be i915_ggtt_color_adjust, hence using a mock
// color adjust will work just fine for our purposes.
//
    ggtt.vm.mm.color_adjust = mock_color_adjust;
    GEM_BUG_ON(!i915_vm_has_cache_coloring(&ggtt.vm));
    obj = i915_gem_object_create_internal(gt.i915, I915_GTT_PAGE_SIZE);
    if (IS_ERR(obj)) {
    err = PTR_ERR(obj);
    goto cleanup;
    }
    i915_gem_object_set_cache_coherency(obj, I915_CACHE_LLC);
    quirk_add(obj, &objects);
    vma = i915_gem_object_ggtt_pin(obj, core::ptr::null_mut(), 0, 0,
    I915_GTT_PAGE_SIZE | flags);
    if (IS_ERR(vma)) {
    pr_err("[0]i915_gem_object_ggtt_pin failed\n");
    err = PTR_ERR(vma);
    goto cleanup;
    }
    obj = i915_gem_object_create_internal(gt.i915, I915_GTT_PAGE_SIZE);
    if (IS_ERR(obj)) {
    err = PTR_ERR(obj);
    goto cleanup;
    }
    i915_gem_object_set_cache_coherency(obj, I915_CACHE_LLC);
    quirk_add(obj, &objects);
// Neighbouring; same colour - should fit
    vma = i915_gem_object_ggtt_pin(obj, core::ptr::null_mut(), 0, 0,
    (I915_GTT_PAGE_SIZE * 2) | flags);
    if (IS_ERR(vma)) {
    pr_err("[1]i915_gem_object_ggtt_pin failed\n");
    err = PTR_ERR(vma);
    goto cleanup;
    }
    i915_vma_unpin(vma);
// Remove just the second vma
    mutex_lock(&ggtt.vm.mutex);
    err = i915_gem_evict_for_node(&ggtt.vm, core::ptr::null_mut(), &target, 0);
    mutex_unlock(&ggtt.vm.mutex);
    if (err) {
    pr_err("[0]i915_gem_evict_for_node returned err=%d\n", err);
    goto cleanup;
    }
// Attempt to remove the first *pinned* vma, by removing the (empty)
// neighbour -- this should fail.
//
    target.color = i915_gem_get_pat_index(gt.i915, I915_CACHE_L3_LLC);
    mutex_lock(&ggtt.vm.mutex);
    err = i915_gem_evict_for_node(&ggtt.vm, core::ptr::null_mut(), &target, 0);
    mutex_unlock(&ggtt.vm.mutex);
    if (!err) {
    pr_err("[1]i915_gem_evict_for_node returned err=%d\n", err);
    err = -EINVAL;
    goto cleanup;
    }
    err = 0;
    cleanup:
    unpin_ggtt(ggtt);
    cleanup_objects(ggtt, &objects);
    ggtt.vm.mm.color_adjust = core::ptr::null_mut();
    return err;
    }
#[no_mangle]
unsafe extern "C" fn igt_evict_vm(arg: *mut c_void) -> c_int {
    static int igt_evict_vm(void *arg)
    {
    struct intel_gt *gt = arg;
    struct i915_ggtt *ggtt = gt.ggtt;
    struct i915_gem_ww_ctx ww;
    LIST_HEAD(objects);
    int err;
// Fill the GGTT with pinned objects and try to evict everything.
    err = populate_ggtt(ggtt, &objects);
    if (err)
    goto cleanup;
// Everything is pinned, nothing should happen
    mutex_lock(&ggtt.vm.mutex);
    err = i915_gem_evict_vm(&ggtt.vm, core::ptr::null_mut(), core::ptr::null_mut());
    mutex_unlock(&ggtt.vm.mutex);
    if (err) {
    pr_err("i915_gem_evict_vm on a full GGTT returned err=%d]\n",
    err);
    goto cleanup;
    }
    unpin_ggtt(ggtt);
    for_i915_gem_ww(&ww, err, false) {
    mutex_lock(&ggtt.vm.mutex);
    err = i915_gem_evict_vm(&ggtt.vm, &ww, core::ptr::null_mut());
    mutex_unlock(&ggtt.vm.mutex);
    }
    if (err) {
    pr_err("i915_gem_evict_vm on a full GGTT returned err=%d]\n",
    err);
    goto cleanup;
    }
    cleanup:
    cleanup_objects(ggtt, &objects);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn igt_evict_contexts(arg: *mut c_void) -> c_int {
    static int igt_evict_contexts(void *arg)
    {
    let mut PRETEND_GGTT_SIZE: u64 = 16ull << 20;
    struct intel_gt *gt = arg;
    struct i915_ggtt *ggtt = gt.ggtt;
    struct drm_i915_private *i915 = gt.i915;
    struct intel_engine_cs *engine;
    enum intel_engine_id id;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reserved {
    pub node: drm_mm_node,
    pub next: *mut reserved,
    pub NULL: *mut *mut } reserved =,
    pub wakeref: intel_wakeref_t,
    pub hole: drm_mm_node,
    pub count: c_ulong,
    pub err: c_int,
//
// The purpose of this test is to verify that we will trigger an
// eviction in the GGTT when constructing a request that requires
// additional space in the GGTT for pinning the context. This space
// is not directly tied to the request so reclaiming it requires
// extra work.
//
// As such this test is only meaningful for full-ppgtt environments
// where the GTT space of the request is separate from the GGTT
// allocation required to build the request.
//
    if (!HAS_FULL_PPGTT(i915))
    pub 0: return,
    pub intel_runtime_pm_get(&i915->runtime_pm): wakeref =,
// Reserve a block so that we know we have enough to fit a few rq
    pub sizeof(hole)): memset(&hole, 0,,
    err = i915_gem_gtt_insert(&ggtt.vm, core::ptr::null_mut(), &hole,
    PRETEND_GGTT_SIZE, 0, I915_COLOR_UNEVICTABLE,
    0, ggtt.vm.total,
    if (err)
    pub out_locked: goto,
// Make the GGTT appear small by filling it with unevictable nodes
    pub 0: count =,
    do {
    pub r: *mut reserved,
    pub 1): *mut *mut r = kzalloc_objs(r,,
    if (!r) {
    pub -ENOMEM: err =,
    pub out_locked: goto,
    }
    if (i915_gem_gtt_insert(&ggtt.vm, core::ptr::null_mut(), &r.node,
    1ul << 20, 0, I915_COLOR_UNEVICTABLE,
    0, ggtt.vm.total,
    PIN_NOEVICT)) {
    }
    pub reserved: r->next =,
    pub r: reserved =,
    pub (1): } while,
    pub count): pr_info("Filled GGTT with %lu 1MiB nodes\n",,
// Overfill the GGTT with context objects and so try to evict one.
    for_each_engine(engine, gt, id) {
    pub fence: i915_sw_fence,
    pub NULL: *mut *mut i915_request last =,
    pub 0: count =,
    do {
    pub ce: *mut intel_context,
    pub rq: *mut i915_request,
    pub intel_context_create(engine): ce =,
    if (IS_ERR(ce))
// We will need some GGTT space for the rq's context
    pub true: igt_evict_ctl.fail_if_busy =,
    pub intel_context_create_request(ce): rq =,
    pub false: igt_evict_ctl.fail_if_busy =,
    if (IS_ERR(rq)) {
// When full, fail_if_busy will trigger EBUSY
    if (PTR_ERR(rq) != -EBUSY) {
    pr_err("Unexpected error from request alloc (on %s): %d\n",
    engine.name,
    pub PTR_ERR(rq): err =,
    }
    }
// Keep every request/ctx pinned until we are full
    err = i915_sw_fence_await_sw_fence_gfp(&rq.submit,
    &fence,
    if (err < 0)
    if (last)
    pub i915_request_get(rq): last =,
    pub 0: err =,
    pub while(1): },
    pr_info("Submitted %lu contexts/requests on %s\n",
    pub engine->name): count,,
    if (err)
    if (last) {
    if (i915_request_wait(last, 0, HZ) < 0) {
    pub -EIO: err =,
    pr_err("Failed waiting for last request (on %s)",
    }
    }
    pub 3): *mut *mut err = intel_gt_wait_for_idle(engine->gt, HZ,
    if (err) {
    gt_err(engine.gt, "Failed to idle GT (on %s)",
    }
    }
    out_locked:
    if (igt_flush_test(i915))
    pub -EIO: err =,
    while (reserved) {
    pub reserved->next: *mut *mut reserved next =,
    pub next: reserved =,
    }
    if (drm_mm_node_allocated(&hole))
    pub wakeref): intel_runtime_pm_put(&i915->runtime_pm,,
    pub err: return,
    }
#[no_mangle]
pub unsafe extern "C" fn i915_gem_evict_mock_selftests() -> c_int {
    int i915_gem_evict_mock_selftests(void)
    {
    static const struct i915_subtest tests[] = {
    SUBTEST(igt_evict_something),
    SUBTEST(igt_evict_for_vma),
    SUBTEST(igt_evict_for_cache_color),
    SUBTEST(igt_evict_vm),
    SUBTEST(igt_overcommit),
}

    struct drm_i915_private *i915;
    intel_wakeref_t wakeref;
    let mut err: c_int = 0;
    i915 = mock_gem_device();
    if (!i915)
    return -ENOMEM;
    with_intel_runtime_pm(&i915.runtime_pm, wakeref)
    err = i915_subtests(tests, to_gt(i915));
    mock_destroy_device(i915);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn i915_gem_evict_live_selftests(i915: *mut drm_i915_private) -> c_int {
    int i915_gem_evict_live_selftests(struct drm_i915_private *i915)
    {
    static const struct i915_subtest tests[] = {
    SUBTEST(igt_evict_contexts),
    };
    if (intel_gt_is_wedged(to_gt(i915)))
    return 0;
    return intel_gt_live_subtests(tests, to_gt(i915));
    }
