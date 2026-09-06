//! Automatically rewritten from C to Rust
//! Source: drivers/ntb/hw/mscc/ntb_hw_switchtec.c
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
// Microsemi Switchtec(tm) PCIe Management Driver
// Copyright (c) 2017, Microsemi Corporation
//

    MODULE_DESCRIPTION("Microsemi Switchtec(tm) NTB Driver");
    MODULE_VERSION("0.1");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Microsemi Corporation");
    let mut max_mw_size: static ulong = SZ_2M;
    module_param(max_mw_size, ulong, 0644);
    MODULE_PARM_DESC(max_mw_size,
    "Max memory window size reported to the upper layer");
    static bool use_lut_mws;
    module_param(use_lut_mws, bool, 0644);
    MODULE_PARM_DESC(use_lut_mws,
    "Enable the use of the LUT based memory windows");
pub const SWITCHTEC_NTB_MAGIC: c_uint = 0x45CC0001;
pub const MAX_MWS: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shared_mw {
    pub magic: u32,
    pub link_sta: u32,
    pub partition_id: u32,
    pub mw_sizes: [u64; MAX_MWS],
    pub spad: [u32; 128],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct switchtec_ntb {
    pub ntb: ntb_dev,
    pub stdev: *mut switchtec_dev,
    pub self_partition: c_int,
    pub peer_partition: c_int,
    pub doorbell_irq: c_int,
    pub message_irq: c_int,
    pub mmio_ntb: *mut ntb_info_regs __iomem,
    pub mmio_ctrl: *mut ntb_ctrl_regs __iomem,
    pub mmio_dbmsg: *mut ntb_dbmsg_regs __iomem,
    pub mmio_self_ctrl: *mut ntb_ctrl_regs __iomem,
    pub mmio_peer_ctrl: *mut ntb_ctrl_regs __iomem,
    pub mmio_self_dbmsg: *mut ntb_dbmsg_regs __iomem,
    pub mmio_peer_dbmsg: *mut ntb_dbmsg_regs __iomem,
    pub mmio_xlink_win: *mut void __iomem,
    pub self_shared: *mut shared_mw,
    pub peer_shared: *mut shared_mw __iomem,
    pub self_shared_dma: dma_addr_t,
    pub db_mask: u64,
    pub db_valid_mask: u64,
    pub db_shift: c_int,
    pub db_peer_shift: c_int,
// synchronize rmw access of db_mask and hw reg
    pub db_mask_lock: spinlock_t,
    pub nr_direct_mw: c_int,
    pub nr_lut_mw: c_int,
    pub nr_rsvd_luts: c_int,
    pub direct_mw_to_bar: [c_int; MAX_DIRECT_MW],
    pub peer_nr_direct_mw: c_int,
    pub peer_nr_lut_mw: c_int,
    pub peer_direct_mw_to_bar: [c_int; MAX_DIRECT_MW],
    pub link_is_up: bool,
    pub link_speed: enum ntb_speed,
    pub link_width: enum ntb_width,
    pub check_link_status_work: work_struct,
    pub link_force_down: bool,
}

    static struct switchtec_ntb *ntb_sndev(struct ntb_dev *ntb)
    {
    return container_of(ntb, struct switchtec_ntb, ntb);
    }
    static int switchtec_ntb_part_op(struct switchtec_ntb *sndev,
    struct ntb_ctrl_regs __iomem *ctl,
    u32 op, int wait_status)
    {
    static const char * const op_text[] = {
    [NTB_CTRL_PART_OP_LOCK] = "lock",
    [NTB_CTRL_PART_OP_CFG] = "configure",
    [NTB_CTRL_PART_OP_RESET] = "reset",
    };
    int i;
    u32 ps;
    int status;
    switch (op) {
    case NTB_CTRL_PART_OP_LOCK:
    status = NTB_CTRL_PART_STATUS_LOCKING;
    break;
    case NTB_CTRL_PART_OP_CFG:
    status = NTB_CTRL_PART_STATUS_CONFIGURING;
    break;
    case NTB_CTRL_PART_OP_RESET:
    status = NTB_CTRL_PART_STATUS_RESETTING;
    break;
    default:
    return -EINVAL;
    }
    iowrite32(op, &ctl.partition_op);
    for (i = 0; i < 1000; i++) {
    if (msleep_interruptible(50) != 0) {
    iowrite32(NTB_CTRL_PART_OP_RESET, &ctl.partition_op);
    return -EINTR;
    }
    ps = ioread32(&ctl.partition_status) & 0xFFFF;
    if (ps != status)
    break;
    }
    if (ps == wait_status)
    return 0;
    if (ps == status) {
    dev_err(&sndev.stdev.dev,
    "Timed out while performing %s (%d). (%08x)\n",
    op_text[op], op,
    ioread32(&ctl.partition_status));
    return -ETIMEDOUT;
    }
    return -EIO;
    }
    static int switchtec_ntb_send_msg(struct switchtec_ntb *sndev, int idx,
    u32 val)
    {
    if (idx < 0 || idx >= ARRAY_SIZE(sndev.mmio_peer_dbmsg.omsg))
    return -EINVAL;
    iowrite32(val, &sndev.mmio_peer_dbmsg.omsg[idx].msg);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn switchtec_ntb_mw_count(ntb: *mut ntb_dev, pidx: c_int) -> c_int {
    static int switchtec_ntb_mw_count(struct ntb_dev *ntb, int pidx)
    {
    struct switchtec_ntb *sndev = ntb_sndev(ntb);
    let mut nr_direct_mw: c_int = sndev.peer_nr_direct_mw;
    let mut nr_lut_mw: c_int = sndev.peer_nr_lut_mw - sndev.nr_rsvd_luts;
    if (pidx != NTB_DEF_PEER_IDX)
    return -EINVAL;
    if (!use_lut_mws)
    nr_lut_mw = 0;
    return nr_direct_mw + nr_lut_mw;
    }
#[no_mangle]
unsafe extern "C" fn lut_index(sndev: *mut switchtec_ntb, mw_idx: c_int) -> c_int {
    static int lut_index(struct switchtec_ntb *sndev, int mw_idx)
    {
    return mw_idx - sndev.nr_direct_mw + sndev.nr_rsvd_luts;
    }
#[no_mangle]
unsafe extern "C" fn peer_lut_index(sndev: *mut switchtec_ntb, mw_idx: c_int) -> c_int {
    static int peer_lut_index(struct switchtec_ntb *sndev, int mw_idx)
    {
    return mw_idx - sndev.peer_nr_direct_mw + sndev.nr_rsvd_luts;
    }
    static int switchtec_ntb_mw_get_align(struct ntb_dev *ntb, int pidx,
    int widx, resource_size_t *addr_align,
    resource_size_t *size_align,
    resource_size_t *size_max)
    {
    struct switchtec_ntb *sndev = ntb_sndev(ntb);
    int lut;
    resource_size_t size;
    if (pidx != NTB_DEF_PEER_IDX)
    return -EINVAL;
    lut = widx >= sndev.peer_nr_direct_mw;
    size = ioread64(&sndev.peer_shared.mw_sizes[widx]);
    if (size == 0)
    return -EINVAL;
    if (addr_align)
// addr_align = lut ? size : SZ_4K;
    if (size_align)
// size_align = lut ? size : SZ_4K;
    if (size_max)
// size_max = size;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn switchtec_ntb_mw_clr_direct(sndev: *mut switchtec_ntb, idx: c_int) {
    static void switchtec_ntb_mw_clr_direct(struct switchtec_ntb *sndev, int idx)
    {
    struct ntb_ctrl_regs __iomem *ctl = sndev.mmio_peer_ctrl;
    let mut bar: c_int = sndev.peer_direct_mw_to_bar[idx];
    u32 ctl_val;
    ctl_val = ioread32(&ctl.bar_entry[bar].ctl);
    ctl_val &= ~NTB_CTRL_BAR_DIR_WIN_EN;
    iowrite32(ctl_val, &ctl.bar_entry[bar].ctl);
    iowrite32(0, &ctl.bar_entry[bar].win_size);
    iowrite32(0, &ctl.bar_ext_entry[bar].win_size);
    iowrite64(sndev.self_partition, &ctl.bar_entry[bar].xlate_addr);
    }
#[no_mangle]
unsafe extern "C" fn switchtec_ntb_mw_clr_lut(sndev: *mut switchtec_ntb, idx: c_int) {
    static void switchtec_ntb_mw_clr_lut(struct switchtec_ntb *sndev, int idx)
    {
    struct ntb_ctrl_regs __iomem *ctl = sndev.mmio_peer_ctrl;
    iowrite64(0, &ctl.lut_entry[peer_lut_index(sndev, idx)]);
    }
    static void switchtec_ntb_mw_set_direct(struct switchtec_ntb *sndev, int idx,
    dma_addr_t addr, resource_size_t size)
    {
    let mut xlate_pos: c_int = ilog2(size);
    let mut bar: c_int = sndev.peer_direct_mw_to_bar[idx];
    struct ntb_ctrl_regs __iomem *ctl = sndev.mmio_peer_ctrl;
    u32 ctl_val;
    ctl_val = ioread32(&ctl.bar_entry[bar].ctl);
    ctl_val |= NTB_CTRL_BAR_DIR_WIN_EN;
    iowrite32(ctl_val, &ctl.bar_entry[bar].ctl);
    iowrite32(xlate_pos | (lower_32_bits(size) & 0xFFFFF000),
    &ctl.bar_entry[bar].win_size);
    iowrite32(upper_32_bits(size), &ctl.bar_ext_entry[bar].win_size);
    iowrite64(sndev.self_partition | addr,
    &ctl.bar_entry[bar].xlate_addr);
    }
    static void switchtec_ntb_mw_set_lut(struct switchtec_ntb *sndev, int idx,
    dma_addr_t addr, resource_size_t size)
    {
    struct ntb_ctrl_regs __iomem *ctl = sndev.mmio_peer_ctrl;
    iowrite64((NTB_CTRL_LUT_EN | (sndev.self_partition << 1) | addr),
    &ctl.lut_entry[peer_lut_index(sndev, idx)]);
    }
    static int switchtec_ntb_mw_set_trans(struct ntb_dev *ntb, int pidx, int widx,
    dma_addr_t addr, resource_size_t size)
    {
    struct switchtec_ntb *sndev = ntb_sndev(ntb);
    struct ntb_ctrl_regs __iomem *ctl = sndev.mmio_peer_ctrl;
    let mut xlate_pos: c_int = ilog2(size);
    let mut nr_direct_mw: c_int = sndev.peer_nr_direct_mw;
    int rc;
    if (pidx != NTB_DEF_PEER_IDX)
    return -EINVAL;
    dev_dbg(&sndev.stdev.dev, "MW %d: part %d addr %pad size %pap\n",
    widx, pidx, &addr, &size);
    if (widx >= switchtec_ntb_mw_count(ntb, pidx))
    return -EINVAL;
    if (size != 0 && xlate_pos < 12)
    return -EINVAL;
    if (xlate_pos >= 0 && !IS_ALIGNED(addr, BIT_ULL(xlate_pos))) {
//
// In certain circumstances we can get a buffer that is
// not aligned to its size. (Most of the time
// dma_alloc_coherent ensures this). This can happen when
// using large buffers allocated by the CMA
// (see CMA_CONFIG_ALIGNMENT)
//
    dev_err(&sndev.stdev.dev,
    "ERROR: Memory window address is not aligned to its size!\n");
    return -EINVAL;
    }
    rc = switchtec_ntb_part_op(sndev, ctl, NTB_CTRL_PART_OP_LOCK,
    NTB_CTRL_PART_STATUS_LOCKED);
    if (rc)
    return rc;
    if (size == 0) {
    if (widx < nr_direct_mw)
    switchtec_ntb_mw_clr_direct(sndev, widx);
    else
    switchtec_ntb_mw_clr_lut(sndev, widx);
    } else {
    if (widx < nr_direct_mw)
    switchtec_ntb_mw_set_direct(sndev, widx, addr, size);
    else
    switchtec_ntb_mw_set_lut(sndev, widx, addr, size);
    }
    rc = switchtec_ntb_part_op(sndev, ctl, NTB_CTRL_PART_OP_CFG,
    NTB_CTRL_PART_STATUS_NORMAL);
    if (rc == -EIO) {
    dev_err(&sndev.stdev.dev,
    "Hardware reported an error configuring mw %d: %08x\n",
    widx, ioread32(&ctl.bar_error));
    if (widx < nr_direct_mw)
    switchtec_ntb_mw_clr_direct(sndev, widx);
    else
    switchtec_ntb_mw_clr_lut(sndev, widx);
    switchtec_ntb_part_op(sndev, ctl, NTB_CTRL_PART_OP_CFG,
    NTB_CTRL_PART_STATUS_NORMAL);
    }
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn switchtec_ntb_peer_mw_count(ntb: *mut ntb_dev) -> c_int {
    static int switchtec_ntb_peer_mw_count(struct ntb_dev *ntb)
    {
    struct switchtec_ntb *sndev = ntb_sndev(ntb);
    let mut nr_lut_mw: c_int = sndev.nr_lut_mw - sndev.nr_rsvd_luts;
    return sndev.nr_direct_mw + (use_lut_mws ? nr_lut_mw : 0);
    }
    static int switchtec_ntb_direct_get_addr(struct switchtec_ntb *sndev,
    int idx, phys_addr_t *base,
    resource_size_t *size)
    {
    let mut bar: c_int = sndev.direct_mw_to_bar[idx];
    let mut offset: usize = 0;
    if (bar < 0)
    return -EINVAL;
    if (idx == 0) {
//
// This is the direct BAR shared with the LUTs
// which means the actual window will be offset
// by the size of all the LUT entries.
//
    offset = LUT_SIZE * sndev.nr_lut_mw;
    }
    if (base)
// base = pci_resource_start(sndev->ntb.pdev, bar) + offset;
    if (size) {
// size = pci_resource_len(sndev->ntb.pdev, bar) - offset;
    if (offset && *size > offset)
// size = offset;
    if (*size > max_mw_size)
// size = max_mw_size;
    }
    return 0;
    }
    static int switchtec_ntb_lut_get_addr(struct switchtec_ntb *sndev,
    int idx, phys_addr_t *base,
    resource_size_t *size)
    {
    let mut bar: c_int = sndev.direct_mw_to_bar[0];
    int offset;
    offset = LUT_SIZE * lut_index(sndev, idx);
    if (base)
// base = pci_resource_start(sndev->ntb.pdev, bar) + offset;
    if (size)
// size = LUT_SIZE;
    return 0;
    }
    static int switchtec_ntb_peer_mw_get_addr(struct ntb_dev *ntb, int idx,
    phys_addr_t *base,
    resource_size_t *size)
    {
    struct switchtec_ntb *sndev = ntb_sndev(ntb);
    if (idx < sndev.nr_direct_mw)
    return switchtec_ntb_direct_get_addr(sndev, idx, base, size);
#[no_mangle]
pub unsafe extern "C" fn if(switchtec_ntb_peer_mw_count(ntb): idx <) -> else {
    else if (idx < switchtec_ntb_peer_mw_count(ntb))
    return switchtec_ntb_lut_get_addr(sndev, idx, base, size);
    else
    return -EINVAL;
    }
    static void switchtec_ntb_part_link_speed(struct switchtec_ntb *sndev,
    int partition,
    enum ntb_speed *speed,
    enum ntb_width *width)
    {
    struct switchtec_dev *stdev = sndev.stdev;
    struct part_cfg_regs __iomem *part_cfg =
    &stdev.mmio_part_cfg_all[partition];
    let mut pff: u32 = ioread32(&part_cfg.vep_pff_inst_id) & 0xFF;
    let mut linksta: u32 = ioread32(&stdev.mmio_pff_csr[pff].pci_cap_region[13]);
    if (speed)
// speed = (linksta >> 16) & 0xF;
    if (width)
// width = (linksta >> 20) & 0x3F;
    }
#[no_mangle]
unsafe extern "C" fn switchtec_ntb_set_link_speed(sndev: *mut switchtec_ntb) {
    static void switchtec_ntb_set_link_speed(struct switchtec_ntb *sndev)
    {
    enum ntb_speed self_speed, peer_speed;
    enum ntb_width self_width, peer_width;
    if (!sndev.link_is_up) {
    sndev.link_speed = NTB_SPEED_NONE;
    sndev.link_width = NTB_WIDTH_NONE;
    return;
    }
    switchtec_ntb_part_link_speed(sndev, sndev.self_partition,
    &self_speed, &self_width);
    switchtec_ntb_part_link_speed(sndev, sndev.peer_partition,
    &peer_speed, &peer_width);
    sndev.link_speed = min(self_speed, peer_speed);
    sndev.link_width = min(self_width, peer_width);
    }
#[no_mangle]
unsafe extern "C" fn crosslink_is_enabled(sndev: *mut switchtec_ntb) -> c_int {
    static int crosslink_is_enabled(struct switchtec_ntb *sndev)
    {
    struct ntb_info_regs __iomem *inf = sndev.mmio_ntb;
    return ioread8(&inf.ntp_info[sndev.peer_partition].xlink_enabled);
    }
#[no_mangle]
unsafe extern "C" fn crosslink_init_dbmsgs(sndev: *mut switchtec_ntb) {
    static void crosslink_init_dbmsgs(struct switchtec_ntb *sndev)
    {
    int i;
    let mut msg_map: u32 = 0;
    if (!crosslink_is_enabled(sndev))
    return;
    for (i = 0; i < ARRAY_SIZE(sndev.mmio_peer_dbmsg.imsg); i++) {
    let mut m: c_int = i | sndev.self_partition << 2;
    msg_map |= m << i * 8;
    }
    iowrite32(msg_map, &sndev.mmio_peer_dbmsg.msg_map);
    iowrite64(sndev.db_valid_mask << sndev.db_peer_shift,
    &sndev.mmio_peer_dbmsg.odb_mask);
    }
    enum switchtec_msg {
    LINK_MESSAGE = 0,
    MSG_LINK_UP = 1,
    MSG_LINK_DOWN = 2,
    MSG_CHECK_LINK = 3,
    MSG_LINK_FORCE_DOWN = 4,
    };
    static int switchtec_ntb_reinit_peer(struct switchtec_ntb *sndev);
#[no_mangle]
unsafe extern "C" fn switchtec_ntb_link_status_update(sndev: *mut switchtec_ntb) {
    static void switchtec_ntb_link_status_update(struct switchtec_ntb *sndev)
    {
    int link_sta;
    let mut old: c_int = sndev.link_is_up;
    link_sta = sndev.self_shared.link_sta;
    if (link_sta) {
    let mut peer: u64 = ioread64(&sndev.peer_shared.magic);
    if ((peer & 0xFFFFFFFF) == SWITCHTEC_NTB_MAGIC)
    link_sta = peer >> 32;
    else
    link_sta = 0;
    }
    sndev.link_is_up = link_sta;
    switchtec_ntb_set_link_speed(sndev);
    if (link_sta != old) {
    switchtec_ntb_send_msg(sndev, LINK_MESSAGE, MSG_CHECK_LINK);
    ntb_link_event(&sndev.ntb);
    dev_info(&sndev.stdev.dev, "ntb link %s\n",
    link_sta ? "up" : "down");
    if (link_sta)
    crosslink_init_dbmsgs(sndev);
    }
    }
#[no_mangle]
unsafe extern "C" fn check_link_status_work(work: *mut work_struct) {
    static void check_link_status_work(struct work_struct *work)
    {
    struct switchtec_ntb *sndev;
    sndev = container_of(work, struct switchtec_ntb,
    check_link_status_work);
    if (sndev.link_force_down) {
    sndev.link_force_down = false;
    switchtec_ntb_reinit_peer(sndev);
    if (sndev.link_is_up) {
    sndev.link_is_up = 0;
    ntb_link_event(&sndev.ntb);
    dev_info(&sndev.stdev.dev, "ntb link forced down\n");
    }
    return;
    }
    switchtec_ntb_link_status_update(sndev);
    }
    static void switchtec_ntb_check_link(struct switchtec_ntb *sndev,
    enum switchtec_msg msg)
    {
    if (msg == MSG_LINK_FORCE_DOWN)
    sndev.link_force_down = true;
    schedule_work(&sndev.check_link_status_work);
    }
#[no_mangle]
unsafe extern "C" fn switchtec_ntb_link_notification(stdev: *mut switchtec_dev) {
    static void switchtec_ntb_link_notification(struct switchtec_dev *stdev)
    {
    struct switchtec_ntb *sndev = stdev.sndev;
    switchtec_ntb_check_link(sndev, MSG_CHECK_LINK);
    }
    static u64 switchtec_ntb_link_is_up(struct ntb_dev *ntb,
    enum ntb_speed *speed,
    enum ntb_width *width)
    {
    struct switchtec_ntb *sndev = ntb_sndev(ntb);
    if (speed)
// speed = sndev->link_speed;
    if (width)
// width = sndev->link_width;
    return sndev.link_is_up;
    }
    static int switchtec_ntb_link_enable(struct ntb_dev *ntb,
    enum ntb_speed max_speed,
    enum ntb_width max_width)
    {
    struct switchtec_ntb *sndev = ntb_sndev(ntb);
    dev_dbg(&sndev.stdev.dev, "enabling link\n");
    sndev.self_shared.link_sta = 1;
    switchtec_ntb_send_msg(sndev, LINK_MESSAGE, MSG_LINK_UP);
    switchtec_ntb_link_status_update(sndev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn switchtec_ntb_link_disable(ntb: *mut ntb_dev) -> c_int {
    static int switchtec_ntb_link_disable(struct ntb_dev *ntb)
    {
    struct switchtec_ntb *sndev = ntb_sndev(ntb);
    dev_dbg(&sndev.stdev.dev, "disabling link\n");
    sndev.self_shared.link_sta = 0;
    switchtec_ntb_send_msg(sndev, LINK_MESSAGE, MSG_LINK_DOWN);
    switchtec_ntb_link_status_update(sndev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn switchtec_ntb_db_valid_mask(ntb: *mut ntb_dev) -> u64 {
    static u64 switchtec_ntb_db_valid_mask(struct ntb_dev *ntb)
    {
    struct switchtec_ntb *sndev = ntb_sndev(ntb);
    return sndev.db_valid_mask;
    }
#[no_mangle]
unsafe extern "C" fn switchtec_ntb_db_vector_count(ntb: *mut ntb_dev) -> c_int {
    static int switchtec_ntb_db_vector_count(struct ntb_dev *ntb)
    {
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn switchtec_ntb_db_vector_mask(ntb: *mut ntb_dev, db_vector: c_int) -> u64 {
    static u64 switchtec_ntb_db_vector_mask(struct ntb_dev *ntb, int db_vector)
    {
    struct switchtec_ntb *sndev = ntb_sndev(ntb);
    if (db_vector < 0 || db_vector > 1)
    return 0;
    return sndev.db_valid_mask;
    }
#[no_mangle]
unsafe extern "C" fn switchtec_ntb_db_read(ntb: *mut ntb_dev) -> u64 {
    static u64 switchtec_ntb_db_read(struct ntb_dev *ntb)
    {
    u64 ret;
    struct switchtec_ntb *sndev = ntb_sndev(ntb);
    ret = ioread64(&sndev.mmio_self_dbmsg.idb) >> sndev.db_shift;
    return ret & sndev.db_valid_mask;
    }
#[no_mangle]
unsafe extern "C" fn switchtec_ntb_db_clear(ntb: *mut ntb_dev, db_bits: u64) -> c_int {
    static int switchtec_ntb_db_clear(struct ntb_dev *ntb, u64 db_bits)
    {
    struct switchtec_ntb *sndev = ntb_sndev(ntb);
    iowrite64(db_bits << sndev.db_shift, &sndev.mmio_self_dbmsg.idb);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn switchtec_ntb_db_set_mask(ntb: *mut ntb_dev, db_bits: u64) -> c_int {
    static int switchtec_ntb_db_set_mask(struct ntb_dev *ntb, u64 db_bits)
    {
    unsigned long irqflags;
    struct switchtec_ntb *sndev = ntb_sndev(ntb);
    if (db_bits & ~sndev.db_valid_mask)
    return -EINVAL;
    spin_lock_irqsave(&sndev.db_mask_lock, irqflags);
    sndev.db_mask |= db_bits << sndev.db_shift;
    iowrite64(~sndev.db_mask, &sndev.mmio_self_dbmsg.idb_mask);
    spin_unlock_irqrestore(&sndev.db_mask_lock, irqflags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn switchtec_ntb_db_clear_mask(ntb: *mut ntb_dev, db_bits: u64) -> c_int {
    static int switchtec_ntb_db_clear_mask(struct ntb_dev *ntb, u64 db_bits)
    {
    unsigned long irqflags;
    struct switchtec_ntb *sndev = ntb_sndev(ntb);
    if (db_bits & ~sndev.db_valid_mask)
    return -EINVAL;
    spin_lock_irqsave(&sndev.db_mask_lock, irqflags);
    sndev.db_mask &= ~(db_bits << sndev.db_shift);
    iowrite64(~sndev.db_mask, &sndev.mmio_self_dbmsg.idb_mask);
    spin_unlock_irqrestore(&sndev.db_mask_lock, irqflags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn switchtec_ntb_db_read_mask(ntb: *mut ntb_dev) -> u64 {
    static u64 switchtec_ntb_db_read_mask(struct ntb_dev *ntb)
    {
    struct switchtec_ntb *sndev = ntb_sndev(ntb);
    return (sndev.db_mask >> sndev.db_shift) & sndev.db_valid_mask;
    }
    static int switchtec_ntb_peer_db_addr(struct ntb_dev *ntb,
    phys_addr_t *db_addr,
    resource_size_t *db_size,
    u64 *db_data,
    int db_bit)
    {
    struct switchtec_ntb *sndev = ntb_sndev(ntb);
    unsigned long offset;
    if (unlikely(db_bit >= BITS_PER_LONG_LONG))
    return -EINVAL;
    offset = (unsigned long)sndev.mmio_peer_dbmsg.odb -
    (unsigned long)sndev.stdev.mmio;
    offset += sndev.db_shift / 8;
    if (db_addr)
// db_addr = pci_resource_start(ntb->pdev, 0) + offset;
    if (db_size)
// db_size = sizeof(u32);
    if (db_data)
// db_data = BIT_ULL(db_bit) << sndev->db_peer_shift;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn switchtec_ntb_peer_db_set(ntb: *mut ntb_dev, db_bits: u64) -> c_int {
    static int switchtec_ntb_peer_db_set(struct ntb_dev *ntb, u64 db_bits)
    {
    struct switchtec_ntb *sndev = ntb_sndev(ntb);
    iowrite64(db_bits << sndev.db_peer_shift,
    &sndev.mmio_peer_dbmsg.odb);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn switchtec_ntb_spad_count(ntb: *mut ntb_dev) -> c_int {
    static int switchtec_ntb_spad_count(struct ntb_dev *ntb)
    {
    struct switchtec_ntb *sndev = ntb_sndev(ntb);
    return ARRAY_SIZE(sndev.self_shared.spad);
    }
#[no_mangle]
unsafe extern "C" fn switchtec_ntb_spad_read(ntb: *mut ntb_dev, idx: c_int) -> u32 {
    static u32 switchtec_ntb_spad_read(struct ntb_dev *ntb, int idx)
    {
    struct switchtec_ntb *sndev = ntb_sndev(ntb);
    if (idx < 0 || idx >= ARRAY_SIZE(sndev.self_shared.spad))
    return 0;
    if (!sndev.self_shared)
    return 0;
    return sndev.self_shared.spad[idx];
    }
#[no_mangle]
unsafe extern "C" fn switchtec_ntb_spad_write(ntb: *mut ntb_dev, idx: c_int, val: u32) -> c_int {
    static int switchtec_ntb_spad_write(struct ntb_dev *ntb, int idx, u32 val)
    {
    struct switchtec_ntb *sndev = ntb_sndev(ntb);
    if (idx < 0 || idx >= ARRAY_SIZE(sndev.self_shared.spad))
    return -EINVAL;
    if (!sndev.self_shared)
    return -EIO;
    sndev.self_shared.spad[idx] = val;
    return 0;
    }
    static u32 switchtec_ntb_peer_spad_read(struct ntb_dev *ntb, int pidx,
    int sidx)
    {
    struct switchtec_ntb *sndev = ntb_sndev(ntb);
    if (pidx != NTB_DEF_PEER_IDX)
    return -EINVAL;
    if (sidx < 0 || sidx >= ARRAY_SIZE(sndev.peer_shared.spad))
    return 0;
    if (!sndev.peer_shared)
    return 0;
    return ioread32(&sndev.peer_shared.spad[sidx]);
    }
    static int switchtec_ntb_peer_spad_write(struct ntb_dev *ntb, int pidx,
    int sidx, u32 val)
    {
    struct switchtec_ntb *sndev = ntb_sndev(ntb);
    if (pidx != NTB_DEF_PEER_IDX)
    return -EINVAL;
    if (sidx < 0 || sidx >= ARRAY_SIZE(sndev.peer_shared.spad))
    return -EINVAL;
    if (!sndev.peer_shared)
    return -EIO;
    iowrite32(val, &sndev.peer_shared.spad[sidx]);
    return 0;
    }
    static int switchtec_ntb_peer_spad_addr(struct ntb_dev *ntb, int pidx,
    int sidx, phys_addr_t *spad_addr)
    {
    struct switchtec_ntb *sndev = ntb_sndev(ntb);
    unsigned long offset;
    if (pidx != NTB_DEF_PEER_IDX)
    return -EINVAL;
    offset = (unsigned long)&sndev.peer_shared.spad[sidx] -
    (unsigned long)sndev.stdev.mmio;
    if (spad_addr)
// spad_addr = pci_resource_start(ntb->pdev, 0) + offset;
    return 0;
    }
    static const struct ntb_dev_ops switchtec_ntb_ops = {
    .mw_count		= switchtec_ntb_mw_count,
    .mw_get_align		= switchtec_ntb_mw_get_align,
    .mw_set_trans		= switchtec_ntb_mw_set_trans,
    .peer_mw_count		= switchtec_ntb_peer_mw_count,
    .peer_mw_get_addr	= switchtec_ntb_peer_mw_get_addr,
    .link_is_up		= switchtec_ntb_link_is_up,
    .link_enable		= switchtec_ntb_link_enable,
    .link_disable		= switchtec_ntb_link_disable,
    .db_valid_mask		= switchtec_ntb_db_valid_mask,
    .db_vector_count	= switchtec_ntb_db_vector_count,
    .db_vector_mask		= switchtec_ntb_db_vector_mask,
    .db_read		= switchtec_ntb_db_read,
    .db_clear		= switchtec_ntb_db_clear,
    .db_set_mask		= switchtec_ntb_db_set_mask,
    .db_clear_mask		= switchtec_ntb_db_clear_mask,
    .db_read_mask		= switchtec_ntb_db_read_mask,
    .peer_db_addr		= switchtec_ntb_peer_db_addr,
    .peer_db_set		= switchtec_ntb_peer_db_set,
    .spad_count		= switchtec_ntb_spad_count,
    .spad_read		= switchtec_ntb_spad_read,
    .spad_write		= switchtec_ntb_spad_write,
    .peer_spad_read		= switchtec_ntb_peer_spad_read,
    .peer_spad_write	= switchtec_ntb_peer_spad_write,
    .peer_spad_addr		= switchtec_ntb_peer_spad_addr,
    };
#[no_mangle]
unsafe extern "C" fn switchtec_ntb_init_sndev(sndev: *mut switchtec_ntb) -> c_int {
    static int switchtec_ntb_init_sndev(struct switchtec_ntb *sndev)
    {
    u64 tpart_vec;
    int self;
    u64 part_map;
    sndev.ntb.pdev = sndev.stdev.pdev;
    sndev.ntb.topo = NTB_TOPO_SWITCH;
    sndev.ntb.ops = &switchtec_ntb_ops;
    INIT_WORK(&sndev.check_link_status_work, check_link_status_work);
    sndev.link_force_down = false;
    sndev.self_partition = sndev.stdev.partition;
    sndev.mmio_ntb = sndev.stdev.mmio_ntb;
    self = sndev.self_partition;
    tpart_vec = ioread32(&sndev.mmio_ntb.ntp_info[self].target_part_high);
    tpart_vec <<= 32;
    tpart_vec |= ioread32(&sndev.mmio_ntb.ntp_info[self].target_part_low);
    part_map = ioread64(&sndev.mmio_ntb.ep_map);
    tpart_vec &= part_map;
    part_map &= ~(1 << sndev.self_partition);
    if (!tpart_vec) {
    if (sndev.stdev.partition_count != 2) {
    dev_err(&sndev.stdev.dev,
    "ntb target partition not defined\n");
    return -ENODEV;
    }
    if (!part_map) {
    dev_err(&sndev.stdev.dev,
    "peer partition is not NT partition\n");
    return -ENODEV;
    }
    sndev.peer_partition = __ffs64(part_map);
    } else {
    if (__ffs64(tpart_vec) != (fls64(tpart_vec) - 1)) {
    dev_err(&sndev.stdev.dev,
    "ntb driver only supports 1 pair of 1-1 ntb mapping\n");
    return -ENODEV;
    }
    sndev.peer_partition = __ffs64(tpart_vec);
    if (!(part_map & (1ULL << sndev.peer_partition))) {
    dev_err(&sndev.stdev.dev,
    "ntb target partition is not NT partition\n");
    return -ENODEV;
    }
    }
    dev_dbg(&sndev.stdev.dev, "Partition ID %d of %d\n",
    sndev.self_partition, sndev.stdev.partition_count);
    sndev.mmio_ctrl = (void * __iomem)sndev.mmio_ntb +
    SWITCHTEC_NTB_REG_CTRL_OFFSET;
    sndev.mmio_dbmsg = (void * __iomem)sndev.mmio_ntb +
    SWITCHTEC_NTB_REG_DBMSG_OFFSET;
    sndev.mmio_self_ctrl = &sndev.mmio_ctrl[sndev.self_partition];
    sndev.mmio_peer_ctrl = &sndev.mmio_ctrl[sndev.peer_partition];
    sndev.mmio_self_dbmsg = &sndev.mmio_dbmsg[sndev.self_partition];
    sndev.mmio_peer_dbmsg = sndev.mmio_self_dbmsg;
    return 0;
    }
    static int config_rsvd_lut_win(struct switchtec_ntb *sndev,
    struct ntb_ctrl_regs __iomem *ctl,
    int lut_idx, int partition, u64 addr)
    {
    let mut peer_bar: c_int = sndev.peer_direct_mw_to_bar[0];
    u32 ctl_val;
    int rc;
    rc = switchtec_ntb_part_op(sndev, ctl, NTB_CTRL_PART_OP_LOCK,
    NTB_CTRL_PART_STATUS_LOCKED);
    if (rc)
    return rc;
    ctl_val = ioread32(&ctl.bar_entry[peer_bar].ctl);
    ctl_val &= 0xFF;
    ctl_val |= NTB_CTRL_BAR_LUT_WIN_EN;
    ctl_val |= ilog2(LUT_SIZE) << 8;
    ctl_val |= (sndev.nr_lut_mw - 1) << 14;
    iowrite32(ctl_val, &ctl.bar_entry[peer_bar].ctl);
    iowrite64((NTB_CTRL_LUT_EN | (partition << 1) | addr),
    &ctl.lut_entry[lut_idx]);
    rc = switchtec_ntb_part_op(sndev, ctl, NTB_CTRL_PART_OP_CFG,
    NTB_CTRL_PART_STATUS_NORMAL);
    if (rc) {
    u32 bar_error, lut_error;
    bar_error = ioread32(&ctl.bar_error);
    lut_error = ioread32(&ctl.lut_error);
    dev_err(&sndev.stdev.dev,
    "Error setting up reserved lut window: %08x / %08x\n",
    bar_error, lut_error);
    return rc;
    }
    return 0;
    }
    static int config_req_id_table(struct switchtec_ntb *sndev,
    struct ntb_ctrl_regs __iomem *mmio_ctrl,
    int *req_ids, int count)
    {
    int i, rc = 0;
    u32 error;
    u32 proxy_id;
    if (ioread16(&mmio_ctrl.req_id_table_size) < count) {
    dev_err(&sndev.stdev.dev,
    "Not enough requester IDs available.\n");
    return -EFAULT;
    }
    rc = switchtec_ntb_part_op(sndev, mmio_ctrl,
    NTB_CTRL_PART_OP_LOCK,
    NTB_CTRL_PART_STATUS_LOCKED);
    if (rc)
    return rc;
    for (i = 0; i < count; i++) {
    iowrite32(req_ids[i] << 16 | NTB_CTRL_REQ_ID_EN,
    &mmio_ctrl.req_id_table[i]);
    proxy_id = ioread32(&mmio_ctrl.req_id_table[i]);
    dev_dbg(&sndev.stdev.dev,
    "Requester ID %02X:%02X.%X . BB:%02X.%X\n",
    req_ids[i] >> 8, (req_ids[i] >> 3) & 0x1F,
    req_ids[i] & 0x7, (proxy_id >> 4) & 0x1F,
    (proxy_id >> 1) & 0x7);
    }
    rc = switchtec_ntb_part_op(sndev, mmio_ctrl,
    NTB_CTRL_PART_OP_CFG,
    NTB_CTRL_PART_STATUS_NORMAL);
    if (rc == -EIO) {
    error = ioread32(&mmio_ctrl.req_id_error);
    dev_err(&sndev.stdev.dev,
    "Error setting up the requester ID table: %08x\n",
    error);
    }
    return 0;
    }
    static int crosslink_setup_mws(struct switchtec_ntb *sndev, int ntb_lut_idx,
    u64 *mw_addrs, int mw_count)
    {
    int rc, i;
    struct ntb_ctrl_regs __iomem *ctl = sndev.mmio_self_ctrl;
    u64 addr;
    size_t size, offset;
    int bar;
    int xlate_pos;
    u32 ctl_val;
    rc = switchtec_ntb_part_op(sndev, ctl, NTB_CTRL_PART_OP_LOCK,
    NTB_CTRL_PART_STATUS_LOCKED);
    if (rc)
    return rc;
    for (i = 0; i < sndev.nr_lut_mw; i++) {
    if (i == ntb_lut_idx)
    continue;
    addr = mw_addrs[0] + LUT_SIZE * i;
    iowrite64((NTB_CTRL_LUT_EN | (sndev.peer_partition << 1) |
    addr),
    &ctl.lut_entry[i]);
    }
    sndev.nr_direct_mw = min_t(int, sndev.nr_direct_mw, mw_count);
    for (i = 0; i < sndev.nr_direct_mw; i++) {
    bar = sndev.direct_mw_to_bar[i];
    offset = (i == 0) ? LUT_SIZE * sndev.nr_lut_mw : 0;
    addr = mw_addrs[i] + offset;
    size = pci_resource_len(sndev.ntb.pdev, bar) - offset;
    xlate_pos = ilog2(size);
    if (offset && size > offset)
    size = offset;
    ctl_val = ioread32(&ctl.bar_entry[bar].ctl);
    ctl_val |= NTB_CTRL_BAR_DIR_WIN_EN;
    iowrite32(ctl_val, &ctl.bar_entry[bar].ctl);
    iowrite32(xlate_pos | (lower_32_bits(size) & 0xFFFFF000),
    &ctl.bar_entry[bar].win_size);
    iowrite32(upper_32_bits(size), &ctl.bar_ext_entry[bar].win_size);
    iowrite64(sndev.peer_partition | addr,
    &ctl.bar_entry[bar].xlate_addr);
    }
    rc = switchtec_ntb_part_op(sndev, ctl, NTB_CTRL_PART_OP_CFG,
    NTB_CTRL_PART_STATUS_NORMAL);
    if (rc) {
    u32 bar_error, lut_error;
    bar_error = ioread32(&ctl.bar_error);
    lut_error = ioread32(&ctl.lut_error);
    dev_err(&sndev.stdev.dev,
    "Error setting up cross link windows: %08x / %08x\n",
    bar_error, lut_error);
    return rc;
    }
    return 0;
    }
    static int crosslink_setup_req_ids(struct switchtec_ntb *sndev,
    struct ntb_ctrl_regs __iomem *mmio_ctrl)
    {
    int req_ids[16];
    int i;
    u32 proxy_id;
    for (i = 0; i < ARRAY_SIZE(req_ids); i++) {
    proxy_id = ioread32(&sndev.mmio_self_ctrl.req_id_table[i]);
    if (!(proxy_id & NTB_CTRL_REQ_ID_EN))
    break;
    req_ids[i] = ((proxy_id >> 1) & 0xFF);
    }
    return config_req_id_table(sndev, mmio_ctrl, req_ids, i);
    }
//
// In crosslink configuration there is a virtual partition in the
// middle of the two switches. The BARs in this partition have to be
// enumerated and assigned addresses.
//
    static int crosslink_enum_partition(struct switchtec_ntb *sndev,
    u64 *bar_addrs)
    {
    struct part_cfg_regs __iomem *part_cfg =
    &sndev.stdev.mmio_part_cfg_all[sndev.peer_partition];
    let mut pff: u32 = ioread32(&part_cfg.vep_pff_inst_id) & 0xFF;
    struct pff_csr_regs __iomem *mmio_pff =
    &sndev.stdev.mmio_pff_csr[pff];
    let mut bar_space: u64 = 0x1000000000LL;
    u64 bar_addr;
    let mut bar_cnt: c_int = 0;
    int i;
    iowrite16(0x6, &mmio_pff.pcicmd);
    for (i = 0; i < ARRAY_SIZE(mmio_pff.pci_bar64); i++) {
    iowrite64(bar_space * i, &mmio_pff.pci_bar64[i]);
    bar_addr = ioread64(&mmio_pff.pci_bar64[i]);
    bar_addr &= ~0xf;
    dev_dbg(&sndev.stdev.dev,
    "Crosslink BAR%d addr: %llx\n",
    i*2, bar_addr);
    if (bar_addr != bar_space * i)
    continue;
    bar_addrs[bar_cnt++] = bar_addr;
    }
    return bar_cnt;
    }
#[no_mangle]
unsafe extern "C" fn switchtec_ntb_init_crosslink(sndev: *mut switchtec_ntb) -> c_int {
    static int switchtec_ntb_init_crosslink(struct switchtec_ntb *sndev)
    {
    int rc;
    let mut bar: c_int = sndev.direct_mw_to_bar[0];
    let mut ntb_lut_idx: c_int = 1;
    u64 bar_addrs[6];
    u64 addr;
    int offset;
    int bar_cnt;
    if (!crosslink_is_enabled(sndev))
    return 0;
    dev_info(&sndev.stdev.dev, "Using crosslink configuration\n");
    sndev.ntb.topo = NTB_TOPO_CROSSLINK;
    bar_cnt = crosslink_enum_partition(sndev, bar_addrs);
    if (bar_cnt < sndev.nr_direct_mw + 1) {
    dev_err(&sndev.stdev.dev,
    "Error enumerating crosslink partition\n");
    return -EINVAL;
    }
    addr = (bar_addrs[0] + SWITCHTEC_GAS_NTB_OFFSET +
    SWITCHTEC_NTB_REG_DBMSG_OFFSET +
    sizeof(struct ntb_dbmsg_regs) * sndev.peer_partition);
    offset = addr & (LUT_SIZE - 1);
    addr -= offset;
    rc = config_rsvd_lut_win(sndev, sndev.mmio_self_ctrl, ntb_lut_idx,
    sndev.peer_partition, addr);
    if (rc)
    return rc;
    rc = crosslink_setup_mws(sndev, ntb_lut_idx, &bar_addrs[1],
    bar_cnt - 1);
    if (rc)
    return rc;
    rc = crosslink_setup_req_ids(sndev, sndev.mmio_peer_ctrl);
    if (rc)
    return rc;
    sndev.mmio_xlink_win = pci_iomap_range(sndev.stdev.pdev, bar,
    LUT_SIZE, LUT_SIZE);
    if (!sndev.mmio_xlink_win) {
    rc = -ENOMEM;
    return rc;
    }
    sndev.mmio_peer_dbmsg = sndev.mmio_xlink_win + offset;
    sndev.nr_rsvd_luts++;
    crosslink_init_dbmsgs(sndev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn switchtec_ntb_deinit_crosslink(sndev: *mut switchtec_ntb) {
    static void switchtec_ntb_deinit_crosslink(struct switchtec_ntb *sndev)
    {
    if (sndev.mmio_xlink_win)
    pci_iounmap(sndev.stdev.pdev, sndev.mmio_xlink_win);
    }
#[no_mangle]
unsafe extern "C" fn map_bars(map: *mut c_int, ctrl: *mut ntb_ctrl_regs __iomem) -> c_int {
    static int map_bars(int *map, struct ntb_ctrl_regs __iomem *ctrl)
    {
    int i;
    let mut cnt: c_int = 0;
    for (i = 0; i < ARRAY_SIZE(ctrl.bar_entry); i++) {
    let mut r: u32 = ioread32(&ctrl.bar_entry[i].ctl);
    if (r & NTB_CTRL_BAR_VALID)
    map[cnt++] = i;
    }
    return cnt;
    }
#[no_mangle]
unsafe extern "C" fn switchtec_ntb_init_mw(sndev: *mut switchtec_ntb) {
    static void switchtec_ntb_init_mw(struct switchtec_ntb *sndev)
    {
    sndev.nr_direct_mw = map_bars(sndev.direct_mw_to_bar,
    sndev.mmio_self_ctrl);
    sndev.nr_lut_mw = ioread16(&sndev.mmio_self_ctrl.lut_table_entries);
    if (sndev.nr_lut_mw)
    sndev.nr_lut_mw = rounddown_pow_of_two(sndev.nr_lut_mw);
    dev_dbg(&sndev.stdev.dev, "MWs: %d direct, %d lut\n",
    sndev.nr_direct_mw, sndev.nr_lut_mw);
    sndev.peer_nr_direct_mw = map_bars(sndev.peer_direct_mw_to_bar,
    sndev.mmio_peer_ctrl);
    sndev.peer_nr_lut_mw =
    ioread16(&sndev.mmio_peer_ctrl.lut_table_entries);
    if (sndev.peer_nr_lut_mw)
    sndev.peer_nr_lut_mw = rounddown_pow_of_two(sndev.peer_nr_lut_mw);
    dev_dbg(&sndev.stdev.dev, "Peer MWs: %d direct, %d lut\n",
    sndev.peer_nr_direct_mw, sndev.peer_nr_lut_mw);
    }
//
// There are 64 doorbells in the switch hardware but this is
// shared among all partitions. So we must split them in half
// (32 for each partition). However, the message interrupts are
// also shared with the top 4 doorbells so we just limit this to
// 28 doorbells per partition.
//
// In crosslink mode, each side has it's own dbmsg register so
// they can each use all 60 of the available doorbells.
//
#[no_mangle]
unsafe extern "C" fn switchtec_ntb_init_db(sndev: *mut switchtec_ntb) {
    static void switchtec_ntb_init_db(struct switchtec_ntb *sndev)
    {
    sndev.db_mask = 0x0FFFFFFFFFFFFFFFULL;
    if (sndev.mmio_peer_dbmsg != sndev.mmio_self_dbmsg) {
    sndev.db_shift = 0;
    sndev.db_peer_shift = 0;
    sndev.db_valid_mask = sndev.db_mask;
    } else if (sndev.self_partition < sndev.peer_partition) {
    sndev.db_shift = 0;
    sndev.db_peer_shift = 32;
    sndev.db_valid_mask = 0x0FFFFFFF;
    } else {
    sndev.db_shift = 32;
    sndev.db_peer_shift = 0;
    sndev.db_valid_mask = 0x0FFFFFFF;
    }
    iowrite64(~sndev.db_mask, &sndev.mmio_self_dbmsg.idb_mask);
    iowrite64(sndev.db_valid_mask << sndev.db_peer_shift,
    &sndev.mmio_peer_dbmsg.odb_mask);
    dev_dbg(&sndev.stdev.dev, "dbs: shift %d/%d, mask %016llx\n",
    sndev.db_shift, sndev.db_peer_shift, sndev.db_valid_mask);
    }
#[no_mangle]
unsafe extern "C" fn switchtec_ntb_init_msgs(sndev: *mut switchtec_ntb) {
    static void switchtec_ntb_init_msgs(struct switchtec_ntb *sndev)
    {
    int i;
    let mut msg_map: u32 = 0;
    for (i = 0; i < ARRAY_SIZE(sndev.mmio_self_dbmsg.imsg); i++) {
    let mut m: c_int = i | sndev.peer_partition << 2;
    msg_map |= m << i * 8;
    }
    iowrite32(msg_map, &sndev.mmio_self_dbmsg.msg_map);
    for (i = 0; i < ARRAY_SIZE(sndev.mmio_self_dbmsg.imsg); i++)
    iowrite64(NTB_DBMSG_IMSG_STATUS | NTB_DBMSG_IMSG_MASK,
    &sndev.mmio_self_dbmsg.imsg[i]);
    }
    static int
    switchtec_ntb_init_req_id_table(struct switchtec_ntb *sndev)
    {
    int req_ids[2];
//
// Root Complex Requester ID (which is 0:00.0)
//
    req_ids[0] = 0;
//
// Host Bridge Requester ID (as read from the mmap address)
//
    req_ids[1] = ioread16(&sndev.mmio_ntb.requester_id);
    return config_req_id_table(sndev, sndev.mmio_self_ctrl, req_ids,
    ARRAY_SIZE(req_ids));
    }
#[no_mangle]
unsafe extern "C" fn switchtec_ntb_init_shared(sndev: *mut switchtec_ntb) {
    static void switchtec_ntb_init_shared(struct switchtec_ntb *sndev)
    {
    int i;
    memset(sndev.self_shared, 0, LUT_SIZE);
    sndev.self_shared.magic = SWITCHTEC_NTB_MAGIC;
    sndev.self_shared.partition_id = sndev.stdev.partition;
    for (i = 0; i < sndev.nr_direct_mw; i++) {
    let mut bar: c_int = sndev.direct_mw_to_bar[i];
    let mut sz: resource_size_t = pci_resource_len(sndev.stdev.pdev, bar);
    if (i == 0)
    sz = min_t(resource_size_t, sz,
    LUT_SIZE * sndev.nr_lut_mw);
    sndev.self_shared.mw_sizes[i] = sz;
    }
    for (i = 0; i < sndev.nr_lut_mw; i++) {
    let mut idx: c_int = sndev.nr_direct_mw + i;
    if (idx >= MAX_MWS) {
    dev_err(&sndev.stdev.dev,
    "Total number of MW cannot be bigger than %d", MAX_MWS);
    break;
    }
    sndev.self_shared.mw_sizes[idx] = LUT_SIZE;
    }
    }
#[no_mangle]
unsafe extern "C" fn switchtec_ntb_init_shared_mw(sndev: *mut switchtec_ntb) -> c_int {
    static int switchtec_ntb_init_shared_mw(struct switchtec_ntb *sndev)
    {
    let mut self_bar: c_int = sndev.direct_mw_to_bar[0];
    int rc;
    sndev.nr_rsvd_luts++;
    sndev.self_shared = dma_alloc_coherent(&sndev.stdev.pdev.dev,
    LUT_SIZE,
    &sndev.self_shared_dma,
    GFP_KERNEL);
    if (!sndev.self_shared) {
    dev_err(&sndev.stdev.dev,
    "unable to allocate memory for shared mw\n");
    return -ENOMEM;
    }
    switchtec_ntb_init_shared(sndev);
    rc = config_rsvd_lut_win(sndev, sndev.mmio_peer_ctrl, 0,
    sndev.self_partition,
    sndev.self_shared_dma);
    if (rc)
    goto unalloc_and_exit;
    sndev.peer_shared = pci_iomap(sndev.stdev.pdev, self_bar, LUT_SIZE);
    if (!sndev.peer_shared) {
    rc = -ENOMEM;
    goto unalloc_and_exit;
    }
    dev_dbg(&sndev.stdev.dev, "Shared MW Ready\n");
    return 0;
    unalloc_and_exit:
    dma_free_coherent(&sndev.stdev.pdev.dev, LUT_SIZE,
    sndev.self_shared, sndev.self_shared_dma);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn switchtec_ntb_deinit_shared_mw(sndev: *mut switchtec_ntb) {
    static void switchtec_ntb_deinit_shared_mw(struct switchtec_ntb *sndev)
    {
    if (sndev.peer_shared)
    pci_iounmap(sndev.stdev.pdev, sndev.peer_shared);
    if (sndev.self_shared)
    dma_free_coherent(&sndev.stdev.pdev.dev, LUT_SIZE,
    sndev.self_shared,
    sndev.self_shared_dma);
    sndev.nr_rsvd_luts--;
    }
#[no_mangle]
unsafe extern "C" fn switchtec_ntb_doorbell_isr(irq: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t switchtec_ntb_doorbell_isr(int irq, void *dev)
    {
    struct switchtec_ntb *sndev = dev;
    dev_dbg(&sndev.stdev.dev, "doorbell\n");
    ntb_db_event(&sndev.ntb, 0);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn switchtec_ntb_message_isr(irq: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t switchtec_ntb_message_isr(int irq, void *dev)
    {
    int i;
    struct switchtec_ntb *sndev = dev;
    for (i = 0; i < ARRAY_SIZE(sndev.mmio_self_dbmsg.imsg); i++) {
    let mut msg: u64 = ioread64(&sndev.mmio_self_dbmsg.imsg[i]);
    if (msg & NTB_DBMSG_IMSG_STATUS) {
    dev_dbg(&sndev.stdev.dev, "message: %d %08x\n",
    i, (u32)msg);
    iowrite8(1, &sndev.mmio_self_dbmsg.imsg[i].status);
    if (i == LINK_MESSAGE)
    switchtec_ntb_check_link(sndev, msg);
    }
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn switchtec_ntb_init_db_msg_irq(sndev: *mut switchtec_ntb) -> c_int {
    static int switchtec_ntb_init_db_msg_irq(struct switchtec_ntb *sndev)
    {
    int i;
    int rc;
    let mut doorbell_irq: c_int = 0;
    let mut message_irq: c_int = 0;
    int event_irq;
    let mut idb_vecs: c_int = sizeof(sndev.mmio_self_dbmsg.idb_vec_map);
    event_irq = ioread32(&sndev.stdev.mmio_part_cfg.vep_vector_number);
    while (doorbell_irq == event_irq)
    doorbell_irq++;
    while (message_irq == doorbell_irq ||
    message_irq == event_irq)
    message_irq++;
    dev_dbg(&sndev.stdev.dev, "irqs - event: %d, db: %d, msgs: %d\n",
    event_irq, doorbell_irq, message_irq);
    for (i = 0; i < idb_vecs - 4; i++)
    iowrite8(doorbell_irq,
    &sndev.mmio_self_dbmsg.idb_vec_map[i]);
    for (; i < idb_vecs; i++)
    iowrite8(message_irq,
    &sndev.mmio_self_dbmsg.idb_vec_map[i]);
    sndev.doorbell_irq = pci_irq_vector(sndev.stdev.pdev, doorbell_irq);
    sndev.message_irq = pci_irq_vector(sndev.stdev.pdev, message_irq);
    rc = request_irq(sndev.doorbell_irq,
    switchtec_ntb_doorbell_isr, 0,
    "switchtec_ntb_doorbell", sndev);
    if (rc)
    return rc;
    rc = request_irq(sndev.message_irq,
    switchtec_ntb_message_isr, 0,
    "switchtec_ntb_message", sndev);
    if (rc) {
    free_irq(sndev.doorbell_irq, sndev);
    return rc;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn switchtec_ntb_deinit_db_msg_irq(sndev: *mut switchtec_ntb) {
    static void switchtec_ntb_deinit_db_msg_irq(struct switchtec_ntb *sndev)
    {
    free_irq(sndev.doorbell_irq, sndev);
    free_irq(sndev.message_irq, sndev);
    }
#[no_mangle]
unsafe extern "C" fn switchtec_ntb_reinit_peer(sndev: *mut switchtec_ntb) -> c_int {
    static int switchtec_ntb_reinit_peer(struct switchtec_ntb *sndev)
    {
    int rc;
    if (crosslink_is_enabled(sndev))
    return 0;
    dev_info(&sndev.stdev.dev, "reinitialize shared memory window\n");
    rc = config_rsvd_lut_win(sndev, sndev.mmio_peer_ctrl, 0,
    sndev.self_partition,
    sndev.self_shared_dma);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn switchtec_ntb_add(dev: *mut device) -> c_int {
    static int switchtec_ntb_add(struct device *dev)
    {
    struct switchtec_dev *stdev = to_stdev(dev);
    struct switchtec_ntb *sndev;
    int rc;
    stdev.sndev = core::ptr::null_mut();
    if (stdev.pdev.class != (PCI_CLASS_BRIDGE_OTHER << 8))
    return -ENODEV;
    sndev = kzalloc_node(sizeof(*sndev), GFP_KERNEL, dev_to_node(dev));
    if (!sndev)
    return -ENOMEM;
    sndev.stdev = stdev;
    rc = switchtec_ntb_init_sndev(sndev);
    if (rc)
    goto free_and_exit;
    switchtec_ntb_init_mw(sndev);
    rc = switchtec_ntb_init_req_id_table(sndev);
    if (rc)
    goto free_and_exit;
    rc = switchtec_ntb_init_crosslink(sndev);
    if (rc)
    goto free_and_exit;
    switchtec_ntb_init_db(sndev);
    switchtec_ntb_init_msgs(sndev);
    rc = switchtec_ntb_init_shared_mw(sndev);
    if (rc)
    goto deinit_crosslink;
    rc = switchtec_ntb_init_db_msg_irq(sndev);
    if (rc)
    goto deinit_shared_and_exit;
//
// If this host crashed, the other host may think the link is
// still up. Tell them to force it down (it will go back up
// once we register the ntb device).
//
    switchtec_ntb_send_msg(sndev, LINK_MESSAGE, MSG_LINK_FORCE_DOWN);
    rc = ntb_register_device(&sndev.ntb);
    if (rc)
    goto deinit_and_exit;
    stdev.sndev = sndev;
    stdev.link_notifier = switchtec_ntb_link_notification;
    dev_info(dev, "NTB device registered\n");
    return 0;
    deinit_and_exit:
    switchtec_ntb_deinit_db_msg_irq(sndev);
    deinit_shared_and_exit:
    switchtec_ntb_deinit_shared_mw(sndev);
    deinit_crosslink:
    switchtec_ntb_deinit_crosslink(sndev);
    free_and_exit:
    kfree(sndev);
    dev_err(dev, "failed to register ntb device: %d\n", rc);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn switchtec_ntb_remove(dev: *mut device) {
    static void switchtec_ntb_remove(struct device *dev)
    {
    struct switchtec_dev *stdev = to_stdev(dev);
    struct switchtec_ntb *sndev = stdev.sndev;
    if (!sndev)
    return;
    stdev.link_notifier = core::ptr::null_mut();
    stdev.sndev = core::ptr::null_mut();
    ntb_unregister_device(&sndev.ntb);
    switchtec_ntb_deinit_db_msg_irq(sndev);
    switchtec_ntb_deinit_shared_mw(sndev);
    switchtec_ntb_deinit_crosslink(sndev);
    cancel_work_sync(&sndev.check_link_status_work);
    kfree(sndev);
    dev_info(dev, "ntb device unregistered\n");
    }
    static struct class_interface switchtec_interface  = {
    .add_dev = switchtec_ntb_add,
    .remove_dev = switchtec_ntb_remove,
    };
#[no_mangle]
unsafe extern "C" fn switchtec_ntb_init() -> int __init {
    static int __init switchtec_ntb_init(void)
    {
    switchtec_interface.class = &switchtec_class;
    return class_interface_register(&switchtec_interface);
    }
    module_init(switchtec_ntb_init);
#[no_mangle]
unsafe extern "C" fn switchtec_ntb_exit() -> void __exit {
    static void __exit switchtec_ntb_exit(void)
    {
    class_interface_unregister(&switchtec_interface);
    }
    module_exit(switchtec_ntb_exit);
