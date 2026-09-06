//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/rc/nuvoton-cir.h
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
// Driver for Nuvoton Technology Corporation w83667hg/w83677hg-i CIR
//
// Copyright (C) 2010 Jarod Wilson <jarod@redhat.com>
// Copyright (C) 2009 Nuvoton PS Team
//
// Special thanks to Nuvoton for providing hardware, spec sheets and
// sample code upon which portions of this driver are based. Indirect
// thanks also to Maxim Levitsky, whose ene_ir driver this driver is
// modeled after.
//

// platform driver name to register

// debugging module parameter

pub const RX_BUF_LEN: c_int = 32;
pub const SIO_ID_MASK: c_uint = 0xfff0;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvt_chip_ver {
    NVT_UNKNOWN	= 0,
    NVT_W83667HG	= 0xa510,
    NVT_6775F	= 0xb470,
    NVT_6776F	= 0xc330,
    NVT_6779D	= 0xc560,
    NVT_INVALID	= 0xffff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvt_chip {
    pub name: *const c_char,
    pub chip_ver: nvt_chip_ver,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvt_dev {
    pub rdev: *mut rc_dev,
    pub lock: spinlock_t,
// for rx
    pub buf: [u8; RX_BUF_LEN],
    pub pkts: c_uint,
// EFER Config register index/data pair
    pub cr_efir: u32,
    pub cr_efdr: u32,
// hardware I/O settings
    pub cir_addr: c_ulong,
    pub cir_wake_addr: c_ulong,
    pub cir_irq: c_int,
    pub chip_ver: nvt_chip_ver,
// hardware id
    pub chip_major: u8,
    pub chip_minor: u8,
// carrier period = 1 / frequency
    pub carrier: u32,
}

// buffer packet constants
pub const BUF_PULSE_BIT: c_uint = 0x80;
pub const BUF_LEN_MASK: c_uint = 0x7f;
pub const BUF_REPEAT_BYTE: c_uint = 0x70;
pub const BUF_REPEAT_MASK: c_uint = 0xf0;
// CIR settings
// total length of CIR and CIR WAKE
pub const CIR_IOREG_LENGTH: c_uint = 0x0f;
// RX limit length, 8 high bits for SLCH, 8 low bits for SLCL

// CIR Regs
pub const CIR_IRCON: c_uint = 0x00;
pub const CIR_IRSTS: c_uint = 0x01;
pub const CIR_IREN: c_uint = 0x02;
pub const CIR_RXFCONT: c_uint = 0x03;
pub const CIR_CP: c_uint = 0x04;
pub const CIR_CC: c_uint = 0x05;
pub const CIR_SLCH: c_uint = 0x06;
pub const CIR_SLCL: c_uint = 0x07;
pub const CIR_FIFOCON: c_uint = 0x08;
pub const CIR_IRFIFOSTS: c_uint = 0x09;
pub const CIR_SRXFIFO: c_uint = 0x0a;
pub const CIR_TXFCONT: c_uint = 0x0b;
pub const CIR_STXFIFO: c_uint = 0x0c;
pub const CIR_FCCH: c_uint = 0x0d;
pub const CIR_FCCL: c_uint = 0x0e;
pub const CIR_IRFSM: c_uint = 0x0f;
// CIR IRCON settings
pub const CIR_IRCON_RECV: c_uint = 0x80;
pub const CIR_IRCON_WIREN: c_uint = 0x40;
pub const CIR_IRCON_TXEN: c_uint = 0x20;
pub const CIR_IRCON_RXEN: c_uint = 0x10;
pub const CIR_IRCON_WRXINV: c_uint = 0x08;
pub const CIR_IRCON_RXINV: c_uint = 0x04;
pub const CIR_IRCON_SAMPLE_PERIOD_SEL_1: c_uint = 0x00;
pub const CIR_IRCON_SAMPLE_PERIOD_SEL_25: c_uint = 0x01;
pub const CIR_IRCON_SAMPLE_PERIOD_SEL_50: c_uint = 0x02;
pub const CIR_IRCON_SAMPLE_PERIOD_SEL_100: c_uint = 0x03;
// FIXME: make this a runtime option
// select sample period as 50us

// CIR IRSTS settings
pub const CIR_IRSTS_RDR: c_uint = 0x80;
pub const CIR_IRSTS_RTR: c_uint = 0x40;
pub const CIR_IRSTS_PE: c_uint = 0x20;
pub const CIR_IRSTS_RFO: c_uint = 0x10;
pub const CIR_IRSTS_TE: c_uint = 0x08;
pub const CIR_IRSTS_TTR: c_uint = 0x04;
pub const CIR_IRSTS_TFU: c_uint = 0x02;
pub const CIR_IRSTS_GH: c_uint = 0x01;
// CIR IREN settings
pub const CIR_IREN_RDR: c_uint = 0x80;
pub const CIR_IREN_RTR: c_uint = 0x40;
pub const CIR_IREN_PE: c_uint = 0x20;
pub const CIR_IREN_RFO: c_uint = 0x10;
pub const CIR_IREN_TE: c_uint = 0x08;
pub const CIR_IREN_TTR: c_uint = 0x04;
pub const CIR_IREN_TFU: c_uint = 0x02;
pub const CIR_IREN_GH: c_uint = 0x01;
// CIR FIFOCON settings
pub const CIR_FIFOCON_TXFIFOCLR: c_uint = 0x80;
pub const CIR_FIFOCON_TX_TRIGGER_LEV_31: c_uint = 0x00;
pub const CIR_FIFOCON_TX_TRIGGER_LEV_24: c_uint = 0x10;
pub const CIR_FIFOCON_TX_TRIGGER_LEV_16: c_uint = 0x20;
pub const CIR_FIFOCON_TX_TRIGGER_LEV_8: c_uint = 0x30;
// FIXME: make this a runtime option
// select TX trigger level as 16

pub const CIR_FIFOCON_RXFIFOCLR: c_uint = 0x08;
pub const CIR_FIFOCON_RX_TRIGGER_LEV_1: c_uint = 0x00;
pub const CIR_FIFOCON_RX_TRIGGER_LEV_8: c_uint = 0x01;
pub const CIR_FIFOCON_RX_TRIGGER_LEV_16: c_uint = 0x02;
pub const CIR_FIFOCON_RX_TRIGGER_LEV_24: c_uint = 0x03;
// FIXME: make this a runtime option
// select RX trigger level as 24

// CIR IRFIFOSTS settings
pub const CIR_IRFIFOSTS_IR_PENDING: c_uint = 0x80;
pub const CIR_IRFIFOSTS_RX_GS: c_uint = 0x40;
pub const CIR_IRFIFOSTS_RX_FTA: c_uint = 0x20;
pub const CIR_IRFIFOSTS_RX_EMPTY: c_uint = 0x10;
pub const CIR_IRFIFOSTS_RX_FULL: c_uint = 0x08;
pub const CIR_IRFIFOSTS_TX_FTA: c_uint = 0x04;
pub const CIR_IRFIFOSTS_TX_EMPTY: c_uint = 0x02;
pub const CIR_IRFIFOSTS_TX_FULL: c_uint = 0x01;
// CIR WAKE UP Regs
pub const CIR_WAKE_IRCON: c_uint = 0x00;
pub const CIR_WAKE_IRSTS: c_uint = 0x01;
pub const CIR_WAKE_IREN: c_uint = 0x02;
pub const CIR_WAKE_FIFO_CMP_DEEP: c_uint = 0x03;
pub const CIR_WAKE_FIFO_CMP_TOL: c_uint = 0x04;
pub const CIR_WAKE_FIFO_COUNT: c_uint = 0x05;
pub const CIR_WAKE_SLCH: c_uint = 0x06;
pub const CIR_WAKE_SLCL: c_uint = 0x07;
pub const CIR_WAKE_FIFOCON: c_uint = 0x08;
pub const CIR_WAKE_SRXFSTS: c_uint = 0x09;
pub const CIR_WAKE_SAMPLE_RX_FIFO: c_uint = 0x0a;
pub const CIR_WAKE_WR_FIFO_DATA: c_uint = 0x0b;
pub const CIR_WAKE_RD_FIFO_ONLY: c_uint = 0x0c;
pub const CIR_WAKE_RD_FIFO_ONLY_IDX: c_uint = 0x0d;
pub const CIR_WAKE_FIFO_IGNORE: c_uint = 0x0e;
pub const CIR_WAKE_IRFSM: c_uint = 0x0f;
// CIR WAKE UP IRCON settings
pub const CIR_WAKE_IRCON_DEC_RST: c_uint = 0x80;
pub const CIR_WAKE_IRCON_MODE1: c_uint = 0x40;
pub const CIR_WAKE_IRCON_MODE0: c_uint = 0x20;
pub const CIR_WAKE_IRCON_RXEN: c_uint = 0x10;
pub const CIR_WAKE_IRCON_R: c_uint = 0x08;
pub const CIR_WAKE_IRCON_RXINV: c_uint = 0x04;
// FIXME/jarod: make this a runtime option
// select a same sample period like cir register

// CIR WAKE IRSTS Bits
pub const CIR_WAKE_IRSTS_RDR: c_uint = 0x80;
pub const CIR_WAKE_IRSTS_RTR: c_uint = 0x40;
pub const CIR_WAKE_IRSTS_PE: c_uint = 0x20;
pub const CIR_WAKE_IRSTS_RFO: c_uint = 0x10;
pub const CIR_WAKE_IRSTS_GH: c_uint = 0x08;
pub const CIR_WAKE_IRSTS_IR_PENDING: c_uint = 0x01;
// CIR WAKE UP IREN Bits
pub const CIR_WAKE_IREN_RDR: c_uint = 0x80;
pub const CIR_WAKE_IREN_RTR: c_uint = 0x40;
pub const CIR_WAKE_IREN_PE: c_uint = 0x20;
pub const CIR_WAKE_IREN_RFO: c_uint = 0x10;
pub const CIR_WAKE_IREN_GH: c_uint = 0x08;
// CIR WAKE FIFOCON settings
pub const CIR_WAKE_FIFOCON_RXFIFOCLR: c_uint = 0x08;
pub const CIR_WAKE_FIFOCON_RX_TRIGGER_LEV_67: c_uint = 0x00;
pub const CIR_WAKE_FIFOCON_RX_TRIGGER_LEV_66: c_uint = 0x01;
pub const CIR_WAKE_FIFOCON_RX_TRIGGER_LEV_65: c_uint = 0x02;
pub const CIR_WAKE_FIFOCON_RX_TRIGGER_LEV_64: c_uint = 0x03;
// FIXME: make this a runtime option
// select WAKE UP RX trigger level as 67

// CIR WAKE SRXFSTS settings
pub const CIR_WAKE_IRFIFOSTS_RX_GS: c_uint = 0x80;
pub const CIR_WAKE_IRFIFOSTS_RX_FTA: c_uint = 0x40;
pub const CIR_WAKE_IRFIFOSTS_RX_EMPTY: c_uint = 0x20;
pub const CIR_WAKE_IRFIFOSTS_RX_FULL: c_uint = 0x10;
//
// The CIR Wake FIFO buffer is 67 bytes long, but the stock remote wakes
// the system comparing only 65 bytes (fails with this set to 67)
//
pub const CIR_WAKE_FIFO_CMP_BYTES: c_int = 65;
// CIR Wake byte comparison tolerance
pub const CIR_WAKE_CMP_TOLERANCE: c_int = 5;
//
// Extended Function Enable Registers:
// Extended Function Index Register
// Extended Function Data Register
//
pub const CR_EFIR: c_uint = 0x2e;
pub const CR_EFDR: c_uint = 0x2f;
// Possible alternate EFER values, depends on how the chip is wired
pub const CR_EFIR2: c_uint = 0x4e;
pub const CR_EFDR2: c_uint = 0x4f;
// Extended Function Mode enable/disable magic values
pub const EFER_EFM_ENABLE: c_uint = 0x87;
pub const EFER_EFM_DISABLE: c_uint = 0xaa;
// Config regs we need to care about
pub const CR_SOFTWARE_RESET: c_uint = 0x02;
pub const CR_LOGICAL_DEV_SEL: c_uint = 0x07;
pub const CR_CHIP_ID_HI: c_uint = 0x20;
pub const CR_CHIP_ID_LO: c_uint = 0x21;
pub const CR_DEV_POWER_DOWN: c_uint = 0x22 /* bit 2 is CIR power, default power on */;
pub const CR_OUTPUT_PIN_SEL: c_uint = 0x27;
pub const CR_MULTIFUNC_PIN_SEL: c_uint = 0x2c;
pub const CR_LOGICAL_DEV_EN: c_uint = 0x30 /* valid for all logical devices */;
// next three regs valid for both the CIR and CIR_WAKE logical devices
pub const CR_CIR_BASE_ADDR_HI: c_uint = 0x60;
pub const CR_CIR_BASE_ADDR_LO: c_uint = 0x61;
pub const CR_CIR_IRQ_RSRC: c_uint = 0x70;
// next three regs valid only for ACPI logical dev
pub const CR_ACPI_CIR_WAKE: c_uint = 0xe0;
pub const CR_ACPI_IRQ_EVENTS: c_uint = 0xf6;
pub const CR_ACPI_IRQ_EVENTS2: c_uint = 0xf7;
// Logical devices that we need to care about
pub const LOGICAL_DEV_LPT: c_uint = 0x01;
pub const LOGICAL_DEV_CIR: c_uint = 0x06;
pub const LOGICAL_DEV_ACPI: c_uint = 0x0a;
pub const LOGICAL_DEV_CIR_WAKE: c_uint = 0x0e;
pub const LOGICAL_DEV_DISABLE: c_uint = 0x00;
pub const LOGICAL_DEV_ENABLE: c_uint = 0x01;
pub const CIR_WAKE_ENABLE_BIT: c_uint = 0x08;
pub const PME_INTR_CIR_PASS_BIT: c_uint = 0x08;
// w83677hg CIR pin config
pub const OUTPUT_PIN_SEL_MASK: c_uint = 0xbc;
pub const OUTPUT_ENABLE_CIR: c_uint = 0x01 /* Pin95=CIRRX, Pin96=CIRTX1 */;
pub const OUTPUT_ENABLE_CIRWB: c_uint = 0x40 /* enable wide-band sensor */;
// w83667hg CIR pin config
pub const MULTIFUNC_PIN_SEL_MASK: c_uint = 0x1f;
pub const MULTIFUNC_ENABLE_CIR: c_uint = 0x80 /* Pin75=CIRRX, Pin76=CIRTX1 */;
pub const MULTIFUNC_ENABLE_CIRWB: c_uint = 0x20 /* enable wide-band sensor */;
// MCE CIR signal length, related on sample period
// MCE CIR controller signal length: about 43ms
// 43ms / 50us (sample period) * 0.85 (inaccuracy)
//
pub const CONTROLLER_BUF_LEN_MIN: c_int = 830;
// MCE CIR keyboard signal length: about 26ms
// 26ms / 50us (sample period) * 0.85 (inaccuracy)
//
pub const KEYBOARD_BUF_LEN_MAX: c_int = 650;
pub const KEYBOARD_BUF_LEN_MIN: c_int = 610;
// MCE CIR mouse signal length: about 24ms
// 24ms / 50us (sample period) * 0.85 (inaccuracy)
//
pub const MOUSE_BUF_LEN_MIN: c_int = 565;
pub const CIR_SAMPLE_PERIOD: c_int = 50;

// MAX silence time that driver will sent to lirc
pub const MAX_SILENCE_TIME: c_int = 60000;

pub const SAMPLE_PERIOD: c_int = 100;

pub const SAMPLE_PERIOD: c_int = 50;

pub const SAMPLE_PERIOD: c_int = 25;

pub const SAMPLE_PERIOD: c_int = 1;

// as VISTA MCE definition, valid carrier value
pub const MAX_CARRIER: c_int = 60000;
pub const MIN_CARRIER: c_int = 30000;
// max wakeup sequence length
pub const WAKEUP_MAX_SIZE: c_int = 65;
