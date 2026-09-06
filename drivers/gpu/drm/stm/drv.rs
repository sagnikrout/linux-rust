//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/stm/drv.c
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
// Copyright (C) STMicroelectronics SA 2017
//
// Authors: Philippe Cornu <philippe.cornu@st.com>
// Yannick Fertre <yannick.fertre@st.com>
// Fabien Dessenne <fabien.dessenne@st.com>
// Mickael Reulier <mickael.reulier@st.com>
//

pub const STM_MAX_FB_WIDTH: c_int = 2048;

    static const struct drm_mode_config_funcs drv_mode_config_funcs = {
    .fb_create = drm_gem_fb_create,
    .atomic_check = drm_atomic_helper_check,
    .atomic_commit = drm_atomic_helper_commit,
    };
    static int stm_gem_dma_dumb_create(struct drm_file *file,
    struct drm_device *dev,
    struct drm_mode_create_dumb *args)
    {
    let mut min_pitch: c_uint = DIV_ROUND_UP(args.width * args.bpp, 8);
//
// in order to optimize data transfer, pitch is aligned on
// 128 bytes, height is aligned on 4 bytes
//
    args.pitch = roundup(min_pitch, 128);
    args.height = roundup(args.height, 4);
    return drm_gem_dma_dumb_create_internal(file, dev, args);
    }
    DEFINE_DRM_GEM_DMA_FOPS(drv_driver_fops);
    static const struct drm_driver drv_driver = {
    .driver_features = DRIVER_MODESET | DRIVER_GEM | DRIVER_ATOMIC,
    .name = "stm",
    .desc = "STMicroelectronics SoC DRM",
    .major = 1,
    .minor = 0,
    .patchlevel = 0,
    .fops = &drv_driver_fops,
    DRM_GEM_DMA_DRIVER_OPS_WITH_DUMB_CREATE(stm_gem_dma_dumb_create),
    DRM_FBDEV_DMA_DRIVER_OPS,
    };
