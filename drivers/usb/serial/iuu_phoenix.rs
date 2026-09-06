//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/serial/iuu_phoenix.h
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
// Infinity Unlimited USB Phoenix driver
//
// Copyright (C) 2007 Alain Degreffe (eczema@ecze.com)
//
// Original code taken from iuutool ( Copyright (C) 2006 Juan Carlos Borrás )
//
// And tested with help of WB Electronics
//
pub const IUU_USB_VENDOR_ID: c_uint = 0x104f;
pub const IUU_USB_PRODUCT_ID: c_uint = 0x0004;
pub const IUU_USB_OP_TIMEOUT: c_uint = 0x0200;
// Programmer commands
pub const IUU_NO_OPERATION: c_uint = 0x00;
pub const IUU_GET_FIRMWARE_VERSION: c_uint = 0x01;
pub const IUU_GET_PRODUCT_NAME: c_uint = 0x02;
pub const IUU_GET_STATE_REGISTER: c_uint = 0x03;
pub const IUU_SET_LED: c_uint = 0x04;
pub const IUU_WAIT_MUS: c_uint = 0x05;
pub const IUU_WAIT_MS: c_uint = 0x06;
pub const IUU_GET_LOADER_VERSION: c_uint = 0x50;
pub const IUU_RST_SET: c_uint = 0x52;
pub const IUU_RST_CLEAR: c_uint = 0x53;
pub const IUU_SET_VCC: c_uint = 0x59;
pub const IUU_UART_ENABLE: c_uint = 0x49;
pub const IUU_UART_DISABLE: c_uint = 0x4A;
pub const IUU_UART_WRITE_I2C: c_uint = 0x4C;
pub const IUU_UART_ESC: c_uint = 0x5E;
pub const IUU_UART_TRAP: c_uint = 0x54;
pub const IUU_UART_TRAP_BREAK: c_uint = 0x5B;
pub const IUU_UART_RX: c_uint = 0x56;
pub const IUU_AVR_ON: c_uint = 0x21;
pub const IUU_AVR_OFF: c_uint = 0x22;
pub const IUU_AVR_1CLK: c_uint = 0x23;
pub const IUU_AVR_RESET: c_uint = 0x24;
pub const IUU_AVR_RESET_PC: c_uint = 0x25;
pub const IUU_AVR_INC_PC: c_uint = 0x26;
pub const IUU_AVR_INCN_PC: c_uint = 0x27;
pub const IUU_AVR_PREAD: c_uint = 0x29;
pub const IUU_AVR_PREADN: c_uint = 0x2A;
pub const IUU_AVR_PWRITE: c_uint = 0x28;
pub const IUU_AVR_DREAD: c_uint = 0x2C;
pub const IUU_AVR_DREADN: c_uint = 0x2D;
pub const IUU_AVR_DWRITE: c_uint = 0x2B;
pub const IUU_AVR_PWRITEN: c_uint = 0x2E;
pub const IUU_EEPROM_ON: c_uint = 0x37;
pub const IUU_EEPROM_OFF: c_uint = 0x38;
pub const IUU_EEPROM_WRITE: c_uint = 0x39;
pub const IUU_EEPROM_WRITEX: c_uint = 0x3A;
pub const IUU_EEPROM_WRITE8: c_uint = 0x3B;
pub const IUU_EEPROM_WRITE16: c_uint = 0x3C;
pub const IUU_EEPROM_WRITEX32: c_uint = 0x3D;
pub const IUU_EEPROM_WRITEX64: c_uint = 0x3E;
pub const IUU_EEPROM_READ: c_uint = 0x3F;
pub const IUU_EEPROM_READX: c_uint = 0x40;
pub const IUU_EEPROM_BREAD: c_uint = 0x41;
pub const IUU_EEPROM_BREADX: c_uint = 0x42;
pub const IUU_PIC_CMD: c_uint = 0x0A;
pub const IUU_PIC_CMD_LOAD: c_uint = 0x0B;
pub const IUU_PIC_CMD_READ: c_uint = 0x0C;
pub const IUU_PIC_ON: c_uint = 0x0D;
pub const IUU_PIC_OFF: c_uint = 0x0E;
pub const IUU_PIC_RESET: c_uint = 0x16;
pub const IUU_PIC_INC_PC: c_uint = 0x0F;
pub const IUU_PIC_INCN_PC: c_uint = 0x10;
pub const IUU_PIC_PWRITE: c_uint = 0x11;
pub const IUU_PIC_PREAD: c_uint = 0x12;
pub const IUU_PIC_PREADN: c_uint = 0x13;
pub const IUU_PIC_DWRITE: c_uint = 0x14;
pub const IUU_PIC_DREAD: c_uint = 0x15;
pub const IUU_UART_NOP: c_uint = 0x00;
pub const IUU_UART_CHANGE: c_uint = 0x02;
pub const IUU_UART_TX: c_uint = 0x04;
pub const IUU_DELAY_MS: c_uint = 0x06;
pub const IUU_OPERATION_OK: c_uint = 0x00;
pub const IUU_DEVICE_NOT_FOUND: c_uint = 0x01;
pub const IUU_INVALID_HANDLE: c_uint = 0x02;
pub const IUU_INVALID_PARAMETER: c_uint = 0x03;
pub const IUU_INVALID_voidERFACE: c_uint = 0x04;
pub const IUU_INVALID_REQUEST_LENGTH: c_uint = 0x05;
pub const IUU_UART_NOT_ENABLED: c_uint = 0x06;
pub const IUU_WRITE_ERROR: c_uint = 0x07;
pub const IUU_READ_ERROR: c_uint = 0x08;
pub const IUU_TX_ERROR: c_uint = 0x09;
pub const IUU_RX_ERROR: c_uint = 0x0A;
pub const IUU_PARITY_NONE: c_uint = 0x00;
pub const IUU_PARITY_EVEN: c_uint = 0x01;
pub const IUU_PARITY_ODD: c_uint = 0x02;
pub const IUU_PARITY_MARK: c_uint = 0x03;
pub const IUU_PARITY_SPACE: c_uint = 0x04;
pub const IUU_SC_INSERTED: c_uint = 0x01;
pub const IUU_VERIFY_ERROR: c_uint = 0x02;
pub const IUU_SIM_INSERTED: c_uint = 0x04;
pub const IUU_TWO_STOP_BITS: c_uint = 0x00;
pub const IUU_ONE_STOP_BIT: c_uint = 0x20;
pub const IUU_BAUD_2400: c_uint = 0x0398;
pub const IUU_BAUD_9600: c_uint = 0x0298;
pub const IUU_BAUD_19200: c_uint = 0x0164;
pub const IUU_BAUD_28800: c_uint = 0x0198;
pub const IUU_BAUD_38400: c_uint = 0x01B2;
pub const IUU_BAUD_57600: c_uint = 0x0030;
pub const IUU_BAUD_115200: c_uint = 0x0098;
pub const IUU_CLK_3579000: c_int = 3579000;
pub const IUU_CLK_3680000: c_int = 3680000;
pub const IUU_CLK_6000000: c_int = 6000000;
pub const IUU_FULLCARD_IN: c_uint = 0x01;
pub const IUU_DEV_ERROR: c_uint = 0x02;
pub const IUU_MINICARD_IN: c_uint = 0x04;
pub const IUU_VCC_5V: c_uint = 0x00;
pub const IUU_VCC_3V: c_uint = 0x01;
