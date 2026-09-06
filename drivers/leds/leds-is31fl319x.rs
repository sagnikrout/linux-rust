//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-is31fl319x.c
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
// Copyright 2015-16 Golden Delicious Computers
//
// Author: Nikolaus Schaller <hns@goldelico.com>
//
// LED driver for the IS31FL319{0,1,3,6,9} to drive 1, 3, 6 or 9 light
// effect LEDs.
//

// register numbers
pub const IS31FL319X_SHUTDOWN: c_uint = 0x00;
// registers for 3190, 3191 and 3193
pub const IS31FL3190_BREATHING: c_uint = 0x01;
pub const IS31FL3190_LEDMODE: c_uint = 0x02;
pub const IS31FL3190_CURRENT: c_uint = 0x03;

pub const IS31FL3190_DATA_UPDATE: c_uint = 0x07;

pub const IS31FL3190_TIME_UPDATE: c_uint = 0x1c;
pub const IS31FL3190_LEDCONTROL: c_uint = 0x1d;
pub const IS31FL3190_RESET: c_uint = 0x2f;
pub const IS31FL3190_CURRENT_uA_MIN: c_int = 5000;
pub const IS31FL3190_CURRENT_uA_DEFAULT: c_int = 42000;
pub const IS31FL3190_CURRENT_uA_MAX: c_int = 42000;
pub const IS31FL3190_CURRENT_SHIFT: c_int = 2;

pub const IS31FL3190_CURRENT_5_mA: c_uint = 0x02;
pub const IS31FL3190_CURRENT_10_mA: c_uint = 0x01;
pub const IS31FL3190_CURRENT_17dot5_mA: c_uint = 0x04;
pub const IS31FL3190_CURRENT_30_mA: c_uint = 0x03;
pub const IS31FL3190_CURRENT_42_mA: c_uint = 0x00;
// registers for 3196 and 3199
pub const IS31FL3196_CTRL1: c_uint = 0x01;
pub const IS31FL3196_CTRL2: c_uint = 0x02;
pub const IS31FL3196_CONFIG1: c_uint = 0x03;
pub const IS31FL3196_CONFIG2: c_uint = 0x04;
pub const IS31FL3196_RAMP_MODE: c_uint = 0x05;
pub const IS31FL3196_BREATH_MARK: c_uint = 0x06;

pub const IS31FL3196_DATA_UPDATE: c_uint = 0x10;

pub const IS31FL3196_T123_1: c_uint = 0x1a;
pub const IS31FL3196_T123_2: c_uint = 0x1b;
pub const IS31FL3196_T123_3: c_uint = 0x1c;

pub const IS31FL3196_TIME_UPDATE: c_uint = 0x26;
pub const IS31FL3196_RESET: c_uint = 0xff;

pub const IS31FL319X_MAX_LEDS: c_int = 9;
// CS (Current Setting) in CONFIG2 register
pub const IS31FL3196_CONFIG2_CS_SHIFT: c_int = 4;

pub const IS31FL3196_CONFIG2_CS_STEP_REF: c_int = 12;
pub const IS31FL3196_CURRENT_uA_MIN: c_int = 5000;
pub const IS31FL3196_CURRENT_uA_MAX: c_int = 40000;
pub const IS31FL3196_CURRENT_uA_STEP: c_int = 5000;
pub const IS31FL3196_CURRENT_uA_DEFAULT: c_int = 20000;
// Audio gain in CONFIG2 register

