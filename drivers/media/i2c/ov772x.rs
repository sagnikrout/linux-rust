//! Automatically rewritten from C to Rust
//! Source: drivers/media/i2c/ov772x.c
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
// ov772x Camera Driver
//
// Copyright (C) 2017 Jacopo Mondi <jacopo+renesas@jmondi.org>
//
// Copyright (C) 2008 Renesas Solutions Corp.
// Kuninori Morimoto <morimoto.kuninori@renesas.com>
//
// Based on ov7670 and soc_camera_platform driver,
//
// Copyright 2006-7 Jonathan Corbet <corbet@lwn.net>
// Copyright (C) 2008 Magnus Damm
// Copyright (C) 2008, Guennadi Liakhovetski <kernel@pengutronix.de>
//

//
// register offset
//
pub const GAIN: c_uint = 0x00 /* AGC - Gain control gain setting */;
pub const BLUE: c_uint = 0x01 /* AWB - Blue channel gain setting */;
pub const RED: c_uint = 0x02 /* AWB - Red   channel gain setting */;
pub const GREEN: c_uint = 0x03 /* AWB - Green channel gain setting */;
pub const COM1: c_uint = 0x04 /* Common control 1 */;
pub const BAVG: c_uint = 0x05 /* U/B Average Level */;
pub const GAVG: c_uint = 0x06 /* Y/Gb Average Level */;
pub const RAVG: c_uint = 0x07 /* V/R Average Level */;
pub const AECH: c_uint = 0x08 /* Exposure Value - AEC MSBs */;
pub const COM2: c_uint = 0x09 /* Common control 2 */;
pub const PID: c_uint = 0x0A /* Product ID Number MSB */;
pub const VER: c_uint = 0x0B /* Product ID Number LSB */;
pub const COM3: c_uint = 0x0C /* Common control 3 */;
pub const COM4: c_uint = 0x0D /* Common control 4 */;
pub const COM5: c_uint = 0x0E /* Common control 5 */;
pub const COM6: c_uint = 0x0F /* Common control 6 */;
pub const AEC: c_uint = 0x10 /* Exposure Value */;
pub const CLKRC: c_uint = 0x11 /* Internal clock */;
pub const COM7: c_uint = 0x12 /* Common control 7 */;
pub const COM8: c_uint = 0x13 /* Common control 8 */;
pub const COM9: c_uint = 0x14 /* Common control 9 */;
pub const COM10: c_uint = 0x15 /* Common control 10 */;
pub const REG16: c_uint = 0x16 /* Register 16 */;
pub const HSTART: c_uint = 0x17 /* Horizontal sensor size */;
pub const HSIZE: c_uint = 0x18 /* Horizontal frame (HREF column) end high 8-bit */;
pub const VSTART: c_uint = 0x19 /* Vertical frame (row) start high 8-bit */;
pub const VSIZE: c_uint = 0x1A /* Vertical sensor size */;
pub const PSHFT: c_uint = 0x1B /* Data format - pixel delay select */;
pub const MIDH: c_uint = 0x1C /* Manufacturer ID byte - high */;
pub const MIDL: c_uint = 0x1D /* Manufacturer ID byte - low  */;
pub const LAEC: c_uint = 0x1F /* Fine AEC value */;
pub const COM11: c_uint = 0x20 /* Common control 11 */;
pub const BDBASE: c_uint = 0x22 /* Banding filter Minimum AEC value */;
pub const DBSTEP: c_uint = 0x23 /* Banding filter Maximum Setp */;
pub const AEW: c_uint = 0x24 /* AGC/AEC - Stable operating region (upper limit) */;
pub const AEB: c_uint = 0x25 /* AGC/AEC - Stable operating region (lower limit) */;
pub const VPT: c_uint = 0x26 /* AGC/AEC Fast mode operating region */;
pub const REG28: c_uint = 0x28 /* Register 28 */;
pub const HOUTSIZE: c_uint = 0x29 /* Horizontal data output size MSBs */;
pub const EXHCH: c_uint = 0x2A /* Dummy pixel insert MSB */;
pub const EXHCL: c_uint = 0x2B /* Dummy pixel insert LSB */;
pub const VOUTSIZE: c_uint = 0x2C /* Vertical data output size MSBs */;
pub const ADVFL: c_uint = 0x2D /* LSB of insert dummy lines in Vertical direction */;
pub const ADVFH: c_uint = 0x2E /* MSG of insert dummy lines in Vertical direction */;
pub const YAVE: c_uint = 0x2F /* Y/G Channel Average value */;
pub const LUMHTH: c_uint = 0x30 /* Histogram AEC/AGC Luminance high level threshold */;
pub const LUMLTH: c_uint = 0x31 /* Histogram AEC/AGC Luminance low  level threshold */;
pub const HREF: c_uint = 0x32 /* Image start and size control */;
pub const DM_LNL: c_uint = 0x33 /* Dummy line low  8 bits */;
pub const DM_LNH: c_uint = 0x34 /* Dummy line high 8 bits */;
pub const ADOFF_B: c_uint = 0x35 /* AD offset compensation value for B  channel */;
pub const ADOFF_R: c_uint = 0x36 /* AD offset compensation value for R  channel */;
pub const ADOFF_GB: c_uint = 0x37 /* AD offset compensation value for Gb channel */;
pub const ADOFF_GR: c_uint = 0x38 /* AD offset compensation value for Gr channel */;
pub const OFF_B: c_uint = 0x39 /* Analog process B  channel offset value */;
pub const OFF_R: c_uint = 0x3A /* Analog process R  channel offset value */;
pub const OFF_GB: c_uint = 0x3B /* Analog process Gb channel offset value */;
pub const OFF_GR: c_uint = 0x3C /* Analog process Gr channel offset value */;
pub const COM12: c_uint = 0x3D /* Common control 12 */;
pub const COM13: c_uint = 0x3E /* Common control 13 */;
pub const COM14: c_uint = 0x3F /* Common control 14 */;
pub const COM15: c_uint = 0x40 /* Common control 15*/;
pub const COM16: c_uint = 0x41 /* Common control 16 */;
pub const TGT_B: c_uint = 0x42 /* BLC blue channel target value */;
pub const TGT_R: c_uint = 0x43 /* BLC red  channel target value */;
pub const TGT_GB: c_uint = 0x44 /* BLC Gb   channel target value */;
pub const TGT_GR: c_uint = 0x45 /* BLC Gr   channel target value */;
// for ov7720
pub const LCC0: c_uint = 0x46 /* Lens correction control 0 */;
pub const LCC1: c_uint = 0x47 /* Lens correction option 1 - X coordinate */;
pub const LCC2: c_uint = 0x48 /* Lens correction option 2 - Y coordinate */;
pub const LCC3: c_uint = 0x49 /* Lens correction option 3 */;
pub const LCC4: c_uint = 0x4A /* Lens correction option 4 - radius of the circular */;
pub const LCC5: c_uint = 0x4B /* Lens correction option 5 */;
pub const LCC6: c_uint = 0x4C /* Lens correction option 6 */;
// for ov7725
pub const LC_CTR: c_uint = 0x46 /* Lens correction control */;
pub const LC_XC: c_uint = 0x47 /* X coordinate of lens correction center relative */;
pub const LC_YC: c_uint = 0x48 /* Y coordinate of lens correction center relative */;
pub const LC_COEF: c_uint = 0x49 /* Lens correction coefficient */;
pub const LC_RADI: c_uint = 0x4A /* Lens correction radius */;
pub const LC_COEFB: c_uint = 0x4B /* Lens B channel compensation coefficient */;
pub const LC_COEFR: c_uint = 0x4C /* Lens R channel compensation coefficient */;
pub const FIXGAIN: c_uint = 0x4D /* Analog fix gain amplifer */;
pub const AREF0: c_uint = 0x4E /* Sensor reference control */;
pub const AREF1: c_uint = 0x4F /* Sensor reference current control */;
pub const AREF2: c_uint = 0x50 /* Analog reference control */;
pub const AREF3: c_uint = 0x51 /* ADC    reference control */;
pub const AREF4: c_uint = 0x52 /* ADC    reference control */;
pub const AREF5: c_uint = 0x53 /* ADC    reference control */;
pub const AREF6: c_uint = 0x54 /* Analog reference control */;
pub const AREF7: c_uint = 0x55 /* Analog reference control */;
pub const UFIX: c_uint = 0x60 /* U channel fixed value output */;
pub const VFIX: c_uint = 0x61 /* V channel fixed value output */;
pub const AWBB_BLK: c_uint = 0x62 /* AWB option for advanced AWB */;
pub const AWB_CTRL0: c_uint = 0x63 /* AWB control byte 0 */;
pub const DSP_CTRL1: c_uint = 0x64 /* DSP control byte 1 */;
pub const DSP_CTRL2: c_uint = 0x65 /* DSP control byte 2 */;
pub const DSP_CTRL3: c_uint = 0x66 /* DSP control byte 3 */;
pub const DSP_CTRL4: c_uint = 0x67 /* DSP control byte 4 */;
pub const AWB_BIAS: c_uint = 0x68 /* AWB BLC level clip */;
pub const AWB_CTRL1: c_uint = 0x69 /* AWB control  1 */;
pub const AWB_CTRL2: c_uint = 0x6A /* AWB control  2 */;
pub const AWB_CTRL3: c_uint = 0x6B /* AWB control  3 */;
pub const AWB_CTRL4: c_uint = 0x6C /* AWB control  4 */;
pub const AWB_CTRL5: c_uint = 0x6D /* AWB control  5 */;
pub const AWB_CTRL6: c_uint = 0x6E /* AWB control  6 */;
pub const AWB_CTRL7: c_uint = 0x6F /* AWB control  7 */;
pub const AWB_CTRL8: c_uint = 0x70 /* AWB control  8 */;
pub const AWB_CTRL9: c_uint = 0x71 /* AWB control  9 */;
pub const AWB_CTRL10: c_uint = 0x72 /* AWB control 10 */;
pub const AWB_CTRL11: c_uint = 0x73 /* AWB control 11 */;
pub const AWB_CTRL12: c_uint = 0x74 /* AWB control 12 */;
pub const AWB_CTRL13: c_uint = 0x75 /* AWB control 13 */;
pub const AWB_CTRL14: c_uint = 0x76 /* AWB control 14 */;
pub const AWB_CTRL15: c_uint = 0x77 /* AWB control 15 */;
pub const AWB_CTRL16: c_uint = 0x78 /* AWB control 16 */;
pub const AWB_CTRL17: c_uint = 0x79 /* AWB control 17 */;
pub const AWB_CTRL18: c_uint = 0x7A /* AWB control 18 */;
pub const AWB_CTRL19: c_uint = 0x7B /* AWB control 19 */;
pub const AWB_CTRL20: c_uint = 0x7C /* AWB control 20 */;
pub const AWB_CTRL21: c_uint = 0x7D /* AWB control 21 */;
pub const GAM1: c_uint = 0x7E /* Gamma Curve  1st segment input end point */;
pub const GAM2: c_uint = 0x7F /* Gamma Curve  2nd segment input end point */;
pub const GAM3: c_uint = 0x80 /* Gamma Curve  3rd segment input end point */;
pub const GAM4: c_uint = 0x81 /* Gamma Curve  4th segment input end point */;
pub const GAM5: c_uint = 0x82 /* Gamma Curve  5th segment input end point */;
pub const GAM6: c_uint = 0x83 /* Gamma Curve  6th segment input end point */;
pub const GAM7: c_uint = 0x84 /* Gamma Curve  7th segment input end point */;
pub const GAM8: c_uint = 0x85 /* Gamma Curve  8th segment input end point */;
pub const GAM9: c_uint = 0x86 /* Gamma Curve  9th segment input end point */;
pub const GAM10: c_uint = 0x87 /* Gamma Curve 10th segment input end point */;
pub const GAM11: c_uint = 0x88 /* Gamma Curve 11th segment input end point */;
pub const GAM12: c_uint = 0x89 /* Gamma Curve 12th segment input end point */;
pub const GAM13: c_uint = 0x8A /* Gamma Curve 13th segment input end point */;
pub const GAM14: c_uint = 0x8B /* Gamma Curve 14th segment input end point */;
pub const GAM15: c_uint = 0x8C /* Gamma Curve 15th segment input end point */;
pub const SLOP: c_uint = 0x8D /* Gamma curve highest segment slope */;
pub const DNSTH: c_uint = 0x8E /* De-noise threshold */;
pub const EDGE_STRNGT: c_uint = 0x8F /* Edge strength  control when manual mode */;
pub const EDGE_TRSHLD: c_uint = 0x90 /* Edge threshold control when manual mode */;
pub const DNSOFF: c_uint = 0x91 /* Auto De-noise threshold control */;
pub const EDGE_UPPER: c_uint = 0x92 /* Edge strength upper limit when Auto mode */;
pub const EDGE_LOWER: c_uint = 0x93 /* Edge strength lower limit when Auto mode */;
pub const MTX1: c_uint = 0x94 /* Matrix coefficient 1 */;
pub const MTX2: c_uint = 0x95 /* Matrix coefficient 2 */;
pub const MTX3: c_uint = 0x96 /* Matrix coefficient 3 */;
pub const MTX4: c_uint = 0x97 /* Matrix coefficient 4 */;
pub const MTX5: c_uint = 0x98 /* Matrix coefficient 5 */;
pub const MTX6: c_uint = 0x99 /* Matrix coefficient 6 */;
pub const MTX_CTRL: c_uint = 0x9A /* Matrix control */;
pub const BRIGHT: c_uint = 0x9B /* Brightness control */;
pub const CNTRST: c_uint = 0x9C /* Contrast contrast */;
pub const CNTRST_CTRL: c_uint = 0x9D /* Contrast contrast center */;
pub const UVAD_J0: c_uint = 0x9E /* Auto UV adjust contrast 0 */;
pub const UVAD_J1: c_uint = 0x9F /* Auto UV adjust contrast 1 */;
pub const SCAL0: c_uint = 0xA0 /* Scaling control 0 */;
pub const SCAL1: c_uint = 0xA1 /* Scaling control 1 */;
pub const SCAL2: c_uint = 0xA2 /* Scaling control 2 */;
pub const FIFODLYM: c_uint = 0xA3 /* FIFO manual mode delay control */;
pub const FIFODLYA: c_uint = 0xA4 /* FIFO auto   mode delay control */;
pub const SDE: c_uint = 0xA6 /* Special digital effect control */;
pub const USAT: c_uint = 0xA7 /* U component saturation control */;
pub const VSAT: c_uint = 0xA8 /* V component saturation control */;
// for ov7720
pub const HUE0: c_uint = 0xA9 /* Hue control 0 */;
pub const HUE1: c_uint = 0xAA /* Hue control 1 */;
// for ov7725
pub const HUECOS: c_uint = 0xA9 /* Cosine value */;
pub const HUESIN: c_uint = 0xAA /* Sine value */;
pub const SIGN: c_uint = 0xAB /* Sign bit for Hue and contrast */;
pub const DSPAUTO: c_uint = 0xAC /* DSP auto function ON/OFF control */;
//
// register detail
//
// COM2
pub const SOFT_SLEEP_MODE: c_uint = 0x10	/* Soft sleep mode */;
// Output drive capability
pub const OCAP_1x: c_uint = 0x00	/* 1x */;
pub const OCAP_2x: c_uint = 0x01	/* 2x */;
pub const OCAP_3x: c_uint = 0x02	/* 3x */;
pub const OCAP_4x: c_uint = 0x03	/* 4x */;
// COM3

