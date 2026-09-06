//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/clk-provider.h
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
// Copyright (c) 2010-2011 Jeremy Kerr <jeremy.kerr@canonical.com>
// Copyright (C) 2011-2012 Linaro Ltd <mturquette@linaro.org>
//

//
// DOC: clk framework flags
//
// Flags used across common struct clk. These flags should only affect the
// top-level framework. Custom flags for dealing with hardware specifics
// belong in struct clk_foo.
//
// * CLK_SET_RATE_GATE - must be gated across rate change
// * CLK_SET_PARENT_GATE - must be gated across re-parent
// * CLK_SET_RATE_PARENT - propagate rate change up one level
// * CLK_IGNORE_UNUSED - do not gate even if unused
// * CLK_GET_RATE_NOCACHE - do not use the cached clk rate
// * CLK_SET_RATE_NO_REPARENT - don't re-parent on rate change
// * CLK_GET_ACCURACY_NOCACHE - do not use the cached clk accuracy
// * CLK_RECALC_NEW_RATES - recalc rates after notifications
// * CLK_SET_RATE_UNGATE - clock needs to run to set rate
// * CLK_IS_CRITICAL - do not gate, ever
// * CLK_OPS_PARENT_ENABLE - parents need enable during gate/ungate, set rate and re-parent
// * CLK_DUTY_CYCLE_PARENT - duty cycle call may be forwarded to the parent clock
//
// Please update clk_flags[] in drivers/clk/clk.c when making changes here!

// unused

//
// struct clk_rate_request - Structure encoding the clk constraints that
// a clock user might require.
//
// Should be initialized by calling clk_hw_init_rate_request().
//
// @core: 		Pointer to the struct clk_core affected by this request
// @rate:		Requested clock rate. This field will be adjusted by
// clock drivers according to hardware capabilities.
// @min_rate:		Minimum rate imposed by clk users.
// @max_rate:		Maximum rate imposed by clk users.
// @best_parent_rate:	The best parent rate a parent can provide to fulfill the
// requested constraints.
// @best_parent_hw:	The most appropriate parent clock that fulfills the
// requested constraints.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_rate_request {
    pub core: *mut clk_core,
    pub rate: c_ulong,
    pub min_rate: c_ulong,
    pub max_rate: c_ulong,
    pub best_parent_rate: c_ulong,
    pub best_parent_hw: *mut clk_hw,
}

