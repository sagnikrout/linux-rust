//! Automatically rewritten from C to Rust
//! Source: drivers/scsi/fcoe/fcoe_ctlr.c
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
// Copyright (c) 2008-2009 Cisco Systems, Inc.  All rights reserved.
// Copyright (c) 2009 Intel Corporation.  All rights reserved.
//
// Maintained at www.Open-FCoE.org
//

    static void fcoe_ctlr_timeout(struct timer_list *);
    static void fcoe_ctlr_timer_work(struct work_struct *);
    static void fcoe_ctlr_recv_work(struct work_struct *);
    static int fcoe_ctlr_flogi_retry(struct fcoe_ctlr *);
    static void fcoe_ctlr_vn_start(struct fcoe_ctlr *);
    static int fcoe_ctlr_vn_recv(struct fcoe_ctlr *, struct sk_buff *);
    static void fcoe_ctlr_vn_timeout(struct fcoe_ctlr *);
    static int fcoe_ctlr_vn_lookup(struct fcoe_ctlr *, u32, u8 *);
    static int fcoe_ctlr_vlan_recv(struct fcoe_ctlr *, struct sk_buff *);
    static u8 fcoe_all_fcfs[ETH_ALEN] = FIP_ALL_FCF_MACS;
    static u8 fcoe_all_enode[ETH_ALEN] = FIP_ALL_ENODE_MACS;
    static u8 fcoe_all_vn2vn[ETH_ALEN] = FIP_ALL_VN2VN_MACS;
    static u8 fcoe_all_p2p[ETH_ALEN] = FIP_ALL_P2P_MACS;
    static const char * const fcoe_ctlr_states[] = {
    [FIP_ST_DISABLED] =	"DISABLED",
    [FIP_ST_LINK_WAIT] =	"LINK_WAIT",
    [FIP_ST_AUTO] =		"AUTO",
    [FIP_ST_NON_FIP] =	"NON_FIP",
    [FIP_ST_ENABLED] =	"ENABLED",
    [FIP_ST_VNMP_START] =	"VNMP_START",
    [FIP_ST_VNMP_PROBE1] =	"VNMP_PROBE1",
    [FIP_ST_VNMP_PROBE2] =	"VNMP_PROBE2",
    [FIP_ST_VNMP_CLAIM] =	"VNMP_CLAIM",
    [FIP_ST_VNMP_UP] =	"VNMP_UP",
    };
    static const char *fcoe_ctlr_state(enum fip_state state)
    {
    const char *cp = "unknown";
    if (state < ARRAY_SIZE(fcoe_ctlr_states))
    cp = fcoe_ctlr_states[state];
    if (!cp)
    cp = "unknown";
    return cp;
    }
