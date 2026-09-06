//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/mediatek/mt76/mmio.c
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


// SPDX-License-Identifier: BSD-3-Clause-Clear
//
// Copyright (C) 2016 Felix Fietkau <nbd@nbd.name>
//

#[no_mangle]
unsafe extern "C" fn mt76_mmio_rr(dev: *mut mt76_dev, offset: u32) -> u32 {
    static u32 mt76_mmio_rr(struct mt76_dev *dev, u32 offset)
    {
    u32 val;
    val = readl(dev.mmio.regs + offset);
    trace_reg_rr(dev, offset, val);
    return val;
    }
#[no_mangle]
unsafe extern "C" fn mt76_mmio_wr(dev: *mut mt76_dev, offset: u32, val: u32) {
    static void mt76_mmio_wr(struct mt76_dev *dev, u32 offset, u32 val)
    {
    trace_reg_wr(dev, offset, val);
    writel(val, dev.mmio.regs + offset);
    }
#[no_mangle]
unsafe extern "C" fn mt76_mmio_rmw(dev: *mut mt76_dev, offset: u32, mask: u32, val: u32) -> u32 {
    static u32 mt76_mmio_rmw(struct mt76_dev *dev, u32 offset, u32 mask, u32 val)
    {
    val |= mt76_mmio_rr(dev, offset) & ~mask;
    mt76_mmio_wr(dev, offset, val);
    return val;
    }
    static void mt76_mmio_write_copy(struct mt76_dev *dev, u32 offset,
    const void *data, int len)
    {
    int i;
    for (i = 0; i + 4 <= len; i += 4)
    writel(get_unaligned_le32(data + i),
    dev.mmio.regs + offset + i);
    if (i < len) {
    u8 tmp[4] = {};
    memcpy(tmp, data + i, len - i);
    writel(get_unaligned_le32(tmp), dev.mmio.regs + offset + i);
    }
    }
    static void mt76_mmio_read_copy(struct mt76_dev *dev, u32 offset,
    void *data, int len)
    {
    int i;
    for (i = 0; i + 4 <= len; i += 4)
    put_unaligned_le32(readl(dev.mmio.regs + offset + i),
    data + i);
    if (i < len) {
    u8 tmp[4];
    put_unaligned_le32(readl(dev.mmio.regs + offset + i), tmp);
    memcpy(data + i, tmp, len - i);
    }
    }
    static int mt76_mmio_wr_rp(struct mt76_dev *dev, u32 base,
    const struct mt76_reg_pair *data, int len)
    {
    while (len > 0) {
    mt76_mmio_wr(dev, data.reg, data.value);
    data++;
    len--;
    }
    return 0;
    }
    static int mt76_mmio_rd_rp(struct mt76_dev *dev, u32 base,
    struct mt76_reg_pair *data, int len)
    {
    while (len > 0) {
    data.value = mt76_mmio_rr(dev, data.reg);
    data++;
    len--;
    }
    return 0;
    }
    void mt76_set_irq_mask(struct mt76_dev *dev, u32 addr,
    u32 clear, u32 set)
    {
    unsigned long flags;
    spin_lock_irqsave(&dev.mmio.irq_lock, flags);
    dev.mmio.irqmask &= ~clear;
    dev.mmio.irqmask |= set;
    if (addr) {
    if (mtk_wed_device_active(&dev.mmio.wed))
    mtk_wed_device_irq_set_mask(&dev.mmio.wed,
    dev.mmio.irqmask);
    else
    mt76_mmio_wr(dev, addr, dev.mmio.irqmask);
    }
    spin_unlock_irqrestore(&dev.mmio.irq_lock, flags);
    }
    EXPORT_SYMBOL_GPL(mt76_set_irq_mask);
#[no_mangle]
pub unsafe extern "C" fn mt76_mmio_init(dev: *mut mt76_dev, regs: *mut void __iomem) {
    void mt76_mmio_init(struct mt76_dev *dev, void __iomem *regs)
    {
    static const struct mt76_bus_ops mt76_mmio_ops = {
    .rr = mt76_mmio_rr,
    .rmw = mt76_mmio_rmw,
    .wr = mt76_mmio_wr,
    .write_copy = mt76_mmio_write_copy,
    .read_copy = mt76_mmio_read_copy,
    .wr_rp = mt76_mmio_wr_rp,
    .rd_rp = mt76_mmio_rd_rp,
    .type = MT76_BUS_MMIO,
    };
    dev.bus = &mt76_mmio_ops;
    dev.mmio.regs = regs;
    spin_lock_init(&dev.mmio.irq_lock);
    }
    EXPORT_SYMBOL_GPL(mt76_mmio_init);
