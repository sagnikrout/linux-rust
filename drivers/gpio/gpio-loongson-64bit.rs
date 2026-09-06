//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-loongson-64bit.c
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
// Loongson GPIO Support
//
// Copyright (C) 2022-2023 Loongson Technology Corporation Limited
//

    enum loongson_gpio_mode {
    BIT_CTRL_MODE,
    BYTE_CTRL_MODE,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct loongson_gpio_chip_data {
    pub label: *const c_char,
    pub mode: enum loongson_gpio_mode,
    pub conf_offset: c_uint,
    pub out_offset: c_uint,
    pub in_offset: c_uint,
    pub inten_offset: c_uint,
    pub intpol_offset: c_uint,
    pub intedge_offset: c_uint,
    pub intclr_offset: c_uint,
    pub intsts_offset: c_uint,
    pub intdual_offset: c_uint,
    pub intr_num: c_uint,
    pub irq_handler: irq_flow_handler_t,
    pub girqchip: *const irq_chip,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct loongson_gpio_chip {
    pub chip: gpio_generic_chip,
    pub lock: spinlock_t,
    pub reg_base: *mut void __iomem,
    pub chip_data: *const loongson_gpio_chip_data,
}

    static inline struct loongson_gpio_chip *to_loongson_gpio_chip(struct gpio_chip *chip)
    {
    return container_of(to_gpio_generic_chip(chip),
    struct loongson_gpio_chip, chip);
    }
    static inline void loongson_commit_direction(struct loongson_gpio_chip *lgpio, unsigned int pin,
    int input)
    {
    let mut bval: u8 = input ? 1 : 0;
    writeb(bval, lgpio.reg_base + lgpio.chip_data.conf_offset + pin);
    }
#[no_mangle]
unsafe extern "C" fn loongson_commit_level(lgpio: *mut loongson_gpio_chip, pin: c_uint, high: c_int) {
    static void loongson_commit_level(struct loongson_gpio_chip *lgpio, unsigned int pin, int high)
    {
    let mut bval: u8 = high ? 1 : 0;
    writeb(bval, lgpio.reg_base + lgpio.chip_data.out_offset + pin);
    }
#[no_mangle]
unsafe extern "C" fn loongson_gpio_direction_input(chip: *mut gpio_chip, pin: c_uint) -> c_int {
    static int loongson_gpio_direction_input(struct gpio_chip *chip, unsigned int pin)
    {
    unsigned long flags;
    struct loongson_gpio_chip *lgpio = to_loongson_gpio_chip(chip);
    spin_lock_irqsave(&lgpio.lock, flags);
    loongson_commit_direction(lgpio, pin, 1);
    spin_unlock_irqrestore(&lgpio.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn loongson_gpio_direction_output(chip: *mut gpio_chip, pin: c_uint, value: c_int) -> c_int {
    static int loongson_gpio_direction_output(struct gpio_chip *chip, unsigned int pin, int value)
    {
    unsigned long flags;
    struct loongson_gpio_chip *lgpio = to_loongson_gpio_chip(chip);
    spin_lock_irqsave(&lgpio.lock, flags);
    loongson_commit_level(lgpio, pin, value);
    loongson_commit_direction(lgpio, pin, 0);
    spin_unlock_irqrestore(&lgpio.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn loongson_gpio_get(chip: *mut gpio_chip, pin: c_uint) -> c_int {
    static int loongson_gpio_get(struct gpio_chip *chip, unsigned int pin)
    {
    u8  bval;
    int val;
    struct loongson_gpio_chip *lgpio = to_loongson_gpio_chip(chip);
    bval = readb(lgpio.reg_base + lgpio.chip_data.in_offset + pin);
    val = bval & 1;
    return val;
    }
#[no_mangle]
unsafe extern "C" fn loongson_gpio_get_direction(chip: *mut gpio_chip, pin: c_uint) -> c_int {
    static int loongson_gpio_get_direction(struct gpio_chip *chip, unsigned int pin)
    {
    u8  bval;
    struct loongson_gpio_chip *lgpio = to_loongson_gpio_chip(chip);
    bval = readb(lgpio.reg_base + lgpio.chip_data.conf_offset + pin);
    if (bval & 1)
    return GPIO_LINE_DIRECTION_IN;
    return GPIO_LINE_DIRECTION_OUT;
    }
#[no_mangle]
unsafe extern "C" fn loongson_gpio_set(chip: *mut gpio_chip, pin: c_uint, value: c_int) -> c_int {
    static int loongson_gpio_set(struct gpio_chip *chip, unsigned int pin, int value)
    {
    unsigned long flags;
    struct loongson_gpio_chip *lgpio = to_loongson_gpio_chip(chip);
    spin_lock_irqsave(&lgpio.lock, flags);
    loongson_commit_level(lgpio, pin, value);
    spin_unlock_irqrestore(&lgpio.lock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn loongson_gpio_to_irq(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    static int loongson_gpio_to_irq(struct gpio_chip *chip, unsigned int offset)
    {
    unsigned int u;
    struct platform_device *pdev = to_platform_device(chip.parent);
    struct loongson_gpio_chip *lgpio = to_loongson_gpio_chip(chip);
    if (lgpio.chip_data.mode == BIT_CTRL_MODE) {
// Get the register index from offset then multiply by bytes per register
    u = readl(lgpio.reg_base + lgpio.chip_data.inten_offset + (offset / 32) * 4);
    u |= BIT(offset % 32);
    writel(u, lgpio.reg_base + lgpio.chip_data.inten_offset + (offset / 32) * 4);
    } else {
    writeb(1, lgpio.reg_base + lgpio.chip_data.inten_offset + offset);
    }
    return platform_get_irq(pdev, offset);
    }
#[no_mangle]
unsafe extern "C" fn loongson_gpio_irq_ack(data: *mut irq_data) {
    static void loongson_gpio_irq_ack(struct irq_data *data)
    {
    struct gpio_chip *chip = irq_data_get_irq_chip_data(data);
    struct loongson_gpio_chip *lgpio = to_loongson_gpio_chip(chip);
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(data);
    writeb(0x1, lgpio.reg_base + lgpio.chip_data.intclr_offset + hwirq);
    }
#[no_mangle]
unsafe extern "C" fn loongson_gpio_irq_mask(data: *mut irq_data) {
    static void loongson_gpio_irq_mask(struct irq_data *data)
    {
    struct gpio_chip *chip = irq_data_get_irq_chip_data(data);
    struct loongson_gpio_chip *lgpio = to_loongson_gpio_chip(chip);
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(data);
    writeb(0x0, lgpio.reg_base + lgpio.chip_data.inten_offset + hwirq);
    }
#[no_mangle]
unsafe extern "C" fn loongson_gpio_irq_unmask(data: *mut irq_data) {
    static void loongson_gpio_irq_unmask(struct irq_data *data)
    {
    struct gpio_chip *chip = irq_data_get_irq_chip_data(data);
    struct loongson_gpio_chip *lgpio = to_loongson_gpio_chip(chip);
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(data);
    writeb(0x1, lgpio.reg_base + lgpio.chip_data.inten_offset + hwirq);
    }
#[no_mangle]
unsafe extern "C" fn loongson_gpio_irq_set_type(data: *mut irq_data, type: c_uint) -> c_int {
    static int loongson_gpio_irq_set_type(struct irq_data *data, unsigned int type)
    {
    struct gpio_chip *chip = irq_data_get_irq_chip_data(data);
    struct loongson_gpio_chip *lgpio = to_loongson_gpio_chip(chip);
    let mut hwirq: irq_hw_number_t = irqd_to_hwirq(data);
    let mut pol: u8 = 0, edge = 0, dual = 0;
    if ((type & IRQ_TYPE_SENSE_MASK) == IRQ_TYPE_EDGE_BOTH) {
    edge = 1;
    dual = 1;
    irq_set_handler_locked(data, handle_edge_irq);
    } else {
    switch (type) {
    case IRQ_TYPE_LEVEL_HIGH:
    pol = 1;
    fallthrough;
    case IRQ_TYPE_LEVEL_LOW:
    irq_set_handler_locked(data, handle_level_irq);
    break;
    case IRQ_TYPE_EDGE_RISING:
    pol = 1;
    fallthrough;
    case IRQ_TYPE_EDGE_FALLING:
    edge = 1;
    irq_set_handler_locked(data, handle_edge_irq);
    break;
    default:
    return -EINVAL;
    }
    }
    writeb(pol, lgpio.reg_base + lgpio.chip_data.intpol_offset + hwirq);
    writeb(edge, lgpio.reg_base + lgpio.chip_data.intedge_offset + hwirq);
    writeb(dual, lgpio.reg_base + lgpio.chip_data.intdual_offset + hwirq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn loongson_gpio_ls2k0300_irq_handler(desc: *mut irq_desc) {
    static void loongson_gpio_ls2k0300_irq_handler(struct irq_desc *desc)
    {
    struct loongson_gpio_chip *lgpio = irq_desc_get_handler_data(desc);
    struct irq_chip *girqchip = irq_desc_get_chip(desc);
    int i;
    chained_irq_enter(girqchip, desc);
    for (i = 0; i < lgpio.chip.gc.ngpio; i++) {
//
// For the GPIO controller of LS2K0300, interrupts status bits
// may be wrongly set even if the corresponding interrupt is
// disabled. Thus interrupt enable bits are checked along with
// status bits to detect interrupts reliably.
//
    if (readb(lgpio.reg_base + lgpio.chip_data.intsts_offset + i) &&
    readb(lgpio.reg_base + lgpio.chip_data.inten_offset + i))
    generic_handle_domain_irq(lgpio.chip.gc.irq.domain, i);
    }
    chained_irq_exit(girqchip, desc);
    }
    static const struct irq_chip loongson_gpio_ls2k0300_irqchip = {
    .irq_ack	= loongson_gpio_irq_ack,
    .irq_mask	= loongson_gpio_irq_mask,
    .irq_unmask	= loongson_gpio_irq_unmask,
    .irq_set_type	= loongson_gpio_irq_set_type,
    .flags		= IRQCHIP_IMMUTABLE | IRQCHIP_SKIP_SET_WAKE,
    GPIOCHIP_IRQ_RESOURCE_HELPERS,
    };
    static int loongson_gpio_init_irqchip(struct platform_device *pdev,
    struct loongson_gpio_chip *lgpio)
    {
    const struct loongson_gpio_chip_data *data = lgpio.chip_data;
    struct gpio_chip *chip = &lgpio.chip.gc;
    int i;
    chip.irq.default_type = IRQ_TYPE_NONE;
    chip.irq.handler = handle_bad_irq;
    chip.irq.parent_handler = data.irq_handler;
    chip.irq.parent_handler_data = lgpio;
    gpio_irq_chip_set_chip(&chip.irq, data.girqchip);
    chip.irq.num_parents = data.intr_num;
    chip.irq.parents = devm_kcalloc(&pdev.dev, data.intr_num,
    sizeof(*chip.irq.parents), GFP_KERNEL);
    if (!chip.irq.parents)
    return -ENOMEM;
    for (i = 0; i < data.intr_num; i++) {
    int ret;
    ret = platform_get_irq(pdev, i);
    if (ret < 0)
    return dev_err_probe(&pdev.dev, ret,
    "failed to get IRQ %d\n", i);
    chip.irq.parents[i] = ret;
    }
    for (i = 0; i < data.intr_num; i++) {
    writeb(0x0, lgpio.reg_base + data.inten_offset + i);
    writeb(0x1, lgpio.reg_base + data.intclr_offset + i);
    }
    return 0;
    }
    static int loongson_gpio_init(struct platform_device *pdev, struct loongson_gpio_chip *lgpio,
    void __iomem *reg_base)
    {
    struct gpio_generic_chip_config config;
    int ret;
    lgpio.reg_base = reg_base;
    if (lgpio.chip_data.mode == BIT_CTRL_MODE) {
    config = (struct gpio_generic_chip_config) {
    .dev = &pdev.dev,
    .sz = 8,
    .dat = lgpio.reg_base + lgpio.chip_data.in_offset,
    .set = lgpio.reg_base + lgpio.chip_data.out_offset,
    .dirin = lgpio.reg_base + lgpio.chip_data.conf_offset,
    };
    ret = gpio_generic_chip_init(&lgpio.chip, &config);
    if (ret) {
    dev_err(&pdev.dev, "unable to init generic GPIO\n");
    return ret;
    }
    } else {
    lgpio.chip.gc.direction_input = loongson_gpio_direction_input;
    lgpio.chip.gc.get = loongson_gpio_get;
    lgpio.chip.gc.get_direction = loongson_gpio_get_direction;
    lgpio.chip.gc.direction_output = loongson_gpio_direction_output;
    lgpio.chip.gc.set = loongson_gpio_set;
    lgpio.chip.gc.parent = &pdev.dev;
    lgpio.chip.gc.base = -1;
    spin_lock_init(&lgpio.lock);
    }
    lgpio.chip.gc.label = lgpio.chip_data.label;
    lgpio.chip.gc.can_sleep = false;
    if (lgpio.chip_data.girqchip) {
    ret = loongson_gpio_init_irqchip(pdev, lgpio);
    if (ret)
    return dev_err_probe(&pdev.dev, ret, "failed to initialize irqchip\n");
    } else if (lgpio.chip_data.inten_offset) {
    lgpio.chip.gc.to_irq = loongson_gpio_to_irq;
    }
    return devm_gpiochip_add_data(&pdev.dev, &lgpio.chip.gc, lgpio);
    }
#[no_mangle]
unsafe extern "C" fn loongson_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int loongson_gpio_probe(struct platform_device *pdev)
    {
    void __iomem *reg_base;
    struct loongson_gpio_chip *lgpio;
    struct device *dev = &pdev.dev;
    struct reset_control *rst;
    lgpio = devm_kzalloc(dev, sizeof(*lgpio), GFP_KERNEL);
    if (!lgpio)
    return -ENOMEM;
    lgpio.chip_data = device_get_match_data(dev);
    reg_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(reg_base))
    return PTR_ERR(reg_base);
    rst = devm_reset_control_get_optional_exclusive_deasserted(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(rst))
    return dev_err_probe(&pdev.dev, PTR_ERR(rst), "failed to get reset control\n");
    return loongson_gpio_init(pdev, lgpio, reg_base);
    }
    static const struct loongson_gpio_chip_data loongson_gpio_ls2k_data = {
    .label = "ls2k_gpio",
    .mode = BIT_CTRL_MODE,
    .conf_offset = 0x0,
    .in_offset = 0x20,
    .out_offset = 0x10,
    .inten_offset = 0x30,
    };
    static const struct loongson_gpio_chip_data loongson_gpio_ls2k0300_data = {
    .label = "ls2k0300_gpio",
    .mode = BYTE_CTRL_MODE,
    .conf_offset = 0x800,
    .in_offset = 0xa00,
    .out_offset = 0x900,
    .inten_offset = 0xb00,
    .intpol_offset = 0xc00,
    .intedge_offset = 0xd00,
    .intclr_offset = 0xe00,
    .intsts_offset = 0xf00,
    .intdual_offset = 0xf80,
    .intr_num = 7,
    .irq_handler = loongson_gpio_ls2k0300_irq_handler,
    .girqchip = &loongson_gpio_ls2k0300_irqchip,
    };
    static const struct loongson_gpio_chip_data loongson_gpio_ls2k0500_data0 = {
    .label = "ls2k0500_gpio",
    .mode = BIT_CTRL_MODE,
    .conf_offset = 0x0,
    .in_offset = 0x8,
    .out_offset = 0x10,
    .inten_offset = 0xb0,
    };
    static const struct loongson_gpio_chip_data loongson_gpio_ls2k0500_data1 = {
    .label = "ls2k0500_gpio",
    .mode = BIT_CTRL_MODE,
    .conf_offset = 0x0,
    .in_offset = 0x8,
    .out_offset = 0x10,
    .inten_offset = 0x98,
    };
    static const struct loongson_gpio_chip_data loongson_gpio_ls2k2000_data0 = {
    .label = "ls2k2000_gpio",
    .mode = BIT_CTRL_MODE,
    .conf_offset = 0x0,
    .in_offset = 0xc,
    .out_offset = 0x8,
    .inten_offset = 0x14,
    };
    static const struct loongson_gpio_chip_data loongson_gpio_ls2k2000_data1 = {
    .label = "ls2k2000_gpio",
    .mode = BYTE_CTRL_MODE,
    .conf_offset = 0x800,
    .in_offset = 0xa00,
    .out_offset = 0x900,
    .inten_offset = 0xb00,
    };
    static const struct loongson_gpio_chip_data loongson_gpio_ls2k2000_data2 = {
    .label = "ls2k2000_gpio",
    .mode = BIT_CTRL_MODE,
    .conf_offset = 0x4,
    .in_offset = 0x8,
    .out_offset = 0x0,
    };
    static const struct loongson_gpio_chip_data loongson_gpio_ls3a5000_data = {
    .label = "ls3a5000_gpio",
    .mode = BIT_CTRL_MODE,
    .conf_offset = 0x0,
    .in_offset = 0xc,
    .out_offset = 0x8,
    .inten_offset = 0x14,
    };
    static const struct loongson_gpio_chip_data loongson_gpio_ls7a_data = {
    .label = "ls7a_gpio",
    .mode = BYTE_CTRL_MODE,
    .conf_offset = 0x800,
    .in_offset = 0xa00,
    .out_offset = 0x900,
    .inten_offset = 0xb00,
    };
// LS7A2000 chipset GPIO
    static const struct loongson_gpio_chip_data loongson_gpio_ls7a2000_data0 = {
    .label = "ls7a2000_gpio",
    .mode = BYTE_CTRL_MODE,
    .conf_offset = 0x800,
    .in_offset = 0xa00,
    .out_offset = 0x900,
    .inten_offset = 0xb00,
    };
// LS7A2000 ACPI GPIO
    static const struct loongson_gpio_chip_data loongson_gpio_ls7a2000_data1 = {
    .label = "ls7a2000_gpio",
    .mode = BIT_CTRL_MODE,
    .conf_offset = 0x4,
    .in_offset = 0x8,
    .out_offset = 0x0,
    };
// Loongson-3A6000 node GPIO
    static const struct loongson_gpio_chip_data loongson_gpio_ls3a6000_data = {
    .label = "ls3a6000_gpio",
    .mode = BIT_CTRL_MODE,
    .conf_offset = 0x0,
    .in_offset = 0xc,
    .out_offset = 0x8,
    .inten_offset = 0x14,
    };
    static const struct of_device_id loongson_gpio_of_match[] = {
    {
    .compatible = "loongson,ls2k-gpio",
    .data = &loongson_gpio_ls2k_data,
    },
    {
    .compatible = "loongson,ls2k0300-gpio",
    .data = &loongson_gpio_ls2k0300_data,
    },
    {
    .compatible = "loongson,ls2k0500-gpio0",
    .data = &loongson_gpio_ls2k0500_data0,
    },
    {
    .compatible = "loongson,ls2k0500-gpio1",
    .data = &loongson_gpio_ls2k0500_data1,
    },
    {
    .compatible = "loongson,ls2k2000-gpio0",
    .data = &loongson_gpio_ls2k2000_data0,
    },
    {
    .compatible = "loongson,ls2k2000-gpio1",
    .data = &loongson_gpio_ls2k2000_data1,
    },
    {
    .compatible = "loongson,ls2k2000-gpio2",
    .data = &loongson_gpio_ls2k2000_data2,
    },
    {
    .compatible = "loongson,ls3a5000-gpio",
    .data = &loongson_gpio_ls3a5000_data,
    },
    {
    .compatible = "loongson,ls7a-gpio",
    .data = &loongson_gpio_ls7a_data,
    },
    {
    .compatible = "loongson,ls7a2000-gpio1",
    .data = &loongson_gpio_ls7a2000_data0,
    },
    {
    .compatible = "loongson,ls7a2000-gpio2",
    .data = &loongson_gpio_ls7a2000_data1,
    },
    {
    .compatible = "loongson,ls3a6000-gpio",
    .data = &loongson_gpio_ls3a6000_data,
    },
    {}
    };
    MODULE_DEVICE_TABLE(of, loongson_gpio_of_match);
    static const struct acpi_device_id loongson_gpio_acpi_match[] = {
    {
    .id = "LOON0002",
    .driver_data = (kernel_ulong_t)&loongson_gpio_ls7a_data,
    },
    {
    .id = "LOON0007",
    .driver_data = (kernel_ulong_t)&loongson_gpio_ls3a5000_data,
    },
    {
    .id = "LOON000A",
    .driver_data = (kernel_ulong_t)&loongson_gpio_ls2k2000_data0,
    },
    {
    .id = "LOON000B",
    .driver_data = (kernel_ulong_t)&loongson_gpio_ls2k2000_data1,
    },
    {
    .id = "LOON000C",
    .driver_data = (kernel_ulong_t)&loongson_gpio_ls2k2000_data2,
    },
    {
    .id = "LOON000D",
    .driver_data = (kernel_ulong_t)&loongson_gpio_ls7a2000_data0,
    },
    {
    .id = "LOON000E",
    .driver_data = (kernel_ulong_t)&loongson_gpio_ls7a2000_data1,
    },
    {
    .id = "LOON000F",
    .driver_data = (kernel_ulong_t)&loongson_gpio_ls3a6000_data,
    },
    {}
    };
    MODULE_DEVICE_TABLE(acpi, loongson_gpio_acpi_match);
    static struct platform_driver loongson_gpio_driver = {
    .driver = {
    .name = "loongson-gpio",
    .of_match_table = loongson_gpio_of_match,
    .acpi_match_table = loongson_gpio_acpi_match,
    },
    .probe = loongson_gpio_probe,
    };
#[no_mangle]
unsafe extern "C" fn loongson_gpio_setup() -> int __init {
    static int __init loongson_gpio_setup(void)
    {
    return platform_driver_register(&loongson_gpio_driver);
    }
    postcore_initcall(loongson_gpio_setup);
    MODULE_DESCRIPTION("Loongson gpio driver");
    MODULE_LICENSE("GPL");
