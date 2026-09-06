//! Automatically rewritten from C to Rust
//! Source: drivers/regulator/pfuze100-regulator.c
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
//
// Copyright (C) 2011-2013 Freescale Semiconductor, Inc. All Rights Reserved.

pub const PFUZE_NUMREGS: c_int = 128;
pub const PFUZE100_VOL_OFFSET: c_int = 0;
pub const PFUZE100_STANDBY_OFFSET: c_int = 1;
pub const PFUZE100_MODE_OFFSET: c_int = 3;
pub const PFUZE100_CONF_OFFSET: c_int = 4;
pub const PFUZE100_DEVICEID: c_uint = 0x0;
pub const PFUZE100_REVID: c_uint = 0x3;
pub const PFUZE100_FABID: c_uint = 0x4;
pub const PFUZE100_COINVOL: c_uint = 0x1a;
pub const PFUZE100_SW1ABVOL: c_uint = 0x20;
pub const PFUZE100_SW1ABMODE: c_uint = 0x23;
pub const PFUZE100_SW1CVOL: c_uint = 0x2e;
pub const PFUZE100_SW1CMODE: c_uint = 0x31;
pub const PFUZE100_SW2VOL: c_uint = 0x35;
pub const PFUZE100_SW2MODE: c_uint = 0x38;
pub const PFUZE100_SW3AVOL: c_uint = 0x3c;
pub const PFUZE100_SW3AMODE: c_uint = 0x3f;
pub const PFUZE100_SW3BVOL: c_uint = 0x43;
pub const PFUZE100_SW3BMODE: c_uint = 0x46;
pub const PFUZE100_SW4VOL: c_uint = 0x4a;
pub const PFUZE100_SW4MODE: c_uint = 0x4d;
pub const PFUZE100_SWBSTCON1: c_uint = 0x66;
pub const PFUZE100_VREFDDRCON: c_uint = 0x6a;
pub const PFUZE100_VSNVSVOL: c_uint = 0x6b;
pub const PFUZE100_VGEN1VOL: c_uint = 0x6c;
pub const PFUZE100_VGEN2VOL: c_uint = 0x6d;
pub const PFUZE100_VGEN3VOL: c_uint = 0x6e;
pub const PFUZE100_VGEN4VOL: c_uint = 0x6f;
pub const PFUZE100_VGEN5VOL: c_uint = 0x70;
pub const PFUZE100_VGEN6VOL: c_uint = 0x71;
pub const PFUZE100_SWxMODE_MASK: c_uint = 0xf;
pub const PFUZE100_SWxMODE_APS_APS: c_uint = 0x8;
pub const PFUZE100_SWxMODE_APS_OFF: c_uint = 0x4;

    enum chips { PFUZE100, PFUZE200, PFUZE3000 = 3, PFUZE3001 = 0x31, };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pfuze_regulator {
    pub desc: regulator_desc,
    pub stby_reg: c_uchar,
    pub stby_mask: c_uchar,
    pub sw_reg: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pfuze_chip {
    pub chip_id: c_int,
    pub flags: c_int,
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub regulator_descs: [pfuze_regulator; PFUZE100_MAX_REGULATOR],
    pub regulators: [*mut regulator_dev; PFUZE100_MAX_REGULATOR],
    pub pfuze_regulators: *const pfuze_regulator,
}

    static const int pfuze100_swbst[] = {
    5000000, 5050000, 5100000, 5150000,
    };
    static const int pfuze100_vsnvs[] = {
    1000000, 1100000, 1200000, 1300000, 1500000, 1800000, 3000000,
    };
    static const int pfuze100_coin[] = {
    2500000, 2700000, 2800000, 2900000, 3000000, 3100000, 3200000, 3300000,
    };
    static const int pfuze3000_sw1a[] = {
    700000, 725000, 750000, 775000, 800000, 825000, 850000, 875000,
    900000, 925000, 950000, 975000, 1000000, 1025000, 1050000, 1075000,
    1100000, 1125000, 1150000, 1175000, 1200000, 1225000, 1250000, 1275000,
    1300000, 1325000, 1350000, 1375000, 1400000, 1425000, 1800000, 3300000,
    };
    static const int pfuze3000_sw2lo[] = {
    1500000, 1550000, 1600000, 1650000, 1700000, 1750000, 1800000, 1850000,
    };
    static const int pfuze3000_sw2hi[] = {
    2500000, 2800000, 2850000, 3000000, 3100000, 3150000, 3200000, 3300000,
    };
    static const struct of_device_id pfuze_dt_ids[] = {
    { .compatible = "fsl,pfuze100", .data = (void *)PFUZE100},
    { .compatible = "fsl,pfuze200", .data = (void *)PFUZE200},
    { .compatible = "fsl,pfuze3000", .data = (void *)PFUZE3000},
    { .compatible = "fsl,pfuze3001", .data = (void *)PFUZE3001},
    { }
    };
    MODULE_DEVICE_TABLE(of, pfuze_dt_ids);
#[no_mangle]
unsafe extern "C" fn pfuze100_set_ramp_delay(rdev: *mut regulator_dev, ramp_delay: c_int) -> c_int {
    static int pfuze100_set_ramp_delay(struct regulator_dev *rdev, int ramp_delay)
    {
    struct pfuze_chip *pfuze100 = rdev_get_drvdata(rdev);
    let mut id: c_int = rdev_get_id(rdev);
    bool reg_has_ramp_delay;
    let mut ramp_bits: c_uint = 0;
    int ret;
    switch (pfuze100.chip_id) {
    case PFUZE3001:
// no dynamic voltage scaling for PF3001
    reg_has_ramp_delay = false;
    break;
    case PFUZE3000:
    reg_has_ramp_delay = (id < PFUZE3000_SWBST);
    break;
    case PFUZE200:
    reg_has_ramp_delay = (id < PFUZE200_SWBST);
    break;
    case PFUZE100:
    default:
    reg_has_ramp_delay = (id < PFUZE100_SWBST);
    break;
    }
    if (reg_has_ramp_delay) {
    if (ramp_delay > 0) {
    ramp_delay = 12500 / ramp_delay;
    ramp_bits = (ramp_delay >> 1) - (ramp_delay >> 3);
    }
    ret = regmap_update_bits(pfuze100.regmap,
    rdev.desc.vsel_reg + 4,
    0xc0, ramp_bits << 6);
    if (ret < 0)
    dev_err(pfuze100.dev, "ramp failed, err %d\n", ret);
    } else {
    ret = -EACCES;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pfuze100_ldo_set_suspend_disable(rdev: *mut regulator_dev) -> c_int {
    static int pfuze100_ldo_set_suspend_disable(struct regulator_dev *rdev)
    {
    struct pfuze_chip *pfuze100 = rdev_get_drvdata(rdev);
    let mut id: c_int = rdev_get_id(rdev);
    struct pfuze_regulator *desc = &pfuze100.regulator_descs[id];
//
// Set the standby bit so the LDO output is turned off when the PMIC
// receives a STANDBY event, using the per-regulator stby_reg/stby_mask
// that describe the standby control for each LDO.
//
// The stby_mask only covers the VGENxSTBY bit. The VGENxLPWR stays at
// its reset value of 0, so the LDO is switched off rather than put
// into low-power mode.
//
    return regmap_update_bits(pfuze100.regmap, desc.stby_reg,
    desc.stby_mask, desc.stby_mask);
    }
    static const struct regulator_ops pfuze100_ldo_regulator_ops = {
    .enable = regulator_enable_regmap,
    .disable = regulator_disable_regmap,
    .is_enabled = regulator_is_enabled_regmap,
    .list_voltage = regulator_list_voltage_linear,
    .set_voltage_sel = regulator_set_voltage_sel_regmap,
    .get_voltage_sel = regulator_get_voltage_sel_regmap,
    .set_suspend_disable = pfuze100_ldo_set_suspend_disable,
    };
    static const struct regulator_ops pfuze100_fixed_regulator_ops = {
    .enable = regulator_enable_regmap,
    .disable = regulator_disable_regmap,
    .is_enabled = regulator_is_enabled_regmap,
    .list_voltage = regulator_list_voltage_linear,
    };
    static const struct regulator_ops pfuze100_sw_regulator_ops = {
    .list_voltage = regulator_list_voltage_linear,
    .set_voltage_sel = regulator_set_voltage_sel_regmap,
    .get_voltage_sel = regulator_get_voltage_sel_regmap,
    .set_voltage_time_sel = regulator_set_voltage_time_sel,
    .set_ramp_delay = pfuze100_set_ramp_delay,
    };
    static const struct regulator_ops pfuze100_sw_disable_regulator_ops = {
    .enable = regulator_enable_regmap,
    .disable = regulator_disable_regmap,
    .is_enabled = regulator_is_enabled_regmap,
    .list_voltage = regulator_list_voltage_linear,
    .set_voltage_sel = regulator_set_voltage_sel_regmap,
    .get_voltage_sel = regulator_get_voltage_sel_regmap,
    .set_voltage_time_sel = regulator_set_voltage_time_sel,
    .set_ramp_delay = pfuze100_set_ramp_delay,
    };
    static const struct regulator_ops pfuze100_swb_regulator_ops = {
    .enable = regulator_enable_regmap,
    .disable = regulator_disable_regmap,
    .is_enabled = regulator_is_enabled_regmap,
    .list_voltage = regulator_list_voltage_table,
    .map_voltage = regulator_map_voltage_ascend,
    .set_voltage_sel = regulator_set_voltage_sel_regmap,
    .get_voltage_sel = regulator_get_voltage_sel_regmap,
    };
    static const struct regulator_ops pfuze3000_sw_regulator_ops = {
    .enable = regulator_enable_regmap,
    .disable = regulator_disable_regmap,
    .is_enabled = regulator_is_enabled_regmap,
    .list_voltage = regulator_list_voltage_table,
    .map_voltage = regulator_map_voltage_ascend,
    .set_voltage_sel = regulator_set_voltage_sel_regmap,
    .get_voltage_sel = regulator_get_voltage_sel_regmap,
    .set_voltage_time_sel = regulator_set_voltage_time_sel,
    .set_ramp_delay = pfuze100_set_ramp_delay,
    };

    [_chip ## _ ## _name] = {	\
    .desc = {	\
    .name = #_name,	\
    .n_voltages = 1,	\
    .ops = &pfuze100_fixed_regulator_ops,	\
    .type = REGULATOR_VOLTAGE,	\
    .id = _chip ## _ ## _name,	\
    .owner = THIS_MODULE,	\
    .min_uV = (voltage),	\
    .enable_reg = (base),	\
    .enable_mask = 0x10,	\
    },	\
    }

    [_chip ## _ ## _name] = {	\
    .desc = {	\
    .name = #_name,\
    .n_voltages = ((max) - (min)) / (step) + 1,	\
    .ops = &pfuze100_sw_regulator_ops,	\
    .type = REGULATOR_VOLTAGE,	\
    .id = _chip ## _ ## _name,	\
    .owner = THIS_MODULE,	\
    .min_uV = (min),	\
    .uV_step = (step),	\
    .vsel_reg = (base) + PFUZE100_VOL_OFFSET,	\
    .vsel_mask = 0x3f,	\
    .enable_reg = (base) + PFUZE100_MODE_OFFSET,	\
    .enable_mask = 0xf,	\
    },	\
    .stby_reg = (base) + PFUZE100_STANDBY_OFFSET,	\
    .stby_mask = 0x3f,	\
    .sw_reg = true,		\
    }

    [_chip ## _ ##  _name] = {	\
    .desc = {	\
    .name = #_name,	\
    .n_voltages = ARRAY_SIZE(voltages),	\
    .ops = &pfuze100_swb_regulator_ops,	\
    .type = REGULATOR_VOLTAGE,	\
    .id = _chip ## _ ## _name,	\
    .owner = THIS_MODULE,	\
    .volt_table = voltages,	\
    .vsel_reg = (base),	\
    .vsel_mask = (mask),	\
    .enable_reg = (base),	\
    .enable_mask = 0x48,	\
    },	\
    }

    [_chip ## _ ## _name] = {	\
    .desc = {	\
    .name = #_name,	\
    .n_voltages = ((max) - (min)) / (step) + 1,	\
    .ops = &pfuze100_ldo_regulator_ops,	\
    .type = REGULATOR_VOLTAGE,	\
    .id = _chip ## _ ## _name,	\
    .owner = THIS_MODULE,	\
    .min_uV = (min),	\
    .uV_step = (step),	\
    .vsel_reg = (base),	\
    .vsel_mask = 0xf,	\
    .enable_reg = (base),	\
    .enable_mask = 0x10,	\
    },	\
    .stby_reg = (base),	\
    .stby_mask = 0x20,	\
    }

    [_chip ## _ ##  _name] = {	\
    .desc = {	\
    .name = #_name,	\
    .n_voltages = ARRAY_SIZE(voltages),	\
    .ops = &pfuze100_swb_regulator_ops,	\
    .type = REGULATOR_VOLTAGE,	\
    .id = _chip ## _ ## _name,	\
    .owner = THIS_MODULE,	\
    .volt_table = voltages,	\
    .vsel_reg = (base),	\
    .vsel_mask = (mask),	\
    .enable_reg = (base),	\
    .enable_mask = 0x8,	\
    },	\
    }

    .desc = {	\
    .name = #_name,	\
    .n_voltages = ((max) - (min)) / (step) + 1,	\
    .ops = &pfuze100_ldo_regulator_ops,	\
    .type = REGULATOR_VOLTAGE,	\
    .id = _chip ## _ ## _name,	\
    .owner = THIS_MODULE,	\
    .min_uV = (min),	\
    .uV_step = (step),	\
    .vsel_reg = (base),	\
    .vsel_mask = 0x3,	\
    .enable_reg = (base),	\
    .enable_mask = 0x10,	\
    },	\
    .stby_reg = (base),	\
    .stby_mask = 0x20,	\
    }
// No linar case for the some switches of PFUZE3000

    [_chip ## _ ##  _name] = {	\
    .desc = {	\
    .name = #_name,	\
    .n_voltages = ARRAY_SIZE(voltages),	\
    .ops = &pfuze3000_sw_regulator_ops,	\
    .type = REGULATOR_VOLTAGE,	\
    .id = _chip ## _ ## _name,	\
    .owner = THIS_MODULE,	\
    .volt_table = voltages,	\
    .vsel_reg = (base) + PFUZE100_VOL_OFFSET,	\
    .vsel_mask = (mask),	\
    .enable_reg = (base) + PFUZE100_MODE_OFFSET,	\
    .enable_mask = 0xf,	\
    .enable_val = 0x8,	\
    .enable_time = 500,	\
    },	\
    .stby_reg = (base) + PFUZE100_STANDBY_OFFSET,	\
    .stby_mask = (mask),	\
    .sw_reg = true,		\
    }

    .desc = {	\
    .name = #_name,\
    .n_voltages = ((max) - (min)) / (step) + 1,	\
    .ops = &pfuze100_sw_regulator_ops,	\
    .type = REGULATOR_VOLTAGE,	\
    .id = _chip ## _ ## _name,	\
    .owner = THIS_MODULE,	\
    .min_uV = (min),	\
    .uV_step = (step),	\
    .vsel_reg = (base) + PFUZE100_VOL_OFFSET,	\
    .vsel_mask = 0xf,	\
    },	\
    .stby_reg = (base) + PFUZE100_STANDBY_OFFSET,	\
    .stby_mask = 0xf,	\
    }
// PFUZE100
    static const struct pfuze_regulator pfuze100_regulators[] = {
    PFUZE100_SW_REG(PFUZE100, SW1AB, PFUZE100_SW1ABVOL, 300000, 1875000, 25000),
    PFUZE100_SW_REG(PFUZE100, SW1C, PFUZE100_SW1CVOL, 300000, 1875000, 25000),
    PFUZE100_SW_REG(PFUZE100, SW2, PFUZE100_SW2VOL, 400000, 1975000, 25000),
    PFUZE100_SW_REG(PFUZE100, SW3A, PFUZE100_SW3AVOL, 400000, 1975000, 25000),
    PFUZE100_SW_REG(PFUZE100, SW3B, PFUZE100_SW3BVOL, 400000, 1975000, 25000),
    PFUZE100_SW_REG(PFUZE100, SW4, PFUZE100_SW4VOL, 400000, 1975000, 25000),
    PFUZE100_SWB_REG(PFUZE100, SWBST, PFUZE100_SWBSTCON1, 0x3 , pfuze100_swbst),
    PFUZE100_SWB_REG(PFUZE100, VSNVS, PFUZE100_VSNVSVOL, 0x7, pfuze100_vsnvs),
    PFUZE100_FIXED_REG(PFUZE100, VREFDDR, PFUZE100_VREFDDRCON, 750000),
    PFUZE100_VGEN_REG(PFUZE100, VGEN1, PFUZE100_VGEN1VOL, 800000, 1550000, 50000),
    PFUZE100_VGEN_REG(PFUZE100, VGEN2, PFUZE100_VGEN2VOL, 800000, 1550000, 50000),
    PFUZE100_VGEN_REG(PFUZE100, VGEN3, PFUZE100_VGEN3VOL, 1800000, 3300000, 100000),
    PFUZE100_VGEN_REG(PFUZE100, VGEN4, PFUZE100_VGEN4VOL, 1800000, 3300000, 100000),
    PFUZE100_VGEN_REG(PFUZE100, VGEN5, PFUZE100_VGEN5VOL, 1800000, 3300000, 100000),
    PFUZE100_VGEN_REG(PFUZE100, VGEN6, PFUZE100_VGEN6VOL, 1800000, 3300000, 100000),
    PFUZE100_COIN_REG(PFUZE100, COIN, PFUZE100_COINVOL, 0x7, pfuze100_coin),
    };
    static const struct pfuze_regulator pfuze200_regulators[] = {
    PFUZE100_SW_REG(PFUZE200, SW1AB, PFUZE100_SW1ABVOL, 300000, 1875000, 25000),
    PFUZE100_SW_REG(PFUZE200, SW2, PFUZE100_SW2VOL, 400000, 1975000, 25000),
    PFUZE100_SW_REG(PFUZE200, SW3A, PFUZE100_SW3AVOL, 400000, 1975000, 25000),
    PFUZE100_SW_REG(PFUZE200, SW3B, PFUZE100_SW3BVOL, 400000, 1975000, 25000),
    PFUZE100_SWB_REG(PFUZE200, SWBST, PFUZE100_SWBSTCON1, 0x3 , pfuze100_swbst),
    PFUZE100_SWB_REG(PFUZE200, VSNVS, PFUZE100_VSNVSVOL, 0x7, pfuze100_vsnvs),
    PFUZE100_FIXED_REG(PFUZE200, VREFDDR, PFUZE100_VREFDDRCON, 750000),
    PFUZE100_VGEN_REG(PFUZE200, VGEN1, PFUZE100_VGEN1VOL, 800000, 1550000, 50000),
    PFUZE100_VGEN_REG(PFUZE200, VGEN2, PFUZE100_VGEN2VOL, 800000, 1550000, 50000),
    PFUZE100_VGEN_REG(PFUZE200, VGEN3, PFUZE100_VGEN3VOL, 1800000, 3300000, 100000),
    PFUZE100_VGEN_REG(PFUZE200, VGEN4, PFUZE100_VGEN4VOL, 1800000, 3300000, 100000),
    PFUZE100_VGEN_REG(PFUZE200, VGEN5, PFUZE100_VGEN5VOL, 1800000, 3300000, 100000),
    PFUZE100_VGEN_REG(PFUZE200, VGEN6, PFUZE100_VGEN6VOL, 1800000, 3300000, 100000),
    PFUZE100_COIN_REG(PFUZE200, COIN, PFUZE100_COINVOL, 0x7, pfuze100_coin),
    };
    static const struct pfuze_regulator pfuze3000_regulators[] = {
    PFUZE3000_SW_REG(PFUZE3000, SW1A, PFUZE100_SW1ABVOL, 0x1f, pfuze3000_sw1a),
    PFUZE100_SW_REG(PFUZE3000, SW1B, PFUZE100_SW1CVOL, 700000, 1475000, 25000),
    PFUZE3000_SW_REG(PFUZE3000, SW2, PFUZE100_SW2VOL, 0x7, pfuze3000_sw2lo),
    PFUZE3000_SW3_REG(PFUZE3000, SW3, PFUZE100_SW3AVOL, 900000, 1650000, 50000),
    PFUZE100_SWB_REG(PFUZE3000, SWBST, PFUZE100_SWBSTCON1, 0x3, pfuze100_swbst),
    PFUZE100_SWB_REG(PFUZE3000, VSNVS, PFUZE100_VSNVSVOL, 0x7, pfuze100_vsnvs),
    PFUZE100_FIXED_REG(PFUZE3000, VREFDDR, PFUZE100_VREFDDRCON, 750000),
    PFUZE100_VGEN_REG(PFUZE3000, VLDO1, PFUZE100_VGEN1VOL, 1800000, 3300000, 100000),
    PFUZE100_VGEN_REG(PFUZE3000, VLDO2, PFUZE100_VGEN2VOL, 800000, 1550000, 50000),
    PFUZE3000_VCC_REG(PFUZE3000, VCCSD, PFUZE100_VGEN3VOL, 2850000, 3300000, 150000),
    PFUZE3000_VCC_REG(PFUZE3000, V33, PFUZE100_VGEN4VOL, 2850000, 3300000, 150000),
    PFUZE100_VGEN_REG(PFUZE3000, VLDO3, PFUZE100_VGEN5VOL, 1800000, 3300000, 100000),
    PFUZE100_VGEN_REG(PFUZE3000, VLDO4, PFUZE100_VGEN6VOL, 1800000, 3300000, 100000),
    };
    static const struct pfuze_regulator pfuze3001_regulators[] = {
    PFUZE3000_SW_REG(PFUZE3001, SW1, PFUZE100_SW1ABVOL, 0x1f, pfuze3000_sw1a),
    PFUZE3000_SW_REG(PFUZE3001, SW2, PFUZE100_SW2VOL, 0x7, pfuze3000_sw2lo),
    PFUZE3000_SW3_REG(PFUZE3001, SW3, PFUZE100_SW3AVOL, 900000, 1650000, 50000),
    PFUZE100_SWB_REG(PFUZE3001, VSNVS, PFUZE100_VSNVSVOL, 0x7, pfuze100_vsnvs),
    PFUZE100_VGEN_REG(PFUZE3001, VLDO1, PFUZE100_VGEN1VOL, 1800000, 3300000, 100000),
    PFUZE100_VGEN_REG(PFUZE3001, VLDO2, PFUZE100_VGEN2VOL, 800000, 1550000, 50000),
    PFUZE3000_VCC_REG(PFUZE3001, VCCSD, PFUZE100_VGEN3VOL, 2850000, 3300000, 150000),
    PFUZE3000_VCC_REG(PFUZE3001, V33, PFUZE100_VGEN4VOL, 2850000, 3300000, 150000),
    PFUZE100_VGEN_REG(PFUZE3001, VLDO3, PFUZE100_VGEN5VOL, 1800000, 3300000, 100000),
    PFUZE100_VGEN_REG(PFUZE3001, VLDO4, PFUZE100_VGEN6VOL, 1800000, 3300000, 100000),
    };
// PFUZE100
    static struct of_regulator_match pfuze100_matches[] = {
    { .name = "sw1ab",	},
    { .name = "sw1c",	},
    { .name = "sw2",	},
    { .name = "sw3a",	},
    { .name = "sw3b",	},
    { .name = "sw4",	},
    { .name = "swbst",	},
    { .name = "vsnvs",	},
    { .name = "vrefddr",	},
    { .name = "vgen1",	},
    { .name = "vgen2",	},
    { .name = "vgen3",	},
    { .name = "vgen4",	},
    { .name = "vgen5",	},
    { .name = "vgen6",	},
    { .name = "coin",	},
    };
// PFUZE200
    static struct of_regulator_match pfuze200_matches[] = {
    { .name = "sw1ab",	},
    { .name = "sw2",	},
    { .name = "sw3a",	},
    { .name = "sw3b",	},
    { .name = "swbst",	},
    { .name = "vsnvs",	},
    { .name = "vrefddr",	},
    { .name = "vgen1",	},
    { .name = "vgen2",	},
    { .name = "vgen3",	},
    { .name = "vgen4",	},
    { .name = "vgen5",	},
    { .name = "vgen6",	},
    { .name = "coin",	},
    };
// PFUZE3000
    static struct of_regulator_match pfuze3000_matches[] = {
    { .name = "sw1a",	},
    { .name = "sw1b",	},
    { .name = "sw2",	},
    { .name = "sw3",	},
    { .name = "swbst",	},
    { .name = "vsnvs",	},
    { .name = "vrefddr",	},
    { .name = "vldo1",	},
    { .name = "vldo2",	},
    { .name = "vccsd",	},
    { .name = "v33",	},
    { .name = "vldo3",	},
    { .name = "vldo4",	},
    };
// PFUZE3001
    static struct of_regulator_match pfuze3001_matches[] = {
    { .name = "sw1",	},
    { .name = "sw2",	},
    { .name = "sw3",	},
    { .name = "vsnvs",	},
    { .name = "vldo1",	},
    { .name = "vldo2",	},
    { .name = "vccsd",	},
    { .name = "v33",	},
    { .name = "vldo3",	},
    { .name = "vldo4",	},
    };
    static struct of_regulator_match *pfuze_matches;
#[no_mangle]
unsafe extern "C" fn pfuze_parse_regulators_dt(chip: *mut pfuze_chip) -> c_int {
    static int pfuze_parse_regulators_dt(struct pfuze_chip *chip)
    {
    struct device *dev = chip.dev;
    struct device_node *np, *parent;
    int ret;
    np = of_node_get(dev.of_node);
    if (!np)
    return -EINVAL;
    if (of_property_read_bool(np, "fsl,pfuze-support-disable-sw"))
    chip.flags |= PFUZE_FLAG_DISABLE_SW;
    parent = of_get_child_by_name(np, "regulators");
    if (!parent) {
    dev_err(dev, "regulators node not found\n");
    of_node_put(np);
    return -EINVAL;
    }
    switch (chip.chip_id) {
    case PFUZE3001:
    pfuze_matches = pfuze3001_matches;
    ret = of_regulator_match(dev, parent, pfuze3001_matches,
    ARRAY_SIZE(pfuze3001_matches));
    break;
    case PFUZE3000:
    pfuze_matches = pfuze3000_matches;
    ret = of_regulator_match(dev, parent, pfuze3000_matches,
    ARRAY_SIZE(pfuze3000_matches));
    break;
    case PFUZE200:
    pfuze_matches = pfuze200_matches;
    ret = of_regulator_match(dev, parent, pfuze200_matches,
    ARRAY_SIZE(pfuze200_matches));
    break;
    case PFUZE100:
    default:
    pfuze_matches = pfuze100_matches;
    ret = of_regulator_match(dev, parent, pfuze100_matches,
    ARRAY_SIZE(pfuze100_matches));
    break;
    }
    of_node_put(parent);
    of_node_put(np);
    if (ret < 0) {
    dev_err(dev, "Error parsing regulator init data: %d\n",
    ret);
    return ret;
    }
    return 0;
    }
    static inline struct regulator_init_data *match_init_data(int index)
    {
    return pfuze_matches[index].init_data;
    }
    static inline struct device_node *match_of_node(int index)
    {
    return pfuze_matches[index].of_node;
    }
#[no_mangle]
unsafe extern "C" fn pfuze_power_off_prepare(data: *mut sys_off_data) -> c_int {
    static int pfuze_power_off_prepare(struct sys_off_data *data)
    {
    struct pfuze_chip *syspm_pfuze_chip = data.cb_data;
    dev_info(syspm_pfuze_chip.dev, "Configure standby mode for power off");
// Switch from default mode: APS/APS to APS/Off
    regmap_update_bits(syspm_pfuze_chip.regmap, PFUZE100_SW1ABMODE,
    PFUZE100_SWxMODE_MASK, PFUZE100_SWxMODE_APS_OFF);
    regmap_update_bits(syspm_pfuze_chip.regmap, PFUZE100_SW1CMODE,
    PFUZE100_SWxMODE_MASK, PFUZE100_SWxMODE_APS_OFF);
    regmap_update_bits(syspm_pfuze_chip.regmap, PFUZE100_SW2MODE,
    PFUZE100_SWxMODE_MASK, PFUZE100_SWxMODE_APS_OFF);
    regmap_update_bits(syspm_pfuze_chip.regmap, PFUZE100_SW3AMODE,
    PFUZE100_SWxMODE_MASK, PFUZE100_SWxMODE_APS_OFF);
    regmap_update_bits(syspm_pfuze_chip.regmap, PFUZE100_SW3BMODE,
    PFUZE100_SWxMODE_MASK, PFUZE100_SWxMODE_APS_OFF);
    regmap_update_bits(syspm_pfuze_chip.regmap, PFUZE100_SW4MODE,
    PFUZE100_SWxMODE_MASK, PFUZE100_SWxMODE_APS_OFF);
    regmap_update_bits(syspm_pfuze_chip.regmap, PFUZE100_VGEN1VOL,
    PFUZE100_VGENxLPWR | PFUZE100_VGENxSTBY,
    PFUZE100_VGENxSTBY);
    regmap_update_bits(syspm_pfuze_chip.regmap, PFUZE100_VGEN2VOL,
    PFUZE100_VGENxLPWR | PFUZE100_VGENxSTBY,
    PFUZE100_VGENxSTBY);
    regmap_update_bits(syspm_pfuze_chip.regmap, PFUZE100_VGEN3VOL,
    PFUZE100_VGENxLPWR | PFUZE100_VGENxSTBY,
    PFUZE100_VGENxSTBY);
    regmap_update_bits(syspm_pfuze_chip.regmap, PFUZE100_VGEN4VOL,
    PFUZE100_VGENxLPWR | PFUZE100_VGENxSTBY,
    PFUZE100_VGENxSTBY);
    regmap_update_bits(syspm_pfuze_chip.regmap, PFUZE100_VGEN5VOL,
    PFUZE100_VGENxLPWR | PFUZE100_VGENxSTBY,
    PFUZE100_VGENxSTBY);
    regmap_update_bits(syspm_pfuze_chip.regmap, PFUZE100_VGEN6VOL,
    PFUZE100_VGENxLPWR | PFUZE100_VGENxSTBY,
    PFUZE100_VGENxSTBY);
    return NOTIFY_DONE;
    }
#[no_mangle]
unsafe extern "C" fn pfuze_power_off_prepare_init(pfuze_chip: *mut pfuze_chip) -> c_int {
    static int pfuze_power_off_prepare_init(struct pfuze_chip *pfuze_chip)
    {
    int err;
    if (pfuze_chip.chip_id != PFUZE100) {
    dev_warn(pfuze_chip.dev, "Requested pm_power_off_prepare handler for not supported chip\n");
    return -ENODEV;
    }
    err = devm_register_sys_off_handler(pfuze_chip.dev,
    SYS_OFF_MODE_POWER_OFF_PREPARE,
    SYS_OFF_PRIO_DEFAULT,
    pfuze_power_off_prepare,
    pfuze_chip);
    if (err) {
    dev_err(pfuze_chip.dev, "failed to register sys-off handler: %d\n",
    err);
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pfuze_identify(pfuze_chip: *mut pfuze_chip) -> c_int {
    static int pfuze_identify(struct pfuze_chip *pfuze_chip)
    {
    unsigned int value;
    int ret;
    ret = regmap_read(pfuze_chip.regmap, PFUZE100_DEVICEID, &value);
    if (ret)
    return ret;
    if (((value & 0x0f) == 0x8) && (pfuze_chip.chip_id == PFUZE100)) {
//
// Freescale misprogrammed 1-3% of parts prior to week 8 of 2013
// as ID=8 in PFUZE100
//
    dev_info(pfuze_chip.dev, "Assuming misprogrammed ID=0x8");
    } else if ((value & 0x0f) != pfuze_chip.chip_id &&
    (value & 0xf0) >> 4 != pfuze_chip.chip_id &&
    (value != pfuze_chip.chip_id)) {
// device id NOT match with your setting
    dev_warn(pfuze_chip.dev, "Illegal ID: %x\n", value);
    return -ENODEV;
    }
    ret = regmap_read(pfuze_chip.regmap, PFUZE100_REVID, &value);
    if (ret)
    return ret;
    dev_info(pfuze_chip.dev,
    "Full layer: %x, Metal layer: %x\n",
    (value & 0xf0) >> 4, value & 0x0f);
    ret = regmap_read(pfuze_chip.regmap, PFUZE100_FABID, &value);
    if (ret)
    return ret;
    dev_info(pfuze_chip.dev, "FAB: %x, FIN: %x\n",
    (value & 0xc) >> 2, value & 0x3);
    return 0;
    }
    static const struct regmap_config pfuze_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = PFUZE_NUMREGS - 1,
    .cache_type = REGCACHE_MAPLE,
    };
#[no_mangle]
unsafe extern "C" fn pfuze100_regulator_probe(client: *mut i2c_client) -> c_int {
    static int pfuze100_regulator_probe(struct i2c_client *client)
    {
    const struct i2c_device_id *id = i2c_client_get_device_id(client);
    struct pfuze_chip *pfuze_chip;
    let mut config: regulator_config = { };
    int i, ret;
    const struct of_device_id *match;
    u32 regulator_num;
    u32 sw_check_start, sw_check_end, sw_hi = 0x40;
    pfuze_chip = devm_kzalloc(&client.dev, sizeof(*pfuze_chip),
    GFP_KERNEL);
    if (!pfuze_chip)
    return -ENOMEM;
    if (client.dev.of_node) {
    match = of_match_device(pfuze_dt_ids, &client.dev);
    if (!match) {
    dev_err(&client.dev, "Error: No device match found\n");
    return -ENODEV;
    }
    pfuze_chip.chip_id = (int)(long)match.data;
    } else if (id) {
    pfuze_chip.chip_id = id.driver_data;
    } else {
    dev_err(&client.dev, "No dts match or id table match found\n");
    return -ENODEV;
    }
    i2c_set_clientdata(client, pfuze_chip);
    pfuze_chip.dev = &client.dev;
    pfuze_chip.regmap = devm_regmap_init_i2c(client, &pfuze_regmap_config);
    if (IS_ERR(pfuze_chip.regmap)) {
    ret = PTR_ERR(pfuze_chip.regmap);
    dev_err(&client.dev,
    "regmap allocation failed with err %d\n", ret);
    return ret;
    }
    ret = pfuze_identify(pfuze_chip);
    if (ret) {
    dev_err(&client.dev, "unrecognized pfuze chip ID!\n");
    return ret;
    }
// use the right regulators after identify the right device
    switch (pfuze_chip.chip_id) {
    case PFUZE3001:
    pfuze_chip.pfuze_regulators = pfuze3001_regulators;
    regulator_num = ARRAY_SIZE(pfuze3001_regulators);
    sw_check_start = PFUZE3001_SW2;
    sw_check_end = PFUZE3001_SW2;
    sw_hi = 1 << 3;
    break;
    case PFUZE3000:
    pfuze_chip.pfuze_regulators = pfuze3000_regulators;
    regulator_num = ARRAY_SIZE(pfuze3000_regulators);
    sw_check_start = PFUZE3000_SW2;
    sw_check_end = PFUZE3000_SW2;
    sw_hi = 1 << 3;
    break;
    case PFUZE200:
    pfuze_chip.pfuze_regulators = pfuze200_regulators;
    regulator_num = ARRAY_SIZE(pfuze200_regulators);
    sw_check_start = PFUZE200_SW2;
    sw_check_end = PFUZE200_SW3B;
    break;
    case PFUZE100:
    default:
    pfuze_chip.pfuze_regulators = pfuze100_regulators;
    regulator_num = ARRAY_SIZE(pfuze100_regulators);
    sw_check_start = PFUZE100_SW2;
    sw_check_end = PFUZE100_SW4;
    break;
    }
    dev_info(&client.dev, "pfuze%s found.\n",
    (pfuze_chip.chip_id == PFUZE100) ? "100" :
    (((pfuze_chip.chip_id == PFUZE200) ? "200" :
    ((pfuze_chip.chip_id == PFUZE3000) ? "3000" : "3001"))));
    memcpy(pfuze_chip.regulator_descs, pfuze_chip.pfuze_regulators,
    regulator_num * sizeof(struct pfuze_regulator));
    ret = pfuze_parse_regulators_dt(pfuze_chip);
    if (ret)
    return ret;
    for (i = 0; i < regulator_num; i++) {
    struct regulator_init_data *init_data;
    struct regulator_desc *desc;
    int val;
    desc = &pfuze_chip.regulator_descs[i].desc;
    init_data = match_init_data(i);
// SW2~SW4 high bit check and modify the voltage value table
    if (i >= sw_check_start && i <= sw_check_end) {
    ret = regmap_read(pfuze_chip.regmap,
    desc.vsel_reg, &val);
    if (ret) {
    dev_err(&client.dev, "Fails to read from the register.\n");
    return ret;
    }
    if (val & sw_hi) {
    if (pfuze_chip.chip_id == PFUZE3000 ||
    pfuze_chip.chip_id == PFUZE3001) {
    desc.volt_table = pfuze3000_sw2hi;
    desc.n_voltages = ARRAY_SIZE(pfuze3000_sw2hi);
    } else {
    desc.min_uV = 800000;
    desc.uV_step = 50000;
    desc.n_voltages = 51;
    }
    }
    }
//
// Allow SW regulators to turn off. Checking it trough a flag is
// a workaround to keep the backward compatibility with existing
// old dtb's which may relay on the fact that we didn't disable
// the switched regulator till yet.
//
    if (pfuze_chip.flags & PFUZE_FLAG_DISABLE_SW) {
    if (pfuze_chip.chip_id == PFUZE100 ||
    pfuze_chip.chip_id == PFUZE200) {
    if (pfuze_chip.regulator_descs[i].sw_reg) {
    desc.ops = &pfuze100_sw_disable_regulator_ops;
    desc.enable_val = 0x8;
    desc.disable_val = 0x0;
    desc.enable_time = 500;
    }
    }
    }
    config.dev = &client.dev;
    config.init_data = init_data;
    config.driver_data = pfuze_chip;
    config.of_node = match_of_node(i);
    pfuze_chip.regulators[i] =
    devm_regulator_register(&client.dev, desc, &config);
    if (IS_ERR(pfuze_chip.regulators[i])) {
    dev_err(&client.dev, "register regulator%s failed\n",
    pfuze_chip.pfuze_regulators[i].desc.name);
    return PTR_ERR(pfuze_chip.regulators[i]);
    }
    }
    if (of_property_read_bool(client.dev.of_node,
    "fsl,pmic-stby-poweroff"))
    return pfuze_power_off_prepare_init(pfuze_chip);
    return 0;
    }
    static struct i2c_driver pfuze_driver = {
    .driver = {
    .name = "pfuze100-regulator",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .of_match_table = pfuze_dt_ids,
    },
    .probe = pfuze100_regulator_probe,
    };
    module_i2c_driver(pfuze_driver);
    MODULE_AUTHOR("Robin Gong <b38343@freescale.com>");
    MODULE_DESCRIPTION("Regulator Driver for Freescale PFUZE100/200/3000/3001 PMIC");
    MODULE_LICENSE("GPL v2");
