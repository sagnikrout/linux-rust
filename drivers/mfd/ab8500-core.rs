//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/ab8500-core.c
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
// Author: Srinidhi Kasagar <srinidhi.kasagar@stericsson.com>
// Author: Rabin Vincent <rabin.vincent@stericsson.com>
// Author: Mattias Wallin <mattias.wallin@stericsson.com>
//

//
// Interrupt register offsets
// Bank : 0x0E
//
pub const AB8500_IT_SOURCE1_REG: c_uint = 0x00;
pub const AB8500_IT_SOURCE2_REG: c_uint = 0x01;
pub const AB8500_IT_SOURCE3_REG: c_uint = 0x02;
pub const AB8500_IT_SOURCE4_REG: c_uint = 0x03;
pub const AB8500_IT_SOURCE5_REG: c_uint = 0x04;
pub const AB8500_IT_SOURCE6_REG: c_uint = 0x05;
pub const AB8500_IT_SOURCE7_REG: c_uint = 0x06;
pub const AB8500_IT_SOURCE8_REG: c_uint = 0x07;
pub const AB9540_IT_SOURCE13_REG: c_uint = 0x0C;
pub const AB8500_IT_SOURCE19_REG: c_uint = 0x12;
pub const AB8500_IT_SOURCE20_REG: c_uint = 0x13;
pub const AB8500_IT_SOURCE21_REG: c_uint = 0x14;
pub const AB8500_IT_SOURCE22_REG: c_uint = 0x15;
pub const AB8500_IT_SOURCE23_REG: c_uint = 0x16;
pub const AB8500_IT_SOURCE24_REG: c_uint = 0x17;
//
// latch registers
//
pub const AB8500_IT_LATCH1_REG: c_uint = 0x20;
pub const AB8500_IT_LATCH2_REG: c_uint = 0x21;
pub const AB8500_IT_LATCH3_REG: c_uint = 0x22;
pub const AB8500_IT_LATCH4_REG: c_uint = 0x23;
pub const AB8500_IT_LATCH5_REG: c_uint = 0x24;
pub const AB8500_IT_LATCH6_REG: c_uint = 0x25;
pub const AB8500_IT_LATCH7_REG: c_uint = 0x26;
pub const AB8500_IT_LATCH8_REG: c_uint = 0x27;
pub const AB8500_IT_LATCH9_REG: c_uint = 0x28;
pub const AB8500_IT_LATCH10_REG: c_uint = 0x29;
pub const AB8500_IT_LATCH12_REG: c_uint = 0x2B;
pub const AB9540_IT_LATCH13_REG: c_uint = 0x2C;
pub const AB8500_IT_LATCH19_REG: c_uint = 0x32;
pub const AB8500_IT_LATCH20_REG: c_uint = 0x33;
pub const AB8500_IT_LATCH21_REG: c_uint = 0x34;
pub const AB8500_IT_LATCH22_REG: c_uint = 0x35;
pub const AB8500_IT_LATCH23_REG: c_uint = 0x36;
pub const AB8500_IT_LATCH24_REG: c_uint = 0x37;
//
// mask registers
//
pub const AB8500_IT_MASK1_REG: c_uint = 0x40;
pub const AB8500_IT_MASK2_REG: c_uint = 0x41;
pub const AB8500_IT_MASK3_REG: c_uint = 0x42;
pub const AB8500_IT_MASK4_REG: c_uint = 0x43;
pub const AB8500_IT_MASK5_REG: c_uint = 0x44;
pub const AB8500_IT_MASK6_REG: c_uint = 0x45;
pub const AB8500_IT_MASK7_REG: c_uint = 0x46;
pub const AB8500_IT_MASK8_REG: c_uint = 0x47;
pub const AB8500_IT_MASK9_REG: c_uint = 0x48;
pub const AB8500_IT_MASK10_REG: c_uint = 0x49;
pub const AB8500_IT_MASK11_REG: c_uint = 0x4A;
pub const AB8500_IT_MASK12_REG: c_uint = 0x4B;
pub const AB8500_IT_MASK13_REG: c_uint = 0x4C;
pub const AB8500_IT_MASK14_REG: c_uint = 0x4D;
pub const AB8500_IT_MASK15_REG: c_uint = 0x4E;
pub const AB8500_IT_MASK16_REG: c_uint = 0x4F;
pub const AB8500_IT_MASK17_REG: c_uint = 0x50;
pub const AB8500_IT_MASK18_REG: c_uint = 0x51;
pub const AB8500_IT_MASK19_REG: c_uint = 0x52;
pub const AB8500_IT_MASK20_REG: c_uint = 0x53;
pub const AB8500_IT_MASK21_REG: c_uint = 0x54;
pub const AB8500_IT_MASK22_REG: c_uint = 0x55;
pub const AB8500_IT_MASK23_REG: c_uint = 0x56;
pub const AB8500_IT_MASK24_REG: c_uint = 0x57;
pub const AB8500_IT_MASK25_REG: c_uint = 0x58;
//
// latch hierarchy registers
//
pub const AB8500_IT_LATCHHIER1_REG: c_uint = 0x60;
pub const AB8500_IT_LATCHHIER2_REG: c_uint = 0x61;
pub const AB8500_IT_LATCHHIER3_REG: c_uint = 0x62;
pub const AB8540_IT_LATCHHIER4_REG: c_uint = 0x63;
pub const AB8500_IT_LATCHHIER_NUM: c_int = 3;
pub const AB8540_IT_LATCHHIER_NUM: c_int = 4;
pub const AB8500_REV_REG: c_uint = 0x80;
pub const AB8500_IC_NAME_REG: c_uint = 0x82;
pub const AB8500_SWITCH_OFF_STATUS: c_uint = 0x00;
pub const AB8500_TURN_ON_STATUS: c_uint = 0x00;
pub const AB8505_TURN_ON_STATUS_2: c_uint = 0x04;
pub const AB8500_CH_USBCH_STAT1_REG: c_uint = 0x02;
pub const VBUS_DET_DBNC100: c_uint = 0x02;
pub const VBUS_DET_DBNC1: c_uint = 0x01;
    static DEFINE_SPINLOCK(on_stat_lock);
    let mut turn_on_stat_mask: static u8 = 0xFF;
    static u8 turn_on_stat_set;
