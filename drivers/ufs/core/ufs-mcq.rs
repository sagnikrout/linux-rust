//! Automatically rewritten from C to Rust
//! Source: drivers/ufs/core/ufs-mcq.c
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
// Copyright (c) 2022 Qualcomm Innovation Center. All rights reserved.
//
// Authors:
// Asutosh Das <quic_asutoshd@quicinc.com>
// Can Guo <quic_cang@quicinc.com>
//

pub const UFS_MCQ_MIN_RW_QUEUES: c_int = 2;
pub const UFS_MCQ_MIN_READ_QUEUES: c_int = 0;
pub const UFS_MCQ_MIN_POLL_QUEUES: c_int = 0;
pub const QUEUE_EN_OFFSET: c_int = 31;
pub const QUEUE_ID_OFFSET: c_int = 16;

pub const MCQ_ENTRY_SIZE_IN_DWORD: c_int = 8;

    UFSHCD_ERROR_MASK |\
    MCQ_CQ_EVENT_STATUS |\
    MCQ_IAG_EVENT_STATUS)
// Max mcq register polling time in microseconds
pub const MCQ_POLL_US: c_int = 500000;
#[no_mangle]
unsafe extern "C" fn rw_queue_count_set(val: *const c_char, kp: *const kernel_param) -> c_int {
    static int rw_queue_count_set(const char *val, const struct kernel_param *kp)
    {
    return param_set_uint_minmax(val, kp, UFS_MCQ_MIN_RW_QUEUES,
    num_possible_cpus());
    }
    static const struct kernel_param_ops rw_queue_count_ops = {
    .set = rw_queue_count_set,
    .get = param_get_uint,
    };
    static unsigned int rw_queues;
    module_param_cb(rw_queues, &rw_queue_count_ops, &rw_queues, 0644);
    MODULE_PARM_DESC(rw_queues,
    "Number of interrupt driven I/O queues used for rw. Default value is nr_cpus");
#[no_mangle]
unsafe extern "C" fn read_queue_count_set(val: *const c_char, kp: *const kernel_param) -> c_int {
    static int read_queue_count_set(const char *val, const struct kernel_param *kp)
    {
    return param_set_uint_minmax(val, kp, UFS_MCQ_MIN_READ_QUEUES,
    num_possible_cpus());
    }
    static const struct kernel_param_ops read_queue_count_ops = {
    .set = read_queue_count_set,
    .get = param_get_uint,
    };
    static unsigned int read_queues;
    module_param_cb(read_queues, &read_queue_count_ops, &read_queues, 0644);
    MODULE_PARM_DESC(read_queues,
    "Number of interrupt driven read queues used for read. Default value is 0");
#[no_mangle]
unsafe extern "C" fn poll_queue_count_set(val: *const c_char, kp: *const kernel_param) -> c_int {
    static int poll_queue_count_set(const char *val, const struct kernel_param *kp)
    {
    return param_set_uint_minmax(val, kp, UFS_MCQ_MIN_POLL_QUEUES,
    num_possible_cpus());
    }
    static const struct kernel_param_ops poll_queue_count_ops = {
    .set = poll_queue_count_set,
    .get = param_get_uint,
    };
    let mut poll_queues: static unsigned int = 1;
    module_param_cb(poll_queues, &poll_queue_count_ops, &poll_queues, 0644);
    MODULE_PARM_DESC(poll_queues,
    "Number of poll queues used for r/w. Default value is 1");
//
// ufshcd_mcq_config_mac - Set the #Max Activ Cmds.
// @hba: per adapter instance
// @max_active_cmds: maximum # of active commands to the device at any time.
//
// The controller won't send more than the max_active_cmds to the device at
// any time.
//
#[no_mangle]
pub unsafe extern "C" fn ufshcd_mcq_config_mac(hba: *mut ufs_hba, max_active_cmds: u32) {
    void ufshcd_mcq_config_mac(struct ufs_hba *hba, u32 max_active_cmds)
    {
    u32 val;
    val = ufshcd_readl(hba, REG_UFS_MCQ_CFG);
    val &= ~MCQ_CFG_MAC_MASK;
    val |= FIELD_PREP(MCQ_CFG_MAC_MASK, max_active_cmds - 1);
    ufshcd_writel(hba, val, REG_UFS_MCQ_CFG);
    }
    EXPORT_SYMBOL_GPL(ufshcd_mcq_config_mac);
//
// ufshcd_mcq_req_to_hwq - find the hardware queue on which the
// request would be issued.
// @hba: per adapter instance
// @req: pointer to the request to be issued
//
// Return: the hardware queue instance on which the request will be or has
// been queued. %NULL if the request has already been freed.
//
    struct ufs_hw_queue *ufshcd_mcq_req_to_hwq(struct ufs_hba *hba,
    struct request *req)
    {
    struct blk_mq_hw_ctx *hctx = READ_ONCE(req.mq_hctx);
    return hctx ? &hba.uhq[hctx.queue_num] : core::ptr::null_mut();
    }
//
// ufshcd_mcq_queue_cfg_addr - get an start address of the MCQ Queue Config
// Registers.
// @hba: per adapter instance
//
// Return: Start address of MCQ Queue Config Registers in HCI
//
#[no_mangle]
pub unsafe extern "C" fn ufshcd_mcq_queue_cfg_addr(hba: *mut ufs_hba) -> c_uint {
    unsigned int ufshcd_mcq_queue_cfg_addr(struct ufs_hba *hba)
    {
    return FIELD_GET(QCFGPTR, hba.mcq_capabilities) * 0x200;
    }
    EXPORT_SYMBOL_GPL(ufshcd_mcq_queue_cfg_addr);
//
// ufshcd_get_hba_mac - Maximum number of commands supported by the host
// controller.
// @hba: per adapter instance
//
// Return: queue depth on success; negative upon error.
//
// MAC = Maximum number of Active Commands supported by the Host Controller.
//
#[no_mangle]
pub unsafe extern "C" fn ufshcd_get_hba_mac(hba: *mut ufs_hba) -> c_int {
    int ufshcd_get_hba_mac(struct ufs_hba *hba)
    {
    int mac;
    if (!hba.vops || !hba.vops.get_hba_mac) {
//
// Extract the maximum number of active transfer tasks value
// from the host controller capabilities register. This value is
// 0-based.
//
    hba.capabilities =
    ufshcd_readl(hba, REG_CONTROLLER_CAPABILITIES);
    mac = hba.capabilities & MASK_TRANSFER_REQUESTS_SLOTS_MCQ;
    mac++;
    } else {
    mac = hba.vops.get_hba_mac(hba);
    }
    if (mac < 0)
    dev_err(hba.dev, "Failed to get mac, err=%d\n", mac);
    return mac;
    }
#[no_mangle]
unsafe extern "C" fn ufshcd_mcq_config_nr_queues(hba: *mut ufs_hba) -> c_int {
    static int ufshcd_mcq_config_nr_queues(struct ufs_hba *hba)
    {
    int i;
    u32 hba_maxq, rem, tot_queues;
    struct Scsi_Host *host = hba.host;
// maxq is 0 based value
    hba_maxq = FIELD_GET(MAX_QUEUE_SUP, hba.mcq_capabilities) + 1;
    tot_queues = read_queues + poll_queues + rw_queues;
    if (hba_maxq < tot_queues) {
    dev_err(hba.dev, "Total queues (%d) exceeds HC capacity (%d)\n",
    tot_queues, hba_maxq);
    return -EOPNOTSUPP;
    }
//
// Device should support at least one I/O queue to handle device
// commands via hba->dev_cmd_queue.
//
    if (hba_maxq == poll_queues) {
    dev_err(hba.dev, "At least one non-poll queue required\n");
    return -EOPNOTSUPP;
    }
    rem = hba_maxq;
    if (rw_queues) {
    hba.nr_queues[HCTX_TYPE_DEFAULT] = rw_queues;
    rem -= hba.nr_queues[HCTX_TYPE_DEFAULT];
    } else {
    rw_queues = num_possible_cpus();
    }
    if (poll_queues) {
    hba.nr_queues[HCTX_TYPE_POLL] = poll_queues;
    rem -= hba.nr_queues[HCTX_TYPE_POLL];
    }
    if (read_queues) {
    hba.nr_queues[HCTX_TYPE_READ] = read_queues;
    rem -= hba.nr_queues[HCTX_TYPE_READ];
    }
    if (!hba.nr_queues[HCTX_TYPE_DEFAULT])
    hba.nr_queues[HCTX_TYPE_DEFAULT] = min3(rem, rw_queues,
    num_possible_cpus());
    for (i = 0; i < HCTX_MAX_TYPES; i++)
    host.nr_hw_queues += hba.nr_queues[i];
    hba.nr_hw_queues = host.nr_hw_queues;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ufshcd_mcq_memory_alloc(hba: *mut ufs_hba) -> c_int {
    int ufshcd_mcq_memory_alloc(struct ufs_hba *hba)
    {
    struct ufs_hw_queue *hwq;
    size_t utrdl_size, cqe_size;
    int i;
    for (i = 0; i < hba.nr_hw_queues; i++) {
    hwq = &hba.uhq[i];
    utrdl_size = sizeof(struct utp_transfer_req_desc) *
    hwq.max_entries;
    hwq.sqe_base_addr = dmam_alloc_coherent(hba.dev, utrdl_size,
    &hwq.sqe_dma_addr,
    GFP_KERNEL);
    if (!hwq.sqe_base_addr) {
    dev_err(hba.dev, "SQE allocation failed\n");
    return -ENOMEM;
    }
    cqe_size = sizeof(struct cq_entry) * hwq.max_entries;
    hwq.cqe_base_addr = dmam_alloc_coherent(hba.dev, cqe_size,
    &hwq.cqe_dma_addr,
    GFP_KERNEL);
    if (!hwq.cqe_base_addr) {
    dev_err(hba.dev, "CQE allocation failed\n");
    return -ENOMEM;
    }
    }
    return 0;
    }
    static void __iomem *mcq_opr_base(struct ufs_hba *hba,
    enum ufshcd_mcq_opr n, int i)
    {
    struct ufshcd_mcq_opr_info_t *opr = &hba.mcq_opr[n];
    return opr.base + opr.stride * i;
    }
#[no_mangle]
pub unsafe extern "C" fn ufshcd_mcq_read_cqis(hba: *mut ufs_hba, i: c_int) -> u32 {
    u32 ufshcd_mcq_read_cqis(struct ufs_hba *hba, int i)
    {
    return readl(mcq_opr_base(hba, OPR_CQIS, i) + REG_CQIS);
    }
    EXPORT_SYMBOL_GPL(ufshcd_mcq_read_cqis);
#[no_mangle]
pub unsafe extern "C" fn ufshcd_mcq_write_cqis(hba: *mut ufs_hba, val: u32, i: c_int) {
    void ufshcd_mcq_write_cqis(struct ufs_hba *hba, u32 val, int i)
    {
    writel(val, mcq_opr_base(hba, OPR_CQIS, i) + REG_CQIS);
    }
    EXPORT_SYMBOL_GPL(ufshcd_mcq_write_cqis);
#[no_mangle]
pub unsafe extern "C" fn ufshcd_mcq_read_mcqiacr(hba: *mut ufs_hba, i: c_int) -> u32 {
    u32 ufshcd_mcq_read_mcqiacr(struct ufs_hba *hba, int i)
    {
    return readl(mcq_opr_base(hba, OPR_CQIS, i) + REG_MCQIACR);
    }
#[no_mangle]
pub unsafe extern "C" fn ufshcd_mcq_write_mcqiacr(hba: *mut ufs_hba, val: u32, i: c_int) {
    void ufshcd_mcq_write_mcqiacr(struct ufs_hba *hba, u32 val, int i)
    {
    writel(val, mcq_opr_base(hba, OPR_CQIS, i) + REG_MCQIACR);
    }
//
// UFSHCI 4.0 MCQ specification doesn't provide a Task Tag or its equivalent in
// the Completion Queue Entry. Find the Task Tag using an indirect method.
// UFSHCI 4.1 and above can directly return the Task Tag in the Completion Queue
// Entry.
//
#[no_mangle]
unsafe extern "C" fn ufshcd_mcq_get_tag(hba: *mut ufs_hba, cqe: *mut cq_entry) -> c_int {
    static int ufshcd_mcq_get_tag(struct ufs_hba *hba, struct cq_entry *cqe)
    {
    u64 addr;
    if (hba.ufs_version >= ufshci_version(4, 1))
    return cqe.task_tag;
// Both UCD types must have a size that is a multiple of 128 bytes
    BUILD_BUG_ON(sizeof(struct utp_transfer_cmd_desc) & GENMASK(6, 0));
    BUILD_BUG_ON(sizeof(struct utp_devman_cmd_desc) & GENMASK(6, 0));
// Bits 63:7 UCD base address, 6:5 are reserved, 4:0 is SQ ID
    addr = le64_to_cpu(cqe.command_desc_base_addr) & CQE_UCD_BA;
// The devman UCD is outside the pool; return its reserved tag.
    if (unlikely(addr == hba.devman_ucd_dma_addr))
    return hba.dev_cmd.tag;
// Pool entries follow the reserved tags.
    return div_u64(addr - hba.ucdl_dma_addr, ufshcd_get_ucd_size(hba)) +
    UFSHCD_NUM_RESERVED;
    }
    static void ufshcd_mcq_process_cqe(struct ufs_hba *hba,
    struct ufs_hw_queue *hwq)
    {
    struct cq_entry *cqe = ufshcd_mcq_cur_cqe(hwq);
    if (cqe.command_desc_base_addr) {
    let mut tag: c_int = ufshcd_mcq_get_tag(hba, cqe);
    ufshcd_compl_one_cqe(hba, tag, cqe);
// After processed the cqe, mark it empty (invalid) entry
    cqe.command_desc_base_addr = 0;
    } else {
    dev_err(hba.dev, "Abnormal CQ entry!\n");
    }
    }
//
// This function is called from the UFS error handler with the UFS host
// controller disabled (HCE = 0). Reading host controller registers, e.g. the
// CQ tail pointer (CQTPy), may not be safe with the host controller disabled.
// Hence, iterate over all completion queue entries. This won't result in
// double completions because ufshcd_mcq_process_cqe() clears a CQE after it
// has been processed.
//
    void ufshcd_mcq_compl_all_cqes_lock(struct ufs_hba *hba,
    struct ufs_hw_queue *hwq)
    {
    unsigned long flags;
    let mut entries: u32 = hwq.max_entries;
    spin_lock_irqsave(&hwq.cq_lock, flags);
    while (entries > 0) {
    ufshcd_mcq_process_cqe(hba, hwq);
    ufshcd_mcq_inc_cq_head_slot(hwq);
    entries--;
    }
    ufshcd_mcq_update_cq_tail_slot(hwq);
    hwq.cq_head_slot = hwq.cq_tail_slot;
    spin_unlock_irqrestore(&hwq.cq_lock, flags);
    }
    unsigned long ufshcd_mcq_poll_cqe_lock(struct ufs_hba *hba,
    struct ufs_hw_queue *hwq)
    {
    let mut completed_reqs: c_ulong = 0;
    unsigned long flags;
    spin_lock_irqsave(&hwq.cq_lock, flags);
    ufshcd_mcq_update_cq_tail_slot(hwq);
    while (!ufshcd_mcq_is_cq_empty(hwq)) {
    ufshcd_mcq_process_cqe(hba, hwq);
    ufshcd_mcq_inc_cq_head_slot(hwq);
    completed_reqs++;
    }
    if (completed_reqs)
    ufshcd_mcq_update_cq_head(hwq);
    spin_unlock_irqrestore(&hwq.cq_lock, flags);
    return completed_reqs;
    }
    EXPORT_SYMBOL_GPL(ufshcd_mcq_poll_cqe_lock);
#[no_mangle]
pub unsafe extern "C" fn ufshcd_mcq_make_queues_operational(hba: *mut ufs_hba) {
    void ufshcd_mcq_make_queues_operational(struct ufs_hba *hba)
    {
    struct ufs_hw_queue *hwq;
    u32 intrs;
    u16 qsize;
    int i;
// Enable required interrupts
    intrs = UFSHCD_ENABLE_MCQ_INTRS;
    if (hba.quirks & UFSHCD_QUIRK_MCQ_BROKEN_INTR)
    intrs &= ~MCQ_CQ_EVENT_STATUS;
    ufshcd_enable_intr(hba, intrs);
    for (i = 0; i < hba.nr_hw_queues; i++) {
    hwq = &hba.uhq[i];
    hwq.id = i;
    qsize = hwq.max_entries * MCQ_ENTRY_SIZE_IN_DWORD - 1;
// Submission Queue Lower Base Address
    ufsmcq_writelx(hba, lower_32_bits(hwq.sqe_dma_addr),
    ufshcd_mcq_cfg_offset(REG_SQLBA, i));
// Submission Queue Upper Base Address
    ufsmcq_writelx(hba, upper_32_bits(hwq.sqe_dma_addr),
    ufshcd_mcq_cfg_offset(REG_SQUBA, i));
// Submission Queue Doorbell Address Offset
    ufsmcq_writelx(hba, ufshcd_mcq_opr_offset(hba, OPR_SQD, i),
    ufshcd_mcq_cfg_offset(REG_SQDAO, i));
// Submission Queue Interrupt Status Address Offset
    ufsmcq_writelx(hba, ufshcd_mcq_opr_offset(hba, OPR_SQIS, i),
    ufshcd_mcq_cfg_offset(REG_SQISAO, i));
// Completion Queue Lower Base Address
    ufsmcq_writelx(hba, lower_32_bits(hwq.cqe_dma_addr),
    ufshcd_mcq_cfg_offset(REG_CQLBA, i));
// Completion Queue Upper Base Address
    ufsmcq_writelx(hba, upper_32_bits(hwq.cqe_dma_addr),
    ufshcd_mcq_cfg_offset(REG_CQUBA, i));
// Completion Queue Doorbell Address Offset
    ufsmcq_writelx(hba, ufshcd_mcq_opr_offset(hba, OPR_CQD, i),
    ufshcd_mcq_cfg_offset(REG_CQDAO, i));
// Completion Queue Interrupt Status Address Offset
    ufsmcq_writelx(hba, ufshcd_mcq_opr_offset(hba, OPR_CQIS, i),
    ufshcd_mcq_cfg_offset(REG_CQISAO, i));
// Save the base addresses for quicker access
    hwq.mcq_sq_head = mcq_opr_base(hba, OPR_SQD, i) + REG_SQHP;
    hwq.mcq_sq_tail = mcq_opr_base(hba, OPR_SQD, i) + REG_SQTP;
    hwq.mcq_cq_head = mcq_opr_base(hba, OPR_CQD, i) + REG_CQHP;
    hwq.mcq_cq_tail = mcq_opr_base(hba, OPR_CQD, i) + REG_CQTP;
// Reinitializing is needed upon HC reset
    hwq.sq_tail_slot = hwq.cq_tail_slot = hwq.cq_head_slot = 0;
// Enable Tail Entry Push Status interrupt only for non-poll queues
    if (i < hba.nr_hw_queues - hba.nr_queues[HCTX_TYPE_POLL])
    writel(1, mcq_opr_base(hba, OPR_CQIS, i) + REG_CQIE);
// Completion Queue Enable|Size to Completion Queue Attribute
    ufsmcq_writel(hba, (1 << QUEUE_EN_OFFSET) | qsize,
    ufshcd_mcq_cfg_offset(REG_CQATTR, i));
//
// Submission Qeueue Enable|Size|Completion Queue ID to
// Submission Queue Attribute
//
    ufsmcq_writel(hba, (1 << QUEUE_EN_OFFSET) | qsize |
    (i << QUEUE_ID_OFFSET),
    ufshcd_mcq_cfg_offset(REG_SQATTR, i));
    }
    }
    EXPORT_SYMBOL_GPL(ufshcd_mcq_make_queues_operational);
#[no_mangle]
pub unsafe extern "C" fn ufshcd_mcq_enable(hba: *mut ufs_hba) {
    void ufshcd_mcq_enable(struct ufs_hba *hba)
    {
    ufshcd_rmwl(hba, MCQ_MODE_SELECT, MCQ_MODE_SELECT, REG_UFS_MEM_CFG);
    hba.mcq_enabled = true;
    }
    EXPORT_SYMBOL_GPL(ufshcd_mcq_enable);
#[no_mangle]
pub unsafe extern "C" fn ufshcd_mcq_disable(hba: *mut ufs_hba) {
    void ufshcd_mcq_disable(struct ufs_hba *hba)
    {
    ufshcd_rmwl(hba, MCQ_MODE_SELECT, 0, REG_UFS_MEM_CFG);
    hba.mcq_enabled = false;
    }
#[no_mangle]
pub unsafe extern "C" fn ufshcd_mcq_enable_esi(hba: *mut ufs_hba) {
    void ufshcd_mcq_enable_esi(struct ufs_hba *hba)
    {
    ufshcd_rmwl(hba, ESI_ENABLE, ESI_ENABLE, REG_UFS_MEM_CFG);
    }
    EXPORT_SYMBOL_GPL(ufshcd_mcq_enable_esi);
#[no_mangle]
pub unsafe extern "C" fn ufshcd_mcq_config_esi(hba: *mut ufs_hba, msg: *mut msi_msg) {
    void ufshcd_mcq_config_esi(struct ufs_hba *hba, struct msi_msg *msg)
    {
    ufshcd_writel(hba, msg.address_lo, REG_UFS_ESILBA);
    ufshcd_writel(hba, msg.address_hi, REG_UFS_ESIUBA);
    }
    EXPORT_SYMBOL_GPL(ufshcd_mcq_config_esi);
#[no_mangle]
pub unsafe extern "C" fn ufshcd_mcq_init(hba: *mut ufs_hba) -> c_int {
    int ufshcd_mcq_init(struct ufs_hba *hba)
    {
    struct ufs_hw_queue *hwq;
    int ret, i;
    ret = ufshcd_mcq_config_nr_queues(hba);
    if (ret)
    return ret;
    ret = ufshcd_vops_mcq_config_resource(hba);
    if (ret)
    return ret;
    ret = ufshcd_mcq_vops_op_runtime_config(hba);
    if (ret) {
    dev_err(hba.dev, "Operation runtime config failed, ret=%d\n",
    ret);
    return ret;
    }
    hba.uhq = devm_kzalloc(hba.dev,
    hba.nr_hw_queues * sizeof(struct ufs_hw_queue),
    GFP_KERNEL);
    if (!hba.uhq) {
    dev_err(hba.dev, "ufs hw queue memory allocation failed\n");
    return -ENOMEM;
    }
    for (i = 0; i < hba.nr_hw_queues; i++) {
    hwq = &hba.uhq[i];
    hwq.max_entries = hba.nutrs + 1;
    spin_lock_init(&hwq.sq_lock);
    spin_lock_init(&hwq.cq_lock);
    mutex_init(&hwq.sq_mutex);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ufshcd_mcq_sq_stop(hba: *mut ufs_hba, hwq: *mut ufs_hw_queue) -> c_int {
    static int ufshcd_mcq_sq_stop(struct ufs_hba *hba, struct ufs_hw_queue *hwq)
    {
    void __iomem *reg;
    let mut id: u32 = hwq.id, val;
    int err;
    if (hba.quirks & UFSHCD_QUIRK_MCQ_BROKEN_RTC)
    return -ETIMEDOUT;
    writel(SQ_STOP, mcq_opr_base(hba, OPR_SQD, id) + REG_SQRTC);
    reg = mcq_opr_base(hba, OPR_SQD, id) + REG_SQRTS;
    err = read_poll_timeout(readl, val, val & SQ_STS, 20,
    MCQ_POLL_US, false, reg);
    if (err)
    dev_err(hba.dev, "%s: failed. hwq-id=%d, err=%d\n",
    __func__, id, err);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ufshcd_mcq_sq_start(hba: *mut ufs_hba, hwq: *mut ufs_hw_queue) -> c_int {
    static int ufshcd_mcq_sq_start(struct ufs_hba *hba, struct ufs_hw_queue *hwq)
    {
    void __iomem *reg;
    let mut id: u32 = hwq.id, val;
    int err;
    if (hba.quirks & UFSHCD_QUIRK_MCQ_BROKEN_RTC)
    return -ETIMEDOUT;
    writel(SQ_START, mcq_opr_base(hba, OPR_SQD, id) + REG_SQRTC);
    reg = mcq_opr_base(hba, OPR_SQD, id) + REG_SQRTS;
    err = read_poll_timeout(readl, val, !(val & SQ_STS), 20,
    MCQ_POLL_US, false, reg);
    if (err)
    dev_err(hba.dev, "%s: failed. hwq-id=%d, err=%d\n",
    __func__, id, err);
    return err;
    }
//
// ufshcd_mcq_sq_cleanup - Clean up submission queue resources
// associated with the pending command.
// @hba: per adapter instance.
// @task_tag: The command's task tag.
//
// Return: 0 for success; error code otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn ufshcd_mcq_sq_cleanup(hba: *mut ufs_hba, task_tag: c_int) -> c_int {
    int ufshcd_mcq_sq_cleanup(struct ufs_hba *hba, int task_tag)
    {
    struct scsi_cmnd *cmd = ufshcd_tag_to_cmd(hba, task_tag);
    struct ufshcd_lrb *lrbp = scsi_cmd_priv(cmd);
    struct request *rq = scsi_cmd_to_rq(cmd);
    struct ufs_hw_queue *hwq;
    void __iomem *reg, *opr_sqd_base;
    u32 nexus, id, val;
    int err;
    if (hba.quirks & UFSHCD_QUIRK_MCQ_BROKEN_RTC)
    return -ETIMEDOUT;
    if (!cmd)
    return -EINVAL;
    hwq = ufshcd_mcq_req_to_hwq(hba, rq);
    if (!hwq)
    return 0;
    id = hwq.id;
    guard(mutex)(&hwq.sq_mutex);
// stop the SQ fetching before working on it
    err = ufshcd_mcq_sq_stop(hba, hwq);
    if (err)
    return err;
// SQCTI = EXT_IID, IID, LUN, Task Tag
    nexus = lrbp.lun << 8 | task_tag;
    opr_sqd_base = mcq_opr_base(hba, OPR_SQD, id);
    writel(nexus, opr_sqd_base + REG_SQCTI);
// Initiate Cleanup
    writel(readl(opr_sqd_base + REG_SQRTC) | SQ_ICU,
    opr_sqd_base + REG_SQRTC);
// Wait until SQRTSy.CUS = 1. Report SQRTSy.RTC.
    reg = opr_sqd_base + REG_SQRTS;
    err = read_poll_timeout(readl, val, val & SQ_CUS, 20,
    MCQ_POLL_US, false, reg);
    if (err)
    dev_err(hba.dev, "%s: failed. hwq=%d, tag=%d err=%d\n",
    __func__, id, task_tag, err);
    else
    dev_info(hba.dev,
    "%s, hwq %d: cleanup return code (RTC) %ld\n",
    __func__, id,
    FIELD_GET(SQ_ICU_ERR_CODE_MASK, readl(reg)));
    if (ufshcd_mcq_sq_start(hba, hwq))
    err = -ETIMEDOUT;
    return err;
    }
//
// ufshcd_mcq_nullify_sqe - Nullify the submission queue entry.
// Write the sqe's Command Type to 0xF. The host controller will not
// fetch any sqe with Command Type = 0xF.
//
// @utrd: UTP Transfer Request Descriptor to be nullified.
//
#[no_mangle]
unsafe extern "C" fn ufshcd_mcq_nullify_sqe(utrd: *mut utp_transfer_req_desc) {
    static void ufshcd_mcq_nullify_sqe(struct utp_transfer_req_desc *utrd)
    {
    utrd.header.command_type = 0xf;
    }
//
// ufshcd_mcq_sqe_search - Search for the command in the submission queue
// If the command is in the submission queue and not issued to the device yet,
// nullify the sqe so the host controller will skip fetching the sqe.
//
// @hba: per adapter instance.
// @hwq: Hardware Queue to be searched.
// @task_tag: The command's task tag.
//
// Return: true if the SQE containing the command is present in the SQ
// (not fetched by the controller); returns false if the SQE is not in the SQ.
//
    static bool ufshcd_mcq_sqe_search(struct ufs_hba *hba,
    struct ufs_hw_queue *hwq, int task_tag)
    {
    struct scsi_cmnd *cmd = ufshcd_tag_to_cmd(hba, task_tag);
    struct ufshcd_lrb *lrbp;
    struct utp_transfer_req_desc *utrd;
    __le64  cmd_desc_base_addr;
    let mut ret: bool = false;
    u64 addr, match;
    u32 sq_head_slot;
    if (hba.quirks & UFSHCD_QUIRK_MCQ_BROKEN_RTC)
    return true;
    if (!cmd)
    return false;
    lrbp = scsi_cmd_priv(cmd);
    mutex_lock(&hwq.sq_mutex);
    ufshcd_mcq_sq_stop(hba, hwq);
    sq_head_slot = ufshcd_mcq_get_sq_head_slot(hwq);
    if (sq_head_slot == hwq.sq_tail_slot)
    goto out;
    cmd_desc_base_addr = lrbp.utr_descriptor_ptr.command_desc_base_addr;
    addr = le64_to_cpu(cmd_desc_base_addr) & CQE_UCD_BA;
    while (sq_head_slot != hwq.sq_tail_slot) {
    utrd = hwq.sqe_base_addr + sq_head_slot;
    match = le64_to_cpu(utrd.command_desc_base_addr) & CQE_UCD_BA;
    if (addr == match) {
    ufshcd_mcq_nullify_sqe(utrd);
    ret = true;
    goto out;
    }
    sq_head_slot++;
    if (sq_head_slot == hwq.max_entries)
    sq_head_slot = 0;
    }
    out:
    ufshcd_mcq_sq_start(hba, hwq);
    mutex_unlock(&hwq.sq_mutex);
    return ret;
    }
//
// ufshcd_mcq_abort - Abort the command in MCQ.
// @cmd: The command to be aborted.
//
// Return: SUCCESS or FAILED error codes
//
#[no_mangle]
pub unsafe extern "C" fn ufshcd_mcq_abort(cmd: *mut scsi_cmnd) -> c_int {
    int ufshcd_mcq_abort(struct scsi_cmnd *cmd)
    {
    struct Scsi_Host *host = cmd.device.host;
    struct ufs_hba *hba = shost_priv(host);
    let mut tag: c_int = scsi_cmd_to_rq(cmd).tag;
    struct ufshcd_lrb *lrbp = scsi_cmd_priv(cmd);
    struct ufs_hw_queue *hwq;
    int err;
// Skip task abort in case previous aborts failed and report failure
    if (lrbp.req_abort_skip) {
    dev_err(hba.dev, "%s: skip abort. tag %d failed earlier\n",
    __func__, tag);
    return FAILED;
    }
    hwq = ufshcd_mcq_req_to_hwq(hba, scsi_cmd_to_rq(cmd));
    if (!hwq) {
    dev_err(hba.dev, "%s: skip abort. cmd at tag %d already completed.\n",
    __func__, tag);
    return FAILED;
    }
    if (ufshcd_mcq_sqe_search(hba, hwq, tag)) {
//
// Failure. The command should not be "stuck" in SQ for
// a long time which resulted in command being aborted.
//
    dev_err(hba.dev, "%s: cmd found in sq. hwq=%d, tag=%d\n",
    __func__, hwq.id, tag);
    return FAILED;
    }
//
// The command is not in the submission queue, and it is not
// in the completion queue either. Query the device to see if
// the command is being processed in the device.
//
    err = ufshcd_try_to_abort_task(hba, tag);
    if (err) {
    dev_err(hba.dev, "%s: device abort failed %d\n", __func__, err);
    lrbp.req_abort_skip = true;
    return FAILED;
    }
    return SUCCESS;
    }
