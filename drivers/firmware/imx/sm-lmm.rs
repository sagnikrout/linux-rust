//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/imx/sm-lmm.c
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
// Copyright 2025 NXP
//

    static const struct scmi_imx_lmm_proto_ops *imx_lmm_ops;
    static struct scmi_protocol_handle *ph;
#[no_mangle]
pub unsafe extern "C" fn scmi_imx_lmm_info(lmid: u32, info: *mut scmi_imx_lmm_info) -> c_int {
    int scmi_imx_lmm_info(u32 lmid, struct scmi_imx_lmm_info *info)
    {
    if (!ph)
    return -EPROBE_DEFER;
    if (!info)
    return -EINVAL;
    return imx_lmm_ops.lmm_info(ph, lmid, info);
    };
    EXPORT_SYMBOL(scmi_imx_lmm_info);
#[no_mangle]
pub unsafe extern "C" fn scmi_imx_lmm_reset_vector_set(lmid: u32, cpuid: u32, flags: u32, vector: u64) -> c_int {
    int scmi_imx_lmm_reset_vector_set(u32 lmid, u32 cpuid, u32 flags, u64 vector)
    {
    if (!ph)
    return -EPROBE_DEFER;
    return imx_lmm_ops.lmm_reset_vector_set(ph, lmid, cpuid, flags, vector);
    }
    EXPORT_SYMBOL(scmi_imx_lmm_reset_vector_set);
#[no_mangle]
pub unsafe extern "C" fn scmi_imx_lmm_operation(lmid: u32, op: enum scmi_imx_lmm_op, flags: u32) -> c_int {
    int scmi_imx_lmm_operation(u32 lmid, enum scmi_imx_lmm_op op, u32 flags)
    {
    if (!ph)
    return -EPROBE_DEFER;
    switch (op) {
    case SCMI_IMX_LMM_BOOT:
    return imx_lmm_ops.lmm_power_boot(ph, lmid, true);
    case SCMI_IMX_LMM_POWER_ON:
    return imx_lmm_ops.lmm_power_boot(ph, lmid, false);
    case SCMI_IMX_LMM_SHUTDOWN:
    return imx_lmm_ops.lmm_shutdown(ph, lmid, flags);
    default:
    break;
    }
    return -EINVAL;
    }
    EXPORT_SYMBOL(scmi_imx_lmm_operation);
#[no_mangle]
unsafe extern "C" fn scmi_imx_lmm_probe(sdev: *mut scmi_device) -> c_int {
    static int scmi_imx_lmm_probe(struct scmi_device *sdev)
    {
    const struct scmi_handle *handle = sdev.handle;
    if (!handle)
    return -ENODEV;
    if (imx_lmm_ops) {
    dev_err(&sdev.dev, "lmm already initialized\n");
    return -EEXIST;
    }
    imx_lmm_ops = handle.devm_protocol_get(sdev, SCMI_PROTOCOL_IMX_LMM, &ph);
    if (IS_ERR(imx_lmm_ops))
    return PTR_ERR(imx_lmm_ops);
    return 0;
    }
    static const struct scmi_device_id scmi_id_table[] = {
    { SCMI_PROTOCOL_IMX_LMM, "imx-lmm" },
    { },
    };
    MODULE_DEVICE_TABLE(scmi, scmi_id_table);
    static struct scmi_driver scmi_imx_lmm_driver = {
    .name = "scmi-imx-lmm",
    .probe = scmi_imx_lmm_probe,
    .id_table = scmi_id_table,
    };
    module_scmi_driver(scmi_imx_lmm_driver);
    MODULE_AUTHOR("Peng Fan <peng.fan@nxp.com>");
    MODULE_DESCRIPTION("IMX SM LMM driver");
    MODULE_LICENSE("GPL");
