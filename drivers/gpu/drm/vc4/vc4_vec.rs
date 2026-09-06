//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/vc4/vc4_vec.c
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
// Copyright (C) 2016 Broadcom
//
// DOC: VC4 SDTV module
//
// The VEC encoder generates PAL or NTSC composite video output.
//
// TV mode selection is done by an atomic property on the encoder,
// because a drm_mode_modeinfo is insufficient to distinguish between
// PAL and PAL-M or NTSC and NTSC-J.
//

// WSE Registers
pub const VEC_WSE_RESET: c_uint = 0xc0;
pub const VEC_WSE_CONTROL: c_uint = 0xc4;

pub const VEC_WSE_WSS_DATA: c_uint = 0xc8;
pub const VEC_WSE_VPS_DATA1: c_uint = 0xcc;
pub const VEC_WSE_VPS_CONTROL: c_uint = 0xd0;
// VEC Registers
pub const VEC_REVID: c_uint = 0x100;
pub const VEC_CONFIG0: c_uint = 0x104;

pub const VEC_CONFIG0_NTSC_STD: c_int = 0;
pub const VEC_CONFIG0_PAL_BDGHI_STD: c_int = 1;
pub const VEC_CONFIG0_PAL_M_STD: c_int = 2;
pub const VEC_CONFIG0_PAL_N_STD: c_int = 3;
pub const VEC_SCHPH: c_uint = 0x108;
pub const VEC_SOFT_RESET: c_uint = 0x10c;
pub const VEC_CLMP0_START: c_uint = 0x144;
pub const VEC_CLMP0_END: c_uint = 0x148;
//
// These set the color subcarrier frequency
// if VEC_CONFIG1_CUSTOM_FREQ is enabled.
//
// VEC_FREQ1_0 contains the most significant 16-bit half-word,
// VEC_FREQ3_2 contains the least significant 16-bit half-word.
// 0x80000000 seems to be equivalent to the pixel clock
// (which itself is the VEC clock divided by 8).
//
// Reference values (with the default pixel clock of 13.5 MHz):
//
// NTSC  (3579545.[45] Hz)     - 0x21F07C1F
// PAL   (4433618.75 Hz)       - 0x2A098ACB
// PAL-M (3575611.[888111] Hz) - 0x21E6EFE3
// PAL-N (3582056.25 Hz)       - 0x21F69446
//
// NOTE: For SECAM, it is used as the Dr center frequency,
// regardless of whether VEC_CONFIG1_CUSTOM_FREQ is enabled or not;
// that is specified as 4406250 Hz, which corresponds to 0x29C71C72.
//
pub const VEC_FREQ3_2: c_uint = 0x180;
pub const VEC_FREQ1_0: c_uint = 0x184;
pub const VEC_CONFIG1: c_uint = 0x188;

pub const VEC_CONFIG1_DITHER_TYPE_LFSR: c_int = 0;

pub const VEC_CONFIG2: c_uint = 0x18c;

pub const VEC_INTERRUPT_CONTROL: c_uint = 0x190;
pub const VEC_INTERRUPT_STATUS: c_uint = 0x194;
//
// Db center frequency for SECAM; the clock for this is the same as for
// VEC_FREQ3_2/VEC_FREQ1_0, which is used for Dr center frequency.
//
// This is specified as 4250000 Hz, which corresponds to 0x284BDA13.
// That is also the default value, so no need to set it explicitly.
//
pub const VEC_FCW_SECAM_B: c_uint = 0x198;
pub const VEC_SECAM_GAIN_VAL: c_uint = 0x19c;
pub const VEC_CONFIG3: c_uint = 0x1a0;

pub const VEC_STATUS0: c_uint = 0x200;
pub const VEC_MASK0: c_uint = 0x204;
pub const VEC_CFG: c_uint = 0x208;

pub const VEC_DAC_TEST: c_uint = 0x20c;
pub const VEC_DAC_CONFIG: c_uint = 0x210;

pub const VEC_DAC_MISC: c_uint = 0x214;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vc4_vec_variant {
    pub dac_config: u32,
}

