//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/video/fbdev/matrox/matroxfb_base.h
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
// Hardware accelerated Matrox Millennium I, II, Mystique, G100, G200, G400 and G450
//
// (c) 1998-2002 Petr Vandrovec <vandrove@vc.cvut.cz>
//
// general, but fairly heavy, debugging

// heavy debugging:
// -- logs putc[s], so every time a char is displayed, it's logged

// This one _could_ cause infinite loops
// It _does_ cause lots and lots of messages during idle loops

// Debug register calls, too?

// Guard accelerator accesses with spin_lock_irqsave...

// Macro flag: #define DEBUG

pub const PCI_SS_VENDOR_ID_SIEMENS_NIXDORF: c_uint = 0x110A;

pub const PCI_SS_ID_MATROX_GENERIC: c_uint = 0xFF00;
pub const PCI_SS_ID_MATROX_PRODUCTIVA_G100_AGP: c_uint = 0xFF01;
pub const PCI_SS_ID_MATROX_MYSTIQUE_G200_AGP: c_uint = 0xFF02;
pub const PCI_SS_ID_MATROX_MILLENIUM_G200_AGP: c_uint = 0xFF03;
pub const PCI_SS_ID_MATROX_MARVEL_G200_AGP: c_uint = 0xFF04;
pub const PCI_SS_ID_MATROX_MGA_G100_PCI: c_uint = 0xFF05;
pub const PCI_SS_ID_MATROX_MGA_G100_AGP: c_uint = 0x1001;
pub const PCI_SS_ID_MATROX_MILLENNIUM_G400_MAX_AGP: c_uint = 0x2179;
pub const PCI_SS_ID_SIEMENS_MGA_G100_AGP: c_uint = 0x001E /* 30 */;
pub const PCI_SS_ID_SIEMENS_MGA_G200_AGP: c_uint = 0x0032 /* 50 */;

// G-series and Mystique have (almost) same DAC

pub const NEED_DAC1064: c_int = 1;

extern "C" {
    pub fn readb(offs: va.vaddr +) -> return;
}
extern "C" {
    pub fn readl(offs: va.vaddr +) -> return;
}

