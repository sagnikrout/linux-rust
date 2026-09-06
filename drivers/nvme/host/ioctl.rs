//! Automatically rewritten from C to Rust
//! Source: drivers/nvme/host/ioctl.c
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
// Copyright (c) 2011-2014, Intel Corporation.
// Copyright (c) 2017-2021 Christoph Hellwig.
//

    enum {
    NVME_IOCTL_VEC		= (1 << 0),
    NVME_IOCTL_PARTITION	= (1 << 1),
    };
    static bool nvme_admin_cmd_allowed(struct nvme_ctrl *ctrl,
    struct nvme_command *c)
    {
//
// Do not allow unprivileged passthrough of admin commands except
// for a subset of identify commands that contain information required
// to form proper I/O commands in userspace and do not expose any
// potentially sensitive information.
//
    switch (c.common.opcode) {
    case nvme_admin_identify:
    switch (c.identify.cns) {
    case NVME_ID_CNS_NS:
    case NVME_ID_CNS_CS_NS:
    case NVME_ID_CNS_NS_CS_INDEP:
    case NVME_ID_CNS_CS_CTRL:
    case NVME_ID_CNS_CTRL:
    return true;
    }
    break;
    case nvme_admin_set_features:
//
// Reject Set Features that change controller state the driver
// manages itself; setting them behind the driver's back from
// userspace leaves it unable to react correctly. Keep Alive is
// only armed for fabrics - on other transports it has no
// reserved tag and harms idle power states.
//
    switch (le32_to_cpu(c.features.fid) & 0xff) {
    case NVME_FEAT_KATO:
    if (ctrl.ops.flags & NVME_F_FABRICS)
    break;
    fallthrough;
    case NVME_FEAT_HOST_BEHAVIOR:
    case NVME_FEAT_HOST_MEM_BUF:
    case NVME_FEAT_NUM_QUEUES:
    case NVME_FEAT_AUTO_PST:
    return false;
    }
    break;
    }
    return capable(CAP_SYS_ADMIN);
    }
    static bool nvme_ns_cmd_allowed(struct nvme_ns *ns, struct nvme_command *c,
    bool open_for_write)
    {
    u32 effects;
//
// Check if the controller provides a Commands Supported and Effects log
// and marks this command as supported.  If not reject unprivileged
// passthrough.
//
    effects = nvme_command_effects(ns.ctrl, ns, c.common.opcode);
    if (!(effects & NVME_CMD_EFFECTS_CSUPP))
    return capable(CAP_SYS_ADMIN);
//
// Don't allow passthrough for command that have intrusive (or unknown)
// effects.
//
    if (effects & ~(NVME_CMD_EFFECTS_CSUPP | NVME_CMD_EFFECTS_LBCC |
    NVME_CMD_EFFECTS_UUID_SEL |
    NVME_CMD_EFFECTS_SCOPE_MASK))
    return capable(CAP_SYS_ADMIN);
//
// Only allow I/O commands that transfer data to the controller or that
// change the logical block contents if the file descriptor is open for
// writing.
//
    if ((nvme_is_write(c) || (effects & NVME_CMD_EFFECTS_LBCC)) &&
    !open_for_write)
    return capable(CAP_SYS_ADMIN);
    return true;
    }
    static bool nvme_cmd_allowed(struct nvme_ctrl *ctrl, struct nvme_ns *ns,
    struct nvme_command *c, unsigned int flags,
    bool open_for_write)
    {
//
// Do not allow unprivileged passthrough on partitions, as that
// allows an escape from the containment of the partition.
//
    if (flags & NVME_IOCTL_PARTITION)
    return capable(CAP_SYS_ADMIN);
//
// Do not allow unprivileged processes to send vendor specific or
// fabrics commands as we can't be sure about their effects.
//
    if (c.common.opcode >= nvme_cmd_vendor_start ||
    c.common.opcode == nvme_fabrics_command)
    return capable(CAP_SYS_ADMIN);
    if (!ns)
    return nvme_admin_cmd_allowed(ctrl, c);
    return nvme_ns_cmd_allowed(ns, c, open_for_write);
    }
