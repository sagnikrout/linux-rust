//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/renesas/vsp1/vsp1_rpf.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// vsp1_rpf.c  --  R-Car VSP1 Read Pixel Formatter
//
// Copyright (C) 2013-2014 Renesas Electronics Corporation
//
// Contact: Laurent Pinchart (laurent.pinchart@ideasonboard.com)
//

pub const RPF_MAX_WIDTH: c_int = 8190;
pub const RPF_MAX_HEIGHT: c_int = 8190;
// Pre extended display list command data structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vsp1_extcmd_auto_fld_body {
    pub top_y0: u32,
    pub bottom_y0: u32,
    pub top_c0: u32,
    pub bottom_c0: u32,
    pub top_c1: u32,
    pub bottom_c1: u32,
    pub reserved0: u32,
    pub reserved1: u32,
    pub __packed: },
// -----------------------------------------------------------------------------
// Device Access
//
    static inline void vsp1_rpf_write(struct vsp1_rwpf *rpf,
    struct vsp1_dl_body *dlb, u32 reg, u32 data)
    {
    vsp1_dl_body_write(dlb, reg + rpf.entity.index * VI6_RPF_OFFSET,
    }
// -----------------------------------------------------------------------------
// VSP1 Entity Operations
//
    static void rpf_configure_stream(struct vsp1_entity *entity,
    struct v4l2_subdev_state *state,
    struct vsp1_pipeline *pipe,
    struct vsp1_dl_list *dl,
    struct vsp1_dl_body *dlb)
    {
    pub to_rwpf(&entity->subdev): *mut *mut vsp1_rwpf rpf =,
    pub rpf->fmtinfo: *const *const vsp1_format_info fmtinfo =,
    pub &rpf->format: *const *const v4l2_pix_format_mplane format =,
    pub source_format: *const v4l2_mbus_framefmt,
    pub sink_format: *const v4l2_mbus_framefmt,
    pub 0: unsigned int left =,
    pub 0: unsigned int top =,
    pub pstride: u32,
    pub infmt: u32,
// Stride
    pstride = format.plane_fmt[0].bytesperline
    pub VI6_RPF_SRCM_PSTRIDE_Y_SHIFT: <<,
    if (format.num_planes > 1)
    pstride |= format.plane_fmt[1].bytesperline
    pub VI6_RPF_SRCM_PSTRIDE_C_SHIFT: <<,
//
// pstride has both STRIDE_Y and STRIDE_C, but multiplying the whole
// of pstride by 2 is conveniently OK here as we are multiplying both
// values.
//
    if (pipe.interlaced)
    pub 2: *mut *mut pstride =,
    pub pstride): vsp1_rpf_write(rpf, dlb, VI6_RPF_SRCM_PSTRIDE,,
// Format
    pub RWPF_PAD_SINK): sink_format = v4l2_subdev_state_get_format(state,,
    pub RWPF_PAD_SOURCE): source_format = v4l2_subdev_state_get_format(state,,
    infmt = (pipe.iif ? 0 : VI6_RPF_INFMT_CIPM)
    pub VI6_RPF_INFMT_RDFMT_SHIFT): | (fmtinfo->hwfmt <<,
    if (fmtinfo.swap_yc)
    pub VI6_RPF_INFMT_SPYCS: infmt |=,
    if (fmtinfo.swap_uv)
    pub VI6_RPF_INFMT_SPUVS: infmt |=,
    if (sink_format.code != source_format.code) {
    pub ycbcr_enc: u16,
    pub quantization: u16,
    pub rdtm: u32,
    if (sink_format.code == MEDIA_BUS_FMT_AYUV8_1X32) {
    pub sink_format->ycbcr_enc: ycbcr_enc =,
    pub sink_format->quantization: quantization =,
    } else {
    pub source_format->ycbcr_enc: ycbcr_enc =,
    pub source_format->quantization: quantization =,
    }
    if (ycbcr_enc == V4L2_YCBCR_ENC_601 &&
    quantization == V4L2_QUANTIZATION_LIM_RANGE)
    pub VI6_RPF_INFMT_RDTM_BT601: rdtm =,
    else if (ycbcr_enc == V4L2_YCBCR_ENC_601 &&
    quantization == V4L2_QUANTIZATION_FULL_RANGE)
    pub VI6_RPF_INFMT_RDTM_BT601_EXT: rdtm =,
    else if (ycbcr_enc == V4L2_YCBCR_ENC_709 &&
    quantization == V4L2_QUANTIZATION_LIM_RANGE)
    pub VI6_RPF_INFMT_RDTM_BT709: rdtm =,
    else
    pub VI6_RPF_INFMT_RDTM_BT709_EXT: rdtm =,
    pub rdtm: infmt |= VI6_RPF_INFMT_CSC |,
    }
    pub infmt): vsp1_rpf_write(rpf, dlb, VI6_RPF_INFMT,,
    pub fmtinfo->swap): vsp1_rpf_write(rpf, dlb, VI6_RPF_DSWAP,,
// No further configuration for VSPX.
    if (pipe.iif) {
// VSPX wants alpha_sel to be set to 0.
    pub 0): vsp1_rpf_write(rpf, dlb, VI6_RPF_ALPH_SEL,,
    }
    if (entity.vsp1.info.gen == 4) {
    pub ext_infmt0: u32,
    pub ext_infmt1: u32,
    pub ext_infmt2: u32,
    switch (fmtinfo.fourcc) {
    case V4L2_PIX_FMT_RGBX1010102:
    pub VI6_RPF_EXT_INFMT0_BYPP_M1_RGB10: ext_infmt0 =,
    pub 0): ext_infmt1 = VI6_RPF_EXT_INFMT1_PACK_CPOS(0, 10, 20,,
    pub 0): ext_infmt2 = VI6_RPF_EXT_INFMT2_PACK_CLEN(10, 10, 10,,
    case V4L2_PIX_FMT_RGBA1010102:
    pub VI6_RPF_EXT_INFMT0_BYPP_M1_RGB10: ext_infmt0 =,
    pub 30): ext_infmt1 = VI6_RPF_EXT_INFMT1_PACK_CPOS(0, 10, 20,,
    pub 2): ext_infmt2 = VI6_RPF_EXT_INFMT2_PACK_CLEN(10, 10, 10,,
    case V4L2_PIX_FMT_ARGB2101010:
    pub VI6_RPF_EXT_INFMT0_BYPP_M1_RGB10: ext_infmt0 =,
    pub 0): ext_infmt1 = VI6_RPF_EXT_INFMT1_PACK_CPOS(2, 12, 22,,
    pub 2): ext_infmt2 = VI6_RPF_EXT_INFMT2_PACK_CLEN(10, 10, 10,,
    case V4L2_PIX_FMT_Y210:
    ext_infmt0 = VI6_RPF_EXT_INFMT0_F2B |
    VI6_RPF_EXT_INFMT0_IPBD_Y_10 |
    pub 0x0: ext_infmt1 =,
    pub 0x0: ext_infmt2 =,
    case V4L2_PIX_FMT_Y212:
    ext_infmt0 = VI6_RPF_EXT_INFMT0_F2B |
    VI6_RPF_EXT_INFMT0_IPBD_Y_12 |
    pub 0x0: ext_infmt1 =,
    pub 0x0: ext_infmt2 =,
    default:
    pub 0: ext_infmt0 =,
    pub 0: ext_infmt1 =,
    pub 0: ext_infmt2 =,
    }
    pub ext_infmt0): vsp1_rpf_write(rpf, dlb, VI6_RPF_EXT_INFMT0,,
    pub ext_infmt1): vsp1_rpf_write(rpf, dlb, VI6_RPF_EXT_INFMT1,,
    pub ext_infmt2): vsp1_rpf_write(rpf, dlb, VI6_RPF_EXT_INFMT2,,
    }
// Output location.
    if (pipe.brx) {
    pub compose: *const v4l2_rect,
    compose = v4l2_subdev_state_get_compose(pipe.brx.state,
    pub compose->left: left =,
    pub compose->top: top =,
    }
    if (pipe.interlaced)
    pub 2: top /=,
    vsp1_rpf_write(rpf, dlb, VI6_RPF_LOC,
    (left << VI6_RPF_LOC_HCOORD_SHIFT) |
    pub VI6_RPF_LOC_VCOORD_SHIFT)): (top <<,
//
// On Gen2 use the alpha channel (extended to 8 bits) when available or
// a fixed alpha value set through the V4L2_CID_ALPHA_COMPONENT control
// otherwise.
//
// The Gen3+ RPF has extended alpha capability and can both multiply the
// alpha channel by a fixed global alpha value, and multiply the pixel
// components to convert the input to premultiplied alpha.
//
// As alpha premultiplication is available in the BRx for both Gen2 and
// Gen3+ we handle it there and use the Gen3 alpha multiplier for global
// alpha multiplication only. This however prevents conversion to
// premultiplied alpha if no BRx is present in the pipeline. If that use
// case turns out to be useful we will revisit the implementation (for
// Gen3 only).
//
// We enable alpha multiplication on Gen3+ using the fixed alpha value
// set through the V4L2_CID_ALPHA_COMPONENT control when the input
// contains an alpha channel. On Gen2 the global alpha is ignored in
// that case.
//
// In all cases, disable color keying.
//
    vsp1_rpf_write(rpf, dlb, VI6_RPF_ALPH_SEL, VI6_RPF_ALPH_SEL_AEXT_EXT |
    (fmtinfo.alpha ? VI6_RPF_ALPH_SEL_ASEL_PACKED
    pub VI6_RPF_ALPH_SEL_ASEL_FIXED)): :,
    if (entity.vsp1.info.gen >= 3) {
    pub mult: u32,
    if (fmtinfo.alpha) {
//
// When the input contains an alpha channel enable the
// alpha multiplier. If the input is premultiplied we
// need to multiply both the alpha channel and the pixel
// components by the global alpha value to keep them
// premultiplied. Otherwise multiply the alpha channel
// only.
//
    bool premultiplied = format.flags
    pub V4L2_PIX_FMT_FLAG_PREMUL_ALPHA: &,
    mult = VI6_RPF_MULT_ALPHA_A_MMD_RATIO
    | (premultiplied ?
    VI6_RPF_MULT_ALPHA_P_MMD_RATIO :
    } else {
//
// When the input doesn't contain an alpha channel the
// global alpha value is applied in the unpacking unit,
// the alpha multiplier isn't needed and must be
// disabled.
//
    mult = VI6_RPF_MULT_ALPHA_A_MMD_NONE
    pub VI6_RPF_MULT_ALPHA_P_MMD_NONE: |,
    }
    pub mult: rpf->mult_alpha =,
    }
    pub 0): vsp1_rpf_write(rpf, dlb, VI6_RPF_MSK_CTRL,,
    pub 0): vsp1_rpf_write(rpf, dlb, VI6_RPF_CKEY_CTRL,,
    }
    static void vsp1_rpf_configure_autofld(struct vsp1_rwpf *rpf,
    struct vsp1_dl_list *dl)
    {
    pub &rpf->format: *const *const v4l2_pix_format_mplane format =,
    pub cmd: *mut vsp1_dl_ext_cmd,
    pub auto_fld: *mut vsp1_extcmd_auto_fld_body,
    pub offset_c: u32 offset_y,,
    pub vsp1_dl_get_pre_cmd(dl): cmd =,
    if (WARN_ONCE(!cmd, "Failed to obtain an autofld cmd"))
// Re-index our auto_fld to match the current RPF.
    pub cmd->data: auto_fld =,
    pub &auto_fld[rpf->entity.index]: auto_fld =,
    pub rpf->mem.addr[0]: auto_fld->top_y0 =,
    pub rpf->mem.addr[1]: auto_fld->top_c0 =,
    pub rpf->mem.addr[2]: auto_fld->top_c1 =,
    pub format->plane_fmt[0].bytesperline: offset_y =,
    pub format->plane_fmt[1].bytesperline: offset_c =,
    pub offset_y: auto_fld->bottom_y0 = rpf->mem.addr[0] +,
    pub offset_c: auto_fld->bottom_c0 = rpf->mem.addr[1] +,
    pub offset_c: auto_fld->bottom_c1 = rpf->mem.addr[2] +,
    pub rpf->entity.index): cmd->flags |= VI6_DL_EXT_AUTOFLD_INT | BIT(16 +,
    }
    static void rpf_configure_frame(struct vsp1_entity *entity,
    struct vsp1_pipeline *pipe,
    struct vsp1_dl_list *dl,
    struct vsp1_dl_body *dlb)
    {
    pub to_rwpf(&entity->subdev): *mut *mut vsp1_rwpf rpf =,
    vsp1_rpf_write(rpf, dlb, VI6_RPF_VRTCOL_SET,
    pub VI6_RPF_VRTCOL_SET_LAYA_SHIFT): rpf->alpha <<,
    vsp1_rpf_write(rpf, dlb, VI6_RPF_MULT_ALPHA, rpf.mult_alpha |
    pub VI6_RPF_MULT_ALPHA_RATIO_SHIFT)): (rpf->alpha <<,
    pub rpf->alpha): vsp1_pipeline_propagate_alpha(pipe, dlb,,
    }
    static void rpf_configure_partition(struct vsp1_entity *entity,
    struct vsp1_pipeline *pipe,
    const struct vsp1_partition *partition,
    struct vsp1_dl_list *dl,
    struct vsp1_dl_body *dlb)
    {
    pub to_rwpf(&entity->subdev): *mut *mut vsp1_rwpf rpf =,
    pub rpf->mem: vsp1_rwpf_memory mem =,
    pub rpf->entity.vsp1: *mut *mut vsp1_device vsp1 =,
    pub rpf->fmtinfo: *const *const vsp1_format_info fmtinfo =,
    pub &rpf->format: *const *const v4l2_pix_format_mplane format =,
    pub partition->rpf[rpf->entity.index]: v4l2_rect crop =,
//
// Source size and crop offsets.
//
// The crop offsets correspond to the location of the crop
// rectangle top left corner in the plane buffer. Only two
// offsets are needed, as planes 2 and 3 always have identical
// strides.
//
    if (pipe.interlaced) {
    pub fmtinfo->vsub): crop.height = round_down(crop.height / 2,,
    pub fmtinfo->vsub): crop.top = round_down(crop.top / 2,,
    }
    vsp1_rpf_write(rpf, dlb, VI6_RPF_SRC_BSIZE,
    (crop.width << VI6_RPF_SRC_BSIZE_BHSIZE_SHIFT) |
    pub VI6_RPF_SRC_BSIZE_BVSIZE_SHIFT)): (crop.height <<,
    vsp1_rpf_write(rpf, dlb, VI6_RPF_SRC_ESIZE,
    (crop.width << VI6_RPF_SRC_ESIZE_EHSIZE_SHIFT) |
    pub VI6_RPF_SRC_ESIZE_EVSIZE_SHIFT)): (crop.height <<,
    mem.addr[0] += crop.top * format.plane_fmt[0].bytesperline
    pub 8: *mut *mut + crop.left  fmtinfo->bpp[0] /,
    if (format.num_planes > 1) {
    pub format->plane_fmt[1].bytesperline: unsigned int bpl =,
    pub offset: c_uint,
    offset = crop.top / fmtinfo.vsub * bpl
    pub 8: *mut *mut + crop.left / fmtinfo->hsub  fmtinfo->bpp[1] /,
    pub offset: mem.addr[1] +=,
    pub offset: mem.addr[2] +=,
    }
//
// On Gen3+ hardware the SPUVS bit has no effect on 3-planar
// formats. Swap the U and V planes manually in that case.
//
    if (vsp1.info.gen >= 3 && format.num_planes == 3 &&
    fmtinfo.swap_uv)
    pub mem.addr[2]): swap(mem.addr[1],,
//
// Interlaced pipelines will use the extended pre-cmd to process
// SRCM_ADDR_{Y,C0,C1}.
//
    if (pipe.interlaced) {
    pub dl): vsp1_rpf_configure_autofld(rpf,,
    } else {
    pub mem.addr[0]): vsp1_rpf_write(rpf, dlb, VI6_RPF_SRCM_ADDR_Y,,
    pub mem.addr[1]): vsp1_rpf_write(rpf, dlb, VI6_RPF_SRCM_ADDR_C0,,
    pub mem.addr[2]): vsp1_rpf_write(rpf, dlb, VI6_RPF_SRCM_ADDR_C1,,
    }
    }
    static void rpf_partition(struct vsp1_entity *entity,
    struct v4l2_subdev_state *state,
    struct vsp1_pipeline *pipe,
    struct vsp1_partition *partition,
    unsigned int partition_idx,
    struct v4l2_rect *window)
    {
    pub to_rwpf(&entity->subdev): *mut *mut vsp1_rwpf rpf =,
    pub &partition->rpf[rpf->entity.index]: *mut *mut v4l2_rect rpf_rect =,
//
// Partition Algorithm Control
//
// The partition algorithm can split this frame into multiple slices. We
// must adjust our partition window based on the pipe configuration to
// match the destination partition window. To achieve this, we adjust
// our crop to provide a 'sub-crop' matching the expected partition
// window.
//
// rpf_rect = *v4l2_subdev_state_get_crop(state, RWPF_PAD_SINK);
    if (pipe.partitions > 1) {
    pub window->width: rpf_rect->width =,
    pub window->left: rpf_rect->left +=,
    }
    }
    static const struct vsp1_entity_operations rpf_entity_ops = {
    .configure_stream = rpf_configure_stream,
    .configure_frame = rpf_configure_frame,
    .configure_partition = rpf_configure_partition,
    .partition = rpf_partition,
}

// -----------------------------------------------------------------------------
// Initialization and Cleanup
//
    struct vsp1_rwpf *vsp1_rpf_create(struct vsp1_device *vsp1, unsigned int index)
    {
    struct vsp1_rwpf *rpf;
    char name[6];
    int ret;
    rpf = devm_kzalloc(vsp1.dev, sizeof(*rpf), GFP_KERNEL);
    if (rpf == core::ptr::null_mut())
    return ERR_PTR(-ENOMEM);
    rpf.entity.ops = &rpf_entity_ops;
    rpf.entity.type = VSP1_ENTITY_RPF;
    rpf.entity.index = index;
    rpf.entity.min_width = RWPF_MIN_WIDTH;
    rpf.entity.min_height = RWPF_MIN_HEIGHT;
    rpf.entity.max_width = RPF_MAX_WIDTH;
    rpf.entity.max_height = RPF_MAX_HEIGHT;
    sprintf(name, "rpf.%u", index);
    ret = vsp1_entity_init(vsp1, &rpf.entity, name, 2, &vsp1_rwpf_subdev_ops,
    MEDIA_ENT_F_PROC_VIDEO_PIXEL_FORMATTER);
    if (ret < 0)
    return ERR_PTR(ret);
// Initialize the control handler.
    ret = vsp1_rwpf_init_ctrls(rpf, 0);
    if (ret < 0) {
    dev_err(vsp1.dev, "rpf%u: failed to initialize controls\n",
    index);
    goto error;
    }
    v4l2_ctrl_handler_setup(&rpf.ctrls);
    return rpf;
    error:
    vsp1_entity_destroy(&rpf.entity);
    return ERR_PTR(ret);
    }
