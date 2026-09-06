//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/cx25821/cx25821.h
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
// Driver for the Conexant CX25821 PCIe bridge
//
// Copyright (C) 2009 Conexant Systems Inc.
// Authors  <shu.lin@conexant.com>, <hiep.huynh@conexant.com>
// Based on Steven Toth <stoth@linuxtv.org> cx23885 driver
//

pub const CX25821_MAXBOARDS: c_int = 2;
pub const LINE_SIZE_D1: c_int = 1440;
// Number of decoders and encoders
pub const MAX_DECODERS: c_int = 8;
pub const MAX_ENCODERS: c_int = 2;
pub const QUAD_DECODERS: c_int = 4;
pub const MAX_CAMERAS: c_int = 16;
// Max number of inputs by card
pub const MAX_CX25821_INPUT: c_int = 8;
pub const RESOURCE_VIDEO0: c_int = 1;
pub const RESOURCE_VIDEO1: c_int = 2;
pub const RESOURCE_VIDEO2: c_int = 4;
pub const RESOURCE_VIDEO3: c_int = 8;
pub const RESOURCE_VIDEO4: c_int = 16;
pub const RESOURCE_VIDEO5: c_int = 32;
pub const RESOURCE_VIDEO6: c_int = 64;
pub const RESOURCE_VIDEO7: c_int = 128;
pub const RESOURCE_VIDEO8: c_int = 256;
pub const RESOURCE_VIDEO9: c_int = 512;
pub const RESOURCE_VIDEO10: c_int = 1024;
pub const RESOURCE_VIDEO11: c_int = 2048;

pub const UNKNOWN_BOARD: c_int = 0;
pub const CX25821_BOARD: c_int = 1;
// Currently supported by the driver

