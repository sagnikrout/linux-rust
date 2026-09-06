//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpib/fmh_gpib/fmh_gpib.h
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
// Copyright: (C) 2006, 2010, 2015 Fluke Corporation
// (C) 2017 Frank Mori Hess
//

// We don't have a real pci vendor/device id, the following will need to be
// patched to match prototype hardware.
//
pub const BOGUS_PCI_VENDOR_ID_FLUKE: c_uint = 0xffff;
pub const BOGUS_PCI_DEVICE_ID_FLUKE_BLADERUNNER: c_uint = 0x0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fmh_priv {
    pub nec7210_priv: nec7210_priv,
    pub gpib_iomem_res: *mut resource,
    pub write_transfer_counter_res: *mut resource,
    pub dma_port_res: *mut resource,
    pub irq: c_int,
    pub dma_channel: *mut dma_chan,
    pub dma_buffer: *mut u8,
    pub dma_buffer_size: c_int,
    pub dma_burst_length: c_int,
    pub fifo_base: *mut void __iomem,
    pub 1: unsigned supports_fifo_interrupts :,
}

// registers beyond the nec7210 register set
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fmh_gpib_regs {
    EXT_STATUS_1_REG = 0x9,
    STATE1_REG = 0xc,
    ISR0_IMR0_REG = 0xe,
    BUS_STATUS_REG = 0xf
}

// IMR0 -- Interrupt Mode Register 0
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum imr0_bits {
    ATN_INTERRUPT_ENABLE_BIT = 0x4,
    IFC_INTERRUPT_ENABLE_BIT = 0x8
}

// ISR0 -- Interrupt Status Register 0
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum isr0_bits {
    ATN_INTERRUPT_BIT = 0x4,
    IFC_INTERRUPT_BIT = 0x8
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fmh_gpib_auxmr_bits {
    AUX_I_REG = 0xe0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum aux_reg_i_bits {
    LOCAL_PPOLL_MODE_BIT = 0x4
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ext_status_1_bits {
    DATA_IN_STATUS_BIT = 0x01,
    DATA_OUT_STATUS_BIT = 0x02,
    COMMAND_OUT_STATUS_BIT = 0x04,
    RFD_HOLDOFF_STATUS_BIT = 0x08,
    END_STATUS_BIT = 0x10
}

// dma fifo reg and bits
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dma_fifo_regs {
    FIFO_DATA_REG = 0x0,
    FIFO_CONTROL_STATUS_REG = 0x1,
    FIFO_XFER_COUNTER_REG = 0x2,
    FIFO_MAX_BURST_LENGTH_REG = 0x3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fifo_data_bits {
    FIFO_DATA_EOI_FLAG = 0x100
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fifo_control_bits {
    TX_FIFO_DMA_REQUEST_ENABLE = 0x0001,
    TX_FIFO_CLEAR = 0x0002,
    TX_FIFO_HALF_EMPTY_INTERRUPT_ENABLE = 0x0008,
    RX_FIFO_DMA_REQUEST_ENABLE = 0x0100,
    RX_FIFO_CLEAR = 0x0200,
    RX_FIFO_HALF_FULL_INTERRUPT_ENABLE = 0x0800
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fifo_status_bits {
    TX_FIFO_EMPTY = 0x0001,
    TX_FIFO_FULL = 0x0002,
    TX_FIFO_HALF_EMPTY = 0x0004,
    TX_FIFO_HALF_EMPTY_INTERRUPT_IS_ENABLED = 0x0008,
    TX_FIFO_DMA_REQUEST_IS_ENABLED = 0x0010,
    RX_FIFO_EMPTY = 0x0100,
    RX_FIFO_FULL = 0x0200,
    RX_FIFO_HALF_FULL = 0x0400,
    RX_FIFO_HALF_FULL_INTERRUPT_IS_ENABLED = 0x0800,
    RX_FIFO_DMA_REQUEST_IS_ENABLED = 0x1000
}

extern "C" {
    pub fn readb(nec_priv->offset: *mut *mut nec_priv->mmiobase + register_num) -> return;
}
extern "C" {
    pub fn readw(fifo_reg_offset: *mut *mut fmh_priv->fifo_base + register_num) -> return;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bus_status_bits {
    BSR_ATN_BIT = 0x01,
    BSR_EOI_BIT = 0x02,
    BSR_SRQ_BIT = 0x04,
    BSR_IFC_BIT = 0x08,
    BSR_REN_BIT = 0x10,
    BSR_DAV_BIT = 0x20,
    BSR_NRFD_BIT = 0x40,
    BSR_NDAC_BIT = 0x80,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fmh_gpib_aux_cmds {
// AUX_RTL2 is an auxiliary command which causes the cb7210 to assert
// (and keep asserted) the local rtl message.  This is used in conjunction
// with the normal nec7210 AUX_RTL command, which
// pulses the rtl message, having the effect of clearing rtl if it was left
// asserted by AUX_RTL2.
//
    AUX_RTL2 = 0x0d,
    AUX_RFD_HOLDOFF_ASAP = 0x15,
    AUX_REQT = 0x18,
    AUX_REQF = 0x19,
    AUX_LO_SPEED = 0x40,
    AUX_HI_SPEED = 0x41
}
