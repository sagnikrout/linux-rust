//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/usb/em28xx/em28xx.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// em28xx.h - driver for Empia EM2800/EM2820/2840 USB video capture devices
//
// Copyright (C) 2005 Markus Rechberger <mrechberger@gmail.com>
// Ludovico Cavedon <cavedon@sssup.it>
// Mauro Carvalho Chehab <mchehab@kernel.org>
// Copyright (C) 2012 Frank Schäfer <fschaefer.oss@googlemail.com>
//
// Based on the em2800 driver from Sascha Sommer <saschasommer@freenet.de>
//

// Boards supported by driver
pub const EM2800_BOARD_UNKNOWN: c_int = 0;
pub const EM2820_BOARD_UNKNOWN: c_int = 1;
pub const EM2820_BOARD_TERRATEC_CINERGY_250: c_int = 2;
pub const EM2820_BOARD_PINNACLE_USB_2: c_int = 3;
pub const EM2820_BOARD_HAUPPAUGE_WINTV_USB_2: c_int = 4;
pub const EM2820_BOARD_MSI_VOX_USB_2: c_int = 5;
pub const EM2800_BOARD_TERRATEC_CINERGY_200: c_int = 6;
pub const EM2800_BOARD_LEADTEK_WINFAST_USBII: c_int = 7;
pub const EM2800_BOARD_KWORLD_USB2800: c_int = 8;
pub const EM2820_BOARD_PINNACLE_DVC_90: c_int = 9;
pub const EM2880_BOARD_HAUPPAUGE_WINTV_HVR_900: c_int = 10;
pub const EM2880_BOARD_TERRATEC_HYBRID_XS: c_int = 11;
pub const EM2820_BOARD_KWORLD_PVRTV2800RF: c_int = 12;
pub const EM2880_BOARD_TERRATEC_PRODIGY_XS: c_int = 13;
pub const EM2820_BOARD_PROLINK_PLAYTV_USB2: c_int = 14;
pub const EM2800_BOARD_VGEAR_POCKETTV: c_int = 15;
pub const EM2883_BOARD_HAUPPAUGE_WINTV_HVR_950: c_int = 16;
pub const EM2880_BOARD_PINNACLE_PCTV_HD_PRO: c_int = 17;
pub const EM2880_BOARD_HAUPPAUGE_WINTV_HVR_900_R2: c_int = 18;
pub const EM2860_BOARD_SAA711X_REFERENCE_DESIGN: c_int = 19;
pub const EM2880_BOARD_AMD_ATI_TV_WONDER_HD_600: c_int = 20;
pub const EM2800_BOARD_GRABBEEX_USB2800: c_int = 21;
pub const EM2750_BOARD_UNKNOWN: c_int = 22;
pub const EM2750_BOARD_DLCW_130: c_int = 23;
pub const EM2820_BOARD_DLINK_USB_TV: c_int = 24;
pub const EM2820_BOARD_GADMEI_UTV310: c_int = 25;
pub const EM2820_BOARD_HERCULES_SMART_TV_USB2: c_int = 26;
pub const EM2820_BOARD_PINNACLE_USB_2_FM1216ME: c_int = 27;
pub const EM2820_BOARD_LEADTEK_WINFAST_USBII_DELUXE: c_int = 28;
pub const EM2860_BOARD_TVP5150_REFERENCE_DESIGN: c_int = 29;
pub const EM2820_BOARD_VIDEOLOGY_20K14XUSB: c_int = 30;
pub const EM2821_BOARD_USBGEAR_VD204: c_int = 31;
pub const EM2821_BOARD_SUPERCOMP_USB_2: c_int = 32;
pub const EM2860_BOARD_ELGATO_VIDEO_CAPTURE: c_int = 33;
pub const EM2860_BOARD_TERRATEC_HYBRID_XS: c_int = 34;
pub const EM2860_BOARD_TYPHOON_DVD_MAKER: c_int = 35;
pub const EM2860_BOARD_NETGMBH_CAM: c_int = 36;
pub const EM2860_BOARD_GADMEI_UTV330: c_int = 37;
pub const EM2861_BOARD_YAKUMO_MOVIE_MIXER: c_int = 38;
pub const EM2861_BOARD_KWORLD_PVRTV_300U: c_int = 39;
pub const EM2861_BOARD_PLEXTOR_PX_TV100U: c_int = 40;
pub const EM2870_BOARD_KWORLD_350U: c_int = 41;
pub const EM2870_BOARD_KWORLD_355U: c_int = 42;
pub const EM2870_BOARD_TERRATEC_XS: c_int = 43;
pub const EM2870_BOARD_TERRATEC_XS_MT2060: c_int = 44;
pub const EM2870_BOARD_PINNACLE_PCTV_DVB: c_int = 45;
pub const EM2870_BOARD_COMPRO_VIDEOMATE: c_int = 46;
pub const EM2880_BOARD_KWORLD_DVB_305U: c_int = 47;
pub const EM2880_BOARD_KWORLD_DVB_310U: c_int = 48;
pub const EM2880_BOARD_MSI_DIGIVOX_AD: c_int = 49;
pub const EM2880_BOARD_MSI_DIGIVOX_AD_II: c_int = 50;
pub const EM2880_BOARD_TERRATEC_HYBRID_XS_FR: c_int = 51;
pub const EM2881_BOARD_DNT_DA2_HYBRID: c_int = 52;
pub const EM2881_BOARD_PINNACLE_HYBRID_PRO: c_int = 53;
pub const EM2882_BOARD_KWORLD_VS_DVBT: c_int = 54;
pub const EM2882_BOARD_TERRATEC_HYBRID_XS: c_int = 55;
pub const EM2882_BOARD_PINNACLE_HYBRID_PRO_330E: c_int = 56;
pub const EM2883_BOARD_KWORLD_HYBRID_330U: c_int = 57;
pub const EM2820_BOARD_COMPRO_VIDEOMATE_FORYOU: c_int = 58;
pub const EM2874_BOARD_PCTV_HD_MINI_80E: c_int = 59;
pub const EM2883_BOARD_HAUPPAUGE_WINTV_HVR_850: c_int = 60;
pub const EM2820_BOARD_PROLINK_PLAYTV_BOX4_USB2: c_int = 61;
pub const EM2820_BOARD_GADMEI_TVR200: c_int = 62;
pub const EM2860_BOARD_KAIOMY_TVNPC_U2: c_int = 63;
pub const EM2860_BOARD_EASYCAP: c_int = 64;
pub const EM2820_BOARD_IODATA_GVMVP_SZ: c_int = 65;
pub const EM2880_BOARD_EMPIRE_DUAL_TV: c_int = 66;
pub const EM2860_BOARD_TERRATEC_GRABBY: c_int = 67;
pub const EM2860_BOARD_TERRATEC_AV350: c_int = 68;
pub const EM2882_BOARD_KWORLD_ATSC_315U: c_int = 69;
pub const EM2882_BOARD_EVGA_INDTUBE: c_int = 70;
pub const EM2820_BOARD_SILVERCREST_WEBCAM: c_int = 71;
pub const EM2861_BOARD_GADMEI_UTV330PLUS: c_int = 72;
pub const EM2870_BOARD_REDDO_DVB_C_USB_BOX: c_int = 73;
pub const EM2800_BOARD_VC211A: c_int = 74;
pub const EM2882_BOARD_DIKOM_DK300: c_int = 75;
pub const EM2870_BOARD_KWORLD_A340: c_int = 76;
pub const EM2874_BOARD_LEADERSHIP_ISDBT: c_int = 77;
pub const EM28174_BOARD_PCTV_290E: c_int = 78;
pub const EM2884_BOARD_TERRATEC_H5: c_int = 79;
pub const EM28174_BOARD_PCTV_460E: c_int = 80;
pub const EM2884_BOARD_HAUPPAUGE_WINTV_HVR_930C: c_int = 81;
pub const EM2884_BOARD_CINERGY_HTC_STICK: c_int = 82;
pub const EM2860_BOARD_HT_VIDBOX_NW03: c_int = 83;
pub const EM2874_BOARD_MAXMEDIA_UB425_TC: c_int = 84;
pub const EM2884_BOARD_PCTV_510E: c_int = 85;
pub const EM2884_BOARD_PCTV_520E: c_int = 86;
pub const EM2884_BOARD_TERRATEC_HTC_USB_XS: c_int = 87;
pub const EM2884_BOARD_C3TECH_DIGITAL_DUO: c_int = 88;
pub const EM2874_BOARD_DELOCK_61959: c_int = 89;
pub const EM2874_BOARD_KWORLD_UB435Q_V2: c_int = 90;
pub const EM2765_BOARD_SPEEDLINK_VAD_LAPLACE: c_int = 91;
pub const EM28178_BOARD_PCTV_461E: c_int = 92;
pub const EM2874_BOARD_KWORLD_UB435Q_V3: c_int = 93;
pub const EM28178_BOARD_PCTV_292E: c_int = 94;
pub const EM2861_BOARD_LEADTEK_VC100: c_int = 95;
pub const EM28178_BOARD_TERRATEC_T2_STICK_HD: c_int = 96;
pub const EM2884_BOARD_ELGATO_EYETV_HYBRID_2008: c_int = 97;
pub const EM28178_BOARD_PLEX_PX_BCUD: c_int = 98;
pub const EM28174_BOARD_HAUPPAUGE_WINTV_DUALHD_DVB: c_int = 99;
pub const EM28174_BOARD_HAUPPAUGE_WINTV_DUALHD_01595: c_int = 100;
pub const EM2884_BOARD_TERRATEC_H6: c_int = 101;
pub const EM2882_BOARD_ZOLID_HYBRID_TV_STICK: c_int = 102;
pub const EM2861_BOARD_MAGIX_VIDEOWANDLER2: c_int = 103;
pub const EM28178_BOARD_PCTV_461E_V2: c_int = 104;
pub const EM2860_BOARD_MYGICA_IGRABBER: c_int = 105;
pub const EM2874_BOARD_HAUPPAUGE_USB_QUADHD: c_int = 106;
pub const EM2860_BOARD_MYGICA_UTV3: c_int = 107;
pub const EM2828X_BOARD_HAUPPAUGE_USB_LIVE2: c_int = 108;
pub const EM2828X_BOARD_HAUPPAUGE_935_V2: c_int = 109;
pub const EM2828X_BOARD_HAUPPAUGE_955_V2: c_int = 110;
pub const EM2828X_BOARD_HAUPPAUGE_975_V2: c_int = 111;
pub const EM28178_BOARD_PCTV_461E_V3: c_int = 112;
pub const EM28281_BOARD_STARTECH_SVID2USB232: c_int = 113;
// Limits minimum and default number of buffers
pub const EM28XX_MIN_BUF: c_int = 4;
pub const EM28XX_DEF_BUF: c_int = 8;
// Limits the max URB message size
pub const URB_MAX_CTRL_SIZE: c_int = 80;
// Params for validated field
pub const EM28XX_BOARD_NOT_VALIDATED: c_int = 1;
pub const EM28XX_BOARD_VALIDATED: c_int = 0;
// Params for em28xx_cmd() audio
pub const EM28XX_START_AUDIO: c_int = 1;
pub const EM28XX_STOP_AUDIO: c_int = 0;
// maximum number of em28xx boards

