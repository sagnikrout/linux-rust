//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/marvell/libertas/if_spi.h
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
// linux/drivers/net/wireless/libertas/if_spi.c
//
// Driver for Marvell SPI WLAN cards.
//
// Copyright 2008 Analog Devices Inc.
//
// Authors:
// Andrey Yurovsky <andrey@cozybit.com>
// Colin McCabe <colin@cozybit.com>
//
pub const IPFIELD_ALIGN_OFFSET: c_int = 2;
pub const IF_SPI_CMD_BUF_SIZE: c_int = 2400;
// Firmware
pub const IF_SPI_FW_NAME_MAX: c_int = 30;
pub const MAX_MAIN_FW_LOAD_CRC_ERR: c_int = 10;
// Chunk size when loading the helper firmware
pub const HELPER_FW_LOAD_CHUNK_SZ: c_int = 64;
// Value to write to indicate end of helper firmware dnld
pub const FIRMWARE_DNLD_OK: c_uint = 0x0000;
// Value to check once the main firmware is downloaded
pub const SUCCESSFUL_FW_DOWNLOAD_MAGIC: c_uint = 0x88888888;
// SPI Interface Unit
// Masks used in SPI register read/write operations
pub const IF_SPI_READ_OPERATION_MASK: c_uint = 0x0;
pub const IF_SPI_WRITE_OPERATION_MASK: c_uint = 0x8000;
// SPI register offsets. 4-byte aligned.
pub const IF_SPI_DEVICEID_CTRL_REG: c_uint = 0x00	/* DeviceID controller reg */;
pub const IF_SPI_IO_READBASE_REG: c_uint = 0x04 	/* Read I/O base reg */;
pub const IF_SPI_IO_WRITEBASE_REG: c_uint = 0x08	/* Write I/O base reg */;
pub const IF_SPI_IO_RDWRPORT_REG: c_uint = 0x0C	/* Read/Write I/O port reg */;
pub const IF_SPI_CMD_READBASE_REG: c_uint = 0x10	/* Read command base reg */;
pub const IF_SPI_CMD_WRITEBASE_REG: c_uint = 0x14	/* Write command base reg */;
pub const IF_SPI_CMD_RDWRPORT_REG: c_uint = 0x18	/* Read/Write command port reg */;
pub const IF_SPI_DATA_READBASE_REG: c_uint = 0x1C	/* Read data base reg */;
pub const IF_SPI_DATA_WRITEBASE_REG: c_uint = 0x20	/* Write data base reg */;
pub const IF_SPI_DATA_RDWRPORT_REG: c_uint = 0x24	/* Read/Write data port reg */;
pub const IF_SPI_SCRATCH_1_REG: c_uint = 0x28	/* Scratch reg 1 */;
pub const IF_SPI_SCRATCH_2_REG: c_uint = 0x2C	/* Scratch reg 2 */;
pub const IF_SPI_SCRATCH_3_REG: c_uint = 0x30	/* Scratch reg 3 */;
pub const IF_SPI_SCRATCH_4_REG: c_uint = 0x34	/* Scratch reg 4 */;
pub const IF_SPI_TX_FRAME_SEQ_NUM_REG: c_uint = 0x38 /* Tx frame sequence number reg */;
pub const IF_SPI_TX_FRAME_STATUS_REG: c_uint = 0x3C	/* Tx frame status reg */;
pub const IF_SPI_HOST_INT_CTRL_REG: c_uint = 0x40	/* Host interrupt controller reg */;
pub const IF_SPI_CARD_INT_CAUSE_REG: c_uint = 0x44	/* Card interrupt cause reg */;
pub const IF_SPI_CARD_INT_STATUS_REG: c_uint = 0x48 /* Card interrupt status reg */;
pub const IF_SPI_CARD_INT_EVENT_MASK_REG: c_uint = 0x4C /* Card interrupt event mask */;
pub const IF_SPI_CARD_INT_STATUS_MASK_REG: c_uint = 0x50 /* Card interrupt status mask */;
pub const IF_SPI_CARD_INT_RESET_SELECT_REG: c_uint = 0x54 /* Card interrupt reset select */;
pub const IF_SPI_HOST_INT_CAUSE_REG: c_uint = 0x58	/* Host interrupt cause reg */;
pub const IF_SPI_HOST_INT_STATUS_REG: c_uint = 0x5C	/* Host interrupt status reg */;
pub const IF_SPI_HOST_INT_EVENT_MASK_REG: c_uint = 0x60 /* Host interrupt event mask */;
pub const IF_SPI_HOST_INT_STATUS_MASK_REG: c_uint = 0x64 /* Host interrupt status mask */;
pub const IF_SPI_HOST_INT_RESET_SELECT_REG: c_uint = 0x68 /* Host interrupt reset select */;
pub const IF_SPI_DELAY_READ_REG: c_uint = 0x6C	/* Delay read reg */;
pub const IF_SPI_SPU_BUS_MODE_REG: c_uint = 0x70	/* SPU BUS mode reg */;
// IF_SPI_DEVICEID_CTRL_REG

