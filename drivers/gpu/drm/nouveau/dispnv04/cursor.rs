//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/dispnv04/cursor.c
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


// SPDX-License-Identifier: MIT

    static void
    nv04_cursor_show(struct nouveau_crtc *nv_crtc, bool update)
    {
    nv_show_cursor(nv_crtc.base.dev, nv_crtc.index, true);
    }
    static void
    nv04_cursor_hide(struct nouveau_crtc *nv_crtc, bool update)
    {
    nv_show_cursor(nv_crtc.base.dev, nv_crtc.index, false);
    }
    static void
    nv04_cursor_set_pos(struct nouveau_crtc *nv_crtc, int x, int y)
    {
    nv_crtc.cursor_saved_x = x; nv_crtc.cursor_saved_y = y;
    NVWriteRAMDAC(nv_crtc.base.dev, nv_crtc.index,
    NV_PRAMDAC_CU_START_POS,
    XLATE(y, 0, NV_PRAMDAC_CU_START_POS_Y) |
    XLATE(x, 0, NV_PRAMDAC_CU_START_POS_X));
    }
    static void
    crtc_wr_cio_state(struct drm_crtc *crtc, struct nv04_crtc_reg *crtcstate, int index)
    {
    NVWriteVgaCrtc(crtc.dev, nouveau_crtc(crtc).index, index,
    crtcstate.CRTC[index]);
    }
    static void
    nv04_cursor_set_offset(struct nouveau_crtc *nv_crtc, uint32_t offset)
    {
    struct drm_device *dev = nv_crtc.base.dev;
    struct nouveau_drm *drm = nouveau_drm(dev);
    struct nv04_crtc_reg *regp = &nv04_display(dev).mode_reg.crtc_reg[nv_crtc.index];
    struct drm_crtc *crtc = &nv_crtc.base;
    regp.CRTC[NV_CIO_CRE_HCUR_ADDR0_INDEX] =
    MASK(NV_CIO_CRE_HCUR_ASI) |
    XLATE(offset, 17, NV_CIO_CRE_HCUR_ADDR0_ADR);
    regp.CRTC[NV_CIO_CRE_HCUR_ADDR1_INDEX] =
    XLATE(offset, 11, NV_CIO_CRE_HCUR_ADDR1_ADR);
    if (crtc.mode.flags & DRM_MODE_FLAG_DBLSCAN)
    regp.CRTC[NV_CIO_CRE_HCUR_ADDR1_INDEX] |=
    MASK(NV_CIO_CRE_HCUR_ADDR1_CUR_DBL);
    regp.CRTC[NV_CIO_CRE_HCUR_ADDR2_INDEX] = offset >> 24;
    crtc_wr_cio_state(crtc, regp, NV_CIO_CRE_HCUR_ADDR0_INDEX);
    crtc_wr_cio_state(crtc, regp, NV_CIO_CRE_HCUR_ADDR1_INDEX);
    crtc_wr_cio_state(crtc, regp, NV_CIO_CRE_HCUR_ADDR2_INDEX);
    if (drm.client.device.info.family == NV_DEVICE_INFO_V0_CURIE)
    nv_fix_nv40_hw_cursor(dev, nv_crtc.index);
    }
    int
    nv04_cursor_init(struct nouveau_crtc *crtc)
    {
    crtc.cursor.set_offset = nv04_cursor_set_offset;
    crtc.cursor.set_pos = nv04_cursor_set_pos;
    crtc.cursor.hide = nv04_cursor_hide;
    crtc.cursor.show = nv04_cursor_show;
    return 0;
    }
