//! Automatically rewritten from C to Rust
//! Source: drivers/hwtracing/coresight/coresight-stm.c
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
// Copyright (c) 2015-2016, The Linux Foundation. All rights reserved.
//
// Description: CoreSight System Trace Macrocell driver
//
// Initial implementation by Pratik Patel
// (C) 2014-2015 Pratik Patel <pratikp@codeaurora.org>
//
// Serious refactoring, code cleanup and upgrading to the Coresight upstream
// framework by Mathieu Poirier
// (C) 2015-2016 Mathieu Poirier <mathieu.poirier@linaro.org>
//
// Guaranteed timing and support for various packet type coming from the
// generic STM API by Chunyan Zhang
// (C) 2015-2016 Chunyan Zhang <zhang.chunyan@linaro.org>
//

pub const STMDMASTARTR: c_uint = 0xc04;
pub const STMDMASTOPR: c_uint = 0xc08;
pub const STMDMASTATR: c_uint = 0xc0c;
pub const STMDMACTLR: c_uint = 0xc10;
pub const STMDMAIDR: c_uint = 0xcfc;
pub const STMHEER: c_uint = 0xd00;
pub const STMHETER: c_uint = 0xd20;
pub const STMHEBSR: c_uint = 0xd60;
pub const STMHEMCR: c_uint = 0xd64;
pub const STMHEMASTR: c_uint = 0xdf4;
pub const STMHEFEAT1R: c_uint = 0xdf8;
pub const STMHEIDR: c_uint = 0xdfc;
pub const STMSPER: c_uint = 0xe00;
pub const STMSPTER: c_uint = 0xe20;
pub const STMPRIVMASKR: c_uint = 0xe40;
pub const STMSPSCR: c_uint = 0xe60;
pub const STMSPMSCR: c_uint = 0xe64;
pub const STMSPOVERRIDER: c_uint = 0xe68;
pub const STMSPMOVERRIDER: c_uint = 0xe6c;
pub const STMSPTRIGCSR: c_uint = 0xe70;
pub const STMTCSR: c_uint = 0xe80;
pub const STMTSSTIMR: c_uint = 0xe84;
pub const STMTSFREQR: c_uint = 0xe8c;
pub const STMSYNCR: c_uint = 0xe90;
pub const STMAUXCR: c_uint = 0xe94;
pub const STMSPFEAT1R: c_uint = 0xea0;
pub const STMSPFEAT2R: c_uint = 0xea4;
pub const STMSPFEAT3R: c_uint = 0xea8;
pub const STMITTRIGGER: c_uint = 0xee8;
pub const STMITATBDATA0: c_uint = 0xeec;
pub const STMITATBCTR2: c_uint = 0xef0;
pub const STMITATBID: c_uint = 0xef4;
pub const STMITATBCTR0: c_uint = 0xef8;
pub const STM_32_CHANNEL: c_int = 32;
pub const BYTES_PER_CHANNEL: c_int = 256;
pub const STM_TRACE_BUF_SIZE: c_int = 4096;
pub const STM_SW_MASTER_END: c_int = 127;
// Register bit definition
pub const STMTCSR_BUSY_BIT: c_int = 23;
// Reserve the first 10 channels for kernel usage
pub const STM_CHANNEL_OFFSET: c_int = 0;
    enum stm_pkt_type {
    STM_PKT_TYPE_DATA	= 0x98,
    STM_PKT_TYPE_FLAG	= 0xE8,
    STM_PKT_TYPE_TRIG	= 0xF8,
    };

    (ch * BYTES_PER_CHANNEL))

    static int boot_nr_channel;
//
// Not really modular but using module_param is the easiest way to
// remain consistent with existing use cases for now.
//
    module_param_named(
    boot_nr_channel, boot_nr_channel, int, S_IRUGO
    );
//
// struct channel_space - central management entity for extended ports
// @base:		memory mapped base address where channels start.
// @phys:		physical base address of channel region.
// @guaraneed:		is the channel delivery guaranteed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct channel_space {
    pub base: *mut void __iomem,
    pub phys: phys_addr_t,
    pub guaranteed: *mut c_ulong,
}