#[no_mangle]
unsafe extern "C" fn drv_load(ddev: *mut drm_device) -> c_int {
    static int drv_load(struct drm_device *ddev)
    {
    struct platform_device *pdev = to_platform_device(ddev.dev);
    struct ltdc_device *ldev;
    int ret;
    DRM_DEBUG("%s\n", __func__);
    ldev = drmm_kzalloc(ddev, sizeof(*ldev), GFP_KERNEL);
    if (!ldev)
    return -ENOMEM;
    ddev.dev_private = (void *)ldev;
    ret = drmm_mode_config_init(ddev);
    if (ret)
    return ret;
//
// set max width and height as default value.
// this value would be used to check framebuffer size limitation
// at drm_mode_addfb().
//
    ddev.mode_config.min_width = 0;
    ddev.mode_config.min_height = 0;
    ddev.mode_config.max_width = STM_MAX_FB_WIDTH;
    ddev.mode_config.max_height = STM_MAX_FB_HEIGHT;
    ddev.mode_config.funcs = &drv_mode_config_funcs;
    ddev.mode_config.normalize_zpos = true;
    ret = ltdc_load(ddev);
    if (ret)
    return ret;
    drm_mode_config_reset(ddev);
    drm_kms_helper_poll_init(ddev);
    platform_set_drvdata(pdev, ddev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn drv_unload(ddev: *mut drm_device) {
    static void drv_unload(struct drm_device *ddev)
    {
    DRM_DEBUG("%s\n", __func__);
    drm_kms_helper_poll_fini(ddev);
    drm_atomic_helper_shutdown(ddev);
    ltdc_unload(ddev);
    }
#[no_mangle]
unsafe extern "C" fn drv_suspend(dev: *mut device) -> __maybe_unused int {
    static __maybe_unused int drv_suspend(struct device *dev)
    {
    struct drm_device *ddev = dev_get_drvdata(dev);
    struct ltdc_device *ldev = ddev.dev_private;
    struct drm_atomic_commit *state;
    WARN_ON(ldev.suspend_state);
    state = drm_atomic_helper_suspend(ddev);
    if (IS_ERR(state))
    return PTR_ERR(state);
    ldev.suspend_state = state;
    pm_runtime_force_suspend(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn drv_resume(dev: *mut device) -> __maybe_unused int {
    static __maybe_unused int drv_resume(struct device *dev)
    {
    struct drm_device *ddev = dev_get_drvdata(dev);
    struct ltdc_device *ldev = ddev.dev_private;
    int ret;
    if (WARN_ON(!ldev.suspend_state))
    return -ENOENT;
    pm_runtime_force_resume(dev);
    ret = drm_atomic_helper_resume(ddev, ldev.suspend_state);
    if (ret)
    pm_runtime_force_suspend(dev);
    ldev.suspend_state = core::ptr::null_mut();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn drv_runtime_suspend(dev: *mut device) -> __maybe_unused int {
    static __maybe_unused int drv_runtime_suspend(struct device *dev)
    {
    struct drm_device *ddev = dev_get_drvdata(dev);
    DRM_DEBUG_DRIVER("\n");
    ltdc_suspend(ddev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn drv_runtime_resume(dev: *mut device) -> __maybe_unused int {
    static __maybe_unused int drv_runtime_resume(struct device *dev)
    {
    struct drm_device *ddev = dev_get_drvdata(dev);
    DRM_DEBUG_DRIVER("\n");
    return ltdc_resume(ddev);
    }
    static const struct dev_pm_ops drv_pm_ops = {
    SET_SYSTEM_SLEEP_PM_OPS(drv_suspend, drv_resume)
    SET_RUNTIME_PM_OPS(drv_runtime_suspend,
    drv_runtime_resume, core::ptr::null_mut())
    };
#[no_mangle]
unsafe extern "C" fn stm_drm_platform_probe(pdev: *mut platform_device) -> c_int {
    static int stm_drm_platform_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct drm_device *ddev;
    int ret;
    DRM_DEBUG("%s\n", __func__);
    ret = aperture_remove_all_conflicting_devices(drv_driver.name);
    if (ret)
    return ret;
    dma_set_coherent_mask(dev, DMA_BIT_MASK(32));
    ddev = drm_dev_alloc(&drv_driver, dev);
    if (IS_ERR(ddev))
    return PTR_ERR(ddev);
    ret = drv_load(ddev);
    if (ret)
    goto err_put;
    ret = drm_dev_register(ddev, 0);
    if (ret)
    goto err_unload;
    drm_client_setup_with_fourcc(ddev, DRM_FORMAT_RGB565);
    return 0;
    err_unload:
    drv_unload(ddev);
    err_put:
    drm_dev_put(ddev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stm_drm_platform_remove(pdev: *mut platform_device) {
    static void stm_drm_platform_remove(struct platform_device *pdev)
    {
    struct drm_device *ddev = platform_get_drvdata(pdev);
    DRM_DEBUG("%s\n", __func__);
    drm_dev_unregister(ddev);
    drv_unload(ddev);
    drm_dev_put(ddev);
    }
#[no_mangle]
unsafe extern "C" fn stm_drm_platform_shutdown(pdev: *mut platform_device) {
    static void stm_drm_platform_shutdown(struct platform_device *pdev)
    {
    drm_atomic_helper_shutdown(platform_get_drvdata(pdev));
    }
    static struct ltdc_plat_data stm_drm_plat_data = {
    .pad_max_freq_hz = 90000000,
    };
    static struct ltdc_plat_data stm_drm_plat_data_mp25 = {
    .pad_max_freq_hz = 150000000,
    };
    static const struct of_device_id drv_dt_ids[] = {
    { .compatible = "st,stm32-ltdc", .data = &stm_drm_plat_data, },
    { .compatible = "st,stm32mp251-ltdc", .data = &stm_drm_plat_data_mp25, },
    { .compatible = "st,stm32mp255-ltdc", .data = &stm_drm_plat_data_mp25, },
    { /* end node */ },
    };
    MODULE_DEVICE_TABLE(of, drv_dt_ids);
    static struct platform_driver stm_drm_platform_driver = {
    .probe = stm_drm_platform_probe,
    .remove = stm_drm_platform_remove,
    .shutdown = stm_drm_platform_shutdown,
    .driver = {
    .name = "stm32-display",
    .of_match_table = drv_dt_ids,
    .pm = &drv_pm_ops,
    },
    };
    drm_module_platform_driver(stm_drm_platform_driver);
    MODULE_AUTHOR("Philippe Cornu <philippe.cornu@st.com>");
    MODULE_AUTHOR("Yannick Fertre <yannick.fertre@st.com>");
    MODULE_AUTHOR("Fabien Dessenne <fabien.dessenne@st.com>");
    MODULE_AUTHOR("Mickael Reulier <mickael.reulier@st.com>");
    MODULE_DESCRIPTION("STMicroelectronics ST DRM LTDC driver");
    MODULE_LICENSE("GPL v2");
