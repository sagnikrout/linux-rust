//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/spi/spi-pxa2xx.h
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
// Copyright (C) 2005 Stephen Street / StreetFire Sound Labs
// Copyright (C) 2013, 2021 Intel Corporation
//

//
// The platform data for SSP controller devices
// (resides in device.platform_data).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pxa2xx_spi_controller {
    pub num_chipselect: u8,
    pub enable_dma: u8,
    pub dma_burst_size: u8,
    pub is_target: bool,
// DMA engine specific config
    pub dma_filter: dma_filter_fn,
    pub tx_param: *mut c_void,
    pub rx_param: *mut c_void,
// For non-PXA arches
    pub ssp: ssp_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct driver_data {
// SSP Info
    pub ssp: *mut ssp_device,
// SPI framework hookup
    pub ssp_type: pxa_ssp_type,
    pub controller: *mut spi_controller,
// PXA hookup
    pub controller_info: *mut pxa2xx_spi_controller,
// SSP masks
    pub dma_cr1: u32,
    pub int_cr1: u32,
    pub clear_sr: u32,
    pub mask_sr: u32,
// DMA engine support
    pub dma_running: core::sync::atomic::AtomicI32,
// Current transfer state info
    pub tx: *mut c_void,
    pub tx_end: *mut c_void,
    pub rx: *mut c_void,
    pub rx_end: *mut c_void,
    pub n_bytes: u8,
    pub drv_data): *mut *mut int (write)(struct driver_data,
    pub drv_data): *mut *mut int (read)(struct driver_data,
    pub drv_data): *mut *mut irqreturn_t (transfer_handler)(struct driver_data,
    pub lpss_base: *mut void __iomem,
// Optional slave FIFO ready signal
    pub gpiod_ready: *mut gpio_desc,
}

extern "C" {
    pub fn pxa_ssp_read_reg(_arg: drv_data->ssp, _arg: reg) -> return;
}
pub const DMA_ALIGNMENT: c_int = 8;
extern "C" {
    pub fn pxa2xx_spi_flush(drv_data: *mut driver_data) -> c_int;
}

extern "C" {
    pub fn pxa2xx_spi_dma_transfer(drv_data: *mut driver_data) -> irqreturn_t;
}
extern "C" {
    pub fn pxa2xx_spi_dma_start(drv_data: *mut driver_data);
}
extern "C" {
    pub fn pxa2xx_spi_dma_stop(drv_data: *mut driver_data);
}
extern "C" {
    pub fn pxa2xx_spi_dma_setup(drv_data: *mut driver_data) -> c_int;
}
extern "C" {
    pub fn pxa2xx_spi_dma_release(drv_data: *mut driver_data);
}
extern "C" {
    pub fn pxa2xx_spi_remove(dev: *mut device);
}
