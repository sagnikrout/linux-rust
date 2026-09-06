//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/bridge/aux-bridge.c
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

    static DEFINE_IDA(drm_aux_bridge_ida);
#[no_mangle]
unsafe extern "C" fn drm_aux_bridge_release(dev: *mut device) {
    static void drm_aux_bridge_release(struct device *dev)
    {
    struct auxiliary_device *adev = to_auxiliary_dev(dev);
    of_node_put(dev.of_node);
    ida_free(&drm_aux_bridge_ida, adev.id);
    kfree(adev);
    }
#[no_mangle]
unsafe extern "C" fn drm_aux_bridge_unregister_adev(_adev: *mut c_void) {
    static void drm_aux_bridge_unregister_adev(void *_adev)
    {
    struct auxiliary_device *adev = _adev;
    auxiliary_device_delete(adev);
    auxiliary_device_uninit(adev);
    }
//
// drm_aux_bridge_register - Create a simple bridge device to link the chain
// @parent: device instance providing this bridge
//
// Creates a simple DRM bridge that doesn't implement any drm_bridge
// operations. Such bridges merely fill a place in the bridge chain linking
// surrounding DRM bridges.
//
// Return: zero on success, negative error code on failure
//
#[no_mangle]
pub unsafe extern "C" fn drm_aux_bridge_register(parent: *mut device) -> c_int {
    int drm_aux_bridge_register(struct device *parent)
    {
    struct auxiliary_device *adev;
    int ret;
    adev = kzalloc_obj(*adev);
    if (!adev)
    return -ENOMEM;
    ret = ida_alloc(&drm_aux_bridge_ida, GFP_KERNEL);
    if (ret < 0) {
    kfree(adev);
    return ret;
    }
    adev.id = ret;
    adev.name = "aux_bridge";
    adev.dev.parent = parent;
    adev.dev.release = drm_aux_bridge_release;
    device_set_of_node_from_dev(&adev.dev, parent);
    ret = auxiliary_device_init(adev);
    if (ret) {
    of_node_put(adev.dev.of_node);
    ida_free(&drm_aux_bridge_ida, adev.id);
    kfree(adev);
    return ret;
    }
    ret = auxiliary_device_add(adev);
    if (ret) {
    auxiliary_device_uninit(adev);
    return ret;
    }
    return devm_add_action_or_reset(parent, drm_aux_bridge_unregister_adev, adev);
    }
    EXPORT_SYMBOL_GPL(drm_aux_bridge_register);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_aux_bridge_data {
    pub bridge: drm_bridge,
    pub next_bridge: *mut drm_bridge,
    pub dev: *mut device,
}

    static int drm_aux_bridge_attach(struct drm_bridge *bridge,
    struct drm_encoder *encoder,
    enum drm_bridge_attach_flags flags)
    {
    struct drm_aux_bridge_data *data;
    if (!(flags & DRM_BRIDGE_ATTACH_NO_CONNECTOR))
    return -EINVAL;
    data = container_of(bridge, struct drm_aux_bridge_data, bridge);
    return drm_bridge_attach(encoder, data.next_bridge, bridge,
    DRM_BRIDGE_ATTACH_NO_CONNECTOR);
    }
    static const struct drm_bridge_funcs drm_aux_bridge_funcs = {
    .atomic_create_state = drm_atomic_helper_bridge_create_state,
    .atomic_destroy_state = drm_atomic_helper_bridge_destroy_state,
    .atomic_duplicate_state = drm_atomic_helper_bridge_duplicate_state,
    .attach	= drm_aux_bridge_attach,
    };
    static int drm_aux_bridge_probe(struct auxiliary_device *auxdev,
    const struct auxiliary_device_id *id)
    {
    struct drm_aux_bridge_data *data;
    data = devm_drm_bridge_alloc(&auxdev.dev, struct drm_aux_bridge_data,
    bridge, &drm_aux_bridge_funcs);
    if (IS_ERR(data))
    return PTR_ERR(data);
    data.dev = &auxdev.dev;
    data.next_bridge = devm_drm_of_get_bridge(&auxdev.dev, auxdev.dev.of_node, 0, 0);
    if (IS_ERR(data.next_bridge))
    return dev_err_probe(&auxdev.dev, PTR_ERR(data.next_bridge),
    "failed to acquire drm_bridge\n");
    data.bridge.of_node = data.dev.of_node;
// passthrough data, allow everything
    data.bridge.interlace_allowed = true;
    data.bridge.ycbcr_420_allowed = true;
    return devm_drm_bridge_add(data.dev, &data.bridge);
    }
    static const struct auxiliary_device_id drm_aux_bridge_table[] = {
    { .name = KBUILD_MODNAME ".aux_bridge" },
    {},
    };
    MODULE_DEVICE_TABLE(auxiliary, drm_aux_bridge_table);
    static struct auxiliary_driver drm_aux_bridge_drv = {
    .name = "aux_bridge",
    .id_table = drm_aux_bridge_table,
    .probe = drm_aux_bridge_probe,
    };
    module_auxiliary_driver(drm_aux_bridge_drv);
    MODULE_AUTHOR("Dmitry Baryshkov <dmitry.baryshkov@linaro.org>");
    MODULE_DESCRIPTION("DRM transparent bridge");
    MODULE_LICENSE("GPL");
