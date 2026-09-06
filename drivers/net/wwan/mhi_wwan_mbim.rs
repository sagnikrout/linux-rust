//! Automatically rewritten from C to Rust
//! Source: drivers/net/wwan/mhi_wwan_mbim.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
// MHI MBIM Network driver - Network/MBIM over MHI bus
//
// Copyright (C) 2021 Linaro Ltd <loic.poulain@linaro.org>
//
// This driver copy some code from cdc_ncm, which is:
// Copyright (C) ST-Ericsson 2010-2012
// and cdc_mbim, which is:
// Copyright (c) 2012  Smith Micro Software, Inc.
// Copyright (c) 2012  Bjørn Mork <bjorn@mork.no>
//

// 3500 allows to optimize skb allocation, the skbs will basically fit in
// one 4K page. Large MBIM packets will simply be split over several MHI
// transfers and chained by the MHI net layer (zerocopy).
//
pub const MHI_DEFAULT_MRU: c_int = 3500;
pub const MHI_MBIM_DEFAULT_MTU: c_int = 1500;
pub const MHI_MAX_BUF_SZ: c_uint = 0xffff;
pub const MBIM_NDP16_SIGN_MASK: c_uint = 0x00ffffff;
pub const MHI_MBIM_LINK_HASH_SIZE: c_int = 8;

