//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/rmi.h
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
// Copyright (c) 2011-2016 Synaptics Incorporated
// Copyright (c) 2011 Unixphere
//

pub const NAME_BUFFER_SIZE: c_int = 256;
//
// struct rmi_2d_axis_alignment - target axis alignment
// @swap_axes: set to TRUE if desired to swap x- and y-axis
// @flip_x: set to TRUE if desired to flip direction on x-axis
// @flip_y: set to TRUE if desired to flip direction on y-axis
// @clip_x_low - reported X coordinates below this setting will be clipped to
// the specified value
// @clip_x_high - reported X coordinates above this setting will be clipped to
// the specified value
// @clip_y_low - reported Y coordinates below this setting will be clipped to
// the specified value
// @clip_y_high - reported Y coordinates above this setting will be clipped to
// the specified value
// @offset_x - this value will be added to all reported X coordinates
// @offset_y - this value will be added to all reported Y coordinates
// @rel_report_enabled - if set to true, the relative reporting will be
// automatically enabled for this sensor.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rmi_2d_axis_alignment {
    pub swap_axes: bool,
    pub flip_x: bool,
    pub flip_y: bool,
    pub clip_x_low: u16,
    pub clip_y_low: u16,
    pub clip_x_high: u16,
    pub clip_y_high: u16,
    pub offset_x: u16,
    pub offset_y: u16,
    pub delta_x_threshold: u8,
    pub delta_y_threshold: u8,
}

// This is used to override any hints an F11 2D sensor might have provided
// as to what type of sensor it is.
//
// @rmi_f11_sensor_default - do not override, determine from F11_2D_QUERY14 if
// available.
// @rmi_f11_sensor_touchscreen - treat the sensor as a touchscreen (direct
// pointing).
// @rmi_f11_sensor_touchpad - thread the sensor as a touchpad (indirect
// pointing).
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rmi_sensor_type {
    rmi_sensor_default = 0,
    rmi_sensor_touchscreen,
    rmi_sensor_touchpad
}

//
// struct rmi_2d_sensor_data - overrides defaults for a 2D sensor.
// @axis_align - provides axis alignment overrides (see above).
// @sensor_type - Forces the driver to treat the sensor as an indirect
// pointing device (touchpad) rather than a direct pointing device
// (touchscreen).  This is useful when F11_2D_QUERY14 register is not
// available.
// @disable_report_mask - Force data to not be reported even if it is supported
// by the firware.
// @topbuttonpad - Used with the "5 buttons touchpads" found on the Lenovo 40
// series
// @kernel_tracking - most moderns RMI f11 firmwares implement Multifinger
// Type B protocol. However, there are some corner cases where the user
// triggers some jumps by tapping with two fingers on the touchpad.
// Use this setting and dmax to filter out these jumps.
// Also, when using an old sensor using MF Type A behavior, set to true to
// report an actual MT protocol B.
// @dmax - the maximum distance (in sensor units) the kernel tracking allows two
// distincts fingers to be considered the same.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rmi_2d_sensor_platform_data {
    pub axis_align: rmi_2d_axis_alignment,
    pub sensor_type: rmi_sensor_type,
    pub x_mm: c_int,
    pub y_mm: c_int,
    pub disable_report_mask: c_int,
    pub rezero_wait: u16,
    pub topbuttonpad: bool,
    pub kernel_tracking: bool,
    pub dmax: c_int,
    pub dribble: c_int,
    pub palm_detect: c_int,
}

//
// struct rmi_gpio_data - overrides defaults for a single F30/F3A GPIOs/LED
// chip.
// @buttonpad - the touchpad is a buttonpad, so enable only the first actual
// button that is found.
// @trackstick_buttons - Set when the function 30 or 3a is handling the physical
// buttons of the trackstick (as a PS/2 passthrough device).
// @disable - the touchpad incorrectly reports F30/F3A and it should be ignored.
// This is a special case which is due to misconfigured firmware.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rmi_gpio_data {
    pub buttonpad: bool,
    pub trackstick_buttons: bool,
    pub disable: bool,
}

//
// Set the state of a register
// DEFAULT - use the default value set by the firmware config
// OFF - explicitly disable the register
// ON - explicitly enable the register
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rmi_reg_state {
    RMI_REG_STATE_DEFAULT = 0,
    RMI_REG_STATE_OFF = 1,
    RMI_REG_STATE_ON = 2
}

