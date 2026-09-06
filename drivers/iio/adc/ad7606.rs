//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/adc/ad7606.h
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


// SPDX-License-Identifier: GPL-2.0
//
// AD7606 ADC driver
//
// Copyright 2011 Analog Devices Inc.
//
pub const AD760X_MAX_CHANNELS: c_int = 16;
pub const AD7616_CONFIGURATION_REGISTER: c_uint = 0x02;

pub const AD7616_RANGE_CH_A_ADDR_OFF: c_uint = 0x04;
pub const AD7616_RANGE_CH_B_ADDR_OFF: c_uint = 0x06;
//
// Range of channels from a group are stored in 2 registers.
// 0, 1, 2, 3 in a register followed by 4, 5, 6, 7 in second register.
// For channels from second group(8-15) the order is the same, only with
// an offset of 2 for register address.
//

// The range of the channel is stored in 2 bits

pub const AD7606_CONFIGURATION_REGISTER: c_uint = 0x02;
pub const AD7606_SINGLE_DOUT: c_uint = 0x00;
//
// Range for AD7606B channels are stored in registers starting with address 0x3.
// Each register stores range for 2 channels(4 bits per channel).
//

pub const AD7606_OS_MODE: c_uint = 0x08;

extern "C" {
    pub fn int(indio_dev: *mut *mut ad7606_sw_setup_cb_t)(struct iio_dev) -> typedef;
}
//
// struct ad7606_chip_info - chip specific information
// @max_samplerate:	maximum supported sample rate
// @name:		device name
// @bits:		data width in bits
// @num_adc_channels:	the number of physical voltage inputs
// @scale_setup_cb:	callback to setup the scales for each channel
// @sw_setup_cb:	callback to setup the software mode if available.
// @oversampling_avail:	pointer to the array which stores the available
// oversampling ratios.
// @oversampling_num:	number of elements stored in oversampling_avail array
// @os_req_reset:	some devices require a reset to update oversampling
// @init_delay_ms:	required delay in milliseconds for initialization
// after a restart
// @offload_storagebits: storage bits used by the offload hw implementation
// @calib_gain_avail:   chip supports gain calibration
// @calib_offset_avail: pointer to offset calibration range/limits array
// @calib_phase_avail:  pointer to phase calibration range/limits array
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad7606_chip_info {
    pub max_samplerate: c_uint,
    pub name: *const c_char,
    pub bits: c_uint,
    pub num_adc_channels: c_uint,
    pub scale_setup_cb: ad7606_scale_setup_cb_t,
    pub sw_setup_cb: ad7606_sw_setup_cb_t,
    pub oversampling_avail: *const c_uint,
    pub oversampling_num: c_uint,
    pub os_req_reset: bool,
    pub init_delay_ms: c_ulong,
    pub offload_storagebits: u8,
    pub calib_gain_avail: bool,
    pub calib_offset_avail: *const c_int,
    pub (*calib_phase_avail)[2]: *const c_int,
}

//
// struct ad7606_chan_info - channel configuration
// @scale_avail:	pointer to the array which stores the available scales
// @num_scales:		number of elements stored in the scale_avail array
// @range:		voltage range selection, selects which scale to apply
// @reg_offset:		offset for the register value, to be applied when
// writing the value of 'range' to the register value
// @r_gain:		gain resistor value in ohms, to be set to match the
// external r_filter value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad7606_chan_info {
pub const AD760X_MAX_SCALES: c_int = 16;
    pub (*scale_avail)[2]: *const c_uint,
    pub num_scales: c_uint,
    pub range: c_uint,
    pub reg_offset: c_uint,
    pub r_gain: c_uint,
}

