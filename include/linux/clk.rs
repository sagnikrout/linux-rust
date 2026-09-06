//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/clk.h
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
// linux/include/linux/clk.h
//
// Copyright (C) 2004 ARM Limited.
// Written by Deep Blue Solutions Limited.
// Copyright (C) 2011-2012 Linaro Ltd <mturquette@linaro.org>
//

//
// DOC: clk notifier callback types
//
// PRE_RATE_CHANGE - called immediately before the clk rate is changed,
// to indicate that the rate change will proceed.  Drivers must
// immediately terminate any operations that will be affected by the
// rate change.  Callbacks may either return NOTIFY_DONE, NOTIFY_OK,
// NOTIFY_STOP or NOTIFY_BAD.
//
// ABORT_RATE_CHANGE: called if the rate change failed for some reason
// after PRE_RATE_CHANGE.  In this case, all registered notifiers on
// the clk will be called with ABORT_RATE_CHANGE. Callbacks must
// always return NOTIFY_DONE or NOTIFY_OK.
//
// POST_RATE_CHANGE - called after the clk rate change has successfully
// completed.  Callbacks must always return NOTIFY_DONE or NOTIFY_OK.
//

//
// struct clk_notifier - associate a clk with a notifier
// @clk: struct clk * to associate the notifier with
// @notifier_head: a blocking_notifier_head for this clk
// @node: linked list pointers
//
// A list of struct clk_notifier is maintained by the notifier code.
// An entry is created whenever code registers the first notifier on a
// particular @clk.  Future notifiers on that @clk are added to the
// @notifier_head.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_notifier {
    pub clk: *mut clk,
    pub notifier_head: srcu_notifier_head,
    pub node: list_head,
}

//
// struct clk_notifier_data - rate data to pass to the notifier callback
// @clk: struct clk * being changed
// @old_rate: previous rate of this clk
// @new_rate: new rate of this clk
//
// For a pre-notifier, old_rate is the clk's rate before this rate
// change, and new_rate is what the rate will be in the future.  For a
// post-notifier, old_rate and new_rate are both set to the clk's
// current rate (this was done to optimize the implementation).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_notifier_data {
    pub clk: *mut clk,
    pub old_rate: c_ulong,
    pub new_rate: c_ulong,
}

//
// struct clk_bulk_data - Data used for bulk clk operations.
//
// @id: clock consumer ID
// @clk: struct clk * to store the associated clock
//
// The CLK APIs provide a series of clk_bulk_() API calls as
// a convenience to consumers which require multiple clks.  This
// structure is used to manage data for these calls.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_bulk_data {
    pub id: *const c_char,
    pub clk: *mut clk,
}

