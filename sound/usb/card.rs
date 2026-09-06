//! Automatically rewritten from C Header to Rust Module
//! Source: sound/usb/card.h
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
pub const MAX_NR_RATES: c_int = 1024;

pub const MAX_URBS: c_int = 12;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct audioformat {
    pub list: list_head,
    pub /: *mut *mut u64 formats; / ALSA format bits,
    pub /: *mut *mut unsigned int channels; / # channels,
    pub /: *mut *mut unsigned int fmt_type; / USB audio format type (1-3),
    pub /: *mut *mut unsigned int fmt_bits; / number of significant bits,
    pub /: *mut *mut unsigned int fmt_sz; / overall audio sub frame/slot size,
    pub /: *mut *mut unsigned int frame_size; / samples per frame for non-audio,
    pub /: *mut *mut unsigned char iface; / interface number,
    pub /: *mut *mut unsigned char altsetting; / corresponding alternate setting,
    pub /: *mut *mut unsigned char ep_idx; / endpoint array index,
    pub /: *mut *mut unsigned char altset_idx; / array index of alternate setting,
    pub /: *mut *mut unsigned char attributes; / corresponding attributes of cs endpoint,
    pub /: *mut *mut unsigned char endpoint; / endpoint,
    pub /: *mut *mut unsigned char ep_attr; / endpoint attributes,
    pub /: *mut *mut bool implicit_fb; / implicit feedback endpoint,
    pub /: *mut *mut unsigned char sync_ep; / sync endpoint number,
    pub /: *mut *mut unsigned char sync_iface; / sync EP interface,
    pub /: *mut *mut unsigned char sync_altsetting; / sync EP alternate setting,
    pub /: *mut *mut unsigned char sync_ep_idx; / sync EP array index,
    pub /: *mut *mut unsigned char datainterval; / log_2 of data packet interval,
    pub /: *mut *mut unsigned char protocol; / UAC_VERSION_1/2/3,
    pub /: *mut *mut unsigned int maxpacksize; / max. packet size,
    pub /: *mut *mut unsigned int rates; / rate bitmasks,
    pub /: *mut *mut unsigned int rate_min, rate_max; / min/max rates,
    pub /: *mut *mut unsigned int nr_rates; / number of rate table entries,
    pub /: *mut *mut *mut unsigned int rate_table; / rate table,
    pub /: *mut *mut unsigned char clock; / associated clock,
    pub /: *mut *mut *mut snd_pcm_chmap_elem chmap; / (optional) channel map,
    pub /: *mut *mut bool dsd_dop; / add DOP headers in case of DSD samples,
    pub /: *mut *mut bool dsd_bitrev; / reverse the bits of each DSD sample,
    pub /: *mut *mut bool dsd_raw; / altsetting is raw DSD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_urb_ctx {
    pub urb: *mut urb,
    pub /: *mut *mut unsigned int buffer_size; / size of data buffer, if data URB,
    pub subs: *mut snd_usb_substream,
    pub ep: *mut snd_usb_endpoint,
    pub /: *mut *mut int index; / index for urb array,
    pub /: *mut *mut int packets; / number of packets per urb,
    pub /: *mut *mut int queued; / queued data bytes by this urb,
    pub /: *mut *mut int packet_size[MAX_PACKS_HS]; / size of packets for next submission,
    pub ready_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_usb_endpoint {
    pub chip: *mut snd_usb_audio,
    pub iface_ref: *mut snd_usb_iface_ref,
    pub clock_ref: *mut snd_usb_clock_ref,
    pub /: *mut *mut int opened; / open refcount; protect with chip->mutex,
    pub /: *mut *mut atomic_t running; / running status,
    pub /: *mut *mut int ep_num; / the referenced endpoint number,
    pub /: *mut *mut *mut int type; / SND_USB_ENDPOINT_TYPE_,
    pub /: *mut *mut unsigned char iface; / interface number,
    pub /: *mut *mut unsigned char altsetting; / corresponding alternate setting,
    pub /: *mut *mut unsigned char ep_idx; / endpoint array index,
    pub /: *mut *mut atomic_t state; / running state,
    pub in_stream_lock): bool,
    pub urb): *mut urb,
    pub data_subs: *mut snd_usb_substream,
    pub sync_source: *mut snd_usb_endpoint,
    pub sync_sink: *mut snd_usb_endpoint,
    pub urb: [snd_urb_ctx; MAX_URBS],
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_usb_packet_info {
    pub packet_size: [c_int; MAX_PACKS_HS],
    pub packets: c_int,
    pub next_packet: [}; MAX_URBS],
    pub /: *mut *mut unsigned int next_packet_head; / ring buffer offset to read,
    pub /: *mut *mut unsigned int next_packet_queued; / queued items in the ring buffer,
    pub /: *mut *mut list_head ready_playback_urbs; / playback URB FIFO for implicit fb,
    pub /: *mut *mut unsigned int nurbs; / # urbs,
    pub /: *mut *mut unsigned long active_mask; / bitmask of active urbs,
    pub /: *mut *mut unsigned long unlink_mask; / bitmask of unlinked urbs,
    pub /: *mut *mut atomic_t submitted_urbs; / currently submitted urbs,
    pub /: *mut *mut *mut char syncbuf; / sync buffer for all sync URBs,
    pub /: *mut *mut dma_addr_t sync_dma; / DMA address of syncbuf,
    pub /: *mut *mut unsigned int pipe; / the data i/o pipe,
    pub /: *mut *mut unsigned int packsize[2]; / small/large packet sizes in samples,
    pub /: *mut *mut unsigned int sample_rem; / remainder from division fs/pps,
    pub /: *mut *mut unsigned int sample_accum; / sample accumulator,
    pub /: *mut *mut unsigned int pps; / packets per second,
    pub /: *mut *mut unsigned int freqn; / nominal sampling rate in fs/fps in Q16.16 format,
    pub /: *mut *mut unsigned int freqm; / momentary sampling rate in fs/fps in Q16.16 format,
    pub /: *mut *mut int freqshift; / how much to shift the feedback value to get Q16.16,
    pub /: *mut *mut unsigned int freqmax; / maximum sampling rate, used for buffer management,
    pub /: *mut *mut unsigned int phase; / phase accumulator,
    pub /: *mut *mut unsigned int maxpacksize; / max packet size in bytes,
    pub /: *mut *mut unsigned int maxframesize; / max packet size in frames,
    pub /: *mut *mut unsigned int max_urb_frames; / max URB size in frames,
    pub /: *mut *mut unsigned int curpacksize; / current packet size in bytes (for capture),
    pub /: *mut *mut unsigned int curframesize; / current packet size in frames (for capture),
    pub /: *mut *mut unsigned int syncmaxsize; / sync endpoint packet size,
    pub /: *mut *mut unsigned int fill_max:1; / fill max packet size always,
    pub /: *mut *mut unsigned int tenor_fb_quirk:1; / corrupted feedback data,
    pub /: *mut *mut unsigned int datainterval; / log_2 of data packet interval,
    pub /: *mut *mut unsigned int syncinterval; / P for adaptive mode, 0 otherwise,
    pub silence_value: c_uchar,
    pub stride: c_uint,
    pub packets: *mut *mut int skip_packets; / quirks for devices to ignore the first n,
    pub /: *mut *mut bool implicit_fb_sync; / syncs with implicit feedback,
    pub /: *mut *mut bool lowlatency_playback; / low-latency playback mode,
    pub /: *mut *mut bool need_setup; / (re-)need for hw_params?,
    pub /: *mut *mut bool need_prepare; / (re-)need for prepare?,
    pub /: *mut *mut bool fixed_rate; / skip rate setup,
// for hw constraints
    pub cur_audiofmt: *const audioformat,
    pub cur_rate: c_uint,
    pub cur_format: snd_pcm_format_t,
    pub cur_channels: c_uint,
    pub cur_frame_bytes: c_uint,
    pub cur_period_frames: c_uint,
    pub cur_period_bytes: c_uint,
    pub cur_buffer_periods: c_uint,
    pub lock: spinlock_t,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_usb_substream {
    pub stream: *mut snd_usb_stream,
    pub dev: *mut usb_device,
    pub pcm_substream: *mut snd_pcm_substream,
    pub /: *mut *mut int direction; / playback or capture,
    pub /: *mut *mut int endpoint; / assigned endpoint,
    pub /: *const *const *const audioformat cur_audiofmt; / current audioformat pointer (for hw_params callback),
    pub /: *mut *mut *mut snd_usb_power_domain str_pd; / UAC3 Power Domain for streaming path,
    pub /: *mut *mut unsigned int channels_max; / max channels in the all audiofmts,
    pub /: *mut *mut unsigned int txfr_quirk:1; / allow sub-frame alignment,
    pub /: *mut *mut unsigned int tx_length_quirk:1; / add length specifier to transfers,
    pub /: *mut *mut unsigned int fmt_type; / USB audio format type (1-3),
    pub /: *mut *mut unsigned int pkt_offset_adj; / Bytes to drop from beginning of packets (for non-compliant devices),
    pub /: *mut *mut unsigned int stream_offset_adj; / Bytes to drop from beginning of stream (for non-compliant devices),
    pub /: *mut *mut unsigned int opened:1; / pcm device opened,
    pub /: *mut *mut unsigned int running: 1; / running status,
    pub /: *mut *mut unsigned int period_elapsed_pending; / delay period handling,
    pub /: *mut *mut unsigned int buffer_bytes; / buffer size in bytes,
    pub /: *mut *mut unsigned int inflight_bytes; / in-flight data bytes on buffer (for playback),
    pub /: *mut *mut unsigned int hwptr_done; / processed byte position in the buffer,
    pub /: *mut *mut unsigned int transfer_done; / processed frames since last period update,
    pub /: *mut *mut unsigned int frame_limit; / limits number of packets in URB,
// data and sync endpoints for this stream
    pub /: *mut *mut unsigned int ep_num; / the endpoint number,
    pub data_endpoint: *mut snd_usb_endpoint,
    pub sync_endpoint: *mut snd_usb_endpoint,
    pub flags: c_ulong,
    pub /: *mut *mut unsigned int speed; / USB_SPEED_XXX,
    pub /: *mut *mut u64 formats; / format bitmasks (all or'ed),
    pub /: *mut *mut unsigned int num_formats; / number of supported audio formats (list),
    pub /: *mut *mut list_head fmt_list; / format list,
    pub lock: spinlock_t,
    pub /: *mut *mut unsigned int last_frame_number; / stored frame number,
    pub marker: c_int,
    pub channel: c_int,
    pub byte_idx: c_int,
    pub dsd_dop: },
    pub /: *mut *mut bool trigger_tstamp_pending_update; / trigger timestamp being updated from initial estimate,
    pub /: *mut *mut bool lowlatency_playback; / low-latency playback mode,
    pub media_ctl: *mut media_ctl,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_usb_stream {
    pub chip: *mut snd_usb_audio,
    pub pcm: *mut snd_pcm,
    pub pcm_index: c_int,
    pub /: *mut *mut unsigned int fmt_type; / USB audio format type (1-3),
    pub substream: [snd_usb_substream; 2],
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_usb_platform_ops {
    pub chip): *mut *mut void (connect_cb)(struct snd_usb_audio,
    pub chip): *mut *mut void (disconnect_cb)(struct snd_usb_audio,
    pub message): *mut *mut *mut void (suspend_cb)(struct usb_interface intf, pm_message_t,
    pub intf): *mut *mut void (resume_cb)(struct usb_interface,
}

extern "C" {
    pub fn snd_usb_register_platform_ops(ops: *mut snd_usb_platform_ops) -> c_int;
}
extern "C" {
    pub fn snd_usb_unregister_platform_ops() -> c_int;
}
extern "C" {
    pub fn snd_usb_rediscover_devices();
}
