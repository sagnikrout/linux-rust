//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/starfive,jh7110-crg.h
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
//
// Copyright (C) 2022 Emil Renner Berthing <kernel@esmil.dk>
// Copyright (C) 2022 StarFive Technology Co., Ltd.
//
// SYSCRG resets
pub const JH7110_SYSRST_JTAG_APB: c_int = 0;
pub const JH7110_SYSRST_SYSCON_APB: c_int = 1;
pub const JH7110_SYSRST_IOMUX_APB: c_int = 2;
pub const JH7110_SYSRST_BUS: c_int = 3;
pub const JH7110_SYSRST_DEBUG: c_int = 4;
pub const JH7110_SYSRST_CORE0: c_int = 5;
pub const JH7110_SYSRST_CORE1: c_int = 6;
pub const JH7110_SYSRST_CORE2: c_int = 7;
pub const JH7110_SYSRST_CORE3: c_int = 8;
pub const JH7110_SYSRST_CORE4: c_int = 9;
pub const JH7110_SYSRST_CORE0_ST: c_int = 10;
pub const JH7110_SYSRST_CORE1_ST: c_int = 11;
pub const JH7110_SYSRST_CORE2_ST: c_int = 12;
pub const JH7110_SYSRST_CORE3_ST: c_int = 13;
pub const JH7110_SYSRST_CORE4_ST: c_int = 14;
pub const JH7110_SYSRST_TRACE0: c_int = 15;
pub const JH7110_SYSRST_TRACE1: c_int = 16;
pub const JH7110_SYSRST_TRACE2: c_int = 17;
pub const JH7110_SYSRST_TRACE3: c_int = 18;
pub const JH7110_SYSRST_TRACE4: c_int = 19;
pub const JH7110_SYSRST_TRACE_COM: c_int = 20;
pub const JH7110_SYSRST_GPU_APB: c_int = 21;
pub const JH7110_SYSRST_GPU_DOMA: c_int = 22;
pub const JH7110_SYSRST_NOC_BUS_APB: c_int = 23;
pub const JH7110_SYSRST_NOC_BUS_AXICFG0_AXI: c_int = 24;
pub const JH7110_SYSRST_NOC_BUS_CPU_AXI: c_int = 25;
pub const JH7110_SYSRST_NOC_BUS_DISP_AXI: c_int = 26;
pub const JH7110_SYSRST_NOC_BUS_GPU_AXI: c_int = 27;
pub const JH7110_SYSRST_NOC_BUS_ISP_AXI: c_int = 28;
pub const JH7110_SYSRST_NOC_BUS_DDRC: c_int = 29;
pub const JH7110_SYSRST_NOC_BUS_STG_AXI: c_int = 30;
pub const JH7110_SYSRST_NOC_BUS_VDEC_AXI: c_int = 31;
pub const JH7110_SYSRST_NOC_BUS_VENC_AXI: c_int = 32;
pub const JH7110_SYSRST_AXI_CFG1_AHB: c_int = 33;
pub const JH7110_SYSRST_AXI_CFG1_MAIN: c_int = 34;
pub const JH7110_SYSRST_AXI_CFG0_MAIN: c_int = 35;
pub const JH7110_SYSRST_AXI_CFG0_MAIN_DIV: c_int = 36;
pub const JH7110_SYSRST_AXI_CFG0_HIFI4: c_int = 37;
pub const JH7110_SYSRST_DDR_AXI: c_int = 38;
pub const JH7110_SYSRST_DDR_OSC: c_int = 39;
pub const JH7110_SYSRST_DDR_APB: c_int = 40;
pub const JH7110_SYSRST_ISP_TOP: c_int = 41;
pub const JH7110_SYSRST_ISP_TOP_AXI: c_int = 42;
pub const JH7110_SYSRST_VOUT_TOP_SRC: c_int = 43;
pub const JH7110_SYSRST_CODAJ12_AXI: c_int = 44;
pub const JH7110_SYSRST_CODAJ12_CORE: c_int = 45;
pub const JH7110_SYSRST_CODAJ12_APB: c_int = 46;
pub const JH7110_SYSRST_WAVE511_AXI: c_int = 47;
pub const JH7110_SYSRST_WAVE511_BPU: c_int = 48;
pub const JH7110_SYSRST_WAVE511_VCE: c_int = 49;
pub const JH7110_SYSRST_WAVE511_APB: c_int = 50;
pub const JH7110_SYSRST_VDEC_JPG: c_int = 51;
pub const JH7110_SYSRST_VDEC_MAIN: c_int = 52;
pub const JH7110_SYSRST_AXIMEM0_AXI: c_int = 53;
pub const JH7110_SYSRST_WAVE420L_AXI: c_int = 54;
pub const JH7110_SYSRST_WAVE420L_BPU: c_int = 55;
pub const JH7110_SYSRST_WAVE420L_VCE: c_int = 56;
pub const JH7110_SYSRST_WAVE420L_APB: c_int = 57;
pub const JH7110_SYSRST_AXIMEM1_AXI: c_int = 58;
pub const JH7110_SYSRST_AXIMEM2_AXI: c_int = 59;
pub const JH7110_SYSRST_INTMEM: c_int = 60;
pub const JH7110_SYSRST_QSPI_AHB: c_int = 61;
pub const JH7110_SYSRST_QSPI_APB: c_int = 62;
pub const JH7110_SYSRST_QSPI_REF: c_int = 63;
pub const JH7110_SYSRST_SDIO0_AHB: c_int = 64;
pub const JH7110_SYSRST_SDIO1_AHB: c_int = 65;
pub const JH7110_SYSRST_GMAC1_AXI: c_int = 66;
pub const JH7110_SYSRST_GMAC1_AHB: c_int = 67;
pub const JH7110_SYSRST_MAILBOX_APB: c_int = 68;
pub const JH7110_SYSRST_SPI0_APB: c_int = 69;
pub const JH7110_SYSRST_SPI1_APB: c_int = 70;
pub const JH7110_SYSRST_SPI2_APB: c_int = 71;
pub const JH7110_SYSRST_SPI3_APB: c_int = 72;
pub const JH7110_SYSRST_SPI4_APB: c_int = 73;
pub const JH7110_SYSRST_SPI5_APB: c_int = 74;
pub const JH7110_SYSRST_SPI6_APB: c_int = 75;
pub const JH7110_SYSRST_I2C0_APB: c_int = 76;
pub const JH7110_SYSRST_I2C1_APB: c_int = 77;
pub const JH7110_SYSRST_I2C2_APB: c_int = 78;
pub const JH7110_SYSRST_I2C3_APB: c_int = 79;
pub const JH7110_SYSRST_I2C4_APB: c_int = 80;
pub const JH7110_SYSRST_I2C5_APB: c_int = 81;
pub const JH7110_SYSRST_I2C6_APB: c_int = 82;
pub const JH7110_SYSRST_UART0_APB: c_int = 83;
pub const JH7110_SYSRST_UART0_CORE: c_int = 84;
pub const JH7110_SYSRST_UART1_APB: c_int = 85;
pub const JH7110_SYSRST_UART1_CORE: c_int = 86;
pub const JH7110_SYSRST_UART2_APB: c_int = 87;
pub const JH7110_SYSRST_UART2_CORE: c_int = 88;
pub const JH7110_SYSRST_UART3_APB: c_int = 89;
pub const JH7110_SYSRST_UART3_CORE: c_int = 90;
pub const JH7110_SYSRST_UART4_APB: c_int = 91;
pub const JH7110_SYSRST_UART4_CORE: c_int = 92;
pub const JH7110_SYSRST_UART5_APB: c_int = 93;
pub const JH7110_SYSRST_UART5_CORE: c_int = 94;
pub const JH7110_SYSRST_SPDIF_APB: c_int = 95;
pub const JH7110_SYSRST_PWMDAC_APB: c_int = 96;
pub const JH7110_SYSRST_PDM_DMIC: c_int = 97;
pub const JH7110_SYSRST_PDM_APB: c_int = 98;
pub const JH7110_SYSRST_I2SRX_APB: c_int = 99;
pub const JH7110_SYSRST_I2SRX_BCLK: c_int = 100;
pub const JH7110_SYSRST_I2STX0_APB: c_int = 101;
pub const JH7110_SYSRST_I2STX0_BCLK: c_int = 102;
pub const JH7110_SYSRST_I2STX1_APB: c_int = 103;
pub const JH7110_SYSRST_I2STX1_BCLK: c_int = 104;
pub const JH7110_SYSRST_TDM_AHB: c_int = 105;
pub const JH7110_SYSRST_TDM_CORE: c_int = 106;
pub const JH7110_SYSRST_TDM_APB: c_int = 107;
pub const JH7110_SYSRST_PWM_APB: c_int = 108;
pub const JH7110_SYSRST_WDT_APB: c_int = 109;
pub const JH7110_SYSRST_WDT_CORE: c_int = 110;
pub const JH7110_SYSRST_CAN0_APB: c_int = 111;
pub const JH7110_SYSRST_CAN0_CORE: c_int = 112;
pub const JH7110_SYSRST_CAN0_TIMER: c_int = 113;
pub const JH7110_SYSRST_CAN1_APB: c_int = 114;
pub const JH7110_SYSRST_CAN1_CORE: c_int = 115;
pub const JH7110_SYSRST_CAN1_TIMER: c_int = 116;
pub const JH7110_SYSRST_TIMER_APB: c_int = 117;
pub const JH7110_SYSRST_TIMER0: c_int = 118;
pub const JH7110_SYSRST_TIMER1: c_int = 119;
pub const JH7110_SYSRST_TIMER2: c_int = 120;
pub const JH7110_SYSRST_TIMER3: c_int = 121;
pub const JH7110_SYSRST_INT_CTRL_APB: c_int = 122;
pub const JH7110_SYSRST_TEMP_APB: c_int = 123;
pub const JH7110_SYSRST_TEMP_CORE: c_int = 124;
pub const JH7110_SYSRST_JTAG_CERTIFICATION: c_int = 125;
pub const JH7110_SYSRST_END: c_int = 126;
// AONCRG resets
pub const JH7110_AONRST_GMAC0_AXI: c_int = 0;
pub const JH7110_AONRST_GMAC0_AHB: c_int = 1;
pub const JH7110_AONRST_IOMUX: c_int = 2;
pub const JH7110_AONRST_PMU_APB: c_int = 3;
pub const JH7110_AONRST_PMU_WKUP: c_int = 4;
pub const JH7110_AONRST_RTC_APB: c_int = 5;
pub const JH7110_AONRST_RTC_CAL: c_int = 6;
pub const JH7110_AONRST_RTC_32K: c_int = 7;
pub const JH7110_AONRST_END: c_int = 8;
// STGCRG resets
pub const JH7110_STGRST_SYSCON: c_int = 0;
pub const JH7110_STGRST_HIFI4_CORE: c_int = 1;
pub const JH7110_STGRST_HIFI4_AXI: c_int = 2;
pub const JH7110_STGRST_SEC_AHB: c_int = 3;
pub const JH7110_STGRST_E24_CORE: c_int = 4;
pub const JH7110_STGRST_DMA1P_AXI: c_int = 5;
pub const JH7110_STGRST_DMA1P_AHB: c_int = 6;
pub const JH7110_STGRST_USB0_AXI: c_int = 7;
pub const JH7110_STGRST_USB0_APB: c_int = 8;
pub const JH7110_STGRST_USB0_UTMI_APB: c_int = 9;
pub const JH7110_STGRST_USB0_PWRUP: c_int = 10;
pub const JH7110_STGRST_PCIE0_AXI_MST0: c_int = 11;
pub const JH7110_STGRST_PCIE0_AXI_SLV0: c_int = 12;
pub const JH7110_STGRST_PCIE0_AXI_SLV: c_int = 13;
pub const JH7110_STGRST_PCIE0_BRG: c_int = 14;
pub const JH7110_STGRST_PCIE0_CORE: c_int = 15;
pub const JH7110_STGRST_PCIE0_APB: c_int = 16;
pub const JH7110_STGRST_PCIE1_AXI_MST0: c_int = 17;
pub const JH7110_STGRST_PCIE1_AXI_SLV0: c_int = 18;
pub const JH7110_STGRST_PCIE1_AXI_SLV: c_int = 19;
pub const JH7110_STGRST_PCIE1_BRG: c_int = 20;
pub const JH7110_STGRST_PCIE1_CORE: c_int = 21;
pub const JH7110_STGRST_PCIE1_APB: c_int = 22;
pub const JH7110_STGRST_END: c_int = 23;
// ISPCRG resets
pub const JH7110_ISPRST_ISPV2_TOP_WRAPPER_P: c_int = 0;
pub const JH7110_ISPRST_ISPV2_TOP_WRAPPER_C: c_int = 1;
pub const JH7110_ISPRST_M31DPHY_HW: c_int = 2;
pub const JH7110_ISPRST_M31DPHY_B09_AON: c_int = 3;
pub const JH7110_ISPRST_VIN_APB: c_int = 4;
pub const JH7110_ISPRST_VIN_PIXEL_IF0: c_int = 5;
pub const JH7110_ISPRST_VIN_PIXEL_IF1: c_int = 6;
pub const JH7110_ISPRST_VIN_PIXEL_IF2: c_int = 7;
pub const JH7110_ISPRST_VIN_PIXEL_IF3: c_int = 8;
pub const JH7110_ISPRST_VIN_SYS: c_int = 9;
pub const JH7110_ISPRST_VIN_P_AXI_RD: c_int = 10;
pub const JH7110_ISPRST_VIN_P_AXI_WR: c_int = 11;
pub const JH7110_ISPRST_END: c_int = 12;
// VOUTCRG resets
pub const JH7110_VOUTRST_DC8200_AXI: c_int = 0;
pub const JH7110_VOUTRST_DC8200_AHB: c_int = 1;
pub const JH7110_VOUTRST_DC8200_CORE: c_int = 2;
pub const JH7110_VOUTRST_DSITX_DPI: c_int = 3;
pub const JH7110_VOUTRST_DSITX_APB: c_int = 4;
pub const JH7110_VOUTRST_DSITX_RXESC: c_int = 5;
pub const JH7110_VOUTRST_DSITX_SYS: c_int = 6;
pub const JH7110_VOUTRST_DSITX_TXBYTEHS: c_int = 7;
pub const JH7110_VOUTRST_DSITX_TXESC: c_int = 8;
pub const JH7110_VOUTRST_HDMI_TX_HDMI: c_int = 9;
pub const JH7110_VOUTRST_MIPITX_DPHY_SYS: c_int = 10;
pub const JH7110_VOUTRST_MIPITX_DPHY_TXBYTEHS: c_int = 11;
pub const JH7110_VOUTRST_END: c_int = 12;
