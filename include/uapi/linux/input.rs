//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/input.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Copyright (c) 1999-2002 Vojtech Pavlik
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License version 2 as published by
// the Free Software Foundation.
//

//
// The event structure itself
// Note that __USE_TIME_BITS64 is defined by libc based on
// application's request to use 64 bit time_t.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct input_event {

    pub time: timeval,

    pub __sec: __kernel_ulong_t,

    pub __usec: c_uint,
    pub __pad: c_uint,

    pub __usec: __kernel_ulong_t,

    pub type: __u16,
    pub code: __u16,
    pub value: __s32,
}

//
// Protocol version.
//
pub const EV_VERSION: c_uint = 0x010001;
//
// IOCTLs (0x00 - 0x7f)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct input_id {
    pub bustype: __u16,
    pub vendor: __u16,
    pub product: __u16,
    pub version: __u16,
}

//
// struct input_absinfo - used by EVIOCGABS/EVIOCSABS ioctls
// @value: latest reported value for the axis.
// @minimum: specifies minimum value for the axis.
// @maximum: specifies maximum value for the axis.
// @fuzz: specifies fuzz value that is used to filter noise from
// the event stream.
// @flat: values that are within this value will be discarded by
// joydev interface and reported as 0 instead.
// @resolution: specifies resolution for the values reported for
// the axis.
//
// Note that input core does not clamp reported values to the
// [minimum, maximum] limits, such task is left to userspace.
//
// The default resolution for main axes (ABS_X, ABS_Y, ABS_Z,
// ABS_MT_POSITION_X, ABS_MT_POSITION_Y) is reported in units
// per millimeter (units/mm), resolution for rotational axes
// (ABS_RX, ABS_RY, ABS_RZ) is reported in units per radian.
// The resolution for the size axes (ABS_MT_TOUCH_MAJOR,
// ABS_MT_TOUCH_MINOR, ABS_MT_WIDTH_MAJOR, ABS_MT_WIDTH_MINOR)
// is reported in units per millimeter (units/mm).
// When INPUT_PROP_ACCELEROMETER is set the resolution changes.
// The main axes (ABS_X, ABS_Y, ABS_Z) are then reported in
// units per g (units/g) and in units per degree per second
// (units/deg/s) for rotational axes (ABS_RX, ABS_RY, ABS_RZ).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct input_absinfo {
    pub value: __s32,
    pub minimum: __s32,
    pub maximum: __s32,
    pub fuzz: __s32,
    pub flat: __s32,
    pub resolution: __s32,
}

