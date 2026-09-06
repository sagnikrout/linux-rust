//! Automatically rewritten from C Header to Rust Module
//! Source: include/video/sstfb.h
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
// linux/drivers/video/sstfb.h -- voodoo graphics frame buffer
//
// Copyright (c) 2000,2001 Ghozlane Toumi <gtoumi@messel.emse.fr>
//
// Created 28 Aug 2001 by Ghozlane Toumi
//
// Debug Stuff
//

//
// Const
//
// pci stuff
pub const PCI_INIT_ENABLE: c_uint = 0x40;

pub const PCI_VCLK_ENABLE: c_uint = 0xc0	/* enable video */;
pub const PCI_VCLK_DISABLE: c_uint = 0xe0;
// register offsets from memBaseAddr
pub const STATUS: c_uint = 0x0000;

pub const FBZMODE: c_uint = 0x0110;

pub const LFBMODE: c_uint = 0x0114;

pub const CLIP_LEFT_RIGHT: c_uint = 0x0118;
pub const CLIP_LOWY_HIGHY: c_uint = 0x011c;
pub const NOPCMD: c_uint = 0x0120;
pub const FASTFILLCMD: c_uint = 0x0124;
pub const SWAPBUFFCMD: c_uint = 0x0128;
pub const FBIINIT4: c_uint = 0x0200		/* misc controls */;

pub const BACKPORCH: c_uint = 0x0208;
pub const VIDEODIMENSIONS: c_uint = 0x020c;
pub const FBIINIT0: c_uint = 0x0210		/* misc+fifo  controls */;

pub const FBIINIT1: c_uint = 0x0214		/* PCI + video controls */;

pub const FBIINIT2: c_uint = 0x0218		/* Dram controls */;

pub const FBIINIT3: c_uint = 0x021c		/* fbi controls */;

pub const HSYNC: c_uint = 0x0220;
pub const VSYNC: c_uint = 0x0224;
pub const DAC_DATA: c_uint = 0x022c;

pub const FBIINIT5: c_uint = 0x0244		/* v2 specific */;

pub const FBIINIT6: c_uint = 0x0248		/* v2 specific */;

pub const FBIINIT7: c_uint = 0x024c		/* v2 specific */;
pub const BLTSRCBASEADDR: c_uint = 0x02c0	/* BitBLT Source base address */;
pub const BLTDSTBASEADDR: c_uint = 0x02c4	/* BitBLT Destination base address */;
pub const BLTXYSTRIDES: c_uint = 0x02c8	/* BitBLT Source and Destination strides */;
pub const BLTSRCCHROMARANGE: c_uint = 0x02cc	/* BitBLT Source Chroma key range */;
pub const BLTDSTCHROMARANGE: c_uint = 0x02d0	/* BitBLT Destination Chroma key range */;
pub const BLTCLIPX: c_uint = 0x02d4	/* BitBLT Min/Max X clip values */;
pub const BLTCLIPY: c_uint = 0x02d8	/* BitBLT Min/Max Y clip values */;
pub const BLTSRCXY: c_uint = 0x02e0	/* BitBLT Source starting XY coordinates */;
pub const BLTDSTXY: c_uint = 0x02e4	/* BitBLT Destination starting XY coordinates */;
pub const BLTSIZE: c_uint = 0x02e8	/* BitBLT width and height */;
pub const BLTROP: c_uint = 0x02ec	/* BitBLT Raster operations */;

pub const BLTCOLOR: c_uint = 0x02f0	/* BitBLT and foreground background colors */;
pub const BLTCOMMAND: c_uint = 0x02f8	/* BitBLT command mode (v2 specific) */;

pub const BLTDATA: c_uint = 0x02fc	/* BitBLT data for CPU-to-Screen BitBLTs */;

// Dac Registers
pub const DACREG_WMA: c_uint = 0x0	/* pixel write mode address */;
pub const DACREG_LUT: c_uint = 0x01	/* color value */;
pub const DACREG_RMR: c_uint = 0x02	/* pixel mask */;
pub const DACREG_RMA: c_uint = 0x03	/* pixel read mode address */;
// Dac registers in indexed mode (TI, ATT dacs)

