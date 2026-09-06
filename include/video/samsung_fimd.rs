//! Automatically rewritten from C Header to Rust Module
//! Source: include/video/samsung_fimd.h
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


// SPDX-License-Identifier: GPL-2.0-only
// include/video/samsung_fimd.h
//
// Copyright 2008 Openmoko, Inc.
// Copyright 2008 Simtec Electronics
// http://armlinux.simtec.co.uk
// Ben Dooks <ben@simtec.co.uk>
//
// S3C Platform - new-style fimd and framebuffer register definitions
//
// This is the register set for the fimd and new style framebuffer interface
// found from the S3C2443 onwards into the S3C2416, S3C2450, the
// S3C64XX series such as the S3C6400 and S3C6410, and Exynos series.
//
// VIDCON0
pub const VIDCON0: c_uint = 0x00;

pub const VIDCON0_VIDOUT_SHIFT: c_int = 26;

pub const VIDCON0_L1_DATA_SHIFT: c_int = 23;

pub const VIDCON0_L0_DATA_SHIFT: c_int = 20;

pub const VIDCON0_PNRMODE_SHIFT: c_int = 17;

pub const VIDCON0_CLKVAL_F_SHIFT: c_int = 6;
pub const VIDCON0_CLKVAL_F_LIMIT: c_uint = 0xff;

pub const VIDCON0_CLKSEL_SHIFT: c_int = 2;

pub const VIDCON1: c_uint = 0x04;

pub const VIDCON1_LINECNT_SHIFT: c_int = 16;

pub const VIDCON1_VSTATUS_SHIFT: c_int = 13;

// VIDCON2
pub const VIDCON2: c_uint = 0x08;

pub const VIDCON2_TVFMTSEL1_SHIFT: c_int = 12;

// PRTCON (S3C6410)
// Might not be present in the S3C6410 documentation,
// but tests prove it's there almost for sure; shouldn't hurt in any case.
//
pub const PRTCON: c_uint = 0x0c;

// VIDTCON0
pub const VIDTCON0: c_uint = 0x10;

pub const VIDTCON0_VBPDE_SHIFT: c_int = 24;
pub const VIDTCON0_VBPDE_LIMIT: c_uint = 0xff;

pub const VIDTCON0_VBPD_SHIFT: c_int = 16;
pub const VIDTCON0_VBPD_LIMIT: c_uint = 0xff;

pub const VIDTCON0_VFPD_SHIFT: c_int = 8;
pub const VIDTCON0_VFPD_LIMIT: c_uint = 0xff;

pub const VIDTCON0_VSPW_SHIFT: c_int = 0;
pub const VIDTCON0_VSPW_LIMIT: c_uint = 0xff;

// VIDTCON1
pub const VIDTCON1: c_uint = 0x14;

pub const VIDTCON1_VFPDE_SHIFT: c_int = 24;
pub const VIDTCON1_VFPDE_LIMIT: c_uint = 0xff;

pub const VIDTCON1_HBPD_SHIFT: c_int = 16;
pub const VIDTCON1_HBPD_LIMIT: c_uint = 0xff;

pub const VIDTCON1_HFPD_SHIFT: c_int = 8;
pub const VIDTCON1_HFPD_LIMIT: c_uint = 0xff;

pub const VIDTCON1_HSPW_SHIFT: c_int = 0;
pub const VIDTCON1_HSPW_LIMIT: c_uint = 0xff;

pub const VIDTCON2: c_uint = 0x18;

pub const VIDTCON2_LINEVAL_SHIFT: c_int = 11;
pub const VIDTCON2_LINEVAL_LIMIT: c_uint = 0x7ff;

pub const VIDTCON2_HOZVAL_SHIFT: c_int = 0;
pub const VIDTCON2_HOZVAL_LIMIT: c_uint = 0x7ff;

// WINCONx

pub const WINCONx_CSCWIDTH_SHIFT: c_int = 26;

pub const WINCONx_BURSTLEN_SHIFT: c_int = 9;

pub const WINCON0_BPPMODE_SHIFT: c_int = 2;

pub const WINCON1_BPPMODE_SHIFT: c_int = 2;

// S5PV210
pub const SHADOWCON: c_uint = 0x34;

// DMA channels (all windows)

// Local input channels (windows 0-2)

// VIDOSDx
pub const VIDOSD_BASE: c_uint = 0x40;

pub const VIDOSDxA_TOPLEFT_X_SHIFT: c_int = 11;
pub const VIDOSDxA_TOPLEFT_X_LIMIT: c_uint = 0x7ff;

pub const VIDOSDxA_TOPLEFT_Y_SHIFT: c_int = 0;
pub const VIDOSDxA_TOPLEFT_Y_LIMIT: c_uint = 0x7ff;

pub const VIDOSDxB_BOTRIGHT_X_SHIFT: c_int = 11;
pub const VIDOSDxB_BOTRIGHT_X_LIMIT: c_uint = 0x7ff;

pub const VIDOSDxB_BOTRIGHT_Y_SHIFT: c_int = 0;
pub const VIDOSDxB_BOTRIGHT_Y_LIMIT: c_uint = 0x7ff;

// For VIDOSD[1..4]C

pub const VIDISD14C_ALPHA0_G_SHIFT: c_int = 16;
pub const VIDISD14C_ALPHA0_G_LIMIT: c_uint = 0xf;

pub const VIDISD14C_ALPHA0_B_SHIFT: c_int = 12;
pub const VIDISD14C_ALPHA0_B_LIMIT: c_uint = 0xf;

