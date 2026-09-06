//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/echoaudio/echoaudio.h
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

//
// PCI vendor ID and device IDs for the hardware
//
pub const VENDOR_ID: c_uint = 0x1057;
pub const DEVICE_ID_56301: c_uint = 0x1801;
pub const DEVICE_ID_56361: c_uint = 0x3410;
pub const SUBVENDOR_ID: c_uint = 0xECC0;
//
// Valid Echo PCI subsystem card IDs
//
pub const DARLA20: c_uint = 0x0010;
pub const GINA20: c_uint = 0x0020;
pub const LAYLA20: c_uint = 0x0030;
pub const DARLA24: c_uint = 0x0040;
pub const GINA24: c_uint = 0x0050;
pub const LAYLA24: c_uint = 0x0060;
pub const MONA: c_uint = 0x0070;
pub const MIA: c_uint = 0x0080;
pub const INDIGO: c_uint = 0x0090;
pub const INDIGO_IO: c_uint = 0x00a0;
pub const INDIGO_DJ: c_uint = 0x00b0;
pub const DC8: c_uint = 0x00c0;
pub const INDIGO_IOX: c_uint = 0x00d0;
pub const INDIGO_DJX: c_uint = 0x00e0;
pub const ECHO3G: c_uint = 0x0100;
//
// Sizes
//

// pipes
pub const E3G_MAX_OUTPUTS: c_int = 16;

// entries
//
// MIDI activity indicator timeout
//
pub const MIDI_ACTIVITY_TIMEOUT_USEC: c_int = 200000;
//
// Clock numbers
//
pub const ECHO_CLOCK_INTERNAL: c_int = 0;
pub const ECHO_CLOCK_WORD: c_int = 1;
pub const ECHO_CLOCK_SUPER: c_int = 2;
pub const ECHO_CLOCK_SPDIF: c_int = 3;
pub const ECHO_CLOCK_ADAT: c_int = 4;
pub const ECHO_CLOCK_ESYNC: c_int = 5;
pub const ECHO_CLOCK_ESYNC96: c_int = 6;
pub const ECHO_CLOCK_MTC: c_int = 7;
pub const ECHO_CLOCK_NUMBER: c_int = 8;
pub const ECHO_CLOCKS: c_uint = 0xffff;
//
// Clock bit numbers - used to report capabilities and whatever clocks
// are being detected dynamically.
//

//
// Digital modes for Mona, Layla24, and Gina24
//
pub const DIGITAL_MODE_NONE: c_uint = 0xFF;
pub const DIGITAL_MODE_SPDIF_RCA: c_int = 0;
pub const DIGITAL_MODE_SPDIF_OPTICAL: c_int = 1;
pub const DIGITAL_MODE_ADAT: c_int = 2;
pub const DIGITAL_MODE_SPDIF_CDROM: c_int = 3;
pub const DIGITAL_MODES: c_int = 4;
//
// Digital mode capability masks
//

