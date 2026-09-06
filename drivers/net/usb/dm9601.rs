//! Automatically rewritten from C to Rust
//! Source: drivers/net/usb/dm9601.c
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
// Davicom DM96xx USB 10/100Mbps ethernet devices
//
// Peter Korsgaard <jacmet@sunsite.dk>
//
// This file is licensed under the terms of the GNU General Public License
// version 2.  This program is licensed "as is" without any warranty of any
// kind, whether express or implied.
//
// #define DEBUG

// datasheet:
    http://ptm2.cc.utu.fi/ftp/network/cards/DM9601/From_NET/DM9601-DS-P01-930914.pdf
//
// control requests
pub const DM_READ_REGS: c_uint = 0x00;
pub const DM_WRITE_REGS: c_uint = 0x01;
pub const DM_READ_MEMS: c_uint = 0x02;
pub const DM_WRITE_REG: c_uint = 0x03;
pub const DM_WRITE_MEMS: c_uint = 0x05;
pub const DM_WRITE_MEM: c_uint = 0x07;
// registers
pub const DM_NET_CTRL: c_uint = 0x00;
pub const DM_RX_CTRL: c_uint = 0x05;
pub const DM_SHARED_CTRL: c_uint = 0x0b;
pub const DM_SHARED_ADDR: c_uint = 0x0c;
pub const DM_SHARED_DATA: c_uint = 0x0d	/* low + high */;
pub const DM_PHY_ADDR: c_uint = 0x10	/* 6 bytes */;
pub const DM_MCAST_ADDR: c_uint = 0x16	/* 8 bytes */;
pub const DM_GPR_CTRL: c_uint = 0x1e;
pub const DM_GPR_DATA: c_uint = 0x1f;
pub const DM_CHIP_ID: c_uint = 0x2c;
pub const DM_MODE_CTRL: c_uint = 0x91	/* only on dm9620 */;
// chip id values
pub const ID_DM9601: c_int = 0;
pub const ID_DM9620: c_int = 1;
pub const DM_MAX_MCAST: c_int = 64;
pub const DM_MCAST_SIZE: c_int = 8;
pub const DM_EEPROM_LEN: c_int = 256;