pub const IS31FL3196_AUDIO_GAIN_DB_STEP: c_int = 3;
//
// regmap is used as a cache of chip's register space,
// to avoid reading back brightness values from chip,
// which is known to hang.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct is31fl319x_chip {
    pub cdef: *const is31fl319x_chipdef,
    pub client: *mut i2c_client,
    pub shutdown_gpio: *mut gpio_desc,
    pub regmap: *mut regmap,
    pub lock: mutex,
    pub audio_gain_db: u32,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct is31fl319x_led {
    pub chip: *mut is31fl319x_chip,
    pub cdev: led_classdev,
    pub max_microamp: u32,
    pub fwnode: *mut fwnode_handle,
    pub leds: [}; IS31FL319X_MAX_LEDS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct is31fl319x_chipdef {
    pub num_leds: c_int,
    pub reset_reg: u8,
    pub is31fl319x_regmap_config: *const regmap_config,
    pub brightness): *mut *mut *mut int (brightness_set)(struct led_classdev cdev, enum led_brightness,
    pub current_default: u32,
    pub current_min: u32,
    pub current_max: u32,
    pub is_3196or3199: bool,
}

#[no_mangle]
unsafe extern "C" fn is31fl319x_readable_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool is31fl319x_readable_reg(struct device *dev, unsigned int reg)
    {
// we have no readable registers
    return false;
    }
#[no_mangle]
unsafe extern "C" fn is31fl3190_volatile_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool is31fl3190_volatile_reg(struct device *dev, unsigned int reg)
    {
// volatile registers are not cached
    switch (reg) {
    case IS31FL3190_DATA_UPDATE:
    case IS31FL3190_TIME_UPDATE:
    case IS31FL3190_RESET:
    return true; /* always write-through */
    default:
    return false;
    }
    }
    static const struct reg_default is31fl3190_reg_defaults[] = {
    { IS31FL3190_LEDMODE, 0x00 },
    { IS31FL3190_CURRENT, 0x00 },
    { IS31FL3190_PWM(0), 0x00 },
    { IS31FL3190_PWM(1), 0x00 },
    { IS31FL3190_PWM(2), 0x00 },
    };
    static const struct regmap_config is31fl3190_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = IS31FL3190_RESET,
    .cache_type = REGCACHE_FLAT,
    .readable_reg = is31fl319x_readable_reg,
    .volatile_reg = is31fl3190_volatile_reg,
    .reg_defaults = is31fl3190_reg_defaults,
    .num_reg_defaults = ARRAY_SIZE(is31fl3190_reg_defaults),
    };
