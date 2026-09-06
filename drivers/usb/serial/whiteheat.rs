//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/serial/whiteheat.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// USB ConnectTech WhiteHEAT driver
//
// Copyright (C) 2002
// Connect Tech Inc.
//
// Copyright (C) 1999, 2000
// Greg Kroah-Hartman (greg@kroah.com)
//
// See Documentation/usb/usb-serial.rst for more information on using this
// driver
//
// WhiteHEAT commands

//
// Commands to the firmware
//
// WHITEHEAT_OPEN
// WHITEHEAT_CLOSE
// WHITEHEAT_STATUS
// WHITEHEAT_GET_DTR_RTS
// WHITEHEAT_REPORT_TX_DONE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct whiteheat_simple {
    pub /: *mut *mut __u8 port; / port number (1 to N),
}

//
// WHITEHEAT_SETUP_PORT
//

pub const WHITEHEAT_HFLOW_NONE: c_uint = 0x00	/* no hardware flow control */;
pub const WHITEHEAT_HFLOW_RTS_TOGGLE: c_uint = 0x01	/* RTS is on during transmit,;
pub const WHITEHEAT_HFLOW_DTR: c_uint = 0x02	/* DTR is off/on when RX;
pub const WHITEHEAT_HFLOW_CTS: c_uint = 0x08	/* when received CTS off/on;
pub const WHITEHEAT_HFLOW_DSR: c_uint = 0x10	/* when received DSR off/on;
pub const WHITEHEAT_HFLOW_RTS: c_uint = 0x80	/* RTS is off/on when RX;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct whiteheat_port_settings {
    pub /: *mut *mut __u8 port; / port number (1 to N),
    pub calculates: *mut *mut __le32 baud; / any value 7 - 460800, firmware,
    pub /: *mut best fit; arrives little endian,
    pub /: *mut *mut __u8 bits; / 5, 6, 7, or 8,
    pub /: *mut *mut __u8 stop; / 1 or 2, default 1 (2 = 1.5 if bits = 5),
    pub /: *mut *mut *mut __u8 parity; / see WHITEHEAT_PAR_ above,
    pub /: *mut *mut *mut __u8 sflow; / see WHITEHEAT_SFLOW_ above,
    pub /: *mut *mut __u8 xoff; / XOFF byte value,
    pub /: *mut *mut __u8 xon; / XON byte value,
    pub /: *mut *mut *mut __u8 hflow; / see WHITEHEAT_HFLOW_ above,
    pub /: *mut *mut __u8 lloop; / 0/1 turns local loopback mode off/on,
// C attribute field omitted
//
// WHITEHEAT_SET_RTS
// WHITEHEAT_SET_DTR
// WHITEHEAT_SET_BREAK
//
pub const WHITEHEAT_RTS_OFF: c_uint = 0x00;
pub const WHITEHEAT_RTS_ON: c_uint = 0x01;
pub const WHITEHEAT_DTR_OFF: c_uint = 0x00;
pub const WHITEHEAT_DTR_ON: c_uint = 0x01;
pub const WHITEHEAT_BREAK_OFF: c_uint = 0x00;
pub const WHITEHEAT_BREAK_ON: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct whiteheat_set_rdb {
    pub /: *mut *mut __u8 port; / port number (1 to N),
    pub /: *mut *mut __u8 state; / 0/1 turns signal off/on,
}

//
// WHITEHEAT_DUMP
//

//
// Allowable address ranges (firmware checks address):
// Type DATA:  0x00 - 0xff
// Type IDATA: 0x80 - 0xff
// Type BDATA: 0x20 - 0x2f
// Type XDATA: 0x0000 - 0xffff
//
// B/I/DATA all read the local memory space
// XDATA reads the external memory space
// BDATA returns bits as bytes
//
// NOTE: 0x80 - 0xff (local space) are the Special Function Registers
// of the 8051, and some have on-read side-effects.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct whiteheat_dump {
    pub /: *mut *mut *mut __u8 mem_type; / see WHITEHEAT_DUMP_ above,
    pub /: *mut *mut __u16 addr; / address, see restrictions above,
    pub /: *mut *mut __u16 length; / number of bytes to dump, max 63 bytes,
}

//
// WHITEHEAT_PURGE
//
pub const WHITEHEAT_PURGE_RX: c_uint = 0x01	/* purge rx fifos */;
pub const WHITEHEAT_PURGE_TX: c_uint = 0x02	/* purge tx fifos */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct whiteheat_purge {
    pub /: *mut *mut __u8 port; / port number (1 to N),
    pub /: *mut *mut __u8 what; / bit pattern of what to purge,
}

//
// WHITEHEAT_ECHO
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct whiteheat_echo {
    pub /: *mut *mut __u8 port; / port number (1 to N),
    pub /: *mut *mut __u8 length; / length of message to echo, max 61 bytes,
    pub /: *mut *mut __u8 echo_data[61]; / data to echo,
}

