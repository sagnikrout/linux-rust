//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/pinctrl/pinctrl-starfive-jh7100.h
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
// Copyright (C) 2021 Emil Renner Berthing <kernel@esmil.dk>
//
pub const PAD_GPIO_OFFSET: c_int = 0;
pub const PAD_FUNC_SHARE_OFFSET: c_int = 64;

//
// GPIOMUX bits:
// | 31 - 24 | 23 - 16 | 15 - 8 |     7    |     6    |  5 - 0  |
// |  dout   |  doen   |  din   | dout rev | doen rev | gpio nr |
//
// dout:     output signal
// doen:     output enable signal
// din:      optional input signal, 0xff = none
// dout rev: output signal reverse bit
// doen rev: output enable signal reverse bit
// gpio nr:  gpio number, 0 - 63
//

pub const GPO_REVERSE: c_uint = 0x80000000;
pub const GPO_LOW: c_int = 0;
pub const GPO_HIGH: c_int = 1;
pub const GPO_ENABLE: c_int = 0;
pub const GPO_DISABLE: c_int = 1;
pub const GPO_CLK_GMAC_PAPHYREF: c_int = 2;
pub const GPO_JTAG_TDO: c_int = 3;
pub const GPO_JTAG_TDO_OEN: c_int = 4;
pub const GPO_DMIC_CLK_OUT: c_int = 5;
pub const GPO_DSP_JTDOEN_PAD: c_int = 6;
pub const GPO_DSP_JTDO_PAD: c_int = 7;
pub const GPO_I2C0_PAD_SCK_OE: c_int = 8;

pub const GPO_I2C0_PAD_SDA_OE: c_int = 9;

pub const GPO_I2C1_PAD_SCK_OE: c_int = 10;

pub const GPO_I2C1_PAD_SDA_OE: c_int = 11;

pub const GPO_I2C2_PAD_SCK_OE: c_int = 12;

pub const GPO_I2C2_PAD_SDA_OE: c_int = 13;

pub const GPO_I2C3_PAD_SCK_OE: c_int = 14;

pub const GPO_I2C3_PAD_SDA_OE: c_int = 15;

pub const GPO_I2SRX_BCLK_OUT: c_int = 16;
pub const GPO_I2SRX_BCLK_OUT_OEN: c_int = 17;
pub const GPO_I2SRX_LRCK_OUT: c_int = 18;
pub const GPO_I2SRX_LRCK_OUT_OEN: c_int = 19;
pub const GPO_I2SRX_MCLK_OUT: c_int = 20;
pub const GPO_I2STX_BCLK_OUT: c_int = 21;
pub const GPO_I2STX_BCLK_OUT_OEN: c_int = 22;
pub const GPO_I2STX_LRCK_OUT: c_int = 23;
pub const GPO_I2STX_LRCK_OUT_OEN: c_int = 24;
pub const GPO_I2STX_MCLK_OUT: c_int = 25;
pub const GPO_I2STX_SDOUT0: c_int = 26;
pub const GPO_I2STX_SDOUT1: c_int = 27;
pub const GPO_LCD_PAD_CSM_N: c_int = 28;
pub const GPO_PWM_PAD_OE_N_BIT0: c_int = 29;
pub const GPO_PWM_PAD_OE_N_BIT1: c_int = 30;
pub const GPO_PWM_PAD_OE_N_BIT2: c_int = 31;
pub const GPO_PWM_PAD_OE_N_BIT3: c_int = 32;
pub const GPO_PWM_PAD_OE_N_BIT4: c_int = 33;
pub const GPO_PWM_PAD_OE_N_BIT5: c_int = 34;
pub const GPO_PWM_PAD_OE_N_BIT6: c_int = 35;
pub const GPO_PWM_PAD_OE_N_BIT7: c_int = 36;
pub const GPO_PWM_PAD_OUT_BIT0: c_int = 37;
pub const GPO_PWM_PAD_OUT_BIT1: c_int = 38;
pub const GPO_PWM_PAD_OUT_BIT2: c_int = 39;
pub const GPO_PWM_PAD_OUT_BIT3: c_int = 40;
pub const GPO_PWM_PAD_OUT_BIT4: c_int = 41;
pub const GPO_PWM_PAD_OUT_BIT5: c_int = 42;
pub const GPO_PWM_PAD_OUT_BIT6: c_int = 43;
pub const GPO_PWM_PAD_OUT_BIT7: c_int = 44;
pub const GPO_PWMDAC_LEFT_OUT: c_int = 45;
pub const GPO_PWMDAC_RIGHT_OUT: c_int = 46;
pub const GPO_QSPI_CSN1_OUT: c_int = 47;
pub const GPO_QSPI_CSN2_OUT: c_int = 48;
pub const GPO_QSPI_CSN3_OUT: c_int = 49;
pub const GPO_REGISTER23_SCFG_CMSENSOR_RST0: c_int = 50;
pub const GPO_REGISTER23_SCFG_CMSENSOR_RST1: c_int = 51;
pub const GPO_REGISTER32_SCFG_GMAC_PHY_RSTN: c_int = 52;
pub const GPO_SDIO0_PAD_CARD_POWER_EN: c_int = 53;
pub const GPO_SDIO0_PAD_CCLK_OUT: c_int = 54;
pub const GPO_SDIO0_PAD_CCMD_OE: c_int = 55;

