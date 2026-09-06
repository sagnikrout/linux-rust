//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/pcm.h
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
// Digital Audio (PCM) abstract layer
// Copyright (c) by Jaroslav Kysela <perex@perex.cz>
// Abramo Bagnara <abramo@alsa-project.org>
//

//
// Hardware (lowlevel) section
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_hardware {
    pub /: *mut *mut *mut unsigned int info; / SNDRV_PCM_INFO_,
    pub /: *mut *mut *mut u64 formats; / SNDRV_PCM_FMTBIT_,
    pub /: *mut *mut *mut u32 subformats; / for S32_LE, SNDRV_PCM_SUBFMTBIT_,
    pub /: *mut *mut *mut unsigned int rates; / SNDRV_PCM_RATE_,
    pub /: *mut *mut unsigned int rate_min; / min rate,
    pub /: *mut *mut unsigned int rate_max; / max rate,
    pub /: *mut *mut unsigned int channels_min; / min channels,
    pub /: *mut *mut unsigned int channels_max; / max channels,
    pub /: *mut *mut size_t buffer_bytes_max; / max buffer size,
    pub /: *mut *mut size_t period_bytes_min; / min period size,
    pub /: *mut *mut size_t period_bytes_max; / max period size,
    pub /: *mut *mut unsigned int periods_min; / min # of periods,
    pub /: *mut *mut unsigned int periods_max; / max # of periods,
    pub /: *mut *mut size_t fifo_size; / fifo size in bytes,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_ops {
    pub substream): *mut *mut int (open)(struct snd_pcm_substream,
    pub substream): *mut *mut int (close)(struct snd_pcm_substream,
    pub arg): *mut unsigned int cmd, void,
    pub params): *mut snd_pcm_hw_params,
    pub substream): *mut *mut int (hw_free)(struct snd_pcm_substream,
    pub substream): *mut *mut int (prepare)(struct snd_pcm_substream,
    pub cmd): *mut *mut *mut int (trigger)(struct snd_pcm_substream substream, int,
    pub substream): *mut *mut int (sync_stop)(struct snd_pcm_substream,
    pub substream): *mut *mut snd_pcm_uframes_t (pointer)(struct snd_pcm_substream,
    pub audio_tstamp_report): *mut snd_pcm_audio_tstamp_report,
    pub bytes): unsigned long pos, unsigned long,
    pub bytes): *mut *mut unsigned long pos, struct iov_iter iter, unsigned long,
    pub offset): c_ulong,
    pub vma): *mut *mut *mut int (mmap)(struct snd_pcm_substream substream, struct vm_area_struct,
    pub substream): *mut *mut int (ack)(struct snd_pcm_substream,
}

//

pub const SNDRV_PCM_DEVICES: c_int = 8;

pub const SNDRV_PCM_IOCTL1_RESET: c_int = 0;
// 1 is absent slot.
pub const SNDRV_PCM_IOCTL1_CHANNEL_INFO: c_int = 2;
// 3 is absent slot.
pub const SNDRV_PCM_IOCTL1_FIFO_SIZE: c_int = 4;
pub const SNDRV_PCM_IOCTL1_SYNC_ID: c_int = 5;
pub const SNDRV_PCM_TRIGGER_STOP: c_int = 0;
pub const SNDRV_PCM_TRIGGER_START: c_int = 1;
pub const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 2;
pub const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 3;
pub const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 4;
pub const SNDRV_PCM_TRIGGER_RESUME: c_int = 5;
pub const SNDRV_PCM_TRIGGER_DRAIN: c_int = 6;

// If you change this don't forget to change rates[] table in pcm_native.c

// extended rates since 6.12

