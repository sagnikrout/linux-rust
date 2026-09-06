//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/snd_wavefront.h
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

// MIDI interface
pub type snd_wavefront_midi_t = _snd_wavefront_midi;
pub type snd_wavefront_card_t = _snd_wavefront_card;
pub type snd_wavefront_t = _snd_wavefront;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _snd_wavefront_midi {
    pub /: *mut *mut unsigned long base; / I/O port address,
    pub /: *mut *mut char isvirtual; / doing virtual MIDI stuff ?,
    pub /: *mut *mut char istimer; / timer is used,
    pub /: *mut *mut snd_wavefront_mpu_id output_mpu; / most-recently-used,
    pub /: *mut *mut snd_wavefront_mpu_id input_mpu; / most-recently-used,
    pub /: *mut *mut unsigned int mode[2]; / MPU401_MODE_XXX,
    pub substream_output: [*mut snd_rawmidi_substream; 2],
    pub substream_input: [*mut snd_rawmidi_substream; 2],
    pub timer: timer_list,
    pub timer_card: *mut snd_wavefront_card_t,
    pub open: spinlock_t,
    pub /: *mut *mut spinlock_t virtual; / protects isvirtual,
}

pub const OUTPUT_READY: c_uint = 0x40;
pub const INPUT_AVAIL: c_uint = 0x80;
pub const MPU_ACK: c_uint = 0xFE;
pub const UART_MODE_ON: c_uint = 0x3F;
extern "C" {
    pub fn snd_wavefront_midi_enable_virtual(: *mut snd_wavefront_card_t);
}
extern "C" {
    pub fn snd_wavefront_midi_disable_virtual(: *mut snd_wavefront_card_t);
}
extern "C" {
    pub fn snd_wavefront_midi_interrupt(: *mut snd_wavefront_card_t);
}
extern "C" {
    pub fn snd_wavefront_midi_start(: *mut snd_wavefront_card_t) -> c_int;
}
extern "C" {
    pub fn snd_wavefront_midi_suspend(card: *mut snd_wavefront_card_t);
}
extern "C" {
    pub fn snd_wavefront_midi_resume(card: *mut snd_wavefront_card_t);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _snd_wavefront {
    pub /: *mut *mut unsigned long irq; / "you were one, one of the few ...",
    pub /: *mut *mut unsigned long base; / low i/o port address,
    pub /: *mut *mut *mut resource res_base; / i/o port resource allocation,

// FX ports. These are mapped through the ICS2115 to the YS225.
//

    pub /: *mut *mut volatile int irq_ok; / set by interrupt handler,
    pub /: *mut *mut volatile int irq_cnt; / ditto,
    pub /: *mut *mut char debug; / debugging flags,
    pub /: *mut *mut int freemem; / installed RAM, in bytes,
    pub /: *mut *mut char fw_version[2]; / major = [0], minor = [1],
    pub /: *mut *mut char hw_version[2]; / major = [0], minor = [1],
    pub /: *mut *mut char israw; / needs Motorola microcode,
    pub /: *mut *mut char has_fx; / has FX processor (Tropez+),
    pub /: *mut *mut char fx_initialized; / FX's register pages initialized,
    pub /: *mut *mut *mut char prog_status[WF_MAX_PROGRAM]; / WF_SLOT_,
    pub /: *mut *mut *mut char patch_status[WF_MAX_PATCH]; / WF_SLOT_,
    pub /: *mut *mut *mut *mut char sample_status[WF_MAX_SAMPLE]; / WF_ST_ | WF_SLOT_,
    pub /: *mut *mut int samples_used; / how many,
    pub /: *mut *mut char interrupts_are_midi; / h/w MPU interrupts enabled ?,
    pub /: *mut *mut char rom_samples_rdonly; / can we write on ROM samples,
    pub /: *mut *mut char midi_in_to_synth; / route external MIDI to synth,
    pub irq_lock: spinlock_t,
    pub interrupt_sleeper: wait_queue_head_t,
    pub /: *mut *mut snd_wavefront_midi_t midi; / ICS2115 MIDI interface,
    pub card: *mut snd_card,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _snd_wavefront_card {
    pub wavefront: snd_wavefront_t,
    pub chip: *mut snd_wss,

    pub wss: *mut pnp_dev,
    pub ctrl: *mut pnp_dev,
    pub mpu: *mut pnp_dev,
    pub synth: *mut pnp_dev,

}

extern "C" {
    pub fn snd_wavefront_internal_interrupt(card: *mut snd_wavefront_card_t);
}
extern "C" {
    pub fn snd_wavefront_cache_firmware(dev: *mut snd_wavefront_t);
}
extern "C" {
    pub fn snd_wavefront_start(dev: *mut snd_wavefront_t) -> c_int;
}
extern "C" {
    pub fn snd_wavefront_detect(card: *mut snd_wavefront_card_t) -> c_int;
}
extern "C" {
    pub fn snd_wavefront_resume_synth(card: *mut snd_wavefront_card_t) -> c_int;
}
extern "C" {
    pub fn snd_wavefront_synth_open(: *mut snd_hwdep, : *mut file) -> c_int;
}
extern "C" {
    pub fn snd_wavefront_synth_release(: *mut snd_hwdep, : *mut file) -> c_int;
}
// FX processor - see also yss225.[ch]
extern "C" {
    pub fn snd_wavefront_fx_start(: *mut snd_wavefront_t) -> c_int;
}
extern "C" {
    pub fn snd_wavefront_fx_detect(: *mut snd_wavefront_t) -> c_int;
}
extern "C" {
    pub fn snd_wavefront_fx_open(: *mut snd_hwdep, : *mut file) -> c_int;
}
extern "C" {
    pub fn snd_wavefront_fx_release(: *mut snd_hwdep, : *mut file) -> c_int;
}
