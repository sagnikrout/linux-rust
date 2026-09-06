//! Automatically rewritten from C to Rust
//! Source: drivers/w1/masters/omap_hdq.c
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
// Copyright (C) 2007,2012 Texas Instruments, Inc.
//

pub const OMAP_HDQ_REVISION: c_uint = 0x00;
pub const OMAP_HDQ_TX_DATA: c_uint = 0x04;
pub const OMAP_HDQ_RX_DATA: c_uint = 0x08;
pub const OMAP_HDQ_CTRL_STATUS: c_uint = 0x0c;

pub const OMAP_HDQ_INT_STATUS: c_uint = 0x10;

pub const OMAP_HDQ_FLAG_CLEAR: c_int = 0;
pub const OMAP_HDQ_FLAG_SET: c_int = 1;

pub const OMAP_HDQ_MAX_USER: c_int = 4;
    static DECLARE_WAIT_QUEUE_HEAD(hdq_wait_queue);
    static int w1_id;
    module_param(w1_id, int, 0400);
    MODULE_PARM_DESC(w1_id, "1-wire id for the slave detection in HDQ mode");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdq_data {
    pub dev: *mut device,
    pub hdq_base: *mut void __iomem,
// lock read/write/break operations
    pub hdq_mutex: mutex,
// interrupt status and a lock for it
    pub hdq_irqstatus: u8,
    pub hdq_spinlock: spinlock_t,
// mode: 0-HDQ 1-W1
    pub mode: c_int,
}

