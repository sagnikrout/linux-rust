//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/spi/offload/types.h
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
// Copyright (C) 2024 Analog Devices Inc.
// Copyright (C) 2024 BayLibre, SAS
//

// This is write xfer but TX uses external data stream rather than tx_buf.

// This is read xfer but RX uses external data stream rather than rx_buf.

// Offload can be triggered by external hardware event.

// Offload can record and then play back TX data when triggered.

// Offload can get TX data from an external stream source.

// Offload can send RX data to an external stream sink.

//
// struct spi_offload_config - offload configuration
//
// This is used to request an offload with specific configuration.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_offload_config {
// @capability_flags: required capabilities. See %SPI_OFFLOAD_CAP_*
    pub capability_flags: u32,
}

//
// struct spi_offload - offload instance
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_offload {
// @provider_dev: for get/put reference counting
    pub provider_dev: *mut device,
// @priv: provider driver private data
    pub priv: *mut c_void,
// @ops: callbacks for offload support
    pub ops: *const spi_offload_ops,
// @xfer_flags: %SPI_OFFLOAD_XFER_* flags supported by provider
    pub xfer_flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum spi_offload_trigger_type {
// Indication from SPI peripheral that data is read to read.
    SPI_OFFLOAD_TRIGGER_DATA_READY,
// Trigger comes from a periodic source such as a clock.
    SPI_OFFLOAD_TRIGGER_PERIODIC,
}

//
// spi_offload_trigger_periodic - configuration parameters for periodic triggers
// @frequency_hz: The rate that the trigger should fire in Hz.
// @offset_ns: A delay in nanoseconds between when this trigger fires
// compared to another trigger. This requires specialized hardware
// that supports such synchronization with a delay between two or
// more triggers. Set to 0 when not needed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_offload_trigger_periodic {
    pub frequency_hz: u64,
    pub offset_ns: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_offload_trigger_config {
// @type: type discriminator for union
    pub type: spi_offload_trigger_type,
    pub periodic: spi_offload_trigger_periodic,
}

//
// struct spi_offload_ops - callbacks implemented by offload providers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spi_offload_ops {
//
// @trigger_enable: Optional callback to enable the trigger for the
// given offload instance.
//
    pub offload): *mut *mut int (trigger_enable)(struct spi_offload,
//
// @trigger_disable: Optional callback to disable the trigger for the
// given offload instance.
//
    pub offload): *mut *mut void (trigger_disable)(struct spi_offload,
//
// @tx_stream_request_dma_chan: Optional callback for controllers that
// have an offload where the TX data stream is connected directly to a
// DMA channel.
//
    pub offload): *mut *mut *mut dma_chan (tx_stream_request_dma_chan)(spi_offload,
//
// @rx_stream_request_dma_chan: Optional callback for controllers that
// have an offload where the RX data stream is connected directly to a
// DMA channel.
//
    pub offload): *mut *mut *mut dma_chan (rx_stream_request_dma_chan)(spi_offload,
}