// maximum number of frames that can be queued
pub const EM28XX_NUM_FRAMES: c_int = 5;
// number of frames that get used for v4l2_read()
pub const EM28XX_NUM_READ_FRAMES: c_int = 2;
// number of buffers for isoc transfers
pub const EM28XX_NUM_BUFS: c_int = 5;
pub const EM28XX_DVB_NUM_BUFS: c_int = 5;
// max number of I2C buses on em28xx devices
pub const NUM_I2C_BUSES: c_int = 2;
//
// isoc transfers: number of packets for each buffer
// windows requests only 64 packets .. so we better do the same
// this is what I found out for all alternate numbers there!
//
pub const EM28XX_NUM_ISOC_PACKETS: c_int = 64;
pub const EM28XX_DVB_NUM_ISOC_PACKETS: c_int = 64;
//
// bulk transfers: transfer buffer size = packet size * packet multiplier
// USB 2.0 spec says bulk packet size is always 512 bytes
//
pub const EM28XX_BULK_PACKET_MULTIPLIER: c_int = 384;
pub const EM28XX_DVB_BULK_PACKET_MULTIPLIER: c_int = 94;
pub const EM28XX_INTERLACED_DEFAULT: c_int = 1;
// time in msecs to wait for AC97 xfers to finish
pub const EM28XX_AC97_XFER_TIMEOUT: c_int = 100;
// max. number of button state polling addresses
pub const EM28XX_NUM_BUTTON_ADDRESSES_MAX: c_int = 5;
pub const PRIMARY_TS: c_int = 0;
pub const SECONDARY_TS: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum em28xx_mode {
    EM28XX_SUSPEND,
    EM28XX_ANALOG_MODE,
    EM28XX_DIGITAL_MODE,
}

