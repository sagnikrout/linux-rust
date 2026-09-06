//! Automatically rewritten from C to Rust
//! Source: drivers/ptp/ptp_ines.c
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
// Copyright (C) 2018 MOSER-BAER AG
//

    MODULE_DESCRIPTION("Driver for the ZHAW InES PTP time stamping IP core");
    MODULE_AUTHOR("Richard Cochran <richardcochran@gmail.com>");
    MODULE_VERSION("1.0");
    MODULE_LICENSE("GPL");
// GLOBAL register
pub const MCAST_MAC_SELECT_SHIFT: c_int = 2;
pub const MCAST_MAC_SELECT_MASK: c_uint = 0x3;

// VERSION register
pub const IF_MAJOR_VER_SHIFT: c_int = 12;
pub const IF_MAJOR_VER_MASK: c_uint = 0xf;
pub const IF_MINOR_VER_SHIFT: c_int = 8;
pub const IF_MINOR_VER_MASK: c_uint = 0xf;
pub const FPGA_MAJOR_VER_SHIFT: c_int = 4;
pub const FPGA_MAJOR_VER_MASK: c_uint = 0xf;
pub const FPGA_MINOR_VER_SHIFT: c_int = 0;
pub const FPGA_MINOR_VER_MASK: c_uint = 0xf;
// INT_STAT register

// INT_MSK register

// BUF_STAT register

// PORT_CONF register

pub const PHY_SPEED_SHIFT: c_int = 4;
pub const PHY_SPEED_MASK: c_uint = 0x3;
pub const P2P_DELAY_WR_POS_SHIFT: c_int = 2;
pub const P2P_DELAY_WR_POS_MASK: c_uint = 0x3;
pub const PTP_MODE_SHIFT: c_int = 0;
pub const PTP_MODE_MASK: c_uint = 0x3;
// TS_STAT_TX register

pub const DATA_READ_POS_SHIFT: c_int = 8;
pub const DATA_READ_POS_MASK: c_uint = 0x1f;
pub const DISCARDED_EVENTS_SHIFT: c_int = 4;
pub const DISCARDED_EVENTS_MASK: c_uint = 0xf;
pub const INES_N_PORTS: c_int = 3;
pub const INES_REGISTER_SIZE: c_uint = 0x80;
pub const INES_PORT_OFFSET: c_uint = 0x20;
pub const INES_PORT_SIZE: c_uint = 0x20;
pub const INES_FIFO_DEPTH: c_int = 90;
pub const INES_MAX_EVENTS: c_int = 100;
pub const BC_PTP_V1: c_int = 0;
pub const BC_PTP_V2: c_int = 1;
pub const TC_E2E_PTP_V2: c_int = 2;
pub const TC_P2P_PTP_V2: c_int = 3;
pub const PHY_SPEED_10: c_int = 0;
pub const PHY_SPEED_100: c_int = 1;
pub const PHY_SPEED_1000: c_int = 2;

    ((PHY_SPEED_1000 << PHY_SPEED_SHIFT) | (BC_PTP_V2 << PTP_MODE_SHIFT))

