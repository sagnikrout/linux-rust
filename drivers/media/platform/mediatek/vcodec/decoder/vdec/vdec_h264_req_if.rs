//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/mediatek/vcodec/decoder/vdec/vdec_h264_req_if.c
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
// struct mtk_h264_dec_slice_param  - parameters for decode current frame
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_h264_dec_slice_param {
    pub sps: mtk_h264_sps_param,
    pub pps: mtk_h264_pps_param,
    pub scaling_matrix: slice_api_h264_scaling_matrix,
    pub decode_params: slice_api_h264_decode_param,
    pub h264_dpb_info: [mtk_h264_dpb_info; 16],
}

//
// struct vdec_h264_dec_info - decode information
// @dpb_sz		: decoding picture buffer size
// @resolution_changed  : flag to notify that a resolution change happened
// @realloc_mv_buf	: flag to notify driver to re-allocate mv buffer
// @cap_num_planes	: number planes of capture buffer
// @bs_dma		: Input bit-stream buffer dma address
// @y_fb_dma		: Y frame buffer dma address
// @c_fb_dma		: C frame buffer dma address
// @vdec_fb_va		: VDEC frame buffer struct virtual address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdec_h264_dec_info {
    pub dpb_sz: u32,
    pub resolution_changed: u32,
    pub realloc_mv_buf: u32,
    pub cap_num_planes: u32,
    pub bs_dma: u64,
    pub y_fb_dma: u64,
    pub c_fb_dma: u64,
    pub vdec_fb_va: u64,
}

//
// struct vdec_h264_vsi - shared memory for decode information exchange
// between VPU and Host.
// The memory is allocated by VPU then mapping to Host
// in vpu_dec_init() and freed in vpu_dec_deinit()
// by VPU.
// AP-W/R : AP is writer/reader on this item
// VPU-W/R: VPU is write/reader on this item
// @pred_buf_dma : HW working prediction buffer dma address (AP-W, VPU-R)
// @mv_buf_dma   : HW working motion vector buffer dma address (AP-W, VPU-R)
// @dec          : decode information (AP-R, VPU-W)
// @pic          : picture information (AP-R, VPU-W)
// @crop         : crop information (AP-R, VPU-W)
// @h264_slice_params : the parameters that hardware use to decode
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdec_h264_vsi {
    pub pred_buf_dma: u64,
    pub mv_buf_dma: [u64; H264_MAX_MV_NUM],
    pub dec: vdec_h264_dec_info,
    pub pic: vdec_pic_info,
    pub crop: v4l2_rect,
    pub h264_slice_params: mtk_h264_dec_slice_param,
}

//
// struct vdec_h264_slice_inst - h264 decoder instance
// @num_nalu : how many nalus be decoded
// @ctx      : point to mtk_vcodec_dec_ctx
// @pred_buf : HW working prediction buffer
// @mv_buf   : HW working motion vector buffer
// @vpu      : VPU instance
// @vsi_ctx  : Local VSI data for this decoding context
// @h264_slice_param : the parameters that hardware use to decode
// @dpb : decoded picture buffer used to store reference buffer information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdec_h264_slice_inst {
    pub num_nalu: c_uint,
    pub ctx: *mut mtk_vcodec_dec_ctx,
    pub pred_buf: mtk_vcodec_mem,
    pub mv_buf: [mtk_vcodec_mem; H264_MAX_MV_NUM],
    pub vpu: vdec_vpu_inst,
    pub vsi_ctx: vdec_h264_vsi,
    pub h264_slice_param: mtk_h264_dec_slice_param,
    pub dpb: [v4l2_h264_dpb_entry; 16],
}