//
// clk_notifier_register - register a clock rate-change notifier callback
// @clk: clock whose rate we are interested in
// @nb: notifier block with callback function pointer
//
// ProTip: debugging across notifier chains can be frustrating. Make sure that
// your notifier callback function prints a nice big warning in case of
// failure.
//
extern "C" {
    pub fn clk_notifier_register(clk: *mut clk, nb: *mut notifier_block) -> c_int;
}
//
// clk_notifier_unregister - unregister a clock rate-change notifier callback
// @clk: clock whose rate we are no longer interested in
// @nb: notifier block which will be unregistered
//
extern "C" {
    pub fn clk_notifier_unregister(clk: *mut clk, nb: *mut notifier_block) -> c_int;
}
//
// devm_clk_notifier_register - register a managed rate-change notifier callback
// @dev: device for clock "consumer"
// @clk: clock whose rate we are interested in
// @nb: notifier block with callback function pointer
//
// Returns 0 on success, -EERROR otherwise
//
// clk_get_accuracy - obtain the clock accuracy in ppb (parts per billion)
// for a clock source.
// @clk: clock source
//
// This gets the clock source accuracy expressed in ppb.
// A perfect clock returns 0.
//
extern "C" {
    pub fn clk_get_accuracy(clk: *mut clk) -> c_long;
}
//
// clk_set_phase - adjust the phase shift of a clock signal
// @clk: clock signal source
// @degrees: number of degrees the signal is shifted
//
// Shifts the phase of a clock signal by the specified degrees. Returns 0 on
// success, -EERROR otherwise.
//
extern "C" {
    pub fn clk_set_phase(clk: *mut clk, degrees: c_int) -> c_int;
}
//
// clk_get_phase - return the phase shift of a clock signal
// @clk: clock signal source
//
// Returns the phase shift of a clock node in degrees, otherwise returns
// -EERROR.
//
extern "C" {
    pub fn clk_get_phase(clk: *mut clk) -> c_int;
}
//
// clk_set_duty_cycle - adjust the duty cycle ratio of a clock signal
// @clk: clock signal source
// @num: numerator of the duty cycle ratio to be applied
// @den: denominator of the duty cycle ratio to be applied
//
// Adjust the duty cycle of a clock signal by the specified ratio. Returns 0 on
// success, -EERROR otherwise.
//
extern "C" {
    pub fn clk_set_duty_cycle(clk: *mut clk, num: c_uint, den: c_uint) -> c_int;
}
//
// clk_get_scaled_duty_cycle - return the duty cycle ratio of a clock signal
// @clk: clock signal source
// @scale: scaling factor to be applied to represent the ratio as an integer
//
// Returns the duty cycle ratio multiplied by the scale provided, otherwise
// returns -EERROR.
//
extern "C" {
    pub fn clk_get_scaled_duty_cycle(clk: *mut clk, scale: c_uint) -> c_int;
}
//
// clk_is_match - check if two clk's point to the same hardware clock
// @p: clk compared against q
// @q: clk compared against p
//
// Returns true if the two struct clk pointers both point to the same hardware
// clock node. Put differently, returns true if @p and @q
// share the same &struct clk_core object.
//
// Returns false otherwise. Note that two NULL clks are treated as matching.
//
extern "C" {
    pub fn clk_is_match(p: *const clk, q: *const clk) -> bool;
}
//
// clk_rate_exclusive_get - get exclusivity over the rate control of a
// producer
// @clk: clock source
//
// This function allows drivers to get exclusive control over the rate of a
// provider. It prevents any other consumer to execute, even indirectly,
// opereation which could alter the rate of the provider or cause glitches
//
// If exlusivity is claimed more than once on clock, even by the same driver,
// the rate effectively gets locked as exclusivity can't be preempted.
//
// Must not be called from within atomic context.
//
// Returns success (0) or negative errno.
//
extern "C" {
    pub fn clk_rate_exclusive_get(clk: *mut clk) -> c_int;
}
//
// devm_clk_rate_exclusive_get - devm variant of clk_rate_exclusive_get
// @dev: device the exclusivity is bound to
// @clk: clock source
//
// Calls clk_rate_exclusive_get() on @clk and registers a devm cleanup handler
// on @dev to call clk_rate_exclusive_put().
//
// Must not be called from within atomic context.
//
extern "C" {
    pub fn devm_clk_rate_exclusive_get(dev: *mut device, clk: *mut clk) -> c_int;
}
//
// clk_rate_exclusive_put - release exclusivity over the rate control of a
// producer
// @clk: clock source
//
// This function allows drivers to release the exclusivity it previously got
// from clk_rate_exclusive_get()
//
// The caller must balance the number of clk_rate_exclusive_get() and
// clk_rate_exclusive_put() calls.
//
// Must not be called from within atomic context.
//
extern "C" {
    pub fn clk_rate_exclusive_put(clk: *mut clk);
}
//
// clk_save_context - save clock context for poweroff
//
// Saves the context of the clock register for powerstates in which the
// contents of the registers will be lost. Occurs deep within the suspend
// code so locking is not necessary.
//
extern "C" {
    pub fn clk_save_context() -> c_int;
}
//
// clk_restore_context - restore clock context after poweroff
//
// This occurs with all clocks enabled. Occurs deep within the resume code
// so locking is not necessary.
//
extern "C" {
    pub fn clk_restore_context();
}

