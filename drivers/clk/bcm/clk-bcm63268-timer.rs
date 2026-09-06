//! Automatically rewritten from C to Rust
//! Source: drivers/clk/bcm/clk-bcm63268-timer.c
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


// SPDX-License-Identifier: GPL-2.0
//
// BCM63268 Timer Clock and Reset Controller Driver
//
// Copyright (C) 2023 Álvaro Fernández Rojas <noltari@gmail.com>
//

pub const BCM63268_TIMER_RESET_SLEEP_MIN_US: c_int = 10000;
pub const BCM63268_TIMER_RESET_SLEEP_MAX_US: c_int = 20000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm63268_tclkrst_hw {
    pub regs: *mut void __iomem,
    pub lock: spinlock_t,
    pub rcdev: reset_controller_dev,
    pub data: clk_hw_onecell_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm63268_tclk_table_entry {
    pub name: *const *const c_char,
    pub bit: u8,
}

    static const struct bcm63268_tclk_table_entry bcm63268_timer_clocks[] = {
    {
    .name = "ephy1",
    .bit = BCM63268_TCLK_EPHY1,
    }, {
    .name = "ephy2",
    .bit = BCM63268_TCLK_EPHY2,
    }, {
    .name = "ephy3",
    .bit = BCM63268_TCLK_EPHY3,
    }, {
    .name = "gphy1",
    .bit = BCM63268_TCLK_GPHY1,
    }, {
    .name = "dsl",
    .bit = BCM63268_TCLK_DSL,
    }, {
    .name = "wakeon_ephy",
    .bit = BCM63268_TCLK_WAKEON_EPHY,
    }, {
    .name = "wakeon_dsl",
    .bit = BCM63268_TCLK_WAKEON_DSL,
    }, {
    .name = "fap1_pll",
    .bit = BCM63268_TCLK_FAP1,
    }, {
    .name = "fap2_pll",
    .bit = BCM63268_TCLK_FAP2,
    }, {
    .name = "uto_50",
    .bit = BCM63268_TCLK_UTO_50,
    }, {
    .name = "uto_extin",
    .bit = BCM63268_TCLK_UTO_EXTIN,
    }, {
    .name = "usb_ref",
    .bit = BCM63268_TCLK_USB_REF,
    }, {
// sentinel
    }
    };
    static inline struct bcm63268_tclkrst_hw *
    to_bcm63268_timer_reset(struct reset_controller_dev *rcdev)
    {
    return container_of(rcdev, struct bcm63268_tclkrst_hw, rcdev);
    }
    static int bcm63268_timer_reset_update(struct reset_controller_dev *rcdev,
    unsigned long id, bool assert)
    {
    struct bcm63268_tclkrst_hw *reset = to_bcm63268_timer_reset(rcdev);
    unsigned long flags;
    uint32_t val;
    spin_lock_irqsave(&reset.lock, flags);
    val = __raw_readl(reset.regs);
    if (assert)
    val &= ~BIT(id);
    else
    val |= BIT(id);
    __raw_writel(val, reset.regs);
    spin_unlock_irqrestore(&reset.lock, flags);
    return 0;
    }
    static int bcm63268_timer_reset_assert(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    return bcm63268_timer_reset_update(rcdev, id, true);
    }
    static int bcm63268_timer_reset_deassert(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    return bcm63268_timer_reset_update(rcdev, id, false);
    }
    static int bcm63268_timer_reset_reset(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    bcm63268_timer_reset_update(rcdev, id, true);
    usleep_range(BCM63268_TIMER_RESET_SLEEP_MIN_US,
    BCM63268_TIMER_RESET_SLEEP_MAX_US);
    bcm63268_timer_reset_update(rcdev, id, false);
//
// Ensure component is taken out reset state by sleeping also after
// deasserting the reset. Otherwise, the component may not be ready
// for operation.
//
    usleep_range(BCM63268_TIMER_RESET_SLEEP_MIN_US,
    BCM63268_TIMER_RESET_SLEEP_MAX_US);
    return 0;
    }
    static int bcm63268_timer_reset_status(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    struct bcm63268_tclkrst_hw *reset = to_bcm63268_timer_reset(rcdev);
    return !(__raw_readl(reset.regs) & BIT(id));
    }
    static const struct reset_control_ops bcm63268_timer_reset_ops = {
    .assert = bcm63268_timer_reset_assert,
    .deassert = bcm63268_timer_reset_deassert,
    .reset = bcm63268_timer_reset_reset,
    .status = bcm63268_timer_reset_status,
    };
#[no_mangle]
unsafe extern "C" fn bcm63268_tclk_probe(pdev: *mut platform_device) -> c_int {
    static int bcm63268_tclk_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    const struct bcm63268_tclk_table_entry *entry;
    struct bcm63268_tclkrst_hw *hw;
    struct clk_hw *clk;
    let mut maxbit: u8 = 0;
    int i, ret;
    for (entry = bcm63268_timer_clocks; entry.name; entry++)
    maxbit = max(maxbit, entry.bit);
    maxbit++;
    hw = devm_kzalloc(&pdev.dev, struct_size(hw, data.hws, maxbit),
    GFP_KERNEL);
    if (!hw)
    return -ENOMEM;
    platform_set_drvdata(pdev, hw);
    spin_lock_init(&hw.lock);
    hw.data.num = maxbit;
    for (i = 0; i < maxbit; i++)
    hw.data.hws[i] = ERR_PTR(-ENODEV);
    hw.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(hw.regs))
    return PTR_ERR(hw.regs);
    for (entry = bcm63268_timer_clocks; entry.name; entry++) {
    clk = devm_clk_hw_register_gate(dev, entry.name, core::ptr::null_mut(), 0,
    hw.regs, entry.bit,
    CLK_GATE_BIG_ENDIAN,
    &hw.lock);
    if (IS_ERR(clk))
    return PTR_ERR(clk);
    hw.data.hws[entry.bit] = clk;
    }
    ret = devm_of_clk_add_hw_provider(dev, of_clk_hw_onecell_get,
    &hw.data);
    if (ret)
    return ret;
    hw.rcdev.of_node = dev.of_node;
    hw.rcdev.ops = &bcm63268_timer_reset_ops;
    ret = devm_reset_controller_register(dev, &hw.rcdev);
    if (ret)
    dev_err(dev, "Failed to register reset controller\n");
    return 0;
    }
    static const struct of_device_id bcm63268_tclk_dt_ids[] = {
    { .compatible = "brcm,bcm63268-timer-clocks" },
    { /* sentinel */ }
    };
    static struct platform_driver bcm63268_tclk = {
    .probe = bcm63268_tclk_probe,
    .driver = {
    .name = "bcm63268-timer-clock",
    .of_match_table = bcm63268_tclk_dt_ids,
    },
    };
    builtin_platform_driver(bcm63268_tclk);
