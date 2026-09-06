//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/vertexcom/mse102x.c
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
// Copyright (C) 2021 in-tech smart charging GmbH
//
// driver is based on micrel/ks8851_spi.c
//

    NETIF_MSG_TIMER)

pub const DET_CMD: c_uint = 0x0001;
pub const DET_SOF: c_uint = 0x0002;
pub const DET_DFT: c_uint = 0x55AA;
pub const CMD_SHIFT: c_int = 12;

pub const DET_CMD_LEN: c_int = 4;
pub const DET_SOF_LEN: c_int = 2;
pub const DET_DFT_LEN: c_int = 2;
pub const MIN_FREQ_HZ: c_int = 6000000;
pub const MAX_FREQ_HZ: c_int = 7142857;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mse102x_stats {
    pub xfer_err: u64,
    pub invalid_ctr: u64,
    pub invalid_dft: u64,
    pub invalid_len: u64,
    pub invalid_rts: u64,
    pub invalid_sof: u64,
    pub tx_timeout: u64,
}

    static const char mse102x_gstrings_stats[][ETH_GSTRING_LEN] = {
    "SPI transfer errors",
    "Invalid CTR",
    "Invalid DFT",
    "Invalid frame length",
    "Invalid RTS",
    "Invalid SOF",
    "TX timeout",
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mse102x_net {
    pub ndev: *mut net_device,
    pub rxd: [u8; 8],
    pub txd: [u8; 8],
    pub ____cacheline_aligned: u32 msg_enable,
    pub txq: sk_buff_head,
    pub stats: mse102x_stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mse102x_net_spi {
    pub mse102x: mse102x_net,
    pub /: *mut *mut mutex lock; / Protect SPI frame transfer,
    pub tx_work: work_struct,
    pub spidev: *mut spi_device,
    pub spi_msg: spi_message,
    pub spi_xfer: spi_transfer,
    pub valid_cmd_received: bool,

    pub device_root: *mut dentry,

}

#[no_mangle]
unsafe extern "C" fn mse102x_info_show(s: *mut seq_file, what: *mut c_void) -> c_int {
    static int mse102x_info_show(struct seq_file *s, void *what)
    {
    struct mse102x_net_spi *mses = s.private;
    seq_printf(s, "TX ring size            : %u\n",
    skb_queue_len(&mses.mse102x.txq));
    seq_printf(s, "IRQ                     : %d\n",
    mses.spidev.irq);
    seq_printf(s, "SPI effective speed     : %lu\n",
    (unsigned long)mses.spi_xfer.effective_speed_hz);
    seq_printf(s, "SPI mode                : %x\n",
    mses.spidev.mode);
    seq_printf(s, "Received valid CMD once : %s\n",
    str_yes_no(mses.valid_cmd_received));
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(mse102x_info);
#[no_mangle]
unsafe extern "C" fn mse102x_init_device_debugfs(mses: *mut mse102x_net_spi) {
    static void mse102x_init_device_debugfs(struct mse102x_net_spi *mses)
    {
    mses.device_root = debugfs_create_dir(dev_name(&mses.mse102x.ndev.dev),
    core::ptr::null_mut());
    debugfs_create_file("info", S_IFREG | 0444, mses.device_root, mses,
    &mse102x_info_fops);
    }
#[no_mangle]
unsafe extern "C" fn mse102x_remove_device_debugfs(mses: *mut mse102x_net_spi) {
    static void mse102x_remove_device_debugfs(struct mse102x_net_spi *mses)
    {
    debugfs_remove_recursive(mses.device_root);
    }

#[no_mangle]
unsafe extern "C" fn mse102x_init_device_debugfs(mses: *mut mse102x_net_spi) {
    static void mse102x_init_device_debugfs(struct mse102x_net_spi *mses)
    {
    }
#[no_mangle]
unsafe extern "C" fn mse102x_remove_device_debugfs(mses: *mut mse102x_net_spi) {
    static void mse102x_remove_device_debugfs(struct mse102x_net_spi *mses)
    {
    }

// SPI register read/write calls.
//
// All these calls issue SPI transactions to access the chip's registers. They
// all require that the necessary lock is held to prevent accesses when the
// chip is busy transferring packet data.
//
#[no_mangle]
unsafe extern "C" fn mse102x_tx_cmd_spi(mse: *mut mse102x_net, cmd: u16) {
    static void mse102x_tx_cmd_spi(struct mse102x_net *mse, u16 cmd)
    {
    struct mse102x_net_spi *mses = to_mse102x_spi(mse);
    struct spi_transfer *xfer = &mses.spi_xfer;
    struct spi_message *msg = &mses.spi_msg;
    __be16 txb[2];
    int ret;
    txb[0] = cpu_to_be16(DET_CMD);
    txb[1] = cpu_to_be16(cmd);
    xfer.tx_buf = txb;
    xfer.rx_buf = core::ptr::null_mut();
    xfer.len = DET_CMD_LEN;
    ret = spi_sync(mses.spidev, msg);
    if (ret < 0) {
    netdev_err(mse.ndev, "%s: spi_sync() failed: %d\n",
    __func__, ret);
    mse.stats.xfer_err++;
    }
    }
#[no_mangle]
unsafe extern "C" fn mse102x_rx_cmd_spi(mse: *mut mse102x_net, rxb: *mut u8) -> c_int {
    static int mse102x_rx_cmd_spi(struct mse102x_net *mse, u8 *rxb)
    {
    struct mse102x_net_spi *mses = to_mse102x_spi(mse);
    struct spi_transfer *xfer = &mses.spi_xfer;
    struct spi_message *msg = &mses.spi_msg;
    __be16 *txb = (__be16 *)mse.txd;
    __be16 *cmd = (__be16 *)mse.rxd;
    u8 *trx = mse.rxd;
    int ret;
    txb[0] = 0;
    txb[1] = 0;
    xfer.tx_buf = txb;
    xfer.rx_buf = trx;
    xfer.len = DET_CMD_LEN;
    ret = spi_sync(mses.spidev, msg);
    if (ret < 0) {
    netdev_err(mse.ndev, "%s: spi_sync() failed: %d\n",
    __func__, ret);
    mse.stats.xfer_err++;
    } else if (*cmd != cpu_to_be16(DET_CMD)) {
    net_dbg_ratelimited("%s: Unexpected response (0x%04x)\n",
    __func__, *cmd);
    ret = -EIO;
    } else {
    memcpy(rxb, trx + 2, 2);
    mses.valid_cmd_received = true;
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn mse102x_push_header(skb: *mut sk_buff) {
    static inline void mse102x_push_header(struct sk_buff *skb)
    {
    __be16 *header = skb_push(skb, DET_SOF_LEN);
// header = cpu_to_be16(DET_SOF);
    }
#[no_mangle]
pub unsafe extern "C" fn mse102x_put_footer(skb: *mut sk_buff) {
    static inline void mse102x_put_footer(struct sk_buff *skb)
    {
    __be16 *footer = skb_put(skb, DET_DFT_LEN);
// footer = cpu_to_be16(DET_DFT);
    }
    static int mse102x_tx_frame_spi(struct mse102x_net *mse, struct sk_buff *txp,
    unsigned int pad)
    {
    struct mse102x_net_spi *mses = to_mse102x_spi(mse);
    struct spi_transfer *xfer = &mses.spi_xfer;
    struct spi_message *msg = &mses.spi_msg;
    struct sk_buff *tskb = core::ptr::null_mut();
    int ret;
    netif_dbg(mse, tx_queued, mse.ndev, "%s: skb %p, %d@%p\n",
    __func__, txp, txp.len, txp.data);
    if ((skb_headroom(txp) < DET_SOF_LEN) ||
    (skb_tailroom(txp) < DET_DFT_LEN + pad)) {
    tskb = skb_copy_expand(txp, DET_SOF_LEN, DET_DFT_LEN + pad,
    GFP_KERNEL);
    if (!tskb)
    return -ENOMEM;
    txp = tskb;
    }
    mse102x_push_header(txp);
    if (pad)
    skb_put_zero(txp, pad);
    mse102x_put_footer(txp);
    xfer.tx_buf = txp.data;
    xfer.rx_buf = core::ptr::null_mut();
    xfer.len = txp.len;
    ret = spi_sync(mses.spidev, msg);
    if (ret < 0) {
    netdev_err(mse.ndev, "%s: spi_sync() failed: %d\n",
    __func__, ret);
    mse.stats.xfer_err++;
    }
    dev_kfree_skb(tskb);
    return ret;
    }
    static int mse102x_rx_frame_spi(struct mse102x_net *mse, u8 *buff,
    unsigned int frame_len, bool drop)
    {
    struct mse102x_net_spi *mses = to_mse102x_spi(mse);
    struct spi_transfer *xfer = &mses.spi_xfer;
    struct spi_message *msg = &mses.spi_msg;
    __be16 *sof = (__be16 *)buff;
    __be16 *dft = (__be16 *)(buff + DET_SOF_LEN + frame_len);
    int ret;
    xfer.rx_buf = buff;
    xfer.tx_buf = core::ptr::null_mut();
    xfer.len = DET_SOF_LEN + frame_len + DET_DFT_LEN;
    ret = spi_sync(mses.spidev, msg);
    if (ret < 0) {
    netdev_err(mse.ndev, "%s: spi_sync() failed: %d\n",
    __func__, ret);
    mse.stats.xfer_err++;
    } else if (drop) {
    netdev_dbg(mse.ndev, "%s: Drop frame\n", __func__);
    ret = -EINVAL;
    } else if (*sof != cpu_to_be16(DET_SOF)) {
    netdev_dbg(mse.ndev, "%s: SPI start of frame is invalid (0x%04x)\n",
    __func__, *sof);
    mse.stats.invalid_sof++;
    ret = -EIO;
    } else if (*dft != cpu_to_be16(DET_DFT)) {
    netdev_dbg(mse.ndev, "%s: SPI frame tail is invalid (0x%04x)\n",
    __func__, *dft);
    mse.stats.invalid_dft++;
    ret = -EIO;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mse102x_dump_packet(msg: *const c_char, len: c_int, data: *const c_char) {
    static void mse102x_dump_packet(const char *msg, int len, const char *data)
    {
    printk(KERN_DEBUG ": %s - packet len:%d\n", msg, len);
    print_hex_dump(KERN_DEBUG, "pk data: ", DUMP_PREFIX_OFFSET, 16, 1,
    data, len, true);
    }
#[no_mangle]
unsafe extern "C" fn mse102x_rx_pkt_spi(mse: *mut mse102x_net) -> irqreturn_t {
    static irqreturn_t mse102x_rx_pkt_spi(struct mse102x_net *mse)
    {
    struct sk_buff *skb;
    unsigned int rxalign;
    unsigned int rxlen;
    let mut drop: bool = false;
    let mut rx: __be16 = 0;
    u16 cmd_resp;
    u8 *rxpkt;
    mse102x_tx_cmd_spi(mse, CMD_CTR);
    if (mse102x_rx_cmd_spi(mse, (u8 *)&rx)) {
    usleep_range(50, 100);
    return IRQ_NONE;
    }
    cmd_resp = be16_to_cpu(rx);
    if ((cmd_resp & CMD_MASK) != CMD_RTS) {
    net_dbg_ratelimited("%s: Unexpected response (0x%04x)\n",
    __func__, cmd_resp);
    mse.stats.invalid_rts++;
    drop = true;
    goto drop;
    }
    rxlen = cmd_resp & LEN_MASK;
    if (rxlen < ETH_ZLEN || rxlen > VLAN_ETH_FRAME_LEN) {
    net_dbg_ratelimited("%s: Invalid frame length: %d\n", __func__,
    rxlen);
    mse.stats.invalid_len++;
    drop = true;
    }
// In case of a invalid CMD_RTS, the frame must be consumed anyway.
// So assume the maximum possible frame length.
//
    drop:
    if (drop)
    rxlen = VLAN_ETH_FRAME_LEN;
    rxalign = ALIGN(rxlen + DET_SOF_LEN + DET_DFT_LEN, 4);
    skb = netdev_alloc_skb_ip_align(mse.ndev, rxalign);
    if (!skb)
    return IRQ_NONE;
// 2 bytes Start of frame (before ethernet header)
// 2 bytes Data frame tail (after ethernet frame)
// They are copied, but ignored.
//
    rxpkt = skb_put(skb, rxlen) - DET_SOF_LEN;
    if (mse102x_rx_frame_spi(mse, rxpkt, rxlen, drop)) {
    mse.ndev.stats.rx_errors++;
    dev_kfree_skb(skb);
    return IRQ_HANDLED;
    }
    if (netif_msg_pktdata(mse))
    mse102x_dump_packet(__func__, skb.len, skb.data);
    skb.protocol = eth_type_trans(skb, mse.ndev);
    netif_rx(skb);
    mse.ndev.stats.rx_packets++;
    mse.ndev.stats.rx_bytes += rxlen;
    return IRQ_HANDLED;
    }
    static int mse102x_tx_pkt_spi(struct mse102x_net *mse, struct sk_buff *txb,
    unsigned long work_timeout)
    {
    let mut pad: c_uint = 0;
    let mut rx: __be16 = 0;
    u16 cmd_resp;
    int ret;
    let mut first: bool = true;
    if (txb.len < ETH_ZLEN)
    pad = ETH_ZLEN - txb.len;
    while (1) {
    mse102x_tx_cmd_spi(mse, CMD_RTS | (txb.len + pad));
    ret = mse102x_rx_cmd_spi(mse, (u8 *)&rx);
    cmd_resp = be16_to_cpu(rx);
    if (!ret) {
// ready to send frame ?
    if (cmd_resp == CMD_CTR)
    break;
    net_dbg_ratelimited("%s: Unexpected response (0x%04x)\n",
    __func__, cmd_resp);
    mse.stats.invalid_ctr++;
    }
// It's not predictable how long / many retries it takes to
// send at least one packet, so TX timeouts are possible.
// That's the reason why the netdev watchdog is not used here.
//
    if (time_after(jiffies, work_timeout))
    return -ETIMEDOUT;
    if (first) {
// throttle at first issue
    netif_stop_queue(mse.ndev);
// fast retry
    usleep_range(50, 100);
    first = false;
    } else {
    msleep(20);
    }
    }
    ret = mse102x_tx_frame_spi(mse, txb, pad);
    if (ret)
    net_dbg_ratelimited("%s: Failed to send (%d), drop frame\n",
    __func__, ret);
    return ret;
    }
pub const TX_QUEUE_MAX: c_int = 10;
#[no_mangle]
unsafe extern "C" fn mse102x_tx_work(work: *mut work_struct) {
    static void mse102x_tx_work(struct work_struct *work)
    {
// Make sure timeout is sufficient to transfer TX_QUEUE_MAX frames
    let mut work_timeout: c_ulong = jiffies + msecs_to_jiffies(1000);
    struct mse102x_net_spi *mses;
    struct mse102x_net *mse;
    struct sk_buff *txb;
    let mut ret: c_int = 0;
    mses = container_of(work, struct mse102x_net_spi, tx_work);
    mse = &mses.mse102x;
    while ((txb = skb_dequeue(&mse.txq))) {
    let mut len: c_uint = max_t(unsigned int, txb.len, ETH_ZLEN);
    mutex_lock(&mses.lock);
    ret = mse102x_tx_pkt_spi(mse, txb, work_timeout);
    mutex_unlock(&mses.lock);
    if (ret) {
    mse.ndev.stats.tx_dropped++;
    } else {
    mse.ndev.stats.tx_bytes += len;
    mse.ndev.stats.tx_packets++;
    }
    dev_kfree_skb(txb);
    }
    if (ret == -ETIMEDOUT) {
    if (netif_msg_timer(mse))
    netdev_err_once(mse.ndev, "tx work timeout\n");
    mse.stats.tx_timeout++;
    }
    netif_wake_queue(mse.ndev);
    }
    static netdev_tx_t mse102x_start_xmit_spi(struct sk_buff *skb,
    struct net_device *ndev)
    {
    struct mse102x_net *mse = netdev_priv(ndev);
    struct mse102x_net_spi *mses = to_mse102x_spi(mse);
    netif_dbg(mse, tx_queued, ndev,
    "%s: skb %p, %d@%p\n", __func__, skb, skb.len, skb.data);
    skb_queue_tail(&mse.txq, skb);
    if (skb_queue_len(&mse.txq) >= TX_QUEUE_MAX)
    netif_stop_queue(ndev);
    schedule_work(&mses.tx_work);
    return NETDEV_TX_OK;
    }
#[no_mangle]
unsafe extern "C" fn mse102x_init_mac(mse: *mut mse102x_net, np: *mut device_node) {
    static void mse102x_init_mac(struct mse102x_net *mse, struct device_node *np)
    {
    struct net_device *ndev = mse.ndev;
    let mut ret: c_int = of_get_ethdev_address(np, ndev);
    if (ret) {
    eth_hw_addr_random(ndev);
    dev_warn(ndev.dev.parent, "Using random MAC address: %pM\n",
    ndev.dev_addr);
    }
    }
// Assumption: this is called for every incoming packet
#[no_mangle]
unsafe extern "C" fn mse102x_irq(irq: c_int, _mse: *mut c_void) -> irqreturn_t {
    static irqreturn_t mse102x_irq(int irq, void *_mse)
    {
    struct mse102x_net *mse = _mse;
    struct mse102x_net_spi *mses = to_mse102x_spi(mse);
    irqreturn_t ret;
    mutex_lock(&mses.lock);
    ret = mse102x_rx_pkt_spi(mse);
    mutex_unlock(&mses.lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mse102x_net_open(ndev: *mut net_device) -> c_int {
    static int mse102x_net_open(struct net_device *ndev)
    {
    struct irq_data *irq_data = irq_get_irq_data(ndev.irq);
    struct mse102x_net *mse = netdev_priv(ndev);
    struct mse102x_net_spi *mses = to_mse102x_spi(mse);
    int ret;
    if (!irq_data) {
    netdev_err(ndev, "Invalid IRQ: %d\n", ndev.irq);
    return -EINVAL;
    }
    switch (irqd_get_trigger_type(irq_data)) {
    case IRQ_TYPE_LEVEL_HIGH:
    case IRQ_TYPE_LEVEL_LOW:
    break;
    default:
    netdev_warn_once(ndev, "Only IRQ type level recommended, please update your device tree firmware.\n");
    break;
    }
    ret = request_threaded_irq(ndev.irq, core::ptr::null_mut(), mse102x_irq, IRQF_ONESHOT,
    ndev.name, mse);
    if (ret < 0) {
    netdev_err(ndev, "Failed to get irq: %d\n", ret);
    return ret;
    }
    netif_dbg(mse, ifup, ndev, "opening\n");
    netif_start_queue(ndev);
    netif_carrier_on(ndev);
// The SPI interrupt can stuck in case of pending packet(s).
// So poll for possible packet(s) to re-arm the interrupt.
//
    mutex_lock(&mses.lock);
    if (mse102x_rx_pkt_spi(mse) == IRQ_NONE)
    mse102x_rx_pkt_spi(mse);
    mutex_unlock(&mses.lock);
    netif_dbg(mse, ifup, ndev, "network device up\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mse102x_net_stop(ndev: *mut net_device) -> c_int {
    static int mse102x_net_stop(struct net_device *ndev)
    {
    struct mse102x_net *mse = netdev_priv(ndev);
    struct mse102x_net_spi *mses = to_mse102x_spi(mse);
    netif_info(mse, ifdown, ndev, "shutting down\n");
    netif_carrier_off(mse.ndev);
// stop any outstanding work
    flush_work(&mses.tx_work);
    netif_stop_queue(ndev);
    skb_queue_purge(&mse.txq);
    free_irq(ndev.irq, mse);
    return 0;
    }
    static const struct net_device_ops mse102x_netdev_ops = {
    .ndo_open		= mse102x_net_open,
    .ndo_stop		= mse102x_net_stop,
    .ndo_start_xmit		= mse102x_start_xmit_spi,
    .ndo_set_mac_address	= eth_mac_addr,
    .ndo_validate_addr	= eth_validate_addr,
    };
// ethtool support
    static void mse102x_get_drvinfo(struct net_device *ndev,
    struct ethtool_drvinfo *di)
    {
    strscpy(di.driver, DRV_NAME, sizeof(di.driver));
    strscpy(di.bus_info, dev_name(ndev.dev.parent), sizeof(di.bus_info));
    }
#[no_mangle]
unsafe extern "C" fn mse102x_get_msglevel(ndev: *mut net_device) -> u32 {
    static u32 mse102x_get_msglevel(struct net_device *ndev)
    {
    struct mse102x_net *mse = netdev_priv(ndev);
    return mse.msg_enable;
    }
#[no_mangle]
unsafe extern "C" fn mse102x_set_msglevel(ndev: *mut net_device, to: u32) {
    static void mse102x_set_msglevel(struct net_device *ndev, u32 to)
    {
    struct mse102x_net *mse = netdev_priv(ndev);
    mse.msg_enable = to;
    }
    static void mse102x_get_ethtool_stats(struct net_device *ndev,
    struct ethtool_stats *estats, u64 *data)
    {
    struct mse102x_net *mse = netdev_priv(ndev);
    struct mse102x_stats *st = &mse.stats;
    memcpy(data, st, ARRAY_SIZE(mse102x_gstrings_stats) * sizeof(u64));
    }
#[no_mangle]
unsafe extern "C" fn mse102x_get_strings(ndev: *mut net_device, stringset: u32, buf: *mut u8) {
    static void mse102x_get_strings(struct net_device *ndev, u32 stringset, u8 *buf)
    {
    switch (stringset) {
    case ETH_SS_STATS:
    memcpy(buf, &mse102x_gstrings_stats,
    sizeof(mse102x_gstrings_stats));
    break;
    default:
    WARN_ON(1);
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn mse102x_get_sset_count(ndev: *mut net_device, sset: c_int) -> c_int {
    static int mse102x_get_sset_count(struct net_device *ndev, int sset)
    {
    switch (sset) {
    case ETH_SS_STATS:
    return ARRAY_SIZE(mse102x_gstrings_stats);
    default:
    return -EINVAL;
    }
    }
    static const struct ethtool_ops mse102x_ethtool_ops = {
    .get_drvinfo		= mse102x_get_drvinfo,
    .get_link		= ethtool_op_get_link,
    .get_msglevel		= mse102x_get_msglevel,
    .set_msglevel		= mse102x_set_msglevel,
    .get_ethtool_stats	= mse102x_get_ethtool_stats,
    .get_strings		= mse102x_get_strings,
    .get_sset_count		= mse102x_get_sset_count,
    };
// driver bus management functions
#[no_mangle]
unsafe extern "C" fn mse102x_suspend(dev: *mut device) -> c_int {
    static int mse102x_suspend(struct device *dev)
    {
    struct mse102x_net *mse = dev_get_drvdata(dev);
    struct net_device *ndev = mse.ndev;
    if (netif_running(ndev)) {
    netif_device_detach(ndev);
    mse102x_net_stop(ndev);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mse102x_resume(dev: *mut device) -> c_int {
    static int mse102x_resume(struct device *dev)
    {
    struct mse102x_net *mse = dev_get_drvdata(dev);
    struct net_device *ndev = mse.ndev;
    if (netif_running(ndev)) {
    mse102x_net_open(ndev);
    netif_device_attach(ndev);
    }
    return 0;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(mse102x_pm_ops, mse102x_suspend, mse102x_resume);
#[no_mangle]
unsafe extern "C" fn mse102x_probe_spi(spi: *mut spi_device) -> c_int {
    static int mse102x_probe_spi(struct spi_device *spi)
    {
    struct device *dev = &spi.dev;
    struct mse102x_net_spi *mses;
    struct net_device *ndev;
    struct mse102x_net *mse;
    int ret;
    spi.bits_per_word = 8;
    spi.mode |= SPI_MODE_3;
// enforce minimum speed to ensure device functionality
    spi.controller.min_speed_hz = MIN_FREQ_HZ;
    if (!spi.max_speed_hz)
    spi.max_speed_hz = MAX_FREQ_HZ;
    if (spi.max_speed_hz < MIN_FREQ_HZ ||
    spi.max_speed_hz > MAX_FREQ_HZ) {
    dev_err(&spi.dev, "SPI max frequency out of range (min: %u, max: %u)\n",
    MIN_FREQ_HZ, MAX_FREQ_HZ);
    return -EINVAL;
    }
    ret = spi_setup(spi);
    if (ret < 0) {
    dev_err(&spi.dev, "Unable to setup SPI device: %d\n", ret);
    return ret;
    }
    ndev = devm_alloc_etherdev(dev, sizeof(struct mse102x_net_spi));
    if (!ndev)
    return -ENOMEM;
    ndev.needed_tailroom += ALIGN(DET_DFT_LEN, 4);
    ndev.needed_headroom += ALIGN(DET_SOF_LEN, 4);
    ndev.priv_flags &= ~IFF_TX_SKB_SHARING;
    ndev.tx_queue_len = 100;
    mse = netdev_priv(ndev);
    mses = to_mse102x_spi(mse);
    mses.spidev = spi;
    mutex_init(&mses.lock);
    INIT_WORK(&mses.tx_work, mse102x_tx_work);
// initialise pre-made spi transfer messages
    spi_message_init(&mses.spi_msg);
    spi_message_add_tail(&mses.spi_xfer, &mses.spi_msg);
    ndev.irq = spi.irq;
    mse.ndev = ndev;
// set the default message enable
    mse.msg_enable = netif_msg_init(-1, MSG_DEFAULT);
    skb_queue_head_init(&mse.txq);
    SET_NETDEV_DEV(ndev, dev);
    dev_set_drvdata(dev, mse);
    netif_carrier_off(mse.ndev);
    ndev.netdev_ops = &mse102x_netdev_ops;
    ndev.ethtool_ops = &mse102x_ethtool_ops;
    mse102x_init_mac(mse, dev.of_node);
    ret = register_netdev(ndev);
    if (ret) {
    dev_err(dev, "failed to register network device: %d\n", ret);
    return ret;
    }
    mse102x_init_device_debugfs(mses);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mse102x_remove_spi(spi: *mut spi_device) {
    static void mse102x_remove_spi(struct spi_device *spi)
    {
    struct mse102x_net *mse = dev_get_drvdata(&spi.dev);
    struct mse102x_net_spi *mses = to_mse102x_spi(mse);
    mse102x_remove_device_debugfs(mses);
    unregister_netdev(mse.ndev);
    }
    static const struct of_device_id mse102x_match_table[] = {
    { .compatible = "vertexcom,mse1021" },
    { .compatible = "vertexcom,mse1022" },
    { }
    };
    MODULE_DEVICE_TABLE(of, mse102x_match_table);
    static const struct spi_device_id mse102x_ids[] = {
    { "mse1021" },
    { "mse1022" },
    { }
    };
    MODULE_DEVICE_TABLE(spi, mse102x_ids);
    static struct spi_driver mse102x_driver = {
    .driver = {
    .name = DRV_NAME,
    .of_match_table = mse102x_match_table,
    .pm = pm_sleep_ptr(&mse102x_pm_ops),
    },
    .probe = mse102x_probe_spi,
    .remove = mse102x_remove_spi,
    .id_table = mse102x_ids,
    };
    module_spi_driver(mse102x_driver);
    MODULE_DESCRIPTION("MSE102x Network driver");
    MODULE_AUTHOR("Stefan Wahren <stefan.wahren@chargebyte.com>");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("spi:" DRV_NAME);
