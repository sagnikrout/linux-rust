//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ptp_clock_kernel.h
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
// PTP 1588 clock support
//
// Copyright (C) 2010 OMICRON electronics GmbH
//

pub const PTP_CLOCK_NAME_LEN: c_int = 32;
//
// struct ptp_clock_request - request PTP clock event
//
// @type:   The type of the request.
// EXTTS:  Configure external trigger timestamping
// PEROUT: Configure periodic output signal (e.g. PPS)
// PPS:    trigger internal PPS event for input
// into kernel PPS subsystem
// @extts:  describes configuration for external trigger timestamping.
// This is only valid when event == PTP_CLK_REQ_EXTTS.
// @perout: describes configuration for periodic output.
// This is only valid when event == PTP_CLK_REQ_PEROUT.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptp_clock_request {
    pub type: },
    pub extts: ptp_extts_request,
    pub perout: ptp_perout_request,
}

//
// struct ptp_system_timestamp - system time corresponding to a PHC timestamp
// @pre_sts:	system time snapshot before capturing PHC
// @post_sts:	system time snapshot after capturing PHC
// @clockid:	clock-base used for capturing the system timestamps
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptp_system_timestamp {
    pub pre_sts: system_time_snapshot,
    pub post_sts: system_time_snapshot,
    pub clockid: clockid_t,
}

//
// struct ptp_clock_info - describes a PTP hardware clock
//
// @owner:     The clock driver should set to THIS_MODULE.
// @name:      A short "friendly name" to identify the clock and to
// help distinguish PHY based devices from MAC based ones.
// The string is not meant to be a unique id.
// @max_adj:   The maximum possible frequency adjustment, in parts per billon.
// @n_alarm:   The number of programmable alarms.
// @n_ext_ts:  The number of external time stamp channels.
// @n_per_out: The number of programmable periodic signals.
// @n_pins:    The number of programmable pins.
// @n_per_lp:  The number of channels that support loopback the periodic
// output signal.
// @pps:       Indicates whether the clock supports a PPS callback.
//
// @supported_perout_flags:  The set of flags the driver supports for the
// PTP_PEROUT_REQUEST ioctl. The PTP core will
// reject a request with any flag not specified
// here.
//
// @supported_extts_flags:  The set of flags the driver supports for the
// PTP_EXTTS_REQUEST ioctl. The PTP core will use
// this list to reject unsupported requests.
// PTP_ENABLE_FEATURE is assumed and does not need to
// be included. If PTP_STRICT_FLAGS is *not* set,
// then both PTP_RISING_EDGE and PTP_FALLING_EDGE
// will be assumed. Note that PTP_STRICT_FLAGS must
// be set if the drivers wants to honor
// PTP_EXTTS_REQUEST2 and any future flags.
//
// @pin_config: Array of length 'n_pins'. If the number of
// programmable pins is nonzero, then drivers must
// allocate and initialize this array.
//
// clock operations
//
// @adjfine:  Adjusts the frequency of the hardware clock.
// parameter scaled_ppm: Desired frequency offset from
// nominal frequency in parts per million, but with a
// 16 bit binary fractional field.
//
// @adjphase:  Indicates that the PHC should use an internal servo
// algorithm to correct the provided phase offset.
// parameter delta: PHC servo phase adjustment target
// in nanoseconds.
//
// @getmaxphase:  Advertises maximum offset that can be provided
// to the hardware clock's phase control functionality
// through adjphase.
//
// @adjtime:  Shifts the time of the hardware clock.
// parameter delta: Desired change in nanoseconds.
//
// @gettime64:  Reads the current time from the hardware clock.
// This method is deprecated.  New drivers should implement
// the @gettimex64 method instead.
// parameter ts: Holds the result.
//
// @gettimex64:  Reads the current time from the hardware clock and optionally
// also the system clock.
// parameter ts: Holds the PHC timestamp.
// parameter sts: If not NULL, it holds a pair of timestamps from
// the system clock. The first reading is made right before
// reading the lowest bits of the PHC timestamp and the second
// reading immediately follows that.
//
// @getcrosststamp:  Reads the current time from the hardware clock and
// system clock simultaneously.
// parameter cts: Contains timestamp (device,system) pair,
// where system time is realtime and monotonic.
//
// @settime64:  Set the current time on the hardware clock.
// parameter ts: Time value to set.
//
// @getcycles64:  Reads the current free running cycle counter from the hardware
// clock.
// If @getcycles64 and @getcyclesx64 are not supported, then
// @gettime64 or @gettimex64 will be used as default
// implementation.
// parameter ts: Holds the result.
//
// @getcyclesx64:  Reads the current free running cycle counter from the
// hardware clock and optionally also the system clock.
// If @getcycles64 and @getcyclesx64 are not supported, then
// @gettimex64 will be used as default implementation if
// available.
// parameter ts: Holds the PHC timestamp.
// parameter sts: If not NULL, it holds a pair of timestamps
// from the system clock. The first reading is made right before
// reading the lowest bits of the PHC timestamp and the second
// reading immediately follows that.
//
// @getcrosscycles:  Reads the current free running cycle counter from the
// hardware clock and system clock simultaneously.
// If @getcycles64 and @getcyclesx64 are not supported, then
// @getcrosststamp will be used as default implementation if
// available.
// parameter cts: Contains timestamp (device,system) pair,
// where system time is realtime and monotonic.
//
// @enable:   Request driver to enable or disable an ancillary feature.
// parameter request: Desired resource to enable or disable.
// parameter on: Caller passes one to enable or zero to disable.
//
// @verify:   Confirm that a pin can perform a given function. The PTP
// Hardware Clock subsystem maintains the 'pin_config'
// array on behalf of the drivers, but the PHC subsystem
// assumes that every pin can perform every function. This
// hook gives drivers a way of telling the core about
// limitations on specific pins. This function must return
// zero if the function can be assigned to this pin, and
// nonzero otherwise.
// parameter pin: index of the pin in question.
// parameter func: the desired function to use.
// parameter chan: the function channel index to use.
//
// @do_aux_work:  Request driver to perform auxiliary (periodic) operations
// Driver should return delay of the next auxiliary work
// scheduling time (>=0) or negative value in case further
// scheduling is not required.
//
// @perout_loopback: Request driver to enable or disable the periodic output
// signal loopback.
// parameter index: index of the periodic output signal channel.
// parameter on: caller passes one to enable or zero to disable.
//
// Drivers should embed their ptp_clock_info within a private
// structure, obtaining a reference to it using container_of().
//
// The callbacks must all return zero on success, non-zero otherwise.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptp_clock_info {
    pub owner: *mut module,
    pub name: [c_char; PTP_CLOCK_NAME_LEN],
    pub max_adj: i32,
    pub n_alarm: c_int,
    pub n_ext_ts: c_int,
    pub n_per_out: c_int,
    pub n_pins: c_int,
    pub n_per_lp: c_int,
    pub pps: c_int,
    pub supported_perout_flags: c_uint,
    pub supported_extts_flags: c_uint,
    pub pin_config: *mut ptp_pin_desc,
    pub scaled_ppm): *mut *mut *mut int (adjfine)(struct ptp_clock_info ptp, long,
    pub phase): *mut *mut *mut int (adjphase)(struct ptp_clock_info ptp, s32,
    pub ptp): *mut *mut s32 (getmaxphase)(struct ptp_clock_info,
    pub delta): *mut *mut *mut int (adjtime)(struct ptp_clock_info ptp, s64,
    pub ts): *mut *mut *mut int (gettime64)(struct ptp_clock_info ptp, struct timespec64,
    pub sts): *mut ptp_system_timestamp,
    pub cts): *mut system_device_crosststamp,
    pub ts): *const *const *const int (settime64)(struct ptp_clock_info p, struct timespec64,
    pub ts): *mut *mut *mut int (getcycles64)(struct ptp_clock_info ptp, struct timespec64,
    pub sts): *mut ptp_system_timestamp,
    pub cts): *mut system_device_crosststamp,
    pub on): *mut *mut ptp_clock_request request, int,
    pub chan): ptp_pin_function func, unsigned int,
    pub ptp): *mut *mut long (do_aux_work)(struct ptp_clock_info,
    pub on): c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ptp_clock_events {
    PTP_CLOCK_ALARM,
    PTP_CLOCK_EXTTS,
    PTP_CLOCK_EXTOFF,
    PTP_CLOCK_PPS,
    PTP_CLOCK_PPSUSR,
}

