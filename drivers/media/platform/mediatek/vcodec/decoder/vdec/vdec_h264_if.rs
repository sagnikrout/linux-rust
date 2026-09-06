//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/mediatek/vcodec/decoder/vdec/vdec_h264_if.c
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
// Copyright (c) 2016 MediaTek Inc.
// Author: PC Chen <pc.chen@mediatek.com>
//

pub const NAL_NON_IDR_SLICE: c_uint = 0x01;
pub const NAL_IDR_SLICE: c_uint = 0x05;
pub const NAL_H264_PPS: c_uint = 0x08;

pub const MB_UNIT_LEN: c_int = 16;
// motion vector size (bytes) for every macro block
pub const HW_MB_STORE_SZ: c_int = 64;
pub const H264_MAX_FB_NUM: c_int = 17;
pub const HDR_PARSING_BUF_SZ: c_int = 1024;

pub const H264_ERR_NOT_VALID: c_int = 3;
//
// struct h264_fb - h264 decode frame buffer information
// @vdec_fb_va  : virtual address of struct vdec_fb
// @y_fb_dma    : dma address of Y frame buffer (luma)
// @c_fb_dma    : dma address of C frame buffer (chroma)
// @poc         : picture order count of frame buffer
// @reserved    : for 8 bytes alignment
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct h264_fb {
    pub vdec_fb_va: u64,
    pub y_fb_dma: u64,
    pub c_fb_dma: u64,
    pub poc: i32,
    pub reserved: u32,
}

//
// struct h264_ring_fb_list - ring frame buffer list
// @fb_list   : frame buffer array
// @read_idx  : read index
// @write_idx : write index
// @count     : buffer count in list
// @reserved  : for 8 bytes alignment
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct h264_ring_fb_list {
    pub fb_list: [h264_fb; H264_MAX_FB_NUM],
    pub read_idx: c_uint,
    pub write_idx: c_uint,
    pub count: c_uint,
    pub reserved: c_uint,
}

//
// struct vdec_h264_dec_info - decode information
// @dpb_sz		: decoding picture buffer size
// @resolution_changed  : resolution change happen
// @realloc_mv_buf	: flag to notify driver to re-allocate mv buffer
// @reserved		: for 8 bytes alignment
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
    pub reserved: u32,
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
// @hdr_buf      : Header parsing buffer (AP-W, VPU-R)
// @pred_buf_dma : HW working prediction buffer dma address (AP-W, VPU-R)
// @mv_buf_dma   : HW working motion vector buffer dma address (AP-W, VPU-R)
// @list_free    : free frame buffer ring list (AP-W/R, VPU-W)
// @list_disp    : display frame buffer ring list (AP-R, VPU-W)
// @dec          : decode information (AP-R, VPU-W)
// @pic          : picture information (AP-R, VPU-W)
// @crop         : crop information (AP-R, VPU-W)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdec_h264_vsi {
    pub hdr_buf: [c_uchar; HDR_PARSING_BUF_SZ],
    pub pred_buf_dma: u64,
    pub mv_buf_dma: [u64; H264_MAX_FB_NUM],
    pub list_free: h264_ring_fb_list,
    pub list_disp: h264_ring_fb_list,
    pub dec: vdec_h264_dec_info,
    pub pic: vdec_pic_info,
    pub crop: v4l2_rect,
}

//
// struct vdec_h264_inst - h264 decoder instance
// @num_nalu : how many nalus be decoded
// @ctx      : point to mtk_vcodec_dec_ctx
// @pred_buf : HW working prediction buffer
// @mv_buf   : HW working motion vector buffer
// @vpu      : VPU instance
// @vsi      : VPU shared information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdec_h264_inst {
    pub num_nalu: c_uint,
    pub ctx: *mut mtk_vcodec_dec_ctx,
    pub pred_buf: mtk_vcodec_mem,
    pub mv_buf: [mtk_vcodec_mem; H264_MAX_FB_NUM],
    pub vpu: vdec_vpu_inst,
    pub vsi: *mut vdec_h264_vsi,
}

