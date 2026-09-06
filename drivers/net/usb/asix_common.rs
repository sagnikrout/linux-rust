//! Automatically rewritten from C to Rust
//! Source: drivers/net/usb/asix_common.c
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
// ASIX AX8817X based USB 2.0 Ethernet Devices
// Copyright (C) 2003-2006 David Hollis <dhollis@davehollis.com>
// Copyright (C) 2005 Phil Chang <pchang23@sbcglobal.net>
// Copyright (C) 2006 James Painter <jamie.painter@iname.com>
// Copyright (c) 2002-2003 TiVo Inc.
//

pub const AX_HOST_EN_RETRIES: c_int = 30;
    int __must_check asix_read_cmd(struct usbnet *dev, u8 cmd, u16 value, u16 index,
    u16 size, void *data, int in_pm)
    {
    int ret;
    int (*fn)(struct usbnet *, u8, u8, u16, u16, void *, u16);
    BUG_ON(!dev);
    if (!in_pm)
    fn = usbnet_read_cmd;
    else
    fn = usbnet_read_cmd_nopm;
    ret = fn(dev, cmd, USB_DIR_IN | USB_TYPE_VENDOR | USB_RECIP_DEVICE,
    value, index, data, size);
    if (unlikely(ret < size)) {
    ret = ret < 0 ? ret : -ENODATA;
    netdev_warn(dev.net, "Failed to read reg index 0x%04x: %d\n",
    index, ret);
    }
    return ret;
    }
    int asix_write_cmd(struct usbnet *dev, u8 cmd, u16 value, u16 index,
    u16 size, void *data, int in_pm)
    {
    int ret;
    int (*fn)(struct usbnet *, u8, u8, u16, u16, const void *, u16);
    BUG_ON(!dev);
    if (!in_pm)
    fn = usbnet_write_cmd;
    else
    fn = usbnet_write_cmd_nopm;
    ret = fn(dev, cmd, USB_DIR_OUT | USB_TYPE_VENDOR | USB_RECIP_DEVICE,
    value, index, data, size);
    if (unlikely(ret < 0))
    netdev_warn(dev.net, "Failed to write reg index 0x%04x: %d\n",
    index, ret);
    return ret;
    }
    void asix_write_cmd_async(struct usbnet *dev, u8 cmd, u16 value, u16 index,
    u16 size, void *data)
    {
    usbnet_write_cmd_async(dev, cmd,
    USB_DIR_OUT | USB_TYPE_VENDOR | USB_RECIP_DEVICE,
    value, index, data, size);
    }
