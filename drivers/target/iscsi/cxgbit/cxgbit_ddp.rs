//! Automatically rewritten from C to Rust
//! Source: drivers/target/iscsi/cxgbit/cxgbit_ddp.c
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
// Copyright (c) 2016 Chelsio Communications, Inc.
//

    static void
    cxgbit_set_one_ppod(struct cxgbi_pagepod *ppod,
    struct cxgbi_task_tag_info *ttinfo,
    struct scatterlist **sg_pp, unsigned int *sg_off)
    {
    struct scatterlist *sg = sg_pp ? *sg_pp : core::ptr::null_mut();
    let mut offset: c_uint = sg_off ? *sg_off : 0;
    let mut addr: dma_addr_t = 0UL;
    let mut len: c_uint = 0;
    int i;
    memcpy(ppod, &ttinfo.hdr, sizeof(struct cxgbi_pagepod_hdr));
    if (sg) {
    addr = sg_dma_address(sg);
    len = sg_dma_len(sg);
    }
    for (i = 0; i < PPOD_PAGES_MAX; i++) {
    if (sg) {
    ppod.addr[i] = cpu_to_be64(addr + offset);
    offset += PAGE_SIZE;
    if (offset == (len + sg.offset)) {
    offset = 0;
    sg = sg_next(sg);
    if (sg) {
    addr = sg_dma_address(sg);
    len = sg_dma_len(sg);
    }
    }
    } else {
    ppod.addr[i] = 0ULL;
    }
    }
//
// the fifth address needs to be repeated in the next ppod, so do
// not move sg
//
    if (sg_pp) {
// sg_pp = sg;
// sg_off = offset;
    }
    if (offset == len) {
    offset = 0;
    if (sg) {
    sg = sg_next(sg);
    if (sg)
    addr = sg_dma_address(sg);
    }
    }
    ppod.addr[i] = sg ? cpu_to_be64(addr + offset) : 0ULL;
    }
    static struct sk_buff *
    cxgbit_ppod_init_idata(struct cxgbit_device *cdev, struct cxgbi_ppm *ppm,
    unsigned int idx, unsigned int npods, unsigned int tid)
    {
    struct ulp_mem_io *req;
    struct ulptx_idata *idata;
    let mut pm_addr: c_uint = (idx << PPOD_SIZE_SHIFT) + ppm.llimit;
    let mut dlen: c_uint = npods << PPOD_SIZE_SHIFT;
    unsigned int wr_len = roundup(sizeof(struct ulp_mem_io) +
    sizeof(struct ulptx_idata) + dlen, 16);
    struct sk_buff *skb;
    skb  = alloc_skb(wr_len, GFP_KERNEL);
    if (!skb)
    return core::ptr::null_mut();
    req = __skb_put(skb, wr_len);
    INIT_ULPTX_WR(req, wr_len, 0, tid);
    req.wr.wr_hi = htonl(FW_WR_OP_V(FW_ULPTX_WR) |
    FW_WR_ATOMIC_V(0));
    req.cmd = htonl(ULPTX_CMD_V(ULP_TX_MEM_WRITE) |
    ULP_MEMIO_ORDER_V(0) |
    T5_ULP_MEMIO_IMM_V(1));
    req.dlen = htonl(ULP_MEMIO_DATA_LEN_V(dlen >> 5));
    req.lock_addr = htonl(ULP_MEMIO_ADDR_V(pm_addr >> 5));
    req.len16 = htonl(DIV_ROUND_UP(wr_len - sizeof(req.wr), 16));
    idata = (struct ulptx_idata *)(req + 1);
    idata.cmd_more = htonl(ULPTX_CMD_V(ULP_TX_SC_IMM));
    idata.len = htonl(dlen);
    return skb;
    }
    static int
    cxgbit_ppod_write_idata(struct cxgbi_ppm *ppm, struct cxgbit_sock *csk,
    struct cxgbi_task_tag_info *ttinfo, unsigned int idx,
    unsigned int npods, struct scatterlist **sg_pp,
    unsigned int *sg_off)
    {
    struct cxgbit_device *cdev = csk.com.cdev;
    struct sk_buff *skb;
    struct ulp_mem_io *req;
    struct ulptx_idata *idata;
    struct cxgbi_pagepod *ppod;
    unsigned int i;
    skb = cxgbit_ppod_init_idata(cdev, ppm, idx, npods, csk.tid);
    if (!skb)
    return -ENOMEM;
    req = (struct ulp_mem_io *)skb.data;
    idata = (struct ulptx_idata *)(req + 1);
    ppod = (struct cxgbi_pagepod *)(idata + 1);
    for (i = 0; i < npods; i++, ppod++)
    cxgbit_set_one_ppod(ppod, ttinfo, sg_pp, sg_off);
    __skb_queue_tail(&csk.ppodq, skb);
    return 0;
    }
    static int
    cxgbit_ddp_set_map(struct cxgbi_ppm *ppm, struct cxgbit_sock *csk,
    struct cxgbi_task_tag_info *ttinfo)
    {
    let mut pidx: c_uint = ttinfo.idx;
    let mut npods: c_uint = ttinfo.npods;
    unsigned int i, cnt;
    struct scatterlist *sg = ttinfo.sgl;
    let mut offset: c_uint = 0;
    let mut ret: c_int = 0;
    for (i = 0; i < npods; i += cnt, pidx += cnt) {
    cnt = npods - i;
    if (cnt > ULPMEM_IDATA_MAX_NPPODS)
    cnt = ULPMEM_IDATA_MAX_NPPODS;
    ret = cxgbit_ppod_write_idata(ppm, csk, ttinfo, pidx, cnt,
    &sg, &offset);
    if (ret < 0)
    break;
    }
    return ret;
    }
    static int cxgbit_ddp_sgl_check(struct scatterlist *sg,
    unsigned int nents)
    {
    let mut last_sgidx: c_uint = nents - 1;
    unsigned int i;
    for (i = 0; i < nents; i++, sg = sg_next(sg)) {
    let mut len: c_uint = sg.length + sg.offset;
    if ((sg.offset & 0x3) || (i && sg.offset) ||
    ((i != last_sgidx) && (len != PAGE_SIZE))) {
    return -EINVAL;
    }
    }
    return 0;
    }
    static int
    cxgbit_ddp_reserve(struct cxgbit_sock *csk, struct cxgbi_task_tag_info *ttinfo,
    unsigned int xferlen)
    {
    struct cxgbit_device *cdev = csk.com.cdev;
    struct cxgbi_ppm *ppm = cdev2ppm(cdev);
    struct scatterlist *sgl = ttinfo.sgl;
    let mut sgcnt: c_uint = ttinfo.nents;
    let mut sg_offset: c_uint = sgl.offset;
    int ret;
    if ((xferlen < DDP_THRESHOLD) || (!sgcnt)) {
    pr_debug("ppm 0x%p, pgidx %u, xfer %u, sgcnt %u, NO ddp.\n",
    ppm, ppm.tformat.pgsz_idx_dflt,
    xferlen, ttinfo.nents);
    return -EINVAL;
    }
    if (cxgbit_ddp_sgl_check(sgl, sgcnt) < 0)
    return -EINVAL;
    ttinfo.nr_pages = (xferlen + sgl.offset +
    (1 << PAGE_SHIFT) - 1) >> PAGE_SHIFT;
//
// the ddp tag will be used for the ttt in the outgoing r2t pdu
//
    ret = cxgbi_ppm_ppods_reserve(ppm, ttinfo.nr_pages, 0, &ttinfo.idx,
    &ttinfo.tag, 0);
    if (ret < 0)
    return ret;
    ttinfo.npods = ret;
    sgl.offset = 0;
    ret = dma_map_sg(&ppm.pdev.dev, sgl, sgcnt, DMA_FROM_DEVICE);
    sgl.offset = sg_offset;
    if (!ret) {
    pr_debug("%s: 0x%x, xfer %u, sgl %u dma mapping err.\n",
    __func__, 0, xferlen, sgcnt);
    goto rel_ppods;
    }
    cxgbi_ppm_make_ppod_hdr(ppm, ttinfo.tag, csk.tid, sgl.offset,
    xferlen, &ttinfo.hdr);
    ret = cxgbit_ddp_set_map(ppm, csk, ttinfo);
    if (ret < 0) {
    __skb_queue_purge(&csk.ppodq);
    dma_unmap_sg(&ppm.pdev.dev, sgl, sgcnt, DMA_FROM_DEVICE);
    goto rel_ppods;
    }
    return 0;
    rel_ppods:
    cxgbi_ppm_ppod_release(ppm, ttinfo.idx);
    return -EINVAL;
    }
    void
    cxgbit_get_r2t_ttt(struct iscsit_conn *conn, struct iscsit_cmd *cmd,
    struct iscsi_r2t *r2t)
    {
    struct cxgbit_sock *csk = conn.context;
    struct cxgbit_device *cdev = csk.com.cdev;
    struct cxgbit_cmd *ccmd = iscsit_priv_cmd(cmd);
    struct cxgbi_task_tag_info *ttinfo = &ccmd.ttinfo;
    int ret;
    if ((!ccmd.setup_ddp) ||
    (!test_bit(CSK_DDP_ENABLE, &csk.com.flags)))
    goto out;
    ccmd.setup_ddp = false;
    ttinfo.sgl = cmd.se_cmd.t_data_sg;
    ttinfo.nents = cmd.se_cmd.t_data_nents;
    ret = cxgbit_ddp_reserve(csk, ttinfo, cmd.se_cmd.data_length);
    if (ret < 0) {
    pr_debug("csk 0x%p, cmd 0x%p, xfer len %u, sgcnt %u no ddp.\n",
    csk, cmd, cmd.se_cmd.data_length, ttinfo.nents);
    ttinfo.sgl = core::ptr::null_mut();
    ttinfo.nents = 0;
    } else {
    ccmd.release = true;
    }
    out:
    pr_debug("cdev 0x%p, cmd 0x%p, tag 0x%x\n", cdev, cmd, ttinfo.tag);
    r2t.targ_xfer_tag = ttinfo.tag;
    }