//
// struct clk_duty - Structure encoding the duty cycle ratio of a clock
//
// @num:	Numerator of the duty cycle ratio
// @den:	Denominator of the duty cycle ratio
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_duty {
    pub num: c_uint,
    pub den: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum clk_ssc_method {
    CLK_SPREAD_NO		= CLK_SSC_NO_SPREAD,
    CLK_SPREAD_CENTER	= CLK_SSC_CENTER_SPREAD,
    CLK_SPREAD_UP		= CLK_SSC_UP_SPREAD,
    CLK_SPREAD_DOWN		= CLK_SSC_DOWN_SPREAD,
}

//
// struct clk_spread_spectrum - Structure encoding spread spectrum of a clock
//
// @modfreq_hz:		Modulation frequency
// @spread_bp:		Modulation percent in permyriad
// @method:		Modulation method
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_spread_spectrum {
    pub modfreq_hz: u32,
    pub spread_bp: u32,
    pub method: clk_ssc_method,
}

//
// struct clk_ops -  Callback operations for hardware clocks; these are to
// be provided by the clock implementation, and will be called by drivers
// through the clk_* api.
//
// @prepare:	Prepare the clock for enabling. This must not return until
// the clock is fully prepared, and it's safe to call clk_enable.
// This callback is intended to allow clock implementations to
// do any initialisation that may sleep. Called with
// prepare_lock held.
//
// @unprepare:	Release the clock from its prepared state. This will typically
// undo any work done in the @prepare callback. Called with
// prepare_lock held.
//
// @is_prepared: Queries the hardware to determine if the clock is prepared.
// This function is allowed to sleep. Optional, if this op is not
// set then the prepare count will be used.
//
// @unprepare_unused: Unprepare the clock atomically.  Only called from
// clk_disable_unused for prepare clocks with special needs.
// Called with prepare mutex held. This function may sleep.
//
// @enable:	Enable the clock atomically. This must not return until the
// clock is generating a valid clock signal, usable by consumer
// devices. Called with enable_lock held. This function must not
// sleep.
//
// @disable:	Disable the clock atomically. Called with enable_lock held.
// This function must not sleep.
//
// @is_enabled:	Queries the hardware to determine if the clock is enabled.
// This function must not sleep. Optional, if this op is not
// set then the enable count will be used.
//
// @disable_unused: Disable the clock atomically.  Only called from
// clk_disable_unused for gate clocks with special needs.
// Called with enable_lock held.  This function must not
// sleep.
//
// @save_context: Save the context of the clock in prepration for poweroff.
//
// @restore_context: Restore the context of the clock after a restoration
// of power.
//
// @recalc_rate: Recalculate the rate of this clock, by querying hardware. The
// parent rate is an input parameter.  It is up to the caller to
// ensure that the prepare_mutex is held across this call. If the
// driver cannot figure out a rate for this clock, it must return
// 0. Returns the calculated rate. Optional, but recommended - if
// this op is not set then clock rate will be initialized to 0.
//
// @determine_rate: Given a target rate as input, returns the closest rate
// actually supported by the clock, and optionally the parent clock
// that should be used to provide the clock rate.
//
// @set_parent:	Change the input source of this clock; for clocks with multiple
// possible parents specify a new parent by passing in the index
// as a u8 corresponding to the parent in either the .parent_names
// or .parents arrays.  This function in affect translates an
// array index into the value programmed into the hardware.
// Returns 0 on success, -EERROR otherwise.
//
// @get_parent:	Queries the hardware to determine the parent of a clock.  The
// return value is a u8 which specifies the index corresponding to
// the parent clock.  This index can be applied to either the
// .parent_names or .parents arrays.  In short, this function
// translates the parent value read from hardware into an array
// index.  Currently only called when the clock is initialized by
// __clk_init.  This callback is mandatory for clocks with
// multiple parents.  It is optional (and unnecessary) for clocks
// with 0 or 1 parents.
//
// @set_rate:	Change the rate of this clock. The requested rate is specified
// by the second argument, which should typically be the return
// of .determine_rate call.  The third argument gives the parent
// rate which is likely helpful for most .set_rate implementation.
// Returns 0 on success, -EERROR otherwise.
//
// @set_rate_and_parent: Change the rate and the parent of this clock. The
// requested rate is specified by the second argument, which
// should typically be the return of clk_round_rate() call.  The
// third argument gives the parent rate which is likely helpful
// for most .set_rate_and_parent implementation. The fourth
// argument gives the parent index. This callback is optional (and
// unnecessary) for clocks with 0 or 1 parents as well as
// for clocks that can tolerate switching the rate and the parent
// separately via calls to .set_parent and .set_rate.
// Returns 0 on success, -EERROR otherwise.
//
// @set_spread_spectrum: Optional callback used to configure the spread
// spectrum modulation frequency, percentage, and method
// to reduce EMI by spreading the clock frequency over a
// wider range.
// Returns 0 on success, -EERROR otherwise.
//
// @recalc_accuracy: Recalculate the accuracy of this clock. The clock accuracy
// is expressed in ppb (parts per billion). The parent accuracy is
// an input parameter.
// Returns the calculated accuracy.  Optional - if	this op is not
// set then clock accuracy will be initialized to parent accuracy
// or 0 (perfect clock) if clock has no parent.
//
// @get_phase:	Queries the hardware to get the current phase of a clock.
// Returned values are 0-359 degrees on success, negative
// error codes on failure.
//
// @set_phase:	Shift the phase this clock signal in degrees specified
// by the second argument. Valid values for degrees are
// 0-359. Return 0 on success, otherwise -EERROR.
//
// @get_duty_cycle: Queries the hardware to get the current duty cycle ratio
// of a clock. Returned values denominator cannot be 0 and must be
// superior or equal to the numerator.
//
// @set_duty_cycle: Apply the duty cycle ratio to this clock signal specified by
// the numerator (2nd argurment) and denominator (3rd  argument).
// Argument must be a valid ratio (denominator > 0
// and >= numerator) Return 0 on success, otherwise -EERROR.
//
// @init:	Perform platform-specific initialization magic.
// This is not used by any of the basic clock types.
// This callback exist for HW which needs to perform some
// initialisation magic for CCF to get an accurate view of the
// clock. It may also be used dynamic resource allocation is
// required. It shall not used to deal with clock parameters,
// such as rate or parents.
// Returns 0 on success, -EERROR otherwise.
//
// @terminate:  Free any resource allocated by init.
//
// @debug_init:	Set up type-specific debugfs entries for this clock.  This
// is called once, after the debugfs directory entry for this
// clock has been created.  The dentry pointer representing that
// directory is provided as an argument.  Called with
// prepare_lock held.  Returns 0 on success, -EERROR otherwise.
//
// The clk_enable/clk_disable and clk_prepare/clk_unprepare pairs allow
// implementations to split any work between atomic (enable) and sleepable
// (prepare) contexts.  If enabling a clock requires code that might sleep,
// this must be done in clk_prepare.  Clock enable code that will never be
// called in a sleepable context may be implemented in clk_enable.
//
// Typically, drivers will call clk_prepare when a clock may be needed later
// (eg. when a device is opened), and clk_enable when the clock is actually
// required (eg. from an interrupt). Note that clk_prepare MUST have been
// called before clk_enable.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_ops {
    pub hw): *mut *mut int (prepare)(struct clk_hw,
    pub hw): *mut *mut void (unprepare)(struct clk_hw,
    pub hw): *mut *mut int (is_prepared)(struct clk_hw,
    pub hw): *mut *mut void (unprepare_unused)(struct clk_hw,
    pub hw): *mut *mut int (enable)(struct clk_hw,
    pub hw): *mut *mut void (disable)(struct clk_hw,
    pub hw): *mut *mut int (is_enabled)(struct clk_hw,
    pub hw): *mut *mut void (disable_unused)(struct clk_hw,
    pub hw): *mut *mut int (save_context)(struct clk_hw,
    pub hw): *mut *mut void (restore_context)(struct clk_hw,
    pub parent_rate): c_ulong,
    pub req): *mut clk_rate_request,
    pub index): *mut *mut *mut int (set_parent)(struct clk_hw hw, u8,
    pub hw): *mut *mut u8 (get_parent)(struct clk_hw,
    pub parent_rate): c_ulong,
    pub index): unsigned long parent_rate, u8,
    pub ss_conf): *const clk_spread_spectrum,
    pub parent_accuracy): c_ulong,
    pub hw): *mut *mut int (get_phase)(struct clk_hw,
    pub degrees): *mut *mut *mut int (set_phase)(struct clk_hw hw, int,
    pub duty): *mut clk_duty,
    pub duty): *mut clk_duty,
    pub hw): *mut *mut int (init)(struct clk_hw,
    pub hw): *mut *mut void (terminate)(struct clk_hw,
    pub dentry): *mut *mut *mut void (debug_init)(struct clk_hw hw, struct dentry,
}

