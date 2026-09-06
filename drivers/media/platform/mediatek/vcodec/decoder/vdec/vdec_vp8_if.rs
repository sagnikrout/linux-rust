//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/mediatek/vcodec/decoder/vdec/vdec_vp8_if.c
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
// Author: Jungchang Tsao <jungchang.tsao@mediatek.com>
// PC Chen <pc.chen@mediatek.com>
//

// Decoding picture buffer size (3 reference frames plus current frame)
pub const VP8_DPB_SIZE: c_int = 4;
// HW working buffer size (bytes)

// HW control register address
pub const VP8_SEGID_DRAM_ADDR: c_uint = 0x3c;
pub const VP8_HW_VLD_ADDR: c_uint = 0x93C;
pub const VP8_HW_VLD_VALUE: c_uint = 0x940;
pub const VP8_BSASET: c_uint = 0x100;
pub const VP8_BSDSET: c_uint = 0x104;
pub const VP8_RW_CKEN_SET: c_uint = 0x0;
pub const VP8_RW_DCM_CON: c_uint = 0x18;
pub const VP8_WO_VLD_SRST: c_uint = 0x108;
pub const VP8_RW_MISC_SYS_SEL: c_uint = 0x84;
pub const VP8_RW_MISC_SPEC_CON: c_uint = 0xC8;
pub const VP8_WO_VLD_SRST: c_uint = 0x108;
pub const VP8_RW_VP8_CTRL: c_uint = 0xA4;
pub const VP8_RW_MISC_DCM_CON: c_uint = 0xEC;
pub const VP8_RW_MISC_SRST: c_uint = 0xF4;
pub const VP8_RW_MISC_FUNC_CON: c_uint = 0xCC;
pub const VP8_MAX_FRM_BUF_NUM: c_int = 5;

// required buffer size (bytes) to store decode information
pub const VP8_HW_SEGMENT_DATA_SZ: c_int = 272;
pub const VP8_HW_SEGMENT_UINT: c_int = 4;
pub const VP8_DEC_TABLE_PROC_LOOP: c_int = 96;
pub const VP8_DEC_TABLE_UNIT: c_int = 3;
pub const VP8_DEC_TABLE_SZ: c_int = 300;
pub const VP8_DEC_TABLE_OFFSET: c_int = 2;
pub const VP8_DEC_TABLE_RW_UNIT: c_int = 4;
//
// struct vdec_vp8_dec_info - decode misc information
// @working_buf_dma   : working buffer dma address
// @prev_y_dma        : previous decoded frame buffer Y plane address
// @cur_y_fb_dma      : current plane Y frame buffer dma address
// @cur_c_fb_dma      : current plane C frame buffer dma address
// @bs_dma	      : bitstream dma address
// @bs_sz	      : bitstream size
// @resolution_changed: resolution change flag 1 - changed,  0 - not changed
// @show_frame	      : display this frame or not
// @wait_key_frame    : wait key frame coming
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdec_vp8_dec_info {
    pub working_buf_dma: u64,
    pub prev_y_dma: u64,
    pub cur_y_fb_dma: u64,
    pub cur_c_fb_dma: u64,
    pub bs_dma: u64,
    pub bs_sz: u32,
    pub resolution_changed: u32,
    pub show_frame: u32,
    pub wait_key_frame: u32,
}

//
// struct vdec_vp8_vsi - VPU shared information
// @dec			: decoding information
// @pic			: picture information
// @dec_table		: decoder coefficient table
// @segment_buf		: segmentation buffer
// @load_data		: flag to indicate reload decode data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdec_vp8_vsi {
    pub dec: vdec_vp8_dec_info,
    pub pic: vdec_pic_info,
    pub dec_table: [u32; VP8_DEC_TABLE_SZ],
    pub segment_buf: [u32; VP8_HW_SEGMENT_DATA_SZ][VP8_HW_SEGMENT_UINT],
    pub load_data: u32,
}

//
// struct vdec_vp8_hw_reg_base - HW register base
// @misc	: base address for misc
// @ld		: base address for ld
// @top		: base address for top
// @cm		: base address for cm
// @hwd		: base address for hwd
// @hwb		: base address for hwb
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdec_vp8_hw_reg_base {
    pub misc: *mut void __iomem,
    pub ld: *mut void __iomem,
    pub top: *mut void __iomem,
    pub cm: *mut void __iomem,
    pub hwd: *mut void __iomem,
    pub hwb: *mut void __iomem,
}

