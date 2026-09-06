//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/comedi/drivers/ni_tio.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Header file for NI general purpose counter support code (ni_tio.c)
//
// COMEDI - Linux Control and Measurement Device Interface
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ni_gpct_register {
    NITIO_G0_AUTO_INC,
    NITIO_G1_AUTO_INC,
    NITIO_G2_AUTO_INC,
    NITIO_G3_AUTO_INC,
    NITIO_G0_CMD,
    NITIO_G1_CMD,
    NITIO_G2_CMD,
    NITIO_G3_CMD,
    NITIO_G0_HW_SAVE,
    NITIO_G1_HW_SAVE,
    NITIO_G2_HW_SAVE,
    NITIO_G3_HW_SAVE,
    NITIO_G0_SW_SAVE,
    NITIO_G1_SW_SAVE,
    NITIO_G2_SW_SAVE,
    NITIO_G3_SW_SAVE,
    NITIO_G0_MODE,
    NITIO_G1_MODE,
    NITIO_G2_MODE,
    NITIO_G3_MODE,
    NITIO_G0_LOADA,
    NITIO_G1_LOADA,
    NITIO_G2_LOADA,
    NITIO_G3_LOADA,
    NITIO_G0_LOADB,
    NITIO_G1_LOADB,
    NITIO_G2_LOADB,
    NITIO_G3_LOADB,
    NITIO_G0_INPUT_SEL,
    NITIO_G1_INPUT_SEL,
    NITIO_G2_INPUT_SEL,
    NITIO_G3_INPUT_SEL,
    NITIO_G0_CNT_MODE,
    NITIO_G1_CNT_MODE,
    NITIO_G2_CNT_MODE,
    NITIO_G3_CNT_MODE,
    NITIO_G0_GATE2,
    NITIO_G1_GATE2,
    NITIO_G2_GATE2,
    NITIO_G3_GATE2,
    NITIO_G01_STATUS,
    NITIO_G23_STATUS,
    NITIO_G01_RESET,
    NITIO_G23_RESET,
    NITIO_G01_STATUS1,
    NITIO_G23_STATUS1,
    NITIO_G01_STATUS2,
    NITIO_G23_STATUS2,
    NITIO_G0_DMA_CFG,
    NITIO_G1_DMA_CFG,
    NITIO_G2_DMA_CFG,
    NITIO_G3_DMA_CFG,
    NITIO_G0_DMA_STATUS,
    NITIO_G1_DMA_STATUS,
    NITIO_G2_DMA_STATUS,
    NITIO_G3_DMA_STATUS,
    NITIO_G0_ABZ,
    NITIO_G1_ABZ,
    NITIO_G0_INT_ACK,
    NITIO_G1_INT_ACK,
    NITIO_G2_INT_ACK,
    NITIO_G3_INT_ACK,
    NITIO_G0_STATUS,
    NITIO_G1_STATUS,
    NITIO_G2_STATUS,
    NITIO_G3_STATUS,
    NITIO_G0_INT_ENA,
    NITIO_G1_INT_ENA,
    NITIO_G2_INT_ENA,
    NITIO_G3_INT_ENA,
    NITIO_NUM_REGS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ni_gpct_variant {
    ni_gpct_variant_e_series,
    ni_gpct_variant_m_series,
    ni_gpct_variant_660x
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ni_gpct {
    pub counter_dev: *mut ni_gpct_device,
    pub counter_index: c_uint,
    pub chip_index: c_uint,
    pub /: *mut *mut u64 clock_period_ps; / clock period in picoseconds,
    pub mite_chan: *mut mite_channel,
    pub /: *mut *mut spinlock_t lock; / protects 'mite_chan',
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ni_gpct_device {
    pub dev: *mut comedi_device,
    pub ni_gpct_register): enum,
    pub ni_gpct_register): *mut *mut *mut unsigned int (read)(struct ni_gpct counter, enum,
    pub variant: ni_gpct_variant,
    pub counters: *mut ni_gpct,
    pub num_counters: c_uint,
    pub num_chips: c_uint,
    pub /: *mut *mut *mut unsigned int (regs)[NITIO_NUM_REGS]; / [num_chips][NITIO_NUM_REGS],
    pub /: *mut *mut spinlock_t regs_lock; / protects 'regs',
    pub /: *const *const *const ni_route_tables routing_tables; / link to routes,
}

extern "C" {
    pub fn ni_gpct_device_destroy(counter_dev: *mut ni_gpct_device);
}
extern "C" {
    pub fn ni_tio_init_counter(counter: *mut ni_gpct);
}
extern "C" {
    pub fn ni_tio_cmd(dev: *mut comedi_device, s: *mut comedi_subdevice) -> c_int;
}
extern "C" {
    pub fn ni_tio_cancel(counter: *mut ni_gpct) -> c_int;
}
extern "C" {
    pub fn ni_tio_acknowledge(counter: *mut ni_gpct);
}
//
// Retrieves the register value of the current source of the output selector for
// the given destination.
//
// If the terminal for the destination is not already configured as an output,
// this function returns -EINVAL as error.
//
// Return: the register value of the destination output selector;
// -EINVAL if terminal is not configured for output.
//
// Sets the register value of the selector MUX for the given destination.
// @counter_dev:Pointer to general counter device.
// @destination:Device-global identifier of route destination.
// @register_value:
// The first several bits of this value should store the desired
// value to write to the register.  All other bits are for
// transmitting information that modify the mode of the particular
// destination/gate.  These mode bits might include a bitwise or of
// CR_INVERT and CR_EDGE.  Note that the calling function should
// have already validated the correctness of this value.
//
// Sets the given destination MUX to its default value or disable it.
//
// Return: 0 if successful; -EINVAL if terminal is unknown.
//
