//! Automatically rewritten from C to Rust
//! Source: drivers/video/backlight/adp8870_bl.c
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
//
// Backlight driver for Analog Devices ADP8870 Backlight Devices
//
// Copyright 2009-2011 Analog Devices Inc.
//

// Macro flag: #define ADP8870_EXT_FEATURES
// Macro flag: #define ADP8870_USE_LEDS
pub const ADP8870_MFDVID: c_uint = 0x00  /* Manufacturer and device ID */;
pub const ADP8870_MDCR: c_uint = 0x01  /* Device mode and status */;
pub const ADP8870_INT_STAT: c_uint = 0x02  /* Interrupts status */;
pub const ADP8870_INT_EN: c_uint = 0x03  /* Interrupts enable */;
pub const ADP8870_CFGR: c_uint = 0x04  /* Configuration register */;
pub const ADP8870_BLSEL: c_uint = 0x05  /* Sink enable backlight or independent */;
pub const ADP8870_PWMLED: c_uint = 0x06  /* PWM Enable Selection Register */;
pub const ADP8870_BLOFF: c_uint = 0x07  /* Backlight off timeout */;
pub const ADP8870_BLDIM: c_uint = 0x08  /* Backlight dim timeout */;
pub const ADP8870_BLFR: c_uint = 0x09  /* Backlight fade in and out rates */;
pub const ADP8870_BLMX1: c_uint = 0x0A  /* Backlight (Brightness Level 1-daylight) maximum current */;
pub const ADP8870_BLDM1: c_uint = 0x0B  /* Backlight (Brightness Level 1-daylight) dim current */;
pub const ADP8870_BLMX2: c_uint = 0x0C  /* Backlight (Brightness Level 2-bright) maximum current */;
pub const ADP8870_BLDM2: c_uint = 0x0D  /* Backlight (Brightness Level 2-bright) dim current */;
pub const ADP8870_BLMX3: c_uint = 0x0E  /* Backlight (Brightness Level 3-office) maximum current */;
pub const ADP8870_BLDM3: c_uint = 0x0F  /* Backlight (Brightness Level 3-office) dim current */;
pub const ADP8870_BLMX4: c_uint = 0x10  /* Backlight (Brightness Level 4-indoor) maximum current */;
pub const ADP8870_BLDM4: c_uint = 0x11  /* Backlight (Brightness Level 4-indoor) dim current */;
pub const ADP8870_BLMX5: c_uint = 0x12  /* Backlight (Brightness Level 5-dark) maximum current */;
pub const ADP8870_BLDM5: c_uint = 0x13  /* Backlight (Brightness Level 5-dark) dim current */;
pub const ADP8870_ISCLAW: c_uint = 0x1A  /* Independent sink current fade law register */;
pub const ADP8870_ISCC: c_uint = 0x1B  /* Independent sink current control register */;
pub const ADP8870_ISCT1: c_uint = 0x1C  /* Independent Sink Current Timer Register LED[7:5] */;
pub const ADP8870_ISCT2: c_uint = 0x1D  /* Independent Sink Current Timer Register LED[4:1] */;
pub const ADP8870_ISCF: c_uint = 0x1E  /* Independent sink current fade register */;
pub const ADP8870_ISC1: c_uint = 0x1F  /* Independent Sink Current LED1 */;
pub const ADP8870_ISC2: c_uint = 0x20  /* Independent Sink Current LED2 */;
pub const ADP8870_ISC3: c_uint = 0x21  /* Independent Sink Current LED3 */;
pub const ADP8870_ISC4: c_uint = 0x22  /* Independent Sink Current LED4 */;
pub const ADP8870_ISC5: c_uint = 0x23  /* Independent Sink Current LED5 */;
pub const ADP8870_ISC6: c_uint = 0x24  /* Independent Sink Current LED6 */;
pub const ADP8870_ISC7: c_uint = 0x25  /* Independent Sink Current LED7 (Brightness Level 1-daylight) */;
pub const ADP8870_ISC7_L2: c_uint = 0x26  /* Independent Sink Current LED7 (Brightness Level 2-bright) */;
pub const ADP8870_ISC7_L3: c_uint = 0x27  /* Independent Sink Current LED7 (Brightness Level 3-office) */;
pub const ADP8870_ISC7_L4: c_uint = 0x28  /* Independent Sink Current LED7 (Brightness Level 4-indoor) */;
pub const ADP8870_ISC7_L5: c_uint = 0x29  /* Independent Sink Current LED7 (Brightness Level 5-dark) */;
pub const ADP8870_CMP_CTL: c_uint = 0x2D  /* ALS Comparator Control Register */;
pub const ADP8870_ALS1_EN: c_uint = 0x2E  /* Main ALS comparator level enable */;
pub const ADP8870_ALS2_EN: c_uint = 0x2F  /* Second ALS comparator level enable */;
pub const ADP8870_ALS1_STAT: c_uint = 0x30  /* Main ALS Comparator Status Register */;
pub const ADP8870_ALS2_STAT: c_uint = 0x31  /* Second ALS Comparator Status Register */;
pub const ADP8870_L2TRP: c_uint = 0x32  /* L2 comparator reference */;
pub const ADP8870_L2HYS: c_uint = 0x33  /* L2 hysteresis */;
pub const ADP8870_L3TRP: c_uint = 0x34  /* L3 comparator reference */;
pub const ADP8870_L3HYS: c_uint = 0x35  /* L3 hysteresis */;
pub const ADP8870_L4TRP: c_uint = 0x36  /* L4 comparator reference */;
pub const ADP8870_L4HYS: c_uint = 0x37  /* L4 hysteresis */;
pub const ADP8870_L5TRP: c_uint = 0x38  /* L5 comparator reference */;
pub const ADP8870_L5HYS: c_uint = 0x39  /* L5 hysteresis */;
pub const ADP8870_PH1LEVL: c_uint = 0x40  /* First phototransistor ambient light level-low byte register */;
pub const ADP8870_PH1LEVH: c_uint = 0x41  /* First phototransistor ambient light level-high byte register */;
pub const ADP8870_PH2LEVL: c_uint = 0x42  /* Second phototransistor ambient light level-low byte register */;
pub const ADP8870_PH2LEVH: c_uint = 0x43  /* Second phototransistor ambient light level-high byte register */;
pub const ADP8870_MANUFID: c_uint = 0x3  /* Analog Devices AD8870 Manufacturer and device ID */;

