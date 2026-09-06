//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/input/adxl34x.h
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
// include/linux/input/adxl34x.h
//
// Digital Accelerometer characteristics are highly application specific
// and may vary between boards and models. The platform_data for the
// device's "struct device" holds this information.
//
// Copyright 2009 Analog Devices Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adxl34x_platform_data {
//
// X,Y,Z Axis Offset:
// offer user offset adjustments in twoscompliment
// form with a scale factor of 15.6 mg/LSB (i.e. 0x7F = +2 g)
//
    pub x_axis_offset: i8,
    pub y_axis_offset: i8,
    pub z_axis_offset: i8,
//
// TAP_X/Y/Z Enable: Setting TAP_X, Y, or Z Enable enables X,
// Y, or Z participation in Tap detection. A '0' excludes the
// selected axis from participation in Tap detection.
// Setting the SUPPRESS bit suppresses Double Tap detection if
// acceleration greater than tap_threshold is present during the
// tap_latency period, i.e. after the first tap but before the
// opening of the second tap window.
//

    pub tap_axis_control: u8,
//
// tap_threshold:
// holds the threshold value for tap detection/interrupts.
// The data format is unsigned. The scale factor is 62.5 mg/LSB
// (i.e. 0xFF = +16 g). A zero value may result in undesirable
// behavior if Tap/Double Tap is enabled.
//
    pub tap_threshold: u8,
//
// tap_duration:
// is an unsigned time value representing the maximum
// time that an event must be above the tap_threshold threshold
// to qualify as a tap event. The scale factor is 625 us/LSB. A zero
// value will prevent Tap/Double Tap functions from working.
//
    pub tap_duration: u8,
//
// tap_latency:
// is an unsigned time value representing the wait time
// from the detection of a tap event to the opening of the time
// window tap_window for a possible second tap event. The scale
// factor is 1.25 ms/LSB. A zero value will disable the Double Tap
// function.
//
    pub tap_latency: u8,
//
// tap_window:
// is an unsigned time value representing the amount
// of time after the expiration of tap_latency during which a second
// tap can begin. The scale factor is 1.25 ms/LSB. A zero value will
// disable the Double Tap function.
//
    pub tap_window: u8,
//
// act_axis_control:
// X/Y/Z Enable: A '1' enables X, Y, or Z participation in activity
// or inactivity detection. A '0' excludes the selected axis from
// participation. If all of the axes are excluded, the function is
// disabled.
// AC/DC: A '0' = DC coupled operation and a '1' = AC coupled
// operation. In DC coupled operation, the current acceleration is
// compared with activity_threshold and inactivity_threshold directly
// to determine whether activity or inactivity is detected. In AC
// coupled operation for activity detection, the acceleration value
// at the start of activity detection is taken as a reference value.
// New samples of acceleration are then compared to this
// reference value and if the magnitude of the difference exceeds
// activity_threshold the device will trigger an activity interrupt. In
// AC coupled operation for inactivity detection, a reference value
// is used again for comparison and is updated whenever the
// device exceeds the inactivity threshold. Once the reference
// value is selected, the device compares the magnitude of the
// difference between the reference value and the current
// acceleration with inactivity_threshold. If the difference is below
// inactivity_threshold for a total of inactivity_time, the device is
// considered inactive and the inactivity interrupt is triggered.
//

    pub act_axis_control: u8,
//
// activity_threshold:
// holds the threshold value for activity detection.
// The data format is unsigned. The scale factor is
// 62.5 mg/LSB. A zero value may result in undesirable behavior if
// Activity interrupt is enabled.
//
    pub activity_threshold: u8,
//
// inactivity_threshold:
// holds the threshold value for inactivity
// detection. The data format is unsigned. The scale
// factor is 62.5 mg/LSB. A zero value may result in undesirable
// behavior if Inactivity interrupt is enabled.
//
    pub inactivity_threshold: u8,
//
// inactivity_time:
// is an unsigned time value representing the
// amount of time that acceleration must be below the value in
// inactivity_threshold for inactivity to be declared. The scale factor
// is 1 second/LSB. Unlike the other interrupt functions, which
// operate on unfiltered data, the inactivity function operates on the
// filtered output data. At least one output sample must be
// generated for the inactivity interrupt to be triggered. This will
// result in the function appearing un-responsive if the
// inactivity_time register is set with a value less than the time
// constant of the Output Data Rate. A zero value will result in an
// interrupt when the output data is below inactivity_threshold.
//
    pub inactivity_time: u8,
//
// free_fall_threshold:
// holds the threshold value for Free-Fall detection.
// The data format is unsigned. The root-sum-square(RSS) value
// of all axes is calculated and compared to the value in
// free_fall_threshold to determine if a free fall event may be
// occurring.  The scale factor is 62.5 mg/LSB. A zero value may
// result in undesirable behavior if Free-Fall interrupt is
// enabled. Values between 300 and 600 mg (0x05 to 0x09) are
// recommended.
//
    pub free_fall_threshold: u8,
//
// free_fall_time:
// is an unsigned time value representing the minimum
// time that the RSS value of all axes must be less than
// free_fall_threshold to generate a Free-Fall interrupt. The
// scale factor is 5 ms/LSB. A zero value may result in
// undesirable behavior if Free-Fall interrupt is enabled.
// Values between 100 to 350 ms (0x14 to 0x46) are recommended.
//
    pub free_fall_time: u8,
//
// data_rate:
// Selects device bandwidth and output data rate.
// RATE = 3200 Hz / (2^(15 - x)). Default value is 0x0A, or 100 Hz
// Output Data Rate. An Output Data Rate should be selected that
// is appropriate for the communication protocol and frequency
// selected. Selecting too high of an Output Data Rate with a low
// communication speed will result in samples being discarded.
//
    pub data_rate: u8,
