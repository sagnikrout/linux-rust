//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/barco-p50-gpio.c
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
// Support for EC-connected GPIOs for identify
// LED/button on Barco P50 board
//
// Copyright (C) 2021 Barco NV
// Author: Santosh Kumar Yadav <santoshkumar.yadav@barco.com>
//

// GPIO lines
pub const P50_GPIO_LINE_LED: c_int = 0;
pub const P50_GPIO_LINE_BTN: c_int = 1;
// GPIO IO Ports
pub const P50_GPIO_IO_PORT_BASE: c_uint = 0x299;
pub const P50_PORT_DATA: c_uint = 0x00;
pub const P50_PORT_CMD: c_uint = 0x01;
pub const P50_STATUS_OBF: c_uint = 0x01 /* EC output buffer full */;
pub const P50_STATUS_IBF: c_uint = 0x02 /* EC input buffer full */;
pub const P50_CMD_READ: c_uint = 0xa0;
pub const P50_CMD_WRITE: c_uint = 0x50;
// EC mailbox registers
pub const P50_MBOX_REG_CMD: c_uint = 0x00;
pub const P50_MBOX_REG_STATUS: c_uint = 0x01;
pub const P50_MBOX_REG_PARAM: c_uint = 0x02;
pub const P50_MBOX_REG_DATA: c_uint = 0x03;
pub const P50_MBOX_CMD_READ_GPIO: c_uint = 0x11;
pub const P50_MBOX_CMD_WRITE_GPIO: c_uint = 0x12;
pub const P50_MBOX_CMD_CLEAR: c_uint = 0xff;
pub const P50_MBOX_STATUS_SUCCESS: c_uint = 0x01;
pub const P50_MBOX_PARAM_LED: c_uint = 0x12;
pub const P50_MBOX_PARAM_BTN: c_uint = 0x13;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct p50_gpio {
    pub gc: gpio_chip,
    pub lock: mutex,
    pub base: c_ulong,
    pub leds_pdev: *mut platform_device,
    pub keys_pdev: *mut platform_device,
}

    static struct platform_device *gpio_pdev;
    static int gpio_params[] = {
    [P50_GPIO_LINE_LED] = P50_MBOX_PARAM_LED,
    [P50_GPIO_LINE_BTN] = P50_MBOX_PARAM_BTN,
    };
    static const char * const gpio_names[] = {
    [P50_GPIO_LINE_LED] = "identify-led",
    [P50_GPIO_LINE_BTN] = "identify-button",
    };
    static const struct software_node gpiochip_node = {
    .name = DRIVER_NAME,
    };
// GPIO LEDs
    static const struct software_node gpio_leds_node = {
    .name = "gpio-leds-identify",
    };
    static const struct property_entry identify_led_props[] = {
    PROPERTY_ENTRY_GPIO("gpios", &gpiochip_node, P50_GPIO_LINE_LED, GPIO_ACTIVE_HIGH),
    { }
    };
    static const struct software_node identify_led_node = {
    .parent = &gpio_leds_node,
    .name = "identify",
    .properties = identify_led_props,
    };
// GPIO keyboard
    static const struct property_entry gpio_keys_props[] = {
    PROPERTY_ENTRY_STRING("label", "identify"),
    PROPERTY_ENTRY_U32("poll-interval", 100),
    { }
    };
    static const struct software_node gpio_keys_node = {
    .name = "gpio-keys-identify",
    .properties = gpio_keys_props,
    };
    static struct property_entry vendor_key_props[] = {
    PROPERTY_ENTRY_U32("linux,code", KEY_VENDOR),
    PROPERTY_ENTRY_GPIO("gpios", &gpiochip_node, P50_GPIO_LINE_BTN, GPIO_ACTIVE_LOW),
    { }
    };
    static const struct software_node vendor_key_node = {
    .parent = &gpio_keys_node,
    .properties = vendor_key_props,
    };
    static const struct software_node *p50_swnodes[] = {
    &gpio_leds_node,
    &identify_led_node,
    &gpio_keys_node,
    &vendor_key_node,
    core::ptr::null_mut()
    };
