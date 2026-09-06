//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/qcom/qcom_scm-legacy.c
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
// Copyright (c) 2010,2015,2019 The Linux Foundation. All rights reserved.
// Copyright (C) 2015 Linaro Ltd.
//

    static DEFINE_MUTEX(qcom_scm_lock);
//
// struct arm_smccc_args
// @args:	The array of values used in registers in smc instruction
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_smccc_args {
    pub args: [c_ulong; 8],
}

//
// struct scm_legacy_command - one SCM command buffer
// @len: total available memory for command and response
// @buf_offset: start of command buffer
// @resp_hdr_offset: start of response buffer
// @id: command to be executed
// @buf: buffer returned from scm_legacy_get_command_buffer()
//
// An SCM command is laid out in memory as follows:
//
// ------------------- <--- struct scm_legacy_command
// | command header  |
// ------------------- <--- scm_legacy_get_command_buffer()
// | command buffer  |
// ------------------- <--- struct scm_legacy_response and
// | response header |      scm_legacy_command_to_response()
// ------------------- <--- scm_legacy_get_response_buffer()
// | response buffer |
// -------------------
//
// There can be arbitrary padding between the headers and buffers so
// you should always use the appropriate scm_legacy_get_*_buffer() routines
// to access the buffers in a safe manner.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scm_legacy_command {
    pub len: __le32,
    pub buf_offset: __le32,
    pub resp_hdr_offset: __le32,
    pub id: __le32,
    pub buf: [__le32; ],
}

//
// struct scm_legacy_response - one SCM response buffer
// @len: total available memory for response
// @buf_offset: start of response data relative to start of scm_legacy_response
// @is_complete: indicates if the command has finished processing
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scm_legacy_response {
    pub len: __le32,
    pub buf_offset: __le32,
    pub is_complete: __le32,
}

//
// scm_legacy_command_to_response() - Get a pointer to a scm_legacy_response
// @cmd: command
//
// Returns a pointer to a response for a command.
//
    static inline struct scm_legacy_response *scm_legacy_command_to_response(
    const struct scm_legacy_command *cmd)
    {
    return (void *)cmd + le32_to_cpu(cmd.resp_hdr_offset);
    }
//
// scm_legacy_get_command_buffer() - Get a pointer to a command buffer
// @cmd: command
//
// Returns a pointer to the command buffer of a command.
//
    static inline void *scm_legacy_get_command_buffer(
    const struct scm_legacy_command *cmd)
    {
    return (void *)cmd.buf;
    }
//
// scm_legacy_get_response_buffer() - Get a pointer to a response buffer
// @rsp: response
//
// Returns a pointer to a response buffer of a response.
//
    static inline void *scm_legacy_get_response_buffer(
    const struct scm_legacy_response *rsp)
    {
    return (void *)rsp + le32_to_cpu(rsp.buf_offset);
    }
    static void __scm_legacy_do(const struct arm_smccc_args *smc,
    struct arm_smccc_res *res)
    {
    do {
    arm_smccc_smc(smc.args[0], smc.args[1], smc.args[2],
    smc.args[3], smc.args[4], smc.args[5],
    smc.args[6], smc.args[7], res);
    } while (res.a0 == QCOM_SCM_INTERRUPTED);
    }
