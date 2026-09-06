//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/stmicro/stmmac/dwmac_dma.h
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
// DMA CRS Control and Status Register Mapping
pub const DMA_BUS_MODE: c_uint = 0x00001000	/* Bus Mode */;
pub const DMA_BUS_MODE_SFT_RESET: c_uint = 0x00000001	/* Software Reset */;
pub const DMA_XMT_POLL_DEMAND: c_uint = 0x00001004	/* Transmit Poll Demand */;
pub const DMA_RCV_POLL_DEMAND: c_uint = 0x00001008	/* Received Poll Demand */;
pub const DMA_RCV_BASE_ADDR: c_uint = 0x0000100c	/* Receive List Base */;
pub const DMA_TX_BASE_ADDR: c_uint = 0x00001010	/* Transmit List Base */;
pub const DMA_STATUS: c_uint = 0x00001014	/* Status Register */;
pub const DMA_STATUS_GPI: c_uint = 0x10000000	/* PMT interrupt */;
pub const DMA_STATUS_GMI: c_uint = 0x08000000	/* MMC interrupt */;
pub const DMA_STATUS_GLI: c_uint = 0x04000000	/* GMAC Line interface int */;

pub const DMA_STATUS_NIS: c_uint = 0x00010000	/* Normal Interrupt Summary */;
pub const DMA_STATUS_AIS: c_uint = 0x00008000	/* Abnormal Interrupt Summary */;
pub const DMA_STATUS_ERI: c_uint = 0x00004000	/* Early Receive Interrupt */;
pub const DMA_STATUS_FBI: c_uint = 0x00002000	/* Fatal Bus Error Interrupt */;
pub const DMA_STATUS_ETI: c_uint = 0x00000400	/* Early Transmit Interrupt */;
pub const DMA_STATUS_RWT: c_uint = 0x00000200	/* Receive Watchdog Timeout */;
pub const DMA_STATUS_RPS: c_uint = 0x00000100	/* Receive Process Stopped */;
pub const DMA_STATUS_RU: c_uint = 0x00000080	/* Receive Buffer Unavailable */;
pub const DMA_STATUS_RI: c_uint = 0x00000040	/* Receive Interrupt */;
pub const DMA_STATUS_UNF: c_uint = 0x00000020	/* Transmit Underflow */;
pub const DMA_STATUS_OVF: c_uint = 0x00000010	/* Receive Overflow */;
pub const DMA_STATUS_TJT: c_uint = 0x00000008	/* Transmit Jabber Timeout */;
pub const DMA_STATUS_TU: c_uint = 0x00000004	/* Transmit Buffer Unavailable */;
pub const DMA_STATUS_TPS: c_uint = 0x00000002	/* Transmit Process Stopped */;
pub const DMA_STATUS_TI: c_uint = 0x00000001	/* Transmit Interrupt */;

pub const DMA_CONTROL: c_uint = 0x00001018	/* Ctrl (Operational Mode) */;
// DMA Control register defines
pub const DMA_CONTROL_FTF: c_uint = 0x00100000	/* Flush transmit FIFO */;
pub const DMA_CONTROL_ST: c_uint = 0x00002000	/* Start/Stop Transmission */;
pub const DMA_CONTROL_SR: c_uint = 0x00000002	/* Start/Stop Receive */;
pub const DMA_INTR_ENA: c_uint = 0x0000101c	/* Interrupt Enable */;
// DMA Normal interrupt
pub const DMA_INTR_ENA_NIE: c_uint = 0x00010000	/* Normal Summary */;
pub const DMA_INTR_ENA_TIE: c_uint = 0x00000001	/* Transmit Interrupt */;
pub const DMA_INTR_ENA_RIE: c_uint = 0x00000040	/* Receive Interrupt */;

// DMA Abnormal interrupt
pub const DMA_INTR_ENA_AIE: c_uint = 0x00008000	/* Abnormal Summary */;
pub const DMA_INTR_ENA_FBE: c_uint = 0x00002000	/* Fatal Bus Error */;
pub const DMA_INTR_ENA_UNE: c_uint = 0x00000020	/* Tx Underflow */;

// DMA default interrupt mask

pub const DMA_MISSED_FRAME_CTR: c_uint = 0x00001020	/* Missed Frame Counter */;
// Following DMA defines are channels oriented
pub const DMA_CHAN_BASE_OFFSET: c_uint = 0x100;

// Rx watchdog register
pub const DMA_RX_WATCHDOG: c_uint = 0x00001024;
// AXI Master Bus Mode
pub const DMA_AXI_BUS_MODE: c_uint = 0x00001028;

pub const DMA_CUR_TX_BUF_ADDR: c_uint = 0x00001050	/* Current Host Tx Buffer */;
pub const DMA_CUR_RX_BUF_ADDR: c_uint = 0x00001054	/* Current Host Rx Buffer */;
pub const DMA_HW_FEATURE: c_uint = 0x00001058	/* HW Feature Register */;
pub const NUM_DWMAC100_DMA_REGS: c_int = 9;
pub const NUM_DWMAC1000_DMA_REGS: c_int = 23;
pub const NUM_DWMAC4_DMA_REGS: c_int = 27;
extern "C" {
    pub fn dwmac_enable_dma_transmission(ioaddr: *mut void __iomem, chan: u32);
}
extern "C" {
    pub fn dwmac_enable_dma_reception(ioaddr: *mut void __iomem, chan: u32);
}
extern "C" {
    pub fn dwmac_dma_reset(ioaddr: *mut void __iomem) -> c_int;
}
