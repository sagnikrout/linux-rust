//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/arizona-irq.c
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
// Arizona interrupt support
//
// Copyright 2012 Wolfson Microelectronics plc
//
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
//

pub const ARIZONA_AOD_IRQ_INDEX: c_int = 0;
pub const ARIZONA_MAIN_IRQ_INDEX: c_int = 1;
#[no_mangle]
unsafe extern "C" fn arizona_map_irq(arizona: *mut arizona, irq: c_int) -> c_int {
    static int arizona_map_irq(struct arizona *arizona, int irq)
    {
    int ret;
    if (arizona.aod_irq_chip) {
    ret = regmap_irq_get_virq(arizona.aod_irq_chip, irq);
    if (ret >= 0)
    return ret;
    }
    return regmap_irq_get_virq(arizona.irq_chip, irq);
    }
    int arizona_request_irq(struct arizona *arizona, int irq, char *name,
    irq_handler_t handler, void *data)
    {
    irq = arizona_map_irq(arizona, irq);
    if (irq < 0)
    return irq;
    return request_threaded_irq(irq, core::ptr::null_mut(), handler, IRQF_ONESHOT,
    name, data);
    }
    EXPORT_SYMBOL_GPL(arizona_request_irq);
#[no_mangle]
pub unsafe extern "C" fn arizona_free_irq(arizona: *mut arizona, irq: c_int, data: *mut c_void) {
    void arizona_free_irq(struct arizona *arizona, int irq, void *data)
    {
    irq = arizona_map_irq(arizona, irq);
    if (irq < 0)
    return;
    free_irq(irq, data);
    }
    EXPORT_SYMBOL_GPL(arizona_free_irq);
#[no_mangle]
pub unsafe extern "C" fn arizona_set_irq_wake(arizona: *mut arizona, irq: c_int, on: c_int) -> c_int {
    int arizona_set_irq_wake(struct arizona *arizona, int irq, int on)
    {
    irq = arizona_map_irq(arizona, irq);
    if (irq < 0)
    return irq;
    return irq_set_irq_wake(irq, on);
    }
    EXPORT_SYMBOL_GPL(arizona_set_irq_wake);
#[no_mangle]
unsafe extern "C" fn arizona_boot_done(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t arizona_boot_done(int irq, void *data)
    {
    struct arizona *arizona = data;
    dev_dbg(arizona.dev, "Boot done\n");
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn arizona_ctrlif_err(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t arizona_ctrlif_err(int irq, void *data)
    {
    struct arizona *arizona = data;
//
// For pretty much all potential sources a register cache sync
// won't help, we've just got a software bug somewhere.
//
    dev_err(arizona.dev, "Control interface error\n");
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn arizona_irq_thread(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t arizona_irq_thread(int irq, void *data)
    {
    struct arizona *arizona = data;
    bool poll;
    unsigned int val;
    int ret;
    ret = pm_runtime_resume_and_get(arizona.dev);
    if (ret < 0) {
    dev_err(arizona.dev, "Failed to resume device: %d\n", ret);
    return IRQ_NONE;
    }
    do {
    poll = false;
    if (arizona.aod_irq_chip) {
//
// Check the AOD status register to determine whether
// the nested IRQ handler should be called.
//
    ret = regmap_read(arizona.regmap,
    ARIZONA_AOD_IRQ1, &val);
    if (ret)
    dev_warn(arizona.dev,
    "Failed to read AOD IRQ1 %d\n", ret);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: val) -> else {
    else if (val)
    handle_nested_irq(
    irq_find_mapping(arizona.virq, 0));
    }
//
// Check if one of the main interrupts is asserted and only
// check that domain if it is.
//
    ret = regmap_read(arizona.regmap, ARIZONA_IRQ_PIN_STATUS,
    &val);
    if (ret == 0 && val & ARIZONA_IRQ1_STS) {
    handle_nested_irq(irq_find_mapping(arizona.virq, 1));
    } else if (ret != 0) {
    dev_err(arizona.dev,
    "Failed to read main IRQ status: %d\n", ret);
    }

//
// Poll the IRQ pin status to see if we're really done
// if the interrupt controller can't do it for us.
//
    if (!arizona.pdata.irq_gpio) {
    break;
    } else if (arizona.pdata.irq_flags & IRQF_TRIGGER_RISING &&
    gpio_get_value_cansleep(arizona.pdata.irq_gpio)) {
    poll = true;
    } else if (arizona.pdata.irq_flags & IRQF_TRIGGER_FALLING &&
    !gpio_get_value_cansleep(arizona.pdata.irq_gpio)) {
    poll = true;
    }

    } while (poll);
    pm_runtime_put_autosuspend(arizona.dev);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn arizona_irq_enable(data: *mut irq_data) {
    static void arizona_irq_enable(struct irq_data *data)
    {
    }
#[no_mangle]
unsafe extern "C" fn arizona_irq_disable(data: *mut irq_data) {
    static void arizona_irq_disable(struct irq_data *data)
    {
    }
#[no_mangle]
unsafe extern "C" fn arizona_irq_set_wake(data: *mut irq_data, on: c_uint) -> c_int {
    static int arizona_irq_set_wake(struct irq_data *data, unsigned int on)
    {
    struct arizona *arizona = irq_data_get_irq_chip_data(data);
    return irq_set_irq_wake(arizona.irq, on);
    }
    static struct irq_chip arizona_irq_chip = {
    .name			= "arizona",
    .irq_disable		= arizona_irq_disable,
    .irq_enable		= arizona_irq_enable,
    .irq_set_wake		= arizona_irq_set_wake,
    };
    static struct lock_class_key arizona_irq_lock_class;
    static struct lock_class_key arizona_irq_request_class;
    static int arizona_irq_map(struct irq_domain *h, unsigned int virq,
    irq_hw_number_t hw)
    {
    struct arizona *data = h.host_data;
    irq_set_chip_data(virq, data);
    irq_set_lockdep_class(virq, &arizona_irq_lock_class,
    &arizona_irq_request_class);
    irq_set_chip_and_handler(virq, &arizona_irq_chip, handle_simple_irq);
    irq_set_nested_thread(virq, 1);
    irq_set_noprobe(virq);
    return 0;
    }
    static const struct irq_domain_ops arizona_domain_ops = {
    .map	= arizona_irq_map,
    .xlate	= irq_domain_xlate_twocell,
    };
#[no_mangle]
pub unsafe extern "C" fn arizona_irq_init(arizona: *mut arizona) -> c_int {
    int arizona_irq_init(struct arizona *arizona)
    {
    let mut flags: c_int = IRQF_ONESHOT;
    int ret;
    const struct regmap_irq_chip *aod, *irq;
    struct irq_data *irq_data;
    unsigned int virq;
    arizona.ctrlif_error = true;
    switch (arizona.type) {

    case WM5102:
    aod = &wm5102_aod;
    irq = &wm5102_irq;
    arizona.ctrlif_error = false;
    break;

    case WM5110:
    case WM8280:
    aod = &wm5110_aod;
    switch (arizona.rev) {
    case 0 ... 2:
    irq = &wm5110_irq;
    break;
    default:
    irq = &wm5110_revd_irq;
    break;
    }
    arizona.ctrlif_error = false;
    break;

    case WM1831:
    case CS47L24:
    aod = core::ptr::null_mut();
    irq = &cs47l24_irq;
    arizona.ctrlif_error = false;
    break;

    case WM8997:
    aod = &wm8997_aod;
    irq = &wm8997_irq;
    arizona.ctrlif_error = false;
    break;

    case WM8998:
    case WM1814:
    aod = &wm8998_aod;
    irq = &wm8998_irq;
    arizona.ctrlif_error = false;
    break;

    default:
    BUG_ON("Unknown Arizona class device" == core::ptr::null_mut());
    return -EINVAL;
    }
// Disable all wake sources by default
    regmap_write(arizona.regmap, ARIZONA_WAKE_CONTROL, 0);
// Read the flags from the interrupt controller if not specified
    if (!arizona.pdata.irq_flags) {
    irq_data = irq_get_irq_data(arizona.irq);
    if (!irq_data) {
    dev_err(arizona.dev, "Invalid IRQ: %d\n",
    arizona.irq);
    return -EINVAL;
    }
    arizona.pdata.irq_flags = irqd_get_trigger_type(irq_data);
    switch (arizona.pdata.irq_flags) {
    case IRQF_TRIGGER_LOW:
    case IRQF_TRIGGER_HIGH:
    case IRQF_TRIGGER_RISING:
    case IRQF_TRIGGER_FALLING:
    break;
    case IRQ_TYPE_NONE:
    default:
// Device default
    arizona.pdata.irq_flags = IRQF_TRIGGER_LOW;
    break;
    }
    }
    if (arizona.pdata.irq_flags & (IRQF_TRIGGER_HIGH |
    IRQF_TRIGGER_RISING)) {
    ret = regmap_update_bits(arizona.regmap, ARIZONA_IRQ_CTRL_1,
    ARIZONA_IRQ_POL, 0);
    if (ret != 0) {
    dev_err(arizona.dev, "Couldn't set IRQ polarity: %d\n",
    ret);
    goto err;
    }
    }
    flags |= arizona.pdata.irq_flags;
// Allocate a virtual IRQ domain to distribute to the regmap domains
    arizona.virq = irq_domain_create_linear(core::ptr::null_mut(), 2, &arizona_domain_ops, arizona);
    if (!arizona.virq) {
    dev_err(arizona.dev, "Failed to add core IRQ domain\n");
    ret = -EINVAL;
    goto err;
    }
    if (aod) {
    virq = irq_create_mapping(arizona.virq, ARIZONA_AOD_IRQ_INDEX);
    if (!virq) {
    dev_err(arizona.dev, "Failed to map AOD IRQs\n");
    ret = -EINVAL;
    goto err_domain;
    }
    ret = regmap_add_irq_chip(arizona.regmap, virq, IRQF_ONESHOT,
    0, aod, &arizona.aod_irq_chip);
    if (ret != 0) {
    dev_err(arizona.dev,
    "Failed to add AOD IRQs: %d\n", ret);
    goto err_map_aod;
    }
    }
    virq = irq_create_mapping(arizona.virq, ARIZONA_MAIN_IRQ_INDEX);
    if (!virq) {
    dev_err(arizona.dev, "Failed to map main IRQs\n");
    ret = -EINVAL;
    goto err_aod;
    }
    ret = regmap_add_irq_chip(arizona.regmap, virq, IRQF_ONESHOT,
    0, irq, &arizona.irq_chip);
    if (ret != 0) {
    dev_err(arizona.dev, "Failed to add main IRQs: %d\n", ret);
    goto err_map_main_irq;
    }

// Used to emulate edge trigger and to work around broken pinmux
    if (arizona.pdata.irq_gpio) {
    if (gpio_to_irq(arizona.pdata.irq_gpio) != arizona.irq) {
    dev_warn(arizona.dev, "IRQ %d is not GPIO %d (%d)\n",
    arizona.irq, arizona.pdata.irq_gpio,
    gpio_to_irq(arizona.pdata.irq_gpio));
    arizona.irq = gpio_to_irq(arizona.pdata.irq_gpio);
    }
    ret = devm_gpio_request_one(arizona.dev,
    arizona.pdata.irq_gpio,
    GPIOF_IN, "arizona IRQ");
    if (ret != 0) {
    dev_err(arizona.dev,
    "Failed to request IRQ GPIO %d:: %d\n",
    arizona.pdata.irq_gpio, ret);
    arizona.pdata.irq_gpio = 0;
    }
    }

    ret = request_threaded_irq(arizona.irq, core::ptr::null_mut(), arizona_irq_thread,
    flags, "arizona", arizona);
    if (ret != 0) {
    dev_err(arizona.dev, "Failed to request primary IRQ %d: %d\n",
    arizona.irq, ret);
    goto err_main_irq;
    }
// Make sure the boot done IRQ is unmasked for resumes
    ret = arizona_request_irq(arizona, ARIZONA_IRQ_BOOT_DONE, "Boot done",
    arizona_boot_done, arizona);
    if (ret != 0) {
    dev_err(arizona.dev, "Failed to request boot done %d: %d\n",
    arizona.irq, ret);
    goto err_boot_done;
    }
// Handle control interface errors in the core
    if (arizona.ctrlif_error) {
    ret = arizona_request_irq(arizona, ARIZONA_IRQ_CTRLIF_ERR,
    "Control interface error",
    arizona_ctrlif_err, arizona);
    if (ret != 0) {
    dev_err(arizona.dev,
    "Failed to request CTRLIF_ERR %d: %d\n",
    arizona.irq, ret);
    goto err_ctrlif;
    }
    }
    return 0;
    err_ctrlif:
    arizona_free_irq(arizona, ARIZONA_IRQ_BOOT_DONE, arizona);
    err_boot_done:
    free_irq(arizona.irq, arizona);
    err_main_irq:
    regmap_del_irq_chip(irq_find_mapping(arizona.virq,
    ARIZONA_MAIN_IRQ_INDEX),
    arizona.irq_chip);
    err_map_main_irq:
    irq_dispose_mapping(irq_find_mapping(arizona.virq,
    ARIZONA_MAIN_IRQ_INDEX));
    err_aod:
    regmap_del_irq_chip(irq_find_mapping(arizona.virq,
    ARIZONA_AOD_IRQ_INDEX),
    arizona.aod_irq_chip);
    err_map_aod:
    irq_dispose_mapping(irq_find_mapping(arizona.virq,
    ARIZONA_AOD_IRQ_INDEX));
    err_domain:
    irq_domain_remove(arizona.virq);
    err:
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn arizona_irq_exit(arizona: *mut arizona) -> c_int {
    int arizona_irq_exit(struct arizona *arizona)
    {
    unsigned int virq;
    if (arizona.ctrlif_error)
    arizona_free_irq(arizona, ARIZONA_IRQ_CTRLIF_ERR, arizona);
    arizona_free_irq(arizona, ARIZONA_IRQ_BOOT_DONE, arizona);
    virq = irq_find_mapping(arizona.virq, ARIZONA_MAIN_IRQ_INDEX);
    regmap_del_irq_chip(virq, arizona.irq_chip);
    irq_dispose_mapping(virq);
    virq = irq_find_mapping(arizona.virq, ARIZONA_AOD_IRQ_INDEX);
    regmap_del_irq_chip(virq, arizona.aod_irq_chip);
    irq_dispose_mapping(virq);
    irq_domain_remove(arizona.virq);
    free_irq(arizona.irq, arizona);
    return 0;
    }