pub const MESSAGE_TYPE_SYNC: c_int = 1;
pub const MESSAGE_TYPE_P_DELAY_REQ: c_int = 2;
pub const MESSAGE_TYPE_P_DELAY_RESP: c_int = 3;
pub const MESSAGE_TYPE_DELAY_REQ: c_int = 4;
    static LIST_HEAD(ines_clocks);
    static DEFINE_MUTEX(ines_clocks_lock);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ines_global_regs {
    pub id: u32,
    pub test: u32,
    pub global: u32,
    pub version: u32,
    pub test2: u32,
    pub int_stat: u32,
    pub int_msk: u32,
    pub buf_stat: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ines_port_registers {
    pub port_conf: u32,
    pub p_delay: u32,
    pub ts_stat_tx: u32,
    pub ts_stat_rx: u32,
    pub ts_tx: u32,
    pub ts_rx: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ines_timestamp {
    pub list: list_head,
    pub tmo: c_ulong,
    pub tag: u16,
    pub sec: u64,
    pub nsec: u64,
    pub clkid: u64,
    pub portnum: u16,
    pub seqid: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ines_port {
    pub regs: *mut ines_port_registers,
    pub mii_ts: mii_timestamper,
    pub clock: *mut ines_clock,
    pub rxts_enabled: bool,
    pub txts_enabled: bool,
    pub index: c_uint,
    pub ts_work: delayed_work,
// lock protects event list and tx_skb
    pub lock: spinlock_t,
    pub tx_skb: *mut sk_buff,
    pub events: list_head,
    pub pool: list_head,
    pub pool_data: [ines_timestamp; INES_MAX_EVENTS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ines_clock {
    pub port: [ines_port; INES_N_PORTS],
    pub regs: *mut ines_global_regs __iomem,
    pub base: *mut void __iomem,
    pub node: *mut device_node,
    pub dev: *mut device,
    pub list: list_head,
}

    static bool ines_match(struct sk_buff *skb, unsigned int ptp_class,
    struct ines_timestamp *ts, struct device *dev);
    static int ines_rxfifo_read(struct ines_port *port);
    static u64 ines_rxts64(struct ines_port *port, unsigned int words);
    static bool ines_timestamp_expired(struct ines_timestamp *ts);
    static u64 ines_txts64(struct ines_port *port, unsigned int words);
    static void ines_txtstamp_work(struct work_struct *work);
    static bool is_sync_pdelay_resp(struct sk_buff *skb, int type);
    static u8 tag_to_msgtype(u8 tag);
#[no_mangle]
unsafe extern "C" fn ines_clock_cleanup(clock: *mut ines_clock) {
    static void ines_clock_cleanup(struct ines_clock *clock)
    {
    struct ines_port *port;
    int i;
    for (i = 0; i < INES_N_PORTS; i++) {
    port = &clock.port[i];
    cancel_delayed_work_sync(&port.ts_work);
    }
    }
    static int ines_clock_init(struct ines_clock *clock, struct device *device,
    void __iomem *addr)
    {
    struct device_node *node = device.of_node;
    unsigned long port_addr;
    struct ines_port *port;
    int i, j;
    INIT_LIST_HEAD(&clock.list);
    clock.node = node;
    clock.dev  = device;
    clock.base = addr;
    clock.regs = clock.base;
    for (i = 0; i < INES_N_PORTS; i++) {
    port = &clock.port[i];
    port_addr = (unsigned long) clock.base +
    INES_PORT_OFFSET + i * INES_PORT_SIZE;
    port.regs = (struct ines_port_registers *) port_addr;
    port.clock = clock;
    port.index = i;
    INIT_DELAYED_WORK(&port.ts_work, ines_txtstamp_work);
    spin_lock_init(&port.lock);
    INIT_LIST_HEAD(&port.events);
    INIT_LIST_HEAD(&port.pool);
    for (j = 0; j < INES_MAX_EVENTS; j++)
    list_add(&port.pool_data[j].list, &port.pool);
    }
    ines_write32(clock, 0xBEEF, test);
    ines_write32(clock, 0xBEEF, test2);
    dev_dbg(device, "ID      0x%x\n", ines_read32(clock, id));
    dev_dbg(device, "TEST    0x%x\n", ines_read32(clock, test));
    dev_dbg(device, "VERSION 0x%x\n", ines_read32(clock, version));
    dev_dbg(device, "TEST2   0x%x\n", ines_read32(clock, test2));
    for (i = 0; i < INES_N_PORTS; i++) {
    port = &clock.port[i];
    ines_write32(port, PORT_CONF, port_conf);
    }
    return 0;
    }
    static struct ines_port *ines_find_port(struct device_node *node, u32 index)
    {
    struct ines_port *port = core::ptr::null_mut();
    struct ines_clock *clock;
    struct list_head *this;
    mutex_lock(&ines_clocks_lock);
    list_for_each(this, &ines_clocks) {
    clock = list_entry(this, struct ines_clock, list);
    if (clock.node == node) {
    port = &clock.port[index];
    break;
    }
    }
    mutex_unlock(&ines_clocks_lock);
    return port;
    }
#[no_mangle]
unsafe extern "C" fn ines_find_rxts(port: *mut ines_port, skb: *mut sk_buff, type: c_int) -> u64 {
    static u64 ines_find_rxts(struct ines_port *port, struct sk_buff *skb, int type)
    {
    struct list_head *this, *next;
    struct ines_timestamp *ts;
    unsigned long flags;
    let mut ns: u64 = 0;
    if (type == PTP_CLASS_NONE)
    return 0;
    spin_lock_irqsave(&port.lock, flags);
    ines_rxfifo_read(port);
    list_for_each_safe(this, next, &port.events) {
    ts = list_entry(this, struct ines_timestamp, list);
    if (ines_timestamp_expired(ts)) {
    list_del_init(&ts.list);
    list_add(&ts.list, &port.pool);
    continue;
    }
    if (ines_match(skb, type, ts, port.clock.dev)) {
    ns = ts.sec * 1000000000ULL + ts.nsec;
    list_del_init(&ts.list);
    list_add(&ts.list, &port.pool);
    break;
    }
    }
    spin_unlock_irqrestore(&port.lock, flags);
    return ns;
    }
#[no_mangle]
unsafe extern "C" fn ines_find_txts(port: *mut ines_port, skb: *mut sk_buff) -> u64 {
    static u64 ines_find_txts(struct ines_port *port, struct sk_buff *skb)
    {
    let mut class: c_uint = ptp_classify_raw(skb), i;
    u32 data_rd_pos, buf_stat, mask, ts_stat_tx;
    struct ines_timestamp ts;
    unsigned long flags;
    let mut ns: u64 = 0;
    mask = TX_FIFO_NE_1 << port.index;
    spin_lock_irqsave(&port.lock, flags);
    for (i = 0; i < INES_FIFO_DEPTH; i++) {
    buf_stat = ines_read32(port.clock, buf_stat);
    if (!(buf_stat & mask)) {
    dev_dbg(port.clock.dev,
    "Tx timestamp FIFO unexpectedly empty\n");
    break;
    }
    ts_stat_tx = ines_read32(port, ts_stat_tx);
    data_rd_pos = (ts_stat_tx >> DATA_READ_POS_SHIFT) &
    DATA_READ_POS_MASK;
    if (data_rd_pos) {
    dev_err(port.clock.dev,
    "unexpected Tx read pos %u\n", data_rd_pos);
    break;
    }
    ts.tag     = ines_read32(port, ts_tx);
    ts.sec     = ines_txts64(port, 3);
    ts.nsec    = ines_txts64(port, 2);
    ts.clkid   = ines_txts64(port, 4);
    ts.portnum = ines_read32(port, ts_tx);
    ts.seqid   = ines_read32(port, ts_tx);
    if (ines_match(skb, class, &ts, port.clock.dev)) {
    ns = ts.sec * 1000000000ULL + ts.nsec;
    break;
    }
    }
    spin_unlock_irqrestore(&port.lock, flags);
    return ns;
    }
    static int ines_hwtstamp_get(struct mii_timestamper *mii_ts,
    struct kernel_hwtstamp_config *cfg)
    {
    struct ines_port *port = container_of(mii_ts, struct ines_port, mii_ts);
    unsigned long flags;
    u32 port_conf;
    cfg.rx_filter = port.rxts_enabled ? HWTSTAMP_FILTER_PTP_V2_EVENT
    : HWTSTAMP_FILTER_NONE;
    if (port.txts_enabled) {
    spin_lock_irqsave(&port.lock, flags);
    port_conf = ines_read32(port, port_conf);
    spin_unlock_irqrestore(&port.lock, flags);
    cfg.tx_type = (port_conf & CM_ONE_STEP) ? HWTSTAMP_TX_ONESTEP_P2P
    : HWTSTAMP_TX_OFF;
    } else {
    cfg.tx_type = HWTSTAMP_TX_OFF;
    }
    return 0;
    }
    static int ines_hwtstamp_set(struct mii_timestamper *mii_ts,
    struct kernel_hwtstamp_config *cfg,
    struct netlink_ext_ack *extack)
    {
    struct ines_port *port = container_of(mii_ts, struct ines_port, mii_ts);
    let mut cm_one_step: u32 = 0, port_conf, ts_stat_rx, ts_stat_tx;
    unsigned long flags;
    switch (cfg.tx_type) {
    case HWTSTAMP_TX_OFF:
    ts_stat_tx = 0;
    break;
    case HWTSTAMP_TX_ON:
    ts_stat_tx = TS_ENABLE;
    break;
    case HWTSTAMP_TX_ONESTEP_P2P:
    ts_stat_tx = TS_ENABLE;
    cm_one_step = CM_ONE_STEP;
    break;
    default:
    return -ERANGE;
    }
    switch (cfg.rx_filter) {
    case HWTSTAMP_FILTER_NONE:
    ts_stat_rx = 0;
    break;
    case HWTSTAMP_FILTER_ALL:
    case HWTSTAMP_FILTER_PTP_V1_L4_EVENT:
    case HWTSTAMP_FILTER_PTP_V1_L4_SYNC:
    case HWTSTAMP_FILTER_PTP_V1_L4_DELAY_REQ:
    return -ERANGE;
    case HWTSTAMP_FILTER_PTP_V2_L4_EVENT:
    case HWTSTAMP_FILTER_PTP_V2_L4_SYNC:
    case HWTSTAMP_FILTER_PTP_V2_L4_DELAY_REQ:
    case HWTSTAMP_FILTER_PTP_V2_L2_EVENT:
    case HWTSTAMP_FILTER_PTP_V2_L2_SYNC:
    case HWTSTAMP_FILTER_PTP_V2_L2_DELAY_REQ:
    case HWTSTAMP_FILTER_PTP_V2_EVENT:
    case HWTSTAMP_FILTER_PTP_V2_SYNC:
    case HWTSTAMP_FILTER_PTP_V2_DELAY_REQ:
    ts_stat_rx = TS_ENABLE;
    cfg.rx_filter = HWTSTAMP_FILTER_PTP_V2_EVENT;
    break;
    default:
    return -ERANGE;
    }
    spin_lock_irqsave(&port.lock, flags);
    port_conf = ines_read32(port, port_conf);
    port_conf &= ~CM_ONE_STEP;
    port_conf |= cm_one_step;
    ines_write32(port, port_conf, port_conf);
    ines_write32(port, ts_stat_rx, ts_stat_rx);
    ines_write32(port, ts_stat_tx, ts_stat_tx);
    port.rxts_enabled = ts_stat_rx == TS_ENABLE;
    port.txts_enabled = ts_stat_tx == TS_ENABLE;
    spin_unlock_irqrestore(&port.lock, flags);
    return 0;
    }
    static void ines_link_state(struct mii_timestamper *mii_ts,
    struct phy_device *phydev)
    {
    struct ines_port *port = container_of(mii_ts, struct ines_port, mii_ts);
    u32 port_conf, speed_conf;
    unsigned long flags;
    switch (phydev.speed) {
    case SPEED_10:
    speed_conf = PHY_SPEED_10 << PHY_SPEED_SHIFT;
    break;
    case SPEED_100:
    speed_conf = PHY_SPEED_100 << PHY_SPEED_SHIFT;
    break;
    case SPEED_1000:
    speed_conf = PHY_SPEED_1000 << PHY_SPEED_SHIFT;
    break;
    default:
    dev_err(port.clock.dev, "bad speed: %d\n", phydev.speed);
    return;
    }
    spin_lock_irqsave(&port.lock, flags);
    port_conf = ines_read32(port, port_conf);
    port_conf &= ~(0x3 << PHY_SPEED_SHIFT);
    port_conf |= speed_conf;
    ines_write32(port, port_conf, port_conf);
    spin_unlock_irqrestore(&port.lock, flags);
    }
    static bool ines_match(struct sk_buff *skb, unsigned int ptp_class,
    struct ines_timestamp *ts, struct device *dev)
    {
    struct ptp_header *hdr;
    u16 portn, seqid;
    u8 msgtype;
    u64 clkid;
    if (unlikely(ptp_class & PTP_CLASS_V1))
    return false;
    hdr = ptp_parse_header(skb, ptp_class);
    if (!hdr)
    return false;
    msgtype = ptp_get_msgtype(hdr, ptp_class);
    clkid = be64_to_cpup((__be64 *)&hdr.source_port_identity.clock_identity.id[0]);
    portn = be16_to_cpu(hdr.source_port_identity.port_number);
    seqid = be16_to_cpu(hdr.sequence_id);
    if (tag_to_msgtype(ts.tag & 0x7) != msgtype) {
    dev_dbg(dev, "msgtype mismatch ts %hhu != skb %hhu\n",
    tag_to_msgtype(ts.tag & 0x7), msgtype);
    return false;
    }
    if (ts.clkid != clkid) {
    dev_dbg(dev, "clkid mismatch ts %llx != skb %llx\n",
    ts.clkid, clkid);
    return false;
    }
    if (ts.portnum != portn) {
    dev_dbg(dev, "portn mismatch ts %hu != skb %hu\n",
    ts.portnum, portn);
    return false;
    }
    if (ts.seqid != seqid) {
    dev_dbg(dev, "seqid mismatch ts %hu != skb %hu\n",
    ts.seqid, seqid);
    return false;
    }
    return true;
    }
    static bool ines_rxtstamp(struct mii_timestamper *mii_ts,
    struct sk_buff *skb, int type)
    {
    struct ines_port *port = container_of(mii_ts, struct ines_port, mii_ts);
    struct skb_shared_hwtstamps *ssh;
    u64 ns;
    if (!port.rxts_enabled)
    return false;
    ns = ines_find_rxts(port, skb, type);
    if (!ns)
    return false;
    ssh = skb_hwtstamps(skb);
    ssh.hwtstamp = ns_to_ktime(ns);
    netif_rx(skb);
    return true;
    }
#[no_mangle]
unsafe extern "C" fn ines_rxfifo_read(port: *mut ines_port) -> c_int {
    static int ines_rxfifo_read(struct ines_port *port)
    {
    u32 data_rd_pos, buf_stat, mask, ts_stat_rx;
    struct ines_timestamp *ts;
    unsigned int i;
    mask = RX_FIFO_NE_1 << port.index;
    for (i = 0; i < INES_FIFO_DEPTH; i++) {
    if (list_empty(&port.pool)) {
    dev_err(port.clock.dev, "event pool is empty\n");
    return -1;
    }
    buf_stat = ines_read32(port.clock, buf_stat);
    if (!(buf_stat & mask))
    break;
    ts_stat_rx = ines_read32(port, ts_stat_rx);
    data_rd_pos = (ts_stat_rx >> DATA_READ_POS_SHIFT) &
    DATA_READ_POS_MASK;
    if (data_rd_pos) {
    dev_err(port.clock.dev, "unexpected Rx read pos %u\n",
    data_rd_pos);
    break;
    }
    ts = list_first_entry(&port.pool, struct ines_timestamp, list);
    ts.tmo     = jiffies + HZ;
    ts.tag     = ines_read32(port, ts_rx);
    ts.sec     = ines_rxts64(port, 3);
    ts.nsec    = ines_rxts64(port, 2);
    ts.clkid   = ines_rxts64(port, 4);
    ts.portnum = ines_read32(port, ts_rx);
    ts.seqid   = ines_read32(port, ts_rx);
    list_del_init(&ts.list);
    list_add_tail(&ts.list, &port.events);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ines_rxts64(port: *mut ines_port, words: c_uint) -> u64 {
    static u64 ines_rxts64(struct ines_port *port, unsigned int words)
    {
    unsigned int i;
    u64 result;
    u16 word;
    word = ines_read32(port, ts_rx);
    result = word;
    words--;
    for (i = 0; i < words; i++) {
    word = ines_read32(port, ts_rx);
    result <<= 16;
    result |= word;
    }
    return result;
    }
#[no_mangle]
unsafe extern "C" fn ines_timestamp_expired(ts: *mut ines_timestamp) -> bool {
    static bool ines_timestamp_expired(struct ines_timestamp *ts)
    {
    return time_after(jiffies, ts.tmo);
    }
    static int ines_ts_info(struct mii_timestamper *mii_ts,
    struct kernel_ethtool_ts_info *info)
    {
    info.so_timestamping =
    SOF_TIMESTAMPING_TX_HARDWARE |
    SOF_TIMESTAMPING_TX_SOFTWARE |
    SOF_TIMESTAMPING_RX_HARDWARE |
    SOF_TIMESTAMPING_RAW_HARDWARE;
    info.tx_types =
    (1 << HWTSTAMP_TX_OFF) |
    (1 << HWTSTAMP_TX_ON) |
    (1 << HWTSTAMP_TX_ONESTEP_P2P);
    info.rx_filters =
    (1 << HWTSTAMP_FILTER_NONE) |
    (1 << HWTSTAMP_FILTER_PTP_V2_EVENT);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ines_txts64(port: *mut ines_port, words: c_uint) -> u64 {
    static u64 ines_txts64(struct ines_port *port, unsigned int words)
    {
    unsigned int i;
    u64 result;
    u16 word;
    word = ines_read32(port, ts_tx);
    result = word;
    words--;
    for (i = 0; i < words; i++) {
    word = ines_read32(port, ts_tx);
    result <<= 16;
    result |= word;
    }
    return result;
    }
#[no_mangle]
unsafe extern "C" fn ines_txts_onestep(port: *mut ines_port, skb: *mut sk_buff, type: c_int) -> bool {
    static bool ines_txts_onestep(struct ines_port *port, struct sk_buff *skb, int type)
    {
    unsigned long flags;
    u32 port_conf;
    spin_lock_irqsave(&port.lock, flags);
    port_conf = ines_read32(port, port_conf);
    spin_unlock_irqrestore(&port.lock, flags);
    if (port_conf & CM_ONE_STEP)
    return is_sync_pdelay_resp(skb, type);
    return false;
    }
    static void ines_txtstamp(struct mii_timestamper *mii_ts,
    struct sk_buff *skb, int type)
    {
    struct ines_port *port = container_of(mii_ts, struct ines_port, mii_ts);
    struct sk_buff *old_skb = core::ptr::null_mut();
    unsigned long flags;
    if (!port.txts_enabled || ines_txts_onestep(port, skb, type)) {
    kfree_skb(skb);
    return;
    }
    spin_lock_irqsave(&port.lock, flags);
    if (port.tx_skb)
    old_skb = port.tx_skb;
    port.tx_skb = skb;
    spin_unlock_irqrestore(&port.lock, flags);
    kfree_skb(old_skb);
    schedule_delayed_work(&port.ts_work, 1);
    }
#[no_mangle]
unsafe extern "C" fn ines_txtstamp_work(work: *mut work_struct) {
    static void ines_txtstamp_work(struct work_struct *work)
    {
    struct ines_port *port =
    container_of(work, struct ines_port, ts_work.work);
    struct skb_shared_hwtstamps ssh;
    struct sk_buff *skb;
    unsigned long flags;
    u64 ns;
    spin_lock_irqsave(&port.lock, flags);
    skb = port.tx_skb;
    port.tx_skb = core::ptr::null_mut();
    spin_unlock_irqrestore(&port.lock, flags);
    ns = ines_find_txts(port, skb);
    if (!ns) {
    kfree_skb(skb);
    return;
    }
    ssh.hwtstamp = ns_to_ktime(ns);
    skb_complete_tx_timestamp(skb, &ssh);
    }
#[no_mangle]
unsafe extern "C" fn is_sync_pdelay_resp(skb: *mut sk_buff, type: c_int) -> bool {
    static bool is_sync_pdelay_resp(struct sk_buff *skb, int type)
    {
    struct ptp_header *hdr;
    u8 msgtype;
    hdr = ptp_parse_header(skb, type);
    if (!hdr)
    return false;
    msgtype = ptp_get_msgtype(hdr, type);
    switch (msgtype) {
    case PTP_MSGTYPE_SYNC:
    case PTP_MSGTYPE_PDELAY_RESP:
    return true;
    default:
    return false;
    }
    }
#[no_mangle]
unsafe extern "C" fn tag_to_msgtype(tag: u8) -> u8 {
    static u8 tag_to_msgtype(u8 tag)
    {
    switch (tag) {
    case MESSAGE_TYPE_SYNC:
    return PTP_MSGTYPE_SYNC;
    case MESSAGE_TYPE_P_DELAY_REQ:
    return PTP_MSGTYPE_PDELAY_REQ;
    case MESSAGE_TYPE_P_DELAY_RESP:
    return PTP_MSGTYPE_PDELAY_RESP;
    case MESSAGE_TYPE_DELAY_REQ:
    return PTP_MSGTYPE_DELAY_REQ;
    }
    return 0xf;
    }
    static struct mii_timestamper *ines_ptp_probe_channel(struct device *device,
    unsigned int index)
    {
    struct device_node *node = device.of_node;
    struct ines_port *port;
    if (index > INES_N_PORTS - 1) {
    dev_err(device, "bad port index %u\n", index);
    return ERR_PTR(-EINVAL);
    }
    port = ines_find_port(node, index);
    if (!port) {
    dev_err(device, "missing port index %u\n", index);
    return ERR_PTR(-ENODEV);
    }
    port.mii_ts.rxtstamp = ines_rxtstamp;
    port.mii_ts.txtstamp = ines_txtstamp;
    port.mii_ts.hwtstamp_set = ines_hwtstamp_set;
    port.mii_ts.hwtstamp_get = ines_hwtstamp_get;
    port.mii_ts.link_state = ines_link_state;
    port.mii_ts.ts_info = ines_ts_info;
    return &port.mii_ts;
    }
    static void ines_ptp_release_channel(struct device *device,
    struct mii_timestamper *mii_ts)
    {
    }
    static struct mii_timestamping_ctrl ines_ctrl = {
    .probe_channel = ines_ptp_probe_channel,
    .release_channel = ines_ptp_release_channel,
    };
#[no_mangle]
unsafe extern "C" fn ines_ptp_ctrl_probe(pld: *mut platform_device) -> c_int {
    static int ines_ptp_ctrl_probe(struct platform_device *pld)
    {
    struct ines_clock *clock;
    void __iomem *addr;
    let mut err: c_int = 0;
    addr = devm_platform_ioremap_resource(pld, 0);
    if (IS_ERR(addr)) {
    err = PTR_ERR(addr);
    goto out;
    }
    clock = kzalloc_obj(*clock);
    if (!clock) {
    err = -ENOMEM;
    goto out;
    }
    if (ines_clock_init(clock, &pld.dev, addr)) {
    kfree(clock);
    err = -ENOMEM;
    goto out;
    }
    err = register_mii_tstamp_controller(&pld.dev, &ines_ctrl);
    if (err) {
    kfree(clock);
    goto out;
    }
    mutex_lock(&ines_clocks_lock);
    list_add_tail(&ines_clocks, &clock.list);
    mutex_unlock(&ines_clocks_lock);
    dev_set_drvdata(&pld.dev, clock);
    out:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ines_ptp_ctrl_remove(pld: *mut platform_device) {
    static void ines_ptp_ctrl_remove(struct platform_device *pld)
    {
    struct ines_clock *clock = dev_get_drvdata(&pld.dev);
    unregister_mii_tstamp_controller(&pld.dev);
    mutex_lock(&ines_clocks_lock);
    list_del(&clock.list);
    mutex_unlock(&ines_clocks_lock);
    ines_clock_cleanup(clock);
    kfree(clock);
    }
    static const struct of_device_id ines_ptp_ctrl_of_match[] = {
    { .compatible = "ines,ptp-ctrl" },
    { }
    };
    MODULE_DEVICE_TABLE(of, ines_ptp_ctrl_of_match);
    static struct platform_driver ines_ptp_ctrl_driver = {
    .probe  = ines_ptp_ctrl_probe,
    .remove = ines_ptp_ctrl_remove,
    .driver = {
    .name = "ines_ptp_ctrl",
    .of_match_table = ines_ptp_ctrl_of_match,
    },
    };
    module_platform_driver(ines_ptp_ctrl_driver);
