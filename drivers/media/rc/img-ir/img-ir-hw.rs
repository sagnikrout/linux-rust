//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/rc/img-ir/img-ir-hw.h
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
// ImgTec IR Hardware Decoder found in PowerDown Controller.
//
// Copyright 2010-2014 Imagination Technologies Ltd.
//

// constants
pub const IMG_IR_CODETYPE_PULSELEN: c_uint = 0x0	/* Sony */;
pub const IMG_IR_CODETYPE_PULSEDIST: c_uint = 0x1	/* NEC, Toshiba, Micom, Sharp */;
pub const IMG_IR_CODETYPE_BIPHASE: c_uint = 0x2	/* RC-5/6 */;
pub const IMG_IR_CODETYPE_2BITPULSEPOS: c_uint = 0x3	/* RC-MM */;
// Timing information
//
// struct img_ir_control - Decoder control settings
// @decoden:	Primary decoder enable
// @code_type:	Decode type (see IMG_IR_CODETYPE_*)
// @hdrtog:	Detect header toggle symbol after leader symbol
// @ldrdec:	Don't discard leader if maximum width reached
// @decodinpol:	Decoder input polarity (1=active high)
// @bitorien:	Bit orientation (1=MSB first)
// @d1validsel:	Decoder 2 takes over if it detects valid data
// @bitinv:	Bit inversion switch (1=don't invert)
// @decodend2:	Secondary decoder enable (no leader symbol)
// @bitoriend2:	Bit orientation (1=MSB first)
// @bitinvd2:	Secondary decoder bit inversion switch (1=don't invert)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_ir_control {
    pub decoden:1: unsigned,
    pub code_type:2: unsigned,
    pub hdrtog:1: unsigned,
    pub ldrdec:1: unsigned,
    pub decodinpol:1: unsigned,
    pub bitorien:1: unsigned,
    pub d1validsel:1: unsigned,
    pub bitinv:1: unsigned,
    pub decodend2:1: unsigned,
    pub bitoriend2:1: unsigned,
    pub bitinvd2:1: unsigned,
}

//
// struct img_ir_timing_range - range of timing values
// @min:	Minimum timing value
// @max:	Maximum timing value (if < @min, this will be set to @min during
// preprocessing step, so it is normally not explicitly initialised
// and is taken care of by the tolerance)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_ir_timing_range {
    pub min: u16,
    pub max: u16,
}

//
// struct img_ir_symbol_timing - timing data for a symbol
// @pulse:	Timing range for the length of the pulse in this symbol
// @space:	Timing range for the length of the space in this symbol
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_ir_symbol_timing {
    pub pulse: img_ir_timing_range,
    pub space: img_ir_timing_range,
}

//
// struct img_ir_free_timing - timing data for free time symbol
// @minlen:	Minimum number of bits of data
// @maxlen:	Maximum number of bits of data
// @ft_min:	Minimum free time after message
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_ir_free_timing {
// measured in bits
    pub minlen: u8,
    pub maxlen: u8,
    pub ft_min: u16,
}

//
// struct img_ir_timings - Timing values.
// @ldr:	Leader symbol timing data
// @s00:	Zero symbol timing data for primary decoder
// @s01:	One symbol timing data for primary decoder
// @s10:	Zero symbol timing data for secondary (no leader symbol) decoder
// @s11:	One symbol timing data for secondary (no leader symbol) decoder
// @ft:		Free time symbol timing data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_ir_timings {
    pub s11: img_ir_symbol_timing ldr, s00, s01, s10,,
    pub ft: img_ir_free_timing,
}

//
// struct img_ir_filter - Filter IR events.
// @data:	Data to match.
// @mask:	Mask of bits to compare.
// @minlen:	Additional minimum number of bits.
// @maxlen:	Additional maximum number of bits.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_ir_filter {
    pub data: u64,
    pub mask: u64,
    pub minlen: u8,
    pub maxlen: u8,
}

//
// struct img_ir_timing_regvals - Calculated timing register values.
// @ldr:	Leader symbol timing register value
// @s00:	Zero symbol timing register value for primary decoder
// @s01:	One symbol timing register value for primary decoder
// @s10:	Zero symbol timing register value for secondary decoder
// @s11:	One symbol timing register value for secondary decoder
// @ft:		Free time symbol timing register value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_ir_timing_regvals {
    pub ft: u32 ldr, s00, s01, s10, s11,,
}

