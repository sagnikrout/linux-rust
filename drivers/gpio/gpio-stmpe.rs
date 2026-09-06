//! Automatically rewritten from C to Rust
//! Source: drivers/gpio/gpio-stmpe.c
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
// Copyright (C) ST-Ericsson SA 2010
//
// Author: Rabin Vincent <rabin.vincent@stericsson.com> for ST-Ericsson
//

//
// These registers are modified under the irq bus lock and cached to avoid
// unnecessary writes in bus_sync_unlock.
//
    enum { REG_RE, REG_FE, REG_IE };
    enum { LSB, CSB, MSB };
pub const CACHE_NR_REGS: c_int = 3;
// No variant has more than 24 GPIOs

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmpe_gpio {
    pub chip: gpio_chip,
    pub stmpe: *mut stmpe,
    pub irq_lock: mutex,
    pub norequest_mask: u32,
// Caches of interrupt control registers for bus_lock
    pub regs: [u8; CACHE_NR_REGS][CACHE_NR_BANKS],
    pub oldregs: [u8; CACHE_NR_REGS][CACHE_NR_BANKS],
}

#[no_mangle]
unsafe extern "C" fn stmpe_gpio_get(chip: *mut gpio_chip, offset: unsigned) -> c_int {
    static int stmpe_gpio_get(struct gpio_chip *chip, unsigned offset)
    {
    struct stmpe_gpio *stmpe_gpio = gpiochip_get_data(chip);
    struct stmpe *stmpe = stmpe_gpio.stmpe;
    let mut reg: u8 = stmpe.regs[STMPE_IDX_GPMR_LSB + (offset / 8)];
    let mut mask: u8 = BIT(offset % 8);
    int ret;
    ret = stmpe_reg_read(stmpe, reg);
    if (ret < 0)
    return ret;
    return !!(ret & mask);
    }
#[no_mangle]
unsafe extern "C" fn stmpe_gpio_set(chip: *mut gpio_chip, offset: c_uint, val: c_int) -> c_int {
    static int stmpe_gpio_set(struct gpio_chip *chip, unsigned int offset, int val)
    {
    struct stmpe_gpio *stmpe_gpio = gpiochip_get_data(chip);
    struct stmpe *stmpe = stmpe_gpio.stmpe;
    let mut which: c_int = val ? STMPE_IDX_GPSR_LSB : STMPE_IDX_GPCR_LSB;
    let mut reg: u8 = stmpe.regs[which + (offset / 8)];
    let mut mask: u8 = BIT(offset % 8);
//
// Some variants have single register for gpio set/clear functionality.
// For them we need to write 0 to clear and 1 to set.
//
    if (stmpe.regs[STMPE_IDX_GPSR_LSB] == stmpe.regs[STMPE_IDX_GPCR_LSB])
    return stmpe_set_bits(stmpe, reg, mask, val ? mask : 0);
    return stmpe_reg_write(stmpe, reg, mask);
    }
    static int stmpe_gpio_get_direction(struct gpio_chip *chip,
    unsigned offset)
    {
    struct stmpe_gpio *stmpe_gpio = gpiochip_get_data(chip);
    struct stmpe *stmpe = stmpe_gpio.stmpe;
    let mut reg: u8 = stmpe.regs[STMPE_IDX_GPDR_LSB] - (offset / 8);
    let mut mask: u8 = BIT(offset % 8);
    int ret;
    ret = stmpe_reg_read(stmpe, reg);
    if (ret < 0)
    return ret;
    if (ret & mask)
    return GPIO_LINE_DIRECTION_OUT;
    return GPIO_LINE_DIRECTION_IN;
    }
    static int stmpe_gpio_direction_output(struct gpio_chip *chip,
    unsigned offset, int val)
    {
    struct stmpe_gpio *stmpe_gpio = gpiochip_get_data(chip);
    struct stmpe *stmpe = stmpe_gpio.stmpe;
    let mut reg: u8 = stmpe.regs[STMPE_IDX_GPDR_LSB + (offset / 8)];
    let mut mask: u8 = BIT(offset % 8);
    int ret;
    ret = stmpe_gpio_set(chip, offset, val);
    if (ret)
    return ret;
    return stmpe_set_bits(stmpe, reg, mask, mask);
    }
    static int stmpe_gpio_direction_input(struct gpio_chip *chip,
    unsigned offset)
    {
    struct stmpe_gpio *stmpe_gpio = gpiochip_get_data(chip);
    struct stmpe *stmpe = stmpe_gpio.stmpe;
    let mut reg: u8 = stmpe.regs[STMPE_IDX_GPDR_LSB + (offset / 8)];
    let mut mask: u8 = BIT(offset % 8);
    return stmpe_set_bits(stmpe, reg, mask, 0);
    }
#[no_mangle]
unsafe extern "C" fn stmpe_gpio_request(chip: *mut gpio_chip, offset: unsigned) -> c_int {
    static int stmpe_gpio_request(struct gpio_chip *chip, unsigned offset)
    {
    struct stmpe_gpio *stmpe_gpio = gpiochip_get_data(chip);
    struct stmpe *stmpe = stmpe_gpio.stmpe;
    if (stmpe_gpio.norequest_mask & BIT(offset))
    return -EINVAL;
    return stmpe_set_altfunc(stmpe, BIT(offset), STMPE_BLOCK_GPIO);
    }
    static const struct gpio_chip template_chip = {
    .label			= "stmpe",
    .owner			= THIS_MODULE,
    .get_direction		= stmpe_gpio_get_direction,
    .direction_input	= stmpe_gpio_direction_input,
    .get			= stmpe_gpio_get,
    .direction_output	= stmpe_gpio_direction_output,
    .set			= stmpe_gpio_set,
    .request		= stmpe_gpio_request,
    .can_sleep		= true,
    };
#[no_mangle]
unsafe extern "C" fn stmpe_gpio_irq_set_type(d: *mut irq_data, type: c_uint) -> c_int {
    static int stmpe_gpio_irq_set_type(struct irq_data *d, unsigned int type)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct stmpe_gpio *stmpe_gpio = gpiochip_get_data(gc);
    let mut offset: c_int = d.hwirq;
    let mut regoffset: c_int = offset / 8;
    let mut mask: c_int = BIT(offset % 8);
    if (type & IRQ_TYPE_LEVEL_MASK)
    return -EINVAL;
// STMPE801 and STMPE 1600 don't have RE and FE registers
    if (stmpe_gpio.stmpe.partnum == STMPE801 ||
    stmpe_gpio.stmpe.partnum == STMPE1600)
    return 0;
    if (type & IRQ_TYPE_EDGE_RISING)
    stmpe_gpio.regs[REG_RE][regoffset] |= mask;
    else
    stmpe_gpio.regs[REG_RE][regoffset] &= ~mask;
    if (type & IRQ_TYPE_EDGE_FALLING)
    stmpe_gpio.regs[REG_FE][regoffset] |= mask;
    else
    stmpe_gpio.regs[REG_FE][regoffset] &= ~mask;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stmpe_gpio_irq_lock(d: *mut irq_data) {
    static void stmpe_gpio_irq_lock(struct irq_data *d)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct stmpe_gpio *stmpe_gpio = gpiochip_get_data(gc);
    mutex_lock(&stmpe_gpio.irq_lock);
    }
#[no_mangle]
unsafe extern "C" fn stmpe_gpio_irq_sync_unlock(d: *mut irq_data) {
    static void stmpe_gpio_irq_sync_unlock(struct irq_data *d)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct stmpe_gpio *stmpe_gpio = gpiochip_get_data(gc);
    struct stmpe *stmpe = stmpe_gpio.stmpe;
    let mut num_banks: c_int = DIV_ROUND_UP(stmpe.num_gpios, 8);
    static const u8 regmap[CACHE_NR_REGS][CACHE_NR_BANKS] = {
    [REG_RE][LSB] = STMPE_IDX_GPRER_LSB,
    [REG_RE][CSB] = STMPE_IDX_GPRER_CSB,
    [REG_RE][MSB] = STMPE_IDX_GPRER_MSB,
    [REG_FE][LSB] = STMPE_IDX_GPFER_LSB,
    [REG_FE][CSB] = STMPE_IDX_GPFER_CSB,
    [REG_FE][MSB] = STMPE_IDX_GPFER_MSB,
    [REG_IE][LSB] = STMPE_IDX_IEGPIOR_LSB,
    [REG_IE][CSB] = STMPE_IDX_IEGPIOR_CSB,
    [REG_IE][MSB] = STMPE_IDX_IEGPIOR_MSB,
    };
    int ret, i, j;
//
// STMPE1600: to be able to get IRQ from pins,
// a read must be done on GPMR register, or a write in
// GPSR or GPCR registers
//
    if (stmpe.partnum == STMPE1600) {
    ret = stmpe_reg_read(stmpe, stmpe.regs[STMPE_IDX_GPMR_LSB]);
    if (ret < 0) {
    dev_err(stmpe.dev, "Failed to read GPMR_LSB: %d\n", ret);
    goto err;
    }
    ret = stmpe_reg_read(stmpe, stmpe.regs[STMPE_IDX_GPMR_CSB]);
    if (ret < 0) {
    dev_err(stmpe.dev, "Failed to read GPMR_CSB: %d\n", ret);
    goto err;
    }
    }
    for (i = 0; i < CACHE_NR_REGS; i++) {
// STMPE801 and STMPE1600 don't have RE and FE registers
    if ((stmpe.partnum == STMPE801 ||
    stmpe.partnum == STMPE1600) &&
    (i != REG_IE))
    continue;
    for (j = 0; j < num_banks; j++) {
    let mut old: u8 = stmpe_gpio.oldregs[i][j];
    let mut new: u8 = stmpe_gpio.regs[i][j];
    if (new == old)
    continue;
    stmpe_gpio.oldregs[i][j] = new;
    stmpe_reg_write(stmpe, stmpe.regs[regmap[i][j]], new);
    }
    }
    err:
    mutex_unlock(&stmpe_gpio.irq_lock);
    }
#[no_mangle]
unsafe extern "C" fn stmpe_gpio_irq_mask(d: *mut irq_data) {
    static void stmpe_gpio_irq_mask(struct irq_data *d)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct stmpe_gpio *stmpe_gpio = gpiochip_get_data(gc);
    let mut offset: c_int = d.hwirq;
    let mut regoffset: c_int = offset / 8;
    let mut mask: c_int = BIT(offset % 8);
    stmpe_gpio.regs[REG_IE][regoffset] &= ~mask;
    gpiochip_disable_irq(gc, offset);
    }
#[no_mangle]
unsafe extern "C" fn stmpe_gpio_irq_unmask(d: *mut irq_data) {
    static void stmpe_gpio_irq_unmask(struct irq_data *d)
    {
    struct gpio_chip *gc = irq_data_get_irq_chip_data(d);
    struct stmpe_gpio *stmpe_gpio = gpiochip_get_data(gc);
    let mut offset: c_int = d.hwirq;
    let mut regoffset: c_int = offset / 8;
    let mut mask: c_int = BIT(offset % 8);
    gpiochip_enable_irq(gc, offset);
    stmpe_gpio.regs[REG_IE][regoffset] |= mask;
    }
    static void stmpe_dbg_show_one(struct seq_file *s, struct gpio_chip *gc,
    unsigned int offset)
    {
    struct stmpe_gpio *stmpe_gpio = gpiochip_get_data(gc);
    struct stmpe *stmpe = stmpe_gpio.stmpe;
    let mut val: bool = !!stmpe_gpio_get(gc, offset);
    let mut bank: u8 = offset / 8;
    let mut dir_reg: u8 = stmpe.regs[STMPE_IDX_GPDR_LSB + bank];
    let mut mask: u8 = BIT(offset % 8);
    int ret;
    u8 dir;
    char *label __free(kfree) = gpiochip_dup_line_label(gc, offset);
    if (IS_ERR(label))
    return;
    ret = stmpe_reg_read(stmpe, dir_reg);
    if (ret < 0)
    return;
    dir = !!(ret & mask);
    if (dir) {
    seq_printf(s, " gpio-%-3d (%-20.20s) out %s",
    offset, label ?: "(none)", str_hi_lo(val));
    } else {
    u8 edge_det_reg;
    u8 rise_reg;
    u8 fall_reg;
    u8 irqen_reg;
    static const char * const edge_det_values[] = {
    "edge-inactive",
    "edge-asserted",
    "not-supported"
    };
    static const char * const rise_values[] = {
    "no-rising-edge-detection",
    "rising-edge-detection",
    "not-supported"
    };
    static const char * const fall_values[] = {
    "no-falling-edge-detection",
    "falling-edge-detection",
    "not-supported"
    };
pub const NOT_SUPPORTED_IDX: c_int = 2;
    let mut edge_det: u8 = NOT_SUPPORTED_IDX;
    let mut rise: u8 = NOT_SUPPORTED_IDX;
    let mut fall: u8 = NOT_SUPPORTED_IDX;
    bool irqen;
    switch (stmpe.partnum) {
    case STMPE610:
    case STMPE811:
    case STMPE1601:
    case STMPE2401:
    case STMPE2403:
    edge_det_reg = stmpe.regs[STMPE_IDX_GPEDR_LSB + bank];
    ret = stmpe_reg_read(stmpe, edge_det_reg);
    if (ret < 0)
    return;
    edge_det = !!(ret & mask);
    fallthrough;
    case STMPE1801:
    rise_reg = stmpe.regs[STMPE_IDX_GPRER_LSB + bank];
    fall_reg = stmpe.regs[STMPE_IDX_GPFER_LSB + bank];
    ret = stmpe_reg_read(stmpe, rise_reg);
    if (ret < 0)
    return;
    rise = !!(ret & mask);
    ret = stmpe_reg_read(stmpe, fall_reg);
    if (ret < 0)
    return;
    fall = !!(ret & mask);
    fallthrough;
    case STMPE801:
    case STMPE1600:
    irqen_reg = stmpe.regs[STMPE_IDX_IEGPIOR_LSB + bank];
    break;
    default:
    return;
    }
    ret = stmpe_reg_read(stmpe, irqen_reg);
    if (ret < 0)
    return;
    irqen = !!(ret & mask);
    seq_printf(s, " gpio-%-3d (%-20.20s) in  %s %13s %13s %25s %25s",
    offset, label ?: "(none)",
    str_hi_lo(val),
    edge_det_values[edge_det],
    irqen ? "IRQ-enabled" : "IRQ-disabled",
    rise_values[rise],
    fall_values[fall]);
    }
    }
#[no_mangle]
unsafe extern "C" fn stmpe_dbg_show(s: *mut seq_file, gc: *mut gpio_chip) {
    static void stmpe_dbg_show(struct seq_file *s, struct gpio_chip *gc)
    {
    unsigned i;
    for (i = 0; i < gc.ngpio; i++) {
    stmpe_dbg_show_one(s, gc, i);
    seq_putc(s, '\n');
    }
    }
    static const struct irq_chip stmpe_gpio_irq_chip = {
    .name			= "stmpe-gpio",
    .irq_bus_lock		= stmpe_gpio_irq_lock,
    .irq_bus_sync_unlock	= stmpe_gpio_irq_sync_unlock,
    .irq_mask		= stmpe_gpio_irq_mask,
    .irq_unmask		= stmpe_gpio_irq_unmask,
    .irq_set_type		= stmpe_gpio_irq_set_type,
    .flags			= IRQCHIP_IMMUTABLE,
    GPIOCHIP_IRQ_RESOURCE_HELPERS,
    };
pub const MAX_GPIOS: c_int = 24;
#[no_mangle]
unsafe extern "C" fn stmpe_gpio_irq(irq: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t stmpe_gpio_irq(int irq, void *dev)
    {
    struct stmpe_gpio *stmpe_gpio = dev;
    struct stmpe *stmpe = stmpe_gpio.stmpe;
    u8 statmsbreg;
    let mut num_banks: c_int = DIV_ROUND_UP(stmpe.num_gpios, 8);
    u8 status[DIV_ROUND_UP(MAX_GPIOS, 8)];
    int ret;
    int i;
//
// the stmpe_block_read() call below, imposes to set statmsbreg
// with the register located at the lowest address. As STMPE1600
// variant is the only one which respect registers address's order
// (LSB regs located at lowest address than MSB ones) whereas all
// the others have a registers layout with MSB located before the
// LSB regs.
//
    if (stmpe.partnum == STMPE1600)
    statmsbreg = stmpe.regs[STMPE_IDX_ISGPIOR_LSB];
    else
    statmsbreg = stmpe.regs[STMPE_IDX_ISGPIOR_MSB];
    ret = stmpe_block_read(stmpe, statmsbreg, num_banks, status);
    if (ret < 0)
    return IRQ_NONE;
    for (i = 0; i < num_banks; i++) {
    int bank = (stmpe_gpio.stmpe.partnum == STMPE1600) ? i :
    num_banks - i - 1;
    let mut enabled: c_uint = stmpe_gpio.regs[REG_IE][bank];
    let mut stat: c_uint = status[i];
    stat &= enabled;
    if (!stat)
    continue;
    while (stat) {
    let mut bit: c_int = __ffs(stat);
    let mut line: c_int = bank * 8 + bit;
    int child_irq = irq_find_mapping(stmpe_gpio.chip.irq.domain,
    line);
    handle_nested_irq(child_irq);
    stat &= ~BIT(bit);
    }
//
// interrupt status register write has no effect on
// 801/1801/1600, bits are cleared when read.
// Edge detect register is not present on 801/1600/1801
//
    if (stmpe.partnum != STMPE801 && stmpe.partnum != STMPE1600 &&
    stmpe.partnum != STMPE1801) {
    stmpe_reg_write(stmpe, statmsbreg + i, status[i]);
    stmpe_reg_write(stmpe,
    stmpe.regs[STMPE_IDX_GPEDR_MSB] + i,
    status[i]);
    }
    }
    return IRQ_HANDLED;
    }
    static void stmpe_init_irq_valid_mask(struct gpio_chip *gc,
    unsigned long *valid_mask,
    unsigned int ngpios)
    {
    struct stmpe_gpio *stmpe_gpio = gpiochip_get_data(gc);
    int i;
    if (!stmpe_gpio.norequest_mask)
    return;
// Forbid unused lines to be mapped as IRQs
    for (i = 0; i < sizeof(u32); i++) {
    if (stmpe_gpio.norequest_mask & BIT(i))
    clear_bit(i, valid_mask);
    }
    }
#[no_mangle]
unsafe extern "C" fn stmpe_gpio_disable(stmpe: *mut c_void) {
    static void stmpe_gpio_disable(void *stmpe)
    {
    stmpe_disable(stmpe, STMPE_BLOCK_GPIO);
    }
#[no_mangle]
unsafe extern "C" fn stmpe_gpio_probe(pdev: *mut platform_device) -> c_int {
    static int stmpe_gpio_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct stmpe *stmpe = dev_get_drvdata(dev.parent);
    struct stmpe_gpio *stmpe_gpio;
    int ret, irq;
    if (stmpe.num_gpios > MAX_GPIOS) {
    dev_err(dev, "Need to increase maximum GPIO number\n");
    return -EINVAL;
    }
    stmpe_gpio = devm_kzalloc(dev, sizeof(*stmpe_gpio), GFP_KERNEL);
    if (!stmpe_gpio)
    return -ENOMEM;
    mutex_init(&stmpe_gpio.irq_lock);
    stmpe_gpio.stmpe = stmpe;
    stmpe_gpio.chip = template_chip;
    stmpe_gpio.chip.ngpio = stmpe.num_gpios;
    stmpe_gpio.chip.parent = dev;
    stmpe_gpio.chip.base = -1;
    if (IS_ENABLED(CONFIG_DEBUG_FS))
    stmpe_gpio.chip.dbg_show = stmpe_dbg_show;
    device_property_read_u32(dev, "st,norequest-mask", &stmpe_gpio.norequest_mask);
    ret = stmpe_enable(stmpe, STMPE_BLOCK_GPIO);
    if (ret)
    return ret;
    ret = devm_add_action_or_reset(dev, stmpe_gpio_disable, stmpe);
    if (ret)
    return ret;
    irq = platform_get_irq(pdev, 0);
    if (irq > 0) {
    struct gpio_irq_chip *girq;
    ret = devm_request_threaded_irq(dev, irq, core::ptr::null_mut(), stmpe_gpio_irq,
    IRQF_ONESHOT, "stmpe-gpio", stmpe_gpio);
    if (ret)
    return dev_err_probe(dev, ret, "unable to register IRQ handler\n");
    girq = &stmpe_gpio.chip.irq;
    gpio_irq_chip_set_chip(girq, &stmpe_gpio_irq_chip);
// This will let us handle the parent IRQ in the driver
    girq.parent_handler = core::ptr::null_mut();
    girq.num_parents = 0;
    girq.parents = core::ptr::null_mut();
    girq.default_type = IRQ_TYPE_NONE;
    girq.handler = handle_simple_irq;
    girq.threaded = true;
    girq.init_valid_mask = stmpe_init_irq_valid_mask;
    }
    return devm_gpiochip_add_data(dev, &stmpe_gpio.chip, stmpe_gpio);
    }
    static const struct of_device_id stmpe_gpio_of_matches[] = {
    { .compatible = "st,stmpe-gpio", },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, stmpe_gpio_of_matches);
    static struct platform_driver stmpe_gpio_driver = {
    .driver = {
    .name = "stmpe-gpio",
    .of_match_table = stmpe_gpio_of_matches,
    },
    .probe		= stmpe_gpio_probe,
    };
#[no_mangle]
unsafe extern "C" fn stmpe_gpio_init() -> int __init {
    static int __init stmpe_gpio_init(void)
    {
    return platform_driver_register(&stmpe_gpio_driver);
    }
    subsys_initcall(stmpe_gpio_init);
#[no_mangle]
unsafe extern "C" fn stmpe_gpio_exit() -> void __exit {
    static void __exit stmpe_gpio_exit(void)
    {
    platform_driver_unregister(&stmpe_gpio_driver);
    }
    module_exit(stmpe_gpio_exit);
    MODULE_DESCRIPTION("STMPE expander GPIO");
    MODULE_AUTHOR("Rabin Vincent <rabin.vincent@stericsson.com>");
    MODULE_LICENSE("GPL");
