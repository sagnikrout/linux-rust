//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/marvell/octeon_ep_vf/octep_vf_cnxk.c
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
// Marvell Octeon EP (EndPoint) VF Ethernet Driver
//
// Copyright (C) 2020 Marvell.
//

// Dump useful hardware IQ/OQ CSRs for debug purpose
#[no_mangle]
unsafe extern "C" fn cnxk_vf_dump_q_regs(oct: *mut octep_vf_device, qno: c_int) {
    static void cnxk_vf_dump_q_regs(struct octep_vf_device *oct, int qno)
    {
    struct device *dev = &oct.pdev.dev;
    dev_info(dev, "IQ-%d register dump\n", qno);
    dev_info(dev, "R[%d]_IN_INSTR_DBELL[0x%llx]: 0x%016llx\n",
    qno, CNXK_VF_SDP_R_IN_INSTR_DBELL(qno),
    octep_vf_read_csr64(oct, CNXK_VF_SDP_R_IN_INSTR_DBELL(qno)));
    dev_info(dev, "R[%d]_IN_CONTROL[0x%llx]: 0x%016llx\n",
    qno, CNXK_VF_SDP_R_IN_CONTROL(qno),
    octep_vf_read_csr64(oct, CNXK_VF_SDP_R_IN_CONTROL(qno)));
    dev_info(dev, "R[%d]_IN_ENABLE[0x%llx]: 0x%016llx\n",
    qno, CNXK_VF_SDP_R_IN_ENABLE(qno),
    octep_vf_read_csr64(oct, CNXK_VF_SDP_R_IN_ENABLE(qno)));
    dev_info(dev, "R[%d]_IN_INSTR_BADDR[0x%llx]: 0x%016llx\n",
    qno, CNXK_VF_SDP_R_IN_INSTR_BADDR(qno),
    octep_vf_read_csr64(oct, CNXK_VF_SDP_R_IN_INSTR_BADDR(qno)));
    dev_info(dev, "R[%d]_IN_INSTR_RSIZE[0x%llx]: 0x%016llx\n",
    qno, CNXK_VF_SDP_R_IN_INSTR_RSIZE(qno),
    octep_vf_read_csr64(oct, CNXK_VF_SDP_R_IN_INSTR_RSIZE(qno)));
    dev_info(dev, "R[%d]_IN_CNTS[0x%llx]: 0x%016llx\n",
    qno, CNXK_VF_SDP_R_IN_CNTS(qno),
    octep_vf_read_csr64(oct, CNXK_VF_SDP_R_IN_CNTS(qno)));
    dev_info(dev, "R[%d]_IN_INT_LEVELS[0x%llx]: 0x%016llx\n",
    qno, CNXK_VF_SDP_R_IN_INT_LEVELS(qno),
    octep_vf_read_csr64(oct, CNXK_VF_SDP_R_IN_INT_LEVELS(qno)));
    dev_info(dev, "R[%d]_IN_PKT_CNT[0x%llx]: 0x%016llx\n",
    qno, CNXK_VF_SDP_R_IN_PKT_CNT(qno),
    octep_vf_read_csr64(oct, CNXK_VF_SDP_R_IN_PKT_CNT(qno)));
    dev_info(dev, "R[%d]_IN_BYTE_CNT[0x%llx]: 0x%016llx\n",
    qno, CNXK_VF_SDP_R_IN_BYTE_CNT(qno),
    octep_vf_read_csr64(oct, CNXK_VF_SDP_R_IN_BYTE_CNT(qno)));
    dev_info(dev, "OQ-%d register dump\n", qno);
    dev_info(dev, "R[%d]_OUT_SLIST_DBELL[0x%llx]: 0x%016llx\n",
    qno, CNXK_VF_SDP_R_OUT_SLIST_DBELL(qno),
    octep_vf_read_csr64(oct, CNXK_VF_SDP_R_OUT_SLIST_DBELL(qno)));
    dev_info(dev, "R[%d]_OUT_CONTROL[0x%llx]: 0x%016llx\n",
    qno, CNXK_VF_SDP_R_OUT_CONTROL(qno),
    octep_vf_read_csr64(oct, CNXK_VF_SDP_R_OUT_CONTROL(qno)));
    dev_info(dev, "R[%d]_OUT_ENABLE[0x%llx]: 0x%016llx\n",
    qno, CNXK_VF_SDP_R_OUT_ENABLE(qno),
    octep_vf_read_csr64(oct, CNXK_VF_SDP_R_OUT_ENABLE(qno)));
    dev_info(dev, "R[%d]_OUT_SLIST_BADDR[0x%llx]: 0x%016llx\n",
    qno, CNXK_VF_SDP_R_OUT_SLIST_BADDR(qno),
    octep_vf_read_csr64(oct, CNXK_VF_SDP_R_OUT_SLIST_BADDR(qno)));
    dev_info(dev, "R[%d]_OUT_SLIST_RSIZE[0x%llx]: 0x%016llx\n",
    qno, CNXK_VF_SDP_R_OUT_SLIST_RSIZE(qno),
    octep_vf_read_csr64(oct, CNXK_VF_SDP_R_OUT_SLIST_RSIZE(qno)));
    dev_info(dev, "R[%d]_OUT_CNTS[0x%llx]: 0x%016llx\n",
    qno, CNXK_VF_SDP_R_OUT_CNTS(qno),
    octep_vf_read_csr64(oct, CNXK_VF_SDP_R_OUT_CNTS(qno)));
    dev_info(dev, "R[%d]_OUT_INT_LEVELS[0x%llx]: 0x%016llx\n",
    qno, CNXK_VF_SDP_R_OUT_INT_LEVELS(qno),
    octep_vf_read_csr64(oct, CNXK_VF_SDP_R_OUT_INT_LEVELS(qno)));
    dev_info(dev, "R[%d]_OUT_PKT_CNT[0x%llx]: 0x%016llx\n",
    qno, CNXK_VF_SDP_R_OUT_PKT_CNT(qno),
    octep_vf_read_csr64(oct, CNXK_VF_SDP_R_OUT_PKT_CNT(qno)));
    dev_info(dev, "R[%d]_OUT_BYTE_CNT[0x%llx]: 0x%016llx\n",
    qno, CNXK_VF_SDP_R_OUT_BYTE_CNT(qno),
    octep_vf_read_csr64(oct, CNXK_VF_SDP_R_OUT_BYTE_CNT(qno)));
    dev_info(dev, "R[%d]_ERR_TYPE[0x%llx]: 0x%016llx\n",
    qno, CNXK_VF_SDP_R_ERR_TYPE(qno),
    octep_vf_read_csr64(oct, CNXK_VF_SDP_R_ERR_TYPE(qno)));
    }
// Reset Hardware Tx queue
#[no_mangle]
unsafe extern "C" fn cnxk_vf_reset_iq(oct: *mut octep_vf_device, q_no: c_int) {
    static void cnxk_vf_reset_iq(struct octep_vf_device *oct, int q_no)
    {
    let mut val: u64 = ULL(0);
    dev_dbg(&oct.pdev.dev, "Reset VF IQ-%d\n", q_no);
// Disable the Tx/Instruction Ring
    octep_vf_write_csr64(oct, CNXK_VF_SDP_R_IN_ENABLE(q_no), val);
// clear the Instruction Ring packet/byte counts and doorbell CSRs
    octep_vf_write_csr64(oct, CNXK_VF_SDP_R_IN_INT_LEVELS(q_no), val);
    octep_vf_write_csr64(oct, CNXK_VF_SDP_R_IN_PKT_CNT(q_no), val);
    octep_vf_write_csr64(oct, CNXK_VF_SDP_R_IN_BYTE_CNT(q_no), val);
    octep_vf_write_csr64(oct, CNXK_VF_SDP_R_IN_INSTR_BADDR(q_no), val);
    octep_vf_write_csr64(oct, CNXK_VF_SDP_R_IN_INSTR_RSIZE(q_no), val);
    val = GENMASK_ULL(31, 0);
    octep_vf_write_csr64(oct, CNXK_VF_SDP_R_IN_INSTR_DBELL(q_no), val);
    val = octep_vf_read_csr64(oct, CNXK_VF_SDP_R_IN_CNTS(q_no));
    octep_vf_write_csr64(oct, CNXK_VF_SDP_R_IN_CNTS(q_no), val & GENMASK_ULL(31, 0));
    }
// Reset Hardware Rx queue
#[no_mangle]
unsafe extern "C" fn cnxk_vf_reset_oq(oct: *mut octep_vf_device, q_no: c_int) {
    static void cnxk_vf_reset_oq(struct octep_vf_device *oct, int q_no)
    {
    let mut val: u64 = ULL(0);
// Disable Output (Rx) Ring
    octep_vf_write_csr64(oct, CNXK_VF_SDP_R_OUT_ENABLE(q_no), val);
// Clear count CSRs
    val = octep_vf_read_csr(oct, CNXK_VF_SDP_R_OUT_CNTS(q_no));
    octep_vf_write_csr(oct, CNXK_VF_SDP_R_OUT_CNTS(q_no), val);
    octep_vf_write_csr64(oct, CNXK_VF_SDP_R_OUT_PKT_CNT(q_no), GENMASK_ULL(35, 0));
    octep_vf_write_csr64(oct, CNXK_VF_SDP_R_OUT_SLIST_DBELL(q_no), GENMASK_ULL(31, 0));
    }
// Reset all hardware Tx/Rx queues
#[no_mangle]
unsafe extern "C" fn octep_vf_reset_io_queues_cnxk(oct: *mut octep_vf_device) {
    static void octep_vf_reset_io_queues_cnxk(struct octep_vf_device *oct)
    {
    struct pci_dev *pdev = oct.pdev;
    int q;
    dev_dbg(&pdev.dev, "Reset OCTEP_CNXK VF IO Queues\n");
    for (q = 0; q < CFG_GET_PORTS_ACTIVE_IO_RINGS(oct.conf); q++) {
    cnxk_vf_reset_iq(oct, q);
    cnxk_vf_reset_oq(oct, q);
    }
    }
// Initialize configuration limits and initial active config
#[no_mangle]
unsafe extern "C" fn octep_vf_init_config_cnxk_vf(oct: *mut octep_vf_device) {
    static void octep_vf_init_config_cnxk_vf(struct octep_vf_device *oct)
    {
    struct octep_vf_config *conf = oct.conf;
    u64 reg_val;
    reg_val = octep_vf_read_csr64(oct, CNXK_VF_SDP_R_IN_CONTROL(0));
    conf.ring_cfg.max_io_rings = (reg_val >> CNXK_VF_R_IN_CTL_RPVF_POS) &
    CNXK_VF_R_IN_CTL_RPVF_MASK;
    conf.ring_cfg.active_io_rings = conf.ring_cfg.max_io_rings;
    conf.iq.num_descs = OCTEP_VF_IQ_MAX_DESCRIPTORS;
    conf.iq.instr_type = OCTEP_VF_64BYTE_INSTR;
    conf.iq.db_min = OCTEP_VF_DB_MIN;
    conf.iq.intr_threshold = OCTEP_VF_IQ_INTR_THRESHOLD;
    conf.oq.num_descs = OCTEP_VF_OQ_MAX_DESCRIPTORS;
    conf.oq.buf_size = OCTEP_VF_OQ_BUF_SIZE;
    conf.oq.refill_threshold = OCTEP_VF_OQ_REFILL_THRESHOLD;
    conf.oq.oq_intr_pkt = OCTEP_VF_OQ_INTR_PKT_THRESHOLD;
    conf.oq.oq_intr_time = OCTEP_VF_OQ_INTR_TIME_THRESHOLD;
    conf.oq.wmark = OCTEP_VF_OQ_WMARK_MIN;
    conf.msix_cfg.ioq_msix = conf.ring_cfg.active_io_rings;
    }
// Setup registers for a hardware Tx Queue
#[no_mangle]
unsafe extern "C" fn octep_vf_setup_iq_regs_cnxk(oct: *mut octep_vf_device, iq_no: c_int) {
    static void octep_vf_setup_iq_regs_cnxk(struct octep_vf_device *oct, int iq_no)
    {
    struct octep_vf_iq *iq = oct.iq[iq_no];
    u32 reset_instr_cnt;
    u64 reg_val;
    reg_val = octep_vf_read_csr64(oct, CNXK_VF_SDP_R_IN_CONTROL(iq_no));
// wait for IDLE to set to 1
    if (!(reg_val & CNXK_VF_R_IN_CTL_IDLE)) {
    do {
    reg_val = octep_vf_read_csr64(oct, CNXK_VF_SDP_R_IN_CONTROL(iq_no));
    } while (!(reg_val & CNXK_VF_R_IN_CTL_IDLE));
    }
    reg_val |= CNXK_VF_R_IN_CTL_RDSIZE;
    reg_val |= CNXK_VF_R_IN_CTL_IS_64B;
    reg_val |= CNXK_VF_R_IN_CTL_ESR;
    octep_vf_write_csr64(oct, CNXK_VF_SDP_R_IN_CONTROL(iq_no), reg_val);
// Write the start of the input queue's ring and its size
    octep_vf_write_csr64(oct, CNXK_VF_SDP_R_IN_INSTR_BADDR(iq_no), iq.desc_ring_dma);
    octep_vf_write_csr64(oct, CNXK_VF_SDP_R_IN_INSTR_RSIZE(iq_no), iq.max_count);
// Remember the doorbell & instruction count register addr for this queue
    iq.doorbell_reg = oct.mmio.hw_addr + CNXK_VF_SDP_R_IN_INSTR_DBELL(iq_no);
    iq.inst_cnt_reg = oct.mmio.hw_addr + CNXK_VF_SDP_R_IN_CNTS(iq_no);
    iq.intr_lvl_reg = oct.mmio.hw_addr + CNXK_VF_SDP_R_IN_INT_LEVELS(iq_no);
// Store the current instruction counter (used in flush_iq calculation)
    reset_instr_cnt = readl(iq.inst_cnt_reg);
    writel(reset_instr_cnt, iq.inst_cnt_reg);
// INTR_THRESHOLD is set to max(FFFFFFFF) to disable the INTR
    reg_val = CFG_GET_IQ_INTR_THRESHOLD(oct.conf) & GENMASK_ULL(31, 0);
    octep_vf_write_csr64(oct, CNXK_VF_SDP_R_IN_INT_LEVELS(iq_no), reg_val);
    }
// Setup registers for a hardware Rx Queue
#[no_mangle]
unsafe extern "C" fn octep_vf_setup_oq_regs_cnxk(oct: *mut octep_vf_device, oq_no: c_int) -> c_int {
    static int octep_vf_setup_oq_regs_cnxk(struct octep_vf_device *oct, int oq_no)
    {
    struct octep_vf_oq *oq = oct.oq[oq_no];
    unsigned long t_out_jiffies;
    let mut time_threshold: u32 = 0;
    let mut oq_ctl: u64 = ULL(0);
    u64 reg_ba_val;
    u64 reg_val;
    reg_val = octep_vf_read_csr64(oct, CNXK_VF_SDP_R_OUT_CONTROL(oq_no));
// wait for IDLE to set to 1
    if (!(reg_val & CNXK_VF_R_OUT_CTL_IDLE)) {
    do {
    reg_val = octep_vf_read_csr64(oct, CNXK_VF_SDP_R_OUT_CONTROL(oq_no));
    } while (!(reg_val & CNXK_VF_R_OUT_CTL_IDLE));
    }
    octep_vf_write_csr64(oct, CNXK_VF_SDP_R_OUT_WMARK(oq_no),
    oq.max_count);
// Wait for WMARK to get applied
    usleep_range(10, 15);
    octep_vf_write_csr64(oct, CNXK_VF_SDP_R_OUT_SLIST_BADDR(oq_no),
    oq.desc_ring_dma);
    octep_vf_write_csr64(oct, CNXK_VF_SDP_R_OUT_SLIST_RSIZE(oq_no),
    oq.max_count);
    reg_ba_val = octep_vf_read_csr64(oct,
    CNXK_VF_SDP_R_OUT_SLIST_BADDR(oq_no));
    if (reg_ba_val != oq.desc_ring_dma) {
    t_out_jiffies = jiffies + 10 * HZ;
    do {
    if (reg_ba_val == ULLONG_MAX)
    return -EFAULT;
    octep_vf_write_csr64(oct,
    CNXK_VF_SDP_R_OUT_SLIST_BADDR
    (oq_no), oq.desc_ring_dma);
    octep_vf_write_csr64(oct,
    CNXK_VF_SDP_R_OUT_SLIST_RSIZE
    (oq_no), oq.max_count);
    reg_ba_val =
    octep_vf_read_csr64(oct,
    CNXK_VF_SDP_R_OUT_SLIST_BADDR
    (oq_no));
    } while ((reg_ba_val != oq.desc_ring_dma) &&
    time_before(jiffies, t_out_jiffies));
    if (reg_ba_val != oq.desc_ring_dma)
    return -EAGAIN;
    }
    reg_val &= ~(CNXK_VF_R_OUT_CTL_IMODE);
    reg_val &= ~(CNXK_VF_R_OUT_CTL_ROR_P);
    reg_val &= ~(CNXK_VF_R_OUT_CTL_NSR_P);
    reg_val &= ~(CNXK_VF_R_OUT_CTL_ROR_I);
    reg_val &= ~(CNXK_VF_R_OUT_CTL_NSR_I);
    reg_val &= ~(CNXK_VF_R_OUT_CTL_ES_I);
    reg_val &= ~(CNXK_VF_R_OUT_CTL_ROR_D);
    reg_val &= ~(CNXK_VF_R_OUT_CTL_NSR_D);
    reg_val &= ~(CNXK_VF_R_OUT_CTL_ES_D);
    reg_val |= (CNXK_VF_R_OUT_CTL_ES_P);
    octep_vf_write_csr64(oct, CNXK_VF_SDP_R_OUT_CONTROL(oq_no), reg_val);
    oq_ctl = octep_vf_read_csr64(oct, CNXK_VF_SDP_R_OUT_CONTROL(oq_no));
// Clear the ISIZE and BSIZE (22-0)
    oq_ctl &= ~GENMASK_ULL(22, 0);
// Populate the BSIZE (15-0)
    oq_ctl |= (oq.buffer_size & GENMASK_ULL(15, 0));
    octep_vf_write_csr64(oct, CNXK_VF_SDP_R_OUT_CONTROL(oq_no), oq_ctl);
// Get the mapped address of the pkt_sent and pkts_credit regs
    oq.pkts_sent_reg = oct.mmio.hw_addr + CNXK_VF_SDP_R_OUT_CNTS(oq_no);
    oq.pkts_credit_reg = oct.mmio.hw_addr + CNXK_VF_SDP_R_OUT_SLIST_DBELL(oq_no);
    time_threshold = CFG_GET_OQ_INTR_TIME(oct.conf);
    reg_val = ((u64)time_threshold << 32) | CFG_GET_OQ_INTR_PKT(oct.conf);
    octep_vf_write_csr64(oct, CNXK_VF_SDP_R_OUT_INT_LEVELS(oq_no), reg_val);
// set watermark for backpressure
    reg_val = octep_vf_read_csr64(oct, CNXK_VF_SDP_R_OUT_WMARK(oq_no));
    reg_val &= ~GENMASK_ULL(31, 0);
    reg_val |= CFG_GET_OQ_WMARK(oct.conf);
    octep_vf_write_csr64(oct, CNXK_VF_SDP_R_OUT_WMARK(oq_no), reg_val);
    return 0;
    }
// Setup registers for a VF mailbox
#[no_mangle]
unsafe extern "C" fn octep_vf_setup_mbox_regs_cnxk(oct: *mut octep_vf_device, q_no: c_int) {
    static void octep_vf_setup_mbox_regs_cnxk(struct octep_vf_device *oct, int q_no)
    {
    struct octep_vf_mbox *mbox = oct.mbox;
// PF to VF DATA reg. VF reads from this reg
    mbox.mbox_read_reg = oct.mmio.hw_addr + CNXK_VF_SDP_R_MBOX_PF_VF_DATA(q_no);
// VF mbox interrupt reg
    mbox.mbox_int_reg = oct.mmio.hw_addr + CNXK_VF_SDP_R_MBOX_PF_VF_INT(q_no);
// VF to PF DATA reg. VF writes into this reg
    mbox.mbox_write_reg = oct.mmio.hw_addr + CNXK_VF_SDP_R_MBOX_VF_PF_DATA(q_no);
    }
// Mailbox Interrupt handler
#[no_mangle]
unsafe extern "C" fn cnxk_handle_vf_mbox_intr(oct: *mut octep_vf_device) {
    static void cnxk_handle_vf_mbox_intr(struct octep_vf_device *oct)
    {
    if (oct.mbox)
    schedule_work(&oct.mbox.wk.work);
    else
    dev_err(&oct.pdev.dev, "cannot schedule work on invalid mbox\n");
    }
// Tx/Rx queue interrupt handler
#[no_mangle]
unsafe extern "C" fn octep_vf_ioq_intr_handler_cnxk(data: *mut c_void) -> irqreturn_t {
    static irqreturn_t octep_vf_ioq_intr_handler_cnxk(void *data)
    {
    struct octep_vf_ioq_vector *vector = data;
    struct octep_vf_device *oct;
    struct octep_vf_oq *oq;
    u64 reg_val;
    oct = vector.octep_vf_dev;
    oq = vector.oq;
// Mailbox interrupt arrives along with interrupt of tx/rx ring pair 0
    if (oq.q_no == 0) {
    reg_val = octep_vf_read_csr64(oct, CNXK_VF_SDP_R_MBOX_PF_VF_INT(0));
    if (reg_val & CNXK_VF_SDP_R_MBOX_PF_VF_INT_STATUS) {
    cnxk_handle_vf_mbox_intr(oct);
    octep_vf_write_csr64(oct, CNXK_VF_SDP_R_MBOX_PF_VF_INT(0), reg_val);
    }
    }
    napi_schedule_irqoff(oq.napi);
    return IRQ_HANDLED;
    }
// Re-initialize Octeon hardware registers
#[no_mangle]
unsafe extern "C" fn octep_vf_reinit_regs_cnxk(oct: *mut octep_vf_device) {
    static void octep_vf_reinit_regs_cnxk(struct octep_vf_device *oct)
    {
    u32 i;
    for (i = 0; i < CFG_GET_PORTS_ACTIVE_IO_RINGS(oct.conf); i++)
    oct.hw_ops.setup_iq_regs(oct, i);
    for (i = 0; i < CFG_GET_PORTS_ACTIVE_IO_RINGS(oct.conf); i++)
    oct.hw_ops.setup_oq_regs(oct, i);
    oct.hw_ops.enable_interrupts(oct);
    oct.hw_ops.enable_io_queues(oct);
    for (i = 0; i < CFG_GET_PORTS_ACTIVE_IO_RINGS(oct.conf); i++)
    writel(oct.oq[i].max_count, oct.oq[i].pkts_credit_reg);
    }
// Enable all interrupts
#[no_mangle]
unsafe extern "C" fn octep_vf_enable_interrupts_cnxk(oct: *mut octep_vf_device) {
    static void octep_vf_enable_interrupts_cnxk(struct octep_vf_device *oct)
    {
    int num_rings, q;
    u64 reg_val;
    num_rings = CFG_GET_PORTS_ACTIVE_IO_RINGS(oct.conf);
    for (q = 0; q < num_rings; q++) {
    reg_val = octep_vf_read_csr64(oct, CNXK_VF_SDP_R_IN_INT_LEVELS(q));
    reg_val |= BIT_ULL_MASK(62);
    octep_vf_write_csr64(oct, CNXK_VF_SDP_R_IN_INT_LEVELS(q), reg_val);
    reg_val = octep_vf_read_csr64(oct, CNXK_VF_SDP_R_OUT_INT_LEVELS(q));
    reg_val |= BIT_ULL_MASK(62);
    octep_vf_write_csr64(oct, CNXK_VF_SDP_R_OUT_INT_LEVELS(q), reg_val);
    }
// Enable PF to VF mbox interrupt by setting 2nd bit
    octep_vf_write_csr64(oct, CNXK_VF_SDP_R_MBOX_PF_VF_INT(0),
    CNXK_VF_SDP_R_MBOX_PF_VF_INT_ENAB);
    }
// Disable all interrupts
#[no_mangle]
unsafe extern "C" fn octep_vf_disable_interrupts_cnxk(oct: *mut octep_vf_device) {
    static void octep_vf_disable_interrupts_cnxk(struct octep_vf_device *oct)
    {
    int num_rings, q;
    u64 reg_val;
// Disable PF to VF mbox interrupt by setting 2nd bit
    if (oct.mbox)
    octep_vf_write_csr64(oct, CNXK_VF_SDP_R_MBOX_PF_VF_INT(0), 0x0);
    num_rings = CFG_GET_PORTS_ACTIVE_IO_RINGS(oct.conf);
    for (q = 0; q < num_rings; q++) {
    reg_val = octep_vf_read_csr64(oct, CNXK_VF_SDP_R_IN_INT_LEVELS(q));
    reg_val &= ~BIT_ULL_MASK(62);
    octep_vf_write_csr64(oct, CNXK_VF_SDP_R_IN_INT_LEVELS(q), reg_val);
    reg_val = octep_vf_read_csr64(oct, CNXK_VF_SDP_R_OUT_INT_LEVELS(q));
    reg_val &= ~BIT_ULL_MASK(62);
    octep_vf_write_csr64(oct, CNXK_VF_SDP_R_OUT_INT_LEVELS(q), reg_val);
    }
    }
// Get new Octeon Read Index: index of descriptor that Octeon reads next.
#[no_mangle]
unsafe extern "C" fn octep_vf_update_iq_read_index_cnxk(iq: *mut octep_vf_iq) -> u32 {
    static u32 octep_vf_update_iq_read_index_cnxk(struct octep_vf_iq *iq)
    {
    let mut pkt_in_done: u32 = readl(iq.inst_cnt_reg);
    u32 last_done, new_idx;
    last_done = pkt_in_done - iq.pkt_in_done;
    iq.pkt_in_done = pkt_in_done;
    new_idx = (iq.octep_vf_read_index + last_done) % iq.max_count;
    return new_idx;
    }
// Enable a hardware Tx Queue
#[no_mangle]
unsafe extern "C" fn octep_vf_enable_iq_cnxk(oct: *mut octep_vf_device, iq_no: c_int) {
    static void octep_vf_enable_iq_cnxk(struct octep_vf_device *oct, int iq_no)
    {
    let mut loop: u64 = HZ;
    u64 reg_val;
    octep_vf_write_csr64(oct, CNXK_VF_SDP_R_IN_INSTR_DBELL(iq_no), GENMASK_ULL(31, 0));
    while (octep_vf_read_csr64(oct, CNXK_VF_SDP_R_IN_INSTR_DBELL(iq_no)) &&
    loop--) {
    schedule_timeout_interruptible(1);
    }
    reg_val = octep_vf_read_csr64(oct,  CNXK_VF_SDP_R_IN_INT_LEVELS(iq_no));
    reg_val |= BIT_ULL_MASK(62);
    octep_vf_write_csr64(oct, CNXK_VF_SDP_R_IN_INT_LEVELS(iq_no), reg_val);
    reg_val = octep_vf_read_csr64(oct, CNXK_VF_SDP_R_IN_ENABLE(iq_no));
    reg_val |= ULL(1);
    octep_vf_write_csr64(oct, CNXK_VF_SDP_R_IN_ENABLE(iq_no), reg_val);
    }
// Enable a hardware Rx Queue
#[no_mangle]
unsafe extern "C" fn octep_vf_enable_oq_cnxk(oct: *mut octep_vf_device, oq_no: c_int) {
    static void octep_vf_enable_oq_cnxk(struct octep_vf_device *oct, int oq_no)
    {
    u64 reg_val;
    reg_val = octep_vf_read_csr64(oct,  CNXK_VF_SDP_R_OUT_INT_LEVELS(oq_no));
    reg_val |= BIT_ULL_MASK(62);
    octep_vf_write_csr64(oct, CNXK_VF_SDP_R_OUT_INT_LEVELS(oq_no), reg_val);
    octep_vf_write_csr64(oct, CNXK_VF_SDP_R_OUT_SLIST_DBELL(oq_no), GENMASK_ULL(31, 0));
    reg_val = octep_vf_read_csr64(oct, CNXK_VF_SDP_R_OUT_ENABLE(oq_no));
    reg_val |= ULL(1);
    octep_vf_write_csr64(oct, CNXK_VF_SDP_R_OUT_ENABLE(oq_no), reg_val);
    }
// Enable all hardware Tx/Rx Queues assigned to VF
#[no_mangle]
unsafe extern "C" fn octep_vf_enable_io_queues_cnxk(oct: *mut octep_vf_device) {
    static void octep_vf_enable_io_queues_cnxk(struct octep_vf_device *oct)
    {
    u8 q;
    for (q = 0; q < CFG_GET_PORTS_ACTIVE_IO_RINGS(oct.conf); q++) {
    octep_vf_enable_iq_cnxk(oct, q);
    octep_vf_enable_oq_cnxk(oct, q);
    }
    }
// Disable a hardware Tx Queue assigned to VF
#[no_mangle]
unsafe extern "C" fn octep_vf_disable_iq_cnxk(oct: *mut octep_vf_device, iq_no: c_int) {
    static void octep_vf_disable_iq_cnxk(struct octep_vf_device *oct, int iq_no)
    {
    u64 reg_val;
    reg_val = octep_vf_read_csr64(oct, CNXK_VF_SDP_R_IN_ENABLE(iq_no));
    reg_val &= ~ULL(1);
    octep_vf_write_csr64(oct, CNXK_VF_SDP_R_IN_ENABLE(iq_no), reg_val);
    }
// Disable a hardware Rx Queue assigned to VF
#[no_mangle]
unsafe extern "C" fn octep_vf_disable_oq_cnxk(oct: *mut octep_vf_device, oq_no: c_int) {
    static void octep_vf_disable_oq_cnxk(struct octep_vf_device *oct, int oq_no)
    {
    u64 reg_val;
    reg_val = octep_vf_read_csr64(oct, CNXK_VF_SDP_R_OUT_ENABLE(oq_no));
    reg_val &= ~ULL(1);
    octep_vf_write_csr64(oct, CNXK_VF_SDP_R_OUT_ENABLE(oq_no), reg_val);
    }
// Disable all hardware Tx/Rx Queues assigned to VF
#[no_mangle]
unsafe extern "C" fn octep_vf_disable_io_queues_cnxk(oct: *mut octep_vf_device) {
    static void octep_vf_disable_io_queues_cnxk(struct octep_vf_device *oct)
    {
    int q;
    for (q = 0; q < CFG_GET_PORTS_ACTIVE_IO_RINGS(oct.conf); q++) {
    octep_vf_disable_iq_cnxk(oct, q);
    octep_vf_disable_oq_cnxk(oct, q);
    }
    }
// Dump hardware registers (including Tx/Rx queues) for debugging.
#[no_mangle]
unsafe extern "C" fn octep_vf_dump_registers_cnxk(oct: *mut octep_vf_device) {
    static void octep_vf_dump_registers_cnxk(struct octep_vf_device *oct)
    {
    u8 num_rings, q;
    num_rings = CFG_GET_PORTS_ACTIVE_IO_RINGS(oct.conf);
    for (q = 0; q < num_rings; q++)
    cnxk_vf_dump_q_regs(oct, q);
    }
//
// octep_vf_device_setup_cnxk() - Setup Octeon device.
//
// @oct: Octeon device private data structure.
//
// - initialize hardware operations.
// - get target side pcie port number for the device.
// - set initial configuration and max limits.
//
#[no_mangle]
pub unsafe extern "C" fn octep_vf_device_setup_cnxk(oct: *mut octep_vf_device) {
    void octep_vf_device_setup_cnxk(struct octep_vf_device *oct)
    {
    oct.hw_ops.setup_iq_regs = octep_vf_setup_iq_regs_cnxk;
    oct.hw_ops.setup_oq_regs = octep_vf_setup_oq_regs_cnxk;
    oct.hw_ops.setup_mbox_regs = octep_vf_setup_mbox_regs_cnxk;
    oct.hw_ops.ioq_intr_handler = octep_vf_ioq_intr_handler_cnxk;
    oct.hw_ops.reinit_regs = octep_vf_reinit_regs_cnxk;
    oct.hw_ops.enable_interrupts = octep_vf_enable_interrupts_cnxk;
    oct.hw_ops.disable_interrupts = octep_vf_disable_interrupts_cnxk;
    oct.hw_ops.update_iq_read_idx = octep_vf_update_iq_read_index_cnxk;
    oct.hw_ops.enable_iq = octep_vf_enable_iq_cnxk;
    oct.hw_ops.enable_oq = octep_vf_enable_oq_cnxk;
    oct.hw_ops.enable_io_queues = octep_vf_enable_io_queues_cnxk;
    oct.hw_ops.disable_iq = octep_vf_disable_iq_cnxk;
    oct.hw_ops.disable_oq = octep_vf_disable_oq_cnxk;
    oct.hw_ops.disable_io_queues = octep_vf_disable_io_queues_cnxk;
    oct.hw_ops.reset_io_queues = octep_vf_reset_io_queues_cnxk;
    oct.hw_ops.dump_registers = octep_vf_dump_registers_cnxk;
    octep_vf_init_config_cnxk_vf(oct);
    }
