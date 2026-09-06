//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/imx_sc_thermal.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright 2018-2020 NXP.
//

pub const IMX_SC_MISC_FUNC_GET_TEMP: c_int = 13;
    static struct imx_sc_ipc *thermal_ipc_handle;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_sc_sensor {
    pub tzd: *mut thermal_zone_device,
    pub resource_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct req_get_temp {
    pub resource_id: u16,
    pub type: u8,
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct resp_get_temp {
    pub celsius: i16,
    pub tenths: i8,
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_sc_msg_misc_get_temp {
    pub hdr: imx_sc_rpc_msg,
    union {
    pub req: req_get_temp,
    pub resp: resp_get_temp,
    pub data: },
    pub __aligned(4): } __packed,
#[no_mangle]
unsafe extern "C" fn imx_sc_thermal_get_temp(tz: *mut thermal_zone_device, temp: *mut c_int) -> c_int {
    static int imx_sc_thermal_get_temp(struct thermal_zone_device *tz, int *temp)
    {
    pub msg: imx_sc_msg_misc_get_temp,
    pub &msg.hdr: *mut *mut imx_sc_rpc_msg hdr =,
    pub thermal_zone_device_priv(tz): *mut *mut imx_sc_sensor sensor =,
    pub ret: c_int,
    pub sensor->resource_id: msg.data.req.resource_id =,
    pub IMX_SC_C_TEMP: msg.data.req.type =,
    pub IMX_SC_RPC_VERSION: hdr->ver =,
    pub IMX_SC_RPC_SVC_MISC: hdr->svc =,
    pub IMX_SC_MISC_FUNC_GET_TEMP: hdr->func =,
    pub 2: hdr->size =,
    pub true): ret = imx_scu_call_rpc(thermal_ipc_handle, &msg,,
    if (ret)
    pub ret: return,
// temp = msg.data.resp.celsius * 1000 + msg.data.resp.tenths * 100;
    pub 0: return,
    }
    static const struct thermal_zone_device_ops imx_sc_thermal_ops = {
    .get_temp = imx_sc_thermal_get_temp,
}

#[no_mangle]
unsafe extern "C" fn imx_sc_thermal_probe(pdev: *mut platform_device) -> c_int {
    static int imx_sc_thermal_probe(struct platform_device *pdev)
    {
    struct imx_sc_sensor *sensor;
    const int *resource_id;
    int i, ret;
    ret = imx_scu_get_handle(&thermal_ipc_handle);
    if (ret)
    return ret;
    resource_id = of_device_get_match_data(&pdev.dev);
    if (!resource_id)
    return -EINVAL;
    for (i = 0; resource_id[i] >= 0; i++) {
    sensor = devm_kzalloc(&pdev.dev, sizeof(*sensor), GFP_KERNEL);
    if (!sensor)
    return -ENOMEM;
    sensor.resource_id = resource_id[i];
    sensor.tzd = devm_thermal_of_zone_register(&pdev.dev, sensor.resource_id,
    sensor, &imx_sc_thermal_ops);
    if (IS_ERR(sensor.tzd)) {
//
// Save the error value before freeing the
// sensor pointer, otherwise we endup with a
// use-after-free error
//
    ret = PTR_ERR(sensor.tzd);
    devm_kfree(&pdev.dev, sensor);
//
// The thermal framework notifies us there is
// no thermal zone description for such a
// sensor id
//
    if (ret == -ENODEV)
    continue;
    return dev_err_probe(&pdev.dev, ret, "failed to register thermal zone\n");
    }
    devm_thermal_add_hwmon_sysfs(&pdev.dev, sensor.tzd);
    }
    return 0;
    }
    static const int imx_sc_sensors[] = {
    IMX_SC_R_SYSTEM, IMX_SC_R_PMIC_0,
    IMX_SC_R_AP_0, IMX_SC_R_AP_1,
    IMX_SC_R_GPU_0_PID0, IMX_SC_R_GPU_1_PID0,
    IMX_SC_R_DRC_0, -1 };
    static const struct of_device_id imx_sc_thermal_table[] = {
    { .compatible = "fsl,imx-sc-thermal", .data =  imx_sc_sensors },
    {}
    };
    MODULE_DEVICE_TABLE(of, imx_sc_thermal_table);
    static struct platform_driver imx_sc_thermal_driver = {
    .probe = imx_sc_thermal_probe,
    .driver = {
    .name = "imx-sc-thermal",
    .of_match_table = imx_sc_thermal_table,
    },
    };
    module_platform_driver(imx_sc_thermal_driver);
    MODULE_AUTHOR("Anson Huang <Anson.Huang@nxp.com>");
    MODULE_DESCRIPTION("Thermal driver for NXP i.MX SoCs with system controller");
    MODULE_LICENSE("GPL v2");
