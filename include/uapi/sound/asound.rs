//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/sound/asound.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// Advanced Linux Sound Architecture - ALSA - Driver
// Copyright (c) 1994-2003 by Jaroslav Kysela <perex@perex.cz>,
// Abramo Bagnara <abramo@alsa-project.org>
//

//
// protocol version
//

//
// Digital audio interface
//
pub const AES_IEC958_STATUS_SIZE: c_int = 24;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_aes_iec958 {
    pub /: *mut *mut unsigned char status[AES_IEC958_STATUS_SIZE]; / AES/IEC958 channel status bits,
    pub /: *mut *mut unsigned char subcode[147]; / AES/IEC958 subcode bits,
    pub /: *mut *mut unsigned char pad; / nothing,
    pub /: *mut *mut unsigned char dig_subframe[4]; / AES/IEC958 subframe bits,
}

//
// CEA-861 Audio InfoFrame. Used in HDMI and DisplayPort
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_cea_861_aud_if {
    pub /: *mut *mut unsigned char db1_ct_cc; / coding type and channel count,
    pub /: *mut *mut unsigned char db2_sf_ss; / sample frequency and size,
    pub /: *mut *mut unsigned char db3; / not used, all zeros,
    pub /: *mut *mut unsigned char db4_ca; / channel allocation code,
    pub /: *mut *mut unsigned char db5_dminh_lsv; / downmix inhibit & level-shift values,
}

//
// Section for driver hardware dependent interface - /dev/snd/hw?
//

// Don't forget to change the following:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_hwdep_info {
    pub /: *mut *mut unsigned int device; / WR: device number,
    pub /: *mut *mut int card; / R: card number,
    pub /: *mut *mut unsigned char id[64]; / ID (user selectable),
    pub /: *mut *mut unsigned char name[80]; / hwdep name,
    pub /: *mut *mut int iface; / hwdep interface,
    pub /: *mut *mut unsigned char reserved[64]; / reserved for future,
}

