//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/cx231xx/cx231xx.h
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

pub const PWR_SLEEP_INTERVAL: c_int = 10;
// I2C addresses for control block in Cx231xx
pub const AFE_DEVICE_ADDRESS: c_uint = 0x60;
pub const I2S_BLK_DEVICE_ADDRESS: c_uint = 0x98;
pub const VID_BLK_I2C_ADDRESS: c_uint = 0x88;
pub const VERVE_I2C_ADDRESS: c_uint = 0x40;
pub const DIF_USE_BASEBAND: c_uint = 0xFFFFFFFF;
// Boards supported by driver
pub const CX231XX_BOARD_UNKNOWN: c_int = 0;
pub const CX231XX_BOARD_CNXT_CARRAERA: c_int = 1;
pub const CX231XX_BOARD_CNXT_SHELBY: c_int = 2;
pub const CX231XX_BOARD_CNXT_RDE_253S: c_int = 3;
pub const CX231XX_BOARD_CNXT_RDU_253S: c_int = 4;
pub const CX231XX_BOARD_CNXT_VIDEO_GRABBER: c_int = 5;
pub const CX231XX_BOARD_CNXT_RDE_250: c_int = 6;
pub const CX231XX_BOARD_CNXT_RDU_250: c_int = 7;
pub const CX231XX_BOARD_HAUPPAUGE_EXETER: c_int = 8;
pub const CX231XX_BOARD_HAUPPAUGE_USBLIVE2: c_int = 9;
pub const CX231XX_BOARD_PV_PLAYTV_USB_HYBRID: c_int = 10;
pub const CX231XX_BOARD_PV_XCAPTURE_USB: c_int = 11;
pub const CX231XX_BOARD_KWORLD_UB430_USB_HYBRID: c_int = 12;
pub const CX231XX_BOARD_ICONBIT_U100: c_int = 13;
pub const CX231XX_BOARD_HAUPPAUGE_USB2_FM_PAL: c_int = 14;
pub const CX231XX_BOARD_HAUPPAUGE_USB2_FM_NTSC: c_int = 15;
pub const CX231XX_BOARD_ELGATO_VIDEO_CAPTURE_V2: c_int = 16;
pub const CX231XX_BOARD_OTG102: c_int = 17;
pub const CX231XX_BOARD_KWORLD_UB445_USB_HYBRID: c_int = 18;
pub const CX231XX_BOARD_HAUPPAUGE_930C_HD_1113xx: c_int = 19;
pub const CX231XX_BOARD_HAUPPAUGE_930C_HD_1114xx: c_int = 20;
pub const CX231XX_BOARD_HAUPPAUGE_955Q: c_int = 21;
pub const CX231XX_BOARD_TERRATEC_GRABBY: c_int = 22;
pub const CX231XX_BOARD_EVROMEDIA_FULL_HYBRID_FULLHD: c_int = 23;
pub const CX231XX_BOARD_ASTROMETA_T2HYBRID: c_int = 24;
pub const CX231XX_BOARD_THE_IMAGING_SOURCE_DFG_USB2_PRO: c_int = 25;
pub const CX231XX_BOARD_HAUPPAUGE_935C: c_int = 26;
pub const CX231XX_BOARD_HAUPPAUGE_975: c_int = 27;
// Limits minimum and default number of buffers
pub const CX231XX_MIN_BUF: c_int = 4;
pub const CX231XX_DEF_BUF: c_int = 12;
pub const CX231XX_DEF_VBI_BUF: c_int = 6;
pub const VBI_LINE_COUNT: c_int = 17;
pub const VBI_LINE_LENGTH: c_int = 1440;
// Limits the max URB message size
pub const URB_MAX_CTRL_SIZE: c_int = 80;
// Params for validated field
pub const CX231XX_BOARD_NOT_VALIDATED: c_int = 1;
pub const CX231XX_BOARD_VALIDATED: c_int = 0;
// maximum number of cx231xx boards
pub const CX231XX_MAXBOARDS: c_int = 8;
// maximum number of frames that can be queued
pub const CX231XX_NUM_FRAMES: c_int = 5;
// number of buffers for isoc transfers
pub const CX231XX_NUM_BUFS: c_int = 8;
// number of packets for each buffer
//
pub const CX231XX_NUM_PACKETS: c_int = 40;
// default alternate; 0 means choose the best
pub const CX231XX_PINOUT: c_int = 0;
pub const CX231XX_INTERLACED_DEFAULT: c_int = 1;
// time to wait when stopping the isoc transfer

