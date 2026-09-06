//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/rc/fintek-cir.h
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
// Driver for Feature Integration Technology Inc. (aka Fintek) LPC CIR
//
// Copyright (C) 2011 Jarod Wilson <jarod@redhat.com>
//
// Special thanks to Fintek for providing hardware and spec sheets.
// This driver is based upon the nuvoton, ite and ene drivers for
// similar hardware.
//

// platform driver name to register

pub const VENDOR_ID_FINTEK: c_uint = 0x1934;
// debugging module parameter

pub const TX_BUF_LEN: c_int = 256;
pub const RX_BUF_LEN: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fintek_dev {
    pub pdev: *mut pnp_dev,
    pub rdev: *mut rc_dev,
    pub fintek_lock: spinlock_t,
// for rx
    pub buf: [u8; RX_BUF_LEN],
    pub pkts: c_uint,
    pub lock: spinlock_t,
    pub buf: [u8; TX_BUF_LEN],
    pub buf_count: c_uint,
    pub cur_buf_num: c_uint,
    pub queue: wait_queue_head_t,
    pub tx: },
// Config register index/data port pair
    pub cr_ip: u32,
    pub cr_dp: u32,
// hardware I/O settings
    pub cir_addr: c_ulong,
    pub cir_irq: c_int,
    pub cir_port_len: c_int,
// hardware id
    pub chip_major: u8,
    pub chip_minor: u8,
    pub chip_vendor: u16,
    pub logical_dev_cir: u8,
// hardware features
    pub hw_learning_capable: bool,
    pub hw_tx_capable: bool,
// rx settings
    pub learning_enabled: bool,
    pub carrier_detect_enabled: bool,
    pub parser_state: },
    pub rem: u8 cmd,,
// carrier period = 1 / frequency
    pub carrier: u32,
}

