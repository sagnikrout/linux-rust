//! Automatically rewritten from C to Rust
//! Source: drivers/interconnect/qcom/smd-rpm.c
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
// RPM over SMD communication wrapper for interconnects
//
// Copyright (C) 2019 Linaro Ltd
// Author: Georgi Djakov <georgi.djakov@linaro.org>
//

pub const RPM_KEY_BW: c_uint = 0x00007762;
    static struct qcom_smd_rpm *icc_smd_rpm;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icc_rpm_smd_req {
    pub key: __le32,
    pub nbytes: __le32,
    pub value: __le32,
}

#[no_mangle]
pub unsafe extern "C" fn qcom_icc_rpm_smd_available() -> bool {
    bool qcom_icc_rpm_smd_available(void)
    {
    return !!icc_smd_rpm;
    }
    EXPORT_SYMBOL_GPL(qcom_icc_rpm_smd_available);
#[no_mangle]
pub unsafe extern "C" fn qcom_icc_rpm_smd_send(ctx: c_int, rsc_type: c_int, id: c_int, val: u32) -> c_int {
    int qcom_icc_rpm_smd_send(int ctx, int rsc_type, int id, u32 val)
    {
    struct icc_rpm_smd_req req = {
    .key = cpu_to_le32(RPM_KEY_BW),
    .nbytes = cpu_to_le32(sizeof(u32)),
    .value = cpu_to_le32(val),
    };
    return qcom_rpm_smd_write(icc_smd_rpm, ctx, rsc_type, id, &req,
    sizeof(req));
    }
    EXPORT_SYMBOL_GPL(qcom_icc_rpm_smd_send);
#[no_mangle]
pub unsafe extern "C" fn qcom_icc_rpm_set_bus_rate(clk: *const rpm_clk_resource, ctx: c_int, rate: u32) -> c_int {
    int qcom_icc_rpm_set_bus_rate(const struct rpm_clk_resource *clk, int ctx, u32 rate)
    {
    struct clk_smd_rpm_req req = {
    .key = cpu_to_le32(QCOM_RPM_SMD_KEY_RATE),
    .nbytes = cpu_to_le32(sizeof(u32)),
    };
// Branch clocks are only on/off
    if (clk.branch)
    rate = !!rate;
    req.value = cpu_to_le32(rate);
    return qcom_rpm_smd_write(icc_smd_rpm,
    ctx,
    clk.resource_type,
    clk.clock_id,
    &req, sizeof(req));
    }
    EXPORT_SYMBOL_GPL(qcom_icc_rpm_set_bus_rate);
#[no_mangle]
unsafe extern "C" fn qcom_icc_rpm_smd_remove(pdev: *mut platform_device) {
    static void qcom_icc_rpm_smd_remove(struct platform_device *pdev)
    {
    icc_smd_rpm = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn qcom_icc_rpm_smd_probe(pdev: *mut platform_device) -> c_int {
    static int qcom_icc_rpm_smd_probe(struct platform_device *pdev)
    {
    icc_smd_rpm = dev_get_drvdata(pdev.dev.parent);
    if (!icc_smd_rpm) {
    dev_err(&pdev.dev, "unable to retrieve handle to RPM\n");
    return -ENODEV;
    }
    return 0;
    }
    static struct platform_driver qcom_interconnect_rpm_smd_driver = {
    .driver = {
    .name		= "icc_smd_rpm",
    },
    .probe = qcom_icc_rpm_smd_probe,
    .remove = qcom_icc_rpm_smd_remove,
    };
    module_platform_driver(qcom_interconnect_rpm_smd_driver);
    MODULE_AUTHOR("Georgi Djakov <georgi.djakov@linaro.org>");
    MODULE_DESCRIPTION("Qualcomm SMD RPM interconnect proxy driver");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:icc_smd_rpm");