#[no_mangle]
unsafe extern "C" fn get_mv_buf_size(width: c_uint, height: c_uint) -> c_uint {
    static unsigned int get_mv_buf_size(unsigned int width, unsigned int height)
    {
    return HW_MB_STORE_SZ * (width/MB_UNIT_LEN) * (height/MB_UNIT_LEN);
    }
#[no_mangle]
unsafe extern "C" fn allocate_prediction_buf(inst: *mut vdec_h264_inst) -> c_int {
    static int allocate_prediction_buf(struct vdec_h264_inst *inst)
    {
    let mut err: c_int = 0;
    inst.pred_buf.size = BUF_PREDICTION_SZ;
    err = mtk_vcodec_mem_alloc(inst.ctx, &inst.pred_buf);
    if (err) {
    mtk_vdec_err(inst.ctx, "failed to allocate ppl buf");
    return err;
    }
    inst.vsi.pred_buf_dma = inst.pred_buf.dma_addr;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn free_prediction_buf(inst: *mut vdec_h264_inst) {
    static void free_prediction_buf(struct vdec_h264_inst *inst)
    {
    struct mtk_vcodec_mem *mem = core::ptr::null_mut();
    inst.vsi.pred_buf_dma = 0;
    mem = &inst.pred_buf;
    if (mem.va)
    mtk_vcodec_mem_free(inst.ctx, mem);
    }
#[no_mangle]
unsafe extern "C" fn alloc_mv_buf(inst: *mut vdec_h264_inst, pic: *mut vdec_pic_info) -> c_int {
    static int alloc_mv_buf(struct vdec_h264_inst *inst, struct vdec_pic_info *pic)
    {
    int i;
    int err;
    struct mtk_vcodec_mem *mem = core::ptr::null_mut();
    let mut buf_sz: c_uint = get_mv_buf_size(pic.buf_w, pic.buf_h);
    for (i = 0; i < H264_MAX_FB_NUM; i++) {
    mem = &inst.mv_buf[i];
    if (mem.va)
    mtk_vcodec_mem_free(inst.ctx, mem);
    mem.size = buf_sz;
    err = mtk_vcodec_mem_alloc(inst.ctx, mem);
    if (err) {
    mtk_vdec_err(inst.ctx, "failed to allocate mv buf");
    return err;
    }
    inst.vsi.mv_buf_dma[i] = mem.dma_addr;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn free_mv_buf(inst: *mut vdec_h264_inst) {
    static void free_mv_buf(struct vdec_h264_inst *inst)
    {
    int i;
    struct mtk_vcodec_mem *mem = core::ptr::null_mut();
    for (i = 0; i < H264_MAX_FB_NUM; i++) {
    inst.vsi.mv_buf_dma[i] = 0;
    mem = &inst.mv_buf[i];
    if (mem.va)
    mtk_vcodec_mem_free(inst.ctx, mem);
    }
    }
#[no_mangle]
unsafe extern "C" fn check_list_validity(inst: *mut vdec_h264_inst, disp_list: bool) -> c_int {
    static int check_list_validity(struct vdec_h264_inst *inst, bool disp_list)
    {
    struct h264_ring_fb_list *list;
    list = disp_list ? &inst.vsi.list_disp : &inst.vsi.list_free;
    if (list.count > H264_MAX_FB_NUM ||
    list.read_idx >= H264_MAX_FB_NUM ||
    list.write_idx >= H264_MAX_FB_NUM) {
    mtk_vdec_err(inst.ctx, "%s list err: cnt=%d r_idx=%d w_idx=%d",
    disp_list ? "disp" : "free", list.count,
    list.read_idx, list.write_idx);
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn put_fb_to_free(inst: *mut vdec_h264_inst, fb: *mut vdec_fb) {
    static void put_fb_to_free(struct vdec_h264_inst *inst, struct vdec_fb *fb)
    {
    struct h264_ring_fb_list *list;
    if (fb) {
    if (check_list_validity(inst, false))
    return;
    list = &inst.vsi.list_free;
    if (list.count == H264_MAX_FB_NUM) {
    mtk_vdec_err(inst.ctx, "[FB] put fb free_list full");
    return;
    }
    mtk_vdec_debug(inst.ctx, "[FB] put fb into free_list @(%p, %llx)",
    fb.base_y.va, (u64)fb.base_y.dma_addr);
    list.fb_list[list.write_idx].vdec_fb_va = (u64)(uintptr_t)fb;
    list.write_idx = (list.write_idx == H264_MAX_FB_NUM - 1) ?
    0 : list.write_idx + 1;
    list.count++;
    }
    }
    static void get_pic_info(struct vdec_h264_inst *inst,
    struct vdec_pic_info *pic)
    {
// pic = inst->vsi->pic;
    mtk_vdec_debug(inst.ctx, "pic(%d, %d), buf(%d, %d)",
    pic.pic_w, pic.pic_h, pic.buf_w, pic.buf_h);
    mtk_vdec_debug(inst.ctx, "fb size: Y(%d), C(%d)", pic.fb_sz[0], pic.fb_sz[1]);
    }
#[no_mangle]
unsafe extern "C" fn get_crop_info(inst: *mut vdec_h264_inst, cr: *mut v4l2_rect) {
    static void get_crop_info(struct vdec_h264_inst *inst, struct v4l2_rect *cr)
    {
    cr.left = inst.vsi.crop.left;
    cr.top = inst.vsi.crop.top;
    cr.width = inst.vsi.crop.width;
    cr.height = inst.vsi.crop.height;
    mtk_vdec_debug(inst.ctx, "l=%d, t=%d, w=%d, h=%d", cr.left, cr.top,
    cr.width, cr.height);
    }
#[no_mangle]
unsafe extern "C" fn get_dpb_size(inst: *mut vdec_h264_inst, dpb_sz: *mut c_uint) {
    static void get_dpb_size(struct vdec_h264_inst *inst, unsigned int *dpb_sz)
    {
// dpb_sz = inst->vsi->dec.dpb_sz;
    mtk_vdec_debug(inst.ctx, "sz=%d", *dpb_sz);
    }
#[no_mangle]
unsafe extern "C" fn vdec_h264_init(ctx: *mut mtk_vcodec_dec_ctx) -> c_int {
    static int vdec_h264_init(struct mtk_vcodec_dec_ctx *ctx)
    {
    struct vdec_h264_inst *inst = core::ptr::null_mut();
    int err;
    inst = kzalloc_obj(*inst);
    if (!inst)
    return -ENOMEM;
    inst.ctx = ctx;
    inst.vpu.id = IPI_VDEC_H264;
    inst.vpu.ctx = ctx;
    err = vpu_dec_init(&inst.vpu);
    if (err) {
    mtk_vdec_err(ctx, "vdec_h264 init err=%d", err);
    goto error_free_inst;
    }
    inst.vsi = (struct vdec_h264_vsi *)inst.vpu.vsi;
    err = allocate_prediction_buf(inst);
    if (err)
    goto error_deinit;
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
unsafe extern "C" fn vdec_h264_deinit(h_vdec: *mut c_void) {
    static void vdec_h264_deinit(void *h_vdec)
    {
    struct vdec_h264_inst *inst = (struct vdec_h264_inst *)h_vdec;
    vpu_dec_deinit(&inst.vpu);
    free_prediction_buf(inst);
    free_mv_buf(inst);
    kfree(inst);
    }
#[no_mangle]
unsafe extern "C" fn find_start_code(data: *mut c_uchar, data_sz: c_uint) -> c_int {
    static int find_start_code(unsigned char *data, unsigned int data_sz)
    {
    if (data_sz > 3 && data[0] == 0 && data[1] == 0 && data[2] == 1)
    return 3;
    if (data_sz > 4 && data[0] == 0 && data[1] == 0 && data[2] == 0 &&
    data[3] == 1)
    return 4;
    return -1;
    }
    static int vdec_h264_decode(void *h_vdec, struct mtk_vcodec_mem *bs,
    struct vdec_fb *fb, bool *res_chg)
    {
    struct vdec_h264_inst *inst = (struct vdec_h264_inst *)h_vdec;
    struct vdec_vpu_inst *vpu = &inst.vpu;
    let mut nal_start_idx: c_int = 0;
    let mut err: c_int = 0;
    unsigned int nal_start;
    unsigned int nal_type;
    unsigned char *buf;
    unsigned int buf_sz;
    unsigned int data[2];
    let mut vdec_fb_va: u64 = (u64)(uintptr_t)fb;
    let mut y_fb_dma: u64 = fb ? (u64)fb.base_y.dma_addr : 0;
    let mut c_fb_dma: u64 = fb ? (u64)fb.base_c.dma_addr : 0;
    mtk_vdec_debug(inst.ctx, "+ [%d] FB y_dma=%llx c_dma=%llx va=%p",
    ++inst.num_nalu, y_fb_dma, c_fb_dma, fb);
// bs NULL means flush decoder
    if (bs == core::ptr::null_mut())
    return vpu_dec_reset(vpu);
    buf = (unsigned char *)bs.va;
    buf_sz = bs.size;
    nal_start_idx = find_start_code(buf, buf_sz);
    if (nal_start_idx < 0) {
    mtk_vdec_err(inst.ctx, "invalid nal start code");
    err = -EIO;
    goto err_free_fb_out;
    }
    nal_start = buf[nal_start_idx];
    nal_type = NAL_TYPE(buf[nal_start_idx]);
    mtk_vdec_debug(inst.ctx, "\n + NALU[%d] type %d +\n", inst.num_nalu,
    nal_type);
    if (nal_type == NAL_H264_PPS) {
    buf_sz -= nal_start_idx;
    if (buf_sz > HDR_PARSING_BUF_SZ) {
    err = -EILSEQ;
    goto err_free_fb_out;
    }
    memcpy(inst.vsi.hdr_buf, buf + nal_start_idx, buf_sz);
    }
    inst.vsi.dec.bs_dma = (uint64_t)bs.dma_addr;
    inst.vsi.dec.y_fb_dma = y_fb_dma;
    inst.vsi.dec.c_fb_dma = c_fb_dma;
    inst.vsi.dec.vdec_fb_va = vdec_fb_va;
    data[0] = buf_sz;
    data[1] = nal_start;
    err = vpu_dec_start(vpu, data, 2);
    if (err) {
    if (err > 0 && (DEC_ERR_RET(err) == H264_ERR_NOT_VALID)) {
    mtk_vdec_err(inst.ctx, "- error bitstream - err = %d -", err);
    err = -EIO;
    }
    goto err_free_fb_out;
    }
// res_chg = inst->vsi->dec.resolution_changed;
    if (*res_chg) {
    struct vdec_pic_info pic;
    mtk_vdec_debug(inst.ctx, "- resolution changed -");
    get_pic_info(inst, &pic);
    if (inst.vsi.dec.realloc_mv_buf) {
    err = alloc_mv_buf(inst, &pic);
    if (err)
    goto err_free_fb_out;
    }
    }
    if (nal_type == NAL_NON_IDR_SLICE || nal_type == NAL_IDR_SLICE) {
// wait decoder done interrupt
    err = mtk_vcodec_wait_for_done_ctx(inst.ctx,
    MTK_INST_IRQ_RECEIVED,
    WAIT_INTR_TIMEOUT_MS, 0);
    if (err)
    goto err_free_fb_out;
    vpu_dec_end(vpu);
    }
    mtk_vdec_debug(inst.ctx, "\n - NALU[%d] type=%d -\n", inst.num_nalu, nal_type);
    return 0;
    err_free_fb_out:
    put_fb_to_free(inst, fb);
    mtk_vdec_err(inst.ctx, "\n - NALU[%d] err=%d -\n", inst.num_nalu, err);
    return err;
    }
    static void vdec_h264_get_fb(struct vdec_h264_inst *inst,
    struct h264_ring_fb_list *list,
    bool disp_list, struct vdec_fb **out_fb)
    {
    struct vdec_fb *fb;
    if (check_list_validity(inst, disp_list))
    return;
    if (list.count == 0) {
    mtk_vdec_debug(inst.ctx, "[FB] there is no %s fb", disp_list ? "disp" : "free");
// out_fb = NULL;
    return;
    }
    fb = (struct vdec_fb *)
    (uintptr_t)list.fb_list[list.read_idx].vdec_fb_va;
    fb.status |= (disp_list ? FB_ST_DISPLAY : FB_ST_FREE);
// out_fb = fb;
    mtk_vdec_debug(inst.ctx, "[FB] get %s fb st=%d poc=%d %llx",
    disp_list ? "disp" : "free",
    fb.status, list.fb_list[list.read_idx].poc,
    list.fb_list[list.read_idx].vdec_fb_va);
    list.read_idx = (list.read_idx == H264_MAX_FB_NUM - 1) ?
    0 : list.read_idx + 1;
    list.count--;
    }
    static int vdec_h264_get_param(void *h_vdec, enum vdec_get_param_type type,
    void *out)
    {
    struct vdec_h264_inst *inst = (struct vdec_h264_inst *)h_vdec;
    switch (type) {
    case GET_PARAM_DISP_FRAME_BUFFER:
    vdec_h264_get_fb(inst, &inst.vsi.list_disp, true, out);
    break;
    case GET_PARAM_FREE_FRAME_BUFFER:
    vdec_h264_get_fb(inst, &inst.vsi.list_free, false, out);
    break;
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
    const struct vdec_common_if vdec_h264_if = {
    .init		= vdec_h264_init,
    .decode		= vdec_h264_decode,
    .get_param	= vdec_h264_get_param,
    .deinit		= vdec_h264_deinit,
    };
