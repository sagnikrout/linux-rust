//! Automatically rewritten from C to Rust
//! Source: drivers/soc/ti/knav_dma.c
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
// Copyright (C) 2014 Texas Instruments Incorporated
// Authors:	Santosh Shilimkar <santosh.shilimkar@ti.com>
// Sandeep Nair <sandeep_n@ti.com>
// Cyril Chemparathy <cyril@ti.com>
//

pub const REG_MASK: c_uint = 0xffffffff;

pub const DMA_TX_PRIO_SHIFT: c_int = 0;
pub const DMA_RX_PRIO_SHIFT: c_int = 16;
pub const DMA_PRIO_DEFAULT: c_int = 0;

pub const DMA_RX_TIMEOUT_SHIFT: c_int = 0;

pub const CHAN_SOP_OFF_SHIFT: c_int = 16;

pub const DESC_TYPE_SHIFT: c_int = 26;

//
// QMGR & QNUM together make up 14 bits with QMGR as the 2 MSb's in the logical
// navigator cloud mapping scheme.
// using the 14bit physical queue numbers directly maps into this scheme.
//

pub const DMA_MAX_QMS: c_int = 4;

pub const DMA_INVALID_ID: c_uint = 0xffff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_global {
    pub revision: u32,
    pub perf_control: u32,
    pub emulation_control: u32,
    pub priority_control: u32,
    pub qm_base_address: [u32; DMA_MAX_QMS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_chan {
    pub control: u32,
    pub mode: u32,
    pub __rsvd: [u32; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_tx_sched {
    pub prio: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_rx_flow {
    pub control: u32,
    pub tags: u32,
    pub tag_sel: u32,
    pub fdq_sel: [u32; 2],
    pub thresh: [u32; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct knav_dma_pool_device {
    pub dev: *mut device,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct knav_dma_device {
    pub enable_all: bool loopback,,
    pub rx_timeout: unsigned tx_priority, rx_priority,,
    pub logical_queue_managers: unsigned,
    pub qm_base_address: [unsigned; DMA_MAX_QMS],
    pub reg_global: *mut reg_global __iomem,
    pub reg_tx_chan: *mut reg_chan __iomem,
    pub reg_rx_flow: *mut reg_rx_flow __iomem,
    pub reg_rx_chan: *mut reg_chan __iomem,
    pub reg_tx_sched: *mut reg_tx_sched __iomem,
    pub max_tx_chan: unsigned max_rx_chan,,
    pub max_rx_flow: unsigned,
    pub name: [c_char; 32],
    pub ref_count: core::sync::atomic::AtomicI32,
    pub list: list_head,
    pub chan_list: list_head,
    pub lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct knav_dma_chan {
    pub direction: enum dma_transfer_direction,
    pub dma: *mut knav_dma_device,
    pub ref_count: core::sync::atomic::AtomicI32,
// registers
    pub reg_chan: *mut reg_chan __iomem,
    pub reg_tx_sched: *mut reg_tx_sched __iomem,
    pub reg_rx_flow: *mut reg_rx_flow __iomem,
// configuration stuff
    pub flow: unsigned channel,,
    pub cfg: knav_dma_cfg,
    pub list: list_head,
    pub lock: spinlock_t,
}

    ch.channel : ch.flow)
    static struct knav_dma_pool_device *kdev;
    static bool device_ready;
#[no_mangle]
pub unsafe extern "C" fn knav_dma_device_ready() -> bool {
    bool knav_dma_device_ready(void)
    {
    return device_ready;
    }
    EXPORT_SYMBOL_GPL(knav_dma_device_ready);
#[no_mangle]
unsafe extern "C" fn check_config(chan: *mut knav_dma_chan, cfg: *mut knav_dma_cfg) -> bool {
    static bool check_config(struct knav_dma_chan *chan, struct knav_dma_cfg *cfg)
    {
    if (!memcmp(&chan.cfg, cfg, sizeof(*cfg)))
    return true;
    else
    return false;
    }
    static int chan_start(struct knav_dma_chan *chan,
    struct knav_dma_cfg *cfg)
    {
    let mut v: u32 = 0;
    spin_lock(&chan.lock);
    if ((chan.direction == DMA_MEM_TO_DEV) && chan.reg_chan) {
    if (cfg.u.tx.filt_pswords)
    v |= DMA_TX_FILT_PSWORDS;
    if (cfg.u.tx.filt_einfo)
    v |= DMA_TX_FILT_EINFO;
    writel_relaxed(v, &chan.reg_chan.mode);
    writel_relaxed(DMA_ENABLE, &chan.reg_chan.control);
    }
    if (chan.reg_tx_sched)
    writel_relaxed(cfg.u.tx.priority, &chan.reg_tx_sched.prio);
    if (chan.reg_rx_flow) {
    v = 0;
    if (cfg.u.rx.einfo_present)
    v |= CHAN_HAS_EPIB;
    if (cfg.u.rx.psinfo_present)
    v |= CHAN_HAS_PSINFO;
    if (cfg.u.rx.err_mode == DMA_RETRY)
    v |= CHAN_ERR_RETRY;
    v |= (cfg.u.rx.desc_type & DESC_TYPE_MASK) << DESC_TYPE_SHIFT;
    if (cfg.u.rx.psinfo_at_sop)
    v |= CHAN_PSINFO_AT_SOP;
    v |= (cfg.u.rx.sop_offset & CHAN_SOP_OFF_MASK)
    << CHAN_SOP_OFF_SHIFT;
    v |= cfg.u.rx.dst_q & CHAN_QNUM_MASK;
    writel_relaxed(v, &chan.reg_rx_flow.control);
    writel_relaxed(0, &chan.reg_rx_flow.tags);
    writel_relaxed(0, &chan.reg_rx_flow.tag_sel);
    v =  cfg.u.rx.fdq[0] << 16;
    v |=  cfg.u.rx.fdq[1] & CHAN_QNUM_MASK;
    writel_relaxed(v, &chan.reg_rx_flow.fdq_sel[0]);
    v =  cfg.u.rx.fdq[2] << 16;
    v |=  cfg.u.rx.fdq[3] & CHAN_QNUM_MASK;
    writel_relaxed(v, &chan.reg_rx_flow.fdq_sel[1]);
    writel_relaxed(0, &chan.reg_rx_flow.thresh[0]);
    writel_relaxed(0, &chan.reg_rx_flow.thresh[1]);
    writel_relaxed(0, &chan.reg_rx_flow.thresh[2]);
    }
// Keep a copy of the cfg
    memcpy(&chan.cfg, cfg, sizeof(*cfg));
    spin_unlock(&chan.lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn chan_teardown(chan: *mut knav_dma_chan) -> c_int {
    static int chan_teardown(struct knav_dma_chan *chan)
    {
    unsigned long end, value;
    if (!chan.reg_chan)
    return 0;
// indicate teardown
    writel_relaxed(DMA_TEARDOWN, &chan.reg_chan.control);
// wait for the dma to shut itself down
    end = jiffies + msecs_to_jiffies(DMA_TIMEOUT);
    do {
    value = readl_relaxed(&chan.reg_chan.control);
    if ((value & DMA_ENABLE) == 0)
    break;
    } while (time_after(end, jiffies));
    if (readl_relaxed(&chan.reg_chan.control) & DMA_ENABLE) {
    dev_err(kdev.dev, "timeout waiting for teardown\n");
    return -ETIMEDOUT;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn chan_stop(chan: *mut knav_dma_chan) {
    static void chan_stop(struct knav_dma_chan *chan)
    {
    spin_lock(&chan.lock);
    if (chan.reg_rx_flow) {
// first detach fdqs, starve out the flow
    writel_relaxed(0, &chan.reg_rx_flow.fdq_sel[0]);
    writel_relaxed(0, &chan.reg_rx_flow.fdq_sel[1]);
    writel_relaxed(0, &chan.reg_rx_flow.thresh[0]);
    writel_relaxed(0, &chan.reg_rx_flow.thresh[1]);
    writel_relaxed(0, &chan.reg_rx_flow.thresh[2]);
    }
// teardown the dma channel
    chan_teardown(chan);
// then disconnect the completion side
    if (chan.reg_rx_flow) {
    writel_relaxed(0, &chan.reg_rx_flow.control);
    writel_relaxed(0, &chan.reg_rx_flow.tags);
    writel_relaxed(0, &chan.reg_rx_flow.tag_sel);
    }
    memset(&chan.cfg, 0, sizeof(struct knav_dma_cfg));
    spin_unlock(&chan.lock);
    dev_dbg(kdev.dev, "channel stopped\n");
    }
#[no_mangle]
unsafe extern "C" fn dma_hw_enable_all(dma: *mut knav_dma_device) {
    static void dma_hw_enable_all(struct knav_dma_device *dma)
    {
    int i;
    for (i = 0; i < dma.max_tx_chan; i++) {
    writel_relaxed(0, &dma.reg_tx_chan[i].mode);
    writel_relaxed(DMA_ENABLE, &dma.reg_tx_chan[i].control);
    }
    }
#[no_mangle]
unsafe extern "C" fn knav_dma_hw_init(dma: *mut knav_dma_device) {
    static void knav_dma_hw_init(struct knav_dma_device *dma)
    {
    unsigned v;
    int i;
    spin_lock(&dma.lock);
    v  = dma.loopback ? DMA_LOOPBACK : 0;
    writel_relaxed(v, &dma.reg_global.emulation_control);
    v = readl_relaxed(&dma.reg_global.perf_control);
    v |= ((dma.rx_timeout & DMA_RX_TIMEOUT_MASK) << DMA_RX_TIMEOUT_SHIFT);
    writel_relaxed(v, &dma.reg_global.perf_control);
    v = ((dma.tx_priority << DMA_TX_PRIO_SHIFT) |
    (dma.rx_priority << DMA_RX_PRIO_SHIFT));
    writel_relaxed(v, &dma.reg_global.priority_control);
// Always enable all Rx channels. Rx paths are managed using flows
    for (i = 0; i < dma.max_rx_chan; i++)
    writel_relaxed(DMA_ENABLE, &dma.reg_rx_chan[i].control);
    for (i = 0; i < dma.logical_queue_managers; i++)
    writel_relaxed(dma.qm_base_address[i],
    &dma.reg_global.qm_base_address[i]);
    spin_unlock(&dma.lock);
    }
#[no_mangle]
unsafe extern "C" fn knav_dma_hw_destroy(dma: *mut knav_dma_device) {
    static void knav_dma_hw_destroy(struct knav_dma_device *dma)
    {
    int i;
    unsigned v;
    spin_lock(&dma.lock);
    v = ~DMA_ENABLE & REG_MASK;
    for (i = 0; i < dma.max_rx_chan; i++)
    writel_relaxed(v, &dma.reg_rx_chan[i].control);
    for (i = 0; i < dma.max_tx_chan; i++)
    writel_relaxed(v, &dma.reg_tx_chan[i].control);
    spin_unlock(&dma.lock);
    }
    static void dma_debug_show_channels(struct seq_file *s,
    struct knav_dma_chan *chan)
    {
    int i;
    seq_printf(s, "\t%s %d:\t",
    ((chan.direction == DMA_MEM_TO_DEV) ? "tx chan" : "rx flow"),
    chan_number(chan));
    if (chan.direction == DMA_MEM_TO_DEV) {
    seq_printf(s, "einfo - %d, pswords - %d, priority - %d\n",
    chan.cfg.u.tx.filt_einfo,
    chan.cfg.u.tx.filt_pswords,
    chan.cfg.u.tx.priority);
    } else {
    seq_printf(s, "einfo - %d, psinfo - %d, desc_type - %d\n",
    chan.cfg.u.rx.einfo_present,
    chan.cfg.u.rx.psinfo_present,
    chan.cfg.u.rx.desc_type);
    seq_printf(s, "\t\t\tdst_q: [%d], thresh: %d fdq: ",
    chan.cfg.u.rx.dst_q,
    chan.cfg.u.rx.thresh);
    for (i = 0; i < KNAV_DMA_FDQ_PER_CHAN; i++)
    seq_printf(s, "[%d]", chan.cfg.u.rx.fdq[i]);
    seq_printf(s, "\n");
    }
    }
    static void dma_debug_show_devices(struct seq_file *s,
    struct knav_dma_device *dma)
    {
    struct knav_dma_chan *chan;
    list_for_each_entry(chan, &dma.chan_list, list) {
    if (atomic_read(&chan.ref_count))
    dma_debug_show_channels(s, chan);
    }
    }
#[no_mangle]
unsafe extern "C" fn knav_dma_debug_show(s: *mut seq_file, v: *mut c_void) -> c_int {
    static int knav_dma_debug_show(struct seq_file *s, void *v)
    {
    struct knav_dma_device *dma;
    list_for_each_entry(dma, &kdev.list, list) {
    if (atomic_read(&dma.ref_count)) {
    seq_printf(s, "%s : max_tx_chan: (%d), max_rx_flows: (%d)\n",
    dma.name, dma.max_tx_chan, dma.max_rx_flow);
    dma_debug_show_devices(s, dma);
    }
    }
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(knav_dma_debug);
    static int of_channel_match_helper(struct device_node *np, const char *name,
    const char **dma_instance)
    {
    struct of_phandle_args args;
    struct device_node *dma_node;
    int index;
    dma_node = of_parse_phandle(np, "ti,navigator-dmas", 0);
    if (!dma_node)
    return -ENODEV;
// dma_instance = dma_node->name;
    index = of_property_match_string(np, "ti,navigator-dma-names", name);
    if (index < 0) {
    dev_err(kdev.dev, "No 'ti,navigator-dma-names' property\n");
    return -ENODEV;
    }
    if (of_parse_phandle_with_fixed_args(np, "ti,navigator-dmas",
    1, index, &args)) {
    dev_err(kdev.dev, "Missing the phandle args name %s\n", name);
    return -ENODEV;
    }
    return args.args[0];
    }
//
// knav_dma_open_channel() - try to setup an exclusive slave channel
// @dev:	pointer to client device structure
// @name:	slave channel name
// @config:	dma configuration parameters
//
// Return: Pointer to appropriate DMA channel on success or NULL on error.
//
    void *knav_dma_open_channel(struct device *dev, const char *name,
    struct knav_dma_cfg *config)
    {
    struct knav_dma_device *dma = core::ptr::null_mut(), *iter1;
    struct knav_dma_chan *chan = core::ptr::null_mut(), *iter2;
    let mut chan_num: c_int = -1;
    const char *instance;
    if (!kdev) {
    pr_err("keystone-navigator-dma driver not registered\n");
    return core::ptr::null_mut();
    }
    chan_num = of_channel_match_helper(dev.of_node, name, &instance);
    if (chan_num < 0) {
    dev_err(kdev.dev, "No DMA instance with name %s\n", name);
    return core::ptr::null_mut();
    }
    dev_dbg(kdev.dev, "initializing %s channel %d from DMA %s\n",
    config.direction == DMA_MEM_TO_DEV ? "transmit" :
    config.direction == DMA_DEV_TO_MEM ? "receive"  :
    "unknown", chan_num, instance);
    if (config.direction != DMA_MEM_TO_DEV &&
    config.direction != DMA_DEV_TO_MEM) {
    dev_err(kdev.dev, "bad direction\n");
    return core::ptr::null_mut();
    }
// Look for correct dma instance
    list_for_each_entry(iter1, &kdev.list, list) {
    if (!strcmp(iter1.name, instance)) {
    dma = iter1;
    break;
    }
    }
    if (!dma) {
    dev_err(kdev.dev, "No DMA instance with name %s\n", instance);
    return core::ptr::null_mut();
    }
// Look for correct dma channel from dma instance
    list_for_each_entry(iter2, &dma.chan_list, list) {
    if (config.direction == DMA_MEM_TO_DEV) {
    if (iter2.channel == chan_num) {
    chan = iter2;
    break;
    }
    } else {
    if (iter2.flow == chan_num) {
    chan = iter2;
    break;
    }
    }
    }
    if (!chan) {
    dev_err(kdev.dev, "channel %d is not in DMA %s\n",
    chan_num, instance);
    return core::ptr::null_mut();
    }
    if (atomic_read(&chan.ref_count) >= 1) {
    if (!check_config(chan, config)) {
    dev_err(kdev.dev, "channel %d config miss-match\n",
    chan_num);
    return core::ptr::null_mut();
    }
    }
    if (atomic_inc_return(&chan.dma.ref_count) <= 1)
    knav_dma_hw_init(chan.dma);
    if (atomic_inc_return(&chan.ref_count) <= 1)
    chan_start(chan, config);
    dev_dbg(kdev.dev, "channel %d opened from DMA %s\n",
    chan_num, instance);
    return chan;
    }
    EXPORT_SYMBOL_GPL(knav_dma_open_channel);
//
// knav_dma_close_channel()	- Destroy a dma channel
//
// @channel:	dma channel handle
//
#[no_mangle]
pub unsafe extern "C" fn knav_dma_close_channel(channel: *mut c_void) {
    void knav_dma_close_channel(void *channel)
    {
    struct knav_dma_chan *chan = channel;
    if (!kdev) {
    pr_err("keystone-navigator-dma driver not registered\n");
    return;
    }
    if (atomic_dec_return(&chan.ref_count) <= 0)
    chan_stop(chan);
    if (atomic_dec_return(&chan.dma.ref_count) <= 0)
    knav_dma_hw_destroy(chan.dma);
    dev_dbg(kdev.dev, "channel %d or flow %d closed from DMA %s\n",
    chan.channel, chan.flow, chan.dma.name);
    }
    EXPORT_SYMBOL_GPL(knav_dma_close_channel);
    static void __iomem *pktdma_get_regs(struct knav_dma_device *dma,
    struct device_node *node,
    unsigned index, resource_size_t *_size)
    {
    struct device *dev = kdev.dev;
    struct resource res;
    void __iomem *regs;
    int ret;
    ret = of_address_to_resource(node, index, &res);
    if (ret) {
    dev_err(dev, "Can't translate of node(%pOFn) address for index(%d)\n",
    node, index);
    return IOMEM_ERR_PTR(ret);
    }
    regs = devm_ioremap_resource(kdev.dev, &res);
    if (IS_ERR(regs))
    dev_err(dev, "Failed to map register base for index(%d) node(%pOFn)\n",
    index, node);
    if (_size)
// _size = resource_size(&res);
    return regs;
    }
#[no_mangle]
unsafe extern "C" fn pktdma_init_rx_chan(chan: *mut knav_dma_chan, flow: u32) -> c_int {
    static int pktdma_init_rx_chan(struct knav_dma_chan *chan, u32 flow)
    {
    struct knav_dma_device *dma = chan.dma;
    chan.flow = flow;
    chan.reg_rx_flow = dma.reg_rx_flow + flow;
    chan.channel = DMA_INVALID_ID;
    dev_dbg(kdev.dev, "rx flow(%d) (%p)\n", chan.flow, chan.reg_rx_flow);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pktdma_init_tx_chan(chan: *mut knav_dma_chan, channel: u32) -> c_int {
    static int pktdma_init_tx_chan(struct knav_dma_chan *chan, u32 channel)
    {
    struct knav_dma_device *dma = chan.dma;
    chan.channel = channel;
    chan.reg_chan = dma.reg_tx_chan + channel;
    chan.reg_tx_sched = dma.reg_tx_sched + channel;
    chan.flow = DMA_INVALID_ID;
    dev_dbg(kdev.dev, "tx channel(%d) (%p)\n", chan.channel, chan.reg_chan);
    return 0;
    }
    static int pktdma_init_chan(struct knav_dma_device *dma,
    enum dma_transfer_direction dir,
    unsigned chan_num)
    {
    struct device *dev = kdev.dev;
    struct knav_dma_chan *chan;
    let mut ret: c_int = -EINVAL;
    chan = devm_kzalloc(dev, sizeof(*chan), GFP_KERNEL);
    if (!chan)
    return -ENOMEM;
    INIT_LIST_HEAD(&chan.list);
    chan.dma	= dma;
    chan.direction	= DMA_TRANS_NONE;
    atomic_set(&chan.ref_count, 0);
    spin_lock_init(&chan.lock);
    if (dir == DMA_MEM_TO_DEV) {
    chan.direction = dir;
    ret = pktdma_init_tx_chan(chan, chan_num);
    } else if (dir == DMA_DEV_TO_MEM) {
    chan.direction = dir;
    ret = pktdma_init_rx_chan(chan, chan_num);
    } else {
    dev_err(dev, "channel(%d) direction unknown\n", chan_num);
    }
    list_add_tail(&chan.list, &dma.chan_list);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dma_init(cloud: *mut device_node, dma_node: *mut device_node) -> c_int {
    static int dma_init(struct device_node *cloud, struct device_node *dma_node)
    {
    unsigned max_tx_chan, max_rx_chan, max_rx_flow, max_tx_sched;
    struct device_node *node = dma_node;
    struct knav_dma_device *dma;
    int ret, num_chan = 0;
    resource_size_t size;
    u32 timeout;
    u32 i;
    dma = devm_kzalloc(kdev.dev, sizeof(*dma), GFP_KERNEL);
    if (!dma) {
    dev_err(kdev.dev, "could not allocate driver mem\n");
    return -ENOMEM;
    }
    INIT_LIST_HEAD(&dma.list);
    INIT_LIST_HEAD(&dma.chan_list);
    ret = of_property_read_variable_u32_array(cloud, "ti,navigator-cloud-address",
    dma.qm_base_address, 1, DMA_MAX_QMS);
    if (ret < 0) {
    dev_err(kdev.dev, "invalid navigator cloud addresses\n");
    return -ENODEV;
    }
    dma.logical_queue_managers = ret;
    dma.reg_global	 = pktdma_get_regs(dma, node, 0, &size);
    if (IS_ERR(dma.reg_global))
    return PTR_ERR(dma.reg_global);
    if (size < sizeof(struct reg_global)) {
    dev_err(kdev.dev, "bad size %pa for global regs\n", &size);
    return -ENODEV;
    }
    dma.reg_tx_chan = pktdma_get_regs(dma, node, 1, &size);
    if (IS_ERR(dma.reg_tx_chan))
    return PTR_ERR(dma.reg_tx_chan);
    max_tx_chan = size / sizeof(struct reg_chan);
    dma.reg_rx_chan = pktdma_get_regs(dma, node, 2, &size);
    if (IS_ERR(dma.reg_rx_chan))
    return PTR_ERR(dma.reg_rx_chan);
    max_rx_chan = size / sizeof(struct reg_chan);
    dma.reg_tx_sched = pktdma_get_regs(dma, node, 3, &size);
    if (IS_ERR(dma.reg_tx_sched))
    return PTR_ERR(dma.reg_tx_sched);
    max_tx_sched = size / sizeof(struct reg_tx_sched);
    dma.reg_rx_flow = pktdma_get_regs(dma, node, 4, &size);
    if (IS_ERR(dma.reg_rx_flow))
    return PTR_ERR(dma.reg_rx_flow);
    max_rx_flow = size / sizeof(struct reg_rx_flow);
    dma.rx_priority = DMA_PRIO_DEFAULT;
    dma.tx_priority = DMA_PRIO_DEFAULT;
    dma.enable_all	= of_property_read_bool(node, "ti,enable-all");
    dma.loopback	= of_property_read_bool(node, "ti,loop-back");
    ret = of_property_read_u32(node, "ti,rx-retry-timeout", &timeout);
    if (ret < 0) {
    dev_dbg(kdev.dev, "unspecified rx timeout using value %d\n",
    DMA_RX_TIMEOUT_DEFAULT);
    timeout = DMA_RX_TIMEOUT_DEFAULT;
    }
    dma.rx_timeout = timeout;
    dma.max_rx_chan = max_rx_chan;
    dma.max_rx_flow = max_rx_flow;
    dma.max_tx_chan = min(max_tx_chan, max_tx_sched);
    atomic_set(&dma.ref_count, 0);
    strcpy(dma.name, node.name);
    spin_lock_init(&dma.lock);
    for (i = 0; i < dma.max_tx_chan; i++) {
    if (pktdma_init_chan(dma, DMA_MEM_TO_DEV, i) >= 0)
    num_chan++;
    }
    for (i = 0; i < dma.max_rx_flow; i++) {
    if (pktdma_init_chan(dma, DMA_DEV_TO_MEM, i) >= 0)
    num_chan++;
    }
    list_add_tail(&dma.list, &kdev.list);
//
// For DSP software usecases or userpace transport software, setup all
// the DMA hardware resources.
//
    if (dma.enable_all) {
    atomic_inc(&dma.ref_count);
    knav_dma_hw_init(dma);
    dma_hw_enable_all(dma);
    }
    dev_info(kdev.dev, "DMA %s registered %d logical channels, flows %d, tx chans: %d, rx chans: %d%s\n",
    dma.name, num_chan, dma.max_rx_flow,
    dma.max_tx_chan, dma.max_rx_chan,
    dma.loopback ? ", loopback" : "");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn knav_dma_probe(pdev: *mut platform_device) -> c_int {
    static int knav_dma_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *node = pdev.dev.of_node;
    let mut ret: c_int = 0;
    if (!node)
    return dev_err_probe(dev, -EINVAL, "could not find device info\n");
    kdev = devm_kzalloc(dev,
    sizeof(struct knav_dma_pool_device), GFP_KERNEL);
    if (!kdev)
    return -ENOMEM;
    kdev.dev = dev;
    INIT_LIST_HEAD(&kdev.list);
    pm_runtime_enable(kdev.dev);
    ret = pm_runtime_resume_and_get(kdev.dev);
    if (ret < 0) {
    dev_err(dev, "unable to enable pktdma, err %d\n", ret);
    goto err_pm_disable;
    }
// Initialise all packet dmas
    for_each_child_of_node_scoped(node, child) {
    ret = dma_init(node, child);
    if (ret) {
    dev_err(dev, "init failed with %d\n", ret);
    break;
    }
    }
    if (list_empty(&kdev.list)) {
    ret = dev_err_probe(dev, -ENODEV, "no valid dma instance\n");
    goto err_put_sync;
    }
    debugfs_create_file("knav_dma", S_IFREG | S_IRUGO, core::ptr::null_mut(), core::ptr::null_mut(),
    &knav_dma_debug_fops);
    device_ready = true;
    return ret;
    err_put_sync:
    pm_runtime_put_sync(kdev.dev);
    err_pm_disable:
    pm_runtime_disable(kdev.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn knav_dma_remove(pdev: *mut platform_device) {
    static void knav_dma_remove(struct platform_device *pdev)
    {
    struct knav_dma_device *dma;
    list_for_each_entry(dma, &kdev.list, list) {
    if (atomic_dec_return(&dma.ref_count) == 0)
    knav_dma_hw_destroy(dma);
    }
    pm_runtime_put_sync(&pdev.dev);
    pm_runtime_disable(&pdev.dev);
    }
    static struct of_device_id of_match[] = {
    { .compatible = "ti,keystone-navigator-dma", },
    {},
    };
    MODULE_DEVICE_TABLE(of, of_match);
    static struct platform_driver knav_dma_driver = {
    .probe	= knav_dma_probe,
    .remove	= knav_dma_remove,
    .driver	= {
    .name		= "keystone-navigator-dma",
    .of_match_table	= of_match,
    },
    };
    module_platform_driver(knav_dma_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("TI Keystone Navigator Packet DMA driver");
    MODULE_AUTHOR("Sandeep Nair <sandeep_n@ti.com>");
    MODULE_AUTHOR("Santosh Shilimkar <santosh.shilimkar@ti.com>");
