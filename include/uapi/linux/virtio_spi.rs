//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/virtio_spi.h
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


// SPDX-License-Identifier: BSD-3-Clause
//
// Copyright (C) 2023 OpenSynergy GmbH
// Copyright (C) 2025 Qualcomm Innovation Center, Inc. All rights reserved.
//

// Sample data on trailing clock edge

// Clock is high when IDLE

// Chip Select is active high

// Transmit LSB first

// Loopback mode

//
// struct virtio_spi_config - All config fields are read-only for the
// Virtio SPI driver
// @cs_max_number: maximum number of chipselect the host SPI controller
// supports.
// @cs_change_supported: indicates if the host SPI controller supports to toggle
// chipselect after each transfer in one message:
// 0: unsupported, chipselect will be kept in active state throughout the
// message transaction;
// 1: supported.
// Note: Message here contains a sequence of SPI transfers.
// @tx_nbits_supported: indicates the supported number of bit for writing:
// bit 0: DUAL (2-bit transfer), 1 for supported
// bit 1: QUAD (4-bit transfer), 1 for supported
// bit 2: OCTAL (8-bit transfer), 1 for supported
// other bits are reserved as 0, 1-bit transfer is always supported.
// @rx_nbits_supported: indicates the supported number of bit for reading:
// bit 0: DUAL (2-bit transfer), 1 for supported
// bit 1: QUAD (4-bit transfer), 1 for supported
// bit 2: OCTAL (8-bit transfer), 1 for supported
// other bits are reserved as 0, 1-bit transfer is always supported.
// @bits_per_word_mask: mask indicating which values of bits_per_word are
// supported. If not set, no limitation for bits_per_word.
// @mode_func_supported: indicates the following features are supported or not:
// bit 0-1: CPHA feature
// 0b00: invalid, should support as least one CPHA setting
// 0b01: supports CPHA=0 only
// 0b10: supports CPHA=1 only
// 0b11: supports CPHA=0 and CPHA=1.
// bit 2-3: CPOL feature
// 0b00: invalid, should support as least one CPOL setting
// 0b01: supports CPOL=0 only
// 0b10: supports CPOL=1 only
// 0b11: supports CPOL=0 and CPOL=1.
// bit 4: chipselect active high feature, 0 for unsupported and 1 for
// supported, chipselect active low is supported by default.
// bit 5: LSB first feature, 0 for unsupported and 1 for supported,
// MSB first is supported by default.
// bit 6: loopback mode feature, 0 for unsupported and 1 for supported,
// normal mode is supported by default.
// @max_freq_hz: the maximum clock rate supported in Hz unit, 0 means no
// limitation for transfer speed.
// @max_word_delay_ns: the maximum word delay supported, in nanoseconds.
// A value of 0 indicates that word delay is unsupported.
// Each transfer may consist of a sequence of words.
// @max_cs_setup_ns: the maximum delay supported after chipselect is asserted,
// in ns unit, 0 means delay is not supported to introduce after chipselect is
// asserted.
// @max_cs_hold_ns: the maximum delay supported before chipselect is deasserted,
// in ns unit, 0 means delay is not supported to introduce before chipselect
// is deasserted.
// @max_cs_incative_ns: maximum delay supported after chipselect is deasserted,
// in ns unit, 0 means delay is not supported to introduce after chipselect is
// deasserted.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_spi_config {
    pub cs_max_number: __u8,
    pub cs_change_supported: __u8,

    pub tx_nbits_supported: __u8,
    pub rx_nbits_supported: __u8,
    pub bits_per_word_mask: __le32,

    pub mode_func_supported: __le32,
    pub max_freq_hz: __le32,
    pub max_word_delay_ns: __le32,
    pub max_cs_setup_ns: __le32,
    pub max_cs_hold_ns: __le32,
    pub max_cs_inactive_ns: __le32,
}

//
// struct spi_transfer_head - virtio SPI transfer descriptor
// @chip_select_id: chipselect index the SPI transfer used.
// @bits_per_word: the number of bits in each SPI transfer word.
// @cs_change: whether to deselect device after finishing this transfer
// before starting the next transfer, 0 means cs keep asserted and
// 1 means cs deasserted then asserted again.
// @tx_nbits: bus width for write transfer.
// 0,1: bus width is 1, also known as SINGLE
// 2  : bus width is 2, also known as DUAL
// 4  : bus width is 4, also known as QUAD
// 8  : bus width is 8, also known as OCTAL
// other values are invalid.
// @rx_nbits: bus width for read transfer.
// 0,1: bus width is 1, also known as SINGLE
// 2  : bus width is 2, also known as DUAL
// 4  : bus width is 4, also known as QUAD
// 8  : bus width is 8, also known as OCTAL
// other values are invalid.
// @reserved: for future use.
// @mode: SPI transfer mode.
// bit 0: CPHA, determines the timing (i.e. phase) of the data
// bits relative to the clock pulses.For CPHA=0, the
// "out" side changes the data on the trailing edge of the
// preceding clock cycle, while the "in" side captures the data
// on (or shortly after) the leading edge of the clock cycle.
// For CPHA=1, the "out" side changes the data on the leading
// edge of the current clock cycle, while the "in" side
// captures the data on (or shortly after) the trailing edge of
// the clock cycle.
// bit 1: CPOL, determines the polarity of the clock. CPOL=0 is a
// clock which idles at 0, and each cycle consists of a pulse
// of 1. CPOL=1 is a clock which idles at 1, and each cycle
// consists of a pulse of 0.
// bit 2: CS_HIGH, if 1, chip select active high, else active low.
// bit 3: LSB_FIRST, determines per-word bits-on-wire, if 0, MSB
// first, else LSB first.
// bit 4: LOOP, loopback mode.
// @freq: the transfer speed in Hz.
// @word_delay_ns: delay to be inserted between consecutive words of a
// transfer, in ns unit.
// @cs_setup_ns: delay to be introduced after CS is asserted, in ns
// unit.
// @cs_delay_hold_ns: delay to be introduced before CS is deasserted
// for each transfer, in ns unit.
// @cs_change_delay_inactive_ns: delay to be introduced after CS is
// deasserted and before next asserted, in ns unit.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_transfer_head {
    pub chip_select_id: __u8,
    pub bits_per_word: __u8,
    pub cs_change: __u8,
    pub tx_nbits: __u8,
    pub rx_nbits: __u8,
    pub reserved: [__u8; 3],
    pub mode: __le32,
    pub freq: __le32,
    pub word_delay_ns: __le32,
    pub cs_setup_ns: __le32,
    pub cs_delay_hold_ns: __le32,
    pub cs_change_delay_inactive_ns: __le32,
}

//
// struct spi_transfer_result - virtio SPI transfer result
// @result: Transfer result code.
// VIRTIO_SPI_TRANS_OK: Transfer successful.
// VIRTIO_SPI_PARAM_ERR: Parameter error.
// VIRTIO_SPI_TRANS_ERR: Transfer error.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_transfer_result {
pub const VIRTIO_SPI_TRANS_OK: c_int = 0;
pub const VIRTIO_SPI_PARAM_ERR: c_int = 1;
pub const VIRTIO_SPI_TRANS_ERR: c_int = 2;
    pub result: __u8,
}