//
// iowrite32_rep works for us if:
// (1) Copies data as 32bit quantities, not byte after byte,
// (2) Performs LE ordered stores, and
// (3) It copes with unaligned source (destination is guaranteed to be page
// aligned and length is guaranteed to be multiple of 4).
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_timming {
    pub pixclock: c_uint,
    pub mnp: c_int,
    pub crtc: c_uint,
    pub HDisplay: c_uint,
    pub HSyncStart: c_uint,
    pub HSyncEnd: c_uint,
    pub HTotal: c_uint,
    pub VDisplay: c_uint,
    pub VSyncStart: c_uint,
    pub VSyncEnd: c_uint,
    pub VTotal: c_uint,
    pub sync: c_uint,
    pub dblscan: c_int,
    pub interlaced: c_int,
    pub /: *mut *mut unsigned int delay; / CRTC delay,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct matrox_pll_cache {
    pub valid: c_uint,
    pub mnp_key: c_uint,
    pub mnp_value: c_uint,
    pub data: [}; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct matrox_pll_limits {
    pub vcomin: c_uint,
    pub vcomax: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct matrox_pll_features {
    pub vco_freq_min: c_uint,
    pub ref_freq: c_uint,
    pub feed_div_min: c_uint,
    pub feed_div_max: c_uint,
    pub in_div_min: c_uint,
    pub in_div_max: c_uint,
    pub post_shift_max: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct matrox_DAC1064_features {
    pub xvrefctrl: u_int8_t,
    pub xmiscctrl: u_int8_t,
}

// current hardware status
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mavenregs {
    pub regs: [u_int8_t; 256],
    pub mode: c_int,
    pub vlines: c_int,
    pub xtal: c_int,
    pub fv: c_int,
    pub htotal: u_int16_t,
    pub hcorr: u_int16_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct matrox_crtc2 {
    pub ctl: u_int32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct matrox_hw_state {
    pub MXoptionReg: u_int32_t,
    pub DACclk: [c_uchar; 6],
    pub DACreg: [c_uchar; 80],
    pub MiscOutReg: c_uchar,
    pub DACpal: [c_uchar; 768],
    pub CRTC: [c_uchar; 25],
    pub CRTCEXT: [c_uchar; 9],
    pub SEQ: [c_uchar; 5],
// unused for MGA mode, but who knows...
    pub GCTL: [c_uchar; 9],
// unused for MGA mode, but who knows...
    pub ATTR: [c_uchar; 21],
// TVOut only
    pub maven: mavenregs,
    pub crtc2: matrox_crtc2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct matrox_accel_data {

    pub ramdac_rev: c_uchar,

    pub m_dwg_rect: u_int32_t,
    pub m_opmode: u_int32_t,
    pub m_access: u_int32_t,
    pub m_pitch: u_int32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct matrox_altout {
    pub name: *const c_char,
    pub input): *mut *mut *mut *mut int (compute)(void altout_dev, struct my_timming,
    pub altout_dev): *mut *mut *mut int (program)(void,
    pub altout_dev): *mut *mut *mut int (start)(void,
    pub mode): *mut *mut *mut int (verifymode)(void altout_dev, u_int32_t,
    pub ctrl): *mut *mut v4l2_queryctrl,
    pub ctrl): *mut *mut v4l2_control,
    pub ctrl): *mut *mut v4l2_control,
}

pub const MATROXFB_SRC_NONE: c_int = 0;
pub const MATROXFB_SRC_CRTC1: c_int = 1;
pub const MATROXFB_SRC_CRTC2: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mga_chip {

    struct matrox_bios {
    unsigned int	bios_valid : 1;
    unsigned int	pins_len;
    unsigned char	pins[128];
    struct {
    unsigned char vMaj, vMin, vRev;
    } version;
    struct {
    unsigned char state, tvout;
    } output;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct matrox_vsync {
    pub wait: wait_queue_head_t,
    pub cnt: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct matrox_fb_info {
    pub fbcon: fb_info,
    pub next_fb: list_head,
    pub dead: c_int,
    pub initialized: c_int,
    pub usecount: c_uint,
    pub userusecount: c_uint,
    pub irq_flags: c_ulong,
    pub curr: matroxfb_par,
    pub hw: matrox_hw_state,
    pub accel: matrox_accel_data,
    pub pcidev: *mut *mut pci_dev,
    pub vsync: matrox_vsync,
    pub pixclock: c_uint,
    pub mnp: c_int,
    pub panpos: c_int,
    pub crtc1: },
    pub vsync: matrox_vsync,
    pub pixclock: c_uint,
    pub mnp: c_int,
    pub info: *mut *mut matroxfb_dh_fb_info,
    pub lock: rw_semaphore,
    pub crtc2: },
    pub lock: rw_semaphore,
    pub gamma: int brightness, contrast, saturation, hue,,
    pub deflicker: int testout,,
    pub tvo_params: },
    pub altout: },
pub const MATROXFB_MAX_OUTPUTS: c_int = 3;
    pub src: c_uint,
    pub output: *mut *mut matrox_altout,
    pub data: *mut *mut c_void,
    pub mode: c_uint,
    pub default_src: c_uint,
    pub outputs: [}; MATROXFB_MAX_OUTPUTS],
pub const MATROXFB_MAX_FB_DRIVERS: c_int = 5;
    pub (drivers[MATROXFB_MAX_FB_DRIVERS]): *mut *mut matroxfb_driver,
    pub (drivers_data[MATROXFB_MAX_FB_DRIVERS]): *mut *mut c_void,
    pub drivers_count: c_uint,
    pub /: *mut *mut unsigned long base; / physical,
    pub /: *mut *mut vaddr_t vbase; / CPU view,
    pub len: c_uint,
    pub len_usable: c_uint,
    pub len_maximum: c_uint,
    pub video: },
    pub /: *mut *mut unsigned long base; / physical,
    pub /: *mut *mut vaddr_t vbase; / CPU view,
    pub len: c_uint,
    pub mmio: },
    pub max_pixel_clock: c_uint,
    pub max_pixel_clock_panellink: c_uint,
    pub hw_switch: *mut *mut matrox_switch,
    pub pll: matrox_pll_features,
    pub DAC1064: matrox_DAC1064_features,
    pub features: },
    pub DAC: spinlock_t,
    pub accel: spinlock_t,
    pub lock: },
    pub chip: mga_chip,
    pub interleave: c_int,
    pub millenium: c_int,
    pub milleniumII: c_int,
    pub cfb4: c_int,
    pub vxres: *const *const c_int,
    pub cross4MB: c_int,
    pub text: c_int,
    pub plnwt: c_int,
    pub srcorg: c_int,
    pub capable: },
    pub wc_cookie: c_int,
    pub precise_width: c_int,
    pub mga_24bpp_fix: c_int,
    pub novga: c_int,
    pub nobios: c_int,
    pub nopciretry: c_int,
    pub noinit: c_int,
    pub sgram: c_int,
    pub support32MB: c_int,
    pub accelerator: c_int,
    pub text_type_aux: c_int,
    pub video64bits: c_int,
    pub crtc2: c_int,
    pub maven_capable: c_int,
    pub vgastep: c_uint,
    pub textmode: c_uint,
    pub textstep: c_uint,
    pub /: *mut *mut unsigned int textvram; / character cells,
    pub /: *mut *mut unsigned int ydstorg; / offset in bytes from video start to usable memory,
