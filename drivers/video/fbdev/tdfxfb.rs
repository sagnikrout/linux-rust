//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/tdfxfb.c
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
//
// tdfxfb.c
//
// Author: Hannu Mallat <hmallat@cc.hut.fi>
//
// Copyright © 1999 Hannu Mallat
// All rights reserved
//
// Created      : Thu Sep 23 18:17:43 1999, hmallat
// Last modified: Tue Nov  2 21:19:47 1999, hmallat
//
// I2C part copied from the i2c-voodoo3.c driver by:
// Frodo Looijaard <frodol@dds.nl>,
// Philip Edelbrock <phil@netroedge.com>,
// Ralph Metzler <rjkm@thp.uni-koeln.de>, and
// Mark D. Studebaker <mdsxyz123@yahoo.com>
//
// Lots of the information here comes from the Daryll Strauss' Banshee
// patches to the XF86 server, and the rest comes from the 3dfx
// Banshee specification. I'm very much indebted to Daryll for his
// work on the X server.
//
// Voodoo3 support was contributed Harold Oga. Lots of additions
// (proper acceleration, 24 bpp, hardware cursor) and bug fixes by Attila
// Kesmarki. Thanks guys!
//
// Voodoo1 and Voodoo2 support aren't relevant to this driver as they
// behave very differently from the Voodoo3/4/5. For anyone wanting to
// use frame buffer on the Voodoo1/2, see the sstfb driver (which is
// located at http://www.sourceforge.net/projects/sstfb).
//
// While I _am_ grateful to 3Dfx for releasing the specs for Banshee,
// I do wish the next version is a bit more complete. Without the XF86
// patches I couldn't have gotten even this far... for instance, the
// extensions to the VGA register set go completely unmentioned in the
// spec! Also, lots of references are made to the 'SST core', but no
// spec is publicly available, AFAIK.
//
// The structure of this driver comes pretty much from the Permedia
// driver by Ilario Nardinocchi, which in turn is based on skeletonfb.
//
// TODO:
// - multihead support (basically need to support an array of fb_infos)
// - support other architectures (PPC, Alpha); does the fact that the VGA
// core can be accessed only thru I/O (not memory mapped) complicate
// things?
//
// Version history:
//
// 0.1.4 (released 2002-05-28)	ported over to new fbdev api by James Simmons
//
// 0.1.3 (released 1999-11-02)	added Attila's panning support, code
// reorg, hwcursor address page size alignment
// (for mmapping both frame buffer and regs),
// and my changes to get rid of hardcoded
// VGA i/o register locations (uses PCI
// configuration info now)
// 0.1.2 (released 1999-10-19)	added Attila Kesmarki's bug fixes and
// improvements
// 0.1.1 (released 1999-10-07)	added Voodoo3 support by Harold Oga.
// 0.1.0 (released 1999-10-06)	initial version
//

pub const BANSHEE_MAX_PIXCLOCK: c_int = 270000;
pub const VOODOO3_MAX_PIXCLOCK: c_int = 300000;
pub const VOODOO5_MAX_PIXCLOCK: c_int = 350000;
    static const struct fb_fix_screeninfo tdfx_fix = {
    .type =		FB_TYPE_PACKED_PIXELS,
    .visual =	FB_VISUAL_PSEUDOCOLOR,
    .ypanstep =	1,
    .ywrapstep =	1,
    .accel =	FB_ACCEL_3DFX_BANSHEE
    };
    static const struct fb_var_screeninfo tdfx_var = {
// "640x480, 8 bpp @ 60 Hz
    .xres =		640,
    .yres =		480,
    .xres_virtual =	640,
    .yres_virtual =	1024,
    .bits_per_pixel = 8,
    .red =		{0, 8, 0},
    .blue =		{0, 8, 0},
    .green =	{0, 8, 0},
    .activate =	FB_ACTIVATE_NOW,
    .height =	-1,
    .width =	-1,
    .accel_flags =	FB_ACCELF_TEXT,
    .pixclock =	39722,
    .left_margin =	40,
    .right_margin =	24,
    .upper_margin =	32,
    .lower_margin =	11,
    .hsync_len =	96,
    .vsync_len =	2,
    .vmode =	FB_VMODE_NONINTERLACED
    };
//
// PCI driver prototypes
//
    static int tdfxfb_probe(struct pci_dev *pdev, const struct pci_device_id *id);
    static void tdfxfb_remove(struct pci_dev *pdev);
    static const struct pci_device_id tdfxfb_id_table[] = {
    {
    PCI_DEVICE(PCI_VENDOR_ID_3DFX, PCI_DEVICE_ID_3DFX_BANSHEE),
    .class = PCI_BASE_CLASS_DISPLAY << 16, .class_mask = 0xff0000,
    }, {
    PCI_DEVICE(PCI_VENDOR_ID_3DFX, PCI_DEVICE_ID_3DFX_VOODOO3),
    .class = PCI_BASE_CLASS_DISPLAY << 16, .class_mask = 0xff0000,
    }, {
    PCI_DEVICE(PCI_VENDOR_ID_3DFX, PCI_DEVICE_ID_3DFX_VOODOO5),
    .class = PCI_BASE_CLASS_DISPLAY << 16, .class_mask = 0xff0000,
    },
    { }
    };
    static struct pci_driver tdfxfb_driver = {
    .name		= "tdfxfb",
    .id_table	= tdfxfb_id_table,
    .probe		= tdfxfb_probe,
    .remove		= tdfxfb_remove,
    };
    MODULE_DEVICE_TABLE(pci, tdfxfb_id_table);
//
// Driver data
//
    static int nopan;
    static int nowrap = 1;      /* not implemented (yet) */
    let mut hwcursor: static int = 1;
    static char *mode_option;
    static bool nomtrr;
