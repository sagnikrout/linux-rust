//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/imx/dc/dc-ed.c
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

pub const PIXENGCFG_STATIC: c_uint = 0x8;

pub const DIV_RESET: c_uint = 0x80;

pub const SINGLE: c_int = 0;

pub const PIXENGCFG_DYNAMIC: c_uint = 0xc;
pub const PIXENGCFG_TRIGGER: c_uint = 0x14;

pub const STATICCONTROL: c_uint = 0x8;

pub const CONTROL: c_uint = 0xc;

    static const struct dc_subdev_info dc_ed_info[] = {
    { .reg_start = 0x56180980, .id = 0, },
    { .reg_start = 0x56180a00, .id = 1, },
    { .reg_start = 0x561809c0, .id = 4, },
    { .reg_start = 0x56180a40, .id = 5, },
    };
    static const struct regmap_range dc_ed_pec_regmap_write_ranges[] = {
    regmap_reg_range(PIXENGCFG_STATIC, PIXENGCFG_STATIC),
    regmap_reg_range(PIXENGCFG_DYNAMIC, PIXENGCFG_DYNAMIC),
    regmap_reg_range(PIXENGCFG_TRIGGER, PIXENGCFG_TRIGGER),
    };
    static const struct regmap_access_table dc_ed_pec_regmap_write_table = {
    .yes_ranges = dc_ed_pec_regmap_write_ranges,
    .n_yes_ranges = ARRAY_SIZE(dc_ed_pec_regmap_write_ranges),
    };
    static const struct regmap_range dc_ed_pec_regmap_read_ranges[] = {
    regmap_reg_range(PIXENGCFG_STATIC, PIXENGCFG_STATIC),
    regmap_reg_range(PIXENGCFG_DYNAMIC, PIXENGCFG_DYNAMIC),
    };
    static const struct regmap_access_table dc_ed_pec_regmap_read_table = {
    .yes_ranges = dc_ed_pec_regmap_read_ranges,
    .n_yes_ranges = ARRAY_SIZE(dc_ed_pec_regmap_read_ranges),
    };
    static const struct regmap_range dc_ed_pec_regmap_volatile_ranges[] = {
    regmap_reg_range(PIXENGCFG_TRIGGER, PIXENGCFG_TRIGGER),
    };
    static const struct regmap_access_table dc_ed_pec_regmap_volatile_table = {
    .yes_ranges = dc_ed_pec_regmap_volatile_ranges,
    .n_yes_ranges = ARRAY_SIZE(dc_ed_pec_regmap_volatile_ranges),
    };
    static const struct regmap_config dc_ed_pec_regmap_config = {
    .name = "pec",
    .reg_bits = 32,
    .reg_stride = 4,
    .val_bits = 32,
    .fast_io = true,
    .wr_table = &dc_ed_pec_regmap_write_table,
    .rd_table = &dc_ed_pec_regmap_read_table,
    .volatile_table = &dc_ed_pec_regmap_volatile_table,
    .max_register = PIXENGCFG_TRIGGER,
    };
    static const struct regmap_range dc_ed_regmap_ranges[] = {
    regmap_reg_range(STATICCONTROL, STATICCONTROL),
    regmap_reg_range(CONTROL, CONTROL),
    };
    static const struct regmap_access_table dc_ed_regmap_access_table = {
    .yes_ranges = dc_ed_regmap_ranges,
    .n_yes_ranges = ARRAY_SIZE(dc_ed_regmap_ranges),
    };
    static const struct regmap_config dc_ed_cfg_regmap_config = {
    .name = "cfg",
    .reg_bits = 32,
    .reg_stride = 4,
    .val_bits = 32,
    .fast_io = true,
    .wr_table = &dc_ed_regmap_access_table,
    .rd_table = &dc_ed_regmap_access_table,
    .max_register = CONTROL,
    };
    static const enum dc_link_id src_sels[] = {
    LINK_ID_NONE,
    LINK_ID_CONSTFRAME0,
    LINK_ID_CONSTFRAME1,
    LINK_ID_CONSTFRAME4,
    LINK_ID_CONSTFRAME5,
    LINK_ID_LAYERBLEND3,
    LINK_ID_LAYERBLEND2,
    LINK_ID_LAYERBLEND1,
    LINK_ID_LAYERBLEND0,
    };
