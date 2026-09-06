//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/bridge/ti-tdp158.c
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
//
// Copyright 2024 Freebox SAS
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tdp158 {
    pub bridge: drm_bridge,
    pub next: *mut drm_bridge,
    pub 36: *mut *mut gpio_desc enable; // Operation Enable - pin,
    pub 3.3V: *mut *mut regulator vcc; //,
    pub 1.1V: *mut *mut regulator vdd; //,
    pub dev: *mut device,
}

    static void tdp158_enable(struct drm_bridge *bridge,
    struct drm_atomic_commit *state)
    {
    int err;
    struct tdp158 *tdp158 = bridge.driver_private;
    err = regulator_enable(tdp158.vcc);
    if (err)
    dev_err(tdp158.dev, "failed to enable vcc: %d", err);
    err = regulator_enable(tdp158.vdd);
    if (err)
    dev_err(tdp158.dev, "failed to enable vdd: %d", err);
    gpiod_set_value_cansleep(tdp158.enable, 1);
    }
    static void tdp158_disable(struct drm_bridge *bridge,
    struct drm_atomic_commit *state)
    {
    struct tdp158 *tdp158 = bridge.driver_private;
    gpiod_set_value_cansleep(tdp158.enable, 0);
    regulator_disable(tdp158.vdd);
    regulator_disable(tdp158.vcc);
    }
    static int tdp158_attach(struct drm_bridge *bridge,
    struct drm_encoder *encoder,
    enum drm_bridge_attach_flags flags)
    {
    struct tdp158 *tdp158 = bridge.driver_private;
    return drm_bridge_attach(encoder, tdp158.next, bridge, flags);
    }
    static const struct drm_bridge_funcs tdp158_bridge_funcs = {
    .attach = tdp158_attach,
    .atomic_enable = tdp158_enable,
    .atomic_disable = tdp158_disable,
    .atomic_duplicate_state = drm_atomic_helper_bridge_duplicate_state,
    .atomic_destroy_state = drm_atomic_helper_bridge_destroy_state,
    .atomic_create_state = drm_atomic_helper_bridge_create_state,
    };
#[no_mangle]
unsafe extern "C" fn tdp158_probe(client: *mut i2c_client) -> c_int {
    static int tdp158_probe(struct i2c_client *client)
    {
    struct tdp158 *tdp158;
    struct device *dev = &client.dev;
    tdp158 = devm_drm_bridge_alloc(dev, struct tdp158, bridge,
    &tdp158_bridge_funcs);
    if (IS_ERR(tdp158))
    return PTR_ERR(tdp158);
    tdp158.next = devm_drm_of_get_bridge(dev, dev.of_node, 1, 0);
    if (IS_ERR(tdp158.next))
    return dev_err_probe(dev, PTR_ERR(tdp158.next), "missing bridge");
    tdp158.vcc = devm_regulator_get(dev, "vcc");
    if (IS_ERR(tdp158.vcc))
    return dev_err_probe(dev, PTR_ERR(tdp158.vcc), "vcc");
    tdp158.vdd = devm_regulator_get(dev, "vdd");
    if (IS_ERR(tdp158.vdd))
    return dev_err_probe(dev, PTR_ERR(tdp158.vdd), "vdd");
    tdp158.enable = devm_gpiod_get_optional(dev, "enable", GPIOD_OUT_LOW);
    if (IS_ERR(tdp158.enable))
    return dev_err_probe(dev, PTR_ERR(tdp158.enable), "enable");
    tdp158.bridge.of_node = dev.of_node;
    tdp158.bridge.driver_private = tdp158;
    tdp158.dev = dev;
    return devm_drm_bridge_add(dev, &tdp158.bridge);
    }
    static const struct of_device_id tdp158_match_table[] = {
    { .compatible = "ti,tdp158" },
    { }
    };
    MODULE_DEVICE_TABLE(of, tdp158_match_table);
    static struct i2c_driver tdp158_driver = {
    .probe = tdp158_probe,
    .driver = {
    .name = "tdp158",
    .of_match_table = tdp158_match_table,
    },
    };
    module_i2c_driver(tdp158_driver);
    MODULE_DESCRIPTION("TI TDP158 driver");
    MODULE_LICENSE("GPL");
