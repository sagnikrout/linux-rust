//! Automatically rewritten from C to Rust
//! Source: drivers/i2c/busses/i2c-qcom-cci.c
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
// Copyright (c) 2012-2016, The Linux Foundation. All rights reserved.
// Copyright (c) 2017-2022 Linaro Limited.

pub const CCI_HW_VERSION: c_uint = 0x0;
pub const CCI_RESET_CMD: c_uint = 0x004;
pub const CCI_RESET_CMD_MASK: c_uint = 0x0f73f3f7;
pub const CCI_RESET_CMD_M0_MASK: c_uint = 0x000003f1;
pub const CCI_RESET_CMD_M1_MASK: c_uint = 0x0003f001;
pub const CCI_QUEUE_START: c_uint = 0x008;
pub const CCI_HALT_REQ: c_uint = 0x034;

pub const CCI_IRQ_GLOBAL_CLEAR_CMD: c_uint = 0xc00;
pub const CCI_IRQ_MASK_0: c_uint = 0xc04;

pub const CCI_IRQ_MASK_0_I2C_M0_ERROR: c_uint = 0x18000ee6;
pub const CCI_IRQ_MASK_0_I2C_M1_ERROR: c_uint = 0x60ee6000;
pub const CCI_IRQ_CLEAR_0: c_uint = 0xc08;
pub const CCI_IRQ_STATUS_0: c_uint = 0xc0c;

pub const CCI_IRQ_STATUS_0_I2C_M0_ERROR: c_uint = 0x18000ee6;
pub const CCI_IRQ_STATUS_0_I2C_M1_ERROR: c_uint = 0x60ee6000;