//
// struct vdec_vp8_vpu_inst - VPU instance for VP8 decode
// @wq_hd	: Wait queue to wait VPU message ack
// @signaled	: 1 - Host has received ack message from VPU, 0 - not received
// @failure	: VPU execution result status 0 - success, others - fail
// @inst_addr	: VPU decoder instance address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdec_vp8_vpu_inst {
    pub wq_hd: wait_queue_head_t,
    pub signaled: c_int,
    pub failure: c_int,
    pub inst_addr: u32,
}

// frame buffer (fb) list
// [available_fb_node_list]  - decode fb are initialized to 0 and populated in
// [fb_use_list]  - fb is set after decode and is moved to this list
// [fb_free_list] - fb is not needed for reference will be moved from
// [fb_use_list] to [fb_free_list] and
// once user remove fb from [fb_free_list],
// it is circulated back to [available_fb_node_list]
// [fb_disp_list] - fb is set after decode and is moved to this list
// once user remove fb from [fb_disp_list] it is
// circulated back to [available_fb_node_list]
//
// struct vdec_vp8_inst - VP8 decoder instance
// @cur_fb		   : current frame buffer
// @dec_fb		   : decode frame buffer node
// @available_fb_node_list : list to store available frame buffer node
// @fb_use_list		   : list to store frame buffer in use
// @fb_free_list	   : list to store free frame buffer
// @fb_disp_list	   : list to store display ready frame buffer
// @working_buf		   : HW decoder working buffer
// @reg_base		   : HW register base address
// @frm_cnt		   : decode frame count
// @ctx			   : V4L2 context
// @vpu			   : VPU instance for decoder
// @vsi			   : VPU share information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vdec_vp8_inst {
    pub cur_fb: *mut vdec_fb,
    pub dec_fb: [vdec_fb_node; VP8_MAX_FRM_BUF_NODE_NUM],
    pub available_fb_node_list: list_head,
    pub fb_use_list: list_head,
    pub fb_free_list: list_head,
    pub fb_disp_list: list_head,
    pub working_buf: mtk_vcodec_mem,
    pub reg_base: vdec_vp8_hw_reg_base,
    pub frm_cnt: c_uint,
    pub ctx: *mut mtk_vcodec_dec_ctx,
    pub vpu: vdec_vpu_inst,
    pub vsi: *mut vdec_vp8_vsi,
}

