//! Automatically rewritten from C to Rust
//! Source: drivers/video/backlight/aw99706.c
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
// aw99706 - Backlight driver for the AWINIC AW99706
//
// Copyright (C) 2025 Junjie Cao <caojunjie650@gmail.com>
// Copyright (C) 2025 Pengyu Luo <mitltlatltl@gmail.com>
//
// Based on vendor driver:
// Copyright (c) 2023 AWINIC Technology CO., LTD
//

pub const AW99706_MAX_BRT_LVL: c_int = 4095;
pub const AW99706_REG_MAX: c_uint = 0x1F;
pub const AW99706_ID: c_uint = 0x07;
// registers list
pub const AW99706_CFG0_REG: c_uint = 0x00;

pub const AW99706_CFG1_REG: c_uint = 0x01;

pub const AW99706_CFG2_REG: c_uint = 0x02;

pub const AW99706_CFG3_REG: c_uint = 0x03;
pub const AW99706_CFG4_REG: c_uint = 0x04;

pub const AW99706_CFG5_REG: c_uint = 0x05;

pub const AW99706_CFG6_REG: c_uint = 0x06;

pub const AW99706_CFG7_REG: c_uint = 0x07;
pub const AW99706_CFG8_REG: c_uint = 0x08;
pub const AW99706_CFG9_REG: c_uint = 0x09;
pub const AW99706_CFGA_REG: c_uint = 0x0A;
pub const AW99706_CFGB_REG: c_uint = 0x0B;
pub const AW99706_CFGC_REG: c_uint = 0x0C;
pub const AW99706_CFGD_REG: c_uint = 0x0D;
pub const AW99706_FLAG_REG: c_uint = 0x10;

pub const AW99706_CHIPID_REG: c_uint = 0x11;
pub const AW99706_LED_OPEN_FLAG_REG: c_uint = 0x12;
pub const AW99706_LED_SHORT_FLAG_REG: c_uint = 0x13;
pub const AW99706_MTPLDOSEL_REG: c_uint = 0x1E;
pub const AW99706_MTPRUN_REG: c_uint = 0x1F;

// Boost switching frequency table, in Hz
    static const u32 aw99706_sw_freq_tbl[] = {
    RESV, RESV, RESV, RESV, 300000, 400000, 500000, 600000,
    660000, 750000, 850000, 1000000, 1200000, 1330000, 1500000, 1700000
    };
// Switching current limitation table, in uA
    static const u32 aw99706_sw_ilmt_tbl[] = {
    1500000, 2000000, 2500000, 3000000
    };
