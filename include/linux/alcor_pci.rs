//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/alcor_pci.h
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
// Copyright (C) 2018 Oleksij Rempel <linux@rempel-privat.de>
//
// Driver for Alcor Micro AU6601 and AU6621 controllers
//
pub const ALCOR_SD_CARD: c_int = 0;
pub const ALCOR_MS_CARD: c_int = 1;

pub const PCI_ID_ALCOR_MICRO: c_uint = 0x1AEA;
pub const PCI_ID_AU6601: c_uint = 0x6601;
pub const PCI_ID_AU6621: c_uint = 0x6621;
pub const PCI_ID_AU6625: c_uint = 0x6625;

pub const AU6601_BASE_CLOCK: c_int = 31000000;
pub const AU6601_MIN_CLOCK: c_int = 150000;
pub const AU6601_MAX_CLOCK: c_int = 208000000;
pub const AU6601_MAX_DMA_SEGMENTS: c_int = 64;
pub const AU6601_MAX_PIO_SEGMENTS: c_int = 1;
pub const AU6601_MAX_DMA_BLOCK_SIZE: c_uint = 0x1000;
pub const AU6601_MAX_PIO_BLOCK_SIZE: c_uint = 0x200;
pub const AU6601_MAX_DMA_BLOCKS: c_int = 1;
pub const AU6601_DMA_LOCAL_SEGMENTS: c_int = 1;
// registers spotter by reverse engineering but still
// with unknown functionality:
// 0x10 - ADMA phy address. AU6621 only?
// 0x51 - LED ctrl?
// 0x52 - unknown
// 0x61 - LED related? Always toggled BIT0
// 0x63 - Same as 0x61?
// 0x77 - unknown
//
// SDMA phy address. Higher then 0x0800.0000?
// The au6601 and au6621 have different DMA engines with different issues. One
// For example au6621 engine is triggered by addr change. No other interaction
// is needed. This means, if we get two buffers with same address, then engine
// will stall.
//
pub const AU6601_REG_SDMA_ADDR: c_uint = 0x00;
pub const AU6601_SDMA_MASK: c_uint = 0xffffffff;
pub const AU6601_DMA_BOUNDARY: c_uint = 0x05;
pub const AU6621_DMA_PAGE_CNT: c_uint = 0x05;
// PIO
pub const AU6601_REG_BUFFER: c_uint = 0x08;
// ADMA ctrl? AU6621 only.
pub const AU6621_DMA_CTRL: c_uint = 0x0c;

// CMD index
pub const AU6601_REG_CMD_OPCODE: c_uint = 0x23;
// CMD parametr
pub const AU6601_REG_CMD_ARG: c_uint = 0x24;
// CMD response 4x4 Bytes
pub const AU6601_REG_CMD_RSP0: c_uint = 0x30;
pub const AU6601_REG_CMD_RSP1: c_uint = 0x34;
pub const AU6601_REG_CMD_RSP2: c_uint = 0x38;
pub const AU6601_REG_CMD_RSP3: c_uint = 0x3C;
// default timeout set to 125: 125 * 40ms = 5 sec
// how exactly it is calculated?
//
pub const AU6601_TIME_OUT_CTRL: c_uint = 0x69;
// Block size for SDMA or PIO
pub const AU6601_REG_BLOCK_SIZE: c_uint = 0x6c;
// Some power related reg, used together with AU6601_OUTPUT_ENABLE
pub const AU6601_POWER_CONTROL: c_uint = 0x70;
// PLL ctrl
pub const AU6601_CLK_SELECT: c_uint = 0x72;
pub const AU6601_CLK_OVER_CLK: c_uint = 0x80;
pub const AU6601_CLK_384_MHZ: c_uint = 0x30;
pub const AU6601_CLK_125_MHZ: c_uint = 0x20;
pub const AU6601_CLK_48_MHZ: c_uint = 0x10;
pub const AU6601_CLK_EXT_PLL: c_uint = 0x04;
pub const AU6601_CLK_X2_MODE: c_uint = 0x02;
pub const AU6601_CLK_ENABLE: c_uint = 0x01;
pub const AU6601_CLK_31_25_MHZ: c_uint = 0x00;
pub const AU6601_CLK_DIVIDER: c_uint = 0x73;
pub const AU6601_INTERFACE_MODE_CTRL: c_uint = 0x74;
pub const AU6601_DLINK_MODE: c_uint = 0x80;
pub const AU6601_INTERRUPT_DELAY_TIME: c_uint = 0x40;
pub const AU6601_SIGNAL_REQ_CTRL: c_uint = 0x30;

// same register values are used for:
// - AU6601_OUTPUT_ENABLE
// - AU6601_POWER_CONTROL
//
pub const AU6601_ACTIVE_CTRL: c_uint = 0x75;

// AU6601_MS_CARD_ACTIVE - will cativate MS card section?

// card slot state. It should automatically detect type of
// the card
//
pub const AU6601_DETECT_STATUS: c_uint = 0x76;

pub const AU6601_DETECT_STATUS_M: c_uint = 0xf;
pub const AU6601_REG_SW_RESET: c_uint = 0x79;

