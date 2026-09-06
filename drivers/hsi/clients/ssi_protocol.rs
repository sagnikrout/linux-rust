//! Automatically rewritten from C to Rust
//! Source: drivers/hsi/clients/ssi_protocol.c
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
// ssi_protocol.c
//
// Implementation of the SSI McSAAB improved protocol.
//
// Copyright (C) 2010 Nokia Corporation. All rights reserved.
// Copyright (C) 2013 Sebastian Reichel <sre@kernel.org>
//
// Contact: Carlos Chinea <carlos.chinea@nokia.com>
//

pub const SSIP_TXQUEUE_LEN: c_int = 100;
pub const SSIP_MAX_MTU: c_int = 65535;
pub const SSIP_DEFAULT_MTU: c_int = 4000;
pub const PN_MEDIA_SOS: c_int = 21;

pub const SSIP_CMT_LOADER_SYNC: c_uint = 0x11223344;
//
// SSI protocol command definitions
//

// Commands
pub const SSIP_SW_BREAK: c_int = 0;
pub const SSIP_BOOTINFO_REQ: c_int = 1;
pub const SSIP_BOOTINFO_RESP: c_int = 2;
pub const SSIP_WAKETEST_RESULT: c_int = 3;
pub const SSIP_START_TRANS: c_int = 4;
pub const SSIP_READY: c_int = 5;
// Payloads

pub const SSIP_LOCAL_VERID: c_int = 1;
pub const SSIP_WAKETEST_OK: c_int = 0;
pub const SSIP_WAKETEST_FAILED: c_int = 1;

// Generic Command

// Commands for the control channel

    SSIP_CMD(SSIP_BOOTINFO_REQ, SSIP_DATA_VERSION(ver))

    SSIP_CMD(SSIP_BOOTINFO_RESP, SSIP_DATA_VERSION(ver))

    SSIP_CMD(SSIP_START_TRANS, (((pdulen) << 8) | SSIP_MSG_ID(id)))

pub const SSIP_WAKETEST_FLAG: c_int = 0;
// Main state machine states
    enum {
    INIT,
    HANDSHAKE,
    ACTIVE,
    };
// Send state machine states
    enum {
    SEND_IDLE,
    WAIT4READY,
    SEND_READY,
    SENDING,
    SENDING_SWBREAK,
    };
// Receive state machine states
    enum {
    RECV_IDLE,
    RECV_READY,
    RECEIVING,
    };
//
// struct ssi_protocol - SSI protocol (McSAAB) data
// @main_state: Main state machine
// @send_state: TX state machine
// @recv_state: RX state machine
// @flags: Flags, currently only used to follow wake line test
// @rxid: RX data id
// @txid: TX data id
// @txqueue_len: TX queue length
// @tx_wd: TX watchdog
// @rx_wd: RX watchdog
// @keep_alive: Workaround for SSI HW bug
// @lock: To serialize access to this struct
// @netdev: Phonet network device
// @txqueue: TX data queue
// @cmdqueue: Queue of free commands
// @work: &struct work_struct for scheduled work
// @cl: HSI client own reference
// @link: Link for ssip_list
// @tx_usecnt: Refcount to keep track the slaves that use the wake line
// @channel_id_cmd: HSI channel id for command stream
// @channel_id_data: HSI channel id for data stream
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssi_protocol {
    pub main_state: c_uint,
    pub send_state: c_uint,
    pub recv_state: c_uint,
    pub flags: c_ulong,
    pub rxid: u8,
    pub txid: u8,
    pub txqueue_len: c_uint,
    pub tx_wd: timer_list,
    pub rx_wd: timer_list,
    pub /: *mut *mut timer_list keep_alive; / wake-up workaround,
    pub lock: spinlock_t,
    pub netdev: *mut net_device,
    pub txqueue: list_head,
    pub cmdqueue: list_head,
    pub work: work_struct,
    pub cl: *mut hsi_client,
    pub link: list_head,
    pub tx_usecnt: core::sync::atomic::AtomicI32,
    pub channel_id_cmd: c_int,
    pub channel_id_data: c_int,
}

// List of ssi protocol instances
    static LIST_HEAD(ssip_list);
    static void ssip_rxcmd_complete(struct hsi_msg *msg);
