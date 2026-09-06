//! Automatically rewritten from C to Rust
//! Source: drivers/hwtracing/coresight/coresight-etb10.c
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
// Copyright (c) 2011-2012, The Linux Foundation. All rights reserved.
//
// Description: CoreSight Embedded Trace Buffer driver
//

pub const ETB_RAM_DEPTH_REG: c_uint = 0x004;
pub const ETB_STATUS_REG: c_uint = 0x00c;
pub const ETB_RAM_READ_DATA_REG: c_uint = 0x010;
pub const ETB_RAM_READ_POINTER: c_uint = 0x014;
pub const ETB_RAM_WRITE_POINTER: c_uint = 0x018;
pub const ETB_TRG: c_uint = 0x01c;
pub const ETB_CTL_REG: c_uint = 0x020;
pub const ETB_RWD_REG: c_uint = 0x024;
pub const ETB_FFSR: c_uint = 0x300;
pub const ETB_FFCR: c_uint = 0x304;
pub const ETB_ITMISCOP0: c_uint = 0xee0;
pub const ETB_ITTRFLINACK: c_uint = 0xee4;
pub const ETB_ITTRFLIN: c_uint = 0xee8;
pub const ETB_ITATBDATA0: c_uint = 0xeeC;
pub const ETB_ITATBCTR2: c_uint = 0xef0;
pub const ETB_ITATBCTR1: c_uint = 0xef4;
pub const ETB_ITATBCTR0: c_uint = 0xef8;
// register description
// STS - 0x00C

// CTL - 0x020

// FFCR - 0x304

pub const ETB_FFCR_BIT: c_int = 6;
pub const ETB_FFSR_BIT: c_int = 1;
pub const ETB_FRAME_SIZE_WORDS: c_int = 4;
//
// struct etb_drvdata - specifics associated to an ETB component
// @base:	memory mapped base address for this component.
// @atclk:	optional clock for the core parts of the ETB.
// @csdev:	component vitals needed by the framework.
// @miscdev:	specifics to handle "/dev/xyz.etb" entry.
// @spinlock:	only one at a time pls.
// @reading:	synchronise user space access to etb buffer.
// @pid:	Process ID of the process being monitored by the session
// that is using this component.
// @buf:	area of memory where ETB buffer content gets sent.
// @buffer_depth: size of @buf.
// @trigger_cntr: amount of words to store after a trigger.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct etb_drvdata {
    pub base: *mut void __iomem,
    pub atclk: *mut clk,
    pub csdev: *mut coresight_device,
    pub miscdev: miscdevice,
    pub spinlock: raw_spinlock_t,
    pub reading: core::sync::atomic::AtomicI32,
    pub pid: pid_t,
    pub buf: *mut u8,
    pub buffer_depth: u32,
    pub trigger_cntr: u32,
}

    static int etb_set_buffer(struct coresight_device *csdev,
    struct perf_output_handle *handle);