// generic DSP loader
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_hwdep_dsp_status {
    pub /: *mut *mut unsigned int version; / R: driver-specific version,
    pub /: *mut *mut unsigned char id[32]; / R: driver-specific ID string,
    pub /: *mut *mut unsigned int num_dsps; / R: number of DSP images to transfer,
    pub /: *mut *mut unsigned int dsp_loaded; / R: bit flags indicating the loaded DSPs,
    pub /: *mut *mut unsigned int chip_ready; / R: 1 = initialization finished,
    pub /: *mut *mut unsigned char reserved[16]; / reserved for future use,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_hwdep_dsp_image {
    pub /: *mut *mut unsigned int index; / W: DSP index,
    pub /: *mut *mut unsigned char name[64]; / W: ID (e.g. file name),
    pub /: *mut *mut *mut unsigned char __user image; / W: binary image,
    pub /: *mut *mut size_t length; / W: size of image in bytes,
    pub /: *mut *mut unsigned long driver_data; / W: driver-specific data,
}

//
// Digital Audio (PCM) interface - /dev/snd/pcm??
//

pub type snd_pcm_uframes_t = c_ulong;
pub type snd_pcm_sframes_t = signed long;
// Don't forget to change the following:
pub type snd_pcm_access_t = c_int;

pub type snd_pcm_format_t = c_int;
pub const SNDRV_PCM_FORMAT_S8: c_int = 0;
pub const SNDRV_PCM_FORMAT_U8: c_int = 1;
pub const SNDRV_PCM_FORMAT_S16_LE: c_int = 2;
pub const SNDRV_PCM_FORMAT_S16_BE: c_int = 3;
pub const SNDRV_PCM_FORMAT_U16_LE: c_int = 4;
pub const SNDRV_PCM_FORMAT_U16_BE: c_int = 5;

//
// For S32/U32 formats, 'msbits' hardware parameter is often used to deliver information about the
// available bit count in most significant bit. It's for the case of so-called 'left-justified' or
// `right-padding` sample which has less width than 32 bit.
//
pub const SNDRV_PCM_FORMAT_S32_LE: c_int = 10;
pub const SNDRV_PCM_FORMAT_S32_BE: c_int = 11;
pub const SNDRV_PCM_FORMAT_U32_LE: c_int = 12;
pub const SNDRV_PCM_FORMAT_U32_BE: c_int = 13;

pub const SNDRV_PCM_FORMAT_MU_LAW: c_int = 20;
pub const SNDRV_PCM_FORMAT_A_LAW: c_int = 21;
pub const SNDRV_PCM_FORMAT_IMA_ADPCM: c_int = 22;
pub const SNDRV_PCM_FORMAT_MPEG: c_int = 23;
pub const SNDRV_PCM_FORMAT_GSM: c_int = 24;

// gap in the numbering for a future standard linear format
pub const SNDRV_PCM_FORMAT_SPECIAL: c_int = 31;

pub type snd_pcm_subformat_t = c_int;
pub const SNDRV_PCM_SUBFORMAT_STD: c_int = 0;
pub const SNDRV_PCM_SUBFORMAT_MSBITS_MAX: c_int = 1;
pub const SNDRV_PCM_SUBFORMAT_MSBITS_20: c_int = 2;
pub const SNDRV_PCM_SUBFORMAT_MSBITS_24: c_int = 3;

pub const SNDRV_PCM_INFO_MMAP: c_uint = 0x00000001	/* hardware supports mmap */;
pub const SNDRV_PCM_INFO_MMAP_VALID: c_uint = 0x00000002	/* period data are valid during transfer */;
pub const SNDRV_PCM_INFO_DOUBLE: c_uint = 0x00000004	/* Double buffering needed for PCM start/stop */;
pub const SNDRV_PCM_INFO_BATCH: c_uint = 0x00000010	/* double buffering */;
pub const SNDRV_PCM_INFO_SYNC_APPLPTR: c_uint = 0x00000020	/* need the explicit sync of appl_ptr update */;
pub const SNDRV_PCM_INFO_PERFECT_DRAIN: c_uint = 0x00000040	/* silencing at the end of stream is not required */;
pub const SNDRV_PCM_INFO_INTERLEAVED: c_uint = 0x00000100	/* channels are interleaved */;
pub const SNDRV_PCM_INFO_NONINTERLEAVED: c_uint = 0x00000200	/* channels are not interleaved */;
pub const SNDRV_PCM_INFO_COMPLEX: c_uint = 0x00000400	/* complex frame organization (mmap only) */;
pub const SNDRV_PCM_INFO_BLOCK_TRANSFER: c_uint = 0x00010000	/* hardware transfer block of samples */;
pub const SNDRV_PCM_INFO_OVERRANGE: c_uint = 0x00020000	/* hardware supports ADC (capture) overrange detection */;
pub const SNDRV_PCM_INFO_RESUME: c_uint = 0x00040000	/* hardware supports stream resume after suspend */;
pub const SNDRV_PCM_INFO_PAUSE: c_uint = 0x00080000	/* pause ioctl is supported */;
pub const SNDRV_PCM_INFO_HALF_DUPLEX: c_uint = 0x00100000	/* only half duplex */;
pub const SNDRV_PCM_INFO_JOINT_DUPLEX: c_uint = 0x00200000	/* playback and capture stream are somewhat correlated */;
pub const SNDRV_PCM_INFO_SYNC_START: c_uint = 0x00400000	/* pcm support some kind of sync go */;
pub const SNDRV_PCM_INFO_NO_PERIOD_WAKEUP: c_uint = 0x00800000	/* period wakeup can be disabled */;
pub const SNDRV_PCM_INFO_HAS_WALL_CLOCK: c_uint = 0x01000000      /* (Deprecated)has audio wall clock for audio/system time sync */;
pub const SNDRV_PCM_INFO_HAS_LINK_ATIME: c_uint = 0x01000000  /* report hardware link audio time, reset on startup */;
pub const SNDRV_PCM_INFO_HAS_LINK_ABSOLUTE_ATIME: c_uint = 0x02000000  /* report absolute hardware link audio time, not reset on startup */;
pub const SNDRV_PCM_INFO_HAS_LINK_ESTIMATED_ATIME: c_uint = 0x04000000  /* report estimated link audio time */;
pub const SNDRV_PCM_INFO_HAS_LINK_SYNCHRONIZED_ATIME: c_uint = 0x08000000  /* report synchronized audio/system time */;
pub const SNDRV_PCM_INFO_EXPLICIT_SYNC: c_uint = 0x10000000	/* needs explicit sync of pointers and data */;
pub const SNDRV_PCM_INFO_NO_REWINDS: c_uint = 0x20000000	/* hardware can only support monotonic changes of appl_ptr */;
pub const SNDRV_PCM_INFO_DRAIN_TRIGGER: c_uint = 0x40000000		/* internal kernel flag - trigger in drain */;
pub const SNDRV_PCM_INFO_FIFO_IN_FRAMES: c_uint = 0x80000000	/* internal kernel flag - FIFO size is in frames */;

pub type snd_pcm_state_t = c_int;

#[repr(C)]
#[derive(Copy, Clone)]
pub union snd_pcm_sync_id {
    pub id: [c_uchar; 16],
    pub id16: [c_ushort; 8],
    pub id32: [c_uint; 4],
    pub __attribute__((deprecated)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_info {
    pub /: *mut *mut unsigned int device; / RO/WR (control): device number,
    pub /: *mut *mut unsigned int subdevice; / RO/WR (control): subdevice number,
    pub /: *mut *mut int stream; / RO/WR (control): stream direction,
    pub /: *mut *mut int card; / R: card number,
    pub /: *mut *mut unsigned char id[64]; / ID (user selectable),
    pub /: *mut *mut unsigned char name[80]; / name of this device,
    pub /: *mut *mut unsigned char subname[32]; / subdevice name,
    pub /: *mut *mut *mut int dev_class; / SNDRV_PCM_CLASS_,
    pub /: *mut *mut *mut int dev_subclass; / SNDRV_PCM_SUBCLASS_,
    pub subdevices_count: c_uint,
    pub subdevices_avail: c_uint,
    pub /: *mut *mut unsigned char pad1[16]; / was: hardware synchronization ID,
    pub /: *mut *mut unsigned char reserved[64]; / reserved for future...,
}

pub type snd_pcm_hw_param_t = c_int;

// interrupts in us
//

// interrupts
//

// interrupts
//

// buffer
//

// in us
//

// of the silence samples
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_interval {
    pub max: unsigned int min,,
}

pub const SNDRV_MASK_MAX: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_mask {
    pub bits: [__u32; (SNDRV_MASK_MAX+31)/32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_hw_params {
    pub flags: c_uint,
    pub 1]: SNDRV_PCM_HW_PARAM_FIRST_MASK +,
    pub /: *mut *mut snd_mask mres[5]; / reserved masks,
    pub 1]: SNDRV_PCM_HW_PARAM_FIRST_INTERVAL +,
    pub /: *mut *mut snd_interval ires[9]; / reserved intervals,
    pub /: *mut *mut unsigned int rmask; / W: requested masks,
    pub /: *mut *mut unsigned int cmask; / R: changed masks,
    pub /: *mut *mut unsigned int info; / R: Info flags for returned setup,
    pub /: *mut *mut unsigned int msbits; / R: used most significant bits (in sample bit-width),
    pub /: *mut *mut unsigned int rate_num; / R: rate numerator,
    pub /: *mut *mut unsigned int rate_den; / R: rate denominator,
    pub /: *mut *mut snd_pcm_uframes_t fifo_size; / R: chip FIFO size in frames,
    pub /: *mut *mut unsigned char sync[16]; / R: synchronization ID (perfect sync - one clock source),
    pub /: *mut *mut unsigned char reserved[48]; / reserved for future,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_sw_params {
    pub /: *mut *mut int tstamp_mode; / timestamp mode,
    pub period_step: c_uint,
    pub /: *mut *mut unsigned int sleep_min; / min ticks to sleep,
    pub /: *mut *mut snd_pcm_uframes_t avail_min; / min avail frames for wakeup,
    pub /: *mut *mut snd_pcm_uframes_t xfer_align; / obsolete: xfer size need to be a multiple,
    pub /: *mut *mut snd_pcm_uframes_t start_threshold; / min hw_avail frames for automatic start,
//
// The following two thresholds alleviate playback buffer underruns; when
// hw_avail drops below the threshold, the respective action is triggered:
//
    pub /: *mut *mut snd_pcm_uframes_t stop_threshold; / - stop playback,
    pub /: *mut *mut snd_pcm_uframes_t silence_threshold; / - pre-fill buffer with silence,
    pub boundary,: *mut *mut snd_pcm_uframes_t silence_size; / max size of silence pre-fill; when >=,
// fill played area with silence immediately
    pub /: *mut *mut snd_pcm_uframes_t boundary; / pointers wrap point,
    pub /: *mut *mut unsigned int proto; / protocol version,
    pub /: *mut *mut unsigned int tstamp_type; / timestamp type (req. proto >= 2.0.12),
    pub /: *mut *mut unsigned char reserved[56]; / reserved for future,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_channel_info {
    pub channel: c_uint,
    pub /: *mut *mut __kernel_off_t offset; / mmap offset,
    pub /: *mut *mut unsigned int first; / offset to first sample in bits,
    pub /: *mut *mut unsigned int step; / samples distance in bits,
}

//
// first definition for backwards compatibility only,
// maps to wallclock/link time for HDAudio playback and DEFAULT/DMA time for everything else
//
// timestamp definitions
// explicit padding avoids incompatibility between i386 and x86-64
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_status {
    pub /: *mut *mut snd_pcm_state_t state; / stream state,
    pub /: *mut *mut __time_pad pad1; / align to timespec,
    pub /: *mut *mut timespec trigger_tstamp; / time when stream was started/stopped/paused,
    pub /: *mut *mut timespec tstamp; / reference timestamp,
    pub /: *mut *mut snd_pcm_uframes_t appl_ptr; / appl ptr,
    pub /: *mut *mut snd_pcm_uframes_t hw_ptr; / hw ptr,
    pub /: *mut *mut snd_pcm_sframes_t delay; / current delay in frames,
    pub /: *mut *mut snd_pcm_uframes_t avail; / number of frames available,
    pub /: *mut *mut snd_pcm_uframes_t avail_max; / max frames available on hw since last status,
    pub /: *mut *mut snd_pcm_uframes_t overrange; / count of ADC (capture) overrange detections from last status,
    pub /: *mut *mut snd_pcm_state_t suspended_state; / suspended stream state,
    pub /: *mut *mut __u32 audio_tstamp_data; / needed for 64-bit alignment, used for configs/report to/from userspace,
    pub /: *mut *mut timespec audio_tstamp; / sample counter, wall clock, PHC or on-demand sync'ed,
    pub /: *mut *mut timespec driver_tstamp; / useful in case reference system tstamp is reported with delay,
    pub /: *mut *mut __u32 audio_tstamp_accuracy; / in ns units, only valid if indicated in audio_tstamp_data,
    pub /: *mut *mut *mut unsigned char reserved[52-2sizeof(struct timespec)]; / must be filled with zero,
}

//
// For mmap operations, we need the 64-bit layout, both for compat mode,
// and for y2038 compatibility. For 64-bit applications, the two definitions
// are identical, so we keep the traditional version.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __snd_timespec {
    pub tv_sec: __s32,
    pub tv_nsec: __s32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __snd_timespec64 {
    pub tv_sec: __s64,
    pub tv_nsec: __s64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __snd_pcm_mmap_status {
    pub /: *mut *mut snd_pcm_state_t state; / RO: state - SNDRV_PCM_STATE_XXXX,
    pub /: *mut *mut int pad1; / Needed for 64 bit alignment,
    pub /: *mut *mut snd_pcm_uframes_t hw_ptr; / RO: hw ptr (0...boundary-1),
    pub /: *mut *mut __snd_timespec tstamp; / Timestamp,
    pub /: *mut *mut snd_pcm_state_t suspended_state; / RO: suspended stream state,
    pub /: *mut *mut __snd_timespec audio_tstamp; / from sample counter or wall clock,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __snd_pcm_mmap_control {
    pub /: *mut *mut snd_pcm_uframes_t appl_ptr; / RW: appl ptr (0...boundary-1),
    pub /: *mut *mut snd_pcm_uframes_t avail_min; / RW: min available frames for wakeup,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __snd_pcm_sync_ptr {
    pub flags: c_uint,
    pub status: __snd_pcm_mmap_status,
    pub reserved: [c_uchar; 64],
    pub s: },
    pub control: __snd_pcm_mmap_control,
    pub reserved: [c_uchar; 64],
    pub c: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __snd_pcm_mmap_status64 {
    pub /: *mut *mut snd_pcm_state_t state; / RO: state - SNDRV_PCM_STATE_XXXX,
    pub /: *mut *mut __u32 pad1; / Needed for 64 bit alignment,
    pub __pad1: __pad_before_uframe,
    pub /: *mut *mut snd_pcm_uframes_t hw_ptr; / RO: hw ptr (0...boundary-1),
    pub __pad2: __pad_after_uframe,
    pub /: *mut *mut __snd_timespec64 tstamp; / Timestamp,
    pub /: *mut *mut snd_pcm_state_t suspended_state;/ RO: suspended stream state,
    pub /: *mut *mut __u32 pad3; / Needed for 64 bit alignment,
    pub /: *mut *mut __snd_timespec64 audio_tstamp; / sample counter or wall clock,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __snd_pcm_mmap_control64 {
    pub __pad1: __pad_before_uframe,
    pub /: *mut *mut snd_pcm_uframes_t appl_ptr; / RW: appl ptr (0...boundary-1),
    pub binary: __pad_before_uframe __pad2; // This should be __pad_after_uframe, but,
// backwards compatibility constraints prevent a fix.
    pub __pad3: __pad_before_uframe,
    pub /: *mut *mut snd_pcm_uframes_t avail_min; / RW: min available frames for wakeup,
    pub __pad4: __pad_after_uframe,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __snd_pcm_sync_ptr64 {
    pub flags: __u32,
    pub pad1: __u32,
    pub status: __snd_pcm_mmap_status64,
    pub reserved: [c_uchar; 64],
    pub s: },
    pub control: __snd_pcm_mmap_control64,
    pub reserved: [c_uchar; 64],
    pub c: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_xferi {
    pub result: snd_pcm_sframes_t,
    pub buf: *mut void __user,
    pub frames: snd_pcm_uframes_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_xfern {
    pub result: snd_pcm_sframes_t,
    pub bufs: *mut *mut void __user  __user,
    pub frames: snd_pcm_uframes_t,
}

// channel positions
// this follows the alsa-lib mixer channel value + 3
// new definitions
// new definitions for UAC2
pub const SNDRV_CHMAP_POSITION_MASK: c_uint = 0xffff;

//
// MIDI v1.0 interface
//
// Raw MIDI section - /dev/snd/midi??
//

pub const SNDRV_RAWMIDI_INFO_OUTPUT: c_uint = 0x00000001;
pub const SNDRV_RAWMIDI_INFO_INPUT: c_uint = 0x00000002;
pub const SNDRV_RAWMIDI_INFO_DUPLEX: c_uint = 0x00000004;
pub const SNDRV_RAWMIDI_INFO_UMP: c_uint = 0x00000008;
pub const SNDRV_RAWMIDI_INFO_STREAM_INACTIVE: c_uint = 0x00000010;
pub const SNDRV_RAWMIDI_DEVICE_UNKNOWN: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_rawmidi_info {
    pub /: *mut *mut unsigned int device; / RO/WR (control): device number,
    pub /: *mut *mut unsigned int subdevice; / RO/WR (control): subdevice number,
    pub /: *mut *mut int stream; / WR: stream,
    pub /: *mut *mut int card; / R: card number,
    pub /: *mut *mut unsigned int flags; / SNDRV_RAWMIDI_INFO_XXXX,
    pub /: *mut *mut unsigned char id[64]; / ID (user selectable),
    pub /: *mut *mut unsigned char name[80]; / name of device,
    pub /: *mut *mut unsigned char subname[32]; / name of active or selected subdevice,
    pub subdevices_count: c_uint,
    pub subdevices_avail: c_uint,
    pub /: *mut *mut int tied_device; / R: tied rawmidi device (UMP/legacy),
    pub /: *mut *mut unsigned char reserved[60]; / reserved for future use,
}

pub const SNDRV_RAWMIDI_MODE_FRAMING_SHIFT: c_int = 0;

pub const SNDRV_RAWMIDI_MODE_CLOCK_SHIFT: c_int = 3;

pub const SNDRV_RAWMIDI_FRAMING_DATA_LENGTH: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_rawmidi_framing_tstamp {
// For now, frame_type is always 0. Midi 2.0 is expected to add new
// types here. Applications are expected to skip unknown frame types.
//
    pub frame_type: __u8,
    pub /: *mut *mut __u8 length; / number of valid bytes in data field,
    pub reserved: [__u8; 2],
    pub /: *mut *mut __u32 tv_nsec; / nanoseconds,
    pub /: *mut *mut __u64 tv_sec; / seconds,
    pub data: [__u8; SNDRV_RAWMIDI_FRAMING_DATA_LENGTH],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_rawmidi_params {
    pub stream: c_int,
    pub /: *mut *mut size_t buffer_size; / queue size in bytes,
    pub /: *mut *mut size_t avail_min; / minimum avail bytes for wakeup,
    pub /: *mut *mut unsigned int no_active_sensing: 1; / do not send active sensing byte in close(),
    pub /: *mut *mut unsigned int mode; / For input data only, frame incoming data,
    pub /: *mut *mut unsigned char reserved[12]; / reserved for future use,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_rawmidi_status {
    pub stream: c_int,
    pub pad1: __time_pad,
    pub /: *mut *mut timespec tstamp; / Timestamp,
    pub /: *mut *mut size_t avail; / available bytes,
    pub /: *mut *mut size_t xruns; / count of overruns since last status (in bytes),
    pub /: *mut *mut unsigned char reserved[16]; / reserved for future use,
}

// UMP EP info flags
pub const SNDRV_UMP_EP_INFO_STATIC_BLOCKS: c_uint = 0x01;
// UMP EP Protocol / JRTS capability bits
pub const SNDRV_UMP_EP_INFO_PROTO_MIDI_MASK: c_uint = 0x0300;
pub const SNDRV_UMP_EP_INFO_PROTO_MIDI1: c_uint = 0x0100 /* MIDI 1.0 */;
pub const SNDRV_UMP_EP_INFO_PROTO_MIDI2: c_uint = 0x0200 /* MIDI 2.0 */;
pub const SNDRV_UMP_EP_INFO_PROTO_JRTS_MASK: c_uint = 0x0003;
pub const SNDRV_UMP_EP_INFO_PROTO_JRTS_TX: c_uint = 0x0001 /* JRTS Transmit */;
pub const SNDRV_UMP_EP_INFO_PROTO_JRTS_RX: c_uint = 0x0002 /* JRTS Receive */;
// UMP Endpoint information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ump_endpoint_info {
    pub /: *mut *mut int card; / card number,
    pub /: *mut *mut int device; / device number,
    pub /: *mut *mut unsigned int flags; / additional info,
    pub /: *mut *mut unsigned int protocol_caps; / protocol capabilities,
    pub /: *mut *mut unsigned int protocol; / current protocol,
    pub /: *mut *mut unsigned int num_blocks; / # of function blocks,
    pub /: *mut *mut unsigned short version; / UMP major/minor version,
    pub /: *mut *mut unsigned short family_id; / MIDI device family ID,
    pub /: *mut *mut unsigned short model_id; / MIDI family model ID,
    pub /: *mut *mut unsigned int manufacturer_id; / MIDI manufacturer ID,
    pub /: *mut *mut unsigned char sw_revision[4]; / software revision,
    pub padding: c_ushort,
    pub /: *mut *mut unsigned char name[128]; / endpoint name string,
    pub /: *mut *mut unsigned char product_id[128]; / unique product id string,
    pub reserved: [c_uchar; 32],
    pub __packed: },
// UMP direction
pub const SNDRV_UMP_DIR_INPUT: c_uint = 0x01;
pub const SNDRV_UMP_DIR_OUTPUT: c_uint = 0x02;
pub const SNDRV_UMP_DIR_BIDIRECTION: c_uint = 0x03;
// UMP block info flags

// UMP block user-interface hint
pub const SNDRV_UMP_BLOCK_UI_HINT_UNKNOWN: c_uint = 0x00;
pub const SNDRV_UMP_BLOCK_UI_HINT_RECEIVER: c_uint = 0x01;
pub const SNDRV_UMP_BLOCK_UI_HINT_SENDER: c_uint = 0x02;
pub const SNDRV_UMP_BLOCK_UI_HINT_BOTH: c_uint = 0x03;
// UMP groups and blocks
pub const SNDRV_UMP_MAX_GROUPS: c_int = 16;
pub const SNDRV_UMP_MAX_BLOCKS: c_int = 32;
// UMP Block information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ump_block_info {
    pub /: *mut *mut int card; / card number,
    pub /: *mut *mut int device; / device number,
    pub /: *mut *mut unsigned char block_id; / block ID (R/W),
    pub /: *mut *mut unsigned char direction; / UMP direction,
    pub /: *mut *mut unsigned char active; / Activeness,
    pub /: *mut *mut unsigned char first_group; / first group ID,
    pub /: *mut *mut unsigned char num_groups; / number of groups,
    pub /: *mut *mut unsigned char midi_ci_version; / MIDI-CI support version,
    pub /: *mut *mut unsigned char sysex8_streams; / max number of sysex8 streams,
    pub /: *mut *mut unsigned char ui_hint; / user interface hint,
    pub /: *mut *mut unsigned int flags; / various info flags,
    pub /: *mut *mut unsigned char name[128]; / block name string,
    pub reserved: [c_uchar; 32],
    pub __packed: },

// Additional ioctls for UMP rawmidi devices

//
// Timer section - /dev/snd/timer
//

}

// slave timer classes
// global timers (device member)
pub const SNDRV_TIMER_GLOBAL_SYSTEM: c_int = 0;

pub const SNDRV_TIMER_GLOBAL_HPET: c_int = 2;
pub const SNDRV_TIMER_GLOBAL_HRTIMER: c_int = 3;
pub const SNDRV_TIMER_GLOBAL_UDRIVEN: c_int = 4;
// info flags

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_timer_id {
    pub dev_class: c_int,
    pub dev_sclass: c_int,
    pub card: c_int,
    pub device: c_int,
    pub subdevice: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_timer_ginfo {
    pub /: *mut *mut snd_timer_id tid; / requested timer ID,
    pub /: *mut *mut *mut unsigned int flags; / timer flags - SNDRV_TIMER_FLG_,
    pub /: *mut *mut int card; / card number,
    pub /: *mut *mut unsigned char id[64]; / timer identification,
    pub /: *mut *mut unsigned char name[80]; / timer name,
    pub /: *mut *mut unsigned long reserved0; / reserved for future use,
    pub /: *mut *mut unsigned long resolution; / average period resolution in ns,
    pub /: *mut *mut unsigned long resolution_min; / minimal period resolution in ns,
    pub /: *mut *mut unsigned long resolution_max; / maximal period resolution in ns,
    pub /: *mut *mut unsigned int clients; / active timer clients,
    pub reserved: [c_uchar; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_timer_gparams {
    pub /: *mut *mut snd_timer_id tid; / requested timer ID,
    pub /: *mut *mut unsigned long period_num; / requested precise period duration (in seconds) - numerator,
    pub /: *mut *mut unsigned long period_den; / requested precise period duration (in seconds) - denominator,
    pub reserved: [c_uchar; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_timer_gstatus {
    pub /: *mut *mut snd_timer_id tid; / requested timer ID,
    pub /: *mut *mut unsigned long resolution; / current period resolution in ns,
    pub /: *mut *mut unsigned long resolution_num; / precise current period resolution (in seconds) - numerator,
    pub /: *mut *mut unsigned long resolution_den; / precise current period resolution (in seconds) - denominator,
    pub reserved: [c_uchar; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_timer_select {
    pub /: *mut *mut snd_timer_id id; / bind to timer ID,
    pub /: *mut *mut unsigned char reserved[32]; / reserved,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_timer_info {
    pub /: *mut *mut *mut unsigned int flags; / timer flags - SNDRV_TIMER_FLG_,
    pub /: *mut *mut int card; / card number,
    pub /: *mut *mut unsigned char id[64]; / timer identificator,
    pub /: *mut *mut unsigned char name[80]; / timer name,
    pub /: *mut *mut unsigned long reserved0; / reserved for future use,
    pub /: *mut *mut unsigned long resolution; / average period resolution in ns,
    pub /: *mut *mut unsigned char reserved[64]; / reserved,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_timer_params {
    pub /: *mut *mut *mut unsigned int flags; / flags - SNDRV_TIMER_PSFLG_,
    pub /: *mut *mut unsigned int ticks; / requested resolution in ticks,
    pub /: *mut *mut unsigned int queue_size; / total size of queue (32-1024),
    pub /: *mut *mut unsigned int reserved0; / reserved, was: failure locations,
    pub /: *mut *mut *mut unsigned int filter; / event filter (bitmask of SNDRV_TIMER_EVENT_),
    pub /: *mut *mut unsigned char reserved[60]; / reserved,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_timer_status {
    pub /: *mut *mut timespec tstamp; / Timestamp - last update,
    pub /: *mut *mut unsigned int resolution; / current period resolution in ns,
    pub /: *mut *mut unsigned int lost; / counter of master tick lost,
    pub /: *mut *mut unsigned int overrun; / count of read queue overruns,
    pub /: *mut *mut unsigned int queue; / used queue size,
    pub /: *mut *mut unsigned char reserved[64]; / reserved,
}

//
// This structure describes the userspace-driven timer. Such timers are purely virtual,
// and can only be triggered from software (for instance, by userspace application).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_timer_uinfo {
// To pretend being a normal timer, we need to know the resolution in ns.
    pub resolution: __u64,
    pub fd: c_int,
    pub id: c_uint,
    pub reserved: [c_uchar; 16],
}

// The following four ioctls are changed since 1.0.9 due to confliction

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_timer_read {
    pub resolution: c_uint,
    pub ticks: c_uint,
}

// master timer events for slave timer instances
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_timer_tread {
    pub event: c_int,
    pub pad1: __time_pad,
    pub tstamp: timespec,
    pub val: c_uint,
    pub pad2: __time_pad,
}

//
// Section for driver control interface - /dev/snd/control?
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_card_info {
    pub /: *mut *mut int card; / card number,
    pub /: *mut *mut int pad; / reserved for future (was type),
    pub /: *mut *mut unsigned char id[16]; / ID of card (user selectable),
    pub /: *mut *mut unsigned char driver[16]; / Driver name,
    pub /: *mut *mut unsigned char name[32]; / Short name of soundcard,
    pub /: *mut *mut unsigned char longname[80]; / name + info text about soundcard,
    pub /: *mut *mut unsigned char reserved_[16]; / reserved for future (was ID of mixer),
    pub /: *mut *mut unsigned char mixername[80]; / visual mixer identification,
    pub /: *mut *mut unsigned char components[128]; / card components / fine identification, delimited with one space (AC97 etc..),
}

//
// Card components can exceed the fixed 128 bytes in snd_ctl_card_info.
// Use SNDRV_CTL_IOCTL_CARD_BYTES with type SND_CTL_CARD_BTYPE_COMPONENTS
// to retrieve the full string.
//
// Type values for struct snd_ctl_card_bytes::type
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_card_bytes {
    pub /: *mut *mut *mut __u32 type; / SND_CTL_CARD_BTYPE_,
    pub /: *mut *mut __u32 data_allocated; / size of @data buffer in bytes,
    pub /: *mut *mut __u32 data_len; / in/out: actual data length in bytes,
    pub /: *mut *mut __u32 reserved; / explicit pad,
    pub /: *mut *mut __u64 data; / user buffer (pointer stored as __u64),
}

pub type snd_ctl_elem_type_t = c_int;

pub type snd_ctl_elem_iface_t = c_int;

// (1 << 3) is unused.

// bits 30 and 31 are obsoleted (for indirect access)
// for further details see the ACPI and PCI power management specification
pub const SNDRV_CTL_POWER_D0: c_uint = 0x0000	/* full On */;
pub const SNDRV_CTL_POWER_D1: c_uint = 0x0100	/* partial On */;
pub const SNDRV_CTL_POWER_D2: c_uint = 0x0200	/* partial On */;
pub const SNDRV_CTL_POWER_D3: c_uint = 0x0300	/* Off */;

pub const SNDRV_CTL_ELEM_ID_NAME_MAXLEN: c_int = 44;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_id {
    pub /: *mut *mut unsigned int numid; / numeric identifier, zero = invalid,
    pub /: *mut *mut snd_ctl_elem_iface_t iface; / interface identifier,
    pub /: *mut *mut unsigned int device; / device/client number,
    pub /: *mut *mut unsigned int subdevice; / subdevice (substream) number,
    pub /: *mut *mut unsigned char name[SNDRV_CTL_ELEM_ID_NAME_MAXLEN]; / ASCII name of item,
    pub /: *mut *mut unsigned int index; / index of item,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_list {
    pub /: *mut *mut unsigned int offset; / W: first element ID to get,
    pub /: *mut *mut unsigned int space; / W: count of element IDs to get,
    pub /: *mut *mut unsigned int used; / R: count of element IDs set,
    pub /: *mut *mut unsigned int count; / R: count of all elements,
    pub /: *mut *mut *mut snd_ctl_elem_id __user pids; / R: IDs,
    pub reserved: [c_uchar; 50],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_info {
    pub /: *mut *mut snd_ctl_elem_id id; / W: element ID,
    pub /: *mut *mut *mut snd_ctl_elem_type_t type; / R: value type - SNDRV_CTL_ELEM_TYPE_,
    pub /: *mut *mut *mut unsigned int access; / R: value access (bitmask) - SNDRV_CTL_ELEM_ACCESS_,
    pub /: *mut *mut unsigned int count; / count of values,
    pub /: *mut *mut __kernel_pid_t owner; / owner's PID of this control,
    pub /: *mut *mut long min; / R: minimum value,
    pub /: *mut *mut long max; / R: maximum value,
    pub /: *mut *mut long step; / R: step (0 variable),
    pub integer: },
    pub /: *mut *mut long long min; / R: minimum value,
    pub /: *mut *mut long long max; / R: maximum value,
    pub /: *mut *mut long long step; / R: step (0 variable),
    pub integer64: },
    pub /: *mut *mut unsigned int items; / R: number of items,
    pub /: *mut *mut unsigned int item; / W: item number,
    pub /: *mut *mut char name[64]; / R: value name,
    pub /: *mut *mut __u64 names_ptr; / W: names list (ELEM_ADD only),
    pub names_length: c_uint,
    pub enumerated: },
    pub reserved: [c_uchar; 128],
    pub value: },
    pub reserved: [c_uchar; 64],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value {
    pub /: *mut *mut snd_ctl_elem_id id; / W: element ID,
    pub /: *mut *mut unsigned int indirect: 1; / W: indirect access - obsoleted,
    pub value: [c_long; 128],
    pub /: *mut *mut *mut long value_ptr; / obsoleted,
    pub integer: },
    pub value: [c_longlong; 64],
    pub /: *mut *mut *mut long long value_ptr; / obsoleted,
    pub integer64: },
    pub item: [c_uint; 128],
    pub /: *mut *mut *mut unsigned int item_ptr; / obsoleted,
    pub enumerated: },
    pub data: [c_uchar; 512],
    pub /: *mut *mut *mut unsigned char data_ptr; / obsoleted,
    pub bytes: },
    pub iec958: snd_aes_iec958,
    pub /: *mut *mut } value; / RO,
    pub reserved: [c_uchar; 128],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_tlv {
    pub /: *mut *mut unsigned int numid; / control element numeric identification,
    pub /: *mut *mut unsigned int length; / in bytes aligned to 4,
    pub /: *mut *mut unsigned int tlv[]; / first TLV,
}

//
// Read interface.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sndrv_ctl_event_type {
    SNDRV_CTL_EVENT_ELEM = 0,
    SNDRV_CTL_EVENT_LAST = SNDRV_CTL_EVENT_ELEM,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_event {
    pub /: *mut *mut *mut int type; / event type - SNDRV_CTL_EVENT_,
    pub mask: c_uint,
    pub id: snd_ctl_elem_id,
    pub elem: },
    pub data8: [c_uchar; 60],
    pub data: },
}

//
// Control names
//

