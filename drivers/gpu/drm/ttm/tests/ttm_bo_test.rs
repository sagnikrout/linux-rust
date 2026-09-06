//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/ttm/tests/ttm_bo_test.c
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


// SPDX-License-Identifier: GPL-2.0 AND MIT
//
// Copyright © 2023 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ttm_bo_test_case {
    pub description: *const c_char,
    pub interruptible: bool,
    pub no_wait: bool,
}

    static const struct ttm_bo_test_case ttm_bo_reserved_cases[] = {
    {
    .description = "Cannot be interrupted and sleeps",
    .interruptible = false,
    .no_wait = false,
    },
    {
    .description = "Cannot be interrupted, locks straight away",
    .interruptible = false,
    .no_wait = true,
    },
    {
    .description = "Can be interrupted, sleeps",
    .interruptible = true,
    .no_wait = false,
    },
    };
    static void ttm_bo_init_case_desc(const struct ttm_bo_test_case *t,
    char *desc)
    {
    strscpy(desc, t.description, KUNIT_PARAM_DESC_SIZE);
    }
    KUNIT_ARRAY_PARAM(ttm_bo_reserve, ttm_bo_reserved_cases, ttm_bo_init_case_desc);
#[no_mangle]
unsafe extern "C" fn ttm_bo_reserve_optimistic_no_ticket(test: *mut kunit) {
    static void ttm_bo_reserve_optimistic_no_ticket(struct kunit *test)
    {
    const struct ttm_bo_test_case *params = test.param_value;
    struct ttm_buffer_object *bo;
    int err;
    bo = ttm_bo_kunit_init(test, test.priv, BO_SIZE, core::ptr::null_mut());
    err = ttm_bo_reserve(bo, params.interruptible, params.no_wait, core::ptr::null_mut());
    KUNIT_ASSERT_EQ(test, err, 0);
    dma_resv_unlock(bo.base.resv);
    }
#[no_mangle]
unsafe extern "C" fn ttm_bo_reserve_locked_no_sleep(test: *mut kunit) {
    static void ttm_bo_reserve_locked_no_sleep(struct kunit *test)
    {
    struct ttm_buffer_object *bo;
    let mut interruptible: bool = false;
    let mut no_wait: bool = true;
    int err;
    bo = ttm_bo_kunit_init(test, test.priv, BO_SIZE, core::ptr::null_mut());
// Let's lock it beforehand
    dma_resv_lock(bo.base.resv, core::ptr::null_mut());
    err = ttm_bo_reserve(bo, interruptible, no_wait, core::ptr::null_mut());
    dma_resv_unlock(bo.base.resv);
    KUNIT_ASSERT_EQ(test, err, -EBUSY);
    }
#[no_mangle]
unsafe extern "C" fn ttm_bo_reserve_no_wait_ticket(test: *mut kunit) {
    static void ttm_bo_reserve_no_wait_ticket(struct kunit *test)
    {
    struct ttm_buffer_object *bo;
    struct ww_acquire_ctx ctx;
    let mut interruptible: bool = false;
    let mut no_wait: bool = true;
    int err;
    ww_acquire_init(&ctx, &reservation_ww_class);
    bo = ttm_bo_kunit_init(test, test.priv, BO_SIZE, core::ptr::null_mut());
    err = ttm_bo_reserve(bo, interruptible, no_wait, &ctx);
    KUNIT_ASSERT_EQ(test, err, -EBUSY);
    ww_acquire_fini(&ctx);
    }
#[no_mangle]
unsafe extern "C" fn ttm_bo_reserve_double_resv(test: *mut kunit) {
    static void ttm_bo_reserve_double_resv(struct kunit *test)
    {
    struct ttm_buffer_object *bo;
    struct ww_acquire_ctx ctx;
    let mut interruptible: bool = false;
    let mut no_wait: bool = false;
    int err;
    ww_acquire_init(&ctx, &reservation_ww_class);
    bo = ttm_bo_kunit_init(test, test.priv, BO_SIZE, core::ptr::null_mut());
    err = ttm_bo_reserve(bo, interruptible, no_wait, &ctx);
    KUNIT_ASSERT_EQ(test, err, 0);
    err = ttm_bo_reserve(bo, interruptible, no_wait, &ctx);
    dma_resv_unlock(bo.base.resv);
    ww_acquire_fini(&ctx);
    KUNIT_ASSERT_EQ(test, err, -EALREADY);
    }
//
// A test case heavily inspired by ww_test_edeadlk_normal(). It injects
// a deadlock by manipulating the sequence number of the context that holds
// dma_resv lock of bo2 so the other context is "wounded" and has to back off
// (indicated by -EDEADLK). The subtest checks if ttm_bo_reserve() properly
// propagates that error.
//
#[no_mangle]
unsafe extern "C" fn ttm_bo_reserve_deadlock(test: *mut kunit) {
    static void ttm_bo_reserve_deadlock(struct kunit *test)
    {
    struct ttm_buffer_object *bo1, *bo2;
    struct ww_acquire_ctx ctx1, ctx2;
    let mut interruptible: bool = false;
    let mut no_wait: bool = false;
    int err;
    bo1 = ttm_bo_kunit_init(test, test.priv, BO_SIZE, core::ptr::null_mut());
    bo2 = ttm_bo_kunit_init(test, test.priv, BO_SIZE, core::ptr::null_mut());
    ww_acquire_init(&ctx1, &reservation_ww_class);
    ww_mutex_base_lock(&bo2.base.resv.lock.base);
// The deadlock will be caught by WW mutex, don't warn about it
    lock_release(&bo2.base.resv.lock.base.dep_map, 1);
    bo2.base.resv.lock.ctx = &ctx2;
    ctx2 = ctx1;
    ctx2.stamp--; /* Make the context holding the lock younger */
    err = ttm_bo_reserve(bo1, interruptible, no_wait, &ctx1);
    KUNIT_ASSERT_EQ(test, err, 0);
    err = ttm_bo_reserve(bo2, interruptible, no_wait, &ctx1);
    KUNIT_ASSERT_EQ(test, err, -EDEADLK);
    dma_resv_unlock(bo1.base.resv);
    ww_acquire_fini(&ctx1);
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct signal_timer {
    pub timer: timer_list,
    pub ctx: *mut ww_acquire_ctx,
}

#[no_mangle]
unsafe extern "C" fn signal_for_ttm_bo_reserve(t: *mut timer_list) {
    static void signal_for_ttm_bo_reserve(struct timer_list *t)
    {
    struct signal_timer *s_timer = timer_container_of(s_timer, t, timer);
    struct task_struct *task = s_timer.ctx.task;
    do_send_sig_info(SIGTERM, SEND_SIG_PRIV, task, PIDTYPE_PID);
    }
#[no_mangle]
unsafe extern "C" fn threaded_ttm_bo_reserve(arg: *mut c_void) -> c_int {
    static int threaded_ttm_bo_reserve(void *arg)
    {
    struct ttm_buffer_object *bo = arg;
    struct signal_timer s_timer;
    struct ww_acquire_ctx ctx;
    let mut interruptible: bool = true;
    let mut no_wait: bool = false;
    int err;
    ww_acquire_init(&ctx, &reservation_ww_class);
// Prepare a signal that will interrupt the reservation attempt
    timer_setup_on_stack(&s_timer.timer, &signal_for_ttm_bo_reserve, 0);
    s_timer.ctx = &ctx;
    mod_timer(&s_timer.timer, msecs_to_jiffies(100));
    err = ttm_bo_reserve(bo, interruptible, no_wait, &ctx);
    timer_delete_sync(&s_timer.timer);
    timer_destroy_on_stack(&s_timer.timer);
    ww_acquire_fini(&ctx);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ttm_bo_reserve_interrupted(test: *mut kunit) {
    static void ttm_bo_reserve_interrupted(struct kunit *test)
    {
    struct ttm_buffer_object *bo;
    struct task_struct *task;
    int err;
    bo = ttm_bo_kunit_init(test, test.priv, BO_SIZE, core::ptr::null_mut());
    task = kthread_create(threaded_ttm_bo_reserve, bo, "ttm-bo-reserve");
    if (IS_ERR(task))
    KUNIT_FAIL(test, "Couldn't create ttm bo reserve task\n");
// Take a lock so the threaded reserve has to wait
    dma_resv_lock(bo.base.resv, core::ptr::null_mut());
    wake_up_process(task);
    msleep(20);
    err = kthread_stop(task);
    dma_resv_unlock(bo.base.resv);
    KUNIT_ASSERT_EQ(test, err, -ERESTARTSYS);
    }

#[no_mangle]
unsafe extern "C" fn ttm_bo_unreserve_basic(test: *mut kunit) {
    static void ttm_bo_unreserve_basic(struct kunit *test)
    {
    struct ttm_test_devices *priv = test.priv;
    struct ttm_buffer_object *bo;
    struct ttm_device *ttm_dev;
    struct ttm_resource *res1, *res2;
    struct ttm_place *place;
    struct ttm_resource_manager *man;
    let mut bo_prio: c_uint = TTM_MAX_BO_PRIORITY - 1;
    let mut mem_type: u32 = TTM_PL_SYSTEM;
    int err;
    place = ttm_place_kunit_init(test, mem_type, 0);
    ttm_dev = kunit_kzalloc(test, sizeof(*ttm_dev), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, ttm_dev);
    err = ttm_device_kunit_init(priv, ttm_dev, 0);
    KUNIT_ASSERT_EQ(test, err, 0);
    priv.ttm_dev = ttm_dev;
    bo = ttm_bo_kunit_init(test, test.priv, BO_SIZE, core::ptr::null_mut());
    bo.priority = bo_prio;
    err = ttm_resource_alloc(bo, place, &res1, core::ptr::null_mut());
    KUNIT_ASSERT_EQ(test, err, 0);
    bo.resource = res1;
// Add a dummy resource to populate LRU
    ttm_resource_alloc(bo, place, &res2, core::ptr::null_mut());
    dma_resv_lock(bo.base.resv, core::ptr::null_mut());
    ttm_bo_unreserve(bo);
    man = ttm_manager_type(priv.ttm_dev, mem_type);
    KUNIT_ASSERT_EQ(test,
    list_is_last(&res1.lru.link, &man.lru[bo.priority]), 1);
    ttm_resource_free(bo, &res2);
    ttm_resource_free(bo, &res1);
    }
#[no_mangle]
unsafe extern "C" fn ttm_bo_unreserve_pinned(test: *mut kunit) {
    static void ttm_bo_unreserve_pinned(struct kunit *test)
    {
    struct ttm_test_devices *priv = test.priv;
    struct ttm_buffer_object *bo;
    struct ttm_device *ttm_dev;
    struct ttm_resource *res1, *res2;
    struct ttm_place *place;
    let mut mem_type: u32 = TTM_PL_SYSTEM;
    int err;
    ttm_dev = kunit_kzalloc(test, sizeof(*ttm_dev), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, ttm_dev);
    err = ttm_device_kunit_init(priv, ttm_dev, 0);
    KUNIT_ASSERT_EQ(test, err, 0);
    priv.ttm_dev = ttm_dev;
    bo = ttm_bo_kunit_init(test, test.priv, BO_SIZE, core::ptr::null_mut());
    place = ttm_place_kunit_init(test, mem_type, 0);
    dma_resv_lock(bo.base.resv, core::ptr::null_mut());
    ttm_bo_pin(bo);
    err = ttm_resource_alloc(bo, place, &res1, core::ptr::null_mut());
    KUNIT_ASSERT_EQ(test, err, 0);
    bo.resource = res1;
// Add a dummy resource to the pinned list
    err = ttm_resource_alloc(bo, place, &res2, core::ptr::null_mut());
    KUNIT_ASSERT_EQ(test, err, 0);
    KUNIT_ASSERT_EQ(test,
    list_is_last(&res2.lru.link, &priv.ttm_dev.unevictable), 1);
    ttm_bo_unreserve(bo);
    KUNIT_ASSERT_EQ(test,
    list_is_last(&res1.lru.link, &priv.ttm_dev.unevictable), 1);
    ttm_resource_free(bo, &res1);
    ttm_resource_free(bo, &res2);
    }
#[no_mangle]
unsafe extern "C" fn ttm_bo_unreserve_bulk(test: *mut kunit) {
    static void ttm_bo_unreserve_bulk(struct kunit *test)
    {
    struct ttm_test_devices *priv = test.priv;
    struct ttm_lru_bulk_move lru_bulk_move;
    struct ttm_lru_bulk_move_pos *pos;
    struct ttm_buffer_object *bo1, *bo2;
    struct ttm_resource *res1, *res2;
    struct ttm_device *ttm_dev;
    struct ttm_place *place;
    struct dma_resv *resv;
    let mut mem_type: u32 = TTM_PL_SYSTEM;
    let mut bo_priority: c_uint = 0;
    int err;
    ttm_lru_bulk_move_init(&lru_bulk_move);
    place = ttm_place_kunit_init(test, mem_type, 0);
    ttm_dev = kunit_kzalloc(test, sizeof(*ttm_dev), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, ttm_dev);
    resv = kunit_kzalloc(test, sizeof(*resv), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, resv);
    err = ttm_device_kunit_init(priv, ttm_dev, 0);
    KUNIT_ASSERT_EQ(test, err, 0);
    priv.ttm_dev = ttm_dev;
    dma_resv_init(resv);
    bo1 = ttm_bo_kunit_init(test, test.priv, BO_SIZE, resv);
    bo2 = ttm_bo_kunit_init(test, test.priv, BO_SIZE, resv);
    dma_resv_lock(bo1.base.resv, core::ptr::null_mut());
    ttm_bo_set_bulk_move(bo1, &lru_bulk_move);
    dma_resv_unlock(bo1.base.resv);
    err = ttm_resource_alloc(bo1, place, &res1, core::ptr::null_mut());
    KUNIT_ASSERT_EQ(test, err, 0);
    bo1.resource = res1;
    dma_resv_lock(bo2.base.resv, core::ptr::null_mut());
    ttm_bo_set_bulk_move(bo2, &lru_bulk_move);
    dma_resv_unlock(bo2.base.resv);
    err = ttm_resource_alloc(bo2, place, &res2, core::ptr::null_mut());
    KUNIT_ASSERT_EQ(test, err, 0);
    bo2.resource = res2;
    ttm_bo_reserve(bo1, false, false, core::ptr::null_mut());
    ttm_bo_unreserve(bo1);
    pos = &lru_bulk_move.pos[mem_type][bo_priority];
    KUNIT_ASSERT_PTR_EQ(test, res1, pos.last);
    ttm_resource_free(bo1, &res1);
    ttm_resource_free(bo2, &res2);
    dma_resv_fini(resv);
    }
#[no_mangle]
unsafe extern "C" fn ttm_bo_fini_basic(test: *mut kunit) {
    static void ttm_bo_fini_basic(struct kunit *test)
    {
    struct ttm_test_devices *priv = test.priv;
    struct ttm_buffer_object *bo;
    struct ttm_resource *res;
    struct ttm_device *ttm_dev;
    struct ttm_place *place;
    let mut mem_type: u32 = TTM_PL_SYSTEM;
    int err;
    place = ttm_place_kunit_init(test, mem_type, 0);
    ttm_dev = kunit_kzalloc(test, sizeof(*ttm_dev), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, ttm_dev);
    err = ttm_device_kunit_init(priv, ttm_dev, 0);
    KUNIT_ASSERT_EQ(test, err, 0);
    priv.ttm_dev = ttm_dev;
    bo = ttm_bo_kunit_init(test, test.priv, BO_SIZE, core::ptr::null_mut());
    bo.type = ttm_bo_type_device;
    err = ttm_resource_alloc(bo, place, &res, core::ptr::null_mut());
    KUNIT_ASSERT_EQ(test, err, 0);
    bo.resource = res;
    dma_resv_lock(bo.base.resv, core::ptr::null_mut());
    err = ttm_tt_create(bo, false);
    dma_resv_unlock(bo.base.resv);
    KUNIT_EXPECT_EQ(test, err, 0);
    ttm_bo_fini(bo);
    }
    static const char *mock_name(struct dma_fence *f)
    {
    return "kunit-ttm-bo-put";
    }
    static const struct dma_fence_ops mock_fence_ops = {
    .get_driver_name = mock_name,
    .get_timeline_name = mock_name,
    };
#[no_mangle]
unsafe extern "C" fn ttm_bo_fini_shared_resv(test: *mut kunit) {
    static void ttm_bo_fini_shared_resv(struct kunit *test)
    {
    struct ttm_test_devices *priv = test.priv;
    struct ttm_buffer_object *bo;
    struct dma_resv *external_resv;
    struct dma_fence *fence;
// A dummy DMA fence lock
    spinlock_t fence_lock;
    struct ttm_device *ttm_dev;
    int err;
    ttm_dev = kunit_kzalloc(test, sizeof(*ttm_dev), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, ttm_dev);
    err = ttm_device_kunit_init(priv, ttm_dev, 0);
    KUNIT_ASSERT_EQ(test, err, 0);
    priv.ttm_dev = ttm_dev;
    external_resv = kunit_kzalloc(test, sizeof(*ttm_dev), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, external_resv);
    dma_resv_init(external_resv);
    fence = kunit_kzalloc(test, sizeof(*fence), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, fence);
    spin_lock_init(&fence_lock);
    dma_fence_init(fence, &mock_fence_ops, &fence_lock, 0, 0);
    dma_resv_lock(external_resv, core::ptr::null_mut());
    dma_resv_reserve_fences(external_resv, 1);
    dma_resv_add_fence(external_resv, fence, DMA_RESV_USAGE_BOOKKEEP);
    dma_resv_unlock(external_resv);
    dma_fence_signal(fence);
    bo = ttm_bo_kunit_init(test, test.priv, BO_SIZE, core::ptr::null_mut());
    bo.type = ttm_bo_type_device;
    bo.base.resv = external_resv;
    ttm_bo_fini(bo);
    }
#[no_mangle]
unsafe extern "C" fn ttm_bo_pin_basic(test: *mut kunit) {
    static void ttm_bo_pin_basic(struct kunit *test)
    {
    struct ttm_test_devices *priv = test.priv;
    struct ttm_buffer_object *bo;
    struct ttm_device *ttm_dev;
    let mut no_pins: c_uint = 3;
    int err;
    ttm_dev = kunit_kzalloc(test, sizeof(*ttm_dev), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, ttm_dev);
    err = ttm_device_kunit_init(priv, ttm_dev, 0);
    KUNIT_ASSERT_EQ(test, err, 0);
    priv.ttm_dev = ttm_dev;
    bo = ttm_bo_kunit_init(test, test.priv, BO_SIZE, core::ptr::null_mut());
    for (int i = 0; i < no_pins; i++) {
    dma_resv_lock(bo.base.resv, core::ptr::null_mut());
    ttm_bo_pin(bo);
    dma_resv_unlock(bo.base.resv);
    }
    KUNIT_ASSERT_EQ(test, bo.pin_count, no_pins);
    }
#[no_mangle]
unsafe extern "C" fn ttm_bo_pin_unpin_resource(test: *mut kunit) {
    static void ttm_bo_pin_unpin_resource(struct kunit *test)
    {
    struct ttm_test_devices *priv = test.priv;
    struct ttm_lru_bulk_move lru_bulk_move;
    struct ttm_lru_bulk_move_pos *pos;
    struct ttm_buffer_object *bo;
    struct ttm_resource *res;
    struct ttm_device *ttm_dev;
    struct ttm_place *place;
    let mut mem_type: u32 = TTM_PL_SYSTEM;
    let mut bo_priority: c_uint = 0;
    int err;
    ttm_lru_bulk_move_init(&lru_bulk_move);
    place = ttm_place_kunit_init(test, mem_type, 0);
    ttm_dev = kunit_kzalloc(test, sizeof(*ttm_dev), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, ttm_dev);
    err = ttm_device_kunit_init(priv, ttm_dev, 0);
    KUNIT_ASSERT_EQ(test, err, 0);
    priv.ttm_dev = ttm_dev;
    bo = ttm_bo_kunit_init(test, test.priv, BO_SIZE, core::ptr::null_mut());
    err = ttm_resource_alloc(bo, place, &res, core::ptr::null_mut());
    KUNIT_ASSERT_EQ(test, err, 0);
    bo.resource = res;
    dma_resv_lock(bo.base.resv, core::ptr::null_mut());
    ttm_bo_set_bulk_move(bo, &lru_bulk_move);
    ttm_bo_pin(bo);
    dma_resv_unlock(bo.base.resv);
    pos = &lru_bulk_move.pos[mem_type][bo_priority];
    KUNIT_ASSERT_EQ(test, bo.pin_count, 1);
    KUNIT_ASSERT_NULL(test, pos.first);
    KUNIT_ASSERT_NULL(test, pos.last);
    dma_resv_lock(bo.base.resv, core::ptr::null_mut());
    ttm_bo_unpin(bo);
    dma_resv_unlock(bo.base.resv);
    KUNIT_ASSERT_PTR_EQ(test, res, pos.last);
    KUNIT_ASSERT_EQ(test, bo.pin_count, 0);
    ttm_resource_free(bo, &res);
    }
#[no_mangle]
unsafe extern "C" fn ttm_bo_multiple_pin_one_unpin(test: *mut kunit) {
    static void ttm_bo_multiple_pin_one_unpin(struct kunit *test)
    {
    struct ttm_test_devices *priv = test.priv;
    struct ttm_lru_bulk_move lru_bulk_move;
    struct ttm_lru_bulk_move_pos *pos;
    struct ttm_buffer_object *bo;
    struct ttm_resource *res;
    struct ttm_device *ttm_dev;
    struct ttm_place *place;
    let mut mem_type: u32 = TTM_PL_SYSTEM;
    let mut bo_priority: c_uint = 0;
    int err;
    ttm_lru_bulk_move_init(&lru_bulk_move);
    place = ttm_place_kunit_init(test, mem_type, 0);
    ttm_dev = kunit_kzalloc(test, sizeof(*ttm_dev), GFP_KERNEL);
    KUNIT_ASSERT_NOT_NULL(test, ttm_dev);
    err = ttm_device_kunit_init(priv, ttm_dev, 0);
    KUNIT_ASSERT_EQ(test, err, 0);
    priv.ttm_dev = ttm_dev;
    bo = ttm_bo_kunit_init(test, test.priv, BO_SIZE, core::ptr::null_mut());
    err = ttm_resource_alloc(bo, place, &res, core::ptr::null_mut());
    KUNIT_ASSERT_EQ(test, err, 0);
    bo.resource = res;
    dma_resv_lock(bo.base.resv, core::ptr::null_mut());
    ttm_bo_set_bulk_move(bo, &lru_bulk_move);
// Multiple pins
    ttm_bo_pin(bo);
    ttm_bo_pin(bo);
    dma_resv_unlock(bo.base.resv);
    pos = &lru_bulk_move.pos[mem_type][bo_priority];
    KUNIT_ASSERT_EQ(test, bo.pin_count, 2);
    KUNIT_ASSERT_NULL(test, pos.first);
    KUNIT_ASSERT_NULL(test, pos.last);
    dma_resv_lock(bo.base.resv, core::ptr::null_mut());
    ttm_bo_unpin(bo);
    dma_resv_unlock(bo.base.resv);
    KUNIT_ASSERT_EQ(test, bo.pin_count, 1);
    KUNIT_ASSERT_NULL(test, pos.first);
    KUNIT_ASSERT_NULL(test, pos.last);
    dma_resv_lock(bo.base.resv, core::ptr::null_mut());
    ttm_bo_unpin(bo);
    dma_resv_unlock(bo.base.resv);
    ttm_resource_free(bo, &res);
    }
    static struct kunit_case ttm_bo_test_cases[] = {
    KUNIT_CASE_PARAM(ttm_bo_reserve_optimistic_no_ticket,
    ttm_bo_reserve_gen_params),
    KUNIT_CASE(ttm_bo_reserve_locked_no_sleep),
    KUNIT_CASE(ttm_bo_reserve_no_wait_ticket),
    KUNIT_CASE(ttm_bo_reserve_double_resv),

    KUNIT_CASE(ttm_bo_reserve_interrupted),

    KUNIT_CASE(ttm_bo_reserve_deadlock),
    KUNIT_CASE(ttm_bo_unreserve_basic),
    KUNIT_CASE(ttm_bo_unreserve_pinned),
    KUNIT_CASE(ttm_bo_unreserve_bulk),
    KUNIT_CASE(ttm_bo_fini_basic),
    KUNIT_CASE(ttm_bo_fini_shared_resv),
    KUNIT_CASE(ttm_bo_pin_basic),
    KUNIT_CASE(ttm_bo_pin_unpin_resource),
    KUNIT_CASE(ttm_bo_multiple_pin_one_unpin),
    {}
    };
    static struct kunit_suite ttm_bo_test_suite = {
    .name = "ttm_bo",
    .init = ttm_test_devices_init,
    .exit = ttm_test_devices_fini,
    .test_cases = ttm_bo_test_cases,
    };
    kunit_test_suites(&ttm_bo_test_suite);
    MODULE_DESCRIPTION("KUnit tests for ttm_bo APIs");
    MODULE_LICENSE("GPL and additional rights");
