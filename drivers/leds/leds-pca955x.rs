//! Automatically rewritten from C to Rust
//! Source: drivers/leds/leds-pca955x.c
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
// Copyright 2007-2008 Extreme Engineering Solutions, Inc.
//
// Author: Nate Case <ncase@xes-inc.com>
//
// LED driver for various PCA955x I2C LED drivers
//
// Supported devices:
//
// Device		Description		7-bit slave address
// ------		-----------		-------------------
// PCA9550		2-bit driver		0x60 .. 0x61
// PCA9551		8-bit driver		0x60 .. 0x67
// PCA9552		16-bit driver		0x60 .. 0x67
// PCA9553/01	4-bit driver		0x62
// PCA9553/02	4-bit driver		0x63
//
// Philips PCA955x LED driver chips follow a register map as shown below:
//
// Control Register		Description
// ----------------		-----------
// 0x0				Input register 0
// ..
// NUM_INPUT_REGS - 1		Last Input register X
//
// NUM_INPUT_REGS			Frequency prescaler 0
// NUM_INPUT_REGS + 1		PWM register 0
// NUM_INPUT_REGS + 2		Frequency prescaler 1
// NUM_INPUT_REGS + 3		PWM register 1
//
// NUM_INPUT_REGS + 4		LED selector 0
// NUM_INPUT_REGS + 4
// + NUM_LED_REGS - 1		Last LED selector
//
// where NUM_INPUT_REGS and NUM_LED_REGS vary depending on how many
// bits the chip supports.
//

// LED select registers determine the source that drives LED outputs
pub const PCA955X_LS_LED_ON: c_uint = 0x0	/* Output LOW */;
pub const PCA955X_LS_LED_OFF: c_uint = 0x1	/* Output HI-Z */;
pub const PCA955X_LS_BLINK0: c_uint = 0x2	/* Blink at PWM0 rate */;
pub const PCA955X_LS_BLINK1: c_uint = 0x3	/* Blink at PWM1 rate */;

