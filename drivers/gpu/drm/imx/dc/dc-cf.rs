//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/imx/dc/dc-cf.c
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

pub const STATICCONTROL: c_uint = 0x8;
pub const FRAMEDIMENSIONS: c_uint = 0xc;

pub const CONSTANTCOLOR: c_uint = 0x10;

    static const struct dc_subdev_info dc_cf_info[] = {
    { .reg_start = 0x56180960, .id = 0, },
    { .reg_start = 0x561809e0, .id = 1, },
    { .reg_start = 0x561809a0, .id = 4, },
    { .reg_start = 0x56180a20, .id = 5, },
    };
    static const struct regmap_range dc_cf_regmap_ranges[] = {
    regmap_reg_range(STATICCONTROL, CONSTANTCOLOR),
    };
    static const struct regmap_access_table dc_cf_regmap_access_table = {
    .yes_ranges = dc_cf_regmap_ranges,
    .n_yes_ranges = ARRAY_SIZE(dc_cf_regmap_ranges),
    };
    static const struct regmap_config dc_cf_cfg_regmap_config = {
    .name = "cfg",
    .reg_bits = 32,
    .reg_stride = 4,
    .val_bits = 32,
    .fast_io = true,
    .wr_table = &dc_cf_regmap_access_table,
    .rd_table = &dc_cf_regmap_access_table,
    .max_register = CONSTANTCOLOR,
    };
#[no_mangle]
pub unsafe extern "C" fn dc_cf_enable_shden(cf: *mut dc_cf) {
    static inline void dc_cf_enable_shden(struct dc_cf *cf)
    {
    regmap_write(cf.reg_cfg, STATICCONTROL, SHDEN);
    }
#[no_mangle]
pub unsafe extern "C" fn dc_cf_get_link_id(cf: *mut dc_cf) -> enum dc_link_id {
    enum dc_link_id dc_cf_get_link_id(struct dc_cf *cf)
    {
    return cf.link;
    }
    void dc_cf_framedimensions(struct dc_cf *cf, unsigned int w,
    unsigned int h)
    {
    regmap_write(cf.reg_cfg, FRAMEDIMENSIONS, WIDTH(w) | HEIGHT(h));
    }
#[no_mangle]
pub unsafe extern "C" fn dc_cf_constantcolor_black(cf: *mut dc_cf) {
    void dc_cf_constantcolor_black(struct dc_cf *cf)
    {
    regmap_write(cf.reg_cfg, CONSTANTCOLOR, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn dc_cf_constantcolor_blue(cf: *mut dc_cf) {
    void dc_cf_constantcolor_blue(struct dc_cf *cf)
    {
    regmap_write(cf.reg_cfg, CONSTANTCOLOR, BLUE(0xff));
    }
#[no_mangle]
pub unsafe extern "C" fn dc_cf_init(cf: *mut dc_cf) {
    void dc_cf_init(struct dc_cf *cf)
    {
    dc_cf_enable_shden(cf);
    }
#[no_mangle]
unsafe extern "C" fn dc_cf_bind(dev: *mut device, master: *mut device, data: *mut c_void) -> c_int {
    static int dc_cf_bind(struct device *dev, struct device *master, void *data)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct dc_drm_device *dc_drm = data;
    struct resource *res_pec;
    void __iomem *base_cfg;
    struct dc_cf *cf;
    int id;
    cf = devm_kzalloc(dev, sizeof(*cf), GFP_KERNEL);
    if (!cf)
    return -ENOMEM;
    res_pec = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    base_cfg = devm_platform_ioremap_resource_byname(pdev, "cfg");
    if (IS_ERR(base_cfg))
    return PTR_ERR(base_cfg);
    cf.reg_cfg = devm_regmap_init_mmio(dev, base_cfg,
    &dc_cf_cfg_regmap_config);
    if (IS_ERR(cf.reg_cfg))
    return PTR_ERR(cf.reg_cfg);
    id = dc_subdev_get_id(dc_cf_info, ARRAY_SIZE(dc_cf_info), res_pec);
    if (id < 0) {
    dev_err(dev, "failed to get instance number: %d\n", id);
    return id;
    }
    switch (id) {
    case 0:
    cf.link = LINK_ID_CONSTFRAME0;
    dc_drm.cf_cont[0] = cf;
    break;
    case 1:
    cf.link = LINK_ID_CONSTFRAME1;
    dc_drm.cf_cont[1] = cf;
    break;
    case 4:
    cf.link = LINK_ID_CONSTFRAME4;
    dc_drm.cf_safe[0] = cf;
    break;
    case 5:
    cf.link = LINK_ID_CONSTFRAME5;
    dc_drm.cf_safe[1] = cf;
    break;
    }
    return 0;
    }
    static const struct component_ops dc_cf_ops = {
    .bind = dc_cf_bind,
    };
#[no_mangle]
unsafe extern "C" fn dc_cf_probe(pdev: *mut platform_device) -> c_int {
    static int dc_cf_probe(struct platform_device *pdev)
    {
    int ret;
    ret = component_add(&pdev.dev, &dc_cf_ops);
    if (ret)
    return dev_err_probe(&pdev.dev, ret,
    "failed to add component\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dc_cf_remove(pdev: *mut platform_device) {
    static void dc_cf_remove(struct platform_device *pdev)
    {
    component_del(&pdev.dev, &dc_cf_ops);
    }
    static const struct of_device_id dc_cf_dt_ids[] = {
    { .compatible = "fsl,imx8qxp-dc-constframe" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, dc_cf_dt_ids);
    struct platform_driver dc_cf_driver = {
    .probe = dc_cf_probe,
    .remove = dc_cf_remove,
    .driver = {
    .name = "imx8-dc-constframe",
    .suppress_bind_attrs = true,
    .of_match_table = dc_cf_dt_ids,
    },
    };
