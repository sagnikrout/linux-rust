//! Automatically rewritten from C to Rust
//! Source: drivers/leds/flash/leds-rt4505.c
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

pub const RT4505_REG_RESET: c_uint = 0x0;
pub const RT4505_REG_CONFIG: c_uint = 0x8;
pub const RT4505_REG_ILED: c_uint = 0x9;
pub const RT4505_REG_ENABLE: c_uint = 0xA;
pub const RT4505_REG_FLAGS: c_uint = 0xB;

pub const RT4505_ITORCH_SHIFT: c_int = 5;

pub const RT4505_ITORCH_MINUA: c_int = 46000;
pub const RT4505_ITORCH_MAXUA: c_int = 375000;
pub const RT4505_ITORCH_STPUA: c_int = 47000;
pub const RT4505_IFLASH_MINUA: c_int = 93750;
pub const RT4505_IFLASH_MAXUA: c_int = 1500000;
pub const RT4505_IFLASH_STPUA: c_int = 93750;
pub const RT4505_FLASHTO_MINUS: c_int = 100000;
pub const RT4505_FLASHTO_MAXUS: c_int = 800000;
pub const RT4505_FLASHTO_STPUS: c_int = 100000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt4505_priv {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub lock: mutex,
    pub flash: led_classdev_flash,
    pub v4l2_flash: *mut v4l2_flash,
}

    static int rt4505_torch_brightness_set(struct led_classdev *lcdev,
    enum led_brightness level)
    {
    struct rt4505_priv *priv =
    container_of(lcdev, struct rt4505_priv, flash.led_cdev);
    let mut val: u32 = 0;
    int ret;
    mutex_lock(&priv.lock);
    if (level != LED_OFF) {
    ret = regmap_update_bits(priv.regmap,
    RT4505_REG_ILED, RT4505_ITORCH_MASK,
    (level - 1) << RT4505_ITORCH_SHIFT);
    if (ret)
    goto unlock;
    val = RT4505_TORCH_SET;
    }
    ret = regmap_update_bits(priv.regmap, RT4505_REG_ENABLE,
    RT4505_ENABLE_MASK, val);
    unlock:
    mutex_unlock(&priv.lock);
    return ret;
    }
    static enum led_brightness rt4505_torch_brightness_get(
    struct led_classdev *lcdev)
    {
    struct rt4505_priv *priv =
    container_of(lcdev, struct rt4505_priv, flash.led_cdev);
    u32 val;
    int ret;
    mutex_lock(&priv.lock);
    ret = regmap_read(priv.regmap, RT4505_REG_ENABLE, &val);
    if (ret) {
    dev_err(lcdev.dev, "Failed to get LED enable\n");
    ret = LED_OFF;
    goto unlock;
    }
    if ((val & RT4505_ENABLE_MASK) != RT4505_TORCH_SET) {
    ret = LED_OFF;
    goto unlock;
    }
    ret = regmap_read(priv.regmap, RT4505_REG_ILED, &val);
    if (ret) {
    dev_err(lcdev.dev, "Failed to get LED brightness\n");
    ret = LED_OFF;
    goto unlock;
    }
    ret = ((val & RT4505_ITORCH_MASK) >> RT4505_ITORCH_SHIFT) + 1;
    unlock:
    mutex_unlock(&priv.lock);
    return ret;
    }
    static int rt4505_flash_brightness_set(struct led_classdev_flash *fled_cdev,
    u32 brightness)
    {
    struct rt4505_priv *priv =
    container_of(fled_cdev, struct rt4505_priv, flash);
    struct led_flash_setting *s = &fled_cdev.brightness;
    let mut val: u32 = (brightness - s.min) / s.step;
    int ret;
    mutex_lock(&priv.lock);
    ret = regmap_update_bits(priv.regmap, RT4505_REG_ILED,
    RT4505_IFLASH_MASK, val);
    mutex_unlock(&priv.lock);
    return ret;
    }
    static int rt4505_flash_strobe_set(struct led_classdev_flash *fled_cdev,
    bool state)
    {
    struct rt4505_priv *priv =
    container_of(fled_cdev, struct rt4505_priv, flash);
    let mut val: u32 = state ? RT4505_FLASH_SET : 0;
    int ret;
    mutex_lock(&priv.lock);
    ret = regmap_update_bits(priv.regmap, RT4505_REG_ENABLE,
    RT4505_ENABLE_MASK, val);
    mutex_unlock(&priv.lock);
    return ret;
    }
    static int rt4505_flash_strobe_get(struct led_classdev_flash *fled_cdev,
    bool *state)
    {
    struct rt4505_priv *priv =
    container_of(fled_cdev, struct rt4505_priv, flash);
    u32 val;
    int ret;
    mutex_lock(&priv.lock);
    ret = regmap_read(priv.regmap, RT4505_REG_ENABLE, &val);
    if (ret)
    goto unlock;
// state = (val & RT4505_FLASH_GET) == RT4505_FLASH_GET;
    unlock:
    mutex_unlock(&priv.lock);
    return ret;
    }
    static int rt4505_flash_timeout_set(struct led_classdev_flash *fled_cdev,
    u32 timeout)
    {
    struct rt4505_priv *priv =
    container_of(fled_cdev, struct rt4505_priv, flash);
    struct led_flash_setting *s = &fled_cdev.timeout;
    let mut val: u32 = (timeout - s.min) / s.step;
    int ret;
    mutex_lock(&priv.lock);
    ret = regmap_update_bits(priv.regmap, RT4505_REG_CONFIG,
    RT4505_FLASHTO_MASK, val);
    mutex_unlock(&priv.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rt4505_fault_get(fled_cdev: *mut led_classdev_flash, fault: *mut u32) -> c_int {
    static int rt4505_fault_get(struct led_classdev_flash *fled_cdev, u32 *fault)
    {
    struct rt4505_priv *priv =
    container_of(fled_cdev, struct rt4505_priv, flash);
    u32 val, led_faults = 0;
    int ret;
    ret = regmap_read(priv.regmap, RT4505_REG_FLAGS, &val);
    if (ret)
    return ret;
    if (val & RT4505_OVP_MASK)
    led_faults |= LED_FAULT_OVER_VOLTAGE;
    if (val & RT4505_SHORT_MASK)
    led_faults |= LED_FAULT_SHORT_CIRCUIT;
    if (val & RT4505_OTP_MASK)
    led_faults |= LED_FAULT_OVER_TEMPERATURE;
    if (val & RT4505_TIMEOUT_MASK)
    led_faults |= LED_FAULT_TIMEOUT;
// fault = led_faults;
    return 0;
    }
    static const struct led_flash_ops rt4505_flash_ops = {
    .flash_brightness_set = rt4505_flash_brightness_set,
    .strobe_set = rt4505_flash_strobe_set,
    .strobe_get = rt4505_flash_strobe_get,
    .timeout_set = rt4505_flash_timeout_set,
    .fault_get = rt4505_fault_get,
    };
#[no_mangle]
unsafe extern "C" fn rt4505_is_accessible_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool rt4505_is_accessible_reg(struct device *dev, unsigned int reg)
    {
    if (reg == RT4505_REG_RESET ||
    (reg >= RT4505_REG_CONFIG && reg <= RT4505_REG_FLAGS))
    return true;
    return false;
    }
    static const struct regmap_config rt4505_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = RT4505_REG_FLAGS,
    .readable_reg = rt4505_is_accessible_reg,
    .writeable_reg = rt4505_is_accessible_reg,
    };

    static int rt4505_flash_external_strobe_set(struct v4l2_flash *v4l2_flash,
    bool enable)
    {
    struct led_classdev_flash *flash = v4l2_flash.fled_cdev;
    struct rt4505_priv *priv =
    container_of(flash, struct rt4505_priv, flash);
    let mut val: u32 = enable ? RT4505_EXT_FLASH_SET : 0;
    int ret;
    mutex_lock(&priv.lock);
    ret = regmap_update_bits(priv.regmap, RT4505_REG_ENABLE,
    RT4505_ENABLE_MASK, val);
    mutex_unlock(&priv.lock);
    return ret;
    }
    static const struct v4l2_flash_ops v4l2_flash_ops = {
    .external_strobe_set = rt4505_flash_external_strobe_set,
    };
    static void rt4505_init_v4l2_config(struct rt4505_priv *priv,
    struct v4l2_flash_config *config)
    {
    struct led_classdev_flash *flash = &priv.flash;
    struct led_classdev *lcdev = &flash.led_cdev;
    struct led_flash_setting *s;
    strscpy(config.dev_name, lcdev.dev.kobj.name,
    sizeof(config.dev_name));
    s = &config.intensity;
    s.min = RT4505_ITORCH_MINUA;
    s.step = RT4505_ITORCH_STPUA;
    s.max = s.val = s.min + (lcdev.max_brightness - 1) * s.step;
    config.flash_faults = LED_FAULT_OVER_VOLTAGE |
    LED_FAULT_SHORT_CIRCUIT |
    LED_FAULT_LED_OVER_TEMPERATURE |
    LED_FAULT_TIMEOUT;
    config.has_external_strobe = 1;
    }

    static const struct v4l2_flash_ops v4l2_flash_ops;
    static void rt4505_init_v4l2_config(struct rt4505_priv *priv,
    struct v4l2_flash_config *config)
    {
    }

    static void rt4505_init_flash_properties(struct rt4505_priv *priv,
    struct fwnode_handle *child)
    {
    struct led_classdev_flash *flash = &priv.flash;
    struct led_classdev *lcdev = &flash.led_cdev;
    struct led_flash_setting *s;
    u32 val;
    int ret;
    ret = fwnode_property_read_u32(child, "led-max-microamp", &val);
    if (ret) {
    dev_warn(priv.dev, "led-max-microamp DT property missing\n");
    val = RT4505_ITORCH_MINUA;
    } else
    val = clamp_val(val, RT4505_ITORCH_MINUA, RT4505_ITORCH_MAXUA);
    lcdev.max_brightness =
    (val - RT4505_ITORCH_MINUA) / RT4505_ITORCH_STPUA + 1;
    lcdev.brightness_set_blocking = rt4505_torch_brightness_set;
    lcdev.brightness_get = rt4505_torch_brightness_get;
    lcdev.flags |= LED_DEV_CAP_FLASH;
    ret = fwnode_property_read_u32(child, "flash-max-microamp", &val);
    if (ret) {
    dev_warn(priv.dev, "flash-max-microamp DT property missing\n");
    val = RT4505_IFLASH_MINUA;
    } else
    val = clamp_val(val, RT4505_IFLASH_MINUA, RT4505_IFLASH_MAXUA);
    s = &flash.brightness;
    s.min = RT4505_IFLASH_MINUA;
    s.step = RT4505_IFLASH_STPUA;
    s.max = s.val = val;
    ret = fwnode_property_read_u32(child, "flash-max-timeout-us", &val);
    if (ret) {
    dev_warn(priv.dev,
    "flash-max-timeout-us DT property missing\n");
    val = RT4505_FLASHTO_MINUS;
    } else
    val = clamp_val(val, RT4505_FLASHTO_MINUS,
    RT4505_FLASHTO_MAXUS);
    s = &flash.timeout;
    s.min = RT4505_FLASHTO_MINUS;
    s.step = RT4505_FLASHTO_STPUS;
    s.max = s.val = val;
    flash.ops = &rt4505_flash_ops;
    }
#[no_mangle]
unsafe extern "C" fn rt4505_probe(client: *mut i2c_client) -> c_int {
    static int rt4505_probe(struct i2c_client *client)
    {
    struct rt4505_priv *priv;
    struct fwnode_handle *child;
    let mut init_data: led_init_data = {};
    let mut v4l2_config: v4l2_flash_config = {};
    int ret;
    priv = devm_kzalloc(&client.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.dev = &client.dev;
    mutex_init(&priv.lock);
    priv.regmap = devm_regmap_init_i2c(client, &rt4505_regmap_config);
    if (IS_ERR(priv.regmap)) {
    dev_err(priv.dev, "Failed to allocate register map\n");
    return PTR_ERR(priv.regmap);
    }
    ret = regmap_write(priv.regmap, RT4505_REG_RESET, RT4505_RESET_MASK);
    if (ret) {
    dev_err(priv.dev, "Failed to reset registers\n");
    return ret;
    }
    child = device_get_next_child_node(&client.dev, core::ptr::null_mut());
    if (!child) {
    dev_err(priv.dev, "Failed to get child node\n");
    return -EINVAL;
    }
    init_data.fwnode = child;
    rt4505_init_flash_properties(priv, child);
    ret = devm_led_classdev_flash_register_ext(priv.dev, &priv.flash,
    &init_data);
    if (ret) {
    dev_err(priv.dev, "Failed to register flash\n");
    return ret;
    }
    rt4505_init_v4l2_config(priv, &v4l2_config);
    priv.v4l2_flash = v4l2_flash_init(priv.dev, init_data.fwnode,
    &priv.flash, &v4l2_flash_ops,
    &v4l2_config);
    if (IS_ERR(priv.v4l2_flash)) {
    dev_err(priv.dev, "Failed to register v4l2 flash\n");
    return PTR_ERR(priv.v4l2_flash);
    }
    i2c_set_clientdata(client, priv);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rt4505_remove(client: *mut i2c_client) {
    static void rt4505_remove(struct i2c_client *client)
    {
    struct rt4505_priv *priv = i2c_get_clientdata(client);
    v4l2_flash_release(priv.v4l2_flash);
    }
#[no_mangle]
unsafe extern "C" fn rt4505_shutdown(client: *mut i2c_client) {
    static void rt4505_shutdown(struct i2c_client *client)
    {
    struct rt4505_priv *priv = i2c_get_clientdata(client);
// Reset registers to make sure all off before shutdown
    regmap_write(priv.regmap, RT4505_REG_RESET, RT4505_RESET_MASK);
    }
    static const struct of_device_id __maybe_unused rt4505_leds_match[] = {
    { .compatible = "richtek,rt4505", },
    {}
    };
    MODULE_DEVICE_TABLE(of, rt4505_leds_match);
    static struct i2c_driver rt4505_driver = {
    .driver = {
    .name = "rt4505",
    .of_match_table = of_match_ptr(rt4505_leds_match),
    },
    .probe = rt4505_probe,
    .remove = rt4505_remove,
    .shutdown = rt4505_shutdown,
    };
    module_i2c_driver(rt4505_driver);
    MODULE_AUTHOR("ChiYuan Huang <cy_huang@richtek.com>");
    MODULE_DESCRIPTION("Richtek RT4505 LED driver");
    MODULE_LICENSE("GPL v2");
