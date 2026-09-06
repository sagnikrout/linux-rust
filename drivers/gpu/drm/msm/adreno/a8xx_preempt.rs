//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/msm/adreno/a8xx_preempt.c
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
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.

#[no_mangle]
unsafe extern "C" fn preempt_prepare_postamble(a6xx_gpu: *mut a6xx_gpu) {
    static void preempt_prepare_postamble(struct a6xx_gpu *a6xx_gpu)
    {
    u32 *postamble = a6xx_gpu.preempt_postamble_ptr;
    let mut count: u32 = 0;
    postamble[count++] = PKT7(CP_REG_RMW, 3);
    postamble[count++] = REG_A8XX_RBBM_PERFCTR_SRAM_INIT_CMD;
    postamble[count++] = 0;
    postamble[count++] = 1;
    postamble[count++] = PKT7(CP_WAIT_REG_MEM, 6);
    postamble[count++] = CP_WAIT_REG_MEM_0_FUNCTION(WRITE_EQ);
    postamble[count++] = CP_WAIT_REG_MEM_POLL_ADDR_LO(
    REG_A8XX_RBBM_PERFCTR_SRAM_INIT_STATUS);
    postamble[count++] = CP_WAIT_REG_MEM_POLL_ADDR_HI(0);
    postamble[count++] = CP_WAIT_REG_MEM_3_REF(0x1);
    postamble[count++] = CP_WAIT_REG_MEM_4_MASK(0x1);
    postamble[count++] = CP_WAIT_REG_MEM_5_DELAY_LOOP_CYCLES(0);
    a6xx_gpu.preempt_postamble_len = count;
    a6xx_gpu.postamble_enabled = true;
    }
#[no_mangle]
unsafe extern "C" fn preempt_disable_postamble(a6xx_gpu: *mut a6xx_gpu) {
    static void preempt_disable_postamble(struct a6xx_gpu *a6xx_gpu)
    {
    u32 *postamble = a6xx_gpu.preempt_postamble_ptr;
//
// Disable the postamble by replacing the first packet header with a NOP
// that covers the whole buffer.
//
// postamble = PKT7(CP_NOP, (a6xx_gpu->preempt_postamble_len - 1));
    a6xx_gpu.postamble_enabled = false;
    }
//
// Set preemption keepalive vote. Please note that this vote is different from the one used in
// a8xx_irq()
//
#[no_mangle]
unsafe extern "C" fn a8xx_preempt_keepalive_vote(gpu: *mut msm_gpu, on: bool) {
    static void a8xx_preempt_keepalive_vote(struct msm_gpu *gpu, bool on)
    {
    struct adreno_gpu *adreno_gpu = to_adreno_gpu(gpu);
    struct a6xx_gpu *a6xx_gpu = to_a6xx_gpu(adreno_gpu);
    gmu_write(&a6xx_gpu.gmu, REG_A8XX_GMU_PWR_COL_PREEMPT_KEEPALIVE, on);
    }
#[no_mangle]
pub unsafe extern "C" fn a8xx_preempt_irq(gpu: *mut msm_gpu) {
    void a8xx_preempt_irq(struct msm_gpu *gpu)
    {
    uint32_t status;
    struct adreno_gpu *adreno_gpu = to_adreno_gpu(gpu);
    struct a6xx_gpu *a6xx_gpu = to_a6xx_gpu(adreno_gpu);
    struct drm_device *dev = gpu.dev;
    if (!try_preempt_state(a6xx_gpu, PREEMPT_TRIGGERED, PREEMPT_PENDING))
    return;
// Delete the preemption watchdog timer
    timer_delete(&a6xx_gpu.preempt_timer);
//
// The hardware should be setting the stop bit of CP_CONTEXT_SWITCH_CNTL
// to zero before firing the interrupt, but there is a non zero chance
// of a hardware condition or a software race that could set it again
// before we have a chance to finish. If that happens, log and go for
// recovery
//
    status = gpu_read(gpu, REG_A8XX_CP_CONTEXT_SWITCH_CNTL);
    if (unlikely(status & A8XX_CP_CONTEXT_SWITCH_CNTL_STOP)) {
    DRM_DEV_ERROR(&gpu.pdev.dev,
    "!!!!!!!!!!!!!!!! preemption faulted !!!!!!!!!!!!!! irq\n");
    set_preempt_state(a6xx_gpu, PREEMPT_FAULTED);
    dev_err(dev.dev, "%s: Preemption failed to complete\n",
    gpu.name);
    kthread_queue_work(gpu.worker, &gpu.recover_work);
    return;
    }
    a6xx_gpu.cur_ring = a6xx_gpu.next_ring;
    a6xx_gpu.next_ring = core::ptr::null_mut();
    set_preempt_state(a6xx_gpu, PREEMPT_FINISH);
    update_wptr(a6xx_gpu, a6xx_gpu.cur_ring);
    set_preempt_state(a6xx_gpu, PREEMPT_NONE);
    a8xx_preempt_keepalive_vote(gpu, false);
    trace_msm_gpu_preemption_irq(a6xx_gpu.cur_ring.id);
//
// Retrigger preemption to avoid a deadlock that might occur when preemption
// is skipped due to it being already in flight when requested.
//
    a8xx_preempt_trigger(gpu);
    }
#[no_mangle]
pub unsafe extern "C" fn a8xx_preempt_hw_init(gpu: *mut msm_gpu) {
    void a8xx_preempt_hw_init(struct msm_gpu *gpu)
    {
    struct adreno_gpu *adreno_gpu = to_adreno_gpu(gpu);
    struct a6xx_gpu *a6xx_gpu = to_a6xx_gpu(adreno_gpu);
    int i;
// No preemption if we only have one ring
    if (gpu.nr_rings == 1)
    return;
    for (i = 0; i < gpu.nr_rings; i++) {
    struct a6xx_preempt_record *record_ptr = a6xx_gpu.preempt[i];
    record_ptr.wptr = 0;
    record_ptr.rptr = 0;
    record_ptr.rptr_addr = shadowptr(a6xx_gpu, gpu.rb[i]);
    record_ptr.info = 0;
    record_ptr.data = 0;
    record_ptr.rbase = gpu.rb[i].iova;
    }
// Write a 0 to signal that we aren't switching pagetables
    gpu_write64(gpu, REG_A8XX_CP_CONTEXT_SWITCH_SMMU_INFO, 0);
// Enable the GMEM save/restore feature for preemption
    gpu_write(gpu, REG_A6XX_RB_CONTEXT_SWITCH_GMEM_SAVE_RESTORE_ENABLE, 0x1);
// Reset the preemption state
    set_preempt_state(a6xx_gpu, PREEMPT_NONE);
    spin_lock_init(&a6xx_gpu.eval_lock);
// Always come up on rb 0
    a6xx_gpu.cur_ring = gpu.rb[0];
    }
#[no_mangle]
pub unsafe extern "C" fn a8xx_preempt_trigger(gpu: *mut msm_gpu) {
    void a8xx_preempt_trigger(struct msm_gpu *gpu)
    {
    struct adreno_gpu *adreno_gpu = to_adreno_gpu(gpu);
    struct a6xx_gpu *a6xx_gpu = to_a6xx_gpu(adreno_gpu);
    unsigned long flags;
    struct msm_ringbuffer *ring;
    unsigned int cntl;
    bool sysprof;
    if (gpu.nr_rings == 1)
    return;
//
// Lock to make sure another thread attempting preemption doesn't skip it
// while we are still evaluating the next ring. This makes sure the other
// thread does start preemption if we abort it and avoids a soft lock.
//
    spin_lock_irqsave(&a6xx_gpu.eval_lock, flags);
//
// Try to start preemption by moving from NONE to START. If
// unsuccessful, a preemption is already in flight
//
    if (!try_preempt_state(a6xx_gpu, PREEMPT_NONE, PREEMPT_START)) {
    spin_unlock_irqrestore(&a6xx_gpu.eval_lock, flags);
    return;
    }
    cntl = A8XX_CP_CONTEXT_SWITCH_CNTL_LEVEL(a6xx_gpu.preempt_level);
    if (a6xx_gpu.skip_save_restore)
    cntl |= A8XX_CP_CONTEXT_SWITCH_CNTL_SKIP_SAVE_RESTORE;
    if (a6xx_gpu.uses_gmem)
    cntl |= A8XX_CP_CONTEXT_SWITCH_CNTL_USES_GMEM;
    cntl |= A8XX_CP_CONTEXT_SWITCH_CNTL_STOP;
// Get the next ring to preempt to
    ring = get_next_ring(gpu);
//
// If no ring is populated or the highest priority ring is the current
// one do nothing except to update the wptr to the latest and greatest
//
    if (!ring || (a6xx_gpu.cur_ring == ring)) {
    set_preempt_state(a6xx_gpu, PREEMPT_FINISH);
    update_wptr(a6xx_gpu, a6xx_gpu.cur_ring);
    set_preempt_state(a6xx_gpu, PREEMPT_NONE);
    spin_unlock_irqrestore(&a6xx_gpu.eval_lock, flags);
    return;
    }
    spin_unlock_irqrestore(&a6xx_gpu.eval_lock, flags);
    spin_lock_irqsave(&ring.preempt_lock, flags);
    struct a7xx_cp_smmu_info *smmu_info_ptr =
    a6xx_gpu.preempt_smmu[ring.id];
    struct a6xx_preempt_record *record_ptr = a6xx_gpu.preempt[ring.id];
    let mut ttbr0: u64 = ring.memptrs.ttbr0;
    let mut context_idr: u32 = ring.memptrs.context_idr;
    smmu_info_ptr.ttbr0 = ttbr0;
    smmu_info_ptr.context_idr = context_idr;
    record_ptr.wptr = get_wptr(ring);
//
// The GPU will write the wptr we set above when we preempt. Reset
// restore_wptr to make sure that we don't write WPTR to the same
// thing twice. It's still possible subsequent submissions will update
// wptr again, in which case they will set the flag to true. This has
// to be protected by the lock for setting the flag and updating wptr
// to be atomic.
//
    ring.restore_wptr = false;
    trace_msm_gpu_preemption_trigger(a6xx_gpu.cur_ring.id, ring.id);
    spin_unlock_irqrestore(&ring.preempt_lock, flags);
// Set the keepalive bit to keep the GPU ON until preemption is complete
    a8xx_preempt_keepalive_vote(gpu, true);
    a6xx_fenced_write(a6xx_gpu,
    REG_A8XX_CP_CONTEXT_SWITCH_SMMU_INFO, a6xx_gpu.preempt_smmu_iova[ring.id],
    BIT(1), true);
    a6xx_fenced_write(a6xx_gpu,
    REG_A8XX_CP_CONTEXT_SWITCH_PRIV_NON_SECURE_RESTORE_ADDR,
    a6xx_gpu.preempt_iova[ring.id], BIT(1), true);
    a6xx_gpu.next_ring = ring;
// Start a timer to catch a stuck preemption
    mod_timer(&a6xx_gpu.preempt_timer, jiffies + msecs_to_jiffies(10000));
// Enable or disable postamble as needed
    sysprof = msm_gpu_sysprof_no_perfcntr_zap(gpu);
    if (!sysprof && !a6xx_gpu.postamble_enabled)
    preempt_prepare_postamble(a6xx_gpu);
    if (sysprof && a6xx_gpu.postamble_enabled)
    preempt_disable_postamble(a6xx_gpu);
// Set the preemption state to triggered
    set_preempt_state(a6xx_gpu, PREEMPT_TRIGGERED);
// Trigger the preemption
    a6xx_fenced_write(a6xx_gpu, REG_A8XX_CP_CONTEXT_SWITCH_CNTL, cntl, BIT(1), false);
    }
