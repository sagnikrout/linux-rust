//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pwm.h
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
// enum pwm_polarity - polarity of a PWM signal
// @PWM_POLARITY_NORMAL: a high signal for the duration of the duty-
// cycle, followed by a low signal for the remainder of the pulse
// period
// @PWM_POLARITY_INVERSED: a low signal for the duration of the duty-
// cycle, followed by a high signal for the remainder of the pulse
// period
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pwm_polarity {
    PWM_POLARITY_NORMAL,
    PWM_POLARITY_INVERSED,
}

//
// struct pwm_args - board-dependent PWM arguments
// @period: reference period
// @polarity: reference polarity
//
// This structure describes board-dependent arguments attached to a PWM
// device. These arguments are usually retrieved from the PWM lookup table or
// device tree.
//
// Do not confuse this with the PWM state: PWM arguments represent the initial
// configuration that users want to use on this PWM device rather than the
// current PWM hardware state.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pwm_args {
    pub period: u64,
    pub polarity: pwm_polarity,
}

//
// struct pwm_waveform - description of a PWM waveform
// @period_length_ns: PWM period
// @duty_length_ns: PWM duty cycle
// @duty_offset_ns: offset of the rising edge from the period's start
//
// This is a representation of a PWM waveform alternative to struct pwm_state
// below. It's more expressive than struct pwm_state as it contains a
// duty_offset_ns and so can represent offsets other than zero (with .polarity =
// PWM_POLARITY_NORMAL) and period - duty_cycle (.polarity =
// PWM_POLARITY_INVERSED).
//
// Note there is no explicit bool for enabled. A "disabled" PWM is represented
// by .period_length_ns = 0. Note further that the behaviour of a "disabled" PWM
// is undefined. Depending on the hardware's capabilities it might drive the
// active or inactive level, go high-z or even continue to toggle.
//
// The unit for all three members is nanoseconds.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pwm_waveform {
    pub period_length_ns: u64,
    pub duty_length_ns: u64,
    pub duty_offset_ns: u64,
}

//
// struct pwm_state - state of a PWM channel
// @period: PWM period (in nanoseconds)
// @duty_cycle: PWM duty cycle (in nanoseconds)
// @polarity: PWM polarity
// @enabled: PWM enabled status
// @usage_power: If set, the PWM driver is only required to maintain the power
// output but has more freedom regarding signal form.
// If supported, the signal can be optimized, for example to
// improve EMI by phase shifting individual channels.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pwm_state {
    pub period: u64,
    pub duty_cycle: u64,
    pub polarity: pwm_polarity,
    pub enabled: bool,
    pub usage_power: bool,
}

//
// struct pwm_device - PWM channel object
// @label: name of the PWM device
// @flags: flags associated with the PWM device
// @hwpwm: per-chip relative index of the PWM device
// @chip: PWM chip providing this PWM device
// @args: PWM arguments
// @state: last applied state
// @last: last implemented state (for PWM_DEBUG)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pwm_device {
    pub label: *const c_char,
    pub flags: c_ulong,
    pub hwpwm: c_uint,
    pub chip: *mut pwm_chip,
    pub args: pwm_args,
    pub state: pwm_state,
    pub last: pwm_state,
}

