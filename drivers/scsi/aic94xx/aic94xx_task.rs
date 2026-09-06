//! Automatically rewritten from C to Rust
//! Source: drivers/scsi/aic94xx/aic94xx_task.c
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
// Aic94xx SAS/SATA Tasks
//
// Copyright (C) 2005 Adaptec, Inc.  All rights reserved.
// Copyright (C) 2005 Luben Tuikov <luben_tuikov@adaptec.com>
//

    static void asd_unbuild_ata_ascb(struct asd_ascb *a);
    static void asd_unbuild_smp_ascb(struct asd_ascb *a);
    static void asd_unbuild_ssp_ascb(struct asd_ascb *a);
#[no_mangle]
unsafe extern "C" fn asd_can_dequeue(asd_ha: *mut asd_ha_struct, num: c_int) {
    static void asd_can_dequeue(struct asd_ha_struct *asd_ha, int num)
    {
    unsigned long flags;
    spin_lock_irqsave(&asd_ha.seq.pend_q_lock, flags);
    asd_ha.seq.can_queue += num;
    spin_unlock_irqrestore(&asd_ha.seq.pend_q_lock, flags);
    }
// DMA_... to our direction translation.
//
    static const u8 data_dir_flags[] = {
    [DMA_BIDIRECTIONAL]	= DATA_DIR_BYRECIPIENT,	/* UNSPECIFIED */
    [DMA_TO_DEVICE]		= DATA_DIR_OUT,		/* OUTBOUND */
    [DMA_FROM_DEVICE]	= DATA_DIR_IN,		/* INBOUND */
    [DMA_NONE]		= DATA_DIR_NONE,	/* NO TRANSFER */
    };
    static int asd_map_scatterlist(struct sas_task *task,
    struct sg_el *sg_arr,
    gfp_t gfp_flags)
    {
    struct asd_ascb *ascb = task.lldd_task;
    struct asd_ha_struct *asd_ha = ascb.ha;
    struct scatterlist *sc;
    int num_sg, res;
    if (task.data_dir == DMA_NONE)
    return 0;
    if (task.num_scatter == 0) {
    void *p = task.scatter;
    dma_addr_t dma = dma_map_single(&asd_ha.pcidev.dev, p,
    task.total_xfer_len,
    task.data_dir);
    if (dma_mapping_error(&asd_ha.pcidev.dev, dma))
    return -ENOMEM;
    sg_arr[0].bus_addr = cpu_to_le64((u64)dma);
    sg_arr[0].size = cpu_to_le32(task.total_xfer_len);
    sg_arr[0].flags |= ASD_SG_EL_LIST_EOL;
    return 0;
    }
// STP tasks come from libata which has already mapped
// the SG list
    if (sas_protocol_ata(task.task_proto))
    num_sg = task.num_scatter;
    else
    num_sg = dma_map_sg(&asd_ha.pcidev.dev, task.scatter,
    task.num_scatter, task.data_dir);
    if (num_sg == 0)
    return -ENOMEM;
    if (num_sg > 3) {
    int i;
    ascb.sg_arr = asd_alloc_coherent(asd_ha,
    num_sg*sizeof(struct sg_el),
    gfp_flags);
    if (!ascb.sg_arr) {
    res = -ENOMEM;
    goto err_unmap;
    }
    for_each_sg(task.scatter, sc, num_sg, i) {
    struct sg_el *sg =
    &((struct sg_el *)ascb.sg_arr.vaddr)[i];
    sg.bus_addr = cpu_to_le64((u64)sg_dma_address(sc));
    sg.size = cpu_to_le32((u32)sg_dma_len(sc));
    if (i == num_sg-1)
    sg.flags |= ASD_SG_EL_LIST_EOL;
    }
    for_each_sg(task.scatter, sc, 2, i) {
    sg_arr[i].bus_addr =
    cpu_to_le64((u64)sg_dma_address(sc));
    sg_arr[i].size = cpu_to_le32((u32)sg_dma_len(sc));
    }
    sg_arr[1].next_sg_offs = 2 * sizeof(*sg_arr);
    sg_arr[1].flags |= ASD_SG_EL_LIST_EOS;
    memset(&sg_arr[2], 0, sizeof(*sg_arr));
    sg_arr[2].bus_addr=cpu_to_le64((u64)ascb.sg_arr.dma_handle);
    } else {
    int i;
    for_each_sg(task.scatter, sc, num_sg, i) {
    sg_arr[i].bus_addr =
    cpu_to_le64((u64)sg_dma_address(sc));
    sg_arr[i].size = cpu_to_le32((u32)sg_dma_len(sc));
    }
    sg_arr[i-1].flags |= ASD_SG_EL_LIST_EOL;
    }
    return 0;
    err_unmap:
    if (sas_protocol_ata(task.task_proto))
    dma_unmap_sg(&asd_ha.pcidev.dev, task.scatter,
    task.num_scatter, task.data_dir);
    return res;
    }
#[no_mangle]
unsafe extern "C" fn asd_unmap_scatterlist(ascb: *mut asd_ascb) {
    static void asd_unmap_scatterlist(struct asd_ascb *ascb)
    {
    struct asd_ha_struct *asd_ha = ascb.ha;
    struct sas_task *task = ascb.uldd_task;
    if (task.data_dir == DMA_NONE)
    return;
    if (task.num_scatter == 0) {
    dma_addr_t dma = (dma_addr_t)
    le64_to_cpu(ascb.scb.ssp_task.sg_element[0].bus_addr);
    dma_unmap_single(&ascb.ha.pcidev.dev, dma,
    task.total_xfer_len, task.data_dir);
    return;
    }
    asd_free_coherent(asd_ha, ascb.sg_arr);
    if (task.task_proto != SAS_PROTOCOL_STP)
    dma_unmap_sg(&asd_ha.pcidev.dev, task.scatter,
    task.num_scatter, task.data_dir);
    }
// ---------- Task complete tasklet ----------
    static void asd_get_response_tasklet(struct asd_ascb *ascb,
    struct done_list_struct *dl)
    {
    struct asd_ha_struct *asd_ha = ascb.ha;
    struct sas_task *task = ascb.uldd_task;
    struct task_status_struct *ts = &task.task_status;
    unsigned long flags;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_resp_sb_struct {
    pub index_escb: __le16,
    pub len_lsb: u8,
    pub flags: u8,
    pub dl->status_block: *mut *mut *mut } __attribute__ ((packed)) resp_sb = (void ),
// int  size   = ((resp_sb->flags & 7) << 8) | resp_sb->len_lsb;
    pub 4)-1: int edb_id = ((resp_sb->flags & 0x70) >>,
    pub escb: *mut asd_ascb,
    pub edb: *mut asd_dma_tok,
    pub r: *mut c_void,
    pub flags): spin_lock_irqsave(&asd_ha->seq.tc_index_lock,,
    escb = asd_tc_index_find(&asd_ha.seq,
    pub flags): spin_unlock_irqrestore(&asd_ha->seq.tc_index_lock,,
    if (!escb) {
    pub dl?!\n"): ASD_DPRINTK("Uh-oh! No escb for this,
    }
    pub 0: ts->buf_valid_size =,
    pub escb->edb_index]: edb = asd_ha->seq.edb_arr[edb_id +,
    pub edb->vaddr: r =,
    if (task.task_proto == SAS_PROTOCOL_SSP) {
    struct ssp_response_iu *iu =
    pub ssp_frame_hdr): r + 16 + sizeof(struct,
    pub )r): *mut *mut ts->residual = le32_to_cpu((__le32,
    pub iu): sas_ssp_task_response(&asd_ha->pcidev->dev, task,,
    }  else {
    pub &ts->buf[0]: *mut *mut *mut ata_task_resp resp = (void ),
    pub )r): *mut *mut ts->residual = le32_to_cpu((__le32,
    if (SAS_STATUS_BUF_SIZE >= sizeof(*resp)) {
    pub )(r+6)): *mut *mut resp->frame_len = le16_to_cpu((__le16,
    pub ATA_RESP_FIS_SIZE): memcpy(&resp->ending_fis[0], r+16,,
    pub sizeof(*resp): *mut ts->buf_valid_size =,
    }
    }
    pub edb_id): asd_invalidate_edb(escb,,
    }
    static void asd_task_tasklet_complete(struct asd_ascb *ascb,
    struct done_list_struct *dl)
    {
    pub ascb->uldd_task: *mut *mut sas_task task =,
    pub &task->task_status: *mut *mut task_status_ts =,
    pub flags: c_ulong,
    pub dl->opcode: u8 opcode =,
    pub 1): asd_can_dequeue(ascb->ha,,
    Again:
    switch (opcode) {
    case TC_NO_ERROR:
    pub SAS_TASK_COMPLETE: ts->resp =,
    pub SAS_SAM_STAT_GOOD: ts->stat =,
    case TC_UNDERRUN:
    pub SAS_TASK_COMPLETE: ts->resp =,
    pub SAS_DATA_UNDERRUN: ts->stat =,
    pub )dl->status_block): *mut *mut ts->residual = le32_to_cpu((__le32,
    case TC_OVERRUN:
    pub SAS_TASK_COMPLETE: ts->resp =,
    pub SAS_DATA_OVERRUN: ts->stat =,
    pub 0: ts->residual =,
    case TC_SSP_RESP:
    case TC_ATA_RESP:
    pub SAS_TASK_COMPLETE: ts->resp =,
    pub SAS_PROTO_RESPONSE: ts->stat =,
    pub dl): asd_get_response_tasklet(ascb,,
    case TF_OPEN_REJECT:
    pub SAS_TASK_UNDELIVERED: ts->resp =,
    pub SAS_OPEN_REJECT: ts->stat =,
    if (dl.status_block[1] & 2)
    pub dl->status_block[2]: ts->open_rej_reason = 1 +,
#[no_mangle]
pub unsafe extern "C" fn if(1: dl->status_block[1] &) -> else {
    else if (dl.status_block[1] & 1)
    pub 4)+10: ts->open_rej_reason = (dl->status_block[2] >>,
    else
    pub SAS_OREJ_UNKNOWN: ts->open_rej_reason =,
    case TF_OPEN_TO:
    pub SAS_TASK_UNDELIVERED: ts->resp =,
    pub SAS_OPEN_TO: ts->stat =,
    case TF_PHY_DOWN:
    case TU_PHY_DOWN:
    pub SAS_TASK_UNDELIVERED: ts->resp =,
    pub SAS_PHY_DOWN: ts->stat =,
    case TI_PHY_DOWN:
    pub SAS_TASK_COMPLETE: ts->resp =,
    pub SAS_PHY_DOWN: ts->stat =,
    case TI_BREAK:
    case TI_PROTO_ERR:
    case TI_NAK:
    case TI_ACK_NAK_TO:
    case TF_SMP_XMIT_RCV_ERR:
    case TC_ATA_R_ERR_RECV:
    pub SAS_TASK_COMPLETE: ts->resp =,
    pub SAS_INTERRUPTED: ts->stat =,
    case TF_BREAK:
    case TU_BREAK:
    case TU_ACK_NAK_TO:
    case TF_SMPRSP_TO:
    pub SAS_TASK_UNDELIVERED: ts->resp =,
    pub SAS_DEV_NO_RESPONSE: ts->stat =,
    case TF_NAK_RECV:
    pub SAS_TASK_COMPLETE: ts->resp =,
    pub SAS_NAK_R_ERR: ts->stat =,
    case TA_I_T_NEXUS_LOSS:
    pub dl->status_block[0]: opcode =,
    pub Again: goto,
    case TF_INV_CONN_HANDLE:
    pub SAS_TASK_UNDELIVERED: ts->resp =,
    pub SAS_DEVICE_UNKNOWN: ts->stat =,
    case TF_REQUESTED_N_PENDING:
    pub SAS_TASK_UNDELIVERED: ts->resp =,
    pub SAS_PENDING: ts->stat =,
    case TC_TASK_CLEARED:
    case TA_ON_REQ:
    pub SAS_TASK_COMPLETE: ts->resp =,
    pub SAS_ABORTED_TASK: ts->stat =,
    case TF_NO_SMP_CONN:
    case TF_TMF_NO_CTX:
    case TF_TMF_NO_TAG:
    case TF_TMF_TAG_FREE:
    case TF_TMF_TASK_DONE:
    case TF_TMF_NO_CONN_HANDLE:
    case TF_IRTT_TO:
    case TF_IU_SHORT:
    case TF_DATA_OFFS_ERR:
    pub SAS_TASK_UNDELIVERED: ts->resp =,
    pub SAS_DEV_NO_RESPONSE: ts->stat =,
    case TC_LINK_ADM_RESP:
    case TC_CONTROL_PHY:
    case TC_RESUME:
    case TC_PARTIAL_SG_LIST:
    default:
    pub opcode): ASD_DPRINTK("%s: dl opcode: 0x%x?\n", __func__,,
    }
    switch (task.task_proto) {
    case SAS_PROTOCOL_SATA:
    case SAS_PROTOCOL_STP:
    case SAS_PROTOCOL_SMP:
    case SAS_PROTOCOL_SSP:
    default:
    }
    pub flags): spin_lock_irqsave(&task->task_state_lock,,
    pub ~SAS_TASK_STATE_PENDING: task->task_state_flags &=,
    pub SAS_TASK_STATE_DONE: task->task_state_flags |=,
    if (unlikely((task.task_state_flags & SAS_TASK_STATE_ABORTED))) {
    pub ascb->completion: *mut *mut completion completion =,
    pub flags): spin_unlock_irqrestore(&task->task_state_lock,,
    ASD_DPRINTK("task 0x%p done with opcode 0x%x resp 0x%x "
    "stat 0x%x but aborted by upper layer!\n",
    pub ts->stat): task, opcode, ts->resp,,
    if (completion)
    } else {
    pub flags): spin_unlock_irqrestore(&task->task_state_lock,,
    pub NULL: task->lldd_task =,
    }
    }
// ---------- ATA ----------
    static int asd_build_ata_ascb(struct asd_ascb *ascb, struct sas_task *task,
    gfp_t gfp_flags)
    {
    pub task->dev: *mut *mut domain_device dev =,
    pub scb: *mut scb,
    pub flags: u8,
    pub 0: int res =,
    pub ascb->scb: scb =,
    if (unlikely(task.ata_task.device_control_reg_update))
    pub CONTROL_ATA_DEV: scb->header.opcode =,
#[no_mangle]
pub unsafe extern "C" fn if(ATA_DEV_ATAPI: dev->sata_dev.class ==) -> else {
    else if (dev.sata_dev.class == ATA_DEV_ATAPI)
    pub INITIATE_ATAPI_TASK: scb->header.opcode =,
    else
    pub INITIATE_ATA_TASK: scb->header.opcode =,
    pub /: *mut *mut scb->ata_task.proto_conn_rate = (1 << 5); / STP,
    if (dev.port.oob_mode == SAS_OOB_MODE)
    pub dev->linkrate: scb->ata_task.proto_conn_rate |=,
    pub cpu_to_le32(task->total_xfer_len): scb->ata_task.total_xfer_len =,
    pub task->ata_task.fis: scb->ata_task.fis =,
    if (likely(!task.ata_task.device_control_reg_update))
    pub /: *mut *mut scb->ata_task.fis.flags |= 0x80; / C=1: update ATA cmd reg,
    pub /: *mut *mut scb->ata_task.fis.flags &= 0xF0; / PM_PORT field shall be 0,
    if (dev.sata_dev.class == ATA_DEV_ATAPI)
    memcpy(scb.ata_task.atapi_packet, task.ata_task.atapi_packet,
    pub cpu_to_le16(0xFFFF): scb->ata_task.sister_scb =,
    scb.ata_task.conn_handle = cpu_to_le16(
    pub long)dev->lldd_dev): (u16)(unsigned,
    if (likely(!task.ata_task.device_control_reg_update)) {
    pub 0: flags =,
    if (task.ata_task.dma_xfer)
    pub DATA_XFER_MODE_DMA: flags |=,
    if (task.ata_task.use_ncq &&
    dev.sata_dev.class != ATA_DEV_ATAPI)
    pub ATA_Q_TYPE_NCQ: flags |=,
    pub data_dir_flags: [flags |=; task->data_dir],
    pub flags: scb->ata_task.ata_flags =,
    pub 0: scb->ata_task.retry_count =,
    pub 0: scb->ata_task.flags =,
    }
    pub asd_task_tasklet_complete: ascb->tasklet_complete =,
    if (likely(!task.ata_task.device_control_reg_update))
    res = asd_map_scatterlist(task, scb.ata_task.sg_element,
    pub res: return,
    }
#[no_mangle]
unsafe extern "C" fn asd_unbuild_ata_ascb(a: *mut asd_ascb) {
    static void asd_unbuild_ata_ascb(struct asd_ascb *a)
    {
    }
// ---------- SMP ----------
    static int asd_build_smp_ascb(struct asd_ascb *ascb, struct sas_task *task,
    gfp_t gfp_flags)
    {
    pub ascb->ha: *mut *mut asd_ha_asd_ha =,
    pub task->dev: *mut *mut domain_device dev =,
    pub scb: *mut scb,
    dma_map_sg(&asd_ha.pcidev.dev, &task.smp_task.smp_req, 1,
    dma_map_sg(&asd_ha.pcidev.dev, &task.smp_task.smp_resp, 1,
    pub ascb->scb: scb =,
    pub INITIATE_SMP_TASK: scb->header.opcode =,
    pub dev->linkrate: scb->smp_task.proto_conn_rate =,
    scb.smp_task.smp_req.bus_addr =
    scb.smp_task.smp_req.size =
    scb.smp_task.smp_resp.bus_addr =
    scb.smp_task.smp_resp.size =
    pub cpu_to_le16(0xFFFF): scb->smp_task.sister_scb =,
    scb.smp_task.conn_handle = cpu_to_le16((u16)
    pub long)dev->lldd_dev): (unsigned,
    pub asd_task_tasklet_complete: ascb->tasklet_complete =,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn asd_unbuild_smp_ascb(a: *mut asd_ascb) {
    static void asd_unbuild_smp_ascb(struct asd_ascb *a)
    {
    pub a->uldd_task: *mut *mut sas_task task =,
    dma_unmap_sg(&a.ha.pcidev.dev, &task.smp_task.smp_req, 1,
    dma_unmap_sg(&a.ha.pcidev.dev, &task.smp_task.smp_resp, 1,
    }
// ---------- SSP ----------
    static int asd_build_ssp_ascb(struct asd_ascb *ascb, struct sas_task *task,
    gfp_t gfp_flags)
    {
    pub task->dev: *mut *mut domain_device dev =,
    pub scb: *mut scb,
    pub 0: int res =,
    pub ascb->scb: scb =,
    pub INITIATE_SSP_TASK: scb->header.opcode =,
    pub /: *mut *mut scb->ssp_task.proto_conn_rate = (1 << 4); / SSP,
    pub dev->linkrate: scb->ssp_task.proto_conn_rate |=,
    pub cpu_to_le32(task->total_xfer_len): scb->ssp_task.total_xfer_len =,
    pub SSP_DATA: scb->ssp_task.ssp_frame.frame_type =,
    memcpy(scb.ssp_task.ssp_frame.hashed_dest_addr, dev.hashed_sas_addr,
    memcpy(scb.ssp_task.ssp_frame.hashed_src_addr,
    pub HASHED_SAS_ADDR_SIZE): dev->port->ha->hashed_sas_addr,,
    pub cpu_to_be16(0xFFFF): scb->ssp_task.ssp_frame.tptt =,
    pub 8): memcpy(scb->ssp_task.ssp_cmd.lun, task->ssp_task.LUN,,
    pub 7): scb->ssp_task.ssp_cmd.efb_prio_attr |= (task->ssp_task.task_attr &,
    memcpy(scb.ssp_task.ssp_cmd.cdb, task.ssp_task.cmd.cmnd,
    pub cpu_to_le16(0xFFFF): scb->ssp_task.sister_scb =,
    scb.ssp_task.conn_handle = cpu_to_le16(
    pub long)dev->lldd_dev): (u16)(unsigned,
    pub data_dir_flags: [scb->ssp_task.data_dir =; task->data_dir],
    pub asd_task_tasklet_complete: ascb->tasklet_complete =,
    pub gfp_flags): res = asd_map_scatterlist(task, scb->ssp_task.sg_element,,
    pub res: return,
    }
#[no_mangle]
unsafe extern "C" fn asd_unbuild_ssp_ascb(a: *mut asd_ascb) {
    static void asd_unbuild_ssp_ascb(struct asd_ascb *a)
    {
    }
// ---------- Execute Task ----------
#[no_mangle]
unsafe extern "C" fn asd_can_queue(asd_ha: *mut asd_ha_struct, num: c_int) -> c_int {
    static int asd_can_queue(struct asd_ha_struct *asd_ha, int num)
    {
    pub 0: int res =,
    pub flags: c_ulong,
    pub flags): spin_lock_irqsave(&asd_ha->seq.pend_q_lock,,
    if ((asd_ha.seq.can_queue - num) < 0)
    pub -SAS_QUEUE_FULL: res =,
    else
    pub num: asd_ha->seq.can_queue -=,
    pub flags): spin_unlock_irqrestore(&asd_ha->seq.pend_q_lock,,
    pub res: return,
    }
#[no_mangle]
pub unsafe extern "C" fn asd_execute_task(task: *mut sas_task, gfp_flags: gfp_t) -> c_int {
    int asd_execute_task(struct sas_task *task, gfp_t gfp_flags)
    {
    pub 0: int res =,
    pub task: *mut *mut sas_task t =,
    pub a: *mut *mut asd_ascb ascb = NULL,,
    pub task->dev->port->ha->lldd_ha: *mut *mut asd_ha_asd_ha =,
    pub 1): res = asd_can_queue(asd_ha,,
    if (res)
    pub res: return,
    pub 1: res =,
    pub gfp_flags): ascb = asd_ascb_alloc_list(asd_ha, &res,,
    if (res) {
    pub -ENOMEM: res =,
    pub out_err: goto,
    }
    pub &ascb->list): __list_add(&alist, ascb->list.prev,,
    list_for_each_entry(a, &alist, list) {
    pub t: a->uldd_task =,
    pub a: t->lldd_task =,
    }
    list_for_each_entry(a, &alist, list) {
    pub a->uldd_task: t =,
    pub 1: a->uldd_timer =,
    if (t.task_proto & SAS_PROTOCOL_STP)
    pub SAS_PROTOCOL_STP: t->task_proto =,
    switch (t.task_proto) {
    case SAS_PROTOCOL_SATA:
    case SAS_PROTOCOL_STP:
    pub gfp_flags): res = asd_build_ata_ascb(a, t,,
    case SAS_PROTOCOL_SMP:
    pub gfp_flags): res = asd_build_smp_ascb(a, t,,
    case SAS_PROTOCOL_SSP:
    pub gfp_flags): res = asd_build_ssp_ascb(a, t,,
    default:
    asd_printk("unknown sas_task proto: 0x%x\n",
    pub -ENOMEM: res =,
    }
    if (res)
    pub out_err_unmap: goto,
    }
    pub 1): res = asd_post_ascb_list(asd_ha, ascb,,
    if (unlikely(res)) {
    pub NULL: a =,
    pub &ascb->list): __list_add(&alist, ascb->list.prev,,
    pub out_err_unmap: goto,
    }
    pub 0: return,
    out_err_unmap:
    {
    pub a: *mut *mut asd_ascb b =,
    list_for_each_entry(a, &alist, list) {
    if (a == b)
    pub a->uldd_task: t =,
    switch (t.task_proto) {
    case SAS_PROTOCOL_SATA:
    case SAS_PROTOCOL_STP:
    case SAS_PROTOCOL_SMP:
    case SAS_PROTOCOL_SSP:
    default:
    }
    pub NULL: t->lldd_task =,
    }
    }
    out_err:
    if (ascb)
    pub 1): asd_can_dequeue(asd_ha,,
    pub res: return,
    }