// General VEC hardware state.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vc4_vec {
    pub encoder: vc4_encoder,
    pub connector: drm_connector,
    pub pdev: *mut platform_device,
    pub variant: *const vc4_vec_variant,
    pub regs: *mut void __iomem,
    pub clock: *mut clk,
    pub legacy_tv_mode_property: *mut drm_property,
    pub regset: debugfs_regset32,
}

    ({										\
    kunit_fail_current_test("Accessing a register in a unit test!\n");	\
    readl(vec.regs + (offset));						\
    })

    do {										\
    kunit_fail_current_test("Accessing a register in a unit test!\n");	\
    writel(val, vec.regs + (offset));					\
    } while (0)

    container_of_const(_encoder, struct vc4_vec, encoder.base)

    container_of_const(_connector, struct vc4_vec, connector)
    enum vc4_vec_tv_mode_id {
    VC4_VEC_TV_MODE_NTSC,
    VC4_VEC_TV_MODE_NTSC_J,
    VC4_VEC_TV_MODE_PAL,
    VC4_VEC_TV_MODE_PAL_M,
    VC4_VEC_TV_MODE_NTSC_443,
    VC4_VEC_TV_MODE_PAL_60,
    VC4_VEC_TV_MODE_PAL_N,
    VC4_VEC_TV_MODE_SECAM,
    VC4_VEC_TV_MODE_MONOCHROME,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vc4_vec_tv_mode {
    pub mode: c_uint,
    pub expected_htotal: u16,
    pub config0: u32,
    pub config1: u32,
    pub custom_freq: u32,
}

    static const struct debugfs_reg32 vec_regs[] = {
    VC4_REG32(VEC_WSE_CONTROL),
    VC4_REG32(VEC_WSE_WSS_DATA),
    VC4_REG32(VEC_WSE_VPS_DATA1),
    VC4_REG32(VEC_WSE_VPS_CONTROL),
    VC4_REG32(VEC_REVID),
    VC4_REG32(VEC_CONFIG0),
    VC4_REG32(VEC_SCHPH),
    VC4_REG32(VEC_CLMP0_START),
    VC4_REG32(VEC_CLMP0_END),
    VC4_REG32(VEC_FREQ3_2),
    VC4_REG32(VEC_FREQ1_0),
    VC4_REG32(VEC_CONFIG1),
    VC4_REG32(VEC_CONFIG2),
    VC4_REG32(VEC_INTERRUPT_CONTROL),
    VC4_REG32(VEC_INTERRUPT_STATUS),
    VC4_REG32(VEC_FCW_SECAM_B),
    VC4_REG32(VEC_SECAM_GAIN_VAL),
    VC4_REG32(VEC_CONFIG3),
    VC4_REG32(VEC_STATUS0),
    VC4_REG32(VEC_MASK0),
    VC4_REG32(VEC_CFG),
    VC4_REG32(VEC_DAC_TEST),
    VC4_REG32(VEC_DAC_CONFIG),
    VC4_REG32(VEC_DAC_MISC),
    };
    static const struct vc4_vec_tv_mode vc4_vec_tv_modes[] = {
    {
    .mode = DRM_MODE_TV_MODE_NTSC,
    .expected_htotal = 858,
    .config0 = VEC_CONFIG0_NTSC_STD | VEC_CONFIG0_PDEN,
    .config1 = VEC_CONFIG1_C_CVBS_CVBS,
    },
    {
    .mode = DRM_MODE_TV_MODE_NTSC_443,
    .expected_htotal = 858,
    .config0 = VEC_CONFIG0_NTSC_STD,
    .config1 = VEC_CONFIG1_C_CVBS_CVBS | VEC_CONFIG1_CUSTOM_FREQ,
    .custom_freq = 0x2a098acb,
    },
    {
    .mode = DRM_MODE_TV_MODE_NTSC_J,
    .expected_htotal = 858,
    .config0 = VEC_CONFIG0_NTSC_STD,
    .config1 = VEC_CONFIG1_C_CVBS_CVBS,
    },
    {
    .mode = DRM_MODE_TV_MODE_PAL,
    .expected_htotal = 864,
    .config0 = VEC_CONFIG0_PAL_BDGHI_STD,
    .config1 = VEC_CONFIG1_C_CVBS_CVBS,
    },
    {
// PAL-60
    .mode = DRM_MODE_TV_MODE_PAL,
    .expected_htotal = 858,
    .config0 = VEC_CONFIG0_PAL_M_STD,
    .config1 = VEC_CONFIG1_C_CVBS_CVBS | VEC_CONFIG1_CUSTOM_FREQ,
    .custom_freq = 0x2a098acb,
    },
    {
    .mode = DRM_MODE_TV_MODE_PAL_M,
    .expected_htotal = 858,
    .config0 = VEC_CONFIG0_PAL_M_STD,
    .config1 = VEC_CONFIG1_C_CVBS_CVBS,
    },
    {
    .mode = DRM_MODE_TV_MODE_PAL_N,
    .expected_htotal = 864,
    .config0 = VEC_CONFIG0_PAL_N_STD,
    .config1 = VEC_CONFIG1_C_CVBS_CVBS,
    },
    {
    .mode = DRM_MODE_TV_MODE_SECAM,
    .expected_htotal = 864,
    .config0 = VEC_CONFIG0_SECAM_STD,
    .config1 = VEC_CONFIG1_C_CVBS_CVBS,
    .custom_freq = 0x29c71c72,
    },
    {
// 50Hz mono
    .mode = DRM_MODE_TV_MODE_MONOCHROME,
    .expected_htotal = 864,
    .config0 = VEC_CONFIG0_PAL_BDGHI_STD | VEC_CONFIG0_BURDIS |
    VEC_CONFIG0_CHRDIS,
    .config1 = VEC_CONFIG1_C_CVBS_CVBS,
    },
    {
// 60Hz mono
    .mode = DRM_MODE_TV_MODE_MONOCHROME,
    .expected_htotal = 858,
    .config0 = VEC_CONFIG0_PAL_M_STD | VEC_CONFIG0_BURDIS |
    VEC_CONFIG0_CHRDIS,
    .config1 = VEC_CONFIG1_C_CVBS_CVBS,
    },
    };
    static inline const struct vc4_vec_tv_mode *
    vc4_vec_tv_mode_lookup(unsigned int mode, u16 htotal)
    {
    unsigned int i;
    for (i = 0; i < ARRAY_SIZE(vc4_vec_tv_modes); i++) {
    const struct vc4_vec_tv_mode *tv_mode = &vc4_vec_tv_modes[i];
    if (tv_mode.mode == mode &&
    tv_mode.expected_htotal == htotal)
    return tv_mode;
    }
    return core::ptr::null_mut();
    }
    static const struct drm_prop_enum_list legacy_tv_mode_names[] = {
    { VC4_VEC_TV_MODE_NTSC, "NTSC", },
    { VC4_VEC_TV_MODE_NTSC_443, "NTSC-443", },
    { VC4_VEC_TV_MODE_NTSC_J, "NTSC-J", },
    { VC4_VEC_TV_MODE_PAL, "PAL", },
    { VC4_VEC_TV_MODE_PAL_60, "PAL-60", },
    { VC4_VEC_TV_MODE_PAL_M, "PAL-M", },
    { VC4_VEC_TV_MODE_PAL_N, "PAL-N", },
    { VC4_VEC_TV_MODE_SECAM, "SECAM", },
    { VC4_VEC_TV_MODE_MONOCHROME, "Mono", },
    };
    static enum drm_connector_status
    vc4_vec_connector_detect(struct drm_connector *connector, bool force)
    {
    return connector_status_unknown;
    }
#[no_mangle]
unsafe extern "C" fn vc4_vec_connector_reset(connector: *mut drm_connector) {
    static void vc4_vec_connector_reset(struct drm_connector *connector)
    {
    drm_atomic_helper_connector_reset(connector);
    drm_atomic_helper_connector_tv_reset(connector);
    }
    static int
    vc4_vec_connector_set_property(struct drm_connector *connector,
    struct drm_connector_state *state,
    struct drm_property *property,
    uint64_t val)
    {
    struct vc4_vec *vec = connector_to_vc4_vec(connector);
    if (property != vec.legacy_tv_mode_property)
    return -EINVAL;
    switch (val) {
    case VC4_VEC_TV_MODE_NTSC:
    state.tv.mode = DRM_MODE_TV_MODE_NTSC;
    break;
    case VC4_VEC_TV_MODE_NTSC_443:
    state.tv.mode = DRM_MODE_TV_MODE_NTSC_443;
    break;
    case VC4_VEC_TV_MODE_NTSC_J:
    state.tv.mode = DRM_MODE_TV_MODE_NTSC_J;
    break;
    case VC4_VEC_TV_MODE_PAL:
    case VC4_VEC_TV_MODE_PAL_60:
    state.tv.mode = DRM_MODE_TV_MODE_PAL;
    break;
    case VC4_VEC_TV_MODE_PAL_M:
    state.tv.mode = DRM_MODE_TV_MODE_PAL_M;
    break;
    case VC4_VEC_TV_MODE_PAL_N:
    state.tv.mode = DRM_MODE_TV_MODE_PAL_N;
    break;
    case VC4_VEC_TV_MODE_SECAM:
    state.tv.mode = DRM_MODE_TV_MODE_SECAM;
    break;
    case VC4_VEC_TV_MODE_MONOCHROME:
    state.tv.mode = DRM_MODE_TV_MODE_MONOCHROME;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static int
    vc4_vec_connector_get_property(struct drm_connector *connector,
    const struct drm_connector_state *state,
    struct drm_property *property,
    uint64_t *val)
    {
    struct vc4_vec *vec = connector_to_vc4_vec(connector);
    if (property != vec.legacy_tv_mode_property)
    return -EINVAL;
    switch (state.tv.mode) {
    case DRM_MODE_TV_MODE_NTSC:
// val = VC4_VEC_TV_MODE_NTSC;
    break;
    case DRM_MODE_TV_MODE_NTSC_443:
// val = VC4_VEC_TV_MODE_NTSC_443;
    break;
    case DRM_MODE_TV_MODE_NTSC_J:
// val = VC4_VEC_TV_MODE_NTSC_J;
    break;
    case DRM_MODE_TV_MODE_PAL:
// val = VC4_VEC_TV_MODE_PAL;
    break;
    case DRM_MODE_TV_MODE_PAL_M:
// val = VC4_VEC_TV_MODE_PAL_M;
    break;
    case DRM_MODE_TV_MODE_PAL_N:
// val = VC4_VEC_TV_MODE_PAL_N;
    break;
    case DRM_MODE_TV_MODE_SECAM:
// val = VC4_VEC_TV_MODE_SECAM;
    break;
    case DRM_MODE_TV_MODE_MONOCHROME:
// val = VC4_VEC_TV_MODE_MONOCHROME;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static const struct drm_connector_funcs vc4_vec_connector_funcs = {
    .detect = vc4_vec_connector_detect,
    .fill_modes = drm_helper_probe_single_connector_modes,
    .reset = vc4_vec_connector_reset,
    .atomic_duplicate_state = drm_atomic_helper_connector_duplicate_state,
    .atomic_destroy_state = drm_atomic_helper_connector_destroy_state,
    .atomic_get_property = vc4_vec_connector_get_property,
    .atomic_set_property = vc4_vec_connector_set_property,
    };
    static const struct drm_connector_helper_funcs vc4_vec_connector_helper_funcs = {
    .atomic_check = drm_atomic_helper_connector_tv_check,
    .get_modes = drm_connector_helper_tv_get_modes,
    };
#[no_mangle]
unsafe extern "C" fn vc4_vec_connector_init(dev: *mut drm_device, vec: *mut vc4_vec) -> c_int {
    static int vc4_vec_connector_init(struct drm_device *dev, struct vc4_vec *vec)
    {
    struct drm_connector *connector = &vec.connector;
    struct drm_property *prop;
    int ret;
    connector.interlace_allowed = true;
    ret = drmm_connector_init(dev, connector, &vc4_vec_connector_funcs,
    DRM_MODE_CONNECTOR_Composite, core::ptr::null_mut());
    if (ret)
    return ret;
    drm_connector_helper_add(connector, &vc4_vec_connector_helper_funcs);
    drm_object_attach_property(&connector.base,
    dev.mode_config.tv_mode_property,
    DRM_MODE_TV_MODE_NTSC);
    prop = drm_property_create_enum(dev, 0, "mode",
    legacy_tv_mode_names,
    ARRAY_SIZE(legacy_tv_mode_names));
    if (!prop)
    return -ENOMEM;
    vec.legacy_tv_mode_property = prop;
    drm_object_attach_property(&connector.base, prop, VC4_VEC_TV_MODE_NTSC);
    drm_connector_attach_tv_margin_properties(connector);
    drm_connector_attach_encoder(connector, &vec.encoder.base);
    return 0;
    }
    static void vc4_vec_encoder_disable(struct drm_encoder *encoder,
    struct drm_atomic_commit *state)
    {
    struct drm_device *drm = encoder.dev;
    struct vc4_vec *vec = encoder_to_vc4_vec(encoder);
    int idx;
    if (!drm_dev_enter(drm, &idx))
    return;
    VEC_WRITE(VEC_CFG, 0);
    VEC_WRITE(VEC_DAC_MISC,
    VEC_DAC_MISC_VCD_PWRDN |
    VEC_DAC_MISC_BIAS_PWRDN |
    VEC_DAC_MISC_DAC_PWRDN |
    VEC_DAC_MISC_LDO_PWRDN);
    clk_disable_unprepare(vec.clock);
    pm_runtime_put(&vec.pdev.dev);
    drm_dev_exit(idx);
    }
    static void vc4_vec_encoder_enable(struct drm_encoder *encoder,
    struct drm_atomic_commit *state)
    {
    struct drm_device *drm = encoder.dev;
    struct vc4_vec *vec = encoder_to_vc4_vec(encoder);
    struct drm_connector *connector = &vec.connector;
    struct drm_connector_state *conn_state =
    drm_atomic_get_new_connector_state(state, connector);
    struct drm_display_mode *adjusted_mode =
    &encoder.crtc.state.adjusted_mode;
    const struct vc4_vec_tv_mode *tv_mode;
    int idx, ret;
    if (!drm_dev_enter(drm, &idx))
    return;
    tv_mode = vc4_vec_tv_mode_lookup(conn_state.tv.mode,
    adjusted_mode.htotal);
    if (!tv_mode)
    goto err_dev_exit;
    ret = pm_runtime_resume_and_get(&vec.pdev.dev);
    if (ret < 0) {
    drm_err(drm, "Failed to retain power domain: %d\n", ret);
    goto err_dev_exit;
    }
//
// We need to set the clock rate each time we enable the encoder
// because there's a chance we share the same parent with the HDMI
// clock, and both drivers are requesting different rates.
// The good news is, these 2 encoders cannot be enabled at the same
// time, thus preventing incompatible rate requests.
//
    ret = clk_set_rate(vec.clock, 108000000);
    if (ret) {
    drm_err(drm, "Failed to set clock rate: %d\n", ret);
    goto err_put_runtime_pm;
    }
    ret = clk_prepare_enable(vec.clock);
    if (ret) {
    drm_err(drm, "Failed to turn on core clock: %d\n", ret);
    goto err_put_runtime_pm;
    }
// Reset the different blocks
    VEC_WRITE(VEC_WSE_RESET, 1);
    VEC_WRITE(VEC_SOFT_RESET, 1);
// Disable the CGSM-A and WSE blocks
    VEC_WRITE(VEC_WSE_CONTROL, 0);
// Write config common to all modes.
//
// Color subcarrier phase: phase = 360 * SCHPH / 256.
// 0x28 <=> 39.375 deg.
//
    VEC_WRITE(VEC_SCHPH, 0x28);
//
// Reset to default values.
//
    VEC_WRITE(VEC_CLMP0_START, 0xac);
    VEC_WRITE(VEC_CLMP0_END, 0xec);
    VEC_WRITE(VEC_CONFIG2,
    VEC_CONFIG2_UV_DIG_DIS |
    VEC_CONFIG2_RGB_DIG_DIS |
    ((adjusted_mode.flags & DRM_MODE_FLAG_INTERLACE) ? 0 : VEC_CONFIG2_PROG_SCAN));
    VEC_WRITE(VEC_CONFIG3, VEC_CONFIG3_HORIZ_LEN_STD);
    VEC_WRITE(VEC_DAC_CONFIG, vec.variant.dac_config);
// Mask all interrupts.
    VEC_WRITE(VEC_MASK0, 0);
    VEC_WRITE(VEC_CONFIG0, tv_mode.config0);
    VEC_WRITE(VEC_CONFIG1, tv_mode.config1);
    if (tv_mode.custom_freq) {
    VEC_WRITE(VEC_FREQ3_2,
    (tv_mode.custom_freq >> 16) & 0xffff);
    VEC_WRITE(VEC_FREQ1_0,
    tv_mode.custom_freq & 0xffff);
    }
    VEC_WRITE(VEC_DAC_MISC,
    VEC_DAC_MISC_VID_ACT | VEC_DAC_MISC_DAC_RST_N);
    VEC_WRITE(VEC_CFG, VEC_CFG_VEC_EN);
    drm_dev_exit(idx);
    return;
    err_put_runtime_pm:
    pm_runtime_put(&vec.pdev.dev);
    err_dev_exit:
    drm_dev_exit(idx);
    }
    static int vc4_vec_encoder_atomic_check(struct drm_encoder *encoder,
    struct drm_crtc_state *crtc_state,
    struct drm_connector_state *conn_state)
    {
    const struct drm_display_mode *mode = &crtc_state.adjusted_mode;
    const struct vc4_vec_tv_mode *tv_mode;
    tv_mode = vc4_vec_tv_mode_lookup(conn_state.tv.mode, mode.htotal);
    if (!tv_mode)
    return -EINVAL;
    if (mode.crtc_hdisplay % 4)
    return -EINVAL;
    if (!(mode.crtc_hsync_end - mode.crtc_hsync_start))
    return -EINVAL;
    switch (mode.htotal) {
// NTSC
    case 858:
    if (mode.crtc_vtotal > 262)
    return -EINVAL;
    if (mode.crtc_vdisplay < 1 || mode.crtc_vdisplay > 253)
    return -EINVAL;
    if (!(mode.crtc_vsync_start - mode.crtc_vdisplay))
    return -EINVAL;
    if ((mode.crtc_vsync_end - mode.crtc_vsync_start) != 3)
    return -EINVAL;
    if ((mode.crtc_vtotal - mode.crtc_vsync_end) < 4)
    return -EINVAL;
    break;
// PAL/SECAM
    case 864:
    if (mode.crtc_vtotal > 312)
    return -EINVAL;
    if (mode.crtc_vdisplay < 1 || mode.crtc_vdisplay > 305)
    return -EINVAL;
    if (!(mode.crtc_vsync_start - mode.crtc_vdisplay))
    return -EINVAL;
    if ((mode.crtc_vsync_end - mode.crtc_vsync_start) != 3)
    return -EINVAL;
    if ((mode.crtc_vtotal - mode.crtc_vsync_end) < 2)
    return -EINVAL;
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static const struct drm_encoder_helper_funcs vc4_vec_encoder_helper_funcs = {
    .atomic_check = vc4_vec_encoder_atomic_check,
    .atomic_disable = vc4_vec_encoder_disable,
    .atomic_enable = vc4_vec_encoder_enable,
    };
#[no_mangle]
unsafe extern "C" fn vc4_vec_late_register(encoder: *mut drm_encoder) -> c_int {
    static int vc4_vec_late_register(struct drm_encoder *encoder)
    {
    struct drm_device *drm = encoder.dev;
    struct vc4_vec *vec = encoder_to_vc4_vec(encoder);
    vc4_debugfs_add_regset32(drm, "vec_regs", &vec.regset);
    return 0;
    }
    static const struct drm_encoder_funcs vc4_vec_encoder_funcs = {
    .late_register = vc4_vec_late_register,
    };
    static const struct vc4_vec_variant bcm2835_vec_variant = {
    .dac_config = VEC_DAC_CONFIG_DAC_CTRL(0xc) |
    VEC_DAC_CONFIG_DRIVER_CTRL(0xc) |
    VEC_DAC_CONFIG_LDO_BIAS_CTRL(0x46)
    };
    static const struct vc4_vec_variant bcm2711_vec_variant = {
    .dac_config = VEC_DAC_CONFIG_DAC_CTRL(0x0) |
    VEC_DAC_CONFIG_DRIVER_CTRL(0x80) |
    VEC_DAC_CONFIG_LDO_BIAS_CTRL(0x61)
    };
    static const struct of_device_id vc4_vec_dt_match[] = {
    { .compatible = "brcm,bcm2835-vec", .data = &bcm2835_vec_variant },
    { .compatible = "brcm,bcm2711-vec", .data = &bcm2711_vec_variant },
    { /* sentinel */ },
    };
#[no_mangle]
unsafe extern "C" fn vc4_vec_bind(dev: *mut device, master: *mut device, data: *mut c_void) -> c_int {
    static int vc4_vec_bind(struct device *dev, struct device *master, void *data)
    {
    struct platform_device *pdev = to_platform_device(dev);
    struct drm_device *drm = dev_get_drvdata(master);
    struct vc4_vec *vec;
    int ret;
    ret = drm_mode_create_tv_properties(drm,
    BIT(DRM_MODE_TV_MODE_NTSC) |
    BIT(DRM_MODE_TV_MODE_NTSC_443) |
    BIT(DRM_MODE_TV_MODE_NTSC_J) |
    BIT(DRM_MODE_TV_MODE_PAL) |
    BIT(DRM_MODE_TV_MODE_PAL_M) |
    BIT(DRM_MODE_TV_MODE_PAL_N) |
    BIT(DRM_MODE_TV_MODE_SECAM) |
    BIT(DRM_MODE_TV_MODE_MONOCHROME));
    if (ret)
    return ret;
    vec = drmm_kzalloc(drm, sizeof(*vec), GFP_KERNEL);
    if (!vec)
    return -ENOMEM;
    vec.encoder.type = VC4_ENCODER_TYPE_VEC;
    vec.pdev = pdev;
    vec.variant = (const struct vc4_vec_variant *)
    of_device_get_match_data(dev);
    vec.regs = vc4_ioremap_regs(pdev, 0);
    if (IS_ERR(vec.regs))
    return PTR_ERR(vec.regs);
    vec.regset.base = vec.regs;
    vec.regset.regs = vec_regs;
    vec.regset.nregs = ARRAY_SIZE(vec_regs);
    vec.clock = devm_clk_get(dev, core::ptr::null_mut());
    if (IS_ERR(vec.clock)) {
    ret = PTR_ERR(vec.clock);
    if (ret != -EPROBE_DEFER)
    drm_err(drm, "Failed to get clock: %d\n", ret);
    return ret;
    }
    ret = devm_pm_runtime_enable(dev);
    if (ret)
    return ret;
    ret = drmm_encoder_init(drm, &vec.encoder.base,
    &vc4_vec_encoder_funcs,
    DRM_MODE_ENCODER_TVDAC,
    core::ptr::null_mut());
    if (ret)
    return ret;
    drm_encoder_helper_add(&vec.encoder.base, &vc4_vec_encoder_helper_funcs);
    ret = vc4_vec_connector_init(drm, vec);
    if (ret)
    return ret;
    dev_set_drvdata(dev, vec);
    return 0;
    }
    static const struct component_ops vc4_vec_ops = {
    .bind   = vc4_vec_bind,
    };
#[no_mangle]
unsafe extern "C" fn vc4_vec_dev_probe(pdev: *mut platform_device) -> c_int {
    static int vc4_vec_dev_probe(struct platform_device *pdev)
    {
    return component_add(&pdev.dev, &vc4_vec_ops);
    }
#[no_mangle]
unsafe extern "C" fn vc4_vec_dev_remove(pdev: *mut platform_device) {
    static void vc4_vec_dev_remove(struct platform_device *pdev)
    {
    component_del(&pdev.dev, &vc4_vec_ops);
    }
    struct platform_driver vc4_vec_driver = {
    .probe = vc4_vec_dev_probe,
    .remove = vc4_vec_dev_remove,
    .driver = {
    .name = "vc4_vec",
    .of_match_table = vc4_vec_dt_match,
    },
    };