// IF_SPI_HOST_INT_CTRL_REG
// Host Interrupt Control bit : Wake up

// Host Interrupt Control bit : WLAN ready

// #define IF_SPI_HICT_FIFO_FIRST_HALF_EMPTY		(1<<2)
// #define IF_SPI_HICT_FIFO_SECOND_HALF_EMPTY		(1<<3)
// #define IF_SPI_HICT_IRQSRC_WLAN			(1<<4)
// Host Interrupt Control bit : Tx auto download

// Host Interrupt Control bit : Rx auto upload

// Host Interrupt Control bit : Command auto download

// Host Interrupt Control bit : Command auto upload

// IF_SPI_CARD_INT_CAUSE_REG
// Card Interrupt Case bit : Tx download over

// Card Interrupt Case bit : Rx upload over

// Card Interrupt Case bit : Command download over

// Card Interrupt Case bit : Host event

// Card Interrupt Case bit : Command upload over

// Card Interrupt Case bit : Power down

// IF_SPI_CARD_INT_STATUS_REG

// IF_SPI_HOST_INT_CAUSE_REG

// IF_SPI_HOST_INT_STATUS_REG
// Host Interrupt Status bit : Tx download ready

// Host Interrupt Status bit : Rx upload ready

// Host Interrupt Status bit : Command download ready

// Host Interrupt Status bit : Card event

// Host Interrupt Status bit : Command upload ready

// Host Interrupt Status bit : I/O write FIFO overflow

// Host Interrupt Status bit : I/O read FIFO underflow

// Host Interrupt Status bit : Data write FIFO overflow

// Host Interrupt Status bit : Data read FIFO underflow

// Host Interrupt Status bit : Command write FIFO overflow

// Host Interrupt Status bit : Command read FIFO underflow

// IF_SPI_HOST_INT_STATUS_MASK_REG
// Host Interrupt Status Mask bit : Tx download ready

// Host Interrupt Status Mask bit : Rx upload ready

// Host Interrupt Status Mask bit : Command download ready

// Host Interrupt Status Mask bit : Card event

// Host Interrupt Status Mask bit : Command upload ready

// Host Interrupt Status Mask bit : I/O write FIFO overflow

// Host Interrupt Status Mask bit : I/O read FIFO underflow

// Host Interrupt Status Mask bit : Data write FIFO overflow

// Host Interrupt Status Mask bit : Data write FIFO underflow

// Host Interrupt Status Mask bit : Command write FIFO overflow

// Host Interrupt Status Mask bit : Command write FIFO underflow

// IF_SPI_SPU_BUS_MODE_REG
// SCK edge on which the WLAN module outputs data on MISO
pub const IF_SPI_BUS_MODE_SPI_CLOCK_PHASE_FALLING: c_uint = 0x8;
pub const IF_SPI_BUS_MODE_SPI_CLOCK_PHASE_RISING: c_uint = 0x0;
// In a SPU read operation, there is a delay between writing the SPU
// register name and getting back data from the WLAN module.
// This can be specified in terms of nanoseconds or in terms of dummy
// clock cycles which the master must output before receiving a response.
pub const IF_SPI_BUS_MODE_DELAY_METHOD_DUMMY_CLOCK: c_uint = 0x4;
pub const IF_SPI_BUS_MODE_DELAY_METHOD_TIMED: c_uint = 0x0;
// Some different modes of SPI operation
pub const IF_SPI_BUS_MODE_8_BIT_ADDRESS_16_BIT_DATA: c_uint = 0x00;
pub const IF_SPI_BUS_MODE_8_BIT_ADDRESS_32_BIT_DATA: c_uint = 0x01;
pub const IF_SPI_BUS_MODE_16_BIT_ADDRESS_16_BIT_DATA: c_uint = 0x02;
pub const IF_SPI_BUS_MODE_16_BIT_ADDRESS_32_BIT_DATA: c_uint = 0x03;
