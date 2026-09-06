//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/intel/speed_select_if/isst_if_mbox_msr.c
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
// Intel Speed Select Interface: Mbox via MSR Interface
// Copyright (c) 2019, Intel Corporation.
// All rights reserved.
//
// Author: Srinivas Pandruvada <srinivas.pandruvada@linux.intel.com>
//

pub const MSR_OS_MAILBOX_BUSY_BIT: c_int = 31;
//
// Based on experiments count is never more than 1, as the MSR overhead
// is enough to finish the command. So here this is the worst case number.
//
pub const OS_MAILBOX_RETRY_COUNT: c_int = 3;
    static int isst_if_send_mbox_cmd(u8 command, u8 sub_command, u32 parameter,
    u32 command_data, u32 *response_data)
    {
    u32 retries;
    u64 data;
    int ret;
// Poll for rb bit == 0
    retries = OS_MAILBOX_RETRY_COUNT;
    do {
    rdmsrq(MSR_OS_MAILBOX_INTERFACE, data);
    if (data & BIT_ULL(MSR_OS_MAILBOX_BUSY_BIT)) {
    ret = -EBUSY;
    continue;
    }
    ret = 0;
    break;
    } while (--retries);
    if (ret)
    return ret;
// Write DATA register
    wrmsrq(MSR_OS_MAILBOX_DATA, command_data);
// Write command register
    data = BIT_ULL(MSR_OS_MAILBOX_BUSY_BIT) |
    (parameter & GENMASK_ULL(13, 0)) << 16 |
    (sub_command << 8) |
    command;
    wrmsrq(MSR_OS_MAILBOX_INTERFACE, data);
// Poll for rb bit == 0
    retries = OS_MAILBOX_RETRY_COUNT;
    do {
    rdmsrq(MSR_OS_MAILBOX_INTERFACE, data);
    if (data & BIT_ULL(MSR_OS_MAILBOX_BUSY_BIT)) {
    ret = -EBUSY;
    continue;
    }
    if (data & 0xff)
    return -ENXIO;
    if (response_data) {
    rdmsrq(MSR_OS_MAILBOX_DATA, data);
// response_data = data;
    }
    ret = 0;
    break;
    } while (--retries);
    return ret;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msrl_action {
    pub err: c_int,
    pub mbox_cmd: *mut isst_if_mbox_cmd,
}

// revisit, smp_call_function_single should be enough for atomic mailbox!
#[no_mangle]
unsafe extern "C" fn msrl_update_func(info: *mut c_void) {
    static void msrl_update_func(void *info)
    {
    struct msrl_action *act = info;
    act.err = isst_if_send_mbox_cmd(act.mbox_cmd.command,
    act.mbox_cmd.sub_command,
    act.mbox_cmd.parameter,
    act.mbox_cmd.req_data,
    &act.mbox_cmd.resp_data);
    }
#[no_mangle]
unsafe extern "C" fn isst_if_mbox_proc_cmd(cmd_ptr: *mut u8, write_only: *mut c_int, resume: c_int) -> c_long {
    static long isst_if_mbox_proc_cmd(u8 *cmd_ptr, int *write_only, int resume)
    {
    struct msrl_action action;
    int ret;
    action.mbox_cmd = (struct isst_if_mbox_cmd *)cmd_ptr;
    if (isst_if_mbox_cmd_invalid(action.mbox_cmd))
    return -EINVAL;
    if (isst_if_mbox_cmd_set_req(action.mbox_cmd) &&
    !capable(CAP_SYS_ADMIN))
    return -EPERM;
//
// To complete mailbox command, we need to access two MSRs.
// So we don't want race to complete a mailbox transcation.
// Here smp_call ensures that msrl_update_func() has no race
// and also with wait flag, wait for completion.
// smp_call_function_single is using get_cpu() and put_cpu().
//
    ret = smp_call_function_single(action.mbox_cmd.logical_cpu,
    msrl_update_func, &action, 1);
    if (ret)
    return ret;
    if (!action.err && !resume && isst_if_mbox_cmd_set_req(action.mbox_cmd))
    action.err = isst_store_cmd(action.mbox_cmd.command,
    action.mbox_cmd.sub_command,
    action.mbox_cmd.logical_cpu, 1,
    action.mbox_cmd.parameter,
    action.mbox_cmd.req_data);
// write_only = 0;
    return action.err;
    }
    static int isst_pm_notify(struct notifier_block *nb,
    unsigned long mode, void *_unused)
    {
    switch (mode) {
    case PM_POST_HIBERNATION:
    case PM_POST_RESTORE:
    case PM_POST_SUSPEND:
    isst_resume_common();
    break;
    default:
    break;
    }
    return 0;
    }
    static struct notifier_block isst_pm_nb = {
    .notifier_call = isst_pm_notify,
    };
    static const struct x86_cpu_id isst_if_cpu_ids[] = {
    X86_MATCH_VFM(INTEL_SKYLAKE_X, core::ptr::null_mut()),
    {}
    };
    MODULE_DEVICE_TABLE(x86cpu, isst_if_cpu_ids);
#[no_mangle]
unsafe extern "C" fn isst_if_mbox_init() -> int __init {
    static int __init isst_if_mbox_init(void)
    {
    struct isst_if_cmd_cb cb;
    const struct x86_cpu_id *id;
    u64 data;
    int ret;
    id = x86_match_cpu(isst_if_cpu_ids);
    if (!id)
    return -ENODEV;
// Check presence of mailbox MSRs
    ret = rdmsrq_safe(MSR_OS_MAILBOX_INTERFACE, &data);
    if (ret)
    return ret;
    ret = rdmsrq_safe(MSR_OS_MAILBOX_DATA, &data);
    if (ret)
    return ret;
    memset(&cb, 0, sizeof(cb));
    cb.cmd_size = sizeof(struct isst_if_mbox_cmd);
    cb.offset = offsetof(struct isst_if_mbox_cmds, mbox_cmd);
    cb.cmd_callback = isst_if_mbox_proc_cmd;
    cb.owner = THIS_MODULE;
    ret = isst_if_cdev_register(ISST_IF_DEV_MBOX, &cb);
    if (ret)
    return ret;
    ret = register_pm_notifier(&isst_pm_nb);
    if (ret)
    isst_if_cdev_unregister(ISST_IF_DEV_MBOX);
    return ret;
    }
    module_init(isst_if_mbox_init)
#[no_mangle]
unsafe extern "C" fn isst_if_mbox_exit() -> void __exit {
    static void __exit isst_if_mbox_exit(void)
    {
    unregister_pm_notifier(&isst_pm_nb);
    isst_if_cdev_unregister(ISST_IF_DEV_MBOX);
    }
    module_exit(isst_if_mbox_exit)
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("Intel speed select interface mailbox driver");
