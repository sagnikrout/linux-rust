//! Automatically rewritten from C to Rust
//! Source: drivers/s390/block/dasd_eer.c
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
// Character device driver for extended error reporting.
//
// Copyright IBM Corp. 2005
// extended error reporting for DASD ECKD devices
// Author(s): Stefan Weinhuber <wein@de.ibm.com>
//

//
// SECTION: the internal buffer
//
// The internal buffer is meant to store obaque blobs of data, so it does
// not know of higher level concepts like triggers.
// It consists of a number of pages that are used as a ringbuffer. Each data
// blob is stored in a simple record that consists of an integer, which
// contains the size of the following data, and the data bytes themselfes.
//
// To allow for multiple independent readers we create one internal buffer
// each time the device is opened and destroy the buffer when the file is
// closed again. The number of pages used for this buffer is determined by
// the module parmeter eer_pages.
//
// One record can be written to a buffer by using the functions
// - dasd_eer_start_record (one time per record to write the size to the
// buffer and reserve the space for the data)
// - dasd_eer_write_buffer (one or more times per record to write the data)
// The data can be written in several steps but you will have to compute
// the total size up front for the invocation of dasd_eer_start_record.
// If the ringbuffer is full, dasd_eer_start_record will remove the required
// number of old records.
//
// A record is typically read in two steps, first read the integer that
// specifies the size of the following data, then read the data.
// Both can be done by
// - dasd_eer_read_buffer
//
// For all mentioned functions you need to get the bufferlock first and keep
// it until a complete record is written or read.
//
// All information necessary to keep track of an internal buffer is kept in
// a struct eerbuffer. The buffer specific to a file pointer is strored in
// the private_data field of that file. To be able to write data to all
// existing buffers, each buffer is also added to the bufferlist.
// If the user does not want to read a complete record in one go, we have to
// keep track of the rest of the record. residual stores the number of bytes
// that are still to deliver. If the rest of the record is invalidated between
// two reads then residual will be set to -1 so that the next read will fail.
// All entries in the eerbuffer structure are protected with the bufferlock.
// To avoid races between writing to a buffer on the one side and creating
// and destroying buffers on the other side, the bufferlock must also be used
// to protect the bufferlist.
//
    let mut eer_pages: static int = 5;
    module_param(eer_pages, int, S_IRUGO|S_IWUSR);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eerbuffer {
    pub list: list_head,
    pub buffer: *mut c_char,
    pub buffersize: c_int,
    pub buffer_page_count: c_int,
    pub head: c_int,
    pub tail: c_int,
    pub residual: c_int,
}

    static LIST_HEAD(bufferlist);
    static DEFINE_SPINLOCK(bufferlock);
    static DECLARE_WAIT_QUEUE_HEAD(dasd_eer_read_wait_queue);