// HDQ register I/O routines
#[no_mangle]
pub unsafe extern "C" fn hdq_reg_in(hdq_data: *mut hdq_data, offset: u32) -> u8 {
    static inline u8 hdq_reg_in(struct hdq_data *hdq_data, u32 offset)
    {
    return __raw_readl(hdq_data.hdq_base + offset);
    }
#[no_mangle]
pub unsafe extern "C" fn hdq_reg_out(hdq_data: *mut hdq_data, offset: u32, val: u8) {
    static inline void hdq_reg_out(struct hdq_data *hdq_data, u32 offset, u8 val)
    {
    __raw_writel(val, hdq_data.hdq_base + offset);
    }
    static inline u8 hdq_reg_merge(struct hdq_data *hdq_data, u32 offset,
    u8 val, u8 mask)
    {
    u8 new_val = (__raw_readl(hdq_data.hdq_base + offset) & ~mask)
    | (val & mask);
    __raw_writel(new_val, hdq_data.hdq_base + offset);
    return new_val;
    }
//
// Wait for one or more bits in flag change.
// HDQ_FLAG_SET: wait until any bit in the flag is set.
// HDQ_FLAG_CLEAR: wait until all bits in the flag are cleared.
// return 0 on success and -ETIMEDOUT in the case of timeout.
//
    static int hdq_wait_for_flag(struct hdq_data *hdq_data, u32 offset,
    u8 flag, u8 flag_set, u8 *status)
    {
    let mut ret: c_int = 0;
    let mut timeout: c_ulong = jiffies + OMAP_HDQ_TIMEOUT;
    if (flag_set == OMAP_HDQ_FLAG_CLEAR) {
// wait for the flag clear
    while (((*status = hdq_reg_in(hdq_data, offset)) & flag)
    && time_before(jiffies, timeout)) {
    schedule_timeout_uninterruptible(1);
    }
    if (*status & flag)
    ret = -ETIMEDOUT;
    } else if (flag_set == OMAP_HDQ_FLAG_SET) {
// wait for the flag set
    while (!((*status = hdq_reg_in(hdq_data, offset)) & flag)
    && time_before(jiffies, timeout)) {
    schedule_timeout_uninterruptible(1);
    }
    if (!(*status & flag))
    ret = -ETIMEDOUT;
    } else
    return -EINVAL;
    return ret;
    }
// Clear saved irqstatus after using an interrupt
#[no_mangle]
unsafe extern "C" fn hdq_reset_irqstatus(hdq_data: *mut hdq_data, bits: u8) -> u8 {
    static u8 hdq_reset_irqstatus(struct hdq_data *hdq_data, u8 bits)
    {
    unsigned long irqflags;
    u8 status;
    spin_lock_irqsave(&hdq_data.hdq_spinlock, irqflags);
    status = hdq_data.hdq_irqstatus;
// this is a read-modify-write
    hdq_data.hdq_irqstatus &= ~bits;
    spin_unlock_irqrestore(&hdq_data.hdq_spinlock, irqflags);
    return status;
    }
// write out a byte and fill *status with HDQ_INT_STATUS
#[no_mangle]
unsafe extern "C" fn hdq_write_byte(hdq_data: *mut hdq_data, val: u8, status: *mut u8) -> c_int {
    static int hdq_write_byte(struct hdq_data *hdq_data, u8 val, u8 *status)
    {
    int ret;
    u8 tmp_status;
    ret = mutex_lock_interruptible(&hdq_data.hdq_mutex);
    if (ret < 0) {
    ret = -EINTR;
    goto rtn;
    }
    if (hdq_data.hdq_irqstatus)
    dev_err(hdq_data.dev, "TX irqstatus not cleared (%02x)\n",
    hdq_data.hdq_irqstatus);
// status = 0;
    hdq_reg_out(hdq_data, OMAP_HDQ_TX_DATA, val);
// set the GO bit
    hdq_reg_merge(hdq_data, OMAP_HDQ_CTRL_STATUS, OMAP_HDQ_CTRL_STATUS_GO,
    OMAP_HDQ_CTRL_STATUS_DIR | OMAP_HDQ_CTRL_STATUS_GO);
// wait for the TXCOMPLETE bit
    ret = wait_event_timeout(hdq_wait_queue,
    (hdq_data.hdq_irqstatus & OMAP_HDQ_INT_STATUS_TXCOMPLETE),
    OMAP_HDQ_TIMEOUT);
// status = hdq_reset_irqstatus(hdq_data, OMAP_HDQ_INT_STATUS_TXCOMPLETE);
    if (ret == 0) {
    dev_dbg(hdq_data.dev, "TX wait elapsed\n");
    ret = -ETIMEDOUT;
    goto out;
    }
// check irqstatus
    if (!(*status & OMAP_HDQ_INT_STATUS_TXCOMPLETE)) {
    dev_dbg(hdq_data.dev, "timeout waiting for"
    " TXCOMPLETE/RXCOMPLETE, %x\n", *status);
    ret = -ETIMEDOUT;
    goto out;
    }
// wait for the GO bit return to zero
    ret = hdq_wait_for_flag(hdq_data, OMAP_HDQ_CTRL_STATUS,
    OMAP_HDQ_CTRL_STATUS_GO,
    OMAP_HDQ_FLAG_CLEAR, &tmp_status);
    if (ret) {
    dev_dbg(hdq_data.dev, "timeout waiting GO bit"
    " return to zero, %x\n", tmp_status);
    }
    out:
    mutex_unlock(&hdq_data.hdq_mutex);
    rtn:
    return ret;
    }
// HDQ Interrupt service routine
#[no_mangle]
unsafe extern "C" fn hdq_isr(irq: c_int, _hdq: *mut c_void) -> irqreturn_t {
    static irqreturn_t hdq_isr(int irq, void *_hdq)
    {
    struct hdq_data *hdq_data = _hdq;
    unsigned long irqflags;
    spin_lock_irqsave(&hdq_data.hdq_spinlock, irqflags);
    hdq_data.hdq_irqstatus |= hdq_reg_in(hdq_data, OMAP_HDQ_INT_STATUS);
    spin_unlock_irqrestore(&hdq_data.hdq_spinlock, irqflags);
    dev_dbg(hdq_data.dev, "hdq_isr: %x\n", hdq_data.hdq_irqstatus);
    if (hdq_data.hdq_irqstatus &
    (OMAP_HDQ_INT_STATUS_TXCOMPLETE | OMAP_HDQ_INT_STATUS_RXCOMPLETE
    | OMAP_HDQ_INT_STATUS_TIMEOUT)) {
// wake up sleeping process
    wake_up(&hdq_wait_queue);
    }
    return IRQ_HANDLED;
    }
// W1 search callback function  in HDQ mode
    static void omap_w1_search_bus(void *_hdq, struct w1_master *master_dev,
    u8 search_type, w1_slave_found_callback slave_found)
    {
    u64 module_id, rn_le, cs, id;
    if (w1_id)
    module_id = w1_id;
    else
    module_id = 0x1;
    rn_le = cpu_to_le64(module_id);
//
// HDQ might not obey truly the 1-wire spec.
// So calculate CRC based on module parameter.
//
    cs = w1_calc_crc8((u8 *)&rn_le, 7);
    id = (cs << 56) | module_id;
    slave_found(master_dev, id);
    }
// Issue break pulse to the device
#[no_mangle]
unsafe extern "C" fn omap_hdq_break(hdq_data: *mut hdq_data) -> c_int {
    static int omap_hdq_break(struct hdq_data *hdq_data)
    {
    let mut ret: c_int = 0;
    u8 tmp_status;
    ret = mutex_lock_interruptible(&hdq_data.hdq_mutex);
    if (ret < 0) {
    dev_dbg(hdq_data.dev, "Could not acquire mutex\n");
    ret = -EINTR;
    goto rtn;
    }
    if (hdq_data.hdq_irqstatus)
    dev_err(hdq_data.dev, "break irqstatus not cleared (%02x)\n",
    hdq_data.hdq_irqstatus);
// set the INIT and GO bit
    hdq_reg_merge(hdq_data, OMAP_HDQ_CTRL_STATUS,
    OMAP_HDQ_CTRL_STATUS_INITIALIZATION | OMAP_HDQ_CTRL_STATUS_GO,
    OMAP_HDQ_CTRL_STATUS_DIR | OMAP_HDQ_CTRL_STATUS_INITIALIZATION |
    OMAP_HDQ_CTRL_STATUS_GO);
// wait for the TIMEOUT bit
    ret = wait_event_timeout(hdq_wait_queue,
    (hdq_data.hdq_irqstatus & OMAP_HDQ_INT_STATUS_TIMEOUT),
    OMAP_HDQ_TIMEOUT);
    tmp_status = hdq_reset_irqstatus(hdq_data, OMAP_HDQ_INT_STATUS_TIMEOUT);
    if (ret == 0) {
    dev_dbg(hdq_data.dev, "break wait elapsed\n");
    ret = -EINTR;
    goto out;
    }
// check irqstatus
    if (!(tmp_status & OMAP_HDQ_INT_STATUS_TIMEOUT)) {
    dev_dbg(hdq_data.dev, "timeout waiting for TIMEOUT, %x\n",
    tmp_status);
    ret = -ETIMEDOUT;
    goto out;
    }
//
// check for the presence detect bit to get
// set to show that the slave is responding
//
    if (!(hdq_reg_in(hdq_data, OMAP_HDQ_CTRL_STATUS) &
    OMAP_HDQ_CTRL_STATUS_PRESENCE)) {
    dev_dbg(hdq_data.dev, "Presence bit not set\n");
    ret = -ETIMEDOUT;
    goto out;
    }
//
// wait for both INIT and GO bits rerurn to zero.
// zero wait time expected for interrupt mode.
//
    ret = hdq_wait_for_flag(hdq_data, OMAP_HDQ_CTRL_STATUS,
    OMAP_HDQ_CTRL_STATUS_INITIALIZATION |
    OMAP_HDQ_CTRL_STATUS_GO, OMAP_HDQ_FLAG_CLEAR,
    &tmp_status);
    if (ret)
    dev_dbg(hdq_data.dev, "timeout waiting INIT&GO bits"
    " return to zero, %x\n", tmp_status);
    out:
    mutex_unlock(&hdq_data.hdq_mutex);
    rtn:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hdq_read_byte(hdq_data: *mut hdq_data, val: *mut u8) -> c_int {
    static int hdq_read_byte(struct hdq_data *hdq_data, u8 *val)
    {
    let mut ret: c_int = 0;
    u8 status;
    ret = mutex_lock_interruptible(&hdq_data.hdq_mutex);
    if (ret < 0) {
    ret = -EINTR;
    goto rtn;
    }
    if (pm_runtime_suspended(hdq_data.dev)) {
    ret = -EINVAL;
    goto out;
    }
    if (!(hdq_data.hdq_irqstatus & OMAP_HDQ_INT_STATUS_RXCOMPLETE)) {
    hdq_reg_merge(hdq_data, OMAP_HDQ_CTRL_STATUS,
    OMAP_HDQ_CTRL_STATUS_DIR | OMAP_HDQ_CTRL_STATUS_GO,
    OMAP_HDQ_CTRL_STATUS_DIR | OMAP_HDQ_CTRL_STATUS_GO);
//
// The RX comes immediately after TX.
//
    wait_event_timeout(hdq_wait_queue,
    (hdq_data.hdq_irqstatus
    & (OMAP_HDQ_INT_STATUS_RXCOMPLETE |
    OMAP_HDQ_INT_STATUS_TIMEOUT)),
    OMAP_HDQ_TIMEOUT);
    status = hdq_reset_irqstatus(hdq_data,
    OMAP_HDQ_INT_STATUS_RXCOMPLETE |
    OMAP_HDQ_INT_STATUS_TIMEOUT);
    hdq_reg_merge(hdq_data, OMAP_HDQ_CTRL_STATUS, 0,
    OMAP_HDQ_CTRL_STATUS_DIR);
// check irqstatus
    if (!(status & OMAP_HDQ_INT_STATUS_RXCOMPLETE)) {
    dev_dbg(hdq_data.dev, "timeout waiting for"
    " RXCOMPLETE, %x", status);
    ret = -ETIMEDOUT;
    goto out;
    }
    } else { /* interrupt had occurred before hdq_read_byte was called */
    hdq_reset_irqstatus(hdq_data, OMAP_HDQ_INT_STATUS_RXCOMPLETE);
    }
// the data is ready. Read it in!
// val = hdq_reg_in(hdq_data, OMAP_HDQ_RX_DATA);
    out:
    mutex_unlock(&hdq_data.hdq_mutex);
    rtn:
    return ret;
    }
//
// W1 triplet callback function - used for searching ROM addresses.
// Registered only when controller is in 1-wire mode.
//
#[no_mangle]
unsafe extern "C" fn omap_w1_triplet(_hdq: *mut c_void, bdir: u8) -> u8 {
    static u8 omap_w1_triplet(void *_hdq, u8 bdir)
    {
    u8 id_bit, comp_bit;
    int err;
    u8 ret = 0x3; /* no slaves responded */
    struct hdq_data *hdq_data = _hdq;
    u8 ctrl = OMAP_HDQ_CTRL_STATUS_SINGLE | OMAP_HDQ_CTRL_STATUS_GO |
    OMAP_HDQ_CTRL_STATUS_INTERRUPTMASK;
    let mut mask: u8 = ctrl | OMAP_HDQ_CTRL_STATUS_DIR;
    err = pm_runtime_get_sync(hdq_data.dev);
    if (err < 0) {
    pm_runtime_put_noidle(hdq_data.dev);
    return err;
    }
    err = mutex_lock_interruptible(&hdq_data.hdq_mutex);
    if (err < 0) {
    dev_dbg(hdq_data.dev, "Could not acquire mutex\n");
    goto rtn;
    }
// read id_bit
    hdq_reg_merge(_hdq, OMAP_HDQ_CTRL_STATUS,
    ctrl | OMAP_HDQ_CTRL_STATUS_DIR, mask);
    err = wait_event_timeout(hdq_wait_queue,
    (hdq_data.hdq_irqstatus
    & OMAP_HDQ_INT_STATUS_RXCOMPLETE),
    OMAP_HDQ_TIMEOUT);
// Must clear irqstatus for another RXCOMPLETE interrupt
    hdq_reset_irqstatus(hdq_data, OMAP_HDQ_INT_STATUS_RXCOMPLETE);
    if (err == 0) {
    dev_dbg(hdq_data.dev, "RX wait elapsed\n");
    goto out;
    }
    id_bit = (hdq_reg_in(_hdq, OMAP_HDQ_RX_DATA) & 0x01);
// read comp_bit
    hdq_reg_merge(_hdq, OMAP_HDQ_CTRL_STATUS,
    ctrl | OMAP_HDQ_CTRL_STATUS_DIR, mask);
    err = wait_event_timeout(hdq_wait_queue,
    (hdq_data.hdq_irqstatus
    & OMAP_HDQ_INT_STATUS_RXCOMPLETE),
    OMAP_HDQ_TIMEOUT);
// Must clear irqstatus for another RXCOMPLETE interrupt
    hdq_reset_irqstatus(hdq_data, OMAP_HDQ_INT_STATUS_RXCOMPLETE);
    if (err == 0) {
    dev_dbg(hdq_data.dev, "RX wait elapsed\n");
    goto out;
    }
    comp_bit = (hdq_reg_in(_hdq, OMAP_HDQ_RX_DATA) & 0x01);
    if (id_bit && comp_bit) {
    ret = 0x03;  /* no slaves responded */
    goto out;
    }
    if (!id_bit && !comp_bit) {
// Both bits are valid, take the direction given
    ret = bdir ? 0x04 : 0;
    } else {
// Only one bit is valid, take that direction
    bdir = id_bit;
    ret = id_bit ? 0x05 : 0x02;
    }
// write bdir bit
    hdq_reg_out(_hdq, OMAP_HDQ_TX_DATA, bdir);
    hdq_reg_merge(_hdq, OMAP_HDQ_CTRL_STATUS, ctrl, mask);
    err = wait_event_timeout(hdq_wait_queue,
    (hdq_data.hdq_irqstatus
    & OMAP_HDQ_INT_STATUS_TXCOMPLETE),
    OMAP_HDQ_TIMEOUT);
// Must clear irqstatus for another TXCOMPLETE interrupt
    hdq_reset_irqstatus(hdq_data, OMAP_HDQ_INT_STATUS_TXCOMPLETE);
    if (err == 0) {
    dev_dbg(hdq_data.dev, "TX wait elapsed\n");
    goto out;
    }
    hdq_reg_merge(_hdq, OMAP_HDQ_CTRL_STATUS, 0,
    OMAP_HDQ_CTRL_STATUS_SINGLE);
    out:
    mutex_unlock(&hdq_data.hdq_mutex);
    rtn:
    pm_runtime_put_autosuspend(hdq_data.dev);
    return ret;
    }
// reset callback
#[no_mangle]
unsafe extern "C" fn omap_w1_reset_bus(_hdq: *mut c_void) -> u8 {
    static u8 omap_w1_reset_bus(void *_hdq)
    {
    struct hdq_data *hdq_data = _hdq;
    int err;
    err = pm_runtime_get_sync(hdq_data.dev);
    if (err < 0) {
    pm_runtime_put_noidle(hdq_data.dev);
    return err;
    }
    omap_hdq_break(hdq_data);
    pm_runtime_put_autosuspend(hdq_data.dev);
    return 0;
    }
// Read a byte of data from the device
#[no_mangle]
unsafe extern "C" fn omap_w1_read_byte(_hdq: *mut c_void) -> u8 {
    static u8 omap_w1_read_byte(void *_hdq)
    {
    struct hdq_data *hdq_data = _hdq;
    let mut val: u8 = 0;
    int ret;
    ret = pm_runtime_get_sync(hdq_data.dev);
    if (ret < 0) {
    pm_runtime_put_noidle(hdq_data.dev);
    return -1;
    }
    ret = hdq_read_byte(hdq_data, &val);
    if (ret)
    val = -1;
    pm_runtime_put_autosuspend(hdq_data.dev);
    return val;
    }
// Write a byte of data to the device
#[no_mangle]
unsafe extern "C" fn omap_w1_write_byte(_hdq: *mut c_void, byte: u8) {
    static void omap_w1_write_byte(void *_hdq, u8 byte)
    {
    struct hdq_data *hdq_data = _hdq;
    int ret;
    u8 status;
    ret = pm_runtime_get_sync(hdq_data.dev);
    if (ret < 0) {
    pm_runtime_put_noidle(hdq_data.dev);
    return;
    }
//
// We need to reset the slave before
// issuing the SKIP ROM command, else
// the slave will not work.
//
    if (byte == W1_SKIP_ROM)
    omap_hdq_break(hdq_data);
    ret = hdq_write_byte(hdq_data, byte, &status);
    if (ret < 0) {
    dev_dbg(hdq_data.dev, "TX failure:Ctrl status %x\n", status);
    goto out_err;
    }
    out_err:
    pm_runtime_put_autosuspend(hdq_data.dev);
    }
    static struct w1_bus_master omap_w1_master = {
    .read_byte	= omap_w1_read_byte,
    .write_byte	= omap_w1_write_byte,
    .reset_bus	= omap_w1_reset_bus,
    };
#[no_mangle]
unsafe extern "C" fn omap_hdq_runtime_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused omap_hdq_runtime_suspend(struct device *dev)
    {
    struct hdq_data *hdq_data = dev_get_drvdata(dev);
    hdq_reg_out(hdq_data, 0, hdq_data.mode);
    hdq_reg_in(hdq_data, OMAP_HDQ_INT_STATUS);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn omap_hdq_runtime_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused omap_hdq_runtime_resume(struct device *dev)
    {
    struct hdq_data *hdq_data = dev_get_drvdata(dev);
// select HDQ/1W mode & enable clocks
    hdq_reg_out(hdq_data, OMAP_HDQ_CTRL_STATUS,
    OMAP_HDQ_CTRL_STATUS_CLOCKENABLE |
    OMAP_HDQ_CTRL_STATUS_INTERRUPTMASK |
    hdq_data.mode);
    hdq_reg_in(hdq_data, OMAP_HDQ_INT_STATUS);
    return 0;
    }
    static const struct dev_pm_ops omap_hdq_pm_ops = {
    SET_RUNTIME_PM_OPS(omap_hdq_runtime_suspend,
    omap_hdq_runtime_resume, core::ptr::null_mut())
    };
#[no_mangle]
unsafe extern "C" fn omap_hdq_probe(pdev: *mut platform_device) -> c_int {
    static int omap_hdq_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct hdq_data *hdq_data;
    int ret, irq;
    u8 rev;
    const char *mode;
    hdq_data = devm_kzalloc(dev, sizeof(*hdq_data), GFP_KERNEL);
    if (!hdq_data)
    return -ENOMEM;
    hdq_data.dev = dev;
    platform_set_drvdata(pdev, hdq_data);
    hdq_data.hdq_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(hdq_data.hdq_base))
    return PTR_ERR(hdq_data.hdq_base);
    mutex_init(&hdq_data.hdq_mutex);
    ret = of_property_read_string(pdev.dev.of_node, "ti,mode", &mode);
    if (ret < 0 || !strcmp(mode, "hdq")) {
    hdq_data.mode = 0;
    omap_w1_master.search = omap_w1_search_bus;
    } else {
    hdq_data.mode = 1;
    omap_w1_master.triplet = omap_w1_triplet;
    }
    pm_runtime_enable(&pdev.dev);
    pm_runtime_use_autosuspend(&pdev.dev);
    pm_runtime_set_autosuspend_delay(&pdev.dev, 300);
    ret = pm_runtime_get_sync(&pdev.dev);
    if (ret < 0) {
    pm_runtime_put_noidle(&pdev.dev);
    dev_dbg(&pdev.dev, "pm_runtime_get_sync failed\n");
    goto err_w1;
    }
    rev = hdq_reg_in(hdq_data, OMAP_HDQ_REVISION);
    dev_info(&pdev.dev, "OMAP HDQ Hardware Rev %c.%c. Driver in %s mode\n",
    (rev >> 4) + '0', (rev & 0x0f) + '0', "Interrupt");
    spin_lock_init(&hdq_data.hdq_spinlock);
    irq = platform_get_irq(pdev, 0);
    if (irq	< 0) {
    dev_dbg(&pdev.dev, "Failed to get IRQ: %d\n", irq);
    ret = irq;
    goto err_irq;
    }
    ret = devm_request_irq(dev, irq, hdq_isr, 0, "omap_hdq", hdq_data);
    if (ret < 0) {
    dev_dbg(&pdev.dev, "could not request irq\n");
    goto err_irq;
    }
    omap_hdq_break(hdq_data);
    pm_runtime_put_autosuspend(&pdev.dev);
    omap_w1_master.data = hdq_data;
    ret = w1_add_master_device(&omap_w1_master);
    if (ret) {
    dev_dbg(&pdev.dev, "Failure in registering w1 master\n");
    goto err_w1;
    }
    return 0;
    err_irq:
    pm_runtime_put_sync(&pdev.dev);
    err_w1:
    pm_runtime_dont_use_autosuspend(&pdev.dev);
    pm_runtime_disable(&pdev.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn omap_hdq_remove(pdev: *mut platform_device) {
    static void omap_hdq_remove(struct platform_device *pdev)
    {
    int active;
    active = pm_runtime_get_sync(&pdev.dev);
    if (active < 0)
    pm_runtime_put_noidle(&pdev.dev);
    w1_remove_master_device(&omap_w1_master);
    pm_runtime_dont_use_autosuspend(&pdev.dev);
    if (active >= 0)
    pm_runtime_put_sync(&pdev.dev);
    pm_runtime_disable(&pdev.dev);
    }
    static const struct of_device_id omap_hdq_dt_ids[] = {
    { .compatible = "ti,omap3-1w" },
    { .compatible = "ti,am4372-hdq" },
    {}
    };
    MODULE_DEVICE_TABLE(of, omap_hdq_dt_ids);
    static struct platform_driver omap_hdq_driver = {
    .probe = omap_hdq_probe,
    .remove = omap_hdq_remove,
    .driver = {
    .name =	"omap_hdq",
    .of_match_table = omap_hdq_dt_ids,
    .pm = &omap_hdq_pm_ops,
    },
    };
    module_platform_driver(omap_hdq_driver);
    MODULE_AUTHOR("Texas Instruments");
    MODULE_DESCRIPTION("HDQ-1W driver Library");
    MODULE_LICENSE("GPL");
