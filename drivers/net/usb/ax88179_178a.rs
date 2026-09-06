//! Automatically rewritten from C to Rust
//! Source: drivers/net/usb/ax88179_178a.c
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
//
// ASIX AX88179/178A USB 3.0/2.0 to Gigabit Ethernet Devices
//
// Copyright (C) 2011-2013 ASIX
//

pub const AX88179_PHY_ID: c_uint = 0x03;
pub const AX_EEPROM_LEN: c_uint = 0x100;
pub const AX88179_EEPROM_MAGIC: c_uint = 0x17900b95;
pub const AX_MCAST_FLTSIZE: c_int = 8;
pub const AX_MAX_MCAST: c_int = 64;

pub const AX_RXHDR_L4_TYPE_MASK: c_uint = 0x1c;
pub const AX_RXHDR_L4_TYPE_UDP: c_int = 4;
pub const AX_RXHDR_L4_TYPE_TCP: c_int = 16;
pub const AX_RXHDR_L3CSUM_ERR: c_int = 2;
pub const AX_RXHDR_L4CSUM_ERR: c_int = 1;

pub const AX_ACCESS_MAC: c_uint = 0x01;
pub const AX_ACCESS_PHY: c_uint = 0x02;
pub const AX_ACCESS_EEPROM: c_uint = 0x04;
pub const AX_ACCESS_EFUS: c_uint = 0x05;
pub const AX_RELOAD_EEPROM_EFUSE: c_uint = 0x06;
pub const AX_PAUSE_WATERLVL_HIGH: c_uint = 0x54;
pub const AX_PAUSE_WATERLVL_LOW: c_uint = 0x55;
pub const PHYSICAL_LINK_STATUS: c_uint = 0x02;
pub const AX_USB_SS: c_uint = 0x04;
pub const AX_USB_HS: c_uint = 0x02;
pub const GENERAL_STATUS: c_uint = 0x03;
// Check AX88179 version. UA1:Bit2 = 0,  UA2:Bit2 = 1
pub const AX_SECLD: c_uint = 0x04;
pub const AX_SROM_ADDR: c_uint = 0x07;
pub const AX_SROM_CMD: c_uint = 0x0a;
pub const EEP_RD: c_uint = 0x04;
pub const EEP_BUSY: c_uint = 0x10;
pub const AX_SROM_DATA_LOW: c_uint = 0x08;
pub const AX_SROM_DATA_HIGH: c_uint = 0x09;
pub const AX_RX_CTL: c_uint = 0x0b;
pub const AX_RX_CTL_DROPCRCERR: c_uint = 0x0100;
pub const AX_RX_CTL_IPE: c_uint = 0x0200;
pub const AX_RX_CTL_START: c_uint = 0x0080;
pub const AX_RX_CTL_AP: c_uint = 0x0020;
pub const AX_RX_CTL_AM: c_uint = 0x0010;
pub const AX_RX_CTL_AB: c_uint = 0x0008;
pub const AX_RX_CTL_AMALL: c_uint = 0x0002;
pub const AX_RX_CTL_PRO: c_uint = 0x0001;
pub const AX_RX_CTL_STOP: c_uint = 0x0000;
pub const AX_NODE_ID: c_uint = 0x10;
pub const AX_MULFLTARY: c_uint = 0x16;
pub const AX_MEDIUM_STATUS_MODE: c_uint = 0x22;
pub const AX_MEDIUM_GIGAMODE: c_uint = 0x01;
pub const AX_MEDIUM_FULL_DUPLEX: c_uint = 0x02;
pub const AX_MEDIUM_EN_125MHZ: c_uint = 0x08;
pub const AX_MEDIUM_RXFLOW_CTRLEN: c_uint = 0x10;
pub const AX_MEDIUM_TXFLOW_CTRLEN: c_uint = 0x20;
pub const AX_MEDIUM_RECEIVE_EN: c_uint = 0x100;
pub const AX_MEDIUM_PS: c_uint = 0x200;
pub const AX_MEDIUM_JUMBO_EN: c_uint = 0x8040;
pub const AX_MONITOR_MOD: c_uint = 0x24;
pub const AX_MONITOR_MODE_RWLC: c_uint = 0x02;
pub const AX_MONITOR_MODE_RWMP: c_uint = 0x04;
pub const AX_MONITOR_MODE_PMEPOL: c_uint = 0x20;
pub const AX_MONITOR_MODE_PMETYPE: c_uint = 0x40;
pub const AX_GPIO_CTRL: c_uint = 0x25;
pub const AX_GPIO_CTRL_GPIO3EN: c_uint = 0x80;
pub const AX_GPIO_CTRL_GPIO2EN: c_uint = 0x40;
pub const AX_GPIO_CTRL_GPIO1EN: c_uint = 0x20;
pub const AX_PHYPWR_RSTCTL: c_uint = 0x26;
pub const AX_PHYPWR_RSTCTL_BZ: c_uint = 0x0010;
pub const AX_PHYPWR_RSTCTL_IPRL: c_uint = 0x0020;
pub const AX_PHYPWR_RSTCTL_AT: c_uint = 0x1000;
pub const AX_RX_BULKIN_QCTRL: c_uint = 0x2e;
pub const AX_CLK_SELECT: c_uint = 0x33;
pub const AX_CLK_SELECT_BCS: c_uint = 0x01;
pub const AX_CLK_SELECT_ACS: c_uint = 0x02;
pub const AX_CLK_SELECT_ULR: c_uint = 0x08;
pub const AX_RXCOE_CTL: c_uint = 0x34;
pub const AX_RXCOE_IP: c_uint = 0x01;
pub const AX_RXCOE_TCP: c_uint = 0x02;
pub const AX_RXCOE_UDP: c_uint = 0x04;
pub const AX_RXCOE_TCPV6: c_uint = 0x20;
pub const AX_RXCOE_UDPV6: c_uint = 0x40;
pub const AX_TXCOE_CTL: c_uint = 0x35;
pub const AX_TXCOE_IP: c_uint = 0x01;
pub const AX_TXCOE_TCP: c_uint = 0x02;
pub const AX_TXCOE_UDP: c_uint = 0x04;
pub const AX_TXCOE_TCPV6: c_uint = 0x20;
pub const AX_TXCOE_UDPV6: c_uint = 0x40;
pub const AX_LEDCTRL: c_uint = 0x73;
pub const GMII_PHY_PHYSR: c_uint = 0x11;
pub const GMII_PHY_PHYSR_SMASK: c_uint = 0xc000;
pub const GMII_PHY_PHYSR_GIGA: c_uint = 0x8000;
pub const GMII_PHY_PHYSR_100: c_uint = 0x4000;
pub const GMII_PHY_PHYSR_FULL: c_uint = 0x2000;
pub const GMII_PHY_PHYSR_LINK: c_uint = 0x400;
pub const GMII_LED_ACT: c_uint = 0x1a;
pub const GMII_LED_ACTIVE_MASK: c_uint = 0xff8f;

pub const GMII_LED_LINK: c_uint = 0x1c;
pub const GMII_LED_LINK_MASK: c_uint = 0xf888;

pub const LED0_USB3_MASK: c_uint = 0x001f;

pub const LED1_USB3_MASK: c_uint = 0x03e0;

