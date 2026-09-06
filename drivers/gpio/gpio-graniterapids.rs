//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-graniterapids.c
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
// Intel Granite Rapids-D vGPIO driver
//
// Copyright (c) 2024, Intel Corporation.
//
// Author: Aapo Vienamo <aapo.vienamo@linux.intel.com>
//

pub const GNR_NUM_PINS: c_int = 128;
pub const GNR_PINS_PER_REG: c_int = 32;

pub const GNR_CFG_PADBAR: c_uint = 0x00;
pub const GNR_CFG_LOCK_OFFSET: c_uint = 0x04;
pub const GNR_GPI_STATUS_OFFSET: c_uint = 0x14;
pub const GNR_GPI_ENABLE_OFFSET: c_uint = 0x24;

//
// struct gnr_gpio - Intel Granite Rapids-D vGPIO driver state
// @gc: GPIO controller interface
// @reg_base: base address of the GPIO registers
// @pad_base: base address of the vGPIO pad configuration registers
// @ro_bitmap: bitmap of read-only pins
// @lock: guard the registers
// @pad_backup: backup of the register state for suspend
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gnr_gpio {
    pub gc: gpio_chip,
    pub reg_base: *mut void __iomem,
    pub pad_base: *mut void __iomem,
    pub GNR_NUM_PINS): DECLARE_BITMAP(ro_bitmap,,
    pub lock: raw_spinlock_t,
    pub pad_backup: [u32; ],
}

    static void __iomem *gnr_gpio_get_padcfg_addr(const struct gnr_gpio *priv,
    unsigned int gpio)
    {
    return priv.pad_base + gpio * sizeof(u32);
    }
    static int gnr_gpio_configure_line(struct gpio_chip *gc, unsigned int gpio,
    u32 clear_mask, u32 set_mask)
    {
    struct gnr_gpio *priv = gpiochip_get_data(gc);
    void __iomem *addr = gnr_gpio_get_padcfg_addr(priv, gpio);
    u32 dw;
    if (test_bit(gpio, priv.ro_bitmap))
    return -EACCES;
    guard(raw_spinlock_irqsave)(&priv.lock);
    dw = readl(addr);
    dw &= ~clear_mask;
    dw |= set_mask;
    writel(dw, addr);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gnr_gpio_request(gc: *mut gpio_chip, gpio: c_uint) -> c_int {
    static int gnr_gpio_request(struct gpio_chip *gc, unsigned int gpio)
    {
    struct gnr_gpio *priv = gpiochip_get_data(gc);
    u32 dw;
    dw = readl(gnr_gpio_get_padcfg_addr(priv, gpio));
    if (!(dw & GNR_CFG_DW_HOSTSW_MODE)) {
    dev_warn(gc.parent, "GPIO %u is not owned by host", gpio);
    return -EBUSY;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gnr_gpio_get(gc: *mut gpio_chip, gpio: c_uint) -> c_int {
    static int gnr_gpio_get(struct gpio_chip *gc, unsigned int gpio)
    {
    const struct gnr_gpio *priv = gpiochip_get_data(gc);
    u32 dw;
    dw = readl(gnr_gpio_get_padcfg_addr(priv, gpio));
    return !!(dw & GNR_CFG_DW_RXSTATE);
    }
#[no_mangle]
unsafe extern "C" fn gnr_gpio_set(gc: *mut gpio_chip, gpio: c_uint, value: c_int) -> c_int {
    static int gnr_gpio_set(struct gpio_chip *gc, unsigned int gpio, int value)
    {
    let mut clear: u32 = 0;
    let mut set: u32 = 0;
    if (value)
    set = GNR_CFG_DW_TXSTATE;
    else
    clear = GNR_CFG_DW_TXSTATE;
    return gnr_gpio_configure_line(gc, gpio, clear, set);
    }
#[no_mangle]
unsafe extern "C" fn gnr_gpio_get_direction(gc: *mut gpio_chip, gpio: c_uint) -> c_int {
    static int gnr_gpio_get_direction(struct gpio_chip *gc, unsigned int gpio)
    {
    struct gnr_gpio *priv = gpiochip_get_data(gc);
    u32 dw;
    dw = readl(gnr_gpio_get_padcfg_addr(priv, gpio));
    if (dw & GNR_CFG_DW_TXDIS)
    return GPIO_LINE_DIRECTION_IN;
    return GPIO_LINE_DIRECTION_OUT;
    }
#[no_mangle]
unsafe extern "C" fn gnr_gpio_direction_input(gc: *mut gpio_chip, gpio: c_uint) -> c_int {
    static int gnr_gpio_direction_input(struct gpio_chip *gc, unsigned int gpio)
    {
    return gnr_gpio_configure_line(gc, gpio, GNR_CFG_DW_RXDIS, 0);
    }
#[no_mangle]
unsafe extern "C" fn gnr_gpio_direction_output(gc: *mut gpio_chip, gpio: c_uint, value: c_int) -> c_int {
    static int gnr_gpio_direction_output(struct gpio_chip *gc, unsigned int gpio, int value)
    {
    let mut clear: u32 = GNR_CFG_DW_TXDIS;
    let mut set: u32 = value ? GNR_CFG_DW_TXSTATE : 0;
    return gnr_gpio_configure_line(gc, gpio, clear, set);
    }
    static const struct gpio_chip gnr_gpio_chip = {
    .owner		  = THIS_MODULE,
    .request	  = gnr_gpio_request,
    .get		  = gnr_gpio_get,
    .set		  = gnr_gpio_set,
    .get_direction    = gnr_gpio_get_direction,
    .direction_input  = gnr_gpio_direction_input,
    .direction_output = gnr_gpio_direction_output,
    };
    static void __iomem *gnr_gpio_get_reg_addr(const struct gnr_gpio *priv,
    unsigned int base,
    unsigned int gpio)
    {
    return priv.reg_base + base + gpio * sizeof(u32);
    }
#[no_mangle]
unsafe extern "C" fn gnr_gpio_irq_ack(d: *mut irq_data) {
    static void gnr_gpio_irq_ack(struct irq_data *d)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct gnr_gpio *priv = gpiochip_get_data(gc);
    let mut gpio: irq_hw_number_t = irqd_to_hwirq(d);
    let mut reg_idx: c_uint = gpio / GNR_PINS_PER_REG;
    let mut bit_idx: c_uint = gpio % GNR_PINS_PER_REG;
    void __iomem *addr = gnr_gpio_get_reg_addr(priv, GNR_GPI_STATUS_OFFSET, reg_idx);
    u32 reg;
    guard(raw_spinlock_irqsave)(&priv.lock);
    reg = readl(addr);
    reg |= BIT(bit_idx);
    writel(reg, addr);
    }
#[no_mangle]
unsafe extern "C" fn gnr_gpio_irq_mask_unmask(gc: *mut gpio_chip, gpio: c_ulong, mask: bool) {
    static void gnr_gpio_irq_mask_unmask(struct gpio_chip *gc, unsigned long gpio, bool mask)
    {
    struct gnr_gpio *priv = gpiochip_get_data(gc);
    let mut reg_idx: c_uint = gpio / GNR_PINS_PER_REG;
    let mut bit_idx: c_uint = gpio % GNR_PINS_PER_REG;
    void __iomem *addr = gnr_gpio_get_reg_addr(priv, GNR_GPI_ENABLE_OFFSET, reg_idx);
    u32 reg;
    guard(raw_spinlock_irqsave)(&priv.lock);
    reg = readl(addr);
    if (mask)
    reg &= ~BIT(bit_idx);
    else
    reg |= BIT(bit_idx);
    writel(reg, addr);
    }
#[no_mangle]
unsafe extern "C" fn gnr_gpio_irq_mask(d: *mut irq_data) {
    static void gnr_gpio_irq_mask(struct irq_data *d)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(d);
    gnr_gpio_irq_mask_unmask(gc, hwirq, true);
    gpiochip_disable_irq(gc, hwirq);
    }
#[no_mangle]
unsafe extern "C" fn gnr_gpio_irq_unmask(d: *mut irq_data) {
    static void gnr_gpio_irq_unmask(struct irq_data *d)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(d);
    gpiochip_enable_irq(gc, hwirq);
    gnr_gpio_irq_mask_unmask(gc, hwirq, false);
    }
#[no_mangle]
unsafe extern "C" fn gnr_gpio_irq_set_type(d: *mut irq_data, type: c_uint) -> c_int {
    static int gnr_gpio_irq_set_type(struct irq_data *d, unsigned int type)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct gnr_gpio *priv = gpiochip_get_data(gc);
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(d);
    u32 reg;
    u32 set;
// Allow interrupts only if Interrupt Select field is non-zero
    reg = readl(gnr_gpio_get_padcfg_addr(priv, hwirq));
    if (!(reg & GNR_CFG_DW_INTSEL_MASK)) {
    dev_dbg(gc.parent, "GPIO %lu cannot be used as IRQ", hwirq);
    return -EPERM;
    }
// Falling edge and level low triggers not supported by the GPIO controller
    switch (type) {
    case IRQ_TYPE_NONE:
    set = GNR_CFG_DW_RX_DISABLE;
    break;
    case IRQ_TYPE_EDGE_RISING:
    set = GNR_CFG_DW_RX_EDGE;
    irq_set_handler_locked(d, handle_edge_irq);
    break;
    case IRQ_TYPE_LEVEL_HIGH:
    set = GNR_CFG_DW_RX_LEVEL;
    irq_set_handler_locked(d, handle_level_irq);
    break;
    default:
    return -EINVAL;
    }
    return gnr_gpio_configure_line(gc, hwirq, GNR_CFG_DW_RX_MASK, set);
    }
    static const struct irq_chip gnr_gpio_irq_chip = {
    .name		= "gpio-graniterapids",
    .irq_ack	= gnr_gpio_irq_ack,
    .irq_mask	= gnr_gpio_irq_mask,
    .irq_unmask	= gnr_gpio_irq_unmask,
    .irq_set_type	= gnr_gpio_irq_set_type,
    .flags		= IRQCHIP_IMMUTABLE,
    GPIOCHIP_IRQ_RESOURCE_HELPERS,
    };
    static void gnr_gpio_init_pin_ro_bits(struct device *dev,
    const void __iomem *cfg_lock_base,
    unsigned long *ro_bitmap)
    {
    u32 tmp[GNR_NUM_REGS];
    memcpy_fromio(tmp, cfg_lock_base, sizeof(tmp));
    bitmap_from_arr32(ro_bitmap, tmp, GNR_NUM_PINS);
    }
#[no_mangle]
unsafe extern "C" fn gnr_gpio_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t gnr_gpio_irq(int irq, void *data)
    {
    struct gnr_gpio *priv = data;
    let mut handled: c_uint = 0;
    for (unsigned int i = 0; i < GNR_NUM_REGS; i++) {
    const void __iomem *reg = priv.reg_base + i * sizeof(u32);
    unsigned long pending;
    unsigned long enabled;
    unsigned int bit_idx;
    scoped_guard(raw_spinlock, &priv.lock) {
    pending = readl(reg + GNR_GPI_STATUS_OFFSET);
    enabled = readl(reg + GNR_GPI_ENABLE_OFFSET);
    }
// Only enabled interrupts
    pending &= enabled;
    for_each_set_bit(bit_idx, &pending, GNR_PINS_PER_REG) {
    let mut hwirq: c_uint = i * GNR_PINS_PER_REG + bit_idx;
    generic_handle_domain_irq(priv.gc.irq.domain, hwirq);
    }
    handled += pending ? 1 : 0;
    }
    return IRQ_RETVAL(handled);
    }
#[no_mangle]
unsafe extern "C" fn gnr_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int gnr_gpio_probe(struct platform_device *pdev)
    {
    let mut num_backup_pins: usize = IS_ENABLED(CONFIG_PM_SLEEP) ? GNR_NUM_PINS : 0;
    struct device *dev = &pdev.dev;
    struct gpio_irq_chip *girq;
    struct gnr_gpio *priv;
    void __iomem *regs;
    int irq, ret;
    u32 offset;
    priv = devm_kzalloc(dev, struct_size(priv, pad_backup, num_backup_pins), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    raw_spin_lock_init(&priv.lock);
    regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(regs))
    return PTR_ERR(regs);
    priv.reg_base = regs;
    offset = readl(priv.reg_base + GNR_CFG_PADBAR);
    priv.pad_base = priv.reg_base + offset;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    ret = devm_request_irq(dev, irq, gnr_gpio_irq, IRQF_SHARED | IRQF_NO_THREAD,
    dev_name(dev), priv);
    if (ret)
    return dev_err_probe(dev, ret, "failed to request interrupt\n");
    gnr_gpio_init_pin_ro_bits(dev, priv.reg_base + GNR_CFG_LOCK_OFFSET,
    priv.ro_bitmap);
    priv.gc	= gnr_gpio_chip;
    priv.gc.label	= dev_name(dev);
    priv.gc.parent	= dev;
    priv.gc.ngpio	= GNR_NUM_PINS;
    priv.gc.base	= -1;
    girq = &priv.gc.irq;
    gpio_irq_chip_set_chip(girq, &gnr_gpio_irq_chip);
    girq.parent_handler	= core::ptr::null_mut();
    girq.num_parents	= 0;
    girq.parents		= core::ptr::null_mut();
    girq.default_type	= IRQ_TYPE_NONE;
    girq.handler		= handle_bad_irq;
    platform_set_drvdata(pdev, priv);
    return devm_gpiochip_add_data(dev, &priv.gc, priv);
    }
#[no_mangle]
unsafe extern "C" fn gnr_gpio_suspend(dev: *mut device) -> c_int {
    static int gnr_gpio_suspend(struct device *dev)
    {
    struct gnr_gpio *priv = dev_get_drvdata(dev);
    unsigned int i;
    guard(raw_spinlock_irqsave)(&priv.lock);
    for_each_clear_bit(i, priv.ro_bitmap, priv.gc.ngpio)
    priv.pad_backup[i] = readl(gnr_gpio_get_padcfg_addr(priv, i));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn gnr_gpio_resume(dev: *mut device) -> c_int {
    static int gnr_gpio_resume(struct device *dev)
    {
    struct gnr_gpio *priv = dev_get_drvdata(dev);
    unsigned int i;
    guard(raw_spinlock_irqsave)(&priv.lock);
    for_each_clear_bit(i, priv.ro_bitmap, priv.gc.ngpio)
    writel(priv.pad_backup[i], gnr_gpio_get_padcfg_addr(priv, i));
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(gnr_gpio_pm_ops, gnr_gpio_suspend, gnr_gpio_resume);
    static const struct acpi_device_id gnr_gpio_acpi_match[] = {
    { "INTC1109" },
    {}
    };
    MODULE_DEVICE_TABLE(acpi, gnr_gpio_acpi_match);
    static struct platform_driver gnr_gpio_driver = {
    .driver = {
    .name		  = "gpio-graniterapids",
    .pm		  = pm_sleep_ptr(&gnr_gpio_pm_ops),
    .acpi_match_table = gnr_gpio_acpi_match,
    },
    .probe = gnr_gpio_probe,
    };
    module_platform_driver(gnr_gpio_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Aapo Vienamo <aapo.vienamo@linux.intel.com>");
    MODULE_DESCRIPTION("Intel Granite Rapids-D vGPIO driver");
