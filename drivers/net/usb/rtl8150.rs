//! Automatically rewritten from C to Rust
//! Source: drivers/net/usb/rtl8150.c
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
// Copyright (c) 2002 Petko Manolov (petkan@users.sourceforge.net)
//

pub const IDR: c_uint = 0x0120;
pub const MAR: c_uint = 0x0126;
pub const CR: c_uint = 0x012e;
pub const TCR: c_uint = 0x012f;
pub const RCR: c_uint = 0x0130;
pub const TSR: c_uint = 0x0132;
pub const RSR: c_uint = 0x0133;
pub const CON0: c_uint = 0x0135;
pub const CON1: c_uint = 0x0136;
pub const MSR: c_uint = 0x0137;
pub const PHYADD: c_uint = 0x0138;
pub const PHYDAT: c_uint = 0x0139;
pub const PHYCNT: c_uint = 0x013b;
pub const GPPC: c_uint = 0x013d;
pub const BMCR: c_uint = 0x0140;
pub const BMSR: c_uint = 0x0142;
pub const ANAR: c_uint = 0x0144;
pub const ANLP: c_uint = 0x0146;
pub const AER: c_uint = 0x0148;
pub const CSCR: c_uint = 0x014C  /* This one has the link status */;

pub const IDR_EEPROM: c_uint = 0x1202;
pub const PHY_READ: c_int = 0;
pub const PHY_WRITE: c_uint = 0x20;
pub const PHY_GO: c_uint = 0x40;
pub const MII_TIMEOUT: c_int = 10;
pub const INTBUFSIZE: c_int = 8;
pub const RTL8150_REQT_READ: c_uint = 0xc0;
pub const RTL8150_REQT_WRITE: c_uint = 0x40;
pub const RTL8150_REQ_GET_REGS: c_uint = 0x05;
pub const RTL8150_REQ_SET_REGS: c_uint = 0x05;
// Transmit status register errors

// Receive status register errors

// Media status register definitions

// USB endpoints
    enum rtl8150_usb_ep {
    RTL8150_USB_EP_CONTROL = 0,
    RTL8150_USB_EP_BULK_IN = 1,
    RTL8150_USB_EP_BULK_OUT = 2,
    RTL8150_USB_EP_INT_IN = 3,
    };
// Interrupt pipe data
pub const INT_TSR: c_uint = 0x00;
pub const INT_RSR: c_uint = 0x01;
pub const INT_MSR: c_uint = 0x02;
pub const INT_WAKSR: c_uint = 0x03;
pub const INT_TXOK_CNT: c_uint = 0x04;
pub const INT_RXLOST_CNT: c_uint = 0x05;
pub const INT_CRERR_CNT: c_uint = 0x06;
pub const INT_COL_CNT: c_uint = 0x07;
pub const RTL8150_MTU: c_int = 1540;

pub const RX_SKB_POOL_SIZE: c_int = 4;
// rtl8150 flags
pub const RTL8150_HW_CRC: c_int = 0;
pub const RX_REG_SET: c_int = 1;
pub const RTL8150_UNPLUG: c_int = 2;
pub const RX_URB_FAIL: c_int = 3;
// Define these values to match your device
pub const VENDOR_ID_REALTEK: c_uint = 0x0bda;
pub const VENDOR_ID_MELCO: c_uint = 0x0411;
pub const VENDOR_ID_MICRONET: c_uint = 0x3980;
pub const VENDOR_ID_LONGSHINE: c_uint = 0x07b8;
pub const VENDOR_ID_OQO: c_uint = 0x1557;
pub const VENDOR_ID_ZYXEL: c_uint = 0x0586;
pub const PRODUCT_ID_RTL8150: c_uint = 0x8150;
pub const PRODUCT_ID_LUAKTX: c_uint = 0x0012;
pub const PRODUCT_ID_LCS8138TX: c_uint = 0x401a;
pub const PRODUCT_ID_SP128AR: c_uint = 0x0003;
pub const PRODUCT_ID_PRESTIGE: c_uint = 0x401a;