//
// struct input_keymap_entry - used by EVIOCGKEYCODE/EVIOCSKEYCODE ioctls
// @scancode: scancode represented in machine-endian form.
// @len: length of the scancode that resides in @scancode buffer.
// @index: index in the keymap, may be used instead of scancode
// @flags: allows to specify how kernel should handle the request. For
// example, setting INPUT_KEYMAP_BY_INDEX flag indicates that kernel
// should perform lookup in keymap by @index instead of @scancode
// @keycode: key code assigned to this scancode
//
// The structure is used to retrieve and modify keymap data. Users have
// option of performing lookup either by @scancode itself or by @index
// in keymap entry. EVIOCGKEYCODE will also return scancode or index
// (depending on which element was used to perform lookup).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct input_keymap_entry {

    pub flags: __u8,
    pub len: __u8,
    pub index: __u16,
    pub keycode: __u32,
    pub scancode: [__u8; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct input_mask {
    pub type: __u32,
    pub codes_size: __u32,
    pub codes_ptr: __u64,
}

//
// EVIOCGMTSLOTS(len) - get MT slot values
// @len: size of the data buffer in bytes
//
// The ioctl buffer argument should be binary equivalent to
//
// struct input_mt_request_layout {
// __u32 code;
// __s32 values[num_slots];
// };
//
// where num_slots is the (arbitrary) number of MT slots to extract.
//
// The ioctl size argument (len) is the size of the buffer, which
// should satisfy len = (num_slots + 1) * sizeof(__s32).  If len is
// too small to fit all available slots, the first num_slots are
// returned.
//
// Before the call, code is set to the wanted ABS_MT event type. On
// return, values[] is filled with the slot values for the specified
// ABS_MT code.
//
// If the request code is not an ABS_MT value, -EINVAL is returned.
//

//
// EVIOCGMASK - Retrieve current event mask
//
// This ioctl allows user to retrieve the current event mask for specific
// event type. The argument must be of type "struct input_mask" and
// specifies the event type to query, the address of the receive buffer and
// the size of the receive buffer.
//
// The event mask is a per-client mask that specifies which events are
// forwarded to the client. Each event code is represented by a single bit
// in the event mask. If the bit is set, the event is passed to the client
// normally. Otherwise, the event is filtered and will never be queued on
// the client's receive buffer.
//
// Event masks do not affect global state of the input device. They only
// affect the file descriptor they are applied to.
//
// The default event mask for a client has all bits set, i.e. all events
// are forwarded to the client. If the kernel is queried for an unknown
// event type or if the receive buffer is larger than the number of
// event codes known to the kernel, the kernel returns all zeroes for those
// codes.
//
// At maximum, codes_size bytes are copied.
//
// This ioctl may fail with ENODEV in case the file is revoked, EFAULT
// if the receive-buffer points to invalid memory, or EINVAL if the kernel
// does not implement the ioctl.
//

//
// EVIOCSMASK - Set event mask
//
// This ioctl is the counterpart to EVIOCGMASK. Instead of receiving the
// current event mask, this changes the client's event mask for a specific
// type.  See EVIOCGMASK for a description of event-masks and the
// argument-type.
//
// This ioctl provides full forward compatibility. If the passed event type
// is unknown to the kernel, or if the number of event codes specified in
// the mask is bigger than what is known to the kernel, the ioctl is still
// accepted and applied. However, any unknown codes are left untouched and
// stay cleared. That means, the kernel always filters unknown codes
// regardless of what the client requests.  If the new mask doesn't cover
// all known event-codes, all remaining codes are automatically cleared and
// thus filtered.
//
// This ioctl may fail with ENODEV in case the file is revoked. EFAULT is
// returned if the receive-buffer points to invalid memory. EINVAL is returned
// if the kernel does not implement the ioctl.
//

//
// IDs.
//
pub const ID_BUS: c_int = 0;
pub const ID_VENDOR: c_int = 1;
pub const ID_PRODUCT: c_int = 2;
pub const ID_VERSION: c_int = 3;
pub const BUS_PCI: c_uint = 0x01;
pub const BUS_ISAPNP: c_uint = 0x02;
pub const BUS_USB: c_uint = 0x03;
pub const BUS_HIL: c_uint = 0x04;
pub const BUS_BLUETOOTH: c_uint = 0x05;
pub const BUS_VIRTUAL: c_uint = 0x06;
pub const BUS_ISA: c_uint = 0x10;
pub const BUS_I8042: c_uint = 0x11;
pub const BUS_XTKBD: c_uint = 0x12;
pub const BUS_RS232: c_uint = 0x13;
pub const BUS_GAMEPORT: c_uint = 0x14;
pub const BUS_PARPORT: c_uint = 0x15;
pub const BUS_AMIGA: c_uint = 0x16;
pub const BUS_ADB: c_uint = 0x17;
pub const BUS_I2C: c_uint = 0x18;
pub const BUS_HOST: c_uint = 0x19;
pub const BUS_GSC: c_uint = 0x1A;
pub const BUS_ATARI: c_uint = 0x1B;
pub const BUS_SPI: c_uint = 0x1C;
pub const BUS_RMI: c_uint = 0x1D;
pub const BUS_CEC: c_uint = 0x1E;
pub const BUS_INTEL_ISHTP: c_uint = 0x1F;
pub const BUS_AMD_SFH: c_uint = 0x20;
pub const BUS_SDW: c_uint = 0x21;
//
// MT_TOOL types
//
pub const MT_TOOL_FINGER: c_uint = 0x00;
pub const MT_TOOL_PEN: c_uint = 0x01;
pub const MT_TOOL_PALM: c_uint = 0x02;
pub const MT_TOOL_DIAL: c_uint = 0x0a;
pub const MT_TOOL_MAX: c_uint = 0x0f;
//
// Values describing the status of a force-feedback effect
//
pub const FF_STATUS_STOPPED: c_uint = 0x00;
pub const FF_STATUS_PLAYING: c_uint = 0x01;
pub const FF_STATUS_MAX: c_uint = 0x01;
//
// Structures used in ioctls to upload effects to a device
// They are pieces of a bigger structure (called ff_effect)
//
// All duration values are expressed in ms. Values above 32767 ms (0x7fff)
// should not be used and have unspecified results.
//
// struct ff_replay - defines scheduling of the force-feedback effect
// @length: duration of the effect
// @delay: delay before effect should start playing
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ff_replay {
    pub length: __u16,
    pub delay: __u16,
}

//
// struct ff_trigger - defines what triggers the force-feedback effect
// @button: number of the button triggering the effect
// @interval: controls how soon the effect can be re-triggered
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ff_trigger {
    pub button: __u16,
    pub interval: __u16,
}

//
// struct ff_envelope - generic force-feedback effect envelope
// @attack_length: duration of the attack (ms)
// @attack_level: level at the beginning of the attack
// @fade_length: duration of fade (ms)
// @fade_level: level at the end of fade
//
// The @attack_level and @fade_level are absolute values; when applying
// envelope force-feedback core will convert to positive/negative
// value based on polarity of the default level of the effect.
// Valid range for the attack and fade levels is 0x0000 - 0x7fff
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ff_envelope {
    pub attack_length: __u16,
    pub attack_level: __u16,
    pub fade_length: __u16,
    pub fade_level: __u16,
}

//
// struct ff_constant_effect - defines parameters of a constant force-feedback effect
// @level: strength of the effect; may be negative
// @envelope: envelope data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ff_constant_effect {
    pub level: __s16,
    pub envelope: ff_envelope,
}

