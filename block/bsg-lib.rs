//! Automatically rewritten from C to Rust
//! Source: block/bsg-lib.c
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
// === KERNEL_MACRO_PRELUDE_START ===
macro_rules! EXPORT_SYMBOL { ($($tt:tt)*) => {}; }
macro_rules! EXPORT_SYMBOL_GPL { ($($tt:tt)*) => {}; }
macro_rules! MODULE_LICENSE { ($($tt:tt)*) => {}; }
macro_rules! MODULE_AUTHOR { ($($tt:tt)*) => {}; }
macro_rules! MODULE_DESCRIPTION { ($($tt:tt)*) => {}; }
macro_rules! MODULE_ALIAS { ($($tt:tt)*) => {}; }
macro_rules! module_init { ($($tt:tt)*) => {}; }
macro_rules! module_exit { ($($tt:tt)*) => {}; }
macro_rules! early_initcall { ($($tt:tt)*) => {}; }
macro_rules! core_initcall { ($($tt:tt)*) => {}; }
macro_rules! postcore_initcall { ($($tt:tt)*) => {}; }
macro_rules! arch_initcall { ($($tt:tt)*) => {}; }
macro_rules! subsys_initcall { ($($tt:tt)*) => {}; }
macro_rules! fs_initcall { ($($tt:tt)*) => {}; }
macro_rules! device_initcall { ($($tt:tt)*) => {}; }
macro_rules! late_initcall { ($($tt:tt)*) => {}; }
macro_rules! __setup { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_MUTEX { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_SPINLOCK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DECLARE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DEFINE { ($($tt:tt)*) => {}; }
macro_rules! ARRAY_SIZE { ($($tt:tt)*) => { 1 }; }
macro_rules! container_of { ($($tt:tt)*) => { core::ptr::null_mut() }; }
macro_rules! sizeof { ($($tt:tt)*) => { 0usize }; }
macro_rules! IS_ENABLED { ($($tt:tt)*) => { false }; }
macro_rules! DECLARE_WORK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_WAKE_Q { ($($tt:tt)*) => {}; }
macro_rules! LLIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! LIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! SET_UID { ($($tt:tt)*) => {}; }
macro_rules! SET_GID { ($($tt:tt)*) => {}; }
macro_rules! list_for_each_entry { ($($tt:tt)*) => { if false }; }
macro_rules! list_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! llist_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! pr_info_once { ($($tt:tt)*) => {}; }
macro_rules! pr_info { ($($tt:tt)*) => {}; }
macro_rules! pr_warn { ($($tt:tt)*) => {}; }
macro_rules! pr_err { ($($tt:tt)*) => {}; }
macro_rules! pr_debug { ($($tt:tt)*) => {}; }
macro_rules! early_param { ($($tt:tt)*) => {}; }
macro_rules! BUILD_BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! WARN_ON { ($($tt:tt)*) => { false }; }
macro_rules! WARN_ON_ONCE { ($($tt:tt)*) => { false }; }
macro_rules! BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! BUG { () => {}; }
macro_rules! IS_ERR { ($($tt:tt)*) => { false }; }
macro_rules! PTR_ERR { ($($tt:tt)*) => { 0 }; }
macro_rules! ERR_PTR { ($($tt:tt)*) => { core::ptr::null_mut() }; }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seq_file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cred { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inode { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct notifier_block { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw_notifier_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_header { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_root { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_set { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_dir_entry { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kern_ipc_perm { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_params { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_queue { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msgseg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_sender { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_receiver { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sembuf { pub sem_num: u16, pub sem_op: i16, pub sem_flg: i16 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem_array { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shmid_kernel { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shm_file_data { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wake_q_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct work_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct llist_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_head { pub _opaque: [u8; 0] }

pub type pid_type = c_int;
pub type cpu_pm_event = c_int;
pub type spinlock_t = u32;
pub type raw_spinlock_t = u32;
pub type kernel_cap_t = u64;
pub type cap_user_header_t = *mut c_void;
pub type cap_user_data_t = *mut c_void;
pub type async_cookie_t = u64;
pub type atomic_long_t = core::sync::atomic::AtomicI64;
pub type key_t = i32;
pub type kuid_t = u32;
pub type kgid_t = u32;
pub type int = c_int;
pub type uint = c_uint;
pub type ulong = c_ulong;
pub type long = c_long;
pub type void = c_void;

// Standard Linux Error Codes
pub const EPERM: c_int = 1;
pub const ENOENT: c_int = 2;
pub const ESRCH: c_int = 3;
pub const EINTR: c_int = 4;
pub const EIO: c_int = 5;
pub const ENXIO: c_int = 6;
pub const E2BIG: c_int = 7;
pub const ENOEXEC: c_int = 8;
pub const EBADF: c_int = 9;
pub const ECHILD: c_int = 10;
pub const EAGAIN: c_int = 11;
pub const ENOMEM: c_int = 12;
pub const EACCES: c_int = 13;
pub const EFAULT: c_int = 14;
pub const EBUSY: c_int = 16;
pub const EEXIST: c_int = 17;
pub const EXDEV: c_int = 18;
pub const ENODEV: c_int = 19;
pub const ENOTDIR: c_int = 20;
pub const EISDIR: c_int = 21;
pub const EINVAL: c_int = 22;
pub const ENFILE: c_int = 23;
pub const EMFILE: c_int = 24;
pub const ENOSPC: c_int = 28;
pub const EROFS: c_int = 30;
pub const EIDRM: c_int = 43;
pub const EOPNOTSUPP: c_int = 95;
pub const ENOTSUPP: c_int = 524;

// Standard Memory Constants
pub const PAGE_SHIFT: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
pub const GFP_KERNEL: c_uint = 0xcc0;
pub const GFP_ATOMIC: c_uint = 0x80000;
pub const GFP_NOWAIT: c_uint = 0;

// Standard Core Primitives
extern "C" {
    pub static current: *mut task_struct;
    pub fn printk(fmt: *const c_char, ...) -> c_int;
    pub fn rcu_read_lock();
    pub fn rcu_read_unlock();
    pub fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    pub fn kfree(ptr: *mut c_void);
}
// === KERNEL_MACRO_PRELUDE_END ===


// SPDX-License-Identifier: GPL-2.0-or-later
//
// BSG helper library
//
// Copyright (C) 2008   James Smart, Emulex Corporation
// Copyright (C) 2011   Red Hat, Inc.  All rights reserved.
// Copyright (C) 2011   Mike Christie
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bsg_set {
    pub tag_set: blk_mq_tag_set,
    pub bd: *mut bsg_device,
    pub job_fn: *mut bsg_job_fn,
    pub timeout_fn: *mut bsg_timeout_fn,
}

#[no_mangle]
pub unsafe extern "C" fn bsg_transport_sg_io_fn(q: *mut request_queue, hdr: *mut sg_io_v4, open_for_write: bool, timeout: c_uint) -> c_int {
pub static mut job: *mut c_void = core::ptr::null_mut();
pub static mut rq: *mut c_void = core::ptr::null_mut();
pub static mut bio: *mut c_void = core::ptr::null_mut();
pub static mut reply: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    if (hdr.protocol != BSG_PROTOCOL_SCSI  ||
    hdr.subprotocol != BSG_SUB_PROTOCOL_SCSI_TRANSPORT) {
    return -EINVAL;
    }
    if (!capable(CAP_SYS_RAWIO)) {
    return -EPERM;
    }
    rq = blk_mq_alloc_request(q, hdr.dout_xfer_len ?
    REQ_OP_DRV_OUT : REQ_OP_DRV_IN, 0);
    if (IS_ERR(rq)) {
    return PTR_ERR(rq);
    }
    rq.timeout = timeout;
    job = blk_mq_rq_to_pdu(rq);
    reply = job.reply;
    memset(job, 0, sizeof!(*job));
    job.reply = reply;
    job.reply_len = SCSI_SENSE_BUFFERSIZE;
    job.dd_data = job + 1;
    job.request_len = hdr.request_len;
    job.request = memdup_user(uptr64(hdr.request), hdr.request_len);
    if (IS_ERR(job.request)) {
    ret = PTR_ERR(job.request);
// goto;
    }
    if (hdr.dout_xfer_len && hdr.din_xfer_len) {
    job.bidi_rq = blk_mq_alloc_request(rq.q, REQ_OP_DRV_IN, 0);
    if (IS_ERR(job.bidi_rq)) {
    ret = PTR_ERR(job.bidi_rq);
// goto;
    }
    ret = blk_rq_map_user(rq.q, job.bidi_rq, core::ptr::null_mut(),
    uptr64(hdr.din_xferp), hdr.din_xfer_len,
    GFP_KERNEL);
    if (ret) {
// goto;
    }
    job.bidi_bio = job.bidi_rq.bio;
    } else {
    job.bidi_rq = core::ptr::null_mut();
    job.bidi_bio = core::ptr::null_mut();
    }
    ret = 0;
    if (hdr.dout_xfer_len) {
    ret = blk_rq_map_user(rq.q, rq, core::ptr::null_mut(), uptr64(hdr.dout_xferp),
    hdr.dout_xfer_len, GFP_KERNEL);
    } else if (hdr.din_xfer_len) {
    ret = blk_rq_map_user(rq.q, rq, core::ptr::null_mut(), uptr64(hdr.din_xferp),
    hdr.din_xfer_len, GFP_KERNEL);
    }
    if (ret) {
// goto;
    }
    bio = rq.bio;
    blk_execute_rq(rq, !(hdr.flags & BSG_FLAG_Q_AT_TAIL));
//
// The assignments below don't make much sense, but are kept for
// bug by bug backwards compatibility:
//
    hdr.device_status = job.result & 0xff;
    hdr.transport_status = host_byte(job.result);
    hdr.driver_status = 0;
    hdr.info = 0;
    if (hdr.device_status || hdr.transport_status || hdr.driver_status) {
    hdr.info |= SG_INFO_CHECK;
    }
    hdr.response_len = 0;
    if (job.result < 0) {
// we're only returning the result field in the reply
    job.reply_len = sizeof!(u32);
    ret = job.result;
    }
    if (job.reply_len && hdr.response) {
pub static mut len: c_int = 0;
    if (copy_to_user(uptr64(hdr.response), job.reply, len)) {
    ret = -EFAULT;
    }
    else {
    hdr.response_len = len;
    }
    }
// we assume all request payload was transferred, residual == 0
    hdr.dout_resid = 0;
    if (job.bidi_rq) {
pub static mut rsp_len: c_uint = 0;
    if (WARN_ON!(job.reply_payload_rcv_len > rsp_len)) {
    hdr.din_resid = 0;
    }
    else {
    hdr.din_resid = rsp_len - job.reply_payload_rcv_len;
    }
    } else {
    hdr.din_resid = 0;
    }
    blk_rq_unmap_user(bio);
// label;
    if (job.bidi_rq) {
    blk_rq_unmap_user(job.bidi_bio);
    }
// label;
    if (job.bidi_rq) {
    blk_mq_free_request(job.bidi_rq);
    }
// label;
    kfree(job.request);
// label;
    blk_mq_free_request(rq);
    return ret;
    }
//
// bsg_teardown_job - routine to teardown a bsg job
// @kref: kref inside bsg_job that is to be torn down
//
#[no_mangle]
unsafe extern "C" fn bsg_teardown_job(kref: *mut kref) {
    let mut job = container_of!(kref, bsg_job, kref);
    let mut rq = blk_mq_rq_from_pdu(job);
    put_device(job.dev);	/* release reference for the request */
    kfree(job.request_payload.sg_list);
    kfree(job.reply_payload.sg_list);
    blk_mq_end_request(rq, BLK_STS_OK);
    }
#[no_mangle]
pub unsafe extern "C" fn bsg_job_put(job: *mut bsg_job) {
    kref_put(&job.kref, bsg_teardown_job);
    }
    EXPORT_SYMBOL_GPL(bsg_job_put);
#[no_mangle]
pub unsafe extern "C" fn bsg_job_get(job: *mut bsg_job) -> c_int {
    return kref_get_unless_zero(&job.kref);
    }
    EXPORT_SYMBOL_GPL(bsg_job_get);
//
// bsg_job_done - completion routine for bsg requests
// @job: bsg_job that is complete
// @result: job reply result
// @reply_payload_rcv_len: length of payload recvd
//
// The LLD should call this when the bsg job has completed.
//
#[no_mangle]
pub unsafe extern "C" fn bsg_job_done(job: *mut bsg_job, result: c_int, reply_payload_rcv_len: c_uint) {
    let mut rq = blk_mq_rq_from_pdu(job);
    job.result = result;
    job.reply_payload_rcv_len = reply_payload_rcv_len;
    if (likely(!blk_should_fake_timeout(rq.q))) {
    blk_mq_complete_request(rq);
    }
    }
    EXPORT_SYMBOL_GPL(bsg_job_done);
//
// bsg_complete - softirq done routine for destroying the bsg requests
// @rq: BSG request that holds the job to be destroyed
//
#[no_mangle]
unsafe extern "C" fn bsg_complete(rq: *mut request) {
    let mut job = blk_mq_rq_to_pdu(rq);
    bsg_job_put(job);
    }
#[no_mangle]
unsafe extern "C" fn bsg_map_buffer(buf: *mut bsg_buffer, req: *mut request) -> c_int {
pub static mut sz: usize = 0;
    BUG_ON!(!req.nr_phys_segments);
    buf.sg_list = kmalloc(sz, GFP_KERNEL);
    if (!buf.sg_list) {
    return -ENOMEM;
    }
    sg_init_table(buf.sg_list, req.nr_phys_segments);
    buf.sg_cnt = blk_rq_map_sg(req, buf.sg_list);
    buf.payload_len = blk_rq_bytes(req);
    return 0;
    }
//
// bsg_prepare_job - create the bsg_job structure for the bsg request
// @dev: device that is being sent the bsg request
// @req: BSG request that needs a job structure
//
#[no_mangle]
unsafe extern "C" fn bsg_prepare_job(dev: *mut device, req: *mut request) -> bool {
    let mut job = blk_mq_rq_to_pdu(req);
    let mut ret = 0;
    job.timeout = req.timeout;
    if (req.bio) {
    ret = bsg_map_buffer(&job.request_payload, req);
    if (ret) {
// goto;
    }
    }
    if (job.bidi_rq) {
    ret = bsg_map_buffer(&job.reply_payload, job.bidi_rq);
    if (ret) {
// goto;
    }
    }
    job.dev = dev;
// take a reference for the request
    get_device(job.dev);
    kref_init(&job.kref);
    return true;
// label;
    kfree(job.request_payload.sg_list);
// label;
    job.result = -ENOMEM;
    return false;
    }
//
// bsg_queue_rq - generic handler for bsg requests
// @hctx: hardware queue
// @bd: queue data
//
// On error the create_bsg_job function should return a -Exyz error value
// that will be set to ->result.
//
// Drivers/subsys should pass this to the queue init function.
//
    static blk_status_t bsg_queue_rq(blk_mq_hw_ctx *hctx,
    const struct blk_mq_queue_data *bd)
    {
    let mut q = hctx.queue;
    let mut dev = q.queuedata;
    let mut req = bd.rq;
    let mut bset = container_of!(q.tag_set, bsg_set, tag_set);
pub static mut sts: blk_status_t = 0;
    let mut ret = 0;
    blk_mq_start_request(req);
    if (!get_device(dev)) {
    return BLK_STS_IOERR;
    }
    if (!bsg_prepare_job(dev, req)) {
// goto;
    }
    ret = bset.job_fn(blk_mq_rq_to_pdu(req));
    if (!ret) {
    sts = BLK_STS_OK;
    }
// label;
    put_device(dev);
    return sts;
    }
// called right after the request is allocated for the request_queue
#[no_mangle]
pub unsafe extern "C" fn bsg_init_rq(set: *mut blk_mq_tag_set, req: *mut request, hctx_idx: c_uint, numa_node: c_int) -> c_int {
    let mut job = blk_mq_rq_to_pdu(req);
    job.reply = kzalloc(SCSI_SENSE_BUFFERSIZE, GFP_KERNEL);
    if (!job.reply) {
    return -ENOMEM;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn bsg_exit_rq(set: *mut blk_mq_tag_set, req: *mut request, hctx_idx: c_uint) {
    let mut job = blk_mq_rq_to_pdu(req);
    kfree(job.reply);
    }
#[no_mangle]
pub unsafe extern "C" fn bsg_remove_queue(q: *mut request_queue) {
    if (q) {
    let mut bset = container_of!(q.tag_set, bsg_set, tag_set);
    bsg_unregister_queue(bset.bd);
    blk_mq_destroy_queue(q);
    blk_put_queue(q);
    blk_mq_free_tag_set(&bset.tag_set);
    kfree(bset);
    }
    }
    EXPORT_SYMBOL_GPL(bsg_remove_queue);
#[no_mangle]
unsafe extern "C" fn bsg_timeout(rq: *mut request) -> enum blk_eh_timer_return {
    let mut bset = container_of!(rq.q.tag_set, bsg_set, tag_set);
    if (!bset.timeout_fn) {
    return BLK_EH_DONE;
    }
    return bset.timeout_fn(rq);
    }
pub static mut blk_mq_ops: usize = 0;
//
// bsg_setup_queue - Create and add the bsg hooks so we can receive requests
// @dev: device to attach bsg device to
// @name: device to give bsg device
// @lim: queue limits for the bsg queue
// @job_fn: bsg job handler
// @timeout: timeout handler function pointer
// @dd_job_size: size of LLD data needed for each job
//
#[no_mangle]
pub unsafe extern "C" fn bsg_setup_queue(dev: *mut device, name: *mut c_char, lim: *mut queue_limits, job_fn: *mut bsg_job_fn, timeout: *mut bsg_timeout_fn, dd_job_size: c_int) -> *mut c_void {
pub static mut bset: *mut c_void = core::ptr::null_mut();
pub static mut set: *mut c_void = core::ptr::null_mut();
pub static mut q: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    bset = kzalloc_obj(*bset);
    if (!bset) {
    return ERR_PTR(-ENOMEM);
    }
    bset.job_fn = job_fn;
    bset.timeout_fn = timeout;
    set = &bset.tag_set;
    set.ops = &bsg_mq_ops;
    set.nr_hw_queues = 1;
    set.queue_depth = 128;
    set.numa_node = NUMA_NO_NODE;
    set.cmd_size = sizeof!(bsg_job) + dd_job_size;
    set.flags = BLK_MQ_F_BLOCKING;
    if (blk_mq_alloc_tag_set(set)) {
// goto;
    }
    q = blk_mq_alloc_queue(set, lim, dev);
    if (IS_ERR(q)) {
    ret = PTR_ERR(q);
// goto;
    }
    blk_queue_rq_timeout(q, BLK_DEFAULT_SG_TIMEOUT);
    bset.bd = bsg_register_queue(q, dev, name, bsg_transport_sg_io_fn, core::ptr::null_mut());
    if (IS_ERR(bset.bd)) {
    ret = PTR_ERR(bset.bd);
// goto;
    }
    return q;
// label;
    blk_mq_destroy_queue(q);
    blk_put_queue(q);
// label;
    blk_mq_free_tag_set(set);
// label;
    kfree(bset);
    return ERR_PTR(ret);
    }
    EXPORT_SYMBOL_GPL(bsg_setup_queue);