#[no_mangle]
unsafe extern "C" fn get_hw_reg_base(inst: *mut vdec_vp8_inst) {
    static void get_hw_reg_base(struct vdec_vp8_inst *inst)
    {
    void __iomem **reg_base = inst.ctx.dev.reg_base;
    inst.reg_base.top = mtk_vcodec_get_reg_addr(reg_base, VDEC_TOP);
    inst.reg_base.cm = mtk_vcodec_get_reg_addr(reg_base, VDEC_CM);
    inst.reg_base.hwd = mtk_vcodec_get_reg_addr(reg_base, VDEC_HWD);
    inst.reg_base.misc = mtk_vcodec_get_reg_addr(reg_base, VDEC_MISC);
    inst.reg_base.ld = mtk_vcodec_get_reg_addr(reg_base, VDEC_LD);
    inst.reg_base.hwb = mtk_vcodec_get_reg_addr(reg_base, VDEC_HWB);
    }
#[no_mangle]
unsafe extern "C" fn write_hw_segmentation_data(inst: *mut vdec_vp8_inst) {
    static void write_hw_segmentation_data(struct vdec_vp8_inst *inst)
    {
    int i, j;
    u32 seg_id_addr;
    u32 val;
    void __iomem *cm = inst.reg_base.cm;
    struct vdec_vp8_vsi *vsi = inst.vsi;
    seg_id_addr = readl(inst.reg_base.top + VP8_SEGID_DRAM_ADDR) >> 4;
    for (i = 0; i < ARRAY_SIZE(vsi.segment_buf); i++) {
    for (j = ARRAY_SIZE(vsi.segment_buf[i]) - 1; j >= 0; j--) {
    val = (1 << 16) + ((seg_id_addr + i) << 2) + j;
    writel(val, cm + VP8_HW_VLD_ADDR);
    val = vsi.segment_buf[i][j];
    writel(val, cm + VP8_HW_VLD_VALUE);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn read_hw_segmentation_data(inst: *mut vdec_vp8_inst) {
    static void read_hw_segmentation_data(struct vdec_vp8_inst *inst)
    {
    int i, j;
    u32 seg_id_addr;
    u32 val;
    void __iomem *cm = inst.reg_base.cm;
    struct vdec_vp8_vsi *vsi = inst.vsi;
    seg_id_addr = readl(inst.reg_base.top + VP8_SEGID_DRAM_ADDR) >> 4;
    for (i = 0; i < ARRAY_SIZE(vsi.segment_buf); i++) {
    for (j = ARRAY_SIZE(vsi.segment_buf[i]) - 1; j >= 0; j--) {
    val = ((seg_id_addr + i) << 2) + j;
    writel(val, cm + VP8_HW_VLD_ADDR);
    val = readl(cm + VP8_HW_VLD_VALUE);
    vsi.segment_buf[i][j] = val;
    }
    }
    }
// reset HW and enable HW read/write data function
#[no_mangle]
unsafe extern "C" fn enable_hw_rw_function(inst: *mut vdec_vp8_inst) {
    static void enable_hw_rw_function(struct vdec_vp8_inst *inst)
    {
    let mut val: u32 = 0;
    void __iomem *misc = inst.reg_base.misc;
    void __iomem *ld = inst.reg_base.ld;
    void __iomem *hwb = inst.reg_base.hwb;
    void __iomem *hwd = inst.reg_base.hwd;
    mtk_vcodec_write_vdecsys(inst.ctx, VP8_RW_CKEN_SET, 0x1);
    writel(0x101, ld + VP8_WO_VLD_SRST);
    writel(0x101, hwb + VP8_WO_VLD_SRST);
    mtk_vcodec_write_vdecsys(inst.ctx, 0, 0x1);
    val = readl(misc + VP8_RW_MISC_SRST);
    writel((val & 0xFFFFFFFE), misc + VP8_RW_MISC_SRST);
    writel(0x1, misc + VP8_RW_MISC_SYS_SEL);
    writel(0x17F, misc + VP8_RW_MISC_SPEC_CON);
    writel(0x71201100, misc + VP8_RW_MISC_FUNC_CON);
    writel(0x0, ld + VP8_WO_VLD_SRST);
    writel(0x0, hwb + VP8_WO_VLD_SRST);
    mtk_vcodec_write_vdecsys(inst.ctx, VP8_RW_DCM_CON, 0x1);
    writel(0x1, misc + VP8_RW_MISC_DCM_CON);
    writel(0x1, hwd + VP8_RW_VP8_CTRL);
    }
#[no_mangle]
unsafe extern "C" fn store_dec_table(inst: *mut vdec_vp8_inst) {
    static void store_dec_table(struct vdec_vp8_inst *inst)
    {
    int i, j;
    let mut addr: u32 = 0, val = 0;
    void __iomem *hwd = inst.reg_base.hwd;
    u32 *p = &inst.vsi.dec_table[VP8_DEC_TABLE_OFFSET];
    for (i = 0; i < VP8_DEC_TABLE_PROC_LOOP; i++) {
    writel(addr, hwd + VP8_BSASET);
    for (j = 0; j < VP8_DEC_TABLE_UNIT ; j++) {
    val = *p++;
    writel(val, hwd + VP8_BSDSET);
    }
    addr += VP8_DEC_TABLE_RW_UNIT;
    }
    }
#[no_mangle]
unsafe extern "C" fn load_dec_table(inst: *mut vdec_vp8_inst) {
    static void load_dec_table(struct vdec_vp8_inst *inst)
    {
    int i;
    let mut addr: u32 = 0;
    u32 *p = &inst.vsi.dec_table[VP8_DEC_TABLE_OFFSET];
    void __iomem *hwd = inst.reg_base.hwd;
    for (i = 0; i < VP8_DEC_TABLE_PROC_LOOP; i++) {
    writel(addr, hwd + VP8_BSASET);
// read total 11 bytes
// p++ = readl(hwd + VP8_BSDSET);
// p++ = readl(hwd + VP8_BSDSET) & 0xFFFFFF;
    addr += VP8_DEC_TABLE_RW_UNIT;
    }
    }
#[no_mangle]
unsafe extern "C" fn get_pic_info(inst: *mut vdec_vp8_inst, pic: *mut vdec_pic_info) {
    static void get_pic_info(struct vdec_vp8_inst *inst, struct vdec_pic_info *pic)
    {
// pic = inst->vsi->pic;
    mtk_vdec_debug(inst.ctx, "pic(%d, %d), buf(%d, %d)",
    pic.pic_w, pic.pic_h, pic.buf_w, pic.buf_h);
    mtk_vdec_debug(inst.ctx, "fb size: Y(%d), C(%d)",
    pic.fb_sz[0], pic.fb_sz[1]);
    }
#[no_mangle]
unsafe extern "C" fn vp8_dec_finish(inst: *mut vdec_vp8_inst) {
    static void vp8_dec_finish(struct vdec_vp8_inst *inst)
    {
    struct vdec_fb_node *node;
    let mut prev_y_dma: u64 = inst.vsi.dec.prev_y_dma;
    mtk_vdec_debug(inst.ctx, "prev fb base dma=%llx", prev_y_dma);
// put last decode ok frame to fb_free_list
    if (prev_y_dma != 0) {
    list_for_each_entry(node, &inst.fb_use_list, list) {
    struct vdec_fb *fb = (struct vdec_fb *)node.fb;
    if (prev_y_dma == (uint64_t)fb.base_y.dma_addr) {
    list_move_tail(&node.list,
    &inst.fb_free_list);
    break;
    }
    }
    }
// available_fb_node_list -> fb_use_list
    node = list_first_entry(&inst.available_fb_node_list,
    struct vdec_fb_node, list);
    node.fb = inst.cur_fb;
    list_move_tail(&node.list, &inst.fb_use_list);
// available_fb_node_list -> fb_disp_list
    if (inst.vsi.dec.show_frame) {
    node = list_first_entry(&inst.available_fb_node_list,
    struct vdec_fb_node, list);
    node.fb = inst.cur_fb;
    list_move_tail(&node.list, &inst.fb_disp_list);
    }
    }
#[no_mangle]
unsafe extern "C" fn move_fb_list_use_to_free(inst: *mut vdec_vp8_inst) {
    static void move_fb_list_use_to_free(struct vdec_vp8_inst *inst)
    {
    struct vdec_fb_node *node, *tmp;
    list_for_each_entry_safe(node, tmp, &inst.fb_use_list, list)
    list_move_tail(&node.list, &inst.fb_free_list);
    }
#[no_mangle]
unsafe extern "C" fn init_list(inst: *mut vdec_vp8_inst) {
    static void init_list(struct vdec_vp8_inst *inst)
    {
    int i;
    INIT_LIST_HEAD(&inst.available_fb_node_list);
    INIT_LIST_HEAD(&inst.fb_use_list);
    INIT_LIST_HEAD(&inst.fb_free_list);
    INIT_LIST_HEAD(&inst.fb_disp_list);
    for (i = 0; i < ARRAY_SIZE(inst.dec_fb); i++) {
    INIT_LIST_HEAD(&inst.dec_fb[i].list);
    inst.dec_fb[i].fb = core::ptr::null_mut();
    list_add_tail(&inst.dec_fb[i].list,
    &inst.available_fb_node_list);
    }
    }
#[no_mangle]
unsafe extern "C" fn add_fb_to_free_list(inst: *mut vdec_vp8_inst, fb: *mut c_void) {
    static void add_fb_to_free_list(struct vdec_vp8_inst *inst, void *fb)
    {
    struct vdec_fb_node *node;
    if (fb) {
    node = list_first_entry(&inst.available_fb_node_list,
    struct vdec_fb_node, list);
    node.fb = fb;
    list_move_tail(&node.list, &inst.fb_free_list);
    }
    }
#[no_mangle]
unsafe extern "C" fn alloc_working_buf(inst: *mut vdec_vp8_inst) -> c_int {
    static int alloc_working_buf(struct vdec_vp8_inst *inst)
    {
    int err;
    struct mtk_vcodec_mem *mem = &inst.working_buf;
    mem.size = VP8_WORKING_BUF_SZ;
    err = mtk_vcodec_mem_alloc(inst.ctx, mem);
    if (err) {
    mtk_vdec_err(inst.ctx, "Cannot allocate working buffer");
    return err;
    }
    inst.vsi.dec.working_buf_dma = (uint64_t)mem.dma_addr;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn free_working_buf(inst: *mut vdec_vp8_inst) {
    static void free_working_buf(struct vdec_vp8_inst *inst)
    {
    struct mtk_vcodec_mem *mem = &inst.working_buf;
    if (mem.va)
    mtk_vcodec_mem_free(inst.ctx, mem);
    inst.vsi.dec.working_buf_dma = 0;
    }
#[no_mangle]
unsafe extern "C" fn vdec_vp8_init(ctx: *mut mtk_vcodec_dec_ctx) -> c_int {
    static int vdec_vp8_init(struct mtk_vcodec_dec_ctx *ctx)
    {
    struct vdec_vp8_inst *inst;
    int err;
    inst = kzalloc_obj(*inst);
    if (!inst)
    return  -ENOMEM;
    inst.ctx = ctx;
    inst.vpu.id = IPI_VDEC_VP8;
    inst.vpu.ctx = ctx;
    err = vpu_dec_init(&inst.vpu);
    if (err) {
    mtk_vdec_err(ctx, "vdec_vp8 init err=%d", err);
    goto error_free_inst;
    }
    inst.vsi = (struct vdec_vp8_vsi *)inst.vpu.vsi;
    init_list(inst);
    err = alloc_working_buf(inst);
    if (err)
    goto error_deinit;
    get_hw_reg_base(inst);
    mtk_vdec_debug(ctx, "VP8 Instance >> %p", inst);
    ctx.drv_handle = inst;
    return 0;
    error_deinit:
    vpu_dec_deinit(&inst.vpu);
    error_free_inst:
    kfree(inst);
    return err;
    }
    static int vdec_vp8_decode(void *h_vdec, struct mtk_vcodec_mem *bs,
    struct vdec_fb *fb, bool *res_chg)
    {
    struct vdec_vp8_inst *inst = (struct vdec_vp8_inst *)h_vdec;
    struct vdec_vp8_dec_info *dec = &inst.vsi.dec;
    struct vdec_vpu_inst *vpu = &inst.vpu;
    unsigned char *bs_va;
    unsigned int data;
    let mut err: c_int = 0;
    uint64_t y_fb_dma;
    uint64_t c_fb_dma;
// bs NULL means flush decoder
    if (bs == core::ptr::null_mut()) {
    move_fb_list_use_to_free(inst);
    return vpu_dec_reset(vpu);
    }
    y_fb_dma = fb ? (u64)fb.base_y.dma_addr : 0;
    c_fb_dma = fb ? (u64)fb.base_c.dma_addr : 0;
    mtk_vdec_debug(inst.ctx, "+ [%d] FB y_dma=%llx c_dma=%llx fb=%p",
    inst.frm_cnt, y_fb_dma, c_fb_dma, fb);
    inst.cur_fb = fb;
    dec.bs_dma = bs.dma_addr;
    dec.bs_sz = bs.size;
    dec.cur_y_fb_dma = y_fb_dma;
    dec.cur_c_fb_dma = c_fb_dma;
    mtk_vdec_debug(inst.ctx, "\n + FRAME[%d] +\n", inst.frm_cnt);
    write_hw_segmentation_data(inst);
    enable_hw_rw_function(inst);
    store_dec_table(inst);
    bs_va = (unsigned char *)bs.va;
// retrieve width/hight and scale info from header
    data = (*(bs_va + 9) << 24) | (*(bs_va + 8) << 16) |
    (*(bs_va + 7) << 8) | *(bs_va + 6);
    err = vpu_dec_start(vpu, &data, 1);
    if (err) {
    add_fb_to_free_list(inst, fb);
    if (dec.wait_key_frame) {
    mtk_vdec_debug(inst.ctx, "wait key frame !");
    return 0;
    }
    goto error;
    }
    if (dec.resolution_changed) {
    mtk_vdec_debug(inst.ctx, "- resolution_changed -");
// res_chg = true;
    add_fb_to_free_list(inst, fb);
    return 0;
    }
// wait decoder done interrupt
    mtk_vcodec_wait_for_done_ctx(inst.ctx, MTK_INST_IRQ_RECEIVED,
    WAIT_INTR_TIMEOUT_MS, 0);
    if (inst.vsi.load_data)
    load_dec_table(inst);
    vp8_dec_finish(inst);
    read_hw_segmentation_data(inst);
    err = vpu_dec_end(vpu);
    if (err)
    goto error;
    mtk_vdec_debug(inst.ctx, "\n - FRAME[%d] - show=%d\n", inst.frm_cnt, dec.show_frame);
    inst.frm_cnt++;
// res_chg = false;
    return 0;
    error:
    mtk_vdec_err(inst.ctx, "\n - FRAME[%d] - err=%d\n", inst.frm_cnt, err);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn get_disp_fb(inst: *mut vdec_vp8_inst, out_fb: *mut vdec_fb) {
    static void get_disp_fb(struct vdec_vp8_inst *inst, struct vdec_fb **out_fb)
    {
    struct vdec_fb_node *node;
    struct vdec_fb *fb;
    node = list_first_entry_or_null(&inst.fb_disp_list,
    struct vdec_fb_node, list);
    if (node) {
    list_move_tail(&node.list, &inst.available_fb_node_list);
    fb = (struct vdec_fb *)node.fb;
    fb.status |= FB_ST_DISPLAY;
    mtk_vdec_debug(inst.ctx, "[FB] get disp fb %p st=%d", node.fb, fb.status);
    } else {
    fb = core::ptr::null_mut();
    mtk_vdec_debug(inst.ctx, "[FB] there is no disp fb");
    }
// out_fb = fb;
    }
#[no_mangle]
unsafe extern "C" fn get_free_fb(inst: *mut vdec_vp8_inst, out_fb: *mut vdec_fb) {
    static void get_free_fb(struct vdec_vp8_inst *inst, struct vdec_fb **out_fb)
    {
    struct vdec_fb_node *node;
    struct vdec_fb *fb;
    node = list_first_entry_or_null(&inst.fb_free_list,
    struct vdec_fb_node, list);
    if (node) {
    list_move_tail(&node.list, &inst.available_fb_node_list);
    fb = (struct vdec_fb *)node.fb;
    fb.status |= FB_ST_FREE;
    mtk_vdec_debug(inst.ctx, "[FB] get free fb %p st=%d", node.fb, fb.status);
    } else {
    fb = core::ptr::null_mut();
    mtk_vdec_debug(inst.ctx, "[FB] there is no free fb");
    }
// out_fb = fb;
    }
#[no_mangle]
unsafe extern "C" fn get_crop_info(inst: *mut vdec_vp8_inst, cr: *mut v4l2_rect) {
    static void get_crop_info(struct vdec_vp8_inst *inst, struct v4l2_rect *cr)
    {
    cr.left = 0;
    cr.top = 0;
    cr.width = inst.vsi.pic.pic_w;
    cr.height = inst.vsi.pic.pic_h;
    mtk_vdec_debug(inst.ctx, "get crop info l=%d, t=%d, w=%d, h=%d",
    cr.left, cr.top, cr.width, cr.height);
    }
    static int vdec_vp8_get_param(void *h_vdec, enum vdec_get_param_type type,
    void *out)
    {
    struct vdec_vp8_inst *inst = (struct vdec_vp8_inst *)h_vdec;
    switch (type) {
    case GET_PARAM_DISP_FRAME_BUFFER:
    get_disp_fb(inst, out);
    break;
    case GET_PARAM_FREE_FRAME_BUFFER:
    get_free_fb(inst, out);
    break;
    case GET_PARAM_PIC_INFO:
    get_pic_info(inst, out);
    break;
    case GET_PARAM_CROP_INFO:
    get_crop_info(inst, out);
    break;
    case GET_PARAM_DPB_SIZE:
// ((unsigned int *)out) = VP8_DPB_SIZE;
    break;
    default:
    mtk_vdec_err(inst.ctx, "invalid get parameter type=%d", type);
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn vdec_vp8_deinit(h_vdec: *mut c_void) {
    static void vdec_vp8_deinit(void *h_vdec)
    {
    struct vdec_vp8_inst *inst = (struct vdec_vp8_inst *)h_vdec;
    vpu_dec_deinit(&inst.vpu);
    free_working_buf(inst);
    kfree(inst);
    }
    const struct vdec_common_if vdec_vp8_if = {
    .init		= vdec_vp8_init,
    .decode		= vdec_vp8_decode,
    .get_param	= vdec_vp8_get_param,
    .deinit		= vdec_vp8_deinit,
    };
