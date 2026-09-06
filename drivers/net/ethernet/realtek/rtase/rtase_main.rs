//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/realtek/rtase/rtase_main.c
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// rtase is the Linux device driver released for Realtek Automotive Switch
// controllers with PCI-Express interface.
//
// Copyright(c) 2024 Realtek Semiconductor Corp.
//
// Below is a simplified block diagram of the chip and its relevant interfaces.
//
// *
// *  CPU network device
// *
// *   +-------------+
// *   |  PCIE Host  |
// ***********++
// ||
// PCIE
// ||
// ********************++
// *            | PCIE Endpoint |
// *            +---------------+
// *                | GMAC |
// *                +--++--+  Realtek
// *                   ||     RTL90xx Series
// *                   ||
// *     +-------------++----------------+
// *     |           | MAC |             |
// *     |           +-----+             |
// *     |                               |
// *     |     Ethernet Switch Core      |
// *     |                               |
// *     |   +-----+           +-----+   |
// *     |   | MAC |...........| MAC |   |
// *     +---+-----+-----------+-----+---+
// *         | PHY |...........| PHY |
// *         +--++-+           +--++-+
// *************||****************||
//
// The block of the Realtek RTL90xx series is our entire chip architecture,
// the GMAC is connected to the switch core, and there is no PHY in between.
// In addition, this driver is mainly used to control GMAC, but does not
// control the switch core, so it is not the same as DSA. Linux only plays
// the role of a normal leaf node in this model.
//

pub const RTK_OPTS1_DEBUG_VALUE: c_uint = 0x0BADBEEF;
pub const RTK_MAGIC_NUMBER: c_uint = 0x0BADBADBADBADBAD;
    static const struct pci_device_id rtase_pci_tbl[] = {
    {PCI_VDEVICE(REALTEK, 0x906A)},
    {}
    };
    MODULE_DEVICE_TABLE(pci, rtase_pci_tbl);
    MODULE_AUTHOR("Realtek ARD Software Team");
    MODULE_DESCRIPTION("Network Driver for the PCIe interface of Realtek Automotive Ethernet Switch");
    MODULE_LICENSE("Dual BSD/GPL");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtase_counters {
    pub tx_packets: __le64,
    pub rx_packets: __le64,
    pub tx_errors: __le64,
    pub rx_errors: __le32,
    pub rx_missed: __le16,
    pub align_errors: __le16,
    pub tx_one_collision: __le32,
    pub tx_multi_collision: __le32,
    pub rx_unicast: __le64,
    pub rx_broadcast: __le64,
    pub rx_multicast: __le32,
    pub tx_aborted: __le16,
    pub tx_underrun: __le16,
    pub __packed: },
