//! Automatically rewritten from C to Rust
//! Source: drivers/video/backlight/lm3509_bl.c
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


// SPDX-License-Identifier: GPL-2.0-or-later

pub const LM3509_SINK_MAIN: c_int = 0;
pub const LM3509_SINK_SUB: c_int = 1;
pub const LM3509_NUM_SINKS: c_int = 2;
pub const LM3509_DEF_BRIGHTNESS: c_uint = 0x12;
pub const LM3509_MAX_BRIGHTNESS: c_uint = 0x1F;
pub const REG_GP: c_uint = 0x10;
pub const REG_BMAIN: c_uint = 0xA0;
pub const REG_BSUB: c_uint = 0xB0;
pub const REG_MAX: c_uint = 0xFF;
    enum {
    REG_GP_ENM_BIT = 0,
    REG_GP_ENS_BIT,
    REG_GP_UNI_BIT,
    REG_GP_RMP0_BIT,
    REG_GP_RMP1_BIT,
    REG_GP_OLED_BIT,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lm3509_bl {
    pub regmap: *mut regmap,
    pub bl_main: *mut backlight_device,
    pub bl_sub: *mut backlight_device,
    pub reset_gpio: *mut gpio_desc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lm3509_bl_led_data {
    pub label: *const c_char,
    pub led_sources: c_int,
    pub brightness: u32,
    pub max_brightness: u32,
}

#[no_mangle]
unsafe extern "C" fn lm3509_reset(data: *mut lm3509_bl) {
    static void lm3509_reset(struct lm3509_bl *data)
    {
    if (data.reset_gpio) {
    gpiod_set_value(data.reset_gpio, 1);
    udelay(1);
    gpiod_set_value(data.reset_gpio, 0);
    udelay(10);
    }
    }
    static int lm3509_update_status(struct backlight_device *bl,
    unsigned int en_mask, unsigned int br_reg)
    {
    struct lm3509_bl *data = bl_get_data(bl);
    int ret;
    bool en;
    ret = regmap_write(data.regmap, br_reg, backlight_get_brightness(bl));
    if (ret < 0)
    return ret;
    en = !backlight_is_blank(bl);
    return regmap_update_bits(data.regmap, REG_GP, en_mask,
    en ? en_mask : 0);
    }
#[no_mangle]
unsafe extern "C" fn lm3509_main_update_status(bl: *mut backlight_device) -> c_int {
    static int lm3509_main_update_status(struct backlight_device *bl)
    {
    return lm3509_update_status(bl, BIT(REG_GP_ENM_BIT), REG_BMAIN);
    }
    static const struct backlight_ops lm3509_main_ops = {
    .options = BL_CORE_SUSPENDRESUME,
    .update_status = lm3509_main_update_status,
    };
#[no_mangle]
unsafe extern "C" fn lm3509_sub_update_status(bl: *mut backlight_device) -> c_int {
    static int lm3509_sub_update_status(struct backlight_device *bl)
    {
    return lm3509_update_status(bl, BIT(REG_GP_ENS_BIT), REG_BSUB);
    }
    static const struct backlight_ops lm3509_sub_ops = {
    .options = BL_CORE_SUSPENDRESUME,
    .update_status = lm3509_sub_update_status,
    };
    static struct backlight_device *
    lm3509_backlight_register(struct device *dev, const char *name_suffix,
    struct lm3509_bl *data,
    const struct backlight_ops *ops,
    const struct lm3509_bl_led_data *led_data)
    {
    struct backlight_device *bd;
    struct backlight_properties props;
    const char *label = led_data.label;
    char name[64];
    memset(&props, 0, sizeof(props));
    props.type = BACKLIGHT_RAW;
    props.brightness = led_data.brightness;
    props.max_brightness = led_data.max_brightness;
    props.scale = BACKLIGHT_SCALE_NON_LINEAR;
    if (!label) {
    snprintf(name, sizeof(name), "lm3509-%s-%s", dev_name(dev),
    name_suffix);
    label = name;
    }
    bd = devm_backlight_device_register(dev, label, dev, data, ops, &props);
    if (IS_ERR(bd))
    return bd;
    backlight_update_status(bd);
    return bd;
    }
    static const struct regmap_config lm3509_regmap = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = REG_MAX,
    };
    static int lm3509_parse_led_sources(struct device_node *node,
    int default_led_sources)
    {
    u32 sources[LM3509_NUM_SINKS];
    int ret, num_sources, i;
    num_sources = of_property_count_u32_elems(node, "led-sources");
    if (num_sources < 0)
    return default_led_sources;
#[no_mangle]
pub unsafe extern "C" fn if(ARRAY_SIZE(sources): num_sources >) -> else {
    else if (num_sources > ARRAY_SIZE(sources))
    return -EINVAL;
    ret = of_property_read_u32_array(node, "led-sources", sources,
    num_sources);
    if (ret)
    return ret;
    for (i = 0; i < num_sources; i++) {
    if (sources[i] >= LM3509_NUM_SINKS)
    return -EINVAL;
    ret |= BIT(sources[i]);
    }
    return ret;
    }
    static int lm3509_parse_dt_node(struct device *dev,
    struct lm3509_bl_led_data *led_data)
    {
    let mut seen_led_sources: c_int = 0;
    for_each_child_of_node_scoped(dev.of_node, child) {
    struct lm3509_bl_led_data *ld;
    int ret;
    u32 reg;
    int valid_led_sources;
    ret = of_property_read_u32(child, "reg", &reg);
    if (ret < 0)
    return ret;
    if (reg >= LM3509_NUM_SINKS)
    return -EINVAL;
    ld = &led_data[reg];
    ld.led_sources = lm3509_parse_led_sources(child, BIT(reg));
    if (ld.led_sources < 0)
    return ld.led_sources;
    if (reg == 0)
    valid_led_sources = BIT(LM3509_SINK_MAIN) |
    BIT(LM3509_SINK_SUB);
    else
    valid_led_sources = BIT(LM3509_SINK_SUB);
    if (ld.led_sources != (ld.led_sources & valid_led_sources))
    return -EINVAL;
    if (seen_led_sources & ld.led_sources)
    return -EINVAL;
    seen_led_sources |= ld.led_sources;
    ld.label = core::ptr::null_mut();
    of_property_read_string(child, "label", &ld.label);
    ld.max_brightness = LM3509_MAX_BRIGHTNESS;
    of_property_read_u32(child, "max-brightness",
    &ld.max_brightness);
    ld.max_brightness =
    min_t(u32, ld.max_brightness, LM3509_MAX_BRIGHTNESS);
    ld.brightness = LM3509_DEF_BRIGHTNESS;
    of_property_read_u32(child, "default-brightness",
    &ld.brightness);
    ld.brightness = min_t(u32, ld.brightness, ld.max_brightness);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lm3509_probe(client: *mut i2c_client) -> c_int {
    static int lm3509_probe(struct i2c_client *client)
    {
    struct lm3509_bl *data;
    struct device *dev = &client.dev;
    int ret;
    let mut oled_mode: bool = false;
    let mut reg_gp_val: c_uint = 0;
    struct lm3509_bl_led_data led_data[LM3509_NUM_SINKS];
    let mut rate_of_change: u32 = 0;
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_I2C)) {
    dev_err(dev, "i2c functionality check failed\n");
    return -EOPNOTSUPP;
    }
    data = devm_kzalloc(dev, sizeof(struct lm3509_bl), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.regmap = devm_regmap_init_i2c(client, &lm3509_regmap);
    if (IS_ERR(data.regmap))
    return PTR_ERR(data.regmap);
    i2c_set_clientdata(client, data);
    data.reset_gpio = devm_gpiod_get_optional(dev, "reset", GPIOD_OUT_LOW);
    if (IS_ERR(data.reset_gpio))
    return dev_err_probe(dev, PTR_ERR(data.reset_gpio),
    "Failed to get 'reset' gpio\n");
    lm3509_reset(data);
    memset(led_data, 0, sizeof(led_data));
    ret = lm3509_parse_dt_node(dev, led_data);
    if (ret)
    return ret;
    oled_mode = of_property_read_bool(dev.of_node, "ti,oled-mode");
    if (!of_property_read_u32(dev.of_node,
    "ti,brightness-rate-of-change-us",
    &rate_of_change)) {
    switch (rate_of_change) {
    case 51:
    reg_gp_val = 0;
    break;
    case 13000:
    reg_gp_val = BIT(REG_GP_RMP1_BIT);
    break;
    case 26000:
    reg_gp_val = BIT(REG_GP_RMP0_BIT);
    break;
    case 52000:
    reg_gp_val = BIT(REG_GP_RMP0_BIT) |
    BIT(REG_GP_RMP1_BIT);
    break;
    default:
    dev_warn(dev, "invalid rate of change %u\n",
    rate_of_change);
    break;
    }
    }
    if (led_data[0].led_sources ==
    (BIT(LM3509_SINK_MAIN) | BIT(LM3509_SINK_SUB)))
    reg_gp_val |= BIT(REG_GP_UNI_BIT);
    if (oled_mode)
    reg_gp_val |= BIT(REG_GP_OLED_BIT);
    ret = regmap_write(data.regmap, REG_GP, reg_gp_val);
    if (ret < 0)
    return dev_err_probe(dev, ret, "failed to write register\n");
    if (led_data[0].led_sources) {
    data.bl_main = lm3509_backlight_register(
    dev, "main", data, &lm3509_main_ops, &led_data[0]);
    if (IS_ERR(data.bl_main)) {
    return dev_err_probe(
    dev, PTR_ERR(data.bl_main),
    "failed to register main backlight\n");
    }
    }
    if (led_data[1].led_sources) {
    data.bl_sub = lm3509_backlight_register(
    dev, "sub", data, &lm3509_sub_ops, &led_data[1]);
    if (IS_ERR(data.bl_sub)) {
    return dev_err_probe(
    dev, PTR_ERR(data.bl_sub),
    "failed to register secondary backlight\n");
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lm3509_remove(client: *mut i2c_client) {
    static void lm3509_remove(struct i2c_client *client)
    {
    struct lm3509_bl *data = i2c_get_clientdata(client);
    regmap_write(data.regmap, REG_GP, 0x00);
    }
    static const struct i2c_device_id lm3509_id[] = {
    { .name = LM3509_NAME },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, lm3509_id);
    static const struct of_device_id lm3509_match_table[] = {
    {
    .compatible = "ti,lm3509",
    },
    {},
    };
    MODULE_DEVICE_TABLE(of, lm3509_match_table);
    static struct i2c_driver lm3509_i2c_driver = {
    .driver = {
    .name = LM3509_NAME,
    .of_match_table = lm3509_match_table,
    },
    .probe = lm3509_probe,
    .remove = lm3509_remove,
    .id_table = lm3509_id,
    };
    module_i2c_driver(lm3509_i2c_driver);
    MODULE_DESCRIPTION("Texas Instruments Backlight driver for LM3509");
    MODULE_AUTHOR("Patrick Gansterer <paroga@paroga.com>");
    MODULE_LICENSE("GPL");