//
// struct ff_ramp_effect - defines parameters of a ramp force-feedback effect
// @start_level: beginning strength of the effect; may be negative
// @end_level: final strength of the effect; may be negative
// @envelope: envelope data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ff_ramp_effect {
    pub start_level: __s16,
    pub end_level: __s16,
    pub envelope: ff_envelope,
}

//
// struct ff_condition_effect - defines a spring or friction force-feedback effect
// @right_saturation: maximum level when joystick moved all way to the right
// @left_saturation: same for the left side
// @right_coeff: controls how fast the force grows when the joystick moves
// to the right
// @left_coeff: same for the left side
// @deadband: size of the dead zone, where no force is produced
// @center: position of the dead zone
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ff_condition_effect {
    pub right_saturation: __u16,
    pub left_saturation: __u16,
    pub right_coeff: __s16,
    pub left_coeff: __s16,
    pub deadband: __u16,
    pub center: __s16,
}

//
// struct ff_periodic_effect - defines parameters of a periodic force-feedback effect
// @waveform: kind of the effect (wave)
// @period: period of the wave (ms)
// @magnitude: peak value
// @offset: mean value of the wave (roughly)
// @phase: 'horizontal' shift
// @envelope: envelope data
// @custom_len: number of samples (FF_CUSTOM only)
// @custom_data: buffer of samples (FF_CUSTOM only)
//
// Known waveforms - FF_SQUARE, FF_TRIANGLE, FF_SINE, FF_SAW_UP,
// FF_SAW_DOWN, FF_CUSTOM. The exact syntax FF_CUSTOM is undefined
// for the time being as no driver supports it yet.
//
// Note: the data pointed by custom_data is copied by the driver.
// You can therefore dispose of the memory after the upload/update.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ff_periodic_effect {
    pub waveform: __u16,
    pub period: __u16,
    pub magnitude: __s16,
    pub offset: __s16,
    pub phase: __u16,
    pub envelope: ff_envelope,
    pub custom_len: __u32,
    pub custom_data: *mut __s16 __user,
}

