//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpib/cb7210/cb7210.h
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
// copyright            : (C) 2002 by Frank Mori Hess
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pci_chip {
    PCI_CHIP_NONE = 0,
    PCI_CHIP_AMCC_S5933,
    PCI_CHIP_QUANCOM
}

// struct which defines private_data for cb7210 boards
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cb7210_priv {
    pub nec7210_priv: nec7210_priv,
    pub pci_device: *mut pci_dev,
// base address of amccs5933 pci chip
    pub amcc_iobase: c_ulong,
    pub fifo_iobase: c_ulong,
    pub irq: c_uint,
    pub pci_chip: pci_chip,
    pub hs_mode_bits: u8,
    pub 1: unsigned out_fifo_half_empty :,
    pub 1: unsigned in_fifo_half_full :,
}

// pci-gpib register offset
// uses 10 ioports
// fifo size in bytes
// cb7210 specific registers and bits
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cb7210_regs {
    BUS_STATUS = 0x7,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cb7210_page_in {
    BUS_STATUS_PAGE = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hs_regs {
// write registers
    HS_MODE = 0x8,	/* HS_MODE register */
    HS_INT_LEVEL = 0x9,	/* HS_INT_LEVEL register */
// read registers
    HS_STATUS = 0x8,	/* HS_STATUS register */
}

// don't use for register_num < 8, since it doesn't lock
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bus_status_bits {
    BSR_ATN_BIT = 0x1,
    BSR_EOI_BIT = 0x2,
    BSR_SRQ_BIT = 0x4,
    BSR_IFC_BIT = 0x8,
    BSR_REN_BIT = 0x10,
    BSR_DAV_BIT = 0x20,
    BSR_NRFD_BIT = 0x40,
    BSR_NDAC_BIT = 0x80,
}

// CBI 488.2 HS control
//
// when both bit 0 and 1 are set, it
// 1 clears the transmit state machine to an initial condition
// 2 clears any residual interrupts left latched on cbi488.2
// 3 resets all control bits in HS_MODE to zero
// 4 enables TX empty interrupts
// when both bit 0 and 1 are zero, then the high speed mode is disabled
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hs_mode_bits {
    HS_ENABLE_MASK = 0x3,
    HS_TX_ENABLE = (1 << 0),
    HS_RX_ENABLE = (1 << 1),
    HS_HF_INT_EN = (1 << 3),
    HS_CLR_SRQ_INT = (1 << 4),
    HS_CLR_EOI_EMPTY_INT = (1 << 5),
    HS_CLR_HF_INT = (1 << 6),
    HS_SYS_CONTROL = (1 << 7),
}

// CBI 488.2 status
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hs_status_bits {
    HS_FIFO_FULL = (1 << 0),
    HS_HALF_FULL = (1 << 1),
    HS_SRQ_INT = (1 << 2),
    HS_EOI_INT = (1 << 3),
    HS_TX_MSB_NOT_EMPTY = (1 << 4),
    HS_RX_MSB_NOT_EMPTY = (1 << 5),
    HS_TX_LSB_NOT_EMPTY = (1 << 6),
    HS_RX_LSB_NOT_EMPTY = (1 << 7),
}

// CBI488.2 hs_int_level register
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hs_int_level_bits {
    HS_RESET7210 = (1 << 7),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cb7210_aux_cmds {
//
// AUX_RTL2 is an undocumented aux command which causes cb7210 to assert
// (and keep asserted) local rtl message.  This is used in conjunction
// with the (stupid) cb7210 implementation
// of the normal nec7210 AUX_RTL aux command, which
// causes the rtl message to toggle between on and off.
//
    AUX_RTL2 = 0xd,
    AUX_LO_SPEED = 0x40,
    AUX_HI_SPEED = 0x41,
}
