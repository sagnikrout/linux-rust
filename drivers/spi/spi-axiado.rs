//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/spi/spi-axiado.h
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
// Axiado SPI controller driver (Host mode only)
//
// Copyright (C) 2022-2025 Axiado Corporation (or its affiliates).
//
// Name of this driver

// Axiado - SPI Digital Blocks IP design registers
pub const AX_SPI_TX_FAETR: c_uint = 0x18    // TX-FAETR;
pub const ALMOST_EMPTY_TRESHOLD: c_uint = 0x00	// Programmed threshold value;
pub const AX_SPI_RX_FAFTR: c_uint = 0x28    // RX-FAETR;
pub const ALMOST_FULL_TRESHOLD: c_uint = 0x0c	// Programmed threshold value;

pub const AX_SPI_CR1: c_uint = 0x00	// CR1;
pub const AX_SPI_CR1_CLR: c_uint = 0x00	// CR1 - Clear;
pub const AX_SPI_CR1_SCR: c_uint = 0x01	// CR1 - controller reset;
pub const AX_SPI_CR1_SCE: c_uint = 0x02	// CR1 - Controller Enable/Disable;
pub const AX_SPI_CR1_CPHA: c_uint = 0x08	// CR1 - CPH;
pub const AX_SPI_CR1_CPOL: c_uint = 0x10	// CR1 - CPO;
pub const AX_SPI_CR2: c_uint = 0x04	// CR2;
pub const AX_SPI_CR2_SWD: c_uint = 0x04	// CR2 - Write Enabel/Disable;
pub const AX_SPI_CR2_SRD: c_uint = 0x08	// CR2 - Read Enable/Disable;
pub const AX_SPI_CR2_SRI: c_uint = 0x10	// CR2 - Read First Byte Ignore;
pub const AX_SPI_CR2_HTE: c_uint = 0x40	// CR2 - Host Transmit Enable;
pub const AX_SPI_CR3: c_uint = 0x08	// CR3;
pub const AX_SPI_CR3_SDL: c_uint = 0x00	// CR3 - Data lines;
pub const AX_SPI_CR3_QUAD: c_uint = 0x02	// CR3 - Data lines;
// As per Digital Blocks datasheet clock frequency range
// Min - 244KHz
// Max - 62.5MHz
// SCK Clock Divider Register Values
//
pub const AX_SPI_RX_FBCAR: c_uint = 0x24	// RX_FBCAR;
pub const AX_SPI_TX_FBCAR: c_uint = 0x14	// TX_FBCAR;
pub const AX_SPI_SCDR: c_uint = 0x2c	// SCDR;
pub const AX_SPI_SCD_MIN: c_uint = 0x1fe	// Valid SCD (SCK Clock Divider Register);
pub const AX_SPI_SCD_DEFAULT: c_uint = 0x06	// Default SCD (SCK Clock Divider Register);
pub const AX_SPI_SCD_MAX: c_uint = 0x00	// Valid SCD (SCK Clock Divider Register);
pub const AX_SPI_SCDR_SCS: c_uint = 0x0200	// SCDR - AMBA Bus Clock source;
pub const AX_SPI_IMR: c_uint = 0x34	// IMR;
pub const AX_SPI_IMR_CLR: c_uint = 0x00	// IMR - Clear;
pub const AX_SPI_IMR_TFOM: c_uint = 0x02	// IMR - TFO;
pub const AX_SPI_IMR_MTCM: c_uint = 0x40	// IMR - MTC;
pub const AX_SPI_IMR_TFEM: c_uint = 0x10	// IMR - TFE;
pub const AX_SPI_IMR_RFFM: c_uint = 0x20	// IMR - RFFM;
pub const AX_SPI_ISR: c_uint = 0x30	// ISR;
pub const AX_SPI_ISR_CLR: c_uint = 0xff	// ISR - Clear;
pub const AX_SPI_ISR_MTC: c_uint = 0x40	// ISR - MTC;
pub const AX_SPI_ISR_TFE: c_uint = 0x10	// ISR - TFE;
pub const AX_SPI_ISR_RFF: c_uint = 0x20	// ISR - RFF;
pub const AX_SPI_IVR: c_uint = 0x38	// IVR;
pub const AX_SPI_IVR_TFOV: c_uint = 0x02	// IVR - TFOV;
pub const AX_SPI_IVR_MTCV: c_uint = 0x40	// IVR - MTCV;
pub const AX_SPI_IVR_TFEV: c_uint = 0x10	// IVR - TFEV;
pub const AX_SPI_IVR_RFFV: c_uint = 0x20	// IVR - RFFV;
pub const AX_SPI_TXFIFO: c_uint = 0x0c	// TX_FIFO;
pub const AX_SPI_TX_RX_FBCR: c_uint = 0x10	// TX_RX_FBCR;
pub const AX_SPI_RXFIFO: c_uint = 0x1c	// RX_FIFO;
pub const AX_SPI_TS0: c_uint = 0x00	// Target select 0;
pub const AX_SPI_TS1: c_uint = 0x01	// Target select 1;
pub const AX_SPI_TS2: c_uint = 0x10	// Target select 2;
pub const AX_SPI_TS3: c_uint = 0x11	// Target select 3;
pub const SPI_AUTOSUSPEND_TIMEOUT: c_int = 3000;
// Default number of chip select lines also used as maximum number of chip select lines
pub const AX_SPI_DEFAULT_NUM_CS: c_int = 4;
// Default number of command buffer size

// Target select mask
// 00 – TS0
// 01 – TS1
// 10 – TS2
// 11 – TS3
//
pub const AX_SPI_DEFAULT_TS_MASK: c_uint = 0x03;
pub const AX_SPI_RX_FIFO_DRAIN_LIMIT: c_int = 24;
pub const AX_SPI_TRX_FIFO_TIMEOUT: c_int = 1000;
//
// struct ax_spi - This definition defines spi driver instance
// @regs:					Virtual address of the SPI controller registers
// @ref_clk:					Pointer to the peripheral clock
// @pclk:					Pointer to the APB clock
// @clk_rate:					Reference clock rate in Hz
// @speed_hz:					Current SPI bus clock speed in Hz
// @tx_buf:					Pointer	to the TX buffer
// @rx_buf:					Pointer to the RX buffer
// @tx_bytes:					Number of bytes left to transfer
// @rx_bytes:					Number of bytes requested
// @tx_fifo_depth:				Depth of the TX FIFO
// @current_rx_fifo_word:			Buffers the 32-bit word read from RXFIFO
// @bytes_left_in_current_rx_word:		Bytes to be extracted from current 32-bit word
// @current_rx_fifo_word_for_irq:		Buffers the 32-bit word read from RXFIFO for IRQ
// @bytes_left_in_current_rx_word_for_irq:	IRQ bytes to be extracted from current 32-bit word
// @rx_discard:					Number of bytes to discard
// @rx_copy_remaining:				Number of bytes to copy
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ax_spi {
    pub regs: *mut void __iomem,
    pub ref_clk: *mut clk,
    pub pclk: *mut clk,
    pub clk_rate: c_uint,
    pub speed_hz: u32,
    pub tx_buf: *const u8,
    pub rx_buf: *mut u8,
    pub tx_bytes: c_int,
    pub rx_bytes: c_int,
    pub tx_fifo_depth: c_uint,
    pub current_rx_fifo_word: u32,
    pub bytes_left_in_current_rx_word: c_int,
    pub current_rx_fifo_word_for_irq: u32,
    pub bytes_left_in_current_rx_word_for_irq: c_int,
    pub rx_discard: c_int,
    pub rx_copy_remaining: c_int,
}
