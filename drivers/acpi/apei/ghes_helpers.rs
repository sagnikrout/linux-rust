//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/apei/ghes_helpers.c
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
// Copyright(c) 2025 Intel Corporation. All rights reserved

#[no_mangle]
pub unsafe extern "C" fn cxl_cper_sec_prot_err_valid(prot_err: *mut cxl_cper_sec_prot_err) -> c_int {
    int cxl_cper_sec_prot_err_valid(struct cxl_cper_sec_prot_err *prot_err)
    {
    if (!(prot_err.valid_bits & PROT_ERR_VALID_AGENT_ADDRESS)) {
    pr_err_ratelimited("CXL CPER invalid agent type\n");
    return -EINVAL;
    }
    if (!(prot_err.valid_bits & PROT_ERR_VALID_ERROR_LOG)) {
    pr_err_ratelimited("CXL CPER invalid protocol error log\n");
    return -EINVAL;
    }
    if (prot_err.err_len != sizeof(struct cxl_ras_capability_regs)) {
    pr_err_ratelimited("CXL CPER invalid RAS Cap size (%u)\n",
    prot_err.err_len);
    return -EINVAL;
    }
    if ((prot_err.agent_type == RCD || prot_err.agent_type == DEVICE ||
    prot_err.agent_type == LD || prot_err.agent_type == FMLD) &&
    !(prot_err.valid_bits & PROT_ERR_VALID_SERIAL_NUMBER))
    pr_warn_ratelimited(FW_WARN
    "CXL CPER no device serial number\n");
    return 0;
    }
    EXPORT_SYMBOL_GPL(cxl_cper_sec_prot_err_valid);
    int cxl_cper_setup_prot_err_work_data(struct cxl_cper_prot_err_work_data *wd,
    struct cxl_cper_sec_prot_err *prot_err,
    int severity)
    {
    u8 *dvsec_start, *cap_start;
    switch (prot_err.agent_type) {
    case RCD:
    case DEVICE:
    case LD:
    case FMLD:
    case RP:
    case DSP:
    case USP:
    memcpy(&wd.prot_err, prot_err, sizeof(wd.prot_err));
    dvsec_start = (u8 *)(prot_err + 1);
    cap_start = dvsec_start + prot_err.dvsec_len;
    memcpy(&wd.ras_cap, cap_start, sizeof(wd.ras_cap));
    wd.severity = cper_severity_to_aer(severity);
    break;
    default:
    pr_err_ratelimited("CXL CPER invalid agent type: %d\n",
    prot_err.agent_type);
    return -EINVAL;
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(cxl_cper_setup_prot_err_work_data);
