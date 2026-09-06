//! Automatically rewritten from C to Rust
//! Source: drivers/media/cec/platform/s5p/exynos_hdmi_cecctrl.c
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
// drivers/media/platform/s5p-cec/exynos_hdmi_cecctrl.c
//
// Copyright (c) 2009, 2014 Samsung Electronics
// http://www.samsung.com
//
// cec ftn file for Samsung TVOUT driver
//

pub const S5P_HDMI_FIN: c_int = 24000000;
pub const CEC_DIV_RATIO: c_int = 320000;
pub const CEC_MESSAGE_BROADCAST_MASK: c_uint = 0x0F;
pub const CEC_MESSAGE_BROADCAST: c_uint = 0x0F;
pub const CEC_FILTER_THRESHOLD: c_uint = 0x15;
#[no_mangle]
pub unsafe extern "C" fn s5p_cec_set_divider(cec: *mut s5p_cec_dev) {
    void s5p_cec_set_divider(struct s5p_cec_dev *cec)
    {
    u32 div_ratio, div_val;
    unsigned int reg;
    div_ratio  = S5P_HDMI_FIN / CEC_DIV_RATIO - 1;
    if (regmap_read(cec.pmu, EXYNOS_HDMI_PHY_CONTROL, &reg)) {
    dev_err(cec.dev, "failed to read phy control\n");
    return;
    }
    reg = (reg & ~(0x3FF << 16)) | (div_ratio << 16);
    if (regmap_write(cec.pmu, EXYNOS_HDMI_PHY_CONTROL, reg)) {
    dev_err(cec.dev, "failed to write phy control\n");
    return;
    }
    div_val = CEC_DIV_RATIO * 0.00005 - 1;
    writeb(0x0, cec.reg + S5P_CEC_DIVISOR_3);
    writeb(0x0, cec.reg + S5P_CEC_DIVISOR_2);
    writeb(0x0, cec.reg + S5P_CEC_DIVISOR_1);
    writeb(div_val, cec.reg + S5P_CEC_DIVISOR_0);
    }
#[no_mangle]
pub unsafe extern "C" fn s5p_cec_enable_rx(cec: *mut s5p_cec_dev) {
    void s5p_cec_enable_rx(struct s5p_cec_dev *cec)
    {
    u8 reg;
    reg = readb(cec.reg + S5P_CEC_RX_CTRL);
    reg |= S5P_CEC_RX_CTRL_ENABLE;
    writeb(reg, cec.reg + S5P_CEC_RX_CTRL);
    }
#[no_mangle]
pub unsafe extern "C" fn s5p_cec_mask_rx_interrupts(cec: *mut s5p_cec_dev) {
    void s5p_cec_mask_rx_interrupts(struct s5p_cec_dev *cec)
    {
    u8 reg;
    reg = readb(cec.reg + S5P_CEC_IRQ_MASK);
    reg |= S5P_CEC_IRQ_RX_DONE;
    reg |= S5P_CEC_IRQ_RX_ERROR;
    writeb(reg, cec.reg + S5P_CEC_IRQ_MASK);
    }
#[no_mangle]
pub unsafe extern "C" fn s5p_cec_unmask_rx_interrupts(cec: *mut s5p_cec_dev) {
    void s5p_cec_unmask_rx_interrupts(struct s5p_cec_dev *cec)
    {
    u8 reg;
    reg = readb(cec.reg + S5P_CEC_IRQ_MASK);
    reg &= ~S5P_CEC_IRQ_RX_DONE;
    reg &= ~S5P_CEC_IRQ_RX_ERROR;
    writeb(reg, cec.reg + S5P_CEC_IRQ_MASK);
    }
#[no_mangle]
pub unsafe extern "C" fn s5p_cec_mask_tx_interrupts(cec: *mut s5p_cec_dev) {
    void s5p_cec_mask_tx_interrupts(struct s5p_cec_dev *cec)
    {
    u8 reg;
    reg = readb(cec.reg + S5P_CEC_IRQ_MASK);
    reg |= S5P_CEC_IRQ_TX_DONE;
    reg |= S5P_CEC_IRQ_TX_ERROR;
    writeb(reg, cec.reg + S5P_CEC_IRQ_MASK);
    }
#[no_mangle]
pub unsafe extern "C" fn s5p_cec_unmask_tx_interrupts(cec: *mut s5p_cec_dev) {
    void s5p_cec_unmask_tx_interrupts(struct s5p_cec_dev *cec)
    {
    u8 reg;
    reg = readb(cec.reg + S5P_CEC_IRQ_MASK);
    reg &= ~S5P_CEC_IRQ_TX_DONE;
    reg &= ~S5P_CEC_IRQ_TX_ERROR;
    writeb(reg, cec.reg + S5P_CEC_IRQ_MASK);
    }
#[no_mangle]
pub unsafe extern "C" fn s5p_cec_reset(cec: *mut s5p_cec_dev) {
    void s5p_cec_reset(struct s5p_cec_dev *cec)
    {
    u8 reg;
    writeb(S5P_CEC_RX_CTRL_RESET, cec.reg + S5P_CEC_RX_CTRL);
    writeb(S5P_CEC_TX_CTRL_RESET, cec.reg + S5P_CEC_TX_CTRL);
    reg = readb(cec.reg + 0xc4);
    reg &= ~0x1;
    writeb(reg, cec.reg + 0xc4);
    }
#[no_mangle]
pub unsafe extern "C" fn s5p_cec_tx_reset(cec: *mut s5p_cec_dev) {
    void s5p_cec_tx_reset(struct s5p_cec_dev *cec)
    {
    writeb(S5P_CEC_TX_CTRL_RESET, cec.reg + S5P_CEC_TX_CTRL);
    }
#[no_mangle]
pub unsafe extern "C" fn s5p_cec_rx_reset(cec: *mut s5p_cec_dev) {
    void s5p_cec_rx_reset(struct s5p_cec_dev *cec)
    {
    u8 reg;
    writeb(S5P_CEC_RX_CTRL_RESET, cec.reg + S5P_CEC_RX_CTRL);
    reg = readb(cec.reg + 0xc4);
    reg &= ~0x1;
    writeb(reg, cec.reg + 0xc4);
    }
#[no_mangle]
pub unsafe extern "C" fn s5p_cec_threshold(cec: *mut s5p_cec_dev) {
    void s5p_cec_threshold(struct s5p_cec_dev *cec)
    {
    writeb(CEC_FILTER_THRESHOLD, cec.reg + S5P_CEC_RX_FILTER_TH);
    writeb(0, cec.reg + S5P_CEC_RX_FILTER_CTRL);
    }
    void s5p_cec_copy_packet(struct s5p_cec_dev *cec, char *data,
    size_t count, u8 retries)
    {
    let mut i: c_int = 0;
    u8 reg;
    while (i < count) {
    writeb(data[i], cec.reg + (S5P_CEC_TX_BUFF0 + (i * 4)));
    i++;
    }
    writeb(count, cec.reg + S5P_CEC_TX_BYTES);
    reg = readb(cec.reg + S5P_CEC_TX_CTRL);
    reg |= S5P_CEC_TX_CTRL_START;
    reg &= ~0x70;
    reg |= retries << 4;
    if ((data[0] & CEC_MESSAGE_BROADCAST_MASK) == CEC_MESSAGE_BROADCAST) {
    dev_dbg(cec.dev, "Broadcast");
    reg |= S5P_CEC_TX_CTRL_BCAST;
    } else {
    dev_dbg(cec.dev, "No Broadcast");
    reg &= ~S5P_CEC_TX_CTRL_BCAST;
    }
    writeb(reg, cec.reg + S5P_CEC_TX_CTRL);
    dev_dbg(cec.dev, "cec-tx: cec count (%zu): %*ph", count,
    (int)count, data);
    }
#[no_mangle]
pub unsafe extern "C" fn s5p_cec_set_addr(cec: *mut s5p_cec_dev, addr: u32) {
    void s5p_cec_set_addr(struct s5p_cec_dev *cec, u32 addr)
    {
    writeb(addr & 0x0F, cec.reg + S5P_CEC_LOGIC_ADDR);
    }
#[no_mangle]
pub unsafe extern "C" fn s5p_cec_get_status(cec: *mut s5p_cec_dev) -> u32 {
    u32 s5p_cec_get_status(struct s5p_cec_dev *cec)
    {
    let mut status: u32 = 0;
    status = readb(cec.reg + S5P_CEC_STATUS_0) & 0xf;
    status |= (readb(cec.reg + S5P_CEC_TX_STAT1) & 0xf) << 4;
    status |= readb(cec.reg + S5P_CEC_STATUS_1) << 8;
    status |= readb(cec.reg + S5P_CEC_STATUS_2) << 16;
    status |= readb(cec.reg + S5P_CEC_STATUS_3) << 24;
    dev_dbg(cec.dev, "status = 0x%x!\n", status);
    return status;
    }
#[no_mangle]
pub unsafe extern "C" fn s5p_clr_pending_tx(cec: *mut s5p_cec_dev) {
    void s5p_clr_pending_tx(struct s5p_cec_dev *cec)
    {
    writeb(S5P_CEC_IRQ_TX_DONE | S5P_CEC_IRQ_TX_ERROR,
    cec.reg + S5P_CEC_IRQ_CLEAR);
    }
#[no_mangle]
pub unsafe extern "C" fn s5p_clr_pending_rx(cec: *mut s5p_cec_dev) {
    void s5p_clr_pending_rx(struct s5p_cec_dev *cec)
    {
    writeb(S5P_CEC_IRQ_RX_DONE | S5P_CEC_IRQ_RX_ERROR,
    cec.reg + S5P_CEC_IRQ_CLEAR);
    }
#[no_mangle]
pub unsafe extern "C" fn s5p_cec_get_rx_buf(cec: *mut s5p_cec_dev, size: u32, buffer: *mut u8) {
    void s5p_cec_get_rx_buf(struct s5p_cec_dev *cec, u32 size, u8 *buffer)
    {
    let mut i: u32 = 0;
    char debug[40];
    while (i < size) {
    buffer[i] = readb(cec.reg + S5P_CEC_RX_BUFF0 + (i * 4));
    sprintf(debug + i * 2, "%02x ", buffer[i]);
    i++;
    }
    dev_dbg(cec.dev, "cec-rx: cec size(%d): %s", size, debug);
    }