pub const EXT_3GBOX_NC: c_uint = 0x01	/* 3G box not connected */;
pub const EXT_3GBOX_NOT_SET: c_uint = 0x02	/* 3G box not detected yet */;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct audiopipe {
    pub contains: *mut *mut *mut volatile __le32 dma_counter; / Commpage register that,
// the current dma position
// (lower 32 bits only)
//
    pub a: *mut *mut u32 last_period; / Counter position last time,
// period elapsed
//
    pub pcm_pointer: *mut *mut u32 last_counter; / Used exclusively by,
// under PCM core locks.
// The last position, which is used
// to compute...
//
    pub tranferred: *mut *mut u32 position; / ...the number of bytes,
// by the DMA engine, modulo the
// buffer size
//
    pub <0: *mut *mut short index; / Index of the first channel or,
// if hw is not configured yet
//
    pub interleave: c_short,
    pub /: *mut *mut snd_dma_buffer sgpage; / Room for the scatter-gather list,
    pub hw: snd_pcm_hardware,
    pub constr: snd_pcm_hw_constraint_list,
    pub sglist_head: c_short,
    pub /: *mut *mut char state; / pipe state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct audioformat {
    pub memory:: *mut *mut u8 interleave; / How the data is arranged in,
// mono = 1, stereo = 2, ...
//
    pub /: *mut *mut u8 bits_per_sample; / 8, 16, 24, 32 (24 bits left aligned),
    pub and: *mut *mut char mono_to_stereo; / Only used if interleave is 1,
// if this is an output pipe.
//
    pub /: *mut *mut char data_are_bigendian; / 1 = big endian, 0 = little endian,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct echoaudio {
    pub lock: spinlock_t,
    pub substream: [*mut snd_pcm_substream; DSP_MAXPIPES],
    pub mode_mutex: mutex,
    pub digital_mode_list: [u16 num_digital_modes,; 6],
    pub clock_source_list: [u16 num_clock_sources,; 10],
    pub /: *mut *mut unsigned int opencount; / protected by mode_mutex,
    pub clock_src_ctl: *mut snd_kcontrol,
    pub digital_pcm: *mut *mut snd_pcm analog_pcm,,
    pub card: *mut snd_card,
    pub card_name: *const c_char,
    pub pci: *mut pci_dev,
    pub dsp_registers_phys: c_ulong,
    pub iores: *mut resource,
    pub commpage_dma_buf: *mut snd_dma_buffer,
    pub irq: c_int,

    pub rmidi: *mut snd_rawmidi,
    pub midi_out: *mut *mut snd_rawmidi_substream midi_in,,

    pub timer: timer_list,
    pub /: *mut *mut char tinuse; / Timer in use,
    pub /: *mut *mut char midi_full; / MIDI output buffer is full,
    pub /: *mut *mut char can_set_rate; / protected by mode_mutex,
    pub /: *mut *mut char rate_set; / protected by mode_mutex,
// This stuff is used mainly by the lowlevel code
    pub memory: *mut *mut *mut comm_page comm_page; / Virtual address of the,
// seen by DSP
//
    pub /: *mut *mut u32 pipe_alloc_mask; / Bitmask of allocated pipes,
    pub cyclic: *mut *mut u32 pipe_cyclic_mask; / Bitmask of pipes with,
// buffers
//
    pub /: *mut *mut u32 sample_rate; / Card sample rate in Hz,
    pub mode: *mut *mut u8 digital_mode; / Current digital,
// (see DIGITAL_MODE_*)
//
    pub /: *mut *mut u8 spdif_status; / Gina20, Darla20, Darla24 - only,
    pub /: *mut *mut u8 clock_state; / Gina20, Darla20, Darla24 - only,
    pub clock: *mut *mut u8 input_clock; / Currently selected sample,
// source
//
    pub /: *mut *mut u8 output_clock; / Layla20 only,
    pub /: *mut *mut char meters_enabled; / VU-meters status,
    pub /: *mut *mut char asic_loaded; / Set true when ASIC loaded,
    pub /: *mut *mut char bad_board; / Set true if DSP won't load,
    pub /: *mut *mut char professional_spdif; / 0 = consumer; 1 = professional,
    pub /: *mut *mut char non_audio_spdif; / 3G - only,
    pub /: *mut *mut char digital_in_automute; / Gina24, Layla24, Mona - only,
    pub has_phantom_power: c_char,
    pub /: *mut *mut char hasnt_input_nominal_level; / Gina3G,
    pub /: *mut *mut char phantom_power; / Gina3G - only,
    pub has_midi: c_char,
    pub midi_input_enabled: c_char,

// External module -dependent pipe and bus indexes
    pub px_num: char px_digital_out, px_analog_in, px_digital_in,,
    pub bx_num: char bx_digital_out, bx_analog_in, bx_digital_in,,

    pub -10dBV: *mut *mut char nominal_level[ECHO_MAXAUDIOPIPES]; / True ==,
// False == +4dBu
    pub -50..+50: *mut *mut s8 input_gain[ECHO_MAXAUDIOINPUTS]; / Input level,
// unit is 0.5dB
    pub dB: *mut *mut s8 output_gain[ECHO_MAXAUDIOOUTPUTS]; / Output level -128..+6,
// (-128=muted)
    pub monitor_gain: [i8; ECHO_MAXAUDIOOUTPUTS][ECHO_MAXAUDIOINPUTS],
// -128..+6 dB
    pub vmixer_gain: [i8; ECHO_MAXAUDIOOUTPUTS][ECHO_MAXAUDIOOUTPUTS],
// -128..+6 dB
    pub modes: *mut *mut u16 digital_modes; / Bitmask of supported,
// (see ECHOCAPS_HAS_DIGITAL_MODE_*)
    pub /: *mut *mut u16 input_clock_types; / Suppoted input clock types,
    pub -: *mut *mut u16 output_clock_types; / Suppoted output clock types,
// Layla20 only
    pub subdevice_id: u16 device_id,,
    pub loaded,: *mut *mut *mut u16 dsp_code; / Current DSP code,
// NULL if nothing loaded
    pub /: *mut *mut short dsp_code_to_load; / DSP code to load,
    pub /: *mut *mut short asic_code; / Current ASIC code,
    pub the: *mut *mut u32 comm_page_phys; / Physical address of,
// memory seen by DSP
    pub /: *mut *mut *mut u32 __iomem dsp_registers; / DSP's register base,
    pub or: *mut *mut u32 active_mask; / Chs. active mask,
// punks out
    pub /: *const *const *const firmware fw_cache[8]; / Cached firmwares,

    pub /: *mut *mut u16 mtc_state; / State for MIDI input parsing state machine,
    pub midi_buffer: [u8; MIDI_IN_BUFFER_SIZE],
}

extern "C" {
    pub fn init_dsp_comm_page(chip: *mut echoaudio) -> static int;
}
extern "C" {
    pub fn init_line_levels(chip: *mut echoaudio) -> static int;
}
extern "C" {
    pub fn free_pipes(chip: *mut echoaudio, pipe: *mut audiopipe) -> static int;
}
extern "C" {
    pub fn load_firmware(chip: *mut echoaudio) -> static int;
}
extern "C" {
    pub fn wait_handshake(chip: *mut echoaudio) -> static int;
}
extern "C" {
    pub fn send_vector(chip: *mut echoaudio, command: u32) -> static int;
}

extern "C" {
    pub fn enable_midi_input(chip: *mut echoaudio, enable: c_char) -> static int;
}
extern "C" {
    pub fn midi_service_irq(chip: *mut echoaudio) -> static int;
}

extern "C" {
    pub fn readl(_arg: &chip->dsp_registers[index]) -> return;
}
// Pipe and bus indexes. PX_* and BX_* are defined as chip->px_* and chip->bx_
extern "C" {
    pub fn px_analog_in(_arg: chip) -> return;
}
extern "C" {
    pub fn px_num(px_analog_in(chip: chip) -) -> return;
}
extern "C" {
    pub fn bx_analog_in(_arg: chip) -> return;
}
extern "C" {
    pub fn bx_num(bx_analog_in(chip: chip) -) -> return;
}
extern "C" {
    pub fn bx_digital_out(_arg: chip) -> return;
}
extern "C" {
    pub fn bx_digital_in(bx_analog_in(chip: chip) -) -> return;
}
extern "C" {
    pub fn num_busses_out(num_analog_busses_out(chip: chip) -) -> return;
}
extern "C" {
    pub fn num_busses_in(num_analog_busses_in(chip: chip) -) -> return;
}
// The monitor array is a one-dimensional array; compute the offset
// into the array