// 0 except for 6MB Millenium
    pub memtype: c_int,
    pub g450dac: c_int,
    pub dfp_type: c_int,
    pub /: *mut *mut int panellink; / G400 DFP possible (not G450/G550),
    pub dualhead: c_int,
    pub fbResource: c_uint,
    pub devflags: },
    pub fbops: fb_ops,
    pub bios: matrox_bios,
    pub pixel: matrox_pll_limits,
    pub system: matrox_pll_limits,
    pub video: matrox_pll_limits,
    pub limits: },
    pub pixel: matrox_pll_cache,
    pub system: matrox_pll_cache,
    pub video: matrox_pll_cache,
    pub cache: },
    pub video: c_uint,
    pub system: c_uint,
    pub pll: },
    pub opt: u_int32_t,
    pub opt2: u_int32_t,
    pub opt3: u_int32_t,
    pub mctlwtst: u_int32_t,
    pub mctlwtst_core: u_int32_t,
    pub memmisc: u_int32_t,
    pub memrdbk: u_int32_t,
    pub maccess: u_int32_t,
    pub reg: },
    pub memory: },
    pub values: },
    pub cmap: [u_int32_t; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct matrox_switch {
    pub minfo): *mut *mut int (preinit)(struct matrox_fb_info,
    pub minfo): *mut *mut void (reset)(struct matrox_fb_info,
    pub my_timming*): *mut *mut *mut int (init)(struct matrox_fb_info minfo, struct,
    pub minfo): *mut *mut void (restore)(struct matrox_fb_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct matroxfb_driver {
    pub node: list_head,
    pub name: *mut *mut c_char,
    pub info): *mut *mut *mut *mut void (probe)(struct matrox_fb_info,
    pub data): *mut *mut *mut *mut void (remove)(struct matrox_fb_info info, void,
}

extern "C" {
    pub fn matroxfb_register_driver(drv: *mut *mut matroxfb_driver) -> c_int;
}
extern "C" {
    pub fn matroxfb_unregister_driver(drv: *mut *mut matroxfb_driver);
}
pub const PCI_OPTION_REG: c_uint = 0x40;
pub const PCI_OPTION_ENABLE_ROM: c_uint = 0x40000000;
pub const PCI_MGA_INDEX: c_uint = 0x44;
pub const PCI_MGA_DATA: c_uint = 0x48;
pub const PCI_OPTION2_REG: c_uint = 0x50;
pub const PCI_OPTION3_REG: c_uint = 0x54;
pub const PCI_MEMMISC_REG: c_uint = 0x58;
pub const M_DWGCTL: c_uint = 0x1C00;
pub const M_MACCESS: c_uint = 0x1C04;
pub const M_CTLWTST: c_uint = 0x1C08;
pub const M_PLNWT: c_uint = 0x1C1C;
pub const M_BCOL: c_uint = 0x1C20;
pub const M_FCOL: c_uint = 0x1C24;
pub const M_SGN: c_uint = 0x1C58;
pub const M_LEN: c_uint = 0x1C5C;
pub const M_AR0: c_uint = 0x1C60;
pub const M_AR1: c_uint = 0x1C64;
pub const M_AR2: c_uint = 0x1C68;
pub const M_AR3: c_uint = 0x1C6C;
pub const M_AR4: c_uint = 0x1C70;
pub const M_AR5: c_uint = 0x1C74;
pub const M_AR6: c_uint = 0x1C78;
pub const M_CXBNDRY: c_uint = 0x1C80;
pub const M_FXBNDRY: c_uint = 0x1C84;
pub const M_YDSTLEN: c_uint = 0x1C88;
pub const M_PITCH: c_uint = 0x1C8C;
pub const M_YDST: c_uint = 0x1C90;
pub const M_YDSTORG: c_uint = 0x1C94;
pub const M_YTOP: c_uint = 0x1C98;
pub const M_YBOT: c_uint = 0x1C9C;
// mystique only
pub const M_CACHEFLUSH: c_uint = 0x1FFF;
pub const M_EXEC: c_uint = 0x0100;
pub const M_DWG_TRAP: c_uint = 0x04;
pub const M_DWG_BITBLT: c_uint = 0x08;
pub const M_DWG_ILOAD: c_uint = 0x09;
pub const M_DWG_LINEAR: c_uint = 0x0080;
pub const M_DWG_SOLID: c_uint = 0x0800;
pub const M_DWG_ARZERO: c_uint = 0x1000;
pub const M_DWG_SGNZERO: c_uint = 0x2000;
pub const M_DWG_SHIFTZERO: c_uint = 0x4000;
pub const M_DWG_REPLACE: c_uint = 0x000C0000;

pub const M_DWG_XOR: c_uint = 0x00060010;
pub const M_DWG_BFCOL: c_uint = 0x04000000;
pub const M_DWG_BMONOWF: c_uint = 0x08000000;
pub const M_DWG_TRANSC: c_uint = 0x40000000;
pub const M_FIFOSTATUS: c_uint = 0x1E10;
pub const M_STATUS: c_uint = 0x1E14;
pub const M_ICLEAR: c_uint = 0x1E18;
pub const M_IEN: c_uint = 0x1E1C;
pub const M_VCOUNT: c_uint = 0x1E20;
pub const M_RESET: c_uint = 0x1E40;
pub const M_MEMRDBK: c_uint = 0x1E44;
pub const M_AGP2PLL: c_uint = 0x1E4C;
pub const M_OPMODE: c_uint = 0x1E54;
pub const M_OPMODE_DMA_GEN_WRITE: c_uint = 0x00;
pub const M_OPMODE_DMA_BLIT: c_uint = 0x04;
pub const M_OPMODE_DMA_VECTOR_WRITE: c_uint = 0x08;
pub const M_OPMODE_DMA_LE: c_uint = 0x0000		/* little endian - no transformation */;
pub const M_OPMODE_DMA_BE_8BPP: c_uint = 0x0000;
pub const M_OPMODE_DMA_BE_16BPP: c_uint = 0x0100;
pub const M_OPMODE_DMA_BE_32BPP: c_uint = 0x0200;
pub const M_OPMODE_DIR_LE: c_uint = 0x000000	/* little endian - no transformation */;
pub const M_OPMODE_DIR_BE_8BPP: c_uint = 0x000000;
pub const M_OPMODE_DIR_BE_16BPP: c_uint = 0x010000;
pub const M_OPMODE_DIR_BE_32BPP: c_uint = 0x020000;
pub const M_ATTR_INDEX: c_uint = 0x1FC0;
pub const M_ATTR_DATA: c_uint = 0x1FC1;
pub const M_MISC_REG: c_uint = 0x1FC2;
pub const M_3C2_RD: c_uint = 0x1FC2;
pub const M_SEQ_INDEX: c_uint = 0x1FC4;
pub const M_SEQ_DATA: c_uint = 0x1FC5;
pub const M_SEQ1: c_uint = 0x01;
pub const M_SEQ1_SCROFF: c_uint = 0x20;
pub const M_MISC_REG_READ: c_uint = 0x1FCC;
pub const M_GRAPHICS_INDEX: c_uint = 0x1FCE;
pub const M_GRAPHICS_DATA: c_uint = 0x1FCF;
pub const M_CRTC_INDEX: c_uint = 0x1FD4;
pub const M_ATTR_RESET: c_uint = 0x1FDA;
pub const M_3DA_WR: c_uint = 0x1FDA;
pub const M_INSTS1: c_uint = 0x1FDA;
pub const M_EXTVGA_INDEX: c_uint = 0x1FDE;
pub const M_EXTVGA_DATA: c_uint = 0x1FDF;
// G200 only
pub const M_SRCORG: c_uint = 0x2CB4;
pub const M_DSTORG: c_uint = 0x2CB8;
pub const M_RAMDAC_BASE: c_uint = 0x3C00;
// fortunately, same on TVP3026 and MGA1064

pub const M_X_INDEX: c_uint = 0x00;
pub const M_X_DATAREG: c_uint = 0x0A;
pub const DAC_XGENIOCTRL: c_uint = 0x2A;
pub const DAC_XGENIODATA: c_uint = 0x2B;
pub const M_C2CTL: c_uint = 0x3C10;
pub const MX_OPTION_BSWAP: c_uint = 0x00000000;

// code speedup

extern "C" {
    pub fn matroxfb_DAC_in(minfo: *const matrox_fb_info, reg: c_int) -> c_int;
}
extern "C" {
    pub fn matroxfb_var2my(fvsi: *mut *mut fb_var_screeninfo, mt: *mut *mut my_timming);
}
extern "C" {
    pub fn matroxfb_wait_for_sync(minfo: *mut matrox_fb_info, crtc: u_int32_t) -> c_int;
}
extern "C" {
    pub fn matroxfb_enable_irq(minfo: *mut matrox_fb_info, reenable: c_int) -> c_int;
}

// Macro flag: #define CRITBEGIN
// Macro flag: #define CRITEND
// Macro flag: #define CRITFLAGS