//
// clk_prepare - prepare a clock source
// @clk: clock source
//
// This prepares the clock source for use.
//
// Must not be called from within atomic context.
//
extern "C" {
    pub fn clk_prepare(clk: *mut clk) -> c_int;
}
//
// clk_unprepare - undo preparation of a clock source
// @clk: clock source
//
// This undoes a previously prepared clock.  The caller must balance
// the number of prepare and unprepare calls.
//
// Must not be called from within atomic context.
//
extern "C" {
    pub fn clk_unprepare(clk: *mut clk);
}
extern "C" {
    pub fn clk_bulk_unprepare(num_clks: c_int, clks: *const clk_bulk_data);
}
//
// clk_is_enabled_when_prepared - indicate if preparing a clock also enables it.
// @clk: clock source
//
// Returns true if clk_prepare() implicitly enables the clock, effectively
// making clk_enable()/clk_disable() no-ops, false otherwise.
//
// This is of interest mainly to the power management code where actually
// disabling the clock also requires unpreparing it to have any material
// effect.
//
// Regardless of the value returned here, the caller must always invoke
// clk_enable() or clk_prepare_enable()  and counterparts for usage counts
// to be right.
//
extern "C" {
    pub fn clk_is_enabled_when_prepared(clk: *mut clk) -> bool;
}

