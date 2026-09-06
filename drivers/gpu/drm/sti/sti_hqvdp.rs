//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/sti/sti_hqvdp.c
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
// Authors: Fabien Dessenne <fabien.dessenne@st.com> for STMicroelectronics.
//

// Firmware name

// Regs address
pub const HQVDP_DMEM: c_uint = 0x00000000               /* 0x00000000 */;
pub const HQVDP_PMEM: c_uint = 0x00040000               /* 0x00040000 */;
pub const HQVDP_RD_PLUG: c_uint = 0x000E0000               /* 0x000E0000 */;

pub const HQVDP_WR_PLUG: c_uint = 0x000E2000               /* 0x000E2000 */;

pub const HQVDP_MBX: c_uint = 0x000E4000               /* 0x000E4000 */;

// Plugs config
pub const PLUG_CONTROL_ENABLE: c_uint = 0x00000001;
pub const PLUG_PAGE_SIZE_256: c_uint = 0x00000002;
pub const PLUG_MIN_OPC_8: c_uint = 0x00000003;
pub const PLUG_MAX_OPC_64: c_uint = 0x00000006;
pub const PLUG_MAX_CHK_2X: c_uint = 0x00000001;
pub const PLUG_MAX_MSG_1X: c_uint = 0x00000000;
pub const PLUG_MIN_SPACE_1: c_uint = 0x00000000;
// SW reset CTRL

// Startup ctrl 1

// Startup ctrl 2

// Info xP70

// SOFT_VSYNC
pub const SOFT_VSYNC_HW: c_uint = 0x00000000;
pub const SOFT_VSYNC_SW_CMD: c_uint = 0x00000001;
pub const SOFT_VSYNC_SW_CTRL_IRQ: c_uint = 0x00000003;
// Reset & boot poll config
pub const POLL_MAX_ATTEMPT: c_int = 50;
pub const POLL_DELAY_MS: c_int = 20;
pub const SCALE_FACTOR: c_int = 8192;
pub const SCALE_MAX_FOR_LEG_LUT_F: c_int = 4096;
pub const SCALE_MAX_FOR_LEG_LUT_E: c_int = 4915;
pub const SCALE_MAX_FOR_LEG_LUT_D: c_int = 6654;
pub const SCALE_MAX_FOR_LEG_LUT_C: c_int = 8192;
    enum sti_hvsrc_orient {
    HVSRC_HORI,
    HVSRC_VERT
    };
// Command structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sti_hqvdp_top {
    pub config: u32,
    pub mem_format: u32,
    pub current_luma: u32,
    pub current_enh_luma: u32,
    pub current_right_luma: u32,
    pub current_enh_right_luma: u32,
    pub current_chroma: u32,
    pub current_enh_chroma: u32,
    pub current_right_chroma: u32,
    pub current_enh_right_chroma: u32,
    pub output_luma: u32,
    pub output_chroma: u32,
    pub luma_src_pitch: u32,
    pub luma_enh_src_pitch: u32,
    pub luma_right_src_pitch: u32,
    pub luma_enh_right_src_pitch: u32,
    pub chroma_src_pitch: u32,
    pub chroma_enh_src_pitch: u32,
    pub chroma_right_src_pitch: u32,
    pub chroma_enh_right_src_pitch: u32,
    pub luma_processed_pitch: u32,
    pub chroma_processed_pitch: u32,
    pub input_frame_size: u32,
    pub input_viewport_ori: u32,
    pub input_viewport_ori_right: u32,
    pub input_viewport_size: u32,
    pub left_view_border_width: u32,
    pub right_view_border_width: u32,
    pub left_view_3d_offset_width: u32,
    pub right_view_3d_offset_width: u32,
    pub side_stripe_color: u32,
    pub crc_reset_ctrl: u32,
}

