//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/quantenna/qtnfmac/pcie/pearl_pcie.c
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


// SPDX-License-Identifier: GPL-2.0+
// Copyright (c) 2018 Quantenna Communications

pub const PEARL_TX_BD_SIZE_DEFAULT: c_int = 32;
pub const PEARL_RX_BD_SIZE_DEFAULT: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qtnf_pearl_bda {
    pub bda_len: __le16,
    pub bda_version: __le16,
    pub bda_pci_endian: __le32,
    pub bda_ep_state: __le32,
    pub bda_rc_state: __le32,
    pub bda_dma_mask: __le32,
    pub bda_msi_addr: __le32,
    pub bda_flashsz: __le32,
    pub bda_boardname: [u8; PCIE_BDA_NAMELEN],
    pub bda_rc_msi_enabled: __le32,
    pub bda_hhbm_list: [u8; PCIE_HHBM_MAX_SIZE],
    pub bda_dsbw_start_index: __le32,
    pub bda_dsbw_end_index: __le32,
    pub bda_dsbw_total_bytes: __le32,
    pub bda_rc_tx_bd_base: __le32,
    pub bda_rc_tx_bd_num: __le32,
    pub bda_pcie_mac: [u8; QTN_ENET_ADDR_LENGTH],
    pub /: *mut *mut qtnf_shm_ipc_region bda_shm_reg1 __aligned(4096); / host TX,
    pub /: *mut *mut qtnf_shm_ipc_region bda_shm_reg2 __aligned(4096); / host RX,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qtnf_pearl_tx_bd {
    pub addr: __le32,
    pub addr_h: __le32,
    pub info: __le32,
    pub info_h: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qtnf_pearl_rx_bd {
    pub addr: __le32,
    pub addr_h: __le32,
    pub info: __le32,
    pub info_h: __le32,
    pub next_ptr: __le32,
    pub next_ptr_h: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qtnf_pearl_fw_hdr {
    pub boardflg: [u8; 8],
    pub fwsize: __le32,
    pub seqnum: __le32,
    pub type: __le32,
    pub pktlen: __le32,
    pub crc: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qtnf_pcie_pearl_state {
    pub base: qtnf_pcie_bus_priv,
// lock for irq configuration changes
    pub irq_lock: spinlock_t,
    pub bda: *mut qtnf_pearl_bda __iomem,
    pub pcie_reg_base: *mut void __iomem,
    pub tx_bd_vbase: *mut qtnf_pearl_tx_bd,
    pub tx_bd_pbase: dma_addr_t,
    pub rx_bd_vbase: *mut qtnf_pearl_rx_bd,
    pub rx_bd_pbase: dma_addr_t,
    pub bd_table_paddr: dma_addr_t,
    pub bd_table_vaddr: *mut c_void,
    pub bd_table_len: u32,
    pub pcie_irq_mask: u32,
    pub pcie_irq_rx_count: u32,
    pub pcie_irq_tx_count: u32,
    pub pcie_irq_uf_count: u32,
}

#[no_mangle]
pub unsafe extern "C" fn qtnf_init_hdp_irqs(ps: *mut qtnf_pcie_pearl_state) {
    static inline void qtnf_init_hdp_irqs(struct qtnf_pcie_pearl_state *ps)
    {
    unsigned long flags;
    spin_lock_irqsave(&ps.irq_lock, flags);
    ps.pcie_irq_mask = (PCIE_HDP_INT_RX_BITS | PCIE_HDP_INT_TX_BITS);
    spin_unlock_irqrestore(&ps.irq_lock, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn qtnf_enable_hdp_irqs(ps: *mut qtnf_pcie_pearl_state) {
    static inline void qtnf_enable_hdp_irqs(struct qtnf_pcie_pearl_state *ps)
    {
    unsigned long flags;
    spin_lock_irqsave(&ps.irq_lock, flags);
    writel(ps.pcie_irq_mask, PCIE_HDP_INT_EN(ps.pcie_reg_base));
    spin_unlock_irqrestore(&ps.irq_lock, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn qtnf_disable_hdp_irqs(ps: *mut qtnf_pcie_pearl_state) {
    static inline void qtnf_disable_hdp_irqs(struct qtnf_pcie_pearl_state *ps)
    {
    unsigned long flags;
    spin_lock_irqsave(&ps.irq_lock, flags);
    writel(0x0, PCIE_HDP_INT_EN(ps.pcie_reg_base));
    spin_unlock_irqrestore(&ps.irq_lock, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn qtnf_en_rxdone_irq(ps: *mut qtnf_pcie_pearl_state) {
    static inline void qtnf_en_rxdone_irq(struct qtnf_pcie_pearl_state *ps)
    {
    unsigned long flags;
    spin_lock_irqsave(&ps.irq_lock, flags);
    ps.pcie_irq_mask |= PCIE_HDP_INT_RX_BITS;
    writel(ps.pcie_irq_mask, PCIE_HDP_INT_EN(ps.pcie_reg_base));
    spin_unlock_irqrestore(&ps.irq_lock, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn qtnf_dis_rxdone_irq(ps: *mut qtnf_pcie_pearl_state) {
    static inline void qtnf_dis_rxdone_irq(struct qtnf_pcie_pearl_state *ps)
    {
    unsigned long flags;
    spin_lock_irqsave(&ps.irq_lock, flags);
    ps.pcie_irq_mask &= ~PCIE_HDP_INT_RX_BITS;
    writel(ps.pcie_irq_mask, PCIE_HDP_INT_EN(ps.pcie_reg_base));
    spin_unlock_irqrestore(&ps.irq_lock, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn qtnf_en_txdone_irq(ps: *mut qtnf_pcie_pearl_state) {
    static inline void qtnf_en_txdone_irq(struct qtnf_pcie_pearl_state *ps)
    {
    unsigned long flags;
    spin_lock_irqsave(&ps.irq_lock, flags);
    ps.pcie_irq_mask |= PCIE_HDP_INT_TX_BITS;
    writel(ps.pcie_irq_mask, PCIE_HDP_INT_EN(ps.pcie_reg_base));
    spin_unlock_irqrestore(&ps.irq_lock, flags);
    }
#[no_mangle]
pub unsafe extern "C" fn qtnf_dis_txdone_irq(ps: *mut qtnf_pcie_pearl_state) {
    static inline void qtnf_dis_txdone_irq(struct qtnf_pcie_pearl_state *ps)
    {
    unsigned long flags;
    spin_lock_irqsave(&ps.irq_lock, flags);
    ps.pcie_irq_mask &= ~PCIE_HDP_INT_TX_BITS;
    writel(ps.pcie_irq_mask, PCIE_HDP_INT_EN(ps.pcie_reg_base));
    spin_unlock_irqrestore(&ps.irq_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn qtnf_deassert_intx(ps: *mut qtnf_pcie_pearl_state) {
    static void qtnf_deassert_intx(struct qtnf_pcie_pearl_state *ps)
    {
    void __iomem *reg = ps.base.sysctl_bar + PEARL_PCIE_CFG0_OFFSET;
    u32 cfg;
    cfg = readl(reg);
    cfg &= ~PEARL_ASSERT_INTX;
    qtnf_non_posted_write(cfg, reg);
    }
#[no_mangle]
unsafe extern "C" fn qtnf_pearl_reset_ep(ps: *mut qtnf_pcie_pearl_state) {
    static void qtnf_pearl_reset_ep(struct qtnf_pcie_pearl_state *ps)
    {
    let mut data: u32 = QTN_PEARL_IPC_IRQ_WORD(QTN_PEARL_LHOST_EP_RESET);
    void __iomem *reg = ps.base.sysctl_bar +
    QTN_PEARL_SYSCTL_LHOST_IRQ_OFFSET;
    qtnf_non_posted_write(data, reg);
    msleep(QTN_EP_RESET_WAIT_MS);
    pci_restore_state(ps.base.pdev);
    }
#[no_mangle]
unsafe extern "C" fn qtnf_pcie_pearl_ipc_gen_ep_int(arg: *mut c_void) {
    static void qtnf_pcie_pearl_ipc_gen_ep_int(void *arg)
    {
    const struct qtnf_pcie_pearl_state *ps = arg;
    let mut data: u32 = QTN_PEARL_IPC_IRQ_WORD(QTN_PEARL_LHOST_IPC_IRQ);
    void __iomem *reg = ps.base.sysctl_bar +
    QTN_PEARL_SYSCTL_LHOST_IRQ_OFFSET;
    qtnf_non_posted_write(data, reg);
    }
#[no_mangle]
unsafe extern "C" fn qtnf_is_state(reg: *mut __le32 __iomem, state: u32) -> c_int {
    static int qtnf_is_state(__le32 __iomem *reg, u32 state)
    {
    let mut s: u32 = readl(reg);
    return s & state;
    }
#[no_mangle]
unsafe extern "C" fn qtnf_set_state(reg: *mut __le32 __iomem, state: u32) {
    static void qtnf_set_state(__le32 __iomem *reg, u32 state)
    {
    let mut s: u32 = readl(reg);
    qtnf_non_posted_write(state | s, reg);
    }
#[no_mangle]
unsafe extern "C" fn qtnf_clear_state(reg: *mut __le32 __iomem, state: u32) {
    static void qtnf_clear_state(__le32 __iomem *reg, u32 state)
    {
    let mut s: u32 = readl(reg);
    qtnf_non_posted_write(s & ~state, reg);
    }
#[no_mangle]
unsafe extern "C" fn qtnf_poll_state(reg: *mut __le32 __iomem, state: u32, delay_in_ms: u32) -> c_int {
    static int qtnf_poll_state(__le32 __iomem *reg, u32 state, u32 delay_in_ms)
    {
    let mut timeout: u32 = 0;
    while ((qtnf_is_state(reg, state) == 0)) {
    usleep_range(1000, 1200);
    if (++timeout > delay_in_ms)
    return -1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pearl_alloc_bd_table(ps: *mut qtnf_pcie_pearl_state) -> c_int {
    static int pearl_alloc_bd_table(struct qtnf_pcie_pearl_state *ps)
    {
    struct qtnf_pcie_bus_priv *priv = &ps.base;
    dma_addr_t paddr;
    void *vaddr;
    int len;
    len = priv.tx_bd_num * sizeof(struct qtnf_pearl_tx_bd) +
    priv.rx_bd_num * sizeof(struct qtnf_pearl_rx_bd);
    vaddr = dmam_alloc_coherent(&priv.pdev.dev, len, &paddr, GFP_KERNEL);
    if (!vaddr)
    return -ENOMEM;
// tx bd
    ps.bd_table_vaddr = vaddr;
    ps.bd_table_paddr = paddr;
    ps.bd_table_len = len;
    ps.tx_bd_vbase = vaddr;
    ps.tx_bd_pbase = paddr;
    pr_debug("TX descriptor table: vaddr=0x%p paddr=%pad\n", vaddr, &paddr);
    priv.tx_bd_r_index = 0;
    priv.tx_bd_w_index = 0;
// rx bd
    vaddr = ((struct qtnf_pearl_tx_bd *)vaddr) + priv.tx_bd_num;
    paddr += priv.tx_bd_num * sizeof(struct qtnf_pearl_tx_bd);
    ps.rx_bd_vbase = vaddr;
    ps.rx_bd_pbase = paddr;

    writel(QTN_HOST_HI32(paddr),
    PCIE_HDP_TX_HOST_Q_BASE_H(ps.pcie_reg_base));

    writel(QTN_HOST_LO32(paddr),
    PCIE_HDP_TX_HOST_Q_BASE_L(ps.pcie_reg_base));
    writel(priv.rx_bd_num | (sizeof(struct qtnf_pearl_rx_bd)) << 16,
    PCIE_HDP_TX_HOST_Q_SZ_CTRL(ps.pcie_reg_base));
    pr_debug("RX descriptor table: vaddr=0x%p paddr=%pad\n", vaddr, &paddr);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pearl_skb2rbd_attach(ps: *mut qtnf_pcie_pearl_state, index: u16) -> c_int {
    static int pearl_skb2rbd_attach(struct qtnf_pcie_pearl_state *ps, u16 index)
    {
    struct qtnf_pcie_bus_priv *priv = &ps.base;
    struct qtnf_pearl_rx_bd *rxbd;
    struct sk_buff *skb;
    dma_addr_t paddr;
    skb = netdev_alloc_skb_ip_align(core::ptr::null_mut(), SKB_BUF_SIZE);
    if (!skb) {
    priv.rx_skb[index] = core::ptr::null_mut();
    return -ENOMEM;
    }
    priv.rx_skb[index] = skb;
    rxbd = &ps.rx_bd_vbase[index];
    paddr = dma_map_single(&priv.pdev.dev, skb.data, SKB_BUF_SIZE,
    DMA_FROM_DEVICE);
    if (dma_mapping_error(&priv.pdev.dev, paddr)) {
    pr_err("skb DMA mapping error: %pad\n", &paddr);
    return -ENOMEM;
    }
// keep rx skb paddrs in rx buffer descriptors for cleanup purposes
    rxbd.addr = cpu_to_le32(QTN_HOST_LO32(paddr));
    rxbd.addr_h = cpu_to_le32(QTN_HOST_HI32(paddr));
    rxbd.info = 0x0;
    priv.rx_bd_w_index = index;
// sync up all descriptor updates
    wmb();

    writel(QTN_HOST_HI32(paddr),
    PCIE_HDP_HHBM_BUF_PTR_H(ps.pcie_reg_base));

    writel(QTN_HOST_LO32(paddr),
    PCIE_HDP_HHBM_BUF_PTR(ps.pcie_reg_base));
    writel(index, PCIE_HDP_TX_HOST_Q_WR_PTR(ps.pcie_reg_base));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pearl_alloc_rx_buffers(ps: *mut qtnf_pcie_pearl_state) -> c_int {
    static int pearl_alloc_rx_buffers(struct qtnf_pcie_pearl_state *ps)
    {
    u16 i;
    let mut ret: c_int = 0;
    memset(ps.rx_bd_vbase, 0x0,
    ps.base.rx_bd_num * sizeof(struct qtnf_pearl_rx_bd));
    for (i = 0; i < ps.base.rx_bd_num; i++) {
    ret = pearl_skb2rbd_attach(ps, i);
    if (ret)
    break;
    }
    return ret;
    }
// all rx/tx activity should have ceased before calling this function
#[no_mangle]
unsafe extern "C" fn qtnf_pearl_free_xfer_buffers(ps: *mut qtnf_pcie_pearl_state) {
    static void qtnf_pearl_free_xfer_buffers(struct qtnf_pcie_pearl_state *ps)
    {
    struct qtnf_pcie_bus_priv *priv = &ps.base;
    struct qtnf_pearl_tx_bd *txbd;
    struct qtnf_pearl_rx_bd *rxbd;
    struct sk_buff *skb;
    dma_addr_t paddr;
    int i;
// free rx buffers
    for (i = 0; i < priv.rx_bd_num; i++) {
    if (priv.rx_skb && priv.rx_skb[i]) {
    rxbd = &ps.rx_bd_vbase[i];
    skb = priv.rx_skb[i];
    paddr = QTN_HOST_ADDR(le32_to_cpu(rxbd.addr_h),
    le32_to_cpu(rxbd.addr));
    dma_unmap_single(&priv.pdev.dev, paddr,
    SKB_BUF_SIZE, DMA_FROM_DEVICE);
    dev_kfree_skb_any(skb);
    priv.rx_skb[i] = core::ptr::null_mut();
    }
    }
// free tx buffers
    for (i = 0; i < priv.tx_bd_num; i++) {
    if (priv.tx_skb && priv.tx_skb[i]) {
    txbd = &ps.tx_bd_vbase[i];
    skb = priv.tx_skb[i];
    paddr = QTN_HOST_ADDR(le32_to_cpu(txbd.addr_h),
    le32_to_cpu(txbd.addr));
    dma_unmap_single(&priv.pdev.dev, paddr, skb.len,
    DMA_TO_DEVICE);
    dev_kfree_skb_any(skb);
    priv.tx_skb[i] = core::ptr::null_mut();
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn pearl_hhbm_init(ps: *mut qtnf_pcie_pearl_state) -> c_int {
    static int pearl_hhbm_init(struct qtnf_pcie_pearl_state *ps)
    {
    u32 val;
    val = readl(PCIE_HHBM_CONFIG(ps.pcie_reg_base));
    val |= HHBM_CONFIG_SOFT_RESET;
    writel(val, PCIE_HHBM_CONFIG(ps.pcie_reg_base));
    usleep_range(50, 100);
    val &= ~HHBM_CONFIG_SOFT_RESET;

    val |= HHBM_64BIT;

    writel(val, PCIE_HHBM_CONFIG(ps.pcie_reg_base));
    writel(ps.base.rx_bd_num, PCIE_HHBM_Q_LIMIT_REG(ps.pcie_reg_base));
    return 0;
    }
    static int qtnf_pcie_pearl_init_xfer(struct qtnf_pcie_pearl_state *ps,
    unsigned int tx_bd_size,
    unsigned int rx_bd_size)
    {
    struct qtnf_pcie_bus_priv *priv = &ps.base;
    int ret;
    u32 val;
    if (tx_bd_size == 0)
    tx_bd_size = PEARL_TX_BD_SIZE_DEFAULT;
    val = tx_bd_size * sizeof(struct qtnf_pearl_tx_bd);
    if (!is_power_of_2(tx_bd_size) || val > PCIE_HHBM_MAX_SIZE) {
    pr_warn("invalid tx_bd_size value %u, use default %u\n",
    tx_bd_size, PEARL_TX_BD_SIZE_DEFAULT);
    priv.tx_bd_num = PEARL_TX_BD_SIZE_DEFAULT;
    } else {
    priv.tx_bd_num = tx_bd_size;
    }
    if (rx_bd_size == 0)
    rx_bd_size = PEARL_RX_BD_SIZE_DEFAULT;
    val = rx_bd_size * sizeof(dma_addr_t);
    if (!is_power_of_2(rx_bd_size) || val > PCIE_HHBM_MAX_SIZE) {
    pr_warn("invalid rx_bd_size value %u, use default %u\n",
    rx_bd_size, PEARL_RX_BD_SIZE_DEFAULT);
    priv.rx_bd_num = PEARL_RX_BD_SIZE_DEFAULT;
    } else {
    priv.rx_bd_num = rx_bd_size;
    }
    priv.rx_bd_w_index = 0;
    priv.rx_bd_r_index = 0;
    ret = pearl_hhbm_init(ps);
    if (ret) {
    pr_err("failed to init h/w queues\n");
    return ret;
    }
    ret = qtnf_pcie_alloc_skb_array(priv);
    if (ret) {
    pr_err("failed to allocate skb array\n");
    return ret;
    }
    ret = pearl_alloc_bd_table(ps);
    if (ret) {
    pr_err("failed to allocate bd table\n");
    return ret;
    }
    ret = pearl_alloc_rx_buffers(ps);
    if (ret) {
    pr_err("failed to allocate rx buffers\n");
    return ret;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn qtnf_pearl_data_tx_reclaim(ps: *mut qtnf_pcie_pearl_state) {
    static void qtnf_pearl_data_tx_reclaim(struct qtnf_pcie_pearl_state *ps)
    {
    struct qtnf_pcie_bus_priv *priv = &ps.base;
    struct qtnf_pearl_tx_bd *txbd;
    struct sk_buff *skb;
    unsigned long flags;
    dma_addr_t paddr;
    u32 tx_done_index;
    let mut count: c_int = 0;
    int i;
    spin_lock_irqsave(&priv.tx_reclaim_lock, flags);
    tx_done_index = readl(PCIE_HDP_RX0DMA_CNT(ps.pcie_reg_base))
    & (priv.tx_bd_num - 1);
    i = priv.tx_bd_r_index;
    while (CIRC_CNT(tx_done_index, i, priv.tx_bd_num)) {
    skb = priv.tx_skb[i];
    if (likely(skb)) {
    txbd = &ps.tx_bd_vbase[i];
    paddr = QTN_HOST_ADDR(le32_to_cpu(txbd.addr_h),
    le32_to_cpu(txbd.addr));
    dma_unmap_single(&priv.pdev.dev, paddr, skb.len,
    DMA_TO_DEVICE);
    if (skb.dev) {
    dev_sw_netstats_tx_add(skb.dev, 1, skb.len);
    if (unlikely(priv.tx_stopped)) {
    qtnf_wake_all_queues(skb.dev);
    priv.tx_stopped = 0;
    }
    }
    dev_kfree_skb_any(skb);
    }
    priv.tx_skb[i] = core::ptr::null_mut();
    count++;
    if (++i >= priv.tx_bd_num)
    i = 0;
    }
    priv.tx_reclaim_done += count;
    priv.tx_reclaim_req++;
    priv.tx_bd_r_index = i;
    spin_unlock_irqrestore(&priv.tx_reclaim_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn qtnf_tx_queue_ready(ps: *mut qtnf_pcie_pearl_state) -> c_int {
    static int qtnf_tx_queue_ready(struct qtnf_pcie_pearl_state *ps)
    {
    struct qtnf_pcie_bus_priv *priv = &ps.base;
    if (!CIRC_SPACE(priv.tx_bd_w_index, priv.tx_bd_r_index,
    priv.tx_bd_num)) {
    qtnf_pearl_data_tx_reclaim(ps);
    if (!CIRC_SPACE(priv.tx_bd_w_index, priv.tx_bd_r_index,
    priv.tx_bd_num)) {
    pr_warn_ratelimited("reclaim full Tx queue\n");
    priv.tx_full_count++;
    return 0;
    }
    }
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn qtnf_pcie_skb_send(bus: *mut qtnf_bus, skb: *mut sk_buff) -> c_int {
    static int qtnf_pcie_skb_send(struct qtnf_bus *bus, struct sk_buff *skb)
    {
    struct qtnf_pcie_pearl_state *ps = get_bus_priv(bus);
    struct qtnf_pcie_bus_priv *priv = &ps.base;
    dma_addr_t txbd_paddr, skb_paddr;
    struct qtnf_pearl_tx_bd *txbd;
    unsigned long flags;
    int len, i;
    u32 info;
    let mut ret: c_int = 0;
    spin_lock_irqsave(&priv.tx_lock, flags);
    if (!qtnf_tx_queue_ready(ps)) {
    if (skb.dev) {
    netif_tx_stop_all_queues(skb.dev);
    priv.tx_stopped = 1;
    }
    spin_unlock_irqrestore(&priv.tx_lock, flags);
    return NETDEV_TX_BUSY;
    }
    i = priv.tx_bd_w_index;
    priv.tx_skb[i] = skb;
    len = skb.len;
    skb_paddr = dma_map_single(&priv.pdev.dev, skb.data, skb.len,
    DMA_TO_DEVICE);
    if (dma_mapping_error(&priv.pdev.dev, skb_paddr)) {
    pr_err("skb DMA mapping error: %pad\n", &skb_paddr);
    ret = -ENOMEM;
    goto tx_done;
    }
    txbd = &ps.tx_bd_vbase[i];
    txbd.addr = cpu_to_le32(QTN_HOST_LO32(skb_paddr));
    txbd.addr_h = cpu_to_le32(QTN_HOST_HI32(skb_paddr));
    info = (len & QTN_PCIE_TX_DESC_LEN_MASK) << QTN_PCIE_TX_DESC_LEN_SHIFT;
    txbd.info = cpu_to_le32(info);
// sync up all descriptor updates before passing them to EP
    dma_wmb();
// write new TX descriptor to PCIE_RX_FIFO on EP
    txbd_paddr = ps.tx_bd_pbase + i * sizeof(struct qtnf_pearl_tx_bd);

    writel(QTN_HOST_HI32(txbd_paddr),
    PCIE_HDP_HOST_WR_DESC0_H(ps.pcie_reg_base));

    writel(QTN_HOST_LO32(txbd_paddr),
    PCIE_HDP_HOST_WR_DESC0(ps.pcie_reg_base));
    if (++i >= priv.tx_bd_num)
    i = 0;
    priv.tx_bd_w_index = i;
    tx_done:
    if (ret) {
    pr_err_ratelimited("drop skb\n");
    if (skb.dev)
    skb.dev.stats.tx_dropped++;
    dev_kfree_skb_any(skb);
    }
    priv.tx_done_count++;
    spin_unlock_irqrestore(&priv.tx_lock, flags);
    qtnf_pearl_data_tx_reclaim(ps);
    return NETDEV_TX_OK;
    }
    static int qtnf_pcie_data_tx(struct qtnf_bus *bus, struct sk_buff *skb,
    unsigned int macid, unsigned int vifid)
    {
    return qtnf_pcie_skb_send(bus, skb);
    }
    static int qtnf_pcie_data_tx_meta(struct qtnf_bus *bus, struct sk_buff *skb,
    unsigned int macid, unsigned int vifid)
    {
    struct qtnf_frame_meta_info *meta;
    let mut tail_need: c_int = sizeof(*meta) - skb_tailroom(skb);
    int ret;
    if (tail_need > 0 && pskb_expand_head(skb, 0, tail_need, GFP_ATOMIC)) {
    skb.dev.stats.tx_dropped++;
    dev_kfree_skb_any(skb);
    return NETDEV_TX_OK;
    }
    meta = skb_put(skb, sizeof(*meta));
    meta.magic_s = HBM_FRAME_META_MAGIC_PATTERN_S;
    meta.magic_e = HBM_FRAME_META_MAGIC_PATTERN_E;
    meta.macid = macid;
    meta.ifidx = vifid;
    ret = qtnf_pcie_skb_send(bus, skb);
    if (unlikely(ret == NETDEV_TX_BUSY))
    __skb_trim(skb, skb.len - sizeof(*meta));
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn qtnf_pcie_pearl_interrupt(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t qtnf_pcie_pearl_interrupt(int irq, void *data)
    {
    struct qtnf_bus *bus = (struct qtnf_bus *)data;
    struct qtnf_pcie_pearl_state *ps = get_bus_priv(bus);
    struct qtnf_pcie_bus_priv *priv = &ps.base;
    u32 status;
    priv.pcie_irq_count++;
    status = readl(PCIE_HDP_INT_STATUS(ps.pcie_reg_base));
    qtnf_shm_ipc_irq_handler(&priv.shm_ipc_ep_in);
    qtnf_shm_ipc_irq_handler(&priv.shm_ipc_ep_out);
    if (!(status & ps.pcie_irq_mask))
    goto irq_done;
    if (status & PCIE_HDP_INT_RX_BITS)
    ps.pcie_irq_rx_count++;
    if (status & PCIE_HDP_INT_TX_BITS)
    ps.pcie_irq_tx_count++;
    if (status & PCIE_HDP_INT_HHBM_UF)
    ps.pcie_irq_uf_count++;
    if (status & PCIE_HDP_INT_RX_BITS) {
    qtnf_dis_rxdone_irq(ps);
    napi_schedule(&bus.mux_napi);
    }
    if (status & PCIE_HDP_INT_TX_BITS) {
    qtnf_dis_txdone_irq(ps);
    tasklet_hi_schedule(&priv.reclaim_tq);
    }
    irq_done:
// H/W workaround: clean all bits, not only enabled
    qtnf_non_posted_write(~0U, PCIE_HDP_INT_STATUS(ps.pcie_reg_base));
    if (!priv.msi_enabled)
    qtnf_deassert_intx(ps);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn qtnf_rx_data_ready(ps: *mut qtnf_pcie_pearl_state) -> c_int {
    static int qtnf_rx_data_ready(struct qtnf_pcie_pearl_state *ps)
    {
    let mut index: u16 = ps.base.rx_bd_r_index;
    struct qtnf_pearl_rx_bd *rxbd;
    u32 descw;
    rxbd = &ps.rx_bd_vbase[index];
    descw = le32_to_cpu(rxbd.info);
    if (descw & QTN_TXDONE_MASK)
    return 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qtnf_pcie_pearl_rx_poll(napi: *mut napi_struct, budget: c_int) -> c_int {
    static int qtnf_pcie_pearl_rx_poll(struct napi_struct *napi, int budget)
    {
    struct qtnf_bus *bus = container_of(napi, struct qtnf_bus, mux_napi);
    struct qtnf_pcie_pearl_state *ps = get_bus_priv(bus);
    struct qtnf_pcie_bus_priv *priv = &ps.base;
    struct net_device *ndev = core::ptr::null_mut();
    struct sk_buff *skb = core::ptr::null_mut();
    let mut processed: c_int = 0;
    struct qtnf_pearl_rx_bd *rxbd;
    dma_addr_t skb_paddr;
    int consume;
    u32 descw;
    u32 psize;
    u16 r_idx;
    u16 w_idx;
    int ret;
    while (processed < budget) {
    if (!qtnf_rx_data_ready(ps))
    goto rx_out;
    r_idx = priv.rx_bd_r_index;
    rxbd = &ps.rx_bd_vbase[r_idx];
    descw = le32_to_cpu(rxbd.info);
    skb = priv.rx_skb[r_idx];
    psize = QTN_GET_LEN(descw);
    consume = 1;
    if (!(descw & QTN_TXDONE_MASK)) {
    pr_warn("skip invalid rxbd[%d]\n", r_idx);
    consume = 0;
    }
    if (!skb) {
    pr_warn("skip missing rx_skb[%d]\n", r_idx);
    consume = 0;
    }
    if (skb && (skb_tailroom(skb) <  psize)) {
    pr_err("skip packet with invalid length: %u > %u\n",
    psize, skb_tailroom(skb));
    consume = 0;
    }
    if (skb) {
    skb_paddr = QTN_HOST_ADDR(le32_to_cpu(rxbd.addr_h),
    le32_to_cpu(rxbd.addr));
    dma_unmap_single(&priv.pdev.dev, skb_paddr,
    SKB_BUF_SIZE, DMA_FROM_DEVICE);
    }
    if (consume) {
    skb_put(skb, psize);
    ndev = qtnf_classify_skb(bus, skb);
    if (likely(ndev)) {
    dev_sw_netstats_rx_add(ndev, skb.len);
    skb.protocol = eth_type_trans(skb, ndev);
    napi_gro_receive(napi, skb);
    } else {
    pr_debug("drop untagged skb\n");
    bus.mux_dev.stats.rx_dropped++;
    dev_kfree_skb_any(skb);
    }
    } else {
    if (skb) {
    bus.mux_dev.stats.rx_dropped++;
    dev_kfree_skb_any(skb);
    }
    }
    priv.rx_skb[r_idx] = core::ptr::null_mut();
    if (++r_idx >= priv.rx_bd_num)
    r_idx = 0;
    priv.rx_bd_r_index = r_idx;
// repalce processed buffer by a new one
    w_idx = priv.rx_bd_w_index;
    while (CIRC_SPACE(priv.rx_bd_w_index, priv.rx_bd_r_index,
    priv.rx_bd_num) > 0) {
    if (++w_idx >= priv.rx_bd_num)
    w_idx = 0;
    ret = pearl_skb2rbd_attach(ps, w_idx);
    if (ret) {
    pr_err("failed to allocate new rx_skb[%d]\n",
    w_idx);
    break;
    }
    }
    processed++;
    }
    rx_out:
    if (processed < budget) {
    napi_complete(napi);
    qtnf_en_rxdone_irq(ps);
    }
    return processed;
    }
    static void
    qtnf_pcie_data_tx_timeout(struct qtnf_bus *bus, struct net_device *ndev)
    {
    struct qtnf_pcie_pearl_state *ps = (void *)get_bus_priv(bus);
    tasklet_hi_schedule(&ps.base.reclaim_tq);
    }
#[no_mangle]
unsafe extern "C" fn qtnf_pcie_data_rx_start(bus: *mut qtnf_bus) {
    static void qtnf_pcie_data_rx_start(struct qtnf_bus *bus)
    {
    struct qtnf_pcie_pearl_state *ps = (void *)get_bus_priv(bus);
    qtnf_enable_hdp_irqs(ps);
    napi_enable(&bus.mux_napi);
    }
#[no_mangle]
unsafe extern "C" fn qtnf_pcie_data_rx_stop(bus: *mut qtnf_bus) {
    static void qtnf_pcie_data_rx_stop(struct qtnf_bus *bus)
    {
    struct qtnf_pcie_pearl_state *ps = (void *)get_bus_priv(bus);
    napi_disable(&bus.mux_napi);
    qtnf_disable_hdp_irqs(ps);
    }
#[no_mangle]
unsafe extern "C" fn qtnf_pearl_tx_use_meta_info_set(bus: *mut qtnf_bus, use_meta: bool) {
    static void qtnf_pearl_tx_use_meta_info_set(struct qtnf_bus *bus, bool use_meta)
    {
    if (use_meta)
    bus.bus_ops.data_tx = qtnf_pcie_data_tx_meta;
    else
    bus.bus_ops.data_tx = qtnf_pcie_data_tx;
    }
    static struct qtnf_bus_ops qtnf_pcie_pearl_bus_ops = {
// control path methods
    .control_tx	= qtnf_pcie_control_tx,
// data path methods
    .data_tx		= qtnf_pcie_data_tx,
    .data_tx_timeout	= qtnf_pcie_data_tx_timeout,
    .data_tx_use_meta_set	= qtnf_pearl_tx_use_meta_info_set,
    .data_rx_start		= qtnf_pcie_data_rx_start,
    .data_rx_stop		= qtnf_pcie_data_rx_stop,
    };
#[no_mangle]
unsafe extern "C" fn qtnf_dbg_irq_stats(s: *mut seq_file, data: *mut c_void) -> c_int {
    static int qtnf_dbg_irq_stats(struct seq_file *s, void *data)
    {
    struct qtnf_bus *bus = dev_get_drvdata(s.private);
    struct qtnf_pcie_pearl_state *ps = get_bus_priv(bus);
    let mut reg: u32 = readl(PCIE_HDP_INT_EN(ps.pcie_reg_base));
    u32 status;
    seq_printf(s, "pcie_irq_count(%u)\n", ps.base.pcie_irq_count);
    seq_printf(s, "pcie_irq_tx_count(%u)\n", ps.pcie_irq_tx_count);
    status = reg &  PCIE_HDP_INT_TX_BITS;
    seq_printf(s, "pcie_irq_tx_status(%s)\n",
    (status == PCIE_HDP_INT_TX_BITS) ? "EN" : "DIS");
    seq_printf(s, "pcie_irq_rx_count(%u)\n", ps.pcie_irq_rx_count);
    status = reg &  PCIE_HDP_INT_RX_BITS;
    seq_printf(s, "pcie_irq_rx_status(%s)\n",
    (status == PCIE_HDP_INT_RX_BITS) ? "EN" : "DIS");
    seq_printf(s, "pcie_irq_uf_count(%u)\n", ps.pcie_irq_uf_count);
    status = reg &  PCIE_HDP_INT_HHBM_UF;
    seq_printf(s, "pcie_irq_hhbm_uf_status(%s)\n",
    (status == PCIE_HDP_INT_HHBM_UF) ? "EN" : "DIS");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qtnf_dbg_hdp_stats(s: *mut seq_file, data: *mut c_void) -> c_int {
    static int qtnf_dbg_hdp_stats(struct seq_file *s, void *data)
    {
    struct qtnf_bus *bus = dev_get_drvdata(s.private);
    struct qtnf_pcie_pearl_state *ps = get_bus_priv(bus);
    struct qtnf_pcie_bus_priv *priv = &ps.base;
    seq_printf(s, "tx_full_count(%u)\n", priv.tx_full_count);
    seq_printf(s, "tx_done_count(%u)\n", priv.tx_done_count);
    seq_printf(s, "tx_reclaim_done(%u)\n", priv.tx_reclaim_done);
    seq_printf(s, "tx_reclaim_req(%u)\n", priv.tx_reclaim_req);
    seq_printf(s, "tx_bd_r_index(%u)\n", priv.tx_bd_r_index);
    seq_printf(s, "tx_bd_p_index(%u)\n",
    readl(PCIE_HDP_RX0DMA_CNT(ps.pcie_reg_base))
    & (priv.tx_bd_num - 1));
    seq_printf(s, "tx_bd_w_index(%u)\n", priv.tx_bd_w_index);
    seq_printf(s, "tx queue len(%u)\n",
    CIRC_CNT(priv.tx_bd_w_index, priv.tx_bd_r_index,
    priv.tx_bd_num));
    seq_printf(s, "rx_bd_r_index(%u)\n", priv.rx_bd_r_index);
    seq_printf(s, "rx_bd_p_index(%u)\n",
    readl(PCIE_HDP_TX0DMA_CNT(ps.pcie_reg_base))
    & (priv.rx_bd_num - 1));
    seq_printf(s, "rx_bd_w_index(%u)\n", priv.rx_bd_w_index);
    seq_printf(s, "rx alloc queue len(%u)\n",
    CIRC_SPACE(priv.rx_bd_w_index, priv.rx_bd_r_index,
    priv.rx_bd_num));
    return 0;
    }
    static int qtnf_ep_fw_send(struct pci_dev *pdev, uint32_t size,
    int blk, const u8 *pblk, const u8 *fw)
    {
    struct qtnf_bus *bus = pci_get_drvdata(pdev);
    struct qtnf_pearl_fw_hdr *hdr;
    u8 *pdata;
    let mut hds: c_int = sizeof(*hdr);
    struct sk_buff *skb = core::ptr::null_mut();
    let mut len: c_int = 0;
    int ret;
    skb = __dev_alloc_skb(QTN_PCIE_FW_BUFSZ, GFP_KERNEL);
    if (!skb)
    return -ENOMEM;
    skb.len = QTN_PCIE_FW_BUFSZ;
    skb.dev = core::ptr::null_mut();
    hdr = (struct qtnf_pearl_fw_hdr *)skb.data;
    memcpy(hdr.boardflg, QTN_PCIE_BOARDFLG, strlen(QTN_PCIE_BOARDFLG));
    hdr.fwsize = cpu_to_le32(size);
    hdr.seqnum = cpu_to_le32(blk);
    if (blk)
    hdr.type = cpu_to_le32(QTN_FW_DSUB);
    else
    hdr.type = cpu_to_le32(QTN_FW_DBEGIN);
    pdata = skb.data + hds;
    len = QTN_PCIE_FW_BUFSZ - hds;
    if (pblk >= (fw + size - len)) {
    len = fw + size - pblk;
    hdr.type = cpu_to_le32(QTN_FW_DEND);
    }
    hdr.pktlen = cpu_to_le32(len);
    memcpy(pdata, pblk, len);
    hdr.crc = cpu_to_le32(~crc32(0, pdata, len));
    ret = qtnf_pcie_skb_send(bus, skb);
    return (ret == NETDEV_TX_OK) ? len : 0;
    }
    static int
    qtnf_ep_fw_load(struct qtnf_pcie_pearl_state *ps, const u8 *fw, u32 fw_size)
    {
    let mut blk_size: c_int = QTN_PCIE_FW_BUFSZ - sizeof(struct qtnf_pearl_fw_hdr);
    let mut blk_count: c_int = fw_size / blk_size + ((fw_size % blk_size) ? 1 : 0);
    const u8 *pblk = fw;
    let mut threshold: c_int = 0;
    let mut blk: c_int = 0;
    int len;
    pr_debug("FW upload started: fw_addr=0x%p size=%d\n", fw, fw_size);
    while (blk < blk_count) {
    if (++threshold > 10000) {
    pr_err("FW upload failed: too many retries\n");
    return -ETIMEDOUT;
    }
    len = qtnf_ep_fw_send(ps.base.pdev, fw_size, blk, pblk, fw);
    if (len <= 0)
    continue;
    if (!((blk + 1) & QTN_PCIE_FW_DLMASK) ||
    (blk == (blk_count - 1))) {
    qtnf_set_state(&ps.bda.bda_rc_state,
    QTN_RC_FW_SYNC);
    if (qtnf_poll_state(&ps.bda.bda_ep_state,
    QTN_EP_FW_SYNC,
    QTN_FW_DL_TIMEOUT_MS)) {
    pr_err("FW upload failed: SYNC timed out\n");
    return -ETIMEDOUT;
    }
    qtnf_clear_state(&ps.bda.bda_ep_state,
    QTN_EP_FW_SYNC);
    if (qtnf_is_state(&ps.bda.bda_ep_state,
    QTN_EP_FW_RETRY)) {
    if (blk == (blk_count - 1)) {
    int last_round =
    blk_count & QTN_PCIE_FW_DLMASK;
    blk -= last_round;
    pblk -= ((last_round - 1) *
    blk_size + len);
    } else {
    blk -= QTN_PCIE_FW_DLMASK;
    pblk -= QTN_PCIE_FW_DLMASK * blk_size;
    }
    qtnf_clear_state(&ps.bda.bda_ep_state,
    QTN_EP_FW_RETRY);
    pr_warn("FW upload retry: block #%d\n", blk);
    continue;
    }
    qtnf_pearl_data_tx_reclaim(ps);
    }
    pblk += len;
    blk++;
    }
    pr_debug("FW upload completed: totally sent %d blocks\n", blk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qtnf_pearl_fw_work_handler(work: *mut work_struct) {
    static void qtnf_pearl_fw_work_handler(struct work_struct *work)
    {
    struct qtnf_bus *bus = container_of(work, struct qtnf_bus, fw_work);
    struct qtnf_pcie_pearl_state *ps = (void *)get_bus_priv(bus);
    let mut state: u32 = QTN_RC_FW_LOADRDY | QTN_RC_FW_QLINK;
    const char *fwname = QTN_PCI_PEARL_FW_NAME;
    struct pci_dev *pdev = ps.base.pdev;
    const struct firmware *fw;
    int ret;
    if (ps.base.flashboot) {
    state |= QTN_RC_FW_FLASHBOOT;
    } else {
    ret = request_firmware(&fw, fwname, &pdev.dev);
    if (ret < 0) {
    pr_err("failed to get firmware %s\n", fwname);
    goto fw_load_exit;
    }
    }
    qtnf_set_state(&ps.bda.bda_rc_state, state);
    if (qtnf_poll_state(&ps.bda.bda_ep_state, QTN_EP_FW_LOADRDY,
    QTN_FW_DL_TIMEOUT_MS)) {
    pr_err("card is not ready\n");
    if (!ps.base.flashboot)
    release_firmware(fw);
    goto fw_load_exit;
    }
    qtnf_clear_state(&ps.bda.bda_ep_state, QTN_EP_FW_LOADRDY);
    if (ps.base.flashboot) {
    pr_info("booting firmware from flash\n");
    } else {
    pr_info("starting firmware upload: %s\n", fwname);
    ret = qtnf_ep_fw_load(ps, fw.data, fw.size);
    release_firmware(fw);
    if (ret) {
    pr_err("firmware upload error\n");
    goto fw_load_exit;
    }
    }
    if (qtnf_poll_state(&ps.bda.bda_ep_state, QTN_EP_FW_DONE,
    QTN_FW_DL_TIMEOUT_MS)) {
    pr_err("firmware bringup timed out\n");
    goto fw_load_exit;
    }
    if (qtnf_poll_state(&ps.bda.bda_ep_state,
    QTN_EP_FW_QLINK_DONE, QTN_FW_QLINK_TIMEOUT_MS)) {
    pr_err("firmware runtime failure\n");
    goto fw_load_exit;
    }
    pr_info("firmware is up and running\n");
    ret = qtnf_pcie_fw_boot_done(bus);
    if (ret)
    goto fw_load_exit;
    qtnf_debugfs_add_entry(bus, "hdp_stats", qtnf_dbg_hdp_stats);
    qtnf_debugfs_add_entry(bus, "irq_stats", qtnf_dbg_irq_stats);
    fw_load_exit:
    put_device(&pdev.dev);
    }
#[no_mangle]
unsafe extern "C" fn qtnf_pearl_reclaim_tasklet_fn(t: *mut tasklet_struct) {
    static void qtnf_pearl_reclaim_tasklet_fn(struct tasklet_struct *t)
    {
    struct qtnf_pcie_pearl_state *ps = from_tasklet(ps, t, base.reclaim_tq);
    qtnf_pearl_data_tx_reclaim(ps);
    qtnf_en_txdone_irq(ps);
    }
#[no_mangle]
unsafe extern "C" fn qtnf_pearl_dma_mask_get() -> u64 {
    static u64 qtnf_pearl_dma_mask_get(void)
    {

    return DMA_BIT_MASK(64);

    return DMA_BIT_MASK(32);

    }
    static int qtnf_pcie_pearl_probe(struct qtnf_bus *bus, unsigned int tx_bd_size,
    unsigned int rx_bd_size)
    {
    struct qtnf_shm_ipc_int ipc_int;
    struct qtnf_pcie_pearl_state *ps = get_bus_priv(bus);
    struct pci_dev *pdev = ps.base.pdev;
    int ret;
    bus.bus_ops = &qtnf_pcie_pearl_bus_ops;
    spin_lock_init(&ps.irq_lock);
    INIT_WORK(&bus.fw_work, qtnf_pearl_fw_work_handler);
    ps.pcie_reg_base = ps.base.dmareg_bar;
    ps.bda = ps.base.epmem_bar;
    writel(ps.base.msi_enabled, &ps.bda.bda_rc_msi_enabled);
    ret = qtnf_pcie_pearl_init_xfer(ps, tx_bd_size, rx_bd_size);
    if (ret) {
    pr_err("PCIE xfer init failed\n");
    return ret;
    }
// init default irq settings
    qtnf_init_hdp_irqs(ps);
// start with disabled irqs
    qtnf_disable_hdp_irqs(ps);
    ret = devm_request_irq(&pdev.dev, pdev.irq,
    &qtnf_pcie_pearl_interrupt, 0,
    "qtnf_pearl_irq", (void *)bus);
    if (ret) {
    pr_err("failed to request pcie irq %d\n", pdev.irq);
    qtnf_pearl_free_xfer_buffers(ps);
    return ret;
    }
    tasklet_setup(&ps.base.reclaim_tq, qtnf_pearl_reclaim_tasklet_fn);
    netif_napi_add_weight(bus.mux_dev, &bus.mux_napi,
    qtnf_pcie_pearl_rx_poll, 10);
    ipc_int.fn = qtnf_pcie_pearl_ipc_gen_ep_int;
    ipc_int.arg = ps;
    qtnf_pcie_init_shm_ipc(&ps.base, &ps.bda.bda_shm_reg1,
    &ps.bda.bda_shm_reg2, &ipc_int);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn qtnf_pcie_pearl_remove(bus: *mut qtnf_bus) {
    static void qtnf_pcie_pearl_remove(struct qtnf_bus *bus)
    {
    struct qtnf_pcie_pearl_state *ps = get_bus_priv(bus);
    qtnf_pearl_reset_ep(ps);
    qtnf_pearl_free_xfer_buffers(ps);
    }

#[no_mangle]
unsafe extern "C" fn qtnf_pcie_pearl_suspend(bus: *mut qtnf_bus) -> c_int {
    static int qtnf_pcie_pearl_suspend(struct qtnf_bus *bus)
    {
    return -EOPNOTSUPP;
    }
#[no_mangle]
unsafe extern "C" fn qtnf_pcie_pearl_resume(bus: *mut qtnf_bus) -> c_int {
    static int qtnf_pcie_pearl_resume(struct qtnf_bus *bus)
    {
    return 0;
    }

    struct qtnf_bus *qtnf_pcie_pearl_alloc(struct pci_dev *pdev)
    {
    struct qtnf_bus *bus;
    struct qtnf_pcie_pearl_state *ps;
    bus = devm_kzalloc(&pdev.dev, sizeof(*bus) + sizeof(*ps), GFP_KERNEL);
    if (!bus)
    return core::ptr::null_mut();
    ps = get_bus_priv(bus);
    ps.base.probe_cb = qtnf_pcie_pearl_probe;
    ps.base.remove_cb = qtnf_pcie_pearl_remove;
    ps.base.dma_mask_get_cb = qtnf_pearl_dma_mask_get;

    ps.base.resume_cb = qtnf_pcie_pearl_resume;
    ps.base.suspend_cb = qtnf_pcie_pearl_suspend;

    return bus;
    }