//
// data_range:
// FULL_RES: When this bit is set with the device is
// in Full-Resolution Mode, where the output resolution increases
// with RANGE to maintain a 4 mg/LSB scale factor. When this
// bit is cleared the device is in 10-bit Mode and RANGE determine the
// maximum g-Range and scale factor.
//

pub const ADXL_RANGE_PM_2g: c_int = 0;
pub const ADXL_RANGE_PM_4g: c_int = 1;
pub const ADXL_RANGE_PM_8g: c_int = 2;
pub const ADXL_RANGE_PM_16g: c_int = 3;
    pub data_range: u8,
//
// low_power_mode:
// A '0' = Normal operation and a '1' = Reduced
// power operation with somewhat higher noise.
//
    pub low_power_mode: u8,
//
// power_mode:
// LINK: A '1' with both the activity and inactivity functions
// enabled will delay the start of the activity function until
// inactivity is detected. Once activity is detected, inactivity
// detection will begin and prevent the detection of activity. This
// bit serially links the activity and inactivity functions. When '0'
// the inactivity and activity functions are concurrent. Additional
// information can be found in the ADXL34x datasheet's Application
// section under Link Mode.
// AUTO_SLEEP: A '1' sets the ADXL34x to switch to Sleep Mode
// when inactivity (acceleration has been below inactivity_threshold
// for at least inactivity_time) is detected and the LINK bit is set.
// A '0' disables automatic switching to Sleep Mode. See the
// Sleep Bit section of the ADXL34x datasheet for more information.
//

    pub power_mode: u8,
//
// fifo_mode:
// BYPASS The FIFO is bypassed
// FIFO   FIFO collects up to 32 values then stops collecting data
// STREAM FIFO holds the last 32 data values. Once full, the FIFO's
// oldest data is lost as it is replaced with newer data
//
// DEFAULT should be ADXL_FIFO_STREAM
//
pub const ADXL_FIFO_BYPASS: c_int = 0;
pub const ADXL_FIFO_FIFO: c_int = 1;
pub const ADXL_FIFO_STREAM: c_int = 2;
    pub fifo_mode: u8,
//
// watermark:
// The Watermark feature can be used to reduce the interrupt load
// of the system. The FIFO fills up to the value stored in watermark
// [1..32] and then generates an interrupt.
// A '0' disables the watermark feature.
//
    pub watermark: u8,
//
// When acceleration measurements are received from the ADXL34x
// events are sent to the event subsystem. The following settings
// select the event type and event code for new x, y and z axis data
// respectively.
//
    pub /: *mut *mut u32 ev_type; / EV_ABS or EV_REL,
    pub /: *mut *mut u32 ev_code_x; / ABS_X,Y,Z or REL_X,Y,Z,
    pub /: *mut *mut u32 ev_code_y; / ABS_X,Y,Z or REL_X,Y,Z,
    pub /: *mut *mut u32 ev_code_z; / ABS_X,Y,Z or REL_X,Y,Z,
//
// A valid BTN or KEY Code; use tap_axis_control to disable
// event reporting
//
    pub /: *mut *mut u32 ev_code_tap[3]; / EV_KEY {X-Axis, Y-Axis, Z-Axis},
//
// A valid BTN or KEY Code for Free-Fall or Activity enables
// input event reporting. A '0' disables the Free-Fall or
// Activity reporting.
//
    pub /: *mut *mut u32 ev_code_ff; / EV_KEY,
    pub /: *mut *mut u32 ev_code_act_inactivity; / EV_KEY,
//
// Use ADXL34x INT2 pin instead of INT1 pin for interrupt output
//
    pub use_int2: u8,
//
// ADXL346 only ORIENTATION SENSING feature
// The orientation function of the ADXL346 reports both 2-D and
// 3-D orientation concurrently.
//
pub const ADXL_EN_ORIENTATION_2D: c_int = 1;
pub const ADXL_EN_ORIENTATION_3D: c_int = 2;
pub const ADXL_EN_ORIENTATION_2D_3D: c_int = 3;
    pub orientation_enable: u8,
//
// The width of the deadzone region between two or more
// orientation positions is determined by setting the Deadzone
// value. The deadzone region size can be specified with a
// resolution of 3.6deg. The deadzone angle represents the total
// angle where the orientation is considered invalid.
//

    pub deadzone_angle: u8,
//
// To eliminate most human motion such as walking or shaking,
// a Divisor value should be selected to effectively limit the
// orientation bandwidth. Set the depth of the filter used to
// low-pass filter the measured acceleration for stable
// orientation sensing
//
pub const ADXL_LP_FILTER_DIVISOR_2: c_int = 0;
pub const ADXL_LP_FILTER_DIVISOR_4: c_int = 1;
pub const ADXL_LP_FILTER_DIVISOR_8: c_int = 2;
pub const ADXL_LP_FILTER_DIVISOR_16: c_int = 3;
pub const ADXL_LP_FILTER_DIVISOR_32: c_int = 4;
pub const ADXL_LP_FILTER_DIVISOR_64: c_int = 5;
pub const ADXL_LP_FILTER_DIVISOR_128: c_int = 6;
pub const ADXL_LP_FILTER_DIVISOR_256: c_int = 7;
    pub divisor_length: u8,
    pub /: *mut *mut u32 ev_codes_orient_2d[4]; / EV_KEY {+X, -X, +Y, -Y},
    pub /: *mut *mut u32 ev_codes_orient_3d[6]; / EV_KEY {+Z, +Y, +X, -X, -Y, -Z},
}
