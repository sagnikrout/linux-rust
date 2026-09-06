//! Automatically rewritten from C to Rust
//! Source: drivers/net/mctp/mctp-i3c.c
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
// Implements DMTF specification
// "DSP0233 Management Component Transport Protocol (MCTP) I3C Transport
// Binding"
// https://www.dmtf.org/sites/default/files/standards/documents/DSP0233_1.0.0.pdf
//
// Copyright (c) 2023 Code Construct
//

pub const MCTP_I3C_MAXBUF: c_int = 65536;
// 48 bit Provisioned Id
pub const PID_SIZE: c_int = 6;
// 64 byte payload, 4 byte MCTP header
    let mut MCTP_I3C_MINMTU: static int = 64 + 4;
// One byte less to allow for the PEC
    let mut MCTP_I3C_MAXMTU: static int = MCTP_I3C_MAXBUF - 1;
// 4 byte MCTP header, no data, 1 byte PEC
    let mut MCTP_I3C_MINLEN: static int = 4 + 1;
// Sufficient for 64kB at min mtu
    let mut MCTP_I3C_TX_QUEUE_LEN: static int = 1100;
// Somewhat arbitrary
    let mut MCTP_I3C_IBI_SLOTS: static int = 8;
// Mandatory Data Byte in an IBI, from DSP0233
pub const I3C_MDB_MCTP: c_uint = 0xAE;
// From MIPI Device Characteristics Register (DCR) Assignments
pub const I3C_DCR_MCTP: c_uint = 0xCC;
    static const char *MCTP_I3C_OF_PROP = "mctp-controller";
// List of mctp_i3c_busdev
    static LIST_HEAD(busdevs);
// Protects busdevs, as well as mctp_i3c_bus.devs lists
    static DEFINE_MUTEX(busdevs_lock);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mctp_i3c_bus {
    pub ndev: *mut net_device,
    pub tx_thread: *mut task_struct,
    pub tx_wq: wait_queue_head_t,
// tx_lock protects tx_skb and devs
    pub tx_lock: spinlock_t,
// Next skb to transmit
    pub tx_skb: *mut sk_buff,
// Scratch buffer for xmit
    pub tx_scratch: [u8; MCTP_I3C_MAXBUF],
// Element of busdevs
    pub list: list_head,
// Provisioned ID of our controller
    pub pid: u64,
    pub bus: *mut i3c_bus,
// Head of mctp_i3c_device.list. Protected by busdevs_lock
    pub devs: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mctp_i3c_device {
    pub i3c: *mut i3c_device,
    pub mbus: *mut mctp_i3c_bus,
    pub /: *mut *mut list_head list; / Element of mctp_i3c_bus.devs,
// Held while tx_thread is using this device
    pub lock: mutex,
// Whether BCR indicates MDB is present in IBI
    pub have_mdb: bool,
// I3C dynamic address
    pub addr: u8,
// Maximum read length
    pub mrl: u16,
// Maximum write length
    pub mwl: u16,
// Provisioned ID
    pub pid: u64,
}

// We synthesise a mac header using the Provisioned ID.
// Used to pass dest to mctp_i3c_start_xmit.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mctp_i3c_internal_hdr {
    pub dest: [u8; PID_SIZE],
    pub source: [u8; PID_SIZE],
    pub __packed: },