pub const AB9540_MODEM_CTRL2_REG: c_uint = 0x23;

//
// Map interrupt numbers to the LATCH and MASK register offsets, Interrupt
// numbers are indexed into this array with (num / 8). The interupts are
// defined in linux/mfd/ab8500.h
//
// This is one off from the register names, i.e. AB8500_IT_MASK1_REG is at
// offset 0.
//
// AB8500 support
    static const int ab8500_irq_regoffset[AB8500_NUM_IRQ_REGS] = {
    0, 1, 2, 3, 4, 6, 7, 8, 9, 11, 18, 19, 20, 21,
    };
// AB9540 / AB8505 support
    static const int ab9540_irq_regoffset[AB9540_NUM_IRQ_REGS] = {
    0, 1, 2, 3, 4, 6, 7, 8, 9, 11, 18, 19, 20, 21, 12, 13, 24, 5, 22, 23
    };
// AB8540 support
    static const int ab8540_irq_regoffset[AB8540_NUM_IRQ_REGS] = {
    0, 1, 2, 3, 4, -1, -1, -1, -1, 11, 18, 19, 20, 21, 12, 13, 24, 5, 22,
    23, 25, 26, 27, 28, 29, 30, 31,
    };
    static const char ab8500_version_str[][7] = {
    [AB8500_VERSION_AB8500] = "AB8500",
    [AB8500_VERSION_AB8505] = "AB8505",
    [AB8500_VERSION_AB9540] = "AB9540",
    [AB8500_VERSION_AB8540] = "AB8540",
    };
