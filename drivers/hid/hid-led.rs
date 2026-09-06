//! Automatically rewritten from C to Rust
//! Source: drivers/hid/hid-led.c
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
// Simple USB RGB LED driver
//
// Copyright 2016 Heiner Kallweit <hkallweit1@gmail.com>
// Based on drivers/hid/hid-thingm.c and
// drivers/usb/misc/usbled.c
//

    enum hidled_report_type {
    RAW_REQUEST,
    OUTPUT_REPORT
    };
    enum hidled_type {
    RISO_KAGAKU,
    DREAM_CHEEKY,
    THINGM,
    DELCOM,
    LUXAFOR,
    };
    static unsigned const char riso_kagaku_tbl[] = {
// R+2G+4B -> riso kagaku color index
    [0] = 0, /* black   */
    [1] = 2, /* red     */
    [2] = 1, /* green   */
    [3] = 5, /* yellow  */
    [4] = 3, /* blue    */
    [5] = 6, /* magenta */
    [6] = 4, /* cyan    */
    [7] = 7  /* white   */
    };

    union delcom_packet {
    __u8 data[8];
    struct {
    __u8 major_cmd;
    __u8 minor_cmd;
    __u8 data_lsb;
    __u8 data_msb;
    } tx;
    struct {
    __u8 cmd;
    } rx;
    struct {
    __le16 family_code;
    __le16 security_code;
    __u8 fw_version;
    } fw;
    };
