//! Automatically rewritten from C to Rust
//! Source: drivers/hid/uhid.c
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
// User-space I/O driver support for HID subsystem
// Copyright (c) 2012 David Herrmann
//

pub const UHID_BUFSIZE: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uhid_device {
    pub devlock: mutex,
// This flag tracks whether the HID device is usable for commands from
// userspace. The flag is already set before hid_add_device(), which
// runs in workqueue context, to allow hid_add_device() to communicate
// with userspace.
// However, if hid_add_device() fails, the flag is cleared without
// holding devlock.
// We guarantee that if @running changes from true to false while you're
// holding @devlock, it's still fine to access @hid.
//
    pub running: bool,
    pub rd_data: *mut __u8,
    pub rd_size: c_uint,
// When this is NULL, userspace may use UHID_CREATE/UHID_CREATE2.
    pub hid: *mut hid_device,
    pub input_buf: uhid_event,
    pub waitq: wait_queue_head_t,
    pub qlock: spinlock_t,
    pub head: __u8,
    pub tail: __u8,
    pub outq: [*mut uhid_event; UHID_BUFSIZE],
// blocking GET_REPORT support; state changes protected by qlock
    pub report_lock: mutex,
    pub report_wait: wait_queue_head_t,
    pub report_running: bool,
    pub report_id: u32,
    pub report_type: u32,
    pub report_buf: uhid_event,
    pub worker: work_struct,
}

    static struct miscdevice uhid_misc;