//
// struct img_ir_scancode_req - Scancode request data.
// @protocol:	Protocol code of received message (defaults to
// RC_PROTO_UNKNOWN).
// @scancode:	Scan code of received message (must be written by
// handler if IMG_IR_SCANCODE is returned).
// @toggle:	Toggle bit (defaults to 0).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_ir_scancode_req {
    pub protocol: rc_proto,
    pub scancode: u32,
    pub toggle: u8,
}

//
// struct img_ir_decoder - Decoder settings for an IR protocol.
// @type:	Protocol types bitmap.
// @tolerance:	Timing tolerance as a percentage (default 10%).
// @unit:	Unit of timings in nanoseconds (default 1 us).
// @timings:	Primary timings
// @rtimings:	Additional override timings while waiting for repeats.
// @repeat:	Maximum repeat interval (always in milliseconds).
// @control:	Control flags.
//
// @scancode:	Pointer to function to convert the IR data into a scancode (it
// must be safe to execute in interrupt context).
// Returns IMG_IR_SCANCODE to emit new scancode.
// Returns IMG_IR_REPEATCODE to repeat previous code.
// Returns -errno (e.g. -EINVAL) on error.
// @filter:	Pointer to function to convert scancode filter to raw hardware
// filter. The minlen and maxlen fields will have been initialised
// to the maximum range.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_ir_decoder {
// core description
    pub type: u64,
    pub tolerance: c_uint,
    pub unit: c_uint,
    pub timings: img_ir_timings,
    pub rtimings: img_ir_timings,
    pub repeat: c_uint,
    pub control: img_ir_control,
// scancode logic
    pub request): *mut img_ir_scancode_req,
    pub protocols): *mut *mut img_ir_filter out, u64,
}

//
// struct img_ir_reg_timings - Reg values for decoder timings at clock rate.
// @ctrl:	Processed control register value.
// @timings:	Processed primary timings.
// @rtimings:	Processed repeat timings.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_ir_reg_timings {
    pub ctrl: u32,
    pub timings: img_ir_timing_regvals,
    pub rtimings: img_ir_timing_regvals,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum img_ir_mode {
    IMG_IR_M_NORMAL,
    IMG_IR_M_REPEATING,

    IMG_IR_M_WAKE,

}

//
// struct img_ir_priv_hw - Private driver data for hardware decoder.
// @ct_quirks:		Quirk bits for each code type.
// @rdev:		Remote control device
// @clk_nb:		Notifier block for clock notify events.
// @end_timer:		Timer until repeat timeout.
// @suspend_timer:	Timer to re-enable protocol.
// @decoder:		Current decoder settings.
// @enabled_protocols:	Currently enabled protocols.
// @clk_hz:		Current core clock rate in Hz.
// @reg_timings:	Timing reg values for decoder at clock rate.
// @flags:		IMG_IR_F_*.
// @filters:		HW filters (derived from scancode filters).
// @mode:		Current decode mode.
// @stopping:		Indicates that decoder is being taken down and timers
// should not be restarted.
// @suspend_irqen:	Saved IRQ enable mask over suspend.
// @quirk_suspend_irq:	Saved IRQ enable mask over quirk suspend timer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_ir_priv_hw {
    pub ct_quirks: [c_uint; 4],
    pub rdev: *mut rc_dev,
    pub clk_nb: notifier_block,
    pub end_timer: timer_list,
    pub suspend_timer: timer_list,
    pub decoder: *const img_ir_decoder,
    pub enabled_protocols: u64,
    pub clk_hz: c_ulong,
    pub reg_timings: img_ir_reg_timings,
    pub flags: c_uint,
    pub filters: [img_ir_filter; RC_FILTER_MAX],
    pub mode: img_ir_mode,
    pub stopping: bool,
    pub suspend_irqen: u32,
    pub quirk_suspend_irq: u32,
}

extern "C" {
    pub fn img_ir_isr_hw(priv: *mut img_ir_priv, irq_status: u32);
}
extern "C" {
    pub fn img_ir_setup_hw(priv: *mut img_ir_priv);
}
extern "C" {
    pub fn img_ir_probe_hw(priv: *mut img_ir_priv) -> c_int;
}
extern "C" {
    pub fn img_ir_remove_hw(priv: *mut img_ir_priv);
}

extern "C" {
    pub fn img_ir_suspend(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn img_ir_resume(dev: *mut device) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct img_ir_priv_hw {
}

