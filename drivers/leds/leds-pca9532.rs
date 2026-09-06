//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-pca9532.c
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
// pca9532.c - 16-bit Led dimmer
//
// Copyright (C) 2011 Jan Weitzel
// Copyright (C) 2008 Riku Voipio
//
// Datasheet: http://www.nxp.com/documents/data_sheet/PCA9532.pdf
//

// m =  num_leds

pub const PCA9532_PWM_PERIOD_DIV: c_int = 152;
pub const PCA9532_PWM_DUTY_DIV: c_int = 256;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pca9532_chip_info {
    pub num_leds: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pca9532_data {
    pub client: *mut i2c_client,
    pub leds: [pca9532_led; 16],
    pub update_lock: mutex,
    pub idev: *mut input_dev,
    pub work: work_struct,

    pub gpio: gpio_chip,

    pub chip_info: *const pca9532_chip_info,
pub const PCA9532_PWM_ID_0: c_int = 0;
pub const PCA9532_PWM_ID_1: c_int = 1;
    pub pwm: [u8; 2],
    pub psc: [u8; 2],
    pub hw_blink: bool,
}

    static int pca9532_probe(struct i2c_client *client);
    static void pca9532_remove(struct i2c_client *client);
    enum {
    pca9530,
    pca9531,
    pca9532,
    pca9533,
    };
    static const struct i2c_device_id pca9532_id[] = {
    { .name = "pca9530", .driver_data = pca9530 },
    { .name = "pca9531", .driver_data = pca9531 },
    { .name = "pca9532", .driver_data = pca9532 },
    { .name = "pca9533", .driver_data = pca9533 },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, pca9532_id);
    static const struct pca9532_chip_info pca9532_chip_info_tbl[] = {
    [pca9530] = {
    .num_leds = 2,
    },
    [pca9531] = {
    .num_leds = 8,
    },
    [pca9532] = {
    .num_leds = 16,
    },
    [pca9533] = {
    .num_leds = 4,
    },
    };

    static const struct of_device_id of_pca9532_leds_match[] = {
    { .compatible = "nxp,pca9530", .data = (void *)pca9530 },
    { .compatible = "nxp,pca9531", .data = (void *)pca9531 },
    { .compatible = "nxp,pca9532", .data = (void *)pca9532 },
    { .compatible = "nxp,pca9533", .data = (void *)pca9533 },
    {},
    };
    MODULE_DEVICE_TABLE(of, of_pca9532_leds_match);

    static struct i2c_driver pca9532_driver = {
    .driver = {
    .name = "leds-pca953x",
    .of_match_table = of_match_ptr(of_pca9532_leds_match),
    },
    .probe = pca9532_probe,
    .remove = pca9532_remove,
    .id_table = pca9532_id,
    };
// We have two pwm/blinkers, but 16 possible leds to drive. Additionally,
// the clever Thecus people are using one pwm to drive the beeper. So,
// as a compromise we average one pwm to the values requested by all
// leds that are not ON/OFF.
//
    static int pca9532_calcpwm(struct i2c_client *client, int pwm, int blink,
    enum led_brightness value)
    {
    let mut a: c_int = 0, b = 0, i = 0;
    struct pca9532_data *data = i2c_get_clientdata(client);
    for (i = 0; i < data.chip_info.num_leds; i++) {
    if (data.leds[i].type == PCA9532_TYPE_LED &&
    data.leds[i].state == PCA9532_PWM0+pwm) {
    a++;
    b += data.leds[i].ldev.brightness;
    }
    }
    if (a == 0) {
    dev_err(&client.dev,
    "fear of division by zero %d/%d, wanted %d\n",
    b, a, value);
    return -EINVAL;
    }
    b = b/a;
    if (b > 0xFF)
    return -EINVAL;
    data.pwm[pwm] = b;
    data.psc[pwm] = blink;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pca9532_setpwm(client: *mut i2c_client, pwm: c_int) -> c_int {
    static int pca9532_setpwm(struct i2c_client *client, int pwm)
    {
    struct pca9532_data *data = i2c_get_clientdata(client);
    let mut maxleds: u8 = data.chip_info.num_leds;
    mutex_lock(&data.update_lock);
    i2c_smbus_write_byte_data(client, PCA9532_REG_PWM(maxleds, pwm),
    data.pwm[pwm]);
    i2c_smbus_write_byte_data(client, PCA9532_REG_PSC(maxleds, pwm),
    data.psc[pwm]);
    mutex_unlock(&data.update_lock);
    return 0;
    }
// Set LED routing
#[no_mangle]
unsafe extern "C" fn pca9532_setled(led: *mut pca9532_led) {
    static void pca9532_setled(struct pca9532_led *led)
    {
    struct i2c_client *client = led.client;
    struct pca9532_data *data = i2c_get_clientdata(client);
    let mut maxleds: u8 = data.chip_info.num_leds;
    char reg;
    mutex_lock(&data.update_lock);
    reg = i2c_smbus_read_byte_data(client, LED_REG(maxleds, led.id));
// zero led bits
    reg = reg & ~LED_MASK(led.id);
// set the new value
    reg = reg | (led.state << LED_SHIFT(led.id));
    i2c_smbus_write_byte_data(client, LED_REG(maxleds, led.id), reg);
    mutex_unlock(&data.update_lock);
    }
    static int pca9532_set_brightness(struct led_classdev *led_cdev,
    enum led_brightness value)
    {
    let mut err: c_int = 0;
    struct pca9532_led *led = ldev_to_led(led_cdev);
    if (value == LED_OFF) {
    led.state = PCA9532_OFF;
    } else if (led.state == PCA9532_PWM1) {
    return 0; /* Non-zero brightness shall not stop HW blinking */
    } else if (value == LED_FULL) {
    led.state = PCA9532_ON;
    } else {
    led.state = PCA9532_PWM0; /* Thecus: hardcode one pwm */
    err = pca9532_calcpwm(led.client, PCA9532_PWM_ID_0, 0, value);
    if (err)
    return err;
    }
    if (led.state == PCA9532_PWM0)
    pca9532_setpwm(led.client, PCA9532_PWM_ID_0);
    pca9532_setled(led);
    return err;
    }
    static int pca9532_update_hw_blink(struct pca9532_led *led,
    unsigned long delay_on, unsigned long delay_off)
    {
    struct pca9532_data *data = i2c_get_clientdata(led.client);
    unsigned int psc;
    int i;
// Look for others LEDs that already use PWM1
    for (i = 0; i < data.chip_info.num_leds; i++) {
    struct pca9532_led *other = &data.leds[i];
    if (other == led)
    continue;
    if (other.state == PCA9532_PWM1) {
    if (other.ldev.blink_delay_on != delay_on ||
    other.ldev.blink_delay_off != delay_off) {
// HW can handle only one blink configuration at a time
    return -EINVAL;
    }
    }
    }
    psc = ((delay_on + delay_off) * PCA9532_PWM_PERIOD_DIV - 1) / 1000;
    if (psc > U8_MAX) {
// Blink period too long to be handled by hardware
    return -EINVAL;
    }
    led.state = PCA9532_PWM1;
    data.psc[PCA9532_PWM_ID_1] = psc;
    data.pwm[PCA9532_PWM_ID_1] = (delay_on * PCA9532_PWM_DUTY_DIV) / (delay_on + delay_off);
    return pca9532_setpwm(data.client, PCA9532_PWM_ID_1);
    }
    static int pca9532_set_blink(struct led_classdev *led_cdev,
    unsigned long *delay_on, unsigned long *delay_off)
    {
    struct pca9532_led *led = ldev_to_led(led_cdev);
    struct i2c_client *client = led.client;
    struct pca9532_data *data = i2c_get_clientdata(client);
    int err;
    if (!data.hw_blink)
    return -EINVAL;
    if (*delay_on == 0 && *delay_off == 0) {
// led subsystem ask us for a blink rate
// delay_on = 500;
// delay_off = 500;
    }
    err = pca9532_update_hw_blink(led, *delay_on, *delay_off);
    if (err)
    return err;
    pca9532_setled(led);
    return 0;
    }
    static int pca9532_event(struct input_dev *dev, unsigned int type,
    unsigned int code, int value)
    {
    struct pca9532_data *data = input_get_drvdata(dev);
    if (!(type == EV_SND && (code == SND_BELL || code == SND_TONE)))
    return -1;
// XXX: allow different kind of beeps with psc/pwm modifications
    if (value > 1 && value < 32767)
    data.pwm[PCA9532_PWM_ID_1] = 127;
    else
    data.pwm[PCA9532_PWM_ID_1] = 0;
    schedule_work(&data.work);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pca9532_input_work(work: *mut work_struct) {
    static void pca9532_input_work(struct work_struct *work)
    {
    struct pca9532_data *data =
    container_of(work, struct pca9532_data, work);
    let mut maxleds: u8 = data.chip_info.num_leds;
    mutex_lock(&data.update_lock);
    i2c_smbus_write_byte_data(data.client, PCA9532_REG_PWM(maxleds, 1),
    data.pwm[PCA9532_PWM_ID_1]);
    mutex_unlock(&data.update_lock);
    }
#[no_mangle]
unsafe extern "C" fn pca9532_getled(led: *mut pca9532_led) -> enum pca9532_state {
    static enum pca9532_state pca9532_getled(struct pca9532_led *led)
    {
    struct i2c_client *client = led.client;
    struct pca9532_data *data = i2c_get_clientdata(client);
    let mut maxleds: u8 = data.chip_info.num_leds;
    char reg;
    enum pca9532_state ret;
    mutex_lock(&data.update_lock);
    reg = i2c_smbus_read_byte_data(client, LED_REG(maxleds, led.id));
    ret = (reg & LED_MASK(led.id)) >> LED_SHIFT(led.id);
    mutex_unlock(&data.update_lock);
    return ret;
    }

#[no_mangle]
unsafe extern "C" fn pca9532_gpio_request_pin(gc: *mut gpio_chip, offset: unsigned) -> c_int {
    static int pca9532_gpio_request_pin(struct gpio_chip *gc, unsigned offset)
    {
    struct pca9532_data *data = gpiochip_get_data(gc);
    struct pca9532_led *led = &data.leds[offset];
    if (led.type == PCA9532_TYPE_GPIO)
    return 0;
    return -EBUSY;
    }
    static int pca9532_gpio_set_value(struct gpio_chip *gc, unsigned int offset,
    int val)
    {
    struct pca9532_data *data = gpiochip_get_data(gc);
    struct pca9532_led *led = &data.leds[offset];
    if (val)
    led.state = PCA9532_OFF;
    else
    led.state = PCA9532_ON;
    pca9532_setled(led);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pca9532_gpio_get_value(gc: *mut gpio_chip, offset: unsigned) -> c_int {
    static int pca9532_gpio_get_value(struct gpio_chip *gc, unsigned offset)
    {
    struct pca9532_data *data = gpiochip_get_data(gc);
    unsigned char reg;
    reg = i2c_smbus_read_byte_data(data.client, PCA9532_REG_INPUT(offset));
    return !!(reg & (1 << (offset % 8)));
    }
#[no_mangle]
unsafe extern "C" fn pca9532_gpio_direction_input(gc: *mut gpio_chip, offset: unsigned) -> c_int {
    static int pca9532_gpio_direction_input(struct gpio_chip *gc, unsigned offset)
    {
// To use as input ensure pin is not driven
    pca9532_gpio_set_value(gc, offset, 1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pca9532_gpio_direction_output(gc: *mut gpio_chip, offset: unsigned, val: c_int) -> c_int {
    static int pca9532_gpio_direction_output(struct gpio_chip *gc, unsigned offset, int val)
    {
    return pca9532_gpio_set_value(gc, offset, val);
    }

#[no_mangle]
unsafe extern "C" fn pca9532_destroy_devices(data: *mut pca9532_data, n_devs: c_int) {
    static void pca9532_destroy_devices(struct pca9532_data *data, int n_devs)
    {
    let mut i: c_int = n_devs;
    while (--i >= 0) {
    switch (data.leds[i].type) {
    case PCA9532_TYPE_NONE:
    case PCA9532_TYPE_GPIO:
    break;
    case PCA9532_TYPE_LED:
    led_classdev_unregister(&data.leds[i].ldev);
    break;
    case PCA9532_TYPE_N2100_BEEP:
    if (data.idev != core::ptr::null_mut()) {
    cancel_work_sync(&data.work);
    data.idev = core::ptr::null_mut();
    }
    break;
    }
    }

    if (data.gpio.parent)
    gpiochip_remove(&data.gpio);

    }
    static int pca9532_configure(struct i2c_client *client,
    struct pca9532_data *data, struct pca9532_platform_data *pdata)
    {
    int i, err = 0;
    let mut gpios: c_int = 0;
    let mut maxleds: u8 = data.chip_info.num_leds;
    for (i = 0; i < 2; i++)	{
    data.pwm[i] = pdata.pwm[i];
    data.psc[i] = pdata.psc[i];
    err = i2c_smbus_write_byte_data(client, PCA9532_REG_PWM(maxleds, i),
    data.pwm[i]);
    if (err < 0)
    return err;
    err = i2c_smbus_write_byte_data(client, PCA9532_REG_PSC(maxleds, i),
    data.psc[i]);
    if (err < 0)
    return err;
    }
    data.hw_blink = true;
    for (i = 0; i < data.chip_info.num_leds; i++) {
    struct pca9532_led *led = &data.leds[i];
    struct pca9532_led *pled = &pdata.leds[i];
    led.client = client;
    led.id = i;
    led.type = pled.type;
    switch (led.type) {
    case PCA9532_TYPE_NONE:
    break;
    case PCA9532_TYPE_GPIO:
    gpios++;
    break;
    case PCA9532_TYPE_LED:
    if (pled.state == PCA9532_KEEP)
    led.state = pca9532_getled(led);
    else
    led.state = pled.state;
    led.name = pled.name;
    led.ldev.name = led.name;
    led.ldev.default_trigger = pled.default_trigger;
    led.ldev.brightness = LED_OFF;
    led.ldev.brightness_set_blocking =
    pca9532_set_brightness;
    led.ldev.blink_set = pca9532_set_blink;
    err = led_classdev_register(&client.dev, &led.ldev);
    if (err < 0) {
    dev_err(&client.dev,
    "couldn't register LED %s\n",
    led.name);
    goto exit;
    }
    pca9532_setled(led);
    break;
    case PCA9532_TYPE_N2100_BEEP:
// PWM1 is reserved for beeper so blink will not use hardware
    data.hw_blink = false;
    BUG_ON(data.idev);
    led.state = PCA9532_PWM1;
    pca9532_setled(led);
    data.idev = devm_input_allocate_device(&client.dev);
    if (data.idev == core::ptr::null_mut()) {
    err = -ENOMEM;
    goto exit;
    }
    data.idev.name = pled.name;
    data.idev.phys = "i2c/pca9532";
    data.idev.id.bustype = BUS_HOST;
    data.idev.id.vendor = 0x001f;
    data.idev.id.product = 0x0001;
    data.idev.id.version = 0x0100;
    data.idev.evbit[0] = BIT_MASK(EV_SND);
    data.idev.sndbit[0] = BIT_MASK(SND_BELL) |
    BIT_MASK(SND_TONE);
    data.idev.event = pca9532_event;
    input_set_drvdata(data.idev, data);
    INIT_WORK(&data.work, pca9532_input_work);
    err = input_register_device(data.idev);
    if (err) {
    cancel_work_sync(&data.work);
    data.idev = core::ptr::null_mut();
    goto exit;
    }
    break;
    }
    }

    if (gpios) {
    data.gpio.label = "gpio-pca9532";
    data.gpio.direction_input = pca9532_gpio_direction_input;
    data.gpio.direction_output = pca9532_gpio_direction_output;
    data.gpio.set = pca9532_gpio_set_value;
    data.gpio.get = pca9532_gpio_get_value;
    data.gpio.request = pca9532_gpio_request_pin;
    data.gpio.can_sleep = 1;
    data.gpio.base = pdata.gpio_base;
    data.gpio.ngpio = data.chip_info.num_leds;
    data.gpio.parent = &client.dev;
    data.gpio.owner = THIS_MODULE;
    err = gpiochip_add_data(&data.gpio, data);
    if (err) {
// Use data->gpio.dev as a flag for freeing gpiochip
    data.gpio.parent = core::ptr::null_mut();
    dev_warn(&client.dev, "could not add gpiochip\n");
    } else {
    dev_info(&client.dev, "gpios %i...%i\n",
    data.gpio.base, data.gpio.base +
    data.gpio.ngpio - 1);
    }
    }

    return 0;
    exit:
    pca9532_destroy_devices(data, i);
    return err;
    }
    static struct pca9532_platform_data *
    pca9532_of_populate_pdata(struct device *dev, struct device_node *np)
    {
    struct pca9532_platform_data *pdata;
    int devid, maxleds;
    let mut i: c_int = 0;
    const char *state;
    devid = (int)(uintptr_t)of_device_get_match_data(dev);
    maxleds = pca9532_chip_info_tbl[devid].num_leds;
    pdata = devm_kzalloc(dev, sizeof(*pdata), GFP_KERNEL);
    if (!pdata)
    return ERR_PTR(-ENOMEM);
    pdata.gpio_base = -1;
    of_property_read_u8_array(np, "nxp,pwm", &pdata.pwm[PCA9532_PWM_ID_0],
    ARRAY_SIZE(pdata.pwm));
    of_property_read_u8_array(np, "nxp,psc", &pdata.psc[PCA9532_PWM_ID_0],
    ARRAY_SIZE(pdata.psc));
    for_each_available_child_of_node_scoped(np, child) {
    if (of_property_read_string(child, "label",
    &pdata.leds[i].name))
    pdata.leds[i].name = child.name;
    of_property_read_u32(child, "type", &pdata.leds[i].type);
    of_property_read_string(child, "linux,default-trigger",
    &pdata.leds[i].default_trigger);
    if (!of_property_read_string(child, "default-state", &state)) {
    if (!strcmp(state, "on"))
    pdata.leds[i].state = PCA9532_ON;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(state, _arg: "keep")) -> else {
    else if (!strcmp(state, "keep"))
    pdata.leds[i].state = PCA9532_KEEP;
    }
    if (++i >= maxleds)
    break;
    }
    return pdata;
    }
#[no_mangle]
unsafe extern "C" fn pca9532_probe(client: *mut i2c_client) -> c_int {
    static int pca9532_probe(struct i2c_client *client)
    {
    const struct i2c_device_id *id = i2c_client_get_device_id(client);
    int devid;
    struct pca9532_data *data = i2c_get_clientdata(client);
    struct pca9532_platform_data *pca9532_pdata =
    dev_get_platdata(&client.dev);
    struct device_node *np = dev_of_node(&client.dev);
    if (!pca9532_pdata) {
    if (np) {
    pca9532_pdata =
    pca9532_of_populate_pdata(&client.dev, np);
    if (IS_ERR(pca9532_pdata))
    return PTR_ERR(pca9532_pdata);
    } else {
    dev_err(&client.dev, "no platform data\n");
    return -EINVAL;
    }
    devid = (int)(uintptr_t)of_device_get_match_data(&client.dev);
    } else {
    devid = id.driver_data;
    }
    if (!i2c_check_functionality(client.adapter,
    I2C_FUNC_SMBUS_BYTE_DATA))
    return -EIO;
    data = devm_kzalloc(&client.dev, sizeof(*data), GFP_KERNEL);
    if (!data)
    return -ENOMEM;
    data.chip_info = &pca9532_chip_info_tbl[devid];
    dev_info(&client.dev, "setting platform data\n");
    i2c_set_clientdata(client, data);
    data.client = client;
    mutex_init(&data.update_lock);
    return pca9532_configure(client, data, pca9532_pdata);
    }
#[no_mangle]
unsafe extern "C" fn pca9532_remove(client: *mut i2c_client) {
    static void pca9532_remove(struct i2c_client *client)
    {
    struct pca9532_data *data = i2c_get_clientdata(client);
    pca9532_destroy_devices(data, data.chip_info.num_leds);
    }
    module_i2c_driver(pca9532_driver);
    MODULE_AUTHOR("Riku Voipio");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("PCA 9532 LED dimmer");
