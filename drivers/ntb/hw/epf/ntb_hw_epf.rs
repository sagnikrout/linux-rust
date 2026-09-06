//! Automatically rewritten from C to Rust
//! Source: drivers/ntb/hw/epf/ntb_hw_epf.c
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
// Host side endpoint driver to implement Non-Transparent Bridge functionality
//
// Copyright (C) 2020 Texas Instruments
// Author: Kishon Vijay Abraham I <kishon@ti.com>
//

pub const NTB_EPF_COMMAND: c_uint = 0x0;
pub const CMD_CONFIGURE_DOORBELL: c_int = 1;
pub const CMD_TEARDOWN_DOORBELL: c_int = 2;
pub const CMD_CONFIGURE_MW: c_int = 3;
pub const CMD_TEARDOWN_MW: c_int = 4;
pub const CMD_LINK_UP: c_int = 5;
pub const CMD_LINK_DOWN: c_int = 6;
pub const NTB_EPF_ARGUMENT: c_uint = 0x4;

pub const NTB_EPF_CMD_STATUS: c_uint = 0x8;
pub const COMMAND_STATUS_OK: c_int = 1;
pub const COMMAND_STATUS_ERROR: c_int = 2;
pub const NTB_EPF_LINK_STATUS: c_uint = 0x0A;

pub const NTB_EPF_TOPOLOGY: c_uint = 0x0C;
pub const NTB_EPF_LOWER_ADDR: c_uint = 0x10;
pub const NTB_EPF_UPPER_ADDR: c_uint = 0x14;
pub const NTB_EPF_LOWER_SIZE: c_uint = 0x18;
pub const NTB_EPF_UPPER_SIZE: c_uint = 0x1C;
pub const NTB_EPF_MW_COUNT: c_uint = 0x20;
pub const NTB_EPF_MW1_OFFSET: c_uint = 0x24;
pub const NTB_EPF_SPAD_OFFSET: c_uint = 0x28;
pub const NTB_EPF_SPAD_COUNT: c_uint = 0x2C;
pub const NTB_EPF_DB_ENTRY_SIZE: c_uint = 0x30;

