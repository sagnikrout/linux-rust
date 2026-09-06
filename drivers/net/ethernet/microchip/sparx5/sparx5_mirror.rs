//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/microchip/sparx5/sparx5_mirror.c
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


// SPDX-License-Identifier: GPL-2.0+
// Microchip Sparx5 Switch driver
//
// Copyright (c) 2024 Microchip Technology Inc. and its subsidiaries.
//

pub const SPX5_MIRROR_PROBE_MAX: c_int = 3;
pub const SPX5_MIRROR_DISABLED: c_int = 0;
pub const SPX5_MIRROR_EGRESS: c_int = 1;
pub const SPX5_MIRROR_INGRESS: c_int = 2;

// Convert from bool ingress/egress to mirror direction
#[no_mangle]
unsafe extern "C" fn sparx5_mirror_to_dir(ingress: bool) -> u32 {
    static u32 sparx5_mirror_to_dir(bool ingress)
    {
    return ingress ? SPX5_MIRROR_INGRESS : SPX5_MIRROR_EGRESS;
    }
// Get ports belonging to this mirror
#[no_mangle]
unsafe extern "C" fn sparx5_mirror_port_get(sparx5: *mut sparx5, idx: u32) -> u64 {
    static u64 sparx5_mirror_port_get(struct sparx5 *sparx5, u32 idx)
    {
    u64 val;
    val = spx5_rd(sparx5, ANA_AC_PROBE_PORT_CFG(idx));
    if (is_sparx5(sparx5))
    val |= (u64)spx5_rd(sparx5, ANA_AC_PROBE_PORT_CFG1(idx)) << 32;
    return val;
    }
// Add port to mirror (only front ports)
#[no_mangle]
unsafe extern "C" fn sparx5_mirror_port_add(sparx5: *mut sparx5, idx: u32, portno: u32) {
    static void sparx5_mirror_port_add(struct sparx5 *sparx5, u32 idx, u32 portno)
    {
    let mut reg: u64 = portno;
    u32 val;
    val = BIT(do_div(reg, 32));
    if (reg == 0)
    return spx5_rmw(val, val, sparx5, ANA_AC_PROBE_PORT_CFG(idx));
    else
    return spx5_rmw(val, val, sparx5, ANA_AC_PROBE_PORT_CFG1(idx));
    }
// Delete port from mirror (only front ports)
#[no_mangle]
unsafe extern "C" fn sparx5_mirror_port_del(sparx5: *mut sparx5, idx: u32, portno: u32) {
    static void sparx5_mirror_port_del(struct sparx5 *sparx5, u32 idx, u32 portno)
    {
    let mut reg: u64 = portno;
    u32 val;
    val = BIT(do_div(reg, 32));
    if (reg == 0)
    return spx5_rmw(0, val, sparx5, ANA_AC_PROBE_PORT_CFG(idx));
    else
    return spx5_rmw(0, val, sparx5, ANA_AC_PROBE_PORT_CFG1(idx));
    }
// Check if mirror contains port
#[no_mangle]
unsafe extern "C" fn sparx5_mirror_contains(sparx5: *mut sparx5, idx: u32, portno: u32) -> bool {
    static bool sparx5_mirror_contains(struct sparx5 *sparx5, u32 idx, u32 portno)
    {
    return (sparx5_mirror_port_get(sparx5, idx) & BIT_ULL(portno)) != 0;
    }
// Check if mirror is empty
#[no_mangle]
unsafe extern "C" fn sparx5_mirror_is_empty(sparx5: *mut sparx5, idx: u32) -> bool {
    static bool sparx5_mirror_is_empty(struct sparx5 *sparx5, u32 idx)
    {
    return sparx5_mirror_port_get(sparx5, idx) == 0;
    }
// Get direction of mirror
#[no_mangle]
unsafe extern "C" fn sparx5_mirror_dir_get(sparx5: *mut sparx5, idx: u32) -> u32 {
    static u32 sparx5_mirror_dir_get(struct sparx5 *sparx5, u32 idx)
    {
    let mut val: u32 = spx5_rd(sparx5, ANA_AC_PROBE_CFG(idx));
    return ANA_AC_PROBE_CFG_PROBE_DIRECTION_GET(val);
    }
// Set direction of mirror
#[no_mangle]
unsafe extern "C" fn sparx5_mirror_dir_set(sparx5: *mut sparx5, idx: u32, dir: u32) {
    static void sparx5_mirror_dir_set(struct sparx5 *sparx5, u32 idx, u32 dir)
    {
    spx5_rmw(ANA_AC_PROBE_CFG_PROBE_DIRECTION_SET(dir),
    ANA_AC_PROBE_CFG_PROBE_DIRECTION, sparx5,
    ANA_AC_PROBE_CFG(idx));
    }
// Set the monitor port for this mirror
    static void sparx5_mirror_monitor_set(struct sparx5 *sparx5, u32 idx,
    u32 portno)
    {
    spx5_rmw(QFWD_FRAME_COPY_CFG_FRMC_PORT_VAL_SET(portno),
    QFWD_FRAME_COPY_CFG_FRMC_PORT_VAL, sparx5,
    QFWD_FRAME_COPY_CFG(idx + SPX5_QFWD_MP_OFFSET));
    }
// Get the monitor port of this mirror
#[no_mangle]
unsafe extern "C" fn sparx5_mirror_monitor_get(sparx5: *mut sparx5, idx: u32) -> u32 {
    static u32 sparx5_mirror_monitor_get(struct sparx5 *sparx5, u32 idx)
    {
    u32 val = spx5_rd(sparx5,
    QFWD_FRAME_COPY_CFG(idx + SPX5_QFWD_MP_OFFSET));
    return QFWD_FRAME_COPY_CFG_FRMC_PORT_VAL_GET(val);
    }
// Check if port is the monitor port of this mirror
    static bool sparx5_mirror_has_monitor(struct sparx5 *sparx5, u32 idx,
    u32 portno)
    {
    return sparx5_mirror_monitor_get(sparx5, idx) == portno;
    }
// Get a suitable mirror for this port
    static int sparx5_mirror_get(struct sparx5_port *sport,
    struct sparx5_port *mport, u32 dir, u32 *idx)
    {
    struct sparx5 *sparx5 = sport.sparx5;
    u32 i;
// Check if this port is already used as a monitor port
    for (i = 0; i < SPX5_MIRROR_PROBE_MAX; i++)
    if (sparx5_mirror_has_monitor(sparx5, i, sport.portno))
    return -EINVAL;
// Check if existing mirror can be reused
// (same direction and monitor port).
//
    for (i = 0; i < SPX5_MIRROR_PROBE_MAX; i++) {
    if (sparx5_mirror_dir_get(sparx5, i) == dir &&
    sparx5_mirror_has_monitor(sparx5, i, mport.portno)) {
// idx = i;
    return 0;
    }
    }
// Return free mirror
    for (i = 0; i < SPX5_MIRROR_PROBE_MAX; i++) {
    if (sparx5_mirror_is_empty(sparx5, i)) {
// idx = i;
    return 0;
    }
    }
    return -ENOENT;
    }
#[no_mangle]
pub unsafe extern "C" fn sparx5_mirror_add(entry: *mut sparx5_mall_entry) -> c_int {
    int sparx5_mirror_add(struct sparx5_mall_entry *entry)
    {
    u32 mirror_idx, dir = sparx5_mirror_to_dir(entry.ingress);
    struct sparx5_port *sport, *mport;
    struct sparx5 *sparx5;
    int err;
// Source port
    sport = entry.port;
// monitor port
    mport = entry.mirror.port;
    sparx5 = sport.sparx5;
    if (sport.portno == mport.portno)
    return -EINVAL;
    err = sparx5_mirror_get(sport, mport, dir, &mirror_idx);
    if (err)
    return err;
    if (sparx5_mirror_contains(sparx5, mirror_idx, sport.portno))
    return -EEXIST;
// Add port to mirror
    sparx5_mirror_port_add(sparx5, mirror_idx, sport.portno);
// Set direction of mirror
    sparx5_mirror_dir_set(sparx5, mirror_idx, dir);
// Set monitor port for mirror
    sparx5_mirror_monitor_set(sparx5, mirror_idx, mport.portno);
    entry.mirror.idx = mirror_idx;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn sparx5_mirror_del(entry: *mut sparx5_mall_entry) {
    void sparx5_mirror_del(struct sparx5_mall_entry *entry)
    {
    struct sparx5_port *port = entry.port;
    struct sparx5 *sparx5 = port.sparx5;
    let mut mirror_idx: u32 = entry.mirror.idx;
    sparx5_mirror_port_del(sparx5, mirror_idx, port.portno);
    if (!sparx5_mirror_is_empty(sparx5, mirror_idx))
    return;
    sparx5_mirror_dir_set(sparx5, mirror_idx, SPX5_MIRROR_DISABLED);
    sparx5_mirror_monitor_set(sparx5,
    mirror_idx,
    sparx5.data.consts.n_ports);
    }
    void sparx5_mirror_stats(struct sparx5_mall_entry *entry,
    struct flow_stats *fstats)
    {
    struct sparx5_port *port = entry.port;
    struct rtnl_link_stats64 new_stats;
    struct flow_stats *old_stats;
    old_stats = &entry.port.mirror_stats;
    sparx5_get_stats64(port.ndev, &new_stats);
    if (entry.ingress) {
    flow_stats_update(fstats,
    new_stats.rx_bytes - old_stats.bytes,
    new_stats.rx_packets - old_stats.pkts,
    new_stats.rx_dropped - old_stats.drops,
    old_stats.lastused,
    FLOW_ACTION_HW_STATS_IMMEDIATE);
    old_stats.bytes = new_stats.rx_bytes;
    old_stats.pkts = new_stats.rx_packets;
    old_stats.drops = new_stats.rx_dropped;
    old_stats.lastused = jiffies;
    } else {
    flow_stats_update(fstats,
    new_stats.tx_bytes - old_stats.bytes,
    new_stats.tx_packets - old_stats.pkts,
    new_stats.tx_dropped - old_stats.drops,
    old_stats.lastused,
    FLOW_ACTION_HW_STATS_IMMEDIATE);
    old_stats.bytes = new_stats.tx_bytes;
    old_stats.pkts = new_stats.tx_packets;
    old_stats.drops = new_stats.tx_dropped;
    old_stats.lastused = jiffies;
    }
    }
