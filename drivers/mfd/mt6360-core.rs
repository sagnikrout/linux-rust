//! Automatically rewritten from C to Rust
//! Source: drivers/mfd/mt6360-core.c
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
// Copyright (c) 2020 MediaTek Inc.
//
// Author: Gene Chen <gene_chen@richtek.com>
//

    enum {
    MT6360_SLAVE_TCPC = 0,
    MT6360_SLAVE_PMIC,
    MT6360_SLAVE_LDO,
    MT6360_SLAVE_PMU,
    MT6360_SLAVE_MAX,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt6360_ddata {
    pub i2c: [*mut i2c_client; MT6360_SLAVE_MAX],
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub irq_data: *mut regmap_irq_chip_data,
    pub chip_rev: c_uint,
    pub crc8_tbl: [u8; CRC8_TABLE_SIZE],
}

pub const MT6360_TCPC_SLAVEID: c_uint = 0x4E;
pub const MT6360_PMIC_SLAVEID: c_uint = 0x1A;
pub const MT6360_LDO_SLAVEID: c_uint = 0x64;
pub const MT6360_PMU_SLAVEID: c_uint = 0x34;
pub const MT6360_REG_TCPCSTART: c_uint = 0x00;
pub const MT6360_REG_TCPCEND: c_uint = 0xFF;
pub const MT6360_REG_PMICSTART: c_uint = 0x100;
pub const MT6360_REG_PMICEND: c_uint = 0x13B;
pub const MT6360_REG_LDOSTART: c_uint = 0x200;
pub const MT6360_REG_LDOEND: c_uint = 0x21C;
pub const MT6360_REG_PMUSTART: c_uint = 0x300;
pub const MT6360_PMU_DEV_INFO: c_uint = 0x300;
pub const MT6360_PMU_CHG_IRQ1: c_uint = 0x3D0;
pub const MT6360_PMU_CHG_MASK1: c_uint = 0x3F0;
pub const MT6360_REG_PMUEND: c_uint = 0x3FF;
pub const MT6360_PMU_IRQ_REGNUM: c_int = 16;
pub const CHIP_VEN_MASK: c_uint = 0xF0;
pub const CHIP_VEN_MT6360: c_uint = 0x50;
pub const CHIP_REV_MASK: c_uint = 0x0F;
pub const MT6360_ADDRESS_MASK: c_uint = 0x3F;
pub const MT6360_DATA_SIZE_1_BYTE: c_uint = 0x00;
pub const MT6360_DATA_SIZE_2_BYTES: c_uint = 0x40;
pub const MT6360_DATA_SIZE_3_BYTES: c_uint = 0x80;
pub const MT6360_DATA_SIZE_4_BYTES: c_uint = 0xC0;
pub const MT6360_CRC8_POLYNOMIAL: c_uint = 0x7;
pub const MT6360_CRC_I2C_ADDR_SIZE: c_int = 1;
pub const MT6360_CRC_REG_ADDR_SIZE: c_int = 1;
// prealloca read size = i2c device addr + i2c reg addr + val ... + crc8

// prealloca write size = i2c device addr + i2c reg addr + val ... + crc8 + dummy byte

pub const MT6360_CRC_CRC8_SIZE: c_int = 1;
pub const MT6360_CRC_DUMMY_BYTE_SIZE: c_int = 1;
pub const MT6360_REGMAP_REG_BYTE_SIZE: c_int = 2;

// reg 0 -> 0 ~ 7
pub const MT6360_CHG_TREG_EVT: c_int = 4;
pub const MT6360_CHG_AICR_EVT: c_int = 5;
pub const MT6360_CHG_MIVR_EVT: c_int = 6;
pub const MT6360_PWR_RDY_EVT: c_int = 7;
// REG 1 -> 8 ~ 15
pub const MT6360_CHG_BATSYSUV_EVT: c_int = 9;
pub const MT6360_FLED_CHG_VINOVP_EVT: c_int = 11;
pub const MT6360_CHG_VSYSUV_EVT: c_int = 12;
pub const MT6360_CHG_VSYSOV_EVT: c_int = 13;
pub const MT6360_CHG_VBATOV_EVT: c_int = 14;
pub const MT6360_CHG_VBUSOV_EVT: c_int = 15;
// REG 2 -> 16 ~ 23
// REG 3 -> 24 ~ 31
pub const MT6360_WD_PMU_DET: c_int = 25;
pub const MT6360_WD_PMU_DONE: c_int = 26;
pub const MT6360_CHG_TMRI: c_int = 27;
pub const MT6360_CHG_ADPBADI: c_int = 29;
pub const MT6360_CHG_RVPI: c_int = 30;
pub const MT6360_OTPI: c_int = 31;
// REG 4 -> 32 ~ 39
pub const MT6360_CHG_AICCMEASL: c_int = 32;
pub const MT6360_CHGDET_DONEI: c_int = 34;
pub const MT6360_WDTMRI: c_int = 35;
pub const MT6360_SSFINISHI: c_int = 36;
pub const MT6360_CHG_RECHGI: c_int = 37;
pub const MT6360_CHG_TERMI: c_int = 38;
pub const MT6360_CHG_IEOCI: c_int = 39;
// REG 5 -> 40 ~ 47
pub const MT6360_PUMPX_DONEI: c_int = 40;
pub const MT6360_BAT_OVP_ADC_EVT: c_int = 41;
pub const MT6360_TYPEC_OTP_EVT: c_int = 42;
pub const MT6360_ADC_WAKEUP_EVT: c_int = 43;
pub const MT6360_ADC_DONEI: c_int = 44;
pub const MT6360_BST_BATUVI: c_int = 45;
pub const MT6360_BST_VBUSOVI: c_int = 46;
pub const MT6360_BST_OLPI: c_int = 47;
// REG 6 -> 48 ~ 55
pub const MT6360_ATTACH_I: c_int = 48;
pub const MT6360_DETACH_I: c_int = 49;
pub const MT6360_QC30_STPDONE: c_int = 51;
pub const MT6360_QC_VBUSDET_DONE: c_int = 52;
pub const MT6360_HVDCP_DET: c_int = 53;
pub const MT6360_CHGDETI: c_int = 54;
pub const MT6360_DCDTI: c_int = 55;
// REG 7 -> 56 ~ 63
pub const MT6360_FOD_DONE_EVT: c_int = 56;
pub const MT6360_FOD_OV_EVT: c_int = 57;
pub const MT6360_CHRDET_UVP_EVT: c_int = 58;
pub const MT6360_CHRDET_OVP_EVT: c_int = 59;
pub const MT6360_CHRDET_EXT_EVT: c_int = 60;
pub const MT6360_FOD_LR_EVT: c_int = 61;
pub const MT6360_FOD_HR_EVT: c_int = 62;
pub const MT6360_FOD_DISCHG_FAIL_EVT: c_int = 63;
// REG 8 -> 64 ~ 71
pub const MT6360_USBID_EVT: c_int = 64;
pub const MT6360_APWDTRST_EVT: c_int = 65;
pub const MT6360_EN_EVT: c_int = 66;
pub const MT6360_QONB_RST_EVT: c_int = 67;
pub const MT6360_MRSTB_EVT: c_int = 68;
pub const MT6360_OTP_EVT: c_int = 69;
pub const MT6360_VDDAOV_EVT: c_int = 70;
pub const MT6360_SYSUV_EVT: c_int = 71;
// REG 9 -> 72 ~ 79
pub const MT6360_FLED_STRBPIN_EVT: c_int = 72;
pub const MT6360_FLED_TORPIN_EVT: c_int = 73;
pub const MT6360_FLED_TX_EVT: c_int = 74;
pub const MT6360_FLED_LVF_EVT: c_int = 75;
pub const MT6360_FLED2_SHORT_EVT: c_int = 78;
pub const MT6360_FLED1_SHORT_EVT: c_int = 79;
// REG 10 -> 80 ~ 87
pub const MT6360_FLED2_STRB_EVT: c_int = 80;
pub const MT6360_FLED1_STRB_EVT: c_int = 81;
pub const MT6360_FLED2_STRB_TO_EVT: c_int = 82;
pub const MT6360_FLED1_STRB_TO_EVT: c_int = 83;
pub const MT6360_FLED2_TOR_EVT: c_int = 84;
pub const MT6360_FLED1_TOR_EVT: c_int = 85;
// REG 11 -> 88 ~ 95
// REG 12 -> 96 ~ 103
pub const MT6360_BUCK1_PGB_EVT: c_int = 96;
pub const MT6360_BUCK1_OC_EVT: c_int = 100;
pub const MT6360_BUCK1_OV_EVT: c_int = 101;
pub const MT6360_BUCK1_UV_EVT: c_int = 102;
// REG 13 -> 104 ~ 111
pub const MT6360_BUCK2_PGB_EVT: c_int = 104;
pub const MT6360_BUCK2_OC_EVT: c_int = 108;
pub const MT6360_BUCK2_OV_EVT: c_int = 109;
pub const MT6360_BUCK2_UV_EVT: c_int = 110;
// REG 14 -> 112 ~ 119
pub const MT6360_LDO1_OC_EVT: c_int = 113;
pub const MT6360_LDO2_OC_EVT: c_int = 114;
pub const MT6360_LDO3_OC_EVT: c_int = 115;
pub const MT6360_LDO5_OC_EVT: c_int = 117;
pub const MT6360_LDO6_OC_EVT: c_int = 118;
pub const MT6360_LDO7_OC_EVT: c_int = 119;
// REG 15 -> 120 ~ 127
pub const MT6360_LDO1_PGB_EVT: c_int = 121;
pub const MT6360_LDO2_PGB_EVT: c_int = 122;
pub const MT6360_LDO3_PGB_EVT: c_int = 123;
pub const MT6360_LDO5_PGB_EVT: c_int = 125;
pub const MT6360_LDO6_PGB_EVT: c_int = 126;
pub const MT6360_LDO7_PGB_EVT: c_int = 127;
    static const struct regmap_irq mt6360_irqs[] =  {
    REGMAP_IRQ_REG_LINE(MT6360_CHG_TREG_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_CHG_AICR_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_CHG_MIVR_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_PWR_RDY_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_CHG_BATSYSUV_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_FLED_CHG_VINOVP_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_CHG_VSYSUV_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_CHG_VSYSOV_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_CHG_VBATOV_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_CHG_VBUSOV_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_WD_PMU_DET, 8),
    REGMAP_IRQ_REG_LINE(MT6360_WD_PMU_DONE, 8),
    REGMAP_IRQ_REG_LINE(MT6360_CHG_TMRI, 8),
    REGMAP_IRQ_REG_LINE(MT6360_CHG_ADPBADI, 8),
    REGMAP_IRQ_REG_LINE(MT6360_CHG_RVPI, 8),
    REGMAP_IRQ_REG_LINE(MT6360_OTPI, 8),
    REGMAP_IRQ_REG_LINE(MT6360_CHG_AICCMEASL, 8),
    REGMAP_IRQ_REG_LINE(MT6360_CHGDET_DONEI, 8),
    REGMAP_IRQ_REG_LINE(MT6360_WDTMRI, 8),
    REGMAP_IRQ_REG_LINE(MT6360_SSFINISHI, 8),
    REGMAP_IRQ_REG_LINE(MT6360_CHG_RECHGI, 8),
    REGMAP_IRQ_REG_LINE(MT6360_CHG_TERMI, 8),
    REGMAP_IRQ_REG_LINE(MT6360_CHG_IEOCI, 8),
    REGMAP_IRQ_REG_LINE(MT6360_PUMPX_DONEI, 8),
    REGMAP_IRQ_REG_LINE(MT6360_BAT_OVP_ADC_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_TYPEC_OTP_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_ADC_WAKEUP_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_ADC_DONEI, 8),
    REGMAP_IRQ_REG_LINE(MT6360_BST_BATUVI, 8),
    REGMAP_IRQ_REG_LINE(MT6360_BST_VBUSOVI, 8),
    REGMAP_IRQ_REG_LINE(MT6360_BST_OLPI, 8),
    REGMAP_IRQ_REG_LINE(MT6360_ATTACH_I, 8),
    REGMAP_IRQ_REG_LINE(MT6360_DETACH_I, 8),
    REGMAP_IRQ_REG_LINE(MT6360_QC30_STPDONE, 8),
    REGMAP_IRQ_REG_LINE(MT6360_QC_VBUSDET_DONE, 8),
    REGMAP_IRQ_REG_LINE(MT6360_HVDCP_DET, 8),
    REGMAP_IRQ_REG_LINE(MT6360_CHGDETI, 8),
    REGMAP_IRQ_REG_LINE(MT6360_DCDTI, 8),
    REGMAP_IRQ_REG_LINE(MT6360_FOD_DONE_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_FOD_OV_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_CHRDET_UVP_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_CHRDET_OVP_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_CHRDET_EXT_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_FOD_LR_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_FOD_HR_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_FOD_DISCHG_FAIL_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_USBID_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_APWDTRST_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_EN_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_QONB_RST_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_MRSTB_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_OTP_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_VDDAOV_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_SYSUV_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_FLED_STRBPIN_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_FLED_TORPIN_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_FLED_TX_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_FLED_LVF_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_FLED2_SHORT_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_FLED1_SHORT_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_FLED2_STRB_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_FLED1_STRB_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_FLED2_STRB_TO_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_FLED1_STRB_TO_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_FLED2_TOR_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_FLED1_TOR_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_BUCK1_PGB_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_BUCK1_OC_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_BUCK1_OV_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_BUCK1_UV_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_BUCK2_PGB_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_BUCK2_OC_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_BUCK2_OV_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_BUCK2_UV_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_LDO1_OC_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_LDO2_OC_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_LDO3_OC_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_LDO5_OC_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_LDO6_OC_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_LDO7_OC_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_LDO1_PGB_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_LDO2_PGB_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_LDO3_PGB_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_LDO5_PGB_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_LDO6_PGB_EVT, 8),
    REGMAP_IRQ_REG_LINE(MT6360_LDO7_PGB_EVT, 8),
    };
    static const struct regmap_irq_chip mt6360_irq_chip = {
    .name = "mt6360_irqs",
    .irqs = mt6360_irqs,
    .num_irqs = ARRAY_SIZE(mt6360_irqs),
    .num_regs = MT6360_PMU_IRQ_REGNUM,
    .mask_base = MT6360_PMU_CHG_MASK1,
    .status_base = MT6360_PMU_CHG_IRQ1,
    .ack_base = MT6360_PMU_CHG_IRQ1,
    .init_ack_masked = true,
    .use_ack = true,
    };
    static const struct resource mt6360_adc_resources[] = {
    DEFINE_RES_IRQ_NAMED(MT6360_ADC_DONEI, "adc_donei"),
    };
    static const struct resource mt6360_chg_resources[] = {
    DEFINE_RES_IRQ_NAMED(MT6360_CHG_TREG_EVT, "chg_treg_evt"),
    DEFINE_RES_IRQ_NAMED(MT6360_PWR_RDY_EVT, "pwr_rdy_evt"),
    DEFINE_RES_IRQ_NAMED(MT6360_CHG_BATSYSUV_EVT, "chg_batsysuv_evt"),
    DEFINE_RES_IRQ_NAMED(MT6360_CHG_VSYSUV_EVT, "chg_vsysuv_evt"),
    DEFINE_RES_IRQ_NAMED(MT6360_CHG_VSYSOV_EVT, "chg_vsysov_evt"),
    DEFINE_RES_IRQ_NAMED(MT6360_CHG_VBATOV_EVT, "chg_vbatov_evt"),
    DEFINE_RES_IRQ_NAMED(MT6360_CHG_VBUSOV_EVT, "chg_vbusov_evt"),
    DEFINE_RES_IRQ_NAMED(MT6360_CHG_AICCMEASL, "chg_aiccmeasl"),
    DEFINE_RES_IRQ_NAMED(MT6360_WDTMRI, "wdtmri"),
    DEFINE_RES_IRQ_NAMED(MT6360_CHG_RECHGI, "chg_rechgi"),
    DEFINE_RES_IRQ_NAMED(MT6360_CHG_TERMI, "chg_termi"),
    DEFINE_RES_IRQ_NAMED(MT6360_CHG_IEOCI, "chg_ieoci"),
    DEFINE_RES_IRQ_NAMED(MT6360_PUMPX_DONEI, "pumpx_donei"),
    DEFINE_RES_IRQ_NAMED(MT6360_ATTACH_I, "attach_i"),
    DEFINE_RES_IRQ_NAMED(MT6360_CHRDET_EXT_EVT, "chrdet_ext_evt"),
    };
    static const struct resource mt6360_led_resources[] = {
    DEFINE_RES_IRQ_NAMED(MT6360_FLED_CHG_VINOVP_EVT, "fled_chg_vinovp_evt"),
    DEFINE_RES_IRQ_NAMED(MT6360_FLED_LVF_EVT, "fled_lvf_evt"),
    DEFINE_RES_IRQ_NAMED(MT6360_FLED2_SHORT_EVT, "fled2_short_evt"),
    DEFINE_RES_IRQ_NAMED(MT6360_FLED1_SHORT_EVT, "fled1_short_evt"),
    DEFINE_RES_IRQ_NAMED(MT6360_FLED2_STRB_TO_EVT, "fled2_strb_to_evt"),
    DEFINE_RES_IRQ_NAMED(MT6360_FLED1_STRB_TO_EVT, "fled1_strb_to_evt"),
    };
    static const struct resource mt6360_regulator_resources[] = {
    DEFINE_RES_IRQ_NAMED(MT6360_BUCK1_PGB_EVT, "buck1_pgb_evt"),
    DEFINE_RES_IRQ_NAMED(MT6360_BUCK1_OC_EVT, "buck1_oc_evt"),
    DEFINE_RES_IRQ_NAMED(MT6360_BUCK1_OV_EVT, "buck1_ov_evt"),
    DEFINE_RES_IRQ_NAMED(MT6360_BUCK1_UV_EVT, "buck1_uv_evt"),
    DEFINE_RES_IRQ_NAMED(MT6360_BUCK2_PGB_EVT, "buck2_pgb_evt"),
    DEFINE_RES_IRQ_NAMED(MT6360_BUCK2_OC_EVT, "buck2_oc_evt"),
    DEFINE_RES_IRQ_NAMED(MT6360_BUCK2_OV_EVT, "buck2_ov_evt"),
    DEFINE_RES_IRQ_NAMED(MT6360_BUCK2_UV_EVT, "buck2_uv_evt"),
    DEFINE_RES_IRQ_NAMED(MT6360_LDO1_OC_EVT, "ldo1_oc_evt"),
    DEFINE_RES_IRQ_NAMED(MT6360_LDO2_OC_EVT, "ldo2_oc_evt"),
    DEFINE_RES_IRQ_NAMED(MT6360_LDO3_OC_EVT, "ldo3_oc_evt"),
    DEFINE_RES_IRQ_NAMED(MT6360_LDO5_OC_EVT, "ldo5_oc_evt"),
    DEFINE_RES_IRQ_NAMED(MT6360_LDO6_OC_EVT, "ldo6_oc_evt"),
    DEFINE_RES_IRQ_NAMED(MT6360_LDO7_OC_EVT, "ldo7_oc_evt"),
    DEFINE_RES_IRQ_NAMED(MT6360_LDO1_PGB_EVT, "ldo1_pgb_evt"),
    DEFINE_RES_IRQ_NAMED(MT6360_LDO2_PGB_EVT, "ldo2_pgb_evt"),
    DEFINE_RES_IRQ_NAMED(MT6360_LDO3_PGB_EVT, "ldo3_pgb_evt"),
    DEFINE_RES_IRQ_NAMED(MT6360_LDO5_PGB_EVT, "ldo5_pgb_evt"),
    DEFINE_RES_IRQ_NAMED(MT6360_LDO6_PGB_EVT, "ldo6_pgb_evt"),
    DEFINE_RES_IRQ_NAMED(MT6360_LDO7_PGB_EVT, "ldo7_pgb_evt"),
    };
    static const struct mfd_cell mt6360_devs[] = {
    MFD_CELL_OF("mt6360-adc", mt6360_adc_resources,
    core::ptr::null_mut(), 0, 0, "mediatek,mt6360-adc"),
    MFD_CELL_OF("mt6360-chg", mt6360_chg_resources,
    core::ptr::null_mut(), 0, 0, "mediatek,mt6360-chg"),
    MFD_CELL_OF("mt6360-led", mt6360_led_resources,
    core::ptr::null_mut(), 0, 0, "mediatek,mt6360-led"),
    MFD_CELL_RES("mt6360-regulator", mt6360_regulator_resources),
    MFD_CELL_OF("mt6360-tcpc", core::ptr::null_mut(),
    core::ptr::null_mut(), 0, 0, "mediatek,mt6360-tcpc"),
    };
#[no_mangle]
unsafe extern "C" fn mt6360_check_vendor_info(ddata: *mut mt6360_ddata) -> c_int {
    static int mt6360_check_vendor_info(struct mt6360_ddata *ddata)
    {
    u32 info;
    int ret;
    ret = regmap_read(ddata.regmap, MT6360_PMU_DEV_INFO, &info);
    if (ret < 0)
    return ret;
    if ((info & CHIP_VEN_MASK) != CHIP_VEN_MT6360) {
    dev_err(ddata.dev, "Device not supported\n");
    return -ENODEV;
    }
    ddata.chip_rev = info & CHIP_REV_MASK;
    return 0;
    }
    static const unsigned short mt6360_slave_addr[MT6360_SLAVE_MAX] = {
    MT6360_TCPC_SLAVEID,
    MT6360_PMIC_SLAVEID,
    MT6360_LDO_SLAVEID,
    MT6360_PMU_SLAVEID,
    };
#[no_mangle]
unsafe extern "C" fn mt6360_xlate_pmicldo_addr(addr: *mut u8, rw_size: c_int) -> c_int {
    static int mt6360_xlate_pmicldo_addr(u8 *addr, int rw_size)
    {
// Address is already in encoded [5:0]
// addr &= MT6360_ADDRESS_MASK;
    switch (rw_size) {
    case 1:
// addr |= MT6360_DATA_SIZE_1_BYTE;
    break;
    case 2:
// addr |= MT6360_DATA_SIZE_2_BYTES;
    break;
    case 3:
// addr |= MT6360_DATA_SIZE_3_BYTES;
    break;
    case 4:
// addr |= MT6360_DATA_SIZE_4_BYTES;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static int mt6360_regmap_read(void *context, const void *reg, size_t reg_size,
    void *val, size_t val_size)
    {
    struct mt6360_ddata *ddata = context;
    let mut bank: u8 = *(u8 *)reg;
    let mut reg_addr: u8 = *(u8 *)(reg + 1);
    struct i2c_client *i2c;
    let mut crc_needed: bool = false;
    let mut buf_len: c_int = MT6360_ALLOC_READ_SIZE(val_size);
    let mut read_size: c_int = val_size;
    u8 crc;
    int ret;
    if (bank >= MT6360_SLAVE_MAX)
    return -EINVAL;
    i2c = ddata.i2c[bank];
    if (bank == MT6360_SLAVE_PMIC || bank == MT6360_SLAVE_LDO) {
    crc_needed = true;
    ret = mt6360_xlate_pmicldo_addr(&reg_addr, val_size);
    if (ret < 0)
    return ret;
    read_size += MT6360_CRC_CRC8_SIZE;
    }
    u8 *buf __free(kfree) = kzalloc(buf_len, GFP_KERNEL);
    if (!buf)
    return -ENOMEM;
    buf[0] = I2C_ADDR_XLATE_8BIT(i2c.addr, I2C_SMBUS_READ);
    buf[1] = reg_addr;
    ret = i2c_smbus_read_i2c_block_data(i2c, reg_addr, read_size,
    buf + MT6360_CRC_PREDATA_OFFSET);
    if (ret < 0)
    return ret;
#[no_mangle]
pub unsafe extern "C" fn if(read_size: ret !=) -> else {
    else if (ret != read_size)
    return -EIO;
    if (crc_needed) {
    crc = crc8(ddata.crc8_tbl, buf, val_size + MT6360_CRC_PREDATA_OFFSET, 0);
    if (crc != buf[val_size + MT6360_CRC_PREDATA_OFFSET])
    return -EIO;
    }
    memcpy(val, buf + MT6360_CRC_PREDATA_OFFSET, val_size);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mt6360_regmap_write(context: *mut c_void, val: *const c_void, val_size: usize) -> c_int {
    static int mt6360_regmap_write(void *context, const void *val, size_t val_size)
    {
    struct mt6360_ddata *ddata = context;
    let mut bank: u8 = *(u8 *)val;
    let mut reg_addr: u8 = *(u8 *)(val + 1);
    struct i2c_client *i2c;
    let mut crc_needed: bool = false;
    u8 *buf;
    let mut buf_len: c_int = MT6360_ALLOC_WRITE_SIZE(val_size);
    let mut write_size: c_int = val_size - MT6360_REGMAP_REG_BYTE_SIZE;
    int ret;
    if (bank >= MT6360_SLAVE_MAX)
    return -EINVAL;
    i2c = ddata.i2c[bank];
    if (bank == MT6360_SLAVE_PMIC || bank == MT6360_SLAVE_LDO) {
    crc_needed = true;
    ret = mt6360_xlate_pmicldo_addr(&reg_addr, val_size - MT6360_REGMAP_REG_BYTE_SIZE);
    if (ret < 0)
    return ret;
    }
    buf = kzalloc(buf_len, GFP_KERNEL);
    if (!buf)
    return -ENOMEM;
    buf[0] = I2C_ADDR_XLATE_8BIT(i2c.addr, I2C_SMBUS_WRITE);
    buf[1] = reg_addr;
    memcpy(buf + MT6360_CRC_PREDATA_OFFSET, val + MT6360_REGMAP_REG_BYTE_SIZE, write_size);
    if (crc_needed) {
    buf[val_size] = crc8(ddata.crc8_tbl, buf, val_size, 0);
    write_size += (MT6360_CRC_CRC8_SIZE + MT6360_CRC_DUMMY_BYTE_SIZE);
    }
    ret = i2c_smbus_write_i2c_block_data(i2c, reg_addr, write_size,
    buf + MT6360_CRC_PREDATA_OFFSET);
    kfree(buf);
    return ret;
    }
    static const struct regmap_bus mt6360_regmap_bus = {
    .read		= mt6360_regmap_read,
    .write		= mt6360_regmap_write,
// Due to PMIC and LDO CRC access size limit
    .max_raw_read	= 4,
    .max_raw_write	= 4,
    };
#[no_mangle]
unsafe extern "C" fn mt6360_is_readwrite_reg(dev: *mut device, reg: c_uint) -> bool {
    static bool mt6360_is_readwrite_reg(struct device *dev, unsigned int reg)
    {
    switch (reg) {
    case MT6360_REG_TCPCSTART ... MT6360_REG_TCPCEND:
    fallthrough;
    case MT6360_REG_PMICSTART ... MT6360_REG_PMICEND:
    fallthrough;
    case MT6360_REG_LDOSTART ... MT6360_REG_LDOEND:
    fallthrough;
    case MT6360_REG_PMUSTART ... MT6360_REG_PMUEND:
    return true;
    }
    return false;
    }
    static const struct regmap_config mt6360_regmap_config = {
    .reg_bits		= 16,
    .val_bits		= 8,
    .reg_format_endian	= REGMAP_ENDIAN_BIG,
    .max_register		= MT6360_REG_PMUEND,
    .writeable_reg		= mt6360_is_readwrite_reg,
    .readable_reg		= mt6360_is_readwrite_reg,
    };
#[no_mangle]
unsafe extern "C" fn mt6360_probe(client: *mut i2c_client) -> c_int {
    static int mt6360_probe(struct i2c_client *client)
    {
    struct mt6360_ddata *ddata;
    int i, ret;
    ddata = devm_kzalloc(&client.dev, sizeof(*ddata), GFP_KERNEL);
    if (!ddata)
    return -ENOMEM;
    ddata.dev = &client.dev;
    i2c_set_clientdata(client, ddata);
    for (i = 0; i < MT6360_SLAVE_MAX - 1; i++) {
    ddata.i2c[i] = devm_i2c_new_dummy_device(&client.dev,
    client.adapter,
    mt6360_slave_addr[i]);
    if (IS_ERR(ddata.i2c[i])) {
    dev_err(&client.dev,
    "Failed to get new dummy I2C device for address 0x%x",
    mt6360_slave_addr[i]);
    return PTR_ERR(ddata.i2c[i]);
    }
    }
    ddata.i2c[MT6360_SLAVE_MAX - 1] = client;
    crc8_populate_msb(ddata.crc8_tbl, MT6360_CRC8_POLYNOMIAL);
    ddata.regmap = devm_regmap_init(ddata.dev, &mt6360_regmap_bus, ddata,
    &mt6360_regmap_config);
    if (IS_ERR(ddata.regmap)) {
    dev_err(&client.dev, "Failed to register regmap\n");
    return PTR_ERR(ddata.regmap);
    }
    ret = mt6360_check_vendor_info(ddata);
    if (ret)
    return ret;
    ret = devm_regmap_add_irq_chip(&client.dev, ddata.regmap, client.irq,
    0, 0, &mt6360_irq_chip,
    &ddata.irq_data);
    if (ret) {
    dev_err(&client.dev, "Failed to add Regmap IRQ Chip\n");
    return ret;
    }
    ret = devm_mfd_add_devices(&client.dev, PLATFORM_DEVID_AUTO,
    mt6360_devs, ARRAY_SIZE(mt6360_devs), core::ptr::null_mut(),
    0, regmap_irq_get_domain(ddata.irq_data));
    if (ret) {
    dev_err(&client.dev,
    "Failed to register subordinate devices\n");
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mt6360_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused mt6360_suspend(struct device *dev)
    {
    struct i2c_client *i2c = to_i2c_client(dev);
    if (device_may_wakeup(dev))
    enable_irq_wake(i2c.irq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mt6360_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused mt6360_resume(struct device *dev)
    {
    struct i2c_client *i2c = to_i2c_client(dev);
    if (device_may_wakeup(dev))
    disable_irq_wake(i2c.irq);
    return 0;
    }
    static SIMPLE_DEV_PM_OPS(mt6360_pm_ops, mt6360_suspend, mt6360_resume);
    static const struct of_device_id __maybe_unused mt6360_of_id[] = {
    { .compatible = "mediatek,mt6360", },
    {},
    };
    MODULE_DEVICE_TABLE(of, mt6360_of_id);
    static struct i2c_driver mt6360_driver = {
    .driver = {
    .name = "mt6360",
    .pm = &mt6360_pm_ops,
    .of_match_table = of_match_ptr(mt6360_of_id),
    },
    .probe = mt6360_probe,
    };
    module_i2c_driver(mt6360_driver);
    MODULE_AUTHOR("Gene Chen <gene_chen@richtek.com>");
    MODULE_DESCRIPTION("MT6360 I2C Driver");
    MODULE_LICENSE("GPL v2");
