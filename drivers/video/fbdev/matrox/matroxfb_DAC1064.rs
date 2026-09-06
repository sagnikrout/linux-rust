//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/matrox/matroxfb_DAC1064.h
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

extern "C" {
    pub fn DAC1064_global_init(minfo: *mut matrox_fb_info);
}
extern "C" {
    pub fn DAC1064_global_restore(minfo: *mut matrox_fb_info);
}

pub const M1064_INDEX: c_uint = 0x00;
pub const M1064_PALWRADD: c_uint = 0x00;
pub const M1064_PALDATA: c_uint = 0x01;
pub const M1064_PIXRDMSK: c_uint = 0x02;
pub const M1064_PALRDADD: c_uint = 0x03;
pub const M1064_X_DATAREG: c_uint = 0x0A;
pub const M1064_CURPOSXL: c_uint = 0x0C	/* can be accessed as DWORD */;
pub const M1064_CURPOSXH: c_uint = 0x0D;
pub const M1064_CURPOSYL: c_uint = 0x0E;
pub const M1064_CURPOSYH: c_uint = 0x0F;
pub const M1064_XCURADDL: c_uint = 0x04;
pub const M1064_XCURADDH: c_uint = 0x05;
pub const M1064_XCURCTRL: c_uint = 0x06;
pub const M1064_XCURCTRL_DIS: c_uint = 0x00	/* transparent, transparent, transparent, transparent */;
pub const M1064_XCURCTRL_3COLOR: c_uint = 0x01	/* transparent, 0, 1, 2 */;
pub const M1064_XCURCTRL_XGA: c_uint = 0x02	/* 0, 1, transparent, complement */;
pub const M1064_XCURCTRL_XWIN: c_uint = 0x03	/* transparent, transparent, 0, 1 */;
// drive DVI by standard(0)/DVI(1) PLL
// if set(1), C?DVICLKEN and C?DVICLKSEL must be set(1)
pub const M1064_XDVICLKCTRL_DVIDATAPATHSEL: c_uint = 0x01;
// drive CRTC1 by standard(0)/DVI(1) PLL
pub const M1064_XDVICLKCTRL_C1DVICLKSEL: c_uint = 0x02;
// drive CRTC2 by standard(0)/DVI(1) PLL
pub const M1064_XDVICLKCTRL_C2DVICLKSEL: c_uint = 0x04;
// pixel clock allowed to(0)/blocked from(1) driving CRTC1
pub const M1064_XDVICLKCTRL_C1DVICLKEN: c_uint = 0x08;
// DVI PLL loop filter bandwidth selection bits
pub const M1064_XDVICLKCTRL_DVILOOPCTL: c_uint = 0x30;
// CRTC2 pixel clock allowed to(0)/blocked from(1) driving CRTC2
pub const M1064_XDVICLKCTRL_C2DVICLKEN: c_uint = 0x40;
// P1PLL loop filter bandwidth selection
pub const M1064_XDVICLKCTRL_P1LOOPBWDTCTL: c_uint = 0x80;
pub const M1064_XCURCOL0RED: c_uint = 0x08;
pub const M1064_XCURCOL0GREEN: c_uint = 0x09;
pub const M1064_XCURCOL0BLUE: c_uint = 0x0A;
pub const M1064_XCURCOL1RED: c_uint = 0x0C;
pub const M1064_XCURCOL1GREEN: c_uint = 0x0D;
pub const M1064_XCURCOL1BLUE: c_uint = 0x0E;
pub const M1064_XDVICLKCTRL: c_uint = 0x0F;
pub const M1064_XCURCOL2RED: c_uint = 0x10;
pub const M1064_XCURCOL2GREEN: c_uint = 0x11;
pub const M1064_XCURCOL2BLUE: c_uint = 0x12;
pub const DAC1064_XVREFCTRL: c_uint = 0x18;
pub const DAC1064_XVREFCTRL_INTERNAL: c_uint = 0x3F;
pub const DAC1064_XVREFCTRL_EXTERNAL: c_uint = 0x00;
pub const DAC1064_XVREFCTRL_G100_DEFAULT: c_uint = 0x03;
pub const M1064_XMULCTRL: c_uint = 0x19;
pub const M1064_XMULCTRL_DEPTH_8BPP: c_uint = 0x00	/* 8 bpp paletized */;
pub const M1064_XMULCTRL_DEPTH_15BPP_1BPP: c_uint = 0x01	/* 15 bpp paletized + 1 bpp overlay */;
pub const M1064_XMULCTRL_DEPTH_16BPP: c_uint = 0x02	/* 16 bpp paletized */;
pub const M1064_XMULCTRL_DEPTH_24BPP: c_uint = 0x03	/* 24 bpp paletized */;
pub const M1064_XMULCTRL_DEPTH_24BPP_8BPP: c_uint = 0x04	/* 24 bpp direct + 8 bpp overlay paletized */;
pub const M1064_XMULCTRL_2G8V16: c_uint = 0x05	/* 15 bpp video direct, half xres, 8bpp paletized */;
pub const M1064_XMULCTRL_G16V16: c_uint = 0x06	/* 15 bpp video, 15bpp graphics, one of them paletized */;
pub const M1064_XMULCTRL_DEPTH_32BPP: c_uint = 0x07	/* 24 bpp paletized + 8 bpp unused */;
pub const M1064_XMULCTRL_GRAPHICS_PALETIZED: c_uint = 0x00;
pub const M1064_XMULCTRL_VIDEO_PALETIZED: c_uint = 0x08;
pub const M1064_XPIXCLKCTRL: c_uint = 0x1A;
pub const M1064_XPIXCLKCTRL_SRC_PCI: c_uint = 0x00;
pub const M1064_XPIXCLKCTRL_SRC_PLL: c_uint = 0x01;
pub const M1064_XPIXCLKCTRL_SRC_EXT: c_uint = 0x02;
pub const M1064_XPIXCLKCTRL_SRC_SYS: c_uint = 0x03	/* G200/G400 */;
pub const M1064_XPIXCLKCTRL_SRC_PLL2: c_uint = 0x03	/* G450 */;
pub const M1064_XPIXCLKCTRL_SRC_MASK: c_uint = 0x03;
pub const M1064_XPIXCLKCTRL_EN: c_uint = 0x00;
pub const M1064_XPIXCLKCTRL_DIS: c_uint = 0x04;
pub const M1064_XPIXCLKCTRL_PLL_DOWN: c_uint = 0x00;
pub const M1064_XPIXCLKCTRL_PLL_UP: c_uint = 0x08;
pub const M1064_XGENCTRL: c_uint = 0x1D;
pub const M1064_XGENCTRL_VS_0: c_uint = 0x00;
pub const M1064_XGENCTRL_VS_1: c_uint = 0x01;
pub const M1064_XGENCTRL_ALPHA_DIS: c_uint = 0x00;
pub const M1064_XGENCTRL_ALPHA_EN: c_uint = 0x02;
pub const M1064_XGENCTRL_BLACK_0IRE: c_uint = 0x00;
pub const M1064_XGENCTRL_BLACK_75IRE: c_uint = 0x10;
pub const M1064_XGENCTRL_SYNC_ON_GREEN: c_uint = 0x00;
pub const M1064_XGENCTRL_NO_SYNC_ON_GREEN: c_uint = 0x20;
pub const M1064_XGENCTRL_SYNC_ON_GREEN_MASK: c_uint = 0x20;
pub const M1064_XMISCCTRL: c_uint = 0x1E;
pub const M1064_XMISCCTRL_DAC_DIS: c_uint = 0x00;
pub const M1064_XMISCCTRL_DAC_EN: c_uint = 0x01;
pub const M1064_XMISCCTRL_MFC_VGA: c_uint = 0x00;
pub const M1064_XMISCCTRL_MFC_MAFC: c_uint = 0x02;
pub const M1064_XMISCCTRL_MFC_DIS: c_uint = 0x06;
pub const GX00_XMISCCTRL_MFC_MAFC: c_uint = 0x02;
pub const GX00_XMISCCTRL_MFC_PANELLINK: c_uint = 0x04;
pub const GX00_XMISCCTRL_MFC_DIS: c_uint = 0x06;
pub const GX00_XMISCCTRL_MFC_MASK: c_uint = 0x06;
pub const M1064_XMISCCTRL_DAC_6BIT: c_uint = 0x00;
pub const M1064_XMISCCTRL_DAC_8BIT: c_uint = 0x08;
pub const M1064_XMISCCTRL_DAC_WIDTHMASK: c_uint = 0x08;
pub const M1064_XMISCCTRL_LUT_DIS: c_uint = 0x00;
pub const M1064_XMISCCTRL_LUT_EN: c_uint = 0x10;
pub const G400_XMISCCTRL_VDO_MAFC12: c_uint = 0x00;
pub const G400_XMISCCTRL_VDO_BYPASS656: c_uint = 0x40;
pub const G400_XMISCCTRL_VDO_C2_MAFC12: c_uint = 0x80;
pub const G400_XMISCCTRL_VDO_C2_BYPASS656: c_uint = 0xC0;
pub const G400_XMISCCTRL_VDO_MASK: c_uint = 0xE0;
pub const M1064_XGENIOCTRL: c_uint = 0x2A;
pub const M1064_XGENIODATA: c_uint = 0x2B;
pub const DAC1064_XSYSPLLM: c_uint = 0x2C;
pub const DAC1064_XSYSPLLN: c_uint = 0x2D;
pub const DAC1064_XSYSPLLP: c_uint = 0x2E;
pub const DAC1064_XSYSPLLSTAT: c_uint = 0x2F;
pub const M1064_XZOOMCTRL: c_uint = 0x38;
pub const M1064_XZOOMCTRL_1: c_uint = 0x00;
pub const M1064_XZOOMCTRL_2: c_uint = 0x01;
pub const M1064_XZOOMCTRL_4: c_uint = 0x03;
pub const M1064_XSENSETEST: c_uint = 0x3A;
pub const M1064_XSENSETEST_BCOMP: c_uint = 0x01;
pub const M1064_XSENSETEST_GCOMP: c_uint = 0x02;
pub const M1064_XSENSETEST_RCOMP: c_uint = 0x04;
pub const M1064_XSENSETEST_PDOWN: c_uint = 0x00;
pub const M1064_XSENSETEST_PUP: c_uint = 0x80;
pub const M1064_XCRCREML: c_uint = 0x3C;
pub const M1064_XCRCREMH: c_uint = 0x3D;
pub const M1064_XCRCBITSEL: c_uint = 0x3E;
pub const M1064_XCOLKEYMASKL: c_uint = 0x40;
pub const M1064_XCOLKEYMASKH: c_uint = 0x41;
pub const M1064_XCOLKEYL: c_uint = 0x42;
pub const M1064_XCOLKEYH: c_uint = 0x43;
pub const M1064_XPIXPLLAM: c_uint = 0x44;
pub const M1064_XPIXPLLAN: c_uint = 0x45;
pub const M1064_XPIXPLLAP: c_uint = 0x46;
pub const M1064_XPIXPLLBM: c_uint = 0x48;
pub const M1064_XPIXPLLBN: c_uint = 0x49;
pub const M1064_XPIXPLLBP: c_uint = 0x4A;
pub const M1064_XPIXPLLCM: c_uint = 0x4C;
pub const M1064_XPIXPLLCN: c_uint = 0x4D;
pub const M1064_XPIXPLLCP: c_uint = 0x4E;
pub const M1064_XPIXPLLSTAT: c_uint = 0x4F;
pub const M1064_XTVO_IDX: c_uint = 0x87;
pub const M1064_XTVO_DATA: c_uint = 0x88;
pub const M1064_XOUTPUTCONN: c_uint = 0x8A;
pub const M1064_XSYNCCTRL: c_uint = 0x8B;
pub const M1064_XVIDPLLSTAT: c_uint = 0x8C;
pub const M1064_XVIDPLLP: c_uint = 0x8D;
pub const M1064_XVIDPLLM: c_uint = 0x8E;
pub const M1064_XVIDPLLN: c_uint = 0x8F;
pub const M1064_XPWRCTRL: c_uint = 0xA0;
pub const M1064_XPWRCTRL_PANELPDN: c_uint = 0x04;
pub const M1064_XPANMODE: c_uint = 0xA2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum POS1064 {
    POS1064_XCURADDL=0, POS1064_XCURADDH, POS1064_XCURCTRL,
    POS1064_XCURCOL0RED, POS1064_XCURCOL0GREEN, POS1064_XCURCOL0BLUE,
    POS1064_XCURCOL1RED, POS1064_XCURCOL1GREEN, POS1064_XCURCOL1BLUE,
    POS1064_XCURCOL2RED, POS1064_XCURCOL2GREEN, POS1064_XCURCOL2BLUE,
    POS1064_XVREFCTRL, POS1064_XMULCTRL, POS1064_XPIXCLKCTRL, POS1064_XGENCTRL,
    POS1064_XMISCCTRL,
    POS1064_XGENIOCTRL, POS1064_XGENIODATA, POS1064_XZOOMCTRL, POS1064_XSENSETEST,
    POS1064_XCRCBITSEL,
    POS1064_XCOLKEYMASKL, POS1064_XCOLKEYMASKH, POS1064_XCOLKEYL, POS1064_XCOLKEYH,
    POS1064_XOUTPUTCONN, POS1064_XPANMODE, POS1064_XPWRCTRL };