pub const VFLIP_IMG: c_uint = 0x80	/* Vertical flip image ON/OFF selection */;
pub const HFLIP_IMG: c_uint = 0x40	/* Horizontal mirror image ON/OFF selection */;
pub const SWAP_RGB: c_uint = 0x20	/* Swap B/R  output sequence in RGB mode */;
pub const SWAP_YUV: c_uint = 0x10	/* Swap Y/UV output sequence in YUV mode */;
pub const SWAP_ML: c_uint = 0x08	/* Swap output MSB/LSB */;
// Tri-state option for output clock
pub const NOTRI_CLOCK: c_uint = 0x04	/*   0: Tri-state    at this period */;
// 1: No tri-state at this period
// Tri-state option for output data
pub const NOTRI_DATA: c_uint = 0x02	/*   0: Tri-state    at this period */;
// 1: No tri-state at this period
pub const SCOLOR_TEST: c_uint = 0x01	/* Sensor color bar test pattern */;
// COM4
// PLL frequency control
pub const PLL_BYPASS: c_uint = 0x00	/*  00: Bypass PLL */;
pub const PLL_4x: c_uint = 0x40	/*  01: PLL 4x */;
pub const PLL_6x: c_uint = 0x80	/*  10: PLL 6x */;
pub const PLL_8x: c_uint = 0xc0	/*  11: PLL 8x */;
// AEC evaluate window
pub const AEC_FULL: c_uint = 0x00	/*  00: Full window */;
pub const AEC_1p2: c_uint = 0x10	/*  01: 1/2  window */;
pub const AEC_1p4: c_uint = 0x20	/*  10: 1/4  window */;
pub const AEC_2p3: c_uint = 0x30	/*  11: Low 2/3 window */;
pub const COM4_RESERVED: c_uint = 0x01	/* Reserved bit */;
// COM5
pub const AFR_ON_OFF: c_uint = 0x80	/* Auto frame rate control ON/OFF selection */;
pub const AFR_SPPED: c_uint = 0x40	/* Auto frame rate control speed selection */;
// Auto frame rate max rate control
pub const AFR_NO_RATE: c_uint = 0x00	/*     No  reduction of frame rate */;
pub const AFR_1p2: c_uint = 0x10	/*     Max reduction to 1/2 frame rate */;
pub const AFR_1p4: c_uint = 0x20	/*     Max reduction to 1/4 frame rate */;
pub const AFR_1p8: c_uint = 0x30	/* Max reduction to 1/8 frame rate */;
// Auto frame rate active point control
pub const AF_2x: c_uint = 0x00	/*     Add frame when AGC reaches  2x gain */;
pub const AF_4x: c_uint = 0x04	/*     Add frame when AGC reaches  4x gain */;
pub const AF_8x: c_uint = 0x08	/*     Add frame when AGC reaches  8x gain */;
pub const AF_16x: c_uint = 0x0c	/* Add frame when AGC reaches 16x gain */;
// AEC max step control
pub const AEC_NO_LIMIT: c_uint = 0x01	/*   0 : AEC increase step has limit */;
// 1 : No limit to AEC increase step
// CLKRC
// Input clock divider register
pub const CLKRC_RESERVED: c_uint = 0x80	/* Reserved bit */;

