//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/hi3660-clock.h
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
// Copyright (c) 2016-2017 Linaro Ltd.
// Copyright (c) 2016-2017 HiSilicon Technologies Co., Ltd.
//
// fixed rate clocks
pub const HI3660_CLKIN_SYS: c_int = 0;
pub const HI3660_CLKIN_REF: c_int = 1;
pub const HI3660_CLK_FLL_SRC: c_int = 2;
pub const HI3660_CLK_PPLL0: c_int = 3;
pub const HI3660_CLK_PPLL1: c_int = 4;
pub const HI3660_CLK_PPLL2: c_int = 5;
pub const HI3660_CLK_PPLL3: c_int = 6;
pub const HI3660_CLK_SCPLL: c_int = 7;
pub const HI3660_PCLK: c_int = 8;
pub const HI3660_CLK_UART0_DBG: c_int = 9;
pub const HI3660_CLK_UART6: c_int = 10;
pub const HI3660_OSC32K: c_int = 11;
pub const HI3660_OSC19M: c_int = 12;
pub const HI3660_CLK_480M: c_int = 13;
pub const HI3660_CLK_INV: c_int = 14;
// clk in crgctrl
pub const HI3660_FACTOR_UART3: c_int = 15;
pub const HI3660_CLK_FACTOR_MMC: c_int = 16;
pub const HI3660_CLK_GATE_I2C0: c_int = 17;
pub const HI3660_CLK_GATE_I2C1: c_int = 18;
pub const HI3660_CLK_GATE_I2C2: c_int = 19;
pub const HI3660_CLK_GATE_I2C6: c_int = 20;
pub const HI3660_CLK_DIV_SYSBUS: c_int = 21;
pub const HI3660_CLK_DIV_320M: c_int = 22;
pub const HI3660_CLK_DIV_A53: c_int = 23;
pub const HI3660_CLK_GATE_SPI0: c_int = 24;
pub const HI3660_CLK_GATE_SPI2: c_int = 25;
pub const HI3660_PCIEPHY_REF: c_int = 26;
pub const HI3660_CLK_ABB_USB: c_int = 27;
pub const HI3660_HCLK_GATE_SDIO0: c_int = 28;
pub const HI3660_HCLK_GATE_SD: c_int = 29;
pub const HI3660_CLK_GATE_AOMM: c_int = 30;
pub const HI3660_PCLK_GPIO0: c_int = 31;
pub const HI3660_PCLK_GPIO1: c_int = 32;
pub const HI3660_PCLK_GPIO2: c_int = 33;
pub const HI3660_PCLK_GPIO3: c_int = 34;
pub const HI3660_PCLK_GPIO4: c_int = 35;
pub const HI3660_PCLK_GPIO5: c_int = 36;
pub const HI3660_PCLK_GPIO6: c_int = 37;
pub const HI3660_PCLK_GPIO7: c_int = 38;
pub const HI3660_PCLK_GPIO8: c_int = 39;
pub const HI3660_PCLK_GPIO9: c_int = 40;
pub const HI3660_PCLK_GPIO10: c_int = 41;
pub const HI3660_PCLK_GPIO11: c_int = 42;
pub const HI3660_PCLK_GPIO12: c_int = 43;
pub const HI3660_PCLK_GPIO13: c_int = 44;
pub const HI3660_PCLK_GPIO14: c_int = 45;
pub const HI3660_PCLK_GPIO15: c_int = 46;
pub const HI3660_PCLK_GPIO16: c_int = 47;
pub const HI3660_PCLK_GPIO17: c_int = 48;
pub const HI3660_PCLK_GPIO18: c_int = 49;
pub const HI3660_PCLK_GPIO19: c_int = 50;
pub const HI3660_PCLK_GPIO20: c_int = 51;
pub const HI3660_PCLK_GPIO21: c_int = 52;
pub const HI3660_CLK_GATE_SPI3: c_int = 53;
pub const HI3660_CLK_GATE_I2C7: c_int = 54;
pub const HI3660_CLK_GATE_I2C3: c_int = 55;
pub const HI3660_CLK_GATE_SPI1: c_int = 56;
pub const HI3660_CLK_GATE_UART1: c_int = 57;
pub const HI3660_CLK_GATE_UART2: c_int = 58;
pub const HI3660_CLK_GATE_UART4: c_int = 59;
pub const HI3660_CLK_GATE_UART5: c_int = 60;
pub const HI3660_CLK_GATE_I2C4: c_int = 61;
pub const HI3660_CLK_GATE_DMAC: c_int = 62;
pub const HI3660_PCLK_GATE_DSS: c_int = 63;
pub const HI3660_ACLK_GATE_DSS: c_int = 64;
pub const HI3660_CLK_GATE_LDI1: c_int = 65;
pub const HI3660_CLK_GATE_LDI0: c_int = 66;
pub const HI3660_CLK_GATE_VIVOBUS: c_int = 67;
pub const HI3660_CLK_GATE_EDC0: c_int = 68;
pub const HI3660_CLK_GATE_TXDPHY0_CFG: c_int = 69;
pub const HI3660_CLK_GATE_TXDPHY0_REF: c_int = 70;
pub const HI3660_CLK_GATE_TXDPHY1_CFG: c_int = 71;
pub const HI3660_CLK_GATE_TXDPHY1_REF: c_int = 72;
pub const HI3660_ACLK_GATE_USB3OTG: c_int = 73;
pub const HI3660_CLK_GATE_SPI4: c_int = 74;
pub const HI3660_CLK_GATE_SD: c_int = 75;
pub const HI3660_CLK_GATE_SDIO0: c_int = 76;
pub const HI3660_CLK_GATE_UFS_SUBSYS: c_int = 77;
pub const HI3660_PCLK_GATE_DSI0: c_int = 78;
pub const HI3660_PCLK_GATE_DSI1: c_int = 79;
pub const HI3660_ACLK_GATE_PCIE: c_int = 80;
pub const HI3660_PCLK_GATE_PCIE_SYS: c_int = 81;
pub const HI3660_CLK_GATE_PCIEAUX: c_int = 82;
pub const HI3660_PCLK_GATE_PCIE_PHY: c_int = 83;
pub const HI3660_CLK_ANDGT_LDI0: c_int = 84;
pub const HI3660_CLK_ANDGT_LDI1: c_int = 85;
pub const HI3660_CLK_ANDGT_EDC0: c_int = 86;
pub const HI3660_CLK_GATE_UFSPHY_GT: c_int = 87;
pub const HI3660_CLK_ANDGT_MMC: c_int = 88;
pub const HI3660_CLK_ANDGT_SD: c_int = 89;
pub const HI3660_CLK_A53HPM_ANDGT: c_int = 90;
pub const HI3660_CLK_ANDGT_SDIO: c_int = 91;
pub const HI3660_CLK_ANDGT_UART0: c_int = 92;
pub const HI3660_CLK_ANDGT_UART1: c_int = 93;
pub const HI3660_CLK_ANDGT_UARTH: c_int = 94;
pub const HI3660_CLK_ANDGT_SPI: c_int = 95;
pub const HI3660_CLK_VIVOBUS_ANDGT: c_int = 96;
pub const HI3660_CLK_AOMM_ANDGT: c_int = 97;
pub const HI3660_CLK_320M_PLL_GT: c_int = 98;
pub const HI3660_AUTODIV_EMMC0BUS: c_int = 99;
pub const HI3660_AUTODIV_SYSBUS: c_int = 100;
pub const HI3660_CLK_GATE_UFSPHY_CFG: c_int = 101;
pub const HI3660_CLK_GATE_UFSIO_REF: c_int = 102;
pub const HI3660_CLK_MUX_SYSBUS: c_int = 103;
pub const HI3660_CLK_MUX_UART0: c_int = 104;
pub const HI3660_CLK_MUX_UART1: c_int = 105;
pub const HI3660_CLK_MUX_UARTH: c_int = 106;
pub const HI3660_CLK_MUX_SPI: c_int = 107;
pub const HI3660_CLK_MUX_I2C: c_int = 108;
pub const HI3660_CLK_MUX_MMC_PLL: c_int = 109;
pub const HI3660_CLK_MUX_LDI1: c_int = 110;
pub const HI3660_CLK_MUX_LDI0: c_int = 111;
pub const HI3660_CLK_MUX_SD_PLL: c_int = 112;
pub const HI3660_CLK_MUX_SD_SYS: c_int = 113;
pub const HI3660_CLK_MUX_EDC0: c_int = 114;
pub const HI3660_CLK_MUX_SDIO_SYS: c_int = 115;
pub const HI3660_CLK_MUX_SDIO_PLL: c_int = 116;
pub const HI3660_CLK_MUX_VIVOBUS: c_int = 117;
pub const HI3660_CLK_MUX_A53HPM: c_int = 118;
pub const HI3660_CLK_MUX_320M: c_int = 119;
pub const HI3660_CLK_MUX_IOPERI: c_int = 120;
pub const HI3660_CLK_DIV_UART0: c_int = 121;
pub const HI3660_CLK_DIV_UART1: c_int = 122;
pub const HI3660_CLK_DIV_UARTH: c_int = 123;
pub const HI3660_CLK_DIV_MMC: c_int = 124;
pub const HI3660_CLK_DIV_SD: c_int = 125;
pub const HI3660_CLK_DIV_EDC0: c_int = 126;
pub const HI3660_CLK_DIV_LDI0: c_int = 127;
pub const HI3660_CLK_DIV_SDIO: c_int = 128;
pub const HI3660_CLK_DIV_LDI1: c_int = 129;
pub const HI3660_CLK_DIV_SPI: c_int = 130;
pub const HI3660_CLK_DIV_VIVOBUS: c_int = 131;
pub const HI3660_CLK_DIV_I2C: c_int = 132;
pub const HI3660_CLK_DIV_UFSPHY: c_int = 133;
pub const HI3660_CLK_DIV_CFGBUS: c_int = 134;
pub const HI3660_CLK_DIV_MMC0BUS: c_int = 135;
pub const HI3660_CLK_DIV_MMC1BUS: c_int = 136;
pub const HI3660_CLK_DIV_UFSPERI: c_int = 137;
pub const HI3660_CLK_DIV_AOMM: c_int = 138;
pub const HI3660_CLK_DIV_IOPERI: c_int = 139;
pub const HI3660_VENC_VOLT_HOLD: c_int = 140;
pub const HI3660_PERI_VOLT_HOLD: c_int = 141;
pub const HI3660_CLK_GATE_VENC: c_int = 142;
pub const HI3660_CLK_GATE_VDEC: c_int = 143;
pub const HI3660_CLK_ANDGT_VENC: c_int = 144;
pub const HI3660_CLK_ANDGT_VDEC: c_int = 145;
pub const HI3660_CLK_MUX_VENC: c_int = 146;
pub const HI3660_CLK_MUX_VDEC: c_int = 147;
pub const HI3660_CLK_DIV_VENC: c_int = 148;
pub const HI3660_CLK_DIV_VDEC: c_int = 149;
pub const HI3660_CLK_FAC_ISP_SNCLK: c_int = 150;
pub const HI3660_CLK_GATE_ISP_SNCLK0: c_int = 151;
pub const HI3660_CLK_GATE_ISP_SNCLK1: c_int = 152;
pub const HI3660_CLK_GATE_ISP_SNCLK2: c_int = 153;
pub const HI3660_CLK_ANGT_ISP_SNCLK: c_int = 154;
pub const HI3660_CLK_MUX_ISP_SNCLK: c_int = 155;
pub const HI3660_CLK_DIV_ISP_SNCLK: c_int = 156;
// clk in pmuctrl
pub const HI3660_GATE_ABB_192: c_int = 0;
// clk in pctrl
pub const HI3660_GATE_UFS_TCXO_EN: c_int = 0;
pub const HI3660_GATE_USB_TCXO_EN: c_int = 1;
// clk in sctrl
pub const HI3660_PCLK_AO_GPIO0: c_int = 0;
pub const HI3660_PCLK_AO_GPIO1: c_int = 1;
pub const HI3660_PCLK_AO_GPIO2: c_int = 2;
pub const HI3660_PCLK_AO_GPIO3: c_int = 3;
pub const HI3660_PCLK_AO_GPIO4: c_int = 4;
pub const HI3660_PCLK_AO_GPIO5: c_int = 5;
pub const HI3660_PCLK_AO_GPIO6: c_int = 6;
pub const HI3660_PCLK_GATE_MMBUF: c_int = 7;
pub const HI3660_CLK_GATE_DSS_AXI_MM: c_int = 8;
pub const HI3660_PCLK_MMBUF_ANDGT: c_int = 9;
pub const HI3660_CLK_MMBUF_PLL_ANDGT: c_int = 10;
pub const HI3660_CLK_FLL_MMBUF_ANDGT: c_int = 11;
pub const HI3660_CLK_SYS_MMBUF_ANDGT: c_int = 12;
pub const HI3660_CLK_GATE_PCIEPHY_GT: c_int = 13;
pub const HI3660_ACLK_MUX_MMBUF: c_int = 14;
pub const HI3660_CLK_SW_MMBUF: c_int = 15;
pub const HI3660_CLK_DIV_AOBUS: c_int = 16;
pub const HI3660_PCLK_DIV_MMBUF: c_int = 17;
pub const HI3660_ACLK_DIV_MMBUF: c_int = 18;
pub const HI3660_CLK_DIV_PCIEPHY: c_int = 19;
// clk in iomcu
pub const HI3660_CLK_I2C0_IOMCU: c_int = 0;
pub const HI3660_CLK_I2C1_IOMCU: c_int = 1;
pub const HI3660_CLK_I2C2_IOMCU: c_int = 2;
pub const HI3660_CLK_I2C6_IOMCU: c_int = 3;
pub const HI3660_CLK_IOMCU_PERI0: c_int = 4;
// clk in stub clock
pub const HI3660_CLK_STUB_CLUSTER0: c_int = 0;
pub const HI3660_CLK_STUB_CLUSTER1: c_int = 1;
pub const HI3660_CLK_STUB_GPU: c_int = 2;
pub const HI3660_CLK_STUB_DDR: c_int = 3;
pub const HI3660_CLK_STUB_NUM: c_int = 4;
