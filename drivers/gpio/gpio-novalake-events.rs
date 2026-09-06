//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-novalake-events.c
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
// Intel Nova Lake GPIO-signaled ACPI events driver
//
// Copyright (c) 2026, Intel Corporation.
//
// Author: Alan Borzeszkowski <alan.borzeszkowski@linux.intel.com>
//
// Intel client platforms released in 2026 and later (starting with Intel Nova
// Lake) support two modes of handling ACPI General Purpose Events (GPE):
// exposed GPIO interrupt mode and legacy mode.
//
// By default, the platform uses legacy mode, handling GPEs as usual. If this
// driver is installed, it signals to the platform (on every boot) that exposed
// GPIO interrupt mode is supported. The platform then switches to exposed
// mode, which takes effect on next boot. From the user perspective, this
// change is transparent.
//
// However, if driver is uninstalled while in exposed interrupt mode, GPEs will
// _not_ be handled until platform falls back to legacy mode. This means that
// USB keyboard, mouse might not function properly for the fallback duration.
// Fallback requires two reboots to take effect: on first reboot, platform no
// longer receives signal from this driver and switches to legacy mode, which
// takes effect on second boot.
//
// Example ACPI event: Power Management Event coming from motherboard PCH,
// waking system from sleep following USB mouse hotplug.
//
// This driver supports up to 128 GPIO pins in each GPE block, per ACPI
// specification v6.6 section 5.6.4.
//

//
// GPE block has two registers, each register takes half the block size.
// Convert size to bits to get total GPIO pin count.
//

pub const GPE_STS_REG_OFFSET: c_int = 0;

//
// struct nvl_gpio - Intel Nova Lake GPIO driver state
// @gc: GPIO controller interface
// @reg_base: Base address of the GPE registers
// @lock: Guard register access
// @blk_size: GPE block length
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvl_gpio {
    pub gc: gpio_chip,
    pub reg_base: *mut void __iomem,
    pub lock: raw_spinlock_t,
    pub blk_size: usize,
}

    static void __iomem *nvl_gpio_get_byte_addr(struct nvl_gpio *priv,
    unsigned int reg_offset,
    unsigned long gpio)
    {
    return priv.reg_base + reg_offset + gpio;
    }