pub const VIDISD14C_ALPHA1_R_SHIFT: c_int = 8;
pub const VIDISD14C_ALPHA1_R_LIMIT: c_uint = 0xf;

pub const VIDISD14C_ALPHA1_G_SHIFT: c_int = 4;
pub const VIDISD14C_ALPHA1_G_LIMIT: c_uint = 0xf;

pub const VIDISD14C_ALPHA1_B_SHIFT: c_int = 0;
pub const VIDISD14C_ALPHA1_B_LIMIT: c_uint = 0xf;

pub const VIDW_ALPHA: c_uint = 0x021c;

// Video buffer addresses

pub const VIDW_BUF_SIZE_OFFSET_SHIFT: c_int = 13;
pub const VIDW_BUF_SIZE_OFFSET_LIMIT: c_uint = 0x1fff;

pub const VIDW_BUF_SIZE_PAGEWIDTH_SHIFT: c_int = 0;
pub const VIDW_BUF_SIZE_PAGEWIDTH_LIMIT: c_uint = 0x1fff;

// Interrupt controls and status
pub const VIDINTCON0: c_uint = 0x130;

pub const VIDINTCON0_FIFOINTERVAL_SHIFT: c_int = 20;
pub const VIDINTCON0_FIFOINTERVAL_LIMIT: c_uint = 0x3f;

pub const VIDINTCON0_FRAMESEL0_SHIFT: c_int = 15;

pub const VIDINTCON0_FIFIOSEL_SHIFT: c_int = 5;

pub const VIDINTCON0_FIFOLEVEL_SHIFT: c_int = 2;

pub const VIDINTCON0_INT_FIFO_SHIFT: c_int = 0;

pub const VIDINTCON1: c_uint = 0x134;

// Window colour-key control registers
pub const WKEYCON: c_uint = 0x140;
pub const WKEYCON0: c_uint = 0x00;
pub const WKEYCON1: c_uint = 0x04;

pub const WxKEYCON0_COMPKEY_SHIFT: c_int = 0;
pub const WxKEYCON0_COMPKEY_LIMIT: c_uint = 0xffffff;

pub const WxKEYCON1_COLVAL_SHIFT: c_int = 0;
pub const WxKEYCON1_COLVAL_LIMIT: c_uint = 0xffffff;

// Dithering control
pub const DITHMODE: c_uint = 0x170;

pub const DITHMODE_R_POS_SHIFT: c_int = 5;

pub const DITHMODE_G_POS_SHIFT: c_int = 3;

pub const DITHMODE_B_POS_SHIFT: c_int = 1;

// Window blanking (MAP)

pub const WINxMAP_MAP_COLOUR_SHIFT: c_int = 0;
pub const WINxMAP_MAP_COLOUR_LIMIT: c_uint = 0xffffff;

// Winodw palette control
pub const WPALCON: c_uint = 0x1A0;

pub const WPALCON_W1PAL_SHIFT: c_int = 3;

pub const WPALCON_W0PAL_SHIFT: c_int = 0;

// Blending equation control

pub const BLENDEQ_ZERO: c_uint = 0x0;
pub const BLENDEQ_ONE: c_uint = 0x1;
pub const BLENDEQ_ALPHA_A: c_uint = 0x2;
pub const BLENDEQ_ONE_MINUS_ALPHA_A: c_uint = 0x3;
pub const BLENDEQ_ALPHA0: c_uint = 0x6;

pub const BLENDCON: c_uint = 0x260;

// Display port clock control
pub const DP_MIE_CLKCON: c_uint = 0x27c;
pub const DP_MIE_CLK_DISABLE: c_uint = 0x0;
pub const DP_MIE_CLK_DP_ENABLE: c_uint = 0x2;
pub const DP_MIE_CLK_MIE_ENABLE: c_uint = 0x3;
// Notes on per-window bpp settings
//
// Value	Win0	 Win1	  Win2	   Win3	    Win 4
// 0000		1(P)	 1(P)	  1(P)	   1(P)	    1(P)
// 0001		2(P)	 2(P)     2(P)	   2(P)	    2(P)
// 0010		4(P)	 4(P)     4(P)	   4(P)     -none-
// 0011		8(P)	 8(P)     -none-   -none-   -none-
// 0100		-none-	 8(A232)  8(A232)  -none-   -none-
// 0101		16(565)	 16(565)  16(565)  16(565)   16(565)
// 0110		-none-	 16(A555) 16(A555) 16(A555)  16(A555)
// 0111		16(I555) 16(I565) 16(I555) 16(I555)  16(I555)
// 1000		18(666)	 18(666)  18(666)  18(666)   18(666)
// 1001		-none-	 18(A665) 18(A665) 18(A665)  16(A665)
// 1010		-none-	 19(A666) 19(A666) 19(A666)  19(A666)
// 1011		24(888)	 24(888)  24(888)  24(888)   24(888)
// 1100		-none-	 24(A887) 24(A887) 24(A887)  24(A887)
// 1101		-none-	 25(A888) 25(A888) 25(A888)  25(A888)
// 1110		-none-	 -none-	  -none-   -none-    -none-
// 1111		-none-	 -none-   -none-   -none-    -none-
//

// FIMD Version 8 register offset definitions
pub const FIMD_V8_VIDTCON0: c_uint = 0x20010;
pub const FIMD_V8_VIDTCON1: c_uint = 0x20014;
pub const FIMD_V8_VIDTCON2: c_uint = 0x20018;
pub const FIMD_V8_VIDTCON3: c_uint = 0x2001C;
pub const FIMD_V8_VIDCON1: c_uint = 0x20004;
