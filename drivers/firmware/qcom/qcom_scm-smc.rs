//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/qcom/qcom_scm-smc.c
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


// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2015,2019 The Linux Foundation. All rights reserved.
//

//
// struct arm_smccc_args
// @args:	The array of values used in registers in smc instruction
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_smccc_args {
    pub args: [c_ulong; 8],
}

// Macro flag: #define CREATE_TRACE_POINTS

    static DEFINE_MUTEX(qcom_scm_lock);
pub const QCOM_SCM_EBUSY_WAIT_MS: c_int = 30;
pub const QCOM_SCM_EBUSY_MAX_RETRY: c_int = 20;
pub const SCM_SMC_N_REG_ARGS: c_int = 4;

pub const SCM_SMC_FIRST_REG_IDX: c_int = 2;

    static void __scm_smc_do_quirk(const struct arm_smccc_args *smc,
    struct arm_smccc_res *res)
    {
    let mut a0: c_ulong = smc.args[0];
    let mut quirk: arm_smccc_quirk = { .id = ARM_SMCCC_QUIRK_QCOM_A6 };
    quirk.state.a6 = 0;
    do {
    trace_scm_smc_request(a0, smc);
    arm_smccc_smc_quirk(a0, smc.args[1], smc.args[2],
    smc.args[3], smc.args[4], smc.args[5],
    quirk.state.a6, smc.args[7], res, &quirk);
    if (res.a0 == QCOM_SCM_INTERRUPTED)
    a0 = res.a0;
    } while (res.a0 == QCOM_SCM_INTERRUPTED);
    }
