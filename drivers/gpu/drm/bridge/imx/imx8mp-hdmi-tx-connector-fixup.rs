//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/bridge/imx/imx8mp-hdmi-tx-connector-fixup.c
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
// Add an hdmi-connector node to boards using the imx8mp hdmi_tx which
// don't have one. This is needed for the i.MX LCDIF to work with
// DRM_BRIDGE_ATTACH_NO_CONNECTOR.
//
// Copyright (C) 2026 GE HealthCare
// Author: Luca Ceresoli <luca.ceresoli@bootlin.com>
//

// Embedded dtbo symbols created by cmd_wrap_S_dtb in scripts/Makefile.dtbs
    extern char __dtbo_imx8mp_hdmi_tx_connector_fixup_begin[];
    extern char __dtbo_imx8mp_hdmi_tx_connector_fixup_end[];
#[no_mangle]
unsafe extern "C" fn imx8mp_hdmi_tx_connector_fixup_init() -> int __init {
    static int __init imx8mp_hdmi_tx_connector_fixup_init(void)
    {
    struct device_node *soc      __free(device_node) = core::ptr::null_mut();
    struct device_node *hdmi_tx  __free(device_node) = core::ptr::null_mut();
    struct device_node *endpoint __free(device_node) = core::ptr::null_mut();
    void *dtbo_start;
    u32 dtbo_size;
    int ovcs_id;
    int err;
    soc = of_find_node_by_path("/soc@0");
    if (!soc)
    return 0;
// This applies to i.MX8MP only, do nothing on other systems
    if (!of_device_is_compatible(soc, "fsl,imx8mp-soc"))
    return 0;
    hdmi_tx = of_find_node_by_path("/soc@0/bus@32c00000/hdmi@32fd8000");
    if (!of_device_is_available(hdmi_tx))
    return 0;
// If endpoint exists, assume an hdmi-connector exists already
    endpoint = of_graph_get_endpoint_by_regs(hdmi_tx, 1, -1);
    if (endpoint)
    return 0;
//
// Boards with an HDMI connector should describe it in a device
// tree node with compatible = "hdmi-connector".
//
// If you see this warning, it means such a node was not found and
// a fallback one is added using a device tree overlay. Please add
// one in your device tree, also describing the exact connector
// type (the added overlay assumes Type A as a fallback, but it
// might be wrong).
//
// This node is necessary for modern DRM, where bridge drivers do
// not create a connector (see the DRM_BRIDGE_ATTACH_NO_CONNECTOR
// flag). See https://docs.kernel.org/gpu/drm-kms-helpers.html for
// more info.
//
    pr_warn("Please add a hdmi-connector DT node for imx8mp-hdmi-tx.\n");
    dtbo_start = __dtbo_imx8mp_hdmi_tx_connector_fixup_begin;
    dtbo_size = __dtbo_imx8mp_hdmi_tx_connector_fixup_end -
    __dtbo_imx8mp_hdmi_tx_connector_fixup_begin;
    err = of_overlay_fdt_apply(dtbo_start, dtbo_size, &ovcs_id, core::ptr::null_mut());
    if (err)
    err = of_overlay_remove(&ovcs_id);
    return err;
    }
    subsys_initcall(imx8mp_hdmi_tx_connector_fixup_init);
