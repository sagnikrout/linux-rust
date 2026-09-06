//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/mgag200/mgag200_g200eh3.c
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
// PIXPLLC
//
    static int mgag200_g200eh3_pixpllc_atomic_check(struct drm_crtc *crtc,
    struct drm_atomic_commit *new_state)
    {
    let mut vcomax: static unsigned int = 3000000;
    let mut vcomin: static unsigned int = 1500000;
    let mut pllreffreq: static unsigned int = 25000;
    struct drm_crtc_state *new_crtc_state = drm_atomic_get_new_crtc_state(new_state, crtc);
    struct mgag200_crtc_state *new_mgag200_crtc_state = to_mgag200_crtc_state(new_crtc_state);
    let mut clock: c_long = new_crtc_state.mode.clock;
    struct mgag200_pll_values *pixpllc = &new_mgag200_crtc_state.pixpllc;
    unsigned int delta, tmpdelta;
    unsigned int testp, testm, testn;
    unsigned int p, m, n, s;
    unsigned int computed;
    m = n = p = s = 0;
    delta = 0xffffffff;
    testp = 0;
    for (testm = 150; testm >= 6; testm--) {
    if (clock * testm > vcomax)
    continue;
    if (clock * testm < vcomin)
    continue;
    for (testn = 120; testn >= 60; testn--) {
    computed = (pllreffreq * testn) / testm;
    if (computed > clock)
    tmpdelta = computed - clock;
    else
    tmpdelta = clock - computed;
    if (tmpdelta < delta) {
    delta = tmpdelta;
    n = testn + 1;
    m = testm + 1;
    p = testp + 1;
    }
    if (delta == 0)
    break;
    }
    if (delta == 0)
    break;
    }
    pixpllc.m = m;
    pixpllc.n = n;
    pixpllc.p = p;
    pixpllc.s = s;
    return 0;
    }
//
// Mode-setting pipeline
//
    static const struct drm_plane_helper_funcs mgag200_g200eh3_primary_plane_helper_funcs = {
    MGAG200_PRIMARY_PLANE_HELPER_FUNCS,
    };
    static const struct drm_plane_funcs mgag200_g200eh3_primary_plane_funcs = {
    MGAG200_PRIMARY_PLANE_FUNCS,
    };
    static const struct drm_crtc_helper_funcs mgag200_g200eh3_crtc_helper_funcs = {
    MGAG200_CRTC_HELPER_FUNCS,
    };
    static const struct drm_crtc_funcs mgag200_g200eh3_crtc_funcs = {
    MGAG200_CRTC_FUNCS,
    };
#[no_mangle]
unsafe extern "C" fn mgag200_g200eh3_pipeline_init(mdev: *mut mga_device) -> c_int {
    static int mgag200_g200eh3_pipeline_init(struct mga_device *mdev)
    {
    struct drm_device *dev = &mdev.base;
    struct drm_plane *primary_plane = &mdev.primary_plane;
    struct drm_crtc *crtc = &mdev.crtc;
    int ret;
    ret = drm_universal_plane_init(dev, primary_plane, 0,
    &mgag200_g200eh3_primary_plane_funcs,
    mgag200_primary_plane_formats,
    mgag200_primary_plane_formats_size,
    mgag200_primary_plane_fmtmods,
    DRM_PLANE_TYPE_PRIMARY, core::ptr::null_mut());
    if (ret) {
    drm_err(dev, "drm_universal_plane_init() failed: %d\n", ret);
    return ret;
    }
    drm_plane_helper_add(primary_plane, &mgag200_g200eh3_primary_plane_helper_funcs);
    drm_plane_enable_fb_damage_clips(primary_plane);
    ret = drm_crtc_init_with_planes(dev, crtc, primary_plane, core::ptr::null_mut(),
    &mgag200_g200eh3_crtc_funcs, core::ptr::null_mut());
    if (ret) {
    drm_err(dev, "drm_crtc_init_with_planes() failed: %d\n", ret);
    return ret;
    }
    drm_crtc_helper_add(crtc, &mgag200_g200eh3_crtc_helper_funcs);
// FIXME: legacy gamma tables, but atomic gamma doesn't work without
    drm_mode_crtc_set_gamma_size(crtc, MGAG200_LUT_SIZE);
    drm_crtc_enable_color_mgmt(crtc, 0, false, MGAG200_LUT_SIZE);
    ret = mgag200_vga_bmc_output_init(mdev);
    if (ret)
    return ret;
    return 0;
    }
//
// DRM device
//
    static const struct mgag200_device_info mgag200_g200eh3_device_info =
    MGAG200_DEVICE_INFO_INIT(2048, 2048, 0, false, 1, 0, false);
    static const struct mgag200_device_funcs mgag200_g200eh3_device_funcs = {
    .pixpllc_atomic_check = mgag200_g200eh3_pixpllc_atomic_check,
    .pixpllc_atomic_update = mgag200_g200eh_pixpllc_atomic_update, // same as G200EH
    };
    struct mga_device *mgag200_g200eh3_device_create(struct pci_dev *pdev,
    const struct drm_driver *drv)
    {
    struct mga_device *mdev;
    struct drm_device *dev;
    resource_size_t vram_available;
    int ret;
    mdev = devm_drm_dev_alloc(&pdev.dev, drv, struct mga_device, base);
    if (IS_ERR(mdev))
    return mdev;
    dev = &mdev.base;
    pci_set_drvdata(pdev, dev);
    ret = mgag200_init_pci_options(pdev, 0x00000120, 0x0000b000);
    if (ret)
    return ERR_PTR(ret);
    ret = mgag200_device_preinit(mdev);
    if (ret)
    return ERR_PTR(ret);
    ret = mgag200_device_init(mdev, &mgag200_g200eh3_device_info,
    &mgag200_g200eh3_device_funcs);
    if (ret)
    return ERR_PTR(ret);
    mgag200_g200eh_init_registers(mdev); // same as G200EH
    vram_available = mgag200_device_probe_vram(mdev);
    ret = mgag200_mode_config_init(mdev, vram_available);
    if (ret)
    return ERR_PTR(ret);
    ret = mgag200_g200eh3_pipeline_init(mdev);
    if (ret)
    return ERR_PTR(ret);
    drm_mode_config_reset(dev);
    drm_kms_helper_poll_init(dev);
    return mdev;
    }
