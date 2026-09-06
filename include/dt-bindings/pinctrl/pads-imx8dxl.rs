//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/pinctrl/pads-imx8dxl.h
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
// Copyright 2019~2020 NXP
//
// pin id
pub const IMX8DXL_PCIE_CTRL0_PERST_B: c_int = 0;
pub const IMX8DXL_PCIE_CTRL0_CLKREQ_B: c_int = 1;
pub const IMX8DXL_PCIE_CTRL0_WAKE_B: c_int = 2;
pub const IMX8DXL_COMP_CTL_GPIO_1V8_3V3_PCIESEP: c_int = 3;
pub const IMX8DXL_USB_SS3_TC0: c_int = 4;
pub const IMX8DXL_USB_SS3_TC1: c_int = 5;
pub const IMX8DXL_USB_SS3_TC2: c_int = 6;
pub const IMX8DXL_USB_SS3_TC3: c_int = 7;
pub const IMX8DXL_COMP_CTL_GPIO_3V3_USB3IO: c_int = 8;
pub const IMX8DXL_EMMC0_CLK: c_int = 9;
pub const IMX8DXL_EMMC0_CMD: c_int = 10;
pub const IMX8DXL_EMMC0_DATA0: c_int = 11;
pub const IMX8DXL_EMMC0_DATA1: c_int = 12;
pub const IMX8DXL_EMMC0_DATA2: c_int = 13;
pub const IMX8DXL_EMMC0_DATA3: c_int = 14;
pub const IMX8DXL_EMMC0_DATA4: c_int = 15;
pub const IMX8DXL_EMMC0_DATA5: c_int = 16;
pub const IMX8DXL_EMMC0_DATA6: c_int = 17;
pub const IMX8DXL_EMMC0_DATA7: c_int = 18;
pub const IMX8DXL_EMMC0_STROBE: c_int = 19;
pub const IMX8DXL_EMMC0_RESET_B: c_int = 20;
pub const IMX8DXL_COMP_CTL_GPIO_1V8_3V3_SD1FIX0: c_int = 21;
pub const IMX8DXL_USDHC1_RESET_B: c_int = 22;
pub const IMX8DXL_USDHC1_VSELECT: c_int = 23;
pub const IMX8DXL_CTL_NAND_RE_P_N: c_int = 24;
pub const IMX8DXL_USDHC1_WP: c_int = 25;
pub const IMX8DXL_USDHC1_CD_B: c_int = 26;
pub const IMX8DXL_CTL_NAND_DQS_P_N: c_int = 27;
pub const IMX8DXL_COMP_CTL_GPIO_1V8_3V3_VSELSEP: c_int = 28;
pub const IMX8DXL_ENET0_RGMII_TXC: c_int = 29;
pub const IMX8DXL_ENET0_RGMII_TX_CTL: c_int = 30;
pub const IMX8DXL_ENET0_RGMII_TXD0: c_int = 31;
pub const IMX8DXL_ENET0_RGMII_TXD1: c_int = 32;
pub const IMX8DXL_ENET0_RGMII_TXD2: c_int = 33;
pub const IMX8DXL_ENET0_RGMII_TXD3: c_int = 34;
pub const IMX8DXL_COMP_CTL_GPIO_1V8_3V3_ENET_ENETB0: c_int = 35;
pub const IMX8DXL_ENET0_RGMII_RXC: c_int = 36;
pub const IMX8DXL_ENET0_RGMII_RX_CTL: c_int = 37;
pub const IMX8DXL_ENET0_RGMII_RXD0: c_int = 38;
pub const IMX8DXL_ENET0_RGMII_RXD1: c_int = 39;
pub const IMX8DXL_ENET0_RGMII_RXD2: c_int = 40;
pub const IMX8DXL_ENET0_RGMII_RXD3: c_int = 41;
pub const IMX8DXL_COMP_CTL_GPIO_1V8_3V3_ENET_ENETB1: c_int = 42;
pub const IMX8DXL_ENET0_REFCLK_125M_25M: c_int = 43;
pub const IMX8DXL_ENET0_MDIO: c_int = 44;
pub const IMX8DXL_ENET0_MDC: c_int = 45;
pub const IMX8DXL_COMP_CTL_GPIO_1V8_3V3_GPIOCT: c_int = 46;
pub const IMX8DXL_ENET1_RGMII_TXC: c_int = 47;
pub const IMX8DXL_ENET1_RGMII_TXD2: c_int = 48;
pub const IMX8DXL_ENET1_RGMII_TX_CTL: c_int = 49;
pub const IMX8DXL_ENET1_RGMII_TXD3: c_int = 50;
pub const IMX8DXL_ENET1_RGMII_RXC: c_int = 51;
pub const IMX8DXL_ENET1_RGMII_RXD3: c_int = 52;
pub const IMX8DXL_ENET1_RGMII_RXD2: c_int = 53;
pub const IMX8DXL_ENET1_RGMII_RXD1: c_int = 54;
pub const IMX8DXL_ENET1_RGMII_TXD0: c_int = 55;
pub const IMX8DXL_ENET1_RGMII_TXD1: c_int = 56;
pub const IMX8DXL_ENET1_RGMII_RXD0: c_int = 57;
pub const IMX8DXL_ENET1_RGMII_RX_CTL: c_int = 58;
pub const IMX8DXL_ENET1_REFCLK_125M_25M: c_int = 59;
pub const IMX8DXL_COMP_CTL_GPIO_1V8_3V3_GPIORHB: c_int = 60;
pub const IMX8DXL_SPI3_SCK: c_int = 61;
pub const IMX8DXL_SPI3_SDO: c_int = 62;
pub const IMX8DXL_SPI3_SDI: c_int = 63;
pub const IMX8DXL_SPI3_CS0: c_int = 64;
pub const IMX8DXL_SPI3_CS1: c_int = 65;
pub const IMX8DXL_MCLK_IN1: c_int = 66;
pub const IMX8DXL_MCLK_IN0: c_int = 67;
pub const IMX8DXL_MCLK_OUT0: c_int = 68;
pub const IMX8DXL_UART1_TX: c_int = 69;
pub const IMX8DXL_UART1_RX: c_int = 70;
pub const IMX8DXL_UART1_RTS_B: c_int = 71;
pub const IMX8DXL_UART1_CTS_B: c_int = 72;
pub const IMX8DXL_COMP_CTL_GPIO_1V8_3V3_GPIORHK: c_int = 73;
pub const IMX8DXL_SPI0_SCK: c_int = 74;
pub const IMX8DXL_SPI0_SDI: c_int = 75;
pub const IMX8DXL_SPI0_SDO: c_int = 76;
pub const IMX8DXL_SPI0_CS1: c_int = 77;
pub const IMX8DXL_SPI0_CS0: c_int = 78;
pub const IMX8DXL_COMP_CTL_GPIO_1V8_3V3_GPIORHT: c_int = 79;
pub const IMX8DXL_ADC_IN1: c_int = 80;
pub const IMX8DXL_ADC_IN0: c_int = 81;
pub const IMX8DXL_ADC_IN3: c_int = 82;
pub const IMX8DXL_ADC_IN2: c_int = 83;
pub const IMX8DXL_ADC_IN5: c_int = 84;
pub const IMX8DXL_ADC_IN4: c_int = 85;
pub const IMX8DXL_FLEXCAN0_RX: c_int = 86;
pub const IMX8DXL_FLEXCAN0_TX: c_int = 87;
pub const IMX8DXL_FLEXCAN1_RX: c_int = 88;
pub const IMX8DXL_FLEXCAN1_TX: c_int = 89;
pub const IMX8DXL_FLEXCAN2_RX: c_int = 90;
pub const IMX8DXL_FLEXCAN2_TX: c_int = 91;
pub const IMX8DXL_UART0_RX: c_int = 92;
pub const IMX8DXL_UART0_TX: c_int = 93;
pub const IMX8DXL_UART2_TX: c_int = 94;
pub const IMX8DXL_UART2_RX: c_int = 95;
pub const IMX8DXL_COMP_CTL_GPIO_1V8_3V3_GPIOLH: c_int = 96;
pub const IMX8DXL_JTAG_TRST_B: c_int = 97;
pub const IMX8DXL_PMIC_I2C_SCL: c_int = 98;
pub const IMX8DXL_PMIC_I2C_SDA: c_int = 99;
pub const IMX8DXL_PMIC_INT_B: c_int = 100;
pub const IMX8DXL_SCU_GPIO0_00: c_int = 101;
pub const IMX8DXL_SCU_GPIO0_01: c_int = 102;
pub const IMX8DXL_SCU_PMIC_STANDBY: c_int = 103;
pub const IMX8DXL_SCU_BOOT_MODE1: c_int = 104;
pub const IMX8DXL_SCU_BOOT_MODE0: c_int = 105;
pub const IMX8DXL_SCU_BOOT_MODE2: c_int = 106;
pub const IMX8DXL_SNVS_TAMPER_OUT1: c_int = 107;
pub const IMX8DXL_SNVS_TAMPER_OUT2: c_int = 108;
pub const IMX8DXL_SNVS_TAMPER_OUT3: c_int = 109;
pub const IMX8DXL_SNVS_TAMPER_OUT4: c_int = 110;
pub const IMX8DXL_SNVS_TAMPER_IN0: c_int = 111;
pub const IMX8DXL_SNVS_TAMPER_IN1: c_int = 112;
pub const IMX8DXL_SNVS_TAMPER_IN2: c_int = 113;
pub const IMX8DXL_SNVS_TAMPER_IN3: c_int = 114;
pub const IMX8DXL_SPI1_SCK: c_int = 115;
pub const IMX8DXL_SPI1_SDO: c_int = 116;
pub const IMX8DXL_SPI1_SDI: c_int = 117;
pub const IMX8DXL_SPI1_CS0: c_int = 118;
pub const IMX8DXL_COMP_CTL_GPIO_1V8_3V3_GPIORHD: c_int = 119;
pub const IMX8DXL_QSPI0A_DATA1: c_int = 120;
pub const IMX8DXL_QSPI0A_DATA0: c_int = 121;
pub const IMX8DXL_QSPI0A_DATA3: c_int = 122;
pub const IMX8DXL_QSPI0A_DATA2: c_int = 123;
pub const IMX8DXL_QSPI0A_SS0_B: c_int = 124;
pub const IMX8DXL_QSPI0A_DQS: c_int = 125;
pub const IMX8DXL_QSPI0A_SCLK: c_int = 126;
pub const IMX8DXL_COMP_CTL_GPIO_1V8_3V3_QSPI0A: c_int = 127;
pub const IMX8DXL_QSPI0B_SCLK: c_int = 128;
pub const IMX8DXL_QSPI0B_DQS: c_int = 129;
pub const IMX8DXL_QSPI0B_DATA1: c_int = 130;
pub const IMX8DXL_QSPI0B_DATA0: c_int = 131;
pub const IMX8DXL_QSPI0B_DATA3: c_int = 132;
pub const IMX8DXL_QSPI0B_DATA2: c_int = 133;
pub const IMX8DXL_QSPI0B_SS0_B: c_int = 134;
pub const IMX8DXL_COMP_CTL_GPIO_1V8_3V3_QSPI0B: c_int = 135;
// format: <pin_id mux_mode>

