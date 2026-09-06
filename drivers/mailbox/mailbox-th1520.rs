//! Automatically rewritten from C to Rust
//! Source: drivers/mailbox/mailbox-th1520.c
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
// Copyright (C) 2021 Alibaba Group Holding Limited.
//

// Status Register
pub const TH_1520_MBOX_STA: c_uint = 0x0;
pub const TH_1520_MBOX_CLR: c_uint = 0x4;
pub const TH_1520_MBOX_MASK: c_uint = 0xc;
// Transmit/receive data register:
// INFO0 ~ INFO6
//
pub const TH_1520_MBOX_INFO_NUM: c_int = 8;
pub const TH_1520_MBOX_DATA_INFO_NUM: c_int = 7;
pub const TH_1520_MBOX_INFO0: c_uint = 0x14;
// Transmit ack register: INFO7
pub const TH_1520_MBOX_INFO7: c_uint = 0x30;
// Generate remote icu IRQ Register
pub const TH_1520_MBOX_GEN: c_uint = 0x10;

pub const TH_1520_MBOX_CHAN_RES_SIZE: c_uint = 0x1000;
pub const TH_1520_MBOX_CHANS: c_int = 4;
pub const TH_1520_MBOX_CHAN_NAME_SIZE: c_int = 20;
pub const TH_1520_MBOX_ACK_MAGIC: c_uint = 0xdeadbeaf;

