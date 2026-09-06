//! Automatically rewritten from C to Rust
//! Source: drivers/pinctrl/bcm/pinctrl-bcm281xx.c
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
// Copyright (C) 2013-2017 Broadcom

// BCM281XX Pin Control Registers Definitions
// Function Select bits are the same for all pin control registers
pub const BCM281XX_PIN_REG_F_SEL_MASK: c_uint = 0x0700;
pub const BCM281XX_PIN_REG_F_SEL_SHIFT: c_int = 8;
// Standard pin register
pub const BCM281XX_STD_PIN_REG_DRV_STR_MASK: c_uint = 0x0007;
pub const BCM281XX_STD_PIN_REG_DRV_STR_SHIFT: c_int = 0;
pub const BCM281XX_STD_PIN_REG_INPUT_DIS_MASK: c_uint = 0x0008;
pub const BCM281XX_STD_PIN_REG_INPUT_DIS_SHIFT: c_int = 3;
pub const BCM281XX_STD_PIN_REG_SLEW_MASK: c_uint = 0x0010;
pub const BCM281XX_STD_PIN_REG_SLEW_SHIFT: c_int = 4;
pub const BCM281XX_STD_PIN_REG_PULL_UP_MASK: c_uint = 0x0020;
pub const BCM281XX_STD_PIN_REG_PULL_UP_SHIFT: c_int = 5;
pub const BCM281XX_STD_PIN_REG_PULL_DN_MASK: c_uint = 0x0040;
pub const BCM281XX_STD_PIN_REG_PULL_DN_SHIFT: c_int = 6;
pub const BCM281XX_STD_PIN_REG_HYST_MASK: c_uint = 0x0080;
pub const BCM281XX_STD_PIN_REG_HYST_SHIFT: c_int = 7;
// I2C pin register
pub const BCM281XX_I2C_PIN_REG_INPUT_DIS_MASK: c_uint = 0x0004;
pub const BCM281XX_I2C_PIN_REG_INPUT_DIS_SHIFT: c_int = 2;
pub const BCM281XX_I2C_PIN_REG_SLEW_MASK: c_uint = 0x0008;
pub const BCM281XX_I2C_PIN_REG_SLEW_SHIFT: c_int = 3;
pub const BCM281XX_I2C_PIN_REG_PULL_UP_STR_MASK: c_uint = 0x0070;
pub const BCM281XX_I2C_PIN_REG_PULL_UP_STR_SHIFT: c_int = 4;
// HDMI pin register
pub const BCM281XX_HDMI_PIN_REG_INPUT_DIS_MASK: c_uint = 0x0008;
pub const BCM281XX_HDMI_PIN_REG_INPUT_DIS_SHIFT: c_int = 3;
pub const BCM281XX_HDMI_PIN_REG_MODE_MASK: c_uint = 0x0010;
pub const BCM281XX_HDMI_PIN_REG_MODE_SHIFT: c_int = 4;
// BCM21664 access lock registers
pub const BCM21664_WR_ACCESS_OFFSET: c_uint = 0x07F0;
pub const BCM21664_WR_ACCESS_PASSWORD: c_uint = 0xA5A501;

pub const BCM21664_ACCESS_LOCK_COUNT: c_int = 5;
//
// bcm281xx_pin_type - types of pin register
//
    enum bcm281xx_pin_type {
    BCM281XX_PIN_TYPE_UNKNOWN = 0,
    BCM281XX_PIN_TYPE_STD,
    BCM281XX_PIN_TYPE_I2C,
    BCM281XX_PIN_TYPE_HDMI,
    };
    let mut std_pin: static enum bcm281xx_pin_type = BCM281XX_PIN_TYPE_STD;
    let mut i2c_pin: static enum bcm281xx_pin_type = BCM281XX_PIN_TYPE_I2C;
    let mut hdmi_pin: static enum bcm281xx_pin_type = BCM281XX_PIN_TYPE_HDMI;
//
// bcm281xx_pin_function- define pin function
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm281xx_pin_function {
    pub name: *const c_char,
    pub groups: *const *const c_char,
    pub ngroups: c_uint,
}

//
// Device types (used in bcm281xx_pinctrl_desc to differentiate
// the two device types from each other)
//
    enum bcm281xx_pinctrl_type {
    BCM281XX_PINCTRL_TYPE,
    BCM21664_PINCTRL_TYPE,
    };
//
// bcm281xx_pinctrl_info - description of a pinctrl device supported
// by this driver, intended to be used as a provider of OF match data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm281xx_pinctrl_info {
    pub device_type: enum bcm281xx_pinctrl_type,
// List of all pins
    pub pins: *const pinctrl_pin_desc,
    pub npins: c_uint,
    pub functions: *const bcm281xx_pin_function,
    pub nfunctions: c_uint,
    pub regmap_config: *const regmap_config,
}

//
// bcm281xx_pinctrl_data - Broadcom-specific pinctrl data
// @reg_base - base of pinctrl registers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm281xx_pinctrl_data {
    pub dev: *mut device,
    pub reg_base: *mut void __iomem,
    pub regmap: *mut regmap,
    pub info: *const bcm281xx_pinctrl_info,
}

