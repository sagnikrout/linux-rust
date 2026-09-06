//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpib/eastwood/fluke_gpib.h
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
// Author: Frank Mori Hess <fmh6jj@gmail.com>
// copyright: (C) 2006, 2010, 2015 Fluke Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fluke_priv {
    pub nec7210_priv: nec7210_priv,
    pub gpib_iomem_res: *mut resource,
    pub write_transfer_counter_res: *mut resource,
    pub dma_port_res: *mut resource,
    pub irq: c_int,
    pub dma_channel: *mut dma_chan,
    pub dma_buffer: *mut u8,
    pub dma_buffer_size: c_int,
    pub write_transfer_counter: *mut void __iomem,
}

// cb7210 specific registers and bits
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cb7210_regs {
    STATE1_REG = 0x4,
    ISR0_IMR0 = 0x6,
    BUS_STATUS = 0x7
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cb7210_page_in {
    ISR0_IMR0_PAGE = 1,
    BUS_STATUS_PAGE = 1,
    STATE1_PAGE = 1
}

// IMR0 -- Interrupt Mode Register 0
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum imr0_bits {
    FLUKE_IFCIE_BIT = 0x8,	/* interface clear interrupt */
}

// ISR0 -- Interrupt Status Register 0
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum isr0_bits {
    FLUKE_IFCI_BIT = 0x8,	/* interface clear interrupt */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum state1_bits {
    SOURCE_HANDSHAKE_SIDS_BITS = 0x0, /* source idle state */
    SOURCE_HANDSHAKE_SGNS_BITS = 0x1, /* source generate state */
    SOURCE_HANDSHAKE_SDYS_BITS = 0x2, /* source delay state */
    SOURCE_HANDSHAKE_STRS_BITS = 0x5, /* source transfer state */
    SOURCE_HANDSHAKE_MASK = 0x7
}

//
// we customized the cb7210 vhdl to give the "data in" status
// on the unused bit 7 of the address0 register.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cb7210_address0 {
    DATA_IN_STATUS = 0x80
}

// don't use without locking nec_priv->register_page_lock
// chip auto clears the page after a read
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
    AUX_NBAF = 0xe,	// new byte available false (also clears seoi)
    AUX_LO_SPEED = 0x40,
    AUX_HI_SPEED = 0x41,
}