// -------------------------------------------------------------------------
// Hardware-specific funcions
// -------------------------------------------------------------------------
#[no_mangle]
pub unsafe extern "C" fn vga_inb(par: *mut tdfx_par, reg: u32) -> u8 {
    static inline u8 vga_inb(struct tdfx_par *par, u32 reg)
    {
    return inb(par.iobase + reg - 0x300);
    }
#[no_mangle]
pub unsafe extern "C" fn vga_outb(par: *mut tdfx_par, reg: u32, val: u8) {
    static inline void vga_outb(struct tdfx_par *par, u32 reg, u8 val)
    {
    outb(val, par.iobase + reg - 0x300);
    }
#[no_mangle]
pub unsafe extern "C" fn gra_outb(par: *mut tdfx_par, idx: u32, val: u8) {
    static inline void gra_outb(struct tdfx_par *par, u32 idx, u8 val)
    {
    vga_outb(par, GRA_I, idx);
    wmb();
    vga_outb(par, GRA_D, val);
    wmb();
    }
#[no_mangle]
pub unsafe extern "C" fn seq_outb(par: *mut tdfx_par, idx: u32, val: u8) {
    static inline void seq_outb(struct tdfx_par *par, u32 idx, u8 val)
    {
    vga_outb(par, SEQ_I, idx);
    wmb();
    vga_outb(par, SEQ_D, val);
    wmb();
    }
#[no_mangle]
pub unsafe extern "C" fn seq_inb(par: *mut tdfx_par, idx: u32) -> u8 {
    static inline u8 seq_inb(struct tdfx_par *par, u32 idx)
    {
    vga_outb(par, SEQ_I, idx);
    mb();
    return vga_inb(par, SEQ_D);
    }
#[no_mangle]
pub unsafe extern "C" fn crt_outb(par: *mut tdfx_par, idx: u32, val: u8) {
    static inline void crt_outb(struct tdfx_par *par, u32 idx, u8 val)
    {
    vga_outb(par, CRT_I, idx);
    wmb();
    vga_outb(par, CRT_D, val);
    wmb();
    }
#[no_mangle]
pub unsafe extern "C" fn crt_inb(par: *mut tdfx_par, idx: u32) -> u8 {
    static inline u8 crt_inb(struct tdfx_par *par, u32 idx)
    {
    vga_outb(par, CRT_I, idx);
    mb();
    return vga_inb(par, CRT_D);
    }
#[no_mangle]
pub unsafe extern "C" fn att_outb(par: *mut tdfx_par, idx: u32, val: u8) {
    static inline void att_outb(struct tdfx_par *par, u32 idx, u8 val)
    {
    vga_inb(par, IS1_R);
    vga_outb(par, ATT_IW, idx);
    vga_outb(par, ATT_IW, val);
    }
#[no_mangle]
pub unsafe extern "C" fn vga_disable_video(par: *mut tdfx_par) {
    static inline void vga_disable_video(struct tdfx_par *par)
    {
    unsigned char s;
    s = seq_inb(par, 0x01) | 0x20;
    seq_outb(par, 0x00, 0x01);
    seq_outb(par, 0x01, s);
    seq_outb(par, 0x00, 0x03);
    }
#[no_mangle]
pub unsafe extern "C" fn vga_enable_video(par: *mut tdfx_par) {
    static inline void vga_enable_video(struct tdfx_par *par)
    {
    unsigned char s;
    s = seq_inb(par, 0x01) & 0xdf;
    seq_outb(par, 0x00, 0x01);
    seq_outb(par, 0x01, s);
    seq_outb(par, 0x00, 0x03);
    }
#[no_mangle]
pub unsafe extern "C" fn vga_enable_palette(par: *mut tdfx_par) {
    static inline void vga_enable_palette(struct tdfx_par *par)
    {
    vga_inb(par, IS1_R);
    mb();
    vga_outb(par, ATT_IW, 0x20);
    }
#[no_mangle]
pub unsafe extern "C" fn tdfx_inl(par: *mut tdfx_par, reg: c_uint) -> u32 {
    static inline u32 tdfx_inl(struct tdfx_par *par, unsigned int reg)
    {
    return readl(par.regbase_virt + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn tdfx_outl(par: *mut tdfx_par, reg: c_uint, val: u32) {
    static inline void tdfx_outl(struct tdfx_par *par, unsigned int reg, u32 val)
    {
    writel(val, par.regbase_virt + reg);
    }
#[no_mangle]
pub unsafe extern "C" fn banshee_make_room(par: *mut tdfx_par, size: c_int) {
    static inline void banshee_make_room(struct tdfx_par *par, int size)
    {
// Note: The Voodoo3's onboard FIFO has 32 slots. This loop
// won't quit if you ask for more.
    while ((tdfx_inl(par, STATUS) & 0x1f) < size - 1)
    cpu_relax();
    }
#[no_mangle]
unsafe extern "C" fn banshee_wait_idle(info: *mut fb_info) -> c_int {
    static int banshee_wait_idle(struct fb_info *info)
    {
    struct tdfx_par *par = info.par;
    let mut i: c_int = 0;
    banshee_make_room(par, 1);
    tdfx_outl(par, COMMAND_3D, COMMAND_3D_NOP);
    do {
    if ((tdfx_inl(par, STATUS) & STATUS_BUSY) == 0)
    i++;
    } while (i < 3);
    return 0;
    }
//
// Set the color of a palette entry in 8bpp mode
//
#[no_mangle]
pub unsafe extern "C" fn do_setpalentry(par: *mut tdfx_par, regno: unsigned, c: u32) {
    static inline void do_setpalentry(struct tdfx_par *par, unsigned regno, u32 c)
    {
    banshee_make_room(par, 2);
    tdfx_outl(par, DACADDR, regno);
// read after write makes it working
    tdfx_inl(par, DACADDR);
    tdfx_outl(par, DACDATA, c);
    }
#[no_mangle]
unsafe extern "C" fn do_calc_pll(freq: c_int, freq_out: *mut c_int) -> u32 {
    static u32 do_calc_pll(int freq, int *freq_out)
    {
    int m, n, k, best_m, best_n, best_k, best_error;
    let mut fref: c_int = 14318;
    best_error = freq;
    best_n = best_m = best_k = 0;
    for (k = 3; k >= 0; k--) {
    for (m = 63; m >= 0; m--) {
//
// Estimate value of n that produces target frequency
// with current m and k
//
    let mut n_estimated: c_int = ((freq * (m + 2) << k) / fref) - 2;
// Search neighborhood of estimated n
    for (n = max(0, n_estimated);
    n <= min(255, n_estimated + 1);
    n++) {
//
// Calculate PLL freqency with current m, k and
// estimated n
//
    let mut f: c_int = (fref * (n + 2) / (m + 2)) >> k;
    let mut error: c_int = abs(f - freq);
//
// If this is the closest we've come to the
// target frequency then remember n, m and k
//
    if (error < best_error) {
    best_error = error;
    best_n = n;
    best_m = m;
    best_k = k;
    }
    }
    }
    }
    n = best_n;
    m = best_m;
    k = best_k;
// freq_out = (fref * (n + 2) / (m + 2)) >> k;
    return (n << 8) | (m << 2) | k;
    }
//
// Convert a pllctrl register value back to a frequency in kHz.
// Formula from 3dfx documentation.
//
#[no_mangle]
unsafe extern "C" fn tdfx_pll_to_khz(pll: u32) -> u32 {
    static u32 tdfx_pll_to_khz(u32 pll)
    {
    return (14318 * (((pll >> 8) & 0xff) + 2) /
    (((pll >> 2) & 0x3f) + 2)) >> (pll & 3);
    }
// Layout of the "OEM config" table in voodoo 3 BIOS
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tdfx_bios_cfg {
    pub /: *mut *mut __le32 pciinit0; / 0x00,
    pub /: *mut *mut __le32 miscinit0; / 0x04,
    pub /: *mut *mut __le32 miscinit1; / 0x08,
    pub /: *mut *mut __le32 draminit0; / 0x0c,
    pub /: *mut *mut __le32 draminit1; / 0x10,
    pub /: *mut *mut __le32 agpinit0; / 0x14,
    pub /: *mut *mut __le32 pllctrl1; / 0x18 - memory PLL,
    pub /: *mut *mut __le32 pllctrl2; / 0x1c - graphics PLL,
    pub /: *mut *mut __le32 sgrammode; / 0x20 - SGRAM/SDRAM mode register data,
    pub __packed: },
pub const TDFX_ROM_CFG_PTR: c_uint = 0x50;
    static bool tdfxfb_get_bios_cfg(struct pci_dev *pdev,
    struct tdfx_bios_cfg *cfg)
    {
    pub oemcfg: u16 romcfg,,
    pub rom: *mut void __iomem,
    pub romsize: usize,
    pub image: *mut u8,
    pub khz: u32,
// This only works for the Voodoo 3 for now
    if (pdev.device != PCI_DEVICE_ID_3DFX_VOODOO3)
    pub false: return,
    pub &romsize): rom = pci_map_rom(pdev,,
    if (!rom || !romsize)
    pub false: return,
    pub vmalloc(romsize): image =,
    if (!image) {
    pub rom): pci_unmap_rom(pdev,,
    pub false: return,
    }
    pub romsize): memcpy_fromio(image, rom,,
    pub rom): pci_unmap_rom(pdev,,
// ROM[0x50] -> ROM config table -> OEM config table
    if (TDFX_ROM_CFG_PTR + 2 > romsize)
    pub out: goto,
    pub 8: romcfg = image[TDFX_ROM_CFG_PTR] | image[TDFX_ROM_CFG_PTR + 1] <<,
    if (romcfg == 0xffff || romcfg + 2 > romsize)
    pub out: goto,
    pub 8: oemcfg = image[romcfg] | image[romcfg + 1] <<,
    if (oemcfg == 0xffff || oemcfg + sizeof(*cfg) > romsize)
    pub out: goto,
    pub sizeof(*cfg)): *mut memcpy(cfg, image + oemcfg,,
//
// Make sure we didn't read garbage from the BIOS and will
// end up setting a frequency that explodes someone's expensive
// card.
//
    pub tdfx_pll_to_khz(le32_to_cpu(cfg->pllctrl1)): khz =,
    if (khz < 40000 || khz > 250000 || !le32_to_cpu(cfg.draminit0))
    pub false: return,
    pub true: return,
    out:
    pub false: return,
    }
//
// Try to work out if the card was booted or not, just checks if
// one of the dram config registers matches what is in the config
// table if there is one.
//
// If we have a BIOS config table attempt to manually boot the
// card if needed.
//
#[no_mangle]
unsafe extern "C" fn tdfxfb_hw_init(info: *mut fb_info, pdev: *mut pci_dev) -> c_int {
    static int tdfxfb_hw_init(struct fb_info *info, struct pci_dev *pdev)
    {
    pub dram_mode: u32 mempll, gfxpll, draminit0, draminit1, miscinit1,,
    pub info->par: *mut *mut tdfx_par par =,
    pub cfg: tdfx_bios_cfg,
    pub &cfg): bool have_cfg = tdfxfb_get_bios_cfg(pdev,,
//
// Can't tell if the card is booted or not,
// also cannot boot it. Card might not function.
//
    if (!have_cfg)
    pub 0: return,
// Card is, probably, already configured.
    if (tdfx_inl(par, DRAMINIT0) == le32_to_cpu(cfg.draminit0))
    pub 0: return,
    dev_info(&pdev.dev,
    pub table\n"): "Manually booting card using config,
    pub le32_to_cpu(cfg.pllctrl1): mempll =,
    pub le32_to_cpu(cfg.pllctrl2): gfxpll =,
    pub le32_to_cpu(cfg.draminit0): draminit0 =,
    pub le32_to_cpu(cfg.draminit1): draminit1 =,
    pub le32_to_cpu(cfg.miscinit1): miscinit1 =,
    pub le32_to_cpu(cfg.sgrammode): dram_mode =,
    pub le32_to_cpu(cfg.pciinit0)): tdfx_outl(par, PCIINIT0,,
    pub le32_to_cpu(cfg.agpinit0)): tdfx_outl(par, AGPINIT,,
// memory clock, and the graphics clock if the card wants one
    pub mempll): tdfx_outl(par, PLLCTRL1,,
    if (gfxpll)
    pub gfxpll): tdfx_outl(par, PLLCTRL2,,
// flush posted writes
    pub PLLCTRL1): tdfx_inl(par,,
// PLL lock
    pub miscinit1): tdfx_outl(par, MISCINIT1,,
    pub draminit0): tdfx_outl(par, DRAMINIT0,,
    pub draminit1): tdfx_outl(par, DRAMINIT1,,
// SDRAM/SGRAM wake up: load the mode register
    pub dram_mode): tdfx_outl(par, DRAMDATA,,
    pub 0x10d): tdfx_outl(par, DRAMCOMMAND,,
    pub 0x00001fff): tdfx_outl(par, LFBMEMORYCONFIG,,
    pub le32_to_cpu(cfg.miscinit0)): tdfx_outl(par, MISCINIT0,,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn do_write_regs(info: *mut fb_info, reg: *mut banshee_reg) {
    static void do_write_regs(struct fb_info *info, struct banshee_reg *reg)
    {
    pub info->par: *mut *mut tdfx_par par =,
    pub i: c_int,
    pub 0x01): tdfx_outl(par, MISCINIT1, tdfx_inl(par, MISCINIT1) |,
// Wake the VGA core if it hasn't already been woken up
    pub reg->vgainit0): tdfx_outl(par, VGAINIT0,,
    pub 0x01): vga_outb(par, 0x3c3,,
    pub /: *mut *mut crt_outb(par, 0x11, crt_inb(par, 0x11) & 0x7f); / CRT unprotect,
    pub 3): banshee_make_room(par,,
    pub 0x001FFFFF): tdfx_outl(par, VGAINIT1, reg->vgainit1 &,
    pub ~0x00000001): tdfx_outl(par, VIDPROCCFG, reg->vidcfg &,

    pub reg->mempll): tdfx_outl(par, PLLCTRL1,,
    pub reg->gfxpll): tdfx_outl(par, PLLCTRL2,,

    pub reg->vidpll): tdfx_outl(par, PLLCTRL0,,
    pub 0x01): vga_outb(par, MISC_W, reg->misc[0x00] |,
    pub i++): for (i = 0; i < 5;,
    pub reg->seq[i]): seq_outb(par, i,,
    pub i++): for (i = 0; i < 25;,
    pub reg->crt[i]): crt_outb(par, i,,
    pub i++): for (i = 0; i < 9;,
    pub reg->gra[i]): gra_outb(par, i,,
    pub i++): for (i = 0; i < 21;,
    pub reg->att[i]): att_outb(par, i,,
    pub reg->ext[0]): crt_outb(par, 0x1a,,
    pub reg->ext[1]): crt_outb(par, 0x1b,,
    pub 9): banshee_make_room(par,,
    pub reg->vgainit0): tdfx_outl(par, VGAINIT0,,
    pub reg->dacmode): tdfx_outl(par, DACMODE,,
    pub reg->stride): tdfx_outl(par, VIDDESKSTRIDE,,
    pub reg->curspataddr): tdfx_outl(par, HWCURPATADDR,,
    pub reg->screensize): tdfx_outl(par, VIDSCREENSIZE,,
    pub reg->startaddr): tdfx_outl(par, VIDDESKSTART,,
    pub reg->vidcfg): tdfx_outl(par, VIDPROCCFG,,
    pub reg->vgainit1): tdfx_outl(par, VGAINIT1,,
    pub reg->miscinit0): tdfx_outl(par, MISCINIT0,,
    pub 8): banshee_make_room(par,,
    pub reg->startaddr): tdfx_outl(par, SRCBASE,,
    pub reg->startaddr): tdfx_outl(par, DSTBASE,,
    pub 0): tdfx_outl(par, COMMANDEXTRA_2D,,
    pub 0): tdfx_outl(par, CLIP0MIN,,
    pub 0x0fff0fff): tdfx_outl(par, CLIP0MAX,,
    pub 0): tdfx_outl(par, CLIP1MIN,,
    pub 0x0fff0fff): tdfx_outl(par, CLIP1MAX,,
    pub 0): tdfx_outl(par, SRCXY,,
    }
#[no_mangle]
unsafe extern "C" fn do_lfb_size(par: *mut tdfx_par, dev_id: c_ushort) -> c_ulong {
    static unsigned long do_lfb_size(struct tdfx_par *par, unsigned short dev_id)
    {
    pub DRAMINIT0): u32 draminit0 = tdfx_inl(par,,
    pub DRAMINIT1): u32 draminit1 = tdfx_inl(par,,
    pub miscinit1: u32,
    pub 4: int num_chips = (draminit0 & DRAMINIT0_SGRAM_NUM) ? 8 :,
    pub /: *mut *mut int chip_size; / in MB,
    pub DRAMINIT1_MEM_SDRAM: int has_sgram = draminit1 &,
    if (dev_id < PCI_DEVICE_ID_3DFX_VOODOO5) {
// Banshee/Voodoo3
    pub 2: chip_size =,
    if (has_sgram && !(draminit0 & DRAMINIT0_SGRAM_TYPE))
    pub 1: chip_size =,
    } else {
// Voodoo4/5
    pub 0: has_sgram =,
    pub DRAMINIT0_SGRAM_TYPE_MASK: chip_size = draminit0 &,
    pub DRAMINIT0_SGRAM_TYPE_SHIFT): chip_size = 1 << (chip_size >>,
    }
// disable block writes for SDRAM
    pub MISCINIT1): miscinit1 = tdfx_inl(par,,
    pub MISCINIT1_2DBLOCK_DIS: miscinit1 |= has_sgram ? 0 :,
    pub MISCINIT1_CLUT_INV: miscinit1 |=,
    pub 1): banshee_make_room(par,,
    pub miscinit1): tdfx_outl(par, MISCINIT1,,
    pub 1024: *mut *mut *mut *mut return num_chips  chip_size  1024l,
    }
// -------------------------------------------------------------------------
#[no_mangle]
unsafe extern "C" fn tdfxfb_check_var(var: *mut fb_var_screeninfo, info: *mut fb_info) -> c_int {
    static int tdfxfb_check_var(struct fb_var_screeninfo *var, struct fb_info *info)
    {
    pub info->par: *mut *mut tdfx_par par =,
    pub lpitch: u32,
    if (var.bits_per_pixel != 8  && var.bits_per_pixel != 16 &&
    var.bits_per_pixel != 24 && var.bits_per_pixel != 32) {
    pub var->bits_per_pixel): DPRINTK("depth not supported: %u\n",,
    pub -EINVAL: return,
    }
    if (var.xres != var.xres_virtual)
    pub var->xres: var->xres_virtual =,
    if (var.yres > var.yres_virtual)
    pub var->yres: var->yres_virtual =,
    if (var.xoffset) {
    pub supported\n"): DPRINTK("xoffset not,
    pub -EINVAL: return,
    }
    pub 0: var->yoffset =,
//
// Banshee doesn't support interlace, but Voodoo4/5 and probably
// Voodoo3 do.
// no direct information about device id now?
// use max_pixclock for this...
//
    if (((var.vmode & FB_VMODE_MASK) == FB_VMODE_INTERLACED) &&
    (par.max_pixclock < VOODOO3_MAX_PIXCLOCK)) {
    pub supported\n"): DPRINTK("interlace not,
    pub -EINVAL: return,
    }
    if (info.monspecs.hfmax && info.monspecs.vfmax &&
    info.monspecs.dclkmax && fb_validate_mode(var, info) < 0) {
    pub specs\n"): DPRINTK("mode outside monitor's,
    pub -EINVAL: return,
    }
    pub /: *mut *mut var->xres = (var->xres + 15) & ~15; / could sometimes be 8,
    pub 3): *mut *mut lpitch = var->xres  ((var->bits_per_pixel + 7) >>,
    if (var.xres < 320 || var.xres > 2048) {
    pub var->xres): DPRINTK("width not supported: %u\n",,
    pub -EINVAL: return,
    }
    if (var.yres < 200 || var.yres > 2048) {
    pub var->yres): DPRINTK("height not supported: %u\n",,
    pub -EINVAL: return,
    }
    if (lpitch * var.yres_virtual > info.fix.smem_len) {
    pub lpitch: var->yres_virtual = info->fix.smem_len /,
    if (var.yres_virtual < var.yres) {
    DPRINTK("no memory for screen (%ux%ux%u)\n",
    var.xres, var.yres_virtual,
    pub -EINVAL: return,
    }
    }
    if (!var.pixclock)
    pub -EINVAL: return,
    if (PICOS2KHZ(var.pixclock) > par.max_pixclock) {
    DPRINTK("pixclock too high (%ldKHz)\n",
    pub -EINVAL: return,
    }
    pub 0: var->transp.offset =,
    pub 0: var->transp.length =,
    switch (var.bits_per_pixel) {
    case 8:
    pub 8: var->red.length =,
    pub 0: var->red.offset =,
    pub var->red: var->green =,
    pub var->red: var->blue =,
    case 16:
    pub 11: var->red.offset =,
    pub 5: var->red.length =,
    pub 5: var->green.offset =,
    pub 6: var->green.length =,
    pub 0: var->blue.offset =,
    pub 5: var->blue.length =,
    case 32:
    pub 24: var->transp.offset =,
    pub 8: var->transp.length =,
    case 24:
    pub 16: var->red.offset =,
    pub 8: var->green.offset =,
    pub 0: var->blue.offset =,
    pub 8: var->red.length = var->green.length = var->blue.length =,
    }
    pub -1: var->width =,
    pub -1: var->height =,
    pub FB_ACCELF_TEXT: var->accel_flags =,
    DPRINTK("Checking graphics mode at %dx%d depth %d\n",
    pub var->bits_per_pixel): var->xres, var->yres,,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn tdfxfb_set_par(info: *mut fb_info) -> c_int {
    static int tdfxfb_set_par(struct fb_info *info)
    {
    pub info->par: *mut *mut tdfx_par par =,
    pub info->var.xres: u32 hdispend =,
    pub info->var.right_margin: u32 hsyncsta = hdispend +,
    pub info->var.hsync_len: u32 hsyncend = hsyncsta +,
    pub info->var.left_margin: u32 htotal = hsyncend +,
    pub hbe: u32 hd, hs, he, ht, hbs,,
    pub vbe: u32 vd, vs, ve, vt, vbs,,
    pub reg: banshee_reg,
    pub freq: int fout,,
    pub wd: u32,
    pub 3: u32 cpp = (info->var.bits_per_pixel + 7) >>,
    pub sizeof(reg)): memset(&reg, 0,,
    reg.vidcfg = VIDCFG_VIDPROC_ENABLE | VIDCFG_DESK_ENABLE |
    VIDCFG_CURS_X11 |
    ((cpp - 1) << VIDCFG_PIXFMT_SHIFT) |
    pub 0): (cpp != 1 ? VIDCFG_CLUT_BYPASS :,
// PLL settings
    pub PICOS2KHZ(info->var.pixclock): freq =,
    pub ~VIDCFG_2X: reg.vidcfg &=,
    if (freq > par.max_pixclock / 2) {
    pub freq: freq = freq > par->max_pixclock ? par->max_pixclock :,
    pub DACMODE_2X: reg.dacmode |=,
    pub VIDCFG_2X: reg.vidcfg |=,
    pub 1: hdispend >>=,
    pub 1: hsyncsta >>=,
    pub 1: hsyncend >>=,
    pub 1: htotal >>=,
    }
    pub 1: wd = (hdispend >> 3) -,
    pub wd: hd =,
    pub 1: hs = (hsyncsta >> 3) -,
    pub 1: he = (hsyncend >> 3) -,
    pub 1: ht = (htotal >> 3) -,
    pub hd: hbs =,
    pub ht: hbe =,
    if ((info.var.vmode & FB_VMODE_MASK) == FB_VMODE_DOUBLE) {
    pub 1: vd = (info->var.yres << 1) -,
    pub 1): vs = vd + (info->var.lower_margin <<,
    pub 1): ve = vs + (info->var.vsync_len <<,
    pub 1: vt = ve + (info->var.upper_margin << 1) -,
    pub 13): reg.screensize = info->var.xres | (info->var.yres <<,
    pub VIDCFG_HALF_MODE: reg.vidcfg |=,
    pub 0x80: reg.crt[VGA_CRTC_MAX_SCAN] =,
    } else {
    pub 1: vd = info->var.yres -,
    pub info->var.lower_margin: vs = vd +,
    pub info->var.vsync_len: ve = vs +,
    pub 1: vt = ve + info->var.upper_margin -,
    pub 12): reg.screensize = info->var.xres | (info->var.yres <<,
    pub ~VIDCFG_HALF_MODE: reg.vidcfg &=,
    }
    pub vd: vbs =,
    pub vt: vbe =,
// this is all pretty standard VGA register stuffing
    reg.misc[0x00] = 0x0f |
    (info.var.xres < 400 ? 0xa0 :
    info.var.xres < 480 ? 0x60 :
    pub 0x20): info->var.xres < 768 ? 0xe0 :,
    pub 0x40: reg.gra[VGA_GFX_MODE] =,
    pub 0x05: reg.gra[VGA_GFX_MISC] =,
    pub 0x0f: reg.gra[VGA_GFX_COMPARE_MASK] =,
    pub 0xff: reg.gra[VGA_GFX_BIT_MASK] =,
    pub 0x00: reg.att[VGA_ATC_PALETTE0] =,
    pub 0x01: reg.att[VGA_ATC_PALETTE1] =,
    pub 0x02: reg.att[VGA_ATC_PALETTE2] =,
    pub 0x03: reg.att[VGA_ATC_PALETTE3] =,
    pub 0x04: reg.att[VGA_ATC_PALETTE4] =,
    pub 0x05: reg.att[VGA_ATC_PALETTE5] =,
    pub 0x06: reg.att[VGA_ATC_PALETTE6] =,
    pub 0x07: reg.att[VGA_ATC_PALETTE7] =,
    pub 0x08: reg.att[VGA_ATC_PALETTE8] =,
    pub 0x09: reg.att[VGA_ATC_PALETTE9] =,
    pub 0x0a: reg.att[VGA_ATC_PALETTEA] =,
    pub 0x0b: reg.att[VGA_ATC_PALETTEB] =,
    pub 0x0c: reg.att[VGA_ATC_PALETTEC] =,
    pub 0x0d: reg.att[VGA_ATC_PALETTED] =,
    pub 0x0e: reg.att[VGA_ATC_PALETTEE] =,
    pub 0x0f: reg.att[VGA_ATC_PALETTEF] =,
    pub 0x41: reg.att[VGA_ATC_MODE] =,
    pub 0x0f: reg.att[VGA_ATC_PLANE_ENABLE] =,
    pub 0x03: reg.seq[VGA_SEQ_RESET] =,
    pub /: *mut *mut reg.seq[VGA_SEQ_CLOCK_MODE] = 0x01; / fixme: clkdiv2?,
    pub 0x0f: reg.seq[VGA_SEQ_PLANE_WRITE] =,
    pub 0x00: reg.seq[VGA_SEQ_CHARACTER_MAP] =,
    pub 0x0e: reg.seq[VGA_SEQ_MEMORY_MODE] =,
    pub 4: reg.crt[VGA_CRTC_H_TOTAL] = ht -,
    pub hd: reg.crt[VGA_CRTC_H_DISP] =,
    pub hbs: reg.crt[VGA_CRTC_H_BLANK_START] =,
    pub 0x1f): reg.crt[VGA_CRTC_H_BLANK_END] = 0x80 | (hbe &,
    pub hs: reg.crt[VGA_CRTC_H_SYNC_START] =,
    pub 0x1f): reg.crt[VGA_CRTC_H_SYNC_END] = ((hbe & 0x20) << 2) | (he &,
    pub vt: reg.crt[VGA_CRTC_V_TOTAL] =,
    reg.crt[VGA_CRTC_OVERFLOW]      = ((vs & 0x200) >> 2) |
    ((vd & 0x200) >> 3) |
    ((vt & 0x200) >> 4) | 0x10 |
    ((vbs & 0x100) >> 5) |
    ((vs & 0x100) >> 6) |
    ((vd & 0x100) >> 7) |
    pub 8): ((vt & 0x100) >>,
    pub 4): reg.crt[VGA_CRTC_MAX_SCAN] |= 0x40 | ((vbs & 0x200) >>,
    pub vs: reg.crt[VGA_CRTC_V_SYNC_START] =,
    pub 0x20: reg.crt[VGA_CRTC_V_SYNC_END] = (ve & 0x0f) |,
    pub vd: reg.crt[VGA_CRTC_V_DISP_END] =,
    pub wd: reg.crt[VGA_CRTC_OFFSET] =,
    pub vbs: reg.crt[VGA_CRTC_V_BLANK_START] =,
    pub 1: reg.crt[VGA_CRTC_V_BLANK_END] = vbe +,
    pub 0xc3: reg.crt[VGA_CRTC_MODE] =,
    pub 0xff: reg.crt[VGA_CRTC_LINE_COMPARE] =,
// Banshee's nonvga stuff
    reg.ext[0x00] = (((ht & 0x100) >> 8) |
    ((hd & 0x100) >> 6) |
    ((hbs & 0x100) >> 4) |
    ((hbe & 0x40) >> 1) |
    ((hs & 0x100) >> 2) |
    pub 2)): ((he & 0x20) <<,
    reg.ext[0x01] = (((vt & 0x400) >> 10) |
    ((vd & 0x400) >> 8) |
    ((vbs & 0x400) >> 6) |
    pub 4)): ((vbe & 0x400) >>,
    reg.vgainit0 =	VGAINIT0_8BIT_DAC     |
    VGAINIT0_EXT_ENABLE   |
    VGAINIT0_WAKEUP_3C3   |
    VGAINIT0_ALT_READBACK |
    pub 0x1fffff: reg.vgainit1 = tdfx_inl(par, VGAINIT1) &,
    if (hwcursor)
    pub info->fix.smem_len: reg.curspataddr =,
    pub 0: reg.cursloc =,
    pub 0: reg.cursc0 =,
    pub 0xffffff: reg.cursc1 =,
    pub cpp: *mut *mut reg.stride = info->var.xres,
    reg.startaddr = info.var.yoffset * reg.stride
    pub cpp: *mut *mut + info->var.xoffset,
    pub &fout): reg.vidpll = do_calc_pll(freq,,

    pub &fout): reg.mempll = do_calc_pll(...,,
    pub &fout): reg.gfxpll = do_calc_pll(...,,

    if ((info.var.vmode & FB_VMODE_MASK) == FB_VMODE_INTERLACED)
    pub VIDCFG_INTERLACE: reg.vidcfg |=,
    pub MISCINIT0): reg.miscinit0 = tdfx_inl(par,,

    switch (info.var.bits_per_pixel) {
    case 8:
    case 24:
    pub 30): reg.miscinit0 &= ~(1 <<,
    pub 31): reg.miscinit0 &= ~(1 <<,
    case 16:
    pub 30): reg.miscinit0 |= (1 <<,
    pub 31): reg.miscinit0 |= (1 <<,
    case 32:
    pub 30): reg.miscinit0 |= (1 <<,
    pub 31): reg.miscinit0 &= ~(1 <<,
    }

    pub &reg): do_write_regs(info,,
// Now change fb_fix_screeninfo according to changes in par
    pub reg.stride: info->fix.line_length =,
    info.fix.visual = (info.var.bits_per_pixel == 8)
    ? FB_VISUAL_PSEUDOCOLOR
    pub FB_VISUAL_TRUECOLOR: :,
    DPRINTK("Graphics mode is now set at %dx%d depth %d\n",
    pub info->var.bits_per_pixel): info->var.xres, info->var.yres,,
    pub 0: return,
    }
// A handy macro shamelessly pinched from matroxfb

    static int tdfxfb_setcolreg(unsigned regno, unsigned red, unsigned green,
    unsigned blue, unsigned transp,
    struct fb_info *info)
    {
    pub info->par: *mut *mut tdfx_par par =,
    pub rgbcol: u32,
    if (regno >= info.cmap.len || regno > 255)
    pub 1: return,
// grayscale works only partially under directcolor
    if (info.var.grayscale) {
// grayscale = 0.30*R + 0.59*G + 0.11*B
    pub 8: *mut *mut *mut *mut blue = (red  77 + green  151 + blue  28) >>,
    pub blue: green =,
    pub blue: red =,
    }
    switch (info.fix.visual) {
    case FB_VISUAL_PSEUDOCOLOR:
    rgbcol = (((u32)red   & 0xff00) << 8) |
    (((u32)green & 0xff00) << 0) |
    pub 8): (((u32)blue & 0xff00) >>,
    pub rgbcol): do_setpalentry(par, regno,,
// Truecolor has no hardware color palettes.
    case FB_VISUAL_TRUECOLOR:
    if (regno < 16) {
    rgbcol = (CNVT_TOHW(red, info.var.red.length) <<
    info.var.red.offset) |
    (CNVT_TOHW(green, info.var.green.length) <<
    info.var.green.offset) |
    (CNVT_TOHW(blue, info.var.blue.length) <<
    info.var.blue.offset) |
    (CNVT_TOHW(transp, info.var.transp.length) <<
    pub rgbcol: par->palette[regno] =,
    }
    default:
    pub info->var.bits_per_pixel): DPRINTK("bad depth %u\n",,
    }
    pub 0: return,
    }
// 0 unblank, 1 blank, 2 no vsync, 3 no hsync, 4 off
#[no_mangle]
unsafe extern "C" fn tdfxfb_blank(blank: c_int, info: *mut fb_info) -> c_int {
    static int tdfxfb_blank(int blank, struct fb_info *info)
    {
    pub info->par: *mut *mut tdfx_par par =,
    pub 1: int vgablank =,
    pub DACMODE): u32 dacmode = tdfx_inl(par,,
    pub BIT(3)): dacmode &= ~(BIT(1) |,
    switch (blank) {
    pub /: *mut *mut case FB_BLANK_UNBLANK: / Screen: On; HSync: On, VSync: On,
    pub 0: vgablank =,
    pub /: *mut *mut case FB_BLANK_NORMAL: / Screen: Off; HSync: On, VSync: On,
    pub /: *mut *mut case FB_BLANK_VSYNC_SUSPEND: / Screen: Off; HSync: On, VSync: Off,
    pub BIT(3): dacmode |=,
    pub /: *mut *mut case FB_BLANK_HSYNC_SUSPEND: / Screen: Off; HSync: Off, VSync: On,
    pub BIT(1): dacmode |=,
    pub /: *mut *mut case FB_BLANK_POWERDOWN: / Screen: Off; HSync: Off, VSync: Off,
    pub BIT(3): dacmode |= BIT(1) |,
    }
    pub 1): banshee_make_room(par,,
    pub dacmode): tdfx_outl(par, DACMODE,,
    if (vgablank)
    else
    pub 0: return,
    }
//
// Set the starting position of the visible screen to var->yoffset
//
    static int tdfxfb_pan_display(struct fb_var_screeninfo *var,
    struct fb_info *info)
    {
    pub info->par: *mut *mut tdfx_par par =,
    pub info->fix.line_length: *mut *mut u32 addr = var->yoffset,
    if (nopan || var.xoffset)
    pub -EINVAL: return,
    pub 1): banshee_make_room(par,,
    pub addr): tdfx_outl(par, VIDDESKSTART,,
    pub 0: return,
    }

//
// FillRect 2D command (solidfill or invert (via ROP_XOR))
//
    static void tdfxfb_fillrect(struct fb_info *info,
    const struct fb_fillrect *rect)
    {
    pub info->par: *mut *mut tdfx_par par =,
    pub info->var.bits_per_pixel: u32 bpp =,
    pub info->fix.line_length: u32 stride =,
    pub 13): u32 fmt = stride | ((bpp + ((bpp == 8) ? 0 : 8)) <<,
    pub tdfx_rop: c_int,
    pub rect->dx: u32 dx =,
    pub rect->dy: u32 dy =,
    pub 0: u32 dstbase =,
    if (rect.rop == ROP_COPY)
    pub TDFX_ROP_COPY: tdfx_rop =,
    else
    pub TDFX_ROP_XOR: tdfx_rop =,
// assume always rect->height < 4096
    if (dy + rect.height > 4095) {
    pub dy: *mut *mut dstbase = stride,
    pub 0: dy =,
    }
// assume always rect->width < 4096
    if (dx + rect.width > 4095) {
    pub 3: *mut *mut dstbase += dx  bpp >>,
    pub 0: dx =,
    }
    pub 6): banshee_make_room(par,,
    pub fmt): tdfx_outl(par, DSTFORMAT,,
    if (info.fix.visual == FB_VISUAL_PSEUDOCOLOR) {
    pub rect->color): tdfx_outl(par, COLORFORE,,
    } else { /* FB_VISUAL_TRUECOLOR */
    pub par->palette[rect->color]): tdfx_outl(par, COLORFORE,,
    }
    pub 24)): tdfx_outl(par, COMMAND_2D, COMMAND_2D_FILLRECT | (tdfx_rop <<,
    pub dstbase): tdfx_outl(par, DSTBASE,,
    pub 16)): tdfx_outl(par, DSTSIZE, rect->width | (rect->height <<,
    pub 16)): tdfx_outl(par, LAUNCH_2D, dx | (dy <<,
    }
//
// Screen-to-Screen BitBlt 2D command (for the bmove fb op.)
//
    static void tdfxfb_copyarea(struct fb_info *info,
    const struct fb_copyarea *area)
    {
    pub info->par: *mut *mut tdfx_par par =,
    pub area->dy: u32 sx = area->sx, sy = area->sy, dx = area->dx, dy =,
    pub info->var.bits_per_pixel: u32 bpp =,
    pub info->fix.line_length: u32 stride =,
    pub 24): u32 blitcmd = COMMAND_2D_S2S_BITBLT | (TDFX_ROP_COPY <<,
    pub 13): u32 fmt = stride | ((bpp + ((bpp == 8) ? 0 : 8)) <<,
    pub 0: u32 dstbase =,
    pub 0: u32 srcbase =,
// assume always area->height < 4096
    if (sy + area.height > 4095) {
    pub sy: *mut *mut srcbase = stride,
    pub 0: sy =,
    }
// assume always area->width < 4096
    if (sx + area.width > 4095) {
    pub 3: *mut *mut srcbase += sx  bpp >>,
    pub 0: sx =,
    }
// assume always area->height < 4096
    if (dy + area.height > 4095) {
    pub dy: *mut *mut dstbase = stride,
    pub 0: dy =,
    }
// assume always area->width < 4096
    if (dx + area.width > 4095) {
    pub 3: *mut *mut dstbase += dx  bpp >>,
    pub 0: dx =,
    }
    if (area.sx <= area.dx) {
// -X
    pub BIT(14): blitcmd |=,
    pub 1: sx += area->width -,
    pub 1: dx += area->width -,
    }
    if (area.sy <= area.dy) {
// -Y
    pub BIT(15): blitcmd |=,
    pub 1: sy += area->height -,
    pub 1: dy += area->height -,
    }
    pub 8): banshee_make_room(par,,
    pub fmt): tdfx_outl(par, SRCFORMAT,,
    pub fmt): tdfx_outl(par, DSTFORMAT,,
    pub blitcmd): tdfx_outl(par, COMMAND_2D,,
    pub 16)): tdfx_outl(par, DSTSIZE, area->width | (area->height <<,
    pub 16)): tdfx_outl(par, DSTXY, dx | (dy <<,
    pub srcbase): tdfx_outl(par, SRCBASE,,
    pub dstbase): tdfx_outl(par, DSTBASE,,
    pub 16)): tdfx_outl(par, LAUNCH_2D, sx | (sy <<,
    }
#[no_mangle]
unsafe extern "C" fn tdfxfb_imageblit(info: *mut fb_info, image: *const fb_image) {
    static void tdfxfb_imageblit(struct fb_info *info, const struct fb_image *image)
    {
    pub info->par: *mut *mut tdfx_par par =,
    pub 3): *mut *mut *mut int size = image->height  ((image->width  image->depth + 7) >>,
    pub fifo_free: c_int,
    pub info->fix.line_length: int i, stride =,
    pub info->var.bits_per_pixel: u32 bpp =,
    pub 13): u32 dstfmt = stride | ((bpp + ((bpp == 8) ? 0 : 8)) <<,
    pub image->data: *mut *mut *mut u8 chardata = (u8 ),
    pub srcfmt: u32,
    pub image->dx: u32 dx =,
    pub image->dy: u32 dy =,
    pub 0: u32 dstbase =,
    if (image.depth != 1) {

    pub 2)): banshee_make_room(par, 6 + ((size + 3) >>,
    srcfmt = stride | ((bpp + ((bpp == 8) ? 0 : 8)) << 13) |

    pub image): cfb_imageblit(info,,

    }
    pub 9): banshee_make_room(par,,
    switch (info.fix.visual) {
    case FB_VISUAL_PSEUDOCOLOR:
    pub image->fg_color): tdfx_outl(par, COLORFORE,,
    pub image->bg_color): tdfx_outl(par, COLORBACK,,
    case FB_VISUAL_TRUECOLOR:
    default:
    tdfx_outl(par, COLORFORE,
    tdfx_outl(par, COLORBACK,
    }

    pub BIT(20): srcfmt = 0x400000 |,

    pub 0x400000: srcfmt =,

// assume always image->height < 4096
    if (dy + image.height > 4095) {
    pub dy: *mut *mut dstbase = stride,
    pub 0: dy =,
    }
// assume always image->width < 4096
    if (dx + image.width > 4095) {
    pub 3: *mut *mut dstbase += dx  bpp >>,
    pub 0: dx =,
    }
    pub dstbase): tdfx_outl(par, DSTBASE,,
    pub 0): tdfx_outl(par, SRCXY,,
    pub 16)): tdfx_outl(par, DSTXY, dx | (dy <<,
    tdfx_outl(par, COMMAND_2D,
    pub 24)): COMMAND_2D_H2S_BITBLT | (TDFX_ROP_COPY <<,
    pub srcfmt): tdfx_outl(par, SRCFORMAT,,
    pub dstfmt): tdfx_outl(par, DSTFORMAT,,
    pub 16)): tdfx_outl(par, DSTSIZE, image->width | (image->height <<,
// A count of how many free FIFO entries we've requested.
// When this goes negative, we need to request more.
    pub 0: fifo_free =,
// Send four bytes at a time of data
    pub {: for (i = (size >> 2); i > 0; i--),
    if (--fifo_free < 0) {
    pub 31: fifo_free =,
    pub fifo_free): banshee_make_room(par,,
    }
    pub )chardata): *mut *mut tdfx_outl(par, LAUNCH_2D, (u32,
    pub 4: chardata +=,
    }
// Send the leftovers now
    pub 3): banshee_make_room(par,,
    switch (size % 4) {
    case 0:
    case 1:
    pub chardata): *mut tdfx_outl(par, LAUNCH_2D,,
    case 2:
    pub )chardata): *mut *mut tdfx_outl(par, LAUNCH_2D, (u16,
    case 3:
    tdfx_outl(par, LAUNCH_2D,
// (u16 *)chardata | (chardata[3] << 24));
    }
    }

#[no_mangle]
unsafe extern "C" fn tdfxfb_cursor(info: *mut fb_info, cursor: *mut fb_cursor) -> c_int {
    static int tdfxfb_cursor(struct fb_info *info, struct fb_cursor *cursor)
    {
    pub info->par: *mut *mut tdfx_par par =,
    pub vidcfg: u32,
    if (!hwcursor)
    pub /: *mut *mut return -EINVAL; / just to force soft_cursor() call,
// Too large of a cursor or wrong bpp :-(
    if (cursor.image.width > 64 ||
    cursor.image.height > 64 ||
    cursor.image.depth > 1)
    pub -EINVAL: return,
    pub VIDPROCCFG): vidcfg = tdfx_inl(par,,
    if (cursor.enable)
    pub VIDCFG_HWCURSOR_ENABLE): tdfx_outl(par, VIDPROCCFG, vidcfg |,
    else
    pub ~VIDCFG_HWCURSOR_ENABLE): tdfx_outl(par, VIDPROCCFG, vidcfg &,
//
// If the cursor is not be changed this means either we want the
// current cursor state (if enable is set) or we want to query what
// we can do with the cursor (if enable is not set)
//
    if (!cursor.set)
    pub 0: return,
// fix cursor color - XFree86 forgets to restore it properly
    if (cursor.set & FB_CUR_SETCMAP) {
    pub info->cmap: fb_cmap cmap =,
    pub cursor->image.bg_color: u32 bg_idx =,
    pub cursor->image.fg_color: u32 fg_idx =,
    pub fg_color: unsigned long bg_color,,
    fg_color = (((u32)cmap.red[fg_idx]   & 0xff00) << 8) |
    (((u32)cmap.green[fg_idx] & 0xff00) << 0) |
    pub 8): (((u32)cmap.blue[fg_idx] & 0xff00) >>,
    bg_color = (((u32)cmap.red[bg_idx]   & 0xff00) << 8) |
    (((u32)cmap.green[bg_idx] & 0xff00) << 0) |
    pub 8): (((u32)cmap.blue[bg_idx] & 0xff00) >>,
    pub 2): banshee_make_room(par,,
    pub bg_color): tdfx_outl(par, HWCURC0,,
    pub fg_color): tdfx_outl(par, HWCURC1,,
    }
    if (cursor.set & FB_CUR_SETPOS) {
    pub cursor->image.dx: int x =,
    pub info->var.yoffset: int y = cursor->image.dy -,
    pub 63: x +=,
    pub 63: y +=,
    pub 1): banshee_make_room(par,,
    pub x): tdfx_outl(par, HWCURLOC, (y << 16) +,
    }
    if (cursor.set & (FB_CUR_SETIMAGE | FB_CUR_SETSHAPE)) {
//
// Voodoo 3 and above cards use 2 monochrome cursor patterns.
// The reason is so the card can fetch 8 words at a time
// and are stored on chip for use for the next 8 scanlines.
// This reduces the number of times for access to draw the
// cursor for each screen refresh.
// Each pattern is a bitmap of 64 bit wide and 64 bit high
// (total of 8192 bits or 1024 bytes). The two patterns are
// stored in such a way that pattern 0 always resides in the
// lower half (least significant 64 bits) of a 128 bit word
// and pattern 1 the upper half. If you examine the data of
// the cursor image the graphics card uses then from the
// beginning you see line one of pattern 0, line one of
// pattern 1, line two of pattern 0, line two of pattern 1,
// etc etc. The linear stride for the cursor is always 16 bytes
// (128 bits) which is the maximum cursor width times two for
// the two monochrome patterns.
//
    pub info->fix.smem_len: *mut *mut u8 __iomem cursorbase = info->screen_base +,
    pub )cursor->image.data: *mut *mut u8 bitmap = (u8,
    pub )cursor->mask: *mut *mut u8 mask = (u8,
    pub i: c_int,
    pub 1024): fb_memset_io(cursorbase, 0,,
    pub {: for (i = 0; i < cursor->image.height; i++),
    pub 0: int h =,
    pub 3: int j = (cursor->image.width + 7) >>,
    pub {: for (; j > 0; j--),
    pub bitmap: *mut *mut u8 data = mask ^,
    if (cursor.rop == ROP_COPY)
    pub bitmap: *mut *mut data = mask &,
// Pattern 0. Copy the cursor mask to it
    pub h): *mut *mut fb_writeb(mask, cursorbase +,
// Pattern 1. Copy the cursor bitmap to it
    pub 8): fb_writeb(data, cursorbase + h +,
    }
    pub 16: cursorbase +=,
    }
    }
    pub 0: return,
    }
    static const struct fb_ops tdfxfb_ops = {
    .owner		= THIS_MODULE,
    __FB_DEFAULT_IOMEM_OPS_RDWR,
    .fb_check_var	= tdfxfb_check_var,
    .fb_set_par	= tdfxfb_set_par,
    .fb_setcolreg	= tdfxfb_setcolreg,
    .fb_blank	= tdfxfb_blank,
    .fb_pan_display	= tdfxfb_pan_display,
    .fb_sync	= banshee_wait_idle,
    .fb_cursor	= tdfxfb_cursor,

    .fb_fillrect	= tdfxfb_fillrect,
    .fb_copyarea	= tdfxfb_copyarea,
    .fb_imageblit	= tdfxfb_imageblit,

    __FB_DEFAULT_IOMEM_OPS_DRAW,

    __FB_DEFAULT_IOMEM_OPS_MMAP,
}

// The voo GPIO registers don't have individual masks for each bit
    so we always have to read before writing. */
#[no_mangle]
unsafe extern "C" fn tdfxfb_i2c_setscl(data: *mut c_void, val: c_int) {
    static void tdfxfb_i2c_setscl(void *data, int val)
    {
    struct tdfxfb_i2c_chan 	*chan = data;
    struct tdfx_par 	*par = chan.par;
    unsigned int r;
    r = tdfx_inl(par, VIDSERPARPORT);
    if (val)
    r |= I2C_SCL_OUT;
    else
    r &= ~I2C_SCL_OUT;
    tdfx_outl(par, VIDSERPARPORT, r);
    tdfx_inl(par, VIDSERPARPORT);	/* flush posted write */
    }
#[no_mangle]
unsafe extern "C" fn tdfxfb_i2c_setsda(data: *mut c_void, val: c_int) {
    static void tdfxfb_i2c_setsda(void *data, int val)
    {
    struct tdfxfb_i2c_chan 	*chan = data;
    struct tdfx_par 	*par = chan.par;
    unsigned int r;
    r = tdfx_inl(par, VIDSERPARPORT);
    if (val)
    r |= I2C_SDA_OUT;
    else
    r &= ~I2C_SDA_OUT;
    tdfx_outl(par, VIDSERPARPORT, r);
    tdfx_inl(par, VIDSERPARPORT);	/* flush posted write */
    }
// The GPIO pins are open drain, so the pins always remain outputs.
    We rely on the i2c-algo-bit routines to set the pins high before
    reading the input from other chips. */
#[no_mangle]
unsafe extern "C" fn tdfxfb_i2c_getscl(data: *mut c_void) -> c_int {
    static int tdfxfb_i2c_getscl(void *data)
    {
    struct tdfxfb_i2c_chan 	*chan = data;
    struct tdfx_par 	*par = chan.par;
    return (0 != (tdfx_inl(par, VIDSERPARPORT) & I2C_SCL_IN));
    }
#[no_mangle]
unsafe extern "C" fn tdfxfb_i2c_getsda(data: *mut c_void) -> c_int {
    static int tdfxfb_i2c_getsda(void *data)
    {
    struct tdfxfb_i2c_chan 	*chan = data;
    struct tdfx_par 	*par = chan.par;
    return (0 != (tdfx_inl(par, VIDSERPARPORT) & I2C_SDA_IN));
    }
#[no_mangle]
unsafe extern "C" fn tdfxfb_ddc_setscl(data: *mut c_void, val: c_int) {
    static void tdfxfb_ddc_setscl(void *data, int val)
    {
    struct tdfxfb_i2c_chan 	*chan = data;
    struct tdfx_par 	*par = chan.par;
    unsigned int r;
    r = tdfx_inl(par, VIDSERPARPORT);
    if (val)
    r |= DDC_SCL_OUT;
    else
    r &= ~DDC_SCL_OUT;
    tdfx_outl(par, VIDSERPARPORT, r);
    tdfx_inl(par, VIDSERPARPORT);	/* flush posted write */
    }
#[no_mangle]
unsafe extern "C" fn tdfxfb_ddc_setsda(data: *mut c_void, val: c_int) {
    static void tdfxfb_ddc_setsda(void *data, int val)
    {
    struct tdfxfb_i2c_chan 	*chan = data;
    struct tdfx_par 	*par = chan.par;
    unsigned int r;
    r = tdfx_inl(par, VIDSERPARPORT);
    if (val)
    r |= DDC_SDA_OUT;
    else
    r &= ~DDC_SDA_OUT;
    tdfx_outl(par, VIDSERPARPORT, r);
    tdfx_inl(par, VIDSERPARPORT);	/* flush posted write */
    }
#[no_mangle]
unsafe extern "C" fn tdfxfb_ddc_getscl(data: *mut c_void) -> c_int {
    static int tdfxfb_ddc_getscl(void *data)
    {
    struct tdfxfb_i2c_chan 	*chan = data;
    struct tdfx_par 	*par = chan.par;
    return (0 != (tdfx_inl(par, VIDSERPARPORT) & DDC_SCL_IN));
    }
#[no_mangle]
unsafe extern "C" fn tdfxfb_ddc_getsda(data: *mut c_void) -> c_int {
    static int tdfxfb_ddc_getsda(void *data)
    {
    struct tdfxfb_i2c_chan 	*chan = data;
    struct tdfx_par 	*par = chan.par;
    return (0 != (tdfx_inl(par, VIDSERPARPORT) & DDC_SDA_IN));
    }
    static int tdfxfb_setup_ddc_bus(struct tdfxfb_i2c_chan *chan, const char *name,
    struct device *dev)
    {
    int rc;
    strscpy(chan.adapter.name, name, sizeof(chan.adapter.name));
    chan.adapter.owner		= THIS_MODULE;
    chan.adapter.algo_data		= &chan.algo;
    chan.adapter.dev.parent	= dev;
    chan.algo.setsda		= tdfxfb_ddc_setsda;
    chan.algo.setscl		= tdfxfb_ddc_setscl;
    chan.algo.getsda		= tdfxfb_ddc_getsda;
    chan.algo.getscl		= tdfxfb_ddc_getscl;
    chan.algo.udelay		= 10;
    chan.algo.timeout		= msecs_to_jiffies(500);
    chan.algo.data 		= chan;
    i2c_set_adapdata(&chan.adapter, chan);
    rc = i2c_bit_add_bus(&chan.adapter);
    if (rc == 0)
    DPRINTK("I2C bus %s registered.\n", name);
    else
    chan.par = core::ptr::null_mut();
    return rc;
    }
    static int tdfxfb_setup_i2c_bus(struct tdfxfb_i2c_chan *chan, const char *name,
    struct device *dev)
    {
    int rc;
    strscpy(chan.adapter.name, name, sizeof(chan.adapter.name));
    chan.adapter.owner		= THIS_MODULE;
    chan.adapter.algo_data		= &chan.algo;
    chan.adapter.dev.parent	= dev;
    chan.algo.setsda		= tdfxfb_i2c_setsda;
    chan.algo.setscl		= tdfxfb_i2c_setscl;
    chan.algo.getsda		= tdfxfb_i2c_getsda;
    chan.algo.getscl		= tdfxfb_i2c_getscl;
    chan.algo.udelay		= 10;
    chan.algo.timeout		= msecs_to_jiffies(500);
    chan.algo.data 		= chan;
    i2c_set_adapdata(&chan.adapter, chan);
    rc = i2c_bit_add_bus(&chan.adapter);
    if (rc == 0)
    DPRINTK("I2C bus %s registered.\n", name);
    else
    chan.par = core::ptr::null_mut();
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn tdfxfb_create_i2c_busses(info: *mut fb_info) {
    static void tdfxfb_create_i2c_busses(struct fb_info *info)
    {
    struct tdfx_par *par = info.par;
    tdfx_outl(par, VIDINFORMAT, 0x8160);
    tdfx_outl(par, VIDSERPARPORT, 0xcffc0020);
    par.chan[0].par = par;
    par.chan[1].par = par;
    tdfxfb_setup_ddc_bus(&par.chan[0], "Voodoo3-DDC", info.device);
    tdfxfb_setup_i2c_bus(&par.chan[1], "Voodoo3-I2C", info.device);
    }
#[no_mangle]
unsafe extern "C" fn tdfxfb_delete_i2c_busses(par: *mut tdfx_par) {
    static void tdfxfb_delete_i2c_busses(struct tdfx_par *par)
    {
    if (par.chan[0].par)
    i2c_del_adapter(&par.chan[0].adapter);
    par.chan[0].par = core::ptr::null_mut();
    if (par.chan[1].par)
    i2c_del_adapter(&par.chan[1].adapter);
    par.chan[1].par = core::ptr::null_mut();
    }
    static int tdfxfb_probe_i2c_connector(struct tdfx_par *par,
    struct fb_monspecs *specs)
    {
    u8 *edid = core::ptr::null_mut();
    DPRINTK("Probe DDC Bus\n");
    if (par.chan[0].par)
    edid = fb_ddc_read(&par.chan[0].adapter);
    if (edid) {
    fb_edid_to_monspecs(edid, specs);
    kfree(edid);
    return 0;
    }
    return 1;
    }

//
// tdfxfb_probe - Device Initializiation
//
// @pdev:  PCI Device to initialize
// @id:    PCI Device ID
//
// Initializes and allocates resources for PCI device @pdev.
//
#[no_mangle]
unsafe extern "C" fn tdfxfb_probe(pdev: *mut pci_dev, id: *const pci_device_id) -> c_int {
    static int tdfxfb_probe(struct pci_dev *pdev, const struct pci_device_id *id)
    {
    struct tdfx_par *default_par;
    struct fb_info *info;
    int err, lpitch;
    struct fb_monspecs *specs;
    bool found;
    err = aperture_remove_conflicting_pci_devices(pdev, "tdfxfb");
    if (err)
    return err;
    err = pcim_enable_device(pdev);
    if (err) {
    printk(KERN_ERR "tdfxfb: Can't enable pdev: %d\n", err);
    return err;
    }
    info = framebuffer_alloc(sizeof(struct tdfx_par), &pdev.dev);
    if (!info)
    return -ENOMEM;
    default_par = info.par;
    info.fix = tdfx_fix;
// Configure the default fb_fix_screeninfo first
    switch (pdev.device) {
    case PCI_DEVICE_ID_3DFX_BANSHEE:
    strcpy(info.fix.id, "3Dfx Banshee");
    default_par.max_pixclock = BANSHEE_MAX_PIXCLOCK;
    break;
    case PCI_DEVICE_ID_3DFX_VOODOO3:
    strcpy(info.fix.id, "3Dfx Voodoo3");
    default_par.max_pixclock = VOODOO3_MAX_PIXCLOCK;
    break;
    case PCI_DEVICE_ID_3DFX_VOODOO5:
    strcpy(info.fix.id, "3Dfx Voodoo5");
    default_par.max_pixclock = VOODOO5_MAX_PIXCLOCK;
    break;
    }
    info.fix.mmio_start = pci_resource_start(pdev, 0);
    info.fix.mmio_len = pci_resource_len(pdev, 0);
    if (!request_mem_region(info.fix.mmio_start, info.fix.mmio_len,
    "tdfx regbase")) {
    printk(KERN_ERR "tdfxfb: Can't reserve regbase\n");
    goto out_err;
    }
    default_par.regbase_virt =
    ioremap(info.fix.mmio_start, info.fix.mmio_len);
    if (!default_par.regbase_virt) {
    printk(KERN_ERR "fb: Can't remap %s register area.\n",
    info.fix.id);
    goto out_err_regbase;
    }
    if (tdfxfb_hw_init(info, pdev))
    goto out_err_regbase;
    info.fix.smem_start = pci_resource_start(pdev, 1);
    info.fix.smem_len = do_lfb_size(default_par, pdev.device);
    if (!info.fix.smem_len) {
    printk(KERN_ERR "fb: Can't count %s memory.\n", info.fix.id);
    goto out_err_regbase;
    }
    if (!request_mem_region(info.fix.smem_start,
    pci_resource_len(pdev, 1), "tdfx smem")) {
    printk(KERN_ERR "tdfxfb: Can't reserve smem\n");
    goto out_err_regbase;
    }
    info.screen_base = ioremap_wc(info.fix.smem_start,
    info.fix.smem_len);
    if (!info.screen_base) {
    printk(KERN_ERR "fb: Can't remap %s framebuffer.\n",
    info.fix.id);
    goto out_err_screenbase;
    }
    default_par.iobase = pci_resource_start(pdev, 2);
    if (!request_region(pci_resource_start(pdev, 2),
    pci_resource_len(pdev, 2), "tdfx iobase")) {
    printk(KERN_ERR "tdfxfb: Can't reserve iobase\n");
    goto out_err_screenbase;
    }
    printk(KERN_INFO "fb: %s memory = %dK\n", info.fix.id,
    info.fix.smem_len >> 10);
    if (!nomtrr)
    default_par.wc_cookie= arch_phys_wc_add(info.fix.smem_start,
    info.fix.smem_len);
    info.fix.ypanstep	= nopan ? 0 : 1;
    info.fix.ywrapstep	= nowrap ? 0 : 1;
    info.fbops		= &tdfxfb_ops;
    info.pseudo_palette	= default_par.palette;
    info.flags		= FBINFO_HWACCEL_YPAN;

    info.flags		|= FBINFO_HWACCEL_FILLRECT |
    FBINFO_HWACCEL_COPYAREA |
    FBINFO_HWACCEL_IMAGEBLIT |
    FBINFO_READS_FAST;

// reserve 8192 bits for cursor
// the 2.4 driver says PAGE_MASK boundary is not enough for Voodoo4
    if (hwcursor)
    info.fix.smem_len = (info.fix.smem_len - 1024) &
    (PAGE_MASK << 1);
    specs = &info.monspecs;
    found = false;
    info.var.bits_per_pixel = 8;

    tdfxfb_create_i2c_busses(info);
    err = tdfxfb_probe_i2c_connector(default_par, specs);
    if (!err) {
    if (specs.modedb == core::ptr::null_mut())
    DPRINTK("Unable to get Mode Database\n");
    else {
    const struct fb_videomode *m;
    fb_videomode_to_modelist(specs.modedb,
    specs.modedb_len,
    &info.modelist);
    m = fb_find_best_display(specs, &info.modelist);
    if (m) {
    fb_videomode_to_var(&info.var, m);
// fill all other info->var's fields
    if (tdfxfb_check_var(&info.var, info) < 0)
    info.var = tdfx_var;
    else
    found = true;
    }
    }
    }

    if (!mode_option && !found)
    mode_option = "640x480@60";
    if (mode_option) {
    err = fb_find_mode(&info.var, info, mode_option,
    specs.modedb, specs.modedb_len,
    core::ptr::null_mut(), info.var.bits_per_pixel);
    if (!err || err == 4)
    info.var = tdfx_var;
    }
    if (found) {
    fb_destroy_modedb(specs.modedb);
    specs.modedb = core::ptr::null_mut();
    }
// maximize virtual vertical length
    lpitch = info.var.xres_virtual * ((info.var.bits_per_pixel + 7) >> 3);
    info.var.yres_virtual = info.fix.smem_len / lpitch;
    if (info.var.yres_virtual < info.var.yres)
    goto out_err_iobase;
    if (fb_alloc_cmap(&info.cmap, 256, 0) < 0) {
    printk(KERN_ERR "tdfxfb: Can't allocate color map\n");
    goto out_err_iobase;
    }
//
// Program a video mode and clear the framebuffer now, this
// ensures the display comes up even if fbcon doesn't bind
// when the framebuffer is registered.
//
    tdfxfb_set_par(info);
    memset_io(info.screen_base, 0, info.fix.smem_len);
    if (register_framebuffer(info) < 0) {
    printk(KERN_ERR "tdfxfb: can't register framebuffer\n");
    fb_dealloc_cmap(&info.cmap);
    goto out_err_iobase;
    }
//
// Our driver data
//
    pci_set_drvdata(pdev, info);
    return 0;
    out_err_iobase:

    fb_destroy_modelist(&info.modelist);
    tdfxfb_delete_i2c_busses(default_par);

    arch_phys_wc_del(default_par.wc_cookie);
    release_region(pci_resource_start(pdev, 2),
    pci_resource_len(pdev, 2));
    out_err_screenbase:
    if (info.screen_base)
    iounmap(info.screen_base);
    release_mem_region(info.fix.smem_start, pci_resource_len(pdev, 1));
    out_err_regbase:
//
// Cleanup after anything that was remapped/allocated.
//
    if (default_par.regbase_virt)
    iounmap(default_par.regbase_virt);
    release_mem_region(info.fix.mmio_start, info.fix.mmio_len);
    out_err:
    framebuffer_release(info);
    return -ENXIO;
    }

#[no_mangle]
unsafe extern "C" fn tdfxfb_setup(options: *mut c_char) -> void __init {
    static void __init tdfxfb_setup(char *options)
    {
    char *this_opt;
    if (!options || !*options)
    return;
    while ((this_opt = strsep(&options, ",")) != core::ptr::null_mut()) {
    if (!*this_opt)
    continue;
    if (!strcmp(this_opt, "nopan")) {
    nopan = 1;
    } else if (!strcmp(this_opt, "nowrap")) {
    nowrap = 1;
    } else if (!strncmp(this_opt, "hwcursor=", 9)) {
    hwcursor = simple_strtoul(this_opt + 9, core::ptr::null_mut(), 0);
    } else if (!strncmp(this_opt, "nomtrr", 6)) {
    nomtrr = 1;
    } else {
    mode_option = this_opt;
    }
    }
    }

//
// tdfxfb_remove - Device removal
//
// @pdev:  PCI Device to cleanup
//
// Releases all resources allocated during the course of the driver's
// lifetime for the PCI device @pdev.
//
#[no_mangle]
unsafe extern "C" fn tdfxfb_remove(pdev: *mut pci_dev) {
    static void tdfxfb_remove(struct pci_dev *pdev)
    {
    struct fb_info *info = pci_get_drvdata(pdev);
    struct tdfx_par *par = info.par;
    unregister_framebuffer(info);

    tdfxfb_delete_i2c_busses(par);

    arch_phys_wc_del(par.wc_cookie);
    iounmap(par.regbase_virt);
    iounmap(info.screen_base);
// Clean up after reserved regions
    release_region(pci_resource_start(pdev, 2),
    pci_resource_len(pdev, 2));
    release_mem_region(pci_resource_start(pdev, 1),
    pci_resource_len(pdev, 1));
    release_mem_region(pci_resource_start(pdev, 0),
    pci_resource_len(pdev, 0));
    fb_dealloc_cmap(&info.cmap);
    framebuffer_release(info);
    }
#[no_mangle]
unsafe extern "C" fn tdfxfb_init() -> int __init {
    static int __init tdfxfb_init(void)
    {

    char *option = core::ptr::null_mut();

    if (fb_modesetting_disabled("tdfxfb"))
    return -ENODEV;

    if (fb_get_options("tdfxfb", &option))
    return -ENODEV;
    tdfxfb_setup(option);

    return pci_register_driver(&tdfxfb_driver);
    }
#[no_mangle]
unsafe extern "C" fn tdfxfb_exit() -> void __exit {
    static void __exit tdfxfb_exit(void)
    {
    pci_unregister_driver(&tdfxfb_driver);
    }
    MODULE_AUTHOR("Hannu Mallat <hmallat@cc.hut.fi>");
    MODULE_DESCRIPTION("3Dfx framebuffer device driver");
    MODULE_LICENSE("GPL");
    module_param(hwcursor, int, 0644);
    MODULE_PARM_DESC(hwcursor, "Enable hardware cursor "
    "(1=enable, 0=disable, default=1)");
    module_param(mode_option, charp, 0);
    MODULE_PARM_DESC(mode_option, "Initial video mode e.g. '648x480-8@60'");
    module_param(nomtrr, bool, 0);
    MODULE_PARM_DESC(nomtrr, "Disable MTRR support (default: enabled)");
    module_init(tdfxfb_init);
    module_exit(tdfxfb_exit);