// buffer packet constants, largely identical to mceusb.c
pub const BUF_PULSE_BIT: c_uint = 0x80;
pub const BUF_LEN_MASK: c_uint = 0x1f;
pub const BUF_SAMPLE_MASK: c_uint = 0x7f;
pub const BUF_COMMAND_HEADER: c_uint = 0x9f;
pub const BUF_COMMAND_MASK: c_uint = 0xe0;
pub const BUF_COMMAND_NULL: c_uint = 0x00;
pub const BUF_HW_CMD_HEADER: c_uint = 0xff;
pub const BUF_CMD_G_REVISION: c_uint = 0x0b;
pub const BUF_CMD_S_CARRIER: c_uint = 0x06;
pub const BUF_CMD_S_TIMEOUT: c_uint = 0x0c;
pub const BUF_CMD_SIG_END: c_uint = 0x01;
pub const BUF_CMD_S_TXMASK: c_uint = 0x08;
pub const BUF_CMD_S_RXSENSOR: c_uint = 0x14;
pub const BUF_RSP_PULSE_COUNT: c_uint = 0x15;
pub const CIR_SAMPLE_PERIOD: c_int = 50;
//
// Configuration Register:
// Index Port
// Data Port
//
pub const CR_INDEX_PORT: c_uint = 0x2e;
pub const CR_DATA_PORT: c_uint = 0x2f;
// Possible alternate values, depends on how the chip is wired
pub const CR_INDEX_PORT2: c_uint = 0x4e;
pub const CR_DATA_PORT2: c_uint = 0x4f;
//
// GCR_CONFIG_PORT_SEL bit 4 specifies which Index Port value is
// active. 1 = 0x4e, 0 = 0x2e
//
pub const PORT_SEL_PORT_4E_EN: c_uint = 0x10;
// Extended Function Mode enable/disable magic values
pub const CONFIG_REG_ENABLE: c_uint = 0x87;
pub const CONFIG_REG_DISABLE: c_uint = 0xaa;
// Chip IDs found in CR_CHIP_ID_{HI,LO}
pub const CHIP_ID_HIGH_F71809U: c_uint = 0x04;
pub const CHIP_ID_LOW_F71809U: c_uint = 0x08;
//
// Global control regs we need to care about:
// Global Control                  def.
// Register name           addr    val.
pub const GCR_SOFTWARE_RESET: c_uint = 0x02 /* 0x00 */;
pub const GCR_LOGICAL_DEV_NO: c_uint = 0x07 /* 0x00 */;
pub const GCR_CHIP_ID_HI: c_uint = 0x20 /* 0x04 */;
pub const GCR_CHIP_ID_LO: c_uint = 0x21 /* 0x08 */;
pub const GCR_VENDOR_ID_HI: c_uint = 0x23 /* 0x19 */;
pub const GCR_VENDOR_ID_LO: c_uint = 0x24 /* 0x34 */;
pub const GCR_CONFIG_PORT_SEL: c_uint = 0x25 /* 0x01 */;
pub const GCR_KBMOUSE_WAKEUP: c_uint = 0x27;
pub const LOGICAL_DEV_DISABLE: c_uint = 0x00;
pub const LOGICAL_DEV_ENABLE: c_uint = 0x01;
// Logical device number of the CIR function
pub const LOGICAL_DEV_CIR_REV1: c_uint = 0x05;
pub const LOGICAL_DEV_CIR_REV2: c_uint = 0x08;
// CIR Logical Device (LDN 0x08) config registers
pub const CIR_CR_COMMAND_INDEX: c_uint = 0x04;
pub const CIR_CR_IRCS: c_uint = 0x05 /* Before host writes command to IR, host;
pub const CIR_CR_COMMAND_DATA: c_uint = 0x06 /* Host read or write command data */;
pub const CIR_CR_CLASS: c_uint = 0x07 /* 0xff = rx-only, 0x66 = rx + 2 tx,;
pub const CIR_CR_DEV_EN: c_uint = 0x30 /* bit0 = 1 enables CIR */;
pub const CIR_CR_BASE_ADDR_HI: c_uint = 0x60 /* MSB of CIR IO base addr */;
pub const CIR_CR_BASE_ADDR_LO: c_uint = 0x61 /* LSB of CIR IO base addr */;
pub const CIR_CR_IRQ_SEL: c_uint = 0x70 /* bits3-0 store CIR IRQ */;
pub const CIR_CR_PSOUT_STATUS: c_uint = 0xf1;
pub const CIR_CR_WAKE_KEY3_ADDR: c_uint = 0xf8;
pub const CIR_CR_WAKE_KEY3_CODE: c_uint = 0xf9;
pub const CIR_CR_WAKE_KEY3_DC: c_uint = 0xfa;
pub const CIR_CR_WAKE_CONTROL: c_uint = 0xfb;
pub const CIR_CR_WAKE_KEY12_ADDR: c_uint = 0xfc;
pub const CIR_CR_WAKE_KEY4_ADDR: c_uint = 0xfd;
pub const CIR_CR_WAKE_KEY5_ADDR: c_uint = 0xfe;
pub const CLASS_RX_ONLY: c_uint = 0xff;
pub const CLASS_RX_2TX: c_uint = 0x66;
pub const CLASS_RX_1TX: c_uint = 0x33;
// CIR device registers
pub const CIR_STATUS: c_uint = 0x00;
pub const CIR_RX_DATA: c_uint = 0x01;
pub const CIR_TX_CONTROL: c_uint = 0x02;
pub const CIR_TX_DATA: c_uint = 0x03;
pub const CIR_CONTROL: c_uint = 0x04;
// Bits to enable CIR wake
pub const LOGICAL_DEV_ACPI: c_uint = 0x01;
pub const LDEV_ACPI_WAKE_EN_REG: c_uint = 0xe8;
pub const ACPI_WAKE_EN_CIR_BIT: c_uint = 0x04;
pub const LDEV_ACPI_PME_EN_REG: c_uint = 0xf0;
pub const LDEV_ACPI_PME_CLR_REG: c_uint = 0xf1;
pub const ACPI_PME_CIR_BIT: c_uint = 0x02;
pub const LDEV_ACPI_STATE_REG: c_uint = 0xf4;
pub const ACPI_STATE_CIR_BIT: c_uint = 0x20;
//
// CIR status register (0x00):
// 7 - CIR_IRQ_EN (1 = enable CIR IRQ, 0 = disable)
// 3 - TX_FINISH (1 when TX finished, write 1 to clear)
// 2 - TX_UNDERRUN (1 on TX underrun, write 1 to clear)
// 1 - RX_TIMEOUT (1 on RX timeout, write 1 to clear)
// 0 - RX_RECEIVE (1 on RX receive, write 1 to clear)
//
pub const CIR_STATUS_IRQ_EN: c_uint = 0x80;
pub const CIR_STATUS_TX_FINISH: c_uint = 0x08;
pub const CIR_STATUS_TX_UNDERRUN: c_uint = 0x04;
pub const CIR_STATUS_RX_TIMEOUT: c_uint = 0x02;
pub const CIR_STATUS_RX_RECEIVE: c_uint = 0x01;
pub const CIR_STATUS_IRQ_MASK: c_uint = 0x0f;
//
// CIR TX control register (0x02):
// 7 - TX_START (1 to indicate TX start, auto-cleared when done)
// 6 - TX_END (1 to indicate TX data written to TX fifo)
//
pub const CIR_TX_CONTROL_TX_START: c_uint = 0x80;
pub const CIR_TX_CONTROL_TX_END: c_uint = 0x40;