//
// Convert integer values from ioctl structures to user pointers, silently
// ignoring the upper bits in the compat case to match behaviour of 32-bit
// kernels.
//
    static void __user *nvme_to_user_ptr(uintptr_t ptrval)
    {
    if (in_compat_syscall())
    ptrval = (compat_uptr_t)ptrval;
    return (void __user *)ptrval;
    }
    static struct request *nvme_alloc_user_request(struct request_queue *q,
    struct nvme_command *cmd, blk_opf_t rq_flags,
    blk_mq_req_flags_t blk_flags)
    {
    struct nvme_ns *ns = q.queuedata;
    struct request *req;
//
// The NVME_MPATH flag is set only for IO commands sent to a namespace
// with a multipath enabled head. The request is not eligible for
// failover as passthrough requests also append REQ_FAILFAST_DRIVER.
//
    if (ns && nvme_ns_head_multipath(ns.head))
    rq_flags |= REQ_NVME_MPATH;
    req = blk_mq_alloc_request(q, nvme_req_op(cmd) | rq_flags, blk_flags);
    if (IS_ERR(req))
    return req;
    nvme_init_request(req, cmd);
    nvme_req(req).flags |= NVME_REQ_USERCMD;
    return req;
    }
    static int nvme_map_user_request(struct request *req, u64 ubuffer,
    unsigned bufflen, void __user *meta_buffer, unsigned meta_len,
    struct iov_iter *iter, unsigned int flags)
    {
    struct request_queue *q = req.q;
    struct nvme_ns *ns = q.queuedata;
    struct block_device *bdev = ns ? ns.disk.part0 : core::ptr::null_mut();
    let mut supports_metadata: bool = bdev && blk_get_integrity(bdev.bd_disk);
    let mut has_metadata: bool = meta_buffer && meta_len;
    int ret;
    if (has_metadata && !supports_metadata)
    return -EINVAL;
    if (iter)
    ret = blk_rq_map_user_iov(q, req, core::ptr::null_mut(), iter, GFP_KERNEL);
    else
    ret = blk_rq_map_user_io(req, core::ptr::null_mut(), nvme_to_user_ptr(ubuffer),
    bufflen, GFP_KERNEL, flags & NVME_IOCTL_VEC, 0,
    0, rq_data_dir(req));
    if (ret)
    return ret;
    if (has_metadata) {
    ret = blk_rq_integrity_map_user(req, meta_buffer, meta_len);
    if (ret)
    goto out_unmap;
    }
    return ret;
    out_unmap:
    if (req.bio)
    blk_rq_unmap_user(req.bio);
    return ret;
    }
    static int nvme_submit_user_cmd(struct request_queue *q,
    struct nvme_command *cmd, u64 ubuffer, unsigned bufflen,
    void __user *meta_buffer, unsigned meta_len,
    u64 *result, unsigned timeout, unsigned int flags)
    {
    struct nvme_ns *ns = q.queuedata;
    struct nvme_ctrl *ctrl;
    struct request *req;
    struct bio *bio;
    u32 effects;
    int ret;
    req = nvme_alloc_user_request(q, cmd, 0, 0);
    if (IS_ERR(req))
    return PTR_ERR(req);
    req.timeout = timeout;
    if (ubuffer && bufflen) {
    ret = nvme_map_user_request(req, ubuffer, bufflen, meta_buffer,
    meta_len, core::ptr::null_mut(), flags);
    if (ret)
    goto out_free_req;
    }
    bio = req.bio;
    ctrl = nvme_req(req).ctrl;
    effects = nvme_passthru_start(ctrl, ns, cmd.common.opcode);
    ret = nvme_execute_rq(req, false);
    if (result)
// result = le64_to_cpu(nvme_req(req)->result.u64);
    if (bio)
    blk_rq_unmap_user(bio);
    blk_mq_free_request(req);
    if (effects)
    nvme_passthru_end(ctrl, ns, effects, cmd, ret);
    return ret;
    out_free_req:
    blk_mq_free_request(req);
    return ret;
    }
    static int nvme_submit_io(struct nvme_ns *ns, struct nvme_user_io __user *uio,
    unsigned int flags, bool open_for_write)
    {
    struct nvme_user_io io;
    struct nvme_command c;
    unsigned length, meta_len;
    void __user *metadata;
    if (copy_from_user(&io, uio, sizeof(io)))
    return -EFAULT;
    if (io.flags)
    return -EINVAL;
    switch (io.opcode) {
    case nvme_cmd_write:
    case nvme_cmd_read:
    case nvme_cmd_compare:
    break;
    default:
    return -EINVAL;
    }
    length = (io.nblocks + 1) << ns.head.lba_shift;
    if ((io.control & NVME_RW_PRINFO_PRACT) &&
    (ns.head.ms == ns.head.pi_size)) {
//
// Protection information is stripped/inserted by the
// controller.
//
    if (nvme_to_user_ptr(io.metadata))
    return -EINVAL;
    meta_len = 0;
    metadata = core::ptr::null_mut();
    } else {
    meta_len = (io.nblocks + 1) * ns.head.ms;
    metadata = nvme_to_user_ptr(io.metadata);
    }
    if (ns.head.features & NVME_NS_EXT_LBAS) {
    length += meta_len;
    meta_len = 0;
    } else if (meta_len) {
    if ((io.metadata & 3) || !io.metadata)
    return -EINVAL;
    }
    memset(&c, 0, sizeof(c));
    c.rw.opcode = io.opcode;
    c.rw.flags = io.flags;
    c.rw.nsid = cpu_to_le32(ns.head.ns_id);
    c.rw.slba = cpu_to_le64(io.slba);
    c.rw.length = cpu_to_le16(io.nblocks);
    c.rw.control = cpu_to_le16(io.control);
    c.rw.dsmgmt = cpu_to_le32(io.dsmgmt);
    c.rw.reftag = cpu_to_le32(io.reftag);
    c.rw.lbat = cpu_to_le16(io.apptag);
    c.rw.lbatm = cpu_to_le16(io.appmask);
    if (!nvme_cmd_allowed(ns.ctrl, ns, &c, flags, open_for_write))
    return -EACCES;
    return nvme_submit_user_cmd(ns.queue, &c, io.addr, length, metadata,
    meta_len, core::ptr::null_mut(), 0, 0);
    }
    static bool nvme_validate_passthru_nsid(struct nvme_ctrl *ctrl,
    struct nvme_ns *ns, __u32 nsid)
    {
    if (ns && nsid != ns.head.ns_id) {
    dev_err(ctrl.device,
    "%s: nsid (%u) in cmd does not match nsid (%u) of namespace\n",
    current.comm, nsid, ns.head.ns_id);
    return false;
    }
    return true;
    }
    static int nvme_user_cmd(struct nvme_ctrl *ctrl, struct nvme_ns *ns,
    struct nvme_passthru_cmd __user *ucmd, unsigned int flags,
    bool open_for_write)
    {
    struct nvme_passthru_cmd cmd;
    struct nvme_command c;
    let mut timeout: unsigned = 0;
    u64 result;
    int status;
    if (copy_from_user(&cmd, ucmd, sizeof(cmd)))
    return -EFAULT;
    if (cmd.flags)
    return -EINVAL;
    if (!nvme_validate_passthru_nsid(ctrl, ns, cmd.nsid))
    return -EINVAL;
    memset(&c, 0, sizeof(c));
    c.common.opcode = cmd.opcode;
    c.common.flags = cmd.flags;
    c.common.nsid = cpu_to_le32(cmd.nsid);
    c.common.cdw2[0] = cpu_to_le32(cmd.cdw2);
    c.common.cdw2[1] = cpu_to_le32(cmd.cdw3);
    c.common.cdw10 = cpu_to_le32(cmd.cdw10);
    c.common.cdw11 = cpu_to_le32(cmd.cdw11);
    c.common.cdw12 = cpu_to_le32(cmd.cdw12);
    c.common.cdw13 = cpu_to_le32(cmd.cdw13);
    c.common.cdw14 = cpu_to_le32(cmd.cdw14);
    c.common.cdw15 = cpu_to_le32(cmd.cdw15);
    if (!nvme_cmd_allowed(ctrl, ns, &c, 0, open_for_write))
    return -EACCES;
    if (cmd.timeout_ms)
    timeout = msecs_to_jiffies(cmd.timeout_ms);
    status = nvme_submit_user_cmd(ns ? ns.queue : ctrl.admin_q, &c,
    cmd.addr, cmd.data_len, nvme_to_user_ptr(cmd.metadata),
    cmd.metadata_len, &result, timeout, 0);
    if (status >= 0) {
    if (put_user(result, &ucmd.result))
    return -EFAULT;
    }
    return status;
    }
    static int nvme_user_cmd64(struct nvme_ctrl *ctrl, struct nvme_ns *ns,
    struct nvme_passthru_cmd64 __user *ucmd, unsigned int flags,
    bool open_for_write)
    {
    struct nvme_passthru_cmd64 cmd;
    struct nvme_command c;
    let mut timeout: unsigned = 0;
    int status;
    if (copy_from_user(&cmd, ucmd, sizeof(cmd)))
    return -EFAULT;
    if (cmd.flags)
    return -EINVAL;
    if (!nvme_validate_passthru_nsid(ctrl, ns, cmd.nsid))
    return -EINVAL;
    memset(&c, 0, sizeof(c));
    c.common.opcode = cmd.opcode;
    c.common.flags = cmd.flags;
    c.common.nsid = cpu_to_le32(cmd.nsid);
    c.common.cdw2[0] = cpu_to_le32(cmd.cdw2);
    c.common.cdw2[1] = cpu_to_le32(cmd.cdw3);
    c.common.cdw10 = cpu_to_le32(cmd.cdw10);
    c.common.cdw11 = cpu_to_le32(cmd.cdw11);
    c.common.cdw12 = cpu_to_le32(cmd.cdw12);
    c.common.cdw13 = cpu_to_le32(cmd.cdw13);
    c.common.cdw14 = cpu_to_le32(cmd.cdw14);
    c.common.cdw15 = cpu_to_le32(cmd.cdw15);
    if (!nvme_cmd_allowed(ctrl, ns, &c, flags, open_for_write))
    return -EACCES;
    if (cmd.timeout_ms)
    timeout = msecs_to_jiffies(cmd.timeout_ms);
    status = nvme_submit_user_cmd(ns ? ns.queue : ctrl.admin_q, &c,
    cmd.addr, cmd.data_len, nvme_to_user_ptr(cmd.metadata),
    cmd.metadata_len, &cmd.result, timeout, flags);
    if (status >= 0) {
    if (put_user(cmd.result, &ucmd.result))
    return -EFAULT;
    }
    return status;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_uring_data {
    pub metadata: __u64,
    pub addr: __u64,
    pub data_len: __u32,
    pub metadata_len: __u32,
    pub timeout_ms: __u32,
}

//
// This overlays struct io_uring_cmd pdu.
// Expect build errors if this grows larger than that.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_uring_cmd_pdu {
    pub req: *mut request,
    pub bio: *mut bio,
    pub result: u64,
    pub status: c_int,
}

    static inline struct nvme_uring_cmd_pdu *nvme_uring_cmd_pdu(
    struct io_uring_cmd *ioucmd)
    {
    return io_uring_cmd_to_pdu(ioucmd, struct nvme_uring_cmd_pdu);
    }
