//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/rfkill.h
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


//
// Copyright (C) 2006 - 2007 Ivo van Doorn
// Copyright (C) 2007 Dmitry Torokhov
// Copyright 2009 Johannes Berg <johannes@sipsolutions.net>
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

// don't allow anyone to use these in the kernel
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rfkill_user_states {
    RFKILL_USER_STATE_SOFT_BLOCKED	= RFKILL_STATE_SOFT_BLOCKED,
    RFKILL_USER_STATE_UNBLOCKED	= RFKILL_STATE_UNBLOCKED,
    RFKILL_USER_STATE_HARD_BLOCKED	= RFKILL_STATE_HARD_BLOCKED,
}

// this is opaque
//
// struct rfkill_ops - rfkill driver methods
//
// @poll: poll the rfkill block state(s) -- only assign this method
// when you need polling. When called, simply call one of the
// rfkill_set{,_hw,_sw}_state family of functions. If the hw
// is getting unblocked you need to take into account the return
// value of those functions to make sure the software block is
// properly used.
// @query: query the rfkill block state(s) and call exactly one of the
// rfkill_set{,_hw,_sw}_state family of functions. Assign this
// method if input events can cause hardware state changes to make
// the rfkill core query your driver before setting a requested
// block.
// @set_block: turn the transmitter on (blocked == false) or off
// (blocked == true) -- ignore and return 0 when hard blocked.
// This callback must be assigned.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rfkill_ops {
    pub data): *mut *mut *mut void (poll)(struct rfkill rfkill, void,
    pub data): *mut *mut *mut void (query)(struct rfkill rfkill, void,
    pub blocked): *mut *mut *mut int (set_block)(void data, bool,
}