//
// pwm_get_state() - retrieve the current PWM state
// @pwm: PWM device
// @state: state to fill with the current PWM state
//
// The returned PWM state represents the state that was applied by a previous call to
// pwm_apply_might_sleep(). Drivers may have to slightly tweak that state before programming it to
// hardware. If pwm_apply_might_sleep() was never called, this returns either the current hardware
// state (if supported) or the default settings.
//
// state = pwm->state;
// args = pwm->args;
//
// pwm_init_state() - prepare a new state to be applied with pwm_apply_might_sleep()
// @pwm: PWM device
// @state: state to fill with the prepared PWM state
//
// This functions prepares a state that can later be tweaked and applied
// to the PWM device with pwm_apply_might_sleep(). This is a convenient function
// that first retrieves the current PWM state and the replaces the period
// and polarity fields with the reference values defined in pwm->args.
// Once the function returns, you can adjust the ->enabled and ->duty_cycle
// fields according to your needs before calling pwm_apply_might_sleep().
//
// ->duty_cycle is initially set to zero to avoid cases where the current
// ->duty_cycle value exceed the pwm_args->period one, which would trigger
// an error if the user calls pwm_apply_might_sleep() without adjusting ->duty_cycle
// first.
//
// First get the current state.
// Then fill it with the reference config
//
// pwm_get_relative_duty_cycle() - Get a relative duty cycle value
// @state: PWM state to extract the duty cycle from
// @scale: target scale of the relative duty cycle
//
// This functions converts the absolute duty cycle stored in @state (expressed
// in nanosecond) into a value relative to the period.
//
// For example if you want to get the duty_cycle expressed in percent, call:
//
// pwm_get_state(pwm, &state);
// duty = pwm_get_relative_duty_cycle(&state, 100);
//
// Returns: rounded relative duty cycle multiplied by @scale
//
// pwm_set_relative_duty_cycle() - Set a relative duty cycle value
// @state: PWM state to fill
// @duty_cycle: relative duty cycle value
// @scale: scale in which @duty_cycle is expressed
//
// This functions converts a relative into an absolute duty cycle (expressed
// in nanoseconds), and puts the result in state->duty_cycle.
//
// For example if you want to configure a 50% duty cycle, call:
//
// pwm_init_state(pwm, &state);
// pwm_set_relative_duty_cycle(&state, 50, 100);
// pwm_apply_might_sleep(pwm, &state);
//
// Returns: 0 on success or ``-EINVAL`` if @duty_cycle and/or @scale are
// inconsistent (@scale == 0 or @duty_cycle > @scale)
//
// struct pwm_capture - PWM capture data
// @period: period of the PWM signal (in nanoseconds)
// @duty_cycle: duty cycle of the PWM signal (in nanoseconds)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pwm_capture {
    pub period: c_uint,
    pub duty_cycle: c_uint,
}

pub const PWM_WFHWSIZE: c_int = 20;
//
// struct pwm_ops - PWM controller operations
// @request: optional hook for requesting a PWM
// @free: optional hook for freeing a PWM
// @capture: capture and report PWM signal
// @sizeof_wfhw: size (in bytes) of driver specific waveform presentation
// @round_waveform_tohw: convert a struct pwm_waveform to driver specific presentation
// @round_waveform_fromhw: convert a driver specific waveform presentation to struct pwm_waveform
// @read_waveform: read driver specific waveform presentation from hardware
// @write_waveform: write driver specific waveform presentation to hardware
// @apply: atomically apply a new PWM config
// @get_state: get the current PWM state.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pwm_ops {
    pub pwm): *mut *mut *mut int (request)(struct pwm_chip chip, struct pwm_device,
    pub pwm): *mut *mut *mut void (free)(struct pwm_chip chip, struct pwm_device,
    pub timeout): *mut *mut pwm_capture result, unsigned long,
    pub sizeof_wfhw: usize,
    pub wfhw): *const *const pwm_waveform wf, void,
    pub wf): *const *const void wfhw, struct pwm_waveform,
    pub wfhw): *mut c_void,
    pub wfhw): *const c_void,
    pub state): *const pwm_state,
    pub state): *mut pwm_state,
}

//
// struct pwm_chip - abstract a PWM controller
// @dev: device providing the PWMs
// @cdev: &struct cdev for this device
// @ops: callbacks for this PWM controller
// @owner: module providing this chip
// @id: unique number of this PWM chip
// @npwm: number of PWMs controlled by this chip
// @of_xlate: request a PWM device given a device tree PWM specifier
// @atomic: can the driver's ->apply() be called in atomic context
// @gpio: &struct gpio_chip to operate this PWM chip's lines as GPO
// @uses_pwmchip_alloc: signals if pwmchip_allow was used to allocate this chip
// @operational: signals if the chip can be used (or is already deregistered)
// @nonatomic_lock: mutex for nonatomic chips
// @atomic_lock: mutex for atomic chips
// @pwms: array of PWM devices allocated by the framework
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pwm_chip {
    pub dev: device,
    pub cdev: cdev,
    pub ops: *const pwm_ops,
    pub owner: *mut module,
    pub id: c_uint,
    pub npwm: c_uint,
    pub args): *const of_phandle_args,
    pub atomic: bool,
// only used internally by the PWM framework
    pub gpio: gpio_chip,
    pub uses_pwmchip_alloc: bool,
    pub operational: bool,