#[no_mangle]
pub unsafe extern "C" fn ssip_set_cmd(msg: *mut hsi_msg, cmd: u32) {
    static inline void ssip_set_cmd(struct hsi_msg *msg, u32 cmd)
    {
    u32 *data;
    data = sg_virt(msg.sgt.sgl);
// data = cmd;
    }
#[no_mangle]
pub unsafe extern "C" fn ssip_get_cmd(msg: *mut hsi_msg) -> u32 {
    static inline u32 ssip_get_cmd(struct hsi_msg *msg)
    {
    u32 *data;
    data = sg_virt(msg.sgt.sgl);
    return *data;
    }
#[no_mangle]
unsafe extern "C" fn ssip_skb_to_msg(skb: *mut sk_buff, msg: *mut hsi_msg) {
    static void ssip_skb_to_msg(struct sk_buff *skb, struct hsi_msg *msg)
    {
    skb_frag_t *frag;
    struct scatterlist *sg;
    int i;
    BUG_ON(msg.sgt.nents != (unsigned int)(skb_shinfo(skb).nr_frags + 1));
    sg = msg.sgt.sgl;
    sg_set_buf(sg, skb.data, skb_headlen(skb));
    for (i = 0; i < skb_shinfo(skb).nr_frags; i++) {
    sg = sg_next(sg);
    BUG_ON(!sg);
    frag = &skb_shinfo(skb).frags[i];
    sg_set_page(sg, skb_frag_page(frag), skb_frag_size(frag),
    skb_frag_off(frag));
    }
    }
#[no_mangle]
unsafe extern "C" fn ssip_free_data(msg: *mut hsi_msg) {
    static void ssip_free_data(struct hsi_msg *msg)
    {
    struct sk_buff *skb;
    skb = msg.context;
    pr_debug("free data: msg %p context %p skb %p\n", msg, msg.context,
    skb);
    msg.destructor = core::ptr::null_mut();
    dev_kfree_skb(skb);
    hsi_free_msg(msg);
    }
    static struct hsi_msg *ssip_alloc_data(struct ssi_protocol *ssi,
    struct sk_buff *skb, gfp_t flags)
    {
    struct hsi_msg *msg;
    msg = hsi_alloc_msg(skb_shinfo(skb).nr_frags + 1, flags);
    if (!msg)
    return core::ptr::null_mut();
    ssip_skb_to_msg(skb, msg);
    msg.destructor = ssip_free_data;
    msg.channel = ssi.channel_id_data;
    msg.context = skb;
    return msg;
    }
#[no_mangle]
pub unsafe extern "C" fn ssip_release_cmd(msg: *mut hsi_msg) {
    static inline void ssip_release_cmd(struct hsi_msg *msg)
    {
    struct ssi_protocol *ssi = hsi_client_drvdata(msg.cl);
    dev_dbg(&msg.cl.device, "Release cmd 0x%08x\n", ssip_get_cmd(msg));
    spin_lock_bh(&ssi.lock);
    list_add_tail(&msg.link, &ssi.cmdqueue);
    spin_unlock_bh(&ssi.lock);
    }
    static struct hsi_msg *ssip_claim_cmd(struct ssi_protocol *ssi)
    {
    struct hsi_msg *msg;
    BUG_ON(list_empty(&ssi.cmdqueue));
    spin_lock_bh(&ssi.lock);
    msg = list_first_entry(&ssi.cmdqueue, struct hsi_msg, link);
    list_del(&msg.link);
    spin_unlock_bh(&ssi.lock);
    msg.destructor = ssip_release_cmd;
    return msg;
    }
#[no_mangle]
unsafe extern "C" fn ssip_free_cmds(ssi: *mut ssi_protocol) {
    static void ssip_free_cmds(struct ssi_protocol *ssi)
    {
    struct hsi_msg *msg, *tmp;
    list_for_each_entry_safe(msg, tmp, &ssi.cmdqueue, link) {
    list_del(&msg.link);
    msg.destructor = core::ptr::null_mut();
    kfree(sg_virt(msg.sgt.sgl));
    hsi_free_msg(msg);
    }
    }
#[no_mangle]
unsafe extern "C" fn ssip_alloc_cmds(ssi: *mut ssi_protocol) -> c_int {
    static int ssip_alloc_cmds(struct ssi_protocol *ssi)
    {
    struct hsi_msg *msg;
    u32 *buf;
    unsigned int i;
    for (i = 0; i < SSIP_MAX_CMDS; i++) {
    msg = hsi_alloc_msg(1, GFP_KERNEL);
    if (!msg)
    goto out;
    buf = kmalloc_obj(*buf);
    if (!buf) {
    hsi_free_msg(msg);
    goto out;
    }
    sg_init_one(msg.sgt.sgl, buf, sizeof(*buf));
    msg.channel = ssi.channel_id_cmd;
    list_add_tail(&msg.link, &ssi.cmdqueue);
    }
    return 0;
    out:
    ssip_free_cmds(ssi);
    return -ENOMEM;
    }
#[no_mangle]
unsafe extern "C" fn ssip_set_rxstate(ssi: *mut ssi_protocol, state: c_uint) {
    static void ssip_set_rxstate(struct ssi_protocol *ssi, unsigned int state)
    {
    ssi.recv_state = state;
    switch (state) {
    case RECV_IDLE:
    timer_delete(&ssi.rx_wd);
    if (ssi.send_state == SEND_IDLE)
    timer_delete(&ssi.keep_alive);
    break;
    case RECV_READY:
// CMT speech workaround
    if (atomic_read(&ssi.tx_usecnt))
    break;
    fallthrough;
    case RECEIVING:
    mod_timer(&ssi.keep_alive, jiffies +
    msecs_to_jiffies(SSIP_KATOUT));
    mod_timer(&ssi.rx_wd, jiffies + msecs_to_jiffies(SSIP_WDTOUT));
    break;
    default:
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn ssip_set_txstate(ssi: *mut ssi_protocol, state: c_uint) {
    static void ssip_set_txstate(struct ssi_protocol *ssi, unsigned int state)
    {
    ssi.send_state = state;
    switch (state) {
    case SEND_IDLE:
    case SEND_READY:
    timer_delete(&ssi.tx_wd);
    if (ssi.recv_state == RECV_IDLE)
    timer_delete(&ssi.keep_alive);
    break;
    case WAIT4READY:
    case SENDING:
    case SENDING_SWBREAK:
    mod_timer(&ssi.keep_alive,
    jiffies + msecs_to_jiffies(SSIP_KATOUT));
    mod_timer(&ssi.tx_wd, jiffies + msecs_to_jiffies(SSIP_WDTOUT));
    break;
    default:
    break;
    }
    }
    struct hsi_client *ssip_slave_get_master(struct hsi_client *slave)
    {
    struct hsi_client *master = ERR_PTR(-ENODEV);
    struct ssi_protocol *ssi;
    list_for_each_entry(ssi, &ssip_list, link)
    if (slave.device.parent == ssi.cl.device.parent) {
    master = ssi.cl;
    break;
    }
    return master;
    }
    EXPORT_SYMBOL_GPL(ssip_slave_get_master);
#[no_mangle]
pub unsafe extern "C" fn ssip_slave_start_tx(master: *mut hsi_client) -> c_int {
    int ssip_slave_start_tx(struct hsi_client *master)
    {
    struct ssi_protocol *ssi = hsi_client_drvdata(master);
    dev_dbg(&master.device, "start TX %d\n", atomic_read(&ssi.tx_usecnt));
    spin_lock_bh(&ssi.lock);
    if (ssi.send_state == SEND_IDLE) {
    ssip_set_txstate(ssi, WAIT4READY);
    hsi_start_tx(master);
    }
    spin_unlock_bh(&ssi.lock);
    atomic_inc(&ssi.tx_usecnt);
    return 0;
    }
    EXPORT_SYMBOL_GPL(ssip_slave_start_tx);
#[no_mangle]
pub unsafe extern "C" fn ssip_slave_stop_tx(master: *mut hsi_client) -> c_int {
    int ssip_slave_stop_tx(struct hsi_client *master)
    {
    struct ssi_protocol *ssi = hsi_client_drvdata(master);
    WARN_ON_ONCE(atomic_read(&ssi.tx_usecnt) == 0);
    if (atomic_dec_and_test(&ssi.tx_usecnt)) {
    spin_lock_bh(&ssi.lock);
    if ((ssi.send_state == SEND_READY) ||
    (ssi.send_state == WAIT4READY)) {
    ssip_set_txstate(ssi, SEND_IDLE);
    hsi_stop_tx(master);
    }
    spin_unlock_bh(&ssi.lock);
    }
    dev_dbg(&master.device, "stop TX %d\n", atomic_read(&ssi.tx_usecnt));
    return 0;
    }
    EXPORT_SYMBOL_GPL(ssip_slave_stop_tx);
#[no_mangle]
pub unsafe extern "C" fn ssip_slave_running(master: *mut hsi_client) -> c_int {
    int ssip_slave_running(struct hsi_client *master)
    {
    struct ssi_protocol *ssi = hsi_client_drvdata(master);
    return netif_running(ssi.netdev);
    }
    EXPORT_SYMBOL_GPL(ssip_slave_running);
#[no_mangle]
unsafe extern "C" fn ssip_reset(cl: *mut hsi_client) {
    static void ssip_reset(struct hsi_client *cl)
    {
    struct ssi_protocol *ssi = hsi_client_drvdata(cl);
    struct list_head *head, *tmp;
    struct hsi_msg *msg;
    if (netif_running(ssi.netdev))
    netif_carrier_off(ssi.netdev);
    hsi_flush(cl);
    spin_lock_bh(&ssi.lock);
    if (ssi.send_state != SEND_IDLE)
    hsi_stop_tx(cl);
    spin_unlock_bh(&ssi.lock);
    if (test_and_clear_bit(SSIP_WAKETEST_FLAG, &ssi.flags))
    ssi_waketest(cl, 0); /* FIXME: To be removed */
    spin_lock_bh(&ssi.lock);
    timer_delete(&ssi.rx_wd);
    timer_delete(&ssi.tx_wd);
    timer_delete(&ssi.keep_alive);
    cancel_work_sync(&ssi.work);
    ssi.main_state = 0;
    ssi.send_state = 0;
    ssi.recv_state = 0;
    ssi.flags = 0;
    ssi.rxid = 0;
    ssi.txid = 0;
    list_for_each_safe(head, tmp, &ssi.txqueue) {
    msg = list_entry(head, struct hsi_msg, link);
    dev_dbg(&cl.device, "Pending TX data\n");
    list_del(head);
    ssip_free_data(msg);
    }
    ssi.txqueue_len = 0;
    spin_unlock_bh(&ssi.lock);
    }
#[no_mangle]
unsafe extern "C" fn ssip_dump_state(cl: *mut hsi_client) {
    static void ssip_dump_state(struct hsi_client *cl)
    {
    struct ssi_protocol *ssi = hsi_client_drvdata(cl);
    struct hsi_msg *msg;
    spin_lock_bh(&ssi.lock);
    dev_err(&cl.device, "Main state: %d\n", ssi.main_state);
    dev_err(&cl.device, "Recv state: %d\n", ssi.recv_state);
    dev_err(&cl.device, "Send state: %d\n", ssi.send_state);
    dev_err(&cl.device, "CMT %s\n", (ssi.main_state == ACTIVE) ?
    "Online" : "Offline");
    dev_err(&cl.device, "Wake test %d\n",
    test_bit(SSIP_WAKETEST_FLAG, &ssi.flags));
    dev_err(&cl.device, "Data RX id: %d\n", ssi.rxid);
    dev_err(&cl.device, "Data TX id: %d\n", ssi.txid);
    list_for_each_entry(msg, &ssi.txqueue, link)
    dev_err(&cl.device, "pending TX data (%p)\n", msg);
    spin_unlock_bh(&ssi.lock);
    }
#[no_mangle]
unsafe extern "C" fn ssip_error(cl: *mut hsi_client) {
    static void ssip_error(struct hsi_client *cl)
    {
    struct ssi_protocol *ssi = hsi_client_drvdata(cl);
    struct hsi_msg *msg;
    ssip_dump_state(cl);
    ssip_reset(cl);
    msg = ssip_claim_cmd(ssi);
    msg.complete = ssip_rxcmd_complete;
    hsi_async_read(cl, msg);
    }
#[no_mangle]
unsafe extern "C" fn ssip_keep_alive(t: *mut timer_list) {
    static void ssip_keep_alive(struct timer_list *t)
    {
    struct ssi_protocol *ssi = timer_container_of(ssi, t, keep_alive);
    struct hsi_client *cl = ssi.cl;
    dev_dbg(&cl.device, "Keep alive kick in: m(%d) r(%d) s(%d)\n",
    ssi.main_state, ssi.recv_state, ssi.send_state);
    spin_lock(&ssi.lock);
    if (ssi.recv_state == RECV_IDLE)
    switch (ssi.send_state) {
    case SEND_READY:
    if (atomic_read(&ssi.tx_usecnt) == 0)
    break;
    fallthrough;
//
// Workaround for cmt-speech in that case
// we relay on audio timers.
//
    case SEND_IDLE:
    spin_unlock(&ssi.lock);
    return;
    }
    mod_timer(&ssi.keep_alive, jiffies + msecs_to_jiffies(SSIP_KATOUT));
    spin_unlock(&ssi.lock);
    }
#[no_mangle]
unsafe extern "C" fn ssip_rx_wd(t: *mut timer_list) {
    static void ssip_rx_wd(struct timer_list *t)
    {
    struct ssi_protocol *ssi = timer_container_of(ssi, t, rx_wd);
    struct hsi_client *cl = ssi.cl;
    dev_err(&cl.device, "Watchdog triggered\n");
    ssip_error(cl);
    }
#[no_mangle]
unsafe extern "C" fn ssip_tx_wd(t: *mut timer_list) {
    static void ssip_tx_wd(struct timer_list *t)
    {
    struct ssi_protocol *ssi = timer_container_of(ssi, t, tx_wd);
    struct hsi_client *cl = ssi.cl;
    dev_err(&cl.device, "Watchdog triggered\n");
    ssip_error(cl);
    }
#[no_mangle]
unsafe extern "C" fn ssip_send_bootinfo_req_cmd(cl: *mut hsi_client) {
    static void ssip_send_bootinfo_req_cmd(struct hsi_client *cl)
    {
    struct ssi_protocol *ssi = hsi_client_drvdata(cl);
    struct hsi_msg *msg;
    dev_dbg(&cl.device, "Issuing BOOT INFO REQ command\n");
    msg = ssip_claim_cmd(ssi);
    ssip_set_cmd(msg, SSIP_BOOTINFO_REQ_CMD(SSIP_LOCAL_VERID));
    msg.complete = ssip_release_cmd;
    hsi_async_write(cl, msg);
    dev_dbg(&cl.device, "Issuing RX command\n");
    msg = ssip_claim_cmd(ssi);
    msg.complete = ssip_rxcmd_complete;
    hsi_async_read(cl, msg);
    }
#[no_mangle]
unsafe extern "C" fn ssip_start_rx(cl: *mut hsi_client) {
    static void ssip_start_rx(struct hsi_client *cl)
    {
    struct ssi_protocol *ssi = hsi_client_drvdata(cl);
    struct hsi_msg *msg;
    dev_dbg(&cl.device, "RX start M(%d) R(%d)\n", ssi.main_state,
    ssi.recv_state);
    spin_lock_bh(&ssi.lock);
//
// We can have two UP events in a row due to a short low
// high transition. Therefore we need to ignore the sencond UP event.
//
    if ((ssi.main_state != ACTIVE) || (ssi.recv_state == RECV_READY)) {
    spin_unlock_bh(&ssi.lock);
    return;
    }
    ssip_set_rxstate(ssi, RECV_READY);
    spin_unlock_bh(&ssi.lock);
    msg = ssip_claim_cmd(ssi);
    ssip_set_cmd(msg, SSIP_READY_CMD);
    msg.complete = ssip_release_cmd;
    dev_dbg(&cl.device, "Send READY\n");
    hsi_async_write(cl, msg);
    }
#[no_mangle]
unsafe extern "C" fn ssip_stop_rx(cl: *mut hsi_client) {
    static void ssip_stop_rx(struct hsi_client *cl)
    {
    struct ssi_protocol *ssi = hsi_client_drvdata(cl);
    dev_dbg(&cl.device, "RX stop M(%d)\n", ssi.main_state);
    spin_lock_bh(&ssi.lock);
    if (likely(ssi.main_state == ACTIVE))
    ssip_set_rxstate(ssi, RECV_IDLE);
    spin_unlock_bh(&ssi.lock);
    }
#[no_mangle]
unsafe extern "C" fn ssip_free_strans(msg: *mut hsi_msg) {
    static void ssip_free_strans(struct hsi_msg *msg)
    {
    ssip_free_data(msg.context);
    ssip_release_cmd(msg);
    }
#[no_mangle]
unsafe extern "C" fn ssip_strans_complete(msg: *mut hsi_msg) {
    static void ssip_strans_complete(struct hsi_msg *msg)
    {
    struct hsi_client *cl = msg.cl;
    struct ssi_protocol *ssi = hsi_client_drvdata(cl);
    struct hsi_msg *data;
    data = msg.context;
    ssip_release_cmd(msg);
    spin_lock_bh(&ssi.lock);
    ssip_set_txstate(ssi, SENDING);
    spin_unlock_bh(&ssi.lock);
    hsi_async_write(cl, data);
    }
#[no_mangle]
unsafe extern "C" fn ssip_xmit(cl: *mut hsi_client) -> c_int {
    static int ssip_xmit(struct hsi_client *cl)
    {
    struct ssi_protocol *ssi = hsi_client_drvdata(cl);
    struct hsi_msg *msg, *dmsg;
    struct sk_buff *skb;
    spin_lock_bh(&ssi.lock);
    if (list_empty(&ssi.txqueue)) {
    spin_unlock_bh(&ssi.lock);
    return 0;
    }
    dmsg = list_first_entry(&ssi.txqueue, struct hsi_msg, link);
    list_del(&dmsg.link);
    ssi.txqueue_len--;
    spin_unlock_bh(&ssi.lock);
    msg = ssip_claim_cmd(ssi);
    skb = dmsg.context;
    msg.context = dmsg;
    msg.complete = ssip_strans_complete;
    msg.destructor = ssip_free_strans;
    spin_lock_bh(&ssi.lock);
    ssip_set_cmd(msg, SSIP_START_TRANS_CMD(SSIP_BYTES_TO_FRAMES(skb.len),
    ssi.txid));
    ssi.txid++;
    ssip_set_txstate(ssi, SENDING);
    spin_unlock_bh(&ssi.lock);
    dev_dbg(&cl.device, "Send STRANS (%d frames)\n",
    SSIP_BYTES_TO_FRAMES(skb.len));
    return hsi_async_write(cl, msg);
    }
// In soft IRQ context
#[no_mangle]
unsafe extern "C" fn ssip_pn_rx(skb: *mut sk_buff) {
    static void ssip_pn_rx(struct sk_buff *skb)
    {
    struct net_device *dev = skb.dev;
    if (unlikely(!netif_running(dev))) {
    dev_dbg(&dev.dev, "Drop RX packet\n");
    dev.stats.rx_dropped++;
    dev_kfree_skb(skb);
    return;
    }
    if (unlikely(!pskb_may_pull(skb, SSIP_MIN_PN_HDR))) {
    dev_dbg(&dev.dev, "Error drop RX packet\n");
    dev.stats.rx_errors++;
    dev.stats.rx_length_errors++;
    dev_kfree_skb(skb);
    return;
    }
    dev.stats.rx_packets++;
    dev.stats.rx_bytes += skb.len;
// length field is exchanged in network byte order
    ((u16 *)skb.data)[2] = ntohs(((u16 *)skb.data)[2]);
    dev_dbg(&dev.dev, "RX length fixed (%04x . %u)\n",
    ((u16 *)skb.data)[2], ntohs(((u16 *)skb.data)[2]));
    skb.protocol = htons(ETH_P_PHONET);
    skb_reset_mac_header(skb);
    __skb_pull(skb, 1);
    netif_rx(skb);
    }
#[no_mangle]
unsafe extern "C" fn ssip_rx_data_complete(msg: *mut hsi_msg) {
    static void ssip_rx_data_complete(struct hsi_msg *msg)
    {
    struct hsi_client *cl = msg.cl;
    struct ssi_protocol *ssi = hsi_client_drvdata(cl);
    struct sk_buff *skb;
    if (msg.status == HSI_STATUS_ERROR) {
    dev_err(&cl.device, "RX data error\n");
    ssip_free_data(msg);
    ssip_error(cl);
    return;
    }
    timer_delete(&ssi.rx_wd); /* FIXME: Revisit */
    skb = msg.context;
    ssip_pn_rx(skb);
    hsi_free_msg(msg);
    }
#[no_mangle]
unsafe extern "C" fn ssip_rx_bootinforeq(cl: *mut hsi_client, cmd: u32) {
    static void ssip_rx_bootinforeq(struct hsi_client *cl, u32 cmd)
    {
    struct ssi_protocol *ssi = hsi_client_drvdata(cl);
    struct hsi_msg *msg;
// Workaroud: Ignore CMT Loader message leftover
    if (cmd == SSIP_CMT_LOADER_SYNC)
    return;
    switch (ssi.main_state) {
    case ACTIVE:
    dev_err(&cl.device, "Boot info req on active state\n");
    ssip_error(cl);
    fallthrough;
    case INIT:
    case HANDSHAKE:
    spin_lock_bh(&ssi.lock);
    ssi.main_state = HANDSHAKE;
    spin_unlock_bh(&ssi.lock);
    if (!test_and_set_bit(SSIP_WAKETEST_FLAG, &ssi.flags))
    ssi_waketest(cl, 1); /* FIXME: To be removed */
    spin_lock_bh(&ssi.lock);
// Start boot handshake watchdog
    mod_timer(&ssi.tx_wd, jiffies + msecs_to_jiffies(SSIP_WDTOUT));
    spin_unlock_bh(&ssi.lock);
    dev_dbg(&cl.device, "Send BOOTINFO_RESP\n");
    if (SSIP_DATA_VERSION(cmd) != SSIP_LOCAL_VERID)
    dev_warn(&cl.device, "boot info req verid mismatch\n");
    msg = ssip_claim_cmd(ssi);
    ssip_set_cmd(msg, SSIP_BOOTINFO_RESP_CMD(SSIP_LOCAL_VERID));
    msg.complete = ssip_release_cmd;
    hsi_async_write(cl, msg);
    break;
    default:
    dev_dbg(&cl.device, "Wrong state M(%d)\n", ssi.main_state);
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn ssip_rx_bootinforesp(cl: *mut hsi_client, cmd: u32) {
    static void ssip_rx_bootinforesp(struct hsi_client *cl, u32 cmd)
    {
    struct ssi_protocol *ssi = hsi_client_drvdata(cl);
    if (SSIP_DATA_VERSION(cmd) != SSIP_LOCAL_VERID)
    dev_warn(&cl.device, "boot info resp verid mismatch\n");
    spin_lock_bh(&ssi.lock);
    if (ssi.main_state != ACTIVE)
// Use tx_wd as a boot watchdog in non ACTIVE state
    mod_timer(&ssi.tx_wd, jiffies + msecs_to_jiffies(SSIP_WDTOUT));
    else
    dev_dbg(&cl.device, "boot info resp ignored M(%d)\n",
    ssi.main_state);
    spin_unlock_bh(&ssi.lock);
    }
#[no_mangle]
unsafe extern "C" fn ssip_rx_waketest(cl: *mut hsi_client, cmd: u32) {
    static void ssip_rx_waketest(struct hsi_client *cl, u32 cmd)
    {
    struct ssi_protocol *ssi = hsi_client_drvdata(cl);
    let mut wkres: c_uint = SSIP_PAYLOAD(cmd);
    spin_lock_bh(&ssi.lock);
    if (ssi.main_state != HANDSHAKE) {
    dev_dbg(&cl.device, "wake lines test ignored M(%d)\n",
    ssi.main_state);
    spin_unlock_bh(&ssi.lock);
    return;
    }
    spin_unlock_bh(&ssi.lock);
    if (test_and_clear_bit(SSIP_WAKETEST_FLAG, &ssi.flags))
    ssi_waketest(cl, 0); /* FIXME: To be removed */
    spin_lock_bh(&ssi.lock);
    ssi.main_state = ACTIVE;
    timer_delete(&ssi.tx_wd); /* Stop boot handshake timer */
    spin_unlock_bh(&ssi.lock);
    dev_notice(&cl.device, "WAKELINES TEST %s\n",
    wkres & SSIP_WAKETEST_FAILED ? "FAILED" : "OK");
    if (wkres & SSIP_WAKETEST_FAILED) {
    ssip_error(cl);
    return;
    }
    dev_dbg(&cl.device, "CMT is ONLINE\n");
    netif_wake_queue(ssi.netdev);
    netif_carrier_on(ssi.netdev);
    }
#[no_mangle]
unsafe extern "C" fn ssip_rx_ready(cl: *mut hsi_client) {
    static void ssip_rx_ready(struct hsi_client *cl)
    {
    struct ssi_protocol *ssi = hsi_client_drvdata(cl);
    spin_lock_bh(&ssi.lock);
    if (unlikely(ssi.main_state != ACTIVE)) {
    dev_dbg(&cl.device, "READY on wrong state: S(%d) M(%d)\n",
    ssi.send_state, ssi.main_state);
    spin_unlock_bh(&ssi.lock);
    return;
    }
    if (ssi.send_state != WAIT4READY) {
    dev_dbg(&cl.device, "Ignore spurious READY command\n");
    spin_unlock_bh(&ssi.lock);
    return;
    }
    ssip_set_txstate(ssi, SEND_READY);
    spin_unlock_bh(&ssi.lock);
    ssip_xmit(cl);
    }
#[no_mangle]
unsafe extern "C" fn ssip_rx_strans(cl: *mut hsi_client, cmd: u32) {
    static void ssip_rx_strans(struct hsi_client *cl, u32 cmd)
    {
    struct ssi_protocol *ssi = hsi_client_drvdata(cl);
    struct sk_buff *skb;
    struct hsi_msg *msg;
    let mut len: c_int = SSIP_PDU_LENGTH(cmd);
    dev_dbg(&cl.device, "RX strans: %d frames\n", len);
    spin_lock_bh(&ssi.lock);
    if (unlikely(ssi.main_state != ACTIVE)) {
    dev_err(&cl.device, "START TRANS wrong state: S(%d) M(%d)\n",
    ssi.send_state, ssi.main_state);
    spin_unlock_bh(&ssi.lock);
    return;
    }
    ssip_set_rxstate(ssi, RECEIVING);
    if (unlikely(SSIP_MSG_ID(cmd) != ssi.rxid)) {
    dev_err(&cl.device, "START TRANS id %d expected %d\n",
    SSIP_MSG_ID(cmd), ssi.rxid);
    spin_unlock_bh(&ssi.lock);
    goto out1;
    }
    ssi.rxid++;
    spin_unlock_bh(&ssi.lock);
    skb = netdev_alloc_skb(ssi.netdev, len * 4);
    if (unlikely(!skb)) {
    dev_err(&cl.device, "No memory for rx skb\n");
    goto out1;
    }
    skb_put(skb, len * 4);
    msg = ssip_alloc_data(ssi, skb, GFP_ATOMIC);
    if (unlikely(!msg)) {
    dev_err(&cl.device, "No memory for RX data msg\n");
    goto out2;
    }
    msg.complete = ssip_rx_data_complete;
    hsi_async_read(cl, msg);
    return;
    out2:
    dev_kfree_skb(skb);
    out1:
    ssip_error(cl);
    }
#[no_mangle]
unsafe extern "C" fn ssip_rxcmd_complete(msg: *mut hsi_msg) {
    static void ssip_rxcmd_complete(struct hsi_msg *msg)
    {
    struct hsi_client *cl = msg.cl;
    let mut cmd: u32 = ssip_get_cmd(msg);
    let mut cmdid: c_uint = SSIP_COMMAND(cmd);
    if (msg.status == HSI_STATUS_ERROR) {
    dev_err(&cl.device, "RX error detected\n");
    ssip_release_cmd(msg);
    ssip_error(cl);
    return;
    }
    hsi_async_read(cl, msg);
    dev_dbg(&cl.device, "RX cmd: 0x%08x\n", cmd);
    switch (cmdid) {
    case SSIP_SW_BREAK:
// Ignored
    break;
    case SSIP_BOOTINFO_REQ:
    ssip_rx_bootinforeq(cl, cmd);
    break;
    case SSIP_BOOTINFO_RESP:
    ssip_rx_bootinforesp(cl, cmd);
    break;
    case SSIP_WAKETEST_RESULT:
    ssip_rx_waketest(cl, cmd);
    break;
    case SSIP_START_TRANS:
    ssip_rx_strans(cl, cmd);
    break;
    case SSIP_READY:
    ssip_rx_ready(cl);
    break;
    default:
    dev_warn(&cl.device, "command 0x%08x not supported\n", cmd);
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn ssip_swbreak_complete(msg: *mut hsi_msg) {
    static void ssip_swbreak_complete(struct hsi_msg *msg)
    {
    struct hsi_client *cl = msg.cl;
    struct ssi_protocol *ssi = hsi_client_drvdata(cl);
    ssip_release_cmd(msg);
    spin_lock_bh(&ssi.lock);
    if (list_empty(&ssi.txqueue)) {
    if (atomic_read(&ssi.tx_usecnt)) {
    ssip_set_txstate(ssi, SEND_READY);
    } else {
    ssip_set_txstate(ssi, SEND_IDLE);
    hsi_stop_tx(cl);
    }
    spin_unlock_bh(&ssi.lock);
    } else {
    spin_unlock_bh(&ssi.lock);
    ssip_xmit(cl);
    }
    netif_wake_queue(ssi.netdev);
    }
#[no_mangle]
unsafe extern "C" fn ssip_tx_data_complete(msg: *mut hsi_msg) {
    static void ssip_tx_data_complete(struct hsi_msg *msg)
    {
    struct hsi_client *cl = msg.cl;
    struct ssi_protocol *ssi = hsi_client_drvdata(cl);
    struct hsi_msg *cmsg;
    if (msg.status == HSI_STATUS_ERROR) {
    dev_err(&cl.device, "TX data error\n");
    ssip_error(cl);
    goto out;
    }
    spin_lock_bh(&ssi.lock);
    if (list_empty(&ssi.txqueue)) {
    ssip_set_txstate(ssi, SENDING_SWBREAK);
    spin_unlock_bh(&ssi.lock);
    cmsg = ssip_claim_cmd(ssi);
    ssip_set_cmd(cmsg, SSIP_SWBREAK_CMD);
    cmsg.complete = ssip_swbreak_complete;
    dev_dbg(&cl.device, "Send SWBREAK\n");
    hsi_async_write(cl, cmsg);
    } else {
    spin_unlock_bh(&ssi.lock);
    ssip_xmit(cl);
    }
    out:
    ssip_free_data(msg);
    }
#[no_mangle]
unsafe extern "C" fn ssip_port_event(cl: *mut hsi_client, event: c_ulong) {
    static void ssip_port_event(struct hsi_client *cl, unsigned long event)
    {
    switch (event) {
    case HSI_EVENT_START_RX:
    ssip_start_rx(cl);
    break;
    case HSI_EVENT_STOP_RX:
    ssip_stop_rx(cl);
    break;
    default:
    return;
    }
    }
#[no_mangle]
unsafe extern "C" fn ssip_pn_open(dev: *mut net_device) -> c_int {
    static int ssip_pn_open(struct net_device *dev)
    {
    struct hsi_client *cl = to_hsi_client(dev.dev.parent);
    struct ssi_protocol *ssi = hsi_client_drvdata(cl);
    int err;
    err = hsi_claim_port(cl, 1);
    if (err < 0) {
    dev_err(&cl.device, "SSI port already claimed\n");
    return err;
    }
    err = hsi_register_port_event(cl, ssip_port_event);
    if (err < 0) {
    dev_err(&cl.device, "Register HSI port event failed (%d)\n",
    err);
    hsi_release_port(cl);
    return err;
    }
    dev_dbg(&cl.device, "Configuring SSI port\n");
    hsi_setup(cl);
    if (!test_and_set_bit(SSIP_WAKETEST_FLAG, &ssi.flags))
    ssi_waketest(cl, 1); /* FIXME: To be removed */
    spin_lock_bh(&ssi.lock);
    ssi.main_state = HANDSHAKE;
    spin_unlock_bh(&ssi.lock);
    ssip_send_bootinfo_req_cmd(cl);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ssip_pn_stop(dev: *mut net_device) -> c_int {
    static int ssip_pn_stop(struct net_device *dev)
    {
    struct hsi_client *cl = to_hsi_client(dev.dev.parent);
    ssip_reset(cl);
    hsi_unregister_port_event(cl);
    hsi_release_port(cl);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ssip_xmit_work(work: *mut work_struct) {
    static void ssip_xmit_work(struct work_struct *work)
    {
    struct ssi_protocol *ssi =
    container_of(work, struct ssi_protocol, work);
    struct hsi_client *cl = ssi.cl;
    ssip_xmit(cl);
    }
#[no_mangle]
unsafe extern "C" fn ssip_pn_xmit(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t {
    static netdev_tx_t ssip_pn_xmit(struct sk_buff *skb, struct net_device *dev)
    {
    struct hsi_client *cl = to_hsi_client(dev.dev.parent);
    struct ssi_protocol *ssi = hsi_client_drvdata(cl);
    struct hsi_msg *msg;
    if ((skb.protocol != htons(ETH_P_PHONET)) ||
    (skb.len < SSIP_MIN_PN_HDR))
    goto drop;
// Pad to 32-bits - FIXME: Revisit
    if ((skb.len & 3) && skb_pad(skb, 4 - (skb.len & 3)))
    goto inc_dropped;
//
// Modem sends Phonet messages over SSI with its own endianness.
// Assume that modem has the same endianness as we do.
//
    if (skb_cow_head(skb, 0))
    goto drop;
// length field is exchanged in network byte order
    ((u16 *)skb.data)[2] = htons(((u16 *)skb.data)[2]);
    msg = ssip_alloc_data(ssi, skb, GFP_ATOMIC);
    if (!msg) {
    dev_dbg(&cl.device, "Dropping tx data: No memory\n");
    goto drop;
    }
    msg.complete = ssip_tx_data_complete;
    spin_lock_bh(&ssi.lock);
    if (unlikely(ssi.main_state != ACTIVE)) {
    spin_unlock_bh(&ssi.lock);
    dev_dbg(&cl.device, "Dropping tx data: CMT is OFFLINE\n");
    goto drop2;
    }
    list_add_tail(&msg.link, &ssi.txqueue);
    ssi.txqueue_len++;
    if (dev.tx_queue_len < ssi.txqueue_len) {
    dev_info(&cl.device, "TX queue full %d\n", ssi.txqueue_len);
    netif_stop_queue(dev);
    }
    if (ssi.send_state == SEND_IDLE) {
    ssip_set_txstate(ssi, WAIT4READY);
    spin_unlock_bh(&ssi.lock);
    dev_dbg(&cl.device, "Start TX qlen %d\n", ssi.txqueue_len);
    hsi_start_tx(cl);
    } else if (ssi.send_state == SEND_READY) {
// Needed for cmt-speech workaround
    dev_dbg(&cl.device, "Start TX on SEND READY qlen %d\n",
    ssi.txqueue_len);
    spin_unlock_bh(&ssi.lock);
    schedule_work(&ssi.work);
    } else {
    spin_unlock_bh(&ssi.lock);
    }
    dev.stats.tx_packets++;
    dev.stats.tx_bytes += skb.len;
    return NETDEV_TX_OK;
    drop2:
    hsi_free_msg(msg);
    drop:
    dev_kfree_skb(skb);
    inc_dropped:
    dev.stats.tx_dropped++;
    return NETDEV_TX_OK;
    }
// CMT reset event handler
#[no_mangle]
pub unsafe extern "C" fn ssip_reset_event(master: *mut hsi_client) {
    void ssip_reset_event(struct hsi_client *master)
    {
    struct ssi_protocol *ssi = hsi_client_drvdata(master);
    dev_err(&ssi.cl.device, "CMT reset detected!\n");
    ssip_error(ssi.cl);
    }
    EXPORT_SYMBOL_GPL(ssip_reset_event);
    static const struct net_device_ops ssip_pn_ops = {
    .ndo_open	= ssip_pn_open,
    .ndo_stop	= ssip_pn_stop,
    .ndo_start_xmit	= ssip_pn_xmit,
    };
#[no_mangle]
unsafe extern "C" fn ssip_pn_setup(dev: *mut net_device) {
    static void ssip_pn_setup(struct net_device *dev)
    {
    let mut addr: static u8 = PN_MEDIA_SOS;
    dev.features		= 0;
    dev.netdev_ops		= &ssip_pn_ops;
    dev.type		= ARPHRD_PHONET;
    dev.flags		= IFF_POINTOPOINT | IFF_NOARP;
    dev.mtu		= SSIP_DEFAULT_MTU;
    dev.hard_header_len	= 1;
    dev.addr_len		= 1;
    dev_addr_set(dev, &addr);
    dev.tx_queue_len	= SSIP_TXQUEUE_LEN;
    dev.needs_free_netdev	= true;
    dev.header_ops		= &phonet_header_ops;
    }
#[no_mangle]
unsafe extern "C" fn ssi_protocol_probe(dev: *mut device) -> c_int {
    static int ssi_protocol_probe(struct device *dev)
    {
    static const char ifname[] = "phonet%d";
    struct hsi_client *cl = to_hsi_client(dev);
    struct ssi_protocol *ssi;
    int err;
    ssi = kzalloc_obj(*ssi);
    if (!ssi)
    return -ENOMEM;
    spin_lock_init(&ssi.lock);
    timer_setup(&ssi.rx_wd, ssip_rx_wd, TIMER_DEFERRABLE);
    timer_setup(&ssi.tx_wd, ssip_tx_wd, TIMER_DEFERRABLE);
    timer_setup(&ssi.keep_alive, ssip_keep_alive, 0);
    INIT_LIST_HEAD(&ssi.txqueue);
    INIT_LIST_HEAD(&ssi.cmdqueue);
    atomic_set(&ssi.tx_usecnt, 0);
    hsi_client_set_drvdata(cl, ssi);
    ssi.cl = cl;
    INIT_WORK(&ssi.work, ssip_xmit_work);
    ssi.channel_id_cmd = hsi_get_channel_id_by_name(cl, "mcsaab-control");
    if (ssi.channel_id_cmd < 0) {
    err = ssi.channel_id_cmd;
    dev_err(dev, "Could not get cmd channel (%d)\n", err);
    goto out;
    }
    ssi.channel_id_data = hsi_get_channel_id_by_name(cl, "mcsaab-data");
    if (ssi.channel_id_data < 0) {
    err = ssi.channel_id_data;
    dev_err(dev, "Could not get data channel (%d)\n", err);
    goto out;
    }
    err = ssip_alloc_cmds(ssi);
    if (err < 0) {
    dev_err(dev, "No memory for commands\n");
    goto out;
    }
    ssi.netdev = alloc_netdev(0, ifname, NET_NAME_UNKNOWN, ssip_pn_setup);
    if (!ssi.netdev) {
    dev_err(dev, "No memory for netdev\n");
    err = -ENOMEM;
    goto out1;
    }
// MTU range: 6 - 65535
    ssi.netdev.min_mtu = PHONET_MIN_MTU;
    ssi.netdev.max_mtu = SSIP_MAX_MTU;
    SET_NETDEV_DEV(ssi.netdev, dev);
    netif_carrier_off(ssi.netdev);
    err = register_netdev(ssi.netdev);
    if (err < 0) {
    dev_err(dev, "Register netdev failed (%d)\n", err);
    goto out2;
    }
    list_add(&ssi.link, &ssip_list);
    dev_dbg(dev, "channel configuration: cmd=%d, data=%d\n",
    ssi.channel_id_cmd, ssi.channel_id_data);
    return 0;
    out2:
    free_netdev(ssi.netdev);
    out1:
    ssip_free_cmds(ssi);
    out:
    kfree(ssi);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ssi_protocol_remove(dev: *mut device) -> c_int {
    static int ssi_protocol_remove(struct device *dev)
    {
    struct hsi_client *cl = to_hsi_client(dev);
    struct ssi_protocol *ssi = hsi_client_drvdata(cl);
    list_del(&ssi.link);
    unregister_netdev(ssi.netdev);
    ssip_free_cmds(ssi);
    hsi_client_set_drvdata(cl, core::ptr::null_mut());
    kfree(ssi);
    return 0;
    }
    static struct hsi_client_driver ssip_driver = {
    .driver = {
    .name	= "ssi-protocol",
    .owner	= THIS_MODULE,
    .probe	= ssi_protocol_probe,
    .remove	= ssi_protocol_remove,
    },
    };
#[no_mangle]
unsafe extern "C" fn ssip_init() -> int __init {
    static int __init ssip_init(void)
    {
    pr_info("SSI protocol aka McSAAB added\n");
    return hsi_register_client_driver(&ssip_driver);
    }
    module_init(ssip_init);
#[no_mangle]
unsafe extern "C" fn ssip_exit() -> void __exit {
    static void __exit ssip_exit(void)
    {
    hsi_unregister_client_driver(&ssip_driver);
    pr_info("SSI protocol driver removed\n");
    }
    module_exit(ssip_exit);
    MODULE_ALIAS("hsi:ssi-protocol");
    MODULE_AUTHOR("Carlos Chinea <carlos.chinea@nokia.com>");
    MODULE_AUTHOR("Remi Denis-Courmont <remi.denis-courmont@nokia.com>");
    MODULE_DESCRIPTION("SSI protocol improved aka McSAAB");
    MODULE_LICENSE("GPL");
