//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ieee802154/atusb.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// atusb.h - Definitions shared between kernel and ATUSB firmware
//
// Written 2013 by Werner Almesberger <werner@almesberger.net>
//
// (at your option) any later version.
//
// This file should be identical for kernel and firmware.
// Kernel: drivers/net/ieee802154/atusb.h
// Firmware: ben-wpan/atusb/fw/include/atusb/atusb.h
//
pub const ATUSB_VENDOR_ID: c_uint = 0x20b7	/* Qi Hardware*/;
pub const ATUSB_PRODUCT_ID: c_uint = 0x1540	/* 802.15.4, device 0 */;
// -- -         -

// Commands to our device. Make sure this is synced with the firmware
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum atusb_requests {
    ATUSB_ID			= 0x00,	/* system status/control grp */
    ATUSB_BUILD,
    ATUSB_RESET,
    ATUSB_RF_RESET			= 0x10,	/* debug/test group */
    ATUSB_POLL_INT,
    ATUSB_TEST,			/* atusb-sil only */
    ATUSB_TIMER,
    ATUSB_GPIO,
    ATUSB_SLP_TR,
    ATUSB_GPIO_CLEANUP,
    ATUSB_REG_WRITE			= 0x20,	/* transceiver group */
    ATUSB_REG_READ,
    ATUSB_BUF_WRITE,
    ATUSB_BUF_READ,
    ATUSB_SRAM_WRITE,
    ATUSB_SRAM_READ,
    ATUSB_SPI_WRITE			= 0x30,	/* SPI group */
    ATUSB_SPI_READ1,
    ATUSB_SPI_READ2,
    ATUSB_SPI_WRITE2_SYNC,
    ATUSB_RX_MODE			= 0x40, /* HardMAC group */
    ATUSB_TX,
    ATUSB_EUI64_WRITE		= 0x50, /* Parameter in EEPROM grp */
    ATUSB_EUI64_READ,
}

//
// Direction	bRequest		wValue		wIndex	wLength
//
// ->host	ATUSB_ID		-		-	3
// ->host	ATUSB_BUILD		-		-	#bytes
// host->	ATUSB_RESET		-		-	0
//
// host->	ATUSB_RF_RESET		-		-	0
// ->host	ATUSB_POLL_INT		-		-	1
// host->	ATUSB_TEST		-		-	0
// ->host	ATUSB_TIMER		-		-	#bytes (6)
// ->host	ATUSB_GPIO		dir+data	mask+p#	3
// host->	ATUSB_SLP_TR		-		-	0
// host->	ATUSB_GPIO_CLEANUP	-		-	0
//
// host->	ATUSB_REG_WRITE		value		addr	0
// ->host	ATUSB_REG_READ		-		addr	1
// host->	ATUSB_BUF_WRITE		-		-	#bytes
// ->host	ATUSB_BUF_READ		-		-	#bytes
// host->	ATUSB_SRAM_WRITE	-		addr	#bytes
// ->host	ATUSB_SRAM_READ		-		addr	#bytes
//
// host->	ATUSB_SPI_WRITE		byte0		byte1	#bytes
// ->host	ATUSB_SPI_READ1		byte0		-	#bytes
// ->host	ATUSB_SPI_READ2		byte0		byte1	#bytes
// ->host	ATUSB_SPI_WRITE2_SYNC	byte0		byte1	0/1
//
// host->	ATUSB_RX_MODE		on		-	0
// host->	ATUSB_TX		flags		ack_seq	#bytes
// host->	ATUSB_EUI64_WRITE	-		-	#bytes (8)
// ->host	ATUSB_EUI64_READ	-		-	#bytes (8)
//

