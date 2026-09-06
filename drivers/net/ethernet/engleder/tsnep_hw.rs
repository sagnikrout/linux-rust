//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/engleder/tsnep_hw.h
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
// Copyright (C) 2021 Gerhard Engleder <gerhard@engleder-embedded.com>
// Hardware definition of TSNEP and EtherCAT MAC device

// type
pub const ECM_TYPE: c_uint = 0x0000;
pub const ECM_REVISION_MASK: c_uint = 0x000000FF;
pub const ECM_REVISION_SHIFT: c_int = 0;
pub const ECM_VERSION_MASK: c_uint = 0x0000FF00;
pub const ECM_VERSION_SHIFT: c_int = 8;
pub const ECM_QUEUE_COUNT_MASK: c_uint = 0x00070000;
pub const ECM_QUEUE_COUNT_SHIFT: c_int = 16;
pub const ECM_GATE_CONTROL: c_uint = 0x02000000;
// system time
pub const ECM_SYSTEM_TIME_LOW: c_uint = 0x0008;
pub const ECM_SYSTEM_TIME_HIGH: c_uint = 0x000C;
// clock
pub const ECM_CLOCK_RATE: c_uint = 0x0010;
pub const ECM_CLOCK_RATE_OFFSET_MASK: c_uint = 0x7FFFFFFF;
pub const ECM_CLOCK_RATE_OFFSET_SIGN: c_uint = 0x80000000;
// interrupt
pub const ECM_INT_ENABLE: c_uint = 0x0018;
pub const ECM_INT_ACTIVE: c_uint = 0x001C;
pub const ECM_INT_ACKNOWLEDGE: c_uint = 0x001C;
pub const ECM_INT_LINK: c_uint = 0x00000020;
pub const ECM_INT_TX_0: c_uint = 0x00000100;
pub const ECM_INT_RX_0: c_uint = 0x00000200;
pub const ECM_INT_TXRX_SHIFT: c_int = 2;
pub const ECM_INT_ALL: c_uint = 0x7FFFFFFF;
pub const ECM_INT_DISABLE: c_uint = 0x80000000;
// reset
pub const ECM_RESET: c_uint = 0x0020;
pub const ECM_RESET_COMMON: c_uint = 0x00000001;
pub const ECM_RESET_CHANNEL: c_uint = 0x00000100;
pub const ECM_RESET_TXRX: c_uint = 0x00010000;
// counter
pub const ECM_COUNTER_LOW: c_uint = 0x0028;
pub const ECM_COUNTER_HIGH: c_uint = 0x002C;
// interrupt delay
pub const ECM_INT_DELAY: c_uint = 0x0030;
pub const ECM_INT_DELAY_MASK: c_uint = 0xF0;
pub const ECM_INT_DELAY_SHIFT: c_int = 4;
pub const ECM_INT_DELAY_BASE_US: c_int = 16;
pub const ECM_INT_DELAY_OFFSET: c_int = 1;
// control and status
pub const ECM_STATUS: c_uint = 0x0080;
pub const ECM_LINK_MODE_OFF: c_uint = 0x01000000;
pub const ECM_LINK_MODE_100: c_uint = 0x02000000;
pub const ECM_LINK_MODE_1000: c_uint = 0x04000000;
pub const ECM_NO_LINK: c_uint = 0x01000000;
pub const ECM_LINK_MODE_MASK: c_uint = 0x06000000;
// management data
pub const ECM_MD_CONTROL: c_uint = 0x0084;
pub const ECM_MD_STATUS: c_uint = 0x0084;
pub const ECM_MD_PREAMBLE: c_uint = 0x00000001;
pub const ECM_MD_READ: c_uint = 0x00000004;
pub const ECM_MD_WRITE: c_uint = 0x00000002;
pub const ECM_MD_ADDR_MASK: c_uint = 0x000000F8;
pub const ECM_MD_ADDR_SHIFT: c_int = 3;
pub const ECM_MD_PHY_ADDR_MASK: c_uint = 0x00001F00;
pub const ECM_MD_PHY_ADDR_SHIFT: c_int = 8;
pub const ECM_MD_BUSY: c_uint = 0x00000001;
pub const ECM_MD_DATA_MASK: c_uint = 0xFFFF0000;
pub const ECM_MD_DATA_SHIFT: c_int = 16;
// statistic
pub const ECM_STAT: c_uint = 0x00B0;
pub const ECM_STAT_RX_ERR_MASK: c_uint = 0x000000FF;
pub const ECM_STAT_RX_ERR_SHIFT: c_int = 0;
pub const ECM_STAT_INV_FRM_MASK: c_uint = 0x0000FF00;
pub const ECM_STAT_INV_FRM_SHIFT: c_int = 8;
pub const ECM_STAT_FWD_RX_ERR_MASK: c_uint = 0x00FF0000;
pub const ECM_STAT_FWD_RX_ERR_SHIFT: c_int = 16;
// tsnep
pub const TSNEP_MAC_SIZE: c_uint = 0x4000;
pub const TSNEP_QUEUE_SIZE: c_uint = 0x1000;

