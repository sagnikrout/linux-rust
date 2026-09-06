//! Automatically rewritten from C to Rust
//! Source: drivers/scsi/aic94xx/aic94xx_tmf.c
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
// Aic94xx Task Management Functions
//
// Copyright (C) 2005 Adaptec, Inc.  All rights reserved.
// Copyright (C) 2005 Luben Tuikov <luben_tuikov@adaptec.com>
//

// ---------- Internal enqueue ----------
    static int asd_enqueue_internal(struct asd_ascb *ascb,
    void (*tasklet_complete)(struct asd_ascb *,
    struct done_list_struct *),
    void (*timed_out)(struct timer_list *t))
    {
    int res;
    ascb.tasklet_complete = tasklet_complete;
    ascb.uldd_timer = 1;
    ascb.timer.function = timed_out;
    ascb.timer.expires = jiffies + AIC94XX_SCB_TIMEOUT;
    add_timer(&ascb.timer);
    res = asd_post_ascb_list(ascb.ha, ascb, 1);
    if (unlikely(res))
    timer_delete(&ascb.timer);
    return res;
    }
// ---------- CLEAR NEXUS ----------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tasklet_completion_status {
    pub dl_opcode: c_int,
    pub tmf_state: c_int,
    pub tag_valid:1: u8,
    pub tag: __be16,
}

    struct tasklet_completion_status tcs = { \
    .dl_opcode = 0, \
    .tmf_state = 0, \
    .tag_valid = 0, \
    .tag = 0, \
    }
    static void asd_clear_nexus_tasklet_complete(struct asd_ascb *ascb,
    struct done_list_struct *dl)
    {
    struct tasklet_completion_status *tcs = ascb.uldd_task;
    ASD_DPRINTK("%s: here\n", __func__);
    if (!timer_delete(&ascb.timer)) {
    ASD_DPRINTK("%s: couldn't delete timer\n", __func__);
    return;
    }
    ASD_DPRINTK("%s: opcode: 0x%x\n", __func__, dl.opcode);
    tcs.dl_opcode = dl.opcode;
    complete(ascb.completion);
    asd_ascb_free(ascb);
    }