pub const DM_TIMEOUT: c_int = 1000;
#[no_mangle]
unsafe extern "C" fn dm_read(dev: *mut usbnet, reg: u8, length: u16, data: *mut c_void) -> c_int {
    static int dm_read(struct usbnet *dev, u8 reg, u16 length, void *data)
    {
    int err;
    err = usbnet_read_cmd(dev, DM_READ_REGS,
    USB_DIR_IN | USB_TYPE_VENDOR | USB_RECIP_DEVICE,
    0, reg, data, length);
    if(err != length && err >= 0)
    err = -EINVAL;
    return err;
    }
#[no_mangle]
unsafe extern "C" fn dm_read_reg(dev: *mut usbnet, reg: u8, value: *mut u8) -> c_int {
    static int dm_read_reg(struct usbnet *dev, u8 reg, u8 *value)
    {
    return dm_read(dev, reg, 1, value);
    }
#[no_mangle]
unsafe extern "C" fn dm_write(dev: *mut usbnet, reg: u8, length: u16, data: *mut c_void) -> c_int {
    static int dm_write(struct usbnet *dev, u8 reg, u16 length, void *data)
    {
    int err;
    err = usbnet_write_cmd(dev, DM_WRITE_REGS,
    USB_DIR_OUT | USB_TYPE_VENDOR | USB_RECIP_DEVICE,
    0, reg, data, length);
    if (err >= 0 && err < length)
    err = -EINVAL;
    return err;
    }
#[no_mangle]
unsafe extern "C" fn dm_write_reg(dev: *mut usbnet, reg: u8, value: u8) -> c_int {
    static int dm_write_reg(struct usbnet *dev, u8 reg, u8 value)
    {
    return usbnet_write_cmd(dev, DM_WRITE_REG,
    USB_DIR_OUT | USB_TYPE_VENDOR | USB_RECIP_DEVICE,
    value, reg, core::ptr::null_mut(), 0);
    }
    static void dm_write_async(struct usbnet *dev, u8 reg, u16 length,
    const void *data)
    {
    usbnet_write_cmd_async(dev, DM_WRITE_REGS,
    USB_DIR_OUT | USB_TYPE_VENDOR | USB_RECIP_DEVICE,
    0, reg, data, length);
    }
#[no_mangle]
unsafe extern "C" fn dm_write_reg_async(dev: *mut usbnet, reg: u8, value: u8) {
    static void dm_write_reg_async(struct usbnet *dev, u8 reg, u8 value)
    {
    usbnet_write_cmd_async(dev, DM_WRITE_REG,
    USB_DIR_OUT | USB_TYPE_VENDOR | USB_RECIP_DEVICE,
    value, reg, core::ptr::null_mut(), 0);
    }
#[no_mangle]
unsafe extern "C" fn dm_read_shared_word(dev: *mut usbnet, phy: c_int, reg: u8, value: *mut __le16) -> c_int {
    static int dm_read_shared_word(struct usbnet *dev, int phy, u8 reg, __le16 *value)
    {
    int ret, i;
    mutex_lock(&dev.phy_mutex);
    dm_write_reg(dev, DM_SHARED_ADDR, phy ? (reg | 0x40) : reg);
    dm_write_reg(dev, DM_SHARED_CTRL, phy ? 0xc : 0x4);
    for (i = 0; i < DM_TIMEOUT; i++) {
    let mut tmp: u8 = 0;
    udelay(1);
    ret = dm_read_reg(dev, DM_SHARED_CTRL, &tmp);
    if (ret < 0)
    goto out;
// ready
    if ((tmp & 1) == 0)
    break;
    }
    if (i == DM_TIMEOUT) {
    netdev_err(dev.net, "%s read timed out!\n", phy ? "phy" : "eeprom");
    ret = -EIO;
    goto out;
    }
    dm_write_reg(dev, DM_SHARED_CTRL, 0x0);
    ret = dm_read(dev, DM_SHARED_DATA, 2, value);
    netdev_dbg(dev.net, "read shared %d 0x%02x returned 0x%04x, %d\n",
    phy, reg, *value, ret);
    out:
    mutex_unlock(&dev.phy_mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dm_write_shared_word(dev: *mut usbnet, phy: c_int, reg: u8, value: __le16) -> c_int {
    static int dm_write_shared_word(struct usbnet *dev, int phy, u8 reg, __le16 value)
    {
    int ret, i;
    mutex_lock(&dev.phy_mutex);
    ret = dm_write(dev, DM_SHARED_DATA, 2, &value);
    if (ret < 0)
    goto out;
    dm_write_reg(dev, DM_SHARED_ADDR, phy ? (reg | 0x40) : reg);
    dm_write_reg(dev, DM_SHARED_CTRL, phy ? 0x1a : 0x12);
    for (i = 0; i < DM_TIMEOUT; i++) {
    let mut tmp: u8 = 0;
    udelay(1);
    ret = dm_read_reg(dev, DM_SHARED_CTRL, &tmp);
    if (ret < 0)
    goto out;
// ready
    if ((tmp & 1) == 0)
    break;
    }
    if (i == DM_TIMEOUT) {
    netdev_err(dev.net, "%s write timed out!\n", phy ? "phy" : "eeprom");
    ret = -EIO;
    goto out;
    }
    dm_write_reg(dev, DM_SHARED_CTRL, 0x0);
    out:
    mutex_unlock(&dev.phy_mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dm_read_eeprom_word(dev: *mut usbnet, offset: u8, value: *mut c_void) -> c_int {
    static int dm_read_eeprom_word(struct usbnet *dev, u8 offset, void *value)
    {
    return dm_read_shared_word(dev, 0, offset, value);
    }
#[no_mangle]
unsafe extern "C" fn dm9601_get_eeprom_len(dev: *mut net_device) -> c_int {
    static int dm9601_get_eeprom_len(struct net_device *dev)
    {
    return DM_EEPROM_LEN;
    }
    static int dm9601_get_eeprom(struct net_device *net,
    struct ethtool_eeprom *eeprom, u8 * data)
    {
    struct usbnet *dev = netdev_priv(net);
    __le16 *ebuf = (__le16 *) data;
    int i;
// access is 16bit
    if ((eeprom.offset % 2) || (eeprom.len % 2))
    return -EINVAL;
    for (i = 0; i < eeprom.len / 2; i++) {
    if (dm_read_eeprom_word(dev, eeprom.offset / 2 + i,
    &ebuf[i]) < 0)
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dm9601_mdio_read(netdev: *mut net_device, phy_id: c_int, loc: c_int) -> c_int {
    static int dm9601_mdio_read(struct net_device *netdev, int phy_id, int loc)
    {
    struct usbnet *dev = netdev_priv(netdev);
    __le16 res;
    int err;
    if (phy_id) {
    netdev_dbg(dev.net, "Only internal phy supported\n");
    return 0;
    }
    err = dm_read_shared_word(dev, 1, loc, &res);
    if (err < 0) {
    netdev_err(dev.net, "MDIO read error: %d\n", err);
    return 0;
    }
    netdev_dbg(dev.net,
    "dm9601_mdio_read() phy_id=0x%02x, loc=0x%02x, returns=0x%04x\n",
    phy_id, loc, le16_to_cpu(res));
    return le16_to_cpu(res);
    }
    static void dm9601_mdio_write(struct net_device *netdev, int phy_id, int loc,
    int val)
    {
    struct usbnet *dev = netdev_priv(netdev);
    let mut res: __le16 = cpu_to_le16(val);
    if (phy_id) {
    netdev_dbg(dev.net, "Only internal phy supported\n");
    return;
    }
    netdev_dbg(dev.net, "dm9601_mdio_write() phy_id=0x%02x, loc=0x%02x, val=0x%04x\n",
    phy_id, loc, val);
    dm_write_shared_word(dev, 1, loc, res);
    }
    static const struct ethtool_ops dm9601_ethtool_ops = {
    .get_drvinfo	= usbnet_get_drvinfo,
    .get_link	= usbnet_get_link,
    .get_msglevel	= usbnet_get_msglevel,
    .set_msglevel	= usbnet_set_msglevel,
    .get_eeprom_len	= dm9601_get_eeprom_len,
    .get_eeprom	= dm9601_get_eeprom,
    .nway_reset	= usbnet_nway_reset,
    .get_link_ksettings	= usbnet_get_link_ksettings_mii,
    .set_link_ksettings	= usbnet_set_link_ksettings_mii,
    };
#[no_mangle]
unsafe extern "C" fn dm9601_set_multicast(net: *mut net_device) {
    static void dm9601_set_multicast(struct net_device *net)
    {
    struct usbnet *dev = netdev_priv(net);
// We use the 20 byte dev->data for our 8 byte filter buffer
// to avoid allocating memory that is tricky to free later
    u8 *hashes = (u8 *) & dev.data;
    let mut rx_ctl: u8 = 0x31;
    memset(hashes, 0x00, DM_MCAST_SIZE);
    hashes[DM_MCAST_SIZE - 1] |= 0x80;	/* broadcast address */
    if (net.flags & IFF_PROMISC) {
    rx_ctl |= 0x02;
    } else if (net.flags & IFF_ALLMULTI ||
    netdev_mc_count(net) > DM_MAX_MCAST) {
    rx_ctl |= 0x08;
    } else if (!netdev_mc_empty(net)) {
    struct netdev_hw_addr *ha;
    netdev_for_each_mc_addr(ha, net) {
    let mut crc: u32 = ether_crc(ETH_ALEN, ha.addr) >> 26;
    hashes[crc >> 3] |= 1 << (crc & 0x7);
    }
    }
    dm_write_async(dev, DM_MCAST_ADDR, DM_MCAST_SIZE, hashes);
    dm_write_reg_async(dev, DM_RX_CTRL, rx_ctl);
    }
#[no_mangle]
unsafe extern "C" fn __dm9601_set_mac_address(dev: *mut usbnet) {
    static void __dm9601_set_mac_address(struct usbnet *dev)
    {
    dm_write_async(dev, DM_PHY_ADDR, ETH_ALEN, dev.net.dev_addr);
    }
#[no_mangle]
unsafe extern "C" fn dm9601_set_mac_address(net: *mut net_device, p: *mut c_void) -> c_int {
    static int dm9601_set_mac_address(struct net_device *net, void *p)
    {
    struct sockaddr *addr = p;
    struct usbnet *dev = netdev_priv(net);
    if (!is_valid_ether_addr(addr.sa_data)) {
    dev_err(&net.dev, "not setting invalid mac address %pM\n",
    addr.sa_data);
    return -EINVAL;
    }
    eth_hw_addr_set(net, addr.sa_data);
    __dm9601_set_mac_address(dev);
    return 0;
    }
    static const struct net_device_ops dm9601_netdev_ops = {
    .ndo_open		= usbnet_open,
    .ndo_stop		= usbnet_stop,
    .ndo_start_xmit		= usbnet_start_xmit,
    .ndo_tx_timeout		= usbnet_tx_timeout,
    .ndo_change_mtu		= usbnet_change_mtu,
    .ndo_get_stats64	= dev_get_tstats64,
    .ndo_validate_addr	= eth_validate_addr,
    .ndo_eth_ioctl		= usbnet_mii_ioctl,
    .ndo_set_rx_mode	= dm9601_set_multicast,
    .ndo_set_mac_address	= dm9601_set_mac_address,
    };
#[no_mangle]
unsafe extern "C" fn dm9601_bind(dev: *mut usbnet, intf: *mut usb_interface) -> c_int {
    static int dm9601_bind(struct usbnet *dev, struct usb_interface *intf)
    {
    int ret;
    u8 mac[ETH_ALEN], id;
    ret = usbnet_get_endpoints(dev, intf);
    if (ret)
    goto out;
    dev.net.netdev_ops = &dm9601_netdev_ops;
    dev.net.ethtool_ops = &dm9601_ethtool_ops;
    dev.net.hard_header_len += DM_TX_OVERHEAD;
    dev.hard_mtu = dev.net.mtu + dev.net.hard_header_len;
// dm9620/21a require room for 4 byte padding, even in dm9601
// mode, so we need +1 to be able to receive full size
// ethernet frames.
//
    dev.rx_urb_size = dev.net.mtu + ETH_HLEN + DM_RX_OVERHEAD + 1;
    dev.mii.dev = dev.net;
    dev.mii.mdio_read = dm9601_mdio_read;
    dev.mii.mdio_write = dm9601_mdio_write;
    dev.mii.phy_id_mask = 0x1f;
    dev.mii.reg_num_mask = 0x1f;
// reset
    dm_write_reg(dev, DM_NET_CTRL, 1);
    udelay(20);
// read MAC
    if (dm_read(dev, DM_PHY_ADDR, ETH_ALEN, mac) < 0) {
    printk(KERN_ERR "Error reading MAC address\n");
    ret = -ENODEV;
    goto out;
    }
//
// Overwrite the auto-generated address only with good ones.
//
    if (is_valid_ether_addr(mac))
    eth_hw_addr_set(dev.net, mac);
    else {
    printk(KERN_WARNING
    "dm9601: No valid MAC address in EEPROM, using %pM\n",
    dev.net.dev_addr);
    __dm9601_set_mac_address(dev);
    }
    if (dm_read_reg(dev, DM_CHIP_ID, &id) < 0) {
    netdev_err(dev.net, "Error reading chip ID\n");
    ret = -ENODEV;
    goto out;
    }
// put dm9620 devices in dm9601 mode
    if (id == ID_DM9620) {
    u8 mode;
    if (dm_read_reg(dev, DM_MODE_CTRL, &mode) < 0) {
    netdev_err(dev.net, "Error reading MODE_CTRL\n");
    ret = -ENODEV;
    goto out;
    }
    dm_write_reg(dev, DM_MODE_CTRL, mode & 0x7f);
    }
// power up phy
    dm_write_reg(dev, DM_GPR_CTRL, 1);
    dm_write_reg(dev, DM_GPR_DATA, 0);
// receive broadcast packets
    dm9601_set_multicast(dev.net);
    dm9601_mdio_write(dev.net, dev.mii.phy_id, MII_BMCR, BMCR_RESET);
    dm9601_mdio_write(dev.net, dev.mii.phy_id, MII_ADVERTISE,
    ADVERTISE_ALL | ADVERTISE_CSMA | ADVERTISE_PAUSE_CAP);
    mii_nway_restart(&dev.mii);
    out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dm9601_rx_fixup(dev: *mut usbnet, skb: *mut sk_buff) -> c_int {
    static int dm9601_rx_fixup(struct usbnet *dev, struct sk_buff *skb)
    {
    u8 status;
    int len;
// format:
    b1: rx status
    b2: packet length (incl crc) low
    b3: packet length (incl crc) high
    b4..n-4: packet data
    bn-3..bn: ethernet crc
//
    if (unlikely(skb.len < DM_RX_OVERHEAD)) {
    dev_err(&dev.udev.dev, "unexpected tiny rx frame\n");
    return 0;
    }
    status = skb.data[0];
    len = (skb.data[1] | (skb.data[2] << 8)) - 4;
    if (unlikely(status & 0xbf)) {
    if (status & 0x01) dev.net.stats.rx_fifo_errors++;
    if (status & 0x02) dev.net.stats.rx_crc_errors++;
    if (status & 0x04) dev.net.stats.rx_frame_errors++;
    if (status & 0x20) dev.net.stats.rx_missed_errors++;
    if (status & 0x90) dev.net.stats.rx_length_errors++;
    return 0;
    }
    skb_pull(skb, 3);
    skb_trim(skb, len);
    return 1;
    }
    static struct sk_buff *dm9601_tx_fixup(struct usbnet *dev, struct sk_buff *skb,
    gfp_t flags)
    {
    int len, pad;
// format:
    b1: packet length low
    b2: packet length high
    b3..n: packet data
//
    len = skb.len + DM_TX_OVERHEAD;
// workaround for dm962x errata with tx fifo getting out of
// sync if a USB bulk transfer retry happens right after a
// packet with odd / maxpacket length by adding up to 3 bytes
// padding.
//
    while ((len & 1) || !(len % dev.maxpacket))
    len++;
    len -= DM_TX_OVERHEAD; /* hw header doesn't count as part of length */
    pad = len - skb.len;
    if (skb_headroom(skb) < DM_TX_OVERHEAD || skb_tailroom(skb) < pad) {
    struct sk_buff *skb2;
    skb2 = skb_copy_expand(skb, DM_TX_OVERHEAD, pad, flags);
    dev_kfree_skb_any(skb);
    skb = skb2;
    if (!skb)
    return core::ptr::null_mut();
    }
    __skb_push(skb, DM_TX_OVERHEAD);
    if (pad) {
    memset(skb.data + skb.len, 0, pad);
    __skb_put(skb, pad);
    }
    skb.data[0] = len;
    skb.data[1] = len >> 8;
    return skb;
    }
#[no_mangle]
unsafe extern "C" fn dm9601_status(dev: *mut usbnet, urb: *mut urb) {
    static void dm9601_status(struct usbnet *dev, struct urb *urb)
    {
    int link;
    u8 *buf;
// format:
    b0: net status
    b1: tx status 1
    b2: tx status 2
    b3: rx status
    b4: rx overflow
    b5: rx count
    b6: tx count
    b7: gpr
//
    if (urb.actual_length < 8)
    return;
    buf = urb.transfer_buffer;
    link = !!(buf[0] & 0x40);
    if (netif_carrier_ok(dev.net) != link) {
    usbnet_link_change(dev, link, 1);
    netdev_dbg(dev.net, "Link Status is: %d\n", link);
    }
    }
#[no_mangle]
unsafe extern "C" fn dm9601_link_reset(dev: *mut usbnet) -> c_int {
    static int dm9601_link_reset(struct usbnet *dev)
    {
    let mut ecmd: ethtool_cmd = { .cmd = ETHTOOL_GSET };
    mii_check_media(&dev.mii, 1, 1);
    mii_ethtool_gset(&dev.mii, &ecmd);
    netdev_dbg(dev.net, "link_reset() speed: %u duplex: %d\n",
    ethtool_cmd_speed(&ecmd), ecmd.duplex);
    return 0;
    }
    static const struct driver_info dm9601_info = {
    .description	= "Davicom DM96xx USB 10/100 Ethernet",
    .flags		= FLAG_ETHER | FLAG_LINK_INTR,
    .bind		= dm9601_bind,
    .rx_fixup	= dm9601_rx_fixup,
    .tx_fixup	= dm9601_tx_fixup,
    .status		= dm9601_status,
    .link_reset	= dm9601_link_reset,
    .reset		= dm9601_link_reset,
    };
    static const struct usb_device_id products[] = {
    {
    USB_DEVICE(0x07aa, 0x9601),	/* Corega FEther USB-TXC */
    .driver_info = (unsigned long)&dm9601_info,
    },
    {
    USB_DEVICE(0x0a46, 0x9601),	/* Davicom USB-100 */
    .driver_info = (unsigned long)&dm9601_info,
    },
    {
    USB_DEVICE(0x0a46, 0x6688),	/* ZT6688 USB NIC */
    .driver_info = (unsigned long)&dm9601_info,
    },
    {
    USB_DEVICE(0x0a46, 0x0268),	/* ShanTou ST268 USB NIC */
    .driver_info = (unsigned long)&dm9601_info,
    },
    {
    USB_DEVICE(0x0a46, 0x8515),	/* ADMtek ADM8515 USB NIC */
    .driver_info = (unsigned long)&dm9601_info,
    },
    {
    USB_DEVICE(0x0a47, 0x9601),	/* Hirose USB-100 */
    .driver_info = (unsigned long)&dm9601_info,
    },
    {
    USB_DEVICE(0x0fe6, 0x8101),	/* DM9601 USB to Fast Ethernet Adapter */
    .driver_info = (unsigned long)&dm9601_info,
    },
    {
    USB_DEVICE(0x0a46, 0x9000),	/* DM9000E */
    .driver_info = (unsigned long)&dm9601_info,
    },
    {
    USB_DEVICE(0x0a46, 0x9620),	/* DM9620 USB to Fast Ethernet Adapter */
    .driver_info = (unsigned long)&dm9601_info,
    },
    {
    USB_DEVICE(0x0a46, 0x9621),	/* DM9621A USB to Fast Ethernet Adapter */
    .driver_info = (unsigned long)&dm9601_info,
    },
    {
    USB_DEVICE(0x0a46, 0x9622),	/* DM9622 USB to Fast Ethernet Adapter */
    .driver_info = (unsigned long)&dm9601_info,
    },
    {
    USB_DEVICE(0x0a46, 0x0269),	/* DM962OA USB to Fast Ethernet Adapter */
    .driver_info = (unsigned long)&dm9601_info,
    },
    {
    USB_DEVICE(0x0a46, 0x1269),	/* DM9621A USB to Fast Ethernet Adapter */
    .driver_info = (unsigned long)&dm9601_info,
    },
    {
    USB_DEVICE(0x0586, 0x3427),	/* ZyXEL Keenetic Plus DSL xDSL modem */
    .driver_info = (unsigned long)&dm9601_info,
    },
    {},			// END
    };
    MODULE_DEVICE_TABLE(usb, products);
    static struct usb_driver dm9601_driver = {
    .name = "dm9601",
    .id_table = products,
    .probe = usbnet_probe,
    .disconnect = usbnet_disconnect,
    .suspend = usbnet_suspend,
    .resume = usbnet_resume,
    .disable_hub_initiated_lpm = 1,
    };
    module_usb_driver(dm9601_driver);
    MODULE_AUTHOR("Peter Korsgaard <jacmet@sunsite.dk>");
    MODULE_DESCRIPTION("Davicom DM96xx USB 10/100 ethernet devices");
    MODULE_LICENSE("GPL");
