//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/iio/adc/ad_sigma_delta.h
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
// Support code for Analog Devices Sigma-Delta ADCs
//
// Copyright 2012 Analog Devices Inc.
// Author: Lars-Peter Clausen <lars@metafoo.de>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ad_sigma_delta_mode {
    AD_SD_MODE_CONTINUOUS = 0,
    AD_SD_MODE_SINGLE = 1,
    AD_SD_MODE_IDLE = 2,
    AD_SD_MODE_POWERDOWN = 3,
}

//
// struct ad_sigma_delta_calib_data - Calibration data for Sigma Delta devices
// @mode: Calibration mode.
// @channel: Calibration channel.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad_sd_calib_data {
    pub mode: c_uint,
    pub channel: c_uint,
}

//
// struct ad_sigma_delta_info - Sigma Delta driver specific callbacks and options
// @set_channel: Will be called to select the current channel, may be NULL.
// @append_status: Will be called to enable status append at the end of the sample, may be NULL.
// @set_mode: Will be called to select the current mode, may be NULL.
// @disable_all: Will be called to disable all channels, may be NULL.
// @disable_one: Will be called to disable a single channel after
// ad_sigma_delta_single_conversion(), may be NULL.
// Usage of this callback expects iio_chan_spec.address to contain
// the value required for the driver to identify the channel.
// @postprocess_sample: Is called for each sampled data word, can be used to
// modify or drop the sample data, it, may be NULL.
// @has_registers: true if the device has writable and readable registers, false
// if there is just one read-only sample data shift register.
// @has_named_irqs: Set to true if there is more than one IRQ line.
// @supports_spi_offload: Set to true if the driver supports SPI offload. Often
// special considerations are needed for scan_type and other channel
// info, so individual drivers have to set this to let the core
// code know that it can use SPI offload if it is available.
// @addr_shift: Shift of the register address in the communications register.
// @read_mask: Mask for the communications register having the read bit set.
// @status_ch_mask: Mask for the channel number stored in status register.
// @data_reg: Address of the data register, if 0 the default address of 0x3 will
// be used.
// @irq_flags: flags for the interrupt used by the triggered buffer
// @num_slots: Number of sequencer slots
// @num_resetclks: Number of SPI clk cycles with MOSI=1 to reset the chip.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad_sigma_delta_info {
    pub channel): *mut *mut *mut int (set_channel)(struct ad_sigma_delta , unsigned int,
    pub append): *mut *mut *mut int (append_status)(struct ad_sigma_delta , bool,
    pub mode): *mut *mut *mut int (set_mode)(struct ad_sigma_delta , enum ad_sigma_delta_mode,
    pub ): *mut *mut int (disable_all)(struct ad_sigma_delta,
    pub chan): *mut *mut *mut int (disable_one)(struct ad_sigma_delta , unsigned int,
    pub raw_sample): *mut *mut *mut int (postprocess_sample)(struct ad_sigma_delta , unsigned int,
    pub has_registers: bool,
    pub has_named_irqs: bool,
    pub supports_spi_offload: bool,
    pub addr_shift: c_uint,
    pub read_mask: c_uint,
    pub status_ch_mask: c_uint,
    pub data_reg: c_uint,
    pub irq_flags: c_ulong,
    pub num_slots: c_uint,
    pub num_resetclks: c_uint,
}

//
// struct ad_sigma_delta - Sigma Delta device struct
// @spi: The spi device associated with the Sigma Delta device.
// @trig: The IIO trigger associated with the Sigma Delta device.
//
// Most of the fields are private to the sigma delta library code and should not
// be accessed by individual drivers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad_sigma_delta {
    pub spi: *mut spi_device,
    pub trig: *mut iio_trigger,
// private:
    pub completion: completion,
    pub /: *mut *mut spinlock_t irq_lock; / protects .irq_dis and irq en/disable state,
    pub irq_dis: bool,
    pub bus_locked: bool,
    pub keep_cs_asserted: bool,
    pub comm: u8,
    pub info: *const ad_sigma_delta_info,
    pub active_slots: c_uint,
    pub current_slot: c_uint,
    pub num_slots: c_uint,
    pub rdy_gpiod: *mut gpio_desc,
    pub irq_line: c_int,
    pub status_appended: bool,
// map slots to channels in order to know what to expect from devices
    pub slots: *mut c_uint,
    pub sample_msg: spi_message,
    pub sample_xfer: [spi_transfer; 2],
    pub samples_buf: *mut u8,
    pub offload: *mut spi_offload,
    pub offload_trigger: *mut spi_offload_trigger,
//
// DMA (thus cache coherency maintenance) requires the
// transfer buffers to live in their own cache lines.
// 'tx_buf' is up to 32 bits.
// 'rx_buf' is up to 32 bits per sample + 64 bit timestamp,
// rounded to 16 bytes to take into account padding.
//
    pub __aligned(IIO_DMA_MINALIGN): u8 tx_buf[4],
    pub __aligned(8): u8 rx_buf[16],
    pub sample_addr: u8,
}

extern "C" {
    pub fn ad_sd_set_comm(sigma_delta: *mut ad_sigma_delta, comm: u8);
}
extern "C" {
    pub fn ad_sd_reset(sigma_delta: *mut ad_sigma_delta) -> c_int;
}
extern "C" {
    pub fn devm_ad_sd_setup_buffer_and_trigger(dev: *mut device, indio_dev: *mut iio_dev) -> c_int;
}
extern "C" {
    pub fn ad_sd_validate_trigger(indio_dev: *mut iio_dev, trig: *mut iio_trigger) -> c_int;
}