//
// Pin number definition.  The order here must be the same as defined in the
// PADCTRLREG block in the RDB.
//
pub const BCM281XX_PIN_ADCSYNC: c_int = 0;
pub const BCM281XX_PIN_BAT_RM: c_int = 1;
pub const BCM281XX_PIN_BSC1_SCL: c_int = 2;
pub const BCM281XX_PIN_BSC1_SDA: c_int = 3;
pub const BCM281XX_PIN_BSC2_SCL: c_int = 4;
pub const BCM281XX_PIN_BSC2_SDA: c_int = 5;
pub const BCM281XX_PIN_CLASSGPWR: c_int = 6;
pub const BCM281XX_PIN_CLK_CX8: c_int = 7;
pub const BCM281XX_PIN_CLKOUT_0: c_int = 8;
pub const BCM281XX_PIN_CLKOUT_1: c_int = 9;
pub const BCM281XX_PIN_CLKOUT_2: c_int = 10;
pub const BCM281XX_PIN_CLKOUT_3: c_int = 11;
pub const BCM281XX_PIN_CLKREQ_IN_0: c_int = 12;
pub const BCM281XX_PIN_CLKREQ_IN_1: c_int = 13;
pub const BCM281XX_PIN_CWS_SYS_REQ1: c_int = 14;
pub const BCM281XX_PIN_CWS_SYS_REQ2: c_int = 15;
pub const BCM281XX_PIN_CWS_SYS_REQ3: c_int = 16;
pub const BCM281XX_PIN_DIGMIC1_CLK: c_int = 17;
pub const BCM281XX_PIN_DIGMIC1_DQ: c_int = 18;
pub const BCM281XX_PIN_DIGMIC2_CLK: c_int = 19;
pub const BCM281XX_PIN_DIGMIC2_DQ: c_int = 20;
pub const BCM281XX_PIN_GPEN13: c_int = 21;
pub const BCM281XX_PIN_GPEN14: c_int = 22;
pub const BCM281XX_PIN_GPEN15: c_int = 23;
pub const BCM281XX_PIN_GPIO00: c_int = 24;
pub const BCM281XX_PIN_GPIO01: c_int = 25;
pub const BCM281XX_PIN_GPIO02: c_int = 26;
pub const BCM281XX_PIN_GPIO03: c_int = 27;
pub const BCM281XX_PIN_GPIO04: c_int = 28;
pub const BCM281XX_PIN_GPIO05: c_int = 29;
pub const BCM281XX_PIN_GPIO06: c_int = 30;
pub const BCM281XX_PIN_GPIO07: c_int = 31;
pub const BCM281XX_PIN_GPIO08: c_int = 32;
pub const BCM281XX_PIN_GPIO09: c_int = 33;
pub const BCM281XX_PIN_GPIO10: c_int = 34;
pub const BCM281XX_PIN_GPIO11: c_int = 35;
pub const BCM281XX_PIN_GPIO12: c_int = 36;
pub const BCM281XX_PIN_GPIO13: c_int = 37;
pub const BCM281XX_PIN_GPIO14: c_int = 38;
pub const BCM281XX_PIN_GPS_PABLANK: c_int = 39;
pub const BCM281XX_PIN_GPS_TMARK: c_int = 40;
pub const BCM281XX_PIN_HDMI_SCL: c_int = 41;
pub const BCM281XX_PIN_HDMI_SDA: c_int = 42;
pub const BCM281XX_PIN_IC_DM: c_int = 43;
pub const BCM281XX_PIN_IC_DP: c_int = 44;
pub const BCM281XX_PIN_KP_COL_IP_0: c_int = 45;
pub const BCM281XX_PIN_KP_COL_IP_1: c_int = 46;
pub const BCM281XX_PIN_KP_COL_IP_2: c_int = 47;
pub const BCM281XX_PIN_KP_COL_IP_3: c_int = 48;
pub const BCM281XX_PIN_KP_ROW_OP_0: c_int = 49;
pub const BCM281XX_PIN_KP_ROW_OP_1: c_int = 50;
pub const BCM281XX_PIN_KP_ROW_OP_2: c_int = 51;
pub const BCM281XX_PIN_KP_ROW_OP_3: c_int = 52;
pub const BCM281XX_PIN_LCD_B_0: c_int = 53;
pub const BCM281XX_PIN_LCD_B_1: c_int = 54;
pub const BCM281XX_PIN_LCD_B_2: c_int = 55;
pub const BCM281XX_PIN_LCD_B_3: c_int = 56;
pub const BCM281XX_PIN_LCD_B_4: c_int = 57;
pub const BCM281XX_PIN_LCD_B_5: c_int = 58;
pub const BCM281XX_PIN_LCD_B_6: c_int = 59;
pub const BCM281XX_PIN_LCD_B_7: c_int = 60;
pub const BCM281XX_PIN_LCD_G_0: c_int = 61;
pub const BCM281XX_PIN_LCD_G_1: c_int = 62;
pub const BCM281XX_PIN_LCD_G_2: c_int = 63;
pub const BCM281XX_PIN_LCD_G_3: c_int = 64;
pub const BCM281XX_PIN_LCD_G_4: c_int = 65;
pub const BCM281XX_PIN_LCD_G_5: c_int = 66;
pub const BCM281XX_PIN_LCD_G_6: c_int = 67;
pub const BCM281XX_PIN_LCD_G_7: c_int = 68;
pub const BCM281XX_PIN_LCD_HSYNC: c_int = 69;
pub const BCM281XX_PIN_LCD_OE: c_int = 70;
pub const BCM281XX_PIN_LCD_PCLK: c_int = 71;
pub const BCM281XX_PIN_LCD_R_0: c_int = 72;
pub const BCM281XX_PIN_LCD_R_1: c_int = 73;
pub const BCM281XX_PIN_LCD_R_2: c_int = 74;
pub const BCM281XX_PIN_LCD_R_3: c_int = 75;
pub const BCM281XX_PIN_LCD_R_4: c_int = 76;
pub const BCM281XX_PIN_LCD_R_5: c_int = 77;
pub const BCM281XX_PIN_LCD_R_6: c_int = 78;
pub const BCM281XX_PIN_LCD_R_7: c_int = 79;
pub const BCM281XX_PIN_LCD_VSYNC: c_int = 80;
pub const BCM281XX_PIN_MDMGPIO0: c_int = 81;
pub const BCM281XX_PIN_MDMGPIO1: c_int = 82;
pub const BCM281XX_PIN_MDMGPIO2: c_int = 83;
pub const BCM281XX_PIN_MDMGPIO3: c_int = 84;
pub const BCM281XX_PIN_MDMGPIO4: c_int = 85;
pub const BCM281XX_PIN_MDMGPIO5: c_int = 86;
pub const BCM281XX_PIN_MDMGPIO6: c_int = 87;
pub const BCM281XX_PIN_MDMGPIO7: c_int = 88;
pub const BCM281XX_PIN_MDMGPIO8: c_int = 89;
pub const BCM281XX_PIN_MPHI_DATA_0: c_int = 90;
pub const BCM281XX_PIN_MPHI_DATA_1: c_int = 91;
pub const BCM281XX_PIN_MPHI_DATA_2: c_int = 92;
pub const BCM281XX_PIN_MPHI_DATA_3: c_int = 93;
pub const BCM281XX_PIN_MPHI_DATA_4: c_int = 94;
pub const BCM281XX_PIN_MPHI_DATA_5: c_int = 95;
pub const BCM281XX_PIN_MPHI_DATA_6: c_int = 96;
pub const BCM281XX_PIN_MPHI_DATA_7: c_int = 97;
pub const BCM281XX_PIN_MPHI_DATA_8: c_int = 98;
pub const BCM281XX_PIN_MPHI_DATA_9: c_int = 99;
pub const BCM281XX_PIN_MPHI_DATA_10: c_int = 100;
pub const BCM281XX_PIN_MPHI_DATA_11: c_int = 101;
pub const BCM281XX_PIN_MPHI_DATA_12: c_int = 102;
pub const BCM281XX_PIN_MPHI_DATA_13: c_int = 103;
pub const BCM281XX_PIN_MPHI_DATA_14: c_int = 104;
pub const BCM281XX_PIN_MPHI_DATA_15: c_int = 105;
pub const BCM281XX_PIN_MPHI_HA0: c_int = 106;
pub const BCM281XX_PIN_MPHI_HAT0: c_int = 107;
pub const BCM281XX_PIN_MPHI_HAT1: c_int = 108;
pub const BCM281XX_PIN_MPHI_HCE0_N: c_int = 109;
pub const BCM281XX_PIN_MPHI_HCE1_N: c_int = 110;
pub const BCM281XX_PIN_MPHI_HRD_N: c_int = 111;
pub const BCM281XX_PIN_MPHI_HWR_N: c_int = 112;
pub const BCM281XX_PIN_MPHI_RUN0: c_int = 113;
pub const BCM281XX_PIN_MPHI_RUN1: c_int = 114;
pub const BCM281XX_PIN_MTX_SCAN_CLK: c_int = 115;
pub const BCM281XX_PIN_MTX_SCAN_DATA: c_int = 116;
pub const BCM281XX_PIN_NAND_AD_0: c_int = 117;
pub const BCM281XX_PIN_NAND_AD_1: c_int = 118;
pub const BCM281XX_PIN_NAND_AD_2: c_int = 119;
pub const BCM281XX_PIN_NAND_AD_3: c_int = 120;
pub const BCM281XX_PIN_NAND_AD_4: c_int = 121;
pub const BCM281XX_PIN_NAND_AD_5: c_int = 122;
pub const BCM281XX_PIN_NAND_AD_6: c_int = 123;
pub const BCM281XX_PIN_NAND_AD_7: c_int = 124;
pub const BCM281XX_PIN_NAND_ALE: c_int = 125;
pub const BCM281XX_PIN_NAND_CEN_0: c_int = 126;
pub const BCM281XX_PIN_NAND_CEN_1: c_int = 127;
pub const BCM281XX_PIN_NAND_CLE: c_int = 128;
pub const BCM281XX_PIN_NAND_OEN: c_int = 129;
pub const BCM281XX_PIN_NAND_RDY_0: c_int = 130;
pub const BCM281XX_PIN_NAND_RDY_1: c_int = 131;
pub const BCM281XX_PIN_NAND_WEN: c_int = 132;
pub const BCM281XX_PIN_NAND_WP: c_int = 133;
pub const BCM281XX_PIN_PC1: c_int = 134;
pub const BCM281XX_PIN_PC2: c_int = 135;
pub const BCM281XX_PIN_PMU_INT: c_int = 136;
pub const BCM281XX_PIN_PMU_SCL: c_int = 137;
pub const BCM281XX_PIN_PMU_SDA: c_int = 138;
pub const BCM281XX_PIN_RFST2G_MTSLOTEN3G: c_int = 139;
pub const BCM281XX_PIN_RGMII_0_RX_CTL: c_int = 140;
pub const BCM281XX_PIN_RGMII_0_RXC: c_int = 141;
pub const BCM281XX_PIN_RGMII_0_RXD_0: c_int = 142;
pub const BCM281XX_PIN_RGMII_0_RXD_1: c_int = 143;
pub const BCM281XX_PIN_RGMII_0_RXD_2: c_int = 144;
pub const BCM281XX_PIN_RGMII_0_RXD_3: c_int = 145;
pub const BCM281XX_PIN_RGMII_0_TX_CTL: c_int = 146;
pub const BCM281XX_PIN_RGMII_0_TXC: c_int = 147;
pub const BCM281XX_PIN_RGMII_0_TXD_0: c_int = 148;
pub const BCM281XX_PIN_RGMII_0_TXD_1: c_int = 149;
pub const BCM281XX_PIN_RGMII_0_TXD_2: c_int = 150;
pub const BCM281XX_PIN_RGMII_0_TXD_3: c_int = 151;
pub const BCM281XX_PIN_RGMII_1_RX_CTL: c_int = 152;
pub const BCM281XX_PIN_RGMII_1_RXC: c_int = 153;
pub const BCM281XX_PIN_RGMII_1_RXD_0: c_int = 154;
pub const BCM281XX_PIN_RGMII_1_RXD_1: c_int = 155;
pub const BCM281XX_PIN_RGMII_1_RXD_2: c_int = 156;
pub const BCM281XX_PIN_RGMII_1_RXD_3: c_int = 157;
pub const BCM281XX_PIN_RGMII_1_TX_CTL: c_int = 158;
pub const BCM281XX_PIN_RGMII_1_TXC: c_int = 159;
pub const BCM281XX_PIN_RGMII_1_TXD_0: c_int = 160;
pub const BCM281XX_PIN_RGMII_1_TXD_1: c_int = 161;
pub const BCM281XX_PIN_RGMII_1_TXD_2: c_int = 162;
pub const BCM281XX_PIN_RGMII_1_TXD_3: c_int = 163;
pub const BCM281XX_PIN_RGMII_GPIO_0: c_int = 164;
pub const BCM281XX_PIN_RGMII_GPIO_1: c_int = 165;
pub const BCM281XX_PIN_RGMII_GPIO_2: c_int = 166;
pub const BCM281XX_PIN_RGMII_GPIO_3: c_int = 167;
pub const BCM281XX_PIN_RTXDATA2G_TXDATA3G1: c_int = 168;
pub const BCM281XX_PIN_RTXEN2G_TXDATA3G2: c_int = 169;
pub const BCM281XX_PIN_RXDATA3G0: c_int = 170;
pub const BCM281XX_PIN_RXDATA3G1: c_int = 171;
pub const BCM281XX_PIN_RXDATA3G2: c_int = 172;
pub const BCM281XX_PIN_SDIO1_CLK: c_int = 173;
pub const BCM281XX_PIN_SDIO1_CMD: c_int = 174;
pub const BCM281XX_PIN_SDIO1_DATA_0: c_int = 175;
pub const BCM281XX_PIN_SDIO1_DATA_1: c_int = 176;
pub const BCM281XX_PIN_SDIO1_DATA_2: c_int = 177;
pub const BCM281XX_PIN_SDIO1_DATA_3: c_int = 178;
pub const BCM281XX_PIN_SDIO4_CLK: c_int = 179;
pub const BCM281XX_PIN_SDIO4_CMD: c_int = 180;
pub const BCM281XX_PIN_SDIO4_DATA_0: c_int = 181;
pub const BCM281XX_PIN_SDIO4_DATA_1: c_int = 182;
pub const BCM281XX_PIN_SDIO4_DATA_2: c_int = 183;
pub const BCM281XX_PIN_SDIO4_DATA_3: c_int = 184;
pub const BCM281XX_PIN_SIM_CLK: c_int = 185;
pub const BCM281XX_PIN_SIM_DATA: c_int = 186;
pub const BCM281XX_PIN_SIM_DET: c_int = 187;
pub const BCM281XX_PIN_SIM_RESETN: c_int = 188;
pub const BCM281XX_PIN_SIM2_CLK: c_int = 189;
pub const BCM281XX_PIN_SIM2_DATA: c_int = 190;
pub const BCM281XX_PIN_SIM2_DET: c_int = 191;
pub const BCM281XX_PIN_SIM2_RESETN: c_int = 192;
pub const BCM281XX_PIN_SRI_C: c_int = 193;
pub const BCM281XX_PIN_SRI_D: c_int = 194;
pub const BCM281XX_PIN_SRI_E: c_int = 195;
pub const BCM281XX_PIN_SSP_EXTCLK: c_int = 196;
pub const BCM281XX_PIN_SSP0_CLK: c_int = 197;
pub const BCM281XX_PIN_SSP0_FS: c_int = 198;
pub const BCM281XX_PIN_SSP0_RXD: c_int = 199;
pub const BCM281XX_PIN_SSP0_TXD: c_int = 200;
pub const BCM281XX_PIN_SSP2_CLK: c_int = 201;
pub const BCM281XX_PIN_SSP2_FS_0: c_int = 202;
pub const BCM281XX_PIN_SSP2_FS_1: c_int = 203;
pub const BCM281XX_PIN_SSP2_FS_2: c_int = 204;
pub const BCM281XX_PIN_SSP2_FS_3: c_int = 205;
pub const BCM281XX_PIN_SSP2_RXD_0: c_int = 206;
pub const BCM281XX_PIN_SSP2_RXD_1: c_int = 207;
pub const BCM281XX_PIN_SSP2_TXD_0: c_int = 208;
pub const BCM281XX_PIN_SSP2_TXD_1: c_int = 209;
pub const BCM281XX_PIN_SSP3_CLK: c_int = 210;
pub const BCM281XX_PIN_SSP3_FS: c_int = 211;
pub const BCM281XX_PIN_SSP3_RXD: c_int = 212;
pub const BCM281XX_PIN_SSP3_TXD: c_int = 213;
pub const BCM281XX_PIN_SSP4_CLK: c_int = 214;
pub const BCM281XX_PIN_SSP4_FS: c_int = 215;
pub const BCM281XX_PIN_SSP4_RXD: c_int = 216;
pub const BCM281XX_PIN_SSP4_TXD: c_int = 217;
pub const BCM281XX_PIN_SSP5_CLK: c_int = 218;
pub const BCM281XX_PIN_SSP5_FS: c_int = 219;
pub const BCM281XX_PIN_SSP5_RXD: c_int = 220;
pub const BCM281XX_PIN_SSP5_TXD: c_int = 221;
pub const BCM281XX_PIN_SSP6_CLK: c_int = 222;
pub const BCM281XX_PIN_SSP6_FS: c_int = 223;
pub const BCM281XX_PIN_SSP6_RXD: c_int = 224;
pub const BCM281XX_PIN_SSP6_TXD: c_int = 225;
pub const BCM281XX_PIN_STAT_1: c_int = 226;
pub const BCM281XX_PIN_STAT_2: c_int = 227;
pub const BCM281XX_PIN_SYSCLKEN: c_int = 228;
pub const BCM281XX_PIN_TRACECLK: c_int = 229;
pub const BCM281XX_PIN_TRACEDT00: c_int = 230;
pub const BCM281XX_PIN_TRACEDT01: c_int = 231;
pub const BCM281XX_PIN_TRACEDT02: c_int = 232;
pub const BCM281XX_PIN_TRACEDT03: c_int = 233;
pub const BCM281XX_PIN_TRACEDT04: c_int = 234;
pub const BCM281XX_PIN_TRACEDT05: c_int = 235;
pub const BCM281XX_PIN_TRACEDT06: c_int = 236;
pub const BCM281XX_PIN_TRACEDT07: c_int = 237;
pub const BCM281XX_PIN_TRACEDT08: c_int = 238;
pub const BCM281XX_PIN_TRACEDT09: c_int = 239;
pub const BCM281XX_PIN_TRACEDT10: c_int = 240;
pub const BCM281XX_PIN_TRACEDT11: c_int = 241;
pub const BCM281XX_PIN_TRACEDT12: c_int = 242;
pub const BCM281XX_PIN_TRACEDT13: c_int = 243;
pub const BCM281XX_PIN_TRACEDT14: c_int = 244;
pub const BCM281XX_PIN_TRACEDT15: c_int = 245;
pub const BCM281XX_PIN_TXDATA3G0: c_int = 246;
pub const BCM281XX_PIN_TXPWRIND: c_int = 247;
pub const BCM281XX_PIN_UARTB1_UCTS: c_int = 248;
pub const BCM281XX_PIN_UARTB1_URTS: c_int = 249;
pub const BCM281XX_PIN_UARTB1_URXD: c_int = 250;
pub const BCM281XX_PIN_UARTB1_UTXD: c_int = 251;
pub const BCM281XX_PIN_UARTB2_URXD: c_int = 252;
pub const BCM281XX_PIN_UARTB2_UTXD: c_int = 253;
pub const BCM281XX_PIN_UARTB3_UCTS: c_int = 254;
pub const BCM281XX_PIN_UARTB3_URTS: c_int = 255;
pub const BCM281XX_PIN_UARTB3_URXD: c_int = 256;
pub const BCM281XX_PIN_UARTB3_UTXD: c_int = 257;
pub const BCM281XX_PIN_UARTB4_UCTS: c_int = 258;
pub const BCM281XX_PIN_UARTB4_URTS: c_int = 259;
pub const BCM281XX_PIN_UARTB4_URXD: c_int = 260;
pub const BCM281XX_PIN_UARTB4_UTXD: c_int = 261;
pub const BCM281XX_PIN_VC_CAM1_SCL: c_int = 262;
pub const BCM281XX_PIN_VC_CAM1_SDA: c_int = 263;
pub const BCM281XX_PIN_VC_CAM2_SCL: c_int = 264;
pub const BCM281XX_PIN_VC_CAM2_SDA: c_int = 265;
pub const BCM281XX_PIN_VC_CAM3_SCL: c_int = 266;
pub const BCM281XX_PIN_VC_CAM3_SDA: c_int = 267;

    { .number = a, .name = b, .drv_data = &c##_pin }
//
// Pin description definition.  The order here must be the same as defined in
// the PADCTRLREG block in the RDB, since the pin number is used as an index
// into this array.
//
    static const struct pinctrl_pin_desc bcm281xx_pinctrl_pins[] = {
    BCM281XX_PIN_DESC(BCM281XX_PIN_ADCSYNC, "adcsync", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_BAT_RM, "bat_rm", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_BSC1_SCL, "bsc1_scl", i2c),
    BCM281XX_PIN_DESC(BCM281XX_PIN_BSC1_SDA, "bsc1_sda", i2c),
    BCM281XX_PIN_DESC(BCM281XX_PIN_BSC2_SCL, "bsc2_scl", i2c),
    BCM281XX_PIN_DESC(BCM281XX_PIN_BSC2_SDA, "bsc2_sda", i2c),
    BCM281XX_PIN_DESC(BCM281XX_PIN_CLASSGPWR, "classgpwr", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_CLK_CX8, "clk_cx8", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_CLKOUT_0, "clkout_0", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_CLKOUT_1, "clkout_1", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_CLKOUT_2, "clkout_2", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_CLKOUT_3, "clkout_3", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_CLKREQ_IN_0, "clkreq_in_0", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_CLKREQ_IN_1, "clkreq_in_1", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_CWS_SYS_REQ1, "cws_sys_req1", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_CWS_SYS_REQ2, "cws_sys_req2", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_CWS_SYS_REQ3, "cws_sys_req3", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_DIGMIC1_CLK, "digmic1_clk", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_DIGMIC1_DQ, "digmic1_dq", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_DIGMIC2_CLK, "digmic2_clk", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_DIGMIC2_DQ, "digmic2_dq", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_GPEN13, "gpen13", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_GPEN14, "gpen14", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_GPEN15, "gpen15", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_GPIO00, "gpio00", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_GPIO01, "gpio01", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_GPIO02, "gpio02", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_GPIO03, "gpio03", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_GPIO04, "gpio04", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_GPIO05, "gpio05", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_GPIO06, "gpio06", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_GPIO07, "gpio07", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_GPIO08, "gpio08", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_GPIO09, "gpio09", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_GPIO10, "gpio10", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_GPIO11, "gpio11", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_GPIO12, "gpio12", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_GPIO13, "gpio13", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_GPIO14, "gpio14", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_GPS_PABLANK, "gps_pablank", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_GPS_TMARK, "gps_tmark", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_HDMI_SCL, "hdmi_scl", hdmi),
    BCM281XX_PIN_DESC(BCM281XX_PIN_HDMI_SDA, "hdmi_sda", hdmi),
    BCM281XX_PIN_DESC(BCM281XX_PIN_IC_DM, "ic_dm", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_IC_DP, "ic_dp", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_KP_COL_IP_0, "kp_col_ip_0", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_KP_COL_IP_1, "kp_col_ip_1", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_KP_COL_IP_2, "kp_col_ip_2", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_KP_COL_IP_3, "kp_col_ip_3", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_KP_ROW_OP_0, "kp_row_op_0", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_KP_ROW_OP_1, "kp_row_op_1", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_KP_ROW_OP_2, "kp_row_op_2", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_KP_ROW_OP_3, "kp_row_op_3", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_LCD_B_0, "lcd_b_0", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_LCD_B_1, "lcd_b_1", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_LCD_B_2, "lcd_b_2", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_LCD_B_3, "lcd_b_3", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_LCD_B_4, "lcd_b_4", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_LCD_B_5, "lcd_b_5", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_LCD_B_6, "lcd_b_6", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_LCD_B_7, "lcd_b_7", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_LCD_G_0, "lcd_g_0", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_LCD_G_1, "lcd_g_1", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_LCD_G_2, "lcd_g_2", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_LCD_G_3, "lcd_g_3", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_LCD_G_4, "lcd_g_4", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_LCD_G_5, "lcd_g_5", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_LCD_G_6, "lcd_g_6", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_LCD_G_7, "lcd_g_7", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_LCD_HSYNC, "lcd_hsync", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_LCD_OE, "lcd_oe", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_LCD_PCLK, "lcd_pclk", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_LCD_R_0, "lcd_r_0", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_LCD_R_1, "lcd_r_1", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_LCD_R_2, "lcd_r_2", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_LCD_R_3, "lcd_r_3", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_LCD_R_4, "lcd_r_4", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_LCD_R_5, "lcd_r_5", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_LCD_R_6, "lcd_r_6", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_LCD_R_7, "lcd_r_7", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_LCD_VSYNC, "lcd_vsync", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_MDMGPIO0, "mdmgpio0", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_MDMGPIO1, "mdmgpio1", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_MDMGPIO2, "mdmgpio2", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_MDMGPIO3, "mdmgpio3", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_MDMGPIO4, "mdmgpio4", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_MDMGPIO5, "mdmgpio5", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_MDMGPIO6, "mdmgpio6", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_MDMGPIO7, "mdmgpio7", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_MDMGPIO8, "mdmgpio8", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_MPHI_DATA_0, "mphi_data_0", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_MPHI_DATA_1, "mphi_data_1", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_MPHI_DATA_2, "mphi_data_2", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_MPHI_DATA_3, "mphi_data_3", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_MPHI_DATA_4, "mphi_data_4", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_MPHI_DATA_5, "mphi_data_5", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_MPHI_DATA_6, "mphi_data_6", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_MPHI_DATA_7, "mphi_data_7", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_MPHI_DATA_8, "mphi_data_8", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_MPHI_DATA_9, "mphi_data_9", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_MPHI_DATA_10, "mphi_data_10", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_MPHI_DATA_11, "mphi_data_11", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_MPHI_DATA_12, "mphi_data_12", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_MPHI_DATA_13, "mphi_data_13", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_MPHI_DATA_14, "mphi_data_14", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_MPHI_DATA_15, "mphi_data_15", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_MPHI_HA0, "mphi_ha0", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_MPHI_HAT0, "mphi_hat0", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_MPHI_HAT1, "mphi_hat1", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_MPHI_HCE0_N, "mphi_hce0_n", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_MPHI_HCE1_N, "mphi_hce1_n", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_MPHI_HRD_N, "mphi_hrd_n", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_MPHI_HWR_N, "mphi_hwr_n", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_MPHI_RUN0, "mphi_run0", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_MPHI_RUN1, "mphi_run1", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_MTX_SCAN_CLK, "mtx_scan_clk", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_MTX_SCAN_DATA, "mtx_scan_data", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_NAND_AD_0, "nand_ad_0", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_NAND_AD_1, "nand_ad_1", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_NAND_AD_2, "nand_ad_2", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_NAND_AD_3, "nand_ad_3", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_NAND_AD_4, "nand_ad_4", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_NAND_AD_5, "nand_ad_5", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_NAND_AD_6, "nand_ad_6", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_NAND_AD_7, "nand_ad_7", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_NAND_ALE, "nand_ale", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_NAND_CEN_0, "nand_cen_0", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_NAND_CEN_1, "nand_cen_1", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_NAND_CLE, "nand_cle", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_NAND_OEN, "nand_oen", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_NAND_RDY_0, "nand_rdy_0", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_NAND_RDY_1, "nand_rdy_1", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_NAND_WEN, "nand_wen", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_NAND_WP, "nand_wp", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_PC1, "pc1", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_PC2, "pc2", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_PMU_INT, "pmu_int", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_PMU_SCL, "pmu_scl", i2c),
    BCM281XX_PIN_DESC(BCM281XX_PIN_PMU_SDA, "pmu_sda", i2c),
    BCM281XX_PIN_DESC(BCM281XX_PIN_RFST2G_MTSLOTEN3G, "rfst2g_mtsloten3g",
    std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_RGMII_0_RX_CTL, "rgmii_0_rx_ctl", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_RGMII_0_RXC, "rgmii_0_rxc", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_RGMII_0_RXD_0, "rgmii_0_rxd_0", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_RGMII_0_RXD_1, "rgmii_0_rxd_1", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_RGMII_0_RXD_2, "rgmii_0_rxd_2", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_RGMII_0_RXD_3, "rgmii_0_rxd_3", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_RGMII_0_TX_CTL, "rgmii_0_tx_ctl", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_RGMII_0_TXC, "rgmii_0_txc", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_RGMII_0_TXD_0, "rgmii_0_txd_0", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_RGMII_0_TXD_1, "rgmii_0_txd_1", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_RGMII_0_TXD_2, "rgmii_0_txd_2", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_RGMII_0_TXD_3, "rgmii_0_txd_3", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_RGMII_1_RX_CTL, "rgmii_1_rx_ctl", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_RGMII_1_RXC, "rgmii_1_rxc", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_RGMII_1_RXD_0, "rgmii_1_rxd_0", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_RGMII_1_RXD_1, "rgmii_1_rxd_1", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_RGMII_1_RXD_2, "rgmii_1_rxd_2", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_RGMII_1_RXD_3, "rgmii_1_rxd_3", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_RGMII_1_TX_CTL, "rgmii_1_tx_ctl", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_RGMII_1_TXC, "rgmii_1_txc", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_RGMII_1_TXD_0, "rgmii_1_txd_0", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_RGMII_1_TXD_1, "rgmii_1_txd_1", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_RGMII_1_TXD_2, "rgmii_1_txd_2", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_RGMII_1_TXD_3, "rgmii_1_txd_3", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_RGMII_GPIO_0, "rgmii_gpio_0", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_RGMII_GPIO_1, "rgmii_gpio_1", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_RGMII_GPIO_2, "rgmii_gpio_2", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_RGMII_GPIO_3, "rgmii_gpio_3", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_RTXDATA2G_TXDATA3G1,
    "rtxdata2g_txdata3g1", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_RTXEN2G_TXDATA3G2, "rtxen2g_txdata3g2",
    std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_RXDATA3G0, "rxdata3g0", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_RXDATA3G1, "rxdata3g1", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_RXDATA3G2, "rxdata3g2", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SDIO1_CLK, "sdio1_clk", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SDIO1_CMD, "sdio1_cmd", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SDIO1_DATA_0, "sdio1_data_0", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SDIO1_DATA_1, "sdio1_data_1", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SDIO1_DATA_2, "sdio1_data_2", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SDIO1_DATA_3, "sdio1_data_3", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SDIO4_CLK, "sdio4_clk", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SDIO4_CMD, "sdio4_cmd", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SDIO4_DATA_0, "sdio4_data_0", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SDIO4_DATA_1, "sdio4_data_1", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SDIO4_DATA_2, "sdio4_data_2", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SDIO4_DATA_3, "sdio4_data_3", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SIM_CLK, "sim_clk", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SIM_DATA, "sim_data", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SIM_DET, "sim_det", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SIM_RESETN, "sim_resetn", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SIM2_CLK, "sim2_clk", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SIM2_DATA, "sim2_data", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SIM2_DET, "sim2_det", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SIM2_RESETN, "sim2_resetn", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SRI_C, "sri_c", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SRI_D, "sri_d", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SRI_E, "sri_e", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SSP_EXTCLK, "ssp_extclk", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SSP0_CLK, "ssp0_clk", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SSP0_FS, "ssp0_fs", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SSP0_RXD, "ssp0_rxd", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SSP0_TXD, "ssp0_txd", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SSP2_CLK, "ssp2_clk", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SSP2_FS_0, "ssp2_fs_0", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SSP2_FS_1, "ssp2_fs_1", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SSP2_FS_2, "ssp2_fs_2", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SSP2_FS_3, "ssp2_fs_3", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SSP2_RXD_0, "ssp2_rxd_0", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SSP2_RXD_1, "ssp2_rxd_1", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SSP2_TXD_0, "ssp2_txd_0", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SSP2_TXD_1, "ssp2_txd_1", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SSP3_CLK, "ssp3_clk", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SSP3_FS, "ssp3_fs", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SSP3_RXD, "ssp3_rxd", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SSP3_TXD, "ssp3_txd", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SSP4_CLK, "ssp4_clk", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SSP4_FS, "ssp4_fs", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SSP4_RXD, "ssp4_rxd", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SSP4_TXD, "ssp4_txd", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SSP5_CLK, "ssp5_clk", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SSP5_FS, "ssp5_fs", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SSP5_RXD, "ssp5_rxd", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SSP5_TXD, "ssp5_txd", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SSP6_CLK, "ssp6_clk", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SSP6_FS, "ssp6_fs", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SSP6_RXD, "ssp6_rxd", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SSP6_TXD, "ssp6_txd", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_STAT_1, "stat_1", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_STAT_2, "stat_2", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_SYSCLKEN, "sysclken", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_TRACECLK, "traceclk", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_TRACEDT00, "tracedt00", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_TRACEDT01, "tracedt01", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_TRACEDT02, "tracedt02", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_TRACEDT03, "tracedt03", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_TRACEDT04, "tracedt04", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_TRACEDT05, "tracedt05", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_TRACEDT06, "tracedt06", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_TRACEDT07, "tracedt07", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_TRACEDT08, "tracedt08", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_TRACEDT09, "tracedt09", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_TRACEDT10, "tracedt10", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_TRACEDT11, "tracedt11", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_TRACEDT12, "tracedt12", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_TRACEDT13, "tracedt13", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_TRACEDT14, "tracedt14", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_TRACEDT15, "tracedt15", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_TXDATA3G0, "txdata3g0", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_TXPWRIND, "txpwrind", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_UARTB1_UCTS, "uartb1_ucts", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_UARTB1_URTS, "uartb1_urts", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_UARTB1_URXD, "uartb1_urxd", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_UARTB1_UTXD, "uartb1_utxd", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_UARTB2_URXD, "uartb2_urxd", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_UARTB2_UTXD, "uartb2_utxd", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_UARTB3_UCTS, "uartb3_ucts", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_UARTB3_URTS, "uartb3_urts", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_UARTB3_URXD, "uartb3_urxd", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_UARTB3_UTXD, "uartb3_utxd", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_UARTB4_UCTS, "uartb4_ucts", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_UARTB4_URTS, "uartb4_urts", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_UARTB4_URXD, "uartb4_urxd", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_UARTB4_UTXD, "uartb4_utxd", std),
    BCM281XX_PIN_DESC(BCM281XX_PIN_VC_CAM1_SCL, "vc_cam1_scl", i2c),
    BCM281XX_PIN_DESC(BCM281XX_PIN_VC_CAM1_SDA, "vc_cam1_sda", i2c),
    BCM281XX_PIN_DESC(BCM281XX_PIN_VC_CAM2_SCL, "vc_cam2_scl", i2c),
    BCM281XX_PIN_DESC(BCM281XX_PIN_VC_CAM2_SDA, "vc_cam2_sda", i2c),
    BCM281XX_PIN_DESC(BCM281XX_PIN_VC_CAM3_SCL, "vc_cam3_scl", i2c),
    BCM281XX_PIN_DESC(BCM281XX_PIN_VC_CAM3_SDA, "vc_cam3_sda", i2c),
    };
    static const char * const bcm281xx_alt_groups[] = {
    "adcsync",
    "bat_rm",
    "bsc1_scl",
    "bsc1_sda",
    "bsc2_scl",
    "bsc2_sda",
    "classgpwr",
    "clk_cx8",
    "clkout_0",
    "clkout_1",
    "clkout_2",
    "clkout_3",
    "clkreq_in_0",
    "clkreq_in_1",
    "cws_sys_req1",
    "cws_sys_req2",
    "cws_sys_req3",
    "digmic1_clk",
    "digmic1_dq",
    "digmic2_clk",
    "digmic2_dq",
    "gpen13",
    "gpen14",
    "gpen15",
    "gpio00",
    "gpio01",
    "gpio02",
    "gpio03",
    "gpio04",
    "gpio05",
    "gpio06",
    "gpio07",
    "gpio08",
    "gpio09",
    "gpio10",
    "gpio11",
    "gpio12",
    "gpio13",
    "gpio14",
    "gps_pablank",
    "gps_tmark",
    "hdmi_scl",
    "hdmi_sda",
    "ic_dm",
    "ic_dp",
    "kp_col_ip_0",
    "kp_col_ip_1",
    "kp_col_ip_2",
    "kp_col_ip_3",
    "kp_row_op_0",
    "kp_row_op_1",
    "kp_row_op_2",
    "kp_row_op_3",
    "lcd_b_0",
    "lcd_b_1",
    "lcd_b_2",
    "lcd_b_3",
    "lcd_b_4",
    "lcd_b_5",
    "lcd_b_6",
    "lcd_b_7",
    "lcd_g_0",
    "lcd_g_1",
    "lcd_g_2",
    "lcd_g_3",
    "lcd_g_4",
    "lcd_g_5",
    "lcd_g_6",
    "lcd_g_7",
    "lcd_hsync",
    "lcd_oe",
    "lcd_pclk",
    "lcd_r_0",
    "lcd_r_1",
    "lcd_r_2",
    "lcd_r_3",
    "lcd_r_4",
    "lcd_r_5",
    "lcd_r_6",
    "lcd_r_7",
    "lcd_vsync",
    "mdmgpio0",
    "mdmgpio1",
    "mdmgpio2",
    "mdmgpio3",
    "mdmgpio4",
    "mdmgpio5",
    "mdmgpio6",
    "mdmgpio7",
    "mdmgpio8",
    "mphi_data_0",
    "mphi_data_1",
    "mphi_data_2",
    "mphi_data_3",
    "mphi_data_4",
    "mphi_data_5",
    "mphi_data_6",
    "mphi_data_7",
    "mphi_data_8",
    "mphi_data_9",
    "mphi_data_10",
    "mphi_data_11",
    "mphi_data_12",
    "mphi_data_13",
    "mphi_data_14",
    "mphi_data_15",
    "mphi_ha0",
    "mphi_hat0",
    "mphi_hat1",
    "mphi_hce0_n",
    "mphi_hce1_n",
    "mphi_hrd_n",
    "mphi_hwr_n",
    "mphi_run0",
    "mphi_run1",
    "mtx_scan_clk",
    "mtx_scan_data",
    "nand_ad_0",
    "nand_ad_1",
    "nand_ad_2",
    "nand_ad_3",
    "nand_ad_4",
    "nand_ad_5",
    "nand_ad_6",
    "nand_ad_7",
    "nand_ale",
    "nand_cen_0",
    "nand_cen_1",
    "nand_cle",
    "nand_oen",
    "nand_rdy_0",
    "nand_rdy_1",
    "nand_wen",
    "nand_wp",
    "pc1",
    "pc2",
    "pmu_int",
    "pmu_scl",
    "pmu_sda",
    "rfst2g_mtsloten3g",
    "rgmii_0_rx_ctl",
    "rgmii_0_rxc",
    "rgmii_0_rxd_0",
    "rgmii_0_rxd_1",
    "rgmii_0_rxd_2",
    "rgmii_0_rxd_3",
    "rgmii_0_tx_ctl",
    "rgmii_0_txc",
    "rgmii_0_txd_0",
    "rgmii_0_txd_1",
    "rgmii_0_txd_2",
    "rgmii_0_txd_3",
    "rgmii_1_rx_ctl",
    "rgmii_1_rxc",
    "rgmii_1_rxd_0",
    "rgmii_1_rxd_1",
    "rgmii_1_rxd_2",
    "rgmii_1_rxd_3",
    "rgmii_1_tx_ctl",
    "rgmii_1_txc",
    "rgmii_1_txd_0",
    "rgmii_1_txd_1",
    "rgmii_1_txd_2",
    "rgmii_1_txd_3",
    "rgmii_gpio_0",
    "rgmii_gpio_1",
    "rgmii_gpio_2",
    "rgmii_gpio_3",
    "rtxdata2g_txdata3g1",
    "rtxen2g_txdata3g2",
    "rxdata3g0",
    "rxdata3g1",
    "rxdata3g2",
    "sdio1_clk",
    "sdio1_cmd",
    "sdio1_data_0",
    "sdio1_data_1",
    "sdio1_data_2",
    "sdio1_data_3",
    "sdio4_clk",
    "sdio4_cmd",
    "sdio4_data_0",
    "sdio4_data_1",
    "sdio4_data_2",
    "sdio4_data_3",
    "sim_clk",
    "sim_data",
    "sim_det",
    "sim_resetn",
    "sim2_clk",
    "sim2_data",
    "sim2_det",
    "sim2_resetn",
    "sri_c",
    "sri_d",
    "sri_e",
    "ssp_extclk",
    "ssp0_clk",
    "ssp0_fs",
    "ssp0_rxd",
    "ssp0_txd",
    "ssp2_clk",
    "ssp2_fs_0",
    "ssp2_fs_1",
    "ssp2_fs_2",
    "ssp2_fs_3",
    "ssp2_rxd_0",
    "ssp2_rxd_1",
    "ssp2_txd_0",
    "ssp2_txd_1",
    "ssp3_clk",
    "ssp3_fs",
    "ssp3_rxd",
    "ssp3_txd",
    "ssp4_clk",
    "ssp4_fs",
    "ssp4_rxd",
    "ssp4_txd",
    "ssp5_clk",
    "ssp5_fs",
    "ssp5_rxd",
    "ssp5_txd",
    "ssp6_clk",
    "ssp6_fs",
    "ssp6_rxd",
    "ssp6_txd",
    "stat_1",
    "stat_2",
    "sysclken",
    "traceclk",
    "tracedt00",
    "tracedt01",
    "tracedt02",
    "tracedt03",
    "tracedt04",
    "tracedt05",
    "tracedt06",
    "tracedt07",
    "tracedt08",
    "tracedt09",
    "tracedt10",
    "tracedt11",
    "tracedt12",
    "tracedt13",
    "tracedt14",
    "tracedt15",
    "txdata3g0",
    "txpwrind",
    "uartb1_ucts",
    "uartb1_urts",
    "uartb1_urxd",
    "uartb1_utxd",
    "uartb2_urxd",
    "uartb2_utxd",
    "uartb3_ucts",
    "uartb3_urts",
    "uartb3_urxd",
    "uartb3_utxd",
    "uartb4_ucts",
    "uartb4_urts",
    "uartb4_urxd",
    "uartb4_utxd",
    "vc_cam1_scl",
    "vc_cam1_sda",
    "vc_cam2_scl",
    "vc_cam2_sda",
    "vc_cam3_scl",
    "vc_cam3_sda",
    };
// Every pin can implement all ALT1-ALT4 functions

    {							\
    .name = #fcn_name,				\
    .groups = bcm281xx_alt_groups,			\
    .ngroups = ARRAY_SIZE(bcm281xx_alt_groups),	\
    }
    static const struct bcm281xx_pin_function bcm281xx_functions[] = {
    BCM281XX_PIN_FUNCTION(alt1),
    BCM281XX_PIN_FUNCTION(alt2),
    BCM281XX_PIN_FUNCTION(alt3),
    BCM281XX_PIN_FUNCTION(alt4),
    };
    static const struct regmap_config bcm281xx_pinctrl_regmap_config = {
    .reg_bits = 32,
    .reg_stride = 4,
    .val_bits = 32,
    .max_register = BCM281XX_PIN_VC_CAM3_SDA * 4,
    };
    static const struct bcm281xx_pinctrl_info bcm281xx_pinctrl = {
    .device_type = BCM281XX_PINCTRL_TYPE,
    .pins = bcm281xx_pinctrl_pins,
    .npins = ARRAY_SIZE(bcm281xx_pinctrl_pins),
    .functions = bcm281xx_functions,
    .nfunctions = ARRAY_SIZE(bcm281xx_functions),
    .regmap_config = &bcm281xx_pinctrl_regmap_config,
    };
// BCM21664 data
pub const BCM21664_PIN_ADCSYN: c_int = 0;
pub const BCM21664_PIN_BATRM: c_int = 1;
pub const BCM21664_PIN_BSC1CLK: c_int = 2;
pub const BCM21664_PIN_BSC1DAT: c_int = 3;
pub const BCM21664_PIN_CAMCS0: c_int = 4;
pub const BCM21664_PIN_CAMCS1: c_int = 5;
pub const BCM21664_PIN_CLK32K: c_int = 6;
pub const BCM21664_PIN_CLK_CX8: c_int = 7;
pub const BCM21664_PIN_DCLK1: c_int = 8;
pub const BCM21664_PIN_DCLK4: c_int = 9;
pub const BCM21664_PIN_DCLKREQ1: c_int = 10;
pub const BCM21664_PIN_DCLKREQ4: c_int = 11;
pub const BCM21664_PIN_DMIC0CLK: c_int = 12;
pub const BCM21664_PIN_DMIC0DQ: c_int = 13;
pub const BCM21664_PIN_DSI0TE: c_int = 14;
pub const BCM21664_PIN_GPIO00: c_int = 15;
pub const BCM21664_PIN_GPIO01: c_int = 16;
pub const BCM21664_PIN_GPIO02: c_int = 17;
pub const BCM21664_PIN_GPIO03: c_int = 18;
pub const BCM21664_PIN_GPIO04: c_int = 19;
pub const BCM21664_PIN_GPIO05: c_int = 20;
pub const BCM21664_PIN_GPIO06: c_int = 21;
pub const BCM21664_PIN_GPIO07: c_int = 22;
pub const BCM21664_PIN_GPIO08: c_int = 23;
pub const BCM21664_PIN_GPIO09: c_int = 24;
pub const BCM21664_PIN_GPIO10: c_int = 25;
pub const BCM21664_PIN_GPIO11: c_int = 26;
pub const BCM21664_PIN_GPIO12: c_int = 27;
pub const BCM21664_PIN_GPIO13: c_int = 28;
pub const BCM21664_PIN_GPIO14: c_int = 29;
pub const BCM21664_PIN_GPIO15: c_int = 30;
pub const BCM21664_PIN_GPIO16: c_int = 31;
pub const BCM21664_PIN_GPIO17: c_int = 32;
pub const BCM21664_PIN_GPIO18: c_int = 33;
pub const BCM21664_PIN_GPIO19: c_int = 34;
pub const BCM21664_PIN_GPIO20: c_int = 35;
pub const BCM21664_PIN_GPIO21: c_int = 36;
pub const BCM21664_PIN_GPIO22: c_int = 37;
pub const BCM21664_PIN_GPIO23: c_int = 38;
pub const BCM21664_PIN_GPIO24: c_int = 39;
pub const BCM21664_PIN_GPIO25: c_int = 40;
pub const BCM21664_PIN_GPIO26: c_int = 41;
pub const BCM21664_PIN_GPIO27: c_int = 42;
pub const BCM21664_PIN_GPIO28: c_int = 43;
pub const BCM21664_PIN_GPIO32: c_int = 44;
pub const BCM21664_PIN_GPIO33: c_int = 45;
pub const BCM21664_PIN_GPIO34: c_int = 46;
pub const BCM21664_PIN_GPS_CALREQ: c_int = 47;
pub const BCM21664_PIN_GPS_HOSTREQ: c_int = 48;
pub const BCM21664_PIN_GPS_PABLANK: c_int = 49;
pub const BCM21664_PIN_GPS_TMARK: c_int = 50;
pub const BCM21664_PIN_ICUSBDM: c_int = 51;
pub const BCM21664_PIN_ICUSBDP: c_int = 52;
pub const BCM21664_PIN_LCDCS0: c_int = 53;
pub const BCM21664_PIN_LCDRES: c_int = 54;
pub const BCM21664_PIN_LCDSCL: c_int = 55;
pub const BCM21664_PIN_LCDSDA: c_int = 56;
pub const BCM21664_PIN_LCDTE: c_int = 57;
pub const BCM21664_PIN_MDMGPIO00: c_int = 58;
pub const BCM21664_PIN_MDMGPIO01: c_int = 59;
pub const BCM21664_PIN_MDMGPIO02: c_int = 60;
pub const BCM21664_PIN_MDMGPIO03: c_int = 61;
pub const BCM21664_PIN_MDMGPIO04: c_int = 62;
pub const BCM21664_PIN_MDMGPIO05: c_int = 63;
pub const BCM21664_PIN_MDMGPIO06: c_int = 64;
pub const BCM21664_PIN_MDMGPIO07: c_int = 65;
pub const BCM21664_PIN_MDMGPIO08: c_int = 66;
pub const BCM21664_PIN_MMC0CK: c_int = 67;
pub const BCM21664_PIN_MMC0CMD: c_int = 68;
pub const BCM21664_PIN_MMC0DAT0: c_int = 69;
pub const BCM21664_PIN_MMC0DAT1: c_int = 70;
pub const BCM21664_PIN_MMC0DAT2: c_int = 71;
pub const BCM21664_PIN_MMC0DAT3: c_int = 72;
pub const BCM21664_PIN_MMC0DAT4: c_int = 73;
pub const BCM21664_PIN_MMC0DAT5: c_int = 74;
pub const BCM21664_PIN_MMC0DAT6: c_int = 75;
pub const BCM21664_PIN_MMC0DAT7: c_int = 76;
pub const BCM21664_PIN_MMC0RST: c_int = 77;
pub const BCM21664_PIN_MMC1CK: c_int = 78;
pub const BCM21664_PIN_MMC1CMD: c_int = 79;
pub const BCM21664_PIN_MMC1DAT0: c_int = 80;
pub const BCM21664_PIN_MMC1DAT1: c_int = 81;
pub const BCM21664_PIN_MMC1DAT2: c_int = 82;
pub const BCM21664_PIN_MMC1DAT3: c_int = 83;
pub const BCM21664_PIN_MMC1DAT4: c_int = 84;
pub const BCM21664_PIN_MMC1DAT5: c_int = 85;
pub const BCM21664_PIN_MMC1DAT6: c_int = 86;
pub const BCM21664_PIN_MMC1DAT7: c_int = 87;
pub const BCM21664_PIN_MMC1RST: c_int = 88;
pub const BCM21664_PIN_PC1: c_int = 89;
pub const BCM21664_PIN_PC2: c_int = 90;
pub const BCM21664_PIN_PMBSCCLK: c_int = 91;
pub const BCM21664_PIN_PMBSCDAT: c_int = 92;
pub const BCM21664_PIN_PMUINT: c_int = 93;
pub const BCM21664_PIN_RESETN: c_int = 94;
pub const BCM21664_PIN_RFST2G_MTSLOTEN3G: c_int = 95;
pub const BCM21664_PIN_RTXDATA2G_TXDATA3G1: c_int = 96;
pub const BCM21664_PIN_RTXEN2G_TXDATA3G2: c_int = 97;
pub const BCM21664_PIN_RXDATA3G0: c_int = 98;
pub const BCM21664_PIN_RXDATA3G1: c_int = 99;
pub const BCM21664_PIN_RXDATA3G2: c_int = 100;
pub const BCM21664_PIN_SDCK: c_int = 101;
pub const BCM21664_PIN_SDCMD: c_int = 102;
pub const BCM21664_PIN_SDDAT0: c_int = 103;
pub const BCM21664_PIN_SDDAT1: c_int = 104;
pub const BCM21664_PIN_SDDAT2: c_int = 105;
pub const BCM21664_PIN_SDDAT3: c_int = 106;
pub const BCM21664_PIN_SIMCLK: c_int = 107;
pub const BCM21664_PIN_SIMDAT: c_int = 108;
pub const BCM21664_PIN_SIMDET: c_int = 109;
pub const BCM21664_PIN_SIMRST: c_int = 110;
pub const BCM21664_PIN_GPIO93: c_int = 111;
pub const BCM21664_PIN_GPIO94: c_int = 112;
pub const BCM21664_PIN_SPI0CLK: c_int = 113;
pub const BCM21664_PIN_SPI0FSS: c_int = 114;
pub const BCM21664_PIN_SPI0RXD: c_int = 115;
pub const BCM21664_PIN_SPI0TXD: c_int = 116;
pub const BCM21664_PIN_SRI_C: c_int = 117;
pub const BCM21664_PIN_SRI_D: c_int = 118;
pub const BCM21664_PIN_SRI_E: c_int = 119;
pub const BCM21664_PIN_SSPCK: c_int = 120;
pub const BCM21664_PIN_SSPDI: c_int = 121;
pub const BCM21664_PIN_SSPDO: c_int = 122;
pub const BCM21664_PIN_SSPSYN: c_int = 123;
pub const BCM21664_PIN_STAT1: c_int = 124;
pub const BCM21664_PIN_STAT2: c_int = 125;
pub const BCM21664_PIN_SWCLKTCK: c_int = 126;
pub const BCM21664_PIN_SWDIOTMS: c_int = 127;
pub const BCM21664_PIN_SYSCLKEN: c_int = 128;
pub const BCM21664_PIN_TDI: c_int = 129;
pub const BCM21664_PIN_TDO: c_int = 130;
pub const BCM21664_PIN_TESTMODE: c_int = 131;
pub const BCM21664_PIN_TRACECLK: c_int = 132;
pub const BCM21664_PIN_TRACEDT00: c_int = 133;
pub const BCM21664_PIN_TRACEDT01: c_int = 134;
pub const BCM21664_PIN_TRACEDT02: c_int = 135;
pub const BCM21664_PIN_TRACEDT03: c_int = 136;
pub const BCM21664_PIN_TRACEDT04: c_int = 137;
pub const BCM21664_PIN_TRACEDT05: c_int = 138;
pub const BCM21664_PIN_TRACEDT06: c_int = 139;
pub const BCM21664_PIN_TRACEDT07: c_int = 140;
pub const BCM21664_PIN_TRSTB: c_int = 141;
pub const BCM21664_PIN_TXDATA3G0: c_int = 142;
pub const BCM21664_PIN_UBCTSN: c_int = 143;
pub const BCM21664_PIN_UBRTSN: c_int = 144;
pub const BCM21664_PIN_UBRX: c_int = 145;
pub const BCM21664_PIN_UBTX: c_int = 146;
pub const BCM21664_PIN_TRACEDT08: c_int = 147;
pub const BCM21664_PIN_TRACEDT09: c_int = 148;
pub const BCM21664_PIN_TRACEDT10: c_int = 149;
pub const BCM21664_PIN_TRACEDT11: c_int = 150;
pub const BCM21664_PIN_TRACEDT12: c_int = 151;
pub const BCM21664_PIN_TRACEDT13: c_int = 152;
pub const BCM21664_PIN_TRACEDT14: c_int = 153;
pub const BCM21664_PIN_TRACEDT15: c_int = 154;
    static const struct pinctrl_pin_desc bcm21664_pinctrl_pins[] = {
    BCM281XX_PIN_DESC(BCM21664_PIN_ADCSYN, "adcsyn", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_BATRM, "batrm", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_BSC1CLK, "bsc1clk", i2c),
    BCM281XX_PIN_DESC(BCM21664_PIN_BSC1DAT, "bsc1dat", i2c),
    BCM281XX_PIN_DESC(BCM21664_PIN_CAMCS0, "camcs0", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_CAMCS1, "camcs1", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_CLK32K, "clk32k", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_CLK_CX8, "clk_cx8", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_DCLK1, "dclk1", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_DCLK4, "dclk4", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_DCLKREQ1, "dclkreq1", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_DCLKREQ4, "dclkreq4", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_DMIC0CLK, "dmic0clk", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_DMIC0DQ, "dmic0dq", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_DSI0TE, "dsi0te", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_GPIO00, "gpio00", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_GPIO01, "gpio01", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_GPIO02, "gpio02", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_GPIO03, "gpio03", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_GPIO04, "gpio04", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_GPIO05, "gpio05", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_GPIO06, "gpio06", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_GPIO07, "gpio07", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_GPIO08, "gpio08", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_GPIO09, "gpio09", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_GPIO10, "gpio10", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_GPIO11, "gpio11", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_GPIO12, "gpio12", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_GPIO13, "gpio13", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_GPIO14, "gpio14", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_GPIO15, "gpio15", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_GPIO16, "gpio16", i2c),
    BCM281XX_PIN_DESC(BCM21664_PIN_GPIO17, "gpio17", i2c),
    BCM281XX_PIN_DESC(BCM21664_PIN_GPIO18, "gpio18", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_GPIO19, "gpio19", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_GPIO20, "gpio20", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_GPIO21, "gpio21", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_GPIO22, "gpio22", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_GPIO23, "gpio23", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_GPIO24, "gpio24", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_GPIO25, "gpio25", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_GPIO26, "gpio26", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_GPIO27, "gpio27", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_GPIO28, "gpio28", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_GPIO32, "gpio32", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_GPIO33, "gpio33", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_GPIO34, "gpio34", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_GPS_CALREQ, "gps_calreq", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_GPS_HOSTREQ, "gps_hostreq", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_GPS_PABLANK, "gps_pablank", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_GPS_TMARK, "gps_tmark", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_ICUSBDM, "icusbdm", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_ICUSBDP, "icusbdp", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_LCDCS0, "lcdcs0", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_LCDRES, "lcdres", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_LCDSCL, "lcdscl", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_LCDSDA, "lcdsda", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_LCDTE, "lcdte", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_MDMGPIO00, "mdmgpio00", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_MDMGPIO01, "mdmgpio01", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_MDMGPIO02, "mdmgpio02", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_MDMGPIO03, "mdmgpio03", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_MDMGPIO04, "mdmgpio04", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_MDMGPIO05, "mdmgpio05", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_MDMGPIO06, "mdmgpio06", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_MDMGPIO07, "mdmgpio07", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_MDMGPIO08, "mdmgpio08", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_MMC0CK, "mmc0ck", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_MMC0CMD, "mmc0cmd", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_MMC0DAT0, "mmc0dat0", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_MMC0DAT1, "mmc0dat1", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_MMC0DAT2, "mmc0dat2", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_MMC0DAT3, "mmc0dat3", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_MMC0DAT4, "mmc0dat4", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_MMC0DAT5, "mmc0dat5", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_MMC0DAT6, "mmc0dat6", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_MMC0DAT7, "mmc0dat7", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_MMC0RST, "mmc0rst", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_MMC1CK, "mmc1ck", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_MMC1CMD, "mmc1cmd", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_MMC1DAT0, "mmc1dat0", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_MMC1DAT1, "mmc1dat1", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_MMC1DAT2, "mmc1dat2", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_MMC1DAT3, "mmc1dat3", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_MMC1DAT4, "mmc1dat4", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_MMC1DAT5, "mmc1dat5", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_MMC1DAT6, "mmc1dat6", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_MMC1DAT7, "mmc1dat7", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_MMC1RST, "mmc1rst", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_PC1, "pc1", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_PC2, "pc2", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_PMBSCCLK, "pmbscclk", i2c),
    BCM281XX_PIN_DESC(BCM21664_PIN_PMBSCDAT, "pmbscdat", i2c),
    BCM281XX_PIN_DESC(BCM21664_PIN_PMUINT, "pmuint", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_RESETN, "resetn", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_RFST2G_MTSLOTEN3G, "rfst2g_mtsloten3g", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_RTXDATA2G_TXDATA3G1, "rtxdata2g_txdata3g1", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_RTXEN2G_TXDATA3G2, "rtxen2g_txdata3g2", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_RXDATA3G0, "rxdata3g0", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_RXDATA3G1, "rxdata3g1", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_RXDATA3G2, "rxdata3g2", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_SDCK, "sdck", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_SDCMD, "sdcmd", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_SDDAT0, "sddat0", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_SDDAT1, "sddat1", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_SDDAT2, "sddat2", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_SDDAT3, "sddat3", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_SIMCLK, "simclk", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_SIMDAT, "simdat", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_SIMDET, "simdet", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_SIMRST, "simrst", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_GPIO93, "gpio93", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_GPIO94, "gpio94", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_SPI0CLK, "spi0clk", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_SPI0FSS, "spi0fss", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_SPI0RXD, "spi0rxd", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_SPI0TXD, "spi0txd", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_SRI_C, "sri_c", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_SRI_D, "sri_d", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_SRI_E, "sri_e", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_SSPCK, "sspck", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_SSPDI, "sspdi", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_SSPDO, "sspdo", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_SSPSYN, "sspsyn", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_STAT1, "stat1", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_STAT2, "stat2", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_SWCLKTCK, "swclktck", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_SWDIOTMS, "swdiotms", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_SYSCLKEN, "sysclken", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_TDI, "tdi", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_TDO, "tdo", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_TESTMODE, "testmode", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_TRACECLK, "traceclk", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_TRACEDT00, "tracedt00", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_TRACEDT01, "tracedt01", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_TRACEDT02, "tracedt02", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_TRACEDT03, "tracedt03", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_TRACEDT04, "tracedt04", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_TRACEDT05, "tracedt05", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_TRACEDT06, "tracedt06", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_TRACEDT07, "tracedt07", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_TRSTB, "trstb", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_TXDATA3G0, "txdata3g0", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_UBCTSN, "ubctsn", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_UBRTSN, "ubrtsn", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_UBRX, "ubrx", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_UBTX, "ubtx", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_TRACEDT08, "tracedt08", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_TRACEDT09, "tracedt09", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_TRACEDT10, "tracedt10", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_TRACEDT11, "tracedt11", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_TRACEDT12, "tracedt12", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_TRACEDT13, "tracedt13", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_TRACEDT14, "tracedt14", std),
    BCM281XX_PIN_DESC(BCM21664_PIN_TRACEDT15, "tracedt15", std),
    };
    static const char * const bcm21664_alt_groups[] = {
    "adcsyn",
    "batrm",
    "bsc1clk",
    "bsc1dat",
    "camcs0",
    "camcs1",
    "clk32k",
    "clk_cx8",
    "dclk1",
    "dclk4",
    "dclkreq1",
    "dclkreq4",
    "dmic0clk",
    "dmic0dq",
    "dsi0te",
    "gpio00",
    "gpio01",
    "gpio02",
    "gpio03",
    "gpio04",
    "gpio05",
    "gpio06",
    "gpio07",
    "gpio08",
    "gpio09",
    "gpio10",
    "gpio11",
    "gpio12",
    "gpio13",
    "gpio14",
    "gpio15",
    "gpio16",
    "gpio17",
    "gpio18",
    "gpio19",
    "gpio20",
    "gpio21",
    "gpio22",
    "gpio23",
    "gpio24",
    "gpio25",
    "gpio26",
    "gpio27",
    "gpio28",
    "gpio32",
    "gpio33",
    "gpio34",
    "gps_calreq",
    "gps_hostreq",
    "gps_pablank",
    "gps_tmark",
    "icusbdm",
    "icusbdp",
    "lcdcs0",
    "lcdres",
    "lcdscl",
    "lcdsda",
    "lcdte",
    "mdmgpio00",
    "mdmgpio01",
    "mdmgpio02",
    "mdmgpio03",
    "mdmgpio04",
    "mdmgpio05",
    "mdmgpio06",
    "mdmgpio07",
    "mdmgpio08",
    "mmc0ck",
    "mmc0cmd",
    "mmc0dat0",
    "mmc0dat1",
    "mmc0dat2",
    "mmc0dat3",
    "mmc0dat4",
    "mmc0dat5",
    "mmc0dat6",
    "mmc0dat7",
    "mmc0rst",
    "mmc1ck",
    "mmc1cmd",
    "mmc1dat0",
    "mmc1dat1",
    "mmc1dat2",
    "mmc1dat3",
    "mmc1dat4",
    "mmc1dat5",
    "mmc1dat6",
    "mmc1dat7",
    "mmc1rst",
    "pc1",
    "pc2",
    "pmbscclk",
    "pmbscdat",
    "pmuint",
    "resetn",
    "rfst2g_mtsloten3g",
    "rtxdata2g_txdata3g1",
    "rtxen2g_txdata3g2",
    "rxdata3g0",
    "rxdata3g1",
    "rxdata3g2",
    "sdck",
    "sdcmd",
    "sddat0",
    "sddat1",
    "sddat2",
    "sddat3",
    "simclk",
    "simdat",
    "simdet",
    "simrst",
    "gpio93",
    "gpio94",
    "spi0clk",
    "spi0fss",
    "spi0rxd",
    "spi0txd",
    "sri_c",
    "sri_d",
    "sri_e",
    "sspck",
    "sspdi",
    "sspdo",
    "sspsyn",
    "stat1",
    "stat2",
    "swclktck",
    "swdiotms",
    "sysclken",
    "tdi",
    "tdo",
    "testmode",
    "traceclk",
    "tracedt00",
    "tracedt01",
    "tracedt02",
    "tracedt03",
    "tracedt04",
    "tracedt05",
    "tracedt06",
    "tracedt07",
    "trstb",
    "txdata3g0",
    "ubctsn",
    "ubrtsn",
    "ubrx",
    "ubtx",
    "tracedt08",
    "tracedt09",
    "tracedt10",
    "tracedt11",
    "tracedt12",
    "tracedt13",
    "tracedt14",
    "tracedt15",
    };

    {							\
    .name = #fcn_name,				\
    .groups = bcm21664_alt_groups,			\
    .ngroups = ARRAY_SIZE(bcm21664_alt_groups),	\
    }
    static const struct bcm281xx_pin_function bcm21664_functions[] = {
    BCM21664_PIN_FUNCTION(alt1),
    BCM21664_PIN_FUNCTION(alt2),
    BCM21664_PIN_FUNCTION(alt3),
    BCM21664_PIN_FUNCTION(alt4),
    BCM21664_PIN_FUNCTION(alt5),
    BCM21664_PIN_FUNCTION(alt6),
    };
    static const struct regmap_config bcm21664_pinctrl_regmap_config = {
    .reg_bits = 32,
    .reg_stride = 4,
    .val_bits = 32,
    .max_register = BCM21664_WR_ACCESS_OFFSET,
    };
    static const struct bcm281xx_pinctrl_info bcm21664_pinctrl = {
    .device_type = BCM21664_PINCTRL_TYPE,
    .pins = bcm21664_pinctrl_pins,
    .npins = ARRAY_SIZE(bcm21664_pinctrl_pins),
    .functions = bcm21664_functions,
    .nfunctions = ARRAY_SIZE(bcm21664_functions),
    .regmap_config = &bcm21664_pinctrl_regmap_config,
    };
// BCM21664 pinctrl access lock handlers
#[no_mangle]
unsafe extern "C" fn bcm21664_pinctrl_lock_all(pdata: *mut bcm281xx_pinctrl_data) -> c_int {
    static int bcm21664_pinctrl_lock_all(struct bcm281xx_pinctrl_data *pdata)
    {
    int i, rc;
    for (i = 0; i < BCM21664_ACCESS_LOCK_COUNT; i++) {
    rc = regmap_write(pdata.regmap, BCM21664_WR_ACCESS_OFFSET,
    BCM21664_WR_ACCESS_PASSWORD);
    if (rc) {
    dev_err(pdata.dev, "Failed to enable write access: %d\n",
    rc);
    return rc;
    }
    rc = regmap_write(pdata.regmap, BCM21664_ACCESS_LOCK_OFFSET(i),
    0xffffffff);
    if (rc) {
    dev_err(pdata.dev, "Failed to write access lock: %d\n",
    rc);
    return rc;
    }
    }
    return 0;
    }
    static int bcm21664_pinctrl_set_pin_lock(struct bcm281xx_pinctrl_data *pdata,
    unsigned int pin, bool lock)
    {
    let mut access_lock: c_uint = pin / 32;
    int rc;
    dev_dbg(pdata.dev,
    "%s(): %s pin %s (%d)\n",
    __func__, lock ? "Lock" : "Unlock", pdata.info.pins[pin].name,
    pin);
    rc = regmap_write(pdata.regmap, BCM21664_WR_ACCESS_OFFSET,
    BCM21664_WR_ACCESS_PASSWORD);
    if (rc) {
    dev_err(pdata.dev, "Failed to enable write access: %d\n",
    rc);
    return rc;
    }
    rc = regmap_update_bits(pdata.regmap,
    BCM21664_ACCESS_LOCK_OFFSET(access_lock),
    BIT(pin % 32),
    (int)lock << (pin % 32));
    if (rc) {
    dev_err(pdata.dev, "Failed to %s pin: %d\n",
    lock ? "lock" : "unlock", rc);
    return rc;
    }
    return 0;
    }
    static inline enum bcm281xx_pin_type pin_type_get(struct pinctrl_dev *pctldev,
    unsigned int pin)
    {
    struct bcm281xx_pinctrl_data *pdata = pinctrl_dev_get_drvdata(pctldev);
    if (pin >= pdata.info.npins)
    return BCM281XX_PIN_TYPE_UNKNOWN;
    return *(enum bcm281xx_pin_type *)(pdata.info.pins[pin].drv_data);
    }

    (BCM281XX_ ## type ## _PIN_REG_ ## param ## _SHIFT)

    (BCM281XX_ ## type ## _PIN_REG_ ## param ## _MASK)
//
// This helper function is used to build up the value and mask used to write to
// a pin register, but does not actually write to the register.
//
    static inline void bcm281xx_pin_update(u32 *reg_val, u32 *reg_mask,
    u32 param_val, u32 param_shift,
    u32 param_mask)
    {
// reg_val &= ~param_mask;
// reg_val |= (param_val << param_shift) & param_mask;
// reg_mask |= param_mask;
    }
#[no_mangle]
unsafe extern "C" fn bcm281xx_pinctrl_get_groups_count(pctldev: *mut pinctrl_dev) -> c_int {
    static int bcm281xx_pinctrl_get_groups_count(struct pinctrl_dev *pctldev)
    {
    struct bcm281xx_pinctrl_data *pdata = pinctrl_dev_get_drvdata(pctldev);
    return pdata.info.npins;
    }
    static const char *bcm281xx_pinctrl_get_group_name(struct pinctrl_dev *pctldev,
    unsigned int group)
    {
    struct bcm281xx_pinctrl_data *pdata = pinctrl_dev_get_drvdata(pctldev);
    return pdata.info.pins[group].name;
    }
    static int bcm281xx_pinctrl_get_group_pins(struct pinctrl_dev *pctldev,
    unsigned int group,
    const unsigned **pins,
    unsigned int *num_pins)
    {
    struct bcm281xx_pinctrl_data *pdata = pinctrl_dev_get_drvdata(pctldev);
// pins = &pdata->info->pins[group].number;
// num_pins = 1;
    return 0;
    }
    static void bcm281xx_pinctrl_pin_dbg_show(struct pinctrl_dev *pctldev,
    struct seq_file *s,
    unsigned int offset)
    {
    seq_printf(s, " %s", dev_name(pctldev.dev));
    }
    static const struct pinctrl_ops bcm281xx_pinctrl_ops = {
    .get_groups_count = bcm281xx_pinctrl_get_groups_count,
    .get_group_name = bcm281xx_pinctrl_get_group_name,
    .get_group_pins = bcm281xx_pinctrl_get_group_pins,
    .pin_dbg_show = bcm281xx_pinctrl_pin_dbg_show,
    .dt_node_to_map = pinconf_generic_dt_node_to_map_pin,
    .dt_free_map = pinctrl_utils_free_map,
    };
#[no_mangle]
unsafe extern "C" fn bcm281xx_pinctrl_get_fcns_count(pctldev: *mut pinctrl_dev) -> c_int {
    static int bcm281xx_pinctrl_get_fcns_count(struct pinctrl_dev *pctldev)
    {
    struct bcm281xx_pinctrl_data *pdata = pinctrl_dev_get_drvdata(pctldev);
    return pdata.info.nfunctions;
    }
    static const char *bcm281xx_pinctrl_get_fcn_name(struct pinctrl_dev *pctldev,
    unsigned int function)
    {
    struct bcm281xx_pinctrl_data *pdata = pinctrl_dev_get_drvdata(pctldev);
    return pdata.info.functions[function].name;
    }
    static int bcm281xx_pinctrl_get_fcn_groups(struct pinctrl_dev *pctldev,
    unsigned int function,
    const char * const **groups,
    unsigned int * const num_groups)
    {
    struct bcm281xx_pinctrl_data *pdata = pinctrl_dev_get_drvdata(pctldev);
// groups = pdata->info->functions[function].groups;
// num_groups = pdata->info->functions[function].ngroups;
    return 0;
    }
    static int bcm281xx_pinmux_set(struct pinctrl_dev *pctldev,
    unsigned int function,
    unsigned int group)
    {
    struct bcm281xx_pinctrl_data *pdata = pinctrl_dev_get_drvdata(pctldev);
    const struct bcm281xx_pin_function *f = &pdata.info.functions[function];
    let mut device_type: enum bcm281xx_pinctrl_type = pdata.info.device_type;
    let mut pin: c_uint = pdata.info.pins[group].number;
    let mut offset: u32 = 4 * pin;
    let mut rc: c_int = 0;
    dev_dbg(pctldev.dev,
    "%s(): Enable function %s (%d) of pin %s (%d) @offset 0x%x.\n",
    __func__, f.name, function, pdata.info.pins[group].name,
    pin, offset);
    if (device_type == BCM21664_PINCTRL_TYPE) {
    rc = bcm21664_pinctrl_set_pin_lock(pdata, pin, false);
    if (rc) {
// Error is printed in bcm21664_pinctrl_set_pin_lock
    return rc;
    }
    }
    rc = regmap_update_bits(pdata.regmap, offset,
    BCM281XX_PIN_REG_F_SEL_MASK,
    function << BCM281XX_PIN_REG_F_SEL_SHIFT);
    if (rc)
    dev_err(pctldev.dev,
    "Error updating register for pin %s (%d).\n",
    pdata.info.pins[group].name, pin);
    if (device_type == BCM21664_PINCTRL_TYPE) {
    rc = bcm21664_pinctrl_set_pin_lock(pdata, pin, true);
    if (rc) {
// Error is printed in bcm21664_pinctrl_set_pin_lock
    return rc;
    }
    }
    return rc;
    }
    static const struct pinmux_ops bcm281xx_pinctrl_pinmux_ops = {
    .get_functions_count = bcm281xx_pinctrl_get_fcns_count,
    .get_function_name = bcm281xx_pinctrl_get_fcn_name,
    .get_function_groups = bcm281xx_pinctrl_get_fcn_groups,
    .set_mux = bcm281xx_pinmux_set,
    };
    static int bcm281xx_pinctrl_pin_config_get(struct pinctrl_dev *pctldev,
    unsigned int pin,
    unsigned long *config)
    {
    return -ENOTSUPP;
    }
// Goes through the configs and update register val/mask
    static int bcm281xx_std_pin_update(struct pinctrl_dev *pctldev,
    unsigned int pin,
    unsigned long *configs,
    unsigned int num_configs,
    u32 *val,
    u32 *mask)
    {
    struct bcm281xx_pinctrl_data *pdata = pinctrl_dev_get_drvdata(pctldev);
    int i;
    enum pin_config_param param;
    u32 arg;
    for (i = 0; i < num_configs; i++) {
    param = pinconf_to_config_param(configs[i]);
    arg = pinconf_to_config_argument(configs[i]);
    switch (param) {
    case PIN_CONFIG_INPUT_SCHMITT_ENABLE:
    arg = (arg >= 1 ? 1 : 0);
    bcm281xx_pin_update(val, mask, arg,
    BCM281XX_PIN_SHIFT(STD, HYST),
    BCM281XX_PIN_MASK(STD, HYST));
    break;
//
// The pin bias can only be one of pull-up, pull-down, or
// disable.  The user does not need to specify a value for the
// property, and the default value from pinconf-generic is
// ignored.
//
    case PIN_CONFIG_BIAS_DISABLE:
    bcm281xx_pin_update(val, mask, 0,
    BCM281XX_PIN_SHIFT(STD, PULL_UP),
    BCM281XX_PIN_MASK(STD, PULL_UP));
    bcm281xx_pin_update(val, mask, 0,
    BCM281XX_PIN_SHIFT(STD, PULL_DN),
    BCM281XX_PIN_MASK(STD, PULL_DN));
    break;
    case PIN_CONFIG_BIAS_PULL_UP:
    bcm281xx_pin_update(val, mask, 1,
    BCM281XX_PIN_SHIFT(STD, PULL_UP),
    BCM281XX_PIN_MASK(STD, PULL_UP));
    bcm281xx_pin_update(val, mask, 0,
    BCM281XX_PIN_SHIFT(STD, PULL_DN),
    BCM281XX_PIN_MASK(STD, PULL_DN));
    break;
    case PIN_CONFIG_BIAS_PULL_DOWN:
    bcm281xx_pin_update(val, mask, 0,
    BCM281XX_PIN_SHIFT(STD, PULL_UP),
    BCM281XX_PIN_MASK(STD, PULL_UP));
    bcm281xx_pin_update(val, mask, 1,
    BCM281XX_PIN_SHIFT(STD, PULL_DN),
    BCM281XX_PIN_MASK(STD, PULL_DN));
    break;
    case PIN_CONFIG_SLEW_RATE:
    arg = (arg >= 1 ? 1 : 0);
    bcm281xx_pin_update(val, mask, arg,
    BCM281XX_PIN_SHIFT(STD, SLEW),
    BCM281XX_PIN_MASK(STD, SLEW));
    break;
    case PIN_CONFIG_INPUT_ENABLE:
// inversed since register is for input _disable_
    arg = (arg >= 1 ? 0 : 1);
    bcm281xx_pin_update(val, mask, arg,
    BCM281XX_PIN_SHIFT(STD, INPUT_DIS),
    BCM281XX_PIN_MASK(STD, INPUT_DIS));
    break;
    case PIN_CONFIG_DRIVE_STRENGTH:
// Valid range is 2-16 mA, even numbers only
    if ((arg < 2) || (arg > 16) || (arg % 2)) {
    dev_err(pctldev.dev,
    "Invalid Drive Strength value (%d) for "
    "pin %s (%d). Valid values are "
    "(2..16) mA, even numbers only.\n",
    arg, pdata.info.pins[pin].name, pin);
    return -EINVAL;
    }
    bcm281xx_pin_update(val, mask, (arg/2)-1,
    BCM281XX_PIN_SHIFT(STD, DRV_STR),
    BCM281XX_PIN_MASK(STD, DRV_STR));
    break;
    default:
    dev_err(pctldev.dev,
    "Unrecognized pin config %d for pin %s (%d).\n",
    param, pdata.info.pins[pin].name, pin);
    return -EINVAL;
    } /* switch config */
    } /* for each config */
    return 0;
    }
//
// The pull-up strength for an I2C pin is represented by bits 4-6 in the
// register with the following mapping:
// 0b000: No pull-up
// 0b001: 1200 Ohm
// 0b010: 1800 Ohm
// 0b011: 720 Ohm
// 0b100: 2700 Ohm
// 0b101: 831 Ohm
// 0b110: 1080 Ohm
// 0b111: 568 Ohm
// This array maps pull-up strength in Ohms to register values (1+index).
//
    static const u16 bcm281xx_pullup_map[] = {
    1200, 1800, 720, 2700, 831, 1080, 568
    };
// Goes through the configs and update register val/mask
    static int bcm281xx_i2c_pin_update(struct pinctrl_dev *pctldev,
    unsigned int pin,
    unsigned long *configs,
    unsigned int num_configs,
    u32 *val,
    u32 *mask)
    {
    struct bcm281xx_pinctrl_data *pdata = pinctrl_dev_get_drvdata(pctldev);
    int i, j;
    enum pin_config_param param;
    u32 arg;
    for (i = 0; i < num_configs; i++) {
    param = pinconf_to_config_param(configs[i]);
    arg = pinconf_to_config_argument(configs[i]);
    switch (param) {
    case PIN_CONFIG_BIAS_PULL_UP:
    for (j = 0; j < ARRAY_SIZE(bcm281xx_pullup_map); j++)
    if (bcm281xx_pullup_map[j] == arg)
    break;
    if (j == ARRAY_SIZE(bcm281xx_pullup_map)) {
    dev_err(pctldev.dev,
    "Invalid pull-up value (%d) for pin %s "
    "(%d). Valid values are 568, 720, 831, "
    "1080, 1200, 1800, 2700 Ohms.\n",
    arg, pdata.info.pins[pin].name, pin);
    return -EINVAL;
    }
    bcm281xx_pin_update(val, mask, j+1,
    BCM281XX_PIN_SHIFT(I2C, PULL_UP_STR),
    BCM281XX_PIN_MASK(I2C, PULL_UP_STR));
    break;
    case PIN_CONFIG_BIAS_DISABLE:
    bcm281xx_pin_update(val, mask, 0,
    BCM281XX_PIN_SHIFT(I2C, PULL_UP_STR),
    BCM281XX_PIN_MASK(I2C, PULL_UP_STR));
    break;
    case PIN_CONFIG_SLEW_RATE:
    arg = (arg >= 1 ? 1 : 0);
    bcm281xx_pin_update(val, mask, arg,
    BCM281XX_PIN_SHIFT(I2C, SLEW),
    BCM281XX_PIN_MASK(I2C, SLEW));
    break;
    case PIN_CONFIG_INPUT_ENABLE:
// inversed since register is for input _disable_
    arg = (arg >= 1 ? 0 : 1);
    bcm281xx_pin_update(val, mask, arg,
    BCM281XX_PIN_SHIFT(I2C, INPUT_DIS),
    BCM281XX_PIN_MASK(I2C, INPUT_DIS));
    break;
    default:
    dev_err(pctldev.dev,
    "Unrecognized pin config %d for pin %s (%d).\n",
    param, pdata.info.pins[pin].name, pin);
    return -EINVAL;
    } /* switch config */
    } /* for each config */
    return 0;
    }
// Goes through the configs and update register val/mask
    static int bcm21664_i2c_pin_update(struct pinctrl_dev *pctldev,
    unsigned int pin,
    unsigned long *configs,
    unsigned int num_configs,
    u32 *val,
    u32 *mask)
    {
    struct bcm281xx_pinctrl_data *pdata = pinctrl_dev_get_drvdata(pctldev);
    int i;
    enum pin_config_param param;
    u32 arg;
    for (i = 0; i < num_configs; i++) {
    param = pinconf_to_config_param(configs[i]);
    arg = pinconf_to_config_argument(configs[i]);
//
// BCM21664 I2C pins use the same config bits as standard pins,
// but only pull up/none, slew rate and input enable/disable
// options are supported.
//
    switch (param) {
    case PIN_CONFIG_BIAS_PULL_UP:
    bcm281xx_pin_update(val, mask, 1,
    BCM281XX_PIN_SHIFT(STD, PULL_UP),
    BCM281XX_PIN_MASK(STD, PULL_UP));
    break;
    case PIN_CONFIG_BIAS_DISABLE:
    bcm281xx_pin_update(val, mask, 0,
    BCM281XX_PIN_SHIFT(STD, PULL_UP),
    BCM281XX_PIN_MASK(STD, PULL_UP));
    break;
    case PIN_CONFIG_SLEW_RATE:
    arg = (arg >= 1 ? 1 : 0);
    bcm281xx_pin_update(val, mask, arg,
    BCM281XX_PIN_SHIFT(STD, SLEW),
    BCM281XX_PIN_MASK(STD, SLEW));
    break;
    case PIN_CONFIG_INPUT_ENABLE:
// inversed since register is for input _disable_
    arg = (arg >= 1 ? 0 : 1);
    bcm281xx_pin_update(val, mask, arg,
    BCM281XX_PIN_SHIFT(STD, INPUT_DIS),
    BCM281XX_PIN_MASK(STD, INPUT_DIS));
    break;
    default:
    dev_err(pctldev.dev,
    "Unrecognized pin config %d for pin %s (%d).\n",
    param, pdata.info.pins[pin].name, pin);
    return -EINVAL;
    } /* switch config */
    } /* for each config */
    return 0;
    }
// Goes through the configs and update register val/mask
    static int bcm281xx_hdmi_pin_update(struct pinctrl_dev *pctldev,
    unsigned int pin,
    unsigned long *configs,
    unsigned int num_configs,
    u32 *val,
    u32 *mask)
    {
    struct bcm281xx_pinctrl_data *pdata = pinctrl_dev_get_drvdata(pctldev);
    int i;
    enum pin_config_param param;
    u32 arg;
    for (i = 0; i < num_configs; i++) {
    param = pinconf_to_config_param(configs[i]);
    arg = pinconf_to_config_argument(configs[i]);
    switch (param) {
    case PIN_CONFIG_SLEW_RATE:
    arg = (arg >= 1 ? 1 : 0);
    bcm281xx_pin_update(val, mask, arg,
    BCM281XX_PIN_SHIFT(HDMI, MODE),
    BCM281XX_PIN_MASK(HDMI, MODE));
    break;
    case PIN_CONFIG_INPUT_ENABLE:
// inversed since register is for input _disable_
    arg = (arg >= 1 ? 0 : 1);
    bcm281xx_pin_update(val, mask, arg,
    BCM281XX_PIN_SHIFT(HDMI, INPUT_DIS),
    BCM281XX_PIN_MASK(HDMI, INPUT_DIS));
    break;
    default:
    dev_err(pctldev.dev,
    "Unrecognized pin config %d for pin %s (%d).\n",
    param, pdata.info.pins[pin].name, pin);
    return -EINVAL;
    } /* switch config */
    } /* for each config */
    return 0;
    }
    static int bcm281xx_pinctrl_pin_config_set(struct pinctrl_dev *pctldev,
    unsigned int pin,
    unsigned long *configs,
    unsigned int num_configs)
    {
    struct bcm281xx_pinctrl_data *pdata = pinctrl_dev_get_drvdata(pctldev);
    let mut device_type: enum bcm281xx_pinctrl_type = pdata.info.device_type;
    enum bcm281xx_pin_type pin_type;
    let mut offset: u32 = 4 * pin;
    u32 cfg_val, cfg_mask;
    int rc;
    cfg_val = 0;
    cfg_mask = 0;
    pin_type = pin_type_get(pctldev, pin);
// Different pins have different configuration options
    switch (pin_type) {
    case BCM281XX_PIN_TYPE_STD:
    rc = bcm281xx_std_pin_update(pctldev, pin, configs,
    num_configs, &cfg_val, &cfg_mask);
    break;
    case BCM281XX_PIN_TYPE_I2C:
    if (device_type == BCM21664_PINCTRL_TYPE)
    rc = bcm21664_i2c_pin_update(pctldev, pin, configs,
    num_configs, &cfg_val, &cfg_mask);
    else
    rc = bcm281xx_i2c_pin_update(pctldev, pin, configs,
    num_configs, &cfg_val, &cfg_mask);
    break;
    case BCM281XX_PIN_TYPE_HDMI:
    rc = bcm281xx_hdmi_pin_update(pctldev, pin, configs,
    num_configs, &cfg_val, &cfg_mask);
    break;
    default:
    dev_err(pctldev.dev, "Unknown pin type for pin %s (%d).\n",
    pdata.info.pins[pin].name, pin);
    return -EINVAL;
    } /* switch pin type */
    if (rc)
    return rc;
    dev_dbg(pctldev.dev,
    "%s(): Set pin %s (%d) with config 0x%x, mask 0x%x\n",
    __func__, pdata.info.pins[pin].name, pin, cfg_val, cfg_mask);
    if (device_type == BCM21664_PINCTRL_TYPE) {
    rc = bcm21664_pinctrl_set_pin_lock(pdata, pin, false);
    if (rc) {
// Error is printed in bcm21664_pinctrl_set_pin_lock
    return rc;
    }
    }
    rc = regmap_update_bits(pdata.regmap, offset, cfg_mask, cfg_val);
    if (rc) {
    dev_err(pctldev.dev,
    "Error updating register for pin %s (%d).\n",
    pdata.info.pins[pin].name, pin);
    return rc;
    }
    if (device_type == BCM21664_PINCTRL_TYPE) {
    rc = bcm21664_pinctrl_set_pin_lock(pdata, pin, true);
    if (rc) {
// Error is printed in bcm21664_pinctrl_set_pin_lock
    return rc;
    }
    }
    return 0;
    }
    static const struct pinconf_ops bcm281xx_pinctrl_pinconf_ops = {
    .pin_config_get = bcm281xx_pinctrl_pin_config_get,
    .pin_config_set = bcm281xx_pinctrl_pin_config_set,
    };
    static struct pinctrl_desc bcm281xx_pinctrl_desc = {
// name, pins, npins members initialized in probe function
    .pctlops = &bcm281xx_pinctrl_ops,
    .pmxops = &bcm281xx_pinctrl_pinmux_ops,
    .confops = &bcm281xx_pinctrl_pinconf_ops,
    .owner = THIS_MODULE,
    };
    static struct bcm281xx_pinctrl_data bcm281xx_pinctrl_pdata;
#[no_mangle]
unsafe extern "C" fn bcm281xx_pinctrl_probe(pdev: *mut platform_device) -> int __init {
    static int __init bcm281xx_pinctrl_probe(struct platform_device *pdev)
    {
    struct bcm281xx_pinctrl_data *pdata = &bcm281xx_pinctrl_pdata;
    struct pinctrl_dev *pctl;
    int rc;
// Set device pointer in platform data
    pdata.dev = &pdev.dev;
// Get the data to use from OF match
    pdata.info = of_device_get_match_data(&pdev.dev);
    if (!pdata.info) {
    dev_err(&pdev.dev, "Failed to get data from OF match\n");
    return -ENODEV;
    }
// So far We can assume there is only 1 bank of registers
    pdata.reg_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(pdata.reg_base)) {
    dev_err(&pdev.dev, "Failed to ioremap MEM resource\n");
    return PTR_ERR(pdata.reg_base);
    }
// Initialize the dynamic part of pinctrl_desc
    pdata.regmap = devm_regmap_init_mmio(&pdev.dev, pdata.reg_base,
    pdata.info.regmap_config);
    if (IS_ERR(pdata.regmap)) {
    dev_err(&pdev.dev, "Regmap MMIO init failed.\n");
    return -ENODEV;
    }
    bcm281xx_pinctrl_desc.name = dev_name(&pdev.dev);
    bcm281xx_pinctrl_desc.pins = pdata.info.pins;
    bcm281xx_pinctrl_desc.npins = pdata.info.npins;
//
// For BCM21664, lock all pins by default; they will be unlocked
// as needed
//
    if (pdata.info.device_type == BCM21664_PINCTRL_TYPE) {
    rc = bcm21664_pinctrl_lock_all(pdata);
    if (rc) {
    dev_err(&pdev.dev, "Failed to lock all pins\n");
    return rc;
    }
    }
    pctl = devm_pinctrl_register(&pdev.dev, &bcm281xx_pinctrl_desc, pdata);
    if (IS_ERR(pctl)) {
    dev_err(&pdev.dev, "Failed to register pinctrl\n");
    return PTR_ERR(pctl);
    }
    platform_set_drvdata(pdev, pdata);
    return 0;
    }
    static const struct of_device_id bcm281xx_pinctrl_of_match[] = {
    { .compatible = "brcm,bcm11351-pinctrl", .data = &bcm281xx_pinctrl },
    { .compatible = "brcm,bcm21664-pinctrl", .data = &bcm21664_pinctrl },
    { },
    };
    static struct platform_driver bcm281xx_pinctrl_driver = {
    .driver = {
    .name = "bcm281xx-pinctrl",
    .of_match_table = bcm281xx_pinctrl_of_match,
    },
    };
    builtin_platform_driver_probe(bcm281xx_pinctrl_driver, bcm281xx_pinctrl_probe);
