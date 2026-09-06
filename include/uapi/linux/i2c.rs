//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/i2c.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// i2c.h - definitions for the I2C bus interface
//
// Copyright (C) 1995-2000 Simon G. Vogl
// With some changes from Kyösti Mälkki <kmalkki@cc.hut.fi> and
// Frodo Looijaard <frodol@dds.nl>
//

//
// struct i2c_msg - an I2C transaction segment beginning with START
//
// @addr: Slave address, either 7 or 10 bits. When this is a 10 bit address,
// %I2C_M_TEN must be set in @flags and the adapter must support
// %I2C_FUNC_10BIT_ADDR.
//
// @flags:
// Supported by all adapters:
// %I2C_M_RD: read data (from slave to master). Guaranteed to be 0x0001! If
// not set, the transaction is interpreted as write.
//
// Optional:
// %I2C_M_DMA_SAFE: the buffer of this message is DMA safe. Makes only sense
// in kernelspace, because userspace buffers are copied anyway
//
// Only if I2C_FUNC_10BIT_ADDR is set:
// %I2C_M_TEN: this is a 10 bit chip address
//
// Only if I2C_FUNC_SMBUS_READ_BLOCK_DATA is set:
// %I2C_M_RECV_LEN: message length will be first received byte
//
// Only if I2C_FUNC_NOSTART is set:
// %I2C_M_NOSTART: skip repeated start sequence
//
// Only if I2C_FUNC_PROTOCOL_MANGLING is set:
// %I2C_M_NO_RD_ACK: in a read message, master ACK/NACK bit is skipped
// %I2C_M_IGNORE_NAK: treat NACK from client as ACK
// %I2C_M_REV_DIR_ADDR: toggles the Rd/Wr bit
// %I2C_M_STOP: force a STOP condition after the message
//
// @len: Number of data bytes in @buf being read from or written to the I2C
// slave address. For read transactions where %I2C_M_RECV_LEN is set, the
// caller guarantees that this buffer can hold up to %I2C_SMBUS_BLOCK_MAX
// bytes in addition to the initial length byte sent by the slave (plus,
// if used, the SMBus PEC); and this value will be incremented by the number
// of block data bytes received.
//
// @buf: The buffer into which data is read, or from which it's written.
//
// An i2c_msg is the low level representation of one segment of an I2C
// transaction.  It is visible to drivers in the @i2c_transfer() procedure,
// to userspace from i2c-dev, and to I2C adapter drivers through the
// @i2c_adapter.@master_xfer() method.
//
// Except when I2C "protocol mangling" is used, all I2C adapters implement
// the standard rules for I2C transactions.  Each transaction begins with a
// START.  That is followed by the slave address, and a bit encoding read
// versus write.  Then follow all the data bytes, possibly including a byte
// with SMBus PEC.  The transfer terminates with a NAK, or when all those
// bytes have been transferred and ACKed.  If this is the last message in a
// group, it is followed by a STOP.  Otherwise it is followed by the next
// @i2c_msg transaction segment, beginning with a (repeated) START.
//
// Alternatively, when the adapter supports %I2C_FUNC_PROTOCOL_MANGLING then
// passing certain @flags may have changed those standard protocol behaviors.
// Those flags are only for use with broken/nonconforming slaves, and with
// adapters which are known to support the specific mangling options they need.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_msg {
    pub addr: __u16,
    pub flags: __u16,
pub const I2C_M_RD: c_uint = 0x0001	/* guaranteed to be 0x0001! */;
pub const I2C_M_TEN: c_uint = 0x0010	/* use only if I2C_FUNC_10BIT_ADDR */;
pub const I2C_M_DMA_SAFE: c_uint = 0x0200	/* use only in kernel space */;
pub const I2C_M_RECV_LEN: c_uint = 0x0400	/* use only if I2C_FUNC_SMBUS_READ_BLOCK_DATA */;
pub const I2C_M_NO_RD_ACK: c_uint = 0x0800	/* use only if I2C_FUNC_PROTOCOL_MANGLING */;
pub const I2C_M_IGNORE_NAK: c_uint = 0x1000	/* use only if I2C_FUNC_PROTOCOL_MANGLING */;
pub const I2C_M_REV_DIR_ADDR: c_uint = 0x2000	/* use only if I2C_FUNC_PROTOCOL_MANGLING */;
pub const I2C_M_NOSTART: c_uint = 0x4000	/* use only if I2C_FUNC_NOSTART */;
pub const I2C_M_STOP: c_uint = 0x8000	/* use only if I2C_FUNC_PROTOCOL_MANGLING */;
    pub len: __u16,
    pub buf: *mut __u8,
}

