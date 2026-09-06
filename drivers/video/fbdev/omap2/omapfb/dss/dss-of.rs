//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/omap2/omapfb/dss/dss-of.c
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
// Copyright (C) 2013 Texas Instruments
// Author: Tomi Valkeinen <tomi.valkeinen@ti.com>
//

    struct device_node *dss_of_port_get_parent_device(struct device_node *port)
    {
    struct device_node *np;
    int i;
    if (!port)
    return core::ptr::null_mut();
    np = of_get_parent(port);
    for (i = 0; i < 2 && np; ++i) {
    struct property *prop;
    prop = of_find_property(np, "compatible", core::ptr::null_mut());
    if (prop)
    return np;
    np = of_get_next_parent(np);
    }
    of_node_put(np);
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn dss_of_port_get_port_number(port: *mut device_node) -> u32 {
    u32 dss_of_port_get_port_number(struct device_node *port)
    {
    int r;
    u32 reg;
    r = of_property_read_u32(port, "reg", &reg);
    if (r)
    reg = 0;
    return reg;
    }
    struct omap_dss_device *
    omapdss_of_find_source_for_first_ep(struct device_node *node)
    {
    struct device_node *ep;
    struct device_node *src_port;
    struct omap_dss_device *src;
    ep = of_graph_get_endpoint_by_regs(node, 0, -1);
    if (!ep)
    return ERR_PTR(-EINVAL);
    src_port = of_graph_get_remote_port(ep);
    of_node_put(ep);
    if (!src_port)
    return ERR_PTR(-EINVAL);
    src = omap_dss_find_output_by_port_node(src_port);
    of_node_put(src_port);
    return src ? src : ERR_PTR(-EPROBE_DEFER);
    }
    EXPORT_SYMBOL_GPL(omapdss_of_find_source_for_first_ep);
