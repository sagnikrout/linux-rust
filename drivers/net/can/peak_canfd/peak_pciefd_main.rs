//! Automatically rewritten from C to Rust
//! Source: drivers/net/can/peak_canfd/peak_pciefd_main.c
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
// Copyright (C) 2007, 2011 Wolfgang Grandegger <wg@grandegger.com>
//
// Derived from the PCAN project file driver/src/pcan_pci.c:
//
// Copyright (C) 2001-2025 PEAK System-Technik GmbH
// Author: Stéphane Grosjean <s.grosjean@peak-system.fr>
//

    MODULE_AUTHOR("Stéphane Grosjean <s.grosjean@peak-system.fr>");
    MODULE_DESCRIPTION("Socket-CAN driver for PEAK PCAN PCIe/M.2 FD family cards");
    MODULE_LICENSE("GPL v2");

pub const PEAK_PCI_VENDOR_ID: c_uint = 0x001c	/* The PCI device and vendor IDs */;
pub const PEAK_PCIEFD_ID: c_uint = 0x0013	/* for PCIe slot cards */;
pub const PCAN_CPCIEFD_ID: c_uint = 0x0014	/* for Compact-PCI Serial slot cards */;
pub const PCAN_PCIE104FD_ID: c_uint = 0x0017	/* for PCIe-104 Express slot cards */;
pub const PCAN_MINIPCIEFD_ID: c_uint = 0x0018	/* for mini-PCIe slot cards */;
pub const PCAN_PCIEFD_OEM_ID: c_uint = 0x0019	/* for PCIe slot OEM cards */;
pub const PCAN_M2_ID: c_uint = 0x001a	/* for M2 slot cards */;
// PEAK PCIe board access description

// System Control Registers
pub const PCIEFD_REG_SYS_CTL_SET: c_uint = 0x0000	/* set bits */;
pub const PCIEFD_REG_SYS_CTL_CLR: c_uint = 0x0004	/* clear bits */;
// Version info registers
pub const PCIEFD_REG_SYS_VER1: c_uint = 0x0040	/* version reg #1 */;
pub const PCIEFD_REG_SYS_VER2: c_uint = 0x0044	/* version reg #2 */;

    ((u32)(y) << 16) | \
    ((u32)(z) << 8))
// System Control Registers Bits
pub const PCIEFD_SYS_CTL_TS_RST: c_uint = 0x00000001	/* timestamp clock */;
pub const PCIEFD_SYS_CTL_CLK_EN: c_uint = 0x00000002	/* system clock */;
// CAN-FD channel addresses

// CAN-FD channel registers
pub const PCIEFD_REG_CAN_MISC: c_uint = 0x0000	/* Misc. control */;
pub const PCIEFD_REG_CAN_CLK_SEL: c_uint = 0x0008	/* Clock selector */;
pub const PCIEFD_REG_CAN_CMD_PORT_L: c_uint = 0x0010	/* 64-bits command port */;
pub const PCIEFD_REG_CAN_CMD_PORT_H: c_uint = 0x0014;
pub const PCIEFD_REG_CAN_TX_REQ_ACC: c_uint = 0x0020	/* Tx request accumulator */;
pub const PCIEFD_REG_CAN_TX_CTL_SET: c_uint = 0x0030	/* Tx control set register */;
pub const PCIEFD_REG_CAN_TX_CTL_CLR: c_uint = 0x0038	/* Tx control clear register */;
pub const PCIEFD_REG_CAN_TX_DMA_ADDR_L: c_uint = 0x0040	/* 64-bits addr for Tx DMA */;
pub const PCIEFD_REG_CAN_TX_DMA_ADDR_H: c_uint = 0x0044;
pub const PCIEFD_REG_CAN_RX_CTL_SET: c_uint = 0x0050	/* Rx control set register */;
pub const PCIEFD_REG_CAN_RX_CTL_CLR: c_uint = 0x0058	/* Rx control clear register */;
pub const PCIEFD_REG_CAN_RX_CTL_WRT: c_uint = 0x0060	/* Rx control write register */;
pub const PCIEFD_REG_CAN_RX_CTL_ACK: c_uint = 0x0068	/* Rx control ACK register */;
pub const PCIEFD_REG_CAN_RX_DMA_ADDR_L: c_uint = 0x0070	/* 64-bits addr for Rx DMA */;
pub const PCIEFD_REG_CAN_RX_DMA_ADDR_H: c_uint = 0x0074;
// CAN-FD channel misc register bits
pub const CANFD_MISC_TS_RST: c_uint = 0x00000001	/* timestamp cnt rst */;
// CAN-FD channel Clock SELector Source & DIVider
pub const CANFD_CLK_SEL_DIV_MASK: c_uint = 0x00000007;
pub const CANFD_CLK_SEL_DIV_60MHZ: c_uint = 0x00000000	/* SRC=240MHz only */;
pub const CANFD_CLK_SEL_DIV_40MHZ: c_uint = 0x00000001	/* SRC=240MHz only */;
pub const CANFD_CLK_SEL_DIV_30MHZ: c_uint = 0x00000002	/* SRC=240MHz only */;
pub const CANFD_CLK_SEL_DIV_24MHZ: c_uint = 0x00000003	/* SRC=240MHz only */;
pub const CANFD_CLK_SEL_DIV_20MHZ: c_uint = 0x00000004	/* SRC=240MHz only */;
pub const CANFD_CLK_SEL_SRC_MASK: c_uint = 0x00000008	/* 0=80MHz, 1=240MHz */;
pub const CANFD_CLK_SEL_SRC_240MHZ: c_uint = 0x00000008;

    CANFD_CLK_SEL_SRC_MASK)

    CANFD_CLK_SEL_DIV_20MHZ)

    CANFD_CLK_SEL_DIV_24MHZ)

    CANFD_CLK_SEL_DIV_30MHZ)

    CANFD_CLK_SEL_DIV_40MHZ)

    CANFD_CLK_SEL_DIV_60MHZ)

