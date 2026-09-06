//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/imx/dc/dc-tc.c
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
// Copyright 2024 NXP
//

pub const TCON_CTRL: c_uint = 0x410;
pub const CTRL_RST_VAL: c_uint = 0x01401408;
// red: MAPBIT 29-20, green: MAPBIT 19-10, blue: MAPBIT 9-0
pub const MAPBIT3_0: c_uint = 0x418;
pub const MAPBIT7_4: c_uint = 0x41c;
pub const MAPBIT11_8: c_uint = 0x420;
pub const MAPBIT15_12: c_uint = 0x424;
pub const MAPBIT19_16: c_uint = 0x428;
pub const MAPBIT23_20: c_uint = 0x42c;
pub const MAPBIT27_24: c_uint = 0x430;
pub const MAPBIT31_28: c_uint = 0x434;
    static const struct dc_subdev_info dc_tc_info[] = {
    { .reg_start = 0x5618c800, .id = 0, },
    { .reg_start = 0x5618e400, .id = 1, },
    };
    static const struct regmap_range dc_tc_regmap_ranges[] = {
    regmap_reg_range(TCON_CTRL, TCON_CTRL),
    regmap_reg_range(MAPBIT3_0, MAPBIT31_28),
    };
    static const struct regmap_access_table dc_tc_regmap_access_table = {
    .yes_ranges = dc_tc_regmap_ranges,
    .n_yes_ranges = ARRAY_SIZE(dc_tc_regmap_ranges),
    };
    static const struct regmap_config dc_tc_regmap_config = {
    .reg_bits = 32,
    .reg_stride = 4,
    .val_bits = 32,
    .fast_io = true,
    .wr_table = &dc_tc_regmap_access_table,
    .rd_table = &dc_tc_regmap_access_table,
    .max_register = MAPBIT31_28,
    };
//
// The pixels reach TCON are always in 30-bit BGR format.
// The first bridge always receives pixels in 30-bit RGB format.
// So, map the format to MEDIA_BUS_FMT_RGB101010_1X30.
//
    static const u32 dc_tc_mapbit[] = {
    0x17161514, 0x1b1a1918, 0x0b0a1d1c, 0x0f0e0d0c,
    0x13121110, 0x03020100, 0x07060504, 0x00000908,
    };
#[no_mangle]
pub unsafe extern "C" fn dc_tc_init(tc: *mut dc_tc) {
    void dc_tc_init(struct dc_tc *tc)
    {
// reset TCON_CTRL to POR default so that TCON works in bypass mode
    regmap_write(tc.reg, TCON_CTRL, CTRL_RST_VAL);
// set format
    regmap_bulk_write(tc.reg, MAPBIT3_0, dc_tc_mapbit,
    ARRAY_SIZE(dc_tc_mapbit));
    }
#[no_mangle]
unsafe extern "C" fn dc_tc_bind(dev: *mut device, master: *mut device, data: *mut c_void) -> c_int {
    static int dc_tc_bind(struct device *dev, struct device *master, void *data)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct dc_drm_device *dc_drm = data;
    struct resource *res;
    void __iomem *base;
    struct dc_tc *tc;
    int id;
    tc = devm_kzalloc(dev, sizeof(*tc), GFP_KERNEL);
    if (!tc)
    return -ENOMEM;
    base = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(base))
    return PTR_ERR(base);
    tc.reg = devm_regmap_init_mmio(dev, base, &dc_tc_regmap_config);
    if (IS_ERR(tc.reg))
    return PTR_ERR(tc.reg);
    id = dc_subdev_get_id(dc_tc_info, ARRAY_SIZE(dc_tc_info), res);
    if (id < 0) {
    dev_err(dev, "failed to get instance number: %d\n", id);
    return id;
    }
    tc.dev = dev;
    dc_drm.tc[id] = tc;
    return 0;
    }
    static const struct component_ops dc_tc_ops = {
    .bind = dc_tc_bind,
    };
#[no_mangle]
unsafe extern "C" fn dc_tc_probe(pdev: *mut platform_device) -> c_int {
    static int dc_tc_probe(struct platform_device *pdev)
    {
    int ret;
    ret = component_add(&pdev.dev, &dc_tc_ops);
    if (ret)
    return dev_err_probe(&pdev.dev, ret,
    "failed to add component\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dc_tc_remove(pdev: *mut platform_device) {
    static void dc_tc_remove(struct platform_device *pdev)
    {
    component_del(&pdev.dev, &dc_tc_ops);
    }
    static const struct of_device_id dc_tc_dt_ids[] = {
    { .compatible = "fsl,imx8qxp-dc-tcon" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, dc_tc_dt_ids);
    struct platform_driver dc_tc_driver = {
    .probe = dc_tc_probe,
    .remove = dc_tc_remove,
    .driver = {
    .name = "imx8-dc-tcon",
    .suppress_bind_attrs = true,
    .of_match_table = dc_tc_dt_ids,
    },
    };