//
// struct clk_parent_data - clk parent information
// @hw: parent clk_hw pointer (used for clk providers with internal clks)
// @fw_name: parent name local to provider registering clk
// @name: globally unique parent name (used as a fallback)
// @index: parent index local to provider registering clk (if @fw_name absent)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_parent_data {
    pub hw: *const clk_hw,
    pub fw_name: *const c_char,
    pub name: *const c_char,
    pub index: c_int,
}

//
// struct clk_init_data - holds init data that's common to all clocks and is
// shared between the clock provider and the common clock framework.
//
// @name: clock name
// @ops: operations this clock supports
// @parent_names: array of string names for all possible parents
// @parent_data: array of parent data for all possible parents (when some
// parents are external to the clk controller)
// @parent_hws: array of pointers to all possible parents (when all parents
// are internal to the clk controller)
// @num_parents: number of possible parents
// @flags: framework-level hints and quirks
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_init_data {
    pub name: *const c_char,
    pub ops: *const clk_ops,
// Only one of the following three should be assigned
    pub parent_names: *const *const c_char,
    pub parent_data: *const clk_parent_data,
    pub parent_hws: *const clk_hw,
    pub num_parents: u8,
    pub flags: c_ulong,
}

//
// struct clk_hw - handle for traversing from a struct clk to its corresponding
// hardware-specific structure.  struct clk_hw should be declared within struct
// clk_foo and then referenced by the struct clk instance that uses struct
// clk_foo's clk_ops
//
// @core: pointer to the struct clk_core instance that points back to this
// struct clk_hw instance
//
// @clk: pointer to the per-user struct clk instance that can be used to call
// into the clk API
//
// @init: pointer to struct clk_init_data that contains the init data shared
// with the common clock framework. This pointer will be set to NULL once
// a clk_register() variant is called on this clk_hw pointer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_hw {
    pub core: *mut clk_core,
    pub clk: *mut clk,
    pub init: *const clk_init_data,
}

//
// DOC: Basic clock implementations common to many platforms
//
// Each basic clock hardware type is comprised of a structure describing the
// clock hardware, implementations of the relevant callbacks in struct clk_ops,
// unique flags for that hardware type, a registration function and an
// alternative macro for static initialization
//
// struct clk_fixed_rate - fixed-rate clock
// @hw:		handle between common and hardware-specific interfaces
// @fixed_rate:	constant frequency of clock
// @fixed_accuracy: constant accuracy of clock in ppb (parts per billion)
// @flags:	hardware specific flags
//
// Flags:
// * CLK_FIXED_RATE_PARENT_ACCURACY - Use the accuracy of the parent clk
// instead of what's set in @fixed_accuracy.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_fixed_rate {
    pub hw: clk_hw,
    pub fixed_rate: c_ulong,
    pub fixed_accuracy: c_ulong,
    pub flags: c_ulong,
}

//
// clk_hw_register_fixed_rate - register fixed-rate clock with the clock
// framework
// @dev: device that is registering this clock
// @name: name of this clock
// @parent_name: name of clock's parent
// @flags: framework-specific flags
// @fixed_rate: non-adjustable clock rate
//

//
// devm_clk_hw_register_fixed_rate - register fixed-rate clock with the clock
// framework
// @dev: device that is registering this clock
// @name: name of this clock
// @parent_name: name of clock's parent
// @flags: framework-specific flags
// @fixed_rate: non-adjustable clock rate
//

//
// devm_clk_hw_register_fixed_rate_parent_data - register fixed-rate clock with
// the clock framework
// @dev: device that is registering this clock
// @name: name of this clock
// @parent_data: parent clk data
// @flags: framework-specific flags
// @fixed_rate: non-adjustable clock rate
//

