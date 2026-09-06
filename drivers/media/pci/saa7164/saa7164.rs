//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/saa7164/saa7164.h
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
// Driver for the NXP SAA7164 PCIe bridge
//
// Copyright (c) 2010-2015 Steven Toth <stoth@kernellabs.com>
//

pub const SAA7164_MAXBOARDS: c_int = 8;

pub const SAA7164_BOARD_UNKNOWN: c_int = 0;
pub const SAA7164_BOARD_UNKNOWN_REV2: c_int = 1;
pub const SAA7164_BOARD_UNKNOWN_REV3: c_int = 2;
pub const SAA7164_BOARD_HAUPPAUGE_HVR2250: c_int = 3;
pub const SAA7164_BOARD_HAUPPAUGE_HVR2200: c_int = 4;
pub const SAA7164_BOARD_HAUPPAUGE_HVR2200_2: c_int = 5;
pub const SAA7164_BOARD_HAUPPAUGE_HVR2200_3: c_int = 6;
pub const SAA7164_BOARD_HAUPPAUGE_HVR2250_2: c_int = 7;
pub const SAA7164_BOARD_HAUPPAUGE_HVR2250_3: c_int = 8;
pub const SAA7164_BOARD_HAUPPAUGE_HVR2200_4: c_int = 9;
pub const SAA7164_BOARD_HAUPPAUGE_HVR2200_5: c_int = 10;
pub const SAA7164_BOARD_HAUPPAUGE_HVR2255proto: c_int = 11;
pub const SAA7164_BOARD_HAUPPAUGE_HVR2255: c_int = 12;
pub const SAA7164_BOARD_HAUPPAUGE_HVR2205: c_int = 13;
pub const SAA7164_MAX_UNITS: c_int = 8;
pub const SAA7164_TS_NUMBER_OF_LINES: c_int = 312;
pub const SAA7164_PS_NUMBER_OF_LINES: c_int = 256;

pub const SAA7164_MAX_VBI_BUFFERS: c_int = 64;
// Port related defines

pub const DBGLVL_FW: c_int = 4;
pub const DBGLVL_DVB: c_int = 8;
pub const DBGLVL_I2C: c_int = 16;
pub const DBGLVL_API: c_int = 32;
pub const DBGLVL_CMD: c_int = 64;
pub const DBGLVL_BUS: c_int = 128;
pub const DBGLVL_IRQ: c_int = 256;
pub const DBGLVL_BUF: c_int = 512;
pub const DBGLVL_ENC: c_int = 1024;
pub const DBGLVL_VBI: c_int = 2048;
pub const DBGLVL_THR: c_int = 4096;
pub const DBGLVL_CPU: c_int = 8192;

