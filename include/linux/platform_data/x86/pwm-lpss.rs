//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_data/x86/pwm-lpss.h
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
// Intel Low Power Subsystem PWM controller driver

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pwm_lpss_boardinfo {
    pub clk_rate: c_ulong,
    pub npwm: c_uint,
    pub base_unit_bits: c_ulong,
//
// NOTE:
// Intel Broxton, Apollo Lake, and Gemini Lake have different programming flow.
//
// Initial Enable or First Activation
// 1. Program the base unit and on time divisor values.
// 2. Set the software update bit.
// 3. Poll in a loop on the PWMCTRL bit until software update bit is cleared.+
// 4. Enable the PWM output by setting PWM Enable.
// 5. Repeat the above steps for the next PWM Module.
//
// Dynamic update while PWM is Enabled
// 1. Program the base unit and on-time divisor values.
// 2. Set the software update bit.
// 3. Repeat the above steps for the next PWM module.
//
// + After setting PWMCTRL register's SW update bit, hardware automatically
// deasserts the SW update bit after a brief delay. It was observed that
// setting of PWM enable is typically done via read-modify-write of the PWMCTRL
// register. If there is no/little delay between setting software update bit
// and setting enable bit via read-modify-write, it is possible that the read
// could return with software enable as 1. In that case, the last write to set
// enable to 1 could also set sw_update to 1. If this happens, sw_update gets
// stuck and the driver code can hang as it explicitly waits for sw_update bit
// to be 0 after setting the enable bit to 1. To avoid this race condition,
// SW should poll on the software update bit to make sure that it is 0 before
// doing the read-modify-write to set the enable bit to 1.
//
// Also, we noted that if sw_update bit was set in step #1 above then when it
// is set again in step #2, sw_update bit never gets cleared and the flow hangs.
// As such, we need to make sure that sw_update bit is 0 when doing step #1.
//
    pub bypass: bool,
//
// On some devices the _PS0/_PS3 AML code of the GPU (GFX0) device
// messes with the PWM0 controllers state,
//
    pub other_devices_aml_touches_pwm_regs: bool,
}
