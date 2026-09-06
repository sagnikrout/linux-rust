//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/arm/display/komeda/komeda_drv.c
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
// (C) COPYRIGHT 2018 ARM Limited. All rights reserved.
// Author: James.Qian.Wang <james.qian.wang@arm.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct komeda_drv {
    pub mdev: *mut komeda_dev,
    pub kms: *mut komeda_kms_dev,
}

    struct komeda_dev *dev_to_mdev(struct device *dev)
    {
    struct komeda_drv *mdrv = dev_get_drvdata(dev);
    return mdrv ? mdrv.mdev : core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn komeda_platform_remove(pdev: *mut platform_device) {
    static void komeda_platform_remove(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct komeda_drv *mdrv = dev_get_drvdata(dev);
    komeda_kms_detach(mdrv.kms);
    if (pm_runtime_enabled(dev))
    pm_runtime_disable(dev);
    else
    komeda_dev_suspend(mdrv.mdev);
    komeda_dev_destroy(mdrv.mdev);
    dev_set_drvdata(dev, core::ptr::null_mut());
    devm_kfree(dev, mdrv);
    }
#[no_mangle]
unsafe extern "C" fn komeda_platform_shutdown(pdev: *mut platform_device) {
    static void komeda_platform_shutdown(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct komeda_drv *mdrv = dev_get_drvdata(dev);
    komeda_kms_shutdown(mdrv.kms);
    }
#[no_mangle]
unsafe extern "C" fn komeda_platform_probe(pdev: *mut platform_device) -> c_int {
    static int komeda_platform_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct komeda_drv *mdrv;
    int err;
    err = dma_set_mask_and_coherent(dev, DMA_BIT_MASK(40));
    if (err)
    return dev_err_probe(dev, err, "DMA mask error\n");
    mdrv = devm_kzalloc(dev, sizeof(*mdrv), GFP_KERNEL);
    if (!mdrv)
    return -ENOMEM;
    mdrv.mdev = komeda_dev_create(dev);
    if (IS_ERR(mdrv.mdev)) {
    err = PTR_ERR(mdrv.mdev);
    goto free_mdrv;
    }
    pm_runtime_enable(dev);
    if (!pm_runtime_enabled(dev)) {
    err = komeda_dev_resume(mdrv.mdev);
    if (err)
    goto err_destroy_mdev;
    }
    mdrv.kms = komeda_kms_attach(mdrv.mdev);
    if (IS_ERR(mdrv.kms)) {
    err = PTR_ERR(mdrv.kms);
    goto destroy_mdev;
    }
    dev_set_drvdata(dev, mdrv);
    drm_client_setup(&mdrv.kms.base, core::ptr::null_mut());
    return 0;
    destroy_mdev:
    if (pm_runtime_enabled(dev))
    pm_runtime_disable(dev);
    else
    komeda_dev_suspend(mdrv.mdev);
    err_destroy_mdev:
    komeda_dev_destroy(mdrv.mdev);
    free_mdrv:
    devm_kfree(dev, mdrv);
    return err;
    }
    static const struct of_device_id komeda_of_match[] = {
    { .compatible = "arm,mali-d71", .data = d71_identify, },
    { .compatible = "arm,mali-d32", .data = d71_identify, },
    { .compatible = "armchina,linlon-d6", .data = d71_identify, },
    {},
    };
    MODULE_DEVICE_TABLE(of, komeda_of_match);
#[no_mangle]
unsafe extern "C" fn komeda_rt_pm_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused komeda_rt_pm_suspend(struct device *dev)
    {
    struct komeda_drv *mdrv = dev_get_drvdata(dev);
    return komeda_dev_suspend(mdrv.mdev);
    }
#[no_mangle]
unsafe extern "C" fn komeda_rt_pm_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused komeda_rt_pm_resume(struct device *dev)
    {
    struct komeda_drv *mdrv = dev_get_drvdata(dev);
    return komeda_dev_resume(mdrv.mdev);
    }
#[no_mangle]
unsafe extern "C" fn komeda_pm_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused komeda_pm_suspend(struct device *dev)
    {
    struct komeda_drv *mdrv = dev_get_drvdata(dev);
    int res;
    res = drm_mode_config_helper_suspend(&mdrv.kms.base);
    if (!pm_runtime_status_suspended(dev))
    komeda_dev_suspend(mdrv.mdev);
    return res;
    }
#[no_mangle]
unsafe extern "C" fn komeda_pm_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused komeda_pm_resume(struct device *dev)
    {
    struct komeda_drv *mdrv = dev_get_drvdata(dev);
    let mut err: c_int = 0;
    if (!pm_runtime_status_suspended(dev))
    err = komeda_dev_resume(mdrv.mdev);
    return err ? err : drm_mode_config_helper_resume(&mdrv.kms.base);
    }
    static const struct dev_pm_ops komeda_pm_ops = {
    SET_SYSTEM_SLEEP_PM_OPS(komeda_pm_suspend, komeda_pm_resume)
    SET_RUNTIME_PM_OPS(komeda_rt_pm_suspend, komeda_rt_pm_resume, core::ptr::null_mut())
    };
    static struct platform_driver komeda_platform_driver = {
    .probe	= komeda_platform_probe,
    .remove = komeda_platform_remove,
    .shutdown = komeda_platform_shutdown,
    .driver	= {
    .name = "komeda",
    .of_match_table	= komeda_of_match,
    .pm = &komeda_pm_ops,
    },
    };
    drm_module_platform_driver(komeda_platform_driver);
    MODULE_AUTHOR("James.Qian.Wang <james.qian.wang@arm.com>");
    MODULE_DESCRIPTION("Komeda KMS driver");
    MODULE_LICENSE("GPL v2");
