//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mfd/ipaq-micro.h
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
// Header file for the compaq Micro MFD
//

pub const TX_BUF_SIZE: c_int = 32;
pub const RX_BUF_SIZE: c_int = 16;
pub const CHAR_SOF: c_uint = 0x02;
//
// These are the different messages that can be sent to the microcontroller
// to control various aspects.
//
pub const MSG_VERSION: c_uint = 0x0;
pub const MSG_KEYBOARD: c_uint = 0x2;
pub const MSG_TOUCHSCREEN: c_uint = 0x3;
pub const MSG_EEPROM_READ: c_uint = 0x4;
pub const MSG_EEPROM_WRITE: c_uint = 0x5;
pub const MSG_THERMAL_SENSOR: c_uint = 0x6;
pub const MSG_NOTIFY_LED: c_uint = 0x8;
pub const MSG_BATTERY: c_uint = 0x9;
pub const MSG_SPI_READ: c_uint = 0xb;
pub const MSG_SPI_WRITE: c_uint = 0xc;
pub const MSG_BACKLIGHT: c_uint = 0xd /* H3600 only */;
pub const MSG_CODEC_CTRL: c_uint = 0xe /* H3100 only */;
pub const MSG_DISPLAY_CTRL: c_uint = 0xf /* H3100 only */;
// state of receiver parser
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rx_state {
    STATE_SOF = 0,     /* Next byte should be start of frame */
    STATE_ID,          /* Next byte is ID & message length   */
    STATE_DATA,        /* Next byte is a data byte           */
    STATE_CHKSUM       /* Next byte should be checksum       */
}

//
// struct ipaq_micro_txdev - TX state
// @len: length of message in TX buffer
// @index: current index into TX buffer
// @buf: TX buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipaq_micro_txdev {
    pub len: u8,
    pub index: u8,
    pub buf: [u8; TX_BUF_SIZE],
}

//
// struct ipaq_micro_rxdev - RX state
// @state: context of RX state machine
// @chksum: calculated checksum
// @id: message ID from packet
// @len: RX buffer length
// @index: RX buffer index
// @buf: RX buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipaq_micro_rxdev {
    pub state: rx_state,
    pub chksum: c_uchar,
    pub id: u8,
    pub len: c_uint,
    pub index: c_uint,
    pub buf: [u8; RX_BUF_SIZE],
}

//
// struct ipaq_micro_msg - message to the iPAQ microcontroller
// @id: 4-bit ID of the message
// @tx_len: length of TX data
// @tx_data: TX data to send
// @rx_len: length of received RX data
// @rx_data: RX data to receive
// @ack: a completion that will be completed when RX is complete
// @node: list node if message gets queued
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipaq_micro_msg {
    pub id: u8,
    pub tx_len: u8,
    pub tx_data: [u8; TX_BUF_SIZE],
    pub rx_len: u8,
    pub rx_data: [u8; RX_BUF_SIZE],
    pub ack: completion,
    pub node: list_head,
}

//
// struct ipaq_micro - iPAQ microcontroller state
// @dev: corresponding platform device
// @base: virtual memory base for underlying serial device
// @sdlc: virtual memory base for Synchronous Data Link Controller
// @version: version string
// @tx: TX state
// @rx: RX state
// @lock: lock for this state container
// @msg: current message
// @queue: message queue
// @key: callback for asynchronous key events
// @key_data: data to pass along with key events
// @ts: callback for asynchronous touchscreen events
// @ts_data: data to pass along with key events
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipaq_micro {
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub sdlc: *mut void __iomem,
    pub version: [c_char; 5],
    pub /: *mut *mut ipaq_micro_txdev tx; / transmit ISR state,
    pub /: *mut *mut ipaq_micro_rxdev rx; / receive ISR state,
    pub lock: spinlock_t,
    pub msg: *mut ipaq_micro_msg,
    pub queue: list_head,
    pub rxdata): *mut *mut *mut void (key) (void data, int len, unsigned char,
    pub key_data: *mut c_void,
    pub rxdata): *mut *mut *mut void (ts) (void data, int len, unsigned char,
    pub ts_data: *mut c_void,
}

extern "C" {
    pub fn ipaq_micro_tx_msg(_arg: micro, _arg: msg) -> return;
}