//
// fcoe_ctlr_set_state() - Set and do debug printing for the new FIP state.
// @fip: The FCoE controller
// @state: The new state
//
#[no_mangle]
unsafe extern "C" fn fcoe_ctlr_set_state(fip: *mut fcoe_ctlr, state: enum fip_state) {
    static void fcoe_ctlr_set_state(struct fcoe_ctlr *fip, enum fip_state state)
    {
    if (state == fip.state)
    return;
    if (fip.lp)
    LIBFCOE_FIP_DBG(fip, "state %s . %s\n",
    fcoe_ctlr_state(fip.state), fcoe_ctlr_state(state));
    fip.state = state;
    }
//
// fcoe_ctlr_mtu_valid() - Check if a FCF's MTU is valid
// @fcf: The FCF to check
//
// Return non-zero if FCF fcoe_size has been validated.
//
#[no_mangle]
pub unsafe extern "C" fn fcoe_ctlr_mtu_valid(fcf: *const fcoe_fcf) -> c_int {
    static inline int fcoe_ctlr_mtu_valid(const struct fcoe_fcf *fcf)
    {
    return (fcf.flags & FIP_FL_SOL) != 0;
    }
//
// fcoe_ctlr_fcf_usable() - Check if a FCF is usable
// @fcf: The FCF to check
//
// Return non-zero if the FCF is usable.
//
#[no_mangle]
pub unsafe extern "C" fn fcoe_ctlr_fcf_usable(fcf: *mut fcoe_fcf) -> c_int {
    static inline int fcoe_ctlr_fcf_usable(struct fcoe_fcf *fcf)
    {
    let mut flags: u16 = FIP_FL_SOL | FIP_FL_AVAIL;
    return (fcf.flags & flags) == flags;
    }
//
// fcoe_ctlr_map_dest() - Set flag and OUI for mapping destination addresses
// @fip: The FCoE controller
//
#[no_mangle]
unsafe extern "C" fn fcoe_ctlr_map_dest(fip: *mut fcoe_ctlr) {
    static void fcoe_ctlr_map_dest(struct fcoe_ctlr *fip)
    {
    if (fip.mode == FIP_MODE_VN2VN)
    hton24(fip.dest_addr, FIP_VN_FC_MAP);
    else
    hton24(fip.dest_addr, FIP_DEF_FC_MAP);
    hton24(fip.dest_addr + 3, 0);
    fip.map_dest = 1;
    }
//
// fcoe_ctlr_init() - Initialize the FCoE Controller instance
// @fip: The FCoE controller to initialize
// @mode: FIP mode to set
//
#[no_mangle]
pub unsafe extern "C" fn fcoe_ctlr_init(fip: *mut fcoe_ctlr, mode: enum fip_mode) {
    void fcoe_ctlr_init(struct fcoe_ctlr *fip, enum fip_mode mode)
    {
    fcoe_ctlr_set_state(fip, FIP_ST_LINK_WAIT);
    fip.mode = mode;
    fip.fip_resp = false;
    INIT_LIST_HEAD(&fip.fcfs);
    mutex_init(&fip.ctlr_mutex);
    spin_lock_init(&fip.ctlr_lock);
    fip.flogi_oxid = FC_XID_UNKNOWN;
    timer_setup(&fip.timer, fcoe_ctlr_timeout, 0);
    INIT_WORK(&fip.timer_work, fcoe_ctlr_timer_work);
    INIT_WORK(&fip.recv_work, fcoe_ctlr_recv_work);
    skb_queue_head_init(&fip.fip_recv_list);
    }
    EXPORT_SYMBOL(fcoe_ctlr_init);
//
// fcoe_sysfs_fcf_add() - Add a fcoe_fcf{,_device} to a fcoe_ctlr{,_device}
// @new: The newly discovered FCF
//
// Called with fip->ctlr_mutex held
//
#[no_mangle]
unsafe extern "C" fn fcoe_sysfs_fcf_add(new: *mut fcoe_fcf) -> c_int {
    static int fcoe_sysfs_fcf_add(struct fcoe_fcf *new)
    {
    struct fcoe_ctlr *fip = new.fip;
    struct fcoe_ctlr_device *ctlr_dev;
    struct fcoe_fcf_device *temp, *fcf_dev;
    let mut rc: c_int = -ENOMEM;
    LIBFCOE_FIP_DBG(fip, "New FCF fab %16.16llx mac %pM\n",
    new.fabric_name, new.fcf_mac);
    temp = kzalloc_obj(*temp);
    if (!temp)
    goto out;
    temp.fabric_name = new.fabric_name;
    temp.switch_name = new.switch_name;
    temp.fc_map = new.fc_map;
    temp.vfid = new.vfid;
    memcpy(temp.mac, new.fcf_mac, ETH_ALEN);
    temp.priority = new.pri;
    temp.fka_period = new.fka_period;
    temp.selected = 0; /* default to unselected */
//
// If ctlr_dev doesn't exist then it means we're a libfcoe user
// who doesn't use fcoe_syfs and didn't allocate a fcoe_ctlr_device.
// fnic would be an example of a driver with this behavior. In this
// case we want to add the fcoe_fcf to the fcoe_ctlr list, but we
// don't want to make sysfs changes.
//
    ctlr_dev = fcoe_ctlr_to_ctlr_dev(fip);
    if (ctlr_dev) {
    mutex_lock(&ctlr_dev.lock);
    fcf_dev = fcoe_fcf_device_add(ctlr_dev, temp);
    if (unlikely(!fcf_dev)) {
    rc = -ENOMEM;
    mutex_unlock(&ctlr_dev.lock);
    goto out;
    }
//
// The fcoe_sysfs layer can return a CONNECTED fcf that
// has a priv (fcf was never deleted) or a CONNECTED fcf
// that doesn't have a priv (fcf was deleted). However,
// libfcoe will always delete FCFs before trying to add
// them. This is ensured because both recv_adv and
// age_fcfs are protected by the the fcoe_ctlr's mutex.
// This means that we should never get a FCF with a
// non-NULL priv pointer.
//
    BUG_ON(fcf_dev.priv);
    fcf_dev.priv = new;
    new.fcf_dev = fcf_dev;
    mutex_unlock(&ctlr_dev.lock);
    }
    list_add(&new.list, &fip.fcfs);
    fip.fcf_count++;
    rc = 0;
    out:
    kfree(temp);
    return rc;
    }
//
// fcoe_sysfs_fcf_del() - Remove a fcoe_fcf{,_device} to a fcoe_ctlr{,_device}
// @new: The FCF to be removed
//
// Called with fip->ctlr_mutex held
//
#[no_mangle]
unsafe extern "C" fn fcoe_sysfs_fcf_del(new: *mut fcoe_fcf) {
    static void fcoe_sysfs_fcf_del(struct fcoe_fcf *new)
    {
    struct fcoe_ctlr *fip = new.fip;
    struct fcoe_ctlr_device *cdev;
    struct fcoe_fcf_device *fcf_dev;
    list_del(&new.list);
    fip.fcf_count--;
//
// If ctlr_dev doesn't exist then it means we're a libfcoe user
// who doesn't use fcoe_syfs and didn't allocate a fcoe_ctlr_device
// or a fcoe_fcf_device.
//
// fnic would be an example of a driver with this behavior. In this
// case we want to remove the fcoe_fcf from the fcoe_ctlr list (above),
// but we don't want to make sysfs changes.
//
    cdev = fcoe_ctlr_to_ctlr_dev(fip);
    if (cdev) {
    mutex_lock(&cdev.lock);
    fcf_dev = fcoe_fcf_to_fcf_dev(new);
    WARN_ON(!fcf_dev);
    new.fcf_dev = core::ptr::null_mut();
    fcoe_fcf_device_delete(fcf_dev);
    mutex_unlock(&cdev.lock);
    }
    kfree(new);
    }
//
// fcoe_ctlr_reset_fcfs() - Reset and free all FCFs for a controller
// @fip: The FCoE controller whose FCFs are to be reset
//
// Called with &fcoe_ctlr lock held.
//
#[no_mangle]
unsafe extern "C" fn fcoe_ctlr_reset_fcfs(fip: *mut fcoe_ctlr) {
    static void fcoe_ctlr_reset_fcfs(struct fcoe_ctlr *fip)
    {
    struct fcoe_fcf *fcf;
    struct fcoe_fcf *next;
    fip.sel_fcf = core::ptr::null_mut();
    list_for_each_entry_safe(fcf, next, &fip.fcfs, list) {
    fcoe_sysfs_fcf_del(fcf);
    }
    WARN_ON(fip.fcf_count);
    fip.sel_time = 0;
    }
//
// fcoe_ctlr_destroy() - Disable and tear down a FCoE controller
// @fip: The FCoE controller to tear down
//
// This is called by FCoE drivers before freeing the &fcoe_ctlr.
//
// The receive handler will have been deleted before this to guarantee
// that no more recv_work will be scheduled.
//
// The timer routine will simply return once we set FIP_ST_DISABLED.
// This guarantees that no further timeouts or work will be scheduled.
//
#[no_mangle]
pub unsafe extern "C" fn fcoe_ctlr_destroy(fip: *mut fcoe_ctlr) {
    void fcoe_ctlr_destroy(struct fcoe_ctlr *fip)
    {
    cancel_work_sync(&fip.recv_work);
    skb_queue_purge(&fip.fip_recv_list);
    mutex_lock(&fip.ctlr_mutex);
    fcoe_ctlr_set_state(fip, FIP_ST_DISABLED);
    fcoe_ctlr_reset_fcfs(fip);
    mutex_unlock(&fip.ctlr_mutex);
    timer_delete_sync(&fip.timer);
    cancel_work_sync(&fip.timer_work);
    }
    EXPORT_SYMBOL(fcoe_ctlr_destroy);
//
// fcoe_ctlr_announce() - announce new FCF selection
// @fip: The FCoE controller
//
// Also sets the destination MAC for FCoE and control packets
//
// Called with neither ctlr_mutex nor ctlr_lock held.
//
#[no_mangle]
unsafe extern "C" fn fcoe_ctlr_announce(fip: *mut fcoe_ctlr) {
    static void fcoe_ctlr_announce(struct fcoe_ctlr *fip)
    {
    struct fcoe_fcf *sel;
    struct fcoe_fcf *fcf;
    mutex_lock(&fip.ctlr_mutex);
    spin_lock_bh(&fip.ctlr_lock);
    kfree_skb(fip.flogi_req);
    fip.flogi_req = core::ptr::null_mut();
    list_for_each_entry(fcf, &fip.fcfs, list)
    fcf.flogi_sent = 0;
    spin_unlock_bh(&fip.ctlr_lock);
    sel = fip.sel_fcf;
    if (sel && ether_addr_equal(sel.fcf_mac, fip.dest_addr))
    goto unlock;
    if (!is_zero_ether_addr(fip.dest_addr)) {
    printk(KERN_NOTICE "libfcoe: host%d: "
    "FIP Fibre-Channel Forwarder MAC %pM deselected\n",
    fip.lp.host.host_no, fip.dest_addr);
    eth_zero_addr(fip.dest_addr);
    }
    if (sel) {
    printk(KERN_INFO "libfcoe: host%d: FIP selected "
    "Fibre-Channel Forwarder MAC %pM\n",
    fip.lp.host.host_no, sel.fcf_mac);
    memcpy(fip.dest_addr, sel.fcoe_mac, ETH_ALEN);
    fip.map_dest = 0;
    }
    unlock:
    mutex_unlock(&fip.ctlr_mutex);
    }
//
// fcoe_ctlr_fcoe_size() - Return the maximum FCoE size required for VN_Port
// @fip: The FCoE controller to get the maximum FCoE size from
//
// Returns the maximum packet size including the FCoE header and trailer,
// but not including any Ethernet or VLAN headers.
//
#[no_mangle]
pub unsafe extern "C" fn fcoe_ctlr_fcoe_size(fip: *mut fcoe_ctlr) -> u32 {
    static inline u32 fcoe_ctlr_fcoe_size(struct fcoe_ctlr *fip)
    {
//
// Determine the max FCoE frame size allowed, including
// FCoE header and trailer.
// Note:  lp->mfs is currently the payload size, not the frame size.
//
    return fip.lp.mfs + sizeof(struct fc_frame_header) +
    sizeof(struct fcoe_hdr) + sizeof(struct fcoe_crc_eof);
    }
//
// fcoe_ctlr_solicit() - Send a FIP solicitation
// @fip: The FCoE controller to send the solicitation on
// @fcf: The destination FCF (if NULL, a multicast solicitation is sent)
//
#[no_mangle]
unsafe extern "C" fn fcoe_ctlr_solicit(fip: *mut fcoe_ctlr, fcf: *mut fcoe_fcf) {
    static void fcoe_ctlr_solicit(struct fcoe_ctlr *fip, struct fcoe_fcf *fcf)
    {
    struct sk_buff *skb;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fip_sol {
    pub eth: ethhdr,
    pub fip: fip_header,
    struct {
    pub mac: fip_mac_desc,
    pub wwnn: fip_wwn_desc,
    pub size: fip_size_desc,
    pub desc: } __packed,
    pub sol: *mut *mut } __packed,
    pub fcoe_size: u32,
    pub dev_alloc_skb(sizeof(*sol)): *mut skb =,
    if (!skb)
    pub )skb->data: *mut sol = (struct fip_sol,
    pub sizeof(*sol)): *mut memset(sol, 0,,
    pub ETH_ALEN): memcpy(sol->eth.h_dest, fcf ? fcf->fcf_mac : fcoe_all_fcfs,,
    pub ETH_ALEN): memcpy(sol->eth.h_source, fip->ctl_src_addr,,
    pub htons(ETH_P_FIP): sol->eth.h_proto =,
    pub FIP_VER_ENCAPS(FIP_VER): sol->fip.fip_ver =,
    pub htons(FIP_OP_DISC): sol->fip.fip_op =,
    pub FIP_SC_SOL: sol->fip.fip_subcode =,
    pub FIP_BPW): sol->fip.fip_dl_len = htons(sizeof(sol->desc) /,
    pub htons(FIP_FL_FPMA): sol->fip.fip_flags =,
    if (fip.spma)
    pub htons(FIP_FL_SPMA): sol->fip.fip_flags |=,
    pub FIP_DT_MAC: sol->desc.mac.fd_desc.fip_dtype =,
    pub FIP_BPW: sol->desc.mac.fd_desc.fip_dlen = sizeof(sol->desc.mac) /,
    pub ETH_ALEN): memcpy(sol->desc.mac.fd_mac, fip->ctl_src_addr,,
    pub FIP_DT_NAME: sol->desc.wwnn.fd_desc.fip_dtype =,
    pub FIP_BPW: sol->desc.wwnn.fd_desc.fip_dlen = sizeof(sol->desc.wwnn) /,
    pub &sol->desc.wwnn.fd_wwn): put_unaligned_be64(fip->lp->wwnn,,
    pub fcoe_ctlr_fcoe_size(fip): fcoe_size =,
    pub FIP_DT_FCOE_SIZE: sol->desc.size.fd_desc.fip_dtype =,
    pub FIP_BPW: sol->desc.size.fd_desc.fip_dlen = sizeof(sol->desc.size) /,
    pub htons(fcoe_size): sol->desc.size.fd_size =,
    pub sizeof(*sol)): *mut skb_put(skb,,
    pub htons(ETH_P_FIP): skb->protocol =,
    pub fip->priority: skb->priority =,
    pub skb): fip->send(fip,,
    if (!fcf)
    pub jiffies: fip->sol_time =,
    }
//
// fcoe_ctlr_link_up() - Start FCoE controller
// @fip: The FCoE controller to start
//
// Called from the LLD when the network link is ready.
//
#[no_mangle]
pub unsafe extern "C" fn fcoe_ctlr_link_up(fip: *mut fcoe_ctlr) {
    void fcoe_ctlr_link_up(struct fcoe_ctlr *fip)
    {
    if (fip.state == FIP_ST_NON_FIP || fip.state == FIP_ST_AUTO) {
    } else if (fip.state == FIP_ST_LINK_WAIT) {
    if (fip.mode == FIP_MODE_NON_FIP)
    pub FIP_ST_NON_FIP): fcoe_ctlr_set_state(fip,,
    else
    pub FIP_ST_AUTO): fcoe_ctlr_set_state(fip,,
    switch (fip.mode) {
    default:
    pub fip->mode): LIBFCOE_FIP_DBG(fip, "invalid mode %d\n",,
    case FIP_MODE_AUTO:
    pub mode.\n"): LIBFCOE_FIP_DBG(fip, "%s", "setting AUTO,
    case FIP_MODE_FABRIC:
    case FIP_MODE_NON_FIP:
    pub NULL): fcoe_ctlr_solicit(fip,,
    case FIP_MODE_VN2VN:
    }
    } else
    }
//
// fcoe_ctlr_reset() - Reset a FCoE controller
// @fip:       The FCoE controller to reset
//
#[no_mangle]
unsafe extern "C" fn fcoe_ctlr_reset(fip: *mut fcoe_ctlr) {
    static void fcoe_ctlr_reset(struct fcoe_ctlr *fip)
    {
    pub 0: fip->ctlr_ka_time =,
    pub 0: fip->port_ka_time =,
    pub 0: fip->sol_time =,
    pub FC_XID_UNKNOWN: fip->flogi_oxid =,
    }
//
// fcoe_ctlr_link_down() - Stop a FCoE controller
// @fip: The FCoE controller to be stopped
//
// Returns non-zero if the link was up and now isn't.
//
// Called from the LLD when the network link is not ready.
// There may be multiple calls while the link is down.
//
#[no_mangle]
pub unsafe extern "C" fn fcoe_ctlr_link_down(fip: *mut fcoe_ctlr) -> c_int {
    int fcoe_ctlr_link_down(struct fcoe_ctlr *fip)
    {
    pub link_dropped: c_int,
    pub down.\n"): LIBFCOE_FIP_DBG(fip, "link,
    pub FIP_ST_LINK_WAIT: link_dropped = fip->state !=,
    pub FIP_ST_LINK_WAIT): fcoe_ctlr_set_state(fip,,
    if (link_dropped)
    pub link_dropped: return,
    }
//
// fcoe_ctlr_send_keep_alive() - Send a keep-alive to the selected FCF
// @fip:   The FCoE controller to send the FKA on
// @lport: libfc fc_lport to send from
// @ports: 0 for controller keep-alive, 1 for port keep-alive
// @sa:	   The source MAC address
//
// A controller keep-alive is sent every fka_period (typically 8 seconds).
// The source MAC is the native MAC address.
//
// A port keep-alive is sent every 90 seconds while logged in.
// The source MAC is the assigned mapped source address.
// The destination is the FCF's F-port.
//
    static void fcoe_ctlr_send_keep_alive(struct fcoe_ctlr *fip,
    struct fc_lport *lport,
    int ports, u8 *sa)
    {
    pub skb: *mut sk_buff,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fip_kal {
    pub eth: ethhdr,
    pub fip: fip_header,
    pub mac: fip_mac_desc,
    pub kal: *mut *mut } __packed,
    pub vn: *mut fip_vn_desc,
    pub len: u32,
    pub lp: *mut fc_lport,
    pub fcf: *mut fcoe_fcf,
    pub fip->sel_fcf: fcf =,
    pub fip->lp: lp =,
    if (!fcf || (ports && !lp.port_id))
    pub sizeof(*vn): *mut *mut *mut len = sizeof(kal) + ports,
    pub dev_alloc_skb(len): skb =,
    if (!skb)
    pub )skb->data: *mut kal = (struct fip_kal,
    pub len): memset(kal, 0,,
    pub ETH_ALEN): memcpy(kal->eth.h_dest, fcf->fcf_mac,,
    pub ETH_ALEN): memcpy(kal->eth.h_source, sa,,
    pub htons(ETH_P_FIP): kal->eth.h_proto =,
    pub FIP_VER_ENCAPS(FIP_VER): kal->fip.fip_ver =,
    pub htons(FIP_OP_CTRL): kal->fip.fip_op =,
    pub FIP_SC_KEEP_ALIVE: kal->fip.fip_subcode =,
    kal.fip.fip_dl_len = htons((sizeof(kal.mac) +
    pub FIP_BPW): *mut *mut *mut ports  sizeof(vn)) /,
    pub htons(FIP_FL_FPMA): kal->fip.fip_flags =,
    if (fip.spma)
    pub htons(FIP_FL_SPMA): kal->fip.fip_flags |=,
    pub FIP_DT_MAC: kal->mac.fd_desc.fip_dtype =,
    pub FIP_BPW: kal->mac.fd_desc.fip_dlen = sizeof(kal->mac) /,
    pub ETH_ALEN): memcpy(kal->mac.fd_mac, fip->ctl_src_addr,,
    if (ports) {
    pub 1): *mut *mut vn = (struct fip_vn_desc )(kal +,
    pub FIP_DT_VN_ID: vn->fd_desc.fip_dtype =,
    pub FIP_BPW: *mut *mut vn->fd_desc.fip_dlen = sizeof(vn) /,
    pub ETH_ALEN): memcpy(vn->fd_mac, fip->get_src_addr(lport),,
    pub lport->port_id): hton24(vn->fd_fc_id,,
    pub &vn->fd_wwpn): put_unaligned_be64(lport->wwpn,,
    }
    pub len): skb_put(skb,,
    pub htons(ETH_P_FIP): skb->protocol =,
    pub fip->priority: skb->priority =,
    pub skb): fip->send(fip,,
    }
//
// fcoe_ctlr_encaps() - Encapsulate an ELS frame for FIP, without sending it
// @fip:   The FCoE controller for the ELS frame
// @lport: The local port
// @dtype: The FIP descriptor type for the frame
// @skb:   The FCoE ELS frame including FC header but no FCoE headers
// @d_id:  The destination port ID.
//
// Returns non-zero error code on failure.
//
// The caller must check that the length is a multiple of 4.
//
// The @skb must have enough headroom (28 bytes) and tailroom (8 bytes).
// Headroom includes the FIP encapsulation description, FIP header, and
// Ethernet header.  The tailroom is for the FIP MAC descriptor.
//
    static int fcoe_ctlr_encaps(struct fcoe_ctlr *fip, struct fc_lport *lport,
    u8 dtype, struct sk_buff *skb, u32 d_id)
    {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fip_encaps_head {
    pub eth: ethhdr,
    pub fip: fip_header,
    pub encaps: fip_encaps,
    pub cap: *mut *mut } __packed,
    pub fh: *mut fc_frame_header,
    pub mac: *mut fip_mac_desc,
    pub fcf: *mut fcoe_fcf,
    pub dlen: usize,
    pub fip_flags: u16,
    pub op: u8,
    pub )skb->data: *mut fh = (struct fc_frame_header,
    pub 1): *mut *mut *mut op = (u8 )(fh +,
    pub /: *mut *mut dlen = sizeof(struct fip_encaps) + skb->len; / len before push,
    pub sizeof(*cap)): *mut cap = skb_push(skb,,
    pub sizeof(*cap)): *mut memset(cap, 0,,
    if (lport.point_to_multipoint) {
    if (fcoe_ctlr_vn_lookup(fip, d_id, cap.eth.h_dest))
    pub -ENODEV: return,
    pub 0: fip_flags =,
    } else {
    pub fip->sel_fcf: fcf =,
    if (!fcf)
    pub -ENODEV: return,
    pub fcf->flags: fip_flags =,
    fip_flags &= fip.spma ? FIP_FL_SPMA | FIP_FL_FPMA :
    if (!fip_flags)
    pub -ENODEV: return,
    pub ETH_ALEN): memcpy(cap->eth.h_dest, fcf->fcf_mac,,
    }
    pub ETH_ALEN): memcpy(cap->eth.h_source, fip->ctl_src_addr,,
    pub htons(ETH_P_FIP): cap->eth.h_proto =,
    pub FIP_VER_ENCAPS(FIP_VER): cap->fip.fip_ver =,
    pub htons(FIP_OP_LS): cap->fip.fip_op =,
    if (op == ELS_LS_ACC || op == ELS_LS_RJT)
    pub FIP_SC_REP: cap->fip.fip_subcode =,
    else
    pub FIP_SC_REQ: cap->fip.fip_subcode =,
    pub htons(fip_flags): cap->fip.fip_flags =,
    pub dtype: cap->encaps.fd_desc.fip_dtype =,
    pub FIP_BPW: cap->encaps.fd_desc.fip_dlen = dlen /,
    if (op != ELS_LS_RJT) {
    pub sizeof(*mac): *mut dlen +=,
    pub sizeof(*mac)): *mut mac = skb_put_zero(skb,,
    pub FIP_DT_MAC: mac->fd_desc.fip_dtype =,
    pub FIP_BPW: *mut *mut mac->fd_desc.fip_dlen = sizeof(mac) /,
    if (dtype != FIP_DT_FLOGI && dtype != FIP_DT_FDISC) {
    pub ETH_ALEN): memcpy(mac->fd_mac, fip->get_src_addr(lport),,
    } else if (fip.mode == FIP_MODE_VN2VN) {
    pub FIP_VN_FC_MAP): hton24(mac->fd_mac,,
    pub fip->port_id): hton24(mac->fd_mac + 3,,
    } else if (fip_flags & FIP_FL_SPMA) {
    pub SPMA\n"): LIBFCOE_FIP_DBG(fip, "FLOGI/FDISC sent with,
    pub ETH_ALEN): memcpy(mac->fd_mac, fip->ctl_src_addr,,
    } else {
    pub FPMA\n"): LIBFCOE_FIP_DBG(fip, "FLOGI/FDISC sent with,
// FPMA only FLOGI.  Must leave the MAC desc zeroed.
    }
    }
    pub FIP_BPW): cap->fip.fip_dl_len = htons(dlen /,
    pub htons(ETH_P_FIP): skb->protocol =,
    pub fip->priority: skb->priority =,
    pub 0: return,
    }
//
// fcoe_ctlr_els_send() - Send an ELS frame encapsulated by FIP if appropriate.
// @fip:	FCoE controller.
// @lport:	libfc fc_lport to send from
// @skb:	FCoE ELS frame including FC header but no FCoE headers.
//
// Returns a non-zero error code if the frame should not be sent.
// Returns zero if the caller should send the frame with FCoE encapsulation.
//
// The caller must check that the length is a multiple of 4.
// The SKB must have enough headroom (28 bytes) and tailroom (8 bytes).
// The the skb must also be an fc_frame.
//
// This is called from the lower-level driver with spinlocks held,
// so we must not take a mutex here.
//
    int fcoe_ctlr_els_send(struct fcoe_ctlr *fip, struct fc_lport *lport,
    struct sk_buff *skb)
    {
    pub fp: *mut fc_frame,
    pub fh: *mut fc_frame_header,
    pub old_xid: u16,
    pub op: u8,
    pub mac: [u8; ETH_ALEN],
    pub skb): fp = container_of(skb, struct fc_frame,,
    pub )skb->data: *mut fh = (struct fc_frame_header,
    pub 1): *mut *mut *mut op = (u8 )(fh +,
    if (op == ELS_FLOGI && fip.mode != FIP_MODE_VN2VN) {
    pub fip->flogi_oxid: old_xid =,
    pub ntohs(fh->fh_ox_id): fip->flogi_oxid =,
    if (fip.state == FIP_ST_AUTO) {
    if (old_xid == FC_XID_UNKNOWN)
    pub 0: fip->flogi_count =,
    if (fip.flogi_count < 3)
    pub drop: goto,
    pub 0: return,
    }
    if (fip.state == FIP_ST_NON_FIP)
    }
    if (fip.state == FIP_ST_NON_FIP)
    pub 0: return,
    if (!fip.sel_fcf && fip.mode != FIP_MODE_VN2VN)
    pub drop: goto,
    switch (op) {
    case ELS_FLOGI:
    pub FIP_DT_FLOGI: op =,
    if (fip.mode == FIP_MODE_VN2VN)
    pub skb: fip->flogi_req =,
    pub 1: fip->flogi_req_send =,
    pub -EINPROGRESS: return,
    case ELS_FDISC:
    if (ntoh24(fh.fh_s_id))
    pub 0: return,
    pub FIP_DT_FDISC: op =,
    case ELS_LOGO:
    if (fip.mode == FIP_MODE_VN2VN) {
    if (fip.state != FIP_ST_VNMP_UP)
    pub drop: goto,
    if (ntoh24(fh.fh_d_id) == FC_FID_FLOGI)
    pub drop: goto,
    } else {
    if (fip.state != FIP_ST_ENABLED)
    pub 0: return,
    if (ntoh24(fh.fh_d_id) != FC_FID_FLOGI)
    pub 0: return,
    }
    pub FIP_DT_LOGO: op =,
    case ELS_LS_ACC:
//
// If non-FIP, we may have gotten an SID by accepting an FLOGI
// from a point-to-point connection.  Switch to using
// the source mac based on the SID.  The destination
// MAC in this case would have been set by receiving the
// FLOGI.
//
    if (fip.state == FIP_ST_NON_FIP) {
    if (fip.flogi_oxid == FC_XID_UNKNOWN)
    pub 0: return,
    pub FC_XID_UNKNOWN: fip->flogi_oxid =,
    pub fh->fh_d_id): fc_fcoe_set_mac(mac,,
    pub mac): fip->update_mac(lport,,
    }
    case ELS_LS_RJT:
    pub fr_encaps(fp): op =,
    if (op)
    pub 0: return,
    default:
    if (fip.state != FIP_ST_ENABLED &&
    fip.state != FIP_ST_VNMP_UP)
    pub drop: goto,
    pub 0: return,
    }
    LIBFCOE_FIP_DBG(fip, "els_send op %u d_id %x\n",
    pub ntoh24(fh->fh_d_id)): op,,
    if (fcoe_ctlr_encaps(fip, lport, op, skb, ntoh24(fh.fh_d_id)))
    pub drop: goto,
    pub skb): fip->send(fip,,
    pub -EINPROGRESS: return,
    drop:
    LIBFCOE_FIP_DBG(fip, "drop els_send op %u d_id %x\n",
    pub ntoh24(fh->fh_d_id)): op,,
    pub -EINVAL: return,
    }
//
// fcoe_ctlr_age_fcfs() - Reset and free all old FCFs for a controller
// @fip: The FCoE controller to free FCFs on
//
// Called with lock held and preemption disabled.
//
// An FCF is considered old if we have missed two advertisements.
// That is, there have been no valid advertisement from it for 2.5
// times its keep-alive period.
//
// In addition, determine the time when an FCF selection can occur.
//
// Also, increment the MissDiscAdvCount when no advertisement is received
// for the corresponding FCF for 1.5 * FKA_ADV_PERIOD (FC-BB-5 LESB).
//
// Returns the time in jiffies for the next call.
//
#[no_mangle]
unsafe extern "C" fn fcoe_ctlr_age_fcfs(fip: *mut fcoe_ctlr) -> c_ulong {
    static unsigned long fcoe_ctlr_age_fcfs(struct fcoe_ctlr *fip)
    {
    pub fcf: *mut fcoe_fcf,
    pub next: *mut fcoe_fcf,
    pub msecs_to_jiffies(FIP_VN_KA_PERIOD): unsigned long next_timer = jiffies +,
    pub deadline: c_ulong,
    pub 0: unsigned long sel_time =,
    pub del_list: list_head,
    list_for_each_entry_safe(fcf, next, &fip.fcfs, list) {
    pub 2: deadline = fcf->time + fcf->fka_period + fcf->fka_period /,
    if (fip.sel_fcf == fcf) {
    if (time_after(jiffies, deadline)) {
    pub miss_cnt: u64,
    pub this_cpu_inc_return(fip->lp->stats->MissDiscAdvCount): miss_cnt =,
    printk(KERN_INFO "libfcoe: host%d: "
    "Missing Discovery Advertisement "
    "for fab %16.16llx count %lld\n",
    fip.lp.host.host_no, fcf.fabric_name,
    } else if (time_after(next_timer, deadline))
    pub deadline: next_timer =,
    }
    pub fcf->fka_period: deadline +=,
    if (time_after_eq(jiffies, deadline)) {
    if (fip.sel_fcf == fcf)
    pub NULL: fip->sel_fcf =,
//
// Move to delete list so we can call
// fcoe_sysfs_fcf_del (which can sleep)
// after the put_cpu().
//
    pub &del_list): list_add(&fcf->list,,
    } else {
    if (time_after(next_timer, deadline))
    pub deadline: next_timer =,
    if (fcoe_ctlr_mtu_valid(fcf) &&
    (!sel_time || time_before(sel_time, fcf.time)))
    pub fcf->time: sel_time =,
    }
    }
    list_for_each_entry_safe(fcf, next, &del_list, list) {
// Removes fcf from current list
    }
    if (sel_time && !fip.sel_fcf && !fip.sel_time) {
    pub msecs_to_jiffies(FCOE_CTLR_START_DELAY): sel_time +=,
    pub sel_time: fip->sel_time =,
    }
    pub next_timer: return,
    }
//
// fcoe_ctlr_parse_adv() - Decode a FIP advertisement into a new FCF entry
// @fip: The FCoE controller receiving the advertisement
// @skb: The received FIP advertisement frame
// @fcf: The resulting FCF entry
//
// Returns zero on a valid parsed advertisement,
// otherwise returns non zero value.
//
    static int fcoe_ctlr_parse_adv(struct fcoe_ctlr *fip,
    struct sk_buff *skb, struct fcoe_fcf *fcf)
    {
    pub fiph: *mut fip_header,
    pub NULL: *mut *mut fip_desc desc =,
    pub wwn: *mut fip_wwn_desc,
    pub fab: *mut fip_fab_desc,
    pub fka: *mut fip_fka_desc,
    pub t: c_ulong,
    pub rlen: usize,
    pub dlen: usize,
    pub desc_mask: u32,
    pub sizeof(*fcf)): *mut memset(fcf, 0,,
    pub msecs_to_jiffies(FCOE_CTLR_DEF_FKA): fcf->fka_period =,
    pub )skb->data: *mut fiph = (struct fip_header,
    pub ntohs(fiph->fip_flags): fcf->flags =,
//
// mask of required descriptors. validating each one clears its bit.
//
    desc_mask = BIT(FIP_DT_PRI) | BIT(FIP_DT_MAC) | BIT(FIP_DT_NAME) |
    pub BIT(FIP_DT_FKA): BIT(FIP_DT_FAB) |,
    pub 4: *mut *mut rlen = ntohs(fiph->fip_dl_len),
    if (rlen + sizeof(*fiph) > skb.len)
    pub -EINVAL: return,
    pub 1): *mut *mut desc = (struct fip_desc )(fiph +,
    while (rlen > 0) {
    pub FIP_BPW: *mut *mut dlen = desc->fip_dlen,
    if (dlen < sizeof(*desc) || dlen > rlen)
    pub -EINVAL: return,
// Drop Adv if there are duplicate critical descriptors
    if ((desc.fip_dtype < 32) &&
    !(desc_mask & 1U << desc.fip_dtype)) {
    LIBFCOE_FIP_DBG(fip, "Duplicate Critical "
    pub adv\n"): "Descriptors in FIP,
    pub -EINVAL: return,
    }
    switch (desc.fip_dtype) {
    case FIP_DT_PRI:
    if (dlen != sizeof(struct fip_pri_desc))
    pub len_err: goto,
    pub )desc)->fd_pri: *mut fcf->pri = ((struct fip_pri_desc,
    pub ~BIT(FIP_DT_PRI): desc_mask &=,
    case FIP_DT_MAC:
    if (dlen != sizeof(struct fip_mac_desc))
    pub len_err: goto,
    memcpy(fcf.fcf_mac,
    ((struct fip_mac_desc *)desc).fd_mac,
    pub ETH_ALEN): memcpy(fcf->fcoe_mac, fcf->fcf_mac,,
    if (!is_valid_ether_addr(fcf.fcf_mac)) {
    LIBFCOE_FIP_DBG(fip,
    "Invalid MAC addr %pM in FIP adv\n",
    pub -EINVAL: return,
    }
    pub ~BIT(FIP_DT_MAC): desc_mask &=,
    case FIP_DT_NAME:
    if (dlen != sizeof(struct fip_wwn_desc))
    pub len_err: goto,
    pub )desc: *mut wwn = (struct fip_wwn_desc,
    pub get_unaligned_be64(&wwn->fd_wwn): fcf->switch_name =,
    pub ~BIT(FIP_DT_NAME): desc_mask &=,
    case FIP_DT_FAB:
    if (dlen != sizeof(struct fip_fab_desc))
    pub len_err: goto,
    pub )desc: *mut fab = (struct fip_fab_desc,
    pub get_unaligned_be64(&fab->fd_wwn): fcf->fabric_name =,
    pub ntohs(fab->fd_vfid): fcf->vfid =,
    pub ntoh24(fab->fd_map): fcf->fc_map =,
    pub ~BIT(FIP_DT_FAB): desc_mask &=,
    case FIP_DT_FKA:
    if (dlen != sizeof(struct fip_fka_desc))
    pub len_err: goto,
    pub )desc: *mut fka = (struct fip_fka_desc,
    if (fka.fd_flags & FIP_FKA_ADV_D)
    pub 1: fcf->fd_flags =,
    pub ntohl(fka->fd_fka_period): t =,
    if (t >= FCOE_CTLR_MIN_FKA)
    pub msecs_to_jiffies(t): fcf->fka_period =,
    pub ~BIT(FIP_DT_FKA): desc_mask &=,
    case FIP_DT_MAP_OUI:
    case FIP_DT_FCOE_SIZE:
    case FIP_DT_FLOGI:
    case FIP_DT_FDISC:
    case FIP_DT_LOGO:
    case FIP_DT_ELP:
    default:
    LIBFCOE_FIP_DBG(fip, "unexpected descriptor type %x "
    pub desc->fip_dtype): "in FIP adv\n",,
// standard says ignore unknown descriptors >= 128
    if (desc.fip_dtype < FIP_DT_NON_CRITICAL)
    pub -EINVAL: return,
    }
    pub dlen): *mut *mut *mut desc = (struct fip_desc )((char )desc +,
    pub dlen: rlen -=,
    }
    if (!fcf.fc_map || (fcf.fc_map & 0x10000))
    pub -EINVAL: return,
    if (!fcf.switch_name)
    pub -EINVAL: return,
    if (desc_mask) {
    LIBFCOE_FIP_DBG(fip, "adv missing descriptors mask %x\n",
    pub -EINVAL: return,
    }
    pub 0: return,
    len_err:
    LIBFCOE_FIP_DBG(fip, "FIP length error in descriptor type %x len %zu\n",
    pub dlen): desc->fip_dtype,,
    pub -EINVAL: return,
    }
//
// fcoe_ctlr_recv_adv() - Handle an incoming advertisement
// @fip: The FCoE controller receiving the advertisement
// @skb: The received FIP packet
//
#[no_mangle]
unsafe extern "C" fn fcoe_ctlr_recv_adv(fip: *mut fcoe_ctlr, skb: *mut sk_buff) {
    static void fcoe_ctlr_recv_adv(struct fcoe_ctlr *fip, struct sk_buff *skb)
    {
    pub fcf: *mut fcoe_fcf,
    pub new: fcoe_fcf,
    pub msecs_to_jiffies(FCOE_CTLR_SOL_TOV): unsigned long sol_tov =,
    pub 0: int first =,
    pub mtu_valid: c_int,
    pub 0: int found =,
    pub 0: int rc =,
    if (fcoe_ctlr_parse_adv(fip, skb, &new))
    pub list_empty(&fip->fcfs): first =,
    list_for_each_entry(fcf, &fip.fcfs, list) {
    if (fcf.switch_name == new.switch_name &&
    fcf.fabric_name == new.fabric_name &&
    fcf.fc_map == new.fc_map &&
    ether_addr_equal(fcf.fcf_mac, new.fcf_mac)) {
    pub 1: found =,
    }
    }
    if (!found) {
    if (fip.fcf_count >= FCOE_CTLR_FCF_LIMIT)
    pub out: goto,
    pub GFP_ATOMIC): *mut *mut fcf = kmalloc_obj(fcf,,
    if (!fcf)
    pub out: goto,
    pub sizeof(new)): memcpy(fcf, &new,,
    pub fip: fcf->fip =,
    pub fcoe_sysfs_fcf_add(fcf): rc =,
    if (rc) {
    printk(KERN_ERR "Failed to allocate sysfs instance "
    "for FCF, fab %16.16llx mac %pM\n",
    pub new.fcf_mac): new.fabric_name,,
    pub out: goto,
    }
    } else {
//
// Update the FCF's keep-alive descriptor flags.
// Other flag changes from new advertisements are
// ignored after a solicited advertisement is
// received and the FCF is selectable (usable).
//
    pub new.fd_flags: fcf->fd_flags =,
    if (!fcoe_ctlr_fcf_usable(fcf))
    pub new.flags: fcf->flags =,
    if (fcf == fip.sel_fcf && !fcf.fd_flags) {
    pub fcf->fka_period: fip->ctlr_ka_time -=,
    pub new.fka_period: fip->ctlr_ka_time +=,
    if (time_before(fip.ctlr_ka_time, fip.timer.expires))
    pub fip->ctlr_ka_time): mod_timer(&fip->timer,,
    }
    pub new.fka_period: fcf->fka_period =,
    pub ETH_ALEN): memcpy(fcf->fcf_mac, new.fcf_mac,,
    }
    pub fcoe_ctlr_mtu_valid(fcf): mtu_valid =,
    pub jiffies: fcf->time =,
    if (!found)
    LIBFCOE_FIP_DBG(fip, "New FCF fab %16.16llx mac %pM\n",
    pub fcf->fcf_mac): fcf->fabric_name,,
//
// If this advertisement is not solicited and our max receive size
// hasn't been verified, send a solicited advertisement.
//
    if (!mtu_valid)
    pub fcf): fcoe_ctlr_solicit(fip,,
//
// If its been a while since we did a solicit, and this is
// the first advertisement we've received, do a multicast
// solicitation to gather as many advertisements as we can
// before selection occurs.
//
    if (first && time_after(jiffies, fip.sol_time + sol_tov))
    pub NULL): fcoe_ctlr_solicit(fip,,
//
// Put this FCF at the head of the list for priority among equals.
// This helps in the case of an NPV switch which insists we use
// the FCF that answers multicast solicitations, not the others that
// are sending periodic multicast advertisements.
//
    if (mtu_valid)
    pub &fip->fcfs): list_move(&fcf->list,,
//
// If this is the first validated FCF, note the time and
// set a timer to trigger selection.
//
    if (mtu_valid && !fip.sel_fcf && !fip.sel_time &&
    fcoe_ctlr_fcf_usable(fcf)) {
    fip.sel_time = jiffies +
    if (!timer_pending(&fip.timer) ||
    time_before(fip.sel_time, fip.timer.expires))
    pub fip->sel_time): mod_timer(&fip->timer,,
    }
    out:
    }
//
// fcoe_ctlr_recv_els() - Handle an incoming FIP encapsulated ELS frame
// @fip: The FCoE controller which received the packet
// @skb: The received FIP packet
//
#[no_mangle]
unsafe extern "C" fn fcoe_ctlr_recv_els(fip: *mut fcoe_ctlr, skb: *mut sk_buff) {
    static void fcoe_ctlr_recv_els(struct fcoe_ctlr *fip, struct sk_buff *skb)
    {
    pub fip->lp: *mut *mut fc_lport lport =,
    pub fiph: *mut fip_header,
    pub )skb: *mut *mut fc_frame fp = (fc_frame,
    pub NULL: *mut *mut fc_frame_header fh =,
    pub desc: *mut fip_desc,
    pub els: *mut fip_encaps,
    pub sel: *mut fcoe_fcf,
    pub 0: enum fip_desc_type els_dtype =,
    pub els_op: u8,
    pub sub: u8,
    pub }: u8 granted_mac[ETH_ALEN] = { 0,
    pub 0: size_t els_len =,
    pub rlen: usize,
    pub dlen: usize,
    pub 0: u32 desc_mask =,
    pub 0: u32 desc_cnt =,
    pub )skb->data: *mut fiph = (struct fip_header,
    pub fiph->fip_subcode: sub =,
    if (sub != FIP_SC_REQ && sub != FIP_SC_REP)
    pub drop: goto,
    pub 4: *mut *mut rlen = ntohs(fiph->fip_dl_len),
    if (rlen + sizeof(*fiph) > skb.len)
    pub drop: goto,
    pub 1): *mut *mut desc = (struct fip_desc )(fiph +,
    while (rlen > 0) {
    pub FIP_BPW: *mut *mut dlen = desc->fip_dlen,
    if (dlen < sizeof(*desc) || dlen > rlen)
    pub drop: goto,
// Drop ELS if there are duplicate critical descriptors
    if (desc.fip_dtype < 32) {
    if ((desc.fip_dtype != FIP_DT_MAC) &&
    (desc_mask & 1U << desc.fip_dtype)) {
    LIBFCOE_FIP_DBG(fip, "Duplicate Critical "
    pub ELS\n"): "Descriptors in FIP,
    pub drop: goto,
    }
    pub desc->fip_dtype): desc_mask |= (1 <<,
    }
    switch (desc.fip_dtype) {
    case FIP_DT_MAC:
    pub fip->sel_fcf: sel =,
    if (desc_cnt == 1) {
    LIBFCOE_FIP_DBG(fip, "FIP descriptors "
    pub order\n"): "received out of,
    pub drop: goto,
    }
//
// Some switch implementations send two MAC descriptors,
// with first MAC(granted_mac) being the FPMA, and the
// second one(fcoe_mac) is used as destination address
// for sending/receiving FCoE packets. FIP traffic is
// sent using fip_mac. For regular switches, both
// fip_mac and fcoe_mac would be the same.
//
    if (desc_cnt == 2)
    memcpy(granted_mac,
    ((struct fip_mac_desc *)desc).fd_mac,
    if (dlen != sizeof(struct fip_mac_desc))
    pub len_err: goto,
    if ((desc_cnt == 3) && (sel))
    memcpy(sel.fcoe_mac,
    ((struct fip_mac_desc *)desc).fd_mac,
    case FIP_DT_FLOGI:
    case FIP_DT_FDISC:
    case FIP_DT_LOGO:
    case FIP_DT_ELP:
    if (desc_cnt != 1) {
    LIBFCOE_FIP_DBG(fip, "FIP descriptors "
    pub order\n"): "received out of,
    pub drop: goto,
    }
    if (fh)
    pub drop: goto,
    if (dlen < sizeof(*els) + sizeof(*fh) + 1)
    pub len_err: goto,
    pub sizeof(*els): *mut els_len = dlen -,
    pub )desc: *mut els = (struct fip_encaps,
    pub 1): *mut *mut fh = (struct fc_frame_header )(els +,
    pub desc->fip_dtype: els_dtype =,
    default:
    LIBFCOE_FIP_DBG(fip, "unexpected descriptor type %x "
    pub desc->fip_dtype): "in FIP adv\n",,
// standard says ignore unknown descriptors >= 128
    if (desc.fip_dtype < FIP_DT_NON_CRITICAL)
    pub drop: goto,
    if (desc_cnt <= 2) {
    LIBFCOE_FIP_DBG(fip, "FIP descriptors "
    pub order\n"): "received out of,
    pub drop: goto,
    }
    }
    pub dlen): *mut *mut *mut desc = (struct fip_desc )((char )desc +,
    pub dlen: rlen -=,
    }
    if (!fh)
    pub drop: goto,
    pub 1): *mut *mut *mut els_op = (u8 )(fh +,
    if ((els_dtype == FIP_DT_FLOGI || els_dtype == FIP_DT_FDISC) &&
    sub == FIP_SC_REP && fip.mode != FIP_MODE_VN2VN) {
    if (els_op == ELS_LS_ACC) {
    if (!is_valid_ether_addr(granted_mac)) {
    LIBFCOE_FIP_DBG(fip,
    "Invalid MAC address %pM in FIP ELS\n",
    pub drop: goto,
    }
    pub ETH_ALEN): memcpy(fr_cb(fp)->granted_mac, granted_mac,,
    if (fip.flogi_oxid == ntohs(fh.fh_ox_id)) {
    pub FC_XID_UNKNOWN: fip->flogi_oxid =,
    if (els_dtype == FIP_DT_FLOGI)
    }
    } else if (els_dtype == FIP_DT_FLOGI &&
    !fcoe_ctlr_flogi_retry(fip))
    pub /: *mut *mut goto drop; / retrying FLOGI so drop reject,
    }
    if ((desc_cnt == 0) || ((els_op != ELS_LS_RJT) &&
    (!(1U << FIP_DT_MAC & desc_mask)))) {
    LIBFCOE_FIP_DBG(fip, "Missing critical descriptors "
    pub ELS\n"): "in FIP,
    pub drop: goto,
    }
//
// Convert skb into an fc_frame containing only the ELS.
//
    pub skb->data): *mut *mut skb_pull(skb, (u8 )fh -,
    pub els_len): skb_trim(skb,,
    pub )skb: *mut fp = (struct fc_frame,
    pub FC_SOF_I3: fr_sof(fp) =,
    pub FC_EOF_T: fr_eof(fp) =,
    pub lport: fr_dev(fp) =,
    pub els_dtype: fr_encaps(fp) =,
    pub FIP_BPW): this_cpu_add(lport->stats->RxWords, skb->len /,
    pub fp): fc_exch_recv(lport,,
    len_err:
    LIBFCOE_FIP_DBG(fip, "FIP length error in descriptor type %x len %zu\n",
    pub dlen): desc->fip_dtype,,
    drop:
    }
//
// fcoe_ctlr_recv_clr_vlink() - Handle an incoming link reset frame
// @fip: The FCoE controller that received the frame
// @skb: The received FIP packet
//
// There may be multiple VN_Port descriptors.
// The overall length has already been checked.
//
    static void fcoe_ctlr_recv_clr_vlink(struct fcoe_ctlr *fip,
    struct sk_buff *skb)
    {
    pub desc: *mut fip_desc,
    pub mp: *mut fip_mac_desc,
    pub wp: *mut fip_wwn_desc,
    pub vp: *mut fip_vn_desc,
    pub rlen: usize,
    pub dlen: usize,
    pub fip->sel_fcf: *mut *mut fcoe_fcf fcf =,
    pub fip->lp: *mut *mut fc_lport lport =,
    pub NULL: *mut *mut fc_lport vn_port =,
    pub desc_mask: u32,
    pub num_vlink_desc: c_int,
    pub 0: int reset_phys_port =,
    pub NULL: *mut *mut *mut fip_vn_desc vlink_desc_arr =,
    pub )skb->data: *mut *mut fip_header fh = (fip_header,
    pub eth_hdr(skb): *mut *mut ethhdr eh =,
    pub received\n"): LIBFCOE_FIP_DBG(fip, "Clear Virtual Link,
    if (!fcf) {
//
// We are yet to select best FCF, but we got CVL in the
// meantime. reset the ctlr and let it rediscover the FCF
//
    LIBFCOE_FIP_DBG(fip, "Resetting fcoe_ctlr as FCF has not been "
    pub yet\n"): "selected,
    }
//
// If we've selected an FCF check that the CVL is from there to avoid
// processing CVLs from an unexpected source.  If it is from an
// unexpected source drop it on the floor.
//
    if (!ether_addr_equal(eh.h_source, fcf.fcf_mac)) {
    LIBFCOE_FIP_DBG(fip, "Dropping CVL due to source address "
    pub eh->h_source): "mismatch with FCF src=%pM\n",,
    }
//
// If we haven't logged into the fabric but receive a CVL we should
// reset everything and go back to solicitation.
//
    if (!lport.port_id) {
    pub resoliciting\n"): LIBFCOE_FIP_DBG(fip, "lport not logged in,,
    pub NULL): fcoe_ctlr_solicit(fip,,
    }
//
// mask of required descriptors.  Validating each one clears its bit.
//
    pub BIT(FIP_DT_NAME): desc_mask = BIT(FIP_DT_MAC) |,
    pub FIP_BPW: *mut *mut rlen = ntohs(fh->fip_dl_len),
    pub 1): *mut *mut desc = (struct fip_desc )(fh +,
//
// Actually need to subtract 'sizeof(*mp) - sizeof(*wp)' from 'rlen'
// before determining max Vx_Port descriptor but a buggy FCF could have
// omitted either or both MAC Address and Name Identifier descriptors
//
    pub sizeof(*vp): *mut num_vlink_desc = rlen /,
    if (num_vlink_desc)
    pub GFP_ATOMIC): vlink_desc_arr = kmalloc_objs(vp, num_vlink_desc,,
    if (!vlink_desc_arr)
    pub 0: num_vlink_desc =,
    while (rlen >= sizeof(*desc)) {
    pub FIP_BPW: *mut *mut dlen = desc->fip_dlen,
    if (dlen < sizeof(*desc) || dlen > rlen)
    pub err: goto,
// Drop CVL if there are duplicate critical descriptors
    if ((desc.fip_dtype < 32) &&
    (desc.fip_dtype != FIP_DT_VN_ID) &&
    !(desc_mask & 1U << desc.fip_dtype)) {
    LIBFCOE_FIP_DBG(fip, "Duplicate Critical "
    pub CVL\n"): "Descriptors in FIP,
    pub err: goto,
    }
    switch (desc.fip_dtype) {
    case FIP_DT_MAC:
    pub )desc: *mut mp = (struct fip_mac_desc,
    if (dlen < sizeof(*mp))
    pub err: goto,
    if (!ether_addr_equal(mp.fd_mac, fcf.fcf_mac))
    pub err: goto,
    pub ~BIT(FIP_DT_MAC): desc_mask &=,
    case FIP_DT_NAME:
    pub )desc: *mut wp = (struct fip_wwn_desc,
    if (dlen < sizeof(*wp))
    pub err: goto,
    if (get_unaligned_be64(&wp.fd_wwn) != fcf.switch_name)
    pub err: goto,
    pub ~BIT(FIP_DT_NAME): desc_mask &=,
    case FIP_DT_VN_ID:
    pub )desc: *mut vp = (struct fip_vn_desc,
    if (dlen < sizeof(*vp))
    pub err: goto,
    pub vp: vlink_desc_arr[num_vlink_desc++] =,
    vn_port = fc_vport_id_lookup(lport,
    if (vn_port && (vn_port == lport)) {
    }
    default:
// standard says ignore unknown descriptors >= 128
    if (desc.fip_dtype < FIP_DT_NON_CRITICAL)
    pub err: goto,
    }
    pub dlen): *mut *mut *mut desc = (struct fip_desc )((char )desc +,
    pub dlen: rlen -=,
    }
//
// reset only if all required descriptors were present and valid.
//
    if (desc_mask)
    LIBFCOE_FIP_DBG(fip, "missing descriptors mask %x\n",
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !num_vlink_desc) -> else {
    pub found\n"): LIBFCOE_FIP_DBG(fip, "CVL: no Vx_Port descriptor,
//
// No Vx_Port description. Clear all NPIV ports,
// followed by physical port
//
    list_for_each_entry(vn_port, &lport.vports, list)
    pub NULL): fcoe_ctlr_solicit(fip,,
    } else {
    pub i: c_int,
    pub Link\n"): LIBFCOE_FIP_DBG(fip, "performing Clear Virtual,
    pub {: for (i = 0; i < num_vlink_desc; i++),
    pub vlink_desc_arr: [vp =; i],
    vn_port = fc_vport_id_lookup(lport,
    if (!vn_port)
//
// 'port_id' is already validated, check MAC address and
// wwpn
//
    if (!ether_addr_equal(fip.get_src_addr(vn_port),
    vp.fd_mac) ||
    get_unaligned_be64(&vp.fd_wwpn) !=
    vn_port.wwpn)
    if (vn_port == lport)
//
// Physical port, defer processing till all
// listed NPIV ports are cleared
//
    pub 1: reset_phys_port =,
    else    /* NPIV port */
    }
    if (reset_phys_port) {
    pub NULL): fcoe_ctlr_solicit(fip,,
    }
    }
    err:
    }
//
// fcoe_ctlr_recv() - Receive a FIP packet
// @fip: The FCoE controller that received the packet
// @skb: The received FIP packet
//
// This may be called from either NET_RX_SOFTIRQ or IRQ.
//
#[no_mangle]
pub unsafe extern "C" fn fcoe_ctlr_recv(fip: *mut fcoe_ctlr, skb: *mut sk_buff) {
    void fcoe_ctlr_recv(struct fcoe_ctlr *fip, struct sk_buff *skb)
    {
    pub GFP_ATOMIC): skb = skb_share_check(skb,,
    if (!skb)
    pub skb): skb_queue_tail(&fip->fip_recv_list,,
    }
//
// fcoe_ctlr_recv_handler() - Receive a FIP frame
// @fip: The FCoE controller that received the frame
// @skb: The received FIP frame
//
// Returns non-zero if the frame is dropped.
//
#[no_mangle]
unsafe extern "C" fn fcoe_ctlr_recv_handler(fip: *mut fcoe_ctlr, skb: *mut sk_buff) -> c_int {
    static int fcoe_ctlr_recv_handler(struct fcoe_ctlr *fip, struct sk_buff *skb)
    {
    pub fiph: *mut fip_header,
    pub eh: *mut ethhdr,
    pub state: enum fip_state,
    pub false: bool fip_vlan_resp =,
    pub op: u16,
    pub sub: u8,
    if (skb_linearize(skb))
    pub drop: goto,
    if (skb.len < sizeof(*fiph))
    pub drop: goto,
    pub eth_hdr(skb): eh =,
    if (fip.mode == FIP_MODE_VN2VN) {
    if (!ether_addr_equal(eh.h_dest, fip.ctl_src_addr) &&
    !ether_addr_equal(eh.h_dest, fcoe_all_vn2vn) &&
    !ether_addr_equal(eh.h_dest, fcoe_all_p2p))
    pub drop: goto,
    } else if (!ether_addr_equal(eh.h_dest, fip.ctl_src_addr) &&
    !ether_addr_equal(eh.h_dest, fcoe_all_enode))
    pub drop: goto,
    pub )skb->data: *mut fiph = (struct fip_header,
    pub ntohs(fiph->fip_op): op =,
    pub fiph->fip_subcode: sub =,
    if (FIP_VER_DECAPS(fiph.fip_ver) != FIP_VER)
    pub drop: goto,
    if (ntohs(fiph.fip_dl_len) * FIP_BPW + sizeof(*fiph) > skb.len)
    pub drop: goto,
    pub fip->state: state =,
    if (state == FIP_ST_AUTO) {
    pub 0: fip->map_dest =,
    pub FIP_ST_ENABLED): fcoe_ctlr_set_state(fip,,
    pub FIP_ST_ENABLED: state =,
    pub mode\n"): LIBFCOE_FIP_DBG(fip, "Using FIP,
    }
    pub fip->fip_resp: fip_vlan_resp =,
    if (fip.mode == FIP_MODE_VN2VN && op == FIP_OP_VN2VN)
    pub skb): return fcoe_ctlr_vn_recv(fip,,
    if (fip_vlan_resp && op == FIP_OP_VLAN) {
    pub discovery\n"): LIBFCOE_FIP_DBG(fip, "fip vlan,
    pub skb): return fcoe_ctlr_vlan_recv(fip,,
    }
    if (state != FIP_ST_ENABLED && state != FIP_ST_VNMP_UP &&
    state != FIP_ST_VNMP_CLAIM)
    pub drop: goto,
    if (op == FIP_OP_LS) {
    pub /: *mut *mut fcoe_ctlr_recv_els(fip, skb); / consumes skb,
    pub 0: return,
    }
    if (state != FIP_ST_ENABLED)
    pub drop: goto,
    if (op == FIP_OP_DISC && sub == FIP_SC_ADV)
    pub skb): fcoe_ctlr_recv_adv(fip,,
#[no_mangle]
pub unsafe extern "C" fn if(FIP_SC_CLR_VLINK: op == FIP_OP_CTRL && sub ==) -> else {
    else if (op == FIP_OP_CTRL && sub == FIP_SC_CLR_VLINK)
    pub skb): fcoe_ctlr_recv_clr_vlink(fip,,
    pub 0: return,
    drop:
    pub -1: return,
    }
//
// fcoe_ctlr_select() - Select the best FCF (if possible)
// @fip: The FCoE controller
//
// Returns the selected FCF, or NULL if none are usable.
//
// If there are conflicting advertisements, no FCF can be chosen.
//
// If there is already a selected FCF, this will choose a better one or
// an equivalent one that hasn't already been sent a FLOGI.
//
// Called with lock held.
//
    static struct fcoe_fcf *fcoe_ctlr_select(struct fcoe_ctlr *fip)
    {
    pub fcf: *mut fcoe_fcf,
    pub fip->sel_fcf: *mut *mut fcoe_fcf best =,
    list_for_each_entry(fcf, &fip.fcfs, list) {
    LIBFCOE_FIP_DBG(fip, "consider FCF fab %16.16llx "
    "VFID %d mac %pM map %x val %d "
    "sent %u pri %u\n",
    fcf.fabric_name, fcf.vfid, fcf.fcf_mac,
    fcf.fc_map, fcoe_ctlr_mtu_valid(fcf),
    pub fcf->pri): fcf->flogi_sent,,
    if (!fcoe_ctlr_fcf_usable(fcf)) {
    LIBFCOE_FIP_DBG(fip, "FCF for fab %16.16llx "
    "map %x %svalid %savailable\n",
    fcf.fabric_name, fcf.fc_map,
    (fcf.flags & FIP_FL_SOL) ? "" : "in",
    (fcf.flags & FIP_FL_AVAIL) ?
    pub "un"): "" :,
    }
    if (!best || fcf.pri < best.pri || best.flogi_sent)
    pub fcf: best =,
    if (fcf.fabric_name != best.fabric_name ||
    fcf.vfid != best.vfid ||
    fcf.fc_map != best.fc_map) {
    LIBFCOE_FIP_DBG(fip, "Conflicting fabric, VFID, "
    pub FC-MAP\n"): "or,
    pub NULL: return,
    }
    }
    pub best: fip->sel_fcf =,
    if (best) {
    pub best->fcf_mac): LIBFCOE_FIP_DBG(fip, "using FCF mac %pM\n",,
    fip.port_ka_time = jiffies +
    pub best->fka_period: fip->ctlr_ka_time = jiffies +,
    if (time_before(fip.ctlr_ka_time, fip.timer.expires))
    pub fip->ctlr_ka_time): mod_timer(&fip->timer,,
    }
    pub best: return,
    }
//
// fcoe_ctlr_flogi_send_locked() - send FIP-encapsulated FLOGI to current FCF
// @fip: The FCoE controller
//
// Returns non-zero error if it could not be sent.
//
// Called with ctlr_mutex and ctlr_lock held.
// Caller must verify that fip->sel_fcf is not NULL.
//
#[no_mangle]
unsafe extern "C" fn fcoe_ctlr_flogi_send_locked(fip: *mut fcoe_ctlr) -> c_int {
    static int fcoe_ctlr_flogi_send_locked(struct fcoe_ctlr *fip)
    {
    pub skb: *mut sk_buff,
    pub skb_orig: *mut sk_buff,
    pub fh: *mut fc_frame_header,
    pub error: c_int,
    pub fip->flogi_req: skb_orig =,
    if (!skb_orig)
    pub -EINVAL: return,
//
// Clone and send the FLOGI request.  If clone fails, use original.
//
    pub GFP_ATOMIC): skb = skb_clone(skb_orig,,
    if (!skb) {
    pub skb_orig: skb =,
    pub NULL: fip->flogi_req =,
    }
    pub )skb->data: *mut fh = (struct fc_frame_header,
    error = fcoe_ctlr_encaps(fip, fip.lp, FIP_DT_FLOGI, skb,
    if (error) {
    pub error: return,
    }
    pub skb): fip->send(fip,,
    pub 1: fip->sel_fcf->flogi_sent =,
    pub 0: return,
    }
//
// fcoe_ctlr_flogi_retry() - resend FLOGI request to a new FCF if possible
// @fip: The FCoE controller
//
// Returns non-zero error code if there's no FLOGI request to retry or
// no alternate FCF available.
//
#[no_mangle]
unsafe extern "C" fn fcoe_ctlr_flogi_retry(fip: *mut fcoe_ctlr) -> c_int {
    static int fcoe_ctlr_flogi_retry(struct fcoe_ctlr *fip)
    {
    pub fcf: *mut fcoe_fcf,
    pub error: c_int,
    pub reselect\n"): LIBFCOE_FIP_DBG(fip, "re-sending FLOGI -,
    pub fcoe_ctlr_select(fip): fcf =,
    if (!fcf || fcf.flogi_sent) {
    pub NULL: fip->flogi_req =,
    pub -ENOENT: error =,
    } else {
    pub NULL): fcoe_ctlr_solicit(fip,,
    pub fcoe_ctlr_flogi_send_locked(fip): error =,
    }
    pub error: return,
    }
//
// fcoe_ctlr_flogi_send() - Handle sending of FIP FLOGI.
// @fip: The FCoE controller that timed out
//
// Done here because fcoe_ctlr_els_send() can't get mutex.
//
// Called with ctlr_mutex held.  The caller must not hold ctlr_lock.
//
#[no_mangle]
unsafe extern "C" fn fcoe_ctlr_flogi_send(fip: *mut fcoe_ctlr) {
    static void fcoe_ctlr_flogi_send(struct fcoe_ctlr *fip)
    {
    pub fcf: *mut fcoe_fcf,
    pub fip->sel_fcf: fcf =,
    if (!fcf || !fip.flogi_req_send)
    pub unlock: goto,
    pub FLOGI\n"): LIBFCOE_FIP_DBG(fip, "sending,
//
// If this FLOGI is being sent due to a timeout retry
// to the same FCF as before, select a different FCF if possible.
//
    if (fcf.flogi_sent) {
    pub reselect\n"): LIBFCOE_FIP_DBG(fip, "sending FLOGI -,
    pub fcoe_ctlr_select(fip): fcf =,
    if (!fcf || fcf.flogi_sent) {
    pub clearing\n"): LIBFCOE_FIP_DBG(fip, "sending FLOGI -,
    list_for_each_entry(fcf, &fip.fcfs, list)
    pub 0: fcf->flogi_sent =,
    pub fcoe_ctlr_select(fip): fcf =,
    }
    }
    if (fcf) {
    pub 0: fip->flogi_req_send =,
    } else /* XXX */
    pub send\n"): LIBFCOE_FIP_DBG(fip, "No FCF selected - defer,
    unlock:
    }
//
// fcoe_ctlr_timeout() - FIP timeout handler
// @t: Timer context use to obtain the controller reference
//
#[no_mangle]
unsafe extern "C" fn fcoe_ctlr_timeout(t: *mut timer_list) {
    static void fcoe_ctlr_timeout(struct timer_list *t)
    {
    pub timer): *mut *mut fcoe_ctlr fip = timer_container_of(fip, t,,
    }
//
// fcoe_ctlr_timer_work() - Worker thread function for timer work
// @work: Handle to a FCoE controller
//
// Ages FCFs.  Triggers FCF selection if possible.
// Sends keep-alives and resets.
//
#[no_mangle]
unsafe extern "C" fn fcoe_ctlr_timer_work(work: *mut work_struct) {
    static void fcoe_ctlr_timer_work(struct work_struct *work)
    {
    pub fip: *mut fcoe_ctlr,
    pub vport: *mut fc_lport,
    pub mac: *mut u8,
    pub 0: u8 reset =,
    pub 0: u8 send_ctlr_ka =,
    pub 0: u8 send_port_ka =,
    pub sel: *mut fcoe_fcf,
    pub fcf: *mut fcoe_fcf,
    pub next_timer: c_ulong,
    pub timer_work): fip = container_of(work, struct fcoe_ctlr,,
    if (fip.mode == FIP_MODE_VN2VN)
    pub fcoe_ctlr_vn_timeout(fip): return,
    if (fip.state == FIP_ST_DISABLED) {
    }
    pub fip->sel_fcf: fcf =,
    pub fcoe_ctlr_age_fcfs(fip): next_timer =,
    pub fip->sel_fcf: sel =,
    if (!sel && fip.sel_time) {
    if (time_after_eq(jiffies, fip.sel_time)) {
    pub fcoe_ctlr_select(fip): sel =,
    pub 0: fip->sel_time =,
    } else if (time_after(next_timer, fip.sel_time))
    pub fip->sel_time: next_timer =,
    }
    if (sel && fip.flogi_req_send)
#[no_mangle]
pub unsafe extern "C" fn if(fcf: !sel &&) -> else {
    else if (!sel && fcf)
    pub 1: reset =,
    if (sel && !sel.fd_flags) {
    if (time_after_eq(jiffies, fip.ctlr_ka_time)) {
    pub sel->fka_period: fip->ctlr_ka_time = jiffies +,
    pub 1: send_ctlr_ka =,
    }
    if (time_after(next_timer, fip.ctlr_ka_time))
    pub fip->ctlr_ka_time: next_timer =,
    if (time_after_eq(jiffies, fip.port_ka_time)) {
    fip.port_ka_time = jiffies +
    pub 1: send_port_ka =,
    }
    if (time_after(next_timer, fip.port_ka_time))
    pub fip->port_ka_time: next_timer =,
    }
    if (!list_empty(&fip.fcfs))
    pub next_timer): mod_timer(&fip->timer,,
    if (reset) {
// restart things with a solicitation
    pub NULL): fcoe_ctlr_solicit(fip,,
    }
    if (send_ctlr_ka)
    pub fip->ctl_src_addr): fcoe_ctlr_send_keep_alive(fip, NULL, 0,,
    if (send_port_ka) {
    pub fip->get_src_addr(fip->lp): mac =,
    pub mac): fcoe_ctlr_send_keep_alive(fip, fip->lp, 1,,
    list_for_each_entry(vport, &fip.lp.vports, list) {
    pub fip->get_src_addr(vport): mac =,
    pub mac): fcoe_ctlr_send_keep_alive(fip, vport, 1,,
    }
    }
    }
//
// fcoe_ctlr_recv_work() - Worker thread function for receiving FIP frames
// @recv_work: Handle to a FCoE controller
//
#[no_mangle]
unsafe extern "C" fn fcoe_ctlr_recv_work(recv_work: *mut work_struct) {
    static void fcoe_ctlr_recv_work(struct work_struct *recv_work)
    {
    pub fip: *mut fcoe_ctlr,
    pub skb: *mut sk_buff,
    pub recv_work): fip = container_of(recv_work, struct fcoe_ctlr,,
    while ((skb = skb_dequeue(&fip.fip_recv_list)))
    pub skb): fcoe_ctlr_recv_handler(fip,,
    }
//
// fcoe_ctlr_recv_flogi() - Snoop pre-FIP receipt of FLOGI response
// @fip: The FCoE controller
// @lport: The local port
// @fp:	 The FC frame to snoop
//
// Snoop potential response to FLOGI or even incoming FLOGI.
//
// The caller has checked that we are waiting for login as indicated
// by fip->flogi_oxid != FC_XID_UNKNOWN.
//
// The caller is responsible for freeing the frame.
// Fill in the granted_mac address.
//
// Return non-zero if the frame should not be delivered to libfc.
//
    int fcoe_ctlr_recv_flogi(struct fcoe_ctlr *fip, struct fc_lport *lport,
    struct fc_frame *fp)
    {
    pub fh: *mut fc_frame_header,
    pub op: u8,
    pub sa: *mut u8,
    pub eth_hdr(&fp->skb)->h_source: sa =,
    pub fc_frame_header_get(fp): fh =,
    if (fh.fh_type != FC_TYPE_ELS)
    pub 0: return,
    pub fc_frame_payload_op(fp): op =,
    if (op == ELS_LS_ACC && fh.fh_r_ctl == FC_RCTL_ELS_REP &&
    fip.flogi_oxid == ntohs(fh.fh_ox_id)) {
    if (fip.state != FIP_ST_AUTO && fip.state != FIP_ST_NON_FIP) {
    pub -EINVAL: return,
    }
    pub FIP_ST_NON_FIP): fcoe_ctlr_set_state(fip,,
    LIBFCOE_FIP_DBG(fip,
    pub mode\n"): "received FLOGI LS_ACC using non-FIP,
//
// FLOGI accepted.
// If the src mac addr is FC_OUI-based, then we mark the
// address_mode flag to use FC_OUI-based Ethernet DA.
// Otherwise we use the FCoE gateway addr
//
    if (ether_addr_equal(sa, (u8[6])FC_FCOE_FLOGI_MAC)) {
    } else {
    pub ETH_ALEN): memcpy(fip->dest_addr, sa,,
    pub 0: fip->map_dest =,
    }
    pub FC_XID_UNKNOWN: fip->flogi_oxid =,
    pub fh->fh_d_id): fc_fcoe_set_mac(fr_cb(fp)->granted_mac,,
    } else if (op == ELS_FLOGI && fh.fh_r_ctl == FC_RCTL_ELS_REQ && sa) {
//
// Save source MAC for point-to-point responses.
//
    if (fip.state == FIP_ST_AUTO || fip.state == FIP_ST_NON_FIP) {
    pub ETH_ALEN): memcpy(fip->dest_addr, sa,,
    pub 0: fip->map_dest =,
    if (fip.state == FIP_ST_AUTO)
    LIBFCOE_FIP_DBG(fip, "received non-FIP FLOGI. "
    pub mode\n"): "Setting non-FIP,
    pub FIP_ST_NON_FIP): fcoe_ctlr_set_state(fip,,
    }
    }
    pub 0: return,
    }
//
// fcoe_wwn_from_mac() - Converts a 48-bit IEEE MAC address to a 64-bit FC WWN
// @mac:    The MAC address to convert
// @scheme: The scheme to use when converting
// @port:   The port indicator for converting
//
// Returns: u64 fc world wide name
//
    u64 fcoe_wwn_from_mac(unsigned char mac[ETH_ALEN],
    unsigned int scheme, unsigned int port)
    {
    pub wwn: u64,
    pub host_mac: u64,
// The MAC is in NO, so flip only the low 48 bits
    host_mac = ((u64) mac[0] << 40) |
    ((u64) mac[1] << 32) |
    ((u64) mac[2] << 24) |
    ((u64) mac[3] << 16) |
    ((u64) mac[4] << 8) |
    pub mac: [(u64); 5],
    pub 48)): WARN_ON(host_mac >= (1ULL <<,
    pub 60): wwn = host_mac | ((u64) scheme <<,
    switch (scheme) {
    case 1:
    pub 0): WARN_ON(port !=,
    case 2:
    pub 0xfff): WARN_ON(port >=,
    pub 48: wwn |= (u64) port <<,
    default:
    }
    pub wwn: return,
    }
//
// fcoe_ctlr_rport() - return the fcoe_rport for a given fc_rport_priv
// @rdata: libfc remote port
//
    static inline struct fcoe_rport *fcoe_ctlr_rport(struct fc_rport_priv *rdata)
    {
    pub rdata): return container_of(rdata, struct fcoe_rport,,
    }
//
// fcoe_ctlr_vn_send() - Send a FIP VN2VN Probe Request or Reply.
// @fip: The FCoE controller
// @sub: sub-opcode for probe request, reply, or advertisement.
// @dest: The destination Ethernet MAC address
// @min_len: minimum size of the Ethernet payload to be sent
//
    static void fcoe_ctlr_vn_send(struct fcoe_ctlr *fip,
    enum fip_vn2vn_subcode sub,
    const u8 *dest, size_t min_len)
    {
    pub skb: *mut sk_buff,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fip_vn2vn_probe_frame {
    pub eth: ethhdr,
    pub fip: fip_header,
    pub mac: fip_mac_desc,
    pub wwnn: fip_wwn_desc,
    pub vn: fip_vn_desc,
    pub frame: *mut *mut } __packed,
    pub ff: *mut fip_fc4_feat,
    pub size: *mut fip_size_desc,
    pub fcp_feat: u32,
    pub len: usize,
    pub dlen: usize,
    pub sizeof(*frame): *mut len =,
    pub 0: dlen =,
    if (sub == FIP_SC_VN_CLAIM_NOTIFY || sub == FIP_SC_VN_CLAIM_REP) {
    dlen = sizeof(struct fip_fc4_feat) +
    pub fip_size_desc): sizeof(struct,
    pub dlen: len +=,
    }
    pub sizeof(frame->vn): dlen += sizeof(frame->mac) + sizeof(frame->wwnn) +,
    pub ethhdr)): len = max(len, min_len + sizeof(struct,
    pub dev_alloc_skb(len): skb =,
    if (!skb)
    pub )skb->data: *mut frame = (struct fip_vn2vn_probe_frame,
    pub len): memset(frame, 0,,
    pub ETH_ALEN): memcpy(frame->eth.h_dest, dest,,
    if (sub == FIP_SC_VN_BEACON) {
    pub FIP_VN_FC_MAP): hton24(frame->eth.h_source,,
    pub fip->port_id): hton24(frame->eth.h_source + 3,,
    } else {
    pub ETH_ALEN): memcpy(frame->eth.h_source, fip->ctl_src_addr,,
    }
    pub htons(ETH_P_FIP): frame->eth.h_proto =,
    pub FIP_VER_ENCAPS(FIP_VER): frame->fip.fip_ver =,
    pub htons(FIP_OP_VN2VN): frame->fip.fip_op =,
    pub sub: frame->fip.fip_subcode =,
    pub FIP_BPW): frame->fip.fip_dl_len = htons(dlen /,
    pub FIP_DT_MAC: frame->mac.fd_desc.fip_dtype =,
    pub FIP_BPW: frame->mac.fd_desc.fip_dlen = sizeof(frame->mac) /,
    pub ETH_ALEN): memcpy(frame->mac.fd_mac, fip->ctl_src_addr,,
    pub FIP_DT_NAME: frame->wwnn.fd_desc.fip_dtype =,
    pub FIP_BPW: frame->wwnn.fd_desc.fip_dlen = sizeof(frame->wwnn) /,
    pub &frame->wwnn.fd_wwn): put_unaligned_be64(fip->lp->wwnn,,
    pub FIP_DT_VN_ID: frame->vn.fd_desc.fip_dtype =,
    pub FIP_BPW: frame->vn.fd_desc.fip_dlen = sizeof(frame->vn) /,
    pub FIP_VN_FC_MAP): hton24(frame->vn.fd_mac,,
    pub fip->port_id): hton24(frame->vn.fd_mac + 3,,
    pub fip->port_id): hton24(frame->vn.fd_fc_id,,
    pub &frame->vn.fd_wwpn): put_unaligned_be64(fip->lp->wwpn,,
//
// For claims, add FC-4 features.
// TBD: Add interface to get fc-4 types and features from libfc.
//
    if (sub == FIP_SC_VN_CLAIM_NOTIFY || sub == FIP_SC_VN_CLAIM_REP) {
    pub 1): *mut *mut ff = (struct fip_fc4_feat )(frame +,
    pub FIP_DT_FC4F: ff->fd_desc.fip_dtype =,
    pub FIP_BPW: *mut *mut ff->fd_desc.fip_dlen = sizeof(ff) /,
    pub fip->lp->fcts: ff->fd_fts =,
    pub 0: fcp_feat =,
    if (fip.lp.service_params & FCP_SPPF_INIT_FCN)
    pub FCP_FEAT_INIT: fcp_feat |=,
    if (fip.lp.service_params & FCP_SPPF_TARG_FCN)
    pub FCP_FEAT_TARG: fcp_feat |=,
    pub 32: *mut *mut fcp_feat <<= (FC_TYPE_FCP  4) %,
    pub htonl(fcp_feat): *mut *mut ff->fd_ff.fd_feat[FC_TYPE_FCP  4 / 32] =,
    pub 1): *mut *mut size = (struct fip_size_desc )(ff +,
    pub FIP_DT_FCOE_SIZE: size->fd_desc.fip_dtype =,
    pub FIP_BPW: *mut *mut size->fd_desc.fip_dlen = sizeof(size) /,
    pub htons(fcoe_ctlr_fcoe_size(fip)): size->fd_size =,
    }
    pub len): skb_put(skb,,
    pub htons(ETH_P_FIP): skb->protocol =,
    pub fip->priority: skb->priority =,
    pub skb): fip->send(fip,,
    }
//
// fcoe_ctlr_vn_rport_callback - Event handler for rport events.
// @lport: The lport which is receiving the event
// @rdata: remote port private data
// @event: The event that occurred
//
// Locking Note:  The rport lock must not be held when calling this function.
//
    static void fcoe_ctlr_vn_rport_callback(struct fc_lport *lport,
    struct fc_rport_priv *rdata,
    enum fc_rport_event event)
    {
    pub lport->disc.priv: *mut *mut fcoe_ctlr fip =,
    pub fcoe_ctlr_rport(rdata): *mut *mut fcoe_rport frport =,
    LIBFCOE_FIP_DBG(fip, "vn_rport_callback %x event %d\n",
    pub event): rdata->ids.port_id,,
    switch (event) {
    case RPORT_EV_READY:
    pub 0: frport->login_count =,
    case RPORT_EV_LOGO:
    case RPORT_EV_FAILED:
    case RPORT_EV_STOP:
    if (frport.login_count > FCOE_CTLR_VN2VN_LOGIN_LIMIT) {
    LIBFCOE_FIP_DBG(fip,
    "rport FLOGI limited port_id %6.6x\n",
    }
    default:
    }
    }
    static struct fc_rport_operations fcoe_ctlr_vn_rport_ops = {
    .event_callback = fcoe_ctlr_vn_rport_callback,
}

//
// fcoe_ctlr_disc_stop_locked() - stop discovery in VN2VN mode
// @lport: The local port
//
// Called with ctlr_mutex held.
//
#[no_mangle]
unsafe extern "C" fn fcoe_ctlr_disc_stop_locked(lport: *mut fc_lport) {
    static void fcoe_ctlr_disc_stop_locked(struct fc_lport *lport)
    {
    struct fc_rport_priv *rdata;
    mutex_lock(&lport.disc.disc_mutex);
    list_for_each_entry_rcu(rdata, &lport.disc.rports, peers) {
    if (kref_get_unless_zero(&rdata.kref)) {
    fc_rport_logoff(rdata);
    kref_put(&rdata.kref, fc_rport_destroy);
    }
    }
    lport.disc.disc_callback = core::ptr::null_mut();
    mutex_unlock(&lport.disc.disc_mutex);
    }
//
// fcoe_ctlr_disc_stop() - stop discovery in VN2VN mode
// @lport: The local port
//
// Called through the local port template for discovery.
// Called without the ctlr_mutex held.
//
#[no_mangle]
unsafe extern "C" fn fcoe_ctlr_disc_stop(lport: *mut fc_lport) {
    static void fcoe_ctlr_disc_stop(struct fc_lport *lport)
    {
    struct fcoe_ctlr *fip = lport.disc.priv;
    mutex_lock(&fip.ctlr_mutex);
    fcoe_ctlr_disc_stop_locked(lport);
    mutex_unlock(&fip.ctlr_mutex);
    }
//
// fcoe_ctlr_disc_stop_final() - stop discovery for shutdown in VN2VN mode
// @lport: The local port
//
// Called through the local port template for discovery.
// Called without the ctlr_mutex held.
//
#[no_mangle]
unsafe extern "C" fn fcoe_ctlr_disc_stop_final(lport: *mut fc_lport) {
    static void fcoe_ctlr_disc_stop_final(struct fc_lport *lport)
    {
    fcoe_ctlr_disc_stop(lport);
    fc_rport_flush_queue();
    synchronize_rcu();
    }
//
// fcoe_ctlr_vn_restart() - VN2VN probe restart with new port_id
// @fip: The FCoE controller
//
// Called with fcoe_ctlr lock held.
//
#[no_mangle]
unsafe extern "C" fn fcoe_ctlr_vn_restart(fip: *mut fcoe_ctlr) {
    static void fcoe_ctlr_vn_restart(struct fcoe_ctlr *fip)
    {
    unsigned long wait;
    u32 port_id;
    fcoe_ctlr_disc_stop_locked(fip.lp);
//
// Get proposed port ID.
// If this is the first try after link up, use any previous port_id.
// If there was none, use the low bits of the port_name.
// On subsequent tries, get the next random one.
// Don't use reserved IDs, use another non-zero value, just as random.
//
    port_id = fip.port_id;
    if (fip.probe_tries)
    port_id = prandom_u32_state(&fip.rnd_state) & 0xffff;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !port_id) -> else {
    else if (!port_id)
    port_id = fip.lp.wwpn & 0xffff;
    if (!port_id || port_id == 0xffff)
    port_id = 1;
    fip.port_id = port_id;
    if (fip.probe_tries < FIP_VN_RLIM_COUNT) {
    fip.probe_tries++;
    wait = get_random_u32_below(FIP_VN_PROBE_WAIT);
    } else
    wait = FIP_VN_RLIM_INT;
    mod_timer(&fip.timer, jiffies + msecs_to_jiffies(wait));
    fcoe_ctlr_set_state(fip, FIP_ST_VNMP_START);
    }
//
// fcoe_ctlr_vn_start() - Start in VN2VN mode
// @fip: The FCoE controller
//
// Called with fcoe_ctlr lock held.
//
#[no_mangle]
unsafe extern "C" fn fcoe_ctlr_vn_start(fip: *mut fcoe_ctlr) {
    static void fcoe_ctlr_vn_start(struct fcoe_ctlr *fip)
    {
    fip.probe_tries = 0;
    prandom_seed_state(&fip.rnd_state, fip.lp.wwpn);
    fcoe_ctlr_vn_restart(fip);
    }
//
// fcoe_ctlr_vn_parse - parse probe request or response
// @fip: The FCoE controller
// @skb: incoming packet
// @frport: parsed FCoE rport from the probe request
//
// Returns non-zero error number on error.
// Does not consume the packet.
//
    static int fcoe_ctlr_vn_parse(struct fcoe_ctlr *fip,
    struct sk_buff *skb,
    struct fcoe_rport *frport)
    {
    struct fip_header *fiph;
    struct fip_desc *desc = core::ptr::null_mut();
    struct fip_mac_desc *macd = core::ptr::null_mut();
    struct fip_wwn_desc *wwn = core::ptr::null_mut();
    struct fip_vn_desc *vn = core::ptr::null_mut();
    struct fip_size_desc *size = core::ptr::null_mut();
    size_t rlen;
    size_t dlen;
    let mut desc_mask: u32 = 0;
    u32 dtype;
    u8 sub;
    fiph = (struct fip_header *)skb.data;
    frport.flags = ntohs(fiph.fip_flags);
    sub = fiph.fip_subcode;
    switch (sub) {
    case FIP_SC_VN_PROBE_REQ:
    case FIP_SC_VN_PROBE_REP:
    case FIP_SC_VN_BEACON:
    desc_mask = BIT(FIP_DT_MAC) | BIT(FIP_DT_NAME) |
    BIT(FIP_DT_VN_ID);
    break;
    case FIP_SC_VN_CLAIM_NOTIFY:
    case FIP_SC_VN_CLAIM_REP:
    desc_mask = BIT(FIP_DT_MAC) | BIT(FIP_DT_NAME) |
    BIT(FIP_DT_VN_ID) | BIT(FIP_DT_FC4F) |
    BIT(FIP_DT_FCOE_SIZE);
    break;
    default:
    LIBFCOE_FIP_DBG(fip, "vn_parse unknown subcode %u\n", sub);
    return -EINVAL;
    }
    rlen = ntohs(fiph.fip_dl_len) * 4;
    if (rlen + sizeof(*fiph) > skb.len)
    return -EINVAL;
    desc = (struct fip_desc *)(fiph + 1);
    while (rlen > 0) {
    dlen = desc.fip_dlen * FIP_BPW;
    if (dlen < sizeof(*desc) || dlen > rlen)
    return -EINVAL;
    dtype = desc.fip_dtype;
    if (dtype < 32) {
    if (!(desc_mask & BIT(dtype))) {
    LIBFCOE_FIP_DBG(fip,
    "unexpected or duplicated desc "
    "desc type %u in "
    "FIP VN2VN subtype %u\n",
    dtype, sub);
    return -EINVAL;
    }
    desc_mask &= ~BIT(dtype);
    }
    switch (dtype) {
    case FIP_DT_MAC:
    if (dlen != sizeof(struct fip_mac_desc))
    goto len_err;
    macd = (struct fip_mac_desc *)desc;
    if (!is_valid_ether_addr(macd.fd_mac)) {
    LIBFCOE_FIP_DBG(fip,
    "Invalid MAC addr %pM in FIP VN2VN\n",
    macd.fd_mac);
    return -EINVAL;
    }
    memcpy(frport.enode_mac, macd.fd_mac, ETH_ALEN);
    break;
    case FIP_DT_NAME:
    if (dlen != sizeof(struct fip_wwn_desc))
    goto len_err;
    wwn = (struct fip_wwn_desc *)desc;
    frport.rdata.ids.node_name =
    get_unaligned_be64(&wwn.fd_wwn);
    break;
    case FIP_DT_VN_ID:
    if (dlen != sizeof(struct fip_vn_desc))
    goto len_err;
    vn = (struct fip_vn_desc *)desc;
    memcpy(frport.vn_mac, vn.fd_mac, ETH_ALEN);
    frport.rdata.ids.port_id = ntoh24(vn.fd_fc_id);
    frport.rdata.ids.port_name =
    get_unaligned_be64(&vn.fd_wwpn);
    break;
    case FIP_DT_FC4F:
    if (dlen != sizeof(struct fip_fc4_feat))
    goto len_err;
    break;
    case FIP_DT_FCOE_SIZE:
    if (dlen != sizeof(struct fip_size_desc))
    goto len_err;
    size = (struct fip_size_desc *)desc;
    frport.fcoe_len = ntohs(size.fd_size);
    break;
    default:
    LIBFCOE_FIP_DBG(fip, "unexpected descriptor type %x "
    "in FIP probe\n", dtype);
// standard says ignore unknown descriptors >= 128
    if (dtype < FIP_DT_NON_CRITICAL)
    return -EINVAL;
    break;
    }
    desc = (struct fip_desc *)((char *)desc + dlen);
    rlen -= dlen;
    }
    return 0;
    len_err:
    LIBFCOE_FIP_DBG(fip, "FIP length error in descriptor type %x len %zu\n",
    dtype, dlen);
    return -EINVAL;
    }
//
// fcoe_ctlr_vn_send_claim() - send multicast FIP VN2VN Claim Notification.
// @fip: The FCoE controller
//
// Called with ctlr_mutex held.
//
#[no_mangle]
unsafe extern "C" fn fcoe_ctlr_vn_send_claim(fip: *mut fcoe_ctlr) {
    static void fcoe_ctlr_vn_send_claim(struct fcoe_ctlr *fip)
    {
    fcoe_ctlr_vn_send(fip, FIP_SC_VN_CLAIM_NOTIFY, fcoe_all_vn2vn, 0);
    fip.sol_time = jiffies;
    }
//
// fcoe_ctlr_vn_probe_req() - handle incoming VN2VN probe request.
// @fip: The FCoE controller
// @frport: parsed FCoE rport from the probe request
//
// Called with ctlr_mutex held.
//
    static void fcoe_ctlr_vn_probe_req(struct fcoe_ctlr *fip,
    struct fcoe_rport *frport)
    {
    if (frport.rdata.ids.port_id != fip.port_id)
    return;
    switch (fip.state) {
    case FIP_ST_VNMP_CLAIM:
    case FIP_ST_VNMP_UP:
    LIBFCOE_FIP_DBG(fip, "vn_probe_req: send reply, state %x\n",
    fip.state);
    fcoe_ctlr_vn_send(fip, FIP_SC_VN_PROBE_REP,
    frport.enode_mac, 0);
    break;
    case FIP_ST_VNMP_PROBE1:
    case FIP_ST_VNMP_PROBE2:
//
// Decide whether to reply to the Probe.
// Our selected address is never a "recorded" one, so
// only reply if our WWPN is greater and the
// Probe's REC bit is not set.
// If we don't reply, we will change our address.
//
    if (fip.lp.wwpn > frport.rdata.ids.port_name &&
    !(frport.flags & FIP_FL_REC_OR_P2P)) {
    LIBFCOE_FIP_DBG(fip, "vn_probe_req: "
    "port_id collision\n");
    fcoe_ctlr_vn_send(fip, FIP_SC_VN_PROBE_REP,
    frport.enode_mac, 0);
    break;
    }
    fallthrough;
    case FIP_ST_VNMP_START:
    LIBFCOE_FIP_DBG(fip, "vn_probe_req: "
    "restart VN2VN negotiation\n");
    fcoe_ctlr_vn_restart(fip);
    break;
    default:
    LIBFCOE_FIP_DBG(fip, "vn_probe_req: ignore state %x\n",
    fip.state);
    break;
    }
    }
//
// fcoe_ctlr_vn_probe_reply() - handle incoming VN2VN probe reply.
// @fip: The FCoE controller
// @frport: parsed FCoE rport from the probe request
//
// Called with ctlr_mutex held.
//
    static void fcoe_ctlr_vn_probe_reply(struct fcoe_ctlr *fip,
    struct fcoe_rport *frport)
    {
    if (frport.rdata.ids.port_id != fip.port_id)
    return;
    switch (fip.state) {
    case FIP_ST_VNMP_START:
    case FIP_ST_VNMP_PROBE1:
    case FIP_ST_VNMP_PROBE2:
    case FIP_ST_VNMP_CLAIM:
    LIBFCOE_FIP_DBG(fip, "vn_probe_reply: restart state %x\n",
    fip.state);
    fcoe_ctlr_vn_restart(fip);
    break;
    case FIP_ST_VNMP_UP:
    LIBFCOE_FIP_DBG(fip, "vn_probe_reply: send claim notify\n");
    fcoe_ctlr_vn_send_claim(fip);
    break;
    default:
    break;
    }
    }
//
// fcoe_ctlr_vn_add() - Add a VN2VN entry to the list, based on a claim reply.
// @fip: The FCoE controller
// @new: newly-parsed FCoE rport as a template for new rdata
//
// Called with ctlr_mutex held.
//
#[no_mangle]
unsafe extern "C" fn fcoe_ctlr_vn_add(fip: *mut fcoe_ctlr, new: *mut fcoe_rport) {
    static void fcoe_ctlr_vn_add(struct fcoe_ctlr *fip, struct fcoe_rport *new)
    {
    struct fc_lport *lport = fip.lp;
    struct fc_rport_priv *rdata;
    struct fc_rport_identifiers *ids;
    struct fcoe_rport *frport;
    u32 port_id;
    port_id = new.rdata.ids.port_id;
    if (port_id == fip.port_id)
    return;
    mutex_lock(&lport.disc.disc_mutex);
    rdata = fc_rport_create(lport, port_id);
    if (!rdata) {
    mutex_unlock(&lport.disc.disc_mutex);
    return;
    }
    mutex_lock(&rdata.rp_mutex);
    mutex_unlock(&lport.disc.disc_mutex);
    rdata.ops = &fcoe_ctlr_vn_rport_ops;
    rdata.disc_id = lport.disc.disc_id;
    ids = &rdata.ids;
    if ((ids.port_name != -1 &&
    ids.port_name != new.rdata.ids.port_name) ||
    (ids.node_name != -1 &&
    ids.node_name != new.rdata.ids.node_name)) {
    mutex_unlock(&rdata.rp_mutex);
    LIBFCOE_FIP_DBG(fip, "vn_add rport logoff %6.6x\n", port_id);
    fc_rport_logoff(rdata);
    mutex_lock(&rdata.rp_mutex);
    }
    ids.port_name = new.rdata.ids.port_name;
    ids.node_name = new.rdata.ids.node_name;
    mutex_unlock(&rdata.rp_mutex);
    frport = fcoe_ctlr_rport(rdata);
    LIBFCOE_FIP_DBG(fip, "vn_add rport %6.6x %s state %d\n",
    port_id, frport.fcoe_len ? "old" : "new",
    rdata.rp_state);
    frport.fcoe_len = new.fcoe_len;
    frport.flags = new.flags;
    frport.login_count = new.login_count;
    memcpy(frport.enode_mac, new.enode_mac, ETH_ALEN);
    memcpy(frport.vn_mac, new.vn_mac, ETH_ALEN);
    frport.time = 0;
    }
//
// fcoe_ctlr_vn_lookup() - Find VN remote port's MAC address
// @fip: The FCoE controller
// @port_id:  The port_id of the remote VN_node
// @mac: buffer which will hold the VN_NODE destination MAC address, if found.
//
// Returns non-zero error if no remote port found.
//
#[no_mangle]
unsafe extern "C" fn fcoe_ctlr_vn_lookup(fip: *mut fcoe_ctlr, port_id: u32, mac: *mut u8) -> c_int {
    static int fcoe_ctlr_vn_lookup(struct fcoe_ctlr *fip, u32 port_id, u8 *mac)
    {
    struct fc_lport *lport = fip.lp;
    struct fc_rport_priv *rdata;
    struct fcoe_rport *frport;
    let mut ret: c_int = -1;
    rdata = fc_rport_lookup(lport, port_id);
    if (rdata) {
    frport = fcoe_ctlr_rport(rdata);
    memcpy(mac, frport.enode_mac, ETH_ALEN);
    ret = 0;
    kref_put(&rdata.kref, fc_rport_destroy);
    }
    return ret;
    }
//
// fcoe_ctlr_vn_claim_notify() - handle received FIP VN2VN Claim Notification
// @fip: The FCoE controller
// @new: newly-parsed FCoE rport as a template for new rdata
//
// Called with ctlr_mutex held.
//
    static void fcoe_ctlr_vn_claim_notify(struct fcoe_ctlr *fip,
    struct fcoe_rport *new)
    {
    if (new.flags & FIP_FL_REC_OR_P2P) {
    LIBFCOE_FIP_DBG(fip, "send probe req for P2P/REC\n");
    fcoe_ctlr_vn_send(fip, FIP_SC_VN_PROBE_REQ, fcoe_all_vn2vn, 0);
    return;
    }
    switch (fip.state) {
    case FIP_ST_VNMP_START:
    case FIP_ST_VNMP_PROBE1:
    case FIP_ST_VNMP_PROBE2:
    if (new.rdata.ids.port_id == fip.port_id) {
    LIBFCOE_FIP_DBG(fip, "vn_claim_notify: "
    "restart, state %d\n",
    fip.state);
    fcoe_ctlr_vn_restart(fip);
    }
    break;
    case FIP_ST_VNMP_CLAIM:
    case FIP_ST_VNMP_UP:
    if (new.rdata.ids.port_id == fip.port_id) {
    if (new.rdata.ids.port_name > fip.lp.wwpn) {
    LIBFCOE_FIP_DBG(fip, "vn_claim_notify: "
    "restart, port_id collision\n");
    fcoe_ctlr_vn_restart(fip);
    break;
    }
    LIBFCOE_FIP_DBG(fip, "vn_claim_notify: "
    "send claim notify\n");
    fcoe_ctlr_vn_send_claim(fip);
    break;
    }
    LIBFCOE_FIP_DBG(fip, "vn_claim_notify: send reply to %x\n",
    new.rdata.ids.port_id);
    fcoe_ctlr_vn_send(fip, FIP_SC_VN_CLAIM_REP, new.enode_mac,
    min((u32)new.fcoe_len,
    fcoe_ctlr_fcoe_size(fip)));
    fcoe_ctlr_vn_add(fip, new);
    break;
    default:
    LIBFCOE_FIP_DBG(fip, "vn_claim_notify: "
    "ignoring claim from %x\n",
    new.rdata.ids.port_id);
    break;
    }
    }
//
// fcoe_ctlr_vn_claim_resp() - handle received Claim Response
// @fip: The FCoE controller that received the frame
// @new: newly-parsed FCoE rport from the Claim Response
//
// Called with ctlr_mutex held.
//
    static void fcoe_ctlr_vn_claim_resp(struct fcoe_ctlr *fip,
    struct fcoe_rport *new)
    {
    LIBFCOE_FIP_DBG(fip, "claim resp from from rport %x - state %s\n",
    new.rdata.ids.port_id, fcoe_ctlr_state(fip.state));
    if (fip.state == FIP_ST_VNMP_UP || fip.state == FIP_ST_VNMP_CLAIM)
    fcoe_ctlr_vn_add(fip, new);
    }
//
// fcoe_ctlr_vn_beacon() - handle received beacon.
// @fip: The FCoE controller that received the frame
// @new: newly-parsed FCoE rport from the Beacon
//
// Called with ctlr_mutex held.
//
    static void fcoe_ctlr_vn_beacon(struct fcoe_ctlr *fip,
    struct fcoe_rport *new)
    {
    struct fc_lport *lport = fip.lp;
    struct fc_rport_priv *rdata;
    struct fcoe_rport *frport;
    if (new.flags & FIP_FL_REC_OR_P2P) {
    LIBFCOE_FIP_DBG(fip, "p2p beacon while in vn2vn mode\n");
    fcoe_ctlr_vn_send(fip, FIP_SC_VN_PROBE_REQ, fcoe_all_vn2vn, 0);
    return;
    }
    rdata = fc_rport_lookup(lport, new.rdata.ids.port_id);
    if (rdata) {
    if (rdata.ids.node_name == new.rdata.ids.node_name &&
    rdata.ids.port_name == new.rdata.ids.port_name) {
    frport = fcoe_ctlr_rport(rdata);
    LIBFCOE_FIP_DBG(fip, "beacon from rport %x\n",
    rdata.ids.port_id);
    if (!frport.time && fip.state == FIP_ST_VNMP_UP) {
    LIBFCOE_FIP_DBG(fip, "beacon expired "
    "for rport %x\n",
    rdata.ids.port_id);
    fc_rport_login(rdata);
    }
    frport.time = jiffies;
    }
    kref_put(&rdata.kref, fc_rport_destroy);
    return;
    }
    if (fip.state != FIP_ST_VNMP_UP)
    return;
//
// Beacon from a new neighbor.
// Send a claim notify if one hasn't been sent recently.
// Don't add the neighbor yet.
//
    LIBFCOE_FIP_DBG(fip, "beacon from new rport %x. sending claim notify\n",
    new.rdata.ids.port_id);
    if (time_after(jiffies,
    fip.sol_time + msecs_to_jiffies(FIP_VN_ANN_WAIT)))
    fcoe_ctlr_vn_send_claim(fip);
    }
//
// fcoe_ctlr_vn_age() - Check for VN_ports without recent beacons
// @fip: The FCoE controller
//
// Called with ctlr_mutex held.
// Called only in state FIP_ST_VNMP_UP.
// Returns the soonest time for next age-out or a time far in the future.
//
#[no_mangle]
unsafe extern "C" fn fcoe_ctlr_vn_age(fip: *mut fcoe_ctlr) -> c_ulong {
    static unsigned long fcoe_ctlr_vn_age(struct fcoe_ctlr *fip)
    {
    struct fc_lport *lport = fip.lp;
    struct fc_rport_priv *rdata;
    struct fcoe_rport *frport;
    unsigned long next_time;
    unsigned long deadline;
    next_time = jiffies + msecs_to_jiffies(FIP_VN_BEACON_INT * 10);
    mutex_lock(&lport.disc.disc_mutex);
    list_for_each_entry_rcu(rdata, &lport.disc.rports, peers) {
    if (!kref_get_unless_zero(&rdata.kref))
    continue;
    frport = fcoe_ctlr_rport(rdata);
    if (!frport.time) {
    kref_put(&rdata.kref, fc_rport_destroy);
    continue;
    }
    deadline = frport.time +
    msecs_to_jiffies(FIP_VN_BEACON_INT * 25 / 10);
    if (time_after_eq(jiffies, deadline)) {
    frport.time = 0;
    LIBFCOE_FIP_DBG(fip,
    "port %16.16llx fc_id %6.6x beacon expired\n",
    rdata.ids.port_name, rdata.ids.port_id);
    fc_rport_logoff(rdata);
    } else if (time_before(deadline, next_time))
    next_time = deadline;
    kref_put(&rdata.kref, fc_rport_destroy);
    }
    mutex_unlock(&lport.disc.disc_mutex);
    return next_time;
    }
//
// fcoe_ctlr_vn_recv() - Receive a FIP frame
// @fip: The FCoE controller that received the frame
// @skb: The received FIP frame
//
// Returns non-zero if the frame is dropped.
// Always consumes the frame.
//
#[no_mangle]
unsafe extern "C" fn fcoe_ctlr_vn_recv(fip: *mut fcoe_ctlr, skb: *mut sk_buff) -> c_int {
    static int fcoe_ctlr_vn_recv(struct fcoe_ctlr *fip, struct sk_buff *skb)
    {
    struct fip_header *fiph;
    enum fip_vn2vn_subcode sub;
    let mut frport: fcoe_rport = { };
    int rc, vlan_id = 0;
    fiph = (struct fip_header *)skb.data;
    sub = fiph.fip_subcode;
    if (fip.lp.vlan)
    vlan_id = skb_vlan_tag_get_id(skb);
    if (vlan_id && vlan_id != fip.lp.vlan) {
    LIBFCOE_FIP_DBG(fip, "vn_recv drop frame sub %x vlan %d\n",
    sub, vlan_id);
    rc = -EAGAIN;
    goto drop;
    }
    rc = fcoe_ctlr_vn_parse(fip, skb, &frport);
    if (rc) {
    LIBFCOE_FIP_DBG(fip, "vn_recv vn_parse error %d\n", rc);
    goto drop;
    }
    mutex_lock(&fip.ctlr_mutex);
    switch (sub) {
    case FIP_SC_VN_PROBE_REQ:
    fcoe_ctlr_vn_probe_req(fip, &frport);
    break;
    case FIP_SC_VN_PROBE_REP:
    fcoe_ctlr_vn_probe_reply(fip, &frport);
    break;
    case FIP_SC_VN_CLAIM_NOTIFY:
    fcoe_ctlr_vn_claim_notify(fip, &frport);
    break;
    case FIP_SC_VN_CLAIM_REP:
    fcoe_ctlr_vn_claim_resp(fip, &frport);
    break;
    case FIP_SC_VN_BEACON:
    fcoe_ctlr_vn_beacon(fip, &frport);
    break;
    default:
    LIBFCOE_FIP_DBG(fip, "vn_recv unknown subcode %d\n", sub);
    rc = -1;
    break;
    }
    mutex_unlock(&fip.ctlr_mutex);
    drop:
    kfree_skb(skb);
    return rc;
    }
//
// fcoe_ctlr_vlan_parse - parse vlan discovery request or response
// @fip: The FCoE controller
// @skb: incoming packet
// @frport: parsed FCoE rport from the probe request
//
// Returns non-zero error number on error.
// Does not consume the packet.
//
    static int fcoe_ctlr_vlan_parse(struct fcoe_ctlr *fip,
    struct sk_buff *skb,
    struct fcoe_rport *frport)
    {
    struct fip_header *fiph;
    struct fip_desc *desc = core::ptr::null_mut();
    struct fip_mac_desc *macd = core::ptr::null_mut();
    struct fip_wwn_desc *wwn = core::ptr::null_mut();
    size_t rlen;
    size_t dlen;
    let mut desc_mask: u32 = 0;
    u32 dtype;
    u8 sub;
    fiph = (struct fip_header *)skb.data;
    frport.flags = ntohs(fiph.fip_flags);
    sub = fiph.fip_subcode;
    switch (sub) {
    case FIP_SC_VL_REQ:
    desc_mask = BIT(FIP_DT_MAC) | BIT(FIP_DT_NAME);
    break;
    default:
    LIBFCOE_FIP_DBG(fip, "vn_parse unknown subcode %u\n", sub);
    return -EINVAL;
    }
    rlen = ntohs(fiph.fip_dl_len) * 4;
    if (rlen + sizeof(*fiph) > skb.len)
    return -EINVAL;
    desc = (struct fip_desc *)(fiph + 1);
    while (rlen > 0) {
    dlen = desc.fip_dlen * FIP_BPW;
    if (dlen < sizeof(*desc) || dlen > rlen)
    return -EINVAL;
    dtype = desc.fip_dtype;
    if (dtype < 32) {
    if (!(desc_mask & BIT(dtype))) {
    LIBFCOE_FIP_DBG(fip,
    "unexpected or duplicated desc "
    "desc type %u in "
    "FIP VN2VN subtype %u\n",
    dtype, sub);
    return -EINVAL;
    }
    desc_mask &= ~BIT(dtype);
    }
    switch (dtype) {
    case FIP_DT_MAC:
    if (dlen != sizeof(struct fip_mac_desc))
    goto len_err;
    macd = (struct fip_mac_desc *)desc;
    if (!is_valid_ether_addr(macd.fd_mac)) {
    LIBFCOE_FIP_DBG(fip,
    "Invalid MAC addr %pM in FIP VN2VN\n",
    macd.fd_mac);
    return -EINVAL;
    }
    memcpy(frport.enode_mac, macd.fd_mac, ETH_ALEN);
    break;
    case FIP_DT_NAME:
    if (dlen != sizeof(struct fip_wwn_desc))
    goto len_err;
    wwn = (struct fip_wwn_desc *)desc;
    frport.rdata.ids.node_name =
    get_unaligned_be64(&wwn.fd_wwn);
    break;
    default:
    LIBFCOE_FIP_DBG(fip, "unexpected descriptor type %x "
    "in FIP probe\n", dtype);
// standard says ignore unknown descriptors >= 128
    if (dtype < FIP_DT_NON_CRITICAL)
    return -EINVAL;
    break;
    }
    desc = (struct fip_desc *)((char *)desc + dlen);
    rlen -= dlen;
    }
    return 0;
    len_err:
    LIBFCOE_FIP_DBG(fip, "FIP length error in descriptor type %x len %zu\n",
    dtype, dlen);
    return -EINVAL;
    }
//
// fcoe_ctlr_vlan_send() - Send a FIP VLAN Notification
// @fip: The FCoE controller
// @sub: sub-opcode for vlan notification or vn2vn vlan notification
// @dest: The destination Ethernet MAC address
//
    static void fcoe_ctlr_vlan_send(struct fcoe_ctlr *fip,
    enum fip_vlan_subcode sub,
    const u8 *dest)
    {
    struct sk_buff *skb;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fip_vlan_notify_frame {
    pub eth: ethhdr,
    pub fip: fip_header,
    pub mac: fip_mac_desc,
    pub vlan: fip_vlan_desc,
    pub frame: *mut *mut } __packed,
    pub len: usize,
    pub dlen: usize,
    pub sizeof(*frame): *mut len =,
    pub sizeof(frame->vlan): dlen = sizeof(frame->mac) +,
    pub ethhdr)): len = max(len, sizeof(struct,
    pub dev_alloc_skb(len): skb =,
    if (!skb)
    LIBFCOE_FIP_DBG(fip, "fip %s vlan notification, vlan %d\n",
    fip.mode == FIP_MODE_VN2VN ? "vn2vn" : "fcf",
    pub )skb->data: *mut frame = (struct fip_vlan_notify_frame,
    pub len): memset(frame, 0,,
    pub ETH_ALEN): memcpy(frame->eth.h_dest, dest,,
    pub ETH_ALEN): memcpy(frame->eth.h_source, fip->ctl_src_addr,,
    pub htons(ETH_P_FIP): frame->eth.h_proto =,
    pub FIP_VER_ENCAPS(FIP_VER): frame->fip.fip_ver =,
    pub htons(FIP_OP_VLAN): frame->fip.fip_op =,
    pub sub: frame->fip.fip_subcode =,
    pub FIP_BPW): frame->fip.fip_dl_len = htons(dlen /,
    pub FIP_DT_MAC: frame->mac.fd_desc.fip_dtype =,
    pub FIP_BPW: frame->mac.fd_desc.fip_dlen = sizeof(frame->mac) /,
    pub ETH_ALEN): memcpy(frame->mac.fd_mac, fip->ctl_src_addr,,
    pub FIP_DT_VLAN: frame->vlan.fd_desc.fip_dtype =,
    pub FIP_BPW: frame->vlan.fd_desc.fip_dlen = sizeof(frame->vlan) /,
    pub &frame->vlan.fd_vlan): put_unaligned_be16(fip->lp->vlan,,
    pub len): skb_put(skb,,
    pub htons(ETH_P_FIP): skb->protocol =,
    pub fip->priority: skb->priority =,
    pub skb): fip->send(fip,,
    }
//
// fcoe_ctlr_vlan_disc_reply() - send FIP VLAN Discovery Notification.
// @fip: The FCoE controller
// @frport: The newly-parsed FCoE rport from the Discovery Request
//
// Called with ctlr_mutex held.
//
    static void fcoe_ctlr_vlan_disc_reply(struct fcoe_ctlr *fip,
    struct fcoe_rport *frport)
    {
    pub FIP_SC_VL_NOTE: enum fip_vlan_subcode sub =,
    if (fip.mode == FIP_MODE_VN2VN)
    pub FIP_SC_VL_VN2VN_NOTE: sub =,
    pub frport->enode_mac): fcoe_ctlr_vlan_send(fip, sub,,
    }
//
// fcoe_ctlr_vlan_recv - vlan request receive handler for VN2VN mode.
// @fip: The FCoE controller
// @skb: The received FIP packet
//
#[no_mangle]
unsafe extern "C" fn fcoe_ctlr_vlan_recv(fip: *mut fcoe_ctlr, skb: *mut sk_buff) -> c_int {
    static int fcoe_ctlr_vlan_recv(struct fcoe_ctlr *fip, struct sk_buff *skb)
    {
    pub fiph: *mut fip_header,
    pub sub: enum fip_vlan_subcode,
    pub }: fcoe_rport frport = {,
    pub rc: c_int,
    pub )skb->data: *mut fiph = (struct fip_header,
    pub fiph->fip_subcode: sub =,
    pub &frport): rc = fcoe_ctlr_vlan_parse(fip, skb,,
    if (rc) {
    pub rc): LIBFCOE_FIP_DBG(fip, "vlan_recv vlan_parse error %d\n",,
    pub drop: goto,
    }
    if (sub == FIP_SC_VL_REQ)
    pub &frport): fcoe_ctlr_vlan_disc_reply(fip,,
    drop:
    pub rc: return,
    }
//
// fcoe_ctlr_disc_recv - discovery receive handler for VN2VN mode.
// @lport: The local port
// @fp: The received frame
//
// This should never be called since we don't see RSCNs or other
// fabric-generated ELSes.
//
#[no_mangle]
unsafe extern "C" fn fcoe_ctlr_disc_recv(lport: *mut fc_lport, fp: *mut fc_frame) {
    static void fcoe_ctlr_disc_recv(struct fc_lport *lport, struct fc_frame *fp)
    {
    pub rjt_data: fc_seq_els_data,
    pub ELS_RJT_UNSUP: rjt_data.reason =,
    pub ELS_EXPL_NONE: rjt_data.explan =,
    pub &rjt_data): fc_seq_els_rsp_send(fp, ELS_LS_RJT,,
    }
//
// fcoe_ctlr_disc_start - start discovery for VN2VN mode.
//
// This sets a flag indicating that remote ports should be created
// and started for the peers we discover.  We use the disc_callback
// pointer as that flag.  Peers already discovered are created here.
//
// The lport lock is held during this call. The callback must be done
// later, without holding either the lport or discovery locks.
// The fcoe_ctlr lock may also be held during this call.
//
    static void fcoe_ctlr_disc_start(void (*callback)(struct fc_lport *,
    enum fc_disc_event),
    struct fc_lport *lport)
    {
    pub &lport->disc: *mut *mut fc_disc disc =,
    pub disc->priv: *mut *mut fcoe_ctlr fip =,
    pub callback: disc->disc_callback =,
    pub 1: disc->disc_id = (disc->disc_id + 2) |,
    pub 1: disc->pending =,
    }
//
// fcoe_ctlr_vn_disc() - report FIP VN_port discovery results after claim state.
// @fip: The FCoE controller
//
// Starts the FLOGI and PLOGI login process to each discovered rport for which
// we've received at least one beacon.
// Performs the discovery complete callback.
//
#[no_mangle]
unsafe extern "C" fn fcoe_ctlr_vn_disc(fip: *mut fcoe_ctlr) {
    static void fcoe_ctlr_vn_disc(struct fcoe_ctlr *fip)
    {
    pub fip->lp: *mut *mut fc_lport lport =,
    pub &lport->disc: *mut *mut fc_disc disc =,
    pub rdata: *mut fc_rport_priv,
    pub frport: *mut fcoe_rport,
    pub fc_disc_event): *mut *mut *mut void (callback)(struct fc_lport , enum,
    pub NULL: callback = disc->pending ? disc->disc_callback :,
    pub 0: disc->pending =,
    list_for_each_entry_rcu(rdata, &disc.rports, peers) {
    if (!kref_get_unless_zero(&rdata.kref))
    pub fcoe_ctlr_rport(rdata): frport =,
    if (frport.time)
    pub fc_rport_destroy): kref_put(&rdata->kref,,
    }
    if (callback)
    pub DISC_EV_SUCCESS): callback(lport,,
    }
//
// fcoe_ctlr_vn_timeout - timer work function for VN2VN mode.
// @fip: The FCoE controller
//
#[no_mangle]
unsafe extern "C" fn fcoe_ctlr_vn_timeout(fip: *mut fcoe_ctlr) {
    static void fcoe_ctlr_vn_timeout(struct fcoe_ctlr *fip)
    {
    pub next_time: c_ulong,
    pub mac: [u8; ETH_ALEN],
    pub 0: u32 new_port_id =,
    switch (fip.state) {
    case FIP_ST_VNMP_START:
    pub FIP_ST_VNMP_PROBE1): fcoe_ctlr_set_state(fip,,
    pub request\n"): LIBFCOE_FIP_DBG(fip, "vn_timeout: send 1st probe,
    pub 0): fcoe_ctlr_vn_send(fip, FIP_SC_VN_PROBE_REQ, fcoe_all_vn2vn,,
    pub msecs_to_jiffies(FIP_VN_PROBE_WAIT): next_time = jiffies +,
    case FIP_ST_VNMP_PROBE1:
    pub FIP_ST_VNMP_PROBE2): fcoe_ctlr_set_state(fip,,
    pub request\n"): LIBFCOE_FIP_DBG(fip, "vn_timeout: send 2nd probe,
    pub 0): fcoe_ctlr_vn_send(fip, FIP_SC_VN_PROBE_REQ, fcoe_all_vn2vn,,
    pub msecs_to_jiffies(FIP_VN_ANN_WAIT): next_time = jiffies +,
    case FIP_ST_VNMP_PROBE2:
    pub FIP_ST_VNMP_CLAIM): fcoe_ctlr_set_state(fip,,
    pub fip->port_id: new_port_id =,
    pub FIP_VN_FC_MAP): hton24(mac,,
    pub new_port_id): hton24(mac + 3,,
    pub mac): fip->update_mac(fip->lp,,
    pub notify\n"): LIBFCOE_FIP_DBG(fip, "vn_timeout: send claim,
    pub msecs_to_jiffies(FIP_VN_ANN_WAIT): next_time = jiffies +,
    case FIP_ST_VNMP_CLAIM:
//
// This may be invoked either by starting discovery so don't
// go to the next state unless it's been long enough.
//
    pub msecs_to_jiffies(FIP_VN_ANN_WAIT): next_time = fip->sol_time +,
    if (time_after_eq(jiffies, next_time)) {
    pub FIP_ST_VNMP_UP): fcoe_ctlr_set_state(fip,,
    pub beacon\n"): LIBFCOE_FIP_DBG(fip, "vn_timeout: send vn2vn,
    fcoe_ctlr_vn_send(fip, FIP_SC_VN_BEACON,
    pub 0): fcoe_all_vn2vn,,
    pub msecs_to_jiffies(FIP_VN_ANN_WAIT): next_time = jiffies +,
    pub next_time: fip->port_ka_time =,
    }
    case FIP_ST_VNMP_UP:
    pub fcoe_ctlr_vn_age(fip): next_time =,
    if (time_after_eq(jiffies, fip.port_ka_time)) {
    pub beacon\n"): LIBFCOE_FIP_DBG(fip, "vn_timeout: send vn2vn,
    fcoe_ctlr_vn_send(fip, FIP_SC_VN_BEACON,
    pub 0): fcoe_all_vn2vn,,
    fip.port_ka_time = jiffies +
    msecs_to_jiffies(FIP_VN_BEACON_INT +
    }
    if (time_before(fip.port_ka_time, next_time))
    pub fip->port_ka_time: next_time =,
    case FIP_ST_LINK_WAIT:
    pub unlock: goto,
    default:
    pub fip->state): WARN(1, "unexpected state %d\n",,
    pub unlock: goto,
    }
    pub next_time): mod_timer(&fip->timer,,
    unlock:
// If port ID is new, notify local port after dropping ctlr_mutex
    if (new_port_id)
    pub new_port_id): fc_lport_set_local_id(fip->lp,,
    }
//
// fcoe_ctlr_mode_set() - Set or reset the ctlr's mode
// @lport: The local port to be (re)configured
// @fip:   The FCoE controller whose mode is changing
// @fip_mode: The new fip mode
//
// Note that the we shouldn't be changing the libfc discovery settings
// (fc_disc_config) while an lport is going through the libfc state
// machine. The mode can only be changed when a fcoe_ctlr device is
// disabled, so that should ensure that this routine is only called
// when nothing is happening.
//
    static void fcoe_ctlr_mode_set(struct fc_lport *lport, struct fcoe_ctlr *fip,
    enum fip_mode fip_mode)
    {
    pub priv: *mut c_void,
    WARN_ON(lport.state != LPORT_ST_RESET &&
    pub LPORT_ST_DISABLED): lport->state !=,
    if (fip_mode == FIP_MODE_VN2VN) {
    pub fcoe_rport): lport->rport_priv_size = sizeof(struct,
    pub 1: lport->point_to_multipoint =,
    pub fcoe_ctlr_disc_recv: lport->tt.disc_recv_req =,
    pub fcoe_ctlr_disc_start: lport->tt.disc_start =,
    pub fcoe_ctlr_disc_stop: lport->tt.disc_stop =,
    pub fcoe_ctlr_disc_stop_final: lport->tt.disc_stop_final =,
    pub fip: priv =,
    } else {
    pub 0: lport->rport_priv_size =,
    pub 0: lport->point_to_multipoint =,
    pub NULL: lport->tt.disc_recv_req =,
    pub NULL: lport->tt.disc_start =,
    pub NULL: lport->tt.disc_stop =,
    pub NULL: lport->tt.disc_stop_final =,
    pub lport: priv =,
    }
    pub priv): fc_disc_config(lport,,
    }
//
// fcoe_libfc_config() - Sets up libfc related properties for local port
// @lport:    The local port to configure libfc for
// @fip:      The FCoE controller in use by the local port
// @tt:       The libfc function template
// @init_fcp: If non-zero, the FCP portion of libfc should be initialized
//
// Returns : 0 for success
//
    int fcoe_libfc_config(struct fc_lport *lport, struct fcoe_ctlr *fip,
    const struct libfc_function_template *tt, int init_fcp)
    {
// Set the function pointers set by the LLDD
    pub sizeof(*tt)): *mut memcpy(&lport->tt, tt,,
    if (init_fcp && fc_fcp_init(lport))
    pub -ENOMEM: return,
    pub fip->mode): fcoe_ctlr_mode_set(lport, fip,,
    pub 0: return,
    }
#[no_mangle]
pub unsafe extern "C" fn fcoe_fcf_get_selected(fcf_dev: *mut fcoe_fcf_device) {
    void fcoe_fcf_get_selected(struct fcoe_fcf_device *fcf_dev)
    {
    pub fcoe_fcf_dev_to_ctlr_dev(fcf_dev): *mut *mut fcoe_ctlr_device ctlr_dev =,
    pub fcoe_ctlr_device_priv(ctlr_dev): *mut *mut fcoe_ctlr fip =,
    pub fcf: *mut fcoe_fcf,
    pub fcoe_fcf_device_priv(fcf_dev): fcf =,
    if (fcf)
    pub 0: fcf_dev->selected = (fcf == fip->sel_fcf) ? 1 :,
    else
    pub 0: fcf_dev->selected =,
    }
#[no_mangle]
pub unsafe extern "C" fn fcoe_ctlr_set_fip_mode(ctlr_dev: *mut fcoe_ctlr_device) {
    void fcoe_ctlr_set_fip_mode(struct fcoe_ctlr_device *ctlr_dev)
    {
    pub fcoe_ctlr_device_priv(ctlr_dev): *mut *mut fcoe_ctlr ctlr =,
    pub ctlr->lp: *mut *mut fc_lport lport =,
    switch (ctlr_dev.mode) {
    case FIP_CONN_TYPE_VN2VN:
    pub FIP_MODE_VN2VN: ctlr->mode =,
    case FIP_CONN_TYPE_FABRIC:
    default:
    pub FIP_MODE_FABRIC: ctlr->mode =,
    }
    pub ctlr->mode): fcoe_ctlr_mode_set(lport, ctlr,,
    }