// TV frequency range copied from tuner-core.c

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum port_t {
    SAA7164_MPEG_UNDEFINED = 0,
    SAA7164_MPEG_DVB,
    SAA7164_MPEG_ENCODER,
    SAA7164_MPEG_VBI,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum saa7164_i2c_bus_nr {
    SAA7164_I2C_BUS_0 = 0,
    SAA7164_I2C_BUS_1,
    SAA7164_I2C_BUS_2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum saa7164_buffer_flags {
    SAA7164_BUFFER_UNDEFINED = 0,
    SAA7164_BUFFER_FREE,
    SAA7164_BUFFER_BUSY,
    SAA7164_BUFFER_FULL
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum saa7164_unit_type {
    SAA7164_UNIT_UNDEFINED = 0,
    SAA7164_UNIT_DIGITAL_DEMODULATOR,
    SAA7164_UNIT_ANALOG_DEMODULATOR,
    SAA7164_UNIT_TUNER,
    SAA7164_UNIT_EEPROM,
    SAA7164_UNIT_ZILOG_IRBLASTER,
    SAA7164_UNIT_ENCODER,
}

// The PCIe bridge doesn't grant direct access to i2c.
// Instead, you address i2c devices using a uniqely
// allocated 'unitid' value via a messaging API. This
// is a problem. The kernel and existing demod/tuner
// drivers expect to talk 'i2c', so we have to maintain
// a translation layer, and a series of functions to
// convert i2c bus + device address into a unit id.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct saa7164_unit {
    pub type: saa7164_unit_type,
    pub id: u8,
    pub name: *mut c_char,
    pub i2c_bus_nr: saa7164_i2c_bus_nr,
    pub i2c_bus_addr: u8,
    pub i2c_reg_len: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct saa7164_board {
    pub name: *mut c_char,
    pub portf: portd, porte,,
    pub chiprev: },
    pub unit: [saa7164_unit; SAA7164_MAX_UNITS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct saa7164_subid {
    pub subvendor: u16,
    pub subdevice: u16,
    pub card: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct saa7164_encoder_fh {
    pub fh: v4l2_fh,
    pub port: *mut saa7164_port,
    pub v4l_reading: core::sync::atomic::AtomicI32,
}

extern "C" {
    pub fn container_of(_arg: file_to_v4l2_fh(filp), saa7164_encoder_fh: struct, _arg: fh) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct saa7164_vbi_fh {
    pub fh: v4l2_fh,
    pub port: *mut saa7164_port,
    pub v4l_reading: core::sync::atomic::AtomicI32,
}

extern "C" {
    pub fn container_of(_arg: file_to_v4l2_fh(filp), saa7164_vbi_fh: struct, _arg: fh) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct saa7164_histogram_bucket {
    pub val: u32,
    pub count: u32,
    pub update_time: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct saa7164_histogram {
    pub name: [c_char; 32],
    pub counter1: [saa7164_histogram_bucket; 64],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct saa7164_user_buffer {
    pub list: list_head,
// Attributes
    pub data: *mut u8,
    pub pos: u32,
    pub actual_size: u32,
    pub crc: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct saa7164_fw_status {
// RISC Core details
    pub status: u32,
    pub mode: u32,
    pub spec: u32,
    pub inst: u32,
    pub cpuload: u32,
    pub remainheap: u32,
// Firmware version
    pub version: u32,
    pub major: u32,
    pub sub: u32,
    pub rel: u32,
    pub buildnr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct saa7164_dvb {
    pub lock: mutex,
    pub adapter: dvb_adapter,
    pub frontend: *mut dvb_frontend,
    pub demux: dvb_demux,
    pub dmxdev: dmxdev,
    pub fe_hw: dmx_frontend,
    pub fe_mem: dmx_frontend,
    pub net: dvb_net,
    pub feeding: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct saa7164_i2c {
    pub dev: *mut saa7164_dev,
    pub nr: saa7164_i2c_bus_nr,
// I2C I/O
    pub i2c_adap: i2c_adapter,
    pub i2c_client: i2c_client,
    pub i2c_rc: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct saa7164_tvnorm {
    pub name: *mut c_char,
    pub id: v4l2_std_id,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct saa7164_encoder_params {
    pub encodernorm: saa7164_tvnorm,
    pub height: u32,
    pub width: u32,
    pub is_50hz: u32,
    pub /: *mut *mut u32 bitrate; / bps,
    pub /: *mut *mut u32 bitrate_peak; / bps,
    pub bitrate_mode: u32,
    pub /: *mut *mut u32 stream_type; / V4L2_MPEG_STREAM_TYPE_MPEG2_TS,
    pub audio_sampling_freq: u32,
    pub ctl_mute: u32,
    pub ctl_aspect: u32,
    pub refdist: u32,
    pub gop_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct saa7164_vbi_params {
    pub encodernorm: saa7164_tvnorm,
    pub height: u32,
    pub width: u32,
    pub is_50hz: u32,
    pub /: *mut *mut u32 bitrate; / bps,
    pub /: *mut *mut u32 bitrate_peak; / bps,
    pub bitrate_mode: u32,
    pub /: *mut *mut u32 stream_type; / V4L2_MPEG_STREAM_TYPE_MPEG2_TS,
    pub audio_sampling_freq: u32,
    pub ctl_mute: u32,
    pub ctl_aspect: u32,
    pub refdist: u32,
    pub gop_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct saa7164_buffer {
    pub list: list_head,
// Note of which h/w buffer list index position we occupy
    pub idx: c_int,
    pub port: *mut saa7164_port,
// Hardware Specific
// PCI Memory allocations
    pub /: *mut *mut saa7164_buffer_flags flags; / Free, Busy, Full,
// A block of page align PCI memory
    pub /: *mut *mut u32 pci_size; / PCI allocation size in bytes,
    pub /: *mut *mut *mut u64 cpu; / Virtual address,
    pub /: *mut *mut dma_addr_t dma; / Physical address,
    pub /: *mut *mut u32 crc; / Checksum for the entire buffer data,
// A page table that splits the block into a number of entries
    pub /: *mut *mut u32 pt_size; / PCI allocation size in bytes,
    pub /: *mut *mut *mut u64 pt_cpu; / Virtual address,
    pub /: *mut *mut dma_addr_t pt_dma; / Physical address,
// Encoder fops
    pub pos: u32,
    pub actual_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct saa7164_port {
    pub dev: *mut saa7164_dev,
    pub type: port_t,
    pub nr: c_int,
// --- Generic port attributes ---
// HW stream parameters
    pub hw_streamingparams: tmHWStreamParameters,
// DMA configuration values, is seeded during initialization
    pub hwcfg: tmComResDMATermDescrHeader,
// hardware specific registers
    pub bufcounter: u32,
    pub pitch: u32,
    pub bufsize: u32,
    pub bufoffset: u32,
    pub bufptr32l: u32,
    pub bufptr32h: u32,
    pub bufptr64: u64,
    pub /: *mut *mut u32 numpte; / Number of entries in array, only valid in head,
    pub dmaqueue_lock: mutex,
    pub dmaqueue: saa7164_buffer,
    pub last_svc_msecs: u64 last_irq_msecs,,
    pub last_svc_msecs_diff: u64 last_irq_msecs_diff,,
    pub last_svc_wp: u32,
    pub last_svc_rp: u32,
    pub last_irq_svc_msecs_diff: u64,
    pub last_read_msecs_diff: u64 last_read_msecs,,
    pub last_poll_msecs_diff: u64 last_poll_msecs,,
    pub irq_interval: saa7164_histogram,
    pub svc_interval: saa7164_histogram,
    pub irq_svc_interval: saa7164_histogram,
    pub read_interval: saa7164_histogram,
    pub poll_interval: saa7164_histogram,
// --- DVB Transport Specific ---
    pub dvb: saa7164_dvb,
    pub i2c_client_demod: *mut i2c_client,
    pub i2c_client_tuner: *mut i2c_client,
// --- Encoder/V4L related attributes ---
// Encoder
// Defaults established in saa7164-encoder.c
    pub encodernorm: saa7164_tvnorm,
    pub ctrl_handler: v4l2_ctrl_handler,
    pub std: v4l2_std_id,
    pub height: u32,
    pub width: u32,
    pub freq: u32,
    pub mux_input: u8,
    pub encoder_profile: u8,
    pub video_format: u8,
    pub audio_format: u8,
    pub video_resolution: u8,
    pub ctl_brightness: u16,
    pub ctl_contrast: u16,
    pub ctl_hue: u16,
    pub ctl_saturation: u16,
    pub ctl_sharpness: u16,
    pub ctl_volume: i8,
    pub audfeat: tmComResAFeatureDescrHeader,
    pub encunit: tmComResEncoderDescrHeader,
    pub vidproc: tmComResProcDescrHeader,
    pub ifunit: tmComResExtDevDescrHeader,
    pub tunerunit: tmComResTunerDescrHeader,
    pub workenc: work_struct,
// V4L Encoder Video
    pub encoder_params: saa7164_encoder_params,
    pub v4l_device: *mut video_device,
    pub v4l_reader_count: core::sync::atomic::AtomicI32,
    pub list_buf_used: saa7164_buffer,
    pub list_buf_free: saa7164_buffer,
    pub wait_read: wait_queue_head_t,
// V4L VBI
    pub vbi_fmt_ntsc: tmComResVBIFormatDescrHeader,
    pub vbi_params: saa7164_vbi_params,
    pub enc_port: *mut saa7164_port,
// Debug
    pub sync_errors: u32,
    pub v_cc_errors: u32,
    pub a_cc_errors: u32,
    pub last_v_cc: u8,
    pub last_a_cc: u8,
    pub done_first_interrupt: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct saa7164_dev {
    pub devlist: list_head,
    pub refcount: core::sync::atomic::AtomicI32,
    pub v4l2_dev: v4l2_device,
// pci stuff
    pub pci: *mut pci_dev,
    pub pci_lat: unsigned char pci_rev,,
    pub pci_slot: int pci_bus,,
    pub lmmio: *mut u32 __iomem,
    pub bmmio: *mut u8 __iomem,
    pub lmmio2: *mut u32 __iomem,
    pub bmmio2: *mut u8 __iomem,
    pub pci_irqmask: c_int,
// board details
    pub nr: c_int,
    pub hwrevision: c_int,
    pub board: u32,
    pub name: [c_char; 16],
// firmware status
    pub fw_status: saa7164_fw_status,
    pub firmwareloaded: u32,
    pub hwdesc: tmComResHWDescr,
    pub intfdesc: tmComResInterfaceDescr,
    pub busdesc: tmComResBusDescr,
    pub bus: tmComResBusInfo,
// Interrupt status and ack registers
    pub int_status: u32,
    pub int_ack: u32,
    pub msi: bool,
    pub cmds: [cmd; SAA_CMD_MAX_MSG_UNITS],
    pub lock: mutex,
// I2c related
    pub i2c_bus: [saa7164_i2c; 3],
// Transport related
    pub ports: [saa7164_port; SAA7164_MAX_PORTS],
// Deferred command/api interrupts handling
    pub workcmd: work_struct,
// A kernel thread to monitor the firmware log, used
// only in debug mode.
//
    pub kthread: *mut task_struct,
}

// -----------------------------------------------------------
// saa7164-core.c
extern "C" {
    pub fn saa7164_dumpregs(dev: *mut saa7164_dev, addr: u32);
}
extern "C" {
    pub fn saa7164_getfirmwarestatus(dev: *mut saa7164_dev);
}
extern "C" {
    pub fn saa7164_getcurrentfirmwareversion(dev: *mut saa7164_dev) -> u32;
}
extern "C" {
    pub fn saa7164_histogram_update(hg: *mut saa7164_histogram, val: u32);
}
// -----------------------------------------------------------
// saa7164-fw.c
extern "C" {
    pub fn saa7164_downloadfirmware(dev: *mut saa7164_dev) -> c_int;
}
// -----------------------------------------------------------
// saa7164-i2c.c
extern "C" {
    pub fn saa7164_i2c_register(bus: *mut saa7164_i2c) -> c_int;
}
extern "C" {
    pub fn saa7164_i2c_unregister(bus: *mut saa7164_i2c) -> c_int;
}
// -----------------------------------------------------------
// saa7164-bus.c
extern "C" {
    pub fn saa7164_bus_setup(dev: *mut saa7164_dev) -> c_int;
}
extern "C" {
    pub fn saa7164_bus_dump(dev: *mut saa7164_dev);
}
// -----------------------------------------------------------
// saa7164-cmd.c
extern "C" {
    pub fn saa7164_irq_dequeue(dev: *mut saa7164_dev) -> c_int;
}
// -----------------------------------------------------------
// saa7164-api.c
extern "C" {
    pub fn saa7164_api_get_fw_version(dev: *mut saa7164_dev, version: *mut u32) -> c_int;
}
extern "C" {
    pub fn saa7164_api_enum_subdevs(dev: *mut saa7164_dev) -> c_int;
}
extern "C" {
    pub fn saa7164_api_read_eeprom(dev: *mut saa7164_dev, buf: *mut u8, buflen: c_int) -> c_int;
}
extern "C" {
    pub fn saa7164_api_set_gpiobit(dev: *mut saa7164_dev, unitid: u8, pin: u8) -> c_int;
}
extern "C" {
    pub fn saa7164_api_clear_gpiobit(dev: *mut saa7164_dev, unitid: u8, pin: u8) -> c_int;
}
extern "C" {
    pub fn saa7164_api_transition_port(port: *mut saa7164_port, mode: u8) -> c_int;
}
extern "C" {
    pub fn saa7164_api_initialize_dif(port: *mut saa7164_port) -> c_int;
}
extern "C" {
    pub fn saa7164_api_configure_dif(port: *mut saa7164_port, std: u32) -> c_int;
}
extern "C" {
    pub fn saa7164_api_set_encoder(port: *mut saa7164_port) -> c_int;
}
extern "C" {
    pub fn saa7164_api_get_encoder(port: *mut saa7164_port) -> c_int;
}
extern "C" {
    pub fn saa7164_api_set_aspect_ratio(port: *mut saa7164_port) -> c_int;
}
extern "C" {
    pub fn saa7164_api_set_usercontrol(port: *mut saa7164_port, ctl: u8) -> c_int;
}
extern "C" {
    pub fn saa7164_api_get_usercontrol(port: *mut saa7164_port, ctl: u8) -> c_int;
}
extern "C" {
    pub fn saa7164_api_set_videomux(port: *mut saa7164_port) -> c_int;
}
extern "C" {
    pub fn saa7164_api_audio_mute(port: *mut saa7164_port, mute: c_int) -> c_int;
}
extern "C" {
    pub fn saa7164_api_set_audio_volume(port: *mut saa7164_port, level: i8) -> c_int;
}
extern "C" {
    pub fn saa7164_api_set_audio_std(port: *mut saa7164_port) -> c_int;
}
extern "C" {
    pub fn saa7164_api_set_audio_detection(port: *mut saa7164_port, autodetect: c_int) -> c_int;
}
extern "C" {
    pub fn saa7164_api_get_videomux(port: *mut saa7164_port) -> c_int;
}
extern "C" {
    pub fn saa7164_api_set_vbi_format(port: *mut saa7164_port) -> c_int;
}
extern "C" {
    pub fn saa7164_api_set_debug(dev: *mut saa7164_dev, level: u8) -> c_int;
}
extern "C" {
    pub fn saa7164_api_collect_debug(dev: *mut saa7164_dev) -> c_int;
}
// -----------------------------------------------------------
// saa7164-cards.c
extern "C" {
    pub fn saa7164_card_list(dev: *mut saa7164_dev);
}
extern "C" {
    pub fn saa7164_gpio_setup(dev: *mut saa7164_dev);
}
extern "C" {
    pub fn saa7164_card_setup(dev: *mut saa7164_dev);
}
extern "C" {
    pub fn saa7164_i2caddr_to_reglen(bus: *mut saa7164_i2c, addr: c_int) -> c_int;
}
extern "C" {
    pub fn saa7164_i2caddr_to_unitid(bus: *mut saa7164_i2c, addr: c_int) -> c_int;
}
// -----------------------------------------------------------
// saa7164-dvb.c
extern "C" {
    pub fn saa7164_dvb_register(port: *mut saa7164_port) -> c_int;
}
extern "C" {
    pub fn saa7164_dvb_unregister(port: *mut saa7164_port) -> c_int;
}
// -----------------------------------------------------------
// saa7164-buffer.c
extern "C" {
    pub fn saa7164_buffer_dealloc(buf: *mut saa7164_buffer) -> c_int;
}
extern "C" {
    pub fn saa7164_buffer_activate(buf: *mut saa7164_buffer, i: c_int) -> c_int;
}
extern "C" {
    pub fn saa7164_buffer_cfg_port(port: *mut saa7164_port) -> c_int;
}
extern "C" {
    pub fn saa7164_buffer_dealloc_user(buf: *mut saa7164_user_buffer);
}
extern "C" {
    pub fn saa7164_buffer_zero_offsets(port: *mut saa7164_port, i: c_int) -> c_int;
}
// -----------------------------------------------------------
// saa7164-encoder.c
extern "C" {
    pub fn saa7164_s_std(port: *mut saa7164_port, id: v4l2_std_id) -> c_int;
}
extern "C" {
    pub fn saa7164_g_std(port: *mut saa7164_port, id: *mut v4l2_std_id) -> c_int;
}
extern "C" {
    pub fn saa7164_enum_input(file: *mut file, priv: *mut c_void, i: *mut v4l2_input) -> c_int;
}
extern "C" {
    pub fn saa7164_g_input(port: *mut saa7164_port, i: *mut c_uint) -> c_int;
}
extern "C" {
    pub fn saa7164_s_input(port: *mut saa7164_port, i: c_uint) -> c_int;
}
extern "C" {
    pub fn saa7164_g_tuner(file: *mut file, priv: *mut c_void, t: *mut v4l2_tuner) -> c_int;
}
extern "C" {
    pub fn saa7164_s_tuner(file: *mut file, priv: *mut c_void, t: *const v4l2_tuner) -> c_int;
}
extern "C" {
    pub fn saa7164_g_frequency(port: *mut saa7164_port, f: *mut v4l2_frequency) -> c_int;
}
extern "C" {
    pub fn saa7164_encoder_register(port: *mut saa7164_port) -> c_int;
}
extern "C" {
    pub fn saa7164_encoder_unregister(port: *mut saa7164_port);
}
// -----------------------------------------------------------
// saa7164-vbi.c
extern "C" {
    pub fn saa7164_vbi_register(port: *mut saa7164_port) -> c_int;
}
extern "C" {
    pub fn saa7164_vbi_unregister(port: *mut saa7164_port);
}
// -----------------------------------------------------------

