//! Automatically rewritten from C to Rust
//! Source: drivers/irqchip/irq-gic-realview.c
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
// Special GIC quirks for the ARM RealView
// Copyright (C) 2015 Linus Walleij
//

pub const REALVIEW_SYS_LOCK_OFFSET: c_uint = 0x20;
pub const REALVIEW_SYS_PLD_CTRL1: c_uint = 0x74;
pub const REALVIEW_EB_REVB_SYS_PLD_CTRL1: c_uint = 0xD8;
pub const VERSATILE_LOCK_VAL: c_uint = 0xA05F;

pub const PLD_INTMODE_LEGACY: c_uint = 0x0;

// For some reason RealView EB Rev B moved this register
    static const struct of_device_id syscon_pldset_of_match[] = {
    {
    .compatible = "arm,realview-eb11mp-revb-syscon",
    .data = (void *)REALVIEW_EB_REVB_SYS_PLD_CTRL1,
    },
    {
    .compatible = "arm,realview-eb11mp-revc-syscon",
    .data = (void *)REALVIEW_SYS_PLD_CTRL1,
    },
    {
    .compatible = "arm,realview-eb-syscon",
    .data = (void *)REALVIEW_SYS_PLD_CTRL1,
    },
    {
    .compatible = "arm,realview-pb11mp-syscon",
    .data = (void *)REALVIEW_SYS_PLD_CTRL1,
    },
    {},
    };
    static int __init
    realview_gic_of_init(struct device_node *node, struct device_node *parent)
    {
    struct regmap *map;
    struct device_node *np;
    const struct of_device_id *gic_id;
    u32 pld1_ctrl;
    np = of_find_matching_node_and_match(core::ptr::null_mut(), syscon_pldset_of_match,
    &gic_id);
    if (!np)
    return -ENODEV;
    pld1_ctrl = (u32)gic_id.data;
// The PB11MPCore GIC needs to be configured in the syscon
    map = syscon_node_to_regmap(np);
    of_node_put(np);
    if (!IS_ERR(map)) {
// new irq mode with no DCC
    regmap_write(map, REALVIEW_SYS_LOCK_OFFSET,
    VERSATILE_LOCK_VAL);
    regmap_update_bits(map, pld1_ctrl,
    PLD_INTMODE_NEW_NO_DCC,
    PLD_INTMODE_MASK);
    regmap_write(map, REALVIEW_SYS_LOCK_OFFSET, 0x0000);
    pr_info("RealView GIC: set up interrupt controller to NEW mode, no DCC\n");
    } else {
    pr_err("RealView GIC setup: could not find syscon\n");
    return -ENODEV;
    }
    return gic_of_init(node, parent);
    }
    IRQCHIP_DECLARE(armtc11mp_gic, "arm,tc11mp-gic", realview_gic_of_init);
    IRQCHIP_DECLARE(armeb11mp_gic, "arm,eb11mp-gic", realview_gic_of_init);
