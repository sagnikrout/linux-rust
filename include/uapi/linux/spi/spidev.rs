//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/spi/spidev.h
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
// include/linux/spi/spidev.h
//
// Copyright (C) 2006 SWAPP
// Andrea Paterniani <a.paterniani@swapp-eng.it>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 675 Mass Ave, Cambridge, MA 02139, USA.
//

// IOCTL commands

//
// struct spi_ioc_transfer - describes a single SPI transfer
// @tx_buf: Holds pointer to userspace buffer with transmit data, or null.
// If no data is provided, zeroes are shifted out.
// @rx_buf: Holds pointer to userspace buffer for receive data, or null.
// @len: Length of tx and rx buffers, in bytes.
// @speed_hz: Temporary override of the device's bitrate.
// @bits_per_word: Temporary override of the device's wordsize.
// @delay_usecs: If nonzero, how long to delay after the last bit transfer
// before optionally deselecting the device before the next transfer.
// @cs_change: True to deselect device before starting the next transfer.
// @word_delay_usecs: If nonzero, how long to wait between words within one
// transfer. This property needs explicit support in the SPI controller,
// otherwise it is silently ignored.
//
// This structure is mapped directly to the kernel spi_transfer structure;
// the fields have the same meanings, except of course that the pointers
// are in a different address space (and may be of different sizes in some
// cases, such as 32-bit i386 userspace over a 64-bit x86_64 kernel).
// Zero-initialize the structure, including currently unused fields, to
// accommodate potential future updates.
//
// SPI_IOC_MESSAGE gives userspace the equivalent of kernel spi_sync().
// Pass it an array of related transfers, they'll execute together.
// Each transfer may be half duplex (either direction) or full duplex.
//
// struct spi_ioc_transfer mesg[4];
// ...
// status = ioctl(fd, SPI_IOC_MESSAGE(4), mesg);
//
// So for example one transfer might send a nine bit command (right aligned
// in a 16-bit word), the next could read a block of 8-bit data before
// terminating that command by temporarily deselecting the chip; the next
// could send a different nine bit command (re-selecting the chip), and the
// last transfer might write some register values.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_ioc_transfer {
    pub tx_buf: __u64,
    pub rx_buf: __u64,
    pub len: __u32,
    pub speed_hz: __u32,
    pub delay_usecs: __u16,
    pub bits_per_word: __u8,
    pub cs_change: __u8,
    pub tx_nbits: __u8,
    pub rx_nbits: __u8,
    pub word_delay_usecs: __u8,
    pub pad: __u8,
// If the contents of 'struct spi_ioc_transfer' ever change
// incompatibly, then the ioctl number (currently 0) must change;
// ioctls with constant size fields get a bit more in the way of
// error checking than ones (like this) where that field varies.
//
// NOTE: struct layout is the same in 64bit and 32bit userspace.
//
}

// not all platforms use <asm-generic/ioctl.h> or _IOC_TYPECHECK() ...

// Read / Write of SPI mode (SPI_MODE_0..SPI_MODE_3) (limited to 8 bits)

// Read / Write SPI bit justification

// Read / Write SPI device word length (1..N)

// Read / Write SPI device default max speed hz

// Read / Write of the SPI mode field

