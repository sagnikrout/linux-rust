//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/mediatek/mt76/mt7603/core.c
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

#[no_mangle]
pub unsafe extern "C" fn mt7603_rx_poll_complete(mdev: *mut mt76_dev, q: enum mt76_rxq_id) {
    void mt7603_rx_poll_complete(struct mt76_dev *mdev, enum mt76_rxq_id q)
    {
    struct mt7603_dev *dev = container_of(mdev, struct mt7603_dev, mt76);
    mt7603_irq_enable(dev, MT_INT_RX_DONE(q));
    }
#[no_mangle]
pub unsafe extern "C" fn mt7603_irq_handler(irq: c_int, dev_instance: *mut c_void) -> irqreturn_t {
    irqreturn_t mt7603_irq_handler(int irq, void *dev_instance)
    {
    struct mt7603_dev *dev = dev_instance;
    u32 intr;
    intr = mt76_rr(dev, MT_INT_SOURCE_CSR);
    mt76_wr(dev, MT_INT_SOURCE_CSR, intr);
    if (!test_bit(MT76_STATE_INITIALIZED, &dev.mphy.state))
    return IRQ_NONE;
    trace_dev_irq(&dev.mt76, intr, dev.mt76.mmio.irqmask);
    intr &= dev.mt76.mmio.irqmask;
    if (intr & MT_INT_MAC_IRQ3) {
    let mut hwintr: u32 = mt76_rr(dev, MT_HW_INT_STATUS(3));
    mt76_wr(dev, MT_HW_INT_STATUS(3), hwintr);
    if (hwintr & MT_HW_INT3_PRE_TBTT0)
    tasklet_schedule(&dev.mt76.pre_tbtt_tasklet);
    if ((hwintr & MT_HW_INT3_TBTT0) && dev.mt76.csa_complete)
    mt76_csa_finish(&dev.mt76);
    }
    if (intr & MT_INT_TX_DONE_ALL) {
    mt7603_irq_disable(dev, MT_INT_TX_DONE_ALL);
    napi_schedule(&dev.mt76.tx_napi);
    }
    if (intr & MT_INT_RX_DONE(0)) {
    dev.rx_pse_check = 0;
    mt7603_irq_disable(dev, MT_INT_RX_DONE(0));
    napi_schedule(&dev.mt76.napi[0]);
    }
    if (intr & MT_INT_RX_DONE(1)) {
    dev.rx_pse_check = 0;
    mt7603_irq_disable(dev, MT_INT_RX_DONE(1));
    napi_schedule(&dev.mt76.napi[1]);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
pub unsafe extern "C" fn mt7603_reg_map(dev: *mut mt7603_dev, addr: u32) -> u32 {
    u32 mt7603_reg_map(struct mt7603_dev *dev, u32 addr)
    {
    let mut base: u32 = addr & MT_MCU_PCIE_REMAP_2_BASE;
    let mut offset: u32 = addr & MT_MCU_PCIE_REMAP_2_OFFSET;
    dev.bus_ops.wr(&dev.mt76, MT_MCU_PCIE_REMAP_2, base);
    return MT_PCIE_REMAP_BASE_2 + offset;
    }