//
// struct rmi_f01_power_management -When non-zero, these values will be written
// to the touch sensor to override the default firmware settigns.  For a
// detailed explanation of what each field does, see the corresponding
// documention in the RMI4 specification.
//
// @nosleep - specifies whether the device is permitted to sleep or doze (that
// is, enter a temporary low power state) when no fingers are touching the
// sensor.
// @wakeup_threshold - controls the capacitance threshold at which the touch
// sensor will decide to wake up from that low power state.
// @doze_holdoff - controls how long the touch sensor waits after the last
// finger lifts before entering the doze state, in units of 100ms.
// @doze_interval - controls the interval between checks for finger presence
// when the touch sensor is in doze mode, in units of 10ms.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rmi_f01_power_management {
    pub nosleep: rmi_reg_state,
    pub wakeup_threshold: u8,
    pub doze_holdoff: u8,
    pub doze_interval: u8,
}

//
// struct rmi_device_platform_data_spi - provides parameters used in SPI
// communications.  All Synaptics SPI products support a standard SPI
// interface; some also support what is called SPI V2 mode, depending on
// firmware and/or ASIC limitations.  In V2 mode, the touch sensor can
// support shorter delays during certain operations, and these are specified
// separately from the standard mode delays.
//
// @block_delay - for standard SPI transactions consisting of both a read and
// write operation, the delay (in microseconds) between the read and write
// operations.
// @split_read_block_delay_us - for V2 SPI transactions consisting of both a
// read and write operation, the delay (in microseconds) between the read and
// write operations.
// @read_delay_us - the delay between each byte of a read operation in normal
// SPI mode.
// @write_delay_us - the delay between each byte of a write operation in normal
// SPI mode.
// @split_read_byte_delay_us - the delay between each byte of a read operation
// in V2 mode.
// @pre_delay_us - the delay before the start of a SPI transaction.  This is
// typically useful in conjunction with custom chip select assertions (see
// below).
// @post_delay_us - the delay after the completion of an SPI transaction.  This
// is typically useful in conjunction with custom chip select assertions (see
// below).
// @cs_assert - For systems where the SPI subsystem does not control the CS/SSB
// line, or where such control is broken, you can provide a custom routine to
// handle a GPIO as CS/SSB.  This routine will be called at the beginning and
// end of each SPI transaction.  The RMI SPI implementation will wait
// pre_delay_us after this routine returns before starting the SPI transfer;
// and post_delay_us after completion of the SPI transfer(s) before calling it
// with assert==FALSE.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rmi_device_platform_data_spi {
    pub block_delay_us: u32,
    pub split_read_block_delay_us: u32,
    pub read_delay_us: u32,
    pub write_delay_us: u32,
    pub split_read_byte_delay_us: u32,
    pub pre_delay_us: u32,
    pub post_delay_us: u32,
    pub bits_per_word: u8,
    pub mode: u16,
    pub cs_assert_data: *mut c_void,
    pub assert): *const *const *const int (cs_assert)(void cs_assert_data, bool,
}

//
// struct rmi_device_platform_data - system specific configuration info.
//
// @reset_delay_ms - after issuing a reset command to the touch sensor, the
// driver waits a few milliseconds to give the firmware a chance to
// re-initialize.  You can override the default wait period here.
// @irq: irq associated with the attn gpio line, or negative
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rmi_device_platform_data {
    pub reset_delay_ms: c_int,
    pub irq: c_int,
    pub spi_data: rmi_device_platform_data_spi,
// function handler pdata
    pub sensor_pdata: rmi_2d_sensor_platform_data,
    pub power_management: rmi_f01_power_management,
    pub gpio_data: rmi_gpio_data,
}

//
// struct rmi_function_descriptor - RMI function base addresses
//
// @query_base_addr: The RMI Query base address
// @command_base_addr: The RMI Command base address
// @control_base_addr: The RMI Control base address
// @data_base_addr: The RMI Data base address
// @interrupt_source_count: The number of irqs this RMI function needs
// @function_number: The RMI function number
//
// This struct is used when iterating the Page Description Table. The addresses
// are 16-bit values to include the current page address.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rmi_function_descriptor {
    pub query_base_addr: u16,
    pub command_base_addr: u16,
    pub control_base_addr: u16,
    pub data_base_addr: u16,
    pub interrupt_source_count: u8,
    pub function_number: u8,
    pub function_version: u8,
}

