//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/platform/cznic/turris-omnia-mcu.h
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
// CZ.NIC's Turris Omnia MCU driver
//
// 2024 by Marek Behún <kabel@kernel.org>
//

//
// struct omnia_mcu - driver private data structure
// @client:			I2C client
// @type:			MCU type (STM32, GD32, MKL, or unknown)
// @features:			bitmap of features supported by the MCU firmware
// @board_serial_number:	board serial number, if stored in MCU
// @board_first_mac:		board first MAC address, if stored in MCU
// @board_revision:		board revision, if stored in MCU
// @gc:				GPIO chip
// @lock:			mutex to protect internal GPIO chip state
// @mask:			bitmap of masked IRQs
// @rising:			bitmap of rising edge IRQs
// @falling:			bitmap of falling edge IRQs
// @both:			bitmap of both edges IRQs
// @cached:			bitmap of cached IRQ line values (when an IRQ line is configured for
// both edges, we cache the corresponding GPIO values in the IRQ
// handler)
// @is_cached:			bitmap of which IRQ line values are cached
// @button_release_emul_work:	front button release emulation work, used with old MCU firmware
// versions which did not send button release events, only button press
// events
// @last_status:		cached value of the status word, to be compared with new value to
// determine which interrupt events occurred, used with old MCU
// firmware versions which only informed that the status word changed,
// but not which bits of the status word changed
// @button_pressed_emul:	the front button is still emulated to be pressed
// @rtcdev:			RTC device, does not actually count real-time, the device is only
// used for the RTC alarm mechanism, so that the board can be
// configured to wake up from poweroff state at a specific time
// @rtc_alarm:			RTC alarm that was set for the board to wake up on, in MCU time
// (seconds since last MCU reset)
// @front_button_poweron:	the front button should power on the device after it is powered off
// @wdt:			watchdog driver structure
// @trng:			RNG driver structure
// @trng_entropy_ready:		RNG entropy ready completion
// @msg_signed:			message signed completion
// @sign_lock:			mutex to protect message signing state
// @sign_requested:		flag indicating that message signing was requested but not completed
// @sign_err:			message signing error number, filled in interrupt handler
// @signature:			message signing signature, filled in interrupt handler
// @board_public_key:		board public key, if stored in MCU
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct omnia_mcu {
    pub client: *mut i2c_client,
    pub type: *const c_char,
    pub features: u32,
    pub board_serial_number: u64,
    pub board_first_mac: [u8; ETH_ALEN],
    pub board_revision: u8,

    pub gc: gpio_chip,
    pub lock: mutex,
    pub is_cached: unsigned long mask, rising, falling, both, cached,,
    pub button_release_emul_work: delayed_work,
    pub last_status: c_ulong,
    pub button_pressed_emul: bool,

    pub rtcdev: *mut rtc_device,
    pub rtc_alarm: u32,
    pub front_button_poweron: bool,

    pub wdt: watchdog_device,

    pub trng: hwrng,
    pub trng_entropy_ready: completion,

    pub msg_signed: completion,
    pub sign_lock: mutex,
    pub sign_requested: bool,
    pub sign_err: c_int,
    pub signature: [u8; OMNIA_MCU_CRYPTO_SIGNATURE_LEN],
    pub board_public_key: [u8; OMNIA_MCU_CRYPTO_PUBLIC_KEY_LEN],
}

extern "C" {
    pub fn omnia_mcu_register_gpiochip(mcu: *mut omnia_mcu) -> c_int;
}

extern "C" {
    pub fn omnia_mcu_register_keyctl(mcu: *mut omnia_mcu) -> c_int;
}

extern "C" {
    pub fn omnia_mcu_register_sys_off_and_wakeup(mcu: *mut omnia_mcu) -> c_int;
}

extern "C" {
    pub fn omnia_mcu_register_trng(mcu: *mut omnia_mcu) -> c_int;
}

extern "C" {
    pub fn omnia_mcu_register_watchdog(mcu: *mut omnia_mcu) -> c_int;
}

