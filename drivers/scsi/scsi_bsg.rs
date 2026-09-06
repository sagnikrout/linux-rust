//! Automatically rewritten from C to Rust
//! Source: drivers/scsi/scsi_bsg.c
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
// Per-command BSG SCSI PDU stored in io_uring_cmd.pdu[32].
// Holds temporary state between submission, completion and task_work.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_bsg_uring_cmd_pdu {
    pub /: *mut *mut *mut bio bio; / mapped user buffer, unmap in task work,
    pub /: *mut *mut *mut request req; / block request, freed in task work,
    pub /: *mut *mut u64 response_addr; / user space response buffer address,
    pub /: *mut *mut u32 max_response_len; / user response buffer size,
}

    static_assert(sizeof(struct scsi_bsg_uring_cmd_pdu) <= sizeof_field(struct io_uring_cmd, pdu));
    static inline struct scsi_bsg_uring_cmd_pdu *scsi_bsg_uring_cmd_pdu(
    struct io_uring_cmd *ioucmd)
    {
    return io_uring_cmd_to_pdu(ioucmd, struct scsi_bsg_uring_cmd_pdu);
    }
// Task work: build res2 (layout in uapi/linux/bsg.h) and copy sense to user.
#[no_mangle]
unsafe extern "C" fn scsi_bsg_uring_task_cb(tw_req: io_tw_req, tw: io_tw_token_t) {
    static void scsi_bsg_uring_task_cb(struct io_tw_req tw_req, io_tw_token_t tw)
    {
    struct io_uring_cmd *ioucmd = io_uring_cmd_from_tw(tw_req);
    struct scsi_bsg_uring_cmd_pdu *pdu = scsi_bsg_uring_cmd_pdu(ioucmd);
    struct request *rq = pdu.req;
    struct scsi_cmnd *scmd = blk_mq_rq_to_pdu(rq);
    u64 res2;
    let mut ret: c_int = 0;
    let mut driver_status: u8 = 0;
    let mut sense_len_wr: u8 = 0;
    if (pdu.bio)
    blk_rq_unmap_user(pdu.bio);
    if (scsi_status_is_check_condition(scmd.result)) {
    driver_status = DRIVER_SENSE;
    if (pdu.response_addr)
    sense_len_wr = min_t(unsigned int, pdu.max_response_len,
    scmd.sense_len);
    }
    if (sense_len_wr) {
    if (copy_to_user(uptr64(pdu.response_addr), scmd.sense_buffer,
    sense_len_wr))
    ret = -EFAULT;
    }
    res2 = bsg_scsi_res2_build(status_byte(scmd.result), driver_status,
    host_byte(scmd.result), sense_len_wr,
    scmd.resid_len);
    blk_mq_free_request(rq);
    io_uring_cmd_done32(ioucmd, ret, res2,
    IO_URING_CMD_TASK_WORK_ISSUE_FLAGS);
    }
    static enum rq_end_io_ret scsi_bsg_uring_cmd_done(struct request *req,
    blk_status_t status,
    const struct io_comp_batch *iocb)
    {
    struct io_uring_cmd *ioucmd = req.end_io_data;
    io_uring_cmd_do_in_task_lazy(ioucmd, scsi_bsg_uring_task_cb);
    return RQ_END_IO_NONE;
    }
    static int scsi_bsg_map_user_buffer(struct request *req,
    struct io_uring_cmd *ioucmd,
    unsigned int issue_flags, gfp_t gfp_mask,
    bool is_write, u64 buf_addr,
    unsigned long buf_len)
    {
    struct iov_iter iter;
    int ret;
    if (ioucmd.flags & IORING_URING_CMD_FIXED) {
    ret = io_uring_cmd_import_fixed(buf_addr, buf_len,
    is_write ? WRITE : READ,
    &iter, ioucmd, issue_flags);
    if (ret < 0)
    return ret;
    ret = blk_rq_map_user_iov(req.q, req, core::ptr::null_mut(), &iter, gfp_mask);
    } else {
    ret = blk_rq_map_user(req.q, req, core::ptr::null_mut(), uptr64(buf_addr),
    buf_len, gfp_mask);
    }
    return ret;
    }
    static int scsi_bsg_uring_cmd(struct request_queue *q, struct io_uring_cmd *ioucmd,
    unsigned int issue_flags, bool open_for_write)
    {
    struct scsi_bsg_uring_cmd_pdu *pdu = scsi_bsg_uring_cmd_pdu(ioucmd);
    const struct bsg_uring_cmd *cmd =
    io_uring_sqe128_cmd(ioucmd.sqe, struct bsg_uring_cmd);
    struct scsi_cmnd *scmd;
    struct request *req;
    let mut blk_flags: blk_mq_req_flags_t = 0;
    let mut gfp_mask: gfp_t = GFP_KERNEL;
    let mut request: u64 = READ_ONCE(cmd.request);
    let mut request_len: u32 = READ_ONCE(cmd.request_len);
    let mut dout_xferp: u64 = READ_ONCE(cmd.dout_xferp);
    let mut dout_xfer_len: u32 = READ_ONCE(cmd.dout_xfer_len);
    let mut din_xferp: u64 = READ_ONCE(cmd.din_xferp);
    let mut din_xfer_len: u32 = READ_ONCE(cmd.din_xfer_len);
    int ret;
    if (cmd.protocol != BSG_PROTOCOL_SCSI ||
    cmd.subprotocol != BSG_SUB_PROTOCOL_SCSI_CMD)
    return -EINVAL;
    if (!request || request_len == 0)
    return -EINVAL;
    if (dout_xfer_len && din_xfer_len) {
    pr_warn_once("BIDI support in bsg has been removed.\n");
    return -EOPNOTSUPP;
    }
    if (cmd.dout_iovec_count > 0 || cmd.din_iovec_count > 0)
    return -EOPNOTSUPP;
    if (issue_flags & IO_URING_F_NONBLOCK) {
    blk_flags = BLK_MQ_REQ_NOWAIT;
    gfp_mask = GFP_NOWAIT;
    }
    req = scsi_alloc_request(q, dout_xfer_len ?
    REQ_OP_DRV_OUT : REQ_OP_DRV_IN, blk_flags);
    if (IS_ERR(req))
    return PTR_ERR(req);
    scmd = blk_mq_rq_to_pdu(req);
    if (request_len > sizeof(scmd.cmnd)) {
    ret = -EINVAL;
    goto out_free_req;
    }
    scmd.cmd_len = request_len;
    scmd.allowed = SG_DEFAULT_RETRIES;
    if (copy_from_user(scmd.cmnd, uptr64(request), request_len)) {
    ret = -EFAULT;
    goto out_free_req;
    }
    if (!scsi_cmd_allowed(scmd.cmnd, open_for_write)) {
    ret = -EPERM;
    goto out_free_req;
    }
    pdu.response_addr = cmd.response;
    pdu.max_response_len = cmd.max_response_len;
    if (dout_xfer_len || din_xfer_len) {
    let mut is_write: bool = dout_xfer_len > 0;
    let mut buf_addr: u64 = is_write ? dout_xferp : din_xferp;
    let mut buf_len: c_ulong = is_write ? dout_xfer_len : din_xfer_len;
    ret = scsi_bsg_map_user_buffer(req, ioucmd, issue_flags,
    gfp_mask, is_write, buf_addr,
    buf_len);
    if (ret)
    goto out_free_req;
    pdu.bio = req.bio;
    } else {
    pdu.bio = core::ptr::null_mut();
    }
    req.timeout = cmd.timeout_ms ?
    msecs_to_jiffies(cmd.timeout_ms) : BLK_DEFAULT_SG_TIMEOUT;
    req.end_io = scsi_bsg_uring_cmd_done;
    req.end_io_data = ioucmd;
    pdu.req = req;
    blk_execute_rq_nowait(req, false);
    return -EIOCBQUEUED;
    out_free_req:
    blk_mq_free_request(req);
    return ret;
    }
    static int scsi_bsg_sg_io_fn(struct request_queue *q, struct sg_io_v4 *hdr,
    bool open_for_write, unsigned int timeout)
    {
    struct scsi_cmnd *scmd;
    struct request *rq;
    struct bio *bio;
    int ret;
    if (hdr.protocol != BSG_PROTOCOL_SCSI  ||
    hdr.subprotocol != BSG_SUB_PROTOCOL_SCSI_CMD)
    return -EINVAL;
    if (hdr.dout_xfer_len && hdr.din_xfer_len) {
    pr_warn_once("BIDI support in bsg has been removed.\n");
    return -EOPNOTSUPP;
    }
    rq = scsi_alloc_request(q, hdr.dout_xfer_len ?
    REQ_OP_DRV_OUT : REQ_OP_DRV_IN, 0);
    if (IS_ERR(rq))
    return PTR_ERR(rq);
    rq.timeout = timeout;
    scmd = blk_mq_rq_to_pdu(rq);
    scmd.cmd_len = hdr.request_len;
    if (scmd.cmd_len > sizeof(scmd.cmnd)) {
    ret = -EINVAL;
    goto out_put_request;
    }
    ret = -EFAULT;
    if (copy_from_user(scmd.cmnd, uptr64(hdr.request), scmd.cmd_len))
    goto out_put_request;
    ret = -EPERM;
    if (!scsi_cmd_allowed(scmd.cmnd, open_for_write))
    goto out_put_request;
    ret = 0;
    if (hdr.dout_xfer_len) {
    ret = blk_rq_map_user(rq.q, rq, core::ptr::null_mut(), uptr64(hdr.dout_xferp),
    hdr.dout_xfer_len, GFP_KERNEL);
    } else if (hdr.din_xfer_len) {
    ret = blk_rq_map_user(rq.q, rq, core::ptr::null_mut(), uptr64(hdr.din_xferp),
    hdr.din_xfer_len, GFP_KERNEL);
    }
    if (ret)
    goto out_put_request;
    bio = rq.bio;
    blk_execute_rq(rq, !(hdr.flags & BSG_FLAG_Q_AT_TAIL));
//
// fill in all the output members
//
    hdr.device_status = scmd.result & 0xff;
    hdr.transport_status = host_byte(scmd.result);
    hdr.driver_status = 0;
    if (scsi_status_is_check_condition(scmd.result))
    hdr.driver_status = DRIVER_SENSE;
    hdr.info = 0;
    if (hdr.device_status || hdr.transport_status || hdr.driver_status)
    hdr.info |= SG_INFO_CHECK;
    hdr.response_len = 0;
    if (scmd.sense_len && hdr.response) {
    int len = min_t(unsigned int, hdr.max_response_len,
    scmd.sense_len);
    if (copy_to_user(uptr64(hdr.response), scmd.sense_buffer,
    len))
    ret = -EFAULT;
    else
    hdr.response_len = len;
    }
    if (rq_data_dir(rq) == READ)
    hdr.din_resid = scmd.resid_len;
    else
    hdr.dout_resid = scmd.resid_len;
    blk_rq_unmap_user(bio);
    out_put_request:
    blk_mq_free_request(rq);
    return ret;
    }
    struct bsg_device *scsi_bsg_register_queue(struct scsi_device *sdev)
    {
    return bsg_register_queue(sdev.request_queue, &sdev.sdev_gendev,
    dev_name(&sdev.sdev_gendev), scsi_bsg_sg_io_fn,
    scsi_bsg_uring_cmd);
    }