//
// struct rmi_transport_dev - represent an RMI transport device
//
// @dev: Pointer to the communication device, e.g. i2c or spi
// @rmi_dev: Pointer to the RMI device
// @proto_name: name of the transport protocol (SPI, i2c, etc)
// @ops: pointer to transport operations implementation
//
// The RMI transport device implements the glue between different communication
// buses such as I2C and SPI.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rmi_transport_dev {
    pub dev: *mut device,
    pub rmi_dev: *mut rmi_device,
    pub proto_name: *const c_char,
    pub ops: *const rmi_transport_ops,
    pub pdata: rmi_device_platform_data,
    pub input: *mut input_dev,
}

//
// struct rmi_transport_ops - defines transport protocol operations.
//
// @write_block: Writing a block of data to the specified address
// @read_block: Read a block of data from the specified address.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rmi_transport_ops {
    pub len): *const *const void buf, size_t,
    pub len): *mut *mut void buf, size_t,
    pub reset_addr): *mut *mut *mut int (reset)(struct rmi_transport_dev xport, u16,
}

//
// struct rmi_driver - driver for an RMI4 sensor on the RMI bus.
//
// @driver: Device driver model driver
// @reset_handler: Called when a reset is detected.
// @clear_irq_bits: Clear the specified bits in the current interrupt mask.
// @set_irq_bist: Set the specified bits in the current interrupt mask.
// @store_productid: Callback for cache product id from function 01
// @data: Private data pointer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rmi_driver {
    pub driver: device_driver,
    pub rmi_dev): *mut *mut int (reset_handler)(struct rmi_device,
    pub mask): *mut *mut *mut int (clear_irq_bits)(struct rmi_device rmi_dev, unsigned long,
    pub mask): *mut *mut *mut int (set_irq_bits)(struct rmi_device rmi_dev, unsigned long,
    pub rmi_dev): *mut *mut int (store_productid)(struct rmi_device,
    pub input): *mut input_dev,
    pub data: *mut c_void,
}

//
// struct rmi_device - represents an RMI4 sensor device on the RMI bus.
//
// @dev: The device created for the RMI bus
// @number: Unique number for the device on the bus.
// @driver: Pointer to associated driver
// @xport: Pointer to the transport interface
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rmi_device {
    pub dev: device,
    pub number: c_int,
    pub driver: *mut rmi_driver,
    pub xport: *mut rmi_transport_dev,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rmi4_attn_data {
    pub irq_status: c_ulong,
    pub size: usize,
    pub data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rmi_driver_data {
    pub function_list: list_head,
    pub rmi_dev: *mut rmi_device,
    pub f01_container: *mut rmi_function,
    pub f34_container: *mut rmi_function,
    pub bootloader_mode: bool,
    pub num_of_irq_regs: c_int,
    pub irq_count: c_int,
    pub irq_memory: *mut c_void,
    pub irq_status: *mut c_ulong,
    pub fn_irq_bits: *mut c_ulong,
    pub current_irq_mask: *mut c_ulong,
    pub new_irq_mask: *mut c_ulong,
    pub irq_mutex: mutex,
    pub input: *mut input_dev,
    pub irqdomain: *mut irq_domain,
    pub pdt_props: u8,
    pub num_rx_electrodes: u8,
    pub num_tx_electrodes: u8,
    pub enabled: bool,
    pub enabled_mutex: mutex,
    pub attn_data: rmi4_attn_data,
    pub 16): DECLARE_KFIFO(attn_fifo, struct rmi4_attn_data,,
}

extern "C" {
    pub fn rmi_register_transport_device(xport: *mut rmi_transport_dev) -> c_int;
}
extern "C" {
    pub fn rmi_unregister_transport_device(xport: *mut rmi_transport_dev);
}
extern "C" {
    pub fn rmi_driver_suspend(rmi_dev: *mut rmi_device, enable_wake: bool) -> c_int;
}
extern "C" {
    pub fn rmi_driver_resume(rmi_dev: *mut rmi_device, clear_wake: bool) -> c_int;
}
