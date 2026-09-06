//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/cx23885/cx23885.h
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
// Driver for the Conexant CX23885 PCIe bridge
//
// Copyright (c) 2006 Steven Toth <stoth@linuxtv.org>
//

pub const CX23885_MAXBOARDS: c_int = 8;
// Max number of inputs by card
pub const MAX_CX23885_INPUT: c_int = 8;

pub const CX23885_BOARD_UNKNOWN: c_int = 0;
pub const CX23885_BOARD_HAUPPAUGE_HVR1800lp: c_int = 1;
pub const CX23885_BOARD_HAUPPAUGE_HVR1800: c_int = 2;
pub const CX23885_BOARD_HAUPPAUGE_HVR1250: c_int = 3;
pub const CX23885_BOARD_DVICO_FUSIONHDTV_5_EXP: c_int = 4;
pub const CX23885_BOARD_HAUPPAUGE_HVR1500Q: c_int = 5;
pub const CX23885_BOARD_HAUPPAUGE_HVR1500: c_int = 6;
pub const CX23885_BOARD_HAUPPAUGE_HVR1200: c_int = 7;
pub const CX23885_BOARD_HAUPPAUGE_HVR1700: c_int = 8;
pub const CX23885_BOARD_HAUPPAUGE_HVR1400: c_int = 9;
pub const CX23885_BOARD_DVICO_FUSIONHDTV_7_DUAL_EXP: c_int = 10;
pub const CX23885_BOARD_DVICO_FUSIONHDTV_DVB_T_DUAL_EXP: c_int = 11;
pub const CX23885_BOARD_LEADTEK_WINFAST_PXDVR3200_H: c_int = 12;
pub const CX23885_BOARD_COMPRO_VIDEOMATE_E650F: c_int = 13;
pub const CX23885_BOARD_TBS_6920: c_int = 14;
pub const CX23885_BOARD_TEVII_S470: c_int = 15;
pub const CX23885_BOARD_DVBWORLD_2005: c_int = 16;
pub const CX23885_BOARD_NETUP_DUAL_DVBS2_CI: c_int = 17;
pub const CX23885_BOARD_HAUPPAUGE_HVR1270: c_int = 18;
pub const CX23885_BOARD_HAUPPAUGE_HVR1275: c_int = 19;
pub const CX23885_BOARD_HAUPPAUGE_HVR1255: c_int = 20;
pub const CX23885_BOARD_HAUPPAUGE_HVR1210: c_int = 21;
pub const CX23885_BOARD_MYGICA_X8506: c_int = 22;
pub const CX23885_BOARD_MAGICPRO_PROHDTVE2: c_int = 23;
pub const CX23885_BOARD_HAUPPAUGE_HVR1850: c_int = 24;
pub const CX23885_BOARD_COMPRO_VIDEOMATE_E800: c_int = 25;
pub const CX23885_BOARD_HAUPPAUGE_HVR1290: c_int = 26;
pub const CX23885_BOARD_MYGICA_X8558PRO: c_int = 27;
pub const CX23885_BOARD_LEADTEK_WINFAST_PXTV1200: c_int = 28;
pub const CX23885_BOARD_GOTVIEW_X5_3D_HYBRID: c_int = 29;
pub const CX23885_BOARD_NETUP_DUAL_DVB_T_C_CI_RF: c_int = 30;
pub const CX23885_BOARD_LEADTEK_WINFAST_PXDVR3200_H_XC4000: c_int = 31;
pub const CX23885_BOARD_MPX885: c_int = 32;
pub const CX23885_BOARD_MYGICA_X8507: c_int = 33;
pub const CX23885_BOARD_TERRATEC_CINERGY_T_PCIE_DUAL: c_int = 34;
pub const CX23885_BOARD_TEVII_S471: c_int = 35;
pub const CX23885_BOARD_HAUPPAUGE_HVR1255_22111: c_int = 36;
pub const CX23885_BOARD_PROF_8000: c_int = 37;
pub const CX23885_BOARD_HAUPPAUGE_HVR4400: c_int = 38;
pub const CX23885_BOARD_AVERMEDIA_HC81R: c_int = 39;
pub const CX23885_BOARD_TBS_6981: c_int = 40;
pub const CX23885_BOARD_TBS_6980: c_int = 41;
pub const CX23885_BOARD_LEADTEK_WINFAST_PXPVR2200: c_int = 42;
pub const CX23885_BOARD_HAUPPAUGE_IMPACTVCBE: c_int = 43;
pub const CX23885_BOARD_DVICO_FUSIONHDTV_DVB_T_DUAL_EXP2: c_int = 44;
pub const CX23885_BOARD_DVBSKY_T9580: c_int = 45;
pub const CX23885_BOARD_DVBSKY_T980C: c_int = 46;
pub const CX23885_BOARD_DVBSKY_S950C: c_int = 47;
pub const CX23885_BOARD_TT_CT2_4500_CI: c_int = 48;
pub const CX23885_BOARD_DVBSKY_S950: c_int = 49;
pub const CX23885_BOARD_DVBSKY_S952: c_int = 50;
pub const CX23885_BOARD_DVBSKY_T982: c_int = 51;
pub const CX23885_BOARD_HAUPPAUGE_HVR5525: c_int = 52;
pub const CX23885_BOARD_HAUPPAUGE_STARBURST: c_int = 53;
pub const CX23885_BOARD_VIEWCAST_260E: c_int = 54;
pub const CX23885_BOARD_VIEWCAST_460E: c_int = 55;
pub const CX23885_BOARD_HAUPPAUGE_QUADHD_DVB: c_int = 56;
pub const CX23885_BOARD_HAUPPAUGE_QUADHD_ATSC: c_int = 57;
pub const CX23885_BOARD_HAUPPAUGE_HVR1265_K4: c_int = 58;
pub const CX23885_BOARD_HAUPPAUGE_STARBURST2: c_int = 59;
pub const CX23885_BOARD_HAUPPAUGE_QUADHD_DVB_885: c_int = 60;
pub const CX23885_BOARD_HAUPPAUGE_QUADHD_ATSC_885: c_int = 61;
pub const CX23885_BOARD_AVERMEDIA_CE310B: c_int = 62;
pub const CX23885_BOARD_AVERMEDIA_H789C: c_int = 63;
pub const GPIO_0: c_uint = 0x00000001;
pub const GPIO_1: c_uint = 0x00000002;
pub const GPIO_2: c_uint = 0x00000004;
pub const GPIO_3: c_uint = 0x00000008;
pub const GPIO_4: c_uint = 0x00000010;
pub const GPIO_5: c_uint = 0x00000020;
pub const GPIO_6: c_uint = 0x00000040;
pub const GPIO_7: c_uint = 0x00000080;
pub const GPIO_8: c_uint = 0x00000100;
pub const GPIO_9: c_uint = 0x00000200;
pub const GPIO_10: c_uint = 0x00000400;
pub const GPIO_11: c_uint = 0x00000800;
pub const GPIO_12: c_uint = 0x00001000;
pub const GPIO_13: c_uint = 0x00002000;
pub const GPIO_14: c_uint = 0x00004000;
pub const GPIO_15: c_uint = 0x00008000;
// Currently unsupported by the driver: PAL/H, NTSC/Kr, SECAM B/G/H/LC

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx23885_fmt {
    pub /: *mut *mut u32 fourcc; / v4l2 format id,
    pub depth: c_int,
    pub flags: c_int,
    pub cxformat: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx23885_tvnorm {
    pub name: *mut c_char,
    pub id: v4l2_std_id,
    pub cxiformat: u32,
    pub cxoformat: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cx23885_itype {
    CX23885_VMUX_COMPOSITE1 = 1,
    CX23885_VMUX_COMPOSITE2,
    CX23885_VMUX_COMPOSITE3,
    CX23885_VMUX_COMPOSITE4,
    CX23885_VMUX_SVIDEO,
    CX23885_VMUX_COMPONENT,
    CX23885_VMUX_TELEVISION,
    CX23885_VMUX_CABLE,
    CX23885_VMUX_DVB,
    CX23885_VMUX_DEBUG,
    CX23885_RADIO,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cx23885_src_sel_type {
    CX23885_SRC_SEL_EXT_656_VIDEO = 0,
    CX23885_SRC_SEL_PARALLEL_MPEG_VIDEO
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx23885_riscmem {
    pub size: c_uint,
    pub cpu: *mut __le32,
    pub jmp: *mut __le32,
    pub dma: dma_addr_t,
}

// buffer for one video frame
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx23885_buffer {
// common v4l buffer stuff -- must be first
    pub vb: vb2_v4l2_buffer,
    pub queue: list_head,
// cx23885 specific
    pub bpl: c_uint,
    pub risc: cx23885_riscmem,
    pub fmt: *mut cx23885_fmt,
    pub count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx23885_input {
    pub type: cx23885_itype,
    pub vmux: c_uint,
    pub amux: c_uint,
    pub gpio3: u32 gpio0, gpio1, gpio2,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx23885_board {
    pub name: *mut c_char,
    pub portc: port_t porta, portb,,
    pub num_fds_portc: int num_fds_portb,,
    pub tuner_type: c_uint,
    pub radio_type: c_uint,
    pub tuner_addr: c_uchar,
    pub radio_addr: c_uchar,
    pub tuner_bus: c_uint,
// Vendors can and do run the PCIe bridge at different
// clock rates, driven physically by crystals on the PCBs.
// The core has to accommodate this. This allows the user
// to add new boards with new frequencys. The value is
// expressed in Hz.
//
// The core framework will default this value based on
// current designs, but it can vary.
//
    pub clk_freq: u32,
    pub input: [cx23885_input; MAX_CX23885_INPUT],
    pub /: *mut *mut int ci_type; / for NetUP,
// Force bottom field first during DMA (888 workaround)
    pub force_bff: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx23885_subid {
    pub subvendor: u16,
    pub subdevice: u16,
    pub card: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx23885_i2c {
    pub dev: *mut cx23885_dev,
    pub nr: c_int,
// i2c i/o
    pub i2c_adap: i2c_adapter,
    pub i2c_client: i2c_client,
    pub i2c_rc: u32,
// 885 registers used for raw address
    pub i2c_period: u32,
    pub reg_ctrl: u32,
    pub reg_stat: u32,
    pub reg_addr: u32,
    pub reg_rdata: u32,
    pub reg_wdata: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx23885_dmaqueue {
    pub active: list_head,
    pub count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx23885_tsport {
    pub dev: *mut cx23885_dev,
    pub nr: unsigned,
    pub sram_chno: c_int,
    pub frontends: vb2_dvb_frontends,
// dma queues
    pub mpegq: cx23885_dmaqueue,
    pub ts_packet_size: u32,
    pub ts_packet_count: u32,
    pub width: c_int,
    pub height: c_int,
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
    pub open): *mut *mut *mut void (gate_ctrl)(struct cx23885_tsport port, int,
    pub port_priv: *mut c_void,
// Workaround for a temp dvb_frontend that the tuner can attached to
    pub analog_fe: dvb_frontend,
    pub i2c_client_demod: *mut i2c_client,
    pub i2c_client_tuner: *mut i2c_client,
    pub i2c_client_sec: *mut i2c_client,
    pub i2c_client_ci: *mut i2c_client,
    pub fe): *mut *mut int (set_frontend)(struct dvb_frontend,
    pub voltage): fe_sec_voltage,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx23885_kernel_ir {
    pub cx: *mut cx23885_dev,
    pub name: *mut c_char,
    pub phys: *mut c_char,
    pub rc: *mut rc_dev,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx23885_audio_buffer {
    pub bpl: c_uint,
    pub risc: cx23885_riscmem,
    pub vaddr: *mut c_void,
    pub sglist: *mut scatterlist,
    pub sglen: c_int,
    pub nr_pages: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx23885_audio_dev {
    pub dev: *mut cx23885_dev,
    pub pci: *mut pci_dev,
    pub card: *mut snd_card,
    pub lock: spinlock_t,
    pub count: core::sync::atomic::AtomicI32,
    pub dma_size: c_uint,
    pub period_size: c_uint,
    pub num_periods: c_uint,
    pub buf: *mut cx23885_audio_buffer,
    pub substream: *mut snd_pcm_substream,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cx23885_dev {
    pub refcount: core::sync::atomic::AtomicI32,
    pub v4l2_dev: v4l2_device,
    pub ctrl_handler: v4l2_ctrl_handler,
// pci stuff
    pub pci: *mut pci_dev,
    pub pci_lat: unsigned char pci_rev,,
    pub pci_slot: int pci_bus,,
    pub lmmio: *mut u32 __iomem,
    pub bmmio: *mut u8 __iomem,
    pub pci_irqmask: c_int,
    pub /: *mut *mut spinlock_t pci_irqmask_lock; / protects mask reg too,
    pub hwrevision: c_int,
// This valud is board specific and is used to configure the
// AV core so we see nice clean and stable video and audio.
    pub clk_freq: u32,
// I2C adapters: Master 1 & 2 (External) & Master 3 (Internal only)
    pub i2c_bus: [cx23885_i2c; 3],
    pub nr: c_int,
    pub lock: mutex,
    pub gpio_lock: mutex,
// board details
    pub board: c_uint,
    pub name: [c_char; 32],
    pub ts2: cx23885_tsport ts1,,
// sram configuration
    pub sram_channels: *mut sram_channel,
    pub bridge: },
// Analog video
    pub input: c_uint,
    pub /: *mut *mut unsigned int audinput; / Selectable audio input,
    pub tvaudio: u32,
    pub tvnorm: v4l2_std_id,
    pub tuner_type: c_uint,
    pub tuner_addr: c_uchar,
    pub tuner_bus: c_uint,
    pub radio_type: c_uint,
    pub radio_addr: c_uchar,
    pub sd_cx25840: *mut v4l2_subdev,
    pub cx25840_work: work_struct,
    pub disable_analog: c_uint,
// Infrared
    pub sd_ir: *mut v4l2_subdev,
    pub ir_rx_work: work_struct,
    pub ir_rx_notifications: c_ulong,
    pub ir_tx_work: work_struct,
    pub ir_tx_notifications: c_ulong,
    pub kernel_ir: *mut cx23885_kernel_ir,
    pub ir_input_stopping: core::sync::atomic::AtomicI32,
// V4l
    pub freq: u32,
    pub video_dev: *mut video_device,
    pub vbi_dev: *mut video_device,
// video capture
    pub fmt: *mut cx23885_fmt,
    pub height: unsigned int width,,
    pub field: unsigned,
    pub vidq: cx23885_dmaqueue,
    pub vb2_vidq: vb2_queue,
    pub vbiq: cx23885_dmaqueue,
    pub vb2_vbiq: vb2_queue,
    pub slock: spinlock_t,
// MPEG Encoder ONLY settings
    pub cx23417_mailbox: u32,
    pub cxhdl: cx2341x_handler,
    pub v4l_device: *mut video_device,
    pub vb2_mpegq: vb2_queue,
    pub encodernorm: cx23885_tvnorm,
// Analog raw audio
    pub audio_dev: *mut cx23885_audio_dev,
// Does the system require periodic DMA resets?
    pub need_dma_reset:1: c_uint,
}

extern "C" {
    pub fn container_of(_arg: v4l2_dev, cx23885_dev: struct, _arg: v4l2_dev) -> return;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sram_channel {
    pub name: *mut c_char,
    pub cmds_start: u32,
    pub ctrl_start: u32,
    pub cdt: u32,
    pub fifo_start: u32,
    pub fifo_size: u32,
    pub ptr1_reg: u32,
    pub ptr2_reg: u32,
    pub cnt1_reg: u32,
    pub cnt2_reg: u32,
    pub jumponly: u32,
}

// -----------------------------------------------------------

// -----------------------------------------------------------
// cx23885-core.c
extern "C" {
    pub fn cx23885_cancel_buffers(port: *mut cx23885_tsport);
}
extern "C" {
    pub fn cx23885_gpio_set(dev: *mut cx23885_dev, mask: u32);
}
extern "C" {
    pub fn cx23885_gpio_clear(dev: *mut cx23885_dev, mask: u32);
}
extern "C" {
    pub fn cx23885_gpio_get(dev: *mut cx23885_dev, mask: u32) -> u32;
}
extern "C" {
    pub fn cx23885_irq_add_enable(dev: *mut cx23885_dev, mask: u32);
}
extern "C" {
    pub fn cx23885_irq_enable(dev: *mut cx23885_dev, mask: u32);
}
extern "C" {
    pub fn cx23885_irq_disable(dev: *mut cx23885_dev, mask: u32);
}
extern "C" {
    pub fn cx23885_irq_remove(dev: *mut cx23885_dev, mask: u32);
}
// -----------------------------------------------------------
// cx23885-cards.c
extern "C" {
    pub fn cx23885_card_list(dev: *mut cx23885_dev);
}
extern "C" {
    pub fn cx23885_ir_init(dev: *mut cx23885_dev) -> c_int;
}
extern "C" {
    pub fn cx23885_ir_pci_int_enable(dev: *mut cx23885_dev);
}
extern "C" {
    pub fn cx23885_ir_fini(dev: *mut cx23885_dev);
}
extern "C" {
    pub fn cx23885_gpio_setup(dev: *mut cx23885_dev);
}
extern "C" {
    pub fn cx23885_card_setup(dev: *mut cx23885_dev);
}
extern "C" {
    pub fn cx23885_card_setup_pre_i2c(dev: *mut cx23885_dev);
}
extern "C" {
    pub fn cx23885_dvb_register(port: *mut cx23885_tsport) -> c_int;
}
extern "C" {
    pub fn cx23885_dvb_unregister(port: *mut cx23885_tsport) -> c_int;
}
// -----------------------------------------------------------
// cx23885-video.c
// Video
extern "C" {
    pub fn cx23885_video_register(dev: *mut cx23885_dev) -> c_int;
}
extern "C" {
    pub fn cx23885_video_unregister(dev: *mut cx23885_dev);
}
extern "C" {
    pub fn cx23885_video_irq(dev: *mut cx23885_dev, status: u32) -> c_int;
}
extern "C" {
    pub fn cx23885_enum_input(dev: *mut cx23885_dev, i: *mut v4l2_input) -> c_int;
}
extern "C" {
    pub fn cx23885_set_input(file: *mut file, priv: *mut c_void, i: c_uint) -> c_int;
}
extern "C" {
    pub fn cx23885_get_input(file: *mut file, priv: *mut c_void, i: *mut c_uint) -> c_int;
}
extern "C" {
    pub fn cx23885_set_frequency(file: *mut file, priv: *mut c_void, f: *const v4l2_frequency) -> c_int;
}
extern "C" {
    pub fn cx23885_set_tvnorm(dev: *mut cx23885_dev, norm: v4l2_std_id) -> c_int;
}
// -----------------------------------------------------------
// cx23885-vbi.c
extern "C" {
    pub fn cx23885_vbi_timeout(data: c_ulong);
}
extern "C" {
    pub fn cx23885_vbi_irq(dev: *mut cx23885_dev, status: u32) -> c_int;
}
// cx23885-i2c.c
extern "C" {
    pub fn cx23885_i2c_register(bus: *mut cx23885_i2c) -> c_int;
}
extern "C" {
    pub fn cx23885_i2c_unregister(bus: *mut cx23885_i2c) -> c_int;
}
extern "C" {
    pub fn cx23885_av_clk(dev: *mut cx23885_dev, enable: c_int);
}
// -----------------------------------------------------------
// cx23885-417.c
extern "C" {
    pub fn cx23885_417_register(dev: *mut cx23885_dev) -> c_int;
}
extern "C" {
    pub fn cx23885_417_unregister(dev: *mut cx23885_dev);
}
extern "C" {
    pub fn cx23885_irq_417(dev: *mut cx23885_dev, status: u32) -> c_int;
}
extern "C" {
    pub fn cx23885_417_check_encoder(dev: *mut cx23885_dev);
}
extern "C" {
    pub fn cx23885_mc417_init(dev: *mut cx23885_dev);
}
extern "C" {
    pub fn mc417_memory_read(dev: *mut cx23885_dev, address: u32, value: *mut u32) -> c_int;
}
extern "C" {
    pub fn mc417_memory_write(dev: *mut cx23885_dev, address: u32, value: u32) -> c_int;
}
extern "C" {
    pub fn mc417_gpio_set(dev: *mut cx23885_dev, mask: u32);
}
extern "C" {
    pub fn mc417_gpio_clear(dev: *mut cx23885_dev, mask: u32);
}
extern "C" {
    pub fn mc417_gpio_enable(dev: *mut cx23885_dev, mask: u32, asoutput: c_int);
}
// -----------------------------------------------------------
// cx23885-alsa.c
extern "C" {
    pub fn cx23885_audio_unregister(dev: *mut cx23885_dev);
}
extern "C" {
    pub fn cx23885_audio_irq(dev: *mut cx23885_dev, status: u32, mask: u32) -> c_int;
}
// -----------------------------------------------------------
// tv norms
