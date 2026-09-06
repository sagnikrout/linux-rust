//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/musb/cppi_dma.h
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
// Copyright (C) 2005-2006 by Texas Instruments

// CPPI RX/TX state RAM
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cppi_tx_stateram {
    pub /: *mut *mut u32 tx_head; / "DMA packet" head descriptor,
    pub tx_buf: u32,
    pub /: *mut *mut u32 tx_current; / current descriptor,
    pub tx_buf_current: u32,
    pub /: *mut *mut u32 tx_info; / flags, remaining buflen,
    pub tx_rem_len: u32,
    pub /: *mut *mut u32 tx_dummy; / unused,
    pub tx_complete: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cppi_rx_stateram {
    pub rx_skipbytes: u32,
    pub rx_head: u32,
    pub /: *mut *mut u32 rx_sop; / "DMA packet" head descriptor,
    pub /: *mut *mut u32 rx_current; / current descriptor,
    pub rx_buf_current: u32,
    pub rx_len_len: u32,
    pub rx_cnt_cnt: u32,
    pub rx_complete: u32,
}

// hw_options bits in CPPI buffer descriptors

pub const CPPI_RECV_PKTLEN_MASK: c_uint = 0xFFFF;
pub const CPPI_BUFFER_LEN_MASK: c_uint = 0xFFFF;

// CPPI data structure definitions

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cppi_descriptor {
// hardware overlay
    pub /: *mut *mut u32 hw_next; / next buffer descriptor Pointer,
    pub /: *mut *mut u32 hw_bufp; / i/o buffer pointer,
    pub /: *mut *mut u32 hw_off_len; / buffer_offset16, buffer_length16,
    pub etc*/: *mut *mut u32 hw_options; / flags: SOP, EOP,
    pub next: *mut cppi_descriptor,
    pub /: *mut *mut dma_addr_t dma; / address of this descriptor,
    pub /: *mut *mut u32 buflen; / for RX: original buffer length,
// C attribute field omitted
    pub cppi: struct,
// CPPI  Channel Control structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cppi_channel {
    pub channel: dma_channel,
// back pointer to the DMA controller structure
    pub controller: *mut cppi,
// which direction of which endpoint?
    pub hw_ep: *mut musb_hw_ep,
    pub transmit: bool,
    pub index: u8,
// DMA modes:  RNDIS or "transparent"
    pub is_rndis: u8,
// book keeping for current transfer request
    pub buf_dma: dma_addr_t,
    pub buf_len: u32,
    pub maxpacket: u32,
    pub /: *mut *mut u32 offset; / dma requested,
    pub /: *mut *mut *mut void __iomem state_ram; / CPPI state,
    pub freelist: *mut cppi_descriptor,
// BD management fields
    pub head: *mut cppi_descriptor,
    pub tail: *mut cppi_descriptor,
    pub last_processed: *mut cppi_descriptor,
// use tx_complete in host role to track endpoints waiting for
// FIFONOTEMPTY to clear.
//
    pub tx_complete: list_head,
}

// CPPI DMA controller object
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cppi {
    pub controller: dma_controller,
    pub /: *mut *mut *mut void __iomem mregs; / Mentor regs,
    pub /: *mut *mut *mut void __iomem tibase; / TI/CPPI regs,
    pub irq: c_int,
    pub tx: [cppi_channel; 4],
    pub rx: [cppi_channel; 4],
    pub pool: *mut dma_pool,
    pub tx_complete: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cppi41_dma_channel {
    pub channel: dma_channel,
    pub controller: *mut cppi41_dma_controller,
    pub hw_ep: *mut musb_hw_ep,
    pub dc: *mut dma_chan,
    pub cookie: dma_cookie_t,
    pub port_num: u8,
    pub is_tx: u8,
    pub is_allocated: u8,
    pub usb_toggle: u8,
    pub buf_addr: dma_addr_t,
    pub total_len: u32,
    pub prog_len: u32,
    pub transferred: u32,
    pub packet_sz: u32,
    pub tx_check: list_head,
    pub tx_zlp: c_int,
}