#[no_mangle]
unsafe extern "C" fn get_vdec_decode_parameters(inst: *mut vdec_h264_slice_inst) -> c_int {
    static int get_vdec_decode_parameters(struct vdec_h264_slice_inst *inst)
    {
    const struct v4l2_ctrl_h264_decode_params *dec_params;
    const struct v4l2_ctrl_h264_sps *sps;
    const struct v4l2_ctrl_h264_pps *pps;
    const struct v4l2_ctrl_h264_scaling_matrix *scaling_matrix;
    struct mtk_h264_dec_slice_param *slice_param = &inst.h264_slice_param;
    struct v4l2_h264_reflist_builder reflist_builder;
    struct v4l2_h264_reference v4l2_p0_reflist[V4L2_H264_REF_LIST_LEN];
    struct v4l2_h264_reference v4l2_b0_reflist[V4L2_H264_REF_LIST_LEN];
    struct v4l2_h264_reference v4l2_b1_reflist[V4L2_H264_REF_LIST_LEN];
    u8 *p0_reflist = slice_param.decode_params.ref_pic_list_p0;
    u8 *b0_reflist = slice_param.decode_params.ref_pic_list_b0;
    u8 *b1_reflist = slice_param.decode_params.ref_pic_list_b1;
    dec_params =
    mtk_vdec_h264_get_ctrl_ptr(inst.ctx, V4L2_CID_STATELESS_H264_DECODE_PARAMS);
    if (IS_ERR(dec_params))
    return PTR_ERR(dec_params);
    sps = mtk_vdec_h264_get_ctrl_ptr(inst.ctx, V4L2_CID_STATELESS_H264_SPS);
    if (IS_ERR(sps))
    return PTR_ERR(sps);
    pps = mtk_vdec_h264_get_ctrl_ptr(inst.ctx, V4L2_CID_STATELESS_H264_PPS);
    if (IS_ERR(pps))
    return PTR_ERR(pps);
    scaling_matrix =
    mtk_vdec_h264_get_ctrl_ptr(inst.ctx, V4L2_CID_STATELESS_H264_SCALING_MATRIX);
    if (IS_ERR(scaling_matrix))
    return PTR_ERR(scaling_matrix);
    mtk_vdec_h264_update_dpb(dec_params, inst.dpb);
    mtk_vdec_h264_copy_sps_params(&slice_param.sps, sps);
    mtk_vdec_h264_copy_pps_params(&slice_param.pps, pps);
    mtk_vdec_h264_copy_scaling_matrix(&slice_param.scaling_matrix, scaling_matrix);
    mtk_vdec_h264_copy_decode_params(&slice_param.decode_params,
    dec_params, inst.dpb);
    mtk_vdec_h264_fill_dpb_info(inst.ctx, &slice_param.decode_params,
    slice_param.h264_dpb_info);
// Build the reference lists
    v4l2_h264_init_reflist_builder(&reflist_builder, dec_params, sps,
    inst.dpb);
    v4l2_h264_build_p_ref_list(&reflist_builder, v4l2_p0_reflist);
    v4l2_h264_build_b_ref_lists(&reflist_builder, v4l2_b0_reflist,
    v4l2_b1_reflist);
// Adapt the built lists to the firmware's expectations
    mtk_vdec_h264_get_ref_list(p0_reflist, v4l2_p0_reflist, reflist_builder.num_valid);
    mtk_vdec_h264_get_ref_list(b0_reflist, v4l2_b0_reflist, reflist_builder.num_valid);
    mtk_vdec_h264_get_ref_list(b1_reflist, v4l2_b1_reflist, reflist_builder.num_valid);
    memcpy(&inst.vsi_ctx.h264_slice_params, slice_param,
    sizeof(inst.vsi_ctx.h264_slice_params));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn allocate_prediction_buf(inst: *mut vdec_h264_slice_inst) -> c_int {
    static int allocate_prediction_buf(struct vdec_h264_slice_inst *inst)
    {
    int err;
    inst.pred_buf.size = BUF_PREDICTION_SZ;
    err = mtk_vcodec_mem_alloc(inst.ctx, &inst.pred_buf);
    if (err) {
    mtk_vdec_err(inst.ctx, "failed to allocate ppl buf");
    return err;
    }
    inst.vsi_ctx.pred_buf_dma = inst.pred_buf.dma_addr;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn free_prediction_buf(inst: *mut vdec_h264_slice_inst) {
    static void free_prediction_buf(struct vdec_h264_slice_inst *inst)
    {
    struct mtk_vcodec_mem *mem = &inst.pred_buf;
    inst.vsi_ctx.pred_buf_dma = 0;
    if (mem.va)
    mtk_vcodec_mem_free(inst.ctx, mem);
    }
    static int alloc_mv_buf(struct vdec_h264_slice_inst *inst,
    struct vdec_pic_info *pic)
    {
    int i;
    int err;
    struct mtk_vcodec_mem *mem = core::ptr::null_mut();
    let mut buf_sz: c_uint = mtk_vdec_h264_get_mv_buf_size(pic.buf_w, pic.buf_h);
    mtk_v4l2_vdec_dbg(3, inst.ctx, "size = 0x%x", buf_sz);
    for (i = 0; i < H264_MAX_MV_NUM; i++) {
    mem = &inst.mv_buf[i];
    if (mem.va)
    mtk_vcodec_mem_free(inst.ctx, mem);
    mem.size = buf_sz;
    err = mtk_vcodec_mem_alloc(inst.ctx, mem);
    if (err) {
    mtk_vdec_err(inst.ctx, "failed to allocate mv buf");
    return err;
    }
    inst.vsi_ctx.mv_buf_dma[i] = mem.dma_addr;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn free_mv_buf(inst: *mut vdec_h264_slice_inst) {
    static void free_mv_buf(struct vdec_h264_slice_inst *inst)
    {
    int i;
    struct mtk_vcodec_mem *mem;
    for (i = 0; i < H264_MAX_MV_NUM; i++) {
    inst.vsi_ctx.mv_buf_dma[i] = 0;
    mem = &inst.mv_buf[i];
    if (mem.va)
    mtk_vcodec_mem_free(inst.ctx, mem);
    }
    }
    static void get_pic_info(struct vdec_h264_slice_inst *inst,
    struct vdec_pic_info *pic)
    {
    struct mtk_vcodec_dec_ctx *ctx = inst.ctx;
    ctx.picinfo.buf_w = ALIGN(ctx.picinfo.pic_w, VCODEC_DEC_ALIGNED_64);
    ctx.picinfo.buf_h = ALIGN(ctx.picinfo.pic_h, VCODEC_DEC_ALIGNED_64);
    ctx.picinfo.fb_sz[0] = ctx.picinfo.buf_w * ctx.picinfo.buf_h;
    ctx.picinfo.fb_sz[1] = ctx.picinfo.fb_sz[0] >> 1;
    inst.vsi_ctx.dec.cap_num_planes =
    ctx.q_data[MTK_Q_DATA_DST].fmt.num_planes;
// pic = ctx->picinfo;
    mtk_vdec_debug(inst.ctx, "pic(%d, %d), buf(%d, %d)",
    ctx.picinfo.pic_w, ctx.picinfo.pic_h,
    ctx.picinfo.buf_w, ctx.picinfo.buf_h);
    mtk_vdec_debug(inst.ctx, "Y/C(%d, %d)", ctx.picinfo.fb_sz[0],
    ctx.picinfo.fb_sz[1]);
    if (ctx.last_decoded_picinfo.pic_w != ctx.picinfo.pic_w ||
    ctx.last_decoded_picinfo.pic_h != ctx.picinfo.pic_h) {
    inst.vsi_ctx.dec.resolution_changed = true;
    if (ctx.last_decoded_picinfo.buf_w != ctx.picinfo.buf_w ||
    ctx.last_decoded_picinfo.buf_h != ctx.picinfo.buf_h)
    inst.vsi_ctx.dec.realloc_mv_buf = true;
    mtk_v4l2_vdec_dbg(1, inst.ctx, "ResChg: (%d %d) : old(%d, %d) . new(%d, %d)",
    inst.vsi_ctx.dec.resolution_changed,
    inst.vsi_ctx.dec.realloc_mv_buf,
    ctx.last_decoded_picinfo.pic_w,
    ctx.last_decoded_picinfo.pic_h,
    ctx.picinfo.pic_w, ctx.picinfo.pic_h);
    }
    }
#[no_mangle]
unsafe extern "C" fn get_crop_info(inst: *mut vdec_h264_slice_inst, cr: *mut v4l2_rect) {
    static void get_crop_info(struct vdec_h264_slice_inst *inst, struct v4l2_rect *cr)
    {
    cr.left = inst.vsi_ctx.crop.left;
    cr.top = inst.vsi_ctx.crop.top;
    cr.width = inst.vsi_ctx.crop.width;
    cr.height = inst.vsi_ctx.crop.height;
    mtk_vdec_debug(inst.ctx, "l=%d, t=%d, w=%d, h=%d",
    cr.left, cr.top, cr.width, cr.height);
    }
#[no_mangle]
unsafe extern "C" fn get_dpb_size(inst: *mut vdec_h264_slice_inst, dpb_sz: *mut c_uint) {
    static void get_dpb_size(struct vdec_h264_slice_inst *inst, unsigned int *dpb_sz)
    {
// dpb_sz = inst->vsi_ctx.dec.dpb_sz;
    mtk_vdec_debug(inst.ctx, "sz=%d", *dpb_sz);
    }
#[no_mangle]
unsafe extern "C" fn vdec_h264_slice_init(ctx: *mut mtk_vcodec_dec_ctx) -> c_int {
    static int vdec_h264_slice_init(struct mtk_vcodec_dec_ctx *ctx)
    {
    struct vdec_h264_slice_inst *inst;
    int err;
    inst = kzalloc_obj(*inst);
    if (!inst)
    return -ENOMEM;
    inst.ctx = ctx;
    inst.vpu.id = SCP_IPI_VDEC_H264;
    inst.vpu.ctx = ctx;
    err = vpu_dec_init(&inst.vpu);
    if (err) {
    mtk_vdec_err(ctx, "vdec_h264 init err=%d", err);
    goto error_free_inst;
    }
    memcpy(&inst.vsi_ctx, inst.vpu.vsi, sizeof(inst.vsi_ctx));
    inst.vsi_ctx.dec.resolution_changed = true;
    inst.vsi_ctx.dec.realloc_mv_buf = true;
    err = allocate_prediction_buf(inst);
    if (err)
    goto error_deinit;
    mtk_vdec_debug(ctx, "struct size = %zu,%zu,%zu,%zu\n",
    sizeof(struct mtk_h264_sps_param),
    sizeof(struct mtk_h264_pps_param),
    sizeof(struct mtk_h264_dec_slice_param),
    sizeof(struct mtk_h264_dpb_info));
    mtk_vdec_debug(ctx, "H264 Instance >> %p", inst);
    ctx.drv_handle = inst;
    return 0;
    error_deinit:
    vpu_dec_deinit(&inst.vpu);
    error_free_inst:
    kfree(inst);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn vdec_h264_slice_deinit(h_vdec: *mut c_void) {
    static void vdec_h264_slice_deinit(void *h_vdec)
    {
    struct vdec_h264_slice_inst *inst = h_vdec;
    vpu_dec_deinit(&inst.vpu);
    free_prediction_buf(inst);
    free_mv_buf(inst);
    kfree(inst);
    }
    static int vdec_h264_slice_decode(void *h_vdec, struct mtk_vcodec_mem *bs,
    struct vdec_fb *unused, bool *res_chg)
    {
    struct vdec_h264_slice_inst *inst = h_vdec;
    const struct v4l2_ctrl_h264_decode_params *dec_params =
    mtk_vdec_h264_get_ctrl_ptr(inst.ctx, V4L2_CID_STATELESS_H264_DECODE_PARAMS);
    struct vdec_vpu_inst *vpu = &inst.vpu;
    struct mtk_video_dec_buf *src_buf_info;
    struct mtk_video_dec_buf *dst_buf_info;
    struct vdec_fb *fb;
    u32 data[2];
    u64 y_fb_dma;
    u64 c_fb_dma;
    int err;
    inst.num_nalu++;
// bs NULL means flush decoder
    if (!bs)
    return vpu_dec_reset(vpu);
    fb = inst.ctx.dev.vdec_pdata.get_cap_buffer(inst.ctx);
    if (!fb) {
    mtk_vdec_err(inst.ctx, "fb buffer is core::ptr::null_mut()");
    return -ENOMEM;
    }
    src_buf_info = container_of(bs, struct mtk_video_dec_buf, bs_buffer);
    dst_buf_info = container_of(fb, struct mtk_video_dec_buf, frame_buffer);
    y_fb_dma = fb.base_y.dma_addr;
    c_fb_dma = fb.base_c.dma_addr;
    mtk_vdec_debug(inst.ctx, "+ [%d] FB y_dma=%llx c_dma=%llx va=%p",
    inst.num_nalu, y_fb_dma, c_fb_dma, fb);
    inst.vsi_ctx.dec.bs_dma = (uint64_t)bs.dma_addr;
    inst.vsi_ctx.dec.y_fb_dma = y_fb_dma;
    inst.vsi_ctx.dec.c_fb_dma = c_fb_dma;
    inst.vsi_ctx.dec.vdec_fb_va = (u64)(uintptr_t)fb;
    v4l2_m2m_buf_copy_metadata(&src_buf_info.m2m_buf.vb,
    &dst_buf_info.m2m_buf.vb);
    err = get_vdec_decode_parameters(inst);
    if (err)
    goto err_free_fb_out;
    data[0] = bs.size;
//
// Reconstruct the first byte of the NAL unit, as the firmware requests
// that information to be passed even though it is present in the stream
// itself...
//
    data[1] = (dec_params.nal_ref_idc << 5) |
    ((dec_params.flags & V4L2_H264_DECODE_PARAM_FLAG_IDR_PIC)
    ? 0x5 : 0x1);
// res_chg = inst->vsi_ctx.dec.resolution_changed;
    if (*res_chg) {
    mtk_vdec_debug(inst.ctx, "- resolution changed -");
    if (inst.vsi_ctx.dec.realloc_mv_buf) {
    err = alloc_mv_buf(inst, &inst.ctx.picinfo);
    inst.vsi_ctx.dec.realloc_mv_buf = false;
    if (err)
    goto err_free_fb_out;
    }
// res_chg = false;
    }
    memcpy(inst.vpu.vsi, &inst.vsi_ctx, sizeof(inst.vsi_ctx));
    err = vpu_dec_start(vpu, data, 2);
    if (err)
    goto err_free_fb_out;
// wait decoder done interrupt
    err = mtk_vcodec_wait_for_done_ctx(inst.ctx,
    MTK_INST_IRQ_RECEIVED,
    WAIT_INTR_TIMEOUT_MS, 0);
    if (err)
    goto err_free_fb_out;
    vpu_dec_end(vpu);
    memcpy(&inst.vsi_ctx, inst.vpu.vsi, sizeof(inst.vsi_ctx));
    mtk_vdec_debug(inst.ctx, "\n - NALU[%d]", inst.num_nalu);
    return 0;
    err_free_fb_out:
    mtk_vdec_err(inst.ctx, "\n - NALU[%d] err=%d -\n", inst.num_nalu, err);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn vdec_h264_slice_get_param(h_vdec: *mut c_void, type: enum vdec_get_param_type, out: *mut c_void) -> c_int {
    static int vdec_h264_slice_get_param(void *h_vdec, enum vdec_get_param_type type, void *out)
    {
    struct vdec_h264_slice_inst *inst = h_vdec;
    switch (type) {
    case GET_PARAM_PIC_INFO:
    get_pic_info(inst, out);
    break;
    case GET_PARAM_DPB_SIZE:
    get_dpb_size(inst, out);
    break;
    case GET_PARAM_CROP_INFO:
    get_crop_info(inst, out);
    break;
    default:
    mtk_vdec_err(inst.ctx, "invalid get parameter type=%d", type);
    return -EINVAL;
    }
    return 0;
    }
    const struct vdec_common_if vdec_h264_slice_if = {
    .init		= vdec_h264_slice_init,
    .decode		= vdec_h264_slice_decode,
    .get_param	= vdec_h264_slice_get_param,
    .deinit		= vdec_h264_slice_deinit,
    };
