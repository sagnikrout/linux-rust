//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/bridge/aux-hpd-bridge.c
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
// Copyright (C) 2023 Linaro Ltd.
//
// Author: Dmitry Baryshkov <dmitry.baryshkov@linaro.org>
//

    static DEFINE_IDA(drm_aux_hpd_bridge_ida);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_aux_hpd_bridge_data {
    pub bridge: drm_bridge,
    pub dev: *mut device,
}

#[no_mangle]
unsafe extern "C" fn drm_aux_hpd_bridge_release(dev: *mut device) {
    static void drm_aux_hpd_bridge_release(struct device *dev)
    {
    struct auxiliary_device *adev = to_auxiliary_dev(dev);
    ida_free(&drm_aux_hpd_bridge_ida, adev.id);
    of_node_put(adev.dev.platform_data);
    of_node_put(adev.dev.of_node);
    kfree(adev);
    }
#[no_mangle]
unsafe extern "C" fn drm_aux_hpd_bridge_free_adev(_adev: *mut c_void) {
    static void drm_aux_hpd_bridge_free_adev(void *_adev)
    {
    auxiliary_device_uninit(_adev);
    }
//
// devm_drm_dp_hpd_bridge_alloc - allocate a HPD DisplayPort bridge
// @parent: device instance providing this bridge
// @np: device node pointer corresponding to this bridge instance
//
// Creates a simple DRM bridge with the type set to
// DRM_MODE_CONNECTOR_DisplayPort, which terminates the bridge chain and is
// able to send the HPD events.
//
// Return: bridge auxiliary device pointer or an error pointer
//
    struct auxiliary_device *devm_drm_dp_hpd_bridge_alloc(struct device *parent, struct device_node *np)
    {
    struct auxiliary_device *adev;
    int ret;
    adev = kzalloc_obj(*adev);
    if (!adev)
    return ERR_PTR(-ENOMEM);
    ret = ida_alloc(&drm_aux_hpd_bridge_ida, GFP_KERNEL);
    if (ret < 0) {
    kfree(adev);
    return ERR_PTR(ret);
    }
    adev.id = ret;
    adev.name = "dp_hpd_bridge";
    adev.dev.parent = parent;
    adev.dev.release = drm_aux_hpd_bridge_release;
    adev.dev.platform_data = of_node_get(np);
    device_set_of_node_from_dev(&adev.dev, parent);
    ret = auxiliary_device_init(adev);
    if (ret) {
    of_node_put(adev.dev.platform_data);
    of_node_put(adev.dev.of_node);
    ida_free(&drm_aux_hpd_bridge_ida, adev.id);
    kfree(adev);
    return ERR_PTR(ret);
    }
    ret = devm_add_action_or_reset(parent, drm_aux_hpd_bridge_free_adev, adev);
    if (ret)
    return ERR_PTR(ret);
    return adev;
    }
    EXPORT_SYMBOL_GPL(devm_drm_dp_hpd_bridge_alloc);
#[no_mangle]
unsafe extern "C" fn drm_aux_hpd_bridge_del_adev(_adev: *mut c_void) {
    static void drm_aux_hpd_bridge_del_adev(void *_adev)
    {
    auxiliary_device_delete(_adev);
    }
//
// devm_drm_dp_hpd_bridge_add - register a HDP DisplayPort bridge
// @dev: struct device to tie registration lifetime to
// @adev: bridge auxiliary device to be registered
//
// Returns: zero on success or a negative errno
//
#[no_mangle]
pub unsafe extern "C" fn devm_drm_dp_hpd_bridge_add(dev: *mut device, adev: *mut auxiliary_device) -> c_int {
    int devm_drm_dp_hpd_bridge_add(struct device *dev, struct auxiliary_device *adev)
    {
    int ret;
    ret = auxiliary_device_add(adev);
    if (ret)
    return ret;
    return devm_add_action_or_reset(dev, drm_aux_hpd_bridge_del_adev, adev);
    }
    EXPORT_SYMBOL_GPL(devm_drm_dp_hpd_bridge_add);
//
// drm_dp_hpd_bridge_register - allocate and register a HDP DisplayPort bridge
// @parent: device instance providing this bridge
// @np: device node pointer corresponding to this bridge instance
//
// Return: device instance that will handle created bridge or an error pointer
//
    struct device *drm_dp_hpd_bridge_register(struct device *parent, struct device_node *np)
    {
    struct auxiliary_device *adev;
    int ret;
    adev = devm_drm_dp_hpd_bridge_alloc(parent, np);
    if (IS_ERR(adev))
    return ERR_CAST(adev);
    ret = devm_drm_dp_hpd_bridge_add(parent, adev);
    if (ret)
    return ERR_PTR(ret);
    return &adev.dev;
    }
    EXPORT_SYMBOL_GPL(drm_dp_hpd_bridge_register);
//
// drm_aux_hpd_bridge_notify - notify hot plug detection events
// @dev: device created for the HPD bridge
// @status: output connection status
//
// A wrapper around drm_bridge_hpd_notify() that is used to report hot plug
// detection events for bridges created via drm_dp_hpd_bridge_register().
//
// This function shall be called in a context that can sleep.
//
#[no_mangle]
pub unsafe extern "C" fn drm_aux_hpd_bridge_notify(dev: *mut device, status: enum drm_connector_status) {
    void drm_aux_hpd_bridge_notify(struct device *dev, enum drm_connector_status status)
    {
    struct auxiliary_device *adev = to_auxiliary_dev(dev);
    struct drm_aux_hpd_bridge_data *data = auxiliary_get_drvdata(adev);
    if (!data)
    return;
    drm_bridge_hpd_notify(&data.bridge, status);
    }
    EXPORT_SYMBOL_GPL(drm_aux_hpd_bridge_notify);
    static int drm_aux_hpd_bridge_attach(struct drm_bridge *bridge,
    struct drm_encoder *encoder,
    enum drm_bridge_attach_flags flags)
    {
    return flags & DRM_BRIDGE_ATTACH_NO_CONNECTOR ? 0 : -EINVAL;
    }
    static const struct drm_bridge_funcs drm_aux_hpd_bridge_funcs = {
    .atomic_create_state = drm_atomic_helper_bridge_create_state,
    .atomic_destroy_state = drm_atomic_helper_bridge_destroy_state,
    .atomic_duplicate_state = drm_atomic_helper_bridge_duplicate_state,
    .attach	= drm_aux_hpd_bridge_attach,
    };
    static int drm_aux_hpd_bridge_probe(struct auxiliary_device *auxdev,
    const struct auxiliary_device_id *id)
    {
    struct drm_aux_hpd_bridge_data *data;
    data = devm_drm_bridge_alloc(&auxdev.dev,
    struct drm_aux_hpd_bridge_data, bridge,
    &drm_aux_hpd_bridge_funcs);
    if (IS_ERR(data))
    return PTR_ERR(data);
    data.dev = &auxdev.dev;
    data.bridge.of_node = dev_get_platdata(data.dev);
    data.bridge.ops = DRM_BRIDGE_OP_HPD;
    data.bridge.type = id.driver_data;
// passthrough data, allow everything
    data.bridge.interlace_allowed = true;
    data.bridge.ycbcr_420_allowed = true;
    auxiliary_set_drvdata(auxdev, data);
    return devm_drm_bridge_add(data.dev, &data.bridge);
    }
    static const struct auxiliary_device_id drm_aux_hpd_bridge_table[] = {
    { .name = KBUILD_MODNAME ".dp_hpd_bridge", .driver_data = DRM_MODE_CONNECTOR_DisplayPort, },
    {},
    };
    MODULE_DEVICE_TABLE(auxiliary, drm_aux_hpd_bridge_table);
    static struct auxiliary_driver drm_aux_hpd_bridge_drv = {
    .name = "aux_hpd_bridge",
    .id_table = drm_aux_hpd_bridge_table,
    .probe = drm_aux_hpd_bridge_probe,
    };
    module_auxiliary_driver(drm_aux_hpd_bridge_drv);
    MODULE_AUTHOR("Dmitry Baryshkov <dmitry.baryshkov@linaro.org>");
    MODULE_DESCRIPTION("DRM HPD bridge");
    MODULE_LICENSE("GPL");