// table of devices that work with this driver
    static const struct usb_device_id rtl8150_table[] = {
    {USB_DEVICE(VENDOR_ID_REALTEK, PRODUCT_ID_RTL8150)},
    {USB_DEVICE(VENDOR_ID_MELCO, PRODUCT_ID_LUAKTX)},
    {USB_DEVICE(VENDOR_ID_MICRONET, PRODUCT_ID_SP128AR)},
    {USB_DEVICE(VENDOR_ID_LONGSHINE, PRODUCT_ID_LCS8138TX)},
    {USB_DEVICE(VENDOR_ID_OQO, PRODUCT_ID_RTL8150)},
    {USB_DEVICE(VENDOR_ID_ZYXEL, PRODUCT_ID_PRESTIGE)},
    {}
    };
    MODULE_DEVICE_TABLE(usb, rtl8150_table);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8150 {
    pub flags: c_ulong,
    pub udev: *mut usb_device,
    pub tl: tasklet_struct,
    pub netdev: *mut net_device,
    pub intr_urb: *mut *mut *mut urb rx_urb, tx_urb,,
    pub rx_skb: *mut *mut sk_buff tx_skb,,
    pub rx_skb_pool: [*mut sk_buff; RX_SKB_POOL_SIZE],
    pub rx_pool_lock: spinlock_t,
    pub dr: usb_ctrlrequest,
    pub intr_interval: c_int,
    pub intr_buff: *mut u8,
    pub phy: u8,
}

    typedef struct rtl8150 rtl8150_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct async_req {
    pub dr: usb_ctrlrequest,
    pub rx_creg: u16,
}

    static const char driver_name [] = "rtl8150";
