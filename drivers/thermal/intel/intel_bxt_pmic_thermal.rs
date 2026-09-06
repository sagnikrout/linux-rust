//! Automatically rewritten from C to Rust
//! Source: drivers/thermal/intel/intel_bxt_pmic_thermal.c
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
// Intel Broxton PMIC thermal driver
//
// Copyright (C) 2016 Intel Corporation. All rights reserved.
//

pub const BXTWC_THRM0IRQ: c_uint = 0x4E04;
pub const BXTWC_THRM1IRQ: c_uint = 0x4E05;
pub const BXTWC_THRM2IRQ: c_uint = 0x4E06;
pub const BXTWC_MTHRM0IRQ: c_uint = 0x4E12;
pub const BXTWC_MTHRM1IRQ: c_uint = 0x4E13;
pub const BXTWC_MTHRM2IRQ: c_uint = 0x4E14;
pub const BXTWC_STHRM0IRQ: c_uint = 0x4F19;
pub const BXTWC_STHRM1IRQ: c_uint = 0x4F1A;
pub const BXTWC_STHRM2IRQ: c_uint = 0x4F1B;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trip_config_map {
    pub irq_reg: u16,
    pub irq_en: u16,
    pub evt_stat: u16,
    pub irq_mask: u8,
    pub irq_en_mask: u8,
    pub evt_mask: u8,
    pub trip_num: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct thermal_irq_map {
    pub handle: [c_char; 20],
    pub num_trips: c_int,
    pub trip_config: *const trip_config_map,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmic_thermal_data {
    pub maps: *const thermal_irq_map,
    pub num_maps: c_int,
}

    static const struct trip_config_map bxtwc_str0_trip_config[] = {
    {
    .irq_reg = BXTWC_THRM0IRQ,
    .irq_mask = 0x01,
    .irq_en = BXTWC_MTHRM0IRQ,
    .irq_en_mask = 0x01,
    .evt_stat = BXTWC_STHRM0IRQ,
    .evt_mask = 0x01,
    .trip_num = 0
    },
    {
    .irq_reg = BXTWC_THRM0IRQ,
    .irq_mask = 0x10,
    .irq_en = BXTWC_MTHRM0IRQ,
    .irq_en_mask = 0x10,
    .evt_stat = BXTWC_STHRM0IRQ,
    .evt_mask = 0x10,
    .trip_num = 1
    }
    };
    static const struct trip_config_map bxtwc_str1_trip_config[] = {
    {
    .irq_reg = BXTWC_THRM0IRQ,
    .irq_mask = 0x02,
    .irq_en = BXTWC_MTHRM0IRQ,
    .irq_en_mask = 0x02,
    .evt_stat = BXTWC_STHRM0IRQ,
    .evt_mask = 0x02,
    .trip_num = 0
    },
    {
    .irq_reg = BXTWC_THRM0IRQ,
    .irq_mask = 0x20,
    .irq_en = BXTWC_MTHRM0IRQ,
    .irq_en_mask = 0x20,
    .evt_stat = BXTWC_STHRM0IRQ,
    .evt_mask = 0x20,
    .trip_num = 1
    },
    };
    static const struct trip_config_map bxtwc_str2_trip_config[] = {
    {
    .irq_reg = BXTWC_THRM0IRQ,
    .irq_mask = 0x04,
    .irq_en = BXTWC_MTHRM0IRQ,
    .irq_en_mask = 0x04,
    .evt_stat = BXTWC_STHRM0IRQ,
    .evt_mask = 0x04,
    .trip_num = 0
    },
    {
    .irq_reg = BXTWC_THRM0IRQ,
    .irq_mask = 0x40,
    .irq_en = BXTWC_MTHRM0IRQ,
    .irq_en_mask = 0x40,
    .evt_stat = BXTWC_STHRM0IRQ,
    .evt_mask = 0x40,
    .trip_num = 1
    },
    };
    static const struct trip_config_map bxtwc_str3_trip_config[] = {
    {
    .irq_reg = BXTWC_THRM2IRQ,
    .irq_mask = 0x10,
    .irq_en = BXTWC_MTHRM2IRQ,
    .irq_en_mask = 0x10,
    .evt_stat = BXTWC_STHRM2IRQ,
    .evt_mask = 0x10,
    .trip_num = 0
    },
    };
    static const struct thermal_irq_map bxtwc_thermal_irq_map[] = {
    {
    .handle = "STR0",
    .trip_config = bxtwc_str0_trip_config,
    .num_trips = ARRAY_SIZE(bxtwc_str0_trip_config),
    },
    {
    .handle = "STR1",
    .trip_config = bxtwc_str1_trip_config,
    .num_trips = ARRAY_SIZE(bxtwc_str1_trip_config),
    },
    {
    .handle = "STR2",
    .trip_config = bxtwc_str2_trip_config,
    .num_trips = ARRAY_SIZE(bxtwc_str2_trip_config),
    },
    {
    .handle = "STR3",
    .trip_config = bxtwc_str3_trip_config,
    .num_trips = ARRAY_SIZE(bxtwc_str3_trip_config),
    },
    };
    static const struct pmic_thermal_data bxtwc_thermal_data = {
    .maps = bxtwc_thermal_irq_map,
    .num_maps = ARRAY_SIZE(bxtwc_thermal_irq_map),
    };
#[no_mangle]
unsafe extern "C" fn pmic_thermal_irq_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t pmic_thermal_irq_handler(int irq, void *data)
    {
    struct platform_device *pdev = data;
    struct thermal_zone_device *tzd;
    struct pmic_thermal_data *td;
    struct intel_soc_pmic *pmic;
    struct regmap *regmap;
    u8 reg_val, mask, irq_stat;
    u16 reg, evt_stat_reg;
    int i, j, ret;
    pmic = dev_get_drvdata(pdev.dev.parent);
    regmap = pmic.regmap;
    td = (struct pmic_thermal_data *)
    platform_get_device_id(pdev).driver_data;
// Resolve thermal irqs
    for (i = 0; i < td.num_maps; i++) {
    for (j = 0; j < td.maps[i].num_trips; j++) {
    reg = td.maps[i].trip_config[j].irq_reg;
    mask = td.maps[i].trip_config[j].irq_mask;
//
// Read the irq register to resolve whether the
// interrupt was triggered for this sensor
//
    if (regmap_read(regmap, reg, &ret))
    return IRQ_HANDLED;
    reg_val = (u8)ret;
    irq_stat = ((u8)ret & mask);
    if (!irq_stat)
    continue;
//
// Read the status register to find out what
// event occurred i.e a high or a low
//
    evt_stat_reg = td.maps[i].trip_config[j].evt_stat;
    if (regmap_read(regmap, evt_stat_reg, &ret))
    return IRQ_HANDLED;
    tzd = thermal_zone_get_zone_by_name(td.maps[i].handle);
    if (!IS_ERR(tzd))
    thermal_zone_device_update(tzd,
    THERMAL_EVENT_UNSPECIFIED);
// Clear the appropriate irq
    regmap_write(regmap, reg, reg_val & mask);
    }
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn pmic_thermal_probe(pdev: *mut platform_device) -> c_int {
    static int pmic_thermal_probe(struct platform_device *pdev)
    {
    struct regmap_irq_chip_data *regmap_irq_chip;
    struct pmic_thermal_data *thermal_data;
    int ret, irq, virq, i, j, pmic_irq_count;
    struct intel_soc_pmic *pmic;
    struct regmap *regmap;
    struct device *dev;
    u16 reg;
    u8 mask;
    dev = &pdev.dev;
    pmic = dev_get_drvdata(pdev.dev.parent);
    if (!pmic) {
    dev_err(dev, "Failed to get struct intel_soc_pmic pointer\n");
    return -ENODEV;
    }
    thermal_data = (struct pmic_thermal_data *)
    platform_get_device_id(pdev).driver_data;
    if (!thermal_data) {
    dev_err(dev, "No thermal data initialized!!\n");
    return -ENODEV;
    }
    regmap = pmic.regmap;
    regmap_irq_chip = pmic.irq_chip_data;
    pmic_irq_count = 0;
    while ((irq = platform_get_irq(pdev, pmic_irq_count)) != -ENXIO) {
    virq = regmap_irq_get_virq(regmap_irq_chip, irq);
    if (virq < 0) {
    dev_err(dev, "failed to get virq by irq %d\n", irq);
    return virq;
    }
    ret = devm_request_threaded_irq(&pdev.dev, virq,
    core::ptr::null_mut(), pmic_thermal_irq_handler,
    IRQF_ONESHOT, "pmic_thermal", pdev);
    if (ret)
    return ret;
    pmic_irq_count++;
    }
// Enable thermal interrupts
    for (i = 0; i < thermal_data.num_maps; i++) {
    for (j = 0; j < thermal_data.maps[i].num_trips; j++) {
    reg = thermal_data.maps[i].trip_config[j].irq_en;
    mask = thermal_data.maps[i].trip_config[j].irq_en_mask;
    ret = regmap_update_bits(regmap, reg, mask, 0x00);
    if (ret)
    return ret;
    }
    }
    return 0;
    }
    static const struct platform_device_id pmic_thermal_id_table[] = {
    {
    .name = "bxt_wcove_thermal",
    .driver_data = (kernel_ulong_t)&bxtwc_thermal_data,
    },
    {},
    };
    static struct platform_driver pmic_thermal_driver = {
    .probe = pmic_thermal_probe,
    .driver = {
    .name = "pmic_thermal",
    },
    .id_table = pmic_thermal_id_table,
    };
    MODULE_DEVICE_TABLE(platform, pmic_thermal_id_table);
    module_platform_driver(pmic_thermal_driver);
    MODULE_AUTHOR("Yegnesh S Iyer <yegnesh.s.iyer@intel.com>");
    MODULE_DESCRIPTION("Intel Broxton PMIC Thermal Driver");
    MODULE_LICENSE("GPL v2");