//
// depending on the chip being atomic or not either the mutex or
// the spinlock is used. It protects .operational and
// synchronizes the callbacks in .ops
//
    pub nonatomic_lock: mutex,
    pub atomic_lock: spinlock_t,
}

//
// pwmchip_supports_waveform() - checks if the given chip supports waveform callbacks
// @chip: The pwm_chip to test
//
// Returns: true iff the pwm chip support the waveform functions like
// pwm_set_waveform_might_sleep() and pwm_round_waveform_might_sleep()
//
// only check for .write_waveform(). If that is available,
// .round_waveform_tohw() and .round_waveform_fromhw() asserted to be
// available, too, in pwmchip_add().
//
extern "C" {
    pub fn dev_get_drvdata(_arg: &chip->dev) -> return;
}

// PWM consumer APIs
extern "C" {
    pub fn pwm_round_waveform_might_sleep(pwm: *mut pwm_device, wf: *mut pwm_waveform) -> c_int;
}
extern "C" {
    pub fn pwm_get_waveform_might_sleep(pwm: *mut pwm_device, wf: *mut pwm_waveform) -> c_int;
}
extern "C" {
    pub fn pwm_set_waveform_might_sleep(pwm: *mut pwm_device, wf: *const pwm_waveform, exact: bool) -> c_int;
}
extern "C" {
    pub fn pwm_apply_might_sleep(pwm: *mut pwm_device, state: *const pwm_state) -> c_int;
}
extern "C" {
    pub fn pwm_apply_atomic(pwm: *mut pwm_device, state: *const pwm_state) -> c_int;
}
extern "C" {
    pub fn pwm_get_state_hw(pwm: *mut pwm_device, state: *mut pwm_state) -> c_int;
}
extern "C" {
    pub fn pwm_adjust_config(pwm: *mut pwm_device) -> c_int;
}
//
// pwm_config() - change a PWM device configuration
// @pwm: PWM device
// @duty_ns: "on" time (in nanoseconds)
// @period_ns: duration (in nanoseconds) of one cycle
//
// Returns: 0 on success or a negative error code on failure.
//
extern "C" {
    pub fn pwm_apply_might_sleep(_arg: pwm, _arg: &state) -> return;
}
//
// pwm_enable() - start a PWM output toggling
// @pwm: PWM device
//
// Returns: 0 on success or a negative error code on failure.
//
extern "C" {
    pub fn pwm_apply_might_sleep(_arg: pwm, _arg: &state) -> return;
}
//
// pwm_disable() - stop a PWM output toggling
// @pwm: PWM device
//
// pwm_might_sleep() - is pwm_apply_atomic() supported?
// @pwm: PWM device
//
// Returns: false if pwm_apply_atomic() can be called from atomic context.
//
// PWM provider APIs
extern "C" {
    pub fn pwmchip_put(chip: *mut pwm_chip);
}
extern "C" {
    pub fn __pwmchip_add(chip: *mut pwm_chip, owner: *mut module) -> c_int;
}

extern "C" {
    pub fn pwmchip_remove(chip: *mut pwm_chip);
}
//
// For FFI wrapper use only:
// The Rust PWM abstraction needs this to properly free the pwm_chip.
//
extern "C" {
    pub fn pwmchip_release(dev: *mut device);
}
extern "C" {
    pub fn __devm_pwmchip_add(dev: *mut device, chip: *mut pwm_chip, owner: *mut module) -> c_int;
}

extern "C" {
    pub fn pwm_put(pwm: *mut pwm_device);
}

extern "C" {
    pub fn ERR_PTR(_arg: -EINVAL) -> return;
}
extern "C" {
    pub fn pwmchip_alloc(_arg: parent, _arg: npwm, _arg: sizeof_priv) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pwm_lookup {
    pub list: list_head,
    pub provider: *const c_char,
    pub index: c_uint,
    pub dev_id: *const c_char,
    pub con_id: *const c_char,
    pub period: c_uint,
    pub polarity: pwm_polarity,
    pub /: *const *const *const char module; / optional, may be NULL,
}

extern "C" {
    pub fn pwm_add_table(table: *mut pwm_lookup, num: usize);
}
extern "C" {
    pub fn pwm_remove_table(table: *mut pwm_lookup, num: usize);
}