pub const TSNEP_MAX_QUEUES: c_int = 8;

pub const TSNEP_DESC_SIZE: c_int = 256;
pub const TSNEP_DESC_OFFSET: c_int = 128;
// tsnep register
pub const TSNEP_INFO: c_uint = 0x0100;
pub const TSNEP_INFO_TX_TIME: c_uint = 0x00010000;
pub const TSNEP_CONTROL: c_uint = 0x0108;
pub const TSNEP_CONTROL_TX_RESET: c_uint = 0x00000001;
pub const TSNEP_CONTROL_TX_ENABLE: c_uint = 0x00000002;
pub const TSNEP_CONTROL_TX_DMA_ERROR: c_uint = 0x00000010;
pub const TSNEP_CONTROL_TX_DESC_ERROR: c_uint = 0x00000020;
pub const TSNEP_CONTROL_RX_RESET: c_uint = 0x00000100;
pub const TSNEP_CONTROL_RX_ENABLE: c_uint = 0x00000200;
pub const TSNEP_CONTROL_RX_DISABLE: c_uint = 0x00000400;
pub const TSNEP_CONTROL_RX_DMA_ERROR: c_uint = 0x00001000;
pub const TSNEP_CONTROL_RX_DESC_ERROR: c_uint = 0x00002000;
pub const TSNEP_TX_DESC_ADDR_LOW: c_uint = 0x0140;
pub const TSNEP_TX_DESC_ADDR_HIGH: c_uint = 0x0144;
pub const TSNEP_RX_DESC_ADDR_LOW: c_uint = 0x0180;
pub const TSNEP_RX_DESC_ADDR_HIGH: c_uint = 0x0184;
pub const TSNEP_RESET_OWNER_COUNTER: c_uint = 0x01;
pub const TSNEP_RX_STATISTIC: c_uint = 0x0190;
pub const TSNEP_RX_STATISTIC_NO_DESC_MASK: c_uint = 0x000000FF;
pub const TSNEP_RX_STATISTIC_NO_DESC_SHIFT: c_int = 0;
pub const TSNEP_RX_STATISTIC_BUFFER_TOO_SMALL_MASK: c_uint = 0x0000FF00;
pub const TSNEP_RX_STATISTIC_BUFFER_TOO_SMALL_SHIFT: c_int = 8;
pub const TSNEP_RX_STATISTIC_FIFO_OVERFLOW_MASK: c_uint = 0x00FF0000;
pub const TSNEP_RX_STATISTIC_FIFO_OVERFLOW_SHIFT: c_int = 16;
pub const TSNEP_RX_STATISTIC_INVALID_FRAME_MASK: c_uint = 0xFF000000;
pub const TSNEP_RX_STATISTIC_INVALID_FRAME_SHIFT: c_int = 24;
pub const TSNEP_RX_STATISTIC_NO_DESC: c_uint = 0x0190;
pub const TSNEP_RX_STATISTIC_BUFFER_TOO_SMALL: c_uint = 0x0191;
pub const TSNEP_RX_STATISTIC_FIFO_OVERFLOW: c_uint = 0x0192;
pub const TSNEP_RX_STATISTIC_INVALID_FRAME: c_uint = 0x0193;
pub const TSNEP_MAC_ADDRESS_LOW: c_uint = 0x0800;
pub const TSNEP_MAC_ADDRESS_HIGH: c_uint = 0x0804;
pub const TSNEP_RX_FILTER: c_uint = 0x0806;
pub const TSNEP_RX_FILTER_ACCEPT_ALL_MULTICASTS: c_uint = 0x0001;
pub const TSNEP_RX_FILTER_ACCEPT_ALL_UNICASTS: c_uint = 0x0002;
pub const TSNEP_GC: c_uint = 0x0808;
pub const TSNEP_GC_ENABLE_A: c_uint = 0x00000002;
pub const TSNEP_GC_ENABLE_B: c_uint = 0x00000004;
pub const TSNEP_GC_DISABLE: c_uint = 0x00000008;
pub const TSNEP_GC_ENABLE_TIMEOUT: c_uint = 0x00000010;
pub const TSNEP_GC_ACTIVE_A: c_uint = 0x00000002;
pub const TSNEP_GC_ACTIVE_B: c_uint = 0x00000004;
pub const TSNEP_GC_CHANGE_AB: c_uint = 0x00000008;
pub const TSNEP_GC_TIMEOUT_ACTIVE: c_uint = 0x00000010;
pub const TSNEP_GC_TIMEOUT_SIGNAL: c_uint = 0x00000020;
pub const TSNEP_GC_LIST_ERROR: c_uint = 0x00000080;
pub const TSNEP_GC_OPEN: c_uint = 0x00FF0000;
pub const TSNEP_GC_OPEN_SHIFT: c_int = 16;
pub const TSNEP_GC_NEXT_OPEN: c_uint = 0xFF000000;
pub const TSNEP_GC_NEXT_OPEN_SHIFT: c_int = 24;
pub const TSNEP_GC_TIMEOUT: c_int = 131072;
pub const TSNEP_GC_TIME: c_uint = 0x080C;
pub const TSNEP_GC_CHANGE: c_uint = 0x0810;
pub const TSNEP_GCL_A: c_uint = 0x2000;
pub const TSNEP_GCL_B: c_uint = 0x2800;