//
// clk_hw_register_fixed_rate_parent_hw - register fixed-rate clock with
// the clock framework
// @dev: device that is registering this clock
// @name: name of this clock
// @parent_hw: pointer to parent clk
// @flags: framework-specific flags
// @fixed_rate: non-adjustable clock rate
//

//
// clk_hw_register_fixed_rate_parent_data - register fixed-rate clock with
// the clock framework
// @dev: device that is registering this clock
// @name: name of this clock
// @parent_data: parent clk data
// @flags: framework-specific flags
// @fixed_rate: non-adjustable clock rate
//

//
// clk_hw_register_fixed_rate_with_accuracy - register fixed-rate clock with
// the clock framework
// @dev: device that is registering this clock
// @name: name of this clock
// @parent_name: name of clock's parent
// @flags: framework-specific flags
// @fixed_rate: non-adjustable clock rate
// @fixed_accuracy: non-adjustable clock accuracy
//

//
// clk_hw_register_fixed_rate_with_accuracy_parent_hw - register fixed-rate
// clock with the clock framework
// @dev: device that is registering this clock
// @name: name of this clock
// @parent_hw: pointer to parent clk
// @flags: framework-specific flags
// @fixed_rate: non-adjustable clock rate
// @fixed_accuracy: non-adjustable clock accuracy
//

//
// clk_hw_register_fixed_rate_with_accuracy_parent_data - register fixed-rate
// clock with the clock framework
// @dev: device that is registering this clock
// @name: name of this clock
// @parent_data: name of clock's parent
// @flags: framework-specific flags
// @fixed_rate: non-adjustable clock rate
// @fixed_accuracy: non-adjustable clock accuracy
//

//
// clk_hw_register_fixed_rate_parent_accuracy - register fixed-rate clock with
// the clock framework
// @dev: device that is registering this clock
// @name: name of this clock
// @parent_data: name of clock's parent
// @flags: framework-specific flags
// @fixed_rate: non-adjustable clock rate
//

extern "C" {
    pub fn clk_unregister_fixed_rate(clk: *mut clk);
}
extern "C" {
    pub fn clk_hw_unregister_fixed_rate(hw: *mut clk_hw);
}
extern "C" {
    pub fn of_fixed_clk_setup(np: *mut device_node);
}
//
// struct clk_gate - gating clock
//
// @hw:		handle between common and hardware-specific interfaces
// @reg:	register controlling gate
// @bit_idx:	single bit controlling gate
// @flags:	hardware-specific flags
// @lock:	register lock
//
// Clock which can gate its output.  Implements .enable & .disable
//
// Flags:
// CLK_GATE_SET_TO_DISABLE - by default this clock sets the bit at bit_idx to
// enable the clock.  Setting this flag does the opposite: setting the bit
// disable the clock and clearing it enables the clock
// CLK_GATE_HIWORD_MASK - The gate settings are only in lower 16-bit
// of this register, and mask of gate bits are in higher 16-bit of this
// register.  While setting the gate bits, higher 16-bit should also be
// updated to indicate changing gate bits.
// CLK_GATE_BIG_ENDIAN - by default little endian register accesses are used for
// the gate register.  Setting this flag makes the register accesses big
// endian.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_gate {
    pub hw: clk_hw,
    pub reg: *mut void __iomem,
    pub bit_idx: u8,
    pub flags: u8,
    pub lock: *mut spinlock_t,
}

//
// clk_hw_register_gate - register a gate clock with the clock framework
// @dev: device that is registering this clock
// @name: name of this clock
// @parent_name: name of this clock's parent
// @flags: framework-specific flags for this clock
// @reg: register address to control gating of this clock
// @bit_idx: which bit in the register controls gating of this clock
// @clk_gate_flags: gate-specific flags for this clock
// @lock: shared register lock for this clock
//

//
// clk_hw_register_gate_parent_hw - register a gate clock with the clock
// framework
// @dev: device that is registering this clock
// @name: name of this clock
// @parent_hw: pointer to parent clk
// @flags: framework-specific flags for this clock
// @reg: register address to control gating of this clock
// @bit_idx: which bit in the register controls gating of this clock
// @clk_gate_flags: gate-specific flags for this clock
// @lock: shared register lock for this clock
//

//
// clk_hw_register_gate_parent_data - register a gate clock with the clock
// framework
// @dev: device that is registering this clock
// @name: name of this clock
// @parent_data: parent clk data
// @flags: framework-specific flags for this clock
// @reg: register address to control gating of this clock
// @bit_idx: which bit in the register controls gating of this clock
// @clk_gate_flags: gate-specific flags for this clock
// @lock: shared register lock for this clock
//

//
// devm_clk_hw_register_gate - register a gate clock with the clock framework
// @dev: device that is registering this clock
// @name: name of this clock
// @parent_name: name of this clock's parent
// @flags: framework-specific flags for this clock
// @reg: register address to control gating of this clock
// @bit_idx: which bit in the register controls gating of this clock
// @clk_gate_flags: gate-specific flags for this clock
// @lock: shared register lock for this clock
//