//
// struct ptp_clock_event - decribes a PTP hardware clock event
//
// @type:  One of the ptp_clock_events enumeration values.
// @index: Identifies the source of the event.
// @timestamp: When the event occurred (%PTP_CLOCK_EXTTS only).
// @offset:    When the event occurred (%PTP_CLOCK_EXTOFF only).
// @pps_times: When the event occurred (%PTP_CLOCK_PPSUSR only).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ptp_clock_event {
    pub type: c_int,
    pub index: c_int,
    pub timestamp: u64,
    pub offset: i64,
    pub pps_times: pps_event_time,
}

//
// scaled_ppm_to_ppb() - convert scaled ppm to ppb
//
// @ppm:    Parts per million, but with a 16 bit binary fractional field
//
// The 'freq' field in the 'struct timex' is in parts per
// million, but with a 16 bit binary fractional field.
//
// We want to calculate
//
// ppb = scaled_ppm * 1000 / 2^16
//
// which simplifies to
//
// ppb = scaled_ppm * 125 / 2^13
//
// diff_by_scaled_ppm - Calculate difference using scaled ppm
// @base: the base increment value to adjust
// @scaled_ppm: scaled parts per million to adjust by
// @diff: on return, the absolute value of calculated diff
//
// Calculate the difference to adjust the base increment using scaled parts
// per million.
//
// Use mul_u64_u64_div_u64 to perform the difference calculation in avoid
// possible overflow.
//
// Returns: true if scaled_ppm is negative, false otherwise
//
// diff = mul_u64_u64_div_u64(base, (u64)scaled_ppm, 1000000ULL << 16);
//
// adjust_by_scaled_ppm - Adjust a base increment by scaled parts per million
// @base: the base increment value to adjust
// @scaled_ppm: scaled parts per million frequency adjustment
//
// Helper function which calculates a new increment value based on the
// requested scaled parts per million adjustment.
//