pub const TSNEP_RX_ASSIGN: c_uint = 0x0840;
pub const TSNEP_RX_ASSIGN_ACTIVE: c_uint = 0x00000001;
pub const TSNEP_RX_ASSIGN_QUEUE_MASK: c_uint = 0x00000006;
pub const TSNEP_RX_ASSIGN_QUEUE_SHIFT: c_int = 1;
pub const TSNEP_RX_ASSIGN_OFFSET: c_int = 1;
pub const TSNEP_RX_ASSIGN_ETHER_TYPE: c_uint = 0x0880;
pub const TSNEP_RX_ASSIGN_ETHER_TYPE_OFFSET: c_int = 2;
pub const TSNEP_RX_ASSIGN_ETHER_TYPE_COUNT: c_int = 2;
// tsnep gate control list operation
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsnep_gcl_operation {
    pub properties: u32,
    pub interval: u32,
}

pub const TSNEP_GCL_MASK: c_uint = 0x000000FF;
pub const TSNEP_GCL_INSERT: c_uint = 0x20000000;
pub const TSNEP_GCL_CHANGE: c_uint = 0x40000000;
pub const TSNEP_GCL_LAST: c_uint = 0x80000000;
pub const TSNEP_GCL_MIN_INTERVAL: c_int = 32;
// tsnep TX/RX descriptor
pub const TSNEP_DESC_SIZE: c_int = 256;
pub const TSNEP_DESC_SIZE_DATA_AFTER: c_int = 2048;
pub const TSNEP_DESC_OFFSET: c_int = 128;

pub const TSNEP_DESC_OWNER_COUNTER_MASK: c_uint = 0xC0000000;
pub const TSNEP_DESC_OWNER_COUNTER_SHIFT: c_int = 30;
pub const TSNEP_DESC_LENGTH_MASK: c_uint = 0x00003FFF;
pub const TSNEP_DESC_INTERRUPT_FLAG: c_uint = 0x00040000;
pub const TSNEP_DESC_EXTENDED_WRITEBACK_FLAG: c_uint = 0x00080000;
pub const TSNEP_DESC_NO_LINK_FLAG: c_uint = 0x01000000;
// tsnep TX descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsnep_tx_desc {
    pub properties: __le32,
    pub more_properties: __le32,
    pub reserved: [__le32; 2],
    pub next: __le64,
    pub tx: __le64,
}

pub const TSNEP_TX_DESC_OWNER_MASK: c_uint = 0xE0000000;
pub const TSNEP_TX_DESC_OWNER_USER_FLAG: c_uint = 0x20000000;
pub const TSNEP_TX_DESC_LAST_FRAGMENT_FLAG: c_uint = 0x00010000;
pub const TSNEP_TX_DESC_DATA_AFTER_DESC_FLAG: c_uint = 0x00020000;
// tsnep TX descriptor writeback
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsnep_tx_desc_wb {
    pub properties: __le32,
    pub reserved1: __le32,
    pub counter: __le64,
    pub timestamp: __le64,
    pub dma_delay: __le32,
    pub reserved2: __le32,
}

pub const TSNEP_TX_DESC_UNDERRUN_ERROR_FLAG: c_uint = 0x00010000;
pub const TSNEP_TX_DESC_DMA_DELAY_FIRST_DATA_MASK: c_uint = 0x0000FFFC;
pub const TSNEP_TX_DESC_DMA_DELAY_FIRST_DATA_SHIFT: c_int = 2;
pub const TSNEP_TX_DESC_DMA_DELAY_LAST_DATA_MASK: c_uint = 0xFFFC0000;
pub const TSNEP_TX_DESC_DMA_DELAY_LAST_DATA_SHIFT: c_int = 18;
pub const TSNEP_TX_DESC_DMA_DELAY_NS: c_int = 64;
// tsnep RX descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsnep_rx_desc {
    pub properties: __le32,
    pub reserved: [__le32; 3],
    pub next: __le64,
    pub rx: __le64,
}

pub const TSNEP_RX_DESC_BUFFER_SIZE_MASK: c_uint = 0x00003FFC;
// tsnep RX descriptor writeback
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsnep_rx_desc_wb {
    pub properties: __le32,
    pub reserved: [__le32; 7],
}

// tsnep RX inline meta
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsnep_rx_inline {
    pub counter: __le64,
    pub timestamp: __le64,
}

