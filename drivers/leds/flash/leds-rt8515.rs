//! Automatically rewritten from C to Rust
//! Source: drivers/leds/flash/leds-rt8515.c
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
// LED driver for Richtek RT8515 flash/torch white LEDs
// found on some Samsung mobile phones.
//
// This is a 1.5A Boost dual channel driver produced around 2011.
//
// The component lacks a datasheet, but in the schematic picture
// from the LG P970 service manual you can see the connections
// from the RT8515 to the LED, with two resistors connected
// from the pins "RFS" and "RTS" to ground.
//
// On the LG P970:
// RFS (resistance flash setting?) is 20 kOhm
// RTS (resistance torch setting?) is 39 kOhm
//
// Some sleuthing finds us the RT9387A which we have a datasheet for:
// https://static5.arrow.com/pdfs/2014/7/27/8/21/12/794/rtt_/manual/94download_ds.jspprt9387a.jspprt9387a.pdf
// This apparently works the same way so in theory this driver
// should cover RT9387A as well. This has not been tested, please
// update the compatibles if you add RT9387A support.
//
// Linus Walleij <linus.walleij@linaro.org>
//

// We can provide 15-700 mA out to the LED
pub const RT8515_MIN_IOUT_MA: c_int = 15;
pub const RT8515_MAX_IOUT_MA: c_int = 700;
// The maximum intensity is 1-16 for flash and 1-100 for torch
pub const RT8515_FLASH_MAX: c_int = 16;
pub const RT8515_TORCH_MAX: c_int = 100;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt8515 {
    pub fled: led_classdev_flash,
    pub dev: *mut device,
    pub v4l2_flash: *mut v4l2_flash,
    pub lock: mutex,
    pub reg: *mut regulator,
    pub enable_torch: *mut gpio_desc,
    pub enable_flash: *mut gpio_desc,
    pub powerdown_timer: timer_list,
    pub /: *mut *mut u32 max_timeout; / Flash max timeout,
    pub flash_max_intensity: c_int,
    pub torch_max_intensity: c_int,
}

    static struct rt8515 *to_rt8515(struct led_classdev_flash *fled)
    {
    return container_of(fled, struct rt8515, fled);
    }
