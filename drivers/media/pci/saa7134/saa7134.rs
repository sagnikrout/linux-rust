//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/saa7134/saa7134.h
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
// v4l2 device driver for philips saa7134 based TV cards
//
// (c) 2001,02 Gerd Knorr <kraxel@bytesex.org>
//

// -----------------------------------------------------------
// enums
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum saa7134_tvaudio_mode {
    TVAUDIO_FM_MONO       = 1,
    TVAUDIO_FM_BG_STEREO  = 2,
    TVAUDIO_FM_SAT_STEREO = 3,
    TVAUDIO_FM_K_STEREO   = 4,
    TVAUDIO_NICAM_AM      = 5,
    TVAUDIO_NICAM_FM      = 6,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum saa7134_audio_in {
    TV    = 1,
    LINE1 = 2,
    LINE2 = 3,
    LINE2_LEFT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum saa7134_video_out {
    CCIR656 = 1,
}

// -----------------------------------------------------------
// static data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct saa7134_tvnorm {
    pub name: *mut c_char,
    pub id: v4l2_std_id,
// video decoder
    pub sync_control: c_uint,
    pub luma_control: c_uint,
    pub chroma_ctrl1: c_uint,
    pub chroma_gain: c_uint,
    pub chroma_ctrl2: c_uint,
    pub vgate_misc: c_uint,
// video scaler
    pub h_start: c_uint,
    pub h_stop: c_uint,
    pub video_v_start: c_uint,
    pub video_v_stop: c_uint,
    pub vbi_v_start_0: c_uint,
    pub vbi_v_stop_0: c_uint,
    pub src_timing: c_uint,
    pub vbi_v_start_1: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct saa7134_tvaudio {
    pub name: *mut c_char,
    pub std: v4l2_std_id,
    pub mode: saa7134_tvaudio_mode,
    pub carr1: c_int,
    pub carr2: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct saa7134_format {
    pub fourcc: c_uint,
    pub depth: c_uint,
    pub pm: c_uint,
    pub /: *mut *mut unsigned int vshift; / vertical downsampling (for planar yuv),
    pub /: *mut *mut unsigned int hshift; / horizontal downsampling (for planar yuv),
    pub bswap:1: c_uint,
    pub wswap:1: c_uint,
    pub yuv:1: c_uint,
    pub planar:1: c_uint,
    pub uvswap:1: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct saa7134_card_ir {
    pub dev: *mut rc_dev,
    pub phys: [c_char; 32],
    pub polling: u32,
    pub last_gpio: u32,
    pub mask_keyup: u32 mask_keycode, mask_keydown,,
    pub running: bool,
    pub timer: timer_list,
// IR core raw decoding
    pub raw_decode: u32,
}

// -----------------------------------------------------------
// card configuration

pub const SAA7134_BOARD_UNKNOWN: c_int = 0;
pub const SAA7134_BOARD_PROTEUS_PRO: c_int = 1;
pub const SAA7134_BOARD_FLYVIDEO3000: c_int = 2;
pub const SAA7134_BOARD_FLYVIDEO2000: c_int = 3;
pub const SAA7134_BOARD_EMPRESS: c_int = 4;
pub const SAA7134_BOARD_MONSTERTV: c_int = 5;
pub const SAA7134_BOARD_MD9717: c_int = 6;
pub const SAA7134_BOARD_TVSTATION_RDS: c_int = 7;
pub const SAA7134_BOARD_CINERGY400: c_int = 8;
pub const SAA7134_BOARD_MD5044: c_int = 9;
pub const SAA7134_BOARD_KWORLD: c_int = 10;
pub const SAA7134_BOARD_CINERGY600: c_int = 11;
pub const SAA7134_BOARD_MD7134: c_int = 12;
pub const SAA7134_BOARD_TYPHOON_90031: c_int = 13;
pub const SAA7134_BOARD_ELSA: c_int = 14;
pub const SAA7134_BOARD_ELSA_500TV: c_int = 15;
pub const SAA7134_BOARD_ASUSTeK_TVFM7134: c_int = 16;
pub const SAA7134_BOARD_VA1000POWER: c_int = 17;
pub const SAA7134_BOARD_BMK_MPEX_NOTUNER: c_int = 18;
pub const SAA7134_BOARD_VIDEOMATE_TV: c_int = 19;
pub const SAA7134_BOARD_CRONOS_PLUS: c_int = 20;
pub const SAA7134_BOARD_10MOONSTVMASTER: c_int = 21;
pub const SAA7134_BOARD_MD2819: c_int = 22;
pub const SAA7134_BOARD_BMK_MPEX_TUNER: c_int = 23;
pub const SAA7134_BOARD_TVSTATION_DVR: c_int = 24;
pub const SAA7134_BOARD_ASUSTEK_TVFM7133: c_int = 25;
pub const SAA7134_BOARD_PINNACLE_PCTV_STEREO: c_int = 26;
pub const SAA7134_BOARD_MANLI_MTV002: c_int = 27;
pub const SAA7134_BOARD_MANLI_MTV001: c_int = 28;
pub const SAA7134_BOARD_TG3000TV: c_int = 29;
pub const SAA7134_BOARD_ECS_TVP3XP: c_int = 30;
pub const SAA7134_BOARD_ECS_TVP3XP_4CB5: c_int = 31;
pub const SAA7134_BOARD_AVACSSMARTTV: c_int = 32;
pub const SAA7134_BOARD_AVERMEDIA_DVD_EZMAKER: c_int = 33;
pub const SAA7134_BOARD_NOVAC_PRIMETV7133: c_int = 34;
pub const SAA7134_BOARD_AVERMEDIA_STUDIO_305: c_int = 35;
pub const SAA7134_BOARD_UPMOST_PURPLE_TV: c_int = 36;
pub const SAA7134_BOARD_ITEMS_MTV005: c_int = 37;
pub const SAA7134_BOARD_CINERGY200: c_int = 38;
pub const SAA7134_BOARD_FLYTVPLATINUM_MINI: c_int = 39;
pub const SAA7134_BOARD_VIDEOMATE_TV_PVR: c_int = 40;
pub const SAA7134_BOARD_VIDEOMATE_TV_GOLD_PLUS: c_int = 41;
pub const SAA7134_BOARD_SABRENT_SBTTVFM: c_int = 42;
pub const SAA7134_BOARD_ZOLID_XPERT_TV7134: c_int = 43;
pub const SAA7134_BOARD_EMPIRE_PCI_TV_RADIO_LE: c_int = 44;
pub const SAA7134_BOARD_AVERMEDIA_STUDIO_307: c_int = 45;
pub const SAA7134_BOARD_AVERMEDIA_CARDBUS: c_int = 46;
pub const SAA7134_BOARD_CINERGY400_CARDBUS: c_int = 47;
pub const SAA7134_BOARD_CINERGY600_MK3: c_int = 48;
pub const SAA7134_BOARD_VIDEOMATE_GOLD_PLUS: c_int = 49;
pub const SAA7134_BOARD_PINNACLE_300I_DVBT_PAL: c_int = 50;
pub const SAA7134_BOARD_PROVIDEO_PV952: c_int = 51;
pub const SAA7134_BOARD_AVERMEDIA_305: c_int = 52;
pub const SAA7134_BOARD_ASUSTeK_TVFM7135: c_int = 53;
pub const SAA7134_BOARD_FLYTVPLATINUM_FM: c_int = 54;
pub const SAA7134_BOARD_FLYDVBTDUO: c_int = 55;
pub const SAA7134_BOARD_AVERMEDIA_307: c_int = 56;
pub const SAA7134_BOARD_AVERMEDIA_GO_007_FM: c_int = 57;
pub const SAA7134_BOARD_ADS_INSTANT_TV: c_int = 58;
pub const SAA7134_BOARD_KWORLD_VSTREAM_XPERT: c_int = 59;
pub const SAA7134_BOARD_FLYDVBT_DUO_CARDBUS: c_int = 60;
pub const SAA7134_BOARD_PHILIPS_TOUGH: c_int = 61;
pub const SAA7134_BOARD_VIDEOMATE_TV_GOLD_PLUSII: c_int = 62;
pub const SAA7134_BOARD_KWORLD_XPERT: c_int = 63;
pub const SAA7134_BOARD_FLYTV_DIGIMATRIX: c_int = 64;
pub const SAA7134_BOARD_KWORLD_TERMINATOR: c_int = 65;
pub const SAA7134_BOARD_YUAN_TUN900: c_int = 66;
pub const SAA7134_BOARD_BEHOLD_409FM: c_int = 67;
pub const SAA7134_BOARD_GOTVIEW_7135: c_int = 68;
pub const SAA7134_BOARD_PHILIPS_EUROPA: c_int = 69;
pub const SAA7134_BOARD_VIDEOMATE_DVBT_300: c_int = 70;
pub const SAA7134_BOARD_VIDEOMATE_DVBT_200: c_int = 71;
pub const SAA7134_BOARD_RTD_VFG7350: c_int = 72;
pub const SAA7134_BOARD_RTD_VFG7330: c_int = 73;
pub const SAA7134_BOARD_FLYTVPLATINUM_MINI2: c_int = 74;
pub const SAA7134_BOARD_AVERMEDIA_AVERTVHD_A180: c_int = 75;
pub const SAA7134_BOARD_MONSTERTV_MOBILE: c_int = 76;
pub const SAA7134_BOARD_PINNACLE_PCTV_110i: c_int = 77;
pub const SAA7134_BOARD_ASUSTeK_P7131_DUAL: c_int = 78;
pub const SAA7134_BOARD_SEDNA_PC_TV_CARDBUS: c_int = 79;
pub const SAA7134_BOARD_ASUSTEK_DIGIMATRIX_TV: c_int = 80;
pub const SAA7134_BOARD_PHILIPS_TIGER: c_int = 81;
pub const SAA7134_BOARD_MSI_TVATANYWHERE_PLUS: c_int = 82;
pub const SAA7134_BOARD_CINERGY250PCI: c_int = 83;
pub const SAA7134_BOARD_FLYDVB_TRIO: c_int = 84;
pub const SAA7134_BOARD_AVERMEDIA_777: c_int = 85;
pub const SAA7134_BOARD_FLYDVBT_LR301: c_int = 86;
pub const SAA7134_BOARD_ADS_DUO_CARDBUS_PTV331: c_int = 87;
pub const SAA7134_BOARD_TEVION_DVBT_220RF: c_int = 88;
pub const SAA7134_BOARD_ELSA_700TV: c_int = 89;
pub const SAA7134_BOARD_KWORLD_ATSC110: c_int = 90;
pub const SAA7134_BOARD_AVERMEDIA_A169_B: c_int = 91;
pub const SAA7134_BOARD_AVERMEDIA_A169_B1: c_int = 92;
pub const SAA7134_BOARD_MD7134_BRIDGE_2: c_int = 93;
pub const SAA7134_BOARD_FLYDVBT_HYBRID_CARDBUS: c_int = 94;
pub const SAA7134_BOARD_FLYVIDEO3000_NTSC: c_int = 95;
pub const SAA7134_BOARD_MEDION_MD8800_QUADRO: c_int = 96;
pub const SAA7134_BOARD_FLYDVBS_LR300: c_int = 97;
pub const SAA7134_BOARD_PROTEUS_2309: c_int = 98;
pub const SAA7134_BOARD_AVERMEDIA_A16AR: c_int = 99;
pub const SAA7134_BOARD_ASUS_EUROPA2_HYBRID: c_int = 100;
pub const SAA7134_BOARD_PINNACLE_PCTV_310i: c_int = 101;
pub const SAA7134_BOARD_AVERMEDIA_STUDIO_507: c_int = 102;
pub const SAA7134_BOARD_VIDEOMATE_DVBT_200A: c_int = 103;
pub const SAA7134_BOARD_HAUPPAUGE_HVR1110: c_int = 104;
pub const SAA7134_BOARD_CINERGY_HT_PCMCIA: c_int = 105;
pub const SAA7134_BOARD_ENCORE_ENLTV: c_int = 106;
pub const SAA7134_BOARD_ENCORE_ENLTV_FM: c_int = 107;
pub const SAA7134_BOARD_CINERGY_HT_PCI: c_int = 108;
pub const SAA7134_BOARD_PHILIPS_TIGER_S: c_int = 109;
pub const SAA7134_BOARD_AVERMEDIA_M102: c_int = 110;
pub const SAA7134_BOARD_ASUS_P7131_4871: c_int = 111;
pub const SAA7134_BOARD_ASUSTeK_P7131_HYBRID_LNA: c_int = 112;
pub const SAA7134_BOARD_ECS_TVP3XP_4CB6: c_int = 113;
pub const SAA7134_BOARD_KWORLD_DVBT_210: c_int = 114;
pub const SAA7134_BOARD_SABRENT_TV_PCB05: c_int = 115;
pub const SAA7134_BOARD_10MOONSTVMASTER3: c_int = 116;
pub const SAA7134_BOARD_AVERMEDIA_SUPER_007: c_int = 117;
pub const SAA7134_BOARD_BEHOLD_401: c_int = 118;
pub const SAA7134_BOARD_BEHOLD_403: c_int = 119;
pub const SAA7134_BOARD_BEHOLD_403FM: c_int = 120;
pub const SAA7134_BOARD_BEHOLD_405: c_int = 121;
pub const SAA7134_BOARD_BEHOLD_405FM: c_int = 122;
pub const SAA7134_BOARD_BEHOLD_407: c_int = 123;
pub const SAA7134_BOARD_BEHOLD_407FM: c_int = 124;
pub const SAA7134_BOARD_BEHOLD_409: c_int = 125;
pub const SAA7134_BOARD_BEHOLD_505FM: c_int = 126;
pub const SAA7134_BOARD_BEHOLD_507_9FM: c_int = 127;
pub const SAA7134_BOARD_BEHOLD_COLUMBUS_TVFM: c_int = 128;
pub const SAA7134_BOARD_BEHOLD_607FM_MK3: c_int = 129;
pub const SAA7134_BOARD_BEHOLD_M6: c_int = 130;
pub const SAA7134_BOARD_TWINHAN_DTV_DVB_3056: c_int = 131;
pub const SAA7134_BOARD_GENIUS_TVGO_A11MCE: c_int = 132;
pub const SAA7134_BOARD_PHILIPS_SNAKE: c_int = 133;
pub const SAA7134_BOARD_CREATIX_CTX953: c_int = 134;
pub const SAA7134_BOARD_MSI_TVANYWHERE_AD11: c_int = 135;
pub const SAA7134_BOARD_AVERMEDIA_CARDBUS_506: c_int = 136;
pub const SAA7134_BOARD_AVERMEDIA_A16D: c_int = 137;
pub const SAA7134_BOARD_AVERMEDIA_M115: c_int = 138;
pub const SAA7134_BOARD_VIDEOMATE_T750: c_int = 139;
pub const SAA7134_BOARD_AVERMEDIA_A700_PRO: c_int = 140;
pub const SAA7134_BOARD_AVERMEDIA_A700_HYBRID: c_int = 141;
pub const SAA7134_BOARD_BEHOLD_H6: c_int = 142;
pub const SAA7134_BOARD_BEHOLD_M63: c_int = 143;
pub const SAA7134_BOARD_BEHOLD_M6_EXTRA: c_int = 144;
pub const SAA7134_BOARD_AVERMEDIA_M103: c_int = 145;
pub const SAA7134_BOARD_ASUSTeK_P7131_ANALOG: c_int = 146;
pub const SAA7134_BOARD_ASUSTeK_TIGER_3IN1: c_int = 147;
pub const SAA7134_BOARD_ENCORE_ENLTV_FM53: c_int = 148;
pub const SAA7134_BOARD_AVERMEDIA_M135A: c_int = 149;
pub const SAA7134_BOARD_REAL_ANGEL_220: c_int = 150;
pub const SAA7134_BOARD_ADS_INSTANT_HDTV_PCI: c_int = 151;
pub const SAA7134_BOARD_ASUSTeK_TIGER: c_int = 152;
pub const SAA7134_BOARD_KWORLD_PLUS_TV_ANALOG: c_int = 153;
pub const SAA7134_BOARD_AVERMEDIA_GO_007_FM_PLUS: c_int = 154;
pub const SAA7134_BOARD_HAUPPAUGE_HVR1150: c_int = 155;
pub const SAA7134_BOARD_HAUPPAUGE_HVR1120: c_int = 156;
pub const SAA7134_BOARD_AVERMEDIA_STUDIO_507UA: c_int = 157;
pub const SAA7134_BOARD_AVERMEDIA_CARDBUS_501: c_int = 158;
pub const SAA7134_BOARD_BEHOLD_505RDS_MK5: c_int = 159;
pub const SAA7134_BOARD_BEHOLD_507RDS_MK3: c_int = 160;
pub const SAA7134_BOARD_BEHOLD_507RDS_MK5: c_int = 161;
pub const SAA7134_BOARD_BEHOLD_607FM_MK5: c_int = 162;
pub const SAA7134_BOARD_BEHOLD_609FM_MK3: c_int = 163;
pub const SAA7134_BOARD_BEHOLD_609FM_MK5: c_int = 164;
pub const SAA7134_BOARD_BEHOLD_607RDS_MK3: c_int = 165;
pub const SAA7134_BOARD_BEHOLD_607RDS_MK5: c_int = 166;
pub const SAA7134_BOARD_BEHOLD_609RDS_MK3: c_int = 167;
pub const SAA7134_BOARD_BEHOLD_609RDS_MK5: c_int = 168;
pub const SAA7134_BOARD_VIDEOMATE_S350: c_int = 169;
pub const SAA7134_BOARD_AVERMEDIA_STUDIO_505: c_int = 170;
pub const SAA7134_BOARD_BEHOLD_X7: c_int = 171;
pub const SAA7134_BOARD_ROVERMEDIA_LINK_PRO_FM: c_int = 172;
pub const SAA7134_BOARD_ZOLID_HYBRID_PCI: c_int = 173;
pub const SAA7134_BOARD_ASUS_EUROPA_HYBRID: c_int = 174;
pub const SAA7134_BOARD_LEADTEK_WINFAST_DTV1000S: c_int = 175;
pub const SAA7134_BOARD_BEHOLD_505RDS_MK3: c_int = 176;
pub const SAA7134_BOARD_HAWELL_HW_404M7: c_int = 177;
pub const SAA7134_BOARD_BEHOLD_H7: c_int = 178;
pub const SAA7134_BOARD_BEHOLD_A7: c_int = 179;
pub const SAA7134_BOARD_AVERMEDIA_M733A: c_int = 180;
pub const SAA7134_BOARD_TECHNOTREND_BUDGET_T3000: c_int = 181;
pub const SAA7134_BOARD_KWORLD_PCI_SBTVD_FULLSEG: c_int = 182;
pub const SAA7134_BOARD_VIDEOMATE_M1F: c_int = 183;
pub const SAA7134_BOARD_ENCORE_ENLTV_FM3: c_int = 184;
pub const SAA7134_BOARD_MAGICPRO_PROHDTV_PRO2: c_int = 185;
pub const SAA7134_BOARD_BEHOLD_501: c_int = 186;
pub const SAA7134_BOARD_BEHOLD_503FM: c_int = 187;
pub const SAA7134_BOARD_SENSORAY811_911: c_int = 188;
pub const SAA7134_BOARD_KWORLD_PC150U: c_int = 189;
pub const SAA7134_BOARD_ASUSTeK_PS3_100: c_int = 190;
pub const SAA7134_BOARD_HAWELL_HW_9004V1: c_int = 191;
pub const SAA7134_BOARD_AVERMEDIA_A706: c_int = 192;
pub const SAA7134_BOARD_WIS_VOYAGER: c_int = 193;
pub const SAA7134_BOARD_AVERMEDIA_505: c_int = 194;
pub const SAA7134_BOARD_LEADTEK_WINFAST_TV2100_FM: c_int = 195;
pub const SAA7134_BOARD_SNAZIO_TVPVR_PRO: c_int = 196;
pub const SAA7134_BOARD_LEADTEK_WINFAST_HDTV200_H: c_int = 197;
pub const SAA7134_MAXBOARDS: c_int = 32;
pub const SAA7134_INPUT_MAX: c_int = 8;
// -----------------------------------------------------------
// Since we support 2 remote types, lets tell them apart
pub const SAA7134_REMOTE_GPIO: c_int = 1;
pub const SAA7134_REMOTE_I2C: c_int = 2;
// -----------------------------------------------------------
// Video Output Port Register Initialization Options

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum saa7134_input_types {
    SAA7134_NO_INPUT = 0,
    SAA7134_INPUT_MUTE,
    SAA7134_INPUT_RADIO,
    SAA7134_INPUT_TV,
    SAA7134_INPUT_TV_MONO,
    SAA7134_INPUT_COMPOSITE,
    SAA7134_INPUT_COMPOSITE0,
    SAA7134_INPUT_COMPOSITE1,
    SAA7134_INPUT_COMPOSITE2,
    SAA7134_INPUT_COMPOSITE3,
    SAA7134_INPUT_COMPOSITE4,
    SAA7134_INPUT_SVIDEO,
    SAA7134_INPUT_SVIDEO0,
    SAA7134_INPUT_SVIDEO1,
    SAA7134_INPUT_COMPOSITE_OVER_SVIDEO,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct saa7134_input {
    pub type: saa7134_input_types,
    pub vmux: c_uint,
    pub amux: saa7134_audio_in,
    pub gpio: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum saa7134_mpeg_type {
    SAA7134_MPEG_UNUSED,
    SAA7134_MPEG_EMPRESS,
    SAA7134_MPEG_DVB,
    SAA7134_MPEG_GO7007,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum saa7134_mpeg_ts_type {
    SAA7134_MPEG_TS_PARALLEL = 0,
    SAA7134_MPEG_TS_SERIAL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct saa7134_board {
    pub name: *mut c_char,
    pub audio_clock: c_uint,
// input switching
    pub gpiomask: c_uint,
    pub inputs: [saa7134_input; SAA7134_INPUT_MAX],
    pub radio: saa7134_input,
    pub mute: saa7134_input,
// i2c chip info
    pub tuner_type: c_uint,
    pub radio_type: c_uint,
    pub tuner_addr: c_uchar,
    pub radio_addr: c_uchar,
    pub empress_addr: c_uchar,
    pub rds_addr: c_uchar,
    pub tda9887_conf: c_uint,
    pub tda829x_conf: tda829x_config,
// peripheral I/O
    pub video_out: saa7134_video_out,
    pub mpeg: saa7134_mpeg_type,
    pub ts_type: saa7134_mpeg_ts_type,
    pub vid_port_opts: c_uint,
    pub ts_force_val:1: c_uint,
}

// -----------------------------------------------------------
// device / file handle status
pub const RESOURCE_VIDEO: c_int = 2;
pub const RESOURCE_VBI: c_int = 4;
pub const RESOURCE_EMPRESS: c_int = 8;
pub const INTERLACE_AUTO: c_int = 0;
pub const INTERLACE_ON: c_int = 1;
pub const INTERLACE_OFF: c_int = 2;

// saa7134 page table
#[repr(C)]
#[derive(Copy, Clone)]
pub struct saa7134_pgtable {
    pub size: c_uint,
    pub cpu: *mut __le32,
    pub dma: dma_addr_t,
}

// tvaudio thread status
#[repr(C)]
#[derive(Copy, Clone)]
pub struct saa7134_thread {
    pub thread: *mut task_struct,
    pub scan1: c_uint,
    pub scan2: c_uint,
    pub mode: c_uint,
    pub stopped: c_uint,
}

// buffer for one video/vbi/ts frame
#[repr(C)]
#[derive(Copy, Clone)]
pub struct saa7134_buf {
// common v4l buffer stuff -- must be first
    pub vb2: vb2_v4l2_buffer,
// saa7134 specific
    pub top_seen: c_uint,
    pub next): *mut saa7134_buf,
    pub entry: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct saa7134_dmaqueue {
    pub dev: *mut saa7134_dev,
    pub curr: *mut saa7134_buf,
    pub queue: list_head,
    pub timeout: timer_list,
    pub need_two: c_uint,
    pub seq_nr: c_uint,
    pub pt: saa7134_pgtable,
}

// dmasound dsp status
#[repr(C)]
#[derive(Copy, Clone)]
pub struct saa7134_dmasound {
    pub lock: mutex,
    pub minor_mixer: c_int,
    pub minor_dsp: c_int,
    pub users_dsp: c_uint,
// mixer
    pub input: saa7134_audio_in,
    pub count: c_uint,
    pub line1: c_uint,
    pub line2: c_uint,
// dsp
    pub afmt: c_uint,
    pub rate: c_uint,
    pub channels: c_uint,
    pub recording_on: c_uint,
    pub dma_running: c_uint,
    pub blocks: c_uint,
    pub blksize: c_uint,
    pub bufsize: c_uint,
    pub pt: saa7134_pgtable,
    pub vaddr: *mut c_void,
    pub sglist: *mut scatterlist,
    pub sglen: c_int,
    pub nr_pages: c_ulong,
    pub dma_blk: c_uint,
    pub read_offset: c_uint,
    pub read_count: c_uint,
    pub priv_data: *mut *mut c_void,
    pub substream: *mut snd_pcm_substream,
}

// ts/mpeg status
#[repr(C)]
#[derive(Copy, Clone)]
pub struct saa7134_ts {
// TS capture
    pub nr_packets: c_int,
    pub nr_bufs: c_int,
}

// ts/mpeg ops
#[repr(C)]
#[derive(Copy, Clone)]
pub struct saa7134_mpeg_ops {
    pub type: saa7134_mpeg_type,
    pub next: list_head,
    pub dev): *mut *mut int (init)(struct saa7134_dev,
    pub dev): *mut *mut int (fini)(struct saa7134_dev,
    pub dev): *mut *mut void (signal_change)(struct saa7134_dev,
    pub status): c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum saa7134_pads {
    SAA7134_PAD_IF_INPUT,
    SAA7134_PAD_VID_OUT,
    SAA7134_NUM_PADS
}

// global device status
#[repr(C)]
#[derive(Copy, Clone)]
pub struct saa7134_dev {
    pub devlist: list_head,
    pub lock: mutex,
    pub slock: spinlock_t,
    pub v4l2_dev: v4l2_device,
// workstruct for loading modules
    pub request_module_wk: work_struct,
// insmod option/autodetected
    pub autodetected: c_int,
// various device info
    pub resources: c_uint,
    pub video_dev: *mut video_device,
    pub radio_dev: *mut video_device,
    pub vbi_dev: *mut video_device,
    pub dmasound: saa7134_dmasound,
// infrared remote
    pub has_remote: c_int,
    pub remote: *mut saa7134_card_ir,
// pci i/o
    pub name: [c_char; 32],
    pub nr: c_int,
    pub pci: *mut pci_dev,
    pub pci_rev,pci_lat: c_uchar,
    pub lmmio: *mut __u32 __iomem,
    pub bmmio: *mut __u8 __iomem,
// config info
    pub board: c_uint,
    pub tuner_type: c_uint,
    pub radio_type: c_uint,
    pub tuner_addr: c_uchar,
    pub radio_addr: c_uchar,
    pub tda9887_conf: c_uint,
    pub gpio_value: c_uint,
// i2c i/o
    pub i2c_adap: i2c_adapter,
    pub i2c_client: i2c_client,
    pub eedata: [c_uchar; 256],
    pub has_rds: c_int,
// video+ts+vbi capture
    pub video_q: saa7134_dmaqueue,
    pub video_vbq: vb2_queue,
    pub vbi_q: saa7134_dmaqueue,
    pub vbi_vbq: vb2_queue,
    pub field: v4l2_field,
    pub fmt: *mut saa7134_format,
    pub height: unsigned int width,,
    pub vbi_vlen: unsigned int vbi_hlen,,
    pub qos_request: pm_qos_request,
// SAA7134_MPEG_*
    pub ts: saa7134_ts,
    pub ts_q: saa7134_dmaqueue,
    pub ts_field: v4l2_field,
    pub ts_started: c_int,
    pub mops: *mut saa7134_mpeg_ops,
// SAA7134_MPEG_EMPRESS only
    pub empress_dev: *mut video_device,
    pub empress_sd: *mut v4l2_subdev,
    pub empress_vbq: vb2_queue,
    pub empress_workqueue: work_struct,
    pub empress_started: c_int,
    pub empress_ctrl_handler: v4l2_ctrl_handler,
// various v4l controls
    pub /: *mut *mut *mut saa7134_tvnorm tvnorm; / video,
    pub tvaudio: *mut saa7134_tvaudio,
    pub ctrl_handler: v4l2_ctrl_handler,
    pub ctl_input: c_uint,
    pub ctl_bright: c_int,
    pub ctl_contrast: c_int,
    pub ctl_hue: c_int,
    pub ctl_saturation: c_int,
    pub /: *mut *mut int ctl_mute; / audio,
    pub ctl_volume: c_int,
    pub /: *mut *mut int ctl_invert; / private,
    pub ctl_mirror: c_int,
    pub ctl_y_odd: c_int,
    pub ctl_y_even: c_int,
    pub ctl_automute: c_int,
// crop
    pub crop_bounds: v4l2_rect,
    pub crop_defrect: v4l2_rect,
    pub crop_current: v4l2_rect,
// other global state info
    pub automute: c_uint,
    pub thread: saa7134_thread,
    pub input: *mut saa7134_input,
    pub hw_input: *mut saa7134_input,
    pub hw_mute: c_uint,
    pub last_carrier: c_int,
    pub nosignal: c_int,
    pub insuspend: c_uint,
    pub radio_ctrl_handler: v4l2_ctrl_handler,
// I2C keyboard data
    pub init_data: IR_i2c_init_data,

    pub media_dev: *mut media_device,
    pub 1]: media_entity input_ent[SAA7134_INPUT_MAX +,
    pub 1]: media_pad input_pad[SAA7134_INPUT_MAX +,
    pub demod: media_entity,
    pub demod_pad: [media_pad; SAA7134_NUM_PADS],
    pub vbi_pad: media_pad video_pad,,
    pub decoder: *mut media_entity,

// SAA7134_MPEG_DVB only
    pub frontends: vb2_dvb_frontends,
    pub fe): *mut *mut int (original_demod_sleep)(struct dvb_frontend,
    pub voltage): fe_sec_voltage,
    pub arg): *mut *mut *mut int (original_set_high_voltage)(struct dvb_frontend fe, long,

    pub open): *mut *mut *mut void (gate_ctrl)(struct saa7134_dev dev, int,
}

// -----------------------------------------------------------

// -----------------------------------------------------------
// saa7134-core.c
extern "C" {
    pub fn saa7134_track_gpio(dev: *mut saa7134_dev, msg: *const c_char);
}
extern "C" {
    pub fn saa7134_set_gpio(dev: *mut saa7134_dev, bit_no: c_int, value: c_int);
}
pub const SAA7134_PGTABLE_SIZE: c_int = 4096;
extern "C" {
    pub fn saa7134_pgtable_alloc(pci: *mut pci_dev, pt: *mut saa7134_pgtable) -> c_int;
}
extern "C" {
    pub fn saa7134_pgtable_free(pci: *mut pci_dev, pt: *mut saa7134_pgtable);
}
extern "C" {
    pub fn saa7134_buffer_count(size: c_uint, count: c_uint) -> c_int;
}
extern "C" {
    pub fn saa7134_buffer_startpage(buf: *mut saa7134_buf) -> c_int;
}
extern "C" {
    pub fn saa7134_buffer_base(buf: *mut saa7134_buf) -> c_ulong;
}
extern "C" {
    pub fn saa7134_buffer_next(dev: *mut saa7134_dev, q: *mut saa7134_dmaqueue);
}
extern "C" {
    pub fn saa7134_buffer_timeout(t: *mut timer_list);
}
extern "C" {
    pub fn saa7134_stop_streaming(dev: *mut saa7134_dev, q: *mut saa7134_dmaqueue);
}
extern "C" {
    pub fn saa7134_set_dmabits(dev: *mut saa7134_dev) -> c_int;
}
extern "C" {
    pub fn int(dev: *mut *mut saa7134_dmasound_init)(struct saa7134_dev) -> extern;
}
extern "C" {
    pub fn int(dev: *mut *mut saa7134_dmasound_exit)(struct saa7134_dev) -> extern;
}
// -----------------------------------------------------------
// saa7134-cards.c
extern "C" {
    pub fn saa7134_board_init1(dev: *mut saa7134_dev) -> c_int;
}
extern "C" {
    pub fn saa7134_board_init2(dev: *mut saa7134_dev) -> c_int;
}
extern "C" {
    pub fn saa7134_tuner_callback(priv: *mut c_void, component: c_int, command: c_int, arg: c_int) -> c_int;
}
// -----------------------------------------------------------
// saa7134-i2c.c
extern "C" {
    pub fn saa7134_i2c_register(dev: *mut saa7134_dev) -> c_int;
}
extern "C" {
    pub fn saa7134_i2c_unregister(dev: *mut saa7134_dev) -> c_int;
}
// -----------------------------------------------------------
// saa7134-video.c
extern "C" {
    pub fn saa7134_vb2_buffer_queue(vb: *mut vb2_buffer);
}
extern "C" {
    pub fn saa7134_vb2_start_streaming(vq: *mut vb2_queue, count: c_uint) -> c_int;
}
extern "C" {
    pub fn saa7134_vb2_stop_streaming(vq: *mut vb2_queue);
}
extern "C" {
    pub fn saa7134_s_std(file: *mut file, priv: *mut c_void, id: v4l2_std_id) -> c_int;
}
extern "C" {
    pub fn saa7134_g_std(file: *mut file, priv: *mut c_void, id: *mut v4l2_std_id) -> c_int;
}
extern "C" {
    pub fn saa7134_querystd(file: *mut file, priv: *mut c_void, std: *mut v4l2_std_id) -> c_int;
}
extern "C" {
    pub fn saa7134_enum_input(file: *mut file, priv: *mut c_void, i: *mut v4l2_input) -> c_int;
}
extern "C" {
    pub fn saa7134_g_input(file: *mut file, priv: *mut c_void, i: *mut c_uint) -> c_int;
}
extern "C" {
    pub fn saa7134_s_input(file: *mut file, priv: *mut c_void, i: c_uint) -> c_int;
}
extern "C" {
    pub fn saa7134_videoport_init(dev: *mut saa7134_dev) -> c_int;
}
extern "C" {
    pub fn saa7134_set_tvnorm_hw(dev: *mut saa7134_dev);
}
extern "C" {
    pub fn saa7134_video_init1(dev: *mut saa7134_dev) -> c_int;
}
extern "C" {
    pub fn saa7134_video_init2(dev: *mut saa7134_dev) -> c_int;
}
extern "C" {
    pub fn saa7134_irq_video_signalchange(dev: *mut saa7134_dev);
}
extern "C" {
    pub fn saa7134_irq_video_done(dev: *mut saa7134_dev, status: c_ulong);
}
extern "C" {
    pub fn saa7134_video_fini(dev: *mut saa7134_dev);
}
// -----------------------------------------------------------
// saa7134-ts.c

extern "C" {
    pub fn saa7134_ts_buffer_init(vb2: *mut vb2_buffer) -> c_int;
}
extern "C" {
    pub fn saa7134_ts_buffer_prepare(vb2: *mut vb2_buffer) -> c_int;
}
extern "C" {
    pub fn saa7134_ts_start_streaming(vq: *mut vb2_queue, count: c_uint) -> c_int;
}
extern "C" {
    pub fn saa7134_ts_stop_streaming(vq: *mut vb2_queue);
}
extern "C" {
    pub fn saa7134_ts_init1(dev: *mut saa7134_dev) -> c_int;
}
extern "C" {
    pub fn saa7134_ts_fini(dev: *mut saa7134_dev) -> c_int;
}
extern "C" {
    pub fn saa7134_irq_ts_done(dev: *mut saa7134_dev, status: c_ulong);
}
extern "C" {
    pub fn saa7134_ts_register(ops: *mut saa7134_mpeg_ops) -> c_int;
}
extern "C" {
    pub fn saa7134_ts_unregister(ops: *mut saa7134_mpeg_ops);
}
extern "C" {
    pub fn saa7134_ts_init_hw(dev: *mut saa7134_dev) -> c_int;
}
extern "C" {
    pub fn saa7134_ts_start(dev: *mut saa7134_dev) -> c_int;
}
extern "C" {
    pub fn saa7134_ts_stop(dev: *mut saa7134_dev) -> c_int;
}
// -----------------------------------------------------------
// saa7134-vbi.c
extern "C" {
    pub fn saa7134_vbi_init1(dev: *mut saa7134_dev) -> c_int;
}
extern "C" {
    pub fn saa7134_vbi_fini(dev: *mut saa7134_dev) -> c_int;
}
extern "C" {
    pub fn saa7134_irq_vbi_done(dev: *mut saa7134_dev, status: c_ulong);
}
// -----------------------------------------------------------
// saa7134-tvaudio.c
extern "C" {
    pub fn saa7134_tvaudio_rx2mode(rx: u32) -> c_int;
}
extern "C" {
    pub fn saa7134_tvaudio_setmute(dev: *mut saa7134_dev);
}
extern "C" {
    pub fn saa7134_tvaudio_setvolume(dev: *mut saa7134_dev, level: c_int);
}
extern "C" {
    pub fn saa7134_tvaudio_getstereo(dev: *mut saa7134_dev) -> c_int;
}
extern "C" {
    pub fn saa7134_tvaudio_init(dev: *mut saa7134_dev);
}
extern "C" {
    pub fn saa7134_tvaudio_init2(dev: *mut saa7134_dev) -> c_int;
}
extern "C" {
    pub fn saa7134_tvaudio_fini(dev: *mut saa7134_dev) -> c_int;
}
extern "C" {
    pub fn saa7134_tvaudio_do_scan(dev: *mut saa7134_dev) -> c_int;
}
extern "C" {
    pub fn saa7134_tvaudio_close(dev: *mut saa7134_dev) -> c_int;
}
extern "C" {
    pub fn saa_dsp_writel(dev: *mut saa7134_dev, reg: c_int, value: u32) -> c_int;
}
extern "C" {
    pub fn saa7134_enable_i2s(dev: *mut saa7134_dev);
}
// -----------------------------------------------------------
// saa7134-oss.c
extern "C" {
    pub fn saa7134_oss_init1(dev: *mut saa7134_dev) -> c_int;
}
extern "C" {
    pub fn saa7134_oss_fini(dev: *mut saa7134_dev) -> c_int;
}
extern "C" {
    pub fn saa7134_irq_oss_done(dev: *mut saa7134_dev, status: c_ulong);
}
// -----------------------------------------------------------
// saa7134-input.c

extern "C" {
    pub fn saa7134_input_init1(dev: *mut saa7134_dev) -> c_int;
}
extern "C" {
    pub fn saa7134_input_fini(dev: *mut saa7134_dev);
}
extern "C" {
    pub fn saa7134_input_irq(dev: *mut saa7134_dev);
}
extern "C" {
    pub fn saa7134_probe_i2c_ir(dev: *mut saa7134_dev);
}
extern "C" {
    pub fn saa7134_ir_open(dev: *mut rc_dev) -> c_int;
}
extern "C" {
    pub fn saa7134_ir_close(dev: *mut rc_dev);
}