#[no_mangle]
unsafe extern "C" fn nvl_gpio_get(gc: *mut gpio_chip, gpio: c_uint) -> c_int {
    static int nvl_gpio_get(struct gpio_chip *gc, unsigned int gpio)
    {
    struct nvl_gpio *priv = gpiochip_get_data(gc);
    let mut byte_idx: c_uint = gpio / BITS_PER_BYTE;
    let mut bit_idx: c_uint = gpio % BITS_PER_BYTE;
    void __iomem *addr;
    u8 reg;
    addr = nvl_gpio_get_byte_addr(priv, GPE_STS_REG_OFFSET, byte_idx);
    guard(raw_spinlock_irqsave)(&priv.lock);
    reg = ioread8(addr);
    return !!(reg & BIT(bit_idx));
    }
    static const struct gpio_chip nvl_gpio_chip = {
    .owner	= THIS_MODULE,
    .get	= nvl_gpio_get,
    };
#[no_mangle]
unsafe extern "C" fn nvl_gpio_irq_set_type(d: *mut irq_data, type: c_uint) -> c_int {
    static int nvl_gpio_irq_set_type(struct irq_data *d, unsigned int type)
    {
    if (type & IRQ_TYPE_EDGE_BOTH)
    irq_set_handler_locked(d, handle_edge_irq);
#[no_mangle]
pub unsafe extern "C" fn if(IRQ_TYPE_LEVEL_MASK: type &) -> else {
    else if (type & IRQ_TYPE_LEVEL_MASK)
    irq_set_handler_locked(d, handle_level_irq);
    return 0;
    }
    static void nvl_gpio_irq_mask_unmask(struct gpio_chip *gc, unsigned long hwirq,
    bool mask)
    {
    struct nvl_gpio *priv = gpiochip_get_data(gc);
    let mut byte_idx: c_uint = hwirq / BITS_PER_BYTE;
    let mut bit_idx: c_uint = hwirq % BITS_PER_BYTE;
    void __iomem *addr;
    u8 reg;
    addr = nvl_gpio_get_byte_addr(priv, GPE_EN_REG_OFFSET(priv.blk_size), byte_idx);
    guard(raw_spinlock_irqsave)(&priv.lock);
    reg = ioread8(addr);
    if (mask)
    reg &= ~BIT(bit_idx);
    else
    reg |= BIT(bit_idx);
    iowrite8(reg, addr);
    }
#[no_mangle]
unsafe extern "C" fn nvl_gpio_irq_unmask(d: *mut irq_data) {
    static void nvl_gpio_irq_unmask(struct irq_data *d)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(d);
    gpiochip_enable_irq(gc, hwirq);
    nvl_gpio_irq_mask_unmask(gc, hwirq, false);
    }
#[no_mangle]
unsafe extern "C" fn nvl_gpio_irq_mask(d: *mut irq_data) {
    static void nvl_gpio_irq_mask(struct irq_data *d)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(d);
    nvl_gpio_irq_mask_unmask(gc, hwirq, true);
    gpiochip_disable_irq(gc, hwirq);
    }
#[no_mangle]
unsafe extern "C" fn nvl_gpio_irq_ack(d: *mut irq_data) {
    static void nvl_gpio_irq_ack(struct irq_data *d)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct nvl_gpio *priv = gpiochip_get_data(gc);
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(d);
    let mut byte_idx: c_uint = hwirq / BITS_PER_BYTE;
    let mut bit_idx: c_uint = hwirq % BITS_PER_BYTE;
    void __iomem *addr;
    u8 reg;
    addr = nvl_gpio_get_byte_addr(priv, GPE_STS_REG_OFFSET, byte_idx);
    guard(raw_spinlock_irqsave)(&priv.lock);
    reg = ioread8(addr);
    reg |= BIT(bit_idx);
    iowrite8(reg, addr);
    }
    static const struct irq_chip nvl_gpio_irq_chip = {
    .name		= "gpio-novalake",
    .irq_ack	= nvl_gpio_irq_ack,
    .irq_mask	= nvl_gpio_irq_mask,
    .irq_unmask	= nvl_gpio_irq_unmask,
    .irq_set_type	= nvl_gpio_irq_set_type,
    .flags		= IRQCHIP_IMMUTABLE,
    GPIOCHIP_IRQ_RESOURCE_HELPERS,
    };
#[no_mangle]
unsafe extern "C" fn nvl_gpio_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t nvl_gpio_irq(int irq, void *data)
    {
    struct nvl_gpio *priv = data;
    let mut block_size: usize = priv.blk_size;
    let mut handled: c_uint = 0;
    for (unsigned int i = 0; i < block_size; i++) {
    const void __iomem *reg = priv.reg_base + i;
    unsigned long pending;
    unsigned long enabled;
    unsigned int bit_idx;
    scoped_guard(raw_spinlock, &priv.lock) {
    pending = ioread8(reg + GPE_STS_REG_OFFSET);
    enabled = ioread8(reg + GPE_EN_REG_OFFSET(block_size));
    }
    pending &= enabled;
    for_each_set_bit(bit_idx, &pending, BITS_PER_BYTE) {
    let mut hwirq: c_uint = i * BITS_PER_BYTE + bit_idx;
    generic_handle_domain_irq(priv.gc.irq.domain, hwirq);
    }
    handled += pending ? 1 : 0;
    }
    return IRQ_RETVAL(handled);
    }
// UUID for GPE device _DSM: 079406e6-bdea-49cf-8563-03e2811901cb
    static const guid_t nvl_gpe_dsm_guid =
    GUID_INIT(0x079406e6, 0xbdea, 0x49cf,
    0x85, 0x63, 0x03, 0xe2, 0x81, 0x19, 0x01, 0xcb);
pub const DSM_GPE_MODE_REV: c_int = 1;
pub const DSM_GPE_MODE_FN_INDEX: c_int = 1;
pub const DSM_ENABLE_GPE_MODE: c_int = 1;
#[no_mangle]
unsafe extern "C" fn nvl_acpi_enable_gpe_mode(dev: *mut device) -> c_int {
    static int nvl_acpi_enable_gpe_mode(struct device *dev)
    {
    union acpi_object argv4[2];
    union acpi_object *obj;
    argv4[0].type = ACPI_TYPE_PACKAGE;
    argv4[0].package.count = 1;
    argv4[0].package.elements = &argv4[1];
    argv4[1].integer.type = ACPI_TYPE_INTEGER;
    argv4[1].integer.value = DSM_ENABLE_GPE_MODE;
    obj = acpi_evaluate_dsm_typed(ACPI_HANDLE(dev), &nvl_gpe_dsm_guid,
    DSM_GPE_MODE_REV, DSM_GPE_MODE_FN_INDEX,
    argv4, ACPI_TYPE_BUFFER);
    if (!obj)
    return -EIO;
    ACPI_FREE(obj);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nvl_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int nvl_gpio_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    resource_size_t ioresource_size;
    struct gpio_irq_chip *girq;
    struct nvl_gpio *priv;
    struct resource *res;
    void __iomem *regs;
    int ret, irq;
    res = platform_get_resource(pdev, IORESOURCE_IO, 0);
    if (!res)
    return -ENXIO;
//
// GPE block length should be non-negative multiple of two and allow up
// to 128 pins. ACPI v6.6 section 5.2.9 and 5.6.4.
//
    ioresource_size = resource_size(res);
    if (!ioresource_size || ioresource_size % 2 || ioresource_size > 0x20)
    return dev_err_probe(dev, -EINVAL,
    "invalid GPE block length, resource: %pR\n",
    res);
    regs = devm_ioport_map(dev, res.start, ioresource_size);
    if (!regs)
    return -ENOMEM;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    raw_spin_lock_init(&priv.lock);
    priv.reg_base = regs;
    priv.blk_size = ioresource_size;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    ret = devm_request_irq(dev, irq, nvl_gpio_irq, IRQF_SHARED, dev_name(dev), priv);
    if (ret)
    return ret;
    priv.gc	= nvl_gpio_chip;
    priv.gc.label	= dev_name(dev);
    priv.gc.parent	= dev;
    priv.gc.ngpio	= GPE_REG_PIN_COUNT(priv.blk_size);
    priv.gc.base	= -1;
    girq = &priv.gc.irq;
    gpio_irq_chip_set_chip(girq, &nvl_gpio_irq_chip);
    girq.parent_handler	= core::ptr::null_mut();
    girq.num_parents	= 0;
    girq.parents		= core::ptr::null_mut();
    girq.default_type	= IRQ_TYPE_NONE;
    girq.handler		= handle_bad_irq;
    ret = devm_gpiochip_add_data(dev, &priv.gc, priv);
    if (ret)
    return ret;
    return nvl_acpi_enable_gpe_mode(dev);
    }
    static const struct acpi_device_id nvl_gpio_acpi_match[] = {
    { "INTC1114" },
    {}
    };
    MODULE_DEVICE_TABLE(acpi, nvl_gpio_acpi_match);
    static struct platform_driver nvl_gpio_driver = {
    .driver = {
    .name		  = "gpio-novalake-events",
    .acpi_match_table = nvl_gpio_acpi_match,
    },
    .probe = nvl_gpio_probe,
    };
    module_platform_driver(nvl_gpio_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Alan Borzeszkowski <alan.borzeszkowski@linux.intel.com>");
    MODULE_DESCRIPTION("Intel Nova Lake ACPI GPIO events driver");
