//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/kyro/STG4000VTG.c
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
// linux/drivers/video/kyro/STG4000VTG.c
//
// Copyright (C) 2002 STMicroelectronics
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file COPYING in the main directory of this archive
// for more details.
//

#[no_mangle]
pub unsafe extern "C" fn DisableVGA(pSTGReg: *mut volatile STG4000REG __iomem) {
    void DisableVGA(volatile STG4000REG __iomem *pSTGReg)
    {
    u32 tmp;
    let mut count: volatile u32 = 0, i;
// Reset the VGA registers
    tmp = STG_READ_REG(SoftwareReset);
    CLEAR_BIT(8);
    STG_WRITE_REG(SoftwareReset, tmp);
// Just for Delay
    for (i = 0; i < 1000; i++) {
    count++;
    }
// Pull-out the VGA registers from reset
    tmp = STG_READ_REG(SoftwareReset);
    tmp |= SET_BIT(8);
    STG_WRITE_REG(SoftwareReset, tmp);
    }
#[no_mangle]
pub unsafe extern "C" fn StopVTG(pSTGReg: *mut volatile STG4000REG __iomem) {
    void StopVTG(volatile STG4000REG __iomem *pSTGReg)
    {
    let mut tmp: u32 = 0;
// Stop Ver and Hor Sync Generator
    tmp = (STG_READ_REG(DACSyncCtrl)) | SET_BIT(0) | SET_BIT(2);
    CLEAR_BIT(31);
    STG_WRITE_REG(DACSyncCtrl, tmp);
    }
#[no_mangle]
pub unsafe extern "C" fn StartVTG(pSTGReg: *mut volatile STG4000REG __iomem) {
    void StartVTG(volatile STG4000REG __iomem *pSTGReg)
    {
    let mut tmp: u32 = 0;
// Start Ver and Hor Sync Generator
    tmp = ((STG_READ_REG(DACSyncCtrl)) | SET_BIT(31));
    CLEAR_BIT(0);
    CLEAR_BIT(2);
    STG_WRITE_REG(DACSyncCtrl, tmp);
    }
    void SetupVTG(volatile STG4000REG __iomem *pSTGReg,
    const struct kyrofb_info * pTiming)
    {
    let mut tmp: u32 = 0;
    let mut margins: u32 = 0;
    u32 ulBorder;
    let mut xRes: u32 = pTiming.XRES;
    let mut yRes: u32 = pTiming.YRES;
// Horizontal
    u32 HAddrTime, HRightBorder, HLeftBorder;
    u32 HBackPorcStrt, HFrontPorchStrt, HTotal,
    HLeftBorderStrt, HRightBorderStrt, HDisplayStrt;
// Vertical
    u32 VDisplayStrt, VBottomBorder, VTopBorder;
    u32 VBackPorchStrt, VTotal, VTopBorderStrt,
    VFrontPorchStrt, VBottomBorderStrt, VAddrTime;
// Need to calculate the right border
    if ((xRes == 640) && (yRes == 480)) {
    if ((pTiming.VFREQ == 60) || (pTiming.VFREQ == 72)) {
    margins = 8;
    }
    }
// Work out the Border
    ulBorder =
    (pTiming.HTot -
    (pTiming.HST + (pTiming.HBP - margins) + xRes +
    (pTiming.HFP - margins))) >> 1;
// Border the same for Vertical and Horizontal
    VBottomBorder = HLeftBorder = VTopBorder = HRightBorder = ulBorder;
// Get Timing values for Horizontal
    HAddrTime = xRes;
    HBackPorcStrt = pTiming.HST;
    HTotal = pTiming.HTot;
    HDisplayStrt =
    pTiming.HST + (pTiming.HBP - margins) + HLeftBorder;
    HLeftBorderStrt = HDisplayStrt - HLeftBorder;
    HFrontPorchStrt =
    pTiming.HST + (pTiming.HBP - margins) + HLeftBorder +
    HAddrTime + HRightBorder;
    HRightBorderStrt = HFrontPorchStrt - HRightBorder;
// Get Timing values for Vertical
    VAddrTime = yRes;
    VBackPorchStrt = pTiming.VST;
    VTotal = pTiming.VTot;
    VDisplayStrt =
    pTiming.VST + (pTiming.VBP - margins) + VTopBorder;
    VTopBorderStrt = VDisplayStrt - VTopBorder;
    VFrontPorchStrt =
    pTiming.VST + (pTiming.VBP - margins) + VTopBorder +
    VAddrTime + VBottomBorder;
    VBottomBorderStrt = VFrontPorchStrt - VBottomBorder;
// Set Hor Timing 1, 2, 3
    tmp = STG_READ_REG(DACHorTim1);
    CLEAR_BITS_FRM_TO(0, 11);
    CLEAR_BITS_FRM_TO(16, 27);
    tmp |= (HTotal) | (HBackPorcStrt << 16);
    STG_WRITE_REG(DACHorTim1, tmp);
    tmp = STG_READ_REG(DACHorTim2);
    CLEAR_BITS_FRM_TO(0, 11);
    CLEAR_BITS_FRM_TO(16, 27);
    tmp |= (HDisplayStrt << 16) | HLeftBorderStrt;
    STG_WRITE_REG(DACHorTim2, tmp);
    tmp = STG_READ_REG(DACHorTim3);
    CLEAR_BITS_FRM_TO(0, 11);
    CLEAR_BITS_FRM_TO(16, 27);
    tmp |= (HFrontPorchStrt << 16) | HRightBorderStrt;
    STG_WRITE_REG(DACHorTim3, tmp);
// Set Ver Timing 1, 2, 3
    tmp = STG_READ_REG(DACVerTim1);
    CLEAR_BITS_FRM_TO(0, 11);
    CLEAR_BITS_FRM_TO(16, 27);
    tmp |= (VBackPorchStrt << 16) | (VTotal);
    STG_WRITE_REG(DACVerTim1, tmp);
    tmp = STG_READ_REG(DACVerTim2);
    CLEAR_BITS_FRM_TO(0, 11);
    CLEAR_BITS_FRM_TO(16, 27);
    tmp |= (VDisplayStrt << 16) | VTopBorderStrt;
    STG_WRITE_REG(DACVerTim2, tmp);
    tmp = STG_READ_REG(DACVerTim3);
    CLEAR_BITS_FRM_TO(0, 11);
    CLEAR_BITS_FRM_TO(16, 27);
    tmp |= (VFrontPorchStrt << 16) | VBottomBorderStrt;
    STG_WRITE_REG(DACVerTim3, tmp);
// Set Verical and Horizontal Polarity
    tmp = STG_READ_REG(DACSyncCtrl) | SET_BIT(3) | SET_BIT(1);
    if ((pTiming.HSP > 0) && (pTiming.VSP < 0)) {	/* +hsync -vsync */
    tmp &= ~0x8;
    } else if ((pTiming.HSP < 0) && (pTiming.VSP > 0)) {	/* -hsync +vsync */
    tmp &= ~0x2;
    } else if ((pTiming.HSP < 0) && (pTiming.VSP < 0)) {	/* -hsync -vsync */
    tmp &= ~0xA;
    } else if ((pTiming.HSP > 0) && (pTiming.VSP > 0)) {	/* +hsync -vsync */
    tmp &= ~0x0;
    }
    STG_WRITE_REG(DACSyncCtrl, tmp);
    }
