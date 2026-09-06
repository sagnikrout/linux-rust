//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/netronome/nfp/nfp_app.h
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
pub const _NFP_APP_H: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfp_app_id {
    NFP_APP_CORE_NIC	= 0x1,
    NFP_APP_BPF_NIC		= 0x2,
    NFP_APP_FLOWER_NIC	= 0x3,
    NFP_APP_ACTIVE_BUFFER_MGMT_NIC = 0x4,
}

//
// struct nfp_app_type - application definition
// @id:		application ID
// @name:	application name
// @ctrl_cap_mask:  ctrl vNIC capability mask, allows disabling features like
// IRQMOD which are on by default but counter-productive for
// control messages which are often latency-sensitive
// @ctrl_has_meta:  control messages have prepend of type:5/port:CTRL
//
// Callbacks
// @init:	perform basic app checks and init
// @clean:	clean app state
// @extra_cap:	extra capabilities string
// @ndo_init:	vNIC and repr netdev .ndo_init
// @ndo_uninit:	vNIC and repr netdev .ndo_unint
// @vnic_alloc:	allocate vNICs (assign port types, etc.)
// @vnic_free:	free up app's vNIC state
// @vnic_init:	vNIC netdev was registered
// @vnic_clean:	vNIC netdev about to be unregistered
// @repr_init:	representor about to be registered
// @repr_preclean:	representor about to unregistered, executed before app
// reference to the it is removed
// @repr_clean:	representor about to be unregistered
// @repr_open:	representor netdev open callback
// @repr_stop:	representor netdev stop callback
// @check_mtu:	MTU change request on a netdev (verify it is valid)
// @repr_change_mtu:	MTU change request on repr (make and verify change)
// @port_get_stats:		get extra ethtool statistics for a port
// @port_get_stats_count:	get count of extra statistics for a port
// @port_get_stats_strings:	get strings for extra statistics
// @start:	start application logic
// @stop:	stop application logic
// @netdev_event:	Netdevice notifier event
// @ctrl_msg_rx:    control message handler
// @ctrl_msg_rx_raw:	handler for control messages from data queues
// @setup_tc:	setup TC ndo
// @bpf:	BPF ndo offload-related calls
// @xdp_offload:    offload an XDP program
// @eswitch_mode_get:    get SR-IOV eswitch mode
// @eswitch_mode_set:    set SR-IOV eswitch mode
// @sriov_enable: app-specific sriov initialisation
// @sriov_disable: app-specific sriov clean-up
// @dev_get:	get representor or internal port representing netdev
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_app_type {
    pub id: nfp_app_id,
    pub name: *const c_char,
    pub ctrl_cap_mask: u32,
    pub ctrl_has_meta: bool,
    pub app): *mut *mut int (init)(struct nfp_app,
    pub app): *mut *mut void (clean)(struct nfp_app,
    pub nn): *const *const *const *const char (extra_cap)(struct nfp_app app, struct nfp_net,
    pub netdev): *mut *mut *mut int (ndo_init)(struct nfp_app app, struct net_device,
    pub netdev): *mut *mut *mut void (ndo_uninit)(struct nfp_app app, struct net_device,
    pub id): c_uint,
    pub nn): *mut *mut *mut void (vnic_free)(struct nfp_app app, struct nfp_net,
    pub nn): *mut *mut *mut int (vnic_init)(struct nfp_app app, struct nfp_net,
    pub nn): *mut *mut *mut void (vnic_clean)(struct nfp_app app, struct nfp_net,
    pub netdev): *mut *mut *mut int (repr_init)(struct nfp_app app, struct net_device,
    pub netdev): *mut *mut *mut void (repr_preclean)(struct nfp_app app, struct net_device,
    pub netdev): *mut *mut *mut void (repr_clean)(struct nfp_app app, struct net_device,
    pub repr): *mut *mut *mut int (repr_open)(struct nfp_app app, struct nfp_repr,
    pub repr): *mut *mut *mut int (repr_stop)(struct nfp_app app, struct nfp_repr,
    pub new_mtu): c_int,
    pub new_mtu): c_int,
    pub data): *mut *mut nfp_port port, u64,
    pub port): *mut *mut *mut int (port_get_stats_count)(struct nfp_app app, struct nfp_port,
    pub data): *mut *mut nfp_port port, u8,
    pub app): *mut *mut int (start)(struct nfp_app,
    pub app): *mut *mut void (stop)(struct nfp_app,
    pub ptr): *mut unsigned long event, void,
    pub skb): *mut *mut *mut void (ctrl_msg_rx)(struct nfp_app app, struct sk_buff,
    pub len): c_uint,
    pub type_data): *mut tc_setup_type type, void,
    pub xdp): *mut netdev_bpf,
    pub extack): *mut netlink_ext_ack,
    pub num_vfs): *mut *mut *mut int (sriov_enable)(struct nfp_app app, int,
    pub app): *mut *mut void (sriov_disable)(struct nfp_app,
    pub app): *mut *mut devlink_eswitch_mode (eswitch_mode_get)(struct nfp_app,
    pub mode): *mut *mut *mut int (eswitch_mode_set)(struct nfp_app app, u16,
    pub redir_egress): *mut bool,
}

//
// struct nfp_app - NFP application container
// @pdev:	backpointer to PCI device
// @pf:		backpointer to NFP PF structure
// @cpp:	pointer to the CPP handle
// @ctrl:	pointer to ctrl vNIC struct
// @reprs:	array of pointers to representors
// @type:	pointer to const application ops and info
// @ctrl_mtu:	MTU to set on the control vNIC (set in .init())
// @netdev_nb:	Netdevice notifier block
// @priv:	app-specific priv data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_app {
    pub pdev: *mut pci_dev,
    pub pf: *mut nfp_pf,
    pub cpp: *mut nfp_cpp,
    pub ctrl: *mut nfp_net,
    pub 1]: *mut *mut nfp_reprs __rcu reprs[NFP_REPR_TYPE_MAX +,
    pub type: *const nfp_app_type,
    pub ctrl_mtu: c_uint,
    pub netdev_nb: notifier_block,
    pub priv: *mut c_void,
}

extern "C" {
    pub fn devl_lock_is_held(_arg: priv_to_devlink(app->pf)) -> return;
}
extern "C" {
    pub fn nfp_check_rhashtable_empty(ptr: *mut c_void, arg: *mut c_void);
}
extern "C" {
    pub fn __nfp_ctrl_tx(nn: *mut nfp_net, skb: *mut sk_buff) -> bool;
}
extern "C" {
    pub fn nfp_ctrl_tx(nn: *mut nfp_net, skb: *mut sk_buff) -> bool;
}
extern "C" {
    pub fn nfp_app_ndo_init(netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn nfp_app_ndo_uninit(netdev: *mut net_device);
}
extern "C" {
    pub fn __nfp_ctrl_tx(_arg: app->ctrl, _arg: skb) -> return;
}
extern "C" {
    pub fn nfp_ctrl_tx(_arg: app->ctrl, _arg: skb) -> return;
}
// mode = app->type->eswitch_mode_get(app);
extern "C" {
    pub fn nfp_app_port_get_stats_count(port: *mut nfp_port) -> c_int;
}
extern "C" {
    pub fn nfp_app_free(app: *mut nfp_app);
}
extern "C" {
    pub fn nfp_app_start(app: *mut nfp_app, ctrl: *mut nfp_net) -> c_int;
}
extern "C" {
    pub fn nfp_app_stop(app: *mut nfp_app);
}
// Callbacks shared between apps