#[no_mangle]
unsafe extern "C" fn uhid_device_add_worker(work: *mut work_struct) {
    static void uhid_device_add_worker(struct work_struct *work)
    {
    struct uhid_device *uhid = container_of(work, struct uhid_device, worker);
    int ret;
    ret = hid_add_device(uhid.hid);
    if (ret) {
    hid_err(uhid.hid, "Cannot register HID device: error %d\n", ret);
// We used to call hid_destroy_device() here, but that's really
// messy to get right because we have to coordinate with
// concurrent writes from userspace that might be in the middle
// of using uhid->hid.
// Just leave uhid->hid as-is for now, and clean it up when
// userspace tries to close or reinitialize the uhid instance.
//
// However, we do have to clear the ->running flag and do a
// wakeup to make sure userspace knows that the device is gone.
//
    WRITE_ONCE(uhid.running, false);
    wake_up_interruptible(&uhid.report_wait);
    }
    }
#[no_mangle]
unsafe extern "C" fn uhid_queue(uhid: *mut uhid_device, ev: *mut uhid_event) {
    static void uhid_queue(struct uhid_device *uhid, struct uhid_event *ev)
    {
    __u8 newhead;
    newhead = (uhid.head + 1) % UHID_BUFSIZE;
    if (newhead != uhid.tail) {
    uhid.outq[uhid.head] = ev;
    uhid.head = newhead;
    wake_up_interruptible(&uhid.waitq);
    } else {
    hid_warn(uhid.hid, "Output queue is full\n");
    kfree(ev);
    }
    }
#[no_mangle]
unsafe extern "C" fn uhid_queue_event(uhid: *mut uhid_device, event: __u32) -> c_int {
    static int uhid_queue_event(struct uhid_device *uhid, __u32 event)
    {
    unsigned long flags;
    struct uhid_event *ev;
    ev = kzalloc_obj(*ev);
    if (!ev)
    return -ENOMEM;
    ev.type = event;
    spin_lock_irqsave(&uhid.qlock, flags);
    uhid_queue(uhid, ev);
    spin_unlock_irqrestore(&uhid.qlock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn uhid_hid_start(hid: *mut hid_device) -> c_int {
    static int uhid_hid_start(struct hid_device *hid)
    {
    struct uhid_device *uhid = hid.driver_data;
    struct uhid_event *ev;
    unsigned long flags;
    ev = kzalloc_obj(*ev);
    if (!ev)
    return -ENOMEM;
    ev.type = UHID_START;
    if (hid.report_enum[HID_FEATURE_REPORT].numbered)
    ev.u.start.dev_flags |= UHID_DEV_NUMBERED_FEATURE_REPORTS;
    if (hid.report_enum[HID_OUTPUT_REPORT].numbered)
    ev.u.start.dev_flags |= UHID_DEV_NUMBERED_OUTPUT_REPORTS;
    if (hid.report_enum[HID_INPUT_REPORT].numbered)
    ev.u.start.dev_flags |= UHID_DEV_NUMBERED_INPUT_REPORTS;
    spin_lock_irqsave(&uhid.qlock, flags);
    uhid_queue(uhid, ev);
    spin_unlock_irqrestore(&uhid.qlock, flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn uhid_hid_stop(hid: *mut hid_device) {
    static void uhid_hid_stop(struct hid_device *hid)
    {
    struct uhid_device *uhid = hid.driver_data;
    hid.claimed = 0;
    uhid_queue_event(uhid, UHID_STOP);
    }
#[no_mangle]
unsafe extern "C" fn uhid_hid_open(hid: *mut hid_device) -> c_int {
    static int uhid_hid_open(struct hid_device *hid)
    {
    struct uhid_device *uhid = hid.driver_data;
    return uhid_queue_event(uhid, UHID_OPEN);
    }
#[no_mangle]
unsafe extern "C" fn uhid_hid_close(hid: *mut hid_device) {
    static void uhid_hid_close(struct hid_device *hid)
    {
    struct uhid_device *uhid = hid.driver_data;
    uhid_queue_event(uhid, UHID_CLOSE);
    }
#[no_mangle]
unsafe extern "C" fn uhid_hid_parse(hid: *mut hid_device) -> c_int {
    static int uhid_hid_parse(struct hid_device *hid)
    {
    struct uhid_device *uhid = hid.driver_data;
    return hid_parse_report(hid, uhid.rd_data, uhid.rd_size);
    }
// must be called with report_lock held
    static int __uhid_report_queue_and_wait(struct uhid_device *uhid,
    struct uhid_event *ev,
    __u32 *report_id)
    {
    unsigned long flags;
    int ret;
    spin_lock_irqsave(&uhid.qlock, flags);
// report_id = ++uhid->report_id;
    uhid.report_type = ev.type + 1;
    uhid.report_running = true;
    uhid_queue(uhid, ev);
    spin_unlock_irqrestore(&uhid.qlock, flags);
    ret = wait_event_interruptible_timeout(uhid.report_wait,
    !uhid.report_running || !READ_ONCE(uhid.running),
    5 * HZ);
    if (!ret || !READ_ONCE(uhid.running) || uhid.report_running)
    ret = -EIO;
#[no_mangle]
pub unsafe extern "C" fn if(0: ret <) -> else {
    else if (ret < 0)
    ret = -ERESTARTSYS;
    else
    ret = 0;
    uhid.report_running = false;
    return ret;
    }
    static void uhid_report_wake_up(struct uhid_device *uhid, u32 id,
    const struct uhid_event *ev)
    {
    unsigned long flags;
    spin_lock_irqsave(&uhid.qlock, flags);
// id for old report; drop it silently
    if (uhid.report_type != ev.type || uhid.report_id != id)
    goto unlock;
    if (!uhid.report_running)
    goto unlock;
    memcpy(&uhid.report_buf, ev, sizeof(*ev));
    uhid.report_running = false;
    wake_up_interruptible(&uhid.report_wait);
    unlock:
    spin_unlock_irqrestore(&uhid.qlock, flags);
    }
    static int uhid_hid_get_report(struct hid_device *hid, unsigned char rnum,
    u8 *buf, size_t count, u8 rtype)
    {
    struct uhid_device *uhid = hid.driver_data;
    struct uhid_get_report_reply_req *req;
    struct uhid_event *ev;
    int ret;
    if (!READ_ONCE(uhid.running))
    return -EIO;
    ev = kzalloc_obj(*ev);
    if (!ev)
    return -ENOMEM;
    ev.type = UHID_GET_REPORT;
    ev.u.get_report.rnum = rnum;
    ev.u.get_report.rtype = rtype;
    ret = mutex_lock_interruptible(&uhid.report_lock);
    if (ret) {
    kfree(ev);
    return ret;
    }
// this _always_ takes ownership of @ev
    ret = __uhid_report_queue_and_wait(uhid, ev, &ev.u.get_report.id);
    if (ret)
    goto unlock;
    req = &uhid.report_buf.u.get_report_reply;
    if (req.err) {
    ret = -EIO;
    } else {
    ret = min3(count, (size_t)req.size, (size_t)UHID_DATA_MAX);
    memcpy(buf, req.data, ret);
    }
    unlock:
    mutex_unlock(&uhid.report_lock);
    return ret;
    }
    static int uhid_hid_set_report(struct hid_device *hid, unsigned char rnum,
    const u8 *buf, size_t count, u8 rtype)
    {
    struct uhid_device *uhid = hid.driver_data;
    struct uhid_event *ev;
    int ret;
    if (!READ_ONCE(uhid.running) || count > UHID_DATA_MAX)
    return -EIO;
    ev = kzalloc_obj(*ev);
    if (!ev)
    return -ENOMEM;
    ev.type = UHID_SET_REPORT;
    ev.u.set_report.rnum = rnum;
    ev.u.set_report.rtype = rtype;
    ev.u.set_report.size = count;
    memcpy(ev.u.set_report.data, buf, count);
    ret = mutex_lock_interruptible(&uhid.report_lock);
    if (ret) {
    kfree(ev);
    return ret;
    }
// this _always_ takes ownership of @ev
    ret = __uhid_report_queue_and_wait(uhid, ev, &ev.u.set_report.id);
    if (ret)
    goto unlock;
    if (uhid.report_buf.u.set_report_reply.err)
    ret = -EIO;
    else
    ret = count;
    unlock:
    mutex_unlock(&uhid.report_lock);
    return ret;
    }
    static int uhid_hid_raw_request(struct hid_device *hid, unsigned char reportnum,
    __u8 *buf, size_t len, unsigned char rtype,
    int reqtype)
    {
    u8 u_rtype;
    switch (rtype) {
    case HID_FEATURE_REPORT:
    u_rtype = UHID_FEATURE_REPORT;
    break;
    case HID_OUTPUT_REPORT:
    u_rtype = UHID_OUTPUT_REPORT;
    break;
    case HID_INPUT_REPORT:
    u_rtype = UHID_INPUT_REPORT;
    break;
    default:
    return -EINVAL;
    }
    switch (reqtype) {
    case HID_REQ_GET_REPORT:
    return uhid_hid_get_report(hid, reportnum, buf, len, u_rtype);
    case HID_REQ_SET_REPORT:
    return uhid_hid_set_report(hid, reportnum, buf, len, u_rtype);
    default:
    return -EIO;
    }
    }
    static int uhid_hid_output_raw(struct hid_device *hid, __u8 *buf, size_t count,
    unsigned char report_type)
    {
    struct uhid_device *uhid = hid.driver_data;
    __u8 rtype;
    unsigned long flags;
    struct uhid_event *ev;
    switch (report_type) {
    case HID_FEATURE_REPORT:
    rtype = UHID_FEATURE_REPORT;
    break;
    case HID_OUTPUT_REPORT:
    rtype = UHID_OUTPUT_REPORT;
    break;
    default:
    return -EINVAL;
    }
    if (count < 1 || count > UHID_DATA_MAX)
    return -EINVAL;
    ev = kzalloc_obj(*ev);
    if (!ev)
    return -ENOMEM;
    ev.type = UHID_OUTPUT;
    ev.u.output.size = count;
    ev.u.output.rtype = rtype;
    memcpy(ev.u.output.data, buf, count);
    spin_lock_irqsave(&uhid.qlock, flags);
    uhid_queue(uhid, ev);
    spin_unlock_irqrestore(&uhid.qlock, flags);
    return count;
    }
    static int uhid_hid_output_report(struct hid_device *hid, __u8 *buf,
    size_t count)
    {
    return uhid_hid_output_raw(hid, buf, count, HID_OUTPUT_REPORT);
    }
    static const struct hid_ll_driver uhid_hid_driver = {
    .start = uhid_hid_start,
    .stop = uhid_hid_stop,
    .open = uhid_hid_open,
    .close = uhid_hid_close,
    .parse = uhid_hid_parse,
    .raw_request = uhid_hid_raw_request,
    .output_report = uhid_hid_output_report,
    .max_buffer_size = UHID_DATA_MAX,
    };

// Apparently we haven't stepped on these rakes enough times yet.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uhid_create_req_compat {
    pub name: [__u8; 128],
    pub phys: [__u8; 64],
    pub uniq: [__u8; 64],
    pub rd_data: compat_uptr_t,
    pub rd_size: __u16,
    pub bus: __u16,
    pub vendor: __u32,
    pub product: __u32,
    pub version: __u32,
    pub country: __u32,
    pub __attribute__((__packed__)): },
    static int uhid_event_from_user(const char __user *buffer, size_t len,
    struct uhid_event *event)
    {
    if (in_compat_syscall()) {
    pub type: u32,
    if (get_user(type, buffer))
    pub -EFAULT: return,
    if (type == UHID_CREATE) {
//
// This is our messed up request with compat pointer.
// It is largish (more than 256 bytes) so we better
// allocate it from the heap.
//
    pub compat: *mut uhid_create_req_compat,
    pub kzalloc_obj(*compat): *mut compat =,
    if (!compat)
    pub -ENOMEM: return,
    pub sizeof(type): buffer +=,
    pub sizeof(type): len -=,
    if (copy_from_user(compat, buffer,
    min(len, sizeof(*compat)))) {
    pub -EFAULT: return,
    }
// Shuffle the data over to proper structure
    pub type: event->type =,
    memcpy(event.u.create.name, compat.name,
    memcpy(event.u.create.phys, compat.phys,
    memcpy(event.u.create.uniq, compat.uniq,
    pub compat_ptr(compat->rd_data): event->u.create.rd_data =,
    pub compat->rd_size: event->u.create.rd_size =,
    pub compat->bus: event->u.create.bus =,
    pub compat->vendor: event->u.create.vendor =,
    pub compat->product: event->u.create.product =,
    pub compat->version: event->u.create.version =,
    pub compat->country: event->u.create.country =,
    pub 0: return,
    }
// All others can be copied directly
    }
    if (copy_from_user(event, buffer, min(len, sizeof(*event))))
    pub -EFAULT: return,
    pub 0: return,
    }

    static int uhid_event_from_user(const char __user *buffer, size_t len,
    struct uhid_event *event)
    {
    if (copy_from_user(event, buffer, min(len, sizeof(*event))))
    pub -EFAULT: return,
    pub 0: return,
    }

    static int uhid_dev_create2(struct uhid_device *uhid,
    const struct uhid_event *ev)
    {
    pub hid: *mut hid_device,
    pub rd_size: usize,
    pub rd_data: *mut c_void,
    pub ret: c_int,
    if (uhid.hid)
    pub -EALREADY: return,
    pub ev->u.create2.rd_size: rd_size =,
    if (rd_size <= 0 || rd_size > HID_MAX_DESCRIPTOR_SIZE)
    pub -EINVAL: return,
    pub GFP_KERNEL): rd_data = kmemdup(ev->u.create2.rd_data, rd_size,,
    if (!rd_data)
    pub -ENOMEM: return,
    pub rd_size: uhid->rd_size =,
    pub rd_data: uhid->rd_data =,
    pub hid_allocate_device(): hid =,
    if (IS_ERR(hid)) {
    pub PTR_ERR(hid): ret =,
    pub err_free: goto,
    }
    pub sizeof(ev->u.create2.name)): BUILD_BUG_ON(sizeof(hid->name) !=,
    pub sizeof(hid->name)): strscpy(hid->name, ev->u.create2.name,,
    pub sizeof(ev->u.create2.phys)): BUILD_BUG_ON(sizeof(hid->phys) !=,
    pub sizeof(hid->phys)): strscpy(hid->phys, ev->u.create2.phys,,
    pub sizeof(ev->u.create2.uniq)): BUILD_BUG_ON(sizeof(hid->uniq) !=,
    pub sizeof(hid->uniq)): strscpy(hid->uniq, ev->u.create2.uniq,,
    pub &uhid_hid_driver: hid->ll_driver =,
    pub ev->u.create2.bus: hid->bus =,
    pub ev->u.create2.vendor: hid->vendor =,
    pub ev->u.create2.product: hid->product =,
    pub ev->u.create2.version: hid->version =,
    pub ev->u.create2.country: hid->country =,
    pub uhid: hid->driver_data =,
    pub uhid_misc.this_device: hid->dev.parent =,
    pub hid: uhid->hid =,
    pub true: uhid->running =,
// Adding of a HID device is done through a worker, to allow HID drivers
// which use feature requests during .probe to work, without they would
// be blocked on devlock, which is held by uhid_char_write.
//
    pub 0: return,
    err_free:
    pub NULL: uhid->rd_data =,
    pub 0: uhid->rd_size =,
    pub ret: return,
    }
    static int uhid_dev_create(struct uhid_device *uhid,
    struct uhid_event *ev)
    {
    pub orig: uhid_create_req,
    pub ev->u.create: orig =,
    if (orig.rd_size <= 0 || orig.rd_size > HID_MAX_DESCRIPTOR_SIZE)
    pub -EINVAL: return,
    if (copy_from_user(&ev.u.create2.rd_data, orig.rd_data, orig.rd_size))
    pub -EFAULT: return,
    pub sizeof(orig.name)): memcpy(ev->u.create2.name, orig.name,,
    pub sizeof(orig.phys)): memcpy(ev->u.create2.phys, orig.phys,,
    pub sizeof(orig.uniq)): memcpy(ev->u.create2.uniq, orig.uniq,,
    pub orig.rd_size: ev->u.create2.rd_size =,
    pub orig.bus: ev->u.create2.bus =,
    pub orig.vendor: ev->u.create2.vendor =,
    pub orig.product: ev->u.create2.product =,
    pub orig.version: ev->u.create2.version =,
    pub orig.country: ev->u.create2.country =,
    pub ev): return uhid_dev_create2(uhid,,
    }
#[no_mangle]
unsafe extern "C" fn uhid_dev_destroy(uhid: *mut uhid_device) -> c_int {
    static int uhid_dev_destroy(struct uhid_device *uhid)
    {
    if (!uhid.hid)
    pub -EINVAL: return,
    pub false): WRITE_ONCE(uhid->running,,
    pub NULL: uhid->hid =,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn uhid_dev_input(uhid: *mut uhid_device, ev: *mut uhid_event) -> c_int {
    static int uhid_dev_input(struct uhid_device *uhid, struct uhid_event *ev)
    {
    if (!READ_ONCE(uhid.running))
    pub -EINVAL: return,
    hid_safe_input_report(uhid.hid, HID_INPUT_REPORT, ev.u.input.data, UHID_DATA_MAX,
    pub 0): min_t(size_t, ev->u.input.size, UHID_DATA_MAX),,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn uhid_dev_input2(uhid: *mut uhid_device, ev: *mut uhid_event) -> c_int {
    static int uhid_dev_input2(struct uhid_device *uhid, struct uhid_event *ev)
    {
    if (!READ_ONCE(uhid.running))
    pub -EINVAL: return,
    hid_safe_input_report(uhid.hid, HID_INPUT_REPORT, ev.u.input2.data, UHID_DATA_MAX,
    pub 0): min_t(size_t, ev->u.input2.size, UHID_DATA_MAX),,
    pub 0: return,
    }
    static int uhid_dev_get_report_reply(struct uhid_device *uhid,
    struct uhid_event *ev)
    {
    if (!READ_ONCE(uhid.running))
    pub -EINVAL: return,
    pub ev): uhid_report_wake_up(uhid, ev->u.get_report_reply.id,,
    pub 0: return,
    }
    static int uhid_dev_set_report_reply(struct uhid_device *uhid,
    struct uhid_event *ev)
    {
    if (!READ_ONCE(uhid.running))
    pub -EINVAL: return,
    pub ev): uhid_report_wake_up(uhid, ev->u.set_report_reply.id,,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn uhid_char_open(inode: *mut inode, file: *mut file) -> c_int {
    static int uhid_char_open(struct inode *inode, struct file *file)
    {
    pub uhid: *mut uhid_device,
    pub kzalloc_obj(*uhid): *mut uhid =,
    if (!uhid)
    pub -ENOMEM: return,
    pub false: uhid->running =,
    pub uhid_device_add_worker): INIT_WORK(&uhid->worker,,
    pub uhid: file->private_data =,
    pub file): stream_open(inode,,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn uhid_char_release(inode: *mut inode, file: *mut file) -> c_int {
    static int uhid_char_release(struct inode *inode, struct file *file)
    {
    pub file->private_data: *mut *mut uhid_device uhid =,
    pub i: c_uint,
    pub ++i): for (i = 0; i < UHID_BUFSIZE;,
    pub 0: return,
    }
    static ssize_t uhid_char_read(struct file *file, char __user *buffer,
    size_t count, loff_t *ppos)
    {
    pub file->private_data: *mut *mut uhid_device uhid =,
    pub ret: c_int,
    pub flags: c_ulong,
    pub len: usize,
// they need at least the "type" member of uhid_event
    if (count < sizeof(__u32))
    pub -EINVAL: return,
    try_again:
    if (file.f_flags & O_NONBLOCK) {
    if (uhid.head == uhid.tail)
    pub -EAGAIN: return,
    } else {
    ret = wait_event_interruptible(uhid.waitq,
    pub uhid->tail): uhid->head !=,
    if (ret)
    pub ret: return,
    }
    pub mutex_lock_interruptible(&uhid->devlock): ret =,
    if (ret)
    pub ret: return,
    if (uhid.head == uhid.tail) {
    pub try_again: goto,
    } else {
    pub sizeof(**uhid->outq)): *mut len = min(count,,
    if (copy_to_user(buffer, uhid.outq[uhid.tail], len)) {
    pub -EFAULT: ret =,
    } else {
    pub NULL: uhid->outq[uhid->tail] =,
    pub flags): spin_lock_irqsave(&uhid->qlock,,
    pub UHID_BUFSIZE: uhid->tail = (uhid->tail + 1) %,
    pub flags): spin_unlock_irqrestore(&uhid->qlock,,
    }
    }
    pub len: return ret ? ret :,
    }
    static ssize_t uhid_char_write(struct file *file, const char __user *buffer,
    size_t count, loff_t *ppos)
    {
    pub file->private_data: *mut *mut uhid_device uhid =,
    pub ret: c_int,
    pub len: usize,
// we need at least the "type" member of uhid_event
    if (count < sizeof(__u32))
    pub -EINVAL: return,
    pub mutex_lock_interruptible(&uhid->devlock): ret =,
    if (ret)
    pub ret: return,
    pub sizeof(uhid->input_buf)): memset(&uhid->input_buf, 0,,
    pub sizeof(uhid->input_buf)): len = min(count,,
    pub &uhid->input_buf): ret = uhid_event_from_user(buffer, len,,
    if (ret)
    pub unlock: goto,
    switch (uhid.input_buf.type) {
    case UHID_CREATE:
//
// 'struct uhid_create_req' contains a __user pointer which is
// copied from, so it's unsafe to allow this with elevated
// privileges (e.g. from a setuid binary) or via kernel_write().
//
    if (file.f_cred != current_cred()) {
    pr_err_once("UHID_CREATE from different security context by process %d (%s), this is not allowed.\n",
    pub current->comm): task_tgid_vnr(current),,
    pub -EACCES: ret =,
    pub unlock: goto,
    }
    pub &uhid->input_buf): ret = uhid_dev_create(uhid,,
    case UHID_CREATE2:
    pub &uhid->input_buf): ret = uhid_dev_create2(uhid,,
    case UHID_DESTROY:
    pub uhid_dev_destroy(uhid): ret =,
    case UHID_INPUT:
    pub &uhid->input_buf): ret = uhid_dev_input(uhid,,
    case UHID_INPUT2:
    pub &uhid->input_buf): ret = uhid_dev_input2(uhid,,
    case UHID_GET_REPORT_REPLY:
    pub &uhid->input_buf): ret = uhid_dev_get_report_reply(uhid,,
    case UHID_SET_REPORT_REPLY:
    pub &uhid->input_buf): ret = uhid_dev_set_report_reply(uhid,,
    default:
    pub -EOPNOTSUPP: ret =,
    }
    unlock:
// return "count" not "len" to not confuse the caller
    pub count: return ret ? ret :,
    }
#[no_mangle]
unsafe extern "C" fn uhid_char_poll(file: *mut file, wait: *mut poll_table) -> __poll_t {
    static __poll_t uhid_char_poll(struct file *file, poll_table *wait)
    {
    pub file->private_data: *mut *mut uhid_device uhid =,
    pub /: *mut *mut __poll_t mask = EPOLLOUT | EPOLLWRNORM; / uhid is always writable,
    pub wait): poll_wait(file, &uhid->waitq,,
    if (uhid.head != uhid.tail)
    pub EPOLLRDNORM: mask |= EPOLLIN |,
    pub mask: return,
    }
    static const struct file_operations uhid_fops = {
    .owner		= THIS_MODULE,
    .open		= uhid_char_open,
    .release	= uhid_char_release,
    .read		= uhid_char_read,
    .write		= uhid_char_write,
    .poll		= uhid_char_poll,
}

    static struct miscdevice uhid_misc = {
    .fops		= &uhid_fops,
    .minor		= UHID_MINOR,
    .name		= UHID_NAME,
    };
    module_misc_device(uhid_misc);
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("David Herrmann <dh.herrmann@gmail.com>");
    MODULE_DESCRIPTION("User-space I/O driver support for HID subsystem");
    MODULE_ALIAS_MISCDEV(UHID_MINOR);
    MODULE_ALIAS("devname:" UHID_NAME);
