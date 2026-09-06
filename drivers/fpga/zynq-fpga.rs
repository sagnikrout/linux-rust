//! Automatically rewritten from C to Rust
//! Source: drivers/fpga/zynq-fpga.c
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
// Copyright (c) 2011-2015 Xilinx Inc.
// Copyright (c) 2015, National Instruments Corp.
//
// FPGA Manager Driver for Xilinx Zynq, heavily based on xdevcfg driver
// in their vendor tree.
//

// Offsets into SLCR regmap
// FPGA Software Reset Control
pub const SLCR_FPGA_RST_CTRL_OFFSET: c_uint = 0x240;
// Level Shifters Enable
pub const SLCR_LVL_SHFTR_EN_OFFSET: c_uint = 0x900;
// Constant Definitions
// Control Register
pub const CTRL_OFFSET: c_uint = 0x00;
// Lock Register
pub const LOCK_OFFSET: c_uint = 0x04;
// Interrupt Status Register
pub const INT_STS_OFFSET: c_uint = 0x0c;
// Interrupt Mask Register
pub const INT_MASK_OFFSET: c_uint = 0x10;
// Status Register
pub const STATUS_OFFSET: c_uint = 0x14;
// DMA Source Address Register
pub const DMA_SRC_ADDR_OFFSET: c_uint = 0x18;
// DMA Destination Address Reg
pub const DMA_DST_ADDR_OFFSET: c_uint = 0x1c;
// DMA Source Transfer Length
pub const DMA_SRC_LEN_OFFSET: c_uint = 0x20;
// DMA Destination Transfer
pub const DMA_DEST_LEN_OFFSET: c_uint = 0x24;
// Unlock Register
pub const UNLOCK_OFFSET: c_uint = 0x34;
// Misc. Control Register
pub const MCTRL_OFFSET: c_uint = 0x80;
// Control Register Bit definitions
// Signal to reset FPGA

// Enable PCAP for PR

// Enable PCAP

// Lower rate to allow decrypt on the fly

// System booted in secure mode

// Miscellaneous Control Register bit definitions
// Internal PCAP loopback

// Status register bit definitions
// FPGA init status

// Interrupt Status/Mask Register Bit definitions
// DMA command done

// DMA and PCAP cmd done

// FPGA programmed

pub const IXR_ERROR_FLAGS_MASK: c_uint = 0x00F0C860;
pub const IXR_ALL_MASK: c_uint = 0xF8F7F87F;
// Miscellaneous constant values
// Invalid DMA addr

// Used to unlock the dev
pub const UNLOCK_MASK: c_uint = 0x757bdf0d;
// Timeout for polling reset bits
pub const INIT_POLL_TIMEOUT: c_int = 2500000;
// Delay for polling reset bits
pub const INIT_POLL_DELAY: c_int = 20;
// Signal this is the last DMA transfer, wait for the AXI and PCAP before
// interrupting
//
pub const DMA_SRC_LAST_TRANSFER: c_int = 1;
// Timeout for DMA completion
pub const DMA_TIMEOUT_MS: c_int = 5000;
// Masks for controlling stuff in SLCR
// Disable all Level shifters
pub const LVL_SHFTR_DISABLE_ALL_MASK: c_uint = 0x0;
// Enable Level shifters from PS to PL
pub const LVL_SHFTR_ENABLE_PS_TO_PL: c_uint = 0xa;
// Enable Level shifters from PL to PS
pub const LVL_SHFTR_ENABLE_PL_TO_PS: c_uint = 0xf;
// Enable global resets
pub const FPGA_RST_ALL_MASK: c_uint = 0xf;
// Disable global resets
pub const FPGA_RST_NONE_MASK: c_uint = 0x0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct zynq_fpga_priv {
    pub irq: c_int,
    pub clk: *mut clk,
    pub io_base: *mut void __iomem,
    pub slcr: *mut regmap,
    pub dma_lock: spinlock_t,
    pub dma_elm: c_uint,
    pub dma_nelms: c_uint,
    pub cur_sg: *mut scatterlist,
    pub dma_done: completion,
}

    static inline void zynq_fpga_write(struct zynq_fpga_priv *priv, u32 offset,
    u32 val)
    {
    writel(val, priv.io_base + offset);
    }
    static inline u32 zynq_fpga_read(const struct zynq_fpga_priv *priv,
    u32 offset)
    {
    return readl(priv.io_base + offset);
    }

    readl_poll_timeout(priv.io_base + addr, val, cond, sleep_us, \
    timeout_us)
