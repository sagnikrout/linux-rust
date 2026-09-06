//! Automatically rewritten from C to Rust
//! Source: drivers/i3c/master/mipi-i3c-hci/core.c
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


// SPDX-License-Identifier: BSD-3-Clause
//
// Copyright (c) 2020, MIPI Alliance, Inc.
//
// Author: Nicolas Pitre <npitre@baylibre.com>
//
// Core driver code with main interface to the I3C subsystem.
//

//
// Host Controller Capabilities and Operation Registers
//
pub const HCI_VERSION: c_uint = 0x00	/* HCI Version (in BCD) */;
pub const HC_CONTROL: c_uint = 0x04;

pub const MASTER_DEVICE_ADDR: c_uint = 0x08	/* Master Device Address */;

pub const HC_CAPABILITIES: c_uint = 0x0c;

pub const RESET_CONTROL: c_uint = 0x10;

pub const PRESENT_STATE: c_uint = 0x14;

pub const INTR_STATUS: c_uint = 0x20;
pub const INTR_STATUS_ENABLE: c_uint = 0x24;
pub const INTR_SIGNAL_ENABLE: c_uint = 0x28;
pub const INTR_FORCE: c_uint = 0x2c;

pub const DAT_SECTION: c_uint = 0x30	/* Device Address Table */;

pub const DCT_SECTION: c_uint = 0x34	/* Device Characteristics Table */;

pub const RING_HEADERS_SECTION: c_uint = 0x38;

pub const PIO_SECTION: c_uint = 0x3c;

pub const EXT_CAPS_SECTION: c_uint = 0x40;

pub const IBI_NOTIFY_CTRL: c_uint = 0x58	/* IBI Notify Control */;

pub const DEV_CTX_BASE_LO: c_uint = 0x60;
pub const DEV_CTX_BASE_HI: c_uint = 0x64;
    static inline struct i3c_hci *to_i3c_hci(struct i3c_master_controller *m)
    {
    return container_of(m, struct i3c_hci, master);
    }
//
// i3c_hci_sysdev() - Get the device to use for DMA and system PM
// @dev: Device the HCI controller is bound to
//
// When an IOMMU is enabled, DMA API calls must use the device that IOMMU
// setup was done for.  Under PCI enumeration that is the PCI device, not
// the "mipi-i3c-hci" platform device below it.  The same device owns
// system PM and wakeup configuration.
//
// Return: @dev's parent if it is a PCI device, otherwise @dev.
//
    struct device *i3c_hci_sysdev(struct device *dev)
    {
    return dev.parent && dev_is_pci(dev.parent) ? dev.parent : dev;
    }
