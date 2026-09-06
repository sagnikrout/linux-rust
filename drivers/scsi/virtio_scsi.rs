//! Automatically rewritten from C to Rust
//! Source: drivers/scsi/virtio_scsi.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Virtio SCSI HBA driver
//
// Copyright IBM Corp. 2010
// Copyright Red Hat, Inc. 2011
//
// Authors:
// Stefan Hajnoczi   <stefanha@linux.vnet.ibm.com>
// Paolo Bonzini   <pbonzini@redhat.com>
//

pub const VIRTIO_SCSI_MEMPOOL_SZ: c_int = 64;
pub const VIRTIO_SCSI_EVENT_LEN: c_int = 8;
pub const VIRTIO_SCSI_VQ_BASE: c_int = 2;
    static unsigned int virtscsi_poll_queues;
    module_param(virtscsi_poll_queues, uint, 0644);
    MODULE_PARM_DESC(virtscsi_poll_queues,
    "The number of dedicated virtqueues for polling I/O");
// Command queue element
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_scsi_cmd {
    pub sc: *mut scsi_cmnd,
    pub comp: *mut completion,
    union {
    pub cmd: virtio_scsi_cmd_req,
    pub cmd_pi: virtio_scsi_cmd_req_pi,
    pub tmf: virtio_scsi_ctrl_tmf_req,
    pub an: virtio_scsi_ctrl_an_req,
    pub req: },
    union {
    pub cmd: virtio_scsi_cmd_resp,
    pub tmf: virtio_scsi_ctrl_tmf_resp,
    pub an: virtio_scsi_ctrl_an_resp,
    pub evt: virtio_scsi_event,
    pub resp: },
    pub ____cacheline_aligned_in_smp: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_scsi_event_node {
    pub vscsi: *mut virtio_scsi,
    pub event: *mut virtio_scsi_event,
    pub work: work_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_scsi_vq {
// Protects vq
    pub vq_lock: spinlock_t,
    pub vq: *mut virtqueue,
}

// Driver instance state
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_scsi {
    pub vdev: *mut virtio_device,
// Get some buffers ready for event vq
    pub event_list: [virtio_scsi_event_node; VIRTIO_SCSI_EVENT_LEN],
    pub num_queues: u32,
    pub io_queues: [c_int; HCTX_MAX_TYPES],
    pub node: hlist_node,
// Protected by event_vq lock
    pub stop_events: bool,
    pub ctrl_vq: virtio_scsi_vq,
    pub event_vq: virtio_scsi_vq,
    pub events: [virtio_scsi_event; VIRTIO_SCSI_EVENT_LEN],
    pub req_vqs: [virtio_scsi_vq; ],
}

    static struct kmem_cache *virtscsi_cmd_cache;
    static mempool_t *virtscsi_cmd_pool;
    static inline struct Scsi_Host *virtio_scsi_host(struct virtio_device *vdev)
    {
    return vdev.priv;
    }
#[no_mangle]
unsafe extern "C" fn virtscsi_compute_resid(sc: *mut scsi_cmnd, resid: u32) {
    static void virtscsi_compute_resid(struct scsi_cmnd *sc, u32 resid)
    {
    if (resid)
    scsi_set_resid(sc, min(resid, scsi_bufflen(sc)));
    }
//
// virtscsi_complete_cmd - finish a scsi_cmd and invoke scsi_done
//
// Called with vq_lock held.
//
#[no_mangle]
unsafe extern "C" fn virtscsi_complete_cmd(vscsi: *mut virtio_scsi, buf: *mut c_void) {
    static void virtscsi_complete_cmd(struct virtio_scsi *vscsi, void *buf)
    {
    struct virtio_scsi_cmd *cmd = buf;
    struct scsi_cmnd *sc = cmd.sc;
    struct virtio_scsi_cmd_resp *resp = &cmd.resp.cmd;
    let mut sense_len: unsigned = virtio32_to_cpu(vscsi.vdev, resp.sense_len);
    dev_dbg(&sc.device.sdev_gendev,
    "cmd %p response %u status %#02x sense_len %u\n",
    sc, resp.response, resp.status, sense_len);
    sc.result = resp.status;
    virtscsi_compute_resid(sc, virtio32_to_cpu(vscsi.vdev, resp.resid));
    switch (resp.response) {
    case VIRTIO_SCSI_S_OK:
    set_host_byte(sc, DID_OK);
    break;
    case VIRTIO_SCSI_S_OVERRUN:
    set_host_byte(sc, DID_ERROR);
    break;
    case VIRTIO_SCSI_S_ABORTED:
    set_host_byte(sc, DID_ABORT);
    break;
    case VIRTIO_SCSI_S_BAD_TARGET:
    set_host_byte(sc, DID_BAD_TARGET);
    break;
    case VIRTIO_SCSI_S_RESET:
    set_host_byte(sc, DID_RESET);
    break;
    case VIRTIO_SCSI_S_BUSY:
    set_host_byte(sc, DID_BUS_BUSY);
    break;
    case VIRTIO_SCSI_S_TRANSPORT_FAILURE:
    set_host_byte(sc, DID_TRANSPORT_DISRUPTED);
    break;
    case VIRTIO_SCSI_S_TARGET_FAILURE:
    set_host_byte(sc, DID_BAD_TARGET);
    break;
    case VIRTIO_SCSI_S_NEXUS_FAILURE:
    set_status_byte(sc, SAM_STAT_RESERVATION_CONFLICT);
    break;
    default:
    scmd_printk(KERN_WARNING, sc, "Unknown response %d",
    resp.response);
    fallthrough;
    case VIRTIO_SCSI_S_FAILURE:
    set_host_byte(sc, DID_ERROR);
    break;
    }
    WARN_ON(sense_len > VIRTIO_SCSI_SENSE_SIZE);
    if (resp.sense_len) {
    memcpy(sc.sense_buffer, resp.sense,
    min_t(u32, sense_len, VIRTIO_SCSI_SENSE_SIZE));
    }
    scsi_done(sc);
    }
    static void virtscsi_vq_done(struct virtio_scsi *vscsi,
    struct virtio_scsi_vq *virtscsi_vq,
    void (*fn)(struct virtio_scsi *vscsi, void *buf))
    {
    void *buf;
    unsigned int len;
    unsigned long flags;
    struct virtqueue *vq = virtscsi_vq.vq;
    spin_lock_irqsave(&virtscsi_vq.vq_lock, flags);
    do {
    virtqueue_disable_cb(vq);
    while ((buf = virtqueue_get_buf(vq, &len)) != core::ptr::null_mut())
    fn(vscsi, buf);
    } while (!virtqueue_enable_cb(vq));
    spin_unlock_irqrestore(&virtscsi_vq.vq_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn virtscsi_req_done(vq: *mut virtqueue) {
    static void virtscsi_req_done(struct virtqueue *vq)
    {
    struct Scsi_Host *sh = virtio_scsi_host(vq.vdev);
    struct virtio_scsi *vscsi = shost_priv(sh);
    let mut index: c_int = vq.index - VIRTIO_SCSI_VQ_BASE;
    struct virtio_scsi_vq *req_vq = &vscsi.req_vqs[index];
    virtscsi_vq_done(vscsi, req_vq, virtscsi_complete_cmd);
    };
#[no_mangle]
unsafe extern "C" fn virtscsi_poll_requests(vscsi: *mut virtio_scsi) {
    static void virtscsi_poll_requests(struct virtio_scsi *vscsi)
    {
    int i, num_vqs;
    num_vqs = vscsi.num_queues;
    for (i = 0; i < num_vqs; i++)
    virtscsi_vq_done(vscsi, &vscsi.req_vqs[i],
    virtscsi_complete_cmd);
    }
#[no_mangle]
unsafe extern "C" fn virtscsi_complete_free(vscsi: *mut virtio_scsi, buf: *mut c_void) {
    static void virtscsi_complete_free(struct virtio_scsi *vscsi, void *buf)
    {
    struct virtio_scsi_cmd *cmd = buf;
    if (cmd.comp)
    complete(cmd.comp);
    }
#[no_mangle]
unsafe extern "C" fn virtscsi_ctrl_done(vq: *mut virtqueue) {
    static void virtscsi_ctrl_done(struct virtqueue *vq)
    {
    struct Scsi_Host *sh = virtio_scsi_host(vq.vdev);
    struct virtio_scsi *vscsi = shost_priv(sh);
    virtscsi_vq_done(vscsi, &vscsi.ctrl_vq, virtscsi_complete_free);
    };
    static int virtscsi_kick_event(struct virtio_scsi *vscsi,
    struct virtio_scsi_event_node *event_node)
    {
    int err;
    struct scatterlist sg;
    unsigned long flags;
    sg_init_one(&sg, event_node.event, sizeof(struct virtio_scsi_event));
    spin_lock_irqsave(&vscsi.event_vq.vq_lock, flags);
    err = virtqueue_add_inbuf_cache_clean(vscsi.event_vq.vq, &sg, 1, event_node,
    GFP_ATOMIC);
    if (!err)
    virtqueue_kick(vscsi.event_vq.vq);
    spin_unlock_irqrestore(&vscsi.event_vq.vq_lock, flags);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn virtscsi_kick_event_all(vscsi: *mut virtio_scsi) -> c_int {
    static int virtscsi_kick_event_all(struct virtio_scsi *vscsi)
    {
    int i;
    for (i = 0; i < VIRTIO_SCSI_EVENT_LEN; i++) {
    vscsi.event_list[i].vscsi = vscsi;
    vscsi.event_list[i].event = &vscsi.events[i];
    virtscsi_kick_event(vscsi, &vscsi.event_list[i]);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn virtscsi_cancel_event_work(vscsi: *mut virtio_scsi) {
    static void virtscsi_cancel_event_work(struct virtio_scsi *vscsi)
    {
    int i;
// Stop scheduling work before calling cancel_work_sync.
    spin_lock_irq(&vscsi.event_vq.vq_lock);
    vscsi.stop_events = true;
    spin_unlock_irq(&vscsi.event_vq.vq_lock);
    for (i = 0; i < VIRTIO_SCSI_EVENT_LEN; i++)
    cancel_work_sync(&vscsi.event_list[i].work);
    }
    static void virtscsi_handle_transport_reset(struct virtio_scsi *vscsi,
    struct virtio_scsi_event *event)
    {
    struct scsi_device *sdev;
    struct Scsi_Host *shost = virtio_scsi_host(vscsi.vdev);
    let mut target: c_uint = event.lun[1];
    let mut lun: c_uint = (event.lun[2] << 8) | event.lun[3];
    let mut reason: c_uint = virtio32_to_cpu(vscsi.vdev, event.reason);
    switch (reason) {
    case VIRTIO_SCSI_EVT_RESET_RESCAN:
    if (lun == 0) {
    scsi_scan_target(&shost.shost_gendev, 0, target,
    SCAN_WILD_CARD, SCSI_SCAN_INITIAL);
    } else {
    scsi_add_device(shost, 0, target, lun);
    }
    break;
    case VIRTIO_SCSI_EVT_RESET_REMOVED:
    sdev = scsi_device_lookup(shost, 0, target, lun);
    if (sdev) {
    scsi_remove_device(sdev);
    scsi_device_put(sdev);
    } else {
    pr_err("SCSI device %d 0 %d %d not found\n",
    shost.host_no, target, lun);
    }
    break;
    default:
    pr_info("Unsupported virtio scsi event reason %x\n", reason);
    }
    }
    static void virtscsi_handle_param_change(struct virtio_scsi *vscsi,
    struct virtio_scsi_event *event)
    {
    struct scsi_device *sdev;
    struct Scsi_Host *shost = virtio_scsi_host(vscsi.vdev);
    let mut target: c_uint = event.lun[1];
    let mut lun: c_uint = (event.lun[2] << 8) | event.lun[3];
    let mut asc: u8 = virtio32_to_cpu(vscsi.vdev, event.reason) & 255;
    let mut ascq: u8 = virtio32_to_cpu(vscsi.vdev, event.reason) >> 8;
    sdev = scsi_device_lookup(shost, 0, target, lun);
    if (!sdev) {
    pr_err("SCSI device %d 0 %d %d not found\n",
    shost.host_no, target, lun);
    return;
    }
// Handle "Parameters changed", "Mode parameters changed", and
    "Capacity data has changed".  */
    if (asc == 0x2a && (ascq == 0x00 || ascq == 0x01 || ascq == 0x09))
    scsi_rescan_device(sdev);
    scsi_device_put(sdev);
    }
#[no_mangle]
unsafe extern "C" fn virtscsi_rescan_hotunplug(vscsi: *mut virtio_scsi) -> c_int {
    static int virtscsi_rescan_hotunplug(struct virtio_scsi *vscsi)
    {
    struct scsi_device *sdev;
    struct Scsi_Host *shost = virtio_scsi_host(vscsi.vdev);
    unsigned char scsi_cmd[MAX_COMMAND_SIZE];
    int result, inquiry_len, inq_result_len = 256;
    char *inq_result = kmalloc(inq_result_len, GFP_KERNEL);
    if (!inq_result)
    return -ENOMEM;
    shost_for_each_device(sdev, shost) {
    inquiry_len = sdev.inquiry_len ? sdev.inquiry_len : 36;
    memset(scsi_cmd, 0, sizeof(scsi_cmd));
    scsi_cmd[0] = INQUIRY;
    scsi_cmd[4] = (unsigned char) inquiry_len;
    memset(inq_result, 0, inq_result_len);
    result = scsi_execute_cmd(sdev, scsi_cmd, REQ_OP_DRV_IN,
    inq_result, inquiry_len,
    SD_TIMEOUT, SD_MAX_RETRIES, core::ptr::null_mut());
    if (result == 0 && inq_result[0] >> 5) {
// PQ indicates the LUN is not attached
    scsi_remove_device(sdev);
    } else if (result > 0 && host_byte(result) == DID_BAD_TARGET) {
//
// If all LUNs of a virtio-scsi device are unplugged
// it will respond with BAD TARGET on any INQUIRY
// command.
// Remove the device in this case as well.
//
    scsi_remove_device(sdev);
    }
    }
    kfree(inq_result);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn virtscsi_handle_event(work: *mut work_struct) {
    static void virtscsi_handle_event(struct work_struct *work)
    {
    struct virtio_scsi_event_node *event_node =
    container_of(work, struct virtio_scsi_event_node, work);
    struct virtio_scsi *vscsi = event_node.vscsi;
    struct virtio_scsi_event *event = event_node.event;
    if (event.event &
    cpu_to_virtio32(vscsi.vdev, VIRTIO_SCSI_T_EVENTS_MISSED)) {
    int ret;
    event.event &= ~cpu_to_virtio32(vscsi.vdev,
    VIRTIO_SCSI_T_EVENTS_MISSED);
    ret = virtscsi_rescan_hotunplug(vscsi);
    if (ret)
    return;
    scsi_scan_host(virtio_scsi_host(vscsi.vdev));
    }
    switch (virtio32_to_cpu(vscsi.vdev, event.event)) {
    case VIRTIO_SCSI_T_NO_EVENT:
    break;
    case VIRTIO_SCSI_T_TRANSPORT_RESET:
    virtscsi_handle_transport_reset(vscsi, event);
    break;
    case VIRTIO_SCSI_T_PARAM_CHANGE:
    virtscsi_handle_param_change(vscsi, event);
    break;
    default:
    pr_err("Unsupported virtio scsi event %x\n",
    virtio32_to_cpu(vscsi.vdev, event.event));
    }
    virtscsi_kick_event(vscsi, event_node);
    }
#[no_mangle]
unsafe extern "C" fn virtscsi_complete_event(vscsi: *mut virtio_scsi, buf: *mut c_void) {
    static void virtscsi_complete_event(struct virtio_scsi *vscsi, void *buf)
    {
    struct virtio_scsi_event_node *event_node = buf;
    if (!vscsi.stop_events)
    queue_work(system_freezable_wq, &event_node.work);
    }
#[no_mangle]
unsafe extern "C" fn virtscsi_event_done(vq: *mut virtqueue) {
    static void virtscsi_event_done(struct virtqueue *vq)
    {
    struct Scsi_Host *sh = virtio_scsi_host(vq.vdev);
    struct virtio_scsi *vscsi = shost_priv(sh);
    virtscsi_vq_done(vscsi, &vscsi.event_vq, virtscsi_complete_event);
    };
    static int __virtscsi_add_cmd(struct virtqueue *vq,
    struct virtio_scsi_cmd *cmd,
    size_t req_size, size_t resp_size)
    {
    struct scsi_cmnd *sc = cmd.sc;
    struct scatterlist *sgs[6], req, resp;
    struct sg_table *out, *in;
    let mut out_num: unsigned = 0, in_num = 0;
    out = in = core::ptr::null_mut();
    if (sc && sc.sc_data_direction != DMA_NONE) {
    if (sc.sc_data_direction != DMA_FROM_DEVICE)
    out = &sc.sdb.table;
    if (sc.sc_data_direction != DMA_TO_DEVICE)
    in = &sc.sdb.table;
    }
// Request header.
    sg_init_one(&req, &cmd.req, req_size);
    sgs[out_num++] = &req;
// Data-out buffer.
    if (out) {
// Place WRITE protection SGLs before Data OUT payload
    if (scsi_prot_sg_count(sc))
    sgs[out_num++] = scsi_prot_sglist(sc);
    sgs[out_num++] = out.sgl;
    }
// Response header.
    sg_init_one(&resp, &cmd.resp, resp_size);
    sgs[out_num + in_num++] = &resp;
// Data-in buffer
    if (in) {
// Place READ protection SGLs before Data IN payload
    if (scsi_prot_sg_count(sc))
    sgs[out_num + in_num++] = scsi_prot_sglist(sc);
    sgs[out_num + in_num++] = in.sgl;
    }
    return virtqueue_add_sgs(vq, sgs, out_num, in_num, cmd, GFP_ATOMIC);
    }
#[no_mangle]
unsafe extern "C" fn virtscsi_kick_vq(vq: *mut virtio_scsi_vq) {
    static void virtscsi_kick_vq(struct virtio_scsi_vq *vq)
    {
    bool needs_kick;
    unsigned long flags;
    spin_lock_irqsave(&vq.vq_lock, flags);
    needs_kick = virtqueue_kick_prepare(vq.vq);
    spin_unlock_irqrestore(&vq.vq_lock, flags);
    if (needs_kick)
    virtqueue_notify(vq.vq);
    }
//
// virtscsi_add_cmd - add a virtio_scsi_cmd to a virtqueue, optionally kick it
// @vq		: the struct virtqueue we're talking about
// @cmd		: command structure
// @req_size	: size of the request buffer
// @resp_size	: size of the response buffer
// @kick	: whether to kick the virtqueue immediately
//
    static int virtscsi_add_cmd(struct virtio_scsi_vq *vq,
    struct virtio_scsi_cmd *cmd,
    size_t req_size, size_t resp_size,
    bool kick)
    {
    unsigned long flags;
    int err;
    let mut needs_kick: bool = false;
    spin_lock_irqsave(&vq.vq_lock, flags);
    err = __virtscsi_add_cmd(vq.vq, cmd, req_size, resp_size);
    if (!err && kick)
    needs_kick = virtqueue_kick_prepare(vq.vq);
    spin_unlock_irqrestore(&vq.vq_lock, flags);
    if (needs_kick)
    virtqueue_notify(vq.vq);
    return err;
    }
    static void virtio_scsi_init_hdr(struct virtio_device *vdev,
    struct virtio_scsi_cmd_req *cmd,
    struct scsi_cmnd *sc)
    {
    cmd.lun[0] = 1;
    cmd.lun[1] = sc.device.id;
    cmd.lun[2] = (sc.device.lun >> 8) | 0x40;
    cmd.lun[3] = sc.device.lun & 0xff;
    cmd.tag = cpu_to_virtio64(vdev, (unsigned long)sc);
    cmd.task_attr = VIRTIO_SCSI_S_SIMPLE;
    cmd.prio = 0;
    cmd.crn = 0;
    }

    static void virtio_scsi_init_hdr_pi(struct virtio_device *vdev,
    struct virtio_scsi_cmd_req_pi *cmd_pi,
    struct scsi_cmnd *sc)
    {
    struct request *rq = scsi_cmd_to_rq(sc);
    struct blk_integrity *bi;
    virtio_scsi_init_hdr(vdev, (struct virtio_scsi_cmd_req *)cmd_pi, sc);
    if (!rq || !scsi_prot_sg_count(sc))
    return;
    bi = blk_get_integrity(rq.q.disk);
    if (sc.sc_data_direction == DMA_TO_DEVICE)
    cmd_pi.pi_bytesout = cpu_to_virtio32(vdev,
    bio_integrity_bytes(bi,
    blk_rq_sectors(rq)));
#[no_mangle]
pub unsafe extern "C" fn if(DMA_FROM_DEVICE: sc->sc_data_direction ==) -> else {
    else if (sc.sc_data_direction == DMA_FROM_DEVICE)
    cmd_pi.pi_bytesin = cpu_to_virtio32(vdev,
    bio_integrity_bytes(bi,
    blk_rq_sectors(rq)));
    }

    static struct virtio_scsi_vq *virtscsi_pick_vq_mq(struct virtio_scsi *vscsi,
    struct scsi_cmnd *sc)
    {
    let mut tag: u32 = blk_mq_unique_tag(scsi_cmd_to_rq(sc));
    let mut hwq: u16 = blk_mq_unique_tag_to_hwq(tag);
    return &vscsi.req_vqs[hwq];
    }
    static enum scsi_qc_status virtscsi_queuecommand(struct Scsi_Host *shost,
    struct scsi_cmnd *sc)
    {
    struct virtio_scsi *vscsi = shost_priv(shost);
    struct virtio_scsi_vq *req_vq = virtscsi_pick_vq_mq(vscsi, sc);
    struct virtio_scsi_cmd *cmd = scsi_cmd_priv(sc);
    bool kick;
    unsigned long flags;
    int req_size;
    int ret;
    BUG_ON(scsi_sg_count(sc) > shost.sg_tablesize);
// TODO: check feature bit and fail if unsupported?
    BUG_ON(sc.sc_data_direction == DMA_BIDIRECTIONAL);
    dev_dbg(&sc.device.sdev_gendev,
    "cmd %p CDB: %#02x\n", sc, sc.cmnd[0]);
    cmd.sc = sc;
    BUG_ON(sc.cmd_len > VIRTIO_SCSI_CDB_SIZE);

    if (virtio_has_feature(vscsi.vdev, VIRTIO_SCSI_F_T10_PI)) {
    virtio_scsi_init_hdr_pi(vscsi.vdev, &cmd.req.cmd_pi, sc);
    memcpy(cmd.req.cmd_pi.cdb, sc.cmnd, sc.cmd_len);
    req_size = sizeof(cmd.req.cmd_pi);
    } else

    {
    virtio_scsi_init_hdr(vscsi.vdev, &cmd.req.cmd, sc);
    memcpy(cmd.req.cmd.cdb, sc.cmnd, sc.cmd_len);
    req_size = sizeof(cmd.req.cmd);
    }
    kick = (sc.flags & SCMD_LAST) != 0;
    ret = virtscsi_add_cmd(req_vq, cmd, req_size, sizeof(cmd.resp.cmd), kick);
    if (ret == -EIO) {
    cmd.resp.cmd.response = VIRTIO_SCSI_S_BAD_TARGET;
    spin_lock_irqsave(&req_vq.vq_lock, flags);
    virtscsi_complete_cmd(vscsi, cmd);
    spin_unlock_irqrestore(&req_vq.vq_lock, flags);
    } else if (ret != 0) {
    return SCSI_MLQUEUE_HOST_BUSY;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn virtscsi_tmf(vscsi: *mut virtio_scsi, cmd: *mut virtio_scsi_cmd) -> c_int {
    static int virtscsi_tmf(struct virtio_scsi *vscsi, struct virtio_scsi_cmd *cmd)
    {
    DECLARE_COMPLETION_ONSTACK(comp);
    let mut ret: c_int = FAILED;
    cmd.comp = &comp;
    if (virtscsi_add_cmd(&vscsi.ctrl_vq, cmd,
    sizeof cmd.req.tmf, sizeof cmd.resp.tmf, true) < 0)
    goto out;
    wait_for_completion(&comp);
    if (cmd.resp.tmf.response == VIRTIO_SCSI_S_OK ||
    cmd.resp.tmf.response == VIRTIO_SCSI_S_FUNCTION_SUCCEEDED)
    ret = SUCCESS;
//
// The spec guarantees that all requests related to the TMF have
// been completed, but the callback might not have run yet if
// we're using independent interrupts (e.g. MSI).  Poll the
// virtqueues once.
//
// In the abort case, scsi_done() will do nothing, because the
// command timed out and hence SCMD_STATE_COMPLETE has been set.
//
    virtscsi_poll_requests(vscsi);
    out:
    mempool_free(cmd, virtscsi_cmd_pool);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn virtscsi_device_reset(sc: *mut scsi_cmnd) -> c_int {
    static int virtscsi_device_reset(struct scsi_cmnd *sc)
    {
    struct virtio_scsi *vscsi = shost_priv(sc.device.host);
    struct virtio_scsi_cmd *cmd;
    sdev_printk(KERN_INFO, sc.device, "device reset\n");
    cmd = mempool_alloc(virtscsi_cmd_pool, GFP_NOIO);
    if (!cmd)
    return FAILED;
    memset(cmd, 0, sizeof(*cmd));
    cmd.req.tmf = (struct virtio_scsi_ctrl_tmf_req){
    .type = VIRTIO_SCSI_T_TMF,
    .subtype = cpu_to_virtio32(vscsi.vdev,
    VIRTIO_SCSI_T_TMF_LOGICAL_UNIT_RESET),
    .lun[0] = 1,
    .lun[1] = sc.device.id,
    .lun[2] = (sc.device.lun >> 8) | 0x40,
    .lun[3] = sc.device.lun & 0xff,
    };
    return virtscsi_tmf(vscsi, cmd);
    }
#[no_mangle]
unsafe extern "C" fn virtscsi_device_alloc(sdevice: *mut scsi_device) -> c_int {
    static int virtscsi_device_alloc(struct scsi_device *sdevice)
    {
//
// Passed through SCSI targets (e.g. with qemu's 'scsi-block')
// may have transfer limits which come from the host SCSI
// controller or something on the host side other than the
// target itself.
//
// To make this work properly, the hypervisor can adjust the
// target's VPD information to advertise these limits.  But
// for that to work, the guest has to look at the VPD pages,
// which we won't do by default if it is an SPC-2 device, even
// if it does actually support it.
//
// So, set the blist to always try to read the VPD pages.
//
    sdevice.sdev_bflags = BLIST_TRY_VPD_PAGES;
    return 0;
    }
//
// virtscsi_change_queue_depth() - Change a virtscsi target's queue depth
// @sdev:	Virtscsi target whose queue depth to change
// @qdepth:	New queue depth
//
#[no_mangle]
unsafe extern "C" fn virtscsi_change_queue_depth(sdev: *mut scsi_device, qdepth: c_int) -> c_int {
    static int virtscsi_change_queue_depth(struct scsi_device *sdev, int qdepth)
    {
    struct Scsi_Host *shost = sdev.host;
    let mut max_depth: c_int = shost.cmd_per_lun;
    return scsi_change_queue_depth(sdev, min(max_depth, qdepth));
    }
#[no_mangle]
unsafe extern "C" fn virtscsi_abort(sc: *mut scsi_cmnd) -> c_int {
    static int virtscsi_abort(struct scsi_cmnd *sc)
    {
    struct virtio_scsi *vscsi = shost_priv(sc.device.host);
    struct virtio_scsi_cmd *cmd;
    scmd_printk(KERN_INFO, sc, "abort\n");
    cmd = mempool_alloc(virtscsi_cmd_pool, GFP_NOIO);
    if (!cmd)
    return FAILED;
    memset(cmd, 0, sizeof(*cmd));
    cmd.req.tmf = (struct virtio_scsi_ctrl_tmf_req){
    .type = VIRTIO_SCSI_T_TMF,
    .subtype = VIRTIO_SCSI_T_TMF_ABORT_TASK,
    .lun[0] = 1,
    .lun[1] = sc.device.id,
    .lun[2] = (sc.device.lun >> 8) | 0x40,
    .lun[3] = sc.device.lun & 0xff,
    .tag = cpu_to_virtio64(vscsi.vdev, (unsigned long)sc),
    };
    return virtscsi_tmf(vscsi, cmd);
    }
#[no_mangle]
unsafe extern "C" fn virtscsi_map_queues(shost: *mut Scsi_Host) {
    static void virtscsi_map_queues(struct Scsi_Host *shost)
    {
    struct virtio_scsi *vscsi = shost_priv(shost);
    int i, qoff;
    for (i = 0, qoff = 0; i < shost.nr_maps; i++) {
    struct blk_mq_queue_map *map = &shost.tag_set.map[i];
    map.nr_queues = vscsi.io_queues[i];
    map.queue_offset = qoff;
    qoff += map.nr_queues;
    if (map.nr_queues == 0)
    continue;
//
// Regular queues have interrupts and hence CPU affinity is
// defined by the core virtio code, but polling queues have
// no interrupts so we let the block layer assign CPU affinity.
//
    if (i == HCTX_TYPE_POLL)
    blk_mq_map_queues(map);
    else
    blk_mq_map_hw_queues(map, &vscsi.vdev.dev, 2);
    }
    }
#[no_mangle]
unsafe extern "C" fn virtscsi_mq_poll(shost: *mut Scsi_Host, queue_num: c_uint) -> c_int {
    static int virtscsi_mq_poll(struct Scsi_Host *shost, unsigned int queue_num)
    {
    struct virtio_scsi *vscsi = shost_priv(shost);
    struct virtio_scsi_vq *virtscsi_vq = &vscsi.req_vqs[queue_num];
    unsigned long flags;
    unsigned int len;
    let mut found: c_int = 0;
    void *buf;
    spin_lock_irqsave(&virtscsi_vq.vq_lock, flags);
    while ((buf = virtqueue_get_buf(virtscsi_vq.vq, &len)) != core::ptr::null_mut()) {
    virtscsi_complete_cmd(vscsi, buf);
    found++;
    }
    spin_unlock_irqrestore(&virtscsi_vq.vq_lock, flags);
    return found;
    }
#[no_mangle]
unsafe extern "C" fn virtscsi_commit_rqs(shost: *mut Scsi_Host, hwq: u16) {
    static void virtscsi_commit_rqs(struct Scsi_Host *shost, u16 hwq)
    {
    struct virtio_scsi *vscsi = shost_priv(shost);
    virtscsi_kick_vq(&vscsi.req_vqs[hwq]);
    }
//
// The host guarantees to respond to each command, although I/O
// latencies might be higher than on bare metal.  Reset the timer
// unconditionally to give the host a chance to perform EH.
//
#[no_mangle]
unsafe extern "C" fn virtscsi_eh_timed_out(scmnd: *mut scsi_cmnd) -> enum scsi_timeout_action {
    static enum scsi_timeout_action virtscsi_eh_timed_out(struct scsi_cmnd *scmnd)
    {
    return SCSI_EH_RESET_TIMER;
    }
    static const struct scsi_host_template virtscsi_host_template = {
    .module = THIS_MODULE,
    .name = "Virtio SCSI HBA",
    .proc_name = "virtio_scsi",
    .this_id = -1,
    .cmd_size = sizeof(struct virtio_scsi_cmd),
    .queuecommand = virtscsi_queuecommand,
    .mq_poll = virtscsi_mq_poll,
    .commit_rqs = virtscsi_commit_rqs,
    .change_queue_depth = virtscsi_change_queue_depth,
    .eh_abort_handler = virtscsi_abort,
    .eh_device_reset_handler = virtscsi_device_reset,
    .eh_timed_out = virtscsi_eh_timed_out,
    .sdev_init = virtscsi_device_alloc,
    .dma_boundary = UINT_MAX,
    .map_queues = virtscsi_map_queues,
    .track_queue_depth = 1,
    };

    ({ \
    __virtio_native_type(struct virtio_scsi_config, fld) __val; \
    virtio_cread(vdev, struct virtio_scsi_config, fld, &__val); \
    __val; \
    })

    do { \
    __virtio_native_type(struct virtio_scsi_config, fld) __val = (val); \
    virtio_cwrite(vdev, struct virtio_scsi_config, fld, &__val); \
    } while(0)
    static void virtscsi_init_vq(struct virtio_scsi_vq *virtscsi_vq,
    struct virtqueue *vq)
    {
    spin_lock_init(&virtscsi_vq.vq_lock);
    virtscsi_vq.vq = vq;
    }
#[no_mangle]
unsafe extern "C" fn virtscsi_remove_vqs(vdev: *mut virtio_device) {
    static void virtscsi_remove_vqs(struct virtio_device *vdev)
    {
// Stop all the virtqueues.
    virtio_reset_device(vdev);
    vdev.config.del_vqs(vdev);
    }
    static int virtscsi_init(struct virtio_device *vdev,
    struct virtio_scsi *vscsi)
    {
    int err;
    u32 i;
    u32 num_vqs, num_poll_vqs, num_req_vqs;
    struct virtqueue_info *vqs_info;
    struct virtqueue **vqs;
    let mut desc: irq_affinity = { .pre_vectors = 2 };
    num_req_vqs = vscsi.num_queues;
    num_vqs = num_req_vqs + VIRTIO_SCSI_VQ_BASE;
    vqs = kmalloc_objs(struct virtqueue *, num_vqs);
    vqs_info = kzalloc_objs(*vqs_info, num_vqs);
    if (!vqs || !vqs_info) {
    err = -ENOMEM;
    goto out;
    }
    num_poll_vqs = min_t(unsigned int, virtscsi_poll_queues,
    num_req_vqs - 1);
    vscsi.io_queues[HCTX_TYPE_DEFAULT] = num_req_vqs - num_poll_vqs;
    vscsi.io_queues[HCTX_TYPE_READ] = 0;
    vscsi.io_queues[HCTX_TYPE_POLL] = num_poll_vqs;
    dev_info(&vdev.dev, "%d/%d/%d default/read/poll queues\n",
    vscsi.io_queues[HCTX_TYPE_DEFAULT],
    vscsi.io_queues[HCTX_TYPE_READ],
    vscsi.io_queues[HCTX_TYPE_POLL]);
    vqs_info[0].callback = virtscsi_ctrl_done;
    vqs_info[0].name = "control";
    vqs_info[1].callback = virtscsi_event_done;
    vqs_info[1].name = "event";
    for (i = VIRTIO_SCSI_VQ_BASE; i < num_vqs - num_poll_vqs; i++) {
    vqs_info[i].callback = virtscsi_req_done;
    vqs_info[i].name = "request";
    }
    for (; i < num_vqs; i++)
    vqs_info[i].name = "request_poll";
// Discover virtqueues and write information to configuration.
    err = virtio_find_vqs(vdev, num_vqs, vqs, vqs_info, &desc);
    if (err)
    goto out;
    virtscsi_init_vq(&vscsi.ctrl_vq, vqs[0]);
    virtscsi_init_vq(&vscsi.event_vq, vqs[1]);
    for (i = VIRTIO_SCSI_VQ_BASE; i < num_vqs; i++)
    virtscsi_init_vq(&vscsi.req_vqs[i - VIRTIO_SCSI_VQ_BASE],
    vqs[i]);
    virtscsi_config_set(vdev, cdb_size, VIRTIO_SCSI_CDB_SIZE);
    virtscsi_config_set(vdev, sense_size, VIRTIO_SCSI_SENSE_SIZE);
    err = 0;
    out:
    kfree(vqs_info);
    kfree(vqs);
    if (err)
    virtscsi_remove_vqs(vdev);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn virtscsi_probe(vdev: *mut virtio_device) -> c_int {
    static int virtscsi_probe(struct virtio_device *vdev)
    {
    struct Scsi_Host *shost;
    struct virtio_scsi *vscsi;
    int err;
    u32 sg_elems, num_targets;
    u32 cmd_per_lun;
    u32 num_queues;
    if (!vdev.config.get) {
    dev_err(&vdev.dev, "%s failure: config access disabled\n",
    __func__);
    return -EINVAL;
    }
// We need to know how many queues before we allocate.
    num_queues = virtscsi_config_get(vdev, num_queues) ? : 1;
    num_queues = min_t(unsigned int, nr_cpu_ids, num_queues);
    num_queues = blk_mq_num_possible_queues(num_queues);
    num_targets = virtscsi_config_get(vdev, max_target) + 1;
    shost = scsi_host_alloc(&virtscsi_host_template,
    struct_size(vscsi, req_vqs, num_queues));
    if (!shost)
    return -ENOMEM;
    sg_elems = virtscsi_config_get(vdev, seg_max) ?: 1;
    shost.sg_tablesize = sg_elems;
    shost.nr_maps = 1;
    vscsi = shost_priv(shost);
    vscsi.vdev = vdev;
    vscsi.num_queues = num_queues;
    vdev.priv = shost;
    err = virtscsi_init(vdev, vscsi);
    if (err)
    goto virtscsi_init_failed;
    if (vscsi.io_queues[HCTX_TYPE_POLL])
    shost.nr_maps = HCTX_TYPE_POLL + 1;
    shost.can_queue = virtqueue_get_vring_size(vscsi.req_vqs[0].vq);
    cmd_per_lun = virtscsi_config_get(vdev, cmd_per_lun) ?: 1;
    shost.cmd_per_lun = min_t(u32, cmd_per_lun, shost.can_queue);
    shost.max_sectors = virtscsi_config_get(vdev, max_sectors) ?: 0xFFFF;
// LUNs > 256 are reported with format 1, so they go in the range
// 16640-32767.
//
    shost.max_lun = virtscsi_config_get(vdev, max_lun) + 1 + 0x4000;
    shost.max_id = num_targets;
    shost.max_channel = 0;
    shost.max_cmd_len = VIRTIO_SCSI_CDB_SIZE;
    shost.nr_hw_queues = num_queues;

    if (virtio_has_feature(vdev, VIRTIO_SCSI_F_T10_PI)) {
    int host_prot;
    host_prot = SHOST_DIF_TYPE1_PROTECTION | SHOST_DIF_TYPE2_PROTECTION |
    SHOST_DIF_TYPE3_PROTECTION | SHOST_DIX_TYPE1_PROTECTION |
    SHOST_DIX_TYPE2_PROTECTION | SHOST_DIX_TYPE3_PROTECTION;
    scsi_host_set_prot(shost, host_prot);
    scsi_host_set_guard(shost, SHOST_DIX_GUARD_CRC);
    }

    err = scsi_add_host(shost, &vdev.dev);
    if (err)
    goto scsi_add_host_failed;
    virtio_device_ready(vdev);
    for (int i = 0; i < VIRTIO_SCSI_EVENT_LEN; i++)
    INIT_WORK(&vscsi.event_list[i].work, virtscsi_handle_event);
    virtscsi_kick_event_all(vscsi);
    scsi_scan_host(shost);
    return 0;
    scsi_add_host_failed:
    vdev.config.del_vqs(vdev);
    virtscsi_init_failed:
    scsi_host_put(shost);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn virtscsi_remove(vdev: *mut virtio_device) {
    static void virtscsi_remove(struct virtio_device *vdev)
    {
    struct Scsi_Host *shost = virtio_scsi_host(vdev);
    struct virtio_scsi *vscsi = shost_priv(shost);
    virtscsi_cancel_event_work(vscsi);
    scsi_remove_host(shost);
    virtscsi_remove_vqs(vdev);
    scsi_host_put(shost);
    }

#[no_mangle]
unsafe extern "C" fn virtscsi_freeze(vdev: *mut virtio_device) -> c_int {
    static int virtscsi_freeze(struct virtio_device *vdev)
    {
    virtscsi_remove_vqs(vdev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn virtscsi_restore(vdev: *mut virtio_device) -> c_int {
    static int virtscsi_restore(struct virtio_device *vdev)
    {
    struct Scsi_Host *sh = virtio_scsi_host(vdev);
    struct virtio_scsi *vscsi = shost_priv(sh);
    int err;
    err = virtscsi_init(vdev, vscsi);
    if (err)
    return err;
    virtio_device_ready(vdev);
    virtscsi_kick_event_all(vscsi);
    return err;
    }

    static struct virtio_device_id id_table[] = {
    { VIRTIO_ID_SCSI, VIRTIO_DEV_ANY_ID },
    { 0 },
    };
    static unsigned int features[] = {
    VIRTIO_SCSI_F_HOTPLUG,
    VIRTIO_SCSI_F_CHANGE,

    VIRTIO_SCSI_F_T10_PI,

    };
    static struct virtio_driver virtio_scsi_driver = {
    .feature_table = features,
    .feature_table_size = ARRAY_SIZE(features),
    .driver.name = KBUILD_MODNAME,
    .id_table = id_table,
    .probe = virtscsi_probe,

    .freeze = virtscsi_freeze,
    .restore = virtscsi_restore,

    .remove = virtscsi_remove,
    };
#[no_mangle]
unsafe extern "C" fn virtio_scsi_init() -> int __init {
    static int __init virtio_scsi_init(void)
    {
    let mut ret: c_int = -ENOMEM;
    virtscsi_cmd_cache = KMEM_CACHE(virtio_scsi_cmd, 0);
    if (!virtscsi_cmd_cache) {
    pr_err("kmem_cache_create() for virtscsi_cmd_cache failed\n");
    goto error;
    }
    virtscsi_cmd_pool =
    mempool_create_slab_pool(VIRTIO_SCSI_MEMPOOL_SZ,
    virtscsi_cmd_cache);
    if (!virtscsi_cmd_pool) {
    pr_err("mempool_create() for virtscsi_cmd_pool failed\n");
    goto error;
    }
    ret = register_virtio_driver(&virtio_scsi_driver);
    if (ret < 0)
    goto error;
    return 0;
    error:
    mempool_destroy(virtscsi_cmd_pool);
    virtscsi_cmd_pool = core::ptr::null_mut();
    kmem_cache_destroy(virtscsi_cmd_cache);
    virtscsi_cmd_cache = core::ptr::null_mut();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn virtio_scsi_fini() -> void __exit {
    static void __exit virtio_scsi_fini(void)
    {
    unregister_virtio_driver(&virtio_scsi_driver);
    mempool_destroy(virtscsi_cmd_pool);
    kmem_cache_destroy(virtscsi_cmd_cache);
    }
    module_init(virtio_scsi_init);
    module_exit(virtio_scsi_fini);
    MODULE_DEVICE_TABLE(virtio, id_table);
    MODULE_DESCRIPTION("Virtio SCSI HBA driver");
    MODULE_LICENSE("GPL");