//
// struct em28xx_usb_bufs - Contains URB-related buffer data
//
// @max_pkt_size:	max packet size of isoc transaction
// @num_packets:	number of packets in each buffer
// @num_bufs:		number of allocated urb
// @urb:		urb for isoc/bulk transfers
// @buf:		transfer buffers for isoc/bulk transfer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct em28xx_usb_bufs {
    pub max_pkt_size: c_int,
    pub num_packets: c_int,
    pub num_bufs: c_int,
    pub urb: *mut urb,
    pub buf: *mut c_char,
}

//
// struct em28xx_usb_ctl - Contains URB-related buffer data
//
// @analog_bufs:	isoc/bulk transfer buffers for analog mode
// @digital_bufs:	isoc/bulk transfer buffers for digital mode
// @vid_buf:		Stores already requested video buffers
// @vbi_buf:		Stores already requested VBI buffers
// @urb_data_copy:	copy data from URB
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct em28xx_usb_ctl {
    pub analog_bufs: em28xx_usb_bufs,
    pub digital_bufs: em28xx_usb_bufs,
    pub vid_buf: *mut em28xx_buffer,
    pub vbi_buf: *mut em28xx_buffer,
    pub urb): *mut *mut *mut int (urb_data_copy)(struct em28xx dev, struct urb,
}

