//! Automatically rewritten from C Header to Rust Module
//! Source: include/video/tgafb.h
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


//
// linux/drivers/video/tgafb.h -- DEC 21030 TGA frame buffer device
//
// Copyright (C) 1999,2000 Martin Lucina, Tom Zerucha
//
// $Id: tgafb.h,v 1.4.2.3 2000/04/04 06:44:56 mato Exp $
//
// This file is subject to the terms and conditions of the GNU General Public
// License. See the file COPYING in the main directory of this archive for
// more details.
//
// TGA hardware description (minimal)
//
pub const TGA_TYPE_8PLANE: c_int = 0;
pub const TGA_TYPE_24PLANE: c_int = 1;
pub const TGA_TYPE_24PLUSZ: c_int = 3;
//
// Offsets within Memory Space
//
pub const TGA_ROM_OFFSET: c_uint = 0x0000000;
pub const TGA_REGS_OFFSET: c_uint = 0x0100000;
pub const TGA_8PLANE_FB_OFFSET: c_uint = 0x0200000;
pub const TGA_24PLANE_FB_OFFSET: c_uint = 0x0804000;
pub const TGA_24PLUSZ_FB_OFFSET: c_uint = 0x1004000;
pub const TGA_FOREGROUND_REG: c_uint = 0x0020;
pub const TGA_BACKGROUND_REG: c_uint = 0x0024;
pub const TGA_PLANEMASK_REG: c_uint = 0x0028;
pub const TGA_PIXELMASK_ONESHOT_REG: c_uint = 0x002c;
pub const TGA_MODE_REG: c_uint = 0x0030;
pub const TGA_RASTEROP_REG: c_uint = 0x0034;
pub const TGA_PIXELSHIFT_REG: c_uint = 0x0038;
pub const TGA_DEEP_REG: c_uint = 0x0050;
pub const TGA_START_REG: c_uint = 0x0054;
pub const TGA_PIXELMASK_REG: c_uint = 0x005c;
pub const TGA_CURSOR_BASE_REG: c_uint = 0x0060;
pub const TGA_HORIZ_REG: c_uint = 0x0064;
pub const TGA_VERT_REG: c_uint = 0x0068;
pub const TGA_BASE_ADDR_REG: c_uint = 0x006c;
pub const TGA_VALID_REG: c_uint = 0x0070;
pub const TGA_CURSOR_XY_REG: c_uint = 0x0074;
pub const TGA_INTR_STAT_REG: c_uint = 0x007c;
pub const TGA_DATA_REG: c_uint = 0x0080;
pub const TGA_RAMDAC_SETUP_REG: c_uint = 0x00c0;
pub const TGA_BLOCK_COLOR0_REG: c_uint = 0x0140;
pub const TGA_BLOCK_COLOR1_REG: c_uint = 0x0144;
pub const TGA_BLOCK_COLOR2_REG: c_uint = 0x0148;
pub const TGA_BLOCK_COLOR3_REG: c_uint = 0x014c;
pub const TGA_BLOCK_COLOR4_REG: c_uint = 0x0150;
pub const TGA_BLOCK_COLOR5_REG: c_uint = 0x0154;
pub const TGA_BLOCK_COLOR6_REG: c_uint = 0x0158;
pub const TGA_BLOCK_COLOR7_REG: c_uint = 0x015c;
pub const TGA_COPY64_SRC: c_uint = 0x0160;
pub const TGA_COPY64_DST: c_uint = 0x0164;
pub const TGA_CLOCK_REG: c_uint = 0x01e8;
pub const TGA_RAMDAC_REG: c_uint = 0x01f0;
pub const TGA_CMD_STAT_REG: c_uint = 0x01f8;
//
// Useful defines for managing the registers
//
pub const TGA_HORIZ_ODD: c_uint = 0x80000000;
pub const TGA_HORIZ_POLARITY: c_uint = 0x40000000;
pub const TGA_HORIZ_ACT_MSB: c_uint = 0x30000000;
pub const TGA_HORIZ_BP: c_uint = 0x0fe00000;
pub const TGA_HORIZ_SYNC: c_uint = 0x001fc000;
pub const TGA_HORIZ_FP: c_uint = 0x00007c00;
pub const TGA_HORIZ_ACT_LSB: c_uint = 0x000001ff;
pub const TGA_VERT_SE: c_uint = 0x80000000;
pub const TGA_VERT_POLARITY: c_uint = 0x40000000;
pub const TGA_VERT_RESERVED: c_uint = 0x30000000;
pub const TGA_VERT_BP: c_uint = 0x0fc00000;
pub const TGA_VERT_SYNC: c_uint = 0x003f0000;
pub const TGA_VERT_FP: c_uint = 0x0000f800;
pub const TGA_VERT_ACTIVE: c_uint = 0x000007ff;
pub const TGA_VALID_VIDEO: c_uint = 0x01;
pub const TGA_VALID_BLANK: c_uint = 0x02;
pub const TGA_VALID_CURSOR: c_uint = 0x04;
pub const TGA_MODE_SBM_8BPP: c_uint = 0x000;
pub const TGA_MODE_SBM_24BPP: c_uint = 0x300;
pub const TGA_MODE_SIMPLE: c_uint = 0x00;
pub const TGA_MODE_SIMPLEZ: c_uint = 0x10;
pub const TGA_MODE_OPAQUE_STIPPLE: c_uint = 0x01;
pub const TGA_MODE_OPAQUE_FILL: c_uint = 0x21;
pub const TGA_MODE_TRANSPARENT_STIPPLE: c_uint = 0x03;
pub const TGA_MODE_TRANSPARENT_FILL: c_uint = 0x23;
pub const TGA_MODE_BLOCK_STIPPLE: c_uint = 0x0d;
pub const TGA_MODE_BLOCK_FILL: c_uint = 0x2d;
pub const TGA_MODE_COPY: c_uint = 0x07;
pub const TGA_MODE_DMA_READ_COPY_ND: c_uint = 0x17;
pub const TGA_MODE_DMA_READ_COPY_D: c_uint = 0x37;
pub const TGA_MODE_DMA_WRITE_COPY: c_uint = 0x1f;
//
// Useful defines for managing the ICS1562 PLL clock
//