// Cause the specified irq mask bits to generate IRQs
#[no_mangle]
pub unsafe extern "C" fn zynq_fpga_set_irq(priv: *mut zynq_fpga_priv, enable: u32) {
    static inline void zynq_fpga_set_irq(struct zynq_fpga_priv *priv, u32 enable)
    {
    zynq_fpga_write(priv, INT_MASK_OFFSET, ~enable);
    }
// Must be called with dma_lock held
#[no_mangle]
unsafe extern "C" fn zynq_step_dma(priv: *mut zynq_fpga_priv) {
    static void zynq_step_dma(struct zynq_fpga_priv *priv)
    {
    u32 addr;
    u32 len;
    bool first;
    first = priv.dma_elm == 0;
    while (priv.cur_sg) {
// Feed the DMA queue until it is full.
    if (zynq_fpga_read(priv, STATUS_OFFSET) & STATUS_DMA_Q_F)
    break;
    addr = sg_dma_address(priv.cur_sg);
    len = sg_dma_len(priv.cur_sg);
    if (priv.dma_elm + 1 == priv.dma_nelms) {
// The last transfer waits for the PCAP to finish too,
// notice this also changes the irq_mask to ignore
// IXR_DMA_DONE_MASK which ensures we do not trigger
// the completion too early.
//
    addr |= DMA_SRC_LAST_TRANSFER;
    priv.cur_sg = core::ptr::null_mut();
    } else {
    priv.cur_sg = sg_next(priv.cur_sg);
    priv.dma_elm++;
    }
    zynq_fpga_write(priv, DMA_SRC_ADDR_OFFSET, addr);
    zynq_fpga_write(priv, DMA_DST_ADDR_OFFSET, DMA_INVALID_ADDRESS);
    zynq_fpga_write(priv, DMA_SRC_LEN_OFFSET, len / 4);
    zynq_fpga_write(priv, DMA_DEST_LEN_OFFSET, 0);
    }
// Once the first transfer is queued we can turn on the ISR, future
// calls to zynq_step_dma will happen from the ISR context. The
// dma_lock spinlock guarantees this handover is done coherently, the
// ISR enable is put at the end to avoid another CPU spinning in the
// ISR on this lock.
//
    if (first && priv.cur_sg) {
    zynq_fpga_set_irq(priv,
    IXR_DMA_DONE_MASK | IXR_ERROR_FLAGS_MASK);
    } else if (!priv.cur_sg) {
// The last transfer changes to DMA & PCAP mode since we do
// not want to continue until everything has been flushed into
// the PCAP.
//
    zynq_fpga_set_irq(priv,
    IXR_D_P_DONE_MASK | IXR_ERROR_FLAGS_MASK);
    }
    }
#[no_mangle]
unsafe extern "C" fn zynq_fpga_isr(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t zynq_fpga_isr(int irq, void *data)
    {
    struct zynq_fpga_priv *priv = data;
    u32 intr_status;
// If anything other than DMA completion is reported stop and hand
// control back to zynq_fpga_ops_write, something went wrong,
// otherwise progress the DMA.
//
    spin_lock(&priv.dma_lock);
    intr_status = zynq_fpga_read(priv, INT_STS_OFFSET);
    if (!(intr_status & IXR_ERROR_FLAGS_MASK) &&
    (intr_status & IXR_DMA_DONE_MASK) && priv.cur_sg) {
    zynq_fpga_write(priv, INT_STS_OFFSET, IXR_DMA_DONE_MASK);
    zynq_step_dma(priv);
    spin_unlock(&priv.dma_lock);
    return IRQ_HANDLED;
    }
    spin_unlock(&priv.dma_lock);
    zynq_fpga_set_irq(priv, 0);
    complete(&priv.dma_done);
    return IRQ_HANDLED;
    }
// Sanity check the proposed bitstream. It must start with the sync word in
// the correct byte order, and be dword aligned. The input is a Xilinx .bin
// file with every 32 bit quantity swapped.
//
#[no_mangle]
unsafe extern "C" fn zynq_fpga_has_sync(buf: *const u8, count: usize) -> bool {
    static bool zynq_fpga_has_sync(const u8 *buf, size_t count)
    {
    for (; count >= 4; buf += 4, count -= 4)
    if (buf[0] == 0x66 && buf[1] == 0x55 && buf[2] == 0x99 &&
    buf[3] == 0xaa)
    return true;
    return false;
    }
    static int zynq_fpga_ops_write_init(struct fpga_manager *mgr,
    struct fpga_image_info *info,
    const char *buf, size_t count)
    {
    struct zynq_fpga_priv *priv;
    u32 ctrl, status;
    int err;
    priv = mgr.priv;
    err = clk_enable(priv.clk);
    if (err)
    return err;
// check if bitstream is encrypted & and system's still secure
    if (info.flags & FPGA_MGR_ENCRYPTED_BITSTREAM) {
    ctrl = zynq_fpga_read(priv, CTRL_OFFSET);
    if (!(ctrl & CTRL_SEC_EN_MASK)) {
    dev_err(&mgr.dev,
    "System not secure, can't use encrypted bitstreams\n");
    err = -EINVAL;
    goto out_err;
    }
    }
// don't globally reset PL if we're doing partial reconfig
    if (!(info.flags & FPGA_MGR_PARTIAL_RECONFIG)) {
    if (!zynq_fpga_has_sync(buf, count)) {
    dev_err(&mgr.dev,
    "Invalid bitstream, could not find a sync word. Bitstream must be a byte swapped .bin file\n");
    err = -EINVAL;
    goto out_err;
    }
// assert AXI interface resets
    regmap_write(priv.slcr, SLCR_FPGA_RST_CTRL_OFFSET,
    FPGA_RST_ALL_MASK);
// disable all level shifters
    regmap_write(priv.slcr, SLCR_LVL_SHFTR_EN_OFFSET,
    LVL_SHFTR_DISABLE_ALL_MASK);
// enable level shifters from PS to PL
    regmap_write(priv.slcr, SLCR_LVL_SHFTR_EN_OFFSET,
    LVL_SHFTR_ENABLE_PS_TO_PL);
// create a rising edge on PCFG_INIT. PCFG_INIT follows
// PCFG_PROG_B, so we need to poll it after setting PCFG_PROG_B
// to make sure the rising edge actually happens.
// Note: PCFG_PROG_B is low active, sequence as described in
// UG585 v1.10 page 211
//
    ctrl = zynq_fpga_read(priv, CTRL_OFFSET);
    ctrl |= CTRL_PCFG_PROG_B_MASK;
    zynq_fpga_write(priv, CTRL_OFFSET, ctrl);
    err = zynq_fpga_poll_timeout(priv, STATUS_OFFSET, status,
    status & STATUS_PCFG_INIT_MASK,
    INIT_POLL_DELAY,
    INIT_POLL_TIMEOUT);
    if (err) {
    dev_err(&mgr.dev, "Timeout waiting for PCFG_INIT\n");
    goto out_err;
    }
    ctrl = zynq_fpga_read(priv, CTRL_OFFSET);
    ctrl &= ~CTRL_PCFG_PROG_B_MASK;
    zynq_fpga_write(priv, CTRL_OFFSET, ctrl);
    err = zynq_fpga_poll_timeout(priv, STATUS_OFFSET, status,
    !(status & STATUS_PCFG_INIT_MASK),
    INIT_POLL_DELAY,
    INIT_POLL_TIMEOUT);
    if (err) {
    dev_err(&mgr.dev, "Timeout waiting for !PCFG_INIT\n");
    goto out_err;
    }
    ctrl = zynq_fpga_read(priv, CTRL_OFFSET);
    ctrl |= CTRL_PCFG_PROG_B_MASK;
    zynq_fpga_write(priv, CTRL_OFFSET, ctrl);
    err = zynq_fpga_poll_timeout(priv, STATUS_OFFSET, status,
    status & STATUS_PCFG_INIT_MASK,
    INIT_POLL_DELAY,
    INIT_POLL_TIMEOUT);
    if (err) {
    dev_err(&mgr.dev, "Timeout waiting for PCFG_INIT\n");
    goto out_err;
    }
    }
// set configuration register with following options:
// - enable PCAP interface
// - set throughput for maximum speed (if bistream not encrypted)
// - set CPU in user mode
//
    ctrl = zynq_fpga_read(priv, CTRL_OFFSET);
    if (info.flags & FPGA_MGR_ENCRYPTED_BITSTREAM)
    zynq_fpga_write(priv, CTRL_OFFSET,
    (CTRL_PCAP_PR_MASK | CTRL_PCAP_MODE_MASK
    | CTRL_PCAP_RATE_EN_MASK | ctrl));
    else
    zynq_fpga_write(priv, CTRL_OFFSET,
    (CTRL_PCAP_PR_MASK | CTRL_PCAP_MODE_MASK
    | ctrl));
// We expect that the command queue is empty right now.
    status = zynq_fpga_read(priv, STATUS_OFFSET);
    if ((status & STATUS_DMA_Q_F) ||
    (status & STATUS_DMA_Q_E) != STATUS_DMA_Q_E) {
    dev_err(&mgr.dev, "DMA command queue not right\n");
    err = -EBUSY;
    goto out_err;
    }
// ensure internal PCAP loopback is disabled
    ctrl = zynq_fpga_read(priv, MCTRL_OFFSET);
    zynq_fpga_write(priv, MCTRL_OFFSET, (~MCTRL_PCAP_LPBK_MASK & ctrl));
    clk_disable(priv.clk);
    return 0;
    out_err:
    clk_disable(priv.clk);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn zynq_fpga_ops_write(mgr: *mut fpga_manager, sgt: *mut sg_table) -> c_int {
    static int zynq_fpga_ops_write(struct fpga_manager *mgr, struct sg_table *sgt)
    {
    struct zynq_fpga_priv *priv;
    const char *why;
    int err;
    u32 intr_status;
    unsigned long time_left;
    unsigned long flags;
    struct scatterlist *sg;
    int i;
    priv = mgr.priv;
// The hardware can only DMA multiples of 4 bytes, and it requires the
// starting addresses to be aligned to 64 bits (UG585 pg 212).
//
    for_each_sg(sgt.sgl, sg, sgt.nents, i) {
    if ((sg.offset % 8) || (sg.length % 4)) {
    dev_err(&mgr.dev,
    "Invalid bitstream, chunks must be aligned\n");
    return -EINVAL;
    }
    }
    err = dma_map_sgtable(mgr.dev.parent, sgt, DMA_TO_DEVICE, 0);
    if (err) {
    dev_err(&mgr.dev, "Unable to DMA map (TO_DEVICE)\n");
    return err;
    }
    priv.dma_nelms = sgt.nents;
// enable clock
    err = clk_enable(priv.clk);
    if (err)
    goto out_free;
    zynq_fpga_write(priv, INT_STS_OFFSET, IXR_ALL_MASK);
    reinit_completion(&priv.dma_done);
// zynq_step_dma will turn on interrupts
    spin_lock_irqsave(&priv.dma_lock, flags);
    priv.dma_elm = 0;
    priv.cur_sg = sgt.sgl;
    zynq_step_dma(priv);
    spin_unlock_irqrestore(&priv.dma_lock, flags);
    time_left = wait_for_completion_timeout(&priv.dma_done,
    msecs_to_jiffies(DMA_TIMEOUT_MS));
    spin_lock_irqsave(&priv.dma_lock, flags);
    zynq_fpga_set_irq(priv, 0);
    priv.cur_sg = core::ptr::null_mut();
    spin_unlock_irqrestore(&priv.dma_lock, flags);
    intr_status = zynq_fpga_read(priv, INT_STS_OFFSET);
    zynq_fpga_write(priv, INT_STS_OFFSET, IXR_ALL_MASK);
// There doesn't seem to be a way to force cancel any DMA, so if
// something went wrong we are relying on the hardware to have halted
// the DMA before we get here, if there was we could use
// wait_for_completion_interruptible too.
//
    if (intr_status & IXR_ERROR_FLAGS_MASK) {
    why = "DMA reported error";
    err = -EIO;
    goto out_report;
    }
    if (priv.cur_sg ||
    !((intr_status & IXR_D_P_DONE_MASK) == IXR_D_P_DONE_MASK)) {
    if (time_left == 0)
    why = "DMA timed out";
    else
    why = "DMA did not complete";
    err = -EIO;
    goto out_report;
    }
    err = 0;
    goto out_clk;
    out_report:
    dev_err(&mgr.dev,
    "%s: INT_STS:0x%x CTRL:0x%x LOCK:0x%x INT_MASK:0x%x STATUS:0x%x MCTRL:0x%x\n",
    why,
    intr_status,
    zynq_fpga_read(priv, CTRL_OFFSET),
    zynq_fpga_read(priv, LOCK_OFFSET),
    zynq_fpga_read(priv, INT_MASK_OFFSET),
    zynq_fpga_read(priv, STATUS_OFFSET),
    zynq_fpga_read(priv, MCTRL_OFFSET));
    out_clk:
    clk_disable(priv.clk);
    out_free:
    dma_unmap_sgtable(mgr.dev.parent, sgt, DMA_TO_DEVICE, 0);
    return err;
    }
    static int zynq_fpga_ops_write_complete(struct fpga_manager *mgr,
    struct fpga_image_info *info)
    {
    struct zynq_fpga_priv *priv = mgr.priv;
    int err;
    u32 intr_status;
    err = clk_enable(priv.clk);
    if (err)
    return err;
    err = zynq_fpga_poll_timeout(priv, INT_STS_OFFSET, intr_status,
    intr_status & IXR_PCFG_DONE_MASK,
    INIT_POLL_DELAY,
    INIT_POLL_TIMEOUT);
// Release 'PR' control back to the ICAP
    zynq_fpga_write(priv, CTRL_OFFSET,
    zynq_fpga_read(priv, CTRL_OFFSET) & ~CTRL_PCAP_PR_MASK);
    clk_disable(priv.clk);
    if (err)
    return err;
// for the partial reconfig case we didn't touch the level shifters
    if (!(info.flags & FPGA_MGR_PARTIAL_RECONFIG)) {
// enable level shifters from PL to PS
    regmap_write(priv.slcr, SLCR_LVL_SHFTR_EN_OFFSET,
    LVL_SHFTR_ENABLE_PL_TO_PS);
// deassert AXI interface resets
    regmap_write(priv.slcr, SLCR_FPGA_RST_CTRL_OFFSET,
    FPGA_RST_NONE_MASK);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn zynq_fpga_ops_state(mgr: *mut fpga_manager) -> enum fpga_mgr_states {
    static enum fpga_mgr_states zynq_fpga_ops_state(struct fpga_manager *mgr)
    {
    int err;
    u32 intr_status;
    struct zynq_fpga_priv *priv;
    priv = mgr.priv;
    err = clk_enable(priv.clk);
    if (err)
    return FPGA_MGR_STATE_UNKNOWN;
    intr_status = zynq_fpga_read(priv, INT_STS_OFFSET);
    clk_disable(priv.clk);
    if (intr_status & IXR_PCFG_DONE_MASK)
    return FPGA_MGR_STATE_OPERATING;
    return FPGA_MGR_STATE_UNKNOWN;
    }
    static const struct fpga_manager_ops zynq_fpga_ops = {
    .initial_header_size = 128,
    .state = zynq_fpga_ops_state,
    .write_init = zynq_fpga_ops_write_init,
    .write_sg = zynq_fpga_ops_write,
    .write_complete = zynq_fpga_ops_write_complete,
    };
#[no_mangle]
unsafe extern "C" fn zynq_fpga_probe(pdev: *mut platform_device) -> c_int {
    static int zynq_fpga_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct zynq_fpga_priv *priv;
    struct fpga_manager *mgr;
    int err;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    spin_lock_init(&priv.dma_lock);
    priv.io_base = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(priv.io_base))
    return PTR_ERR(priv.io_base);
    priv.slcr = syscon_regmap_lookup_by_phandle(dev.of_node,
    "syscon");
    if (IS_ERR(priv.slcr)) {
    dev_err(dev, "unable to get zynq-slcr regmap\n");
    return PTR_ERR(priv.slcr);
    }
    init_completion(&priv.dma_done);
    priv.irq = platform_get_irq(pdev, 0);
    if (priv.irq < 0)
    return priv.irq;
    priv.clk = devm_clk_get(dev, "ref_clk");
    if (IS_ERR(priv.clk))
    return dev_err_probe(dev, PTR_ERR(priv.clk),
    "input clock not found\n");
    err = clk_prepare_enable(priv.clk);
    if (err) {
    dev_err(dev, "unable to enable clock\n");
    return err;
    }
// unlock the device
    zynq_fpga_write(priv, UNLOCK_OFFSET, UNLOCK_MASK);
    zynq_fpga_set_irq(priv, 0);
    zynq_fpga_write(priv, INT_STS_OFFSET, IXR_ALL_MASK);
    err = devm_request_irq(dev, priv.irq, zynq_fpga_isr, 0, dev_name(dev),
    priv);
    if (err) {
    clk_disable_unprepare(priv.clk);
    return err;
    }
    clk_disable(priv.clk);
    mgr = fpga_mgr_register(dev, "Xilinx Zynq FPGA Manager",
    &zynq_fpga_ops, priv);
    if (IS_ERR(mgr)) {
    dev_err(dev, "unable to register FPGA manager\n");
    clk_unprepare(priv.clk);
    return PTR_ERR(mgr);
    }
    platform_set_drvdata(pdev, mgr);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn zynq_fpga_remove(pdev: *mut platform_device) {
    static void zynq_fpga_remove(struct platform_device *pdev)
    {
    struct zynq_fpga_priv *priv;
    struct fpga_manager *mgr;
    mgr = platform_get_drvdata(pdev);
    priv = mgr.priv;
    fpga_mgr_unregister(mgr);
    clk_unprepare(priv.clk);
    }

    static const struct of_device_id zynq_fpga_of_match[] = {
    { .compatible = "xlnx,zynq-devcfg-1.0", },
    {},
    };
    MODULE_DEVICE_TABLE(of, zynq_fpga_of_match);

    static struct platform_driver zynq_fpga_driver = {
    .probe = zynq_fpga_probe,
    .remove = zynq_fpga_remove,
    .driver = {
    .name = "zynq_fpga_manager",
    .of_match_table = of_match_ptr(zynq_fpga_of_match),
    },
    };
    module_platform_driver(zynq_fpga_driver);
    MODULE_AUTHOR("Moritz Fischer <moritz.fischer@ettus.com>");
    MODULE_AUTHOR("Michal Simek <michal.simek@amd.com>");
    MODULE_DESCRIPTION("Xilinx Zynq FPGA Manager");
    MODULE_LICENSE("GPL v2");
