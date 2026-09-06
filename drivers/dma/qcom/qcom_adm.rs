//! Automatically rewritten from C to Rust
//! Source: drivers/dma/qcom/qcom_adm.c
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
// Copyright (c) 2013-2015, The Linux Foundation. All rights reserved.
//

// ADM registers - calculated from channel number and security domain
pub const ADM_CHAN_MULTI: c_uint = 0x4;
pub const ADM_CI_MULTI: c_uint = 0x4;
pub const ADM_CRCI_MULTI: c_uint = 0x4;
pub const ADM_EE_MULTI: c_uint = 0x800;

pub const ADM_GP_CTL: c_uint = 0x3d8;

    ADM_EE_OFFS(ee))
// channel status

// channel result

// channel conf

// channel result conf

// CRCI CTL

// CI configuration

// GP CTL

// Command pointer list entry

// Command list entry

pub const ADM_CMD_TYPE_SINGLE: c_uint = 0x0;
pub const ADM_CMD_TYPE_BOX: c_uint = 0x3;

pub const ADM_DESC_ALIGN: c_int = 8;

pub const ADM_MAX_CHANNELS: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adm_desc_hw_box {
    pub cmd: u32,
    pub src_addr: u32,
    pub dst_addr: u32,
    pub row_len: u32,
    pub num_rows: u32,
    pub row_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adm_desc_hw_single {
    pub cmd: u32,
    pub src_addr: u32,
    pub dst_addr: u32,
    pub len: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adm_async_desc {
    pub vd: virt_dma_desc,
    pub adev: *mut adm_device,
    pub length: usize,
    pub dir: enum dma_transfer_direction,
    pub dma_addr: dma_addr_t,
    pub dma_len: usize,
    pub cpl: *mut c_void,
    pub cp_addr: dma_addr_t,
    pub crci: u32,
    pub mux: u32,
    pub blk_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct adm_chan {
    pub vc: virt_dma_chan,
    pub adev: *mut adm_device,
// parsed from DT
    pub /: *mut *mut u32 id; / channel id,
    pub curr_txd: *mut adm_async_desc,
    pub slave: dma_slave_config,
    pub crci: u32,
    pub mux: u32,
    pub node: list_head,
    pub error: c_int,
    pub initialized: c_int,
}

    static inline struct adm_chan *to_adm_chan(struct dma_chan *common)
    {
    return container_of(common, struct adm_chan, vc.chan);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adm_device {
    pub regs: *mut void __iomem,
    pub dev: *mut device,
    pub common: dma_device,
    pub dma_parms: device_dma_parameters,
    pub channels: *mut adm_chan,
    pub ee: u32,
    pub core_clk: *mut clk,
    pub iface_clk: *mut clk,
    pub clk_reset: *mut reset_control,
    pub c0_reset: *mut reset_control,
    pub c1_reset: *mut reset_control,
    pub c2_reset: *mut reset_control,
    pub irq: c_int,
}

//
// adm_free_chan - Frees dma resources associated with the specific channel
//
// @chan: dma channel
//
// Free all allocated descriptors associated with this channel
//
#[no_mangle]
unsafe extern "C" fn adm_free_chan(chan: *mut dma_chan) {
    static void adm_free_chan(struct dma_chan *chan)
    {
// free all queued descriptors
    vchan_free_chan_resources(to_virt_chan(chan));
    }
//
// adm_get_blksize - Get block size from burst value
//
// @burst: Burst size of transaction
//
#[no_mangle]
unsafe extern "C" fn adm_get_blksize(burst: c_uint) -> c_int {
    static int adm_get_blksize(unsigned int burst)
    {
    int ret;
    switch (burst) {
    case 16:
    case 32:
    case 64:
    case 128:
    ret = ffs(burst >> 4) - 1;
    break;
    case 192:
    ret = 4;
    break;
    case 256:
    ret = 5;
    break;
    default:
    ret = -EINVAL;
    break;
    }
    return ret;
    }
//
// adm_process_fc_descriptors - Process descriptors for flow controlled xfers
//
// @achan: ADM channel
// @desc: Descriptor memory pointer
// @sg: Scatterlist entry
// @crci: CRCI value
// @burst: Burst size of transaction
// @direction: DMA transfer direction
//
    static void *adm_process_fc_descriptors(struct adm_chan *achan, void *desc,
    struct scatterlist *sg, u32 crci,
    u32 burst,
    enum dma_transfer_direction direction)
    {
    struct adm_desc_hw_box *box_desc = core::ptr::null_mut();
    struct adm_desc_hw_single *single_desc;
    let mut remainder: u32 = sg_dma_len(sg);
    u32 rows, row_offset, crci_cmd;
    let mut mem_addr: u32 = sg_dma_address(sg);
    u32 *incr_addr = &mem_addr;
    u32 *src, *dst;
    if (direction == DMA_DEV_TO_MEM) {
    crci_cmd = ADM_CMD_SRC_CRCI(crci);
    row_offset = burst;
    src = &achan.slave.src_addr;
    dst = &mem_addr;
    } else {
    crci_cmd = ADM_CMD_DST_CRCI(crci);
    row_offset = burst << 16;
    src = &mem_addr;
    dst = &achan.slave.dst_addr;
    }
    while (remainder >= burst) {
    box_desc = desc;
    box_desc.cmd = ADM_CMD_TYPE_BOX | crci_cmd;
    box_desc.row_offset = row_offset;
    box_desc.src_addr = *src;
    box_desc.dst_addr = *dst;
    rows = remainder / burst;
    rows = min_t(u32, rows, ADM_MAX_ROWS);
    box_desc.num_rows = rows << 16 | rows;
    box_desc.row_len = burst << 16 | burst;
// incr_addr += burst * rows;
    remainder -= burst * rows;
    desc += sizeof(*box_desc);
    }
// if leftover bytes, do one single descriptor
    if (remainder) {
    single_desc = desc;
    single_desc.cmd = ADM_CMD_TYPE_SINGLE | crci_cmd;
    single_desc.len = remainder;
    single_desc.src_addr = *src;
    single_desc.dst_addr = *dst;
    desc += sizeof(*single_desc);
    if (sg_is_last(sg))
    single_desc.cmd |= ADM_CMD_LC;
    } else {
    if (box_desc && sg_is_last(sg))
    box_desc.cmd |= ADM_CMD_LC;
    }
    return desc;
    }
//
// adm_process_non_fc_descriptors - Process descriptors for non-fc xfers
//
// @achan: ADM channel
// @desc: Descriptor memory pointer
// @sg: Scatterlist entry
// @direction: DMA transfer direction
//
    static void *adm_process_non_fc_descriptors(struct adm_chan *achan, void *desc,
    struct scatterlist *sg,
    enum dma_transfer_direction direction)
    {
    struct adm_desc_hw_single *single_desc;
    let mut remainder: u32 = sg_dma_len(sg);
    let mut mem_addr: u32 = sg_dma_address(sg);
    u32 *incr_addr = &mem_addr;
    u32 *src, *dst;
    if (direction == DMA_DEV_TO_MEM) {
    src = &achan.slave.src_addr;
    dst = &mem_addr;
    } else {
    src = &mem_addr;
    dst = &achan.slave.dst_addr;
    }
    do {
    single_desc = desc;
    single_desc.cmd = ADM_CMD_TYPE_SINGLE;
    single_desc.src_addr = *src;
    single_desc.dst_addr = *dst;
    single_desc.len = (remainder > ADM_MAX_XFER) ?
    ADM_MAX_XFER : remainder;
    remainder -= single_desc.len;
// incr_addr += single_desc->len;
    desc += sizeof(*single_desc);
    } while (remainder);
// set last command if this is the end of the whole transaction
    if (sg_is_last(sg))
    single_desc.cmd |= ADM_CMD_LC;
    return desc;
    }
//
// adm_prep_slave_sg - Prep slave sg transaction
//
// @chan: dma channel
// @sgl: scatter gather list
// @sg_len: length of sg
// @direction: DMA transfer direction
// @flags: DMA flags
// @context: transfer context (unused)
//
    static struct dma_async_tx_descriptor *adm_prep_slave_sg(struct dma_chan *chan,
    struct scatterlist *sgl,
    unsigned int sg_len,
    enum dma_transfer_direction direction,
    unsigned long flags,
    void *context)
    {
    struct adm_chan *achan = to_adm_chan(chan);
    struct adm_device *adev = achan.adev;
    struct adm_async_desc *async_desc;
    struct scatterlist *sg;
    dma_addr_t cple_addr;
    u32 i, burst;
    let mut single_count: u32 = 0, box_count = 0, crci = 0;
    void *desc;
    u32 *cple;
    let mut blk_size: c_int = 0;
    if (!is_slave_direction(direction)) {
    dev_err(adev.dev, "invalid dma direction\n");
    return core::ptr::null_mut();
    }
//
// get burst value from slave configuration
//
    burst = (direction == DMA_MEM_TO_DEV) ?
    achan.slave.dst_maxburst :
    achan.slave.src_maxburst;
// if using flow control, validate burst and crci values
    if (achan.slave.device_fc) {
    blk_size = adm_get_blksize(burst);
    if (blk_size < 0) {
    dev_err(adev.dev, "invalid burst value: %d\n",
    burst);
    return core::ptr::null_mut();
    }
    crci = achan.crci & 0xf;
    if (!crci || achan.crci > 0x1f) {
    dev_err(adev.dev, "invalid crci value\n");
    return core::ptr::null_mut();
    }
    }
// iterate through sgs and compute allocation size of structures
    if (achan.slave.device_fc) {
    for_each_sg(sgl, sg, sg_len, i) {
    box_count += DIV_ROUND_UP(sg_dma_len(sg) / burst,
    ADM_MAX_ROWS);
    if (sg_dma_len(sg) % burst)
    single_count++;
    }
    } else {
    single_count = sg_nents_for_dma(sgl, sg_len, ADM_MAX_XFER);
    }
    async_desc = kzalloc_obj(*async_desc, GFP_NOWAIT);
    if (!async_desc) {
    dev_err(adev.dev, "not enough memory for async_desc struct\n");
    return core::ptr::null_mut();
    }
    async_desc.mux = achan.mux ? ADM_CRCI_CTL_MUX_SEL : 0;
    async_desc.crci = crci;
    async_desc.blk_size = blk_size;
    async_desc.dma_len = single_count * sizeof(struct adm_desc_hw_single) +
    box_count * sizeof(struct adm_desc_hw_box) +
    sizeof(*cple) + 2 * ADM_DESC_ALIGN;
    async_desc.cpl = kzalloc(async_desc.dma_len, GFP_NOWAIT);
    if (!async_desc.cpl) {
    dev_err(adev.dev, "not enough memory for cpl struct\n");
    goto free;
    }
    async_desc.adev = adev;
// both command list entry and descriptors must be 8 byte aligned
    cple = PTR_ALIGN(async_desc.cpl, ADM_DESC_ALIGN);
    desc = PTR_ALIGN(cple + 1, ADM_DESC_ALIGN);
    for_each_sg(sgl, sg, sg_len, i) {
    async_desc.length += sg_dma_len(sg);
    if (achan.slave.device_fc)
    desc = adm_process_fc_descriptors(achan, desc, sg, crci,
    burst, direction);
    else
    desc = adm_process_non_fc_descriptors(achan, desc, sg,
    direction);
    }
    async_desc.dma_addr = dma_map_single(adev.dev, async_desc.cpl,
    async_desc.dma_len,
    DMA_TO_DEVICE);
    if (dma_mapping_error(adev.dev, async_desc.dma_addr)) {
    dev_err(adev.dev, "dma mapping error for cpl\n");
    goto free;
    }
    cple_addr = async_desc.dma_addr + ((void *)cple - async_desc.cpl);
// init cmd list
    dma_sync_single_for_cpu(adev.dev, cple_addr, sizeof(*cple),
    DMA_TO_DEVICE);
// cple = ADM_CPLE_LP;
// cple |= (async_desc->dma_addr + ADM_DESC_ALIGN) >> 3;
    dma_sync_single_for_device(adev.dev, cple_addr, sizeof(*cple),
    DMA_TO_DEVICE);
    return vchan_tx_prep(&achan.vc, &async_desc.vd, flags);
    free:
    kfree(async_desc);
    return core::ptr::null_mut();
    }
//
// adm_terminate_all - terminate all transactions on a channel
// @chan: dma channel
//
// Dequeues and frees all transactions, aborts current transaction
// No callbacks are done
//
#[no_mangle]
unsafe extern "C" fn adm_terminate_all(chan: *mut dma_chan) -> c_int {
    static int adm_terminate_all(struct dma_chan *chan)
    {
    struct adm_chan *achan = to_adm_chan(chan);
    struct adm_device *adev = achan.adev;
    unsigned long flags;
    LIST_HEAD(head);
    spin_lock_irqsave(&achan.vc.lock, flags);
    vchan_get_all_descriptors(&achan.vc, &head);
// send flush command to terminate current transaction
    writel_relaxed(0x0,
    adev.regs + ADM_CH_FLUSH_STATE0(achan.id, adev.ee));
    spin_unlock_irqrestore(&achan.vc.lock, flags);
    vchan_dma_desc_free_list(&achan.vc, &head);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adm_slave_config(chan: *mut dma_chan, cfg: *mut dma_slave_config) -> c_int {
    static int adm_slave_config(struct dma_chan *chan, struct dma_slave_config *cfg)
    {
    struct adm_chan *achan = to_adm_chan(chan);
    struct qcom_adm_peripheral_config *config = cfg.peripheral_config;
    unsigned long flag;
    spin_lock_irqsave(&achan.vc.lock, flag);
    memcpy(&achan.slave, cfg, sizeof(struct dma_slave_config));
    if (cfg.peripheral_size == sizeof(*config))
    achan.crci = config.crci;
    spin_unlock_irqrestore(&achan.vc.lock, flag);
    return 0;
    }
//
// adm_start_dma - start next transaction
// @achan: ADM dma channel
//
#[no_mangle]
unsafe extern "C" fn adm_start_dma(achan: *mut adm_chan) {
    static void adm_start_dma(struct adm_chan *achan)
    {
    struct virt_dma_desc *vd = vchan_next_desc(&achan.vc);
    struct adm_device *adev = achan.adev;
    struct adm_async_desc *async_desc;
    lockdep_assert_held(&achan.vc.lock);
    if (!vd)
    return;
    list_del(&vd.node);
// write next command list out to the CMD FIFO
    async_desc = container_of(vd, struct adm_async_desc, vd);
    achan.curr_txd = async_desc;
// reset channel error
    achan.error = 0;
    if (!achan.initialized) {
// enable interrupts
    writel(ADM_CH_CONF_SHADOW_EN |
    ADM_CH_CONF_PERM_MPU_CONF |
    ADM_CH_CONF_MPU_DISABLE |
    ADM_CH_CONF_SEC_DOMAIN(adev.ee),
    adev.regs + ADM_CH_CONF(achan.id));
    writel(ADM_CH_RSLT_CONF_IRQ_EN | ADM_CH_RSLT_CONF_FLUSH_EN,
    adev.regs + ADM_CH_RSLT_CONF(achan.id, adev.ee));
    achan.initialized = 1;
    }
// set the crci block size if this transaction requires CRCI
    if (async_desc.crci) {
    writel(async_desc.mux | async_desc.blk_size,
    adev.regs + ADM_CRCI_CTL(async_desc.crci, adev.ee));
    }
// make sure IRQ enable doesn't get reordered
    wmb();
// write next command list out to the CMD FIFO
    writel(ALIGN(async_desc.dma_addr, ADM_DESC_ALIGN) >> 3,
    adev.regs + ADM_CH_CMD_PTR(achan.id, adev.ee));
    }
//
// adm_dma_irq - irq handler for ADM controller
// @irq: IRQ of interrupt
// @data: callback data
//
// IRQ handler for the bam controller
//
#[no_mangle]
unsafe extern "C" fn adm_dma_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t adm_dma_irq(int irq, void *data)
    {
    struct adm_device *adev = data;
    u32 srcs, i;
    struct adm_async_desc *async_desc;
    unsigned long flags;
    srcs = readl_relaxed(adev.regs +
    ADM_SEC_DOMAIN_IRQ_STATUS(adev.ee));
    for (i = 0; i < ADM_MAX_CHANNELS; i++) {
    struct adm_chan *achan = &adev.channels[i];
    u32 status, result;
    if (srcs & BIT(i)) {
    status = readl_relaxed(adev.regs +
    ADM_CH_STATUS_SD(i, adev.ee));
// if no result present, skip
    if (!(status & ADM_CH_STATUS_VALID))
    continue;
    result = readl_relaxed(adev.regs +
    ADM_CH_RSLT(i, adev.ee));
// no valid results, skip
    if (!(result & ADM_CH_RSLT_VALID))
    continue;
// flag error if transaction was flushed or failed
    if (result & (ADM_CH_RSLT_ERR | ADM_CH_RSLT_FLUSH))
    achan.error = 1;
    spin_lock_irqsave(&achan.vc.lock, flags);
    async_desc = achan.curr_txd;
    achan.curr_txd = core::ptr::null_mut();
    if (async_desc) {
    vchan_cookie_complete(&async_desc.vd);
// kick off next DMA
    adm_start_dma(achan);
    }
    spin_unlock_irqrestore(&achan.vc.lock, flags);
    }
    }
    return IRQ_HANDLED;
    }
//
// adm_tx_status - returns status of transaction
// @chan: dma channel
// @cookie: transaction cookie
// @txstate: DMA transaction state
//
// Return status of dma transaction
//
    static enum dma_status adm_tx_status(struct dma_chan *chan, dma_cookie_t cookie,
    struct dma_tx_state *txstate)
    {
    struct adm_chan *achan = to_adm_chan(chan);
    struct virt_dma_desc *vd;
    enum dma_status ret;
    unsigned long flags;
    let mut residue: usize = 0;
    ret = dma_cookie_status(chan, cookie, txstate);
    if (ret == DMA_COMPLETE || !txstate)
    return ret;
    spin_lock_irqsave(&achan.vc.lock, flags);
    vd = vchan_find_desc(&achan.vc, cookie);
    if (vd)
    residue = container_of(vd, struct adm_async_desc, vd).length;
    spin_unlock_irqrestore(&achan.vc.lock, flags);
//
// residue is either the full length if it is in the issued list, or 0
// if it is in progress.  We have no reliable way of determining
// anything in between
//
    dma_set_residue(txstate, residue);
    if (achan.error)
    return DMA_ERROR;
    return ret;
    }
//
// adm_issue_pending - starts pending transactions
// @chan: dma channel
//
// Issues all pending transactions and starts DMA
//
#[no_mangle]
unsafe extern "C" fn adm_issue_pending(chan: *mut dma_chan) {
    static void adm_issue_pending(struct dma_chan *chan)
    {
    struct adm_chan *achan = to_adm_chan(chan);
    unsigned long flags;
    spin_lock_irqsave(&achan.vc.lock, flags);
    if (vchan_issue_pending(&achan.vc) && !achan.curr_txd)
    adm_start_dma(achan);
    spin_unlock_irqrestore(&achan.vc.lock, flags);
    }
//
// adm_dma_free_desc - free descriptor memory
// @vd: virtual descriptor
//
#[no_mangle]
unsafe extern "C" fn adm_dma_free_desc(vd: *mut virt_dma_desc) {
    static void adm_dma_free_desc(struct virt_dma_desc *vd)
    {
    struct adm_async_desc *async_desc = container_of(vd,
    struct adm_async_desc, vd);
    dma_unmap_single(async_desc.adev.dev, async_desc.dma_addr,
    async_desc.dma_len, DMA_TO_DEVICE);
    kfree(async_desc.cpl);
    kfree(async_desc);
    }
    static void adm_channel_init(struct adm_device *adev, struct adm_chan *achan,
    u32 index)
    {
    achan.id = index;
    achan.adev = adev;
    vchan_init(&achan.vc, &adev.common);
    achan.vc.desc_free = adm_dma_free_desc;
    }
//
// adm_dma_xlate
// @dma_spec:	pointer to DMA specifier as found in the device tree
// @ofdma:	pointer to DMA controller data
//
// This can use either 1-cell or 2-cell formats, the first cell
// identifies the slave device, while the optional second cell
// contains the crci value.
//
// Returns pointer to appropriate dma channel on success or NULL on error.
//
    static struct dma_chan *adm_dma_xlate(struct of_phandle_args *dma_spec,
    struct of_dma *ofdma)
    {
    struct dma_device *dev = ofdma.of_dma_data;
    struct dma_chan *chan, *candidate = core::ptr::null_mut();
    struct adm_chan *achan;
    if (!dev || dma_spec.args_count > 2)
    return core::ptr::null_mut();
    list_for_each_entry(chan, &dev.channels, device_node)
    if (chan.chan_id == dma_spec.args[0]) {
    candidate = chan;
    break;
    }
    if (!candidate)
    return core::ptr::null_mut();
    achan = to_adm_chan(candidate);
    if (dma_spec.args_count == 2)
    achan.crci = dma_spec.args[1];
    else
    achan.crci = 0;
    return dma_get_slave_channel(candidate);
    }
#[no_mangle]
unsafe extern "C" fn adm_dma_probe(pdev: *mut platform_device) -> c_int {
    static int adm_dma_probe(struct platform_device *pdev)
    {
    struct adm_device *adev;
    int ret;
    u32 i;
    adev = devm_kzalloc(&pdev.dev, sizeof(*adev), GFP_KERNEL);
    if (!adev)
    return -ENOMEM;
    adev.dev = &pdev.dev;
    adev.regs = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(adev.regs))
    return PTR_ERR(adev.regs);
    adev.irq = platform_get_irq(pdev, 0);
    if (adev.irq < 0)
    return adev.irq;
    ret = of_property_read_u32(pdev.dev.of_node, "qcom,ee", &adev.ee);
    if (ret) {
    dev_err(adev.dev, "Execution environment unspecified\n");
    return ret;
    }
    adev.core_clk = devm_clk_get(adev.dev, "core");
    if (IS_ERR(adev.core_clk))
    return PTR_ERR(adev.core_clk);
    adev.iface_clk = devm_clk_get(adev.dev, "iface");
    if (IS_ERR(adev.iface_clk))
    return PTR_ERR(adev.iface_clk);
    adev.clk_reset = devm_reset_control_get_exclusive(&pdev.dev, "clk");
    if (IS_ERR(adev.clk_reset)) {
    dev_err(adev.dev, "failed to get ADM0 reset\n");
    return PTR_ERR(adev.clk_reset);
    }
    adev.c0_reset = devm_reset_control_get_exclusive(&pdev.dev, "c0");
    if (IS_ERR(adev.c0_reset)) {
    dev_err(adev.dev, "failed to get ADM0 C0 reset\n");
    return PTR_ERR(adev.c0_reset);
    }
    adev.c1_reset = devm_reset_control_get_exclusive(&pdev.dev, "c1");
    if (IS_ERR(adev.c1_reset)) {
    dev_err(adev.dev, "failed to get ADM0 C1 reset\n");
    return PTR_ERR(adev.c1_reset);
    }
    adev.c2_reset = devm_reset_control_get_exclusive(&pdev.dev, "c2");
    if (IS_ERR(adev.c2_reset)) {
    dev_err(adev.dev, "failed to get ADM0 C2 reset\n");
    return PTR_ERR(adev.c2_reset);
    }
    ret = clk_prepare_enable(adev.core_clk);
    if (ret) {
    dev_err(adev.dev, "failed to prepare/enable core clock\n");
    return ret;
    }
    ret = clk_prepare_enable(adev.iface_clk);
    if (ret) {
    dev_err(adev.dev, "failed to prepare/enable iface clock\n");
    goto err_disable_core_clk;
    }
    reset_control_assert(adev.clk_reset);
    reset_control_assert(adev.c0_reset);
    reset_control_assert(adev.c1_reset);
    reset_control_assert(adev.c2_reset);
    udelay(2);
    reset_control_deassert(adev.clk_reset);
    reset_control_deassert(adev.c0_reset);
    reset_control_deassert(adev.c1_reset);
    reset_control_deassert(adev.c2_reset);
    adev.channels = devm_kcalloc(adev.dev, ADM_MAX_CHANNELS,
    sizeof(*adev.channels), GFP_KERNEL);
    if (!adev.channels) {
    ret = -ENOMEM;
    goto err_disable_clks;
    }
// allocate and initialize channels
    INIT_LIST_HEAD(&adev.common.channels);
    for (i = 0; i < ADM_MAX_CHANNELS; i++)
    adm_channel_init(adev, &adev.channels[i], i);
// reset CRCIs
    for (i = 0; i < 16; i++)
    writel(ADM_CRCI_CTL_RST, adev.regs +
    ADM_CRCI_CTL(i, adev.ee));
// configure client interfaces
    writel(ADM_CI_RANGE_START(0x40) | ADM_CI_RANGE_END(0xb0) |
    ADM_CI_BURST_8_WORDS, adev.regs + ADM_CI_CONF(0));
    writel(ADM_CI_RANGE_START(0x2a) | ADM_CI_RANGE_END(0x2c) |
    ADM_CI_BURST_8_WORDS, adev.regs + ADM_CI_CONF(1));
    writel(ADM_CI_RANGE_START(0x12) | ADM_CI_RANGE_END(0x28) |
    ADM_CI_BURST_8_WORDS, adev.regs + ADM_CI_CONF(2));
    writel(ADM_GP_CTL_LP_EN | ADM_GP_CTL_LP_CNT(0xf),
    adev.regs + ADM_GP_CTL);
    ret = devm_request_irq(adev.dev, adev.irq, adm_dma_irq,
    0, "adm_dma", adev);
    if (ret)
    goto err_disable_clks;
    platform_set_drvdata(pdev, adev);
    adev.common.dev = adev.dev;
    adev.common.dev.dma_parms = &adev.dma_parms;
// set capabilities
    dma_cap_zero(adev.common.cap_mask);
    dma_cap_set(DMA_SLAVE, adev.common.cap_mask);
    dma_cap_set(DMA_PRIVATE, adev.common.cap_mask);
// initialize dmaengine apis
    adev.common.directions = BIT(DMA_DEV_TO_MEM | DMA_MEM_TO_DEV);
    adev.common.residue_granularity = DMA_RESIDUE_GRANULARITY_DESCRIPTOR;
    adev.common.src_addr_widths = DMA_SLAVE_BUSWIDTH_4_BYTES;
    adev.common.dst_addr_widths = DMA_SLAVE_BUSWIDTH_4_BYTES;
    adev.common.device_free_chan_resources = adm_free_chan;
    adev.common.device_prep_slave_sg = adm_prep_slave_sg;
    adev.common.device_issue_pending = adm_issue_pending;
    adev.common.device_tx_status = adm_tx_status;
    adev.common.device_terminate_all = adm_terminate_all;
    adev.common.device_config = adm_slave_config;
    ret = dma_async_device_register(&adev.common);
    if (ret) {
    dev_err(adev.dev, "failed to register dma async device\n");
    goto err_disable_clks;
    }
    ret = of_dma_controller_register(pdev.dev.of_node, adm_dma_xlate,
    &adev.common);
    if (ret)
    goto err_unregister_dma;
    return 0;
    err_unregister_dma:
    dma_async_device_unregister(&adev.common);
    err_disable_clks:
    clk_disable_unprepare(adev.iface_clk);
    err_disable_core_clk:
    clk_disable_unprepare(adev.core_clk);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn adm_dma_remove(pdev: *mut platform_device) {
    static void adm_dma_remove(struct platform_device *pdev)
    {
    struct adm_device *adev = platform_get_drvdata(pdev);
    struct adm_chan *achan;
    u32 i;
    of_dma_controller_free(pdev.dev.of_node);
    dma_async_device_unregister(&adev.common);
    for (i = 0; i < ADM_MAX_CHANNELS; i++) {
    achan = &adev.channels[i];
// mask IRQs for this channel/EE pair
    writel(0, adev.regs + ADM_CH_RSLT_CONF(achan.id, adev.ee));
    tasklet_kill(&adev.channels[i].vc.task);
    adm_terminate_all(&adev.channels[i].vc.chan);
    }
    devm_free_irq(adev.dev, adev.irq, adev);
    clk_disable_unprepare(adev.core_clk);
    clk_disable_unprepare(adev.iface_clk);
    }
    static const struct of_device_id adm_of_match[] = {
    { .compatible = "qcom,adm", },
    {}
    };
    MODULE_DEVICE_TABLE(of, adm_of_match);
    static struct platform_driver adm_dma_driver = {
    .probe = adm_dma_probe,
    .remove = adm_dma_remove,
    .driver = {
    .name = "adm-dma-engine",
    .of_match_table = adm_of_match,
    },
    };
    module_platform_driver(adm_dma_driver);
    MODULE_AUTHOR("Andy Gross <agross@codeaurora.org>");
    MODULE_DESCRIPTION("QCOM ADM DMA engine driver");
    MODULE_LICENSE("GPL v2");
