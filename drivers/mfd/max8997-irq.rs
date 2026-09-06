//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/max8997-irq.c
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
// max8997-irq.c - Interrupt controller support for MAX8997
//
// Copyright (C) 2011 Samsung Electronics Co.Ltd
// MyungJoo Ham <myungjoo.ham@samsung.com>
//
// This driver is based on max8998-irq.c

    static const u8 max8997_mask_reg[] = {
    [PMIC_INT1] = MAX8997_REG_INT1MSK,
    [PMIC_INT2] = MAX8997_REG_INT2MSK,
    [PMIC_INT3] = MAX8997_REG_INT3MSK,
    [PMIC_INT4] = MAX8997_REG_INT4MSK,
    [FUEL_GAUGE] = MAX8997_REG_INVALID,
    [MUIC_INT1] = MAX8997_MUIC_REG_INTMASK1,
    [MUIC_INT2] = MAX8997_MUIC_REG_INTMASK2,
    [MUIC_INT3] = MAX8997_MUIC_REG_INTMASK3,
    [GPIO_LOW] = MAX8997_REG_INVALID,
    [GPIO_HI] = MAX8997_REG_INVALID,
    [FLASH_STATUS] = MAX8997_REG_INVALID,
    };
    static struct i2c_client *get_i2c(struct max8997_dev *max8997,
    enum max8997_irq_source src)
    {
    switch (src) {
    case PMIC_INT1 ... PMIC_INT4:
    return max8997.i2c;
    case FUEL_GAUGE:
    return core::ptr::null_mut();
    case MUIC_INT1 ... MUIC_INT3:
    return max8997.muic;
    case GPIO_LOW ... GPIO_HI:
    return max8997.i2c;
    case FLASH_STATUS:
    return max8997.i2c;
    default:
    return ERR_PTR(-EINVAL);
    }
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct max8997_irq_data {
    pub mask: c_int,
    pub group: enum max8997_irq_source,
}

    [(idx)] = { .group = (_group), .mask = (_mask) }
    static const struct max8997_irq_data max8997_irqs[] = {
    DECLARE_IRQ(MAX8997_PMICIRQ_PWRONR,	PMIC_INT1, 1 << 0),
    DECLARE_IRQ(MAX8997_PMICIRQ_PWRONF,	PMIC_INT1, 1 << 1),
    DECLARE_IRQ(MAX8997_PMICIRQ_PWRON1SEC,	PMIC_INT1, 1 << 3),
    DECLARE_IRQ(MAX8997_PMICIRQ_JIGONR,	PMIC_INT1, 1 << 4),
    DECLARE_IRQ(MAX8997_PMICIRQ_JIGONF,	PMIC_INT1, 1 << 5),
    DECLARE_IRQ(MAX8997_PMICIRQ_LOWBAT2,	PMIC_INT1, 1 << 6),
    DECLARE_IRQ(MAX8997_PMICIRQ_LOWBAT1,	PMIC_INT1, 1 << 7),
    DECLARE_IRQ(MAX8997_PMICIRQ_JIGR,	PMIC_INT2, 1 << 0),
    DECLARE_IRQ(MAX8997_PMICIRQ_JIGF,	PMIC_INT2, 1 << 1),
    DECLARE_IRQ(MAX8997_PMICIRQ_MR,		PMIC_INT2, 1 << 2),
    DECLARE_IRQ(MAX8997_PMICIRQ_DVS1OK,	PMIC_INT2, 1 << 3),
    DECLARE_IRQ(MAX8997_PMICIRQ_DVS2OK,	PMIC_INT2, 1 << 4),
    DECLARE_IRQ(MAX8997_PMICIRQ_DVS3OK,	PMIC_INT2, 1 << 5),
    DECLARE_IRQ(MAX8997_PMICIRQ_DVS4OK,	PMIC_INT2, 1 << 6),
    DECLARE_IRQ(MAX8997_PMICIRQ_CHGINS,	PMIC_INT3, 1 << 0),
    DECLARE_IRQ(MAX8997_PMICIRQ_CHGRM,	PMIC_INT3, 1 << 1),
    DECLARE_IRQ(MAX8997_PMICIRQ_DCINOVP,	PMIC_INT3, 1 << 2),
    DECLARE_IRQ(MAX8997_PMICIRQ_TOPOFFR,	PMIC_INT3, 1 << 3),
    DECLARE_IRQ(MAX8997_PMICIRQ_CHGRSTF,	PMIC_INT3, 1 << 5),
    DECLARE_IRQ(MAX8997_PMICIRQ_MBCHGTMEXPD,	PMIC_INT3, 1 << 7),
    DECLARE_IRQ(MAX8997_PMICIRQ_RTC60S,	PMIC_INT4, 1 << 0),
    DECLARE_IRQ(MAX8997_PMICIRQ_RTCA1,	PMIC_INT4, 1 << 1),
    DECLARE_IRQ(MAX8997_PMICIRQ_RTCA2,	PMIC_INT4, 1 << 2),
    DECLARE_IRQ(MAX8997_PMICIRQ_SMPL_INT,	PMIC_INT4, 1 << 3),
    DECLARE_IRQ(MAX8997_PMICIRQ_RTC1S,	PMIC_INT4, 1 << 4),
    DECLARE_IRQ(MAX8997_PMICIRQ_WTSR,	PMIC_INT4, 1 << 5),
    DECLARE_IRQ(MAX8997_MUICIRQ_ADCError,	MUIC_INT1, 1 << 2),
    DECLARE_IRQ(MAX8997_MUICIRQ_ADCLow,	MUIC_INT1, 1 << 1),
    DECLARE_IRQ(MAX8997_MUICIRQ_ADC,	MUIC_INT1, 1 << 0),
    DECLARE_IRQ(MAX8997_MUICIRQ_VBVolt,	MUIC_INT2, 1 << 4),
    DECLARE_IRQ(MAX8997_MUICIRQ_DBChg,	MUIC_INT2, 1 << 3),
    DECLARE_IRQ(MAX8997_MUICIRQ_DCDTmr,	MUIC_INT2, 1 << 2),
    DECLARE_IRQ(MAX8997_MUICIRQ_ChgDetRun,	MUIC_INT2, 1 << 1),
    DECLARE_IRQ(MAX8997_MUICIRQ_ChgTyp,	MUIC_INT2, 1 << 0),
    DECLARE_IRQ(MAX8997_MUICIRQ_OVP,	MUIC_INT3, 1 << 2),
    };
#[no_mangle]
unsafe extern "C" fn max8997_irq_lock(data: *mut irq_data) {
    static void max8997_irq_lock(struct irq_data *data)
    {
    struct max8997_dev *max8997 = irq_data_get_irq_chip_data(data);
    mutex_lock(&max8997.irqlock);
    }
#[no_mangle]
unsafe extern "C" fn max8997_irq_sync_unlock(data: *mut irq_data) {
    static void max8997_irq_sync_unlock(struct irq_data *data)
    {
    struct max8997_dev *max8997 = irq_data_get_irq_chip_data(data);
    int i;
    for (i = 0; i < MAX8997_IRQ_GROUP_NR; i++) {
    let mut mask_reg: u8 = max8997_mask_reg[i];
    struct i2c_client *i2c = get_i2c(max8997, i);
    if (mask_reg == MAX8997_REG_INVALID ||
    IS_ERR_OR_NULL(i2c))
    continue;
    max8997.irq_masks_cache[i] = max8997.irq_masks_cur[i];
    max8997_write_reg(i2c, max8997_mask_reg[i],
    max8997.irq_masks_cur[i]);
    }
    mutex_unlock(&max8997.irqlock);
    }
    inline static const struct max8997_irq_data *
    irq_to_max8997_irq(struct max8997_dev *max8997, struct irq_data *data)
    {
    return &max8997_irqs[data.hwirq];
    }
#[no_mangle]
unsafe extern "C" fn max8997_irq_mask(data: *mut irq_data) {
    static void max8997_irq_mask(struct irq_data *data)
    {
    struct max8997_dev *max8997 = irq_data_get_irq_chip_data(data);
    const struct max8997_irq_data *irq_data = irq_to_max8997_irq(max8997,
    data);
    max8997.irq_masks_cur[irq_data.group] |= irq_data.mask;
    }
#[no_mangle]
unsafe extern "C" fn max8997_irq_unmask(data: *mut irq_data) {
    static void max8997_irq_unmask(struct irq_data *data)
    {
    struct max8997_dev *max8997 = irq_data_get_irq_chip_data(data);
    const struct max8997_irq_data *irq_data = irq_to_max8997_irq(max8997,
    data);
    max8997.irq_masks_cur[irq_data.group] &= ~irq_data.mask;
    }
    static struct irq_chip max8997_irq_chip = {
    .name			= "max8997",
    .irq_bus_lock		= max8997_irq_lock,
    .irq_bus_sync_unlock	= max8997_irq_sync_unlock,
    .irq_mask		= max8997_irq_mask,
    .irq_unmask		= max8997_irq_unmask,
    };

#[no_mangle]
unsafe extern "C" fn max8997_irq_thread(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t max8997_irq_thread(int irq, void *data)
    {
    struct max8997_dev *max8997 = data;
    u8 irq_reg[MAX8997_IRQ_GROUP_NR] = {};
    u8 irq_src;
    int ret;
    int i, cur_irq;
    ret = max8997_read_reg(max8997.i2c, MAX8997_REG_INTSRC, &irq_src);
    if (ret < 0) {
    dev_err(max8997.dev, "Failed to read interrupt source: %d\n",
    ret);
    return IRQ_NONE;
    }
    if (irq_src & MAX8997_IRQSRC_PMIC) {
// PMIC INT1 ~ INT4
    max8997_bulk_read(max8997.i2c, MAX8997_REG_INT1, 4,
    &irq_reg[PMIC_INT1]);
    }
    if (irq_src & MAX8997_IRQSRC_FUELGAUGE) {
//
// TODO: FUEL GAUGE
//
// This is to be supported by Max17042 driver. When
// an interrupt incurs here, it should be relayed to a
// Max17042 device that is connected (probably by
// platform-data). However, we do not have interrupt
// handling in Max17042 driver currently. The Max17042 IRQ
// driver should be ready to be used as a stand-alone device and
// a Max8997-dependent device. Because it is not ready in
// Max17042-side and it is not too critical in operating
// Max8997, we do not implement this in initial releases.
//
    irq_reg[FUEL_GAUGE] = 0;
    }
    if (irq_src & MAX8997_IRQSRC_MUIC) {
// MUIC INT1 ~ INT3
    max8997_bulk_read(max8997.muic, MAX8997_MUIC_REG_INT1, 3,
    &irq_reg[MUIC_INT1]);
    }
    if (irq_src & MAX8997_IRQSRC_GPIO) {
// GPIO Interrupt
    u8 gpio_info[MAX8997_NUM_GPIO];
    irq_reg[GPIO_LOW] = 0;
    irq_reg[GPIO_HI] = 0;
    max8997_bulk_read(max8997.i2c, MAX8997_REG_GPIOCNTL1,
    MAX8997_NUM_GPIO, gpio_info);
    for (i = 0; i < MAX8997_NUM_GPIO; i++) {
    let mut interrupt: bool = false;
    switch (gpio_info[i] & MAX8997_GPIO_INT_MASK) {
    case MAX8997_GPIO_INT_BOTH:
    if (max8997.gpio_status[i] != gpio_info[i])
    interrupt = true;
    break;
    case MAX8997_GPIO_INT_RISE:
    if ((max8997.gpio_status[i] != gpio_info[i]) &&
    (gpio_info[i] & MAX8997_GPIO_DATA_MASK))
    interrupt = true;
    break;
    case MAX8997_GPIO_INT_FALL:
    if ((max8997.gpio_status[i] != gpio_info[i]) &&
    !(gpio_info[i] & MAX8997_GPIO_DATA_MASK))
    interrupt = true;
    break;
    default:
    break;
    }
    if (interrupt) {
    if (i < 8)
    irq_reg[GPIO_LOW] |= (1 << i);
    else
    irq_reg[GPIO_HI] |= (1 << (i - 8));
    }
    }
    }
    if (irq_src & MAX8997_IRQSRC_FLASH) {
// Flash Status Interrupt
    ret = max8997_read_reg(max8997.i2c, MAX8997_REG_FLASHSTATUS,
    &irq_reg[FLASH_STATUS]);
    }
// Apply masking
    for (i = 0; i < MAX8997_IRQ_GROUP_NR; i++)
    irq_reg[i] &= ~max8997.irq_masks_cur[i];
// Report
    for (i = 0; i < MAX8997_IRQ_NR; i++) {
    if (irq_reg[max8997_irqs[i].group] & max8997_irqs[i].mask) {
    cur_irq = irq_find_mapping(max8997.irq_domain, i);
    if (cur_irq)
    handle_nested_irq(cur_irq);
    }
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
pub unsafe extern "C" fn max8997_irq_resume(max8997: *mut max8997_dev) -> c_int {
    int max8997_irq_resume(struct max8997_dev *max8997)
    {
    if (max8997.irq && max8997.irq_domain)
    max8997_irq_thread(0, max8997);
    return 0;
    }
    static int max8997_irq_domain_map(struct irq_domain *d, unsigned int irq,
    irq_hw_number_t hw)
    {
    struct max8997_dev *max8997 = d.host_data;
    irq_set_chip_data(irq, max8997);
    irq_set_chip_and_handler(irq, &max8997_irq_chip, handle_edge_irq);
    irq_set_nested_thread(irq, 1);
    irq_set_noprobe(irq);
    return 0;
    }
    static const struct irq_domain_ops max8997_irq_domain_ops = {
    .map = max8997_irq_domain_map,
    };
#[no_mangle]
pub unsafe extern "C" fn max8997_irq_init(max8997: *mut max8997_dev) -> c_int {
    int max8997_irq_init(struct max8997_dev *max8997)
    {
    struct irq_domain *domain;
    int i;
    int ret;
    u8 val;
    if (!max8997.irq) {
    dev_warn(max8997.dev, "No interrupt specified.\n");
    return 0;
    }
    mutex_init(&max8997.irqlock);
// Mask individual interrupt sources
    for (i = 0; i < MAX8997_IRQ_GROUP_NR; i++) {
    struct i2c_client *i2c;
    max8997.irq_masks_cur[i] = 0xff;
    max8997.irq_masks_cache[i] = 0xff;
    i2c = get_i2c(max8997, i);
    if (IS_ERR_OR_NULL(i2c))
    continue;
    if (max8997_mask_reg[i] == MAX8997_REG_INVALID)
    continue;
    max8997_write_reg(i2c, max8997_mask_reg[i], 0xff);
    }
    for (i = 0; i < MAX8997_NUM_GPIO; i++) {
    max8997.gpio_status[i] = (max8997_read_reg(max8997.i2c,
    MAX8997_REG_GPIOCNTL1 + i,
    &val)
    & MAX8997_GPIO_DATA_MASK) ?
    true : false;
    }
    domain = irq_domain_create_linear(core::ptr::null_mut(), MAX8997_IRQ_NR,
    &max8997_irq_domain_ops, max8997);
    if (!domain) {
    dev_err(max8997.dev, "could not create irq domain\n");
    return -ENODEV;
    }
    max8997.irq_domain = domain;
    ret = devm_request_threaded_irq(max8997.dev, max8997.irq, core::ptr::null_mut(),
    max8997_irq_thread,
    IRQF_TRIGGER_FALLING | IRQF_ONESHOT,
    "max8997-irq", max8997);
    if (ret) {
    dev_err(max8997.dev, "Failed to request IRQ %d: %d\n",
    max8997.irq, ret);
    return ret;
    }
    if (!max8997.ono)
    return 0;
    ret = devm_request_threaded_irq(max8997.dev, max8997.ono, core::ptr::null_mut(),
    max8997_irq_thread,
    IRQF_TRIGGER_FALLING | IRQF_TRIGGER_RISING |
    IRQF_ONESHOT, "max8997-ono", max8997);
    if (ret)
    dev_err(max8997.dev, "Failed to request ono-IRQ %d: %d\n",
    max8997.ono, ret);
    return 0;
    }
