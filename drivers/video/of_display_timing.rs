//! Automatically rewritten from C to Rust
//! Source: drivers/video/of_display_timing.c
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
// OF helpers for parsing display timings
//
// Copyright (c) 2012 Steffen Trumtrar <s.trumtrar@pengutronix.de>, Pengutronix
//
// based on of_videomode.c by Sascha Hauer <s.hauer@pengutronix.de>
//

//
// parse_timing_property - parse timing_entry from device_node
// @np: device_node with the property
// @name: name of the property
// @result: will be set to the return value
//
// DESCRIPTION:
// Every display_timing can be specified with either just the typical value or
// a range consisting of min/typ/max. This function helps handling this
//
    static int parse_timing_property(const struct device_node *np, const char *name,
    struct timing_entry *result)
    {
    struct property *prop;
    int length, cells, ret;
    prop = of_find_property(np, name, &length);
    if (!prop) {
    pr_err("%pOF: could not find property %s\n", np, name);
    return -EINVAL;
    }
    cells = length / sizeof(u32);
    if (cells == 1) {
    ret = of_property_read_u32(np, name, &result.typ);
    result.min = result.typ;
    result.max = result.typ;
    } else if (cells == 3) {
    ret = of_property_read_u32_array(np, name, &result.min, cells);
    } else {
    pr_err("%pOF: illegal timing specification in %s\n", np, name);
    return -EINVAL;
    }
    return ret;
    }
//
// of_parse_display_timing - parse display_timing entry from device_node
// @np: device_node with the properties
// @dt: display_timing that contains the result. I may be partially written in case of errors
//
    static int of_parse_display_timing(const struct device_node *np,
    struct display_timing *dt)
    {
    let mut val: u32 = 0;
    let mut ret: c_int = 0;
    memset(dt, 0, sizeof(*dt));
    ret |= parse_timing_property(np, "hback-porch", &dt.hback_porch);
    ret |= parse_timing_property(np, "hfront-porch", &dt.hfront_porch);
    ret |= parse_timing_property(np, "hactive", &dt.hactive);
    ret |= parse_timing_property(np, "hsync-len", &dt.hsync_len);
    ret |= parse_timing_property(np, "vback-porch", &dt.vback_porch);
    ret |= parse_timing_property(np, "vfront-porch", &dt.vfront_porch);
    ret |= parse_timing_property(np, "vactive", &dt.vactive);
    ret |= parse_timing_property(np, "vsync-len", &dt.vsync_len);
    ret |= parse_timing_property(np, "clock-frequency", &dt.pixelclock);
    dt.flags = 0;
    if (!of_property_read_u32(np, "vsync-active", &val))
    dt.flags |= val ? DISPLAY_FLAGS_VSYNC_HIGH :
    DISPLAY_FLAGS_VSYNC_LOW;
    if (!of_property_read_u32(np, "hsync-active", &val))
    dt.flags |= val ? DISPLAY_FLAGS_HSYNC_HIGH :
    DISPLAY_FLAGS_HSYNC_LOW;
    if (!of_property_read_u32(np, "de-active", &val))
    dt.flags |= val ? DISPLAY_FLAGS_DE_HIGH :
    DISPLAY_FLAGS_DE_LOW;
    if (!of_property_read_u32(np, "pixelclk-active", &val))
    dt.flags |= val ? DISPLAY_FLAGS_PIXDATA_POSEDGE :
    DISPLAY_FLAGS_PIXDATA_NEGEDGE;
    if (!of_property_read_u32(np, "syncclk-active", &val))
    dt.flags |= val ? DISPLAY_FLAGS_SYNC_POSEDGE :
    DISPLAY_FLAGS_SYNC_NEGEDGE;
    else if (dt.flags & (DISPLAY_FLAGS_PIXDATA_POSEDGE |
    DISPLAY_FLAGS_PIXDATA_NEGEDGE))
    dt.flags |= dt.flags & DISPLAY_FLAGS_PIXDATA_POSEDGE ?
    DISPLAY_FLAGS_SYNC_POSEDGE :
    DISPLAY_FLAGS_SYNC_NEGEDGE;
    if (of_property_read_bool(np, "interlaced"))
    dt.flags |= DISPLAY_FLAGS_INTERLACED;
    if (of_property_read_bool(np, "doublescan"))
    dt.flags |= DISPLAY_FLAGS_DOUBLESCAN;
    if (of_property_read_bool(np, "doubleclk"))
    dt.flags |= DISPLAY_FLAGS_DOUBLECLK;
    if (ret) {
    pr_err("%pOF: error reading timing properties\n", np);
    return -EINVAL;
    }
    return 0;
    }