//
// struct ad7606_state - driver instance specific data
// @dev:		pointer to kernel device
// @chip_info:		entry in the table of chips that describes this device
// @bops:		bus operations (SPI or parallel)
// @chan_info:		scale configuration for channels
// @oversampling:	oversampling selection
// @cnvst_pwm:		pointer to the PWM device connected to the cnvst pin
// @base_address:	address from where to read data in parallel operation
// @sw_mode_en:		software mode enabled
// @oversampling_avail:	pointer to the array which stores the available
// oversampling ratios.
// @num_os_ratios:	number of elements stored in oversampling_avail array
// @back:		pointer to the iio_backend structure, if used
// @write_scale:	pointer to the function which writes the scale
// @write_os:		pointer to the function which writes the os
// @lock:		protect sensor state from concurrent accesses to GPIOs
// @gpio_convst:	GPIO descriptor for conversion start signal (CONVST)
// @gpio_reset:		GPIO descriptor for device hard-reset
// @gpio_range:		GPIO descriptor for range selection
// @gpio_standby:	GPIO descriptor for stand-by signal (STBY),
// controls power-down mode of device
// @gpio_frstdata:	GPIO descriptor for reading from device when data
// is being read on the first channel
// @gpio_os:		GPIO descriptors to control oversampling on the device
// @trig:		The IIO trigger associated with the device.
// @completion:		completion to indicate end of conversion
// @data:		buffer for reading data from the device
// @offload_en:		SPI offload enabled
// @bus_data:		bus-specific variables
// @d16:		be16 buffer for reading data from the device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad7606_state {
    pub dev: *mut device,
    pub chip_info: *const ad7606_chip_info,
    pub bops: *const ad7606_bus_ops,
    pub chan_info: [ad7606_chan_info; AD760X_MAX_CHANNELS],
    pub oversampling: c_uint,
    pub cnvst_pwm: *mut pwm_device,
    pub base_address: *mut void __iomem,
    pub sw_mode_en: bool,
    pub oversampling_avail: *const c_uint,
    pub num_os_ratios: c_uint,
    pub back: *mut iio_backend,
    pub val): *mut *mut *mut int (write_scale)(struct iio_dev indio_dev, int ch, int,
    pub val): *mut *mut *mut int (write_os)(struct iio_dev indio_dev, int,
    pub /: *mut *mut mutex lock; / protect sensor state,
    pub gpio_convst: *mut gpio_desc,
    pub gpio_reset: *mut gpio_desc,
    pub gpio_range: *mut gpio_desc,
    pub gpio_standby: *mut gpio_desc,
    pub gpio_frstdata: *mut gpio_desc,
    pub gpio_os: *mut gpio_descs,
    pub trig: *mut iio_trigger,
    pub completion: completion,
    pub offload_en: bool,
    pub bus_data: *mut c_void,
//
// DMA (thus cache coherency maintenance) may require the
// transfer buffers to live in their own cache lines.
// 16 * 16-bit samples for AD7616
// 8 * 32-bit samples for AD7616C-18 (and similar)
//
    pub buf16: [u16; 16],
    pub buf32: [u32; 8],
}

//
// struct ad7606_bus_ops - driver bus operations
// @iio_backend_config:	function pointer for configuring the iio_backend for
// the compatibles that use it
// @read_block:		function pointer for reading blocks of data
// @sw_mode_config:	pointer to a function which configured the device
// for software mode
// @offload_config:     function pointer for configuring offload support,
// where any
// @reg_read:		function pointer for reading spi register
// @reg_write:		function pointer for writing spi register
// @update_scan_mode:	function pointer for handling the calls to iio_info's
// update_scan mode when enabling/disabling channels.
// @rd_wr_cmd:		pointer to the function which calculates the spi address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad7606_bus_ops {
// more methods added in future?
    pub indio_dev): *mut *mut *mut int (iio_backend_config)(struct device dev, struct iio_dev,
    pub indio_dev): *mut *mut *mut int (offload_config)(struct device dev, struct iio_dev,
    pub data): *mut *mut *mut int (read_block)(struct device dev, int num, void,
    pub indio_dev): *mut *mut int (sw_mode_config)(struct iio_dev,
    pub addr): *mut *mut *mut int (reg_read)(struct ad7606_state st, unsigned int,
    pub val): c_uint,
    pub scan_mask): *const *const *const int (update_scan_mode)(struct iio_dev indio_dev, unsigned long,
    pub is_write_op): *mut *mut u16 (rd_wr_cmd)(int addr, char,
}

//
// struct ad7606_bus_info - aggregate ad7606_chip_info and ad7606_bus_ops
// @chip_info:		entry in the table of chips that describes this device
// @bops:		bus operations (SPI or parallel)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ad7606_bus_info {
    pub chip_info: *const ad7606_chip_info,
    pub bops: *const ad7606_bus_ops,
}

extern "C" {
    pub fn ad7606_reset(st: *mut ad7606_state) -> c_int;
}
extern "C" {
    pub fn ad7606_pwm_set_swing(st: *mut ad7606_state) -> c_int;
}
extern "C" {
    pub fn ad7606_pwm_set_low(st: *mut ad7606_state) -> c_int;
}