pub const CX25821_BOARD_CONEXANT_ATHENA10: c_int = 1;
pub const MAX_VID_CHANNEL_NUM: c_int = 12;
//
// Maximum capture-only channels. This can go away once video/audio output
// is fully supported in this driver.
//
pub const MAX_VID_CAP_CHANNEL_NUM: c_int = 10;
pub const VID_CHANNEL_NUM: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx25821_fmt {
    pub /: *mut *mut u32 fourcc; / v4l2 format id,
    pub depth: c_int,
    pub flags: c_int,
    pub cxformat: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx25821_tvnorm {
    pub name: *mut c_char,
    pub id: v4l2_std_id,
    pub cxiformat: u32,
    pub cxoformat: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cx25821_src_sel_type {
    CX25821_SRC_SEL_EXT_656_VIDEO = 0,
    CX25821_SRC_SEL_PARALLEL_MPEG_VIDEO
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx25821_riscmem {
    pub size: c_uint,
    pub cpu: *mut __le32,
    pub jmp: *mut __le32,
    pub dma: dma_addr_t,
}

// buffer for one video frame
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx25821_buffer {
// common v4l buffer stuff -- must be first
    pub vb: vb2_v4l2_buffer,
    pub queue: list_head,
// cx25821 specific
    pub bpl: c_uint,
    pub risc: cx25821_riscmem,
    pub fmt: *const cx25821_fmt,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum port {
    CX25821_UNDEFINED = 0,
    CX25821_RAW,
    CX25821_264
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx25821_board {
    pub name: *const c_char,
    pub porta: port,
    pub portb: port,
    pub portc: port,
    pub clk_freq: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx25821_i2c {
    pub dev: *mut cx25821_dev,
    pub nr: c_int,
// i2c i/o
    pub i2c_adap: i2c_adapter,
    pub i2c_client: i2c_client,
    pub i2c_rc: u32,
// cx25821 registers used for raw address
    pub i2c_period: u32,
    pub reg_ctrl: u32,
    pub reg_stat: u32,
    pub reg_addr: u32,
    pub reg_rdata: u32,
    pub reg_wdata: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx25821_dmaqueue {
    pub active: list_head,
    pub count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx25821_video_out_data {
    pub chan: *mut cx25821_channel,
    pub _line_size: c_int,
    pub _prog_cnt: c_int,
    pub _pixel_format: c_int,
    pub _is_first_frame: c_int,
    pub _is_running: c_int,
    pub _file_status: c_int,
    pub _lines_count: c_int,
    pub _frame_count: c_int,
    pub _risc_size: c_uint,
    pub _dma_virt_start_addr: *mut __le32,
    pub _dma_virt_addr: *mut __le32,
    pub _dma_phys_addr: dma_addr_t,
    pub _dma_phys_start_addr: dma_addr_t,
    pub _data_buf_size: c_uint,
    pub _data_buf_virt_addr: *mut __le32,
    pub _data_buf_phys_addr: dma_addr_t,
    pub upstream_riscbuf_size: u32,
    pub upstream_databuf_size: u32,
    pub is_60hz: c_int,
    pub _frame_index: c_int,
    pub cur_frame_index: c_int,
    pub curpos: c_int,
    pub waitq: wait_queue_head_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx25821_channel {
    pub id: unsigned,
    pub dev: *mut cx25821_dev,
    pub hdl: v4l2_ctrl_handler,
    pub vdev: video_device,
    pub dma_vidq: cx25821_dmaqueue,
    pub vidq: vb2_queue,
    pub sram_channels: *const sram_channel,
    pub fmt: *const cx25821_fmt,
    pub field: unsigned,
    pub height: unsigned int width,,
    pub pixel_formats: c_int,
    pub use_cif_resolution: c_int,
    pub cif_width: c_int,
// video output data for the video output channel
    pub out: *mut cx25821_video_out_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx25821_dev {
    pub v4l2_dev: v4l2_device,
// pci stuff
    pub pci: *mut pci_dev,
    pub pci_lat: unsigned char pci_rev,,
    pub pci_slot: int pci_bus,,
    pub base_io_addr: u32,
    pub lmmio: *mut u32 __iomem,
    pub bmmio: *mut u8 __iomem,
    pub pci_irqmask: c_int,
    pub hwrevision: c_int,
// used by cx25821-alsa
    pub card: *mut snd_card,
    pub clk_freq: u32,
// I2C adapters: Master 1 & 2 (External) & Master 3 (Internal only)
    pub i2c_bus: [cx25821_i2c; 3],
    pub nr: c_int,
    pub lock: mutex,
    pub channels: [cx25821_channel; MAX_VID_CHANNEL_NUM],
// board details
    pub board: c_uint,
    pub name: [c_char; 32],
// Analog video
    pub input: c_uint,
    pub tvnorm: v4l2_std_id,
    pub _max_num_decoders: c_ushort,
// Analog Audio Upstream
    pub _audio_is_running: c_int,
    pub _audiopixel_format: c_int,
    pub _is_first_audio_frame: c_int,
    pub _audiofile_status: c_int,
    pub _audio_lines_count: c_int,
    pub _audioframe_count: c_int,
    pub _audio_upstream_channel: c_int,
    pub /: *mut *mut int _last_index_irq; / The last interrupt index processed.,
    pub _risc_audio_jmp_addr: *mut __le32,
    pub _risc_virt_start_addr: *mut __le32,
    pub _risc_virt_addr: *mut __le32,
    pub _risc_phys_addr: dma_addr_t,
    pub _risc_phys_start_addr: dma_addr_t,
    pub _audiorisc_size: c_uint,
    pub _audiodata_buf_size: c_uint,
    pub _audiodata_buf_virt_addr: *mut __le32,
    pub _audiodata_buf_phys_addr: dma_addr_t,
    pub _audiofilename: *mut c_char,
    pub audio_upstream_riscbuf_size: u32,
    pub audio_upstream_databuf_size: u32,
    pub _audioframe_index: c_int,
    pub _audio_work_entry: work_struct,
    pub input_audiofilename: *mut c_char,
// V4l
    pub slock: spinlock_t,
// Video Upstream
    pub vid_out_data: [cx25821_video_out_data; 2],
}

extern "C" {
    pub fn container_of(_arg: v4l2_dev, cx25821_dev: struct, _arg: v4l2_dev) -> return;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sram_channel {
    pub name: *mut c_char,
    pub i: u32,
    pub cmds_start: u32,
    pub ctrl_start: u32,
    pub cdt: u32,
    pub fifo_start: u32,
    pub fifo_size: u32,
    pub ptr1_reg: u32,
    pub ptr2_reg: u32,
    pub cnt1_reg: u32,
    pub cnt2_reg: u32,
    pub int_msk: u32,
    pub int_stat: u32,
    pub int_mstat: u32,
    pub dma_ctl: u32,
    pub gpcnt_ctl: u32,
    pub gpcnt: u32,
    pub aud_length: u32,
    pub aud_cfg: u32,
    pub fld_aud_fifo_en: u32,
    pub fld_aud_risc_en: u32,
// For Upstream Video
    pub vid_fmt_ctl: u32,
    pub vid_active_ctl1: u32,
    pub vid_active_ctl2: u32,
    pub vid_cdt_size: u32,
    pub vip_ctl: u32,
    pub pix_frmt: u32,
    pub jumponly: u32,
    pub irq_bit: u32,
}

extern "C" {
    pub fn cx25821_i2c_register(bus: *mut cx25821_i2c) -> c_int;
}
extern "C" {
    pub fn cx25821_i2c_read(bus: *mut cx25821_i2c, reg_addr: u16, value: *mut c_int) -> c_int;
}
extern "C" {
    pub fn cx25821_i2c_write(bus: *mut cx25821_i2c, reg_addr: u16, value: c_int) -> c_int;
}
extern "C" {
    pub fn cx25821_i2c_unregister(bus: *mut cx25821_i2c) -> c_int;
}
extern "C" {
    pub fn cx25821_gpio_init(dev: *mut cx25821_dev);
}
extern "C" {
    pub fn medusa_video_init(dev: *mut cx25821_dev) -> c_int;
}
extern "C" {
    pub fn medusa_set_videostandard(dev: *mut cx25821_dev) -> c_int;
}
extern "C" {
    pub fn medusa_set_hue(dev: *mut cx25821_dev, hue: c_int, decoder: c_int) -> c_int;
}
extern "C" {
    pub fn cx25821_dev_unregister(dev: *mut cx25821_dev);
}
