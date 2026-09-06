//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/sti/sti_tvout.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (C) STMicroelectronics SA 2014
// Authors: Benjamin Gaignard <benjamin.gaignard@st.com>
// Vincent Abriou <vincent.abriou@st.com>
// for STMicroelectronics.
//

// glue registers
pub const TVO_CSC_MAIN_M0: c_uint = 0x000;
pub const TVO_CSC_MAIN_M1: c_uint = 0x004;
pub const TVO_CSC_MAIN_M2: c_uint = 0x008;
pub const TVO_CSC_MAIN_M3: c_uint = 0x00c;
pub const TVO_CSC_MAIN_M4: c_uint = 0x010;
pub const TVO_CSC_MAIN_M5: c_uint = 0x014;
pub const TVO_CSC_MAIN_M6: c_uint = 0x018;
pub const TVO_CSC_MAIN_M7: c_uint = 0x01c;
pub const TVO_MAIN_IN_VID_FORMAT: c_uint = 0x030;
pub const TVO_CSC_AUX_M0: c_uint = 0x100;
pub const TVO_CSC_AUX_M1: c_uint = 0x104;
pub const TVO_CSC_AUX_M2: c_uint = 0x108;
pub const TVO_CSC_AUX_M3: c_uint = 0x10c;
pub const TVO_CSC_AUX_M4: c_uint = 0x110;
pub const TVO_CSC_AUX_M5: c_uint = 0x114;
pub const TVO_CSC_AUX_M6: c_uint = 0x118;
pub const TVO_CSC_AUX_M7: c_uint = 0x11c;
pub const TVO_AUX_IN_VID_FORMAT: c_uint = 0x130;
pub const TVO_VIP_HDF: c_uint = 0x400;
pub const TVO_HD_SYNC_SEL: c_uint = 0x418;
pub const TVO_HD_DAC_CFG_OFF: c_uint = 0x420;
pub const TVO_VIP_HDMI: c_uint = 0x500;
pub const TVO_HDMI_FORCE_COLOR_0: c_uint = 0x504;
pub const TVO_HDMI_FORCE_COLOR_1: c_uint = 0x508;
pub const TVO_HDMI_CLIP_VALUE_B_CB: c_uint = 0x50c;
pub const TVO_HDMI_CLIP_VALUE_Y_G: c_uint = 0x510;
pub const TVO_HDMI_CLIP_VALUE_R_CR: c_uint = 0x514;
pub const TVO_HDMI_SYNC_SEL: c_uint = 0x518;
pub const TVO_HDMI_DFV_OBS: c_uint = 0x540;
pub const TVO_VIP_DVO: c_uint = 0x600;
pub const TVO_DVO_SYNC_SEL: c_uint = 0x618;
pub const TVO_DVO_CONFIG: c_uint = 0x620;

pub const TVO_VIP_REORDER_R_SHIFT: c_int = 24;
pub const TVO_VIP_REORDER_G_SHIFT: c_int = 20;
pub const TVO_VIP_REORDER_B_SHIFT: c_int = 16;
pub const TVO_VIP_REORDER_MASK: c_uint = 0x3;
pub const TVO_VIP_REORDER_Y_G_SEL: c_int = 0;
pub const TVO_VIP_REORDER_CB_B_SEL: c_int = 1;
pub const TVO_VIP_REORDER_CR_R_SEL: c_int = 2;
pub const TVO_VIP_CLIP_SHIFT: c_int = 8;
pub const TVO_VIP_CLIP_MASK: c_uint = 0x7;
pub const TVO_VIP_CLIP_DISABLED: c_int = 0;
pub const TVO_VIP_CLIP_EAV_SAV: c_int = 1;
pub const TVO_VIP_CLIP_LIMITED_RANGE_RGB_Y: c_int = 2;
pub const TVO_VIP_CLIP_LIMITED_RANGE_CB_CR: c_int = 3;
pub const TVO_VIP_CLIP_PROG_RANGE: c_int = 4;
pub const TVO_VIP_RND_SHIFT: c_int = 4;
pub const TVO_VIP_RND_MASK: c_uint = 0x3;
pub const TVO_VIP_RND_8BIT_ROUNDED: c_int = 0;
pub const TVO_VIP_RND_10BIT_ROUNDED: c_int = 1;
pub const TVO_VIP_RND_12BIT_ROUNDED: c_int = 2;
pub const TVO_VIP_SEL_INPUT_MASK: c_uint = 0xf;
pub const TVO_VIP_SEL_INPUT_MAIN: c_uint = 0x0;
pub const TVO_VIP_SEL_INPUT_AUX: c_uint = 0x8;
pub const TVO_VIP_SEL_INPUT_FORCE_COLOR: c_uint = 0xf;
pub const TVO_VIP_SEL_INPUT_BYPASS_MASK: c_uint = 0x1;
pub const TVO_VIP_SEL_INPUT_BYPASSED: c_int = 1;
pub const TVO_SYNC_MAIN_VTG_SET_REF: c_uint = 0x00;
pub const TVO_SYNC_AUX_VTG_SET_REF: c_uint = 0x10;
pub const TVO_SYNC_HD_DCS_SHIFT: c_int = 8;
pub const TVO_SYNC_DVO_PAD_HSYNC_SHIFT: c_int = 8;
pub const TVO_SYNC_DVO_PAD_VSYNC_SHIFT: c_int = 16;

