//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/wm97xx.h
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
// Register bits and API for Wolfson WM97xx series of codecs
//

//
// WM97xx variants
//
pub const WM97xx_GENERIC: c_uint = 0x0000;
pub const WM97xx_WM1613: c_uint = 0x1613;
//
// WM97xx AC97 Touchscreen registers
//
pub const AC97_WM97XX_DIGITISER1: c_uint = 0x76;
pub const AC97_WM97XX_DIGITISER2: c_uint = 0x78;
pub const AC97_WM97XX_DIGITISER_RD: c_uint = 0x7a;
pub const AC97_WM9713_DIG1: c_uint = 0x74;

//
// WM97xx register bits
//
pub const WM97XX_POLL: c_uint = 0x8000	/* initiate a polling measurement */;
pub const WM97XX_ADCSEL_X: c_uint = 0x1000	/* x coord measurement */;
pub const WM97XX_ADCSEL_Y: c_uint = 0x2000	/* y coord measurement */;
pub const WM97XX_ADCSEL_PRES: c_uint = 0x3000	/* pressure measurement */;
pub const WM97XX_AUX_ID1: c_uint = 0x4000;
pub const WM97XX_AUX_ID2: c_uint = 0x5000;
pub const WM97XX_AUX_ID3: c_uint = 0x6000;
pub const WM97XX_AUX_ID4: c_uint = 0x7000;
pub const WM97XX_ADCSEL_MASK: c_uint = 0x7000	/* ADC selection mask */;
pub const WM97XX_COO: c_uint = 0x0800	/* enable coordinate mode */;
pub const WM97XX_CTC: c_uint = 0x0400	/* enable continuous mode */;
pub const WM97XX_CM_RATE_93: c_uint = 0x0000	/* 93.75Hz continuous rate */;
pub const WM97XX_CM_RATE_187: c_uint = 0x0100	/* 187.5Hz continuous rate */;
pub const WM97XX_CM_RATE_375: c_uint = 0x0200	/* 375Hz continuous rate */;
pub const WM97XX_CM_RATE_750: c_uint = 0x0300	/* 750Hz continuous rate */;
pub const WM97XX_CM_RATE_8K: c_uint = 0x00f0	/* 8kHz continuous rate */;
pub const WM97XX_CM_RATE_12K: c_uint = 0x01f0	/* 12kHz continuous rate */;
pub const WM97XX_CM_RATE_24K: c_uint = 0x02f0	/* 24kHz continuous rate */;
pub const WM97XX_CM_RATE_48K: c_uint = 0x03f0	/* 48kHz continuous rate */;
pub const WM97XX_CM_RATE_MASK: c_uint = 0x03f0;

pub const WM97XX_DELAY_MASK: c_uint = 0x00f0;
pub const WM97XX_SLEN: c_uint = 0x0008	/* slot read back enable */;

pub const WM97XX_SLT_MASK: c_uint = 0x0007;
pub const WM97XX_PRP_DETW: c_uint = 0x4000	/* detect on, digitise off, wake */;
pub const WM97XX_PRP_DET: c_uint = 0x8000	/* detect on, digitise off, no wake */;
pub const WM97XX_PRP_DET_DIG: c_uint = 0xc000	/* setect on, digitise on */;
pub const WM97XX_RPR: c_uint = 0x2000	/* wake up on pen down */;
pub const WM97XX_PEN_DOWN: c_uint = 0x8000	/* pen is down */;
// WM9712 Bits
pub const WM9712_45W: c_uint = 0x1000	/* set for 5-wire touchscreen */;
pub const WM9712_PDEN: c_uint = 0x0800	/* measure only when pen down */;
pub const WM9712_WAIT: c_uint = 0x0200	/* wait until adc is read before next sample */;
pub const WM9712_PIL: c_uint = 0x0100	/* current used for pressure measurement. set 400uA else 200uA */;
pub const WM9712_MASK_HI: c_uint = 0x0040	/* hi on mask pin (47) stops conversions */;
pub const WM9712_MASK_EDGE: c_uint = 0x0080	/* rising/falling edge on pin delays sample */;
pub const WM9712_MASK_SYNC: c_uint = 0x00c0	/* rising/falling edge on mask initiates sample */;

