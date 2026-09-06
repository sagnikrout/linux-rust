//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/mediatek/mt76/mt7921/sdio_mcu.c
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
// Copyright (C) 2021 MediaTek Inc.

    static int
    mt7921s_mcu_send_message(struct mt76_dev *mdev, struct sk_buff *skb,
    int cmd, int *seq)
    {
    struct mt792x_dev *dev = container_of(mdev, struct mt792x_dev, mt76);
    let mut type: enum mt7921_sdio_pkt_type = MT7921_SDIO_CMD;
    let mut txq: enum mt76_mcuq_id = MT_MCUQ_WM;
    int ret, pad;
// We just return in case firmware assertion to avoid blocking the
// common workqueue to run, for example, the coredump work might be
// blocked by mt792x_mac_work that is excuting register access via sdio
// bus.
//
    if (dev.fw_assert)
    return -EBUSY;
    ret = mt76_connac2_mcu_fill_message(mdev, skb, cmd, seq);
    if (ret)
    return ret;
    mdev.mcu.timeout = 3 * HZ;
    if (cmd == MCU_CMD(FW_SCATTER))
    type = MT7921_SDIO_FWDL;
    mt792x_skb_add_usb_sdio_hdr(dev, skb, type);
    pad = round_up(skb.len, 4) - skb.len;
    __skb_put_zero(skb, pad);
    ret = mt76_tx_queue_skb_raw(dev, mdev.q_mcu[txq], skb, 0);
    if (ret)
    return ret;
    mt76_queue_kick(dev, mdev.q_mcu[txq]);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mt7921s_read_rm3r(dev: *mut mt792x_dev) -> u32 {
    static u32 mt7921s_read_rm3r(struct mt792x_dev *dev)
    {
    struct mt76_sdio *sdio = &dev.mt76.sdio;
    return sdio_readl(sdio.func, MCR_D2HRM3R, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn mt7921s_clear_rm3r_drv_own(dev: *mut mt792x_dev) -> u32 {
    static u32 mt7921s_clear_rm3r_drv_own(struct mt792x_dev *dev)
    {
    struct mt76_sdio *sdio = &dev.mt76.sdio;
    u32 val;
    val = sdio_readl(sdio.func, MCR_D2HRM3R, core::ptr::null_mut());
    if (val)
    sdio_writel(sdio.func, H2D_SW_INT_CLEAR_MAILBOX_ACK,
    MCR_WSICR, core::ptr::null_mut());
    return val;
    }
#[no_mangle]
pub unsafe extern "C" fn mt7921s_mcu_init(dev: *mut mt792x_dev) -> c_int {
    int mt7921s_mcu_init(struct mt792x_dev *dev)
    {
    static const struct mt76_mcu_ops mt7921s_mcu_ops = {
    .headroom = MT_SDIO_HDR_SIZE +
    sizeof(struct mt76_connac2_mcu_txd),
    .tailroom = MT_SDIO_TAIL_SIZE,
    .mcu_skb_send_msg = mt7921s_mcu_send_message,
    .mcu_parse_response = mt7921_mcu_parse_response,
    .mcu_rr = mt76_connac_mcu_reg_rr,
    .mcu_wr = mt76_connac_mcu_reg_wr,
    };
    int ret;
    mt7921s_mcu_drv_pmctrl(dev);
    dev.mt76.mcu_ops = &mt7921s_mcu_ops;
    ret = mt7921_run_firmware(dev);
    if (ret)
    return ret;
    set_bit(MT76_STATE_MCU_RUNNING, &dev.mphy.state);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mt7921s_mcu_drv_pmctrl(dev: *mut mt792x_dev) -> c_int {
    int mt7921s_mcu_drv_pmctrl(struct mt792x_dev *dev)
    {
    struct sdio_func *func = dev.mt76.sdio.func;
    struct mt76_phy *mphy = &dev.mt76.phy;
    struct mt76_connac_pm *pm = &dev.pm;
    u32 status;
    int err;
    sdio_claim_host(func);
    sdio_writel(func, WHLPCR_FW_OWN_REQ_CLR, MCR_WHLPCR, core::ptr::null_mut());
    err = readx_poll_timeout(mt76s_read_pcr, &dev.mt76, status,
    status & WHLPCR_IS_DRIVER_OWN, 2000, 1000000);
    if (!err && test_bit(MT76_STATE_MCU_RUNNING, &dev.mphy.state))
    err = readx_poll_timeout(mt7921s_read_rm3r, dev, status,
    status & D2HRM3R_IS_DRIVER_OWN,
    2000, 1000000);
    sdio_release_host(func);
    if (err < 0) {
    dev_err(dev.mt76.dev, "driver own failed\n");
    return -EIO;
    }
    clear_bit(MT76_STATE_PM, &mphy.state);
    pm.stats.last_wake_event = jiffies;
    pm.stats.doze_time += pm.stats.last_wake_event -
    pm.stats.last_doze_event;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mt7921s_mcu_fw_pmctrl(dev: *mut mt792x_dev) -> c_int {
    int mt7921s_mcu_fw_pmctrl(struct mt792x_dev *dev)
    {
    struct sdio_func *func = dev.mt76.sdio.func;
    struct mt76_phy *mphy = &dev.mt76.phy;
    struct mt76_connac_pm *pm = &dev.pm;
    u32 status;
    int err;
    sdio_claim_host(func);
    if (test_bit(MT76_STATE_MCU_RUNNING, &dev.mphy.state)) {
    err = readx_poll_timeout(mt7921s_clear_rm3r_drv_own,
    dev, status,
    !(status & D2HRM3R_IS_DRIVER_OWN),
    2000, 1000000);
    if (err < 0) {
    dev_err(dev.mt76.dev, "mailbox ACK not cleared\n");
    goto out;
    }
    }
    sdio_writel(func, WHLPCR_FW_OWN_REQ_SET, MCR_WHLPCR, core::ptr::null_mut());
    err = readx_poll_timeout(mt76s_read_pcr, &dev.mt76, status,
    !(status & WHLPCR_IS_DRIVER_OWN), 2000, 1000000);
    out:
    sdio_release_host(func);
    if (err < 0) {
    dev_err(dev.mt76.dev, "firmware own failed\n");
    clear_bit(MT76_STATE_PM, &mphy.state);
    return -EIO;
    }
    pm.stats.last_doze_event = jiffies;
    pm.stats.awake_time += pm.stats.last_doze_event -
    pm.stats.last_wake_event;
    return 0;
    }
