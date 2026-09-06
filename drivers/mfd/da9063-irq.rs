//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/da9063-irq.c
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


// SPDX-License-Identifier: GPL-2.0+
// Interrupt support for Dialog DA9063
//
// Copyright 2012 Dialog Semiconductor Ltd.
// Copyright 2013 Philipp Zabel, Pengutronix
//
// Author: Michal Hajduk, Dialog Semiconductor
//

pub const DA9063_REG_EVENT_A_OFFSET: c_int = 0;
pub const DA9063_REG_EVENT_B_OFFSET: c_int = 1;
pub const DA9063_REG_EVENT_C_OFFSET: c_int = 2;
pub const DA9063_REG_EVENT_D_OFFSET: c_int = 3;
    static const struct regmap_irq da9063_irqs[] = {
// DA9063 event A register
    REGMAP_IRQ_REG(DA9063_IRQ_ONKEY,
    DA9063_REG_EVENT_A_OFFSET, DA9063_M_ONKEY),
    REGMAP_IRQ_REG(DA9063_IRQ_ALARM,
    DA9063_REG_EVENT_A_OFFSET, DA9063_M_ALARM),
    REGMAP_IRQ_REG(DA9063_IRQ_TICK,
    DA9063_REG_EVENT_A_OFFSET, DA9063_M_TICK),
    REGMAP_IRQ_REG(DA9063_IRQ_ADC_RDY,
    DA9063_REG_EVENT_A_OFFSET, DA9063_M_ADC_RDY),
    REGMAP_IRQ_REG(DA9063_IRQ_SEQ_RDY,
    DA9063_REG_EVENT_A_OFFSET, DA9063_M_SEQ_RDY),
// DA9063 event B register
    REGMAP_IRQ_REG(DA9063_IRQ_WAKE,
    DA9063_REG_EVENT_B_OFFSET, DA9063_M_WAKE),
    REGMAP_IRQ_REG(DA9063_IRQ_TEMP,
    DA9063_REG_EVENT_B_OFFSET, DA9063_M_TEMP),
    REGMAP_IRQ_REG(DA9063_IRQ_COMP_1V2,
    DA9063_REG_EVENT_B_OFFSET, DA9063_M_COMP_1V2),
    REGMAP_IRQ_REG(DA9063_IRQ_LDO_LIM,
    DA9063_REG_EVENT_B_OFFSET, DA9063_M_LDO_LIM),
    REGMAP_IRQ_REG(DA9063_IRQ_REG_UVOV,
    DA9063_REG_EVENT_B_OFFSET, DA9063_M_UVOV),
    REGMAP_IRQ_REG(DA9063_IRQ_DVC_RDY,
    DA9063_REG_EVENT_B_OFFSET, DA9063_M_DVC_RDY),
    REGMAP_IRQ_REG(DA9063_IRQ_VDD_MON,
    DA9063_REG_EVENT_B_OFFSET, DA9063_M_VDD_MON),
    REGMAP_IRQ_REG(DA9063_IRQ_WARN,
    DA9063_REG_EVENT_B_OFFSET, DA9063_M_VDD_WARN),
// DA9063 event C register
    REGMAP_IRQ_REG(DA9063_IRQ_GPI0,
    DA9063_REG_EVENT_C_OFFSET, DA9063_M_GPI0),
    REGMAP_IRQ_REG(DA9063_IRQ_GPI1,
    DA9063_REG_EVENT_C_OFFSET, DA9063_M_GPI1),
    REGMAP_IRQ_REG(DA9063_IRQ_GPI2,
    DA9063_REG_EVENT_C_OFFSET, DA9063_M_GPI2),
    REGMAP_IRQ_REG(DA9063_IRQ_GPI3,
    DA9063_REG_EVENT_C_OFFSET, DA9063_M_GPI3),
    REGMAP_IRQ_REG(DA9063_IRQ_GPI4,
    DA9063_REG_EVENT_C_OFFSET, DA9063_M_GPI4),
    REGMAP_IRQ_REG(DA9063_IRQ_GPI5,
    DA9063_REG_EVENT_C_OFFSET, DA9063_M_GPI5),
    REGMAP_IRQ_REG(DA9063_IRQ_GPI6,
    DA9063_REG_EVENT_C_OFFSET, DA9063_M_GPI6),
    REGMAP_IRQ_REG(DA9063_IRQ_GPI7,
    DA9063_REG_EVENT_C_OFFSET, DA9063_M_GPI7),
// DA9063 event D register
    REGMAP_IRQ_REG(DA9063_IRQ_GPI8,
    DA9063_REG_EVENT_D_OFFSET, DA9063_M_GPI8),
    REGMAP_IRQ_REG(DA9063_IRQ_GPI9,
    DA9063_REG_EVENT_D_OFFSET, DA9063_M_GPI9),
    REGMAP_IRQ_REG(DA9063_IRQ_GPI10,
    DA9063_REG_EVENT_D_OFFSET, DA9063_M_GPI10),
    REGMAP_IRQ_REG(DA9063_IRQ_GPI11,
    DA9063_REG_EVENT_D_OFFSET, DA9063_M_GPI11),
    REGMAP_IRQ_REG(DA9063_IRQ_GPI12,
    DA9063_REG_EVENT_D_OFFSET, DA9063_M_GPI12),
    REGMAP_IRQ_REG(DA9063_IRQ_GPI13,
    DA9063_REG_EVENT_D_OFFSET, DA9063_M_GPI13),
    REGMAP_IRQ_REG(DA9063_IRQ_GPI14,
    DA9063_REG_EVENT_D_OFFSET, DA9063_M_GPI14),
    REGMAP_IRQ_REG(DA9063_IRQ_GPI15,
    DA9063_REG_EVENT_D_OFFSET, DA9063_M_GPI15),
    };
    static const struct regmap_irq_chip da9063_irq_chip = {
    .name = "da9063-irq",
    .irqs = da9063_irqs,
    .num_irqs = ARRAY_SIZE(da9063_irqs),
    .num_regs = 4,
    .status_base = DA9063_REG_EVENT_A,
    .mask_base = DA9063_REG_IRQ_MASK_A,
    .ack_base = DA9063_REG_EVENT_A,
    .init_ack_masked = true,
    };
    static const struct regmap_irq da9063l_irqs[] = {
// DA9063 event A register
    REGMAP_IRQ_REG(DA9063_IRQ_ONKEY,
    DA9063_REG_EVENT_A_OFFSET, DA9063_M_ONKEY),
    REGMAP_IRQ_REG(DA9063_IRQ_ADC_RDY,
    DA9063_REG_EVENT_A_OFFSET, DA9063_M_ADC_RDY),
    REGMAP_IRQ_REG(DA9063_IRQ_SEQ_RDY,
    DA9063_REG_EVENT_A_OFFSET, DA9063_M_SEQ_RDY),
// DA9063 event B register
    REGMAP_IRQ_REG(DA9063_IRQ_WAKE,
    DA9063_REG_EVENT_B_OFFSET, DA9063_M_WAKE),
    REGMAP_IRQ_REG(DA9063_IRQ_TEMP,
    DA9063_REG_EVENT_B_OFFSET, DA9063_M_TEMP),
    REGMAP_IRQ_REG(DA9063_IRQ_COMP_1V2,
    DA9063_REG_EVENT_B_OFFSET, DA9063_M_COMP_1V2),
    REGMAP_IRQ_REG(DA9063_IRQ_LDO_LIM,
    DA9063_REG_EVENT_B_OFFSET, DA9063_M_LDO_LIM),
    REGMAP_IRQ_REG(DA9063_IRQ_REG_UVOV,
    DA9063_REG_EVENT_B_OFFSET, DA9063_M_UVOV),
    REGMAP_IRQ_REG(DA9063_IRQ_DVC_RDY,
    DA9063_REG_EVENT_B_OFFSET, DA9063_M_DVC_RDY),
    REGMAP_IRQ_REG(DA9063_IRQ_VDD_MON,
    DA9063_REG_EVENT_B_OFFSET, DA9063_M_VDD_MON),
    REGMAP_IRQ_REG(DA9063_IRQ_WARN,
    DA9063_REG_EVENT_B_OFFSET, DA9063_M_VDD_WARN),
// DA9063 event C register
    REGMAP_IRQ_REG(DA9063_IRQ_GPI0,
    DA9063_REG_EVENT_C_OFFSET, DA9063_M_GPI0),
    REGMAP_IRQ_REG(DA9063_IRQ_GPI1,
    DA9063_REG_EVENT_C_OFFSET, DA9063_M_GPI1),
    REGMAP_IRQ_REG(DA9063_IRQ_GPI2,
    DA9063_REG_EVENT_C_OFFSET, DA9063_M_GPI2),
    REGMAP_IRQ_REG(DA9063_IRQ_GPI3,
    DA9063_REG_EVENT_C_OFFSET, DA9063_M_GPI3),
    REGMAP_IRQ_REG(DA9063_IRQ_GPI4,
    DA9063_REG_EVENT_C_OFFSET, DA9063_M_GPI4),
    REGMAP_IRQ_REG(DA9063_IRQ_GPI5,
    DA9063_REG_EVENT_C_OFFSET, DA9063_M_GPI5),
    REGMAP_IRQ_REG(DA9063_IRQ_GPI6,
    DA9063_REG_EVENT_C_OFFSET, DA9063_M_GPI6),
    REGMAP_IRQ_REG(DA9063_IRQ_GPI7,
    DA9063_REG_EVENT_C_OFFSET, DA9063_M_GPI7),
// DA9063 event D register
    REGMAP_IRQ_REG(DA9063_IRQ_GPI8,
    DA9063_REG_EVENT_D_OFFSET, DA9063_M_GPI8),
    REGMAP_IRQ_REG(DA9063_IRQ_GPI9,
    DA9063_REG_EVENT_D_OFFSET, DA9063_M_GPI9),
    REGMAP_IRQ_REG(DA9063_IRQ_GPI10,
    DA9063_REG_EVENT_D_OFFSET, DA9063_M_GPI10),
    REGMAP_IRQ_REG(DA9063_IRQ_GPI11,
    DA9063_REG_EVENT_D_OFFSET, DA9063_M_GPI11),
    REGMAP_IRQ_REG(DA9063_IRQ_GPI12,
    DA9063_REG_EVENT_D_OFFSET, DA9063_M_GPI12),
    REGMAP_IRQ_REG(DA9063_IRQ_GPI13,
    DA9063_REG_EVENT_D_OFFSET, DA9063_M_GPI13),
    REGMAP_IRQ_REG(DA9063_IRQ_GPI14,
    DA9063_REG_EVENT_D_OFFSET, DA9063_M_GPI14),
    REGMAP_IRQ_REG(DA9063_IRQ_GPI15,
    DA9063_REG_EVENT_D_OFFSET, DA9063_M_GPI15),
    };
    static const struct regmap_irq_chip da9063l_irq_chip = {
    .name = "da9063l-irq",
    .irqs = da9063l_irqs,
    .num_irqs = ARRAY_SIZE(da9063l_irqs),
    .num_regs = 4,
    .status_base = DA9063_REG_EVENT_A,
    .mask_base = DA9063_REG_IRQ_MASK_A,
    .ack_base = DA9063_REG_EVENT_A,
    .init_ack_masked = true,
    };
#[no_mangle]
pub unsafe extern "C" fn da9063_irq_init(da9063: *mut da9063) -> c_int {
    int da9063_irq_init(struct da9063 *da9063)
    {
    const struct regmap_irq_chip *irq_chip;
    int ret;
    if (!da9063.chip_irq) {
    dev_err(da9063.dev, "No IRQ configured\n");
    return -EINVAL;
    }
    if (da9063.type == PMIC_TYPE_DA9063)
    irq_chip = &da9063_irq_chip;
    else
    irq_chip = &da9063l_irq_chip;
    ret = devm_regmap_add_irq_chip(da9063.dev, da9063.regmap,
    da9063.chip_irq,
    IRQF_TRIGGER_LOW | IRQF_ONESHOT | IRQF_SHARED,
    da9063.irq_base, irq_chip, &da9063.regmap_irq);
    if (ret) {
    dev_err(da9063.dev, "Failed to reguest IRQ %d: %d\n",
    da9063.chip_irq, ret);
    return ret;
    }
    return 0;
    }