//
// struct em28xx_fmt - Struct to enumberate video formats
//
// @fourcc:	v4l2 format id
// @depth:	mean number of bits to represent a pixel
// @reg:	em28xx register value to set it
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct em28xx_fmt {
    pub fourcc: u32,
    pub depth: c_int,
    pub reg: c_int,
}

//
// struct em28xx_buffer- buffer for storing one video frame
//
// @vb:		common v4l buffer stuff
// @list:	List to associate it with the other buffers
// @mem:	pointer to the buffer, as returned by vb2_plane_vaddr()
// @length:	length of the buffer, as returned by vb2_plane_size()
// @top_field:	If non-zero, indicate that the buffer is the top field
// @pos:	Indicate the next position of the buffer to be filled.
// @vb_buf:	pointer to vmalloc memory address in vb
//
// .. note::
//
// in interlaced mode, @pos is reset to zero at the start of each new
// field (not frame !)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct em28xx_buffer {
    pub /: *mut *mut vb2_v4l2_buffer vb; / must be first,
    pub list: list_head,
    pub mem: *mut c_void,
    pub length: c_uint,
    pub top_field: c_int,
    pub pos: c_uint,
    pub vb_buf: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct em28xx_dmaqueue {
    pub active: list_head,
    pub wq: wait_queue_head_t,
}

