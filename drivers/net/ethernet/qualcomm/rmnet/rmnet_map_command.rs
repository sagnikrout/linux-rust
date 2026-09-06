//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/qualcomm/rmnet/rmnet_map_command.c
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
// Copyright (c) 2013-2018, The Linux Foundation. All rights reserved.
//

    static u8 rmnet_map_do_flow_control(struct sk_buff *skb,
    struct rmnet_port *port,
    int enable)
    {
    struct rmnet_map_header *map_header = (void *)skb.data;
    struct rmnet_endpoint *ep;
    struct net_device *vnd;
    u8 mux_id;
    int r;
    mux_id = map_header.mux_id;
    if (mux_id >= RMNET_MAX_LOGICAL_EP) {
    kfree_skb(skb);
    return RX_HANDLER_CONSUMED;
    }
    ep = rmnet_get_endpoint(port, mux_id);
    if (!ep) {
    kfree_skb(skb);
    return RX_HANDLER_CONSUMED;
    }
    vnd = ep.egress_dev;
// Ignore the ip family and pass the sequence number for both v4 and v6
// sequence. User space does not support creating dedicated flows for
// the 2 protocols
//
    r = rmnet_vnd_do_flow_control(vnd, enable);
    if (r) {
    kfree_skb(skb);
    return RMNET_MAP_COMMAND_UNSUPPORTED;
    } else {
    return RMNET_MAP_COMMAND_ACK;
    }
    }
    static void rmnet_map_send_ack(struct sk_buff *skb,
    unsigned char type,
    struct rmnet_port *port)
    {
    struct rmnet_map_header *map_header = (void *)skb.data;
    struct rmnet_map_control_command *cmd;
    struct net_device *dev = skb.dev;
    if (port.data_format & RMNET_FLAGS_INGRESS_MAP_CKSUMV4)
    skb_trim(skb,
    skb.len - sizeof(struct rmnet_map_dl_csum_trailer));
    skb.protocol = htons(ETH_P_MAP);
// Command data immediately follows the MAP header
    cmd = (struct rmnet_map_control_command *)(map_header + 1);
    cmd.cmd_type = type & 0x03;
    netif_tx_lock(dev);
    dev.netdev_ops.ndo_start_xmit(skb, dev);
    netif_tx_unlock(dev);
    }
// Process MAP command frame and send N/ACK message as appropriate. Message cmd
// name is decoded here and appropriate handler is called.
//
#[no_mangle]
pub unsafe extern "C" fn rmnet_map_command(skb: *mut sk_buff, port: *mut rmnet_port) {
    void rmnet_map_command(struct sk_buff *skb, struct rmnet_port *port)
    {
    struct rmnet_map_header *map_header = (void *)skb.data;
    struct rmnet_map_control_command *cmd;
    unsigned char command_name;
    let mut rc: c_uchar = 0;
// Command data immediately follows the MAP header
    cmd = (struct rmnet_map_control_command *)(map_header + 1);
    command_name = cmd.command_name;
    switch (command_name) {
    case RMNET_MAP_COMMAND_FLOW_ENABLE:
    rc = rmnet_map_do_flow_control(skb, port, 1);
    break;
    case RMNET_MAP_COMMAND_FLOW_DISABLE:
    rc = rmnet_map_do_flow_control(skb, port, 0);
    break;
    default:
    rc = RMNET_MAP_COMMAND_UNSUPPORTED;
    kfree_skb(skb);
    break;
    }
    if (rc == RMNET_MAP_COMMAND_ACK)
    rmnet_map_send_ack(skb, rc, port);
    }
