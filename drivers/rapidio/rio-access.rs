//! Automatically rewritten from C to Rust
//! Source: drivers/rapidio/rio-access.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// RapidIO configuration space access support
//
// Copyright 2005 MontaVista Software, Inc.
// Matt Porter <mporter@kernel.crashing.org>
//

//
// Wrappers for all RIO configuration access functions.  They just check
// alignment and call the low-level functions pointed to by rio_mport->ops.
//
pub const RIO_8_BAD: c_int = 0;

//
// RIO_LOP_READ - Generate rio_local_read_config_* functions
// @size: Size of configuration space read (8, 16, 32 bits)
// @type: C type of value argument
// @len: Length of configuration space read (1, 2, 4 bytes)
//
// Generates rio_local_read_config_* functions used to access
// configuration space registers on the local device.
//

    int __rio_local_read_config_##size \
    (struct rio_mport *mport, u32 offset, type *value)		\
    {									\
    int res;							\
    u32 data = 0;							\
    if (RIO_##size##_BAD) return RIO_BAD_SIZE;			\
    res = mport.ops.lcread(mport, mport.id, offset, len, &data);	\
// value = (type)data;						\
    return res;							\
    }
//
// RIO_LOP_WRITE - Generate rio_local_write_config_* functions
// @size: Size of configuration space write (8, 16, 32 bits)
// @type: C type of value argument
// @len: Length of configuration space write (1, 2, 4 bytes)
//
// Generates rio_local_write_config_* functions used to access
// configuration space registers on the local device.
//

    int __rio_local_write_config_##size \
    (struct rio_mport *mport, u32 offset, type value)		\
    {									\
    if (RIO_##size##_BAD) return RIO_BAD_SIZE;			\
    return mport.ops.lcwrite(mport, mport.id, offset, len, value);\
    }
    RIO_LOP_READ(8, u8, 1)
    RIO_LOP_READ(16, u16, 2)
    RIO_LOP_READ(32, u32, 4)
    RIO_LOP_WRITE(8, u8, 1)
    RIO_LOP_WRITE(16, u16, 2)
    RIO_LOP_WRITE(32, u32, 4)
    EXPORT_SYMBOL_GPL(__rio_local_read_config_8);
    EXPORT_SYMBOL_GPL(__rio_local_read_config_16);
    EXPORT_SYMBOL_GPL(__rio_local_read_config_32);
    EXPORT_SYMBOL_GPL(__rio_local_write_config_8);
    EXPORT_SYMBOL_GPL(__rio_local_write_config_16);
    EXPORT_SYMBOL_GPL(__rio_local_write_config_32);
//
// RIO_OP_READ - Generate rio_mport_read_config_* functions
// @size: Size of configuration space read (8, 16, 32 bits)
// @type: C type of value argument
// @len: Length of configuration space read (1, 2, 4 bytes)
//
// Generates rio_mport_read_config_* functions used to access
// configuration space registers on the local device.
//

    int rio_mport_read_config_##size \
    (struct rio_mport *mport, u16 destid, u8 hopcount, u32 offset, type *value)	\
    {									\
    int res;							\
    u32 data = 0;							\
    if (RIO_##size##_BAD) return RIO_BAD_SIZE;			\
    res = mport.ops.cread(mport, mport.id, destid, hopcount, offset, len, &data); \
// value = (type)data;						\
    return res;							\
    }
//
// RIO_OP_WRITE - Generate rio_mport_write_config_* functions
// @size: Size of configuration space write (8, 16, 32 bits)
// @type: C type of value argument
// @len: Length of configuration space write (1, 2, 4 bytes)
//
// Generates rio_mport_write_config_* functions used to access
// configuration space registers on the local device.
//

    int rio_mport_write_config_##size \
    (struct rio_mport *mport, u16 destid, u8 hopcount, u32 offset, type value)	\
    {									\
    if (RIO_##size##_BAD) return RIO_BAD_SIZE;			\
    return mport.ops.cwrite(mport, mport.id, destid, hopcount,	\
    offset, len, value);				\
    }
    RIO_OP_READ(8, u8, 1)
    RIO_OP_READ(16, u16, 2)
    RIO_OP_READ(32, u32, 4)
    RIO_OP_WRITE(8, u8, 1)
    RIO_OP_WRITE(16, u16, 2)
    RIO_OP_WRITE(32, u32, 4)
    EXPORT_SYMBOL_GPL(rio_mport_read_config_8);
    EXPORT_SYMBOL_GPL(rio_mport_read_config_16);
    EXPORT_SYMBOL_GPL(rio_mport_read_config_32);
    EXPORT_SYMBOL_GPL(rio_mport_write_config_8);
    EXPORT_SYMBOL_GPL(rio_mport_write_config_16);
    EXPORT_SYMBOL_GPL(rio_mport_write_config_32);
//
// rio_mport_send_doorbell - Send a doorbell message
//
// @mport: RIO master port
// @destid: RIO device destination ID
// @data: Doorbell message data
//
// Send a doorbell message to a RIO device. The doorbell message
// has a 16-bit info field provided by the data argument.
//
#[no_mangle]
pub unsafe extern "C" fn rio_mport_send_doorbell(mport: *mut rio_mport, destid: u16, data: u16) -> c_int {
    int rio_mport_send_doorbell(struct rio_mport *mport, u16 destid, u16 data)
    {
    return mport.ops.dsend(mport, mport.id, destid, data);
    }
    EXPORT_SYMBOL_GPL(rio_mport_send_doorbell);