//
// ptp_clock_register() - register a PTP hardware clock driver
//
// @info:   Structure describing the new clock.
// @parent: Pointer to the parent device of the new clock.
//
// Returns: a valid pointer on success or PTR_ERR on failure.  If PHC
// support is missing at the configuration level, this function
// returns NULL, and drivers are expected to gracefully handle that
// case separately.
//
// ptp_clock_unregister() - unregister a PTP hardware clock driver
//
// @ptp:  The clock to remove from service.
//
extern "C" {
    pub fn ptp_clock_unregister(ptp: *mut ptp_clock) -> c_int;
}
//
// ptp_clock_event() - notify the PTP layer about an event
//
// @ptp:    The clock obtained from ptp_clock_register().
// @event:  Message structure describing the event.
//
// ptp_clock_index() - obtain the device index of a PTP clock
//
// @ptp:    The clock obtained from ptp_clock_register().
//
extern "C" {
    pub fn ptp_clock_index(ptp: *mut ptp_clock) -> c_int;
}
//
// ptp_clock_index_by_of_node() - obtain the device index of
// a PTP clock based on the PTP device of_node
//
// @np:    The device of_node pointer of the PTP device.
// Return: The PHC index on success or -1 on failure.
//
extern "C" {
    pub fn ptp_clock_index_by_of_node(np: *mut device_node) -> c_int;
}
//
// ptp_clock_index_by_dev() - obtain the device index of
// a PTP clock based on the PTP device.
//
// @parent:    The parent device (PTP device) pointer of the PTP clock.
// Return: The PHC index on success or -1 on failure.
//
extern "C" {
    pub fn ptp_clock_index_by_dev(parent: *mut device) -> c_int;
}
//
// ptp_find_pin() - obtain the pin index of a given auxiliary function
//
// The caller must hold ptp_clock::pincfg_mux.  Drivers do not have
// access to that mutex as ptp_clock is an opaque type.  However, the
// core code acquires the mutex before invoking the driver's
// ptp_clock_info::enable() callback, and so drivers may call this
// function from that context.
//
// @ptp:    The clock obtained from ptp_clock_register().
// @func:   One of the ptp_pin_function enumerated values.
// @chan:   The particular functional channel to find.
// Return:  Pin index in the range of zero to ptp_clock_caps.n_pins - 1,
// or -1 if the auxiliary function cannot be found.
//
// ptp_find_pin_unlocked() - wrapper for ptp_find_pin()
//
// This function acquires the ptp_clock::pincfg_mux mutex before
// invoking ptp_find_pin().  Instead of using this function, drivers
// should most likely call ptp_find_pin() directly from their
// ptp_clock_info::enable() method.
//
// @ptp:    The clock obtained from ptp_clock_register().
// @func:   One of the ptp_pin_function enumerated values.
// @chan:   The particular functional channel to find.
// Return:  Pin index in the range of zero to ptp_clock_caps.n_pins - 1,
// or -1 if the auxiliary function cannot be found.
//
// ptp_schedule_worker() - schedule ptp auxiliary work
//
// @ptp:    The clock obtained from ptp_clock_register().
// @delay:  number of jiffies to wait before queuing
// See kthread_queue_delayed_work() for more info.
//
extern "C" {
    pub fn ptp_schedule_worker(ptp: *mut ptp_clock, delay: c_ulong) -> c_int;
}
//
// ptp_cancel_worker_sync() - cancel ptp auxiliary clock
//
// @ptp:     The clock obtained from ptp_clock_register().
//
extern "C" {
    pub fn ptp_cancel_worker_sync(ptp: *mut ptp_clock);
}

//
// These are called by the network core, and don't work if PTP is in
// a loadable module.
//
// ptp_get_vclocks_index() - get all vclocks index on pclock, and
// caller is responsible to free memory
// of vclock_index
//
// @pclock_index: phc index of ptp pclock.
// @vclock_index: pointer to pointer of vclock index.
//
// return number of vclocks.
//
extern "C" {
    pub fn ptp_get_vclocks_index(pclock_index: c_int, vclock_index: *mut c_int) -> c_int;
}
//
// ptp_convert_timestamp() - convert timestamp to a ptp vclock time
//
// @hwtstamp:     timestamp
// @vclock_index: phc index of ptp vclock.
//
// Returns: converted timestamp, or 0 on error.
//
extern "C" {
    pub fn ptp_convert_timestamp(hwtstamp: *const ktime_t, vclock_index: c_int) -> ktime_t;
}

