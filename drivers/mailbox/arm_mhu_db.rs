//! Automatically rewritten from C to Rust
//! Source: drivers/mailbox/arm_mhu_db.c
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
// Copyright (C) 2013-2015 Fujitsu Semiconductor Ltd.
// Copyright (C) 2015 Linaro Ltd.
// Based on ARM MHU driver by Jassi Brar <jaswinder.singh@linaro.org>
// Copyright (C) 2020 ARM Ltd.
//

pub const INTR_STAT_OFS: c_uint = 0x0;
pub const INTR_SET_OFS: c_uint = 0x8;
pub const INTR_CLR_OFS: c_uint = 0x10;
pub const MHU_LP_OFFSET: c_uint = 0x0;
pub const MHU_HP_OFFSET: c_uint = 0x20;
pub const MHU_SEC_OFFSET: c_uint = 0x200;
pub const TX_REG_OFFSET: c_uint = 0x100;

pub const MHU_NUM_DOORBELLS: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhu_db_link {
    pub irq: c_uint,
    pub tx_reg: *mut void __iomem,
    pub rx_reg: *mut void __iomem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arm_mhu {
    pub base: *mut void __iomem,
    pub mlink: [mhu_db_link; MHU_CHANS],
    pub mbox: mbox_controller,
    pub dev: *mut device,
}

//
// struct mhu_db_channel - ARM MHU Mailbox allocated channel information
//
// @mhu: Pointer to parent mailbox device
// @pchan: Physical channel within which this doorbell resides in
// @doorbell: doorbell number pertaining to this channel
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhu_db_channel {
    pub mhu: *mut arm_mhu,
    pub pchan: c_uint,
    pub doorbell: c_uint,
}

    static inline struct mbox_chan *
    mhu_db_mbox_to_channel(struct mbox_controller *mbox, unsigned int pchan,
    unsigned int doorbell)
    {
    int i;
    struct mhu_db_channel *chan_info;
    for (i = 0; i < mbox.num_chans; i++) {
    chan_info = mbox.chans[i].con_priv;
    if (chan_info && chan_info.pchan == pchan &&
    chan_info.doorbell == doorbell)
    return &mbox.chans[i];
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn mhu_db_mbox_clear_irq(chan: *mut mbox_chan) {
    static void mhu_db_mbox_clear_irq(struct mbox_chan *chan)
    {
    struct mhu_db_channel *chan_info = chan.con_priv;
    void __iomem *base = chan_info.mhu.mlink[chan_info.pchan].rx_reg;
    writel_relaxed(BIT(chan_info.doorbell), base + INTR_CLR_OFS);
    }
#[no_mangle]
unsafe extern "C" fn mhu_db_mbox_irq_to_pchan_num(mhu: *mut arm_mhu, irq: c_int) -> c_uint {
    static unsigned int mhu_db_mbox_irq_to_pchan_num(struct arm_mhu *mhu, int irq)
    {
    unsigned int pchan;
    for (pchan = 0; pchan < MHU_CHANS; pchan++)
    if (mhu.mlink[pchan].irq == irq)
    break;
    return pchan;
    }
    static struct mbox_chan *
    mhu_db_mbox_irq_to_channel(struct arm_mhu *mhu, unsigned int pchan)
    {
    unsigned long bits;
    unsigned int doorbell;
    struct mbox_chan *chan = core::ptr::null_mut();
    struct mbox_controller *mbox = &mhu.mbox;
    void __iomem *base = mhu.mlink[pchan].rx_reg;
    bits = readl_relaxed(base + INTR_STAT_OFS);
    if (!bits)
// No IRQs fired in specified physical channel
    return core::ptr::null_mut();
// An IRQ has fired, find the associated channel
    for (doorbell = 0; bits; doorbell++) {
    if (!test_and_clear_bit(doorbell, &bits))
    continue;
    chan = mhu_db_mbox_to_channel(mbox, pchan, doorbell);
    if (chan)
    break;
    dev_err(mbox.dev,
    "Channel not registered: pchan: %d doorbell: %d\n",
    pchan, doorbell);
    }
    return chan;
    }
#[no_mangle]
unsafe extern "C" fn mhu_db_mbox_rx_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t mhu_db_mbox_rx_handler(int irq, void *data)
    {
    struct mbox_chan *chan;
    struct arm_mhu *mhu = data;
    let mut pchan: c_uint = mhu_db_mbox_irq_to_pchan_num(mhu, irq);
    while (core::ptr::null_mut() != (chan = mhu_db_mbox_irq_to_channel(mhu, pchan))) {
    mbox_chan_received_data(chan, core::ptr::null_mut());
    mhu_db_mbox_clear_irq(chan);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn mhu_db_last_tx_done(chan: *mut mbox_chan) -> bool {
    static bool mhu_db_last_tx_done(struct mbox_chan *chan)
    {
    struct mhu_db_channel *chan_info = chan.con_priv;
    void __iomem *base = chan_info.mhu.mlink[chan_info.pchan].tx_reg;
    if (readl_relaxed(base + INTR_STAT_OFS) & BIT(chan_info.doorbell))
    return false;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn mhu_db_send_data(chan: *mut mbox_chan, data: *mut c_void) -> c_int {
    static int mhu_db_send_data(struct mbox_chan *chan, void *data)
    {
    struct mhu_db_channel *chan_info = chan.con_priv;
    void __iomem *base = chan_info.mhu.mlink[chan_info.pchan].tx_reg;
// Send event to co-processor
    writel_relaxed(BIT(chan_info.doorbell), base + INTR_SET_OFS);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mhu_db_startup(chan: *mut mbox_chan) -> c_int {
    static int mhu_db_startup(struct mbox_chan *chan)
    {
    mhu_db_mbox_clear_irq(chan);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mhu_db_shutdown(chan: *mut mbox_chan) {
    static void mhu_db_shutdown(struct mbox_chan *chan)
    {
    struct mhu_db_channel *chan_info = chan.con_priv;
    struct mbox_controller *mbox = &chan_info.mhu.mbox;
    int i;
    for (i = 0; i < mbox.num_chans; i++)
    if (chan == &mbox.chans[i])
    break;
    if (mbox.num_chans == i) {
    dev_warn(mbox.dev, "Request to free non-existent channel\n");
    return;
    }
// Reset channel
    mhu_db_mbox_clear_irq(chan);
    devm_kfree(mbox.dev, chan.con_priv);
    chan.con_priv = core::ptr::null_mut();
    }
    static struct mbox_chan *mhu_db_mbox_xlate(struct mbox_controller *mbox,
    const struct of_phandle_args *spec)
    {
    struct arm_mhu *mhu = dev_get_drvdata(mbox.dev);
    struct mhu_db_channel *chan_info;
    struct mbox_chan *chan;
    let mut pchan: c_uint = spec.args[0];
    let mut doorbell: c_uint = spec.args[1];
    int i;
// Bounds checking
    if (pchan >= MHU_CHANS || doorbell >= MHU_NUM_DOORBELLS) {
    dev_err(mbox.dev,
    "Invalid channel requested pchan: %d doorbell: %d\n",
    pchan, doorbell);
    return ERR_PTR(-EINVAL);
    }
// Is requested channel free?
    chan = mhu_db_mbox_to_channel(mbox, pchan, doorbell);
    if (chan) {
    dev_err(mbox.dev, "Channel in use: pchan: %d doorbell: %d\n",
    pchan, doorbell);
    return ERR_PTR(-EBUSY);
    }
// Find the first free slot
    for (i = 0; i < mbox.num_chans; i++)
    if (!mbox.chans[i].con_priv)
    break;
    if (mbox.num_chans == i) {
    dev_err(mbox.dev, "No free channels left\n");
    return ERR_PTR(-EBUSY);
    }
    chan = &mbox.chans[i];
    chan_info = devm_kzalloc(mbox.dev, sizeof(*chan_info), GFP_KERNEL);
    if (!chan_info)
    return ERR_PTR(-ENOMEM);
    chan_info.mhu = mhu;
    chan_info.pchan = pchan;
    chan_info.doorbell = doorbell;
    chan.con_priv = chan_info;
    dev_dbg(mbox.dev, "mbox: created channel phys: %d doorbell: %d\n",
    pchan, doorbell);
    return chan;
    }
    static const struct mbox_chan_ops mhu_db_ops = {
    .send_data = mhu_db_send_data,
    .startup = mhu_db_startup,
    .shutdown = mhu_db_shutdown,
    .last_tx_done = mhu_db_last_tx_done,
    };
#[no_mangle]
unsafe extern "C" fn mhu_db_probe(adev: *mut amba_device, id: *const amba_id) -> c_int {
    static int mhu_db_probe(struct amba_device *adev, const struct amba_id *id)
    {
    u32 cell_count;
    int i, err, max_chans;
    struct arm_mhu *mhu;
    struct mbox_chan *chans;
    struct device *dev = &adev.dev;
    struct device_node *np = dev.of_node;
    int mhu_reg[MHU_CHANS] = {
    MHU_LP_OFFSET, MHU_HP_OFFSET, MHU_SEC_OFFSET,
    };
    if (!of_device_is_compatible(np, "arm,mhu-doorbell"))
    return -ENODEV;
    err = of_property_read_u32(np, "#mbox-cells", &cell_count);
    if (err) {
    dev_err(dev, "failed to read #mbox-cells in '%pOF'\n", np);
    return err;
    }
    if (cell_count == 2) {
    max_chans = MHU_CHAN_MAX;
    } else {
    dev_err(dev, "incorrect value of #mbox-cells in '%pOF'\n", np);
    return -EINVAL;
    }
    mhu = devm_kzalloc(dev, sizeof(*mhu), GFP_KERNEL);
    if (!mhu)
    return -ENOMEM;
    mhu.base = devm_ioremap_resource(dev, &adev.res);
    if (IS_ERR(mhu.base))
    return PTR_ERR(mhu.base);
    chans = devm_kcalloc(dev, max_chans, sizeof(*chans), GFP_KERNEL);
    if (!chans)
    return -ENOMEM;
    mhu.dev = dev;
    mhu.mbox.dev = dev;
    mhu.mbox.chans = chans;
    mhu.mbox.num_chans = max_chans;
    mhu.mbox.txdone_irq = false;
    mhu.mbox.txdone_poll = true;
    mhu.mbox.txpoll_period = 1;
    mhu.mbox.of_xlate = mhu_db_mbox_xlate;
    amba_set_drvdata(adev, mhu);
    mhu.mbox.ops = &mhu_db_ops;
    err = devm_mbox_controller_register(dev, &mhu.mbox);
    if (err) {
    dev_err(dev, "Failed to register mailboxes %d\n", err);
    return err;
    }
    for (i = 0; i < MHU_CHANS; i++) {
    let mut irq: c_int = mhu.mlink[i].irq = adev.irq[i];
    if (irq <= 0) {
    dev_dbg(dev, "No IRQ found for Channel %d\n", i);
    continue;
    }
    mhu.mlink[i].rx_reg = mhu.base + mhu_reg[i];
    mhu.mlink[i].tx_reg = mhu.mlink[i].rx_reg + TX_REG_OFFSET;
    err = devm_request_threaded_irq(dev, irq, core::ptr::null_mut(),
    mhu_db_mbox_rx_handler,
    IRQF_ONESHOT, "mhu_db_link", mhu);
    if (err) {
    mbox_controller_unregister(&mhu.mbox);
    return err;
    }
    }
    dev_info(dev, "ARM MHU Doorbell mailbox registered\n");
    return 0;
    }
    static const struct amba_id mhu_ids[] = {
    {
    .id	= 0x1bb098,
    .mask	= 0xffffff,
    },
    { 0, 0 },
    };
    MODULE_DEVICE_TABLE(amba, mhu_ids);
    static struct amba_driver arm_mhu_db_driver = {
    .drv = {
    .name	= "mhu-doorbell",
    },
    .id_table	= mhu_ids,
    .probe		= mhu_db_probe,
    };
    module_amba_driver(arm_mhu_db_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("ARM MHU Doorbell Driver");
    MODULE_AUTHOR("Sudeep Holla <sudeep.holla@arm.com>");
