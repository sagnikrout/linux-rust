//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/imx/dc/dc-fw.c
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

pub const PIXENGCFG_DYNAMIC: c_uint = 0x8;

pub const FRAMEDIMENSIONS: c_uint = 0x150;
pub const CONTROL: c_uint = 0x170;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_fw {
    pub fu: dc_fu,
}

    static const struct dc_subdev_info dc_fw_info[] = {
    { .reg_start = 0x56180a60, .id = 2, },
    };
    static const struct regmap_range dc_fw_pec_regmap_access_ranges[] = {
    regmap_reg_range(PIXENGCFG_DYNAMIC, PIXENGCFG_DYNAMIC),
    };
    static const struct regmap_access_table dc_fw_pec_regmap_access_table = {
    .yes_ranges = dc_fw_pec_regmap_access_ranges,
    .n_yes_ranges = ARRAY_SIZE(dc_fw_pec_regmap_access_ranges),
    };
    static const struct regmap_config dc_fw_pec_regmap_config = {
    .name = "pec",
    .reg_bits = 32,
    .reg_stride = 4,
    .val_bits = 32,
    .fast_io = true,
    .wr_table = &dc_fw_pec_regmap_access_table,
    .rd_table = &dc_fw_pec_regmap_access_table,
    .max_register = PIXENGCFG_DYNAMIC,
    };
    static const struct regmap_range dc_fw_regmap_ranges[] = {
    regmap_reg_range(STATICCONTROL, FRAMEDIMENSIONS),
    regmap_reg_range(CONTROL, CONTROL),
    };
    static const struct regmap_access_table dc_fw_regmap_access_table = {
    .yes_ranges = dc_fw_regmap_ranges,
    .n_yes_ranges = ARRAY_SIZE(dc_fw_regmap_ranges),
    };
    static const struct regmap_config dc_fw_cfg_regmap_config = {
    .name = "cfg",
    .reg_bits = 32,
    .reg_stride = 4,
    .val_bits = 32,
    .fast_io = true,
    .wr_table = &dc_fw_regmap_access_table,
    .rd_table = &dc_fw_regmap_access_table,
    .max_register = CONTROL,
    };
    static void dc_fw_set_fmt(struct dc_fu *fu, enum dc_fu_frac frac,
    const struct drm_format_info *format)
    {
    let mut bits: u32 = 0, shifts = 0;
    dc_fu_set_src_bpp(fu, frac, format.cpp[0] * 8);
    regmap_write_bits(fu.reg_cfg, CONTROL, INPUTSELECT_MASK,
    INPUTSELECT(INPUTSELECT_INACTIVE));
    regmap_write_bits(fu.reg_cfg, CONTROL, RASTERMODE_MASK,
    RASTERMODE(RASTERMODE_NORMAL));
    regmap_write_bits(fu.reg_cfg, LAYERPROPERTY(frac),
    YUVCONVERSIONMODE_MASK,
    YUVCONVERSIONMODE(YUVCONVERSIONMODE_OFF));
    dc_fu_get_pixel_format_bits(fu, format.format, &bits);
    dc_fu_get_pixel_format_shifts(fu, format.format, &shifts);
    regmap_write(fu.reg_cfg, COLORCOMPONENTBITS(frac), bits);
    regmap_write(fu.reg_cfg, COLORCOMPONENTSHIFT(frac), shifts);
    }