#[no_mangle]
unsafe extern "C" fn mctp_i3c_read(mi: *mut mctp_i3c_device) -> c_int {
    static int mctp_i3c_read(struct mctp_i3c_device *mi)
    {
    pub }: i3c_xfer xfer = { .rnw = 1, .len = mi->mrl,
    pub &mi->mbus->ndev->stats: *mut *mut net_device_stats stats =,
    pub NULL: *mut *mut mctp_i3c_internal_hdr ihdr =,
    pub NULL: *mut *mut sk_buff skb =,
    pub cb: *mut mctp_skb_cb,
    pub rc: int net_status,,
    pub addr: u8 pec,,
    skb = netdev_alloc_skb(mi.mbus.ndev,
    pub mctp_i3c_internal_hdr)): mi->mrl + sizeof(struct,
    if (!skb) {
    pub -ENOMEM: rc =,
    pub err: goto,
    }
    pub htons(ETH_P_MCTP): skb->protocol =,
// Create a header for internal use
    pub mctp_i3c_internal_hdr)): ihdr = skb_put(skb, sizeof(struct,
    pub ihdr->source): put_unaligned_be48(mi->pid,,
    pub ihdr->dest): put_unaligned_be48(mi->mbus->pid,,
    pub mctp_i3c_internal_hdr)): skb_pull(skb, sizeof(struct,
    pub mi->mrl): xfer.data.in = skb_put(skb,,
// Make sure netif_rx() is read in the same order as i3c.
    pub I3C_SDR): rc = i3c_device_do_xfers(mi->i3c, &xfer, 1,,
    if (rc < 0)
    pub err: goto,
    if (WARN_ON_ONCE(xfer.len > mi.mrl)) {
// Bad i3c bus driver
    pub -EIO: rc =,
    pub err: goto,
    }
    if (xfer.len < MCTP_I3C_MINLEN) {
    pub -EIO: rc =,
    pub err: goto,
    }
// check PEC, including address byte
    pub 1: addr = mi->addr << 1 |,
    pub 1): pec = i2c_smbus_pec(0, &addr,,
    pub 1): pec = i2c_smbus_pec(pec, xfer.data.in, xfer.len -,
    if (pec != ((u8 *)xfer.data.in)[xfer.len - 1]) {
    pub -EINVAL: rc =,
    pub err: goto,
    }
// Remove PEC
    pub 1): skb_trim(skb, xfer.len -,
    pub __mctp_cb(skb): cb =,
    pub PID_SIZE: cb->halen =,
    pub cb->haddr): put_unaligned_be48(mi->pid,,
    pub netif_rx(skb): net_status =,
    if (net_status == NET_RX_SUCCESS) {
    pub 1: stats->rx_bytes += xfer.len -,
    } else {
    }
    pub 0: return,
    err:
    pub rc: return,
    }
    static void mctp_i3c_ibi_handler(struct i3c_device *i3c,
    const struct i3c_ibi_payload *payload)
    {
    pub i3cdev_get_drvdata(i3c): *mut *mut mctp_i3c_device mi =,
    if (WARN_ON_ONCE(!mi))
    if (mi.have_mdb) {
    if (payload.len > 0) {
    if (((u8 *)payload.data)[0] != I3C_MDB_MCTP) {
// Not a mctp-i3c interrupt, ignore it
    }
    } else {
// The BCR advertised a Mandatory Data Byte but the
// device didn't send one.
//
    pub MDB"): dev_warn_once(i3cdev_to_dev(i3c), "IBI with missing,
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn mctp_i3c_setup(mi: *mut mctp_i3c_device) -> c_int {
    static int mctp_i3c_setup(struct mctp_i3c_device *mi)
    {
    const struct i3c_ibi_setup ibi = {
    .max_payload_len = 1,
    .num_slots = MCTP_I3C_IBI_SLOTS,
    .handler = mctp_i3c_ibi_handler,
}

    struct i3c_device_info info;
    int rc;
    i3c_device_get_info(mi.i3c, &info);
    mi.have_mdb = info.bcr & BIT(2);
    mi.addr = info.dyn_addr;
    mi.mwl = info.max_write_len;
    mi.mrl = info.max_read_len;
    mi.pid = info.pid;
    rc = i3c_device_request_ibi(mi.i3c, &ibi);
    if (rc == -ENOTSUPP) {
// This driver only supports In-Band Interrupt mode.
// Support for Polling Mode could be added if required.
// (ENOTSUPP is from the i3c layer, not EOPNOTSUPP).
//
    dev_warn(i3cdev_to_dev(mi.i3c),
    "Failed, bus driver doesn't support In-Band Interrupts");
    goto err;
    } else if (rc < 0) {
    dev_err(i3cdev_to_dev(mi.i3c),
    "Failed requesting IBI (%d)\n", rc);
    goto err;
    }
    rc = i3c_device_enable_ibi(mi.i3c);
    if (rc < 0) {
// Assume a driver supporting request_ibi also
// supports enable_ibi.
//
    dev_err(i3cdev_to_dev(mi.i3c), "Failed enabling IBI (%d)\n", rc);
    goto err_free_ibi;
    }
    return 0;
    err_free_ibi:
    i3c_device_free_ibi(mi.i3c);
    err:
    return rc;
    }
// Adds a new MCTP i3c_device to a bus
    static int mctp_i3c_add_device(struct mctp_i3c_bus *mbus,
    struct i3c_device *i3c)
    __must_hold(&busdevs_lock)
    {
    struct mctp_i3c_device *mi = core::ptr::null_mut();
    int rc;
    mi = kzalloc_obj(*mi);
    if (!mi) {
    rc = -ENOMEM;
    goto err;
    }
    mi.mbus = mbus;
    mi.i3c = i3c;
    mutex_init(&mi.lock);
    list_add(&mi.list, &mbus.devs);
    i3cdev_set_drvdata(i3c, mi);
    rc = mctp_i3c_setup(mi);
    if (rc < 0)
    goto err_free;
    return 0;
    err_free:
    list_del(&mi.list);
    kfree(mi);
    err:
    dev_warn(i3cdev_to_dev(i3c), "Error adding mctp-i3c device, %d\n", rc);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn mctp_i3c_probe(i3c: *mut i3c_device) -> c_int {
    static int mctp_i3c_probe(struct i3c_device *i3c)
    {
    struct mctp_i3c_bus *b = core::ptr::null_mut(), *mbus = core::ptr::null_mut();
// Look for a known bus
    mutex_lock(&busdevs_lock);
    list_for_each_entry(b, &busdevs, list)
    if (b.bus == i3c.bus) {
    mbus = b;
    break;
    }
    mutex_unlock(&busdevs_lock);
    if (!mbus) {
// probably no "mctp-controller" property on the i3c bus
    return -ENODEV;
    }
    return mctp_i3c_add_device(mbus, i3c);
    }
#[no_mangle]
unsafe extern "C" fn mctp_i3c_remove_device(mi: *mut mctp_i3c_device) {
    static void mctp_i3c_remove_device(struct mctp_i3c_device *mi)
    __must_hold(&busdevs_lock)
    {
// Ensure the tx thread isn't using the device
    mutex_lock(&mi.lock);
// Counterpart of mctp_i3c_setup
    i3c_device_disable_ibi(mi.i3c);
    i3c_device_free_ibi(mi.i3c);
// Counterpart of mctp_i3c_add_device
    i3cdev_set_drvdata(mi.i3c, core::ptr::null_mut());
    list_del(&mi.list);
// Safe to unlock after removing from the list
    mutex_unlock(&mi.lock);
    kfree(mi);
    }
#[no_mangle]
unsafe extern "C" fn mctp_i3c_remove(i3c: *mut i3c_device) {
    static void mctp_i3c_remove(struct i3c_device *i3c)
    {
    struct mctp_i3c_device *mi = i3cdev_get_drvdata(i3c);
// We my have received a Bus Remove notify prior to device remove,
// so mi will already be removed.
//
    if (!mi)
    return;
    mutex_lock(&busdevs_lock);
    mctp_i3c_remove_device(mi);
    mutex_unlock(&busdevs_lock);
    }
// Returns the device for an address, with mi->lock held
    static struct mctp_i3c_device *
    mctp_i3c_lookup(struct mctp_i3c_bus *mbus, u64 pid)
    {
    struct mctp_i3c_device *mi = core::ptr::null_mut(), *ret = core::ptr::null_mut();
    mutex_lock(&busdevs_lock);
    list_for_each_entry(mi, &mbus.devs, list)
    if (mi.pid == pid) {
    ret = mi;
    mutex_lock(&mi.lock);
    break;
    }
    mutex_unlock(&busdevs_lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mctp_i3c_xmit(mbus: *mut mctp_i3c_bus, skb: *mut sk_buff) {
    static void mctp_i3c_xmit(struct mctp_i3c_bus *mbus, struct sk_buff *skb)
    {
    struct net_device_stats *stats = &mbus.ndev.stats;
    let mut xfer: i3c_xfer = { .rnw = false };
    struct mctp_i3c_internal_hdr *ihdr = core::ptr::null_mut();
    struct mctp_i3c_device *mi = core::ptr::null_mut();
    unsigned int data_len;
    u8 *data = core::ptr::null_mut();
    u8 addr, pec;
    let mut rc: c_int = 0;
    u64 pid;
    skb_pull(skb, sizeof(struct mctp_i3c_internal_hdr));
    data_len = skb.len;
    ihdr = (void *)skb_mac_header(skb);
    pid = get_unaligned_be48(ihdr.dest);
    mi = mctp_i3c_lookup(mbus, pid);
    if (!mi) {
// I3C endpoint went away after the packet was enqueued?
    stats.tx_dropped++;
    goto out;
    }
    if (WARN_ON_ONCE(data_len + 1 > MCTP_I3C_MAXBUF))
    goto out;
    if (data_len + 1 > (unsigned int)mi.mwl) {
// Route MTU was larger than supported by the endpoint
    stats.tx_dropped++;
    goto out;
    }
// Need a linear buffer with space for the PEC
    xfer.len = data_len + 1;
    if (skb_tailroom(skb) >= 1) {
    skb_put(skb, 1);
    data = skb.data;
    } else {
// Otherwise need to copy the buffer
    skb_copy_bits(skb, 0, mbus.tx_scratch, skb.len);
    data = mbus.tx_scratch;
    }
// PEC calculation
    addr = mi.addr << 1;
    pec = i2c_smbus_pec(0, &addr, 1);
    pec = i2c_smbus_pec(pec, data, data_len);
    data[data_len] = pec;
    xfer.data.out = data;
    rc = i3c_device_do_xfers(mi.i3c, &xfer, 1, I3C_SDR);
    if (rc == 0) {
    stats.tx_bytes += data_len;
    stats.tx_packets++;
    } else {
    stats.tx_errors++;
    }
    out:
    if (mi)
    mutex_unlock(&mi.lock);
    }
#[no_mangle]
unsafe extern "C" fn mctp_i3c_tx_thread(data: *mut c_void) -> c_int {
    static int mctp_i3c_tx_thread(void *data)
    {
    struct mctp_i3c_bus *mbus = data;
    struct sk_buff *skb;
    for (;;) {
    if (kthread_should_stop())
    break;
    spin_lock_bh(&mbus.tx_lock);
    skb = mbus.tx_skb;
    mbus.tx_skb = core::ptr::null_mut();
    spin_unlock_bh(&mbus.tx_lock);
    if (netif_queue_stopped(mbus.ndev))
    netif_wake_queue(mbus.ndev);
    if (skb) {
    mctp_i3c_xmit(mbus, skb);
    kfree_skb(skb);
    } else {
    wait_event_idle(mbus.tx_wq,
    mbus.tx_skb || kthread_should_stop());
    }
    }
    return 0;
    }
    static netdev_tx_t mctp_i3c_start_xmit(struct sk_buff *skb,
    struct net_device *ndev)
    {
    struct mctp_i3c_bus *mbus = netdev_priv(ndev);
    netdev_tx_t ret;
    spin_lock(&mbus.tx_lock);
    netif_stop_queue(ndev);
    if (mbus.tx_skb) {
    dev_warn_ratelimited(&ndev.dev, "TX with queue stopped");
    ret = NETDEV_TX_BUSY;
    } else {
    mbus.tx_skb = skb;
    ret = NETDEV_TX_OK;
    }
    spin_unlock(&mbus.tx_lock);
    if (ret == NETDEV_TX_OK)
    wake_up(&mbus.tx_wq);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mctp_i3c_bus_free(mbus: *mut mctp_i3c_bus) {
    static void mctp_i3c_bus_free(struct mctp_i3c_bus *mbus)
    __must_hold(&busdevs_lock)
    {
    struct mctp_i3c_device *mi = core::ptr::null_mut(), *tmp = core::ptr::null_mut();
    if (mbus.tx_thread) {
    kthread_stop(mbus.tx_thread);
    mbus.tx_thread = core::ptr::null_mut();
    }
// Remove any child devices
    list_for_each_entry_safe(mi, tmp, &mbus.devs, list) {
    mctp_i3c_remove_device(mi);
    }
    kfree_skb(mbus.tx_skb);
    list_del(&mbus.list);
    }
#[no_mangle]
unsafe extern "C" fn mctp_i3c_ndo_uninit(ndev: *mut net_device) {
    static void mctp_i3c_ndo_uninit(struct net_device *ndev)
    {
    struct mctp_i3c_bus *mbus = netdev_priv(ndev);
// Perform cleanup here to ensure there are no remaining references
    mctp_i3c_bus_free(mbus);
    }
    static int mctp_i3c_header_create(struct sk_buff *skb, struct net_device *dev,
    unsigned short type, const void *daddr,
    const void *saddr, unsigned int len)
    {
    struct mctp_i3c_internal_hdr *ihdr;
    int rc;
    if (!daddr || !saddr)
    return -EINVAL;
    rc = skb_cow_head(skb, sizeof(struct mctp_i3c_internal_hdr));
    if (rc)
    return rc;
    skb_push(skb, sizeof(struct mctp_i3c_internal_hdr));
    skb_reset_mac_header(skb);
    ihdr = (void *)skb_mac_header(skb);
    memcpy(ihdr.dest, daddr, PID_SIZE);
    memcpy(ihdr.source, saddr, PID_SIZE);
    return 0;
    }
    static const struct net_device_ops mctp_i3c_ops = {
    .ndo_start_xmit = mctp_i3c_start_xmit,
    .ndo_uninit = mctp_i3c_ndo_uninit,
    };
    static const struct header_ops mctp_i3c_headops = {
    .create = mctp_i3c_header_create,
    };
#[no_mangle]
unsafe extern "C" fn mctp_i3c_net_setup(dev: *mut net_device) {
    static void mctp_i3c_net_setup(struct net_device *dev)
    {
    dev.type = ARPHRD_MCTP;
    dev.mtu = MCTP_I3C_MAXMTU;
    dev.min_mtu = MCTP_I3C_MINMTU;
    dev.max_mtu = MCTP_I3C_MAXMTU;
    dev.tx_queue_len = MCTP_I3C_TX_QUEUE_LEN;
    dev.hard_header_len = sizeof(struct mctp_i3c_internal_hdr);
    dev.addr_len = PID_SIZE;
    dev.netdev_ops	= &mctp_i3c_ops;
    dev.header_ops	= &mctp_i3c_headops;
    }
#[no_mangle]
unsafe extern "C" fn mctp_i3c_is_mctp_controller(bus: *mut i3c_bus) -> bool {
    static bool mctp_i3c_is_mctp_controller(struct i3c_bus *bus)
    {
    struct i3c_dev_desc *master = bus.cur_master;
    if (!master)
    return false;
    return of_property_read_bool(master.common.master.dev.of_node,
    MCTP_I3C_OF_PROP);
    }
// Returns the Provisioned Id of a local bus master
#[no_mangle]
unsafe extern "C" fn mctp_i3c_bus_local_pid(bus: *mut i3c_bus, ret_pid: *mut u64) -> c_int {
    static int mctp_i3c_bus_local_pid(struct i3c_bus *bus, u64 *ret_pid)
    {
    struct i3c_dev_desc *master;
    master = bus.cur_master;
    if (WARN_ON_ONCE(!master))
    return -ENOENT;
// ret_pid = master->info.pid;
    return 0;
    }
// Returns an ERR_PTR on failure
    static struct mctp_i3c_bus *mctp_i3c_bus_add(struct i3c_bus *bus)
    __must_hold(&busdevs_lock)
    {
    struct mctp_i3c_bus *mbus = core::ptr::null_mut();
    struct net_device *ndev = core::ptr::null_mut();
    char namebuf[IFNAMSIZ];
    u8 addr[PID_SIZE];
    int rc;
    if (!mctp_i3c_is_mctp_controller(bus))
    return ERR_PTR(-ENOENT);
    snprintf(namebuf, sizeof(namebuf), "mctpi3c%d", bus.id);
    ndev = alloc_netdev(sizeof(*mbus), namebuf, NET_NAME_ENUM,
    mctp_i3c_net_setup);
    if (!ndev) {
    rc = -ENOMEM;
    goto err;
    }
    mbus = netdev_priv(ndev);
    mbus.ndev = ndev;
    mbus.bus = bus;
    INIT_LIST_HEAD(&mbus.devs);
    list_add(&mbus.list, &busdevs);
    rc = mctp_i3c_bus_local_pid(bus, &mbus.pid);
    if (rc < 0) {
    dev_err(&ndev.dev, "No I3C PID available\n");
    goto err_free_uninit;
    }
    put_unaligned_be48(mbus.pid, addr);
    dev_addr_set(ndev, addr);
    init_waitqueue_head(&mbus.tx_wq);
    spin_lock_init(&mbus.tx_lock);
    mbus.tx_thread = kthread_run(mctp_i3c_tx_thread, mbus,
    "%s/tx", ndev.name);
    if (IS_ERR(mbus.tx_thread)) {
    dev_warn(&ndev.dev, "Error creating thread: %pe\n",
    mbus.tx_thread);
    rc = PTR_ERR(mbus.tx_thread);
    mbus.tx_thread = core::ptr::null_mut();
    goto err_free_uninit;
    }
    rc = mctp_register_netdev(ndev, core::ptr::null_mut(), MCTP_PHYS_BINDING_I3C);
    if (rc < 0) {
    dev_warn(&ndev.dev, "netdev register failed: %d\n", rc);
    goto err_free_netdev;
    }
    return mbus;
    err_free_uninit:
// uninit will not get called if a netdev has not been registered,
// so we perform the same mbus cleanup manually.
//
    mctp_i3c_bus_free(mbus);
    err_free_netdev:
    free_netdev(ndev);
    err:
    return ERR_PTR(rc);
    }
#[no_mangle]
unsafe extern "C" fn mctp_i3c_bus_remove(mbus: *mut mctp_i3c_bus) {
    static void mctp_i3c_bus_remove(struct mctp_i3c_bus *mbus)
    __must_hold(&busdevs_lock)
    {
// Unregister calls through to ndo_uninit -> mctp_i3c_bus_free()
    mctp_unregister_netdev(mbus.ndev);
    free_netdev(mbus.ndev);
// mbus is deallocated
    }
// Removes all mctp-i3c busses
#[no_mangle]
unsafe extern "C" fn mctp_i3c_bus_remove_all() {
    static void mctp_i3c_bus_remove_all(void)
    {
    struct mctp_i3c_bus *mbus = core::ptr::null_mut(), *tmp = core::ptr::null_mut();
    mutex_lock(&busdevs_lock);
    list_for_each_entry_safe(mbus, tmp, &busdevs, list) {
    mctp_i3c_bus_remove(mbus);
    }
    mutex_unlock(&busdevs_lock);
    }
// Adds a i3c_bus if it isn't already in the busdevs list.
// Suitable as an i3c_for_each_bus_locked callback.
//
#[no_mangle]
unsafe extern "C" fn mctp_i3c_bus_add_new(bus: *mut i3c_bus, data: *mut c_void) -> c_int {
    static int mctp_i3c_bus_add_new(struct i3c_bus *bus, void *data)
    {
    struct mctp_i3c_bus *mbus = core::ptr::null_mut(), *tmp = core::ptr::null_mut();
    let mut exists: bool = false;
    mutex_lock(&busdevs_lock);
    list_for_each_entry_safe(mbus, tmp, &busdevs, list)
    if (mbus.bus == bus)
    exists = true;
// It is OK for a bus to already exist. That can occur due to
// the race in mod_init between notifier and for_each_bus
//
    if (!exists)
    mctp_i3c_bus_add(bus);
    mutex_unlock(&busdevs_lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mctp_i3c_notify_bus_remove(bus: *mut i3c_bus) {
    static void mctp_i3c_notify_bus_remove(struct i3c_bus *bus)
    {
    struct mctp_i3c_bus *mbus = core::ptr::null_mut(), *tmp;
    mutex_lock(&busdevs_lock);
    list_for_each_entry_safe(mbus, tmp, &busdevs, list)
    if (mbus.bus == bus)
    mctp_i3c_bus_remove(mbus);
    mutex_unlock(&busdevs_lock);
    }
    static int mctp_i3c_notifier_call(struct notifier_block *nb,
    unsigned long action, void *data)
    {
    switch (action) {
    case I3C_NOTIFY_BUS_ADD:
    mctp_i3c_bus_add_new((struct i3c_bus *)data, core::ptr::null_mut());
    break;
    case I3C_NOTIFY_BUS_REMOVE:
    mctp_i3c_notify_bus_remove((struct i3c_bus *)data);
    break;
    }
    return NOTIFY_DONE;
    }
    static struct notifier_block mctp_i3c_notifier = {
    .notifier_call = mctp_i3c_notifier_call,
    };
    static const struct i3c_device_id mctp_i3c_ids[] = {
    I3C_CLASS(I3C_DCR_MCTP, core::ptr::null_mut()),
    { 0 },
    };
    static struct i3c_driver mctp_i3c_driver = {
    .driver = {
    .name = "mctp-i3c",
    },
    .probe = mctp_i3c_probe,
    .remove = mctp_i3c_remove,
    .id_table = mctp_i3c_ids,
    };
#[no_mangle]
unsafe extern "C" fn mctp_i3c_mod_init() -> __init int {
    static __init int mctp_i3c_mod_init(void)
    {
    int rc;
    rc = i3c_register_notifier(&mctp_i3c_notifier);
    if (rc < 0)
    return rc;
    i3c_for_each_bus_locked(mctp_i3c_bus_add_new, core::ptr::null_mut());
    rc = i3c_driver_register(&mctp_i3c_driver);
    if (rc < 0)
    goto err_unregister_notifier;
    return 0;
    err_unregister_notifier:
    i3c_unregister_notifier(&mctp_i3c_notifier);
    mctp_i3c_bus_remove_all();
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn mctp_i3c_mod_exit() -> __exit void {
    static __exit void mctp_i3c_mod_exit(void)
    {
    int rc;
    i3c_driver_unregister(&mctp_i3c_driver);
    rc = i3c_unregister_notifier(&mctp_i3c_notifier);
    if (rc < 0)
    pr_warn("MCTP I3C could not unregister notifier, %d\n", rc);
    mctp_i3c_bus_remove_all();
    }
    module_init(mctp_i3c_mod_init);
    module_exit(mctp_i3c_mod_exit);
    MODULE_DEVICE_TABLE(i3c, mctp_i3c_ids);
    MODULE_DESCRIPTION("MCTP I3C device");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Matt Johnston <matt@codeconstruct.com.au>");
