//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/boot/dts/starfive/jh7110-pinfunc.h
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
// mux bits:
// | 31 - 24 | 23 - 16 | 15 - 10 |  9 - 8   |  7 - 0  |
// |  din    |  dout   |  doen   | function | gpio nr |
//
// dout:     output signal
// doen:     output enable signal
// din:      optional input signal, 0xff = none
// function: function selector
// gpio nr:  gpio number, 0 - 63
//

// sys_iomux dout
pub const GPOUT_LOW: c_int = 0;
pub const GPOUT_HIGH: c_int = 1;
pub const GPOUT_SYS_WAVE511_UART_TX: c_int = 2;
pub const GPOUT_SYS_CAN0_STBY: c_int = 3;
pub const GPOUT_SYS_CAN0_TST_NEXT_BIT: c_int = 4;
pub const GPOUT_SYS_CAN0_TST_SAMPLE_POINT: c_int = 5;
pub const GPOUT_SYS_CAN0_TXD: c_int = 6;
pub const GPOUT_SYS_USB_DRIVE_VBUS: c_int = 7;
pub const GPOUT_SYS_QSPI_CS1: c_int = 8;
pub const GPOUT_SYS_SPDIF: c_int = 9;
pub const GPOUT_SYS_HDMI_CEC_SDA: c_int = 10;
pub const GPOUT_SYS_HDMI_DDC_SCL: c_int = 11;
pub const GPOUT_SYS_HDMI_DDC_SDA: c_int = 12;
pub const GPOUT_SYS_WATCHDOG: c_int = 13;
pub const GPOUT_SYS_I2C0_CLK: c_int = 14;
pub const GPOUT_SYS_I2C0_DATA: c_int = 15;
pub const GPOUT_SYS_SDIO0_BACK_END_POWER: c_int = 16;
pub const GPOUT_SYS_SDIO0_CARD_POWER_EN: c_int = 17;
pub const GPOUT_SYS_SDIO0_CCMD_OD_PULLUP_EN: c_int = 18;
pub const GPOUT_SYS_SDIO0_RST: c_int = 19;
pub const GPOUT_SYS_UART0_TX: c_int = 20;
pub const GPOUT_SYS_HIFI4_JTAG_TDO: c_int = 21;
pub const GPOUT_SYS_JTAG_TDO: c_int = 22;
pub const GPOUT_SYS_PDM_MCLK: c_int = 23;
pub const GPOUT_SYS_PWM_CHANNEL0: c_int = 24;
pub const GPOUT_SYS_PWM_CHANNEL1: c_int = 25;
pub const GPOUT_SYS_PWM_CHANNEL2: c_int = 26;
pub const GPOUT_SYS_PWM_CHANNEL3: c_int = 27;
pub const GPOUT_SYS_PWMDAC_LEFT: c_int = 28;
pub const GPOUT_SYS_PWMDAC_RIGHT: c_int = 29;
pub const GPOUT_SYS_SPI0_CLK: c_int = 30;
pub const GPOUT_SYS_SPI0_FSS: c_int = 31;
pub const GPOUT_SYS_SPI0_TXD: c_int = 32;
pub const GPOUT_SYS_GMAC_PHYCLK: c_int = 33;
pub const GPOUT_SYS_I2SRX_BCLK: c_int = 34;
pub const GPOUT_SYS_I2SRX_LRCK: c_int = 35;
pub const GPOUT_SYS_I2STX0_BCLK: c_int = 36;
pub const GPOUT_SYS_I2STX0_LRCK: c_int = 37;
pub const GPOUT_SYS_MCLK: c_int = 38;
pub const GPOUT_SYS_TDM_CLK: c_int = 39;
pub const GPOUT_SYS_TDM_SYNC: c_int = 40;
pub const GPOUT_SYS_TDM_TXD: c_int = 41;
pub const GPOUT_SYS_TRACE_DATA0: c_int = 42;
pub const GPOUT_SYS_TRACE_DATA1: c_int = 43;
pub const GPOUT_SYS_TRACE_DATA2: c_int = 44;
pub const GPOUT_SYS_TRACE_DATA3: c_int = 45;
pub const GPOUT_SYS_TRACE_REF: c_int = 46;
pub const GPOUT_SYS_CAN1_STBY: c_int = 47;
pub const GPOUT_SYS_CAN1_TST_NEXT_BIT: c_int = 48;
pub const GPOUT_SYS_CAN1_TST_SAMPLE_POINT: c_int = 49;
pub const GPOUT_SYS_CAN1_TXD: c_int = 50;
pub const GPOUT_SYS_I2C1_CLK: c_int = 51;
pub const GPOUT_SYS_I2C1_DATA: c_int = 52;
pub const GPOUT_SYS_SDIO1_BACK_END_POWER: c_int = 53;
pub const GPOUT_SYS_SDIO1_CARD_POWER_EN: c_int = 54;
pub const GPOUT_SYS_SDIO1_CLK: c_int = 55;
pub const GPOUT_SYS_SDIO1_CMD_OD_PULLUP_EN: c_int = 56;
pub const GPOUT_SYS_SDIO1_CMD: c_int = 57;
pub const GPOUT_SYS_SDIO1_DATA0: c_int = 58;
pub const GPOUT_SYS_SDIO1_DATA1: c_int = 59;
pub const GPOUT_SYS_SDIO1_DATA2: c_int = 60;
pub const GPOUT_SYS_SDIO1_DATA3: c_int = 61;
pub const GPOUT_SYS_SDIO1_DATA4: c_int = 62;
pub const GPOUT_SYS_SDIO1_DATA5: c_int = 63;
pub const GPOUT_SYS_SDIO1_DATA6: c_int = 64;
pub const GPOUT_SYS_SDIO1_DATA7: c_int = 65;
pub const GPOUT_SYS_SDIO1_RST: c_int = 66;
pub const GPOUT_SYS_UART1_RTS: c_int = 67;
pub const GPOUT_SYS_UART1_TX: c_int = 68;
pub const GPOUT_SYS_I2STX1_SDO0: c_int = 69;
pub const GPOUT_SYS_I2STX1_SDO1: c_int = 70;
pub const GPOUT_SYS_I2STX1_SDO2: c_int = 71;
pub const GPOUT_SYS_I2STX1_SDO3: c_int = 72;
pub const GPOUT_SYS_SPI1_CLK: c_int = 73;
pub const GPOUT_SYS_SPI1_FSS: c_int = 74;
pub const GPOUT_SYS_SPI1_TXD: c_int = 75;
pub const GPOUT_SYS_I2C2_CLK: c_int = 76;
pub const GPOUT_SYS_I2C2_DATA: c_int = 77;
pub const GPOUT_SYS_UART2_RTS: c_int = 78;
pub const GPOUT_SYS_UART2_TX: c_int = 79;
pub const GPOUT_SYS_SPI2_CLK: c_int = 80;
pub const GPOUT_SYS_SPI2_FSS: c_int = 81;
pub const GPOUT_SYS_SPI2_TXD: c_int = 82;
pub const GPOUT_SYS_I2C3_CLK: c_int = 83;
pub const GPOUT_SYS_I2C3_DATA: c_int = 84;
pub const GPOUT_SYS_UART3_TX: c_int = 85;
pub const GPOUT_SYS_SPI3_CLK: c_int = 86;
pub const GPOUT_SYS_SPI3_FSS: c_int = 87;
pub const GPOUT_SYS_SPI3_TXD: c_int = 88;
pub const GPOUT_SYS_I2C4_CLK: c_int = 89;
pub const GPOUT_SYS_I2C4_DATA: c_int = 90;
pub const GPOUT_SYS_UART4_RTS: c_int = 91;
pub const GPOUT_SYS_UART4_TX: c_int = 92;
pub const GPOUT_SYS_SPI4_CLK: c_int = 93;
pub const GPOUT_SYS_SPI4_FSS: c_int = 94;
pub const GPOUT_SYS_SPI4_TXD: c_int = 95;
pub const GPOUT_SYS_I2C5_CLK: c_int = 96;
pub const GPOUT_SYS_I2C5_DATA: c_int = 97;
pub const GPOUT_SYS_UART5_RTS: c_int = 98;
pub const GPOUT_SYS_UART5_TX: c_int = 99;
pub const GPOUT_SYS_SPI5_CLK: c_int = 100;
pub const GPOUT_SYS_SPI5_FSS: c_int = 101;
pub const GPOUT_SYS_SPI5_TXD: c_int = 102;
pub const GPOUT_SYS_I2C6_CLK: c_int = 103;
pub const GPOUT_SYS_I2C6_DATA: c_int = 104;
pub const GPOUT_SYS_SPI6_CLK: c_int = 105;
pub const GPOUT_SYS_SPI6_FSS: c_int = 106;
pub const GPOUT_SYS_SPI6_TXD: c_int = 107;
// aon_iomux dout
pub const GPOUT_AON_CLK_32K_OUT: c_int = 2;
pub const GPOUT_AON_PTC0_PWM4: c_int = 3;
pub const GPOUT_AON_PTC0_PWM5: c_int = 4;
pub const GPOUT_AON_PTC0_PWM6: c_int = 5;
pub const GPOUT_AON_PTC0_PWM7: c_int = 6;
pub const GPOUT_AON_CLK_GCLK0: c_int = 7;
pub const GPOUT_AON_CLK_GCLK1: c_int = 8;
pub const GPOUT_AON_CLK_GCLK2: c_int = 9;
// sys_iomux doen
pub const GPOEN_ENABLE: c_int = 0;
pub const GPOEN_DISABLE: c_int = 1;
pub const GPOEN_SYS_HDMI_CEC_SDA: c_int = 2;
pub const GPOEN_SYS_HDMI_DDC_SCL: c_int = 3;
pub const GPOEN_SYS_HDMI_DDC_SDA: c_int = 4;
pub const GPOEN_SYS_I2C0_CLK: c_int = 5;
pub const GPOEN_SYS_I2C0_DATA: c_int = 6;
pub const GPOEN_SYS_HIFI4_JTAG_TDO: c_int = 7;
pub const GPOEN_SYS_JTAG_TDO: c_int = 8;
pub const GPOEN_SYS_PWM0_CHANNEL0: c_int = 9;
pub const GPOEN_SYS_PWM0_CHANNEL1: c_int = 10;
pub const GPOEN_SYS_PWM0_CHANNEL2: c_int = 11;
pub const GPOEN_SYS_PWM0_CHANNEL3: c_int = 12;
pub const GPOEN_SYS_SPI0_NSSPCTL: c_int = 13;
pub const GPOEN_SYS_SPI0_NSSP: c_int = 14;
pub const GPOEN_SYS_TDM_SYNC: c_int = 15;
pub const GPOEN_SYS_TDM_TXD: c_int = 16;
pub const GPOEN_SYS_I2C1_CLK: c_int = 17;
pub const GPOEN_SYS_I2C1_DATA: c_int = 18;
pub const GPOEN_SYS_SDIO1_CMD: c_int = 19;
pub const GPOEN_SYS_SDIO1_DATA0: c_int = 20;
pub const GPOEN_SYS_SDIO1_DATA1: c_int = 21;
pub const GPOEN_SYS_SDIO1_DATA2: c_int = 22;
pub const GPOEN_SYS_SDIO1_DATA3: c_int = 23;
pub const GPOEN_SYS_SDIO1_DATA4: c_int = 24;
pub const GPOEN_SYS_SDIO1_DATA5: c_int = 25;
pub const GPOEN_SYS_SDIO1_DATA6: c_int = 26;
pub const GPOEN_SYS_SDIO1_DATA7: c_int = 27;
pub const GPOEN_SYS_SPI1_NSSPCTL: c_int = 28;
pub const GPOEN_SYS_SPI1_NSSP: c_int = 29;
pub const GPOEN_SYS_I2C2_CLK: c_int = 30;
pub const GPOEN_SYS_I2C2_DATA: c_int = 31;
pub const GPOEN_SYS_SPI2_NSSPCTL: c_int = 32;
pub const GPOEN_SYS_SPI2_NSSP: c_int = 33;
pub const GPOEN_SYS_I2C3_CLK: c_int = 34;
pub const GPOEN_SYS_I2C3_DATA: c_int = 35;
pub const GPOEN_SYS_SPI3_NSSPCTL: c_int = 36;
pub const GPOEN_SYS_SPI3_NSSP: c_int = 37;
pub const GPOEN_SYS_I2C4_CLK: c_int = 38;
pub const GPOEN_SYS_I2C4_DATA: c_int = 39;
pub const GPOEN_SYS_SPI4_NSSPCTL: c_int = 40;
pub const GPOEN_SYS_SPI4_NSSP: c_int = 41;
pub const GPOEN_SYS_I2C5_CLK: c_int = 42;
pub const GPOEN_SYS_I2C5_DATA: c_int = 43;
pub const GPOEN_SYS_SPI5_NSSPCTL: c_int = 44;
pub const GPOEN_SYS_SPI5_NSSP: c_int = 45;
pub const GPOEN_SYS_I2C6_CLK: c_int = 46;
pub const GPOEN_SYS_I2C6_DATA: c_int = 47;
pub const GPOEN_SYS_SPI6_NSSPCTL: c_int = 48;
pub const GPOEN_SYS_SPI6_NSSP: c_int = 49;
// aon_iomux doen
pub const GPOEN_AON_PTC0_OE_N_4: c_int = 2;
pub const GPOEN_AON_PTC0_OE_N_5: c_int = 3;
pub const GPOEN_AON_PTC0_OE_N_6: c_int = 4;
pub const GPOEN_AON_PTC0_OE_N_7: c_int = 5;
// sys_iomux gin
pub const GPI_NONE: c_int = 255;
pub const GPI_SYS_WAVE511_UART_RX: c_int = 0;
pub const GPI_SYS_CAN0_RXD: c_int = 1;
pub const GPI_SYS_USB_OVERCURRENT: c_int = 2;
pub const GPI_SYS_SPDIF: c_int = 3;
pub const GPI_SYS_JTAG_RST: c_int = 4;
pub const GPI_SYS_HDMI_CEC_SDA: c_int = 5;
pub const GPI_SYS_HDMI_DDC_SCL: c_int = 6;
pub const GPI_SYS_HDMI_DDC_SDA: c_int = 7;
pub const GPI_SYS_HDMI_HPD: c_int = 8;
pub const GPI_SYS_I2C0_CLK: c_int = 9;
pub const GPI_SYS_I2C0_DATA: c_int = 10;
pub const GPI_SYS_SDIO0_CD: c_int = 11;
pub const GPI_SYS_SDIO0_INT: c_int = 12;
pub const GPI_SYS_SDIO0_WP: c_int = 13;
pub const GPI_SYS_UART0_RX: c_int = 14;
pub const GPI_SYS_HIFI4_JTAG_TCK: c_int = 15;
pub const GPI_SYS_HIFI4_JTAG_TDI: c_int = 16;
pub const GPI_SYS_HIFI4_JTAG_TMS: c_int = 17;
pub const GPI_SYS_HIFI4_JTAG_RST: c_int = 18;
pub const GPI_SYS_JTAG_TDI: c_int = 19;
pub const GPI_SYS_JTAG_TMS: c_int = 20;
pub const GPI_SYS_PDM_DMIC0: c_int = 21;
pub const GPI_SYS_PDM_DMIC1: c_int = 22;
pub const GPI_SYS_I2SRX_SDIN0: c_int = 23;
pub const GPI_SYS_I2SRX_SDIN1: c_int = 24;
pub const GPI_SYS_I2SRX_SDIN2: c_int = 25;
pub const GPI_SYS_SPI0_CLK: c_int = 26;
pub const GPI_SYS_SPI0_FSS: c_int = 27;
pub const GPI_SYS_SPI0_RXD: c_int = 28;
pub const GPI_SYS_JTAG_TCK: c_int = 29;
pub const GPI_SYS_MCLK_EXT: c_int = 30;
pub const GPI_SYS_I2SRX_BCLK: c_int = 31;
pub const GPI_SYS_I2SRX_LRCK: c_int = 32;
pub const GPI_SYS_I2STX1_BCLK: c_int = 33;
pub const GPI_SYS_I2STX1_LRCK: c_int = 34;
pub const GPI_SYS_TDM_CLK: c_int = 35;
pub const GPI_SYS_TDM_RXD: c_int = 36;
pub const GPI_SYS_TDM_SYNC: c_int = 37;
pub const GPI_SYS_CAN1_RXD: c_int = 38;
pub const GPI_SYS_I2C1_CLK: c_int = 39;
pub const GPI_SYS_I2C1_DATA: c_int = 40;
pub const GPI_SYS_SDIO1_CD: c_int = 41;
pub const GPI_SYS_SDIO1_INT: c_int = 42;
pub const GPI_SYS_SDIO1_WP: c_int = 43;
pub const GPI_SYS_SDIO1_CMD: c_int = 44;
pub const GPI_SYS_SDIO1_DATA0: c_int = 45;
pub const GPI_SYS_SDIO1_DATA1: c_int = 46;
pub const GPI_SYS_SDIO1_DATA2: c_int = 47;
pub const GPI_SYS_SDIO1_DATA3: c_int = 48;
pub const GPI_SYS_SDIO1_DATA4: c_int = 49;
pub const GPI_SYS_SDIO1_DATA5: c_int = 50;
pub const GPI_SYS_SDIO1_DATA6: c_int = 51;
pub const GPI_SYS_SDIO1_DATA7: c_int = 52;
pub const GPI_SYS_SDIO1_STRB: c_int = 53;
pub const GPI_SYS_UART1_CTS: c_int = 54;
pub const GPI_SYS_UART1_RX: c_int = 55;
pub const GPI_SYS_SPI1_CLK: c_int = 56;
pub const GPI_SYS_SPI1_FSS: c_int = 57;
pub const GPI_SYS_SPI1_RXD: c_int = 58;
pub const GPI_SYS_I2C2_CLK: c_int = 59;
pub const GPI_SYS_I2C2_DATA: c_int = 60;
pub const GPI_SYS_UART2_CTS: c_int = 61;
pub const GPI_SYS_UART2_RX: c_int = 62;
pub const GPI_SYS_SPI2_CLK: c_int = 63;
pub const GPI_SYS_SPI2_FSS: c_int = 64;
pub const GPI_SYS_SPI2_RXD: c_int = 65;
pub const GPI_SYS_I2C3_CLK: c_int = 66;
pub const GPI_SYS_I2C3_DATA: c_int = 67;
pub const GPI_SYS_UART3_RX: c_int = 68;
pub const GPI_SYS_SPI3_CLK: c_int = 69;
pub const GPI_SYS_SPI3_FSS: c_int = 70;
pub const GPI_SYS_SPI3_RXD: c_int = 71;
pub const GPI_SYS_I2C4_CLK: c_int = 72;
pub const GPI_SYS_I2C4_DATA: c_int = 73;
pub const GPI_SYS_UART4_CTS: c_int = 74;
pub const GPI_SYS_UART4_RX: c_int = 75;
pub const GPI_SYS_SPI4_CLK: c_int = 76;
pub const GPI_SYS_SPI4_FSS: c_int = 77;
pub const GPI_SYS_SPI4_RXD: c_int = 78;
pub const GPI_SYS_I2C5_CLK: c_int = 79;
pub const GPI_SYS_I2C5_DATA: c_int = 80;
pub const GPI_SYS_UART5_CTS: c_int = 81;
pub const GPI_SYS_UART5_RX: c_int = 82;
pub const GPI_SYS_SPI5_CLK: c_int = 83;
pub const GPI_SYS_SPI5_FSS: c_int = 84;
pub const GPI_SYS_SPI5_RXD: c_int = 85;
pub const GPI_SYS_I2C6_CLK: c_int = 86;
pub const GPI_SYS_I2C6_DATA: c_int = 87;
pub const GPI_SYS_SPI6_CLK: c_int = 88;
pub const GPI_SYS_SPI6_FSS: c_int = 89;
pub const GPI_SYS_SPI6_RXD: c_int = 90;
// aon_iomux gin
pub const GPI_AON_PMU_GPIO_WAKEUP_0: c_int = 0;
pub const GPI_AON_PMU_GPIO_WAKEUP_1: c_int = 1;
pub const GPI_AON_PMU_GPIO_WAKEUP_2: c_int = 2;
pub const GPI_AON_PMU_GPIO_WAKEUP_3: c_int = 3;