pub const AU6601_OUTPUT_ENABLE: c_uint = 0x7a;
pub const AU6601_PAD_DRIVE0: c_uint = 0x7b;
pub const AU6601_PAD_DRIVE1: c_uint = 0x7c;
pub const AU6601_PAD_DRIVE2: c_uint = 0x7d;
// read EEPROM?
pub const AU6601_FUNCTION: c_uint = 0x7f;
pub const AU6601_CMD_XFER_CTRL: c_uint = 0x81;
pub const AU6601_CMD_17_BYTE_CRC: c_uint = 0xc0;
pub const AU6601_CMD_6_BYTE_WO_CRC: c_uint = 0x80;
pub const AU6601_CMD_6_BYTE_CRC: c_uint = 0x40;
pub const AU6601_CMD_START_XFER: c_uint = 0x20;
pub const AU6601_CMD_STOP_WAIT_RDY: c_uint = 0x10;
pub const AU6601_CMD_NO_RESP: c_uint = 0x00;
pub const AU6601_REG_BUS_CTRL: c_uint = 0x82;
pub const AU6601_BUS_WIDTH_4BIT: c_uint = 0x20;
pub const AU6601_BUS_WIDTH_8BIT: c_uint = 0x10;
pub const AU6601_BUS_WIDTH_1BIT: c_uint = 0x00;
pub const AU6601_DATA_XFER_CTRL: c_uint = 0x83;

pub const AU6601_DATA_PIN_STATE: c_uint = 0x84;

// BIT(4) - BIT(7) are permanently 1.
// May be reserved or not attached DAT4-DAT7
//

pub const AU6601_BUS_STAT_DAT_MASK: c_uint = 0xf;
pub const AU6601_OPT: c_uint = 0x85;
pub const AU6601_OPT_CMD_LINE_LEVEL: c_uint = 0x80;

pub const AU6601_CLK_DELAY: c_uint = 0x86;
pub const AU6601_CLK_DATA_POSITIVE_EDGE: c_uint = 0x80;
pub const AU6601_CLK_CMD_POSITIVE_EDGE: c_uint = 0x40;

pub const AU6601_REG_INT_STATUS: c_uint = 0x90;
pub const AU6601_REG_INT_ENABLE: c_uint = 0x94;

pub const AU6601_INT_NORMAL_MASK: c_uint = 0x00007FFF;
pub const AU6601_INT_ERROR_MASK: c_uint = 0xFFFF8000;

// MS_CARD mode registers
pub const AU6601_MS_STATUS: c_uint = 0xa0;
pub const AU6601_MS_BUS_MODE_CTRL: c_uint = 0xa1;
pub const AU6601_MS_BUS_8BIT_MODE: c_uint = 0x03;
pub const AU6601_MS_BUS_4BIT_MODE: c_uint = 0x01;
pub const AU6601_MS_BUS_1BIT_MODE: c_uint = 0x00;
pub const AU6601_MS_TPC_CMD: c_uint = 0xa2;
pub const AU6601_MS_TPC_READ_PAGE_DATA: c_uint = 0x02;
pub const AU6601_MS_TPC_READ_REG: c_uint = 0x04;
pub const AU6601_MS_TPC_GET_INT: c_uint = 0x07;
pub const AU6601_MS_TPC_WRITE_PAGE_DATA: c_uint = 0x0D;
pub const AU6601_MS_TPC_WRITE_REG: c_uint = 0x0B;
pub const AU6601_MS_TPC_SET_RW_REG_ADRS: c_uint = 0x08;
pub const AU6601_MS_TPC_SET_CMD: c_uint = 0x0E;
pub const AU6601_MS_TPC_EX_SET_CMD: c_uint = 0x09;
pub const AU6601_MS_TPC_READ_SHORT_DATA: c_uint = 0x03;
pub const AU6601_MS_TPC_WRITE_SHORT_DATA: c_uint = 0x0C;
pub const AU6601_MS_TRANSFER_MODE: c_uint = 0xa3;

pub const AU6601_MS_DATA_PIN_STATE: c_uint = 0xa4;
pub const AU6601_MS_INT_STATUS: c_uint = 0xb0;
pub const AU6601_MS_INT_ENABLE: c_uint = 0xb4;

pub const AU6601_MS_INT_DATA_MASK: c_uint = 0x00000038;
pub const AU6601_MS_INT_TPC_MASK: c_uint = 0x003d8002;
pub const AU6601_MS_INT_TPC_ERROR: c_uint = 0x003d0000;
pub const ALCOR_PCIE_LINK_CTRL_OFFSET: c_uint = 0x10;
pub const ALCOR_PCIE_LINK_CAP_OFFSET: c_uint = 0x0c;
pub const ALCOR_CAP_START_OFFSET: c_uint = 0x34;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct alcor_dev_cfg {
    pub dma: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct alcor_pci_priv {
    pub pdev: *mut pci_dev,
    pub parent_pdev: *mut pci_dev,
    pub dev: *mut device,
    pub iobase: *mut void __iomem,
    pub irq: c_uint,
    pub /: *mut *mut unsigned long id; / idr id,
    pub cfg: *mut alcor_dev_cfg,
}

extern "C" {
    pub fn alcor_write8(priv: *mut alcor_pci_priv, val: u8, addr: c_uint);
}
extern "C" {
    pub fn alcor_write16(priv: *mut alcor_pci_priv, val: u16, addr: c_uint);
}
extern "C" {
    pub fn alcor_write32(priv: *mut alcor_pci_priv, val: u32, addr: c_uint);
}
extern "C" {
    pub fn alcor_write32be(priv: *mut alcor_pci_priv, val: u32, addr: c_uint);
}
extern "C" {
    pub fn alcor_read8(priv: *mut alcor_pci_priv, addr: c_uint) -> u8;
}
extern "C" {
    pub fn alcor_read32(priv: *mut alcor_pci_priv, addr: c_uint) -> u32;
}
extern "C" {
    pub fn alcor_read32be(priv: *mut alcor_pci_priv, addr: c_uint) -> u32;
}
