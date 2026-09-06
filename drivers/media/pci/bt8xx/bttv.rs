//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/bt8xx/bttv.h
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
//
// bttv - Bt848 frame grabber driver
//
// card ID's and external interfaces of the bttv driver
// basically stuff needed by other drivers (i2c, lirc, ...)
// and is supported not to change much over time.
//
// Copyright (C) 1996,97 Ralph Metzler (rjkm@thp.uni-koeln.de)
// (c) 1999,2000 Gerd Knorr <kraxel@goldbach.in-berlin.de>
//

// ----------------------------------------------------------
// exported by bttv-cards.c
pub const BTTV_BOARD_UNKNOWN: c_uint = 0x00;
pub const BTTV_BOARD_MIRO: c_uint = 0x01;
pub const BTTV_BOARD_HAUPPAUGE: c_uint = 0x02;
pub const BTTV_BOARD_STB: c_uint = 0x03;
pub const BTTV_BOARD_INTEL: c_uint = 0x04;
pub const BTTV_BOARD_DIAMOND: c_uint = 0x05;
pub const BTTV_BOARD_AVERMEDIA: c_uint = 0x06;
pub const BTTV_BOARD_MATRIX_VISION: c_uint = 0x07;
pub const BTTV_BOARD_FLYVIDEO: c_uint = 0x08;
pub const BTTV_BOARD_TURBOTV: c_uint = 0x09;
pub const BTTV_BOARD_HAUPPAUGE878: c_uint = 0x0a;
pub const BTTV_BOARD_MIROPRO: c_uint = 0x0b;
pub const BTTV_BOARD_ADSTECH_TV: c_uint = 0x0c;
pub const BTTV_BOARD_AVERMEDIA98: c_uint = 0x0d;
pub const BTTV_BOARD_VHX: c_uint = 0x0e;
pub const BTTV_BOARD_ZOLTRIX: c_uint = 0x0f;
pub const BTTV_BOARD_PIXVIEWPLAYTV: c_uint = 0x10;
pub const BTTV_BOARD_WINVIEW_601: c_uint = 0x11;
pub const BTTV_BOARD_AVEC_INTERCAP: c_uint = 0x12;
pub const BTTV_BOARD_LIFE_FLYKIT: c_uint = 0x13;
pub const BTTV_BOARD_CEI_RAFFLES: c_uint = 0x14;
pub const BTTV_BOARD_CONFERENCETV: c_uint = 0x15;
pub const BTTV_BOARD_PHOEBE_TVMAS: c_uint = 0x16;
pub const BTTV_BOARD_MODTEC_205: c_uint = 0x17;
pub const BTTV_BOARD_MAGICTVIEW061: c_uint = 0x18;
pub const BTTV_BOARD_VOBIS_BOOSTAR: c_uint = 0x19;
pub const BTTV_BOARD_HAUPPAUG_WCAM: c_uint = 0x1a;
pub const BTTV_BOARD_MAXI: c_uint = 0x1b;
pub const BTTV_BOARD_TERRATV: c_uint = 0x1c;
pub const BTTV_BOARD_PXC200: c_uint = 0x1d;
pub const BTTV_BOARD_FLYVIDEO_98: c_uint = 0x1e;
pub const BTTV_BOARD_IPROTV: c_uint = 0x1f;
pub const BTTV_BOARD_INTEL_C_S_PCI: c_uint = 0x20;
pub const BTTV_BOARD_TERRATVALUE: c_uint = 0x21;
pub const BTTV_BOARD_WINFAST2000: c_uint = 0x22;
pub const BTTV_BOARD_CHRONOS_VS2: c_uint = 0x23;
pub const BTTV_BOARD_TYPHOON_TVIEW: c_uint = 0x24;
pub const BTTV_BOARD_PXELVWPLTVPRO: c_uint = 0x25;
pub const BTTV_BOARD_MAGICTVIEW063: c_uint = 0x26;
pub const BTTV_BOARD_PINNACLE: c_uint = 0x27;
pub const BTTV_BOARD_STB2: c_uint = 0x28;
pub const BTTV_BOARD_AVPHONE98: c_uint = 0x29;
pub const BTTV_BOARD_PV951: c_uint = 0x2a;
pub const BTTV_BOARD_ONAIR_TV: c_uint = 0x2b;
pub const BTTV_BOARD_SIGMA_TVII_FM: c_uint = 0x2c;
pub const BTTV_BOARD_MATRIX_VISION2: c_uint = 0x2d;
pub const BTTV_BOARD_ZOLTRIX_GENIE: c_uint = 0x2e;
pub const BTTV_BOARD_TERRATVRADIO: c_uint = 0x2f;
pub const BTTV_BOARD_DYNALINK: c_uint = 0x30;
pub const BTTV_BOARD_GVBCTV3PCI: c_uint = 0x31;
pub const BTTV_BOARD_PXELVWPLTVPAK: c_uint = 0x32;
pub const BTTV_BOARD_EAGLE: c_uint = 0x33;
pub const BTTV_BOARD_PINNACLEPRO: c_uint = 0x34;
pub const BTTV_BOARD_TVIEW_RDS_FM: c_uint = 0x35;
pub const BTTV_BOARD_LIFETEC_9415: c_uint = 0x36;
pub const BTTV_BOARD_BESTBUY_EASYTV: c_uint = 0x37;
pub const BTTV_BOARD_FLYVIDEO_98FM: c_uint = 0x38;
pub const BTTV_BOARD_GRANDTEC: c_uint = 0x39;
pub const BTTV_BOARD_ASKEY_CPH060: c_uint = 0x3a;
pub const BTTV_BOARD_ASKEY_CPH03X: c_uint = 0x3b;
pub const BTTV_BOARD_MM100PCTV: c_uint = 0x3c;
pub const BTTV_BOARD_GMV1: c_uint = 0x3d;
pub const BTTV_BOARD_BESTBUY_EASYTV2: c_uint = 0x3e;
pub const BTTV_BOARD_ATI_TVWONDER: c_uint = 0x3f;
pub const BTTV_BOARD_ATI_TVWONDERVE: c_uint = 0x40;
pub const BTTV_BOARD_FLYVIDEO2000: c_uint = 0x41;
pub const BTTV_BOARD_TERRATVALUER: c_uint = 0x42;
pub const BTTV_BOARD_GVBCTV4PCI: c_uint = 0x43;
pub const BTTV_BOARD_VOODOOTV_FM: c_uint = 0x44;
pub const BTTV_BOARD_AIMMS: c_uint = 0x45;
pub const BTTV_BOARD_PV_BT878P_PLUS: c_uint = 0x46;
pub const BTTV_BOARD_FLYVIDEO98EZ: c_uint = 0x47;
pub const BTTV_BOARD_PV_BT878P_9B: c_uint = 0x48;
pub const BTTV_BOARD_SENSORAY311_611: c_uint = 0x49;
pub const BTTV_BOARD_RV605: c_uint = 0x4a;
pub const BTTV_BOARD_POWERCLR_MTV878: c_uint = 0x4b;
pub const BTTV_BOARD_WINDVR: c_uint = 0x4c;
pub const BTTV_BOARD_GRANDTEC_MULTI: c_uint = 0x4d;
pub const BTTV_BOARD_KWORLD: c_uint = 0x4e;
pub const BTTV_BOARD_DSP_TCVIDEO: c_uint = 0x4f;
pub const BTTV_BOARD_HAUPPAUGEPVR: c_uint = 0x50;
pub const BTTV_BOARD_GVBCTV5PCI: c_uint = 0x51;
pub const BTTV_BOARD_OSPREY1x0: c_uint = 0x52;
pub const BTTV_BOARD_OSPREY1x0_848: c_uint = 0x53;
pub const BTTV_BOARD_OSPREY101_848: c_uint = 0x54;
pub const BTTV_BOARD_OSPREY1x1: c_uint = 0x55;
pub const BTTV_BOARD_OSPREY1x1_SVID: c_uint = 0x56;
pub const BTTV_BOARD_OSPREY2xx: c_uint = 0x57;
pub const BTTV_BOARD_OSPREY2x0_SVID: c_uint = 0x58;
pub const BTTV_BOARD_OSPREY2x0: c_uint = 0x59;
pub const BTTV_BOARD_OSPREY500: c_uint = 0x5a;
pub const BTTV_BOARD_OSPREY540: c_uint = 0x5b;
pub const BTTV_BOARD_OSPREY2000: c_uint = 0x5c;
pub const BTTV_BOARD_IDS_EAGLE: c_uint = 0x5d;
pub const BTTV_BOARD_PINNACLESAT: c_uint = 0x5e;
pub const BTTV_BOARD_FORMAC_PROTV: c_uint = 0x5f;
pub const BTTV_BOARD_MACHTV: c_uint = 0x60;
pub const BTTV_BOARD_EURESYS_PICOLO: c_uint = 0x61;
pub const BTTV_BOARD_PV150: c_uint = 0x62;
pub const BTTV_BOARD_AD_TVK503: c_uint = 0x63;
pub const BTTV_BOARD_HERCULES_SM_TV: c_uint = 0x64;
pub const BTTV_BOARD_PACETV: c_uint = 0x65;
pub const BTTV_BOARD_IVC200: c_uint = 0x66;
pub const BTTV_BOARD_XGUARD: c_uint = 0x67;
pub const BTTV_BOARD_NEBULA_DIGITV: c_uint = 0x68;
pub const BTTV_BOARD_PV143: c_uint = 0x69;
pub const BTTV_BOARD_VD009X1_VD011_MINIDIN: c_uint = 0x6a;
pub const BTTV_BOARD_VD009X1_VD011_COMBI: c_uint = 0x6b;
pub const BTTV_BOARD_VD009_MINIDIN: c_uint = 0x6c;
pub const BTTV_BOARD_VD009_COMBI: c_uint = 0x6d;
pub const BTTV_BOARD_IVC100: c_uint = 0x6e;
pub const BTTV_BOARD_IVC120: c_uint = 0x6f;
pub const BTTV_BOARD_PC_HDTV: c_uint = 0x70;
pub const BTTV_BOARD_TWINHAN_DST: c_uint = 0x71;
pub const BTTV_BOARD_WINFASTVC100: c_uint = 0x72;
pub const BTTV_BOARD_TEV560: c_uint = 0x73;
pub const BTTV_BOARD_SIMUS_GVC1100: c_uint = 0x74;
pub const BTTV_BOARD_NGSTV_PLUS: c_uint = 0x75;
pub const BTTV_BOARD_LMLBT4: c_uint = 0x76;
pub const BTTV_BOARD_TEKRAM_M205: c_uint = 0x77;
pub const BTTV_BOARD_CONTVFMI: c_uint = 0x78;
pub const BTTV_BOARD_PICOLO_TETRA_CHIP: c_uint = 0x79;
pub const BTTV_BOARD_SPIRIT_TV: c_uint = 0x7a;
pub const BTTV_BOARD_AVDVBT_771: c_uint = 0x7b;
pub const BTTV_BOARD_AVDVBT_761: c_uint = 0x7c;
pub const BTTV_BOARD_MATRIX_VISIONSQ: c_uint = 0x7d;
pub const BTTV_BOARD_MATRIX_VISIONSLC: c_uint = 0x7e;
pub const BTTV_BOARD_APAC_VIEWCOMP: c_uint = 0x7f;
pub const BTTV_BOARD_DVICO_DVBT_LITE: c_uint = 0x80;
pub const BTTV_BOARD_VGEAR_MYVCD: c_uint = 0x81;
pub const BTTV_BOARD_SUPER_TV: c_uint = 0x82;
pub const BTTV_BOARD_TIBET_CS16: c_uint = 0x83;
pub const BTTV_BOARD_KODICOM_4400R: c_uint = 0x84;
pub const BTTV_BOARD_KODICOM_4400R_SL: c_uint = 0x85;
pub const BTTV_BOARD_ADLINK_RTV24: c_uint = 0x86;
pub const BTTV_BOARD_DVICO_FUSIONHDTV_5_LITE: c_uint = 0x87;
pub const BTTV_BOARD_ACORP_Y878F: c_uint = 0x88;
pub const BTTV_BOARD_CONCEPTRONIC_CTVFMI2: c_uint = 0x89;
pub const BTTV_BOARD_PV_BT878P_2E: c_uint = 0x8a;
pub const BTTV_BOARD_PV_M4900: c_uint = 0x8b;
pub const BTTV_BOARD_OSPREY440: c_uint = 0x8c;
pub const BTTV_BOARD_ASOUND_SKYEYE: c_uint = 0x8d;
pub const BTTV_BOARD_SABRENT_TVFM: c_uint = 0x8e;
pub const BTTV_BOARD_HAUPPAUGE_IMPACTVCB: c_uint = 0x8f;
pub const BTTV_BOARD_MACHTV_MAGICTV: c_uint = 0x90;
pub const BTTV_BOARD_SSAI_SECURITY: c_uint = 0x91;
pub const BTTV_BOARD_SSAI_ULTRASOUND: c_uint = 0x92;
pub const BTTV_BOARD_VOODOOTV_200: c_uint = 0x93;
pub const BTTV_BOARD_DVICO_FUSIONHDTV_2: c_uint = 0x94;
pub const BTTV_BOARD_TYPHOON_TVTUNERPCI: c_uint = 0x95;
pub const BTTV_BOARD_GEOVISION_GV600: c_uint = 0x96;
pub const BTTV_BOARD_KOZUMI_KTV_01C: c_uint = 0x97;
pub const BTTV_BOARD_ENLTV_FM_2: c_uint = 0x98;
pub const BTTV_BOARD_VD012: c_uint = 0x99;
pub const BTTV_BOARD_VD012_X1: c_uint = 0x9a;
pub const BTTV_BOARD_VD012_X2: c_uint = 0x9b;
pub const BTTV_BOARD_IVCE8784: c_uint = 0x9c;
pub const BTTV_BOARD_GEOVISION_GV800S: c_uint = 0x9d;
pub const BTTV_BOARD_GEOVISION_GV800S_SL: c_uint = 0x9e;
pub const BTTV_BOARD_PV183: c_uint = 0x9f;
pub const BTTV_BOARD_TVT_TD3116: c_uint = 0xa0;
pub const BTTV_BOARD_APOSONIC_WDVR: c_uint = 0xa1;
pub const BTTV_BOARD_ADLINK_MPG24: c_uint = 0xa2;
pub const BTTV_BOARD_BT848_CAP_14: c_uint = 0xa3;
pub const BTTV_BOARD_CYBERVISION_CV06: c_uint = 0xa4;
pub const BTTV_BOARD_KWORLD_VSTREAM_XPERT: c_uint = 0xa5;
pub const BTTV_BOARD_PCI_8604PW: c_uint = 0xa6;
// more card-specific defines
pub const PT2254_L_CHANNEL: c_uint = 0x10;
pub const PT2254_R_CHANNEL: c_uint = 0x08;
pub const PT2254_DBS_IN_2: c_uint = 0x400;
pub const PT2254_DBS_IN_10: c_uint = 0x20000;
pub const WINVIEW_PT2254_CLK: c_uint = 0x40;
pub const WINVIEW_PT2254_DATA: c_uint = 0x20;
pub const WINVIEW_PT2254_STROBE: c_uint = 0x80;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bttv_core {
// device structs
    pub v4l2_dev: v4l2_device,
    pub pci: *mut pci_dev,
    pub i2c_adap: i2c_adapter,
    pub /: *mut *mut list_head subs; / bttv_sub_device,
// device config
    pub /: *mut *mut unsigned int nr; / dev nr (for printk("bttv%d: ...");,
    pub /: *mut *mut unsigned int type; / card type (pointer into tvcards[]),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tvcard {
    pub name: *mut c_char,
    pub volume): *mut *mut *mut void (volume_gpio)(struct bttv btv, __u16,
    pub set): *mut *mut *mut *mut void (audio_mode_gpio)(struct bttv btv, struct v4l2_tuner tuner, int,
    pub input): *mut *mut *mut void (muxsel_hook)(struct bttv btv, unsigned int,
// MUX bits for each input, two bits per input starting with the LSB
    pub /: *mut *mut u32 muxsel; / Use MUXSEL() to set,
    pub gpiomask: u32,
    pub /: *mut *mut u32 gpiomux[4]; / Tuner, Radio, external, internal,
    pub /: *mut *mut u32 gpiomute; / GPIO mute setting,
    pub /: *mut *mut u32 gpiomask2; / GPIO MUX mask,
    pub tuner_type: c_uint,
    pub tuner_addr: u8,
    pub /: *mut *mut u8 video_inputs; / Number of inputs,
    pub /: *mut *mut unsigned int svhs:4; / Which input is s-video,
pub const NO_SVHS: c_int = 15;
    pub pll:2: c_uint,
pub const PLL_NONE: c_int = 0;
pub const PLL_28: c_int = 1;
pub const PLL_35: c_int = 2;
pub const PLL_14: c_int = 3;
// i2c audio flags
    pub no_msp34xx:1: c_uint,
    pub no_tda7432:1: c_uint,
    pub msp34xx_alt:1: c_uint,
// Note: currently no card definition needs to mark the presence
    pub /: *mut *mut unsigned int no_video:1; / video pci function is unused,
    pub has_dvb:1: c_uint,
    pub has_remote:1: c_uint,
    pub has_radio:1: c_uint,
    pub /: *mut *mut unsigned int has_dig_in:1; / Has digital input (always last input),
    pub no_gpioirq:1: c_uint,
}

//
// This bit of cpp voodoo is used to create a macro with a variable number of
// arguments (1 to 16).  It will pack each argument into a word two bits at a
// time.  It can't be a function because it needs to be compile time constant to
// initialize structures.  Since each argument must fit in two bits, it's ok
// that they are changed to octal.  One should not use hex number, macros, or
// anything else with this macro.  Just use plain integers from 0 to 3.
//

// identification / initialization of the card
extern "C" {
    pub fn bttv_idcard(btv: *mut bttv);
}
extern "C" {
    pub fn bttv_init_card1(btv: *mut bttv);
}
extern "C" {
    pub fn bttv_init_card2(btv: *mut bttv);
}
extern "C" {
    pub fn bttv_init_tuner(btv: *mut bttv);
}
// card-specific functions
extern "C" {
    pub fn bttv_tda9880_setnorm(btv: *mut bttv, gpiobits: u32) -> u32;
}
// extra tweaks for some chipsets
extern "C" {
    pub fn bttv_check_chipset();
}
extern "C" {
    pub fn bttv_handle_chipset(btv: *mut bttv) -> c_int;
}
// ----------------------------------------------------------
// exported by bttv-if.c
// this obsolete -- please use the sysfs-based
extern "C" {
    pub fn bttv_get_pcidev(card: c_uint) -> *mut pci_dev;
}
// sets GPOE register (BT848_GPIO_OUT_EN) to new value:
//
// fills data with GPDATA register contents
//
extern "C" {
    pub fn bttv_read_gpio(card: c_uint, data: *mut c_ulong) -> c_int;
}
// sets GPDATA register to new value:
//
// ----------------------------------------------------------
// sysfs/driver-moded based gpio access interface
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bttv_sub_device {
    pub dev: device,
    pub core: *mut bttv_core,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bttv_sub_driver {
    pub drv: device_driver,
    pub wanted: [c_char; 20],
    pub sub): *mut *mut int (probe)(struct bttv_sub_device,
    pub sub): *mut *mut void (remove)(struct bttv_sub_device,
}

extern "C" {
    pub fn bttv_sub_register(drv: *mut bttv_sub_driver, wanted: *mut c_char) -> c_int;
}
extern "C" {
    pub fn bttv_sub_unregister(drv: *mut bttv_sub_driver) -> c_int;
}
// gpio access functions
extern "C" {
    pub fn bttv_gpio_inout(core: *mut bttv_core, mask: u32, outbits: u32);
}
extern "C" {
    pub fn bttv_gpio_read(core: *mut bttv_core) -> u32;
}
extern "C" {
    pub fn bttv_gpio_write(core: *mut bttv_core, value: u32);
}
extern "C" {
    pub fn bttv_gpio_bits(core: *mut bttv_core, mask: u32, bits: u32);
}

// ----------------------------------------------------------
// i2c

extern "C" {
    pub fn bttv_I2CRead(btv: *mut bttv, addr: c_uchar, probe_for: *mut c_char) -> c_int;
}
extern "C" {
    pub fn bttv_readee(btv: *mut bttv, eedata: *mut c_uchar, addr: c_int);
}
extern "C" {
    pub fn bttv_input_init(dev: *mut bttv) -> c_int;
}
extern "C" {
    pub fn bttv_input_fini(dev: *mut bttv);
}
extern "C" {
    pub fn bttv_input_irq(dev: *mut bttv);
}
