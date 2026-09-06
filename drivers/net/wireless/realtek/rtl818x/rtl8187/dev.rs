//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/realtek/rtl818x/rtl8187/dev.c
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
// Linux device driver for RTL8187
//
// Copyright 2007 Michael Wu <flamingice@sourmilk.net>
// Copyright 2007 Andrea Merello <andrea.merello@gmail.com>
//
// Based on the r8187 driver, which is:
// Copyright 2005 Andrea Merello <andrea.merello@gmail.com>, et al.
//
// The driver was extended to the RTL8187B in 2008 by:
// Herton Ronaldo Krzesinski <herton@mandriva.com.br>
// Hin-Tak Leung <htl10@users.sourceforge.net>
// Larry Finger <Larry.Finger@lwfinger.net>
//
// Magic delays and register offsets below are taken from the original
// r8187 driver sources.  Thanks to Realtek for their support!
//

    MODULE_AUTHOR("Michael Wu <flamingice@sourmilk.net>");
    MODULE_AUTHOR("Andrea Merello <andrea.merello@gmail.com>");
    MODULE_AUTHOR("Herton Ronaldo Krzesinski <herton@mandriva.com.br>");
    MODULE_AUTHOR("Hin-Tak Leung <htl10@users.sourceforge.net>");
    MODULE_AUTHOR("Larry Finger <Larry.Finger@lwfinger.net>");
    MODULE_DESCRIPTION("RTL8187/RTL8187B USB wireless driver");
    MODULE_LICENSE("GPL");
    static const struct usb_device_id rtl8187_table[] = {
// Asus
    {USB_DEVICE(0x0b05, 0x171d), .driver_info = DEVICE_RTL8187},
// Belkin
    {USB_DEVICE(0x050d, 0x705e), .driver_info = DEVICE_RTL8187B},
// Realtek
    {USB_DEVICE(0x0bda, 0x8187), .driver_info = DEVICE_RTL8187},
    {USB_DEVICE(0x0bda, 0x8189), .driver_info = DEVICE_RTL8187B},
    {USB_DEVICE(0x0bda, 0x8197), .driver_info = DEVICE_RTL8187B},
    {USB_DEVICE(0x0bda, 0x8198), .driver_info = DEVICE_RTL8187B},
// Surecom
    {USB_DEVICE(0x0769, 0x11F2), .driver_info = DEVICE_RTL8187},
// Logitech
    {USB_DEVICE(0x0789, 0x010C), .driver_info = DEVICE_RTL8187},
// Netgear
    {USB_DEVICE(0x0846, 0x6100), .driver_info = DEVICE_RTL8187},
    {USB_DEVICE(0x0846, 0x6a00), .driver_info = DEVICE_RTL8187},
    {USB_DEVICE(0x0846, 0x4260), .driver_info = DEVICE_RTL8187B},
// HP
    {USB_DEVICE(0x03f0, 0xca02), .driver_info = DEVICE_RTL8187},
// Sitecom
    {USB_DEVICE(0x0df6, 0x000d), .driver_info = DEVICE_RTL8187},
    {USB_DEVICE(0x0df6, 0x0028), .driver_info = DEVICE_RTL8187B},
    {USB_DEVICE(0x0df6, 0x0029), .driver_info = DEVICE_RTL8187B},
// Sphairon Access Systems GmbH
    {USB_DEVICE(0x114B, 0x0150), .driver_info = DEVICE_RTL8187},
// Dick Smith Electronics
    {USB_DEVICE(0x1371, 0x9401), .driver_info = DEVICE_RTL8187},
// Abocom
    {USB_DEVICE(0x13d1, 0xabe6), .driver_info = DEVICE_RTL8187},
// Qcom
    {USB_DEVICE(0x18E8, 0x6232), .driver_info = DEVICE_RTL8187},
// AirLive
    {USB_DEVICE(0x1b75, 0x8187), .driver_info = DEVICE_RTL8187},
// Linksys
    {USB_DEVICE(0x1737, 0x0073), .driver_info = DEVICE_RTL8187B},
    {}
    };
    MODULE_DEVICE_TABLE(usb, rtl8187_table);
    static const struct ieee80211_rate rtl818x_rates[] = {
    { .bitrate = 10, .hw_value = 0, },
    { .bitrate = 20, .hw_value = 1, },
    { .bitrate = 55, .hw_value = 2, },
    { .bitrate = 110, .hw_value = 3, },
    { .bitrate = 60, .hw_value = 4, },
    { .bitrate = 90, .hw_value = 5, },
    { .bitrate = 120, .hw_value = 6, },
    { .bitrate = 180, .hw_value = 7, },
    { .bitrate = 240, .hw_value = 8, },
    { .bitrate = 360, .hw_value = 9, },
    { .bitrate = 480, .hw_value = 10, },
    { .bitrate = 540, .hw_value = 11, },
    };
    static const struct ieee80211_channel rtl818x_channels[] = {
    { .center_freq = 2412 },
    { .center_freq = 2417 },
    { .center_freq = 2422 },
    { .center_freq = 2427 },
    { .center_freq = 2432 },
    { .center_freq = 2437 },
    { .center_freq = 2442 },
    { .center_freq = 2447 },
    { .center_freq = 2452 },
    { .center_freq = 2457 },
    { .center_freq = 2462 },
    { .center_freq = 2467 },
    { .center_freq = 2472 },
    { .center_freq = 2484 },
    };