//
// struct stm_drvdata - specifics associated to an STM component
// @base:		memory mapped base address for this component.
// @atclk:		optional clock for the core parts of the STM.
// @pclk:		APB clock if present, otherwise NULL
// @csdev:		component vitals needed by the framework.
// @spinlock:		only one at a time pls.
// @chs:		the channels accociated to this STM.
// @stm:		structure associated to the generic STM interface.
// @traceid:		value of the current ID for this component.
// @write_bytes:	Maximus bytes this STM can write at a time.
// @stmsper:		settings for register STMSPER.
// @stmspscr:		settings for register STMSPSCR.
// @numsp:		the total number of stimulus port support by this STM.
// @stmheer:		settings for register STMHEER.
// @stmheter:		settings for register STMHETER.
// @stmhebsr:		settings for register STMHEBSR.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stm_drvdata {
    pub base: *mut void __iomem,
    pub atclk: *mut clk,
    pub pclk: *mut clk,
    pub csdev: *mut coresight_device,
    pub spinlock: spinlock_t,
    pub chs: channel_space,
    pub stm: stm_data,
    pub traceid: u8,
    pub write_bytes: u32,
    pub stmsper: u32,
    pub stmspscr: u32,
    pub numsp: u32,
    pub stmheer: u32,
    pub stmheter: u32,
    pub stmhebsr: u32,
}

