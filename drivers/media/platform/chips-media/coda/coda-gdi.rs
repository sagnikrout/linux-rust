//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/chips-media/coda/coda-gdi.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Coda multi-standard codec IP
//
// Copyright (C) 2014 Philipp Zabel, Pengutronix
//

    (((XY2_##luma_sel) | (luma_bit)) << 8 | \
    (XY2_##chroma_sel) | (chroma_bit))
    static const u16 xy2ca_zero_map[16] = {
    XY2(ZERO, 0, ZERO, 0),
    XY2(ZERO, 0, ZERO, 0),
    XY2(ZERO, 0, ZERO, 0),
    XY2(ZERO, 0, ZERO, 0),
    XY2(ZERO, 0, ZERO, 0),
    XY2(ZERO, 0, ZERO, 0),
    XY2(ZERO, 0, ZERO, 0),
    XY2(ZERO, 0, ZERO, 0),
    XY2(ZERO, 0, ZERO, 0),
    XY2(ZERO, 0, ZERO, 0),
    XY2(ZERO, 0, ZERO, 0),
    XY2(ZERO, 0, ZERO, 0),
    XY2(ZERO, 0, ZERO, 0),
    XY2(ZERO, 0, ZERO, 0),
    XY2(ZERO, 0, ZERO, 0),
    XY2(ZERO, 0, ZERO, 0),
    };
    static const u16 xy2ca_tiled_map[16] = {
    XY2(Y,    0, Y,    0),
    XY2(Y,    1, Y,    1),
    XY2(Y,    2, Y,    2),
    XY2(Y,    3, X,    3),
    XY2(X,    3, ZERO, 0),
    XY2(ZERO, 0, ZERO, 0),
    XY2(ZERO, 0, ZERO, 0),
    XY2(ZERO, 0, ZERO, 0),
    XY2(ZERO, 0, ZERO, 0),
    XY2(ZERO, 0, ZERO, 0),
    XY2(ZERO, 0, ZERO, 0),
    XY2(ZERO, 0, ZERO, 0),
    XY2(ZERO, 0, ZERO, 0),
    XY2(ZERO, 0, ZERO, 0),
    XY2(ZERO, 0, ZERO, 0),
    XY2(ZERO, 0, ZERO, 0),
    };
//
// RA[15:0], CA[15:8] are hardwired to contain the 24-bit macroblock
// start offset (macroblock size is 16x16 for luma, 16x8 for chroma).
// Bits CA[4:0] are set using XY2CA above. BA[3:0] seems to be unused.
//

    (((RBC_##luma_sel) | (luma_bit)) << 6 | \
    (RBC_##chroma_sel) | (chroma_bit))
    static const u16 rbc2axi_tiled_map[32] = {
    RBC(ZERO, 0, ZERO, 0),
    RBC(ZERO, 0, ZERO, 0),
    RBC(ZERO, 0, ZERO, 0),
    RBC(CA,   0, CA,   0),
    RBC(CA,   1, CA,   1),
    RBC(CA,   2, CA,   2),
    RBC(CA,   3, CA,   3),
    RBC(CA,   4, CA,   8),
    RBC(CA,   8, CA,   9),
    RBC(CA,   9, CA,  10),
    RBC(CA,  10, CA,  11),
    RBC(CA,  11, CA,  12),
    RBC(CA,  12, CA,  13),
    RBC(CA,  13, CA,  14),
    RBC(CA,  14, CA,  15),
    RBC(CA,  15, RA,   0),
    RBC(RA,   0, RA,   1),
    RBC(RA,   1, RA,   2),
    RBC(RA,   2, RA,   3),
    RBC(RA,   3, RA,   4),
    RBC(RA,   4, RA,   5),
    RBC(RA,   5, RA,   6),
    RBC(RA,   6, RA,   7),
    RBC(RA,   7, RA,   8),
    RBC(RA,   8, RA,   9),
    RBC(RA,   9, RA,  10),
    RBC(RA,  10, RA,  11),
    RBC(RA,  11, RA,  12),
    RBC(RA,  12, RA,  13),
    RBC(RA,  13, RA,  14),
    RBC(RA,  14, RA,  15),
    RBC(RA,  15, ZERO, 0),
    };
#[no_mangle]
pub unsafe extern "C" fn coda_set_gdi_regs(ctx: *mut coda_ctx) {
    void coda_set_gdi_regs(struct coda_ctx *ctx)
    {
    struct coda_dev *dev = ctx.dev;
    const u16 *xy2ca_map;
    u32 xy2rbc_config;
    int i;
    switch (ctx.tiled_map_type) {
    case GDI_LINEAR_FRAME_MAP:
    default:
    xy2ca_map = xy2ca_zero_map;
    xy2rbc_config = 0;
    break;
    case GDI_TILED_FRAME_MB_RASTER_MAP:
    xy2ca_map = xy2ca_tiled_map;
    xy2rbc_config = CODA9_XY2RBC_TILED_MAP |
    CODA9_XY2RBC_CA_INC_HOR |
    (16 - 1) << 12 | (8 - 1) << 4;
    break;
    }
    for (i = 0; i < 16; i++)
    coda_write(dev, xy2ca_map[i],
    CODA9_GDI_XY2_CAS_0 + 4 * i);
    for (i = 0; i < 4; i++)
    coda_write(dev, XY2(ZERO, 0, ZERO, 0),
    CODA9_GDI_XY2_BA_0 + 4 * i);
    for (i = 0; i < 16; i++)
    coda_write(dev, XY2(ZERO, 0, ZERO, 0),
    CODA9_GDI_XY2_RAS_0 + 4 * i);
    coda_write(dev, xy2rbc_config, CODA9_GDI_XY2_RBC_CONFIG);
    if (xy2rbc_config) {
    for (i = 0; i < 32; i++)
    coda_write(dev, rbc2axi_tiled_map[i],
    CODA9_GDI_RBC2_AXI_0 + 4 * i);
    }
    }
