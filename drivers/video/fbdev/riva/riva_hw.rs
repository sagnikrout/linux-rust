//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/riva/riva_hw.h
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


// \
//
// GPL licensing note -- nVidia is allowing a liberal interpretation of
// the documentation restriction above, to merely say that this nVidia's
// copyright and disclaimer should be included with all code derived
// from this source.  -- Jeff Garzik <jgarzik@pobox.com>, 01/Nov/99
//
// $XFree86: xc/programs/Xserver/hw/xfree86/drivers/nv/riva_hw.h,v 1.21 2002/10/14 18:22:46 mvojkovi Exp $
pub const RIVA_SW_VERSION: c_uint = 0x00010003;

pub type Bool = c_int;

pub const TRUE: c_int = 1;

pub const FALSE: c_int = 0;

pub const NULL: c_int = 0;

//
// Typedefs to force certain sized values.
//
pub type U008 = c_uchar;
pub type U016 = c_ushort;
pub type U032 = c_uint;
//
// HW access macros.
//

//
// Define different architectures.
//
pub const NV_ARCH_03: c_uint = 0x03;
pub const NV_ARCH_04: c_uint = 0x04;
pub const NV_ARCH_10: c_uint = 0x10;
pub const NV_ARCH_20: c_uint = 0x20;
pub const NV_ARCH_30: c_uint = 0x30;
pub const NV_ARCH_40: c_uint = 0x40;
// \
//
// FIFO registers.
//
// Raster OPeration. Windows style ROP3.
//

//
// 8X8 Monochrome pattern.
//

//
// Scissor clip rectangle.
//

//
// 2D filled rectangle.
//

//
// 2D screen-screen BLT.
//

//
// 2D pixel BLT.
//

//
// Filled rectangle combined with monochrome expand.  Useful for glyphs.
//

//
// 3D textured, Z buffered triangle.
//

// This is a problem on LynxOS

// This is a problem on LynxOS

//
// 2D line.
//

//
// 2D/3D surfaces
//

// \
//
// Virtualized RIVA H/W interface.
//
pub const FP_ENABLE: c_int = 1;
pub const FP_DITHER: c_int = 2;
//
// Virtialized chip interface. Makes RIVA 128 and TNT look alike.
//
// Chip specific settings.
//
// Non-FIFO registers.
//
// Common chip functions.
//
// Current extended mode settings.
//
// FIFO registers.
//
// Extended mode state information.
//
// function prototypes
//
// External routines.
//
extern "C" {
    pub fn RivaGetConfig(chip: *mut RIVA_HW_INST, pdev: *mut pci_dev, c: c_uint) -> c_int;
}
//
// FIFO Free Count. Should attempt to yield processor if RIVA is busy.
//