// MDCR Device mode and status

// ADP8870_ALS1_EN Main ALS comparator level enable

pub const CFGR_BLV_SHIFT: c_int = 3;
pub const CFGR_BLV_MASK: c_uint = 0x7;
pub const ADP8870_FLAG_LED_MASK: c_uint = 0xFF;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adp8870_bl {
    pub client: *mut i2c_client,
    pub bl: *mut backlight_device,
    pub led: *mut adp8870_led,
    pub pdata: *mut adp8870_backlight_platform_data,
    pub lock: mutex,
    pub cached_daylight_max: c_ulong,
    pub id: c_int,
    pub revid: c_int,
    pub current_brightness: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adp8870_led {
    pub cdev: led_classdev,
    pub work: work_struct,
    pub client: *mut i2c_client,
    pub new_brightness: enum led_brightness,
    pub id: c_int,
    pub flags: c_int,
}

#[no_mangle]
unsafe extern "C" fn adp8870_read(client: *mut i2c_client, reg: c_int, val: *mut u8) -> c_int {
    static int adp8870_read(struct i2c_client *client, int reg, uint8_t *val)
    {
    int ret;
    ret = i2c_smbus_read_byte_data(client, reg);
    if (ret < 0) {
    dev_err(&client.dev, "failed reading at 0x%02x\n", reg);
    return ret;
    }
// val = ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adp8870_write(client: *mut i2c_client, reg: u8, val: u8) -> c_int {
    static int adp8870_write(struct i2c_client *client, u8 reg, u8 val)
    {
    let mut ret: c_int = i2c_smbus_write_byte_data(client, reg, val);
    if (ret)
    dev_err(&client.dev, "failed to write\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn adp8870_set_bits(client: *mut i2c_client, reg: c_int, bit_mask: u8) -> c_int {
    static int adp8870_set_bits(struct i2c_client *client, int reg, uint8_t bit_mask)
    {
    struct adp8870_bl *data = i2c_get_clientdata(client);
    uint8_t reg_val;
    int ret;
    mutex_lock(&data.lock);
    ret = adp8870_read(client, reg, &reg_val);
    if (!ret && ((reg_val & bit_mask) != bit_mask)) {
    reg_val |= bit_mask;
    ret = adp8870_write(client, reg, reg_val);
    }
    mutex_unlock(&data.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn adp8870_clr_bits(client: *mut i2c_client, reg: c_int, bit_mask: u8) -> c_int {
    static int adp8870_clr_bits(struct i2c_client *client, int reg, uint8_t bit_mask)
    {
    struct adp8870_bl *data = i2c_get_clientdata(client);
    uint8_t reg_val;
    int ret;
    mutex_lock(&data.lock);
    ret = adp8870_read(client, reg, &reg_val);
    if (!ret && (reg_val & bit_mask)) {
    reg_val &= ~bit_mask;
    ret = adp8870_write(client, reg, reg_val);
    }
    mutex_unlock(&data.lock);
    return ret;
    }
//
// Independent sink / LED
//

#[no_mangle]
unsafe extern "C" fn adp8870_led_work(work: *mut work_struct) {
    static void adp8870_led_work(struct work_struct *work)
    {
    struct adp8870_led *led = container_of(work, struct adp8870_led, work);
    adp8870_write(led.client, ADP8870_ISC1 + led.id - 1,
    led.new_brightness >> 1);
    }
    static void adp8870_led_set(struct led_classdev *led_cdev,
    enum led_brightness value)
    {
    struct adp8870_led *led;
    led = container_of(led_cdev, struct adp8870_led, cdev);
    led.new_brightness = value;
//
// Use workqueue for IO since I2C operations can sleep.
//
    schedule_work(&led.work);
    }
#[no_mangle]
unsafe extern "C" fn adp8870_led_setup(led: *mut adp8870_led) -> c_int {
    static int adp8870_led_setup(struct adp8870_led *led)
    {
    struct i2c_client *client = led.client;
    let mut ret: c_int = 0;
    ret = adp8870_write(client, ADP8870_ISC1 + led.id - 1, 0);
    if (ret)
    return ret;
    ret = adp8870_set_bits(client, ADP8870_ISCC, 1 << (led.id - 1));
    if (ret)
    return ret;
    if (led.id > 4)
    ret = adp8870_set_bits(client, ADP8870_ISCT1,
    (led.flags & 0x3) << ((led.id - 5) * 2));
    else
    ret = adp8870_set_bits(client, ADP8870_ISCT2,
    (led.flags & 0x3) << ((led.id - 1) * 2));
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn adp8870_led_probe(client: *mut i2c_client) -> c_int {
    static int adp8870_led_probe(struct i2c_client *client)
    {
    struct adp8870_backlight_platform_data *pdata =
    dev_get_platdata(&client.dev);
    struct adp8870_bl *data = i2c_get_clientdata(client);
    struct adp8870_led *led, *led_dat;
    struct led_info *cur_led;
    int ret, i;
    led = devm_kcalloc(&client.dev, pdata.num_leds, sizeof(*led),
    GFP_KERNEL);
    if (led == core::ptr::null_mut())
    return -ENOMEM;
    ret = adp8870_write(client, ADP8870_ISCLAW, pdata.led_fade_law);
    if (ret)
    return ret;
    ret = adp8870_write(client, ADP8870_ISCT1,
    (pdata.led_on_time & 0x3) << 6);
    if (ret)
    return ret;
    ret = adp8870_write(client, ADP8870_ISCF,
    FADE_VAL(pdata.led_fade_in, pdata.led_fade_out));
    if (ret)
    return ret;
    for (i = 0; i < pdata.num_leds; ++i) {
    cur_led = &pdata.leds[i];
    led_dat = &led[i];
    led_dat.id = cur_led.flags & ADP8870_FLAG_LED_MASK;
    if (led_dat.id > 7 || led_dat.id < 1) {
    dev_err(&client.dev, "Invalid LED ID %d\n",
    led_dat.id);
    ret = -EINVAL;
    goto err;
    }
    if (pdata.bl_led_assign & (1 << (led_dat.id - 1))) {
    dev_err(&client.dev, "LED %d used by Backlight\n",
    led_dat.id);
    ret = -EBUSY;
    goto err;
    }
    led_dat.cdev.name = cur_led.name;
    led_dat.cdev.default_trigger = cur_led.default_trigger;
    led_dat.cdev.brightness_set = adp8870_led_set;
    led_dat.cdev.brightness = LED_OFF;
    led_dat.flags = cur_led.flags >> FLAG_OFFT_SHIFT;
    led_dat.client = client;
    led_dat.new_brightness = LED_OFF;
    INIT_WORK(&led_dat.work, adp8870_led_work);
    ret = led_classdev_register(&client.dev, &led_dat.cdev);
    if (ret) {
    dev_err(&client.dev, "failed to register LED %d\n",
    led_dat.id);
    goto err;
    }
    ret = adp8870_led_setup(led_dat);
    if (ret) {
    dev_err(&client.dev, "failed to write\n");
    i++;
    goto err;
    }
    }
    data.led = led;
    return 0;
    err:
    for (i = i - 1; i >= 0; --i) {
    led_classdev_unregister(&led[i].cdev);
    cancel_work_sync(&led[i].work);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn adp8870_led_remove(client: *mut i2c_client) -> c_int {
    static int adp8870_led_remove(struct i2c_client *client)
    {
    struct adp8870_backlight_platform_data *pdata =
    dev_get_platdata(&client.dev);
    struct adp8870_bl *data = i2c_get_clientdata(client);
    int i;
    for (i = 0; i < pdata.num_leds; i++) {
    led_classdev_unregister(&data.led[i].cdev);
    cancel_work_sync(&data.led[i].work);
    }
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn adp8870_led_probe(client: *mut i2c_client) -> c_int {
    static int adp8870_led_probe(struct i2c_client *client)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adp8870_led_remove(client: *mut i2c_client) -> c_int {
    static int adp8870_led_remove(struct i2c_client *client)
    {
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn adp8870_bl_set(bl: *mut backlight_device, brightness: c_int) -> c_int {
    static int adp8870_bl_set(struct backlight_device *bl, int brightness)
    {
    struct adp8870_bl *data = bl_get_data(bl);
    struct i2c_client *client = data.client;
    let mut ret: c_int = 0;
    if (data.pdata.en_ambl_sens) {
    if ((brightness > 0) && (brightness < ADP8870_MAX_BRIGHTNESS)) {
// Disable Ambient Light auto adjust
    ret = adp8870_clr_bits(client, ADP8870_MDCR,
    CMP_AUTOEN);
    if (ret)
    return ret;
    ret = adp8870_write(client, ADP8870_BLMX1, brightness);
    if (ret)
    return ret;
    } else {
//
// MAX_BRIGHTNESS -> Enable Ambient Light auto adjust
// restore daylight l1 sysfs brightness
//
    ret = adp8870_write(client, ADP8870_BLMX1,
    data.cached_daylight_max);
    if (ret)
    return ret;
    ret = adp8870_set_bits(client, ADP8870_MDCR,
    CMP_AUTOEN);
    if (ret)
    return ret;
    }
    } else {
    ret = adp8870_write(client, ADP8870_BLMX1, brightness);
    if (ret)
    return ret;
    }
    if (data.current_brightness && brightness == 0)
    ret = adp8870_set_bits(client,
    ADP8870_MDCR, DIM_EN);
#[no_mangle]
pub unsafe extern "C" fn if(brightness: data->current_brightness == 0 &&) -> else {
    else if (data.current_brightness == 0 && brightness)
    ret = adp8870_clr_bits(client,
    ADP8870_MDCR, DIM_EN);
    if (!ret)
    data.current_brightness = brightness;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn adp8870_bl_update_status(bl: *mut backlight_device) -> c_int {
    static int adp8870_bl_update_status(struct backlight_device *bl)
    {
    return adp8870_bl_set(bl, backlight_get_brightness(bl));
    }
#[no_mangle]
unsafe extern "C" fn adp8870_bl_get_brightness(bl: *mut backlight_device) -> c_int {
    static int adp8870_bl_get_brightness(struct backlight_device *bl)
    {
    struct adp8870_bl *data = bl_get_data(bl);
    return data.current_brightness;
    }
    static const struct backlight_ops adp8870_bl_ops = {
    .update_status	= adp8870_bl_update_status,
    .get_brightness	= adp8870_bl_get_brightness,
    };
#[no_mangle]
unsafe extern "C" fn adp8870_bl_setup(bl: *mut backlight_device) -> c_int {
    static int adp8870_bl_setup(struct backlight_device *bl)
    {
    struct adp8870_bl *data = bl_get_data(bl);
    struct i2c_client *client = data.client;
    struct adp8870_backlight_platform_data *pdata = data.pdata;
    let mut ret: c_int = 0;
    ret = adp8870_write(client, ADP8870_BLSEL, ~pdata.bl_led_assign);
    if (ret)
    return ret;
    ret = adp8870_write(client, ADP8870_PWMLED, pdata.pwm_assign);
    if (ret)
    return ret;
    ret = adp8870_write(client, ADP8870_BLMX1, pdata.l1_daylight_max);
    if (ret)
    return ret;
    ret = adp8870_write(client, ADP8870_BLDM1, pdata.l1_daylight_dim);
    if (ret)
    return ret;
    if (pdata.en_ambl_sens) {
    data.cached_daylight_max = pdata.l1_daylight_max;
    ret = adp8870_write(client, ADP8870_BLMX2,
    pdata.l2_bright_max);
    if (ret)
    return ret;
    ret = adp8870_write(client, ADP8870_BLDM2,
    pdata.l2_bright_dim);
    if (ret)
    return ret;
    ret = adp8870_write(client, ADP8870_BLMX3,
    pdata.l3_office_max);
    if (ret)
    return ret;
    ret = adp8870_write(client, ADP8870_BLDM3,
    pdata.l3_office_dim);
    if (ret)
    return ret;
    ret = adp8870_write(client, ADP8870_BLMX4,
    pdata.l4_indoor_max);
    if (ret)
    return ret;
    ret = adp8870_write(client, ADP8870_BLDM4,
    pdata.l4_indor_dim);
    if (ret)
    return ret;
    ret = adp8870_write(client, ADP8870_BLMX5,
    pdata.l5_dark_max);
    if (ret)
    return ret;
    ret = adp8870_write(client, ADP8870_BLDM5,
    pdata.l5_dark_dim);
    if (ret)
    return ret;
    ret = adp8870_write(client, ADP8870_L2TRP, pdata.l2_trip);
    if (ret)
    return ret;
    ret = adp8870_write(client, ADP8870_L2HYS, pdata.l2_hyst);
    if (ret)
    return ret;
    ret = adp8870_write(client, ADP8870_L3TRP, pdata.l3_trip);
    if (ret)
    return ret;
    ret = adp8870_write(client, ADP8870_L3HYS, pdata.l3_hyst);
    if (ret)
    return ret;
    ret = adp8870_write(client, ADP8870_L4TRP, pdata.l4_trip);
    if (ret)
    return ret;
    ret = adp8870_write(client, ADP8870_L4HYS, pdata.l4_hyst);
    if (ret)
    return ret;
    ret = adp8870_write(client, ADP8870_L5TRP, pdata.l5_trip);
    if (ret)
    return ret;
    ret = adp8870_write(client, ADP8870_L5HYS, pdata.l5_hyst);
    if (ret)
    return ret;
    ret = adp8870_write(client, ADP8870_ALS1_EN, L5_EN | L4_EN |
    L3_EN | L2_EN);
    if (ret)
    return ret;
    ret = adp8870_write(client, ADP8870_CMP_CTL,
    ALS_CMPR_CFG_VAL(pdata.abml_filt));
    if (ret)
    return ret;
    }
    ret = adp8870_write(client, ADP8870_CFGR,
    BL_CFGR_VAL(pdata.bl_fade_law, 0));
    if (ret)
    return ret;
    ret = adp8870_write(client, ADP8870_BLFR, FADE_VAL(pdata.bl_fade_in,
    pdata.bl_fade_out));
    if (ret)
    return ret;
//
// ADP8870 Rev0 requires GDWN_DIS bit set
//
    ret = adp8870_set_bits(client, ADP8870_MDCR, BLEN | DIM_EN | NSTBY |
    (data.revid == 0 ? GDWN_DIS : 0));
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn adp8870_show(dev: *mut device, buf: *mut c_char, reg: c_int) -> isize {
    static ssize_t adp8870_show(struct device *dev, char *buf, int reg)
    {
    struct adp8870_bl *data = dev_get_drvdata(dev);
    int error;
    uint8_t reg_val;
    mutex_lock(&data.lock);
    error = adp8870_read(data.client, reg, &reg_val);
    mutex_unlock(&data.lock);
    if (error < 0)
    return error;
    return sprintf(buf, "%u\n", reg_val);
    }
    static ssize_t adp8870_store(struct device *dev, const char *buf,
    size_t count, int reg)
    {
    struct adp8870_bl *data = dev_get_drvdata(dev);
    unsigned long val;
    int ret;
    ret = kstrtoul(buf, 10, &val);
    if (ret)
    return ret;
    mutex_lock(&data.lock);
    adp8870_write(data.client, reg, val);
    mutex_unlock(&data.lock);
    return count;
    }
    static ssize_t adp8870_bl_l5_dark_max_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    return adp8870_show(dev, buf, ADP8870_BLMX5);
    }
    static ssize_t adp8870_bl_l5_dark_max_store(struct device *dev,
    struct device_attribute *attr, const char *buf, size_t count)
    {
    return adp8870_store(dev, buf, count, ADP8870_BLMX5);
    }
    static DEVICE_ATTR(l5_dark_max, 0664, adp8870_bl_l5_dark_max_show,
    adp8870_bl_l5_dark_max_store);
    static ssize_t adp8870_bl_l4_indoor_max_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    return adp8870_show(dev, buf, ADP8870_BLMX4);
    }
    static ssize_t adp8870_bl_l4_indoor_max_store(struct device *dev,
    struct device_attribute *attr, const char *buf, size_t count)
    {
    return adp8870_store(dev, buf, count, ADP8870_BLMX4);
    }
    static DEVICE_ATTR(l4_indoor_max, 0664, adp8870_bl_l4_indoor_max_show,
    adp8870_bl_l4_indoor_max_store);
    static ssize_t adp8870_bl_l3_office_max_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    return adp8870_show(dev, buf, ADP8870_BLMX3);
    }
    static ssize_t adp8870_bl_l3_office_max_store(struct device *dev,
    struct device_attribute *attr, const char *buf, size_t count)
    {
    return adp8870_store(dev, buf, count, ADP8870_BLMX3);
    }
    static DEVICE_ATTR(l3_office_max, 0664, adp8870_bl_l3_office_max_show,
    adp8870_bl_l3_office_max_store);
    static ssize_t adp8870_bl_l2_bright_max_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    return adp8870_show(dev, buf, ADP8870_BLMX2);
    }
    static ssize_t adp8870_bl_l2_bright_max_store(struct device *dev,
    struct device_attribute *attr, const char *buf, size_t count)
    {
    return adp8870_store(dev, buf, count, ADP8870_BLMX2);
    }
    static DEVICE_ATTR(l2_bright_max, 0664, adp8870_bl_l2_bright_max_show,
    adp8870_bl_l2_bright_max_store);
    static ssize_t adp8870_bl_l1_daylight_max_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    return adp8870_show(dev, buf, ADP8870_BLMX1);
    }
    static ssize_t adp8870_bl_l1_daylight_max_store(struct device *dev,
    struct device_attribute *attr, const char *buf, size_t count)
    {
    struct adp8870_bl *data = dev_get_drvdata(dev);
    let mut ret: c_int = kstrtoul(buf, 10, &data.cached_daylight_max);
    if (ret)
    return ret;
    return adp8870_store(dev, buf, count, ADP8870_BLMX1);
    }
    static DEVICE_ATTR(l1_daylight_max, 0664, adp8870_bl_l1_daylight_max_show,
    adp8870_bl_l1_daylight_max_store);
    static ssize_t adp8870_bl_l5_dark_dim_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    return adp8870_show(dev, buf, ADP8870_BLDM5);
    }
    static ssize_t adp8870_bl_l5_dark_dim_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    return adp8870_store(dev, buf, count, ADP8870_BLDM5);
    }
    static DEVICE_ATTR(l5_dark_dim, 0664, adp8870_bl_l5_dark_dim_show,
    adp8870_bl_l5_dark_dim_store);
    static ssize_t adp8870_bl_l4_indoor_dim_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    return adp8870_show(dev, buf, ADP8870_BLDM4);
    }
    static ssize_t adp8870_bl_l4_indoor_dim_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    return adp8870_store(dev, buf, count, ADP8870_BLDM4);
    }
    static DEVICE_ATTR(l4_indoor_dim, 0664, adp8870_bl_l4_indoor_dim_show,
    adp8870_bl_l4_indoor_dim_store);
    static ssize_t adp8870_bl_l3_office_dim_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    return adp8870_show(dev, buf, ADP8870_BLDM3);
    }
    static ssize_t adp8870_bl_l3_office_dim_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    return adp8870_store(dev, buf, count, ADP8870_BLDM3);
    }
    static DEVICE_ATTR(l3_office_dim, 0664, adp8870_bl_l3_office_dim_show,
    adp8870_bl_l3_office_dim_store);
    static ssize_t adp8870_bl_l2_bright_dim_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    return adp8870_show(dev, buf, ADP8870_BLDM2);
    }
    static ssize_t adp8870_bl_l2_bright_dim_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    return adp8870_store(dev, buf, count, ADP8870_BLDM2);
    }
    static DEVICE_ATTR(l2_bright_dim, 0664, adp8870_bl_l2_bright_dim_show,
    adp8870_bl_l2_bright_dim_store);
    static ssize_t adp8870_bl_l1_daylight_dim_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    return adp8870_show(dev, buf, ADP8870_BLDM1);
    }
    static ssize_t adp8870_bl_l1_daylight_dim_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    return adp8870_store(dev, buf, count, ADP8870_BLDM1);
    }
    static DEVICE_ATTR(l1_daylight_dim, 0664, adp8870_bl_l1_daylight_dim_show,
    adp8870_bl_l1_daylight_dim_store);

    static ssize_t adp8870_bl_ambient_light_level_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct adp8870_bl *data = dev_get_drvdata(dev);
    int error;
    uint8_t reg_val;
    uint16_t ret_val;
    mutex_lock(&data.lock);
    error = adp8870_read(data.client, ADP8870_PH1LEVL, &reg_val);
    if (error < 0) {
    mutex_unlock(&data.lock);
    return error;
    }
    ret_val = reg_val;
    error = adp8870_read(data.client, ADP8870_PH1LEVH, &reg_val);
    mutex_unlock(&data.lock);
    if (error < 0)
    return error;
// Return 13-bit conversion value for the first light sensor
    ret_val += (reg_val & 0x1F) << 8;
    return sprintf(buf, "%u\n", ret_val);
    }
    static DEVICE_ATTR(ambient_light_level, 0444,
    adp8870_bl_ambient_light_level_show, core::ptr::null_mut());
    static ssize_t adp8870_bl_ambient_light_zone_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct adp8870_bl *data = dev_get_drvdata(dev);
    int error;
    uint8_t reg_val;
    mutex_lock(&data.lock);
    error = adp8870_read(data.client, ADP8870_CFGR, &reg_val);
    mutex_unlock(&data.lock);
    if (error < 0)
    return error;
    return sprintf(buf, "%u\n",
    ((reg_val >> CFGR_BLV_SHIFT) & CFGR_BLV_MASK) + 1);
    }
    static ssize_t adp8870_bl_ambient_light_zone_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t count)
    {
    struct adp8870_bl *data = dev_get_drvdata(dev);
    unsigned long val;
    uint8_t reg_val;
    int ret;
    ret = kstrtoul(buf, 10, &val);
    if (ret)
    return ret;
    if (val == 0) {
// Enable automatic ambient light sensing
    adp8870_set_bits(data.client, ADP8870_MDCR, CMP_AUTOEN);
    } else if ((val > 0) && (val < 6)) {
// Disable automatic ambient light sensing
    adp8870_clr_bits(data.client, ADP8870_MDCR, CMP_AUTOEN);
// Set user supplied ambient light zone
    mutex_lock(&data.lock);
    ret = adp8870_read(data.client, ADP8870_CFGR, &reg_val);
    if (!ret) {
    reg_val &= ~(CFGR_BLV_MASK << CFGR_BLV_SHIFT);
    reg_val |= (val - 1) << CFGR_BLV_SHIFT;
    adp8870_write(data.client, ADP8870_CFGR, reg_val);
    }
    mutex_unlock(&data.lock);
    }
    return count;
    }
    static DEVICE_ATTR(ambient_light_zone, 0664,
    adp8870_bl_ambient_light_zone_show,
    adp8870_bl_ambient_light_zone_store);

    static struct attribute *adp8870_bl_attributes[] = {
    &dev_attr_l5_dark_max.attr,
    &dev_attr_l5_dark_dim.attr,
    &dev_attr_l4_indoor_max.attr,
    &dev_attr_l4_indoor_dim.attr,
    &dev_attr_l3_office_max.attr,
    &dev_attr_l3_office_dim.attr,
    &dev_attr_l2_bright_max.attr,
    &dev_attr_l2_bright_dim.attr,
    &dev_attr_l1_daylight_max.attr,
    &dev_attr_l1_daylight_dim.attr,

    &dev_attr_ambient_light_level.attr,
    &dev_attr_ambient_light_zone.attr,

    core::ptr::null_mut()
    };
    static const struct attribute_group adp8870_bl_attr_group = {
    .attrs = adp8870_bl_attributes,
    };
#[no_mangle]
unsafe extern "C" fn adp8870_probe(client: *mut i2c_client) -> c_int {
    static int adp8870_probe(struct i2c_client *client)
    {
    const struct i2c_device_id *id = i2c_client_get_device_id(client);
    struct backlight_properties props;
    struct backlight_device *bl;
    struct adp8870_bl *data;
    struct adp8870_backlight_platform_data *pdata =
    dev_get_platdata(&client.dev);
    uint8_t reg_val;
    int ret;
    if (!i2c_check_functionality(client.adapter,
    I2C_FUNC_SMBUS_BYTE_DATA)) {
    dev_err(&client.dev, "SMBUS Byte Data not Supported\n");
    return -EIO;
    }
    if (!pdata) {
    dev_err(&client.dev, "no platform data?\n");
    return -EINVAL;
    }
    ret = adp8870_read(client, ADP8870_MFDVID, &reg_val);
    if (ret < 0)
    return -EIO;
    if (ADP8870_MANID(reg_val) != ADP8870_MANUFID) {
    dev_err(&client.dev, "failed to probe\n");
    return -ENODEV;
    }
    data = devm_kzalloc(&client.dev, sizeof(*data), GFP_KERNEL);
    if (data == core::ptr::null_mut())
    return -ENOMEM;
    data.revid = ADP8870_DEVID(reg_val);
    data.client = client;
    data.pdata = pdata;
    data.id = id.driver_data;
    data.current_brightness = 0;
    i2c_set_clientdata(client, data);
    mutex_init(&data.lock);
    memset(&props, 0, sizeof(props));
    props.type = BACKLIGHT_RAW;
    props.max_brightness = props.brightness = ADP8870_MAX_BRIGHTNESS;
    bl = devm_backlight_device_register(&client.dev,
    dev_driver_string(&client.dev),
    &client.dev, data, &adp8870_bl_ops, &props);
    if (IS_ERR(bl)) {
    dev_err(&client.dev, "failed to register backlight\n");
    return PTR_ERR(bl);
    }
    data.bl = bl;
    if (pdata.en_ambl_sens) {
    ret = sysfs_create_group(&bl.dev.kobj,
    &adp8870_bl_attr_group);
    if (ret) {
    dev_err(&client.dev, "failed to register sysfs\n");
    return ret;
    }
    }
    ret = adp8870_bl_setup(bl);
    if (ret) {
    ret = -EIO;
    goto out;
    }
    backlight_update_status(bl);
    dev_info(&client.dev, "Rev.%d Backlight\n", data.revid);
    if (pdata.num_leds)
    adp8870_led_probe(client);
    return 0;
    out:
    if (data.pdata.en_ambl_sens)
    sysfs_remove_group(&data.bl.dev.kobj,
    &adp8870_bl_attr_group);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn adp8870_remove(client: *mut i2c_client) {
    static void adp8870_remove(struct i2c_client *client)
    {
    struct adp8870_bl *data = i2c_get_clientdata(client);
    adp8870_clr_bits(client, ADP8870_MDCR, NSTBY);
    if (data.led)
    adp8870_led_remove(client);
    if (data.pdata.en_ambl_sens)
    sysfs_remove_group(&data.bl.dev.kobj,
    &adp8870_bl_attr_group);
    }

#[no_mangle]
unsafe extern "C" fn adp8870_i2c_suspend(dev: *mut device) -> c_int {
    static int adp8870_i2c_suspend(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    adp8870_clr_bits(client, ADP8870_MDCR, NSTBY);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adp8870_i2c_resume(dev: *mut device) -> c_int {
    static int adp8870_i2c_resume(struct device *dev)
    {
    struct i2c_client *client = to_i2c_client(dev);
    adp8870_set_bits(client, ADP8870_MDCR, NSTBY | BLEN);
    return 0;
    }

    static SIMPLE_DEV_PM_OPS(adp8870_i2c_pm_ops, adp8870_i2c_suspend,
    adp8870_i2c_resume);
    static const struct i2c_device_id adp8870_id[] = {
    { .name = "adp8870" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, adp8870_id);
    static struct i2c_driver adp8870_driver = {
    .driver = {
    .name	= KBUILD_MODNAME,
    .pm	= &adp8870_i2c_pm_ops,
    },
    .probe = adp8870_probe,
    .remove = adp8870_remove,
    .id_table = adp8870_id,
    };
    module_i2c_driver(adp8870_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_AUTHOR("Michael Hennerich <michael.hennerich@analog.com>");
    MODULE_DESCRIPTION("ADP8870 Backlight driver");