pub const TGA_PLL_MAX_FREQ: c_int = 230000;
//
// Useful defines for managing the BT485 on the 8-plane TGA
//
pub const BT485_READ_BIT: c_uint = 0x01;
pub const BT485_WRITE_BIT: c_uint = 0x00;
pub const BT485_ADDR_PAL_WRITE: c_uint = 0x00;
pub const BT485_DATA_PAL: c_uint = 0x02;
pub const BT485_PIXEL_MASK: c_uint = 0x04;
pub const BT485_ADDR_PAL_READ: c_uint = 0x06;
pub const BT485_ADDR_CUR_WRITE: c_uint = 0x08;
pub const BT485_DATA_CUR: c_uint = 0x0a;
pub const BT485_CMD_0: c_uint = 0x0c;
pub const BT485_ADDR_CUR_READ: c_uint = 0x0e;
pub const BT485_CMD_1: c_uint = 0x10;
pub const BT485_CMD_2: c_uint = 0x12;
pub const BT485_STATUS: c_uint = 0x14;
pub const BT485_CMD_3: c_uint = 0x14;
pub const BT485_CUR_RAM: c_uint = 0x16;
pub const BT485_CUR_LOW_X: c_uint = 0x18;
pub const BT485_CUR_HIGH_X: c_uint = 0x1a;
pub const BT485_CUR_LOW_Y: c_uint = 0x1c;
pub const BT485_CUR_HIGH_Y: c_uint = 0x1e;
//
// Useful defines for managing the BT463 on the 24-plane TGAs/SFB+s
//
pub const BT463_ADDR_LO: c_uint = 0x0;
pub const BT463_ADDR_HI: c_uint = 0x1;
pub const BT463_REG_ACC: c_uint = 0x2;
pub const BT463_PALETTE: c_uint = 0x3;
pub const BT463_CUR_CLR_0: c_uint = 0x0100;
pub const BT463_CUR_CLR_1: c_uint = 0x0101;
pub const BT463_CMD_REG_0: c_uint = 0x0201;
pub const BT463_CMD_REG_1: c_uint = 0x0202;
pub const BT463_CMD_REG_2: c_uint = 0x0203;
pub const BT463_READ_MASK_0: c_uint = 0x0205;
pub const BT463_READ_MASK_1: c_uint = 0x0206;
pub const BT463_READ_MASK_2: c_uint = 0x0207;
pub const BT463_READ_MASK_3: c_uint = 0x0208;
pub const BT463_BLINK_MASK_0: c_uint = 0x0209;
pub const BT463_BLINK_MASK_1: c_uint = 0x020a;
pub const BT463_BLINK_MASK_2: c_uint = 0x020b;
pub const BT463_BLINK_MASK_3: c_uint = 0x020c;
pub const BT463_WINDOW_TYPE_BASE: c_uint = 0x0300;
//
// Useful defines for managing the BT459 on the 8-plane SFB+s
//
pub const BT459_ADDR_LO: c_uint = 0x0;
pub const BT459_ADDR_HI: c_uint = 0x1;
pub const BT459_REG_ACC: c_uint = 0x2;
pub const BT459_PALETTE: c_uint = 0x3;
pub const BT459_CUR_CLR_1: c_uint = 0x0181;
pub const BT459_CUR_CLR_2: c_uint = 0x0182;
pub const BT459_CUR_CLR_3: c_uint = 0x0183;
pub const BT459_CMD_REG_0: c_uint = 0x0201;
pub const BT459_CMD_REG_1: c_uint = 0x0202;
pub const BT459_CMD_REG_2: c_uint = 0x0203;
pub const BT459_READ_MASK: c_uint = 0x0204;
pub const BT459_BLINK_MASK: c_uint = 0x0206;
pub const BT459_CUR_CMD_REG: c_uint = 0x0300;
//
// The framebuffer driver private data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tga_par {
// PCI/TC device.
    pub dev: *mut device,
// Device dependent information.
    pub tga_mem_base: *mut void __iomem,
    pub tga_fb_base: *mut void __iomem,
    pub tga_regs_base: *mut void __iomem,
    pub /: *mut *mut u8 tga_type; / TGA_TYPE_XXX,
    pub /: *mut *mut u8 tga_chip_rev; / dc21030 revision,
// Remember blank mode.
    pub vesa_blanked: u8,
// Define the video mode.
    pub /: *mut *mut u32 xres, yres; / resolution in pixels,
    pub /: *mut *mut u32 htimings; / horizontal timing register,
    pub /: *mut *mut u32 vtimings; / vertical timing register,
    pub /: *mut *mut u32 pll_freq; / pixclock in mhz,
    pub /: *mut *mut u32 bits_per_pixel; / bits per pixel,
    pub /: *mut *mut u32 sync_on_green; / set if sync is on green,
    pub palette: [u32; 16],
}

//
// Macros for reading/writing TGA and RAMDAC registers
//
extern "C" {
    pub fn readl(+r: par->tga_regs_base) -> return;
}
