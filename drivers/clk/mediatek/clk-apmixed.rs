//! Automatically rewritten from C to Rust
//! Source: drivers/clk/mediatek/clk-apmixed.c
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
// Copyright (c) 2015 MediaTek Inc.
// Author: James Liao <jamesjj.liao@mediatek.com>
//

    REF2USB_TX_OUT_EN)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_ref2usb_tx {
    pub hw: clk_hw,
    pub base_addr: *mut void __iomem,
}

    static inline struct mtk_ref2usb_tx *to_mtk_ref2usb_tx(struct clk_hw *hw)
    {
    return container_of(hw, struct mtk_ref2usb_tx, hw);
    }
#[no_mangle]
unsafe extern "C" fn mtk_ref2usb_tx_is_prepared(hw: *mut clk_hw) -> c_int {
    static int mtk_ref2usb_tx_is_prepared(struct clk_hw *hw)
    {
    struct mtk_ref2usb_tx *tx = to_mtk_ref2usb_tx(hw);
    return (readl(tx.base_addr) & REF2USB_EN_MASK) == REF2USB_EN_MASK;
    }
#[no_mangle]
unsafe extern "C" fn mtk_ref2usb_tx_prepare(hw: *mut clk_hw) -> c_int {
    static int mtk_ref2usb_tx_prepare(struct clk_hw *hw)
    {
    struct mtk_ref2usb_tx *tx = to_mtk_ref2usb_tx(hw);
    u32 val;
    val = readl(tx.base_addr);
    val |= REF2USB_TX_EN;
    writel(val, tx.base_addr);
    udelay(100);
    val |= REF2USB_TX_LPF_EN;
    writel(val, tx.base_addr);
    val |= REF2USB_TX_OUT_EN;
    writel(val, tx.base_addr);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mtk_ref2usb_tx_unprepare(hw: *mut clk_hw) {
    static void mtk_ref2usb_tx_unprepare(struct clk_hw *hw)
    {
    struct mtk_ref2usb_tx *tx = to_mtk_ref2usb_tx(hw);
    u32 val;
    val = readl(tx.base_addr);
    val &= ~REF2USB_EN_MASK;
    writel(val, tx.base_addr);
    }
    static const struct clk_ops mtk_ref2usb_tx_ops = {
    .is_prepared	= mtk_ref2usb_tx_is_prepared,
    .prepare	= mtk_ref2usb_tx_prepare,
    .unprepare	= mtk_ref2usb_tx_unprepare,
    };
    struct clk_hw *mtk_clk_register_ref2usb_tx(const char *name,
    const char *parent_name, void __iomem *reg)
    {
    struct mtk_ref2usb_tx *tx;
    let mut init: clk_init_data = {};
    int ret;
    tx = kzalloc_obj(*tx);
    if (!tx)
    return ERR_PTR(-ENOMEM);
    tx.base_addr = reg;
    tx.hw.init = &init;
    init.name = name;
    init.ops = &mtk_ref2usb_tx_ops;
    init.parent_names = &parent_name;
    init.num_parents = 1;
    ret = clk_hw_register(core::ptr::null_mut(), &tx.hw);
    if (ret) {
    kfree(tx);
    return ERR_PTR(ret);
    }
    return &tx.hw;
    }
    EXPORT_SYMBOL_GPL(mtk_clk_register_ref2usb_tx);
#[no_mangle]
pub unsafe extern "C" fn mtk_clk_unregister_ref2usb_tx(hw: *mut clk_hw) {
    void mtk_clk_unregister_ref2usb_tx(struct clk_hw *hw)
    {
    struct mtk_ref2usb_tx *tx = to_mtk_ref2usb_tx(hw);
    clk_hw_unregister(hw);
    kfree(tx);
    }
    EXPORT_SYMBOL_GPL(mtk_clk_unregister_ref2usb_tx);
    MODULE_LICENSE("GPL");
