//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/tilcdc/tilcdc_panel_legacy.c
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
// Copyright (C) 2025 Bootlin
// Author: Kory Maincent <kory.maincent@bootlin.com>
//
// To support the legacy "ti,tilcdc,panel" binding, the devicetree has to
// be transformed to the new panel-dpi binding with the endpoint associated.
//

// Embedded dtbo symbols created by cmd_wrap_S_dtb in scripts/Makefile.lib
    extern char __dtbo_tilcdc_panel_legacy_begin[];
    extern char __dtbo_tilcdc_panel_legacy_end[];
    static int __init
    tilcdc_panel_update_prop(struct of_changeset *ocs, struct device_node *node,
    char *name, void *val, int length)
    {
    struct property *prop;
    prop = kzalloc_obj(*prop);
    if (!prop)
    return -ENOMEM;
    prop.name = kstrdup(name, GFP_KERNEL);
    prop.length = length;
    prop.value = kmemdup(val, length, GFP_KERNEL);
    if (!prop.name || !prop.value) {
    kfree(prop.name);
    kfree(prop.value);
    kfree(prop);
    return -ENOMEM;
    }
    return of_changeset_update_property(ocs, node, prop);
    }
    static int __init tilcdc_panel_copy_props(struct device_node *old_panel,
    struct device_node *new_panel)
    {
    struct device_node *old_timing __free(device_node) = core::ptr::null_mut();
    struct device_node *new_timing __free(device_node) = core::ptr::null_mut();
    struct device_node *panel_info __free(device_node) = core::ptr::null_mut();
    struct device_node *child __free(device_node) = core::ptr::null_mut();
    let mut invert_pxl_clk: u32 = 0, sync_edge = 0;
    struct of_changeset ocs;
    struct property *prop;
    int ret;
    child = of_get_child_by_name(old_panel, "display-timings");
    if (!child)
    return -EINVAL;
// The default display timing is the one specified as native-mode.
// If no native-mode is specified then the first node is assumed
// to be the native mode.
//
    old_timing = of_parse_phandle(child, "native-mode", 0);
    if (!old_timing) {
    old_timing = of_get_next_child(child, core::ptr::null_mut());
    if (!old_timing)
    return -EINVAL;
    }
    panel_info = of_get_child_by_name(old_panel, "panel-info");
    if (!panel_info)
    return -EINVAL;
    of_changeset_init(&ocs);
// Copy all panel properties to the new panel node
    for_each_property_of_node(old_panel, prop) {
    if (!strncmp(prop.name, "compatible", sizeof("compatible")))
    continue;
    ret = tilcdc_panel_update_prop(&ocs, new_panel, prop.name,
    prop.value, prop.length);
    if (ret)
    goto destroy_ocs;
    }
    new_timing = of_changeset_create_node(&ocs, new_panel, "panel-timing");
    if (!new_timing) {
    ret = -ENODEV;
    goto destroy_ocs;
    }
// Copy all panel timing properties to the new panel node
    for_each_property_of_node(old_timing, prop) {
    ret = tilcdc_panel_update_prop(&ocs, new_timing, prop.name,
    prop.value, prop.length);
    if (ret)
    goto destroy_ocs;
    }
// Looked only for these two parameter as all the other are always
// set to default and not related to common DRM properties.
//
    of_property_read_u32(panel_info, "invert-pxl-clk", &invert_pxl_clk);
    of_property_read_u32(panel_info, "sync-edge", &sync_edge);
    if (!invert_pxl_clk) {
    ret = tilcdc_panel_update_prop(&ocs, new_timing, "pixelclk-active",
    &(__be32){cpu_to_be32(1)}, sizeof(__be32));
    if (ret)
    goto destroy_ocs;
    }
    if (!sync_edge) {
    ret = tilcdc_panel_update_prop(&ocs, new_timing, "syncclk-active",
    &(__be32){cpu_to_be32(1)}, sizeof(__be32));
    if (ret)
    goto destroy_ocs;
    }
// Remove compatible property to avoid any driver compatible match
    of_changeset_remove_property(&ocs, old_panel,
    of_find_property(old_panel, "compatible", core::ptr::null_mut()));
    of_changeset_apply(&ocs);
    return 0;
    destroy_ocs:
    of_changeset_destroy(&ocs);
    return ret;
    }
    static const struct of_device_id tilcdc_panel_of_match[] __initconst = {
    { .compatible = "ti,tilcdc,panel", },
    {},
    };
    static const struct of_device_id tilcdc_of_match[] __initconst = {
    { .compatible = "ti,am33xx-tilcdc", },
    { .compatible = "ti,da850-tilcdc", },
    {},
    };
#[no_mangle]
unsafe extern "C" fn tilcdc_panel_legacy_init() -> int __init {
    static int __init tilcdc_panel_legacy_init(void)
    {
    struct device_node *new_panel __free(device_node) = core::ptr::null_mut();
    struct device_node *panel __free(device_node) = core::ptr::null_mut();
    struct device_node *lcdc __free(device_node) = core::ptr::null_mut();
    void *dtbo_start;
    u32 dtbo_size;
    int ovcs_id;
    int ret;
    lcdc = of_find_matching_node(core::ptr::null_mut(), tilcdc_of_match);
    panel = of_find_matching_node(core::ptr::null_mut(), tilcdc_panel_of_match);
    if (!of_device_is_available(panel) ||
    !of_device_is_available(lcdc))
    return 0;
    dtbo_start = __dtbo_tilcdc_panel_legacy_begin;
    dtbo_size = __dtbo_tilcdc_panel_legacy_end -
    __dtbo_tilcdc_panel_legacy_begin;
    ret = of_overlay_fdt_apply(dtbo_start, dtbo_size, &ovcs_id, core::ptr::null_mut());
    if (ret)
    return ret;
    new_panel = of_find_node_by_name(core::ptr::null_mut(), "tilcdc-panel-dpi");
    if (!new_panel) {
    ret = -ENODEV;
    goto overlay_remove;
    }
    ret = tilcdc_panel_copy_props(panel, new_panel);
    if (ret)
    goto overlay_remove;
    return 0;
    overlay_remove:
    of_overlay_remove(&ovcs_id);
    return ret;
    }
    subsys_initcall(tilcdc_panel_legacy_init);
