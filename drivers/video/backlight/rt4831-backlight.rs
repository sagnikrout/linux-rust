//! Automatically rewritten from C to Rust
//! Source: drivers/video/backlight/rt4831-backlight.c
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

pub const RT4831_REG_BLCFG: c_uint = 0x02;
pub const RT4831_REG_BLDIML: c_uint = 0x04;
pub const RT4831_REG_ENABLE: c_uint = 0x08;
pub const RT4831_REG_BLOPT2: c_uint = 0x11;
pub const RT4831_BLMAX_BRIGHTNESS: c_int = 2048;

pub const RT4831_BLOVP_SHIFT: c_int = 5;

pub const RT4831_BLDIMH_SHIFT: c_int = 3;

pub const RT4831_BLOCP_MINUA: c_int = 900000;
pub const RT4831_BLOCP_MAXUA: c_int = 1800000;
pub const RT4831_BLOCP_STEPUA: c_int = 300000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt4831_priv {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub bl: *mut backlight_device,
}

#[no_mangle]
unsafe extern "C" fn rt4831_bl_update_status(bl_dev: *mut backlight_device) -> c_int {
    static int rt4831_bl_update_status(struct backlight_device *bl_dev)
    {
    struct rt4831_priv *priv = bl_get_data(bl_dev);
    let mut brightness: c_int = backlight_get_brightness(bl_dev);
    let mut enable: c_uint = brightness ? RT4831_BLEN_MASK : 0;
    u8 v[2];
    int ret;
    if (brightness) {
    v[0] = (brightness - 1) & RT4831_BLDIML_MASK;
    v[1] = ((brightness - 1) & RT4831_BLDIMH_MASK) >> RT4831_BLDIMH_SHIFT;
    ret = regmap_raw_write(priv.regmap, RT4831_REG_BLDIML, v, sizeof(v));
    if (ret)
    return ret;
    }
    return regmap_update_bits(priv.regmap, RT4831_REG_ENABLE, RT4831_BLEN_MASK, enable);
    }
#[no_mangle]
unsafe extern "C" fn rt4831_bl_get_brightness(bl_dev: *mut backlight_device) -> c_int {
    static int rt4831_bl_get_brightness(struct backlight_device *bl_dev)
    {
    struct rt4831_priv *priv = bl_get_data(bl_dev);
    unsigned int val;
    u8 v[2];
    int ret;
    ret = regmap_read(priv.regmap, RT4831_REG_ENABLE, &val);
    if (ret)
    return ret;
    if (!(val & RT4831_BLEN_MASK))
    return 0;
    ret = regmap_raw_read(priv.regmap, RT4831_REG_BLDIML, v, sizeof(v));
    if (ret)
    return ret;
    ret = (v[1] << RT4831_BLDIMH_SHIFT) + (v[0] & RT4831_BLDIML_MASK) + 1;
    return ret;
    }
    static const struct backlight_ops rt4831_bl_ops = {
    .options = BL_CORE_SUSPENDRESUME,
    .update_status = rt4831_bl_update_status,
    .get_brightness = rt4831_bl_get_brightness,
    };
    static int rt4831_parse_backlight_properties(struct rt4831_priv *priv,
    struct backlight_properties *bl_props)
    {
    struct device *dev = priv.dev;
    u8 propval;
    u32 brightness, ocp_uA;
    let mut val: c_uint = 0;
    int ret;
// common properties
    ret = device_property_read_u32(dev, "max-brightness", &brightness);
    if (ret)
    brightness = RT4831_BLMAX_BRIGHTNESS;
    bl_props.max_brightness = min_t(u32, brightness, RT4831_BLMAX_BRIGHTNESS);
    ret = device_property_read_u32(dev, "default-brightness", &brightness);
    if (ret)
    brightness = bl_props.max_brightness;
    bl_props.brightness = min_t(u32, brightness, bl_props.max_brightness);
// vendor properties
    if (device_property_read_bool(dev, "richtek,pwm-enable"))
    val = RT4831_BLPWMEN_MASK;
    ret = regmap_update_bits(priv.regmap, RT4831_REG_BLCFG, RT4831_BLPWMEN_MASK, val);
    if (ret)
    return ret;
    ret = device_property_read_u8(dev, "richtek,bled-ovp-sel", &propval);
    if (ret)
    propval = RT4831_BLOVPLVL_21V;
    propval = min_t(u8, propval, RT4831_BLOVPLVL_29V);
    ret = regmap_update_bits(priv.regmap, RT4831_REG_BLCFG, RT4831_BLOVP_MASK,
    propval << RT4831_BLOVP_SHIFT);
    if (ret)
    return ret;
//
// This OCP level is used to protect and limit the inductor current.
// If inductor peak current reach the level, low-side MOSFET will be
// turned off. Meanwhile, the output channel current may be limited.
// To match the configured channel current, the inductor chosen must
// be higher than the OCP level.
//
// Not like the OVP level, the default 21V can be used in the most
// application. But if the chosen OCP level is smaller than needed,
// it will also affect the backlight channel output current to be
// smaller than the register setting.
//
    ret = device_property_read_u32(dev, "richtek,bled-ocp-microamp",
    &ocp_uA);
    if (!ret) {
    ocp_uA = clamp_val(ocp_uA, RT4831_BLOCP_MINUA,
    RT4831_BLOCP_MAXUA);
    val = DIV_ROUND_UP(ocp_uA - RT4831_BLOCP_MINUA,
    RT4831_BLOCP_STEPUA);
    ret = regmap_update_bits(priv.regmap, RT4831_REG_BLOPT2,
    RT4831_BLOCP_MASK, val);
    if (ret)
    return ret;
    }
    ret = device_property_read_u8(dev, "richtek,channel-use", &propval);
    if (ret) {
    dev_err(dev, "richtek,channel-use DT property missing\n");
    return ret;
    }
    if (!(propval & RT4831_BLCH_MASK)) {
    dev_err(dev, "No channel specified\n");
    return -EINVAL;
    }
    return regmap_update_bits(priv.regmap, RT4831_REG_ENABLE, RT4831_BLCH_MASK, propval);
    }
#[no_mangle]
unsafe extern "C" fn rt4831_bl_probe(pdev: *mut platform_device) -> c_int {
    static int rt4831_bl_probe(struct platform_device *pdev)
    {
    struct rt4831_priv *priv;
    struct backlight_properties bl_props = { .type = BACKLIGHT_RAW,
    .scale = BACKLIGHT_SCALE_LINEAR };
    int ret;
    priv = devm_kzalloc(&pdev.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.dev = &pdev.dev;
    priv.regmap = dev_get_regmap(pdev.dev.parent, core::ptr::null_mut());
    if (!priv.regmap) {
    dev_err(&pdev.dev, "Failed to init regmap\n");
    return -ENODEV;
    }
    ret = rt4831_parse_backlight_properties(priv, &bl_props);
    if (ret) {
    dev_err(&pdev.dev, "Failed to parse backlight properties\n");
    return ret;
    }
    priv.bl = devm_backlight_device_register(&pdev.dev, pdev.name, &pdev.dev, priv,
    &rt4831_bl_ops, &bl_props);
    if (IS_ERR(priv.bl)) {
    dev_err(&pdev.dev, "Failed to register backlight\n");
    return PTR_ERR(priv.bl);
    }
    backlight_update_status(priv.bl);
    platform_set_drvdata(pdev, priv);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rt4831_bl_remove(pdev: *mut platform_device) {
    static void rt4831_bl_remove(struct platform_device *pdev)
    {
    struct rt4831_priv *priv = platform_get_drvdata(pdev);
    struct backlight_device *bl_dev = priv.bl;
    bl_dev.props.brightness = 0;
    backlight_update_status(priv.bl);
    }
    static const struct of_device_id __maybe_unused rt4831_bl_of_match[] = {
    { .compatible = "richtek,rt4831-backlight", },
    {}
    };
    MODULE_DEVICE_TABLE(of, rt4831_bl_of_match);
    static struct platform_driver rt4831_bl_driver = {
    .driver = {
    .name = "rt4831-backlight",
    .of_match_table = rt4831_bl_of_match,
    },
    .probe = rt4831_bl_probe,
    .remove = rt4831_bl_remove,
    };
    module_platform_driver(rt4831_bl_driver);
    MODULE_AUTHOR("ChiYuan Huang <cy_huang@richtek.com>");
    MODULE_DESCRIPTION("Richtek RT4831 Backlight Driver");
    MODULE_LICENSE("GPL v2");