#[no_mangle]
unsafe extern "C" fn rt8515_gpio_led_off(rt: *mut rt8515) {
    static void rt8515_gpio_led_off(struct rt8515 *rt)
    {
    gpiod_set_value(rt.enable_flash, 0);
    gpiod_set_value(rt.enable_torch, 0);
    }
    static void rt8515_gpio_brightness_commit(struct gpio_desc *gpiod,
    int brightness)
    {
    int i;
//
// Toggling a GPIO line with a small delay increases the
// brightness one step at a time.
//
    for (i = 0; i < brightness; i++) {
    gpiod_set_value(gpiod, 0);
    udelay(1);
    gpiod_set_value(gpiod, 1);
    udelay(1);
    }
    }
// This is setting the torch light level
    static int rt8515_led_brightness_set(struct led_classdev *led,
    enum led_brightness brightness)
    {
    struct led_classdev_flash *fled = lcdev_to_flcdev(led);
    struct rt8515 *rt = to_rt8515(fled);
    mutex_lock(&rt.lock);
    if (brightness == LED_OFF) {
// Off
    rt8515_gpio_led_off(rt);
    } else if (brightness < RT8515_TORCH_MAX) {
// Step it up to movie mode brightness using the flash pin
    rt8515_gpio_brightness_commit(rt.enable_torch, brightness);
    } else {
// Max torch brightness requested
    gpiod_set_value(rt.enable_torch, 1);
    }
    mutex_unlock(&rt.lock);
    return 0;
    }
    static int rt8515_led_flash_strobe_set(struct led_classdev_flash *fled,
    bool state)
    {
    struct rt8515 *rt = to_rt8515(fled);
    struct led_flash_setting *timeout = &fled.timeout;
    let mut brightness: c_int = rt.flash_max_intensity;
    mutex_lock(&rt.lock);
    if (state) {
// Enable LED flash mode and set brightness
    rt8515_gpio_brightness_commit(rt.enable_flash, brightness);
// Set timeout
    mod_timer(&rt.powerdown_timer,
    jiffies + usecs_to_jiffies(timeout.val));
    } else {
    timer_delete_sync(&rt.powerdown_timer);
// Turn the LED off
    rt8515_gpio_led_off(rt);
    }
    fled.led_cdev.brightness = LED_OFF;
// After this the torch LED will be disabled
    mutex_unlock(&rt.lock);
    return 0;
    }
    static int rt8515_led_flash_strobe_get(struct led_classdev_flash *fled,
    bool *state)
    {
    struct rt8515 *rt = to_rt8515(fled);
// state = timer_pending(&rt->powerdown_timer);
    return 0;
    }
    static int rt8515_led_flash_timeout_set(struct led_classdev_flash *fled,
    u32 timeout)
    {
// The timeout is stored in the led-class-flash core
    return 0;
    }
    static const struct led_flash_ops rt8515_flash_ops = {
    .strobe_set = rt8515_led_flash_strobe_set,
    .strobe_get = rt8515_led_flash_strobe_get,
    .timeout_set = rt8515_led_flash_timeout_set,
    };
#[no_mangle]
unsafe extern "C" fn rt8515_powerdown_timer(t: *mut timer_list) {
    static void rt8515_powerdown_timer(struct timer_list *t)
    {
    struct rt8515 *rt = timer_container_of(rt, t, powerdown_timer);
// Turn the LED off
    rt8515_gpio_led_off(rt);
    }
#[no_mangle]
unsafe extern "C" fn rt8515_init_flash_timeout(rt: *mut rt8515) {
    static void rt8515_init_flash_timeout(struct rt8515 *rt)
    {
    struct led_classdev_flash *fled = &rt.fled;
    struct led_flash_setting *s;
// Init flash timeout setting
    s = &fled.timeout;
    s.min = 1;
    s.max = rt.max_timeout;
    s.step = 1;
//
// Set default timeout to RT8515_TIMEOUT_US except if
// max_timeout from DT is lower.
//
    s.val = min(rt.max_timeout, RT8515_TIMEOUT_US);
    }

// Configure the V2L2 flash subdevice
    static void rt8515_init_v4l2_flash_config(struct rt8515 *rt,
    struct v4l2_flash_config *v4l2_sd_cfg)
    {
    struct led_classdev *led = &rt.fled.led_cdev;
    struct led_flash_setting *s;
    strscpy(v4l2_sd_cfg.dev_name, led.dev.kobj.name,
    sizeof(v4l2_sd_cfg.dev_name));
//
// Init flash intensity setting: this is a linear scale
// capped from the device tree max intensity setting
// 1..flash_max_intensity
//
    s = &v4l2_sd_cfg.intensity;
    s.min = 1;
    s.max = rt.flash_max_intensity;
    s.step = 1;
    s.val = s.max;
    }
#[no_mangle]
unsafe extern "C" fn rt8515_v4l2_flash_release(rt: *mut rt8515) {
    static void rt8515_v4l2_flash_release(struct rt8515 *rt)
    {
    v4l2_flash_release(rt.v4l2_flash);
    }

    static void rt8515_init_v4l2_flash_config(struct rt8515 *rt,
    struct v4l2_flash_config *v4l2_sd_cfg)
    {
    }
#[no_mangle]
unsafe extern "C" fn rt8515_v4l2_flash_release(rt: *mut rt8515) {
    static void rt8515_v4l2_flash_release(struct rt8515 *rt)
    {
    }

    static void rt8515_determine_max_intensity(struct rt8515 *rt,
    struct fwnode_handle *led,
    const char *resistance,
    const char *max_ua_prop, int hw_max,
    int *max_intensity_setting)
    {
    u32 res = 0; /* Can't be 0 so 0 is undefined */
    u32 ua;
    u32 max_ma;
    int max_intensity;
    int ret;
    fwnode_property_read_u32(rt.dev.fwnode, resistance, &res);
    ret = fwnode_property_read_u32(led, max_ua_prop, &ua);
// Missing info in DT, OK go with hardware maxima
    if (ret || res == 0) {
    dev_err(rt.dev,
    "either %s or %s missing from DT, using HW max\n",
    resistance, max_ua_prop);
    max_ma = RT8515_MAX_IOUT_MA;
    max_intensity = hw_max;
    goto out_assign_max;
    }
//
// Formula from the datasheet, this is the maximum current
// defined by the hardware.
//
    max_ma = (5500 * 1000) / res;
//
// Calculate max intensity (linear scaling)
// Formula is ((ua / 1000) / max_ma) * 100, then simplified
//
    max_intensity = (ua / 10) / max_ma;
    dev_info(rt.dev,
    "current restricted from %u to %u mA, max intensity %d/100\n",
    max_ma, (ua / 1000), max_intensity);
    out_assign_max:
    dev_info(rt.dev, "max intensity %d/%d = %d mA\n",
    max_intensity, hw_max, max_ma);
// max_intensity_setting = max_intensity;
    }
#[no_mangle]
unsafe extern "C" fn rt8515_probe(pdev: *mut platform_device) -> c_int {
    static int rt8515_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct fwnode_handle *child;
    struct rt8515 *rt;
    struct led_classdev *led;
    struct led_classdev_flash *fled;
    let mut init_data: led_init_data = {};
    let mut v4l2_sd_cfg: v4l2_flash_config = {};
    int ret;
    rt = devm_kzalloc(dev, sizeof(*rt), GFP_KERNEL);
    if (!rt)
    return -ENOMEM;
    rt.dev = dev;
    fled = &rt.fled;
    led = &fled.led_cdev;
// ENF - Enable Flash line
    rt.enable_flash = devm_gpiod_get(dev, "enf", GPIOD_OUT_LOW);
    if (IS_ERR(rt.enable_flash))
    return dev_err_probe(dev, PTR_ERR(rt.enable_flash),
    "cannot get ENF (enable flash) GPIO\n");
// ENT - Enable Torch line
    rt.enable_torch = devm_gpiod_get(dev, "ent", GPIOD_OUT_LOW);
    if (IS_ERR(rt.enable_torch))
    return dev_err_probe(dev, PTR_ERR(rt.enable_torch),
    "cannot get ENT (enable torch) GPIO\n");
    child = device_get_next_child_node(dev, core::ptr::null_mut());
    if (!child) {
    dev_err(dev,
    "No fwnode child node found for connected LED.\n");
    return -EINVAL;
    }
    init_data.fwnode = child;
    rt8515_determine_max_intensity(rt, child, "richtek,rfs-ohms",
    "flash-max-microamp",
    RT8515_FLASH_MAX,
    &rt.flash_max_intensity);
    rt8515_determine_max_intensity(rt, child, "richtek,rts-ohms",
    "led-max-microamp",
    RT8515_TORCH_MAX,
    &rt.torch_max_intensity);
    ret = fwnode_property_read_u32(child, "flash-max-timeout-us",
    &rt.max_timeout);
    if (ret) {
    rt.max_timeout = RT8515_MAX_TIMEOUT_US;
    dev_warn(dev,
    "flash-max-timeout-us property missing\n");
    }
    timer_setup(&rt.powerdown_timer, rt8515_powerdown_timer, 0);
    rt8515_init_flash_timeout(rt);
    fled.ops = &rt8515_flash_ops;
    led.max_brightness = rt.torch_max_intensity;
    led.brightness_set_blocking = rt8515_led_brightness_set;
    led.flags |= LED_CORE_SUSPENDRESUME | LED_DEV_CAP_FLASH;
    mutex_init(&rt.lock);
    platform_set_drvdata(pdev, rt);
    ret = devm_led_classdev_flash_register_ext(dev, fled, &init_data);
    if (ret) {
    fwnode_handle_put(child);
    mutex_destroy(&rt.lock);
    dev_err(dev, "can't register LED %s\n", led.name);
    return ret;
    }
    rt8515_init_v4l2_flash_config(rt, &v4l2_sd_cfg);
// Create a V4L2 Flash device if V4L2 flash is enabled
    rt.v4l2_flash = v4l2_flash_init(dev, child, fled, core::ptr::null_mut(), &v4l2_sd_cfg);
    if (IS_ERR(rt.v4l2_flash)) {
    ret = PTR_ERR(rt.v4l2_flash);
    dev_err(dev, "failed to register V4L2 flash device (%d)\n",
    ret);
//
// Continue without the V4L2 flash
// (we still have the classdev)
//
    }
    fwnode_handle_put(child);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rt8515_remove(pdev: *mut platform_device) {
    static void rt8515_remove(struct platform_device *pdev)
    {
    struct rt8515 *rt = platform_get_drvdata(pdev);
    rt8515_v4l2_flash_release(rt);
    timer_delete_sync(&rt.powerdown_timer);
    mutex_destroy(&rt.lock);
    }
    static const struct of_device_id rt8515_match[] = {
    { .compatible = "richtek,rt8515", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, rt8515_match);
    static struct platform_driver rt8515_driver = {
    .driver = {
    .name  = "rt8515",
    .of_match_table = rt8515_match,
    },
    .probe  = rt8515_probe,
    .remove = rt8515_remove,
    };
    module_platform_driver(rt8515_driver);
    MODULE_AUTHOR("Linus Walleij <linus.walleij@linaro.org>");
    MODULE_DESCRIPTION("Richtek RT8515 LED driver");
    MODULE_LICENSE("GPL");