//
// How many free bytes are available on the buffer.
// Needs to be called with bufferlock held.
//
#[no_mangle]
unsafe extern "C" fn dasd_eer_get_free_bytes(eerb: *mut eerbuffer) -> c_int {
    static int dasd_eer_get_free_bytes(struct eerbuffer *eerb)
    {
    if (eerb.head < eerb.tail)
    return eerb.tail - eerb.head - 1;
    return eerb.buffersize - eerb.head + eerb.tail -1;
    }
//
// How many bytes of buffer space are used.
// Needs to be called with bufferlock held.
//
#[no_mangle]
unsafe extern "C" fn dasd_eer_get_filled_bytes(eerb: *mut eerbuffer) -> c_int {
    static int dasd_eer_get_filled_bytes(struct eerbuffer *eerb)
    {
    if (eerb.head >= eerb.tail)
    return eerb.head - eerb.tail;
    return eerb.buffersize - eerb.tail + eerb.head;
    }
//
// The dasd_eer_write_buffer function just copies count bytes of data
// to the buffer. Make sure to call dasd_eer_start_record first, to
// make sure that enough free space is available.
// Needs to be called with bufferlock held.
//
    static void dasd_eer_write_buffer(struct eerbuffer *eerb,
    char *data, int count)
    {
    unsigned long headindex,localhead;
    unsigned long rest, len;
    char *nextdata;
    nextdata = data;
    rest = count;
    while (rest > 0) {
    headindex = eerb.head / PAGE_SIZE;
    localhead = eerb.head % PAGE_SIZE;
    len = min(rest, PAGE_SIZE - localhead);
    memcpy(eerb.buffer[headindex]+localhead, nextdata, len);
    nextdata += len;
    rest -= len;
    eerb.head += len;
    if (eerb.head == eerb.buffersize)
    eerb.head = 0; /* wrap around */
    BUG_ON(eerb.head > eerb.buffersize);
    }
    }
//
// Needs to be called with bufferlock held.
//
#[no_mangle]
unsafe extern "C" fn dasd_eer_read_buffer(eerb: *mut eerbuffer, data: *mut c_char, count: c_int) -> c_int {
    static int dasd_eer_read_buffer(struct eerbuffer *eerb, char *data, int count)
    {
    unsigned long tailindex,localtail;
    unsigned long rest, len, finalcount;
    char *nextdata;
    finalcount = min(count, dasd_eer_get_filled_bytes(eerb));
    nextdata = data;
    rest = finalcount;
    while (rest > 0) {
    tailindex = eerb.tail / PAGE_SIZE;
    localtail = eerb.tail % PAGE_SIZE;
    len = min(rest, PAGE_SIZE - localtail);
    memcpy(nextdata, eerb.buffer[tailindex] + localtail, len);
    nextdata += len;
    rest -= len;
    eerb.tail += len;
    if (eerb.tail == eerb.buffersize)
    eerb.tail = 0; /* wrap around */
    BUG_ON(eerb.tail > eerb.buffersize);
    }
    return finalcount;
    }
//
// Whenever you want to write a blob of data to the internal buffer you
// have to start by using this function first. It will write the number
// of bytes that will be written to the buffer. If necessary it will remove
// old records to make room for the new one.
// Needs to be called with bufferlock held.
//
#[no_mangle]
unsafe extern "C" fn dasd_eer_start_record(eerb: *mut eerbuffer, count: c_int) -> c_int {
    static int dasd_eer_start_record(struct eerbuffer *eerb, int count)
    {
    int tailcount;
    if (count + sizeof(count) > eerb.buffersize)
    return -ENOMEM;
    while (dasd_eer_get_free_bytes(eerb) < count + sizeof(count)) {
    if (eerb.residual > 0) {
    eerb.tail += eerb.residual;
    if (eerb.tail >= eerb.buffersize)
    eerb.tail -= eerb.buffersize;
    eerb.residual = -1;
    }
    dasd_eer_read_buffer(eerb, (char *) &tailcount,
    sizeof(tailcount));
    eerb.tail += tailcount;
    if (eerb.tail >= eerb.buffersize)
    eerb.tail -= eerb.buffersize;
    }
    dasd_eer_write_buffer(eerb, (char*) &count, sizeof(count));
    return 0;
    };
//
// Release pages that are not used anymore.
//
#[no_mangle]
unsafe extern "C" fn dasd_eer_free_buffer_pages(buf: *mut c_char, no_pages: c_int) {
    static void dasd_eer_free_buffer_pages(char **buf, int no_pages)
    {
    int i;
    for (i = 0; i < no_pages; i++)
    kfree(buf[i]);
    }
//
// Allocate a new set of memory pages.
//
#[no_mangle]
unsafe extern "C" fn dasd_eer_allocate_buffer_pages(buf: *mut c_char, no_pages: c_int) -> c_int {
    static int dasd_eer_allocate_buffer_pages(char **buf, int no_pages)
    {
    int i;
    for (i = 0; i < no_pages; i++) {
    buf[i] = kzalloc(PAGE_SIZE, GFP_KERNEL);
    if (!buf[i]) {
    dasd_eer_free_buffer_pages(buf, i);
    return -ENOMEM;
    }
    }
    return 0;
    }
//
// SECTION: The extended error reporting functionality
//
// When a DASD device driver wants to report an error, it calls the
// function dasd_eer_write and gives the respective trigger ID as
// parameter. Currently there are four kinds of triggers:
//
// DASD_EER_FATALERROR:  all kinds of unrecoverable I/O problems
// DASD_EER_PPRCSUSPEND: PPRC was suspended
// DASD_EER_NOPATH:      There is no path to the device left.
// DASD_EER_STATECHANGE: The state of the device has changed.
//
// For the first three triggers all required information can be supplied by
// the caller. For these triggers a record is written by the function
// dasd_eer_write_standard_trigger.
//
// The DASD_EER_STATECHANGE trigger is special since a sense subsystem
// status ccw need to be executed to gather the necessary sense data first.
// The dasd_eer_snss function will queue the SNSS request and the request
// callback will then call dasd_eer_write with the DASD_EER_STATCHANGE
// trigger.
//
// To avoid memory allocations at runtime, the necessary memory is allocated
// when the extended error reporting is enabled for a device (by
// dasd_eer_probe). There is one sense subsystem status request for each
// eer enabled DASD device. The presence of the cqr in device->eer_cqr
// indicates that eer is enable for the device. The use of the snss request
// is protected by the DASD_FLAG_EER_IN_USE bit. When this flag indicates
// that the cqr is currently in use, dasd_eer_snss cannot start a second
// request but sets the DASD_FLAG_EER_SNSS flag instead. The callback of
// the SNSS request will check the bit and call dasd_eer_snss again.
//
pub const SNSS_DATA_SIZE: c_int = 44;
pub const DASD_EER_BUSID_SIZE: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dasd_eer_header {
    pub total_size: __u32,
    pub trigger: __u32,
    pub tv_sec: __u64,
    pub tv_usec: __u64,
    pub busid: [c_char; DASD_EER_BUSID_SIZE],
// C attribute field omitted
//
// The following function can be used for those triggers that have
// all necessary data available when the function is called.
// If the parameter cqr is not NULL, the chain of requests will be searched
// for valid sense data, and all valid sense data sets will be added to
// the triggers data.
//
    static void dasd_eer_write_standard_trigger(struct dasd_device *device,
    struct dasd_ccw_req *cqr,
    int trigger)
    {
    pub temp_cqr: *mut dasd_ccw_req,
    pub data_size: c_int,
    pub ts: timespec64,
    pub header: dasd_eer_header,
    pub flags: c_ulong,
    pub eerb: *mut eerbuffer,
    pub sense: *mut c_char,
// go through cqr chain and count the valid sense data sets
    pub 0: data_size =,
    pub temp_cqr->refers): for (temp_cqr = cqr; temp_cqr; temp_cqr =,
    if (dasd_get_sense(&temp_cqr.irb))
    pub 32: data_size +=,
    pub /: *mut *mut header.total_size = sizeof(header) + data_size + 4; / "EOR",
    pub trigger: header.trigger =,
    pub ts.tv_sec: header.tv_sec =,
    pub NSEC_PER_USEC: header.tv_usec = ts.tv_nsec /,
    strscpy(header.busid, dev_name(&device.cdev.dev),
    pub flags): spin_lock_irqsave(&bufferlock,,
    list_for_each_entry(eerb, &bufferlist, list) {
    pub header.total_size): dasd_eer_start_record(eerb,,
    pub sizeof(header)): *mut *mut dasd_eer_write_buffer(eerb, (char ) &header,,
    pub {: for (temp_cqr = cqr; temp_cqr; temp_cqr = temp_cqr->refers),
    pub dasd_get_sense(&temp_cqr->irb): sense =,
    if (sense)
    pub 32): dasd_eer_write_buffer(eerb, sense,,
    }
    pub 4): dasd_eer_write_buffer(eerb, "EOR",,
    }
    pub flags): spin_unlock_irqrestore(&bufferlock,,
    }
//
// This function writes a DASD_EER_STATECHANGE trigger.
//
    static void dasd_eer_write_snss_trigger(struct dasd_device *device,
    struct dasd_ccw_req *cqr,
    int trigger)
    {
    pub data_size: c_int,
    pub snss_rc: c_int,
    pub ts: timespec64,
    pub header: dasd_eer_header,
    pub flags: c_ulong,
    pub eerb: *mut eerbuffer,
    pub -EIO: snss_rc = (cqr->status == DASD_CQR_DONE) ? 0 :,
    if (snss_rc)
    pub 0: data_size =,
    else
    pub SNSS_DATA_SIZE: data_size =,
    pub /: *mut *mut header.total_size = sizeof(header) + data_size + 4; / "EOR",
    pub DASD_EER_STATECHANGE: header.trigger =,
    pub ts.tv_sec: header.tv_sec =,
    pub NSEC_PER_USEC: header.tv_usec = ts.tv_nsec /,
    strscpy(header.busid, dev_name(&device.cdev.dev),
    pub flags): spin_lock_irqsave(&bufferlock,,
    list_for_each_entry(eerb, &bufferlist, list) {
    pub header.total_size): dasd_eer_start_record(eerb,,
    pub sizeof(header)): *mut *mut dasd_eer_write_buffer(eerb, (char ) &header ,,
    if (!snss_rc)
    pub SNSS_DATA_SIZE): dasd_eer_write_buffer(eerb, cqr->data,,
    pub 4): dasd_eer_write_buffer(eerb, "EOR",,
    }
    pub flags): spin_unlock_irqrestore(&bufferlock,,
    }
//
// This function is called for all triggers. It calls the appropriate
// function that writes the actual trigger records.
//
    void dasd_eer_write(struct dasd_device *device, struct dasd_ccw_req *cqr,
    unsigned int id)
    {
    if (!device.eer_cqr)
    switch (id) {
    case DASD_EER_FATALERROR:
    case DASD_EER_PPRCSUSPEND:
    pub id): dasd_eer_write_standard_trigger(device, cqr,,
    case DASD_EER_NOPATH:
    case DASD_EER_NOSPC:
    case DASD_EER_AUTOQUIESCE:
    pub id): dasd_eer_write_standard_trigger(device, NULL,,
    case DASD_EER_STATECHANGE:
    pub id): dasd_eer_write_snss_trigger(device, cqr,,
    default: /* unknown trigger, so we write it without any sense data */
    pub id): dasd_eer_write_standard_trigger(device, NULL,,
    }
    }
//
// Start a sense subsystem status request.
// Needs to be called with the device held.
//
#[no_mangle]
pub unsafe extern "C" fn dasd_eer_snss(device: *mut dasd_device) {
    void dasd_eer_snss(struct dasd_device *device)
    {
    pub cqr: *mut dasd_ccw_req,
    pub device->eer_cqr: cqr =,
    if (!cqr)	/* Device not eer enabled. */
    if (test_and_set_bit(DASD_FLAG_EER_IN_USE, &device.flags)) {
// Sense subsystem status request in use.
    pub &device->flags): set_bit(DASD_FLAG_EER_SNSS,,
    }
// cdev is already locked, can't use dasd_add_request_head
    pub &device->flags): clear_bit(DASD_FLAG_EER_SNSS,,
    pub DASD_CQR_QUEUED: cqr->status =,
    pub &device->ccw_queue): list_add(&cqr->devlist,,
    }
//
// Callback function for use with sense subsystem status request.
//
#[no_mangle]
unsafe extern "C" fn dasd_eer_snss_cb(cqr: *mut dasd_ccw_req, data: *mut c_void) {
    static void dasd_eer_snss_cb(struct dasd_ccw_req *cqr, void *data)
    {
    pub cqr->startdev: *mut *mut dasd_device device =,
    pub flags: c_ulong,
    pub DASD_EER_STATECHANGE): dasd_eer_write(device, cqr,,
    pub flags): spin_lock_irqsave(get_ccwdev_lock(device->cdev),,
    if (device.eer_cqr == cqr) {
    pub &device->flags): clear_bit(DASD_FLAG_EER_IN_USE,,
    if (test_bit(DASD_FLAG_EER_SNSS, &device.flags))
// Another SNSS has been requested in the meantime.
    pub NULL: cqr =,
    }
    pub flags): spin_unlock_irqrestore(get_ccwdev_lock(device->cdev),,
    if (cqr)
//
// Extended error recovery has been switched off while
// the SNSS request was running. It could even have
// been switched off and on again in which case there
// is a new ccw in device->eer_cqr. Free the "old"
// snss request now.
//
    pub device): dasd_sfree_request(cqr,,
    }
//
// Enable error reporting on a given device.
//
#[no_mangle]
pub unsafe extern "C" fn dasd_eer_enable(device: *mut dasd_device) -> c_int {
    int dasd_eer_enable(struct dasd_device *device)
    {
    pub NULL: *mut *mut dasd_ccw_req cqr =,
    pub flags: c_ulong,
    pub ccw: *mut ccw1,
    pub 0: int rc =,
    pub flags): spin_lock_irqsave(get_ccwdev_lock(device->cdev),,
    if (device.eer_cqr)
    pub out: goto,
    else if (!device.discipline ||
    strcmp(device.discipline.name, "ECKD"))
    pub -EMEDIUMTYPE: rc =,
#[no_mangle]
pub unsafe extern "C" fn if(_arg: test_bit(DASD_FLAG_OFFLINE, _arg: &device->flags)) -> else {
    else if (test_bit(DASD_FLAG_OFFLINE, &device.flags))
    pub -EBUSY: rc =,
    if (rc)
    pub out: goto,
    cqr = dasd_smalloc_request(DASD_ECKD_MAGIC, 1 /* SNSS */,
    pub NULL): SNSS_DATA_SIZE, device,,
    if (IS_ERR(cqr)) {
    pub -ENOMEM: rc =,
    pub NULL: cqr =,
    pub out: goto,
    }
    pub device: cqr->startdev =,
    pub 255: cqr->retries =,
    pub HZ: *mut *mut cqr->expires = 10,
    pub &cqr->flags): clear_bit(DASD_CQR_FLAGS_USE_ERP,,
    pub &cqr->flags): set_bit(DASD_CQR_ALLOW_SLOCK,,
    pub cqr->cpaddr: ccw =,
    pub DASD_ECKD_CCW_SNSS: ccw->cmd_code =,
    pub SNSS_DATA_SIZE: ccw->count =,
    pub 0: ccw->flags =,
    pub virt_to_dma32(cqr->data): ccw->cda =,
    pub get_tod_clock(): cqr->buildclk =,
    pub DASD_CQR_FILLED: cqr->status =,
    pub dasd_eer_snss_cb: cqr->callback =,
    if (!device.eer_cqr) {
    pub cqr: device->eer_cqr =,
    pub NULL: cqr =,
    }
    out:
    pub flags): spin_unlock_irqrestore(get_ccwdev_lock(device->cdev),,
    if (cqr)
    pub device): dasd_sfree_request(cqr,,
    pub rc: return,
    }
//
// Disable error reporting on a given device.
//
#[no_mangle]
pub unsafe extern "C" fn dasd_eer_disable(device: *mut dasd_device) {
    void dasd_eer_disable(struct dasd_device *device)
    {
    pub cqr: *mut dasd_ccw_req,
    pub flags: c_ulong,
    pub in_use: c_int,
    if (!device.eer_cqr)
    pub flags): spin_lock_irqsave(get_ccwdev_lock(device->cdev),,
    pub device->eer_cqr: cqr =,
    pub NULL: device->eer_cqr =,
    pub &device->flags): clear_bit(DASD_FLAG_EER_SNSS,,
    pub &device->flags): in_use = test_and_clear_bit(DASD_FLAG_EER_IN_USE,,
    pub flags): spin_unlock_irqrestore(get_ccwdev_lock(device->cdev),,
    if (cqr && !in_use)
    pub device): dasd_sfree_request(cqr,,
    }
//
// SECTION: the device operations
//
// On the one side we need a lock to access our internal buffer, on the
// other side a copy_to_user can sleep. So we need to copy the data we have
// to transfer in a readbuffer, which is protected by the readbuffer_mutex.
//
    pub readbuffer: [static char; PAGE_SIZE],
    pub DEFINE_MUTEX(readbuffer_mutex): static,
#[no_mangle]
unsafe extern "C" fn dasd_eer_open(inp: *mut inode, filp: *mut file) -> c_int {
    static int dasd_eer_open(struct inode *inp, struct file *filp)
    {
    pub eerb: *mut eerbuffer,
    pub flags: c_ulong,
    pub eerbuffer): eerb = kzalloc_obj(struct,
    if (!eerb)
    pub -ENOMEM: return,
    pub eer_pages: eerb->buffer_page_count =,
    if (eerb.buffer_page_count < 1 ||
    eerb.buffer_page_count > INT_MAX / PAGE_SIZE) {
    DBF_EVENT(DBF_WARNING, "can't open device since module "
    "parameter eer_pages is smaller than 1 or"
    pub PAGE_SIZE)): " bigger than %d", (int)(INT_MAX /,
    pub -EINVAL: return,
    }
    pub PAGE_SIZE: *mut *mut eerb->buffersize = eerb->buffer_page_count,
    eerb.buffer = kmalloc_array(eerb.buffer_page_count, sizeof(char *),
    if (!eerb.buffer) {
    pub -ENOMEM: return,
    }
    if (dasd_eer_allocate_buffer_pages(eerb.buffer,
    eerb.buffer_page_count)) {
    pub -ENOMEM: return,
    }
    pub eerb: filp->private_data =,
    pub flags): spin_lock_irqsave(&bufferlock,,
    pub &bufferlist): list_add(&eerb->list,,
    pub flags): spin_unlock_irqrestore(&bufferlock,,
    pub nonseekable_open(inp,filp): return,
    }
#[no_mangle]
unsafe extern "C" fn dasd_eer_close(inp: *mut inode, filp: *mut file) -> c_int {
    static int dasd_eer_close(struct inode *inp, struct file *filp)
    {
    pub eerb: *mut eerbuffer,
    pub flags: c_ulong,
    pub filp->private_data: *mut *mut eerb = (struct eerbuffer ),
    pub flags): spin_lock_irqsave(&bufferlock,,
    pub flags): spin_unlock_irqrestore(&bufferlock,,
    pub eerb->buffer_page_count): dasd_eer_free_buffer_pages(eerb->buffer,,
    pub 0: return,
    }
    static ssize_t dasd_eer_read(struct file *filp, char __user *buf,
    size_t count, loff_t *ppos)
    {
    pub tc,rc: c_int,
    pub tailcount,effective_count: c_int,
    pub flags: c_ulong,
    pub eerb: *mut eerbuffer,
    pub filp->private_data: *mut *mut eerb = (struct eerbuffer ),
    if (mutex_lock_interruptible(&readbuffer_mutex))
    pub -ERESTARTSYS: return,
    pub flags): spin_lock_irqsave(&bufferlock,,
    if (eerb.residual < 0) { /* the remainder of this record */
// has been deleted
    pub 0: eerb->residual =,
    pub flags): spin_unlock_irqrestore(&bufferlock,,
    pub -EIO: return,
    } else if (eerb.residual > 0) {
// OK we still have a second half of a record to deliver
    pub count): effective_count = min(eerb->residual, (int),
    pub effective_count: eerb->residual -=,
    } else {
    pub 0: tc =,
    while (!tc) {
    tc = dasd_eer_read_buffer(eerb, (char *) &tailcount,
    if (!tc) {
// no data available
    pub flags): spin_unlock_irqrestore(&bufferlock,,
    if (filp.f_flags & O_NONBLOCK)
    pub -EAGAIN: return,
    rc = wait_event_interruptible(
    dasd_eer_read_wait_queue,
    pub eerb->tail): eerb->head !=,
    if (rc)
    pub rc: return,
    if (mutex_lock_interruptible(&readbuffer_mutex))
    pub -ERESTARTSYS: return,
    pub flags): spin_lock_irqsave(&bufferlock,,
    }
    }
    pub sizeof(tailcount)): WARN_ON(tc !=,
    pub min(tailcount,(int)count): effective_count =,
    pub effective_count: eerb->residual = tailcount -,
    }
    pub effective_count): tc = dasd_eer_read_buffer(eerb, readbuffer,,
    pub effective_count): WARN_ON(tc !=,
    pub flags): spin_unlock_irqrestore(&bufferlock,,
    if (copy_to_user(buf, readbuffer, effective_count)) {
    pub -EFAULT: return,
    }
    pub effective_count: return,
    }
#[no_mangle]
unsafe extern "C" fn dasd_eer_poll(filp: *mut file, ptable: *mut poll_table) -> __poll_t {
    static __poll_t dasd_eer_poll(struct file *filp, poll_table *ptable)
    {
    pub mask: __poll_t,
    pub flags: c_ulong,
    pub eerb: *mut eerbuffer,
    pub filp->private_data: *mut *mut eerb = (struct eerbuffer ),
    pub ptable): poll_wait(filp, &dasd_eer_read_wait_queue,,
    pub flags): spin_lock_irqsave(&bufferlock,,
    if (eerb.head != eerb.tail)
    pub EPOLLRDNORM: mask = EPOLLIN |,
    else
    pub 0: mask =,
    pub flags): spin_unlock_irqrestore(&bufferlock,,
    pub mask: return,
    }
    static const struct file_operations dasd_eer_fops = {
    .open		= &dasd_eer_open,
    .release	= &dasd_eer_close,
    .read		= &dasd_eer_read,
    .poll		= &dasd_eer_poll,
    .owner		= THIS_MODULE,
    .llseek		= noop_llseek,
}

    static struct miscdevice *dasd_eer_dev = core::ptr::null_mut();
#[no_mangle]
pub unsafe extern "C" fn dasd_eer_init() -> int __init {
    int __init dasd_eer_init(void)
    {
    int rc;
    dasd_eer_dev = kzalloc_obj(*dasd_eer_dev);
    if (!dasd_eer_dev)
    return -ENOMEM;
    dasd_eer_dev.minor = MISC_DYNAMIC_MINOR;
    dasd_eer_dev.name  = "dasd_eer";
    dasd_eer_dev.fops  = &dasd_eer_fops;
    rc = misc_register(dasd_eer_dev);
    if (rc) {
    kfree(dasd_eer_dev);
    dasd_eer_dev = core::ptr::null_mut();
    DBF_EVENT(DBF_ERR, "%s", "dasd_eer_init could not "
    "register misc device");
    return rc;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn dasd_eer_exit() {
    void dasd_eer_exit(void)
    {
    if (dasd_eer_dev) {
    misc_deregister(dasd_eer_dev);
    kfree(dasd_eer_dev);
    dasd_eer_dev = core::ptr::null_mut();
    }
    }
