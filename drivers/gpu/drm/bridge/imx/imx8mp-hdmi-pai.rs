//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/bridge/imx/imx8mp-hdmi-pai.c
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
// Copyright 2025 NXP
//

pub const HTX_PAI_CTRL: c_uint = 0x00;

pub const HTX_PAI_CTRL_EXT: c_uint = 0x04;

pub const HTX_PAI_FIELD_CTRL: c_uint = 0x08;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx8mp_hdmi_pai {
    pub regmap: *mut regmap,
    pub dev: *mut device,
}

    static void imx8mp_hdmi_pai_enable(struct dw_hdmi *dw_hdmi, int channel,
    int width, int rate, int non_pcm,
    int iec958)
    {
    const struct dw_hdmi_plat_data *pdata = dw_hdmi_to_plat_data(dw_hdmi);
    struct imx8mp_hdmi_pai *hdmi_pai = pdata.priv_audio;
    int val;
    if (pm_runtime_resume_and_get(hdmi_pai.dev) < 0)
    return;
// PAI set control extended
    val =  WTMK_HIGH(3) | WTMK_LOW(3);
    val |= NUM_CH(channel);
    regmap_write(hdmi_pai.regmap, HTX_PAI_CTRL_EXT, val);
// IEC60958 format
    if (iec958) {
    val = FIELD_PREP_CONST(P_SEL,
    __bf_shf(IEC958_SUBFRAME_PARITY));
    val |= FIELD_PREP_CONST(C_SEL,
    __bf_shf(IEC958_SUBFRAME_CHANNEL_STATUS));
    val |= FIELD_PREP_CONST(U_SEL,
    __bf_shf(IEC958_SUBFRAME_USER_DATA));
    val |= FIELD_PREP_CONST(V_SEL,
    __bf_shf(IEC958_SUBFRAME_VALIDITY));
    val |= FIELD_PREP_CONST(D_SEL,
    __bf_shf(IEC958_SUBFRAME_SAMPLE_24_MASK));
    val |= FIELD_PREP_CONST(PRE_SEL,
    __bf_shf(IEC958_SUBFRAME_PREAMBLE_MASK));
    } else {
//
// The allowed PCM widths are 24bit and 32bit, as they are supported
// by aud2htx module.
// for 24bit, D_SEL = 0, select all the bits.
// for 32bit, D_SEL = 8, select 24bit in MSB.
//
    val = FIELD_PREP(D_SEL, width - 24);
    }
    regmap_write(hdmi_pai.regmap, HTX_PAI_FIELD_CTRL, val);
// PAI start running
    regmap_write(hdmi_pai.regmap, HTX_PAI_CTRL, ENABLE);
    }
#[no_mangle]
unsafe extern "C" fn imx8mp_hdmi_pai_disable(dw_hdmi: *mut dw_hdmi) {
    static void imx8mp_hdmi_pai_disable(struct dw_hdmi *dw_hdmi)
    {
    const struct dw_hdmi_plat_data *pdata = dw_hdmi_to_plat_data(dw_hdmi);
    struct imx8mp_hdmi_pai *hdmi_pai = pdata.priv_audio;
// Stop PAI
    regmap_write(hdmi_pai.regmap, HTX_PAI_CTRL, 0);
    pm_runtime_put_sync(hdmi_pai.dev);
    }
    static const struct regmap_config imx8mp_hdmi_pai_regmap_config = {
    .reg_bits = 32,
    .reg_stride = 4,
    .val_bits = 32,
    .max_register = HTX_PAI_FIELD_CTRL,
    };
#[no_mangle]
unsafe extern "C" fn imx8mp_hdmi_pai_bind(dev: *mut device, master: *mut device, data: *mut c_void) -> c_int {
    static int imx8mp_hdmi_pai_bind(struct device *dev, struct device *master, void *data)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct dw_hdmi_plat_data *plat_data = data;
    struct imx8mp_hdmi_pai *hdmi_pai;
    struct resource *res;
    void __iomem *base;
    int ret;
    hdmi_pai = devm_kzalloc(dev, sizeof(*hdmi_pai), GFP_KERNEL);
    if (!hdmi_pai)
    return -ENOMEM;
    base = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(base))
    return PTR_ERR(base);
    hdmi_pai.regmap = devm_regmap_init_mmio_clk(dev, "apb", base,
    &imx8mp_hdmi_pai_regmap_config);
    if (IS_ERR(hdmi_pai.regmap)) {
    dev_err(dev, "regmap init failed\n");
    return PTR_ERR(hdmi_pai.regmap);
    }
    plat_data.enable_audio = imx8mp_hdmi_pai_enable;
    plat_data.disable_audio = imx8mp_hdmi_pai_disable;
    plat_data.priv_audio = hdmi_pai;
    hdmi_pai.dev = dev;
    ret = devm_pm_runtime_enable(dev);
    if (ret < 0) {
    dev_err(dev, "failed to enable PM runtime: %d\n", ret);
    return ret;
    }
    return 0;
    }
    static const struct component_ops imx8mp_hdmi_pai_ops = {
    .bind   = imx8mp_hdmi_pai_bind,
    };
#[no_mangle]
unsafe extern "C" fn imx8mp_hdmi_pai_probe(pdev: *mut platform_device) -> c_int {
    static int imx8mp_hdmi_pai_probe(struct platform_device *pdev)
    {
    return component_add(&pdev.dev, &imx8mp_hdmi_pai_ops);
    }
#[no_mangle]
unsafe extern "C" fn imx8mp_hdmi_pai_remove(pdev: *mut platform_device) {
    static void imx8mp_hdmi_pai_remove(struct platform_device *pdev)
    {
    component_del(&pdev.dev, &imx8mp_hdmi_pai_ops);
    }
    static const struct of_device_id imx8mp_hdmi_pai_of_table[] = {
    { .compatible = "fsl,imx8mp-hdmi-pai" },
    { /* Sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, imx8mp_hdmi_pai_of_table);
    static struct platform_driver imx8mp_hdmi_pai_platform_driver = {
    .probe		= imx8mp_hdmi_pai_probe,
    .remove		= imx8mp_hdmi_pai_remove,
    .driver		= {
    .name	= "imx8mp-hdmi-pai",
    .of_match_table = imx8mp_hdmi_pai_of_table,
    },
    };
    module_platform_driver(imx8mp_hdmi_pai_platform_driver);
    MODULE_DESCRIPTION("i.MX8MP HDMI PAI driver");
    MODULE_LICENSE("GPL");
