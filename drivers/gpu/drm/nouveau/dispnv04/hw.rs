//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/dispnv04/hw.h
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
// Copyright 2008 Stuart Bennett
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
// WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF
// OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

extern "C" {
    pub fn NVWriteVgaSeq(: *mut drm_device, head: c_int, index: u8, value: u8);
}
extern "C" {
    pub fn NVReadVgaSeq(: *mut drm_device, head: c_int, index: u8) -> u8;
}
extern "C" {
    pub fn NVWriteVgaGr(: *mut drm_device, head: c_int, index: u8, value: u8);
}
extern "C" {
    pub fn NVReadVgaGr(: *mut drm_device, head: c_int, index: u8) -> u8;
}
extern "C" {
    pub fn NVSetOwner(: *mut drm_device, owner: c_int);
}
extern "C" {
    pub fn NVBlankScreen(: *mut drm_device, head: c_int, blank: bool);
}
extern "C" {
    pub fn nouveau_hw_pllvals_to_clk(pllvals: *mut nvkm_pll_vals) -> c_int;
}
extern "C" {
    pub fn nouveau_hw_get_clock(: *mut drm_device, plltype: nvbios_pll_type) -> c_int;
}
extern "C" {
    pub fn nouveau_hw_save_vga_fonts(: *mut drm_device, save: bool);
}
// nouveau_calc.c
extern "C" {
    pub fn NVReadRAMDAC(_arg: dev, _arg: ramdac, 8: *mut *mut NV_PRAMDAC_FP_TMDS_DATA + dl) -> return;
}
// CR57 and CR58 are a fun pair of regs. CR57 provides an index (0-0xf) for CR58
// I suspect they in fact do nothing, but are merely a way to carry useful
// per-head variables around
//
// Known uses:
// CR57		CR58
// 0x00		index to the appropriate dcb entry (or 7f for inactive)
// 0x02		dcb entry's "or" value (or 00 for inactive)
// 0x03		bit0 set for dual link (LVDS, possibly elsewhere too)
// 0x08 or 0x09	pxclk in MHz
// 0x0f		laptop panel info -	low nibble for PEXTDEV_BOOT_0 strap
// high nibble for xlat strap value
//
extern "C" {
    pub fn NVReadVgaCrtc(_arg: dev, _arg: head, _arg: NV_CIO_CRE_58) -> return;
}
// Only NV4x have two pvio ranges; other twoHeads cards MUST call
// NVSetOwner for the relevant head to be programmed
// Only NV4x have two pvio ranges; other twoHeads cards MUST call
// NVSetOwner for the relevant head to be programmed
// Reenable sequencer, then turn on screen
// makes cr0-7 on the specified head read-only
// shadow lock: connects 0x60?3d? regs to "real" 0x3d? regs
// bit7: unlocks HDT, HBS, HBE, HRS, HRE, HEB
// bit6: seems to have some effect on CR09 (double scan, VBS_9)
// bit5: unlocks HDE
// bit4: unlocks VDE
// bit3: unlocks VDT, OVL, VRS, ?VRE?, VBS, VBE, LSR, EBR
// bit2: same as bit 1 of 0x60?804
// bit0: same as bit 0 of 0x60?804
//
// 0xfa is generic "unlock all" mask
// renders the extended crtc regs (cr19+) on all crtcs impervious:
// immutable and unreadable
//
// NV11 has independently lockable extended crtcs, except when tied
// nv04 cursor max dimensions of 32x32 (A1R5G5B5)
pub const NV04_CURSOR_SIZE: c_int = 32;
// limit nv10 cursors to 64x64 (ARGB8) (we could go to 64x255)
pub const NV10_CURSOR_SIZE: c_int = 64;
// on some nv40 (such as the "true" (in the NV_PFB_BOOT_0 sense) nv40,
// the gf6800gt) a hardware bug requires a write to PRAMDAC_CURSOR_POS
// for changes to the CRTC CURCTL regs to take effect, whether changing
// the pixmap location, or just showing/hiding the cursor
//
// Hilarious, the 24th bit doesn't want to stick to
// PCRTC_START...
//
// curctl1 |= MASK(NV_CIO_CRE_HCUR_ADDR1_ENABLE);
// curctl1 &= ~MASK(NV_CIO_CRE_HCUR_ADDR1_ENABLE);
// Alignment requirements taken from the Haiku driver