// Configs for interlaced : no IT, no pass thru, 3 fields
pub const TOP_CONFIG_INTER_BTM: c_uint = 0x00000000;
pub const TOP_CONFIG_INTER_TOP: c_uint = 0x00000002;
// Config for progressive : no IT, no pass thru, 3 fields
pub const TOP_CONFIG_PROGRESSIVE: c_uint = 0x00000001;
// Default MemFormat: in=420_raster_dual out=444_raster;opaque Mem2Tv mode
pub const TOP_MEM_FORMAT_DFLT: c_uint = 0x00018060;
// Min/Max size
pub const MAX_WIDTH: c_uint = 0x1FFF;
pub const MAX_HEIGHT: c_uint = 0x0FFF;
pub const MIN_WIDTH: c_uint = 0x0030;
pub const MIN_HEIGHT: c_uint = 0x0010;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sti_hqvdp_vc1re {
    pub ctrl_prv_csdi: u32,
    pub ctrl_cur_csdi: u32,
    pub ctrl_nxt_csdi: u32,
    pub ctrl_cur_fmd: u32,
    pub ctrl_nxt_fmd: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sti_hqvdp_fmd {
    pub config: u32,
    pub viewport_ori: u32,
    pub viewport_size: u32,
    pub next_next_luma: u32,
    pub next_next_right_luma: u32,
    pub next_next_next_luma: u32,
    pub next_next_next_right_luma: u32,
    pub threshold_scd: u32,
    pub threshold_rfd: u32,
    pub threshold_move: u32,
    pub threshold_cfd: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sti_hqvdp_csdi {
    pub config: u32,
    pub config2: u32,
    pub dcdi_config: u32,
    pub prev_luma: u32,
    pub prev_enh_luma: u32,
    pub prev_right_luma: u32,
    pub prev_enh_right_luma: u32,
    pub next_luma: u32,
    pub next_enh_luma: u32,
    pub next_right_luma: u32,
    pub next_enh_right_luma: u32,
    pub prev_chroma: u32,
    pub prev_enh_chroma: u32,
    pub prev_right_chroma: u32,
    pub prev_enh_right_chroma: u32,
    pub next_chroma: u32,
    pub next_enh_chroma: u32,
    pub next_right_chroma: u32,
    pub next_enh_right_chroma: u32,
    pub prev_motion: u32,
    pub prev_right_motion: u32,
    pub cur_motion: u32,
    pub cur_right_motion: u32,
    pub next_motion: u32,
    pub next_right_motion: u32,
}

// Config for progressive: by pass
pub const CSDI_CONFIG_PROG: c_uint = 0x00000000;
// Config for directional deinterlacing without motion
pub const CSDI_CONFIG_INTER_DIR: c_uint = 0x00000016;
// Additional configs for fader, blender, motion,... deinterlace algorithms
pub const CSDI_CONFIG2_DFLT: c_uint = 0x000001B3;
pub const CSDI_DCDI_CONFIG_DFLT: c_uint = 0x00203803;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sti_hqvdp_hvsrc {
    pub hor_panoramic_ctrl: u32,
    pub output_picture_size: u32,
    pub init_horizontal: u32,
    pub init_vertical: u32,
    pub param_ctrl: u32,
    pub yh_coef: [u32; NB_COEF],
    pub ch_coef: [u32; NB_COEF],
    pub yv_coef: [u32; NB_COEF],
    pub cv_coef: [u32; NB_COEF],
    pub hori_shift: u32,
    pub vert_shift: u32,
}

// Default ParamCtrl: all controls enabled
pub const HVSRC_PARAM_CTRL_DFLT: c_uint = 0xFFFFFFFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sti_hqvdp_iqi {
    pub config: u32,
    pub demo_wind_size: u32,
    pub pk_config: u32,
    pub coeff0_coeff1: u32,
    pub coeff2_coeff3: u32,
    pub coeff4: u32,
    pub pk_lut: u32,
    pub pk_gain: u32,
    pub pk_coring_level: u32,
    pub cti_config: u32,
    pub le_config: u32,
    pub le_lut: [u32; 64],
    pub con_bri: u32,
    pub sat_gain: u32,
    pub pxf_conf: u32,
    pub default_color: u32,
}

// Default Config : IQI bypassed
pub const IQI_CONFIG_DFLT: c_uint = 0x00000001;
// Default Contrast & Brightness gain = 256
pub const IQI_CON_BRI_DFLT: c_uint = 0x00000100;
// Default Saturation gain = 256
pub const IQI_SAT_GAIN_DFLT: c_uint = 0x00000100;
// Default PxfConf : P2I bypassed
pub const IQI_PXF_CONF_DFLT: c_uint = 0x00000001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sti_hqvdp_top_status {
    pub processing_time: u32,
    pub input_y_crc: u32,
    pub input_uv_crc: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sti_hqvdp_fmd_status {
    pub fmd_repeat_move_status: u32,
    pub fmd_scene_count_status: u32,
    pub cfd_sum: u32,
    pub field_sum: u32,
    pub next_y_fmd_crc: u32,
    pub next_next_y_fmd_crc: u32,
    pub next_next_next_y_fmd_crc: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sti_hqvdp_csdi_status {
    pub prev_y_csdi_crc: u32,
    pub cur_y_csdi_crc: u32,
    pub next_y_csdi_crc: u32,
    pub prev_uv_csdi_crc: u32,
    pub cur_uv_csdi_crc: u32,
    pub next_uv_csdi_crc: u32,
    pub y_csdi_crc: u32,
    pub uv_csdi_crc: u32,
    pub uv_cup_crc: u32,
    pub mot_csdi_crc: u32,
    pub mot_cur_csdi_crc: u32,
    pub mot_prev_csdi_crc: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sti_hqvdp_hvsrc_status {
    pub y_hvsrc_crc: u32,
    pub u_hvsrc_crc: u32,
    pub v_hvsrc_crc: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sti_hqvdp_iqi_status {
    pub pxf_it_status: u32,
    pub y_iqi_crc: u32,
    pub u_iqi_crc: u32,
    pub v_iqi_crc: u32,
}

// Main commands. We use 2 commands one being processed by the firmware, one
// ready to be fetched upon next Vsync
pub const NB_VDP_CMD: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sti_hqvdp_cmd {
    pub top: sti_hqvdp_top,
    pub vc1re: sti_hqvdp_vc1re,
    pub fmd: sti_hqvdp_fmd,
    pub csdi: sti_hqvdp_csdi,
    pub hvsrc: sti_hqvdp_hvsrc,
    pub iqi: sti_hqvdp_iqi,
    pub top_status: sti_hqvdp_top_status,
    pub fmd_status: sti_hqvdp_fmd_status,
    pub csdi_status: sti_hqvdp_csdi_status,
    pub hvsrc_status: sti_hqvdp_hvsrc_status,
    pub iqi_status: sti_hqvdp_iqi_status,
}

//
// STI HQVDP structure
//
// @dev:               driver device
// @drm_dev:           the drm device
// @regs:              registers
// @plane:             plane structure for hqvdp it self
// @clk:               IP clock
// @clk_pix_main:      pix main clock
// @reset:             reset control
// @vtg_nb:            notifier to handle VTG Vsync
// @btm_field_pending: is there any bottom field (interlaced frame) to display
// @hqvdp_cmd:         buffer of commands
// @hqvdp_cmd_paddr:   physical address of hqvdp_cmd
// @vtg:               vtg for main data path
// @xp70_initialized:  true if xp70 is already initialized
// @vtg_registered:    true if registered to VTG
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sti_hqvdp {
    pub dev: *mut device,
    pub drm_dev: *mut drm_device,
    pub regs: *mut void __iomem,
    pub plane: sti_plane,
    pub clk: *mut clk,
    pub clk_pix_main: *mut clk,
    pub reset: *mut reset_control,
    pub vtg_nb: notifier_block,
    pub btm_field_pending: bool,
    pub hqvdp_cmd: *mut c_void,
    pub hqvdp_cmd_paddr: u32,
    pub vtg: *mut sti_vtg,
    pub xp70_initialized: bool,
    pub vtg_registered: bool,
}

    static const uint32_t hqvdp_supported_formats[] = {
    DRM_FORMAT_NV12,
    };
//
// sti_hqvdp_get_free_cmd
// @hqvdp: hqvdp structure
//
// Look for a hqvdp_cmd that is not being used (or about to be used) by the FW.
//
// RETURNS:
// the offset of the command to be used.
// -1 in error cases
//
#[no_mangle]
unsafe extern "C" fn sti_hqvdp_get_free_cmd(hqvdp: *mut sti_hqvdp) -> c_int {
    static int sti_hqvdp_get_free_cmd(struct sti_hqvdp *hqvdp)
    {
    u32 curr_cmd, next_cmd;
    let mut cmd: u32 = hqvdp.hqvdp_cmd_paddr;
    int i;
    curr_cmd = readl(hqvdp.regs + HQVDP_MBX_CURRENT_CMD);
    next_cmd = readl(hqvdp.regs + HQVDP_MBX_NEXT_CMD);
    for (i = 0; i < NB_VDP_CMD; i++) {
    if ((cmd != curr_cmd) && (cmd != next_cmd))
    return i * sizeof(struct sti_hqvdp_cmd);
    cmd += sizeof(struct sti_hqvdp_cmd);
    }
    return -1;
    }
//
// sti_hqvdp_get_curr_cmd
// @hqvdp: hqvdp structure
//
// Look for the hqvdp_cmd that is being used by the FW.
//
// RETURNS:
// the offset of the command to be used.
// -1 in error cases
//
#[no_mangle]
unsafe extern "C" fn sti_hqvdp_get_curr_cmd(hqvdp: *mut sti_hqvdp) -> c_int {
    static int sti_hqvdp_get_curr_cmd(struct sti_hqvdp *hqvdp)
    {
    u32 curr_cmd;
    let mut cmd: u32 = hqvdp.hqvdp_cmd_paddr;
    unsigned int i;
    curr_cmd = readl(hqvdp.regs + HQVDP_MBX_CURRENT_CMD);
    for (i = 0; i < NB_VDP_CMD; i++) {
    if (cmd == curr_cmd)
    return i * sizeof(struct sti_hqvdp_cmd);
    cmd += sizeof(struct sti_hqvdp_cmd);
    }
    return -1;
    }
//
// sti_hqvdp_get_next_cmd
// @hqvdp: hqvdp structure
//
// Look for the next hqvdp_cmd that will be used by the FW.
//
// RETURNS:
// the offset of the next command that will be used.
// -1 in error cases
//
#[no_mangle]
unsafe extern "C" fn sti_hqvdp_get_next_cmd(hqvdp: *mut sti_hqvdp) -> c_int {
    static int sti_hqvdp_get_next_cmd(struct sti_hqvdp *hqvdp)
    {
    int next_cmd;
    let mut cmd: dma_addr_t = hqvdp.hqvdp_cmd_paddr;
    unsigned int i;
    next_cmd = readl(hqvdp.regs + HQVDP_MBX_NEXT_CMD);
    for (i = 0; i < NB_VDP_CMD; i++) {
    if (cmd == next_cmd)
    return i * sizeof(struct sti_hqvdp_cmd);
    cmd += sizeof(struct sti_hqvdp_cmd);
    }
    return -1;
    }

    readl(hqvdp.regs + reg))
    static const char *hqvdp_dbg_get_lut(u32 *coef)
    {
    if (!memcmp(coef, coef_lut_a_legacy, 16))
    return "LUT A";
    if (!memcmp(coef, coef_lut_b, 16))
    return "LUT B";
    if (!memcmp(coef, coef_lut_c_y_legacy, 16))
    return "LUT C Y";
    if (!memcmp(coef, coef_lut_c_c_legacy, 16))
    return "LUT C C";
    if (!memcmp(coef, coef_lut_d_y_legacy, 16))
    return "LUT D Y";
    if (!memcmp(coef, coef_lut_d_c_legacy, 16))
    return "LUT D C";
    if (!memcmp(coef, coef_lut_e_y_legacy, 16))
    return "LUT E Y";
    if (!memcmp(coef, coef_lut_e_c_legacy, 16))
    return "LUT E C";
    if (!memcmp(coef, coef_lut_f_y_legacy, 16))
    return "LUT F Y";
    if (!memcmp(coef, coef_lut_f_c_legacy, 16))
    return "LUT F C";
    return "<UNKNOWN>";
    }
#[no_mangle]
unsafe extern "C" fn hqvdp_dbg_dump_cmd(s: *mut seq_file, c: *mut sti_hqvdp_cmd) {
    static void hqvdp_dbg_dump_cmd(struct seq_file *s, struct sti_hqvdp_cmd *c)
    {
    int src_w, src_h, dst_w, dst_h;
    seq_puts(s, "\n\tTOP:");
    seq_printf(s, "\n\t %-20s 0x%08X", "Config", c.top.config);
    switch (c.top.config) {
    case TOP_CONFIG_PROGRESSIVE:
    seq_puts(s, "\tProgressive");
    break;
    case TOP_CONFIG_INTER_TOP:
    seq_puts(s, "\tInterlaced, top field");
    break;
    case TOP_CONFIG_INTER_BTM:
    seq_puts(s, "\tInterlaced, bottom field");
    break;
    default:
    seq_puts(s, "\t<UNKNOWN>");
    break;
    }
    seq_printf(s, "\n\t %-20s 0x%08X", "MemFormat", c.top.mem_format);
    seq_printf(s, "\n\t %-20s 0x%08X", "CurrentY", c.top.current_luma);
    seq_printf(s, "\n\t %-20s 0x%08X", "CurrentC", c.top.current_chroma);
    seq_printf(s, "\n\t %-20s 0x%08X", "YSrcPitch", c.top.luma_src_pitch);
    seq_printf(s, "\n\t %-20s 0x%08X", "CSrcPitch",
    c.top.chroma_src_pitch);
    seq_printf(s, "\n\t %-20s 0x%08X", "InputFrameSize",
    c.top.input_frame_size);
    seq_printf(s, "\t%dx%d",
    c.top.input_frame_size & 0x0000FFFF,
    c.top.input_frame_size >> 16);
    seq_printf(s, "\n\t %-20s 0x%08X", "InputViewportSize",
    c.top.input_viewport_size);
    src_w = c.top.input_viewport_size & 0x0000FFFF;
    src_h = c.top.input_viewport_size >> 16;
    seq_printf(s, "\t%dx%d", src_w, src_h);
    seq_puts(s, "\n\tHVSRC:");
    seq_printf(s, "\n\t %-20s 0x%08X", "OutputPictureSize",
    c.hvsrc.output_picture_size);
    dst_w = c.hvsrc.output_picture_size & 0x0000FFFF;
    dst_h = c.hvsrc.output_picture_size >> 16;
    seq_printf(s, "\t%dx%d", dst_w, dst_h);
    seq_printf(s, "\n\t %-20s 0x%08X", "ParamCtrl", c.hvsrc.param_ctrl);
    seq_printf(s, "\n\t %-20s %s", "yh_coef",
    hqvdp_dbg_get_lut(c.hvsrc.yh_coef));
    seq_printf(s, "\n\t %-20s %s", "ch_coef",
    hqvdp_dbg_get_lut(c.hvsrc.ch_coef));
    seq_printf(s, "\n\t %-20s %s", "yv_coef",
    hqvdp_dbg_get_lut(c.hvsrc.yv_coef));
    seq_printf(s, "\n\t %-20s %s", "cv_coef",
    hqvdp_dbg_get_lut(c.hvsrc.cv_coef));
    seq_printf(s, "\n\t %-20s", "ScaleH");
    if (dst_w > src_w)
    seq_printf(s, " %d/1", dst_w / src_w);
    else
    seq_printf(s, " 1/%d", src_w / dst_w);
    seq_printf(s, "\n\t %-20s", "tScaleV");
    if (dst_h > src_h)
    seq_printf(s, " %d/1", dst_h / src_h);
    else
    seq_printf(s, " 1/%d", src_h / dst_h);
    seq_puts(s, "\n\tCSDI:");
    seq_printf(s, "\n\t %-20s 0x%08X\t", "Config", c.csdi.config);
    switch (c.csdi.config) {
    case CSDI_CONFIG_PROG:
    seq_puts(s, "Bypass");
    break;
    case CSDI_CONFIG_INTER_DIR:
    seq_puts(s, "Deinterlace, directional");
    break;
    default:
    seq_puts(s, "<UNKNOWN>");
    break;
    }
    seq_printf(s, "\n\t %-20s 0x%08X", "Config2", c.csdi.config2);
    seq_printf(s, "\n\t %-20s 0x%08X", "DcdiConfig", c.csdi.dcdi_config);
    }
#[no_mangle]
unsafe extern "C" fn hqvdp_dbg_show(s: *mut seq_file, data: *mut c_void) -> c_int {
    static int hqvdp_dbg_show(struct seq_file *s, void *data)
    {
    struct drm_info_node *node = s.private;
    struct sti_hqvdp *hqvdp = (struct sti_hqvdp *)node.info_ent.data;
    int cmd, cmd_offset, infoxp70;
    void *virt;
    seq_printf(s, "%s: (vaddr = 0x%p)",
    sti_plane_to_str(&hqvdp.plane), hqvdp.regs);
    DBGFS_DUMP(HQVDP_MBX_IRQ_TO_XP70);
    DBGFS_DUMP(HQVDP_MBX_INFO_HOST);
    DBGFS_DUMP(HQVDP_MBX_IRQ_TO_HOST);
    DBGFS_DUMP(HQVDP_MBX_INFO_XP70);
    infoxp70 = readl(hqvdp.regs + HQVDP_MBX_INFO_XP70);
    seq_puts(s, "\tFirmware state: ");
    if (infoxp70 & INFO_XP70_FW_READY)
    seq_puts(s, "idle and ready");
#[no_mangle]
pub unsafe extern "C" fn if(INFO_XP70_FW_PROCESSING: infoxp70 &) -> else {
    else if (infoxp70 & INFO_XP70_FW_PROCESSING)
    seq_puts(s, "processing a picture");
#[no_mangle]
pub unsafe extern "C" fn if(INFO_XP70_FW_INITQUEUES: infoxp70 &) -> else {
    else if (infoxp70 & INFO_XP70_FW_INITQUEUES)
    seq_puts(s, "programming queues");
    else
    seq_puts(s, "NOT READY");
    DBGFS_DUMP(HQVDP_MBX_SW_RESET_CTRL);
    DBGFS_DUMP(HQVDP_MBX_STARTUP_CTRL1);
    if (readl(hqvdp.regs + HQVDP_MBX_STARTUP_CTRL1)
    & STARTUP_CTRL1_RST_DONE)
    seq_puts(s, "\tReset is done");
    else
    seq_puts(s, "\tReset is NOT done");
    DBGFS_DUMP(HQVDP_MBX_STARTUP_CTRL2);
    if (readl(hqvdp.regs + HQVDP_MBX_STARTUP_CTRL2)
    & STARTUP_CTRL2_FETCH_EN)
    seq_puts(s, "\tFetch is enabled");
    else
    seq_puts(s, "\tFetch is NOT enabled");
    DBGFS_DUMP(HQVDP_MBX_GP_STATUS);
    DBGFS_DUMP(HQVDP_MBX_NEXT_CMD);
    DBGFS_DUMP(HQVDP_MBX_CURRENT_CMD);
    DBGFS_DUMP(HQVDP_MBX_SOFT_VSYNC);
    if (!(readl(hqvdp.regs + HQVDP_MBX_SOFT_VSYNC) & 3))
    seq_puts(s, "\tHW Vsync");
    else
    seq_puts(s, "\tSW Vsync ?!?!");
// Last command
    cmd = readl(hqvdp.regs + HQVDP_MBX_CURRENT_CMD);
    cmd_offset = sti_hqvdp_get_curr_cmd(hqvdp);
    if (cmd_offset == -1) {
    seq_puts(s, "\n\n  Last command: unknown");
    } else {
    virt = hqvdp.hqvdp_cmd + cmd_offset;
    seq_printf(s, "\n\n  Last command: address @ 0x%x (0x%p)",
    cmd, virt);
    hqvdp_dbg_dump_cmd(s, (struct sti_hqvdp_cmd *)virt);
    }
// Next command
    cmd = readl(hqvdp.regs + HQVDP_MBX_NEXT_CMD);
    cmd_offset = sti_hqvdp_get_next_cmd(hqvdp);
    if (cmd_offset == -1) {
    seq_puts(s, "\n\n  Next command: unknown");
    } else {
    virt = hqvdp.hqvdp_cmd + cmd_offset;
    seq_printf(s, "\n\n  Next command address: @ 0x%x (0x%p)",
    cmd, virt);
    hqvdp_dbg_dump_cmd(s, (struct sti_hqvdp_cmd *)virt);
    }
    seq_putc(s, '\n');
    return 0;
    }
    static struct drm_info_list hqvdp_debugfs_files[] = {
    { "hqvdp", hqvdp_dbg_show, 0, core::ptr::null_mut() },
    };
#[no_mangle]
unsafe extern "C" fn hqvdp_debugfs_init(hqvdp: *mut sti_hqvdp, minor: *mut drm_minor) {
    static void hqvdp_debugfs_init(struct sti_hqvdp *hqvdp, struct drm_minor *minor)
    {
    unsigned int i;
    for (i = 0; i < ARRAY_SIZE(hqvdp_debugfs_files); i++)
    hqvdp_debugfs_files[i].data = hqvdp;
    drm_debugfs_create_files(hqvdp_debugfs_files,
    ARRAY_SIZE(hqvdp_debugfs_files),
    minor.debugfs_root, minor);
    }
//
// sti_hqvdp_update_hvsrc
// @orient: horizontal or vertical
// @scale:  scaling/zoom factor
// @hvsrc:  the structure containing the LUT coef
//
// Update the Y and C Lut coef, as well as the shift param
//
// RETURNS:
// None.
//
    static void sti_hqvdp_update_hvsrc(enum sti_hvsrc_orient orient, int scale,
    struct sti_hqvdp_hvsrc *hvsrc)
    {
    const int *coef_c, *coef_y;
    int shift_c, shift_y;
// Get the appropriate coef tables
    if (scale < SCALE_MAX_FOR_LEG_LUT_F) {
    coef_y = coef_lut_f_y_legacy;
    coef_c = coef_lut_f_c_legacy;
    shift_y = SHIFT_LUT_F_Y_LEGACY;
    shift_c = SHIFT_LUT_F_C_LEGACY;
    } else if (scale < SCALE_MAX_FOR_LEG_LUT_E) {
    coef_y = coef_lut_e_y_legacy;
    coef_c = coef_lut_e_c_legacy;
    shift_y = SHIFT_LUT_E_Y_LEGACY;
    shift_c = SHIFT_LUT_E_C_LEGACY;
    } else if (scale < SCALE_MAX_FOR_LEG_LUT_D) {
    coef_y = coef_lut_d_y_legacy;
    coef_c = coef_lut_d_c_legacy;
    shift_y = SHIFT_LUT_D_Y_LEGACY;
    shift_c = SHIFT_LUT_D_C_LEGACY;
    } else if (scale < SCALE_MAX_FOR_LEG_LUT_C) {
    coef_y = coef_lut_c_y_legacy;
    coef_c = coef_lut_c_c_legacy;
    shift_y = SHIFT_LUT_C_Y_LEGACY;
    shift_c = SHIFT_LUT_C_C_LEGACY;
    } else if (scale == SCALE_MAX_FOR_LEG_LUT_C) {
    coef_y = coef_c = coef_lut_b;
    shift_y = shift_c = SHIFT_LUT_B;
    } else {
    coef_y = coef_c = coef_lut_a_legacy;
    shift_y = shift_c = SHIFT_LUT_A_LEGACY;
    }
    if (orient == HVSRC_HORI) {
    hvsrc.hori_shift = (shift_c << 16) | shift_y;
    memcpy(hvsrc.yh_coef, coef_y, sizeof(hvsrc.yh_coef));
    memcpy(hvsrc.ch_coef, coef_c, sizeof(hvsrc.ch_coef));
    } else {
    hvsrc.vert_shift = (shift_c << 16) | shift_y;
    memcpy(hvsrc.yv_coef, coef_y, sizeof(hvsrc.yv_coef));
    memcpy(hvsrc.cv_coef, coef_c, sizeof(hvsrc.cv_coef));
    }
    }
//
// sti_hqvdp_check_hw_scaling
// @hqvdp: hqvdp pointer
// @mode: display mode with timing constraints
// @src_w: source width
// @src_h: source height
// @dst_w: destination width
// @dst_h: destination height
//
// Check if the HW is able to perform the scaling request
// The firmware scaling limitation is "CEIL(1/Zy) <= FLOOR(LFW)" where:
// Zy = OutputHeight / InputHeight
// LFW = (Tx * IPClock) / (MaxNbCycles * Cp)
// Tx : Total video mode horizontal resolution
// IPClock : HQVDP IP clock (Mhz)
// MaxNbCycles: max(InputWidth, OutputWidth)
// Cp: Video mode pixel clock (Mhz)
//
// RETURNS:
// True if the HW can scale.
//
    static bool sti_hqvdp_check_hw_scaling(struct sti_hqvdp *hqvdp,
    struct drm_display_mode *mode,
    int src_w, int src_h,
    int dst_w, int dst_h)
    {
    unsigned long lfw;
    unsigned int inv_zy;
    lfw = mode.htotal * (clk_get_rate(hqvdp.clk) / 1000000);
    lfw /= max(src_w, dst_w) * mode.clock / 1000;
    inv_zy = DIV_ROUND_UP(src_h, dst_h);
    return inv_zy <= lfw;
    }
//
// sti_hqvdp_disable
// @hqvdp: hqvdp pointer
//
// Disables the HQVDP plane
//
#[no_mangle]
unsafe extern "C" fn sti_hqvdp_disable(hqvdp: *mut sti_hqvdp) {
    static void sti_hqvdp_disable(struct sti_hqvdp *hqvdp)
    {
    int i;
    DRM_DEBUG_DRIVER("%s\n", sti_plane_to_str(&hqvdp.plane));
// Unregister VTG Vsync callback
    if (sti_vtg_unregister_client(hqvdp.vtg, &hqvdp.vtg_nb))
    DRM_DEBUG_DRIVER("Warning: cannot unregister VTG notifier\n");
// Set next cmd to NULL
    writel(0, hqvdp.regs + HQVDP_MBX_NEXT_CMD);
    for (i = 0; i < POLL_MAX_ATTEMPT; i++) {
    if (readl(hqvdp.regs + HQVDP_MBX_INFO_XP70)
    & INFO_XP70_FW_READY)
    break;
    msleep(POLL_DELAY_MS);
    }
// VTG can stop now
    clk_disable_unprepare(hqvdp.clk_pix_main);
    if (i == POLL_MAX_ATTEMPT)
    DRM_ERROR("XP70 could not revert to idle\n");
    hqvdp.plane.status = STI_PLANE_DISABLED;
    hqvdp.vtg_registered = false;
    }
//
// sti_hqvdp_vtg_cb
// @nb: notifier block
// @evt: event message
// @data: private data
//
// Handle VTG Vsync event, display pending bottom field
//
// RETURNS:
// 0 on success.
//
#[no_mangle]
unsafe extern "C" fn sti_hqvdp_vtg_cb(nb: *mut notifier_block, evt: c_ulong, data: *mut c_void) -> c_int {
    static int sti_hqvdp_vtg_cb(struct notifier_block *nb, unsigned long evt, void *data)
    {
    struct sti_hqvdp *hqvdp = container_of(nb, struct sti_hqvdp, vtg_nb);
    int btm_cmd_offset, top_cmd_offest;
    struct sti_hqvdp_cmd *btm_cmd, *top_cmd;
    if ((evt != VTG_TOP_FIELD_EVENT) && (evt != VTG_BOTTOM_FIELD_EVENT)) {
    DRM_DEBUG_DRIVER("Unknown event\n");
    return 0;
    }
    if (hqvdp.plane.status == STI_PLANE_FLUSHING) {
// disable need to be synchronize on vsync event
    DRM_DEBUG_DRIVER("Vsync event received => disable %s\n",
    sti_plane_to_str(&hqvdp.plane));
    sti_hqvdp_disable(hqvdp);
    }
    if (hqvdp.btm_field_pending) {
// Create the btm field command from the current one
    btm_cmd_offset = sti_hqvdp_get_free_cmd(hqvdp);
    top_cmd_offest = sti_hqvdp_get_curr_cmd(hqvdp);
    if ((btm_cmd_offset == -1) || (top_cmd_offest == -1)) {
    DRM_DEBUG_DRIVER("Warning: no cmd, will skip field\n");
    return -EBUSY;
    }
    btm_cmd = hqvdp.hqvdp_cmd + btm_cmd_offset;
    top_cmd = hqvdp.hqvdp_cmd + top_cmd_offest;
    memcpy(btm_cmd, top_cmd, sizeof(*btm_cmd));
    btm_cmd.top.config = TOP_CONFIG_INTER_BTM;
    btm_cmd.top.current_luma +=
    btm_cmd.top.luma_src_pitch / 2;
    btm_cmd.top.current_chroma +=
    btm_cmd.top.chroma_src_pitch / 2;
// Post the command to mailbox
    writel(hqvdp.hqvdp_cmd_paddr + btm_cmd_offset,
    hqvdp.regs + HQVDP_MBX_NEXT_CMD);
    hqvdp.btm_field_pending = false;
    dev_dbg(hqvdp.dev, "%s Posted command:0x%x\n",
    __func__, hqvdp.hqvdp_cmd_paddr);
    sti_plane_update_fps(&hqvdp.plane, false, true);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sti_hqvdp_init(hqvdp: *mut sti_hqvdp) {
    static void sti_hqvdp_init(struct sti_hqvdp *hqvdp)
    {
    int size;
    dma_addr_t dma_addr;
    hqvdp.vtg_nb.notifier_call = sti_hqvdp_vtg_cb;
// Allocate memory for the VDP commands
    size = NB_VDP_CMD * sizeof(struct sti_hqvdp_cmd);
    hqvdp.hqvdp_cmd = dma_alloc_wc(hqvdp.dev, size,
    &dma_addr,
    GFP_KERNEL | GFP_DMA);
    if (!hqvdp.hqvdp_cmd) {
    DRM_ERROR("Failed to allocate memory for VDP cmd\n");
    return;
    }
    hqvdp.hqvdp_cmd_paddr = (u32)dma_addr;
    memset(hqvdp.hqvdp_cmd, 0, size);
    }
#[no_mangle]
unsafe extern "C" fn sti_hqvdp_init_plugs(hqvdp: *mut sti_hqvdp) {
    static void sti_hqvdp_init_plugs(struct sti_hqvdp *hqvdp)
    {
// Configure Plugs (same for RD & WR)
    writel(PLUG_PAGE_SIZE_256, hqvdp.regs + HQVDP_RD_PLUG_PAGE_SIZE);
    writel(PLUG_MIN_OPC_8, hqvdp.regs + HQVDP_RD_PLUG_MIN_OPC);
    writel(PLUG_MAX_OPC_64, hqvdp.regs + HQVDP_RD_PLUG_MAX_OPC);
    writel(PLUG_MAX_CHK_2X, hqvdp.regs + HQVDP_RD_PLUG_MAX_CHK);
    writel(PLUG_MAX_MSG_1X, hqvdp.regs + HQVDP_RD_PLUG_MAX_MSG);
    writel(PLUG_MIN_SPACE_1, hqvdp.regs + HQVDP_RD_PLUG_MIN_SPACE);
    writel(PLUG_CONTROL_ENABLE, hqvdp.regs + HQVDP_RD_PLUG_CONTROL);
    writel(PLUG_PAGE_SIZE_256, hqvdp.regs + HQVDP_WR_PLUG_PAGE_SIZE);
    writel(PLUG_MIN_OPC_8, hqvdp.regs + HQVDP_WR_PLUG_MIN_OPC);
    writel(PLUG_MAX_OPC_64, hqvdp.regs + HQVDP_WR_PLUG_MAX_OPC);
    writel(PLUG_MAX_CHK_2X, hqvdp.regs + HQVDP_WR_PLUG_MAX_CHK);
    writel(PLUG_MAX_MSG_1X, hqvdp.regs + HQVDP_WR_PLUG_MAX_MSG);
    writel(PLUG_MIN_SPACE_1, hqvdp.regs + HQVDP_WR_PLUG_MIN_SPACE);
    writel(PLUG_CONTROL_ENABLE, hqvdp.regs + HQVDP_WR_PLUG_CONTROL);
    }
//
// sti_hqvdp_start_xp70
// @hqvdp: hqvdp pointer
//
// Run the xP70 initialization sequence
//
#[no_mangle]
unsafe extern "C" fn sti_hqvdp_start_xp70(hqvdp: *mut sti_hqvdp) {
    static void sti_hqvdp_start_xp70(struct sti_hqvdp *hqvdp)
    {
    const struct firmware *firmware;
    u32 *fw_rd_plug, *fw_wr_plug, *fw_pmem, *fw_dmem;
    u8 *data;
    int i;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_header {
    pub rd_size: c_int,
    pub wr_size: c_int,
    pub pmem_size: c_int,
    pub dmem_size: c_int,
    pub header: *mut },
    if (hqvdp.xp70_initialized) {
    pub initialized\n"): DRM_DEBUG_DRIVER("HQVDP XP70 already,
    }
// Request firmware
    if (request_firmware(&firmware, HQVDP_FMW_NAME, hqvdp.dev)) {
    pub firmware\n"): DRM_ERROR("Can't get HQVDP,
    }
// Check firmware parts
    if (!firmware) {
    pub available\n"): DRM_ERROR("Firmware not,
    }
    pub )firmware->data: *mut header = (struct fw_header,
    if (firmware.size < sizeof(*header)) {
    pub firmware->size): DRM_ERROR("Invalid firmware size (%zu)\n",,
    pub out: goto,
    }
    if ((sizeof(*header) + header.rd_size + header.wr_size +
    header.pmem_size + header.dmem_size) != firmware.size) {
    DRM_ERROR("Invalid fmw structure (%zu+%d+%d+%d+%d != %zu)\n",
    sizeof(*header), header.rd_size, header.wr_size,
    header.pmem_size, header.dmem_size,
    pub out: goto,
    }
    pub )firmware->data: *mut data = (u8,
    pub sizeof(*header): *mut data +=,
    pub )data: *mut fw_rd_plug = (void,
    pub header->rd_size: data +=,
    pub )data: *mut fw_wr_plug = (void,
    pub header->wr_size: data +=,
    pub )data: *mut fw_pmem = (void,
    pub header->pmem_size: data +=,
    pub )data: *mut fw_dmem = (void,
// Enable clock
    if (clk_prepare_enable(hqvdp.clk))
    pub clk\n"): DRM_ERROR("Failed to prepare/enable HQVDP,
// Reset
    pub HQVDP_MBX_SW_RESET_CTRL): writel(SW_RESET_CTRL_FULL, hqvdp->regs +,
    pub {: for (i = 0; i < POLL_MAX_ATTEMPT; i++),
    if (readl(hqvdp.regs + HQVDP_MBX_STARTUP_CTRL1)
    & STARTUP_CTRL1_RST_DONE)
    }
    if (i == POLL_MAX_ATTEMPT) {
    pub reset\n"): DRM_ERROR("Could not,
    pub out: goto,
    }
// Init Read & Write plugs
    pub i++): for (i = 0; i < header->rd_size / 4;,
    pub 4): *mut *mut writel(fw_rd_plug[i], hqvdp->regs + HQVDP_RD_PLUG + i,
    pub i++): for (i = 0; i < header->wr_size / 4;,
    pub 4): *mut *mut writel(fw_wr_plug[i], hqvdp->regs + HQVDP_WR_PLUG + i,
// Authorize Idle Mode
    pub HQVDP_MBX_STARTUP_CTRL1): writel(STARTUP_CTRL1_AUTH_IDLE, hqvdp->regs +,
// Prevent VTG interruption during the boot
    pub HQVDP_MBX_SOFT_VSYNC): writel(SOFT_VSYNC_SW_CTRL_IRQ, hqvdp->regs +,
    pub HQVDP_MBX_NEXT_CMD): writel(0, hqvdp->regs +,
// Download PMEM & DMEM
    pub i++): for (i = 0; i < header->pmem_size / 4;,
    pub 4): *mut *mut writel(fw_pmem[i], hqvdp->regs + HQVDP_PMEM + i,
    pub i++): for (i = 0; i < header->dmem_size / 4;,
    pub 4): *mut *mut writel(fw_dmem[i], hqvdp->regs + HQVDP_DMEM + i,
// Enable fetch
    pub HQVDP_MBX_STARTUP_CTRL2): writel(STARTUP_CTRL2_FETCH_EN, hqvdp->regs +,
// Wait end of boot
    pub {: for (i = 0; i < POLL_MAX_ATTEMPT; i++),
    if (readl(hqvdp.regs + HQVDP_MBX_INFO_XP70)
    & INFO_XP70_FW_READY)
    }
    if (i == POLL_MAX_ATTEMPT) {
    pub boot\n"): DRM_ERROR("Could not,
    pub out: goto,
    }
// Launch Vsync
    pub HQVDP_MBX_SOFT_VSYNC): writel(SOFT_VSYNC_HW, hqvdp->regs +,
    pub initialized\n"): DRM_INFO("HQVDP XP70,
    pub true: hqvdp->xp70_initialized =,
    out:
    }
    static int sti_hqvdp_atomic_check(struct drm_plane *drm_plane,
    struct drm_atomic_commit *state)
    {
    struct drm_plane_state *new_plane_state = drm_atomic_get_new_plane_state(state,
    pub to_sti_plane(drm_plane): *mut *mut sti_plane plane =,
    pub to_sti_hqvdp(plane): *mut *mut sti_hqvdp hqvdp =,
    pub new_plane_state->crtc: *mut *mut drm_crtc crtc =,
    pub new_plane_state->fb: *mut *mut drm_framebuffer fb =,
    pub crtc_state: *mut drm_crtc_state,
    pub mode: *mut drm_display_mode,
    pub dst_h: int dst_x, dst_y, dst_w,,
    pub src_h: int src_x, src_y, src_w,,
// no need for further checks if the plane is being disabled
    if (!crtc || !fb)
    pub 0: return,
    pub crtc): crtc_state = drm_atomic_get_crtc_state(state,,
    if (IS_ERR(crtc_state))
    pub PTR_ERR(crtc_state): return,
    pub &crtc_state->mode: mode =,
    pub new_plane_state->crtc_x: dst_x =,
    pub new_plane_state->crtc_y: dst_y =,
    pub dst_x): dst_w = clamp_val(new_plane_state->crtc_w, 0, mode->hdisplay -,
    pub dst_y): dst_h = clamp_val(new_plane_state->crtc_h, 0, mode->vdisplay -,
// src_x are in 16.16 format
    pub 16: src_x = new_plane_state->src_x >>,
    pub 16: src_y = new_plane_state->src_y >>,
    pub 16: src_w = new_plane_state->src_w >>,
    pub 16: src_h = new_plane_state->src_h >>,
    if (mode.clock && !sti_hqvdp_check_hw_scaling(hqvdp, mode,
    src_w, src_h,
    dst_w, dst_h)) {
    pub capabilities\n"): DRM_ERROR("Scaling beyond HW,
    pub -EINVAL: return,
    }
    if (!drm_fb_dma_get_gem_obj(fb, 0)) {
    pub fb\n"): DRM_ERROR("Can't get DMA GEM object for,
    pub -EINVAL: return,
    }
//
// Input / output size
// Align to upper even value
//
    pub 2): dst_w = ALIGN(dst_w,,
    pub 2): dst_h = ALIGN(dst_h,,
    if ((src_w > MAX_WIDTH) || (src_w < MIN_WIDTH) ||
    (src_h > MAX_HEIGHT) || (src_h < MIN_HEIGHT) ||
    (dst_w > MAX_WIDTH) || (dst_w < MIN_WIDTH) ||
    (dst_h > MAX_HEIGHT) || (dst_h < MIN_HEIGHT)) {
    DRM_ERROR("Invalid in/out size %dx%d . %dx%d\n",
    src_w, src_h,
    pub dst_h): dst_w,,
    pub -EINVAL: return,
    }
    if (!hqvdp.xp70_initialized)
// Start HQVDP XP70 coprocessor
    if (!hqvdp.vtg_registered) {
// Prevent VTG shutdown
    if (clk_prepare_enable(hqvdp.clk_pix_main)) {
    pub clk\n"): DRM_ERROR("Failed to prepare/enable pix main,
    pub -EINVAL: return,
    }
// Register VTG Vsync callback to handle bottom fields
    if (sti_vtg_register_client(hqvdp.vtg,
    &hqvdp.vtg_nb,
    crtc)) {
    pub notifier\n"): DRM_ERROR("Cannot register VTG,
    pub -EINVAL: return,
    }
    pub true: hqvdp->vtg_registered =,
    }
    DRM_DEBUG_KMS("CRTC:%d (%s) drm plane:%d (%s)\n",
    crtc.base.id, sti_mixer_to_str(to_sti_mixer(crtc)),
    pub sti_plane_to_str(plane)): drm_plane->base.id,,
    DRM_DEBUG_KMS("%s dst=(%dx%d)@(%d,%d) - src=(%dx%d)@(%d,%d)\n",
    sti_plane_to_str(plane),
    dst_w, dst_h, dst_x, dst_y,
    pub src_y): src_w, src_h, src_x,,
    pub 0: return,
    }
    static void sti_hqvdp_atomic_update(struct drm_plane *drm_plane,
    struct drm_atomic_commit *state)
    {
    struct drm_plane_state *oldstate = drm_atomic_get_old_plane_state(state,
    struct drm_plane_state *newstate = drm_atomic_get_new_plane_state(state,
    pub to_sti_plane(drm_plane): *mut *mut sti_plane plane =,
    pub to_sti_hqvdp(plane): *mut *mut sti_hqvdp hqvdp =,
    pub newstate->crtc: *mut *mut drm_crtc crtc =,
    pub newstate->fb: *mut *mut drm_framebuffer fb =,
    pub mode: *mut drm_display_mode,
    pub dst_h: int dst_x, dst_y, dst_w,,
    pub src_h: int src_x, src_y, src_w,,
    pub dma_obj: *mut drm_gem_dma_object,
    pub cmd: *mut sti_hqvdp_cmd,
    pub scale_v: int scale_h,,
    pub cmd_offset: c_int,
    if (!crtc || !fb)
    if ((oldstate.fb == newstate.fb) &&
    (oldstate.crtc_x == newstate.crtc_x) &&
    (oldstate.crtc_y == newstate.crtc_y) &&
    (oldstate.crtc_w == newstate.crtc_w) &&
    (oldstate.crtc_h == newstate.crtc_h) &&
    (oldstate.src_x == newstate.src_x) &&
    (oldstate.src_y == newstate.src_y) &&
    (oldstate.src_w == newstate.src_w) &&
    (oldstate.src_h == newstate.src_h)) {
// No change since last update, do not post cmd
    pub cmd\n"): DRM_DEBUG_DRIVER("No change, not posting,
    pub STI_PLANE_UPDATED: plane->status =,
    }
    pub &crtc->mode: mode =,
    pub newstate->crtc_x: dst_x =,
    pub newstate->crtc_y: dst_y =,
    pub dst_x): dst_w = clamp_val(newstate->crtc_w, 0, mode->hdisplay -,
    pub dst_y): dst_h = clamp_val(newstate->crtc_h, 0, mode->vdisplay -,
// src_x are in 16.16 format
    pub 16: src_x = newstate->src_x >>,
    pub 16: src_y = newstate->src_y >>,
    pub 16: src_w = newstate->src_w >>,
    pub 16: src_h = newstate->src_h >>,
    pub sti_hqvdp_get_free_cmd(hqvdp): cmd_offset =,
    if (cmd_offset == -1) {
    pub frame\n"): DRM_DEBUG_DRIVER("Warning: no cmd, will skip,
    }
    pub cmd_offset: cmd = hqvdp->hqvdp_cmd +,
// Static parameters, defaulting to progressive mode
    pub TOP_CONFIG_PROGRESSIVE: cmd->top.config =,
    pub TOP_MEM_FORMAT_DFLT: cmd->top.mem_format =,
    pub HVSRC_PARAM_CTRL_DFLT: cmd->hvsrc.param_ctrl =,
    pub CSDI_CONFIG_PROG: cmd->csdi.config =,
// VC1RE, FMD bypassed : keep everything set to 0
// IQI/P2I bypassed
    pub IQI_CONFIG_DFLT: cmd->iqi.config =,
    pub IQI_CON_BRI_DFLT: cmd->iqi.con_bri =,
    pub IQI_SAT_GAIN_DFLT: cmd->iqi.sat_gain =,
    pub IQI_PXF_CONF_DFLT: cmd->iqi.pxf_conf =,
    pub 0): dma_obj = drm_fb_dma_get_gem_obj(fb,,
    DRM_DEBUG_DRIVER("drm FB:%d format:%.4s phys@:0x%lx\n", fb.base.id,
    (char *)&fb.format.format,
    pub dma_obj->dma_addr): (unsigned long),
// Buffer planes address
    pub fb->offsets[0]: cmd->top.current_luma = (u32) dma_obj->dma_addr +,
    pub fb->offsets[1]: cmd->top.current_chroma = (u32) dma_obj->dma_addr +,
// Pitches
    pub fb->pitches[0]: cmd->top.luma_processed_pitch =,
    pub fb->pitches[0]: cmd->top.luma_src_pitch =,
    pub fb->pitches[1]: cmd->top.chroma_processed_pitch =,
    pub fb->pitches[1]: cmd->top.chroma_src_pitch =,
// Input / output size
// Align to upper even value
    pub 2): dst_w = ALIGN(dst_w,,
    pub 2): dst_h = ALIGN(dst_h,,
    pub src_w: cmd->top.input_viewport_size = src_h << 16 |,
    pub src_w: cmd->top.input_frame_size = src_h << 16 |,
    pub dst_w: cmd->hvsrc.output_picture_size = dst_h << 16 |,
    pub src_x: cmd->top.input_viewport_ori = src_y << 16 |,
// Handle interlaced
    if (fb.flags & DRM_MODE_FB_INTERLACED) {
// Top field to display
    pub TOP_CONFIG_INTER_TOP: cmd->top.config =,
// Update pitches and vert size
    pub src_w: cmd->top.input_frame_size = (src_h / 2) << 16 |,
    pub 2: *mut *mut cmd->top.luma_processed_pitch =,
    pub 2: *mut *mut cmd->top.luma_src_pitch =,
    pub 2: *mut *mut cmd->top.chroma_processed_pitch =,
    pub 2: *mut *mut cmd->top.chroma_src_pitch =,
// Enable directional deinterlacing processing
    pub CSDI_CONFIG_INTER_DIR: cmd->csdi.config =,
    pub CSDI_CONFIG2_DFLT: cmd->csdi.config2 =,
    pub CSDI_DCDI_CONFIG_DFLT: cmd->csdi.dcdi_config =,
    }
// Update hvsrc lut coef
    pub src_w: *mut *mut scale_h = SCALE_FACTOR  dst_w /,
    pub &cmd->hvsrc): sti_hqvdp_update_hvsrc(HVSRC_HORI, scale_h,,
    pub src_h: *mut *mut scale_v = SCALE_FACTOR  dst_h /,
    pub &cmd->hvsrc): sti_hqvdp_update_hvsrc(HVSRC_VERT, scale_v,,
    writel(hqvdp.hqvdp_cmd_paddr + cmd_offset,
    pub HQVDP_MBX_NEXT_CMD): hqvdp->regs +,
// Interlaced : get ready to display the bottom field at next Vsync
    if (fb.flags & DRM_MODE_FB_INTERLACED)
    pub true: hqvdp->btm_field_pending =,
    dev_dbg(hqvdp.dev, "%s Posted command:0x%x\n",
    pub cmd_offset): __func__, hqvdp->hqvdp_cmd_paddr +,
    pub true): sti_plane_update_fps(plane, true,,
    pub STI_PLANE_UPDATED: plane->status =,
    }
    static void sti_hqvdp_atomic_disable(struct drm_plane *drm_plane,
    struct drm_atomic_commit *state)
    {
    struct drm_plane_state *oldstate = drm_atomic_get_old_plane_state(state,
    pub to_sti_plane(drm_plane): *mut *mut sti_plane plane =,
    if (!oldstate.crtc) {
    DRM_DEBUG_DRIVER("drm plane:%d not enabled\n",
    }
    DRM_DEBUG_DRIVER("CRTC:%d (%s) drm plane:%d (%s)\n",
    oldstate.crtc.base.id,
    sti_mixer_to_str(to_sti_mixer(oldstate.crtc)),
    pub sti_plane_to_str(plane)): drm_plane->base.id,,
    pub STI_PLANE_DISABLING: plane->status =,
    }
    static const struct drm_plane_helper_funcs sti_hqvdp_helpers_funcs = {
    .atomic_check = sti_hqvdp_atomic_check,
    .atomic_update = sti_hqvdp_atomic_update,
    .atomic_disable = sti_hqvdp_atomic_disable,
}

#[no_mangle]
unsafe extern "C" fn sti_hqvdp_late_register(drm_plane: *mut drm_plane) -> c_int {
    static int sti_hqvdp_late_register(struct drm_plane *drm_plane)
    {
    struct sti_plane *plane = to_sti_plane(drm_plane);
    struct sti_hqvdp *hqvdp = to_sti_hqvdp(plane);
    hqvdp_debugfs_init(hqvdp, drm_plane.dev.primary);
    return 0;
    }
    static const struct drm_plane_funcs sti_hqvdp_plane_helpers_funcs = {
    .update_plane = drm_atomic_helper_update_plane,
    .disable_plane = drm_atomic_helper_disable_plane,
    .destroy = drm_plane_cleanup,
    .reset = drm_atomic_helper_plane_reset,
    .atomic_duplicate_state = drm_atomic_helper_plane_duplicate_state,
    .atomic_destroy_state = drm_atomic_helper_plane_destroy_state,
    .late_register = sti_hqvdp_late_register,
    };
    static struct drm_plane *sti_hqvdp_create(struct drm_device *drm_dev,
    struct device *dev, int desc)
    {
    struct sti_hqvdp *hqvdp = dev_get_drvdata(dev);
    int res;
    hqvdp.plane.desc = desc;
    hqvdp.plane.status = STI_PLANE_DISABLED;
    sti_hqvdp_init(hqvdp);
    res = drm_universal_plane_init(drm_dev, &hqvdp.plane.drm_plane, 1,
    &sti_hqvdp_plane_helpers_funcs,
    hqvdp_supported_formats,
    ARRAY_SIZE(hqvdp_supported_formats),
    core::ptr::null_mut(), DRM_PLANE_TYPE_OVERLAY, core::ptr::null_mut());
    if (res) {
    DRM_ERROR("Failed to initialize universal plane\n");
    return core::ptr::null_mut();
    }
    drm_plane_helper_add(&hqvdp.plane.drm_plane, &sti_hqvdp_helpers_funcs);
    sti_plane_init_property(&hqvdp.plane, DRM_PLANE_TYPE_OVERLAY);
    return &hqvdp.plane.drm_plane;
    }
#[no_mangle]
unsafe extern "C" fn sti_hqvdp_bind(dev: *mut device, master: *mut device, data: *mut c_void) -> c_int {
    static int sti_hqvdp_bind(struct device *dev, struct device *master, void *data)
    {
    struct sti_hqvdp *hqvdp = dev_get_drvdata(dev);
    struct drm_device *drm_dev = data;
    struct drm_plane *plane;
    DRM_DEBUG_DRIVER("\n");
    hqvdp.drm_dev = drm_dev;
// Create HQVDP plane once xp70 is initialized
    plane = sti_hqvdp_create(drm_dev, hqvdp.dev, STI_HQVDP_0);
    if (!plane)
    DRM_ERROR("Can't create HQVDP plane\n");
    return 0;
    }
    static void sti_hqvdp_unbind(struct device *dev,
    struct device *master, void *data)
    {
// do nothing
    }
    static const struct component_ops sti_hqvdp_ops = {
    .bind = sti_hqvdp_bind,
    .unbind = sti_hqvdp_unbind,
    };
#[no_mangle]
unsafe extern "C" fn sti_hqvdp_probe(pdev: *mut platform_device) -> c_int {
    static int sti_hqvdp_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *vtg_np;
    struct sti_hqvdp *hqvdp;
    DRM_DEBUG_DRIVER("\n");
    hqvdp = devm_kzalloc(dev, sizeof(*hqvdp), GFP_KERNEL);
    if (!hqvdp) {
    DRM_ERROR("Failed to allocate HQVDP context\n");
    return -ENOMEM;
    }
    hqvdp.dev = dev;
    hqvdp.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(hqvdp.regs)) {
    DRM_ERROR("Register mapping failed\n");
    return PTR_ERR(hqvdp.regs);
    }
// Get clock resources
    hqvdp.clk = devm_clk_get(dev, "hqvdp");
    hqvdp.clk_pix_main = devm_clk_get(dev, "pix_main");
    if (IS_ERR(hqvdp.clk) || IS_ERR(hqvdp.clk_pix_main)) {
    DRM_ERROR("Cannot get clocks\n");
    return -ENXIO;
    }
// Get reset resources
    hqvdp.reset = devm_reset_control_get(dev, "hqvdp");
    if (!IS_ERR(hqvdp.reset))
    reset_control_deassert(hqvdp.reset);
    vtg_np = of_parse_phandle(pdev.dev.of_node, "st,vtg", 0);
    if (vtg_np)
    hqvdp.vtg = of_vtg_find(vtg_np);
    of_node_put(vtg_np);
    platform_set_drvdata(pdev, hqvdp);
    return component_add(&pdev.dev, &sti_hqvdp_ops);
    }
#[no_mangle]
unsafe extern "C" fn sti_hqvdp_remove(pdev: *mut platform_device) {
    static void sti_hqvdp_remove(struct platform_device *pdev)
    {
    component_del(&pdev.dev, &sti_hqvdp_ops);
    }
    static const struct of_device_id hqvdp_of_match[] = {
    { .compatible = "st,stih407-hqvdp", },
    { /* end node */ }
    };
    MODULE_DEVICE_TABLE(of, hqvdp_of_match);
    struct platform_driver sti_hqvdp_driver = {
    .driver = {
    .name = "sti-hqvdp",
    .of_match_table = hqvdp_of_match,
    },
    .probe = sti_hqvdp_probe,
    .remove = sti_hqvdp_remove,
    };
    MODULE_AUTHOR("Benjamin Gaignard <benjamin.gaignard@st.com>");
    MODULE_DESCRIPTION("STMicroelectronics SoC DRM driver");
    MODULE_LICENSE("GPL");
