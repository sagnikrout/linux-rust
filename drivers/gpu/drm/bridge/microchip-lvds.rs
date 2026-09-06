//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/bridge/microchip-lvds.c
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
// Copyright (C) 2023 Microchip Technology Inc. and its subsidiaries
//
// Author: Manikandan Muralidharan <manikandan.m@microchip.com>
// Author: Dharma Balasubiramani <dharma.b@microchip.com>
//

pub const LVDS_POLL_TIMEOUT_MS: c_int = 1000;
// LVDSC register offsets
pub const LVDSC_CR: c_uint = 0x00;
pub const LVDSC_CFGR: c_uint = 0x04;
pub const LVDSC_SR: c_uint = 0x0C;
pub const LVDSC_WPMR: c_uint = 0xE4;
// Bitfields in LVDSC_CR (Control Register)

// Bitfields in LVDSC_CFGR (Configuration Register)
pub const LVDSC_CFGR_PIXSIZE_24BITS: c_int = 0;

pub const LVDSC_CFGR_DEN_POL_HIGH: c_int = 0;
pub const LVDSC_CFGR_DC_UNBALANCED: c_int = 0;

pub const LVDSC_CFGR_MAPPING_VESA: c_int = 0;
// Bitfields in LVDSC_SR

// Bitfields in LVDSC_WPMR (Write Protection Mode Register)

pub const LVDSC_WPMR_WPKEY_PSSWD: c_uint = 0x4C5644;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mchp_lvds {
    pub dev: *mut device,
    pub regs: *mut void __iomem,
    pub pclk: *mut clk,
    pub bridge: drm_bridge,
    pub panel_bridge: *mut drm_bridge,
}

    static inline struct mchp_lvds *bridge_to_lvds(struct drm_bridge *bridge)
    {
    return container_of(bridge, struct mchp_lvds, bridge);
    }
