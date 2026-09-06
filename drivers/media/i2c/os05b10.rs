//! Automatically rewritten from C to Rust
//! Source: drivers/media/i2c/os05b10.c
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
// V4L2 Support for the os05b10
//
// Copyright (C) 2025 Silicon Signals Pvt. Ltd.
//
// Inspired from imx219, ov2735 camera drivers.
//

pub const OS05B10_CHIP_ID: c_uint = 0x530641;

pub const OS05B10_MODE_STANDBY: c_uint = 0x00;
pub const OS05B10_MODE_STREAMING: c_uint = 0x01;

pub const OS05B10_EXPOSURE_MIN: c_int = 2;
pub const OS05B10_EXPOSURE_STEP: c_int = 1;
pub const OS05B10_EXPOSURE_MARGIN: c_int = 8;

pub const OS05B10_ANALOG_GAIN_MIN: c_uint = 0x80;
pub const OS05B10_ANALOG_GAIN_MAX: c_uint = 0x7C0;
pub const OS05B10_ANALOG_GAIN_STEP: c_int = 1;
pub const OS05B10_ANALOG_GAIN_DEFAULT: c_uint = 0x80;

pub const OS05B10_VTS_MAX: c_uint = 0x7fff;

    static const struct v4l2_rect os05b10_native_area = {
    .top = 0,
    .left = 0,
    .width = 2608,
    .height = 1960,
    };
    static const struct v4l2_rect os05b10_active_area = {
    .top = 8,
    .left = 8,
    .width = 2592,
    .height = 1944,
    };
    static const char * const os05b10_supply_name[] = {
    "avdd",		/* Analog supply */
    "dovdd",	/* Digital IO */
    "dvdd",		/* Digital core */
    };
    static const struct cci_reg_sequence os05b10_common_regs[] = {
    { CCI_REG8(0x0301), 0x44 },
    { CCI_REG8(0x0303), 0x02 },
    { CCI_REG8(0x0305), 0x32 },
    { CCI_REG8(0x0306), 0x00 },
    { CCI_REG8(0x0325), 0x3b },
    { CCI_REG8(0x3002), 0x21 },
    { CCI_REG8(0x3016), 0x72 },
    { CCI_REG8(0x301e), 0xb4 },
    { CCI_REG8(0x301f), 0xd0 },
    { CCI_REG8(0x3021), 0x03 },
    { CCI_REG8(0x3022), 0x01 },
    { CCI_REG8(0x3107), 0xa1 },
    { CCI_REG8(0x3108), 0x7d },
    { CCI_REG8(0x3109), 0xfc },
    { CCI_REG8(0x3503), 0x88 },
    { CCI_REG8(0x350a), 0x04 },
    { CCI_REG8(0x350b), 0x00 },
    { CCI_REG8(0x350c), 0x00 },
    { CCI_REG8(0x350d), 0x80 },
    { CCI_REG8(0x350e), 0x04 },
    { CCI_REG8(0x350f), 0x00 },
    { CCI_REG8(0x3510), 0x00 },
    { CCI_REG8(0x3511), 0x00 },
    { CCI_REG8(0x3512), 0x20 },
    { CCI_REG8(0x3600), 0x4d },
    { CCI_REG8(0x3601), 0x08 },
    { CCI_REG8(0x3610), 0x87 },
    { CCI_REG8(0x3611), 0x24 },
    { CCI_REG8(0x3614), 0x4c },
    { CCI_REG8(0x3620), 0x0c },
    { CCI_REG8(0x3632), 0x80 },
    { CCI_REG8(0x3633), 0x00 },
    { CCI_REG8(0x3636), 0xcc },
    { CCI_REG8(0x3637), 0x27 },
    { CCI_REG8(0x3660), 0x00 },
    { CCI_REG8(0x3662), 0x10 },
    { CCI_REG8(0x3665), 0x00 },
    { CCI_REG8(0x3666), 0x00 },
    { CCI_REG8(0x366a), 0x14 },
    { CCI_REG8(0x3670), 0x0b },
    { CCI_REG8(0x3671), 0x0b },
    { CCI_REG8(0x3672), 0x0b },
    { CCI_REG8(0x3673), 0x0b },
    { CCI_REG8(0x3678), 0x2b },
    { CCI_REG8(0x367a), 0x11 },
    { CCI_REG8(0x367b), 0x11 },
    { CCI_REG8(0x367c), 0x11 },
    { CCI_REG8(0x367d), 0x11 },
    { CCI_REG8(0x3681), 0xff },
    { CCI_REG8(0x3682), 0x86 },
    { CCI_REG8(0x3683), 0x44 },
    { CCI_REG8(0x3684), 0x24 },
    { CCI_REG8(0x3685), 0x00 },
    { CCI_REG8(0x368a), 0x00 },
    { CCI_REG8(0x368d), 0x2b },
    { CCI_REG8(0x368e), 0x2b },
    { CCI_REG8(0x3690), 0x00 },
    { CCI_REG8(0x3691), 0x0b },
    { CCI_REG8(0x3692), 0x0b },
    { CCI_REG8(0x3693), 0x0b },
    { CCI_REG8(0x3694), 0x0b },
    { CCI_REG8(0x369d), 0x68 },
    { CCI_REG8(0x369e), 0x34 },
    { CCI_REG8(0x369f), 0x1b },
    { CCI_REG8(0x36a0), 0x0f },
    { CCI_REG8(0x36a1), 0x77 },
    { CCI_REG8(0x36b0), 0x30 },
    { CCI_REG8(0x36b2), 0x00 },
    { CCI_REG8(0x36b3), 0x00 },
    { CCI_REG8(0x36b4), 0x00 },
    { CCI_REG8(0x36b5), 0x00 },
    { CCI_REG8(0x36b6), 0x00 },
    { CCI_REG8(0x36b7), 0x00 },
    { CCI_REG8(0x36b8), 0x00 },
    { CCI_REG8(0x36b9), 0x00 },
    { CCI_REG8(0x36ba), 0x00 },
    { CCI_REG8(0x36bb), 0x00 },
    { CCI_REG8(0x36bc), 0x00 },
    { CCI_REG8(0x36bd), 0x00 },
    { CCI_REG8(0x36be), 0x00 },
    { CCI_REG8(0x36bf), 0x00 },
    { CCI_REG8(0x36c0), 0x01 },
    { CCI_REG8(0x36c1), 0x00 },
    { CCI_REG8(0x36c2), 0x00 },
    { CCI_REG8(0x36c3), 0x00 },
    { CCI_REG8(0x36c4), 0x00 },
    { CCI_REG8(0x36c5), 0x00 },
    { CCI_REG8(0x36c6), 0x00 },
    { CCI_REG8(0x36c7), 0x00 },
    { CCI_REG8(0x36c8), 0x00 },
    { CCI_REG8(0x36c9), 0x00 },
    { CCI_REG8(0x36ca), 0x0e },
    { CCI_REG8(0x36cb), 0x0e },
    { CCI_REG8(0x36cc), 0x0e },
    { CCI_REG8(0x36cd), 0x0e },
    { CCI_REG8(0x36ce), 0x0c },
    { CCI_REG8(0x36cf), 0x0c },
    { CCI_REG8(0x36d0), 0x0c },
    { CCI_REG8(0x36d1), 0x0c },
    { CCI_REG8(0x36d2), 0x00 },
    { CCI_REG8(0x36d3), 0x08 },
    { CCI_REG8(0x36d4), 0x10 },
    { CCI_REG8(0x36d5), 0x10 },
    { CCI_REG8(0x36d6), 0x00 },
    { CCI_REG8(0x36d7), 0x08 },
    { CCI_REG8(0x36d8), 0x10 },
    { CCI_REG8(0x36d9), 0x10 },
    { CCI_REG8(0x3701), 0x1d },
    { CCI_REG8(0x3703), 0x2a },
    { CCI_REG8(0x3704), 0x05 },
    { CCI_REG8(0x3709), 0x57 },
    { CCI_REG8(0x370b), 0x63 },
    { CCI_REG8(0x3706), 0x28 },
    { CCI_REG8(0x370a), 0x00 },
    { CCI_REG8(0x370b), 0x63 },
    { CCI_REG8(0x370e), 0x0c },
    { CCI_REG8(0x370f), 0x1c },
    { CCI_REG8(0x3710), 0x00 },
    { CCI_REG8(0x3713), 0x00 },
    { CCI_REG8(0x3714), 0x24 },
    { CCI_REG8(0x3716), 0x24 },
    { CCI_REG8(0x371a), 0x1e },
    { CCI_REG8(0x3724), 0x09 },
    { CCI_REG8(0x3725), 0xb2 },
    { CCI_REG8(0x372b), 0x54 },
    { CCI_REG8(0x3730), 0xe1 },
    { CCI_REG8(0x3735), 0x80 },
    { CCI_REG8(0x3739), 0x10 },
    { CCI_REG8(0x373f), 0xb0 },
    { CCI_REG8(0x3740), 0x28 },
    { CCI_REG8(0x3741), 0x21 },
    { CCI_REG8(0x3742), 0x21 },
    { CCI_REG8(0x3743), 0x21 },
    { CCI_REG8(0x3744), 0x63 },
    { CCI_REG8(0x3745), 0x5a },
    { CCI_REG8(0x3746), 0x5a },
    { CCI_REG8(0x3747), 0x5a },
    { CCI_REG8(0x3748), 0x00 },
    { CCI_REG8(0x3749), 0x00 },
    { CCI_REG8(0x374a), 0x00 },
    { CCI_REG8(0x374b), 0x00 },
    { CCI_REG8(0x3756), 0x00 },
    { CCI_REG8(0x3757), 0x0e },
    { CCI_REG8(0x375d), 0x84 },
    { CCI_REG8(0x3760), 0x11 },
    { CCI_REG8(0x3767), 0x08 },
    { CCI_REG8(0x376f), 0x42 },
    { CCI_REG8(0x3771), 0x00 },
    { CCI_REG8(0x3773), 0x01 },
    { CCI_REG8(0x3774), 0x02 },
    { CCI_REG8(0x3775), 0x12 },
    { CCI_REG8(0x3776), 0x02 },
    { CCI_REG8(0x377b), 0x40 },
    { CCI_REG8(0x377c), 0x00 },
    { CCI_REG8(0x377d), 0x0c },
    { CCI_REG8(0x3782), 0x02 },
    { CCI_REG8(0x3787), 0x24 },
    { CCI_REG8(0x378a), 0x01 },
    { CCI_REG8(0x378d), 0x00 },
    { CCI_REG8(0x3790), 0x1f },
    { CCI_REG8(0x3791), 0x58 },
    { CCI_REG8(0x3795), 0x24 },
    { CCI_REG8(0x3796), 0x01 },
    { CCI_REG8(0x3798), 0x40 },
    { CCI_REG8(0x379c), 0x00 },
    { CCI_REG8(0x379d), 0x00 },
    { CCI_REG8(0x379e), 0x00 },
    { CCI_REG8(0x379f), 0x01 },
    { CCI_REG8(0x37a1), 0x10 },
    { CCI_REG8(0x37a6), 0x00 },
    { CCI_REG8(0x37ab), 0x0e },
    { CCI_REG8(0x37ac), 0xa0 },
    { CCI_REG8(0x37be), 0x0a },
    { CCI_REG8(0x37bf), 0x05 },
    { CCI_REG8(0x37bb), 0x02 },
    { CCI_REG8(0x37bf), 0x05 },
    { CCI_REG8(0x37c2), 0x04 },
    { CCI_REG8(0x37c4), 0x11 },
    { CCI_REG8(0x37c5), 0x80 },
    { CCI_REG8(0x37c6), 0x14 },
    { CCI_REG8(0x37c7), 0x08 },
    { CCI_REG8(0x37c8), 0x42 },
    { CCI_REG8(0x37cd), 0x17 },
    { CCI_REG8(0x37ce), 0x01 },
    { CCI_REG8(0x37d8), 0x02 },
    { CCI_REG8(0x37d9), 0x08 },
    { CCI_REG8(0x37dc), 0x01 },
    { CCI_REG8(0x37e0), 0x0c },
    { CCI_REG8(0x37e1), 0x20 },
    { CCI_REG8(0x37e2), 0x10 },
    { CCI_REG8(0x37e3), 0x04 },
    { CCI_REG8(0x37e4), 0x28 },
    { CCI_REG8(0x37e5), 0x02 },
    { CCI_REG8(0x37ef), 0x00 },
    { CCI_REG8(0x37f4), 0x00 },
    { CCI_REG8(0x37f5), 0x00 },
    { CCI_REG8(0x37f6), 0x00 },
    { CCI_REG8(0x37f7), 0x00 },
    { CCI_REG8(0x3800), 0x01 },
    { CCI_REG8(0x3801), 0x30 },
    { CCI_REG8(0x3802), 0x00 },
    { CCI_REG8(0x3803), 0x00 },
    { CCI_REG8(0x3804), 0x0b },
    { CCI_REG8(0x3805), 0x5f },
    { CCI_REG8(0x3806), 0x07 },
    { CCI_REG8(0x3807), 0xa7 },
    { CCI_REG8(0x3808), 0x0a },
    { CCI_REG8(0x3809), 0x20 },
    { CCI_REG8(0x380a), 0x07 },
    { CCI_REG8(0x380b), 0x98 },
    { CCI_REG8(0x380c), 0x06 },
    { CCI_REG8(0x380d), 0xd0 },
    { CCI_REG8(0x3810), 0x00 },
    { CCI_REG8(0x3811), 0x08 },
    { CCI_REG8(0x3812), 0x00 },
    { CCI_REG8(0x3813), 0x08 },
    { CCI_REG8(0x3814), 0x01 },
    { CCI_REG8(0x3815), 0x01 },
    { CCI_REG8(0x3816), 0x01 },
    { CCI_REG8(0x3817), 0x01 },
    { CCI_REG8(0x3818), 0x00 },
    { CCI_REG8(0x3819), 0x00 },
    { CCI_REG8(0x381a), 0x00 },
    { CCI_REG8(0x381b), 0x01 },
    { CCI_REG8(0x3820), 0x88 },
    { CCI_REG8(0x3821), 0x00 },
    { CCI_REG8(0x3822), 0x12 },
    { CCI_REG8(0x3823), 0x08 },
    { CCI_REG8(0x3824), 0x00 },
    { CCI_REG8(0x3825), 0x20 },
    { CCI_REG8(0x3826), 0x00 },
    { CCI_REG8(0x3827), 0x08 },
    { CCI_REG8(0x3829), 0x03 },
    { CCI_REG8(0x382a), 0x00 },
    { CCI_REG8(0x382b), 0x00 },
    { CCI_REG8(0x3832), 0x08 },
    { CCI_REG8(0x3838), 0x00 },
    { CCI_REG8(0x3839), 0x00 },
    { CCI_REG8(0x383a), 0x00 },
    { CCI_REG8(0x383b), 0x00 },
    { CCI_REG8(0x383d), 0x01 },
    { CCI_REG8(0x383e), 0x00 },
    { CCI_REG8(0x383f), 0x00 },
    { CCI_REG8(0x3843), 0x00 },
    { CCI_REG8(0x3880), 0x16 },
    { CCI_REG8(0x3881), 0x00 },
    { CCI_REG8(0x3882), 0x08 },
    { CCI_REG8(0x389a), 0x00 },
    { CCI_REG8(0x389b), 0x00 },
    { CCI_REG8(0x38a2), 0x02 },
    { CCI_REG8(0x38a3), 0x02 },
    { CCI_REG8(0x38a4), 0x02 },
    { CCI_REG8(0x38a5), 0x02 },
    { CCI_REG8(0x38a7), 0x04 },
    { CCI_REG8(0x38b8), 0x02 },
    { CCI_REG8(0x3c80), 0x3e },
    { CCI_REG8(0x3c86), 0x01 },
    { CCI_REG8(0x3c87), 0x02 },
    { CCI_REG8(0x389c), 0x00 },
    { CCI_REG8(0x3ca2), 0x0c },
    { CCI_REG8(0x3d85), 0x1b },
    { CCI_REG8(0x3d8c), 0x01 },
    { CCI_REG8(0x3d8d), 0xe2 },
    { CCI_REG8(0x3f00), 0xcb },
    { CCI_REG8(0x3f03), 0x08 },
    { CCI_REG8(0x3f9e), 0x07 },
    { CCI_REG8(0x3f9f), 0x04 },
    { CCI_REG8(0x4000), 0xf3 },
    { CCI_REG8(0x4002), 0x00 },
    { CCI_REG8(0x4003), 0x40 },
    { CCI_REG8(0x4008), 0x02 },
    { CCI_REG8(0x4009), 0x0d },
    { CCI_REG8(0x400a), 0x01 },
    { CCI_REG8(0x400b), 0x00 },
    { CCI_REG8(0x4040), 0x00 },
    { CCI_REG8(0x4041), 0x07 },
    { CCI_REG8(0x4090), 0x14 },
    { CCI_REG8(0x40b0), 0x01 },
    { CCI_REG8(0x40b1), 0x01 },
    { CCI_REG8(0x40b2), 0x30 },
    { CCI_REG8(0x40b3), 0x04 },
    { CCI_REG8(0x40b4), 0xe8 },
    { CCI_REG8(0x40b5), 0x01 },
    { CCI_REG8(0x40b7), 0x07 },
    { CCI_REG8(0x40b8), 0xff },
    { CCI_REG8(0x40b9), 0x00 },
    { CCI_REG8(0x40ba), 0x00 },
    { CCI_REG8(0x4300), 0xff },
    { CCI_REG8(0x4301), 0x00 },
    { CCI_REG8(0x4302), 0x0f },
    { CCI_REG8(0x4303), 0x20 },
    { CCI_REG8(0x4304), 0x20 },
    { CCI_REG8(0x4305), 0x83 },
    { CCI_REG8(0x4306), 0x21 },
    { CCI_REG8(0x430d), 0x00 },
    { CCI_REG8(0x4505), 0xc4 },
    { CCI_REG8(0x4506), 0x00 },
    { CCI_REG8(0x4507), 0x60 },
    { CCI_REG8(0x4803), 0x00 },
    { CCI_REG8(0x4809), 0x8e },
    { CCI_REG8(0x480e), 0x00 },
    { CCI_REG8(0x4813), 0x00 },
    { CCI_REG8(0x4814), 0x2a },
    { CCI_REG8(0x481b), 0x40 },
    { CCI_REG8(0x481f), 0x30 },
    { CCI_REG8(0x4825), 0x34 },
    { CCI_REG8(0x4829), 0x64 },
    { CCI_REG8(0x4837), 0x12 },
    { CCI_REG8(0x484b), 0x07 },
    { CCI_REG8(0x4883), 0x36 },
    { CCI_REG8(0x4885), 0x03 },
    { CCI_REG8(0x488b), 0x00 },
    { CCI_REG8(0x4d06), 0x01 },
    { CCI_REG8(0x4e00), 0x2a },
    { CCI_REG8(0x4e0d), 0x00 },
    { CCI_REG8(0x5000), 0xf9 },
    { CCI_REG8(0x5001), 0x09 },
    { CCI_REG8(0x5004), 0x00 },
    { CCI_REG8(0x5005), 0x0e },
    { CCI_REG8(0x5036), 0x00 },
    { CCI_REG8(0x5080), 0x04 },
    { CCI_REG8(0x5082), 0x00 },
    { CCI_REG8(0x5180), 0x00 },
    { CCI_REG8(0x5181), 0x10 },
    { CCI_REG8(0x5182), 0x01 },
    { CCI_REG8(0x5183), 0xdf },
    { CCI_REG8(0x5184), 0x02 },
    { CCI_REG8(0x5185), 0x6c },
    { CCI_REG8(0x5189), 0x48 },
    { CCI_REG8(0x520a), 0x03 },
    { CCI_REG8(0x520b), 0x0f },
    { CCI_REG8(0x520c), 0x3f },
    { CCI_REG8(0x580b), 0x03 },
    { CCI_REG8(0x580d), 0x00 },
    { CCI_REG8(0x580f), 0x00 },
    { CCI_REG8(0x5820), 0x00 },
    { CCI_REG8(0x5821), 0x00 },
    { CCI_REG8(0x3222), 0x03 },
    { CCI_REG8(0x3208), 0x06 },
    { CCI_REG8(0x3701), 0x1d },
    { CCI_REG8(0x37ab), 0x01 },
    { CCI_REG8(0x3790), 0x21 },
    { CCI_REG8(0x38be), 0x00 },
    { CCI_REG8(0x3791), 0x5a },
    { CCI_REG8(0x37bf), 0x1c },
    { CCI_REG8(0x3610), 0x37 },
    { CCI_REG8(0x3208), 0x16 },
    { CCI_REG8(0x3208), 0x07 },
    { CCI_REG8(0x3701), 0x1d },
    { CCI_REG8(0x37ab), 0x0e },
    { CCI_REG8(0x3790), 0x21 },
    { CCI_REG8(0x38be), 0x00 },
    { CCI_REG8(0x3791), 0x5a },
    { CCI_REG8(0x37bf), 0x0a },
    { CCI_REG8(0x3610), 0x87 },
    { CCI_REG8(0x3208), 0x17 },
    { CCI_REG8(0x3208), 0x08 },
    { CCI_REG8(0x3701), 0x1d },
    { CCI_REG8(0x37ab), 0x0e },
    { CCI_REG8(0x3790), 0x21 },
    { CCI_REG8(0x38be), 0x00 },
    { CCI_REG8(0x3791), 0x5a },
    { CCI_REG8(0x37bf), 0x0a },
    { CCI_REG8(0x3610), 0x87 },
    { CCI_REG8(0x3208), 0x18 },
    { CCI_REG8(0x3208), 0x09 },
    { CCI_REG8(0x3701), 0x1d },
    { CCI_REG8(0x37ab), 0x0e },
    { CCI_REG8(0x3790), 0x28 },
    { CCI_REG8(0x38be), 0x00 },
    { CCI_REG8(0x3791), 0x63 },
    { CCI_REG8(0x37bf), 0x0a },
    { CCI_REG8(0x3610), 0x87 },
    { CCI_REG8(0x3208), 0x19 },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct os05b10 {
    pub dev: *mut device,
    pub cci: *mut regmap,
    pub sd: v4l2_subdev,
    pub pad: media_pad,
    pub xclk: *mut clk,
    pub client: *mut i2c_client,
    pub reset_gpio: *mut gpio_desc,
    pub supplies: [regulator_bulk_data; ARRAY_SIZE(os05b10_supply_name)],
// V4L2 Controls
    pub handler: v4l2_ctrl_handler,
    pub link_freq: *mut v4l2_ctrl,
    pub hblank: *mut v4l2_ctrl,
    pub vblank: *mut v4l2_ctrl,
    pub gain: *mut v4l2_ctrl,
    pub exposure: *mut v4l2_ctrl,
    pub link_freq_index: u32,
    pub data_lanes: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct os05b10_mode {
    pub width: u32,
    pub height: u32,
    pub vts: u32,
    pub hts: u32,
    pub exp: u32,
    pub bpp: u8,
}

    static const struct os05b10_mode supported_modes_10bit[] = {
    {
    .width = 2592,
    .height = 1944,
    .vts = 2006,
    .hts = 1744,
    .exp = 1944,
    .bpp = 10,
    },
    };
    static const s64 link_frequencies[] = {
    OS05B10_LINK_FREQ_600MHZ,
    };
    static const u32 os05b10_mbus_codes[] = {
    MEDIA_BUS_FMT_SBGGR10_1X10,
    };
    static inline struct os05b10 *to_os05b10(struct v4l2_subdev *sd)
    {
    return container_of_const(sd, struct os05b10, sd);
    };
#[no_mangle]
unsafe extern "C" fn os05b10_set_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    static int os05b10_set_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct os05b10 *os05b10 = container_of_const(ctrl.handler,
    struct os05b10, handler);
    struct v4l2_subdev_state *state;
    struct v4l2_mbus_framefmt *fmt;
    int vmax, ret;
    state = v4l2_subdev_get_locked_active_state(&os05b10.sd);
    fmt = v4l2_subdev_state_get_format(state, 0);
    if (ctrl.id == V4L2_CID_VBLANK) {
// Honour the VBLANK limits when setting exposure.
    let mut max: i64 = fmt.height + ctrl.val - OS05B10_EXPOSURE_MARGIN;
    ret = __v4l2_ctrl_modify_range(os05b10.exposure,
    os05b10.exposure.minimum, max,
    os05b10.exposure.step,
    os05b10.exposure.default_value);
    if (ret)
    return ret;
    }
    if (pm_runtime_get_if_in_use(os05b10.dev) == 0)
    return 0;
    switch (ctrl.id) {
    case V4L2_CID_VBLANK:
    vmax = fmt.height + ctrl.val;
    ret = cci_write(os05b10.cci, OS05B10_REG_VTS, vmax, core::ptr::null_mut());
    break;
    case V4L2_CID_ANALOGUE_GAIN:
    ret = cci_write(os05b10.cci, OS05B10_REG_ANALOG_GAIN,
    ctrl.val, core::ptr::null_mut());
    break;
    case V4L2_CID_EXPOSURE:
    ret = cci_write(os05b10.cci, OS05B10_REG_EXPOSURE,
    ctrl.val, core::ptr::null_mut());
    break;
    default:
    ret = -EINVAL;
    break;
    }
    pm_runtime_put(os05b10.dev);
    return ret;
    }
    static int os05b10_enum_mbus_code(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_mbus_code_enum *code)
    {
    if (code.index >= ARRAY_SIZE(os05b10_mbus_codes))
    return -EINVAL;
    code.code = os05b10_mbus_codes[code.index];
    return 0;
    }
    static int os05b10_set_framing_limits(struct os05b10 *os05b10,
    const struct os05b10_mode *mode)
    {
    u32 hblank, vblank, vblank_max, max_exp;
    int ret;
    hblank = mode.hts - mode.width;
    ret = __v4l2_ctrl_modify_range(os05b10.hblank, hblank, hblank, 1,
    hblank);
    if (ret)
    return ret;
    vblank = mode.vts - mode.height;
    vblank_max = OS05B10_VTS_MAX - mode.height;
    ret = __v4l2_ctrl_modify_range(os05b10.vblank, 0, vblank_max, 1,
    vblank);
    if (ret)
    return ret;
    max_exp = mode.vts - OS05B10_EXPOSURE_MARGIN;
    return __v4l2_ctrl_modify_range(os05b10.exposure,
    OS05B10_EXPOSURE_MIN, max_exp,
    OS05B10_EXPOSURE_STEP, mode.exp);
    }
    static int os05b10_set_pad_format(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_format *fmt)
    {
    const struct os05b10_mode *mode = &supported_modes_10bit[0];
    struct os05b10 *os05b10 = to_os05b10(sd);
    struct v4l2_mbus_framefmt *format;
    int ret;
    fmt.format.width = mode.width;
    fmt.format.height = mode.height;
    fmt.format.field = V4L2_FIELD_NONE;
    fmt.format.colorspace = V4L2_COLORSPACE_RAW;
    fmt.format.quantization = V4L2_QUANTIZATION_FULL_RANGE;
    fmt.format.xfer_func = V4L2_XFER_FUNC_NONE;
    format = v4l2_subdev_state_get_format(sd_state, 0);
    if (fmt.which == V4L2_SUBDEV_FORMAT_ACTIVE) {
    ret = os05b10_set_framing_limits(os05b10, mode);
    if (ret)
    return ret;
    }
// format = fmt->format;
    return 0;
    }
    static int os05b10_get_selection(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_selection *sel)
    {
    switch (sel.target) {
    case V4L2_SEL_TGT_NATIVE_SIZE:
    case V4L2_SEL_TGT_CROP_BOUNDS:
    sel.r = os05b10_native_area;
    return 0;
    case V4L2_SEL_TGT_CROP:
    case V4L2_SEL_TGT_CROP_DEFAULT:
    sel.r = os05b10_active_area;
    return 0;
    default:
    return -EINVAL;
    }
    }
    static int os05b10_enum_frame_size(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_frame_size_enum *fse)
    {
    if (fse.index >= ARRAY_SIZE(supported_modes_10bit))
    return -EINVAL;
    fse.min_width = supported_modes_10bit[fse.index].width;
    fse.max_width = fse.min_width;
    fse.min_height = supported_modes_10bit[fse.index].height;
    fse.max_height = fse.min_height;
    return 0;
    }
    static int os05b10_enable_streams(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *state,
    u32 pad, u64 streams_mask)
    {
    struct os05b10 *os05b10 = to_os05b10(sd);
    int ret;
    ret = pm_runtime_resume_and_get(os05b10.dev);
    if (ret < 0)
    return ret;
// Write common registers
    ret = cci_multi_reg_write(os05b10.cci, os05b10_common_regs,
    ARRAY_SIZE(os05b10_common_regs), core::ptr::null_mut());
    if (ret) {
    dev_err(os05b10.dev, "failed to write common registers\n");
    goto err_rpm_put;
    }
// Apply customized user controls
    ret = __v4l2_ctrl_handler_setup(os05b10.sd.ctrl_handler);
    if (ret)
    goto err_rpm_put;
// Stream ON
    ret = cci_write(os05b10.cci, OS05B10_REG_CTRL_MODE,
    OS05B10_MODE_STREAMING, core::ptr::null_mut());
    if (ret)
    goto err_rpm_put;
    return 0;
    err_rpm_put:
    pm_runtime_put(os05b10.dev);
    return ret;
    }
    static int os05b10_disable_streams(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *state,
    u32 pad, u64 streams_mask)
    {
    struct os05b10 *os05b10 = to_os05b10(sd);
    int ret;
    ret = cci_write(os05b10.cci, OS05B10_REG_CTRL_MODE,
    OS05B10_MODE_STANDBY, core::ptr::null_mut());
    if (ret)
    dev_err(os05b10.dev, "failed to set stream off\n");
    pm_runtime_put(os05b10.dev);
    return 0;
    }
    static int os05b10_init_state(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *state)
    {
    struct v4l2_mbus_framefmt *format;
    const struct os05b10_mode *mode;
// Initialize try_fmt
    format = v4l2_subdev_state_get_format(state, 0);
    mode = &supported_modes_10bit[0];
    format.code = MEDIA_BUS_FMT_SBGGR10_1X10;
// Update image pad formate
    format.width = mode.width;
    format.height = mode.height;
    format.field = V4L2_FIELD_NONE;
    format.colorspace = V4L2_COLORSPACE_RAW;
    format.quantization = V4L2_QUANTIZATION_FULL_RANGE;
    format.xfer_func = V4L2_XFER_FUNC_NONE;
    return 0;
    }
    static const struct v4l2_subdev_video_ops os05b10_video_ops = {
    .s_stream = v4l2_subdev_s_stream_helper,
    };
    static const struct v4l2_subdev_pad_ops os05b10_pad_ops = {
    .enum_mbus_code = os05b10_enum_mbus_code,
    .get_fmt = v4l2_subdev_get_fmt,
    .set_fmt = os05b10_set_pad_format,
    .get_selection = os05b10_get_selection,
    .enum_frame_size = os05b10_enum_frame_size,
    .enable_streams = os05b10_enable_streams,
    .disable_streams = os05b10_disable_streams,
    };
    static const struct v4l2_subdev_internal_ops os05b10_internal_ops = {
    .init_state = os05b10_init_state,
    };
    static const struct v4l2_subdev_ops os05b10_subdev_ops = {
    .video = &os05b10_video_ops,
    .pad = &os05b10_pad_ops,
    };
    static const struct v4l2_ctrl_ops os05b10_ctrl_ops = {
    .s_ctrl = os05b10_set_ctrl,
    };
#[no_mangle]
unsafe extern "C" fn os05b10_identify_module(os05b10: *mut os05b10) -> c_int {
    static int os05b10_identify_module(struct os05b10 *os05b10)
    {
    int ret;
    u64 val;
    ret = cci_read(os05b10.cci, OS05B10_REG_CHIP_ID, &val, core::ptr::null_mut());
    if (ret)
    return dev_err_probe(os05b10.dev, ret,
    "failed to read chip id %x\n",
    OS05B10_CHIP_ID);
    if (val != OS05B10_CHIP_ID)
    return dev_err_probe(os05b10.dev, -ENODEV,
    "chip id mismatch: %x!=%llx\n",
    OS05B10_CHIP_ID, val);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn os05b10_power_on(dev: *mut device) -> c_int {
    static int os05b10_power_on(struct device *dev)
    {
    struct v4l2_subdev *sd = dev_get_drvdata(dev);
    struct os05b10 *os05b10 = to_os05b10(sd);
    unsigned long delay_us;
    int ret;
// Enable power rails
    ret = regulator_bulk_enable(ARRAY_SIZE(os05b10_supply_name),
    os05b10.supplies);
    if (ret) {
    dev_err(os05b10.dev, "failed to enable regulators\n");
    return ret;
    }
// Enable xclk
    ret = clk_prepare_enable(os05b10.xclk);
    if (ret) {
    dev_err(os05b10.dev, "failed to enable clock\n");
    goto err_regulator_off;
    }
    gpiod_set_value_cansleep(os05b10.reset_gpio, 0);
// Delay T1
    fsleep(5 * USEC_PER_MSEC);
// Delay T2 (8192 cycles before SCCB/I2C access)
    delay_us = DIV_ROUND_UP(8192, OS05B10_XCLK_FREQ / 1000 / 1000);
    usleep_range(delay_us, delay_us * 2);
    return 0;
    err_regulator_off:
    regulator_bulk_disable(ARRAY_SIZE(os05b10_supply_name),
    os05b10.supplies);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn os05b10_power_off(dev: *mut device) -> c_int {
    static int os05b10_power_off(struct device *dev)
    {
    struct v4l2_subdev *sd = dev_get_drvdata(dev);
    struct os05b10 *os05b10 = to_os05b10(sd);
    gpiod_set_value_cansleep(os05b10.reset_gpio, 1);
    regulator_bulk_disable(ARRAY_SIZE(os05b10_supply_name),
    os05b10.supplies);
    clk_disable_unprepare(os05b10.xclk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn os05b10_parse_endpoint(os05b10: *mut os05b10) -> c_int {
    static int os05b10_parse_endpoint(struct os05b10 *os05b10)
    {
    struct v4l2_fwnode_endpoint bus_cfg = {
    .bus_type = V4L2_MBUS_CSI2_DPHY
    };
    unsigned long link_freq_bitmap;
    struct fwnode_handle *ep;
    int ret;
    ep = fwnode_graph_get_endpoint_by_id(dev_fwnode(os05b10.dev), 0, 0, 0);
    if (!ep) {
    dev_err(os05b10.dev, "Failed to get next endpoint\n");
    return -EINVAL;
    }
    ret = v4l2_fwnode_endpoint_alloc_parse(ep, &bus_cfg);
    fwnode_handle_put(ep);
    if (ret)
    return ret;
    if (bus_cfg.bus.mipi_csi2.num_data_lanes != 4) {
    ret = dev_err_probe(os05b10.dev, -EINVAL,
    "only 4 data lanes are supported\n");
    goto error_out;
    }
    os05b10.data_lanes = bus_cfg.bus.mipi_csi2.num_data_lanes;
    ret = v4l2_link_freq_to_bitmap(os05b10.dev, bus_cfg.link_frequencies,
    bus_cfg.nr_of_link_frequencies,
    link_frequencies,
    ARRAY_SIZE(link_frequencies),
    &link_freq_bitmap);
    if (ret) {
    dev_err(os05b10.dev, "only 600MHz frequency is available\n");
    goto error_out;
    }
    os05b10.link_freq_index = __ffs(link_freq_bitmap);
    error_out:
    v4l2_fwnode_endpoint_free(&bus_cfg);
    return ret;
    }
    static u64 os05b10_pixel_rate(struct os05b10 *os05b10,
    const struct os05b10_mode *mode)
    {
    let mut link_freq: u64 = link_frequencies[os05b10.link_freq_index];
    let mut pixel_rate: u64 = div_u64(link_freq * 2 * os05b10.data_lanes, mode.bpp);
    dev_dbg(os05b10.dev,
    "link_freq=%llu bpp=%u lanes=%u pixel_rate=%llu\n",
    link_freq, mode.bpp, os05b10.data_lanes, pixel_rate);
    return pixel_rate;
    }
#[no_mangle]
unsafe extern "C" fn os05b10_init_controls(os05b10: *mut os05b10) -> c_int {
    static int os05b10_init_controls(struct os05b10 *os05b10)
    {
    const struct os05b10_mode *mode = &supported_modes_10bit[0];
    u64 hblank_def, vblank_def, exp_max, pixel_rate;
    struct v4l2_fwnode_device_properties props;
    struct v4l2_ctrl_handler *ctrl_hdlr;
    int ret;
    ctrl_hdlr = &os05b10.handler;
    v4l2_ctrl_handler_init(ctrl_hdlr, 8);
    pixel_rate = os05b10_pixel_rate(os05b10, mode);
    v4l2_ctrl_new_std(ctrl_hdlr, &os05b10_ctrl_ops, V4L2_CID_PIXEL_RATE,
    pixel_rate, pixel_rate, 1, pixel_rate);
    os05b10.link_freq = v4l2_ctrl_new_int_menu(ctrl_hdlr, &os05b10_ctrl_ops,
    V4L2_CID_LINK_FREQ,
    ARRAY_SIZE(link_frequencies) - 1,
    os05b10.link_freq_index,
    link_frequencies);
    if (os05b10.link_freq)
    os05b10.link_freq.flags |= V4L2_CTRL_FLAG_READ_ONLY;
    hblank_def = mode.hts - mode.width;
    os05b10.hblank = v4l2_ctrl_new_std(ctrl_hdlr, core::ptr::null_mut(), V4L2_CID_HBLANK,
    hblank_def, hblank_def,
    1, hblank_def);
    if (os05b10.hblank)
    os05b10.hblank.flags |= V4L2_CTRL_FLAG_READ_ONLY;
    vblank_def = mode.vts - mode.height;
    os05b10.vblank = v4l2_ctrl_new_std(ctrl_hdlr, &os05b10_ctrl_ops,
    V4L2_CID_VBLANK, vblank_def,
    OS05B10_VTS_MAX - mode.height,
    1, vblank_def);
    exp_max = mode.vts - OS05B10_EXPOSURE_MARGIN;
    os05b10.exposure = v4l2_ctrl_new_std(ctrl_hdlr, &os05b10_ctrl_ops,
    V4L2_CID_EXPOSURE,
    OS05B10_EXPOSURE_MIN,
    exp_max, OS05B10_EXPOSURE_STEP,
    mode.exp);
    os05b10.gain = v4l2_ctrl_new_std(ctrl_hdlr, &os05b10_ctrl_ops,
    V4L2_CID_ANALOGUE_GAIN,
    OS05B10_ANALOG_GAIN_MIN,
    OS05B10_ANALOG_GAIN_MAX,
    OS05B10_ANALOG_GAIN_STEP,
    OS05B10_ANALOG_GAIN_DEFAULT);
    if (ctrl_hdlr.error) {
    ret = ctrl_hdlr.error;
    dev_err(os05b10.dev, "control init failed (%d)\n", ret);
    goto error;
    }
    ret = v4l2_fwnode_device_parse(os05b10.dev, &props);
    if (ret)
    goto error;
    ret = v4l2_ctrl_new_fwnode_properties(ctrl_hdlr, &os05b10_ctrl_ops,
    &props);
    if (ret)
    goto error;
    os05b10.sd.ctrl_handler = ctrl_hdlr;
    return 0;
    error:
    v4l2_ctrl_handler_free(ctrl_hdlr);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn os05b10_probe(client: *mut i2c_client) -> c_int {
    static int os05b10_probe(struct i2c_client *client)
    {
    struct os05b10 *os05b10;
    unsigned int xclk_freq;
    int ret;
    os05b10 = devm_kzalloc(&client.dev, sizeof(*os05b10), GFP_KERNEL);
    if (!os05b10)
    return -ENOMEM;
    os05b10.client = client;
    os05b10.dev = &client.dev;
    v4l2_i2c_subdev_init(&os05b10.sd, client, &os05b10_subdev_ops);
    os05b10.cci = devm_cci_regmap_init_i2c(client, 16);
    if (IS_ERR(os05b10.cci))
    return dev_err_probe(os05b10.dev, PTR_ERR(os05b10.cci),
    "failed to initialize CCI\n");
    os05b10.xclk = devm_v4l2_sensor_clk_get(os05b10.dev, core::ptr::null_mut());
    if (IS_ERR(os05b10.xclk))
    return dev_err_probe(os05b10.dev, PTR_ERR(os05b10.xclk),
    "failed to get xclk\n");
    xclk_freq = clk_get_rate(os05b10.xclk);
    if (xclk_freq != OS05B10_XCLK_FREQ)
    return dev_err_probe(os05b10.dev, -EINVAL,
    "xclk frequency not supported: %d Hz\n",
    xclk_freq);
    for (unsigned int i = 0; i < ARRAY_SIZE(os05b10_supply_name); i++)
    os05b10.supplies[i].supply = os05b10_supply_name[i];
    ret = devm_regulator_bulk_get(os05b10.dev,
    ARRAY_SIZE(os05b10_supply_name),
    os05b10.supplies);
    if (ret)
    return dev_err_probe(os05b10.dev, ret,
    "failed to get regulators\n");
    ret = os05b10_parse_endpoint(os05b10);
    if (ret)
    return dev_err_probe(os05b10.dev, ret,
    "failed to parse endpoint configuration\n");
    os05b10.reset_gpio = devm_gpiod_get_optional(&client.dev, "reset",
    GPIOD_OUT_HIGH);
    if (IS_ERR(os05b10.reset_gpio))
    return dev_err_probe(os05b10.dev, PTR_ERR(os05b10.reset_gpio),
    "failed to get reset GPIO\n");
    ret = os05b10_power_on(os05b10.dev);
    if (ret)
    return ret;
    ret = os05b10_identify_module(os05b10);
    if (ret)
    goto error_power_off;
// This needs the pm runtime to be registered.
    ret = os05b10_init_controls(os05b10);
    if (ret)
    goto error_power_off;
// Initialize subdev
    os05b10.sd.flags |= V4L2_SUBDEV_FL_HAS_DEVNODE;
    os05b10.sd.entity.function = MEDIA_ENT_F_CAM_SENSOR;
    os05b10.sd.internal_ops = &os05b10_internal_ops;
    os05b10.pad.flags = MEDIA_PAD_FL_SOURCE;
    ret = media_entity_pads_init(&os05b10.sd.entity, 1, &os05b10.pad);
    if (ret) {
    dev_err_probe(os05b10.dev, ret,
    "failed to init entity pads\n");
    goto error_handler_free;
    }
    os05b10.sd.state_lock = os05b10.handler.lock;
    ret = v4l2_subdev_init_finalize(&os05b10.sd);
    if (ret < 0) {
    dev_err_probe(os05b10.dev, ret, "subdev init error\n");
    goto error_media_entity;
    }
    pm_runtime_set_active(os05b10.dev);
    pm_runtime_enable(os05b10.dev);
    ret = v4l2_async_register_subdev_sensor(&os05b10.sd);
    if (ret < 0) {
    dev_err_probe(os05b10.dev, ret,
    "failed to register os05b10 sub-device\n");
    goto error_subdev_cleanup;
    }
    pm_runtime_idle(os05b10.dev);
    return 0;
    error_subdev_cleanup:
    v4l2_subdev_cleanup(&os05b10.sd);
    pm_runtime_disable(os05b10.dev);
    pm_runtime_set_suspended(os05b10.dev);
    error_media_entity:
    media_entity_cleanup(&os05b10.sd.entity);
    error_handler_free:
    v4l2_ctrl_handler_free(os05b10.sd.ctrl_handler);
    error_power_off:
    os05b10_power_off(os05b10.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn os05b10_remove(client: *mut i2c_client) {
    static void os05b10_remove(struct i2c_client *client)
    {
    struct v4l2_subdev *sd = i2c_get_clientdata(client);
    struct os05b10 *os05b10 = to_os05b10(sd);
    v4l2_async_unregister_subdev(sd);
    v4l2_subdev_cleanup(&os05b10.sd);
    media_entity_cleanup(&sd.entity);
    v4l2_ctrl_handler_free(os05b10.sd.ctrl_handler);
    pm_runtime_disable(&client.dev);
    if (!pm_runtime_status_suspended(&client.dev)) {
    os05b10_power_off(&client.dev);
    pm_runtime_set_suspended(&client.dev);
    }
    }
    static DEFINE_RUNTIME_DEV_PM_OPS(os05b10_pm_ops, os05b10_power_off,
    os05b10_power_on, core::ptr::null_mut());
    static const struct of_device_id os05b10_id[] = {
    { .compatible = "ovti,os05b10" },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(of, os05b10_id);
    static struct i2c_driver os05b10_driver = {
    .driver = {
    .name = "os05b10",
    .pm = pm_ptr(&os05b10_pm_ops),
    .of_match_table = os05b10_id,
    },
    .probe = os05b10_probe,
    .remove = os05b10_remove,
    };
    module_i2c_driver(os05b10_driver);
    MODULE_DESCRIPTION("OS05B10 Camera Sensor Driver");
    MODULE_AUTHOR("Himanshu Bhavani <himanshu.bhavani@siliconsignals.io>");
    MODULE_AUTHOR("Elgin Perumbilly <elgin.perumbilly@siliconsignals.io>");
    MODULE_LICENSE("GPL");
