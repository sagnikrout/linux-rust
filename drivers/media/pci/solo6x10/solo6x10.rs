//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/solo6x10/solo6x10.h
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
// Copyright (C) 2010-2013 Bluecherry, LLC <https://www.bluecherrydvr.com>
//
// Original author:
// Ben Collins <bcollins@ubuntu.com>
//
// Additional work by:
// John Brooks <john.brooks@bluecherry.net>
//

pub const PCI_VENDOR_ID_SOFTLOGIC: c_uint = 0x9413;
pub const PCI_DEVICE_ID_SOLO6010: c_uint = 0x6010;
pub const PCI_DEVICE_ID_SOLO6110: c_uint = 0x6110;

pub const PCI_VENDOR_ID_BLUECHERRY: c_uint = 0x1BB3;
// Neugent Softlogic 6010 based cards
pub const PCI_DEVICE_ID_NEUSOLO_4: c_uint = 0x4304;
pub const PCI_DEVICE_ID_NEUSOLO_9: c_uint = 0x4309;
pub const PCI_DEVICE_ID_NEUSOLO_16: c_uint = 0x4310;
// Bluecherry Softlogic 6010 based cards
pub const PCI_DEVICE_ID_BC_SOLO_4: c_uint = 0x4E04;
pub const PCI_DEVICE_ID_BC_SOLO_9: c_uint = 0x4E09;
pub const PCI_DEVICE_ID_BC_SOLO_16: c_uint = 0x4E10;
// Bluecherry Softlogic 6110 based cards
pub const PCI_DEVICE_ID_BC_6110_4: c_uint = 0x5304;
pub const PCI_DEVICE_ID_BC_6110_8: c_uint = 0x5308;
pub const PCI_DEVICE_ID_BC_6110_16: c_uint = 0x5310;

// Used in pci_device_id, and solo_dev->type
pub const SOLO_DEV_6010: c_int = 0;
pub const SOLO_DEV_6110: c_int = 1;

pub const SOLO_MAX_CHANNELS: c_int = 16;

//
// The SOLO6x10 actually has 8 i2c channels, but we only use 2.
// 0 - Techwell chip(s)
// 1 - SAA7128
//
pub const SOLO_I2C_ADAPTERS: c_int = 2;
pub const SOLO_I2C_TW: c_int = 0;
pub const SOLO_I2C_SAA: c_int = 1;
// DMA Engine setup
pub const SOLO_NR_P2M: c_int = 4;
pub const SOLO_NR_P2M_DESC: c_int = 256;

// Encoder standard modes
pub const SOLO_ENC_MODE_CIF: c_int = 2;
pub const SOLO_ENC_MODE_HD1: c_int = 1;
pub const SOLO_ENC_MODE_D1: c_int = 9;
pub const SOLO_DEFAULT_QP: c_int = 3;

//
// Motion thresholds are in a table of 64x64 samples, with
// each sample representing 16x16 pixels of the source. In
// effect, 44x30 samples are used for NTSC, and 44x36 for PAL.
// The 5th sample on the 10th row is (10*64)+5 = 645.
//
// Internally it is stored as a 45x45 array (45*16 = 720, which is the
// maximum PAL/NTSC width).
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SOLO_I2C_STATE {
    IIC_STATE_IDLE,
    IIC_STATE_START,
    IIC_STATE_READ,
    IIC_STATE_WRITE,
    IIC_STATE_STOP
}

