//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-aspeed-sgpio.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright 2019 American Megatrends International LLC.
//
// Author: Karthikeyan Mani <karthikeyanm@amiindia.co.in>
//

pub const SGPIO_G7_IRQ_STS_BASE: c_uint = 0x40;

pub const SGPIO_G7_CTRL_REG_BASE: c_uint = 0x80;

pub const SELECT_FROM_CSR: c_int = 0;
pub const SELECT_FROM_PARALLEL_IN: c_int = 1;
pub const SELECT_FROM_SERIAL_IN: c_int = 2;
pub const ASPEED_SGPIO_G4_CFG_OFFSET: c_uint = 0x54;
pub const ASPEED_SGPIO_G7_CFG_OFFSET: c_uint = 0x0;

pub const ASPEED_SGPIO_PINS_SHIFT: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aspeed_sgpio_pdata {
    pub pin_mask: u32,
    pub llops: *const aspeed_sgpio_llops,
    pub cfg_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aspeed_sgpio {
    pub chip: gpio_chip,
    pub dev: *mut device,
    pub pclk: *mut clk,
    pub lock: raw_spinlock_t,
    pub base: *mut void __iomem,
    pub irq: c_int,
    pub pdata: *const aspeed_sgpio_pdata,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aspeed_sgpio_bank {
    pub val_regs: u16,
    pub rdata_reg: u16,
    pub irq_regs: u16,
    pub tolerance_regs: u16,
}

//
// Note: The "value" register returns the input value when the GPIO is
// configured as an input.
//
// The "rdata" register returns the output value when the GPIO is
// configured as an output.
//
    static const struct aspeed_sgpio_bank aspeed_sgpio_banks[] = {
    {
    .val_regs = 0x0000,
    .rdata_reg = 0x0070,
    .irq_regs = 0x0004,
    .tolerance_regs = 0x0018,
    },
    {
    .val_regs = 0x001C,
    .rdata_reg = 0x0074,
    .irq_regs = 0x0020,
    .tolerance_regs = 0x0034,
    },
    {
    .val_regs = 0x0038,
    .rdata_reg = 0x0078,
    .irq_regs = 0x003C,
    .tolerance_regs = 0x0050,
    },
    {
    .val_regs = 0x0090,
    .rdata_reg = 0x007C,
    .irq_regs = 0x0094,
    .tolerance_regs = 0x00A8,
    },
    };
    enum aspeed_sgpio_reg {
    reg_val,
    reg_rdata,
    reg_irq_enable,
    reg_irq_type0,
    reg_irq_type1,
    reg_irq_type2,
    reg_irq_status,
    reg_tolerance,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aspeed_sgpio_llops {
    void (*reg_bit_set)(struct aspeed_sgpio *gpio, unsigned int offset,
    pub val): enum aspeed_sgpio_reg reg, bool,
    bool (*reg_bit_get)(struct aspeed_sgpio *gpio, unsigned int offset,
    pub reg): enum aspeed_sgpio_reg,
    int (*reg_bank_get)(struct aspeed_sgpio *gpio, unsigned int offset,
    pub reg): enum aspeed_sgpio_reg,
}

pub const GPIO_VAL_VALUE: c_uint = 0x00;
pub const GPIO_IRQ_ENABLE: c_uint = 0x00;
pub const GPIO_IRQ_TYPE0: c_uint = 0x04;
pub const GPIO_IRQ_TYPE1: c_uint = 0x08;
pub const GPIO_IRQ_TYPE2: c_uint = 0x0C;
pub const GPIO_IRQ_STATUS: c_uint = 0x10;
    static void __iomem *aspeed_sgpio_g4_bank_reg(struct aspeed_sgpio *gpio,
    const struct aspeed_sgpio_bank *bank,
    const enum aspeed_sgpio_reg reg)
    {
    switch (reg) {
    case reg_val:
    return gpio.base + bank.val_regs + GPIO_VAL_VALUE;
    case reg_rdata:
    return gpio.base + bank.rdata_reg;
    case reg_irq_enable:
    return gpio.base + bank.irq_regs + GPIO_IRQ_ENABLE;
    case reg_irq_type0:
    return gpio.base + bank.irq_regs + GPIO_IRQ_TYPE0;
    case reg_irq_type1:
    return gpio.base + bank.irq_regs + GPIO_IRQ_TYPE1;
    case reg_irq_type2:
    return gpio.base + bank.irq_regs + GPIO_IRQ_TYPE2;
    case reg_irq_status:
    return gpio.base + bank.irq_regs + GPIO_IRQ_STATUS;
    case reg_tolerance:
    return gpio.base + bank.tolerance_regs;
    default:
// acturally if code runs to here, it's an error case
    BUG();
    }
    }
#[no_mangle]
unsafe extern "C" fn aspeed_sgpio_g7_reg_mask(reg: enum aspeed_sgpio_reg) -> u32 {
    static u32 aspeed_sgpio_g7_reg_mask(const enum aspeed_sgpio_reg reg)
    {
    switch (reg) {
    case reg_val:
    case reg_rdata:
    return SGPIO_G7_OUT_DATA;
    case reg_irq_enable:
    return SGPIO_G7_IRQ_EN;
    case reg_irq_type0:
    return SGPIO_G7_IRQ_TYPE0;
    case reg_irq_type1:
    return SGPIO_G7_IRQ_TYPE1;
    case reg_irq_type2:
    return SGPIO_G7_IRQ_TYPE2;
    case reg_irq_status:
    return SGPIO_G7_IRQ_STS;
    case reg_tolerance:
    return SGPIO_G7_RST_TOLERANCE;
    default:
    WARN_ON_ONCE(1);
    return 0;
    }
    }

    static const struct aspeed_sgpio_bank *to_bank(unsigned int offset)
    {
    unsigned int bank;
    bank = GPIO_BANK(offset);
    WARN_ON(bank >= ARRAY_SIZE(aspeed_sgpio_banks));
    return &aspeed_sgpio_banks[bank];
    }
    static int aspeed_sgpio_init_valid_mask(struct gpio_chip *gc,
    unsigned long *valid_mask, unsigned int ngpios)
    {
    bitmap_set(valid_mask, 0, ngpios);
    return 0;
    }
    static void aspeed_sgpio_irq_init_valid_mask(struct gpio_chip *gc,
    unsigned long *valid_mask, unsigned int ngpios)
    {
    unsigned int i;
// input GPIOs are even bits
    for (i = 0; i < ngpios; i++) {
    if (i % 2)
    clear_bit(i, valid_mask);
    }
    }
#[no_mangle]
unsafe extern "C" fn aspeed_sgpio_is_input(offset: c_uint) -> bool {
    static bool aspeed_sgpio_is_input(unsigned int offset)
    {
    return !(offset % 2);
    }
#[no_mangle]
unsafe extern "C" fn aspeed_sgpio_get(gc: *mut gpio_chip, offset: c_uint) -> c_int {
    static int aspeed_sgpio_get(struct gpio_chip *gc, unsigned int offset)
    {
    struct aspeed_sgpio *gpio = gpiochip_get_data(gc);
    enum aspeed_sgpio_reg reg;
    let mut rc: c_int = 0;
    guard(raw_spinlock_irqsave)(&gpio.lock);
    reg = aspeed_sgpio_is_input(offset) ? reg_val : reg_rdata;
    rc = gpio.pdata.llops.reg_bit_get(gpio, offset, reg);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn sgpio_set_value(gc: *mut gpio_chip, offset: c_uint, val: c_int) -> c_int {
    static int sgpio_set_value(struct gpio_chip *gc, unsigned int offset, int val)
    {
    struct aspeed_sgpio *gpio = gpiochip_get_data(gc);
    if (aspeed_sgpio_is_input(offset))
    return -EINVAL;
    gpio.pdata.llops.reg_bit_set(gpio, offset, reg_val, val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aspeed_sgpio_set(gc: *mut gpio_chip, offset: c_uint, val: c_int) -> c_int {
    static int aspeed_sgpio_set(struct gpio_chip *gc, unsigned int offset, int val)
    {
    struct aspeed_sgpio *gpio = gpiochip_get_data(gc);
    guard(raw_spinlock_irqsave)(&gpio.lock);
    return sgpio_set_value(gc, offset, val);
    }
#[no_mangle]
unsafe extern "C" fn aspeed_sgpio_dir_in(gc: *mut gpio_chip, offset: c_uint) -> c_int {
    static int aspeed_sgpio_dir_in(struct gpio_chip *gc, unsigned int offset)
    {
    return aspeed_sgpio_is_input(offset) ? 0 : -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn aspeed_sgpio_dir_out(gc: *mut gpio_chip, offset: c_uint, val: c_int) -> c_int {
    static int aspeed_sgpio_dir_out(struct gpio_chip *gc, unsigned int offset, int val)
    {
    struct aspeed_sgpio *gpio = gpiochip_get_data(gc);
    int rc;
// No special action is required for setting the direction; we'll
// error-out in sgpio_set_value if this isn't an output GPIO
    guard(raw_spinlock_irqsave)(&gpio.lock);
    rc = sgpio_set_value(gc, offset, val);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn aspeed_sgpio_get_direction(gc: *mut gpio_chip, offset: c_uint) -> c_int {
    static int aspeed_sgpio_get_direction(struct gpio_chip *gc, unsigned int offset)
    {
    return !!aspeed_sgpio_is_input(offset);
    }
#[no_mangle]
unsafe extern "C" fn aspeed_sgpio_irq_ack(d: *mut irq_data) {
    static void aspeed_sgpio_irq_ack(struct irq_data *d)
    {
    struct aspeed_sgpio *gpio = irq_data_get_irq_chip_data(d);
    let mut offset: c_int = irqd_to_hwirq(d);
    guard(raw_spinlock_irqsave)(&gpio.lock);
    gpio.pdata.llops.reg_bit_set(gpio, offset, reg_irq_status, 1);
    }
#[no_mangle]
unsafe extern "C" fn aspeed_sgpio_irq_set_mask(d: *mut irq_data, set: bool) {
    static void aspeed_sgpio_irq_set_mask(struct irq_data *d, bool set)
    {
    struct aspeed_sgpio *gpio = irq_data_get_irq_chip_data(d);
    let mut offset: c_int = irqd_to_hwirq(d);
// Unmasking the IRQ
    if (set)
    gpiochip_enable_irq(&gpio.chip, offset);
    scoped_guard(raw_spinlock_irqsave, &gpio.lock)
    {
    gpio.pdata.llops.reg_bit_set(gpio, offset, reg_irq_enable,
    set);
    }
// Masking the IRQ
    if (!set)
    gpiochip_disable_irq(&gpio.chip, offset);
    }
#[no_mangle]
unsafe extern "C" fn aspeed_sgpio_irq_mask(d: *mut irq_data) {
    static void aspeed_sgpio_irq_mask(struct irq_data *d)
    {
    aspeed_sgpio_irq_set_mask(d, false);
    }
#[no_mangle]
unsafe extern "C" fn aspeed_sgpio_irq_unmask(d: *mut irq_data) {
    static void aspeed_sgpio_irq_unmask(struct irq_data *d)
    {
    aspeed_sgpio_irq_set_mask(d, true);
    }
#[no_mangle]
unsafe extern "C" fn aspeed_sgpio_set_type(d: *mut irq_data, type: c_uint) -> c_int {
    static int aspeed_sgpio_set_type(struct irq_data *d, unsigned int type)
    {
    let mut type0: u32 = 0;
    let mut type1: u32 = 0;
    let mut type2: u32 = 0;
    irq_flow_handler_t handler;
    struct aspeed_sgpio *gpio = irq_data_get_irq_chip_data(d);
    let mut offset: c_int = irqd_to_hwirq(d);
    switch (type & IRQ_TYPE_SENSE_MASK) {
    case IRQ_TYPE_EDGE_BOTH:
    type2 = 1;
    fallthrough;
    case IRQ_TYPE_EDGE_RISING:
    type0 = 1;
    fallthrough;
    case IRQ_TYPE_EDGE_FALLING:
    handler = handle_edge_irq;
    break;
    case IRQ_TYPE_LEVEL_HIGH:
    type0 = 1;
    fallthrough;
    case IRQ_TYPE_LEVEL_LOW:
    type1 = 1;
    handler = handle_level_irq;
    break;
    default:
    return -EINVAL;
    }
    scoped_guard(raw_spinlock_irqsave, &gpio.lock) {
    gpio.pdata.llops.reg_bit_set(gpio, offset, reg_irq_type0, type0);
    gpio.pdata.llops.reg_bit_set(gpio, offset, reg_irq_type1, type1);
    gpio.pdata.llops.reg_bit_set(gpio, offset, reg_irq_type2, type2);
    }
    irq_set_handler_locked(d, handler);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn aspeed_sgpio_irq_handler(desc: *mut irq_desc) {
    static void aspeed_sgpio_irq_handler(struct irq_desc *desc)
    {
    struct gpio_chip *gc = irq_desc_get_handler_data(desc);
    struct irq_chip *ic = irq_desc_get_chip(desc);
    struct aspeed_sgpio *data = gpiochip_get_data(gc);
    unsigned int i, p, banks;
    unsigned long reg;
    chained_irq_enter(ic, desc);
    banks = DIV_ROUND_UP(gc.ngpio, 64);
    for (i = 0; i < banks; i++) {
    reg = data.pdata.llops.reg_bank_get(data, i << 6, reg_irq_status);
    for_each_set_bit(p, &reg, 32)
    generic_handle_domain_irq(gc.irq.domain, (i * 32 + p) * 2);
    }
    chained_irq_exit(ic, desc);
    }
#[no_mangle]
unsafe extern "C" fn aspeed_sgpio_irq_print_chip(d: *mut irq_data, p: *mut seq_file) {
    static void aspeed_sgpio_irq_print_chip(struct irq_data *d, struct seq_file *p)
    {
    struct aspeed_sgpio *gpio = irq_data_get_irq_chip_data(d);
    seq_puts(p, dev_name(gpio.dev));
    }
    static const struct irq_chip aspeed_sgpio_irq_chip = {
    .irq_ack = aspeed_sgpio_irq_ack,
    .irq_mask = aspeed_sgpio_irq_mask,
    .irq_unmask = aspeed_sgpio_irq_unmask,
    .irq_set_type = aspeed_sgpio_set_type,
    .irq_print_chip = aspeed_sgpio_irq_print_chip,
    .flags = IRQCHIP_IMMUTABLE,
    GPIOCHIP_IRQ_RESOURCE_HELPERS,
    };
    static int aspeed_sgpio_setup_irqs(struct aspeed_sgpio *gpio,
    struct platform_device *pdev)
    {
    int rc, i;
    struct gpio_irq_chip *irq;
    rc = platform_get_irq(pdev, 0);
    if (rc < 0)
    return rc;
    gpio.irq = rc;
// Disable IRQ and clear Interrupt status registers for all SGPIO Pins.
    for (i = 0; i < gpio.chip.ngpio; i += 2) {
// disable irq enable bits
    gpio.pdata.llops.reg_bit_set(gpio, i, reg_irq_enable, 0);
// clear status bits
    gpio.pdata.llops.reg_bit_set(gpio, i, reg_irq_status, 1);
    }
    irq = &gpio.chip.irq;
    gpio_irq_chip_set_chip(irq, &aspeed_sgpio_irq_chip);
    irq.init_valid_mask = aspeed_sgpio_irq_init_valid_mask;
    irq.handler = handle_bad_irq;
    irq.default_type = IRQ_TYPE_NONE;
    irq.parent_handler = aspeed_sgpio_irq_handler;
    irq.parent_handler_data = gpio;
    irq.parents = &gpio.irq;
    irq.num_parents = 1;
// Apply default IRQ settings
    for (i = 0; i < gpio.chip.ngpio; i += 2) {
// set falling or level-low irq
    gpio.pdata.llops.reg_bit_set(gpio, i, reg_irq_type0, 0);
// trigger type is edge
    gpio.pdata.llops.reg_bit_set(gpio, i, reg_irq_type1, 0);
// single edge trigger
    gpio.pdata.llops.reg_bit_set(gpio, i, reg_irq_type2, 0);
    }
    return 0;
    }
    static void aspeed_sgpio_g4_reg_bit_set(struct aspeed_sgpio *gpio, unsigned int offset,
    const enum aspeed_sgpio_reg reg, bool val)
    {
    const struct aspeed_sgpio_bank *bank = to_bank(offset);
    void __iomem *addr = aspeed_sgpio_g4_bank_reg(gpio, bank, reg);
    u32 temp;
    if (reg == reg_val) {
// Since this is an output, read the cached value from rdata, then update val.
    addr = aspeed_sgpio_g4_bank_reg(gpio, bank, reg_rdata);
    temp = ioread32(addr);
    if (val)
    temp |= GPIO_BIT(offset);
    else
    temp &= ~GPIO_BIT(offset);
    addr = aspeed_sgpio_g4_bank_reg(gpio, bank, reg_val);
    iowrite32(temp, addr);
    } else if (reg == reg_irq_status) {
    if (val)
    iowrite32(GPIO_BIT(offset), addr);
    } else {
// When setting other registers, we read from the register itself
    temp = ioread32(addr);
    if (val)
    temp |= GPIO_BIT(offset);
    else
    temp &= ~GPIO_BIT(offset);
    iowrite32(temp, addr);
    }
    }
    static bool aspeed_sgpio_g4_reg_bit_get(struct aspeed_sgpio *gpio, unsigned int offset,
    const enum aspeed_sgpio_reg reg)
    {
    const struct aspeed_sgpio_bank *bank = to_bank(offset);
    void __iomem *addr = aspeed_sgpio_g4_bank_reg(gpio, bank, reg);
    return !!(ioread32(addr) & GPIO_BIT(offset));
    }
    static int aspeed_sgpio_g4_reg_bank_get(struct aspeed_sgpio *gpio, unsigned int offset,
    const enum aspeed_sgpio_reg reg)
    {
    const struct aspeed_sgpio_bank *bank = to_bank(offset);
    void __iomem *addr = aspeed_sgpio_g4_bank_reg(gpio, bank, reg);
    if (reg == reg_irq_status)
    return ioread32(addr);
    else
    return -EOPNOTSUPP;
    }
    static const struct aspeed_sgpio_llops aspeed_sgpio_g4_llops = {
    .reg_bit_set = aspeed_sgpio_g4_reg_bit_set,
    .reg_bit_get = aspeed_sgpio_g4_reg_bit_get,
    .reg_bank_get = aspeed_sgpio_g4_reg_bank_get,
    };
    static const struct aspeed_sgpio_pdata ast2400_sgpio_pdata = {
    .pin_mask = GENMASK(9, 6),
    .llops = &aspeed_sgpio_g4_llops,
    .cfg_offset = ASPEED_SGPIO_G4_CFG_OFFSET,
    };
    static int aspeed_sgpio_reset_tolerance(struct gpio_chip *chip,
    unsigned int offset, bool enable)
    {
    struct aspeed_sgpio *gpio = gpiochip_get_data(chip);
    guard(raw_spinlock_irqsave)(&gpio.lock);
    gpio.pdata.llops.reg_bit_set(gpio, offset, reg_tolerance, enable);
    return 0;
    }
    static int aspeed_sgpio_set_config(struct gpio_chip *chip, unsigned int offset,
    unsigned long config)
    {
    let mut param: c_ulong = pinconf_to_config_param(config);
    let mut arg: u32 = pinconf_to_config_argument(config);
    if (param == PIN_CONFIG_PERSIST_STATE)
    return aspeed_sgpio_reset_tolerance(chip, offset, arg);
    return -ENOTSUPP;
    }
    static const struct aspeed_sgpio_pdata ast2600_sgpiom_pdata = {
    .pin_mask = GENMASK(10, 6),
    .llops = &aspeed_sgpio_g4_llops,
    .cfg_offset = ASPEED_SGPIO_G4_CFG_OFFSET,
    };
    static void aspeed_sgpio_g7_reg_bit_set(struct aspeed_sgpio *gpio, unsigned int offset,
    const enum aspeed_sgpio_reg reg, bool val)
    {
    let mut mask: u32 = aspeed_sgpio_g7_reg_mask(reg);
    void __iomem *addr = gpio.base + SGPIO_G7_CTRL_REG_OFFSET(offset >> 1);
    u32 write_val;
    if (mask) {
    write_val = (ioread32(addr) & ~(mask)) | field_prep(mask, val);
    iowrite32(write_val, addr);
    }
    }
    static bool aspeed_sgpio_g7_reg_bit_get(struct aspeed_sgpio *gpio, unsigned int offset,
    const enum aspeed_sgpio_reg reg)
    {
    let mut mask: u32 = aspeed_sgpio_g7_reg_mask(reg);
    void __iomem *addr;
    addr = gpio.base + SGPIO_G7_CTRL_REG_OFFSET(offset >> 1);
    if (reg == reg_val)
    mask = SGPIO_G7_IN_DATA;
    if (mask)
    return field_get(mask, ioread32(addr));
    else
    return 0;
    }
    static int aspeed_sgpio_g7_reg_bank_get(struct aspeed_sgpio *gpio, unsigned int offset,
    const enum aspeed_sgpio_reg reg)
    {
    void __iomem *addr;
    if (reg == reg_irq_status) {
    addr = gpio.base + SGPIO_G7_IRQ_STS_OFFSET(offset >> 6);
    return ioread32(addr);
    } else {
    return -EOPNOTSUPP;
    }
    }
    static const struct aspeed_sgpio_llops aspeed_sgpio_g7_llops = {
    .reg_bit_set = aspeed_sgpio_g7_reg_bit_set,
    .reg_bit_get = aspeed_sgpio_g7_reg_bit_get,
    .reg_bank_get = aspeed_sgpio_g7_reg_bank_get,
    };
    static const struct aspeed_sgpio_pdata ast2700_sgpiom_pdata = {
    .pin_mask = GENMASK(11, 6),
    .llops = &aspeed_sgpio_g7_llops,
    .cfg_offset = ASPEED_SGPIO_G7_CFG_OFFSET,
    };
    static const struct of_device_id aspeed_sgpio_of_table[] = {
    { .compatible = "aspeed,ast2400-sgpio", .data = &ast2400_sgpio_pdata, },
    { .compatible = "aspeed,ast2500-sgpio", .data = &ast2400_sgpio_pdata, },
    { .compatible = "aspeed,ast2600-sgpiom", .data = &ast2600_sgpiom_pdata, },
    { .compatible = "aspeed,ast2700-sgpiom", .data = &ast2700_sgpiom_pdata, },
    {}
    };
    MODULE_DEVICE_TABLE(of, aspeed_sgpio_of_table);
#[no_mangle]
unsafe extern "C" fn aspeed_sgpio_probe(pdev: *mut platform_device) -> c_int {
    static int aspeed_sgpio_probe(struct platform_device *pdev)
    {
    u32 nr_gpios, sgpio_freq, sgpio_clk_div, gpio_cnt_regval, pin_mask;
    struct aspeed_sgpio *gpio;
    unsigned long apb_freq;
    int rc;
    gpio = devm_kzalloc(&pdev.dev, sizeof(*gpio), GFP_KERNEL);
    if (!gpio)
    return -ENOMEM;
    gpio.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(gpio.base))
    return PTR_ERR(gpio.base);
    gpio.dev = &pdev.dev;
    gpio.pdata = device_get_match_data(&pdev.dev);
    if (!gpio.pdata)
    return -EINVAL;
    pin_mask = gpio.pdata.pin_mask;
    rc = device_property_read_u32(&pdev.dev, "ngpios", &nr_gpios);
    if (rc < 0) {
    dev_err(&pdev.dev, "Could not read ngpios property\n");
    return -EINVAL;
    } else if (nr_gpios % 8) {
    dev_err(&pdev.dev, "Number of GPIOs not multiple of 8: %d\n",
    nr_gpios);
    return -EINVAL;
    }
    rc = device_property_read_u32(&pdev.dev, "bus-frequency", &sgpio_freq);
    if (rc < 0) {
    dev_err(&pdev.dev, "Could not read bus-frequency property\n");
    return -EINVAL;
    }
    gpio.pclk = devm_clk_get(&pdev.dev, core::ptr::null_mut());
    if (IS_ERR(gpio.pclk)) {
    dev_err(&pdev.dev, "devm_clk_get failed\n");
    return PTR_ERR(gpio.pclk);
    }
    apb_freq = clk_get_rate(gpio.pclk);
//
// From the datasheet,
// SGPIO period = 1/PCLK * 2 * (GPIO254[31:16] + 1)
// period = 2 * (GPIO254[31:16] + 1) / PCLK
// frequency = 1 / (2 * (GPIO254[31:16] + 1) / PCLK)
// frequency = PCLK / (2 * (GPIO254[31:16] + 1))
// frequency * 2 * (GPIO254[31:16] + 1) = PCLK
// GPIO254[31:16] = PCLK / (frequency * 2) - 1
//
    if (sgpio_freq == 0)
    return -EINVAL;
    sgpio_clk_div = (apb_freq / (sgpio_freq * 2)) - 1;
    if (sgpio_clk_div > (1 << 16) - 1)
    return -EINVAL;
    gpio_cnt_regval = ((nr_gpios / 8) << ASPEED_SGPIO_PINS_SHIFT) & pin_mask;
    iowrite32(FIELD_PREP(ASPEED_SGPIO_CLK_DIV_MASK, sgpio_clk_div) | gpio_cnt_regval |
    ASPEED_SGPIO_ENABLE, gpio.base + gpio.pdata.cfg_offset);
    raw_spin_lock_init(&gpio.lock);
    gpio.chip.parent = &pdev.dev;
    gpio.chip.ngpio = nr_gpios * 2;
    gpio.chip.init_valid_mask = aspeed_sgpio_init_valid_mask;
    gpio.chip.direction_input = aspeed_sgpio_dir_in;
    gpio.chip.direction_output = aspeed_sgpio_dir_out;
    gpio.chip.get_direction = aspeed_sgpio_get_direction;
    gpio.chip.request = core::ptr::null_mut();
    gpio.chip.free = core::ptr::null_mut();
    gpio.chip.get = aspeed_sgpio_get;
    gpio.chip.set = aspeed_sgpio_set;
    gpio.chip.set_config = aspeed_sgpio_set_config;
    gpio.chip.label = dev_name(&pdev.dev);
    gpio.chip.base = -1;
    aspeed_sgpio_setup_irqs(gpio, pdev);
    rc = devm_gpiochip_add_data(&pdev.dev, &gpio.chip, gpio);
    if (rc < 0)
    return rc;
    return 0;
    }
    static struct platform_driver aspeed_sgpio_driver = {
    .probe = aspeed_sgpio_probe,
    .driver = {
    .name = KBUILD_MODNAME,
    .of_match_table = aspeed_sgpio_of_table,
    },
    };
    module_platform_driver(aspeed_sgpio_driver);
    MODULE_DESCRIPTION("Aspeed Serial GPIO Driver");
