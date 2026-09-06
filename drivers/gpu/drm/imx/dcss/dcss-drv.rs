//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/imx/dcss/dcss-drv.c
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
// Copyright 2019 NXP.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcss_drv {
    pub dcss: *mut dcss_dev,
    pub kms: *mut dcss_kms_dev,
}

    struct dcss_dev *dcss_drv_dev_to_dcss(struct device *dev)
    {
    struct dcss_drv *mdrv = dev_get_drvdata(dev);
    return mdrv ? mdrv.dcss : core::ptr::null_mut();
    }
    struct drm_device *dcss_drv_dev_to_drm(struct device *dev)
    {
    struct dcss_drv *mdrv = dev_get_drvdata(dev);
    return mdrv ? &mdrv.kms.base : core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn dcss_drv_platform_probe(pdev: *mut platform_device) -> c_int {
    static int dcss_drv_platform_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *remote;
    struct dcss_drv *mdrv;
    let mut err: c_int = 0;
    let mut hdmi_output: bool = true;
    if (!dev.of_node)
    return -ENODEV;
    remote = of_graph_get_remote_node(dev.of_node, 0, 0);
    if (!remote)
    return -ENODEV;
    hdmi_output = !of_device_is_compatible(remote, "fsl,imx8mq-nwl-dsi");
    of_node_put(remote);
    mdrv = devm_kzalloc(dev, sizeof(*mdrv), GFP_KERNEL);
    if (!mdrv)
    return -ENOMEM;
    mdrv.dcss = dcss_dev_create(dev, hdmi_output);
    if (IS_ERR(mdrv.dcss))
    return PTR_ERR(mdrv.dcss);
    dev_set_drvdata(dev, mdrv);
    mdrv.kms = dcss_kms_attach(mdrv.dcss);
    if (IS_ERR(mdrv.kms)) {
    err = PTR_ERR(mdrv.kms);
    dev_err_probe(dev, err, "Failed to initialize KMS\n");
    goto dcss_shutoff;
    }
    return 0;
    dcss_shutoff:
    dcss_dev_destroy(mdrv.dcss);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn dcss_drv_platform_remove(pdev: *mut platform_device) {
    static void dcss_drv_platform_remove(struct platform_device *pdev)
    {
    struct dcss_drv *mdrv = dev_get_drvdata(&pdev.dev);
    dcss_kms_detach(mdrv.kms);
    dcss_dev_destroy(mdrv.dcss);
    }
#[no_mangle]
unsafe extern "C" fn dcss_drv_platform_shutdown(pdev: *mut platform_device) {
    static void dcss_drv_platform_shutdown(struct platform_device *pdev)
    {
    struct dcss_drv *mdrv = dev_get_drvdata(&pdev.dev);
    dcss_kms_shutdown(mdrv.kms);
    }
    static struct dcss_type_data dcss_types[] = {
    [DCSS_IMX8MQ] = {
    .name = "DCSS_IMX8MQ",
    .blkctl_ofs = 0x2F000,
    .ctxld_ofs = 0x23000,
    .dtg_ofs = 0x20000,
    .scaler_ofs = 0x1C000,
    .ss_ofs = 0x1B000,
    .dpr_ofs = 0x18000,
    },
    };
    static const struct of_device_id dcss_of_match[] = {
    { .compatible = "nxp,imx8mq-dcss", .data = &dcss_types[DCSS_IMX8MQ], },
    {},
    };
    MODULE_DEVICE_TABLE(of, dcss_of_match);
    static struct platform_driver dcss_platform_driver = {
    .probe	= dcss_drv_platform_probe,
    .remove = dcss_drv_platform_remove,
    .shutdown = dcss_drv_platform_shutdown,
    .driver	= {
    .name = "imx-dcss",
    .of_match_table	= dcss_of_match,
    .pm = pm_ptr(&dcss_dev_pm_ops),
    },
    };
    drm_module_platform_driver(dcss_platform_driver);
    MODULE_AUTHOR("Laurentiu Palcu <laurentiu.palcu@nxp.com>");
    MODULE_DESCRIPTION("DCSS driver for i.MX8MQ");
    MODULE_LICENSE("GPL v2");