pub const GPO_SDIO0_PAD_CCMD_OUT: c_int = 56;
pub const GPO_SDIO0_PAD_CDATA_OE_BIT0: c_int = 57;

pub const GPO_SDIO0_PAD_CDATA_OE_BIT1: c_int = 58;

pub const GPO_SDIO0_PAD_CDATA_OE_BIT2: c_int = 59;

pub const GPO_SDIO0_PAD_CDATA_OE_BIT3: c_int = 60;

pub const GPO_SDIO0_PAD_CDATA_OE_BIT4: c_int = 61;

pub const GPO_SDIO0_PAD_CDATA_OE_BIT5: c_int = 62;

pub const GPO_SDIO0_PAD_CDATA_OE_BIT6: c_int = 63;

pub const GPO_SDIO0_PAD_CDATA_OE_BIT7: c_int = 64;

pub const GPO_SDIO0_PAD_CDATA_OUT_BIT0: c_int = 65;
pub const GPO_SDIO0_PAD_CDATA_OUT_BIT1: c_int = 66;
pub const GPO_SDIO0_PAD_CDATA_OUT_BIT2: c_int = 67;
pub const GPO_SDIO0_PAD_CDATA_OUT_BIT3: c_int = 68;
pub const GPO_SDIO0_PAD_CDATA_OUT_BIT4: c_int = 69;
pub const GPO_SDIO0_PAD_CDATA_OUT_BIT5: c_int = 70;
pub const GPO_SDIO0_PAD_CDATA_OUT_BIT6: c_int = 71;
pub const GPO_SDIO0_PAD_CDATA_OUT_BIT7: c_int = 72;
pub const GPO_SDIO0_PAD_RST_N: c_int = 73;
pub const GPO_SDIO1_PAD_CARD_POWER_EN: c_int = 74;
pub const GPO_SDIO1_PAD_CCLK_OUT: c_int = 75;
pub const GPO_SDIO1_PAD_CCMD_OE: c_int = 76;

pub const GPO_SDIO1_PAD_CCMD_OUT: c_int = 77;
pub const GPO_SDIO1_PAD_CDATA_OE_BIT0: c_int = 78;

pub const GPO_SDIO1_PAD_CDATA_OE_BIT1: c_int = 79;

pub const GPO_SDIO1_PAD_CDATA_OE_BIT2: c_int = 80;

pub const GPO_SDIO1_PAD_CDATA_OE_BIT3: c_int = 81;

pub const GPO_SDIO1_PAD_CDATA_OE_BIT4: c_int = 82;

pub const GPO_SDIO1_PAD_CDATA_OE_BIT5: c_int = 83;

pub const GPO_SDIO1_PAD_CDATA_OE_BIT6: c_int = 84;

pub const GPO_SDIO1_PAD_CDATA_OE_BIT7: c_int = 85;

