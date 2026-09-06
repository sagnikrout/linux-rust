//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-hisi.c
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
// Copyright (c) 2020 HiSilicon Limited.

pub const HISI_GPIO_SWPORT_DR_SET_WX: c_uint = 0x000;
pub const HISI_GPIO_SWPORT_DR_CLR_WX: c_uint = 0x004;
pub const HISI_GPIO_SWPORT_DDR_SET_WX: c_uint = 0x010;
pub const HISI_GPIO_SWPORT_DDR_CLR_WX: c_uint = 0x014;
pub const HISI_GPIO_SWPORT_DDR_ST_WX: c_uint = 0x018;
pub const HISI_GPIO_INTEN_SET_WX: c_uint = 0x020;
pub const HISI_GPIO_INTEN_CLR_WX: c_uint = 0x024;
pub const HISI_GPIO_INTMASK_SET_WX: c_uint = 0x030;
pub const HISI_GPIO_INTMASK_CLR_WX: c_uint = 0x034;
pub const HISI_GPIO_INTTYPE_EDGE_SET_WX: c_uint = 0x040;
pub const HISI_GPIO_INTTYPE_EDGE_CLR_WX: c_uint = 0x044;
pub const HISI_GPIO_INT_POLARITY_SET_WX: c_uint = 0x050;
pub const HISI_GPIO_INT_POLARITY_CLR_WX: c_uint = 0x054;
pub const HISI_GPIO_DEBOUNCE_SET_WX: c_uint = 0x060;
pub const HISI_GPIO_DEBOUNCE_CLR_WX: c_uint = 0x064;
pub const HISI_GPIO_INTSTATUS_WX: c_uint = 0x070;
pub const HISI_GPIO_PORTA_EOI_WX: c_uint = 0x078;
pub const HISI_GPIO_EXT_PORT_WX: c_uint = 0x080;
pub const HISI_GPIO_INTCOMB_MASK_WX: c_uint = 0x0a0;
pub const HISI_GPIO_INT_DEDGE_SET: c_uint = 0x0b0;
pub const HISI_GPIO_INT_DEDGE_CLR: c_uint = 0x0b4;
pub const HISI_GPIO_INT_DEDGE_ST: c_uint = 0x0b8;
pub const HISI_GPIO_LINE_NUM_MAX: c_int = 32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hisi_gpio {
    pub chip: gpio_generic_chip,
    pub dev: *mut device,
    pub reg_base: *mut void __iomem,
    pub line_num: c_uint,
    pub irq: c_int,
}

    static inline u32 hisi_gpio_read_reg(struct gpio_chip *chip,
    unsigned int off)
    {
    struct hisi_gpio *hisi_gpio = container_of(to_gpio_generic_chip(chip),
    struct hisi_gpio, chip);
    void __iomem *reg = hisi_gpio.reg_base + off;
    return readl(reg);
    }
    static inline void hisi_gpio_write_reg(struct gpio_chip *chip,
    unsigned int off, u32 val)
    {
    struct hisi_gpio *hisi_gpio = container_of(to_gpio_generic_chip(chip),
    struct hisi_gpio, chip);
    void __iomem *reg = hisi_gpio.reg_base + off;
    writel(val, reg);
    }
    static void hisi_gpio_set_debounce(struct gpio_chip *chip, unsigned int off,
    u32 debounce)
    {
    if (debounce)
    hisi_gpio_write_reg(chip, HISI_GPIO_DEBOUNCE_SET_WX, BIT(off));
    else
    hisi_gpio_write_reg(chip, HISI_GPIO_DEBOUNCE_CLR_WX, BIT(off));
    }
    static int hisi_gpio_set_config(struct gpio_chip *chip, unsigned int offset,
    unsigned long config)
    {
    let mut config_para: u32 = pinconf_to_config_param(config);
    u32 config_arg;
    switch (config_para) {
    case PIN_CONFIG_INPUT_DEBOUNCE:
    config_arg = pinconf_to_config_argument(config);
    hisi_gpio_set_debounce(chip, offset, config_arg);
    break;
    default:
    return -ENOTSUPP;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hisi_gpio_set_ack(d: *mut irq_data) {
    static void hisi_gpio_set_ack(struct irq_data *d)
    {
    struct gpio_chip *chip = irq_data_get_irq_chip_data(d);
    hisi_gpio_write_reg(chip, HISI_GPIO_PORTA_EOI_WX, BIT(irqd_to_hwirq(d)));
    }
#[no_mangle]
unsafe extern "C" fn hisi_gpio_irq_set_mask(d: *mut irq_data) {
    static void hisi_gpio_irq_set_mask(struct irq_data *d)
    {
    struct gpio_chip *chip = irq_data_get_irq_chip_data(d);
    hisi_gpio_write_reg(chip, HISI_GPIO_INTMASK_SET_WX, BIT(irqd_to_hwirq(d)));
    gpiochip_disable_irq(chip, irqd_to_hwirq(d));
    }
#[no_mangle]
unsafe extern "C" fn hisi_gpio_irq_clr_mask(d: *mut irq_data) {
    static void hisi_gpio_irq_clr_mask(struct irq_data *d)
    {
    struct gpio_chip *chip = irq_data_get_irq_chip_data(d);
    gpiochip_enable_irq(chip, irqd_to_hwirq(d));
    hisi_gpio_write_reg(chip, HISI_GPIO_INTMASK_CLR_WX, BIT(irqd_to_hwirq(d)));
    }
#[no_mangle]
unsafe extern "C" fn hisi_gpio_irq_set_type(d: *mut irq_data, type: u32) -> c_int {
    static int hisi_gpio_irq_set_type(struct irq_data *d, u32 type)
    {
    struct gpio_chip *chip = irq_data_get_irq_chip_data(d);
    let mut mask: c_uint = BIT(irqd_to_hwirq(d));
    switch (type) {
    case IRQ_TYPE_EDGE_BOTH:
    hisi_gpio_write_reg(chip, HISI_GPIO_INT_DEDGE_SET, mask);
    break;
    case IRQ_TYPE_EDGE_RISING:
    hisi_gpio_write_reg(chip, HISI_GPIO_INTTYPE_EDGE_SET_WX, mask);
    hisi_gpio_write_reg(chip, HISI_GPIO_INT_POLARITY_SET_WX, mask);
    break;
    case IRQ_TYPE_EDGE_FALLING:
    hisi_gpio_write_reg(chip, HISI_GPIO_INTTYPE_EDGE_SET_WX, mask);
    hisi_gpio_write_reg(chip, HISI_GPIO_INT_POLARITY_CLR_WX, mask);
    break;
    case IRQ_TYPE_LEVEL_HIGH:
    hisi_gpio_write_reg(chip, HISI_GPIO_INTTYPE_EDGE_CLR_WX, mask);
    hisi_gpio_write_reg(chip, HISI_GPIO_INT_POLARITY_SET_WX, mask);
    break;
    case IRQ_TYPE_LEVEL_LOW:
    hisi_gpio_write_reg(chip, HISI_GPIO_INTTYPE_EDGE_CLR_WX, mask);
    hisi_gpio_write_reg(chip, HISI_GPIO_INT_POLARITY_CLR_WX, mask);
    break;
    default:
    return -EINVAL;
    }
//
// The dual-edge interrupt and other interrupt's registers do not
// take effect at the same time. The registers of the two-edge
// interrupts have higher priorities, the configuration of
// the dual-edge interrupts must be disabled before the configuration
// of other kind of interrupts.
//
    if (type != IRQ_TYPE_EDGE_BOTH) {
    let mut both: c_uint = hisi_gpio_read_reg(chip, HISI_GPIO_INT_DEDGE_ST);
    if (both & mask)
    hisi_gpio_write_reg(chip, HISI_GPIO_INT_DEDGE_CLR, mask);
    }
    if (type & IRQ_TYPE_LEVEL_MASK)
    irq_set_handler_locked(d, handle_level_irq);
#[no_mangle]
pub unsafe extern "C" fn if(IRQ_TYPE_EDGE_BOTH: type &) -> else {
    else if (type & IRQ_TYPE_EDGE_BOTH)
    irq_set_handler_locked(d, handle_edge_irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hisi_gpio_irq_enable(d: *mut irq_data) {
    static void hisi_gpio_irq_enable(struct irq_data *d)
    {
    struct gpio_chip *chip = irq_data_get_irq_chip_data(d);
    hisi_gpio_irq_clr_mask(d);
    hisi_gpio_write_reg(chip, HISI_GPIO_INTEN_SET_WX, BIT(irqd_to_hwirq(d)));
    }
#[no_mangle]
unsafe extern "C" fn hisi_gpio_irq_disable(d: *mut irq_data) {
    static void hisi_gpio_irq_disable(struct irq_data *d)
    {
    struct gpio_chip *chip = irq_data_get_irq_chip_data(d);
    hisi_gpio_irq_set_mask(d);
    hisi_gpio_write_reg(chip, HISI_GPIO_INTEN_CLR_WX, BIT(irqd_to_hwirq(d)));
    }
#[no_mangle]
unsafe extern "C" fn hisi_gpio_irq_handler(desc: *mut irq_desc) {
    static void hisi_gpio_irq_handler(struct irq_desc *desc)
    {
    struct hisi_gpio *hisi_gpio = irq_desc_get_handler_data(desc);
    unsigned long irq_msk = hisi_gpio_read_reg(&hisi_gpio.chip.gc,
    HISI_GPIO_INTSTATUS_WX);
    struct irq_chip *irq_c = irq_desc_get_chip(desc);
    int hwirq;
    chained_irq_enter(irq_c, desc);
    for_each_set_bit(hwirq, &irq_msk, HISI_GPIO_LINE_NUM_MAX)
    generic_handle_domain_irq(hisi_gpio.chip.gc.irq.domain,
    hwirq);
    chained_irq_exit(irq_c, desc);
    }
    static const struct irq_chip hisi_gpio_irq_chip = {
    .name = "HISI-GPIO",
    .irq_ack = hisi_gpio_set_ack,
    .irq_mask = hisi_gpio_irq_set_mask,
    .irq_unmask = hisi_gpio_irq_clr_mask,
    .irq_set_type = hisi_gpio_irq_set_type,
    .irq_enable = hisi_gpio_irq_enable,
    .irq_disable = hisi_gpio_irq_disable,
    .flags = IRQCHIP_IMMUTABLE,
    GPIOCHIP_IRQ_RESOURCE_HELPERS,
    };
#[no_mangle]
unsafe extern "C" fn hisi_gpio_init_irq(hisi_gpio: *mut hisi_gpio) {
    static void hisi_gpio_init_irq(struct hisi_gpio *hisi_gpio)
    {
    struct gpio_chip *chip = &hisi_gpio.chip.gc;
    struct gpio_irq_chip *girq_chip = &chip.irq;
    gpio_irq_chip_set_chip(girq_chip, &hisi_gpio_irq_chip);
    girq_chip.default_type = IRQ_TYPE_NONE;
    girq_chip.num_parents = 1;
    girq_chip.parents = &hisi_gpio.irq;
    girq_chip.parent_handler = hisi_gpio_irq_handler;
    girq_chip.parent_handler_data = hisi_gpio;
// Clear Mask of GPIO controller combine IRQ
    hisi_gpio_write_reg(chip, HISI_GPIO_INTCOMB_MASK_WX, 1);
    }
    static const struct acpi_device_id hisi_gpio_acpi_match[] = {
    {"HISI0184", 0},
    {}
    };
    MODULE_DEVICE_TABLE(acpi, hisi_gpio_acpi_match);
    static const struct of_device_id hisi_gpio_dts_match[] = {
    { .compatible = "hisilicon,ascend910-gpio", },
    { }
    };
    MODULE_DEVICE_TABLE(of, hisi_gpio_dts_match);
    static void hisi_gpio_get_pdata(struct device *dev,
    struct hisi_gpio *hisi_gpio)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct fwnode_handle *fwnode;
    let mut idx: c_int = 0;
    device_for_each_child_node(dev, fwnode)  {
// Cycle for once, no need for an array to save line_num
    if (fwnode_property_read_u32(fwnode, "ngpios",
    &hisi_gpio.line_num)) {
    dev_err(dev,
    "failed to get number of lines for port%d and use default value instead\n",
    idx);
    hisi_gpio.line_num = HISI_GPIO_LINE_NUM_MAX;
    }
    if (WARN_ON(hisi_gpio.line_num > HISI_GPIO_LINE_NUM_MAX))
    hisi_gpio.line_num = HISI_GPIO_LINE_NUM_MAX;
    hisi_gpio.irq = platform_get_irq(pdev, idx);
    dev_info(dev,
    "get hisi_gpio[%d] with %u lines\n", idx,
    hisi_gpio.line_num);
    idx++;
    }
    }
#[no_mangle]
unsafe extern "C" fn hisi_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int hisi_gpio_probe(struct platform_device *pdev)
    {
    struct gpio_generic_chip_config config;
    struct device *dev = &pdev.dev;
    struct hisi_gpio *hisi_gpio;
    int port_num;
    int ret;
//
// One GPIO controller own one port currently,
// if we get more from ACPI table, return error.
//
    port_num = device_get_child_node_count(dev);
    if (WARN_ON(port_num != 1))
    return -ENODEV;
    hisi_gpio = devm_kzalloc(dev, sizeof(*hisi_gpio), GFP_KERNEL);
    if (!hisi_gpio)
    return -ENOMEM;
    hisi_gpio.reg_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(hisi_gpio.reg_base))
    return PTR_ERR(hisi_gpio.reg_base);
    hisi_gpio_get_pdata(dev, hisi_gpio);
    hisi_gpio.dev = dev;
    config = (struct gpio_generic_chip_config) {
    .dev = hisi_gpio.dev,
    .sz = 4,
    .dat = hisi_gpio.reg_base + HISI_GPIO_EXT_PORT_WX,
    .set = hisi_gpio.reg_base + HISI_GPIO_SWPORT_DR_SET_WX,
    .clr = hisi_gpio.reg_base + HISI_GPIO_SWPORT_DR_CLR_WX,
    .dirout = hisi_gpio.reg_base + HISI_GPIO_SWPORT_DDR_SET_WX,
    .dirin = hisi_gpio.reg_base + HISI_GPIO_SWPORT_DDR_CLR_WX,
    .flags = GPIO_GENERIC_NO_SET_ON_INPUT |
    GPIO_GENERIC_UNREADABLE_REG_DIR,
    };
    ret = gpio_generic_chip_init(&hisi_gpio.chip, &config);
    if (ret) {
    dev_err(dev, "failed to init, ret = %d\n", ret);
    return ret;
    }
    hisi_gpio.chip.gc.set_config = hisi_gpio_set_config;
    hisi_gpio.chip.gc.ngpio = hisi_gpio.line_num;
    hisi_gpio.chip.gc.base = -1;
    if (hisi_gpio.irq > 0)
    hisi_gpio_init_irq(hisi_gpio);
    ret = devm_gpiochip_add_data(dev, &hisi_gpio.chip.gc, hisi_gpio);
    if (ret) {
    dev_err(dev, "failed to register gpiochip, ret = %d\n", ret);
    return ret;
    }
    return 0;
    }
    static struct platform_driver hisi_gpio_driver = {
    .driver		= {
    .name	= HISI_GPIO_DRIVER_NAME,
    .acpi_match_table = hisi_gpio_acpi_match,
    .of_match_table = hisi_gpio_dts_match,
    },
    .probe		= hisi_gpio_probe,
    };
    module_platform_driver(hisi_gpio_driver);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Luo Jiaxing <luojiaxing@huawei.com>");
    MODULE_DESCRIPTION("HiSilicon GPIO controller driver");
    MODULE_ALIAS("platform:" HISI_GPIO_DRIVER_NAME);