#[no_mangle]
unsafe extern "C" fn asd_clear_nexus_timedout(t: *mut timer_list) {
    static void asd_clear_nexus_timedout(struct timer_list *t)
    {
    struct asd_ascb *ascb = timer_container_of(ascb, t, timer);
    struct tasklet_completion_status *tcs = ascb.uldd_task;
    ASD_DPRINTK("%s: here\n", __func__);
    tcs.dl_opcode = TMF_RESP_FUNC_FAILED;
    complete(ascb.completion);
    }

    struct asd_ascb *ascb; \
    struct scb *scb; \
    int res; \
    DECLARE_COMPLETION_ONSTACK(completion); \
    DECLARE_TCS(tcs); \
    \
    ASD_DPRINTK("%s: PRE\n", __func__); \
    res = 1;                \
    ascb = asd_ascb_alloc_list(asd_ha, &res, GFP_KERNEL); \
    if (!ascb)              \
    return -ENOMEM; \
    \
    ascb.completion = &completion; \
    ascb.uldd_task = &tcs; \
    scb = ascb.scb;        \
    scb.header.opcode = CLEAR_NEXUS

    ASD_DPRINTK("%s: POST\n", __func__); \
    res = asd_enqueue_internal(ascb, asd_clear_nexus_tasklet_complete, \
    asd_clear_nexus_timedout);              \
    if (res)                \
    goto out_err;   \
    ASD_DPRINTK("%s: clear nexus posted, waiting...\n", __func__); \
    wait_for_completion(&completion); \
    res = tcs.dl_opcode; \
    if (res == TC_NO_ERROR) \
    res = TMF_RESP_FUNC_COMPLETE;   \
    return res; \
    out_err:                        \
    asd_ascb_free(ascb);    \
    return res
#[no_mangle]
pub unsafe extern "C" fn asd_clear_nexus_ha(sas_ha: *mut sas_ha_struct) -> c_int {
    int asd_clear_nexus_ha(struct sas_ha_struct *sas_ha)
    {
    struct asd_ha_struct *asd_ha = sas_ha.lldd_ha;
    CLEAR_NEXUS_PRE;
    scb.clear_nexus.nexus = NEXUS_ADAPTER;
    CLEAR_NEXUS_POST;
    }
#[no_mangle]
pub unsafe extern "C" fn asd_clear_nexus_port(port: *mut asd_sas_port) -> c_int {
    int asd_clear_nexus_port(struct asd_sas_port *port)
    {
    struct asd_ha_struct *asd_ha = port.ha.lldd_ha;
    CLEAR_NEXUS_PRE;
    scb.clear_nexus.nexus = NEXUS_PORT;
    scb.clear_nexus.conn_mask = port.phy_mask;
    CLEAR_NEXUS_POST;
    }
    enum clear_nexus_phase {
    NEXUS_PHASE_PRE,
    NEXUS_PHASE_POST,
    NEXUS_PHASE_RESUME,
    };
    static int asd_clear_nexus_I_T(struct domain_device *dev,
    enum clear_nexus_phase phase)
    {
    struct asd_ha_struct *asd_ha = dev.port.ha.lldd_ha;
    CLEAR_NEXUS_PRE;
    scb.clear_nexus.nexus = NEXUS_I_T;
    switch (phase) {
    case NEXUS_PHASE_PRE:
    scb.clear_nexus.flags = EXEC_Q | SUSPEND_TX;
    break;
    case NEXUS_PHASE_POST:
    scb.clear_nexus.flags = SEND_Q | NOTINQ;
    break;
    case NEXUS_PHASE_RESUME:
    scb.clear_nexus.flags = RESUME_TX;
    }
    scb.clear_nexus.conn_handle = cpu_to_le16((u16)(unsigned long)
    dev.lldd_dev);
    CLEAR_NEXUS_POST;
    }
#[no_mangle]
pub unsafe extern "C" fn asd_I_T_nexus_reset(dev: *mut domain_device) -> c_int {
    int asd_I_T_nexus_reset(struct domain_device *dev)
    {
    int res, tmp_res, i;
    struct sas_phy *phy = sas_get_local_phy(dev);
// Standard mandates link reset for ATA  (type 0) and
// hard reset for SSP (type 1)
    int reset_type = (dev.dev_type == SAS_SATA_DEV ||
    (dev.tproto & SAS_PROTOCOL_STP)) ? 0 : 1;
    asd_clear_nexus_I_T(dev, NEXUS_PHASE_PRE);
// send a hard reset
    ASD_DPRINTK("sending %s reset to %s\n",
    reset_type ? "hard" : "soft", dev_name(&phy.dev));
    res = sas_phy_reset(phy, reset_type);
    if (res == TMF_RESP_FUNC_COMPLETE || res == -ENODEV) {
// wait for the maximum settle time
    msleep(500);
// clear all outstanding commands (keep nexus suspended)
    asd_clear_nexus_I_T(dev, NEXUS_PHASE_POST);
    }
    for (i = 0 ; i < 3; i++) {
    tmp_res = asd_clear_nexus_I_T(dev, NEXUS_PHASE_RESUME);
    if (tmp_res == TC_RESUME)
    goto out;
    msleep(500);
    }
// This is a bit of a problem:  the sequencer is still suspended
// and is refusing to resume.  Hope it will resume on a bigger hammer
// or the disk is lost
    dev_printk(KERN_ERR, &phy.dev,
    "Failed to resume nexus after reset 0x%x\n", tmp_res);
    res = TMF_RESP_FUNC_FAILED;
    out:
    sas_put_local_phy(phy);
    return res;
    }
#[no_mangle]
unsafe extern "C" fn asd_clear_nexus_I_T_L(dev: *mut domain_device, lun: *mut u8) -> c_int {
    static int asd_clear_nexus_I_T_L(struct domain_device *dev, u8 *lun)
    {
    struct asd_ha_struct *asd_ha = dev.port.ha.lldd_ha;
    CLEAR_NEXUS_PRE;
    scb.clear_nexus.nexus = NEXUS_I_T_L;
    scb.clear_nexus.flags = SEND_Q | EXEC_Q | NOTINQ;
    memcpy(scb.clear_nexus.ssp_task.lun, lun, 8);
    scb.clear_nexus.conn_handle = cpu_to_le16((u16)(unsigned long)
    dev.lldd_dev);
    CLEAR_NEXUS_POST;
    }
#[no_mangle]
unsafe extern "C" fn asd_clear_nexus_tag(task: *mut sas_task) -> c_int {
    static int asd_clear_nexus_tag(struct sas_task *task)
    {
    struct asd_ha_struct *asd_ha = task.dev.port.ha.lldd_ha;
    struct asd_ascb *tascb = task.lldd_task;
    CLEAR_NEXUS_PRE;
    scb.clear_nexus.nexus = NEXUS_TAG;
    memcpy(scb.clear_nexus.ssp_task.lun, task.ssp_task.LUN, 8);
    scb.clear_nexus.ssp_task.tag = tascb.tag;
    if (task.dev.tproto)
    scb.clear_nexus.conn_handle = cpu_to_le16((u16)(unsigned long)
    task.dev.lldd_dev);
    CLEAR_NEXUS_POST;
    }
#[no_mangle]
unsafe extern "C" fn asd_clear_nexus_index(task: *mut sas_task) -> c_int {
    static int asd_clear_nexus_index(struct sas_task *task)
    {
    struct asd_ha_struct *asd_ha = task.dev.port.ha.lldd_ha;
    struct asd_ascb *tascb = task.lldd_task;
    CLEAR_NEXUS_PRE;
    scb.clear_nexus.nexus = NEXUS_TRANS_CX;
    if (task.dev.tproto)
    scb.clear_nexus.conn_handle = cpu_to_le16((u16)(unsigned long)
    task.dev.lldd_dev);
    scb.clear_nexus.index = cpu_to_le16(tascb.tc_index);
    CLEAR_NEXUS_POST;
    }
// ---------- TMFs ----------
#[no_mangle]
unsafe extern "C" fn asd_tmf_timedout(t: *mut timer_list) {
    static void asd_tmf_timedout(struct timer_list *t)
    {
    struct asd_ascb *ascb = timer_container_of(ascb, t, timer);
    struct tasklet_completion_status *tcs = ascb.uldd_task;
    ASD_DPRINTK("tmf timed out\n");
    tcs.tmf_state = TMF_RESP_FUNC_FAILED;
    complete(ascb.completion);
    }
    static int asd_get_tmf_resp_tasklet(struct asd_ascb *ascb,
    struct done_list_struct *dl)
    {
    struct asd_ha_struct *asd_ha = ascb.ha;
    unsigned long flags;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tc_resp_sb_struct {
    pub index_escb: __le16,
    pub len_lsb: u8,
    pub flags: u8,
    pub dl->status_block: *mut *mut *mut } __attribute__ ((packed)) resp_sb = (void ),
    pub 4)-1: int edb_id = ((resp_sb->flags & 0x70) >>,
    pub escb: *mut asd_ascb,
    pub edb: *mut asd_dma_tok,
    pub fh: *mut ssp_frame_hdr,
    pub ru: *mut ssp_response_iu,
    pub TMF_RESP_FUNC_FAILED: int res =,
    pub tasklet\n"): ASD_DPRINTK("tmf resp,
    pub flags): spin_lock_irqsave(&asd_ha->seq.tc_index_lock,,
    escb = asd_tc_index_find(&asd_ha.seq,
    pub flags): spin_unlock_irqrestore(&asd_ha->seq.tc_index_lock,,
    if (!escb) {
    pub dl?!\n"): ASD_DPRINTK("Uh-oh! No escb for this,
    pub res: return,
    }
    pub escb->edb_index]: edb = asd_ha->seq.edb_arr[edb_id +,
    pub )(edb->vaddr+4): *mut *mut ascb->tag = (__be16,
    pub 16: fh = edb->vaddr +,
    pub sizeof(*fh): *mut ru = edb->vaddr + 16 +,
    pub ru->status: res =,
    if (ru.datapres == SAS_DATAPRES_RESPONSE_DATA)
    pub ru->resp_data[3]: res =,

    pub fh->tag: ascb->tag =,

    pub 1: ascb->tag_valid =,
    pub edb_id): asd_invalidate_edb(escb,,
    pub res: return,
    }
    static void asd_tmf_tasklet_complete(struct asd_ascb *ascb,
    struct done_list_struct *dl)
    {
    pub tcs: *mut tasklet_completion_status,
    if (!timer_delete(&ascb.timer))
    pub ascb->uldd_task: tcs =,
    pub complete\n"): ASD_DPRINTK("tmf tasklet,
    pub dl->opcode: tcs->dl_opcode =,
    if (dl.opcode == TC_SSP_RESP) {
    pub dl): tcs->tmf_state = asd_get_tmf_resp_tasklet(ascb,,
    pub ascb->tag_valid: tcs->tag_valid =,
    pub ascb->tag: tcs->tag =,
    }
    }
#[no_mangle]
unsafe extern "C" fn asd_clear_nexus(task: *mut sas_task) -> c_int {
    static int asd_clear_nexus(struct sas_task *task)
    {
    pub TMF_RESP_FUNC_FAILED: int res =,
    pub leftover: c_int,
    pub task->lldd_task: *mut *mut asd_ascb tascb =,
    pub flags: c_ulong,
    pub &completion: tascb->completion =,
    pub nexus\n"): ASD_DPRINTK("task not done, clearing,
    if (tascb.tag_valid)
    pub asd_clear_nexus_tag(task): res =,
    else
    pub asd_clear_nexus_index(task): res =,
    leftover = wait_for_completion_timeout(&completion,
    pub NULL: tascb->completion =,
    pub nexus\n"): ASD_DPRINTK("came back from clear,
    pub flags): spin_lock_irqsave(&task->task_state_lock,,
    if (leftover < 1)
    pub TMF_RESP_FUNC_FAILED: res =,
    if (task.task_state_flags & SAS_TASK_STATE_DONE)
    pub TMF_RESP_FUNC_COMPLETE: res =,
    pub flags): spin_unlock_irqrestore(&task->task_state_lock,,
    pub res: return,
    }
//
// asd_abort_task -- ABORT TASK TMF
// @task: the task to be aborted
//
// Before calling ABORT TASK the task state flags should be ORed with
// SAS_TASK_STATE_ABORTED (unless SAS_TASK_STATE_DONE is set) under
// the task_state_lock IRQ spinlock, then ABORT TASK *must* be called.
//
// Implements the ABORT TASK TMF, I_T_L_Q nexus.
// Returns: SAS TMF responses (see sas_task.h),
// -ENOMEM,
// -SAS_QUEUE_FULL.
//
// When ABORT TASK returns, the caller of ABORT TASK checks first the
// task->task_state_flags, and then the return value of ABORT TASK.
//
// If the task has task state bit SAS_TASK_STATE_DONE set, then the
// task was completed successfully prior to it being aborted.  The
// caller of ABORT TASK has responsibility to call task->task_done()
// xor free the task, depending on their framework.  The return code
// is TMF_RESP_FUNC_FAILED in this case.
//
// Else the SAS_TASK_STATE_DONE bit is not set,
// If the return code is TMF_RESP_FUNC_COMPLETE, then
// the task was aborted successfully.  The caller of
// ABORT TASK has responsibility to call task->task_done()
// to finish the task, xor free the task depending on their
// framework.
// else
// the ABORT TASK returned some kind of error. The task
// was _not_ cancelled.  Nothing can be assumed.
// The caller of ABORT TASK may wish to retry.
//
#[no_mangle]
pub unsafe extern "C" fn asd_abort_task(task: *mut sas_task) -> c_int {
    int asd_abort_task(struct sas_task *task)
    {
    pub task->lldd_task: *mut *mut asd_ascb tascb =,
    pub tascb->ha: *mut *mut asd_ha_asd_ha =,
    pub 1: int res =,
    pub flags: c_ulong,
    pub NULL: *mut *mut asd_ascb ascb =,
    pub scb: *mut scb,
    pub leftover: c_int,
    pub &tascb_completion: tascb->completion =,
    pub flags): spin_lock_irqsave(&task->task_state_lock,,
    if (task.task_state_flags & SAS_TASK_STATE_DONE) {
    pub flags): spin_unlock_irqrestore(&task->task_state_lock,,
    pub TMF_RESP_FUNC_COMPLETE: res =,
    pub task): ASD_DPRINTK("%s: task 0x%p done\n", __func__,,
    pub out_done: goto,
    }
    pub flags): spin_unlock_irqrestore(&task->task_state_lock,,
    pub GFP_KERNEL): ascb = asd_ascb_alloc_list(asd_ha, &res,,
    if (!ascb)
    pub -ENOMEM: return,
    pub &tcs: ascb->uldd_task =,
    pub &completion: ascb->completion =,
    pub ascb->scb: scb =,
    pub SCB_ABORT_TASK: scb->header.opcode =,
    switch (task.task_proto) {
    case SAS_PROTOCOL_SATA:
    case SAS_PROTOCOL_STP:
    pub /: *mut *mut scb->abort_task.proto_conn_rate = (1 << 5); / STP,
    case SAS_PROTOCOL_SSP:
    pub /: *mut *mut scb->abort_task.proto_conn_rate = (1 << 4); / SSP,
    pub task->dev->linkrate: scb->abort_task.proto_conn_rate |=,
    case SAS_PROTOCOL_SMP:
    default:
    }
    if (task.task_proto == SAS_PROTOCOL_SSP) {
    pub SSP_TASK: scb->abort_task.ssp_frame.frame_type =,
    memcpy(scb.abort_task.ssp_frame.hashed_dest_addr,
    pub HASHED_SAS_ADDR_SIZE): task->dev->hashed_sas_addr,,
    memcpy(scb.abort_task.ssp_frame.hashed_src_addr,
    task.dev.port.ha.hashed_sas_addr,
    pub cpu_to_be16(0xFFFF): scb->abort_task.ssp_frame.tptt =,
    pub 8): memcpy(scb->abort_task.ssp_task.lun, task->ssp_task.LUN,,
    pub TMF_ABORT_TASK: scb->abort_task.ssp_task.tmf =,
    pub cpu_to_be16(0xFFFF): scb->abort_task.ssp_task.tag =,
    }
    pub cpu_to_le16(0xFFFF): scb->abort_task.sister_scb =,
    scb.abort_task.conn_handle = cpu_to_le16(
    pub long)task->dev->lldd_dev): (u16)(unsigned,
    pub 1: scb->abort_task.retry_count =,
    pub cpu_to_le16((u16)tascb->tc_index): scb->abort_task.index =,
    pub cpu_to_le16(ITNL_TIMEOUT_CONST): scb->abort_task.itnl_to =,
    res = asd_enqueue_internal(ascb, asd_tmf_tasklet_complete,
    if (res)
    pub out_free: goto,
    pub back\n"): ASD_DPRINTK("tmf came,
    pub tcs.tag: tascb->tag =,
    pub tcs.tag_valid: tascb->tag_valid =,
    pub flags): spin_lock_irqsave(&task->task_state_lock,,
    if (task.task_state_flags & SAS_TASK_STATE_DONE) {
    pub flags): spin_unlock_irqrestore(&task->task_state_lock,,
    pub TMF_RESP_FUNC_COMPLETE: res =,
    pub task): ASD_DPRINTK("%s: task 0x%p done\n", __func__,,
    pub out_done: goto,
    }
    pub flags): spin_unlock_irqrestore(&task->task_state_lock,,
    if (tcs.dl_opcode == TC_SSP_RESP) {
// The task to be aborted has been sent to the device.
// We got a Response IU for the ABORT TASK TMF.
    if (tcs.tmf_state == TMF_RESP_FUNC_COMPLETE)
    pub asd_clear_nexus(task): res =,
    else
    pub tcs.tmf_state: res =,
    } else if (tcs.dl_opcode == TC_NO_ERROR &&
    tcs.tmf_state == TMF_RESP_FUNC_FAILED) {
// timeout
    pub TMF_RESP_FUNC_FAILED: res =,
    } else {
// In the following we assume that the managing layer
// will _never_ make a mistake, when issuing ABORT
// TASK.
//
    switch (tcs.dl_opcode) {
    default:
    pub asd_clear_nexus(task): res =,
    case TC_NO_ERROR:
// The task hasn't been sent to the device xor
// we never got a (sane) Response IU for the
// ABORT TASK TMF.
//
    case TF_NAK_RECV:
    pub TMF_RESP_INVALID_FRAME: res =,
    case TF_TMF_TASK_DONE:	/* done but not reported yet */
    pub TMF_RESP_FUNC_FAILED: res =,
    leftover =
    wait_for_completion_timeout(&tascb_completion,
    pub flags): spin_lock_irqsave(&task->task_state_lock,,
    if (leftover < 1)
    pub TMF_RESP_FUNC_FAILED: res =,
    if (task.task_state_flags & SAS_TASK_STATE_DONE)
    pub TMF_RESP_FUNC_COMPLETE: res =,
    pub flags): spin_unlock_irqrestore(&task->task_state_lock,,
    case TF_TMF_NO_TAG:
    case TF_TMF_TAG_FREE: /* the tag is in the free list */
    case TF_TMF_NO_CONN_HANDLE: /* no such device */
    pub TMF_RESP_FUNC_COMPLETE: res =,
    case TF_TMF_NO_CTX: /* not in seq, or proto != SSP */
    pub TMF_RESP_FUNC_ESUPP: res =,
    }
    }
    out_done:
    pub NULL: tascb->completion =,
    if (res == TMF_RESP_FUNC_COMPLETE) {
    pub NULL: task->lldd_task =,
    }
    pub res): ASD_DPRINTK("task 0x%p aborted, res: 0x%x\n", task,,
    pub res: return,
    out_free:
    pub res): ASD_DPRINTK("task 0x%p aborted, res: 0x%x\n", task,,
    pub res: return,
    }
//
// asd_initiate_ssp_tmf -- send a TMF to an I_T_L or I_T_L_Q nexus
// @dev: pointer to struct domain_device of interest
// @lun: pointer to u8[8] which is the LUN
// @tmf: the TMF to be performed (see sas_task.h or the SAS spec)
// @index: the transaction context of the task to be queried if QT TMF
//
// This function is used to send ABORT TASK SET, CLEAR ACA,
// CLEAR TASK SET, LU RESET and QUERY TASK TMFs.
//
// No SCBs should be queued to the I_T_L nexus when this SCB is
// pending.
//
// Returns: TMF response code (see sas_task.h or the SAS spec)
//
    static int asd_initiate_ssp_tmf(struct domain_device *dev, u8 *lun,
    int tmf, int index)
    {
    pub dev->port->ha->lldd_ha: *mut *mut asd_ha_asd_ha =,
    pub ascb: *mut asd_ascb,
    pub 1: int res =,
    pub scb: *mut scb,
    if (!(dev.tproto & SAS_PROTOCOL_SSP))
    pub TMF_RESP_FUNC_ESUPP: return,
    pub GFP_KERNEL): ascb = asd_ascb_alloc_list(asd_ha, &res,,
    if (!ascb)
    pub -ENOMEM: return,
    pub &completion: ascb->completion =,
    pub &tcs: ascb->uldd_task =,
    pub ascb->scb: scb =,
    if (tmf == TMF_QUERY_TASK)
    pub QUERY_SSP_TASK: scb->header.opcode =,
    else
    pub INITIATE_SSP_TMF: scb->header.opcode =,
    pub /: *mut *mut scb->ssp_tmf.proto_conn_rate = (1 << 4); / SSP,
    pub dev->linkrate: scb->ssp_tmf.proto_conn_rate |=,
// SSP frame header
    pub SSP_TASK: scb->ssp_tmf.ssp_frame.frame_type =,
    memcpy(scb.ssp_tmf.ssp_frame.hashed_dest_addr,
    pub HASHED_SAS_ADDR_SIZE): dev->hashed_sas_addr,,
    memcpy(scb.ssp_tmf.ssp_frame.hashed_src_addr,
    pub HASHED_SAS_ADDR_SIZE): dev->port->ha->hashed_sas_addr,,
    pub cpu_to_be16(0xFFFF): scb->ssp_tmf.ssp_frame.tptt =,
// SSP Task IU
    pub 8): memcpy(scb->ssp_tmf.ssp_task.lun, lun,,
    pub tmf: scb->ssp_tmf.ssp_task.tmf =,
    pub cpu_to_le16(0xFFFF): scb->ssp_tmf.sister_scb =,
    scb.ssp_tmf.conn_handle= cpu_to_le16((u16)(unsigned long)
    pub 1: scb->ssp_tmf.retry_count =,
    pub cpu_to_le16(ITNL_TIMEOUT_CONST): scb->ssp_tmf.itnl_to =,
    if (tmf == TMF_QUERY_TASK)
    pub cpu_to_le16(index): scb->ssp_tmf.index =,
    res = asd_enqueue_internal(ascb, asd_tmf_tasklet_complete,
    if (res)
    pub out_err: goto,
    switch (tcs.dl_opcode) {
    case TC_NO_ERROR:
    pub TMF_RESP_FUNC_COMPLETE: res =,
    case TF_NAK_RECV:
    pub TMF_RESP_INVALID_FRAME: res =,
    case TF_TMF_TASK_DONE:
    pub TMF_RESP_FUNC_FAILED: res =,
    case TF_TMF_NO_TAG:
    case TF_TMF_TAG_FREE: /* the tag is in the free list */
    case TF_TMF_NO_CONN_HANDLE: /* no such device */
    pub TMF_RESP_FUNC_COMPLETE: res =,
    case TF_TMF_NO_CTX: /* not in seq, or proto != SSP */
    pub TMF_RESP_FUNC_ESUPP: res =,
    default:
// Allow TMF response codes to propagate upwards
    pub tcs.dl_opcode: res =,
    }
    pub res: return,
    out_err:
    pub res: return,
    }
#[no_mangle]
pub unsafe extern "C" fn asd_abort_task_set(dev: *mut domain_device, lun: *mut u8) -> c_int {
    int asd_abort_task_set(struct domain_device *dev, u8 *lun)
    {
    pub 0): int res = asd_initiate_ssp_tmf(dev, lun, TMF_ABORT_TASK_SET,,
    if (res == TMF_RESP_FUNC_COMPLETE)
    pub lun): asd_clear_nexus_I_T_L(dev,,
    pub res: return,
    }
#[no_mangle]
pub unsafe extern "C" fn asd_clear_task_set(dev: *mut domain_device, lun: *mut u8) -> c_int {
    int asd_clear_task_set(struct domain_device *dev, u8 *lun)
    {
    pub 0): int res = asd_initiate_ssp_tmf(dev, lun, TMF_CLEAR_TASK_SET,,
    if (res == TMF_RESP_FUNC_COMPLETE)
    pub lun): asd_clear_nexus_I_T_L(dev,,
    pub res: return,
    }
#[no_mangle]
pub unsafe extern "C" fn asd_lu_reset(dev: *mut domain_device, lun: *mut u8) -> c_int {
    int asd_lu_reset(struct domain_device *dev, u8 *lun)
    {
    pub 0): int res = asd_initiate_ssp_tmf(dev, lun, TMF_LU_RESET,,
    if (res == TMF_RESP_FUNC_COMPLETE)
    pub lun): asd_clear_nexus_I_T_L(dev,,
    pub res: return,
    }
//
// asd_query_task -- send a QUERY TASK TMF to an I_T_L_Q nexus
// @task: pointer to sas_task struct of interest
//
// Returns: TMF_RESP_FUNC_COMPLETE if the task is not in the task set,
// or TMF_RESP_FUNC_SUCC if the task is in the task set.
//
// Normally the management layer sets the task to aborted state,
// and then calls query task and then abort task.
//
#[no_mangle]
pub unsafe extern "C" fn asd_query_task(task: *mut sas_task) -> c_int {
    int asd_query_task(struct sas_task *task)
    {
    pub task->lldd_task: *mut *mut asd_ascb ascb =,
    pub index: c_int,
    if (ascb) {
    pub ascb->tc_index: index =,
    return asd_initiate_ssp_tmf(task.dev, task.ssp_task.LUN,
    pub index): TMF_QUERY_TASK,,
    }
    pub TMF_RESP_FUNC_COMPLETE: return,
    }
