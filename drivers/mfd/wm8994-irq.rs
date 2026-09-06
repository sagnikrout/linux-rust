//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/wm8994-irq.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// wm8994-irq.c  --  Interrupt controller support for Wolfson WM8994
//
// Copyright 2010 Wolfson Microelectronics PLC.
//
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
//

    static const struct regmap_irq wm8994_irqs[] = {
    [WM8994_IRQ_TEMP_SHUT] = {
    .reg_offset = 1,
    .mask = WM8994_TEMP_SHUT_EINT,
    },
    [WM8994_IRQ_MIC1_DET] = {
    .reg_offset = 1,
    .mask = WM8994_MIC1_DET_EINT,
    },
    [WM8994_IRQ_MIC1_SHRT] = {
    .reg_offset = 1,
    .mask = WM8994_MIC1_SHRT_EINT,
    },
    [WM8994_IRQ_MIC2_DET] = {
    .reg_offset = 1,
    .mask = WM8994_MIC2_DET_EINT,
    },
    [WM8994_IRQ_MIC2_SHRT] = {
    .reg_offset = 1,
    .mask = WM8994_MIC2_SHRT_EINT,
    },
    [WM8994_IRQ_FLL1_LOCK] = {
    .reg_offset = 1,
    .mask = WM8994_FLL1_LOCK_EINT,
    },
    [WM8994_IRQ_FLL2_LOCK] = {
    .reg_offset = 1,
    .mask = WM8994_FLL2_LOCK_EINT,
    },
    [WM8994_IRQ_SRC1_LOCK] = {
    .reg_offset = 1,
    .mask = WM8994_SRC1_LOCK_EINT,
    },
    [WM8994_IRQ_SRC2_LOCK] = {
    .reg_offset = 1,
    .mask = WM8994_SRC2_LOCK_EINT,
    },
    [WM8994_IRQ_AIF1DRC1_SIG_DET] = {
    .reg_offset = 1,
    .mask = WM8994_AIF1DRC1_SIG_DET,
    },
    [WM8994_IRQ_AIF1DRC2_SIG_DET] = {
    .reg_offset = 1,
    .mask = WM8994_AIF1DRC2_SIG_DET_EINT,
    },
    [WM8994_IRQ_AIF2DRC_SIG_DET] = {
    .reg_offset = 1,
    .mask = WM8994_AIF2DRC_SIG_DET_EINT,
    },
    [WM8994_IRQ_FIFOS_ERR] = {
    .reg_offset = 1,
    .mask = WM8994_FIFOS_ERR_EINT,
    },
    [WM8994_IRQ_WSEQ_DONE] = {
    .reg_offset = 1,
    .mask = WM8994_WSEQ_DONE_EINT,
    },
    [WM8994_IRQ_DCS_DONE] = {
    .reg_offset = 1,
    .mask = WM8994_DCS_DONE_EINT,
    },
    [WM8994_IRQ_TEMP_WARN] = {
    .reg_offset = 1,
    .mask = WM8994_TEMP_WARN_EINT,
    },
    [WM8994_IRQ_GPIO(1)] = {
    .mask = WM8994_GP1_EINT,
    },
    [WM8994_IRQ_GPIO(2)] = {
    .mask = WM8994_GP2_EINT,
    },
    [WM8994_IRQ_GPIO(3)] = {
    .mask = WM8994_GP3_EINT,
    },
    [WM8994_IRQ_GPIO(4)] = {
    .mask = WM8994_GP4_EINT,
    },
    [WM8994_IRQ_GPIO(5)] = {
    .mask = WM8994_GP5_EINT,
    },
    [WM8994_IRQ_GPIO(6)] = {
    .mask = WM8994_GP6_EINT,
    },
    [WM8994_IRQ_GPIO(7)] = {
    .mask = WM8994_GP7_EINT,
    },
    [WM8994_IRQ_GPIO(8)] = {
    .mask = WM8994_GP8_EINT,
    },
    [WM8994_IRQ_GPIO(9)] = {
    .mask = WM8994_GP8_EINT,
    },
    [WM8994_IRQ_GPIO(10)] = {
    .mask = WM8994_GP10_EINT,
    },
    [WM8994_IRQ_GPIO(11)] = {
    .mask = WM8994_GP11_EINT,
    },
    };
    static const struct regmap_irq_chip wm8994_irq_chip = {
    .name = "wm8994",
    .irqs = wm8994_irqs,
    .num_irqs = ARRAY_SIZE(wm8994_irqs),
    .num_regs = 2,
    .status_base = WM8994_INTERRUPT_STATUS_1,
    .mask_base = WM8994_INTERRUPT_STATUS_1_MASK,
    .ack_base = WM8994_INTERRUPT_STATUS_1,
    .runtime_pm = true,
    };
#[no_mangle]
pub unsafe extern "C" fn wm8994_irq_init(wm8994: *mut wm8994) -> c_int {
    int wm8994_irq_init(struct wm8994 *wm8994)
    {
    int ret;
    if (!wm8994.irq) {
    dev_warn(wm8994.dev,
    "No interrupt specified, no interrupts\n");
    wm8994.irq_base = 0;
    return 0;
    }
// use a GPIO for edge triggered controllers
    ret = regmap_add_irq_chip(wm8994.regmap, wm8994.irq,
    IRQF_TRIGGER_HIGH | IRQF_ONESHOT,
    wm8994.irq_base, &wm8994_irq_chip,
    &wm8994.irq_data);
    if (ret != 0) {
    dev_err(wm8994.dev, "Failed to register IRQ chip: %d\n", ret);
    return ret;
    }
// Enable top level interrupt if it was masked
    wm8994_reg_write(wm8994, WM8994_INTERRUPT_CONTROL, 0);
    return 0;
    }
    EXPORT_SYMBOL(wm8994_irq_init);
#[no_mangle]
pub unsafe extern "C" fn wm8994_irq_exit(wm8994: *mut wm8994) {
    void wm8994_irq_exit(struct wm8994 *wm8994)
    {
    regmap_del_irq_chip(wm8994.irq, wm8994.irq_data);
    }
    EXPORT_SYMBOL(wm8994_irq_exit);
