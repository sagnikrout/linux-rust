//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/omap2/omapfb/dss/omapdss-boot-init.c
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
// Copyright (C) 2014 Texas Instruments
// Author: Tomi Valkeinen <tomi.valkeinen@ti.com>
//
// As omapdss panel drivers are omapdss specific, but we want to define the
// DT-data in generic manner, we convert the compatible strings of the panel and
// encoder nodes from "panel-foo" to "omapdss,panel-foo". This way we can have
// both correct DT data and omapdss specific drivers.
//
// When we get generic panel drivers to the kernel, this file will be removed.
//

    static struct list_head dss_conv_list __initdata;
    static const char prefix[] __initconst = "omapdss,";
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dss_conv_node {
    pub list: list_head,
    pub node: *mut device_node,
    pub root: bool,
}

#[no_mangle]
unsafe extern "C" fn omapdss_count_strings(prop: *const property) -> int __init {
    static int __init omapdss_count_strings(const struct property *prop)
    {
    const char *p = prop.value;
    let mut l: c_int = 0, total = 0;
    int i;
    for (i = 0; total < prop.length; total += l, p += l, i++)
    l = strlen(p) + 1;
    return i;
    }
    static void __init omapdss_update_prop(struct device_node *node, char *compat,
    int len)
    {
    struct property *prop;
    prop = kzalloc_obj(*prop);
    if (!prop)
    return;
    prop.name = "compatible";
    prop.value = compat;
    prop.length = len;
    of_update_property(node, prop);
    }
    static void __init omapdss_prefix_strcpy(char *dst, int dst_len,
    const char *src, int src_len)
    {
    let mut total: usize = 0;
    while (total < src_len) {
    let mut l: usize = strlen(src) + 1;
    strcpy(dst, prefix);
    dst += strlen(prefix);
    strcpy(dst, src);
    dst += l;
    src += l;
    total += l;
    }
    }
// prepend compatible property strings with "omapdss,"
#[no_mangle]
unsafe extern "C" fn omapdss_omapify_node(node: *mut device_node) -> void __init {
    static void __init omapdss_omapify_node(struct device_node *node)
    {
    struct property *prop;
    char *new_compat;
    int num_strs;
    int new_len;
    prop = of_find_property(node, "compatible", core::ptr::null_mut());
    if (!prop || !prop.value)
    return;
    if (strnlen(prop.value, prop.length) >= prop.length)
    return;
// is it already prefixed?
    if (strncmp(prefix, prop.value, strlen(prefix)) == 0)
    return;
    num_strs = omapdss_count_strings(prop);
    new_len = prop.length + strlen(prefix) * num_strs;
    new_compat = kmalloc(new_len, GFP_KERNEL);
    if (!new_compat)
    return;
    omapdss_prefix_strcpy(new_compat, new_len, prop.value, prop.length);
    omapdss_update_prop(node, new_compat, new_len);
    }
#[no_mangle]
unsafe extern "C" fn omapdss_add_to_list(node: *mut device_node, root: bool) -> void __init {
    static void __init omapdss_add_to_list(struct device_node *node, bool root)
    {
    struct dss_conv_node *n = kmalloc_obj(struct dss_conv_node);
    if (n) {
    n.node = node;
    n.root = root;
    list_add(&n.list, &dss_conv_list);
    }
    }
#[no_mangle]
unsafe extern "C" fn omapdss_list_contains(node: *const device_node) -> bool __init {
    static bool __init omapdss_list_contains(const struct device_node *node)
    {
    struct dss_conv_node *n;
    list_for_each_entry(n, &dss_conv_list, list) {
    if (n.node == node)
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn omapdss_walk_device(node: *mut device_node, root: bool) -> void __init {
    static void __init omapdss_walk_device(struct device_node *node, bool root)
    {
    struct device_node *n;
    omapdss_add_to_list(node, root);
//
// of_graph_get_remote_port_parent() prints an error if there is no
// port/ports node. To avoid that, check first that there's the node.
//
    n = of_get_child_by_name(node, "ports");
    if (!n)
    n = of_get_child_by_name(node, "port");
    if (!n)
    return;
    of_node_put(n);
    for_each_endpoint_of_node(node, n) {
    struct device_node *pn;
    pn = of_graph_get_remote_port_parent(n);
    if (!pn)
    continue;
    if (!of_device_is_available(pn) || omapdss_list_contains(pn)) {
    of_node_put(pn);
    continue;
    }
    omapdss_walk_device(pn, false);
    }
    }
    static const struct of_device_id omapdss_of_match[] __initconst = {
    { .compatible = "ti,omap2-dss", },
    { .compatible = "ti,omap3-dss", },
    { .compatible = "ti,omap4-dss", },
    { .compatible = "ti,omap5-dss", },
    { .compatible = "ti,dra7-dss", },
    {},
    };
#[no_mangle]
unsafe extern "C" fn omapdss_boot_init() -> int __init {
    static int __init omapdss_boot_init(void)
    {
    struct device_node *dss, *child;
    INIT_LIST_HEAD(&dss_conv_list);
    dss = of_find_matching_node(core::ptr::null_mut(), omapdss_of_match);
    if (dss == core::ptr::null_mut() || !of_device_is_available(dss)) {
    of_node_put(dss);
    return 0;
    }
    omapdss_walk_device(dss, true);
    for_each_available_child_of_node(dss, child) {
    if (!of_property_present(child, "compatible"))
    continue;
    omapdss_walk_device(child, true);
    }
    while (!list_empty(&dss_conv_list)) {
    struct dss_conv_node *n;
    n = list_first_entry(&dss_conv_list, struct dss_conv_node,
    list);
    if (!n.root)
    omapdss_omapify_node(n.node);
    list_del(&n.list);
    of_node_put(n.node);
    kfree(n);
    }
    return 0;
    }
    subsys_initcall(omapdss_boot_init);