pub const TVO_MIN_HD_HEIGHT: c_int = 720;
// enum listing the supported output data format
    enum sti_tvout_video_out_type {
    STI_TVOUT_VIDEO_OUT_RGB,
    STI_TVOUT_VIDEO_OUT_YUV,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sti_tvout {
    pub dev: *mut device,
    pub drm_dev: *mut drm_device,
    pub regs: *mut void __iomem,
    pub reset: *mut reset_control,
    pub hdmi: *mut drm_encoder,
    pub hda: *mut drm_encoder,
    pub dvo: *mut drm_encoder,
    pub debugfs_registered: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sti_tvout_encoder {
    pub encoder: drm_encoder,
    pub tvout: *mut sti_tvout,
}

    container_of(x, struct sti_tvout_encoder, encoder)

// preformatter conversion matrix
    static const u32 rgb_to_ycbcr_601[8] = {
    0xF927082E, 0x04C9FEAB, 0x01D30964, 0xFA95FD3D,
    0x0000082E, 0x00002000, 0x00002000, 0x00000000
    };
// 709 RGB to YCbCr
    static const u32 rgb_to_ycbcr_709[8] = {
    0xF891082F, 0x0367FF40, 0x01280B71, 0xF9B1FE20,
    0x0000082F, 0x00002000, 0x00002000, 0x00000000
    };
#[no_mangle]
unsafe extern "C" fn tvout_read(tvout: *mut sti_tvout, offset: c_int) -> u32 {
    static u32 tvout_read(struct sti_tvout *tvout, int offset)
    {
    return readl(tvout.regs + offset);
    }
#[no_mangle]
unsafe extern "C" fn tvout_write(tvout: *mut sti_tvout, val: u32, offset: c_int) {
    static void tvout_write(struct sti_tvout *tvout, u32 val, int offset)
    {
    writel(val, tvout.regs + offset);
    }
//
// tvout_vip_set_color_order - Set the clipping mode of a VIP
//
// @tvout: tvout structure
// @reg: register to set
// @cr_r: red chroma or red order
// @y_g: y or green order
// @cb_b: blue chroma or blue order
//
    static void tvout_vip_set_color_order(struct sti_tvout *tvout, int reg,
    u32 cr_r, u32 y_g, u32 cb_b)
    {
    let mut val: u32 = tvout_read(tvout, reg);
    val &= ~(TVO_VIP_REORDER_MASK << TVO_VIP_REORDER_R_SHIFT);
    val &= ~(TVO_VIP_REORDER_MASK << TVO_VIP_REORDER_G_SHIFT);
    val &= ~(TVO_VIP_REORDER_MASK << TVO_VIP_REORDER_B_SHIFT);
    val |= cr_r << TVO_VIP_REORDER_R_SHIFT;
    val |= y_g << TVO_VIP_REORDER_G_SHIFT;
    val |= cb_b << TVO_VIP_REORDER_B_SHIFT;
    tvout_write(tvout, val, reg);
    }
//
// tvout_vip_set_clip_mode - Set the clipping mode of a VIP
//
// @tvout: tvout structure
// @reg: register to set
// @range: clipping range
//
#[no_mangle]
unsafe extern "C" fn tvout_vip_set_clip_mode(tvout: *mut sti_tvout, reg: c_int, range: u32) {
    static void tvout_vip_set_clip_mode(struct sti_tvout *tvout, int reg, u32 range)
    {
    let mut val: u32 = tvout_read(tvout, reg);
    val &= ~(TVO_VIP_CLIP_MASK << TVO_VIP_CLIP_SHIFT);
    val |= range << TVO_VIP_CLIP_SHIFT;
    tvout_write(tvout, val, reg);
    }
//
// tvout_vip_set_rnd - Set the rounded value of a VIP
//
// @tvout: tvout structure
// @reg: register to set
// @rnd: rounded val per component
//
#[no_mangle]
unsafe extern "C" fn tvout_vip_set_rnd(tvout: *mut sti_tvout, reg: c_int, rnd: u32) {
    static void tvout_vip_set_rnd(struct sti_tvout *tvout, int reg, u32 rnd)
    {
    let mut val: u32 = tvout_read(tvout, reg);
    val &= ~(TVO_VIP_RND_MASK << TVO_VIP_RND_SHIFT);
    val |= rnd << TVO_VIP_RND_SHIFT;
    tvout_write(tvout, val, reg);
    }
//
// tvout_vip_set_sel_input - Select the VIP input
//
// @tvout: tvout structure
// @reg: register to set
// @main_path: main or auxiliary path
// @video_out: selected_input (main/aux + conv)
//
    static void tvout_vip_set_sel_input(struct sti_tvout *tvout,
    int reg,
    bool main_path,
    enum sti_tvout_video_out_type video_out)
    {
    u32 sel_input;
    let mut val: u32 = tvout_read(tvout, reg);
    if (main_path)
    sel_input = TVO_VIP_SEL_INPUT_MAIN;
    else
    sel_input = TVO_VIP_SEL_INPUT_AUX;
    switch (video_out) {
    case STI_TVOUT_VIDEO_OUT_RGB:
    sel_input |= TVO_VIP_SEL_INPUT_BYPASSED;
    break;
    case STI_TVOUT_VIDEO_OUT_YUV:
    sel_input &= ~TVO_VIP_SEL_INPUT_BYPASSED;
    break;
    }
// on stih407 chip the sel_input bypass mode logic is inverted
    sel_input = sel_input ^ TVO_VIP_SEL_INPUT_BYPASS_MASK;
    val &= ~TVO_VIP_SEL_INPUT_MASK;
    val |= sel_input;
    tvout_write(tvout, val, reg);
    }
//
// tvout_vip_set_in_vid_fmt - Select the input video signed or unsigned
//
// @tvout: tvout structure
// @reg: register to set
// @in_vid_fmt: used video input format
//
    static void tvout_vip_set_in_vid_fmt(struct sti_tvout *tvout,
    int reg, u32 in_vid_fmt)
    {
    let mut val: u32 = tvout_read(tvout, reg);
    val &= ~TVO_IN_FMT_SIGNED;
    val |= in_vid_fmt;
    tvout_write(tvout, val, reg);
    }
//
// tvout_preformatter_set_matrix - Set preformatter matrix
//
// @tvout: tvout structure
// @mode: display mode structure
//
    static void tvout_preformatter_set_matrix(struct sti_tvout *tvout,
    struct drm_display_mode *mode)
    {
    unsigned int i;
    const u32 *pf_matrix;
    if (mode.vdisplay >= TVO_MIN_HD_HEIGHT)
    pf_matrix = rgb_to_ycbcr_709;
    else
    pf_matrix = rgb_to_ycbcr_601;
    for (i = 0; i < 8; i++) {
    tvout_write(tvout, *(pf_matrix + i),
    TVO_CSC_MAIN_M0 + (i * 4));
    tvout_write(tvout, *(pf_matrix + i),
    TVO_CSC_AUX_M0 + (i * 4));
    }
    }
//
// tvout_dvo_start - Start VIP block for DVO output
//
// @tvout: pointer on tvout structure
// @main_path: true if main path has to be used in the vip configuration
// else aux path is used.
//
#[no_mangle]
unsafe extern "C" fn tvout_dvo_start(tvout: *mut sti_tvout, main_path: bool) {
    static void tvout_dvo_start(struct sti_tvout *tvout, bool main_path)
    {
    u32 tvo_in_vid_format;
    int val, tmp;
    dev_dbg(tvout.dev, "%s\n", __func__);
    if (main_path) {
    DRM_DEBUG_DRIVER("main vip for DVO\n");
// Select the input sync for dvo
    tmp = TVO_SYNC_MAIN_VTG_SET_REF | VTG_SYNC_ID_DVO;
    val  = tmp << TVO_SYNC_DVO_PAD_VSYNC_SHIFT;
    val |= tmp << TVO_SYNC_DVO_PAD_HSYNC_SHIFT;
    val |= tmp;
    tvout_write(tvout, val, TVO_DVO_SYNC_SEL);
    tvo_in_vid_format = TVO_MAIN_IN_VID_FORMAT;
    } else {
    DRM_DEBUG_DRIVER("aux vip for DVO\n");
// Select the input sync for dvo
    tmp = TVO_SYNC_AUX_VTG_SET_REF | VTG_SYNC_ID_DVO;
    val  = tmp << TVO_SYNC_DVO_PAD_VSYNC_SHIFT;
    val |= tmp << TVO_SYNC_DVO_PAD_HSYNC_SHIFT;
    val |= tmp;
    tvout_write(tvout, val, TVO_DVO_SYNC_SEL);
    tvo_in_vid_format = TVO_AUX_IN_VID_FORMAT;
    }
// Set color channel order
    tvout_vip_set_color_order(tvout, TVO_VIP_DVO,
    TVO_VIP_REORDER_CR_R_SEL,
    TVO_VIP_REORDER_Y_G_SEL,
    TVO_VIP_REORDER_CB_B_SEL);
// Set clipping mode
    tvout_vip_set_clip_mode(tvout, TVO_VIP_DVO, TVO_VIP_CLIP_DISABLED);
// Set round mode (rounded to 8-bit per component)
    tvout_vip_set_rnd(tvout, TVO_VIP_DVO, TVO_VIP_RND_8BIT_ROUNDED);
// Set input video format
    tvout_vip_set_in_vid_fmt(tvout, tvo_in_vid_format, TVO_IN_FMT_SIGNED);
// Input selection
    tvout_vip_set_sel_input(tvout, TVO_VIP_DVO, main_path,
    STI_TVOUT_VIDEO_OUT_RGB);
    }
//
// tvout_hdmi_start - Start VIP block for HDMI output
//
// @tvout: pointer on tvout structure
// @main_path: true if main path has to be used in the vip configuration
// else aux path is used.
//
#[no_mangle]
unsafe extern "C" fn tvout_hdmi_start(tvout: *mut sti_tvout, main_path: bool) {
    static void tvout_hdmi_start(struct sti_tvout *tvout, bool main_path)
    {
    u32 tvo_in_vid_format;
    dev_dbg(tvout.dev, "%s\n", __func__);
    if (main_path) {
    DRM_DEBUG_DRIVER("main vip for hdmi\n");
// select the input sync for hdmi
    tvout_write(tvout,
    TVO_SYNC_MAIN_VTG_SET_REF | VTG_SYNC_ID_HDMI,
    TVO_HDMI_SYNC_SEL);
    tvo_in_vid_format = TVO_MAIN_IN_VID_FORMAT;
    } else {
    DRM_DEBUG_DRIVER("aux vip for hdmi\n");
// select the input sync for hdmi
    tvout_write(tvout,
    TVO_SYNC_AUX_VTG_SET_REF | VTG_SYNC_ID_HDMI,
    TVO_HDMI_SYNC_SEL);
    tvo_in_vid_format = TVO_AUX_IN_VID_FORMAT;
    }
// set color channel order
    tvout_vip_set_color_order(tvout, TVO_VIP_HDMI,
    TVO_VIP_REORDER_CR_R_SEL,
    TVO_VIP_REORDER_Y_G_SEL,
    TVO_VIP_REORDER_CB_B_SEL);
// set clipping mode
    tvout_vip_set_clip_mode(tvout, TVO_VIP_HDMI, TVO_VIP_CLIP_DISABLED);
// set round mode (rounded to 8-bit per component)
    tvout_vip_set_rnd(tvout, TVO_VIP_HDMI, TVO_VIP_RND_8BIT_ROUNDED);
// set input video format
    tvout_vip_set_in_vid_fmt(tvout, tvo_in_vid_format, TVO_IN_FMT_SIGNED);
// input selection
    tvout_vip_set_sel_input(tvout, TVO_VIP_HDMI, main_path,
    STI_TVOUT_VIDEO_OUT_RGB);
    }
//
// tvout_hda_start - Start HDF VIP and HD DAC
//
// @tvout: pointer on tvout structure
// @main_path: true if main path has to be used in the vip configuration
// else aux path is used.
//
#[no_mangle]
unsafe extern "C" fn tvout_hda_start(tvout: *mut sti_tvout, main_path: bool) {
    static void tvout_hda_start(struct sti_tvout *tvout, bool main_path)
    {
    u32 tvo_in_vid_format;
    int val;
    dev_dbg(tvout.dev, "%s\n", __func__);
    if (main_path) {
    DRM_DEBUG_DRIVER("main vip for HDF\n");
// Select the input sync for HD analog and HD DCS
    val  = TVO_SYNC_MAIN_VTG_SET_REF | VTG_SYNC_ID_HDDCS;
    val  = val << TVO_SYNC_HD_DCS_SHIFT;
    val |= TVO_SYNC_MAIN_VTG_SET_REF | VTG_SYNC_ID_HDF;
    tvout_write(tvout, val, TVO_HD_SYNC_SEL);
    tvo_in_vid_format = TVO_MAIN_IN_VID_FORMAT;
    } else {
    DRM_DEBUG_DRIVER("aux vip for HDF\n");
// Select the input sync for HD analog and HD DCS
    val  = TVO_SYNC_AUX_VTG_SET_REF | VTG_SYNC_ID_HDDCS;
    val  = val << TVO_SYNC_HD_DCS_SHIFT;
    val |= TVO_SYNC_AUX_VTG_SET_REF | VTG_SYNC_ID_HDF;
    tvout_write(tvout, val, TVO_HD_SYNC_SEL);
    tvo_in_vid_format = TVO_AUX_IN_VID_FORMAT;
    }
// set color channel order
    tvout_vip_set_color_order(tvout, TVO_VIP_HDF,
    TVO_VIP_REORDER_CR_R_SEL,
    TVO_VIP_REORDER_Y_G_SEL,
    TVO_VIP_REORDER_CB_B_SEL);
// set clipping mode
    tvout_vip_set_clip_mode(tvout, TVO_VIP_HDF, TVO_VIP_CLIP_DISABLED);
// set round mode (rounded to 10-bit per component)
    tvout_vip_set_rnd(tvout, TVO_VIP_HDF, TVO_VIP_RND_10BIT_ROUNDED);
// Set input video format
    tvout_vip_set_in_vid_fmt(tvout, tvo_in_vid_format, TVO_IN_FMT_SIGNED);
// Input selection
    tvout_vip_set_sel_input(tvout, TVO_VIP_HDF, main_path,
    STI_TVOUT_VIDEO_OUT_YUV);
// power up HD DAC
    tvout_write(tvout, 0, TVO_HD_DAC_CFG_OFF);
    }

    readl(tvout.regs + reg))
#[no_mangle]
unsafe extern "C" fn tvout_dbg_vip(s: *mut seq_file, val: c_int) {
    static void tvout_dbg_vip(struct seq_file *s, int val)
    {
    int r, g, b, tmp, mask;
    char *const reorder[] = {"Y_G", "Cb_B", "Cr_R"};
    char *const clipping[] = {"No", "EAV/SAV", "Limited range RGB/Y",
    "Limited range Cb/Cr", "decided by register"};
    char *const round[] = {"8-bit", "10-bit", "12-bit"};
    char *const input_sel[] = {"Main (color matrix enabled)",
    "Main (color matrix by-passed)",
    "", "", "", "", "", "",
    "Aux (color matrix enabled)",
    "Aux (color matrix by-passed)",
    "", "", "", "", "", "Force value"};
    seq_putc(s, '\t');
    mask = TVO_VIP_REORDER_MASK << TVO_VIP_REORDER_R_SHIFT;
    r = (val & mask) >> TVO_VIP_REORDER_R_SHIFT;
    mask = TVO_VIP_REORDER_MASK << TVO_VIP_REORDER_G_SHIFT;
    g = (val & mask) >> TVO_VIP_REORDER_G_SHIFT;
    mask = TVO_VIP_REORDER_MASK << TVO_VIP_REORDER_B_SHIFT;
    b = (val & mask) >> TVO_VIP_REORDER_B_SHIFT;
    seq_printf(s, "%-24s %s.%s %s.%s %s.%s\n", "Reorder:",
    reorder[r], reorder[TVO_VIP_REORDER_CR_R_SEL],
    reorder[g], reorder[TVO_VIP_REORDER_Y_G_SEL],
    reorder[b], reorder[TVO_VIP_REORDER_CB_B_SEL]);
    seq_puts(s, "\t\t\t\t\t");
    mask = TVO_VIP_CLIP_MASK << TVO_VIP_CLIP_SHIFT;
    tmp = (val & mask) >> TVO_VIP_CLIP_SHIFT;
    seq_printf(s, "%-24s %s\n", "Clipping:", clipping[tmp]);
    seq_puts(s, "\t\t\t\t\t");
    mask = TVO_VIP_RND_MASK << TVO_VIP_RND_SHIFT;
    tmp = (val & mask) >> TVO_VIP_RND_SHIFT;
    seq_printf(s, "%-24s input data rounded to %s per component\n",
    "Round:", round[tmp]);
    seq_puts(s, "\t\t\t\t\t");
    tmp = (val & TVO_VIP_SEL_INPUT_MASK);
    seq_printf(s, "%-24s %s", "Input selection:", input_sel[tmp]);
    }
#[no_mangle]
unsafe extern "C" fn tvout_dbg_hd_dac_cfg(s: *mut seq_file, val: c_int) {
    static void tvout_dbg_hd_dac_cfg(struct seq_file *s, int val)
    {
    seq_printf(s, "\t%-24s %s", "HD DAC:",
    val & 1 ? "disabled" : "enabled");
    }
#[no_mangle]
unsafe extern "C" fn tvout_dbg_show(s: *mut seq_file, data: *mut c_void) -> c_int {
    static int tvout_dbg_show(struct seq_file *s, void *data)
    {
    struct drm_info_node *node = s.private;
    struct sti_tvout *tvout = (struct sti_tvout *)node.info_ent.data;
    struct drm_crtc *crtc;
    seq_printf(s, "TVOUT: (vaddr = 0x%p)", tvout.regs);
    seq_puts(s, "\n\n  HDMI encoder: ");
    crtc = tvout.hdmi.crtc;
    if (crtc) {
    seq_printf(s, "connected to %s path",
    sti_crtc_is_main(crtc) ? "main" : "aux");
    DBGFS_DUMP(TVO_HDMI_SYNC_SEL);
    DBGFS_DUMP(TVO_VIP_HDMI);
    tvout_dbg_vip(s, readl(tvout.regs + TVO_VIP_HDMI));
    } else {
    seq_puts(s, "disabled");
    }
    seq_puts(s, "\n\n  DVO encoder: ");
    crtc = tvout.dvo.crtc;
    if (crtc) {
    seq_printf(s, "connected to %s path",
    sti_crtc_is_main(crtc) ? "main" : "aux");
    DBGFS_DUMP(TVO_DVO_SYNC_SEL);
    DBGFS_DUMP(TVO_DVO_CONFIG);
    DBGFS_DUMP(TVO_VIP_DVO);
    tvout_dbg_vip(s, readl(tvout.regs + TVO_VIP_DVO));
    } else {
    seq_puts(s, "disabled");
    }
    seq_puts(s, "\n\n  HDA encoder: ");
    crtc = tvout.hda.crtc;
    if (crtc) {
    seq_printf(s, "connected to %s path",
    sti_crtc_is_main(crtc) ? "main" : "aux");
    DBGFS_DUMP(TVO_HD_SYNC_SEL);
    DBGFS_DUMP(TVO_HD_DAC_CFG_OFF);
    tvout_dbg_hd_dac_cfg(s,
    readl(tvout.regs + TVO_HD_DAC_CFG_OFF));
    DBGFS_DUMP(TVO_VIP_HDF);
    tvout_dbg_vip(s, readl(tvout.regs + TVO_VIP_HDF));
    } else {
    seq_puts(s, "disabled");
    }
    seq_puts(s, "\n\n  main path configuration");
    DBGFS_DUMP(TVO_CSC_MAIN_M0);
    DBGFS_DUMP(TVO_CSC_MAIN_M1);
    DBGFS_DUMP(TVO_CSC_MAIN_M2);
    DBGFS_DUMP(TVO_CSC_MAIN_M3);
    DBGFS_DUMP(TVO_CSC_MAIN_M4);
    DBGFS_DUMP(TVO_CSC_MAIN_M5);
    DBGFS_DUMP(TVO_CSC_MAIN_M6);
    DBGFS_DUMP(TVO_CSC_MAIN_M7);
    DBGFS_DUMP(TVO_MAIN_IN_VID_FORMAT);
    seq_puts(s, "\n\n  auxiliary path configuration");
    DBGFS_DUMP(TVO_CSC_AUX_M0);
    DBGFS_DUMP(TVO_CSC_AUX_M2);
    DBGFS_DUMP(TVO_CSC_AUX_M3);
    DBGFS_DUMP(TVO_CSC_AUX_M4);
    DBGFS_DUMP(TVO_CSC_AUX_M5);
    DBGFS_DUMP(TVO_CSC_AUX_M6);
    DBGFS_DUMP(TVO_CSC_AUX_M7);
    DBGFS_DUMP(TVO_AUX_IN_VID_FORMAT);
    seq_putc(s, '\n');
    return 0;
    }
    static struct drm_info_list tvout_debugfs_files[] = {
    { "tvout", tvout_dbg_show, 0, core::ptr::null_mut() },
    };
#[no_mangle]
unsafe extern "C" fn tvout_debugfs_init(tvout: *mut sti_tvout, minor: *mut drm_minor) {
    static void tvout_debugfs_init(struct sti_tvout *tvout, struct drm_minor *minor)
    {
    unsigned int i;
    for (i = 0; i < ARRAY_SIZE(tvout_debugfs_files); i++)
    tvout_debugfs_files[i].data = tvout;
    drm_debugfs_create_files(tvout_debugfs_files,
    ARRAY_SIZE(tvout_debugfs_files),
    minor.debugfs_root, minor);
    }
#[no_mangle]
unsafe extern "C" fn sti_tvout_encoder_dpms(encoder: *mut drm_encoder, mode: c_int) {
    static void sti_tvout_encoder_dpms(struct drm_encoder *encoder, int mode)
    {
    }
    static void sti_tvout_encoder_mode_set(struct drm_encoder *encoder,
    struct drm_display_mode *mode,
    struct drm_display_mode *adjusted_mode)
    {
    }
#[no_mangle]
unsafe extern "C" fn sti_tvout_encoder_destroy(encoder: *mut drm_encoder) {
    static void sti_tvout_encoder_destroy(struct drm_encoder *encoder)
    {
    struct sti_tvout_encoder *sti_encoder = to_sti_tvout_encoder(encoder);
    drm_encoder_cleanup(encoder);
    kfree(sti_encoder);
    }
#[no_mangle]
unsafe extern "C" fn sti_tvout_late_register(encoder: *mut drm_encoder) -> c_int {
    static int sti_tvout_late_register(struct drm_encoder *encoder)
    {
    struct sti_tvout *tvout = to_sti_tvout(encoder);
    if (tvout.debugfs_registered)
    return 0;
    tvout_debugfs_init(tvout, encoder.dev.primary);
    tvout.debugfs_registered = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sti_tvout_early_unregister(encoder: *mut drm_encoder) {
    static void sti_tvout_early_unregister(struct drm_encoder *encoder)
    {
    struct sti_tvout *tvout = to_sti_tvout(encoder);
    if (!tvout.debugfs_registered)
    return;
    tvout.debugfs_registered = false;
    }
    static const struct drm_encoder_funcs sti_tvout_encoder_funcs = {
    .destroy = sti_tvout_encoder_destroy,
    .late_register = sti_tvout_late_register,
    .early_unregister = sti_tvout_early_unregister,
    };
#[no_mangle]
unsafe extern "C" fn sti_dvo_encoder_enable(encoder: *mut drm_encoder) {
    static void sti_dvo_encoder_enable(struct drm_encoder *encoder)
    {
    struct sti_tvout *tvout = to_sti_tvout(encoder);
    tvout_preformatter_set_matrix(tvout, &encoder.crtc.mode);
    tvout_dvo_start(tvout, sti_crtc_is_main(encoder.crtc));
    }
#[no_mangle]
unsafe extern "C" fn sti_dvo_encoder_disable(encoder: *mut drm_encoder) {
    static void sti_dvo_encoder_disable(struct drm_encoder *encoder)
    {
    struct sti_tvout *tvout = to_sti_tvout(encoder);
// Reset VIP register
    tvout_write(tvout, 0x0, TVO_VIP_DVO);
    }
    static const struct drm_encoder_helper_funcs sti_dvo_encoder_helper_funcs = {
    .dpms = sti_tvout_encoder_dpms,
    .mode_set = sti_tvout_encoder_mode_set,
    .enable = sti_dvo_encoder_enable,
    .disable = sti_dvo_encoder_disable,
    };
    static struct drm_encoder *
    sti_tvout_create_dvo_encoder(struct drm_device *dev,
    struct sti_tvout *tvout)
    {
    struct sti_tvout_encoder *encoder;
    struct drm_encoder *drm_encoder;
    encoder = devm_kzalloc(tvout.dev, sizeof(*encoder), GFP_KERNEL);
    if (!encoder)
    return core::ptr::null_mut();
    encoder.tvout = tvout;
    drm_encoder = &encoder.encoder;
    drm_encoder.possible_crtcs = ENCODER_CRTC_MASK;
    drm_encoder_init(dev, drm_encoder,
    &sti_tvout_encoder_funcs, DRM_MODE_ENCODER_LVDS,
    core::ptr::null_mut());
    drm_encoder_helper_add(drm_encoder, &sti_dvo_encoder_helper_funcs);
    return drm_encoder;
    }
#[no_mangle]
unsafe extern "C" fn sti_hda_encoder_enable(encoder: *mut drm_encoder) {
    static void sti_hda_encoder_enable(struct drm_encoder *encoder)
    {
    struct sti_tvout *tvout = to_sti_tvout(encoder);
    tvout_preformatter_set_matrix(tvout, &encoder.crtc.mode);
    tvout_hda_start(tvout, sti_crtc_is_main(encoder.crtc));
    }
#[no_mangle]
unsafe extern "C" fn sti_hda_encoder_disable(encoder: *mut drm_encoder) {
    static void sti_hda_encoder_disable(struct drm_encoder *encoder)
    {
    struct sti_tvout *tvout = to_sti_tvout(encoder);
// reset VIP register
    tvout_write(tvout, 0x0, TVO_VIP_HDF);
// power down HD DAC
    tvout_write(tvout, 1, TVO_HD_DAC_CFG_OFF);
    }
    static const struct drm_encoder_helper_funcs sti_hda_encoder_helper_funcs = {
    .dpms = sti_tvout_encoder_dpms,
    .mode_set = sti_tvout_encoder_mode_set,
    .commit = sti_hda_encoder_enable,
    .disable = sti_hda_encoder_disable,
    };
    static struct drm_encoder *sti_tvout_create_hda_encoder(struct drm_device *dev,
    struct sti_tvout *tvout)
    {
    struct sti_tvout_encoder *encoder;
    struct drm_encoder *drm_encoder;
    encoder = devm_kzalloc(tvout.dev, sizeof(*encoder), GFP_KERNEL);
    if (!encoder)
    return core::ptr::null_mut();
    encoder.tvout = tvout;
    drm_encoder = &encoder.encoder;
    drm_encoder.possible_crtcs = ENCODER_CRTC_MASK;
    drm_encoder_init(dev, drm_encoder,
    &sti_tvout_encoder_funcs, DRM_MODE_ENCODER_DAC, core::ptr::null_mut());
    drm_encoder_helper_add(drm_encoder, &sti_hda_encoder_helper_funcs);
    return drm_encoder;
    }
#[no_mangle]
unsafe extern "C" fn sti_hdmi_encoder_enable(encoder: *mut drm_encoder) {
    static void sti_hdmi_encoder_enable(struct drm_encoder *encoder)
    {
    struct sti_tvout *tvout = to_sti_tvout(encoder);
    tvout_preformatter_set_matrix(tvout, &encoder.crtc.mode);
    tvout_hdmi_start(tvout, sti_crtc_is_main(encoder.crtc));
    }
#[no_mangle]
unsafe extern "C" fn sti_hdmi_encoder_disable(encoder: *mut drm_encoder) {
    static void sti_hdmi_encoder_disable(struct drm_encoder *encoder)
    {
    struct sti_tvout *tvout = to_sti_tvout(encoder);
// reset VIP register
    tvout_write(tvout, 0x0, TVO_VIP_HDMI);
    }
    static const struct drm_encoder_helper_funcs sti_hdmi_encoder_helper_funcs = {
    .dpms = sti_tvout_encoder_dpms,
    .mode_set = sti_tvout_encoder_mode_set,
    .commit = sti_hdmi_encoder_enable,
    .disable = sti_hdmi_encoder_disable,
    };
    static struct drm_encoder *sti_tvout_create_hdmi_encoder(struct drm_device *dev,
    struct sti_tvout *tvout)
    {
    struct sti_tvout_encoder *encoder;
    struct drm_encoder *drm_encoder;
    encoder = devm_kzalloc(tvout.dev, sizeof(*encoder), GFP_KERNEL);
    if (!encoder)
    return core::ptr::null_mut();
    encoder.tvout = tvout;
    drm_encoder = &encoder.encoder;
    drm_encoder.possible_crtcs = ENCODER_CRTC_MASK;
    drm_encoder_init(dev, drm_encoder,
    &sti_tvout_encoder_funcs, DRM_MODE_ENCODER_TMDS, core::ptr::null_mut());
    drm_encoder_helper_add(drm_encoder, &sti_hdmi_encoder_helper_funcs);
    return drm_encoder;
    }
    static void sti_tvout_create_encoders(struct drm_device *dev,
    struct sti_tvout *tvout)
    {
    tvout.hdmi = sti_tvout_create_hdmi_encoder(dev, tvout);
    tvout.hda = sti_tvout_create_hda_encoder(dev, tvout);
    tvout.dvo = sti_tvout_create_dvo_encoder(dev, tvout);
    tvout.hdmi.possible_clones = drm_encoder_mask(tvout.hdmi) |
    drm_encoder_mask(tvout.hda) | drm_encoder_mask(tvout.dvo);
    tvout.hda.possible_clones = drm_encoder_mask(tvout.hdmi) |
    drm_encoder_mask(tvout.hda) | drm_encoder_mask(tvout.dvo);
    tvout.dvo.possible_clones = drm_encoder_mask(tvout.hdmi) |
    drm_encoder_mask(tvout.hda) | drm_encoder_mask(tvout.dvo);
    }
#[no_mangle]
unsafe extern "C" fn sti_tvout_destroy_encoders(tvout: *mut sti_tvout) {
    static void sti_tvout_destroy_encoders(struct sti_tvout *tvout)
    {
    if (tvout.hdmi)
    drm_encoder_cleanup(tvout.hdmi);
    tvout.hdmi = core::ptr::null_mut();
    if (tvout.hda)
    drm_encoder_cleanup(tvout.hda);
    tvout.hda = core::ptr::null_mut();
    if (tvout.dvo)
    drm_encoder_cleanup(tvout.dvo);
    tvout.dvo = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn sti_tvout_bind(dev: *mut device, master: *mut device, data: *mut c_void) -> c_int {
    static int sti_tvout_bind(struct device *dev, struct device *master, void *data)
    {
    struct sti_tvout *tvout = dev_get_drvdata(dev);
    struct drm_device *drm_dev = data;
    tvout.drm_dev = drm_dev;
    sti_tvout_create_encoders(drm_dev, tvout);
    return 0;
    }
    static void sti_tvout_unbind(struct device *dev, struct device *master,
    void *data)
    {
    struct sti_tvout *tvout = dev_get_drvdata(dev);
    sti_tvout_destroy_encoders(tvout);
    }
    static const struct component_ops sti_tvout_ops = {
    .bind	= sti_tvout_bind,
    .unbind	= sti_tvout_unbind,
    };
#[no_mangle]
unsafe extern "C" fn sti_tvout_probe(pdev: *mut platform_device) -> c_int {
    static int sti_tvout_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *node = dev.of_node;
    struct sti_tvout *tvout;
    DRM_INFO("%s\n", __func__);
    if (!node)
    return -ENODEV;
    tvout = devm_kzalloc(dev, sizeof(*tvout), GFP_KERNEL);
    if (!tvout)
    return -ENOMEM;
    tvout.dev = dev;
    tvout.regs = devm_platform_ioremap_resource_byname(pdev, "tvout-reg");
    if (IS_ERR(tvout.regs))
    return PTR_ERR(tvout.regs);
// get reset resources
    tvout.reset = devm_reset_control_get(dev, "tvout");
// take tvout out of reset
    if (!IS_ERR(tvout.reset))
    reset_control_deassert(tvout.reset);
    platform_set_drvdata(pdev, tvout);
    return component_add(dev, &sti_tvout_ops);
    }
#[no_mangle]
unsafe extern "C" fn sti_tvout_remove(pdev: *mut platform_device) {
    static void sti_tvout_remove(struct platform_device *pdev)
    {
    component_del(&pdev.dev, &sti_tvout_ops);
    }
    static const struct of_device_id tvout_of_match[] = {
    { .compatible = "st,stih407-tvout", },
    { /* end node */ }
    };
    MODULE_DEVICE_TABLE(of, tvout_of_match);
    struct platform_driver sti_tvout_driver = {
    .driver = {
    .name = "sti-tvout",
    .of_match_table = tvout_of_match,
    },
    .probe = sti_tvout_probe,
    .remove = sti_tvout_remove,
    };
    MODULE_AUTHOR("Benjamin Gaignard <benjamin.gaignard@st.com>");
    MODULE_DESCRIPTION("STMicroelectronics SoC DRM driver");
    MODULE_LICENSE("GPL");