#[no_mangle]
pub unsafe extern "C" fn dc_ed_pec_enable_shden(ed: *mut dc_ed) {
    static inline void dc_ed_pec_enable_shden(struct dc_ed *ed)
    {
    regmap_write_bits(ed.reg_pec, PIXENGCFG_STATIC, SHDEN, SHDEN);
    }
#[no_mangle]
pub unsafe extern "C" fn dc_ed_pec_poweron(ed: *mut dc_ed) {
    static inline void dc_ed_pec_poweron(struct dc_ed *ed)
    {
    regmap_write_bits(ed.reg_pec, PIXENGCFG_STATIC, POWERDOWN, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn dc_ed_pec_sync_mode_single(ed: *mut dc_ed) {
    static inline void dc_ed_pec_sync_mode_single(struct dc_ed *ed)
    {
    regmap_write_bits(ed.reg_pec, PIXENGCFG_STATIC, SYNC_MODE, SINGLE);
    }
#[no_mangle]
pub unsafe extern "C" fn dc_ed_pec_div_reset(ed: *mut dc_ed) {
    static inline void dc_ed_pec_div_reset(struct dc_ed *ed)
    {
    regmap_write_bits(ed.reg_pec, PIXENGCFG_STATIC, DIV_MASK,
    DIV(DIV_RESET));
    }
#[no_mangle]
pub unsafe extern "C" fn dc_ed_pec_src_sel(ed: *mut dc_ed, src: enum dc_link_id) {
    void dc_ed_pec_src_sel(struct dc_ed *ed, enum dc_link_id src)
    {
    int i;
    for (i = 0; i < ARRAY_SIZE(src_sels); i++) {
    if (src_sels[i] == src) {
    regmap_write(ed.reg_pec, PIXENGCFG_DYNAMIC, src);
    return;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn dc_ed_pec_sync_trigger(ed: *mut dc_ed) {
    void dc_ed_pec_sync_trigger(struct dc_ed *ed)
    {
    regmap_write(ed.reg_pec, PIXENGCFG_TRIGGER, SYNC_TRIGGER);
    }
#[no_mangle]
pub unsafe extern "C" fn dc_ed_enable_shden(ed: *mut dc_ed) {
    static inline void dc_ed_enable_shden(struct dc_ed *ed)
    {
    regmap_write_bits(ed.reg_cfg, STATICCONTROL, SHDEN, SHDEN);
    }
#[no_mangle]
pub unsafe extern "C" fn dc_ed_kick_mode_external(ed: *mut dc_ed) {
    static inline void dc_ed_kick_mode_external(struct dc_ed *ed)
    {
    regmap_write_bits(ed.reg_cfg, STATICCONTROL, KICK_MODE, EXTERNAL);
    }
#[no_mangle]
pub unsafe extern "C" fn dc_ed_disable_perfcountmode(ed: *mut dc_ed) {
    static inline void dc_ed_disable_perfcountmode(struct dc_ed *ed)
    {
    regmap_write_bits(ed.reg_cfg, STATICCONTROL, PERFCOUNTMODE, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn dc_ed_disable_gamma_apply(ed: *mut dc_ed) {
    static inline void dc_ed_disable_gamma_apply(struct dc_ed *ed)
    {
    regmap_write_bits(ed.reg_cfg, CONTROL, GAMMAAPPLYENABLE, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn dc_ed_init(ed: *mut dc_ed) {
    void dc_ed_init(struct dc_ed *ed)
    {
    dc_ed_pec_src_sel(ed, LINK_ID_NONE);
    dc_ed_pec_enable_shden(ed);
    dc_ed_pec_poweron(ed);
    dc_ed_pec_sync_mode_single(ed);
    dc_ed_pec_div_reset(ed);
    dc_ed_enable_shden(ed);
    dc_ed_disable_perfcountmode(ed);
    dc_ed_kick_mode_external(ed);
    dc_ed_disable_gamma_apply(ed);
    }
#[no_mangle]
unsafe extern "C" fn dc_ed_bind(dev: *mut device, master: *mut device, data: *mut c_void) -> c_int {
    static int dc_ed_bind(struct device *dev, struct device *master, void *data)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct dc_drm_device *dc_drm = data;
    struct resource *res_pec;
    void __iomem *base_pec;
    void __iomem *base_cfg;
    struct dc_ed *ed;
    int id;
    ed = devm_kzalloc(dev, sizeof(*ed), GFP_KERNEL);
    if (!ed)
    return -ENOMEM;
    base_pec = devm_platform_get_and_ioremap_resource(pdev, 0, &res_pec);
    if (IS_ERR(base_pec))
    return PTR_ERR(base_pec);
    base_cfg = devm_platform_ioremap_resource_byname(pdev, "cfg");
    if (IS_ERR(base_cfg))
    return PTR_ERR(base_cfg);
    ed.reg_pec = devm_regmap_init_mmio(dev, base_pec,
    &dc_ed_pec_regmap_config);
    if (IS_ERR(ed.reg_pec))
    return PTR_ERR(ed.reg_pec);
    ed.reg_cfg = devm_regmap_init_mmio(dev, base_cfg,
    &dc_ed_cfg_regmap_config);
    if (IS_ERR(ed.reg_cfg))
    return PTR_ERR(ed.reg_cfg);
    ed.irq_shdload = platform_get_irq_byname(pdev, "shdload");
    if (ed.irq_shdload < 0)
    return ed.irq_shdload;
    ed.dev = dev;
    id = dc_subdev_get_id(dc_ed_info, ARRAY_SIZE(dc_ed_info), res_pec);
    if (id < 0) {
    dev_err(dev, "failed to get instance number: %d\n", id);
    return id;
    }
    switch (id) {
    case 0:
    dc_drm.ed_cont[0] = ed;
    break;
    case 1:
    dc_drm.ed_cont[1] = ed;
    break;
    case 4:
    dc_drm.ed_safe[0] = ed;
    break;
    case 5:
    dc_drm.ed_safe[1] = ed;
    break;
    }
    return 0;
    }
    static const struct component_ops dc_ed_ops = {
    .bind = dc_ed_bind,
    };
#[no_mangle]
unsafe extern "C" fn dc_ed_probe(pdev: *mut platform_device) -> c_int {
    static int dc_ed_probe(struct platform_device *pdev)
    {
    int ret;
    ret = component_add(&pdev.dev, &dc_ed_ops);
    if (ret)
    return dev_err_probe(&pdev.dev, ret,
    "failed to add component\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dc_ed_remove(pdev: *mut platform_device) {
    static void dc_ed_remove(struct platform_device *pdev)
    {
    component_del(&pdev.dev, &dc_ed_ops);
    }
    static const struct of_device_id dc_ed_dt_ids[] = {
    { .compatible = "fsl,imx8qxp-dc-extdst" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, dc_ed_dt_ids);
    struct platform_driver dc_ed_driver = {
    .probe = dc_ed_probe,
    .remove = dc_ed_remove,
    .driver = {
    .name = "imx8-dc-extdst",
    .suppress_bind_attrs = true,
    .of_match_table = dc_ed_dt_ids,
    },
    };
