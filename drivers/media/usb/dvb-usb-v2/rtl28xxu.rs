//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/dvb-usb-v2/rtl28xxu.h
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
// Realtek RTL28xxU DVB USB driver
//
// Copyright (C) 2009 Antti Palosaari <crope@iki.fi>
// Copyright (C) 2011 Antti Palosaari <crope@iki.fi>
//

//
// USB commands
// (usb_control_msg() index parameter)
//
pub const DEMOD: c_uint = 0x0000;
pub const USB: c_uint = 0x0100;
pub const SYS: c_uint = 0x0200;
pub const I2C: c_uint = 0x0300;
pub const I2C_DA: c_uint = 0x0600;
pub const CMD_WR_FLAG: c_uint = 0x0010;
pub const CMD_DEMOD_RD: c_uint = 0x0000;
pub const CMD_DEMOD_WR: c_uint = 0x0010;
pub const CMD_USB_RD: c_uint = 0x0100;
pub const CMD_USB_WR: c_uint = 0x0110;
pub const CMD_SYS_RD: c_uint = 0x0200;
pub const CMD_IR_RD: c_uint = 0x0201;
pub const CMD_IR_WR: c_uint = 0x0211;
pub const CMD_SYS_WR: c_uint = 0x0210;
pub const CMD_I2C_RD: c_uint = 0x0300;
pub const CMD_I2C_WR: c_uint = 0x0310;
pub const CMD_I2C_DA_RD: c_uint = 0x0600;
pub const CMD_I2C_DA_WR: c_uint = 0x0610;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl28xxu_dev {
    pub buf: [u8; 128],
    pub chip_id: u8,
    pub tuner: u8,
    pub tuner_name: *mut c_char,
    pub /: *mut *mut u8 page; / integrated demod active register page,
    pub demod_i2c_adapter: *mut i2c_adapter,
    pub rc_active: bool,
    pub new_i2c_write: bool,
    pub i2c_client_demod: *mut i2c_client,
    pub i2c_client_tuner: *mut i2c_client,
    pub i2c_client_slave_demod: *mut i2c_client,
    pub platform_device_sdr: *mut platform_device,
pub const SLAVE_DEMOD_NONE: c_int = 0;
pub const SLAVE_DEMOD_MN88472: c_int = 1;
pub const SLAVE_DEMOD_MN88473: c_int = 2;
pub const SLAVE_DEMOD_SI2168: c_int = 3;
pub const SLAVE_DEMOD_CXD2837ER: c_int = 4;
    pub slave_demod:3: c_uint,
    pub rtl2830_platform_data: rtl2830_platform_data,
    pub rtl2832_platform_data: rtl2832_platform_data,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtl28xxu_chip_id {
    CHIP_ID_NONE,
    CHIP_ID_RTL2831U,
    CHIP_ID_RTL2832U,
}

// XXX: Hack. This must be keep sync with rtl2832 demod driver.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rtl28xxu_tuner {
    TUNER_NONE,

    TUNER_RTL2830_QT1010          = 0x10,
    TUNER_RTL2830_MT2060,
    TUNER_RTL2830_MXL5005S,

    TUNER_RTL2832_MT2266          = 0x20,
    TUNER_RTL2832_FC2580,
    TUNER_RTL2832_MT2063,
    TUNER_RTL2832_MAX3543,
    TUNER_RTL2832_TUA9001,
    TUNER_RTL2832_MXL5007T,
    TUNER_RTL2832_FC0012,
    TUNER_RTL2832_E4000,
    TUNER_RTL2832_TDA18272,
    TUNER_RTL2832_FC0013,
    TUNER_RTL2832_R820T,
    TUNER_RTL2832_R828D,
    TUNER_RTL2832_SI2157,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl28xxu_req {
    pub value: u16,
    pub index: u16,
    pub size: u16,
    pub data: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl28xxu_reg_val {
    pub reg: u16,
    pub val: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl28xxu_reg_val_mask {
    pub reg: u16,
    pub val: u8,
    pub mask: u8,
}

//
// memory map
//
// 0x0000 DEMOD : demodulator
// 0x2000 USB   : SIE, USB endpoint, debug, DMA
// 0x3000 SYS   : system
// 0xfc00 RC    : remote controller (not RTL2831U)
//
// USB registers
//
// SIE Control Registers
pub const USB_SYSCTL: c_uint = 0x2000 /* USB system control */;
pub const USB_SYSCTL_0: c_uint = 0x2000 /* USB system control */;
pub const USB_SYSCTL_1: c_uint = 0x2001 /* USB system control */;
pub const USB_SYSCTL_2: c_uint = 0x2002 /* USB system control */;
pub const USB_SYSCTL_3: c_uint = 0x2003 /* USB system control */;
pub const USB_IRQSTAT: c_uint = 0x2008 /* SIE interrupt status */;
pub const USB_IRQEN: c_uint = 0x200C /* SIE interrupt enable */;
pub const USB_CTRL: c_uint = 0x2010 /* USB control */;
pub const USB_STAT: c_uint = 0x2014 /* USB status */;
pub const USB_DEVADDR: c_uint = 0x2018 /* USB device address */;
pub const USB_TEST: c_uint = 0x201C /* USB test mode */;
pub const USB_FRAME_NUMBER: c_uint = 0x2020 /* frame number */;
pub const USB_FIFO_ADDR: c_uint = 0x2028 /* address of SIE FIFO RAM */;
pub const USB_FIFO_CMD: c_uint = 0x202A /* SIE FIFO RAM access command */;
pub const USB_FIFO_DATA: c_uint = 0x2030 /* SIE FIFO RAM data */;
// Endpoint Registers
pub const EP0_SETUPA: c_uint = 0x20F8 /* EP 0 setup packet lower byte */;
pub const EP0_SETUPB: c_uint = 0x20FC /* EP 0 setup packet higher byte */;
pub const USB_EP0_CFG: c_uint = 0x2104 /* EP 0 configure */;
pub const USB_EP0_CTL: c_uint = 0x2108 /* EP 0 control */;
pub const USB_EP0_STAT: c_uint = 0x210C /* EP 0 status */;
pub const USB_EP0_IRQSTAT: c_uint = 0x2110 /* EP 0 interrupt status */;
pub const USB_EP0_IRQEN: c_uint = 0x2114 /* EP 0 interrupt enable */;
pub const USB_EP0_MAXPKT: c_uint = 0x2118 /* EP 0 max packet size */;
pub const USB_EP0_BC: c_uint = 0x2120 /* EP 0 FIFO byte counter */;
pub const USB_EPA_CFG: c_uint = 0x2144 /* EP A configure */;
pub const USB_EPA_CFG_0: c_uint = 0x2144 /* EP A configure */;
pub const USB_EPA_CFG_1: c_uint = 0x2145 /* EP A configure */;
pub const USB_EPA_CFG_2: c_uint = 0x2146 /* EP A configure */;
pub const USB_EPA_CFG_3: c_uint = 0x2147 /* EP A configure */;
pub const USB_EPA_CTL: c_uint = 0x2148 /* EP A control */;
pub const USB_EPA_CTL_0: c_uint = 0x2148 /* EP A control */;
pub const USB_EPA_CTL_1: c_uint = 0x2149 /* EP A control */;
pub const USB_EPA_CTL_2: c_uint = 0x214A /* EP A control */;
pub const USB_EPA_CTL_3: c_uint = 0x214B /* EP A control */;
pub const USB_EPA_STAT: c_uint = 0x214C /* EP A status */;
pub const USB_EPA_IRQSTAT: c_uint = 0x2150 /* EP A interrupt status */;
pub const USB_EPA_IRQEN: c_uint = 0x2154 /* EP A interrupt enable */;
pub const USB_EPA_MAXPKT: c_uint = 0x2158 /* EP A max packet size */;
pub const USB_EPA_MAXPKT_0: c_uint = 0x2158 /* EP A max packet size */;
pub const USB_EPA_MAXPKT_1: c_uint = 0x2159 /* EP A max packet size */;
pub const USB_EPA_MAXPKT_2: c_uint = 0x215A /* EP A max packet size */;
pub const USB_EPA_MAXPKT_3: c_uint = 0x215B /* EP A max packet size */;
pub const USB_EPA_FIFO_CFG: c_uint = 0x2160 /* EP A FIFO configure */;
pub const USB_EPA_FIFO_CFG_0: c_uint = 0x2160 /* EP A FIFO configure */;
pub const USB_EPA_FIFO_CFG_1: c_uint = 0x2161 /* EP A FIFO configure */;
pub const USB_EPA_FIFO_CFG_2: c_uint = 0x2162 /* EP A FIFO configure */;
pub const USB_EPA_FIFO_CFG_3: c_uint = 0x2163 /* EP A FIFO configure */;
// Debug Registers
pub const USB_PHYTSTDIS: c_uint = 0x2F04 /* PHY test disable */;
pub const USB_TOUT_VAL: c_uint = 0x2F08 /* USB time-out time */;
pub const USB_VDRCTRL: c_uint = 0x2F10 /* UTMI vendor signal control */;
pub const USB_VSTAIN: c_uint = 0x2F14 /* UTMI vendor signal status in */;
pub const USB_VLOADM: c_uint = 0x2F18 /* UTMI load vendor signal status in */;
pub const USB_VSTAOUT: c_uint = 0x2F1C /* UTMI vendor signal status out */;
pub const USB_UTMI_TST: c_uint = 0x2F80 /* UTMI test */;
pub const USB_UTMI_STATUS: c_uint = 0x2F84 /* UTMI status */;
pub const USB_TSTCTL: c_uint = 0x2F88 /* test control */;
pub const USB_TSTCTL2: c_uint = 0x2F8C /* test control 2 */;
pub const USB_PID_FORCE: c_uint = 0x2F90 /* force PID */;
pub const USB_PKTERR_CNT: c_uint = 0x2F94 /* packet error counter */;
pub const USB_RXERR_CNT: c_uint = 0x2F98 /* RX error counter */;
pub const USB_MEM_BIST: c_uint = 0x2F9C /* MEM BIST test */;
pub const USB_SLBBIST: c_uint = 0x2FA0 /* self-loop-back BIST */;
pub const USB_CNTTEST: c_uint = 0x2FA4 /* counter test */;
pub const USB_PHYTST: c_uint = 0x2FC0 /* USB PHY test */;
pub const USB_DBGIDX: c_uint = 0x2FF0 /* select individual block debug signal */;
pub const USB_DBGMUX: c_uint = 0x2FF4 /* debug signal module mux */;
//
// SYS registers
//
// demod control registers
pub const SYS_SYS0: c_uint = 0x3000 /* include DEMOD_CTL, GPO, GPI, GPOE */;
pub const SYS_DEMOD_CTL: c_uint = 0x3000 /* control register for DVB-T demodulator */;
// GPIO registers
pub const SYS_GPIO_OUT_VAL: c_uint = 0x3001 /* output value of GPIO */;
pub const SYS_GPIO_IN_VAL: c_uint = 0x3002 /* input value of GPIO */;
pub const SYS_GPIO_OUT_EN: c_uint = 0x3003 /* output enable of GPIO */;
pub const SYS_SYS1: c_uint = 0x3004 /* include GPD, SYSINTE, SYSINTS, GP_CFG0 */;
pub const SYS_GPIO_DIR: c_uint = 0x3004 /* direction control for GPIO */;
pub const SYS_SYSINTE: c_uint = 0x3005 /* system interrupt enable */;
pub const SYS_SYSINTS: c_uint = 0x3006 /* system interrupt status */;
pub const SYS_GPIO_CFG0: c_uint = 0x3007 /* PAD configuration for GPIO0-GPIO3 */;
pub const SYS_SYS2: c_uint = 0x3008 /* include GP_CFG1 and 3 reserved bytes */;
pub const SYS_GPIO_CFG1: c_uint = 0x3008 /* PAD configuration for GPIO4 */;
pub const SYS_DEMOD_CTL1: c_uint = 0x300B;
// IrDA registers
pub const SYS_IRRC_PSR: c_uint = 0x3020 /* IR protocol selection */;
pub const SYS_IRRC_PER: c_uint = 0x3024 /* IR protocol extension */;
pub const SYS_IRRC_SF: c_uint = 0x3028 /* IR sampling frequency */;
pub const SYS_IRRC_DPIR: c_uint = 0x302C /* IR data package interval */;
pub const SYS_IRRC_CR: c_uint = 0x3030 /* IR control */;
pub const SYS_IRRC_RP: c_uint = 0x3034 /* IR read port */;
pub const SYS_IRRC_SR: c_uint = 0x3038 /* IR status */;
// I2C master registers
pub const SYS_I2CCR: c_uint = 0x3040 /* I2C clock */;
pub const SYS_I2CMCR: c_uint = 0x3044 /* I2C master control */;
pub const SYS_I2CMSTR: c_uint = 0x3048 /* I2C master SCL timing */;
pub const SYS_I2CMSR: c_uint = 0x304C /* I2C master status */;
pub const SYS_I2CMFR: c_uint = 0x3050 /* I2C master FIFO */;
//
// IR registers
//
pub const IR_RX_BUF: c_uint = 0xFC00;
pub const IR_RX_IE: c_uint = 0xFD00;
pub const IR_RX_IF: c_uint = 0xFD01;
pub const IR_RX_CTRL: c_uint = 0xFD02;
pub const IR_RX_CFG: c_uint = 0xFD03;
pub const IR_MAX_DURATION0: c_uint = 0xFD04;
pub const IR_MAX_DURATION1: c_uint = 0xFD05;
pub const IR_IDLE_LEN0: c_uint = 0xFD06;
pub const IR_IDLE_LEN1: c_uint = 0xFD07;
pub const IR_GLITCH_LEN: c_uint = 0xFD08;
pub const IR_RX_BUF_CTRL: c_uint = 0xFD09;
pub const IR_RX_BUF_DATA: c_uint = 0xFD0A;
pub const IR_RX_BC: c_uint = 0xFD0B;
pub const IR_RX_CLK: c_uint = 0xFD0C;
pub const IR_RX_C_COUNT_L: c_uint = 0xFD0D;
pub const IR_RX_C_COUNT_H: c_uint = 0xFD0E;
pub const IR_SUSPEND_CTRL: c_uint = 0xFD10;
pub const IR_ERR_TOL_CTRL: c_uint = 0xFD11;
pub const IR_UNIT_LEN: c_uint = 0xFD12;
pub const IR_ERR_TOL_LEN: c_uint = 0xFD13;
pub const IR_MAX_H_TOL_LEN: c_uint = 0xFD14;
pub const IR_MAX_L_TOL_LEN: c_uint = 0xFD15;
pub const IR_MASK_CTRL: c_uint = 0xFD16;
pub const IR_MASK_DATA: c_uint = 0xFD17;
pub const IR_RES_MASK_ADDR: c_uint = 0xFD18;
pub const IR_RES_MASK_T_LEN: c_uint = 0xFD19;