#[no_mangle]
unsafe extern "C" fn ab8500_prcmu_write(ab8500: *mut ab8500, addr: u16, data: u8) -> c_int {
    static int ab8500_prcmu_write(struct ab8500 *ab8500, u16 addr, u8 data)
    {
    int ret;
    ret = prcmu_abb_write((u8)(addr >> 8), (u8)(addr & 0xFF), &data, 1);
    if (ret < 0)
    dev_err(ab8500.dev, "prcmu i2c error %d\n", ret);
    return ret;
    }
    static int ab8500_prcmu_write_masked(struct ab8500 *ab8500, u16 addr, u8 mask,
    u8 data)
    {
    int ret;
    ret = prcmu_abb_write_masked((u8)(addr >> 8), (u8)(addr & 0xFF), &data,
    &mask, 1);
    if (ret < 0)
    dev_err(ab8500.dev, "prcmu i2c error %d\n", ret);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ab8500_prcmu_read(ab8500: *mut ab8500, addr: u16) -> c_int {
    static int ab8500_prcmu_read(struct ab8500 *ab8500, u16 addr)
    {
    int ret;
    u8 data;
    ret = prcmu_abb_read((u8)(addr >> 8), (u8)(addr & 0xFF), &data, 1);
    if (ret < 0) {
    dev_err(ab8500.dev, "prcmu i2c error %d\n", ret);
    return ret;
    }
    return (int)data;
    }
#[no_mangle]
unsafe extern "C" fn ab8500_get_chip_id(dev: *mut device) -> c_int {
    static int ab8500_get_chip_id(struct device *dev)
    {
    struct ab8500 *ab8500;
    if (!dev)
    return -EINVAL;
    ab8500 = dev_get_drvdata(dev.parent);
    return ab8500 ? (int)ab8500.chip_id : -EINVAL;
    }
    static int set_register_interruptible(struct ab8500 *ab8500, u8 bank,
    u8 reg, u8 data)
    {
    int ret;
//
// Put the u8 bank and u8 register together into a an u16.
// The bank on higher 8 bits and register in lower 8 bits.
//
    let mut addr: u16 = ((u16)bank) << 8 | reg;
    dev_vdbg(ab8500.dev, "wr: addr %#x <= %#x\n", addr, data);
    mutex_lock(&ab8500.lock);
    ret = ab8500.write(ab8500, addr, data);
    if (ret < 0)
    dev_err(ab8500.dev, "failed to write reg %#x: %d\n",
    addr, ret);
    mutex_unlock(&ab8500.lock);
    return ret;
    }
    static int ab8500_set_register(struct device *dev, u8 bank,
    u8 reg, u8 value)
    {
    int ret;
    struct ab8500 *ab8500 = dev_get_drvdata(dev.parent);
    atomic_inc(&ab8500.transfer_ongoing);
    ret = set_register_interruptible(ab8500, bank, reg, value);
    atomic_dec(&ab8500.transfer_ongoing);
    return ret;
    }
    static int get_register_interruptible(struct ab8500 *ab8500, u8 bank,
    u8 reg, u8 *value)
    {
    int ret;
    let mut addr: u16 = ((u16)bank) << 8 | reg;
    mutex_lock(&ab8500.lock);
    ret = ab8500.read(ab8500, addr);
    if (ret < 0)
    dev_err(ab8500.dev, "failed to read reg %#x: %d\n",
    addr, ret);
    else
// value = ret;
    mutex_unlock(&ab8500.lock);
    dev_vdbg(ab8500.dev, "rd: addr %#x => data %#x\n", addr, ret);
    return (ret < 0) ? ret : 0;
    }
    static int ab8500_get_register(struct device *dev, u8 bank,
    u8 reg, u8 *value)
    {
    int ret;
    struct ab8500 *ab8500 = dev_get_drvdata(dev.parent);
    atomic_inc(&ab8500.transfer_ongoing);
    ret = get_register_interruptible(ab8500, bank, reg, value);
    atomic_dec(&ab8500.transfer_ongoing);
    return ret;
    }
    static int mask_and_set_register_interruptible(struct ab8500 *ab8500, u8 bank,
    u8 reg, u8 bitmask, u8 bitvalues)
    {
    int ret;
    let mut addr: u16 = ((u16)bank) << 8 | reg;
    mutex_lock(&ab8500.lock);
    if (ab8500.write_masked == core::ptr::null_mut()) {
    u8 data;
    ret = ab8500.read(ab8500, addr);
    if (ret < 0) {
    dev_err(ab8500.dev, "failed to read reg %#x: %d\n",
    addr, ret);
    goto out;
    }
    data = (u8)ret;
    data = (~bitmask & data) | (bitmask & bitvalues);
    ret = ab8500.write(ab8500, addr, data);
    if (ret < 0)
    dev_err(ab8500.dev, "failed to write reg %#x: %d\n",
    addr, ret);
    dev_vdbg(ab8500.dev, "mask: addr %#x => data %#x\n", addr,
    data);
    goto out;
    }
    ret = ab8500.write_masked(ab8500, addr, bitmask, bitvalues);
    if (ret < 0)
    dev_err(ab8500.dev, "failed to modify reg %#x: %d\n", addr,
    ret);
    out:
    mutex_unlock(&ab8500.lock);
    return ret;
    }
    static int ab8500_mask_and_set_register(struct device *dev,
    u8 bank, u8 reg, u8 bitmask, u8 bitvalues)
    {
    int ret;
    struct ab8500 *ab8500 = dev_get_drvdata(dev.parent);
    atomic_inc(&ab8500.transfer_ongoing);
    ret = mask_and_set_register_interruptible(ab8500, bank, reg,
    bitmask, bitvalues);
    atomic_dec(&ab8500.transfer_ongoing);
    return ret;
    }
    static struct abx500_ops ab8500_ops = {
    .get_chip_id = ab8500_get_chip_id,
    .get_register = ab8500_get_register,
    .set_register = ab8500_set_register,
    .get_register_page = core::ptr::null_mut(),
    .set_register_page = core::ptr::null_mut(),
    .mask_and_set_register = ab8500_mask_and_set_register,
    .event_registers_startup_state_get = core::ptr::null_mut(),
    .startup_irq_enabled = core::ptr::null_mut(),
    .dump_all_banks = ab8500_dump_all_banks,
    };
#[no_mangle]
unsafe extern "C" fn ab8500_irq_lock(data: *mut irq_data) {
    static void ab8500_irq_lock(struct irq_data *data)
    {
    struct ab8500 *ab8500 = irq_data_get_irq_chip_data(data);
    mutex_lock(&ab8500.irq_lock);
    atomic_inc(&ab8500.transfer_ongoing);
    }
#[no_mangle]
unsafe extern "C" fn ab8500_irq_sync_unlock(data: *mut irq_data) {
    static void ab8500_irq_sync_unlock(struct irq_data *data)
    {
    struct ab8500 *ab8500 = irq_data_get_irq_chip_data(data);
    int i;
    for (i = 0; i < ab8500.mask_size; i++) {
    let mut old: u8 = ab8500.oldmask[i];
    let mut new: u8 = ab8500.mask[i];
    int reg;
    if (new == old)
    continue;
//
// Interrupt register 12 doesn't exist prior to AB8500 version
// 2.0
//
    if (ab8500.irq_reg_offset[i] == 11 &&
    is_ab8500_1p1_or_earlier(ab8500))
    continue;
    if (ab8500.irq_reg_offset[i] < 0)
    continue;
    ab8500.oldmask[i] = new;
    reg = AB8500_IT_MASK1_REG + ab8500.irq_reg_offset[i];
    set_register_interruptible(ab8500, AB8500_INTERRUPT, reg, new);
    }
    atomic_dec(&ab8500.transfer_ongoing);
    mutex_unlock(&ab8500.irq_lock);
    }
#[no_mangle]
unsafe extern "C" fn ab8500_irq_mask(data: *mut irq_data) {
    static void ab8500_irq_mask(struct irq_data *data)
    {
    struct ab8500 *ab8500 = irq_data_get_irq_chip_data(data);
    let mut offset: c_int = data.hwirq;
    let mut index: c_int = offset / 8;
    let mut mask: c_int = 1 << (offset % 8);
    ab8500.mask[index] |= mask;
// The AB8500 GPIOs have two interrupts each (rising & falling).
    if (offset >= AB8500_INT_GPIO6R && offset <= AB8500_INT_GPIO41R)
    ab8500.mask[index + 2] |= mask;
    if (offset >= AB9540_INT_GPIO50R && offset <= AB9540_INT_GPIO54R)
    ab8500.mask[index + 1] |= mask;
    if (offset == AB8540_INT_GPIO43R || offset == AB8540_INT_GPIO44R)
// Here the falling IRQ is one bit lower
    ab8500.mask[index] |= (mask << 1);
    }
#[no_mangle]
unsafe extern "C" fn ab8500_irq_unmask(data: *mut irq_data) {
    static void ab8500_irq_unmask(struct irq_data *data)
    {
    struct ab8500 *ab8500 = irq_data_get_irq_chip_data(data);
    let mut type: c_uint = irqd_get_trigger_type(data);
    let mut offset: c_int = data.hwirq;
    let mut index: c_int = offset / 8;
    let mut mask: c_int = 1 << (offset % 8);
    if (type & IRQ_TYPE_EDGE_RISING)
    ab8500.mask[index] &= ~mask;
// The AB8500 GPIOs have two interrupts each (rising & falling).
    if (type & IRQ_TYPE_EDGE_FALLING) {
    if (offset >= AB8500_INT_GPIO6R && offset <= AB8500_INT_GPIO41R)
    ab8500.mask[index + 2] &= ~mask;
    else if (offset >= AB9540_INT_GPIO50R &&
    offset <= AB9540_INT_GPIO54R)
    ab8500.mask[index + 1] &= ~mask;
    else if (offset == AB8540_INT_GPIO43R ||
    offset == AB8540_INT_GPIO44R)
// Here the falling IRQ is one bit lower
    ab8500.mask[index] &= ~(mask << 1);
    else
    ab8500.mask[index] &= ~mask;
    } else {
// Satisfies the case where type is not set.
    ab8500.mask[index] &= ~mask;
    }
    }
#[no_mangle]
unsafe extern "C" fn ab8500_irq_set_type(data: *mut irq_data, type: c_uint) -> c_int {
    static int ab8500_irq_set_type(struct irq_data *data, unsigned int type)
    {
    return 0;
    }
    static struct irq_chip ab8500_irq_chip = {
    .name			= "ab8500",
    .irq_bus_lock		= ab8500_irq_lock,
    .irq_bus_sync_unlock	= ab8500_irq_sync_unlock,
    .irq_mask		= ab8500_irq_mask,
    .irq_disable		= ab8500_irq_mask,
    .irq_unmask		= ab8500_irq_unmask,
    .irq_set_type		= ab8500_irq_set_type,
    };
#[no_mangle]
unsafe extern "C" fn update_latch_offset(offset: *mut u8, i: c_int) {
    static void update_latch_offset(u8 *offset, int i)
    {
// Fix inconsistent ITFromLatch25 bit mapping...
    if (unlikely(*offset == 17))
// offset = 24;
// Fix inconsistent ab8540 bit mapping...
    if (unlikely(*offset == 16))
// offset = 25;
    if ((i == 3) && (*offset >= 24))
// offset += 2;
    }
    static int ab8500_handle_hierarchical_line(struct ab8500 *ab8500,
    int latch_offset, u8 latch_val)
    {
    int int_bit, line, i;
    for (i = 0; i < ab8500.mask_size; i++)
    if (ab8500.irq_reg_offset[i] == latch_offset)
    break;
    if (i >= ab8500.mask_size) {
    dev_err(ab8500.dev, "Register offset 0x%2x not declared\n",
    latch_offset);
    return -ENXIO;
    }
// ignore masked out interrupts
    latch_val &= ~ab8500.mask[i];
    while (latch_val) {
    int_bit = __ffs(latch_val);
    line = (i << 3) + int_bit;
    latch_val &= ~(1 << int_bit);
//
// This handles the falling edge hwirqs from the GPIO
// lines. Route them back to the line registered for the
// rising IRQ, as this is merely a flag for the same IRQ
// in linux terms.
//
    if (line >= AB8500_INT_GPIO6F && line <= AB8500_INT_GPIO41F)
    line -= 16;
    if (line >= AB9540_INT_GPIO50F && line <= AB9540_INT_GPIO54F)
    line -= 8;
    if (line == AB8540_INT_GPIO43F || line == AB8540_INT_GPIO44F)
    line += 1;
    handle_nested_irq(irq_find_mapping(ab8500.domain, line));
    }
    return 0;
    }
    static int ab8500_handle_hierarchical_latch(struct ab8500 *ab8500,
    int hier_offset, u8 hier_val)
    {
    int latch_bit, status;
    u8 latch_offset, latch_val;
    do {
    latch_bit = __ffs(hier_val);
    latch_offset = (hier_offset << 3) + latch_bit;
    update_latch_offset(&latch_offset, hier_offset);
    status = get_register_interruptible(ab8500,
    AB8500_INTERRUPT,
    AB8500_IT_LATCH1_REG + latch_offset,
    &latch_val);
    if (status < 0 || latch_val == 0)
    goto discard;
    status = ab8500_handle_hierarchical_line(ab8500,
    latch_offset, latch_val);
    if (status < 0)
    return status;
    discard:
    hier_val &= ~(1 << latch_bit);
    } while (hier_val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ab8500_hierarchical_irq(irq: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t ab8500_hierarchical_irq(int irq, void *dev)
    {
    struct ab8500 *ab8500 = dev;
    u8 i;
    dev_vdbg(ab8500.dev, "interrupt\n");
// Hierarchical interrupt version
    for (i = 0; i < (ab8500.it_latchhier_num); i++) {
    int status;
    u8 hier_val;
    status = get_register_interruptible(ab8500, AB8500_INTERRUPT,
    AB8500_IT_LATCHHIER1_REG + i, &hier_val);
    if (status < 0 || hier_val == 0)
    continue;
    status = ab8500_handle_hierarchical_latch(ab8500, i, hier_val);
    if (status < 0)
    break;
    }
    return IRQ_HANDLED;
    }
    static int ab8500_irq_map(struct irq_domain *d, unsigned int virq,
    irq_hw_number_t hwirq)
    {
    struct ab8500 *ab8500 = d.host_data;
    if (!ab8500)
    return -EINVAL;
    irq_set_chip_data(virq, ab8500);
    irq_set_chip_and_handler(virq, &ab8500_irq_chip,
    handle_simple_irq);
    irq_set_nested_thread(virq, 1);
    irq_set_noprobe(virq);
    return 0;
    }
    static const struct irq_domain_ops ab8500_irq_ops = {
    .map    = ab8500_irq_map,
    .xlate  = irq_domain_xlate_twocell,
    };
#[no_mangle]
unsafe extern "C" fn ab8500_irq_init(ab8500: *mut ab8500, np: *mut device_node) -> c_int {
    static int ab8500_irq_init(struct ab8500 *ab8500, struct device_node *np)
    {
    int num_irqs;
    if (is_ab8540(ab8500))
    num_irqs = AB8540_NR_IRQS;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: is_ab9540(ab8500)) -> else {
    else if (is_ab9540(ab8500))
    num_irqs = AB9540_NR_IRQS;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: is_ab8505(ab8500)) -> else {
    else if (is_ab8505(ab8500))
    num_irqs = AB8505_NR_IRQS;
    else
    num_irqs = AB8500_NR_IRQS;
// If ->irq_base is zero this will give a linear mapping
    ab8500.domain = irq_domain_create_simple(dev_fwnode(ab8500.dev), num_irqs, 0,
    &ab8500_irq_ops, ab8500);
    if (!ab8500.domain) {
    dev_err(ab8500.dev, "Failed to create irqdomain\n");
    return -ENODEV;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ab8500_suspend(ab8500: *mut ab8500) -> c_int {
    int ab8500_suspend(struct ab8500 *ab8500)
    {
    if (atomic_read(&ab8500.transfer_ongoing))
    return -EINVAL;
    return 0;
    }
    static const struct mfd_cell ab8500_bm_devs[] = {
    MFD_CELL_OF("ab8500-charger", core::ptr::null_mut(), core::ptr::null_mut(), 0, 0,
    "stericsson,ab8500-charger"),
    MFD_CELL_OF("ab8500-btemp", core::ptr::null_mut(), core::ptr::null_mut(), 0, 0,
    "stericsson,ab8500-btemp"),
    MFD_CELL_OF("ab8500-fg", core::ptr::null_mut(), core::ptr::null_mut(), 0, 0,
    "stericsson,ab8500-fg"),
    MFD_CELL_OF("ab8500-chargalg", core::ptr::null_mut(), core::ptr::null_mut(), 0, 0,
    "stericsson,ab8500-chargalg"),
    };
    static const struct mfd_cell ab8500_devs[] = {
    MFD_CELL_OF("ab8500-sysctrl",
    core::ptr::null_mut(), core::ptr::null_mut(), 0, 0, "stericsson,ab8500-sysctrl"),
    MFD_CELL_OF("ab8500-ext-regulator",
    core::ptr::null_mut(), core::ptr::null_mut(), 0, 0, "stericsson,ab8500-ext-regulator"),
    MFD_CELL_OF("ab8500-regulator",
    core::ptr::null_mut(), core::ptr::null_mut(), 0, 0, "stericsson,ab8500-regulator"),
    MFD_CELL_OF("ab8500-clk",
    core::ptr::null_mut(), core::ptr::null_mut(), 0, 0, "stericsson,ab8500-clk"),
    MFD_CELL_OF("ab8500-gpadc",
    core::ptr::null_mut(), core::ptr::null_mut(), 0, 0, "stericsson,ab8500-gpadc"),
    MFD_CELL_OF("ab8500-rtc",
    core::ptr::null_mut(), core::ptr::null_mut(), 0, 0, "stericsson,ab8500-rtc"),
    MFD_CELL_OF("ab8500-acc-det",
    core::ptr::null_mut(), core::ptr::null_mut(), 0, 0, "stericsson,ab8500-acc-det"),
    MFD_CELL_OF("ab8500-poweron-key",
    core::ptr::null_mut(), core::ptr::null_mut(), 0, 0, "stericsson,ab8500-poweron-key"),
    MFD_CELL_OF("ab8500-pwm",
    core::ptr::null_mut(), core::ptr::null_mut(), 0, 1, "stericsson,ab8500-pwm"),
    MFD_CELL_OF("ab8500-pwm",
    core::ptr::null_mut(), core::ptr::null_mut(), 0, 2, "stericsson,ab8500-pwm"),
    MFD_CELL_OF("ab8500-pwm",
    core::ptr::null_mut(), core::ptr::null_mut(), 0, 3, "stericsson,ab8500-pwm"),
    MFD_CELL_OF("ab8500-denc",
    core::ptr::null_mut(), core::ptr::null_mut(), 0, 0, "stericsson,ab8500-denc"),
    MFD_CELL_OF("pinctrl-ab8500",
    core::ptr::null_mut(), core::ptr::null_mut(), 0, 0, "stericsson,ab8500-gpio"),
    MFD_CELL_OF("abx500-temp",
    core::ptr::null_mut(), core::ptr::null_mut(), 0, 0, "stericsson,abx500-temp"),
    MFD_CELL_OF("ab8500-usb",
    core::ptr::null_mut(), core::ptr::null_mut(), 0, 0, "stericsson,ab8500-usb"),
    MFD_CELL_OF("ab8500-codec",
    core::ptr::null_mut(), core::ptr::null_mut(), 0, 0, "stericsson,ab8500-codec"),
    };
    static const struct mfd_cell ab9540_devs[] = {
    {
    .name = "ab8500-sysctrl",
    },
    {
    .name = "ab8500-ext-regulator",
    },
    {
    .name = "ab8500-regulator",
    },
    {
    .name = "abx500-clk",
    .of_compatible = "stericsson,abx500-clk",
    },
    {
    .name = "ab8500-gpadc",
    .of_compatible = "stericsson,ab8500-gpadc",
    },
    {
    .name = "ab8500-rtc",
    },
    {
    .name = "ab8500-acc-det",
    },
    {
    .name = "ab8500-poweron-key",
    },
    {
    .name = "ab8500-pwm",
    .id = 1,
    },
    {
    .name = "abx500-temp",
    },
    {
    .name = "pinctrl-ab9540",
    .of_compatible = "stericsson,ab9540-gpio",
    },
    {
    .name = "ab9540-usb",
    },
    {
    .name = "ab9540-codec",
    },
    {
    .name = "ab-iddet",
    },
    };
// Device list for ab8505
    static const struct mfd_cell ab8505_devs[] = {
    {
    .name = "ab8500-sysctrl",
    .of_compatible = "stericsson,ab8500-sysctrl",
    },
    {
    .name = "ab8500-regulator",
    .of_compatible = "stericsson,ab8505-regulator",
    },
    {
    .name = "abx500-clk",
    .of_compatible = "stericsson,ab8500-clk",
    },
    {
    .name = "ab8500-gpadc",
    .of_compatible = "stericsson,ab8500-gpadc",
    },
    {
    .name = "ab8500-rtc",
    .of_compatible = "stericsson,ab8500-rtc",
    },
    {
    .name = "ab8500-acc-det",
    .of_compatible = "stericsson,ab8500-acc-det",
    },
    {
    .name = "ab8500-poweron-key",
    .of_compatible = "stericsson,ab8500-poweron-key",
    },
    {
    .name = "ab8500-pwm",
    .of_compatible = "stericsson,ab8500-pwm",
    .id = 1,
    },
    {
    .name = "pinctrl-ab8505",
    .of_compatible = "stericsson,ab8505-gpio",
    },
    {
    .name = "ab8500-usb",
    .of_compatible = "stericsson,ab8500-usb",
    },
    {
    .name = "ab8500-codec",
    .of_compatible = "stericsson,ab8500-codec",
    },
    {
    .name = "ab-iddet",
    },
    };
    static const struct mfd_cell ab8540_devs[] = {
    {
    .name = "ab8500-sysctrl",
    },
    {
    .name = "ab8500-ext-regulator",
    },
    {
    .name = "ab8500-regulator",
    },
    {
    .name = "abx500-clk",
    .of_compatible = "stericsson,abx500-clk",
    },
    {
    .name = "ab8500-gpadc",
    .of_compatible = "stericsson,ab8500-gpadc",
    },
    {
    .name = "ab8500-acc-det",
    },
    {
    .name = "ab8500-poweron-key",
    },
    {
    .name = "ab8500-pwm",
    .id = 1,
    },
    {
    .name = "abx500-temp",
    },
    {
    .name = "pinctrl-ab8540",
    },
    {
    .name = "ab8540-usb",
    },
    {
    .name = "ab8540-codec",
    },
    {
    .name = "ab-iddet",
    },
    };
    static const struct mfd_cell ab8540_cut1_devs[] = {
    {
    .name = "ab8500-rtc",
    .of_compatible = "stericsson,ab8500-rtc",
    },
    };
    static const struct mfd_cell ab8540_cut2_devs[] = {
    {
    .name = "ab8540-rtc",
    .of_compatible = "stericsson,ab8540-rtc",
    },
    };
    static ssize_t chip_id_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct ab8500 *ab8500;
    ab8500 = dev_get_drvdata(dev);
    return sprintf(buf, "%#x\n", ab8500 ? ab8500.chip_id : -EINVAL);
    }
//
// ab8500 has switched off due to (SWITCH_OFF_STATUS):
// 0x01 Swoff bit programming
// 0x02 Thermal protection activation
// 0x04 Vbat lower then BattOk falling threshold
// 0x08 Watchdog expired
// 0x10 Non presence of 32kHz clock
// 0x20 Battery level lower than power on reset threshold
// 0x40 Power on key 1 pressed longer than 10 seconds
// 0x80 DB8500 thermal shutdown
//
    static ssize_t switch_off_status_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    int ret;
    u8 value;
    struct ab8500 *ab8500;
    ab8500 = dev_get_drvdata(dev);
    ret = get_register_interruptible(ab8500, AB8500_RTC,
    AB8500_SWITCH_OFF_STATUS, &value);
    if (ret < 0)
    return ret;
    return sprintf(buf, "%#x\n", value);
    }
// use mask and set to override the register turn_on_stat value
#[no_mangle]
pub unsafe extern "C" fn ab8500_override_turn_on_stat(mask: u8, set: u8) {
    void ab8500_override_turn_on_stat(u8 mask, u8 set)
    {
    spin_lock(&on_stat_lock);
    turn_on_stat_mask = mask;
    turn_on_stat_set = set;
    spin_unlock(&on_stat_lock);
    }
//
// ab8500 has turned on due to (TURN_ON_STATUS):
// 0x01 PORnVbat
// 0x02 PonKey1dbF
// 0x04 PonKey2dbF
// 0x08 RTCAlarm
// 0x10 MainChDet
// 0x20 VbusDet
// 0x40 UsbIDDetect
// 0x80 Reserved
//
    static ssize_t turn_on_status_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    int ret;
    u8 value;
    struct ab8500 *ab8500;
    ab8500 = dev_get_drvdata(dev);
    ret = get_register_interruptible(ab8500, AB8500_SYS_CTRL1_BLOCK,
    AB8500_TURN_ON_STATUS, &value);
    if (ret < 0)
    return ret;
//
// In L9540, turn_on_status register is not updated correctly if
// the device is rebooted with AC/USB charger connected. Due to
// this, the device boots android instead of entering into charge
// only mode. Read the AC/USB status register to detect the charger
// presence and update the turn on status manually.
//
    if (is_ab9540(ab8500)) {
    spin_lock(&on_stat_lock);
    value = (value & turn_on_stat_mask) | turn_on_stat_set;
    spin_unlock(&on_stat_lock);
    }
    return sprintf(buf, "%#x\n", value);
    }
    static ssize_t turn_on_status_2_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    int ret;
    u8 value;
    struct ab8500 *ab8500;
    ab8500 = dev_get_drvdata(dev);
    ret = get_register_interruptible(ab8500, AB8500_SYS_CTRL1_BLOCK,
    AB8505_TURN_ON_STATUS_2, &value);
    if (ret < 0)
    return ret;
    return sprintf(buf, "%#x\n", (value & 0x1));
    }
    static ssize_t dbbrstn_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct ab8500 *ab8500;
    int ret;
    u8 value;
    ab8500 = dev_get_drvdata(dev);
    ret = get_register_interruptible(ab8500, AB8500_REGU_CTRL2,
    AB9540_MODEM_CTRL2_REG, &value);
    if (ret < 0)
    return ret;
    return sprintf(buf, "%d\n",
    (value & AB9540_MODEM_CTRL2_SWDBBRSTN_BIT) ? 1 : 0);
    }
    static ssize_t dbbrstn_store(struct device *dev,
    struct device_attribute *attr, const char *buf, size_t count)
    {
    struct ab8500 *ab8500;
    let mut ret: c_int = count;
    int err;
    u8 bitvalues;
    ab8500 = dev_get_drvdata(dev);
    if (count > 0) {
    switch (buf[0]) {
    case '0':
    bitvalues = 0;
    break;
    case '1':
    bitvalues = AB9540_MODEM_CTRL2_SWDBBRSTN_BIT;
    break;
    default:
    goto exit;
    }
    err = mask_and_set_register_interruptible(ab8500,
    AB8500_REGU_CTRL2, AB9540_MODEM_CTRL2_REG,
    AB9540_MODEM_CTRL2_SWDBBRSTN_BIT, bitvalues);
    if (err)
    dev_info(ab8500.dev,
    "Failed to set DBBRSTN %c, err %#x\n",
    buf[0], err);
    }
    exit:
    return ret;
    }
    static DEVICE_ATTR_RO(chip_id);
    static DEVICE_ATTR_RO(switch_off_status);
    static DEVICE_ATTR_RO(turn_on_status);
    static DEVICE_ATTR_RO(turn_on_status_2);
    static DEVICE_ATTR_RW(dbbrstn);
    static struct attribute *ab8500_sysfs_entries[] = {
    &dev_attr_chip_id.attr,
    &dev_attr_switch_off_status.attr,
    &dev_attr_turn_on_status.attr,
    core::ptr::null_mut(),
    };
    static struct attribute *ab8505_sysfs_entries[] = {
    &dev_attr_turn_on_status_2.attr,
    core::ptr::null_mut(),
    };
    static struct attribute *ab9540_sysfs_entries[] = {
    &dev_attr_chip_id.attr,
    &dev_attr_switch_off_status.attr,
    &dev_attr_turn_on_status.attr,
    &dev_attr_dbbrstn.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group ab8500_attr_group = {
    .attrs	= ab8500_sysfs_entries,
    };
    static const struct attribute_group ab8505_attr_group = {
    .attrs	= ab8505_sysfs_entries,
    };
    static const struct attribute_group ab9540_attr_group = {
    .attrs	= ab9540_sysfs_entries,
    };
#[no_mangle]
unsafe extern "C" fn ab8500_probe(pdev: *mut platform_device) -> c_int {
    static int ab8500_probe(struct platform_device *pdev)
    {
    static const char * const switch_off_status[] = {
    "Swoff bit programming",
    "Thermal protection activation",
    "Vbat lower then BattOk falling threshold",
    "Watchdog expired",
    "Non presence of 32kHz clock",
    "Battery level lower than power on reset threshold",
    "Power on key 1 pressed longer than 10 seconds",
    "DB8500 thermal shutdown"};
    static const char * const turn_on_status[] = {
    "Battery rising (Vbat)",
    "Power On Key 1 dbF",
    "Power On Key 2 dbF",
    "RTC Alarm",
    "Main Charger Detect",
    "Vbus Detect (USB)",
    "USB ID Detect",
    "UART Factory Mode Detect"};
    const struct platform_device_id *platid = platform_get_device_id(pdev);
    let mut version: enum ab8500_version = AB8500_VERSION_UNDEFINED;
    struct device_node *np = pdev.dev.of_node;
    struct ab8500 *ab8500;
    int ret;
    int i;
    int irq;
    u8 value;
    ab8500 = devm_kzalloc(&pdev.dev, sizeof(*ab8500), GFP_KERNEL);
    if (!ab8500)
    return -ENOMEM;
    ab8500.dev = &pdev.dev;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    ab8500.irq = irq;
    ab8500.read = ab8500_prcmu_read;
    ab8500.write = ab8500_prcmu_write;
    ab8500.write_masked = ab8500_prcmu_write_masked;
    mutex_init(&ab8500.lock);
    mutex_init(&ab8500.irq_lock);
    atomic_set(&ab8500.transfer_ongoing, 0);
    platform_set_drvdata(pdev, ab8500);
    if (platid)
    version = platid.driver_data;
    if (version != AB8500_VERSION_UNDEFINED)
    ab8500.version = version;
    else {
    ret = get_register_interruptible(ab8500, AB8500_MISC,
    AB8500_IC_NAME_REG, &value);
    if (ret < 0) {
    dev_err(&pdev.dev, "could not probe HW\n");
    return ret;
    }
    ab8500.version = value;
    }
    ret = get_register_interruptible(ab8500, AB8500_MISC,
    AB8500_REV_REG, &value);
    if (ret < 0)
    return ret;
    ab8500.chip_id = value;
    dev_info(ab8500.dev, "detected chip, %s rev. %1x.%1x\n",
    ab8500_version_str[ab8500.version],
    ab8500.chip_id >> 4,
    ab8500.chip_id & 0x0F);
// Configure AB8540
    if (is_ab8540(ab8500)) {
    ab8500.mask_size = AB8540_NUM_IRQ_REGS;
    ab8500.irq_reg_offset = ab8540_irq_regoffset;
    ab8500.it_latchhier_num = AB8540_IT_LATCHHIER_NUM;
    } /* Configure AB8500 or AB9540 IRQ */
#[no_mangle]
pub unsafe extern "C" fn if(is_ab8505(ab8500): is_ab9540(ab8500) ||) -> else {
    ab8500.mask_size = AB9540_NUM_IRQ_REGS;
    ab8500.irq_reg_offset = ab9540_irq_regoffset;
    ab8500.it_latchhier_num = AB8500_IT_LATCHHIER_NUM;
    } else {
    ab8500.mask_size = AB8500_NUM_IRQ_REGS;
    ab8500.irq_reg_offset = ab8500_irq_regoffset;
    ab8500.it_latchhier_num = AB8500_IT_LATCHHIER_NUM;
    }
    ab8500.mask = devm_kzalloc(&pdev.dev, ab8500.mask_size,
    GFP_KERNEL);
    if (!ab8500.mask)
    return -ENOMEM;
    ab8500.oldmask = devm_kzalloc(&pdev.dev, ab8500.mask_size,
    GFP_KERNEL);
    if (!ab8500.oldmask)
    return -ENOMEM;
//
// ab8500 has switched off due to (SWITCH_OFF_STATUS):
// 0x01 Swoff bit programming
// 0x02 Thermal protection activation
// 0x04 Vbat lower then BattOk falling threshold
// 0x08 Watchdog expired
// 0x10 Non presence of 32kHz clock
// 0x20 Battery level lower than power on reset threshold
// 0x40 Power on key 1 pressed longer than 10 seconds
// 0x80 DB8500 thermal shutdown
//
    ret = get_register_interruptible(ab8500, AB8500_RTC,
    AB8500_SWITCH_OFF_STATUS, &value);
    if (ret < 0)
    return ret;
    dev_info(ab8500.dev, "switch off cause(s) (%#x): ", value);
    if (value) {
    for (i = 0; i < ARRAY_SIZE(switch_off_status); i++) {
    if (value & 1)
    pr_cont(" \"%s\"", switch_off_status[i]);
    value = value >> 1;
    }
    pr_cont("\n");
    } else {
    pr_cont(" None\n");
    }
    ret = get_register_interruptible(ab8500, AB8500_SYS_CTRL1_BLOCK,
    AB8500_TURN_ON_STATUS, &value);
    if (ret < 0)
    return ret;
    dev_info(ab8500.dev, "turn on reason(s) (%#x): ", value);
    if (value) {
    for (i = 0; i < ARRAY_SIZE(turn_on_status); i++) {
    if (value & 1)
    pr_cont("\"%s\" ", turn_on_status[i]);
    value = value >> 1;
    }
    pr_cont("\n");
    } else {
    pr_cont("None\n");
    }
    if (is_ab9540(ab8500)) {
    ret = get_register_interruptible(ab8500, AB8500_CHARGER,
    AB8500_CH_USBCH_STAT1_REG, &value);
    if (ret < 0)
    return ret;
    if ((value & VBUS_DET_DBNC1) && (value & VBUS_DET_DBNC100))
    ab8500_override_turn_on_stat(~AB8500_POW_KEY_1_ON,
    AB8500_VBUS_DET);
    }
// Clear and mask all interrupts
    for (i = 0; i < ab8500.mask_size; i++) {
//
// Interrupt register 12 doesn't exist prior to AB8500 version
// 2.0
//
    if (ab8500.irq_reg_offset[i] == 11 &&
    is_ab8500_1p1_or_earlier(ab8500))
    continue;
    if (ab8500.irq_reg_offset[i] < 0)
    continue;
    get_register_interruptible(ab8500, AB8500_INTERRUPT,
    AB8500_IT_LATCH1_REG + ab8500.irq_reg_offset[i],
    &value);
    set_register_interruptible(ab8500, AB8500_INTERRUPT,
    AB8500_IT_MASK1_REG + ab8500.irq_reg_offset[i], 0xff);
    }
    ret = abx500_register_ops(ab8500.dev, &ab8500_ops);
    if (ret)
    return ret;
    for (i = 0; i < ab8500.mask_size; i++)
    ab8500.mask[i] = ab8500.oldmask[i] = 0xff;
    ret = ab8500_irq_init(ab8500, np);
    if (ret)
    return ret;
    ret = devm_request_threaded_irq(&pdev.dev, ab8500.irq, core::ptr::null_mut(),
    ab8500_hierarchical_irq,
    IRQF_ONESHOT | IRQF_NO_SUSPEND,
    "ab8500", ab8500);
    if (ret)
    return ret;
    if (is_ab9540(ab8500))
    ret = mfd_add_devices(ab8500.dev, 0, ab9540_devs,
    ARRAY_SIZE(ab9540_devs), core::ptr::null_mut(),
    0, ab8500.domain);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: is_ab8540(ab8500)) -> else {
    ret = mfd_add_devices(ab8500.dev, 0, ab8540_devs,
    ARRAY_SIZE(ab8540_devs), core::ptr::null_mut(),
    0, ab8500.domain);
    if (ret)
    return ret;
    if (is_ab8540_1p2_or_earlier(ab8500))
    ret = mfd_add_devices(ab8500.dev, 0, ab8540_cut1_devs,
    ARRAY_SIZE(ab8540_cut1_devs), core::ptr::null_mut(),
    0, ab8500.domain);
    else /* ab8540 >= cut2 */
    ret = mfd_add_devices(ab8500.dev, 0, ab8540_cut2_devs,
    ARRAY_SIZE(ab8540_cut2_devs), core::ptr::null_mut(),
    0, ab8500.domain);
    } else if (is_ab8505(ab8500))
    ret = mfd_add_devices(ab8500.dev, 0, ab8505_devs,
    ARRAY_SIZE(ab8505_devs), core::ptr::null_mut(),
    0, ab8500.domain);
    else
    ret = mfd_add_devices(ab8500.dev, 0, ab8500_devs,
    ARRAY_SIZE(ab8500_devs), core::ptr::null_mut(),
    0, ab8500.domain);
    if (ret)
    return ret;
// Add battery management devices
    ret = mfd_add_devices(ab8500.dev, 0, ab8500_bm_devs,
    ARRAY_SIZE(ab8500_bm_devs), core::ptr::null_mut(),
    0, ab8500.domain);
    if (ret)
    dev_err(ab8500.dev, "error adding bm devices\n");
    if (((is_ab8505(ab8500) || is_ab9540(ab8500)) &&
    ab8500.chip_id >= AB8500_CUT2P0) || is_ab8540(ab8500))
    ret = sysfs_create_group(&ab8500.dev.kobj,
    &ab9540_attr_group);
    else
    ret = sysfs_create_group(&ab8500.dev.kobj,
    &ab8500_attr_group);
    if ((is_ab8505(ab8500) || is_ab9540(ab8500)) &&
    ab8500.chip_id >= AB8500_CUT2P0)
    ret = sysfs_create_group(&ab8500.dev.kobj,
    &ab8505_attr_group);
    if (ret)
    dev_err(ab8500.dev, "error creating sysfs entries\n");
    return ret;
    }
    static const struct platform_device_id ab8500_id[] = {
    { "ab8500-core", AB8500_VERSION_AB8500 },
    { "ab8505-core", AB8500_VERSION_AB8505 },
    { "ab9540-i2c", AB8500_VERSION_AB9540 },
    { "ab8540-i2c", AB8500_VERSION_AB8540 },
    { }
    };
    static struct platform_driver ab8500_core_driver = {
    .driver = {
    .name = "ab8500-core",
    .suppress_bind_attrs = true,
    },
    .probe	= ab8500_probe,
    .id_table = ab8500_id,
    };
#[no_mangle]
unsafe extern "C" fn ab8500_core_init() -> int __init {
    static int __init ab8500_core_init(void)
    {
    return platform_driver_register(&ab8500_core_driver);
    }
    core_initcall(ab8500_core_init);