pub const SLEEP_S5H1432: c_int = 30;
pub const CX23417_OSC_EN: c_int = 8;
pub const CX23417_RESET: c_int = 9;
pub const EP5_BUF_SIZE: c_int = 4096;
pub const EP5_TIMEOUT_MS: c_int = 2000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx23417_fmt {
    pub /: *mut *mut u32 fourcc; / v4l2 format id,
    pub depth: c_int,
    pub flags: c_int,
    pub cxformat: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cx231xx_mode {
    CX231XX_SUSPEND,
    CX231XX_ANALOG_MODE,
    CX231XX_DIGITAL_MODE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cx231xx_std_mode {
    CX231XX_TV_AIR = 0,
    CX231XX_TV_CABLE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cx231xx_stream_state {
    STREAM_OFF,
    STREAM_INTERRUPT,
    STREAM_ON,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx231xx_isoc_ctl {
// max packet size of isoc transaction
    pub max_pkt_size: c_int,
// number of allocated urbs
    pub num_bufs: c_int,
// urb for isoc transfers
    pub urb: *mut urb,
// transfer buffers for isoc transfer
    pub transfer_buffer: *mut c_char,
// Last buffer command and region
    pub cmd: u8,
    pub pktsize: int pos, size,,
// Last field: ODD or EVEN?
    pub field: c_int,
// Stores incomplete commands
    pub tmp_buf: u32,
    pub tmp_buf_len: c_int,
// Stores already requested buffers
    pub buf: *mut cx231xx_buffer,
// Stores the number of received fields
    pub nfields: c_int,
// isoc urb callback
    pub urb): *mut *mut *mut int (isoc_copy) (struct cx231xx dev, struct urb,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx231xx_bulk_ctl {
// max packet size of bulk transaction
    pub max_pkt_size: c_int,
// number of allocated urbs
    pub num_bufs: c_int,
// urb for bulk transfers
    pub urb: *mut urb,
// transfer buffers for bulk transfer
    pub transfer_buffer: *mut c_char,
// Last buffer command and region
    pub cmd: u8,
    pub pktsize: int pos, size,,
// Last field: ODD or EVEN?
    pub field: c_int,
// Stores incomplete commands
    pub tmp_buf: u32,
    pub tmp_buf_len: c_int,
// Stores already requested buffers
    pub buf: *mut cx231xx_buffer,
// Stores the number of received fields
    pub nfields: c_int,
// bulk urb callback
    pub urb): *mut *mut *mut int (bulk_copy) (struct cx231xx dev, struct urb,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx231xx_fmt {
    pub name: *mut c_char,
    pub /: *mut *mut u32 fourcc; / v4l2 format id,
    pub depth: c_int,
    pub reg: c_int,
}

// buffer for one video frame
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx231xx_buffer {
// common v4l buffer stuff -- must be first
    pub vb: vb2_v4l2_buffer,
    pub list: list_head,
    pub frame: list_head,
    pub top_field: c_int,
    pub receiving: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ps_package_head {
    CX231XX_NEED_ADD_PS_PACKAGE_HEAD = 0,
    CX231XX_NONEED_PS_PACKAGE_HEAD
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx231xx_dmaqueue {
    pub active: list_head,
    pub wq: wait_queue_head_t,
// Counters to control buffer fill
    pub pos: c_int,
    pub is_partial_line: u8,
    pub partial_buf: [u8; 8],
    pub last_sav: u8,
    pub current_field: c_int,
    pub bytes_left_in_line: u32,
    pub lines_completed: u32,
    pub field1_done: u8,
    pub lines_per_field: u32,
    pub sequence: u32,
// Mpeg2 control buffer
    pub p_left_data: *mut u8,
    pub left_data_count: u32,
    pub mpeg_buffer_done: u8,
    pub mpeg_buffer_completed: u32,
    pub add_ps_package_head: ps_package_head,
    pub ps_head: [c_char; 10],
}

// inputs
pub const MAX_CX231XX_INPUT: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cx231xx_itype {
    CX231XX_VMUX_COMPOSITE1 = 1,
    CX231XX_VMUX_SVIDEO,
    CX231XX_VMUX_TELEVISION,
    CX231XX_VMUX_CABLE,
    CX231XX_RADIO,
    CX231XX_VMUX_DVB,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cx231xx_v_input {
    CX231XX_VIN_1_1 = 0x1,
    CX231XX_VIN_2_1,
    CX231XX_VIN_3_1,
    CX231XX_VIN_4_1,
    CX231XX_VIN_1_2 = 0x01,
    CX231XX_VIN_2_2,
    CX231XX_VIN_3_2,
    CX231XX_VIN_1_3 = 0x1,
    CX231XX_VIN_2_3,
    CX231XX_VIN_3_3,
}

// cx231xx has two audio inputs: tuner and line in
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cx231xx_amux {
// This is the only entry for cx231xx tuner input
    CX231XX_AMUX_VIDEO,	/* cx231xx tuner */
    CX231XX_AMUX_LINE_IN,	/* Line In */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx231xx_reg_seq {
    pub bit: c_uchar,
    pub val: c_uchar,
    pub sleep: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx231xx_input {
    pub type: cx231xx_itype,
    pub vmux: c_uint,
    pub amux: cx231xx_amux,
    pub gpio: *mut cx231xx_reg_seq,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cx231xx_decoder {
    CX231XX_NODECODER,
    CX231XX_AVDECODER
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CX231XX_I2C_MASTER_PORT {
    I2C_0 = 0,       /* master 0 - internal connection */
    I2C_1 = 1,       /* master 1 - used with mux */
    I2C_2 = 2,       /* master 2 */
    I2C_1_MUX_1 = 3, /* master 1 - port 1 (I2C_DEMOD_EN = 0) */
    I2C_1_MUX_3 = 4  /* master 1 - port 3 (I2C_DEMOD_EN = 1) */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx231xx_board {
    pub name: *mut c_char,
    pub vchannels: c_int,
    pub tuner_type: c_int,
    pub tuner_addr: c_int,
    pub /: *mut *mut v4l2_std_id norm; / tv norm,
// demod related
    pub demod_addr: c_int,
    pub demod_addr2: c_int,
    pub /: *mut *mut u8 demod_xfer_mode; / 0 - Serial; 1 - parallel,
// GPIO Pins
    pub dvb_gpio: *mut cx231xx_reg_seq,
    pub suspend_gpio: *mut cx231xx_reg_seq,
    pub tuner_gpio: *mut cx231xx_reg_seq,
// Negative means don't use it
    pub tuner_sif_gpio: i8,
    pub tuner_scl_gpio: i8,
    pub tuner_sda_gpio: i8,
// PIN ctrl
    pub ctl_pin_status_mask: u32,
    pub agc_analog_digital_select_gpio: u8,
    pub gpio_pin_status_mask: u32,
// i2c masters
    pub tuner_i2c_master: u8,
    pub demod_i2c_master: u8,
    pub ir_i2c_master: u8,
// for devices with I2C chips for IR
    pub rc_map_name: *mut c_char,
    pub max_range_640_480:1: c_uint,
    pub has_dvb:1: c_uint,
    pub has_417:1: c_uint,
    pub valid:1: c_uint,
    pub no_alt_vanc:1: c_uint,
    pub external_av:1: c_uint,
    pub i2c_speed: unsigned char xclk,,
    pub decoder: cx231xx_decoder,
    pub output_mode: c_int,
    pub input: [cx231xx_input; MAX_CX231XX_INPUT],
    pub radio: cx231xx_input,
    pub ir_codes: *mut rc_map,
}

// device states
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cx231xx_dev_state {
    DEV_INITIALIZED = 0x01,
    DEV_DISCONNECTED = 0x02,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AFE_MODE {
    AFE_MODE_LOW_IF,
    AFE_MODE_BASEBAND,
    AFE_MODE_EU_HI_IF,
    AFE_MODE_US_HI_IF,
    AFE_MODE_JAPAN_HI_IF
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AUDIO_INPUT {
    AUDIO_INPUT_MUTE,
    AUDIO_INPUT_LINE,
    AUDIO_INPUT_TUNER_TV,
    AUDIO_INPUT_SPDIF,
    AUDIO_INPUT_TUNER_FM
}

pub const CX231XX_AUDIO_BUFS: c_int = 5;
pub const CX231XX_NUM_AUDIO_PACKETS: c_int = 16;
pub const CX231XX_ISO_NUM_AUDIO_PACKETS: c_int = 64;
// cx231xx extensions
pub const CX231XX_AUDIO: c_uint = 0x10;
pub const CX231XX_DVB: c_uint = 0x20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx231xx_audio {
    pub name: [c_char; 50],
    pub transfer_buffer: [*mut c_char; CX231XX_AUDIO_BUFS],
    pub urb: [*mut urb; CX231XX_AUDIO_BUFS],
    pub udev: *mut usb_device,
    pub capture_transfer_done: c_uint,
    pub capture_pcm_substream: *mut snd_pcm_substream,
    pub hwptr_done_capture: c_uint,
    pub sndcard: *mut snd_card,
    pub shutdown: int users,,
// locks
    pub slock: spinlock_t,
    pub /: *mut *mut int alt; / alternate,
    pub /: *mut *mut int max_pkt_size; / max packet size of isoc transaction,
    pub /: *mut *mut int num_alt; / Number of alternative settings,
    pub /: *mut *mut *mut unsigned int alt_max_pkt_size; / array of wMaxPacketSize,
    pub end_point_addr: u16,
}

//
// set/get i2c
// 00--1Mb/s, 01-400kb/s, 10--100kb/s, 11--5Mb/s
pub const I2C_SPEED_1M: c_uint = 0x0;
pub const I2C_SPEED_400K: c_uint = 0x1;
pub const I2C_SPEED_100K: c_uint = 0x2;
pub const I2C_SPEED_5M: c_uint = 0x3;
// 0-- STOP transaction
pub const I2C_STOP: c_uint = 0x0;
// 1-- do not transmit STOP at end of transaction
pub const I2C_NOSTOP: c_uint = 0x1;
// 1--allow slave to insert clock wait states
pub const I2C_SYNC: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx231xx_i2c {
    pub dev: *mut cx231xx,
    pub nr: c_int,
// i2c i/o
    pub i2c_adap: i2c_adapter,
    pub i2c_rc: c_int,
// different settings for each bus
    pub i2c_period: u8,
    pub i2c_nostop: u8,
    pub i2c_reserve: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx231xx_i2c_xfer_data {
    pub dev_addr: u8,
    pub /: *mut *mut u8 direction; / 1 - IN, 0 - OUT,
    pub /: *mut *mut u8 saddr_len; / sub address len,
    pub /: *mut *mut u16 saddr_dat; / sub addr data,
    pub /: *mut *mut u8 buf_size; / buffer size,
    pub /: *mut *mut *mut u8 p_buffer; / pointer to the buffer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VENDOR_REQUEST_IN {
    pub bRequest: u8,
    pub wValue: u16,
    pub wIndex: u16,
    pub wLength: u16,
    pub direction: u8,
    pub bData: u8,
    pub pBuff: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx231xx_tvnorm {
    pub name: *mut c_char,
    pub id: v4l2_std_id,
    pub cxiformat: u32,
    pub cxoformat: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TRANSFER_TYPE {
    Raw_Video = 0,
    Audio,
    Vbi,			/* VANC */
    Sliced_cc,		/* HANC */
    TS1_serial_mode,
    TS2,
    TS1_parallel_mode
    } ;

    struct cx231xx_video_mode {
// Isoc control struct
    struct cx231xx_dmaqueue vidq;
    struct cx231xx_isoc_ctl isoc_ctl;
    struct cx231xx_bulk_ctl bulk_ctl;
// locks
    spinlock_t slock;

// usb transfer
    int alt;		/* alternate */
    int max_pkt_size;	/* max packet size of isoc transaction */
    int num_alt;		/* Number of alternative settings */
    unsigned int *alt_max_pkt_size;	/* array of wMaxPacketSize */
    u16 end_point_addr;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx231xx_tsport {
    pub dev: *mut cx231xx,
    pub nr: c_int,
    pub sram_chno: c_int,
// dma queues
    pub ts_packet_size: u32,
    pub ts_packet_count: u32,
    pub width: c_int,
    pub height: c_int,
// locks
    pub slock: spinlock_t,
// registers
    pub reg_gpcnt: u32,
    pub reg_gpcnt_ctl: u32,
    pub reg_dma_ctl: u32,
    pub reg_lngth: u32,
    pub reg_hw_sop_ctrl: u32,
    pub reg_gen_ctrl: u32,
    pub reg_bd_pkt_status: u32,
    pub reg_sop_status: u32,
    pub reg_fifo_ovfl_stat: u32,
    pub reg_vld_misc: u32,
    pub reg_ts_clk_en: u32,
    pub reg_ts_int_msk: u32,
    pub reg_ts_int_stat: u32,
    pub reg_src_sel: u32,
// Default register vals
    pub pci_irqmask: c_int,
    pub dma_ctl_val: u32,
    pub ts_int_msk_val: u32,
    pub gen_ctrl_val: u32,
    pub ts_clk_en_val: u32,
    pub src_sel_val: u32,
    pub vld_misc_val: u32,
    pub hw_sop_ctrl_val: u32,
// Allow a single tsport to have multiple frontends
    pub num_frontends: u32,
    pub port_priv: *mut c_void,
}

// main device struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx231xx {
// generic device properties
    pub /: *mut *mut char name[30]; / name (including minor) of the device,
    pub /: *mut *mut int model; / index in the device_data struct,
    pub /: *mut *mut int devno; / marks the number of this device,
    pub /: *mut *mut *mut device dev; / pointer to USB interface's dev,
    pub board: cx231xx_board,
// For I2C IR support
    pub init_data: IR_i2c_init_data,
    pub ir_i2c_client: *mut i2c_client,
    pub /: *mut *mut unsigned int stream_on:1; / Locks streams,
    pub /: *mut *mut unsigned int vbi_stream_on:1; / Locks streams for VBI,
    pub has_audio_class:1: c_uint,
    pub has_alsa_audio:1: c_uint,
    pub /: *mut *mut unsigned int i2c_scan_running:1; / true only during i2c_scan,
    pub format: *mut cx231xx_fmt,
    pub v4l2_dev: v4l2_device,
    pub sd_cx25840: *mut v4l2_subdev,
    pub sd_tuner: *mut v4l2_subdev,
    pub ctrl_handler: v4l2_ctrl_handler,
    pub radio_ctrl_handler: v4l2_ctrl_handler,
    pub mpeg_ctrl_handler: cx2341x_handler,
    pub /: *mut *mut work_wq_trigger; / Trigger to start/stop audio for alsa module,
    pub /: *mut *mut atomic_t stream_started; / stream should be running if true,
    pub devlist: list_head,
    pub /: *mut *mut int tuner_type; / type of the tuner,
    pub /: *mut *mut int tuner_addr; / tuner address,
// I2C adapters: Master 1 & 2 (External) & Master 3 (Internal only)
    pub i2c_bus: [cx231xx_i2c; 3],
    pub muxc: *mut i2c_mux_core,
    pub i2c_mux_adap: [*mut i2c_adapter; 2],
    pub xc_fw_load_done:1: c_uint,
    pub port_3_switch_enabled:1: c_uint,
// locks
    pub gpio_i2c_lock: mutex,
    pub i2c_lock: mutex,
// video for linux
    pub /: *mut *mut int users; / user count for exclusive use,
    pub /: *mut *mut video_device vdev; / video for linux device struct,
    pub /: *mut *mut v4l2_std_id norm; / selected tv norm,
    pub /: *mut *mut int ctl_freq; / selected frequency,
    pub /: *mut *mut unsigned int ctl_ainput; / selected audio input,
// frame properties
    pub /: *mut *mut int width; / current frame width,
    pub /: *mut *mut int height; / current frame height,
    pub /: *mut *mut int interlaced; / 1=interlace fields, 0=just top fields,
    pub size: c_uint,
    pub adev: cx231xx_audio,
// states
    pub state: cx231xx_dev_state,
    pub request_module_wk: work_struct,
// locks
    pub lock: mutex,
    pub /: *mut *mut mutex ctrl_urb_lock; / protects urb_buf,
    pub outqueue: list_head inqueue,,
    pub wait_stream: wait_queue_head_t open, wait_frame,,
    pub vbi_dev: video_device,
    pub radio_dev: video_device,

    pub media_dev: *mut media_device,
    pub vbi_pad: media_pad video_pad,,
    pub input_ent: [media_entity; MAX_CX231XX_INPUT],
    pub input_pad: [media_pad; MAX_CX231XX_INPUT],
    pub vidq: vb2_queue,
    pub vbiq: vb2_queue,
    pub eedata: [c_uchar; 256],
    pub video_mode: cx231xx_video_mode,
    pub vbi_mode: cx231xx_video_mode,
    pub sliced_cc_mode: cx231xx_video_mode,
    pub ts1_mode: cx231xx_video_mode,
    pub devlist_count: core::sync::atomic::AtomicI32,
    pub /: *mut *mut *mut usb_device udev; / the usb device,
    pub /: *mut *mut char urb_buf[URB_MAX_CTRL_SIZE]; / urb control msg buffer,
// helper funcs that call usb_control_msg
    pub len): *mut *mut char buf, int,
    pub len): *mut *mut char buf, int,
    pub req_data): *mut cx231xx_i2c_xfer_data,
    pub len): *mut *mut u8 buf, u8,
    pub len): *mut *mut u8 buf, u8,
    pub freq): *mut *mut *mut int (cx231xx_set_analog_freq) (struct cx231xx dev, u32,
    pub dev): *mut *mut int (cx231xx_reset_analog_tuner) (struct cx231xx,
    pub mode: cx231xx_mode,
    pub dvb: *mut cx231xx_dvb,
// Cx231xx supported PCB config's
    pub current_pcb_config: pcb_config,
    pub current_scenario_idx: u8,
    pub interface_count: u8,
    pub max_iad_interface_count: u8,
// GPIO related register direction and values
    pub gpio_dir: u32,
    pub gpio_val: u32,
// Power Modes
    pub power_mode: c_int,
// afe parameters
    pub afe_mode: AFE_MODE,
    pub afe_ref_count: u32,
// video related parameters
    pub video_input: u32,
    pub active_mode: u32,
    pub /: *mut *mut u8 vbi_or_sliced_cc_mode; / 0 - vbi ; 1 - sliced cc mode,
    pub /: *mut *mut cx231xx_std_mode std_mode; / 0 - Air; 1 - cable,
// mode: digital=1 or analog=0
    pub mode_tv: u8,
    pub USE_ISO: u8,
    pub encodernorm: cx231xx_tvnorm,
    pub ts2: cx231xx_tsport ts1,,
    pub mpegq: vb2_queue,
    pub v4l_device: video_device,
    pub v4l_reader_count: core::sync::atomic::AtomicI32,
    pub freq: u32,
    pub input: c_uint,
    pub cx23417_mailbox: u32,
    pub lmmio: *mut u32 __iomem,
    pub bmmio: *mut u8 __iomem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx231xx_ops {
    pub next: list_head,
    pub name: *mut c_char,
    pub id: c_int,
    pub ): *mut *mut int (init) (struct cx231xx,
    pub ): *mut *mut int (fini) (struct cx231xx,
}

// call back functions in dvb module
extern "C" {
    pub fn cx231xx_set_analog_freq(dev: *mut cx231xx, freq: u32) -> c_int;
}
extern "C" {
    pub fn cx231xx_reset_analog_tuner(dev: *mut cx231xx) -> c_int;
}
// Provided by cx231xx-i2c.c
extern "C" {
    pub fn cx231xx_do_i2c_scan(dev: *mut cx231xx, i2c_port: c_int);
}
extern "C" {
    pub fn cx231xx_i2c_register(bus: *mut cx231xx_i2c) -> c_int;
}
extern "C" {
    pub fn cx231xx_i2c_unregister(bus: *mut cx231xx_i2c);
}
extern "C" {
    pub fn cx231xx_i2c_mux_create(dev: *mut cx231xx) -> c_int;
}
extern "C" {
    pub fn cx231xx_i2c_mux_register(dev: *mut cx231xx, mux_no: c_int) -> c_int;
}
extern "C" {
    pub fn cx231xx_i2c_mux_unregister(dev: *mut cx231xx);
}
// Internal block control functions
extern "C" {
    pub fn cx231xx_set_field(field_mask: u32, data: u32) -> u32;
}
// verve r/w
extern "C" {
    pub fn initGPIO(dev: *mut cx231xx);
}
extern "C" {
    pub fn uninitGPIO(dev: *mut cx231xx);
}
// afe related functions
extern "C" {
    pub fn cx231xx_afe_init_super_block(dev: *mut cx231xx, ref_count: u32) -> c_int;
}
extern "C" {
    pub fn cx231xx_afe_init_channels(dev: *mut cx231xx) -> c_int;
}
extern "C" {
    pub fn cx231xx_afe_setup_AFE_for_baseband(dev: *mut cx231xx) -> c_int;
}
extern "C" {
    pub fn cx231xx_afe_set_input_mux(dev: *mut cx231xx, input_mux: u32) -> c_int;
}
extern "C" {
    pub fn cx231xx_afe_set_mode(dev: *mut cx231xx, mode: AFE_MODE) -> c_int;
}
extern "C" {
    pub fn cx231xx_afe_adjust_ref_count(dev: *mut cx231xx, video_input: u32) -> c_int;
}
// i2s block related functions
extern "C" {
    pub fn cx231xx_i2s_blk_initialize(dev: *mut cx231xx) -> c_int;
}
extern "C" {
    pub fn cx231xx_i2s_blk_set_audio_input(dev: *mut cx231xx, audio_input: u8) -> c_int;
}
// DIF related functions
extern "C" {
    pub fn cx231xx_Get_Colibri_CarrierOffset(mode: u32, standerd: u32) -> u32;
}
extern "C" {
    pub fn cx231xx_Setup_AFE_for_LowIF(dev: *mut cx231xx);
}
extern "C" {
    pub fn reset_s5h1432_demod(dev: *mut cx231xx);
}
extern "C" {
    pub fn update_HH_register_after_set_DIF(dev: *mut cx231xx);
}
extern "C" {
    pub fn cx231xx_dif_set_standard(dev: *mut cx231xx, standard: u32) -> c_int;
}
extern "C" {
    pub fn cx231xx_tuner_pre_channel_change(dev: *mut cx231xx) -> c_int;
}
extern "C" {
    pub fn cx231xx_tuner_post_channel_change(dev: *mut cx231xx) -> c_int;
}
// video parser functions
extern "C" {
    pub fn cx231xx_is_buffer_done(dev: *mut cx231xx, dma_q: *mut cx231xx_dmaqueue) -> u8;
}
extern "C" {
    pub fn cx231xx_swab(from: *mut u16, to: *mut u16, len: u16);
}
// Provided by cx231xx-core.c
extern "C" {
    pub fn cx231xx_request_buffers(dev: *mut cx231xx, count: u32) -> u32;
}
extern "C" {
    pub fn cx231xx_queue_unusedframes(dev: *mut cx231xx);
}
extern "C" {
    pub fn cx231xx_release_buffers(dev: *mut cx231xx);
}
// read from control pipe
// write to control pipe
extern "C" {
    pub fn cx231xx_mode_register(dev: *mut cx231xx, address: u16, mode: u32) -> c_int;
}
// Gpio related functions
extern "C" {
    pub fn cx231xx_set_gpio_value(dev: *mut cx231xx, pin_number: c_int, pin_value: c_int) -> c_int;
}
extern "C" {
    pub fn cx231xx_gpio_i2c_start(dev: *mut cx231xx) -> c_int;
}
extern "C" {
    pub fn cx231xx_gpio_i2c_end(dev: *mut cx231xx) -> c_int;
}
extern "C" {
    pub fn cx231xx_gpio_i2c_write_byte(dev: *mut cx231xx, data: u8) -> c_int;
}
extern "C" {
    pub fn cx231xx_gpio_i2c_read_byte(dev: *mut cx231xx, buf: *mut u8) -> c_int;
}
extern "C" {
    pub fn cx231xx_gpio_i2c_read_ack(dev: *mut cx231xx) -> c_int;
}
extern "C" {
    pub fn cx231xx_gpio_i2c_write_ack(dev: *mut cx231xx) -> c_int;
}
extern "C" {
    pub fn cx231xx_gpio_i2c_write_nak(dev: *mut cx231xx) -> c_int;
}
extern "C" {
    pub fn cx231xx_gpio_i2c_read(dev: *mut cx231xx, dev_addr: u8, buf: *mut u8, len: u8) -> c_int;
}
extern "C" {
    pub fn cx231xx_gpio_i2c_write(dev: *mut cx231xx, dev_addr: u8, buf: *mut u8, len: u8) -> c_int;
}
// audio related functions
extern "C" {
    pub fn cx231xx_capture_start(dev: *mut cx231xx, start: c_int, media_type: u8) -> c_int;
}
extern "C" {
    pub fn cx231xx_set_video_alternate(dev: *mut cx231xx) -> c_int;
}
extern "C" {
    pub fn cx231xx_set_alt_setting(dev: *mut cx231xx, index: u8, alt: u8) -> c_int;
}
extern "C" {
    pub fn is_fw_load(dev: *mut cx231xx) -> c_int;
}
extern "C" {
    pub fn cx231xx_check_fw(dev: *mut cx231xx) -> c_int;
}
extern "C" {
    pub fn cx231xx_stop_TS1(dev: *mut cx231xx);
}
extern "C" {
    pub fn cx231xx_start_TS1(dev: *mut cx231xx);
}
extern "C" {
    pub fn cx231xx_uninit_isoc(dev: *mut cx231xx);
}
extern "C" {
    pub fn cx231xx_uninit_bulk(dev: *mut cx231xx);
}
extern "C" {
    pub fn cx231xx_set_mode(dev: *mut cx231xx, set_mode: cx231xx_mode) -> c_int;
}
extern "C" {
    pub fn cx231xx_unmute_audio(dev: *mut cx231xx) -> c_int;
}
extern "C" {
    pub fn cx231xx_ep5_bulkout(dev: *mut cx231xx, firmware: *mut u8, size: u16) -> c_int;
}
extern "C" {
    pub fn cx231xx_disable656(dev: *mut cx231xx);
}
extern "C" {
    pub fn cx231xx_enable656(dev: *mut cx231xx);
}
extern "C" {
    pub fn cx231xx_demod_reset(dev: *mut cx231xx) -> c_int;
}
extern "C" {
    pub fn cx231xx_gpio_set(dev: *mut cx231xx, gpio: *mut cx231xx_reg_seq) -> c_int;
}
// Device list functions
extern "C" {
    pub fn cx231xx_release_resources(dev: *mut cx231xx);
}
extern "C" {
    pub fn cx231xx_release_analog_resources(dev: *mut cx231xx);
}
extern "C" {
    pub fn cx231xx_register_analog_devices(dev: *mut cx231xx) -> c_int;
}
extern "C" {
    pub fn cx231xx_remove_from_devlist(dev: *mut cx231xx);
}
extern "C" {
    pub fn cx231xx_add_into_devlist(dev: *mut cx231xx);
}
extern "C" {
    pub fn cx231xx_init_extension(dev: *mut cx231xx);
}
extern "C" {
    pub fn cx231xx_close_extension(dev: *mut cx231xx);
}
// hardware init functions
extern "C" {
    pub fn cx231xx_dev_init(dev: *mut cx231xx) -> c_int;
}
extern "C" {
    pub fn cx231xx_dev_uninit(dev: *mut cx231xx);
}
extern "C" {
    pub fn cx231xx_config_i2c(dev: *mut cx231xx);
}
extern "C" {
    pub fn cx231xx_config(dev: *mut cx231xx) -> c_int;
}
// Stream control functions
extern "C" {
    pub fn cx231xx_start_stream(dev: *mut cx231xx, ep_mask: u32) -> c_int;
}
extern "C" {
    pub fn cx231xx_stop_stream(dev: *mut cx231xx, ep_mask: u32) -> c_int;
}
extern "C" {
    pub fn cx231xx_initialize_stream_xfer(dev: *mut cx231xx, media_type: u32) -> c_int;
}
// Power control functions
extern "C" {
    pub fn cx231xx_set_power_mode(dev: *mut cx231xx, mode: AV_MODE) -> c_int;
}
// chip specific control functions
extern "C" {
    pub fn cx231xx_init_ctrl_pin_status(dev: *mut cx231xx) -> c_int;
}
extern "C" {
    pub fn cx231xx_enable_i2c_port_3(dev: *mut cx231xx, is_port_3: bool) -> c_int;
}
// video audio decoder related functions
extern "C" {
    pub fn video_mux(dev: *mut cx231xx, index: c_int);
}
extern "C" {
    pub fn cx231xx_set_video_input_mux(dev: *mut cx231xx, input: u8) -> c_int;
}
extern "C" {
    pub fn cx231xx_set_decoder_video_input(dev: *mut cx231xx, pin_type: u8, input: u32) -> c_int;
}
extern "C" {
    pub fn cx231xx_do_mode_ctrl_overrides(dev: *mut cx231xx) -> c_int;
}
extern "C" {
    pub fn cx231xx_set_audio_input(dev: *mut cx231xx, input: u8) -> c_int;
}
// Provided by cx231xx-video.c
extern "C" {
    pub fn cx231xx_register_extension(dev: *mut cx231xx_ops) -> c_int;
}
extern "C" {
    pub fn cx231xx_unregister_extension(dev: *mut cx231xx_ops);
}
extern "C" {
    pub fn cx231xx_init_extension(dev: *mut cx231xx);
}
extern "C" {
    pub fn cx231xx_close_extension(dev: *mut cx231xx);
}
extern "C" {
    pub fn cx231xx_v4l2_create_entities(dev: *mut cx231xx);
}
extern "C" {
    pub fn cx231xx_g_tuner(file: *mut file, priv: *mut c_void, t: *mut v4l2_tuner) -> c_int;
}
extern "C" {
    pub fn cx231xx_s_tuner(file: *mut file, priv: *mut c_void, t: *const v4l2_tuner) -> c_int;
}
extern "C" {
    pub fn cx231xx_g_input(file: *mut file, priv: *mut c_void, i: *mut c_uint) -> c_int;
}
extern "C" {
    pub fn cx231xx_s_input(file: *mut file, priv: *mut c_void, i: c_uint) -> c_int;
}
extern "C" {
    pub fn cx231xx_g_chip_info(file: *mut file, fh: *mut c_void, chip: *mut v4l2_dbg_chip_info) -> c_int;
}
// Provided by cx231xx-cards.c
extern "C" {
    pub fn cx231xx_pre_card_setup(dev: *mut cx231xx);
}
extern "C" {
    pub fn cx231xx_card_setup(dev: *mut cx231xx);
}
extern "C" {
    pub fn cx231xx_tuner_callback(ptr: *mut c_void, component: c_int, command: c_int, arg: c_int) -> c_int;
}
// cx23885-417.c
extern "C" {
    pub fn cx231xx_417_register(dev: *mut cx231xx) -> c_int;
}
extern "C" {
    pub fn cx231xx_417_unregister(dev: *mut cx231xx);
}
// cx23885-input.c

extern "C" {
    pub fn cx231xx_ir_init(dev: *mut cx231xx) -> c_int;
}
extern "C" {
    pub fn cx231xx_ir_exit(dev: *mut cx231xx);
}

