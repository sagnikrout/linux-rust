//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/rsi/rsi_sdio.h
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


//
// @section LICENSE
// Copyright (c) 2014 Redpine Signals Inc.
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sdio_interrupt_type {
    BUFFER_FULL         = 0x0,
    BUFFER_AVAILABLE    = 0x2,
    FIRMWARE_ASSERT_IND = 0x3,
    MSDU_PACKET_PENDING = 0x4,
    UNKNOWN_INT         = 0XE
}

// Buffer status register related info
pub const PKT_BUFF_SEMI_FULL: c_int = 0;
pub const PKT_BUFF_FULL: c_int = 1;
pub const PKT_MGMT_BUFF_FULL: c_int = 2;
pub const MSDU_PKT_PENDING: c_int = 3;
pub const RECV_NUM_BLOCKS: c_int = 4;
// Interrupt Bit Related Macros
pub const PKT_BUFF_AVAILABLE: c_int = 1;
pub const FW_ASSERT_IND: c_int = 2;
pub const RSI_MASTER_REG_BUF_SIZE: c_int = 12;
pub const RSI_DEVICE_BUFFER_STATUS_REGISTER: c_uint = 0xf3;
pub const RSI_FN1_INT_REGISTER: c_uint = 0xf9;
pub const RSI_INT_ENABLE_REGISTER: c_uint = 0x04;
pub const RSI_INT_ENABLE_MASK: c_uint = 0xfc;
pub const RSI_SD_REQUEST_MASTER: c_uint = 0x10000;
// FOR SD CARD ONLY
pub const SDIO_RX_NUM_BLOCKS_REG: c_uint = 0x000F1;
pub const SDIO_FW_STATUS_REG: c_uint = 0x000F2;
pub const SDIO_NXT_RD_DELAY2: c_uint = 0x000F5;
pub const SDIO_MASTER_ACCESS_MSBYTE: c_uint = 0x000FA;
pub const SDIO_MASTER_ACCESS_LSBYTE: c_uint = 0x000FB;
pub const SDIO_READ_START_LVL: c_uint = 0x000FC;
pub const SDIO_READ_FIFO_CTL: c_uint = 0x000FD;
pub const SDIO_WRITE_FIFO_CTL: c_uint = 0x000FE;
pub const SDIO_WAKEUP_REG: c_uint = 0x000FF;
pub const SDIO_FUN1_INTR_CLR_REG: c_uint = 0x0008;
pub const SDIO_REG_HIGH_SPEED: c_uint = 0x0013;

// common registers in SDIO function1
pub const TA_SOFT_RESET_REG: c_uint = 0x0004;
pub const TA_TH0_PC_REG: c_uint = 0x0400;
pub const TA_HOLD_THREAD_REG: c_uint = 0x0844;
pub const TA_RELEASE_THREAD_REG: c_uint = 0x0848;
pub const TA_SOFT_RST_CLR: c_int = 0;

pub const TA_PC_ZERO: c_int = 0;
pub const TA_HOLD_THREAD_VALUE: c_uint = 0xF;
pub const TA_RELEASE_THREAD_VALUE: c_uint = 0xF;
pub const TA_BASE_ADDR: c_uint = 0x2200;
pub const MISC_CFG_BASE_ADDR: c_uint = 0x4105;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct receive_info {
    pub buffer_full: bool,
    pub semi_buffer_full: bool,
    pub mgmt_buffer_full: bool,
    pub mgmt_buf_full_counter: u32,
    pub buf_semi_full_counter: u32,
    pub watch_bufferfull_count: u8,
    pub sdio_intr_status_zero: u32,
    pub sdio_int_counter: u32,
    pub total_sdio_msdu_pending_intr: u32,
    pub total_sdio_unknown_intr: u32,
    pub buf_full_counter: u32,
    pub buf_available_counter: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsi_91x_sdiodev {
    pub pfunction: *mut sdio_func,
    pub sdio_irq_task: *mut task_struct,
    pub rx_info: receive_info,
    pub next_read_delay: u32,
    pub sdio_high_speed_enable: u32,
    pub sdio_clock_speed: u8,
    pub cardcapability: u32,
    pub prev_desc: [u8; 16],
    pub tx_blk_size: u16,
    pub write_fail: u8,
    pub buff_status_updated: bool,
    pub rx_thread: rsi_thread,
    pub __aligned(4): u8 pktbuffer[8192],
}

extern "C" {
    pub fn rsi_init_sdio_slave_regs(adapter: *mut rsi_hw) -> c_int;
}
extern "C" {
    pub fn rsi_sdio_read_register(adapter: *mut rsi_hw, addr: u32, data: *mut u8) -> c_int;
}
extern "C" {
    pub fn rsi_sdio_host_intf_read_pkt(adapter: *mut rsi_hw, pkt: *mut u8, length: u32) -> c_int;
}
extern "C" {
    pub fn rsi_sdio_master_access_msword(adapter: *mut rsi_hw, ms_word: u16) -> c_int;
}
extern "C" {
    pub fn rsi_sdio_ack_intr(adapter: *mut rsi_hw, int_bit: u8);
}
extern "C" {
    pub fn rsi_sdio_determine_event_timeout(adapter: *mut rsi_hw) -> c_int;
}
extern "C" {
    pub fn rsi_sdio_check_buffer_status(adapter: *mut rsi_hw, q_num: u8) -> c_int;
}
extern "C" {
    pub fn rsi_sdio_rx_thread(data: *mut c_void) -> c_int;
}
