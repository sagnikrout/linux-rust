//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/mgag200/mgag200_bmc.c
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

#[no_mangle]
pub unsafe extern "C" fn mgag200_bmc_stop_scanout(mdev: *mut mga_device) {
    void mgag200_bmc_stop_scanout(struct mga_device *mdev)
    {
    u8 tmp;
    int ret;
//
// 1 - The first step is to inform the BMC of an upcoming mode
// change. We are putting the misc<0> to output.
//
    WREG8(DAC_INDEX, MGA1064_GEN_IO_CTL);
    tmp = RREG8(DAC_DATA);
    tmp |= 0x10;
    WREG_DAC(MGA1064_GEN_IO_CTL, tmp);
// we are putting a 1 on the misc<0> line
    WREG8(DAC_INDEX, MGA1064_GEN_IO_DATA);
    tmp = RREG8(DAC_DATA);
    tmp |= 0x10;
    WREG_DAC(MGA1064_GEN_IO_DATA, tmp);
//
// 2- Second step to mask any further scan request. This is
// done by asserting the remfreqmsk bit (XSPAREREG<7>)
//
    WREG8(DAC_INDEX, MGA1064_SPAREREG);
    tmp = RREG8(DAC_DATA);
    tmp |= 0x80;
    WREG_DAC(MGA1064_SPAREREG, tmp);
//
// 3a- The third step is to verify if there is an active scan.
// We are waiting for a 0 on remhsyncsts (<XSPAREREG<0>).
//
    ret = read_poll_timeout(RREG_DAC, tmp, !(tmp & 0x1),
    1000, 300000, false,
    MGA1064_SPAREREG);
    if (ret == -ETIMEDOUT)
    return;
//
// 3b- This step occurs only if the remote BMC is actually
// scanning. We are waiting for the end of the frame which is
// a 1 on remvsyncsts (XSPAREREG<1>)
//
    (void)read_poll_timeout(RREG_DAC, tmp, (tmp & 0x2),
    1000, 300000, false,
    MGA1064_SPAREREG);
    }
#[no_mangle]
pub unsafe extern "C" fn mgag200_bmc_start_scanout(mdev: *mut mga_device) {
    void mgag200_bmc_start_scanout(struct mga_device *mdev)
    {
    u8 tmp;
// Assert rstlvl2
    WREG8(DAC_INDEX, MGA1064_REMHEADCTL2);
    tmp = RREG8(DAC_DATA);
    tmp |= 0x8;
    WREG8(DAC_DATA, tmp);
    udelay(10);
// Deassert rstlvl2
    tmp &= ~0x08;
    WREG8(DAC_INDEX, MGA1064_REMHEADCTL2);
    WREG8(DAC_DATA, tmp);
// Remove mask of scan request
    WREG8(DAC_INDEX, MGA1064_SPAREREG);
    tmp = RREG8(DAC_DATA);
    tmp &= ~0x80;
    WREG8(DAC_DATA, tmp);
// Put back a 0 on the misc<0> line
    WREG8(DAC_INDEX, MGA1064_GEN_IO_DATA);
    tmp = RREG8(DAC_DATA);
    tmp &= ~0x10;
    WREG_DAC(MGA1064_GEN_IO_DATA, tmp);
    }