#[no_mangle]
unsafe extern "C" fn is31fl3196_volatile_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool is31fl3196_volatile_reg(struct device *dev, unsigned int reg)
    {
// volatile registers are not cached
    switch (reg) {
    case IS31FL3196_DATA_UPDATE:
    case IS31FL3196_TIME_UPDATE:
    case IS31FL3196_RESET:
    return true; /* always write-through */
    default:
    return false;
    }
    }
    static const struct reg_default is31fl3196_reg_defaults[] = {
    { IS31FL3196_CONFIG1, 0x00 },
    { IS31FL3196_CONFIG2, 0x00 },
    { IS31FL3196_PWM(0), 0x00 },
    { IS31FL3196_PWM(1), 0x00 },
    { IS31FL3196_PWM(2), 0x00 },
    { IS31FL3196_PWM(3), 0x00 },
    { IS31FL3196_PWM(4), 0x00 },
    { IS31FL3196_PWM(5), 0x00 },
    { IS31FL3196_PWM(6), 0x00 },
    { IS31FL3196_PWM(7), 0x00 },
    { IS31FL3196_PWM(8), 0x00 },
    };
    static const struct regmap_config is31fl3196_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = IS31FL3196_REG_CNT,
    .cache_type = REGCACHE_FLAT,
    .readable_reg = is31fl319x_readable_reg,
    .volatile_reg = is31fl3196_volatile_reg,
    .reg_defaults = is31fl3196_reg_defaults,
    .num_reg_defaults = ARRAY_SIZE(is31fl3196_reg_defaults),
    };
    static int is31fl3190_brightness_set(struct led_classdev *cdev,
    enum led_brightness brightness)
    {
    struct is31fl319x_led *led = container_of(cdev, struct is31fl319x_led, cdev);
    struct is31fl319x_chip *is31 = led.chip;
    let mut chan: c_int = led - is31.leds;
    int ret;
    int i;
    let mut ctrl: u8 = 0;
    dev_dbg(&is31.client.dev, "channel %d: %d\n", chan, brightness);
    mutex_lock(&is31.lock);
// update PWM register
    ret = regmap_write(is31.regmap, IS31FL3190_PWM(chan), brightness);
    if (ret < 0)
    goto out;
// read current brightness of all PWM channels
    for (i = 0; i < is31.cdef.num_leds; i++) {
    unsigned int pwm_value;
    bool on;
//
// since neither cdev nor the chip can provide
// the current setting, we read from the regmap cache
//
    ret = regmap_read(is31.regmap, IS31FL3190_PWM(i), &pwm_value);
    on = ret >= 0 && pwm_value > LED_OFF;
    ctrl |= on << i;
    }
    if (ctrl > 0) {
    dev_dbg(&is31.client.dev, "power up %02x\n", ctrl);
    regmap_write(is31.regmap, IS31FL3190_LEDCONTROL, ctrl);
// update PWMs
    regmap_write(is31.regmap, IS31FL3190_DATA_UPDATE, 0x00);
// enable chip from shut down and enable all channels
    ret = regmap_write(is31.regmap, IS31FL319X_SHUTDOWN, 0x20);
    } else {
    dev_dbg(&is31.client.dev, "power down\n");
// shut down (no need to clear LEDCONTROL)
    ret = regmap_write(is31.regmap, IS31FL319X_SHUTDOWN, 0x01);
    }
    out:
    mutex_unlock(&is31.lock);
    return ret;
    }
    static int is31fl3196_brightness_set(struct led_classdev *cdev,
    enum led_brightness brightness)
    {
    struct is31fl319x_led *led = container_of(cdev, struct is31fl319x_led, cdev);
    struct is31fl319x_chip *is31 = led.chip;
    let mut chan: c_int = led - is31.leds;
    int ret;
    int i;
    let mut ctrl1: u8 = 0, ctrl2 = 0;
    dev_dbg(&is31.client.dev, "channel %d: %d\n", chan, brightness);
    mutex_lock(&is31.lock);
// update PWM register
    ret = regmap_write(is31.regmap, IS31FL3196_PWM(chan), brightness);
    if (ret < 0)
    goto out;
// read current brightness of all PWM channels
    for (i = 0; i < is31.cdef.num_leds; i++) {
    unsigned int pwm_value;
    bool on;
//
// since neither cdev nor the chip can provide
// the current setting, we read from the regmap cache
//
    ret = regmap_read(is31.regmap, IS31FL3196_PWM(i), &pwm_value);
    on = ret >= 0 && pwm_value > LED_OFF;
    if (i < 3)
    ctrl1 |= on << i;       /* 0..2 => bit 0..2 */
#[no_mangle]
pub unsafe extern "C" fn if(6: i <) -> else {
    else if (i < 6)
    ctrl1 |= on << (i + 1); /* 3..5 => bit 4..6 */
    else
    ctrl2 |= on << (i - 6); /* 6..8 => bit 0..2 */
    }
    if (ctrl1 > 0 || ctrl2 > 0) {
    dev_dbg(&is31.client.dev, "power up %02x %02x\n",
    ctrl1, ctrl2);
    regmap_write(is31.regmap, IS31FL3196_CTRL1, ctrl1);
    regmap_write(is31.regmap, IS31FL3196_CTRL2, ctrl2);
// update PWMs
    regmap_write(is31.regmap, IS31FL3196_DATA_UPDATE, 0x00);
// enable chip from shut down
    ret = regmap_write(is31.regmap, IS31FL319X_SHUTDOWN, 0x01);
    } else {
    dev_dbg(&is31.client.dev, "power down\n");
// shut down (no need to clear CTRL1/2)
    ret = regmap_write(is31.regmap, IS31FL319X_SHUTDOWN, 0x00);
    }
    out:
    mutex_unlock(&is31.lock);
    return ret;
    }
    static const struct is31fl319x_chipdef is31fl3190_cdef = {
    .num_leds = 1,
    .reset_reg = IS31FL3190_RESET,
    .is31fl319x_regmap_config = &is31fl3190_regmap_config,
    .brightness_set = is31fl3190_brightness_set,
    .current_default = IS31FL3190_CURRENT_uA_DEFAULT,
    .current_min = IS31FL3190_CURRENT_uA_MIN,
    .current_max = IS31FL3190_CURRENT_uA_MAX,
    .is_3196or3199 = false,
    };
    static const struct is31fl319x_chipdef is31fl3193_cdef = {
    .num_leds = 3,
    .reset_reg = IS31FL3190_RESET,
    .is31fl319x_regmap_config = &is31fl3190_regmap_config,
    .brightness_set = is31fl3190_brightness_set,
    .current_default = IS31FL3190_CURRENT_uA_DEFAULT,
    .current_min = IS31FL3190_CURRENT_uA_MIN,
    .current_max = IS31FL3190_CURRENT_uA_MAX,
    .is_3196or3199 = false,
    };
    static const struct is31fl319x_chipdef is31fl3196_cdef = {
    .num_leds = 6,
    .reset_reg = IS31FL3196_RESET,
    .is31fl319x_regmap_config = &is31fl3196_regmap_config,
    .brightness_set = is31fl3196_brightness_set,
    .current_default = IS31FL3196_CURRENT_uA_DEFAULT,
    .current_min = IS31FL3196_CURRENT_uA_MIN,
    .current_max = IS31FL3196_CURRENT_uA_MAX,
    .is_3196or3199 = true,
    };
    static const struct is31fl319x_chipdef is31fl3199_cdef = {
    .num_leds = 9,
    .reset_reg = IS31FL3196_RESET,
    .is31fl319x_regmap_config = &is31fl3196_regmap_config,
    .brightness_set = is31fl3196_brightness_set,
    .current_default = IS31FL3196_CURRENT_uA_DEFAULT,
    .current_min = IS31FL3196_CURRENT_uA_MIN,
    .current_max = IS31FL3196_CURRENT_uA_MAX,
    .is_3196or3199 = true,
    };
    static const struct of_device_id of_is31fl319x_match[] = {
    { .compatible = "issi,is31fl3190", .data = &is31fl3190_cdef, },
    { .compatible = "issi,is31fl3191", .data = &is31fl3190_cdef, },
    { .compatible = "issi,is31fl3193", .data = &is31fl3193_cdef, },
    { .compatible = "issi,is31fl3196", .data = &is31fl3196_cdef, },
    { .compatible = "issi,is31fl3199", .data = &is31fl3199_cdef, },
    { .compatible = "si-en,sn3190",    .data = &is31fl3190_cdef, },
    { .compatible = "si-en,sn3191",    .data = &is31fl3190_cdef, },
    { .compatible = "si-en,sn3193",    .data = &is31fl3193_cdef, },
    { .compatible = "si-en,sn3196",    .data = &is31fl3196_cdef, },
    { .compatible = "si-en,sn3199",    .data = &is31fl3199_cdef, },
    { }
    };
    MODULE_DEVICE_TABLE(of, of_is31fl319x_match);
#[no_mangle]
unsafe extern "C" fn is31_free_fwnode(data: *mut c_void) {
    static void is31_free_fwnode(void *data)
    {
    struct is31fl319x_chip *is31 = data;
    int i;
    for (i = 0; i < is31.cdef.num_leds; i++) {
    if (is31.leds[i].fwnode)
    fwnode_handle_put(is31.leds[i].fwnode);
    is31.leds[i].fwnode = core::ptr::null_mut();
    }
    }
#[no_mangle]
unsafe extern "C" fn is31fl319x_parse_fw(dev: *mut device, is31: *mut is31fl319x_chip) -> c_int {
    static int is31fl319x_parse_fw(struct device *dev, struct is31fl319x_chip *is31)
    {
    struct fwnode_handle *fwnode = dev_fwnode(dev);
    int count;
    int ret;
    is31.shutdown_gpio = devm_gpiod_get_optional(dev, "shutdown", GPIOD_OUT_HIGH);
    if (IS_ERR(is31.shutdown_gpio))
    return dev_err_probe(dev, PTR_ERR(is31.shutdown_gpio),
    "Failed to get shutdown gpio\n");
    is31.cdef = device_get_match_data(dev);
    if (!is31.cdef)
    return -ENODEV;
    ret = devm_add_action_or_reset(dev, is31_free_fwnode, is31);
    if (ret)
    return ret;
    count = 0;
    device_for_each_child_node_scoped(dev, child)
    count++;
    dev_dbg(dev, "probing with %d leds defined in DT\n", count);
    if (!count || count > is31.cdef.num_leds)
    return dev_err_probe(dev, -ENODEV,
    "Number of leds defined must be between 1 and %u\n",
    is31.cdef.num_leds);
    device_for_each_child_node_scoped(dev, child) {
    struct is31fl319x_led *led;
    u32 reg;
    ret = fwnode_property_read_u32(child, "reg", &reg);
    if (ret)
    return dev_err_probe(dev, ret, "Failed to read led 'reg' property\n");
    if (reg < 1 || reg > is31.cdef.num_leds)
    return dev_err_probe(dev, -EINVAL, "invalid led reg %u\n", reg);
    led = &is31.leds[reg - 1];
    if (led.fwnode)
    return dev_err_probe(dev, -EINVAL, "led %u is already configured\n", reg);
    led.max_microamp = is31.cdef.current_default;
    ret = fwnode_property_read_u32(child, "led-max-microamp", &led.max_microamp);
    if (!ret) {
    if (led.max_microamp < is31.cdef.current_min)
    return dev_err_probe(dev, -EINVAL, "invalid maximum current\n");
    led.max_microamp = min(led.max_microamp,
    is31.cdef.current_max);
    }
    led.fwnode = fwnode_handle_get(child);
    }
    is31.audio_gain_db = 0;
    if (is31.cdef.is_3196or3199) {
    ret = fwnode_property_read_u32(fwnode, "audio-gain-db", &is31.audio_gain_db);
    if (!ret)
    is31.audio_gain_db = min(is31.audio_gain_db,
    IS31FL3196_AUDIO_GAIN_DB_MAX);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn is31fl3190_microamp_to_cs(dev: *mut device, microamp: u32) -> c_int {
    static inline int is31fl3190_microamp_to_cs(struct device *dev, u32 microamp)
    {
    switch (microamp) {
    case 5000:
    return IS31FL3190_CURRENT_5_mA;
    case 10000:
    return IS31FL3190_CURRENT_10_mA;
    case 17500:
    return IS31FL3190_CURRENT_17dot5_mA;
    case 30000:
    return IS31FL3190_CURRENT_30_mA;
    case 42000:
    return IS31FL3190_CURRENT_42_mA;
    default:
    dev_warn(dev, "Unsupported current value: %d, using 5000 µA!\n", microamp);
    return IS31FL3190_CURRENT_5_mA;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn is31fl3196_microamp_to_cs(dev: *mut device, microamp: u32) -> c_int {
    static inline int is31fl3196_microamp_to_cs(struct device *dev, u32 microamp)
    {
// round down to nearest supported value (range check done by caller)
    let mut step: u32 = microamp / IS31FL3196_CURRENT_uA_STEP;
    return ((IS31FL3196_CONFIG2_CS_STEP_REF - step) &
    IS31FL3196_CONFIG2_CS_MASK) <<
    IS31FL3196_CONFIG2_CS_SHIFT; /* CS encoding */
    }
#[no_mangle]
pub unsafe extern "C" fn is31fl3196_db_to_gain(dezibel: u32) -> c_int {
    static inline int is31fl3196_db_to_gain(u32 dezibel)
    {
// round down to nearest supported value (range check done by caller)
    return dezibel / IS31FL3196_AUDIO_GAIN_DB_STEP;
    }
#[no_mangle]
unsafe extern "C" fn is31fl319x_probe(client: *mut i2c_client) -> c_int {
    static int is31fl319x_probe(struct i2c_client *client)
    {
    struct is31fl319x_chip *is31;
    struct device *dev = &client.dev;
    int err;
    let mut i: c_int = 0;
    u32 aggregated_led_microamp;
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_I2C))
    return -EIO;
    is31 = devm_kzalloc(&client.dev, sizeof(*is31), GFP_KERNEL);
    if (!is31)
    return -ENOMEM;
    err = devm_mutex_init(dev, &is31.lock);
    if (err)
    return err;
    err = is31fl319x_parse_fw(&client.dev, is31);
    if (err)
    return err;
    if (is31.shutdown_gpio) {
    gpiod_direction_output(is31.shutdown_gpio, 0);
    mdelay(5);
    gpiod_direction_output(is31.shutdown_gpio, 1);
    }
    is31.client = client;
    is31.regmap = devm_regmap_init_i2c(client, is31.cdef.is31fl319x_regmap_config);
    if (IS_ERR(is31.regmap))
    return dev_err_probe(dev, PTR_ERR(is31.regmap), "failed to allocate register map\n");
    i2c_set_clientdata(client, is31);
// check for write-reply from chip (we can't read any registers)
    err = regmap_write(is31.regmap, is31.cdef.reset_reg, 0x00);
    if (err < 0)
    return dev_err_probe(dev, err, "no response from chip write\n");
//
// Kernel conventions require per-LED led-max-microamp property.
// But the chip does not allow to limit individual LEDs.
// So we take minimum from all subnodes for safety of hardware.
//
    aggregated_led_microamp = is31.cdef.current_max;
    for (i = 0; i < is31.cdef.num_leds; i++)
    if (is31.leds[i].fwnode &&
    is31.leds[i].max_microamp < aggregated_led_microamp)
    aggregated_led_microamp = is31.leds[i].max_microamp;
    if (is31.cdef.is_3196or3199)
    regmap_write(is31.regmap, IS31FL3196_CONFIG2,
    is31fl3196_microamp_to_cs(dev, aggregated_led_microamp) |
    is31fl3196_db_to_gain(is31.audio_gain_db));
    else
    regmap_update_bits(is31.regmap, IS31FL3190_CURRENT, IS31FL3190_CURRENT_MASK,
    is31fl3190_microamp_to_cs(dev, aggregated_led_microamp) << IS31FL3190_CURRENT_SHIFT);
    for (i = 0; i < is31.cdef.num_leds; i++) {
    struct is31fl319x_led *led = &is31.leds[i];
    let mut init_data: led_init_data = {};
    if (!led.fwnode)
    continue;
    init_data.fwnode = led.fwnode;
    led.chip = is31;
    led.cdev.brightness_set_blocking = is31.cdef.brightness_set;
    err = devm_led_classdev_register_ext(&client.dev, &led.cdev, &init_data);
    if (err < 0)
    return err;
    }
    return 0;
    }
//
// i2c-core (and modalias) requires that id_table be properly filled,
// even though it is not used for DeviceTree based instantiation.
//
    static const struct i2c_device_id is31fl319x_id[] = {
    { .name = "is31fl3190" },
    { .name = "is31fl3191" },
    { .name = "is31fl3193" },
    { .name = "is31fl3196" },
    { .name = "is31fl3199" },
    { .name = "sn3190" },
    { .name = "sn3191" },
    { .name = "sn3193" },
    { .name = "sn3196" },
    { .name = "sn3199" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, is31fl319x_id);
    static struct i2c_driver is31fl319x_driver = {
    .driver   = {
    .name           = "leds-is31fl319x",
    .of_match_table = of_is31fl319x_match,
    },
    .probe = is31fl319x_probe,
    .id_table = is31fl319x_id,
    };
    module_i2c_driver(is31fl319x_driver);
    MODULE_AUTHOR("H. Nikolaus Schaller <hns@goldelico.com>");
    MODULE_AUTHOR("Andrey Utkin <andrey_utkin@fastmail.com>");
    MODULE_DESCRIPTION("IS31FL319X LED driver");
    MODULE_LICENSE("GPL v2");
