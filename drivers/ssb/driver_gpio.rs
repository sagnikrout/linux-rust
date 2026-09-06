//! Automatically rewritten from C to Rust
//! Source: drivers/ssb/driver_gpio.c
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


//
// Sonics Silicon Backplane
// GPIO driver
//
// Copyright 2011, Broadcom Corporation
// Copyright 2012, Hauke Mehrtens <hauke@hauke-m.de>
//
// Licensed under the GNU/GPL. See COPYING for details.
//

    const struct software_node ssb_gpio_swnode = {
    .name = "ssb-gpio",
    };
    EXPORT_SYMBOL_GPL(ssb_gpio_swnode);
//
// Shared
//

#[no_mangle]
unsafe extern "C" fn ssb_gpio_to_irq(chip: *mut gpio_chip, gpio: c_uint) -> c_int {
    static int ssb_gpio_to_irq(struct gpio_chip *chip, unsigned int gpio)
    {
    struct ssb_bus *bus = gpiochip_get_data(chip);
    if (bus.bustype == SSB_BUSTYPE_SSB)
    return irq_find_mapping(bus.irq_domain, gpio);
    else
    return -EINVAL;
    }

//
// ChipCommon
//
#[no_mangle]
unsafe extern "C" fn ssb_gpio_chipco_get_value(chip: *mut gpio_chip, gpio: c_uint) -> c_int {
    static int ssb_gpio_chipco_get_value(struct gpio_chip *chip, unsigned int gpio)
    {
    struct ssb_bus *bus = gpiochip_get_data(chip);
    return !!ssb_chipco_gpio_in(&bus.chipco, 1 << gpio);
    }
    static int ssb_gpio_chipco_set_value(struct gpio_chip *chip, unsigned int gpio,
    int value)
    {
    struct ssb_bus *bus = gpiochip_get_data(chip);
    ssb_chipco_gpio_out(&bus.chipco, 1 << gpio, value ? 1 << gpio : 0);
    return 0;
    }
    static int ssb_gpio_chipco_direction_input(struct gpio_chip *chip,
    unsigned int gpio)
    {
    struct ssb_bus *bus = gpiochip_get_data(chip);
    ssb_chipco_gpio_outen(&bus.chipco, 1 << gpio, 0);
    return 0;
    }
    static int ssb_gpio_chipco_direction_output(struct gpio_chip *chip,
    unsigned int gpio, int value)
    {
    struct ssb_bus *bus = gpiochip_get_data(chip);
    ssb_chipco_gpio_outen(&bus.chipco, 1 << gpio, 1 << gpio);
    ssb_chipco_gpio_out(&bus.chipco, 1 << gpio, value ? 1 << gpio : 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ssb_gpio_chipco_request(chip: *mut gpio_chip, gpio: c_uint) -> c_int {
    static int ssb_gpio_chipco_request(struct gpio_chip *chip, unsigned int gpio)
    {
    struct ssb_bus *bus = gpiochip_get_data(chip);
    ssb_chipco_gpio_control(&bus.chipco, 1 << gpio, 0);
// clear pulldown
    ssb_chipco_gpio_pulldown(&bus.chipco, 1 << gpio, 0);
// Set pullup
    ssb_chipco_gpio_pullup(&bus.chipco, 1 << gpio, 1 << gpio);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ssb_gpio_chipco_free(chip: *mut gpio_chip, gpio: c_uint) {
    static void ssb_gpio_chipco_free(struct gpio_chip *chip, unsigned int gpio)
    {
    struct ssb_bus *bus = gpiochip_get_data(chip);
// clear pullup
    ssb_chipco_gpio_pullup(&bus.chipco, 1 << gpio, 0);
    }

#[no_mangle]
unsafe extern "C" fn ssb_gpio_irq_chipco_mask(d: *mut irq_data) {
    static void ssb_gpio_irq_chipco_mask(struct irq_data *d)
    {
    struct ssb_bus *bus = irq_data_get_irq_chip_data(d);
    let mut gpio: c_int = irqd_to_hwirq(d);
    ssb_chipco_gpio_intmask(&bus.chipco, BIT(gpio), 0);
    }
#[no_mangle]
unsafe extern "C" fn ssb_gpio_irq_chipco_unmask(d: *mut irq_data) {
    static void ssb_gpio_irq_chipco_unmask(struct irq_data *d)
    {
    struct ssb_bus *bus = irq_data_get_irq_chip_data(d);
    let mut gpio: c_int = irqd_to_hwirq(d);
    let mut val: u32 = ssb_chipco_gpio_in(&bus.chipco, BIT(gpio));
    ssb_chipco_gpio_polarity(&bus.chipco, BIT(gpio), val);
    ssb_chipco_gpio_intmask(&bus.chipco, BIT(gpio), BIT(gpio));
    }
    static struct irq_chip ssb_gpio_irq_chipco_chip = {
    .name		= "SSB-GPIO-CC",
    .irq_mask	= ssb_gpio_irq_chipco_mask,
    .irq_unmask	= ssb_gpio_irq_chipco_unmask,
    };
#[no_mangle]
unsafe extern "C" fn ssb_gpio_irq_chipco_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t ssb_gpio_irq_chipco_handler(int irq, void *dev_id)
    {
    struct ssb_bus *bus = dev_id;
    struct ssb_chipcommon *chipco = &bus.chipco;
    let mut val: u32 = chipco_read32(chipco, SSB_CHIPCO_GPIOIN);
    let mut mask: u32 = chipco_read32(chipco, SSB_CHIPCO_GPIOIRQ);
    let mut pol: u32 = chipco_read32(chipco, SSB_CHIPCO_GPIOPOL);
    let mut irqs: c_ulong = (val ^ pol) & mask;
    int gpio;
    if (!irqs)
    return IRQ_NONE;
    for_each_set_bit(gpio, &irqs, bus.gpio.ngpio)
    generic_handle_domain_irq_safe(bus.irq_domain, gpio);
    ssb_chipco_gpio_polarity(chipco, irqs, val & irqs);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn ssb_gpio_irq_chipco_domain_init(bus: *mut ssb_bus) -> c_int {
    static int ssb_gpio_irq_chipco_domain_init(struct ssb_bus *bus)
    {
    struct ssb_chipcommon *chipco = &bus.chipco;
    struct gpio_chip *chip = &bus.gpio;
    int gpio, hwirq, err;
    if (bus.bustype != SSB_BUSTYPE_SSB)
    return 0;
    bus.irq_domain = irq_domain_create_linear(core::ptr::null_mut(), chip.ngpio, &irq_domain_simple_ops,
    chipco);
    if (!bus.irq_domain) {
    err = -ENODEV;
    goto err_irq_domain;
    }
    for (gpio = 0; gpio < chip.ngpio; gpio++) {
    let mut irq: c_int = irq_create_mapping(bus.irq_domain, gpio);
    irq_set_chip_data(irq, bus);
    irq_set_chip_and_handler(irq, &ssb_gpio_irq_chipco_chip,
    handle_simple_irq);
    }
    hwirq = ssb_mips_irq(bus.chipco.dev) + 2;
    err = request_irq(hwirq, ssb_gpio_irq_chipco_handler, IRQF_SHARED,
    "gpio", bus);
    if (err)
    goto err_req_irq;
    ssb_chipco_gpio_intmask(&bus.chipco, ~0, 0);
    chipco_set32(chipco, SSB_CHIPCO_IRQMASK, SSB_CHIPCO_IRQ_GPIO);
    return 0;
    err_req_irq:
    for (gpio = 0; gpio < chip.ngpio; gpio++) {
    let mut irq: c_int = irq_find_mapping(bus.irq_domain, gpio);
    irq_dispose_mapping(irq);
    }
    irq_domain_remove(bus.irq_domain);
    err_irq_domain:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ssb_gpio_irq_chipco_domain_exit(bus: *mut ssb_bus) {
    static void ssb_gpio_irq_chipco_domain_exit(struct ssb_bus *bus)
    {
    struct ssb_chipcommon *chipco = &bus.chipco;
    struct gpio_chip *chip = &bus.gpio;
    int gpio;
    if (bus.bustype != SSB_BUSTYPE_SSB)
    return;
    chipco_mask32(chipco, SSB_CHIPCO_IRQMASK, ~SSB_CHIPCO_IRQ_GPIO);
    free_irq(ssb_mips_irq(bus.chipco.dev) + 2, chipco);
    for (gpio = 0; gpio < chip.ngpio; gpio++) {
    let mut irq: c_int = irq_find_mapping(bus.irq_domain, gpio);
    irq_dispose_mapping(irq);
    }
    irq_domain_remove(bus.irq_domain);
    }

#[no_mangle]
unsafe extern "C" fn ssb_gpio_irq_chipco_domain_init(bus: *mut ssb_bus) -> c_int {
    static int ssb_gpio_irq_chipco_domain_init(struct ssb_bus *bus)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ssb_gpio_irq_chipco_domain_exit(bus: *mut ssb_bus) {
    static void ssb_gpio_irq_chipco_domain_exit(struct ssb_bus *bus)
    {
    }

#[no_mangle]
unsafe extern "C" fn ssb_gpio_chipco_init(bus: *mut ssb_bus) -> c_int {
    static int ssb_gpio_chipco_init(struct ssb_bus *bus)
    {
    struct gpio_chip *chip = &bus.gpio;
    int err;
    chip.label		= "ssb_chipco_gpio";
    chip.owner		= THIS_MODULE;
    chip.request		= ssb_gpio_chipco_request;
    chip.free		= ssb_gpio_chipco_free;
    chip.get		= ssb_gpio_chipco_get_value;
    chip.set		= ssb_gpio_chipco_set_value;
    chip.direction_input	= ssb_gpio_chipco_direction_input;
    chip.direction_output	= ssb_gpio_chipco_direction_output;

    chip.to_irq		= ssb_gpio_to_irq;

    chip.ngpio		= 16;
    if (bus.bustype == SSB_BUSTYPE_SSB)
    chip.fwnode	= software_node_fwnode(&ssb_gpio_swnode);
// There is just one SoC in one device and its GPIO addresses should be
// deterministic to address them more easily. The other buses could get
// a random base number.
//
    if (bus.bustype == SSB_BUSTYPE_SSB)
    chip.base		= 0;
    else
    chip.base		= -1;
    err = ssb_gpio_irq_chipco_domain_init(bus);
    if (err)
    return err;
    err = gpiochip_add_data(chip, bus);
    if (err) {
    ssb_gpio_irq_chipco_domain_exit(bus);
    return err;
    }
    return 0;
    }
//
// EXTIF
//

#[no_mangle]
unsafe extern "C" fn ssb_gpio_extif_get_value(chip: *mut gpio_chip, gpio: c_uint) -> c_int {
    static int ssb_gpio_extif_get_value(struct gpio_chip *chip, unsigned int gpio)
    {
    struct ssb_bus *bus = gpiochip_get_data(chip);
    return !!ssb_extif_gpio_in(&bus.extif, 1 << gpio);
    }
    static int ssb_gpio_extif_set_value(struct gpio_chip *chip, unsigned int gpio,
    int value)
    {
    struct ssb_bus *bus = gpiochip_get_data(chip);
    ssb_extif_gpio_out(&bus.extif, 1 << gpio, value ? 1 << gpio : 0);
    return 0;
    }
    static int ssb_gpio_extif_direction_input(struct gpio_chip *chip,
    unsigned int gpio)
    {
    struct ssb_bus *bus = gpiochip_get_data(chip);
    ssb_extif_gpio_outen(&bus.extif, 1 << gpio, 0);
    return 0;
    }
    static int ssb_gpio_extif_direction_output(struct gpio_chip *chip,
    unsigned int gpio, int value)
    {
    struct ssb_bus *bus = gpiochip_get_data(chip);
    ssb_extif_gpio_outen(&bus.extif, 1 << gpio, 1 << gpio);
    ssb_extif_gpio_out(&bus.extif, 1 << gpio, value ? 1 << gpio : 0);
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn ssb_gpio_irq_extif_mask(d: *mut irq_data) {
    static void ssb_gpio_irq_extif_mask(struct irq_data *d)
    {
    struct ssb_bus *bus = irq_data_get_irq_chip_data(d);
    let mut gpio: c_int = irqd_to_hwirq(d);
    ssb_extif_gpio_intmask(&bus.extif, BIT(gpio), 0);
    }
#[no_mangle]
unsafe extern "C" fn ssb_gpio_irq_extif_unmask(d: *mut irq_data) {
    static void ssb_gpio_irq_extif_unmask(struct irq_data *d)
    {
    struct ssb_bus *bus = irq_data_get_irq_chip_data(d);
    let mut gpio: c_int = irqd_to_hwirq(d);
    let mut val: u32 = ssb_extif_gpio_in(&bus.extif, BIT(gpio));
    ssb_extif_gpio_polarity(&bus.extif, BIT(gpio), val);
    ssb_extif_gpio_intmask(&bus.extif, BIT(gpio), BIT(gpio));
    }
    static struct irq_chip ssb_gpio_irq_extif_chip = {
    .name		= "SSB-GPIO-EXTIF",
    .irq_mask	= ssb_gpio_irq_extif_mask,
    .irq_unmask	= ssb_gpio_irq_extif_unmask,
    };
#[no_mangle]
unsafe extern "C" fn ssb_gpio_irq_extif_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t ssb_gpio_irq_extif_handler(int irq, void *dev_id)
    {
    struct ssb_bus *bus = dev_id;
    struct ssb_extif *extif = &bus.extif;
    let mut val: u32 = ssb_read32(extif.dev, SSB_EXTIF_GPIO_IN);
    let mut mask: u32 = ssb_read32(extif.dev, SSB_EXTIF_GPIO_INTMASK);
    let mut pol: u32 = ssb_read32(extif.dev, SSB_EXTIF_GPIO_INTPOL);
    let mut irqs: c_ulong = (val ^ pol) & mask;
    int gpio;
    if (!irqs)
    return IRQ_NONE;
    for_each_set_bit(gpio, &irqs, bus.gpio.ngpio)
    generic_handle_domain_irq_safe(bus.irq_domain, gpio);
    ssb_extif_gpio_polarity(extif, irqs, val & irqs);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn ssb_gpio_irq_extif_domain_init(bus: *mut ssb_bus) -> c_int {
    static int ssb_gpio_irq_extif_domain_init(struct ssb_bus *bus)
    {
    struct ssb_extif *extif = &bus.extif;
    struct gpio_chip *chip = &bus.gpio;
    int gpio, hwirq, err;
    if (bus.bustype != SSB_BUSTYPE_SSB)
    return 0;
    bus.irq_domain = irq_domain_create_linear(core::ptr::null_mut(), chip.ngpio, &irq_domain_simple_ops,
    extif);
    if (!bus.irq_domain) {
    err = -ENODEV;
    goto err_irq_domain;
    }
    for (gpio = 0; gpio < chip.ngpio; gpio++) {
    let mut irq: c_int = irq_create_mapping(bus.irq_domain, gpio);
    irq_set_chip_data(irq, bus);
    irq_set_chip_and_handler(irq, &ssb_gpio_irq_extif_chip,
    handle_simple_irq);
    }
    hwirq = ssb_mips_irq(bus.extif.dev) + 2;
    err = request_irq(hwirq, ssb_gpio_irq_extif_handler, IRQF_SHARED,
    "gpio", bus);
    if (err)
    goto err_req_irq;
    ssb_extif_gpio_intmask(&bus.extif, ~0, 0);
    return 0;
    err_req_irq:
    for (gpio = 0; gpio < chip.ngpio; gpio++) {
    let mut irq: c_int = irq_find_mapping(bus.irq_domain, gpio);
    irq_dispose_mapping(irq);
    }
    irq_domain_remove(bus.irq_domain);
    err_irq_domain:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ssb_gpio_irq_extif_domain_exit(bus: *mut ssb_bus) {
    static void ssb_gpio_irq_extif_domain_exit(struct ssb_bus *bus)
    {
    struct ssb_extif *extif = &bus.extif;
    struct gpio_chip *chip = &bus.gpio;
    int gpio;
    if (bus.bustype != SSB_BUSTYPE_SSB)
    return;
    free_irq(ssb_mips_irq(bus.extif.dev) + 2, extif);
    for (gpio = 0; gpio < chip.ngpio; gpio++) {
    let mut irq: c_int = irq_find_mapping(bus.irq_domain, gpio);
    irq_dispose_mapping(irq);
    }
    irq_domain_remove(bus.irq_domain);
    }

#[no_mangle]
unsafe extern "C" fn ssb_gpio_irq_extif_domain_init(bus: *mut ssb_bus) -> c_int {
    static int ssb_gpio_irq_extif_domain_init(struct ssb_bus *bus)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ssb_gpio_irq_extif_domain_exit(bus: *mut ssb_bus) {
    static void ssb_gpio_irq_extif_domain_exit(struct ssb_bus *bus)
    {
    }

#[no_mangle]
unsafe extern "C" fn ssb_gpio_extif_init(bus: *mut ssb_bus) -> c_int {
    static int ssb_gpio_extif_init(struct ssb_bus *bus)
    {
    struct gpio_chip *chip = &bus.gpio;
    int err;
    chip.label		= "ssb_extif_gpio";
    chip.owner		= THIS_MODULE;
    chip.get		= ssb_gpio_extif_get_value;
    chip.set		= ssb_gpio_extif_set_value;
    chip.direction_input	= ssb_gpio_extif_direction_input;
    chip.direction_output	= ssb_gpio_extif_direction_output;

    chip.to_irq		= ssb_gpio_to_irq;

    chip.ngpio		= 5;
// There is just one SoC in one device and its GPIO addresses should be
// deterministic to address them more easily. The other buses could get
// a random base number.
//
    if (bus.bustype == SSB_BUSTYPE_SSB) {
    chip.base	= 0;
    chip.fwnode	= software_node_fwnode(&ssb_gpio_swnode);
    } else {
    chip.base	= -1;
    }
    err = ssb_gpio_irq_extif_domain_init(bus);
    if (err)
    return err;
    err = gpiochip_add_data(chip, bus);
    if (err) {
    ssb_gpio_irq_extif_domain_exit(bus);
    return err;
    }
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn ssb_gpio_extif_init(bus: *mut ssb_bus) -> c_int {
    static int ssb_gpio_extif_init(struct ssb_bus *bus)
    {
    return -ENOTSUPP;
    }

//
// Init
//
#[no_mangle]
pub unsafe extern "C" fn ssb_gpio_init(bus: *mut ssb_bus) -> c_int {
    int ssb_gpio_init(struct ssb_bus *bus)
    {
    let mut err: c_int = 0;
//
// Register software node only for the host SoC bus. There is only
// one SoC instance in the system, so there are no concerns with
// registration conflicts.
//
    if (bus.bustype == SSB_BUSTYPE_SSB) {
    err = software_node_register(&ssb_gpio_swnode);
    if (err)
    return err;
    }
    if (ssb_chipco_available(&bus.chipco))
    err = ssb_gpio_chipco_init(bus);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: ssb_extif_available(&bus->extif)) -> else {
    else if (ssb_extif_available(&bus.extif))
    err = ssb_gpio_extif_init(bus);
    else
    err = -ENODEV;
    if (err) {
    if (bus.bustype == SSB_BUSTYPE_SSB)
    software_node_unregister(&ssb_gpio_swnode);
    return err;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ssb_gpio_unregister(bus: *mut ssb_bus) -> c_int {
    int ssb_gpio_unregister(struct ssb_bus *bus)
    {
    if (ssb_chipco_available(&bus.chipco) ||
    ssb_extif_available(&bus.extif)) {
    gpiochip_remove(&bus.gpio);
    if (bus.bustype == SSB_BUSTYPE_SSB)
    software_node_unregister(&ssb_gpio_swnode);
    return 0;
    }
    return -1;
    }
