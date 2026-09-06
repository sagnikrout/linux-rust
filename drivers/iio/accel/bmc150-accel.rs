//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/accel/bmc150-accel.h
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
// We can often guess better than "UNKNOWN" based on the device IDs
// but unfortunately this information is not always accurate. There are some
// devices where ACPI firmware specifies an ID like "BMA250E" when the device
// actually has a BMA222E. The driver attempts to detect those by reading the
// chip ID from the registers but this information is not always enough either.
//
// Therefore, this enum should be only used when the chip ID detection is not
// enough and we can be reasonably sure that the device IDs are reliable
// in practice (e.g. for device tree platforms).
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bmc150_type {
    BOSCH_UNKNOWN,
    BOSCH_BMC156,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bmc150_accel_interrupt {
    pub info: *const bmc150_accel_interrupt_info,
    pub users: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bmc150_accel_trigger {
    pub data: *mut bmc150_accel_data,
    pub indio_trig: *mut iio_trigger,
    pub state): *mut *mut *mut int (setup)(struct bmc150_accel_trigger t, bool,
    pub intr: c_int,
    pub enabled: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bmc150_accel_interrupt_id {
    BMC150_ACCEL_INT_DATA_READY,
    BMC150_ACCEL_INT_ANY_MOTION,
    BMC150_ACCEL_INT_WATERMARK,
    BMC150_ACCEL_INTERRUPTS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bmc150_accel_trigger_id {
    BMC150_ACCEL_TRIGGER_DATA_READY,
    BMC150_ACCEL_TRIGGER_ANY_MOTION,
    BMC150_ACCEL_TRIGGERS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bmc150_accel_data {
    pub regmap: *mut regmap,
    pub irq: c_int,
    pub regulators: [regulator_bulk_data; 2],
    pub interrupts: [bmc150_accel_interrupt; BMC150_ACCEL_INTERRUPTS],
    pub triggers: [bmc150_accel_trigger; BMC150_ACCEL_TRIGGERS],
    pub mutex: mutex,
    pub watermark: u8 fifo_mode,,
    pub buffer: [i16; 8],
//
// Ensure there is sufficient space and correct alignment for
// the timestamp if enabled
//
    pub channels: [__le16; 3],
    pub ts: aligned_s64,
    pub scan: },
    pub bw_bits: u8,
    pub slope_dur: u32,
    pub slope_thres: u32,
    pub range: u32,
    pub ev_enable_state: c_int,
    pub /: *mut *mut int64_t timestamp, old_timestamp; / Only used in hw fifo mode.,
    pub chip_info: *const bmc150_accel_chip_info,
    pub type: bmc150_type,
    pub second_device: *mut i2c_client,
    pub dev): *mut *mut void (resume_callback)(struct device,
    pub resume_work: delayed_work,
    pub orientation: iio_mount_matrix,
}

extern "C" {
    pub fn bmc150_accel_core_remove(dev: *mut device);
}
