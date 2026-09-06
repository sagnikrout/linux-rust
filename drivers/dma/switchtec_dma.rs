//! Automatically rewritten from C to Rust
//! Source: drivers/dma/switchtec_dma.c
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
// Microchip Switchtec(tm) DMA Controller Driver
// Copyright (c) 2025, Kelvin Cao <kelvin.cao@microchip.com>
// Copyright (c) 2025, Microchip Corporation
//

    MODULE_DESCRIPTION("Switchtec PCIe Switch DMA Engine");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Kelvin Cao");
pub const SWITCHTEC_DMAC_CHAN_CTRL_OFFSET: c_uint = 0x1000;
pub const SWITCHTEC_DMAC_CHAN_CFG_STS_OFFSET: c_uint = 0x160000;
pub const SWITCHTEC_DMA_CHAN_HW_REGS_SIZE: c_uint = 0x1000;
pub const SWITCHTEC_DMA_CHAN_FW_REGS_SIZE: c_uint = 0x80;
pub const SWITCHTEC_REG_CAP: c_uint = 0x80;
pub const SWITCHTEC_REG_CHAN_CNT: c_uint = 0x84;
pub const SWITCHTEC_REG_TAG_LIMIT: c_uint = 0x90;
pub const SWITCHTEC_REG_CHAN_STS_VEC: c_uint = 0x94;
pub const SWITCHTEC_REG_SE_BUF_CNT: c_uint = 0x98;
pub const SWITCHTEC_REG_SE_BUF_BASE: c_uint = 0x9a;
pub const SWITCHTEC_DESC_MAX_SIZE: c_uint = 0x100000;

