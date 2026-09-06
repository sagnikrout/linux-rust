//! Automatically rewritten from C to Rust
//! Source: drivers/reset/reset-pistachio.c
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
// Pistachio SoC Reset Controller driver
//
// Copyright (C) 2015 Imagination Technologies Ltd.
//
// Author: Damien Horsley <Damien.Horsley@imgtec.com>
//

pub const PISTACHIO_SOFT_RESET: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pistachio_reset_data {
    pub rcdev: reset_controller_dev,
    pub periph_regs: *mut regmap,
}

#[no_mangle]
pub unsafe extern "C" fn pistachio_reset_shift(id: c_ulong) -> c_int {
    static inline int pistachio_reset_shift(unsigned long id)
    {
    switch (id) {
    case PISTACHIO_RESET_I2C0:
    case PISTACHIO_RESET_I2C1:
    case PISTACHIO_RESET_I2C2:
    case PISTACHIO_RESET_I2C3:
    case PISTACHIO_RESET_I2S_IN:
    case PISTACHIO_RESET_PRL_OUT:
    case PISTACHIO_RESET_SPDIF_OUT:
    case PISTACHIO_RESET_SPI:
    case PISTACHIO_RESET_PWM_PDM:
    case PISTACHIO_RESET_UART0:
    case PISTACHIO_RESET_UART1:
    case PISTACHIO_RESET_QSPI:
    case PISTACHIO_RESET_MDC:
    case PISTACHIO_RESET_SDHOST:
    case PISTACHIO_RESET_ETHERNET:
    case PISTACHIO_RESET_IR:
    case PISTACHIO_RESET_HASH:
    case PISTACHIO_RESET_TIMER:
    return id;
    case PISTACHIO_RESET_I2S_OUT:
    case PISTACHIO_RESET_SPDIF_IN:
    case PISTACHIO_RESET_EVT:
    return id + 6;
    case PISTACHIO_RESET_USB_H:
    case PISTACHIO_RESET_USB_PR:
    case PISTACHIO_RESET_USB_PHY_PR:
    case PISTACHIO_RESET_USB_PHY_PON:
    return id + 7;
    default:
    return -EINVAL;
    }
    }
    static int pistachio_reset_assert(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    struct pistachio_reset_data *rd;
    u32 mask;
    int shift;
    rd = container_of(rcdev, struct pistachio_reset_data, rcdev);
    shift = pistachio_reset_shift(id);
    if (shift < 0)
    return shift;
    mask = BIT(shift);
    return regmap_update_bits(rd.periph_regs, PISTACHIO_SOFT_RESET,
    mask, mask);
    }
    static int pistachio_reset_deassert(struct reset_controller_dev *rcdev,
    unsigned long id)
    {
    struct pistachio_reset_data *rd;
    u32 mask;
    int shift;
    rd = container_of(rcdev, struct pistachio_reset_data, rcdev);
    shift = pistachio_reset_shift(id);
    if (shift < 0)
    return shift;
    mask = BIT(shift);
    return regmap_update_bits(rd.periph_regs, PISTACHIO_SOFT_RESET,
    mask, 0);
    }
    static const struct reset_control_ops pistachio_reset_ops = {
    .assert		= pistachio_reset_assert,
    .deassert	= pistachio_reset_deassert,
    };
#[no_mangle]
unsafe extern "C" fn pistachio_reset_probe(pdev: *mut platform_device) -> c_int {
    static int pistachio_reset_probe(struct platform_device *pdev)
    {
    struct pistachio_reset_data *rd;
    struct device *dev = &pdev.dev;
    struct device_node *np = pdev.dev.of_node;
    rd = devm_kzalloc(dev, sizeof(*rd), GFP_KERNEL);
    if (!rd)
    return -ENOMEM;
    rd.periph_regs = syscon_node_to_regmap(np.parent);
    if (IS_ERR(rd.periph_regs))
    return PTR_ERR(rd.periph_regs);
    rd.rcdev.owner = THIS_MODULE;
    rd.rcdev.nr_resets = PISTACHIO_RESET_MAX + 1;
    rd.rcdev.ops = &pistachio_reset_ops;
    rd.rcdev.of_node = np;
    return devm_reset_controller_register(dev, &rd.rcdev);
    }
    static const struct of_device_id pistachio_reset_dt_ids[] = {
    { .compatible = "img,pistachio-reset", },
    { /* sentinel */ },
    };
    static struct platform_driver pistachio_reset_driver = {
    .probe	= pistachio_reset_probe,
    .driver = {
    .name		= "pistachio-reset",
    .of_match_table	= pistachio_reset_dt_ids,
    },
    };
    builtin_platform_driver(pistachio_reset_driver);