pub const LED2_USB3_MASK: c_uint = 0x7c00;
pub const GMII_PHYPAGE: c_uint = 0x1e;
pub const GMII_PHY_PAGE_SELECT: c_uint = 0x1f;
pub const GMII_PHY_PGSEL_EXT: c_uint = 0x0007;
pub const GMII_PHY_PGSEL_PAGE0: c_uint = 0x0000;
pub const GMII_PHY_PGSEL_PAGE3: c_uint = 0x0003;
pub const GMII_PHY_PGSEL_PAGE5: c_uint = 0x0005;
    static int ax88179_reset(struct usbnet *dev);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ax88179_data {
    pub eee_enabled: u8,
    pub eee_active: u8,
    pub rxctl: u16,
    pub in_pm: u8,
    pub wol_supported: u32,
    pub wolopts: u32,
    pub disconnecting: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ax88179_int_data {
    pub intdata1: __le32,
    pub intdata2: __le32,
}

    static const struct {
    unsigned char ctrl, timer_l, timer_h, size, ifg;
    } AX88179_BULKIN_SIZE[] =	{
    {7, 0x4f, 0,	0x12, 0xff},
    {7, 0x20, 3,	0x16, 0xff},
    {7, 0xae, 7,	0x18, 0xff},
    {7, 0xcc, 0x4c, 0x18, 8},
    };
#[no_mangle]
unsafe extern "C" fn ax88179_set_pm_mode(dev: *mut usbnet, pm_mode: bool) {
    static void ax88179_set_pm_mode(struct usbnet *dev, bool pm_mode)
    {
    struct ax88179_data *ax179_data = dev.driver_priv;
    ax179_data.in_pm = pm_mode;
    }
#[no_mangle]
unsafe extern "C" fn ax88179_in_pm(dev: *mut usbnet) -> c_int {
    static int ax88179_in_pm(struct usbnet *dev)
    {
    struct ax88179_data *ax179_data = dev.driver_priv;
    return ax179_data.in_pm;
    }
    static int __ax88179_read_cmd(struct usbnet *dev, u8 cmd, u16 value, u16 index,
    u16 size, void *data)
    {
    int ret;
    int (*fn)(struct usbnet *, u8, u8, u16, u16, void *, u16);
    struct ax88179_data *ax179_data = dev.driver_priv;
    BUG_ON(!dev);
    if (!ax88179_in_pm(dev))
    fn = usbnet_read_cmd;
    else
    fn = usbnet_read_cmd_nopm;
    ret = fn(dev, cmd, USB_DIR_IN | USB_TYPE_VENDOR | USB_RECIP_DEVICE,
    value, index, data, size);
    if (unlikely((ret < 0) && !(ret == -ENODEV && ax179_data.disconnecting)))
    netdev_warn(dev.net, "Failed to read reg index 0x%04x: %d\n",
    index, ret);
    return ret;
    }
    static int __ax88179_write_cmd(struct usbnet *dev, u8 cmd, u16 value, u16 index,
    u16 size, const void *data)
    {
    int ret;
    int (*fn)(struct usbnet *, u8, u8, u16, u16, const void *, u16);
    struct ax88179_data *ax179_data = dev.driver_priv;
    BUG_ON(!dev);
    if (!ax88179_in_pm(dev))
    fn = usbnet_write_cmd;
    else
    fn = usbnet_write_cmd_nopm;
    ret = fn(dev, cmd, USB_DIR_OUT | USB_TYPE_VENDOR | USB_RECIP_DEVICE,
    value, index, data, size);
    if (unlikely((ret < 0) && !(ret == -ENODEV && ax179_data.disconnecting)))
    netdev_warn(dev.net, "Failed to write reg index 0x%04x: %d\n",
    index, ret);
    return ret;
    }
    static void ax88179_write_cmd_async(struct usbnet *dev, u8 cmd, u16 value,
    u16 index, u16 size, void *data)
    {
    u16 buf;
    if (2 == size) {
    buf = *((u16 *)data);
    cpu_to_le16s(&buf);
    usbnet_write_cmd_async(dev, cmd, USB_DIR_OUT | USB_TYPE_VENDOR |
    USB_RECIP_DEVICE, value, index, &buf,
    size);
    } else {
    usbnet_write_cmd_async(dev, cmd, USB_DIR_OUT | USB_TYPE_VENDOR |
    USB_RECIP_DEVICE, value, index, data,
    size);
    }
    }
    static int ax88179_read_cmd(struct usbnet *dev, u8 cmd, u16 value, u16 index,
    u16 size, void *data)
    {
    int ret;
    if (2 == size) {
    let mut buf: u16 = 0;
    ret = __ax88179_read_cmd(dev, cmd, value, index, size, &buf);
    le16_to_cpus(&buf);
// ((u16 *)data) = buf;
    } else if (4 == size) {
    let mut buf: u32 = 0;
    ret = __ax88179_read_cmd(dev, cmd, value, index, size, &buf);
    le32_to_cpus(&buf);
// ((u32 *)data) = buf;
    } else {
    ret = __ax88179_read_cmd(dev, cmd, value, index, size, data);
    }
    return ret;
    }
    static int ax88179_write_cmd(struct usbnet *dev, u8 cmd, u16 value, u16 index,
    u16 size, const void *data)
    {
    int ret;
    if (2 == size) {
    u16 buf;
    buf = *((u16 *)data);
    cpu_to_le16s(&buf);
    ret = __ax88179_write_cmd(dev, cmd, value, index,
    size, &buf);
    } else {
    ret = __ax88179_write_cmd(dev, cmd, value, index,
    size, data);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ax88179_status(dev: *mut usbnet, urb: *mut urb) {
    static void ax88179_status(struct usbnet *dev, struct urb *urb)
    {
    struct ax88179_int_data *event;
    u32 link;
    if (urb.actual_length < 8)
    return;
    event = urb.transfer_buffer;
    le32_to_cpus((void *)&event.intdata1);
    link = ((( u32)event.intdata1) & AX_INT_PPLS_LINK) >> 16;
    if (netif_carrier_ok(dev.net) != link) {
    usbnet_link_change(dev, link, 1);
    if (!link)
    netdev_info(dev.net, "ax88179 - Link status is: 0\n");
    }
    }
#[no_mangle]
unsafe extern "C" fn ax88179_mdio_read(netdev: *mut net_device, phy_id: c_int, loc: c_int) -> c_int {
    static int ax88179_mdio_read(struct net_device *netdev, int phy_id, int loc)
    {
    struct usbnet *dev = netdev_priv(netdev);
    u16 res;
    ax88179_read_cmd(dev, AX_ACCESS_PHY, phy_id, (__u16)loc, 2, &res);
    return res;
    }
    static void ax88179_mdio_write(struct net_device *netdev, int phy_id, int loc,
    int val)
    {
    struct usbnet *dev = netdev_priv(netdev);
    let mut res: u16 = (u16) val;
    ax88179_write_cmd(dev, AX_ACCESS_PHY, phy_id, (__u16)loc, 2, &res);
    }
    static inline int ax88179_phy_mmd_indirect(struct usbnet *dev, u16 prtad,
    u16 devad)
    {
    u16 tmp16;
    int ret;
    tmp16 = devad;
    ret = ax88179_write_cmd(dev, AX_ACCESS_PHY, AX88179_PHY_ID,
    MII_MMD_CTRL, 2, &tmp16);
    tmp16 = prtad;
    ret = ax88179_write_cmd(dev, AX_ACCESS_PHY, AX88179_PHY_ID,
    MII_MMD_DATA, 2, &tmp16);
    tmp16 = devad | MII_MMD_CTRL_NOINCR;
    ret = ax88179_write_cmd(dev, AX_ACCESS_PHY, AX88179_PHY_ID,
    MII_MMD_CTRL, 2, &tmp16);
    return ret;
    }
    static int
    ax88179_phy_read_mmd_indirect(struct usbnet *dev, u16 prtad, u16 devad)
    {
    int ret;
    u16 tmp16;
    ax88179_phy_mmd_indirect(dev, prtad, devad);
    ret = ax88179_read_cmd(dev, AX_ACCESS_PHY, AX88179_PHY_ID,
    MII_MMD_DATA, 2, &tmp16);
    if (ret < 0)
    return ret;
    return tmp16;
    }
    static int
    ax88179_phy_write_mmd_indirect(struct usbnet *dev, u16 prtad, u16 devad,
    u16 data)
    {
    int ret;
    ax88179_phy_mmd_indirect(dev, prtad, devad);
    ret = ax88179_write_cmd(dev, AX_ACCESS_PHY, AX88179_PHY_ID,
    MII_MMD_DATA, 2, &data);
    if (ret < 0)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ax88179_suspend(intf: *mut usb_interface, message: pm_message_t) -> c_int {
    static int ax88179_suspend(struct usb_interface *intf, pm_message_t message)
    {
    struct usbnet *dev = usb_get_intfdata(intf);
    struct ax88179_data *priv = dev.driver_priv;
    u16 tmp16;
    u8 tmp8;
    ax88179_set_pm_mode(dev, true);
    usbnet_suspend(intf, message);
// Enable WoL
    if (priv.wolopts) {
    ax88179_read_cmd(dev, AX_ACCESS_MAC, AX_MONITOR_MOD,
    1, 1, &tmp8);
    if (priv.wolopts & WAKE_PHY)
    tmp8 |= AX_MONITOR_MODE_RWLC;
    if (priv.wolopts & WAKE_MAGIC)
    tmp8 |= AX_MONITOR_MODE_RWMP;
    ax88179_write_cmd(dev, AX_ACCESS_MAC, AX_MONITOR_MOD,
    1, 1, &tmp8);
    }
// Disable RX path
    ax88179_read_cmd(dev, AX_ACCESS_MAC, AX_MEDIUM_STATUS_MODE,
    2, 2, &tmp16);
    tmp16 &= ~AX_MEDIUM_RECEIVE_EN;
    ax88179_write_cmd(dev, AX_ACCESS_MAC, AX_MEDIUM_STATUS_MODE,
    2, 2, &tmp16);
// Force bulk-in zero length
    ax88179_read_cmd(dev, AX_ACCESS_MAC, AX_PHYPWR_RSTCTL,
    2, 2, &tmp16);
    tmp16 |= AX_PHYPWR_RSTCTL_BZ | AX_PHYPWR_RSTCTL_IPRL;
    ax88179_write_cmd(dev, AX_ACCESS_MAC, AX_PHYPWR_RSTCTL,
    2, 2, &tmp16);
// change clock
    tmp8 = 0;
    ax88179_write_cmd(dev, AX_ACCESS_MAC, AX_CLK_SELECT, 1, 1, &tmp8);
// Configure RX control register => stop operation
    tmp16 = AX_RX_CTL_STOP;
    ax88179_write_cmd(dev, AX_ACCESS_MAC, AX_RX_CTL, 2, 2, &tmp16);
    ax88179_set_pm_mode(dev, false);
    return 0;
    }
// This function is used to enable the autodetach function.
// This function is determined by offset 0x43 of EEPROM
#[no_mangle]
unsafe extern "C" fn ax88179_auto_detach(dev: *mut usbnet) -> c_int {
    static int ax88179_auto_detach(struct usbnet *dev)
    {
    u16 tmp16;
    u8 tmp8;
    if (ax88179_read_cmd(dev, AX_ACCESS_EEPROM, 0x43, 1, 2, &tmp16) < 0)
    return 0;
    if ((tmp16 == 0xFFFF) || (!(tmp16 & 0x0100)))
    return 0;
// Enable Auto Detach bit
    tmp8 = 0;
    ax88179_read_cmd(dev, AX_ACCESS_MAC, AX_CLK_SELECT, 1, 1, &tmp8);
    tmp8 |= AX_CLK_SELECT_ULR;
    ax88179_write_cmd(dev, AX_ACCESS_MAC, AX_CLK_SELECT, 1, 1, &tmp8);
    ax88179_read_cmd(dev, AX_ACCESS_MAC, AX_PHYPWR_RSTCTL, 2, 2, &tmp16);
    tmp16 |= AX_PHYPWR_RSTCTL_AT;
    ax88179_write_cmd(dev, AX_ACCESS_MAC, AX_PHYPWR_RSTCTL, 2, 2, &tmp16);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ax88179_resume(intf: *mut usb_interface) -> c_int {
    static int ax88179_resume(struct usb_interface *intf)
    {
    struct usbnet *dev = usb_get_intfdata(intf);
    ax88179_set_pm_mode(dev, true);
    usbnet_link_change(dev, 0, 0);
    ax88179_reset(dev);
    ax88179_set_pm_mode(dev, false);
    return usbnet_resume(intf);
    }
#[no_mangle]
unsafe extern "C" fn ax88179_disconnect(intf: *mut usb_interface) {
    static void ax88179_disconnect(struct usb_interface *intf)
    {
    struct usbnet *dev = usb_get_intfdata(intf);
    struct ax88179_data *ax179_data;
    if (!dev)
    return;
    ax179_data = dev.driver_priv;
    ax179_data.disconnecting = 1;
    usbnet_disconnect(intf);
    }
    static void
    ax88179_get_wol(struct net_device *net, struct ethtool_wolinfo *wolinfo)
    {
    struct usbnet *dev = netdev_priv(net);
    struct ax88179_data *priv = dev.driver_priv;
    wolinfo.supported = priv.wol_supported;
    wolinfo.wolopts = priv.wolopts;
    }
    static int
    ax88179_set_wol(struct net_device *net, struct ethtool_wolinfo *wolinfo)
    {
    struct usbnet *dev = netdev_priv(net);
    struct ax88179_data *priv = dev.driver_priv;
    if (wolinfo.wolopts & ~(priv.wol_supported))
    return -EINVAL;
    priv.wolopts = wolinfo.wolopts;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ax88179_get_eeprom_len(net: *mut net_device) -> c_int {
    static int ax88179_get_eeprom_len(struct net_device *net)
    {
    return AX_EEPROM_LEN;
    }
    static int
    ax88179_get_eeprom(struct net_device *net, struct ethtool_eeprom *eeprom,
    u8 *data)
    {
    struct usbnet *dev = netdev_priv(net);
    u16 *eeprom_buff;
    int first_word, last_word;
    int i, ret;
    if (eeprom.len == 0)
    return -EINVAL;
    eeprom.magic = AX88179_EEPROM_MAGIC;
    first_word = eeprom.offset >> 1;
    last_word = (eeprom.offset + eeprom.len - 1) >> 1;
    eeprom_buff = kmalloc_array(last_word - first_word + 1, sizeof(u16),
    GFP_KERNEL);
    if (!eeprom_buff)
    return -ENOMEM;
// ax88179/178A returns 2 bytes from eeprom on read
    for (i = first_word; i <= last_word; i++) {
    ret = __ax88179_read_cmd(dev, AX_ACCESS_EEPROM, i, 1, 2,
    &eeprom_buff[i - first_word]);
    if (ret < 0) {
    kfree(eeprom_buff);
    return -EIO;
    }
    }
    memcpy(data, (u8 *)eeprom_buff + (eeprom.offset & 1), eeprom.len);
    kfree(eeprom_buff);
    return 0;
    }
    static int
    ax88179_set_eeprom(struct net_device *net, struct ethtool_eeprom *eeprom,
    u8 *data)
    {
    struct usbnet *dev = netdev_priv(net);
    u16 *eeprom_buff;
    int first_word;
    int last_word;
    int ret;
    int i;
    netdev_dbg(net, "write EEPROM len %d, offset %d, magic 0x%x\n",
    eeprom.len, eeprom.offset, eeprom.magic);
    if (eeprom.len == 0)
    return -EINVAL;
    if (eeprom.magic != AX88179_EEPROM_MAGIC)
    return -EINVAL;
    first_word = eeprom.offset >> 1;
    last_word = (eeprom.offset + eeprom.len - 1) >> 1;
    eeprom_buff = kmalloc_array(last_word - first_word + 1, sizeof(u16),
    GFP_KERNEL);
    if (!eeprom_buff)
    return -ENOMEM;
// align data to 16 bit boundaries, read the missing data from
    the EEPROM */
    if (eeprom.offset & 1) {
    ret = ax88179_read_cmd(dev, AX_ACCESS_EEPROM, first_word, 1, 2,
    &eeprom_buff[0]);
    if (ret < 0) {
    netdev_err(net, "Failed to read EEPROM at offset 0x%02x.\n", first_word);
    goto free;
    }
    }
    if ((eeprom.offset + eeprom.len) & 1) {
    ret = ax88179_read_cmd(dev, AX_ACCESS_EEPROM, last_word, 1, 2,
    &eeprom_buff[last_word - first_word]);
    if (ret < 0) {
    netdev_err(net, "Failed to read EEPROM at offset 0x%02x.\n", last_word);
    goto free;
    }
    }
    memcpy((u8 *)eeprom_buff + (eeprom.offset & 1), data, eeprom.len);
    for (i = first_word; i <= last_word; i++) {
    netdev_dbg(net, "write to EEPROM at offset 0x%02x, data 0x%04x\n",
    i, eeprom_buff[i - first_word]);
    ret = ax88179_write_cmd(dev, AX_ACCESS_EEPROM, i, 1, 2,
    &eeprom_buff[i - first_word]);
    if (ret < 0) {
    netdev_err(net, "Failed to write EEPROM at offset 0x%02x.\n", i);
    goto free;
    }
    msleep(20);
    }
// reload EEPROM data
    ret = ax88179_write_cmd(dev, AX_RELOAD_EEPROM_EFUSE, 0x0000, 0, 0, core::ptr::null_mut());
    if (ret < 0) {
    netdev_err(net, "Failed to reload EEPROM data\n");
    goto free;
    }
    ret = 0;
    free:
    kfree(eeprom_buff);
    return ret;
    }
    static int ax88179_get_link_ksettings(struct net_device *net,
    struct ethtool_link_ksettings *cmd)
    {
    struct usbnet *dev = netdev_priv(net);
    mii_ethtool_get_link_ksettings(&dev.mii, cmd);
    return 0;
    }
    static int ax88179_set_link_ksettings(struct net_device *net,
    const struct ethtool_link_ksettings *cmd)
    {
    struct usbnet *dev = netdev_priv(net);
    return mii_ethtool_set_link_ksettings(&dev.mii, cmd);
    }
    static int
    ax88179_ethtool_get_eee(struct usbnet *dev, struct ethtool_keee *data)
    {
    int val;
// Get Supported EEE
    val = ax88179_phy_read_mmd_indirect(dev, MDIO_PCS_EEE_ABLE,
    MDIO_MMD_PCS);
    if (val < 0)
    return val;
    mii_eee_cap1_mod_linkmode_t(data.supported, val);
// Get advertisement EEE
    val = ax88179_phy_read_mmd_indirect(dev, MDIO_AN_EEE_ADV,
    MDIO_MMD_AN);
    if (val < 0)
    return val;
    mii_eee_cap1_mod_linkmode_t(data.advertised, val);
// Get LP advertisement EEE
    val = ax88179_phy_read_mmd_indirect(dev, MDIO_AN_EEE_LPABLE,
    MDIO_MMD_AN);
    if (val < 0)
    return val;
    mii_eee_cap1_mod_linkmode_t(data.lp_advertised, val);
    return 0;
    }
    static int
    ax88179_ethtool_set_eee(struct usbnet *dev, struct ethtool_keee *data)
    {
    let mut tmp16: u16 = linkmode_to_mii_eee_cap1_t(data.advertised);
    return ax88179_phy_write_mmd_indirect(dev, MDIO_AN_EEE_ADV,
    MDIO_MMD_AN, tmp16);
    }
#[no_mangle]
unsafe extern "C" fn ax88179_chk_eee(dev: *mut usbnet) -> c_int {
    static int ax88179_chk_eee(struct usbnet *dev)
    {
    let mut ecmd: ethtool_cmd = { .cmd = ETHTOOL_GSET };
    struct ax88179_data *priv = dev.driver_priv;
    mii_ethtool_gset(&dev.mii, &ecmd);
    if (ecmd.duplex & DUPLEX_FULL) {
    int eee_lp, eee_cap, eee_adv;
    u32 lp, cap, adv, supported = 0;
    eee_cap = ax88179_phy_read_mmd_indirect(dev,
    MDIO_PCS_EEE_ABLE,
    MDIO_MMD_PCS);
    if (eee_cap < 0) {
    priv.eee_active = 0;
    return false;
    }
    cap = mmd_eee_cap_to_ethtool_sup_t(eee_cap);
    if (!cap) {
    priv.eee_active = 0;
    return false;
    }
    eee_lp = ax88179_phy_read_mmd_indirect(dev,
    MDIO_AN_EEE_LPABLE,
    MDIO_MMD_AN);
    if (eee_lp < 0) {
    priv.eee_active = 0;
    return false;
    }
    eee_adv = ax88179_phy_read_mmd_indirect(dev,
    MDIO_AN_EEE_ADV,
    MDIO_MMD_AN);
    if (eee_adv < 0) {
    priv.eee_active = 0;
    return false;
    }
    adv = mmd_eee_adv_to_ethtool_adv_t(eee_adv);
    lp = mmd_eee_adv_to_ethtool_adv_t(eee_lp);
    supported = (ecmd.speed == SPEED_1000) ?
    SUPPORTED_1000baseT_Full :
    SUPPORTED_100baseT_Full;
    if (!(lp & adv & supported)) {
    priv.eee_active = 0;
    return false;
    }
    priv.eee_active = 1;
    return true;
    }
    priv.eee_active = 0;
    return false;
    }
#[no_mangle]
unsafe extern "C" fn ax88179_disable_eee(dev: *mut usbnet) {
    static void ax88179_disable_eee(struct usbnet *dev)
    {
    u16 tmp16;
    tmp16 = GMII_PHY_PGSEL_PAGE3;
    ax88179_write_cmd(dev, AX_ACCESS_PHY, AX88179_PHY_ID,
    GMII_PHY_PAGE_SELECT, 2, &tmp16);
    tmp16 = 0x3246;
    ax88179_write_cmd(dev, AX_ACCESS_PHY, AX88179_PHY_ID,
    MII_PHYADDR, 2, &tmp16);
    tmp16 = GMII_PHY_PGSEL_PAGE0;
    ax88179_write_cmd(dev, AX_ACCESS_PHY, AX88179_PHY_ID,
    GMII_PHY_PAGE_SELECT, 2, &tmp16);
    }
#[no_mangle]
unsafe extern "C" fn ax88179_enable_eee(dev: *mut usbnet) {
    static void ax88179_enable_eee(struct usbnet *dev)
    {
    u16 tmp16;
    tmp16 = GMII_PHY_PGSEL_PAGE3;
    ax88179_write_cmd(dev, AX_ACCESS_PHY, AX88179_PHY_ID,
    GMII_PHY_PAGE_SELECT, 2, &tmp16);
    tmp16 = 0x3247;
    ax88179_write_cmd(dev, AX_ACCESS_PHY, AX88179_PHY_ID,
    MII_PHYADDR, 2, &tmp16);
    tmp16 = GMII_PHY_PGSEL_PAGE5;
    ax88179_write_cmd(dev, AX_ACCESS_PHY, AX88179_PHY_ID,
    GMII_PHY_PAGE_SELECT, 2, &tmp16);
    tmp16 = 0x0680;
    ax88179_write_cmd(dev, AX_ACCESS_PHY, AX88179_PHY_ID,
    MII_BMSR, 2, &tmp16);
    tmp16 = GMII_PHY_PGSEL_PAGE0;
    ax88179_write_cmd(dev, AX_ACCESS_PHY, AX88179_PHY_ID,
    GMII_PHY_PAGE_SELECT, 2, &tmp16);
    }
#[no_mangle]
unsafe extern "C" fn ax88179_get_eee(net: *mut net_device, edata: *mut ethtool_keee) -> c_int {
    static int ax88179_get_eee(struct net_device *net, struct ethtool_keee *edata)
    {
    struct usbnet *dev = netdev_priv(net);
    struct ax88179_data *priv = dev.driver_priv;
    edata.eee_enabled = priv.eee_enabled;
    edata.eee_active = priv.eee_active;
    return ax88179_ethtool_get_eee(dev, edata);
    }
#[no_mangle]
unsafe extern "C" fn ax88179_set_eee(net: *mut net_device, edata: *mut ethtool_keee) -> c_int {
    static int ax88179_set_eee(struct net_device *net, struct ethtool_keee *edata)
    {
    struct usbnet *dev = netdev_priv(net);
    struct ax88179_data *priv = dev.driver_priv;
    int ret;
    priv.eee_enabled = edata.eee_enabled;
    if (!priv.eee_enabled) {
    ax88179_disable_eee(dev);
    } else {
    priv.eee_enabled = ax88179_chk_eee(dev);
    if (!priv.eee_enabled)
    return -EOPNOTSUPP;
    ax88179_enable_eee(dev);
    }
    ret = ax88179_ethtool_set_eee(dev, edata);
    if (ret)
    return ret;
    mii_nway_restart(&dev.mii);
    usbnet_link_change(dev, 0, 0);
    return ret;
    }
    static const struct ethtool_ops ax88179_ethtool_ops = {
    .get_link		= ethtool_op_get_link,
    .get_msglevel		= usbnet_get_msglevel,
    .set_msglevel		= usbnet_set_msglevel,
    .get_wol		= ax88179_get_wol,
    .set_wol		= ax88179_set_wol,
    .get_eeprom_len		= ax88179_get_eeprom_len,
    .get_eeprom		= ax88179_get_eeprom,
    .set_eeprom		= ax88179_set_eeprom,
    .get_eee		= ax88179_get_eee,
    .set_eee		= ax88179_set_eee,
    .nway_reset		= usbnet_nway_reset,
    .get_link_ksettings	= ax88179_get_link_ksettings,
    .set_link_ksettings	= ax88179_set_link_ksettings,
    .get_ts_info		= ethtool_op_get_ts_info,
    };
#[no_mangle]
unsafe extern "C" fn ax88179_set_multicast(net: *mut net_device) {
    static void ax88179_set_multicast(struct net_device *net)
    {
    struct usbnet *dev = netdev_priv(net);
    struct ax88179_data *data = dev.driver_priv;
    u8 *m_filter = ((u8 *)dev.data);
    data.rxctl = (AX_RX_CTL_START | AX_RX_CTL_AB | AX_RX_CTL_IPE);
    if (net.flags & IFF_PROMISC) {
    data.rxctl |= AX_RX_CTL_PRO;
    } else if (net.flags & IFF_ALLMULTI ||
    netdev_mc_count(net) > AX_MAX_MCAST) {
    data.rxctl |= AX_RX_CTL_AMALL;
    } else if (netdev_mc_empty(net)) {
// just broadcast and directed
    } else {
// We use dev->data for our 8 byte filter buffer
// to avoid allocating memory that is tricky to free later
//
    u32 crc_bits;
    struct netdev_hw_addr *ha;
    memset(m_filter, 0, AX_MCAST_FLTSIZE);
    netdev_for_each_mc_addr(ha, net) {
    crc_bits = ether_crc(ETH_ALEN, ha.addr) >> 26;
// (m_filter + (crc_bits >> 3)) |= (1 << (crc_bits & 7));
    }
    ax88179_write_cmd_async(dev, AX_ACCESS_MAC, AX_MULFLTARY,
    AX_MCAST_FLTSIZE, AX_MCAST_FLTSIZE,
    m_filter);
    data.rxctl |= AX_RX_CTL_AM;
    }
    ax88179_write_cmd_async(dev, AX_ACCESS_MAC, AX_RX_CTL,
    2, 2, &data.rxctl);
    }
    static int
    ax88179_set_features(struct net_device *net, netdev_features_t features)
    {
    u8 tmp;
    struct usbnet *dev = netdev_priv(net);
    let mut changed: netdev_features_t = net.features ^ features;
    if (changed & NETIF_F_IP_CSUM) {
    ax88179_read_cmd(dev, AX_ACCESS_MAC, AX_TXCOE_CTL, 1, 1, &tmp);
    tmp ^= AX_TXCOE_TCP | AX_TXCOE_UDP;
    ax88179_write_cmd(dev, AX_ACCESS_MAC, AX_TXCOE_CTL, 1, 1, &tmp);
    }
    if (changed & NETIF_F_IPV6_CSUM) {
    ax88179_read_cmd(dev, AX_ACCESS_MAC, AX_TXCOE_CTL, 1, 1, &tmp);
    tmp ^= AX_TXCOE_TCPV6 | AX_TXCOE_UDPV6;
    ax88179_write_cmd(dev, AX_ACCESS_MAC, AX_TXCOE_CTL, 1, 1, &tmp);
    }
    if (changed & NETIF_F_RXCSUM) {
    ax88179_read_cmd(dev, AX_ACCESS_MAC, AX_RXCOE_CTL, 1, 1, &tmp);
    tmp ^= AX_RXCOE_IP | AX_RXCOE_TCP | AX_RXCOE_UDP |
    AX_RXCOE_TCPV6 | AX_RXCOE_UDPV6;
    ax88179_write_cmd(dev, AX_ACCESS_MAC, AX_RXCOE_CTL, 1, 1, &tmp);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ax88179_change_mtu(net: *mut net_device, new_mtu: c_int) -> c_int {
    static int ax88179_change_mtu(struct net_device *net, int new_mtu)
    {
    struct usbnet *dev = netdev_priv(net);
    u16 tmp16;
    WRITE_ONCE(net.mtu, new_mtu);
    dev.hard_mtu = net.mtu + net.hard_header_len;
    if (net.mtu > 1500) {
    ax88179_read_cmd(dev, AX_ACCESS_MAC, AX_MEDIUM_STATUS_MODE,
    2, 2, &tmp16);
    tmp16 |= AX_MEDIUM_JUMBO_EN;
    ax88179_write_cmd(dev, AX_ACCESS_MAC, AX_MEDIUM_STATUS_MODE,
    2, 2, &tmp16);
    } else {
    ax88179_read_cmd(dev, AX_ACCESS_MAC, AX_MEDIUM_STATUS_MODE,
    2, 2, &tmp16);
    tmp16 &= ~AX_MEDIUM_JUMBO_EN;
    ax88179_write_cmd(dev, AX_ACCESS_MAC, AX_MEDIUM_STATUS_MODE,
    2, 2, &tmp16);
    }
// max qlen depend on hard_mtu and rx_urb_size
    usbnet_update_max_qlen(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ax88179_set_mac_addr(net: *mut net_device, p: *mut c_void) -> c_int {
    static int ax88179_set_mac_addr(struct net_device *net, void *p)
    {
    struct usbnet *dev = netdev_priv(net);
    struct sockaddr *addr = p;
    int ret;
    if (netif_running(net))
    return -EBUSY;
    if (!is_valid_ether_addr(addr.sa_data))
    return -EADDRNOTAVAIL;
    eth_hw_addr_set(net, addr.sa_data);
// Set the MAC address
    ret = ax88179_write_cmd(dev, AX_ACCESS_MAC, AX_NODE_ID, ETH_ALEN,
    ETH_ALEN, net.dev_addr);
    if (ret < 0)
    return ret;
    return 0;
    }
    static const struct net_device_ops ax88179_netdev_ops = {
    .ndo_open		= usbnet_open,
    .ndo_stop		= usbnet_stop,
    .ndo_start_xmit		= usbnet_start_xmit,
    .ndo_tx_timeout		= usbnet_tx_timeout,
    .ndo_get_stats64	= dev_get_tstats64,
    .ndo_change_mtu		= ax88179_change_mtu,
    .ndo_set_mac_address	= ax88179_set_mac_addr,
    .ndo_validate_addr	= eth_validate_addr,
    .ndo_eth_ioctl		= usbnet_mii_ioctl,
    .ndo_set_rx_mode	= ax88179_set_multicast,
    .ndo_set_features	= ax88179_set_features,
    };
#[no_mangle]
unsafe extern "C" fn ax88179_check_eeprom(dev: *mut usbnet) -> c_int {
    static int ax88179_check_eeprom(struct usbnet *dev)
    {
    u8 i, buf, eeprom[20];
    u16 csum, delay = HZ / 10;
    unsigned long jtimeout;
// Read EEPROM content
    for (i = 0; i < 6; i++) {
    buf = i;
    if (ax88179_write_cmd(dev, AX_ACCESS_MAC, AX_SROM_ADDR,
    1, 1, &buf) < 0)
    return -EINVAL;
    buf = EEP_RD;
    if (ax88179_write_cmd(dev, AX_ACCESS_MAC, AX_SROM_CMD,
    1, 1, &buf) < 0)
    return -EINVAL;
    jtimeout = jiffies + delay;
    do {
    ax88179_read_cmd(dev, AX_ACCESS_MAC, AX_SROM_CMD,
    1, 1, &buf);
    if (time_after(jiffies, jtimeout))
    return -EINVAL;
    } while (buf & EEP_BUSY);
    __ax88179_read_cmd(dev, AX_ACCESS_MAC, AX_SROM_DATA_LOW,
    2, 2, &eeprom[i * 2]);
    if ((i == 0) && (eeprom[0] == 0xFF))
    return -EINVAL;
    }
    csum = eeprom[6] + eeprom[7] + eeprom[8] + eeprom[9];
    csum = (csum >> 8) + (csum & 0xff);
    if ((csum + eeprom[10]) != 0xff)
    return -EINVAL;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ax88179_check_efuse(dev: *mut usbnet, ledmode: *mut u16) -> c_int {
    static int ax88179_check_efuse(struct usbnet *dev, u16 *ledmode)
    {
    u8	i;
    u8	efuse[64];
    let mut csum: u16 = 0;
    if (ax88179_read_cmd(dev, AX_ACCESS_EFUS, 0, 64, 64, efuse) < 0)
    return -EINVAL;
    if (*efuse == 0xFF)
    return -EINVAL;
    for (i = 0; i < 64; i++)
    csum = csum + efuse[i];
    while (csum > 255)
    csum = (csum & 0x00FF) + ((csum >> 8) & 0x00FF);
    if (csum != 0xFF)
    return -EINVAL;
// ledmode = (efuse[51] << 8) | efuse[52];
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ax88179_convert_old_led(dev: *mut usbnet, ledvalue: *mut u16) -> c_int {
    static int ax88179_convert_old_led(struct usbnet *dev, u16 *ledvalue)
    {
    u16 led;
// Loaded the old eFuse LED Mode
    if (ax88179_read_cmd(dev, AX_ACCESS_EEPROM, 0x3C, 1, 2, &led) < 0)
    return -EINVAL;
    led >>= 8;
    switch (led) {
    case 0xFF:
    led = LED0_ACTIVE | LED1_LINK_10 | LED1_LINK_100 |
    LED1_LINK_1000 | LED2_ACTIVE | LED2_LINK_10 |
    LED2_LINK_100 | LED2_LINK_1000 | LED_VALID;
    break;
    case 0xFE:
    led = LED0_ACTIVE | LED1_LINK_1000 | LED2_LINK_100 | LED_VALID;
    break;
    case 0xFD:
    led = LED0_ACTIVE | LED1_LINK_1000 | LED2_LINK_100 |
    LED2_LINK_10 | LED_VALID;
    break;
    case 0xFC:
    led = LED0_ACTIVE | LED1_ACTIVE | LED1_LINK_1000 | LED2_ACTIVE |
    LED2_LINK_100 | LED2_LINK_10 | LED_VALID;
    break;
    default:
    led = LED0_ACTIVE | LED1_LINK_10 | LED1_LINK_100 |
    LED1_LINK_1000 | LED2_ACTIVE | LED2_LINK_10 |
    LED2_LINK_100 | LED2_LINK_1000 | LED_VALID;
    break;
    }
// ledvalue = led;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ax88179_led_setting(dev: *mut usbnet) -> c_int {
    static int ax88179_led_setting(struct usbnet *dev)
    {
    u8 ledfd, value = 0;
    u16 tmp, ledact, ledlink, ledvalue = 0, delay = HZ / 10;
    unsigned long jtimeout;
// Check AX88179 version. UA1 or UA2
    ax88179_read_cmd(dev, AX_ACCESS_MAC, GENERAL_STATUS, 1, 1, &value);
    if (!(value & AX_SECLD)) {	/* UA1 */
    value = AX_GPIO_CTRL_GPIO3EN | AX_GPIO_CTRL_GPIO2EN |
    AX_GPIO_CTRL_GPIO1EN;
    if (ax88179_write_cmd(dev, AX_ACCESS_MAC, AX_GPIO_CTRL,
    1, 1, &value) < 0)
    return -EINVAL;
    }
// Check EEPROM
    if (!ax88179_check_eeprom(dev)) {
    value = 0x42;
    if (ax88179_write_cmd(dev, AX_ACCESS_MAC, AX_SROM_ADDR,
    1, 1, &value) < 0)
    return -EINVAL;
    value = EEP_RD;
    if (ax88179_write_cmd(dev, AX_ACCESS_MAC, AX_SROM_CMD,
    1, 1, &value) < 0)
    return -EINVAL;
    jtimeout = jiffies + delay;
    do {
    ax88179_read_cmd(dev, AX_ACCESS_MAC, AX_SROM_CMD,
    1, 1, &value);
    if (time_after(jiffies, jtimeout))
    return -EINVAL;
    } while (value & EEP_BUSY);
    ax88179_read_cmd(dev, AX_ACCESS_MAC, AX_SROM_DATA_HIGH,
    1, 1, &value);
    ledvalue = (value << 8);
    ax88179_read_cmd(dev, AX_ACCESS_MAC, AX_SROM_DATA_LOW,
    1, 1, &value);
    ledvalue |= value;
// load internal ROM for defaule setting
    if ((ledvalue == 0xFFFF) || ((ledvalue & LED_VALID) == 0))
    ax88179_convert_old_led(dev, &ledvalue);
    } else if (!ax88179_check_efuse(dev, &ledvalue)) {
    if ((ledvalue == 0xFFFF) || ((ledvalue & LED_VALID) == 0))
    ax88179_convert_old_led(dev, &ledvalue);
    } else {
    ax88179_convert_old_led(dev, &ledvalue);
    }
    tmp = GMII_PHY_PGSEL_EXT;
    ax88179_write_cmd(dev, AX_ACCESS_PHY, AX88179_PHY_ID,
    GMII_PHY_PAGE_SELECT, 2, &tmp);
    tmp = 0x2c;
    ax88179_write_cmd(dev, AX_ACCESS_PHY, AX88179_PHY_ID,
    GMII_PHYPAGE, 2, &tmp);
    ax88179_read_cmd(dev, AX_ACCESS_PHY, AX88179_PHY_ID,
    GMII_LED_ACT, 2, &ledact);
    ax88179_read_cmd(dev, AX_ACCESS_PHY, AX88179_PHY_ID,
    GMII_LED_LINK, 2, &ledlink);
    ledact &= GMII_LED_ACTIVE_MASK;
    ledlink &= GMII_LED_LINK_MASK;
    if (ledvalue & LED0_ACTIVE)
    ledact |= GMII_LED0_ACTIVE;
    if (ledvalue & LED1_ACTIVE)
    ledact |= GMII_LED1_ACTIVE;
    if (ledvalue & LED2_ACTIVE)
    ledact |= GMII_LED2_ACTIVE;
    if (ledvalue & LED0_LINK_10)
    ledlink |= GMII_LED0_LINK_10;
    if (ledvalue & LED1_LINK_10)
    ledlink |= GMII_LED1_LINK_10;
    if (ledvalue & LED2_LINK_10)
    ledlink |= GMII_LED2_LINK_10;
    if (ledvalue & LED0_LINK_100)
    ledlink |= GMII_LED0_LINK_100;
    if (ledvalue & LED1_LINK_100)
    ledlink |= GMII_LED1_LINK_100;
    if (ledvalue & LED2_LINK_100)
    ledlink |= GMII_LED2_LINK_100;
    if (ledvalue & LED0_LINK_1000)
    ledlink |= GMII_LED0_LINK_1000;
    if (ledvalue & LED1_LINK_1000)
    ledlink |= GMII_LED1_LINK_1000;
    if (ledvalue & LED2_LINK_1000)
    ledlink |= GMII_LED2_LINK_1000;
    tmp = ledact;
    ax88179_write_cmd(dev, AX_ACCESS_PHY, AX88179_PHY_ID,
    GMII_LED_ACT, 2, &tmp);
    tmp = ledlink;
    ax88179_write_cmd(dev, AX_ACCESS_PHY, AX88179_PHY_ID,
    GMII_LED_LINK, 2, &tmp);
    tmp = GMII_PHY_PGSEL_PAGE0;
    ax88179_write_cmd(dev, AX_ACCESS_PHY, AX88179_PHY_ID,
    GMII_PHY_PAGE_SELECT, 2, &tmp);
// LED full duplex setting
    ledfd = 0;
    if (ledvalue & LED0_FD)
    ledfd |= 0x01;
#[no_mangle]
pub unsafe extern "C" fn if(0: (ledvalue & LED0_USB3_MASK) ==) -> else {
    else if ((ledvalue & LED0_USB3_MASK) == 0)
    ledfd |= 0x02;
    if (ledvalue & LED1_FD)
    ledfd |= 0x04;
#[no_mangle]
pub unsafe extern "C" fn if(0: (ledvalue & LED1_USB3_MASK) ==) -> else {
    else if ((ledvalue & LED1_USB3_MASK) == 0)
    ledfd |= 0x08;
    if (ledvalue & LED2_FD)
    ledfd |= 0x10;
#[no_mangle]
pub unsafe extern "C" fn if(0: (ledvalue & LED2_USB3_MASK) ==) -> else {
    else if ((ledvalue & LED2_USB3_MASK) == 0)
    ledfd |= 0x20;
    ax88179_write_cmd(dev, AX_ACCESS_MAC, AX_LEDCTRL, 1, 1, &ledfd);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ax88179_get_mac_addr(dev: *mut usbnet) {
    static void ax88179_get_mac_addr(struct usbnet *dev)
    {
    u8 mac[ETH_ALEN];
    memset(mac, 0, sizeof(mac));
// Maybe the boot loader passed the MAC address via device tree
    if (!eth_platform_get_mac_address(&dev.udev.dev, mac)) {
    netif_dbg(dev, ifup, dev.net,
    "MAC address read from device tree");
    } else {
    ax88179_read_cmd(dev, AX_ACCESS_MAC, AX_NODE_ID, ETH_ALEN,
    ETH_ALEN, mac);
    netif_dbg(dev, ifup, dev.net,
    "MAC address read from ASIX chip");
    }
    if (is_valid_ether_addr(mac)) {
    eth_hw_addr_set(dev.net, mac);
    if (!is_local_ether_addr(mac))
    dev.net.addr_assign_type = NET_ADDR_PERM;
    } else {
    netdev_info(dev.net, "invalid MAC address, using random\n");
    }
    ax88179_write_cmd(dev, AX_ACCESS_MAC, AX_NODE_ID, ETH_ALEN, ETH_ALEN,
    dev.net.dev_addr);
    }
#[no_mangle]
unsafe extern "C" fn ax88179_bind(dev: *mut usbnet, intf: *mut usb_interface) -> c_int {
    static int ax88179_bind(struct usbnet *dev, struct usb_interface *intf)
    {
    struct ax88179_data *ax179_data;
    int ret;
    ret = usbnet_get_endpoints(dev, intf);
    if (ret < 0)
    return ret;
    ax179_data = kzalloc_obj(*ax179_data);
    if (!ax179_data)
    return -ENOMEM;
    dev.driver_priv = ax179_data;
    dev.net.netdev_ops = &ax88179_netdev_ops;
    dev.net.ethtool_ops = &ax88179_ethtool_ops;
    dev.net.needed_headroom = 8;
    dev.net.max_mtu = 4088;
// Initialize MII structure
    dev.mii.dev = dev.net;
    dev.mii.mdio_read = ax88179_mdio_read;
    dev.mii.mdio_write = ax88179_mdio_write;
    dev.mii.phy_id_mask = 0xff;
    dev.mii.reg_num_mask = 0xff;
    dev.mii.phy_id = 0x03;
    dev.mii.supports_gmii = 1;
    dev.net.features |= NETIF_F_SG | NETIF_F_IP_CSUM |
    NETIF_F_IPV6_CSUM | NETIF_F_RXCSUM | NETIF_F_TSO;
    dev.net.hw_features |= dev.net.features;
    netif_set_tso_max_size(dev.net, 16384);
    ax88179_reset(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ax88179_unbind(dev: *mut usbnet, intf: *mut usb_interface) {
    static void ax88179_unbind(struct usbnet *dev, struct usb_interface *intf)
    {
    struct ax88179_data *ax179_data = dev.driver_priv;
    u16 tmp16;
// Configure RX control register => stop operation
    tmp16 = AX_RX_CTL_STOP;
    ax88179_write_cmd(dev, AX_ACCESS_MAC, AX_RX_CTL, 2, 2, &tmp16);
    tmp16 = 0;
    ax88179_write_cmd(dev, AX_ACCESS_MAC, AX_CLK_SELECT, 1, 1, &tmp16);
// Power down ethernet PHY
    tmp16 = 0;
    ax88179_write_cmd(dev, AX_ACCESS_MAC, AX_PHYPWR_RSTCTL, 2, 2, &tmp16);
    kfree(ax179_data);
    }
    static void
    ax88179_rx_checksum(struct sk_buff *skb, u32 *pkt_hdr)
    {
    skb.ip_summed = CHECKSUM_NONE;
// checksum error bit is set
    if ((*pkt_hdr & AX_RXHDR_L3CSUM_ERR) ||
    (*pkt_hdr & AX_RXHDR_L4CSUM_ERR))
    return;
// It must be a TCP or UDP packet with a valid checksum
    if (((*pkt_hdr & AX_RXHDR_L4_TYPE_MASK) == AX_RXHDR_L4_TYPE_TCP) ||
    ((*pkt_hdr & AX_RXHDR_L4_TYPE_MASK) == AX_RXHDR_L4_TYPE_UDP))
    skb.ip_summed = CHECKSUM_UNNECESSARY;
    }
#[no_mangle]
unsafe extern "C" fn ax88179_rx_fixup(dev: *mut usbnet, skb: *mut sk_buff) -> c_int {
    static int ax88179_rx_fixup(struct usbnet *dev, struct sk_buff *skb)
    {
    struct sk_buff *ax_skb;
    int pkt_cnt;
    u32 rx_hdr;
    u16 hdr_off;
    u32 *pkt_hdr;
// At the end of the SKB, there's a header telling us how many packets
// are bundled into this buffer and where we can find an array of
// per-packet metadata (which contains elements encoded into u16).
//
// SKB contents for current firmware:
// <packet 1> <padding>
// ...
// <packet N> <padding>
// <per-packet metadata entry 1> <dummy header>
// ...
// <per-packet metadata entry N> <dummy header>
// <padding2> <rx_hdr>
//
// where:
// <packet N> contains pkt_len bytes:
// 2 bytes of IP alignment pseudo header
// packet received
// <per-packet metadata entry N> contains 4 bytes:
// pkt_len and fields AX_RXHDR_
// <padding>	0-7 bytes to terminate at
// 8 bytes boundary (64-bit).
// <padding2> 4 bytes to make rx_hdr terminate at
// 8 bytes boundary (64-bit)
// <dummy-header> contains 4 bytes:
// pkt_len=0 and AX_RXHDR_DROP_ERR
// <rx-hdr>	contains 4 bytes:
// pkt_cnt and hdr_off (offset of
// <per-packet metadata entry 1>)
//
// pkt_cnt is number of entrys in the per-packet metadata.
// In current firmware there is 2 entrys per packet.
// The first points to the packet and the
// second is a dummy header.
// This was done probably to align fields in 64-bit and
// maintain compatibility with old firmware.
// This code assumes that <dummy header> and <padding2> are
// optional.
//
    if (skb.len < 4)
    return 0;
    skb_trim(skb, skb.len - 4);
    rx_hdr = get_unaligned_le32(skb_tail_pointer(skb));
    pkt_cnt = (u16)rx_hdr;
    hdr_off = (u16)(rx_hdr >> 16);
    if (pkt_cnt == 0)
    return 0;
// Make sure that the bounds of the metadata array are inside the SKB
// (and in front of the counter at the end).
//
    if (pkt_cnt * 4 + hdr_off > skb.len)
    return 0;
    pkt_hdr = (u32 *)(skb.data + hdr_off);
// Packets must not overlap the metadata array
    skb_trim(skb, hdr_off);
    for (; pkt_cnt > 0; pkt_cnt--, pkt_hdr++) {
    u16 pkt_len_plus_padd;
    u16 pkt_len;
    le32_to_cpus(pkt_hdr);
    pkt_len = (*pkt_hdr >> 16) & 0x1fff;
    pkt_len_plus_padd = (pkt_len + 7) & 0xfff8;
// Skip dummy header used for alignment
//
    if (pkt_len == 0)
    continue;
    if (pkt_len_plus_padd > skb.len)
    return 0;
// Check CRC or runt packet
    if ((*pkt_hdr & (AX_RXHDR_CRC_ERR | AX_RXHDR_DROP_ERR)) ||
    pkt_len < 2 + ETH_HLEN) {
    dev.net.stats.rx_errors++;
    skb_pull(skb, pkt_len_plus_padd);
    continue;
    }
// last packet
    if (pkt_len_plus_padd == skb.len) {
    skb_trim(skb, pkt_len);
// Skip IP alignment pseudo header
    skb_pull(skb, 2);
    ax88179_rx_checksum(skb, pkt_hdr);
    return 1;
    }
    ax_skb = netdev_alloc_skb_ip_align(dev.net, pkt_len);
    if (!ax_skb)
    return 0;
    skb_put(ax_skb, pkt_len);
    memcpy(ax_skb.data, skb.data + 2, pkt_len);
    ax88179_rx_checksum(ax_skb, pkt_hdr);
    usbnet_skb_return(dev, ax_skb);
    skb_pull(skb, pkt_len_plus_padd);
    }
    return 0;
    }
    static struct sk_buff *
    ax88179_tx_fixup(struct usbnet *dev, struct sk_buff *skb, gfp_t flags)
    {
    u32 tx_hdr1, tx_hdr2;
    let mut frame_size: c_int = dev.maxpacket;
    int headroom;
    void *ptr;
    tx_hdr1 = skb.len;
    tx_hdr2 = skb_shinfo(skb).gso_size; /* Set TSO mss */
    if (((skb.len + 8) % frame_size) == 0)
    tx_hdr2 |= 0x80008000;	/* Enable padding */
    headroom = skb_headroom(skb) - 8;
    if ((dev.net.features & NETIF_F_SG) && skb_linearize(skb)) {
    dev_kfree_skb_any(skb);
    return core::ptr::null_mut();
    }
    if ((skb_header_cloned(skb) || headroom < 0) &&
    pskb_expand_head(skb, headroom < 0 ? 8 : 0, 0, GFP_ATOMIC)) {
    dev_kfree_skb_any(skb);
    return core::ptr::null_mut();
    }
    ptr = skb_push(skb, 8);
    put_unaligned_le32(tx_hdr1, ptr);
    put_unaligned_le32(tx_hdr2, ptr + 4);
    usbnet_set_skb_tx_stats(skb, (skb_shinfo(skb).gso_segs ?: 1), 0);
    return skb;
    }
#[no_mangle]
unsafe extern "C" fn ax88179_link_reset(dev: *mut usbnet) -> c_int {
    static int ax88179_link_reset(struct usbnet *dev)
    {
    struct ax88179_data *ax179_data = dev.driver_priv;
    u8 tmp[5], link_sts;
    u16 mode, tmp16, delay = HZ / 10;
    let mut tmp32: u32 = 0x40000000;
    unsigned long jtimeout;
    jtimeout = jiffies + delay;
    while (tmp32 & 0x40000000) {
    mode = 0;
    ax88179_write_cmd(dev, AX_ACCESS_MAC, AX_RX_CTL, 2, 2, &mode);
    ax88179_write_cmd(dev, AX_ACCESS_MAC, AX_RX_CTL, 2, 2,
    &ax179_data.rxctl);
// link up, check the usb device control TX FIFO full or empty
    ax88179_read_cmd(dev, 0x81, 0x8c, 0, 4, &tmp32);
    if (time_after(jiffies, jtimeout))
    return 0;
    }
    mode = AX_MEDIUM_RECEIVE_EN | AX_MEDIUM_TXFLOW_CTRLEN |
    AX_MEDIUM_RXFLOW_CTRLEN;
    ax88179_read_cmd(dev, AX_ACCESS_MAC, PHYSICAL_LINK_STATUS,
    1, 1, &link_sts);
    ax88179_read_cmd(dev, AX_ACCESS_PHY, AX88179_PHY_ID,
    GMII_PHY_PHYSR, 2, &tmp16);
    if (!(tmp16 & GMII_PHY_PHYSR_LINK)) {
    netdev_info(dev.net, "ax88179 - Link status is: 0\n");
    return 0;
    } else if (GMII_PHY_PHYSR_GIGA == (tmp16 & GMII_PHY_PHYSR_SMASK)) {
    mode |= AX_MEDIUM_GIGAMODE | AX_MEDIUM_EN_125MHZ;
    if (dev.net.mtu > 1500)
    mode |= AX_MEDIUM_JUMBO_EN;
    if (link_sts & AX_USB_SS)
    memcpy(tmp, &AX88179_BULKIN_SIZE[0], 5);
#[no_mangle]
pub unsafe extern "C" fn if(AX_USB_HS: link_sts &) -> else {
    else if (link_sts & AX_USB_HS)
    memcpy(tmp, &AX88179_BULKIN_SIZE[1], 5);
    else
    memcpy(tmp, &AX88179_BULKIN_SIZE[3], 5);
    } else if (GMII_PHY_PHYSR_100 == (tmp16 & GMII_PHY_PHYSR_SMASK)) {
    mode |= AX_MEDIUM_PS;
    if (link_sts & (AX_USB_SS | AX_USB_HS))
    memcpy(tmp, &AX88179_BULKIN_SIZE[2], 5);
    else
    memcpy(tmp, &AX88179_BULKIN_SIZE[3], 5);
    } else {
    memcpy(tmp, &AX88179_BULKIN_SIZE[3], 5);
    }
// RX bulk configuration
    ax88179_write_cmd(dev, AX_ACCESS_MAC, AX_RX_BULKIN_QCTRL, 5, 5, tmp);
    dev.rx_urb_size = (1024 * (tmp[3] + 2));
    if (tmp16 & GMII_PHY_PHYSR_FULL)
    mode |= AX_MEDIUM_FULL_DUPLEX;
    ax88179_write_cmd(dev, AX_ACCESS_MAC, AX_MEDIUM_STATUS_MODE,
    2, 2, &mode);
    ax179_data.eee_enabled = ax88179_chk_eee(dev);
    netif_carrier_on(dev.net);
    netdev_info(dev.net, "ax88179 - Link status is: 1\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ax88179_reset(dev: *mut usbnet) -> c_int {
    static int ax88179_reset(struct usbnet *dev)
    {
    u8 buf[5];
    u16 *tmp16;
    u8 *tmp;
    struct ax88179_data *ax179_data = dev.driver_priv;
    struct ethtool_keee eee_data;
    tmp16 = (u16 *)buf;
    tmp = (u8 *)buf;
// Power up ethernet PHY
// tmp16 = 0;
    ax88179_write_cmd(dev, AX_ACCESS_MAC, AX_PHYPWR_RSTCTL, 2, 2, tmp16);
// tmp16 = AX_PHYPWR_RSTCTL_IPRL;
    ax88179_write_cmd(dev, AX_ACCESS_MAC, AX_PHYPWR_RSTCTL, 2, 2, tmp16);
    msleep(500);
// tmp = AX_CLK_SELECT_ACS | AX_CLK_SELECT_BCS;
    ax88179_write_cmd(dev, AX_ACCESS_MAC, AX_CLK_SELECT, 1, 1, tmp);
    msleep(200);
// Ethernet PHY Auto Detach
    ax88179_auto_detach(dev);
// Read MAC address from DTB or asix chip
    ax88179_get_mac_addr(dev);
    memcpy(dev.net.perm_addr, dev.net.dev_addr, ETH_ALEN);
// RX bulk configuration
    memcpy(tmp, &AX88179_BULKIN_SIZE[0], 5);
    ax88179_write_cmd(dev, AX_ACCESS_MAC, AX_RX_BULKIN_QCTRL, 5, 5, tmp);
    dev.rx_urb_size = 1024 * 20;
// tmp = 0x34;
    ax88179_write_cmd(dev, AX_ACCESS_MAC, AX_PAUSE_WATERLVL_LOW, 1, 1, tmp);
// tmp = 0x52;
    ax88179_write_cmd(dev, AX_ACCESS_MAC, AX_PAUSE_WATERLVL_HIGH,
    1, 1, tmp);
// Enable checksum offload
// tmp = AX_RXCOE_IP | AX_RXCOE_TCP | AX_RXCOE_UDP |
    AX_RXCOE_TCPV6 | AX_RXCOE_UDPV6;
    ax88179_write_cmd(dev, AX_ACCESS_MAC, AX_RXCOE_CTL, 1, 1, tmp);
// tmp = AX_TXCOE_IP | AX_TXCOE_TCP | AX_TXCOE_UDP |
    AX_TXCOE_TCPV6 | AX_TXCOE_UDPV6;
    ax88179_write_cmd(dev, AX_ACCESS_MAC, AX_TXCOE_CTL, 1, 1, tmp);
// Configure RX control register => start operation
// tmp16 = AX_RX_CTL_DROPCRCERR | AX_RX_CTL_IPE | AX_RX_CTL_START |
    AX_RX_CTL_AP | AX_RX_CTL_AMALL | AX_RX_CTL_AB;
    ax88179_write_cmd(dev, AX_ACCESS_MAC, AX_RX_CTL, 2, 2, tmp16);
// tmp = AX_MONITOR_MODE_PMETYPE | AX_MONITOR_MODE_PMEPOL |
    AX_MONITOR_MODE_RWMP;
    ax88179_write_cmd(dev, AX_ACCESS_MAC, AX_MONITOR_MOD, 1, 1, tmp);
// Configure default medium type => giga
// tmp16 = AX_MEDIUM_RECEIVE_EN | AX_MEDIUM_TXFLOW_CTRLEN |
    AX_MEDIUM_RXFLOW_CTRLEN | AX_MEDIUM_FULL_DUPLEX |
    AX_MEDIUM_GIGAMODE;
    ax88179_write_cmd(dev, AX_ACCESS_MAC, AX_MEDIUM_STATUS_MODE,
    2, 2, tmp16);
// Check if WoL is supported
    ax179_data.wol_supported = 0;
    if (ax88179_read_cmd(dev, AX_ACCESS_MAC, AX_MONITOR_MOD,
    1, 1, &tmp) > 0)
    ax179_data.wol_supported = WAKE_MAGIC | WAKE_PHY;
    ax88179_led_setting(dev);
    ax179_data.eee_enabled = 0;
    ax179_data.eee_active = 0;
    ax88179_disable_eee(dev);
    ax88179_ethtool_get_eee(dev, &eee_data);
    linkmode_zero(eee_data.advertised);
    ax88179_ethtool_set_eee(dev, &eee_data);
// Restart autoneg
    mii_nway_restart(&dev.mii);
    usbnet_link_change(dev, 0, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ax88179_net_reset(dev: *mut usbnet) -> c_int {
    static int ax88179_net_reset(struct usbnet *dev)
    {
    u16 tmp16;
    ax88179_read_cmd(dev, AX_ACCESS_PHY, AX88179_PHY_ID, GMII_PHY_PHYSR,
    2, &tmp16);
    if (tmp16) {
    ax88179_read_cmd(dev, AX_ACCESS_MAC, AX_MEDIUM_STATUS_MODE,
    2, 2, &tmp16);
    if (!(tmp16 & AX_MEDIUM_RECEIVE_EN)) {
    tmp16 |= AX_MEDIUM_RECEIVE_EN;
    ax88179_write_cmd(dev, AX_ACCESS_MAC, AX_MEDIUM_STATUS_MODE,
    2, 2, &tmp16);
    }
    } else {
    ax88179_reset(dev);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ax88179_stop(dev: *mut usbnet) -> c_int {
    static int ax88179_stop(struct usbnet *dev)
    {
    u16 tmp16;
    ax88179_read_cmd(dev, AX_ACCESS_MAC, AX_MEDIUM_STATUS_MODE,
    2, 2, &tmp16);
    tmp16 &= ~AX_MEDIUM_RECEIVE_EN;
    ax88179_write_cmd(dev, AX_ACCESS_MAC, AX_MEDIUM_STATUS_MODE,
    2, 2, &tmp16);
    return 0;
    }
    static const struct driver_info ax88179_info = {
    .description = "ASIX AX88179 USB 3.0 Gigabit Ethernet",
    .bind = ax88179_bind,
    .unbind = ax88179_unbind,
    .status = ax88179_status,
    .link_reset = ax88179_link_reset,
    .reset = ax88179_net_reset,
    .stop = ax88179_stop,
    .flags = FLAG_ETHER | FLAG_FRAMING_AX,
    .rx_fixup = ax88179_rx_fixup,
    .tx_fixup = ax88179_tx_fixup,
    };
    static const struct driver_info ax88178a_info = {
    .description = "ASIX AX88178A USB 2.0 Gigabit Ethernet",
    .bind = ax88179_bind,
    .unbind = ax88179_unbind,
    .status = ax88179_status,
    .link_reset = ax88179_link_reset,
    .reset = ax88179_net_reset,
    .stop = ax88179_stop,
    .flags = FLAG_ETHER | FLAG_FRAMING_AX,
    .rx_fixup = ax88179_rx_fixup,
    .tx_fixup = ax88179_tx_fixup,
    };
    static const struct driver_info cypress_GX3_info = {
    .description = "Cypress GX3 SuperSpeed to Gigabit Ethernet Controller",
    .bind = ax88179_bind,
    .unbind = ax88179_unbind,
    .status = ax88179_status,
    .link_reset = ax88179_link_reset,
    .reset = ax88179_net_reset,
    .stop = ax88179_stop,
    .flags = FLAG_ETHER | FLAG_FRAMING_AX,
    .rx_fixup = ax88179_rx_fixup,
    .tx_fixup = ax88179_tx_fixup,
    };
    static const struct driver_info dlink_dub1312_info = {
    .description = "D-Link DUB-1312 USB 3.0 to Gigabit Ethernet Adapter",
    .bind = ax88179_bind,
    .unbind = ax88179_unbind,
    .status = ax88179_status,
    .link_reset = ax88179_link_reset,
    .reset = ax88179_net_reset,
    .stop = ax88179_stop,
    .flags = FLAG_ETHER | FLAG_FRAMING_AX,
    .rx_fixup = ax88179_rx_fixup,
    .tx_fixup = ax88179_tx_fixup,
    };
    static const struct driver_info sitecom_info = {
    .description = "Sitecom USB 3.0 to Gigabit Adapter",
    .bind = ax88179_bind,
    .unbind = ax88179_unbind,
    .status = ax88179_status,
    .link_reset = ax88179_link_reset,
    .reset = ax88179_net_reset,
    .stop = ax88179_stop,
    .flags = FLAG_ETHER | FLAG_FRAMING_AX,
    .rx_fixup = ax88179_rx_fixup,
    .tx_fixup = ax88179_tx_fixup,
    };
    static const struct driver_info samsung_info = {
    .description = "Samsung USB Ethernet Adapter",
    .bind = ax88179_bind,
    .unbind = ax88179_unbind,
    .status = ax88179_status,
    .link_reset = ax88179_link_reset,
    .reset = ax88179_net_reset,
    .stop = ax88179_stop,
    .flags = FLAG_ETHER | FLAG_FRAMING_AX,
    .rx_fixup = ax88179_rx_fixup,
    .tx_fixup = ax88179_tx_fixup,
    };
    static const struct driver_info lenovo_info = {
    .description = "Lenovo OneLinkDock Gigabit LAN",
    .bind = ax88179_bind,
    .unbind = ax88179_unbind,
    .status = ax88179_status,
    .link_reset = ax88179_link_reset,
    .reset = ax88179_net_reset,
    .stop = ax88179_stop,
    .flags = FLAG_ETHER | FLAG_FRAMING_AX,
    .rx_fixup = ax88179_rx_fixup,
    .tx_fixup = ax88179_tx_fixup,
    };
    static const struct driver_info belkin_info = {
    .description = "Belkin USB Ethernet Adapter",
    .bind	= ax88179_bind,
    .unbind = ax88179_unbind,
    .status = ax88179_status,
    .link_reset = ax88179_link_reset,
    .reset	= ax88179_net_reset,
    .stop	= ax88179_stop,
    .flags	= FLAG_ETHER | FLAG_FRAMING_AX,
    .rx_fixup = ax88179_rx_fixup,
    .tx_fixup = ax88179_tx_fixup,
    };
    static const struct driver_info toshiba_info = {
    .description = "Toshiba USB Ethernet Adapter",
    .bind	= ax88179_bind,
    .unbind = ax88179_unbind,
    .status = ax88179_status,
    .link_reset = ax88179_link_reset,
    .reset	= ax88179_net_reset,
    .stop = ax88179_stop,
    .flags	= FLAG_ETHER | FLAG_FRAMING_AX,
    .rx_fixup = ax88179_rx_fixup,
    .tx_fixup = ax88179_tx_fixup,
    };
    static const struct driver_info mct_info = {
    .description = "MCT USB 3.0 Gigabit Ethernet Adapter",
    .bind	= ax88179_bind,
    .unbind	= ax88179_unbind,
    .status	= ax88179_status,
    .link_reset = ax88179_link_reset,
    .reset	= ax88179_net_reset,
    .stop	= ax88179_stop,
    .flags	= FLAG_ETHER | FLAG_FRAMING_AX,
    .rx_fixup = ax88179_rx_fixup,
    .tx_fixup = ax88179_tx_fixup,
    };
    static const struct driver_info at_umc2000_info = {
    .description = "AT-UMC2000 USB 3.0/USB 3.1 Gen 1 to Gigabit Ethernet Adapter",
    .bind   = ax88179_bind,
    .unbind = ax88179_unbind,
    .status = ax88179_status,
    .link_reset = ax88179_link_reset,
    .reset  = ax88179_net_reset,
    .stop   = ax88179_stop,
    .flags  = FLAG_ETHER | FLAG_FRAMING_AX,
    .rx_fixup = ax88179_rx_fixup,
    .tx_fixup = ax88179_tx_fixup,
    };
    static const struct driver_info at_umc200_info = {
    .description = "AT-UMC200 USB 3.0/USB 3.1 Gen 1 to Fast Ethernet Adapter",
    .bind   = ax88179_bind,
    .unbind = ax88179_unbind,
    .status = ax88179_status,
    .link_reset = ax88179_link_reset,
    .reset  = ax88179_net_reset,
    .stop   = ax88179_stop,
    .flags  = FLAG_ETHER | FLAG_FRAMING_AX,
    .rx_fixup = ax88179_rx_fixup,
    .tx_fixup = ax88179_tx_fixup,
    };
    static const struct driver_info at_umc2000sp_info = {
    .description = "AT-UMC2000/SP USB 3.0/USB 3.1 Gen 1 to Gigabit Ethernet Adapter",
    .bind   = ax88179_bind,
    .unbind = ax88179_unbind,
    .status = ax88179_status,
    .link_reset = ax88179_link_reset,
    .reset  = ax88179_net_reset,
    .stop   = ax88179_stop,
    .flags  = FLAG_ETHER | FLAG_FRAMING_AX,
    .rx_fixup = ax88179_rx_fixup,
    .tx_fixup = ax88179_tx_fixup,
    };
    static const struct usb_device_id products[] = {
    {
// ASIX AX88179 10/100/1000
    USB_DEVICE_AND_INTERFACE_INFO(0x0b95, 0x1790, 0xff, 0xff, 0),
    .driver_info = (unsigned long)&ax88179_info,
    }, {
// ASIX AX88178A 10/100/1000
    USB_DEVICE_AND_INTERFACE_INFO(0x0b95, 0x178a, 0xff, 0xff, 0),
    .driver_info = (unsigned long)&ax88178a_info,
    }, {
// Cypress GX3 SuperSpeed to Gigabit Ethernet Bridge Controller
    USB_DEVICE_AND_INTERFACE_INFO(0x04b4, 0x3610, 0xff, 0xff, 0),
    .driver_info = (unsigned long)&cypress_GX3_info,
    }, {
// D-Link DUB-1312 USB 3.0 to Gigabit Ethernet Adapter
    USB_DEVICE_AND_INTERFACE_INFO(0x2001, 0x4a00, 0xff, 0xff, 0),
    .driver_info = (unsigned long)&dlink_dub1312_info,
    }, {
// Sitecom USB 3.0 to Gigabit Adapter
    USB_DEVICE_AND_INTERFACE_INFO(0x0df6, 0x0072, 0xff, 0xff, 0),
    .driver_info = (unsigned long)&sitecom_info,
    }, {
// Samsung USB Ethernet Adapter
    USB_DEVICE_AND_INTERFACE_INFO(0x04e8, 0xa100, 0xff, 0xff, 0),
    .driver_info = (unsigned long)&samsung_info,
    }, {
// Lenovo OneLinkDock Gigabit LAN
    USB_DEVICE_AND_INTERFACE_INFO(0x17ef, 0x304b, 0xff, 0xff, 0),
    .driver_info = (unsigned long)&lenovo_info,
    }, {
// Belkin B2B128 USB 3.0 Hub + Gigabit Ethernet Adapter
    USB_DEVICE_AND_INTERFACE_INFO(0x050d, 0x0128, 0xff, 0xff, 0),
    .driver_info = (unsigned long)&belkin_info,
    }, {
// Toshiba USB 3.0 GBit Ethernet Adapter
    USB_DEVICE_AND_INTERFACE_INFO(0x0930, 0x0a13, 0xff, 0xff, 0),
    .driver_info = (unsigned long)&toshiba_info,
    }, {
// Magic Control Technology U3-A9003 USB 3.0 Gigabit Ethernet Adapter
    USB_DEVICE_AND_INTERFACE_INFO(0x0711, 0x0179, 0xff, 0xff, 0),
    .driver_info = (unsigned long)&mct_info,
    }, {
// Allied Telesis AT-UMC2000 USB 3.0/USB 3.1 Gen 1 to Gigabit Ethernet Adapter
    USB_DEVICE_AND_INTERFACE_INFO(0x07c9, 0x000e, 0xff, 0xff, 0),
    .driver_info = (unsigned long)&at_umc2000_info,
    }, {
// Allied Telesis AT-UMC200 USB 3.0/USB 3.1 Gen 1 to Fast Ethernet Adapter
    USB_DEVICE_AND_INTERFACE_INFO(0x07c9, 0x000f, 0xff, 0xff, 0),
    .driver_info = (unsigned long)&at_umc200_info,
    }, {
// Allied Telesis AT-UMC2000/SP USB 3.0/USB 3.1 Gen 1 to Gigabit Ethernet Adapter
    USB_DEVICE_AND_INTERFACE_INFO(0x07c9, 0x0010, 0xff, 0xff, 0),
    .driver_info = (unsigned long)&at_umc2000sp_info,
    },
    { },
    };
    MODULE_DEVICE_TABLE(usb, products);
    static struct usb_driver ax88179_178a_driver = {
    .name =		"ax88179_178a",
    .id_table =	products,
    .probe =	usbnet_probe,
    .suspend =	ax88179_suspend,
    .resume =	ax88179_resume,
    .reset_resume =	ax88179_resume,
    .disconnect =	ax88179_disconnect,
    .supports_autosuspend = 1,
    .disable_hub_initiated_lpm = 1,
    };
    module_usb_driver(ax88179_178a_driver);
    MODULE_DESCRIPTION("ASIX AX88179/178A based USB 3.0/2.0 Gigabit Ethernet Devices");
    MODULE_LICENSE("GPL");