#[no_mangle]
unsafe extern "C" fn etb_get_buffer_depth(drvdata: *mut etb_drvdata) -> c_uint {
    static unsigned int etb_get_buffer_depth(struct etb_drvdata *drvdata)
    {
    return readl_relaxed(drvdata.base + ETB_RAM_DEPTH_REG);
    }
#[no_mangle]
unsafe extern "C" fn __etb_enable_hw(drvdata: *mut etb_drvdata) {
    static void __etb_enable_hw(struct etb_drvdata *drvdata)
    {
    int i;
    u32 depth;
    CS_UNLOCK(drvdata.base);
    depth = drvdata.buffer_depth;
// reset write RAM pointer address
    writel_relaxed(0x0, drvdata.base + ETB_RAM_WRITE_POINTER);
// clear entire RAM buffer
    for (i = 0; i < depth; i++)
    writel_relaxed(0x0, drvdata.base + ETB_RWD_REG);
// reset write RAM pointer address
    writel_relaxed(0x0, drvdata.base + ETB_RAM_WRITE_POINTER);
// reset read RAM pointer address
    writel_relaxed(0x0, drvdata.base + ETB_RAM_READ_POINTER);
    writel_relaxed(drvdata.trigger_cntr, drvdata.base + ETB_TRG);
    writel_relaxed(ETB_FFCR_EN_FTC | ETB_FFCR_STOP_TRIGGER,
    drvdata.base + ETB_FFCR);
// ETB trace capture enable
    writel_relaxed(ETB_CTL_CAPT_EN, drvdata.base + ETB_CTL_REG);
    CS_LOCK(drvdata.base);
    }
#[no_mangle]
unsafe extern "C" fn etb_enable_hw(drvdata: *mut etb_drvdata) -> c_int {
    static int etb_enable_hw(struct etb_drvdata *drvdata)
    {
    let mut rc: c_int = coresight_claim_device(drvdata.csdev);
    if (rc)
    return rc;
    __etb_enable_hw(drvdata);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn etb_enable_sysfs(csdev: *mut coresight_device) -> c_int {
    static int etb_enable_sysfs(struct coresight_device *csdev)
    {
    let mut ret: c_int = 0;
    unsigned long flags;
    struct etb_drvdata *drvdata = dev_get_drvdata(csdev.dev.parent);
    raw_spin_lock_irqsave(&drvdata.spinlock, flags);
// Don't messup with perf sessions.
    if (coresight_get_mode(csdev) == CS_MODE_PERF) {
    ret = -EBUSY;
    goto out;
    }
    if (coresight_get_mode(csdev) == CS_MODE_DISABLED) {
    ret = etb_enable_hw(drvdata);
    if (ret)
    goto out;
    coresight_set_mode(csdev, CS_MODE_SYSFS);
    }
    csdev.refcnt++;
    out:
    raw_spin_unlock_irqrestore(&drvdata.spinlock, flags);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn etb_enable_perf(csdev: *mut coresight_device, path: *mut coresight_path) -> c_int {
    static int etb_enable_perf(struct coresight_device *csdev, struct coresight_path *path)
    {
    let mut ret: c_int = 0;
    pid_t pid;
    unsigned long flags;
    struct etb_drvdata *drvdata = dev_get_drvdata(csdev.dev.parent);
    struct perf_output_handle *handle = path.handle;
    struct cs_buffers *buf = etm_perf_sink_config(handle);
    raw_spin_lock_irqsave(&drvdata.spinlock, flags);
// No need to continue if the component is already in used by sysFS.
    if (coresight_get_mode(drvdata.csdev) == CS_MODE_SYSFS) {
    ret = -EBUSY;
    goto out;
    }
// Get a handle on the pid of the process to monitor
    pid = buf.pid;
    if (drvdata.pid != -1 && drvdata.pid != pid) {
    ret = -EBUSY;
    goto out;
    }
//
// No HW configuration is needed if the sink is already in
// use for this session.
//
    if (drvdata.pid == pid) {
    csdev.refcnt++;
    goto out;
    }
//
// We don't have an internal state to clean up if we fail to setup
// the perf buffer. So we can perform the step before we turn the
// ETB on and leave without cleaning up.
//
    ret = etb_set_buffer(csdev, handle);
    if (ret)
    goto out;
    ret = etb_enable_hw(drvdata);
    if (!ret) {
// Associate with monitored process.
    drvdata.pid = pid;
    coresight_set_mode(drvdata.csdev, CS_MODE_PERF);
    csdev.refcnt++;
    }
    out:
    raw_spin_unlock_irqrestore(&drvdata.spinlock, flags);
    return ret;
    }
    static int etb_enable(struct coresight_device *csdev, enum cs_mode mode,
    struct coresight_path *path)
    {
    int ret;
    switch (mode) {
    case CS_MODE_SYSFS:
    ret = etb_enable_sysfs(csdev);
    break;
    case CS_MODE_PERF:
    ret = etb_enable_perf(csdev, path);
    break;
    default:
    ret = -EINVAL;
    break;
    }
    if (ret)
    return ret;
    dev_dbg(&csdev.dev, "ETB enabled\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __etb_disable_hw(drvdata: *mut etb_drvdata) {
    static void __etb_disable_hw(struct etb_drvdata *drvdata)
    {
    u32 ffcr;
    struct device *dev = &drvdata.csdev.dev;
    struct csdev_access *csa = &drvdata.csdev.access;
    CS_UNLOCK(drvdata.base);
    ffcr = readl_relaxed(drvdata.base + ETB_FFCR);
// stop formatter when a stop has completed
    ffcr |= ETB_FFCR_STOP_FI;
    writel_relaxed(ffcr, drvdata.base + ETB_FFCR);
// manually generate a flush of the system
    ffcr |= ETB_FFCR_FON_MAN;
    writel_relaxed(ffcr, drvdata.base + ETB_FFCR);
    if (coresight_timeout(csa, ETB_FFCR, ETB_FFCR_BIT, 0)) {
    dev_err(dev,
    "timeout while waiting for completion of Manual Flush\n");
    }
// disable trace capture
    writel_relaxed(0x0, drvdata.base + ETB_CTL_REG);
    if (coresight_timeout(csa, ETB_FFSR, ETB_FFSR_BIT, 1)) {
    dev_err(dev,
    "timeout while waiting for Formatter to Stop\n");
    }
    CS_LOCK(drvdata.base);
    }
#[no_mangle]
unsafe extern "C" fn etb_dump_hw(drvdata: *mut etb_drvdata) {
    static void etb_dump_hw(struct etb_drvdata *drvdata)
    {
    let mut lost: bool = false;
    int i;
    u8 *buf_ptr;
    u32 read_data, depth;
    u32 read_ptr, write_ptr;
    u32 frame_off, frame_endoff;
    struct device *dev = &drvdata.csdev.dev;
    CS_UNLOCK(drvdata.base);
    read_ptr = readl_relaxed(drvdata.base + ETB_RAM_READ_POINTER);
    write_ptr = readl_relaxed(drvdata.base + ETB_RAM_WRITE_POINTER);
    frame_off = write_ptr % ETB_FRAME_SIZE_WORDS;
    frame_endoff = ETB_FRAME_SIZE_WORDS - frame_off;
    if (frame_off) {
    dev_err(dev,
    "write_ptr: %lu not aligned to formatter frame size\n",
    (unsigned long)write_ptr);
    dev_err(dev, "frameoff: %lu, frame_endoff: %lu\n",
    (unsigned long)frame_off, (unsigned long)frame_endoff);
    write_ptr += frame_endoff;
    }
    if ((readl_relaxed(drvdata.base + ETB_STATUS_REG)
    & ETB_STATUS_RAM_FULL) == 0) {
    writel_relaxed(0x0, drvdata.base + ETB_RAM_READ_POINTER);
    } else {
    writel_relaxed(write_ptr, drvdata.base + ETB_RAM_READ_POINTER);
    lost = true;
    }
    depth = drvdata.buffer_depth;
    buf_ptr = drvdata.buf;
    for (i = 0; i < depth; i++) {
    read_data = readl_relaxed(drvdata.base +
    ETB_RAM_READ_DATA_REG);
// (u32 *)buf_ptr = read_data;
    buf_ptr += 4;
    }
    if (lost)
    coresight_insert_barrier_packet(drvdata.buf);
    if (frame_off) {
    buf_ptr -= (frame_endoff * 4);
    for (i = 0; i < frame_endoff; i++) {
// buf_ptr++ = 0x0;
    }
    }
    writel_relaxed(read_ptr, drvdata.base + ETB_RAM_READ_POINTER);
    CS_LOCK(drvdata.base);
    }
#[no_mangle]
unsafe extern "C" fn etb_disable_hw(drvdata: *mut etb_drvdata) {
    static void etb_disable_hw(struct etb_drvdata *drvdata)
    {
    __etb_disable_hw(drvdata);
    etb_dump_hw(drvdata);
    coresight_disclaim_device(drvdata.csdev);
    }
#[no_mangle]
unsafe extern "C" fn etb_disable(csdev: *mut coresight_device) -> c_int {
    static int etb_disable(struct coresight_device *csdev)
    {
    struct etb_drvdata *drvdata = dev_get_drvdata(csdev.dev.parent);
    unsigned long flags;
    raw_spin_lock_irqsave(&drvdata.spinlock, flags);
    csdev.refcnt--;
    if (csdev.refcnt) {
    raw_spin_unlock_irqrestore(&drvdata.spinlock, flags);
    return -EBUSY;
    }
// Complain if we (somehow) got out of sync
    WARN_ON_ONCE(coresight_get_mode(csdev) == CS_MODE_DISABLED);
    etb_disable_hw(drvdata);
// Dissociate from monitored process.
    drvdata.pid = -1;
    coresight_set_mode(csdev, CS_MODE_DISABLED);
    raw_spin_unlock_irqrestore(&drvdata.spinlock, flags);
    dev_dbg(&csdev.dev, "ETB disabled\n");
    return 0;
    }
    static void *etb_alloc_buffer(struct coresight_device *csdev,
    struct perf_event *event, void **pages,
    int nr_pages, bool overwrite)
    {
    int node;
    struct cs_buffers *buf;
    node = (event.cpu == -1) ? NUMA_NO_NODE : cpu_to_node(event.cpu);
    buf = kzalloc_node(sizeof(struct cs_buffers), GFP_KERNEL, node);
    if (!buf)
    return core::ptr::null_mut();
    buf.pid = task_pid_nr(event.owner);
    buf.snapshot = overwrite;
    buf.nr_pages = nr_pages;
    buf.data_pages = pages;
    return buf;
    }
#[no_mangle]
unsafe extern "C" fn etb_free_buffer(config: *mut c_void) {
    static void etb_free_buffer(void *config)
    {
    struct cs_buffers *buf = config;
    kfree(buf);
    }
    static int etb_set_buffer(struct coresight_device *csdev,
    struct perf_output_handle *handle)
    {
    let mut ret: c_int = 0;
    unsigned long head;
    struct cs_buffers *buf = etm_perf_sink_config(handle);
    if (!buf)
    return -EINVAL;
// wrap head around to the amount of space we have
    head = handle.head & ((buf.nr_pages << PAGE_SHIFT) - 1);
// find the page to write to
    buf.cur = head / PAGE_SIZE;
// and offset within that page
    buf.offset = head % PAGE_SIZE;
    local_set(&buf.data_size, 0);
    return ret;
    }
    static unsigned long etb_update_buffer(struct coresight_device *csdev,
    struct perf_output_handle *handle,
    void *sink_config)
    {
    let mut lost: bool = false;
    int i, cur;
    u8 *buf_ptr;
    const u32 *barrier;
    u32 read_ptr, write_ptr, capacity;
    u32 status, read_data;
    unsigned long offset, to_read = 0, flags;
    struct cs_buffers *buf = sink_config;
    struct etb_drvdata *drvdata = dev_get_drvdata(csdev.dev.parent);
    if (!buf)
    return 0;
    capacity = drvdata.buffer_depth * ETB_FRAME_SIZE_WORDS;
    raw_spin_lock_irqsave(&drvdata.spinlock, flags);
// Don't do anything if another tracer is using this sink
    if (csdev.refcnt != 1)
    goto out;
    __etb_disable_hw(drvdata);
    CS_UNLOCK(drvdata.base);
// unit is in words, not bytes
    read_ptr = readl_relaxed(drvdata.base + ETB_RAM_READ_POINTER);
    write_ptr = readl_relaxed(drvdata.base + ETB_RAM_WRITE_POINTER);
//
// Entries should be aligned to the frame size.  If they are not
// go back to the last alignment point to give decoding tools a
// chance to fix things.
//
    if (write_ptr % ETB_FRAME_SIZE_WORDS) {
    dev_err(&csdev.dev,
    "write_ptr: %lu not aligned to formatter frame size\n",
    (unsigned long)write_ptr);
    write_ptr &= ~(ETB_FRAME_SIZE_WORDS - 1);
    lost = true;
    }
//
// Get a hold of the status register and see if a wrap around
// has occurred.  If so adjust things accordingly.  Otherwise
// start at the beginning and go until the write pointer has
// been reached.
//
    status = readl_relaxed(drvdata.base + ETB_STATUS_REG);
    if (status & ETB_STATUS_RAM_FULL) {
    lost = true;
    to_read = capacity;
    read_ptr = write_ptr;
    } else {
    to_read = CIRC_CNT(write_ptr, read_ptr, drvdata.buffer_depth);
    to_read *= ETB_FRAME_SIZE_WORDS;
    }
//
// Make sure we don't overwrite data that hasn't been consumed yet.
// It is entirely possible that the HW buffer has more data than the
// ring buffer can currently handle.  If so adjust the start address
// to take only the last traces.
//
// In snapshot mode we are looking to get the latest traces only and as
// such, we don't care about not overwriting data that hasn't been
// processed by user space.
//
    if (!buf.snapshot && to_read > handle.size) {
    let mut mask: u32 = ~(ETB_FRAME_SIZE_WORDS - 1);
// The new read pointer must be frame size aligned
    to_read = handle.size & mask;
//
// Move the RAM read pointer up, keeping in mind that
// everything is in frame size units.
//
    read_ptr = (write_ptr + drvdata.buffer_depth) -
    to_read / ETB_FRAME_SIZE_WORDS;
// Wrap around if need be
    if (read_ptr > (drvdata.buffer_depth - 1))
    read_ptr -= drvdata.buffer_depth;
// let the decoder know we've skipped ahead
    lost = true;
    }
//
// Don't set the TRUNCATED flag in snapshot mode because 1) the
// captured buffer is expected to be truncated and 2) a full buffer
// prevents the event from being re-enabled by the perf core,
// resulting in stale data being send to user space.
//
    if (!buf.snapshot && lost)
    perf_aux_output_flag(handle, PERF_AUX_FLAG_TRUNCATED);
// finally tell HW where we want to start reading from
    writel_relaxed(read_ptr, drvdata.base + ETB_RAM_READ_POINTER);
    cur = buf.cur;
    offset = buf.offset;
    barrier = coresight_barrier_pkt;
    for (i = 0; i < to_read; i += 4) {
    buf_ptr = buf.data_pages[cur] + offset;
    read_data = readl_relaxed(drvdata.base +
    ETB_RAM_READ_DATA_REG);
    if (lost && i < CORESIGHT_BARRIER_PKT_SIZE) {
    read_data = *barrier;
    barrier++;
    }
// (u32 *)buf_ptr = read_data;
    buf_ptr += 4;
    offset += 4;
    if (offset >= PAGE_SIZE) {
    offset = 0;
    cur++;
// wrap around at the end of the buffer
    cur &= buf.nr_pages - 1;
    }
    }
// reset ETB buffer for next run
    writel_relaxed(0x0, drvdata.base + ETB_RAM_READ_POINTER);
    writel_relaxed(0x0, drvdata.base + ETB_RAM_WRITE_POINTER);
//
// In snapshot mode we simply increment the head by the number of byte
// that were written.  User space will figure out how many bytes to get
// from the AUX buffer based on the position of the head.
//
    if (buf.snapshot)
    handle.head += to_read;
    __etb_enable_hw(drvdata);
    CS_LOCK(drvdata.base);
    out:
    raw_spin_unlock_irqrestore(&drvdata.spinlock, flags);
    return to_read;
    }
    static const struct coresight_ops_sink etb_sink_ops = {
    .enable		= etb_enable,
    .disable	= etb_disable,
    .alloc_buffer	= etb_alloc_buffer,
    .free_buffer	= etb_free_buffer,
    .update_buffer	= etb_update_buffer,
    };
    static const struct coresight_ops etb_cs_ops = {
    .sink_ops	= &etb_sink_ops,
    };
#[no_mangle]
unsafe extern "C" fn etb_dump(drvdata: *mut etb_drvdata) {
    static void etb_dump(struct etb_drvdata *drvdata)
    {
    unsigned long flags;
    raw_spin_lock_irqsave(&drvdata.spinlock, flags);
    if (coresight_get_mode(drvdata.csdev) == CS_MODE_SYSFS) {
    __etb_disable_hw(drvdata);
    etb_dump_hw(drvdata);
    __etb_enable_hw(drvdata);
    }
    raw_spin_unlock_irqrestore(&drvdata.spinlock, flags);
    dev_dbg(&drvdata.csdev.dev, "ETB dumped\n");
    }
#[no_mangle]
unsafe extern "C" fn etb_open(inode: *mut inode, file: *mut file) -> c_int {
    static int etb_open(struct inode *inode, struct file *file)
    {
    struct etb_drvdata *drvdata = container_of(file.private_data,
    struct etb_drvdata, miscdev);
    if (atomic_cmpxchg(&drvdata.reading, 0, 1))
    return -EBUSY;
    dev_dbg(&drvdata.csdev.dev, "%s: successfully opened\n", __func__);
    return 0;
    }
    static ssize_t etb_read(struct file *file, char __user *data,
    size_t len, loff_t *ppos)
    {
    u32 depth;
    struct etb_drvdata *drvdata = container_of(file.private_data,
    struct etb_drvdata, miscdev);
    struct device *dev = &drvdata.csdev.dev;
    etb_dump(drvdata);
    depth = drvdata.buffer_depth;
    if (*ppos + len > depth * 4)
    len = depth * 4 - *ppos;
    if (copy_to_user(data, drvdata.buf + *ppos, len)) {
    dev_dbg(dev,
    "%s: copy_to_user failed\n", __func__);
    return -EFAULT;
    }
// ppos += len;
    dev_dbg(dev, "%s: %zu bytes copied, %d bytes left\n",
    __func__, len, (int)(depth * 4 - *ppos));
    return len;
    }
#[no_mangle]
unsafe extern "C" fn etb_release(inode: *mut inode, file: *mut file) -> c_int {
    static int etb_release(struct inode *inode, struct file *file)
    {
    struct etb_drvdata *drvdata = container_of(file.private_data,
    struct etb_drvdata, miscdev);
    atomic_set(&drvdata.reading, 0);
    dev_dbg(&drvdata.csdev.dev, "%s: released\n", __func__);
    return 0;
    }
    static const struct file_operations etb_fops = {
    .owner		= THIS_MODULE,
    .open		= etb_open,
    .read		= etb_read,
    .release	= etb_release,
    };
    static struct attribute *coresight_etb_mgmt_attrs[] = {
    coresight_simple_reg32(rdp, ETB_RAM_DEPTH_REG),
    coresight_simple_reg32(sts, ETB_STATUS_REG),
    coresight_simple_reg32(rrp, ETB_RAM_READ_POINTER),
    coresight_simple_reg32(rwp, ETB_RAM_WRITE_POINTER),
    coresight_simple_reg32(trg, ETB_TRG),
    coresight_simple_reg32(ctl, ETB_CTL_REG),
    coresight_simple_reg32(ffsr, ETB_FFSR),
    coresight_simple_reg32(ffcr, ETB_FFCR),
    core::ptr::null_mut(),
    };
    static ssize_t trigger_cntr_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct etb_drvdata *drvdata = dev_get_drvdata(dev.parent);
    let mut val: c_ulong = drvdata.trigger_cntr;
    return sprintf(buf, "%#lx\n", val);
    }
    static ssize_t trigger_cntr_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t size)
    {
    int ret;
    unsigned long val;
    struct etb_drvdata *drvdata = dev_get_drvdata(dev.parent);
    ret = kstrtoul(buf, 16, &val);
    if (ret)
    return ret;
    drvdata.trigger_cntr = val;
    return size;
    }
    static DEVICE_ATTR_RW(trigger_cntr);
    static struct attribute *coresight_etb_attrs[] = {
    &dev_attr_trigger_cntr.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group coresight_etb_group = {
    .attrs = coresight_etb_attrs,
    };
    static const struct attribute_group coresight_etb_mgmt_group = {
    .attrs = coresight_etb_mgmt_attrs,
    .name = "mgmt",
    };
    static const struct attribute_group *coresight_etb_groups[] = {
    &coresight_etb_group,
    &coresight_etb_mgmt_group,
    core::ptr::null_mut(),
    };
#[no_mangle]
unsafe extern "C" fn etb_probe(adev: *mut amba_device, id: *const amba_id) -> c_int {
    static int etb_probe(struct amba_device *adev, const struct amba_id *id)
    {
    int ret;
    void __iomem *base;
    struct device *dev = &adev.dev;
    struct coresight_platform_data *pdata = core::ptr::null_mut();
    struct etb_drvdata *drvdata;
    struct resource *res = &adev.res;
    let mut desc: coresight_desc = { 0 };
    desc.name = coresight_alloc_device_name("etb", dev);
    if (!desc.name)
    return -ENOMEM;
    drvdata = devm_kzalloc(dev, sizeof(*drvdata), GFP_KERNEL);
    if (!drvdata)
    return -ENOMEM;
    drvdata.atclk = devm_clk_get_optional_enabled(dev, "atclk");
    if (IS_ERR(drvdata.atclk))
    return PTR_ERR(drvdata.atclk);
    dev_set_drvdata(dev, drvdata);
// validity for the resource is already checked by the AMBA core
    base = devm_ioremap_resource(dev, res);
    if (IS_ERR(base))
    return PTR_ERR(base);
    drvdata.base = base;
    desc.access = CSDEV_ACCESS_IOMEM(base);
    raw_spin_lock_init(&drvdata.spinlock);
    drvdata.buffer_depth = etb_get_buffer_depth(drvdata);
    if (drvdata.buffer_depth & 0x80000000)
    return -EINVAL;
    drvdata.buf = devm_kcalloc(dev,
    drvdata.buffer_depth, 4, GFP_KERNEL);
    if (!drvdata.buf)
    return -ENOMEM;
// This device is not associated with a session
    drvdata.pid = -1;
    pdata = coresight_get_platform_data(dev);
    if (IS_ERR(pdata))
    return PTR_ERR(pdata);
    adev.dev.platform_data = pdata;
    desc.type = CORESIGHT_DEV_TYPE_SINK;
    desc.subtype.sink_subtype = CORESIGHT_DEV_SUBTYPE_SINK_BUFFER;
    desc.ops = &etb_cs_ops;
    desc.pdata = pdata;
    desc.dev = dev;
    desc.groups = coresight_etb_groups;
    coresight_clear_self_claim_tag(&desc.access);
    drvdata.csdev = coresight_register(&desc);
    if (IS_ERR(drvdata.csdev))
    return PTR_ERR(drvdata.csdev);
    drvdata.miscdev.name = desc.name;
    drvdata.miscdev.minor = MISC_DYNAMIC_MINOR;
    drvdata.miscdev.fops = &etb_fops;
    ret = misc_register(&drvdata.miscdev);
    if (ret)
    goto err_misc_register;
    pm_runtime_put(&adev.dev);
    return 0;
    err_misc_register:
    coresight_unregister(drvdata.csdev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn etb_remove(adev: *mut amba_device) {
    static void etb_remove(struct amba_device *adev)
    {
    struct etb_drvdata *drvdata = dev_get_drvdata(&adev.dev);
//
// Since misc_open() holds a refcount on the f_ops, which is
// etb fops in this case, device is there until last file
// handler to this device is closed.
//
    misc_deregister(&drvdata.miscdev);
    coresight_unregister(drvdata.csdev);
    }

#[no_mangle]
unsafe extern "C" fn etb_runtime_suspend(dev: *mut device) -> c_int {
    static int etb_runtime_suspend(struct device *dev)
    {
    struct etb_drvdata *drvdata = dev_get_drvdata(dev);
    clk_disable_unprepare(drvdata.atclk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn etb_runtime_resume(dev: *mut device) -> c_int {
    static int etb_runtime_resume(struct device *dev)
    {
    struct etb_drvdata *drvdata = dev_get_drvdata(dev);
    return clk_prepare_enable(drvdata.atclk);
    }

    static const struct dev_pm_ops etb_dev_pm_ops = {
    SET_RUNTIME_PM_OPS(etb_runtime_suspend, etb_runtime_resume, core::ptr::null_mut())
    };
    static const struct amba_id etb_ids[] = {
    {
    .id	= 0x000bb907,
    .mask	= 0x000fffff,
    },
    { 0, 0, core::ptr::null_mut() },
    };
    MODULE_DEVICE_TABLE(amba, etb_ids);
    static struct amba_driver etb_driver = {
    .drv = {
    .name	= "coresight-etb10",
    .pm	= &etb_dev_pm_ops,
    .suppress_bind_attrs = true,
    },
    .probe		= etb_probe,
    .remove		= etb_remove,
    .id_table	= etb_ids,
    };
    module_amba_driver(etb_driver);
    MODULE_AUTHOR("Pratik Patel <pratikp@codeaurora.org>");
    MODULE_AUTHOR("Mathieu Poirier <mathieu.poirier@linaro.org>");
    MODULE_DESCRIPTION("Arm CoreSight Embedded Trace Buffer driver");
    MODULE_LICENSE("GPL v2");
