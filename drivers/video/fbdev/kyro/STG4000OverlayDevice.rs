//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/kyro/STG4000OverlayDevice.c
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
// linux/drivers/video/kyro/STG4000OverlayDevice.c
//
// Copyright (C) 2000 Imagination Technologies Ltd
// Copyright (C) 2002 STMicroelectronics
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file COPYING in the main directory of this archive
// for more details.
//

// HW Defines
pub const STG4000_NO_SCALING: c_uint = 0x800;
pub const STG4000_NO_DECIMATION: c_uint = 0xFFFFFFFF;
// Primary surface
pub const STG4000_PRIM_NUM_PIX: c_int = 5;
pub const STG4000_PRIM_ALIGN: c_int = 4;
pub const STG4000_PRIM_ADDR_BITS: c_int = 20;
pub const STG4000_PRIM_MIN_WIDTH: c_int = 640;
pub const STG4000_PRIM_MAX_WIDTH: c_int = 1600;
pub const STG4000_PRIM_MIN_HEIGHT: c_int = 480;
pub const STG4000_PRIM_MAX_HEIGHT: c_int = 1200;
// Overlay surface
pub const STG4000_OVRL_NUM_PIX: c_int = 4;
pub const STG4000_OVRL_ALIGN: c_int = 2;
pub const STG4000_OVRL_ADDR_BITS: c_int = 20;
pub const STG4000_OVRL_NUM_MODES: c_int = 5;
pub const STG4000_OVRL_MIN_WIDTH: c_int = 0;
pub const STG4000_OVRL_MAX_WIDTH: c_int = 720;
pub const STG4000_OVRL_MIN_HEIGHT: c_int = 0;
pub const STG4000_OVRL_MAX_HEIGHT: c_int = 576;
// Decimation and Scaling
    static u32 adwDecim8[33] = {
    0xffffffff, 0xfffeffff, 0xffdffbff, 0xfefefeff, 0xfdf7efbf,
    0xfbdf7bdf, 0xf7bbddef, 0xeeeeeeef, 0xeeddbb77, 0xedb76db7,
    0xdb6db6db, 0xdb5b5b5b, 0xdab5ad6b, 0xd5ab55ab, 0xd555aaab,
    0xaaaaaaab, 0xaaaa5555, 0xaa952a55, 0xa94a5295, 0xa5252525,
    0xa4924925, 0x92491249, 0x91224489, 0x91111111, 0x90884211,
    0x88410821, 0x88102041, 0x81010101, 0x80800801, 0x80010001,
    0x80000001, 0x00000001, 0x00000000
    };
    typedef struct _OVRL_SRC_DEST {
// clipped on-screen pixel position of overlay
    u32 ulDstX1;
    u32 ulDstY1;
    u32 ulDstX2;
    u32 ulDstY2;
// clipped pixel pos of source data within buffer thses need to be 128 bit word aligned
    u32 ulSrcX1;
    u32 ulSrcY1;
    u32 ulSrcX2;
    u32 ulSrcY2;
// on-screen pixel position of overlay
    s32 lDstX1;
    s32 lDstY1;
    s32 lDstX2;
    s32 lDstY2;
    } OVRL_SRC_DEST;
    static u32 ovlWidth, ovlHeight, ovlStride;
    static int ovlLinear;
