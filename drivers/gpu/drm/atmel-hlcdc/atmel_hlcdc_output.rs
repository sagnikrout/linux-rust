//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/atmel-hlcdc/atmel_hlcdc_output.c
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
// Copyright (C) 2014 Traphandler
// Copyright (C) 2014 Free Electrons
// Copyright (C) 2014 Atmel
//
// Author: Jean-Jacques Hiblot <jjhiblot@traphandler.com>
// Author: Boris BREZILLON <boris.brezillon@free-electrons.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atmel_hlcdc_rgb_output {
    pub encoder: drm_encoder,
    pub bus_fmt: c_int,
}

    static struct atmel_hlcdc_rgb_output *
    atmel_hlcdc_encoder_to_rgb_output(struct drm_encoder *encoder)
    {
    return container_of(encoder, struct atmel_hlcdc_rgb_output, encoder);
    }
#[no_mangle]
pub unsafe extern "C" fn atmel_hlcdc_encoder_get_bus_fmt(encoder: *mut drm_encoder) -> c_int {
    int atmel_hlcdc_encoder_get_bus_fmt(struct drm_encoder *encoder)
    {
    struct atmel_hlcdc_rgb_output *output;
    output = atmel_hlcdc_encoder_to_rgb_output(encoder);
    return output.bus_fmt;
    }
#[no_mangle]
unsafe extern "C" fn atmel_hlcdc_of_bus_fmt(ep: *const device_node) -> c_int {
    static int atmel_hlcdc_of_bus_fmt(const struct device_node *ep)
    {
    u32 bus_width;
    int ret;
    ret = of_property_read_u32(ep, "bus-width", &bus_width);
    if (ret == -EINVAL)
    return 0;
    if (ret)
    return ret;
    switch (bus_width) {
    case 12:
    return MEDIA_BUS_FMT_RGB444_1X12;
    case 16:
    return MEDIA_BUS_FMT_RGB565_1X16;
    case 18:
    return MEDIA_BUS_FMT_RGB666_1X18;
    case 24:
    return MEDIA_BUS_FMT_RGB888_1X24;
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn atmel_hlcdc_attach_endpoint(dev: *mut drm_device, endpoint: c_int) -> c_int {
    static int atmel_hlcdc_attach_endpoint(struct drm_device *dev, int endpoint)
    {
    struct atmel_hlcdc_rgb_output *output;
    struct device_node *ep;
    struct drm_bridge *bridge;
    struct atmel_hlcdc_dc *dc = dev.dev_private;
    struct drm_crtc *crtc = dc.crtc;
    let mut ret: c_int = 0;
    bridge = devm_drm_of_get_bridge(dev.dev, dev.dev.of_node, 0, endpoint);
    if (IS_ERR(bridge))
    return PTR_ERR(bridge);
    output = drmm_simple_encoder_alloc(dev, struct atmel_hlcdc_rgb_output,
    encoder, DRM_MODE_ENCODER_NONE);
    if (IS_ERR(output))
    return PTR_ERR(output);
    ep = of_graph_get_endpoint_by_regs(dev.dev.of_node, 0, endpoint);
    if (!ep)
    return -ENODEV;
    output.bus_fmt = atmel_hlcdc_of_bus_fmt(ep);
    of_node_put(ep);
    if (output.bus_fmt < 0) {
    drm_err(dev, "endpoint %d: invalid bus width\n", endpoint);
    return -EINVAL;
    }
    output.encoder.possible_crtcs = drm_crtc_mask(crtc);
    if (bridge)
    ret = drm_bridge_attach(&output.encoder, bridge, core::ptr::null_mut(), 0);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn atmel_hlcdc_create_outputs(dev: *mut drm_device) -> c_int {
    int atmel_hlcdc_create_outputs(struct drm_device *dev)
    {
    int endpoint, ret = 0;
    let mut attached: c_int = 0;
//
// Always scan the first few endpoints even if we get -ENODEV,
// but keep going after that as long as we keep getting hits.
//
    for (endpoint = 0; !ret || endpoint < 4; endpoint++) {
    ret = atmel_hlcdc_attach_endpoint(dev, endpoint);
    if (ret == -ENODEV)
    continue;
    if (ret)
    break;
    attached++;
    }
// At least one device was successfully attached.
    if (ret == -ENODEV && attached)
    return 0;
    return ret;
    }