#[no_mangle]
unsafe extern "C" fn dc_fw_set_framedimensions(fu: *mut dc_fu, w: c_int, h: c_int) {
    static void dc_fw_set_framedimensions(struct dc_fu *fu, int w, int h)
    {
    regmap_write(fu.reg_cfg, FRAMEDIMENSIONS,
    FRAMEWIDTH(w) | FRAMEHEIGHT(h));
    }
#[no_mangle]
unsafe extern "C" fn dc_fw_init(fu: *mut dc_fu) {
    static void dc_fw_init(struct dc_fu *fu)
    {
    regmap_write(fu.reg_pec, PIXENGCFG_DYNAMIC, LINK_ID_NONE);
    dc_fu_common_hw_init(fu);
    dc_fu_shdldreq_sticky(fu, 0xff);
    }
#[no_mangle]
unsafe extern "C" fn dc_fw_set_ops(fu: *mut dc_fu) {
    static void dc_fw_set_ops(struct dc_fu *fu)
    {
    memcpy(&fu.ops, &dc_fu_common_ops, sizeof(dc_fu_common_ops));
    fu.ops.init = dc_fw_init;
    fu.ops.set_fmt	= dc_fw_set_fmt;
    fu.ops.set_framedimensions = dc_fw_set_framedimensions;
    }
#[no_mangle]
unsafe extern "C" fn dc_fw_bind(dev: *mut device, master: *mut device, data: *mut c_void) -> c_int {
    static int dc_fw_bind(struct device *dev, struct device *master, void *data)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct dc_drm_device *dc_drm = data;
    struct resource *res_pec;
    void __iomem *base_pec;
    void __iomem *base_cfg;
    struct dc_fw *fw;
    struct dc_fu *fu;
    int i, id;
    fw = devm_kzalloc(dev, sizeof(*fw), GFP_KERNEL);
    if (!fw)
    return -ENOMEM;
    fu = &fw.fu;
    base_pec = devm_platform_get_and_ioremap_resource(pdev, 0, &res_pec);
    if (IS_ERR(base_pec))
    return PTR_ERR(base_pec);
    base_cfg = devm_platform_ioremap_resource_byname(pdev, "cfg");
    if (IS_ERR(base_cfg))
    return PTR_ERR(base_cfg);
    fu.reg_pec = devm_regmap_init_mmio(dev, base_pec,
    &dc_fw_pec_regmap_config);
    if (IS_ERR(fu.reg_pec))
    return PTR_ERR(fu.reg_pec);
    fu.reg_cfg = devm_regmap_init_mmio(dev, base_cfg,
    &dc_fw_cfg_regmap_config);
    if (IS_ERR(fu.reg_cfg))
    return PTR_ERR(fu.reg_cfg);
    id = dc_subdev_get_id(dc_fw_info, ARRAY_SIZE(dc_fw_info), res_pec);
    if (id < 0) {
    dev_err(dev, "failed to get instance number: %d\n", id);
    return id;
    }
    fu.link_id = LINK_ID_FETCHWARP2;
    fu.id = DC_FETCHUNIT_FW2;
    for (i = 0; i < DC_FETCHUNIT_FRAC_NUM; i++) {
    fu.reg_baseaddr[i]		  = BASEADDRESS(i);
    fu.reg_sourcebufferattributes[i] = SOURCEBUFFERATTRIBUTES(i);
    fu.reg_sourcebufferdimension[i]  = SOURCEBUFFERDIMENSION(i);
    fu.reg_layeroffset[i]		  = LAYEROFFSET(i);
    fu.reg_clipwindowoffset[i]	  = CLIPWINDOWOFFSET(i);
    fu.reg_clipwindowdimensions[i]	  = CLIPWINDOWDIMENSIONS(i);
    fu.reg_constantcolor[i]	  = CONSTANTCOLOR(i);
    fu.reg_layerproperty[i]	  = LAYERPROPERTY(i);
    }
    snprintf(fu.name, sizeof(fu.name), "FetchWarp%d", id);
    dc_fw_set_ops(fu);
    dc_drm.fu_disp[fu.id] = fu;
    return 0;
    }
    static const struct component_ops dc_fw_ops = {
    .bind = dc_fw_bind,
    };
#[no_mangle]
unsafe extern "C" fn dc_fw_probe(pdev: *mut platform_device) -> c_int {
    static int dc_fw_probe(struct platform_device *pdev)
    {
    int ret;
    ret = component_add(&pdev.dev, &dc_fw_ops);
    if (ret)
    return dev_err_probe(&pdev.dev, ret,
    "failed to add component\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dc_fw_remove(pdev: *mut platform_device) {
    static void dc_fw_remove(struct platform_device *pdev)
    {
    component_del(&pdev.dev, &dc_fw_ops);
    }
    static const struct of_device_id dc_fw_dt_ids[] = {
    { .compatible = "fsl,imx8qxp-dc-fetchwarp" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, dc_fw_dt_ids);
    struct platform_driver dc_fw_driver = {
    .probe = dc_fw_probe,
    .remove = dc_fw_remove,
    .driver = {
    .name = "imx8-dc-fetchwarp",
    .suppress_bind_attrs = true,
    .of_match_table = dc_fw_dt_ids,
    },
    };