pub const GPO_SDIO1_PAD_CDATA_OUT_BIT0: c_int = 86;
pub const GPO_SDIO1_PAD_CDATA_OUT_BIT1: c_int = 87;
pub const GPO_SDIO1_PAD_CDATA_OUT_BIT2: c_int = 88;
pub const GPO_SDIO1_PAD_CDATA_OUT_BIT3: c_int = 89;
pub const GPO_SDIO1_PAD_CDATA_OUT_BIT4: c_int = 90;
pub const GPO_SDIO1_PAD_CDATA_OUT_BIT5: c_int = 91;
pub const GPO_SDIO1_PAD_CDATA_OUT_BIT6: c_int = 92;
pub const GPO_SDIO1_PAD_CDATA_OUT_BIT7: c_int = 93;
pub const GPO_SDIO1_PAD_RST_N: c_int = 94;
pub const GPO_SPDIF_TX_SDOUT: c_int = 95;
pub const GPO_SPDIF_TX_SDOUT_OEN: c_int = 96;
pub const GPO_SPI0_PAD_OE_N: c_int = 97;
pub const GPO_SPI0_PAD_SCK_OUT: c_int = 98;
pub const GPO_SPI0_PAD_SS_0_N: c_int = 99;
pub const GPO_SPI0_PAD_SS_1_N: c_int = 100;
pub const GPO_SPI0_PAD_TXD: c_int = 101;
pub const GPO_SPI1_PAD_OE_N: c_int = 102;
pub const GPO_SPI1_PAD_SCK_OUT: c_int = 103;
pub const GPO_SPI1_PAD_SS_0_N: c_int = 104;
pub const GPO_SPI1_PAD_SS_1_N: c_int = 105;
pub const GPO_SPI1_PAD_TXD: c_int = 106;
pub const GPO_SPI2_PAD_OE_N: c_int = 107;
pub const GPO_SPI2_PAD_SCK_OUT: c_int = 108;
pub const GPO_SPI2_PAD_SS_0_N: c_int = 109;
pub const GPO_SPI2_PAD_SS_1_N: c_int = 110;
pub const GPO_SPI2_PAD_TXD: c_int = 111;
pub const GPO_SPI2AHB_PAD_OE_N_BIT0: c_int = 112;
pub const GPO_SPI2AHB_PAD_OE_N_BIT1: c_int = 113;
pub const GPO_SPI2AHB_PAD_OE_N_BIT2: c_int = 114;
pub const GPO_SPI2AHB_PAD_OE_N_BIT3: c_int = 115;
pub const GPO_SPI2AHB_PAD_TXD_BIT0: c_int = 116;
pub const GPO_SPI2AHB_PAD_TXD_BIT1: c_int = 117;
pub const GPO_SPI2AHB_PAD_TXD_BIT2: c_int = 118;
pub const GPO_SPI2AHB_PAD_TXD_BIT3: c_int = 119;
pub const GPO_SPI3_PAD_OE_N: c_int = 120;
pub const GPO_SPI3_PAD_SCK_OUT: c_int = 121;
pub const GPO_SPI3_PAD_SS_0_N: c_int = 122;
pub const GPO_SPI3_PAD_SS_1_N: c_int = 123;
pub const GPO_SPI3_PAD_TXD: c_int = 124;
pub const GPO_UART0_PAD_DTRN: c_int = 125;
pub const GPO_UART0_PAD_RTSN: c_int = 126;
pub const GPO_UART0_PAD_SOUT: c_int = 127;
pub const GPO_UART1_PAD_SOUT: c_int = 128;
pub const GPO_UART2_PAD_DTR_N: c_int = 129;
pub const GPO_UART2_PAD_RTS_N: c_int = 130;
pub const GPO_UART2_PAD_SOUT: c_int = 131;
pub const GPO_UART3_PAD_SOUT: c_int = 132;
pub const GPO_USB_DRV_BUS: c_int = 133;
pub const GPI_CPU_JTAG_TCK: c_int = 0;
pub const GPI_CPU_JTAG_TDI: c_int = 1;
pub const GPI_CPU_JTAG_TMS: c_int = 2;
pub const GPI_CPU_JTAG_TRST: c_int = 3;
pub const GPI_DMIC_SDIN_BIT0: c_int = 4;
pub const GPI_DMIC_SDIN_BIT1: c_int = 5;
pub const GPI_DSP_JTCK_PAD: c_int = 6;
pub const GPI_DSP_JTDI_PAD: c_int = 7;
pub const GPI_DSP_JTMS_PAD: c_int = 8;
pub const GPI_DSP_TRST_PAD: c_int = 9;
pub const GPI_I2C0_PAD_SCK_IN: c_int = 10;
pub const GPI_I2C0_PAD_SDA_IN: c_int = 11;
pub const GPI_I2C1_PAD_SCK_IN: c_int = 12;
pub const GPI_I2C1_PAD_SDA_IN: c_int = 13;
pub const GPI_I2C2_PAD_SCK_IN: c_int = 14;
pub const GPI_I2C2_PAD_SDA_IN: c_int = 15;
pub const GPI_I2C3_PAD_SCK_IN: c_int = 16;
pub const GPI_I2C3_PAD_SDA_IN: c_int = 17;
pub const GPI_I2SRX_BCLK_IN: c_int = 18;
pub const GPI_I2SRX_LRCK_IN: c_int = 19;
pub const GPI_I2SRX_SDIN_BIT0: c_int = 20;
pub const GPI_I2SRX_SDIN_BIT1: c_int = 21;
pub const GPI_I2SRX_SDIN_BIT2: c_int = 22;
pub const GPI_I2STX_BCLK_IN: c_int = 23;
pub const GPI_I2STX_LRCK_IN: c_int = 24;
pub const GPI_SDIO0_PAD_CARD_DETECT_N: c_int = 25;
pub const GPI_SDIO0_PAD_CARD_WRITE_PRT: c_int = 26;
pub const GPI_SDIO0_PAD_CCMD_IN: c_int = 27;
pub const GPI_SDIO0_PAD_CDATA_IN_BIT0: c_int = 28;
pub const GPI_SDIO0_PAD_CDATA_IN_BIT1: c_int = 29;
pub const GPI_SDIO0_PAD_CDATA_IN_BIT2: c_int = 30;
pub const GPI_SDIO0_PAD_CDATA_IN_BIT3: c_int = 31;
pub const GPI_SDIO0_PAD_CDATA_IN_BIT4: c_int = 32;
pub const GPI_SDIO0_PAD_CDATA_IN_BIT5: c_int = 33;
pub const GPI_SDIO0_PAD_CDATA_IN_BIT6: c_int = 34;
pub const GPI_SDIO0_PAD_CDATA_IN_BIT7: c_int = 35;
pub const GPI_SDIO1_PAD_CARD_DETECT_N: c_int = 36;
pub const GPI_SDIO1_PAD_CARD_WRITE_PRT: c_int = 37;
pub const GPI_SDIO1_PAD_CCMD_IN: c_int = 38;
pub const GPI_SDIO1_PAD_CDATA_IN_BIT0: c_int = 39;
pub const GPI_SDIO1_PAD_CDATA_IN_BIT1: c_int = 40;
pub const GPI_SDIO1_PAD_CDATA_IN_BIT2: c_int = 41;
pub const GPI_SDIO1_PAD_CDATA_IN_BIT3: c_int = 42;
pub const GPI_SDIO1_PAD_CDATA_IN_BIT4: c_int = 43;
pub const GPI_SDIO1_PAD_CDATA_IN_BIT5: c_int = 44;
pub const GPI_SDIO1_PAD_CDATA_IN_BIT6: c_int = 45;
pub const GPI_SDIO1_PAD_CDATA_IN_BIT7: c_int = 46;
pub const GPI_SPDIF_RX_SDIN: c_int = 47;
pub const GPI_SPI0_PAD_RXD: c_int = 48;
pub const GPI_SPI0_PAD_SS_IN_N: c_int = 49;
pub const GPI_SPI1_PAD_RXD: c_int = 50;
pub const GPI_SPI1_PAD_SS_IN_N: c_int = 51;
pub const GPI_SPI2_PAD_RXD: c_int = 52;
pub const GPI_SPI2_PAD_SS_IN_N: c_int = 53;
pub const GPI_SPI2AHB_PAD_RXD_BIT0: c_int = 54;
pub const GPI_SPI2AHB_PAD_RXD_BIT1: c_int = 55;
pub const GPI_SPI2AHB_PAD_RXD_BIT2: c_int = 56;
pub const GPI_SPI2AHB_PAD_RXD_BIT3: c_int = 57;
pub const GPI_SPI2AHB_PAD_SS_N: c_int = 58;
pub const GPI_SPI2AHB_SLV_SCLKIN: c_int = 59;
pub const GPI_SPI3_PAD_RXD: c_int = 60;
pub const GPI_SPI3_PAD_SS_IN_N: c_int = 61;
pub const GPI_UART0_PAD_CTSN: c_int = 62;
pub const GPI_UART0_PAD_DCDN: c_int = 63;
pub const GPI_UART0_PAD_DSRN: c_int = 64;
pub const GPI_UART0_PAD_RIN: c_int = 65;
pub const GPI_UART0_PAD_SIN: c_int = 66;
pub const GPI_UART1_PAD_SIN: c_int = 67;
pub const GPI_UART2_PAD_CTS_N: c_int = 68;
pub const GPI_UART2_PAD_DCD_N: c_int = 69;
pub const GPI_UART2_PAD_DSR_N: c_int = 70;
pub const GPI_UART2_PAD_RI_N: c_int = 71;
pub const GPI_UART2_PAD_SIN: c_int = 72;
pub const GPI_UART3_PAD_SIN: c_int = 73;
pub const GPI_USB_OVER_CURRENT: c_int = 74;
pub const GPI_NONE: c_uint = 0xff;
