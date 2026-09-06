//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/netronome/nfp/nfp_app_nic.c
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (C) 2017-2018 Netronome Systems, Inc.

    int nfp_app_nic_vnic_init_phy_port(struct nfp_pf *pf, struct nfp_app *app,
    struct nfp_net *nn, unsigned int id)
    {
    int err;
    if (!pf.eth_tbl)
    return 0;
    nn.port = nfp_port_alloc(app, NFP_PORT_PHYS_PORT, nn.dp.netdev);
    if (IS_ERR(nn.port))
    return PTR_ERR(nn.port);
    err = nfp_port_init_phy_port(pf, app, nn.port, id);
    if (err) {
    nfp_port_free(nn.port);
    return err;
    }
    return nn.port.type == NFP_PORT_INVALID;
    }
    int nfp_app_nic_vnic_alloc(struct nfp_app *app, struct nfp_net *nn,
    unsigned int id)
    {
    int err;
    err = nfp_app_nic_vnic_init_phy_port(app.pf, app, nn, id);
    if (err)
    return err < 0 ? err : 0;
    nfp_net_get_mac_addr(app.pf, nn.dp.netdev, nn.port);
    return 0;
    }
