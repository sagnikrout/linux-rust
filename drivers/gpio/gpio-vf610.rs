//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-vf610.c
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
// Freescale vf610 GPIO support through PORT and GPIO
//
// Copyright (c) 2014 Toradex AG.
//
// Author: Stefan Agner <stefan@agner.ch>.
//

pub const VF610_GPIO_PER_PORT: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsl_gpio_soc_data {
// SoCs has a Port Data Direction Register (PDDR)
    pub have_paddr: bool,
    pub have_dual_base: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vf610_gpio_port {
    pub chip: gpio_generic_chip,
    pub base: *mut void __iomem,
    pub gpio_base: *mut void __iomem,
    pub sdata: *const fsl_gpio_soc_data,
    pub irqc: [u8; VF610_GPIO_PER_PORT],
    pub clk_port: *mut clk,
    pub clk_gpio: *mut clk,
    pub irq: c_int,
}

pub const GPIO_PDOR: c_uint = 0x00;
pub const GPIO_PSOR: c_uint = 0x04;
pub const GPIO_PCOR: c_uint = 0x08;
pub const GPIO_PTOR: c_uint = 0x0c;
pub const GPIO_PDIR: c_uint = 0x10;
pub const GPIO_PDDR: c_uint = 0x14;

pub const PORT_PCR_IRQC_OFFSET: c_int = 16;
pub const PORT_ISFR: c_uint = 0xa0;
pub const PORT_DFER: c_uint = 0xc0;
pub const PORT_DFCR: c_uint = 0xc4;
pub const PORT_DFWR: c_uint = 0xc8;
pub const PORT_INT_OFF: c_uint = 0x0;
pub const PORT_INT_LOGIC_ZERO: c_uint = 0x8;
pub const PORT_INT_RISING_EDGE: c_uint = 0x9;
pub const PORT_INT_FALLING_EDGE: c_uint = 0xa;
pub const PORT_INT_EITHER_EDGE: c_uint = 0xb;
pub const PORT_INT_LOGIC_ONE: c_uint = 0xc;
pub const IMX8ULP_GPIO_BASE_OFF: c_uint = 0x40;
pub const IMX8ULP_BASE_OFF: c_uint = 0x80;
    static const struct fsl_gpio_soc_data vf610_data = {
    .have_dual_base = true,
    };
    static const struct fsl_gpio_soc_data imx_data = {
    .have_paddr = true,
    .have_dual_base = true,
    };
    static const struct fsl_gpio_soc_data imx8ulp_data = {
    .have_paddr = true,
    };
    static const struct of_device_id vf610_gpio_dt_ids[] = {
    { .compatible = "fsl,vf610-gpio",	.data = &vf610_data },
    { .compatible = "fsl,imx7ulp-gpio",	.data = &imx_data, },
    { .compatible = "fsl,imx8ulp-gpio",	.data = &imx8ulp_data, },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, vf610_gpio_dt_ids);
#[no_mangle]
pub unsafe extern "C" fn vf610_gpio_writel(val: u32, reg: *mut void __iomem) {
    static inline void vf610_gpio_writel(u32 val, void __iomem *reg)
    {
    writel_relaxed(val, reg);
    }
#[no_mangle]
pub unsafe extern "C" fn vf610_gpio_readl(reg: *mut void __iomem) -> u32 {
    static inline u32 vf610_gpio_readl(void __iomem *reg)
    {
    return readl_relaxed(reg);
    }
#[no_mangle]
unsafe extern "C" fn vf610_gpio_irq_handler(desc: *mut irq_desc) {
    static void vf610_gpio_irq_handler(struct irq_desc *desc)
    {
    struct vf610_gpio_port *port =
    gpiochip_get_data(irq_desc_get_handler_data(desc));
    struct irq_chip *chip = irq_desc_get_chip(desc);
    int pin;
    unsigned long irq_isfr;
    chained_irq_enter(chip, desc);
    irq_isfr = vf610_gpio_readl(port.base + PORT_ISFR);
    for_each_set_bit(pin, &irq_isfr, VF610_GPIO_PER_PORT) {
    vf610_gpio_writel(BIT(pin), port.base + PORT_ISFR);
    generic_handle_domain_irq(port.chip.gc.irq.domain, pin);
    }
    chained_irq_exit(chip, desc);
    }
#[no_mangle]
unsafe extern "C" fn vf610_gpio_irq_ack(d: *mut irq_data) {
    static void vf610_gpio_irq_ack(struct irq_data *d)
    {
    struct vf610_gpio_port *port =
    gpiochip_get_data(irq_data_get_irq_chip_data(d));
    let mut gpio: c_int = d.hwirq;
    vf610_gpio_writel(BIT(gpio), port.base + PORT_ISFR);
    }
#[no_mangle]
unsafe extern "C" fn vf610_gpio_irq_set_type(d: *mut irq_data, type: u32) -> c_int {
    static int vf610_gpio_irq_set_type(struct irq_data *d, u32 type)
    {
    struct vf610_gpio_port *port =
    gpiochip_get_data(irq_data_get_irq_chip_data(d));
    u8 irqc;
    switch (type) {
    case IRQ_TYPE_EDGE_RISING:
    irqc = PORT_INT_RISING_EDGE;
    break;
    case IRQ_TYPE_EDGE_FALLING:
    irqc = PORT_INT_FALLING_EDGE;
    break;
    case IRQ_TYPE_EDGE_BOTH:
    irqc = PORT_INT_EITHER_EDGE;
    break;
    case IRQ_TYPE_LEVEL_LOW:
    irqc = PORT_INT_LOGIC_ZERO;
    break;
    case IRQ_TYPE_LEVEL_HIGH:
    irqc = PORT_INT_LOGIC_ONE;
    break;
    default:
    return -EINVAL;
    }
    port.irqc[d.hwirq] = irqc;
    if (type & IRQ_TYPE_LEVEL_MASK)
    irq_set_handler_locked(d, handle_level_irq);
    else
    irq_set_handler_locked(d, handle_edge_irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vf610_gpio_irq_mask(d: *mut irq_data) {
    static void vf610_gpio_irq_mask(struct irq_data *d)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct vf610_gpio_port *port = gpiochip_get_data(gc);
    let mut gpio_num: irq_hw_number_t = irqd_to_hwirq(d);
    void __iomem *pcr_base = port.base + PORT_PCR(gpio_num);
    vf610_gpio_writel(0, pcr_base);
    gpiochip_disable_irq(gc, gpio_num);
    }
#[no_mangle]
unsafe extern "C" fn vf610_gpio_irq_unmask(d: *mut irq_data) {
    static void vf610_gpio_irq_unmask(struct irq_data *d)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct vf610_gpio_port *port = gpiochip_get_data(gc);
    let mut gpio_num: irq_hw_number_t = irqd_to_hwirq(d);
    void __iomem *pcr_base = port.base + PORT_PCR(gpio_num);
    gpiochip_enable_irq(gc, gpio_num);
    vf610_gpio_writel(port.irqc[gpio_num] << PORT_PCR_IRQC_OFFSET,
    pcr_base);
    }
#[no_mangle]
unsafe extern "C" fn vf610_gpio_irq_set_wake(d: *mut irq_data, enable: u32) -> c_int {
    static int vf610_gpio_irq_set_wake(struct irq_data *d, u32 enable)
    {
    struct vf610_gpio_port *port =
    gpiochip_get_data(irq_data_get_irq_chip_data(d));
    if (enable)
    enable_irq_wake(port.irq);
    else
    disable_irq_wake(port.irq);
    return 0;
    }
    static const struct irq_chip vf610_irqchip = {
    .name = "gpio-vf610",
    .irq_ack = vf610_gpio_irq_ack,
    .irq_mask = vf610_gpio_irq_mask,
    .irq_unmask = vf610_gpio_irq_unmask,
    .irq_set_type = vf610_gpio_irq_set_type,
    .irq_set_wake = vf610_gpio_irq_set_wake,
    .flags = IRQCHIP_IMMUTABLE | IRQCHIP_MASK_ON_SUSPEND
    | IRQCHIP_ENABLE_WAKEUP_ON_SUSPEND,
    GPIOCHIP_IRQ_RESOURCE_HELPERS,
    };
#[no_mangle]
unsafe extern "C" fn vf610_gpio_disable_clk(data: *mut c_void) {
    static void vf610_gpio_disable_clk(void *data)
    {
    clk_disable_unprepare(data);
    }
#[no_mangle]
unsafe extern "C" fn vf610_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int vf610_gpio_probe(struct platform_device *pdev)
    {
    struct gpio_generic_chip_config config;
    struct device *dev = &pdev.dev;
    struct vf610_gpio_port *port;
    struct gpio_chip *gc;
    struct gpio_irq_chip *girq;
    unsigned long flags;
    int i;
    int ret;
    bool dual_base;
    port = devm_kzalloc(dev, sizeof(*port), GFP_KERNEL);
    if (!port)
    return -ENOMEM;
    port.sdata = device_get_match_data(dev);
    dual_base = port.sdata.have_dual_base;
//
// Handle legacy compatible combinations which used two reg values
// for the i.MX8ULP and i.MX93.
//
    if (device_is_compatible(dev, "fsl,imx7ulp-gpio") &&
    (device_is_compatible(dev, "fsl,imx93-gpio") ||
    (device_is_compatible(dev, "fsl,imx8ulp-gpio"))))
    dual_base = true;
    if (dual_base) {
    port.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(port.base))
    return PTR_ERR(port.base);
    port.gpio_base = devm_platform_ioremap_resource(pdev, 1);
    if (IS_ERR(port.gpio_base))
    return PTR_ERR(port.gpio_base);
    } else {
    port.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(port.base))
    return PTR_ERR(port.base);
    port.gpio_base = port.base + IMX8ULP_GPIO_BASE_OFF;
    port.base = port.base + IMX8ULP_BASE_OFF;
    }
    port.irq = platform_get_irq(pdev, 0);
    if (port.irq < 0)
    return port.irq;
    port.clk_port = devm_clk_get(dev, "port");
    ret = PTR_ERR_OR_ZERO(port.clk_port);
    if (!ret) {
    ret = clk_prepare_enable(port.clk_port);
    if (ret)
    return ret;
    ret = devm_add_action_or_reset(dev, vf610_gpio_disable_clk,
    port.clk_port);
    if (ret)
    return ret;
    } else if (ret == -EPROBE_DEFER) {
//
// Percolate deferrals, for anything else,
// just live without the clocking.
//
    return ret;
    }
    port.clk_gpio = devm_clk_get(dev, "gpio");
    ret = PTR_ERR_OR_ZERO(port.clk_gpio);
    if (!ret) {
    ret = clk_prepare_enable(port.clk_gpio);
    if (ret)
    return ret;
    ret = devm_add_action_or_reset(dev, vf610_gpio_disable_clk,
    port.clk_gpio);
    if (ret)
    return ret;
    } else if (ret == -EPROBE_DEFER) {
    return ret;
    }
    gc = &port.chip.gc;
    flags = GPIO_GENERIC_PINCTRL_BACKEND;
//
// We only read the output register for current value on output
// lines if the direction register is available so we can switch
// direction.
//
    if (port.sdata.have_paddr)
    flags |= GPIO_GENERIC_READ_OUTPUT_REG_SET;
    config = (struct gpio_generic_chip_config) {
    .dev = dev,
    .sz = 4,
    .dat = port.gpio_base + GPIO_PDIR,
    .set = port.gpio_base + GPIO_PDOR,
    .dirout = port.sdata.have_paddr ?
    port.gpio_base + GPIO_PDDR : core::ptr::null_mut(),
    .flags = flags,
    };
    ret = gpio_generic_chip_init(&port.chip, &config);
    if (ret)
    return dev_err_probe(dev, ret, "unable to init generic GPIO\n");
    gc.label = dev_name(dev);
    gc.base = -1;
// Mask all GPIO interrupts
    for (i = 0; i < gc.ngpio; i++)
    vf610_gpio_writel(0, port.base + PORT_PCR(i));
// Clear the interrupt status register for all GPIO's
    vf610_gpio_writel(~0, port.base + PORT_ISFR);
    girq = &gc.irq;
    gpio_irq_chip_set_chip(girq, &vf610_irqchip);
    girq.parent_handler = vf610_gpio_irq_handler;
    girq.num_parents = 1;
    girq.parents = devm_kcalloc(&pdev.dev, 1,
    sizeof(*girq.parents),
    GFP_KERNEL);
    if (!girq.parents)
    return -ENOMEM;
    girq.parents[0] = port.irq;
    girq.default_type = IRQ_TYPE_NONE;
    girq.handler = handle_edge_irq;
    return devm_gpiochip_add_data(dev, gc, port);
    }
    static struct platform_driver vf610_gpio_driver = {
    .driver		= {
    .name	= "gpio-vf610",
    .of_match_table = vf610_gpio_dt_ids,
    },
    .probe		= vf610_gpio_probe,
    };
    module_platform_driver(vf610_gpio_driver);
    MODULE_DESCRIPTION("VF610 GPIO driver");
    MODULE_LICENSE("GPL");