//
// device related part of the code
//
#[no_mangle]
unsafe extern "C" fn get_registers(dev: *mut *mut rtl8150_t, indx: u16, size: u16, data: *mut c_void) -> c_int {
    static int get_registers(rtl8150_t * dev, u16 indx, u16 size, void *data)
    {
    return usb_control_msg_recv(dev.udev, 0, RTL8150_REQ_GET_REGS,
    RTL8150_REQT_READ, indx, 0, data, size,
    1000, GFP_NOIO);
    }
#[no_mangle]
unsafe extern "C" fn set_registers(dev: *mut *mut rtl8150_t, indx: u16, size: u16, data: *const c_void) -> c_int {
    static int set_registers(rtl8150_t * dev, u16 indx, u16 size, const void *data)
    {
    return usb_control_msg_send(dev.udev, 0, RTL8150_REQ_SET_REGS,
    RTL8150_REQT_WRITE, indx, 0, data, size,
    1000, GFP_NOIO);
    }
#[no_mangle]
unsafe extern "C" fn async_set_reg_cb(urb: *mut urb) {
    static void async_set_reg_cb(struct urb *urb)
    {
    struct async_req *req = (struct async_req *)urb.context;
    let mut status: c_int = urb.status;
    if (status < 0)
    dev_dbg(&urb.dev.dev, "%s failed with %d", __func__, status);
    kfree(req);
    usb_free_urb(urb);
    }
#[no_mangle]
unsafe extern "C" fn async_set_registers(dev: *mut rtl8150_t, indx: u16, size: u16, reg: u16) -> c_int {
    static int async_set_registers(rtl8150_t *dev, u16 indx, u16 size, u16 reg)
    {
    let mut res: c_int = -ENOMEM;
    struct urb *async_urb;
    struct async_req *req;
    req = kmalloc_obj(struct async_req, GFP_ATOMIC);
    if (req == core::ptr::null_mut())
    return res;
    async_urb = usb_alloc_urb(0, GFP_ATOMIC);
    if (async_urb == core::ptr::null_mut()) {
    kfree(req);
    return res;
    }
    req.rx_creg = cpu_to_le16(reg);
    req.dr.bRequestType = RTL8150_REQT_WRITE;
    req.dr.bRequest = RTL8150_REQ_SET_REGS;
    req.dr.wIndex = 0;
    req.dr.wValue = cpu_to_le16(indx);
    req.dr.wLength = cpu_to_le16(size);
    usb_fill_control_urb(async_urb, dev.udev,
    usb_sndctrlpipe(dev.udev, 0), (void *)&req.dr,
    &req.rx_creg, size, async_set_reg_cb, req);
    res = usb_submit_urb(async_urb, GFP_ATOMIC);
    if (res) {
    if (res == -ENODEV)
    netif_device_detach(dev.netdev);
    dev_err(&dev.udev.dev, "%s failed with %d\n", __func__, res);
    kfree(req);
    usb_free_urb(async_urb);
    }
    return res;
    }
#[no_mangle]
unsafe extern "C" fn read_mii_word(dev: *mut *mut rtl8150_t, phy: u8, indx: __u8, reg: *mut *mut u16) -> c_int {
    static int read_mii_word(rtl8150_t * dev, u8 phy, __u8 indx, u16 * reg)
    {
    int i;
    u8 data[3], tmp;
    data[0] = phy;
    data[1] = data[2] = 0;
    tmp = indx | PHY_READ | PHY_GO;
    i = 0;
    set_registers(dev, PHYADD, sizeof(data), data);
    set_registers(dev, PHYCNT, 1, &tmp);
    do {
    get_registers(dev, PHYCNT, 1, data);
    } while ((data[0] & PHY_GO) && (i++ < MII_TIMEOUT));
    if (i <= MII_TIMEOUT) {
    get_registers(dev, PHYDAT, 2, data);
// reg = data[0] | (data[1] << 8);
    return 0;
    } else
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn write_mii_word(dev: *mut *mut rtl8150_t, phy: u8, indx: __u8, reg: u16) -> c_int {
    static int write_mii_word(rtl8150_t * dev, u8 phy, __u8 indx, u16 reg)
    {
    int i;
    u8 data[3], tmp;
    data[0] = phy;
    data[1] = reg & 0xff;
    data[2] = (reg >> 8) & 0xff;
    tmp = indx | PHY_WRITE | PHY_GO;
    i = 0;
    set_registers(dev, PHYADD, sizeof(data), data);
    set_registers(dev, PHYCNT, 1, &tmp);
    do {
    get_registers(dev, PHYCNT, 1, data);
    } while ((data[0] & PHY_GO) && (i++ < MII_TIMEOUT));
    if (i <= MII_TIMEOUT)
    return 0;
    else
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn set_ethernet_addr(dev: *mut rtl8150_t) {
    static void set_ethernet_addr(rtl8150_t *dev)
    {
    u8 node_id[ETH_ALEN];
    int ret;
    ret = get_registers(dev, IDR, sizeof(node_id), node_id);
    if (!ret) {
    eth_hw_addr_set(dev.netdev, node_id);
    } else {
    eth_hw_addr_random(dev.netdev);
    netdev_notice(dev.netdev, "Assigned a random MAC address: %pM\n",
    dev.netdev.dev_addr);
    }
    }
#[no_mangle]
unsafe extern "C" fn rtl8150_set_mac_address(netdev: *mut net_device, p: *mut c_void) -> c_int {
    static int rtl8150_set_mac_address(struct net_device *netdev, void *p)
    {
    struct sockaddr *addr = p;
    rtl8150_t *dev = netdev_priv(netdev);
    if (netif_running(netdev))
    return -EBUSY;
    eth_hw_addr_set(netdev, addr.sa_data);
    netdev_dbg(netdev, "Setting MAC address to %pM\n", netdev.dev_addr);
// Set the IDR registers.
    set_registers(dev, IDR, netdev.addr_len, netdev.dev_addr);

    {
    int i;
    u8 cr;
// Get the CR contents.
    get_registers(dev, CR, 1, &cr);
// Set the WEPROM bit (eeprom write enable).
    cr |= 0x20;
    set_registers(dev, CR, 1, &cr);
// Write the MAC address into eeprom. Eeprom writes must be word-sized,
    so we need to split them up. */
    for (i = 0; i * 2 < netdev.addr_len; i++) {
    set_registers(dev, IDR_EEPROM + (i * 2), 2,
    netdev.dev_addr + (i * 2));
    }
// Clear the WEPROM bit (preventing accidental eeprom writes).
    cr &= 0xdf;
    set_registers(dev, CR, 1, &cr);
    }

    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtl8150_reset(dev: *mut *mut rtl8150_t) -> c_int {
    static int rtl8150_reset(rtl8150_t * dev)
    {
    let mut data: u8 = 0x10;
    let mut i: c_int = HZ;
    set_registers(dev, CR, 1, &data);
    do {
    get_registers(dev, CR, 1, &data);
    } while ((data & 0x10) && --i);
    return (i > 0) ? 1 : 0;
    }
#[no_mangle]
unsafe extern "C" fn alloc_all_urbs(dev: *mut *mut rtl8150_t) -> c_int {
    static int alloc_all_urbs(rtl8150_t * dev)
    {
    dev.rx_urb = usb_alloc_urb(0, GFP_KERNEL);
    if (!dev.rx_urb)
    return 0;
    dev.tx_urb = usb_alloc_urb(0, GFP_KERNEL);
    if (!dev.tx_urb) {
    usb_free_urb(dev.rx_urb);
    return 0;
    }
    dev.intr_urb = usb_alloc_urb(0, GFP_KERNEL);
    if (!dev.intr_urb) {
    usb_free_urb(dev.rx_urb);
    usb_free_urb(dev.tx_urb);
    return 0;
    }
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn free_all_urbs(dev: *mut *mut rtl8150_t) {
    static void free_all_urbs(rtl8150_t * dev)
    {
    usb_free_urb(dev.rx_urb);
    usb_free_urb(dev.tx_urb);
    usb_free_urb(dev.intr_urb);
    }
#[no_mangle]
unsafe extern "C" fn unlink_all_urbs(dev: *mut *mut rtl8150_t) {
    static void unlink_all_urbs(rtl8150_t * dev)
    {
    usb_kill_urb(dev.rx_urb);
    usb_kill_urb(dev.tx_urb);
    usb_kill_urb(dev.intr_urb);
    }
    static inline struct sk_buff *pull_skb(rtl8150_t *dev)
    {
    struct sk_buff *skb;
    int i;
    for (i = 0; i < RX_SKB_POOL_SIZE; i++) {
    if (dev.rx_skb_pool[i]) {
    skb = dev.rx_skb_pool[i];
    dev.rx_skb_pool[i] = core::ptr::null_mut();
    return skb;
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn read_bulk_callback(urb: *mut urb) {
    static void read_bulk_callback(struct urb *urb)
    {
    rtl8150_t *dev;
    unsigned pkt_len, res;
    struct sk_buff *skb;
    struct net_device *netdev;
    let mut status: c_int = urb.status;
    int result;
    unsigned long flags;
    dev = urb.context;
    if (!dev)
    return;
    if (test_bit(RTL8150_UNPLUG, &dev.flags))
    return;
    netdev = dev.netdev;
    if (!netif_device_present(netdev))
    return;
    switch (status) {
    case 0:
    break;
    case -ENOENT:
    return;	/* the urb is in unlink state */
    case -ETIME:
    if (printk_ratelimit())
    dev_warn(&urb.dev.dev, "may be reset is needed?..\n");
    goto goon;
    default:
    if (printk_ratelimit())
    dev_warn(&urb.dev.dev, "Rx status %d\n", status);
    goto goon;
    }
    if (!dev.rx_skb)
    goto resched;
// protect against short packets (tell me why we got some?!?)
    if (urb.actual_length < 4)
    goto goon;
    res = urb.actual_length;
    pkt_len = res - 4;
    skb_put(dev.rx_skb, pkt_len);
    dev.rx_skb.protocol = eth_type_trans(dev.rx_skb, netdev);
    netif_rx(dev.rx_skb);
    netdev.stats.rx_packets++;
    netdev.stats.rx_bytes += pkt_len;
    spin_lock_irqsave(&dev.rx_pool_lock, flags);
    skb = pull_skb(dev);
    spin_unlock_irqrestore(&dev.rx_pool_lock, flags);
    if (!skb)
    goto resched;
    dev.rx_skb = skb;
    goon:
    usb_fill_bulk_urb(dev.rx_urb, dev.udev, usb_rcvbulkpipe(dev.udev, 1),
    dev.rx_skb.data, RTL8150_MTU, read_bulk_callback, dev);
    result = usb_submit_urb(dev.rx_urb, GFP_ATOMIC);
    if (result == -ENODEV)
    netif_device_detach(dev.netdev);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: result) -> else {
    set_bit(RX_URB_FAIL, &dev.flags);
    goto resched;
    } else {
    clear_bit(RX_URB_FAIL, &dev.flags);
    }
    return;
    resched:
    tasklet_schedule(&dev.tl);
    }
#[no_mangle]
unsafe extern "C" fn write_bulk_callback(urb: *mut urb) {
    static void write_bulk_callback(struct urb *urb)
    {
    rtl8150_t *dev;
    let mut status: c_int = urb.status;
    dev = urb.context;
    if (!dev)
    return;
    dev_kfree_skb_irq(dev.tx_skb);
    if (!netif_device_present(dev.netdev))
    return;
    if (status)
    dev_info(&urb.dev.dev, "%s: Tx status %d\n",
    dev.netdev.name, status);
    netif_trans_update(dev.netdev);
    netif_wake_queue(dev.netdev);
    }
#[no_mangle]
unsafe extern "C" fn intr_callback(urb: *mut urb) {
    static void intr_callback(struct urb *urb)
    {
    rtl8150_t *dev;
    __u8 *d;
    let mut status: c_int = urb.status;
    int res;
    dev = urb.context;
    if (!dev)
    return;
    switch (status) {
    case 0:			/* success */
    break;
    case -ECONNRESET:	/* unlink */
    case -ENOENT:
    case -ESHUTDOWN:
    return;
// -EPIPE:  should clear the halt
    default:
    dev_info(&urb.dev.dev, "%s: intr status %d\n",
    dev.netdev.name, status);
    goto resubmit;
    }
    d = urb.transfer_buffer;
    if (d[0] & TSR_ERRORS) {
    dev.netdev.stats.tx_errors++;
    if (d[INT_TSR] & (TSR_ECOL | TSR_JBR))
    dev.netdev.stats.tx_aborted_errors++;
    if (d[INT_TSR] & TSR_LCOL)
    dev.netdev.stats.tx_window_errors++;
    if (d[INT_TSR] & TSR_LOSS_CRS)
    dev.netdev.stats.tx_carrier_errors++;
    }
// Report link status changes to the network stack
    if ((d[INT_MSR] & MSR_LINK) == 0) {
    if (netif_carrier_ok(dev.netdev)) {
    netif_carrier_off(dev.netdev);
    netdev_dbg(dev.netdev, "%s: LINK LOST\n", __func__);
    }
    } else {
    if (!netif_carrier_ok(dev.netdev)) {
    netif_carrier_on(dev.netdev);
    netdev_dbg(dev.netdev, "%s: LINK CAME BACK\n", __func__);
    }
    }
    resubmit:
    res = usb_submit_urb (urb, GFP_ATOMIC);
    if (res == -ENODEV)
    netif_device_detach(dev.netdev);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: res) -> else {
    else if (res)
    dev_err(&dev.udev.dev,
    "can't resubmit intr, %s-%s/input0, status %d\n",
    dev.udev.bus.bus_name, dev.udev.devpath, res);
    }
#[no_mangle]
unsafe extern "C" fn rtl8150_suspend(intf: *mut usb_interface, message: pm_message_t) -> c_int {
    static int rtl8150_suspend(struct usb_interface *intf, pm_message_t message)
    {
    rtl8150_t *dev = usb_get_intfdata(intf);
    netif_device_detach(dev.netdev);
    if (netif_running(dev.netdev)) {
    usb_kill_urb(dev.rx_urb);
    usb_kill_urb(dev.intr_urb);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtl8150_resume(intf: *mut usb_interface) -> c_int {
    static int rtl8150_resume(struct usb_interface *intf)
    {
    rtl8150_t *dev = usb_get_intfdata(intf);
    netif_device_attach(dev.netdev);
    if (netif_running(dev.netdev)) {
    dev.rx_urb.status = 0;
    dev.rx_urb.actual_length = 0;
    read_bulk_callback(dev.rx_urb);
    dev.intr_urb.status = 0;
    dev.intr_urb.actual_length = 0;
    intr_callback(dev.intr_urb);
    }
    return 0;
    }
//
// network related part of the code
//
#[no_mangle]
unsafe extern "C" fn fill_skb_pool(dev: *mut rtl8150_t) {
    static void fill_skb_pool(rtl8150_t *dev)
    {
    struct sk_buff *skb;
    int i;
    for (i = 0; i < RX_SKB_POOL_SIZE; i++) {
    if (dev.rx_skb_pool[i])
    continue;
    skb = dev_alloc_skb(RTL8150_MTU + 2);
    if (!skb) {
    return;
    }
    skb_reserve(skb, 2);
    dev.rx_skb_pool[i] = skb;
    }
    }
#[no_mangle]
unsafe extern "C" fn free_skb_pool(dev: *mut rtl8150_t) {
    static void free_skb_pool(rtl8150_t *dev)
    {
    int i;
    for (i = 0; i < RX_SKB_POOL_SIZE; i++)
    dev_kfree_skb(dev.rx_skb_pool[i]);
    }
#[no_mangle]
unsafe extern "C" fn rx_fixup(t: *mut tasklet_struct) {
    static void rx_fixup(struct tasklet_struct *t)
    {
    struct rtl8150 *dev = from_tasklet(dev, t, tl);
    struct sk_buff *skb;
    int status;
    spin_lock_irq(&dev.rx_pool_lock);
    fill_skb_pool(dev);
    spin_unlock_irq(&dev.rx_pool_lock);
    if (test_bit(RX_URB_FAIL, &dev.flags))
    if (dev.rx_skb)
    goto try_again;
    spin_lock_irq(&dev.rx_pool_lock);
    skb = pull_skb(dev);
    spin_unlock_irq(&dev.rx_pool_lock);
    if (skb == core::ptr::null_mut())
    goto tlsched;
    dev.rx_skb = skb;
    usb_fill_bulk_urb(dev.rx_urb, dev.udev, usb_rcvbulkpipe(dev.udev, 1),
    dev.rx_skb.data, RTL8150_MTU, read_bulk_callback, dev);
    try_again:
    status = usb_submit_urb(dev.rx_urb, GFP_ATOMIC);
    if (status == -ENODEV) {
    netif_device_detach(dev.netdev);
    } else if (status) {
    set_bit(RX_URB_FAIL, &dev.flags);
    goto tlsched;
    } else {
    clear_bit(RX_URB_FAIL, &dev.flags);
    }
    return;
    tlsched:
    tasklet_schedule(&dev.tl);
    }
#[no_mangle]
unsafe extern "C" fn enable_net_traffic(dev: *mut *mut rtl8150_t) -> c_int {
    static int enable_net_traffic(rtl8150_t * dev)
    {
    u8 cr, tcr, rcr, msr;
    if (!rtl8150_reset(dev)) {
    dev_warn(&dev.udev.dev, "device reset failed\n");
    }
// RCR bit7=1 attach Rx info at the end;  =0 HW CRC (which is broken)
    rcr = 0x9e;
    tcr = 0xd8;
    cr = 0x0c;
    if (!(rcr & 0x80))
    set_bit(RTL8150_HW_CRC, &dev.flags);
    set_registers(dev, RCR, 1, &rcr);
    set_registers(dev, TCR, 1, &tcr);
    set_registers(dev, CR, 1, &cr);
    get_registers(dev, MSR, 1, &msr);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn disable_net_traffic(dev: *mut *mut rtl8150_t) {
    static void disable_net_traffic(rtl8150_t * dev)
    {
    u8 cr;
    get_registers(dev, CR, 1, &cr);
    cr &= 0xf3;
    set_registers(dev, CR, 1, &cr);
    }
#[no_mangle]
unsafe extern "C" fn rtl8150_tx_timeout(netdev: *mut net_device, txqueue: c_uint) {
    static void rtl8150_tx_timeout(struct net_device *netdev, unsigned int txqueue)
    {
    rtl8150_t *dev = netdev_priv(netdev);
    dev_warn(&netdev.dev, "Tx timeout.\n");
    usb_unlink_urb(dev.tx_urb);
    netdev.stats.tx_errors++;
    }
#[no_mangle]
unsafe extern "C" fn rtl8150_set_multicast(netdev: *mut net_device) {
    static void rtl8150_set_multicast(struct net_device *netdev)
    {
    rtl8150_t *dev = netdev_priv(netdev);
    let mut rx_creg: u16 = 0x9e;
    if (netdev.flags & IFF_PROMISC) {
    rx_creg |= 0x0001;
    dev_info(&netdev.dev, "%s: promiscuous mode\n", netdev.name);
    } else if (!netdev_mc_empty(netdev) ||
    (netdev.flags & IFF_ALLMULTI)) {
    rx_creg &= 0xfffe;
    rx_creg |= 0x0002;
    dev_dbg(&netdev.dev, "%s: allmulti set\n", netdev.name);
    } else {
// ~RX_MULTICAST, ~RX_PROMISCUOUS
    rx_creg &= 0x00fc;
    }
    async_set_registers(dev, RCR, sizeof(rx_creg), rx_creg);
    }
    static netdev_tx_t rtl8150_start_xmit(struct sk_buff *skb,
    struct net_device *netdev)
    {
    rtl8150_t *dev = netdev_priv(netdev);
    unsigned int skb_len;
    int count, res;
// pad the frame and ensure terminating USB packet, datasheet 9.2.3
    count = max(skb.len, ETH_ZLEN);
    if (count % 64 == 0)
    count++;
    if (skb_padto(skb, count)) {
    netdev.stats.tx_dropped++;
    return NETDEV_TX_OK;
    }
    skb_len = skb.len;
    netif_stop_queue(netdev);
    dev.tx_skb = skb;
    usb_fill_bulk_urb(dev.tx_urb, dev.udev, usb_sndbulkpipe(dev.udev, 2),
    skb.data, count, write_bulk_callback, dev);
    if ((res = usb_submit_urb(dev.tx_urb, GFP_ATOMIC))) {
// Can we get/handle EPIPE here?
    if (res == -ENODEV)
    netif_device_detach(dev.netdev);
    else {
    dev_warn(&netdev.dev, "failed tx_urb %d\n", res);
    netdev.stats.tx_errors++;
    netif_start_queue(netdev);
    }
//
// The URB was not submitted, so write_bulk_callback() will
// never run to free dev->tx_skb.  Drop the skb here and
// clear tx_skb to avoid leaving a stale pointer.
//
    dev.tx_skb = core::ptr::null_mut();
    dev_kfree_skb_any(skb);
    } else {
    netdev.stats.tx_packets++;
    netdev.stats.tx_bytes += skb_len;
    netif_trans_update(netdev);
    }
    return NETDEV_TX_OK;
    }
#[no_mangle]
unsafe extern "C" fn set_carrier(netdev: *mut net_device) {
    static void set_carrier(struct net_device *netdev)
    {
    rtl8150_t *dev = netdev_priv(netdev);
    short tmp;
    if (get_registers(dev, CSCR, 2, &tmp))
    return;
    if (tmp & CSCR_LINK_STATUS)
    netif_carrier_on(netdev);
    else
    netif_carrier_off(netdev);
    }
#[no_mangle]
unsafe extern "C" fn rtl8150_open(netdev: *mut net_device) -> c_int {
    static int rtl8150_open(struct net_device *netdev)
    {
    rtl8150_t *dev = netdev_priv(netdev);
    int res;
    if (dev.rx_skb == core::ptr::null_mut())
    dev.rx_skb = pull_skb(dev);
    if (!dev.rx_skb)
    return -ENOMEM;
    set_registers(dev, IDR, 6, netdev.dev_addr);
    usb_fill_bulk_urb(dev.rx_urb, dev.udev, usb_rcvbulkpipe(dev.udev, 1),
    dev.rx_skb.data, RTL8150_MTU, read_bulk_callback, dev);
    if ((res = usb_submit_urb(dev.rx_urb, GFP_KERNEL))) {
    if (res == -ENODEV)
    netif_device_detach(dev.netdev);
    dev_warn(&netdev.dev, "rx_urb submit failed: %d\n", res);
    return res;
    }
    usb_fill_int_urb(dev.intr_urb, dev.udev, usb_rcvintpipe(dev.udev, 3),
    dev.intr_buff, INTBUFSIZE, intr_callback,
    dev, dev.intr_interval);
    if ((res = usb_submit_urb(dev.intr_urb, GFP_KERNEL))) {
    if (res == -ENODEV)
    netif_device_detach(dev.netdev);
    dev_warn(&netdev.dev, "intr_urb submit failed: %d\n", res);
    usb_kill_urb(dev.rx_urb);
    return res;
    }
    enable_net_traffic(dev);
    set_carrier(netdev);
    netif_start_queue(netdev);
    return res;
    }
#[no_mangle]
unsafe extern "C" fn rtl8150_close(netdev: *mut net_device) -> c_int {
    static int rtl8150_close(struct net_device *netdev)
    {
    rtl8150_t *dev = netdev_priv(netdev);
    netif_stop_queue(netdev);
    if (!test_bit(RTL8150_UNPLUG, &dev.flags))
    disable_net_traffic(dev);
    unlink_all_urbs(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtl8150_get_drvinfo(netdev: *mut net_device, info: *mut ethtool_drvinfo) {
    static void rtl8150_get_drvinfo(struct net_device *netdev, struct ethtool_drvinfo *info)
    {
    rtl8150_t *dev = netdev_priv(netdev);
    strscpy(info.driver, driver_name, sizeof(info.driver));
    usb_make_path(dev.udev, info.bus_info, sizeof(info.bus_info));
    }
    static int rtl8150_get_link_ksettings(struct net_device *netdev,
    struct ethtool_link_ksettings *ecmd)
    {
    rtl8150_t *dev = netdev_priv(netdev);
    let mut lpa: c_short = 0;
    let mut bmcr: c_short = 0;
    u32 supported;
    supported = (SUPPORTED_10baseT_Half |
    SUPPORTED_10baseT_Full |
    SUPPORTED_100baseT_Half |
    SUPPORTED_100baseT_Full |
    SUPPORTED_Autoneg |
    SUPPORTED_TP | SUPPORTED_MII);
    ecmd.base.port = PORT_TP;
    ecmd.base.phy_address = dev.phy;
    get_registers(dev, BMCR, 2, &bmcr);
    get_registers(dev, ANLP, 2, &lpa);
    if (bmcr & BMCR_ANENABLE) {
    u32 speed = ((lpa & (LPA_100HALF | LPA_100FULL)) ?
    SPEED_100 : SPEED_10);
    ecmd.base.speed = speed;
    ecmd.base.autoneg = AUTONEG_ENABLE;
    if (speed == SPEED_100)
    ecmd.base.duplex = (lpa & LPA_100FULL) ?
    DUPLEX_FULL : DUPLEX_HALF;
    else
    ecmd.base.duplex = (lpa & LPA_10FULL) ?
    DUPLEX_FULL : DUPLEX_HALF;
    } else {
    ecmd.base.autoneg = AUTONEG_DISABLE;
    ecmd.base.speed = ((bmcr & BMCR_SPEED100) ?
    SPEED_100 : SPEED_10);
    ecmd.base.duplex = (bmcr & BMCR_FULLDPLX) ?
    DUPLEX_FULL : DUPLEX_HALF;
    }
    ethtool_convert_legacy_u32_to_link_mode(ecmd.link_modes.supported,
    supported);
    return 0;
    }
    static const struct ethtool_ops ops = {
    .get_drvinfo = rtl8150_get_drvinfo,
    .get_link = ethtool_op_get_link,
    .get_link_ksettings = rtl8150_get_link_ksettings,
    };
    static int rtl8150_siocdevprivate(struct net_device *netdev, struct ifreq *rq,
    void __user *udata, int cmd)
    {
    rtl8150_t *dev = netdev_priv(netdev);
    u16 *data = (u16 *) & rq.ifr_ifru;
    let mut res: c_int = 0;
    switch (cmd) {
    case SIOCDEVPRIVATE:
    data[0] = dev.phy;
    fallthrough;
    case SIOCDEVPRIVATE + 1:
    read_mii_word(dev, dev.phy, (data[1] & 0x1f), &data[3]);
    break;
    case SIOCDEVPRIVATE + 2:
    if (!capable(CAP_NET_ADMIN))
    return -EPERM;
    write_mii_word(dev, dev.phy, (data[1] & 0x1f), data[2]);
    break;
    default:
    res = -EOPNOTSUPP;
    }
    return res;
    }
    static const struct net_device_ops rtl8150_netdev_ops = {
    .ndo_open		= rtl8150_open,
    .ndo_stop		= rtl8150_close,
    .ndo_siocdevprivate	= rtl8150_siocdevprivate,
    .ndo_start_xmit		= rtl8150_start_xmit,
    .ndo_tx_timeout		= rtl8150_tx_timeout,
    .ndo_set_rx_mode	= rtl8150_set_multicast,
    .ndo_set_mac_address	= rtl8150_set_mac_address,
    .ndo_validate_addr	= eth_validate_addr,
    };
    static int rtl8150_probe(struct usb_interface *intf,
    const struct usb_device_id *id)
    {
    struct usb_device *udev = interface_to_usbdev(intf);
    rtl8150_t *dev;
    struct net_device *netdev;
    static const u8 bulk_ep_addr[] = {
    RTL8150_USB_EP_BULK_IN | USB_DIR_IN,
    RTL8150_USB_EP_BULK_OUT | USB_DIR_OUT,
    0};
    static const u8 int_ep_addr[] = {
    RTL8150_USB_EP_INT_IN | USB_DIR_IN,
    0};
    netdev = alloc_etherdev(sizeof(rtl8150_t));
    if (!netdev)
    return -ENOMEM;
    dev = netdev_priv(netdev);
    dev.intr_buff = kmalloc(INTBUFSIZE, GFP_KERNEL);
    if (!dev.intr_buff) {
    free_netdev(netdev);
    return -ENOMEM;
    }
// Verify that all required endpoints are present
    if (!usb_check_bulk_endpoints(intf, bulk_ep_addr) ||
    !usb_check_int_endpoints(intf, int_ep_addr)) {
    dev_err(&intf.dev, "couldn't find required endpoints\n");
    goto out;
    }
    tasklet_setup(&dev.tl, rx_fixup);
    spin_lock_init(&dev.rx_pool_lock);
    dev.udev = udev;
    dev.netdev = netdev;
    netdev.netdev_ops = &rtl8150_netdev_ops;
    netdev.watchdog_timeo = RTL8150_TX_TIMEOUT;
    netdev.ethtool_ops = &ops;
    dev.intr_interval = 100;	/* 100ms */
    if (!alloc_all_urbs(dev)) {
    dev_err(&intf.dev, "out of memory\n");
    goto out;
    }
    if (!rtl8150_reset(dev)) {
    dev_err(&intf.dev, "couldn't reset the device\n");
    goto out1;
    }
    fill_skb_pool(dev);
    set_ethernet_addr(dev);
    usb_set_intfdata(intf, dev);
    SET_NETDEV_DEV(netdev, &intf.dev);
    if (register_netdev(netdev) != 0) {
    dev_err(&intf.dev, "couldn't register the device\n");
    goto out2;
    }
    dev_info(&intf.dev, "%s: rtl8150 is detected\n", netdev.name);
    return 0;
    out2:
    usb_set_intfdata(intf, core::ptr::null_mut());
    free_skb_pool(dev);
    out1:
    free_all_urbs(dev);
    out:
    kfree(dev.intr_buff);
    free_netdev(netdev);
    return -EIO;
    }
#[no_mangle]
unsafe extern "C" fn rtl8150_disconnect(intf: *mut usb_interface) {
    static void rtl8150_disconnect(struct usb_interface *intf)
    {
    rtl8150_t *dev = usb_get_intfdata(intf);
    usb_set_intfdata(intf, core::ptr::null_mut());
    if (dev) {
    set_bit(RTL8150_UNPLUG, &dev.flags);
    tasklet_kill(&dev.tl);
    unregister_netdev(dev.netdev);
    unlink_all_urbs(dev);
    free_all_urbs(dev);
    free_skb_pool(dev);
    dev_kfree_skb(dev.rx_skb);
    kfree(dev.intr_buff);
    free_netdev(dev.netdev);
    }
    }
    static struct usb_driver rtl8150_driver = {
    .name		= driver_name,
    .probe		= rtl8150_probe,
    .disconnect	= rtl8150_disconnect,
    .id_table	= rtl8150_table,
    .suspend	= rtl8150_suspend,
    .resume		= rtl8150_resume,
    .disable_hub_initiated_lpm = 1,
    };
    module_usb_driver(rtl8150_driver);
    MODULE_AUTHOR(DRIVER_AUTHOR);
    MODULE_DESCRIPTION(DRIVER_DESC);
    MODULE_LICENSE("GPL");
