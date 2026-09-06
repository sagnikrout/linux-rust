//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/imx/sm-cpu.c
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

    static const struct scmi_imx_cpu_proto_ops *imx_cpu_ops;
    static struct scmi_protocol_handle *ph;
    int scmi_imx_cpu_reset_vector_set(u32 cpuid, u64 vector, bool start, bool boot,
    bool resume)
    {
    if (!ph)
    return -EPROBE_DEFER;
    return imx_cpu_ops.cpu_reset_vector_set(ph, cpuid, vector, start,
    boot, resume);
    }
    EXPORT_SYMBOL(scmi_imx_cpu_reset_vector_set);
#[no_mangle]
pub unsafe extern "C" fn scmi_imx_cpu_start(cpuid: u32, start: bool) -> c_int {
    int scmi_imx_cpu_start(u32 cpuid, bool start)
    {
    if (!ph)
    return -EPROBE_DEFER;
    if (start)
    return imx_cpu_ops.cpu_start(ph, cpuid, true);
    return imx_cpu_ops.cpu_start(ph, cpuid, false);
    };
    EXPORT_SYMBOL(scmi_imx_cpu_start);
#[no_mangle]
pub unsafe extern "C" fn scmi_imx_cpu_started(cpuid: u32, started: *mut bool) -> c_int {
    int scmi_imx_cpu_started(u32 cpuid, bool *started)
    {
    if (!ph)
    return -EPROBE_DEFER;
    if (!started)
    return -EINVAL;
    return imx_cpu_ops.cpu_started(ph, cpuid, started);
    };
    EXPORT_SYMBOL(scmi_imx_cpu_started);
#[no_mangle]
unsafe extern "C" fn scmi_imx_cpu_probe(sdev: *mut scmi_device) -> c_int {
    static int scmi_imx_cpu_probe(struct scmi_device *sdev)
    {
    const struct scmi_handle *handle = sdev.handle;
    if (!handle)
    return -ENODEV;
    if (imx_cpu_ops) {
    dev_err(&sdev.dev, "sm cpu already initialized\n");
    return -EEXIST;
    }
    imx_cpu_ops = handle.devm_protocol_get(sdev, SCMI_PROTOCOL_IMX_CPU, &ph);
    if (IS_ERR(imx_cpu_ops))
    return PTR_ERR(imx_cpu_ops);
    return 0;
    }
    static const struct scmi_device_id scmi_id_table[] = {
    { SCMI_PROTOCOL_IMX_CPU, "imx-cpu" },
    { },
    };
    MODULE_DEVICE_TABLE(scmi, scmi_id_table);
    static struct scmi_driver scmi_imx_cpu_driver = {
    .name = "scmi-imx-cpu",
    .probe = scmi_imx_cpu_probe,
    .id_table = scmi_id_table,
    };
    module_scmi_driver(scmi_imx_cpu_driver);
    MODULE_AUTHOR("Peng Fan <peng.fan@nxp.com>");
    MODULE_DESCRIPTION("IMX SM CPU driver");
    MODULE_LICENSE("GPL");