pub const DACREG_RMR_I: c_uint = 0x00;
pub const DACREG_CR0_I: c_uint = 0x01;

pub const DACREG_CR1_I: c_uint = 0x05;
pub const DACREG_CC_I: c_uint = 0x06;

pub const DACREG_AC0_I: c_uint = 0x48		/* clock A reg C */;
pub const DACREG_AC1_I: c_uint = 0x49;
pub const DACREG_BD0_I: c_uint = 0x6c		/* clock B reg D */;
pub const DACREG_BD1_I: c_uint = 0x6d;
// identification constants
pub const DACREG_MIR_TI: c_uint = 0x97;
pub const DACREG_DIR_TI: c_uint = 0x09;
pub const DACREG_MIR_ATT: c_uint = 0x84;
pub const DACREG_DIR_ATT: c_uint = 0x09;
// ics dac specific registers
pub const DACREG_ICS_PLLWMA: c_uint = 0x04	/* PLL write mode address */;
pub const DACREG_ICS_PLLDATA: c_uint = 0x05	/* PLL data /parameter */;
pub const DACREG_ICS_CMD: c_uint = 0x06	/* command */;

pub const DACREG_ICS_PLLRMA: c_uint = 0x07	/* PLL read mode address */;
//
// pll parameter register:
// indexed : write addr to PLLWMA, write data in PLLDATA.
// for reads use PLLRMA .
// 8 freq registers (0-7) for video clock (CLK0)
// 2 freq registers (a-b) for graphic clock (CLK1)
//
pub const DACREG_ICS_PLL_CLK0_1_INI: c_uint = 0x55	/* initial pll M value for freq f1  */;
pub const DACREG_ICS_PLL_CLK0_7_INI: c_uint = 0x71	/* f7 */;
pub const DACREG_ICS_PLL_CLK1_B_INI: c_uint = 0x79	/* fb */;
pub const DACREG_ICS_PLL_CTRL: c_uint = 0x0e;

// sst default init registers

// SLOW_PCI_WRITES*/	\

// SLOW_PCI_READS*/	\
// Careful with this one : writing back the data just read will trash the DAC
//

//
// Misc Const
//
// ioctl to enable/disable VGA passthrough

// used to know witch clock to set
// freq max

pub const VCO_MAX: c_int = 260000;
//
// driver structs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pll_timing {
    pub m: c_uint,
    pub n: c_uint,
    pub p: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dac_switch {
    pub name: *const c_char,
    pub info): *mut *mut int (detect) (struct fb_info,
    pub clock): *const *const *const *const int (set_pll) (struct fb_info info, struct pll_timing t, int,
    pub bpp): *const *const *const void (set_vidmod) (struct fb_info info, int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sst_spec {
    pub name: *mut *mut c_char,
    pub /: *mut *mut int default_gfx_clock; / 50000 for voodoo1, 75000 for voodoo2,
    pub /: *mut *mut int max_gfxclk; / ! in Mhz ie 60 for voodoo 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sstfb_par {
    pub palette: [u32; 16],
    pub yDim: c_uint,
    pub /: *mut *mut unsigned int hSyncOn; / hsync_len,
    pub /: *mut *mut unsigned int hSyncOff; / left_margin + xres + right_margin,
    pub /: *mut *mut unsigned int hBackPorch;/ left_margin,
    pub vSyncOn: c_uint,
    pub vSyncOff: c_uint,
    pub vBackPorch: c_uint,
    pub pll: pll_timing,
    pub /: *mut *mut unsigned int tiles_in_X;/ num of tiles in X res,
    pub mmio_vbase: *mut u8 __iomem,
    pub /: *mut *mut dac_switch dac_sw; / dac specific functions,
    pub dev: *mut pci_dev,
    pub type: c_int,
    pub revision: u8,
    pub /: *mut *mut u8 vgapass; / VGA pass through: 1=enabled, 0=disabled,
}