#[no_mangle]
unsafe extern "C" fn rtase_w8(tp: *const rtase_private, reg: u16, val8: u8) {
    static void rtase_w8(const struct rtase_private *tp, u16 reg, u8 val8)
    {
    pub reg): writeb(val8, tp->mmio_addr +,
    }
#[no_mangle]
unsafe extern "C" fn rtase_w16(tp: *const rtase_private, reg: u16, val16: u16) {
    static void rtase_w16(const struct rtase_private *tp, u16 reg, u16 val16)
    {
    pub reg): writew(val16, tp->mmio_addr +,
    }
#[no_mangle]
unsafe extern "C" fn rtase_w32(tp: *const rtase_private, reg: u16, val32: u32) {
    static void rtase_w32(const struct rtase_private *tp, u16 reg, u32 val32)
    {
    pub reg): writel(val32, tp->mmio_addr +,
    }
#[no_mangle]
unsafe extern "C" fn rtase_r8(tp: *const rtase_private, reg: u16) -> u8 {
    static u8 rtase_r8(const struct rtase_private *tp, u16 reg)
    {
    pub reg): return readb(tp->mmio_addr +,
    }
#[no_mangle]
unsafe extern "C" fn rtase_r16(tp: *const rtase_private, reg: u16) -> u16 {
    static u16 rtase_r16(const struct rtase_private *tp, u16 reg)
    {
    pub reg): return readw(tp->mmio_addr +,
    }
#[no_mangle]
unsafe extern "C" fn rtase_r32(tp: *const rtase_private, reg: u16) -> u32 {
    static u32 rtase_r32(const struct rtase_private *tp, u16 reg)
    {
    pub reg): return readl(tp->mmio_addr +,
    }
#[no_mangle]
unsafe extern "C" fn rtase_free_desc(tp: *mut rtase_private) {
    static void rtase_free_desc(struct rtase_private *tp)
    {
    pub tp->pdev: *mut *mut pci_dev pdev =,
    pub i: u32,
    pub {: for (i = 0; i < tp->func_tx_queue_num; i++),
    if (!tp.tx_ring[i].desc)
    dma_free_coherent(&pdev.dev, RTASE_TX_RING_DESC_SIZE,
    tp.tx_ring[i].desc,
    pub NULL: tp->tx_ring[i].desc =,
    }
    pub {: for (i = 0; i < tp->func_rx_queue_num; i++),
    if (!tp.rx_ring[i].desc)
    dma_free_coherent(&pdev.dev, RTASE_RX_RING_DESC_SIZE,
    tp.rx_ring[i].desc,
    pub NULL: tp->rx_ring[i].desc =,
    }
    }
#[no_mangle]
unsafe extern "C" fn rtase_alloc_desc(tp: *mut rtase_private) -> c_int {
    static int rtase_alloc_desc(struct rtase_private *tp)
    {
    pub tp->pdev: *mut *mut pci_dev pdev =,
    pub i: u32,
// rx and tx descriptors needs 256 bytes alignment.
// dma_alloc_coherent provides more.
//
    pub {: for (i = 0; i < tp->func_tx_queue_num; i++),
    tp.tx_ring[i].desc =
    dma_alloc_coherent(&pdev.dev,
    RTASE_TX_RING_DESC_SIZE,
    &tp.tx_ring[i].phy_addr,
    if (!tp.tx_ring[i].desc)
    pub err_out: goto,
    }
    pub {: for (i = 0; i < tp->func_rx_queue_num; i++),
    tp.rx_ring[i].desc =
    dma_alloc_coherent(&pdev.dev,
    RTASE_RX_RING_DESC_SIZE,
    &tp.rx_ring[i].phy_addr,
    if (!tp.rx_ring[i].desc)
    pub err_out: goto,
    }
    pub 0: return,
    err_out:
    pub -ENOMEM: return,
    }
    static void rtase_unmap_tx_skb(struct pci_dev *pdev, u32 len,
    struct rtase_tx_desc *desc)
    {
    dma_unmap_single(&pdev.dev, le64_to_cpu(desc.addr), len,
    pub cpu_to_le32(RTK_OPTS1_DEBUG_VALUE): desc->opts1 =,
    pub 0x00: desc->opts2 =,
    pub cpu_to_le64(RTK_MAGIC_NUMBER): desc->addr =,
    }
#[no_mangle]
unsafe extern "C" fn rtase_tx_clear_range(ring: *mut rtase_ring, start: u32, n: u32) {
    static void rtase_tx_clear_range(struct rtase_ring *ring, u32 start, u32 n)
    {
    pub ring->desc: *mut *mut rtase_tx_desc desc_base =,
    pub ring->ivec->tp: *mut *mut rtase_private tp =,
    pub i: u32,
    pub {: for (i = 0; i < n; i++),
    pub RTASE_NUM_DESC: u32 entry = (start + i) %,
    pub entry: *mut *mut rtase_tx_desc desc = desc_base +,
    pub ring->mis.len[entry]: u32 len =,
    pub skb: *mut sk_buff,
    if (len == 0)
    pub desc): rtase_unmap_tx_skb(tp->pdev, len,,
    pub 0: ring->mis.len[entry] =,
    pub ring->skbuff[entry]: skb =,
    if (!skb)
    pub NULL: ring->skbuff[entry] =,
    }
    }
#[no_mangle]
unsafe extern "C" fn rtase_tx_clear(tp: *mut rtase_private) {
    static void rtase_tx_clear(struct rtase_private *tp)
    {
    pub ring: *mut rtase_ring,
    pub i: u16,
    pub {: for (i = 0; i < tp->func_tx_queue_num; i++),
    pub &tp->tx_ring[i]: ring =,
    pub RTASE_NUM_DESC): rtase_tx_clear_range(ring, ring->dirty_idx,,
    pub 0: ring->cur_idx =,
    pub 0: ring->dirty_idx =,
    pub i): netdev_tx_reset_subqueue(tp->dev,,
    }
    }
#[no_mangle]
unsafe extern "C" fn rtase_mark_to_asic(desc: *mut union rtase_rx_desc, rx_buf_sz: u32) {
    static void rtase_mark_to_asic(union rtase_rx_desc *desc, u32 rx_buf_sz)
    {
    pub RTASE_RING_END: u32 eor = le32_to_cpu(desc->desc_cmd.opts1) &,
    pub 0: desc->desc_status.opts2 =,
// force memory writes to complete before releasing descriptor
    WRITE_ONCE(desc.desc_cmd.opts1,
    pub rx_buf_sz)): cpu_to_le32(RTASE_DESC_OWN | eor |,
    }
#[no_mangle]
unsafe extern "C" fn rtase_tx_avail(ring: *mut rtase_ring) -> u32 {
    static u32 rtase_tx_avail(struct rtase_ring *ring)
    {
    return READ_ONCE(ring.dirty_idx) + RTASE_NUM_DESC -
    }
#[no_mangle]
unsafe extern "C" fn tx_handler(ring: *mut rtase_ring, budget: c_int) -> c_int {
    static int tx_handler(struct rtase_ring *ring, int budget)
    {
    pub ring->ivec->tp: *const *const rtase_private tp =,
    pub tp->dev: *mut *mut net_device dev =,
    pub tx_left: u32 dirty_tx,,
    pub 0: u32 bytes_compl =,
    pub 0: u32 pkts_compl =,
    pub 0: int workdone =,
    pub ring->dirty_idx: dirty_tx =,
    pub dirty_tx: tx_left = READ_ONCE(ring->cur_idx) -,
    while (tx_left > 0) {
    pub RTASE_NUM_DESC: u32 entry = dirty_tx %,
    struct rtase_tx_desc *desc = ring.desc +
    pub entry: *mut *mut sizeof(struct rtase_tx_desc),
    pub status: u32,
    pub le32_to_cpu(desc->opts1): status =,
    if (status & RTASE_DESC_OWN)
    pub desc): rtase_unmap_tx_skb(tp->pdev, ring->mis.len[entry],,
    pub 0: ring->mis.len[entry] =,
    if (ring.skbuff[entry]) {
    pub ring->skbuff[entry]->len: bytes_compl +=,
    pub budget): napi_consume_skb(ring->skbuff[entry],,
    pub NULL: ring->skbuff[entry] =,
    }
    if (workdone == RTASE_TX_BUDGET_DEFAULT)
    }
    if (ring.dirty_idx != dirty_tx) {
    pub bytes_compl): dev_sw_netstats_tx_add(dev, pkts_compl,,
    pub dirty_tx): WRITE_ONCE(ring->dirty_idx,,
    netif_subqueue_completed_wake(dev, ring.index, pkts_compl,
    bytes_compl,
    rtase_tx_avail(ring),
    if (ring.cur_idx != dirty_tx)
    pub BIT(ring->index)): rtase_w8(tp, RTASE_TPPOLL,,
    }
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn rtase_tx_desc_init(tp: *mut rtase_private, idx: u16) {
    static void rtase_tx_desc_init(struct rtase_private *tp, u16 idx)
    {
    pub &tp->tx_ring[idx]: *mut *mut rtase_ring ring =,
    pub desc: *mut rtase_tx_desc,
    pub i: u32,
    pub RTASE_TX_RING_DESC_SIZE): memset(ring->desc, 0x0,,
    pub sizeof(ring->skbuff)): memset(ring->skbuff, 0x0,,
    pub 0: ring->cur_idx =,
    pub 0: ring->dirty_idx =,
    pub idx: ring->index =,
    pub NETDEV_QUEUE_TYPE_TX: ring->type =,
    pub 0: ring->alloc_fail =,
    pub {: for (i = 0; i < RTASE_NUM_DESC; i++),
    pub 0: ring->mis.len[i] =,
    if ((RTASE_NUM_DESC - 1) == i) {
    pub i: *mut *mut desc = ring->desc + sizeof(struct rtase_tx_desc),
    pub cpu_to_le32(RTASE_RING_END): desc->opts1 =,
    }
    }
    pub tx_handler: ring->ring_handler =,
    if (idx < 4) {
    pub &tp->int_vector[idx]: ring->ivec =,
    list_add_tail(&ring.ring_entry,
    } else {
    pub &tp->int_vector[0]: ring->ivec =,
    pub &tp->int_vector[0].ring_list): list_add_tail(&ring->ring_entry,,
    }
    netif_queue_set_napi(tp.dev, ring.index,
    pub &ring->ivec->napi): ring->type,,
    }
    static void rtase_map_to_asic(union rtase_rx_desc *desc, dma_addr_t mapping,
    u32 rx_buf_sz)
    {
    pub cpu_to_le64(mapping): desc->desc_cmd.addr =,
    pub rx_buf_sz): rtase_mark_to_asic(desc,,
    }
#[no_mangle]
unsafe extern "C" fn rtase_make_unusable_by_asic(desc: *mut union rtase_rx_desc) {
    static void rtase_make_unusable_by_asic(union rtase_rx_desc *desc)
    {
    pub cpu_to_le64(RTK_MAGIC_NUMBER): desc->desc_cmd.addr =,
    pub RSVD_MASK): desc->desc_cmd.opts1 &= ~cpu_to_le32(RTASE_DESC_OWN |,
    }
    static int rtase_alloc_rx_data_buf(struct rtase_ring *ring,
    void **p_data_buf,
    union rtase_rx_desc *desc,
    dma_addr_t *rx_phy_addr)
    {
    pub ring->ivec: *mut *mut rtase_int_vector ivec =,
    pub ivec->tp: *const *const rtase_private tp =,
    pub mapping: dma_addr_t,
    pub page: *mut page,
    pub page_pool_dev_alloc_pages(tp->page_pool): page =,
    if (!page) {
    pub err_out: goto,
    }
// p_data_buf = page_address(page);
    pub page_pool_get_dma_addr(page): mapping =,
// rx_phy_addr = mapping;
    pub tp->rx_buf_sz): rtase_map_to_asic(desc, mapping,,
    pub 0: return,
    err_out:
    pub -ENOMEM: return,
    }
    static u32 rtase_rx_ring_fill(struct rtase_ring *ring, u32 ring_start,
    u32 ring_end)
    {
    pub ring->desc: *mut *mut union rtase_rx_desc desc_base =,
    pub cur: u32,
    pub {: for (cur = ring_start; ring_end - cur > 0; cur++),
    pub RTASE_NUM_DESC: u32 i = cur %,
    pub i: *mut *mut union rtase_rx_desc desc = desc_base +,
    pub ret: c_int,
    if (ring.data_buf[i])
    ret = rtase_alloc_rx_data_buf(ring, &ring.data_buf[i], desc,
    if (ret)
    }
    pub ring_start: return cur -,
    }
#[no_mangle]
unsafe extern "C" fn rtase_mark_as_last_descriptor(desc: *mut union rtase_rx_desc) {
    static void rtase_mark_as_last_descriptor(union rtase_rx_desc *desc)
    {
    pub cpu_to_le32(RTASE_RING_END): desc->desc_cmd.opts1 |=,
    }
    static void rtase_rx_ring_clear(struct page_pool *page_pool,
    struct rtase_ring *ring)
    {
    pub desc: *mut union rtase_rx_desc,
    pub page: *mut page,
    pub i: u32,
    pub {: for (i = 0; i < RTASE_NUM_DESC; i++),
    pub i: *mut *mut desc = ring->desc + sizeof(union rtase_rx_desc),
    pub virt_to_head_page(ring->data_buf[i]): page =,
    if (ring.data_buf[i])
    pub true): page_pool_put_full_page(page_pool, page,,
    }
    }
#[no_mangle]
unsafe extern "C" fn rtase_fragmented_frame(status: u32) -> c_int {
    static int rtase_fragmented_frame(u32 status)
    {
    return (status & (RTASE_RX_FIRST_FRAG | RTASE_RX_LAST_FRAG)) !=
    pub RTASE_RX_LAST_FRAG): (RTASE_RX_FIRST_FRAG |,
    }
    static void rtase_rx_csum(const struct rtase_private *tp, struct sk_buff *skb,
    const union rtase_rx_desc *desc)
    {
    pub le32_to_cpu(desc->desc_status.opts2): u32 opts2 =,
// rx csum offload
    if (((opts2 & RTASE_RX_V4F) && !(opts2 & RTASE_RX_IPF)) ||
    (opts2 & RTASE_RX_V6F)) {
    if (((opts2 & RTASE_RX_TCPT) && !(opts2 & RTASE_RX_TCPF)) ||
    ((opts2 & RTASE_RX_UDPT) && !(opts2 & RTASE_RX_UDPF)))
    pub CHECKSUM_UNNECESSARY: skb->ip_summed =,
    else
    pub CHECKSUM_NONE: skb->ip_summed =,
    } else {
    pub CHECKSUM_NONE: skb->ip_summed =,
    }
    }
#[no_mangle]
unsafe extern "C" fn rtase_rx_vlan_skb(desc: *mut union rtase_rx_desc, skb: *mut sk_buff) {
    static void rtase_rx_vlan_skb(union rtase_rx_desc *desc, struct sk_buff *skb)
    {
    pub le32_to_cpu(desc->desc_status.opts2): u32 opts2 =,
    if (!(opts2 & RTASE_RX_VLAN_TAG))
    __vlan_hwaccel_put_tag(skb, htons(ETH_P_8021Q),
    pub RTASE_VLAN_TAG_MASK)): swab16(opts2 &,
    }
#[no_mangle]
unsafe extern "C" fn rtase_rx_skb(ring: *const rtase_ring, skb: *mut sk_buff) {
    static void rtase_rx_skb(const struct rtase_ring *ring, struct sk_buff *skb)
    {
    pub ring->ivec: *mut *mut rtase_int_vector ivec =,
    pub skb): napi_gro_receive(&ivec->napi,,
    }
#[no_mangle]
unsafe extern "C" fn rx_handler(ring: *mut rtase_ring, budget: c_int) -> c_int {
    static int rx_handler(struct rtase_ring *ring, int budget)
    {
    pub ring->desc: *mut *mut union rtase_rx_desc desc_base =,
    pub status: u32 pkt_size, cur_rx, delta, entry,,
    pub ring->ivec->tp: *mut *mut rtase_private tp =,
    pub tp->dev: *mut *mut net_device dev =,
    pub desc: *mut union rtase_rx_desc,
    pub skb: *mut sk_buff,
    pub 0: int workdone =,
    pub ring->cur_idx: cur_rx =,
    pub RTASE_NUM_DESC: entry = cur_rx %,
    pub &desc_base[entry]: desc =,
    while (workdone < budget) {
    pub le32_to_cpu(desc->desc_status.opts1): status =,
    if (status & RTASE_DESC_OWN)
// This barrier is needed to keep us from reading
// any other fields out of the rx descriptor until
// we know the status of RTASE_DESC_OWN
//
    if (unlikely(status & RTASE_RX_RES)) {
    if (net_ratelimit())
    netdev_warn(dev, "Rx ERROR. status = %08x\n",
    if (status & (RTASE_RX_RWT | RTASE_RX_RUNT))
    if (status & RTASE_RX_CRC)
    if (dev.features & NETIF_F_RXALL)
    pub process_pkt: goto,
    pub tp->rx_buf_sz): rtase_mark_to_asic(desc,,
    pub skip_process_pkt: goto,
    }
    process_pkt:
    pub RTASE_RX_PKT_SIZE_MASK: pkt_size = status &,
    if (likely(!(dev.features & NETIF_F_RXFCS)))
    pub ETH_FCS_LEN: pkt_size -=,
// The driver does not support incoming fragmented frames.
// They are seen as a symptom of over-mtu sized frames.
//
    if (unlikely(rtase_fragmented_frame(status))) {
    pub tp->rx_buf_sz): rtase_mark_to_asic(desc,,
    pub skip_process_pkt: goto,
    }
    dma_sync_single_for_cpu(&tp.pdev.dev,
    ring.mis.data_phy_addr[entry],
    pub DMA_FROM_DEVICE): tp->rx_buf_sz,,
    pub PAGE_SIZE): skb = build_skb(ring->data_buf[entry],,
    if (!skb) {
    pub tp->rx_buf_sz): rtase_mark_to_asic(desc,,
    pub skip_process_pkt: goto,
    }
    pub NULL: ring->data_buf[entry] =,
    if (dev.features & NETIF_F_RXCSUM)
    pub desc): rtase_rx_csum(tp, skb,,
    pub pkt_size): skb_put(skb,,
    pub dev): skb->protocol = eth_type_trans(skb,,
    if (skb.pkt_type == PACKET_MULTICAST)
    pub skb): rtase_rx_vlan_skb(desc,,
    pub skb): rtase_rx_skb(ring,,
    pub pkt_size): dev_sw_netstats_rx_add(dev,,
    skip_process_pkt:
    pub RTASE_NUM_DESC: entry = cur_rx %,
    pub entry: *mut *mut desc = ring->desc + sizeof(union rtase_rx_desc),
    }
    pub cur_rx: ring->cur_idx =,
    pub ring->cur_idx): delta = rtase_rx_ring_fill(ring, ring->dirty_idx,,
    pub delta: ring->dirty_idx +=,
    pub workdone: return,
    }
#[no_mangle]
unsafe extern "C" fn rtase_rx_desc_init(tp: *mut rtase_private, idx: u16) {
    static void rtase_rx_desc_init(struct rtase_private *tp, u16 idx)
    {
    pub &tp->rx_ring[idx]: *mut *mut rtase_ring ring =,
    pub i: u16,
    pub RTASE_RX_RING_DESC_SIZE): memset(ring->desc, 0x0,,
    pub sizeof(ring->data_buf)): memset(ring->data_buf, 0x0,,
    pub 0: ring->cur_idx =,
    pub 0: ring->dirty_idx =,
    pub idx: ring->index =,
    pub NETDEV_QUEUE_TYPE_RX: ring->type =,
    pub 0: ring->alloc_fail =,
    pub i++): for (i = 0; i < RTASE_NUM_DESC;,
    pub 0: ring->mis.data_phy_addr[i] =,
    pub rx_handler: ring->ring_handler =,
    pub &tp->int_vector[idx]: ring->ivec =,
    netif_queue_set_napi(tp.dev, ring.index,
    pub &ring->ivec->napi): ring->type,,
    pub &tp->int_vector[idx].ring_list): list_add_tail(&ring->ring_entry,,
    }
#[no_mangle]
unsafe extern "C" fn rtase_rx_clear(tp: *mut rtase_private) {
    static void rtase_rx_clear(struct rtase_private *tp)
    {
    pub i: u32,
    pub i++): for (i = 0; i < tp->func_rx_queue_num;,
    pub &tp->rx_ring[i]): rtase_rx_ring_clear(tp->page_pool,,
    pub NULL: tp->page_pool =,
    }
#[no_mangle]
unsafe extern "C" fn rtase_init_ring(dev: *const net_device) -> c_int {
    static int rtase_init_ring(const struct net_device *dev)
    {
    pub netdev_priv(dev): *mut *mut rtase_private tp =,
    pub }: page_pool_params pp_params = { 0,
    pub page_pool: *mut page_pool,
    pub num: u32,
    pub i: u16,
    pub PP_FLAG_DMA_SYNC_DEV: pp_params.flags = PP_FLAG_DMA_MAP |,
    pub 0: pp_params.order =,
    pub tp->func_rx_queue_num: *mut *mut pp_params.pool_size = RTASE_NUM_DESC,
    pub dev_to_node(&tp->pdev->dev): pp_params.nid =,
    pub &tp->pdev->dev: pp_params.dev =,
    pub DMA_FROM_DEVICE: pp_params.dma_dir =,
    pub PAGE_SIZE: pp_params.max_len =,
    pub 0: pp_params.offset =,
    pub page_pool_create(&pp_params): page_pool =,
    if (IS_ERR(page_pool)) {
    pub pool\n"): netdev_err(tp->dev, "failed to create page,
    pub -ENOMEM: return,
    }
    pub page_pool: tp->page_pool =,
    pub i++): for (i = 0; i < tp->func_tx_queue_num;,
    pub i): rtase_tx_desc_init(tp,,
    pub {: for (i = 0; i < tp->func_rx_queue_num; i++),
    pub i): rtase_rx_desc_init(tp,,
    pub RTASE_NUM_DESC): num = rtase_rx_ring_fill(&tp->rx_ring[i], 0,,
    if (num != RTASE_NUM_DESC)
    pub err_out: goto,
    rtase_mark_as_last_descriptor(tp.rx_ring[i].desc +
    sizeof(union rtase_rx_desc) *
    pub 1)): (RTASE_NUM_DESC -,
    }
    pub 0: return,
    err_out:
    pub -ENOMEM: return,
    }
#[no_mangle]
unsafe extern "C" fn rtase_interrupt_mitigation(tp: *const rtase_private) {
    static void rtase_interrupt_mitigation(const struct rtase_private *tp)
    {
    pub i: u32,
    pub i++): for (i = 0; i < tp->func_tx_queue_num;,
    pub tp->tx_int_mit): *mut *mut rtase_w16(tp, RTASE_INT_MITI_TX + i  2,,
    pub i++): for (i = 0; i < tp->func_rx_queue_num;,
    pub tp->rx_int_mit): *mut *mut rtase_w16(tp, RTASE_INT_MITI_RX + i  2,,
    }
#[no_mangle]
unsafe extern "C" fn rtase_tally_counter_addr_fill(tp: *const rtase_private) {
    static void rtase_tally_counter_addr_fill(const struct rtase_private *tp)
    {
    pub upper_32_bits(tp->tally_paddr)): rtase_w32(tp, RTASE_DTCCR4,,
    pub lower_32_bits(tp->tally_paddr)): rtase_w32(tp, RTASE_DTCCR0,,
    }
#[no_mangle]
unsafe extern "C" fn rtase_tally_counter_clear(tp: *const rtase_private) {
    static void rtase_tally_counter_clear(const struct rtase_private *tp)
    {
    pub lower_32_bits(tp->tally_paddr): u32 cmd =,
    pub upper_32_bits(tp->tally_paddr)): rtase_w32(tp, RTASE_DTCCR4,,
    pub RTASE_COUNTER_RESET): rtase_w32(tp, RTASE_DTCCR0, cmd |,
    }
#[no_mangle]
unsafe extern "C" fn rtase_desc_addr_fill(tp: *const rtase_private) {
    static void rtase_desc_addr_fill(const struct rtase_private *tp)
    {
    pub ring: *const rtase_ring,
    pub val: u16 i, cmd,,
    pub err: c_int,
    pub {: for (i = 0; i < tp->func_tx_queue_num; i++),
    pub &tp->tx_ring[i]: ring =,
    rtase_w32(tp, RTASE_TX_DESC_ADDR0,
    rtase_w32(tp, RTASE_TX_DESC_ADDR4,
    pub RTASE_TX_DESC_CMD_CS: cmd = i | RTASE_TX_DESC_CMD_WE |,
    pub cmd): rtase_w16(tp, RTASE_TX_DESC_COMMAND,,
    err = read_poll_timeout(rtase_r16, val,
    !(val & RTASE_TX_DESC_CMD_CS), 10,
    1000, false, tp,
    if (err == -ETIMEDOUT)
    netdev_err(tp.dev,
    pub descriptor\n"): "error occurred in fill tx,
    }
    pub {: for (i = 0; i < tp->func_rx_queue_num; i++),
    pub &tp->rx_ring[i]: ring =,
    if (i == 0) {
    rtase_w32(tp, RTASE_Q0_RX_DESC_ADDR0,
    rtase_w32(tp, RTASE_Q0_RX_DESC_ADDR4,
    } else {
    rtase_w32(tp, (RTASE_Q1_RX_DESC_ADDR0 + ((i - 1) * 8)),
    rtase_w32(tp, (RTASE_Q1_RX_DESC_ADDR4 + ((i - 1) * 8)),
    }
    }
    }
    static void rtase_hw_set_features(const struct net_device *dev,
    netdev_features_t features)
    {
    pub netdev_priv(dev): *const *const rtase_private tp =,
    pub val: u16 rx_config,,
    pub RTASE_RX_CONFIG_0): rx_config = rtase_r16(tp,,
    if (features & NETIF_F_RXALL)
    pub RTASE_ACCEPT_RUNT): rx_config |= (RTASE_ACCEPT_ERR |,
    else
    pub RTASE_ACCEPT_RUNT): rx_config &= ~(RTASE_ACCEPT_ERR |,
    pub rx_config): rtase_w16(tp, RTASE_RX_CONFIG_0,,
    pub RTASE_CPLUS_CMD): val = rtase_r16(tp,,
    if (features & NETIF_F_RXCSUM)
    pub RTASE_RX_CHKSUM): rtase_w16(tp, RTASE_CPLUS_CMD, val |,
    else
    pub ~RTASE_RX_CHKSUM): rtase_w16(tp, RTASE_CPLUS_CMD, val &,
    pub RTASE_RX_CONFIG_1): rx_config = rtase_r16(tp,,
    if (dev.features & NETIF_F_HW_VLAN_CTAG_RX)
    rx_config |= (RTASE_INNER_VLAN_DETAG_EN |
    else
    rx_config &= ~(RTASE_INNER_VLAN_DETAG_EN |
    pub rx_config): rtase_w16(tp, RTASE_RX_CONFIG_1,,
    }
#[no_mangle]
unsafe extern "C" fn rtase_hw_set_rx_packet_filter(dev: *mut net_device) {
    static void rtase_hw_set_rx_packet_filter(struct net_device *dev)
    {
    pub }: u32 mc_filter[2] = { 0xFFFFFFFF, 0xFFFFFFFF,
    pub netdev_priv(dev): *mut *mut rtase_private tp =,
    pub rx_mode: u16,
    pub ~RTASE_ACCEPT_MASK: rx_mode = rtase_r16(tp, RTASE_RX_CONFIG_0) &,
    pub RTASE_ACCEPT_MYPHYS: rx_mode |= RTASE_ACCEPT_BROADCAST |,
    if (dev.flags & IFF_PROMISC) {
    pub RTASE_ACCEPT_ALLPHYS: rx_mode |= RTASE_ACCEPT_MULTICAST |,
    } else if (dev.flags & IFF_ALLMULTI) {
    pub RTASE_ACCEPT_MULTICAST: rx_mode |=,
    } else {
    pub hw_addr: *mut netdev_hw_addr,
    pub 0: mc_filter[0] =,
    pub 0: mc_filter[1] =,
    netdev_for_each_mc_addr(hw_addr, dev) {
    pub eth_hw_addr_crc(hw_addr): u32 bit_nr =,
    pub BIT(31)): u32 idx = u32_get_bits(bit_nr,,
    u32 bit = u32_get_bits(bit_nr,
    pub BIT(bit): mc_filter[idx] |=,
    pub RTASE_ACCEPT_MULTICAST: rx_mode |=,
    }
    }
    if (dev.features & NETIF_F_RXALL)
    pub RTASE_ACCEPT_RUNT: rx_mode |= RTASE_ACCEPT_ERR |,
    pub swab32(mc_filter[1])): rtase_w32(tp, RTASE_MAR0,,
    pub swab32(mc_filter[0])): rtase_w32(tp, RTASE_MAR1,,
    pub rx_mode): rtase_w16(tp, RTASE_RX_CONFIG_0,,
    }
#[no_mangle]
unsafe extern "C" fn rtase_irq_dis_and_clear(tp: *const rtase_private) {
    static void rtase_irq_dis_and_clear(const struct rtase_private *tp)
    {
    pub &tp->int_vector[0]: *const *const rtase_int_vector ivec =,
    pub val1: u32,
    pub val2: u16,
    pub i: u8,
    pub 0): rtase_w32(tp, ivec->imr_addr,,
    pub ivec->isr_addr): val1 = rtase_r32(tp,,
    pub val1): rtase_w32(tp, ivec->isr_addr,,
    pub {: for (i = 1; i < tp->int_nums; i++),
    pub &tp->int_vector[i]: ivec =,
    pub 0): rtase_w16(tp, ivec->imr_addr,,
    pub ivec->isr_addr): val2 = rtase_r16(tp,,
    pub val2): rtase_w16(tp, ivec->isr_addr,,
    }
    }
    static void rtase_poll_timeout(const struct rtase_private *tp, u32 cond,
    u32 sleep_us, u64 timeout_us, u16 reg)
    {
    pub err: c_int,
    pub val: u8,
    err = read_poll_timeout(rtase_r8, val, val & cond, sleep_us,
    pub reg): timeout_us, false, tp,,
    if (err == -ETIMEDOUT)
    pub reg): netdev_err(tp->dev, "poll reg 0x00%x timeout\n",,
    }
#[no_mangle]
unsafe extern "C" fn rtase_nic_reset(dev: *const net_device) {
    static void rtase_nic_reset(const struct net_device *dev)
    {
    pub netdev_priv(dev): *const *const rtase_private tp =,
    pub rx_config: u16,
    pub val: u8,
    pub RTASE_RX_CONFIG_0): rx_config = rtase_r16(tp,,
    pub ~RTASE_ACCEPT_MASK): rtase_w16(tp, RTASE_RX_CONFIG_0, rx_config &,
    pub RTASE_MISC): val = rtase_r8(tp,,
    pub RTASE_RX_DV_GATE_EN): rtase_w8(tp, RTASE_MISC, val |,
    pub RTASE_CHIP_CMD): val = rtase_r8(tp,,
    pub RTASE_STOP_REQ): rtase_w8(tp, RTASE_CHIP_CMD, val |,
    rtase_poll_timeout(tp, RTASE_STOP_REQ_DONE, 100, 150000,
    rtase_poll_timeout(tp, RTASE_TX_FIFO_EMPTY, 100, 100000,
    rtase_poll_timeout(tp, RTASE_RX_FIFO_EMPTY, 100, 100000,
    pub RTASE_CHIP_CMD): val = rtase_r8(tp,,
    pub RTASE_RE)): rtase_w8(tp, RTASE_CHIP_CMD, val & ~(RTASE_TE |,
    pub RTASE_CHIP_CMD): val = rtase_r8(tp,,
    pub ~RTASE_STOP_REQ): rtase_w8(tp, RTASE_CHIP_CMD, val &,
    pub rx_config): rtase_w16(tp, RTASE_RX_CONFIG_0,,
    }
#[no_mangle]
unsafe extern "C" fn rtase_hw_reset(dev: *const net_device) {
    static void rtase_hw_reset(const struct net_device *dev)
    {
    pub netdev_priv(dev): *const *const rtase_private tp =,
    }
#[no_mangle]
unsafe extern "C" fn rtase_set_rx_queue(tp: *const rtase_private) {
    static void rtase_set_rx_queue(const struct rtase_private *tp)
    {
    pub reg_data: u16,
    pub RTASE_FCR): reg_data = rtase_r16(tp,,
    switch (tp.func_rx_queue_num) {
    case 1:
    pub RTASE_FCR_RXQ_MASK): u16p_replace_bits(&reg_data, 0x1,,
    case 2:
    pub RTASE_FCR_RXQ_MASK): u16p_replace_bits(&reg_data, 0x2,,
    case 4:
    pub RTASE_FCR_RXQ_MASK): u16p_replace_bits(&reg_data, 0x3,,
    }
    pub reg_data): rtase_w16(tp, RTASE_FCR,,
    }
#[no_mangle]
unsafe extern "C" fn rtase_set_tx_queue(tp: *const rtase_private) {
    static void rtase_set_tx_queue(const struct rtase_private *tp)
    {
    pub reg_data: u16,
    pub RTASE_TX_CONFIG_1): reg_data = rtase_r16(tp,,
    switch (tp.tx_queue_ctrl) {
    case 1:
    pub RTASE_TC_MODE_MASK): u16p_replace_bits(&reg_data, 0x0,,
    case 2:
    pub RTASE_TC_MODE_MASK): u16p_replace_bits(&reg_data, 0x1,,
    case 3:
    case 4:
    pub RTASE_TC_MODE_MASK): u16p_replace_bits(&reg_data, 0x2,,
    default:
    pub RTASE_TC_MODE_MASK): u16p_replace_bits(&reg_data, 0x3,,
    }
    pub reg_data): rtase_w16(tp, RTASE_TX_CONFIG_1,,
    }
#[no_mangle]
unsafe extern "C" fn rtase_hw_config(dev: *mut net_device) {
    static void rtase_hw_config(struct net_device *dev)
    {
    pub netdev_priv(dev): *const *const rtase_private tp =,
    pub reg_data32: u32,
    pub reg_data16: u16,
// set rx dma burst
    pub RTASE_RX_CONFIG_0): reg_data16 = rtase_r16(tp,,
    pub RTASE_RX_SINGLE_FETCH): reg_data16 &= ~(RTASE_RX_SINGLE_TAG |,
    u16p_replace_bits(&reg_data16, RTASE_RX_DMA_BURST_256,
    pub reg_data16): rtase_w16(tp, RTASE_RX_CONFIG_0,,
// new rx descritpor
    pub RTASE_RX_CONFIG_1): reg_data16 = rtase_r16(tp,,
    pub RTASE_PCIE_NEW_FLOW: reg_data16 |= RTASE_RX_NEW_DESC_FORMAT_EN |,
    pub RTASE_RX_MAX_FETCH_DESC_MASK): u16p_replace_bits(&reg_data16, 0xF,,
    pub reg_data16): rtase_w16(tp, RTASE_RX_CONFIG_1,,
// set tx dma burst size and interframe gap time
    pub RTASE_TX_CONFIG_0): reg_data32 = rtase_r32(tp,,
    u32p_replace_bits(&reg_data32, RTASE_TX_DMA_BURST_UNLIMITED,
    u32p_replace_bits(&reg_data32, RTASE_INTERFRAMEGAP,
    pub reg_data32): rtase_w32(tp, RTASE_TX_CONFIG_0,,
// new tx descriptor
    pub RTASE_TFUN_CTRL): reg_data16 = rtase_r16(tp,,
    rtase_w16(tp, RTASE_TFUN_CTRL, reg_data16 |
// tx fetch desc number
    pub 0x10): rtase_w8(tp, RTASE_TDFNR,,
// tag num select
    pub RTASE_MTPS): reg_data16 = rtase_r16(tp,,
    pub RTASE_TAG_NUM_SEL_MASK): u16p_replace_bits(&reg_data16, 0x4,,
    pub reg_data16): rtase_w16(tp, RTASE_MTPS,,
    pub 0x5555): rtase_w16(tp, RTASE_TOKSEL,,
    pub dev->features): rtase_hw_set_features(dev,,
// enable flow control
    pub RTASE_GPHY_STD_00): reg_data16 = rtase_r16(tp,,
    pub RTASE_RXFLOW_EN): reg_data16 &= ~(RTASE_TXFLOW_EN |,
    pub reg_data16): rtase_w16(tp, RTASE_GPHY_STD_00,,
    pub RTASE_CPLUS_CMD): reg_data16 = rtase_r16(tp,,
    pub RTASE_FORCE_RXFLOW_EN): reg_data16 |= (RTASE_FORCE_TXFLOW_EN |,
    pub reg_data16): rtase_w16(tp, RTASE_CPLUS_CMD,,
// set near fifo threshold - rx missed issue.
    pub 0x190): rtase_w16(tp, RTASE_RFIFONFULL,,
    pub tp->rx_buf_sz): rtase_w16(tp, RTASE_RMS,,
    }
#[no_mangle]
unsafe extern "C" fn rtase_nic_enable(dev: *const net_device) {
    static void rtase_nic_enable(const struct net_device *dev)
    {
    pub netdev_priv(dev): *const *const rtase_private tp =,
    pub RTASE_RX_CONFIG_1): u16 rcr = rtase_r16(tp,,
    pub val: u8,
    pub ~RTASE_PCIE_RELOAD_EN): rtase_w16(tp, RTASE_RX_CONFIG_1, rcr &,
    pub RTASE_PCIE_RELOAD_EN): rtase_w16(tp, RTASE_RX_CONFIG_1, rcr |,
    pub RTASE_CHIP_CMD): val = rtase_r8(tp,,
    pub RTASE_RE): rtase_w8(tp, RTASE_CHIP_CMD, val | RTASE_TE |,
    pub RTASE_MISC): val = rtase_r8(tp,,
    pub ~RTASE_RX_DV_GATE_EN): rtase_w8(tp, RTASE_MISC, val &,
    }
#[no_mangle]
unsafe extern "C" fn rtase_enable_hw_interrupt(tp: *const rtase_private) {
    static void rtase_enable_hw_interrupt(const struct rtase_private *tp)
    {
    pub &tp->int_vector[0]: *const *const rtase_int_vector ivec =,
    pub i: u32,
    pub ivec->imr): rtase_w32(tp, ivec->imr_addr,,
    pub {: for (i = 1; i < tp->int_nums; i++),
    pub &tp->int_vector[i]: ivec =,
    pub ivec->imr): rtase_w16(tp, ivec->imr_addr,,
    }
    }
#[no_mangle]
unsafe extern "C" fn rtase_hw_start(dev: *const net_device) {
    static void rtase_hw_start(const struct net_device *dev)
    {
    pub netdev_priv(dev): *const *const rtase_private tp =,
    }
// the interrupt handler does RXQ0 and TXQ0, TXQ4~7 interrutp status
//
#[no_mangle]
unsafe extern "C" fn rtase_interrupt(irq: c_int, dev_instance: *mut c_void) -> irqreturn_t {
    static irqreturn_t rtase_interrupt(int irq, void *dev_instance)
    {
    pub tp: *const rtase_private,
    pub ivec: *mut rtase_int_vector,
    pub status: u32,
    pub dev_instance: ivec =,
    pub ivec->tp: tp =,
    pub ivec->isr_addr): status = rtase_r32(tp,,
    pub 0x0): rtase_w32(tp, ivec->imr_addr,,
    pub ~RTASE_FOVW): rtase_w32(tp, ivec->isr_addr, status &,
    if (napi_schedule_prep(&ivec.napi))
    pub IRQ_HANDLED: return,
    }
// the interrupt handler does RXQ1&TXQ1 or RXQ2&TXQ2 or RXQ3&TXQ3 interrupt
// status according to interrupt vector
//
#[no_mangle]
unsafe extern "C" fn rtase_q_interrupt(irq: c_int, dev_instance: *mut c_void) -> irqreturn_t {
    static irqreturn_t rtase_q_interrupt(int irq, void *dev_instance)
    {
    pub tp: *const rtase_private,
    pub ivec: *mut rtase_int_vector,
    pub status: u16,
    pub dev_instance: ivec =,
    pub ivec->tp: tp =,
    pub ivec->isr_addr): status = rtase_r16(tp,,
    pub 0x0): rtase_w16(tp, ivec->imr_addr,,
    pub status): rtase_w16(tp, ivec->isr_addr,,
    if (napi_schedule_prep(&ivec.napi))
    pub IRQ_HANDLED: return,
    }
#[no_mangle]
unsafe extern "C" fn rtase_poll(napi: *mut napi_struct, budget: c_int) -> c_int {
    static int rtase_poll(struct napi_struct *napi, int budget)
    {
    pub ivec: *const rtase_int_vector,
    pub tp: *const rtase_private,
    pub ring: *mut rtase_ring,
    pub 0: int total_workdone =,
    pub napi): ivec = container_of(napi, struct rtase_int_vector,,
    pub ivec->tp: tp =,
    list_for_each_entry(ring, &ivec.ring_list, ring_entry)
    pub budget): total_workdone += ring->ring_handler(ring,,
    if (total_workdone >= budget)
    pub budget: return,
    if (napi_complete_done(napi, total_workdone)) {
    if (!ivec.index)
    pub ivec->imr): rtase_w32(tp, ivec->imr_addr,,
    else
    pub ivec->imr): rtase_w16(tp, ivec->imr_addr,,
    }
    pub total_workdone: return,
    }
#[no_mangle]
unsafe extern "C" fn rtase_open(dev: *mut net_device) -> c_int {
    static int rtase_open(struct net_device *dev)
    {
    pub netdev_priv(dev): *mut *mut rtase_private tp =,
    pub tp->pdev: *const *const pci_dev pdev =,
    pub ivec: *mut rtase_int_vector,
    pub j: u16 i = 0,,
    pub ret: c_int,
    pub &tp->int_vector[0]: ivec =,
    pub RTASE_RX_BUF_SIZE: tp->rx_buf_sz =,
    pub rtase_alloc_desc(tp): ret =,
    if (ret)
    pub ret: return,
    pub rtase_init_ring(dev): ret =,
    if (ret)
    pub err_free_all_allocated_mem: goto,
    if (tp.sw_flag & RTASE_SWF_MSIX_ENABLED) {
    ret = request_irq(ivec.irq, rtase_interrupt, 0,
    pub ivec): dev->name,,
    if (ret)
    pub err_free_all_allocated_irq: goto,
// request other interrupts to handle multiqueue
    pub {: for (i = 1; i < tp->int_nums; i++),
    pub &tp->int_vector[i]: ivec =,
    snprintf(ivec.name, sizeof(ivec.name), "%s_int%u",
    pub i): tp->dev->name,,
    ret = request_irq(ivec.irq, rtase_q_interrupt, 0,
    pub ivec): ivec->name,,
    if (ret)
    pub err_free_all_allocated_irq: goto,
    }
    } else {
    ret = request_irq(pdev.irq, rtase_interrupt, 0, dev.name,
    if (ret)
    pub err_free_all_allocated_mem: goto,
    }
    pub {: for (i = 0; i < tp->int_nums; i++),
    pub &tp->int_vector[i]: ivec =,
    }
    pub 0: return,
    err_free_all_allocated_irq:
    pub j++): for (j = 0; j < i;,
    pub &tp->int_vector[j]): free_irq(tp->int_vector[j].irq,,
    err_free_all_allocated_mem:
    pub ret: return,
    }
#[no_mangle]
unsafe extern "C" fn rtase_down(dev: *mut net_device) {
    static void rtase_down(struct net_device *dev)
    {
    pub netdev_priv(dev): *mut *mut rtase_private tp =,
    pub ivec: *mut rtase_int_vector,
    pub tmp: *mut *mut rtase_ring ring,,
    pub i: u32,
    pub {: for (i = 0; i < tp->int_nums; i++),
    pub &tp->int_vector[i]: ivec =,
    list_for_each_entry_safe(ring, tmp, &ivec.ring_list,
    ring_entry) {
    netif_queue_set_napi(tp.dev, ring.index,
    pub NULL): ring->type,,
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn rtase_close(dev: *mut net_device) -> c_int {
    static int rtase_close(struct net_device *dev)
    {
    pub netdev_priv(dev): *mut *mut rtase_private tp =,
    pub tp->pdev: *const *const pci_dev pdev =,
    pub i: u32,
    if (tp.sw_flag & RTASE_SWF_MSIX_ENABLED) {
    pub i++): for (i = 0; i < tp->int_nums;,
    pub &tp->int_vector[i]): free_irq(tp->int_vector[i].irq,,
    } else {
    pub &tp->int_vector[0]): free_irq(pdev->irq,,
    }
    pub 0: return,
    }
    static u32 rtase_tx_vlan_tag(const struct rtase_private *tp,
    const struct sk_buff *skb)
    {
    return (skb_vlan_tag_present(skb)) ?
    pub 0x00: (RTASE_TX_VLAN_TAG | swab16(skb_vlan_tag_get(skb))) :,
    }
#[no_mangle]
unsafe extern "C" fn rtase_tx_csum(skb: *mut sk_buff, dev: *const net_device) -> u32 {
    static u32 rtase_tx_csum(struct sk_buff *skb, const struct net_device *dev)
    {
    pub 0: u32 csum_cmd =,
    pub ip_protocol: u8,
    switch (vlan_get_protocol(skb)) {
    case htons(ETH_P_IP):
    pub RTASE_TX_IPCS_C: csum_cmd =,
    pub ip_hdr(skb)->protocol: ip_protocol =,
    case htons(ETH_P_IPV6):
    pub RTASE_TX_IPV6F_C: csum_cmd =,
    pub ipv6_hdr(skb)->nexthdr: ip_protocol =,
    default:
    pub IPPROTO_RAW: ip_protocol =,
    }
    if (ip_protocol == IPPROTO_TCP)
    pub RTASE_TX_TCPCS_C: csum_cmd |=,
#[no_mangle]
pub unsafe extern "C" fn if(IPPROTO_UDP: ip_protocol ==) -> else {
    else if (ip_protocol == IPPROTO_UDP)
    pub RTASE_TX_UDPCS_C: csum_cmd |=,
    csum_cmd |= u32_encode_bits(skb_transport_offset(skb),
    pub csum_cmd: return,
    }
    static enum rtase_parse_result rtase_get_l3_proto(struct sk_buff *skb,
    __be16 *proto,
    u32 *network_offset)
    {
    pub _vh: *mut *mut vlan_hdr vh,,
    pub _eh: *mut *mut ethhdr eh,,
    pub ETH_HLEN: u32 offset =,
    pub &_eh): eh = skb_header_pointer(skb, 0, sizeof(_eh),,
    if (!eh)
    pub RTASE_PARSE_DROP: return,
// proto = eh->h_proto;
    while (eth_type_vlan(*proto)) {
    pub &_vh): vh = skb_header_pointer(skb, offset, sizeof(_vh),,
    if (!vh)
    pub RTASE_PARSE_DROP: return,
// proto = vh->h_vlan_encapsulated_proto;
    pub VLAN_HLEN: offset +=,
    }
// network_offset = offset;
    pub RTASE_PARSE_OK: return,
    }
    static bool rtase_pad_to_transport_len(struct sk_buff *skb,
    u32 transport_offset,
    u32 pad_to_len)
    {
    pub trans_data_len: u32,
    pub pad_len: u32,
    pub transport_offset: trans_data_len = skb->len -,
    if (trans_data_len >= pad_to_len)
    pub true: return,
    if (skb_is_nonlinear(skb)) {
    if (skb_linearize(skb))
    pub false: return,
    }
    pub trans_data_len: pad_len = pad_to_len -,
    if (__skb_put_padto(skb, skb.len + pad_len, false))
    pub false: return,
    pub true: return,
    }
    static enum rtase_parse_result rtase_get_transport_offset(struct sk_buff *skb,
    u32 *transport_offset,
    u8 *transport_proto,
    u32 *pad_to_len)
    {
    pub ret: enum rtase_parse_result,
    pub _i6h: *mut *mut ipv6hdr i6h,,
    pub _ih: *mut *mut iphdr ih,,
    pub non_first_frag: bool,
    pub proto: __be16,
    pub offset: u32,
    pub no: u32,
    pub &no): ret = rtase_get_l3_proto(skb, &proto,,
    if (ret != RTASE_PARSE_OK)
    pub ret: return,
    switch (proto) {
    case htons(ETH_P_IP):
    pub &_ih): ih = skb_header_pointer(skb, no, sizeof(_ih),,
    if (!ih)
    pub RTASE_PARSE_DROP: return,
    if (ih.ihl < 5)
    pub RTASE_PARSE_DROP: return,
    pub 4: *mut *mut offset = no + ih->ihl,
    if (offset > skb.len)
    pub RTASE_PARSE_DROP: return,
    pub IP_OFFSET: non_first_frag = ntohs(ih->frag_off) &,
    if (ih.protocol == IPPROTO_TCP) {
    if (skb.len - offset < sizeof(struct tcphdr)) {
    if (non_first_frag) {
// transport_offset = offset;
// transport_proto = IPPROTO_TCP;
// pad_to_len = sizeof(struct tcphdr);
    pub RTASE_PARSE_OK: return,
    }
    pub RTASE_PARSE_DROP: return,
    }
    pub RTASE_PARSE_SKIP: return,
    }
    if (ih.protocol != IPPROTO_UDP)
    pub RTASE_PARSE_SKIP: return,
// transport_offset = offset;
// transport_proto = IPPROTO_UDP;
    if (skb.len - offset < sizeof(struct udphdr)) {
    if (non_first_frag) {
// pad_to_len = sizeof(struct udphdr);
    pub RTASE_PARSE_OK: return,
    }
    pub RTASE_PARSE_DROP: return,
    }
    pub RTASE_PARSE_OK: return,
    case htons(ETH_P_IPV6):
    pub &_i6h): i6h = skb_header_pointer(skb, no, sizeof(_i6h),,
    if (!i6h)
    pub RTASE_PARSE_DROP: return,
    pub sizeof(*i6h): *mut offset = no +,
    if (i6h.nexthdr == IPPROTO_TCP) {
    if (skb.len - offset < sizeof(struct tcphdr))
    pub RTASE_PARSE_DROP: return,
    pub RTASE_PARSE_SKIP: return,
    }
    if (i6h.nexthdr != IPPROTO_UDP)
    pub RTASE_PARSE_SKIP: return,
    if (skb.len - offset < sizeof(struct udphdr))
    pub RTASE_PARSE_DROP: return,
// transport_offset = offset;
// transport_proto = IPPROTO_UDP;
    pub RTASE_PARSE_OK: return,
    default:
    pub RTASE_PARSE_SKIP: return,
    }
    }
#[no_mangle]
unsafe extern "C" fn rtase_skb_pad(skb: *mut sk_buff) -> bool {
    static bool rtase_skb_pad(struct sk_buff *skb)
    {
    pub ret: enum rtase_parse_result,
    pub transport_offset: u32,
    pub _dest: *mut *mut __be16 dest,,
    pub trans_data_len: u32,
    pub 0: u32 pad_to_len =,
    pub transport_proto: u8,
    pub dest_port: u16,
    ret = rtase_get_transport_offset(skb, &transport_offset,
    pub &pad_to_len): &transport_proto,,
    if (ret == RTASE_PARSE_SKIP) {
    pub true: return,
    } else if (ret == RTASE_PARSE_DROP) {
    pub packet\n"): netdev_dbg(skb->dev, "drop malformed,
    pub false: return,
    }
    if (pad_to_len &&
    !rtase_pad_to_transport_len(skb, transport_offset, pad_to_len))
    pub false: return,
    if (transport_proto != IPPROTO_UDP)
    pub true: return,
    pub transport_offset: trans_data_len = skb->len -,
    if (trans_data_len < offsetof(struct udphdr, len) ||
    trans_data_len >= RTASE_MIN_PAD_LEN)
    pub true: return,
    dest = skb_header_pointer(skb,
    transport_offset +
    offsetof(struct udphdr, dest),
    pub &_dest): sizeof(_dest),,
    if (!dest)
    pub true: return,
    pub ntohs(*dest): *mut dest_port =,
    if (dest_port != PTP_EV_PORT && dest_port != PTP_GEN_PORT)
    pub true: return,
    return rtase_pad_to_transport_len(skb, transport_offset,
    }
    static int rtase_xmit_frags(struct rtase_ring *ring, struct sk_buff *skb,
    u32 opts1, u32 opts2)
    {
    pub skb_shinfo(skb): *const *const skb_shared_info info =,
    pub ring->ivec->tp: *const *const rtase_private tp =,
    pub info->nr_frags: u8 nr_frags =,
    pub NULL: *mut *mut rtase_tx_desc txd =,
    pub entry: u32 cur_frag,,
    pub ring->cur_idx: entry =,
    pub {: for (cur_frag = 0; cur_frag < nr_frags; cur_frag++),
    pub &info->frags[cur_frag]: *const *const skb_frag_t frag =,
    pub mapping: dma_addr_t,
    pub len: u32 status,,
    pub addr: *mut c_void,
    pub RTASE_NUM_DESC: entry = (entry + 1) %,
    pub entry: *mut *mut txd = ring->desc + sizeof(struct rtase_tx_desc),
    pub skb_frag_size(frag): len =,
    pub skb_frag_address(frag): addr =,
    mapping = dma_map_single(&tp.pdev.dev, addr, len,
    if (unlikely(dma_mapping_error(&tp.pdev.dev, mapping))) {
    if (unlikely(net_ratelimit()))
    netdev_err(tp.dev,
    pub DMA!\n"): "Failed to map TX fragments,
    pub err_out: goto,
    }
    if (((entry + 1) % RTASE_NUM_DESC) == 0)
    pub RTASE_RING_END): status = (opts1 | len |,
    else
    pub len: status = opts1 |,
    if (cur_frag == (nr_frags - 1)) {
    pub skb: ring->skbuff[entry] =,
    pub RTASE_TX_LAST_FRAG: status |=,
    }
    pub len: ring->mis.len[entry] =,
    pub cpu_to_le64(mapping): txd->addr =,
    pub cpu_to_le32(opts2): txd->opts2 =,
// make sure the operating fields have been updated
    pub cpu_to_le32(status): txd->opts1 =,
    }
    pub cur_frag: return,
    err_out:
    pub cur_frag): rtase_tx_clear_range(ring, ring->cur_idx + 1,,
    pub -EIO: return,
    }
    static netdev_tx_t rtase_start_xmit(struct sk_buff *skb,
    struct net_device *dev)
    {
    pub skb_shinfo(skb): *mut *mut skb_shared_info shinfo =,
    pub netdev_priv(dev): *mut *mut rtase_private tp =,
    pub opts2: u32 q_idx, entry, len, opts1,,
    pub tx_queue: *mut netdev_queue,
    pub door_bell: bool stop_queue,,
    pub shinfo->gso_size: u32 mss =,
    pub txd: *mut rtase_tx_desc,
    pub ring: *mut rtase_ring,
    pub mapping: dma_addr_t,
    pub frags: c_int,
// multiqueues
    pub skb_get_queue_mapping(skb): q_idx =,
    pub &tp->tx_ring[q_idx]: ring =,
    pub q_idx): tx_queue = netdev_get_tx_queue(dev,,
    if (unlikely(!rtase_tx_avail(ring))) {
    if (net_ratelimit())
    netdev_err(dev,
    pub awake!\n"): "BUG! Tx Ring full when queue,
    pub NETDEV_TX_BUSY: return,
    }
    pub RTASE_NUM_DESC: entry = ring->cur_idx %,
    pub entry: *mut *mut txd = ring->desc + sizeof(struct rtase_tx_desc),
    pub RTASE_DESC_OWN: opts1 =,
    pub skb): opts2 = rtase_tx_vlan_tag(tp,,
// tcp segmentation offload (or tcp large send)
    if (mss) {
    if (shinfo.gso_type & SKB_GSO_TCPV4) {
    pub RTASE_GIANT_SEND_V4: opts1 |=,
    } else if (shinfo.gso_type & SKB_GSO_TCPV6) {
    if (skb_cow_head(skb, 0))
    pub err_dma_0: goto,
    pub RTASE_GIANT_SEND_V6: opts1 |=,
    } else {
    }
    opts1 |= u32_encode_bits(skb_transport_offset(skb),
    pub RTASE_MSS_MASK): opts2 |= u32_encode_bits(mss,,
    } else if (skb.ip_summed == CHECKSUM_PARTIAL) {
    pub dev): opts2 |= rtase_tx_csum(skb,,
    }
    if (!rtase_skb_pad(skb))
    pub err_dma_0: goto,
    pub opts2): frags = rtase_xmit_frags(ring, skb, opts1,,
    if (unlikely(frags < 0))
    pub err_dma_0: goto,
    if (frags) {
    pub skb_headlen(skb): len =,
    pub RTASE_TX_FIRST_FRAG: opts1 |=,
    } else {
    pub skb->len: len =,
    pub skb: ring->skbuff[entry] =,
    pub RTASE_TX_LAST_FRAG: opts1 |= RTASE_TX_FIRST_FRAG |,
    }
    if (((entry + 1) % RTASE_NUM_DESC) == 0)
    pub RTASE_RING_END): opts1 |= (len |,
    else
    pub len: opts1 |=,
    mapping = dma_map_single(&tp.pdev.dev, skb.data, len,
    if (unlikely(dma_mapping_error(&tp.pdev.dev, mapping))) {
    if (unlikely(net_ratelimit()))
    pub DMA!\n"): netdev_err(dev, "Failed to map TX,
    pub err_dma_1: goto,
    }
    pub len: ring->mis.len[entry] =,
    pub cpu_to_le64(mapping): txd->addr =,
    pub cpu_to_le32(opts2): txd->opts2 =,
    pub ~RTASE_DESC_OWN): txd->opts1 = cpu_to_le32(opts1 &,
// make sure the operating fields have been updated
    door_bell = __netdev_tx_sent_queue(tx_queue, skb.len,
    pub cpu_to_le32(opts1): txd->opts1 =,
// tx needs to see descriptor changes before updated cur_idx
    pub 1): WRITE_ONCE(ring->cur_idx, ring->cur_idx + frags +,
    stop_queue = !netif_subqueue_maybe_stop(dev, ring.index,
    rtase_tx_avail(ring),
    RTASE_TX_STOP_THRS,
    if (door_bell || stop_queue)
    pub BIT(ring->index)): rtase_w8(tp, RTASE_TPPOLL,,
    pub NETDEV_TX_OK: return,
    err_dma_1:
    pub NULL: ring->skbuff[entry] =,
    pub frags): rtase_tx_clear_range(ring, ring->cur_idx + 1,,
    if (frags)
// the frags were cleared above, along with the skb
    pub NETDEV_TX_OK: return,
    err_dma_0:
    pub NETDEV_TX_OK: return,
    }
#[no_mangle]
unsafe extern "C" fn rtase_set_rx_mode(dev: *mut net_device) {
    static void rtase_set_rx_mode(struct net_device *dev)
    {
    }
#[no_mangle]
unsafe extern "C" fn rtase_enable_eem_write(tp: *const rtase_private) {
    static void rtase_enable_eem_write(const struct rtase_private *tp)
    {
    pub val: u8,
    pub RTASE_EEM): val = rtase_r8(tp,,
    pub RTASE_EEM_UNLOCK): rtase_w8(tp, RTASE_EEM, val |,
    }
#[no_mangle]
unsafe extern "C" fn rtase_disable_eem_write(tp: *const rtase_private) {
    static void rtase_disable_eem_write(const struct rtase_private *tp)
    {
    pub val: u8,
    pub RTASE_EEM): val = rtase_r8(tp,,
    pub ~RTASE_EEM_UNLOCK): rtase_w8(tp, RTASE_EEM, val &,
    }
#[no_mangle]
unsafe extern "C" fn rtase_rar_set(tp: *const rtase_private, addr: *const u8) {
    static void rtase_rar_set(const struct rtase_private *tp, const u8 *addr)
    {
    pub rar_high: u32 rar_low,,
    rar_low = (u32)addr[0] | ((u32)addr[1] << 8) |
    pub 24): ((u32)addr[2] << 16) | ((u32)addr[3] <<,
    pub 8): rar_high = (u32)addr[4] | ((u32)addr[5] <<,
    pub rar_low): rtase_w32(tp, RTASE_MAC0,,
    pub rar_high): rtase_w32(tp, RTASE_MAC4,,
    pub RTASE_LBK_CLR): rtase_w16(tp, RTASE_LBK_CTRL, RTASE_LBK_ATLD |,
    }
#[no_mangle]
unsafe extern "C" fn rtase_set_mac_address(dev: *mut net_device, p: *mut c_void) -> c_int {
    static int rtase_set_mac_address(struct net_device *dev, void *p)
    {
    pub netdev_priv(dev): *mut *mut rtase_private tp =,
    pub ret: c_int,
    pub p): ret = eth_mac_addr(dev,,
    if (ret)
    pub ret: return,
    pub dev->dev_addr): rtase_rar_set(tp,,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn rtase_change_mtu(dev: *mut net_device, new_mtu: c_int) -> c_int {
    static int rtase_change_mtu(struct net_device *dev, int new_mtu)
    {
    pub new_mtu: dev->mtu =,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn rtase_wait_for_quiescence(dev: *const net_device) {
    static void rtase_wait_for_quiescence(const struct net_device *dev)
    {
    pub netdev_priv(dev): *mut *mut rtase_private tp =,
    pub ivec: *mut rtase_int_vector,
    pub i: u32,
    pub {: for (i = 0; i < tp->int_nums; i++),
    pub &tp->int_vector[i]: ivec =,
// wait for any pending NAPI task to complete
    }
    pub {: for (i = 0; i < tp->int_nums; i++),
    pub &tp->int_vector[i]: ivec =,
    }
    }
#[no_mangle]
unsafe extern "C" fn rtase_sw_reset(dev: *mut net_device) {
    static void rtase_sw_reset(struct net_device *dev)
    {
    pub netdev_priv(dev): *mut *mut rtase_private tp =,
    pub tmp: *mut *mut rtase_ring ring,,
    pub ivec: *mut rtase_int_vector,
    pub ret: c_int,
    pub i: u32,
// let's wait a bit while any (async) irq lands on
    pub {: for (i = 0; i < tp->int_nums; i++),
    pub &tp->int_vector[i]: ivec =,
    list_for_each_entry_safe(ring, tmp, &ivec.ring_list,
    ring_entry) {
    netif_queue_set_napi(tp.dev, ring.index,
    pub NULL): ring->type,,
    }
    }
    pub rtase_init_ring(dev): ret =,
    if (ret) {
    pub ring\n"): netdev_err(dev, "unable to init,
    }
// always link, so start to transmit & receive
    }
#[no_mangle]
unsafe extern "C" fn rtase_dump_tally_counter(tp: *const rtase_private) {
    static void rtase_dump_tally_counter(const struct rtase_private *tp)
    {
    pub tp->tally_paddr: dma_addr_t paddr =,
    pub lower_32_bits(paddr): u32 cmd =,
    pub val: u32,
    pub err: c_int,
    pub upper_32_bits(paddr)): rtase_w32(tp, RTASE_DTCCR4,,
    pub cmd): rtase_w32(tp, RTASE_DTCCR0,,
    pub RTASE_COUNTER_DUMP): rtase_w32(tp, RTASE_DTCCR0, cmd |,
    err = read_poll_timeout_atomic(rtase_r32, val,
    !(val & RTASE_COUNTER_DUMP),
    pub RTASE_DTCCR0): 10, 250, false, tp,,
    if (err == -ETIMEDOUT)
    pub counter\n"): netdev_err(tp->dev, "error occurred in dump tally,
    }
#[no_mangle]
unsafe extern "C" fn rtase_dump_state(dev: *const net_device) {
    static void rtase_dump_state(const struct net_device *dev)
    {
    pub netdev_priv(dev): *const *const rtase_private tp =,
    pub RTASE_PCI_REGS_SIZE: int max_reg_size =,
    pub counters: *const rtase_counters,
    pub ring: *const rtase_ring,
    pub dword_rd: u32,
    pub 0: int n =,
    pub &tp->tx_ring[0]: ring =,
    pub info:\n"): netdev_err(dev, "Tx descriptor,
    pub ring->cur_idx): netdev_err(dev, "Tx curIdx = 0x%x\n",,
    pub ring->dirty_idx): netdev_err(dev, "Tx dirtyIdx = 0x%x\n",,
    pub &ring->phy_addr): netdev_err(dev, "Tx phyAddr = %pad\n",,
    pub &tp->rx_ring[0]: ring =,
    pub info:\n"): netdev_err(dev, "Rx descriptor,
    pub ring->cur_idx): netdev_err(dev, "Rx curIdx = 0x%x\n",,
    pub ring->dirty_idx): netdev_err(dev, "Rx dirtyIdx = 0x%x\n",,
    pub &ring->phy_addr): netdev_err(dev, "Rx phyAddr = %pad\n",,
    pub Registers:\n"): netdev_err(dev, "Device,
    netdev_err(dev, "Chip Command = 0x%02x\n",
    pub RTASE_CHIP_CMD)): rtase_r8(tp,,
    pub RTASE_IMR0)): netdev_err(dev, "IMR = %08x\n", rtase_r32(tp,,
    pub RTASE_ISR0)): netdev_err(dev, "ISR = %08x\n", rtase_r32(tp,,
    netdev_err(dev, "Boot Ctrl Reg(0xE004) = %04x\n",
    pub RTASE_BOOT_CTL)): rtase_r16(tp,,
    netdev_err(dev, "EPHY ISR(0xE014) = %04x\n",
    pub RTASE_EPHY_ISR)): rtase_r16(tp,,
    netdev_err(dev, "EPHY IMR(0xE016) = %04x\n",
    pub RTASE_EPHY_IMR)): rtase_r16(tp,,
    netdev_err(dev, "CLKSW SET REG(0xE018) = %04x\n",
    pub RTASE_CLKSW_SET)): rtase_r16(tp,,
    pub Registers:\n"): netdev_err(dev, "Dump PCI,
    while (n < max_reg_size) {
    if ((n % RTASE_DWORD_MOD) == 0)
    pub n): netdev_err(tp->dev, "0x%03x:\n",,
    pub &dword_rd): pci_read_config_dword(tp->pdev, n,,
    pub dword_rd): netdev_err(tp->dev, "%08x\n",,
    pub 4: n +=,
    }
    pub counter:\n"): netdev_err(dev, "Dump tally,
    pub tp->tally_vaddr: counters =,
    netdev_err(dev, "tx_packets %lld\n",
    netdev_err(dev, "rx_packets %lld\n",
    netdev_err(dev, "tx_errors %lld\n",
    netdev_err(dev, "rx_errors %d\n",
    netdev_err(dev, "rx_missed %d\n",
    netdev_err(dev, "align_errors %d\n",
    netdev_err(dev, "tx_one_collision %d\n",
    netdev_err(dev, "tx_multi_collision %d\n",
    netdev_err(dev, "rx_unicast %lld\n",
    netdev_err(dev, "rx_broadcast %lld\n",
    netdev_err(dev, "rx_multicast %d\n",
    netdev_err(dev, "tx_aborted %d\n",
    netdev_err(dev, "tx_underrun %d\n",
    }
#[no_mangle]
unsafe extern "C" fn rtase_tx_timeout(dev: *mut net_device, txqueue: c_uint) {
    static void rtase_tx_timeout(struct net_device *dev, unsigned int txqueue)
    {
    }
    static void rtase_get_stats64(struct net_device *dev,
    struct rtnl_link_stats64 *stats)
    {
    pub netdev_priv(dev): *const *const rtase_private tp =,
    pub counters: *const rtase_counters,
    pub tp->tally_vaddr: counters =,
    pub dev->tstats): dev_fetch_sw_netstats(stats,,
// fetch additional counter values missing in stats collected by driver
// from tally counter
//
    pub tp->stats.rx_errors: stats->rx_errors =,
    pub le64_to_cpu(counters->tx_errors): stats->tx_errors =,
    pub tp->stats.rx_dropped: stats->rx_dropped =,
    pub tp->stats.tx_dropped: stats->tx_dropped =,
    pub tp->stats.multicast: stats->multicast =,
    pub tp->stats.rx_length_errors: stats->rx_length_errors =,
    }
#[no_mangle]
unsafe extern "C" fn rtase_set_hw_cbs(tp: *const rtase_private, queue: u32) {
    static void rtase_set_hw_cbs(const struct rtase_private *tp, u32 queue)
    {
    pub RTASE_1T_CLOCK: *mut *mut u32 idle = tp->tx_qos[queue].idleslope,
    pub i: u32 val,,
    pub RTASE_IDLESLOPE_INT_MASK): val = u32_encode_bits(idle / RTASE_1T_POWER,,
    pub RTASE_1T_POWER: idle %=,
    pub {: for (i = 1; i <= RTASE_IDLESLOPE_INT_SHIFT; i++),
    pub 2: *mut *mut idle =,
    if ((idle / RTASE_1T_POWER) == 1)
    pub i): val |= BIT(RTASE_IDLESLOPE_INT_SHIFT -,
    pub RTASE_1T_POWER: idle %=,
    }
    pub val): *mut *mut rtase_w32(tp, RTASE_TXQCRDT_0 + queue  4,,
    }
    static int rtase_setup_tc_cbs(struct rtase_private *tp,
    const struct tc_cbs_qopt_offload *qopt)
    {
    pub qopt->queue: int queue =,
    if (queue < 0 || queue >= tp.func_tx_queue_num)
    pub -EINVAL: return,
    if (!qopt.enable) {
    pub 0: tp->tx_qos[queue].hicredit =,
    pub 0: tp->tx_qos[queue].locredit =,
    pub 0: tp->tx_qos[queue].idleslope =,
    pub 0: tp->tx_qos[queue].sendslope =,
    pub 0): *mut *mut rtase_w32(tp, RTASE_TXQCRDT_0 + queue  4,,
    } else {
    pub qopt->hicredit: tp->tx_qos[queue].hicredit =,
    pub qopt->locredit: tp->tx_qos[queue].locredit =,
    pub qopt->idleslope: tp->tx_qos[queue].idleslope =,
    pub qopt->sendslope: tp->tx_qos[queue].sendslope =,
    pub queue): rtase_set_hw_cbs(tp,,
    }
    pub 0: return,
    }
    static int rtase_setup_tc(struct net_device *dev, enum tc_setup_type type,
    void *type_data)
    {
    pub netdev_priv(dev): *mut *mut rtase_private tp =,
    switch (type) {
    case TC_SETUP_QDISC_CBS:
    pub type_data): return rtase_setup_tc_cbs(tp,,
    default:
    pub -EOPNOTSUPP: return,
    }
    }
    static netdev_features_t rtase_fix_features(struct net_device *dev,
    netdev_features_t features)
    {
    pub features: netdev_features_t features_fix =,
// not support TSO for jumbo frames
    if (dev.mtu > ETH_DATA_LEN)
    pub ~NETIF_F_ALL_TSO: features_fix &=,
    pub features_fix: return,
    }
    static int rtase_set_features(struct net_device *dev,
    netdev_features_t features)
    {
    pub features: netdev_features_t features_set =,
    features_set &= NETIF_F_RXALL | NETIF_F_RXCSUM |
    if (features_set ^ dev.features)
    pub features_set): rtase_hw_set_features(dev,,
    pub 0: return,
    }
    static const struct net_device_ops rtase_netdev_ops = {
    .ndo_open = rtase_open,
    .ndo_stop = rtase_close,
    .ndo_start_xmit = rtase_start_xmit,
    .ndo_set_rx_mode = rtase_set_rx_mode,
    .ndo_set_mac_address = rtase_set_mac_address,
    .ndo_change_mtu = rtase_change_mtu,
    .ndo_tx_timeout = rtase_tx_timeout,
    .ndo_get_stats64 = rtase_get_stats64,
    .ndo_setup_tc = rtase_setup_tc,
    .ndo_fix_features = rtase_fix_features,
    .ndo_set_features = rtase_set_features,
}

#[no_mangle]
unsafe extern "C" fn rtase_get_mac_address(dev: *mut net_device) {
    static void rtase_get_mac_address(struct net_device *dev)
    {
    struct rtase_private *tp = netdev_priv(dev);
    u8 mac_addr[ETH_ALEN] __aligned(2) = {};
    u32 i;
    for (i = 0; i < ETH_ALEN; i++)
    mac_addr[i] = rtase_r8(tp, RTASE_MAC0 + i);
    if (!is_valid_ether_addr(mac_addr)) {
    eth_hw_addr_random(dev);
    netdev_warn(dev, "Random ether addr %pM\n", dev.dev_addr);
    } else {
    eth_hw_addr_set(dev, mac_addr);
    ether_addr_copy(dev.perm_addr, dev.dev_addr);
    }
    rtase_rar_set(tp, dev.dev_addr);
    }
    static int rtase_get_settings(struct net_device *dev,
    struct ethtool_link_ksettings *cmd)
    {
    let mut supported: u32 = SUPPORTED_MII | SUPPORTED_Pause | SUPPORTED_Asym_Pause;
    const struct rtase_private *tp = netdev_priv(dev);
    ethtool_convert_legacy_u32_to_link_mode(cmd.link_modes.supported,
    supported);
    switch (tp.hw_ver) {
    case RTASE_HW_VER_906X_7XA:
    case RTASE_HW_VER_906X_7XC:
    cmd.base.speed = SPEED_5000;
    break;
    case RTASE_HW_VER_907XD_V1:
    case RTASE_HW_VER_907XD_VA:
    cmd.base.speed = SPEED_10000;
    break;
    }
    cmd.base.duplex = DUPLEX_FULL;
    cmd.base.port = PORT_MII;
    cmd.base.autoneg = AUTONEG_DISABLE;
    return 0;
    }
    static void rtase_get_pauseparam(struct net_device *dev,
    struct ethtool_pauseparam *pause)
    {
    const struct rtase_private *tp = netdev_priv(dev);
    let mut value: u16 = rtase_r16(tp, RTASE_CPLUS_CMD);
    pause.autoneg = AUTONEG_DISABLE;
    pause.tx_pause = !!(value & RTASE_FORCE_TXFLOW_EN);
    pause.rx_pause = !!(value & RTASE_FORCE_RXFLOW_EN);
    }
    static int rtase_set_pauseparam(struct net_device *dev,
    struct ethtool_pauseparam *pause)
    {
    const struct rtase_private *tp = netdev_priv(dev);
    let mut value: u16 = rtase_r16(tp, RTASE_CPLUS_CMD);
    if (pause.autoneg)
    return -EOPNOTSUPP;
    value &= ~(RTASE_FORCE_TXFLOW_EN | RTASE_FORCE_RXFLOW_EN);
    if (pause.tx_pause)
    value |= RTASE_FORCE_TXFLOW_EN;
    if (pause.rx_pause)
    value |= RTASE_FORCE_RXFLOW_EN;
    rtase_w16(tp, RTASE_CPLUS_CMD, value);
    return 0;
    }
    static void rtase_get_eth_mac_stats(struct net_device *dev,
    struct ethtool_eth_mac_stats *stats)
    {
    struct rtase_private *tp = netdev_priv(dev);
    const struct rtase_counters *counters;
    counters = tp.tally_vaddr;
    rtase_dump_tally_counter(tp);
    stats.FramesTransmittedOK = le64_to_cpu(counters.tx_packets);
    stats.FramesReceivedOK = le64_to_cpu(counters.rx_packets);
    stats.FramesLostDueToIntMACXmitError =
    le64_to_cpu(counters.tx_errors);
    stats.BroadcastFramesReceivedOK = le64_to_cpu(counters.rx_broadcast);
    }
    static const struct ethtool_ops rtase_ethtool_ops = {
    .get_link = ethtool_op_get_link,
    .get_link_ksettings = rtase_get_settings,
    .get_pauseparam = rtase_get_pauseparam,
    .set_pauseparam = rtase_set_pauseparam,
    .get_eth_mac_stats = rtase_get_eth_mac_stats,
    .get_ts_info = ethtool_op_get_ts_info,
    };
#[no_mangle]
unsafe extern "C" fn rtase_init_netdev_ops(dev: *mut net_device) {
    static void rtase_init_netdev_ops(struct net_device *dev)
    {
    dev.netdev_ops = &rtase_netdev_ops;
    dev.ethtool_ops = &rtase_ethtool_ops;
    }
#[no_mangle]
unsafe extern "C" fn rtase_init_napi(tp: *mut rtase_private) {
    static void rtase_init_napi(struct rtase_private *tp)
    {
    u16 i;
    for (i = 0; i < tp.int_nums; i++) {
    netif_napi_add_config(tp.dev, &tp.int_vector[i].napi,
    tp.int_vector[i].poll, i);
    netif_napi_set_irq(&tp.int_vector[i].napi,
    tp.int_vector[i].irq);
    }
    }
    static void rtase_reset_interrupt(struct pci_dev *pdev,
    const struct rtase_private *tp)
    {
    if (tp.sw_flag & RTASE_SWF_MSIX_ENABLED)
    pci_disable_msix(pdev);
    else
    pci_disable_msi(pdev);
    }
#[no_mangle]
unsafe extern "C" fn rtase_alloc_msix(pdev: *mut pci_dev, tp: *mut rtase_private) -> c_int {
    static int rtase_alloc_msix(struct pci_dev *pdev, struct rtase_private *tp)
    {
    int ret, irq;
    u16 i;
    memset(tp.msix_entry, 0x0, RTASE_NUM_MSIX *
    sizeof(struct msix_entry));
    for (i = 0; i < RTASE_NUM_MSIX; i++)
    tp.msix_entry[i].entry = i;
    ret = pci_enable_msix_exact(pdev, tp.msix_entry, tp.int_nums);
    if (ret)
    return ret;
    for (i = 0; i < tp.int_nums; i++) {
    irq = pci_irq_vector(pdev, i);
    if (irq < 0) {
    pci_disable_msix(pdev);
    return irq;
    }
    tp.int_vector[i].irq = irq;
    }
    return 0;
    }
    static int rtase_alloc_interrupt(struct pci_dev *pdev,
    struct rtase_private *tp)
    {
    int ret;
    ret = rtase_alloc_msix(pdev, tp);
    if (ret) {
    ret = pci_enable_msi(pdev);
    if (ret) {
    dev_err(&pdev.dev,
    "unable to alloc interrupt.(MSI)\n");
    return ret;
    }
    tp.sw_flag |= RTASE_SWF_MSI_ENABLED;
    } else {
    tp.sw_flag |= RTASE_SWF_MSIX_ENABLED;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtase_init_hardware(tp: *const rtase_private) {
    static void rtase_init_hardware(const struct rtase_private *tp)
    {
    u16 i;
    for (i = 0; i < RTASE_VLAN_FILTER_ENTRY_NUM; i++)
    rtase_w32(tp, RTASE_VLAN_ENTRY_0 + i * 4, 0);
    }
#[no_mangle]
unsafe extern "C" fn rtase_init_int_vector(tp: *mut rtase_private) {
    static void rtase_init_int_vector(struct rtase_private *tp)
    {
    u16 i;
// interrupt vector 0
    tp.int_vector[0].tp = tp;
    tp.int_vector[0].index = 0;
    tp.int_vector[0].imr_addr = RTASE_IMR0;
    tp.int_vector[0].isr_addr = RTASE_ISR0;
    tp.int_vector[0].imr = RTASE_ROK | RTASE_RDU | RTASE_TOK |
    RTASE_TOK4 | RTASE_TOK5 | RTASE_TOK6 |
    RTASE_TOK7;
    tp.int_vector[0].poll = rtase_poll;
    memset(tp.int_vector[0].name, 0x0, sizeof(tp.int_vector[0].name));
    INIT_LIST_HEAD(&tp.int_vector[0].ring_list);
// interrupt vector 1 ~ 3
    for (i = 1; i < tp.int_nums; i++) {
    tp.int_vector[i].tp = tp;
    tp.int_vector[i].index = i;
    tp.int_vector[i].imr_addr = RTASE_IMR1 + (i - 1) * 4;
    tp.int_vector[i].isr_addr = RTASE_ISR1 + (i - 1) * 4;
    tp.int_vector[i].imr = RTASE_Q_ROK | RTASE_Q_RDU |
    RTASE_Q_TOK;
    tp.int_vector[i].poll = rtase_poll;
    memset(tp.int_vector[i].name, 0x0,
    sizeof(tp.int_vector[0].name));
    INIT_LIST_HEAD(&tp.int_vector[i].ring_list);
    }
    }
#[no_mangle]
unsafe extern "C" fn rtase_calc_time_mitigation(time_us: u32) -> u16 {
    static u16 rtase_calc_time_mitigation(u32 time_us)
    {
    u8 msb, time_count, time_unit;
    u16 int_miti;
    time_us = min(time_us, RTASE_MITI_MAX_TIME);
    if (time_us > RTASE_MITI_TIME_COUNT_MASK) {
    msb = fls(time_us);
    time_unit = msb - RTASE_MITI_COUNT_BIT_NUM;
    time_count = time_us >> (msb - RTASE_MITI_COUNT_BIT_NUM);
    } else {
    time_unit = 0;
    time_count = time_us;
    }
    int_miti = u16_encode_bits(time_count, RTASE_MITI_TIME_COUNT_MASK) |
    u16_encode_bits(time_unit, RTASE_MITI_TIME_UNIT_MASK);
    return int_miti;
    }
#[no_mangle]
unsafe extern "C" fn rtase_calc_packet_num_mitigation(pkt_num: u16) -> u16 {
    static u16 rtase_calc_packet_num_mitigation(u16 pkt_num)
    {
    u8 msb, pkt_num_count, pkt_num_unit;
    u16 int_miti;
    pkt_num = min(pkt_num, RTASE_MITI_MAX_PKT_NUM);
    if (pkt_num > 60) {
    pkt_num_unit = RTASE_MITI_MAX_PKT_NUM_IDX;
    pkt_num_count = pkt_num / RTASE_MITI_MAX_PKT_NUM_UNIT;
    } else {
    msb = fls(pkt_num);
    if (msb >= RTASE_MITI_COUNT_BIT_NUM) {
    pkt_num_unit = msb - RTASE_MITI_COUNT_BIT_NUM;
    pkt_num_count = pkt_num >> (msb -
    RTASE_MITI_COUNT_BIT_NUM);
    } else {
    pkt_num_unit = 0;
    pkt_num_count = pkt_num;
    }
    }
    int_miti = u16_encode_bits(pkt_num_count,
    RTASE_MITI_PKT_NUM_COUNT_MASK) |
    u16_encode_bits(pkt_num_unit,
    RTASE_MITI_PKT_NUM_UNIT_MASK);
    return int_miti;
    }
    static void rtase_init_software_variable(struct pci_dev *pdev,
    struct rtase_private *tp)
    {
    u16 int_miti;
    tp.tx_queue_ctrl = RTASE_TXQ_CTRL;
    tp.func_tx_queue_num = RTASE_FUNC_TXQ_NUM;
    tp.func_rx_queue_num = RTASE_FUNC_RXQ_NUM;
    tp.int_nums = RTASE_INTERRUPT_NUM;
    int_miti = rtase_calc_time_mitigation(RTASE_MITI_DEFAULT_TIME) |
    rtase_calc_packet_num_mitigation(RTASE_MITI_DEFAULT_PKT_NUM);
    tp.tx_int_mit = int_miti;
    tp.rx_int_mit = int_miti;
    tp.sw_flag = 0;
    rtase_init_int_vector(tp);
// MTU range: 60 - hw-specific max
    tp.dev.min_mtu = ETH_ZLEN;
    tp.dev.max_mtu = RTASE_MAX_JUMBO_SIZE;
    }
#[no_mangle]
unsafe extern "C" fn rtase_check_mac_version_valid(tp: *mut rtase_private) -> c_int {
    static int rtase_check_mac_version_valid(struct rtase_private *tp)
    {
    let mut ret: c_int = -ENODEV;
    tp.hw_ver = rtase_r32(tp, RTASE_TX_CONFIG_0) & RTASE_HW_VER_MASK;
    switch (tp.hw_ver) {
    case RTASE_HW_VER_906X_7XA:
    case RTASE_HW_VER_906X_7XC:
    case RTASE_HW_VER_907XD_V1:
    case RTASE_HW_VER_907XD_VA:
    ret = 0;
    break;
    }
    return ret;
    }
    static int rtase_init_board(struct pci_dev *pdev, struct net_device **dev_out,
    void __iomem **ioaddr_out)
    {
    struct net_device *dev;
    void __iomem *ioaddr;
    let mut ret: c_int = -ENOMEM;
// dev zeroed in alloc_etherdev
    dev = alloc_etherdev_mq(sizeof(struct rtase_private),
    RTASE_FUNC_TXQ_NUM);
    if (!dev)
    goto err_out;
    SET_NETDEV_DEV(dev, &pdev.dev);
    ret = pci_enable_device(pdev);
    if (ret)
    goto err_out_free_dev;
// make sure PCI base addr 1 is MMIO
    if (!(pci_resource_flags(pdev, 2) & IORESOURCE_MEM)) {
    ret = -ENODEV;
    goto err_out_disable;
    }
// check for weird/broken PCI region reporting
    if (pci_resource_len(pdev, 2) < RTASE_REGS_SIZE) {
    ret = -ENODEV;
    goto err_out_disable;
    }
    ret = pci_request_regions(pdev, KBUILD_MODNAME);
    if (ret)
    goto err_out_disable;
    ret = dma_set_mask_and_coherent(&pdev.dev, DMA_BIT_MASK(64));
    if (ret) {
    dev_err(&pdev.dev, "no usable dma addressing method\n");
    goto err_out_free_res;
    }
    pci_set_master(pdev);
// ioremap MMIO region
    ioaddr = ioremap(pci_resource_start(pdev, 2),
    pci_resource_len(pdev, 2));
    if (!ioaddr) {
    ret = -EIO;
    goto err_out_free_res;
    }
// ioaddr_out = ioaddr;
// dev_out = dev;
    return ret;
    err_out_free_res:
    pci_release_regions(pdev);
    err_out_disable:
    pci_disable_device(pdev);
    err_out_free_dev:
    free_netdev(dev);
    err_out:
// ioaddr_out = NULL;
// dev_out = NULL;
    return ret;
    }
    static void rtase_release_board(struct pci_dev *pdev, struct net_device *dev,
    void __iomem *ioaddr)
    {
    const struct rtase_private *tp = netdev_priv(dev);
    rtase_rar_set(tp, tp.dev.perm_addr);
    iounmap(ioaddr);
    if (tp.sw_flag & RTASE_SWF_MSIX_ENABLED)
    pci_disable_msix(pdev);
    else
    pci_disable_msi(pdev);
    pci_release_regions(pdev);
    pci_disable_device(pdev);
    free_netdev(dev);
    }
    static int rtase_init_one(struct pci_dev *pdev,
    const struct pci_device_id *ent)
    {
    struct net_device *dev = core::ptr::null_mut();
    struct rtase_int_vector *ivec;
    void __iomem *ioaddr = core::ptr::null_mut();
    struct rtase_private *tp;
    int ret, i;
    if (!pdev.is_physfn && pdev.is_virtfn) {
    dev_err(&pdev.dev,
    "This module does not support a virtual function.");
    return -EINVAL;
    }
    dev_dbg(&pdev.dev, "Automotive Switch Ethernet driver loaded\n");
    ret = rtase_init_board(pdev, &dev, &ioaddr);
    if (ret)
    return ret;
    tp = netdev_priv(dev);
    tp.mmio_addr = ioaddr;
    tp.dev = dev;
    tp.pdev = pdev;
// identify chip attached to board
    ret = rtase_check_mac_version_valid(tp);
    if (ret) {
    dev_err(&pdev.dev,
    "unknown chip version: 0x%08x, contact rtase maintainers (see MAINTAINERS file)\n",
    tp.hw_ver);
    goto err_out_release_board;
    }
    rtase_init_software_variable(pdev, tp);
    rtase_init_hardware(tp);
    ret = rtase_alloc_interrupt(pdev, tp);
    if (ret) {
    dev_err(&pdev.dev, "unable to alloc MSIX/MSI\n");
    goto err_out_del_napi;
    }
    rtase_init_napi(tp);
    rtase_init_netdev_ops(dev);
    dev.pcpu_stat_type = NETDEV_PCPU_STAT_TSTATS;
    dev.features |= NETIF_F_HW_VLAN_CTAG_TX | NETIF_F_HW_VLAN_CTAG_RX |
    NETIF_F_IP_CSUM | NETIF_F_HIGHDMA |
    NETIF_F_RXCSUM | NETIF_F_SG |
    NETIF_F_TSO | NETIF_F_IPV6_CSUM |
    NETIF_F_TSO6;
    dev.hw_features = NETIF_F_SG | NETIF_F_IP_CSUM |
    NETIF_F_TSO | NETIF_F_RXCSUM |
    NETIF_F_HW_VLAN_CTAG_TX | NETIF_F_HW_VLAN_CTAG_RX |
    NETIF_F_RXALL | NETIF_F_RXFCS |
    NETIF_F_IPV6_CSUM | NETIF_F_TSO6;
    dev.vlan_features = NETIF_F_SG | NETIF_F_IP_CSUM | NETIF_F_TSO |
    NETIF_F_HIGHDMA;
    dev.priv_flags |= IFF_LIVE_ADDR_CHANGE;
    netif_set_tso_max_size(dev, RTASE_LSO_64K);
    netif_set_tso_max_segs(dev, RTASE_NIC_MAX_PHYS_BUF_COUNT_LSO2);
    rtase_get_mac_address(dev);
    tp.tally_vaddr = dma_alloc_coherent(&pdev.dev,
    sizeof(*tp.tally_vaddr),
    &tp.tally_paddr,
    GFP_KERNEL);
    if (!tp.tally_vaddr) {
    ret = -ENOMEM;
    goto err_out_free_dma;
    }
    rtase_tally_counter_clear(tp);
    pci_set_drvdata(pdev, dev);
    netif_carrier_off(dev);
    ret = register_netdev(dev);
    if (ret)
    goto err_out_free_dma;
    netdev_dbg(dev, "%pM, IRQ %d\n", dev.dev_addr, dev.irq);
    return 0;
    err_out_free_dma:
    if (tp.tally_vaddr) {
    dma_free_coherent(&pdev.dev,
    sizeof(*tp.tally_vaddr),
    tp.tally_vaddr,
    tp.tally_paddr);
    tp.tally_vaddr = core::ptr::null_mut();
    }
    err_out_del_napi:
    for (i = 0; i < tp.int_nums; i++) {
    ivec = &tp.int_vector[i];
    netif_napi_del(&ivec.napi);
    }
    err_out_release_board:
    rtase_release_board(pdev, dev, ioaddr);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rtase_remove_one(pdev: *mut pci_dev) {
    static void rtase_remove_one(struct pci_dev *pdev)
    {
    struct net_device *dev = pci_get_drvdata(pdev);
    struct rtase_private *tp = netdev_priv(dev);
    struct rtase_int_vector *ivec;
    u32 i;
    unregister_netdev(dev);
    for (i = 0; i < tp.int_nums; i++) {
    ivec = &tp.int_vector[i];
    netif_napi_del(&ivec.napi);
    }
    rtase_reset_interrupt(pdev, tp);
    if (tp.tally_vaddr) {
    dma_free_coherent(&pdev.dev,
    sizeof(*tp.tally_vaddr),
    tp.tally_vaddr,
    tp.tally_paddr);
    tp.tally_vaddr = core::ptr::null_mut();
    }
    rtase_release_board(pdev, dev, tp.mmio_addr);
    pci_set_drvdata(pdev, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn rtase_shutdown(pdev: *mut pci_dev) {
    static void rtase_shutdown(struct pci_dev *pdev)
    {
    struct net_device *dev = pci_get_drvdata(pdev);
    const struct rtase_private *tp;
    tp = netdev_priv(dev);
    if (netif_running(dev))
    rtase_close(dev);
    rtase_reset_interrupt(pdev, tp);
    }
#[no_mangle]
unsafe extern "C" fn rtase_suspend(device: *mut device) -> c_int {
    static int rtase_suspend(struct device *device)
    {
    struct net_device *dev = dev_get_drvdata(device);
    if (netif_running(dev)) {
    netif_device_detach(dev);
    rtase_hw_reset(dev);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtase_resume(device: *mut device) -> c_int {
    static int rtase_resume(struct device *device)
    {
    struct net_device *dev = dev_get_drvdata(device);
    struct rtase_private *tp = netdev_priv(dev);
    int ret;
// restore last modified mac address
    rtase_rar_set(tp, dev.dev_addr);
    if (!netif_running(dev))
    goto out;
    rtase_wait_for_quiescence(dev);
    rtase_tx_clear(tp);
    rtase_rx_clear(tp);
    ret = rtase_init_ring(dev);
    if (ret) {
    netdev_err(dev, "unable to init ring\n");
    rtase_free_desc(tp);
    return -ENOMEM;
    }
    rtase_hw_config(dev);
// always link, so start to transmit & receive
    rtase_hw_start(dev);
    netif_device_attach(dev);
    out:
    return 0;
    }
    static const struct dev_pm_ops rtase_pm_ops = {
    SYSTEM_SLEEP_PM_OPS(rtase_suspend, rtase_resume)
    };
    static struct pci_driver rtase_pci_driver = {
    .name = KBUILD_MODNAME,
    .id_table = rtase_pci_tbl,
    .probe = rtase_init_one,
    .remove = rtase_remove_one,
    .shutdown = rtase_shutdown,
    .driver.pm = pm_ptr(&rtase_pm_ops),
    };
    module_pci_driver(rtase_pci_driver);