pub const DELCOM_GREEN_LED: c_int = 0;
pub const DELCOM_RED_LED: c_int = 1;
pub const DELCOM_BLUE_LED: c_int = 2;
    struct hidled_device;
    struct hidled_rgb;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hidled_config {
    pub type: enum hidled_type,
    pub name: *const c_char,
    pub short_name: *const c_char,
    pub max_brightness: enum led_brightness,
    pub num_leds: c_int,
    pub report_size: usize,
    pub report_type: enum hidled_report_type,
    pub ldev): *mut *mut int (init)(struct hidled_device,
    pub br): *mut *mut *mut int (write)(struct led_classdev cdev, enum led_brightness,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hidled_led {
    pub cdev: led_classdev,
    pub rgb: *mut hidled_rgb,
    pub name: [c_char; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hidled_rgb {
    pub ldev: *mut hidled_device,
    pub red: hidled_led,
    pub green: hidled_led,
    pub blue: hidled_led,
    pub num: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hidled_device {
    pub config: *const hidled_config,
    pub hdev: *mut hid_device,
    pub rgb: *mut hidled_rgb,
    pub buf: *mut u8,
    pub lock: mutex,
}

pub const MAX_REPORT_SIZE: c_int = 16;

    static bool riso_kagaku_switch_green_blue;
    module_param(riso_kagaku_switch_green_blue, bool, S_IRUGO | S_IWUSR);
    MODULE_PARM_DESC(riso_kagaku_switch_green_blue,
    "switch green and blue RGB component for Riso Kagaku devices");
#[no_mangle]
unsafe extern "C" fn hidled_send(ldev: *mut hidled_device, buf: *mut __u8) -> c_int {
    static int hidled_send(struct hidled_device *ldev, __u8 *buf)
    {
    int ret;
    mutex_lock(&ldev.lock);
//
// buffer provided to hid_hw_raw_request must not be on the stack
// and must not be part of a data structure
//
    memcpy(ldev.buf, buf, ldev.config.report_size);
    if (ldev.config.report_type == RAW_REQUEST)
    ret = hid_hw_raw_request(ldev.hdev, buf[0], ldev.buf,
    ldev.config.report_size,
    HID_FEATURE_REPORT,
    HID_REQ_SET_REPORT);
#[no_mangle]
pub unsafe extern "C" fn if(OUTPUT_REPORT: ldev->config->report_type ==) -> else {
    else if (ldev.config.report_type == OUTPUT_REPORT)
    ret = hid_hw_output_report(ldev.hdev, ldev.buf,
    ldev.config.report_size);
    else
    ret = -EINVAL;
    mutex_unlock(&ldev.lock);
    if (ret < 0)
    return ret;
    let mut ret: return = = ldev.config.report_size ? 0 : -EMSGSIZE;
    }
// reading data is supported for report type RAW_REQUEST only
#[no_mangle]
unsafe extern "C" fn hidled_recv(ldev: *mut hidled_device, buf: *mut __u8) -> c_int {
    static int hidled_recv(struct hidled_device *ldev, __u8 *buf)
    {
    int ret;
    if (ldev.config.report_type != RAW_REQUEST)
    return -EINVAL;
    mutex_lock(&ldev.lock);
    memcpy(ldev.buf, buf, ldev.config.report_size);
    ret = hid_hw_raw_request(ldev.hdev, buf[0], ldev.buf,
    ldev.config.report_size,
    HID_FEATURE_REPORT,
    HID_REQ_SET_REPORT);
    if (ret < 0)
    goto err;
    ret = hid_hw_raw_request(ldev.hdev, buf[0], ldev.buf,
    ldev.config.report_size,
    HID_FEATURE_REPORT,
    HID_REQ_GET_REPORT);
    memcpy(buf, ldev.buf, ldev.config.report_size);
    err:
    mutex_unlock(&ldev.lock);
    return ret < 0 ? ret : 0;
    }
#[no_mangle]
unsafe extern "C" fn riso_kagaku_index(rgb: *mut hidled_rgb) -> u8 {
    static u8 riso_kagaku_index(struct hidled_rgb *rgb)
    {
    enum led_brightness r, g, b;
    r = rgb.red.cdev.brightness;
    g = rgb.green.cdev.brightness;
    b = rgb.blue.cdev.brightness;
    if (riso_kagaku_switch_green_blue)
    return RISO_KAGAKU_IX(r, b, g);
    else
    return RISO_KAGAKU_IX(r, g, b);
    }
#[no_mangle]
unsafe extern "C" fn riso_kagaku_write(cdev: *mut led_classdev, br: enum led_brightness) -> c_int {
    static int riso_kagaku_write(struct led_classdev *cdev, enum led_brightness br)
    {
    struct hidled_led *led = to_hidled_led(cdev);
    struct hidled_rgb *rgb = led.rgb;
    __u8 buf[MAX_REPORT_SIZE] = {};
    buf[1] = riso_kagaku_index(rgb);
    return hidled_send(rgb.ldev, buf);
    }
#[no_mangle]
unsafe extern "C" fn dream_cheeky_write(cdev: *mut led_classdev, br: enum led_brightness) -> c_int {
    static int dream_cheeky_write(struct led_classdev *cdev, enum led_brightness br)
    {
    struct hidled_led *led = to_hidled_led(cdev);
    struct hidled_rgb *rgb = led.rgb;
    __u8 buf[MAX_REPORT_SIZE] = {};
    buf[1] = rgb.red.cdev.brightness;
    buf[2] = rgb.green.cdev.brightness;
    buf[3] = rgb.blue.cdev.brightness;
    buf[7] = 0x1a;
    buf[8] = 0x05;
    return hidled_send(rgb.ldev, buf);
    }
#[no_mangle]
unsafe extern "C" fn dream_cheeky_init(ldev: *mut hidled_device) -> c_int {
    static int dream_cheeky_init(struct hidled_device *ldev)
    {
    __u8 buf[MAX_REPORT_SIZE] = {};
// Dream Cheeky magic
    buf[1] = 0x1f;
    buf[2] = 0x02;
    buf[4] = 0x5f;
    buf[7] = 0x1a;
    buf[8] = 0x03;
    return hidled_send(ldev, buf);
    }
    static int _thingm_write(struct led_classdev *cdev, enum led_brightness br,
    u8 offset)
    {
    struct hidled_led *led = to_hidled_led(cdev);
    __u8 buf[MAX_REPORT_SIZE] = { 1, 'c' };
    buf[2] = led.rgb.red.cdev.brightness;
    buf[3] = led.rgb.green.cdev.brightness;
    buf[4] = led.rgb.blue.cdev.brightness;
    buf[7] = led.rgb.num + offset;
    return hidled_send(led.rgb.ldev, buf);
    }
#[no_mangle]
unsafe extern "C" fn thingm_write_v1(cdev: *mut led_classdev, br: enum led_brightness) -> c_int {
    static int thingm_write_v1(struct led_classdev *cdev, enum led_brightness br)
    {
    return _thingm_write(cdev, br, 0);
    }
#[no_mangle]
unsafe extern "C" fn thingm_write(cdev: *mut led_classdev, br: enum led_brightness) -> c_int {
    static int thingm_write(struct led_classdev *cdev, enum led_brightness br)
    {
    return _thingm_write(cdev, br, 1);
    }
    static const struct hidled_config hidled_config_thingm_v1 = {
    .name = "ThingM blink(1) v1",
    .short_name = "thingm",
    .max_brightness = 255,
    .num_leds = 1,
    .report_size = 9,
    .report_type = RAW_REQUEST,
    .write = thingm_write_v1,
    };
#[no_mangle]
unsafe extern "C" fn thingm_init(ldev: *mut hidled_device) -> c_int {
    static int thingm_init(struct hidled_device *ldev)
    {
    __u8 buf[MAX_REPORT_SIZE] = { 1, 'v' };
    int ret;
    ret = hidled_recv(ldev, buf);
    if (ret)
    return ret;
// Check for firmware major version 1
    if (buf[3] == '1')
    ldev.config = &hidled_config_thingm_v1;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn delcom_get_lednum(led: *const hidled_led) -> c_int {
    static inline int delcom_get_lednum(const struct hidled_led *led)
    {
    if (led == &led.rgb.red)
    return DELCOM_RED_LED;
#[no_mangle]
pub unsafe extern "C" fn if(&led->rgb->green: led ==) -> else {
    else if (led == &led.rgb.green)
    return DELCOM_GREEN_LED;
    else
    return DELCOM_BLUE_LED;
    }
#[no_mangle]
unsafe extern "C" fn delcom_enable_led(led: *mut hidled_led) -> c_int {
    static int delcom_enable_led(struct hidled_led *led)
    {
    let mut dp: union delcom_packet = { .tx.major_cmd = 101, .tx.minor_cmd = 12 };
    dp.tx.data_lsb = 1 << delcom_get_lednum(led);
    dp.tx.data_msb = 0;
    return hidled_send(led.rgb.ldev, dp.data);
    }
#[no_mangle]
unsafe extern "C" fn delcom_set_pwm(led: *mut hidled_led) -> c_int {
    static int delcom_set_pwm(struct hidled_led *led)
    {
    let mut dp: union delcom_packet = { .tx.major_cmd = 101, .tx.minor_cmd = 34 };
    dp.tx.data_lsb = delcom_get_lednum(led);
    dp.tx.data_msb = led.cdev.brightness;
    return hidled_send(led.rgb.ldev, dp.data);
    }
#[no_mangle]
unsafe extern "C" fn delcom_write(cdev: *mut led_classdev, br: enum led_brightness) -> c_int {
    static int delcom_write(struct led_classdev *cdev, enum led_brightness br)
    {
    struct hidled_led *led = to_hidled_led(cdev);
    int ret;
//
// enable LED
// We can't do this in the init function already because the device
// is internally reset later.
//
    ret = delcom_enable_led(led);
    if (ret)
    return ret;
    return delcom_set_pwm(led);
    }
#[no_mangle]
unsafe extern "C" fn delcom_init(ldev: *mut hidled_device) -> c_int {
    static int delcom_init(struct hidled_device *ldev)
    {
    let mut dp: union delcom_packet = { .rx.cmd = 104 };
    int ret;
    ret = hidled_recv(ldev, dp.data);
    if (ret)
    return ret;
//
// Several Delcom devices share the same USB VID/PID
// Check for family id 2 for Visual Signal Indicator
//
    return le16_to_cpu(dp.fw.family_code) == 2 ? 0 : -ENODEV;
    }
#[no_mangle]
unsafe extern "C" fn luxafor_write(cdev: *mut led_classdev, br: enum led_brightness) -> c_int {
    static int luxafor_write(struct led_classdev *cdev, enum led_brightness br)
    {
    struct hidled_led *led = to_hidled_led(cdev);
    __u8 buf[MAX_REPORT_SIZE] = { [1] = 1 };
    buf[2] = led.rgb.num + 1;
    buf[3] = led.rgb.red.cdev.brightness;
    buf[4] = led.rgb.green.cdev.brightness;
    buf[5] = led.rgb.blue.cdev.brightness;
    return hidled_send(led.rgb.ldev, buf);
    }
    static const struct hidled_config hidled_configs[] = {
    {
    .type = RISO_KAGAKU,
    .name = "Riso Kagaku Webmail Notifier",
    .short_name = "riso_kagaku",
    .max_brightness = 1,
    .num_leds = 1,
    .report_size = 6,
    .report_type = OUTPUT_REPORT,
    .write = riso_kagaku_write,
    },
    {
    .type = DREAM_CHEEKY,
    .name = "Dream Cheeky Webmail Notifier",
    .short_name = "dream_cheeky",
    .max_brightness = 63,
    .num_leds = 1,
    .report_size = 9,
    .report_type = RAW_REQUEST,
    .init = dream_cheeky_init,
    .write = dream_cheeky_write,
    },
    {
    .type = THINGM,
    .name = "ThingM blink(1)",
    .short_name = "thingm",
    .max_brightness = 255,
    .num_leds = 2,
    .report_size = 9,
    .report_type = RAW_REQUEST,
    .init = thingm_init,
    .write = thingm_write,
    },
    {
    .type = DELCOM,
    .name = "Delcom Visual Signal Indicator G2",
    .short_name = "delcom",
    .max_brightness = 100,
    .num_leds = 1,
    .report_size = 8,
    .report_type = RAW_REQUEST,
    .init = delcom_init,
    .write = delcom_write,
    },
    {
    .type = LUXAFOR,
    .name = "Greynut Luxafor",
    .short_name = "luxafor",
    .max_brightness = 255,
    .num_leds = 6,
    .report_size = 9,
    .report_type = OUTPUT_REPORT,
    .write = luxafor_write,
    },
    };
    static int hidled_init_led(struct hidled_led *led, const char *color_name,
    struct hidled_rgb *rgb, unsigned int minor)
    {
    const struct hidled_config *config = rgb.ldev.config;
    if (config.num_leds > 1)
    snprintf(led.name, sizeof(led.name), "%s%u:%s:led%u",
    config.short_name, minor, color_name, rgb.num);
    else
    snprintf(led.name, sizeof(led.name), "%s%u:%s",
    config.short_name, minor, color_name);
    led.cdev.name = led.name;
    led.cdev.max_brightness = config.max_brightness;
    led.cdev.brightness_set_blocking = config.write;
    led.cdev.flags = LED_HW_PLUGGABLE;
    led.rgb = rgb;
    return devm_led_classdev_register(&rgb.ldev.hdev.dev, &led.cdev);
    }
#[no_mangle]
unsafe extern "C" fn hidled_init_rgb(rgb: *mut hidled_rgb, minor: c_uint) -> c_int {
    static int hidled_init_rgb(struct hidled_rgb *rgb, unsigned int minor)
    {
    int ret;
// Register the red diode
    ret = hidled_init_led(&rgb.red, "red", rgb, minor);
    if (ret)
    return ret;
// Register the green diode
    ret = hidled_init_led(&rgb.green, "green", rgb, minor);
    if (ret)
    return ret;
// Register the blue diode
    return hidled_init_led(&rgb.blue, "blue", rgb, minor);
    }
#[no_mangle]
unsafe extern "C" fn hidled_probe(hdev: *mut hid_device, id: *const hid_device_id) -> c_int {
    static int hidled_probe(struct hid_device *hdev, const struct hid_device_id *id)
    {
    struct hidled_device *ldev;
    unsigned int minor;
    int ret, i;
    ldev = devm_kzalloc(&hdev.dev, sizeof(*ldev), GFP_KERNEL);
    if (!ldev)
    return -ENOMEM;
    ldev.buf = devm_kmalloc(&hdev.dev, MAX_REPORT_SIZE, GFP_KERNEL);
    if (!ldev.buf)
    return -ENOMEM;
    ret = hid_parse(hdev);
    if (ret)
    return ret;
    ldev.hdev = hdev;
    mutex_init(&ldev.lock);
    for (i = 0; !ldev.config && i < ARRAY_SIZE(hidled_configs); i++)
    if (hidled_configs[i].type == id.driver_data)
    ldev.config = &hidled_configs[i];
    if (!ldev.config)
    return -EINVAL;
    if (ldev.config.init) {
    ret = ldev.config.init(ldev);
    if (ret)
    return ret;
    }
    ldev.rgb = devm_kcalloc(&hdev.dev, ldev.config.num_leds,
    sizeof(struct hidled_rgb), GFP_KERNEL);
    if (!ldev.rgb)
    return -ENOMEM;
    ret = hid_hw_start(hdev, HID_CONNECT_HIDRAW);
    if (ret)
    return ret;
    minor = ((struct hidraw *) hdev.hidraw).minor;
    for (i = 0; i < ldev.config.num_leds; i++) {
    ldev.rgb[i].ldev = ldev;
    ldev.rgb[i].num = i;
    ret = hidled_init_rgb(&ldev.rgb[i], minor);
    if (ret) {
    hid_hw_stop(hdev);
    return ret;
    }
    }
    hid_info(hdev, "%s initialized\n", ldev.config.name);
    return 0;
    }
    static const struct hid_device_id hidled_table[] = {
    { HID_USB_DEVICE(USB_VENDOR_ID_RISO_KAGAKU,
    USB_DEVICE_ID_RI_KA_WEBMAIL), .driver_data = RISO_KAGAKU },
    { HID_USB_DEVICE(USB_VENDOR_ID_DREAM_CHEEKY,
    USB_DEVICE_ID_DREAM_CHEEKY_WN), .driver_data = DREAM_CHEEKY },
    { HID_USB_DEVICE(USB_VENDOR_ID_DREAM_CHEEKY,
    USB_DEVICE_ID_DREAM_CHEEKY_FA), .driver_data = DREAM_CHEEKY },
    { HID_USB_DEVICE(USB_VENDOR_ID_THINGM,
    USB_DEVICE_ID_BLINK1), .driver_data = THINGM },
    { HID_USB_DEVICE(USB_VENDOR_ID_DELCOM,
    USB_DEVICE_ID_DELCOM_VISUAL_IND), .driver_data = DELCOM },
    { HID_USB_DEVICE(USB_VENDOR_ID_MICROCHIP,
    USB_DEVICE_ID_LUXAFOR), .driver_data = LUXAFOR },
    { }
    };
    MODULE_DEVICE_TABLE(hid, hidled_table);
    static struct hid_driver hidled_driver = {
    .name = "hid-led",
    .probe = hidled_probe,
    .id_table = hidled_table,
    };
    module_hid_driver(hidled_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Heiner Kallweit <hkallweit1@gmail.com>");
    MODULE_DESCRIPTION("Simple USB RGB LED driver");
