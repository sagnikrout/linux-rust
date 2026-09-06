//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/pinctrl/am33xx.h
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
// This header provides constants specific to AM33XX pinctrl bindings.
//

// am33xx specific mux bit defines

pub const SLEWCTRL_FAST: c_int = 0;
// update macro depending on INPUT_EN and PULL_ENA

pub const PIN_OUTPUT_PULLDOWN: c_int = 0;

// undef non-existing modes

pub const AM335X_PIN_OFFSET_MIN: c_uint = 0x0800U;
pub const AM335X_PIN_GPMC_AD0: c_uint = 0x800;
pub const AM335X_PIN_GPMC_AD1: c_uint = 0x804;
pub const AM335X_PIN_GPMC_AD2: c_uint = 0x808;
pub const AM335X_PIN_GPMC_AD3: c_uint = 0x80c;
pub const AM335X_PIN_GPMC_AD4: c_uint = 0x810;
pub const AM335X_PIN_GPMC_AD5: c_uint = 0x814;
pub const AM335X_PIN_GPMC_AD6: c_uint = 0x818;
pub const AM335X_PIN_GPMC_AD7: c_uint = 0x81c;
pub const AM335X_PIN_GPMC_AD8: c_uint = 0x820;
pub const AM335X_PIN_GPMC_AD9: c_uint = 0x824;
pub const AM335X_PIN_GPMC_AD10: c_uint = 0x828;
pub const AM335X_PIN_GPMC_AD11: c_uint = 0x82c;
pub const AM335X_PIN_GPMC_AD12: c_uint = 0x830;
pub const AM335X_PIN_GPMC_AD13: c_uint = 0x834;
pub const AM335X_PIN_GPMC_AD14: c_uint = 0x838;
pub const AM335X_PIN_GPMC_AD15: c_uint = 0x83c;
pub const AM335X_PIN_GPMC_A0: c_uint = 0x840;
pub const AM335X_PIN_GPMC_A1: c_uint = 0x844;
pub const AM335X_PIN_GPMC_A2: c_uint = 0x848;
pub const AM335X_PIN_GPMC_A3: c_uint = 0x84c;
pub const AM335X_PIN_GPMC_A4: c_uint = 0x850;
pub const AM335X_PIN_GPMC_A5: c_uint = 0x854;
pub const AM335X_PIN_GPMC_A6: c_uint = 0x858;
pub const AM335X_PIN_GPMC_A7: c_uint = 0x85c;
pub const AM335X_PIN_GPMC_A8: c_uint = 0x860;
pub const AM335X_PIN_GPMC_A9: c_uint = 0x864;
pub const AM335X_PIN_GPMC_A10: c_uint = 0x868;
pub const AM335X_PIN_GPMC_A11: c_uint = 0x86c;
pub const AM335X_PIN_GPMC_WAIT0: c_uint = 0x870;
pub const AM335X_PIN_GPMC_WPN: c_uint = 0x874;
pub const AM335X_PIN_GPMC_BEN1: c_uint = 0x878;
pub const AM335X_PIN_GPMC_CSN0: c_uint = 0x87c;
pub const AM335X_PIN_GPMC_CSN1: c_uint = 0x880;
pub const AM335X_PIN_GPMC_CSN2: c_uint = 0x884;
pub const AM335X_PIN_GPMC_CSN3: c_uint = 0x888;
pub const AM335X_PIN_GPMC_CLK: c_uint = 0x88c;
pub const AM335X_PIN_GPMC_ADVN_ALE: c_uint = 0x890;
pub const AM335X_PIN_GPMC_OEN_REN: c_uint = 0x894;
pub const AM335X_PIN_GPMC_WEN: c_uint = 0x898;
pub const AM335X_PIN_GPMC_BEN0_CLE: c_uint = 0x89c;
pub const AM335X_PIN_LCD_DATA0: c_uint = 0x8a0;
pub const AM335X_PIN_LCD_DATA1: c_uint = 0x8a4;
pub const AM335X_PIN_LCD_DATA2: c_uint = 0x8a8;
pub const AM335X_PIN_LCD_DATA3: c_uint = 0x8ac;
pub const AM335X_PIN_LCD_DATA4: c_uint = 0x8b0;
pub const AM335X_PIN_LCD_DATA5: c_uint = 0x8b4;
pub const AM335X_PIN_LCD_DATA6: c_uint = 0x8b8;
pub const AM335X_PIN_LCD_DATA7: c_uint = 0x8bc;
pub const AM335X_PIN_LCD_DATA8: c_uint = 0x8c0;
pub const AM335X_PIN_LCD_DATA9: c_uint = 0x8c4;
pub const AM335X_PIN_LCD_DATA10: c_uint = 0x8c8;
pub const AM335X_PIN_LCD_DATA11: c_uint = 0x8cc;
pub const AM335X_PIN_LCD_DATA12: c_uint = 0x8d0;
pub const AM335X_PIN_LCD_DATA13: c_uint = 0x8d4;
pub const AM335X_PIN_LCD_DATA14: c_uint = 0x8d8;
pub const AM335X_PIN_LCD_DATA15: c_uint = 0x8dc;
pub const AM335X_PIN_LCD_VSYNC: c_uint = 0x8e0;
pub const AM335X_PIN_LCD_HSYNC: c_uint = 0x8e4;
pub const AM335X_PIN_LCD_PCLK: c_uint = 0x8e8;
pub const AM335X_PIN_LCD_AC_BIAS_EN: c_uint = 0x8ec;
pub const AM335X_PIN_MMC0_DAT3: c_uint = 0x8f0;
pub const AM335X_PIN_MMC0_DAT2: c_uint = 0x8f4;
pub const AM335X_PIN_MMC0_DAT1: c_uint = 0x8f8;
pub const AM335X_PIN_MMC0_DAT0: c_uint = 0x8fc;
pub const AM335X_PIN_MMC0_CLK: c_uint = 0x900;
pub const AM335X_PIN_MMC0_CMD: c_uint = 0x904;
pub const AM335X_PIN_MII1_COL: c_uint = 0x908;
pub const AM335X_PIN_MII1_CRS: c_uint = 0x90c;
pub const AM335X_PIN_MII1_RX_ER: c_uint = 0x910;
pub const AM335X_PIN_MII1_TX_EN: c_uint = 0x914;
pub const AM335X_PIN_MII1_RX_DV: c_uint = 0x918;
pub const AM335X_PIN_MII1_TXD3: c_uint = 0x91c;
pub const AM335X_PIN_MII1_TXD2: c_uint = 0x920;
pub const AM335X_PIN_MII1_TXD1: c_uint = 0x924;
pub const AM335X_PIN_MII1_TXD0: c_uint = 0x928;
pub const AM335X_PIN_MII1_TX_CLK: c_uint = 0x92c;
pub const AM335X_PIN_MII1_RX_CLK: c_uint = 0x930;
pub const AM335X_PIN_MII1_RXD3: c_uint = 0x934;
pub const AM335X_PIN_MII1_RXD2: c_uint = 0x938;
pub const AM335X_PIN_MII1_RXD1: c_uint = 0x93c;
pub const AM335X_PIN_MII1_RXD0: c_uint = 0x940;
pub const AM335X_PIN_RMII1_REF_CLK: c_uint = 0x944;
pub const AM335X_PIN_MDIO: c_uint = 0x948;
pub const AM335X_PIN_MDC: c_uint = 0x94c;
pub const AM335X_PIN_SPI0_SCLK: c_uint = 0x950;
pub const AM335X_PIN_SPI0_D0: c_uint = 0x954;
pub const AM335X_PIN_SPI0_D1: c_uint = 0x958;
pub const AM335X_PIN_SPI0_CS0: c_uint = 0x95c;
pub const AM335X_PIN_SPI0_CS1: c_uint = 0x960;
pub const AM335X_PIN_ECAP0_IN_PWM0_OUT: c_uint = 0x964;
pub const AM335X_PIN_UART0_CTSN: c_uint = 0x968;
pub const AM335X_PIN_UART0_RTSN: c_uint = 0x96c;
pub const AM335X_PIN_UART0_RXD: c_uint = 0x970;
pub const AM335X_PIN_UART0_TXD: c_uint = 0x974;
pub const AM335X_PIN_UART1_CTSN: c_uint = 0x978;
pub const AM335X_PIN_UART1_RTSN: c_uint = 0x97c;
pub const AM335X_PIN_UART1_RXD: c_uint = 0x980;
pub const AM335X_PIN_UART1_TXD: c_uint = 0x984;
pub const AM335X_PIN_I2C0_SDA: c_uint = 0x988;
pub const AM335X_PIN_I2C0_SCL: c_uint = 0x98c;
pub const AM335X_PIN_MCASP0_ACLKX: c_uint = 0x990;
pub const AM335X_PIN_MCASP0_FSX: c_uint = 0x994;
pub const AM335X_PIN_MCASP0_AXR0: c_uint = 0x998;
pub const AM335X_PIN_MCASP0_AHCLKR: c_uint = 0x99c;
pub const AM335X_PIN_MCASP0_ACLKR: c_uint = 0x9a0;
pub const AM335X_PIN_MCASP0_FSR: c_uint = 0x9a4;
pub const AM335X_PIN_MCASP0_AXR1: c_uint = 0x9a8;
pub const AM335X_PIN_MCASP0_AHCLKX: c_uint = 0x9ac;
pub const AM335X_PIN_XDMA_EVENT_INTR0: c_uint = 0x9b0;
pub const AM335X_PIN_XDMA_EVENT_INTR1: c_uint = 0x9b4;
pub const AM335X_PIN_WARMRSTN: c_uint = 0x9b8;
pub const AM335X_PIN_NNMI: c_uint = 0x9c0;
pub const AM335X_PIN_TMS: c_uint = 0x9d0;
pub const AM335X_PIN_TDI: c_uint = 0x9d4;
pub const AM335X_PIN_TDO: c_uint = 0x9d8;
pub const AM335X_PIN_TCK: c_uint = 0x9dc;
pub const AM335X_PIN_TRSTN: c_uint = 0x9e0;
pub const AM335X_PIN_EMU0: c_uint = 0x9e4;
pub const AM335X_PIN_EMU1: c_uint = 0x9e8;
pub const AM335X_PIN_RTC_PWRONRSTN: c_uint = 0x9f8;
pub const AM335X_PIN_PMIC_POWER_EN: c_uint = 0x9fc;
pub const AM335X_PIN_EXT_WAKEUP: c_uint = 0xa00;
pub const AM335X_PIN_USB0_DRVVBUS: c_uint = 0xa1c;
pub const AM335X_PIN_USB1_DRVVBUS: c_uint = 0xa34;
pub const AM335X_PIN_OFFSET_MAX: c_uint = 0x0a34U;