#[no_mangle]
unsafe extern "C" fn stm_hwevent_enable_hw(drvdata: *mut stm_drvdata) {
    static void stm_hwevent_enable_hw(struct stm_drvdata *drvdata)
    {
    CS_UNLOCK(drvdata.base);
    writel_relaxed(drvdata.stmhebsr, drvdata.base + STMHEBSR);
    writel_relaxed(drvdata.stmheter, drvdata.base + STMHETER);
    writel_relaxed(drvdata.stmheer, drvdata.base + STMHEER);
    writel_relaxed(0x01 |	/* Enable HW event tracing */
    0x04,	/* Error detection on event tracing */
    drvdata.base + STMHEMCR);
    CS_LOCK(drvdata.base);
    }
#[no_mangle]
unsafe extern "C" fn stm_port_enable_hw(drvdata: *mut stm_drvdata) {
    static void stm_port_enable_hw(struct stm_drvdata *drvdata)
    {
    CS_UNLOCK(drvdata.base);
// ATB trigger enable on direct writes to TRIG locations
    writel_relaxed(0x10,
    drvdata.base + STMSPTRIGCSR);
    writel_relaxed(drvdata.stmspscr, drvdata.base + STMSPSCR);
    writel_relaxed(drvdata.stmsper, drvdata.base + STMSPER);
    CS_LOCK(drvdata.base);
    }
#[no_mangle]
unsafe extern "C" fn stm_enable_hw(drvdata: *mut stm_drvdata) {
    static void stm_enable_hw(struct stm_drvdata *drvdata)
    {
    if (drvdata.stmheer)
    stm_hwevent_enable_hw(drvdata);
    stm_port_enable_hw(drvdata);
    CS_UNLOCK(drvdata.base);
// 4096 byte between synchronisation packets
    writel_relaxed(0xFFF, drvdata.base + STMSYNCR);
    writel_relaxed((drvdata.traceid << 16 | /* trace id */
    0x02 |			 /* timestamp enable */
    0x01),			 /* global STM enable */
    drvdata.base + STMTCSR);
    CS_LOCK(drvdata.base);
    }
    static int stm_enable(struct coresight_device *csdev, struct perf_event *event,
    enum cs_mode mode,
    __maybe_unused struct coresight_path *path)
    {
    struct stm_drvdata *drvdata = dev_get_drvdata(csdev.dev.parent);
    if (mode != CS_MODE_SYSFS)
    return -EINVAL;
    if (!coresight_take_mode(csdev, mode)) {
// Someone is already using the tracer
    return -EBUSY;
    }
    pm_runtime_get_sync(csdev.dev.parent);
    spin_lock(&drvdata.spinlock);
    stm_enable_hw(drvdata);
    spin_unlock(&drvdata.spinlock);
    dev_dbg(&csdev.dev, "STM tracing enabled\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm_hwevent_disable_hw(drvdata: *mut stm_drvdata) {
    static void stm_hwevent_disable_hw(struct stm_drvdata *drvdata)
    {
    CS_UNLOCK(drvdata.base);
    writel_relaxed(0x0, drvdata.base + STMHEMCR);
    writel_relaxed(0x0, drvdata.base + STMHEER);
    writel_relaxed(0x0, drvdata.base + STMHETER);
    CS_LOCK(drvdata.base);
    }
#[no_mangle]
unsafe extern "C" fn stm_port_disable_hw(drvdata: *mut stm_drvdata) {
    static void stm_port_disable_hw(struct stm_drvdata *drvdata)
    {
    CS_UNLOCK(drvdata.base);
    writel_relaxed(0x0, drvdata.base + STMSPER);
    writel_relaxed(0x0, drvdata.base + STMSPTRIGCSR);
    CS_LOCK(drvdata.base);
    }
#[no_mangle]
unsafe extern "C" fn stm_disable_hw(drvdata: *mut stm_drvdata) {
    static void stm_disable_hw(struct stm_drvdata *drvdata)
    {
    u32 val;
    CS_UNLOCK(drvdata.base);
    val = readl_relaxed(drvdata.base + STMTCSR);
    val &= ~0x1; /* clear global STM enable [0] */
    writel_relaxed(val, drvdata.base + STMTCSR);
    CS_LOCK(drvdata.base);
    stm_port_disable_hw(drvdata);
    if (drvdata.stmheer)
    stm_hwevent_disable_hw(drvdata);
    }
    static void stm_disable(struct coresight_device *csdev,
    struct perf_event *event)
    {
    struct stm_drvdata *drvdata = dev_get_drvdata(csdev.dev.parent);
    struct csdev_access *csa = &csdev.access;
//
// For as long as the tracer isn't disabled another entity can't
// change its status.  As such we can read the status here without
// fearing it will change under us.
//
    if (coresight_get_mode(csdev) == CS_MODE_SYSFS) {
    spin_lock(&drvdata.spinlock);
    stm_disable_hw(drvdata);
    spin_unlock(&drvdata.spinlock);
// Wait until the engine has completely stopped
    coresight_timeout(csa, STMTCSR, STMTCSR_BUSY_BIT, 0);
    pm_runtime_put(csdev.dev.parent);
    coresight_set_mode(csdev, CS_MODE_DISABLED);
    dev_dbg(&csdev.dev, "STM tracing disabled\n");
    }
    }
    static int stm_trace_id(struct coresight_device *csdev, __maybe_unused enum cs_mode mode,
    __maybe_unused struct coresight_device *sink)
    {
    struct stm_drvdata *drvdata;
    drvdata = dev_get_drvdata(csdev.dev.parent);
    return drvdata.traceid;
    }
    static const struct coresight_ops_source stm_source_ops = {
    .enable		= stm_enable,
    .disable	= stm_disable,
    };
    static const struct coresight_ops stm_cs_ops = {
    .trace_id	= stm_trace_id,
    .source_ops	= &stm_source_ops,
    };
#[no_mangle]
unsafe extern "C" fn stm_addr_unaligned(addr: *const c_void, write_bytes: u8) -> bool {
    static bool stm_addr_unaligned(const void *addr, u8 write_bytes)
    {
    return ((unsigned long)addr & (write_bytes - 1));
    }
    static void stm_send(void __iomem *addr, const void *data,
    u32 size, u8 write_bytes)
    {
    u8 paload[8];
    if (stm_addr_unaligned(data, write_bytes)) {
    memcpy(paload, data, size);
    data = paload;
    }
// now we are 64bit/32bit aligned
    switch (size) {

    case 8:
    writeq_relaxed(*(u64 *)data, addr);
    break;

    case 4:
    writel_relaxed(*(u32 *)data, addr);
    break;
    case 2:
    writew_relaxed(*(u16 *)data, addr);
    break;
    case 1:
    writeb_relaxed(*(u8 *)data, addr);
    break;
    default:
    break;
    }
    }
    static int stm_generic_link(struct stm_data *stm_data,
    unsigned int master,  unsigned int channel)
    {
    struct stm_drvdata *drvdata = container_of(stm_data,
    struct stm_drvdata, stm);
    if (!drvdata.csdev)
    return -EINVAL;
    return coresight_enable_sysfs(drvdata.csdev);
    }
    static void stm_generic_unlink(struct stm_data *stm_data,
    unsigned int master,  unsigned int channel)
    {
    struct stm_drvdata *drvdata = container_of(stm_data,
    struct stm_drvdata, stm);
    if (!drvdata.csdev)
    return;
    coresight_disable_sysfs(drvdata.csdev);
    }
    static phys_addr_t
    stm_mmio_addr(struct stm_data *stm_data, unsigned int master,
    unsigned int channel, unsigned int nr_chans)
    {
    struct stm_drvdata *drvdata = container_of(stm_data,
    struct stm_drvdata, stm);
    phys_addr_t addr;
    addr = drvdata.chs.phys + channel * BYTES_PER_CHANNEL;
    if (offset_in_page(addr) ||
    offset_in_page(nr_chans * BYTES_PER_CHANNEL))
    return 0;
    return addr;
    }
    static long stm_generic_set_options(struct stm_data *stm_data,
    unsigned int master,
    unsigned int channel,
    unsigned int nr_chans,
    unsigned long options)
    {
    struct stm_drvdata *drvdata = container_of(stm_data,
    struct stm_drvdata, stm);
    if (!coresight_get_mode(drvdata.csdev))
    return -EINVAL;
    if (channel >= drvdata.numsp)
    return -EINVAL;
    switch (options) {
    case STM_OPTION_GUARANTEED:
    set_bit(channel, drvdata.chs.guaranteed);
    break;
    case STM_OPTION_INVARIANT:
    clear_bit(channel, drvdata.chs.guaranteed);
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    static ssize_t notrace stm_generic_packet(struct stm_data *stm_data,
    unsigned int master,
    unsigned int channel,
    unsigned int packet,
    unsigned int flags,
    unsigned int size,
    const unsigned char *payload)
    {
    void __iomem *ch_addr;
    struct stm_drvdata *drvdata = container_of(stm_data,
    struct stm_drvdata, stm);
    unsigned int stm_flags;
    if (!coresight_get_mode(drvdata.csdev))
    return -EACCES;
    if (channel >= drvdata.numsp)
    return -EINVAL;
    ch_addr = stm_channel_addr(drvdata, channel);
    stm_flags = (flags & STP_PACKET_TIMESTAMPED) ?
    STM_FLAG_TIMESTAMPED : 0;
    stm_flags |= test_bit(channel, drvdata.chs.guaranteed) ?
    STM_FLAG_GUARANTEED : 0;
    if (size > drvdata.write_bytes)
    size = drvdata.write_bytes;
    else
    size = rounddown_pow_of_two(size);
    switch (packet) {
    case STP_PACKET_FLAG:
    ch_addr += stm_channel_off(STM_PKT_TYPE_FLAG, stm_flags);
//
// The generic STM core sets a size of '0' on flag packets.
// As such send a flag packet of size '1' and tell the
// core we did so.
//
    stm_send(ch_addr, payload, 1, drvdata.write_bytes);
    size = 1;
    break;
    case STP_PACKET_DATA:
    stm_flags |= (flags & STP_PACKET_MARKED) ? STM_FLAG_MARKED : 0;
    ch_addr += stm_channel_off(STM_PKT_TYPE_DATA, stm_flags);
    stm_send(ch_addr, payload, size,
    drvdata.write_bytes);
    break;
    default:
    return -ENOTSUPP;
    }
    return size;
    }
    static ssize_t hwevent_enable_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct stm_drvdata *drvdata = dev_get_drvdata(dev.parent);
    let mut val: c_ulong = drvdata.stmheer;
    return scnprintf(buf, PAGE_SIZE, "%#lx\n", val);
    }
    static ssize_t hwevent_enable_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t size)
    {
    struct stm_drvdata *drvdata = dev_get_drvdata(dev.parent);
    unsigned long val;
    let mut ret: c_int = 0;
    ret = kstrtoul(buf, 16, &val);
    if (ret)
    return -EINVAL;
    drvdata.stmheer = val;
// HW event enable and trigger go hand in hand
    drvdata.stmheter = val;
    return size;
    }
    static DEVICE_ATTR_RW(hwevent_enable);
    static ssize_t hwevent_select_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct stm_drvdata *drvdata = dev_get_drvdata(dev.parent);
    let mut val: c_ulong = drvdata.stmhebsr;
    return scnprintf(buf, PAGE_SIZE, "%#lx\n", val);
    }
    static ssize_t hwevent_select_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t size)
    {
    struct stm_drvdata *drvdata = dev_get_drvdata(dev.parent);
    unsigned long val;
    let mut ret: c_int = 0;
    ret = kstrtoul(buf, 16, &val);
    if (ret)
    return -EINVAL;
    drvdata.stmhebsr = val;
    return size;
    }
    static DEVICE_ATTR_RW(hwevent_select);
    static ssize_t port_select_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct stm_drvdata *drvdata = dev_get_drvdata(dev.parent);
    unsigned long val;
    if (!coresight_get_mode(drvdata.csdev)) {
    val = drvdata.stmspscr;
    } else {
    spin_lock(&drvdata.spinlock);
    val = readl_relaxed(drvdata.base + STMSPSCR);
    spin_unlock(&drvdata.spinlock);
    }
    return scnprintf(buf, PAGE_SIZE, "%#lx\n", val);
    }
    static ssize_t port_select_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t size)
    {
    struct stm_drvdata *drvdata = dev_get_drvdata(dev.parent);
    unsigned long val, stmsper;
    let mut ret: c_int = 0;
    ret = kstrtoul(buf, 16, &val);
    if (ret)
    return ret;
    spin_lock(&drvdata.spinlock);
    drvdata.stmspscr = val;
    if (coresight_get_mode(drvdata.csdev)) {
    CS_UNLOCK(drvdata.base);
// Process as per ARM's TRM recommendation
    stmsper = readl_relaxed(drvdata.base + STMSPER);
    writel_relaxed(0x0, drvdata.base + STMSPER);
    writel_relaxed(drvdata.stmspscr, drvdata.base + STMSPSCR);
    writel_relaxed(stmsper, drvdata.base + STMSPER);
    CS_LOCK(drvdata.base);
    }
    spin_unlock(&drvdata.spinlock);
    return size;
    }
    static DEVICE_ATTR_RW(port_select);
    static ssize_t port_enable_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct stm_drvdata *drvdata = dev_get_drvdata(dev.parent);
    unsigned long val;
    if (!coresight_get_mode(drvdata.csdev)) {
    val = drvdata.stmsper;
    } else {
    spin_lock(&drvdata.spinlock);
    val = readl_relaxed(drvdata.base + STMSPER);
    spin_unlock(&drvdata.spinlock);
    }
    return scnprintf(buf, PAGE_SIZE, "%#lx\n", val);
    }
    static ssize_t port_enable_store(struct device *dev,
    struct device_attribute *attr,
    const char *buf, size_t size)
    {
    struct stm_drvdata *drvdata = dev_get_drvdata(dev.parent);
    unsigned long val;
    let mut ret: c_int = 0;
    ret = kstrtoul(buf, 16, &val);
    if (ret)
    return ret;
    spin_lock(&drvdata.spinlock);
    drvdata.stmsper = val;
    if (coresight_get_mode(drvdata.csdev)) {
    CS_UNLOCK(drvdata.base);
    writel_relaxed(drvdata.stmsper, drvdata.base + STMSPER);
    CS_LOCK(drvdata.base);
    }
    spin_unlock(&drvdata.spinlock);
    return size;
    }
    static DEVICE_ATTR_RW(port_enable);
    static ssize_t traceid_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    unsigned long val;
    struct stm_drvdata *drvdata = dev_get_drvdata(dev.parent);
    val = drvdata.traceid;
    return sprintf(buf, "%#lx\n", val);
    }
    static DEVICE_ATTR_RO(traceid);
    static struct attribute *coresight_stm_attrs[] = {
    &dev_attr_hwevent_enable.attr,
    &dev_attr_hwevent_select.attr,
    &dev_attr_port_enable.attr,
    &dev_attr_port_select.attr,
    &dev_attr_traceid.attr,
    core::ptr::null_mut(),
    };
    static struct attribute *coresight_stm_mgmt_attrs[] = {
    coresight_simple_reg32(tcsr, STMTCSR),
    coresight_simple_reg32(tsfreqr, STMTSFREQR),
    coresight_simple_reg32(syncr, STMSYNCR),
    coresight_simple_reg32(sper, STMSPER),
    coresight_simple_reg32(spter, STMSPTER),
    coresight_simple_reg32(privmaskr, STMPRIVMASKR),
    coresight_simple_reg32(spscr, STMSPSCR),
    coresight_simple_reg32(spmscr, STMSPMSCR),
    coresight_simple_reg32(spfeat1r, STMSPFEAT1R),
    coresight_simple_reg32(spfeat2r, STMSPFEAT2R),
    coresight_simple_reg32(spfeat3r, STMSPFEAT3R),
    coresight_simple_reg32(devid, CORESIGHT_DEVID),
    core::ptr::null_mut(),
    };
    static const struct attribute_group coresight_stm_group = {
    .attrs = coresight_stm_attrs,
    };
    static const struct attribute_group coresight_stm_mgmt_group = {
    .attrs = coresight_stm_mgmt_attrs,
    .name = "mgmt",
    };
    static const struct attribute_group *coresight_stm_groups[] = {
    &coresight_stm_group,
    &coresight_stm_mgmt_group,
    core::ptr::null_mut(),
    };

#[no_mangle]
unsafe extern "C" fn of_stm_get_stimulus_area(dev: *mut device, res: *mut resource) -> c_int {
    static int of_stm_get_stimulus_area(struct device *dev, struct resource *res)
    {
    const char *name = core::ptr::null_mut();
    let mut index: c_int = 0, found = 0;
    struct device_node *np = dev.of_node;
    while (!of_property_read_string_index(np, "reg-names", index, &name)) {
    if (strcmp("stm-stimulus-base", name)) {
    index++;
    continue;
    }
// We have a match and @index is where it's at
    found = 1;
    break;
    }
    if (!found)
    return -EINVAL;
    return of_address_to_resource(np, index, res);
    }

    static int of_stm_get_stimulus_area(struct device *dev,
    struct resource *res)
    {
    return -ENOENT;
    }

#[no_mangle]
unsafe extern "C" fn acpi_stm_get_stimulus_area(dev: *mut device, res: *mut resource) -> c_int {
    static int acpi_stm_get_stimulus_area(struct device *dev, struct resource *res)
    {
    int rc;
    let mut found_base: bool = false;
    struct resource_entry *rent;
    LIST_HEAD(res_list);
    struct acpi_device *adev = ACPI_COMPANION(dev);
    rc = acpi_dev_get_resources(adev, &res_list, core::ptr::null_mut(), core::ptr::null_mut());
    if (rc < 0)
    return rc;
//
// The stimulus base for STM device must be listed as the second memory
// resource, followed by the programming base address as described in
// "Section 2.3 Resources" in ACPI for CoreSightTM 1.0 Platform Design
// document (DEN0067).
//
    rc = -ENOENT;
    list_for_each_entry(rent, &res_list, node) {
    if (resource_type(rent.res) != IORESOURCE_MEM)
    continue;
    if (found_base) {
// res = *rent->res;
    rc = 0;
    break;
    }
    found_base = true;
    }
    acpi_dev_free_resource_list(&res_list);
    return rc;
    }

    static int acpi_stm_get_stimulus_area(struct device *dev,
    struct resource *res)
    {
    return -ENOENT;
    }

#[no_mangle]
unsafe extern "C" fn stm_get_stimulus_area(dev: *mut device, res: *mut resource) -> c_int {
    static int stm_get_stimulus_area(struct device *dev, struct resource *res)
    {
    struct fwnode_handle *fwnode = dev_fwnode(dev);
    if (is_of_node(fwnode))
    return of_stm_get_stimulus_area(dev, res);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: is_acpi_node(fwnode)) -> else {
    else if (is_acpi_node(fwnode))
    return acpi_stm_get_stimulus_area(dev, res);
    return -ENOENT;
    }
#[no_mangle]
unsafe extern "C" fn stm_fundamental_data_size(drvdata: *mut stm_drvdata) -> u32 {
    static u32 stm_fundamental_data_size(struct stm_drvdata *drvdata)
    {
    u32 stmspfeat2r;
    if (!IS_ENABLED(CONFIG_64BIT))
    return 4;
    stmspfeat2r = readl_relaxed(drvdata.base + STMSPFEAT2R);
//
// bit[15:12] represents the fundamental data size
// 0 - 32-bit data
// 1 - 64-bit data
//
    return BMVAL(stmspfeat2r, 12, 15) ? 8 : 4;
    }
#[no_mangle]
unsafe extern "C" fn stm_num_stimulus_port(drvdata: *mut stm_drvdata) -> u32 {
    static u32 stm_num_stimulus_port(struct stm_drvdata *drvdata)
    {
    u32 numsp;
    numsp = readl_relaxed(drvdata.base + CORESIGHT_DEVID);
//
// NUMPS in STMDEVID is 17 bit long and if equal to 0x0,
// 32 stimulus ports are supported.
//
    numsp &= 0x1ffff;
    if (!numsp)
    numsp = STM_32_CHANNEL;
    return numsp;
    }
#[no_mangle]
unsafe extern "C" fn stm_init_default_data(drvdata: *mut stm_drvdata) {
    static void stm_init_default_data(struct stm_drvdata *drvdata)
    {
// Don't use port selection
    drvdata.stmspscr = 0x0;
//
// Enable all channel regardless of their number.  When port
// selection isn't used (see above) STMSPER applies to all
// 32 channel group available, hence setting all 32 bits to 1
//
    drvdata.stmsper = ~0x0;
// Set invariant transaction timing on all channels
    bitmap_clear(drvdata.chs.guaranteed, 0, drvdata.numsp);
    }
    static void stm_init_generic_data(struct stm_drvdata *drvdata,
    const char *name)
    {
    drvdata.stm.name = name;
//
// MasterIDs are assigned at HW design phase. As such the core is
// using a single master for interaction with this device.
//
    drvdata.stm.sw_start = 1;
    drvdata.stm.sw_end = 1;
    drvdata.stm.hw_override = true;
    drvdata.stm.sw_nchannels = drvdata.numsp;
    drvdata.stm.sw_mmiosz = BYTES_PER_CHANNEL;
    drvdata.stm.packet = stm_generic_packet;
    drvdata.stm.mmio_addr = stm_mmio_addr;
    drvdata.stm.link = stm_generic_link;
    drvdata.stm.unlink = stm_generic_unlink;
    drvdata.stm.set_options = stm_generic_set_options;
    }
    static const struct amba_id stm_ids[];
    static char *stm_csdev_name(struct coresight_device *csdev)
    {
    let mut stm_pid: u32 = coresight_get_pid(&csdev.access);
    void *uci_data = coresight_get_uci_data_from_amba(stm_ids, stm_pid);
    return uci_data ? (char *)uci_data : "STM";
    }
#[no_mangle]
unsafe extern "C" fn __stm_probe(dev: *mut device, res: *mut resource) -> c_int {
    static int __stm_probe(struct device *dev, struct resource *res)
    {
    int ret, trace_id;
    void __iomem *base;
    struct coresight_platform_data *pdata = core::ptr::null_mut();
    struct stm_drvdata *drvdata;
    struct resource ch_res;
    let mut desc: coresight_desc = { 0 };
    desc.name = coresight_alloc_device_name("stm", dev);
    if (!desc.name)
    return -ENOMEM;
    drvdata = devm_kzalloc(dev, sizeof(*drvdata), GFP_KERNEL);
    if (!drvdata)
    return -ENOMEM;
    ret = coresight_get_enable_clocks(dev, &drvdata.pclk, &drvdata.atclk);
    if (ret)
    return ret;
    dev_set_drvdata(dev, drvdata);
    base = devm_ioremap_resource(dev, res);
    if (IS_ERR(base))
    return PTR_ERR(base);
    drvdata.base = base;
    desc.access = CSDEV_ACCESS_IOMEM(base);
    ret = stm_get_stimulus_area(dev, &ch_res);
    if (ret)
    return ret;
    drvdata.chs.phys = ch_res.start;
    base = devm_ioremap_resource(dev, &ch_res);
    if (IS_ERR(base))
    return PTR_ERR(base);
    drvdata.chs.base = base;
    drvdata.write_bytes = stm_fundamental_data_size(drvdata);
    if (boot_nr_channel)
    drvdata.numsp = boot_nr_channel;
    else
    drvdata.numsp = stm_num_stimulus_port(drvdata);
    drvdata.chs.guaranteed = devm_bitmap_zalloc(dev, drvdata.numsp,
    GFP_KERNEL);
    if (!drvdata.chs.guaranteed)
    return -ENOMEM;
    spin_lock_init(&drvdata.spinlock);
    stm_init_default_data(drvdata);
    stm_init_generic_data(drvdata, desc.name);
    if (stm_register_device(dev, &drvdata.stm, THIS_MODULE)) {
    dev_info(dev,
    "%s : stm_register_device failed, probing deferred\n",
    desc.name);
    return -EPROBE_DEFER;
    }
    pdata = coresight_get_platform_data(dev);
    if (IS_ERR(pdata)) {
    ret = PTR_ERR(pdata);
    goto stm_unregister;
    }
    dev.platform_data = pdata;
    desc.type = CORESIGHT_DEV_TYPE_SOURCE;
    desc.subtype.source_subtype = CORESIGHT_DEV_SUBTYPE_SOURCE_SOFTWARE;
    desc.ops = &stm_cs_ops;
    desc.pdata = pdata;
    desc.dev = dev;
    desc.groups = coresight_stm_groups;
    drvdata.csdev = coresight_register(&desc);
    if (IS_ERR(drvdata.csdev)) {
    ret = PTR_ERR(drvdata.csdev);
    goto stm_unregister;
    }
    trace_id = coresight_trace_id_get_system_id();
    if (trace_id < 0) {
    ret = trace_id;
    goto cs_unregister;
    }
    drvdata.traceid = (u8)trace_id;
    dev_info(&drvdata.csdev.dev, "%s initialized\n",
    stm_csdev_name(drvdata.csdev));
    return 0;
    cs_unregister:
    coresight_unregister(drvdata.csdev);
    stm_unregister:
    stm_unregister_device(&drvdata.stm);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stm_probe(adev: *mut amba_device, id: *const amba_id) -> c_int {
    static int stm_probe(struct amba_device *adev, const struct amba_id *id)
    {
    int ret;
    ret = __stm_probe(&adev.dev, &adev.res);
    if (!ret)
    pm_runtime_put(&adev.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn __stm_remove(dev: *mut device) {
    static void __stm_remove(struct device *dev)
    {
    struct stm_drvdata *drvdata = dev_get_drvdata(dev);
    coresight_trace_id_put_system_id(drvdata.traceid);
    coresight_unregister(drvdata.csdev);
    stm_unregister_device(&drvdata.stm);
    }
#[no_mangle]
unsafe extern "C" fn stm_remove(adev: *mut amba_device) {
    static void stm_remove(struct amba_device *adev)
    {
    __stm_remove(&adev.dev);
    }

#[no_mangle]
unsafe extern "C" fn stm_runtime_suspend(dev: *mut device) -> c_int {
    static int stm_runtime_suspend(struct device *dev)
    {
    struct stm_drvdata *drvdata = dev_get_drvdata(dev);
    clk_disable_unprepare(drvdata.atclk);
    clk_disable_unprepare(drvdata.pclk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn stm_runtime_resume(dev: *mut device) -> c_int {
    static int stm_runtime_resume(struct device *dev)
    {
    struct stm_drvdata *drvdata = dev_get_drvdata(dev);
    int ret;
    ret = clk_prepare_enable(drvdata.pclk);
    if (ret)
    return ret;
    ret = clk_prepare_enable(drvdata.atclk);
    if (ret)
    clk_disable_unprepare(drvdata.pclk);
    return ret;
    }

    static const struct dev_pm_ops stm_dev_pm_ops = {
    SET_RUNTIME_PM_OPS(stm_runtime_suspend, stm_runtime_resume, core::ptr::null_mut())
    };
    static const struct amba_id stm_ids[] = {
    CS_AMBA_ID_DATA(0x000bb962, "STM32"),
    CS_AMBA_ID_DATA(0x000bb963, "STM500"),
    { 0, 0, core::ptr::null_mut() },
    };
    MODULE_DEVICE_TABLE(amba, stm_ids);
    static struct amba_driver stm_driver = {
    .drv = {
    .name   = "coresight-stm",
    .pm	= &stm_dev_pm_ops,
    .suppress_bind_attrs = true,
    },
    .probe          = stm_probe,
    .remove         = stm_remove,
    .id_table	= stm_ids,
    };
#[no_mangle]
unsafe extern "C" fn stm_platform_probe(pdev: *mut platform_device) -> c_int {
    static int stm_platform_probe(struct platform_device *pdev)
    {
    struct resource *res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    let mut ret: c_int = 0;
    pm_runtime_get_noresume(&pdev.dev);
    pm_runtime_set_active(&pdev.dev);
    pm_runtime_enable(&pdev.dev);
    ret = __stm_probe(&pdev.dev, res);
    pm_runtime_put(&pdev.dev);
    if (ret)
    pm_runtime_disable(&pdev.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn stm_platform_remove(pdev: *mut platform_device) {
    static void stm_platform_remove(struct platform_device *pdev)
    {
    struct stm_drvdata *drvdata = dev_get_drvdata(&pdev.dev);
    if (WARN_ON(!drvdata))
    return;
    __stm_remove(&pdev.dev);
    pm_runtime_disable(&pdev.dev);
    }

    static const struct acpi_device_id stm_acpi_ids[] = {
    {"ARMHC502", 0, 0, 0}, /* ARM CoreSight STM */
    {},
    };
    MODULE_DEVICE_TABLE(acpi, stm_acpi_ids);

    static struct platform_driver stm_platform_driver = {
    .probe	= stm_platform_probe,
    .remove = stm_platform_remove,
    .driver	= {
    .name			= "coresight-stm-platform",
    .acpi_match_table	= ACPI_PTR(stm_acpi_ids),
    .suppress_bind_attrs	= true,
    .pm			= &stm_dev_pm_ops,
    },
    };
#[no_mangle]
unsafe extern "C" fn stm_init() -> int __init {
    static int __init stm_init(void)
    {
    return coresight_init_driver("stm", &stm_driver, &stm_platform_driver);
    }
#[no_mangle]
unsafe extern "C" fn stm_exit() -> void __exit {
    static void __exit stm_exit(void)
    {
    coresight_remove_driver(&stm_driver, &stm_platform_driver);
    }
    module_init(stm_init);
    module_exit(stm_exit);
    MODULE_AUTHOR("Pratik Patel <pratikp@codeaurora.org>");
    MODULE_DESCRIPTION("Arm CoreSight System Trace Macrocell driver");
    MODULE_LICENSE("GPL v2");
