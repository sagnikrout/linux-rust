//! Automatically rewritten from C to Rust
//! Source: drivers/net/usb/ch9200.c
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


//
// USB 10M/100M ethernet adapter
//
// This file is licensed under the terms of the GNU General Public License
// version 2. This program is licensed "as is" without any warranty of any
// kind, whether express or implied
//

pub const CH9200_VID: c_uint = 0x1A86;
pub const CH9200_PID_E092: c_uint = 0xE092;
pub const CTRL_TIMEOUT_MS: c_int = 1000;
pub const CONTROL_TIMEOUT_MS: c_int = 1000;
pub const REQUEST_READ: c_uint = 0x0E;
pub const REQUEST_WRITE: c_uint = 0x0F;
// Address space:
// 00-63 : MII
// 64-128: MAC
//
// Note: all accesses must be 16-bit
//
pub const MAC_REG_CTRL: c_int = 64;
pub const MAC_REG_STATUS: c_int = 66;
pub const MAC_REG_INTERRUPT_MASK: c_int = 68;
pub const MAC_REG_PHY_COMMAND: c_int = 70;
pub const MAC_REG_PHY_DATA: c_int = 72;
pub const MAC_REG_STATION_L: c_int = 74;
pub const MAC_REG_STATION_M: c_int = 76;
pub const MAC_REG_STATION_H: c_int = 78;
pub const MAC_REG_HASH_L: c_int = 80;
pub const MAC_REG_HASH_M1: c_int = 82;
pub const MAC_REG_HASH_M2: c_int = 84;
pub const MAC_REG_HASH_H: c_int = 86;
pub const MAC_REG_THRESHOLD: c_int = 88;
pub const MAC_REG_FIFO_DEPTH: c_int = 90;
pub const MAC_REG_PAUSE: c_int = 92;
pub const MAC_REG_FLOW_CONTROL: c_int = 94;
// Control register bits
//
// Note: bits 13 and 15 are reserved
//

// Status register bits
//
// Note: bits 7-15 are reserved
//