//
// devm_clk_hw_register_gate_parent_hw - register a gate clock with the clock
// framework
// @dev: device that is registering this clock
// @name: name of this clock
// @parent_hw: pointer to parent clk
// @flags: framework-specific flags for this clock
// @reg: register address to control gating of this clock
// @bit_idx: which bit in the register controls gating of this clock
// @clk_gate_flags: gate-specific flags for this clock
// @lock: shared register lock for this clock
//

//
// devm_clk_hw_register_gate_parent_data - register a gate clock with the
// clock framework
// @dev: device that is registering this clock
// @name: name of this clock
// @parent_data: parent clk data
// @flags: framework-specific flags for this clock
// @reg: register address to control gating of this clock
// @bit_idx: which bit in the register controls gating of this clock
// @clk_gate_flags: gate-specific flags for this clock
// @lock: shared register lock for this clock
//

extern "C" {
    pub fn clk_unregister_gate(clk: *mut clk);
}
extern "C" {
    pub fn clk_hw_unregister_gate(hw: *mut clk_hw);
}
extern "C" {
    pub fn clk_gate_is_enabled(hw: *mut clk_hw) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_div_table {
    pub val: c_uint,
    pub div: c_uint,
}

//
// struct clk_divider - adjustable divider clock
//
// @hw:		handle between common and hardware-specific interfaces
// @reg:	register containing the divider
// @shift:	shift to the divider bit field
// @width:	width of the divider bit field
// @table:	array of value/divider pairs, last entry should have div = 0
// @lock:	register lock
//
// Clock with an adjustable divider affecting its output frequency.  Implements
// .recalc_rate, .set_rate and .determine_rate
//
// @flags:
// CLK_DIVIDER_ONE_BASED - by default the divisor is the value read from the
// register plus one.  If CLK_DIVIDER_ONE_BASED is set then the divider is
// the raw value read from the register, with the value of zero considered
// invalid, unless CLK_DIVIDER_ALLOW_ZERO is set.
// CLK_DIVIDER_POWER_OF_TWO - clock divisor is 2 raised to the value read from
// the hardware register
// CLK_DIVIDER_ALLOW_ZERO - Allow zero divisors.  For dividers which have
// CLK_DIVIDER_ONE_BASED set, it is possible to end up with a zero divisor.
// Some hardware implementations gracefully handle this case and allow a
// zero divisor by not modifying their input clock
// (divide by one / bypass).
// CLK_DIVIDER_HIWORD_MASK - The divider settings are only in lower 16-bit
// of this register, and mask of divider bits are in higher 16-bit of this
// register.  While setting the divider bits, higher 16-bit should also be
// updated to indicate changing divider bits.
// CLK_DIVIDER_ROUND_CLOSEST - Makes the best calculated divider to be rounded
// to the closest integer instead of the up one.
// CLK_DIVIDER_READ_ONLY - The divider settings are preconfigured and should
// not be changed by the clock framework.
// CLK_DIVIDER_MAX_AT_ZERO - For dividers which are like CLK_DIVIDER_ONE_BASED
// except when the value read from the register is zero, the divisor is
// 2^width of the field.
// CLK_DIVIDER_BIG_ENDIAN - By default little endian register accesses are used
// for the divider register.  Setting this flag makes the register accesses
// big endian.
// CLK_DIVIDER_EVEN_INTEGERS - clock divisor is 2, 4, 6, 8, 10, etc.
// Formula is 2 * (value read from hardware + 1).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_divider {
    pub hw: clk_hw,
    pub reg: *mut void __iomem,
    pub shift: u8,
    pub width: u8,
    pub flags: u16,
    pub table: *const clk_div_table,
    pub lock: *mut spinlock_t,
}

//
// clk_register_divider - register a divider clock with the clock framework
// @dev: device registering this clock
// @name: name of this clock
// @parent_name: name of clock's parent
// @flags: framework-specific flags
// @reg: register address to adjust divider
// @shift: number of bits to shift the bitfield
// @width: width of the bitfield
// @clk_divider_flags: divider-specific flags for this clock
// @lock: shared register lock for this clock
//

//
// clk_hw_register_divider - register a divider clock with the clock framework
// @dev: device registering this clock
// @name: name of this clock
// @parent_name: name of clock's parent
// @flags: framework-specific flags
// @reg: register address to adjust divider
// @shift: number of bits to shift the bitfield
// @width: width of the bitfield
// @clk_divider_flags: divider-specific flags for this clock
// @lock: shared register lock for this clock
//

//
// clk_hw_register_divider_parent_hw - register a divider clock with the clock
// framework
// @dev: device registering this clock
// @name: name of this clock
// @parent_hw: pointer to parent clk
// @flags: framework-specific flags
// @reg: register address to adjust divider
// @shift: number of bits to shift the bitfield
// @width: width of the bitfield
// @clk_divider_flags: divider-specific flags for this clock
// @lock: shared register lock for this clock
//

