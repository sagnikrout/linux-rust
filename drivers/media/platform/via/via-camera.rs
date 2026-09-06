//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/via/via-camera.h
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
// VIA Camera register definitions.
//
pub const VCR_INTCTRL: c_uint = 0x300	/* Capture interrupt control */;
pub const VCR_IC_EAV: c_uint = 0x0001   /* End of active video status */;
pub const VCR_IC_EVBI: c_uint = 0x0002   /* End of VBI status */;
pub const VCR_IC_FBOTFLD: c_uint = 0x0004   /* "flipping" Bottom field is active */;
pub const VCR_IC_ACTBUF: c_uint = 0x0018   /* Active video buffer  */;
pub const VCR_IC_VSYNC: c_uint = 0x0020   /* 0 = VB, 1 = active video */;
pub const VCR_IC_BOTFLD: c_uint = 0x0040   /* Bottom field is active */;
pub const VCR_IC_FFULL: c_uint = 0x0080   /* FIFO full */;
pub const VCR_IC_INTEN: c_uint = 0x0100   /* End of active video int. enable */;
pub const VCR_IC_VBIINT: c_uint = 0x0200   /* End of VBI int enable */;
pub const VCR_IC_VBIBUF: c_uint = 0x0400   /* Current VBI buffer */;
pub const VCR_TSC: c_uint = 0x308	/* Transport stream control */;
pub const VCR_TSC_ENABLE: c_uint = 0x000001   /* Transport stream input enable */;
pub const VCR_TSC_DROPERR: c_uint = 0x000002   /* Drop error packets */;
pub const VCR_TSC_METHOD: c_uint = 0x00000c   /* DMA method (non-functional) */;
pub const VCR_TSC_COUNT: c_uint = 0x07fff0   /* KByte or packet count */;
pub const VCR_TSC_CBMODE: c_uint = 0x080000   /* Change buffer by byte count */;
pub const VCR_TSC_PSSIG: c_uint = 0x100000   /* Packet starting signal disable */;
pub const VCR_TSC_BE: c_uint = 0x200000   /* MSB first (serial mode) */;
pub const VCR_TSC_SERIAL: c_uint = 0x400000   /* Serial input (0 = parallel) */;
pub const VCR_CAPINTC: c_uint = 0x310	/* Capture interface control */;
pub const VCR_CI_ENABLE: c_uint = 0x00000001  /* Capture enable */;
pub const VCR_CI_BSS: c_uint = 0x00000002  /* WTF "bit stream selection" */;
pub const VCR_CI_3BUFS: c_uint = 0x00000004  /* 1 = 3 buffers, 0 = 2 buffers */;
pub const VCR_CI_VIPEN: c_uint = 0x00000008  /* VIP enable */;

pub const VCR_CI_CCIR656_8: c_uint = 0x00000010  /* ... CCIR656, 8 bit */;
pub const VCR_CI_CCIR601_16: c_uint = 0x00000020  /* ... CCIR601, 16 bit */;
pub const VCR_CI_CCIR656_16: c_uint = 0x00000030  /* ... CCIR656, 16 bit */;
pub const VCR_CI_HDMODE: c_uint = 0x00000040  /* CCIR656-16 hdr decode mode; 1=16b */;
pub const VCR_CI_BSWAP: c_uint = 0x00000080  /* Swap bytes (16-bit) */;

pub const VCR_CI_UYVY: c_uint = 0x00000100  /* Byte order 1032 */;
pub const VCR_CI_YVYU: c_uint = 0x00000200  /* Byte order 0321 */;
pub const VCR_CI_VYUY: c_uint = 0x00000300  /* Byte order 3012 */;
pub const VCR_CI_VIPTYPE: c_uint = 0x00000400  /* VIP type */;
pub const VCR_CI_IFSEN: c_uint = 0x00000800  /* Input field signal enable */;