//
// rfkill_alloc - Allocate rfkill structure
// @name: name of the struct -- the string is not copied internally
// @parent: device that has rf switch on it
// @type: type of the switch (RFKILL_TYPE_*)
// @ops: rfkill methods
// @ops_data: data passed to each method
//
// This function should be called by the transmitter driver to allocate an
// rfkill structure. Returns %NULL on failure.
//
// rfkill_register - Register a rfkill structure.
// @rfkill: rfkill structure to be registered
//
// This function should be called by the transmitter driver to register
// the rfkill structure. Before calling this function the driver needs
// to be ready to service method calls from rfkill.
//
// If rfkill_init_sw_state() is not called before registration,
// set_block() will be called to initialize the software blocked state
// to a default value.
//
// If the hardware blocked state is not set before registration,
// it is assumed to be unblocked.
//
extern "C" {
    pub fn rfkill_register(rfkill: *mut rfkill) -> int __must_check;
}
//
// rfkill_pause_polling - Pause polling
// @rfkill: rfkill struct
//
// Pause polling -- say transmitter is off for other reasons.
// NOTE: not necessary for suspend/resume -- in that case the
// core stops polling anyway (but will also correctly handle
// the case of polling having been paused before suspend.)
//
extern "C" {
    pub fn rfkill_pause_polling(rfkill: *mut rfkill);
}
//
// rfkill_resume_polling - Resume polling
// @rfkill: rfkill struct
//
// NOTE: not necessary for suspend/resume -- in that case the
// core stops polling anyway
//
extern "C" {
    pub fn rfkill_resume_polling(rfkill: *mut rfkill);
}
//
// rfkill_unregister - Unregister a rfkill structure.
// @rfkill: rfkill structure to be unregistered
//
// This function should be called by the network driver during device
// teardown to destroy rfkill structure. Until it returns, the driver
// needs to be able to service method calls.
//
extern "C" {
    pub fn rfkill_unregister(rfkill: *mut rfkill);
}
//
// rfkill_destroy - Free rfkill structure
// @rfkill: rfkill structure to be destroyed
//
// Destroys the rfkill structure.
//
extern "C" {
    pub fn rfkill_destroy(rfkill: *mut rfkill);
}
//
// rfkill_set_hw_state_reason - Set the internal rfkill hardware block state
// with a reason
// @rfkill: pointer to the rfkill class to modify.
// @blocked: the current hardware block state to set
// @reason: one of &enum rfkill_hard_block_reasons
//
// Prefer to use rfkill_set_hw_state if you don't need any special reason.
//
// rfkill_set_hw_state - Set the internal rfkill hardware block state
// @rfkill: pointer to the rfkill class to modify.
// @blocked: the current hardware block state to set
//
// rfkill drivers that get events when the hard-blocked state changes
// use this function to notify the rfkill core (and through that also
// userspace) of the current state.  They should also use this after
// resume if the state could have changed.
//
// You need not (but may) call this function if poll_state is assigned.
//
// This function can be called in any context, even from within rfkill
// callbacks.
//
// The function returns the combined block state (true if transmitter
// should be blocked) so that drivers need not keep track of the soft
// block state -- which they might not be able to.
//
// rfkill_set_sw_state - Set the internal rfkill software block state
// @rfkill: pointer to the rfkill class to modify.
// @blocked: the current software block state to set
//
// rfkill drivers that get events when the soft-blocked state changes
// (yes, some platforms directly act on input but allow changing again)
// use this function to notify the rfkill core (and through that also
// userspace) of the current state.
//
// Drivers should also call this function after resume if the state has
// been changed by the user.  This only makes sense for "persistent"
// devices (see rfkill_init_sw_state()).
//
// This function can be called in any context, even from within rfkill
// callbacks.
//
// The function returns the combined block state (true if transmitter
// should be blocked).
//
extern "C" {
    pub fn rfkill_set_sw_state(rfkill: *mut rfkill, blocked: bool) -> bool;
}
//
// rfkill_init_sw_state - Initialize persistent software block state
// @rfkill: pointer to the rfkill class to modify.
// @blocked: the current software block state to set
//
// rfkill drivers that preserve their software block state over power off
// use this function to notify the rfkill core (and through that also
// userspace) of their initial state.  It should only be used before
// registration.
//
// In addition, it marks the device as "persistent", an attribute which
// can be read by userspace.  Persistent devices are expected to preserve
// their own state when suspended.
//
extern "C" {
    pub fn rfkill_init_sw_state(rfkill: *mut rfkill, blocked: bool);
}
//
// rfkill_set_states - Set the internal rfkill block states
// @rfkill: pointer to the rfkill class to modify.
// @sw: the current software block state to set
// @hw: the current hardware block state to set
//
// This function can be called in any context, even from within rfkill
// callbacks.
//
extern "C" {
    pub fn rfkill_set_states(rfkill: *mut rfkill, sw: bool, hw: bool);
}
//
// rfkill_blocked - Query rfkill block state
//
// @rfkill: rfkill struct to query
//
extern "C" {
    pub fn rfkill_blocked(rfkill: *mut rfkill) -> bool;
}
//
// rfkill_soft_blocked - Query soft rfkill block state
//
// @rfkill: rfkill struct to query
//
extern "C" {
    pub fn rfkill_soft_blocked(rfkill: *mut rfkill) -> bool;
}
//
// rfkill_find_type - Helper for finding rfkill type by name
// @name: the name of the type
//
// Returns: enum rfkill_type that corresponds to the name.
//
extern "C" {
    pub fn rfkill_find_type(name: *const c_char) -> rfkill_type;
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}

//
// rfkill_get_led_trigger_name - Get the LED trigger name for the button's LED.
// @rfkill: rfkill struct
//
// This function might return a NULL pointer if registering of the
// LED trigger failed. Use this as "default_trigger" for the LED.
//
// rfkill_set_led_trigger_name - Set the LED trigger name
// @rfkill: rfkill struct
// @name: LED trigger name
//
// This function sets the LED trigger name of the radio LED
// trigger that rfkill creates. It is optional, but if called
// must be called before rfkill_register() to be effective.
//
extern "C" {
    pub fn rfkill_set_led_trigger_name(rfkill: *mut rfkill, name: *const c_char);
}