#[no_mangle]
pub unsafe extern "C" fn ResetOverlayRegisters(pSTGReg: *mut volatile STG4000REG __iomem) {
    void ResetOverlayRegisters(volatile STG4000REG __iomem *pSTGReg)
    {
    u32 tmp;
// Set Overlay address to default
    tmp = STG_READ_REG(DACOverlayAddr);
    CLEAR_BITS_FRM_TO(0, 20);
    CLEAR_BIT(31);
    STG_WRITE_REG(DACOverlayAddr, tmp);
// Set Overlay U address
    tmp = STG_READ_REG(DACOverlayUAddr);
    CLEAR_BITS_FRM_TO(0, 20);
    STG_WRITE_REG(DACOverlayUAddr, tmp);
// Set Overlay V address
    tmp = STG_READ_REG(DACOverlayVAddr);
    CLEAR_BITS_FRM_TO(0, 20);
    STG_WRITE_REG(DACOverlayVAddr, tmp);
// Set Overlay Size
    tmp = STG_READ_REG(DACOverlaySize);
    CLEAR_BITS_FRM_TO(0, 10);
    CLEAR_BITS_FRM_TO(12, 31);
    STG_WRITE_REG(DACOverlaySize, tmp);
// Set Overlay Vt Decimation
    tmp = STG4000_NO_DECIMATION;
    STG_WRITE_REG(DACOverlayVtDec, tmp);
// Set Overlay format to default value
    tmp = STG_READ_REG(DACPixelFormat);
    CLEAR_BITS_FRM_TO(4, 7);
    CLEAR_BITS_FRM_TO(16, 22);
    STG_WRITE_REG(DACPixelFormat, tmp);
// Set Vertical scaling to default
    tmp = STG_READ_REG(DACVerticalScal);
    CLEAR_BITS_FRM_TO(0, 11);
    CLEAR_BITS_FRM_TO(16, 22);
    tmp |= STG4000_NO_SCALING;	/* Set to no scaling */
    STG_WRITE_REG(DACVerticalScal, tmp);
// Set Horizontal Scaling to default
    tmp = STG_READ_REG(DACHorizontalScal);
    CLEAR_BITS_FRM_TO(0, 11);
    CLEAR_BITS_FRM_TO(16, 17);
    tmp |= STG4000_NO_SCALING;	/* Set to no scaling */
    STG_WRITE_REG(DACHorizontalScal, tmp);
// Set Blend mode to Alpha Blend
// ????? SG 08/11/2001 Surely this isn't the alpha blend mode,
    hopefully its overwrite
//
    tmp = STG_READ_REG(DACBlendCtrl);
    CLEAR_BITS_FRM_TO(0, 30);
    tmp = (GRAPHICS_MODE << 28);
    STG_WRITE_REG(DACBlendCtrl, tmp);
    }
    int CreateOverlaySurface(volatile STG4000REG __iomem *pSTGReg,
    u32 inWidth,
    u32 inHeight,
    int bLinear,
    u32 ulOverlayOffset,
    u32 * retStride, u32 * retUVStride)
    {
    u32 tmp;
    u32 ulStride;
    if (inWidth > STG4000_OVRL_MAX_WIDTH ||
    inHeight > STG4000_OVRL_MAX_HEIGHT) {
    return -EINVAL;
    }
// Stride in 16 byte words - 16Bpp
    if (bLinear) {
// Format is 16bits so num 16 byte words is width/8
    if ((inWidth & 0x7) == 0) {	/* inWidth % 8 */
    ulStride = (inWidth / 8);
    } else {
// Round up to next 16byte boundary
    ulStride = ((inWidth + 8) / 8);
    }
    } else {
// Y component is 8bits so num 16 byte words is width/16
    if ((inWidth & 0xf) == 0) {	/* inWidth % 16 */
    ulStride = (inWidth / 16);
    } else {
// Round up to next 16byte boundary
    ulStride = ((inWidth + 16) / 16);
    }
    }
// Set Overlay address and Format mode
    tmp = STG_READ_REG(DACOverlayAddr);
    CLEAR_BITS_FRM_TO(0, 20);
    if (bLinear) {
    CLEAR_BIT(31);	/* Overlay format to Linear */
    } else {
    tmp |= SET_BIT(31);	/* Overlay format to Planer */
    }
// Only bits 24:4 of the Overlay address
    tmp |= (ulOverlayOffset >> 4);
    STG_WRITE_REG(DACOverlayAddr, tmp);
    if (!bLinear) {
    u32 uvSize =
    (inWidth & 0x1) ? (inWidth + 1 / 2) : (inWidth / 2);
    u32 uvStride;
    u32 ulOffset;
// Y component is 8bits so num 32 byte words is width/32
    if ((uvSize & 0xf) == 0) {	/* inWidth % 16 */
    uvStride = (uvSize / 16);
    } else {
// Round up to next 32byte boundary
    uvStride = ((uvSize + 16) / 16);
    }
    ulOffset = ulOverlayOffset + (inHeight * (ulStride * 16));
// Align U,V data to 32byte boundary
    if ((ulOffset & 0x1f) != 0)
    ulOffset = (ulOffset + 32L) & 0xffffffE0L;
    tmp = STG_READ_REG(DACOverlayUAddr);
    CLEAR_BITS_FRM_TO(0, 20);
    tmp |= (ulOffset >> 4);
    STG_WRITE_REG(DACOverlayUAddr, tmp);
    ulOffset += (inHeight / 2) * (uvStride * 16);
// Align U,V data to 32byte boundary
    if ((ulOffset & 0x1f) != 0)
    ulOffset = (ulOffset + 32L) & 0xffffffE0L;
    tmp = STG_READ_REG(DACOverlayVAddr);
    CLEAR_BITS_FRM_TO(0, 20);
    tmp |= (ulOffset >> 4);
    STG_WRITE_REG(DACOverlayVAddr, tmp);
// retUVStride = uvStride * 16;
    }
// Set Overlay YUV pixel format
// Make sure that LUT not used - ??????
//
    tmp = STG_READ_REG(DACPixelFormat);
// Only support Planer or UYVY linear formats
    CLEAR_BITS_FRM_TO(4, 9);
    STG_WRITE_REG(DACPixelFormat, tmp);
    ovlWidth = inWidth;
    ovlHeight = inHeight;
    ovlStride = ulStride;
    ovlLinear = bLinear;
// retStride = ulStride << 4;	/* In bytes
    return 0;
    }
    int SetOverlayBlendMode(volatile STG4000REG __iomem *pSTGReg,
    OVRL_BLEND_MODE mode,
    u32 ulAlpha, u32 ulColorKey)
    {
    u32 tmp;
    tmp = STG_READ_REG(DACBlendCtrl);
    CLEAR_BITS_FRM_TO(28, 30);
    tmp |= (mode << 28);
    switch (mode) {
    case COLOR_KEY:
    CLEAR_BITS_FRM_TO(0, 23);
    tmp |= (ulColorKey & 0x00FFFFFF);
    break;
    case GLOBAL_ALPHA:
    CLEAR_BITS_FRM_TO(24, 27);
    tmp |= ((ulAlpha & 0xF) << 24);
    break;
    case CK_PIXEL_ALPHA:
    CLEAR_BITS_FRM_TO(0, 23);
    tmp |= (ulColorKey & 0x00FFFFFF);
    break;
    case CK_GLOBAL_ALPHA:
    CLEAR_BITS_FRM_TO(0, 23);
    tmp |= (ulColorKey & 0x00FFFFFF);
    CLEAR_BITS_FRM_TO(24, 27);
    tmp |= ((ulAlpha & 0xF) << 24);
    break;
    case GRAPHICS_MODE:
    case PER_PIXEL_ALPHA:
    break;
    default:
    return -EINVAL;
    }
    STG_WRITE_REG(DACBlendCtrl, tmp);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn EnableOverlayPlane(pSTGReg: *mut volatile STG4000REG __iomem) {
    void EnableOverlayPlane(volatile STG4000REG __iomem *pSTGReg)
    {
    u32 tmp;
// Enable Overlay
    tmp = STG_READ_REG(DACPixelFormat);
    tmp |= SET_BIT(7);
    STG_WRITE_REG(DACPixelFormat, tmp);
// Set video stream control
    tmp = STG_READ_REG(DACStreamCtrl);
    tmp |= SET_BIT(1);	/* video stream */
    STG_WRITE_REG(DACStreamCtrl, tmp);
    }
#[no_mangle]
unsafe extern "C" fn Overlap(ulBits: u32, ulPattern: u32) -> u32 {
    static u32 Overlap(u32 ulBits, u32 ulPattern)
    {
    let mut ulCount: u32 = 0;
    while (ulBits) {
    if (!(ulPattern & 1))
    ulCount++;
    ulBits--;
    ulPattern = ulPattern >> 1;
    }
    return ulCount;
    }
    int SetOverlayViewPort(volatile STG4000REG __iomem *pSTGReg,
    u32 left, u32 top,
    u32 right, u32 bottom)
    {
    OVRL_SRC_DEST srcDest;
    u32 ulSrcTop, ulSrcBottom;
    u32 ulSrc, ulDest;
    u32 ulFxScale, ulFxOffset;
    u32 ulHeight, ulWidth;
    u32 ulPattern;
    u32 ulDecimate, ulDecimated;
    u32 ulApplied;
    u32 ulDacXScale, ulDacYScale;
    u32 ulScale;
    u32 ulLeft, ulRight;
    u32 ulSrcLeft, ulSrcRight;
    u32 ulScaleLeft;
    u32 ulhDecim;
    u32 ulsVal;
    u32 ulVertDecFactor;
    int bResult;
    let mut ulClipOff: u32 = 0;
    let mut ulBits: u32 = 0;
    let mut ulsAdd: u32 = 0;
    u32 tmp, ulStride;
    u32 ulExcessPixels, ulClip, ulExtraLines;
    srcDest.ulSrcX1 = 0;
    srcDest.ulSrcY1 = 0;
    srcDest.ulSrcX2 = ovlWidth - 1;
    srcDest.ulSrcY2 = ovlHeight - 1;
    srcDest.ulDstX1 = left;
    srcDest.ulDstY1 = top;
    srcDest.ulDstX2 = right;
    srcDest.ulDstY2 = bottom;
    srcDest.lDstX1 = srcDest.ulDstX1;
    srcDest.lDstY1 = srcDest.ulDstY1;
    srcDest.lDstX2 = srcDest.ulDstX2;
    srcDest.lDstY2 = srcDest.ulDstY2;
// Vertical decimation/scaling
// Get Src Top and Bottom
    ulSrcTop = srcDest.ulSrcY1;
    ulSrcBottom = srcDest.ulSrcY2;
    ulSrc = ulSrcBottom - ulSrcTop;
    ulDest = srcDest.lDstY2 - srcDest.lDstY1;	/* on-screen overlay */
    if (ulSrc <= 1)
    return -EINVAL;
// First work out the position we are to display as offset from the
// source of the buffer
//
    ulFxScale = (ulDest << 11) / ulSrc;	/* fixed point scale factor */
    ulFxOffset = (srcDest.lDstY2 - srcDest.ulDstY2) << 11;
    ulSrcBottom = ulSrcBottom - (ulFxOffset / ulFxScale);
    ulSrc = ulSrcBottom - ulSrcTop;
    ulHeight = ulSrc;
    ulDest = srcDest.ulDstY2 - (srcDest.ulDstY1 - 1);
    ulPattern = adwDecim8[ulBits];
// At this point ulSrc represents the input decimator
    if (ulSrc > ulDest) {
    ulDecimate = ulSrc - ulDest;
    ulBits = 0;
    ulApplied = ulSrc / 32;
    while (((ulBits * ulApplied) +
    Overlap((ulSrc % 32),
    adwDecim8[ulBits])) < ulDecimate)
    ulBits++;
    ulPattern = adwDecim8[ulBits];
    ulDecimated =
    (ulBits * ulApplied) + Overlap((ulSrc % 32),
    ulPattern);
    ulSrc = ulSrc - ulDecimated;	/* the number number of lines that will go into the scaler */
    }
    if (ulBits && (ulBits != 32)) {
    ulVertDecFactor = (63 - ulBits) / (32 - ulBits);	/* vertical decimation factor scaled up to nearest integer */
    } else {
    ulVertDecFactor = 1;
    }
    ulDacYScale = ((ulSrc - 1) * 2048) / (ulDest + 1);
    tmp = STG_READ_REG(DACOverlayVtDec);	/* Decimation */
    CLEAR_BITS_FRM_TO(0, 31);
    tmp = ulPattern;
    STG_WRITE_REG(DACOverlayVtDec, tmp);
// Horizontal decimation/scaling
//
// Now we handle the horizontal case, this is a simplified version of
// the vertical case in that we decimate by factors of 2.  as we are
// working in words we should always be able to decimate by these
// factors.  as we always have to have a buffer which is aligned to a
// whole number of 128 bit words, we must align the left side to the
// lowest to the next lowest 128 bit boundary, and the right hand edge
// to the next largets boundary, (in a similar way to how we didi it in
// PMX1) as the left and right hand edges are aligned to these
// boundaries normally this only becomes an issue when we are chopping
// of one of the sides We shall work out vertical stuff first
//
    ulSrc = srcDest.ulSrcX2 - srcDest.ulSrcX1;
    ulDest = srcDest.lDstX2 - srcDest.lDstX1;

    ulLeft = srcDest.ulDstX1;
    ulRight = srcDest.ulDstX2;

    if (srcDest.ulDstX1 > 2) {
    ulLeft = srcDest.ulDstX1 + 2;
    ulRight = srcDest.ulDstX2 + 1;
    } else {
    ulLeft = srcDest.ulDstX1;
    ulRight = srcDest.ulDstX2 + 1;
    }

// first work out the position we are to display as offset from the source of the buffer
    bResult = 1;
    do {
    if (ulDest == 0)
    return -EINVAL;
// source pixels per dest pixel <<11
    ulFxScale = ((ulSrc - 1) << 11) / (ulDest);
// then number of destination pixels out we are
    ulFxOffset = ulFxScale * ((srcDest.ulDstX1 - srcDest.lDstX1) + ulClipOff);
    ulFxOffset >>= 11;
// this replaces the code which was making a decision as to use either ulFxOffset or ulSrcX1
    ulSrcLeft = srcDest.ulSrcX1 + ulFxOffset;
// then number of destination pixels out we are
    ulFxOffset = ulFxScale * (srcDest.lDstX2 - srcDest.ulDstX2);
    ulFxOffset >>= 11;
    ulSrcRight = srcDest.ulSrcX2 - ulFxOffset;
//
// we must align these to our 128 bit boundaries. we shall
// round down the pixel pos to the nearest 8 pixels.
//
    ulScaleLeft = ulSrcLeft;
// shift fxscale until it is in the range of the scaler
    ulhDecim = 0;
    ulScale = (((ulSrcRight - ulSrcLeft) - 1) << (11 - ulhDecim)) / (ulRight - ulLeft + 2);
    while (ulScale > 0x800) {
    ulhDecim++;
    ulScale = (((ulSrcRight - ulSrcLeft) - 1) << (11 - ulhDecim)) / (ulRight - ulLeft + 2);
    }
//
// to try and get the best values We first try and use
// src/dwdest for the scale factor, then we move onto src-1
//
// we want to check to see if we will need to clip data, if so
// then we should clip our source so that we don't need to
//
    if (!ovlLinear) {
    ulSrcLeft &= ~0x1f;
//
// we must align the right hand edge to the next 32
// pixel` boundary, must be on a 256 boundary so u, and
// v are 128 bit aligned
//
    ulSrcRight = (ulSrcRight + 0x1f) & ~0x1f;
    } else {
    ulSrcLeft &= ~0x7;
//
// we must align the right hand edge to the next
// 8pixel` boundary
//
    ulSrcRight = (ulSrcRight + 0x7) & ~0x7;
    }
// this is the input size line store needs to cope with
    ulWidth = ulSrcRight - ulSrcLeft;
//
// use unclipped value to work out scale factror this is the
// scale factor we want we shall now work out the horizonal
// decimation and scaling
//
    ulsVal = ((ulWidth / 8) >> ulhDecim);
    if ((ulWidth != (ulsVal << ulhDecim) * 8))
    ulsAdd = 1;
// input pixels to scaler;
    ulSrc = ulWidth >> ulhDecim;
    if (ulSrc <= 2)
    return -EINVAL;
    ulExcessPixels = ((((ulScaleLeft - ulSrcLeft)) << (11 - ulhDecim)) / ulScale);
    ulClip = (ulSrc << 11) / ulScale;
    ulClip -= (ulRight - ulLeft);
    ulClip += ulExcessPixels;
    if (ulClip)
    ulClip--;
// We may need to do more here if we really have a HW rev < 5
    } while (!bResult);
    ulExtraLines = (1 << ulhDecim) * ulVertDecFactor;
    ulExtraLines += 64;
    ulHeight += ulExtraLines;
    ulDacXScale = ulScale;
    tmp = STG_READ_REG(DACVerticalScal);
    CLEAR_BITS_FRM_TO(0, 11);
    CLEAR_BITS_FRM_TO(16, 22);	/* Vertical Scaling */
// Calculate new output line stride, this is always the number of 422
    words in the line buffer, so it doesn't matter if the
    mode is 420. Then set the vertical scale register.
//
    ulStride = (ulWidth >> (ulhDecim + 3)) + ulsAdd;
    tmp |= ((ulStride << 16) | (ulDacYScale));	/* DAC_LS_CTRL = stride */
    STG_WRITE_REG(DACVerticalScal, tmp);
// Now set up the overlay size using the modified width and height
    from decimate and scaling calculations
//
    tmp = STG_READ_REG(DACOverlaySize);
    CLEAR_BITS_FRM_TO(0, 10);
    CLEAR_BITS_FRM_TO(12, 31);
    if (ovlLinear) {
    tmp |=
    (ovlStride | ((ulHeight + 1) << 12) |
    (((ulWidth / 8) - 1) << 23));
    } else {
    tmp |=
    (ovlStride | ((ulHeight + 1) << 12) |
    (((ulWidth / 32) - 1) << 23));
    }
    STG_WRITE_REG(DACOverlaySize, tmp);
// Set Video Window Start
    tmp = ((ulLeft << 16)) | (srcDest.ulDstY1);
    STG_WRITE_REG(DACVidWinStart, tmp);
// Set Video Window End
    tmp = ((ulRight) << 16) | (srcDest.ulDstY2);
    STG_WRITE_REG(DACVidWinEnd, tmp);
// Finally set up the rest of the overlay regs in the order
    done in the IMG driver
//
    tmp = STG_READ_REG(DACPixelFormat);
    tmp = ((ulExcessPixels << 16) | tmp) & 0x7fffffff;
    STG_WRITE_REG(DACPixelFormat, tmp);
    tmp = STG_READ_REG(DACHorizontalScal);
    CLEAR_BITS_FRM_TO(0, 11);
    CLEAR_BITS_FRM_TO(16, 17);
    tmp |= ((ulhDecim << 16) | (ulDacXScale));
    STG_WRITE_REG(DACHorizontalScal, tmp);
    return 0;
    }