// WM9712 Registers
pub const AC97_WM9712_POWER: c_uint = 0x24;
pub const AC97_WM9712_REV: c_uint = 0x58;
// WM9705 Bits
pub const WM9705_PDEN: c_uint = 0x1000	/* measure only when pen is down */;
pub const WM9705_PINV: c_uint = 0x0800	/* inverts sense of pen down output */;
pub const WM9705_BSEN: c_uint = 0x0400	/* BUSY flag enable, pin47 is 1 when busy */;
pub const WM9705_BINV: c_uint = 0x0200	/* invert BUSY (pin47) output */;
pub const WM9705_WAIT: c_uint = 0x0100	/* wait until adc is read before next sample */;
pub const WM9705_PIL: c_uint = 0x0080	/* current used for pressure measurement. set 400uA else 200uA */;
pub const WM9705_PHIZ: c_uint = 0x0040	/* set PHONE and PCBEEP inputs to high impedance */;
pub const WM9705_MASK_HI: c_uint = 0x0010	/* hi on mask stops conversions */;
pub const WM9705_MASK_EDGE: c_uint = 0x0020	/* rising/falling edge on pin delays sample */;
pub const WM9705_MASK_SYNC: c_uint = 0x0030	/* rising/falling edge on mask initiates sample */;

// WM9713 Bits
pub const WM9713_PDPOL: c_uint = 0x0400	/* Pen down polarity */;
pub const WM9713_POLL: c_uint = 0x0200	/* initiate a polling measurement */;
pub const WM9713_CTC: c_uint = 0x0100	/* enable continuous mode */;
pub const WM9713_ADCSEL_X: c_uint = 0x0002	/* X measurement */;
pub const WM9713_ADCSEL_Y: c_uint = 0x0004	/* Y measurement */;
pub const WM9713_ADCSEL_PRES: c_uint = 0x0008	/* Pressure measurement */;
pub const WM9713_COO: c_uint = 0x0001	/* enable coordinate mode */;
pub const WM9713_45W: c_uint = 0x1000  /* set for 5 wire panel */;
pub const WM9713_PDEN: c_uint = 0x0800	/* measure only when pen down */;
pub const WM9713_ADCSEL_MASK: c_uint = 0x00fe	/* ADC selection mask */;
pub const WM9713_WAIT: c_uint = 0x0200	/* coordinate wait */;
// AUX ADC ID's
pub const TS_COMP1: c_uint = 0x0;
pub const TS_COMP2: c_uint = 0x1;
pub const TS_BMON: c_uint = 0x2;
pub const TS_WIPER: c_uint = 0x3;
// ID numbers
pub const WM97XX_ID1: c_uint = 0x574d;
pub const WM9712_ID2: c_uint = 0x4c12;
pub const WM9705_ID2: c_uint = 0x4c05;
pub const WM9713_ID2: c_uint = 0x4c13;
// Codec GPIO's
pub const WM97XX_MAX_GPIO: c_int = 16;

// ---------------- Return codes from sample reading functions ---------------
// More data is available; call the sample gathering function again
pub const RC_AGAIN: c_uint = 0x00000001;
// The returned sample is valid
pub const RC_VALID: c_uint = 0x00000002;
// The pen is up (the first RC_VALID without RC_PENUP means pen is down)
pub const RC_PENUP: c_uint = 0x00000004;
// The pen is down (RC_VALID implies RC_PENDOWN, but sometimes it is helpful
pub const RC_PENDOWN: c_uint = 0x00000008;
//
// The wm97xx driver provides a private API for writing platform-specific
// drivers.
//
// The structure used to return arch specific sampled data into
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm97xx_data {
    pub x: c_int,
    pub y: c_int,
    pub p: c_int,
}

//
// Codec GPIO status
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wm97xx_gpio_status {
    WM97XX_GPIO_HIGH,
    WM97XX_GPIO_LOW
}

//
// Codec GPIO direction
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wm97xx_gpio_dir {
    WM97XX_GPIO_IN,
    WM97XX_GPIO_OUT
}

//
// Codec GPIO polarity
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wm97xx_gpio_pol {
    WM97XX_GPIO_POL_HIGH,
    WM97XX_GPIO_POL_LOW
}

//
// Codec GPIO sticky
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wm97xx_gpio_sticky {
    WM97XX_GPIO_STICKY,
    WM97XX_GPIO_NOTSTICKY
}

//
// Codec GPIO wake
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wm97xx_gpio_wake {
    WM97XX_GPIO_WAKE,
    WM97XX_GPIO_NOWAKE
}

//
// Digitiser ioctl commands
//
pub const WM97XX_DIG_START: c_uint = 0x1;
pub const WM97XX_DIG_STOP: c_uint = 0x2;
pub const WM97XX_PHY_INIT: c_uint = 0x3;
pub const WM97XX_AUX_PREPARE: c_uint = 0x4;
pub const WM97XX_DIG_RESTORE: c_uint = 0x5;
//
// Codec driver interface - allows mapping to WM9705/12/13 and newer codecs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm97xx_codec_drv {
    pub id: u16,
    pub name: *mut c_char,