//
// clk_hw_register_divider_parent_data - register a divider clock with the clock
// framework
// @dev: device registering this clock
// @name: name of this clock
// @parent_data: parent clk data
// @flags: framework-specific flags
// @reg: register address to adjust divider
// @shift: number of bits to shift the bitfield
// @width: width of the bitfield
// @clk_divider_flags: divider-specific flags for this clock
// @lock: shared register lock for this clock
//

//
// clk_hw_register_divider_table - register a table based divider clock with
// the clock framework
// @dev: device registering this clock
// @name: name of this clock
// @parent_name: name of clock's parent
// @flags: framework-specific flags
// @reg: register address to adjust divider
// @shift: number of bits to shift the bitfield
// @width: width of the bitfield
// @clk_divider_flags: divider-specific flags for this clock
// @table: array of divider/value pairs ending with a div set to 0
// @lock: shared register lock for this clock
//

//
// clk_hw_register_divider_table_parent_hw - register a table based divider
// clock with the clock framework
// @dev: device registering this clock
// @name: name of this clock
// @parent_hw: pointer to parent clk
// @flags: framework-specific flags
// @reg: register address to adjust divider
// @shift: number of bits to shift the bitfield
// @width: width of the bitfield
// @clk_divider_flags: divider-specific flags for this clock
// @table: array of divider/value pairs ending with a div set to 0
// @lock: shared register lock for this clock
//

//
// clk_hw_register_divider_table_parent_data - register a table based divider
// clock with the clock framework
// @dev: device registering this clock
// @name: name of this clock
// @parent_data: parent clk data
// @flags: framework-specific flags
// @reg: register address to adjust divider
// @shift: number of bits to shift the bitfield
// @width: width of the bitfield
// @clk_divider_flags: divider-specific flags for this clock
// @table: array of divider/value pairs ending with a div set to 0
// @lock: shared register lock for this clock
//

//
// devm_clk_hw_register_divider - register a divider clock with the clock framework
// @dev: device registering this clock
// @name: name of this clock
// @parent_name: name of clock's parent
// @flags: framework-specific flags
// @reg: register address to adjust divider
// @shift: number of bits to shift the bitfield
// @width: width of the bitfield
// @clk_divider_flags: divider-specific flags for this clock
// @lock: shared register lock for this clock
//

//
// devm_clk_hw_register_divider_parent_hw - register a divider clock with the clock framework
// @dev: device registering this clock
// @name: name of this clock
// @parent_hw: pointer to parent clk
// @flags: framework-specific flags
// @reg: register address to adjust divider
// @shift: number of bits to shift the bitfield
// @width: width of the bitfield
// @clk_divider_flags: divider-specific flags for this clock
// @lock: shared register lock for this clock
//

//
// devm_clk_hw_register_divider_parent_data - register a divider clock with the
// clock framework
// @dev: device registering this clock
// @name: name of this clock
// @parent_data: parent clk data
// @flags: framework-specific flags
// @reg: register address to adjust divider
// @shift: number of bits to shift the bitfield
// @width: width of the bitfield
// @clk_divider_flags: divider-specific flags for this clock
// @lock: shared register lock for this clock
//

//
// devm_clk_hw_register_divider_table - register a table based divider clock
// with the clock framework (devres variant)
// @dev: device registering this clock
// @name: name of this clock
// @parent_name: name of clock's parent
// @flags: framework-specific flags
// @reg: register address to adjust divider
// @shift: number of bits to shift the bitfield
// @width: width of the bitfield
// @clk_divider_flags: divider-specific flags for this clock
// @table: array of divider/value pairs ending with a div set to 0
// @lock: shared register lock for this clock
//

extern "C" {
    pub fn clk_unregister_divider(clk: *mut clk);
}
extern "C" {
    pub fn clk_hw_unregister_divider(hw: *mut clk_hw);
}
//
// struct clk_mux - multiplexer clock
//
// @hw:		handle between common and hardware-specific interfaces
// @reg:	register controlling multiplexer
// @table:	array of register values corresponding to the parent index
// @shift:	shift to multiplexer bit field
// @mask:	mask of mutliplexer bit field
// @flags:	hardware-specific flags
// @lock:	register lock
//
// Clock with multiple selectable parents.  Implements .get_parent, .set_parent
// and .recalc_rate
//
// Flags:
// CLK_MUX_INDEX_ONE - register index starts at 1, not 0
// CLK_MUX_INDEX_BIT - register index is a single bit (power of two)
// CLK_MUX_HIWORD_MASK - The mux settings are only in lower 16-bit of this
// register, and mask of mux bits are in higher 16-bit of this register.
// While setting the mux bits, higher 16-bit should also be updated to
// indicate changing mux bits.
// CLK_MUX_READ_ONLY - The mux registers can't be written, only read in the
// .get_parent clk_op.
// CLK_MUX_ROUND_CLOSEST - Use the parent rate that is closest to the desired
// frequency.
// CLK_MUX_BIG_ENDIAN - By default little endian register accesses are used for
// the mux register.  Setting this flag makes the register accesses big
// endian.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_mux {
    pub hw: clk_hw,
    pub reg: *mut void __iomem,
    pub table: *const u32,
    pub mask: u32,
    pub shift: u8,
    pub flags: u8,
    pub lock: *mut spinlock_t,
}

