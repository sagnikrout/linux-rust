//! Automatically rewritten from C to Rust
//! Source: net/llc/llc_input.c
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
// llc_input.c - Minimal input path for LLC
//
// Copyright (c) 1997 by Procom Technology, Inc.
// 2001-2003 by Arnaldo Carvalho de Melo <acme@conectiva.com.br>
//

// Macro flag: #define dprintk(args...)

//
// Packet handler for the station, registerable because in the minimal
// LLC core that is taking shape only the very minimal subset of LLC that
// is needed for things like IPX, Appletalk, etc will stay, with all the
// rest in the llc1 and llc2 modules.
//
    static void (*llc_station_handler)(struct sk_buff *skb);
//
// Packet handlers for LLC_DEST_SAP and LLC_DEST_CONN.
//
    static void (*llc_type_handlers[2])(struct llc_sap *sap,
    struct sk_buff *skb);
    void llc_add_pack(int type, void (*handler)(struct llc_sap *sap,
    struct sk_buff *skb))
    {
    smp_wmb(); /* ensure initialisation is complete before it's called */
    if (type == LLC_DEST_SAP || type == LLC_DEST_CONN)
    llc_type_handlers[type - 1] = handler;
    }
#[no_mangle]
pub unsafe extern "C" fn llc_remove_pack(type: c_int) {
    void llc_remove_pack(int type)
    {
    if (type == LLC_DEST_SAP || type == LLC_DEST_CONN)
    llc_type_handlers[type - 1] = core::ptr::null_mut();
    synchronize_net();
    }
#[no_mangle]
pub unsafe extern "C" fn llc_set_station_handler(skb): *mut *mut void (handler)(struct sk_buff) {
    void llc_set_station_handler(void (*handler)(struct sk_buff *skb))
    {
// Ensure initialisation is complete before it's called
    if (handler)
    smp_wmb();
    llc_station_handler = handler;
    if (!handler)
    synchronize_net();
    }
//
// llc_pdu_type - returns which LLC component must handle for PDU
// @skb: input skb
//
// This function returns which LLC component must handle this PDU.
//
#[no_mangle]
unsafe extern "C" fn llc_pdu_type(skb: *mut sk_buff) -> __inline__ int {
    static __inline__ int llc_pdu_type(struct sk_buff *skb)
    {
    int type = LLC_DEST_CONN; /* I-PDU or S-PDU type */
    struct llc_pdu_sn *pdu = llc_pdu_sn_hdr(skb);
    if ((pdu.ctrl_1 & LLC_PDU_TYPE_MASK) != LLC_PDU_TYPE_U)
    goto out;
    switch (LLC_U_PDU_CMD(pdu)) {
    case LLC_1_PDU_CMD_XID:
    case LLC_1_PDU_CMD_UI:
    case LLC_1_PDU_CMD_TEST:
    type = LLC_DEST_SAP;
    break;
    case LLC_2_PDU_CMD_SABME:
    case LLC_2_PDU_CMD_DISC:
    case LLC_2_PDU_RSP_UA:
    case LLC_2_PDU_RSP_DM:
    case LLC_2_PDU_RSP_FRMR:
    break;
    default:
    type = LLC_DEST_INVALID;
    break;
    }
    out:
    return type;
    }
//
// llc_fixup_skb - initializes skb pointers
// @skb: This argument points to incoming skb
//
// Initializes internal skb pointer to start of network layer by deriving
// length of LLC header; finds length of LLC control field in LLC header
// by looking at the two lowest-order bits of the first control field
// byte; field is either 3 or 4 bytes long.
//
#[no_mangle]
pub unsafe extern "C" fn llc_fixup_skb(skb: *mut sk_buff) -> c_int {
    static inline int llc_fixup_skb(struct sk_buff *skb)
    {
    let mut llc_len: u8 = 2;
    struct llc_pdu_un *pdu;
    if (unlikely(!pskb_may_pull(skb, sizeof(*pdu))))
    return 0;
    pdu = (struct llc_pdu_un *)skb.data;
    if ((pdu.ctrl_1 & LLC_PDU_TYPE_MASK) == LLC_PDU_TYPE_U)
    llc_len = 1;
    llc_len += 2;
    if (unlikely(!pskb_may_pull(skb, llc_len)))
    return 0;
    skb_pull(skb, llc_len);
    skb_reset_transport_header(skb);
    if (skb.protocol == htons(ETH_P_802_2)) {
    __be16 pdulen;
    s32 data_size;
    if (skb.mac_len < ETH_HLEN)
    return 0;
    pdulen = eth_hdr(skb).h_proto;
    data_size = ntohs(pdulen) - llc_len;
    if (data_size < 0 ||
    !pskb_may_pull(skb, data_size))
    return 0;
    if (unlikely(pskb_trim_rcsum(skb, data_size)))
    return 0;
    }
    return 1;
    }
//
// llc_rcv - 802.2 entry point from net lower layers
// @skb: received pdu
// @dev: device that receive pdu
// @pt: packet type
// @orig_dev: the original receive net device
//
// When the system receives a 802.2 frame this function is called. It
// checks SAP and connection of received pdu and passes frame to
// llc_{station,sap,conn}_rcv for sending to proper state machine. If
// the frame is related to a busy connection (a connection is sending
// data now), it queues this frame in the connection's backlog.
//
    int llc_rcv(struct sk_buff *skb, struct net_device *dev,
    struct packet_type *pt, struct net_device *orig_dev)
    {
    struct llc_sap *sap;
    struct llc_pdu_sn *pdu;
    int dest;
    int (*rcv)(struct sk_buff *, struct net_device *,
    struct packet_type *, struct net_device *);
    void (*sta_handler)(struct sk_buff *skb);
    void (*sap_handler)(struct llc_sap *sap, struct sk_buff *skb);
//
// When the interface is in promisc. mode, drop all the crap that it
// receives, do not try to analyse it.
//
    if (unlikely(skb.pkt_type == PACKET_OTHERHOST)) {
    dprintk("%s: PACKET_OTHERHOST\n", __func__);
    goto drop;
    }
    skb = skb_share_check(skb, GFP_ATOMIC);
    if (unlikely(!skb))
    goto out;
    if (unlikely(!llc_fixup_skb(skb)))
    goto drop;
    pdu = llc_pdu_sn_hdr(skb);
    if (unlikely(!pdu.dsap)) /* core::ptr::null_mut() DSAP, refer to station */
    goto handle_station;
    sap = llc_sap_find(pdu.dsap);
    if (unlikely(!sap)) {/* unknown SAP */
    dprintk("%s: llc_sap_find(%02X) failed!\n", __func__,
    pdu.dsap);
    goto drop;
    }
//
// First the upper layer protocols that don't need the full
// LLC functionality
//
    rcv = rcu_dereference(sap.rcv_func);
    dest = llc_pdu_type(skb);
    sap_handler = dest ? READ_ONCE(llc_type_handlers[dest - 1]) : core::ptr::null_mut();
    if (unlikely(!sap_handler)) {
    if (rcv)
    rcv(skb, dev, pt, orig_dev);
    else
    kfree_skb(skb);
    } else {
    if (rcv) {
    struct sk_buff *cskb = skb_clone(skb, GFP_ATOMIC);
    if (cskb)
    rcv(cskb, dev, pt, orig_dev);
    }
    sap_handler(sap, skb);
    }
    llc_sap_put(sap);
    out:
    return 0;
    drop:
    kfree_skb(skb);
    goto out;
    handle_station:
    sta_handler = READ_ONCE(llc_station_handler);
    if (!sta_handler)
    goto drop;
    sta_handler(skb);
    goto out;
    }
    EXPORT_SYMBOL(llc_add_pack);
    EXPORT_SYMBOL(llc_remove_pack);
    EXPORT_SYMBOL(llc_set_station_handler);