pub const NUM_MASTERS: c_int = 2;
pub const NUM_QUEUES: c_int = 2;
pub const CCI_I2C_SET_PARAM: c_int = 1;
pub const CCI_I2C_REPORT: c_int = 8;
pub const CCI_I2C_WRITE: c_int = 9;
pub const CCI_I2C_READ: c_int = 10;

    enum {
    I2C_MODE_STANDARD,
    I2C_MODE_FAST,
    I2C_MODE_FAST_PLUS,
    };
    enum cci_i2c_queue_t {
    QUEUE_0,
    QUEUE_1
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_params {
    pub /: *mut *mut u16 thigh; / HIGH period of the SCL clock in clock ticks,
    pub /: *mut *mut u16 tlow; / LOW period of the SCL clock,
    pub /: *mut *mut u16 tsu_sto; / set-up time for STOP condition,
    pub /: *mut *mut u16 tsu_sta; / set-up time for a repeated START condition,
    pub /: *mut *mut u16 thd_dat; / data hold time,
    pub /: *mut *mut u16 thd_sta; / hold time (repeated) START condition,
    pub /: *mut *mut u16 tbuf; / bus free time between a STOP and START condition,
    pub scl_stretch_en: u8,
    pub trdhld: u16,
    pub /: *mut *mut u16 tsp; / pulse width of spikes suppressed by the input filter,
}

    struct cci;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cci_master {
    pub adap: i2c_adapter,
    pub master: u16,
    pub mode: u8,
    pub status: c_int,
    pub irq_complete: completion,
    pub cci: *mut cci,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cci_data {
    pub num_masters: c_uint,
    pub quirks: i2c_adapter_quirks,
    pub queue_size: [u16; NUM_QUEUES],
    pub params: [hw_params; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cci {
    pub dev: *mut device,
    pub base: *mut void __iomem,
    pub irq: c_uint,
    pub data: *const cci_data,
    pub clocks: *mut clk_bulk_data,
    pub nclocks: c_int,
    pub master: [cci_master; NUM_MASTERS],
}

#[no_mangle]
unsafe extern "C" fn cci_isr(irq: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t cci_isr(int irq, void *dev)
    {
    struct cci *cci = dev;
    u32 val, reset = 0;
    let mut ret: c_int = IRQ_NONE;
    val = readl(cci.base + CCI_IRQ_STATUS_0);
    writel(val, cci.base + CCI_IRQ_CLEAR_0);
    writel(0x1, cci.base + CCI_IRQ_GLOBAL_CLEAR_CMD);
    if (val & CCI_IRQ_STATUS_0_RST_DONE_ACK) {
    complete(&cci.master[0].irq_complete);
    if (cci.master[1].master)
    complete(&cci.master[1].irq_complete);
    ret = IRQ_HANDLED;
    }
    if (val & CCI_IRQ_STATUS_0_I2C_M0_RD_DONE ||
    val & CCI_IRQ_STATUS_0_I2C_M0_Q0_REPORT ||
    val & CCI_IRQ_STATUS_0_I2C_M0_Q1_REPORT) {
    cci.master[0].status = 0;
    complete(&cci.master[0].irq_complete);
    ret = IRQ_HANDLED;
    }
    if (val & CCI_IRQ_STATUS_0_I2C_M1_RD_DONE ||
    val & CCI_IRQ_STATUS_0_I2C_M1_Q0_REPORT ||
    val & CCI_IRQ_STATUS_0_I2C_M1_Q1_REPORT) {
    cci.master[1].status = 0;
    complete(&cci.master[1].irq_complete);
    ret = IRQ_HANDLED;
    }
    if (unlikely(val & CCI_IRQ_STATUS_0_I2C_M0_Q0Q1_HALT_ACK)) {
    reset = CCI_RESET_CMD_M0_MASK;
    ret = IRQ_HANDLED;
    }
    if (unlikely(val & CCI_IRQ_STATUS_0_I2C_M1_Q0Q1_HALT_ACK)) {
    reset = CCI_RESET_CMD_M1_MASK;
    ret = IRQ_HANDLED;
    }
    if (unlikely(reset))
    writel(reset, cci.base + CCI_RESET_CMD);
    if (unlikely(val & CCI_IRQ_STATUS_0_I2C_M0_ERROR)) {
    if (val & CCI_IRQ_STATUS_0_I2C_M0_Q0_NACK_ERR ||
    val & CCI_IRQ_STATUS_0_I2C_M0_Q1_NACK_ERR)
    cci.master[0].status = -ENXIO;
    else
    cci.master[0].status = -EIO;
    writel(CCI_HALT_REQ_I2C_M0_Q0Q1, cci.base + CCI_HALT_REQ);
    ret = IRQ_HANDLED;
    }
    if (unlikely(val & CCI_IRQ_STATUS_0_I2C_M1_ERROR)) {
    if (val & CCI_IRQ_STATUS_0_I2C_M1_Q0_NACK_ERR ||
    val & CCI_IRQ_STATUS_0_I2C_M1_Q1_NACK_ERR)
    cci.master[1].status = -ENXIO;
    else
    cci.master[1].status = -EIO;
    writel(CCI_HALT_REQ_I2C_M1_Q0Q1, cci.base + CCI_HALT_REQ);
    ret = IRQ_HANDLED;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cci_halt(cci: *mut cci, master_num: u8) -> c_int {
    static int cci_halt(struct cci *cci, u8 master_num)
    {
    struct cci_master *master;
    u32 val;
    if (master_num >= cci.data.num_masters) {
    dev_err(cci.dev, "Unsupported master idx (%u)\n", master_num);
    return -EINVAL;
    }
    val = BIT(master_num);
    master = &cci.master[master_num];
    reinit_completion(&master.irq_complete);
    writel(val, cci.base + CCI_HALT_REQ);
    if (!wait_for_completion_timeout(&master.irq_complete, CCI_TIMEOUT)) {
    dev_err(cci.dev, "CCI halt timeout\n");
    return -ETIMEDOUT;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cci_init(cci: *mut cci) {
    static void cci_init(struct cci *cci)
    {
    u32 val = CCI_IRQ_MASK_0_I2C_M0_RD_DONE |
    CCI_IRQ_MASK_0_I2C_M0_Q0_REPORT |
    CCI_IRQ_MASK_0_I2C_M0_Q1_REPORT |
    CCI_IRQ_MASK_0_I2C_M1_RD_DONE |
    CCI_IRQ_MASK_0_I2C_M1_Q0_REPORT |
    CCI_IRQ_MASK_0_I2C_M1_Q1_REPORT |
    CCI_IRQ_MASK_0_RST_DONE_ACK |
    CCI_IRQ_MASK_0_I2C_M0_Q0Q1_HALT_ACK |
    CCI_IRQ_MASK_0_I2C_M1_Q0Q1_HALT_ACK |
    CCI_IRQ_MASK_0_I2C_M0_ERROR |
    CCI_IRQ_MASK_0_I2C_M1_ERROR;
    int i;
    writel(val, cci.base + CCI_IRQ_MASK_0);
    for (i = 0; i < cci.data.num_masters; i++) {
    let mut mode: c_int = cci.master[i].mode;
    const struct hw_params *hw;
    if (!cci.master[i].cci)
    continue;
    hw = &cci.data.params[mode];
    val = hw.thigh << 16 | hw.tlow;
    writel(val, cci.base + CCI_I2C_Mm_SCL_CTL(i));
    val = hw.tsu_sto << 16 | hw.tsu_sta;
    writel(val, cci.base + CCI_I2C_Mm_SDA_CTL_0(i));
    val = hw.thd_dat << 16 | hw.thd_sta;
    writel(val, cci.base + CCI_I2C_Mm_SDA_CTL_1(i));
    val = hw.tbuf;
    writel(val, cci.base + CCI_I2C_Mm_SDA_CTL_2(i));
    val = hw.scl_stretch_en << 8 | hw.trdhld << 4 | hw.tsp;
    writel(val, cci.base + CCI_I2C_Mm_MISC_CTL(i));
    }
    }
#[no_mangle]
unsafe extern "C" fn cci_reset(cci: *mut cci) -> c_int {
    static int cci_reset(struct cci *cci)
    {
//
// we reset the whole controller, here and for implicity use
// master[0].xxx for waiting on it.
//
    reinit_completion(&cci.master[0].irq_complete);
    writel(CCI_RESET_CMD_MASK, cci.base + CCI_RESET_CMD);
    if (!wait_for_completion_timeout(&cci.master[0].irq_complete,
    CCI_TIMEOUT)) {
    dev_err(cci.dev, "CCI reset timeout\n");
    return -ETIMEDOUT;
    }
    cci_init(cci);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cci_run_queue(cci: *mut cci, master: u8, queue: u8) -> c_int {
    static int cci_run_queue(struct cci *cci, u8 master, u8 queue)
    {
    u32 val;
    val = readl(cci.base + CCI_I2C_Mm_Qn_CUR_WORD_CNT(master, queue));
    writel(val, cci.base + CCI_I2C_Mm_Qn_EXEC_WORD_CNT(master, queue));
    reinit_completion(&cci.master[master].irq_complete);
    val = BIT(master * 2 + queue);
    writel(val, cci.base + CCI_QUEUE_START);
    if (!wait_for_completion_timeout(&cci.master[master].irq_complete,
    CCI_TIMEOUT)) {
    dev_err(cci.dev, "master %d queue %d timeout\n",
    master, queue);
    cci_reset(cci);
    return -ETIMEDOUT;
    }
    return cci.master[master].status;
    }
#[no_mangle]
unsafe extern "C" fn cci_validate_queue(cci: *mut cci, master: u8, queue: u8) -> c_int {
    static int cci_validate_queue(struct cci *cci, u8 master, u8 queue)
    {
    u32 val;
    val = readl(cci.base + CCI_I2C_Mm_Qn_CUR_WORD_CNT(master, queue));
    if (val == cci.data.queue_size[queue])
    return -EINVAL;
    if (!val)
    return 0;
    val = CCI_I2C_REPORT | CCI_I2C_REPORT_IRQ_EN;
    writel(val, cci.base + CCI_I2C_Mm_Qn_LOAD_DATA(master, queue));
    return cci_run_queue(cci, master, queue);
    }
    static int cci_i2c_read(struct cci *cci, u16 master,
    u16 addr, u8 *buf, u16 len)
    {
    u32 val, words_read, words_exp;
    let mut queue: u8 = QUEUE_1;
    int i, index = 0, ret;
    let mut first: bool = true;
//
// Call validate queue to make sure queue is empty before starting.
// This is to avoid overflow / underflow of queue.
//
    ret = cci_validate_queue(cci, master, queue);
    if (ret < 0)
    return ret;
    val = CCI_I2C_SET_PARAM | (addr & 0x7f) << 4;
    writel(val, cci.base + CCI_I2C_Mm_Qn_LOAD_DATA(master, queue));
    val = CCI_I2C_READ | len << 4;
    writel(val, cci.base + CCI_I2C_Mm_Qn_LOAD_DATA(master, queue));
    ret = cci_run_queue(cci, master, queue);
    if (ret < 0)
    return ret;
    words_read = readl(cci.base + CCI_I2C_Mm_READ_BUF_LEVEL(master));
    words_exp = len / 4 + 1;
    if (words_read != words_exp) {
    dev_err(cci.dev, "words read = %d, words expected = %d\n",
    words_read, words_exp);
    return -EIO;
    }
    do {
    val = readl(cci.base + CCI_I2C_Mm_READ_DATA(master));
    for (i = 0; i < 4 && index < len; i++) {
    if (first) {
// The LS byte of this register represents the
// first byte read from the slave during a read
// access.
//
    first = false;
    continue;
    }
    buf[index++] = (val >> (i * 8)) & 0xff;
    }
    } while (--words_read);
    return 0;
    }
    static int cci_i2c_write(struct cci *cci, u16 master,
    u16 addr, u8 *buf, u16 len)
    {
    let mut queue: u8 = QUEUE_0;
    u8 load[12] = { 0 };
    let mut i: c_int = 0, j, ret;
    u32 val;
//
// Call validate queue to make sure queue is empty before starting.
// This is to avoid overflow / underflow of queue.
//
    ret = cci_validate_queue(cci, master, queue);
    if (ret < 0)
    return ret;
    val = CCI_I2C_SET_PARAM | (addr & 0x7f) << 4;
    writel(val, cci.base + CCI_I2C_Mm_Qn_LOAD_DATA(master, queue));
    load[i++] = CCI_I2C_WRITE | len << 4;
    for (j = 0; j < len; j++)
    load[i++] = buf[j];
    for (j = 0; j < i; j += 4) {
    val = load[j];
    val |= load[j + 1] << 8;
    val |= load[j + 2] << 16;
    val |= load[j + 3] << 24;
    writel(val, cci.base + CCI_I2C_Mm_Qn_LOAD_DATA(master, queue));
    }
    val = CCI_I2C_REPORT | CCI_I2C_REPORT_IRQ_EN;
    writel(val, cci.base + CCI_I2C_Mm_Qn_LOAD_DATA(master, queue));
    return cci_run_queue(cci, master, queue);
    }
#[no_mangle]
unsafe extern "C" fn cci_xfer(adap: *mut i2c_adapter, msgs[]: i2c_msg, num: c_int) -> c_int {
    static int cci_xfer(struct i2c_adapter *adap, struct i2c_msg msgs[], int num)
    {
    struct cci_master *cci_master = i2c_get_adapdata(adap);
    struct cci *cci = cci_master.cci;
    int i, ret;
    ret = pm_runtime_get_sync(cci.dev);
    if (ret < 0)
    goto err;
    for (i = 0; i < num; i++) {
    if (msgs[i].flags & I2C_M_RD)
    ret = cci_i2c_read(cci, cci_master.master,
    msgs[i].addr, msgs[i].buf,
    msgs[i].len);
    else
    ret = cci_i2c_write(cci, cci_master.master,
    msgs[i].addr, msgs[i].buf,
    msgs[i].len);
    if (ret < 0)
    break;
    }
    if (!ret)
    ret = num;
    err:
    pm_runtime_put_autosuspend(cci.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cci_func(adap: *mut i2c_adapter) -> u32 {
    static u32 cci_func(struct i2c_adapter *adap)
    {
    return I2C_FUNC_I2C | I2C_FUNC_SMBUS_EMUL;
    }
    static const struct i2c_algorithm cci_algo = {
    .xfer = cci_xfer,
    .functionality = cci_func,
    };
#[no_mangle]
unsafe extern "C" fn cci_enable_clocks(cci: *mut cci) -> c_int {
    static int cci_enable_clocks(struct cci *cci)
    {
    return clk_bulk_prepare_enable(cci.nclocks, cci.clocks);
    }
#[no_mangle]
unsafe extern "C" fn cci_disable_clocks(cci: *mut cci) {
    static void cci_disable_clocks(struct cci *cci)
    {
    clk_bulk_disable_unprepare(cci.nclocks, cci.clocks);
    }
#[no_mangle]
unsafe extern "C" fn cci_suspend_runtime(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused cci_suspend_runtime(struct device *dev)
    {
    struct cci *cci = dev_get_drvdata(dev);
    cci_disable_clocks(cci);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cci_resume_runtime(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused cci_resume_runtime(struct device *dev)
    {
    struct cci *cci = dev_get_drvdata(dev);
    int ret;
    ret = cci_enable_clocks(cci);
    if (ret)
    return ret;
    cci_init(cci);
    return 0;
    }
    static const struct dev_pm_ops qcom_cci_pm = {
    SET_SYSTEM_SLEEP_PM_OPS(pm_runtime_force_suspend, pm_runtime_force_resume)
    SET_RUNTIME_PM_OPS(cci_suspend_runtime, cci_resume_runtime, core::ptr::null_mut())
    };
#[no_mangle]
unsafe extern "C" fn cci_probe(pdev: *mut platform_device) -> c_int {
    static int cci_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *child;
    struct resource *r;
    struct cci *cci;
    int ret, i;
    u32 val;
    cci = devm_kzalloc(dev, sizeof(*cci), GFP_KERNEL);
    if (!cci)
    return -ENOMEM;
    cci.dev = dev;
    platform_set_drvdata(pdev, cci);
    cci.data = device_get_match_data(dev);
    if (!cci.data)
    return -ENOENT;
    for_each_available_child_of_node(dev.of_node, child) {
    struct cci_master *master;
    u32 idx;
    ret = of_property_read_u32(child, "reg", &idx);
    if (ret) {
    dev_err(dev, "%pOF invalid 'reg' property", child);
    continue;
    }
    if (idx >= cci.data.num_masters) {
    dev_err(dev, "%pOF invalid 'reg' value: %u (max is %u)",
    child, idx, cci.data.num_masters - 1);
    continue;
    }
    master = &cci.master[idx];
    master.adap.quirks = &cci.data.quirks;
    master.adap.algo = &cci_algo;
    master.adap.dev.parent = dev;
    master.adap.dev.of_node = of_node_get(child);
    master.master = idx;
    master.cci = cci;
    i2c_set_adapdata(&master.adap, master);
    snprintf(master.adap.name, sizeof(master.adap.name), "Qualcomm-CCI");
    master.mode = I2C_MODE_STANDARD;
    ret = of_property_read_u32(child, "clock-frequency", &val);
    if (!ret) {
    if (val == I2C_MAX_FAST_MODE_FREQ)
    master.mode = I2C_MODE_FAST;
#[no_mangle]
pub unsafe extern "C" fn if(I2C_MAX_FAST_MODE_PLUS_FREQ: val ==) -> else {
    else if (val == I2C_MAX_FAST_MODE_PLUS_FREQ)
    master.mode = I2C_MODE_FAST_PLUS;
    }
    init_completion(&master.irq_complete);
    }
// Memory
    cci.base = devm_platform_get_and_ioremap_resource(pdev, 0, &r);
    if (IS_ERR(cci.base))
    return PTR_ERR(cci.base);
// Clocks
    ret = devm_clk_bulk_get_all(dev, &cci.clocks);
    if (ret < 0)
    return dev_err_probe(dev, ret, "failed to get clocks\n");
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !ret) -> else {
    else if (!ret)
    return dev_err_probe(dev, -EINVAL, "not enough clocks in DT\n");
    cci.nclocks = ret;
    ret = cci_enable_clocks(cci);
    if (ret < 0)
    return ret;
// Interrupt
    ret = platform_get_irq(pdev, 0);
    if (ret < 0)
    goto disable_clocks;
    cci.irq = ret;
    ret = devm_request_irq(dev, cci.irq, cci_isr, 0, dev_name(dev), cci);
    if (ret < 0) {
    dev_err(dev, "request_irq failed, ret: %d\n", ret);
    goto disable_clocks;
    }
    val = readl(cci.base + CCI_HW_VERSION);
    dev_dbg(dev, "CCI HW version = 0x%08x", val);
    ret = cci_reset(cci);
    if (ret < 0)
    goto disable_clocks;
    pm_runtime_set_autosuspend_delay(dev, MSEC_PER_SEC);
    ret = devm_pm_runtime_set_active_enabled(dev);
    if (ret)
    goto disable_clocks;
    pm_runtime_use_autosuspend(dev);
    for (i = 0; i < cci.data.num_masters; i++) {
    if (!cci.master[i].cci)
    continue;
    ret = i2c_add_adapter(&cci.master[i].adap);
    if (ret < 0) {
    of_node_put(cci.master[i].adap.dev.of_node);
    goto error_i2c;
    }
    }
    return 0;
    error_i2c:
    for (--i ; i >= 0; i--) {
    if (cci.master[i].cci) {
    i2c_del_adapter(&cci.master[i].adap);
    of_node_put(cci.master[i].adap.dev.of_node);
    }
    }
    disable_clocks:
    cci_disable_clocks(cci);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cci_remove(pdev: *mut platform_device) {
    static void cci_remove(struct platform_device *pdev)
    {
    struct cci *cci = platform_get_drvdata(pdev);
    int i;
    for (i = 0; i < cci.data.num_masters; i++) {
    if (cci.master[i].cci) {
    i2c_del_adapter(&cci.master[i].adap);
    of_node_put(cci.master[i].adap.dev.of_node);
    cci_halt(cci, i);
    }
    }
    }
    static const struct cci_data cci_v1_data = {
    .num_masters = 1,
    .queue_size = { 64, 16 },
    .quirks = {
    .max_write_len = 10,
    .max_read_len = 12,
    },
    .params[I2C_MODE_STANDARD] = {
    .thigh = 78,
    .tlow = 114,
    .tsu_sto = 28,
    .tsu_sta = 28,
    .thd_dat = 10,
    .thd_sta = 77,
    .tbuf = 118,
    .scl_stretch_en = 0,
    .trdhld = 6,
    .tsp = 1
    },
    .params[I2C_MODE_FAST] = {
    .thigh = 20,
    .tlow = 28,
    .tsu_sto = 21,
    .tsu_sta = 21,
    .thd_dat = 13,
    .thd_sta = 18,
    .tbuf = 32,
    .scl_stretch_en = 0,
    .trdhld = 6,
    .tsp = 3
    },
    };
    static const struct cci_data cci_v1_5_data = {
    .num_masters = 2,
    .queue_size = { 64, 16 },
    .quirks = {
    .max_write_len = 10,
    .max_read_len = 12,
    },
    .params[I2C_MODE_STANDARD] = {
    .thigh = 78,
    .tlow = 114,
    .tsu_sto = 28,
    .tsu_sta = 28,
    .thd_dat = 10,
    .thd_sta = 77,
    .tbuf = 118,
    .scl_stretch_en = 0,
    .trdhld = 6,
    .tsp = 1
    },
    .params[I2C_MODE_FAST] = {
    .thigh = 20,
    .tlow = 28,
    .tsu_sto = 21,
    .tsu_sta = 21,
    .thd_dat = 13,
    .thd_sta = 18,
    .tbuf = 32,
    .scl_stretch_en = 0,
    .trdhld = 6,
    .tsp = 3
    },
    };
    static const struct cci_data cci_v2_data = {
    .num_masters = 2,
    .queue_size = { 64, 16 },
    .quirks = {
    .max_write_len = 11,
    .max_read_len = 12,
    },
    .params[I2C_MODE_STANDARD] = {
    .thigh = 201,
    .tlow = 174,
    .tsu_sto = 204,
    .tsu_sta = 231,
    .thd_dat = 22,
    .thd_sta = 162,
    .tbuf = 227,
    .scl_stretch_en = 0,
    .trdhld = 6,
    .tsp = 3
    },
    .params[I2C_MODE_FAST] = {
    .thigh = 38,
    .tlow = 56,
    .tsu_sto = 40,
    .tsu_sta = 40,
    .thd_dat = 22,
    .thd_sta = 35,
    .tbuf = 62,
    .scl_stretch_en = 0,
    .trdhld = 6,
    .tsp = 3
    },
    .params[I2C_MODE_FAST_PLUS] = {
    .thigh = 16,
    .tlow = 22,
    .tsu_sto = 17,
    .tsu_sta = 18,
    .thd_dat = 16,
    .thd_sta = 15,
    .tbuf = 24,
    .scl_stretch_en = 0,
    .trdhld = 3,
    .tsp = 3
    },
    };
    static const struct cci_data cci_msm8953_data = {
    .num_masters = 2,
    .queue_size = { 64, 16 },
    .quirks = {
    .max_write_len = 11,
    .max_read_len = 12,
    },
    .params[I2C_MODE_STANDARD] = {
    .thigh = 78,
    .tlow = 114,
    .tsu_sto = 28,
    .tsu_sta = 28,
    .thd_dat = 10,
    .thd_sta = 77,
    .tbuf = 118,
    .scl_stretch_en = 0,
    .trdhld = 6,
    .tsp = 1
    },
    .params[I2C_MODE_FAST] = {
    .thigh = 20,
    .tlow = 28,
    .tsu_sto = 21,
    .tsu_sta = 21,
    .thd_dat = 13,
    .thd_sta = 18,
    .tbuf = 32,
    .scl_stretch_en = 0,
    .trdhld = 6,
    .tsp = 3
    },
    .params[I2C_MODE_FAST_PLUS] = {
    .thigh = 16,
    .tlow = 22,
    .tsu_sto = 17,
    .tsu_sta = 18,
    .thd_dat = 16,
    .thd_sta = 15,
    .tbuf = 19,
    .scl_stretch_en = 1,
    .trdhld = 3,
    .tsp = 3
    },
    };
    static const struct of_device_id cci_dt_match[] = {
    { .compatible = "qcom,msm8226-cci", .data = &cci_v1_data},
    { .compatible = "qcom,msm8953-cci", .data = &cci_msm8953_data},
    { .compatible = "qcom,msm8974-cci", .data = &cci_v1_5_data},
    { .compatible = "qcom,msm8996-cci", .data = &cci_v2_data},
//
// Legacy compatibles kept for backwards compatibility.
// Do not add any new ones unless they introduce a new config
//
    { .compatible = "qcom,msm8916-cci", .data = &cci_v1_data},
    { .compatible = "qcom,sdm845-cci", .data = &cci_v2_data},
    { .compatible = "qcom,sm8250-cci", .data = &cci_v2_data},
    { .compatible = "qcom,sm8450-cci", .data = &cci_v2_data},
    {}
    };
    MODULE_DEVICE_TABLE(of, cci_dt_match);
    static struct platform_driver qcom_cci_driver = {
    .probe  = cci_probe,
    .remove = cci_remove,
    .driver = {
    .name = "i2c-qcom-cci",
    .of_match_table = cci_dt_match,
    .pm = &qcom_cci_pm,
    },
    };
    module_platform_driver(qcom_cci_driver);
    MODULE_DESCRIPTION("Qualcomm Camera Control Interface driver");
    MODULE_AUTHOR("Todor Tomov <todor.tomov@linaro.org>");
    MODULE_AUTHOR("Loic Poulain <loic.poulain@linaro.org>");
    MODULE_LICENSE("GPL v2");