// read 1 sample
    pub sample): *mut *mut *mut int (poll_sample) (struct wm97xx , int adcsel, int,
// read X,Y,[P] in poll
    pub ): *mut *mut *mut int (poll_touch) (struct wm97xx , struct wm97xx_data,
    pub enable): *mut *mut *mut int (acc_enable) (struct wm97xx , int,
    pub ): *mut *mut void (phy_init) (struct wm97xx,
    pub enable): *mut *mut *mut void (dig_enable) (struct wm97xx , int,
    pub ): *mut *mut void (dig_restore) (struct wm97xx,
    pub ): *mut *mut void (aux_prepare) (struct wm97xx,
}

// Machine specific and accelerated touch operations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm97xx_mach_ops {
// accelerated touch readback - coords are transmited on AC97 link
    pub acc_enabled: c_int,
    pub ): *mut *mut void (acc_pen_up) (struct wm97xx,
    pub ): *mut *mut int (acc_pen_down) (struct wm97xx,
    pub ): *mut *mut int (acc_startup) (struct wm97xx,
    pub ): *mut *mut void (acc_shutdown) (struct wm97xx,
// GPIO pin used for accelerated operation
    pub irq_gpio: c_int,
// pre and post sample - can be used to minimise any analog noise
    pub /: *mut *mut *mut void (pre_sample) (int); / function to run before sampling,
    pub /: *mut *mut *mut void (post_sample) (int); / function to run after sampling,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm97xx {
    pub /: *mut *mut u16 dig[3], id, gpio[6], misc; / Cached codec registers,
    pub /: *mut *mut u16 dig_save[3]; / saved during aux reading,
    pub driver*/: *mut *mut *mut wm97xx_codec_drv codec; / attached codec,
    pub /: *mut *mut *mut input_dev input_dev; / touchscreen input device,
    pub /: *mut *mut *mut snd_ac97 ac97; / ALSA codec access,
    pub /: *mut *mut *mut device dev; / ALSA device,
    pub battery_dev: *mut platform_device,
    pub touch_dev: *mut platform_device,
    pub mach_ops: *mut wm97xx_mach_ops,
    pub codec_mutex: mutex,
    pub /: *mut *mut delayed_work ts_reader; / Used to poll touchscreen,
    pub /: *mut *mut unsigned long ts_reader_interval; / Current interval for timer,
    pub /: *mut *mut unsigned long ts_reader_min_interval; / Minimum interval,
    pub /: *mut *mut unsigned int pen_irq; / Pen IRQ number in use,
    pub ts_workq: *mut workqueue_struct,
    pub /: *mut *mut u16 acc_slot; / AC97 slot used for acc touch data,
    pub /: *mut *mut u16 acc_rate; / acc touch data rate,
    pub /: *mut *mut unsigned pen_is_down:1; / Pen is down,
    pub /: *mut *mut unsigned aux_waiting:1; / aux measurement waiting,
    pub /: *mut *mut unsigned pen_probably_down:1; / used in polling mode,
    pub /: *mut *mut u16 variant; / WM97xx chip variant,
    pub /: *mut *mut u16 suspend_mode; / PRP in suspend mode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm97xx_batt_pdata {
    pub batt_aux: c_int,
    pub temp_aux: c_int,
    pub min_voltage: c_int,
    pub max_voltage: c_int,
    pub batt_div: c_int,
    pub batt_mult: c_int,
    pub temp_div: c_int,
    pub temp_mult: c_int,
    pub batt_tech: c_int,
    pub batt_name: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm97xx_pdata {
    pub /: *mut *mut *mut wm97xx_batt_pdata batt_pdata; / battery data,
}

//
// Codec GPIO access (not supported on WM9705)
// This can be used to set/get codec GPIO and Virtual GPIO status.
//
extern "C" {
    pub fn wm97xx_get_gpio(wm: *mut wm97xx, gpio: u32) -> wm97xx_gpio_status;
}
extern "C" {
    pub fn wm97xx_set_suspend_mode(wm: *mut wm97xx, mode: u16);
}
// codec AC97 IO access
extern "C" {
    pub fn wm97xx_reg_read(wm: *mut wm97xx, reg: u16) -> c_int;
}
extern "C" {
    pub fn wm97xx_reg_write(wm: *mut wm97xx, reg: u16, val: u16);
}
// aux adc readback
extern "C" {
    pub fn wm97xx_read_aux_adc(wm: *mut wm97xx, adcsel: u16) -> c_int;
}
// machine ops
extern "C" {
    pub fn wm97xx_register_mach_ops(: *mut wm97xx, : *mut wm97xx_mach_ops) -> c_int;
}
extern "C" {
    pub fn wm97xx_unregister_mach_ops(: *mut wm97xx);
}