#[no_mangle]
unsafe extern "C" fn rtl8187_iowrite_async_cb(urb: *mut urb) {
    static void rtl8187_iowrite_async_cb(struct urb *urb)
    {
    kfree(urb.context);
    }
    static void rtl8187_iowrite_async(struct rtl8187_priv *priv, __le16 addr,
    void *data, u16 len)
    {
    struct usb_ctrlrequest *dr;
    struct urb *urb;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtl8187_async_write_data {
    pub data: [u8; 4],
    pub dr: usb_ctrlrequest,
    pub buf: *mut },
    pub rc: c_int,
    pub GFP_ATOMIC): *mut *mut buf = kmalloc_obj(buf,,
    if (!buf)
    pub GFP_ATOMIC): urb = usb_alloc_urb(0,,
    if (!urb) {
    }
    pub &buf->dr: dr =,
    pub RTL8187_REQT_WRITE: dr->bRequestType =,
    pub RTL8187_REQ_SET_REG: dr->bRequest =,
    pub addr: dr->wValue =,
    pub 0: dr->wIndex =,
    pub cpu_to_le16(len): dr->wLength =,
    pub len): memcpy(buf, data,,
    usb_fill_control_urb(urb, priv.udev, usb_sndctrlpipe(priv.udev, 0),
    (unsigned char *)dr, buf, len,
    pub buf): rtl8187_iowrite_async_cb,,
    pub &priv->anchored): usb_anchor_urb(urb,,
    pub GFP_ATOMIC): rc = usb_submit_urb(urb,,
    if (rc < 0) {
    }
    }
    static inline void rtl818x_iowrite32_async(struct rtl8187_priv *priv,
    __le32 *addr, u32 val)
    {
    pub cpu_to_le32(val): __le32 buf =,
    rtl8187_iowrite_async(priv, cpu_to_le16((unsigned long)addr),
    pub sizeof(buf)): &buf,,
    }
#[no_mangle]
pub unsafe extern "C" fn rtl8187_write_phy(dev: *mut ieee80211_hw, addr: u8, data: u32) {
    void rtl8187_write_phy(struct ieee80211_hw *dev, u8 addr, u32 data)
    {
    pub dev->priv: *mut *mut rtl8187_priv priv =,
    pub 8: data <<=,
    pub 0x80: data |= addr |,
    pub 0xFF): rtl818x_iowrite8(priv, &priv->map->PHY[3], (data >> 24) &,
    pub 0xFF): rtl818x_iowrite8(priv, &priv->map->PHY[2], (data >> 16) &,
    pub 0xFF): rtl818x_iowrite8(priv, &priv->map->PHY[1], (data >> 8) &,
    pub 0xFF): rtl818x_iowrite8(priv, &priv->map->PHY[0], data &,
    }
#[no_mangle]
unsafe extern "C" fn rtl8187_tx_cb(urb: *mut urb) {
    static void rtl8187_tx_cb(struct urb *urb)
    {
    pub )urb->context: *mut *mut sk_buff skb = (sk_buff,
    pub IEEE80211_SKB_CB(skb): *mut *mut ieee80211_tx_info info =,
    pub info->rate_driver_data[0]: *mut *mut ieee80211_hw hw =,
    pub hw->priv: *mut *mut rtl8187_priv priv =,
    skb_pull(skb, priv.is_rtl8187b ? sizeof(struct rtl8187b_tx_hdr) :
    pub rtl8187_tx_hdr)): sizeof(struct,
    if (!(urb.status) && !(info.flags & IEEE80211_TX_CTL_NO_ACK)) {
    if (priv.is_rtl8187b) {
    pub skb): skb_queue_tail(&priv->b_tx_status.queue,,
// queue is "full", discard last items
    while (skb_queue_len(&priv.b_tx_status.queue) > 5) {
    pub old_skb: *mut sk_buff,
    dev_dbg(&priv.udev.dev,
    pub full\n"): "transmit status queue,
    pub skb_dequeue(&priv->b_tx_status.queue): old_skb =,
    pub old_skb): ieee80211_tx_status_irqsafe(hw,,
    }
    } else {
    pub IEEE80211_TX_STAT_ACK: info->flags |=,
    }
    }
    if (priv.is_rtl8187b)
    pub skb): ieee80211_tx_status_irqsafe(hw,,
    else {
// Retry information for the RTI8187 is only available by
// reading a register in the device. We are in interrupt mode
// here, thus queue the skb and finish on a work queue.
    pub skb): skb_queue_tail(&priv->b_tx_status.queue,,
    pub 0): ieee80211_queue_delayed_work(hw, &priv->work,,
    }
    }
    static void rtl8187_tx(struct ieee80211_hw *dev,
    struct ieee80211_tx_control *control,
    struct sk_buff *skb)
    {
    pub dev->priv: *mut *mut rtl8187_priv priv =,
    pub IEEE80211_SKB_CB(skb): *mut *mut ieee80211_tx_info info =,
    pub )(skb->data): *mut *mut ieee80211_hdr tx_hdr = (ieee80211_hdr,
    pub ep: c_uint,
    pub buf: *mut c_void,
    pub urb: *mut urb,
    pub 0: __le16 rts_dur =,
    pub flags: u32,
    pub rc: c_int,
    pub GFP_ATOMIC): urb = usb_alloc_urb(0,,
    if (!urb) {
    }
    pub skb->len: flags =,
    pub RTL818X_TX_DESC_FLAG_NO_ENC: flags |=,
    pub 24: flags |= ieee80211_get_tx_rate(dev, info)->hw_value <<,
    if (ieee80211_has_morefrags(tx_hdr.frame_control))
    pub RTL818X_TX_DESC_FLAG_MOREFRAG: flags |=,
// HW will perform RTS-CTS when only RTS flags is set.
// HW will perform CTS-to-self when both RTS and CTS flags are set.
// RTS rate and RTS duration will be used also for CTS-to-self.
//
    if (info.control.rates[0].flags & IEEE80211_TX_RC_USE_RTS_CTS) {
    pub RTL818X_TX_DESC_FLAG_RTS: flags |=,
    pub 19: flags |= ieee80211_get_rts_cts_rate(dev, info)->hw_value <<,
    rts_dur = ieee80211_rts_duration(dev, priv.vif,
    pub info): skb->len,,
    } else if (info.control.rates[0].flags & IEEE80211_TX_RC_USE_CTS_PROTECT) {
    pub RTL818X_TX_DESC_FLAG_CTS: flags |= RTL818X_TX_DESC_FLAG_RTS |,
    pub 19: flags |= ieee80211_get_rts_cts_rate(dev, info)->hw_value <<,
    rts_dur = ieee80211_ctstoself_duration(dev, priv.vif,
    pub info): skb->len,,
    }
    if (info.flags & IEEE80211_TX_CTL_ASSIGN_SEQ) {
    if (info.flags & IEEE80211_TX_CTL_FIRST_FRAGMENT)
    pub 0x10: priv->seqno +=,
    pub cpu_to_le16(IEEE80211_SCTL_FRAG): tx_hdr->seq_ctrl &=,
    pub cpu_to_le16(priv->seqno): tx_hdr->seq_ctrl |=,
    }
    if (!priv.is_rtl8187b) {
    pub sizeof(*hdr)): *mut *mut rtl8187_tx_hdr hdr = skb_push(skb,,
    pub cpu_to_le32(flags): hdr->flags =,
    pub 0: hdr->len =,
    pub rts_dur: hdr->rts_duration =,
    pub 8): hdr->retry = cpu_to_le32((info->control.rates[0].count - 1) <<,
    pub hdr: buf =,
    pub 2: ep =,
    } else {
// fc needs to be calculated before skb_push()
    pub }: unsigned int epmap[4] = { 6, 7, 5, 4,
    pub le16_to_cpu(tx_hdr->frame_control): u16 fc =,
    pub sizeof(*hdr)): *mut *mut rtl8187b_tx_hdr hdr = skb_push(skb,,
    struct ieee80211_rate *txrate =
    pub info): ieee80211_get_tx_rate(dev,,
    pub sizeof(*hdr)): *mut memset(hdr, 0,,
    pub cpu_to_le32(flags): hdr->flags =,
    pub rts_dur: hdr->rts_duration =,
    pub 8): hdr->retry = cpu_to_le32((info->control.rates[0].count - 1) <<,
    hdr.tx_duration =
    ieee80211_generic_frame_duration(dev, priv.vif,
    info.band,
    pub txrate): skb->len,,
    pub hdr: buf =,
    if ((fc & IEEE80211_FCTL_FTYPE) == IEEE80211_FTYPE_MGMT)
    pub 12: ep =,
    else
    pub epmap: [ep =; skb_get_queue_mapping(skb)],
    }
    pub dev: info->rate_driver_data[0] =,
    pub urb: info->rate_driver_data[1] =,
    usb_fill_bulk_urb(urb, priv.udev, usb_sndbulkpipe(priv.udev, ep),
    pub skb): buf, skb->len, rtl8187_tx_cb,,
    pub URB_ZERO_PACKET: urb->transfer_flags |=,
    pub &priv->anchored): usb_anchor_urb(urb,,
    pub GFP_ATOMIC): rc = usb_submit_urb(urb,,
    if (rc < 0) {
    }
    }
#[no_mangle]
unsafe extern "C" fn rtl8187_rx_cb(urb: *mut urb) {
    static void rtl8187_rx_cb(struct urb *urb)
    {
    pub )urb->context: *mut *mut sk_buff skb = (sk_buff,
    pub )skb->cb: *mut *mut rtl8187_rx_info info = (rtl8187_rx_info,
    pub info->dev: *mut *mut ieee80211_hw dev =,
    pub dev->priv: *mut *mut rtl8187_priv priv =,
    pub }: ieee80211_rx_status rx_status = { 0,
    pub signal: int rate,,
    pub flags: u32,
    pub f: c_ulong,
    pub f): spin_lock_irqsave(&priv->rx_queue.lock,,
    pub &priv->rx_queue): __skb_unlink(skb,,
    pub f): spin_unlock_irqrestore(&priv->rx_queue.lock,,
    pub urb->actual_length): skb_put(skb,,
    if (unlikely(urb.status))
    pub free_skb: goto,
    if (!priv.is_rtl8187b) {
    pub hdr: *mut rtl8187_rx_hdr,
    if (skb.len < sizeof(struct rtl8187_rx_hdr))
    pub free_skb: goto,
    pub sizeof(*hdr)): *mut hdr = (typeof(hdr))(skb_tail_pointer(skb) -,
    pub le32_to_cpu(hdr->flags): flags =,
// As with the RTL8187B below, the AGC is used to calculate
// signal strength. In this case, the scaling
// constants are derived from the output of p54usb.
//
    pub 6): *mut *mut signal = -4 - ((27  hdr->agc) >>,
    pub 1: rx_status.antenna = (hdr->signal >> 7) &,
    pub le64_to_cpu(hdr->mac_time): rx_status.mactime =,
    } else {
    pub hdr: *mut rtl8187b_rx_hdr,
    if (skb.len < sizeof(struct rtl8187b_rx_hdr))
    pub free_skb: goto,
    pub sizeof(*hdr)): *mut hdr = (typeof(hdr))(skb_tail_pointer(skb) -,
// The Realtek datasheet for the RTL8187B shows that the RX
// header contains the following quantities: signal quality,
// RSSI, AGC, the received power in dB, and the measured SNR.
// In testing, none of these quantities show qualitative
// agreement with AP signal strength, except for the AGC,
// which is inversely proportional to the strength of the
// signal. In the following, the signal strength
// is derived from the AGC. The arbitrary scaling constants
// are chosen to make the results close to the values obtained
// for a BCM4312 using b43 as the driver. The noise is ignored
// for now.
//
    pub le32_to_cpu(hdr->flags): flags =,
    pub 2: signal = 14 - hdr->agc /,
    pub 1: rx_status.antenna = (hdr->rssi >> 7) &,
    pub le64_to_cpu(hdr->mac_time): rx_status.mactime =,
    }
    pub signal: rx_status.signal =,
    pub signal: priv->signal =,
    pub 0xF: rate = (flags >> 20) &,
    pub 0x0FFF): skb_trim(skb, flags &,
    pub rate: rx_status.rate_idx =,
    pub dev->conf.chandef.chan->center_freq: rx_status.freq =,
    pub dev->conf.chandef.chan->band: rx_status.band =,
    pub RX_FLAG_MACTIME_START: rx_status.flag |=,
    if (flags & RTL818X_RX_DESC_FLAG_SPLCP)
    pub RX_ENC_FLAG_SHORTPRE: rx_status.enc_flags |=,
    if (flags & RTL818X_RX_DESC_FLAG_CRC32_ERR)
    pub RX_FLAG_FAILED_FCS_CRC: rx_status.flag |=,
    pub sizeof(rx_status)): memcpy(IEEE80211_SKB_RXCB(skb), &rx_status,,
    pub skb): ieee80211_rx_irqsafe(dev,,
    pub dev_alloc_skb(RTL8187_MAX_RX): skb =,
    if (unlikely(!skb)) {
// TODO check rx queue length and refill *somewhere*
    }
    pub )skb->cb: *mut info = (struct rtl8187_rx_info,
    pub urb: info->urb =,
    pub dev: info->dev =,
    pub skb_tail_pointer(skb): urb->transfer_buffer =,
    pub skb: urb->context =,
    pub skb): skb_queue_tail(&priv->rx_queue,,
    pub &priv->anchored): usb_anchor_urb(urb,,
    if (usb_submit_urb(urb, GFP_ATOMIC)) {
    pub &priv->rx_queue): skb_unlink(skb,,
    }
    free_skb:
    }
#[no_mangle]
unsafe extern "C" fn rtl8187_init_urbs(dev: *mut ieee80211_hw) -> c_int {
    static int rtl8187_init_urbs(struct ieee80211_hw *dev)
    {
    pub dev->priv: *mut *mut rtl8187_priv priv =,
    pub NULL: *mut *mut urb entry =,
    pub skb: *mut sk_buff,
    pub info: *mut rtl8187_rx_info,
    pub 0: int ret =,
    while (skb_queue_len(&priv.rx_queue) < 32) {
    pub GFP_KERNEL): skb = __dev_alloc_skb(RTL8187_MAX_RX,,
    if (!skb) {
    pub -ENOMEM: ret =,
    pub err: goto,
    }
    pub GFP_KERNEL): entry = usb_alloc_urb(0,,
    if (!entry) {
    pub -ENOMEM: ret =,
    pub err: goto,
    }
    usb_fill_bulk_urb(entry, priv.udev,
    usb_rcvbulkpipe(priv.udev,
    priv.is_rtl8187b ? 3 : 1),
    skb_tail_pointer(skb),
    pub skb): RTL8187_MAX_RX, rtl8187_rx_cb,,
    pub )skb->cb: *mut info = (struct rtl8187_rx_info,
    pub entry: info->urb =,
    pub dev: info->dev =,
    pub skb): skb_queue_tail(&priv->rx_queue,,
    pub &priv->anchored): usb_anchor_urb(entry,,
    pub GFP_KERNEL): ret = usb_submit_urb(entry,,
    if (ret) {
    pub &priv->rx_queue): skb_unlink(skb,,
    pub err: goto,
    }
    }
    pub ret: return,
    err:
    pub ret: return,
    }
#[no_mangle]
unsafe extern "C" fn rtl8187b_status_cb(urb: *mut urb) {
    static void rtl8187b_status_cb(struct urb *urb)
    {
    pub )urb->context: *mut *mut ieee80211_hw hw = (ieee80211_hw,
    pub hw->priv: *mut *mut rtl8187_priv priv =,
    pub val: u64,
    pub cmd_type: c_uint,
    if (unlikely(urb.status))
//
// Read from status buffer:
//
// bits [30:31] = cmd type:
// - 0 indicates tx beacon interrupt
// - 1 indicates tx close descriptor
//
// In the case of tx beacon interrupt:
// [0:9] = Last Beacon CW
// [10:29] = reserved
// [30:31] = 00b
// [32:63] = Last Beacon TSF
//
// If it's tx close descriptor:
// [0:7] = Packet Retry Count
// [8:14] = RTS Retry Count
// [15] = TOK
// [16:27] = Sequence No
// [28] = LS
// [29] = FS
// [30:31] = 01b
// [32:47] = unused (reserved?)
// [48:63] = MAC Used Time
//
    pub le64_to_cpu(priv->b_tx_status.buf): val =,
    pub 0x3: cmd_type = (val >> 30) &,
    if (cmd_type == 1) {
    pub seq_no: unsigned int pkt_rc,,
    pub tok: bool,
    pub iter: *mut *mut sk_buff skb,,
    pub ieee80211hdr: *mut ieee80211_hdr,
    pub flags: c_ulong,
    pub 0xFF: pkt_rc = val &,
    pub 15): tok = val & (1 <<,
    pub 0xFFF: seq_no = (val >> 16) &,
    pub flags): spin_lock_irqsave(&priv->b_tx_status.queue.lock,,
    pub NULL: skb =,
    skb_queue_reverse_walk(&priv.b_tx_status.queue, iter) {
    pub )iter->data: *mut ieee80211hdr = (struct ieee80211_hdr,
//
// While testing, it was discovered that the seq_no
// doesn't actually contains the sequence number.
// Instead of returning just the 12 bits of sequence
// number, hardware is returning entire sequence control
// (fragment number plus sequence number) in a 12 bit
// only field overflowing after some time. As a
// workaround, just consider the lower bits, and expect
// it's unlikely we wrongly ack some sent data
//
    if ((le16_to_cpu(ieee80211hdr.seq_ctrl)
    & 0xFFF) == seq_no) {
    pub iter: skb =,
    }
    }
    if (skb) {
    pub IEEE80211_SKB_CB(skb): *mut *mut ieee80211_tx_info info =,
    pub &priv->b_tx_status.queue): __skb_unlink(skb,,
    if (tok)
    pub IEEE80211_TX_STAT_ACK: info->flags |=,
    pub 1: info->status.rates[0].count = pkt_rc +,
    pub skb): ieee80211_tx_status_irqsafe(hw,,
    }
    pub flags): spin_unlock_irqrestore(&priv->b_tx_status.queue.lock,,
    }
    pub &priv->anchored): usb_anchor_urb(urb,,
    if (usb_submit_urb(urb, GFP_ATOMIC))
    }
#[no_mangle]
unsafe extern "C" fn rtl8187b_init_status_urb(dev: *mut ieee80211_hw) -> c_int {
    static int rtl8187b_init_status_urb(struct ieee80211_hw *dev)
    {
    pub dev->priv: *mut *mut rtl8187_priv priv =,
    pub entry: *mut urb,
    pub 0: int ret =,
    pub GFP_KERNEL): entry = usb_alloc_urb(0,,
    if (!entry)
    pub -ENOMEM: return,
    usb_fill_bulk_urb(entry, priv.udev, usb_rcvbulkpipe(priv.udev, 9),
    &priv.b_tx_status.buf, sizeof(priv.b_tx_status.buf),
    pub dev): rtl8187b_status_cb,,
    pub &priv->anchored): usb_anchor_urb(entry,,
    pub GFP_KERNEL): ret = usb_submit_urb(entry,,
    if (ret)
    pub ret: return,
    }
#[no_mangle]
unsafe extern "C" fn rtl8187_set_anaparam(priv: *mut rtl8187_priv, rfon: bool) {
    static void rtl8187_set_anaparam(struct rtl8187_priv *priv, bool rfon)
    {
    pub anaparam2: u32 anaparam,,
    pub reg: u8 anaparam3,,
    if (!priv.is_rtl8187b) {
    if (rfon) {
    pub RTL8187_RTL8225_ANAPARAM_ON: anaparam =,
    pub RTL8187_RTL8225_ANAPARAM2_ON: anaparam2 =,
    } else {
    pub RTL8187_RTL8225_ANAPARAM_OFF: anaparam =,
    pub RTL8187_RTL8225_ANAPARAM2_OFF: anaparam2 =,
    }
    } else {
    if (rfon) {
    pub RTL8187B_RTL8225_ANAPARAM_ON: anaparam =,
    pub RTL8187B_RTL8225_ANAPARAM2_ON: anaparam2 =,
    pub RTL8187B_RTL8225_ANAPARAM3_ON: anaparam3 =,
    } else {
    pub RTL8187B_RTL8225_ANAPARAM_OFF: anaparam =,
    pub RTL8187B_RTL8225_ANAPARAM2_OFF: anaparam2 =,
    pub RTL8187B_RTL8225_ANAPARAM3_OFF: anaparam3 =,
    }
    }
    rtl818x_iowrite8(priv, &priv.map.EEPROM_CMD,
    pub &priv->map->CONFIG3): reg = rtl818x_ioread8(priv,,
    pub RTL818X_CONFIG3_ANAPARAM_WRITE: reg |=,
    pub reg): rtl818x_iowrite8(priv, &priv->map->CONFIG3,,
    pub anaparam): rtl818x_iowrite32(priv, &priv->map->ANAPARAM,,
    pub anaparam2): rtl818x_iowrite32(priv, &priv->map->ANAPARAM2,,
    if (priv.is_rtl8187b)
    pub anaparam3): rtl818x_iowrite8(priv, &priv->map->ANAPARAM3A,,
    pub ~RTL818X_CONFIG3_ANAPARAM_WRITE: reg &=,
    pub reg): rtl818x_iowrite8(priv, &priv->map->CONFIG3,,
    rtl818x_iowrite8(priv, &priv.map.EEPROM_CMD,
    }
#[no_mangle]
unsafe extern "C" fn rtl8187_cmd_reset(dev: *mut ieee80211_hw) -> c_int {
    static int rtl8187_cmd_reset(struct ieee80211_hw *dev)
    {
    pub dev->priv: *mut *mut rtl8187_priv priv =,
    pub reg: u8,
    pub i: c_int,
    pub &priv->map->CMD): reg = rtl818x_ioread8(priv,,
    pub 1): reg &= (1 <<,
    pub RTL818X_CMD_RESET: reg |=,
    pub reg): rtl818x_iowrite8(priv, &priv->map->CMD,,
    pub 10: i =,
    do {
    if (!(rtl818x_ioread8(priv, &priv.map.CMD) &
    RTL818X_CMD_RESET))
    pub (--i): } while,
    if (!i) {
    pub timeout!\n"): wiphy_err(dev->wiphy, "Reset,
    pub -ETIMEDOUT: return,
    }
// reload registers from eeprom
    pub RTL818X_EEPROM_CMD_LOAD): rtl818x_iowrite8(priv, &priv->map->EEPROM_CMD,,
    pub 10: i =,
    do {
    if (!(rtl818x_ioread8(priv, &priv.map.EEPROM_CMD) &
    RTL818X_EEPROM_CMD_CONFIG))
    pub (--i): } while,
    if (!i) {
    pub timeout!\n"): wiphy_err(dev->wiphy, "eeprom reset,
    pub -ETIMEDOUT: return,
    }
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn rtl8187_init_hw(dev: *mut ieee80211_hw) -> c_int {
    static int rtl8187_init_hw(struct ieee80211_hw *dev)
    {
    pub dev->priv: *mut *mut rtl8187_priv priv =,
    pub reg: u8,
    pub res: c_int,
// reset
    pub true): rtl8187_set_anaparam(priv,,
    pub 0): rtl818x_iowrite16(priv, &priv->map->INT_MASK,,
    pub 0x10): *mut *mut rtl818x_iowrite8(priv, (u8 )0xFE18,,
    pub 0x11): *mut *mut rtl818x_iowrite8(priv, (u8 )0xFE18,,
    pub 0x00): *mut *mut rtl818x_iowrite8(priv, (u8 )0xFE18,,
    pub rtl8187_cmd_reset(dev): res =,
    if (res)
    pub res: return,
    pub true): rtl8187_set_anaparam(priv,,
// setup card
    pub 0): rtl818x_iowrite16(priv, &priv->map->RFPinsSelect,,
    pub 0): rtl818x_iowrite8(priv, &priv->map->GPIO0,,
    pub 8)): rtl818x_iowrite16(priv, &priv->map->RFPinsSelect, (4 <<,
    pub 1): rtl818x_iowrite8(priv, &priv->map->GPIO0,,
    pub 0): rtl818x_iowrite8(priv, &priv->map->GP_ENABLE,,
    pub RTL818X_EEPROM_CMD_CONFIG): rtl818x_iowrite8(priv, &priv->map->EEPROM_CMD,,
    pub 0xFFFF): *mut *mut rtl818x_iowrite16(priv, (__le16 )0xFFF4,,
    pub &priv->map->CONFIG1): reg = rtl818x_ioread8(priv,,
    pub 0x3F: reg &=,
    pub 0x80: reg |=,
    pub reg): rtl818x_iowrite8(priv, &priv->map->CONFIG1,,
    pub RTL818X_EEPROM_CMD_NORMAL): rtl818x_iowrite8(priv, &priv->map->EEPROM_CMD,,
    pub 0): rtl818x_iowrite32(priv, &priv->map->INT_TIMEOUT,,
    pub 0): rtl818x_iowrite8(priv, &priv->map->WPA_CONF,,
    pub 0): rtl818x_iowrite8(priv, &priv->map->RATE_FALLBACK,,
// TODO: set RESP_RATE and BRSR properly
    pub 0): rtl818x_iowrite8(priv, &priv->map->RESP_RATE, (8 << 4) |,
    pub 0x01F3): rtl818x_iowrite16(priv, &priv->map->BRSR,,
// host_usb_init
    pub 0): rtl818x_iowrite16(priv, &priv->map->RFPinsSelect,,
    pub 0): rtl818x_iowrite8(priv, &priv->map->GPIO0,,
    pub )0xFE53): *mut reg = rtl818x_ioread8(priv, (u8,
    pub 7)): *mut *mut rtl818x_iowrite8(priv, (u8 )0xFE53, reg | (1 <<,
    pub 8)): rtl818x_iowrite16(priv, &priv->map->RFPinsSelect, (4 <<,
    pub 0x20): rtl818x_iowrite8(priv, &priv->map->GPIO0,,
    pub 0): rtl818x_iowrite8(priv, &priv->map->GP_ENABLE,,
    pub 0x80): rtl818x_iowrite16(priv, &priv->map->RFPinsOutput,,
    pub 0x80): rtl818x_iowrite16(priv, &priv->map->RFPinsSelect,,
    pub 0x80): rtl818x_iowrite16(priv, &priv->map->RFPinsEnable,,
    pub 0x000a8008): rtl818x_iowrite32(priv, &priv->map->RF_TIMING,,
    pub 0xFFFF): rtl818x_iowrite16(priv, &priv->map->BRSR,,
    pub 0x00100044): rtl818x_iowrite32(priv, &priv->map->RF_PARA,,
    rtl818x_iowrite8(priv, &priv.map.EEPROM_CMD,
    pub 0x44): rtl818x_iowrite8(priv, &priv->map->CONFIG3,,
    rtl818x_iowrite8(priv, &priv.map.EEPROM_CMD,
    pub 0x1FF7): rtl818x_iowrite16(priv, &priv->map->RFPinsEnable,,
    pub 0x01F3): rtl818x_iowrite16(priv, &priv->map->BRSR,,
    pub ~1: reg = rtl818x_ioread8(priv, &priv->map->PGSELECT) &,
    pub 1): rtl818x_iowrite8(priv, &priv->map->PGSELECT, reg |,
    pub 0x10): *mut *mut rtl818x_iowrite16(priv, (__le16 )0xFFFE,,
    pub 0x80): rtl818x_iowrite8(priv, &priv->map->TALLY_SEL,,
    pub 0x60): *mut *mut rtl818x_iowrite8(priv, (u8 )0xFFFF,,
    pub reg): rtl818x_iowrite8(priv, &priv->map->PGSELECT,,
    pub 0: return,
    }
    static const u8 rtl8187b_reg_table[][3] = {
    {0xF0, 0x32, 0}, {0xF1, 0x32, 0}, {0xF2, 0x00, 0}, {0xF3, 0x00, 0},
    {0xF4, 0x32, 0}, {0xF5, 0x43, 0}, {0xF6, 0x00, 0}, {0xF7, 0x00, 0},
    {0xF8, 0x46, 0}, {0xF9, 0xA4, 0}, {0xFA, 0x00, 0}, {0xFB, 0x00, 0},
    {0xFC, 0x96, 0}, {0xFD, 0xA4, 0}, {0xFE, 0x00, 0}, {0xFF, 0x00, 0},
    {0x58, 0x4B, 1}, {0x59, 0x00, 1}, {0x5A, 0x4B, 1}, {0x5B, 0x00, 1},
    {0x60, 0x4B, 1}, {0x61, 0x09, 1}, {0x62, 0x4B, 1}, {0x63, 0x09, 1},
    {0xCE, 0x0F, 1}, {0xCF, 0x00, 1}, {0xF0, 0x4E, 1}, {0xF1, 0x01, 1},
    {0xF2, 0x02, 1}, {0xF3, 0x03, 1}, {0xF4, 0x04, 1}, {0xF5, 0x05, 1},
    {0xF6, 0x06, 1}, {0xF7, 0x07, 1}, {0xF8, 0x08, 1},
    {0x4E, 0x00, 2}, {0x0C, 0x04, 2}, {0x21, 0x61, 2}, {0x22, 0x68, 2},
    {0x23, 0x6F, 2}, {0x24, 0x76, 2}, {0x25, 0x7D, 2}, {0x26, 0x84, 2},
    {0x27, 0x8D, 2}, {0x4D, 0x08, 2}, {0x50, 0x05, 2}, {0x51, 0xF5, 2},
    {0x52, 0x04, 2}, {0x53, 0xA0, 2}, {0x54, 0x1F, 2}, {0x55, 0x23, 2},
    {0x56, 0x45, 2}, {0x57, 0x67, 2}, {0x58, 0x08, 2}, {0x59, 0x08, 2},
    {0x5A, 0x08, 2}, {0x5B, 0x08, 2}, {0x60, 0x08, 2}, {0x61, 0x08, 2},
    {0x62, 0x08, 2}, {0x63, 0x08, 2}, {0x64, 0xCF, 2},
    {0x5B, 0x40, 0}, {0x84, 0x88, 0}, {0x85, 0x24, 0}, {0x88, 0x54, 0},
    {0x8B, 0xB8, 0}, {0x8C, 0x07, 0}, {0x8D, 0x00, 0}, {0x94, 0x1B, 0},
    {0x95, 0x12, 0}, {0x96, 0x00, 0}, {0x97, 0x06, 0}, {0x9D, 0x1A, 0},
    {0x9F, 0x10, 0}, {0xB4, 0x22, 0}, {0xBE, 0x80, 0}, {0xDB, 0x00, 0},
    {0xEE, 0x00, 0}, {0x4C, 0x00, 2},
    {0x9F, 0x00, 3}, {0x8C, 0x01, 0}, {0x8D, 0x10, 0}, {0x8E, 0x08, 0},
    {0x8F, 0x00, 0}
}

#[no_mangle]
unsafe extern "C" fn rtl8187b_init_hw(dev: *mut ieee80211_hw) -> c_int {
    static int rtl8187b_init_hw(struct ieee80211_hw *dev)
    {
    struct rtl8187_priv *priv = dev.priv;
    int res, i;
    u8 reg;
    rtl8187_set_anaparam(priv, true);
// Reset PLL sequence on 8187B. Realtek note: reduces power
// consumption about 30 mA
    rtl818x_iowrite8(priv, (u8 *)0xFF61, 0x10);
    reg = rtl818x_ioread8(priv, (u8 *)0xFF62);
    rtl818x_iowrite8(priv, (u8 *)0xFF62, reg & ~(1 << 5));
    rtl818x_iowrite8(priv, (u8 *)0xFF62, reg | (1 << 5));
    res = rtl8187_cmd_reset(dev);
    if (res)
    return res;
    rtl8187_set_anaparam(priv, true);
// BRSR (Basic Rate Set Register) on 8187B looks to be the same as
// RESP_RATE on 8187L in Realtek sources: each bit should be each
// one of the 12 rates, all are enabled
    rtl818x_iowrite16(priv, (__le16 *)0xFF34, 0x0FFF);
    reg = rtl818x_ioread8(priv, &priv.map.CW_CONF);
    reg |= RTL818X_CW_CONF_PERPACKET_RETRY;
    rtl818x_iowrite8(priv, &priv.map.CW_CONF, reg);
// Auto Rate Fallback Register (ARFR): 1M-54M setting
    rtl818x_iowrite16_idx(priv, (__le16 *)0xFFE0, 0x0FFF, 1);
    rtl818x_iowrite8_idx(priv, (u8 *)0xFFE2, 0x00, 1);
    rtl818x_iowrite16_idx(priv, (__le16 *)0xFFD4, 0xFFFF, 1);
    rtl818x_iowrite8(priv, &priv.map.EEPROM_CMD,
    RTL818X_EEPROM_CMD_CONFIG);
    reg = rtl818x_ioread8(priv, &priv.map.CONFIG1);
    rtl818x_iowrite8(priv, &priv.map.CONFIG1, (reg & 0x3F) | 0x80);
    rtl818x_iowrite8(priv, &priv.map.EEPROM_CMD,
    RTL818X_EEPROM_CMD_NORMAL);
    rtl818x_iowrite8(priv, &priv.map.WPA_CONF, 0);
    for (i = 0; i < ARRAY_SIZE(rtl8187b_reg_table); i++) {
    rtl818x_iowrite8_idx(priv,
    (u8 *)(uintptr_t)
    (rtl8187b_reg_table[i][0] | 0xFF00),
    rtl8187b_reg_table[i][1],
    rtl8187b_reg_table[i][2]);
    }
    rtl818x_iowrite16(priv, &priv.map.TID_AC_MAP, 0xFA50);
    rtl818x_iowrite16(priv, &priv.map.INT_MIG, 0);
    rtl818x_iowrite32_idx(priv, (__le32 *)0xFFF0, 0, 1);
    rtl818x_iowrite32_idx(priv, (__le32 *)0xFFF4, 0, 1);
    rtl818x_iowrite8_idx(priv, (u8 *)0xFFF8, 0, 1);
    rtl818x_iowrite32(priv, &priv.map.RF_TIMING, 0x00004001);
// RFSW_CTRL register
    rtl818x_iowrite16_idx(priv, (__le16 *)0xFF72, 0x569A, 2);
    rtl818x_iowrite16(priv, &priv.map.RFPinsOutput, 0x0480);
    rtl818x_iowrite16(priv, &priv.map.RFPinsSelect, 0x2488);
    rtl818x_iowrite16(priv, &priv.map.RFPinsEnable, 0x1FFF);
    msleep(100);
    priv.rf.init(dev);
    reg = RTL818X_CMD_TX_ENABLE | RTL818X_CMD_RX_ENABLE;
    rtl818x_iowrite8(priv, &priv.map.CMD, reg);
    rtl818x_iowrite16(priv, &priv.map.INT_MASK, 0xFFFF);
    rtl818x_iowrite8(priv, (u8 *)0xFE41, 0xF4);
    rtl818x_iowrite8(priv, (u8 *)0xFE40, 0x00);
    rtl818x_iowrite8(priv, (u8 *)0xFE42, 0x00);
    rtl818x_iowrite8(priv, (u8 *)0xFE42, 0x01);
    rtl818x_iowrite8(priv, (u8 *)0xFE40, 0x0F);
    rtl818x_iowrite8(priv, (u8 *)0xFE42, 0x00);
    rtl818x_iowrite8(priv, (u8 *)0xFE42, 0x01);
    reg = rtl818x_ioread8(priv, (u8 *)0xFFDB);
    rtl818x_iowrite8(priv, (u8 *)0xFFDB, reg | (1 << 2));
    rtl818x_iowrite16_idx(priv, (__le16 *)0xFF72, 0x59FA, 3);
    rtl818x_iowrite16_idx(priv, (__le16 *)0xFF74, 0x59D2, 3);
    rtl818x_iowrite16_idx(priv, (__le16 *)0xFF76, 0x59D2, 3);
    rtl818x_iowrite16_idx(priv, (__le16 *)0xFF78, 0x19FA, 3);
    rtl818x_iowrite16_idx(priv, (__le16 *)0xFF7A, 0x19FA, 3);
    rtl818x_iowrite16_idx(priv, (__le16 *)0xFF7C, 0x00D0, 3);
    rtl818x_iowrite8(priv, (u8 *)0xFF61, 0);
    rtl818x_iowrite8_idx(priv, (u8 *)0xFF80, 0x0F, 1);
    rtl818x_iowrite8_idx(priv, (u8 *)0xFF83, 0x03, 1);
    rtl818x_iowrite8(priv, (u8 *)0xFFDA, 0x10);
    rtl818x_iowrite8_idx(priv, (u8 *)0xFF4D, 0x08, 2);
    rtl818x_iowrite32(priv, &priv.map.HSSI_PARA, 0x0600321B);
    rtl818x_iowrite16_idx(priv, (__le16 *)0xFFEC, 0x0800, 1);
    priv.slot_time = 0x9;
    priv.aifsn[0] = 2; /* AIFSN[AC_VO] */
    priv.aifsn[1] = 2; /* AIFSN[AC_VI] */
    priv.aifsn[2] = 7; /* AIFSN[AC_BK] */
    priv.aifsn[3] = 3; /* AIFSN[AC_BE] */
    rtl818x_iowrite8(priv, &priv.map.ACM_CONTROL, 0);
// ENEDCA flag must always be set, transmit issues?
    rtl818x_iowrite8(priv, &priv.map.MSR, RTL818X_MSR_ENEDCA);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rtl8187_work(work: *mut work_struct) {
    static void rtl8187_work(struct work_struct *work)
    {
// The RTL8187 returns the retry count through register 0xFFFA. In
// addition, it appears to be a cumulative retry count, not the
// value for the current TX packet. When multiple TX entries are
// waiting in the queue, the retry count will be the total for all.
// The "error" may matter for purposes of rate setting, but there is
// no other choice with this hardware.
//
    struct rtl8187_priv *priv = container_of(work, struct rtl8187_priv,
    work.work);
    struct ieee80211_tx_info *info;
    struct ieee80211_hw *dev = priv.dev;
    static u16 retry;
    u16 tmp;
    u16 avg_retry;
    int length;
    mutex_lock(&priv.conf_mutex);
    tmp = rtl818x_ioread16(priv, (__le16 *)0xFFFA);
    length = skb_queue_len(&priv.b_tx_status.queue);
    if (unlikely(!length))
    length = 1;
    if (unlikely(tmp < retry))
    tmp = retry;
    avg_retry = (tmp - retry) / length;
    while (skb_queue_len(&priv.b_tx_status.queue) > 0) {
    struct sk_buff *old_skb;
    old_skb = skb_dequeue(&priv.b_tx_status.queue);
    info = IEEE80211_SKB_CB(old_skb);
    info.status.rates[0].count = avg_retry + 1;
    if (info.status.rates[0].count > RETRY_COUNT)
    info.flags &= ~IEEE80211_TX_STAT_ACK;
    ieee80211_tx_status_irqsafe(dev, old_skb);
    }
    retry = tmp;
    mutex_unlock(&priv.conf_mutex);
    }
#[no_mangle]
unsafe extern "C" fn rtl8187_start(dev: *mut ieee80211_hw) -> c_int {
    static int rtl8187_start(struct ieee80211_hw *dev)
    {
    struct rtl8187_priv *priv = dev.priv;
    u32 reg;
    int ret;
    mutex_lock(&priv.conf_mutex);
    ret = (!priv.is_rtl8187b) ? rtl8187_init_hw(dev) :
    rtl8187b_init_hw(dev);
    if (ret)
    goto rtl8187_start_exit;
    init_usb_anchor(&priv.anchored);
    priv.dev = dev;
    if (priv.is_rtl8187b) {
    reg = RTL818X_RX_CONF_MGMT |
    RTL818X_RX_CONF_DATA |
    RTL818X_RX_CONF_BROADCAST |
    RTL818X_RX_CONF_NICMAC |
    RTL818X_RX_CONF_BSSID |
    (7 << 13 /* RX FIFO threshold NONE */) |
    (7 << 10 /* MAX RX DMA */) |
    RTL818X_RX_CONF_RX_AUTORESETPHY |
    RTL818X_RX_CONF_ONLYERLPKT;
    priv.rx_conf = reg;
    rtl818x_iowrite32(priv, &priv.map.RX_CONF, reg);
    reg = rtl818x_ioread8(priv, &priv.map.TX_AGC_CTL);
    reg &= ~RTL818X_TX_AGC_CTL_PERPACKET_GAIN;
    reg &= ~RTL818X_TX_AGC_CTL_PERPACKET_ANTSEL;
    reg &= ~RTL818X_TX_AGC_CTL_FEEDBACK_ANT;
    rtl818x_iowrite8(priv, &priv.map.TX_AGC_CTL, reg);
    rtl818x_iowrite32(priv, &priv.map.TX_CONF,
    RTL818X_TX_CONF_HW_SEQNUM |
    RTL818X_TX_CONF_DISREQQSIZE |
    (RETRY_COUNT << 8  /* short retry limit */) |
    (RETRY_COUNT << 0  /* long retry limit */) |
    (7 << 21 /* MAX TX DMA */));
    ret = rtl8187_init_urbs(dev);
    if (ret)
    goto rtl8187_start_exit;
    ret = rtl8187b_init_status_urb(dev);
    if (ret)
    usb_kill_anchored_urbs(&priv.anchored);
    goto rtl8187_start_exit;
    }
    rtl818x_iowrite16(priv, &priv.map.INT_MASK, 0xFFFF);
    rtl818x_iowrite32(priv, &priv.map.MAR[0], ~0);
    rtl818x_iowrite32(priv, &priv.map.MAR[1], ~0);
    ret = rtl8187_init_urbs(dev);
    if (ret)
    goto rtl8187_start_exit;
    reg = RTL818X_RX_CONF_ONLYERLPKT |
    RTL818X_RX_CONF_RX_AUTORESETPHY |
    RTL818X_RX_CONF_BSSID |
    RTL818X_RX_CONF_MGMT |
    RTL818X_RX_CONF_DATA |
    (7 << 13 /* RX FIFO threshold NONE */) |
    (7 << 10 /* MAX RX DMA */) |
    RTL818X_RX_CONF_BROADCAST |
    RTL818X_RX_CONF_NICMAC;
    priv.rx_conf = reg;
    rtl818x_iowrite32(priv, &priv.map.RX_CONF, reg);
    reg = rtl818x_ioread8(priv, &priv.map.CW_CONF);
    reg &= ~RTL818X_CW_CONF_PERPACKET_CW;
    reg |= RTL818X_CW_CONF_PERPACKET_RETRY;
    rtl818x_iowrite8(priv, &priv.map.CW_CONF, reg);
    reg = rtl818x_ioread8(priv, &priv.map.TX_AGC_CTL);
    reg &= ~RTL818X_TX_AGC_CTL_PERPACKET_GAIN;
    reg &= ~RTL818X_TX_AGC_CTL_PERPACKET_ANTSEL;
    reg &= ~RTL818X_TX_AGC_CTL_FEEDBACK_ANT;
    rtl818x_iowrite8(priv, &priv.map.TX_AGC_CTL, reg);
    reg  = RTL818X_TX_CONF_CW_MIN |
    (7 << 21 /* MAX TX DMA */) |
    RTL818X_TX_CONF_NO_ICV;
    rtl818x_iowrite32(priv, &priv.map.TX_CONF, reg);
    reg = rtl818x_ioread8(priv, &priv.map.CMD);
    reg |= RTL818X_CMD_TX_ENABLE;
    reg |= RTL818X_CMD_RX_ENABLE;
    rtl818x_iowrite8(priv, &priv.map.CMD, reg);
    INIT_DELAYED_WORK(&priv.work, rtl8187_work);
    rtl8187_start_exit:
    mutex_unlock(&priv.conf_mutex);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rtl8187_stop(dev: *mut ieee80211_hw, suspend: bool) {
    static void rtl8187_stop(struct ieee80211_hw *dev, bool suspend)
    {
    struct rtl8187_priv *priv = dev.priv;
    struct sk_buff *skb;
    u32 reg;
    mutex_lock(&priv.conf_mutex);
    rtl818x_iowrite16(priv, &priv.map.INT_MASK, 0);
    reg = rtl818x_ioread8(priv, &priv.map.CMD);
    reg &= ~RTL818X_CMD_TX_ENABLE;
    reg &= ~RTL818X_CMD_RX_ENABLE;
    rtl818x_iowrite8(priv, &priv.map.CMD, reg);
    priv.rf.stop(dev);
    rtl8187_set_anaparam(priv, false);
    rtl818x_iowrite8(priv, &priv.map.EEPROM_CMD, RTL818X_EEPROM_CMD_CONFIG);
    reg = rtl818x_ioread8(priv, &priv.map.CONFIG4);
    rtl818x_iowrite8(priv, &priv.map.CONFIG4, reg | RTL818X_CONFIG4_VCOOFF);
    rtl818x_iowrite8(priv, &priv.map.EEPROM_CMD, RTL818X_EEPROM_CMD_NORMAL);
    usb_kill_anchored_urbs(&priv.anchored);
    while ((skb = skb_dequeue(&priv.b_tx_status.queue)))
    dev_kfree_skb_any(skb);
    mutex_unlock(&priv.conf_mutex);
    if (!priv.is_rtl8187b)
    cancel_delayed_work_sync(&priv.work);
    }
#[no_mangle]
unsafe extern "C" fn rtl8187_get_tsf(dev: *mut ieee80211_hw, vif: *mut ieee80211_vif) -> u64 {
    static u64 rtl8187_get_tsf(struct ieee80211_hw *dev, struct ieee80211_vif *vif)
    {
    struct rtl8187_priv *priv = dev.priv;
    return rtl818x_ioread32(priv, &priv.map.TSFT[0]) |
    (u64)(rtl818x_ioread32(priv, &priv.map.TSFT[1])) << 32;
    }
#[no_mangle]
unsafe extern "C" fn rtl8187_beacon_work(work: *mut work_struct) {
    static void rtl8187_beacon_work(struct work_struct *work)
    {
    struct rtl8187_vif *vif_priv =
    container_of(work, struct rtl8187_vif, beacon_work.work);
    struct ieee80211_vif *vif =
    container_of((void *)vif_priv, struct ieee80211_vif, drv_priv);
    struct ieee80211_hw *dev = vif_priv.dev;
    struct ieee80211_mgmt *mgmt;
    struct sk_buff *skb;
// don't overflow the tx ring
    if (ieee80211_queue_stopped(dev, 0))
    goto resched;
// grab a fresh beacon
    skb = ieee80211_beacon_get(dev, vif, 0);
    if (!skb)
    goto resched;
//
// update beacon timestamp w/ TSF value
// TODO: make hardware update beacon timestamp
//
    mgmt = (struct ieee80211_mgmt *)skb.data;
    mgmt.u.beacon.timestamp = cpu_to_le64(rtl8187_get_tsf(dev, vif));
// TODO: use actual beacon queue
    skb_set_queue_mapping(skb, 0);
    rtl8187_tx(dev, core::ptr::null_mut(), skb);
    resched:
//
// schedule next beacon
// TODO: use hardware support for beacon timing
//
    schedule_delayed_work(&vif_priv.beacon_work,
    usecs_to_jiffies(1024 * vif.bss_conf.beacon_int));
    }
    static int rtl8187_add_interface(struct ieee80211_hw *dev,
    struct ieee80211_vif *vif)
    {
    struct rtl8187_priv *priv = dev.priv;
    struct rtl8187_vif *vif_priv;
    int i;
    let mut ret: c_int = -EOPNOTSUPP;
    mutex_lock(&priv.conf_mutex);
    if (priv.vif)
    goto exit;
    switch (vif.type) {
    case NL80211_IFTYPE_STATION:
    case NL80211_IFTYPE_ADHOC:
    break;
    default:
    goto exit;
    }
    ret = 0;
    priv.vif = vif;
// Initialize driver private area
    vif_priv = (struct rtl8187_vif *)&vif.drv_priv;
    vif_priv.dev = dev;
    INIT_DELAYED_WORK(&vif_priv.beacon_work, rtl8187_beacon_work);
    vif_priv.enable_beacon = false;
    rtl818x_iowrite8(priv, &priv.map.EEPROM_CMD, RTL818X_EEPROM_CMD_CONFIG);
    for (i = 0; i < ETH_ALEN; i++)
    rtl818x_iowrite8(priv, &priv.map.MAC[i],
    ((u8 *)vif.addr)[i]);
    rtl818x_iowrite8(priv, &priv.map.EEPROM_CMD, RTL818X_EEPROM_CMD_NORMAL);
    exit:
    mutex_unlock(&priv.conf_mutex);
    return ret;
    }
    static void rtl8187_remove_interface(struct ieee80211_hw *dev,
    struct ieee80211_vif *vif)
    {
    struct rtl8187_priv *priv = dev.priv;
    mutex_lock(&priv.conf_mutex);
    priv.vif = core::ptr::null_mut();
    mutex_unlock(&priv.conf_mutex);
    }
#[no_mangle]
unsafe extern "C" fn rtl8187_config(dev: *mut ieee80211_hw, radio_idx: c_int, changed: u32) -> c_int {
    static int rtl8187_config(struct ieee80211_hw *dev, int radio_idx, u32 changed)
    {
    struct rtl8187_priv *priv = dev.priv;
    struct ieee80211_conf *conf = &dev.conf;
    u32 reg;
    mutex_lock(&priv.conf_mutex);
    reg = rtl818x_ioread32(priv, &priv.map.TX_CONF);
// Enable TX loopback on MAC level to avoid TX during channel
// changes, as this has be seen to causes problems and the
// card will stop work until next reset
//
    rtl818x_iowrite32(priv, &priv.map.TX_CONF,
    reg | RTL818X_TX_CONF_LOOPBACK_MAC);
    priv.rf.set_chan(dev, conf);
    msleep(10);
    rtl818x_iowrite32(priv, &priv.map.TX_CONF, reg);
    rtl818x_iowrite16(priv, &priv.map.ATIM_WND, 2);
    rtl818x_iowrite16(priv, &priv.map.ATIMTR_INTERVAL, 100);
    rtl818x_iowrite16(priv, &priv.map.BEACON_INTERVAL, 100);
    rtl818x_iowrite16(priv, &priv.map.BEACON_INTERVAL_TIME, 100);
    mutex_unlock(&priv.conf_mutex);
    return 0;
    }
//
// With 8187B, AC_*_PARAM clashes with FEMR definition in struct rtl818x_csr for
// example. Thus we have to use raw values for AC_*_PARAM register addresses.
//
    static __le32 *rtl8187b_ac_addr[4] = {
    (__le32 *) 0xFFF0, /* AC_VO */
    (__le32 *) 0xFFF4, /* AC_VI */
    (__le32 *) 0xFFFC, /* AC_BK */
    (__le32 *) 0xFFF8, /* AC_BE */
    };
pub const SIFS_TIME: c_uint = 0xa;
    static void rtl8187_conf_erp(struct rtl8187_priv *priv, bool use_short_slot,
    bool use_short_preamble)
    {
    if (priv.is_rtl8187b) {
    u8 difs, eifs;
    u16 ack_timeout;
    int queue;
    if (use_short_slot) {
    priv.slot_time = 0x9;
    difs = 0x1c;
    eifs = 0x53;
    } else {
    priv.slot_time = 0x14;
    difs = 0x32;
    eifs = 0x5b;
    }
    rtl818x_iowrite8(priv, &priv.map.SIFS, 0x22);
    rtl818x_iowrite8(priv, &priv.map.SLOT, priv.slot_time);
    rtl818x_iowrite8(priv, &priv.map.DIFS, difs);
//
// BRSR+1 on 8187B is in fact EIFS register
// Value in units of 4 us
//
    rtl818x_iowrite8(priv, (u8 *)&priv.map.BRSR + 1, eifs);
//
// For 8187B, CARRIER_SENSE_COUNTER is in fact ack timeout
// register. In units of 4 us like eifs register
// ack_timeout = ack duration + plcp + difs + preamble
//
    ack_timeout = 112 + 48 + difs;
    if (use_short_preamble)
    ack_timeout += 72;
    else
    ack_timeout += 144;
    rtl818x_iowrite8(priv, &priv.map.CARRIER_SENSE_COUNTER,
    DIV_ROUND_UP(ack_timeout, 4));
    for (queue = 0; queue < 4; queue++)
    rtl818x_iowrite8(priv, (u8 *) rtl8187b_ac_addr[queue],
    priv.aifsn[queue] * priv.slot_time +
    SIFS_TIME);
    } else {
    rtl818x_iowrite8(priv, &priv.map.SIFS, 0x22);
    if (use_short_slot) {
    rtl818x_iowrite8(priv, &priv.map.SLOT, 0x9);
    rtl818x_iowrite8(priv, &priv.map.DIFS, 0x14);
    rtl818x_iowrite8(priv, &priv.map.EIFS, 91 - 0x14);
    } else {
    rtl818x_iowrite8(priv, &priv.map.SLOT, 0x14);
    rtl818x_iowrite8(priv, &priv.map.DIFS, 0x24);
    rtl818x_iowrite8(priv, &priv.map.EIFS, 91 - 0x24);
    }
    }
    }
    static void rtl8187_bss_info_changed(struct ieee80211_hw *dev,
    struct ieee80211_vif *vif,
    struct ieee80211_bss_conf *info,
    u64 changed)
    {
    struct rtl8187_priv *priv = dev.priv;
    struct rtl8187_vif *vif_priv;
    int i;
    u8 reg;
    vif_priv = (struct rtl8187_vif *)&vif.drv_priv;
    if (changed & BSS_CHANGED_BSSID) {
    mutex_lock(&priv.conf_mutex);
    for (i = 0; i < ETH_ALEN; i++)
    rtl818x_iowrite8(priv, &priv.map.BSSID[i],
    info.bssid[i]);
    if (priv.is_rtl8187b)
    reg = RTL818X_MSR_ENEDCA;
    else
    reg = 0;
    if (is_valid_ether_addr(info.bssid)) {
    if (vif.type == NL80211_IFTYPE_ADHOC)
    reg |= RTL818X_MSR_ADHOC;
    else
    reg |= RTL818X_MSR_INFRA;
    }
    else
    reg |= RTL818X_MSR_NO_LINK;
    rtl818x_iowrite8(priv, &priv.map.MSR, reg);
    mutex_unlock(&priv.conf_mutex);
    }
    if (changed & (BSS_CHANGED_ERP_SLOT | BSS_CHANGED_ERP_PREAMBLE))
    rtl8187_conf_erp(priv, info.use_short_slot,
    info.use_short_preamble);
    if (changed & BSS_CHANGED_BEACON_ENABLED)
    vif_priv.enable_beacon = info.enable_beacon;
    if (changed & (BSS_CHANGED_BEACON_ENABLED | BSS_CHANGED_BEACON)) {
    cancel_delayed_work_sync(&vif_priv.beacon_work);
    if (vif_priv.enable_beacon)
    schedule_work(&vif_priv.beacon_work.work);
    }
    }
    static u64 rtl8187_prepare_multicast(struct ieee80211_hw *dev,
    struct netdev_hw_addr_list *mc_list)
    {
    return netdev_hw_addr_list_count(mc_list);
    }
    static void rtl8187_configure_filter(struct ieee80211_hw *dev,
    unsigned int changed_flags,
    unsigned int *total_flags,
    u64 multicast)
    {
    struct rtl8187_priv *priv = dev.priv;
    if (changed_flags & FIF_FCSFAIL)
    priv.rx_conf ^= RTL818X_RX_CONF_FCS;
    if (changed_flags & FIF_CONTROL)
    priv.rx_conf ^= RTL818X_RX_CONF_CTRL;
    if (*total_flags & FIF_OTHER_BSS ||
// total_flags & FIF_ALLMULTI || multicast > 0)
    priv.rx_conf |= RTL818X_RX_CONF_MONITOR;
    else
    priv.rx_conf &= ~RTL818X_RX_CONF_MONITOR;
// total_flags = 0;
    if (priv.rx_conf & RTL818X_RX_CONF_FCS)
// total_flags |= FIF_FCSFAIL;
    if (priv.rx_conf & RTL818X_RX_CONF_CTRL)
// total_flags |= FIF_CONTROL;
    if (priv.rx_conf & RTL818X_RX_CONF_MONITOR) {
// total_flags |= FIF_OTHER_BSS;
// total_flags |= FIF_ALLMULTI;
    }
    rtl818x_iowrite32_async(priv, &priv.map.RX_CONF, priv.rx_conf);
    }
    static int rtl8187_conf_tx(struct ieee80211_hw *dev,
    struct ieee80211_vif *vif,
    unsigned int link_id, u16 queue,
    const struct ieee80211_tx_queue_params *params)
    {
    struct rtl8187_priv *priv = dev.priv;
    u8 cw_min, cw_max;
    if (queue > 3)
    return -EINVAL;
    cw_min = fls(params.cw_min);
    cw_max = fls(params.cw_max);
    if (priv.is_rtl8187b) {
    priv.aifsn[queue] = params.aifs;
//
// This is the structure of AC_*_PARAM registers in 8187B:
// - TXOP limit field, bit offset = 16
// - ECWmax, bit offset = 12
// - ECWmin, bit offset = 8
// - AIFS, bit offset = 0
//
    rtl818x_iowrite32(priv, rtl8187b_ac_addr[queue],
    (params.txop << 16) | (cw_max << 12) |
    (cw_min << 8) | (params.aifs *
    priv.slot_time + SIFS_TIME));
    } else {
    if (queue != 0)
    return -EINVAL;
    rtl818x_iowrite8(priv, &priv.map.CW_VAL,
    cw_min | (cw_max << 4));
    }
    return 0;
    }
    static const struct ieee80211_ops rtl8187_ops = {
    .add_chanctx = ieee80211_emulate_add_chanctx,
    .remove_chanctx = ieee80211_emulate_remove_chanctx,
    .change_chanctx = ieee80211_emulate_change_chanctx,
    .switch_vif_chanctx = ieee80211_emulate_switch_vif_chanctx,
    .tx			= rtl8187_tx,
    .wake_tx_queue		= ieee80211_handle_wake_tx_queue,
    .start			= rtl8187_start,
    .stop			= rtl8187_stop,
    .add_interface		= rtl8187_add_interface,
    .remove_interface	= rtl8187_remove_interface,
    .config			= rtl8187_config,
    .bss_info_changed	= rtl8187_bss_info_changed,
    .prepare_multicast	= rtl8187_prepare_multicast,
    .configure_filter	= rtl8187_configure_filter,
    .conf_tx		= rtl8187_conf_tx,
    .rfkill_poll		= rtl8187_rfkill_poll,
    .get_tsf		= rtl8187_get_tsf,
    };
#[no_mangle]
unsafe extern "C" fn rtl8187_eeprom_register_read(eeprom: *mut eeprom_93cx6) {
    static void rtl8187_eeprom_register_read(struct eeprom_93cx6 *eeprom)
    {
    struct ieee80211_hw *dev = eeprom.data;
    struct rtl8187_priv *priv = dev.priv;
    let mut reg: u8 = rtl818x_ioread8(priv, &priv.map.EEPROM_CMD);
    eeprom.reg_data_in = reg & RTL818X_EEPROM_CMD_WRITE;
    eeprom.reg_data_out = reg & RTL818X_EEPROM_CMD_READ;
    eeprom.reg_data_clock = reg & RTL818X_EEPROM_CMD_CK;
    eeprom.reg_chip_select = reg & RTL818X_EEPROM_CMD_CS;
    }
#[no_mangle]
unsafe extern "C" fn rtl8187_eeprom_register_write(eeprom: *mut eeprom_93cx6) {
    static void rtl8187_eeprom_register_write(struct eeprom_93cx6 *eeprom)
    {
    struct ieee80211_hw *dev = eeprom.data;
    struct rtl8187_priv *priv = dev.priv;
    let mut reg: u8 = RTL818X_EEPROM_CMD_PROGRAM;
    if (eeprom.reg_data_in)
    reg |= RTL818X_EEPROM_CMD_WRITE;
    if (eeprom.reg_data_out)
    reg |= RTL818X_EEPROM_CMD_READ;
    if (eeprom.reg_data_clock)
    reg |= RTL818X_EEPROM_CMD_CK;
    if (eeprom.reg_chip_select)
    reg |= RTL818X_EEPROM_CMD_CS;
    rtl818x_iowrite8(priv, &priv.map.EEPROM_CMD, reg);
    udelay(10);
    }
    static int rtl8187_probe(struct usb_interface *intf,
    const struct usb_device_id *id)
    {
    struct usb_device *udev = interface_to_usbdev(intf);
    struct ieee80211_hw *dev;
    struct rtl8187_priv *priv;
    let mut eeprom: eeprom_93cx6 = {};
    struct ieee80211_channel *channel;
    const char *chip_name;
    u16 txpwr, reg;
    let mut product_id: u16 = le16_to_cpu(udev.descriptor.idProduct);
    int err, i;
    u8 mac_addr[ETH_ALEN];
    dev = ieee80211_alloc_hw(sizeof(*priv), &rtl8187_ops);
    if (!dev) {
    printk(KERN_ERR "rtl8187: ieee80211 alloc failed\n");
    return -ENOMEM;
    }
    priv = dev.priv;
    priv.is_rtl8187b = (id.driver_info == DEVICE_RTL8187B);
// allocate "DMA aware" buffer for register accesses
    priv.io_dmabuf = kmalloc_obj(*priv.io_dmabuf);
    if (!priv.io_dmabuf) {
    err = -ENOMEM;
    goto err_free_dev;
    }
    mutex_init(&priv.io_mutex);
    mutex_init(&priv.conf_mutex);
    SET_IEEE80211_DEV(dev, &intf.dev);
    usb_set_intfdata(intf, dev);
    priv.udev = udev;
    skb_queue_head_init(&priv.rx_queue);
    BUILD_BUG_ON(sizeof(priv.channels) != sizeof(rtl818x_channels));
    BUILD_BUG_ON(sizeof(priv.rates) != sizeof(rtl818x_rates));
    memcpy(priv.channels, rtl818x_channels, sizeof(rtl818x_channels));
    memcpy(priv.rates, rtl818x_rates, sizeof(rtl818x_rates));
    priv.map = (struct rtl818x_csr *)0xFF00;
    priv.band.band = NL80211_BAND_2GHZ;
    priv.band.channels = priv.channels;
    priv.band.n_channels = ARRAY_SIZE(rtl818x_channels);
    priv.band.bitrates = priv.rates;
    priv.band.n_bitrates = ARRAY_SIZE(rtl818x_rates);
    dev.wiphy.bands[NL80211_BAND_2GHZ] = &priv.band;
    ieee80211_hw_set(dev, RX_INCLUDES_FCS);
    ieee80211_hw_set(dev, HOST_BROADCAST_PS_BUFFERING);
    ieee80211_hw_set(dev, SIGNAL_DBM);
// Initialize rate-control variables
    dev.max_rates = 1;
    dev.max_rate_tries = RETRY_COUNT;
    eeprom.data = dev;
    eeprom.register_read = rtl8187_eeprom_register_read;
    eeprom.register_write = rtl8187_eeprom_register_write;
    if (rtl818x_ioread32(priv, &priv.map.RX_CONF) & (1 << 6))
    eeprom.width = PCI_EEPROM_WIDTH_93C66;
    else
    eeprom.width = PCI_EEPROM_WIDTH_93C46;
    rtl818x_iowrite8(priv, &priv.map.EEPROM_CMD, RTL818X_EEPROM_CMD_CONFIG);
    udelay(10);
    eeprom_93cx6_multiread(&eeprom, RTL8187_EEPROM_MAC_ADDR,
    (__le16  *)mac_addr, 3);
    if (!is_valid_ether_addr(mac_addr)) {
    printk(KERN_WARNING "rtl8187: Invalid hwaddr! Using randomly "
    "generated MAC address\n");
    eth_random_addr(mac_addr);
    }
    SET_IEEE80211_PERM_ADDR(dev, mac_addr);
    channel = priv.channels;
    for (i = 0; i < 3; i++) {
    eeprom_93cx6_read(&eeprom, RTL8187_EEPROM_TXPWR_CHAN_1 + i,
    &txpwr);
    (*channel++).hw_value = txpwr & 0xFF;
    (*channel++).hw_value = txpwr >> 8;
    }
    for (i = 0; i < 2; i++) {
    eeprom_93cx6_read(&eeprom, RTL8187_EEPROM_TXPWR_CHAN_4 + i,
    &txpwr);
    (*channel++).hw_value = txpwr & 0xFF;
    (*channel++).hw_value = txpwr >> 8;
    }
    eeprom_93cx6_read(&eeprom, RTL8187_EEPROM_TXPWR_BASE,
    &priv.txpwr_base);
    reg = rtl818x_ioread8(priv, &priv.map.PGSELECT) & ~1;
    rtl818x_iowrite8(priv, &priv.map.PGSELECT, reg | 1);
// 0 means asic B-cut, we should use SW 3 wire
// bit-by-bit banging for radio. 1 means we can use
// USB specific request to write radio registers
    priv.asic_rev = rtl818x_ioread8(priv, (u8 *)0xFFFE) & 0x3;
    rtl818x_iowrite8(priv, &priv.map.PGSELECT, reg);
    rtl818x_iowrite8(priv, &priv.map.EEPROM_CMD, RTL818X_EEPROM_CMD_NORMAL);
    if (!priv.is_rtl8187b) {
    u32 reg32;
    reg32 = rtl818x_ioread32(priv, &priv.map.TX_CONF);
    reg32 &= RTL818X_TX_CONF_HWVER_MASK;
    switch (reg32) {
    case RTL818X_TX_CONF_R8187vD_B:
// Some RTL8187B devices have a USB ID of 0x8187
// detect them here
    chip_name = "RTL8187BvB(early)";
    priv.is_rtl8187b = 1;
    priv.hw_rev = RTL8187BvB;
    break;
    case RTL818X_TX_CONF_R8187vD:
    chip_name = "RTL8187vD";
    break;
    default:
    chip_name = "RTL8187vB (default)";
    }
    } else {
//
// Force USB request to write radio registers for 8187B, Realtek
// only uses it in their sources
//
// if (priv->asic_rev == 0) {
    printk(KERN_WARNING "rtl8187: Forcing use of USB "
    "requests to write to radio registers\n");
    priv.asic_rev = 1;
    }*/
    switch (rtl818x_ioread8(priv, (u8 *)0xFFE1)) {
    case RTL818X_R8187B_B:
    chip_name = "RTL8187BvB";
    priv.hw_rev = RTL8187BvB;
    break;
    case RTL818X_R8187B_D:
    chip_name = "RTL8187BvD";
    priv.hw_rev = RTL8187BvD;
    break;
    case RTL818X_R8187B_E:
    chip_name = "RTL8187BvE";
    priv.hw_rev = RTL8187BvE;
    break;
    default:
    chip_name = "RTL8187BvB (default)";
    priv.hw_rev = RTL8187BvB;
    }
    }
    if (!priv.is_rtl8187b) {
    for (i = 0; i < 2; i++) {
    eeprom_93cx6_read(&eeprom,
    RTL8187_EEPROM_TXPWR_CHAN_6 + i,
    &txpwr);
    (*channel++).hw_value = txpwr & 0xFF;
    (*channel++).hw_value = txpwr >> 8;
    }
    } else {
    eeprom_93cx6_read(&eeprom, RTL8187_EEPROM_TXPWR_CHAN_6,
    &txpwr);
    (*channel++).hw_value = txpwr & 0xFF;
    eeprom_93cx6_read(&eeprom, 0x0A, &txpwr);
    (*channel++).hw_value = txpwr & 0xFF;
    eeprom_93cx6_read(&eeprom, 0x1C, &txpwr);
    (*channel++).hw_value = txpwr & 0xFF;
    (*channel++).hw_value = txpwr >> 8;
    }
// Handle the differing rfkill GPIO bit in different models
    priv.rfkill_mask = RFKILL_MASK_8187_89_97;
    if (product_id == 0x8197 || product_id == 0x8198) {
    eeprom_93cx6_read(&eeprom, RTL8187_EEPROM_SELECT_GPIO, &reg);
    if (reg & 0xFF00)
    priv.rfkill_mask = RFKILL_MASK_8198;
    }
    dev.vif_data_size = sizeof(struct rtl8187_vif);
    dev.wiphy.interface_modes = BIT(NL80211_IFTYPE_STATION) |
    BIT(NL80211_IFTYPE_ADHOC) ;
    wiphy_ext_feature_set(dev.wiphy, NL80211_EXT_FEATURE_CQM_RSSI_LIST);
    if ((id.driver_info == DEVICE_RTL8187) && priv.is_rtl8187b)
    printk(KERN_INFO "rtl8187: inconsistency between id with OEM"
    " info!\n");
    priv.rf = rtl8187_detect_rf(dev);
    dev.extra_tx_headroom = (!priv.is_rtl8187b) ?
    sizeof(struct rtl8187_tx_hdr) :
    sizeof(struct rtl8187b_tx_hdr);
    if (!priv.is_rtl8187b)
    dev.queues = 1;
    else
    dev.queues = 4;
    err = ieee80211_register_hw(dev);
    if (err) {
    printk(KERN_ERR "rtl8187: Cannot register device\n");
    goto err_free_dmabuf;
    }
    skb_queue_head_init(&priv.b_tx_status.queue);
    wiphy_info(dev.wiphy, "hwaddr %pM, %s V%d + %s, rfkill mask %d\n",
    mac_addr, chip_name, priv.asic_rev, priv.rf.name,
    priv.rfkill_mask);

    eeprom_93cx6_read(&eeprom, 0x3F, &reg);
    reg &= 0xFF;
    rtl8187_leds_init(dev, reg);

    rtl8187_rfkill_init(dev);
    return 0;
    err_free_dmabuf:
    kfree(priv.io_dmabuf);
    usb_set_intfdata(intf, core::ptr::null_mut());
    err_free_dev:
    ieee80211_free_hw(dev);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn rtl8187_disconnect(intf: *mut usb_interface) {
    static void rtl8187_disconnect(struct usb_interface *intf)
    {
    struct ieee80211_hw *dev = usb_get_intfdata(intf);
    struct rtl8187_priv *priv;
    if (!dev)
    return;

    rtl8187_leds_exit(dev);

    rtl8187_rfkill_exit(dev);
    ieee80211_unregister_hw(dev);
    priv = dev.priv;
    usb_reset_device(priv.udev);
    kfree(priv.io_dmabuf);
    ieee80211_free_hw(dev);
    }
    static struct usb_driver rtl8187_driver = {
    .name		= KBUILD_MODNAME,
    .id_table	= rtl8187_table,
    .probe		= rtl8187_probe,
    .disconnect	= rtl8187_disconnect,
    .disable_hub_initiated_lpm = 1,
    };
    module_usb_driver(rtl8187_driver);