// For S32/U32 formats, 'msbits' hardware parameter is often used to deliver information about the
// available bit count in most significant bit. It's for the case of so-called 'left-justified' or
// `right-padding` sample which has less width than 32 bit.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_file {
    pub substream: *mut snd_pcm_substream,
    pub no_compat_mmap: c_int,
    pub /: *mut *mut unsigned int user_pversion; / supported protocol version,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_hw_rule {
    pub cond: c_uint,
    pub var: c_int,
    pub deps: [c_int; 5],
    pub func: snd_pcm_hw_rule_func_t,
    pub private: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_hw_constraints {
    pub 1]: SNDRV_PCM_HW_PARAM_FIRST_MASK +,
    pub 1]: SNDRV_PCM_HW_PARAM_FIRST_INTERVAL +,
    pub rules_num: c_uint,
    pub rules_all: c_uint,
    pub rules: *mut snd_pcm_hw_rule,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ratnum {
    pub num: c_uint,
    pub den_step: unsigned int den_min, den_max,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ratden {
    pub num_step: unsigned int num_min, num_max,,
    pub den: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_hw_constraint_ratnums {
    pub nrats: c_int,
    pub rats: *const snd_ratnum,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_hw_constraint_ratdens {
    pub nrats: c_int,
    pub rats: *const snd_ratden,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_hw_constraint_list {
    pub list: *const c_uint,
    pub count: c_uint,
    pub mask: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_hw_constraint_ranges {
    pub count: c_uint,
    pub ranges: *const snd_interval,
    pub mask: c_uint,
}

//
// userspace-provided audio timestamp config to kernel,
// structure is for internal use only and filled with dedicated unpack routine
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_audio_tstamp_config {
// 5 of max 16 bits used
    pub type_requested:4: u32,
    pub /: *mut *mut u32 report_delay:1; / add total delay to A/D or D/A,
}

//
// kernel-provided audio timestamp report to user-space
// structure is for internal use only and read by dedicated pack routine
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_audio_tstamp_report {
// 6 of max 16 bits used for bit-fields
// for backwards compatibility
    pub valid:1: u32,
// actual type if hardware could not support requested timestamp
    pub actual_type:4: u32,
// accuracy represented in ns units
    pub /: *mut *mut u32 accuracy_report:1; / 0 if accuracy unknown, 1 if accuracy field is valid,
    pub /: *mut *mut u32 accuracy; / up to 4.29s, will be packed in separate field,
}

// data &= 0xffff; /* zero-clear MSBs
// data |= (tmp << 16);
// accuracy = report->accuracy;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_runtime {
// -- Status --
    pub /: *mut *mut snd_pcm_state_t state; / stream state,
    pub /: *mut *mut snd_pcm_state_t suspended_state; / suspended stream state,
    pub trigger_master: *mut snd_pcm_substream,
    pub /: *mut *mut timespec64 trigger_tstamp; / trigger timestamp,
    pub /: *mut *mut bool trigger_tstamp_latched; / trigger timestamp latched in low-level driver/hardware,
    pub overrange: c_int,
    pub avail_max: snd_pcm_uframes_t,
    pub /: *mut *mut snd_pcm_uframes_t hw_ptr_base; / Position at buffer restart,
    pub /: *mut *mut snd_pcm_uframes_t hw_ptr_interrupt; / Position at interrupt time,
    pub /: *mut *mut unsigned long hw_ptr_jiffies; / Time when hw_ptr is updated,
    pub /: *mut *mut unsigned long hw_ptr_buffer_jiffies; / buffer time in jiffies,
    pub /: *mut *mut snd_pcm_sframes_t delay; / extra delay; typically FIFO size,
    pub /: *mut *mut u64 hw_ptr_wrap; / offset for hw_ptr due to boundary wrap-around,
// -- HW params --
    pub /: *mut *mut snd_pcm_access_t access; / access mode,
    pub /: *mut *mut *mut snd_pcm_format_t format; / SNDRV_PCM_FORMAT_,
    pub /: *mut *mut snd_pcm_subformat_t subformat; / subformat,
    pub /: *mut *mut unsigned int rate; / rate in Hz,
    pub /: *mut *mut unsigned int channels; / channels,
    pub /: *mut *mut snd_pcm_uframes_t period_size; / period size,
    pub /: *mut *mut unsigned int periods; / periods,
    pub /: *mut *mut snd_pcm_uframes_t buffer_size; / buffer size,
    pub /: *mut *mut snd_pcm_uframes_t min_align; / Min alignment for the format,
    pub byte_align: usize,
    pub frame_bits: c_uint,
    pub sample_bits: c_uint,
    pub info: c_uint,
    pub rate_num: c_uint,
    pub rate_den: c_uint,
    pub 1: unsigned int no_period_wakeup:,
// -- SW params; see struct snd_pcm_sw_params for comments --
    pub tstamp_mode: c_int,
    pub period_step: c_uint,
    pub start_threshold: snd_pcm_uframes_t,
    pub stop_threshold: snd_pcm_uframes_t,
    pub silence_threshold: snd_pcm_uframes_t,
    pub silence_size: snd_pcm_uframes_t,
    pub boundary: snd_pcm_uframes_t,
// internal data of auto-silencer
    pub /: *mut *mut snd_pcm_uframes_t silence_start; / starting pointer to silence area,
    pub /: *mut *mut snd_pcm_uframes_t silence_filled; / already filled part of silence area,
    pub /: *mut *mut bool std_sync_id; / hardware synchronization - standard per card ID,
// -- mmap --
    pub status: *mut snd_pcm_mmap_status,
    pub control: *mut snd_pcm_mmap_control,
// -- locking / scheduling --
    pub /: *mut *mut snd_pcm_uframes_t twake; / do transfer (!poll) wakeup if non-zero,
    pub /: *mut *mut wait_queue_head_t sleep; / poll sleep,
    pub /: *mut *mut wait_queue_head_t tsleep; / transfer sleep,
    pub fasync: *mut snd_fasync,
    pub /: *mut *mut bool stop_operating; / sync_stop will be called,
    pub /: *mut *mut mutex buffer_mutex; / protect for buffer changes,
    pub /: *mut *mut atomic_t buffer_accessing; / >0: in r/w operation, <0: blocked,
// -- private section --
    pub private_data: *mut c_void,
    pub runtime): *mut *mut void (private_free)(struct snd_pcm_runtime,
// -- hardware description --
    pub hw: snd_pcm_hardware,
    pub hw_constraints: snd_pcm_hw_constraints,
// -- timer --
    pub /: *mut *mut unsigned int timer_resolution; / timer resolution,
    pub /: *mut *mut int tstamp_type; / timestamp type,
// -- DMA --
    pub /: *mut *mut *mut unsigned char dma_area; / DMA area,
    pub /: *mut *mut dma_addr_t dma_addr; / physical bus address (not accessible from main CPU),
    pub /: *mut *mut size_t dma_bytes; / size of DMA area,
    pub /: *mut *mut *mut snd_dma_buffer dma_buffer_p; / allocated buffer,
    pub /: *mut *mut unsigned int buffer_changed:1; / buffer allocation changed; set only in managed mode,
// -- audio timestamp config --
    pub audio_tstamp_config: snd_pcm_audio_tstamp_config,
    pub audio_tstamp_report: snd_pcm_audio_tstamp_report,
    pub driver_tstamp: timespec64,

// -- OSS things --
    pub oss: snd_pcm_oss_runtime,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_group {
    pub lock: spinlock_t,
    pub mutex: mutex,
    pub substreams: list_head,
    pub refs: refcount_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_substream {
    pub pcm: *mut snd_pcm,
    pub pstr: *mut snd_pcm_str,
    pub /: *mut *mut *mut void private_data; / copied from pcm->private_data,
    pub number: c_int,
    pub /: *mut *mut char name[32]; / substream name,
    pub /: *mut *mut int stream; / stream (direction),
    pub /: *mut *mut pm_qos_request latency_pm_qos_req; / pm_qos request,
    pub /: *mut *mut size_t buffer_bytes_max; / limit ring buffer size,
    pub dma_buffer: snd_dma_buffer,
    pub dma_max: usize,
// -- hardware operations --
    pub ops: *const snd_pcm_ops,
// -- runtime information --
    pub runtime: *mut snd_pcm_runtime,
// -- timer section --
    pub /: *mut *mut *mut snd_timer timer; / timer,
    pub /: *mut *mut unsigned timer_running: 1; / time is running,
    pub /: *mut *mut long wait_time; / time in ms for R/W to wait for avail,
// -- next substream --
    pub next: *mut snd_pcm_substream,
// -- linked substreams --
    pub /: *mut *mut list_head link_list; / linked list member,
    pub /: *mut *mut snd_pcm_group self_group; / fake group for non linked substream (with substream lock inside),
    pub /: *mut *mut *mut snd_pcm_group group; / pointer to current group,
// -- assigned files --
    pub ref_count: c_int,
    pub mmap_count: core::sync::atomic::AtomicI32,
    pub f_flags: c_uint,
    pub ): *mut *mut void (pcm_release)(struct snd_pcm_substream,
    pub pid: *mut pid,

// -- OSS things --
    pub oss: snd_pcm_oss_substream,

    pub proc_root: *mut snd_info_entry,

// misc flags
    pub 1: unsigned int hw_opened:,
    pub managed_buffer_alloc:1: c_uint,

    pub /: *mut *mut unsigned int xrun_counter; / number of times xrun happens,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_str {
    pub /: *mut *mut int stream; / stream (direction),
    pub pcm: *mut snd_pcm,
// -- substreams --
    pub substream_count: c_uint,
    pub substream_opened: c_uint,
    pub substream: *mut snd_pcm_substream,

// -- OSS things --
    pub oss: snd_pcm_oss_stream,

    pub proc_root: *mut snd_info_entry,

    pub /: *mut *mut unsigned int xrun_debug; / 0 = disabled, 1 = verbose, 2 = stacktrace,

    pub /: *mut *mut *mut snd_kcontrol chmap_kctl; / channel-mapping controls,
    pub dev: *mut device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm {
    pub card: *mut snd_card,
    pub list: list_head,
    pub /: *mut *mut int device; / device number,
    pub info_flags: c_uint,
    pub dev_class: c_ushort,
    pub dev_subclass: c_ushort,
    pub id: [c_char; 64],
    pub name: [c_char; 80],
    pub streams: [snd_pcm_str; 2],
    pub open_mutex: mutex,
    pub open_wait: wait_queue_head_t,
    pub private_data: *mut c_void,
    pub pcm): *mut *mut void (private_free) (struct snd_pcm,
    pub /: *mut *mut bool internal; / pcm is for internal use only,
    pub /: *mut *mut bool nonatomic; / whole PCM operations are in non-atomic context,
    pub /: *mut *mut bool no_device_suspend; / don't invoke device PM suspend,

    pub oss: snd_pcm_oss,

}

//
// Registering
//
extern "C" {
    pub fn snd_pcm_new_stream(pcm: *mut snd_pcm, stream: c_int, substream_count: c_int) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_notify {
    pub pcm): *mut *mut *mut int (n_register) (struct snd_pcm,
    pub pcm): *mut *mut *mut int (n_disconnect) (struct snd_pcm,
    pub pcm): *mut *mut *mut int (n_unregister) (struct snd_pcm,
    pub list: list_head,
}

extern "C" {
    pub fn snd_pcm_notify(notify: *mut snd_pcm_notify, nfree: c_int) -> c_int;
}

//
// Native I/O
//
extern "C" {
    pub fn snd_pcm_info(substream: *mut snd_pcm_substream, info: *mut snd_pcm_info) -> c_int;
}
extern "C" {
    pub fn snd_pcm_start(substream: *mut snd_pcm_substream) -> c_int;
}
extern "C" {
    pub fn snd_pcm_stop(substream: *mut snd_pcm_substream, status: snd_pcm_state_t) -> c_int;
}
extern "C" {
    pub fn snd_pcm_drain_done(substream: *mut snd_pcm_substream) -> c_int;
}
extern "C" {
    pub fn snd_pcm_stop_xrun(substream: *mut snd_pcm_substream) -> c_int;
}

extern "C" {
    pub fn snd_pcm_suspend_all(pcm: *mut snd_pcm) -> c_int;
}

extern "C" {
    pub fn snd_pcm_kernel_ioctl(substream: *mut snd_pcm_substream, cmd: c_uint, arg: *mut c_void) -> c_int;
}
extern "C" {
    pub fn snd_pcm_release_substream(substream: *mut snd_pcm_substream);
}
extern "C" {
    pub fn snd_pcm_detach_substream(substream: *mut snd_pcm_substream);
}
extern "C" {
    pub fn snd_pcm_mmap_data(substream: *mut snd_pcm_substream, file: *mut file, area: *mut vm_area_struct) -> c_int;
}

// buf = 0;

//
// PCM library
//
// snd_pcm_stream_linked - Check whether the substream is linked with others
// @substream: substream to check
//
// Return: true if the given substream is being linked with others
//
extern "C" {
    pub fn snd_pcm_stream_lock(substream: *mut snd_pcm_substream);
}
extern "C" {
    pub fn snd_pcm_stream_unlock(substream: *mut snd_pcm_substream);
}
extern "C" {
    pub fn snd_pcm_stream_lock_irq(substream: *mut snd_pcm_substream);
}
extern "C" {
    pub fn snd_pcm_stream_unlock_irq(substream: *mut snd_pcm_substream);
}
extern "C" {
    pub fn _snd_pcm_stream_lock_irqsave(substream: *mut snd_pcm_substream) -> c_ulong;
}
extern "C" {
    pub fn _snd_pcm_stream_lock_irqsave_nested(substream: *mut snd_pcm_substream) -> c_ulong;
}
//
// snd_pcm_stream_lock_irqsave - Lock the PCM stream
// @substream: PCM substream
// @flags: irq flags
//
// This locks the PCM stream like snd_pcm_stream_lock() but with the local
// IRQ (only when nonatomic is false).  In nonatomic case, this is identical
// as snd_pcm_stream_lock().
//

//
// snd_pcm_stream_lock_irqsave_nested - Single-nested PCM stream locking
// @substream: PCM substream
// @flags: irq flags
//
// This locks the PCM stream like snd_pcm_stream_lock_irqsave() but with
// the single-depth lockdep subclass.
//

// definitions for guard(); use like guard(pcm_stream_lock)
//
// snd_pcm_group_for_each_entry - iterate over the linked substreams
// @s: the iterator
// @substream: the substream
//
// Iterate over the all linked substreams to the given @substream.
// When @substream isn't linked with any others, this gives returns @substream
// itself once.
//

//
// snd_pcm_running - Check whether the substream is in a running state
// @substream: substream to check
//
// Return: true if the given substream is in the state RUNNING, or in the
// state DRAINING for playback.
//
// __snd_pcm_set_state - Change the current PCM state
// @runtime: PCM runtime to set
// @state: the current state to set
//
// Call within the stream lock
//
extern "C" {
    pub fn snd_pcm_get_state(substream: *mut snd_pcm_substream) -> snd_pcm_state_t;
}
//
// bytes_to_samples - Unit conversion of the size from bytes to samples
// @runtime: PCM runtime instance
// @size: size in bytes
//
// Return: the size in samples
//
// bytes_to_frames - Unit conversion of the size from bytes to frames
// @runtime: PCM runtime instance
// @size: size in bytes
//
// Return: the size in frames
//
// samples_to_bytes - Unit conversion of the size from samples to bytes
// @runtime: PCM runtime instance
// @size: size in samples
//
// Return: the byte size
//
// frames_to_bytes - Unit conversion of the size from frames to bytes
// @runtime: PCM runtime instance
// @size: size in frames
//
// Return: the byte size
//
// frame_aligned - Check whether the byte size is aligned to frames
// @runtime: PCM runtime instance
// @bytes: size in bytes
//
// Return: true if aligned, or false if not
//
// snd_pcm_lib_buffer_bytes - Get the buffer size of the current PCM in bytes
// @substream: PCM substream
//
// Return: buffer byte size
//
extern "C" {
    pub fn frames_to_bytes(_arg: runtime, _arg: runtime->buffer_size) -> return;
}
//
// snd_pcm_lib_period_bytes - Get the period size of the current PCM in bytes
// @substream: PCM substream
//
// Return: period byte size
//
extern "C" {
    pub fn frames_to_bytes(_arg: runtime, _arg: runtime->period_size) -> return;
}
//
// snd_pcm_playback_avail - Get the available (writable) space for playback
// @runtime: PCM runtime instance
//
// Result is between 0 ... (boundary - 1)
//
// Return: available frame size
//
// snd_pcm_capture_avail - Get the available (readable) space for capture
// @runtime: PCM runtime instance
//
// Result is between 0 ... (boundary - 1)
//
// Return: available frame size
//
// snd_pcm_playback_hw_avail - Get the queued space for playback
// @runtime: PCM runtime instance
//
// Return: available frame size
//
// snd_pcm_capture_hw_avail - Get the free space for capture
// @runtime: PCM runtime instance
//
// Return: available frame size
//
// snd_pcm_playback_ready - check whether the playback buffer is available
// @substream: the pcm substream instance
//
// Checks whether enough free space is available on the playback buffer.
//
// Return: Non-zero if available, or zero if not.
//
// snd_pcm_capture_ready - check whether the capture buffer is available
// @substream: the pcm substream instance
//
// Checks whether enough capture data is available on the capture buffer.
//
// Return: Non-zero if available, or zero if not.
//
// snd_pcm_playback_data - check whether any data exists on the playback buffer
// @substream: the pcm substream instance
//
// Checks whether any data exists on the playback buffer.
//
// Return: Non-zero if any data exists, or zero if not. If stop_threshold
// is bigger or equal to boundary, then this function returns always non-zero.
//
// snd_pcm_playback_empty - check whether the playback buffer is empty
// @substream: the pcm substream instance
//
// Checks whether the playback buffer is empty.
//
// Return: Non-zero if empty, or zero if not.
//
// snd_pcm_capture_empty - check whether the capture buffer is empty
// @substream: the pcm substream instance
//
// Checks whether the capture buffer is empty.
//
// Return: Non-zero if empty, or zero if not.
//
// snd_pcm_trigger_done - Mark the master substream
// @substream: the pcm substream instance
// @master: the linked master substream
//
// When multiple substreams of the same card are linked and the hardware
// supports the single-shot operation, the driver calls this in the loop
// in snd_pcm_group_for_each_entry() for marking the substream as "done".
// Then most of trigger operations are performed only to the given master
// substream.
//
// The trigger_master mark is cleared at timestamp updates at the end
// of trigger operations.
//
// params_channels - Get the number of channels from the hw params
// @p: hw params
//
// Return: the number of channels
//
// params_rate - Get the sample rate from the hw params
// @p: hw params
//
// Return: the sample rate
//
// params_period_size - Get the period size (in frames) from the hw params
// @p: hw params
//
// Return: the period size in frames
//
// params_periods - Get the number of periods from the hw params
// @p: hw params
//
// Return: the number of periods
//
// params_buffer_size - Get the buffer size (in frames) from the hw params
// @p: hw params
//
// Return: the buffer size in frames
//
// params_buffer_bytes - Get the buffer size (in bytes) from the hw params
// @p: hw params
//
// Return: the buffer size in bytes
//
extern "C" {
    pub fn snd_interval_refine(i: *mut snd_interval, v: *const snd_interval) -> c_int;
}
extern "C" {
    pub fn _snd_pcm_hw_params_any(params: *mut snd_pcm_hw_params);
}
extern "C" {
    pub fn _snd_pcm_hw_param_setempty(params: *mut snd_pcm_hw_params, var: snd_pcm_hw_param_t);
}
extern "C" {
    pub fn snd_pcm_hw_refine(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params) -> c_int;
}
extern "C" {
    pub fn snd_pcm_hw_constraint_integer(runtime: *mut snd_pcm_runtime, var: snd_pcm_hw_param_t) -> c_int;
}
//
// snd_pcm_hw_constraint_single() - Constrain parameter to a single value
// @runtime: PCM runtime instance
// @var: The hw_params variable to constrain
// @val: The value to constrain to
//
// Return: Positive if the value is changed, zero if it's not changed, or a
// negative error code.
//
extern "C" {
    pub fn snd_pcm_hw_constraint_minmax(_arg: runtime, _arg: var, _arg: val, _arg: val) -> return;
}
extern "C" {
    pub fn snd_pcm_format_signed(format: snd_pcm_format_t) -> c_int;
}
extern "C" {
    pub fn snd_pcm_format_unsigned(format: snd_pcm_format_t) -> c_int;
}
extern "C" {
    pub fn snd_pcm_format_linear(format: snd_pcm_format_t) -> c_int;
}
extern "C" {
    pub fn snd_pcm_format_little_endian(format: snd_pcm_format_t) -> c_int;
}
extern "C" {
    pub fn snd_pcm_format_big_endian(format: snd_pcm_format_t) -> c_int;
}

//
// snd_pcm_format_cpu_endian - Check the PCM format is CPU-endian
// @format: the format to check
//
// Return: 1 if the given PCM format is CPU-endian, 0 if
// opposite, or a negative error code if endian not specified.
//
extern "C" {
    pub fn snd_pcm_format_cpu_endian(format: snd_pcm_format_t) -> c_int;
}

extern "C" {
    pub fn snd_pcm_format_size(format: snd_pcm_format_t, samples: usize) -> isize;
}
extern "C" {
    pub fn snd_pcm_format_set_silence(format: snd_pcm_format_t, buf: *mut c_void, frames: c_uint) -> c_int;
}
//
// snd_pcm_set_sync - set the PCM sync id
// @substream: the pcm substream
//
// Use the default PCM sync identifier for the specific card.
//
extern "C" {
    pub fn snd_pcm_period_elapsed_under_stream_lock(substream: *mut snd_pcm_substream);
}
extern "C" {
    pub fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
}
extern "C" {
    pub fn __snd_pcm_lib_xfer(_arg: substream, )buf: *mut (void , _arg: true, _arg: frames, _arg: false) -> return;
}
extern "C" {
    pub fn __snd_pcm_lib_xfer(_arg: substream, )buf: *mut (void , _arg: true, _arg: frames, _arg: false) -> return;
}
extern "C" {
    pub fn __snd_pcm_lib_xfer(_arg: substream, )bufs: *mut (void, _arg: false, _arg: frames, _arg: false) -> return;
}
extern "C" {
    pub fn __snd_pcm_lib_xfer(_arg: substream, )bufs: *mut (void, _arg: false, _arg: frames, _arg: false) -> return;
}
extern "C" {
    pub fn __snd_pcm_lib_xfer(_arg: substream, )buf: *mut (void, _arg: true, _arg: frames, _arg: true) -> return;
}
extern "C" {
    pub fn __snd_pcm_lib_xfer(_arg: substream, _arg: buf, _arg: true, _arg: frames, _arg: true) -> return;
}
extern "C" {
    pub fn __snd_pcm_lib_xfer(_arg: substream, _arg: bufs, _arg: false, _arg: frames, _arg: true) -> return;
}
extern "C" {
    pub fn __snd_pcm_lib_xfer(_arg: substream, _arg: bufs, _arg: false, _arg: frames, _arg: true) -> return;
}
extern "C" {
    pub fn snd_pcm_hw_limit_rates(hw: *mut snd_pcm_hardware) -> c_int;
}
extern "C" {
    pub fn snd_pcm_hw_limit_rates(_arg: &runtime->hw) -> return;
}
extern "C" {
    pub fn snd_pcm_rate_to_rate_bit(rate: c_uint) -> c_uint;
}
extern "C" {
    pub fn snd_pcm_rate_bit_to_rate(rate_bit: c_uint) -> c_uint;
}
//
// snd_pcm_set_runtime_buffer - Set the PCM runtime buffer
// @substream: PCM substream to set
// @bufp: the buffer information, NULL to clear
//
// Copy the buffer information to runtime->dma_buffer when @bufp is non-NULL.
// Otherwise it clears the current buffer information.
//
// snd_pcm_gettime - Fill the timespec64 depending on the timestamp mode
// @runtime: PCM runtime instance
// @tv: timespec64 to fill
//
// Memory
//
extern "C" {
    pub fn snd_pcm_lib_preallocate_free(substream: *mut snd_pcm_substream);
}
extern "C" {
    pub fn snd_pcm_lib_preallocate_free_for_all(pcm: *mut snd_pcm);
}
extern "C" {
    pub fn snd_pcm_lib_malloc_pages(substream: *mut snd_pcm_substream, size: usize) -> c_int;
}
extern "C" {
    pub fn snd_pcm_lib_free_pages(substream: *mut snd_pcm_substream) -> c_int;
}
//
// snd_pcm_set_fixed_buffer - Preallocate and set up the fixed size PCM buffer
// @substream: the pcm substream instance
// @type: DMA type (SNDRV_DMA_TYPE_*)
// @data: DMA type dependent data
// @size: the requested pre-allocation size in bytes
//
// This is a variant of snd_pcm_set_managed_buffer(), but this pre-allocates
// only the given sized buffer and doesn't allow re-allocation nor dynamic
// allocation of a larger buffer unlike the standard one.
// The function may return -ENOMEM error, hence the caller must check it.
//
// Return: zero if successful, or a negative error code
//
extern "C" {
    pub fn snd_pcm_set_managed_buffer(_arg: substream, _arg: type, _arg: data, _arg: size, _arg: 0) -> return;
}
//
// snd_pcm_set_fixed_buffer_all - Preallocate and set up the fixed size PCM buffer
// @pcm: the pcm instance
// @type: DMA type (SNDRV_DMA_TYPE_*)
// @data: DMA type dependent data
// @size: the requested pre-allocation size in bytes
//
// Apply the set up of the fixed buffer via snd_pcm_set_fixed_buffer() for
// all substream.  If any of allocation fails, it returns -ENOMEM, hence the
// caller must check the return value.
//
// Return: zero if successful, or a negative error code
//
extern "C" {
    pub fn snd_pcm_set_managed_buffer_all(_arg: pcm, _arg: type, _arg: data, _arg: size, _arg: 0) -> return;
}

//
// snd_pcm_sgbuf_get_addr - Get the DMA address at the corresponding offset
// @substream: PCM substream
// @ofs: byte offset
//
// Return: DMA address
//
extern "C" {
    pub fn snd_sgbuf_get_addr(_arg: snd_pcm_get_dma_buf(substream), _arg: ofs) -> return;
}
//
// snd_pcm_sgbuf_get_chunk_size - Compute the max size that fits within the
// contig. page from the given size
// @substream: PCM substream
// @ofs: byte offset
// @size: byte size to examine
//
// Return: chunk size
//
extern "C" {
    pub fn snd_sgbuf_get_chunk_size(_arg: snd_pcm_get_dma_buf(substream), _arg: ofs, _arg: size) -> return;
}
// mmap for io-memory area

extern "C" {
    pub fn snd_pcm_lib_mmap_iomem(substream: *mut snd_pcm_substream, area: *mut vm_area_struct) -> c_int;
}

pub const SNDRV_PCM_INFO_MMAP_IOMEM: c_int = 0;

extern "C" {
    pub fn snd_pcm_runtime_buffer_set_silence(runtime: *mut snd_pcm_runtime) -> c_int;
}
//
// snd_pcm_limit_isa_dma_size - Get the max size fitting with ISA DMA transfer
// @dma: DMA number
// @max: pointer to store the max size
//
// max = dma < 4 ? 64 * 1024 : 128 * 1024;
//
// Misc
//

//
// snd_pcm_direction_name - Get a string naming the direction of a stream
// @direction: Stream's direction, one of SNDRV_PCM_STREAM_XXX
//
// Returns a string naming the direction of the stream.
//
// snd_pcm_stream_str - Get a string naming the direction of a stream
// @substream: the pcm substream instance
//
// Return: A string naming the direction of the stream.
//
extern "C" {
    pub fn snd_pcm_direction_name(_arg: substream->stream) -> return;
}
//
// PCM channel-mapping control API
//
// array element of channel maps
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_chmap_elem {
    pub channels: c_uchar,
    pub map: [c_uchar; 15],
}

// channel map information; retrieved via snd_kcontrol_chip()
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_chmap {
    pub /: *mut *mut *mut snd_pcm pcm; / assigned PCM instance,
    pub /: *mut *mut int stream; / PLAYBACK or CAPTURE,
    pub kctl: *mut snd_kcontrol,
    pub chmap: *const snd_pcm_chmap_elem,
    pub max_channels: c_uint,
    pub /: *mut *mut unsigned int channel_mask; / optional: active channels bitmask,
    pub /: *mut *mut *mut void private_data; / optional: private data pointer,
}

//
// snd_pcm_chmap_substream - get the PCM substream assigned to the given chmap info
// @info: chmap information
// @idx: the substream number index
//
// Return: the matched PCM substream, or NULL if not found
//
// ALSA-standard channel maps (RL/RR prior to C/LFE)
// Other world's standard channel maps (C/LFE prior to RL/RR)
// bit masks to be passed to snd_pcm_chmap.channel_mask field

//
// pcm_format_to_bits - Strong-typed conversion of pcm_format to bitwise
// @pcm_format: PCM format
//
// Return: 64bit mask corresponding to the given PCM format
//
// pcm_for_each_format - helper to iterate for each format type
// @f: the iterator variable in snd_pcm_format_t type
//

// printk helpers

// helpers for copying between iov_iter and iomem
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_status64 {
    pub /: *mut *mut snd_pcm_state_t state; / stream state,
    pub rsvd: [u8; 4],
    pub /: *mut *mut s64 trigger_tstamp_sec; / time when stream was started/stopped/paused,
    pub trigger_tstamp_nsec: i64,
    pub /: *mut *mut s64 tstamp_sec; / reference timestamp,
    pub tstamp_nsec: i64,
    pub /: *mut *mut snd_pcm_uframes_t appl_ptr; / appl ptr,
    pub /: *mut *mut snd_pcm_uframes_t hw_ptr; / hw ptr,
    pub /: *mut *mut snd_pcm_sframes_t delay; / current delay in frames,
    pub /: *mut *mut snd_pcm_uframes_t avail; / number of frames available,
    pub /: *mut *mut snd_pcm_uframes_t avail_max; / max frames available on hw since last status,
    pub /: *mut *mut snd_pcm_uframes_t overrange; / count of ADC (capture) overrange detections from last status,
    pub /: *mut *mut snd_pcm_state_t suspended_state; / suspended stream state,
    pub /: *mut *mut __u32 audio_tstamp_data; / needed for 64-bit alignment, used for configs/report to/from userspace,
    pub /: *mut *mut s64 audio_tstamp_sec; / sample counter, wall clock, PHC or on-demand sync'ed,
    pub audio_tstamp_nsec: i64,
    pub /: *mut *mut s64 driver_tstamp_sec; / useful in case reference system tstamp is reported with delay,
    pub driver_tstamp_nsec: i64,
    pub /: *mut *mut __u32 audio_tstamp_accuracy; / in ns units, only valid if indicated in audio_tstamp_data,
    pub /: *mut *mut *mut unsigned char reserved[52-4sizeof(s64)]; / must be filled with zero,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_status32 {
    pub /: *mut *mut snd_pcm_state_t state; / stream state,
    pub /: *mut *mut s32 trigger_tstamp_sec; / time when stream was started/stopped/paused,
    pub trigger_tstamp_nsec: i32,
    pub /: *mut *mut s32 tstamp_sec; / reference timestamp,
    pub tstamp_nsec: i32,
    pub /: *mut *mut u32 appl_ptr; / appl ptr,
    pub /: *mut *mut u32 hw_ptr; / hw ptr,
    pub /: *mut *mut s32 delay; / current delay in frames,
    pub /: *mut *mut u32 avail; / number of frames available,
    pub /: *mut *mut u32 avail_max; / max frames available on hw since last status,
    pub /: *mut *mut u32 overrange; / count of ADC (capture) overrange detections from last status,
    pub /: *mut *mut snd_pcm_state_t suspended_state; / suspended stream state,
    pub /: *mut *mut u32 audio_tstamp_data; / needed for 64-bit alignment, used for configs/report to/from userspace,
    pub /: *mut *mut s32 audio_tstamp_sec; / sample counter, wall clock, PHC or on-demand sync'ed,
    pub audio_tstamp_nsec: i32,
    pub /: *mut *mut s32 driver_tstamp_sec; / useful in case reference system tstamp is reported with delay,
    pub driver_tstamp_nsec: i32,
    pub /: *mut *mut u32 audio_tstamp_accuracy; / in ns units, only valid if indicated in audio_tstamp_data,
    pub /: *mut *mut *mut unsigned char reserved[52-4sizeof(s32)]; / must be filled with zero,
}