// To determine what functionality is present
pub const I2C_FUNC_I2C: c_uint = 0x00000001;
pub const I2C_FUNC_10BIT_ADDR: c_uint = 0x00000002 /* required for I2C_M_TEN */;
pub const I2C_FUNC_PROTOCOL_MANGLING: c_uint = 0x00000004 /* required for I2C_M_IGNORE_NAK etc. */;
pub const I2C_FUNC_SMBUS_PEC: c_uint = 0x00000008;
pub const I2C_FUNC_NOSTART: c_uint = 0x00000010 /* required for I2C_M_NOSTART */;
pub const I2C_FUNC_SLAVE: c_uint = 0x00000020;
pub const I2C_FUNC_SMBUS_BLOCK_PROC_CALL: c_uint = 0x00008000 /* SMBus 2.0 or later */;
pub const I2C_FUNC_SMBUS_QUICK: c_uint = 0x00010000;
pub const I2C_FUNC_SMBUS_READ_BYTE: c_uint = 0x00020000;
pub const I2C_FUNC_SMBUS_WRITE_BYTE: c_uint = 0x00040000;
pub const I2C_FUNC_SMBUS_READ_BYTE_DATA: c_uint = 0x00080000;
pub const I2C_FUNC_SMBUS_WRITE_BYTE_DATA: c_uint = 0x00100000;
pub const I2C_FUNC_SMBUS_READ_WORD_DATA: c_uint = 0x00200000;
pub const I2C_FUNC_SMBUS_WRITE_WORD_DATA: c_uint = 0x00400000;
pub const I2C_FUNC_SMBUS_PROC_CALL: c_uint = 0x00800000;
pub const I2C_FUNC_SMBUS_READ_BLOCK_DATA: c_uint = 0x01000000 /* required for I2C_M_RECV_LEN */;
pub const I2C_FUNC_SMBUS_WRITE_BLOCK_DATA: c_uint = 0x02000000;
pub const I2C_FUNC_SMBUS_READ_I2C_BLOCK: c_uint = 0x04000000 /* I2C-like block xfer  */;
pub const I2C_FUNC_SMBUS_WRITE_I2C_BLOCK: c_uint = 0x08000000 /* w/ 1-byte reg. addr. */;
pub const I2C_FUNC_SMBUS_HOST_NOTIFY: c_uint = 0x10000000 /* SMBus 2.0 or later */;

// if I2C_M_RECV_LEN is also supported

//
// Data for SMBus Messages
//

#[repr(C)]
#[derive(Copy, Clone)]
pub union i2c_smbus_data {
    pub byte: __u8,
    pub word: __u16,
    pub /: *mut *mut __u8 block[I2C_SMBUS_BLOCK_MAX + 2]; / block[0] is used for length,
// and one more for user-space compatibility
}

// i2c_smbus_xfer read or write markers
pub const I2C_SMBUS_READ: c_int = 1;
pub const I2C_SMBUS_WRITE: c_int = 0;
// SMBus transaction types (size parameter in the above functions)
pub const I2C_SMBUS_QUICK: c_int = 0;
pub const I2C_SMBUS_BYTE: c_int = 1;
pub const I2C_SMBUS_BYTE_DATA: c_int = 2;
pub const I2C_SMBUS_WORD_DATA: c_int = 3;
pub const I2C_SMBUS_PROC_CALL: c_int = 4;
pub const I2C_SMBUS_BLOCK_DATA: c_int = 5;
pub const I2C_SMBUS_I2C_BLOCK_BROKEN: c_int = 6;

pub const I2C_SMBUS_I2C_BLOCK_DATA: c_int = 8;