// store MBOX context across system-wide suspend/resume transitions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct th1520_mbox_context {
    pub intr_mask: [u32; TH_1520_MBOX_CHANS],
}

    enum th1520_mbox_icu_cpu_id {
    TH_1520_MBOX_ICU_KERNEL_CPU0, /* 910T */
    TH_1520_MBOX_ICU_CPU1, /* 902 */
    TH_1520_MBOX_ICU_CPU2, /* 906 */
    TH_1520_MBOX_ICU_CPU3, /* 910R */
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct th1520_mbox_con_priv {
    pub idx: enum th1520_mbox_icu_cpu_id,
    pub comm_local_base: *mut void __iomem,
    pub comm_remote_base: *mut void __iomem,
    pub irq_desc: [c_char; TH_1520_MBOX_CHAN_NAME_SIZE],
    pub chan: *mut mbox_chan,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct th1520_mbox_priv {
    pub dev: *mut device,
    pub local_icu: [*mut void __iomem; TH_1520_MBOX_CHANS],
    pub 1]: *mut *mut void __iomem remote_icu[TH_1520_MBOX_CHANS -,
    pub cur_cpu_ch_base: *mut void __iomem,
    pub /: *mut *mut spinlock_t mbox_lock; / control register lock,
    pub mbox: mbox_controller,
    pub mbox_chans: [mbox_chan; TH_1520_MBOX_CHANS],
    pub clocks: [clk_bulk_data; TH_1520_MBOX_CHANS],
    pub con_priv: [th1520_mbox_con_priv; TH_1520_MBOX_CHANS],
    pub irq: c_int,

    pub ctx: *mut th1520_mbox_context,

}

    static struct th1520_mbox_priv *
    to_th1520_mbox_priv(struct mbox_controller *mbox)
    {
    return container_of(mbox, struct th1520_mbox_priv, mbox);
    }
#[no_mangle]
unsafe extern "C" fn th1520_mbox_write(priv: *mut th1520_mbox_priv, val: u32, offs: u32) {
    static void th1520_mbox_write(struct th1520_mbox_priv *priv, u32 val, u32 offs)
    {
    iowrite32(val, priv.cur_cpu_ch_base + offs);
    }
#[no_mangle]
unsafe extern "C" fn th1520_mbox_read(priv: *mut th1520_mbox_priv, offs: u32) -> u32 {
    static u32 th1520_mbox_read(struct th1520_mbox_priv *priv, u32 offs)
    {
    return ioread32(priv.cur_cpu_ch_base + offs);
    }
    static u32 th1520_mbox_rmw(struct th1520_mbox_priv *priv, u32 off, u32 set,
    u32 clr)
    {
    unsigned long flags;
    u32 val;
    spin_lock_irqsave(&priv.mbox_lock, flags);
    val = th1520_mbox_read(priv, off);
    val &= ~clr;
    val |= set;
    th1520_mbox_write(priv, val, off);
    spin_unlock_irqrestore(&priv.mbox_lock, flags);
    return val;
    }
    static void th1520_mbox_chan_write(struct th1520_mbox_con_priv *cp, u32 val,
    u32 offs, bool is_remote)
    {
    if (is_remote)
    iowrite32(val, cp.comm_remote_base + offs);
    else
    iowrite32(val, cp.comm_local_base + offs);
    }
    static u32 th1520_mbox_chan_read(struct th1520_mbox_con_priv *cp, u32 offs,
    bool is_remote)
    {
    if (is_remote)
    return ioread32(cp.comm_remote_base + offs);
    else
    return ioread32(cp.comm_local_base + offs);
    }
    static void th1520_mbox_chan_rmw(struct th1520_mbox_con_priv *cp, u32 off,
    u32 set, u32 clr, bool is_remote)
    {
    struct th1520_mbox_priv *priv = to_th1520_mbox_priv(cp.chan.mbox);
    unsigned long flags;
    u32 val;
    spin_lock_irqsave(&priv.mbox_lock, flags);
    val = th1520_mbox_chan_read(cp, off, is_remote);
    val &= ~clr;
    val |= set;
    th1520_mbox_chan_write(cp, val, off, is_remote);
    spin_unlock_irqrestore(&priv.mbox_lock, flags);
    }
    static void th1520_mbox_chan_rd_data(struct th1520_mbox_con_priv *cp,
    void *data, bool is_remote)
    {
    let mut off: u32 = TH_1520_MBOX_INFO0;
    u32 *arg = data;
    u32 i;
// read info0 ~ info6, totally 28 bytes
// requires data memory size is 28 bytes
//
    for (i = 0; i < TH_1520_MBOX_DATA_INFO_NUM; i++) {
// arg = th1520_mbox_chan_read(cp, off, is_remote);
    off += 4;
    arg++;
    }
    }
    static void th1520_mbox_chan_wr_data(struct th1520_mbox_con_priv *cp,
    void *data, bool is_remote)
    {
    let mut off: u32 = TH_1520_MBOX_INFO0;
    u32 *arg = data;
    u32 i;
// write info0 ~ info6, totally 28 bytes
// requires data memory is 28 bytes valid data
//
    for (i = 0; i < TH_1520_MBOX_DATA_INFO_NUM; i++) {
    th1520_mbox_chan_write(cp, *arg, off, is_remote);
    off += 4;
    arg++;
    }
    }
    static void th1520_mbox_chan_wr_ack(struct th1520_mbox_con_priv *cp, void *data,
    bool is_remote)
    {
    let mut off: u32 = TH_1520_MBOX_INFO7;
    u32 *arg = data;
    th1520_mbox_chan_write(cp, *arg, off, is_remote);
    }
#[no_mangle]
unsafe extern "C" fn th1520_mbox_chan_id_to_mapbit(cp: *mut th1520_mbox_con_priv) -> c_int {
    static int th1520_mbox_chan_id_to_mapbit(struct th1520_mbox_con_priv *cp)
    {
    let mut mapbit: c_int = 0;
    int i;
    for (i = 0; i < TH_1520_MBOX_CHANS; i++) {
    if (i == cp.idx)
    return mapbit;
    if (i != TH_1520_MBOX_ICU_KERNEL_CPU0)
    mapbit++;
    }
    if (i == TH_1520_MBOX_CHANS)
    dev_err(cp.chan.mbox.dev, "convert to mapbit failed\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn th1520_mbox_isr(irq: c_int, p: *mut c_void) -> irqreturn_t {
    static irqreturn_t th1520_mbox_isr(int irq, void *p)
    {
    struct mbox_chan *chan = p;
    struct th1520_mbox_priv *priv = to_th1520_mbox_priv(chan.mbox);
    struct th1520_mbox_con_priv *cp = chan.con_priv;
    let mut mapbit: c_int = th1520_mbox_chan_id_to_mapbit(cp);
    u32 sta, dat[TH_1520_MBOX_DATA_INFO_NUM];
    let mut ack_magic: u32 = TH_1520_MBOX_ACK_MAGIC;
    u32 info0_data, info7_data;
    sta = th1520_mbox_read(priv, TH_1520_MBOX_STA);
    if (!(sta & BIT(mapbit)))
    return IRQ_NONE;
// clear chan irq bit in STA register
    th1520_mbox_rmw(priv, TH_1520_MBOX_CLR, BIT(mapbit), 0);
// info0 is the protocol word, should not be zero!
    info0_data = th1520_mbox_chan_read(cp, TH_1520_MBOX_INFO0, false);
    if (info0_data) {
// read info0~info6 data
    th1520_mbox_chan_rd_data(cp, dat, false);
// clear local info0
    th1520_mbox_chan_write(cp, 0x0, TH_1520_MBOX_INFO0, false);
// notify remote cpu
    th1520_mbox_chan_wr_ack(cp, &ack_magic, true);
// CPU1 902/906 use polling mode to monitor info7
    if (cp.idx != TH_1520_MBOX_ICU_CPU1 &&
    cp.idx != TH_1520_MBOX_ICU_CPU2)
    th1520_mbox_chan_rmw(cp, TH_1520_MBOX_GEN,
    TH_1520_MBOX_GEN_TX_ACK, 0, true);
// transfer the data to client
    mbox_chan_received_data(chan, (void *)dat);
    }
// info7 magic value mean the real ack signal, not generate bit7
    info7_data = th1520_mbox_chan_read(cp, TH_1520_MBOX_INFO7, false);
    if (info7_data == TH_1520_MBOX_ACK_MAGIC) {
// clear local info7
    th1520_mbox_chan_write(cp, 0x0, TH_1520_MBOX_INFO7, false);
// notify framework the last TX has completed
    mbox_chan_txdone(chan, 0);
    }
    if (!info0_data && !info7_data)
    return IRQ_NONE;
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn th1520_mbox_send_data(chan: *mut mbox_chan, data: *mut c_void) -> c_int {
    static int th1520_mbox_send_data(struct mbox_chan *chan, void *data)
    {
    struct th1520_mbox_con_priv *cp = chan.con_priv;
    th1520_mbox_chan_wr_data(cp, data, true);
    th1520_mbox_chan_rmw(cp, TH_1520_MBOX_GEN, TH_1520_MBOX_GEN_RX_DATA, 0,
    true);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn th1520_mbox_startup(chan: *mut mbox_chan) -> c_int {
    static int th1520_mbox_startup(struct mbox_chan *chan)
    {
    struct th1520_mbox_priv *priv = to_th1520_mbox_priv(chan.mbox);
    struct th1520_mbox_con_priv *cp = chan.con_priv;
    u32 data[8] = {};
    int mask_bit;
    int ret;
// clear local and remote generate and info0~info7
    th1520_mbox_chan_rmw(cp, TH_1520_MBOX_GEN, 0x0, 0xff, true);
    th1520_mbox_chan_rmw(cp, TH_1520_MBOX_GEN, 0x0, 0xff, false);
    th1520_mbox_chan_wr_ack(cp, &data[7], true);
    th1520_mbox_chan_wr_ack(cp, &data[7], false);
    th1520_mbox_chan_wr_data(cp, &data[0], true);
    th1520_mbox_chan_wr_data(cp, &data[0], false);
// enable the chan mask
    mask_bit = th1520_mbox_chan_id_to_mapbit(cp);
    th1520_mbox_rmw(priv, TH_1520_MBOX_MASK, BIT(mask_bit), 0);
//
// Mixing devm_ managed resources with manual IRQ handling is generally
// discouraged due to potential complexities with resource management,
// especially when dealing with shared interrupts. However, in this case,
// the approach is safe and effective because:
//
// 1. Each mailbox channel requests its IRQ within the .startup() callback
// and frees it within the .shutdown() callback.
// 2. During device unbinding, the devm_ managed mailbox controller first
// iterates through all channels, ensuring that their IRQs are freed before
// any other devm_ resources are released.
//
// This ordering guarantees that no interrupts can be triggered from the device
// while it is being unbound, preventing race conditions and ensuring system
// stability.
//
    ret = request_irq(priv.irq, th1520_mbox_isr,
    IRQF_SHARED | IRQF_NO_SUSPEND, cp.irq_desc, chan);
    if (ret) {
    dev_err(priv.dev, "Unable to acquire IRQ %d\n", priv.irq);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn th1520_mbox_shutdown(chan: *mut mbox_chan) {
    static void th1520_mbox_shutdown(struct mbox_chan *chan)
    {
    struct th1520_mbox_priv *priv = to_th1520_mbox_priv(chan.mbox);
    struct th1520_mbox_con_priv *cp = chan.con_priv;
    int mask_bit;
    free_irq(priv.irq, chan);
// clear the chan mask
    mask_bit = th1520_mbox_chan_id_to_mapbit(cp);
    th1520_mbox_rmw(priv, TH_1520_MBOX_MASK, 0, BIT(mask_bit));
    }
    static const struct mbox_chan_ops th1520_mbox_ops = {
    .send_data = th1520_mbox_send_data,
    .startup = th1520_mbox_startup,
    .shutdown = th1520_mbox_shutdown,
    };
#[no_mangle]
unsafe extern "C" fn th1520_mbox_init_generic(priv: *mut th1520_mbox_priv) -> c_int {
    static int th1520_mbox_init_generic(struct th1520_mbox_priv *priv)
    {

    priv.ctx = devm_kzalloc(priv.dev, sizeof(*priv.ctx), GFP_KERNEL);
    if (!priv.ctx)
    return -ENOMEM;

// Set default configuration
    th1520_mbox_write(priv, 0xff, TH_1520_MBOX_CLR);
    th1520_mbox_write(priv, 0x0, TH_1520_MBOX_MASK);
    return 0;
    }
    static struct mbox_chan *th1520_mbox_xlate(struct mbox_controller *mbox,
    const struct of_phandle_args *sp)
    {
    u32 chan;
    if (sp.args_count != 1) {
    dev_err(mbox.dev, "Invalid argument count %d\n",
    sp.args_count);
    return ERR_PTR(-EINVAL);
    }
    chan = sp.args[0]; /* comm remote channel */
    if (chan >= mbox.num_chans) {
    dev_err(mbox.dev, "Not supported channel number: %d\n", chan);
    return ERR_PTR(-EINVAL);
    }
    if (chan == TH_1520_MBOX_ICU_KERNEL_CPU0) {
    dev_err(mbox.dev, "Cannot communicate with yourself\n");
    return ERR_PTR(-EINVAL);
    }
    return &mbox.chans[chan];
    }
    static void __iomem *th1520_map_mmio(struct platform_device *pdev,
    char *res_name, size_t offset)
    {
    void __iomem *mapped;
    struct resource *res;
    res = platform_get_resource_byname(pdev, IORESOURCE_MEM, res_name);
    if (!res) {
    dev_err(&pdev.dev, "Failed to get resource: %s\n", res_name);
    return ERR_PTR(-EINVAL);
    }
    mapped = devm_ioremap(&pdev.dev, res.start + offset,
    resource_size(res) - offset);
    if (!mapped) {
    dev_err(&pdev.dev, "Failed to map resource: %s\n", res_name);
    return ERR_PTR(-ENOMEM);
    }
    return mapped;
    }
#[no_mangle]
unsafe extern "C" fn th1520_disable_clk(data: *mut c_void) {
    static void th1520_disable_clk(void *data)
    {
    struct th1520_mbox_priv *priv = data;
    clk_bulk_disable_unprepare(ARRAY_SIZE(priv.clocks), priv.clocks);
    }
#[no_mangle]
unsafe extern "C" fn th1520_mbox_probe(pdev: *mut platform_device) -> c_int {
    static int th1520_mbox_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct th1520_mbox_priv *priv;
    let mut remote_idx: c_uint = 0;
    unsigned int i;
    int ret;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    priv.dev = dev;
    priv.clocks[0].id = "clk-local";
    priv.clocks[1].id = "clk-remote-icu0";
    priv.clocks[2].id = "clk-remote-icu1";
    priv.clocks[3].id = "clk-remote-icu2";
    ret = devm_clk_bulk_get(dev, ARRAY_SIZE(priv.clocks),
    priv.clocks);
    if (ret) {
    dev_err(dev, "Failed to get clocks\n");
    return ret;
    }
    ret = clk_bulk_prepare_enable(ARRAY_SIZE(priv.clocks), priv.clocks);
    if (ret) {
    dev_err(dev, "Failed to enable clocks\n");
    return ret;
    }
    ret = devm_add_action_or_reset(dev, th1520_disable_clk, priv);
    if (ret)
    return ret;
//
// The address mappings in the device tree align precisely with those
// outlined in the manual. However, register offsets within these
// mapped regions are irregular, particularly for remote-icu0.
// Consequently, th1520_map_mmio() requires an additional parameter to
// handle this quirk.
//
    priv.local_icu[TH_1520_MBOX_ICU_KERNEL_CPU0] =
    th1520_map_mmio(pdev, "local", 0x0);
    if (IS_ERR(priv.local_icu[TH_1520_MBOX_ICU_KERNEL_CPU0]))
    return PTR_ERR(priv.local_icu[TH_1520_MBOX_ICU_KERNEL_CPU0]);
    priv.remote_icu[0] = th1520_map_mmio(pdev, "remote-icu0", 0x4000);
    if (IS_ERR(priv.remote_icu[0]))
    return PTR_ERR(priv.remote_icu[0]);
    priv.remote_icu[1] = th1520_map_mmio(pdev, "remote-icu1", 0x0);
    if (IS_ERR(priv.remote_icu[1]))
    return PTR_ERR(priv.remote_icu[1]);
    priv.remote_icu[2] = th1520_map_mmio(pdev, "remote-icu2", 0x0);
    if (IS_ERR(priv.remote_icu[2]))
    return PTR_ERR(priv.remote_icu[2]);
    priv.local_icu[TH_1520_MBOX_ICU_CPU1] =
    priv.local_icu[TH_1520_MBOX_ICU_KERNEL_CPU0] +
    TH_1520_MBOX_CHAN_RES_SIZE;
    priv.local_icu[TH_1520_MBOX_ICU_CPU2] =
    priv.local_icu[TH_1520_MBOX_ICU_CPU1] +
    TH_1520_MBOX_CHAN_RES_SIZE;
    priv.local_icu[TH_1520_MBOX_ICU_CPU3] =
    priv.local_icu[TH_1520_MBOX_ICU_CPU2] +
    TH_1520_MBOX_CHAN_RES_SIZE;
    priv.cur_cpu_ch_base = priv.local_icu[TH_1520_MBOX_ICU_KERNEL_CPU0];
    priv.irq = platform_get_irq(pdev, 0);
    if (priv.irq < 0)
    return priv.irq;
// init the chans
    for (i = 0; i < TH_1520_MBOX_CHANS; i++) {
    struct th1520_mbox_con_priv *cp = &priv.con_priv[i];
    cp.idx = i;
    cp.chan = &priv.mbox_chans[i];
    priv.mbox_chans[i].con_priv = cp;
    snprintf(cp.irq_desc, sizeof(cp.irq_desc),
    "th1520_mbox_chan[%i]", cp.idx);
    cp.comm_local_base = priv.local_icu[i];
    if (i != TH_1520_MBOX_ICU_KERNEL_CPU0) {
    cp.comm_remote_base = priv.remote_icu[remote_idx];
    remote_idx++;
    }
    }
    spin_lock_init(&priv.mbox_lock);
    priv.mbox.dev = dev;
    priv.mbox.ops = &th1520_mbox_ops;
    priv.mbox.chans = priv.mbox_chans;
    priv.mbox.num_chans = TH_1520_MBOX_CHANS;
    priv.mbox.of_xlate = th1520_mbox_xlate;
    priv.mbox.txdone_irq = true;
    platform_set_drvdata(pdev, priv);
    ret = th1520_mbox_init_generic(priv);
    if (ret) {
    dev_err(dev, "Failed to init mailbox context\n");
    return ret;
    }
    return devm_mbox_controller_register(dev, &priv.mbox);
    }
    static const struct of_device_id th1520_mbox_dt_ids[] = {
    { .compatible = "thead,th1520-mbox" },
    {}
    };
    MODULE_DEVICE_TABLE(of, th1520_mbox_dt_ids);

#[no_mangle]
unsafe extern "C" fn th1520_mbox_suspend_noirq(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused th1520_mbox_suspend_noirq(struct device *dev)
    {
    struct th1520_mbox_priv *priv = dev_get_drvdata(dev);
    struct th1520_mbox_context *ctx = priv.ctx;
    u32 i;
//
// ONLY interrupt mask bit should be stored and restores.
// INFO data all assumed to be lost.
//
    for (i = 0; i < TH_1520_MBOX_CHANS; i++) {
    ctx.intr_mask[i] =
    ioread32(priv.local_icu[i] + TH_1520_MBOX_MASK);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn th1520_mbox_resume_noirq(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused th1520_mbox_resume_noirq(struct device *dev)
    {
    struct th1520_mbox_priv *priv = dev_get_drvdata(dev);
    struct th1520_mbox_context *ctx = priv.ctx;
    u32 i;
    for (i = 0; i < TH_1520_MBOX_CHANS; i++) {
    iowrite32(ctx.intr_mask[i],
    priv.local_icu[i] + TH_1520_MBOX_MASK);
    }
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn th1520_mbox_runtime_suspend(dev: *mut device) -> int  __maybe_unused {
    static int  __maybe_unused th1520_mbox_runtime_suspend(struct device *dev)
    {
    struct th1520_mbox_priv *priv = dev_get_drvdata(dev);
    clk_bulk_disable_unprepare(ARRAY_SIZE(priv.clocks), priv.clocks);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn th1520_mbox_runtime_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused th1520_mbox_runtime_resume(struct device *dev)
    {
    struct th1520_mbox_priv *priv = dev_get_drvdata(dev);
    int ret;
    ret = clk_bulk_prepare_enable(ARRAY_SIZE(priv.clocks), priv.clocks);
    if (ret)
    dev_err(dev, "Failed to enable clocks in runtime resume\n");
    return ret;
    }
    static const struct dev_pm_ops th1520_mbox_pm_ops = {

    SET_NOIRQ_SYSTEM_SLEEP_PM_OPS(th1520_mbox_suspend_noirq,
    th1520_mbox_resume_noirq)

    SET_RUNTIME_PM_OPS(th1520_mbox_runtime_suspend,
    th1520_mbox_runtime_resume, core::ptr::null_mut())
    };
    static struct platform_driver th1520_mbox_driver = {
    .probe		= th1520_mbox_probe,
    .driver = {
    .name	= "th1520-mbox",
    .of_match_table = th1520_mbox_dt_ids,
    .pm = &th1520_mbox_pm_ops,
    },
    };
    module_platform_driver(th1520_mbox_driver);
    MODULE_DESCRIPTION("Thead TH-1520 mailbox IPC driver");
    MODULE_LICENSE("GPL");