pub const WDS_BIND_MUX_DATA_PORT_MUX_ID: c_int = 112;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhi_mbim_link {
    pub mbim: *mut mhi_mbim_context,
    pub ndev: *mut net_device,
    pub session: c_uint,
// stats
    pub rx_packets: u64_stats_t,
    pub rx_bytes: u64_stats_t,
    pub rx_errors: u64_stats_t,
    pub tx_packets: u64_stats_t,
    pub tx_bytes: u64_stats_t,
    pub tx_errors: u64_stats_t,
    pub tx_dropped: u64_stats_t,
    pub tx_syncp: u64_stats_sync,
    pub rx_syncp: u64_stats_sync,
    pub hlnode: hlist_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mhi_mbim_context {
    pub mdev: *mut mhi_device,
    pub skbagg_head: *mut sk_buff,
    pub skbagg_tail: *mut sk_buff,
    pub mru: c_uint,
    pub rx_queue_sz: u32,
    pub rx_seq: u16,
    pub tx_seq: u16,
    pub rx_refill: delayed_work,
    pub tx_lock: spinlock_t,
    pub link_list: [hlist_head; MHI_MBIM_LINK_HASH_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mbim_tx_hdr {
    pub nth16: usb_cdc_ncm_nth16,
    pub ndp16: usb_cdc_ncm_ndp16,
    pub dpe16: [usb_cdc_ncm_dpe16; 2],
    pub __packed: },
    static struct mhi_mbim_link *mhi_mbim_get_link_rcu(struct mhi_mbim_context *mbim,
    unsigned int session)
    {
    pub link: *mut mhi_mbim_link,
    hlist_for_each_entry_rcu(link, &mbim.link_list[LINK_HASH(session)], hlnode) {
    if (link.session == session)
    pub link: return,
    }
    pub NULL: return,
    }
#[no_mangle]
unsafe extern "C" fn mhi_mbim_get_link_mux_id(cntrl: *mut mhi_controller) -> c_int {
    static int mhi_mbim_get_link_mux_id(struct mhi_controller *cntrl)
    {
    if (strcmp(cntrl.name, "foxconn-dw5934e") == 0 ||
    strcmp(cntrl.name, "foxconn-t99w640") == 0 ||
    strcmp(cntrl.name, "foxconn-t99w760") == 0)
    pub WDS_BIND_MUX_DATA_PORT_MUX_ID: return,
    pub 0: return,
    }
    static struct sk_buff *mbim_tx_fixup(struct sk_buff *skb, unsigned int session,
    u16 tx_seq)
    {
    pub skb->len: unsigned int dgram_size =,
    pub nth16: *mut usb_cdc_ncm_nth16,
    pub ndp16: *mut usb_cdc_ncm_ndp16,
    pub mbim_hdr: *mut mbim_tx_hdr,
// Only one NDP is sent, containing the IP packet (no aggregation)
// Ensure we have enough headroom for crafting MBIM header
    if (skb_cow_head(skb, sizeof(struct mbim_tx_hdr))) {
    pub NULL: return,
    }
    pub mbim_tx_hdr)): mbim_hdr = skb_push(skb, sizeof(struct,
// Fill NTB header
    pub &mbim_hdr->nth16: nth16 =,
    pub cpu_to_le32(USB_CDC_NCM_NTH16_SIGN): nth16->dwSignature =,
    pub usb_cdc_ncm_nth16)): nth16->wHeaderLength = cpu_to_le16(sizeof(struct,
    pub cpu_to_le16(tx_seq): nth16->wSequence =,
    pub cpu_to_le16(skb->len): nth16->wBlockLength =,
    pub usb_cdc_ncm_nth16)): nth16->wNdpIndex = cpu_to_le16(sizeof(struct,
// Fill the unique NDP
    pub &mbim_hdr->ndp16: ndp16 =,
    pub 24)): ndp16->dwSignature = cpu_to_le32(USB_CDC_MBIM_NDP16_IPS_SIGN | (session <<,
    ndp16.wLength = cpu_to_le16(sizeof(struct usb_cdc_ncm_ndp16)
    pub 2): *mut *mut + sizeof(struct usb_cdc_ncm_dpe16),
    pub 0: ndp16->wNextNdpIndex =,
// Datagram follows the mbim header
    pub mbim_tx_hdr)): ndp16->dpe16[0].wDatagramIndex = cpu_to_le16(sizeof(struct,
    pub cpu_to_le16(dgram_size): ndp16->dpe16[0].wDatagramLength =,
// null termination
    pub 0: ndp16->dpe16[1].wDatagramIndex =,
    pub 0: ndp16->dpe16[1].wDatagramLength =,
    pub skb: return,
    }
#[no_mangle]
unsafe extern "C" fn mhi_mbim_ndo_xmit(skb: *mut sk_buff, ndev: *mut net_device) -> netdev_tx_t {
    static netdev_tx_t mhi_mbim_ndo_xmit(struct sk_buff *skb, struct net_device *ndev)
    {
    pub wwan_netdev_drvpriv(ndev): *mut *mut mhi_mbim_link link =,
    pub link->mbim: *mut *mut mhi_mbim_context mbim =,
    pub flags: c_ulong,
    pub -ENOMEM: int err =,
// Serialize MHI channel queuing and MBIM seq
    pub flags): spin_lock_irqsave(&mbim->tx_lock,,
    pub mbim->tx_seq): skb = mbim_tx_fixup(skb, link->session,,
    if (unlikely(!skb))
    pub exit_unlock: goto,
    pub MHI_EOT): err = mhi_queue_skb(mbim->mdev, DMA_TO_DEVICE, skb, skb->len,,
    if (mhi_queue_is_full(mbim.mdev, DMA_TO_DEVICE))
    if (!err)
    exit_unlock:
    pub flags): spin_unlock_irqrestore(&mbim->tx_lock,,
    if (unlikely(err)) {
    net_err_ratelimited("%s: Failed to queue TX buf (%d)\n",
    pub err): ndev->name,,
    pub exit_drop: goto,
    }
    pub NETDEV_TX_OK: return,
    exit_drop:
    pub NETDEV_TX_OK: return,
    }
#[no_mangle]
unsafe extern "C" fn mbim_rx_verify_nth16(mbim: *mut mhi_mbim_context, skb: *mut sk_buff) -> c_int {
    static int mbim_rx_verify_nth16(struct mhi_mbim_context *mbim, struct sk_buff *skb)
    {
    pub nth16: *mut usb_cdc_ncm_nth16,
    pub len: c_int,
    if (skb.len < sizeof(struct usb_cdc_ncm_nth16) +
    sizeof(struct usb_cdc_ncm_ndp16)) {
    pub short\n"): net_err_ratelimited("frame too,
    pub -EINVAL: return,
    }
    pub )skb->data: *mut nth16 = (struct usb_cdc_ncm_nth16,
    if (nth16.dwSignature != cpu_to_le32(USB_CDC_NCM_NTH16_SIGN)) {
    net_err_ratelimited("invalid NTH16 signature <%#010x>\n",
    pub -EINVAL: return,
    }
// No limit on the block length, except the size of the data pkt
    pub le16_to_cpu(nth16->wBlockLength): len =,
    if (len > skb.len) {
    net_err_ratelimited("NTB does not fit into the skb %u/%u\n",
    pub skb->len): len,,
    pub -EINVAL: return,
    }
    if (mbim.rx_seq + 1 != le16_to_cpu(nth16.wSequence) &&
    (mbim.rx_seq || le16_to_cpu(nth16.wSequence)) &&
    !(mbim.rx_seq == 0xffff && !le16_to_cpu(nth16.wSequence))) {
    net_dbg_ratelimited("sequence number glitch prev=%d curr=%d\n",
    pub le16_to_cpu(nth16->wSequence)): mbim->rx_seq,,
    }
    pub le16_to_cpu(nth16->wSequence): mbim->rx_seq =,
    pub le16_to_cpu(nth16->wNdpIndex): return,
    }
#[no_mangle]
unsafe extern "C" fn mbim_rx_verify_ndp16(skb: *mut sk_buff, ndp16: *mut usb_cdc_ncm_ndp16) -> c_int {
    static int mbim_rx_verify_ndp16(struct sk_buff *skb, struct usb_cdc_ncm_ndp16 *ndp16)
    {
    pub ret: c_int,
    if (le16_to_cpu(ndp16.wLength) < USB_CDC_NCM_NDP16_LENGTH_MIN) {
    net_err_ratelimited("invalid DPT16 length <%u>\n",
    pub -EINVAL: return,
    }
    ret = ((le16_to_cpu(ndp16.wLength) - sizeof(struct usb_cdc_ncm_ndp16))
    pub usb_cdc_ncm_dpe16)): / sizeof(struct,
    pub /: *mut *mut ret--; / Last entry is always a NULL terminator,
    if (sizeof(struct usb_cdc_ncm_ndp16) +
#[no_mangle]
pub unsafe extern "C" fn sizeof(skb->len: usb_cdc_ncm_dpe16) >) -> *mut ret {
    pub ret): net_err_ratelimited("Invalid nframes = %d\n",,
    pub -EINVAL: return,
    }
    pub ret: return,
    }
#[no_mangle]
unsafe extern "C" fn mhi_mbim_rx(mbim: *mut mhi_mbim_context, skb: *mut sk_buff) {
    static void mhi_mbim_rx(struct mhi_mbim_context *mbim, struct sk_buff *skb)
    {
    pub ndpoffset: c_int,
// Check NTB header and retrieve first NDP offset
    pub skb): ndpoffset = mbim_rx_verify_nth16(mbim,,
    if (ndpoffset < 0) {
    pub header\n"): net_err_ratelimited("mbim: Incorrect NTB,
    pub error: goto,
    }
// Process each NDP
    while (1) {
    pub ndp16: usb_cdc_ncm_ndp16,
    pub dpe16: usb_cdc_ncm_dpe16,
    pub link: *mut mhi_mbim_link,
    pub dpeoffset: int nframes, n,,
    pub session: c_uint,
    if (skb_copy_bits(skb, ndpoffset, &ndp16, sizeof(ndp16))) {
    net_err_ratelimited("mbim: Incorrect NDP offset (%u)\n",
    pub error: goto,
    }
// Check NDP header and retrieve number of datagrams
    pub &ndp16): nframes = mbim_rx_verify_ndp16(skb,,
    if (nframes < 0) {
    pub NDP16\n"): net_err_ratelimited("mbim: Incorrect,
    pub error: goto,
    }
// Only IP data type supported, no DSS in MHI context
    if ((ndp16.dwSignature & cpu_to_le32(MBIM_NDP16_SIGN_MASK))
    != cpu_to_le32(USB_CDC_MBIM_NDP16_IPS_SIGN)) {
    pub type\n"): net_err_ratelimited("mbim: Unsupported NDP,
    pub next_ndp: goto,
    }
    pub 24: session = (le32_to_cpu(ndp16.dwSignature) & ~MBIM_NDP16_SIGN_MASK) >>,
    pub session): link = mhi_mbim_get_link_rcu(mbim,,
    if (!link) {
    pub session): net_err_ratelimited("mbim: bad packet session (%u)\n",,
    pub unlock: goto,
    }
// de-aggregate and deliver IP packets
    pub usb_cdc_ncm_ndp16): dpeoffset = ndpoffset + sizeof(struct,
    pub {: for (n = 0; n < nframes; n++, dpeoffset += sizeof(dpe16)),
    pub dgram_len: u16 dgram_offset,,
    pub skbn: *mut sk_buff,
    if (skb_copy_bits(skb, dpeoffset, &dpe16, sizeof(dpe16)))
    pub le16_to_cpu(dpe16.wDatagramIndex): dgram_offset =,
    pub le16_to_cpu(dpe16.wDatagramLength): dgram_len =,
    if (!dgram_offset || !dgram_len)
    pub /: *mut *mut break; / null terminator,
    pub dgram_len): skbn = netdev_alloc_skb(link->ndev,,
    if (!skbn)
    pub dgram_len): skb_put(skbn,,
    pub dgram_len): skb_copy_bits(skb, dgram_offset, skbn->data,,
    switch (skbn.data[0] & 0xf0) {
    case 0x40:
    pub htons(ETH_P_IP): skbn->protocol =,
    case 0x60:
    pub htons(ETH_P_IPV6): skbn->protocol =,
    default:
    net_err_ratelimited("%s: unknown protocol\n",
    }
    pub skbn->len): u64_stats_add(&link->rx_bytes,,
    }
    unlock:
    next_ndp:
// Other NDP to process?
    pub (int)le16_to_cpu(ndp16.wNextNdpIndex): ndpoffset =,
    if (!ndpoffset)
    }
// free skb
    error:
    }
    static struct sk_buff *mhi_net_skb_agg(struct mhi_mbim_context *mbim,
    struct sk_buff *skb)
    {
    pub mbim->skbagg_head: *mut *mut sk_buff head =,
    pub mbim->skbagg_tail: *mut *mut sk_buff tail =,
// This is non-paged skb chaining using frag_list
    if (!head) {
    pub skb: mbim->skbagg_head =,
    pub skb: return,
    }
    if (!skb_shinfo(head).frag_list)
    pub skb: skb_shinfo(head)->frag_list =,
    else
    pub skb: tail->next =,
    pub skb->len: head->len +=,
    pub skb->len: head->data_len +=,
    pub skb->truesize: head->truesize +=,
    pub skb: mbim->skbagg_tail =,
    pub mbim->skbagg_head: return,
    }
#[no_mangle]
unsafe extern "C" fn mhi_net_rx_refill_work(work: *mut work_struct) {
    static void mhi_net_rx_refill_work(struct work_struct *work)
    {
    struct mhi_mbim_context *mbim = container_of(work, struct mhi_mbim_context,
    pub mbim->mdev: *mut *mut mhi_device mdev =,
    pub err: c_int,
    while (!mhi_queue_is_full(mdev, DMA_FROM_DEVICE)) {
    pub GFP_KERNEL): *mut *mut sk_buff skb = alloc_skb(mbim->mru,,
    if (unlikely(!skb))
    err = mhi_queue_skb(mdev, DMA_FROM_DEVICE, skb,
    pub MHI_EOT): mbim->mru,,
    if (unlikely(err)) {
    }
// Do not hog the CPU if rx buffers are consumed faster than
// queued (unlikely).
//
    }
// If we're still starved of rx buffers, reschedule later
    if (mhi_get_free_desc_count(mdev, DMA_FROM_DEVICE) == mbim.rx_queue_sz)
    pub 2): schedule_delayed_work(&mbim->rx_refill, HZ /,
    }
    static void mhi_mbim_dl_callback(struct mhi_device *mhi_dev,
    struct mhi_result *mhi_res)
    {
    pub dev_get_drvdata(&mhi_dev->dev): *mut *mut mhi_mbim_context mbim =,
    pub mhi_res->buf_addr: *mut *mut sk_buff skb =,
    pub free_desc_count: c_int,
    pub DMA_FROM_DEVICE): free_desc_count = mhi_get_free_desc_count(mhi_dev,,
    if (unlikely(mhi_res.transaction_status)) {
    switch (mhi_res.transaction_status) {
    case -EOVERFLOW:
// Packet has been split over multiple transfers
    pub mhi_res->bytes_xferd): skb_put(skb,,
    pub skb): mhi_net_skb_agg(mbim,,
    case -ENOTCONN:
// MHI layer stopping/resetting the DL channel
    default:
// Unknown error, simply drop
    }
    } else {
    pub mhi_res->bytes_xferd): skb_put(skb,,
    if (mbim.skbagg_head) {
// Aggregate the final fragment
    pub skb): skb = mhi_net_skb_agg(mbim,,
    pub NULL: mbim->skbagg_head =,
    }
    pub skb): mhi_mbim_rx(mbim,,
    }
// Refill if RX buffers queue becomes low
    if (free_desc_count >= mbim.rx_queue_sz / 2)
    pub 0): schedule_delayed_work(&mbim->rx_refill,,
    }
    static void mhi_mbim_ndo_get_stats64(struct net_device *ndev,
    struct rtnl_link_stats64 *stats)
    {
    pub wwan_netdev_drvpriv(ndev): *mut *mut mhi_mbim_link link =,
    pub start: c_uint,
    do {
    pub u64_stats_fetch_begin(&link->rx_syncp): start =,
    pub u64_stats_read(&link->rx_packets): stats->rx_packets =,
    pub u64_stats_read(&link->rx_bytes): stats->rx_bytes =,
    pub u64_stats_read(&link->rx_errors): stats->rx_errors =,
    pub start)): } while (u64_stats_fetch_retry(&link->rx_syncp,,
    do {
    pub u64_stats_fetch_begin(&link->tx_syncp): start =,
    pub u64_stats_read(&link->tx_packets): stats->tx_packets =,
    pub u64_stats_read(&link->tx_bytes): stats->tx_bytes =,
    pub u64_stats_read(&link->tx_errors): stats->tx_errors =,
    pub u64_stats_read(&link->tx_dropped): stats->tx_dropped =,
    pub start)): } while (u64_stats_fetch_retry(&link->tx_syncp,,
    }
    static void mhi_mbim_ul_callback(struct mhi_device *mhi_dev,
    struct mhi_result *mhi_res)
    {
    pub dev_get_drvdata(&mhi_dev->dev): *mut *mut mhi_mbim_context mbim =,
    pub mhi_res->buf_addr: *mut *mut sk_buff skb =,
    pub skb->dev: *mut *mut net_device ndev =,
    pub wwan_netdev_drvpriv(ndev): *mut *mut mhi_mbim_link link =,
// Hardware has consumed the buffer, so free the skb (which is not
// freed by the MHI stack) and perform accounting.
//
    if (unlikely(mhi_res.transaction_status)) {
// MHI layer stopping/resetting the UL channel
    if (mhi_res.transaction_status == -ENOTCONN) {
    }
    } else {
    pub mhi_res->bytes_xferd): u64_stats_add(&link->tx_bytes,,
    }
    if (netif_queue_stopped(ndev) && !mhi_queue_is_full(mbim.mdev, DMA_TO_DEVICE))
    }
#[no_mangle]
unsafe extern "C" fn mhi_mbim_ndo_open(ndev: *mut net_device) -> c_int {
    static int mhi_mbim_ndo_open(struct net_device *ndev)
    {
    pub wwan_netdev_drvpriv(ndev): *mut *mut mhi_mbim_link link =,
// Feed the MHI rx buffer pool
    pub 0): schedule_delayed_work(&link->mbim->rx_refill,,
// Carrier is established via out-of-band channel (e.g. qmi)
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn mhi_mbim_ndo_stop(ndev: *mut net_device) -> c_int {
    static int mhi_mbim_ndo_stop(struct net_device *ndev)
    {
    pub 0: return,
    }
    static const struct net_device_ops mhi_mbim_ndo = {
    .ndo_open = mhi_mbim_ndo_open,
    .ndo_stop = mhi_mbim_ndo_stop,
    .ndo_start_xmit = mhi_mbim_ndo_xmit,
    .ndo_get_stats64 = mhi_mbim_ndo_get_stats64,
}

    static int mhi_mbim_newlink(void *ctxt, struct net_device *ndev, u32 if_id,
    struct netlink_ext_ack *extack)
    {
    struct mhi_mbim_link *link = wwan_netdev_drvpriv(ndev);
    struct mhi_mbim_context *mbim = ctxt;
    link.mbim = mbim;
    link.session = mhi_mbim_get_link_mux_id(link.mbim.mdev.mhi_cntrl) + if_id;
    link.ndev = ndev;
    u64_stats_init(&link.rx_syncp);
    u64_stats_init(&link.tx_syncp);
    rcu_read_lock();
    if (mhi_mbim_get_link_rcu(mbim, if_id)) {
    rcu_read_unlock();
    return -EEXIST;
    }
    rcu_read_unlock();
// Already protected by RTNL lock
    hlist_add_head_rcu(&link.hlnode, &mbim.link_list[LINK_HASH(if_id)]);
    return register_netdevice(ndev);
    }
    static void mhi_mbim_dellink(void *ctxt, struct net_device *ndev,
    struct list_head *head)
    {
    struct mhi_mbim_link *link = wwan_netdev_drvpriv(ndev);
    hlist_del_init_rcu(&link.hlnode);
    synchronize_rcu();
    unregister_netdevice_queue(ndev, head);
    }
#[no_mangle]
unsafe extern "C" fn mhi_mbim_setup(ndev: *mut net_device) {
    static void mhi_mbim_setup(struct net_device *ndev)
    {
    ndev.header_ops = core::ptr::null_mut();  /* No header */
    ndev.type = ARPHRD_RAWIP;
    ndev.needed_headroom = sizeof(struct mbim_tx_hdr);
    ndev.hard_header_len = 0;
    ndev.addr_len = 0;
    ndev.flags = IFF_POINTOPOINT | IFF_NOARP;
    ndev.netdev_ops = &mhi_mbim_ndo;
    ndev.mtu = MHI_MBIM_DEFAULT_MTU;
    ndev.min_mtu = ETH_MIN_MTU;
    ndev.max_mtu = MHI_MAX_BUF_SZ - ndev.needed_headroom;
    ndev.tx_queue_len = 1000;
    ndev.needs_free_netdev = true;
    }
    static const struct wwan_ops mhi_mbim_wwan_ops = {
    .priv_size = sizeof(struct mhi_mbim_link),
    .setup = mhi_mbim_setup,
    .newlink = mhi_mbim_newlink,
    .dellink = mhi_mbim_dellink,
    };
#[no_mangle]
unsafe extern "C" fn mhi_mbim_probe(mhi_dev: *mut mhi_device, id: *const mhi_device_id) -> c_int {
    static int mhi_mbim_probe(struct mhi_device *mhi_dev, const struct mhi_device_id *id)
    {
    struct mhi_controller *cntrl = mhi_dev.mhi_cntrl;
    struct mhi_mbim_context *mbim;
    int err;
    mbim = devm_kzalloc(&mhi_dev.dev, sizeof(*mbim), GFP_KERNEL);
    if (!mbim)
    return -ENOMEM;
    spin_lock_init(&mbim.tx_lock);
    dev_set_drvdata(&mhi_dev.dev, mbim);
    mbim.mdev = mhi_dev;
    mbim.mru = mhi_dev.mhi_cntrl.mru ? mhi_dev.mhi_cntrl.mru : MHI_DEFAULT_MRU;
    INIT_DELAYED_WORK(&mbim.rx_refill, mhi_net_rx_refill_work);
// Start MHI channels
    err = mhi_prepare_for_transfer(mhi_dev);
    if (err)
    return err;
// Number of transfer descriptors determines size of the queue
    mbim.rx_queue_sz = mhi_get_free_desc_count(mhi_dev, DMA_FROM_DEVICE);
// Register wwan link ops with MHI controller representing WWAN instance
    return wwan_register_ops(&cntrl.mhi_dev.dev, &mhi_mbim_wwan_ops, mbim, 0);
    }
#[no_mangle]
unsafe extern "C" fn mhi_mbim_remove(mhi_dev: *mut mhi_device) {
    static void mhi_mbim_remove(struct mhi_device *mhi_dev)
    {
    struct mhi_mbim_context *mbim = dev_get_drvdata(&mhi_dev.dev);
    struct mhi_controller *cntrl = mhi_dev.mhi_cntrl;
    mhi_unprepare_from_transfer(mhi_dev);
    cancel_delayed_work_sync(&mbim.rx_refill);
    wwan_unregister_ops(&cntrl.mhi_dev.dev);
    kfree_skb(mbim.skbagg_head);
    dev_set_drvdata(&mhi_dev.dev, core::ptr::null_mut());
    }
    static const struct mhi_device_id mhi_mbim_id_table[] = {
// Hardware accelerated data PATH (to modem IPA), MBIM protocol
    { .chan = "IP_HW0_MBIM", .driver_data = 0 },
    {}
    };
    MODULE_DEVICE_TABLE(mhi, mhi_mbim_id_table);
    static struct mhi_driver mhi_mbim_driver = {
    .probe = mhi_mbim_probe,
    .remove = mhi_mbim_remove,
    .dl_xfer_cb = mhi_mbim_dl_callback,
    .ul_xfer_cb = mhi_mbim_ul_callback,
    .id_table = mhi_mbim_id_table,
    .driver = {
    .name = "mhi_wwan_mbim",
    },
    };
    module_mhi_driver(mhi_mbim_driver);
    MODULE_AUTHOR("Loic Poulain <loic.poulain@linaro.org>");
    MODULE_DESCRIPTION("Network/MBIM over MHI");
    MODULE_LICENSE("GPL v2");
