//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/clock/sun20i-d1-ccu.h
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


// SPDX-License-Identifier: (GPL-2.0+ OR MIT)
//
// Copyright (C) 2020 huangzhenwei@allwinnertech.com
// Copyright (C) 2021 Samuel Holland <samuel@sholland.org>
//
pub const CLK_PLL_CPUX: c_int = 0;
pub const CLK_PLL_DDR0: c_int = 1;
pub const CLK_PLL_PERIPH0_4X: c_int = 2;
pub const CLK_PLL_PERIPH0_2X: c_int = 3;
pub const CLK_PLL_PERIPH0_800M: c_int = 4;
pub const CLK_PLL_PERIPH0: c_int = 5;
pub const CLK_PLL_PERIPH0_DIV3: c_int = 6;
pub const CLK_PLL_VIDEO0_4X: c_int = 7;
pub const CLK_PLL_VIDEO0_2X: c_int = 8;
pub const CLK_PLL_VIDEO0: c_int = 9;
pub const CLK_PLL_VIDEO1_4X: c_int = 10;
pub const CLK_PLL_VIDEO1_2X: c_int = 11;
pub const CLK_PLL_VIDEO1: c_int = 12;
pub const CLK_PLL_VE: c_int = 13;
pub const CLK_PLL_AUDIO0_4X: c_int = 14;
pub const CLK_PLL_AUDIO0_2X: c_int = 15;
pub const CLK_PLL_AUDIO0: c_int = 16;
pub const CLK_PLL_AUDIO1: c_int = 17;
pub const CLK_PLL_AUDIO1_DIV2: c_int = 18;
pub const CLK_PLL_AUDIO1_DIV5: c_int = 19;
pub const CLK_CPUX: c_int = 20;
pub const CLK_CPUX_AXI: c_int = 21;
pub const CLK_CPUX_APB: c_int = 22;
pub const CLK_PSI_AHB: c_int = 23;
pub const CLK_APB0: c_int = 24;
pub const CLK_APB1: c_int = 25;
pub const CLK_MBUS: c_int = 26;
pub const CLK_DE: c_int = 27;
pub const CLK_BUS_DE: c_int = 28;
pub const CLK_DI: c_int = 29;
pub const CLK_BUS_DI: c_int = 30;
pub const CLK_G2D: c_int = 31;
pub const CLK_BUS_G2D: c_int = 32;
pub const CLK_CE: c_int = 33;
pub const CLK_BUS_CE: c_int = 34;
pub const CLK_VE: c_int = 35;
pub const CLK_BUS_VE: c_int = 36;
pub const CLK_BUS_DMA: c_int = 37;
pub const CLK_BUS_MSGBOX0: c_int = 38;
pub const CLK_BUS_MSGBOX1: c_int = 39;
pub const CLK_BUS_MSGBOX2: c_int = 40;
pub const CLK_BUS_SPINLOCK: c_int = 41;
pub const CLK_BUS_HSTIMER: c_int = 42;
pub const CLK_AVS: c_int = 43;
pub const CLK_BUS_DBG: c_int = 44;
pub const CLK_BUS_PWM: c_int = 45;
pub const CLK_BUS_IOMMU: c_int = 46;
pub const CLK_DRAM: c_int = 47;
pub const CLK_MBUS_DMA: c_int = 48;
pub const CLK_MBUS_VE: c_int = 49;
pub const CLK_MBUS_CE: c_int = 50;
pub const CLK_MBUS_TVIN: c_int = 51;
pub const CLK_MBUS_CSI: c_int = 52;
pub const CLK_MBUS_G2D: c_int = 53;
pub const CLK_MBUS_RISCV: c_int = 54;
pub const CLK_BUS_DRAM: c_int = 55;
pub const CLK_MMC0: c_int = 56;
pub const CLK_MMC1: c_int = 57;
pub const CLK_MMC2: c_int = 58;
pub const CLK_BUS_MMC0: c_int = 59;
pub const CLK_BUS_MMC1: c_int = 60;
pub const CLK_BUS_MMC2: c_int = 61;
pub const CLK_BUS_UART0: c_int = 62;
pub const CLK_BUS_UART1: c_int = 63;
pub const CLK_BUS_UART2: c_int = 64;
pub const CLK_BUS_UART3: c_int = 65;
pub const CLK_BUS_UART4: c_int = 66;
pub const CLK_BUS_UART5: c_int = 67;
pub const CLK_BUS_I2C0: c_int = 68;
pub const CLK_BUS_I2C1: c_int = 69;
pub const CLK_BUS_I2C2: c_int = 70;
pub const CLK_BUS_I2C3: c_int = 71;
pub const CLK_SPI0: c_int = 72;
pub const CLK_SPI1: c_int = 73;
pub const CLK_BUS_SPI0: c_int = 74;
pub const CLK_BUS_SPI1: c_int = 75;
pub const CLK_EMAC_25M: c_int = 76;
pub const CLK_BUS_EMAC: c_int = 77;
pub const CLK_IR_TX: c_int = 78;
pub const CLK_BUS_IR_TX: c_int = 79;
pub const CLK_BUS_GPADC: c_int = 80;
pub const CLK_BUS_THS: c_int = 81;
pub const CLK_I2S0: c_int = 82;
pub const CLK_I2S1: c_int = 83;
pub const CLK_I2S2: c_int = 84;
pub const CLK_I2S2_ASRC: c_int = 85;
pub const CLK_BUS_I2S0: c_int = 86;
pub const CLK_BUS_I2S1: c_int = 87;
pub const CLK_BUS_I2S2: c_int = 88;
pub const CLK_SPDIF_TX: c_int = 89;
pub const CLK_SPDIF_RX: c_int = 90;
pub const CLK_BUS_SPDIF: c_int = 91;
pub const CLK_DMIC: c_int = 92;
pub const CLK_BUS_DMIC: c_int = 93;
pub const CLK_AUDIO_DAC: c_int = 94;
pub const CLK_AUDIO_ADC: c_int = 95;
pub const CLK_BUS_AUDIO: c_int = 96;
pub const CLK_USB_OHCI0: c_int = 97;
pub const CLK_USB_OHCI1: c_int = 98;
pub const CLK_BUS_OHCI0: c_int = 99;
pub const CLK_BUS_OHCI1: c_int = 100;
pub const CLK_BUS_EHCI0: c_int = 101;
pub const CLK_BUS_EHCI1: c_int = 102;
pub const CLK_BUS_OTG: c_int = 103;
pub const CLK_BUS_LRADC: c_int = 104;
pub const CLK_BUS_DPSS_TOP: c_int = 105;
pub const CLK_HDMI_24M: c_int = 106;
pub const CLK_HDMI_CEC_32K: c_int = 107;
pub const CLK_HDMI_CEC: c_int = 108;
pub const CLK_BUS_HDMI: c_int = 109;
pub const CLK_MIPI_DSI: c_int = 110;
pub const CLK_BUS_MIPI_DSI: c_int = 111;
pub const CLK_TCON_LCD0: c_int = 112;
pub const CLK_BUS_TCON_LCD0: c_int = 113;
pub const CLK_TCON_TV: c_int = 114;
pub const CLK_BUS_TCON_TV: c_int = 115;
pub const CLK_TVE: c_int = 116;
pub const CLK_BUS_TVE_TOP: c_int = 117;
pub const CLK_BUS_TVE: c_int = 118;
pub const CLK_TVD: c_int = 119;
pub const CLK_BUS_TVD_TOP: c_int = 120;
pub const CLK_BUS_TVD: c_int = 121;
pub const CLK_LEDC: c_int = 122;
pub const CLK_BUS_LEDC: c_int = 123;
pub const CLK_CSI_TOP: c_int = 124;
pub const CLK_CSI_MCLK: c_int = 125;
pub const CLK_BUS_CSI: c_int = 126;
pub const CLK_TPADC: c_int = 127;
pub const CLK_BUS_TPADC: c_int = 128;
pub const CLK_BUS_TZMA: c_int = 129;
pub const CLK_DSP: c_int = 130;
pub const CLK_BUS_DSP_CFG: c_int = 131;
pub const CLK_RISCV: c_int = 132;
pub const CLK_RISCV_AXI: c_int = 133;
pub const CLK_BUS_RISCV_CFG: c_int = 134;
pub const CLK_FANOUT_24M: c_int = 135;
pub const CLK_FANOUT_12M: c_int = 136;
pub const CLK_FANOUT_16M: c_int = 137;
pub const CLK_FANOUT_25M: c_int = 138;
pub const CLK_FANOUT_32K: c_int = 139;
pub const CLK_FANOUT_27M: c_int = 140;
pub const CLK_FANOUT_PCLK: c_int = 141;
pub const CLK_FANOUT0: c_int = 142;
pub const CLK_FANOUT1: c_int = 143;
pub const CLK_FANOUT2: c_int = 144;
pub const CLK_BUS_CAN0: c_int = 145;
pub const CLK_BUS_CAN1: c_int = 146;