// COM7
// SCCB Register Reset
pub const SCCB_RESET: c_uint = 0x80	/*   0 : No change */;
// 1 : Resets all registers to default
// Resolution selection
pub const SLCT_MASK: c_uint = 0x40	/*   Mask of VGA or QVGA */;
pub const SLCT_VGA: c_uint = 0x00	/*   0 : VGA */;
pub const SLCT_QVGA: c_uint = 0x40	/*   1 : QVGA */;
pub const ITU656_ON_OFF: c_uint = 0x20	/* ITU656 protocol ON/OFF selection */;
pub const SENSOR_RAW: c_uint = 0x10	/* Sensor RAW */;
// RGB output format control
pub const FMT_MASK: c_uint = 0x0c	/*      Mask of color format */;
pub const FMT_GBR422: c_uint = 0x00	/*      00 : GBR 4:2:2 */;
pub const FMT_RGB565: c_uint = 0x04	/*      01 : RGB 565 */;
pub const FMT_RGB555: c_uint = 0x08	/*      10 : RGB 555 */;
pub const FMT_RGB444: c_uint = 0x0c	/* 11 : RGB 444 */;
// Output format control
pub const OFMT_MASK: c_uint = 0x03    /*      Mask of output format */;
pub const OFMT_YUV: c_uint = 0x00	/*      00 : YUV */;
pub const OFMT_P_BRAW: c_uint = 0x01	/*      01 : Processed Bayer RAW */;
pub const OFMT_RGB: c_uint = 0x02	/*      10 : RGB */;
pub const OFMT_BRAW: c_uint = 0x03	/* 11 : Bayer RAW */;
// COM8
pub const FAST_ALGO: c_uint = 0x80	/* Enable fast AGC/AEC algorithm */;
// AEC Setp size limit
pub const UNLMT_STEP: c_uint = 0x40	/*   0 : Step size is limited */;
// 1 : Unlimited step size
pub const BNDF_ON_OFF: c_uint = 0x20	/* Banding filter ON/OFF */;
pub const AEC_BND: c_uint = 0x10	/* Enable AEC below banding value */;
pub const AEC_ON_OFF: c_uint = 0x08	/* Fine AEC ON/OFF control */;
pub const AGC_ON: c_uint = 0x04	/* AGC Enable */;
pub const AWB_ON: c_uint = 0x02	/* AWB Enable */;
pub const AEC_ON: c_uint = 0x01	/* AEC Enable */;
// COM9
pub const BASE_AECAGC: c_uint = 0x80	/* Histogram or average based AEC/AGC */;
// Automatic gain ceiling - maximum AGC value
pub const GAIN_2x: c_uint = 0x00	/*    000 :   2x */;
pub const GAIN_4x: c_uint = 0x10	/*    001 :   4x */;
pub const GAIN_8x: c_uint = 0x20	/*    010 :   8x */;
pub const GAIN_16x: c_uint = 0x30	/*    011 :  16x */;
pub const GAIN_32x: c_uint = 0x40	/*    100 :  32x */;
pub const GAIN_64x: c_uint = 0x50	/* 101 :  64x */;
pub const GAIN_128x: c_uint = 0x60	/* 110 : 128x */;
pub const DROP_VSYNC: c_uint = 0x04	/* Drop VSYNC output of corrupt frame */;
pub const DROP_HREF: c_uint = 0x02	/* Drop HREF  output of corrupt frame */;
// COM11
pub const SGLF_ON_OFF: c_uint = 0x02	/* Single frame ON/OFF selection */;
pub const SGLF_TRIG: c_uint = 0x01	/* Single frame transfer trigger */;
// HREF

// EXHCH

// DSP_CTRL1
pub const FIFO_ON: c_uint = 0x80	/* FIFO enable/disable selection */;
pub const UV_ON_OFF: c_uint = 0x40	/* UV adjust function ON/OFF selection */;
pub const YUV444_2_422: c_uint = 0x20	/* YUV444 to 422 UV channel option selection */;
pub const CLR_MTRX_ON_OFF: c_uint = 0x10	/* Color matrix ON/OFF selection */;
pub const INTPLT_ON_OFF: c_uint = 0x08	/* Interpolation ON/OFF selection */;
pub const GMM_ON_OFF: c_uint = 0x04	/* Gamma function ON/OFF selection */;
pub const AUTO_BLK_ON_OFF: c_uint = 0x02	/* Black defect auto correction ON/OFF */;
pub const AUTO_WHT_ON_OFF: c_uint = 0x01	/* White define auto correction ON/OFF */;
// DSP_CTRL3
pub const UV_MASK: c_uint = 0x80	/* UV output sequence option */;
pub const UV_ON: c_uint = 0x80	/*   ON */;
pub const UV_OFF: c_uint = 0x00	/*   OFF */;
pub const CBAR_MASK: c_uint = 0x20	/* DSP Color bar mask */;
pub const CBAR_ON: c_uint = 0x20	/*   ON */;
pub const CBAR_OFF: c_uint = 0x00	/*   OFF */;
// DSP_CTRL4
pub const DSP_OFMT_YUV: c_uint = 0x00;
pub const DSP_OFMT_RGB: c_uint = 0x00;
pub const DSP_OFMT_RAW8: c_uint = 0x02;
pub const DSP_OFMT_RAW10: c_uint = 0x03;
// DSPAUTO (DSP Auto Function ON/OFF Control)
pub const AWB_ACTRL: c_uint = 0x80 /* AWB auto threshold control */;
pub const DENOISE_ACTRL: c_uint = 0x40 /* De-noise auto threshold control */;
pub const EDGE_ACTRL: c_uint = 0x20 /* Edge enhancement auto strength control */;
pub const UV_ACTRL: c_uint = 0x10 /* UV adjust auto slope control */;
pub const SCAL0_ACTRL: c_uint = 0x08 /* Auto scaling factor control */;
pub const SCAL1_2_ACTRL: c_uint = 0x04 /* Auto scaling factor control */;