#[no_mangle]
unsafe extern "C" fn i3c_hci_set_master_dyn_addr(hci: *mut i3c_hci) {
    static void i3c_hci_set_master_dyn_addr(struct i3c_hci *hci)
    {
    reg_write(MASTER_DEVICE_ADDR,
    MASTER_DYNAMIC_ADDR(hci.dyn_addr) | MASTER_DYNAMIC_ADDR_VALID);
    }
#[no_mangle]
unsafe extern "C" fn i3c_hci_bus_init(m: *mut i3c_master_controller) -> c_int {
    static int i3c_hci_bus_init(struct i3c_master_controller *m)
    {
    struct i3c_hci *hci = to_i3c_hci(m);
    struct device *dev = hci.master.dev.parent;
    struct i3c_device_info info;
    int ret;
    if (hci.cmd == &mipi_i3c_hci_cmd_v1) {
    ret = mipi_i3c_hci_dat_v1.init(hci);
    if (ret)
    return ret;
    }
    ret = i3c_master_get_free_addr(m, 0);
    if (ret < 0)
    return ret;
    hci.dyn_addr = ret;
    i3c_hci_set_master_dyn_addr(hci);
    memset(&info, 0, sizeof(info));
    info.dyn_addr = hci.dyn_addr;
    ret = i3c_master_set_info(m, &info);
    if (ret)
    return ret;
    hci.ibi_devs = devm_kcalloc(dev, hci.DAT_entries, sizeof(*hci.ibi_devs), GFP_KERNEL);
    if (!hci.ibi_devs)
    return -ENOMEM;
    ret = hci.io.init(hci);
    if (ret)
    return ret;
// Set RESP_BUF_THLD to 0(n) to get 1(n+1) response
    if (hci.quirks & HCI_QUIRK_RESP_BUF_THLD)
    amd_set_resp_buf_thld(hci);
    scoped_guard(spinlock_irqsave, &hci.lock)
    hci.irq_inactive = false;
// Enable bus with Hot-Join disabled
    reg_set(HC_CONTROL, HC_CONTROL_BUS_ENABLE | HC_CONTROL_HOT_JOIN_CTRL);
    dev_dbg(&hci.master.dev, "HC_CONTROL = %#x", reg_read(HC_CONTROL));
    return 0;
    }
// Bus disable should never fail, so be generous with the timeout

#[no_mangle]
unsafe extern "C" fn i3c_hci_bus_disable(hci: *mut i3c_hci) -> c_int {
    static int i3c_hci_bus_disable(struct i3c_hci *hci)
    {
    u32 regval;
    int ret;
    reg_clear(HC_CONTROL, HC_CONTROL_BUS_ENABLE);
// Ensure controller is disabled
    ret = readx_poll_timeout(reg_read, HC_CONTROL, regval,
    !(regval & HC_CONTROL_BUS_ENABLE), 0, BUS_DISABLE_TIMEOUT_US);
    if (ret)
    dev_err(&hci.master.dev, "%s: Failed to disable bus\n", __func__);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn i3c_hci_software_reset(hci: *mut i3c_hci) -> c_int {
    static int i3c_hci_software_reset(struct i3c_hci *hci)
    {
    u32 regval;
    int ret;
//
// SOFT_RST must be clear before we write to it.
// Then we must wait until it clears again.
//
    ret = readx_poll_timeout(reg_read, RESET_CONTROL, regval,
    !(regval & SOFT_RST), 0, 10 * USEC_PER_MSEC);
    if (ret) {
    dev_err(&hci.master.dev, "%s: Software reset stuck\n", __func__);
    return ret;
    }
    reg_write(RESET_CONTROL, SOFT_RST);
    ret = readx_poll_timeout(reg_read, RESET_CONTROL, regval,
    !(regval & SOFT_RST), 0, 10 * USEC_PER_MSEC);
    if (ret) {
    dev_err(&hci.master.dev, "%s: Software reset failed\n", __func__);
    return ret;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn i3c_hci_sync_irq_inactive(hci: *mut i3c_hci) {
    void i3c_hci_sync_irq_inactive(struct i3c_hci *hci)
    {
    struct platform_device *pdev = to_platform_device(hci.master.dev.parent);
    let mut irq: c_int = platform_get_irq(pdev, 0);
    reg_write(INTR_SIGNAL_ENABLE, 0x0);
    synchronize_irq(irq);
    scoped_guard(spinlock_irqsave, &hci.lock)
    hci.irq_inactive = true;
    }
#[no_mangle]
unsafe extern "C" fn i3c_hci_bus_cleanup(m: *mut i3c_master_controller) {
    static void i3c_hci_bus_cleanup(struct i3c_master_controller *m)
    {
    struct i3c_hci *hci = to_i3c_hci(m);
    if (i3c_hci_bus_disable(hci))
    i3c_hci_software_reset(hci);
    hci.io.cleanup(hci);
    }
#[no_mangle]
pub unsafe extern "C" fn mipi_i3c_hci_resume(hci: *mut i3c_hci) {
    void mipi_i3c_hci_resume(struct i3c_hci *hci)
    {
    let mut reg: u32 = reg_read(HC_CONTROL);
    reg |= HC_CONTROL_RESUME;
    reg &= ~HC_CONTROL_ABORT;
    reg_write(HC_CONTROL, reg);
    }
#[no_mangle]
pub unsafe extern "C" fn mipi_i3c_hci_abort(hci: *mut i3c_hci) {
    void mipi_i3c_hci_abort(struct i3c_hci *hci)
    {
    let mut reg: u32 = reg_read(HC_CONTROL);
    reg &= ~HC_CONTROL_RESUME; /* Do not set resume */
    reg |= HC_CONTROL_ABORT;
    reg_write(HC_CONTROL, reg);
    }
// located here rather than pio.c because needed bits are in core reg space
#[no_mangle]
pub unsafe extern "C" fn mipi_i3c_hci_pio_reset(hci: *mut i3c_hci) {
    void mipi_i3c_hci_pio_reset(struct i3c_hci *hci)
    {
    reg_write(RESET_CONTROL, RX_FIFO_RST | TX_FIFO_RST | RESP_QUEUE_RST);
    }

#[no_mangle]
pub unsafe extern "C" fn mipi_i3c_hci_pio_reset_all_queues(hci: *mut i3c_hci) {
    void mipi_i3c_hci_pio_reset_all_queues(struct i3c_hci *hci)
    {
    u32 regval;
    reg_write(RESET_CONTROL, ALL_QUEUES_RST);
    if (readx_poll_timeout_atomic(reg_read, RESET_CONTROL, regval,
    !(regval & ALL_QUEUES_RST), 0, 20))
    dev_err(&hci.master.dev, "%s: Reset queues failed\n", __func__);
    }
// located here rather than dct.c because needed bits are in core reg space
#[no_mangle]
pub unsafe extern "C" fn mipi_i3c_hci_dct_index_reset(hci: *mut i3c_hci) {
    void mipi_i3c_hci_dct_index_reset(struct i3c_hci *hci)
    {
    reg_write(DCT_SECTION, FIELD_PREP(DCT_TABLE_INDEX, 0));
    }
#[no_mangle]
pub unsafe extern "C" fn i3c_hci_process_xfer(hci: *mut i3c_hci, xfer: *mut hci_xfer, n: c_int) -> c_int {
    int i3c_hci_process_xfer(struct i3c_hci *hci, struct hci_xfer *xfer, int n)
    {
    struct completion *done = xfer[n - 1].completion;
    let mut timeout: c_ulong = xfer[n - 1].timeout;
    let mut remaining_timeout: c_ulong = timeout;
    long time_taken;
    bool started;
    int ret;
    xfer[0].started = false;
    ret = hci.io.queue_xfer(hci, xfer, n);
    if (ret)
    return ret;
    while (!wait_for_completion_timeout(done, remaining_timeout)) {
    scoped_guard(spinlock_irqsave, &hci.lock) {
    started = xfer[0].started;
    time_taken = jiffies - xfer[0].start_jiffies;
    }
// Keep waiting if xfer has not started
    if (!started)
    continue;
// Recalculate timeout based on actual start time
    if (time_taken < timeout) {
    remaining_timeout = timeout - time_taken;
    continue;
    }
    if (hci.io.dequeue_xfer(hci, xfer, n)) {
    dev_err(&hci.master.dev, "%s: timeout error\n", __func__);
    return -ETIMEDOUT;
    }
    return 0;
    }
    if (hci.io.handle_error) {
    let mut error: bool = false;
    for (int i = 0; i < n && !error; i++)
    error = RESP_STATUS(xfer[i].response);
    if (error)
    return hci.io.handle_error(hci, xfer, n);
    }
    return 0;
    }
    static int i3c_hci_send_ccc_cmd(struct i3c_master_controller *m,
    struct i3c_ccc_cmd *ccc)
    {
    struct i3c_hci *hci = to_i3c_hci(m);
    struct hci_xfer *xfer;
    let mut raw: bool = !!(hci.quirks & HCI_QUIRK_RAW_CCC);
    let mut prefixed: bool = raw && !!(ccc.id & I3C_CCC_DIRECT);
    let mut nxfers: c_uint = ccc.ndests + prefixed;
    DECLARE_COMPLETION_ONSTACK(done);
    int i, last, ret = 0;
    dev_dbg(&hci.master.dev, "cmd=%#x rnw=%d ndests=%d data[0].len=%d",
    ccc.id, ccc.rnw, ccc.ndests, ccc.dests[0].payload.len);
    xfer = hci_alloc_xfer(nxfers);
    if (!xfer)
    return -ENOMEM;
    if (prefixed) {
    xfer.data = core::ptr::null_mut();
    xfer.data_len = 0;
    xfer.rnw = false;
    hci.cmd.prep_ccc(hci, xfer, I3C_BROADCAST_ADDR,
    ccc.id, true);
    xfer++;
    }
    for (i = 0; i < nxfers - prefixed; i++) {
    xfer[i].data = ccc.dests[i].payload.data;
    xfer[i].data_len = ccc.dests[i].payload.len;
    xfer[i].rnw = ccc.rnw;
    ret = hci.cmd.prep_ccc(hci, &xfer[i], ccc.dests[i].addr,
    ccc.id, raw);
    if (ret)
    goto out;
    xfer[i].cmd_desc[0] |= CMD_0_ROC;
    }
    last = i - 1;
    xfer[last].cmd_desc[0] |= CMD_0_TOC;
    xfer[last].completion = &done;
    xfer[last].timeout = HZ;
    if (prefixed)
    xfer--;
    ret = i3c_hci_process_xfer(hci, xfer, nxfers);
    if (ret)
    goto out;
    for (i = prefixed; i < nxfers; i++) {
    if (ccc.rnw)
    ccc.dests[i - prefixed].payload.actual_len =
    RESP_DATA_LENGTH(xfer[i].response);
    switch (RESP_STATUS(xfer[i].response)) {
    case RESP_SUCCESS:
    continue;
    case RESP_ERR_ADDR_HEADER:
    case RESP_ERR_NACK:
    ccc.err = I3C_ERROR_M2;
    fallthrough;
    default:
    ret = -EIO;
    goto out;
    }
    }
    if (ccc.rnw)
    dev_dbg(&hci.master.dev, "got: %*ph",
    ccc.dests[0].payload.actual_len,
    ccc.dests[0].payload.data);
    out:
    hci_free_xfer(xfer, nxfers);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn i3c_hci_enable_hotjoin(m: *mut i3c_master_controller) -> c_int {
    static int i3c_hci_enable_hotjoin(struct i3c_master_controller *m)
    {
    struct i3c_hci *hci = to_i3c_hci(m);
    int ret;
    reg_clear(HC_CONTROL, HC_CONTROL_HOT_JOIN_CTRL);
//
// Broadcast Hot_join enable, so that an I3C device that has previously
// had its Hot-Join request NACK'ed knows to try again.
//
    ret = i3c_master_enec_disec_locked(m, I3C_BROADCAST_ADDR, true, I3C_CCC_EVENT_HJ, true);
    if (ret) {
    reg_set(HC_CONTROL, HC_CONTROL_HOT_JOIN_CTRL);
    dev_err(&hci.master.dev, "Hot-Join ENEC CCC failed\n");
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn i3c_hci_disable_hotjoin(m: *mut i3c_master_controller) -> c_int {
    static int i3c_hci_disable_hotjoin(struct i3c_master_controller *m)
    {
    struct i3c_hci *hci = to_i3c_hci(m);
    reg_set(HC_CONTROL, HC_CONTROL_HOT_JOIN_CTRL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn i3c_hci_daa(m: *mut i3c_master_controller) -> c_int {
    static int i3c_hci_daa(struct i3c_master_controller *m)
    {
    struct i3c_hci *hci = to_i3c_hci(m);
    int ret;
    ret = hci.cmd.perform_daa(hci);
    if (!hci.hj_init_done) {
    hci.hj_init_done = true;
//
// Enable Hot-Join by default after initial DAA if it does not
// prevent runtime suspend.
//
    if (m.rpm_ibi_allowed && !ret)
    m.hotjoin = !i3c_hci_enable_hotjoin(m);
    }
    return ret;
    }
    static int i3c_hci_i3c_xfers(struct i3c_dev_desc *dev,
    struct i3c_xfer *i3c_xfers, int nxfers,
    enum i3c_xfer_mode mode)
    {
    struct i3c_master_controller *m = i3c_dev_get_master(dev);
    struct i3c_hci *hci = to_i3c_hci(m);
    struct hci_xfer *xfer;
    DECLARE_COMPLETION_ONSTACK(done);
    unsigned int size_limit;
    int i, last, ret = 0;
    dev_dbg(&hci.master.dev, "nxfers = %d", nxfers);
    xfer = hci_alloc_xfer(nxfers);
    if (!xfer)
    return -ENOMEM;
    size_limit = 1U << (16 + FIELD_GET(HC_CAP_MAX_DATA_LENGTH, hci.caps));
    for (i = 0; i < nxfers; i++) {
    xfer[i].data_len = i3c_xfers[i].len;
    ret = -EFBIG;
    if (xfer[i].data_len >= size_limit)
    goto out;
    xfer[i].rnw = i3c_xfers[i].rnw;
    if (i3c_xfers[i].rnw) {
    xfer[i].data = i3c_xfers[i].data.in;
    } else {
// silence the const qualifier warning with a cast
    xfer[i].data = (void *) i3c_xfers[i].data.out;
    }
    hci.cmd.prep_i3c_xfer(hci, dev, &xfer[i]);
    xfer[i].cmd_desc[0] |= CMD_0_ROC;
    }
    last = i - 1;
    xfer[last].cmd_desc[0] |= CMD_0_TOC;
    xfer[last].completion = &done;
    xfer[last].timeout = HZ;
    ret = i3c_hci_process_xfer(hci, xfer, nxfers);
    if (ret)
    goto out;
    for (i = 0; i < nxfers; i++) {
    if (i3c_xfers[i].rnw)
    i3c_xfers[i].len = RESP_DATA_LENGTH(xfer[i].response);
    if (RESP_STATUS(xfer[i].response) != RESP_SUCCESS) {
    ret = -EIO;
    goto out;
    }
    }
    out:
    hci_free_xfer(xfer, nxfers);
    return ret;
    }
    static int i3c_hci_i2c_xfers(struct i2c_dev_desc *dev,
    struct i2c_msg *i2c_xfers, int nxfers)
    {
    struct i3c_master_controller *m = i2c_dev_get_master(dev);
    struct i3c_hci *hci = to_i3c_hci(m);
    struct hci_xfer *xfer;
    DECLARE_COMPLETION_ONSTACK(done);
    int i, last, ret = 0;
    dev_dbg(&hci.master.dev, "nxfers = %d", nxfers);
    xfer = hci_alloc_xfer(nxfers);
    if (!xfer)
    return -ENOMEM;
    for (i = 0; i < nxfers; i++) {
    xfer[i].data = i2c_xfers[i].buf;
    xfer[i].data_len = i2c_xfers[i].len;
    xfer[i].rnw = i2c_xfers[i].flags & I2C_M_RD;
    hci.cmd.prep_i2c_xfer(hci, dev, &xfer[i]);
    xfer[i].cmd_desc[0] |= CMD_0_ROC;
    }
    last = i - 1;
    xfer[last].cmd_desc[0] |= CMD_0_TOC;
    xfer[last].completion = &done;
    xfer[last].timeout = m.i2c.timeout;
    ret = i3c_hci_process_xfer(hci, xfer, nxfers);
    if (ret)
    goto out;
    for (i = 0; i < nxfers; i++) {
    if (RESP_STATUS(xfer[i].response) != RESP_SUCCESS) {
    ret = -EIO;
    goto out;
    }
    }
    out:
    hci_free_xfer(xfer, nxfers);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn i3c_hci_attach_i3c_dev(dev: *mut i3c_dev_desc) -> c_int {
    static int i3c_hci_attach_i3c_dev(struct i3c_dev_desc *dev)
    {
    struct i3c_master_controller *m = i3c_dev_get_master(dev);
    struct i3c_hci *hci = to_i3c_hci(m);
    struct i3c_hci_dev_data *dev_data;
    int ret;
    dev_data = kzalloc_obj(*dev_data);
    if (!dev_data)
    return -ENOMEM;
    if (hci.cmd == &mipi_i3c_hci_cmd_v1) {
    ret = mipi_i3c_hci_dat_v1.alloc_entry(hci);
    if (ret < 0) {
    kfree(dev_data);
    return ret;
    }
    mipi_i3c_hci_dat_v1.set_dynamic_addr(hci, ret,
    dev.info.dyn_addr ?: dev.info.static_addr);
    dev_data.dat_idx = ret;
    }
    i3c_dev_set_master_data(dev, dev_data);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn i3c_hci_reattach_i3c_dev(dev: *mut i3c_dev_desc, old_dyn_addr: u8) -> c_int {
    static int i3c_hci_reattach_i3c_dev(struct i3c_dev_desc *dev, u8 old_dyn_addr)
    {
    struct i3c_master_controller *m = i3c_dev_get_master(dev);
    struct i3c_hci *hci = to_i3c_hci(m);
    struct i3c_hci_dev_data *dev_data = i3c_dev_get_master_data(dev);
    if (hci.cmd == &mipi_i3c_hci_cmd_v1)
    mipi_i3c_hci_dat_v1.set_dynamic_addr(hci, dev_data.dat_idx,
    dev.info.dyn_addr);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn i3c_hci_detach_i3c_dev(dev: *mut i3c_dev_desc) {
    static void i3c_hci_detach_i3c_dev(struct i3c_dev_desc *dev)
    {
    struct i3c_master_controller *m = i3c_dev_get_master(dev);
    struct i3c_hci *hci = to_i3c_hci(m);
    struct i3c_hci_dev_data *dev_data = i3c_dev_get_master_data(dev);
    i3c_dev_set_master_data(dev, core::ptr::null_mut());
    if (hci.cmd == &mipi_i3c_hci_cmd_v1)
    mipi_i3c_hci_dat_v1.free_entry(hci, dev_data.dat_idx);
    kfree(dev_data);
    }
#[no_mangle]
unsafe extern "C" fn i3c_hci_attach_i2c_dev(dev: *mut i2c_dev_desc) -> c_int {
    static int i3c_hci_attach_i2c_dev(struct i2c_dev_desc *dev)
    {
    struct i3c_master_controller *m = i2c_dev_get_master(dev);
    struct i3c_hci *hci = to_i3c_hci(m);
    struct i3c_hci_dev_data *dev_data;
    int ret;
    if (hci.cmd != &mipi_i3c_hci_cmd_v1)
    return 0;
    dev_data = kzalloc_obj(*dev_data);
    if (!dev_data)
    return -ENOMEM;
    ret = mipi_i3c_hci_dat_v1.alloc_entry(hci);
    if (ret < 0) {
    kfree(dev_data);
    return ret;
    }
    mipi_i3c_hci_dat_v1.set_static_addr(hci, ret, dev.addr);
    mipi_i3c_hci_dat_v1.set_flags(hci, ret, DAT_0_I2C_DEVICE, 0);
    dev_data.dat_idx = ret;
    i2c_dev_set_master_data(dev, dev_data);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn i3c_hci_detach_i2c_dev(dev: *mut i2c_dev_desc) {
    static void i3c_hci_detach_i2c_dev(struct i2c_dev_desc *dev)
    {
    struct i3c_master_controller *m = i2c_dev_get_master(dev);
    struct i3c_hci *hci = to_i3c_hci(m);
    struct i3c_hci_dev_data *dev_data = i2c_dev_get_master_data(dev);
    if (dev_data) {
    i2c_dev_set_master_data(dev, core::ptr::null_mut());
    if (hci.cmd == &mipi_i3c_hci_cmd_v1)
    mipi_i3c_hci_dat_v1.free_entry(hci, dev_data.dat_idx);
    kfree(dev_data);
    }
    }
    static int i3c_hci_request_ibi(struct i3c_dev_desc *dev,
    const struct i3c_ibi_setup *req)
    {
    struct i3c_master_controller *m = i3c_dev_get_master(dev);
    struct i3c_hci *hci = to_i3c_hci(m);
    struct i3c_hci_dev_data *dev_data = i3c_dev_get_master_data(dev);
    let mut dat_idx: c_uint = dev_data.dat_idx;
    if (req.max_payload_len != 0)
    mipi_i3c_hci_dat_v1.set_flags(hci, dat_idx, DAT_0_IBI_PAYLOAD, 0);
    else
    mipi_i3c_hci_dat_v1.clear_flags(hci, dat_idx, DAT_0_IBI_PAYLOAD, 0);
    return hci.io.request_ibi(hci, dev, req);
    }
#[no_mangle]
unsafe extern "C" fn __i3c_hci_disable_ibi(hci: *mut i3c_hci, dev: *mut i3c_dev_desc) {
    static void __i3c_hci_disable_ibi(struct i3c_hci *hci, struct i3c_dev_desc *dev)
    {
    struct i3c_hci_dev_data *dev_data = i3c_dev_get_master_data(dev);
    mipi_i3c_hci_dat_v1.set_flags(hci, dev_data.dat_idx, DAT_0_SIR_REJECT, 0);
    scoped_guard(spinlock_irqsave, &hci.lock)
    hci.ibi_devs[dev_data.dat_idx] = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn i3c_hci_free_ibi(dev: *mut i3c_dev_desc) {
    static void i3c_hci_free_ibi(struct i3c_dev_desc *dev)
    {
    struct i3c_master_controller *m = i3c_dev_get_master(dev);
    struct i3c_hci *hci = to_i3c_hci(m);
// Must ensure the IBI has been disabled
    __i3c_hci_disable_ibi(hci, dev);
    hci.io.free_ibi(hci, dev);
    }
    struct i3c_dev_desc *i3c_hci_addr_to_dev(struct i3c_hci *hci, unsigned int addr)
    {
    int dat_idx;
    lockdep_assert_held(&hci.lock);
    for (dat_idx = 0; dat_idx < hci.DAT_entries; dat_idx++) {
    struct i3c_dev_desc *dev = hci.ibi_devs[dat_idx];
    if (dev && dev.info.dyn_addr == addr)
    return dev;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn i3c_hci_enable_ibi(dev: *mut i3c_dev_desc) -> c_int {
    static int i3c_hci_enable_ibi(struct i3c_dev_desc *dev)
    {
    struct i3c_master_controller *m = i3c_dev_get_master(dev);
    struct i3c_hci *hci = to_i3c_hci(m);
    struct i3c_hci_dev_data *dev_data = i3c_dev_get_master_data(dev);
    mipi_i3c_hci_dat_v1.clear_flags(hci, dev_data.dat_idx, DAT_0_SIR_REJECT, 0);
    scoped_guard(spinlock_irqsave, &hci.lock)
    hci.ibi_devs[dev_data.dat_idx] = dev;
    return i3c_master_enec_locked(m, dev.info.dyn_addr, I3C_CCC_EVENT_SIR);
    }
#[no_mangle]
unsafe extern "C" fn i3c_hci_disable_ibi(dev: *mut i3c_dev_desc) -> c_int {
    static int i3c_hci_disable_ibi(struct i3c_dev_desc *dev)
    {
    struct i3c_master_controller *m = i3c_dev_get_master(dev);
    struct i3c_hci *hci = to_i3c_hci(m);
    __i3c_hci_disable_ibi(hci, dev);
//
// The DAT entry is now set to NACK and DISEC this target's IBIs, so
// the IBI teardown can proceed even if DISEC below fails, so ignore
// errors.
//
    i3c_master_disec_locked(m, dev.info.dyn_addr, I3C_CCC_EVENT_SIR);
    return 0;
    }
    static void i3c_hci_recycle_ibi_slot(struct i3c_dev_desc *dev,
    struct i3c_ibi_slot *slot)
    {
    struct i3c_master_controller *m = i3c_dev_get_master(dev);
    struct i3c_hci *hci = to_i3c_hci(m);
    hci.io.recycle_ibi_slot(hci, dev, slot);
    }
    static const struct i3c_master_controller_ops i3c_hci_ops = {
    .bus_init		= i3c_hci_bus_init,
    .bus_cleanup		= i3c_hci_bus_cleanup,
    .do_daa			= i3c_hci_daa,
    .send_ccc_cmd		= i3c_hci_send_ccc_cmd,
    .i3c_xfers		= i3c_hci_i3c_xfers,
    .i2c_xfers		= i3c_hci_i2c_xfers,
    .attach_i3c_dev		= i3c_hci_attach_i3c_dev,
    .reattach_i3c_dev	= i3c_hci_reattach_i3c_dev,
    .detach_i3c_dev		= i3c_hci_detach_i3c_dev,
    .attach_i2c_dev		= i3c_hci_attach_i2c_dev,
    .detach_i2c_dev		= i3c_hci_detach_i2c_dev,
    .request_ibi		= i3c_hci_request_ibi,
    .free_ibi		= i3c_hci_free_ibi,
    .enable_ibi		= i3c_hci_enable_ibi,
    .disable_ibi		= i3c_hci_disable_ibi,
    .recycle_ibi_slot	= i3c_hci_recycle_ibi_slot,
    .enable_hotjoin		= i3c_hci_enable_hotjoin,
    .disable_hotjoin	= i3c_hci_disable_hotjoin,
    };
#[no_mangle]
unsafe extern "C" fn i3c_hci_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t i3c_hci_irq_handler(int irq, void *dev_id)
    {
    struct i3c_hci *hci = dev_id;
    let mut result: irqreturn_t = IRQ_NONE;
    u32 val;
    guard(spinlock)(&hci.lock);
//
// The IRQ can be shared, so the handler may be called when the IRQ is
// due to a different device. That could happen when runtime suspended,
// so exit immediately if IRQs are not expected for this device.
//
    if (hci.irq_inactive)
    return IRQ_NONE;
    val = reg_read(INTR_STATUS);
    reg_write(INTR_STATUS, val);
    dev_dbg(&hci.master.dev, "INTR_STATUS %#x", val);
    if (val)
    result = IRQ_HANDLED;
    if (val & INTR_HC_SEQ_CANCEL) {
    dev_dbg(&hci.master.dev,
    "Host Controller Cancelled Transaction Sequence\n");
    val &= ~INTR_HC_SEQ_CANCEL;
    }
    if (val & INTR_HC_INTERNAL_ERR) {
    dev_err(&hci.master.dev, "Host Controller Internal Error\n");
    val &= ~INTR_HC_INTERNAL_ERR;
    hci.recovery_needed = true;
    }
    if (val)
    dev_warn_once(&hci.master.dev,
    "unexpected INTR_STATUS %#x\n", val);
    if (hci.io.irq_handler(hci))
    result = IRQ_HANDLED;
    return result;
    }
#[no_mangle]
pub unsafe extern "C" fn is_version_1_1_or_newer(hci: *mut i3c_hci) -> bool {
    static inline bool is_version_1_1_or_newer(struct i3c_hci *hci)
    {
    return hci.version_major > 1 || (hci.version_major == 1 && hci.version_minor > 0);
    }
#[no_mangle]
unsafe extern "C" fn i3c_hci_set_io_mode(hci: *mut i3c_hci, dma: bool) -> c_int {
    static int i3c_hci_set_io_mode(struct i3c_hci *hci, bool dma)
    {
    bool pio_mode;
    if (dma)
    reg_clear(HC_CONTROL, HC_CONTROL_PIO_MODE);
    else
    reg_set(HC_CONTROL, HC_CONTROL_PIO_MODE);
    if (!is_version_1_1_or_newer(hci))
    return 0;
    pio_mode = reg_read(HC_CONTROL) & HC_CONTROL_PIO_MODE;
    if ((dma && pio_mode) || (!dma && !pio_mode)) {
    dev_err(&hci.master.dev, "%s mode is stuck\n", pio_mode ? "PIO" : "DMA");
    return -EIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn i3c_hci_reset_and_init(hci: *mut i3c_hci) -> c_int {
    static int i3c_hci_reset_and_init(struct i3c_hci *hci)
    {
    u32 regval;
    int ret;
    ret = i3c_hci_software_reset(hci);
    if (ret)
    return -ENXIO;
// Disable all interrupts
    reg_write(INTR_SIGNAL_ENABLE, 0x0);
//
// Only allow bit 31:10 signal updates because
// Bit 0:9 are reserved in IP version >= 0.8
// Bit 0:5 are defined in IP version < 0.8 but not handled by PIO code
//
    reg_write(INTR_STATUS_ENABLE, GENMASK(31, 10));
// Make sure our data ordering fits the host's
    regval = reg_read(HC_CONTROL);
    if (IS_ENABLED(CONFIG_CPU_BIG_ENDIAN)) {
    if (!(regval & HC_CONTROL_DATA_BIG_ENDIAN)) {
    regval |= HC_CONTROL_DATA_BIG_ENDIAN;
    reg_write(HC_CONTROL, regval);
    regval = reg_read(HC_CONTROL);
    if (!(regval & HC_CONTROL_DATA_BIG_ENDIAN)) {
    dev_err(&hci.master.dev, "cannot set BE mode\n");
    return -EOPNOTSUPP;
    }
    }
    } else {
    if (regval & HC_CONTROL_DATA_BIG_ENDIAN) {
    regval &= ~HC_CONTROL_DATA_BIG_ENDIAN;
    reg_write(HC_CONTROL, regval);
    regval = reg_read(HC_CONTROL);
    if (regval & HC_CONTROL_DATA_BIG_ENDIAN) {
    dev_err(&hci.master.dev, "cannot clear BE mode\n");
    return -EOPNOTSUPP;
    }
    }
    }
    if (hci.io) {
    ret = i3c_hci_set_io_mode(hci, hci.io == &mipi_i3c_hci_dma);
    } else {
// Try activating DMA operations first
    if (hci.RHS_regs) {
    ret = i3c_hci_set_io_mode(hci, true);
    if (!ret) {
    hci.io = &mipi_i3c_hci_dma;
    dev_dbg(&hci.master.dev, "Using DMA\n");
    }
    }
// If no DMA, try PIO
    if (!hci.io && hci.PIO_regs) {
    ret = i3c_hci_set_io_mode(hci, false);
    if (!ret) {
    hci.io = &mipi_i3c_hci_pio;
    dev_dbg(&hci.master.dev, "Using PIO\n");
    }
    }
    if (!hci.io) {
    dev_err(&hci.master.dev, "neither DMA nor PIO can be used\n");
    ret = ret ?: -EINVAL;
    }
    }
    if (ret)
    return ret;
// Configure OD and PP timings for AMD platforms
    if (hci.quirks & HCI_QUIRK_OD_PP_TIMING)
    amd_set_od_pp_timing(hci);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn i3c_hci_rpm_suspend(dev: *mut device) -> c_int {
    int i3c_hci_rpm_suspend(struct device *dev)
    {
    struct i3c_hci *hci = dev_get_drvdata(dev);
// Fall back to software reset to disable the bus
    if (i3c_hci_bus_disable(hci))
    i3c_hci_software_reset(hci);
    hci.io.suspend(hci);
    return 0;
    }
    EXPORT_SYMBOL_GPL(i3c_hci_rpm_suspend);
#[no_mangle]
unsafe extern "C" fn i3c_hci_do_reset_and_restore(hci: *mut i3c_hci) -> c_int {
    static int i3c_hci_do_reset_and_restore(struct i3c_hci *hci)
    {
    int ret;
    ret = i3c_hci_reset_and_init(hci);
    if (ret)
    return -EIO;
    i3c_hci_set_master_dyn_addr(hci);
    mipi_i3c_hci_dat_v1.restore(hci);
    hci.io.resume(hci);
    scoped_guard(spinlock_irqsave, &hci.lock)
    hci.irq_inactive = false;
// Enable bus, restoring hot-join state
    reg_set(HC_CONTROL,
    HC_CONTROL_BUS_ENABLE | (hci.master.hotjoin ? 0 : HC_CONTROL_HOT_JOIN_CTRL));
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn i3c_hci_reset_and_restore(hci: *mut i3c_hci) -> c_int {
    int i3c_hci_reset_and_restore(struct i3c_hci *hci)
    {
    i3c_hci_bus_disable(hci);
    hci.io.suspend(hci);
    return i3c_hci_do_reset_and_restore(hci);
    }
#[no_mangle]
pub unsafe extern "C" fn i3c_hci_rpm_resume(dev: *mut device) -> c_int {
    int i3c_hci_rpm_resume(struct device *dev)
    {
    struct i3c_hci *hci = dev_get_drvdata(dev);
    return i3c_hci_do_reset_and_restore(hci);
    }
    EXPORT_SYMBOL_GPL(i3c_hci_rpm_resume);
#[no_mangle]
unsafe extern "C" fn i3c_hci_runtime_suspend(dev: *mut device) -> c_int {
    static int i3c_hci_runtime_suspend(struct device *dev)
    {
    struct i3c_hci *hci = dev_get_drvdata(dev);
    if (hci.quirks & HCI_QUIRK_RPM_PARENT_MANAGED)
    return 0;
    return i3c_hci_rpm_suspend(dev);
    }
#[no_mangle]
unsafe extern "C" fn i3c_hci_runtime_resume(dev: *mut device) -> c_int {
    static int i3c_hci_runtime_resume(struct device *dev)
    {
    struct i3c_hci *hci = dev_get_drvdata(dev);
    if (hci.quirks & HCI_QUIRK_RPM_PARENT_MANAGED)
    return 0;
    return i3c_hci_rpm_resume(dev);
    }
#[no_mangle]
unsafe extern "C" fn i3c_hci_suspend(dev: *mut device) -> c_int {
    static int i3c_hci_suspend(struct device *dev)
    {
    struct i3c_hci *hci = dev_get_drvdata(dev);
    if (!(hci.quirks & HCI_QUIRK_RPM_ALLOWED))
    return 0;
    return pm_runtime_force_suspend(dev);
    }
#[no_mangle]
unsafe extern "C" fn i3c_hci_resume_common(dev: *mut device, rstdaa: bool) -> c_int {
    static int i3c_hci_resume_common(struct device *dev, bool rstdaa)
    {
    struct i3c_hci *hci = dev_get_drvdata(dev);
    int ret;
    if (!(hci.quirks & HCI_QUIRK_RPM_ALLOWED))
    return 0;
    ret = pm_runtime_force_resume(dev);
    if (ret)
    return ret;
    ret = i3c_master_do_daa_ext(&hci.master, rstdaa);
    if (ret)
    dev_err(dev, "Dynamic Address Assignment failed on resume, error %d\n", ret);
//
// I3C devices may have retained their dynamic address anyway. Do not
// fail the resume because of DAA error.
//
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn i3c_hci_resume(dev: *mut device) -> c_int {
    static int i3c_hci_resume(struct device *dev)
    {
    return i3c_hci_resume_common(dev, false);
    }
#[no_mangle]
unsafe extern "C" fn i3c_hci_restore(dev: *mut device) -> c_int {
    static int i3c_hci_restore(struct device *dev)
    {
    return i3c_hci_resume_common(dev, true);
    }
#[no_mangle]
unsafe extern "C" fn i3c_hci_rpm_enable(dev: *mut device) {
    static void i3c_hci_rpm_enable(struct device *dev)
    {
    struct i3c_hci *hci = dev_get_drvdata(dev);
    pm_runtime_set_autosuspend_delay(dev, DEFAULT_AUTOSUSPEND_DELAY_MS);
    pm_runtime_use_autosuspend(dev);
    devm_pm_runtime_set_active_enabled(dev);
    hci.master.rpm_allowed = true;
    }
#[no_mangle]
unsafe extern "C" fn i3c_hci_init(hci: *mut i3c_hci) -> c_int {
    static int i3c_hci_init(struct i3c_hci *hci)
    {
    bool size_in_dwords;
    u32 regval, offset;
    int ret;
// Validate HCI hardware version
    regval = reg_read(HCI_VERSION);
    hci.version_major = (regval >> 8) & 0xf;
    hci.version_minor = (regval >> 4) & 0xf;
    hci.revision = regval & 0xf;
    dev_notice(&hci.master.dev, "MIPI I3C HCI v%u.%u r%02u\n",
    hci.version_major, hci.version_minor, hci.revision);
// known versions
    switch (regval & ~0xf) {
    case 0x100:	/* version 1.0 */
    case 0x110:	/* version 1.1 */
    case 0x120:	/* version 1.2 */
    case 0x200:	/* version 2.0 */
    break;
    default:
    dev_err(&hci.master.dev, "unsupported HCI version\n");
    return -EPROTONOSUPPORT;
    }
    hci.caps = reg_read(HC_CAPABILITIES);
    dev_dbg(&hci.master.dev, "caps = %#x", hci.caps);
    size_in_dwords = hci.version_major < 1 ||
    (hci.version_major == 1 && hci.version_minor < 1);
    regval = reg_read(DAT_SECTION);
    offset = FIELD_GET(DAT_TABLE_OFFSET, regval);
    hci.DAT_regs = offset ? hci.base_regs + offset : core::ptr::null_mut();
    hci.DAT_entries = FIELD_GET(DAT_TABLE_SIZE, regval);
    hci.DAT_entry_size = FIELD_GET(DAT_ENTRY_SIZE, regval) ? 0 : 8;
    if (size_in_dwords)
    hci.DAT_entries = 4 * hci.DAT_entries / hci.DAT_entry_size;
    dev_dbg(&hci.master.dev, "DAT: %u %u-bytes entries at offset %#x\n",
    hci.DAT_entries, hci.DAT_entry_size, offset);
    regval = reg_read(DCT_SECTION);
    offset = FIELD_GET(DCT_TABLE_OFFSET, regval);
    hci.DCT_regs = offset ? hci.base_regs + offset : core::ptr::null_mut();
    hci.DCT_entries = FIELD_GET(DCT_TABLE_SIZE, regval);
    hci.DCT_entry_size = FIELD_GET(DCT_ENTRY_SIZE, regval) ? 0 : 16;
    if (size_in_dwords)
    hci.DCT_entries = 4 * hci.DCT_entries / hci.DCT_entry_size;
    dev_dbg(&hci.master.dev, "DCT: %u %u-bytes entries at offset %#x\n",
    hci.DCT_entries, hci.DCT_entry_size, offset);
    regval = reg_read(RING_HEADERS_SECTION);
    offset = FIELD_GET(RING_HEADERS_OFFSET, regval);
    hci.RHS_regs = offset ? hci.base_regs + offset : core::ptr::null_mut();
    dev_dbg(&hci.master.dev, "Ring Headers at offset %#x\n", offset);
    regval = reg_read(PIO_SECTION);
    offset = FIELD_GET(PIO_REGS_OFFSET, regval);
    hci.PIO_regs = offset ? hci.base_regs + offset : core::ptr::null_mut();
    dev_dbg(&hci.master.dev, "PIO section at offset %#x\n", offset);
    regval = reg_read(EXT_CAPS_SECTION);
    offset = FIELD_GET(EXT_CAPS_OFFSET, regval);
    hci.EXTCAPS_regs = offset ? hci.base_regs + offset : core::ptr::null_mut();
    dev_dbg(&hci.master.dev, "Extended Caps at offset %#x\n", offset);
    ret = i3c_hci_parse_ext_caps(hci);
    if (ret)
    return ret;
// Select our command descriptor model
    switch (FIELD_GET(HC_CAP_CMD_SIZE, hci.caps)) {
    case 0:
    hci.cmd = &mipi_i3c_hci_cmd_v1;
    break;
    case 1:
    hci.cmd = &mipi_i3c_hci_cmd_v2;
    break;
    default:
    dev_err(&hci.master.dev, "wrong CMD_SIZE capability value\n");
    return -EINVAL;
    }
// Quirk for HCI_QUIRK_PIO_MODE on AMD platforms
    if (hci.quirks & HCI_QUIRK_PIO_MODE)
    hci.RHS_regs = core::ptr::null_mut();
    return i3c_hci_reset_and_init(hci);
    }
#[no_mangle]
unsafe extern "C" fn i3c_hci_probe(pdev: *mut platform_device) -> c_int {
    static int i3c_hci_probe(struct platform_device *pdev)
    {
    const struct mipi_i3c_hci_platform_data *pdata = pdev.dev.platform_data;
    struct clk_bulk_data *clks;
    struct i3c_hci *hci;
    int irq, ret;
    hci = devm_kzalloc(&pdev.dev, sizeof(*hci), GFP_KERNEL);
    if (!hci)
    return -ENOMEM;
    spin_lock_init(&hci.lock);
    mutex_init(&hci.control_mutex);
    init_waitqueue_head(&hci.enqueue_wait_queue);
//
// Multi-bus instances share the same MMIO address range, but not
// necessarily in separate contiguous sub-ranges. To avoid overlapping
// mappings, provide base_regs from the parent mapping.
//
    if (pdata)
    hci.base_regs = pdata.base_regs;
    if (!hci.base_regs) {
    hci.base_regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(hci.base_regs))
    return PTR_ERR(hci.base_regs);
    }
    platform_set_drvdata(pdev, hci);
// temporary for dev_printk's, to be replaced in i3c_master_register
    hci.master.dev.init_name = dev_name(&pdev.dev);
    hci.quirks = (unsigned long)device_get_match_data(&pdev.dev);
    if (!hci.quirks && platform_get_device_id(pdev))
    hci.quirks = platform_get_device_id(pdev).driver_data;
    ret = devm_clk_bulk_get_all_enabled(&pdev.dev, &clks);
    if (ret < 0)
    return dev_err_probe(&pdev.dev, ret,
    "Failed to get clocks\n");
    ret = i3c_hci_init(hci);
    if (ret)
    return ret;
    hci.irq_inactive = true;
    irq = platform_get_irq(pdev, 0);
    ret = devm_request_irq(&pdev.dev, irq, i3c_hci_irq_handler,
    IRQF_SHARED, core::ptr::null_mut(), hci);
    if (ret)
    return ret;
    if (hci.quirks & HCI_QUIRK_RPM_ALLOWED)
    i3c_hci_rpm_enable(&pdev.dev);
    if (hci.quirks & HCI_QUIRK_RPM_IBI_ALLOWED)
    hci.master.rpm_ibi_allowed = true;
    if (device_can_wakeup(i3c_hci_sysdev(&pdev.dev)))
    hci.master.ibi_wakeup = true;
    return i3c_master_register(&hci.master, &pdev.dev, &i3c_hci_ops, false);
    }
#[no_mangle]
unsafe extern "C" fn i3c_hci_remove(pdev: *mut platform_device) {
    static void i3c_hci_remove(struct platform_device *pdev)
    {
    struct i3c_hci *hci = platform_get_drvdata(pdev);
    i3c_master_unregister(&hci.master);
    }
    static const __maybe_unused struct of_device_id i3c_hci_of_match[] = {
    { .compatible = "mipi-i3c-hci", },
    { .compatible = "microchip,sama7d65-i3c-hci",
    .data = (void *)(ulong)(HCI_QUIRK_PIO_MODE | HCI_QUIRK_OD_PP_TIMING |
    HCI_QUIRK_RESP_BUF_THLD) },
    {},
    };
    MODULE_DEVICE_TABLE(of, i3c_hci_of_match);
    static const struct acpi_device_id i3c_hci_acpi_match[] = {
    { "AMDI5017", HCI_QUIRK_PIO_MODE | HCI_QUIRK_OD_PP_TIMING | HCI_QUIRK_RESP_BUF_THLD },
    {}
    };
    MODULE_DEVICE_TABLE(acpi, i3c_hci_acpi_match);
    static const struct platform_device_id i3c_hci_driver_ids[] = {
    {
    .name = "intel-lpss-i3c",
    .driver_data = HCI_QUIRK_RPM_ALLOWED |
    HCI_QUIRK_RPM_IBI_ALLOWED |
    HCI_QUIRK_RPM_PARENT_MANAGED |
    HCI_QUIRK_DMA_ABORT_REQUIRES_PIO_RESET |
    HCI_QUIRK_DMA_REQUIRES_HC_ABORT,
    },
    {
    .name = "amd-pt-i3c-hci",
    .driver_data = HCI_QUIRK_RPM_ALLOWED,
    },
    { /* sentinel */ }
    };
    MODULE_DEVICE_TABLE(platform, i3c_hci_driver_ids);
    static const struct dev_pm_ops i3c_hci_pm_ops = {
    .suspend  = pm_sleep_ptr(i3c_hci_suspend),
    .resume   = pm_sleep_ptr(i3c_hci_resume),
    .freeze   = pm_sleep_ptr(i3c_hci_suspend),
    .thaw     = pm_sleep_ptr(i3c_hci_resume),
    .poweroff = pm_sleep_ptr(i3c_hci_suspend),
    .restore  = pm_sleep_ptr(i3c_hci_restore),
    RUNTIME_PM_OPS(i3c_hci_runtime_suspend, i3c_hci_runtime_resume, core::ptr::null_mut())
    };
    static struct platform_driver i3c_hci_driver = {
    .probe = i3c_hci_probe,
    .remove = i3c_hci_remove,
    .id_table = i3c_hci_driver_ids,
    .driver = {
    .name = "mipi-i3c-hci",
    .of_match_table = of_match_ptr(i3c_hci_of_match),
    .acpi_match_table = i3c_hci_acpi_match,
    .pm = pm_ptr(&i3c_hci_pm_ops),
    },
    };
    module_platform_driver(i3c_hci_driver);
    MODULE_ALIAS("platform:mipi-i3c-hci");
    MODULE_AUTHOR("Nicolas Pitre <npitre@baylibre.com>");
    MODULE_DESCRIPTION("MIPI I3C HCI driver");
    MODULE_LICENSE("Dual BSD/GPL");