// ULVO threshold table, in uV
    static const u32 aw99706_ulvo_thres_tbl[] = {
    2200000, 5000000
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aw99706_dt_prop {
    pub name: *const *const c_char,
    pub val): *const *const *const int (lookup)(struct aw99706_dt_prop prop, u32 dt_val, u8,
    pub lookup_tbl: *const *const u32,
    pub tbl_size: u8,
    pub reg: u8,
    pub mask: u8,
    pub def_val: u32,
}

    static int aw99706_dt_property_lookup(const struct aw99706_dt_prop *prop,
    u32 dt_val, u8 *val)
    {
    int i;
    if (!prop.lookup_tbl) {
    if (dt_val > (prop.mask >> __ffs(prop.mask)))
    return -EINVAL;
// val = dt_val;
    return 0;
    }
    for (i = 0; i < prop.tbl_size; i++)
    if (prop.lookup_tbl[i] != RESV && prop.lookup_tbl[i] == dt_val)
    break;
// val = i;
    let mut i: return = = prop.tbl_size ? -EINVAL : 0;
    }
pub const MIN_ILED_MAX: c_int = 5000;
pub const MAX_ILED_MAX: c_int = 50000;
pub const STEP_ILED_MAX: c_int = 500;
    static int
    aw99706_dt_property_iled_max_convert(const struct aw99706_dt_prop *prop,
    u32 dt_val, u8 *val)
    {
    if (dt_val > MAX_ILED_MAX || dt_val < MIN_ILED_MAX)
    return -EINVAL;
    if ((dt_val - MIN_ILED_MAX) % STEP_ILED_MAX)
    return -EINVAL;
// val = (dt_val - MIN_ILED_MAX) / STEP_ILED_MAX;
    return 0;
    }
    static const struct aw99706_dt_prop aw99706_dt_props[] = {
    {
    "awinic,dim-mode", aw99706_dt_property_lookup,
    core::ptr::null_mut(), 0,
    AW99706_CFG0_REG, AW99706_DIM_MODE_MASK, 1,
    },
    {
    "awinic,sw-freq-hz", aw99706_dt_property_lookup,
    aw99706_sw_freq_tbl, ARRAY_SIZE(aw99706_sw_freq_tbl),
    AW99706_CFG1_REG, AW99706_SW_FREQ_MASK, 750000,
    },
    {
    "awinic,sw-ilmt-microamp", aw99706_dt_property_lookup,
    aw99706_sw_ilmt_tbl, ARRAY_SIZE(aw99706_sw_ilmt_tbl),
    AW99706_CFG1_REG, AW99706_SW_ILMT_MASK, 3000000,
    },
    {
    "awinic,iled-max-microamp", aw99706_dt_property_iled_max_convert,
    core::ptr::null_mut(), 0,
    AW99706_CFG2_REG, AW99706_ILED_MAX_MASK, 20000,
    },
    {
    "awinic,uvlo-thres-microvolt", aw99706_dt_property_lookup,
    aw99706_ulvo_thres_tbl, ARRAY_SIZE(aw99706_ulvo_thres_tbl),
    AW99706_CFG2_REG, AW99706_UVLOSEL_MASK, 2200000,
    },
    {
    "awinic,ramp-ctl", aw99706_dt_property_lookup,
    core::ptr::null_mut(), 0,
    AW99706_CFG6_REG, AW99706_RAMP_CTL_MASK, 2,
    }
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_init_data {
    pub reg: u8,
    pub mask: u8,
    pub val: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aw99706_device {
    pub client: *mut i2c_client,
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub bl_dev: *mut backlight_device,
    pub hwen_gpio: *mut gpio_desc,
    pub init_tbl: [reg_init_data; ARRAY_SIZE(aw99706_dt_props)],
    pub bl_enable: bool,
}

    enum reg_access {
    REG_NONE_ACCESS	= 0,
    REG_RD_ACCESS	= 1,
    REG_WR_ACCESS	= 2,
    };
    static const u8 aw99706_regs[AW99706_REG_MAX + 1] = {
    [AW99706_CFG0_REG]		= REG_RD_ACCESS | REG_WR_ACCESS,
    [AW99706_CFG1_REG]		= REG_RD_ACCESS | REG_WR_ACCESS,
    [AW99706_CFG2_REG]		= REG_RD_ACCESS | REG_WR_ACCESS,
    [AW99706_CFG3_REG]		= REG_RD_ACCESS | REG_WR_ACCESS,
    [AW99706_CFG4_REG]		= REG_RD_ACCESS | REG_WR_ACCESS,
    [AW99706_CFG5_REG]		= REG_RD_ACCESS | REG_WR_ACCESS,
    [AW99706_CFG6_REG]		= REG_RD_ACCESS | REG_WR_ACCESS,
    [AW99706_CFG7_REG]		= REG_RD_ACCESS | REG_WR_ACCESS,
    [AW99706_CFG8_REG]		= REG_RD_ACCESS | REG_WR_ACCESS,
    [AW99706_CFG9_REG]		= REG_RD_ACCESS | REG_WR_ACCESS,
    [AW99706_CFGA_REG]		= REG_RD_ACCESS | REG_WR_ACCESS,
    [AW99706_CFGB_REG]		= REG_RD_ACCESS | REG_WR_ACCESS,
    [AW99706_CFGC_REG]		= REG_RD_ACCESS | REG_WR_ACCESS,
    [AW99706_CFGD_REG]		= REG_RD_ACCESS | REG_WR_ACCESS,
    [AW99706_FLAG_REG]		= REG_RD_ACCESS,
    [AW99706_CHIPID_REG]		= REG_RD_ACCESS,
    [AW99706_LED_OPEN_FLAG_REG]	= REG_RD_ACCESS,
    [AW99706_LED_SHORT_FLAG_REG]	= REG_RD_ACCESS,
//
// Write bit is dropped here, writing BIT(0) to MTPLDOSEL will unlock
// Multi-time Programmable (MTP).
//
    [AW99706_MTPLDOSEL_REG]		= REG_RD_ACCESS,
    [AW99706_MTPRUN_REG]		= REG_NONE_ACCESS,
    };
#[no_mangle]
unsafe extern "C" fn aw99706_readable_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool aw99706_readable_reg(struct device *dev, unsigned int reg)
    {
    return aw99706_regs[reg] & REG_RD_ACCESS;
    }
#[no_mangle]
unsafe extern "C" fn aw99706_writeable_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool aw99706_writeable_reg(struct device *dev, unsigned int reg)
    {
    return aw99706_regs[reg] & REG_WR_ACCESS;
    }
    static inline int aw99706_i2c_read(struct aw99706_device *aw, u8 reg,
    unsigned int *val)
    {
    return regmap_read(aw.regmap, reg, val);
    }
#[no_mangle]
pub unsafe extern "C" fn aw99706_i2c_write(aw: *mut aw99706_device, reg: u8, val: u8) -> c_int {
    static inline int aw99706_i2c_write(struct aw99706_device *aw, u8 reg, u8 val)
    {
    return regmap_write(aw.regmap, reg, val);
    }
    static inline int aw99706_i2c_update_bits(struct aw99706_device *aw, u8 reg,
    u8 mask, u8 val)
    {
    return regmap_update_bits(aw.regmap, reg, mask, val);
    }
    static void aw99706_dt_parse(struct aw99706_device *aw,
    struct backlight_properties *bl_props)
    {
    const struct aw99706_dt_prop *prop;
    u32 dt_val;
    int ret, i;
    u8 val;
    for (i = 0; i < ARRAY_SIZE(aw99706_dt_props); i++) {
    prop = &aw99706_dt_props[i];
    ret = device_property_read_u32(aw.dev, prop.name, &dt_val);
    if (ret < 0)
    dt_val = prop.def_val;
    if (prop.lookup(prop, dt_val, &val)) {
    dev_warn(aw.dev, "invalid value %d for property %s, using default value %d\n",
    dt_val, prop.name, prop.def_val);
    prop.lookup(prop, prop.def_val, &val);
    }
    aw.init_tbl[i].reg = prop.reg;
    aw.init_tbl[i].mask = prop.mask;
    aw.init_tbl[i].val = val << __ffs(prop.mask);
    }
    bl_props.brightness = AW99706_MAX_BRT_LVL >> 1;
    bl_props.max_brightness = AW99706_MAX_BRT_LVL;
    device_property_read_u32(aw.dev, "default-brightness",
    &bl_props.brightness);
    device_property_read_u32(aw.dev, "max-brightness",
    &bl_props.max_brightness);
    if (bl_props.max_brightness > AW99706_MAX_BRT_LVL)
    bl_props.max_brightness = AW99706_MAX_BRT_LVL;
    if (bl_props.brightness > bl_props.max_brightness)
    bl_props.brightness = bl_props.max_brightness;
    }
#[no_mangle]
unsafe extern "C" fn aw99706_hw_init(aw: *mut aw99706_device) -> c_int {
    static int aw99706_hw_init(struct aw99706_device *aw)
    {
    int ret, i;
    gpiod_set_value_cansleep(aw.hwen_gpio, 1);
    for (i = 0; i < ARRAY_SIZE(aw.init_tbl); i++) {
    ret = aw99706_i2c_update_bits(aw, aw.init_tbl[i].reg,
    aw.init_tbl[i].mask,
    aw.init_tbl[i].val);
    if (ret < 0) {
    dev_err(aw.dev, "Failed to write init data %d\n", ret);
    return ret;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aw99706_bl_enable(aw: *mut aw99706_device, en: bool) -> c_int {
    static int aw99706_bl_enable(struct aw99706_device *aw, bool en)
    {
    int ret;
    u8 val;
    val = FIELD_PREP(AW99706_BACKLIGHT_EN_MASK, en);
    ret = aw99706_i2c_update_bits(aw, AW99706_CFGD_REG,
    AW99706_BACKLIGHT_EN_MASK, val);
    if (ret)
    dev_err(aw.dev, "Failed to enable backlight!\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn aw99706_update_brightness(aw: *mut aw99706_device, brt_lvl: u32) -> c_int {
    static int aw99706_update_brightness(struct aw99706_device *aw, u32 brt_lvl)
    {
    let mut bl_enable_now: bool = !!brt_lvl;
    int ret;
    ret = aw99706_i2c_write(aw, AW99706_CFG4_REG,
    (brt_lvl >> 8) & AW99706_BRT_MSB_MASK);
    if (ret < 0)
    return ret;
    ret = aw99706_i2c_write(aw, AW99706_CFG5_REG,
    brt_lvl & AW99706_BRT_LSB_MASK);
    if (ret < 0)
    return ret;
    if (aw.bl_enable != bl_enable_now) {
    ret = aw99706_bl_enable(aw, bl_enable_now);
    if (!ret)
    aw.bl_enable = bl_enable_now;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn aw99706_bl_update_status(bl: *mut backlight_device) -> c_int {
    static int aw99706_bl_update_status(struct backlight_device *bl)
    {
    struct aw99706_device *aw = bl_get_data(bl);
    return aw99706_update_brightness(aw, backlight_get_brightness(bl));
    }
    static const struct backlight_ops aw99706_bl_ops = {
    .options = BL_CORE_SUSPENDRESUME,
    .update_status = aw99706_bl_update_status,
    };
    static const struct regmap_config aw99706_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = AW99706_REG_MAX,
    .writeable_reg = aw99706_writeable_reg,
    .readable_reg = aw99706_readable_reg,
    };
#[no_mangle]
unsafe extern "C" fn aw99706_chip_id_read(aw: *mut aw99706_device) -> c_int {
    static int aw99706_chip_id_read(struct aw99706_device *aw)
    {
    int ret;
    unsigned int val;
    ret = aw99706_i2c_read(aw, AW99706_CHIPID_REG, &val);
    if (ret < 0)
    return ret;
    return val;
    }
#[no_mangle]
unsafe extern "C" fn aw99706_probe(client: *mut i2c_client) -> c_int {
    static int aw99706_probe(struct i2c_client *client)
    {
    struct device *dev = &client.dev;
    struct aw99706_device *aw;
    struct backlight_device *bl_dev;
    let mut props: backlight_properties = {};
    let mut ret: c_int = 0;
    aw = devm_kzalloc(dev, sizeof(*aw), GFP_KERNEL);
    if (!aw)
    return -ENOMEM;
    aw.client = client;
    aw.dev = dev;
    i2c_set_clientdata(client, aw);
    aw.regmap = devm_regmap_init_i2c(client, &aw99706_regmap_config);
    if (IS_ERR(aw.regmap))
    return dev_err_probe(dev, PTR_ERR(aw.regmap),
    "Failed to init regmap\n");
    ret = aw99706_chip_id_read(aw);
    if (ret != AW99706_ID)
    return dev_err_probe(dev, -ENODEV,
    "Unknown chip id 0x%02x\n", ret);
    aw99706_dt_parse(aw, &props);
    aw.hwen_gpio = devm_gpiod_get(aw.dev, "enable", GPIOD_OUT_LOW);
    if (IS_ERR(aw.hwen_gpio))
    return dev_err_probe(dev, PTR_ERR(aw.hwen_gpio),
    "Failed to get enable gpio\n");
    ret = aw99706_hw_init(aw);
    if (ret < 0)
    return dev_err_probe(dev, ret,
    "Failed to initialize the chip\n");
    props.type = BACKLIGHT_RAW;
    props.scale = BACKLIGHT_SCALE_LINEAR;
    bl_dev = devm_backlight_device_register(dev, "aw99706-backlight", dev,
    aw, &aw99706_bl_ops, &props);
    if (IS_ERR(bl_dev))
    return dev_err_probe(dev, PTR_ERR(bl_dev),
    "Failed to register backlight!\n");
    aw.bl_dev = bl_dev;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aw99706_remove(client: *mut i2c_client) {
    static void aw99706_remove(struct i2c_client *client)
    {
    struct aw99706_device *aw = i2c_get_clientdata(client);
    aw99706_update_brightness(aw, 0);
    msleep(50);
    gpiod_set_value_cansleep(aw.hwen_gpio, 0);
    }
#[no_mangle]
unsafe extern "C" fn aw99706_suspend(dev: *mut device) -> c_int {
    static int aw99706_suspend(struct device *dev)
    {
    struct aw99706_device *aw = dev_get_drvdata(dev);
    return aw99706_update_brightness(aw, 0);
    }
#[no_mangle]
unsafe extern "C" fn aw99706_resume(dev: *mut device) -> c_int {
    static int aw99706_resume(struct device *dev)
    {
    struct aw99706_device *aw = dev_get_drvdata(dev);
    return aw99706_hw_init(aw);
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(aw99706_pm_ops, aw99706_suspend, aw99706_resume);
    static const struct i2c_device_id aw99706_ids[] = {
    { .name = "aw99706" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, aw99706_ids);
    static const struct of_device_id aw99706_match_table[] = {
    { .compatible = "awinic,aw99706", },
    { }
    };
    MODULE_DEVICE_TABLE(of, aw99706_match_table);
    static struct i2c_driver aw99706_i2c_driver = {
    .probe = aw99706_probe,
    .remove = aw99706_remove,
    .id_table = aw99706_ids,
    .driver = {
    .name = "aw99706",
    .of_match_table = aw99706_match_table,
    .pm = pm_ptr(&aw99706_pm_ops),
    },
    };
    module_i2c_driver(aw99706_i2c_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("BackLight driver for aw99706");