pub const SWITCHTEC_INVALID_HFID: c_uint = 0xffff;

    static const char * const channel_status_str[] = {
    [13] = "received a VDM with length error status",
    [14] = "received a VDM or Cpl with Unsupported Request error status",
    [15] = "received a VDM or Cpl with Completion Abort error status",
    [16] = "received a VDM with ECRC error status",
    [17] = "received a VDM with EP error status",
    [18] = "received a VDM with Reserved Cpl error status",
    [19] = "received only part of split SE CplD",
    [20] = "the ISP_DMAC detected a Completion Time Out",
    [21] = "received a Cpl with Unsupported Request status",
    [22] = "received a Cpl with Completion Abort status",
    [23] = "received a Cpl with a reserved status",
    [24] = "received a TLP with ECRC error status in its metadata",
    [25] = "received a TLP with the EP bit set in the header",
    [26] = "the ISP_DMAC tried to process a SE with an invalid Connection ID",
    [27] = "the ISP_DMAC tried to process a SE with an invalid Remote Host interrupt",
    [28] = "a reserved opcode was detected in an SE",
    [29] = "received a SE Cpl with error status",
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct chan_hw_regs {
    pub cq_head: u16,
    pub rsvd1: u16,
    pub sq_tail: u16,
    pub rsvd2: u16,
    pub ctrl: u8,
    pub rsvd3: [u8; 3],
    pub status: u16,
    pub rsvd4: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct chan_fw_regs {
    pub valid_en_se: u32,
    pub cq_base_lo: u32,
    pub cq_base_hi: u32,
    pub cq_size: u16,
    pub rsvd1: u16,
    pub sq_base_lo: u32,
    pub sq_base_hi: u32,
    pub sq_size: u16,
    pub rsvd2: u16,
    pub int_vec: u32,
    pub perf_cfg: u32,
    pub rsvd3: u32,
    pub perf_latency_selector: u32,
    pub perf_fetched_se_cnt_lo: u32,
    pub perf_fetched_se_cnt_hi: u32,
    pub perf_byte_cnt_lo: u32,
    pub perf_byte_cnt_hi: u32,
    pub rsvd4: u32,
    pub perf_se_pending: u16,
    pub perf_se_buf_empty: u16,
    pub perf_chan_idle: u32,
    pub perf_lat_max: u32,
    pub perf_lat_min: u32,
    pub perf_lat_last: u32,
    pub sq_current: u16,
    pub sq_phase: u16,
    pub cq_current: u16,
    pub cq_phase: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct switchtec_dma_chan {
    pub swdma_dev: *mut switchtec_dma_dev,
    pub dma_chan: dma_chan,
    pub mmio_chan_hw: *mut chan_hw_regs __iomem,
    pub mmio_chan_fw: *mut chan_fw_regs __iomem,
// Serialize hardware control register access
    pub hw_ctrl_lock: spinlock_t,
    pub desc_task: tasklet_struct,
// Serialize descriptor preparation
    pub submit_lock: spinlock_t,
    pub ring_active: bool,
    pub cid: c_int,
// Serialize completion processing
    pub complete_lock: spinlock_t,
    pub comp_ring_active: bool,
// channel index and irq
    pub index: c_int,
    pub irq: c_int,
//
// In driver context, head is advanced by producer while
// tail is advanced by consumer.
//
// the head and tail for both desc_ring and hw_sq
    pub head: c_int,
    pub tail: c_int,
    pub phase_tag: c_int,
    pub hw_sq: *mut switchtec_dma_hw_se_desc,
    pub dma_addr_sq: dma_addr_t,
// the tail for hw_cq
    pub cq_tail: c_int,
    pub hw_cq: *mut switchtec_dma_hw_ce,
    pub dma_addr_cq: dma_addr_t,
    pub list: list_head,
    pub desc_ring: [*mut switchtec_dma_desc; SWITCHTEC_DMA_RING_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct switchtec_dma_dev {
    pub dma_dev: dma_device,
    pub pdev: *mut pci_dev __rcu,
    pub bar: *mut void __iomem,
    pub swdma_chans: *mut switchtec_dma_chan,
    pub chan_cnt: c_int,
    pub chan_status_irq: c_int,
}

    enum chan_op {
    ENABLE_CHAN,
    DISABLE_CHAN,
    };
    enum switchtec_dma_opcode {
    SWITCHTEC_DMA_OPC_MEMCPY = 0,
    SWITCHTEC_DMA_OPC_RDIMM = 0x1,
    SWITCHTEC_DMA_OPC_WRIMM = 0x2,
    SWITCHTEC_DMA_OPC_RHI = 0x6,
    SWITCHTEC_DMA_OPC_NOP = 0x7,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct switchtec_dma_hw_se_desc {
    pub opc: u8,
    pub ctrl: u8,
    pub tlp_setting: __le16,
    pub rsvd1: __le16,
    pub cid: __le16,
    pub byte_cnt: __le32,
    pub /: *mut *mut __le32 addr_lo; / SADDR_LO/WIADDR_LO,
    pub /: *mut *mut __le32 addr_hi; / SADDR_HI/WIADDR_HI,
    pub daddr_lo: __le32,
    pub daddr_hi: __le32,
    pub dfid: __le16,
    pub sfid: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct switchtec_dma_hw_ce {
    pub rdimm_cpl_dw0: __le32,
    pub rdimm_cpl_dw1: __le32,
    pub rsvd1: __le32,
    pub cpl_byte_cnt: __le32,
    pub sq_head: __le16,
    pub rsvd2: __le16,
    pub rsvd3: __le32,
    pub sts_code: __le32,
    pub cid: __le16,
    pub phase_tag: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct switchtec_dma_desc {
    pub txd: dma_async_tx_descriptor,
    pub hw: *mut switchtec_dma_hw_se_desc,
    pub orig_size: u32,
    pub completed: bool,
}

    static int wait_for_chan_status(struct chan_hw_regs __iomem *chan_hw, u32 mask,
    bool set)
    {
    u32 status;
    return readl_poll_timeout_atomic(&chan_hw.status, status,
    (set && (status & mask)) ||
    (!set && !(status & mask)),
    10, 100 * USEC_PER_MSEC);
    }
#[no_mangle]
unsafe extern "C" fn halt_channel(swdma_chan: *mut switchtec_dma_chan) -> c_int {
    static int halt_channel(struct switchtec_dma_chan *swdma_chan)
    {
    struct chan_hw_regs __iomem *chan_hw = swdma_chan.mmio_chan_hw;
    struct pci_dev *pdev;
    int ret;
    rcu_read_lock();
    pdev = rcu_dereference(swdma_chan.swdma_dev.pdev);
    if (!pdev) {
    ret = -ENODEV;
    goto unlock_and_exit;
    }
    spin_lock(&swdma_chan.hw_ctrl_lock);
    writeb(SWITCHTEC_CHAN_CTRL_HALT, &chan_hw.ctrl);
    ret = wait_for_chan_status(chan_hw, SWITCHTEC_CHAN_STS_HALTED, true);
    spin_unlock(&swdma_chan.hw_ctrl_lock);
    unlock_and_exit:
    rcu_read_unlock();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn unhalt_channel(swdma_chan: *mut switchtec_dma_chan) -> c_int {
    static int unhalt_channel(struct switchtec_dma_chan *swdma_chan)
    {
    struct chan_hw_regs __iomem *chan_hw = swdma_chan.mmio_chan_hw;
    struct pci_dev *pdev;
    u8 ctrl;
    int ret;
    rcu_read_lock();
    pdev = rcu_dereference(swdma_chan.swdma_dev.pdev);
    if (!pdev) {
    ret = -ENODEV;
    goto unlock_and_exit;
    }
    spin_lock(&swdma_chan.hw_ctrl_lock);
    ctrl = readb(&chan_hw.ctrl);
    ctrl &= ~SWITCHTEC_CHAN_CTRL_HALT;
    writeb(ctrl, &chan_hw.ctrl);
    ret = wait_for_chan_status(chan_hw, SWITCHTEC_CHAN_STS_HALTED, false);
    spin_unlock(&swdma_chan.hw_ctrl_lock);
    unlock_and_exit:
    rcu_read_unlock();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn flush_pci_write(chan_hw: *mut chan_hw_regs __iomem) {
    static void flush_pci_write(struct chan_hw_regs __iomem *chan_hw)
    {
    readl(&chan_hw.cq_head);
    }
#[no_mangle]
unsafe extern "C" fn reset_channel(swdma_chan: *mut switchtec_dma_chan) -> c_int {
    static int reset_channel(struct switchtec_dma_chan *swdma_chan)
    {
    struct chan_hw_regs __iomem *chan_hw = swdma_chan.mmio_chan_hw;
    struct pci_dev *pdev;
    rcu_read_lock();
    pdev = rcu_dereference(swdma_chan.swdma_dev.pdev);
    if (!pdev) {
    rcu_read_unlock();
    return -ENODEV;
    }
    spin_lock(&swdma_chan.hw_ctrl_lock);
    writel(SWITCHTEC_CHAN_CTRL_RESET | SWITCHTEC_CHAN_CTRL_ERR_PAUSE,
    &chan_hw.ctrl);
    flush_pci_write(chan_hw);
    udelay(1000);
    writel(SWITCHTEC_CHAN_CTRL_ERR_PAUSE, &chan_hw.ctrl);
    spin_unlock(&swdma_chan.hw_ctrl_lock);
    flush_pci_write(chan_hw);
    rcu_read_unlock();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pause_reset_channel(swdma_chan: *mut switchtec_dma_chan) -> c_int {
    static int pause_reset_channel(struct switchtec_dma_chan *swdma_chan)
    {
    struct chan_hw_regs __iomem *chan_hw = swdma_chan.mmio_chan_hw;
    struct pci_dev *pdev;
    rcu_read_lock();
    pdev = rcu_dereference(swdma_chan.swdma_dev.pdev);
    if (!pdev) {
    rcu_read_unlock();
    return -ENODEV;
    }
    spin_lock(&swdma_chan.hw_ctrl_lock);
    writeb(SWITCHTEC_CHAN_CTRL_PAUSE, &chan_hw.ctrl);
    spin_unlock(&swdma_chan.hw_ctrl_lock);
    flush_pci_write(chan_hw);
    rcu_read_unlock();
// wait 60ms to ensure no pending CEs
    mdelay(60);
    return reset_channel(swdma_chan);
    }
#[no_mangle]
unsafe extern "C" fn channel_op(swdma_chan: *mut switchtec_dma_chan, op: c_int) -> c_int {
    static int channel_op(struct switchtec_dma_chan *swdma_chan, int op)
    {
    struct chan_fw_regs __iomem *chan_fw = swdma_chan.mmio_chan_fw;
    struct pci_dev *pdev;
    u32 valid_en_se;
    rcu_read_lock();
    pdev = rcu_dereference(swdma_chan.swdma_dev.pdev);
    if (!pdev) {
    rcu_read_unlock();
    return -ENODEV;
    }
    valid_en_se = readl(&chan_fw.valid_en_se);
    if (op == ENABLE_CHAN)
    valid_en_se |= SWITCHTEC_CHAN_ENABLE;
    else
    valid_en_se &= ~SWITCHTEC_CHAN_ENABLE;
    writel(valid_en_se, &chan_fw.valid_en_se);
    rcu_read_unlock();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn enable_channel(swdma_chan: *mut switchtec_dma_chan) -> c_int {
    static int enable_channel(struct switchtec_dma_chan *swdma_chan)
    {
    return channel_op(swdma_chan, ENABLE_CHAN);
    }
#[no_mangle]
unsafe extern "C" fn disable_channel(swdma_chan: *mut switchtec_dma_chan) -> c_int {
    static int disable_channel(struct switchtec_dma_chan *swdma_chan)
    {
    return channel_op(swdma_chan, DISABLE_CHAN);
    }
    static void
    switchtec_dma_cleanup_completed(struct switchtec_dma_chan *swdma_chan)
    {
    struct device *chan_dev = &swdma_chan.dma_chan.dev.device;
    struct switchtec_dma_desc *desc;
    struct switchtec_dma_hw_ce *ce;
    struct dmaengine_result res;
    int tail, cid, se_idx, i;
    __le16 phase_tag;
    u32 sts_code;
    __le32 *p;
    do {
    spin_lock_bh(&swdma_chan.complete_lock);
    if (!swdma_chan.comp_ring_active) {
    spin_unlock_bh(&swdma_chan.complete_lock);
    break;
    }
    ce = &swdma_chan.hw_cq[swdma_chan.cq_tail];
//
// phase_tag is updated by hardware, ensure the value is
// not from the cache
//
    phase_tag = smp_load_acquire(&ce.phase_tag);
    if (le16_to_cpu(phase_tag) == swdma_chan.phase_tag) {
    spin_unlock_bh(&swdma_chan.complete_lock);
    break;
    }
    cid = le16_to_cpu(ce.cid);
    se_idx = cid & (SWITCHTEC_DMA_SQ_SIZE - 1);
    desc = swdma_chan.desc_ring[se_idx];
    tail = swdma_chan.tail;
    res.residue = desc.orig_size - le32_to_cpu(ce.cpl_byte_cnt);
    sts_code = le32_to_cpu(ce.sts_code);
    if (!(sts_code & SWITCHTEC_CE_SC_MASK)) {
    res.result = DMA_TRANS_NOERROR;
    } else {
    if (sts_code & SWITCHTEC_CE_SC_D_RD_CTO)
    res.result = DMA_TRANS_READ_FAILED;
    else
    res.result = DMA_TRANS_WRITE_FAILED;
    dev_err(chan_dev, "CID 0x%04x failed, SC 0x%08x\n", cid,
    (u32)(sts_code & SWITCHTEC_CE_SC_MASK));
    p = (__le32 *)ce;
    for (i = 0; i < sizeof(*ce) / 4; i++) {
    dev_err(chan_dev, "CE DW%d: 0x%08x\n", i,
    le32_to_cpu(*p));
    p++;
    }
    }
    desc.completed = true;
    swdma_chan.cq_tail++;
    swdma_chan.cq_tail &= SWITCHTEC_DMA_CQ_SIZE - 1;
    rcu_read_lock();
    if (!rcu_dereference(swdma_chan.swdma_dev.pdev)) {
    rcu_read_unlock();
    spin_unlock_bh(&swdma_chan.complete_lock);
    return;
    }
    writew(swdma_chan.cq_tail, &swdma_chan.mmio_chan_hw.cq_head);
    rcu_read_unlock();
    if (swdma_chan.cq_tail == 0)
    swdma_chan.phase_tag = !swdma_chan.phase_tag;
// Out of order CE
    if (se_idx != tail) {
    spin_unlock_bh(&swdma_chan.complete_lock);
    continue;
    }
    do {
    dma_cookie_complete(&desc.txd);
    dma_descriptor_unmap(&desc.txd);
    dmaengine_desc_get_callback_invoke(&desc.txd, &res);
    desc.txd.callback = core::ptr::null_mut();
    desc.txd.callback_result = core::ptr::null_mut();
    desc.completed = false;
    tail++;
    tail &= SWITCHTEC_DMA_SQ_SIZE - 1;
//
// Ensure the desc updates are visible before updating
// the tail index
//
    smp_store_release(&swdma_chan.tail, tail);
    desc = swdma_chan.desc_ring[swdma_chan.tail];
    if (!desc.completed)
    break;
    } while (CIRC_CNT(READ_ONCE(swdma_chan.head), swdma_chan.tail,
    SWITCHTEC_DMA_SQ_SIZE));
    spin_unlock_bh(&swdma_chan.complete_lock);
    } while (1);
    }
    static void
    switchtec_dma_abort_desc(struct switchtec_dma_chan *swdma_chan, int force)
    {
    struct switchtec_dma_desc *desc;
    struct dmaengine_result res;
    if (!force)
    switchtec_dma_cleanup_completed(swdma_chan);
    spin_lock_bh(&swdma_chan.complete_lock);
    while (CIRC_CNT(swdma_chan.head, swdma_chan.tail,
    SWITCHTEC_DMA_SQ_SIZE) >= 1) {
    desc = swdma_chan.desc_ring[swdma_chan.tail];
    res.residue = desc.orig_size;
    res.result = DMA_TRANS_ABORTED;
    dma_cookie_complete(&desc.txd);
    dma_descriptor_unmap(&desc.txd);
    if (!force)
    dmaengine_desc_get_callback_invoke(&desc.txd, &res);
    desc.txd.callback = core::ptr::null_mut();
    desc.txd.callback_result = core::ptr::null_mut();
    swdma_chan.tail++;
    swdma_chan.tail &= SWITCHTEC_DMA_SQ_SIZE - 1;
    }
    spin_unlock_bh(&swdma_chan.complete_lock);
    }
#[no_mangle]
unsafe extern "C" fn switchtec_dma_chan_stop(swdma_chan: *mut switchtec_dma_chan) {
    static void switchtec_dma_chan_stop(struct switchtec_dma_chan *swdma_chan)
    {
    int rc;
    rc = halt_channel(swdma_chan);
    if (rc)
    return;
    rcu_read_lock();
    if (!rcu_dereference(swdma_chan.swdma_dev.pdev)) {
    rcu_read_unlock();
    return;
    }
    writel(0, &swdma_chan.mmio_chan_fw.sq_base_lo);
    writel(0, &swdma_chan.mmio_chan_fw.sq_base_hi);
    writel(0, &swdma_chan.mmio_chan_fw.cq_base_lo);
    writel(0, &swdma_chan.mmio_chan_fw.cq_base_hi);
    rcu_read_unlock();
    }
#[no_mangle]
unsafe extern "C" fn switchtec_dma_terminate_all(chan: *mut dma_chan) -> c_int {
    static int switchtec_dma_terminate_all(struct dma_chan *chan)
    {
    struct switchtec_dma_chan *swdma_chan =
    container_of(chan, struct switchtec_dma_chan, dma_chan);
    spin_lock_bh(&swdma_chan.complete_lock);
    swdma_chan.comp_ring_active = false;
    spin_unlock_bh(&swdma_chan.complete_lock);
    return pause_reset_channel(swdma_chan);
    }
#[no_mangle]
unsafe extern "C" fn switchtec_dma_synchronize(chan: *mut dma_chan) {
    static void switchtec_dma_synchronize(struct dma_chan *chan)
    {
    struct switchtec_dma_chan *swdma_chan =
    container_of(chan, struct switchtec_dma_chan, dma_chan);
    int rc;
    switchtec_dma_abort_desc(swdma_chan, 1);
    rc = enable_channel(swdma_chan);
    if (rc)
    return;
    rc = reset_channel(swdma_chan);
    if (rc)
    return;
    rc = unhalt_channel(swdma_chan);
    if (rc)
    return;
    spin_lock_bh(&swdma_chan.submit_lock);
    swdma_chan.head = 0;
    spin_unlock_bh(&swdma_chan.submit_lock);
    spin_lock_bh(&swdma_chan.complete_lock);
    swdma_chan.comp_ring_active = true;
    swdma_chan.phase_tag = 0;
    swdma_chan.tail = 0;
    swdma_chan.cq_tail = 0;
    swdma_chan.cid = 0;
    dma_cookie_init(chan);
    spin_unlock_bh(&swdma_chan.complete_lock);
    }
    static struct dma_async_tx_descriptor *
    switchtec_dma_prep_desc(struct dma_chan *c, u16 dst_fid, dma_addr_t dma_dst,
    u16 src_fid, dma_addr_t dma_src, u64 data,
    size_t len, unsigned long flags)
    __acquires(swdma_chan.submit_lock)
    {
    struct switchtec_dma_chan *swdma_chan =
    container_of(c, struct switchtec_dma_chan, dma_chan);
    struct switchtec_dma_desc *desc;
    int head, tail;
    spin_lock_bh(&swdma_chan.submit_lock);
    if (!swdma_chan.ring_active)
    goto err_unlock;
    tail = READ_ONCE(swdma_chan.tail);
    head = swdma_chan.head;
    if (!CIRC_SPACE(head, tail, SWITCHTEC_DMA_RING_SIZE))
    goto err_unlock;
    desc = swdma_chan.desc_ring[head];
    if (src_fid != SWITCHTEC_INVALID_HFID &&
    dst_fid != SWITCHTEC_INVALID_HFID)
    desc.hw.ctrl |= SWITCHTEC_SE_DFM;
    if (flags & DMA_PREP_INTERRUPT)
    desc.hw.ctrl |= SWITCHTEC_SE_LIOF;
    if (flags & DMA_PREP_FENCE)
    desc.hw.ctrl |= SWITCHTEC_SE_BRR;
    desc.txd.flags = flags;
    desc.completed = false;
    desc.hw.opc = SWITCHTEC_DMA_OPC_MEMCPY;
    desc.hw.addr_lo = cpu_to_le32(lower_32_bits(dma_src));
    desc.hw.addr_hi = cpu_to_le32(upper_32_bits(dma_src));
    desc.hw.daddr_lo = cpu_to_le32(lower_32_bits(dma_dst));
    desc.hw.daddr_hi = cpu_to_le32(upper_32_bits(dma_dst));
    desc.hw.byte_cnt = cpu_to_le32(len);
    desc.hw.tlp_setting = 0;
    desc.hw.dfid = cpu_to_le16(dst_fid);
    desc.hw.sfid = cpu_to_le16(src_fid);
    swdma_chan.cid &= SWITCHTEC_SE_CID_MASK;
    desc.hw.cid = cpu_to_le16(swdma_chan.cid++);
    desc.orig_size = len;
// return with the lock held, it will be released in tx_submit
    return &desc.txd;
    err_unlock:
//
// Keep sparse happy by restoring an even lock count on
// this lock.
//
    __acquire(swdma_chan.submit_lock);
    spin_unlock_bh(&swdma_chan.submit_lock);
    return core::ptr::null_mut();
    }
    static struct dma_async_tx_descriptor *
    switchtec_dma_prep_memcpy(struct dma_chan *c, dma_addr_t dma_dst,
    dma_addr_t dma_src, size_t len, unsigned long flags)
    __acquires(swdma_chan.submit_lock)
    {
    if (len > SWITCHTEC_DESC_MAX_SIZE) {
//
// Keep sparse happy by restoring an even lock count on
// this lock.
//
    __acquire(swdma_chan.submit_lock);
    return core::ptr::null_mut();
    }
    return switchtec_dma_prep_desc(c, SWITCHTEC_INVALID_HFID, dma_dst,
    SWITCHTEC_INVALID_HFID, dma_src, 0, len,
    flags);
    }
    static dma_cookie_t
    switchtec_dma_tx_submit(struct dma_async_tx_descriptor *desc)
    __releases(swdma_chan.submit_lock)
    {
    struct switchtec_dma_chan *swdma_chan =
    container_of(desc.chan, struct switchtec_dma_chan, dma_chan);
    dma_cookie_t cookie;
    int head;
    head = swdma_chan.head + 1;
    head &= SWITCHTEC_DMA_RING_SIZE - 1;
//
// Ensure the desc updates are visible before updating the head index
//
    smp_store_release(&swdma_chan.head, head);
    cookie = dma_cookie_assign(desc);
    spin_unlock_bh(&swdma_chan.submit_lock);
    return cookie;
    }
    static enum dma_status switchtec_dma_tx_status(struct dma_chan *chan,
    dma_cookie_t cookie, struct dma_tx_state *txstate)
    {
    struct switchtec_dma_chan *swdma_chan =
    container_of(chan, struct switchtec_dma_chan, dma_chan);
    enum dma_status ret;
    ret = dma_cookie_status(chan, cookie, txstate);
    if (ret == DMA_COMPLETE)
    return ret;
//
// For jobs where the interrupts are disabled, this is the only place
// to process the completions returned by the hardware. Callers that
// disable interrupts must call tx_status() to determine when a job
// is done, so it is safe to process completions here. If a job has
// interrupts enabled, then the completions will normally be processed
// in the tasklet that is triggered by the interrupt and tx_status()
// does not need to be called.
//
    switchtec_dma_cleanup_completed(swdma_chan);
    return dma_cookie_status(chan, cookie, txstate);
    }
#[no_mangle]
unsafe extern "C" fn switchtec_dma_issue_pending(chan: *mut dma_chan) {
    static void switchtec_dma_issue_pending(struct dma_chan *chan)
    {
    struct switchtec_dma_chan *swdma_chan =
    container_of(chan, struct switchtec_dma_chan, dma_chan);
    struct switchtec_dma_dev *swdma_dev = swdma_chan.swdma_dev;
//
// The sq_tail register is actually for the head of the
// submisssion queue. Chip has the opposite define of head/tail
// to the Linux kernel.
//
    rcu_read_lock();
    if (!rcu_dereference(swdma_dev.pdev)) {
    rcu_read_unlock();
    return;
    }
    spin_lock_bh(&swdma_chan.submit_lock);
    writew(swdma_chan.head, &swdma_chan.mmio_chan_hw.sq_tail);
    spin_unlock_bh(&swdma_chan.submit_lock);
    rcu_read_unlock();
    }
#[no_mangle]
unsafe extern "C" fn switchtec_dma_pause(chan: *mut dma_chan) -> c_int {
    static int switchtec_dma_pause(struct dma_chan *chan)
    {
    struct switchtec_dma_chan *swdma_chan =
    container_of(chan, struct switchtec_dma_chan, dma_chan);
    struct chan_hw_regs __iomem *chan_hw = swdma_chan.mmio_chan_hw;
    struct pci_dev *pdev;
    int ret;
    rcu_read_lock();
    pdev = rcu_dereference(swdma_chan.swdma_dev.pdev);
    if (!pdev) {
    ret = -ENODEV;
    goto unlock_and_exit;
    }
    spin_lock(&swdma_chan.hw_ctrl_lock);
    writeb(SWITCHTEC_CHAN_CTRL_PAUSE, &chan_hw.ctrl);
    ret = wait_for_chan_status(chan_hw, SWITCHTEC_CHAN_STS_PAUSED, true);
    spin_unlock(&swdma_chan.hw_ctrl_lock);
    unlock_and_exit:
    rcu_read_unlock();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn switchtec_dma_resume(chan: *mut dma_chan) -> c_int {
    static int switchtec_dma_resume(struct dma_chan *chan)
    {
    struct switchtec_dma_chan *swdma_chan =
    container_of(chan, struct switchtec_dma_chan, dma_chan);
    struct chan_hw_regs __iomem *chan_hw = swdma_chan.mmio_chan_hw;
    struct pci_dev *pdev;
    int ret;
    rcu_read_lock();
    pdev = rcu_dereference(swdma_chan.swdma_dev.pdev);
    if (!pdev) {
    ret = -ENODEV;
    goto unlock_and_exit;
    }
    spin_lock(&swdma_chan.hw_ctrl_lock);
    writeb(0, &chan_hw.ctrl);
    ret = wait_for_chan_status(chan_hw, SWITCHTEC_CHAN_STS_PAUSED, false);
    spin_unlock(&swdma_chan.hw_ctrl_lock);
    unlock_and_exit:
    rcu_read_unlock();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn switchtec_dma_desc_task(data: c_ulong) {
    static void switchtec_dma_desc_task(unsigned long data)
    {
    struct switchtec_dma_chan *swdma_chan = (void *)data;
    switchtec_dma_cleanup_completed(swdma_chan);
    }
#[no_mangle]
unsafe extern "C" fn switchtec_dma_isr(irq: c_int, chan: *mut c_void) -> irqreturn_t {
    static irqreturn_t switchtec_dma_isr(int irq, void *chan)
    {
    struct switchtec_dma_chan *swdma_chan = chan;
    if (swdma_chan.comp_ring_active)
    tasklet_schedule(&swdma_chan.desc_task);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn switchtec_dma_chan_status_isr(irq: c_int, dma: *mut c_void) -> irqreturn_t {
    static irqreturn_t switchtec_dma_chan_status_isr(int irq, void *dma)
    {
    struct switchtec_dma_dev *swdma_dev = dma;
    struct dma_device *dma_dev = &swdma_dev.dma_dev;
    struct switchtec_dma_chan *swdma_chan;
    struct chan_hw_regs __iomem *chan_hw;
    struct device *chan_dev;
    struct dma_chan *chan;
    u32 chan_status;
    int bit;
    list_for_each_entry(chan, &dma_dev.channels, device_node) {
    swdma_chan = container_of(chan, struct switchtec_dma_chan,
    dma_chan);
    chan_dev = &swdma_chan.dma_chan.dev.device;
    chan_hw = swdma_chan.mmio_chan_hw;
    rcu_read_lock();
    if (!rcu_dereference(swdma_dev.pdev)) {
    rcu_read_unlock();
    goto out;
    }
    chan_status = readl(&chan_hw.status);
    chan_status &= SWITCHTEC_CHAN_STS_PAUSED_MASK;
    rcu_read_unlock();
    bit = ffs(chan_status);
    if (!bit)
    dev_dbg(chan_dev, "No pause bit set.\n");
    else
    dev_err(chan_dev, "Paused, %s\n",
    channel_status_str[bit - 1]);
    }
    out:
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn switchtec_dma_free_desc(swdma_chan: *mut switchtec_dma_chan) {
    static void switchtec_dma_free_desc(struct switchtec_dma_chan *swdma_chan)
    {
    struct switchtec_dma_dev *swdma_dev = swdma_chan.swdma_dev;
    size_t size;
    int i;
    size = SWITCHTEC_DMA_SQ_SIZE * sizeof(*swdma_chan.hw_sq);
    if (swdma_chan.hw_sq)
    dma_free_coherent(swdma_dev.dma_dev.dev, size,
    swdma_chan.hw_sq, swdma_chan.dma_addr_sq);
    size = SWITCHTEC_DMA_CQ_SIZE * sizeof(*swdma_chan.hw_cq);
    if (swdma_chan.hw_cq)
    dma_free_coherent(swdma_dev.dma_dev.dev, size,
    swdma_chan.hw_cq, swdma_chan.dma_addr_cq);
    for (i = 0; i < SWITCHTEC_DMA_RING_SIZE; i++)
    kfree(swdma_chan.desc_ring[i]);
    }
#[no_mangle]
unsafe extern "C" fn switchtec_dma_alloc_desc(swdma_chan: *mut switchtec_dma_chan) -> c_int {
    static int switchtec_dma_alloc_desc(struct switchtec_dma_chan *swdma_chan)
    {
    struct switchtec_dma_dev *swdma_dev = swdma_chan.swdma_dev;
    struct chan_fw_regs __iomem *chan_fw = swdma_chan.mmio_chan_fw;
    struct switchtec_dma_desc *desc;
    struct pci_dev *pdev;
    size_t size;
    int rc, i;
    swdma_chan.head = 0;
    swdma_chan.tail = 0;
    swdma_chan.cq_tail = 0;
    size = SWITCHTEC_DMA_SQ_SIZE * sizeof(*swdma_chan.hw_sq);
    swdma_chan.hw_sq = dma_alloc_coherent(swdma_dev.dma_dev.dev, size,
    &swdma_chan.dma_addr_sq,
    GFP_NOWAIT);
    if (!swdma_chan.hw_sq) {
    rc = -ENOMEM;
    goto free_and_exit;
    }
    size = SWITCHTEC_DMA_CQ_SIZE * sizeof(*swdma_chan.hw_cq);
    swdma_chan.hw_cq = dma_alloc_coherent(swdma_dev.dma_dev.dev, size,
    &swdma_chan.dma_addr_cq,
    GFP_NOWAIT);
    if (!swdma_chan.hw_cq) {
    rc = -ENOMEM;
    goto free_and_exit;
    }
// reset host phase tag
    swdma_chan.phase_tag = 0;
    for (i = 0; i < SWITCHTEC_DMA_RING_SIZE; i++) {
    desc = kzalloc_obj(*desc, GFP_NOWAIT);
    if (!desc) {
    rc = -ENOMEM;
    goto free_and_exit;
    }
    dma_async_tx_descriptor_init(&desc.txd, &swdma_chan.dma_chan);
    desc.txd.tx_submit = switchtec_dma_tx_submit;
    desc.hw = &swdma_chan.hw_sq[i];
    desc.completed = true;
    swdma_chan.desc_ring[i] = desc;
    }
    rcu_read_lock();
    pdev = rcu_dereference(swdma_dev.pdev);
    if (!pdev) {
    rcu_read_unlock();
    rc = -ENODEV;
    goto free_and_exit;
    }
// set sq/cq
    writel(lower_32_bits(swdma_chan.dma_addr_sq), &chan_fw.sq_base_lo);
    writel(upper_32_bits(swdma_chan.dma_addr_sq), &chan_fw.sq_base_hi);
    writel(lower_32_bits(swdma_chan.dma_addr_cq), &chan_fw.cq_base_lo);
    writel(upper_32_bits(swdma_chan.dma_addr_cq), &chan_fw.cq_base_hi);
    writew(SWITCHTEC_DMA_SQ_SIZE, &swdma_chan.mmio_chan_fw.sq_size);
    writew(SWITCHTEC_DMA_CQ_SIZE, &swdma_chan.mmio_chan_fw.cq_size);
    rcu_read_unlock();
    return 0;
    free_and_exit:
    switchtec_dma_free_desc(swdma_chan);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn switchtec_dma_alloc_chan_resources(chan: *mut dma_chan) -> c_int {
    static int switchtec_dma_alloc_chan_resources(struct dma_chan *chan)
    {
    struct switchtec_dma_chan *swdma_chan =
    container_of(chan, struct switchtec_dma_chan, dma_chan);
    struct switchtec_dma_dev *swdma_dev = swdma_chan.swdma_dev;
    u32 perf_cfg;
    int rc;
    rc = switchtec_dma_alloc_desc(swdma_chan);
    if (rc)
    return rc;
    rc = enable_channel(swdma_chan);
    if (rc)
    return rc;
    rc = reset_channel(swdma_chan);
    if (rc)
    return rc;
    rc = unhalt_channel(swdma_chan);
    if (rc)
    return rc;
    swdma_chan.ring_active = true;
    swdma_chan.comp_ring_active = true;
    swdma_chan.cid = 0;
    dma_cookie_init(chan);
    rcu_read_lock();
    if (!rcu_dereference(swdma_dev.pdev)) {
    rcu_read_unlock();
    return -ENODEV;
    }
    perf_cfg = readl(&swdma_chan.mmio_chan_fw.perf_cfg);
    rcu_read_unlock();
    dev_dbg(&chan.dev.device, "Burst Size:  0x%x\n",
    FIELD_GET(PERF_BURST_SIZE_MASK, perf_cfg));
    dev_dbg(&chan.dev.device, "Burst Scale: 0x%x\n",
    FIELD_GET(PERF_BURST_SCALE_MASK, perf_cfg));
    dev_dbg(&chan.dev.device, "Interval:    0x%x\n",
    FIELD_GET(PERF_INTERVAL_MASK, perf_cfg));
    dev_dbg(&chan.dev.device, "Arb Weight:  0x%x\n",
    FIELD_GET(PERF_ARB_WEIGHT_MASK, perf_cfg));
    dev_dbg(&chan.dev.device, "MRRS:        0x%x\n",
    FIELD_GET(PERF_MRRS_MASK, perf_cfg));
    return SWITCHTEC_DMA_SQ_SIZE;
    }
#[no_mangle]
unsafe extern "C" fn switchtec_dma_free_chan_resources(chan: *mut dma_chan) {
    static void switchtec_dma_free_chan_resources(struct dma_chan *chan)
    {
    struct switchtec_dma_chan *swdma_chan =
    container_of(chan, struct switchtec_dma_chan, dma_chan);
    spin_lock_bh(&swdma_chan.submit_lock);
    swdma_chan.ring_active = false;
    spin_unlock_bh(&swdma_chan.submit_lock);
    spin_lock_bh(&swdma_chan.complete_lock);
    swdma_chan.comp_ring_active = false;
    spin_unlock_bh(&swdma_chan.complete_lock);
    switchtec_dma_chan_stop(swdma_chan);
    switchtec_dma_abort_desc(swdma_chan, 0);
    switchtec_dma_free_desc(swdma_chan);
    disable_channel(swdma_chan);
    }
    static int switchtec_dma_chan_init(struct switchtec_dma_dev *swdma_dev,
    struct pci_dev *pdev, int i)
    {
    struct dma_device *dma = &swdma_dev.dma_dev;
    struct switchtec_dma_chan *swdma_chan;
    u32 valid_en_se, thresh;
    int se_buf_len, irq, rc;
    struct dma_chan *chan;
    swdma_chan = kzalloc_obj(*swdma_chan);
    if (!swdma_chan)
    return -ENOMEM;
    swdma_chan.phase_tag = 0;
    swdma_chan.index = i;
    swdma_chan.swdma_dev = swdma_dev;
    spin_lock_init(&swdma_chan.hw_ctrl_lock);
    spin_lock_init(&swdma_chan.submit_lock);
    spin_lock_init(&swdma_chan.complete_lock);
    tasklet_init(&swdma_chan.desc_task, switchtec_dma_desc_task,
    (unsigned long)swdma_chan);
    swdma_chan.mmio_chan_fw =
    swdma_dev.bar + SWITCHTEC_DMAC_CHAN_CFG_STS_OFFSET +
    i * SWITCHTEC_DMA_CHAN_FW_REGS_SIZE;
    swdma_chan.mmio_chan_hw =
    swdma_dev.bar + SWITCHTEC_DMAC_CHAN_CTRL_OFFSET +
    i * SWITCHTEC_DMA_CHAN_HW_REGS_SIZE;
    swdma_dev.swdma_chans[i] = swdma_chan;
    rc = pause_reset_channel(swdma_chan);
    if (rc)
    goto free_and_exit;
// init perf tuner
    writel(FIELD_PREP(PERF_BURST_SCALE_MASK, 1) |
    FIELD_PREP(PERF_MRRS_MASK, 3) |
    FIELD_PREP(PERF_BURST_SIZE_MASK, 6) |
    FIELD_PREP(PERF_ARB_WEIGHT_MASK, 1),
    &swdma_chan.mmio_chan_fw.perf_cfg);
    valid_en_se = readl(&swdma_chan.mmio_chan_fw.valid_en_se);
    dev_dbg(&pdev.dev, "Channel %d: SE buffer base %d\n", i,
    FIELD_GET(SE_BUF_BASE_MASK, valid_en_se));
    se_buf_len = FIELD_GET(SE_BUF_LEN_MASK, valid_en_se);
    dev_dbg(&pdev.dev, "Channel %d: SE buffer count %d\n", i, se_buf_len);
    thresh = se_buf_len / 2;
    valid_en_se |= FIELD_PREP(SE_THRESH_MASK, thresh);
    writel(valid_en_se, &swdma_chan.mmio_chan_fw.valid_en_se);
// request irqs
    irq = readl(&swdma_chan.mmio_chan_fw.int_vec);
    dev_dbg(&pdev.dev, "Channel %d: CE irq vector %d\n", i, irq);
    rc = pci_request_irq(pdev, irq, switchtec_dma_isr, core::ptr::null_mut(), swdma_chan,
    KBUILD_MODNAME);
    if (rc)
    goto free_and_exit;
    swdma_chan.irq = irq;
    chan = &swdma_chan.dma_chan;
    chan.device = dma;
    dma_cookie_init(chan);
    list_add_tail(&chan.device_node, &dma.channels);
    return 0;
    free_and_exit:
    kfree(swdma_chan);
    return rc;
    }
    static int switchtec_dma_chan_free(struct pci_dev *pdev,
    struct switchtec_dma_chan *swdma_chan)
    {
    spin_lock_bh(&swdma_chan.submit_lock);
    swdma_chan.ring_active = false;
    spin_unlock_bh(&swdma_chan.submit_lock);
    spin_lock_bh(&swdma_chan.complete_lock);
    swdma_chan.comp_ring_active = false;
    spin_unlock_bh(&swdma_chan.complete_lock);
    pci_free_irq(pdev, swdma_chan.irq, swdma_chan);
    tasklet_kill(&swdma_chan.desc_task);
    switchtec_dma_chan_stop(swdma_chan);
    return 0;
    }
    static int switchtec_dma_chans_release(struct pci_dev *pdev,
    struct switchtec_dma_dev *swdma_dev)
    {
    int i;
    for (i = 0; i < swdma_dev.chan_cnt; i++)
    switchtec_dma_chan_free(pdev, swdma_dev.swdma_chans[i]);
    return 0;
    }
    static int switchtec_dma_chans_enumerate(struct switchtec_dma_dev *swdma_dev,
    struct pci_dev *pdev, int chan_cnt)
    {
    struct dma_device *dma = &swdma_dev.dma_dev;
    int base, cnt, rc, i;
    swdma_dev.swdma_chans = kzalloc_objs(*swdma_dev.swdma_chans, chan_cnt);
    if (!swdma_dev.swdma_chans)
    return -ENOMEM;
    base = readw(swdma_dev.bar + SWITCHTEC_REG_SE_BUF_BASE);
    cnt = readw(swdma_dev.bar + SWITCHTEC_REG_SE_BUF_CNT);
    dev_dbg(&pdev.dev, "EP SE buffer base %d\n", base);
    dev_dbg(&pdev.dev, "EP SE buffer count %d\n", cnt);
    INIT_LIST_HEAD(&dma.channels);
    for (i = 0; i < chan_cnt; i++) {
    rc = switchtec_dma_chan_init(swdma_dev, pdev, i);
    if (rc) {
    dev_err(&pdev.dev, "Channel %d: init channel failed\n",
    i);
    chan_cnt = i;
    goto err_exit;
    }
    }
    return chan_cnt;
    err_exit:
    for (i = 0; i < chan_cnt; i++)
    switchtec_dma_chan_free(pdev, swdma_dev.swdma_chans[i]);
    kfree(swdma_dev.swdma_chans);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn switchtec_dma_release(dma_dev: *mut dma_device) {
    static void switchtec_dma_release(struct dma_device *dma_dev)
    {
    struct switchtec_dma_dev *swdma_dev =
    container_of(dma_dev, struct switchtec_dma_dev, dma_dev);
    int i;
    for (i = 0; i < swdma_dev.chan_cnt; i++)
    kfree(swdma_dev.swdma_chans[i]);
    kfree(swdma_dev.swdma_chans);
    put_device(dma_dev.dev);
    kfree(swdma_dev);
    }
#[no_mangle]
unsafe extern "C" fn switchtec_dma_create(pdev: *mut pci_dev) -> c_int {
    static int switchtec_dma_create(struct pci_dev *pdev)
    {
    struct switchtec_dma_dev *swdma_dev;
    int chan_cnt, nr_vecs, irq, rc;
    struct dma_device *dma;
    struct dma_chan *chan;
//
// Create the switchtec dma device
//
    swdma_dev = kzalloc_obj(*swdma_dev);
    if (!swdma_dev)
    return -ENOMEM;
    swdma_dev.bar = ioremap(pci_resource_start(pdev, 0),
    pci_resource_len(pdev, 0));
    RCU_INIT_POINTER(swdma_dev.pdev, pdev);
    nr_vecs = pci_msix_vec_count(pdev);
    rc = pci_alloc_irq_vectors(pdev, nr_vecs, nr_vecs, PCI_IRQ_MSIX);
    if (rc < 0)
    goto err_exit;
    irq = readw(swdma_dev.bar + SWITCHTEC_REG_CHAN_STS_VEC);
    pci_dbg(pdev, "Channel pause irq vector %d\n", irq);
    rc = pci_request_irq(pdev, irq, core::ptr::null_mut(), switchtec_dma_chan_status_isr,
    swdma_dev, KBUILD_MODNAME);
    if (rc)
    goto err_exit;
    swdma_dev.chan_status_irq = irq;
    chan_cnt = readl(swdma_dev.bar + SWITCHTEC_REG_CHAN_CNT);
    if (!chan_cnt) {
    pci_err(pdev, "No channel configured.\n");
    rc = -ENXIO;
    goto err_exit;
    }
    chan_cnt = switchtec_dma_chans_enumerate(swdma_dev, pdev, chan_cnt);
    if (chan_cnt < 0) {
    pci_err(pdev, "Failed to enumerate dma channels: %d\n",
    chan_cnt);
    rc = -ENXIO;
    goto err_exit;
    }
    swdma_dev.chan_cnt = chan_cnt;
    dma = &swdma_dev.dma_dev;
    dma.copy_align = DMAENGINE_ALIGN_8_BYTES;
    dma_cap_set(DMA_MEMCPY, dma.cap_mask);
    dma_cap_set(DMA_PRIVATE, dma.cap_mask);
    dma.dev = get_device(&pdev.dev);
    dma.device_alloc_chan_resources = switchtec_dma_alloc_chan_resources;
    dma.device_free_chan_resources = switchtec_dma_free_chan_resources;
    dma.device_prep_dma_memcpy = switchtec_dma_prep_memcpy;
    dma.device_tx_status = switchtec_dma_tx_status;
    dma.device_issue_pending = switchtec_dma_issue_pending;
    dma.device_pause = switchtec_dma_pause;
    dma.device_resume = switchtec_dma_resume;
    dma.device_terminate_all = switchtec_dma_terminate_all;
    dma.device_synchronize = switchtec_dma_synchronize;
    dma.device_release = switchtec_dma_release;
    rc = dma_async_device_register(dma);
    if (rc) {
    pci_err(pdev, "Failed to register dma device: %d\n", rc);
    goto err_chans_release_exit;
    }
    pci_dbg(pdev, "Channel count: %d\n", chan_cnt);
    list_for_each_entry(chan, &dma.channels, device_node)
    pci_dbg(pdev, "%s\n", dma_chan_name(chan));
    pci_set_drvdata(pdev, swdma_dev);
    return 0;
    err_chans_release_exit:
    switchtec_dma_chans_release(pdev, swdma_dev);
    err_exit:
    if (swdma_dev.chan_status_irq)
    free_irq(swdma_dev.chan_status_irq, swdma_dev);
    iounmap(swdma_dev.bar);
    kfree(swdma_dev);
    return rc;
    }
    static int switchtec_dma_probe(struct pci_dev *pdev,
    const struct pci_device_id *id)
    {
    int rc;
    rc = pci_enable_device(pdev);
    if (rc)
    return rc;
    dma_set_mask_and_coherent(&pdev.dev, DMA_BIT_MASK(64));
    rc = pci_request_mem_regions(pdev, KBUILD_MODNAME);
    if (rc)
    goto err_disable;
    pci_set_master(pdev);
    rc = switchtec_dma_create(pdev);
    if (rc)
    goto err_free;
    return 0;
    err_free:
    pci_free_irq_vectors(pdev);
    pci_release_mem_regions(pdev);
    err_disable:
    pci_disable_device(pdev);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn switchtec_dma_remove(pdev: *mut pci_dev) {
    static void switchtec_dma_remove(struct pci_dev *pdev)
    {
    struct switchtec_dma_dev *swdma_dev = pci_get_drvdata(pdev);
    switchtec_dma_chans_release(pdev, swdma_dev);
    rcu_assign_pointer(swdma_dev.pdev, core::ptr::null_mut());
    synchronize_rcu();
    pci_free_irq(pdev, swdma_dev.chan_status_irq, swdma_dev);
    pci_free_irq_vectors(pdev);
    dma_async_device_unregister(&swdma_dev.dma_dev);
    iounmap(swdma_dev.bar);
    pci_release_mem_regions(pdev);
    pci_disable_device(pdev);
    }
//
// Also use the class code to identify the devices, as some of the
// device IDs are also used for other devices with other classes by
// Microsemi.
//

    { \
    .vendor     = vendor_id, \
    .device     = device_id, \
    .subvendor  = PCI_ANY_ID, \
    .subdevice  = PCI_ANY_ID, \
    .class      = PCI_CLASS_SYSTEM_OTHER << 8, \
    .class_mask = 0xffffffff, \
    }
    static const struct pci_device_id switchtec_dma_pci_tbl[] = {
    SW_ID(PCI_VENDOR_ID_MICROSEMI, 0x4000), /* PFX 100XG4 */
    SW_ID(PCI_VENDOR_ID_MICROSEMI, 0x4084), /* PFX 84XG4 */
    SW_ID(PCI_VENDOR_ID_MICROSEMI, 0x4068), /* PFX 68XG4 */
    SW_ID(PCI_VENDOR_ID_MICROSEMI, 0x4052), /* PFX 52XG4 */
    SW_ID(PCI_VENDOR_ID_MICROSEMI, 0x4036), /* PFX 36XG4 */
    SW_ID(PCI_VENDOR_ID_MICROSEMI, 0x4028), /* PFX 28XG4 */
    SW_ID(PCI_VENDOR_ID_MICROSEMI, 0x4100), /* PSX 100XG4 */
    SW_ID(PCI_VENDOR_ID_MICROSEMI, 0x4184), /* PSX 84XG4 */
    SW_ID(PCI_VENDOR_ID_MICROSEMI, 0x4168), /* PSX 68XG4 */
    SW_ID(PCI_VENDOR_ID_MICROSEMI, 0x4152), /* PSX 52XG4 */
    SW_ID(PCI_VENDOR_ID_MICROSEMI, 0x4136), /* PSX 36XG4 */
    SW_ID(PCI_VENDOR_ID_MICROSEMI, 0x4128), /* PSX 28XG4 */
    SW_ID(PCI_VENDOR_ID_MICROSEMI, 0x4352), /* PFXA 52XG4 */
    SW_ID(PCI_VENDOR_ID_MICROSEMI, 0x4336), /* PFXA 36XG4 */
    SW_ID(PCI_VENDOR_ID_MICROSEMI, 0x4328), /* PFXA 28XG4 */
    SW_ID(PCI_VENDOR_ID_MICROSEMI, 0x4452), /* PSXA 52XG4 */
    SW_ID(PCI_VENDOR_ID_MICROSEMI, 0x4436), /* PSXA 36XG4 */
    SW_ID(PCI_VENDOR_ID_MICROSEMI, 0x4428), /* PSXA 28XG4 */
    SW_ID(PCI_VENDOR_ID_MICROSEMI, 0x5000), /* PFX 100XG5 */
    SW_ID(PCI_VENDOR_ID_MICROSEMI, 0x5084), /* PFX 84XG5 */
    SW_ID(PCI_VENDOR_ID_MICROSEMI, 0x5068), /* PFX 68XG5 */
    SW_ID(PCI_VENDOR_ID_MICROSEMI, 0x5052), /* PFX 52XG5 */
    SW_ID(PCI_VENDOR_ID_MICROSEMI, 0x5036), /* PFX 36XG5 */
    SW_ID(PCI_VENDOR_ID_MICROSEMI, 0x5028), /* PFX 28XG5 */
    SW_ID(PCI_VENDOR_ID_MICROSEMI, 0x5100), /* PSX 100XG5 */
    SW_ID(PCI_VENDOR_ID_MICROSEMI, 0x5184), /* PSX 84XG5 */
    SW_ID(PCI_VENDOR_ID_MICROSEMI, 0x5168), /* PSX 68XG5 */
    SW_ID(PCI_VENDOR_ID_MICROSEMI, 0x5152), /* PSX 52XG5 */
    SW_ID(PCI_VENDOR_ID_MICROSEMI, 0x5136), /* PSX 36XG5 */
    SW_ID(PCI_VENDOR_ID_MICROSEMI, 0x5128), /* PSX 28XG5 */
    SW_ID(PCI_VENDOR_ID_MICROSEMI, 0x5300), /* PFXA 100XG5 */
    SW_ID(PCI_VENDOR_ID_MICROSEMI, 0x5384), /* PFXA 84XG5 */
    SW_ID(PCI_VENDOR_ID_MICROSEMI, 0x5368), /* PFXA 68XG5 */
    SW_ID(PCI_VENDOR_ID_MICROSEMI, 0x5352), /* PFXA 52XG5 */
    SW_ID(PCI_VENDOR_ID_MICROSEMI, 0x5336), /* PFXA 36XG5 */
    SW_ID(PCI_VENDOR_ID_MICROSEMI, 0x5328), /* PFXA 28XG5 */
    SW_ID(PCI_VENDOR_ID_MICROSEMI, 0x5400), /* PSXA 100XG5 */
    SW_ID(PCI_VENDOR_ID_MICROSEMI, 0x5484), /* PSXA 84XG5 */
    SW_ID(PCI_VENDOR_ID_MICROSEMI, 0x5468), /* PSXA 68XG5 */
    SW_ID(PCI_VENDOR_ID_MICROSEMI, 0x5452), /* PSXA 52XG5 */
    SW_ID(PCI_VENDOR_ID_MICROSEMI, 0x5436), /* PSXA 36XG5 */
    SW_ID(PCI_VENDOR_ID_MICROSEMI, 0x5428), /* PSXA 28XG5 */
    SW_ID(PCI_VENDOR_ID_EFAR,      0x1001), /* PCI1001 16XG4 */
    SW_ID(PCI_VENDOR_ID_EFAR,      0x1002), /* PCI1002 16XG4 */
    SW_ID(PCI_VENDOR_ID_EFAR,      0x1003), /* PCI1003 16XG4 */
    SW_ID(PCI_VENDOR_ID_EFAR,      0x1004), /* PCI1004 16XG4 */
    SW_ID(PCI_VENDOR_ID_EFAR,      0x1005), /* PCI1005 16XG4 */
    SW_ID(PCI_VENDOR_ID_EFAR,      0x1006), /* PCI1006 16XG4 */
    SW_ID(PCI_VENDOR_ID_EFAR,      0x1008), /* PCI1008 16XG4 */
    {0}
    };
    MODULE_DEVICE_TABLE(pci, switchtec_dma_pci_tbl);
    static struct pci_driver switchtec_dma_pci_driver = {
    .name           = KBUILD_MODNAME,
    .id_table       = switchtec_dma_pci_tbl,
    .probe          = switchtec_dma_probe,
    .remove		= switchtec_dma_remove,
    };
    module_pci_driver(switchtec_dma_pci_driver);
