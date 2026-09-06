//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/intel/int0002_vgpio.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Intel INT0002 "Virtual GPIO" driver
//
// Copyright (C) 2017 Hans de Goede <hdegoede@redhat.com>
//
// Loosely based on android x86 kernel code which is:
//
// Copyright (c) 2014, Intel Corporation.
//
// Author: Dyut Kumar Sil <dyut.k.sil@intel.com>
//
// Some peripherals on Bay Trail and Cherry Trail platforms signal a Power
// Management Event (PME) to the Power Management Controller (PMC) to wakeup
// the system. When this happens software needs to clear the PME bus 0 status
// bit in the GPE0a_STS register to avoid an IRQ storm on IRQ 9.
//
// This is modelled in ACPI through the INT0002 ACPI device, which is
// called a "Virtual GPIO controller" in ACPI because it defines the event
// handler to call when the PME triggers through _AEI and _L02 / _E02
// methods as would be done for a real GPIO interrupt in ACPI. Note this
// is a hack to define an AML event handler for the PME while using existing
// ACPI mechanisms, this is not a real GPIO at all.
//
// This driver will bind to the INT0002 device, and register as a GPIO
// controller, letting gpiolib-acpi call the _L02 handler as it would
// for a real GPIO controller.
//

// For some reason the virtual GPIO pin tied to the GPE is numbered pin 2
pub const GPE0A_PME_B0_VIRT_GPIO_PIN: c_int = 2;

pub const GPE0A_STS_PORT: c_uint = 0x420;
pub const GPE0A_EN_PORT: c_uint = 0x428;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct int0002_data {
    pub chip: gpio_chip,
    pub parent_irq: c_int,
    pub wake_enable_count: c_int,
}

