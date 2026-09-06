//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-twl4030.c
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
// Access to GPIOs on TWL4030/TPS659x0 chips
//
// Copyright (C) 2006-2007 Texas Instruments, Inc.
// Copyright (C) 2006 MontaVista Software, Inc.
//
// Code re-arranged and cleaned up by:
// Syed Mohammed Khasim <x0khasim@ti.com>
//
// Initial Code:
// Andy Lowe / Nishanth Menon
//

//
// The GPIO "subchip" supports 18 GPIOs which can be configured as
// inputs or outputs, with pullups or pulldowns on each pin.  Each
// GPIO can trigger interrupts on either or both edges.
//
// GPIO interrupts can be fed to either of two IRQ lines; this is
// intended to support multiple hosts.
//
// There are also two LED pins used sometimes as output-only GPIOs.
//
// genirq interfaces are not available to modules

// GPIO_CTRL Fields

// Mask for GPIO registers when aggregated into a 32-bit integer
pub const GPIO_32_MASK: c_uint = 0x0003ffff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_twl4030_priv {
    pub gpio_chip: gpio_chip,
    pub mutex: mutex,
    pub irq_base: c_int,
// Bitfields for state caching
    pub usage_count: c_uint,
    pub direction: c_uint,
    pub out_state: c_uint,
}