//
// ID
//
pub const OV7720: c_uint = 0x7720;
pub const OV7725: c_uint = 0x7721;

//
// PLL multipliers
//
    static struct {
    unsigned int mult;
    u8 com4;
    } ov772x_pll[] = {
    { 1, PLL_BYPASS, },
    { 4, PLL_4x, },
    { 6, PLL_6x, },
    { 8, PLL_8x, },
    };
//
// struct
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov772x_color_format {
    pub code: u32,
    pub colorspace: enum v4l2_colorspace,
    pub dsp3: u8,
    pub dsp4: u8,
    pub com3: u8,
    pub com7: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov772x_win_size {
    pub name: *mut c_char,
    pub com7_bit: c_uchar,
    pub sizeimage: c_uint,
    pub rect: v4l2_rect,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ov772x_priv {
    pub subdev: v4l2_subdev,
    pub hdl: v4l2_ctrl_handler,
    pub clk: *mut clk,
    pub regmap: *mut regmap,
    pub info: *mut ov772x_camera_info,
    pub pwdn_gpio: *mut gpio_desc,
    pub rstb_gpio: *mut gpio_desc,
    pub cfmt: *const ov772x_color_format,
    pub win: *const ov772x_win_size,
    pub vflip_ctrl: *mut v4l2_ctrl,
    pub hflip_ctrl: *mut v4l2_ctrl,
    pub test_pattern: c_uint,
// band_filter = COM8[5] ? 256 - BDBASE : 0
    pub band_filter_ctrl: *mut v4l2_ctrl,
    pub fps: c_uint,
// lock to protect power_count and streaming
    pub lock: mutex,
    pub power_count: c_int,
    pub streaming: c_int,
    pub pad: media_pad,
    pub bus_type: enum v4l2_mbus_type,
}

//
// supported color format list
//
    static const struct ov772x_color_format ov772x_cfmts[] = {
    {
    .code		= MEDIA_BUS_FMT_YUYV8_2X8,
    .colorspace	= V4L2_COLORSPACE_SRGB,
    .dsp3		= 0x0,
    .dsp4		= DSP_OFMT_YUV,
    .com3		= SWAP_YUV,
    .com7		= OFMT_YUV,
    },
    {
    .code		= MEDIA_BUS_FMT_YVYU8_2X8,
    .colorspace	= V4L2_COLORSPACE_SRGB,
    .dsp3		= UV_ON,
    .dsp4		= DSP_OFMT_YUV,
    .com3		= SWAP_YUV,
    .com7		= OFMT_YUV,
    },
    {
    .code		= MEDIA_BUS_FMT_UYVY8_2X8,
    .colorspace	= V4L2_COLORSPACE_SRGB,
    .dsp3		= 0x0,
    .dsp4		= DSP_OFMT_YUV,
    .com3		= 0x0,
    .com7		= OFMT_YUV,
    },
    {
    .code		= MEDIA_BUS_FMT_RGB555_2X8_PADHI_LE,
    .colorspace	= V4L2_COLORSPACE_SRGB,
    .dsp3		= 0x0,
    .dsp4		= DSP_OFMT_YUV,
    .com3		= SWAP_RGB,
    .com7		= FMT_RGB555 | OFMT_RGB,
    },
    {
    .code		= MEDIA_BUS_FMT_RGB555_2X8_PADHI_BE,
    .colorspace	= V4L2_COLORSPACE_SRGB,
    .dsp3		= 0x0,
    .dsp4		= DSP_OFMT_YUV,
    .com3		= 0x0,
    .com7		= FMT_RGB555 | OFMT_RGB,
    },
    {
    .code		= MEDIA_BUS_FMT_RGB565_2X8_LE,
    .colorspace	= V4L2_COLORSPACE_SRGB,
    .dsp3		= 0x0,
    .dsp4		= DSP_OFMT_YUV,
    .com3		= SWAP_RGB,
    .com7		= FMT_RGB565 | OFMT_RGB,
    },
    {
    .code		= MEDIA_BUS_FMT_RGB565_2X8_BE,
    .colorspace	= V4L2_COLORSPACE_SRGB,
    .dsp3		= 0x0,
    .dsp4		= DSP_OFMT_YUV,
    .com3		= 0x0,
    .com7		= FMT_RGB565 | OFMT_RGB,
    },
    {
// Setting DSP4 to DSP_OFMT_RAW8 still gives 10-bit output,
// regardless of the COM7 value. We can thus only support 10-bit
// Bayer until someone figures it out.
//
    .code		= MEDIA_BUS_FMT_SBGGR10_1X10,
    .colorspace	= V4L2_COLORSPACE_SRGB,
    .dsp3		= 0x0,
    .dsp4		= DSP_OFMT_RAW10,
    .com3		= 0x0,
    .com7		= SENSOR_RAW | OFMT_BRAW,
    },
    };
//
// window size list
//
    static const struct ov772x_win_size ov772x_win_sizes[] = {
    {
    .name		= "VGA",
    .com7_bit	= SLCT_VGA,
    .sizeimage	= 510 * 748,
    .rect = {
    .left	= 140,
    .top	= 14,
    .width	= VGA_WIDTH,
    .height	= VGA_HEIGHT,
    },
    }, {
    .name		= "QVGA",
    .com7_bit	= SLCT_QVGA,
    .sizeimage	= 278 * 576,
    .rect = {
    .left	= 252,
    .top	= 6,
    .width	= QVGA_WIDTH,
    .height	= QVGA_HEIGHT,
    },
    },
    };
    static const char * const ov772x_test_pattern_menu[] = {
    "Disabled",
    "Vertical Color Bar Type 1",
    };
//
// frame rate settings lists
//
    static const unsigned int ov772x_frame_intervals[] = { 5, 10, 15, 20, 30, 60 };
//
// general function
//
    static struct ov772x_priv *to_ov772x(struct v4l2_subdev *sd)
    {
    return container_of(sd, struct ov772x_priv, subdev);
    }
#[no_mangle]
unsafe extern "C" fn ov772x_reset(priv: *mut ov772x_priv) -> c_int {
    static int ov772x_reset(struct ov772x_priv *priv)
    {
    int ret;
    ret = regmap_write(priv.regmap, COM7, SCCB_RESET);
    if (ret < 0)
    return ret;
    usleep_range(1000, 5000);
    return regmap_update_bits(priv.regmap, COM2, SOFT_SLEEP_MODE,
    SOFT_SLEEP_MODE);
    }
//
// subdev ops
//
#[no_mangle]
unsafe extern "C" fn ov772x_s_stream(sd: *mut v4l2_subdev, enable: c_int) -> c_int {
    static int ov772x_s_stream(struct v4l2_subdev *sd, int enable)
    {
    struct i2c_client *client = v4l2_get_subdevdata(sd);
    struct ov772x_priv *priv = to_ov772x(sd);
    let mut ret: c_int = 0;
    mutex_lock(&priv.lock);
    if (priv.streaming == enable)
    goto done;
    if (priv.bus_type == V4L2_MBUS_BT656) {
    ret = regmap_update_bits(priv.regmap, COM7, ITU656_ON_OFF,
    enable ?
    ITU656_ON_OFF : ~ITU656_ON_OFF);
    if (ret)
    goto done;
    }
    ret = regmap_update_bits(priv.regmap, COM2, SOFT_SLEEP_MODE,
    enable ? 0 : SOFT_SLEEP_MODE);
    if (ret)
    goto done;
    if (enable) {
    dev_dbg(&client.dev, "format %d, win %s\n",
    priv.cfmt.code, priv.win.name);
    }
    priv.streaming = enable;
    done:
    mutex_unlock(&priv.lock);
    return ret;
    }
    static unsigned int ov772x_select_fps(struct ov772x_priv *priv,
    struct v4l2_fract *tpf)
    {
    unsigned int fps = tpf.numerator ?
    tpf.denominator / tpf.numerator :
    tpf.denominator;
    unsigned int best_diff;
    unsigned int diff;
    unsigned int idx;
    unsigned int i;
// Approximate to the closest supported frame interval.
    best_diff = ~0L;
    for (i = 0, idx = 0; i < ARRAY_SIZE(ov772x_frame_intervals); i++) {
    diff = abs(fps - ov772x_frame_intervals[i]);
    if (diff < best_diff) {
    idx = i;
    best_diff = diff;
    }
    }
    return ov772x_frame_intervals[idx];
    }
    static int ov772x_set_frame_rate(struct ov772x_priv *priv,
    unsigned int fps,
    const struct ov772x_color_format *cfmt,
    const struct ov772x_win_size *win)
    {
    let mut fin: c_ulong = clk_get_rate(priv.clk);
    unsigned int best_diff;
    unsigned int fsize;
    unsigned int pclk;
    unsigned int diff;
    unsigned int i;
    let mut clkrc: u8 = 0;
    let mut com4: u8 = 0;
    int ret;
// Use image size (with blankings) to calculate desired pixel clock.
    switch (cfmt.com7 & OFMT_MASK) {
    case OFMT_BRAW:
    fsize = win.sizeimage;
    break;
    case OFMT_RGB:
    case OFMT_YUV:
    default:
    fsize = win.sizeimage * 2;
    break;
    }
    pclk = fps * fsize;
//
// Pixel clock generation circuit is pretty simple:
//
// Fin -> [ / CLKRC_div] -> [ * PLL_mult] -> pclk
//
// Try to approximate the desired pixel clock testing all available
// PLL multipliers (1x, 4x, 6x, 8x) and calculate corresponding
// divisor with:
//
// div = PLL_mult * Fin / pclk
//
// and re-calculate the pixel clock using it:
//
// pclk = Fin * PLL_mult / CLKRC_div
//
// Choose the PLL_mult and CLKRC_div pair that gives a pixel clock
// closer to the desired one.
//
// The desired pixel clock is calculated using a known frame size
// (blanking included) and FPS.
//
    best_diff = ~0L;
    for (i = 0; i < ARRAY_SIZE(ov772x_pll); i++) {
    let mut pll_mult: c_uint = ov772x_pll[i].mult;
    let mut pll_out: c_uint = pll_mult * fin;
    unsigned int t_pclk;
    unsigned int div;
    if (pll_out < pclk)
    continue;
    div = DIV_ROUND_CLOSEST(pll_out, pclk);
    t_pclk = DIV_ROUND_CLOSEST(fin * pll_mult, div);
    diff = abs(pclk - t_pclk);
    if (diff < best_diff) {
    best_diff = diff;
    clkrc = CLKRC_DIV(div);
    com4 = ov772x_pll[i].com4;
    }
    }
    ret = regmap_write(priv.regmap, COM4, com4 | COM4_RESERVED);
    if (ret < 0)
    return ret;
    ret = regmap_write(priv.regmap, CLKRC, clkrc | CLKRC_RESERVED);
    if (ret < 0)
    return ret;
    return 0;
    }
    static int ov772x_get_frame_interval(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_frame_interval *ival)
    {
    struct ov772x_priv *priv = to_ov772x(sd);
    struct v4l2_fract *tpf = &ival.interval;
//
// FIXME: Implement support for V4L2_SUBDEV_FORMAT_TRY, using the V4L2
// subdev active state API.
//
    if (ival.which != V4L2_SUBDEV_FORMAT_ACTIVE)
    return -EINVAL;
    tpf.numerator = 1;
    tpf.denominator = priv.fps;
    return 0;
    }
    static int ov772x_set_frame_interval(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_frame_interval *ival)
    {
    struct ov772x_priv *priv = to_ov772x(sd);
    struct v4l2_fract *tpf = &ival.interval;
    unsigned int fps;
    let mut ret: c_int = 0;
//
// FIXME: Implement support for V4L2_SUBDEV_FORMAT_TRY, using the V4L2
// subdev active state API.
//
    if (ival.which != V4L2_SUBDEV_FORMAT_ACTIVE)
    return -EINVAL;
    mutex_lock(&priv.lock);
    if (priv.streaming) {
    ret = -EBUSY;
    goto error;
    }
    fps = ov772x_select_fps(priv, tpf);
//
// If the device is not powered up by the host driver do
// not apply any changes to H/W at this time. Instead
// the frame rate will be restored right after power-up.
//
    if (priv.power_count > 0) {
    ret = ov772x_set_frame_rate(priv, fps, priv.cfmt, priv.win);
    if (ret)
    goto error;
    }
    tpf.numerator = 1;
    tpf.denominator = fps;
    priv.fps = fps;
    error:
    mutex_unlock(&priv.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ov772x_s_ctrl(ctrl: *mut v4l2_ctrl) -> c_int {
    static int ov772x_s_ctrl(struct v4l2_ctrl *ctrl)
    {
    struct ov772x_priv *priv = container_of(ctrl.handler,
    struct ov772x_priv, hdl);
    struct regmap *regmap = priv.regmap;
    let mut ret: c_int = 0;
    u8 val;
// v4l2_ctrl_lock() locks our own mutex
//
// If the device is not powered up by the host driver do
// not apply any controls to H/W at this time. Instead
// the controls will be restored right after power-up.
//
    if (priv.power_count == 0)
    return 0;
    switch (ctrl.id) {
    case V4L2_CID_VFLIP:
    val = ctrl.val ? VFLIP_IMG : 0x00;
    if (priv.info && (priv.info.flags & OV772X_FLAG_VFLIP))
    val ^= VFLIP_IMG;
    return regmap_update_bits(regmap, COM3, VFLIP_IMG, val);
    case V4L2_CID_HFLIP:
    val = ctrl.val ? HFLIP_IMG : 0x00;
    if (priv.info && (priv.info.flags & OV772X_FLAG_HFLIP))
    val ^= HFLIP_IMG;
    return regmap_update_bits(regmap, COM3, HFLIP_IMG, val);
    case V4L2_CID_BAND_STOP_FILTER:
    if (!ctrl.val) {
// Switch the filter off, it is on now
    ret = regmap_update_bits(regmap, BDBASE, 0xff, 0xff);
    if (!ret)
    ret = regmap_update_bits(regmap, COM8,
    BNDF_ON_OFF, 0);
    } else {
// Switch the filter on, set AEC low limit
    val = 256 - ctrl.val;
    ret = regmap_update_bits(regmap, COM8,
    BNDF_ON_OFF, BNDF_ON_OFF);
    if (!ret)
    ret = regmap_update_bits(regmap, BDBASE,
    0xff, val);
    }
    return ret;
    case V4L2_CID_TEST_PATTERN:
    priv.test_pattern = ctrl.val;
    return 0;
    }
    return -EINVAL;
    }

    static int ov772x_g_register(struct v4l2_subdev *sd,
    struct v4l2_dbg_register *reg)
    {
    struct ov772x_priv *priv = to_ov772x(sd);
    int ret;
    unsigned int val;
    reg.size = 1;
    if (reg.reg > 0xff)
    return -EINVAL;
    ret = regmap_read(priv.regmap, reg.reg, &val);
    if (ret < 0)
    return ret;
    reg.val = (__u64)val;
    return 0;
    }
    static int ov772x_s_register(struct v4l2_subdev *sd,
    const struct v4l2_dbg_register *reg)
    {
    struct ov772x_priv *priv = to_ov772x(sd);
    if (reg.reg > 0xff ||
    reg.val > 0xff)
    return -EINVAL;
    return regmap_write(priv.regmap, reg.reg, reg.val);
    }

#[no_mangle]
unsafe extern "C" fn ov772x_power_on(priv: *mut ov772x_priv) -> c_int {
    static int ov772x_power_on(struct ov772x_priv *priv)
    {
    struct i2c_client *client = v4l2_get_subdevdata(&priv.subdev);
    int ret;
    if (priv.clk) {
    ret = clk_prepare_enable(priv.clk);
    if (ret)
    return ret;
    }
    if (priv.pwdn_gpio) {
    gpiod_set_value(priv.pwdn_gpio, 1);
    usleep_range(500, 1000);
    }
//
// FIXME: The reset signal is connected to a shared GPIO on some
// platforms (namely the SuperH Migo-R). Until a framework becomes
// available to handle this cleanly, request the GPIO temporarily
// to avoid conflicts.
//
    priv.rstb_gpio = gpiod_get_optional(&client.dev, "reset",
    GPIOD_OUT_LOW);
    if (IS_ERR(priv.rstb_gpio)) {
    dev_info(&client.dev, "Unable to get GPIO \"reset\"");
    clk_disable_unprepare(priv.clk);
    return PTR_ERR(priv.rstb_gpio);
    }
    if (priv.rstb_gpio) {
    gpiod_set_value(priv.rstb_gpio, 1);
    usleep_range(500, 1000);
    gpiod_set_value(priv.rstb_gpio, 0);
    usleep_range(500, 1000);
    gpiod_put(priv.rstb_gpio);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ov772x_power_off(priv: *mut ov772x_priv) -> c_int {
    static int ov772x_power_off(struct ov772x_priv *priv)
    {
    clk_disable_unprepare(priv.clk);
    if (priv.pwdn_gpio) {
    gpiod_set_value(priv.pwdn_gpio, 0);
    usleep_range(500, 1000);
    }
    return 0;
    }
    static int ov772x_set_params(struct ov772x_priv *priv,
    const struct ov772x_color_format *cfmt,
    const struct ov772x_win_size *win);
#[no_mangle]
unsafe extern "C" fn ov772x_s_power(sd: *mut v4l2_subdev, on: c_int) -> c_int {
    static int ov772x_s_power(struct v4l2_subdev *sd, int on)
    {
    struct ov772x_priv *priv = to_ov772x(sd);
    let mut ret: c_int = 0;
    mutex_lock(&priv.lock);
// If the power count is modified from 0 to != 0 or from != 0 to 0,
// update the power state.
//
    if (priv.power_count == !on) {
    if (on) {
    ret = ov772x_power_on(priv);
//
// Restore the format, the frame rate, and
// the controls
//
    if (!ret)
    ret = ov772x_set_params(priv, priv.cfmt,
    priv.win);
    } else {
    ret = ov772x_power_off(priv);
    }
    }
    if (!ret) {
// Update the power count.
    priv.power_count += on ? 1 : -1;
    WARN(priv.power_count < 0, "Unbalanced power count\n");
    WARN(priv.power_count > 1, "Duplicated s_power call\n");
    }
    mutex_unlock(&priv.lock);
    return ret;
    }
    static const struct ov772x_win_size *ov772x_select_win(u32 width, u32 height)
    {
    const struct ov772x_win_size *win = &ov772x_win_sizes[0];
    let mut best_diff: u32 = UINT_MAX;
    unsigned int i;
    for (i = 0; i < ARRAY_SIZE(ov772x_win_sizes); ++i) {
    u32 diff = abs(width - ov772x_win_sizes[i].rect.width)
    + abs(height - ov772x_win_sizes[i].rect.height);
    if (diff < best_diff) {
    best_diff = diff;
    win = &ov772x_win_sizes[i];
    }
    }
    return win;
    }
    static void ov772x_select_params(const struct v4l2_mbus_framefmt *mf,
    const struct ov772x_color_format **cfmt,
    const struct ov772x_win_size **win)
    {
    unsigned int i;
// Select a format.
// cfmt = &ov772x_cfmts[0];
    for (i = 0; i < ARRAY_SIZE(ov772x_cfmts); i++) {
    if (mf.code == ov772x_cfmts[i].code) {
// cfmt = &ov772x_cfmts[i];
    break;
    }
    }
// Select a window size.
// win = ov772x_select_win(mf->width, mf->height);
    }
#[no_mangle]
unsafe extern "C" fn ov772x_edgectrl(priv: *mut ov772x_priv) -> c_int {
    static int ov772x_edgectrl(struct ov772x_priv *priv)
    {
    struct regmap *regmap = priv.regmap;
    int ret;
    if (!priv.info)
    return 0;
    if (priv.info.edgectrl.strength & OV772X_MANUAL_EDGE_CTRL) {
//
// Manual Edge Control Mode.
//
// Edge auto strength bit is set by default.
// Remove it when manual mode.
//
    ret = regmap_update_bits(regmap, DSPAUTO, EDGE_ACTRL, 0x00);
    if (ret < 0)
    return ret;
    ret = regmap_update_bits(regmap, EDGE_TRSHLD,
    OV772X_EDGE_THRESHOLD_MASK,
    priv.info.edgectrl.threshold);
    if (ret < 0)
    return ret;
    ret = regmap_update_bits(regmap, EDGE_STRNGT,
    OV772X_EDGE_STRENGTH_MASK,
    priv.info.edgectrl.strength);
    if (ret < 0)
    return ret;
    } else if (priv.info.edgectrl.upper > priv.info.edgectrl.lower) {
//
// Auto Edge Control Mode.
//
// Set upper and lower limit.
//
    ret = regmap_update_bits(regmap, EDGE_UPPER,
    OV772X_EDGE_UPPER_MASK,
    priv.info.edgectrl.upper);
    if (ret < 0)
    return ret;
    ret = regmap_update_bits(regmap, EDGE_LOWER,
    OV772X_EDGE_LOWER_MASK,
    priv.info.edgectrl.lower);
    if (ret < 0)
    return ret;
    }
    return 0;
    }
    static int ov772x_set_params(struct ov772x_priv *priv,
    const struct ov772x_color_format *cfmt,
    const struct ov772x_win_size *win)
    {
    int ret;
    u8  val;
// Reset hardware.
    ov772x_reset(priv);
// Edge Ctrl.
    ret = ov772x_edgectrl(priv);
    if (ret < 0)
    return ret;
// Format and window size.
    ret = regmap_write(priv.regmap, HSTART, win.rect.left >> 2);
    if (ret < 0)
    goto ov772x_set_fmt_error;
    ret = regmap_write(priv.regmap, HSIZE, win.rect.width >> 2);
    if (ret < 0)
    goto ov772x_set_fmt_error;
    ret = regmap_write(priv.regmap, VSTART, win.rect.top >> 1);
    if (ret < 0)
    goto ov772x_set_fmt_error;
    ret = regmap_write(priv.regmap, VSIZE, win.rect.height >> 1);
    if (ret < 0)
    goto ov772x_set_fmt_error;
    ret = regmap_write(priv.regmap, HOUTSIZE, win.rect.width >> 2);
    if (ret < 0)
    goto ov772x_set_fmt_error;
    ret = regmap_write(priv.regmap, VOUTSIZE, win.rect.height >> 1);
    if (ret < 0)
    goto ov772x_set_fmt_error;
    ret = regmap_write(priv.regmap, HREF,
    ((win.rect.top & 1) << HREF_VSTART_SHIFT) |
    ((win.rect.left & 3) << HREF_HSTART_SHIFT) |
    ((win.rect.height & 1) << HREF_VSIZE_SHIFT) |
    ((win.rect.width & 3) << HREF_HSIZE_SHIFT));
    if (ret < 0)
    goto ov772x_set_fmt_error;
    ret = regmap_write(priv.regmap, EXHCH,
    ((win.rect.height & 1) << EXHCH_VSIZE_SHIFT) |
    ((win.rect.width & 3) << EXHCH_HSIZE_SHIFT));
    if (ret < 0)
    goto ov772x_set_fmt_error;
// Set DSP_CTRL3.
    val = cfmt.dsp3;
    if (val) {
    ret = regmap_update_bits(priv.regmap, DSP_CTRL3, UV_MASK, val);
    if (ret < 0)
    goto ov772x_set_fmt_error;
    }
// DSP_CTRL4: AEC reference point and DSP output format.
    if (cfmt.dsp4) {
    ret = regmap_write(priv.regmap, DSP_CTRL4, cfmt.dsp4);
    if (ret < 0)
    goto ov772x_set_fmt_error;
    }
// Set COM3.
    val = cfmt.com3;
    if (priv.info && (priv.info.flags & OV772X_FLAG_VFLIP))
    val |= VFLIP_IMG;
    if (priv.info && (priv.info.flags & OV772X_FLAG_HFLIP))
    val |= HFLIP_IMG;
    if (priv.vflip_ctrl.val)
    val ^= VFLIP_IMG;
    if (priv.hflip_ctrl.val)
    val ^= HFLIP_IMG;
    if (priv.test_pattern)
    val |= SCOLOR_TEST;
    ret = regmap_update_bits(priv.regmap, COM3, SWAP_MASK | IMG_MASK, val);
    if (ret < 0)
    goto ov772x_set_fmt_error;
// COM7: Sensor resolution and output format control.
    ret = regmap_write(priv.regmap, COM7, win.com7_bit | cfmt.com7);
    if (ret < 0)
    goto ov772x_set_fmt_error;
// COM4, CLKRC: Set pixel clock and framerate.
    ret = ov772x_set_frame_rate(priv, priv.fps, cfmt, win);
    if (ret < 0)
    goto ov772x_set_fmt_error;
// Set COM8.
    if (priv.band_filter_ctrl.val) {
    let mut band_filter: c_ushort = priv.band_filter_ctrl.val;
    ret = regmap_update_bits(priv.regmap, COM8,
    BNDF_ON_OFF, BNDF_ON_OFF);
    if (!ret)
    ret = regmap_update_bits(priv.regmap, BDBASE,
    0xff, 256 - band_filter);
    if (ret < 0)
    goto ov772x_set_fmt_error;
    }
    return ret;
    ov772x_set_fmt_error:
    ov772x_reset(priv);
    return ret;
    }
    static int ov772x_get_selection(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_selection *sel)
    {
    struct ov772x_priv *priv = to_ov772x(sd);
    if (sel.which != V4L2_SUBDEV_FORMAT_ACTIVE)
    return -EINVAL;
    sel.r.left = 0;
    sel.r.top = 0;
    switch (sel.target) {
    case V4L2_SEL_TGT_CROP_BOUNDS:
    case V4L2_SEL_TGT_CROP:
    sel.r.width = priv.win.rect.width;
    sel.r.height = priv.win.rect.height;
    return 0;
    default:
    return -EINVAL;
    }
    }
    static int ov772x_get_fmt(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_format *format)
    {
    struct v4l2_mbus_framefmt *mf = &format.format;
    struct ov772x_priv *priv = to_ov772x(sd);
    if (format.pad)
    return -EINVAL;
    mf.width	= priv.win.rect.width;
    mf.height	= priv.win.rect.height;
    mf.code	= priv.cfmt.code;
    mf.colorspace	= priv.cfmt.colorspace;
    mf.field	= V4L2_FIELD_NONE;
    return 0;
    }
    static int ov772x_set_fmt(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_format *format)
    {
    struct ov772x_priv *priv = to_ov772x(sd);
    struct v4l2_mbus_framefmt *mf = &format.format;
    const struct ov772x_color_format *cfmt;
    const struct ov772x_win_size *win;
    let mut ret: c_int = 0;
    if (format.pad)
    return -EINVAL;
    ov772x_select_params(mf, &cfmt, &win);
    mf.code = cfmt.code;
    mf.width = win.rect.width;
    mf.height = win.rect.height;
    mf.field = V4L2_FIELD_NONE;
    mf.colorspace = cfmt.colorspace;
    mf.ycbcr_enc = V4L2_YCBCR_ENC_DEFAULT;
    mf.quantization = V4L2_QUANTIZATION_DEFAULT;
    mf.xfer_func = V4L2_XFER_FUNC_DEFAULT;
    if (format.which == V4L2_SUBDEV_FORMAT_TRY) {
// v4l2_subdev_state_get_format(sd_state, 0) = *mf;
    return 0;
    }
    mutex_lock(&priv.lock);
    if (priv.streaming) {
    ret = -EBUSY;
    goto error;
    }
//
// If the device is not powered up by the host driver do
// not apply any changes to H/W at this time. Instead
// the format will be restored right after power-up.
//
    if (priv.power_count > 0) {
    ret = ov772x_set_params(priv, cfmt, win);
    if (ret < 0)
    goto error;
    }
    priv.win = win;
    priv.cfmt = cfmt;
    error:
    mutex_unlock(&priv.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ov772x_video_probe(priv: *mut ov772x_priv) -> c_int {
    static int ov772x_video_probe(struct ov772x_priv *priv)
    {
    struct i2c_client  *client = v4l2_get_subdevdata(&priv.subdev);
    int		    pid, ver, midh, midl;
    const char         *devname;
    int		    ret;
    ret = ov772x_power_on(priv);
    if (ret < 0)
    return ret;
// Check and show product ID and manufacturer ID.
    ret = regmap_read(priv.regmap, PID, &pid);
    if (ret < 0)
    return ret;
    ret = regmap_read(priv.regmap, VER, &ver);
    if (ret < 0)
    return ret;
    switch (VERSION(pid, ver)) {
    case OV7720:
    devname     = "ov7720";
    break;
    case OV7725:
    devname     = "ov7725";
    break;
    default:
    dev_err(&client.dev,
    "Product ID error %x:%x\n", pid, ver);
    ret = -ENODEV;
    goto done;
    }
    ret = regmap_read(priv.regmap, MIDH, &midh);
    if (ret < 0)
    return ret;
    ret = regmap_read(priv.regmap, MIDL, &midl);
    if (ret < 0)
    return ret;
    dev_info(&client.dev,
    "%s Product ID %0x:%0x Manufacturer ID %x:%x\n",
    devname, pid, ver, midh, midl);
    ret = v4l2_ctrl_handler_setup(&priv.hdl);
    done:
    ov772x_power_off(priv);
    return ret;
    }
    static const struct v4l2_ctrl_ops ov772x_ctrl_ops = {
    .s_ctrl = ov772x_s_ctrl,
    };
    static const struct v4l2_subdev_core_ops ov772x_subdev_core_ops = {
    .log_status = v4l2_ctrl_subdev_log_status,
    .subscribe_event = v4l2_ctrl_subdev_subscribe_event,
    .unsubscribe_event = v4l2_event_subdev_unsubscribe,

    .g_register	= ov772x_g_register,
    .s_register	= ov772x_s_register,

    .s_power	= ov772x_s_power,
    };
    static int ov772x_enum_frame_interval(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_frame_interval_enum *fie)
    {
    if (fie.pad || fie.index >= ARRAY_SIZE(ov772x_frame_intervals))
    return -EINVAL;
    if (fie.width != VGA_WIDTH && fie.width != QVGA_WIDTH)
    return -EINVAL;
    if (fie.height != VGA_HEIGHT && fie.height != QVGA_HEIGHT)
    return -EINVAL;
    fie.interval.numerator = 1;
    fie.interval.denominator = ov772x_frame_intervals[fie.index];
    return 0;
    }
    static int ov772x_enum_mbus_code(struct v4l2_subdev *sd,
    struct v4l2_subdev_state *sd_state,
    struct v4l2_subdev_mbus_code_enum *code)
    {
    if (code.pad || code.index >= ARRAY_SIZE(ov772x_cfmts))
    return -EINVAL;
    code.code = ov772x_cfmts[code.index].code;
    return 0;
    }
    static const struct v4l2_subdev_video_ops ov772x_subdev_video_ops = {
    .s_stream		= ov772x_s_stream,
    };
    static const struct v4l2_subdev_pad_ops ov772x_subdev_pad_ops = {
    .enum_frame_interval	= ov772x_enum_frame_interval,
    .enum_mbus_code		= ov772x_enum_mbus_code,
    .get_selection		= ov772x_get_selection,
    .get_fmt		= ov772x_get_fmt,
    .set_fmt		= ov772x_set_fmt,
    .get_frame_interval	= ov772x_get_frame_interval,
    .set_frame_interval	= ov772x_set_frame_interval,
    };
    static const struct v4l2_subdev_ops ov772x_subdev_ops = {
    .core	= &ov772x_subdev_core_ops,
    .video	= &ov772x_subdev_video_ops,
    .pad	= &ov772x_subdev_pad_ops,
    };
    static int ov772x_parse_dt(struct i2c_client *client,
    struct ov772x_priv *priv)
    {
    struct v4l2_fwnode_endpoint bus_cfg = {
    .bus_type = V4L2_MBUS_PARALLEL
    };
    struct fwnode_handle *ep;
    int ret;
    ep = fwnode_graph_get_next_endpoint(dev_fwnode(&client.dev), core::ptr::null_mut());
    if (!ep) {
    dev_err(&client.dev, "Endpoint node not found\n");
    return -EINVAL;
    }
//
// For backward compatibility with older DTS where the
// bus-type property was not mandatory, assume
// V4L2_MBUS_PARALLEL as it was the only supported bus at the
// time. v4l2_fwnode_endpoint_alloc_parse() will not fail if
// 'bus-type' is not specified.
//
    ret = v4l2_fwnode_endpoint_alloc_parse(ep, &bus_cfg);
    if (ret) {
    bus_cfg = (struct v4l2_fwnode_endpoint)
    { .bus_type = V4L2_MBUS_BT656 };
    ret = v4l2_fwnode_endpoint_alloc_parse(ep, &bus_cfg);
    if (ret)
    goto error_fwnode_put;
    }
    priv.bus_type = bus_cfg.bus_type;
    v4l2_fwnode_endpoint_free(&bus_cfg);
    error_fwnode_put:
    fwnode_handle_put(ep);
    return ret;
    }
//
// i2c_driver function
//
#[no_mangle]
unsafe extern "C" fn ov772x_probe(client: *mut i2c_client) -> c_int {
    static int ov772x_probe(struct i2c_client *client)
    {
    struct ov772x_priv	*priv;
    int			ret;
    static const struct regmap_config ov772x_regmap_config = {
    .reg_bits = 8,
    .val_bits = 8,
    .max_register = DSPAUTO,
    };
    if (!client.dev.of_node && !client.dev.platform_data) {
    dev_err(&client.dev,
    "Missing ov772x platform data for non-DT device\n");
    return -EINVAL;
    }
    priv = devm_kzalloc(&client.dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.regmap = devm_regmap_init_sccb(client, &ov772x_regmap_config);
    if (IS_ERR(priv.regmap)) {
    dev_err(&client.dev, "Failed to allocate register map\n");
    return PTR_ERR(priv.regmap);
    }
    priv.info = client.dev.platform_data;
    mutex_init(&priv.lock);
    v4l2_i2c_subdev_init(&priv.subdev, client, &ov772x_subdev_ops);
    priv.subdev.flags |= V4L2_SUBDEV_FL_HAS_DEVNODE |
    V4L2_SUBDEV_FL_HAS_EVENTS;
    v4l2_ctrl_handler_init(&priv.hdl, 3);
// Use our mutex for the controls
    priv.hdl.lock = &priv.lock;
    priv.vflip_ctrl = v4l2_ctrl_new_std(&priv.hdl, &ov772x_ctrl_ops,
    V4L2_CID_VFLIP, 0, 1, 1, 0);
    priv.hflip_ctrl = v4l2_ctrl_new_std(&priv.hdl, &ov772x_ctrl_ops,
    V4L2_CID_HFLIP, 0, 1, 1, 0);
    priv.band_filter_ctrl = v4l2_ctrl_new_std(&priv.hdl, &ov772x_ctrl_ops,
    V4L2_CID_BAND_STOP_FILTER,
    0, 256, 1, 0);
    v4l2_ctrl_new_std_menu_items(&priv.hdl, &ov772x_ctrl_ops,
    V4L2_CID_TEST_PATTERN,
    ARRAY_SIZE(ov772x_test_pattern_menu) - 1,
    0, 0, ov772x_test_pattern_menu);
    priv.subdev.ctrl_handler = &priv.hdl;
    if (priv.hdl.error) {
    ret = priv.hdl.error;
    goto error_ctrl_free;
    }
    priv.clk = clk_get(&client.dev, core::ptr::null_mut());
    if (IS_ERR(priv.clk)) {
    dev_err(&client.dev, "Unable to get xclk clock\n");
    ret = PTR_ERR(priv.clk);
    goto error_ctrl_free;
    }
    priv.pwdn_gpio = gpiod_get_optional(&client.dev, "powerdown",
    GPIOD_OUT_LOW);
    if (IS_ERR(priv.pwdn_gpio)) {
    dev_info(&client.dev, "Unable to get GPIO \"powerdown\"");
    ret = PTR_ERR(priv.pwdn_gpio);
    goto error_clk_put;
    }
    ret = ov772x_parse_dt(client, priv);
    if (ret)
    goto error_clk_put;
    ret = ov772x_video_probe(priv);
    if (ret < 0)
    goto error_gpio_put;
    priv.pad.flags = MEDIA_PAD_FL_SOURCE;
    priv.subdev.entity.function = MEDIA_ENT_F_CAM_SENSOR;
    ret = media_entity_pads_init(&priv.subdev.entity, 1, &priv.pad);
    if (ret < 0)
    goto error_gpio_put;
    priv.cfmt = &ov772x_cfmts[0];
    priv.win = &ov772x_win_sizes[0];
    priv.fps = 15;
    ret = v4l2_async_register_subdev(&priv.subdev);
    if (ret)
    goto error_entity_cleanup;
    return 0;
    error_entity_cleanup:
    media_entity_cleanup(&priv.subdev.entity);
    error_gpio_put:
    if (priv.pwdn_gpio)
    gpiod_put(priv.pwdn_gpio);
    error_clk_put:
    clk_put(priv.clk);
    error_ctrl_free:
    v4l2_ctrl_handler_free(&priv.hdl);
    mutex_destroy(&priv.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ov772x_remove(client: *mut i2c_client) {
    static void ov772x_remove(struct i2c_client *client)
    {
    struct ov772x_priv *priv = to_ov772x(i2c_get_clientdata(client));
    media_entity_cleanup(&priv.subdev.entity);
    clk_put(priv.clk);
    if (priv.pwdn_gpio)
    gpiod_put(priv.pwdn_gpio);
    v4l2_async_unregister_subdev(&priv.subdev);
    v4l2_ctrl_handler_free(&priv.hdl);
    mutex_destroy(&priv.lock);
    }
    static const struct i2c_device_id ov772x_id[] = {
    { .name = "ov772x" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, ov772x_id);
    static const struct of_device_id ov772x_of_match[] = {
    { .compatible = "ovti,ov7725", },
    { .compatible = "ovti,ov7720", },
    { /* sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, ov772x_of_match);
    static struct i2c_driver ov772x_i2c_driver = {
    .driver = {
    .name = "ov772x",
    .of_match_table = ov772x_of_match,
    },
    .probe    = ov772x_probe,
    .remove   = ov772x_remove,
    .id_table = ov772x_id,
    };
    module_i2c_driver(ov772x_i2c_driver);
    MODULE_DESCRIPTION("V4L2 driver for OV772x image sensor");
    MODULE_AUTHOR("Kuninori Morimoto");
    MODULE_LICENSE("GPL v2");
