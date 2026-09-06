//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/marvell/prestera/prestera_pci.c
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


// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Copyright (c) 2019-2020 Marvell International Ltd. All rights reserved

pub const PRESTERA_MSG_MAX_SIZE: c_int = 1500;
pub const PRESTERA_SUPP_FW_MAJ_VER: c_int = 4;
pub const PRESTERA_SUPP_FW_MIN_VER: c_int = 1;
pub const PRESTERA_PREV_FW_MAJ_VER: c_int = 4;
pub const PRESTERA_PREV_FW_MIN_VER: c_int = 0;

pub const PRESTERA_FW_HDR_MAGIC: c_uint = 0x351D9D06;
pub const PRESTERA_FW_DL_TIMEOUT_MS: c_int = 50000;
pub const PRESTERA_FW_BLK_SZ: c_int = 1024;
pub const PRESTERA_FW_VER_MAJ_MUL: c_int = 1000000;
pub const PRESTERA_FW_VER_MIN_MUL: c_int = 1000;

    (((v) - (PRESTERA_FW_VER_MAJ(v) * PRESTERA_FW_VER_MAJ_MUL)) / \
    PRESTERA_FW_VER_MIN_MUL)

    ((v) - (PRESTERA_FW_VER_MAJ(v) * PRESTERA_FW_VER_MAJ_MUL) - \
    (PRESTERA_FW_VER_MIN(v) * PRESTERA_FW_VER_MIN_MUL))
    enum prestera_pci_bar_t {
    PRESTERA_PCI_BAR_FW = 2,
    PRESTERA_PCI_BAR_PP = 4,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_fw_header {
    pub magic_number: __be32,
    pub version_value: __be32,
    pub reserved: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_ldr_regs {
    pub ldr_ready: u32,
    pub pad1: u32,
    pub ldr_img_size: u32,
    pub ldr_ctl_flags: u32,
    pub ldr_buf_offs: u32,
    pub ldr_buf_size: u32,
    pub ldr_buf_rd: u32,
    pub pad2: u32,
    pub ldr_buf_wr: u32,
    pub ldr_status: u32,
}

pub const PRESTERA_LDR_READY_MAGIC: c_uint = 0xf00dfeed;

// fw loader registers

pub const PRESTERA_EVT_QNUM_MAX: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_fw_evtq_regs {
    pub rd_idx: u32,
    pub pad1: u32,
    pub wr_idx: u32,
    pub pad2: u32,
    pub offs: u32,
    pub len: u32,
}

pub const PRESTERA_CMD_QNUM_MAX: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_fw_cmdq_regs {
    pub req_ctl: u32,
    pub req_len: u32,
    pub rcv_ctl: u32,
    pub rcv_len: u32,
    pub offs: u32,
    pub len: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_fw_regs {
    pub fw_ready: u32,
    pub cmd_offs: u32,
    pub cmd_len: u32,
    pub cmd_qnum: u32,
    pub evt_offs: u32,
    pub evt_qnum: u32,
    pub fw_status: u32,
    pub rx_status: u32,
    pub cmdq_list: [prestera_fw_cmdq_regs; PRESTERA_EVT_QNUM_MAX],
    pub evtq_list: [prestera_fw_evtq_regs; PRESTERA_CMD_QNUM_MAX],
}

pub const PRESTERA_FW_READY_MAGIC: c_uint = 0xcafebabe;
// fw registers

    (PRESTERA_FW_REG_OFFSET(cmdq_list) +		\
    (q) * sizeof(struct prestera_fw_cmdq_regs) +	\
    offsetof(struct prestera_fw_cmdq_regs, f))

// PRESTERA_CMD_REQ_CTL_REG flags

// PRESTERA_CMD_RCV_CTL_REG flags

pub const PRESTERA_FW_EVT_CTL_STATUS_ON: c_int = 0;
pub const PRESTERA_FW_EVT_CTL_STATUS_OFF: c_int = 1;

    (PRESTERA_FW_REG_OFFSET(evtq_list) +		\
    (q) * sizeof(struct prestera_fw_evtq_regs) +	\
    offsetof(struct prestera_fw_evtq_regs, f))

pub const PRESTERA_FW_CMD_DEFAULT_WAIT_MS: c_int = 30000;
pub const PRESTERA_FW_READY_WAIT_MS: c_int = 20000;
pub const PRESTERA_DEV_ID_AC3X_98DX_55: c_uint = 0xC804;
pub const PRESTERA_DEV_ID_AC3X_98DX_65: c_uint = 0xC80C;
pub const PRESTERA_DEV_ID_ALDRIN2: c_uint = 0xCC1E;
pub const PRESTERA_DEV_ID_98DX7312M: c_uint = 0x981F;
pub const PRESTERA_DEV_ID_98DX3500: c_uint = 0x9820;
pub const PRESTERA_DEV_ID_98DX3501: c_uint = 0x9826;
pub const PRESTERA_DEV_ID_98DX3510: c_uint = 0x9821;
pub const PRESTERA_DEV_ID_98DX3520: c_uint = 0x9822;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_fw_evtq {
    pub addr: *mut u8 __iomem,
    pub len: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_fw_cmdq {
// serialize access to dev->send_req
    pub cmd_mtx: mutex,
    pub addr: *mut u8 __iomem,
    pub len: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prestera_fw {
    pub rev_supp: prestera_fw_rev,
    pub bin: *const firmware,
    pub wq: *mut workqueue_struct,
    pub dev: prestera_device,
    pub pci_dev: *mut pci_dev,
    pub ldr_regs: *mut u8 __iomem,
    pub ldr_ring_buf: *mut u8 __iomem,
    pub ldr_buf_len: u32,
    pub ldr_wr_idx: u32,
    pub cmd_mbox_len: usize,
    pub cmd_mbox: *mut u8 __iomem,
    pub cmd_queue: [prestera_fw_cmdq; PRESTERA_CMD_QNUM_MAX],
    pub cmd_qnum: u8,
    pub evt_queue: [prestera_fw_evtq; PRESTERA_EVT_QNUM_MAX],
    pub evt_qnum: u8,
    pub evt_work: work_struct,
    pub evt_buf: *mut u8 __iomem,
    pub evt_msg: *mut u8,
}

    static int prestera_fw_load(struct prestera_fw *fw);
#[no_mangle]
unsafe extern "C" fn prestera_fw_write(fw: *mut prestera_fw, reg: u32, val: u32) {
    static void prestera_fw_write(struct prestera_fw *fw, u32 reg, u32 val)
    {
    writel(val, PRESTERA_FW_REG_ADDR(fw, reg));
    }
#[no_mangle]
unsafe extern "C" fn prestera_fw_read(fw: *mut prestera_fw, reg: u32) -> u32 {
    static u32 prestera_fw_read(struct prestera_fw *fw, u32 reg)
    {
    return readl(PRESTERA_FW_REG_ADDR(fw, reg));
    }
#[no_mangle]
unsafe extern "C" fn prestera_fw_evtq_len(fw: *mut prestera_fw, qid: u8) -> u32 {
    static u32 prestera_fw_evtq_len(struct prestera_fw *fw, u8 qid)
    {
    return fw.evt_queue[qid].len;
    }
#[no_mangle]
unsafe extern "C" fn prestera_fw_evtq_avail(fw: *mut prestera_fw, qid: u8) -> u32 {
    static u32 prestera_fw_evtq_avail(struct prestera_fw *fw, u8 qid)
    {
    let mut wr_idx: u32 = prestera_fw_read(fw, PRESTERA_EVTQ_WR_IDX_REG(qid));
    let mut rd_idx: u32 = prestera_fw_read(fw, PRESTERA_EVTQ_RD_IDX_REG(qid));
    return CIRC_CNT(wr_idx, rd_idx, prestera_fw_evtq_len(fw, qid));
    }
    static void prestera_fw_evtq_rd_set(struct prestera_fw *fw,
    u8 qid, u32 idx)
    {
    let mut rd_idx: u32 = idx & (prestera_fw_evtq_len(fw, qid) - 1);
    prestera_fw_write(fw, PRESTERA_EVTQ_RD_IDX_REG(qid), rd_idx);
    }
    static u8 __iomem *prestera_fw_evtq_buf(struct prestera_fw *fw, u8 qid)
    {
    return fw.evt_queue[qid].addr;
    }
#[no_mangle]
unsafe extern "C" fn prestera_fw_evtq_read32(fw: *mut prestera_fw, qid: u8) -> u32 {
    static u32 prestera_fw_evtq_read32(struct prestera_fw *fw, u8 qid)
    {
    let mut rd_idx: u32 = prestera_fw_read(fw, PRESTERA_EVTQ_RD_IDX_REG(qid));
    u32 val;
    val = readl(prestera_fw_evtq_buf(fw, qid) + rd_idx);
    prestera_fw_evtq_rd_set(fw, qid, rd_idx + 4);
    return val;
    }
    static ssize_t prestera_fw_evtq_read_buf(struct prestera_fw *fw,
    u8 qid, void *buf, size_t len)
    {
    let mut idx: u32 = prestera_fw_read(fw, PRESTERA_EVTQ_RD_IDX_REG(qid));
    u8 __iomem *evtq_addr = prestera_fw_evtq_buf(fw, qid);
    u32 *buf32 = buf;
    int i;
    for (i = 0; i < len / 4; buf32++, i++) {
// buf32 = readl_relaxed(evtq_addr + idx);
    idx = (idx + 4) & (prestera_fw_evtq_len(fw, qid) - 1);
    }
    prestera_fw_evtq_rd_set(fw, qid, idx);
    return i;
    }
#[no_mangle]
unsafe extern "C" fn prestera_fw_evtq_pick(fw: *mut prestera_fw) -> u8 {
    static u8 prestera_fw_evtq_pick(struct prestera_fw *fw)
    {
    int qid;
    for (qid = 0; qid < fw.evt_qnum; qid++) {
    if (prestera_fw_evtq_avail(fw, qid) >= 4)
    return qid;
    }
    return PRESTERA_EVT_QNUM_MAX;
    }
#[no_mangle]
unsafe extern "C" fn prestera_fw_evt_ctl_status_set(fw: *mut prestera_fw, val: u32) {
    static void prestera_fw_evt_ctl_status_set(struct prestera_fw *fw, u32 val)
    {
    let mut status: u32 = prestera_fw_read(fw, PRESTERA_FW_STATUS_REG);
    u32p_replace_bits(&status, val, PRESTERA_FW_EVT_CTL_STATUS_MASK);
    prestera_fw_write(fw, PRESTERA_FW_STATUS_REG, status);
    }
#[no_mangle]
unsafe extern "C" fn prestera_fw_evt_work_fn(work: *mut work_struct) {
    static void prestera_fw_evt_work_fn(struct work_struct *work)
    {
    struct prestera_fw *fw;
    void *msg;
    u8 qid;
    fw = container_of(work, struct prestera_fw, evt_work);
    msg = fw.evt_msg;
    prestera_fw_evt_ctl_status_set(fw, PRESTERA_FW_EVT_CTL_STATUS_OFF);
    while ((qid = prestera_fw_evtq_pick(fw)) < PRESTERA_EVT_QNUM_MAX) {
    u32 idx;
    u32 len;
    len = prestera_fw_evtq_read32(fw, qid);
    idx = prestera_fw_read(fw, PRESTERA_EVTQ_RD_IDX_REG(qid));
    WARN_ON(prestera_fw_evtq_avail(fw, qid) < len);
    if (WARN_ON(len > PRESTERA_MSG_MAX_SIZE)) {
    prestera_fw_evtq_rd_set(fw, qid, idx + len);
    continue;
    }
    prestera_fw_evtq_read_buf(fw, qid, msg, len);
    if (fw.dev.recv_msg)
    fw.dev.recv_msg(&fw.dev, msg, len);
    }
    prestera_fw_evt_ctl_status_set(fw, PRESTERA_FW_EVT_CTL_STATUS_ON);
    }
    static int prestera_fw_wait_reg32(struct prestera_fw *fw, u32 reg, u32 cmp,
    unsigned int waitms)
    {
    u8 __iomem *addr = PRESTERA_FW_REG_ADDR(fw, reg);
    u32 val;
    return readl_poll_timeout(addr, val, cmp == val,
    1 * USEC_PER_MSEC, waitms * USEC_PER_MSEC);
    }
#[no_mangle]
unsafe extern "C" fn prestera_fw_cmdq_lock(fw: *mut prestera_fw, qid: u8) {
    static void prestera_fw_cmdq_lock(struct prestera_fw *fw, u8 qid)
    {
    mutex_lock(&fw.cmd_queue[qid].cmd_mtx);
    }
#[no_mangle]
unsafe extern "C" fn prestera_fw_cmdq_unlock(fw: *mut prestera_fw, qid: u8) {
    static void prestera_fw_cmdq_unlock(struct prestera_fw *fw, u8 qid)
    {
    mutex_unlock(&fw.cmd_queue[qid].cmd_mtx);
    }
#[no_mangle]
unsafe extern "C" fn prestera_fw_cmdq_len(fw: *mut prestera_fw, qid: u8) -> u32 {
    static u32 prestera_fw_cmdq_len(struct prestera_fw *fw, u8 qid)
    {
    return fw.cmd_queue[qid].len;
    }
    static u8 __iomem *prestera_fw_cmdq_buf(struct prestera_fw *fw, u8 qid)
    {
    return fw.cmd_queue[qid].addr;
    }
    static int prestera_fw_cmd_send(struct prestera_fw *fw, int qid,
    void *in_msg, size_t in_size,
    void *out_msg, size_t out_size,
    unsigned int waitms)
    {
    u32 ret_size;
    int err;
    if (!waitms)
    waitms = PRESTERA_FW_CMD_DEFAULT_WAIT_MS;
    if (ALIGN(in_size, 4) > prestera_fw_cmdq_len(fw, qid))
    return -EMSGSIZE;
// wait for finish previous reply from FW
    err = prestera_fw_wait_reg32(fw, PRESTERA_CMDQ_RCV_CTL_REG(qid), 0, 30);
    if (err) {
    dev_err(fw.dev.dev, "finish reply from FW is timed out\n");
    return err;
    }
    prestera_fw_write(fw, PRESTERA_CMDQ_REQ_LEN_REG(qid), in_size);
    memcpy_toio(prestera_fw_cmdq_buf(fw, qid), in_msg, in_size);
    prestera_fw_write(fw, PRESTERA_CMDQ_REQ_CTL_REG(qid),
    PRESTERA_CMD_F_REQ_SENT);
// wait for reply from FW
    err = prestera_fw_wait_reg32(fw, PRESTERA_CMDQ_RCV_CTL_REG(qid),
    PRESTERA_CMD_F_REPL_SENT, waitms);
    if (err) {
    dev_err(fw.dev.dev, "reply from FW is timed out\n");
    goto cmd_exit;
    }
    ret_size = prestera_fw_read(fw, PRESTERA_CMDQ_RCV_LEN_REG(qid));
    if (ret_size > out_size) {
    dev_err(fw.dev.dev, "ret_size (%u) > out_len(%zu)\n",
    ret_size, out_size);
    err = -EMSGSIZE;
    goto cmd_exit;
    }
    memcpy_fromio(out_msg,
    prestera_fw_cmdq_buf(fw, qid) + in_size, ret_size);
    cmd_exit:
    prestera_fw_write(fw, PRESTERA_CMDQ_REQ_CTL_REG(qid),
    PRESTERA_CMD_F_REPL_RCVD);
    return err;
    }
    static int prestera_fw_send_req(struct prestera_device *dev, int qid,
    void *in_msg, size_t in_size, void *out_msg,
    size_t out_size, unsigned int waitms)
    {
    struct prestera_fw *fw;
    ssize_t ret;
    fw = container_of(dev, struct prestera_fw, dev);
    prestera_fw_cmdq_lock(fw, qid);
    ret = prestera_fw_cmd_send(fw, qid, in_msg, in_size, out_msg, out_size,
    waitms);
    prestera_fw_cmdq_unlock(fw, qid);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn prestera_fw_init(fw: *mut prestera_fw) -> c_int {
    static int prestera_fw_init(struct prestera_fw *fw)
    {
    u8 __iomem *base;
    int err;
    u8 qid;
    fw.dev.send_req = prestera_fw_send_req;
    fw.ldr_regs = fw.dev.ctl_regs;
    err = prestera_fw_load(fw);
    if (err)
    return err;
    err = prestera_fw_wait_reg32(fw, PRESTERA_FW_READY_REG,
    PRESTERA_FW_READY_MAGIC,
    PRESTERA_FW_READY_WAIT_MS);
    if (err) {
    dev_err(fw.dev.dev, "FW failed to start\n");
    return err;
    }
    base = fw.dev.ctl_regs;
    fw.cmd_mbox = base + prestera_fw_read(fw, PRESTERA_CMD_BUF_OFFS_REG);
    fw.cmd_mbox_len = prestera_fw_read(fw, PRESTERA_CMD_BUF_LEN_REG);
    fw.cmd_qnum = prestera_fw_read(fw, PRESTERA_CMD_QNUM_REG);
    for (qid = 0; qid < fw.cmd_qnum; qid++) {
    let mut offs: u32 = prestera_fw_read(fw, PRESTERA_CMDQ_OFFS_REG(qid));
    struct prestera_fw_cmdq *cmdq = &fw.cmd_queue[qid];
    cmdq.len = prestera_fw_read(fw, PRESTERA_CMDQ_LEN_REG(qid));
    cmdq.addr = fw.cmd_mbox + offs;
    mutex_init(&cmdq.cmd_mtx);
    }
    fw.evt_buf = base + prestera_fw_read(fw, PRESTERA_EVT_BUF_OFFS_REG);
    fw.evt_qnum = prestera_fw_read(fw, PRESTERA_EVT_QNUM_REG);
    fw.evt_msg = kmalloc(PRESTERA_MSG_MAX_SIZE, GFP_KERNEL);
    if (!fw.evt_msg)
    return -ENOMEM;
    for (qid = 0; qid < fw.evt_qnum; qid++) {
    let mut offs: u32 = prestera_fw_read(fw, PRESTERA_EVTQ_OFFS_REG(qid));
    struct prestera_fw_evtq *evtq = &fw.evt_queue[qid];
    evtq.len = prestera_fw_read(fw, PRESTERA_EVTQ_LEN_REG(qid));
    evtq.addr = fw.evt_buf + offs;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn prestera_fw_uninit(fw: *mut prestera_fw) {
    static void prestera_fw_uninit(struct prestera_fw *fw)
    {
    kfree(fw.evt_msg);
    }
#[no_mangle]
unsafe extern "C" fn prestera_pci_irq_handler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t prestera_pci_irq_handler(int irq, void *dev_id)
    {
    struct prestera_fw *fw = dev_id;
    if (prestera_fw_read(fw, PRESTERA_RX_STATUS_REG)) {
    prestera_fw_write(fw, PRESTERA_RX_STATUS_REG, 0);
    if (fw.dev.recv_pkt)
    fw.dev.recv_pkt(&fw.dev);
    }
    queue_work(fw.wq, &fw.evt_work);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn prestera_ldr_write(fw: *mut prestera_fw, reg: u32, val: u32) {
    static void prestera_ldr_write(struct prestera_fw *fw, u32 reg, u32 val)
    {
    writel(val, PRESTERA_LDR_REG_ADDR(fw, reg));
    }
#[no_mangle]
unsafe extern "C" fn prestera_ldr_read(fw: *mut prestera_fw, reg: u32) -> u32 {
    static u32 prestera_ldr_read(struct prestera_fw *fw, u32 reg)
    {
    return readl(PRESTERA_LDR_REG_ADDR(fw, reg));
    }
    static int prestera_ldr_wait_reg32(struct prestera_fw *fw,
    u32 reg, u32 cmp, unsigned int waitms)
    {
    u8 __iomem *addr = PRESTERA_LDR_REG_ADDR(fw, reg);
    u32 val;
    return readl_poll_timeout(addr, val, cmp == val,
    10 * USEC_PER_MSEC, waitms * USEC_PER_MSEC);
    }
#[no_mangle]
unsafe extern "C" fn prestera_ldr_wait_buf(fw: *mut prestera_fw, len: usize) -> c_int {
    static int prestera_ldr_wait_buf(struct prestera_fw *fw, size_t len)
    {
    u8 __iomem *addr = PRESTERA_LDR_REG_ADDR(fw, PRESTERA_LDR_BUF_RD_REG);
    let mut buf_len: u32 = fw.ldr_buf_len;
    let mut wr_idx: u32 = fw.ldr_wr_idx;
    u32 rd_idx;
    return readl_poll_timeout(addr, rd_idx,
    CIRC_SPACE(wr_idx, rd_idx, buf_len) >= len,
    1 * USEC_PER_MSEC, 100 * USEC_PER_MSEC);
    }
#[no_mangle]
unsafe extern "C" fn prestera_ldr_wait_dl_finish(fw: *mut prestera_fw) -> c_int {
    static int prestera_ldr_wait_dl_finish(struct prestera_fw *fw)
    {
    u8 __iomem *addr = PRESTERA_LDR_REG_ADDR(fw, PRESTERA_LDR_STATUS_REG);
    let mut mask: c_ulong = ~(PRESTERA_LDR_STATUS_IMG_DL);
    u32 val;
    int err;
    err = readl_poll_timeout(addr, val, val & mask, 10 * USEC_PER_MSEC,
    PRESTERA_FW_DL_TIMEOUT_MS * USEC_PER_MSEC);
    if (err) {
    dev_err(fw.dev.dev, "Timeout to load FW img [state=%d]",
    prestera_ldr_read(fw, PRESTERA_LDR_STATUS_REG));
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn prestera_ldr_wr_idx_move(fw: *mut prestera_fw, n: c_uint) {
    static void prestera_ldr_wr_idx_move(struct prestera_fw *fw, unsigned int n)
    {
    fw.ldr_wr_idx = (fw.ldr_wr_idx + (n)) & (fw.ldr_buf_len - 1);
    }
#[no_mangle]
unsafe extern "C" fn prestera_ldr_wr_idx_commit(fw: *mut prestera_fw) {
    static void prestera_ldr_wr_idx_commit(struct prestera_fw *fw)
    {
    prestera_ldr_write(fw, PRESTERA_LDR_BUF_WR_REG, fw.ldr_wr_idx);
    }
    static u8 __iomem *prestera_ldr_wr_ptr(struct prestera_fw *fw)
    {
    return fw.ldr_ring_buf + fw.ldr_wr_idx;
    }
#[no_mangle]
unsafe extern "C" fn prestera_ldr_send(fw: *mut prestera_fw, buf: *const u8, len: usize) -> c_int {
    static int prestera_ldr_send(struct prestera_fw *fw, const u8 *buf, size_t len)
    {
    int err;
    int i;
    err = prestera_ldr_wait_buf(fw, len);
    if (err) {
    dev_err(fw.dev.dev, "failed wait for sending firmware\n");
    return err;
    }
    for (i = 0; i < len; i += 4) {
    writel_relaxed(*(u32 *)(buf + i), prestera_ldr_wr_ptr(fw));
    prestera_ldr_wr_idx_move(fw, 4);
    }
    prestera_ldr_wr_idx_commit(fw);
    return 0;
    }
    static int prestera_ldr_fw_send(struct prestera_fw *fw,
    const char *img, u32 fw_size)
    {
    u32 status;
    u32 pos;
    int err;
    err = prestera_ldr_wait_reg32(fw, PRESTERA_LDR_STATUS_REG,
    PRESTERA_LDR_STATUS_IMG_DL,
    5 * MSEC_PER_SEC);
    if (err) {
    dev_err(fw.dev.dev, "Loader is not ready to load image\n");
    return err;
    }
    for (pos = 0; pos < fw_size; pos += PRESTERA_FW_BLK_SZ) {
    if (pos + PRESTERA_FW_BLK_SZ > fw_size)
    break;
    err = prestera_ldr_send(fw, img + pos, PRESTERA_FW_BLK_SZ);
    if (err)
    return err;
    }
    if (pos < fw_size) {
    err = prestera_ldr_send(fw, img + pos, fw_size - pos);
    if (err)
    return err;
    }
    err = prestera_ldr_wait_dl_finish(fw);
    if (err)
    return err;
    status = prestera_ldr_read(fw, PRESTERA_LDR_STATUS_REG);
    switch (status) {
    case PRESTERA_LDR_STATUS_INVALID_IMG:
    dev_err(fw.dev.dev, "FW img has bad CRC\n");
    return -EINVAL;
    case PRESTERA_LDR_STATUS_NOMEM:
    dev_err(fw.dev.dev, "Loader has no enough mem\n");
    return -ENOMEM;
    }
    return 0;
    }
    static void prestera_fw_rev_parse(const struct prestera_fw_header *hdr,
    struct prestera_fw_rev *rev)
    {
    let mut version: u32 = be32_to_cpu(hdr.version_value);
    rev.maj = PRESTERA_FW_VER_MAJ(version);
    rev.min = PRESTERA_FW_VER_MIN(version);
    rev.sub = PRESTERA_FW_VER_PATCH(version);
    }
#[no_mangle]
unsafe extern "C" fn prestera_fw_rev_check(fw: *mut prestera_fw) -> c_int {
    static int prestera_fw_rev_check(struct prestera_fw *fw)
    {
    struct prestera_fw_rev *rev = &fw.dev.fw_rev;
    if (rev.maj == fw.rev_supp.maj && rev.min >= fw.rev_supp.min)
    return 0;
    dev_err(fw.dev.dev, "Driver supports FW version only '%u.%u.x'",
    fw.rev_supp.maj, fw.rev_supp.min);
    return -EINVAL;
    }
#[no_mangle]
unsafe extern "C" fn prestera_fw_hdr_parse(fw: *mut prestera_fw) -> c_int {
    static int prestera_fw_hdr_parse(struct prestera_fw *fw)
    {
    struct prestera_fw_rev *rev = &fw.dev.fw_rev;
    struct prestera_fw_header *hdr;
    u32 magic;
    if (fw.bin.size < sizeof(*hdr))
    return -EINVAL;
    hdr = (struct prestera_fw_header *)fw.bin.data;
    magic = be32_to_cpu(hdr.magic_number);
    if (magic != PRESTERA_FW_HDR_MAGIC) {
    dev_err(fw.dev.dev, "FW img hdr magic is invalid");
    return -EINVAL;
    }
    prestera_fw_rev_parse(hdr, rev);
    dev_info(fw.dev.dev, "FW version '%u.%u.%u'\n",
    rev.maj, rev.min, rev.sub);
    return prestera_fw_rev_check(fw);
    }
    static const char *prestera_fw_path_fmt_get(struct prestera_fw *fw)
    {
    switch (fw.pci_dev.device) {
    case PRESTERA_DEV_ID_98DX3500:
    case PRESTERA_DEV_ID_98DX3501:
    case PRESTERA_DEV_ID_98DX3510:
    case PRESTERA_DEV_ID_98DX3520:
    return PRESTERA_FW_ARM64_PATH_FMT;
    default:
    return PRESTERA_FW_PATH_FMT;
    }
    }
#[no_mangle]
unsafe extern "C" fn prestera_fw_get(fw: *mut prestera_fw) -> c_int {
    static int prestera_fw_get(struct prestera_fw *fw)
    {
    let mut ver_maj: c_int = PRESTERA_SUPP_FW_MAJ_VER;
    let mut ver_min: c_int = PRESTERA_SUPP_FW_MIN_VER;
    char fw_path[128];
    int err;
    pick_fw_ver:
    snprintf(fw_path, sizeof(fw_path), prestera_fw_path_fmt_get(fw),
    ver_maj, ver_min);
    err = request_firmware_direct(&fw.bin, fw_path, fw.dev.dev);
    if (err) {
    if (ver_maj != PRESTERA_PREV_FW_MAJ_VER ||
    ver_min != PRESTERA_PREV_FW_MIN_VER) {
    ver_maj = PRESTERA_PREV_FW_MAJ_VER;
    ver_min = PRESTERA_PREV_FW_MIN_VER;
    dev_warn(fw.dev.dev,
    "missing latest %s firmware, fall-back to previous %u.%u version\n",
    fw_path, ver_maj, ver_min);
    goto pick_fw_ver;
    } else {
    dev_err(fw.dev.dev, "failed to request previous firmware: %s\n",
    fw_path);
    return err;
    }
    }
    dev_info(fw.dev.dev, "Loading %s ...", fw_path);
    fw.rev_supp.maj = ver_maj;
    fw.rev_supp.min = ver_min;
    fw.rev_supp.sub = 0;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn prestera_fw_put(fw: *mut prestera_fw) {
    static void prestera_fw_put(struct prestera_fw *fw)
    {
    release_firmware(fw.bin);
    }
#[no_mangle]
unsafe extern "C" fn prestera_fw_load(fw: *mut prestera_fw) -> c_int {
    static int prestera_fw_load(struct prestera_fw *fw)
    {
    let mut hlen: usize = sizeof(struct prestera_fw_header);
    int err;
    err = prestera_ldr_wait_reg32(fw, PRESTERA_LDR_READY_REG,
    PRESTERA_LDR_READY_MAGIC,
    5 * MSEC_PER_SEC);
    if (err) {
    dev_err(fw.dev.dev, "waiting for FW loader is timed out");
    return err;
    }
    fw.ldr_ring_buf = fw.ldr_regs +
    prestera_ldr_read(fw, PRESTERA_LDR_BUF_OFFS_REG);
    fw.ldr_buf_len =
    prestera_ldr_read(fw, PRESTERA_LDR_BUF_SIZE_REG);
    fw.ldr_wr_idx = 0;
    err = prestera_fw_get(fw);
    if (err)
    return err;
    err = prestera_fw_hdr_parse(fw);
    if (err) {
    dev_err(fw.dev.dev, "FW image header is invalid\n");
    goto out_release;
    }
    prestera_ldr_write(fw, PRESTERA_LDR_IMG_SIZE_REG, fw.bin.size - hlen);
    prestera_ldr_write(fw, PRESTERA_LDR_CTL_REG, PRESTERA_LDR_CTL_DL_START);
    err = prestera_ldr_fw_send(fw, fw.bin.data + hlen,
    fw.bin.size - hlen);
    out_release:
    prestera_fw_put(fw);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn prestera_pci_pp_use_bar2(pdev: *mut pci_dev) -> bool {
    static bool prestera_pci_pp_use_bar2(struct pci_dev *pdev)
    {
    switch (pdev.device) {
    case PRESTERA_DEV_ID_98DX7312M:
    case PRESTERA_DEV_ID_98DX3500:
    case PRESTERA_DEV_ID_98DX3501:
    case PRESTERA_DEV_ID_98DX3510:
    case PRESTERA_DEV_ID_98DX3520:
    return true;
    default:
    return false;
    }
    }
#[no_mangle]
unsafe extern "C" fn prestera_pci_pp_bar2_offs(pdev: *mut pci_dev) -> u32 {
    static u32 prestera_pci_pp_bar2_offs(struct pci_dev *pdev)
    {
    if (pci_resource_len(pdev, 2) == 0x1000000)
    return 0x0;
    else
    return (pci_resource_len(pdev, 2) / 2);
    }
#[no_mangle]
unsafe extern "C" fn prestera_pci_fw_bar2_offs(pdev: *mut pci_dev) -> u32 {
    static u32 prestera_pci_fw_bar2_offs(struct pci_dev *pdev)
    {
    if (pci_resource_len(pdev, 2) == 0x1000000)
    return 0x400000;
    else
    return 0x0;
    }
    static int prestera_pci_probe(struct pci_dev *pdev,
    const struct pci_device_id *id)
    {
    const char *driver_name = dev_driver_string(&pdev.dev);
    u8 __iomem *mem_addr, *pp_addr = core::ptr::null_mut();
    struct prestera_fw *fw;
    int err;
    err = pcim_enable_device(pdev);
    if (err) {
    dev_err(&pdev.dev, "pci_enable_device failed\n");
    goto err_pci_enable_device;
    }
    err = pcim_request_all_regions(pdev, driver_name);
    if (err) {
    dev_err(&pdev.dev, "pcim_request_all_regions failed\n");
    goto err_pci_request_regions;
    }
    err = dma_set_mask_and_coherent(&pdev.dev, DMA_BIT_MASK(30));
    if (err) {
    dev_err(&pdev.dev, "fail to set DMA mask\n");
    goto err_dma_mask;
    }
    mem_addr = pcim_iomap(pdev, 2, 0);
    if (!mem_addr) {
    dev_err(&pdev.dev, "pci mem ioremap failed\n");
    err = -EIO;
    goto err_mem_ioremap;
    }
// AC5X devices use second half of BAR2
    if (prestera_pci_pp_use_bar2(pdev)) {
    pp_addr = mem_addr + prestera_pci_pp_bar2_offs(pdev);
    mem_addr = mem_addr + prestera_pci_fw_bar2_offs(pdev);
    } else {
    pp_addr = pcim_iomap(pdev, 4, 0);
    if (!pp_addr) {
    dev_err(&pdev.dev, "pp regs ioremap failed\n");
    err = -EIO;
    goto err_pp_ioremap;
    }
    }
    pci_set_master(pdev);
    fw = devm_kzalloc(&pdev.dev, sizeof(*fw), GFP_KERNEL);
    if (!fw) {
    err = -ENOMEM;
    goto err_pci_dev_alloc;
    }
    fw.pci_dev = pdev;
    fw.dev.ctl_regs = mem_addr;
    fw.dev.pp_regs = pp_addr;
    fw.dev.dev = &pdev.dev;
    pci_set_drvdata(pdev, fw);
    err = prestera_fw_init(fw);
    if (err)
    goto err_prestera_fw_init;
    dev_info(fw.dev.dev, "Prestera FW is ready\n");
    fw.wq = alloc_workqueue("prestera_fw_wq", WQ_HIGHPRI | WQ_PERCPU, 1);
    if (!fw.wq) {
    err = -ENOMEM;
    goto err_wq_alloc;
    }
    INIT_WORK(&fw.evt_work, prestera_fw_evt_work_fn);
    err = pci_alloc_irq_vectors(pdev, 1, 1, PCI_IRQ_MSI);
    if (err < 0) {
    dev_err(&pdev.dev, "MSI IRQ init failed\n");
    goto err_irq_alloc;
    }
    err = request_irq(pci_irq_vector(pdev, 0), prestera_pci_irq_handler,
    0, driver_name, fw);
    if (err) {
    dev_err(&pdev.dev, "fail to request IRQ\n");
    goto err_request_irq;
    }
    err = prestera_device_register(&fw.dev);
    if (err)
    goto err_prestera_dev_register;
    return 0;
    err_prestera_dev_register:
    free_irq(pci_irq_vector(pdev, 0), fw);
    err_request_irq:
    pci_free_irq_vectors(pdev);
    err_irq_alloc:
    destroy_workqueue(fw.wq);
    err_wq_alloc:
    prestera_fw_uninit(fw);
    err_prestera_fw_init:
    err_pci_dev_alloc:
    err_pp_ioremap:
    err_mem_ioremap:
    err_dma_mask:
    err_pci_request_regions:
    err_pci_enable_device:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn prestera_pci_remove(pdev: *mut pci_dev) {
    static void prestera_pci_remove(struct pci_dev *pdev)
    {
    struct prestera_fw *fw = pci_get_drvdata(pdev);
    prestera_device_unregister(&fw.dev);
    free_irq(pci_irq_vector(pdev, 0), fw);
    pci_free_irq_vectors(pdev);
    destroy_workqueue(fw.wq);
    prestera_fw_uninit(fw);
    }
    static const struct pci_device_id prestera_pci_devices[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_MARVELL, PRESTERA_DEV_ID_AC3X_98DX_55) },
    { PCI_DEVICE(PCI_VENDOR_ID_MARVELL, PRESTERA_DEV_ID_AC3X_98DX_65) },
    { PCI_DEVICE(PCI_VENDOR_ID_MARVELL, PRESTERA_DEV_ID_ALDRIN2) },
    { PCI_DEVICE(PCI_VENDOR_ID_MARVELL, PRESTERA_DEV_ID_98DX7312M) },
    { PCI_DEVICE(PCI_VENDOR_ID_MARVELL, PRESTERA_DEV_ID_98DX3500) },
    { PCI_DEVICE(PCI_VENDOR_ID_MARVELL, PRESTERA_DEV_ID_98DX3501) },
    { PCI_DEVICE(PCI_VENDOR_ID_MARVELL, PRESTERA_DEV_ID_98DX3510) },
    { PCI_DEVICE(PCI_VENDOR_ID_MARVELL, PRESTERA_DEV_ID_98DX3520) },
    { }
    };
    MODULE_DEVICE_TABLE(pci, prestera_pci_devices);
    static struct pci_driver prestera_pci_driver = {
    .name     = "Prestera DX",
    .id_table = prestera_pci_devices,
    .probe    = prestera_pci_probe,
    .remove   = prestera_pci_remove,
    };
    module_pci_driver(prestera_pci_driver);
    MODULE_LICENSE("Dual BSD/GPL");
    MODULE_DESCRIPTION("Marvell Prestera switch PCI interface");
