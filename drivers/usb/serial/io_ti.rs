//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/serial/io_ti.h
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
// Copyright (C) 1997-2002 Inside Out Networks, Inc.
//
// Feb-16-2001	DMI	Added I2C structure definitions
// May-29-2002	gkh	Ported to Linux
//
// Address Space
pub const DTK_ADDR_SPACE_XDATA: c_uint = 0x03	/* Addr is placed in XDATA space */;
pub const DTK_ADDR_SPACE_I2C_TYPE_II: c_uint = 0x82	/* Addr is placed in I2C area */;
pub const DTK_ADDR_SPACE_I2C_TYPE_III: c_uint = 0x83	/* Addr is placed in I2C area */;
// UART Defines
pub const UMPMEM_BASE_UART1: c_uint = 0xFFA0	/* UMP UART1 base address */;
pub const UMPMEM_BASE_UART2: c_uint = 0xFFB0	/* UMP UART2 base address */;
pub const UMPMEM_OFFS_UART_LSR: c_uint = 0x05	/* UMP UART LSR register offset */;
// Bits per character
pub const UMP_UART_CHAR5BITS: c_uint = 0x00;
pub const UMP_UART_CHAR6BITS: c_uint = 0x01;
pub const UMP_UART_CHAR7BITS: c_uint = 0x02;
pub const UMP_UART_CHAR8BITS: c_uint = 0x03;
// Parity
pub const UMP_UART_NOPARITY: c_uint = 0x00;
pub const UMP_UART_ODDPARITY: c_uint = 0x01;
pub const UMP_UART_EVENPARITY: c_uint = 0x02;
pub const UMP_UART_MARKPARITY: c_uint = 0x03;
pub const UMP_UART_SPACEPARITY: c_uint = 0x04;
// Stop bits
pub const UMP_UART_STOPBIT1: c_uint = 0x00;
pub const UMP_UART_STOPBIT15: c_uint = 0x01;
pub const UMP_UART_STOPBIT2: c_uint = 0x02;
// Line status register masks
pub const UMP_UART_LSR_OV_MASK: c_uint = 0x01;
pub const UMP_UART_LSR_PE_MASK: c_uint = 0x02;
pub const UMP_UART_LSR_FE_MASK: c_uint = 0x04;
pub const UMP_UART_LSR_BR_MASK: c_uint = 0x08;
pub const UMP_UART_LSR_ER_MASK: c_uint = 0x0F;
pub const UMP_UART_LSR_RX_MASK: c_uint = 0x10;
pub const UMP_UART_LSR_TX_MASK: c_uint = 0x20;