// low level access routines
#[no_mangle]
unsafe extern "C" fn p50_wait_ec(p50: *mut p50_gpio, mask: c_int, expected: c_int) -> c_int {
    static int p50_wait_ec(struct p50_gpio *p50, int mask, int expected)
    {
    int i, val;
    for (i = 0; i < 100; i++) {
    val = inb(p50.base + P50_PORT_CMD) & mask;
    if (val == expected)
    return 0;
    usleep_range(500, 2000);
    }
    dev_err(p50.gc.parent, "Timed out waiting for EC (0x%x)\n", val);
    return -ETIMEDOUT;
    }
#[no_mangle]
unsafe extern "C" fn p50_read_mbox_reg(p50: *mut p50_gpio, reg: c_int) -> c_int {
    static int p50_read_mbox_reg(struct p50_gpio *p50, int reg)
    {
    int ret;
    ret = p50_wait_ec(p50, P50_STATUS_IBF, 0);
    if (ret)
    return ret;
// clear output buffer flag, prevent unfinished commands
    inb(p50.base + P50_PORT_DATA);
// cmd/address
    outb(P50_CMD_READ | reg, p50.base + P50_PORT_CMD);
    ret = p50_wait_ec(p50, P50_STATUS_OBF, P50_STATUS_OBF);
    if (ret)
    return ret;
    return inb(p50.base + P50_PORT_DATA);
    }
#[no_mangle]
unsafe extern "C" fn p50_write_mbox_reg(p50: *mut p50_gpio, reg: c_int, val: c_int) -> c_int {
    static int p50_write_mbox_reg(struct p50_gpio *p50, int reg, int val)
    {
    int ret;
    ret = p50_wait_ec(p50, P50_STATUS_IBF, 0);
    if (ret)
    return ret;
// cmd/address
    outb(P50_CMD_WRITE | reg, p50.base + P50_PORT_CMD);
    ret = p50_wait_ec(p50, P50_STATUS_IBF, 0);
    if (ret)
    return ret;
// data
    outb(val, p50.base + P50_PORT_DATA);
    return 0;
    }
// mbox routines
#[no_mangle]
unsafe extern "C" fn p50_wait_mbox_idle(p50: *mut p50_gpio) -> c_int {
    static int p50_wait_mbox_idle(struct p50_gpio *p50)
    {
    int i, val;
    for (i = 0; i < 1000; i++) {
    val = p50_read_mbox_reg(p50, P50_MBOX_REG_CMD);
// cmd is 0 when idle
    if (val <= 0)
    return val;
    usleep_range(500, 2000);
    }
    dev_err(p50.gc.parent,	"Timed out waiting for EC mbox idle (CMD: 0x%x)\n", val);
    return -ETIMEDOUT;
    }
#[no_mangle]
unsafe extern "C" fn p50_send_mbox_cmd(p50: *mut p50_gpio, cmd: c_int, param: c_int, data: c_int) -> c_int {
    static int p50_send_mbox_cmd(struct p50_gpio *p50, int cmd, int param, int data)
    {
    int ret;
    ret = p50_wait_mbox_idle(p50);
    if (ret)
    return ret;
    ret = p50_write_mbox_reg(p50, P50_MBOX_REG_DATA, data);
    if (ret)
    return ret;
    ret = p50_write_mbox_reg(p50, P50_MBOX_REG_PARAM, param);
    if (ret)
    return ret;
    ret = p50_write_mbox_reg(p50, P50_MBOX_REG_CMD, cmd);
    if (ret)
    return ret;
    ret = p50_wait_mbox_idle(p50);
    if (ret)
    return ret;
    ret = p50_read_mbox_reg(p50, P50_MBOX_REG_STATUS);
    if (ret < 0)
    return ret;
    if (ret == P50_MBOX_STATUS_SUCCESS)
    return 0;
    dev_err(p50.gc.parent,	"Mbox command failed (CMD=0x%x STAT=0x%x PARAM=0x%x DATA=0x%x)\n",
    cmd, ret, param, data);
    return -EIO;
    }
// gpio routines
#[no_mangle]
unsafe extern "C" fn p50_gpio_get_direction(gc: *mut gpio_chip, offset: c_uint) -> c_int {
    static int p50_gpio_get_direction(struct gpio_chip *gc, unsigned int offset)
    {
    switch (offset) {
    case P50_GPIO_LINE_BTN:
    return GPIO_LINE_DIRECTION_IN;
    case P50_GPIO_LINE_LED:
    return GPIO_LINE_DIRECTION_OUT;
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn p50_gpio_get(gc: *mut gpio_chip, offset: c_uint) -> c_int {
    static int p50_gpio_get(struct gpio_chip *gc, unsigned int offset)
    {
    struct p50_gpio *p50 = gpiochip_get_data(gc);
    int ret;
    guard(mutex)(&p50.lock);
    ret = p50_send_mbox_cmd(p50, P50_MBOX_CMD_READ_GPIO, gpio_params[offset], 0);
    if (ret < 0)
    return ret;
    ret = p50_read_mbox_reg(p50, P50_MBOX_REG_DATA);
    if (ret < 0)
    return ret;
    return !!ret;
    }
#[no_mangle]
unsafe extern "C" fn p50_gpio_set(gc: *mut gpio_chip, offset: c_uint, value: c_int) -> c_int {
    static int p50_gpio_set(struct gpio_chip *gc, unsigned int offset, int value)
    {
    struct p50_gpio *p50 = gpiochip_get_data(gc);
    guard(mutex)(&p50.lock);
    return p50_send_mbox_cmd(p50, P50_MBOX_CMD_WRITE_GPIO,
    gpio_params[offset], value);
    }
#[no_mangle]
unsafe extern "C" fn p50_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int p50_gpio_probe(struct platform_device *pdev)
    {
    struct platform_device_info key_info = {
    .name	= "gpio-keys-polled",
    .id	= PLATFORM_DEVID_NONE,
    .parent	= &pdev.dev,
    };
    struct platform_device_info led_info = {
    .name	= "leds-gpio",
    .id	= PLATFORM_DEVID_NONE,
    .parent	= &pdev.dev,
    };
    struct p50_gpio *p50;
    struct resource *res;
    int ret;
    res = platform_get_resource(pdev, IORESOURCE_IO, 0);
    if (!res) {
    dev_err(&pdev.dev, "Cannot get I/O ports\n");
    return -ENODEV;
    }
    if (!devm_request_region(&pdev.dev, res.start, resource_size(res), pdev.name)) {
    dev_err(&pdev.dev, "Unable to reserve I/O region\n");
    return -EBUSY;
    }
    p50 = devm_kzalloc(&pdev.dev, sizeof(*p50), GFP_KERNEL);
    if (!p50)
    return -ENOMEM;
    platform_set_drvdata(pdev, p50);
    mutex_init(&p50.lock);
    p50.base = res.start;
    p50.gc.owner = THIS_MODULE;
    p50.gc.parent = &pdev.dev;
    p50.gc.label = dev_name(&pdev.dev);
    p50.gc.ngpio = ARRAY_SIZE(gpio_names);
    p50.gc.names = gpio_names;
    p50.gc.can_sleep = true;
    p50.gc.base = -1;
    p50.gc.get_direction = p50_gpio_get_direction;
    p50.gc.get = p50_gpio_get;
    p50.gc.set = p50_gpio_set;
// reset mbox
    ret = p50_wait_mbox_idle(p50);
    if (ret)
    return ret;
    ret = p50_write_mbox_reg(p50, P50_MBOX_REG_CMD, P50_MBOX_CMD_CLEAR);
    if (ret)
    return ret;
    ret = p50_wait_mbox_idle(p50);
    if (ret)
    return ret;
    ret = devm_gpiochip_add_data(&pdev.dev, &p50.gc, p50);
    if (ret < 0) {
    dev_err(&pdev.dev, "Could not register gpiochip: %d\n", ret);
    return ret;
    }
    ret = software_node_register_node_group(p50_swnodes);
    if (ret)
    return dev_err_probe(&pdev.dev, ret, "failed to register software nodes");
    led_info.fwnode = software_node_fwnode(&gpio_leds_node);
    p50.leds_pdev = platform_device_register_full(&led_info);
    if (IS_ERR(p50.leds_pdev)) {
    ret = PTR_ERR(p50.leds_pdev);
    dev_err(&pdev.dev, "Could not register leds-gpio: %d\n", ret);
    goto err_leds;
    }
    key_info.fwnode = software_node_fwnode(&gpio_keys_node);
    p50.keys_pdev = platform_device_register_full(&key_info);
    if (IS_ERR(p50.keys_pdev)) {
    ret = PTR_ERR(p50.keys_pdev);
    dev_err(&pdev.dev, "Could not register gpio-keys-polled: %d\n", ret);
    goto err_keys;
    }
    return 0;
    err_keys:
    platform_device_unregister(p50.leds_pdev);
    err_leds:
    software_node_unregister_node_group(p50_swnodes);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn p50_gpio_remove(pdev: *mut platform_device) {
    static void p50_gpio_remove(struct platform_device *pdev)
    {
    struct p50_gpio *p50 = platform_get_drvdata(pdev);
    platform_device_unregister(p50.keys_pdev);
    platform_device_unregister(p50.leds_pdev);
    software_node_unregister_node_group(p50_swnodes);
    }
    static struct platform_driver p50_gpio_driver = {
    .driver = {
    .name = DRIVER_NAME,
    },
    .probe = p50_gpio_probe,
    .remove = p50_gpio_remove,
    };
// Board setup
    static const struct dmi_system_id dmi_ids[] __initconst = {
    {
    .matches = {
    DMI_EXACT_MATCH(DMI_SYS_VENDOR, "Barco"),
    DMI_EXACT_MATCH(DMI_PRODUCT_FAMILY, "P50")
    },
    },
    {}
    };
    MODULE_DEVICE_TABLE(dmi, dmi_ids);
#[no_mangle]
unsafe extern "C" fn p50_module_init() -> int __init {
    static int __init p50_module_init(void)
    {
    let mut res: resource = DEFINE_RES_IO(P50_GPIO_IO_PORT_BASE, P50_PORT_CMD + 1);
    struct platform_device_info pdevinfo = {
    .name = DRIVER_NAME,
    .id = PLATFORM_DEVID_NONE,
    .res = &res,
    .num_res = 1,
    .swnode = &gpiochip_node,
    };
    int ret;
    if (!dmi_first_match(dmi_ids))
    return -ENODEV;
    ret = platform_driver_register(&p50_gpio_driver);
    if (ret)
    return ret;
    gpio_pdev = platform_device_register_full(&pdevinfo);
    if (IS_ERR(gpio_pdev)) {
    pr_err("failed registering %s: %ld\n", DRIVER_NAME, PTR_ERR(gpio_pdev));
    platform_driver_unregister(&p50_gpio_driver);
    return PTR_ERR(gpio_pdev);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn p50_module_exit() -> void __exit {
    static void __exit p50_module_exit(void)
    {
    platform_device_unregister(gpio_pdev);
    platform_driver_unregister(&p50_gpio_driver);
    }
    module_init(p50_module_init);
    module_exit(p50_module_exit);
    MODULE_AUTHOR("Santosh Kumar Yadav, Barco NV <santoshkumar.yadav@barco.com>");
    MODULE_DESCRIPTION("Barco P50 identify GPIOs driver");
    MODULE_LICENSE("GPL");