//
// of_get_display_timing - parse a display_timing entry
// @np: device_node with the timing subnode
// @name: name of the timing node
// @dt: display_timing struct to fill
//
    int of_get_display_timing(const struct device_node *np, const char *name,
    struct display_timing *dt)
    {
    struct device_node *timing_np;
    int ret;
    if (!np)
    return -EINVAL;
    timing_np = of_get_child_by_name(np, name);
    if (!timing_np)
    return -ENOENT;
    ret = of_parse_display_timing(timing_np, dt);
    of_node_put(timing_np);
    return ret;
    }
    EXPORT_SYMBOL_GPL(of_get_display_timing);
//
// of_get_display_timings - parse all display_timing entries from a device_node
// @np: device_node with the subnodes
//
    struct display_timings *of_get_display_timings(const struct device_node *np)
    {
    struct device_node *timings_np;
    struct device_node *entry;
    struct device_node *native_mode;
    struct display_timings *disp;
    if (!np)
    return core::ptr::null_mut();
    timings_np = of_get_child_by_name(np, "display-timings");
    if (!timings_np) {
    pr_err("%pOF: could not find display-timings node\n", np);
    return core::ptr::null_mut();
    }
    disp = kzalloc_obj(*disp);
    if (!disp) {
    pr_err("%pOF: could not allocate struct disp'\n", np);
    goto dispfail;
    }
    entry = of_parse_phandle(timings_np, "native-mode", 0);
// assume first child as native mode if none provided
    if (!entry)
    entry = of_get_next_child(timings_np, core::ptr::null_mut());
// if there is no child, it is useless to go on
    if (!entry) {
    pr_err("%pOF: no timing specifications given\n", np);
    goto entryfail;
    }
    pr_debug("%pOF: using %pOFn as default timing\n", np, entry);
    native_mode = entry;
    disp.num_timings = of_get_child_count(timings_np);
    if (disp.num_timings == 0) {
// should never happen, as entry was already found above
    pr_err("%pOF: no timings specified\n", np);
    goto timingfail;
    }
    disp.timings = kzalloc_objs(struct display_timing *, disp.num_timings);
    if (!disp.timings) {
    pr_err("%pOF: could not allocate timings array\n", np);
    goto timingfail;
    }
    disp.num_timings = 0;
    disp.native_mode = 0;
    for_each_child_of_node_scoped(timings_np, child) {
    struct display_timing *dt;
    int r;
    dt = kmalloc_obj(*dt);
    if (!dt) {
    pr_err("%pOF: could not allocate display_timing struct\n",
    np);
    goto timingfail;
    }
    r = of_parse_display_timing(child, dt);
    if (r) {
//
// to not encourage wrong devicetrees, fail in case of
// an error
//
    pr_err("%pOF: error in timing %d\n",
    np, disp.num_timings + 1);
    kfree(dt);
    goto timingfail;
    }
    if (native_mode == child)
    disp.native_mode = disp.num_timings;
    disp.timings[disp.num_timings] = dt;
    disp.num_timings++;
    }
    of_node_put(timings_np);
//
// native_mode points to the device_node returned by of_parse_phandle
// therefore call of_node_put on it
//
    of_node_put(native_mode);
    pr_debug("%pOF: got %d timings. Using timing #%d as default\n",
    np, disp.num_timings,
    disp.native_mode + 1);
    return disp;
    timingfail:
    of_node_put(native_mode);
    display_timings_release(disp);
    disp = core::ptr::null_mut();
    entryfail:
    kfree(disp);
    dispfail:
    of_node_put(timings_np);
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(of_get_display_timings);