// FIFO depth register bits
//
// Note: bits 6 and 14 are reserved
//

    static int control_read(struct usbnet *dev,
    unsigned char request, unsigned short value,
    unsigned short index, void *data, unsigned short size,
    int timeout)
    {
    unsigned char *buf = core::ptr::null_mut();
    unsigned char request_type;
    let mut err: c_int = 0;
    if (request == REQUEST_READ)
    request_type = (USB_DIR_IN | USB_TYPE_VENDOR | USB_RECIP_OTHER);
    else
    request_type = (USB_DIR_IN | USB_TYPE_VENDOR |
    USB_RECIP_DEVICE);
    netdev_dbg(dev.net, "%s() index=0x%02x size=%d\n",
    __func__, index, size);
    buf = kmalloc(size, GFP_KERNEL);
    if (!buf) {
    err = -ENOMEM;
    goto err_out;
    }
    err = usb_control_msg(dev.udev,
    usb_rcvctrlpipe(dev.udev, 0),
    request, request_type, value, index, buf, size,
    timeout);
    if (err == size)
    memcpy(data, buf, size);
#[no_mangle]
pub unsafe extern "C" fn if(0: err >=) -> else {
    else if (err >= 0)
    err = -EINVAL;
    kfree(buf);
    err_out:
    return err;
    }
    static int control_write(struct usbnet *dev, unsigned char request,
    unsigned short value, unsigned short index,
    void *data, unsigned short size, int timeout)
    {
    unsigned char *buf = core::ptr::null_mut();
    unsigned char request_type;
    let mut err: c_int = 0;
    if (request == REQUEST_WRITE)
    request_type = (USB_DIR_OUT | USB_TYPE_VENDOR |
    USB_RECIP_OTHER);
    else
    request_type = (USB_DIR_OUT | USB_TYPE_VENDOR |
    USB_RECIP_DEVICE);
    netdev_dbg(dev.net, "%s() index=0x%02x size=%d\n",
    __func__, index, size);
    if (data) {
    buf = kmemdup(data, size, GFP_KERNEL);
    if (!buf) {
    err = -ENOMEM;
    goto err_out;
    }
    }
    err = usb_control_msg(dev.udev,
    usb_sndctrlpipe(dev.udev, 0),
    request, request_type, value, index, buf, size,
    timeout);
    if (err >= 0 && err < size)
    err = -EINVAL;
    kfree(buf);
    return 0;
    err_out:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ch9200_mdio_read(netdev: *mut net_device, phy_id: c_int, loc: c_int) -> c_int {
    static int ch9200_mdio_read(struct net_device *netdev, int phy_id, int loc)
    {
    struct usbnet *dev = netdev_priv(netdev);
    unsigned char buff[2];
    int ret;
    netdev_dbg(netdev, "%s phy_id:%02x loc:%02x\n",
    __func__, phy_id, loc);
    if (phy_id != 0)
    return -ENODEV;
    ret = control_read(dev, REQUEST_READ, 0, loc * 2, buff, 0x02,
    CONTROL_TIMEOUT_MS);
    if (ret < 0)
    return ret;
    return (buff[0] | buff[1] << 8);
    }
    static void ch9200_mdio_write(struct net_device *netdev,
    int phy_id, int loc, int val)
    {
    struct usbnet *dev = netdev_priv(netdev);
    unsigned char buff[2];
    netdev_dbg(netdev, "%s() phy_id=%02x loc:%02x\n",
    __func__, phy_id, loc);
    if (phy_id != 0)
    return;
    buff[0] = (unsigned char)val;
    buff[1] = (unsigned char)(val >> 8);
    control_write(dev, REQUEST_WRITE, 0, loc * 2, buff, 0x02,
    CONTROL_TIMEOUT_MS);
    }
#[no_mangle]
unsafe extern "C" fn ch9200_link_reset(dev: *mut usbnet) -> c_int {
    static int ch9200_link_reset(struct usbnet *dev)
    {
    struct ethtool_cmd ecmd;
    mii_check_media(&dev.mii, 1, 1);
    mii_ethtool_gset(&dev.mii, &ecmd);
    netdev_dbg(dev.net, "%s() speed:%d duplex:%d\n",
    __func__, ecmd.speed, ecmd.duplex);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ch9200_status(dev: *mut usbnet, urb: *mut urb) {
    static void ch9200_status(struct usbnet *dev, struct urb *urb)
    {
    int link;
    unsigned char *buf;
    if (urb.actual_length < 16)
    return;
    buf = urb.transfer_buffer;
    link = !!(buf[0] & 0x01);
    if (link) {
    netif_carrier_on(dev.net);
    usbnet_defer_kevent(dev, EVENT_LINK_RESET);
    } else {
    netif_carrier_off(dev.net);
    }
    }
    static struct sk_buff *ch9200_tx_fixup(struct usbnet *dev, struct sk_buff *skb,
    gfp_t flags)
    {
    let mut i: c_int = 0;
    let mut len: c_int = 0;
    let mut tx_overhead: c_int = 0;
    tx_overhead = 0x40;
    len = skb.len;
    if (skb_cow_head(skb, tx_overhead)) {
    dev_kfree_skb_any(skb);
    return core::ptr::null_mut();
    }
    __skb_push(skb, tx_overhead);
// usbnet adds padding if length is a multiple of packet size
// if so, adjust length value in header
//
    if ((skb.len % dev.maxpacket) == 0)
    len++;
    skb.data[0] = len;
    skb.data[1] = len >> 8;
    skb.data[2] = 0x00;
    skb.data[3] = 0x80;
    for (i = 4; i < 48; i++)
    skb.data[i] = 0x00;
    skb.data[48] = len;
    skb.data[49] = len >> 8;
    skb.data[50] = 0x00;
    skb.data[51] = 0x80;
    for (i = 52; i < 64; i++)
    skb.data[i] = 0x00;
    return skb;
    }
#[no_mangle]
unsafe extern "C" fn ch9200_rx_fixup(dev: *mut usbnet, skb: *mut sk_buff) -> c_int {
    static int ch9200_rx_fixup(struct usbnet *dev, struct sk_buff *skb)
    {
    let mut len: c_int = 0;
    let mut rx_overhead: c_int = 0;
    rx_overhead = 64;
    if (unlikely(skb.len < rx_overhead)) {
    dev_err(&dev.udev.dev, "unexpected tiny rx frame\n");
    return 0;
    }
    len = (skb.data[skb.len - 16] | skb.data[skb.len - 15] << 8);
    skb_trim(skb, len);
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn get_mac_address(dev: *mut usbnet, data: *mut c_uchar) -> c_int {
    static int get_mac_address(struct usbnet *dev, unsigned char *data)
    {
    let mut err: c_int = 0;
    unsigned char mac_addr[0x06];
    let mut rd_mac_len: c_int = 0;
    netdev_dbg(dev.net, "%s:\n\tusbnet VID:%0x PID:%0x\n", __func__,
    le16_to_cpu(dev.udev.descriptor.idVendor),
    le16_to_cpu(dev.udev.descriptor.idProduct));
    memset(mac_addr, 0, sizeof(mac_addr));
    rd_mac_len = control_read(dev, REQUEST_READ, 0,
    MAC_REG_STATION_L, mac_addr, 0x02,
    CONTROL_TIMEOUT_MS);
    rd_mac_len += control_read(dev, REQUEST_READ, 0, MAC_REG_STATION_M,
    mac_addr + 2, 0x02, CONTROL_TIMEOUT_MS);
    rd_mac_len += control_read(dev, REQUEST_READ, 0, MAC_REG_STATION_H,
    mac_addr + 4, 0x02, CONTROL_TIMEOUT_MS);
    if (rd_mac_len != ETH_ALEN)
    err = -EINVAL;
    data[0] = mac_addr[5];
    data[1] = mac_addr[4];
    data[2] = mac_addr[3];
    data[3] = mac_addr[2];
    data[4] = mac_addr[1];
    data[5] = mac_addr[0];
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ch9200_bind(dev: *mut usbnet, intf: *mut usb_interface) -> c_int {
    static int ch9200_bind(struct usbnet *dev, struct usb_interface *intf)
    {
    let mut retval: c_int = 0;
    unsigned char data[2];
    u8 addr[ETH_ALEN];
    retval = usbnet_get_endpoints(dev, intf);
    if (retval)
    return retval;
    dev.mii.dev = dev.net;
    dev.mii.mdio_read = ch9200_mdio_read;
    dev.mii.mdio_write = ch9200_mdio_write;
    dev.mii.reg_num_mask = 0x1f;
    dev.mii.phy_id_mask = 0x1f;
    dev.hard_mtu = dev.net.mtu + dev.net.hard_header_len;
    dev.rx_urb_size = 24 * 64 + 16;
    mii_nway_restart(&dev.mii);
    data[0] = 0x01;
    data[1] = 0x0F;
    retval = control_write(dev, REQUEST_WRITE, 0, MAC_REG_THRESHOLD, data,
    0x02, CONTROL_TIMEOUT_MS);
    data[0] = 0xA0;
    data[1] = 0x90;
    retval = control_write(dev, REQUEST_WRITE, 0, MAC_REG_FIFO_DEPTH, data,
    0x02, CONTROL_TIMEOUT_MS);
    data[0] = 0x30;
    data[1] = 0x00;
    retval = control_write(dev, REQUEST_WRITE, 0, MAC_REG_PAUSE, data,
    0x02, CONTROL_TIMEOUT_MS);
    data[0] = 0x17;
    data[1] = 0xD8;
    retval = control_write(dev, REQUEST_WRITE, 0, MAC_REG_FLOW_CONTROL,
    data, 0x02, CONTROL_TIMEOUT_MS);
// Undocumented register
    data[0] = 0x01;
    data[1] = 0x00;
    retval = control_write(dev, REQUEST_WRITE, 0, 254, data, 0x02,
    CONTROL_TIMEOUT_MS);
    data[0] = 0x5F;
    data[1] = 0x0D;
    retval = control_write(dev, REQUEST_WRITE, 0, MAC_REG_CTRL, data, 0x02,
    CONTROL_TIMEOUT_MS);
    retval = get_mac_address(dev, addr);
    eth_hw_addr_set(dev.net, addr);
    return retval;
    }
    static const struct driver_info ch9200_info = {
    .description = "CH9200 USB to Network Adaptor",
    .flags = FLAG_ETHER,
    .bind = ch9200_bind,
    .rx_fixup = ch9200_rx_fixup,
    .tx_fixup = ch9200_tx_fixup,
    .status = ch9200_status,
    .link_reset = ch9200_link_reset,
    .reset = ch9200_link_reset,
    };
    static const struct usb_device_id ch9200_products[] = {
    {
    USB_DEVICE(0x1A86, 0xE092),
    .driver_info = (unsigned long)&ch9200_info,
    },
    {},
    };
    MODULE_DEVICE_TABLE(usb, ch9200_products);
    static struct usb_driver ch9200_driver = {
    .name = "ch9200",
    .id_table = ch9200_products,
    .probe = usbnet_probe,
    .disconnect = usbnet_disconnect,
    .suspend = usbnet_suspend,
    .resume = usbnet_resume,
    };
    module_usb_driver(ch9200_driver);
    MODULE_DESCRIPTION("QinHeng CH9200 USB Network device");
    MODULE_LICENSE("GPL");