// CAN-FD channel Rx/Tx control register bits
pub const CANFD_CTL_UNC_BIT: c_uint = 0x00010000	/* Uncached DMA mem */;
pub const CANFD_CTL_RST_BIT: c_uint = 0x00020000	/* reset DMA action */;
pub const CANFD_CTL_IEN_BIT: c_uint = 0x00040000	/* IRQ enable */;
// Rx IRQ Count and Time Limits

// Tx anticipation window (link logical address should be aligned on 2K
// boundary)
//

pub const CANFD_MSG_LNK_TX: c_uint = 0x1001	/* Tx msgs link */;
// 32-bits IRQ status fields, heading Rx DMA area
#[no_mangle]
pub unsafe extern "C" fn pciefd_irq_tag(irq_status: u32) -> c_int {
    static inline int pciefd_irq_tag(u32 irq_status)
    {
    return irq_status & 0x0000000f;
    }
#[no_mangle]
pub unsafe extern "C" fn pciefd_irq_rx_cnt(irq_status: u32) -> c_int {
    static inline int pciefd_irq_rx_cnt(u32 irq_status)
    {
    return (irq_status & 0x000007f0) >> 4;
    }
#[no_mangle]
pub unsafe extern "C" fn pciefd_irq_is_lnk(irq_status: u32) -> c_int {
    static inline int pciefd_irq_is_lnk(u32 irq_status)
    {
    return irq_status & 0x00010000;
    }
// Rx record
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pciefd_rx_dma {
    pub irq_status: __le32,
    pub sys_time_low: __le32,
    pub sys_time_high: __le32,
    pub msg: [pucan_rx_msg; ],
    pub __aligned(4): } __packed,
// Tx Link record
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pciefd_tx_link {
    pub size: __le16,
    pub type: __le16,
    pub laddr_lo: __le32,
    pub laddr_hi: __le32,
    pub __aligned(4): } __packed,
// Tx page descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pciefd_page {
    pub /: *mut *mut *mut void vbase; / page virtual address,
    pub /: *mut *mut dma_addr_t lbase; / page logical address,
    pub offset: u32,
    pub size: u32,
}

// CAN-FD channel object
    struct pciefd_board;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pciefd_can {
    pub /: *mut *mut peak_canfd_priv ucan; / must be the first member,
    pub /: *mut *mut *mut void __iomem reg_base; / channel config base addr,
    pub /: *mut *mut *mut pciefd_board board; / reverse link,
    pub /: *mut *mut pucan_command pucan_cmd; / command buffer,
    pub /: *mut *mut dma_addr_t rx_dma_laddr; / DMA virtual and logical addr,
    pub /: *mut *mut *mut void rx_dma_vaddr; / for Rx and Tx areas,
    pub tx_dma_laddr: dma_addr_t,
    pub tx_dma_vaddr: *mut c_void,
    pub tx_pages: [pciefd_page; PCIEFD_TX_PAGE_COUNT],
    pub /: *mut *mut u16 tx_pages_free; / free Tx pages counter,
    pub /: *mut *mut u16 tx_page_index; / current page used for Tx,
    pub tx_lock: spinlock_t,
    pub irq_status: u32,
    pub /: *mut *mut u32 irq_tag; / next irq tag,
}

// PEAK-PCIe FD board object
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pciefd_board {
    pub reg_base: *mut void __iomem,
    pub pci_dev: *mut pci_dev,
    pub can_count: c_int,
    pub /: *mut *mut spinlock_t cmd_lock; / 64-bits cmds must be atomic,
    pub /: *mut *mut *mut pciefd_can can[]; / array of network devices,
}

// supported device ids.
    static const struct pci_device_id peak_pciefd_tbl[] = {
    {PEAK_PCI_VENDOR_ID, PEAK_PCIEFD_ID, PCI_ANY_ID, PCI_ANY_ID,},
    {PEAK_PCI_VENDOR_ID, PCAN_CPCIEFD_ID, PCI_ANY_ID, PCI_ANY_ID,},
    {PEAK_PCI_VENDOR_ID, PCAN_PCIE104FD_ID, PCI_ANY_ID, PCI_ANY_ID,},
    {PEAK_PCI_VENDOR_ID, PCAN_MINIPCIEFD_ID, PCI_ANY_ID, PCI_ANY_ID,},
    {PEAK_PCI_VENDOR_ID, PCAN_PCIEFD_OEM_ID, PCI_ANY_ID, PCI_ANY_ID,},
    {PEAK_PCI_VENDOR_ID, PCAN_M2_ID, PCI_ANY_ID, PCI_ANY_ID,},
    {0,}
    };
    MODULE_DEVICE_TABLE(pci, peak_pciefd_tbl);
// read a 32 bits value from a SYS block register
#[no_mangle]
pub unsafe extern "C" fn pciefd_sys_readreg(priv: *const pciefd_board, reg: u16) -> u32 {
    static inline u32 pciefd_sys_readreg(const struct pciefd_board *priv, u16 reg)
    {
    return readl(priv.reg_base + reg);
    }
// write a 32 bits value into a SYS block register
    static inline void pciefd_sys_writereg(const struct pciefd_board *priv,
    u32 val, u16 reg)
    {
    writel(val, priv.reg_base + reg);
    }
// read a 32 bits value from CAN-FD block register
#[no_mangle]
pub unsafe extern "C" fn pciefd_can_readreg(priv: *const pciefd_can, reg: u16) -> u32 {
    static inline u32 pciefd_can_readreg(const struct pciefd_can *priv, u16 reg)
    {
    return readl(priv.reg_base + reg);
    }
// write a 32 bits value into a CAN-FD block register
    static inline void pciefd_can_writereg(const struct pciefd_can *priv,
    u32 val, u16 reg)
    {
    writel(val, priv.reg_base + reg);
    }
// give a channel logical Rx DMA address to the board
#[no_mangle]
unsafe extern "C" fn pciefd_can_setup_rx_dma(priv: *mut pciefd_can) {
    static void pciefd_can_setup_rx_dma(struct pciefd_can *priv)
    {

    let mut dma_addr_h: u32 = (u32)(priv.rx_dma_laddr >> 32);

    let mut dma_addr_h: u32 = 0;

// (DMA must be reset for Rx)
    pciefd_can_writereg(priv, CANFD_CTL_RST_BIT, PCIEFD_REG_CAN_RX_CTL_SET);
// write the logical address of the Rx DMA area for this channel
    pciefd_can_writereg(priv, (u32)priv.rx_dma_laddr,
    PCIEFD_REG_CAN_RX_DMA_ADDR_L);
    pciefd_can_writereg(priv, dma_addr_h, PCIEFD_REG_CAN_RX_DMA_ADDR_H);
// also indicates that Rx DMA is cacheable
    pciefd_can_writereg(priv, CANFD_CTL_UNC_BIT, PCIEFD_REG_CAN_RX_CTL_CLR);
    }
// clear channel logical Rx DMA address from the board
#[no_mangle]
unsafe extern "C" fn pciefd_can_clear_rx_dma(priv: *mut pciefd_can) {
    static void pciefd_can_clear_rx_dma(struct pciefd_can *priv)
    {
// DMA must be reset for Rx
    pciefd_can_writereg(priv, CANFD_CTL_RST_BIT, PCIEFD_REG_CAN_RX_CTL_SET);
// clear the logical address of the Rx DMA area for this channel
    pciefd_can_writereg(priv, 0, PCIEFD_REG_CAN_RX_DMA_ADDR_L);
    pciefd_can_writereg(priv, 0, PCIEFD_REG_CAN_RX_DMA_ADDR_H);
    }
// give a channel logical Tx DMA address to the board
#[no_mangle]
unsafe extern "C" fn pciefd_can_setup_tx_dma(priv: *mut pciefd_can) {
    static void pciefd_can_setup_tx_dma(struct pciefd_can *priv)
    {

    let mut dma_addr_h: u32 = (u32)(priv.tx_dma_laddr >> 32);

    let mut dma_addr_h: u32 = 0;

// (DMA must be reset for Tx)
    pciefd_can_writereg(priv, CANFD_CTL_RST_BIT, PCIEFD_REG_CAN_TX_CTL_SET);
// write the logical address of the Tx DMA area for this channel
    pciefd_can_writereg(priv, (u32)priv.tx_dma_laddr,
    PCIEFD_REG_CAN_TX_DMA_ADDR_L);
    pciefd_can_writereg(priv, dma_addr_h, PCIEFD_REG_CAN_TX_DMA_ADDR_H);
// also indicates that Tx DMA is cacheable
    pciefd_can_writereg(priv, CANFD_CTL_UNC_BIT, PCIEFD_REG_CAN_TX_CTL_CLR);
    }
// clear channel logical Tx DMA address from the board
#[no_mangle]
unsafe extern "C" fn pciefd_can_clear_tx_dma(priv: *mut pciefd_can) {
    static void pciefd_can_clear_tx_dma(struct pciefd_can *priv)
    {
// DMA must be reset for Tx
    pciefd_can_writereg(priv, CANFD_CTL_RST_BIT, PCIEFD_REG_CAN_TX_CTL_SET);
// clear the logical address of the Tx DMA area for this channel
    pciefd_can_writereg(priv, 0, PCIEFD_REG_CAN_TX_DMA_ADDR_L);
    pciefd_can_writereg(priv, 0, PCIEFD_REG_CAN_TX_DMA_ADDR_H);
    }
#[no_mangle]
unsafe extern "C" fn pciefd_can_ack_rx_dma(priv: *mut pciefd_can) {
    static void pciefd_can_ack_rx_dma(struct pciefd_can *priv)
    {
// read value of current IRQ tag and inc it for next one
    priv.irq_tag = le32_to_cpu(*(__le32 *)priv.rx_dma_vaddr);
    priv.irq_tag++;
    priv.irq_tag &= 0xf;
// write the next IRQ tag for this CAN
    pciefd_can_writereg(priv, priv.irq_tag, PCIEFD_REG_CAN_RX_CTL_ACK);
    }
// IRQ handler
#[no_mangle]
unsafe extern "C" fn pciefd_irq_handler(irq: c_int, arg: *mut c_void) -> irqreturn_t {
    static irqreturn_t pciefd_irq_handler(int irq, void *arg)
    {
    struct pciefd_can *priv = arg;
    struct pciefd_rx_dma *rx_dma = priv.rx_dma_vaddr;
// INTA mode only to sync with PCIe transaction
    if (!pci_dev_msi_enabled(priv.board.pci_dev))
    (void)pciefd_sys_readreg(priv.board, PCIEFD_REG_SYS_VER1);
// read IRQ status from the first 32-bits of the Rx DMA area
    priv.irq_status = le32_to_cpu(rx_dma.irq_status);
// check if this (shared) IRQ is for this CAN
    if (pciefd_irq_tag(priv.irq_status) != priv.irq_tag)
    return IRQ_NONE;
// handle rx messages (if any)
    peak_canfd_handle_msgs_list(&priv.ucan,
    rx_dma.msg,
    pciefd_irq_rx_cnt(priv.irq_status));
// handle tx link interrupt (if any)
    if (pciefd_irq_is_lnk(priv.irq_status)) {
    unsigned long flags;
    spin_lock_irqsave(&priv.tx_lock, flags);
    priv.tx_pages_free++;
    spin_unlock_irqrestore(&priv.tx_lock, flags);
// wake producer up (only if enough room in echo_skb array)
    spin_lock_irqsave(&priv.ucan.echo_lock, flags);
    if (!priv.ucan.can.echo_skb[priv.ucan.echo_idx])
    netif_wake_queue(priv.ucan.ndev);
    spin_unlock_irqrestore(&priv.ucan.echo_lock, flags);
    }
// re-enable Rx DMA transfer for this CAN
    pciefd_can_ack_rx_dma(priv);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn pciefd_enable_tx_path(ucan: *mut peak_canfd_priv) -> c_int {
    static int pciefd_enable_tx_path(struct peak_canfd_priv *ucan)
    {
    struct pciefd_can *priv = (struct pciefd_can *)ucan;
    int i;
// initialize the Tx pages descriptors
    priv.tx_pages_free = PCIEFD_TX_PAGE_COUNT - 1;
    priv.tx_page_index = 0;
    priv.tx_pages[0].vbase = priv.tx_dma_vaddr;
    priv.tx_pages[0].lbase = priv.tx_dma_laddr;
    for (i = 0; i < PCIEFD_TX_PAGE_COUNT; i++) {
    priv.tx_pages[i].offset = 0;
    priv.tx_pages[i].size = PCIEFD_TX_PAGE_SIZE -
    sizeof(struct pciefd_tx_link);
    if (i) {
    priv.tx_pages[i].vbase =
    priv.tx_pages[i - 1].vbase +
    PCIEFD_TX_PAGE_SIZE;
    priv.tx_pages[i].lbase =
    priv.tx_pages[i - 1].lbase +
    PCIEFD_TX_PAGE_SIZE;
    }
    }
// setup Tx DMA addresses into IP core
    pciefd_can_setup_tx_dma(priv);
// start (TX_RST=0) Tx Path
    pciefd_can_writereg(priv, CANFD_CTL_RST_BIT, PCIEFD_REG_CAN_TX_CTL_CLR);
    return 0;
    }
// board specific CANFD command pre-processing
#[no_mangle]
unsafe extern "C" fn pciefd_pre_cmd(ucan: *mut peak_canfd_priv) -> c_int {
    static int pciefd_pre_cmd(struct peak_canfd_priv *ucan)
    {
    struct pciefd_can *priv = (struct pciefd_can *)ucan;
    let mut cmd: u16 = pucan_cmd_get_opcode(&priv.pucan_cmd);
    int err;
// pre-process command
    switch (cmd) {
    case PUCAN_CMD_NORMAL_MODE:
    case PUCAN_CMD_LISTEN_ONLY_MODE:
    if (ucan.can.state == CAN_STATE_BUS_OFF)
    break;
// going into operational mode: setup IRQ handler
    err = request_irq(priv.ucan.ndev.irq,
    pciefd_irq_handler,
    IRQF_SHARED,
    PCIEFD_DRV_NAME,
    priv);
    if (err)
    return err;
// setup Rx DMA address
    pciefd_can_setup_rx_dma(priv);
// setup max count of msgs per IRQ
    pciefd_can_writereg(priv, (CANFD_CTL_IRQ_TL_DEF) << 8 |
    CANFD_CTL_IRQ_CL_DEF,
    PCIEFD_REG_CAN_RX_CTL_WRT);
// clear DMA RST for Rx (Rx start)
    pciefd_can_writereg(priv, CANFD_CTL_RST_BIT,
    PCIEFD_REG_CAN_RX_CTL_CLR);
// reset timestamps
    pciefd_can_writereg(priv, !CANFD_MISC_TS_RST,
    PCIEFD_REG_CAN_MISC);
// do an initial ACK
    pciefd_can_ack_rx_dma(priv);
// enable IRQ for this CAN after having set next irq_tag
    pciefd_can_writereg(priv, CANFD_CTL_IEN_BIT,
    PCIEFD_REG_CAN_RX_CTL_SET);
// Tx path will be setup as soon as RX_BARRIER is received
    break;
    default:
    break;
    }
    return 0;
    }
// write a command
#[no_mangle]
unsafe extern "C" fn pciefd_write_cmd(ucan: *mut peak_canfd_priv) -> c_int {
    static int pciefd_write_cmd(struct peak_canfd_priv *ucan)
    {
    struct pciefd_can *priv = (struct pciefd_can *)ucan;
    unsigned long flags;
// 64-bits command is atomic
    spin_lock_irqsave(&priv.board.cmd_lock, flags);
    pciefd_can_writereg(priv, *(u32 *)ucan.cmd_buffer,
    PCIEFD_REG_CAN_CMD_PORT_L);
    pciefd_can_writereg(priv, *(u32 *)(ucan.cmd_buffer + 4),
    PCIEFD_REG_CAN_CMD_PORT_H);
    spin_unlock_irqrestore(&priv.board.cmd_lock, flags);
    return 0;
    }
// board specific CANFD command post-processing
#[no_mangle]
unsafe extern "C" fn pciefd_post_cmd(ucan: *mut peak_canfd_priv) -> c_int {
    static int pciefd_post_cmd(struct peak_canfd_priv *ucan)
    {
    struct pciefd_can *priv = (struct pciefd_can *)ucan;
    let mut cmd: u16 = pucan_cmd_get_opcode(&priv.pucan_cmd);
    switch (cmd) {
    case PUCAN_CMD_RESET_MODE:
    if (ucan.can.state == CAN_STATE_STOPPED)
    break;
// controller now in reset mode:
// disable IRQ for this CAN
    pciefd_can_writereg(priv, CANFD_CTL_IEN_BIT,
    PCIEFD_REG_CAN_RX_CTL_CLR);
// stop and reset DMA addresses in Tx/Rx engines
    pciefd_can_clear_tx_dma(priv);
    pciefd_can_clear_rx_dma(priv);
// wait for above commands to complete (read cycle)
    (void)pciefd_sys_readreg(priv.board, PCIEFD_REG_SYS_VER1);
    free_irq(priv.ucan.ndev.irq, priv);
    ucan.can.state = CAN_STATE_STOPPED;
    break;
    }
    return 0;
    }
    static void *pciefd_alloc_tx_msg(struct peak_canfd_priv *ucan, u16 msg_size,
    int *room_left)
    {
    struct pciefd_can *priv = (struct pciefd_can *)ucan;
    struct pciefd_page *page = priv.tx_pages + priv.tx_page_index;
    unsigned long flags;
    void *msg;
    spin_lock_irqsave(&priv.tx_lock, flags);
    if (page.offset + msg_size > page.size) {
    struct pciefd_tx_link *lk;
// not enough space in this page: try another one
    if (!priv.tx_pages_free) {
    spin_unlock_irqrestore(&priv.tx_lock, flags);
// Tx overflow
    return core::ptr::null_mut();
    }
    priv.tx_pages_free--;
// keep address of the very last free slot of current page
    lk = page.vbase + page.offset;
// next, move on a new free page
    priv.tx_page_index = (priv.tx_page_index + 1) %
    PCIEFD_TX_PAGE_COUNT;
    page = priv.tx_pages + priv.tx_page_index;
// put link record to this new page at the end of prev one
    lk.size = cpu_to_le16(sizeof(*lk));
    lk.type = cpu_to_le16(CANFD_MSG_LNK_TX);
    lk.laddr_lo = cpu_to_le32(page.lbase);

    lk.laddr_hi = cpu_to_le32(page.lbase >> 32);

    lk.laddr_hi = 0;

// next msgs will be put from the begininng of this new page
    page.offset = 0;
    }
// room_left = priv->tx_pages_free * page->size;
    spin_unlock_irqrestore(&priv.tx_lock, flags);
    msg = page.vbase + page.offset;
// give back room left in the tx ring
// room_left += page->size - (page->offset + msg_size);
    return msg;
    }
    static int pciefd_write_tx_msg(struct peak_canfd_priv *ucan,
    struct pucan_tx_msg *msg)
    {
    struct pciefd_can *priv = (struct pciefd_can *)ucan;
    struct pciefd_page *page = priv.tx_pages + priv.tx_page_index;
// this slot is now reserved for writing the frame
    page.offset += le16_to_cpu(msg.size);
// tell the board a frame has been written in Tx DMA area
    pciefd_can_writereg(priv, 1, PCIEFD_REG_CAN_TX_REQ_ACC);
    return 0;
    }
// probe for CAN-FD channel #pciefd_board->can_count
#[no_mangle]
unsafe extern "C" fn pciefd_can_probe(pciefd: *mut pciefd_board) -> c_int {
    static int pciefd_can_probe(struct pciefd_board *pciefd)
    {
    struct net_device *ndev;
    struct pciefd_can *priv;
    u32 clk;
    int err;
// allocate the candev object with default isize of echo skbs ring
    ndev = alloc_peak_canfd_dev(sizeof(*priv), pciefd.can_count,
    PCIEFD_ECHO_SKB_MAX);
    if (!ndev) {
    dev_err(&pciefd.pci_dev.dev,
    "failed to alloc candev object\n");
    goto failure;
    }
    priv = netdev_priv(ndev);
// fill-in candev private object:
// setup PCIe-FD own callbacks
    priv.ucan.pre_cmd = pciefd_pre_cmd;
    priv.ucan.write_cmd = pciefd_write_cmd;
    priv.ucan.post_cmd = pciefd_post_cmd;
    priv.ucan.enable_tx_path = pciefd_enable_tx_path;
    priv.ucan.alloc_tx_msg = pciefd_alloc_tx_msg;
    priv.ucan.write_tx_msg = pciefd_write_tx_msg;
// setup PCIe-FD own command buffer
    priv.ucan.cmd_buffer = &priv.pucan_cmd;
    priv.ucan.cmd_maxlen = sizeof(priv.pucan_cmd);
    priv.board = pciefd;
// CAN config regs block address
    priv.reg_base = pciefd.reg_base + PCIEFD_CANX_OFF(priv.ucan.index);
// allocate non-cacheable DMA'able 4KB memory area for Rx
    priv.rx_dma_vaddr = dmam_alloc_coherent(&pciefd.pci_dev.dev,
    PCIEFD_RX_DMA_SIZE,
    &priv.rx_dma_laddr,
    GFP_KERNEL);
    if (!priv.rx_dma_vaddr) {
    dev_err(&pciefd.pci_dev.dev,
    "Rx dmam_alloc_coherent(%u) failure\n",
    PCIEFD_RX_DMA_SIZE);
    goto err_free_candev;
    }
// allocate non-cacheable DMA'able 4KB memory area for Tx
    priv.tx_dma_vaddr = dmam_alloc_coherent(&pciefd.pci_dev.dev,
    PCIEFD_TX_DMA_SIZE,
    &priv.tx_dma_laddr,
    GFP_KERNEL);
    if (!priv.tx_dma_vaddr) {
    dev_err(&pciefd.pci_dev.dev,
    "Tx dmam_alloc_coherent(%u) failure\n",
    PCIEFD_TX_DMA_SIZE);
    goto err_free_candev;
    }
// CAN clock in RST mode
    pciefd_can_writereg(priv, CANFD_MISC_TS_RST, PCIEFD_REG_CAN_MISC);
// read current clock value
    clk = pciefd_can_readreg(priv, PCIEFD_REG_CAN_CLK_SEL);
    switch (clk) {
    case CANFD_CLK_SEL_20MHZ:
    priv.ucan.can.clock.freq = 20 * 1000 * 1000;
    break;
    case CANFD_CLK_SEL_24MHZ:
    priv.ucan.can.clock.freq = 24 * 1000 * 1000;
    break;
    case CANFD_CLK_SEL_30MHZ:
    priv.ucan.can.clock.freq = 30 * 1000 * 1000;
    break;
    case CANFD_CLK_SEL_40MHZ:
    priv.ucan.can.clock.freq = 40 * 1000 * 1000;
    break;
    case CANFD_CLK_SEL_60MHZ:
    priv.ucan.can.clock.freq = 60 * 1000 * 1000;
    break;
    default:
    pciefd_can_writereg(priv, CANFD_CLK_SEL_80MHZ,
    PCIEFD_REG_CAN_CLK_SEL);
    fallthrough;
    case CANFD_CLK_SEL_80MHZ:
    priv.ucan.can.clock.freq = 80 * 1000 * 1000;
    break;
    }
    ndev.irq = pciefd.pci_dev.irq;
    SET_NETDEV_DEV(ndev, &pciefd.pci_dev.dev);
    err = register_candev(ndev);
    if (err) {
    dev_err(&pciefd.pci_dev.dev,
    "couldn't register CAN device: %d\n", err);
    goto err_free_candev;
    }
    spin_lock_init(&priv.tx_lock);
// save the object address in the board structure
    pciefd.can[pciefd.can_count] = priv;
    dev_info(&pciefd.pci_dev.dev, "%s at reg_base=0x%p irq=%d\n",
    ndev.name, priv.reg_base, ndev.irq);
    return 0;
    err_free_candev:
    free_candev(ndev);
    failure:
    return -ENOMEM;
    }
// remove a CAN-FD channel by releasing all of its resources
#[no_mangle]
unsafe extern "C" fn pciefd_can_remove(priv: *mut pciefd_can) {
    static void pciefd_can_remove(struct pciefd_can *priv)
    {
// unregister (close) the can device to go back to RST mode first
    unregister_candev(priv.ucan.ndev);
// finally, free the candev object
    free_candev(priv.ucan.ndev);
    }
// remove all CAN-FD channels by releasing their own resources
#[no_mangle]
unsafe extern "C" fn pciefd_can_remove_all(pciefd: *mut pciefd_board) {
    static void pciefd_can_remove_all(struct pciefd_board *pciefd)
    {
    while (pciefd.can_count > 0)
    pciefd_can_remove(pciefd.can[--pciefd.can_count]);
    }
// probe for the entire device
    static int peak_pciefd_probe(struct pci_dev *pdev,
    const struct pci_device_id *ent)
    {
    struct pciefd_board *pciefd;
    int err, can_count;
    u16 sub_sys_id;
    u8 hw_ver_major;
    u8 hw_ver_minor;
    u8 hw_ver_sub;
    u32 v2;
    err = pci_enable_device(pdev);
    if (err)
    return err;
    err = pci_request_regions(pdev, PCIEFD_DRV_NAME);
    if (err)
    goto err_disable_pci;
// the number of channels depends on sub-system id
    err = pci_read_config_word(pdev, PCI_SUBSYSTEM_ID, &sub_sys_id);
    if (err)
    goto err_release_regions;
    dev_dbg(&pdev.dev, "probing device %04x:%04x:%04x\n",
    pdev.vendor, pdev.device, sub_sys_id);
    if (sub_sys_id >= 0x0012)
    can_count = 4;
#[no_mangle]
pub unsafe extern "C" fn if(0x0010: sub_sys_id >=) -> else {
    else if (sub_sys_id >= 0x0010)
    can_count = 3;
#[no_mangle]
pub unsafe extern "C" fn if(0x0004: sub_sys_id >=) -> else {
    else if (sub_sys_id >= 0x0004)
    can_count = 2;
    else
    can_count = 1;
// allocate board structure object
    pciefd = devm_kzalloc(&pdev.dev, struct_size(pciefd, can, can_count),
    GFP_KERNEL);
    if (!pciefd) {
    err = -ENOMEM;
    goto err_release_regions;
    }
// initialize the board structure
    pciefd.pci_dev = pdev;
    spin_lock_init(&pciefd.cmd_lock);
// save the PCI BAR0 virtual address for further system regs access
    pciefd.reg_base = pci_iomap(pdev, 0, PCIEFD_BAR0_SIZE);
    if (!pciefd.reg_base) {
    dev_err(&pdev.dev, "failed to map PCI resource #0\n");
    err = -ENOMEM;
    goto err_release_regions;
    }
// read the firmware version number
    v2 = pciefd_sys_readreg(pciefd, PCIEFD_REG_SYS_VER2);
    hw_ver_major = (v2 & 0x0000f000) >> 12;
    hw_ver_minor = (v2 & 0x00000f00) >> 8;
    hw_ver_sub = (v2 & 0x000000f0) >> 4;
    dev_info(&pdev.dev,
    "%ux CAN-FD PCAN-PCIe FPGA v%u.%u.%u:\n", can_count,
    hw_ver_major, hw_ver_minor, hw_ver_sub);

// FW < v3.3.0 DMA logic doesn't handle correctly the mix of 32-bit and
// 64-bit logical addresses: this workaround forces usage of 32-bit
// DMA addresses only when such a fw is detected.
//
    if (PCIEFD_FW_VERSION(hw_ver_major, hw_ver_minor, hw_ver_sub) <
    PCIEFD_FW_VERSION(3, 3, 0)) {
    err = dma_set_mask_and_coherent(&pdev.dev, DMA_BIT_MASK(32));
    if (err)
    dev_warn(&pdev.dev,
    "warning: can't set DMA mask %llxh (err %d)\n",
    DMA_BIT_MASK(32), err);
    }

// stop system clock
    pciefd_sys_writereg(pciefd, PCIEFD_SYS_CTL_CLK_EN,
    PCIEFD_REG_SYS_CTL_CLR);
    pci_set_master(pdev);
// create now the corresponding channels objects
    while (pciefd.can_count < can_count) {
    err = pciefd_can_probe(pciefd);
    if (err)
    goto err_free_canfd;
    pciefd.can_count++;
    }
// set system timestamps counter in RST mode
    pciefd_sys_writereg(pciefd, PCIEFD_SYS_CTL_TS_RST,
    PCIEFD_REG_SYS_CTL_SET);
// wait a bit (read cycle)
    (void)pciefd_sys_readreg(pciefd, PCIEFD_REG_SYS_VER1);
// free all clocks
    pciefd_sys_writereg(pciefd, PCIEFD_SYS_CTL_TS_RST,
    PCIEFD_REG_SYS_CTL_CLR);
// start system clock
    pciefd_sys_writereg(pciefd, PCIEFD_SYS_CTL_CLK_EN,
    PCIEFD_REG_SYS_CTL_SET);
// remember the board structure address in the device user data
    pci_set_drvdata(pdev, pciefd);
    return 0;
    err_free_canfd:
    pciefd_can_remove_all(pciefd);
    pci_iounmap(pdev, pciefd.reg_base);
    err_release_regions:
    pci_release_regions(pdev);
    err_disable_pci:
    pci_disable_device(pdev);
// pci_xxx_config_word() return positive PCIBIOS_xxx error codes while
// the probe() function must return a negative errno in case of failure
// (err is unchanged if negative)
//
    return pcibios_err_to_errno(err);
    }
// free the board structure object, as well as its resources:
#[no_mangle]
unsafe extern "C" fn peak_pciefd_remove(pdev: *mut pci_dev) {
    static void peak_pciefd_remove(struct pci_dev *pdev)
    {
    struct pciefd_board *pciefd = pci_get_drvdata(pdev);
// release CAN-FD channels resources
    pciefd_can_remove_all(pciefd);
    pci_iounmap(pdev, pciefd.reg_base);
    pci_release_regions(pdev);
    pci_disable_device(pdev);
    }
    static struct pci_driver peak_pciefd_driver = {
    .name = PCIEFD_DRV_NAME,
    .id_table = peak_pciefd_tbl,
    .probe = peak_pciefd_probe,
    .remove = peak_pciefd_remove,
    };
    module_pci_driver(peak_pciefd_driver);