//
// Legacy doorbell slot layout when paired with pci-epf-*ntb:
//
// slot 0 : reserved for link events
// slot 1 : unused (historical extra offset)
// slot 2 : DB#0
// slot 3 : DB#1
// ...
//
// Thus, NTB_EPF_MIN_DB_COUNT=3 means that we at least create vectors for
// doorbells DB#0 and DB#1.
//
pub const NTB_EPF_MIN_DB_COUNT: c_int = 3;
pub const NTB_EPF_MAX_DB_COUNT: c_int = 31;

    enum pci_barno {
    NO_BAR = -1,
    BAR_0,
    BAR_1,
    BAR_2,
    BAR_3,
    BAR_4,
    BAR_5,
    };
    enum epf_ntb_bar {
    BAR_CONFIG,
    BAR_PEER_SPAD,
    BAR_DB,
    BAR_MW1,
    BAR_MW2,
    BAR_MW3,
    BAR_MW4,
    NTB_BAR_NUM,
    };
    enum epf_irq_slot {
    EPF_IRQ_LINK = 0,
    EPF_IRQ_RESERVED_DB, /* Historically skipped slot */
    EPF_IRQ_DB_START,
    };

    struct ntb_epf_dev;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntb_epf_irq_ctx {
    pub ndev: *mut ntb_epf_dev,
    pub irq_no: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntb_epf_dev {
    pub ntb: ntb_dev,
    pub dev: *mut device,
// Mutex to protect providing commands to NTB EPF
    pub cmd_lock: mutex,
    pub barno_map: *const enum pci_barno,
    pub mw_count: c_uint,
    pub spad_count: c_uint,
    pub db_count: c_uint,
    pub ctrl_reg: *mut void __iomem,
    pub db_reg: *mut void __iomem,
    pub peer_spad_reg: *mut void __iomem,
    pub self_spad: c_uint,
    pub peer_spad: c_uint,
    pub db_val: core::sync::atomic::AtomicI64,
    pub db_valid_mask: u64,
    pub 1]: ntb_epf_irq_ctx irq_ctx[NTB_EPF_MAX_DB_COUNT +,
}

    static int ntb_epf_send_command(struct ntb_epf_dev *ndev, u32 command,
    u32 argument)
    {
    ktime_t timeout;
    bool timedout;
    let mut ret: c_int = 0;
    u32 status;
    mutex_lock(&ndev.cmd_lock);
    writel(argument, ndev.ctrl_reg + NTB_EPF_ARGUMENT);
    writel(command, ndev.ctrl_reg + NTB_EPF_COMMAND);
    timeout = ktime_add_ms(ktime_get(), NTB_EPF_COMMAND_TIMEOUT);
    while (1) {
    timedout = ktime_after(ktime_get(), timeout);
    status = readw(ndev.ctrl_reg + NTB_EPF_CMD_STATUS);
    if (status == COMMAND_STATUS_ERROR) {
    ret = -EINVAL;
    break;
    }
    if (status == COMMAND_STATUS_OK)
    break;
    if (WARN_ON(timedout)) {
    ret = -ETIMEDOUT;
    break;
    }
    usleep_range(5, 10);
    }
    writew(0, ndev.ctrl_reg + NTB_EPF_CMD_STATUS);
    mutex_unlock(&ndev.cmd_lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ntb_epf_mw_to_bar(ndev: *mut ntb_epf_dev, idx: c_int) -> c_int {
    static int ntb_epf_mw_to_bar(struct ntb_epf_dev *ndev, int idx)
    {
    struct device *dev = ndev.dev;
    if (idx < 0 || idx > ndev.mw_count) {
    dev_err(dev, "Unsupported Memory Window index %d\n", idx);
    return -EINVAL;
    }
    return ndev.barno_map[BAR_MW1 + idx];
    }
#[no_mangle]
unsafe extern "C" fn ntb_epf_mw_count(ntb: *mut ntb_dev, pidx: c_int) -> c_int {
    static int ntb_epf_mw_count(struct ntb_dev *ntb, int pidx)
    {
    struct ntb_epf_dev *ndev = ntb_ndev(ntb);
    struct device *dev = ndev.dev;
    if (pidx != NTB_DEF_PEER_IDX) {
    dev_err(dev, "Unsupported Peer ID %d\n", pidx);
    return -EINVAL;
    }
    return ndev.mw_count;
    }
    static int ntb_epf_mw_get_align(struct ntb_dev *ntb, int pidx, int idx,
    resource_size_t *addr_align,
    resource_size_t *size_align,
    resource_size_t *size_max)
    {
    struct ntb_epf_dev *ndev = ntb_ndev(ntb);
    struct device *dev = ndev.dev;
    int bar;
    if (pidx != NTB_DEF_PEER_IDX) {
    dev_err(dev, "Unsupported Peer ID %d\n", pidx);
    return -EINVAL;
    }
    bar = ntb_epf_mw_to_bar(ndev, idx);
    if (bar < 0)
    return bar;
    if (addr_align)
// addr_align = SZ_4K;
    if (size_align)
// size_align = 1;
    if (size_max)
// size_max = pci_resource_len(ndev->ntb.pdev, bar);
    return 0;
    }
    static u64 ntb_epf_link_is_up(struct ntb_dev *ntb,
    enum ntb_speed *speed,
    enum ntb_width *width)
    {
    struct ntb_epf_dev *ndev = ntb_ndev(ntb);
    u32 status;
    status = readw(ndev.ctrl_reg + NTB_EPF_LINK_STATUS);
    return status & LINK_STATUS_UP;
    }
#[no_mangle]
unsafe extern "C" fn ntb_epf_spad_read(ntb: *mut ntb_dev, idx: c_int) -> u32 {
    static u32 ntb_epf_spad_read(struct ntb_dev *ntb, int idx)
    {
    struct ntb_epf_dev *ndev = ntb_ndev(ntb);
    struct device *dev = ndev.dev;
    u32 offset;
    if (idx < 0 || idx >= ndev.spad_count) {
    dev_err(dev, "READ: Invalid ScratchPad Index %d\n", idx);
    return 0;
    }
    offset = readl(ndev.ctrl_reg + NTB_EPF_SPAD_OFFSET);
    offset += (idx << 2);
    return readl(ndev.ctrl_reg + offset);
    }
    static int ntb_epf_spad_write(struct ntb_dev *ntb,
    int idx, u32 val)
    {
    struct ntb_epf_dev *ndev = ntb_ndev(ntb);
    struct device *dev = ndev.dev;
    u32 offset;
    if (idx < 0 || idx >= ndev.spad_count) {
    dev_err(dev, "WRITE: Invalid ScratchPad Index %d\n", idx);
    return -EINVAL;
    }
    offset = readl(ndev.ctrl_reg + NTB_EPF_SPAD_OFFSET);
    offset += (idx << 2);
    writel(val, ndev.ctrl_reg + offset);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ntb_epf_peer_spad_read(ntb: *mut ntb_dev, pidx: c_int, idx: c_int) -> u32 {
    static u32 ntb_epf_peer_spad_read(struct ntb_dev *ntb, int pidx, int idx)
    {
    struct ntb_epf_dev *ndev = ntb_ndev(ntb);
    struct device *dev = ndev.dev;
    u32 offset;
    if (pidx != NTB_DEF_PEER_IDX) {
    dev_err(dev, "Unsupported Peer ID %d\n", pidx);
    return -EINVAL;
    }
    if (idx < 0 || idx >= ndev.spad_count) {
    dev_err(dev, "WRITE: Invalid Peer ScratchPad Index %d\n", idx);
    return -EINVAL;
    }
    offset = (idx << 2);
    return readl(ndev.peer_spad_reg + offset);
    }
    static int ntb_epf_peer_spad_write(struct ntb_dev *ntb, int pidx,
    int idx, u32 val)
    {
    struct ntb_epf_dev *ndev = ntb_ndev(ntb);
    struct device *dev = ndev.dev;
    u32 offset;
    if (pidx != NTB_DEF_PEER_IDX) {
    dev_err(dev, "Unsupported Peer ID %d\n", pidx);
    return -EINVAL;
    }
    if (idx < 0 || idx >= ndev.spad_count) {
    dev_err(dev, "WRITE: Invalid Peer ScratchPad Index %d\n", idx);
    return -EINVAL;
    }
    offset = (idx << 2);
    writel(val, ndev.peer_spad_reg + offset);
    return 0;
    }
    static int ntb_epf_link_enable(struct ntb_dev *ntb,
    enum ntb_speed max_speed,
    enum ntb_width max_width)
    {
    struct ntb_epf_dev *ndev = ntb_ndev(ntb);
    struct device *dev = ndev.dev;
    int ret;
    ret = ntb_epf_send_command(ndev, CMD_LINK_UP, 0);
    if (ret) {
    dev_err(dev, "Fail to enable link\n");
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ntb_epf_link_disable(ntb: *mut ntb_dev) -> c_int {
    static int ntb_epf_link_disable(struct ntb_dev *ntb)
    {
    struct ntb_epf_dev *ndev = ntb_ndev(ntb);
    struct device *dev = ndev.dev;
    int ret;
    ret = ntb_epf_send_command(ndev, CMD_LINK_DOWN, 0);
    if (ret) {
    dev_err(dev, "Fail to disable link\n");
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ntb_epf_vec_isr(irq: c_int, dev: *mut c_void) -> irqreturn_t {
    static irqreturn_t ntb_epf_vec_isr(int irq, void *dev)
    {
    struct ntb_epf_irq_ctx *ctx = dev;
    struct ntb_epf_dev *ndev = ctx.ndev;
    unsigned int db_vector;
    let mut irq_no: c_uint = ctx.irq_no;
    if (irq_no == EPF_IRQ_LINK) {
    ntb_link_event(&ndev.ntb);
    } else if (irq_no == EPF_IRQ_RESERVED_DB) {
    dev_warn_ratelimited(ndev.dev,
    "Unexpected reserved doorbell slot IRQ received\n");
    } else {
    db_vector = irq_no - EPF_IRQ_DB_START;
    if (ndev.db_count < NTB_EPF_MIN_DB_COUNT ||
    db_vector >= ndev.db_count - 1) {
    dev_warn_ratelimited(ndev.dev,
    "Unexpected doorbell vector %u (db_count %u)\n",
    db_vector, ndev.db_count);
    return IRQ_HANDLED;
    }
    atomic64_or(BIT_ULL(db_vector), &ndev.db_val);
    ntb_db_event(&ndev.ntb, db_vector);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn ntb_epf_init_isr(ndev: *mut ntb_epf_dev, msi_min: c_int, msi_max: c_int) -> c_int {
    static int ntb_epf_init_isr(struct ntb_epf_dev *ndev, int msi_min, int msi_max)
    {
    struct pci_dev *pdev = ndev.ntb.pdev;
    struct device *dev = ndev.dev;
    let mut argument: u32 = MSIX_ENABLE;
    int irq;
    int ret;
    int i;
    irq = pci_alloc_irq_vectors(pdev, msi_min, msi_max, PCI_IRQ_MSIX);
    if (irq < 0) {
    dev_dbg(dev, "Failed to get MSIX interrupts\n");
    irq = pci_alloc_irq_vectors(pdev, msi_min, msi_max,
    PCI_IRQ_MSI);
    if (irq < 0) {
    dev_err(dev, "Failed to get MSI interrupts\n");
    return irq;
    }
    argument &= ~MSIX_ENABLE;
    }
    ndev.db_count = irq - 1;
    for (i = 0; i < irq; i++) {
    ndev.irq_ctx[i].ndev = ndev;
    ndev.irq_ctx[i].irq_no = i;
    ret = request_irq(pci_irq_vector(pdev, i), ntb_epf_vec_isr,
    0, "ntb_epf", &ndev.irq_ctx[i]);
    if (ret) {
    dev_err(dev, "Failed to request irq\n");
    goto err_free_irq;
    }
    }
    ret = ntb_epf_send_command(ndev, CMD_CONFIGURE_DOORBELL,
    argument | irq);
    if (ret) {
    dev_err(dev, "Failed to configure doorbell\n");
    goto err_free_irq;
    }
    return 0;
    err_free_irq:
    while (i--)
    free_irq(pci_irq_vector(pdev, i), &ndev.irq_ctx[i]);
    pci_free_irq_vectors(pdev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ntb_epf_peer_mw_count(ntb: *mut ntb_dev) -> c_int {
    static int ntb_epf_peer_mw_count(struct ntb_dev *ntb)
    {
    return ntb_ndev(ntb).mw_count;
    }
#[no_mangle]
unsafe extern "C" fn ntb_epf_spad_count(ntb: *mut ntb_dev) -> c_int {
    static int ntb_epf_spad_count(struct ntb_dev *ntb)
    {
    return ntb_ndev(ntb).spad_count;
    }
#[no_mangle]
unsafe extern "C" fn ntb_epf_db_valid_mask(ntb: *mut ntb_dev) -> u64 {
    static u64 ntb_epf_db_valid_mask(struct ntb_dev *ntb)
    {
    return ntb_ndev(ntb).db_valid_mask;
    }
#[no_mangle]
unsafe extern "C" fn ntb_epf_db_vector_count(ntb: *mut ntb_dev) -> c_int {
    static int ntb_epf_db_vector_count(struct ntb_dev *ntb)
    {
    struct ntb_epf_dev *ndev = ntb_ndev(ntb);
    let mut db_count: c_uint = ndev.db_count;
//
// db_count includes an extra skipped slot due to the legacy
// doorbell layout. Expose only the real doorbell vectors.
//
    if (db_count < NTB_EPF_MIN_DB_COUNT)
    return 0;
    return db_count - 1;
    }
#[no_mangle]
unsafe extern "C" fn ntb_epf_db_vector_mask(ntb: *mut ntb_dev, db_vector: c_int) -> u64 {
    static u64 ntb_epf_db_vector_mask(struct ntb_dev *ntb, int db_vector)
    {
    int nr_vec;
//
// db_count includes one skipped slot in the legacy layout. Valid
// doorbell vectors are therefore [0 .. (db_count - 2)].
//
    nr_vec = ntb_epf_db_vector_count(ntb);
    if (db_vector < 0 || db_vector >= nr_vec)
    return 0;
    return BIT_ULL(db_vector);
    }
#[no_mangle]
unsafe extern "C" fn ntb_epf_db_set_mask(ntb: *mut ntb_dev, db_bits: u64) -> c_int {
    static int ntb_epf_db_set_mask(struct ntb_dev *ntb, u64 db_bits)
    {
    return 0;
    }
    static int ntb_epf_mw_set_trans(struct ntb_dev *ntb, int pidx, int idx,
    dma_addr_t addr, resource_size_t size)
    {
    struct ntb_epf_dev *ndev = ntb_ndev(ntb);
    struct device *dev = ndev.dev;
    resource_size_t mw_size;
    int bar;
    if (pidx != NTB_DEF_PEER_IDX) {
    dev_err(dev, "Unsupported Peer ID %d\n", pidx);
    return -EINVAL;
    }
    bar = ntb_epf_mw_to_bar(ndev, idx);
    if (bar < 0)
    return bar;
    mw_size = pci_resource_len(ntb.pdev, bar);
    if (size > mw_size) {
    dev_err(dev, "Size:%pa is greater than the MW size %pa\n",
    &size, &mw_size);
    return -EINVAL;
    }
    writel(lower_32_bits(addr), ndev.ctrl_reg + NTB_EPF_LOWER_ADDR);
    writel(upper_32_bits(addr), ndev.ctrl_reg + NTB_EPF_UPPER_ADDR);
    writel(lower_32_bits(size), ndev.ctrl_reg + NTB_EPF_LOWER_SIZE);
    writel(upper_32_bits(size), ndev.ctrl_reg + NTB_EPF_UPPER_SIZE);
    ntb_epf_send_command(ndev, CMD_CONFIGURE_MW, idx);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ntb_epf_mw_clear_trans(ntb: *mut ntb_dev, pidx: c_int, idx: c_int) -> c_int {
    static int ntb_epf_mw_clear_trans(struct ntb_dev *ntb, int pidx, int idx)
    {
    struct ntb_epf_dev *ndev = ntb_ndev(ntb);
    struct device *dev = ndev.dev;
    let mut ret: c_int = 0;
    ntb_epf_send_command(ndev, CMD_TEARDOWN_MW, idx);
    if (ret)
    dev_err(dev, "Failed to teardown memory window\n");
    return ret;
    }
    static int ntb_epf_peer_mw_get_addr(struct ntb_dev *ntb, int idx,
    phys_addr_t *base, resource_size_t *size)
    {
    struct ntb_epf_dev *ndev = ntb_ndev(ntb);
    let mut offset: u32 = 0;
    int bar;
    if (idx == 0)
    offset = readl(ndev.ctrl_reg + NTB_EPF_MW1_OFFSET);
    bar = ntb_epf_mw_to_bar(ndev, idx);
    if (bar < 0)
    return bar;
    if (base)
// base = pci_resource_start(ndev->ntb.pdev, bar) + offset;
    if (size)
// size = pci_resource_len(ndev->ntb.pdev, bar) - offset;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ntb_epf_peer_db_set(ntb: *mut ntb_dev, db_bits: u64) -> c_int {
    static int ntb_epf_peer_db_set(struct ntb_dev *ntb, u64 db_bits)
    {
    struct ntb_epf_dev *ndev = ntb_ndev(ntb);
//
// ffs() returns a 1-based bit index (bit 0 -> 1).
//
// With slot 0 reserved for link events, DB#0 would naturally map to
// slot 1. Historically an extra +1 offset was added, so DB#0 maps to
// slot 2 and slot 1 remains unused. Keep this mapping for
// backward-compatibility.
//
    let mut interrupt_num: u32 = ffs(db_bits) + 1;
    struct device *dev = ndev.dev;
    u32 db_entry_size;
    u32 db_offset;
    u32 db_data;
    if (interrupt_num > ndev.db_count) {
    dev_err(dev, "DB interrupt %d greater than Max Supported %d\n",
    interrupt_num, ndev.db_count);
    return -EINVAL;
    }
    db_entry_size = readl(ndev.ctrl_reg + NTB_EPF_DB_ENTRY_SIZE);
    db_data = readl(ndev.ctrl_reg + NTB_EPF_DB_DATA(interrupt_num));
    db_offset = readl(ndev.ctrl_reg + NTB_EPF_DB_OFFSET(interrupt_num));
    writel(db_data, ndev.db_reg + (db_entry_size * interrupt_num) +
    db_offset);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ntb_epf_db_read(ntb: *mut ntb_dev) -> u64 {
    static u64 ntb_epf_db_read(struct ntb_dev *ntb)
    {
    struct ntb_epf_dev *ndev = ntb_ndev(ntb);
    return atomic64_read(&ndev.db_val);
    }
#[no_mangle]
unsafe extern "C" fn ntb_epf_db_clear_mask(ntb: *mut ntb_dev, db_bits: u64) -> c_int {
    static int ntb_epf_db_clear_mask(struct ntb_dev *ntb, u64 db_bits)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ntb_epf_db_clear(ntb: *mut ntb_dev, db_bits: u64) -> c_int {
    static int ntb_epf_db_clear(struct ntb_dev *ntb, u64 db_bits)
    {
    struct ntb_epf_dev *ndev = ntb_ndev(ntb);
    atomic64_and(~db_bits, &ndev.db_val);
    return 0;
    }
    static const struct ntb_dev_ops ntb_epf_ops = {
    .mw_count		= ntb_epf_mw_count,
    .spad_count		= ntb_epf_spad_count,
    .peer_mw_count		= ntb_epf_peer_mw_count,
    .db_valid_mask		= ntb_epf_db_valid_mask,
    .db_vector_count	= ntb_epf_db_vector_count,
    .db_vector_mask		= ntb_epf_db_vector_mask,
    .db_set_mask		= ntb_epf_db_set_mask,
    .mw_set_trans		= ntb_epf_mw_set_trans,
    .mw_clear_trans		= ntb_epf_mw_clear_trans,
    .peer_mw_get_addr	= ntb_epf_peer_mw_get_addr,
    .link_enable		= ntb_epf_link_enable,
    .spad_read		= ntb_epf_spad_read,
    .spad_write		= ntb_epf_spad_write,
    .peer_spad_read		= ntb_epf_peer_spad_read,
    .peer_spad_write	= ntb_epf_peer_spad_write,
    .peer_db_set		= ntb_epf_peer_db_set,
    .db_read		= ntb_epf_db_read,
    .mw_get_align		= ntb_epf_mw_get_align,
    .link_is_up		= ntb_epf_link_is_up,
    .db_clear_mask		= ntb_epf_db_clear_mask,
    .db_clear		= ntb_epf_db_clear,
    .link_disable		= ntb_epf_link_disable,
    };
    static inline void ntb_epf_init_struct(struct ntb_epf_dev *ndev,
    struct pci_dev *pdev)
    {
    ndev.ntb.pdev = pdev;
    ndev.ntb.topo = NTB_TOPO_NONE;
    ndev.ntb.ops = &ntb_epf_ops;
    }
#[no_mangle]
unsafe extern "C" fn ntb_epf_init_dev(ndev: *mut ntb_epf_dev) -> c_int {
    static int ntb_epf_init_dev(struct ntb_epf_dev *ndev)
    {
    struct device *dev = ndev.dev;
    int ret;
    ndev.mw_count = readl(ndev.ctrl_reg + NTB_EPF_MW_COUNT);
    if (ndev.mw_count > NTB_EPF_MAX_MW_COUNT) {
    dev_err(dev, "Unsupported MW count: %u\n", ndev.mw_count);
    return -EINVAL;
    }
// One Link interrupt and rest doorbell interrupt
    ret = ntb_epf_init_isr(ndev, NTB_EPF_MIN_DB_COUNT + 1,
    NTB_EPF_MAX_DB_COUNT + 1);
    if (ret) {
    dev_err(dev, "Failed to init ISR\n");
    return ret;
    }
//
// ndev->db_count includes an extra skipped slot due to the legacy
// doorbell layout, hence -1.
//
    ndev.db_valid_mask = BIT_ULL(ndev.db_count - 1) - 1;
    ndev.spad_count = readl(ndev.ctrl_reg + NTB_EPF_SPAD_COUNT);
    return 0;
    }
    static int ntb_epf_init_pci(struct ntb_epf_dev *ndev,
    struct pci_dev *pdev)
    {
    struct device *dev = ndev.dev;
    size_t spad_sz, spad_off;
    int ret;
    pci_set_drvdata(pdev, ndev);
    ret = pci_enable_device(pdev);
    if (ret) {
    dev_err(dev, "Cannot enable PCI device\n");
    goto err_pci_enable;
    }
    ret = pci_request_regions(pdev, "ntb");
    if (ret) {
    dev_err(dev, "Cannot obtain PCI resources\n");
    goto err_pci_regions;
    }
    pci_set_master(pdev);
    ret = dma_set_mask_and_coherent(dev, DMA_BIT_MASK(64));
    if (ret) {
    ret = dma_set_mask_and_coherent(dev, DMA_BIT_MASK(32));
    if (ret) {
    dev_err(dev, "Cannot set DMA mask\n");
    goto err_pci_regions;
    }
    dev_warn(&pdev.dev, "Cannot DMA highmem\n");
    }
    ndev.ctrl_reg = pci_iomap(pdev, ndev.barno_map[BAR_CONFIG], 0);
    if (!ndev.ctrl_reg) {
    ret = -EIO;
    goto err_pci_regions;
    }
    if (ndev.barno_map[BAR_PEER_SPAD] != ndev.barno_map[BAR_CONFIG]) {
    ndev.peer_spad_reg = pci_iomap(pdev,
    ndev.barno_map[BAR_PEER_SPAD], 0);
    if (!ndev.peer_spad_reg) {
    ret = -EIO;
    goto err_pci_regions;
    }
    } else {
    spad_sz = 4 * readl(ndev.ctrl_reg + NTB_EPF_SPAD_COUNT);
    spad_off = readl(ndev.ctrl_reg + NTB_EPF_SPAD_OFFSET);
    ndev.peer_spad_reg = ndev.ctrl_reg + spad_off  + spad_sz;
    }
    ndev.db_reg = pci_iomap(pdev, ndev.barno_map[BAR_DB], 0);
    if (!ndev.db_reg) {
    ret = -EIO;
    goto err_pci_regions;
    }
    return 0;
    err_pci_regions:
    pci_disable_device(pdev);
    err_pci_enable:
    pci_set_drvdata(pdev, core::ptr::null_mut());
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ntb_epf_deinit_pci(ndev: *mut ntb_epf_dev) {
    static void ntb_epf_deinit_pci(struct ntb_epf_dev *ndev)
    {
    struct pci_dev *pdev = ndev.ntb.pdev;
    pci_iounmap(pdev, ndev.ctrl_reg);
    if (ndev.barno_map[BAR_PEER_SPAD] != ndev.barno_map[BAR_CONFIG])
    pci_iounmap(pdev, ndev.peer_spad_reg);
    pci_iounmap(pdev, ndev.db_reg);
    pci_release_regions(pdev);
    pci_disable_device(pdev);
    pci_set_drvdata(pdev, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn ntb_epf_cleanup_isr(ndev: *mut ntb_epf_dev) {
    static void ntb_epf_cleanup_isr(struct ntb_epf_dev *ndev)
    {
    struct pci_dev *pdev = ndev.ntb.pdev;
    int i;
    ntb_epf_send_command(ndev, CMD_TEARDOWN_DOORBELL, ndev.db_count + 1);
    for (i = 0; i < ndev.db_count + 1; i++)
    free_irq(pci_irq_vector(pdev, i), &ndev.irq_ctx[i]);
    pci_free_irq_vectors(pdev);
    }
    static int ntb_epf_pci_probe(struct pci_dev *pdev,
    const struct pci_device_id *id)
    {
    struct device *dev = &pdev.dev;
    struct ntb_epf_dev *ndev;
    int ret;
    if (pci_is_bridge(pdev))
    return -ENODEV;
    ndev = devm_kzalloc(dev, sizeof(*ndev), GFP_KERNEL);
    if (!ndev)
    return -ENOMEM;
    ndev.barno_map = (const enum pci_barno *)id.driver_data;
    if (!ndev.barno_map)
    return -EINVAL;
    ndev.dev = dev;
    ntb_epf_init_struct(ndev, pdev);
    mutex_init(&ndev.cmd_lock);
    ret = ntb_epf_init_pci(ndev, pdev);
    if (ret) {
    dev_err(dev, "Failed to init PCI\n");
    return ret;
    }
    ret = ntb_epf_init_dev(ndev);
    if (ret) {
    dev_err(dev, "Failed to init device\n");
    goto err_init_dev;
    }
    ret = ntb_register_device(&ndev.ntb);
    if (ret) {
    dev_err(dev, "Failed to register NTB device\n");
    goto err_register_dev;
    }
    return 0;
    err_register_dev:
    ntb_epf_cleanup_isr(ndev);
    err_init_dev:
    ntb_epf_deinit_pci(ndev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ntb_epf_pci_remove(pdev: *mut pci_dev) {
    static void ntb_epf_pci_remove(struct pci_dev *pdev)
    {
    struct ntb_epf_dev *ndev = pci_get_drvdata(pdev);
    ntb_unregister_device(&ndev.ntb);
    ntb_epf_cleanup_isr(ndev);
    ntb_epf_deinit_pci(ndev);
    }
    static const enum pci_barno j721e_map[NTB_BAR_NUM] = {
    [BAR_CONFIG]	= BAR_0,
    [BAR_PEER_SPAD]	= BAR_1,
    [BAR_DB]	= BAR_2,
    [BAR_MW1]	= BAR_2,
    [BAR_MW2]	= BAR_3,
    [BAR_MW3]	= BAR_4,
    [BAR_MW4]	= BAR_5
    };
    static const enum pci_barno mx8_map[NTB_BAR_NUM] = {
    [BAR_CONFIG]	= BAR_0,
    [BAR_PEER_SPAD]	= BAR_0,
    [BAR_DB]	= BAR_2,
    [BAR_MW1]	= BAR_4,
    [BAR_MW2]	= BAR_5,
    [BAR_MW3]	= NO_BAR,
    [BAR_MW4]	= NO_BAR
    };
    static const enum pci_barno rcar_barno[NTB_BAR_NUM] = {
    [BAR_CONFIG]	= BAR_0,
    [BAR_PEER_SPAD]	= BAR_0,
    [BAR_DB]	= BAR_4,
    [BAR_MW1]	= BAR_2,
    [BAR_MW2]	= NO_BAR,
    [BAR_MW3]	= NO_BAR,
    [BAR_MW4]	= NO_BAR,
    };
    static const struct pci_device_id ntb_epf_pci_tbl[] = {
    {
    PCI_DEVICE(PCI_VENDOR_ID_TI, PCI_DEVICE_ID_TI_J721E),
    .class = PCI_CLASS_MEMORY_RAM << 8, .class_mask = 0xffff00,
    .driver_data = (kernel_ulong_t)j721e_map,
    },
    {
    PCI_DEVICE(PCI_VENDOR_ID_FREESCALE, 0x0809),
    .class = PCI_CLASS_MEMORY_RAM << 8, .class_mask = 0xffff00,
    .driver_data = (kernel_ulong_t)mx8_map,
    },
    {
    PCI_DEVICE(PCI_VENDOR_ID_RENESAS, 0x0030),
    .class = PCI_CLASS_MEMORY_RAM << 8, .class_mask = 0xffff00,
    .driver_data = (kernel_ulong_t)rcar_barno,
    },
    { },
    };
    static struct pci_driver ntb_epf_pci_driver = {
    .name		= KBUILD_MODNAME,
    .id_table	= ntb_epf_pci_tbl,
    .probe		= ntb_epf_pci_probe,
    .remove		= ntb_epf_pci_remove,
    };
    module_pci_driver(ntb_epf_pci_driver);
    MODULE_DESCRIPTION("PCI ENDPOINT NTB HOST DRIVER");
    MODULE_AUTHOR("Kishon Vijay Abraham I <kishon@ti.com>");
    MODULE_LICENSE("GPL v2");