// Defined in Table 4-16, Page 68-69 of the 6010 Datasheet
#[repr(C)]
#[derive(Copy, Clone)]
pub struct solo_p2m_desc {
    pub ctrl: u32,
    pub cfg: u32,
    pub dma_addr: u32,
    pub ext_addr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct solo_p2m_dev {
    pub mutex: mutex,
    pub completion: completion,
    pub desc_count: c_int,
    pub desc_idx: c_int,
    pub descs: *mut solo_p2m_desc,
    pub error: c_int,
}

pub const OSD_TEXT_MAX: c_int = 44;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct solo_vb2_buf {
    pub vb: vb2_v4l2_buffer,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum solo_enc_types {
    SOLO_ENC_TYPE_STD,
    SOLO_ENC_TYPE_EXT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct solo_enc_dev {
    pub solo_dev: *mut solo_dev,
// V4L2 Items
    pub hdl: v4l2_ctrl_handler,
    pub md_thresholds: *mut v4l2_ctrl,
    pub vfd: *mut video_device,
// General accounting
    pub lock: mutex,
    pub motion_lock: spinlock_t,
    pub ch: u8,
    pub interval: u8 mode, gop, qp, interlaced,,
    pub bw_weight: u8,
    pub motion_thresh: u16,
    pub motion_global: bool,
    pub motion_enabled: bool,
    pub width: u16,
    pub height: u16,
// OSD buffers
    pub 1]: char osd_text[OSD_TEXT_MAX +,
// VOP stuff
    pub vop: [u8; 64],
    pub vop_len: c_int,
    pub jpeg_header: [u8; 1024],
    pub jpeg_len: c_int,
    pub fmt: u32,
    pub type: solo_enc_types,
    pub sequence: u32,
    pub vidq: vb2_queue,
    pub vidq_active: list_head,
    pub desc_count: c_int,
    pub desc_nelts: c_int,
    pub desc_items: *mut solo_p2m_desc,
    pub desc_dma: dma_addr_t,
    pub av_lock: spinlock_t,
}

// The SOLO6x10 PCI Device
#[repr(C)]
#[derive(Copy, Clone)]
pub struct solo_dev {
// General stuff
    pub pdev: *mut pci_dev,
    pub type: c_int,
    pub time_sync: c_uint,
    pub usec_lsb: c_uint,
    pub clock_mhz: c_uint,
    pub reg_base: *mut u8 __iomem,
    pub nr_chans: c_int,
    pub nr_ext: c_int,
    pub irq_mask: u32,
    pub motion_mask: u32,
    pub v4l2_dev: v4l2_device,

// GPIO
    pub gpio_dev: gpio_chip,

// tw28xx accounting
    pub tw2815: u8 tw2865, tw2864,,
    pub tw28_cnt: u8,
// i2c related items
    pub i2c_adap: [i2c_adapter; SOLO_I2C_ADAPTERS],
    pub i2c_state: SOLO_I2C_STATE,
    pub i2c_mutex: mutex,
    pub i2c_id: c_int,
    pub i2c_wait: wait_queue_head_t,
    pub i2c_msg: *mut i2c_msg,
    pub i2c_msg_num: c_uint,
    pub i2c_msg_ptr: c_uint,
// P2M DMA Engine
    pub p2m_dev: [solo_p2m_dev; SOLO_NR_P2M],
    pub p2m_count: core::sync::atomic::AtomicI32,
    pub p2m_jiffies: c_int,
    pub p2m_timeouts: c_uint,
// V4L2 Display items
    pub vfd: *mut video_device,
    pub erasing: c_uint,
    pub frame_blank: c_uint,
    pub cur_disp_ch: u8,
    pub disp_thread_wait: wait_queue_head_t,
    pub disp_hdl: v4l2_ctrl_handler,
// V4L2 Encoder items
    pub v4l2_enc: [*mut solo_enc_dev; SOLO_MAX_CHANNELS],
    pub enc_bw_remain: u16,
// IDX into hw mp4 encoder
    pub enc_idx: u8,
// Current video settings
    pub video_type: u32,
    pub video_vsize: u16 video_hsize,,
    pub vout_vstart: u16 vout_hstart,,
    pub vin_vstart: u16 vin_hstart,,
    pub fps: u8,
// JPEG Qp setting
    pub jpeg_qp_lock: spinlock_t,
    pub jpeg_qp: [u32; 2],
// Audio components
    pub snd_card: *mut snd_card,
    pub snd_pcm: *mut snd_pcm,
    pub snd_users: core::sync::atomic::AtomicI32,
    pub g723_hw_idx: c_int,
// sysfs stuffs
    pub dev: device,
    pub sdram_size: c_int,
    pub sdram_attr: bin_attribute,
    pub sys_config: c_uint,
// Ring thread
    pub ring_thread: *mut task_struct,
    pub ring_thread_wait: wait_queue_head_t,
// VOP_HEADER handling
    pub vh_buf: *mut c_void,
    pub vh_dma: dma_addr_t,
    pub vh_size: c_int,
// Buffer handling
    pub vidq: vb2_queue,
    pub sequence: u32,
    pub kthread: *mut task_struct,
    pub lock: mutex,
    pub slock: spinlock_t,
    pub old_write: c_int,
    pub vidq_active: list_head,
}

extern "C" {
    pub fn readl(reg: solo_dev->reg_base +) -> return;
}
// Init/exit routines for subsystems
extern "C" {
    pub fn solo_disp_init(solo_dev: *mut solo_dev) -> c_int;
}
extern "C" {
    pub fn solo_disp_exit(solo_dev: *mut solo_dev);
}
extern "C" {
    pub fn solo_gpio_init(solo_dev: *mut solo_dev) -> c_int;
}
extern "C" {
    pub fn solo_gpio_exit(solo_dev: *mut solo_dev);
}
extern "C" {
    pub fn solo_i2c_init(solo_dev: *mut solo_dev) -> c_int;
}
extern "C" {
    pub fn solo_i2c_exit(solo_dev: *mut solo_dev);
}
extern "C" {
    pub fn solo_p2m_init(solo_dev: *mut solo_dev) -> c_int;
}
extern "C" {
    pub fn solo_p2m_exit(solo_dev: *mut solo_dev);
}
extern "C" {
    pub fn solo_v4l2_init(solo_dev: *mut solo_dev, nr: unsigned) -> c_int;
}
extern "C" {
    pub fn solo_v4l2_exit(solo_dev: *mut solo_dev);
}
extern "C" {
    pub fn solo_enc_init(solo_dev: *mut solo_dev) -> c_int;
}
extern "C" {
    pub fn solo_enc_exit(solo_dev: *mut solo_dev);
}
extern "C" {
    pub fn solo_enc_v4l2_init(solo_dev: *mut solo_dev, nr: unsigned) -> c_int;
}
extern "C" {
    pub fn solo_enc_v4l2_exit(solo_dev: *mut solo_dev);
}
extern "C" {
    pub fn solo_g723_init(solo_dev: *mut solo_dev) -> c_int;
}
extern "C" {
    pub fn solo_g723_exit(solo_dev: *mut solo_dev);
}
// ISR's
extern "C" {
    pub fn solo_i2c_isr(solo_dev: *mut solo_dev) -> c_int;
}
extern "C" {
    pub fn solo_p2m_isr(solo_dev: *mut solo_dev, id: c_int);
}
extern "C" {
    pub fn solo_p2m_error_isr(solo_dev: *mut solo_dev);
}
extern "C" {
    pub fn solo_enc_v4l2_isr(solo_dev: *mut solo_dev);
}
extern "C" {
    pub fn solo_g723_isr(solo_dev: *mut solo_dev);
}
extern "C" {
    pub fn solo_motion_isr(solo_dev: *mut solo_dev);
}
extern "C" {
    pub fn solo_video_in_isr(solo_dev: *mut solo_dev);
}
// i2c read/write
extern "C" {
    pub fn solo_i2c_readbyte(solo_dev: *mut solo_dev, id: c_int, addr: u8, off: u8) -> u8;
}
// P2M DMA
// Global s_std ioctl
extern "C" {
    pub fn solo_set_video_type(solo_dev: *mut solo_dev, is_50hz: bool) -> c_int;
}
extern "C" {
    pub fn solo_update_mode(solo_enc: *mut solo_enc_dev);
}
// Set the threshold for motion detection
extern "C" {
    pub fn solo_set_motion_threshold(solo_dev: *mut solo_dev, ch: u8, val: u16) -> c_int;
}
pub const SOLO_DEF_MOT_THRESH: c_uint = 0x0300;
// Write text on OSD
extern "C" {
    pub fn solo_osd_print(solo_enc: *mut solo_enc_dev) -> c_int;
}
// EEPROM commands
extern "C" {
    pub fn solo_eeprom_ewen(solo_dev: *mut solo_dev, w_en: c_int) -> c_uint;
}
extern "C" {
    pub fn solo_eeprom_read(solo_dev: *mut solo_dev, loc: c_int) -> __be16;
}
// JPEG Qp functions
extern "C" {
    pub fn solo_g_jpeg_qp(solo_dev: *mut solo_dev, ch: c_uint) -> c_int;
}

