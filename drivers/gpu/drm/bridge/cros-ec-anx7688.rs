//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/bridge/cros-ec-anx7688.c
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
// CrOS EC ANX7688 HDMI->DP bridge driver
//
// Copyright 2020 Google LLC
//

// Register addresses
pub const ANX7688_VENDOR_ID_REG: c_uint = 0x00;
pub const ANX7688_DEVICE_ID_REG: c_uint = 0x02;
pub const ANX7688_FW_VERSION_REG: c_uint = 0x80;
pub const ANX7688_DP_BANDWIDTH_REG: c_uint = 0x85;
pub const ANX7688_DP_LANE_COUNT_REG: c_uint = 0x86;
pub const ANX7688_VENDOR_ID: c_uint = 0x1f29;
pub const ANX7688_DEVICE_ID: c_uint = 0x7688;
// First supported firmware version (0.85)
pub const ANX7688_MINIMUM_FW_VERSION: c_uint = 0x0085;
    static const struct regmap_config cros_ec_anx7688_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cros_ec_anx7688 {
    pub client: *mut i2c_client,
    pub regmap: *mut regmap,
    pub bridge: drm_bridge,
    pub filter: bool,
}

    static inline struct cros_ec_anx7688 *
    bridge_to_cros_ec_anx7688(struct drm_bridge *bridge)
    {
    return container_of(bridge, struct cros_ec_anx7688, bridge);
    }
    static bool cros_ec_anx7688_bridge_mode_fixup(struct drm_bridge *bridge,
    const struct drm_display_mode *mode,
    struct drm_display_mode *adjusted_mode)
    {
    struct cros_ec_anx7688 *anx = bridge_to_cros_ec_anx7688(bridge);
    int totalbw, requiredbw;
    u8 dpbw, lanecount;
    u8 regs[2];
    int ret;
    if (!anx.filter)
    return true;
// Read both regs 0x85 (bandwidth) and 0x86 (lane count).
    ret = regmap_bulk_read(anx.regmap, ANX7688_DP_BANDWIDTH_REG, regs, 2);
    if (ret < 0) {
    DRM_ERROR("Failed to read bandwidth/lane count\n");
    return false;
    }
    dpbw = regs[0];
    lanecount = regs[1];
// Maximum 0x19 bandwidth (6.75 Gbps Turbo mode), 2 lanes
    if (dpbw > 0x19 || lanecount > 2) {
    DRM_ERROR("Invalid bandwidth/lane count (%02x/%d)\n", dpbw,
    lanecount);
    return false;
    }
// Compute available bandwidth (kHz)
    totalbw = dpbw * lanecount * 270000 * 8 / 10;
// Required bandwidth (8 bpc, kHz)
    requiredbw = mode.clock * 8 * 3;
    DRM_DEBUG_KMS("DP bandwidth: %d kHz (%02x/%d); mode requires %d Khz\n",
    totalbw, dpbw, lanecount, requiredbw);
    if (totalbw == 0) {
    DRM_ERROR("Bandwidth/lane count are 0, not rejecting modes\n");
    return true;
    }
    return totalbw >= requiredbw;
    }
    static const struct drm_bridge_funcs cros_ec_anx7688_bridge_funcs = {
    .atomic_create_state = drm_atomic_helper_bridge_create_state,
    .atomic_destroy_state = drm_atomic_helper_bridge_destroy_state,
    .atomic_duplicate_state = drm_atomic_helper_bridge_duplicate_state,
    .mode_fixup = cros_ec_anx7688_bridge_mode_fixup,
    };
#[no_mangle]
unsafe extern "C" fn cros_ec_anx7688_bridge_probe(client: *mut i2c_client) -> c_int {
    static int cros_ec_anx7688_bridge_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct cros_ec_anx7688 *anx7688;
    u16 vendor, device, fw_version;
    u8 buffer[4];
    int ret;
    anx7688 = devm_drm_bridge_alloc(dev, struct cros_ec_anx7688, bridge,
    &cros_ec_anx7688_bridge_funcs);
    if (IS_ERR(anx7688))
    return PTR_ERR(anx7688);
    anx7688.client = client;
    i2c_set_clientdata(client, anx7688);
    anx7688.regmap = devm_regmap_init_i2c(client, &cros_ec_anx7688_regmap_config);
    if (IS_ERR(anx7688.regmap)) {
    ret = PTR_ERR(anx7688.regmap);
    dev_err(dev, "regmap i2c init failed: %d\n", ret);
    return ret;
    }
// Read both vendor and device id (4 bytes).
    ret = regmap_bulk_read(anx7688.regmap, ANX7688_VENDOR_ID_REG,
    buffer, 4);
    if (ret) {
    dev_err(dev, "Failed to read chip vendor/device id\n");
    return ret;
    }
    vendor = (u16)buffer[1] << 8 | buffer[0];
    device = (u16)buffer[3] << 8 | buffer[2];
    if (vendor != ANX7688_VENDOR_ID || device != ANX7688_DEVICE_ID) {
    dev_err(dev, "Invalid vendor/device id %04x/%04x\n",
    vendor, device);
    return -ENODEV;
    }
    ret = regmap_bulk_read(anx7688.regmap, ANX7688_FW_VERSION_REG,
    buffer, 2);
    if (ret) {
    dev_err(dev, "Failed to read firmware version\n");
    return ret;
    }
    fw_version = (u16)buffer[0] << 8 | buffer[1];
    dev_info(dev, "ANX7688 firmware version 0x%04x\n", fw_version);
    anx7688.bridge.of_node = dev.of_node;
// FW version >= 0.85 supports bandwidth/lane count registers
    if (fw_version >= ANX7688_MINIMUM_FW_VERSION)
    anx7688.filter = true;
    else
// Warn, but not fail, for backwards compatibility
    DRM_WARN("Old ANX7688 FW version (0x%04x), not filtering\n",
    fw_version);
    drm_bridge_add(&anx7688.bridge);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cros_ec_anx7688_bridge_remove(client: *mut i2c_client) {
    static void cros_ec_anx7688_bridge_remove(struct i2c_client *client)
    {
    struct cros_ec_anx7688 *anx7688 = i2c_get_clientdata(client);
    drm_bridge_remove(&anx7688.bridge);
    }
    static const struct of_device_id cros_ec_anx7688_bridge_match_table[] = {
    { .compatible = "google,cros-ec-anx7688" },
    { }
    };
    MODULE_DEVICE_TABLE(of, cros_ec_anx7688_bridge_match_table);
    static struct i2c_driver cros_ec_anx7688_bridge_driver = {
    .probe = cros_ec_anx7688_bridge_probe,
    .remove = cros_ec_anx7688_bridge_remove,
    .driver = {
    .name = "cros-ec-anx7688-bridge",
    .of_match_table = cros_ec_anx7688_bridge_match_table,
    },
    };
    module_i2c_driver(cros_ec_anx7688_bridge_driver);
    MODULE_DESCRIPTION("ChromeOS EC ANX7688 HDMI.DP bridge driver");
    MODULE_AUTHOR("Nicolas Boichat <drinkcat@chromium.org>");
    MODULE_AUTHOR("Enric Balletbo i Serra <enric.balletbo@collabora.com>");
    MODULE_LICENSE("GPL");
