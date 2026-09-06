//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/nouveau/dispnv04/tvnv04.c
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
// Copyright (C) 2009 Francisco Jerez.
// All Rights Reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice (including the
// next paragraph) shall be included in all copies or substantial
// portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
// IN NO EVENT SHALL THE COPYRIGHT OWNER(S) AND/OR ITS SUPPLIERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//

    static struct nvkm_i2c_bus_probe nv04_tv_encoder_info[] = {
    {
    {
    I2C_BOARD_INFO("ch7006", 0x75),
    .platform_data = &(struct ch7006_encoder_params) {
    CH7006_FORMAT_RGB24m12I, CH7006_CLOCK_MASTER,
    0, 0, 0,
    CH7006_SYNC_SLAVE, CH7006_SYNC_SEPARATED,
    CH7006_POUT_3_3V, CH7006_ACTIVE_HSYNC
    }
    },
    0
    },
    { }
    };
#[no_mangle]
pub unsafe extern "C" fn nv04_tv_identify(dev: *mut drm_device, i2c_index: c_int) -> c_int {
    int nv04_tv_identify(struct drm_device *dev, int i2c_index)
    {
    struct nouveau_drm *drm = nouveau_drm(dev);
    struct nvkm_i2c *i2c = nvxx_i2c(drm);
    struct nvkm_i2c_bus *bus = nvkm_i2c_bus_find(i2c, i2c_index);
    if (bus) {
    return nvkm_i2c_bus_probe(bus, "TV encoder",
    nv04_tv_encoder_info,
    core::ptr::null_mut(), core::ptr::null_mut());
    }
    return -ENODEV;
    }

    (NV_PRAMDAC_PLL_COEFF_SELECT_TV_VSCLK1		\
    | NV_PRAMDAC_PLL_COEFF_SELECT_TV_PCLK1)

    (NV_PRAMDAC_PLL_COEFF_SELECT_TV_VSCLK2		\
    | NV_PRAMDAC_PLL_COEFF_SELECT_TV_PCLK2)
#[no_mangle]
unsafe extern "C" fn nv04_tv_dpms(encoder: *mut drm_encoder, mode: c_int) {
    static void nv04_tv_dpms(struct drm_encoder *encoder, int mode)
    {
    struct drm_device *dev = encoder.dev;
    struct nouveau_drm *drm = nouveau_drm(dev);
    struct nouveau_encoder *nv_encoder = nouveau_encoder(encoder);
    struct nv04_mode_state *state = &nv04_display(dev).mode_reg;
    uint8_t crtc1A;
    NV_DEBUG(drm, "Setting dpms mode %d on TV encoder (output %d)\n",
    mode, nv_encoder.dcb.index);
    state.pllsel &= ~(PLLSEL_TV_CRTC1_MASK | PLLSEL_TV_CRTC2_MASK);
    if (mode == DRM_MODE_DPMS_ON) {
    let mut head: c_int = nouveau_crtc(encoder.crtc).index;
    crtc1A = NVReadVgaCrtc(dev, head, NV_CIO_CRE_RPC1_INDEX);
    state.pllsel |= head ? PLLSEL_TV_CRTC2_MASK :
    PLLSEL_TV_CRTC1_MASK;
// Inhibit hsync
    crtc1A |= 0x80;
    NVWriteVgaCrtc(dev, head, NV_CIO_CRE_RPC1_INDEX, crtc1A);
    }
    NVWriteRAMDAC(dev, 0, NV_PRAMDAC_PLL_COEFF_SELECT, state.pllsel);
    get_encoder_i2c_funcs(encoder).dpms(encoder, mode);
    }
#[no_mangle]
unsafe extern "C" fn nv04_tv_bind(dev: *mut drm_device, head: c_int, bind: bool) {
    static void nv04_tv_bind(struct drm_device *dev, int head, bool bind)
    {
    struct nv04_crtc_reg *state = &nv04_display(dev).mode_reg.crtc_reg[head];
    state.tv_setup = 0;
    if (bind)
    state.CRTC[NV_CIO_CRE_49] |= 0x10;
    else
    state.CRTC[NV_CIO_CRE_49] &= ~0x10;
    NVWriteVgaCrtc(dev, head, NV_CIO_CRE_LCD__INDEX,
    state.CRTC[NV_CIO_CRE_LCD__INDEX]);
    NVWriteVgaCrtc(dev, head, NV_CIO_CRE_49,
    state.CRTC[NV_CIO_CRE_49]);
    NVWriteRAMDAC(dev, head, NV_PRAMDAC_TV_SETUP,
    state.tv_setup);
    }
#[no_mangle]
unsafe extern "C" fn nv04_tv_prepare(encoder: *mut drm_encoder) {
    static void nv04_tv_prepare(struct drm_encoder *encoder)
    {
    struct drm_device *dev = encoder.dev;
    let mut head: c_int = nouveau_crtc(encoder.crtc).index;
    const struct drm_encoder_helper_funcs *helper = encoder.helper_private;
    helper.dpms(encoder, DRM_MODE_DPMS_OFF);
    nv04_dfp_disable(dev, head);
    if (nv_two_heads(dev))
    nv04_tv_bind(dev, head ^ 1, false);
    nv04_tv_bind(dev, head, true);
    }
    static void nv04_tv_mode_set(struct drm_encoder *encoder,
    struct drm_display_mode *mode,
    struct drm_display_mode *adjusted_mode)
    {
    struct drm_device *dev = encoder.dev;
    struct nouveau_crtc *nv_crtc = nouveau_crtc(encoder.crtc);
    struct nv04_crtc_reg *regp = &nv04_display(dev).mode_reg.crtc_reg[nv_crtc.index];
    regp.tv_htotal = adjusted_mode.htotal;
    regp.tv_vtotal = adjusted_mode.vtotal;
// These delay the TV signals with respect to the VGA port,
// they might be useful if we ever allow a CRTC to drive
// multiple outputs.
//
    regp.tv_hskew = 1;
    regp.tv_hsync_delay = 1;
    regp.tv_hsync_delay2 = 64;
    regp.tv_vskew = 1;
    regp.tv_vsync_delay = 1;
    get_encoder_i2c_funcs(encoder).mode_set(encoder, mode, adjusted_mode);
    }
#[no_mangle]
unsafe extern "C" fn nv04_tv_commit(encoder: *mut drm_encoder) {
    static void nv04_tv_commit(struct drm_encoder *encoder)
    {
    struct nouveau_encoder *nv_encoder = nouveau_encoder(encoder);
    struct drm_device *dev = encoder.dev;
    struct nouveau_drm *drm = nouveau_drm(dev);
    struct nouveau_crtc *nv_crtc = nouveau_crtc(encoder.crtc);
    const struct drm_encoder_helper_funcs *helper = encoder.helper_private;
    helper.dpms(encoder, DRM_MODE_DPMS_ON);
    NV_DEBUG(drm, "Output %s is running on CRTC %d using output %c\n",
    nv04_encoder_get_connector(nv_encoder).base.name,
    nv_crtc.index, '@' + ffs(nv_encoder.dcb.or));
    }
#[no_mangle]
unsafe extern "C" fn nv04_tv_destroy(encoder: *mut drm_encoder) {
    static void nv04_tv_destroy(struct drm_encoder *encoder)
    {
    get_encoder_i2c_funcs(encoder).destroy(encoder);
    drm_encoder_cleanup(encoder);
    kfree(encoder.helper_private);
    kfree(nouveau_encoder(encoder));
    }
    static const struct drm_encoder_funcs nv04_tv_funcs = {
    .destroy = nv04_tv_destroy,
    };
    static const struct drm_encoder_helper_funcs nv04_tv_helper_funcs = {
    .dpms = nv04_tv_dpms,
    .mode_fixup = nouveau_i2c_encoder_mode_fixup,
    .prepare = nv04_tv_prepare,
    .commit = nv04_tv_commit,
    .mode_set = nv04_tv_mode_set,
    .detect = nouveau_i2c_encoder_detect,
    };
    int
    nv04_tv_create(struct drm_connector *connector, struct dcb_output *entry)
    {
    struct nouveau_encoder *nv_encoder;
    struct drm_encoder *encoder;
    struct drm_device *dev = connector.dev;
    struct nouveau_drm *drm = nouveau_drm(dev);
    struct nvkm_i2c *i2c = nvxx_i2c(drm);
    struct nvkm_i2c_bus *bus = nvkm_i2c_bus_find(i2c, entry.i2c_index);
    int type, ret;
// Ensure that we can talk to this encoder
    type = nv04_tv_identify(dev, entry.i2c_index);
    if (type < 0)
    return type;
// Allocate the necessary memory
    nv_encoder = kzalloc_obj(*nv_encoder);
    if (!nv_encoder)
    return -ENOMEM;
// Initialize the common members
    encoder = to_drm_encoder(nv_encoder);
    drm_encoder_init(dev, encoder, &nv04_tv_funcs, DRM_MODE_ENCODER_TVDAC,
    core::ptr::null_mut());
    drm_encoder_helper_add(encoder, &nv04_tv_helper_funcs);
    nv_encoder.enc_save = nouveau_i2c_encoder_save;
    nv_encoder.enc_restore = nouveau_i2c_encoder_restore;
    encoder.possible_crtcs = entry.heads;
    encoder.possible_clones = 0;
    nv_encoder.dcb = entry;
    nv_encoder.or = ffs(entry.or) - 1;
// Run the slave-specific initialization
    ret = nouveau_i2c_encoder_init(dev, to_encoder_i2c(encoder),
    &bus.i2c,
    &nv04_tv_encoder_info[type].dev);
    if (ret < 0)
    goto fail_cleanup;
// Attach it to the specified connector.
    get_encoder_i2c_funcs(encoder).create_resources(encoder, connector);
    drm_connector_attach_encoder(connector, encoder);
    return 0;
    fail_cleanup:
    drm_encoder_cleanup(encoder);
    kfree(nv_encoder);
    return ret;
    }
