//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/serial.h
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


// SPDX-License-Identifier: GPL-1.0+ WITH Linux-syscall-note
//
// include/linux/serial.h
//
// Copyright (C) 1992 by Theodore Ts'o.
//
// Redistribution of this file is permitted under the terms of the GNU
// Public License (GPL)
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct serial_struct {
    pub type: c_int,
    pub line: c_int,
    pub port: c_uint,
    pub irq: c_int,
    pub flags: c_int,
    pub xmit_fifo_size: c_int,
    pub custom_divisor: c_int,
    pub baud_base: c_int,
    pub close_delay: c_ushort,
    pub io_type: c_char,
    pub reserved_char: [c_char; 1],
    pub hub6: c_int,
    pub /: *mut *mut unsigned short closing_wait; / time to wait before closing,
    pub /: *mut *mut unsigned short closing_wait2; / no longer used...,
    pub iomem_base: *mut c_uchar,
    pub iomem_reg_shift: c_ushort,
    pub port_high: c_uint,
    pub /: *mut *mut unsigned long iomap_base; / cookie passed into ioremap,
}

//
// For the close wait times, 0 means wait forever for serial port to
// flush its output.  65535 means don't wait at all.
//
pub const ASYNC_CLOSING_WAIT_INF: c_int = 0;
pub const ASYNC_CLOSING_WAIT_NONE: c_int = 65535;
//
// These are the supported serial types.
//
pub const PORT_UNKNOWN: c_int = 0;
pub const PORT_8250: c_int = 1;
pub const PORT_16450: c_int = 2;
pub const PORT_16550: c_int = 3;
pub const PORT_16550A: c_int = 4;
pub const PORT_CIRRUS: c_int = 5;
pub const PORT_16650: c_int = 6;
pub const PORT_16650V2: c_int = 7;
pub const PORT_16750: c_int = 8;
pub const PORT_STARTECH: c_int = 9;

pub const PORT_16654: c_int = 11;
pub const PORT_16850: c_int = 12;

pub const PORT_MAX: c_int = 13;
pub const SERIAL_IO_PORT: c_int = 0;
pub const SERIAL_IO_HUB6: c_int = 1;
pub const SERIAL_IO_MEM: c_int = 2;
pub const SERIAL_IO_MEM32: c_int = 3;
pub const SERIAL_IO_AU: c_int = 4;
pub const SERIAL_IO_TSI: c_int = 5;
pub const SERIAL_IO_MEM32BE: c_int = 6;
pub const SERIAL_IO_MEM16: c_int = 7;
pub const SERIAL_IO_BUS: c_int = 8;
pub const UART_CLEAR_FIFO: c_uint = 0x01;
pub const UART_USE_FIFO: c_uint = 0x02;
pub const UART_STARTECH: c_uint = 0x04;
pub const UART_NATSEMI: c_uint = 0x08;
//
// Multiport serial configuration structure --- external structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct serial_multiport_struct {
    pub irq: c_int,
    pub port1: c_int,
    pub match1: unsigned char mask1,,
    pub port2: c_int,
    pub match2: unsigned char mask2,,
    pub port3: c_int,
    pub match3: unsigned char mask3,,
    pub port4: c_int,
    pub match4: unsigned char mask4,,
    pub port_monitor: c_int,
    pub reserved: [c_int; 32],
}

//
// Serial input interrupt line counters -- external structure
// Four lines can interrupt: CTS, DSR, RI, DCD
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct serial_icounter_struct {
    pub dcd: int cts, dsr, rng,,
    pub tx: int rx,,
    pub brk: int frame, overrun, parity,,
    pub buf_overrun: c_int,
    pub reserved: [c_int; 9],
}

//
// struct serial_rs485 - serial interface for controlling RS485 settings.
// @flags:			RS485 feature flags.
// @delay_rts_before_send:	Delay before send (milliseconds).
// @delay_rts_after_send:	Delay after send (milliseconds).
// @addr_recv:			Receive filter for RS485 addressing mode
// (used only when %SER_RS485_ADDR_RECV is set).
// @addr_dest:			Destination address for RS485 addressing mode
// (used only when %SER_RS485_ADDR_DEST is set).
// @padding0:			Padding (set to zero).
// @padding1:			Padding (set to zero).
// @padding:			Deprecated, use @padding0 and @padding1 instead.
// Do not use with @addr_recv and @addr_dest (due to
// overlap).
//
// Serial interface for controlling RS485 settings on chips with suitable
// support. Set with TIOCSRS485 and get with TIOCGRS485 if supported by your
// platform. The set function returns the new state, with any unsupported bits
// reverted appropriately.
//
// The flag bits are:
//
// * %SER_RS485_ENABLED		- RS485 enabled.
// * %SER_RS485_RTS_ON_SEND	- Logical level for RTS pin when sending.
// * %SER_RS485_RTS_AFTER_SEND	- Logical level for RTS pin after sent.
// * %SER_RS485_RX_DURING_TX	- Full-duplex RS485 line.
// * %SER_RS485_TERMINATE_BUS	- Enable bus termination (if supported).
// * %SER_RS485_ADDRB		- Enable RS485 addressing mode.
// * %SER_RS485_ADDR_RECV - Receive address filter (enables @addr_recv). Requires %SER_RS485_ADDRB.
// * %SER_RS485_ADDR_DEST - Destination address (enables @addr_dest). Requires %SER_RS485_ADDRB.
// * %SER_RS485_MODE_RS422	- Enable RS422. Requires %SER_RS485_ENABLED.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct serial_rs485 {
    pub flags: __u32,

// Placeholder for bit 3: SER_RS485_RTS_BEFORE_SEND, which isn't used anymore

    pub delay_rts_before_send: __u32,
    pub delay_rts_after_send: __u32,
// The fields below are defined by flags
    pub /: *mut *mut __u32 padding[5]; / Memory is cheap, new structs are a pain,
    pub addr_recv: __u8,
    pub addr_dest: __u8,
    pub padding0: [__u8; 2],
    pub padding1: [__u32; 4],
}

//
// Serial interface for controlling ISO7816 settings on chips with suitable
// support. Set with TIOCSISO7816 and get with TIOCGISO7816 if supported by
// your platform.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct serial_iso7816 {
    pub /: *mut *mut __u32 flags; / ISO7816 feature flags,

    pub tg: __u32,
    pub sc_fi: __u32,
    pub sc_di: __u32,
    pub clk: __u32,
    pub reserved: [__u32; 5],
}