//
// As this is not a real GPIO at all, but just a hack to model an event in
// ACPI the get / set functions are dummy functions.
//
#[no_mangle]
unsafe extern "C" fn int0002_gpio_get(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    static int int0002_gpio_get(struct gpio_chip *chip, unsigned int offset)
    {
    return 0;
    }
    static int int0002_gpio_set(struct gpio_chip *chip, unsigned int offset,
    int value)
    {
    return 0;
    }
    static int int0002_gpio_direction_output(struct gpio_chip *chip,
    unsigned int offset, int value)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn int0002_irq_ack(data: *mut irq_data) {
    static void int0002_irq_ack(struct irq_data *data)
    {
    outl(GPE0A_PME_B0_STS_BIT, GPE0A_STS_PORT);
    }
#[no_mangle]
unsafe extern "C" fn int0002_irq_unmask(data: *mut irq_data) {
    static void int0002_irq_unmask(struct irq_data *data)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(data);
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(data);
    u32 gpe_en_reg;
    gpiochip_enable_irq(gc, hwirq);
    gpe_en_reg = inl(GPE0A_EN_PORT);
    gpe_en_reg |= GPE0A_PME_B0_EN_BIT;
    outl(gpe_en_reg, GPE0A_EN_PORT);
    }
#[no_mangle]
unsafe extern "C" fn int0002_irq_mask(data: *mut irq_data) {
    static void int0002_irq_mask(struct irq_data *data)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(data);
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(data);
    u32 gpe_en_reg;
    gpe_en_reg = inl(GPE0A_EN_PORT);
    gpe_en_reg &= ~GPE0A_PME_B0_EN_BIT;
    outl(gpe_en_reg, GPE0A_EN_PORT);
    gpiochip_disable_irq(gc, hwirq);
    }
#[no_mangle]
unsafe extern "C" fn int0002_irq_set_wake(data: *mut irq_data, on: c_uint) -> c_int {
    static int int0002_irq_set_wake(struct irq_data *data, unsigned int on)
    {
    struct gpio_chip *chip = irq_data_get_irq_chip_data(data);
    struct int0002_data *int0002 = container_of(chip, struct int0002_data, chip);
//
// Applying of the wakeup flag to our parent IRQ is delayed till system
// suspend, because we only want to do this when using s2idle.
//
    if (on)
    int0002.wake_enable_count++;
    else
    int0002.wake_enable_count--;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn int0002_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t int0002_irq(int irq, void *data)
    {
    struct gpio_chip *chip = data;
    u32 gpe_sts_reg;
    gpe_sts_reg = inl(GPE0A_STS_PORT);
    if (!(gpe_sts_reg & GPE0A_PME_B0_STS_BIT))
    return IRQ_NONE;
    generic_handle_domain_irq_safe(chip.irq.domain, GPE0A_PME_B0_VIRT_GPIO_PIN);
    pm_wakeup_hard_event(chip.parent);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn int0002_check_wake(data: *mut c_void) -> bool {
    static bool int0002_check_wake(void *data)
    {
    u32 gpe_sts_reg;
    gpe_sts_reg = inl(GPE0A_STS_PORT);
    return (gpe_sts_reg & GPE0A_PME_B0_STS_BIT);
    }
    static const struct irq_chip int0002_irqchip = {
    .name			= DRV_NAME,
    .irq_ack		= int0002_irq_ack,
    .irq_mask		= int0002_irq_mask,
    .irq_unmask		= int0002_irq_unmask,
    .irq_set_wake		= int0002_irq_set_wake,
    .flags			= IRQCHIP_IMMUTABLE,
    GPIOCHIP_IRQ_RESOURCE_HELPERS,
    };
    static void int0002_init_irq_valid_mask(struct gpio_chip *chip,
    unsigned long *valid_mask,
    unsigned int ngpios)
    {
    bitmap_clear(valid_mask, 0, GPE0A_PME_B0_VIRT_GPIO_PIN);
    }
#[no_mangle]
unsafe extern "C" fn int0002_probe(pdev: *mut platform_device) -> c_int {
    static int int0002_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct int0002_data *int0002;
    struct gpio_irq_chip *girq;
    struct gpio_chip *chip;
    int irq, ret;
// Menlow has a different INT0002 device? <sigh>
    if (!soc_intel_is_byt() && !soc_intel_is_cht())
    return -ENODEV;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    int0002 = devm_kzalloc(dev, sizeof(*int0002), GFP_KERNEL);
    if (!int0002)
    return -ENOMEM;
    int0002.parent_irq = irq;
    chip = &int0002.chip;
    chip.label = DRV_NAME;
    chip.parent = dev;
    chip.owner = THIS_MODULE;
    chip.get = int0002_gpio_get;
    chip.set = int0002_gpio_set;
    chip.direction_input = int0002_gpio_get;
    chip.direction_output = int0002_gpio_direction_output;
    chip.base = -1;
    chip.ngpio = GPE0A_PME_B0_VIRT_GPIO_PIN + 1;
    chip.irq.init_valid_mask = int0002_init_irq_valid_mask;
//
// We directly request the irq here instead of passing a flow-handler
// to gpiochip_set_chained_irqchip, because the irq is shared.
// FIXME: augment this if we managed to pull handling of shared
// IRQs into gpiolib.
//
    ret = devm_request_irq(dev, irq, int0002_irq, IRQF_SHARED, "INT0002",
    chip);
    if (ret) {
    dev_err(dev, "Error requesting IRQ %d: %d\n", irq, ret);
    return ret;
    }
    girq = &chip.irq;
    gpio_irq_chip_set_chip(girq, &int0002_irqchip);
// This let us handle the parent IRQ in the driver
    girq.parent_handler = core::ptr::null_mut();
    girq.num_parents = 0;
    girq.parents = core::ptr::null_mut();
    girq.default_type = IRQ_TYPE_NONE;
    girq.handler = handle_edge_irq;
    ret = devm_gpiochip_add_data(dev, chip, core::ptr::null_mut());
    if (ret) {
    dev_err(dev, "Error adding gpio chip: %d\n", ret);
    return ret;
    }
    acpi_register_wakeup_handler(irq, int0002_check_wake, core::ptr::null_mut());
    device_init_wakeup(dev, true);
    dev_set_drvdata(dev, int0002);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn int0002_remove(pdev: *mut platform_device) {
    static void int0002_remove(struct platform_device *pdev)
    {
    device_init_wakeup(&pdev.dev, false);
    acpi_unregister_wakeup_handler(int0002_check_wake, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn int0002_suspend(dev: *mut device) -> c_int {
    static int int0002_suspend(struct device *dev)
    {
    struct int0002_data *int0002 = dev_get_drvdata(dev);
//
// The INT0002 parent IRQ is often shared with the ACPI GPE IRQ, don't
// muck with it when firmware based suspend is used, otherwise we may
// cause spurious wakeups from firmware managed suspend.
//
    if (!pm_suspend_via_firmware() && int0002.wake_enable_count)
    enable_irq_wake(int0002.parent_irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn int0002_resume(dev: *mut device) -> c_int {
    static int int0002_resume(struct device *dev)
    {
    struct int0002_data *int0002 = dev_get_drvdata(dev);
    if (!pm_suspend_via_firmware() && int0002.wake_enable_count)
    disable_irq_wake(int0002.parent_irq);
    return 0;
    }
    static const struct dev_pm_ops int0002_pm_ops = {
    .suspend = int0002_suspend,
    .resume = int0002_resume,
    };
    static const struct acpi_device_id int0002_acpi_ids[] = {
    { "INT0002", 0 },
    { },
    };
    MODULE_DEVICE_TABLE(acpi, int0002_acpi_ids);
    static struct platform_driver int0002_driver = {
    .driver	= {
    .name			= DRV_NAME,
    .acpi_match_table	= int0002_acpi_ids,
    .pm			= &int0002_pm_ops,
    },
    .probe	= int0002_probe,
    .remove	= int0002_remove,
    };
    module_platform_driver(int0002_driver);
    MODULE_AUTHOR("Hans de Goede <hdegoede@redhat.com>");
    MODULE_DESCRIPTION("Intel INT0002 Virtual GPIO driver");
    MODULE_LICENSE("GPL v2");