pub const VCR_CI_DIEVEN: c_uint = 0x00001000  /*    ...even field, 30fps */;
pub const VCR_CI_DIBOTH: c_uint = 0x00002000  /*    ...both fields, 60fps */;
pub const VCR_CI_DIBOTH30: c_uint = 0x00003000  /*    ...both fields, 30fps interlace */;
pub const VCR_CI_CONVTYPE: c_uint = 0x00004000  /* 4:2:2 to 4:4:4; 1 = interpolate */;
pub const VCR_CI_CFC: c_uint = 0x00008000  /* Capture flipping control */;
pub const VCR_CI_FILTER: c_uint = 0x00070000  /* Horiz filter mode select;
pub const VCR_CI_CLKINV: c_uint = 0x00080000  /* Input CLK inverted */;
pub const VCR_CI_VREFINV: c_uint = 0x00100000  /* VREF inverted */;
pub const VCR_CI_HREFINV: c_uint = 0x00200000  /* HREF inverted */;
pub const VCR_CI_FLDINV: c_uint = 0x00400000  /* Field inverted */;
pub const VCR_CI_CLKPIN: c_uint = 0x00800000  /* Capture clock pin */;
pub const VCR_CI_THRESH: c_uint = 0x0f000000  /* Capture fifo threshold */;
pub const VCR_CI_HRLE: c_uint = 0x10000000  /* Positive edge of HREF */;
pub const VCR_CI_VRLE: c_uint = 0x20000000  /* Positive edge of VREF */;
pub const VCR_CI_OFLDINV: c_uint = 0x40000000  /* Field output inverted */;
pub const VCR_CI_CLKEN: c_uint = 0x80000000  /* Capture clock enable */;
pub const VCR_HORRANGE: c_uint = 0x314	/* Active video horizontal range */;
pub const VCR_VERTRANGE: c_uint = 0x318	/* Active video vertical range */;
pub const VCR_AVSCALE: c_uint = 0x31c	/* Active video scaling control */;
pub const VCR_AVS_HEN: c_uint = 0x00000800   /* Horizontal scale enable */;
pub const VCR_AVS_VEN: c_uint = 0x04000000   /* Vertical enable */;
pub const VCR_VBIHOR: c_uint = 0x320	/* VBI Data horizontal range */;
pub const VCR_VBIVERT: c_uint = 0x324	/* VBI data vertical range */;
pub const VCR_VBIBUF1: c_uint = 0x328	/* First VBI buffer */;
pub const VCR_VBISTRIDE: c_uint = 0x32c	/* VBI stride */;
pub const VCR_ANCDATACNT: c_uint = 0x330	/* Ancillary data count setting */;
pub const VCR_MAXDATA: c_uint = 0x334	/* Active data count of active video */;
pub const VCR_MAXVBI: c_uint = 0x338	/* Maximum data count of VBI */;
pub const VCR_CAPDATA: c_uint = 0x33c	/* Capture data count */;
pub const VCR_VBUF1: c_uint = 0x340	/* First video buffer */;
pub const VCR_VBUF2: c_uint = 0x344	/* Second video buffer */;
pub const VCR_VBUF3: c_uint = 0x348	/* Third video buffer */;
pub const VCR_VBUF_MASK: c_uint = 0x1ffffff0	/* Bits 28:4 */;
pub const VCR_VBIBUF2: c_uint = 0x34c	/* Second VBI buffer */;
pub const VCR_VSTRIDE: c_uint = 0x350	/* Stride of video + coring control */;
pub const VCR_VS_STRIDE_SHIFT: c_int = 4;
pub const VCR_VS_STRIDE: c_uint = 0x00001ff0  /* Stride (8-byte units) */;
pub const VCR_VS_CCD: c_uint = 0x007f0000  /* Coring compare data */;
pub const VCR_VS_COREEN: c_uint = 0x00800000  /* Coring enable */;
pub const VCR_TS0ERR: c_uint = 0x354	/* TS buffer 0 error indicator */;
pub const VCR_TS1ERR: c_uint = 0x358	/* TS buffer 0 error indicator */;
pub const VCR_TS2ERR: c_uint = 0x35c	/* TS buffer 0 error indicator */;
// Add 0x1000 for the second capture engine registers