#[no_mangle]
unsafe extern "C" fn fill_wq_resume_args(resume: *mut arm_smccc_args, smc_call_ctx: u32) {
    static void fill_wq_resume_args(struct arm_smccc_args *resume, u32 smc_call_ctx)
    {
    memset(resume.args, 0, sizeof(resume.args[0]) * ARRAY_SIZE(resume.args));
    resume.args[0] = ARM_SMCCC_CALL_VAL(ARM_SMCCC_STD_CALL,
    ARM_SMCCC_SMC_64, ARM_SMCCC_OWNER_SIP,
    SCM_SMC_FNID(QCOM_SCM_SVC_WAITQ, QCOM_SCM_WAITQ_RESUME));
    resume.args[1] = QCOM_SCM_ARGS(1);
    resume.args[2] = smc_call_ctx;
    }
#[no_mangle]
pub unsafe extern "C" fn scm_get_wq_ctx(wq_ctx: *mut u32, flags: *mut u32, more_pending: *mut u32) -> c_int {
    int scm_get_wq_ctx(u32 *wq_ctx, u32 *flags, u32 *more_pending)
    {
    int ret;
    struct arm_smccc_res get_wq_res;
    let mut get_wq_ctx: arm_smccc_args = {0};
    get_wq_ctx.args[0] = ARM_SMCCC_CALL_VAL(ARM_SMCCC_FAST_CALL,
    ARM_SMCCC_SMC_64, ARM_SMCCC_OWNER_SIP,
    SCM_SMC_FNID(QCOM_SCM_SVC_WAITQ, QCOM_SCM_WAITQ_GET_WQ_CTX));
// Guaranteed to return only success or error, no WAITQ_*
    __scm_smc_do_quirk(&get_wq_ctx, &get_wq_res);
    ret = get_wq_res.a0;
    if (ret)
    return ret;
    trace_scm_waitq_get_wq_ctx(get_wq_res.a1, get_wq_res.a2, get_wq_res.a3);
// wq_ctx = get_wq_res.a1;
// flags  = get_wq_res.a2;
// more_pending = get_wq_res.a3;
    return 0;
    }
    static int __scm_smc_do_quirk_handle_waitq(struct device *dev, struct arm_smccc_args *waitq,
    struct arm_smccc_res *res)
    {
    int ret;
    u32 wq_ctx, smc_call_ctx;
    struct arm_smccc_args resume;
    struct arm_smccc_args *smc = waitq;
    do {
    __scm_smc_do_quirk(smc, res);
    if (res.a0 == QCOM_SCM_WAITQ_SLEEP) {
    wq_ctx = res.a1;
    smc_call_ctx = res.a2;
    trace_scm_waitq_sleep(wq_ctx, smc_call_ctx);
    ret = qcom_scm_wait_for_wq_completion(dev, wq_ctx);
    if (ret)
    return ret;
    trace_scm_waitq_resume(smc_call_ctx);
    fill_wq_resume_args(&resume, smc_call_ctx);
    smc = &resume;
    }
    } while (res.a0 == QCOM_SCM_WAITQ_SLEEP);
    return 0;
    }
    static int __scm_smc_do(struct device *dev, struct arm_smccc_args *smc,
    struct arm_smccc_res *res, bool atomic)
    {
    int ret, retry_count = 0;
    if (atomic) {
    __scm_smc_do_quirk(smc, res);
    return 0;
    }
    do {
    mutex_lock(&qcom_scm_lock);
    ret = __scm_smc_do_quirk_handle_waitq(dev, smc, res);
    mutex_unlock(&qcom_scm_lock);
    if (ret)
    return ret;
    if (res.a0 == QCOM_SCM_V2_EBUSY) {
    if (retry_count++ > QCOM_SCM_EBUSY_MAX_RETRY)
    break;
    msleep(QCOM_SCM_EBUSY_WAIT_MS);
    }
    }  while (res.a0 == QCOM_SCM_V2_EBUSY);
    return 0;
    }
    int __scm_smc_call(struct device *dev, const struct qcom_scm_desc *desc,
    enum qcom_scm_convention qcom_convention,
    struct qcom_scm_res *res, bool atomic)
    {
    let mut arglen: c_int = desc.arginfo & 0xf;
    int i, ret;
    void *args_virt __free(qcom_tzmem) = core::ptr::null_mut();
    let mut flag: gfp_t = atomic ? GFP_ATOMIC : GFP_KERNEL;
    let mut smccc_call_type: u32 = atomic ? ARM_SMCCC_FAST_CALL : ARM_SMCCC_STD_CALL;
    u32 qcom_smccc_convention = (qcom_convention == SMC_CONVENTION_ARM_32) ?
    ARM_SMCCC_SMC_32 : ARM_SMCCC_SMC_64;
    struct arm_smccc_res smc_res;
    let mut smc: arm_smccc_args = {0};
    smc.args[0] = ARM_SMCCC_CALL_VAL(
    smccc_call_type,
    qcom_smccc_convention,
    desc.owner,
    SCM_SMC_FNID(desc.svc, desc.cmd));
    smc.args[1] = desc.arginfo;
    for (i = 0; i < SCM_SMC_N_REG_ARGS; i++)
    smc.args[i + SCM_SMC_FIRST_REG_IDX] = desc.args[i];
    if (unlikely(arglen > SCM_SMC_N_REG_ARGS)) {
    struct qcom_tzmem_pool *mempool = qcom_scm_get_tzmem_pool();
    if (!mempool)
    return -EINVAL;
    args_virt = qcom_tzmem_alloc(mempool,
    SCM_SMC_N_EXT_ARGS * sizeof(u64),
    flag);
    if (!args_virt)
    return -ENOMEM;
    if (qcom_smccc_convention == ARM_SMCCC_SMC_32) {
    __le32 *args = args_virt;
    for (i = 0; i < SCM_SMC_N_EXT_ARGS; i++)
    args[i] = cpu_to_le32(desc.args[i +
    SCM_SMC_FIRST_EXT_IDX]);
    } else {
    __le64 *args = args_virt;
    for (i = 0; i < SCM_SMC_N_EXT_ARGS; i++)
    args[i] = cpu_to_le64(desc.args[i +
    SCM_SMC_FIRST_EXT_IDX]);
    }
    smc.args[SCM_SMC_LAST_REG_IDX] = qcom_tzmem_to_phys(args_virt);
    }
    ret = __scm_smc_do(dev, &smc, &smc_res, atomic);
    trace_scm_smc_done(ret, smc.args[0], &smc_res);
    if (ret)
    return ret;
    if (res) {
    res.result[0] = smc_res.a1;
    res.result[1] = smc_res.a2;
    res.result[2] = smc_res.a3;
    }
    return (long)smc_res.a0 ? qcom_scm_remap_error(smc_res.a0) : 0;
    }
