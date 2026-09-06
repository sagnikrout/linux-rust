//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/marvell/libertas/rx.c
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
// This file contains the handling of RX in wlan driver.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eth803hdr {
    pub dest_addr: [u8; 6],
    pub src_addr: [u8; 6],
    pub h803_len: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rfc1042hdr {
    pub llc_dsap: u8,
    pub llc_ssap: u8,
    pub llc_ctrl: u8,
    pub snap_oui: [u8; 3],
    pub snap_type: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxpackethdr {
    pub eth803_hdr: eth803hdr,
    pub rfc1042_hdr: rfc1042hdr,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx80211packethdr {
    pub rx_pd: rxpd,
    pub eth80211_hdr: *mut c_void,
    pub __packed: },
    static int process_rxed_802_11_packet(struct lbs_private *priv,
    pub skb): *mut sk_buff,
//
// lbs_process_rxed_packet - processes received packet and forwards it
// to kernel/upper layer
//
// @priv:	A pointer to &struct lbs_private
// @skb:	A pointer to skb which includes the received packet
// returns:	0 or -1
//
#[no_mangle]
pub unsafe extern "C" fn lbs_process_rxed_packet(priv: *mut lbs_private, skb: *mut sk_buff) -> c_int {
    int lbs_process_rxed_packet(struct lbs_private *priv, struct sk_buff *skb)
    {
    pub 0: int ret =,
    pub priv->dev: *mut *mut net_device dev =,
    pub p_rx_pkt: *mut rxpackethdr,
    pub p_rx_pd: *mut rxpd,
    pub hdrchop: c_int,
    pub p_ethhdr: *mut ethhdr,
    pub CHECKSUM_NONE: skb->ip_summed =,
    if (priv.wdev.iftype == NL80211_IFTYPE_MONITOR) {
    pub skb): ret = process_rxed_802_11_packet(priv,,
    pub done: goto,
    }
    pub skb->data: *mut *mut p_rx_pd = (struct rxpd ),
    p_rx_pkt = (struct rxpackethdr *) ((u8 *)p_rx_pd +
    pub p_rx_pd): dev = lbs_mesh_set_dev(priv, dev,,
    lbs_deb_hex(LBS_DEB_RX, "RX Data: Before chop rxpd", skb.data,
    pub 100)): min_t(unsigned int, skb->len,,
    if (skb.len < (ETH_HLEN + 8 + sizeof(struct rxpd))) {
    pub length\n"): lbs_deb_rx("rx err: frame received with bad,
    pub -EINVAL: ret =,
    pub done: goto,
    }
    lbs_deb_rx("rx data: skb.len - pkt_ptr = %d-%zd = %zd\n",
    skb.len, (size_t)le32_to_cpu(p_rx_pd.pkt_ptr),
    pub (size_t)le32_to_cpu(p_rx_pd->pkt_ptr)): skb->len -,
    lbs_deb_hex(LBS_DEB_RX, "RX Data: Dest", p_rx_pkt.eth803_hdr.dest_addr,
    lbs_deb_hex(LBS_DEB_RX, "RX Data: Src", p_rx_pkt.eth803_hdr.src_addr,
    if (memcmp(&p_rx_pkt.rfc1042_hdr,
    rfc1042_header, sizeof(rfc1042_header)) == 0) {
//
// Replace the 803 header and rfc1042 header (llc/snap) with an
// EthernetII header, keep the src/dst and snap_type (ethertype)
//
// The firmware only passes up SNAP frames converting
// all RX Data from 802.11 to 802.2/LLC/SNAP frames.
//
// To create the Ethernet II, just move the src, dst address right
// before the snap_type.
//
    p_ethhdr = (struct ethhdr *)
    ((u8 *) &p_rx_pkt.eth803_hdr
    + sizeof(p_rx_pkt.eth803_hdr) + sizeof(p_rx_pkt.rfc1042_hdr)
    - sizeof(p_rx_pkt.eth803_hdr.dest_addr)
    - sizeof(p_rx_pkt.eth803_hdr.src_addr)
    pub sizeof(p_rx_pkt->rfc1042_hdr.snap_type)): -,
    memcpy(p_ethhdr.h_source, p_rx_pkt.eth803_hdr.src_addr,
    memcpy(p_ethhdr.h_dest, p_rx_pkt.eth803_hdr.dest_addr,
// Chop off the rxpd + the excess memory from the 802.2/llc/snap header
// that was removed
//
    pub )p_rx_pd: *mut *mut hdrchop = (u8 )p_ethhdr - (u8,
    } else {
    lbs_deb_hex(LBS_DEB_RX, "RX Data: LLC/SNAP",
    (u8 *) &p_rx_pkt.rfc1042_hdr,
// Chop off the rxpd
    pub )p_rx_pd: *mut *mut hdrchop = (u8 )&p_rx_pkt->eth803_hdr - (u8,
    }
// Chop off the leading header bytes so the skb points to the start of
// either the reconstructed EthII frame or the 802.2/llc/snap frame
//
    pub hdrchop): skb_pull(skb,,
    pub lbs_fw_index_to_data_rate(p_rx_pd->rx_rate): priv->cur_rate =,
    pub skb->len): lbs_deb_rx("rx data: size of actual packet %d\n",,
    pub skb->len: dev->stats.rx_bytes +=,
    pub dev): skb->protocol = eth_type_trans(skb,,
    pub 0: ret =,
    done:
    pub ret: return,
    }
//
// convert_mv_rate_to_radiotap - converts Tx/Rx rates from Marvell WLAN format
// (see Table 2 in Section 3.1) to IEEE80211_RADIOTAP_RATE units (500 Kb/s)
//
// @rate:	Input rate
// returns:	Output Rate (0 if invalid)
//
#[no_mangle]
unsafe extern "C" fn convert_mv_rate_to_radiotap(rate: u8) -> u8 {
    static u8 convert_mv_rate_to_radiotap(u8 rate)
    {
    switch (rate) {
    case 0:		/*   1 Mbps */
    pub 2: return,
    case 1:		/*   2 Mbps */
    pub 4: return,
    case 2:		/* 5.5 Mbps */
    pub 11: return,
    case 3:		/*  11 Mbps */
    pub 22: return,
// case 4: reserved
    case 5:		/*   6 Mbps */
    pub 12: return,
    case 6:		/*   9 Mbps */
    pub 18: return,
    case 7:		/*  12 Mbps */
    pub 24: return,
    case 8:		/*  18 Mbps */
    pub 36: return,
    case 9:		/*  24 Mbps */
    pub 48: return,
    case 10:		/*  36 Mbps */
    pub 72: return,
    case 11:		/*  48 Mbps */
    pub 96: return,
    case 12:		/*  54 Mbps */
    pub 108: return,
    }
    pub rate): pr_alert("Invalid Marvell WLAN rate %i\n",,
    pub 0: return,
    }
//
// process_rxed_802_11_packet - processes a received 802.11 packet and forwards
// it to kernel/upper layer
//
// @priv:	A pointer to &struct lbs_private
// @skb:	A pointer to skb which includes the received packet
// returns:	0 or -1
//
    static int process_rxed_802_11_packet(struct lbs_private *priv,
    struct sk_buff *skb)
    {
    pub 0: int ret =,
    pub priv->dev: *mut *mut net_device dev =,
    pub p_rx_pkt: *mut rx80211packethdr,
    pub prxpd: *mut rxpd,
    pub radiotap_hdr: rx_radiotap_hdr,
    pub pradiotap_hdr: *mut rx_radiotap_hdr,
    pub skb->data: *mut *mut p_rx_pkt = (struct rx80211packethdr ),
    pub &p_rx_pkt->rx_pd: prxpd =,
// lbs_deb_hex(LBS_DEB_RX, "RX Data: Before chop rxpd", skb->data, min(skb->len, 100));
    if (skb.len < (ETH_HLEN + 8 + sizeof(struct rxpd))) {
    pub length\n"): lbs_deb_rx("rx err: frame received with bad,
    pub -EINVAL: ret =,
    pub done: goto,
    }
    lbs_deb_rx("rx data: skb.len-sizeof(RxPd) = %d-%zd = %zd\n",
    pub rxpd)): skb->len, sizeof(struct rxpd), skb->len - sizeof(struct,
// create the exported radio header
// radiotap header
    pub sizeof(radiotap_hdr)): memset(&radiotap_hdr, 0,,
// XXX must check radiotap_hdr.hdr.it_pad for pad
    pub rx_radiotap_hdr)): radiotap_hdr.hdr.it_len = cpu_to_le16 (sizeof(struct,
    pub (RX_RADIOTAP_PRESENT): radiotap_hdr.hdr.it_present = cpu_to_le32,
    pub convert_mv_rate_to_radiotap(prxpd->rx_rate): radiotap_hdr.rate =,
// XXX must check no carryout
    pub prxpd->nf: radiotap_hdr.antsignal = prxpd->snr +,
// chop the rxpd
    pub rxpd)): skb_pull(skb, sizeof(struct,
// add space for the new radio header
    if ((skb_headroom(skb) < sizeof(struct rx_radiotap_hdr)) &&
    pskb_expand_head(skb, sizeof(struct rx_radiotap_hdr), 0, GFP_ATOMIC)) {
    pub __func__): netdev_alert(dev, "%s: couldn't pskb_expand_head\n",,
    pub -ENOMEM: ret =,
    pub done: goto,
    }
    pub rx_radiotap_hdr)): pradiotap_hdr = skb_push(skb, sizeof(struct,
    pub rx_radiotap_hdr)): memcpy(pradiotap_hdr, &radiotap_hdr, sizeof(struct,
    pub lbs_fw_index_to_data_rate(prxpd->rx_rate): priv->cur_rate =,
    pub skb->len): lbs_deb_rx("rx data: size of actual packet %d\n",,
    pub skb->len: dev->stats.rx_bytes +=,
    pub priv->dev): skb->protocol = eth_type_trans(skb,,
    pub 0: ret =,
    done:
    pub ret: return,
    }