pub const PCA955X_BLINK_DEFAULT_MS: c_int = 1000;
    enum pca955x_type {
    pca9550,
    pca9551,
    pca9552,
    ibm_pca9552,
    pca9553,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pca955x_chipdef {
    pub bits: u8,
    pub /: *mut *mut u8 slv_addr; / 7-bit slave address mask,
    pub /: *mut *mut int slv_addr_shift; / Number of bits to ignore,
    pub /: *mut *mut int blink_div; / PSC divider,
}

    static const struct pca955x_chipdef pca955x_chipdefs[] = {
    [pca9550] = {
    .bits		= 2,
    .slv_addr	= /* 110000x */ 0x60,
    .slv_addr_shift	= 1,
    .blink_div	= 44,
    },
    [pca9551] = {
    .bits		= 8,
    .slv_addr	= /* 1100xxx */ 0x60,
    .slv_addr_shift	= 3,
    .blink_div	= 38,
    },
    [pca9552] = {
    .bits		= 16,
    .slv_addr	= /* 1100xxx */ 0x60,
    .slv_addr_shift	= 3,
    .blink_div	= 44,
    },
    [ibm_pca9552] = {
    .bits		= 16,
    .slv_addr	= /* 0110xxx */ 0x30,
    .slv_addr_shift	= 3,
    .blink_div	= 44,
    },
    [pca9553] = {
    .bits		= 4,
    .slv_addr	= /* 110001x */ 0x62,
    .slv_addr_shift	= 1,
    .blink_div	= 44,
    },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pca955x {
    pub lock: mutex,
    pub leds: *mut pca955x_led,
    pub chipdef: *const pca955x_chipdef,
    pub client: *mut i2c_client,
    pub active_blink: c_ulong,
    pub active_pins: c_ulong,
    pub blink_period: c_ulong,

    pub gpio: gpio_chip,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pca955x_led {
    pub pca955x: *mut pca955x,
    pub led_cdev: led_classdev,
    pub /: *mut *mut int led_num; / 0 .. 15 potentially,
    pub type: u32,
    pub default_state: enum led_default_state,
    pub fwnode: *mut fwnode_handle,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pca955x_platform_data {
    pub leds: *mut pca955x_led,
    pub num_leds: c_int,
}

// 8 bits per input register
#[no_mangle]
pub unsafe extern "C" fn pca955x_num_input_regs(bits: u8) -> u8 {
    static inline u8 pca955x_num_input_regs(u8 bits)
    {
    return (bits + 7) / 8;
    }
// 4 bits per LED selector register
#[no_mangle]
pub unsafe extern "C" fn pca955x_num_led_regs(bits: u8) -> u8 {
    static inline u8 pca955x_num_led_regs(u8 bits)
    {
    return (bits + 3)  / 4;
    }
//
// Return an LED selector register value based on an existing one, with
// the appropriate 2-bit state value set for the given LED number (0-3).
//
#[no_mangle]
pub unsafe extern "C" fn pca955x_ledsel(oldval: u8, led_num: c_int, state: c_int) -> u8 {
    static inline u8 pca955x_ledsel(u8 oldval, int led_num, int state)
    {
    return (oldval & (~(0x3 << (led_num << 1)))) |
    ((state & 0x3) << (led_num << 1));
    }
#[no_mangle]
pub unsafe extern "C" fn pca955x_ledstate(ls: u8, led_num: c_int) -> c_int {
    static inline int pca955x_ledstate(u8 ls, int led_num)
    {
    return (ls >> (led_num << 1)) & 0x3;
    }
//
// Write to frequency prescaler register, used to program the
// period of the PWM output.  period = (PSCx + 1) / coeff
// Where for pca9551 chips coeff = 38 and for all other chips coeff = 44
//
#[no_mangle]
unsafe extern "C" fn pca955x_write_psc(pca955x: *mut pca955x, n: c_int, val: u8) -> c_int {
    static int pca955x_write_psc(struct pca955x *pca955x, int n, u8 val)
    {
    let mut cmd: u8 = pca955x_num_input_regs(pca955x.chipdef.bits) + (2 * n);
    int ret;
    ret = i2c_smbus_write_byte_data(pca955x.client, cmd, val);
    if (ret < 0)
    dev_err(&pca955x.client.dev, "%s: reg 0x%x, val 0x%x, err %d\n", __func__, n,
    val, ret);
    return ret;
    }
//
// Write to PWM register, which determines the duty cycle of the
// output.  LED is OFF when the count is less than the value of this
// register, and ON when it is greater.  If PWMx == 0, LED is always OFF.
//
// Duty cycle is (256 - PWMx) / 256
//
#[no_mangle]
unsafe extern "C" fn pca955x_write_pwm(pca955x: *mut pca955x, n: c_int, val: u8) -> c_int {
    static int pca955x_write_pwm(struct pca955x *pca955x, int n, u8 val)
    {
    let mut cmd: u8 = pca955x_num_input_regs(pca955x.chipdef.bits) + 1 + (2 * n);
    int ret;
    ret = i2c_smbus_write_byte_data(pca955x.client, cmd, val);
    if (ret < 0)
    dev_err(&pca955x.client.dev, "%s: reg 0x%x, val 0x%x, err %d\n", __func__, n,
    val, ret);
    return ret;
    }
//
// Write to LED selector register, which determines the source that
// drives the LED output.
//
#[no_mangle]
unsafe extern "C" fn pca955x_write_ls(pca955x: *mut pca955x, n: c_int, val: u8) -> c_int {
    static int pca955x_write_ls(struct pca955x *pca955x, int n, u8 val)
    {
    let mut cmd: u8 = pca955x_num_input_regs(pca955x.chipdef.bits) + 4 + n;
    int ret;
    ret = i2c_smbus_write_byte_data(pca955x.client, cmd, val);
    if (ret < 0)
    dev_err(&pca955x.client.dev, "%s: reg 0x%x, val 0x%x, err %d\n", __func__, n,
    val, ret);
    return ret;
    }
//
// Read the LED selector register, which determines the source that
// drives the LED output.
//
#[no_mangle]
unsafe extern "C" fn pca955x_read_ls(pca955x: *mut pca955x, n: c_int, val: *mut u8) -> c_int {
    static int pca955x_read_ls(struct pca955x *pca955x, int n, u8 *val)
    {
    let mut cmd: u8 = pca955x_num_input_regs(pca955x.chipdef.bits) + 4 + n;
    int ret;
    ret = i2c_smbus_read_byte_data(pca955x.client, cmd);
    if (ret < 0) {
    dev_err(&pca955x.client.dev, "%s: reg 0x%x, err %d\n", __func__, n, ret);
    return ret;
    }
// val = (u8)ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pca955x_read_pwm(pca955x: *mut pca955x, n: c_int, val: *mut u8) -> c_int {
    static int pca955x_read_pwm(struct pca955x *pca955x, int n, u8 *val)
    {
    let mut cmd: u8 = pca955x_num_input_regs(pca955x.chipdef.bits) + 1 + (2 * n);
    int ret;
    ret = i2c_smbus_read_byte_data(pca955x.client, cmd);
    if (ret < 0) {
    dev_err(&pca955x.client.dev, "%s: reg 0x%x, err %d\n", __func__, n, ret);
    return ret;
    }
// val = (u8)ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pca955x_read_psc(pca955x: *mut pca955x, n: c_int, val: *mut u8) -> c_int {
    static int pca955x_read_psc(struct pca955x *pca955x, int n, u8 *val)
    {
    int ret;
    u8 cmd;
    cmd = pca955x_num_input_regs(pca955x.chipdef.bits) + (2 * n);
    ret = i2c_smbus_read_byte_data(pca955x.client, cmd);
    if (ret < 0) {
    dev_err(&pca955x.client.dev, "%s: reg 0x%x, err %d\n", __func__, n, ret);
    return ret;
    }
// val = (u8)ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pca955x_led_get(led_cdev: *mut led_classdev) -> enum led_brightness {
    static enum led_brightness pca955x_led_get(struct led_classdev *led_cdev)
    {
    struct pca955x_led *pca955x_led = led_to_pca955x(led_cdev);
    struct pca955x *pca955x = pca955x_led.pca955x;
    u8 ls, pwm;
    int ret;
    ret = pca955x_read_ls(pca955x, pca955x_led.led_num / 4, &ls);
    if (ret)
    return ret;
    switch (pca955x_ledstate(ls, pca955x_led.led_num % 4)) {
    case PCA955X_LS_LED_ON:
    case PCA955X_LS_BLINK0:
    ret = LED_FULL;
    break;
    case PCA955X_LS_LED_OFF:
    ret = LED_OFF;
    break;
    case PCA955X_LS_BLINK1:
    ret = pca955x_read_pwm(pca955x, 1, &pwm);
    if (ret)
    return ret;
    ret = 255 - pwm;
    break;
    }
    return ret;
    }
    static int pca955x_led_set(struct led_classdev *led_cdev,
    enum led_brightness value)
    {
    struct pca955x_led *pca955x_led = led_to_pca955x(led_cdev);
    struct pca955x *pca955x = pca955x_led.pca955x;
    let mut reg: c_int = pca955x_led.led_num / 4;
    let mut bit: c_int = pca955x_led.led_num % 4;
    u8 ls;
    int ret;
    mutex_lock(&pca955x.lock);
    ret = pca955x_read_ls(pca955x, reg, &ls);
    if (ret)
    goto out;
    if (test_bit(pca955x_led.led_num, &pca955x.active_blink)) {
    if (value == LED_OFF) {
    clear_bit(pca955x_led.led_num, &pca955x.active_blink);
    ls = pca955x_ledsel(ls, bit, PCA955X_LS_LED_OFF);
    } else {
// No variable brightness for blinking LEDs
    goto out;
    }
    } else {
    switch (value) {
    case LED_FULL:
    ls = pca955x_ledsel(ls, bit, PCA955X_LS_LED_ON);
    break;
    case LED_OFF:
    ls = pca955x_ledsel(ls, bit, PCA955X_LS_LED_OFF);
    break;
    default:
//
// Use PWM1 for all other values. This has the unwanted
// side effect of making all LEDs on the chip share the
// same brightness level if set to a value other than
// OFF or FULL. But, this is probably better than just
// turning off for all other values.
//
    ret = pca955x_write_pwm(pca955x, 1, 255 - value);
    if (ret)
    goto out;
    ls = pca955x_ledsel(ls, bit, PCA955X_LS_BLINK1);
    break;
    }
    }
    ret = pca955x_write_ls(pca955x, reg, ls);
    out:
    mutex_unlock(&pca955x.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pca955x_period_to_psc(pca955x: *mut pca955x, period: c_ulong) -> u8 {
    static u8 pca955x_period_to_psc(struct pca955x *pca955x, unsigned long period)
    {
// psc register value = (blink period * coeff) - 1
    period *= pca955x.chipdef.blink_div;
    period /= MSEC_PER_SEC;
    period -= 1;
    return period;
    }
#[no_mangle]
unsafe extern "C" fn pca955x_psc_to_period(pca955x: *mut pca955x, psc: u8) -> c_ulong {
    static unsigned long pca955x_psc_to_period(struct pca955x *pca955x, u8 psc)
    {
    let mut period: c_ulong = psc;
// blink period = (psc register value + 1) / coeff
    period += 1;
    period *= MSEC_PER_SEC;
    period /= pca955x.chipdef.blink_div;
    return period;
    }
    static int pca955x_led_blink(struct led_classdev *led_cdev,
    unsigned long *delay_on, unsigned long *delay_off)
    {
    struct pca955x_led *pca955x_led = led_to_pca955x(led_cdev);
    struct pca955x *pca955x = pca955x_led.pca955x;
    let mut period: c_ulong = *delay_on + *delay_off;
    let mut ret: c_int = 0;
    mutex_lock(&pca955x.lock);
    if (period) {
    if (*delay_on != *delay_off) {
    ret = -EINVAL;
    goto out;
    }
    if (period < pca955x_psc_to_period(pca955x, 0) ||
    period > pca955x_psc_to_period(pca955x, 0xff)) {
    ret = -EINVAL;
    goto out;
    }
    } else {
    period = pca955x.active_blink ? pca955x.blink_period :
    PCA955X_BLINK_DEFAULT_MS;
    }
    if (!pca955x.active_blink ||
    pca955x.active_blink == BIT(pca955x_led.led_num) ||
    pca955x.blink_period == period) {
    let mut psc: u8 = pca955x_period_to_psc(pca955x, period);
    if (!test_and_set_bit(pca955x_led.led_num,
    &pca955x.active_blink)) {
    u8 ls;
    let mut reg: c_int = pca955x_led.led_num / 4;
    let mut bit: c_int = pca955x_led.led_num % 4;
    ret = pca955x_read_ls(pca955x, reg, &ls);
    if (ret)
    goto out;
    ls = pca955x_ledsel(ls, bit, PCA955X_LS_BLINK0);
    ret = pca955x_write_ls(pca955x, reg, ls);
    if (ret)
    goto out;
//
// Force 50% duty cycle to maintain the specified
// blink rate.
//
    ret = pca955x_write_pwm(pca955x, 0, 128);
    if (ret)
    goto out;
    }
    if (pca955x.blink_period != period) {
    pca955x.blink_period = period;
    ret = pca955x_write_psc(pca955x, 0, psc);
    if (ret)
    goto out;
    }
    period = pca955x_psc_to_period(pca955x, psc);
    period /= 2;
// delay_on = period;
// delay_off = period;
    } else {
    ret = -EBUSY;
    }
    out:
    mutex_unlock(&pca955x.lock);
    return ret;
    }

//
// Read the INPUT register, which contains the state of LEDs.
//
#[no_mangle]
unsafe extern "C" fn pca955x_read_input(client: *mut i2c_client, n: c_int, val: *mut u8) -> c_int {
    static int pca955x_read_input(struct i2c_client *client, int n, u8 *val)
    {
    let mut ret: c_int = i2c_smbus_read_byte_data(client, n);
    if (ret < 0) {
    dev_err(&client.dev, "%s: reg 0x%x, err %d\n",
    __func__, n, ret);
    return ret;
    }
// val = (u8)ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pca955x_gpio_request_pin(gc: *mut gpio_chip, offset: c_uint) -> c_int {
    static int pca955x_gpio_request_pin(struct gpio_chip *gc, unsigned int offset)
    {
    struct pca955x *pca955x = gpiochip_get_data(gc);
    return test_and_set_bit(offset, &pca955x.active_pins) ? -EBUSY : 0;
    }
#[no_mangle]
unsafe extern "C" fn pca955x_gpio_free_pin(gc: *mut gpio_chip, offset: c_uint) {
    static void pca955x_gpio_free_pin(struct gpio_chip *gc, unsigned int offset)
    {
    struct pca955x *pca955x = gpiochip_get_data(gc);
    clear_bit(offset, &pca955x.active_pins);
    }
    static int pca955x_set_value(struct gpio_chip *gc, unsigned int offset,
    int val)
    {
    struct pca955x *pca955x = gpiochip_get_data(gc);
    struct pca955x_led *led = &pca955x.leds[offset];
    if (val)
    return pca955x_led_set(&led.led_cdev, PCA955X_GPIO_HIGH);
    return pca955x_led_set(&led.led_cdev, PCA955X_GPIO_LOW);
    }
    static int pca955x_gpio_set_value(struct gpio_chip *gc, unsigned int offset,
    int val)
    {
    return pca955x_set_value(gc, offset, val);
    }
#[no_mangle]
unsafe extern "C" fn pca955x_gpio_get_value(gc: *mut gpio_chip, offset: c_uint) -> c_int {
    static int pca955x_gpio_get_value(struct gpio_chip *gc, unsigned int offset)
    {
    struct pca955x *pca955x = gpiochip_get_data(gc);
    struct pca955x_led *led = &pca955x.leds[offset];
    let mut reg: u8 = 0;
// There is nothing we can do about errors
    pca955x_read_input(pca955x.client, led.led_num / 8, &reg);
    return !!(reg & (1 << (led.led_num % 8)));
    }
    static int pca955x_gpio_direction_input(struct gpio_chip *gc,
    unsigned int offset)
    {
    struct pca955x *pca955x = gpiochip_get_data(gc);
    struct pca955x_led *led = &pca955x.leds[offset];
// To use as input ensure pin is not driven.
    return pca955x_led_set(&led.led_cdev, PCA955X_GPIO_INPUT);
    }
    static int pca955x_gpio_direction_output(struct gpio_chip *gc,
    unsigned int offset, int val)
    {
    return pca955x_set_value(gc, offset, val);
    }

    static struct pca955x_platform_data *
    pca955x_get_pdata(struct i2c_client *client, const struct pca955x_chipdef *chip)
    {
    struct pca955x_platform_data *pdata;
    struct pca955x_led *led;
    struct fwnode_handle *child;
    int count;
    count = device_get_child_node_count(&client.dev);
    if (count > chip.bits)
    return ERR_PTR(-ENODEV);
    pdata = devm_kzalloc(&client.dev, sizeof(*pdata), GFP_KERNEL);
    if (!pdata)
    return ERR_PTR(-ENOMEM);
    pdata.leds = devm_kcalloc(&client.dev,
    chip.bits, sizeof(struct pca955x_led),
    GFP_KERNEL);
    if (!pdata.leds)
    return ERR_PTR(-ENOMEM);
    device_for_each_child_node(&client.dev, child) {
    u32 reg;
    int res;
    res = fwnode_property_read_u32(child, "reg", &reg);
    if ((res != 0) || (reg >= chip.bits))
    continue;
    led = &pdata.leds[reg];
    led.type = PCA955X_TYPE_LED;
    led.fwnode = child;
    led.default_state = led_init_default_state_get(child);
    fwnode_property_read_u32(child, "type", &led.type);
    }
    pdata.num_leds = chip.bits;
    return pdata;
    }
#[no_mangle]
unsafe extern "C" fn pca955x_probe(client: *mut i2c_client) -> c_int {
    static int pca955x_probe(struct i2c_client *client)
    {
    struct pca955x *pca955x;
    struct pca955x_led *pca955x_led;
    const struct pca955x_chipdef *chip;
    struct led_classdev *led;
    struct led_init_data init_data;
    struct i2c_adapter *adapter;
    u8 i, nls, psc0;
    u8 ls1[4];
    u8 ls2[4];
    struct pca955x_platform_data *pdata;
    let mut keep_psc0: bool = false;
    let mut set_default_label: bool = false;
    char default_label[4];
    int bit, err, reg;
    chip = i2c_get_match_data(client);
    if (!chip)
    return dev_err_probe(&client.dev, -ENODEV, "unknown chip\n");
    adapter = client.adapter;
    pdata = dev_get_platdata(&client.dev);
    if (!pdata) {
    pdata =	pca955x_get_pdata(client, chip);
    if (IS_ERR(pdata))
    return PTR_ERR(pdata);
    }
// Make sure the slave address / chip type combo given is possible
    if ((client.addr & ~((1 << chip.slv_addr_shift) - 1)) !=
    chip.slv_addr) {
    dev_err(&client.dev, "invalid slave address %02x\n",
    client.addr);
    return -ENODEV;
    }
    dev_info(&client.dev, "Using %s %u-bit LED driver at slave address 0x%02x\n",
    client.name, chip.bits, client.addr);
    if (!i2c_check_functionality(adapter, I2C_FUNC_SMBUS_BYTE_DATA))
    return -EIO;
    if (pdata.num_leds != chip.bits) {
    dev_err(&client.dev,
    "board info claims %d LEDs on a %u-bit chip\n",
    pdata.num_leds, chip.bits);
    return -ENODEV;
    }
    pca955x = devm_kzalloc(&client.dev, sizeof(*pca955x), GFP_KERNEL);
    if (!pca955x)
    return -ENOMEM;
    pca955x.leds = devm_kcalloc(&client.dev, chip.bits,
    sizeof(*pca955x_led), GFP_KERNEL);
    if (!pca955x.leds)
    return -ENOMEM;
    i2c_set_clientdata(client, pca955x);
    mutex_init(&pca955x.lock);
    pca955x.client = client;
    pca955x.chipdef = chip;
    pca955x.blink_period = PCA955X_BLINK_DEFAULT_MS;
    init_data.devname_mandatory = false;
    init_data.devicename = "pca955x";
    nls = pca955x_num_led_regs(chip.bits);
// Use auto-increment feature to read all the LED selectors at once.
    err = i2c_smbus_read_i2c_block_data(client,
    0x10 | (pca955x_num_input_regs(chip.bits) + 4), nls,
    ls1);
    if (err < 0)
    return err;
    for (i = 0; i < nls; i++)
    ls2[i] = ls1[i];
    for (i = 0; i < chip.bits; i++) {
    pca955x_led = &pca955x.leds[i];
    pca955x_led.led_num = i;
    pca955x_led.pca955x = pca955x;
    pca955x_led.type = pdata.leds[i].type;
    switch (pca955x_led.type) {
    case PCA955X_TYPE_NONE:
    case PCA955X_TYPE_GPIO:
    break;
    case PCA955X_TYPE_LED:
    bit = i % 4;
    reg = i / 4;
    led = &pca955x_led.led_cdev;
    led.brightness_set_blocking = pca955x_led_set;
    led.brightness_get = pca955x_led_get;
    led.blink_set = pca955x_led_blink;
    if (pdata.leds[i].default_state == LEDS_DEFSTATE_OFF)
    ls2[reg] = pca955x_ledsel(ls2[reg], bit, PCA955X_LS_LED_OFF);
#[no_mangle]
pub unsafe extern "C" fn if(LEDS_DEFSTATE_ON: pdata->leds[i].default_state ==) -> else {
    else if (pdata.leds[i].default_state == LEDS_DEFSTATE_ON)
    ls2[reg] = pca955x_ledsel(ls2[reg], bit, PCA955X_LS_LED_ON);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: pca955x_ledstate(ls2[reg], PCA955X_LS_BLINK0: bit) ==) -> else {
    keep_psc0 = true;
    set_bit(i, &pca955x.active_blink);
    }
    init_data.fwnode = pdata.leds[i].fwnode;
    if (is_of_node(init_data.fwnode)) {
    if (to_of_node(init_data.fwnode).name[0] ==
    '\0')
    set_default_label = true;
    else
    set_default_label = false;
    } else {
    set_default_label = true;
    }
    if (set_default_label) {
    snprintf(default_label, sizeof(default_label), "%hhu", i);
    init_data.default_label = default_label;
    } else {
    init_data.default_label = core::ptr::null_mut();
    }
    err = devm_led_classdev_register_ext(&client.dev, led,
    &init_data);
    if (err)
    return err;
    set_bit(i, &pca955x.active_pins);
    }
    }
    for (i = 0; i < nls; i++) {
    if (ls1[i] != ls2[i]) {
    err = pca955x_write_ls(pca955x, i, ls2[i]);
    if (err)
    return err;
    }
    }
    if (keep_psc0) {
    err = pca955x_read_psc(pca955x, 0, &psc0);
    } else {
    psc0 = pca955x_period_to_psc(pca955x, pca955x.blink_period);
    err = pca955x_write_psc(pca955x, 0, psc0);
    }
    if (err)
    return err;
    pca955x.blink_period = pca955x_psc_to_period(pca955x, psc0);
// Set PWM1 to fast frequency so we do not see flashing
    err = pca955x_write_psc(pca955x, 1, 0);
    if (err)
    return err;

    pca955x.gpio.label = "gpio-pca955x";
    pca955x.gpio.direction_input = pca955x_gpio_direction_input;
    pca955x.gpio.direction_output = pca955x_gpio_direction_output;
    pca955x.gpio.set = pca955x_gpio_set_value;
    pca955x.gpio.get = pca955x_gpio_get_value;
    pca955x.gpio.request = pca955x_gpio_request_pin;
    pca955x.gpio.free = pca955x_gpio_free_pin;
    pca955x.gpio.can_sleep = 1;
    pca955x.gpio.base = -1;
    pca955x.gpio.ngpio = chip.bits;
    pca955x.gpio.parent = &client.dev;
    pca955x.gpio.owner = THIS_MODULE;
    err = devm_gpiochip_add_data(&client.dev, &pca955x.gpio,
    pca955x);
    if (err) {
// Use data->gpio.dev as a flag for freeing gpiochip
    pca955x.gpio.parent = core::ptr::null_mut();
    dev_warn(&client.dev, "could not add gpiochip\n");
    return err;
    }
    dev_info(&client.dev, "gpios %i...%i\n",
    pca955x.gpio.base, pca955x.gpio.base +
    pca955x.gpio.ngpio - 1);

    return 0;
    }
    static const struct i2c_device_id pca955x_id[] = {
    { .name = "pca9550", .driver_data = (kernel_ulong_t)&pca955x_chipdefs[pca9550] },
    { .name = "pca9551", .driver_data = (kernel_ulong_t)&pca955x_chipdefs[pca9551] },
    { .name = "pca9552", .driver_data = (kernel_ulong_t)&pca955x_chipdefs[pca9552] },
    { .name = "ibm-pca9552", .driver_data = (kernel_ulong_t)&pca955x_chipdefs[ibm_pca9552] },
    { .name = "pca9553", .driver_data = (kernel_ulong_t)&pca955x_chipdefs[pca9553] },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, pca955x_id);
    static const struct of_device_id of_pca955x_match[] = {
    { .compatible = "nxp,pca9550", .data = &pca955x_chipdefs[pca9550] },
    { .compatible = "nxp,pca9551", .data = &pca955x_chipdefs[pca9551] },
    { .compatible = "nxp,pca9552", .data = &pca955x_chipdefs[pca9552] },
    { .compatible = "ibm,pca9552", .data = &pca955x_chipdefs[ibm_pca9552] },
    { .compatible = "nxp,pca9553", .data = &pca955x_chipdefs[pca9553] },
    {}
    };
    MODULE_DEVICE_TABLE(of, of_pca955x_match);
    static struct i2c_driver pca955x_driver = {
    .driver = {
    .name	= "leds-pca955x",
    .of_match_table = of_pca955x_match,
    },
    .probe = pca955x_probe,
    .id_table = pca955x_id,
    };
    module_i2c_driver(pca955x_driver);
    MODULE_AUTHOR("Nate Case <ncase@xes-inc.com>");
    MODULE_DESCRIPTION("PCA955x LED driver");
    MODULE_LICENSE("GPL v2");
