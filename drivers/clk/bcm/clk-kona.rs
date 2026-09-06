//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/bcm/clk-kona.h
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
// Copyright (C) 2013 Broadcom Corporation
// Copyright 2013 Linaro Limited
//

pub const BILLION: c_int = 1000000000;
// The common clock framework uses u8 to represent a parent index

//
// Utility macros for object flag management.  If possible, flags
// should be defined such that 0 is the desired default value.
//

// CCU field state tests

// Clock field state tests

// Clock type, used to tell common block what it's part of
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bcm_clk_type {
    bcm_clk_none,		/* undefined clock type */
    bcm_clk_bus,
    bcm_clk_core,
    bcm_clk_peri
}

//
// CCU policy control for clocks.  Clocks can be enabled or disabled
// based on the CCU policy in effect.  One bit in each policy mask
// register (one per CCU policy) represents whether the clock is
// enabled when that policy is effect or not.  The CCU policy engine
// must be stopped to update these bits, and must be restarted again
// afterward.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_clk_policy {
    pub /: *mut *mut u32 offset; / first policy mask register offset,
    pub /: *mut *mut u32 bit; / bit used in all mask registers,
}

// Policy initialization macro

//
// Gating control and status is managed by a 32-bit gate register.
//
// There are several types of gating available:
// - (no gate)
// A clock with no gate is assumed to be always enabled.
// - hardware-only gating (auto-gating)
// Enabling or disabling clocks with this type of gate is
// managed automatically by the hardware.  Such clocks can be
// considered by the software to be enabled.  The current status
// of auto-gated clocks can be read from the gate status bit.
// - software-only gating
// Auto-gating is not available for this type of clock.
// Instead, software manages whether it's enabled by setting or
// clearing the enable bit.  The current gate status of a gate
// under software control can be read from the gate status bit.
// To ensure a change to the gating status is complete, the
// status bit can be polled to verify that the gate has entered
// the desired state.
// - selectable hardware or software gating
// Gating for this type of clock can be configured to be either
// under software or hardware control.  Which type is in use is
// determined by the hw_sw_sel bit of the gate register.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_clk_gate {
    pub /: *mut *mut u32 offset; / gate register offset,
    pub /: *mut *mut u32 status_bit; / 0: gate is disabled; 0: gatge is enabled,
    pub /: *mut *mut u32 en_bit; / 0: disable; 1: enable,
    pub /: *mut *mut u32 hw_sw_sel_bit; / 0: hardware gating; 1: software gating,
    pub /: *mut *mut *mut u32 flags; / BCM_CLK_GATE_FLAGS_ below,
}

//
// Gate flags:
// HW         means this gate can be auto-gated
// SW         means the state of this gate can be software controlled
// NO_DISABLE means this gate is (only) enabled if under software control
// SW_MANAGED means the status of this gate is under software control
// ENABLED    means this software-managed gate is *supposed* to be enabled
//

//
// Gate initialization macros.
//
// Any gate initially under software control will be enabled.
//
// A hardware/software gate initially under software control

// A hardware/software gate initially under hardware control

// A hardware-or-enabled gate (enabled if not under hardware control)

// A software-only gate

// A hardware-only gate

// Gate hysteresis for clocks
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_clk_hyst {
    pub /: *mut *mut u32 offset; / hyst register offset (normally CLKGATE),
    pub /: *mut *mut u32 en_bit; / bit used to enable hysteresis,
    pub /: *mut *mut u32 val_bit; / if enabled: 0 = low delay; 1 = high delay,
}

// Hysteresis initialization macro

//
// Each clock can have zero, one, or two dividers which change the
// output rate of the clock.  Each divider can be either fixed or
// variable.  If there are two dividers, they are the "pre-divider"
// and the "regular" or "downstream" divider.  If there is only one,
// there is no pre-divider.
//
// A fixed divider is any non-zero (positive) value, and it
// indicates how the input rate is affected by the divider.
//
// The value of a variable divider is maintained in a sub-field of a
// 32-bit divider register.  The position of the field in the
// register is defined by its offset and width.  The value recorded
// in this field is always 1 less than the value it represents.
//
// In addition, a variable divider can indicate that some subset
// of its bits represent a "fractional" part of the divider.  Such
// bits comprise the low-order portion of the divider field, and can
// be viewed as representing the portion of the divider that lies to
// the right of the decimal point.  Most variable dividers have zero
// fractional bits.  Variable dividers with non-zero fraction width
// still record a value 1 less than the value they represent; the
// added 1 does *not* affect the low-order bit in this case, it
// affects the bits above the fractional part only.  (Often in this
// code a divider field value is distinguished from the value it
// represents by referring to the latter as a "divisor".)
//
// In order to avoid dealing with fractions, divider arithmetic is
// performed using "scaled" values.  A scaled value is one that's
// been left-shifted by the fractional width of a divider.  Dividing
// a scaled value by a scaled divisor produces the desired quotient
// without loss of precision and without any other special handling
// for fractions.
//
// The recorded value of a variable divider can be modified.  To
// modify either divider (or both), a clock must be enabled (i.e.,
// using its gate).  In addition, a trigger register (described
// below) must be used to commit the change, and polled to verify
// the change is complete.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_clk_div {
    pub /: *mut *mut u32 offset; / divider register offset,
    pub /: *mut *mut u32 shift; / field shift,
    pub /: *mut *mut u32 width; / field width,
    pub /: *mut *mut u32 frac_width; / field fraction width,
    pub /: *mut *mut u64 scaled_div; / scaled divider value,
    pub s: },
    pub /: *mut *mut u32 fixed; / non-zero fixed divider value,
    pub u: },
    pub /: *mut *mut *mut u32 flags; / BCM_CLK_DIV_FLAGS_ below,
}

