//! Automatically rewritten from C to Rust
//! Source: drivers/power/reset/ocelot-reset.c
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


// SPDX-License-Identifier: (GPL-2.0 OR MIT)
//
// Microsemi MIPS SoC reset driver
//
// License: Dual MIT/GPL
// Copyright (c) 2017 Microsemi Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reset_props {
    pub syscon: *const c_char,
    pub protect_reg: u32,
    pub vcore_protect: u32,
    pub if_si_owner_bit: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocelot_reset_context {
    pub base: *mut void __iomem,
    pub cpu_ctrl: *mut regmap,
    pub props: *const reset_props,
    pub restart_handler: notifier_block,
}

pub const BIT_OFF_INVALID: c_int = 32;

pub const ICPU_CFG_CPU_SYSTEM_CTRL_GENERAL_CTRL: c_uint = 0x24;

pub const IF_SI_OWNER_SISL: c_int = 0;
pub const IF_SI_OWNER_SIBM: c_int = 1;
pub const IF_SI_OWNER_SIMC: c_int = 2;
    static int ocelot_restart_handle(struct notifier_block *this,
    unsigned long mode, void *cmd)
    {
    struct ocelot_reset_context *ctx = container_of(this, struct
    ocelot_reset_context,
    restart_handler);
    let mut if_si_owner_bit: u32 = ctx.props.if_si_owner_bit;
// Make sure the core is not protected from reset
    regmap_update_bits(ctx.cpu_ctrl, ctx.props.protect_reg,
    ctx.props.vcore_protect, 0);
// Make the SI back to boot mode
    if (if_si_owner_bit != BIT_OFF_INVALID)
    regmap_update_bits(ctx.cpu_ctrl,
    ICPU_CFG_CPU_SYSTEM_CTRL_GENERAL_CTRL,
    IF_SI_OWNER_MASK << if_si_owner_bit,
    IF_SI_OWNER_SIBM << if_si_owner_bit);
    pr_emerg("Resetting SoC\n");
    writel(SOFT_CHIP_RST, ctx.base);
    pr_emerg("Unable to restart system\n");
    return NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn ocelot_reset_probe(pdev: *mut platform_device) -> c_int {
    static int ocelot_reset_probe(struct platform_device *pdev)
    {
    struct ocelot_reset_context *ctx;
    struct device *dev = &pdev.dev;
    int err;
    ctx = devm_kzalloc(&pdev.dev, sizeof(*ctx), GFP_KERNEL);
    if (!ctx)
    return -ENOMEM;
    ctx.base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(ctx.base))
    return PTR_ERR(ctx.base);
    ctx.props = device_get_match_data(dev);
    ctx.cpu_ctrl = syscon_regmap_lookup_by_compatible(ctx.props.syscon);
    if (IS_ERR(ctx.cpu_ctrl)) {
    dev_err(dev, "No syscon map: %s\n", ctx.props.syscon);
    return PTR_ERR(ctx.cpu_ctrl);
    }
    ctx.restart_handler.notifier_call = ocelot_restart_handle;
    ctx.restart_handler.priority = 192;
    err = register_restart_handler(&ctx.restart_handler);
    if (err)
    dev_err(dev, "can't register restart notifier (err=%d)\n", err);
    return err;
    }
    static const struct reset_props reset_props_jaguar2 = {
    .syscon		 = "mscc,ocelot-cpu-syscon",
    .protect_reg     = 0x20,
    .vcore_protect   = BIT(2),
    .if_si_owner_bit = 6,
    };
    static const struct reset_props reset_props_luton = {
    .syscon		 = "mscc,ocelot-cpu-syscon",
    .protect_reg     = 0x20,
    .vcore_protect   = BIT(2),
    .if_si_owner_bit = BIT_OFF_INVALID, /* n/a */
    };
    static const struct reset_props reset_props_ocelot = {
    .syscon		 = "mscc,ocelot-cpu-syscon",
    .protect_reg     = 0x20,
    .vcore_protect   = BIT(2),
    .if_si_owner_bit = 4,
    };
    static const struct reset_props reset_props_sparx5 = {
    .syscon		 = "microchip,sparx5-cpu-syscon",
    .protect_reg     = 0x84,
    .vcore_protect   = BIT(10),
    .if_si_owner_bit = 6,
    };
    static const struct of_device_id ocelot_reset_of_match[] = {
    {
    .compatible = "mscc,jaguar2-chip-reset",
    .data = &reset_props_jaguar2
    }, {
    .compatible = "mscc,luton-chip-reset",
    .data = &reset_props_luton
    }, {
    .compatible = "mscc,ocelot-chip-reset",
    .data = &reset_props_ocelot
    }, {
    .compatible = "microchip,sparx5-chip-reset",
    .data = &reset_props_sparx5
    },
    { /*sentinel*/ }
    };
    static struct platform_driver ocelot_reset_driver = {
    .probe = ocelot_reset_probe,
    .driver = {
    .name = "ocelot-chip-reset",
    .of_match_table = ocelot_reset_of_match,
    },
    };
    builtin_platform_driver(ocelot_reset_driver);