extern "C" {
    pub fn clk_mux_index_to_val(table: *const u32, flags: c_uint, index: u8) -> c_uint;
}
extern "C" {
    pub fn clk_unregister_mux(clk: *mut clk);
}
extern "C" {
    pub fn clk_hw_unregister_mux(hw: *mut clk_hw);
}
extern "C" {
    pub fn of_fixed_factor_clk_setup(node: *mut device_node);
}
//
// struct clk_fixed_factor - fixed multiplier and divider clock
//
// @hw:		handle between common and hardware-specific interfaces
// @mult:	multiplier
// @div:	divider
// @acc:	fixed accuracy in ppb
// @flags:	behavior modifying flags
//
// Clock with a fixed multiplier and divider. The output frequency is the
// parent clock rate divided by div and multiplied by mult.
// Implements .recalc_rate, .set_rate, .determine_rate and .recalc_accuracy
//
// Flags:
// * CLK_FIXED_FACTOR_FIXED_ACCURACY - Use the value in @acc instead of the
// parent clk accuracy.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_fixed_factor {
    pub hw: clk_hw,
    pub mult: c_uint,
    pub div: c_uint,
    pub acc: c_ulong,
    pub flags: c_uint,
}

extern "C" {
    pub fn clk_unregister_fixed_factor(clk: *mut clk);
}
extern "C" {
    pub fn clk_hw_unregister_fixed_factor(hw: *mut clk_hw);
}

//
// devm_clk_hw_register_fixed_factor_parent_hw - Register a fixed factor clock with
// pointer to parent clock
// @dev: device that is registering this clock
// @name: name of this clock
// @parent_hw: pointer to parent clk
// @flags: fixed factor flags
// @mult: multiplier
// @div: divider
//
// Return: Pointer to fixed factor clk_hw structure that was registered or
// an error pointer.
//

//
// struct clk_fractional_divider - adjustable fractional divider clock
//
// @hw:		handle between common and hardware-specific interfaces
// @reg:	register containing the divider
// @mshift:	shift to the numerator bit field
// @mwidth:	width of the numerator bit field
// @nshift:	shift to the denominator bit field
// @nwidth:	width of the denominator bit field
// @approximation: clk driver's callback for calculating the divider clock
// @lock:	register lock
//
// Clock with adjustable fractional divider affecting its output frequency.
//
// @flags:
// CLK_FRAC_DIVIDER_ZERO_BASED - by default the numerator and denominator
// is the value read from the register. If CLK_FRAC_DIVIDER_ZERO_BASED
// is set then the numerator and denominator are both the value read
// plus one.
// CLK_FRAC_DIVIDER_BIG_ENDIAN - By default little endian register accesses are
// used for the divider register.  Setting this flag makes the register
// accesses big endian.
// CLK_FRAC_DIVIDER_POWER_OF_TWO_PS - By default the resulting fraction might
// be saturated and the caller will get quite far from the good enough
// approximation. Instead the caller may require, by setting this flag,
// to shift left by a few bits in case, when the asked one is quite small
// to satisfy the desired range of denominator. It assumes that on the
// caller's side the power-of-two capable prescaler exists.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_fractional_divider {
    pub hw: clk_hw,
    pub reg: *mut void __iomem,
    pub mshift: u8,
    pub mwidth: u8,
    pub nshift: u8,
    pub nwidth: u8,
    pub flags: u8,
    pub n): *mut *mut unsigned long m, unsigned long,
    pub lock: *mut spinlock_t,
}

extern "C" {
    pub fn clk_hw_unregister_fractional_divider(hw: *mut clk_hw);
}
//
// struct clk_multiplier - adjustable multiplier clock
//
// @hw:		handle between common and hardware-specific interfaces
// @reg:	register containing the multiplier
// @shift:	shift to the multiplier bit field
// @width:	width of the multiplier bit field
// @lock:	register lock
//
// Clock with an adjustable multiplier affecting its output frequency.
// Implements .recalc_rate, .set_rate and .determine_rate
//
// @flags:
// CLK_MULTIPLIER_ZERO_BYPASS - By default, the multiplier is the value read
// from the register, with 0 being a valid value effectively
// zeroing the output clock rate. If CLK_MULTIPLIER_ZERO_BYPASS is
// set, then a null multiplier will be considered as a bypass,
// leaving the parent rate unmodified.
// CLK_MULTIPLIER_ROUND_CLOSEST - Makes the best calculated divider to be
// rounded to the closest integer instead of the down one.
// CLK_MULTIPLIER_BIG_ENDIAN - By default little endian register accesses are
// used for the multiplier register.  Setting this flag makes the register
// accesses big endian.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_multiplier {
    pub hw: clk_hw,
    pub reg: *mut void __iomem,
    pub shift: u8,
    pub width: u8,
    pub flags: u8,
    pub lock: *mut spinlock_t,
}

