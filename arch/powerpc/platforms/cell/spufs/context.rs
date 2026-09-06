//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/cell/spufs/context.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// SPU file system -- SPU context management
//
// (C) Copyright IBM Deutschland Entwicklung GmbH 2005
//
// Author: Arnd Bergmann <arndb@de.ibm.com>
//

    let mut nr_spu_contexts: core::sync::atomic::AtomicI32 = ATOMIC_INIT(0);
    struct spu_context *alloc_spu_context(struct spu_gang *gang)
    {
    struct spu_context *ctx;
    ctx = kzalloc_obj(*ctx);
    if (!ctx)
    goto out;
// Binding to physical processor deferred
// until spu_activate().
//
    if (spu_init_csa(&ctx.csa))
    goto out_free;
    spin_lock_init(&ctx.mmio_lock);
    mutex_init(&ctx.mapping_lock);
    kref_init(&ctx.kref);
    mutex_init(&ctx.state_mutex);
    mutex_init(&ctx.run_mutex);
    init_waitqueue_head(&ctx.ibox_wq);
    init_waitqueue_head(&ctx.wbox_wq);
    init_waitqueue_head(&ctx.stop_wq);
    init_waitqueue_head(&ctx.mfc_wq);
    init_waitqueue_head(&ctx.run_wq);
    ctx.state = SPU_STATE_SAVED;
    ctx.ops = &spu_backing_ops;
    ctx.owner = get_task_mm(current);
    INIT_LIST_HEAD(&ctx.rq);
    INIT_LIST_HEAD(&ctx.aff_list);
    if (gang)
    spu_gang_add_ctx(gang, ctx);
    __spu_update_sched_info(ctx);
    spu_set_timeslice(ctx);
    ctx.stats.util_state = SPU_UTIL_IDLE_LOADED;
    ctx.stats.tstamp = ktime_get_ns();
    atomic_inc(&nr_spu_contexts);
    goto out;
    out_free:
    kfree(ctx);
    ctx = core::ptr::null_mut();
    out:
    return ctx;
    }
#[no_mangle]
pub unsafe extern "C" fn destroy_spu_context(kref: *mut kref) {
    void destroy_spu_context(struct kref *kref)
    {
    struct spu_context *ctx;
    ctx = container_of(kref, struct spu_context, kref);
    spu_context_nospu_trace(destroy_spu_context__enter, ctx);
    mutex_lock(&ctx.state_mutex);
    spu_deactivate(ctx);
    mutex_unlock(&ctx.state_mutex);
    spu_fini_csa(&ctx.csa);
    if (ctx.gang)
    spu_gang_remove_ctx(ctx.gang, ctx);
    if (ctx.prof_priv_kref)
    kref_put(ctx.prof_priv_kref, ctx.prof_priv_release);
    BUG_ON(!list_empty(&ctx.rq));
    atomic_dec(&nr_spu_contexts);
    kfree(ctx.switch_log);
    kfree(ctx);
    }
#[no_mangle]
pub unsafe extern "C" fn get_spu_context(ctx: *mut spu_context) -> *mut spu_context {
    struct spu_context * get_spu_context(struct spu_context *ctx)
    {
    kref_get(&ctx.kref);
    return ctx;
    }
#[no_mangle]
pub unsafe extern "C" fn put_spu_context(ctx: *mut spu_context) -> c_int {
    int put_spu_context(struct spu_context *ctx)
    {
    return kref_put(&ctx.kref, &destroy_spu_context);
    }
// give up the mm reference when the context is about to be destroyed
#[no_mangle]
pub unsafe extern "C" fn spu_forget(ctx: *mut spu_context) {
    void spu_forget(struct spu_context *ctx)
    {
    struct mm_struct *mm;
//
// This is basically an open-coded spu_acquire_saved, except that
// we don't acquire the state mutex interruptible, and we don't
// want this context to be rescheduled on release.
//
    mutex_lock(&ctx.state_mutex);
    if (ctx.state != SPU_STATE_SAVED)
    spu_deactivate(ctx);
    mm = ctx.owner;
    ctx.owner = core::ptr::null_mut();
    mmput(mm);
    spu_release(ctx);
    }
#[no_mangle]
pub unsafe extern "C" fn spu_unmap_mappings(ctx: *mut spu_context) {
    void spu_unmap_mappings(struct spu_context *ctx)
    {
    mutex_lock(&ctx.mapping_lock);
    if (ctx.local_store)
    unmap_mapping_range(ctx.local_store, 0, LS_SIZE, 1);
    if (ctx.mfc)
    unmap_mapping_range(ctx.mfc, 0, SPUFS_MFC_MAP_SIZE, 1);
    if (ctx.cntl)
    unmap_mapping_range(ctx.cntl, 0, SPUFS_CNTL_MAP_SIZE, 1);
    if (ctx.signal1)
    unmap_mapping_range(ctx.signal1, 0, SPUFS_SIGNAL_MAP_SIZE, 1);
    if (ctx.signal2)
    unmap_mapping_range(ctx.signal2, 0, SPUFS_SIGNAL_MAP_SIZE, 1);
    if (ctx.mss)
    unmap_mapping_range(ctx.mss, 0, SPUFS_MSS_MAP_SIZE, 1);
    if (ctx.psmap)
    unmap_mapping_range(ctx.psmap, 0, SPUFS_PS_MAP_SIZE, 1);
    mutex_unlock(&ctx.mapping_lock);
    }
//
// spu_acquire_saved - lock spu contex and make sure it is in saved state
// @ctx:	spu contex to lock
//
#[no_mangle]
pub unsafe extern "C" fn spu_acquire_saved(ctx: *mut spu_context) -> c_int {
    int spu_acquire_saved(struct spu_context *ctx)
    {
    int ret;
    spu_context_nospu_trace(spu_acquire_saved__enter, ctx);
    ret = spu_acquire(ctx);
    if (ret)
    return ret;
    if (ctx.state != SPU_STATE_SAVED) {
    set_bit(SPU_SCHED_WAS_ACTIVE, &ctx.sched_flags);
    spu_deactivate(ctx);
    }
    return 0;
    }
//
// spu_release_saved - unlock spu context and return it to the runqueue
// @ctx:	context to unlock
//
#[no_mangle]
pub unsafe extern "C" fn spu_release_saved(ctx: *mut spu_context) {
    void spu_release_saved(struct spu_context *ctx)
    {
    BUG_ON(ctx.state != SPU_STATE_SAVED);
    if (test_and_clear_bit(SPU_SCHED_WAS_ACTIVE, &ctx.sched_flags) &&
    test_bit(SPU_SCHED_SPU_RUN, &ctx.sched_flags))
    spu_activate(ctx, 0);
    spu_release(ctx);
    }