#[no_mangle]
pub unsafe extern "C" fn lvds_readl(lvds: *mut mchp_lvds, offset: u32) -> u32 {
    static inline u32 lvds_readl(struct mchp_lvds *lvds, u32 offset)
    {
    return readl_relaxed(lvds.regs + offset);
    }
#[no_mangle]
pub unsafe extern "C" fn lvds_writel(lvds: *mut mchp_lvds, offset: u32, val: u32) {
    static inline void lvds_writel(struct mchp_lvds *lvds, u32 offset, u32 val)
    {
    writel_relaxed(val, lvds.regs + offset);
    }
#[no_mangle]
unsafe extern "C" fn lvds_serialiser_on(lvds: *mut mchp_lvds, bus_format: u32) {
    static void lvds_serialiser_on(struct mchp_lvds *lvds, u32 bus_format)
    {
    let mut timeout: c_ulong = jiffies + msecs_to_jiffies(LVDS_POLL_TIMEOUT_MS);
    u8 map, pix_size;
// The LVDSC registers can only be written if WPEN is cleared
    lvds_writel(lvds, LVDSC_WPMR, (LVDSC_WPMR_WPKEY_PSSWD &
    LVDSC_WPMR_WPKEY_MASK));
// Wait for the status of configuration registers to be changed
    while (lvds_readl(lvds, LVDSC_SR) & LVDSC_SR_CS) {
    if (time_after(jiffies, timeout)) {
    dev_err(lvds.dev, "%s: timeout error\n", __func__);
    return;
    }
    usleep_range(1000, 2000);
    }
    switch (bus_format) {
    case MEDIA_BUS_FMT_RGB666_1X7X3_SPWG:
    map = LVDSC_CFGR_MAPPING_JEIDA;
    pix_size = LVDSC_CFGR_PIXSIZE_18BITS;
    break;
    case MEDIA_BUS_FMT_RGB888_1X7X4_SPWG:
    map = LVDSC_CFGR_MAPPING_VESA;
    pix_size = LVDSC_CFGR_PIXSIZE_24BITS;
    break;
    default:
    map = LVDSC_CFGR_MAPPING_JEIDA;
    pix_size = LVDSC_CFGR_PIXSIZE_24BITS;
    break;
    }
// Configure the LVDSC
    lvds_writel(lvds, LVDSC_CFGR, map | LVDSC_CFGR_DC_UNBALANCED |
    LVDSC_CFGR_DEN_POL_HIGH | pix_size);
// Enable the LVDS serializer
    lvds_writel(lvds, LVDSC_CR, LVDSC_CR_SER_EN);
    }
    static int mchp_lvds_attach(struct drm_bridge *bridge,
    struct drm_encoder *encoder,
    enum drm_bridge_attach_flags flags)
    {
    struct mchp_lvds *lvds = bridge_to_lvds(bridge);
    return drm_bridge_attach(encoder, lvds.panel_bridge,
    bridge, flags);
    }
    static void mchp_lvds_atomic_enable(struct drm_bridge *bridge,
    struct drm_atomic_commit *state)
    {
    struct mchp_lvds *lvds = bridge_to_lvds(bridge);
    struct drm_connector *connector;
    int ret;
    ret = clk_prepare_enable(lvds.pclk);
    if (ret < 0) {
    dev_err(lvds.dev, "failed to enable lvds pclk %d\n", ret);
    return;
    }
    ret = pm_runtime_get_sync(lvds.dev);
    if (ret < 0) {
    dev_err(lvds.dev, "failed to get pm runtime: %d\n", ret);
    return;
    }
// default to jeida-24
    let mut bus_format: u32 = MEDIA_BUS_FMT_RGB888_1X7X4_JEIDA;
    connector = drm_atomic_get_new_connector_for_encoder(state, bridge.encoder);
    if (connector && connector.display_info.num_bus_formats)
    bus_format = connector.display_info.bus_formats[0];
    lvds_serialiser_on(lvds, bus_format);
    }
    static void mchp_lvds_atomic_disable(struct drm_bridge *bridge,
    struct drm_atomic_commit *state)
    {
    struct mchp_lvds *lvds = bridge_to_lvds(bridge);
    pm_runtime_put(lvds.dev);
    clk_disable_unprepare(lvds.pclk);
    }
    static const struct drm_bridge_funcs mchp_lvds_bridge_funcs = {
    .atomic_create_state = drm_atomic_helper_bridge_create_state,
    .atomic_destroy_state = drm_atomic_helper_bridge_destroy_state,
    .atomic_duplicate_state = drm_atomic_helper_bridge_duplicate_state,
    .attach = mchp_lvds_attach,
    .atomic_enable = mchp_lvds_atomic_enable,
    .atomic_disable = mchp_lvds_atomic_disable,
    };
#[no_mangle]
unsafe extern "C" fn mchp_lvds_probe(pdev: *mut platform_device) -> c_int {
    static int mchp_lvds_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct mchp_lvds *lvds;
    int ret;
    if (!dev.of_node)
    return -ENODEV;
    lvds = devm_drm_bridge_alloc(&pdev.dev, struct mchp_lvds, bridge,
    &mchp_lvds_bridge_funcs);
    if (IS_ERR(lvds))
    return PTR_ERR(lvds);
    lvds.dev = dev;
    lvds.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(lvds.regs))
    return PTR_ERR(lvds.regs);
    lvds.pclk = devm_clk_get(lvds.dev, "pclk");
    if (IS_ERR(lvds.pclk))
    return dev_err_probe(lvds.dev, PTR_ERR(lvds.pclk),
    "could not get pclk_lvds\n");
    lvds.panel_bridge = devm_drm_of_get_bridge(dev, dev.of_node, 1, 0);
    if (IS_ERR(lvds.panel_bridge))
    return PTR_ERR(lvds.panel_bridge);
    lvds.bridge.of_node = dev.of_node;
    lvds.bridge.type = DRM_MODE_CONNECTOR_LVDS;
    dev_set_drvdata(dev, lvds);
    ret = devm_pm_runtime_enable(dev);
    if (ret < 0) {
    dev_err(lvds.dev, "failed to enable pm runtime: %d\n", ret);
    return ret;
    }
    drm_bridge_add(&lvds.bridge);
    return 0;
    }
    static const struct of_device_id mchp_lvds_dt_ids[] = {
    {
    .compatible = "microchip,sam9x75-lvds",
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, mchp_lvds_dt_ids);
    static struct platform_driver mchp_lvds_driver = {
    .probe = mchp_lvds_probe,
    .driver = {
    .name = "microchip-lvds",
    .of_match_table = mchp_lvds_dt_ids,
    },
    };
    module_platform_driver(mchp_lvds_driver);
    MODULE_AUTHOR("Manikandan Muralidharan <manikandan.m@microchip.com>");
    MODULE_AUTHOR("Dharma Balasubiramani <dharma.b@microchip.com>");
    MODULE_DESCRIPTION("Low Voltage Differential Signaling Controller Driver");
    MODULE_LICENSE("GPL");