//
// struct clk_composite - aggregate clock of mux, divider and gate clocks
//
// @hw:		handle between common and hardware-specific interfaces
// @mux_hw:	handle between composite and hardware-specific mux clock
// @rate_hw:	handle between composite and hardware-specific rate clock
// @gate_hw:	handle between composite and hardware-specific gate clock
// @mux_ops:	clock ops for mux
// @rate_ops:	clock ops for rate
// @gate_ops:	clock ops for gate
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_composite {
    pub hw: clk_hw,
    pub ops: clk_ops,
    pub mux_hw: *mut clk_hw,
    pub rate_hw: *mut clk_hw,
    pub gate_hw: *mut clk_hw,
    pub mux_ops: *const clk_ops,
    pub rate_ops: *const clk_ops,
    pub gate_ops: *const clk_ops,
}

extern "C" {
    pub fn clk_unregister_composite(clk: *mut clk);
}
extern "C" {
    pub fn clk_hw_unregister_composite(hw: *mut clk_hw);
}
extern "C" {
    pub fn clk_hw_register(dev: *mut device, hw: *mut clk_hw) -> int __must_check;
}
extern "C" {
    pub fn devm_clk_hw_register(dev: *mut device, hw: *mut clk_hw) -> int __must_check;
}
extern "C" {
    pub fn of_clk_hw_register(node: *mut device_node, hw: *mut clk_hw) -> int __must_check;
}
extern "C" {
    pub fn clk_unregister(clk: *mut clk);
}
extern "C" {
    pub fn clk_hw_unregister(hw: *mut clk_hw);
}
// helper functions
//
// clk_hw_get_dev() - get device from an hardware clock.
// @hw: the clk_hw pointer to get the struct device from
//
// This is a helper to get the struct device associated with a hardware
// clock. Some clock controllers, such as the one registered with
// CLK_OF_DECLARE(), may have not provided a device pointer while
// registering the clock.
//
// Return: the struct device associated with the clock, or NULL if there
// is none.
//
// clk_hw_get_of_node() - get device_node from a hardware clock.
// @hw: the clk_hw pointer to get the struct device_node from
//
// This is a helper to get the struct device_node associated with a
// hardware clock.
//
// Return: the struct device_node associated with the clock, or NULL
// if there is none.
//

extern "C" {
    pub fn clk_hw_get_num_parents(hw: *const clk_hw) -> c_uint;
}
extern "C" {
    pub fn clk_hw_get_parent_index(hw: *mut clk_hw) -> c_int;
}
extern "C" {
    pub fn clk_hw_set_parent(hw: *mut clk_hw, new_parent: *mut clk_hw) -> c_int;
}
extern "C" {
    pub fn __clk_get_enable_count(clk: *mut clk) -> c_uint;
}
extern "C" {
    pub fn clk_hw_get_rate(hw: *const clk_hw) -> c_ulong;
}
extern "C" {
    pub fn clk_hw_get_flags(hw: *const clk_hw) -> c_ulong;
}

extern "C" {
    pub fn clk_hw_is_prepared(hw: *const clk_hw) -> bool;
}
extern "C" {
    pub fn clk_hw_is_enabled(hw: *const clk_hw) -> bool;
}
extern "C" {
    pub fn __clk_is_enabled(clk: *mut clk) -> bool;
}
extern "C" {
    pub fn __clk_determine_rate(core: *mut clk_hw, req: *mut clk_rate_request) -> c_int;
}
extern "C" {
    pub fn clk_determine_rate_noop(hw: *mut clk_hw, req: *mut clk_rate_request) -> c_int;
}
extern "C" {
    pub fn clk_hw_reparent(hw: *mut clk_hw, new_parent: *mut clk_hw);
}
//
// FIXME clock api without lock protection
//
extern "C" {
    pub fn clk_hw_round_rate(hw: *mut clk_hw, rate: c_ulong) -> c_ulong;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_onecell_data {
    pub clks: *mut clk,
    pub clk_num: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_hw_onecell_data {
    pub num: c_uint,
    pub __counted_by(num): *mut *mut clk_hw hws[],
}

//
// Use this macro when you have a driver that requires two initialization
// routines, one at of_clk_init(), and one at platform device probe
//

//
// This macro is intended for drivers to be able to share the otherwise
// individual struct clk_hw[] compound literals created by the compiler
// when using CLK_HW_INIT_HW. It does NOT support multiple parents.
//

//
// This macro allows the driver to reuse the _parent array for multiple
// fixed factor clk declarations.
//

extern "C" {
    pub fn of_clk_del_provider(np: *mut device_node);
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENOENT) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENOENT) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENOENT) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENOENT) -> return;
}

extern "C" {
    pub fn clk_gate_restore_context(hw: *mut clk_hw);
}
