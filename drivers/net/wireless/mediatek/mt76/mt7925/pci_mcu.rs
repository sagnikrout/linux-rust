//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/mediatek/mt76/mt7925/pci_mcu.c
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
// Copyright (C) 2023 MediaTek Inc.

    static int
    mt7925_mcu_send_message(struct mt76_dev *mdev, struct sk_buff *skb,
    int cmd, int *seq)
    {
    struct mt792x_dev *dev = container_of(mdev, struct mt792x_dev, mt76);
    let mut txq: enum mt76_mcuq_id = MT_MCUQ_WM;
    int ret;
    ret = mt7925_mcu_fill_message(mdev, skb, cmd, seq);
    if (ret)
    return ret;
    mdev.mcu.timeout = 3 * HZ;
    if (cmd == MCU_CMD(FW_SCATTER))
    txq = MT_MCUQ_FWDL;
    return mt76_tx_queue_skb_raw(dev, mdev.q_mcu[txq], skb, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn mt7925e_mcu_init(dev: *mut mt792x_dev) -> c_int {
    int mt7925e_mcu_init(struct mt792x_dev *dev)
    {
    static const struct mt76_mcu_ops mt7925_mcu_ops = {
    .headroom = sizeof(struct mt76_connac2_mcu_txd),
    .mcu_skb_send_msg = mt7925_mcu_send_message,
    .mcu_parse_response = mt7925_mcu_parse_response,
    };
    int err;
    dev.mt76.mcu_ops = &mt7925_mcu_ops;
    err = mt792xe_mcu_fw_pmctrl(dev);
    if (err)
    return err;
    err = __mt792xe_mcu_drv_pmctrl(dev);
    if (err)
    return err;
    mt76_rmw_field(dev, dev.pcie_reg.pm, MT_PCIE_MAC_PM_L0S_DIS, 1);
    err = mt7925_run_firmware(dev);
    mt76_queue_tx_cleanup(dev, dev.mt76.q_mcu[MT_MCUQ_FWDL], false);
    return err;
    }