//
// WHITEHEAT_DO_TEST
//
pub const WHITEHEAT_TEST_UART_RW: c_uint = 0x01  /* read/write uart registers */;
pub const WHITEHEAT_TEST_UART_INTR: c_uint = 0x02  /* uart interrupt */;
pub const WHITEHEAT_TEST_SETUP_CONT: c_uint = 0x03  /* setup for;
pub const WHITEHEAT_TEST_PORT_CONT: c_uint = 0x04  /* port connect */;
pub const WHITEHEAT_TEST_PORT_DISCONT: c_uint = 0x05  /* port disconnect */;
pub const WHITEHEAT_TEST_UART_CLK_START: c_uint = 0x06  /* uart clock test start */;
pub const WHITEHEAT_TEST_UART_CLK_STOP: c_uint = 0x07  /* uart clock test stop */;
pub const WHITEHEAT_TEST_MODEM_FT: c_uint = 0x08  /* modem signals, requires a;
pub const WHITEHEAT_TEST_ERASE_EEPROM: c_uint = 0x09  /* erase eeprom */;
pub const WHITEHEAT_TEST_READ_EEPROM: c_uint = 0x0a  /* read eeprom */;
pub const WHITEHEAT_TEST_PROGRAM_EEPROM: c_uint = 0x0b  /* program eeprom */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct whiteheat_test {
    pub /: *mut *mut __u8 port; / port number (1 to n),
    pub above*/: *mut *mut *mut __u8 test; / see WHITEHEAT_TEST_,
    pub /: *mut *mut __u8 info[32]; / additional info,
}

//
// Replies from the firmware
//
// WHITEHEAT_STATUS
//
pub const WHITEHEAT_EVENT_MODEM: c_uint = 0x01	/* modem field is valid */;
pub const WHITEHEAT_EVENT_ERROR: c_uint = 0x02	/* error field is valid */;
pub const WHITEHEAT_EVENT_FLOW: c_uint = 0x04	/* flow field is valid */;
pub const WHITEHEAT_EVENT_CONNECT: c_uint = 0x08	/* connect field is valid */;
pub const WHITEHEAT_FLOW_NONE: c_uint = 0x00	/* no flow control active */;
pub const WHITEHEAT_FLOW_HARD_OUT: c_uint = 0x01	/* TX is stopped by CTS;
pub const WHITEHEAT_FLOW_HARD_IN: c_uint = 0x02	/* remote TX is stopped;
pub const WHITEHEAT_FLOW_SOFT_OUT: c_uint = 0x04	/* TX is stopped by XOFF;
pub const WHITEHEAT_FLOW_SOFT_IN: c_uint = 0x08	/* remote TX is stopped by XOFF;
pub const WHITEHEAT_FLOW_TX_DONE: c_uint = 0x80	/* TX has completed */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct whiteheat_status_info {
    pub /: *mut *mut __u8 port; / port number (1 to N),
    pub is,: *mut *mut __u8 event; / indicates what the current event,
    pub uart's: *mut *mut __u8 modem; / modem signal status (copy of,
    pub /: *mut *mut __u8 error; / line status (copy of uart's LSR register),
    pub WHITEHEAT_FLOW_*: *mut *mut __u8 flow; / flow control state, see,
    pub means: *mut *mut __u8 connect; / 0 means not connected, non-zero,
}

//
// WHITEHEAT_GET_DTR_RTS
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct whiteheat_dr_info {
    pub /: *mut *mut __u8 mcr; / copy of uart's MCR register,
}

//
// WHITEHEAT_GET_HW_INFO
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct whiteheat_hw_info {
    pub /: *mut *mut __u8 hw_id; / hardware id number, WhiteHEAT = 0,
    pub /: *mut *mut __u8 sw_major_rev; / major version number,
    pub /: *mut *mut __u8 sw_minor_rev; / minor version number,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct whiteheat_hw_eeprom_info {
    pub /: *mut *mut __u8 b0; / B0,
    pub /: *mut *mut __u8 vendor_id_low; / vendor id (low byte),
    pub /: *mut *mut __u8 vendor_id_high; / vendor id (high byte),
    pub /: *mut *mut __u8 product_id_low; / product id (low byte),
    pub /: *mut *mut __u8 product_id_high; / product id (high byte),
    pub /: *mut *mut __u8 device_id_low; / device id (low byte),
    pub /: *mut *mut __u8 device_id_high; / device id (high byte),
    pub not_used_1: __u8,
    pub /: *mut *mut __u8 serial_number_0; / serial number (low byte),
    pub /: *mut *mut __u8 serial_number_1; / serial number,
    pub /: *mut *mut __u8 serial_number_2; / serial number,
    pub /: *mut *mut __u8 serial_number_3; / serial number (high byte),
    pub not_used_2: __u8,
    pub not_used_3: __u8,
    pub /: *mut *mut __u8 checksum_low; / checksum (low byte),
    pub /: *mut *mut __u8 checksum_high; / checksum (high byte,
    pub /: *mut *mut } hw_eeprom_info; / EEPROM contents,
}

//
// WHITEHEAT_EVENT
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct whiteheat_event_info {
    pub /: *mut *mut __u8 port; / port number (1 to N),
    pub /: *mut *mut __u8 event; / see whiteheat_status_info.event,
    pub .error,: *mut *mut __u8 info; / see whiteheat_status_info.modem,,
}

//
// WHITEHEAT_DO_TEST
//
pub const WHITEHEAT_TEST_FAIL: c_uint = 0x00  /* test failed */;
pub const WHITEHEAT_TEST_UNKNOWN: c_uint = 0x01  /* unknown test requested */;
pub const WHITEHEAT_TEST_PASS: c_uint = 0xff  /* test passed */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct whiteheat_test_info {
    pub /: *mut *mut __u8 port; / port number (1 to N),
    pub for,: *mut *mut __u8 test; / indicates which test this is a response,
    pub /: *mut *mut *mut __u8 status; / see WHITEHEAT_TEST_ above,
    pub /: *mut *mut __u8 results[32]; / test-dependent results,
}