//
// struct ff_rumble_effect - defines parameters of a periodic force-feedback effect
// @strong_magnitude: magnitude of the heavy motor
// @weak_magnitude: magnitude of the light one
//
// Some rumble pads have two motors of different weight. Strong_magnitude
// represents the magnitude of the vibration generated by the heavy one.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ff_rumble_effect {
    pub strong_magnitude: __u16,
    pub weak_magnitude: __u16,
}

//
// struct ff_haptic_effect
// @hid_usage: hid_usage according to Haptics page (WAVEFORM_CLICK, etc.)
// @vendor_id: the waveform vendor ID if hid_usage is in the vendor-defined range
// @vendor_waveform_page: the vendor waveform page if hid_usage is in the vendor-defined range
// @intensity: strength of the effect as percentage
// @repeat_count: number of times to retrigger effect
// @retrigger_period: time before effect is retriggered (in ms)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ff_haptic_effect {
    pub hid_usage: __u16,
    pub vendor_id: __u16,
    pub vendor_waveform_page: __u8,
    pub intensity: __u16,
    pub repeat_count: __u16,
    pub retrigger_period: __u16,
}

//
// struct ff_effect - defines force feedback effect
// @type: type of the effect (FF_CONSTANT, FF_PERIODIC, FF_RAMP, FF_SPRING,
// FF_FRICTION, FF_DAMPER, FF_RUMBLE, FF_INERTIA, or FF_CUSTOM)
// @id: an unique id assigned to an effect
// @direction: direction of the effect
// @trigger: trigger conditions (struct ff_trigger)
// @replay: scheduling of the effect (struct ff_replay)
// @u: effect-specific structure (one of ff_constant_effect, ff_ramp_effect,
// ff_periodic_effect, ff_condition_effect, ff_rumble_effect) further
// defining effect parameters
//
// This structure is sent through ioctl from the application to the driver.
// To create a new effect application should set its @id to -1; the kernel
// will return assigned @id which can later be used to update or delete
// this effect.
//
// Direction of the effect is encoded as follows:
// 0 deg -> 0x0000 (down)
// 90 deg -> 0x4000 (left)
// 180 deg -> 0x8000 (up)
// 270 deg -> 0xC000 (right)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ff_effect {
    pub type: __u16,
    pub id: __s16,
    pub direction: __u16,
    pub trigger: ff_trigger,
    pub replay: ff_replay,
    pub constant: ff_constant_effect,
    pub ramp: ff_ramp_effect,
    pub periodic: ff_periodic_effect,
    pub /: *mut *mut ff_condition_effect condition[2]; / One for each axis,
    pub rumble: ff_rumble_effect,
    pub haptic: ff_haptic_effect,
    pub u: },
}

//
// Force feedback effect types
//
pub const FF_HAPTIC: c_uint = 0x4f;
pub const FF_RUMBLE: c_uint = 0x50;
pub const FF_PERIODIC: c_uint = 0x51;
pub const FF_CONSTANT: c_uint = 0x52;
pub const FF_SPRING: c_uint = 0x53;
pub const FF_FRICTION: c_uint = 0x54;
pub const FF_DAMPER: c_uint = 0x55;
pub const FF_INERTIA: c_uint = 0x56;
pub const FF_RAMP: c_uint = 0x57;

//
// Force feedback periodic effect types
//
pub const FF_SQUARE: c_uint = 0x58;
pub const FF_TRIANGLE: c_uint = 0x59;
pub const FF_SINE: c_uint = 0x5a;
pub const FF_SAW_UP: c_uint = 0x5b;
pub const FF_SAW_DOWN: c_uint = 0x5c;
pub const FF_CUSTOM: c_uint = 0x5d;

//
// Set ff device properties
//
pub const FF_GAIN: c_uint = 0x60;
pub const FF_AUTOCENTER: c_uint = 0x61;
//
// ff->playback(effect_id = FF_GAIN) is the first effect_id to
// cause a collision with another ff method, in this case ff->set_gain().
// Therefore the greatest safe value for effect_id is FF_GAIN - 1,
// and thus the total number of effects should never exceed FF_GAIN.
//

pub const FF_MAX: c_uint = 0x7f;