//
// Divider flags:
// EXISTS means this divider exists
// FIXED means it is a fixed-rate divider
//

// Divider initialization macros
// A fixed (non-zero) divider

// A divider with an integral divisor

// A divider whose divisor has an integer and fractional part

//
// Clocks may have multiple "parent" clocks.  If there is more than
// one, a selector must be specified to define which of the parent
// clocks is currently in use.  The selected clock is indicated in a
// sub-field of a 32-bit selector register.  The range of
// representable selector values typically exceeds the number of
// available parent clocks.  Occasionally the reset value of a
// selector field is explicitly set to a (specific) value that does
// not correspond to a defined input clock.
//
// We register all known parent clocks with the common clock code
// using a packed array (i.e., no empty slots) of (parent) clock
// names, and refer to them later using indexes into that array.
// We maintain an array of selector values indexed by common clock
// index values in order to map between these common clock indexes
// and the selector values used by the hardware.
//
// Like dividers, a selector can be modified, but to do so a clock
// must be enabled, and a trigger must be used to commit the change.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_clk_sel {
    pub /: *mut *mut u32 offset; / selector register offset,
    pub /: *mut *mut u32 shift; / field shift,
    pub /: *mut *mut u32 width; / field width,
    pub /: *mut *mut u32 parent_count; / number of entries in parent_sel[],
    pub /: *mut *mut *mut u32 parent_sel; / array of parent selector values,
    pub /: *mut *mut u8 clk_index; / current selected index in parent_sel[],
}

// Selector initialization macro

//
// Making changes to a variable divider or a selector for a clock
// requires the use of a trigger.  A trigger is defined by a single
// bit within a register.  To signal a change, a 1 is written into
// that bit.  To determine when the change has been completed, that
// trigger bit is polled; the read value will be 1 while the change
// is in progress, and 0 when it is complete.
//
// Occasionally a clock will have more than one trigger.  In this
// case, the "pre-trigger" will be used when changing a clock's
// selector and/or its pre-divider.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_clk_trig {
    pub /: *mut *mut u32 offset; / trigger register offset,
    pub /: *mut *mut u32 bit; / trigger bit,
    pub /: *mut *mut *mut u32 flags; / BCM_CLK_TRIG_FLAGS_ below,
}

//
// Trigger flags:
// EXISTS means this trigger exists
//

// Trigger initialization macro

#[repr(C)]
#[derive(Copy, Clone)]
pub struct peri_clk_data {
    pub policy: bcm_clk_policy,
    pub gate: bcm_clk_gate,
    pub hyst: bcm_clk_hyst,
    pub pre_trig: bcm_clk_trig,
    pub pre_div: bcm_clk_div,
    pub trig: bcm_clk_trig,
    pub div: bcm_clk_div,
    pub sel: bcm_clk_sel,
    pub /: *const *const *const char clocks[]; / must be last; use CLOCKS() to declare,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kona_clk {
    pub hw: clk_hw,
    pub /: *mut *mut clk_init_data init_data; / includes name of this clock,
    pub /: *mut *mut *mut ccu_data ccu; / ccu this clock is associated with,
    pub type: bcm_clk_type,
    pub data: *mut c_void,
    pub peri: *mut peri_clk_data,
    pub u: },
}

// Initialization macro for an entry in a CCU's kona_clks[] array.

//
// CCU policy control.  To enable software update of the policy
// tables the CCU policy engine must be stopped by setting the
// software update enable bit (LVM_EN).  After an update the engine
// is restarted using the GO bit and either the GO_ATL or GO_AC bit.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_lvm_en {
    pub /: *mut *mut u32 offset; / LVM_EN register offset,
    pub /: *mut *mut u32 bit; / POLICY_CONFIG_EN bit in register,
}

// Policy enable initialization macro

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_policy_ctl {
    pub /: *mut *mut u32 offset; / POLICY_CTL register offset,
    pub go_bit: u32,
    pub /: *mut *mut u32 atl_bit; / GO, GO_ATL, and GO_AC bits,
    pub ac_bit: u32,
}

// Policy control initialization macro

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccu_policy {
    pub enable: bcm_lvm_en,
    pub control: bcm_policy_ctl,
}

//
// Each CCU defines a mapped area of memory containing registers
// used to manage clocks implemented by the CCU.  Access to memory
// within the CCU's space is serialized by a spinlock.  Before any
// (other) address can be written, a special access "password" value
// must be written to its WR_ACCESS register (located at the base
// address of the range).  We keep track of the name of each CCU as
// it is set up, and maintain them in a list.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccu_data {
    pub /: *mut *mut *mut void __iomem base; / base of mapped address space,
    pub /: *mut *mut spinlock_t lock; / serialization lock,
    pub /: *mut *mut bool write_enabled; / write access is currently enabled,
    pub policy: ccu_policy,
    pub node: *mut device_node,
    pub clk_num: usize,
    pub name: *const c_char,
    pub /: *mut *mut u32 range; / byte range of address space,
    pub /: *mut *mut kona_clk kona_clks[]; / must be last,
}

// Initialization for common fields in a Kona ccu_data structure

// Exported globals
// Externally visible functions
extern "C" {
    pub fn scaled_div_max(div: *mut bcm_clk_div) -> u64;
}
extern "C" {
    pub fn kona_ccu_init(ccu: *mut ccu_data) -> bool __init;
}