#[no_mangle]
unsafe extern "C" fn asix_set_sw_mii(dev: *mut usbnet, in_pm: c_int) -> c_int {
    static int asix_set_sw_mii(struct usbnet *dev, int in_pm)
    {
    int ret;
    ret = asix_write_cmd(dev, AX_CMD_SET_SW_MII, 0x0000, 0, 0, core::ptr::null_mut(), in_pm);
    if (ret < 0)
    netdev_err(dev.net, "Failed to enable software MII access\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn asix_set_hw_mii(dev: *mut usbnet, in_pm: c_int) -> c_int {
    static int asix_set_hw_mii(struct usbnet *dev, int in_pm)
    {
    int ret;
    ret = asix_write_cmd(dev, AX_CMD_SET_HW_MII, 0x0000, 0, 0, core::ptr::null_mut(), in_pm);
    if (ret < 0)
    netdev_err(dev.net, "Failed to enable hardware MII access\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn asix_check_host_enable(dev: *mut usbnet, in_pm: c_int) -> c_int {
    static int asix_check_host_enable(struct usbnet *dev, int in_pm)
    {
    int i, ret;
    u8 smsr;
    for (i = 0; i < AX_HOST_EN_RETRIES; ++i) {
    ret = asix_set_sw_mii(dev, in_pm);
    if (ret == -ENODEV || ret == -ETIMEDOUT)
    break;
    usleep_range(1000, 1100);
    ret = asix_read_cmd(dev, AX_CMD_STATMNGSTS_REG,
    0, 0, 1, &smsr, in_pm);
    if (ret == -ENODEV)
    break;
#[no_mangle]
pub unsafe extern "C" fn if(0: ret <) -> else {
    else if (ret < 0)
    continue;
#[no_mangle]
pub unsafe extern "C" fn if(AX_HOST_EN: smsr &) -> else {
    else if (smsr & AX_HOST_EN)
    break;
    }
    return i >= AX_HOST_EN_RETRIES ? -ETIMEDOUT : ret;
    }
#[no_mangle]
unsafe extern "C" fn reset_asix_rx_fixup_info(rx: *mut asix_rx_fixup_info) {
    static void reset_asix_rx_fixup_info(struct asix_rx_fixup_info *rx)
    {
// Reset the variables that have a lifetime outside of
// asix_rx_fixup_internal() so that future processing starts from a
// known set of initial conditions.
//
    if (rx.ax_skb) {
// Discard any incomplete Ethernet frame in the netdev buffer
    kfree_skb(rx.ax_skb);
    rx.ax_skb = core::ptr::null_mut();
    }
// Assume the Data header 32-bit word is at the start of the current
// or next URB socket buffer so reset all the state variables.
//
    rx.remaining = 0;
    rx.split_head = false;
    rx.header = 0;
    }
    int asix_rx_fixup_internal(struct usbnet *dev, struct sk_buff *skb,
    struct asix_rx_fixup_info *rx)
    {
    let mut offset: c_int = 0;
    u16 size;
// When an Ethernet frame spans multiple URB socket buffers,
// do a sanity test for the Data header synchronisation.
// Attempt to detect the situation of the previous socket buffer having
// been truncated or a socket buffer was missing. These situations
// cause a discontinuity in the data stream and therefore need to avoid
// appending bad data to the end of the current netdev socket buffer.
// Also avoid unnecessarily discarding a good current netdev socket
// buffer.
//
    if (rx.remaining && (rx.remaining + sizeof(u32) <= skb.len)) {
    offset = ((rx.remaining + 1) & 0xfffe);
    rx.header = get_unaligned_le32(skb.data + offset);
    offset = 0;
    size = (u16)(rx.header & 0x7ff);
    if (size != ((~rx.header >> 16) & 0x7ff)) {
    netdev_err(dev.net, "asix_rx_fixup() Data Header synchronisation was lost, remaining %d\n",
    rx.remaining);
    reset_asix_rx_fixup_info(rx);
    }
    }
    while (offset + sizeof(u16) <= skb.len) {
    u16 copy_length;
    if (!rx.remaining) {
    if (skb.len - offset == sizeof(u16)) {
    rx.header = get_unaligned_le16(
    skb.data + offset);
    rx.split_head = true;
    offset += sizeof(u16);
    break;
    }
    if (rx.split_head == true) {
    rx.header |= (get_unaligned_le16(
    skb.data + offset) << 16);
    rx.split_head = false;
    offset += sizeof(u16);
    } else {
    rx.header = get_unaligned_le32(skb.data +
    offset);
    offset += sizeof(u32);
    }
// take frame length from Data header 32-bit word
    size = (u16)(rx.header & 0x7ff);
    if (size != ((~rx.header >> 16) & 0x7ff)) {
    netdev_err(dev.net, "asix_rx_fixup() Bad Header Length 0x%x, offset %d\n",
    rx.header, offset);
    reset_asix_rx_fixup_info(rx);
    return 0;
    }
    if (size > dev.net.mtu + ETH_HLEN + VLAN_HLEN) {
    netdev_dbg(dev.net, "asix_rx_fixup() Bad RX Length %d\n",
    size);
    reset_asix_rx_fixup_info(rx);
    return 0;
    }
// Sometimes may fail to get a netdev socket buffer but
// continue to process the URB socket buffer so that
// synchronisation of the Ethernet frame Data header
// word is maintained.
//
    rx.ax_skb = netdev_alloc_skb_ip_align(dev.net, size);
    rx.remaining = size;
    }
    if (rx.remaining > skb.len - offset) {
    copy_length = skb.len - offset;
    rx.remaining -= copy_length;
    } else {
    copy_length = rx.remaining;
    rx.remaining = 0;
    }
    if (rx.ax_skb) {
    skb_put_data(rx.ax_skb, skb.data + offset,
    copy_length);
    if (!rx.remaining) {
    usbnet_skb_return(dev, rx.ax_skb);
    rx.ax_skb = core::ptr::null_mut();
    }
    }
    offset += (copy_length + 1) & 0xfffe;
    }
    if (skb.len != offset) {
    netdev_err(dev.net, "asix_rx_fixup() Bad SKB Length %d, %d\n",
    skb.len, offset);
    reset_asix_rx_fixup_info(rx);
    return 0;
    }
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn asix_rx_fixup_common(dev: *mut usbnet, skb: *mut sk_buff) -> c_int {
    int asix_rx_fixup_common(struct usbnet *dev, struct sk_buff *skb)
    {
    struct asix_common_private *dp = dev.driver_priv;
    struct asix_rx_fixup_info *rx = &dp.rx_fixup_info;
    return asix_rx_fixup_internal(dev, skb, rx);
    }
#[no_mangle]
pub unsafe extern "C" fn asix_rx_fixup_common_free(dp: *mut asix_common_private) {
    void asix_rx_fixup_common_free(struct asix_common_private *dp)
    {
    struct asix_rx_fixup_info *rx;
    if (!dp)
    return;
    rx = &dp.rx_fixup_info;
    if (rx.ax_skb) {
    kfree_skb(rx.ax_skb);
    rx.ax_skb = core::ptr::null_mut();
    }
    }
    struct sk_buff *asix_tx_fixup(struct usbnet *dev, struct sk_buff *skb,
    gfp_t flags)
    {
    int padlen;
    let mut headroom: c_int = skb_headroom(skb);
    let mut tailroom: c_int = skb_tailroom(skb);
    u32 packet_len;
    let mut padbytes: u32 = 0xffff0000;
    void *ptr;
    padlen = ((skb.len + 4) & (dev.maxpacket - 1)) ? 0 : 4;
// We need to push 4 bytes in front of frame (packet_len)
// and maybe add 4 bytes after the end (if padlen is 4)
//
// Avoid skb_copy_expand() expensive call, using following rules :
// - We are allowed to push 4 bytes in headroom if skb_header_cloned()
// is false (and if we have 4 bytes of headroom)
// - We are allowed to put 4 bytes at tail if skb_cloned()
// is false (and if we have 4 bytes of tailroom)
//
// TCP packets for example are cloned, but __skb_header_release()
// was called in tcp stack, allowing us to use headroom for our needs.
//
    if (!skb_header_cloned(skb) &&
    !(padlen && skb_cloned(skb)) &&
    headroom + tailroom >= 4 + padlen) {
// following should not happen, but better be safe
    if (headroom < 4 ||
    tailroom < padlen) {
    skb.data = memmove(skb.head + 4, skb.data, skb.len);
    skb_set_tail_pointer(skb, skb.len);
    }
    } else {
    struct sk_buff *skb2;
    skb2 = skb_copy_expand(skb, 4, padlen, flags);
    dev_kfree_skb_any(skb);
    skb = skb2;
    if (!skb)
    return core::ptr::null_mut();
    }
    packet_len = ((skb.len ^ 0x0000ffff) << 16) + skb.len;
    ptr = skb_push(skb, 4);
    put_unaligned_le32(packet_len, ptr);
    if (padlen) {
    put_unaligned_le32(padbytes, skb_tail_pointer(skb));
    skb_put(skb, sizeof(padbytes));
    }
    usbnet_set_skb_tx_stats(skb, 1, 0);
    return skb;
    }
#[no_mangle]
pub unsafe extern "C" fn asix_read_phy_addr(dev: *mut usbnet, internal: bool) -> c_int {
    int asix_read_phy_addr(struct usbnet *dev, bool internal)
    {
    int ret, offset;
    u8 buf[2];
    ret = asix_read_cmd(dev, AX_CMD_READ_PHY_ID, 0, 0, 2, buf, 0);
    if (ret < 0)
    goto error;
    if (ret < 2) {
    ret = -EIO;
    goto error;
    }
    offset = (internal ? 1 : 0);
    ret = buf[offset];
    if (ret >= PHY_MAX_ADDR) {
    netdev_err(dev.net, "invalid PHY address: %d\n", ret);
    return -ENODEV;
    }
    netdev_dbg(dev.net, "%s PHY address 0x%x\n",
    internal ? "internal" : "external", ret);
    return ret;
    error:
    netdev_err(dev.net, "Error reading PHY_ID register: %02x\n", ret);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn asix_sw_reset(dev: *mut usbnet, flags: u8, in_pm: c_int) -> c_int {
    int asix_sw_reset(struct usbnet *dev, u8 flags, int in_pm)
    {
    int ret;
    ret = asix_write_cmd(dev, AX_CMD_SW_RESET, flags, 0, 0, core::ptr::null_mut(), in_pm);
    if (ret < 0)
    netdev_err(dev.net, "Failed to send software reset: %02x\n", ret);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn asix_read_rx_ctl(dev: *mut usbnet, in_pm: c_int) -> u16 {
    u16 asix_read_rx_ctl(struct usbnet *dev, int in_pm)
    {
    __le16 v;
    let mut ret: c_int = asix_read_cmd(dev, AX_CMD_READ_RX_CTL, 0, 0, 2, &v, in_pm);
    if (ret < 0) {
    netdev_err(dev.net, "Error reading RX_CTL register: %02x\n", ret);
    goto out;
    }
    ret = le16_to_cpu(v);
    out:
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn asix_write_rx_ctl(dev: *mut usbnet, mode: u16, in_pm: c_int) -> c_int {
    int asix_write_rx_ctl(struct usbnet *dev, u16 mode, int in_pm)
    {
    int ret;
    netdev_dbg(dev.net, "asix_write_rx_ctl() - mode = 0x%04x\n", mode);
    ret = asix_write_cmd(dev, AX_CMD_WRITE_RX_CTL, mode, 0, 0, core::ptr::null_mut(), in_pm);
    if (ret < 0)
    netdev_err(dev.net, "Failed to write RX_CTL mode to 0x%04x: %02x\n",
    mode, ret);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn asix_read_medium_status(dev: *mut usbnet, in_pm: c_int) -> u16 {
    u16 asix_read_medium_status(struct usbnet *dev, int in_pm)
    {
    __le16 v;
    int ret = asix_read_cmd(dev, AX_CMD_READ_MEDIUM_STATUS,
    0, 0, 2, &v, in_pm);
    if (ret < 0) {
    netdev_err(dev.net, "Error reading Medium Status register: %02x\n",
    ret);
    return ret;	/* TODO: callers not checking for error ret */
    }
    return le16_to_cpu(v);
    }
#[no_mangle]
pub unsafe extern "C" fn asix_write_medium_mode(dev: *mut usbnet, mode: u16, in_pm: c_int) -> c_int {
    int asix_write_medium_mode(struct usbnet *dev, u16 mode, int in_pm)
    {
    int ret;
    netdev_dbg(dev.net, "asix_write_medium_mode() - mode = 0x%04x\n", mode);
    ret = asix_write_cmd(dev, AX_CMD_WRITE_MEDIUM_MODE,
    mode, 0, 0, core::ptr::null_mut(), in_pm);
    if (ret < 0)
    netdev_err(dev.net, "Failed to write Medium Mode mode to 0x%04x: %02x\n",
    mode, ret);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn asix_write_gpio(dev: *mut usbnet, value: u16, sleep: c_int, in_pm: c_int) -> c_int {
    int asix_write_gpio(struct usbnet *dev, u16 value, int sleep, int in_pm)
    {
    int ret;
    netdev_dbg(dev.net, "asix_write_gpio() - value = 0x%04x\n", value);
    ret = asix_write_cmd(dev, AX_CMD_WRITE_GPIOS, value, 0, 0, core::ptr::null_mut(), in_pm);
    if (ret < 0)
    netdev_err(dev.net, "Failed to write GPIO value 0x%04x: %02x\n",
    value, ret);
    if (sleep)
    msleep(sleep);
    return ret;
    }
//
// AX88772 & AX88178 have a 16-bit RX_CTL value
//
#[no_mangle]
pub unsafe extern "C" fn asix_set_multicast(net: *mut net_device) {
    void asix_set_multicast(struct net_device *net)
    {
    struct usbnet *dev = netdev_priv(net);
    struct asix_data *data = (struct asix_data *)&dev.data;
    let mut rx_ctl: u16 = AX_DEFAULT_RX_CTL;
    if (net.flags & IFF_PROMISC) {
    rx_ctl |= AX_RX_CTL_PRO;
    } else if (net.flags & IFF_ALLMULTI ||
    netdev_mc_count(net) > AX_MAX_MCAST) {
    rx_ctl |= AX_RX_CTL_AMALL;
    } else if (netdev_mc_empty(net)) {
// just broadcast and directed
    } else {
// We use the 20 byte dev->data
// for our 8 byte filter buffer
// to avoid allocating memory that
// is tricky to free later
    struct netdev_hw_addr *ha;
    u32 crc_bits;
    memset(data.multi_filter, 0, AX_MCAST_FILTER_SIZE);
// Build the multicast hash filter.
    netdev_for_each_mc_addr(ha, net) {
    crc_bits = ether_crc(ETH_ALEN, ha.addr) >> 26;
    data.multi_filter[crc_bits >> 3] |=
    1 << (crc_bits & 7);
    }
    asix_write_cmd_async(dev, AX_CMD_WRITE_MULTI_FILTER, 0, 0,
    AX_MCAST_FILTER_SIZE, data.multi_filter);
    rx_ctl |= AX_RX_CTL_AM;
    }
    asix_write_cmd_async(dev, AX_CMD_WRITE_RX_CTL, rx_ctl, 0, 0, core::ptr::null_mut());
    }
    static int __asix_mdio_read(struct net_device *netdev, int phy_id, int loc,
    bool in_pm)
    {
    struct usbnet *dev = netdev_priv(netdev);
    __le16 res;
    int ret;
    mutex_lock(&dev.phy_mutex);
    ret = asix_check_host_enable(dev, in_pm);
    if (ret == -ENODEV || ret == -ETIMEDOUT) {
    mutex_unlock(&dev.phy_mutex);
    return ret;
    }
    ret = asix_read_cmd(dev, AX_CMD_READ_MII_REG, phy_id, (__u16)loc, 2,
    &res, in_pm);
    if (ret < 0)
    goto out;
    ret = asix_set_hw_mii(dev, in_pm);
    out:
    mutex_unlock(&dev.phy_mutex);
    netdev_dbg(dev.net, "asix_mdio_read() phy_id=0x%02x, loc=0x%02x, returns=0x%04x\n",
    phy_id, loc, le16_to_cpu(res));
    return ret < 0 ? ret : le16_to_cpu(res);
    }
#[no_mangle]
pub unsafe extern "C" fn asix_mdio_read(netdev: *mut net_device, phy_id: c_int, loc: c_int) -> c_int {
    int asix_mdio_read(struct net_device *netdev, int phy_id, int loc)
    {
    return __asix_mdio_read(netdev, phy_id, loc, false);
    }
    static int __asix_mdio_write(struct net_device *netdev, int phy_id, int loc,
    int val, bool in_pm)
    {
    struct usbnet *dev = netdev_priv(netdev);
    let mut res: __le16 = cpu_to_le16(val);
    int ret;
    netdev_dbg(dev.net, "asix_mdio_write() phy_id=0x%02x, loc=0x%02x, val=0x%04x\n",
    phy_id, loc, val);
    mutex_lock(&dev.phy_mutex);
    ret = asix_check_host_enable(dev, in_pm);
    if (ret == -ENODEV)
    goto out;
    ret = asix_write_cmd(dev, AX_CMD_WRITE_MII_REG, phy_id, (__u16)loc, 2,
    &res, in_pm);
    if (ret < 0)
    goto out;
    ret = asix_set_hw_mii(dev, in_pm);
    out:
    mutex_unlock(&dev.phy_mutex);
    return ret < 0 ? ret : 0;
    }
#[no_mangle]
pub unsafe extern "C" fn asix_mdio_write(netdev: *mut net_device, phy_id: c_int, loc: c_int, val: c_int) {
    void asix_mdio_write(struct net_device *netdev, int phy_id, int loc, int val)
    {
    __asix_mdio_write(netdev, phy_id, loc, val, false);
    }
// MDIO read and write wrappers for phylib
#[no_mangle]
pub unsafe extern "C" fn asix_mdio_bus_read(bus: *mut mii_bus, phy_id: c_int, regnum: c_int) -> c_int {
    int asix_mdio_bus_read(struct mii_bus *bus, int phy_id, int regnum)
    {
    struct usbnet *priv = bus.priv;
    return __asix_mdio_read(priv.net, phy_id, regnum, false);
    }
#[no_mangle]
pub unsafe extern "C" fn asix_mdio_bus_write(bus: *mut mii_bus, phy_id: c_int, regnum: c_int, val: u16) -> c_int {
    int asix_mdio_bus_write(struct mii_bus *bus, int phy_id, int regnum, u16 val)
    {
    struct usbnet *priv = bus.priv;
    return __asix_mdio_write(priv.net, phy_id, regnum, val, false);
    }
#[no_mangle]
pub unsafe extern "C" fn asix_mdio_read_nopm(netdev: *mut net_device, phy_id: c_int, loc: c_int) -> c_int {
    int asix_mdio_read_nopm(struct net_device *netdev, int phy_id, int loc)
    {
    return __asix_mdio_read(netdev, phy_id, loc, true);
    }
    void
    asix_mdio_write_nopm(struct net_device *netdev, int phy_id, int loc, int val)
    {
    __asix_mdio_write(netdev, phy_id, loc, val, true);
    }
#[no_mangle]
pub unsafe extern "C" fn asix_get_wol(net: *mut net_device, wolinfo: *mut ethtool_wolinfo) {
    void asix_get_wol(struct net_device *net, struct ethtool_wolinfo *wolinfo)
    {
    struct usbnet *dev = netdev_priv(net);
    u8 opt;
    if (asix_read_cmd(dev, AX_CMD_READ_MONITOR_MODE,
    0, 0, 1, &opt, 0) < 0) {
    wolinfo.supported = 0;
    wolinfo.wolopts = 0;
    return;
    }
    wolinfo.supported = WAKE_PHY | WAKE_MAGIC;
    wolinfo.wolopts = 0;
    if (opt & AX_MONITOR_LINK)
    wolinfo.wolopts |= WAKE_PHY;
    if (opt & AX_MONITOR_MAGIC)
    wolinfo.wolopts |= WAKE_MAGIC;
    }
#[no_mangle]
pub unsafe extern "C" fn asix_set_wol(net: *mut net_device, wolinfo: *mut ethtool_wolinfo) -> c_int {
    int asix_set_wol(struct net_device *net, struct ethtool_wolinfo *wolinfo)
    {
    struct usbnet *dev = netdev_priv(net);
    let mut opt: u8 = 0;
    if (wolinfo.wolopts & ~(WAKE_PHY | WAKE_MAGIC))
    return -EINVAL;
    if (wolinfo.wolopts & WAKE_PHY)
    opt |= AX_MONITOR_LINK;
    if (wolinfo.wolopts & WAKE_MAGIC)
    opt |= AX_MONITOR_MAGIC;
    if (asix_write_cmd(dev, AX_CMD_WRITE_MONITOR_MODE,
    opt, 0, 0, core::ptr::null_mut(), 0) < 0)
    return -EINVAL;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn asix_get_eeprom_len(net: *mut net_device) -> c_int {
    int asix_get_eeprom_len(struct net_device *net)
    {
    return AX_EEPROM_LEN;
    }
    int asix_get_eeprom(struct net_device *net, struct ethtool_eeprom *eeprom,
    u8 *data)
    {
    struct usbnet *dev = netdev_priv(net);
    u16 *eeprom_buff;
    int first_word, last_word;
    int i;
    if (eeprom.len == 0)
    return -EINVAL;
    eeprom.magic = AX_EEPROM_MAGIC;
    first_word = eeprom.offset >> 1;
    last_word = (eeprom.offset + eeprom.len - 1) >> 1;
    eeprom_buff = kmalloc_array(last_word - first_word + 1, sizeof(u16),
    GFP_KERNEL);
    if (!eeprom_buff)
    return -ENOMEM;
// ax8817x returns 2 bytes from eeprom on read
    for (i = first_word; i <= last_word; i++) {
    if (asix_read_cmd(dev, AX_CMD_READ_EEPROM, i, 0, 2,
    &eeprom_buff[i - first_word], 0) < 0) {
    kfree(eeprom_buff);
    return -EIO;
    }
    }
    memcpy(data, (u8 *)eeprom_buff + (eeprom.offset & 1), eeprom.len);
    kfree(eeprom_buff);
    return 0;
    }
    int asix_set_eeprom(struct net_device *net, struct ethtool_eeprom *eeprom,
    u8 *data)
    {
    struct usbnet *dev = netdev_priv(net);
    u16 *eeprom_buff;
    int first_word, last_word;
    int i;
    int ret;
    netdev_dbg(net, "write EEPROM len %d, offset %d, magic 0x%x\n",
    eeprom.len, eeprom.offset, eeprom.magic);
    if (eeprom.len == 0)
    return -EINVAL;
    if (eeprom.magic != AX_EEPROM_MAGIC)
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
    ret = asix_read_cmd(dev, AX_CMD_READ_EEPROM, first_word, 0, 2,
    &eeprom_buff[0], 0);
    if (ret < 0) {
    netdev_err(net, "Failed to read EEPROM at offset 0x%02x.\n", first_word);
    goto free;
    }
    }
    if ((eeprom.offset + eeprom.len) & 1) {
    ret = asix_read_cmd(dev, AX_CMD_READ_EEPROM, last_word, 0, 2,
    &eeprom_buff[last_word - first_word], 0);
    if (ret < 0) {
    netdev_err(net, "Failed to read EEPROM at offset 0x%02x.\n", last_word);
    goto free;
    }
    }
    memcpy((u8 *)eeprom_buff + (eeprom.offset & 1), data, eeprom.len);
// write data to EEPROM
    ret = asix_write_cmd(dev, AX_CMD_WRITE_ENABLE, 0x0000, 0, 0, core::ptr::null_mut(), 0);
    if (ret < 0) {
    netdev_err(net, "Failed to enable EEPROM write\n");
    goto free;
    }
    msleep(20);
    for (i = first_word; i <= last_word; i++) {
    netdev_dbg(net, "write to EEPROM at offset 0x%02x, data 0x%04x\n",
    i, eeprom_buff[i - first_word]);
    ret = asix_write_cmd(dev, AX_CMD_WRITE_EEPROM, i,
    eeprom_buff[i - first_word], 0, core::ptr::null_mut(), 0);
    if (ret < 0) {
    netdev_err(net, "Failed to write EEPROM at offset 0x%02x.\n",
    i);
    goto free;
    }
    msleep(20);
    }
    ret = asix_write_cmd(dev, AX_CMD_WRITE_DISABLE, 0x0000, 0, 0, core::ptr::null_mut(), 0);
    if (ret < 0) {
    netdev_err(net, "Failed to disable EEPROM write\n");
    goto free;
    }
    ret = 0;
    free:
    kfree(eeprom_buff);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn asix_set_mac_address(net: *mut net_device, p: *mut c_void) -> c_int {
    int asix_set_mac_address(struct net_device *net, void *p)
    {
    struct usbnet *dev = netdev_priv(net);
    struct asix_data *data = (struct asix_data *)&dev.data;
    struct sockaddr *addr = p;
    if (netif_running(net))
    return -EBUSY;
    if (!is_valid_ether_addr(addr.sa_data))
    return -EADDRNOTAVAIL;
    eth_hw_addr_set(net, addr.sa_data);
// We use the 20 byte dev->data
// for our 6 byte mac buffer
// to avoid allocating memory that
// is tricky to free later
    memcpy(data.mac_addr, addr.sa_data, ETH_ALEN);
    asix_write_cmd_async(dev, AX_CMD_WRITE_NODE_ID, 0, 0, ETH_ALEN,
    data.mac_addr);
    return 0;
    }