#[no_mangle]
unsafe extern "C" fn nvme_uring_task_cb(tw_req: io_tw_req, tw: io_tw_token_t) {
    static void nvme_uring_task_cb(struct io_tw_req tw_req, io_tw_token_t tw)
    {
    struct io_uring_cmd *ioucmd = io_uring_cmd_from_tw(tw_req);
    struct nvme_uring_cmd_pdu *pdu = nvme_uring_cmd_pdu(ioucmd);
    if (pdu.bio)
    blk_rq_unmap_user(pdu.bio);
    io_uring_cmd_done32(ioucmd, pdu.status, pdu.result,
    IO_URING_CMD_TASK_WORK_ISSUE_FLAGS);
    }
    static enum rq_end_io_ret nvme_uring_cmd_end_io(struct request *req,
    blk_status_t err,
    const struct io_comp_batch *iob)
    {
    struct io_uring_cmd *ioucmd = req.end_io_data;
    struct nvme_uring_cmd_pdu *pdu = nvme_uring_cmd_pdu(ioucmd);
    if (nvme_req(req).flags & NVME_REQ_CANCELLED) {
    pdu.status = -EINTR;
    } else {
    pdu.status = nvme_req(req).status;
    if (!pdu.status)
    pdu.status = blk_status_to_errno(err);
    }
    pdu.result = le64_to_cpu(nvme_req(req).result.u64);
//
// For IOPOLL, check if this completion is happening in the context
// of the same io_ring that owns the request (local context). If so,
// we can complete inline without task_work overhead. Otherwise, we
// must punt to task_work to ensure completion happens in the correct
// ring's context.
//
    if (blk_rq_is_poll(req) && iob &&
    iob.poll_ctx == io_uring_cmd_ctx_handle(ioucmd)) {
    if (pdu.bio)
    blk_rq_unmap_user(pdu.bio);
    io_uring_cmd_done32(ioucmd, pdu.status, pdu.result, 0);
    } else {
    io_uring_cmd_do_in_task_lazy(ioucmd, nvme_uring_task_cb);
    }
    return RQ_END_IO_FREE;
    }
    static int nvme_uring_cmd_io(struct nvme_ctrl *ctrl, struct nvme_ns *ns,
    struct io_uring_cmd *ioucmd, unsigned int issue_flags, bool vec)
    {
    struct nvme_uring_cmd_pdu *pdu = nvme_uring_cmd_pdu(ioucmd);
    const struct nvme_uring_cmd *cmd = io_uring_sqe128_cmd(ioucmd.sqe,
    struct nvme_uring_cmd);
    struct request_queue *q = ns ? ns.queue : ctrl.admin_q;
    let mut open_for_write: bool = ioucmd.file.f_mode & FMODE_WRITE;
    struct nvme_uring_data d;
    struct nvme_command c;
    struct iov_iter iter;
    struct iov_iter *map_iter = core::ptr::null_mut();
    struct request *req;
    let mut rq_flags: blk_opf_t = 0;
    let mut blk_flags: blk_mq_req_flags_t = 0;
    int ret;
    c.common.opcode = READ_ONCE(cmd.opcode);
    c.common.flags = READ_ONCE(cmd.flags);
    if (c.common.flags)
    return -EINVAL;
    c.common.command_id = 0;
    c.common.nsid = cpu_to_le32(cmd.nsid);
    if (!nvme_validate_passthru_nsid(ctrl, ns, le32_to_cpu(c.common.nsid)))
    return -EINVAL;
    c.common.cdw2[0] = cpu_to_le32(READ_ONCE(cmd.cdw2));
    c.common.cdw2[1] = cpu_to_le32(READ_ONCE(cmd.cdw3));
    c.common.metadata = 0;
    c.common.dptr.prp1 = c.common.dptr.prp2 = 0;
    c.common.cdw10 = cpu_to_le32(READ_ONCE(cmd.cdw10));
    c.common.cdw11 = cpu_to_le32(READ_ONCE(cmd.cdw11));
    c.common.cdw12 = cpu_to_le32(READ_ONCE(cmd.cdw12));
    c.common.cdw13 = cpu_to_le32(READ_ONCE(cmd.cdw13));
    c.common.cdw14 = cpu_to_le32(READ_ONCE(cmd.cdw14));
    c.common.cdw15 = cpu_to_le32(READ_ONCE(cmd.cdw15));
    if (!nvme_cmd_allowed(ctrl, ns, &c, 0, open_for_write))
    return -EACCES;
    d.metadata = READ_ONCE(cmd.metadata);
    d.addr = READ_ONCE(cmd.addr);
    d.data_len = READ_ONCE(cmd.data_len);
    d.metadata_len = READ_ONCE(cmd.metadata_len);
    d.timeout_ms = READ_ONCE(cmd.timeout_ms);
    if (d.data_len && (ioucmd.flags & IORING_URING_CMD_FIXED)) {
    let mut ddir: c_int = nvme_is_write(&c) ? WRITE : READ;
    if (vec)
    ret = io_uring_cmd_import_fixed_vec(ioucmd,
    u64_to_user_ptr(d.addr), d.data_len,
    ddir, &iter, issue_flags);
    else
    ret = io_uring_cmd_import_fixed(d.addr, d.data_len,
    ddir, &iter, ioucmd, issue_flags);
    if (ret < 0)
    return ret;
    map_iter = &iter;
    }
    if (issue_flags & IO_URING_F_NONBLOCK) {
    rq_flags |= REQ_NOWAIT;
    blk_flags = BLK_MQ_REQ_NOWAIT;
    }
    if (issue_flags & IO_URING_F_IOPOLL)
    rq_flags |= REQ_POLLED;
    req = nvme_alloc_user_request(q, &c, rq_flags, blk_flags);
    if (IS_ERR(req))
    return PTR_ERR(req);
    req.timeout = d.timeout_ms ? msecs_to_jiffies(d.timeout_ms) : 0;
    if (d.data_len) {
    ret = nvme_map_user_request(req, d.addr, d.data_len,
    nvme_to_user_ptr(d.metadata), d.metadata_len,
    map_iter, vec ? NVME_IOCTL_VEC : 0);
    if (ret)
    goto out_free_req;
    }
// to free bio on completion, as req->bio will be null at that time
    pdu.bio = req.bio;
    pdu.req = req;
    req.end_io_data = ioucmd;
    req.end_io = nvme_uring_cmd_end_io;
    blk_execute_rq_nowait(req, false);
    return -EIOCBQUEUED;
    out_free_req:
    blk_mq_free_request(req);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn is_ctrl_ioctl(cmd: c_uint) -> bool {
    static bool is_ctrl_ioctl(unsigned int cmd)
    {
    if (cmd == NVME_IOCTL_ADMIN_CMD || cmd == NVME_IOCTL_ADMIN64_CMD)
    return true;
    if (is_sed_ioctl(cmd))
    return true;
    return false;
    }
    static int nvme_ctrl_ioctl(struct nvme_ctrl *ctrl, unsigned int cmd,
    void __user *argp, bool open_for_write)
    {
    switch (cmd) {
    case NVME_IOCTL_ADMIN_CMD:
    return nvme_user_cmd(ctrl, core::ptr::null_mut(), argp, 0, open_for_write);
    case NVME_IOCTL_ADMIN64_CMD:
    return nvme_user_cmd64(ctrl, core::ptr::null_mut(), argp, 0, open_for_write);
    default:
    return sed_ioctl(ctrl.opal_dev, cmd, argp);
    }
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_user_io32 {
    pub opcode: __u8,
    pub flags: __u8,
    pub control: __u16,
    pub nblocks: __u16,
    pub rsvd: __u16,
    pub metadata: __u64,
    pub addr: __u64,
    pub slba: __u64,
    pub dsmgmt: __u32,
    pub reftag: __u32,
    pub apptag: __u16,
    pub appmask: __u16,
    pub __attribute__((__packed__)): },

    static int nvme_ns_ioctl(struct nvme_ns *ns, unsigned int cmd,
    void __user *argp, unsigned int flags, bool open_for_write)
    {
    switch (cmd) {
    case NVME_IOCTL_ID:
    pub ns->head->ns_id: return,
    case NVME_IOCTL_IO_CMD:
    pub open_for_write): return nvme_user_cmd(ns->ctrl, ns, argp, flags,,
//
// struct nvme_user_io can have different padding on some 32-bit ABIs.
// Just accept the compat version as all fields that are used are the
// same size and at the same offset.
//

    case NVME_IOCTL_SUBMIT_IO32:

    case NVME_IOCTL_SUBMIT_IO:
    pub open_for_write): return nvme_submit_io(ns, argp, flags,,
    case NVME_IOCTL_IO64_CMD_VEC:
    pub NVME_IOCTL_VEC: flags |=,
    case NVME_IOCTL_IO64_CMD:
    return nvme_user_cmd64(ns.ctrl, ns, argp, flags,
    default:
    pub -ENOTTY: return,
    }
    }
    int nvme_ioctl(struct block_device *bdev, blk_mode_t mode,
    unsigned int cmd, unsigned long arg)
    {
    pub bdev->bd_disk->private_data: *mut *mut nvme_ns ns =,
    pub BLK_OPEN_WRITE: bool open_for_write = mode &,
    pub )arg: *mut *mut void __user argp = (void __user,
    pub 0: unsigned int flags =,
    if (bdev_is_partition(bdev))
    pub NVME_IOCTL_PARTITION: flags |=,
    if (is_ctrl_ioctl(cmd))
    pub open_for_write): return nvme_ctrl_ioctl(ns->ctrl, cmd, argp,,
    pub open_for_write): return nvme_ns_ioctl(ns, cmd, argp, flags,,
    }
#[no_mangle]
pub unsafe extern "C" fn nvme_ns_chr_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    long nvme_ns_chr_ioctl(struct file *file, unsigned int cmd, unsigned long arg)
    {
    struct nvme_ns *ns =
    pub cdev): container_of(file_inode(file)->i_cdev, struct nvme_ns,,
    pub FMODE_WRITE: bool open_for_write = file->f_mode &,
    pub )arg: *mut *mut void __user argp = (void __user,
    if (is_ctrl_ioctl(cmd))
    pub open_for_write): return nvme_ctrl_ioctl(ns->ctrl, cmd, argp,,
    pub open_for_write): return nvme_ns_ioctl(ns, cmd, argp, 0,,
    }
#[no_mangle]
unsafe extern "C" fn nvme_uring_cmd_checks(issue_flags: c_uint) -> c_int {
    static int nvme_uring_cmd_checks(unsigned int issue_flags)
    {
// NVMe passthrough requires big SQE/CQE support
    if ((issue_flags & (IO_URING_F_SQE128|IO_URING_F_CQE32)) !=
    (IO_URING_F_SQE128|IO_URING_F_CQE32))
    pub -EOPNOTSUPP: return,
    pub 0: return,
    }
    static int nvme_ns_uring_cmd(struct nvme_ns *ns, struct io_uring_cmd *ioucmd,
    unsigned int issue_flags)
    {
    pub ns->ctrl: *mut *mut nvme_ctrl ctrl =,
    pub ret: c_int,
    pub nvme_uring_cmd_checks(issue_flags): ret =,
    if (ret)
    pub ret: return,
    switch (ioucmd.cmd_op) {
    case NVME_URING_CMD_IO:
    pub false): ret = nvme_uring_cmd_io(ctrl, ns, ioucmd, issue_flags,,
    case NVME_URING_CMD_IO_VEC:
    pub true): ret = nvme_uring_cmd_io(ctrl, ns, ioucmd, issue_flags,,
    default:
    pub -ENOTTY: ret =,
    }
    pub ret: return,
    }
#[no_mangle]
pub unsafe extern "C" fn nvme_ns_chr_uring_cmd(ioucmd: *mut io_uring_cmd, issue_flags: c_uint) -> c_int {
    int nvme_ns_chr_uring_cmd(struct io_uring_cmd *ioucmd, unsigned int issue_flags)
    {
    struct nvme_ns *ns = container_of(file_inode(ioucmd.file).i_cdev,
    pub cdev): nvme_ns,,
    pub issue_flags): return nvme_ns_uring_cmd(ns, ioucmd,,
    }
    int nvme_ns_chr_uring_cmd_iopoll(struct io_uring_cmd *ioucmd,
    struct io_comp_batch *iob,
    unsigned int poll_flags)
    {
    pub nvme_uring_cmd_pdu(ioucmd): *mut *mut nvme_uring_cmd_pdu pdu =,
    pub pdu->req: *mut *mut request req =,
    if (req && blk_rq_is_poll(req))
    pub poll_flags): return blk_rq_poll(req, iob,,
    pub 0: return,
    }

    static int nvme_ns_head_ctrl_ioctl(struct nvme_ns *ns, unsigned int cmd,
    void __user *argp, struct nvme_ns_head *head, int srcu_idx,
    bool open_for_write)
    __releases_shared(&head.srcu)
    {
    pub ns->ctrl: *mut *mut nvme_ctrl ctrl =,
    pub ret: c_int,
    pub srcu_idx): srcu_read_unlock(&head->srcu,,
    pub open_for_write): ret = nvme_ctrl_ioctl(ctrl, cmd, argp,,
    pub ret: return,
    }
    int nvme_ns_head_ioctl(struct block_device *bdev, blk_mode_t mode,
    unsigned int cmd, unsigned long arg)
    {
    pub bdev->bd_disk->private_data: *mut *mut nvme_ns_head head =,
    pub BLK_OPEN_WRITE: bool open_for_write = mode &,
    pub )arg: *mut *mut void __user argp = (void __user,
    pub ns: *mut nvme_ns,
    pub -EWOULDBLOCK: int srcu_idx, ret =,
    pub 0: unsigned int flags =,
    if (bdev_is_partition(bdev))
    pub NVME_IOCTL_PARTITION: flags |=,
    pub srcu_read_lock(&head->srcu): srcu_idx =,
    pub nvme_find_path(head): ns =,
    if (!ns)
    pub out_unlock: goto,
//
// Handle ioctls that apply to the controller instead of the namespace
// separately and drop the ns SRCU reference early.  This avoids a
// deadlock when deleting namespaces using the passthrough interface.
//
    if (is_ctrl_ioctl(cmd))
    return nvme_ns_head_ctrl_ioctl(ns, cmd, argp, head, srcu_idx,
    pub open_for_write): ret = nvme_ns_ioctl(ns, cmd, argp, flags,,
    out_unlock:
    pub srcu_idx): srcu_read_unlock(&head->srcu,,
    pub ret: return,
    }
    long nvme_ns_head_chr_ioctl(struct file *file, unsigned int cmd,
    unsigned long arg)
    {
    pub FMODE_WRITE: bool open_for_write = file->f_mode &,
    pub file_inode(file)->i_cdev: *mut *mut cdev cdev =,
    struct nvme_ns_head *head =
    pub cdev): container_of(cdev, struct nvme_ns_head,,
    pub )arg: *mut *mut void __user argp = (void __user,
    pub ns: *mut nvme_ns,
    pub -EWOULDBLOCK: int srcu_idx, ret =,
    pub srcu_read_lock(&head->srcu): srcu_idx =,
    pub nvme_find_path(head): ns =,
    if (!ns)
    pub out_unlock: goto,
    if (is_ctrl_ioctl(cmd))
    return nvme_ns_head_ctrl_ioctl(ns, cmd, argp, head, srcu_idx,
    pub open_for_write): ret = nvme_ns_ioctl(ns, cmd, argp, 0,,
    out_unlock:
    pub srcu_idx): srcu_read_unlock(&head->srcu,,
    pub ret: return,
    }
    int nvme_ns_head_chr_uring_cmd(struct io_uring_cmd *ioucmd,
    unsigned int issue_flags)
    {
    pub file_inode(ioucmd->file)->i_cdev: *mut *mut cdev cdev =,
    pub cdev): *mut *mut nvme_ns_head head = container_of(cdev, nvme_ns_head,,
    pub srcu_read_lock(&head->srcu): int srcu_idx =,
    pub nvme_find_path(head): *mut *mut nvme_ns ns =,
    pub -EINVAL: int ret =,
    if (ns)
    pub issue_flags): ret = nvme_ns_uring_cmd(ns, ioucmd,,
    pub srcu_idx): srcu_read_unlock(&head->srcu,,
    pub ret: return,
    }

#[no_mangle]
pub unsafe extern "C" fn nvme_dev_uring_cmd(ioucmd: *mut io_uring_cmd, issue_flags: c_uint) -> c_int {
    int nvme_dev_uring_cmd(struct io_uring_cmd *ioucmd, unsigned int issue_flags)
    {
    pub ioucmd->file->private_data: *mut *mut nvme_ctrl ctrl =,
    pub ret: c_int,
    pub nvme_uring_cmd_checks(issue_flags): ret =,
    if (ret)
    pub ret: return,
    switch (ioucmd.cmd_op) {
    case NVME_URING_CMD_ADMIN:
    pub false): ret = nvme_uring_cmd_io(ctrl, NULL, ioucmd, issue_flags,,
    case NVME_URING_CMD_ADMIN_VEC:
    pub true): ret = nvme_uring_cmd_io(ctrl, NULL, ioucmd, issue_flags,,
    default:
    pub -ENOTTY: ret =,
    }
    pub ret: return,
    }
    static int nvme_dev_user_cmd(struct nvme_ctrl *ctrl, void __user *argp,
    bool open_for_write)
    {
    pub ns: *mut nvme_ns,
    pub srcu_idx: int ret,,
    pub srcu_read_lock(&ctrl->srcu): srcu_idx =,
    if (list_empty(&ctrl.namespaces)) {
    pub -ENOTTY: ret =,
    pub out_unlock: goto,
    }
    pub list): ns = list_first_or_null_rcu(&ctrl->namespaces, struct nvme_ns,,
    if (ns != list_last_entry(&ctrl.namespaces, struct nvme_ns, list)) {
    dev_warn(ctrl.device,
    pub present!\n"): "NVME_IOCTL_IO_CMD not supported when multiple namespaces,
    pub -EINVAL: ret =,
    pub out_unlock: goto,
    }
    dev_warn(ctrl.device,
    pub device!\n"): "using deprecated NVME_IOCTL_IO_CMD ioctl on the char,
    if (!nvme_get_ns(ns)) {
    pub -ENXIO: ret =,
    pub out_unlock: goto,
    }
    pub srcu_idx): srcu_read_unlock(&ctrl->srcu,,
    pub open_for_write): ret = nvme_user_cmd(ctrl, ns, argp, 0,,
    pub ret: return,
    out_unlock:
    pub srcu_idx): srcu_read_unlock(&ctrl->srcu,,
    pub ret: return,
    }
    long nvme_dev_ioctl(struct file *file, unsigned int cmd,
    unsigned long arg)
    {
    pub FMODE_WRITE: bool open_for_write = file->f_mode &,
    pub file->private_data: *mut *mut nvme_ctrl ctrl =,
    pub )arg: *mut *mut void __user argp = (void __user,
    switch (cmd) {
    case NVME_IOCTL_ADMIN_CMD:
    pub open_for_write): return nvme_user_cmd(ctrl, NULL, argp, 0,,
    case NVME_IOCTL_ADMIN64_CMD:
    pub open_for_write): return nvme_user_cmd64(ctrl, NULL, argp, 0,,
    case NVME_IOCTL_IO_CMD:
    pub open_for_write): return nvme_dev_user_cmd(ctrl, argp,,
    case NVME_IOCTL_RESET:
    if (!capable(CAP_SYS_ADMIN))
    pub -EACCES: return,
    pub controller\n"): dev_warn(ctrl->device, "resetting,
    pub nvme_reset_ctrl_sync(ctrl): return,
    case NVME_IOCTL_SUBSYS_RESET:
    if (!capable(CAP_SYS_ADMIN))
    pub -EACCES: return,
    pub nvme_reset_subsystem(ctrl): return,
    case NVME_IOCTL_RESCAN:
    if (!capable(CAP_SYS_ADMIN))
    pub -EACCES: return,
    pub 0: return,
    default:
    pub -ENOTTY: return,
    }
    }