//
// clk_get - lookup and obtain a reference to a clock producer.
// @dev: device for clock "consumer"
// @id: clock consumer ID
//
// Returns a struct clk corresponding to the clock producer, or
// valid IS_ERR() condition containing errno.  The implementation
// uses @dev and @id to determine the clock consumer, and thereby
// the clock producer.  (IOW, @id may be identical strings, but
// clk_get may return different clock producers depending on @dev.)
//
// Drivers must assume that the clock source is not enabled.
//
// clk_get should not be called from within interrupt context.
//
// clk_bulk_get - lookup and obtain a number of references to clock producer.
// @dev: device for clock "consumer"
// @num_clks: the number of clk_bulk_data
// @clks: the clk_bulk_data table of consumer
//
// This helper function allows drivers to get several clk consumers in one
// operation. If any of the clk cannot be acquired then any clks
// that were obtained will be freed before returning to the caller.
//
// Returns 0 if all clocks specified in clk_bulk_data table are obtained
// successfully, or valid IS_ERR() condition containing errno.
// The implementation uses @dev and @clk_bulk_data.id to determine the
// clock consumer, and thereby the clock producer.
// The clock returned is stored in each @clk_bulk_data.clk field.
//
// Drivers must assume that the clock source is not enabled.
//
// clk_bulk_get should not be called from within interrupt context.
//
// clk_bulk_get_all - lookup and obtain all available references to clock
// producer.
// @dev: device for clock "consumer"
// @clks: pointer to the clk_bulk_data table of consumer
//
// This helper function allows drivers to get all clk consumers in one
// operation. If any of the clk cannot be acquired then any clks
// that were obtained will be freed before returning to the caller.
//
// Returns a positive value for the number of clocks obtained while the
// clock references are stored in the clk_bulk_data table in @clks field.
// Returns 0 if there're none and a negative value if something failed.
//
// Drivers must assume that the clock source is not enabled.
//
// clk_bulk_get should not be called from within interrupt context.
//
// clk_bulk_get_optional - lookup and obtain a number of references to clock producer
// @dev: device for clock "consumer"
// @num_clks: the number of clk_bulk_data
// @clks: the clk_bulk_data table of consumer
//
// Behaves the same as clk_bulk_get() except where there is no clock producer.
// In this case, instead of returning -ENOENT, the function returns 0 and
// NULL for a clk for which a clock producer could not be determined.
//
// devm_clk_bulk_get - managed get multiple clk consumers
// @dev: device for clock "consumer"
// @num_clks: the number of clk_bulk_data
// @clks: the clk_bulk_data table of consumer
//
// Return 0 on success, an errno on failure.
//
// This helper function allows drivers to get several clk
// consumers in one operation with management, the clks will
// automatically be freed when the device is unbound.
//
// devm_clk_bulk_get_optional - managed get multiple optional consumer clocks
// @dev: device for clock "consumer"
// @num_clks: the number of clk_bulk_data
// @clks: pointer to the clk_bulk_data table of consumer
//
// Behaves the same as devm_clk_bulk_get() except where there is no clock
// producer.  In this case, instead of returning -ENOENT, the function returns
// NULL for given clk. It is assumed all clocks in clk_bulk_data are optional.
//
// Returns 0 if all clocks specified in clk_bulk_data table are obtained
// successfully or for any clk there was no clk provider available, otherwise
// returns valid IS_ERR() condition containing errno.
// The implementation uses @dev and @clk_bulk_data.id to determine the
// clock consumer, and thereby the clock producer.
// The clock returned is stored in each @clk_bulk_data.clk field.
//
// Drivers must assume that the clock source is not enabled.
//
// clk_bulk_get should not be called from within interrupt context.
//
// devm_clk_bulk_get_enable - Get and enable bulk clocks (managed)
// @dev: device for clock "consumer"
// @num_clks: the number of clk_bulk_data
// @clks: pointer to the clk_bulk_data table of consumer
//
// Behaves the same as devm_clk_bulk_get() but also prepares and enables the
// clocks in one operation with management. The clks will automatically be
// disabled, unprepared and freed when the device is unbound.
//
// Return: 0 if all clocks specified in clk_bulk_data table are obtained and
// enabled successfully. Otherwise returns valid IS_ERR() condition containing
// errno.
//
// devm_clk_bulk_get_optional_enable - Get and enable optional bulk clocks (managed)
// @dev: device for clock "consumer"
// @num_clks: the number of clk_bulk_data
// @clks: pointer to the clk_bulk_data table of consumer
//
// Behaves the same as devm_clk_bulk_get_optional() but also prepares and enables
// the clocks in one operation with management. The clks will automatically be
// disabled, unprepared and freed when the device is unbound.
//
// Return: 0 if all clocks specified in clk_bulk_data table are obtained
// and enabled successfully, or for any clk there was no clk provider available.
// Otherwise returns valid IS_ERR() condition containing errno.
//
// devm_clk_bulk_get_all - managed get multiple clk consumers
// @dev: device for clock "consumer"
// @clks: pointer to the clk_bulk_data table of consumer
//
// Returns a positive value for the number of clocks obtained while the
// clock references are stored in the clk_bulk_data table in @clks field.
// Returns 0 if there're none and a negative value if something failed.
//
// This helper function allows drivers to get several clk
// consumers in one operation with management, the clks will
// automatically be freed when the device is unbound.
//
// devm_clk_bulk_get_all_enabled - Get and enable all clocks of the consumer (managed)
// @dev: device for clock "consumer"
// @clks: pointer to the clk_bulk_data table of consumer
//
// Returns a positive value for the number of clocks obtained while the
// clock references are stored in the clk_bulk_data table in @clks field.
// Returns 0 if there're none and a negative value if something failed.
//
// This helper function allows drivers to get all clocks of the
// consumer and enables them in one operation with management.
// The clks will automatically be disabled and freed when the device
// is unbound.
//
// devm_clk_get - lookup and obtain a managed reference to a clock producer.
// @dev: device for clock "consumer"
// @id: clock consumer ID
//
// Context: May sleep.
//
// Return: a struct clk corresponding to the clock producer, or
// valid IS_ERR() condition containing errno.  The implementation
// uses @dev and @id to determine the clock consumer, and thereby
// the clock producer.  (IOW, @id may be identical strings, but
// clk_get may return different clock producers depending on @dev.)
//
// Drivers must assume that the clock source is neither prepared nor
// enabled.
//
// The clock will automatically be freed when the device is unbound
// from the bus.
//
// devm_clk_get_prepared - devm_clk_get() + clk_prepare()
// @dev: device for clock "consumer"
// @id: clock consumer ID
//
// Context: May sleep.
//
// Return: a struct clk corresponding to the clock producer, or
// valid IS_ERR() condition containing errno.  The implementation
// uses @dev and @id to determine the clock consumer, and thereby
// the clock producer.  (IOW, @id may be identical strings, but
// clk_get may return different clock producers depending on @dev.)
//
// The returned clk (if valid) is prepared. Drivers must however assume
// that the clock is not enabled.
//
// The clock will automatically be unprepared and freed when the device
// is unbound from the bus.
//
// devm_clk_get_enabled - devm_clk_get() + clk_prepare_enable()
// @dev: device for clock "consumer"
// @id: clock consumer ID
//
// Context: May sleep.
//
// Return: a struct clk corresponding to the clock producer, or
// valid IS_ERR() condition containing errno.  The implementation
// uses @dev and @id to determine the clock consumer, and thereby
// the clock producer.  (IOW, @id may be identical strings, but
// clk_get may return different clock producers depending on @dev.)
//
// The returned clk (if valid) is prepared and enabled.
//
// The clock will automatically be disabled, unprepared and freed
// when the device is unbound from the bus.
//
// devm_clk_get_optional - lookup and obtain a managed reference to an optional
// clock producer.
// @dev: device for clock "consumer"
// @id: clock consumer ID
//
// Context: May sleep.
//
// Return: a struct clk corresponding to the clock producer, or
// valid IS_ERR() condition containing errno.  The implementation
// uses @dev and @id to determine the clock consumer, and thereby
// the clock producer.  If no such clk is found, it returns NULL
// which serves as a dummy clk.  That's the only difference compared
// to devm_clk_get().
//
// Drivers must assume that the clock source is neither prepared nor
// enabled.
//
// The clock will automatically be freed when the device is unbound
// from the bus.
//
// devm_clk_get_optional_prepared - devm_clk_get_optional() + clk_prepare()
// @dev: device for clock "consumer"
// @id: clock consumer ID
//
// Context: May sleep.
//
// Return: a struct clk corresponding to the clock producer, or
// valid IS_ERR() condition containing errno.  The implementation
// uses @dev and @id to determine the clock consumer, and thereby
// the clock producer.  If no such clk is found, it returns NULL
// which serves as a dummy clk.  That's the only difference compared
// to devm_clk_get_prepared().
//
// The returned clk (if valid) is prepared. Drivers must however
// assume that the clock is not enabled.
//
// The clock will automatically be unprepared and freed when the
// device is unbound from the bus.
//
// devm_clk_get_optional_enabled - devm_clk_get_optional() +
// clk_prepare_enable()
// @dev: device for clock "consumer"
// @id: clock consumer ID
//
// Context: May sleep.
//
// Return: a struct clk corresponding to the clock producer, or
// valid IS_ERR() condition containing errno.  The implementation
// uses @dev and @id to determine the clock consumer, and thereby
// the clock producer.  If no such clk is found, it returns NULL
// which serves as a dummy clk.  That's the only difference compared
// to devm_clk_get_enabled().
//
// The returned clk (if valid) is prepared and enabled.
//
// The clock will automatically be disabled, unprepared and freed
// when the device is unbound from the bus.
//
// devm_clk_get_optional_enabled_with_rate - devm_clk_get_optional() +
// clk_set_rate() +
// clk_prepare_enable()
// @dev: device for clock "consumer"
// @id: clock consumer ID
// @rate: new clock rate
//
// Context: May sleep.
//
// Return: a struct clk corresponding to the clock producer, or
// valid IS_ERR() condition containing errno.  The implementation
// uses @dev and @id to determine the clock consumer, and thereby
// the clock producer.  If no such clk is found, it returns NULL
// which serves as a dummy clk.  That's the only difference compared
// to devm_clk_get_enabled().
//
// The returned clk (if valid) is prepared and enabled and rate was set.
//
// The clock will automatically be disabled, unprepared and freed
// when the device is unbound from the bus.
//
// devm_get_clk_from_child - lookup and obtain a managed reference to a
// clock producer from child node.
// @dev: device for clock "consumer"
// @np: pointer to clock consumer node
// @con_id: clock consumer ID
//
// This function parses the clocks, and uses them to look up the
// struct clk from the registered list of clock providers by using
// @np and @con_id
//
// The clock will automatically be freed when the device is unbound
// from the bus.
//
// clk_enable - inform the system when the clock source should be running.
// @clk: clock source
//
// If the clock can not be enabled/disabled, this should return success.
//
// May be called from atomic contexts.
//
// Returns success (0) or negative errno.
//
extern "C" {
    pub fn clk_enable(clk: *mut clk) -> c_int;
}
//
// clk_bulk_enable - inform the system when the set of clks should be running.
// @num_clks: the number of clk_bulk_data
// @clks: the clk_bulk_data table of consumer
//
// May be called from atomic contexts.
//
// Returns success (0) or negative errno.
//
// clk_disable - inform the system when the clock source is no longer required.
// @clk: clock source
//
// Inform the system that a clock source is no longer required by
// a driver and may be shut down.
//
// May be called from atomic contexts.
//
// Implementation detail: if the clock source is shared between
// multiple drivers, clk_enable() calls must be balanced by the
// same number of clk_disable() calls for the clock source to be
// disabled.
//
extern "C" {
    pub fn clk_disable(clk: *mut clk);
}
//
// clk_bulk_disable - inform the system when the set of clks is no
// longer required.
// @num_clks: the number of clk_bulk_data
// @clks: the clk_bulk_data table of consumer
//
// Inform the system that a set of clks is no longer required by
// a driver and may be shut down.
//
// May be called from atomic contexts.
//
// Implementation detail: if the set of clks is shared between
// multiple drivers, clk_bulk_enable() calls must be balanced by the
// same number of clk_bulk_disable() calls for the clock source to be
// disabled.
//
extern "C" {
    pub fn clk_bulk_disable(num_clks: c_int, clks: *const clk_bulk_data);
}
//
// clk_get_rate - obtain the current clock rate (in Hz) for a clock source.
// This is only valid once the clock source has been enabled.
// @clk: clock source
//
extern "C" {
    pub fn clk_get_rate(clk: *mut clk) -> c_ulong;
}
//
// clk_put	- "free" the clock source
// @clk: clock source
//
// Note: drivers must ensure that all clk_enable calls made on this
// clock source are balanced by clk_disable calls prior to calling
// this function.
//
// clk_put should not be called from within interrupt context.
//
extern "C" {
    pub fn clk_put(clk: *mut clk);
}
//
// clk_bulk_put	- "free" the clock source
// @num_clks: the number of clk_bulk_data
// @clks: the clk_bulk_data table of consumer
//
// Note: drivers must ensure that all clk_bulk_enable calls made on this
// clock source are balanced by clk_bulk_disable calls prior to calling
// this function.
//
// clk_bulk_put should not be called from within interrupt context.
//
extern "C" {
    pub fn clk_bulk_put(num_clks: c_int, clks: *mut clk_bulk_data);
}
//
// clk_bulk_put_all - "free" all the clock source
// @num_clks: the number of clk_bulk_data
// @clks: the clk_bulk_data table of consumer
//
// Note: drivers must ensure that all clk_bulk_enable calls made on this
// clock source are balanced by clk_bulk_disable calls prior to calling
// this function.
//
// clk_bulk_put_all should not be called from within interrupt context.
//
extern "C" {
    pub fn clk_bulk_put_all(num_clks: c_int, clks: *mut clk_bulk_data);
}
//
// devm_clk_put	- "free" a managed clock source
// @dev: device used to acquire the clock
// @clk: clock source acquired with devm_clk_get()
//
// Note: drivers must ensure that all clk_enable calls made on this
// clock source are balanced by clk_disable calls prior to calling
// this function.
//
// clk_put should not be called from within interrupt context.
//
extern "C" {
    pub fn devm_clk_put(dev: *mut device, clk: *mut clk);
}
//
// The remaining APIs are optional for machine class support.
//
// clk_round_rate - adjust a rate to the exact rate a clock can provide
// @clk: clock source
// @rate: desired clock rate in Hz
//
// This answers the question "if I were to pass @rate to clk_set_rate(),
// what clock rate would I end up with?" without changing the hardware
// in any way.  In other words:
//
// rate = clk_round_rate(clk, r);
//
// and:
//
// clk_set_rate(clk, r);
// rate = clk_get_rate(clk);
//
// are equivalent except the former does not modify the clock hardware
// in any way.
//
// Returns rounded clock rate in Hz, or negative errno.
//
extern "C" {
    pub fn clk_round_rate(clk: *mut clk, rate: c_ulong) -> c_long;
}
//
// clk_set_rate - set the clock rate for a clock source
// @clk: clock source
// @rate: desired clock rate in Hz
//
// Updating the rate starts at the top-most affected clock and then
// walks the tree down to the bottom-most clock that needs updating.
//
// Returns success (0) or negative errno.
//
extern "C" {
    pub fn clk_set_rate(clk: *mut clk, rate: c_ulong) -> c_int;
}
//
// clk_set_rate_exclusive- set the clock rate and claim exclusivity over
// clock source
// @clk: clock source
// @rate: desired clock rate in Hz
//
// This helper function allows drivers to atomically set the rate of a producer
// and claim exclusivity over the rate control of the producer.
//
// It is essentially a combination of clk_set_rate() and
// clk_rate_exclusite_get(). Caller must balance this call with a call to
// clk_rate_exclusive_put()
//
// Returns success (0) or negative errno.
//
extern "C" {
    pub fn clk_set_rate_exclusive(clk: *mut clk, rate: c_ulong) -> c_int;
}
//
// clk_has_parent - check if a clock is a possible parent for another
// @clk: clock source
// @parent: parent clock source
//
// This function can be used in drivers that need to check that a clock can be
// the parent of another without actually changing the parent.
//
// Returns true if @parent is a possible parent for @clk, false otherwise.
//
extern "C" {
    pub fn clk_has_parent(clk: *const clk, parent: *const clk) -> bool;
}
//
// clk_set_rate_range - set a rate range for a clock source
// @clk: clock source
// @min: desired minimum clock rate in Hz, inclusive
// @max: desired maximum clock rate in Hz, inclusive
//
// Returns success (0) or negative errno.
//
extern "C" {
    pub fn clk_set_rate_range(clk: *mut clk, min: c_ulong, max: c_ulong) -> c_int;
}
//
// clk_set_min_rate - set a minimum clock rate for a clock source
// @clk: clock source
// @rate: desired minimum clock rate in Hz, inclusive
//
// Returns success (0) or negative errno.
//
extern "C" {
    pub fn clk_set_min_rate(clk: *mut clk, rate: c_ulong) -> c_int;
}
//
// clk_set_max_rate - set a maximum clock rate for a clock source
// @clk: clock source
// @rate: desired maximum clock rate in Hz, inclusive
//
// Returns success (0) or negative errno.
//
extern "C" {
    pub fn clk_set_max_rate(clk: *mut clk, rate: c_ulong) -> c_int;
}
//
// clk_set_parent - set the parent clock source for this clock
// @clk: clock source
// @parent: parent clock source
//
// Returns success (0) or negative errno.
//
extern "C" {
    pub fn clk_set_parent(clk: *mut clk, parent: *mut clk) -> c_int;
}
//
// clk_get_parent - get the parent clock source for this clock
// @clk: clock source
//
// Returns struct clk corresponding to parent clock source, or NULL
// if clk is NULL.
//
// clk_get_sys - get a clock based upon the device name
// @dev_id: device name
// @con_id: connection ID
//
// Returns a struct clk corresponding to the clock producer, or
// valid IS_ERR() condition containing errno.  The implementation
// uses @dev_id and @con_id to determine the clock consumer, and
// thereby the clock producer. In contrast to clk_get() this function
// takes the device name instead of the device itself for identification.
//
// Drivers must assume that the clock source is not enabled.
//
// clk_get_sys should not be called from within interrupt context.
//

// clk_prepare_enable helps cases using clk_enable in non-atomic context.
// clk_disable_unprepare helps cases using clk_disable in non-atomic context.
//
// clk_drop_range - Reset any range set on that clock
// @clk: clock source
//
// Returns success (0) or negative errno.
//
extern "C" {
    pub fn clk_set_rate_range(_arg: clk, _arg: 0, _arg: ULONG_MAX) -> return;
}
//
// clk_get_optional - lookup and obtain a reference to an optional clock
// producer.
// @dev: device for clock "consumer"
// @id: clock consumer ID
//
// Behaves the same as clk_get() except where there is no clock producer. In
// this case, instead of returning -ENOENT, the function returns NULL.
//

extern "C" {
    pub fn ERR_PTR(_arg: -ENOENT) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENOENT) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENOENT) -> return;
}

