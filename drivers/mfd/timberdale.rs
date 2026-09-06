//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mfd/timberdale.h
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
//
// timberdale.h timberdale FPGA MFD driver defines
// Copyright (c) 2009 Intel Corporation
//
// Supports:
// Timberdale FPGA
//

// This driver only support versions >= 3.8 and < 4.0
pub const TIMB_SUPPORTED_MAJOR: c_int = 3;
// This driver only support minor >= 8
pub const TIMB_REQUIRED_MINOR: c_int = 8;
// Registers of the control area
pub const TIMB_REV_MAJOR: c_uint = 0x00;
pub const TIMB_REV_MINOR: c_uint = 0x04;
pub const TIMB_HW_CONFIG: c_uint = 0x08;
pub const TIMB_SW_RST: c_uint = 0x40;
// bits in the TIMB_HW_CONFIG register
pub const TIMB_HW_CONFIG_SPI_8BIT: c_uint = 0x80;
pub const TIMB_HW_VER_MASK: c_uint = 0x0f;
pub const TIMB_HW_VER0: c_uint = 0x00;
pub const TIMB_HW_VER1: c_uint = 0x01;
pub const TIMB_HW_VER2: c_uint = 0x02;
pub const TIMB_HW_VER3: c_uint = 0x03;
pub const OCORESOFFSET: c_uint = 0x0;
pub const OCORESEND: c_uint = 0x1f;
pub const SPIOFFSET: c_uint = 0x80;
pub const SPIEND: c_uint = 0xff;
pub const UARTLITEOFFSET: c_uint = 0x100;
pub const UARTLITEEND: c_uint = 0x10f;
pub const RDSOFFSET: c_uint = 0x180;
pub const RDSEND: c_uint = 0x183;
pub const ETHOFFSET: c_uint = 0x300;
pub const ETHEND: c_uint = 0x3ff;
pub const GPIOOFFSET: c_uint = 0x400;
pub const GPIOEND: c_uint = 0x7ff;
pub const CHIPCTLOFFSET: c_uint = 0x800;
pub const CHIPCTLEND: c_uint = 0x8ff;

pub const INTCOFFSET: c_uint = 0xc00;
pub const INTCEND: c_uint = 0xfff;

pub const MOSTOFFSET: c_uint = 0x1000;
pub const MOSTEND: c_uint = 0x13ff;
pub const UARTOFFSET: c_uint = 0x1400;
pub const UARTEND: c_uint = 0x17ff;
pub const XIICOFFSET: c_uint = 0x1800;
pub const XIICEND: c_uint = 0x19ff;
pub const I2SOFFSET: c_uint = 0x1C00;
pub const I2SEND: c_uint = 0x1fff;
pub const LOGIWOFFSET: c_uint = 0x30000;
pub const LOGIWEND: c_uint = 0x37fff;
pub const MLCOREOFFSET: c_uint = 0x40000;
pub const MLCOREEND: c_uint = 0x43fff;
pub const DMAOFFSET: c_uint = 0x01000000;
pub const DMAEND: c_uint = 0x013fffff;
// SDHC0 is placed in PCI bar 1
pub const SDHC0OFFSET: c_uint = 0x00;
pub const SDHC0END: c_uint = 0xff;
// SDHC1 is placed in PCI bar 2
pub const SDHC1OFFSET: c_uint = 0x00;
pub const SDHC1END: c_uint = 0xff;
pub const PCI_VENDOR_ID_TIMB: c_uint = 0x10ee;
pub const PCI_DEVICE_ID_TIMB: c_uint = 0xa123;
pub const IRQ_TIMBERDALE_INIC: c_int = 0;
pub const IRQ_TIMBERDALE_MLB: c_int = 1;
pub const IRQ_TIMBERDALE_GPIO: c_int = 2;
pub const IRQ_TIMBERDALE_I2C: c_int = 3;
pub const IRQ_TIMBERDALE_UART: c_int = 4;
pub const IRQ_TIMBERDALE_DMA: c_int = 5;
pub const IRQ_TIMBERDALE_I2S: c_int = 6;
pub const IRQ_TIMBERDALE_TSC_INT: c_int = 7;
pub const IRQ_TIMBERDALE_SDHC: c_int = 8;
pub const IRQ_TIMBERDALE_ADV7180: c_int = 9;
pub const IRQ_TIMBERDALE_ETHSW_IF: c_int = 10;
pub const IRQ_TIMBERDALE_SPI: c_int = 11;
pub const IRQ_TIMBERDALE_UARTLITE: c_int = 12;
pub const IRQ_TIMBERDALE_MLCORE: c_int = 13;
pub const IRQ_TIMBERDALE_MLCORE_BUF: c_int = 14;
pub const IRQ_TIMBERDALE_RDS: c_int = 15;
pub const TIMBERDALE_NR_IRQS: c_int = 16;
pub const GPIO_PIN_ASCB: c_int = 8;
pub const GPIO_PIN_INIC_RST: c_int = 14;
pub const GPIO_PIN_BT_RST: c_int = 15;
// DMA Channels
pub const DMA_UART_RX: c_int = 0;
pub const DMA_UART_TX: c_int = 1;
pub const DMA_MLB_RX: c_int = 2;
pub const DMA_MLB_TX: c_int = 3;
pub const DMA_VIDEO_RX: c_int = 4;
pub const DMA_VIDEO_DROP: c_int = 5;
pub const DMA_SDHCI_RX: c_int = 6;
pub const DMA_SDHCI_TX: c_int = 7;
pub const DMA_ETH_RX: c_int = 8;
pub const DMA_ETH_TX: c_int = 9;