// Port Settings Constants)
pub const UMP_MASK_UART_FLAGS_RTS_FLOW: c_uint = 0x0001;
pub const UMP_MASK_UART_FLAGS_RTS_DISABLE: c_uint = 0x0002;
pub const UMP_MASK_UART_FLAGS_PARITY: c_uint = 0x0008;
pub const UMP_MASK_UART_FLAGS_OUT_X_DSR_FLOW: c_uint = 0x0010;
pub const UMP_MASK_UART_FLAGS_OUT_X_CTS_FLOW: c_uint = 0x0020;
pub const UMP_MASK_UART_FLAGS_OUT_X: c_uint = 0x0040;
pub const UMP_MASK_UART_FLAGS_OUT_XA: c_uint = 0x0080;
pub const UMP_MASK_UART_FLAGS_IN_X: c_uint = 0x0100;
pub const UMP_MASK_UART_FLAGS_DTR_FLOW: c_uint = 0x0800;
pub const UMP_MASK_UART_FLAGS_DTR_DISABLE: c_uint = 0x1000;
pub const UMP_MASK_UART_FLAGS_RECEIVE_MS_INT: c_uint = 0x2000;
pub const UMP_MASK_UART_FLAGS_AUTO_START_ON_ERR: c_uint = 0x4000;
pub const UMP_DMA_MODE_CONTINOUS: c_uint = 0x01;
pub const UMP_PIPE_TRANS_TIMEOUT_ENA: c_uint = 0x80;
pub const UMP_PIPE_TRANSFER_MODE_MASK: c_uint = 0x03;
pub const UMP_PIPE_TRANS_TIMEOUT_MASK: c_uint = 0x7C;
// Purge port Direction Mask Bits
pub const UMP_PORT_DIR_OUT: c_uint = 0x01;
pub const UMP_PORT_DIR_IN: c_uint = 0x02;
// Address of Port 0
pub const UMPM_UART1_PORT: c_uint = 0x03;
// Commands
pub const UMPC_SET_CONFIG: c_uint = 0x05;
pub const UMPC_OPEN_PORT: c_uint = 0x06;
pub const UMPC_CLOSE_PORT: c_uint = 0x07;
pub const UMPC_START_PORT: c_uint = 0x08;
pub const UMPC_STOP_PORT: c_uint = 0x09;
pub const UMPC_TEST_PORT: c_uint = 0x0A;
pub const UMPC_PURGE_PORT: c_uint = 0x0B;
// Force the Firmware to complete the current Read
pub const UMPC_COMPLETE_READ: c_uint = 0x80;
// Force UMP back into BOOT Mode
pub const UMPC_HARDWARE_RESET: c_uint = 0x81;
//
// Copy current download image to type 0xf2 record in 16k I2C
// firmware will change 0xff record to type 2 record when complete
//
pub const UMPC_COPY_DNLD_TO_I2C: c_uint = 0x82;
//
// Special function register commands
// wIndex is register address
// wValue is MSB/LSB mask/data
//
pub const UMPC_WRITE_SFR: c_uint = 0x83	/* Write SFR Register */;
// wIndex is register address
pub const UMPC_READ_SFR: c_uint = 0x84	/* Read SRF Register */;
// Set or Clear DTR (wValue bit 0 Set/Clear)	wIndex ModuleID (port)
pub const UMPC_SET_CLR_DTR: c_uint = 0x85;
// Set or Clear RTS (wValue bit 0 Set/Clear)	wIndex ModuleID (port)
pub const UMPC_SET_CLR_RTS: c_uint = 0x86;
// Set or Clear LOOPBACK (wValue bit 0 Set/Clear) wIndex ModuleID (port)
pub const UMPC_SET_CLR_LOOPBACK: c_uint = 0x87;
// Set or Clear BREAK (wValue bit 0 Set/Clear)	wIndex ModuleID (port)
pub const UMPC_SET_CLR_BREAK: c_uint = 0x88;
// Read MSR wIndex ModuleID (port)
pub const UMPC_READ_MSR: c_uint = 0x89;
// Toolkit commands
// Read-write group
pub const UMPC_MEMORY_READ: c_uint = 0x92;
pub const UMPC_MEMORY_WRITE: c_uint = 0x93;
//
// UMP DMA Definitions
//
pub const UMPD_OEDB1_ADDRESS: c_uint = 0xFF08;
pub const UMPD_OEDB2_ADDRESS: c_uint = 0xFF10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct out_endpoint_desc_block {
    pub Configuration: u8,
    pub XBufAddr: u8,
    pub XByteCount: u8,
    pub Unused1: u8,
    pub Unused2: u8,
    pub YBufAddr: u8,
    pub YByteCount: u8,
    pub BufferSize: u8,
}

//
// TYPE DEFINITIONS
// Structures for Firmware commands
//
// UART settings
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ump_uart_config {
    pub /: *mut *mut u16 wBaudRate; / Baud rate,
    pub /: *mut *mut u16 wFlags; / Bitmap mask of flags,
    pub /: *mut *mut u8 bDataBits; / 5..8 - data bits per character,
    pub /: *mut *mut u8 bParity; / Parity settings,
    pub /: *mut *mut u8 bStopBits; / Stop bits settings,
    pub /: *mut *mut char cXon; / XON character,
    pub /: *mut *mut char cXoff; / XOFF character,
    pub /: *mut *mut u8 bUartMode; / Will be updated when a user,
// interface is defined
}

//
// TYPE DEFINITIONS
// Structures for USB interrupts
//
// Interrupt packet structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ump_interrupt {
    pub /: *mut *mut u8 bICode; / Interrupt code (interrupt num),
    pub /: *mut *mut u8 bIInfo; / Interrupt information,
}

pub const TIUMP_INTERRUPT_CODE_LSR: c_uint = 0x03;
pub const TIUMP_INTERRUPT_CODE_MSR: c_uint = 0x04;
