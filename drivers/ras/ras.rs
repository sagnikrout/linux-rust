//! Automatically rewritten from C to Rust
//! Source: drivers/ras/ras.c
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
// Copyright (C) 2014 Intel Corporation
//
// Authors:
// Chen, Gong <gong.chen@linux.intel.com>
//

//
// Once set, this function pointer should never be unset.
//
// The library module will set this pointer if it successfully loads. The module
// should not be unloaded except for testing and debug purposes.
//
    static unsigned long (*amd_atl_umc_na_to_spa)(struct atl_err *err);
#[no_mangle]
pub unsafe extern "C" fn amd_atl_register_decoder(): *mut *mut unsigned long (f)(struct atl_err) {
    void amd_atl_register_decoder(unsigned long (*f)(struct atl_err *))
    {
    amd_atl_umc_na_to_spa = f;
    }
    EXPORT_SYMBOL_GPL(amd_atl_register_decoder);
#[no_mangle]
pub unsafe extern "C" fn amd_atl_unregister_decoder() {
    void amd_atl_unregister_decoder(void)
    {
    amd_atl_umc_na_to_spa = core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(amd_atl_unregister_decoder);
#[no_mangle]
pub unsafe extern "C" fn amd_convert_umc_mca_addr_to_sys_addr(err: *mut atl_err) -> c_ulong {
    unsigned long amd_convert_umc_mca_addr_to_sys_addr(struct atl_err *err)
    {
    if (!amd_atl_umc_na_to_spa)
    return -EINVAL;
    return amd_atl_umc_na_to_spa(err);
    }
    EXPORT_SYMBOL_GPL(amd_convert_umc_mca_addr_to_sys_addr);

// Macro flag: #define CREATE_TRACE_POINTS

    void log_non_standard_event(const guid_t *sec_type, const guid_t *fru_id,
    const char *fru_text, const u8 sev, const u8 *err,
    const u32 len)
    {
    trace_non_standard_event(sec_type, fru_id, fru_text, sev, err, len);
    }
    EXPORT_SYMBOL_GPL(log_non_standard_event);
#[no_mangle]
pub unsafe extern "C" fn log_arm_hw_error(err: *mut cper_sec_proc_arm, sev: u8) {
    void log_arm_hw_error(struct cper_sec_proc_arm *err, const u8 sev)
    {
    struct cper_arm_err_info *err_info;
    struct cper_arm_ctx_info *ctx_info;
    u8 *ven_err_data;
    let mut ctx_len: u32 = 0;
    int n, sz, cpu;
    s32 vsei_len;
    u32 pei_len;
    u8 *pei_err, *ctx_err;
    pei_len = sizeof(struct cper_arm_err_info) * err.err_info_num;
    pei_err = (u8 *)(err + 1);
    err_info = (struct cper_arm_err_info *)(err + 1);
    ctx_info = (struct cper_arm_ctx_info *)(err_info + err.err_info_num);
    ctx_err = (u8 *)ctx_info;
    for (n = 0; n < err.context_info_num; n++) {
    sz = sizeof(struct cper_arm_ctx_info);
    if (sz + (long)ctx_info - (long)err >= err.section_length)
    sz += ctx_info.size;
    ctx_info = (struct cper_arm_ctx_info *)((long)ctx_info + sz);
    ctx_len += sz;
    }
    vsei_len = err.section_length - (sizeof(struct cper_sec_proc_arm) + pei_len + ctx_len);
    if (vsei_len < 0) {
    pr_warn(FW_BUG "section length: %d\n", err.section_length);
    pr_warn(FW_BUG "section length is too small\n");
    pr_warn(FW_BUG "firmware-generated error record is incorrect\n");
    vsei_len = 0;
    }
    ven_err_data = (u8 *)ctx_info;
    cpu = GET_LOGICAL_INDEX(err.mpidr);
    if (cpu < 0)
    cpu = -1;
    trace_arm_event(err, pei_err, pei_len, ctx_err, ctx_len,
    ven_err_data, (u32)vsei_len, sev, cpu);
    }
#[no_mangle]
unsafe extern "C" fn ras_init() -> int __init {
    static int __init ras_init(void)
    {
    let mut rc: c_int = 0;
    ras_debugfs_init();
    rc = ras_add_daemon_trace();
    return rc;
    }
    subsys_initcall(ras_init);

    EXPORT_TRACEPOINT_SYMBOL_GPL(extlog_mem_event);

    EXPORT_TRACEPOINT_SYMBOL_GPL(mc_event);
    EXPORT_TRACEPOINT_SYMBOL_GPL(non_standard_event);
    EXPORT_TRACEPOINT_SYMBOL_GPL(arm_event);
#[no_mangle]
unsafe extern "C" fn parse_ras_param(str: *mut c_char) -> int __init {
    static int __init parse_ras_param(char *str)
    {

    parse_cec_param(str);

    return 1;
    }
    __setup("ras", parse_ras_param);