// ----------------------------------------------------------------------
//
// To configure TWL4030 GPIO module registers
//
#[no_mangle]
pub unsafe extern "C" fn gpio_twl4030_write(address: u8, data: u8) -> c_int {
    static inline int gpio_twl4030_write(u8 address, u8 data)
    {
    return twl_i2c_write_u8(TWL4030_MODULE_GPIO, data, address);
    }
// ----------------------------------------------------------------------
//
// LED register offsets from TWL_MODULE_LED base
// PWMs A and B are dedicated to LEDs A and B, respectively.
//
pub const TWL4030_LED_LEDEN_REG: c_uint = 0x00;
pub const TWL4030_PWMAON_REG: c_uint = 0x01;
pub const TWL4030_PWMAOFF_REG: c_uint = 0x02;
pub const TWL4030_PWMBON_REG: c_uint = 0x03;
pub const TWL4030_PWMBOFF_REG: c_uint = 0x04;
// LEDEN bits

// ----------------------------------------------------------------------
//
// To read a TWL4030 GPIO module register
//
#[no_mangle]
pub unsafe extern "C" fn gpio_twl4030_read(address: u8) -> c_int {
    static inline int gpio_twl4030_read(u8 address)
    {
    u8 data;
    let mut ret: c_int = 0;
    ret = twl_i2c_read_u8(TWL4030_MODULE_GPIO, &data, address);
    return (ret < 0) ? ret : data;
    }
// ----------------------------------------------------------------------
    static u8 cached_leden;
// The LED lines are open drain outputs ... a FET pulls to GND, so an
// external pullup is needed.  We could also expose the integrated PWM
// as a LED brightness control; we initialize it as "always on".
//
#[no_mangle]
unsafe extern "C" fn twl4030_led_set_value(led: c_int, value: c_int) -> c_int {
    static int twl4030_led_set_value(int led, int value)
    {
    let mut mask: u8 = LEDEN_LEDAON | LEDEN_LEDAPWM;
    if (led)
    mask <<= 1;
    if (value)
    cached_leden &= ~mask;
    else
    cached_leden |= mask;
    return twl_i2c_write_u8(TWL4030_MODULE_LED, cached_leden,
    TWL4030_LED_LEDEN_REG);
    }
#[no_mangle]
unsafe extern "C" fn twl4030_set_gpio_direction(gpio: c_int, is_input: c_int) -> c_int {
    static int twl4030_set_gpio_direction(int gpio, int is_input)
    {
    let mut d_bnk: u8 = gpio >> 3;
    let mut d_msk: u8 = BIT(gpio & 0x7);
    let mut reg: u8 = 0;
    let mut base: u8 = REG_GPIODATADIR1 + d_bnk;
    let mut ret: c_int = 0;
    ret = gpio_twl4030_read(base);
    if (ret >= 0) {
    if (is_input)
    reg = ret & ~d_msk;
    else
    reg = ret | d_msk;
    ret = gpio_twl4030_write(base, reg);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn twl4030_get_gpio_direction(gpio: c_int) -> c_int {
    static int twl4030_get_gpio_direction(int gpio)
    {
    let mut d_bnk: u8 = gpio >> 3;
    let mut d_msk: u8 = BIT(gpio & 0x7);
    let mut base: u8 = REG_GPIODATADIR1 + d_bnk;
    let mut ret: c_int = 0;
    ret = gpio_twl4030_read(base);
    if (ret < 0)
    return ret;
    if (ret & d_msk)
    return GPIO_LINE_DIRECTION_OUT;
    return GPIO_LINE_DIRECTION_IN;
    }
#[no_mangle]
unsafe extern "C" fn twl4030_set_gpio_dataout(gpio: c_int, enable: c_int) -> c_int {
    static int twl4030_set_gpio_dataout(int gpio, int enable)
    {
    let mut d_bnk: u8 = gpio >> 3;
    let mut d_msk: u8 = BIT(gpio & 0x7);
    let mut base: u8 = 0;
    if (enable)
    base = REG_SETGPIODATAOUT1 + d_bnk;
    else
    base = REG_CLEARGPIODATAOUT1 + d_bnk;
    return gpio_twl4030_write(base, d_msk);
    }
#[no_mangle]
unsafe extern "C" fn twl4030_get_gpio_datain(gpio: c_int) -> c_int {
    static int twl4030_get_gpio_datain(int gpio)
    {
    let mut d_bnk: u8 = gpio >> 3;
    let mut d_off: u8 = gpio & 0x7;
    let mut base: u8 = 0;
    let mut ret: c_int = 0;
    base = REG_GPIODATAIN1 + d_bnk;
    ret = gpio_twl4030_read(base);
    if (ret > 0)
    ret = (ret >> d_off) & 0x1;
    return ret;
    }
// ----------------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn twl_request(chip: *mut gpio_chip, offset: unsigned) -> c_int {
    static int twl_request(struct gpio_chip *chip, unsigned offset)
    {
    struct gpio_twl4030_priv *priv = gpiochip_get_data(chip);
    let mut status: c_int = 0;
    mutex_lock(&priv.mutex);
// Support the two LED outputs as output-only GPIOs.
    if (offset >= TWL4030_GPIO_MAX) {
    u8	ledclr_mask = LEDEN_LEDAON | LEDEN_LEDAEXT
    | LEDEN_LEDAPWM | LEDEN_PWM_LENGTHA;
    let mut reg: u8 = TWL4030_PWMAON_REG;
    offset -= TWL4030_GPIO_MAX;
    if (offset) {
    ledclr_mask <<= 1;
    reg = TWL4030_PWMBON_REG;
    }
// initialize PWM to always-drive
// Configure PWM OFF register first
    status = twl_i2c_write_u8(TWL4030_MODULE_LED, 0x7f, reg + 1);
    if (status < 0)
    goto done;
// Followed by PWM ON register
    status = twl_i2c_write_u8(TWL4030_MODULE_LED, 0x7f, reg);
    if (status < 0)
    goto done;
// init LED to not-driven (high)
    status = twl_i2c_read_u8(TWL4030_MODULE_LED, &cached_leden,
    TWL4030_LED_LEDEN_REG);
    if (status < 0)
    goto done;
    cached_leden &= ~ledclr_mask;
    status = twl_i2c_write_u8(TWL4030_MODULE_LED, cached_leden,
    TWL4030_LED_LEDEN_REG);
    if (status < 0)
    goto done;
    status = 0;
    goto done;
    }
// on first use, turn GPIO module "on"
    if (!priv.usage_count) {
    struct twl4030_gpio_platform_data *pdata;
    let mut value: u8 = MASK_GPIO_CTRL_GPIO_ON;
// optionally have the first two GPIOs switch vMMC1
// and vMMC2 power supplies based on card presence.
//
    pdata = dev_get_platdata(chip.parent);
    if (pdata)
    value |= pdata.mmc_cd & 0x03;
    status = gpio_twl4030_write(REG_GPIO_CTRL, value);
    }
    done:
    if (!status)
    priv.usage_count |= BIT(offset);
    mutex_unlock(&priv.mutex);
    return status;
    }
#[no_mangle]
unsafe extern "C" fn twl_free(chip: *mut gpio_chip, offset: unsigned) {
    static void twl_free(struct gpio_chip *chip, unsigned offset)
    {
    struct gpio_twl4030_priv *priv = gpiochip_get_data(chip);
    mutex_lock(&priv.mutex);
    if (offset >= TWL4030_GPIO_MAX) {
    WARN_ON_ONCE(twl4030_led_set_value(offset - TWL4030_GPIO_MAX, 1));
    goto out;
    }
    priv.usage_count &= ~BIT(offset);
// on last use, switch off GPIO module
    if (!priv.usage_count)
    gpio_twl4030_write(REG_GPIO_CTRL, 0x0);
    out:
    mutex_unlock(&priv.mutex);
    }
#[no_mangle]
unsafe extern "C" fn twl_direction_in(chip: *mut gpio_chip, offset: unsigned) -> c_int {
    static int twl_direction_in(struct gpio_chip *chip, unsigned offset)
    {
    struct gpio_twl4030_priv *priv = gpiochip_get_data(chip);
    int ret;
    mutex_lock(&priv.mutex);
    if (offset < TWL4030_GPIO_MAX)
    ret = twl4030_set_gpio_direction(offset, 1);
    else
    ret = -EINVAL;	/* LED outputs can't be set as input */
    if (!ret)
    priv.direction &= ~BIT(offset);
    mutex_unlock(&priv.mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn twl_get(chip: *mut gpio_chip, offset: unsigned) -> c_int {
    static int twl_get(struct gpio_chip *chip, unsigned offset)
    {
    struct gpio_twl4030_priv *priv = gpiochip_get_data(chip);
    int ret;
    let mut status: c_int = 0;
    mutex_lock(&priv.mutex);
    if (!(priv.usage_count & BIT(offset))) {
    ret = -EPERM;
    goto out;
    }
    if (priv.direction & BIT(offset))
    status = priv.out_state & BIT(offset);
    else
    status = twl4030_get_gpio_datain(offset);
    ret = (status < 0) ? status : !!status;
    out:
    mutex_unlock(&priv.mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn twl_set(chip: *mut gpio_chip, offset: c_uint, value: c_int) -> c_int {
    static int twl_set(struct gpio_chip *chip, unsigned int offset, int value)
    {
    struct gpio_twl4030_priv *priv = gpiochip_get_data(chip);
    int ret;
    mutex_lock(&priv.mutex);
    if (offset < TWL4030_GPIO_MAX)
    ret = twl4030_set_gpio_dataout(offset, value);
    else
    ret = twl4030_led_set_value(offset - TWL4030_GPIO_MAX, value);
    if (value)
    priv.out_state |= BIT(offset);
    else
    priv.out_state &= ~BIT(offset);
    mutex_unlock(&priv.mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn twl_direction_out(chip: *mut gpio_chip, offset: unsigned, value: c_int) -> c_int {
    static int twl_direction_out(struct gpio_chip *chip, unsigned offset, int value)
    {
    struct gpio_twl4030_priv *priv = gpiochip_get_data(chip);
    let mut ret: c_int = 0;
    mutex_lock(&priv.mutex);
    if (offset < TWL4030_GPIO_MAX) {
    ret = twl4030_set_gpio_direction(offset, 0);
    if (ret) {
    mutex_unlock(&priv.mutex);
    return ret;
    }
    }
//
// LED gpios i.e. offset >= TWL4030_GPIO_MAX are always output
//
    priv.direction |= BIT(offset);
    mutex_unlock(&priv.mutex);
    return twl_set(chip, offset, value);
    }
#[no_mangle]
unsafe extern "C" fn twl_get_direction(chip: *mut gpio_chip, offset: unsigned) -> c_int {
    static int twl_get_direction(struct gpio_chip *chip, unsigned offset)
    {
    struct gpio_twl4030_priv *priv = gpiochip_get_data(chip);
//
// Default GPIO_LINE_DIRECTION_OUT
// LED GPIOs >= TWL4030_GPIO_MAX are always output
//
    let mut ret: c_int = GPIO_LINE_DIRECTION_OUT;
    mutex_lock(&priv.mutex);
    if (offset < TWL4030_GPIO_MAX) {
    ret = twl4030_get_gpio_direction(offset);
    if (ret) {
    mutex_unlock(&priv.mutex);
    return ret;
    }
    }
    mutex_unlock(&priv.mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn twl_to_irq(chip: *mut gpio_chip, offset: unsigned) -> c_int {
    static int twl_to_irq(struct gpio_chip *chip, unsigned offset)
    {
    struct gpio_twl4030_priv *priv = gpiochip_get_data(chip);
    return (priv.irq_base && (offset < TWL4030_GPIO_MAX))
    ? (priv.irq_base + offset)
    : -EINVAL;
    }
    static const struct gpio_chip template_chip = {
    .label			= "twl4030",
    .owner			= THIS_MODULE,
    .request		= twl_request,
    .free			= twl_free,
    .direction_input	= twl_direction_in,
    .direction_output	= twl_direction_out,
    .get_direction		= twl_get_direction,
    .get			= twl_get,
    .set			= twl_set,
    .to_irq			= twl_to_irq,
    .can_sleep		= true,
    };
// ----------------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn gpio_twl4030_pulls(ups: u32, downs: u32) -> c_int {
    static int gpio_twl4030_pulls(u32 ups, u32 downs)
    {
    u8		message[5];
    unsigned	i, gpio_bit;
// For most pins, a pulldown was enabled by default.
// We should have data that's specific to this board.
//
    for (gpio_bit = 1, i = 0; i < 5; i++) {
    u8		bit_mask;
    unsigned	j;
    for (bit_mask = 0, j = 0; j < 8; j += 2, gpio_bit <<= 1) {
    if (ups & gpio_bit)
    bit_mask |= 1 << (j + 1);
#[no_mangle]
pub unsafe extern "C" fn if(gpio_bit: downs &) -> else {
    else if (downs & gpio_bit)
    bit_mask |= 1 << (j + 0);
    }
    message[i] = bit_mask;
    }
    return twl_i2c_write(TWL4030_MODULE_GPIO, message,
    REG_GPIOPUPDCTR1, 5);
    }
#[no_mangle]
unsafe extern "C" fn gpio_twl4030_debounce(debounce: u32, mmc_cd: u8) -> c_int {
    static int gpio_twl4030_debounce(u32 debounce, u8 mmc_cd)
    {
    u8		message[3];
// 30 msec of debouncing is always used for MMC card detect,
// and is optional for everything else.
//
    message[0] = (debounce & 0xff) | (mmc_cd & 0x03);
    debounce >>= 8;
    message[1] = (debounce & 0xff);
    debounce >>= 8;
    message[2] = (debounce & 0x03);
    return twl_i2c_write(TWL4030_MODULE_GPIO, message,
    REG_GPIO_DEBEN1, 3);
    }
    static struct twl4030_gpio_platform_data *of_gpio_twl4030(struct device *dev)
    {
    struct twl4030_gpio_platform_data *omap_twl_info;
    omap_twl_info = devm_kzalloc(dev, sizeof(*omap_twl_info), GFP_KERNEL);
    if (!omap_twl_info)
    return core::ptr::null_mut();
    omap_twl_info.use_leds = of_property_read_bool(dev.of_node,
    "ti,use-leds");
    of_property_read_u32(dev.of_node, "ti,debounce",
    &omap_twl_info.debounce);
    of_property_read_u32(dev.of_node, "ti,mmc-cd",
    (u32 *)&omap_twl_info.mmc_cd);
    of_property_read_u32(dev.of_node, "ti,pullups",
    &omap_twl_info.pullups);
    of_property_read_u32(dev.of_node, "ti,pulldowns",
    &omap_twl_info.pulldowns);
    return omap_twl_info;
    }
// Called from the registered devm action
#[no_mangle]
unsafe extern "C" fn gpio_twl4030_power_off_action(data: *mut c_void) {
    static void gpio_twl4030_power_off_action(void *data)
    {
    struct gpio_desc *d = data;
    gpiod_unexport(d);
    gpiochip_free_own_desc(d);
    }
#[no_mangle]
unsafe extern "C" fn gpio_twl4030_probe(pdev: *mut platform_device) -> c_int {
    static int gpio_twl4030_probe(struct platform_device *pdev)
    {
    struct twl4030_gpio_platform_data *pdata;
    struct gpio_twl4030_priv *priv;
    int ret, irq_base;
    priv = devm_kzalloc(&pdev.dev, sizeof(struct gpio_twl4030_priv),
    GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
// maybe setup IRQs
    if (is_module()) {
    dev_err(&pdev.dev, "can't dispatch IRQs from modules\n");
    goto no_irqs;
    }
    irq_base = devm_irq_alloc_descs(&pdev.dev, -1,
    0, TWL4030_GPIO_MAX, 0);
    if (irq_base < 0) {
    dev_err(&pdev.dev, "Failed to alloc irq_descs\n");
    return irq_base;
    }
    irq_domain_create_legacy(dev_fwnode(&pdev.dev), TWL4030_GPIO_MAX, irq_base, 0,
    &irq_domain_simple_ops, core::ptr::null_mut());
    ret = twl4030_sih_setup(&pdev.dev, TWL4030_MODULE_GPIO, irq_base);
    if (ret < 0)
    return ret;
    priv.irq_base = irq_base;
    no_irqs:
    priv.gpio_chip = template_chip;
    priv.gpio_chip.base = -1;
    priv.gpio_chip.ngpio = TWL4030_GPIO_MAX;
    priv.gpio_chip.parent = &pdev.dev;
    mutex_init(&priv.mutex);
    pdata = of_gpio_twl4030(&pdev.dev);
    if (pdata == core::ptr::null_mut()) {
    dev_err(&pdev.dev, "Platform data is missing\n");
    return -ENXIO;
    }
//
// NOTE:  boards may waste power if they don't set pullups
// and pulldowns correctly ... default for non-ULPI pins is
// pulldown, and some other pins may have external pullups
// or pulldowns.  Careful!
//
    ret = gpio_twl4030_pulls(pdata.pullups, pdata.pulldowns);
    if (ret)
    dev_dbg(&pdev.dev, "pullups %.05x %.05x -. %d\n",
    pdata.pullups, pdata.pulldowns, ret);
    ret = gpio_twl4030_debounce(pdata.debounce, pdata.mmc_cd);
    if (ret)
    dev_dbg(&pdev.dev, "debounce %.03x %.01x -. %d\n",
    pdata.debounce, pdata.mmc_cd, ret);
//
// NOTE: we assume VIBRA_CTL.VIBRA_EN, in MODULE_AUDIO_VOICE,
// is (still) clear if use_leds is set.
//
    if (pdata.use_leds)
    priv.gpio_chip.ngpio += 2;
    ret = devm_gpiochip_add_data(&pdev.dev, &priv.gpio_chip, priv);
    if (ret < 0) {
    dev_err(&pdev.dev, "could not register gpiochip, %d\n", ret);
    priv.gpio_chip.ngpio = 0;
    return ret;
    }
//
// Special quirk for the OMAP3 to hog and export a WLAN power
// GPIO.
//
    if (IS_ENABLED(CONFIG_ARCH_OMAP3) &&
    of_machine_is_compatible("compulab,omap3-sbc-t3730")) {
    struct gpio_desc *d;
    d = gpiochip_request_own_desc(&priv.gpio_chip,
    2, "wlan pwr",
    GPIO_ACTIVE_HIGH,
    GPIOD_OUT_HIGH);
    if (IS_ERR(d))
    return dev_err_probe(&pdev.dev, PTR_ERR(d),
    "unable to hog wlan pwr GPIO\n");
    gpiod_export(d, 0);
    ret = devm_add_action_or_reset(&pdev.dev, gpio_twl4030_power_off_action, d);
    if (ret)
    return ret;
    }
    return 0;
    }
    static const struct of_device_id twl_gpio_match[] = {
    { .compatible = "ti,twl4030-gpio", },
    { },
    };
    MODULE_DEVICE_TABLE(of, twl_gpio_match);
// Note:  this hardware lives inside an I2C-based multi-function device.
    MODULE_ALIAS("platform:twl4030_gpio");
    static struct platform_driver gpio_twl4030_driver = {
    .driver = {
    .name	= "twl4030_gpio",
    .of_match_table = twl_gpio_match,
    },
    .probe		= gpio_twl4030_probe,
    };
#[no_mangle]
unsafe extern "C" fn gpio_twl4030_init() -> int __init {
    static int __init gpio_twl4030_init(void)
    {
    return platform_driver_register(&gpio_twl4030_driver);
    }
    subsys_initcall(gpio_twl4030_init);
#[no_mangle]
unsafe extern "C" fn gpio_twl4030_exit() -> void __exit {
    static void __exit gpio_twl4030_exit(void)
    {
    platform_driver_unregister(&gpio_twl4030_driver);
    }
    module_exit(gpio_twl4030_exit);
    MODULE_AUTHOR("Texas Instruments, Inc.");
    MODULE_DESCRIPTION("GPIO interface for TWL4030");
    MODULE_LICENSE("GPL");