// inputs
pub const MAX_EM28XX_INPUT: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum enum28xx_itype {
    EM28XX_VMUX_COMPOSITE = 1,
    EM28XX_VMUX_SVIDEO,
    EM28XX_VMUX_TELEVISION,
    EM28XX_RADIO,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum em28xx_ac97_mode {
    EM28XX_NO_AC97 = 0,
    EM28XX_AC97_EM202,
    EM28XX_AC97_SIGMATEL,
    EM28XX_AC97_OTHER,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct em28xx_audio_mode {
    pub ac97: em28xx_ac97_mode,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum em28xx_int_audio_type {
    EM28XX_INT_AUDIO_NONE = 0,
    EM28XX_INT_AUDIO_AC97,
    EM28XX_INT_AUDIO_I2S,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum em28xx_usb_audio_type {
    EM28XX_USB_AUDIO_NONE = 0,
    EM28XX_USB_AUDIO_CLASS,
    EM28XX_USB_AUDIO_VENDOR,
}

//
// enum em28xx_amux - describes the type of audio input used by em28xx
//
// @EM28XX_AMUX_UNUSED:
// Used only on em28xx dev->map field, in order to mark an entry
// as unused.
// @EM28XX_AMUX_VIDEO:
// On devices without AC97, this is the only value that it is currently
// allowed.
// On devices with AC97, it corresponds to the AC97 mixer "Video" control.
// @EM28XX_AMUX_LINE_IN:
// Only for devices with AC97. Corresponds to AC97 mixer "Line In".
// @EM28XX_AMUX_VIDEO2:
// Only for devices with AC97. It means that em28xx should use "Line In"
// And AC97 should use the "Video" mixer control.
// @EM28XX_AMUX_PHONE:
// Only for devices with AC97. Corresponds to AC97 mixer "Phone".
// @EM28XX_AMUX_MIC:
// Only for devices with AC97. Corresponds to AC97 mixer "Mic".
// @EM28XX_AMUX_CD:
// Only for devices with AC97. Corresponds to AC97 mixer "CD".
// @EM28XX_AMUX_AUX:
// Only for devices with AC97. Corresponds to AC97 mixer "Aux".
// @EM28XX_AMUX_PCM_OUT:
// Only for devices with AC97. Corresponds to AC97 mixer "PCM out".
//
// The em28xx chip itself has only two audio inputs: tuner and line in.
// On almost all devices, only the tuner input is used.
//
// However, on most devices, an auxiliary AC97 codec device is used,
// usually connected to the em28xx tuner input (except for
// @EM28XX_AMUX_LINE_IN).
//
// The AC97 device typically have several different inputs and outputs.
// The exact number and description depends on their model.
//
// It is possible to AC97 to mixer more than one different entries at the
// same time, via the alsa mux.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum em28xx_amux {
    EM28XX_AMUX_UNUSED = -1,
    EM28XX_AMUX_VIDEO = 0,
    EM28XX_AMUX_LINE_IN,

// Some less-common mixer setups
    EM28XX_AMUX_VIDEO2,
    EM28XX_AMUX_PHONE,
    EM28XX_AMUX_MIC,
    EM28XX_AMUX_CD,
    EM28XX_AMUX_AUX,
    EM28XX_AMUX_PCM_OUT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum em28xx_aout {
// AC97 outputs
    EM28XX_AOUT_MASTER = BIT(0),
    EM28XX_AOUT_LINE   = BIT(1),
    EM28XX_AOUT_MONO   = BIT(2),
    EM28XX_AOUT_LFE    = BIT(3),
    EM28XX_AOUT_SURR   = BIT(4),

// PCM IN Mixer - used by AC97_RECORD_SELECT register
    EM28XX_AOUT_PCM_IN = BIT(7),

// Bits 10-8 are used to indicate the PCM IN record select
    EM28XX_AOUT_PCM_MIC_PCM = 0 << 8,
    EM28XX_AOUT_PCM_CD	= 1 << 8,
    EM28XX_AOUT_PCM_VIDEO	= 2 << 8,
    EM28XX_AOUT_PCM_AUX	= 3 << 8,
    EM28XX_AOUT_PCM_LINE	= 4 << 8,
    EM28XX_AOUT_PCM_STEREO	= 5 << 8,
    EM28XX_AOUT_PCM_MONO	= 6 << 8,
    EM28XX_AOUT_PCM_PHONE	= 7 << 8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct em28xx_reg_seq {
    pub reg: c_int,
    pub mask: unsigned char val,,
    pub sleep: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct em28xx_input {
    pub type: enum28xx_itype,
    pub vmux: c_uint,
    pub amux: em28xx_amux,
    pub aout: em28xx_aout,
    pub gpio: *const em28xx_reg_seq,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum em28xx_decoder {
    EM28XX_NODECODER = 0,
    EM28XX_TVP5150,
    EM28XX_SAA711X,
    EM28XX_BUILTIN,
}

// Built in decoder capture options
pub const EM2828X_COMPOSITE: c_int = 0;
pub const EM2828X_SVIDEO: c_int = 1;
pub const EM2828X_TELEVISION: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum em28xx_sensor {
    EM28XX_NOSENSOR = 0,
    EM28XX_MT9V011,
    EM28XX_MT9M001,
    EM28XX_MT9M111,
    EM28XX_OV2640,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum em28xx_adecoder {
    EM28XX_NOADECODER = 0,
    EM28XX_TVAUDIO,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum em28xx_led_role {
    EM28XX_LED_ANALOG_CAPTURING = 0,
    EM28XX_LED_DIGITAL_CAPTURING,
    EM28XX_LED_DIGITAL_CAPTURING_TS2,
    EM28XX_LED_ILLUMINATION,
    EM28XX_NUM_LED_ROLES, /* must be the last */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct em28xx_led {
    pub role: em28xx_led_role,
    pub gpio_reg: u8,
    pub gpio_mask: u8,
    pub inverted: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum em28xx_button_role {
    EM28XX_BUTTON_SNAPSHOT = 0,
    EM28XX_BUTTON_ILLUMINATION,
    EM28XX_NUM_BUTTON_ROLES, /* must be the last */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct em28xx_button {
    pub role: em28xx_button_role,
    pub reg_r: u8,
    pub reg_clearing: u8,
    pub mask: u8,
    pub inverted: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum em2828x_media_pads {
    EM2828X_PAD_INPUT,
    EM2828X_PAD_VID_OUT,
    EM2828X_NUM_PADS
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct em28xx_board {
    pub name: *mut c_char,
    pub vchannels: c_int,
    pub tuner_type: c_int,
    pub tuner_addr: c_int,
    pub /: *mut *mut unsigned int def_i2c_bus; / Default I2C bus,
// i2c flags
    pub tda9887_conf: c_uint,
// GPIO sequences
    pub dvb_gpio: *const em28xx_reg_seq,
    pub suspend_gpio: *const em28xx_reg_seq,
    pub tuner_gpio: *const em28xx_reg_seq,
    pub mute_gpio: *const em28xx_reg_seq,
    pub is_em2800:1: c_uint,
    pub has_msp34xx:1: c_uint,
    pub mts_firmware:1: c_uint,
    pub max_range_640_480:1: c_uint,
    pub has_dvb:1: c_uint,
    pub has_dual_ts:1: c_uint,
    pub is_webcam:1: c_uint,
    pub valid:1: c_uint,
    pub has_ir_i2c:1: c_uint,
    pub i2c_speed: unsigned char xclk,,
    pub radio_addr: c_uchar,
    pub tvaudio_addr: c_ushort,
    pub decoder: em28xx_decoder,
    pub adecoder: em28xx_adecoder,
    pub input: [em28xx_input; MAX_EM28XX_INPUT],
    pub radio: em28xx_input,
    pub ir_codes: *mut c_char,
// LEDs that need to be controlled explicitly
    pub leds: *mut em28xx_led,
// Buttons
    pub buttons: *const em28xx_button,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct em28xx_eeprom {
    pub /: *mut *mut u8 id[4]; / 1a eb 67 95,
    pub vendor_ID: __le16,
    pub product_ID: __le16,
    pub chip_conf: __le16,
    pub board_conf: __le16,
    pub string3: __le16 string1, string2,,
    pub string_idx_table: u8,
}

pub const EM28XX_CAPTURE_STREAM_EN: c_int = 1;
// em28xx extensions
pub const EM28XX_AUDIO: c_uint = 0x10;
pub const EM28XX_DVB: c_uint = 0x20;
pub const EM28XX_RC: c_uint = 0x30;
pub const EM28XX_V4L2: c_uint = 0x40;
// em28xx resource types (used for res_get/res_lock etc
pub const EM28XX_RESOURCE_VIDEO: c_uint = 0x01;
pub const EM28XX_RESOURCE_VBI: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct em28xx_v4l2 {
    pub dev: *mut em28xx,
    pub v4l2_dev: v4l2_device,
    pub ctrl_handler: v4l2_ctrl_handler,
    pub vdev: video_device,
    pub vbi_dev: video_device,
    pub radio_dev: video_device,
// Videobuf2
    pub vb_vidq: vb2_queue,
    pub vb_vbiq: vb2_queue,
    pub /: *mut *mut mutex vb_queue_lock; / Protects vb_vidq,
    pub /: *mut *mut mutex vb_vbi_queue_lock; / Protects vb_vbiq,
    pub vinmode: u8,
    pub vinctl: u8,
// Camera specific fields
    pub sensor_xres: c_int,
    pub sensor_yres: c_int,
    pub sensor_xtal: c_int,
    pub /: *mut *mut int streaming_users; / number of actively streaming users,
    pub /: *mut *mut u32 frequency; / selected tuner frequency,
    pub format: *mut em28xx_fmt,
    pub /: *mut *mut v4l2_std_id norm; / selected tv norm,
// Progressive/interlaced mode
    pub progressive: bool,
    pub /: *mut *mut int interlaced_fieldmode; / 1=interlaced fields, 0=just top fields,
// FIXME: everything else than interlaced_fieldmode=1 doesn't work
// Frame properties
    pub /: *mut *mut int width; / current frame width,
    pub /: *mut *mut int height; / current frame height,
    pub /: *mut *mut unsigned int hscale; / horizontal scale factor (see datasheet),
    pub /: *mut *mut unsigned int vscale; / vertical scale factor (see datasheet),
    pub vbi_width: c_uint,
    pub /: *mut *mut unsigned int vbi_height; / lines per field,
// Capture state tracking
    pub capture_type: c_int,
    pub top_field: bool,
    pub vbi_read: c_int,
    pub field_count: c_uint,

    pub vbi_pad: media_pad video_pad,,
    pub decoder_pads: [media_pad; EM2828X_NUM_PADS],
    pub decoder: *mut media_entity,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct em28xx_audio {
    pub name: [c_char; 50],
    pub num_urb: c_uint,
    pub transfer_buffer: *mut c_char,
    pub urb: *mut urb,
    pub udev: *mut usb_device,
    pub capture_transfer_done: c_uint,
    pub capture_pcm_substream: *mut snd_pcm_substream,
    pub hwptr_done_capture: c_uint,
    pub sndcard: *mut snd_card,
    pub period: usize,
    pub users: c_int,
    pub /: *mut *mut spinlock_t slock; / Protects struct em28xx_audio,
// Controls streaming
    pub /: *mut *mut work_wq_trigger; / trigger to start/stop audio,
    pub /: *mut *mut atomic_t stream_started; / stream should be running if true,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum em28xx_i2c_algo_type {
    EM28XX_I2C_ALGO_EM28XX = 0,
    EM28XX_I2C_ALGO_EM2800,
    EM28XX_I2C_ALGO_EM25XX_BUS_B,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct em28xx_i2c_bus {
    pub dev: *mut em28xx,
    pub bus: c_uint,
    pub algo_type: em28xx_i2c_algo_type,
}

// main device struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct em28xx {
    pub ref: kref,
// Sub-module data
    pub v4l2: *mut em28xx_v4l2,
    pub dvb: *mut em28xx_dvb,
    pub adev: em28xx_audio,
    pub ir: *mut em28xx_IR,
// generic device properties
    pub struct: int model; // index in the device_data,
    pub device: int devno; // marks the number of this,
    pub chip_id: em28xx_chip_id,
    pub bridge: unsigned int is_em25xx:1; // em25xx/em276x/7x/8x family,
    pub disconnected: unsigned int disconnected:1; // device has been,
    pub has_video:1: c_uint,
    pub is_audio_only:1: c_uint,
    pub is_webcam:1: c_uint,
    pub has_msp34xx:1: c_uint,
    pub i2c_speed:2: c_uint,
    pub int_audio_type: em28xx_int_audio_type,
    pub usb_audio_type: em28xx_usb_audio_type,
    pub name: [c_uchar; 32],
    pub board: em28xx_board,
    pub specific: em28xx_sensor em28xx_sensor; // camera,
// Some older em28xx chips needs a waiting time after writing
    pub wait_after_write: c_uint,
    pub devlist: list_head,
    pub stream: u32 i2s_speed; // I2S speed for audio digital,
    pub audio_mode: em28xx_audio_mode,
    pub tuner: int tuner_type; // type of the,
// i2c i/o
    pub i2c_adap: [i2c_adapter; NUM_I2C_BUSES],
    pub i2c_client: [i2c_client; NUM_I2C_BUSES],
    pub i2c_bus: [em28xx_i2c_bus; NUM_I2C_BUSES],
    pub eeprom_addrwidth_16bit:1: c_uchar,
    pub bus: unsigned int def_i2c_bus; // Default I2C,
    pub bus: unsigned int cur_i2c_bus; // Current I2C,
    pub i2c_bus_lock: rt_mutex,
// video for linux
    pub input: unsigned int ctl_input; // selected,
    pub input: unsigned int ctl_ainput;// selected audio,
    pub output: unsigned int ctl_aoutput;// selected audio,
    pub amux_map: [em28xx_amux; MAX_EM28XX_INPUT],
    pub mute: c_int,
    pub volume: c_int,
    pub ID: unsigned long hash; // eeprom hash - for boards with generic,
    pub -: unsigned long i2c_hash; // i2c devicelist hash,
// for boards with generic ID
    pub request_module_wk: work_struct,
// locks
    pub /: *mut *mut mutex lock; / protects em28xx struct,
    pub /: *mut *mut mutex ctrl_urb_lock; / protects urb_buf,
// resources in use
    pub resources: c_uint,
// eeprom content
    pub eedata: *mut u8,
    pub eedata_len: u16,
// Isoc control struct
    pub vidq: em28xx_dmaqueue,
    pub vbiq: em28xx_dmaqueue,
    pub usb_ctl: em28xx_usb_ctl,
    pub /: *mut *mut spinlock_t slock; / Protects em28xx video/vbi/dvb IRQ stream data,
// usb transfer
    pub interface: *mut *mut usb_interface intf; // the usb,
    pub interface: u8 ifnum; // number of the assigned usb,
    pub analog: u8 analog_ep_isoc; // address of isoc endpoint for,
    pub analog: u8 analog_ep_bulk; // address of bulk endpoint for,
    pub TS2: u8 dvb_ep_isoc_ts2; // address of isoc endpoint for DVB,
    pub TS2: u8 dvb_ep_bulk_ts2; // address of bulk endpoint for DVB,
    pub DVB: u8 dvb_ep_isoc; // address of isoc endpoint for,
    pub DVB: u8 dvb_ep_bulk; // address of bulk endpoint for,
    pub setting: int alt; // alternate,
    pub alt: int max_pkt_size; // max packet size of the selected ep at,
    pub for: int packet_multiplier; // multiplier for wMaxPacketSize, used,
// URB buffer size definition
    pub settings: int num_alt; // number of alternative,
    pub wMaxPacketSize: *mut *mut unsigned int alt_max_pkt_size_isoc; // array of isoc,
    pub isoc: unsigned int analog_xfer_bulk:1; // use bulk instead of,
// transfers for analog
    pub transfers: int dvb_alt_isoc; // alternate setting for DVB isoc,
    pub the: unsigned int dvb_max_pkt_size_isoc; // isoc max packet size of,
// selected DVB ep at dvb_alt
    pub the: unsigned int dvb_max_pkt_size_isoc_ts2; // isoc max packet size of,
// selected DVB ep at dvb_alt
    pub isoc: unsigned int dvb_xfer_bulk:1; // use bulk instead of,
// transfers for DVB
    pub buffer: char urb_buf[URB_MAX_CTRL_SIZE]; // urb control msg,
// helper funcs that call usb_control_msg
    pub len): *mut *mut char buf, int,
    pub reg): *mut *mut *mut int (em28xx_read_reg)(struct em28xx dev, u16,
    pub len): *mut *mut char buf, int,
    pub len): *mut *mut char buf, int,
    pub reg): *mut *mut *mut int (em28xx_read_reg_req)(struct em28xx dev, u8 req, u16,
    pub freq): *mut *mut *mut int (em28xx_set_analog_freq)(struct em28xx dev, u32,
    pub mode: em28xx_mode,
// Button state polling
    pub buttons_query_work: delayed_work,
    pub button_polling_addresses: [u8; EM28XX_NUM_BUTTON_ADDRESSES_MAX],
    pub button_polling_last_values: [u8; EM28XX_NUM_BUTTON_ADDRESSES_MAX],
    pub num_button_polling_addresses: u8,
    pub [ms]: u16 button_polling_interval; //,
// Snapshot button input device
    pub dev: char snapshot_button_path[30]; // path of the input,
    pub sbutton_input_dev: *mut input_dev,
    pub analog_xfer_mode: c_int,

    pub media_dev: *mut media_device,
    pub input_ent: [media_entity; MAX_EM28XX_INPUT],
    pub input_pad: [media_pad; MAX_EM28XX_INPUT],
    pub dev_next: *mut em28xx,
    pub ts: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct em28xx_ops {
    pub next: list_head,
    pub name: *mut c_char,
    pub id: c_int,
    pub dev): *mut *mut int (init)(struct em28xx,
    pub dev): *mut *mut int (fini)(struct em28xx,
    pub dev): *mut *mut int (suspend)(struct em28xx,
    pub dev): *mut *mut int (resume)(struct em28xx,
}

// Provided by em28xx-i2c.c
extern "C" {
    pub fn em28xx_do_i2c_scan(dev: *mut em28xx, bus: c_uint);
}
extern "C" {
    pub fn em28xx_i2c_unregister(dev: *mut em28xx, bus: c_uint) -> c_int;
}
// Provided by em28xx-core.c
extern "C" {
    pub fn em28xx_read_reg_req(dev: *mut em28xx, req: u8, reg: u16) -> c_int;
}
extern "C" {
    pub fn em28xx_read_reg(dev: *mut em28xx, reg: u16) -> c_int;
}
extern "C" {
    pub fn em28xx_write_regs(dev: *mut em28xx, reg: u16, buf: *mut c_char, len: c_int) -> c_int;
}
extern "C" {
    pub fn em28xx_write_reg(dev: *mut em28xx, reg: u16, val: u8) -> c_int;
}
extern "C" {
    pub fn em28xx_toggle_reg_bits(dev: *mut em28xx, reg: u16, bitmask: u8) -> c_int;
}
extern "C" {
    pub fn em28xx_read_ac97(dev: *mut em28xx, reg: u8) -> c_int;
}
extern "C" {
    pub fn em28xx_write_ac97(dev: *mut em28xx, reg: u8, val: u16) -> c_int;
}
extern "C" {
    pub fn em28xx_audio_analog_set(dev: *mut em28xx) -> c_int;
}
extern "C" {
    pub fn em28xx_audio_setup(dev: *mut em28xx) -> c_int;
}
extern "C" {
    pub fn em2828X_decoder_vmux(dev: *mut em28xx, vin: c_uint);
}
extern "C" {
    pub fn em28xx_capture_start(dev: *mut em28xx, start: c_int) -> c_int;
}
extern "C" {
    pub fn em28xx_uninit_usb_xfer(dev: *mut em28xx, mode: em28xx_mode);
}
extern "C" {
    pub fn em28xx_stop_urbs(dev: *mut em28xx);
}
extern "C" {
    pub fn em28xx_set_mode(dev: *mut em28xx, set_mode: em28xx_mode) -> c_int;
}
extern "C" {
    pub fn em28xx_gpio_set(dev: *mut em28xx, gpio: *const em28xx_reg_seq) -> c_int;
}
extern "C" {
    pub fn em28xx_register_extension(dev: *mut em28xx_ops) -> c_int;
}
extern "C" {
    pub fn em28xx_unregister_extension(dev: *mut em28xx_ops);
}
extern "C" {
    pub fn em28xx_init_extension(dev: *mut em28xx);
}
extern "C" {
    pub fn em28xx_close_extension(dev: *mut em28xx);
}
extern "C" {
    pub fn em28xx_suspend_extension(dev: *mut em28xx) -> c_int;
}
extern "C" {
    pub fn em28xx_resume_extension(dev: *mut em28xx) -> c_int;
}
// Provided by em28xx-cards.c
extern "C" {
    pub fn em28xx_tuner_callback(ptr: *mut c_void, component: c_int, command: c_int, arg: c_int) -> c_int;
}
extern "C" {
    pub fn em28xx_setup_xc3028(dev: *mut em28xx, ctl: *mut xc2028_ctrl);
}
extern "C" {
    pub fn em28xx_free_device(ref: *mut kref);
}
// Provided by em28xx-camera.c
extern "C" {
    pub fn em28xx_detect_sensor(dev: *mut em28xx) -> c_int;
}
extern "C" {
    pub fn em28xx_init_camera(dev: *mut em28xx) -> c_int;
}