#[no_mangle]
pub unsafe extern "C" fn cxgbit_unmap_cmd(conn: *mut iscsit_conn, cmd: *mut iscsit_cmd) {
    void cxgbit_unmap_cmd(struct iscsit_conn *conn, struct iscsit_cmd *cmd)
    {
    struct cxgbit_cmd *ccmd = iscsit_priv_cmd(cmd);
    if (ccmd.release) {
    if (cmd.se_cmd.se_cmd_flags & SCF_PASSTHROUGH_SG_TO_MEM_NOALLOC) {
    put_page(sg_page(&ccmd.sg));
    } else {
    struct cxgbit_sock *csk = conn.context;
    struct cxgbit_device *cdev = csk.com.cdev;
    struct cxgbi_ppm *ppm = cdev2ppm(cdev);
    struct cxgbi_task_tag_info *ttinfo = &ccmd.ttinfo;
// Abort the TCP conn if DDP is not complete to
// avoid any possibility of DDP after freeing
// the cmd.
//
    if (unlikely(cmd.write_data_done !=
    cmd.se_cmd.data_length))
    cxgbit_abort_conn(csk);
    if (unlikely(ttinfo.sgl)) {
    dma_unmap_sg(&ppm.pdev.dev, ttinfo.sgl,
    ttinfo.nents, DMA_FROM_DEVICE);
    ttinfo.nents = 0;
    ttinfo.sgl = core::ptr::null_mut();
    }
    cxgbi_ppm_ppod_release(ppm, ttinfo.idx);
    }
    ccmd.release = false;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn cxgbit_ddp_init(cdev: *mut cxgbit_device) -> c_int {
    int cxgbit_ddp_init(struct cxgbit_device *cdev)
    {
    struct cxgb4_lld_info *lldi = &cdev.lldi;
    struct net_device *ndev = cdev.lldi.ports[0];
    struct cxgbi_tag_format tformat;
    int ret, i;
    if (!lldi.vr.iscsi.size) {
    pr_warn("%s, iscsi NOT enabled, check config!\n", ndev.name);
    return -EACCES;
    }
    memset(&tformat, 0, sizeof(struct cxgbi_tag_format));
    for (i = 0; i < 4; i++)
    tformat.pgsz_order[i] = (lldi.iscsi_pgsz_order >> (i << 3))
    & 0xF;
    cxgbi_tagmask_check(lldi.iscsi_tagmask, &tformat);
    ret = cxgbi_ppm_init(lldi.iscsi_ppm, cdev.lldi.ports[0],
    cdev.lldi.pdev, &cdev.lldi, &tformat,
    lldi.vr.iscsi.size, lldi.iscsi_llimit,
    lldi.vr.iscsi.start, 2,
    lldi.vr.ppod_edram.start,
    lldi.vr.ppod_edram.size);
    if (ret >= 0) {
    struct cxgbi_ppm *ppm = (struct cxgbi_ppm *)(*lldi.iscsi_ppm);
    if ((ppm.tformat.pgsz_idx_dflt < DDP_PGIDX_MAX) &&
    (ppm.ppmax >= 1024))
    set_bit(CDEV_DDP_ENABLE, &cdev.flags);
    ret = 0;
    }
    return ret;
    }