//
// scm_legacy_call() - Sends a command to the SCM and waits for the command to
// finish processing.
// @dev:	device
// @desc:	descriptor structure containing arguments and return values
// @res:        results from SMC call
//
// A note on cache maintenance:
// Note that any buffers that are expected to be accessed by the secure world
// must be flushed before invoking qcom_scm_call and invalidated in the cache
// immediately after qcom_scm_call returns. Cache maintenance on the command
// and response buffers is taken care of by qcom_scm_call; however, callers are
// responsible for any other cached buffers passed over to the secure world.
//
    int scm_legacy_call(struct device *dev, const struct qcom_scm_desc *desc,
    struct qcom_scm_res *res)
    {
    let mut arglen: u8 = desc.arginfo & 0xf;
    let mut ret: c_int = 0, context_id;
    unsigned int i;
    struct scm_legacy_command *cmd;
    struct scm_legacy_response *rsp;
    let mut smc: arm_smccc_args = {0};
    struct arm_smccc_res smc_res;
    let mut cmd_len: usize = arglen * sizeof(__le32);
    let mut resp_len: usize = MAX_QCOM_SCM_RETS * sizeof(__le32);
    let mut alloc_len: usize = sizeof(*cmd) + cmd_len + sizeof(*rsp) + resp_len;
    dma_addr_t cmd_phys;
    __le32 *arg_buf;
    const __le32 *res_buf;
    cmd = kzalloc(PAGE_ALIGN(alloc_len), GFP_KERNEL);
    if (!cmd)
    return -ENOMEM;
    cmd.len = cpu_to_le32(alloc_len);
    cmd.buf_offset = cpu_to_le32(sizeof(*cmd));
    cmd.resp_hdr_offset = cpu_to_le32(sizeof(*cmd) + cmd_len);
    cmd.id = cpu_to_le32(SCM_LEGACY_FNID(desc.svc, desc.cmd));
    arg_buf = scm_legacy_get_command_buffer(cmd);
    for (i = 0; i < arglen; i++)
    arg_buf[i] = cpu_to_le32(desc.args[i]);
    rsp = scm_legacy_command_to_response(cmd);
    cmd_phys = dma_map_single(dev, cmd, alloc_len, DMA_TO_DEVICE);
    if (dma_mapping_error(dev, cmd_phys)) {
    kfree(cmd);
    return -ENOMEM;
    }
    smc.args[0] = 1;
    smc.args[1] = (unsigned long)&context_id;
    smc.args[2] = cmd_phys;
    mutex_lock(&qcom_scm_lock);
    __scm_legacy_do(&smc, &smc_res);
    if (smc_res.a0)
    ret = qcom_scm_remap_error(smc_res.a0);
    mutex_unlock(&qcom_scm_lock);
    if (ret)
    goto out;
    do {
    dma_sync_single_for_cpu(dev, cmd_phys + sizeof(*cmd) + cmd_len,
    sizeof(*rsp), DMA_FROM_DEVICE);
    } while (!rsp.is_complete);
    dma_sync_single_for_cpu(dev, cmd_phys + sizeof(*cmd) + cmd_len +
    le32_to_cpu(rsp.buf_offset),
    resp_len, DMA_FROM_DEVICE);
    if (res) {
    res_buf = scm_legacy_get_response_buffer(rsp);
    for (i = 0; i < MAX_QCOM_SCM_RETS; i++)
    res.result[i] = le32_to_cpu(res_buf[i]);
    }
    out:
    dma_unmap_single(dev, cmd_phys, alloc_len, DMA_TO_DEVICE);
    kfree(cmd);
    return ret;
    }
pub const SCM_LEGACY_ATOMIC_N_REG_ARGS: c_int = 5;
pub const SCM_LEGACY_ATOMIC_FIRST_REG_IDX: c_int = 2;

    ((SCM_LEGACY_FNID(svc, cmd) << 12) | \
    SCM_LEGACY_CLASS_REGISTER | \
    SCM_LEGACY_MASK_IRQS | \
    (n & 0xf))
//
// scm_legacy_call_atomic() - Send an atomic SCM command with up to 5 arguments
// and 3 return values
// @unused: device, legacy argument, not used, can be NULL
// @desc: SCM call descriptor containing arguments
// @res:  SCM call return values
//
// This shall only be used with commands that are guaranteed to be
// uninterruptable, atomic and SMP safe.
//
    int scm_legacy_call_atomic(struct device *unused,
    const struct qcom_scm_desc *desc,
    struct qcom_scm_res *res)
    {
    int context_id;
    struct arm_smccc_res smc_res;
    let mut arglen: usize = desc.arginfo & 0xf;
    BUG_ON(arglen > SCM_LEGACY_ATOMIC_N_REG_ARGS);
    arm_smccc_smc(SCM_LEGACY_ATOMIC_ID(desc.svc, desc.cmd, arglen),
    (unsigned long)&context_id,
    desc.args[0], desc.args[1], desc.args[2],
    desc.args[3], desc.args[4], 0, &smc_res);
    if (res) {
    res.result[0] = smc_res.a1;
    res.result[1] = smc_res.a2;
    res.result[2] = smc_res.a3;
    }
    return smc_res.a0;
    }
