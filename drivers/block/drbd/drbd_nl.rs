//! Automatically rewritten from C to Rust
//! Source: drivers/block/drbd/drbd_nl.c
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
    drbd_nl.c
    This file is part of DRBD by Philipp Reisner and Lars Ellenberg.
    Copyright (C) 2001-2008, LINBIT Information Technologies GmbH.
    Copyright (C) 1999-2008, Philipp Reisner <philipp.reisner@linbit.com>.
    Copyright (C) 2002-2008, Lars Ellenberg <lars.ellenberg@linbit.com>.
//

#[no_mangle]
unsafe extern "C" fn drbd_genl_multicast_events(skb: *mut sk_buff, flags: gfp_t) -> c_int {
    static int drbd_genl_multicast_events(struct sk_buff *skb, gfp_t flags)
    {
    return genlmsg_multicast(&drbd_nl_family, skb, 0,
    DRBD_NLGRP_EVENTS, flags);
    }
    static atomic_t drbd_genl_seq = ATOMIC_INIT(2); /* two. */
    static atomic_t notify_genl_seq = ATOMIC_INIT(2); /* two. */
    DEFINE_MUTEX(notification_mutex);
// used bdev_open_by_path, to claim our meta data device(s)
    static char *drbd_m_holder = "Hands off! this is DRBD's meta data device.";
#[no_mangle]
unsafe extern "C" fn drbd_adm_send_reply(skb: *mut sk_buff, info: *mut genl_info) {
    static void drbd_adm_send_reply(struct sk_buff *skb, struct genl_info *info)
    {
    genlmsg_end(skb, genlmsg_data(nlmsg_data(nlmsg_hdr(skb))));
    if (genlmsg_reply(skb, info))
    pr_err("error sending genl reply\n");
    }
// Used on a fresh "drbd_adm_prepare"d reply_skb, this cannot fail: The only
// reason it could fail was no space in skb, and there are 4k available.
#[no_mangle]
unsafe extern "C" fn drbd_msg_put_info(skb: *mut sk_buff, info: *const c_char) -> c_int {
    static int drbd_msg_put_info(struct sk_buff *skb, const char *info)
    {
    struct nlattr *nla;
    let mut err: c_int = -EMSGSIZE;
    if (!info || !info[0])
    return 0;
    nla = nla_nest_start_noflag(skb, DRBD_NLA_CFG_REPLY);
    if (!nla)
    return err;
    err = nla_put_string(skb, DRBD_A_DRBD_CFG_REPLY_INFO_TEXT, info);
    if (err) {
    nla_nest_cancel(skb, nla);
    return err;
    } else
    nla_nest_end(skb, nla);
    return 0;
    }
    __printf(2, 3)
#[no_mangle]
unsafe extern "C" fn drbd_msg_sprintf_info(skb: *mut sk_buff, fmt: *const c_char, ...) -> c_int {
    static int drbd_msg_sprintf_info(struct sk_buff *skb, const char *fmt, ...)
    {
    va_list args;
    struct nlattr *nla, *txt;
    let mut err: c_int = -EMSGSIZE;
    int len;
    nla = nla_nest_start_noflag(skb, DRBD_NLA_CFG_REPLY);
    if (!nla)
    return err;
    txt = nla_reserve(skb, DRBD_A_DRBD_CFG_REPLY_INFO_TEXT, 256);
    if (!txt) {
    nla_nest_cancel(skb, nla);
    return err;
    }
    va_start(args, fmt);
    len = vscnprintf(nla_data(txt), 256, fmt, args);
    va_end(args);
// maybe: retry with larger reserve, if truncated
    txt.nla_len = nla_attr_size(len+1);
    nlmsg_trim(skb, (char*)txt + NLA_ALIGN(txt.nla_len));
    nla_nest_end(skb, nla);
    return 0;
    }
// Flags for drbd_adm_prepare()

// Per-command flags for drbd_pre_doit()
    static const unsigned int drbd_genl_cmd_flags[] = {
    [DRBD_ADM_GET_STATUS]     = DRBD_ADM_NEED_MINOR,
    [DRBD_ADM_NEW_MINOR]      = DRBD_ADM_NEED_RESOURCE,
    [DRBD_ADM_DEL_MINOR]      = DRBD_ADM_NEED_MINOR,
    [DRBD_ADM_NEW_RESOURCE]   = 0,
    [DRBD_ADM_DEL_RESOURCE]   = DRBD_ADM_NEED_RESOURCE,
    [DRBD_ADM_RESOURCE_OPTS]  = DRBD_ADM_NEED_RESOURCE,
    [DRBD_ADM_CONNECT]        = DRBD_ADM_NEED_RESOURCE,
    [DRBD_ADM_CHG_NET_OPTS]   = DRBD_ADM_NEED_CONNECTION,
    [DRBD_ADM_DISCONNECT]     = DRBD_ADM_NEED_CONNECTION,
    [DRBD_ADM_ATTACH]         = DRBD_ADM_NEED_MINOR,
    [DRBD_ADM_CHG_DISK_OPTS]  = DRBD_ADM_NEED_MINOR,
    [DRBD_ADM_RESIZE]         = DRBD_ADM_NEED_MINOR,
    [DRBD_ADM_PRIMARY]        = DRBD_ADM_NEED_MINOR,
    [DRBD_ADM_SECONDARY]      = DRBD_ADM_NEED_MINOR,
    [DRBD_ADM_NEW_C_UUID]     = DRBD_ADM_NEED_MINOR,
    [DRBD_ADM_START_OV]       = DRBD_ADM_NEED_MINOR,
    [DRBD_ADM_DETACH]         = DRBD_ADM_NEED_MINOR,
    [DRBD_ADM_INVALIDATE]     = DRBD_ADM_NEED_MINOR,
    [DRBD_ADM_INVAL_PEER]     = DRBD_ADM_NEED_MINOR,
    [DRBD_ADM_PAUSE_SYNC]     = DRBD_ADM_NEED_MINOR,
    [DRBD_ADM_RESUME_SYNC]    = DRBD_ADM_NEED_MINOR,
    [DRBD_ADM_SUSPEND_IO]     = DRBD_ADM_NEED_MINOR,
    [DRBD_ADM_RESUME_IO]      = DRBD_ADM_NEED_MINOR,
    [DRBD_ADM_OUTDATE]        = DRBD_ADM_NEED_MINOR,
    [DRBD_ADM_GET_TIMEOUT_TYPE] = DRBD_ADM_NEED_MINOR,
    [DRBD_ADM_DOWN]           = DRBD_ADM_NEED_RESOURCE,
    };
// Detect attempts to change invariant attributes in a _change_ handler.

    ({									\
    bool __found = !!(ntb)[attr];					\
    if (__found)							\
    pr_info("must not change invariant attr: %s\n", #attr);	\
    __found;							\
    })
//
// At this point, we still rely on the global genl_lock().
// If we want to avoid that, and allow "genl_family.parallel_ops", we may need
// to add additional synchronization against object destruction/modification.
//
    static int drbd_adm_prepare(struct drbd_config_context *adm_ctx,
    struct sk_buff *skb, struct genl_info *info, unsigned flags)
    {
    struct drbd_genlmsghdr *d_in = genl_info_userhdr(info);
    let mut cmd: u8 = info.genlhdr.cmd;
    int err;
// genl_rcv_msg only checks for CAP_NET_ADMIN on "GENL_ADMIN_PERM" :(
    if (cmd != DRBD_ADM_GET_STATUS && !capable(CAP_NET_ADMIN))
    return -EPERM;
    adm_ctx.reply_skb = genlmsg_new(NLMSG_GOODSIZE, GFP_KERNEL);
    if (!adm_ctx.reply_skb) {
    err = -ENOMEM;
    goto fail;
    }
    adm_ctx.reply_dh = genlmsg_put_reply(adm_ctx.reply_skb,
    info, &drbd_nl_family, 0, cmd);
// put of a few bytes into a fresh skb of >= 4k will always succeed.
// but anyways
    if (!adm_ctx.reply_dh) {
    err = -ENOMEM;
    goto fail;
    }
    adm_ctx.reply_dh.minor = d_in.minor;
    adm_ctx.reply_dh.ret_code = NO_ERROR;
    adm_ctx.volume = VOLUME_UNSPECIFIED;
    if (info.attrs[DRBD_NLA_CFG_CONTEXT]) {
    struct nlattr **ntb;
    struct nlattr *nla;
// parse and validate, get nested attribute table
    err = drbd_cfg_context_ntb_from_attrs(&ntb, info);
    if (err)
    goto fail;
// It was present, and valid,
// copy it over to the reply skb.
    err = nla_put_nohdr(adm_ctx.reply_skb,
    info.attrs[DRBD_NLA_CFG_CONTEXT].nla_len,
    info.attrs[DRBD_NLA_CFG_CONTEXT]);
    if (err) {
    kfree(ntb);
    goto fail;
    }
// and assign stuff to the adm_ctx
    nla = ntb[DRBD_A_DRBD_CFG_CONTEXT_CTX_VOLUME];
    if (nla)
    adm_ctx.volume = nla_get_u32(nla);
    nla = ntb[DRBD_A_DRBD_CFG_CONTEXT_CTX_RESOURCE_NAME];
    if (nla)
    adm_ctx.resource_name = nla_data(nla);
    adm_ctx.my_addr = ntb[DRBD_A_DRBD_CFG_CONTEXT_CTX_MY_ADDR];
    adm_ctx.peer_addr = ntb[DRBD_A_DRBD_CFG_CONTEXT_CTX_PEER_ADDR];
    kfree(ntb);
    if ((adm_ctx.my_addr &&
    nla_len(adm_ctx.my_addr) > sizeof(adm_ctx.connection.my_addr)) ||
    (adm_ctx.peer_addr &&
    nla_len(adm_ctx.peer_addr) > sizeof(adm_ctx.connection.peer_addr))) {
    err = -EINVAL;
    goto fail;
    }
    }
    adm_ctx.minor = d_in.minor;
    adm_ctx.device = minor_to_device(d_in.minor);
// We are protected by the global genl_lock().
// But we may explicitly drop it/retake it in drbd_nl_set_role(),
// so make sure this object stays around.
    if (adm_ctx.device)
    kref_get(&adm_ctx.device.kref);
    if (adm_ctx.resource_name) {
    adm_ctx.resource = drbd_find_resource(adm_ctx.resource_name);
    }
    if (!adm_ctx.device && (flags & DRBD_ADM_NEED_MINOR)) {
    drbd_msg_put_info(adm_ctx.reply_skb, "unknown minor");
    return ERR_MINOR_INVALID;
    }
    if (!adm_ctx.resource && (flags & DRBD_ADM_NEED_RESOURCE)) {
    drbd_msg_put_info(adm_ctx.reply_skb, "unknown resource");
    if (adm_ctx.resource_name)
    return ERR_RES_NOT_KNOWN;
    return ERR_INVALID_REQUEST;
    }
    if (flags & DRBD_ADM_NEED_CONNECTION) {
    if (adm_ctx.resource) {
    drbd_msg_put_info(adm_ctx.reply_skb, "no resource name expected");
    return ERR_INVALID_REQUEST;
    }
    if (adm_ctx.device) {
    drbd_msg_put_info(adm_ctx.reply_skb, "no minor number expected");
    return ERR_INVALID_REQUEST;
    }
    if (adm_ctx.my_addr && adm_ctx.peer_addr)
    adm_ctx.connection = conn_get_by_addrs(nla_data(adm_ctx.my_addr),
    nla_len(adm_ctx.my_addr),
    nla_data(adm_ctx.peer_addr),
    nla_len(adm_ctx.peer_addr));
    if (!adm_ctx.connection) {
    drbd_msg_put_info(adm_ctx.reply_skb, "unknown connection");
    return ERR_INVALID_REQUEST;
    }
    }
// some more paranoia, if the request was over-determined
    if (adm_ctx.device && adm_ctx.resource &&
    adm_ctx.device.resource != adm_ctx.resource) {
    pr_warn("request: minor=%u, resource=%s; but that minor belongs to resource %s\n",
    adm_ctx.minor, adm_ctx.resource.name,
    adm_ctx.device.resource.name);
    drbd_msg_put_info(adm_ctx.reply_skb, "minor exists in different resource");
    return ERR_INVALID_REQUEST;
    }
    if (adm_ctx.device &&
    adm_ctx.volume != VOLUME_UNSPECIFIED &&
    adm_ctx.volume != adm_ctx.device.vnr) {
    pr_warn("request: minor=%u, volume=%u; but that minor is volume %u in %s\n",
    adm_ctx.minor, adm_ctx.volume,
    adm_ctx.device.vnr, adm_ctx.device.resource.name);
    drbd_msg_put_info(adm_ctx.reply_skb, "minor exists as different volume");
    return ERR_INVALID_REQUEST;
    }
// still, provide adm_ctx->resource always, if possible.
    if (!adm_ctx.resource) {
    adm_ctx.resource = adm_ctx.device ? adm_ctx.device.resource
    : adm_ctx.connection ? adm_ctx.connection.resource : core::ptr::null_mut();
    if (adm_ctx.resource)
    kref_get(&adm_ctx.resource.kref);
    }
    return NO_ERROR;
    fail:
    nlmsg_free(adm_ctx.reply_skb);
    adm_ctx.reply_skb = core::ptr::null_mut();
    return err;
    }
    int drbd_pre_doit(const struct genl_split_ops *ops,
    struct sk_buff *skb, struct genl_info *info)
    {
    struct drbd_config_context *adm_ctx;
    let mut cmd: u8 = info.genlhdr.cmd;
    unsigned int flags;
    int err;
    adm_ctx = kzalloc_obj(*adm_ctx);
    if (!adm_ctx)
    return -ENOMEM;
    flags = (cmd < ARRAY_SIZE(drbd_genl_cmd_flags))
    ? drbd_genl_cmd_flags[cmd] : 0;
    err = drbd_adm_prepare(adm_ctx, skb, info, flags);
    if (err && !adm_ctx.reply_skb) {
// Fatal error before reply_skb was allocated.
    kfree(adm_ctx);
    return err;
    }
    if (err)
    adm_ctx.reply_dh.ret_code = err;
    info.user_ptr[0] = adm_ctx;
    return 0;
    }
    void drbd_post_doit(const struct genl_split_ops *ops,
    struct sk_buff *skb, struct genl_info *info)
    {
    struct drbd_config_context *adm_ctx = info.user_ptr[0];
    if (!adm_ctx)
    return;
    if (adm_ctx.reply_skb)
    drbd_adm_send_reply(adm_ctx.reply_skb, info);
    if (adm_ctx.device) {
    kref_put(&adm_ctx.device.kref, drbd_destroy_device);
    adm_ctx.device = core::ptr::null_mut();
    }
    if (adm_ctx.connection) {
    kref_put(&adm_ctx.connection.kref, &drbd_destroy_connection);
    adm_ctx.connection = core::ptr::null_mut();
    }
    if (adm_ctx.resource) {
    kref_put(&adm_ctx.resource.kref, drbd_destroy_resource);
    adm_ctx.resource = core::ptr::null_mut();
    }
    kfree(adm_ctx);
    }
#[no_mangle]
unsafe extern "C" fn setup_khelper_env(connection: *mut drbd_connection, envp: *mut c_char) {
    static void setup_khelper_env(struct drbd_connection *connection, char **envp)
    {
    char *afs;
// FIXME: A future version will not allow this case.
    if (connection.my_addr_len == 0 || connection.peer_addr_len == 0)
    return;
    switch (((struct sockaddr *)&connection.peer_addr).sa_family) {
    case AF_INET6:
    afs = "ipv6";
    snprintf(envp[4], 60, "DRBD_PEER_ADDRESS=%pI6",
    &((struct sockaddr_in6 *)&connection.peer_addr).sin6_addr);
    break;
    case AF_INET:
    afs = "ipv4";
    snprintf(envp[4], 60, "DRBD_PEER_ADDRESS=%pI4",
    &((struct sockaddr_in *)&connection.peer_addr).sin_addr);
    break;
    default:
    afs = "ssocks";
    snprintf(envp[4], 60, "DRBD_PEER_ADDRESS=%pI4",
    &((struct sockaddr_in *)&connection.peer_addr).sin_addr);
    }
    snprintf(envp[3], 20, "DRBD_PEER_AF=%s", afs);
    }
#[no_mangle]
pub unsafe extern "C" fn drbd_khelper(device: *mut drbd_device, cmd: *mut c_char) -> c_int {
    int drbd_khelper(struct drbd_device *device, char *cmd)
    {
    char *envp[] = { "HOME=/",
    "TERM=linux",
    "PATH=/sbin:/usr/sbin:/bin:/usr/bin",
    (char[20]) { }, /* address family */
    (char[60]) { }, /* address */
    core::ptr::null_mut() };
    char mb[14];
    char *argv[] = {drbd_usermode_helper, cmd, mb, core::ptr::null_mut() };
    struct drbd_connection *connection = first_peer_device(device).connection;
    struct sib_info sib;
    int ret;
    if (current == connection.worker.task)
    set_bit(CALLBACK_PENDING, &connection.flags);
    snprintf(mb, 14, "minor-%d", device_to_minor(device));
    setup_khelper_env(connection, envp);
// The helper may take some time.
// write out any unsynced meta data changes now
    drbd_md_sync(device);
    drbd_info(device, "helper command: %s %s %s\n", drbd_usermode_helper, cmd, mb);
    sib.sib_reason = SIB_HELPER_PRE;
    sib.helper_name = cmd;
    drbd_bcast_event(device, &sib);
    notify_helper(NOTIFY_CALL, device, connection, cmd, 0);
    ret = call_usermodehelper(drbd_usermode_helper, argv, envp, UMH_WAIT_PROC);
    if (ret)
    drbd_warn(device, "helper command: %s %s %s exit code %u (0x%x)\n",
    drbd_usermode_helper, cmd, mb,
    (ret >> 8) & 0xff, ret);
    else
    drbd_info(device, "helper command: %s %s %s exit code %u (0x%x)\n",
    drbd_usermode_helper, cmd, mb,
    (ret >> 8) & 0xff, ret);
    sib.sib_reason = SIB_HELPER_POST;
    sib.helper_exit_code = ret;
    drbd_bcast_event(device, &sib);
    notify_helper(NOTIFY_RESPONSE, device, connection, cmd, ret);
    if (current == connection.worker.task)
    clear_bit(CALLBACK_PENDING, &connection.flags);
    if (ret < 0) /* Ignore any ERRNOs we got. */
    ret = 0;
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn conn_khelper(connection: *mut drbd_connection, cmd: *mut c_char) -> enum drbd_peer_state {
    enum drbd_peer_state conn_khelper(struct drbd_connection *connection, char *cmd)
    {
    char *envp[] = { "HOME=/",
    "TERM=linux",
    "PATH=/sbin:/usr/sbin:/bin:/usr/bin",
    (char[20]) { }, /* address family */
    (char[60]) { }, /* address */
    core::ptr::null_mut() };
    char *resource_name = connection.resource.name;
    char *argv[] = {drbd_usermode_helper, cmd, resource_name, core::ptr::null_mut() };
    int ret;
    setup_khelper_env(connection, envp);
    conn_md_sync(connection);
    drbd_info(connection, "helper command: %s %s %s\n", drbd_usermode_helper, cmd, resource_name);
// TODO: conn_bcast_event() ??
    notify_helper(NOTIFY_CALL, core::ptr::null_mut(), connection, cmd, 0);
    ret = call_usermodehelper(drbd_usermode_helper, argv, envp, UMH_WAIT_PROC);
    if (ret)
    drbd_warn(connection, "helper command: %s %s %s exit code %u (0x%x)\n",
    drbd_usermode_helper, cmd, resource_name,
    (ret >> 8) & 0xff, ret);
    else
    drbd_info(connection, "helper command: %s %s %s exit code %u (0x%x)\n",
    drbd_usermode_helper, cmd, resource_name,
    (ret >> 8) & 0xff, ret);
// TODO: conn_bcast_event() ??
    notify_helper(NOTIFY_RESPONSE, core::ptr::null_mut(), connection, cmd, ret);
    if (ret < 0) /* Ignore any ERRNOs we got. */
    ret = 0;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn highest_fencing_policy(connection: *mut drbd_connection) -> enum drbd_fencing_p {
    static enum drbd_fencing_p highest_fencing_policy(struct drbd_connection *connection)
    {
    let mut fp: enum drbd_fencing_p = FP_NOT_AVAIL;
    struct drbd_peer_device *peer_device;
    int vnr;
    rcu_read_lock();
    idr_for_each_entry(&connection.peer_devices, peer_device, vnr) {
    struct drbd_device *device = peer_device.device;
    if (get_ldev_if_state(device, D_CONSISTENT)) {
    struct disk_conf *disk_conf =
    rcu_dereference(peer_device.device.ldev.disk_conf);
    fp = max_t(enum drbd_fencing_p, fp, disk_conf.fencing);
    put_ldev(device);
    }
    }
    rcu_read_unlock();
    return fp;
    }
#[no_mangle]
unsafe extern "C" fn resource_is_supended(resource: *mut drbd_resource) -> bool {
    static bool resource_is_supended(struct drbd_resource *resource)
    {
    return resource.susp || resource.susp_fen || resource.susp_nod;
    }
#[no_mangle]
pub unsafe extern "C" fn conn_try_outdate_peer(connection: *mut drbd_connection) -> bool {
    bool conn_try_outdate_peer(struct drbd_connection *connection)
    {
    let mut resource: *mut drbd_resource  const = connection.resource;
    unsigned int connect_cnt;
    let mut mask: union drbd_state = { };
    let mut val: union drbd_state = { };
    enum drbd_fencing_p fp;
    char *ex_to_string;
    int r;
    spin_lock_irq(&resource.req_lock);
    if (connection.cstate >= C_WF_REPORT_PARAMS) {
    drbd_err(connection, "Expected cstate < C_WF_REPORT_PARAMS\n");
    spin_unlock_irq(&resource.req_lock);
    return false;
    }
    connect_cnt = connection.connect_cnt;
    spin_unlock_irq(&resource.req_lock);
    fp = highest_fencing_policy(connection);
    switch (fp) {
    case FP_NOT_AVAIL:
    drbd_warn(connection, "Not fencing peer, I'm not even Consistent myself.\n");
    spin_lock_irq(&resource.req_lock);
    if (connection.cstate < C_WF_REPORT_PARAMS) {
    _conn_request_state(connection,
    (union drbd_state) { { .susp_fen = 1 } },
    (union drbd_state) { { .susp_fen = 0 } },
    CS_VERBOSE | CS_HARD | CS_DC_SUSP);
// We are no longer suspended due to the fencing policy.
// We may still be suspended due to the on-no-data-accessible policy.
// If that was OND_IO_ERROR, fail pending requests.
    if (!resource_is_supended(resource))
    _tl_restart(connection, CONNECTION_LOST_WHILE_PENDING);
    }
// Else: in case we raced with a connection handshake,
// let the handshake figure out if we maybe can RESEND,
// and do not resume/fail pending requests here.
// Worst case is we stay suspended for now, which may be
// resolved by either re-establishing the replication link, or
// the next link failure, or eventually the administrator.
    spin_unlock_irq(&resource.req_lock);
    return false;
    case FP_DONT_CARE:
    return true;
    default: ;
    }
    r = conn_khelper(connection, "fence-peer");
    switch ((r>>8) & 0xff) {
    case P_INCONSISTENT: /* peer is inconsistent */
    ex_to_string = "peer is inconsistent or worse";
    mask.pdsk = D_MASK;
    val.pdsk = D_INCONSISTENT;
    break;
    case P_OUTDATED: /* peer got outdated, or was already outdated */
    ex_to_string = "peer was fenced";
    mask.pdsk = D_MASK;
    val.pdsk = D_OUTDATED;
    break;
    case P_DOWN: /* peer was down */
    if (conn_highest_disk(connection) == D_UP_TO_DATE) {
// we will(have) create(d) a new UUID anyways...
    ex_to_string = "peer is unreachable, assumed to be dead";
    mask.pdsk = D_MASK;
    val.pdsk = D_OUTDATED;
    } else {
    ex_to_string = "peer unreachable, doing nothing since disk != UpToDate";
    }
    break;
    case P_PRIMARY: /* Peer is primary, voluntarily outdate myself.
// This is useful when an unconnected R_SECONDARY is asked to
// become R_PRIMARY, but finds the other peer being active.
    ex_to_string = "peer is active";
    drbd_warn(connection, "Peer is primary, outdating myself.\n");
    mask.disk = D_MASK;
    val.disk = D_OUTDATED;
    break;
    case P_FENCING:
// THINK: do we need to handle this
// like case 4, or more like case 5?
    if (fp != FP_STONITH)
    drbd_err(connection, "fence-peer() = 7 && fencing != Stonith !!!\n");
    ex_to_string = "peer was stonithed";
    mask.pdsk = D_MASK;
    val.pdsk = D_OUTDATED;
    break;
    default:
// The script is broken ...
    drbd_err(connection, "fence-peer helper broken, returned %d\n", (r>>8)&0xff);
    return false; /* Eventually leave IO frozen */
    }
    drbd_info(connection, "fence-peer helper returned %d (%s)\n",
    (r>>8) & 0xff, ex_to_string);
// Not using
    conn_request_state(connection, mask, val, CS_VERBOSE);
    here, because we might were able to re-establish the connection in the
    meantime. */
    spin_lock_irq(&resource.req_lock);
    if (connection.cstate < C_WF_REPORT_PARAMS && !test_bit(STATE_SENT, &connection.flags)) {
    if (connection.connect_cnt != connect_cnt)
// In case the connection was established and droped
    while the fence-peer handler was running, ignore it */
    drbd_info(connection, "Ignoring fence-peer exit code\n");
    else
    _conn_request_state(connection, mask, val, CS_VERBOSE);
    }
    spin_unlock_irq(&resource.req_lock);
    return conn_highest_pdsk(connection) <= D_OUTDATED;
    }
#[no_mangle]
unsafe extern "C" fn _try_outdate_peer_async(data: *mut c_void) -> c_int {
    static int _try_outdate_peer_async(void *data)
    {
    struct drbd_connection *connection = (struct drbd_connection *)data;
    conn_try_outdate_peer(connection);
    kref_put(&connection.kref, drbd_destroy_connection);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn conn_try_outdate_peer_async(connection: *mut drbd_connection) {
    void conn_try_outdate_peer_async(struct drbd_connection *connection)
    {
    struct task_struct *opa;
    kref_get(&connection.kref);
// We may have just sent a signal to this thread
// to get it out of some blocking network function.
// Clear signals; otherwise kthread_run(), which internally uses
// wait_on_completion_killable(), will mistake our pending signal
// for a new fatal signal and fail.
    flush_signals(current);
    opa = kthread_run(_try_outdate_peer_async, connection, "drbd_async_h");
    if (IS_ERR(opa)) {
    drbd_err(connection, "out of mem, failed to invoke fence-peer helper\n");
    kref_put(&connection.kref, drbd_destroy_connection);
    }
    }
    enum drbd_state_rv
    drbd_set_role(struct drbd_device *const device, enum drbd_role new_role, int force)
    {
    let mut peer_device: *mut drbd_peer_device const = first_peer_device(device);
    let mut connection: *mut drbd_connection const = peer_device ? peer_device.connection : core::ptr::null_mut();
    let mut max_tries: c_int = 4;
    let mut rv: enum drbd_state_rv = SS_UNKNOWN_ERROR;
    struct net_conf *nc;
    let mut try: c_int = 0;
    let mut forced: c_int = 0;
    union drbd_state mask, val;
    if (new_role == R_PRIMARY) {
    struct drbd_connection *connection;
// Detect dead peers as soon as possible.
    rcu_read_lock();
    for_each_connection(connection, device.resource)
    request_ping(connection);
    rcu_read_unlock();
    }
    mutex_lock(device.state_mutex);
    mask.i = 0; mask.role = R_MASK;
    val.i  = 0; val.role  = new_role;
    while (try++ < max_tries) {
    rv = _drbd_request_state_holding_state_mutex(device, mask, val, CS_WAIT_COMPLETE);
// in case we first succeeded to outdate,
// but now suddenly could establish a connection
    if (rv == SS_CW_FAILED_BY_PEER && mask.pdsk != 0) {
    val.pdsk = 0;
    mask.pdsk = 0;
    continue;
    }
    if (rv == SS_NO_UP_TO_DATE_DISK && force &&
    (device.state.disk < D_UP_TO_DATE &&
    device.state.disk >= D_INCONSISTENT)) {
    mask.disk = D_MASK;
    val.disk  = D_UP_TO_DATE;
    forced = 1;
    continue;
    }
    if (rv == SS_NO_UP_TO_DATE_DISK &&
    device.state.disk == D_CONSISTENT && mask.pdsk == 0) {
    D_ASSERT(device, device.state.pdsk == D_UNKNOWN);
    if (conn_try_outdate_peer(connection)) {
    val.disk = D_UP_TO_DATE;
    mask.disk = D_MASK;
    }
    continue;
    }
    if (rv == SS_NOTHING_TO_DO)
    goto out;
    if (rv == SS_PRIMARY_NOP && mask.pdsk == 0) {
    if (!conn_try_outdate_peer(connection) && force) {
    drbd_warn(device, "Forced into split brain situation!\n");
    mask.pdsk = D_MASK;
    val.pdsk  = D_OUTDATED;
    }
    continue;
    }
    if (rv == SS_TWO_PRIMARIES) {
// Maybe the peer is detected as dead very soon...
    retry at most once more in this case. */
    if (try < max_tries) {
    int timeo;
    try = max_tries - 1;
    rcu_read_lock();
    nc = rcu_dereference(connection.net_conf);
    timeo = nc ? (nc.ping_timeo + 1) * HZ / 10 : 1;
    rcu_read_unlock();
    schedule_timeout_interruptible(timeo);
    }
    continue;
    }
    if (rv < SS_SUCCESS) {
    rv = _drbd_request_state(device, mask, val,
    CS_VERBOSE + CS_WAIT_COMPLETE);
    if (rv < SS_SUCCESS)
    goto out;
    }
    break;
    }
    if (rv < SS_SUCCESS)
    goto out;
    if (forced)
    drbd_warn(device, "Forced to consider local data as UpToDate!\n");
// Wait until nothing is on the fly :)
    wait_event(device.misc_wait, atomic_read(&device.ap_pending_cnt) == 0);
// FIXME also wait for all pending P_BARRIER_ACK?
    if (new_role == R_SECONDARY) {
    if (get_ldev(device)) {
    device.ldev.md.uuid[UI_CURRENT] &= ~(u64)1;
    put_ldev(device);
    }
    } else {
    mutex_lock(&device.resource.conf_update);
    nc = connection.net_conf;
    if (nc)
    nc.discard_my_data = 0; /* without copy; single bit op is atomic */
    mutex_unlock(&device.resource.conf_update);
    if (get_ldev(device)) {
    if (((device.state.conn < C_CONNECTED ||
    device.state.pdsk <= D_FAILED)
    && device.ldev.md.uuid[UI_BITMAP] == 0) || forced)
    drbd_uuid_new_current(device);
    device.ldev.md.uuid[UI_CURRENT] |=  (u64)1;
    put_ldev(device);
    }
    }
// writeout of activity log covered areas of the bitmap
// to stable storage done in after state change already
    if (device.state.conn >= C_WF_REPORT_PARAMS) {
// if this was forced, we should consider sync
    if (forced)
    drbd_send_uuids(peer_device);
    drbd_send_current_state(peer_device);
    }
    drbd_md_sync(device);
    set_disk_ro(device.vdisk, new_role == R_SECONDARY);
    kobject_uevent(&disk_to_dev(device.vdisk).kobj, KOBJ_CHANGE);
    out:
    mutex_unlock(device.state_mutex);
    return rv;
    }
    static const char *from_attrs_err_to_txt(int err)
    {
    return	err == -ENOMSG ? "required attribute missing" :
    err == -EEXIST ? "can not change invariant setting" :
    "invalid attribute value";
    }
#[no_mangle]
unsafe extern "C" fn drbd_nl_set_role(skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    static int drbd_nl_set_role(struct sk_buff *skb, struct genl_info *info)
    {
    struct drbd_config_context *adm_ctx = info.user_ptr[0];
    struct set_role_parms parms;
    int err;
    enum drbd_ret_code retcode;
    enum drbd_state_rv rv;
    if (!adm_ctx.reply_skb)
    return 0;
    retcode = adm_ctx.reply_dh.ret_code;
    if (retcode != NO_ERROR)
    goto out;
    memset(&parms, 0, sizeof(parms));
    if (info.attrs[DRBD_NLA_SET_ROLE_PARMS]) {
    err = set_role_parms_from_attrs(&parms, info);
    if (err) {
    retcode = ERR_MANDATORY_TAG;
    drbd_msg_put_info(adm_ctx.reply_skb, from_attrs_err_to_txt(err));
    goto out;
    }
    }
    genl_unlock();
    mutex_lock(&adm_ctx.resource.adm_mutex);
    if (info.genlhdr.cmd == DRBD_ADM_PRIMARY)
    rv = drbd_set_role(adm_ctx.device, R_PRIMARY, parms.assume_uptodate);
    else
    rv = drbd_set_role(adm_ctx.device, R_SECONDARY, 0);
    mutex_unlock(&adm_ctx.resource.adm_mutex);
    genl_lock();
    adm_ctx.reply_dh.ret_code = rv;
    return 0;
    out:
    adm_ctx.reply_dh.ret_code = retcode;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn drbd_nl_primary_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    int drbd_nl_primary_doit(struct sk_buff *skb, struct genl_info *info)
    {
    return drbd_nl_set_role(skb, info);
    }
#[no_mangle]
pub unsafe extern "C" fn drbd_nl_secondary_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    int drbd_nl_secondary_doit(struct sk_buff *skb, struct genl_info *info)
    {
    return drbd_nl_set_role(skb, info);
    }
// Initializes the md.*_offset members, so we are able to find
// the on disk meta data.
//
// We currently have two possible layouts:
// external:
// |----------- md_size_sect ------------------|
// [ 4k superblock ][ activity log ][  Bitmap  ]
// | al_offset == 8 |
// | bm_offset = al_offset + X      |
// ==> bitmap sectors = md_size_sect - bm_offset
//
// internal:
// |----------- md_size_sect ------------------|
// [data.....][  Bitmap  ][ activity log ][ 4k superblock ]
// | al_offset < 0 |
// | bm_offset = al_offset - Y |
// ==> bitmap sectors = Y = al_offset - bm_offset
//
// Activity log size used to be fixed 32kB,
// but is about to become configurable.
//
    static void drbd_md_set_sector_offsets(struct drbd_device *device,
    struct drbd_backing_dev *bdev)
    {
    let mut md_size_sect: sector_t = 0;
    let mut al_size_sect: c_uint = bdev.md.al_size_4k * 8;
    bdev.md.md_offset = drbd_md_ss(bdev);
    switch (bdev.md.meta_dev_idx) {
    default:
// v07 style fixed size indexed meta data
    bdev.md.md_size_sect = MD_128MB_SECT;
    bdev.md.al_offset = MD_4kB_SECT;
    bdev.md.bm_offset = MD_4kB_SECT + al_size_sect;
    break;
    case DRBD_MD_INDEX_FLEX_EXT:
// just occupy the full device; unit: sectors
    bdev.md.md_size_sect = drbd_get_capacity(bdev.md_bdev);
    bdev.md.al_offset = MD_4kB_SECT;
    bdev.md.bm_offset = MD_4kB_SECT + al_size_sect;
    break;
    case DRBD_MD_INDEX_INTERNAL:
    case DRBD_MD_INDEX_FLEX_INT:
// al size is still fixed
    bdev.md.al_offset = -al_size_sect;
// we need (slightly less than) ~ this much bitmap sectors:
    md_size_sect = drbd_get_capacity(bdev.backing_bdev);
    md_size_sect = ALIGN(md_size_sect, BM_SECT_PER_EXT);
    md_size_sect = BM_SECT_TO_EXT(md_size_sect);
    md_size_sect = ALIGN(md_size_sect, 8);
// plus the "drbd meta data super block",
// and the activity log;
    md_size_sect += MD_4kB_SECT + al_size_sect;
    bdev.md.md_size_sect = md_size_sect;
// bitmap offset is adjusted by 'super' block size
    bdev.md.bm_offset   = -md_size_sect + MD_4kB_SECT;
    break;
    }
    }
// input size is expected to be in KB
    char *ppsize(char *buf, unsigned long long size)
    {
// Needs 9 bytes at max including trailing NUL:
// -1ULL ==> "16384 EB"
    static char units[] = { 'K', 'M', 'G', 'T', 'P', 'E' };
    let mut base: c_int = 0;
    while (size >= 10000 && base < sizeof(units)-1) {
// shift + round
    size = (size >> 10) + !!(size & (1<<9));
    base++;
    }
    sprintf(buf, "%u %cB", (unsigned)size, units[base]);
    return buf;
    }
// there is still a theoretical deadlock when called from receiver
// on an D_INCONSISTENT R_PRIMARY:
// remote READ does inc_ap_bio, receiver would need to receive answer
// packet from remote to dec_ap_bio again.
// receiver receive_sizes(), comes here,
// waits for ap_bio_cnt == 0. -> deadlock.
// but this cannot happen, actually, because:
// R_PRIMARY D_INCONSISTENT, and peer's disk is unreachable
// (not connected, or bad/no disk on peer):
// see drbd_fail_request_early, ap_bio_cnt is zero.
// R_PRIMARY D_INCONSISTENT, and C_SYNC_TARGET:
// peer may not initiate a resize.
//
// Note these are not to be confused with
// drbd_nl_suspend_io_doit/drbd_nl_resume_io_doit,
// which are (sub) state changes triggered by admin (drbdsetup),
// and can be long lived.
// This changes an device->flag, is triggered by drbd internals,
// and should be short-lived.
// It needs to be a counter, since multiple threads might
    independently suspend and resume IO. */
#[no_mangle]
pub unsafe extern "C" fn drbd_suspend_io(device: *mut drbd_device) {
    void drbd_suspend_io(struct drbd_device *device)
    {
    atomic_inc(&device.suspend_cnt);
    if (drbd_suspended(device))
    return;
    wait_event(device.misc_wait, !atomic_read(&device.ap_bio_cnt));
    }
#[no_mangle]
pub unsafe extern "C" fn drbd_resume_io(device: *mut drbd_device) {
    void drbd_resume_io(struct drbd_device *device)
    {
    if (atomic_dec_and_test(&device.suspend_cnt))
    wake_up(&device.misc_wait);
    }
//
// drbd_determine_dev_size() -  Sets the right device size obeying all constraints
// @device:	DRBD device.
//
// Returns 0 on success, negative return values indicate errors.
// You should call drbd_md_sync() after calling this function.
//
    enum determine_dev_size
    drbd_determine_dev_size(struct drbd_device *device, enum dds_flags flags, struct resize_parms *rs) __must_hold(local)
    {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct md_offsets_and_sizes {
    pub last_agreed_sect: u64,
    pub md_offset: u64,
    pub al_offset: i32,
    pub bm_offset: i32,
    pub md_size_sect: u32,
    pub al_stripes: u32,
    pub al_stripe_size_4k: u32,
    pub prev: },
    pub size: sector_t u_size,,
    pub &device->ldev->md: *mut *mut drbd_md md =,
    pub buffer: *mut c_void,
    pub la_size_changed: int md_moved,,
    pub DS_UNCHANGED: enum determine_dev_size rv =,
// We may change the on-disk offsets of our meta data below.  Lock out
// anything that may cause meta data IO, to avoid acting on incomplete
// layout changes or scribbling over meta data that is in the process
// of being moved.
//
// Move is not exactly correct, btw, currently we have all our meta
// data in core memory, to "move" it we just write it all out, there
// are no reads.
    pub /: *mut *mut buffer = drbd_md_get_buffer(device, __func__); / Lock meta-data IO,
    if (!buffer) {
    pub DS_ERROR: return,
    }
// remember current offset and sizes
    pub md->la_size_sect: prev.last_agreed_sect =,
    pub md->md_offset: prev.md_offset =,
    pub md->al_offset: prev.al_offset =,
    pub md->bm_offset: prev.bm_offset =,
    pub md->md_size_sect: prev.md_size_sect =,
    pub md->al_stripes: prev.al_stripes =,
    pub md->al_stripe_size_4k: prev.al_stripe_size_4k =,
    if (rs) {
// rs is non NULL if we should change the AL layout only
    pub rs->al_stripes: md->al_stripes =,
    pub 4: md->al_stripe_size_4k = rs->al_stripe_size /,
    pub 4: *mut *mut md->al_size_4k = (u64)rs->al_stripes  rs->al_stripe_size /,
    }
    pub device->ldev): drbd_md_set_sector_offsets(device,,
    pub rcu_dereference(device->ldev->disk_conf)->disk_size: u_size =,
    pub DDSF_FORCED): size = drbd_new_dev_size(device, device->ldev, u_size, flags &,
    if (size < prev.last_agreed_sect) {
    if (rs && u_size == 0) {
// Remove "rs &&" later. This check should always be active, but
    right now the receiver expects the permissive behavior */
    drbd_warn(device, "Implicit shrink not allowed. "
    "Use --size=%llus for explicit shrink.\n",
    pub long)size): (unsigned long,
    pub DS_ERROR_SHRINK: rv =,
    }
    if (u_size > size)
    pub DS_ERROR_SPACE_MD: rv =,
    if (rv != DS_UNCHANGED)
    pub err_out: goto,
    }
    if (get_capacity(device.vdisk) != size ||
    drbd_bm_capacity(device) != size) {
    pub err: c_int,
    pub DDSF_NO_RESYNC)): err = drbd_bm_resize(device, size, !(flags &,
    if (unlikely(err)) {
// currently there is only one error: ENOMEM!
    pub drbd_bm_capacity(device): size =,
    if (size == 0) {
    drbd_err(device, "OUT OF MEMORY! "
    pub bitmap!\n"): "Could not allocate,
    } else {
    drbd_err(device, "BM resizing failed. "
    pub unchanged\n"): "Leaving size,
    }
    pub DS_ERROR: rv =,
    }
// racy, see comments above.
    pub size): drbd_set_my_capacity(device,,
    pub size: md->la_size_sect =,
    }
    if (rv <= DS_ERROR)
    pub err_out: goto,
    pub md->la_size_sect): la_size_changed = (prev.last_agreed_sect !=,
    md_moved = prev.md_offset    != md.md_offset
    pub md->md_size_sect: || prev.md_size_sect !=,
    if (la_size_changed || md_moved || rs) {
    pub prev_flags: u32,
// We do some synchronous IO below, which may take some time.
// Clear the timer, to avoid scary "timer expired!" messages,
// "Superblock" is written out at least twice below, anyways.
// We won't change the "al-extents" setting, we just may need
// to move the on-disk location of the activity log ringbuffer.
// Lock for transaction is good enough, it may well be "dirty"
// or even "starving".
    pub lc_try_lock_for_transaction(device->act_log)): wait_event(device->al_wait,,
// mark current on-disk bitmap and activity log as unreliable
    pub md->flags: prev_flags =,
    pub MDF_AL_DISABLED: md->flags |= MDF_FULL_SYNC |,
    pub buffer): drbd_md_write(device,,
    pub buffer): drbd_al_initialize(device,,
    drbd_info(device, "Writing the whole bitmap, %s\n",
    la_size_changed && md_moved ? "size changed and md moved" :
    pub moved"): la_size_changed ? "size changed" : "md,
// next line implicitly does drbd_suspend_io()+drbd_resume_io()
    drbd_bitmap_io(device, md_moved ? &drbd_bm_write_all : &drbd_bm_write,
    pub NULL): "size changed", BM_LOCKED_MASK,,
// on-disk bitmap and activity log is authoritative again
// (unless there was an IO error meanwhile...)
    pub prev_flags: md->flags =,
    pub buffer): drbd_md_write(device,,
    if (rs)
    drbd_info(device, "Changed AL layout to al-stripes = %d, al-stripe-size-kB = %d\n",
    pub 4): *mut *mut md->al_stripes, md->al_stripe_size_4k,
    }
    if (size > prev.last_agreed_sect)
    pub DS_GREW_FROM_ZERO: rv = prev.last_agreed_sect ? DS_GREW :,
    if (size < prev.last_agreed_sect)
    pub DS_SHRUNK: rv =,
    if (0) {
    err_out:
// restore previous offset and sizes
    pub prev.last_agreed_sect: md->la_size_sect =,
    pub prev.md_offset: md->md_offset =,
    pub prev.al_offset: md->al_offset =,
    pub prev.bm_offset: md->bm_offset =,
    pub prev.md_size_sect: md->md_size_sect =,
    pub prev.al_stripes: md->al_stripes =,
    pub prev.al_stripe_size_4k: md->al_stripe_size_4k =,
    pub prev.al_stripe_size_4k: *mut *mut md->al_size_4k = (u64)prev.al_stripes,
    }
    pub rv: return,
    }
    sector_t
    drbd_new_dev_size(struct drbd_device *device, struct drbd_backing_dev *bdev,
    sector_t u_size, int assume_peer_has_space)
    {
    pub /: *mut *mut sector_t p_size = device->p_size; / partner's disk size.,
    pub /: *mut *mut sector_t la_size_sect = bdev->md.la_size_sect; / last agreed size.,
    pub /: *mut *mut sector_t m_size; / my size,
    pub 0: sector_t size =,
    pub drbd_get_max_capacity(bdev): m_size =,
    if (device.state.conn < C_CONNECTED && assume_peer_has_space) {
    pub user!\n"): drbd_warn(device, "Resize while not connected was forced by the,
    pub m_size: p_size =,
    }
    if (p_size && m_size) {
    pub m_size): size = min_t(sector_t, p_size,,
    } else {
    if (la_size_sect) {
    pub la_size_sect: size =,
    if (m_size && m_size < size)
    pub m_size: size =,
    if (p_size && p_size < size)
    pub p_size: size =,
    } else {
    if (m_size)
    pub m_size: size =,
    if (p_size)
    pub p_size: size =,
    }
    }
    if (size == 0)
    pub diskless!\n"): drbd_err(device, "Both nodes,
    if (u_size) {
    if (u_size > size)
    drbd_err(device, "Requested disk size is too big (%lu > %lu)\n",
    pub long)size>>1): (unsigned long)u_size>>1, (unsigned,
    else
    pub u_size: size =,
    }
    pub size: return,
    }
//
// drbd_check_al_size() - Ensures that the AL is of the right size
// @device:	DRBD device.
//
// Returns -EBUSY if current al lru is still used, -ENOMEM when allocation
// failed, and 0 on success. You should call drbd_md_sync() after you called
// this function.
//
#[no_mangle]
unsafe extern "C" fn drbd_check_al_size(device: *mut drbd_device, dc: *mut disk_conf) -> c_int {
    static int drbd_check_al_size(struct drbd_device *device, struct disk_conf *dc)
    {
    pub t: *mut *mut lru_cache n,,
    pub e: *mut lc_element,
    pub in_use: c_uint,
    pub i: c_int,
    if (device.act_log &&
    device.act_log.nr_elements == dc.al_extents)
    pub 0: return,
    pub 0: in_use =,
    pub device->act_log: t =,
    n = lc_create("act_log", drbd_al_ext_cache, AL_UPDATES_PER_TRANSACTION,
    pub 0): dc->al_extents, sizeof(struct lc_element),,
    if (n == core::ptr::null_mut()) {
    pub lru!\n"): drbd_err(device, "Cannot allocate act_log,
    pub -ENOMEM: return,
    }
    if (t) {
    pub {: for (i = 0; i < t->nr_elements; i++),
    pub i): e = lc_element_by_index(t,,
    if (e.refcnt)
    drbd_err(device, "refcnt(%d)==%d\n",
    pub e->refcnt): e->lc_number,,
    pub e->refcnt: in_use +=,
    }
    }
    if (!in_use)
    pub n: device->act_log =,
    if (in_use) {
    pub use!\n"): drbd_err(device, "Activity log still in,
    pub -EBUSY: return,
    } else {
    }
    pub /: *mut *mut drbd_md_mark_dirty(device); / we changed device->act_log->nr_elemens,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn drbd_max_peer_bio_size(device: *mut drbd_device) -> c_uint {
    static unsigned int drbd_max_peer_bio_size(struct drbd_device *device)
    {
//
// We may ignore peer limits if the peer is modern enough.  From 8.3.8
// onwards the peer can use multiple BIOs for a single peer_request.
//
    if (device.state.conn < C_WF_REPORT_PARAMS)
    pub device->peer_max_bio_size: return,
    if (first_peer_device(device).connection.agreed_pro_version < 94)
    pub DRBD_MAX_SIZE_H80_PACKET): return min(device->peer_max_bio_size,,
//
// Correct old drbd (up to 8.3.7) if it believes it can do more than
// 32KiB.
//
    if (first_peer_device(device).connection.agreed_pro_version == 94)
    pub DRBD_MAX_SIZE_H80_PACKET: return,
//
// drbd 8.3.8 onwards, before 8.4.0
//
    if (first_peer_device(device).connection.agreed_pro_version < 100)
    pub DRBD_MAX_BIO_SIZE_P95: return,
    pub DRBD_MAX_BIO_SIZE: return,
    }
#[no_mangle]
unsafe extern "C" fn drbd_max_discard_sectors(connection: *mut drbd_connection) -> c_uint {
    static unsigned int drbd_max_discard_sectors(struct drbd_connection *connection)
    {
// when we introduced REQ_WRITE_SAME support, we also bumped
// our maximum supported batch bio size used for discards.
    if (connection.agreed_features & DRBD_FF_WSAME)
    pub DRBD_MAX_BBIO_SECTORS: return,
// before, with DRBD <= 8.4.6, we only allowed up to one AL_EXTENT_SIZE.
    pub 9: return AL_EXTENT_SIZE >>,
    }
    static bool drbd_discard_supported(struct drbd_connection *connection,
    struct drbd_backing_dev *bdev)
    {
    if (bdev && !bdev_max_discard_sectors(bdev.backing_bdev))
    pub false: return,
    if (connection.cstate >= C_CONNECTED &&
    !(connection.agreed_features & DRBD_FF_TRIM)) {
    drbd_info(connection,
    pub discards\n"): "peer DRBD too old, does not support TRIM: disabling,
    pub false: return,
    }
    pub true: return,
    }
// This is the workaround for "bio would need to, but cannot, be split"
#[no_mangle]
unsafe extern "C" fn drbd_backing_dev_max_segments(device: *mut drbd_device) -> c_uint {
    static unsigned int drbd_backing_dev_max_segments(struct drbd_device *device)
    {
    pub max_segments: c_uint,
    pub rcu_dereference(device->ldev->disk_conf)->max_bio_bvecs: max_segments =,
    if (!max_segments)
    pub BLK_MAX_SEGMENTS: return,
    pub max_segments: return,
    }
    void drbd_reconsider_queue_parameters(struct drbd_device *device,
    struct drbd_backing_dev *bdev, struct o_qlim *o)
    {
    struct drbd_connection *connection =
    pub device->rq_queue: *const *const request_queue  q =,
    pub 9: unsigned int now = queue_max_hw_sectors(q) <<,
    pub lim: queue_limits,
    pub NULL: *mut *mut request_queue b =,
    pub new: c_uint,
    if (bdev) {
    pub bdev->backing_bdev->bd_disk->queue: b =,
    device.local_max_bio_size =
    pub SECTOR_SHIFT: queue_max_hw_sectors(b) <<,
    }
//
// We may later detach and re-attach on a disconnected Primary.  Avoid
// decreasing the value in this case.
//
// We want to store what we know the peer DRBD can handle, not what the
// peer IO backend can handle.
//
    new = min3(DRBD_MAX_BIO_SIZE, device.local_max_bio_size,
    pub device->peer_max_bio_size)): max(drbd_max_peer_bio_size(device),,
    if (new != now) {
    if (device.state.role == R_PRIMARY && new < now)
    pub %u)\n",: drbd_err(device, "ASSERT FAILED new < now; (%u <,
    pub now): new,,
    pub new): drbd_info(device, "max BIO size = %u\n",,
    }
    pub queue_limits_start_update(q): lim =,
    if (bdev) {
    pub drbd_backing_dev_max_segments(device): lim.max_segments =,
    } else {
    pub BLK_MAX_SEGMENTS: lim.max_segments =,
    lim.features = BLK_FEAT_WRITE_CACHE | BLK_FEAT_FUA |
    pub BLK_FEAT_STABLE_WRITES: BLK_FEAT_ROTATIONAL |,
    }
    pub SECTOR_SHIFT: lim.max_hw_sectors = new >>,
    pub 1: lim.seg_boundary_mask = PAGE_SIZE -,
//
// We don't care for the granularity, really.
//
// Stacking limits below should fix it for the local device.  Whether or
// not it is a suitable granularity on the remote device is not our
// problem, really. If you care, you need to use devices with similar
// topology on all peers.
//
    if (drbd_discard_supported(connection, bdev)) {
    pub 512: lim.discard_granularity =,
    lim.max_hw_discard_sectors =
    } else {
    pub 0: lim.discard_granularity =,
    pub 0: lim.max_hw_discard_sectors =,
    }
    if (bdev) {
    pub 0): blk_stack_limits(&lim, &b->limits,,
//
// blk_set_stacking_limits() cleared the features, and
// blk_stack_limits() may or may not have inherited
// BLK_FEAT_STABLE_WRITES from the backing device.
//
// DRBD always requires stable writes because:
// 1. The same bio data is read for both local disk I/O and
// network transmission. If the page changes mid-flight,
// the local and remote copies could diverge.
// 2. When data integrity is enabled, DRBD calculates a
// checksum before sending the data. If the page changes
// between checksum calculation and transmission, the
// receiver will detect a checksum mismatch.
//
    pub BLK_FEAT_STABLE_WRITES: lim.features |=,
    }
//
// If we can handle "zeroes" efficiently on the protocol, we want to do
// that, even if our backend does not announce max_write_zeroes_sectors
// itself.
//
    if (connection.agreed_features & DRBD_FF_WZEROES)
    pub DRBD_MAX_BBIO_SECTORS: lim.max_write_zeroes_sectors =,
    else
    pub 0: lim.max_write_zeroes_sectors =,
    pub 0: lim.max_hw_wzeroes_unmap_sectors =,
    if ((lim.discard_granularity >> SECTOR_SHIFT) >
    lim.max_hw_discard_sectors) {
    pub 0: lim.discard_granularity =,
    pub 0: lim.max_hw_discard_sectors =,
    }
    if (queue_limits_commit_update(q, &lim))
    pub failed\n"): drbd_err(device, "setting new queue limits,
    }
// Starts the worker thread
#[no_mangle]
unsafe extern "C" fn conn_reconfig_start(connection: *mut drbd_connection) {
    static void conn_reconfig_start(struct drbd_connection *connection)
    {
    }
// if still unconfigured, stops worker again.
#[no_mangle]
unsafe extern "C" fn conn_reconfig_done(connection: *mut drbd_connection) {
    static void conn_reconfig_done(struct drbd_connection *connection)
    {
    pub stop_threads: bool,
    stop_threads = conn_all_vols_unconf(connection) &&
    pub C_STANDALONE: connection->cstate ==,
    if (stop_threads) {
// ack_receiver thread and ack_sender workqueue are implicitly
// stopped by receiver in conn_disconnect()
    }
    }
// Make sure IO is suspended before calling this function().
#[no_mangle]
unsafe extern "C" fn drbd_suspend_al(device: *mut drbd_device) {
    static void drbd_suspend_al(struct drbd_device *device)
    {
    pub 0: int s =,
    if (!lc_try_lock(device.act_log)) {
    pub drbd_suspend_al()\n"): drbd_warn(device, "Failed to lock al in,
    }
    if (device.state.conn < C_CONNECTED)
    pub &device->flags): s = !test_and_set_bit(AL_SUSPENDED,,
    if (s)
    pub updates\n"): drbd_info(device, "Suspended AL,
    }
#[no_mangle]
unsafe extern "C" fn should_set_defaults(info: *mut genl_info) -> bool {
    static bool should_set_defaults(struct genl_info *info)
    {
    pub genl_info_userhdr(info): *mut *mut drbd_genlmsghdr dh =,
    pub DRBD_GENL_F_SET_DEFAULTS): return 0 != (dh->flags &,
    }
#[no_mangle]
unsafe extern "C" fn drbd_al_extents_max(bdev: *mut drbd_backing_dev) -> c_uint {
    static unsigned int drbd_al_extents_max(struct drbd_backing_dev *bdev)
    {
// This is limited by 16 bit "slot" numbers,
// and by available on-disk context storage.
//
// Also (u16)~0 is special (denotes a "free" extent).
//
// One transaction occupies one 4kB on-disk block,
// we have n such blocks in the on disk ring buffer,
// the "current" transaction may fail (n-1),
// and there is 919 slot numbers context information per transaction.
//
// 72 transaction blocks amounts to more than 2**16 context slots,
// so cap there first.
//
    pub DRBD_AL_EXTENTS_MAX: unsigned int max_al_nr =,
    const unsigned int sufficient_on_disk =
    (max_al_nr + AL_CONTEXT_PER_TRANSACTION -1)
    pub bdev->md.al_size_4k: unsigned int al_size_4k =,
    if (al_size_4k > sufficient_on_disk)
    pub max_al_nr: return,
    pub AL_CONTEXT_PER_TRANSACTION: *mut *mut return (al_size_4k - 1),
    }
#[no_mangle]
unsafe extern "C" fn write_ordering_changed(a: *mut disk_conf, b: *mut disk_conf) -> bool {
    static bool write_ordering_changed(struct disk_conf *a, struct disk_conf *b)
    {
    return	a.disk_barrier != b.disk_barrier ||
    a.disk_flushes != b.disk_flushes ||
    pub b->disk_drain: a->disk_drain !=,
    }
    static void sanitize_disk_conf(struct drbd_device *device, struct disk_conf *disk_conf,
    struct drbd_backing_dev *nbc)
    {
    pub nbc->backing_bdev: *mut *mut block_device bdev =,
    if (disk_conf.al_extents < DRBD_AL_EXTENTS_MIN)
    pub DRBD_AL_EXTENTS_MIN: disk_conf->al_extents =,
    if (disk_conf.al_extents > drbd_al_extents_max(nbc))
    pub drbd_al_extents_max(nbc): disk_conf->al_extents =,
    if (!bdev_max_discard_sectors(bdev)) {
    if (disk_conf.rs_discard_granularity) {
    pub /: *mut *mut disk_conf->rs_discard_granularity = 0; / disable feature,
    pub disabled\n"): drbd_info(device, "rs_discard_granularity feature,
    }
    }
    if (disk_conf.rs_discard_granularity) {
    pub disk_conf->rs_discard_granularity: int orig_value =,
    pub 9: sector_t discard_size = bdev_max_discard_sectors(bdev) <<,
    pub bdev_discard_granularity(bdev): unsigned int discard_granularity =,
    pub remainder: c_int,
    if (discard_granularity > disk_conf.rs_discard_granularity)
    pub discard_granularity: disk_conf->rs_discard_granularity =,
    remainder = disk_conf.rs_discard_granularity %
    pub remainder: disk_conf->rs_discard_granularity +=,
    if (disk_conf.rs_discard_granularity > discard_size)
    pub discard_size: disk_conf->rs_discard_granularity =,
    if (disk_conf.rs_discard_granularity != orig_value)
    drbd_info(device, "rs_discard_granularity changed to %d\n",
    }
    }
#[no_mangle]
unsafe extern "C" fn disk_opts_check_al_size(device: *mut drbd_device, dc: *mut disk_conf) -> c_int {
    static int disk_opts_check_al_size(struct drbd_device *device, struct disk_conf *dc)
    {
    pub -EBUSY: int err =,
    if (device.act_log &&
    device.act_log.nr_elements == dc.al_extents)
    pub 0: return,
// If IO completion is currently blocked, we would likely wait
// "forever" for the activity log to become unused. So we don't.
    if (atomic_read(&device.ap_bio_cnt))
    pub out: goto,
    pub lc_try_lock(device->act_log)): wait_event(device->al_wait,,
    pub dc): err = drbd_check_al_size(device,,
    out:
    pub err: return,
    }
#[no_mangle]
pub unsafe extern "C" fn drbd_nl_chg_disk_opts_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    int drbd_nl_chg_disk_opts_doit(struct sk_buff *skb, struct genl_info *info)
    {
    pub info->user_ptr[0]: *mut *mut drbd_config_context adm_ctx =,
    pub retcode: enum drbd_ret_code,
    pub device: *mut drbd_device,
    pub old_disk_conf: *mut *mut disk_conf new_disk_conf,,
    pub NULL: *mut *mut *mut fifo_buffer old_plan = NULL, new_plan =,
    pub ntb: *mut nlattr,
    pub err: c_int,
    pub fifo_size: c_uint,
    if (!adm_ctx.reply_skb)
    pub 0: return,
    pub adm_ctx->reply_dh->ret_code: retcode =,
    if (retcode != NO_ERROR)
    pub finish: goto,
    pub adm_ctx->device: device =,
// we also need a disk
// to change the options on
    if (!get_ldev(device)) {
    pub ERR_NO_DISK: retcode =,
    pub out: goto,
    }
    pub disk_conf): new_disk_conf = kmalloc_obj(struct,
    if (!new_disk_conf) {
    pub ERR_NOMEM: retcode =,
    pub fail: goto,
    }
    pub device->ldev->disk_conf: old_disk_conf =,
// new_disk_conf = *old_disk_conf;
    if (should_set_defaults(info))
    pub info): err = disk_conf_from_attrs(new_disk_conf,,
    if (err && err != -ENOMSG) {
    pub ERR_MANDATORY_TAG: retcode =,
    pub from_attrs_err_to_txt(err)): drbd_msg_put_info(adm_ctx->reply_skb,,
    pub fail_unlock: goto,
    }
    pub info): err = disk_conf_ntb_from_attrs(&ntb,,
    if (!err) {
    if (has_invariant(ntb, DRBD_A_DISK_CONF_BACKING_DEV) ||
    has_invariant(ntb, DRBD_A_DISK_CONF_META_DEV) ||
    has_invariant(ntb, DRBD_A_DISK_CONF_META_DEV_IDX) ||
    has_invariant(ntb, DRBD_A_DISK_CONF_DISK_SIZE) ||
    has_invariant(ntb, DRBD_A_DISK_CONF_MAX_BIO_BVECS)) {
    pub ERR_MANDATORY_TAG: retcode =,
    drbd_msg_put_info(adm_ctx.reply_skb,
    pub setting"): "cannot change invariant,
    pub fail_unlock: goto,
    }
    }
    if (!expect(device, new_disk_conf.resync_rate >= 1))
    pub 1: new_disk_conf->resync_rate =,
    pub device->ldev): sanitize_disk_conf(device, new_disk_conf,,
    if (new_disk_conf.c_plan_ahead > DRBD_C_PLAN_AHEAD_MAX)
    pub DRBD_C_PLAN_AHEAD_MAX: new_disk_conf->c_plan_ahead =,
    pub HZ: *mut *mut *mut fifo_size = (new_disk_conf->c_plan_ahead  10  SLEEP_TIME) /,
    if (fifo_size != device.rs_plan_s.size) {
    pub fifo_alloc(fifo_size): new_plan =,
    if (!new_plan) {
    pub failed"): drbd_err(device, "kmalloc of fifo_buffer,
    pub ERR_NOMEM: retcode =,
    pub fail_unlock: goto,
    }
    }
    pub new_disk_conf): err = disk_opts_check_al_size(device,,
    if (err) {
// Could be just "busy". Ignore?
// Introduce dedicated error code?
    drbd_msg_put_info(adm_ctx.reply_skb,
    pub setting"): "Try again without changing current al-extents,
    pub ERR_NOMEM: retcode =,
    pub fail_unlock: goto,
    }
    pub new_disk_conf->resync_after): retcode = drbd_resync_after_valid(device,,
    if (retcode == NO_ERROR) {
    pub new_disk_conf): rcu_assign_pointer(device->ldev->disk_conf,,
    }
    if (retcode != NO_ERROR)
    pub fail_unlock: goto,
    if (new_plan) {
    pub device->rs_plan_s: old_plan =,
    pub new_plan): rcu_assign_pointer(device->rs_plan_s,,
    }
    if (new_disk_conf.al_updates)
    pub ~MDF_AL_DISABLED: device->ldev->md.flags &=,
    else
    pub MDF_AL_DISABLED: device->ldev->md.flags |=,
    if (new_disk_conf.md_flushes)
    pub &device->flags): clear_bit(MD_NO_FUA,,
    else
    pub &device->flags): set_bit(MD_NO_FUA,,
    if (write_ordering_changed(old_disk_conf, new_disk_conf))
    pub WO_BDEV_FLUSH): drbd_bump_write_ordering(device->resource, NULL,,
    if (old_disk_conf.discard_zeroes_if_aligned !=
    new_disk_conf.discard_zeroes_if_aligned)
    pub NULL): drbd_reconsider_queue_parameters(device, device->ldev,,
    if (device.state.conn >= C_CONNECTED) {
    pub peer_device: *mut drbd_peer_device,
    for_each_peer_device(peer_device, device)
    }
    pub HZ): mod_timer(&device->request_timer, jiffies +,
    pub success: goto,
    fail_unlock:
    fail:
    success:
    out:
    finish:
    pub retcode: adm_ctx->reply_dh->ret_code =,
    pub 0: return,
    }
    static struct file *open_backing_dev(struct drbd_device *device,
    const char *bdev_path, void *claim_ptr, bool do_bd_link)
    {
    pub file: *mut file,
    pub 0: int err =,
    file = bdev_file_open_by_path(bdev_path, BLK_OPEN_READ | BLK_OPEN_WRITE,
    pub NULL): claim_ptr,,
    if (IS_ERR(file)) {
    drbd_err(device, "open(\"%s\") failed with %ld\n",
    pub PTR_ERR(file)): bdev_path,,
    pub file: return,
    }
    if (!do_bd_link)
    pub file: return,
    pub device->vdisk): err = bd_link_disk_holder(file_bdev(file),,
    if (err) {
    drbd_err(device, "bd_link_disk_holder(\"%s\", ...) failed with %d\n",
    pub err): bdev_path,,
    pub ERR_PTR(err): file =,
    }
    pub file: return,
    }
    static int open_backing_devices(struct drbd_device *device,
    struct disk_conf *new_disk_conf,
    struct drbd_backing_dev *nbc)
    {
    pub file: *mut file,
    file = open_backing_dev(device, new_disk_conf.backing_dev, device,
    if (IS_ERR(file))
    pub ERR_OPEN_DISK: return,
    pub file_bdev(file): nbc->backing_bdev =,
    pub file: nbc->backing_bdev_file =,
//
// meta_dev_idx >= 0: external fixed size, possibly multiple
// drbd sharing one meta device.  TODO in that case, paranoia
// check that [md_bdev, meta_dev_idx] is not yet used by some
// other drbd minor!  (if you use drbd.conf + drbdadm, that
// should check it for you already; but if you don't, or
// someone fooled it, we need to double check here)
//
    file = open_backing_dev(device, new_disk_conf.meta_dev,
// claim ptr: device, if claimed exclusively; shared drbd_m_holder,
// if potentially shared with other drbd minors
    (new_disk_conf.meta_dev_idx < 0) ? (void*)device : (void*)drbd_m_holder,
// avoid double bd_claim_by_disk() for the same (source,target) tuple,
// as would happen with internal metadata.
    (new_disk_conf.meta_dev_idx != DRBD_MD_INDEX_FLEX_INT &&
    pub DRBD_MD_INDEX_INTERNAL)): new_disk_conf->meta_dev_idx !=,
    if (IS_ERR(file))
    pub ERR_OPEN_MD_DISK: return,
    pub file_bdev(file): nbc->md_bdev =,
    pub file: nbc->f_md_bdev =,
    pub NO_ERROR: return,
    }
    static void close_backing_dev(struct drbd_device *device,
    struct file *bdev_file, bool do_bd_unlink)
    {
    if (!bdev_file)
    if (do_bd_unlink)
    pub device->vdisk): bd_unlink_disk_holder(file_bdev(bdev_file),,
    }
#[no_mangle]
pub unsafe extern "C" fn drbd_backing_dev_free(device: *mut drbd_device, ldev: *mut drbd_backing_dev) {
    void drbd_backing_dev_free(struct drbd_device *device, struct drbd_backing_dev *ldev)
    {
    if (ldev == core::ptr::null_mut())
    close_backing_dev(device, ldev.f_md_bdev,
    pub ldev->backing_bdev): ldev->md_bdev !=,
    pub true): close_backing_dev(device, ldev->backing_bdev_file,,
    }
#[no_mangle]
pub unsafe extern "C" fn drbd_nl_attach_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    int drbd_nl_attach_doit(struct sk_buff *skb, struct genl_info *info)
    {
    pub info->user_ptr[0]: *mut *mut drbd_config_context adm_ctx =,
    pub device: *mut drbd_device,
    pub peer_device: *mut drbd_peer_device,
    pub connection: *mut drbd_connection,
    pub err: c_int,
    pub retcode: enum drbd_ret_code,
    pub dd: enum determine_dev_size,
    pub max_possible_sectors: sector_t,
    pub min_md_device_sectors: sector_t,
    pub /: *mut *mut *mut drbd_backing_dev nbc = NULL; / new_backing_conf,
    pub NULL: *mut *mut disk_conf new_disk_conf =,
    pub NULL: *mut *mut lru_cache resync_lru =,
    pub NULL: *mut *mut fifo_buffer new_plan =,
    pub os: union drbd_state ns,,
    pub rv: enum drbd_state_rv,
    pub nc: *mut net_conf,
    if (!adm_ctx.reply_skb)
    pub 0: return,
    pub adm_ctx->reply_dh->ret_code: retcode =,
    if (retcode != NO_ERROR)
    pub finish: goto,
    pub adm_ctx->device: device =,
    pub first_peer_device(device): peer_device =,
    pub peer_device->connection: connection =,
// if you want to reconfigure, please tear down first
    if (device.state.disk > D_DISKLESS) {
    pub ERR_DISK_CONFIGURED: retcode =,
    pub fail: goto,
    }
// It may just now have detached because of IO error.  Make sure
// drbd_ldev_destroy is done already, we may end up here very fast,
// e.g. if someone calls attach from the on-io-error handler,
// to realize a "hot spare" feature (not that I'd recommend that)
    pub &device->flags)): wait_event(device->misc_wait, !test_bit(GOING_DISKLESS,,
// make sure there is no leftover from previous force-detach attempts
    pub &device->flags): clear_bit(FORCE_DETACH,,
    pub &device->flags): clear_bit(WAS_IO_ERROR,,
    pub &device->flags): clear_bit(WAS_READ_ERROR,,
// and no leftover from previously aborted resync or verify, either
    pub 0: device->rs_total =,
    pub 0: device->rs_failed =,
    pub 0): atomic_set(&device->rs_pending_cnt,,
// allocation not in the IO path, drbdsetup context
    pub drbd_backing_dev): nbc = kzalloc_obj(struct,
    if (!nbc) {
    pub ERR_NOMEM: retcode =,
    pub fail: goto,
    }
    pub disk_conf): new_disk_conf = kzalloc_obj(struct,
    if (!new_disk_conf) {
    pub ERR_NOMEM: retcode =,
    pub fail: goto,
    }
    pub new_disk_conf: nbc->disk_conf =,
    pub info): err = disk_conf_from_attrs(new_disk_conf,,
    if (err) {
    pub ERR_MANDATORY_TAG: retcode =,
    pub from_attrs_err_to_txt(err)): drbd_msg_put_info(adm_ctx->reply_skb,,
    pub fail: goto,
    }
    if (new_disk_conf.c_plan_ahead > DRBD_C_PLAN_AHEAD_MAX)
    pub DRBD_C_PLAN_AHEAD_MAX: new_disk_conf->c_plan_ahead =,
    pub HZ): *mut *mut *mut new_plan = fifo_alloc((new_disk_conf->c_plan_ahead  10  SLEEP_TIME) /,
    if (!new_plan) {
    pub ERR_NOMEM: retcode =,
    pub fail: goto,
    }
    if (new_disk_conf.meta_dev_idx < DRBD_MD_INDEX_FLEX_INT) {
    pub ERR_MD_IDX_INVALID: retcode =,
    pub fail: goto,
    }
    pub rcu_dereference(connection->net_conf): nc =,
    if (nc) {
    if (new_disk_conf.fencing == FP_STONITH && nc.wire_protocol == DRBD_PROT_A) {
    pub ERR_STONITH_AND_PROT_A: retcode =,
    pub fail: goto,
    }
    }
    pub nbc): retcode = open_backing_devices(device, new_disk_conf,,
    if (retcode != NO_ERROR)
    pub fail: goto,
    if ((nbc.backing_bdev == nbc.md_bdev) !=
    (new_disk_conf.meta_dev_idx == DRBD_MD_INDEX_INTERNAL ||
    new_disk_conf.meta_dev_idx == DRBD_MD_INDEX_FLEX_INT)) {
    pub ERR_MD_IDX_INVALID: retcode =,
    pub fail: goto,
    }
    resync_lru = lc_create("resync", drbd_bm_ext_cache,
    1, 61, sizeof(struct bm_extent),
    pub lce)): offsetof(struct bm_extent,,
    if (!resync_lru) {
    pub ERR_NOMEM: retcode =,
    pub fail: goto,
    }
// Read our meta data super block early.
// This also sets other on-disk offsets.
    pub nbc): retcode = drbd_md_read(device,,
    if (retcode != NO_ERROR)
    pub fail: goto,
    pub nbc): sanitize_disk_conf(device, new_disk_conf,,
    if (drbd_get_max_capacity(nbc) < new_disk_conf.disk_size) {
    drbd_err(device, "max capacity %llu smaller than disk size %llu\n",
    (unsigned long long) drbd_get_max_capacity(nbc),
    pub new_disk_conf->disk_size): (unsigned long long),
    pub ERR_DISK_TOO_SMALL: retcode =,
    pub fail: goto,
    }
    if (new_disk_conf.meta_dev_idx < 0) {
    pub DRBD_MAX_SECTORS_FLEX: max_possible_sectors =,
// at least one MB, otherwise it does not make sense
    pub (2<<10): min_md_device_sectors =,
    } else {
    pub DRBD_MAX_SECTORS: max_possible_sectors =,
    pub 1): *mut *mut min_md_device_sectors = MD_128MB_SECT  (new_disk_conf->meta_dev_idx +,
    }
    if (drbd_get_capacity(nbc.md_bdev) < min_md_device_sectors) {
    pub ERR_MD_DISK_TOO_SMALL: retcode =,
    drbd_warn(device, "refusing attach: md-device too small, "
    "at least %llu sectors needed for this meta-disk type\n",
    pub min_md_device_sectors): (unsigned long long),
    pub fail: goto,
    }
// Make sure the new disk is big enough
// (we may currently be R_PRIMARY with no local disk...)
    if (drbd_get_max_capacity(nbc) < get_capacity(device.vdisk)) {
    pub ERR_DISK_TOO_SMALL: retcode =,
    pub fail: goto,
    }
    pub drbd_get_capacity(nbc->backing_bdev): nbc->known_size =,
    if (nbc.known_size > max_possible_sectors) {
    drbd_warn(device, "==> truncating very big lower level device "
    "to currently maximum possible %llu sectors <==\n",
    pub max_possible_sectors): (unsigned long long),
    if (new_disk_conf.meta_dev_idx >= 0)
    drbd_warn(device, "==>> using internal or flexible "
    pub <<==\n"): "meta data may help,
    }
// also wait for the last barrier ack.
// FIXME see also https://daiquiri.linbit/cgi-bin/bugzilla/show_bug.cgi?id=171
// We need a way to either ignore barrier acks for barriers sent before a device
// was attached, or a way to wait for all pending barrier acks to come in.
// As barriers are counted per resource,
// we'd need to suspend io on all devices of a resource.
//
    pub drbd_suspended(device)): wait_event(device->misc_wait, !atomic_read(&device->ap_pending_cnt) ||,
// and for any other previously queued work
    pub CS_VERBOSE): rv = _drbd_request_state(device, NS(disk, D_ATTACHING),,
    pub drbd_ret_code)rv: retcode = (enum,
    if (rv < SS_SUCCESS)
    pub fail: goto,
    if (!get_ldev_if_state(device, D_ATTACHING))
    pub force_diskless: goto,
    if (!device.bitmap) {
    if (drbd_bm_init(device)) {
    pub ERR_NOMEM: retcode =,
    pub force_diskless_dec: goto,
    }
    }
    if (device.state.pdsk != D_UP_TO_DATE && device.ed_uuid &&
    (device.state.role == R_PRIMARY || device.state.peer == R_PRIMARY) &&
    (device.ed_uuid & ~((u64)1)) != (nbc.md.uuid[UI_CURRENT] & ~((u64)1))) {
    drbd_err(device, "Can only attach to data with current UUID=%016llX\n",
    pub long)device->ed_uuid): (unsigned long,
    pub ERR_DATA_NOT_CURRENT: retcode =,
    pub force_diskless_dec: goto,
    }
// Since we are diskless, fix the activity log first...
    if (drbd_check_al_size(device, new_disk_conf)) {
    pub ERR_NOMEM: retcode =,
    pub force_diskless_dec: goto,
    }
// Prevent shrinking of consistent devices !
    {
    pub 0): unsigned long long nsz = drbd_new_dev_size(device, nbc, nbc->disk_conf->disk_size,,
    pub nbc->md.la_size_sect: unsigned long long eff =,
    if (drbd_md_test_flag(nbc, MDF_CONSISTENT) && nsz < eff) {
    if (nsz == nbc.disk_conf.disk_size) {
    pub eff): drbd_warn(device, "truncating a consistent device during attach (%llu < %llu)\n", nsz,,
    } else {
    pub eff): drbd_warn(device, "refusing to truncate a consistent device (%llu < %llu)\n", nsz,,
    drbd_msg_sprintf_info(adm_ctx.reply_skb,
    "To-be-attached device has last effective > current size, and is consistent\n"
    pub nsz): "(%llu > %llu sectors). Refusing to attach.", eff,,
    pub ERR_IMPLICIT_SHRINK: retcode =,
    pub force_diskless_dec: goto,
    }
    }
    }
    pub new_disk_conf->resync_after): retcode = drbd_resync_after_valid(device,,
    if (retcode != NO_ERROR) {
    pub force_diskless_dec: goto,
    }
// Reset the "barriers don't work" bits here, then force meta data to
// be written, to ensure we determine if barriers are supported.
    if (new_disk_conf.md_flushes)
    pub &device->flags): clear_bit(MD_NO_FUA,,
    else
    pub &device->flags): set_bit(MD_NO_FUA,,
// Point of no return reached.
// Devices and memory are no longer released by error cleanup below.
// now device takes over responsibility, and the state engine should
// clean it up somewhere.
    pub NULL): D_ASSERT(device, device->ldev ==,
    pub nbc: device->ldev =,
    pub resync_lru: device->resync =,
    pub new_plan: device->rs_plan_s =,
    pub NULL: nbc =,
    pub NULL: resync_lru =,
    pub NULL: new_disk_conf =,
    pub NULL: new_plan =,
    pub WO_BDEV_FLUSH): drbd_bump_write_ordering(device->resource, device->ldev,,
    if (drbd_md_test_flag(device.ldev, MDF_CRASHED_PRIMARY))
    pub &device->flags): set_bit(CRASHED_PRIMARY,,
    else
    pub &device->flags): clear_bit(CRASHED_PRIMARY,,
    if (drbd_md_test_flag(device.ldev, MDF_PRIMARY_IND) &&
    !(device.state.role == R_PRIMARY && device.resource.susp_nod))
    pub &device->flags): set_bit(CRASHED_PRIMARY,,
    pub 0: device->send_cnt =,
    pub 0: device->recv_cnt =,
    pub 0: device->read_cnt =,
    pub 0: device->writ_cnt =,
    pub NULL): drbd_reconsider_queue_parameters(device, device->ldev,,
// If I am currently not R_PRIMARY,
// but meta data primary indicator is set,
// I just now recover from a hard crash,
// and have been R_PRIMARY before that crash.
//
// Now, if I had no connection before that crash
// (have been degraded R_PRIMARY), chances are that
// I won't find my peer now either.
//
// In that case, and _only_ in that case,
// we use the degr-wfc-timeout instead of the default,
// so we can automatically recover from a crash of a
// degraded but active "cluster" after a certain timeout.
//
    pub &device->flags): clear_bit(USE_DEGR_WFC_T,,
    if (device.state.role != R_PRIMARY &&
    drbd_md_test_flag(device.ldev, MDF_PRIMARY_IND) &&
    !drbd_md_test_flag(device.ldev, MDF_CONNECTED_IND))
    pub &device->flags): set_bit(USE_DEGR_WFC_T,,
    pub NULL): dd = drbd_determine_dev_size(device, 0,,
    if (dd <= DS_ERROR) {
    pub ERR_NOMEM_BITMAP: retcode =,
    pub force_diskless_dec: goto,
    } else if (dd == DS_GREW)
    pub &device->flags): set_bit(RESYNC_AFTER_NEG,,
    if (drbd_md_test_flag(device.ldev, MDF_FULL_SYNC) ||
    (test_bit(CRASHED_PRIMARY, &device.flags) &&
    drbd_md_test_flag(device.ldev, MDF_AL_DISABLED))) {
    drbd_info(device, "Assuming that all blocks are out of sync "
    pub FullSync)\n"): "(aka,
    if (drbd_bitmap_io(device, &drbd_bmio_set_n_write,
    "set_n_write from attaching", BM_LOCKED_MASK,
    core::ptr::null_mut())) {
    pub ERR_IO_MD_DISK: retcode =,
    pub force_diskless_dec: goto,
    }
    } else {
    if (drbd_bitmap_io(device, &drbd_bm_read,
    "read from attaching", BM_LOCKED_MASK,
    core::ptr::null_mut())) {
    pub ERR_IO_MD_DISK: retcode =,
    pub force_diskless_dec: goto,
    }
    }
    if (_drbd_bm_total_weight(device) == drbd_bm_bits(device))
    pub /: *mut *mut drbd_suspend_al(device); / IO is still suspended here...,
    pub drbd_read_state(device): os =,
    pub os: ns =,
// If MDF_CONSISTENT is not set go into inconsistent state,
    otherwise investigate MDF_WasUpToDate...
    If MDF_WAS_UP_TO_DATE is not set go into D_OUTDATED disk state,
    otherwise into D_CONSISTENT state.
//
    if (drbd_md_test_flag(device.ldev, MDF_CONSISTENT)) {
    if (drbd_md_test_flag(device.ldev, MDF_WAS_UP_TO_DATE))
    pub D_CONSISTENT: ns.disk =,
    else
    pub D_OUTDATED: ns.disk =,
    } else {
    pub D_INCONSISTENT: ns.disk =,
    }
    if (drbd_md_test_flag(device.ldev, MDF_PEER_OUT_DATED))
    pub D_OUTDATED: ns.pdsk =,
    if (ns.disk == D_CONSISTENT &&
    (ns.pdsk == D_OUTDATED || rcu_dereference(device.ldev.disk_conf).fencing == FP_DONT_CARE))
    pub D_UP_TO_DATE: ns.disk =,
// All tests on MDF_PRIMARY_IND, MDF_CONNECTED_IND,
    MDF_CONSISTENT and MDF_WAS_UP_TO_DATE must happen before
    this point, because drbd_request_state() modifies these
    flags. */
    if (rcu_dereference(device.ldev.disk_conf).al_updates)
    pub ~MDF_AL_DISABLED: device->ldev->md.flags &=,
    else
    pub MDF_AL_DISABLED: device->ldev->md.flags |=,
// In case we are C_CONNECTED postpone any decision on the new disk
    state after the negotiation phase. */
    if (device.state.conn == C_CONNECTED) {
    pub ns.i: device->new_state_tmp.i =,
    pub os.i: ns.i =,
    pub D_NEGOTIATING: ns.disk =,
// We expect to receive up-to-date UUIDs soon.
    To avoid a race in receive_state, free p_uuid while
    holding req_lock. I.e. atomic with the state change */
    pub NULL: device->p_uuid =,
    }
    pub NULL): rv = _drbd_set_state(device, ns, CS_VERBOSE,,
    if (rv < SS_SUCCESS)
    pub force_diskless_dec: goto,
    pub HZ): mod_timer(&device->request_timer, jiffies +,
    if (device.state.role == R_PRIMARY)
    pub (u64)1: device->ldev->md.uuid[UI_CURRENT] |=,
    else
    pub ~(u64)1: device->ldev->md.uuid[UI_CURRENT] &=,
    pub KOBJ_CHANGE): kobject_uevent(&disk_to_dev(device->vdisk)->kobj,,
    pub retcode: adm_ctx->reply_dh->ret_code =,
    pub 0: return,
    force_diskless_dec:
    force_diskless:
    pub D_DISKLESS)): drbd_force_state(device, NS(disk,,
    fail:
    if (nbc) {
    close_backing_dev(device, nbc.f_md_bdev,
    pub nbc->backing_bdev): nbc->md_bdev !=,
    pub true): close_backing_dev(device, nbc->backing_bdev_file,,
    }
    finish:
    pub retcode: adm_ctx->reply_dh->ret_code =,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn adm_detach(device: *mut drbd_device, force: c_int) -> c_int {
    static int adm_detach(struct drbd_device *device, int force)
    {
    if (force) {
    pub &device->flags): set_bit(FORCE_DETACH,,
    pub D_FAILED)): drbd_force_state(device, NS(disk,,
    pub SS_SUCCESS: return,
    }
    pub drbd_request_detach_interruptible(device): return,
    }
// Detaching the disk is a process in multiple stages.  First we need to lock
// out application IO, in-flight IO, IO stuck in drbd_al_begin_io.
// Then we transition to D_DISKLESS, and wait for put_ldev() to return all
// internal references as well.
// Only then we have finally detached.
#[no_mangle]
pub unsafe extern "C" fn drbd_nl_detach_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    int drbd_nl_detach_doit(struct sk_buff *skb, struct genl_info *info)
    {
    pub info->user_ptr[0]: *mut *mut drbd_config_context adm_ctx =,
    pub retcode: enum drbd_ret_code,
    pub }: detach_parms parms = {,
    pub err: c_int,
    if (!adm_ctx.reply_skb)
    pub 0: return,
    pub adm_ctx->reply_dh->ret_code: retcode =,
    if (retcode != NO_ERROR)
    pub out: goto,
    if (info.attrs[DRBD_NLA_DETACH_PARMS]) {
    pub info): err = detach_parms_from_attrs(&parms,,
    if (err) {
    pub ERR_MANDATORY_TAG: retcode =,
    pub from_attrs_err_to_txt(err)): drbd_msg_put_info(adm_ctx->reply_skb,,
    pub out: goto,
    }
    }
    pub parms.force_detach): retcode = adm_detach(adm_ctx->device,,
    out:
    pub retcode: adm_ctx->reply_dh->ret_code =,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn conn_resync_running(connection: *mut drbd_connection) -> bool {
    static bool conn_resync_running(struct drbd_connection *connection)
    {
    pub peer_device: *mut drbd_peer_device,
    pub false: bool rv =,
    pub vnr: c_int,
    idr_for_each_entry(&connection.peer_devices, peer_device, vnr) {
    pub peer_device->device: *mut *mut drbd_device device =,
    if (device.state.conn == C_SYNC_SOURCE ||
    device.state.conn == C_SYNC_TARGET ||
    device.state.conn == C_PAUSED_SYNC_S ||
    device.state.conn == C_PAUSED_SYNC_T) {
    pub true: rv =,
    }
    }
    pub rv: return,
    }
#[no_mangle]
unsafe extern "C" fn conn_ov_running(connection: *mut drbd_connection) -> bool {
    static bool conn_ov_running(struct drbd_connection *connection)
    {
    pub peer_device: *mut drbd_peer_device,
    pub false: bool rv =,
    pub vnr: c_int,
    idr_for_each_entry(&connection.peer_devices, peer_device, vnr) {
    pub peer_device->device: *mut *mut drbd_device device =,
    if (device.state.conn == C_VERIFY_S ||
    device.state.conn == C_VERIFY_T) {
    pub true: rv =,
    }
    }
    pub rv: return,
    }
    static enum drbd_ret_code
    _check_net_options(struct drbd_connection *connection, struct net_conf *old_net_conf, struct net_conf *new_net_conf)
    {
    pub peer_device: *mut drbd_peer_device,
    pub i: c_int,
    if (old_net_conf && connection.cstate == C_WF_REPORT_PARAMS && connection.agreed_pro_version < 100) {
    if (new_net_conf.wire_protocol != old_net_conf.wire_protocol)
    pub ERR_NEED_APV_100: return,
    if (new_net_conf.two_primaries != old_net_conf.two_primaries)
    pub ERR_NEED_APV_100: return,
    if (strcmp(new_net_conf.integrity_alg, old_net_conf.integrity_alg))
    pub ERR_NEED_APV_100: return,
    }
    if (!new_net_conf.two_primaries &&
    conn_highest_role(connection) == R_PRIMARY &&
    conn_highest_peer(connection) == R_PRIMARY)
    pub ERR_NEED_ALLOW_TWO_PRI: return,
    if (new_net_conf.two_primaries &&
    (new_net_conf.wire_protocol != DRBD_PROT_C))
    pub ERR_NOT_PROTO_C: return,
    idr_for_each_entry(&connection.peer_devices, peer_device, i) {
    pub peer_device->device: *mut *mut drbd_device device =,
    if (get_ldev(device)) {
    pub rcu_dereference(device->ldev->disk_conf)->fencing: enum drbd_fencing_p fp =,
    if (new_net_conf.wire_protocol == DRBD_PROT_A && fp == FP_STONITH)
    pub ERR_STONITH_AND_PROT_A: return,
    }
    if (device.state.role == R_PRIMARY && new_net_conf.discard_my_data)
    pub ERR_DISCARD_IMPOSSIBLE: return,
    }
    if (new_net_conf.on_congestion != OC_BLOCK && new_net_conf.wire_protocol != DRBD_PROT_A)
    pub ERR_CONG_NOT_PROTO_A: return,
    pub NO_ERROR: return,
    }
    static enum drbd_ret_code
    check_net_options(struct drbd_connection *connection, struct net_conf *new_net_conf)
    {
    pub rv: enum drbd_ret_code,
    pub peer_device: *mut drbd_peer_device,
    pub i: c_int,
    pub new_net_conf): rv = _check_net_options(connection, rcu_dereference(connection->net_conf),,
// connection->peer_devices protected by genl_lock() here
    idr_for_each_entry(&connection.peer_devices, peer_device, i) {
    pub peer_device->device: *mut *mut drbd_device device =,
    if (!device.bitmap) {
    if (drbd_bm_init(device))
    pub ERR_NOMEM: return,
    }
    }
    pub rv: return,
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto {
    pub verify_tfm: *mut crypto_shash,
    pub csums_tfm: *mut crypto_shash,
    pub cram_hmac_tfm: *mut crypto_shash,
    pub integrity_tfm: *mut crypto_shash,
}

    static int
    alloc_shash(struct crypto_shash **tfm, char *tfm_name, int err_alg)
    {
    if (!tfm_name[0])
    return NO_ERROR;
// tfm = crypto_alloc_shash(tfm_name, 0, 0);
    if (IS_ERR(*tfm)) {
// tfm = NULL;
    return err_alg;
    }
    return NO_ERROR;
    }
    static enum drbd_ret_code
    alloc_crypto(struct crypto *crypto, struct net_conf *new_net_conf)
    {
    char hmac_name[CRYPTO_MAX_ALG_NAME];
    enum drbd_ret_code rv;
    rv = alloc_shash(&crypto.csums_tfm, new_net_conf.csums_alg,
    ERR_CSUMS_ALG);
    if (rv != NO_ERROR)
    return rv;
    rv = alloc_shash(&crypto.verify_tfm, new_net_conf.verify_alg,
    ERR_VERIFY_ALG);
    if (rv != NO_ERROR)
    return rv;
    rv = alloc_shash(&crypto.integrity_tfm, new_net_conf.integrity_alg,
    ERR_INTEGRITY_ALG);
    if (rv != NO_ERROR)
    return rv;
    if (new_net_conf.cram_hmac_alg[0] != 0) {
    snprintf(hmac_name, CRYPTO_MAX_ALG_NAME, "hmac(%s)",
    new_net_conf.cram_hmac_alg);
    rv = alloc_shash(&crypto.cram_hmac_tfm, hmac_name,
    ERR_AUTH_ALG);
    }
    return rv;
    }
#[no_mangle]
unsafe extern "C" fn free_crypto(crypto: *mut crypto) {
    static void free_crypto(struct crypto *crypto)
    {
    crypto_free_shash(crypto.cram_hmac_tfm);
    crypto_free_shash(crypto.integrity_tfm);
    crypto_free_shash(crypto.csums_tfm);
    crypto_free_shash(crypto.verify_tfm);
    }
#[no_mangle]
pub unsafe extern "C" fn drbd_nl_chg_net_opts_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    int drbd_nl_chg_net_opts_doit(struct sk_buff *skb, struct genl_info *info)
    {
    struct drbd_config_context *adm_ctx = info.user_ptr[0];
    enum drbd_ret_code retcode;
    struct drbd_connection *connection;
    struct net_conf *old_net_conf, *new_net_conf = core::ptr::null_mut();
    struct nlattr **ntb;
    int err;
    int ovr; /* online verify running */
    int rsr; /* re-sync running */
    let mut crypto: crypto = { };
    if (!adm_ctx.reply_skb)
    return 0;
    retcode = adm_ctx.reply_dh.ret_code;
    if (retcode != NO_ERROR)
    goto finish;
    connection = adm_ctx.connection;
    mutex_lock(&adm_ctx.resource.adm_mutex);
    new_net_conf = kzalloc_obj(struct net_conf);
    if (!new_net_conf) {
    retcode = ERR_NOMEM;
    goto out;
    }
    conn_reconfig_start(connection);
    mutex_lock(&connection.data.mutex);
    mutex_lock(&connection.resource.conf_update);
    old_net_conf = connection.net_conf;
    if (!old_net_conf) {
    drbd_msg_put_info(adm_ctx.reply_skb, "net conf missing, try connect");
    retcode = ERR_INVALID_REQUEST;
    goto fail;
    }
// new_net_conf = *old_net_conf;
    if (should_set_defaults(info))
    set_net_conf_defaults(new_net_conf);
    err = net_conf_from_attrs(new_net_conf, info);
    if (err && err != -ENOMSG) {
    retcode = ERR_MANDATORY_TAG;
    drbd_msg_put_info(adm_ctx.reply_skb, from_attrs_err_to_txt(err));
    goto fail;
    }
    err = net_conf_ntb_from_attrs(&ntb, info);
    if (!err) {
    if (has_invariant(ntb, DRBD_A_NET_CONF_DISCARD_MY_DATA) ||
    has_invariant(ntb, DRBD_A_NET_CONF_TENTATIVE)) {
    retcode = ERR_MANDATORY_TAG;
    drbd_msg_put_info(adm_ctx.reply_skb,
    "cannot change invariant setting");
    kfree(ntb);
    goto fail;
    }
    kfree(ntb);
    }
    retcode = check_net_options(connection, new_net_conf);
    if (retcode != NO_ERROR)
    goto fail;
// re-sync running
    rsr = conn_resync_running(connection);
    if (rsr && strcmp(new_net_conf.csums_alg, old_net_conf.csums_alg)) {
    retcode = ERR_CSUMS_RESYNC_RUNNING;
    goto fail;
    }
// online verify running
    ovr = conn_ov_running(connection);
    if (ovr && strcmp(new_net_conf.verify_alg, old_net_conf.verify_alg)) {
    retcode = ERR_VERIFY_RUNNING;
    goto fail;
    }
    retcode = alloc_crypto(&crypto, new_net_conf);
    if (retcode != NO_ERROR)
    goto fail;
    rcu_assign_pointer(connection.net_conf, new_net_conf);
    if (!rsr) {
    crypto_free_shash(connection.csums_tfm);
    connection.csums_tfm = crypto.csums_tfm;
    crypto.csums_tfm = core::ptr::null_mut();
    }
    if (!ovr) {
    crypto_free_shash(connection.verify_tfm);
    connection.verify_tfm = crypto.verify_tfm;
    crypto.verify_tfm = core::ptr::null_mut();
    }
    crypto_free_shash(connection.integrity_tfm);
    connection.integrity_tfm = crypto.integrity_tfm;
    if (connection.cstate >= C_WF_REPORT_PARAMS && connection.agreed_pro_version >= 100)
// Do this without trying to take connection->data.mutex again.
    __drbd_send_protocol(connection, P_PROTOCOL_UPDATE);
    crypto_free_shash(connection.cram_hmac_tfm);
    connection.cram_hmac_tfm = crypto.cram_hmac_tfm;
    mutex_unlock(&connection.resource.conf_update);
    mutex_unlock(&connection.data.mutex);
    kvfree_rcu_mightsleep(old_net_conf);
    if (connection.cstate >= C_WF_REPORT_PARAMS) {
    struct drbd_peer_device *peer_device;
    int vnr;
    idr_for_each_entry(&connection.peer_devices, peer_device, vnr)
    drbd_send_sync_param(peer_device);
    }
    goto done;
    fail:
    mutex_unlock(&connection.resource.conf_update);
    mutex_unlock(&connection.data.mutex);
    free_crypto(&crypto);
    kfree(new_net_conf);
    done:
    conn_reconfig_done(connection);
    out:
    mutex_unlock(&adm_ctx.resource.adm_mutex);
    finish:
    adm_ctx.reply_dh.ret_code = retcode;
    return 0;
    }
    static void connection_to_info(struct connection_info *info,
    struct drbd_connection *connection)
    {
    info.conn_connection_state = connection.cstate;
    info.conn_role = conn_highest_peer(connection);
    }
    static void peer_device_to_info(struct peer_device_info *info,
    struct drbd_peer_device *peer_device)
    {
    struct drbd_device *device = peer_device.device;
    info.peer_repl_state =
    max_t(enum drbd_conns, C_WF_REPORT_PARAMS, device.state.conn);
    info.peer_disk_state = device.state.pdsk;
    info.peer_resync_susp_user = device.state.user_isp;
    info.peer_resync_susp_peer = device.state.peer_isp;
    info.peer_resync_susp_dependency = device.state.aftr_isp;
    }
#[no_mangle]
pub unsafe extern "C" fn drbd_nl_connect_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    int drbd_nl_connect_doit(struct sk_buff *skb, struct genl_info *info)
    {
    struct connection_info connection_info;
    enum drbd_notification_type flags;
    let mut peer_devices: c_uint = 0;
    struct drbd_config_context *adm_ctx = info.user_ptr[0];
    struct drbd_peer_device *peer_device;
    struct net_conf *old_net_conf, *new_net_conf = core::ptr::null_mut();
    let mut crypto: crypto = { };
    struct drbd_resource *resource;
    struct drbd_connection *connection;
    enum drbd_ret_code retcode;
    enum drbd_state_rv rv;
    int i;
    int err;
    if (!adm_ctx.reply_skb)
    return 0;
    retcode = adm_ctx.reply_dh.ret_code;
    if (retcode != NO_ERROR)
    goto out;
    if (!(adm_ctx.my_addr && adm_ctx.peer_addr)) {
    drbd_msg_put_info(adm_ctx.reply_skb, "connection endpoint(s) missing");
    retcode = ERR_INVALID_REQUEST;
    goto out;
    }
// No need for _rcu here. All reconfiguration is
// strictly serialized on genl_lock(). We are protected against
// concurrent reconfiguration/addition/deletion
    for_each_resource(resource, &drbd_resources) {
    for_each_connection(connection, resource) {
    if (nla_len(adm_ctx.my_addr) == connection.my_addr_len &&
    !memcmp(nla_data(adm_ctx.my_addr), &connection.my_addr,
    connection.my_addr_len)) {
    retcode = ERR_LOCAL_ADDR;
    goto out;
    }
    if (nla_len(adm_ctx.peer_addr) == connection.peer_addr_len &&
    !memcmp(nla_data(adm_ctx.peer_addr), &connection.peer_addr,
    connection.peer_addr_len)) {
    retcode = ERR_PEER_ADDR;
    goto out;
    }
    }
    }
    mutex_lock(&adm_ctx.resource.adm_mutex);
    connection = first_connection(adm_ctx.resource);
    conn_reconfig_start(connection);
    if (connection.cstate > C_STANDALONE) {
    retcode = ERR_NET_CONFIGURED;
    goto fail;
    }
// allocation not in the IO path, drbdsetup / netlink process context
    new_net_conf = kzalloc_obj(*new_net_conf);
    if (!new_net_conf) {
    retcode = ERR_NOMEM;
    goto fail;
    }
    set_net_conf_defaults(new_net_conf);
    err = net_conf_from_attrs(new_net_conf, info);
    if (err && err != -ENOMSG) {
    retcode = ERR_MANDATORY_TAG;
    drbd_msg_put_info(adm_ctx.reply_skb, from_attrs_err_to_txt(err));
    goto fail;
    }
    retcode = check_net_options(connection, new_net_conf);
    if (retcode != NO_ERROR)
    goto fail;
    retcode = alloc_crypto(&crypto, new_net_conf);
    if (retcode != NO_ERROR)
    goto fail;
    ((char *)new_net_conf.shared_secret)[SHARED_SECRET_MAX-1] = 0;
    drbd_flush_workqueue(&connection.sender_work);
    mutex_lock(&adm_ctx.resource.conf_update);
    old_net_conf = connection.net_conf;
    if (old_net_conf) {
    retcode = ERR_NET_CONFIGURED;
    mutex_unlock(&adm_ctx.resource.conf_update);
    goto fail;
    }
    rcu_assign_pointer(connection.net_conf, new_net_conf);
    conn_free_crypto(connection);
    connection.cram_hmac_tfm = crypto.cram_hmac_tfm;
    connection.integrity_tfm = crypto.integrity_tfm;
    connection.csums_tfm = crypto.csums_tfm;
    connection.verify_tfm = crypto.verify_tfm;
    connection.my_addr_len = nla_len(adm_ctx.my_addr);
    memcpy(&connection.my_addr, nla_data(adm_ctx.my_addr), connection.my_addr_len);
    connection.peer_addr_len = nla_len(adm_ctx.peer_addr);
    memcpy(&connection.peer_addr, nla_data(adm_ctx.peer_addr), connection.peer_addr_len);
    idr_for_each_entry(&connection.peer_devices, peer_device, i) {
    peer_devices++;
    }
    connection_to_info(&connection_info, connection);
    flags = (peer_devices--) ? NOTIFY_CONTINUES : 0;
    mutex_lock(&notification_mutex);
    notify_connection_state(core::ptr::null_mut(), 0, connection, &connection_info, NOTIFY_CREATE | flags);
    idr_for_each_entry(&connection.peer_devices, peer_device, i) {
    struct peer_device_info peer_device_info;
    peer_device_to_info(&peer_device_info, peer_device);
    flags = (peer_devices--) ? NOTIFY_CONTINUES : 0;
    notify_peer_device_state(core::ptr::null_mut(), 0, peer_device, &peer_device_info, NOTIFY_CREATE | flags);
    }
    mutex_unlock(&notification_mutex);
    mutex_unlock(&adm_ctx.resource.conf_update);
    rcu_read_lock();
    idr_for_each_entry(&connection.peer_devices, peer_device, i) {
    struct drbd_device *device = peer_device.device;
    device.send_cnt = 0;
    device.recv_cnt = 0;
    }
    rcu_read_unlock();
    rv = conn_request_state(connection, NS(conn, C_UNCONNECTED), CS_VERBOSE);
    conn_reconfig_done(connection);
    mutex_unlock(&adm_ctx.resource.adm_mutex);
    adm_ctx.reply_dh.ret_code = rv;
    return 0;
    fail:
    free_crypto(&crypto);
    kfree(new_net_conf);
    conn_reconfig_done(connection);
    mutex_unlock(&adm_ctx.resource.adm_mutex);
    out:
    adm_ctx.reply_dh.ret_code = retcode;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn conn_try_disconnect(connection: *mut drbd_connection, force: bool) -> enum drbd_state_rv {
    static enum drbd_state_rv conn_try_disconnect(struct drbd_connection *connection, bool force)
    {
    enum drbd_conns cstate;
    enum drbd_state_rv rv;
    repeat:
    rv = conn_request_state(connection, NS(conn, C_DISCONNECTING),
    force ? CS_HARD : 0);
    switch (rv) {
    case SS_NOTHING_TO_DO:
    break;
    case SS_ALREADY_STANDALONE:
    return SS_SUCCESS;
    case SS_PRIMARY_NOP:
// Our state checking code wants to see the peer outdated.
    rv = conn_request_state(connection, NS2(conn, C_DISCONNECTING, pdsk, D_OUTDATED), 0);
    if (rv == SS_OUTDATE_WO_CONN) /* lost connection before graceful disconnect succeeded */
    rv = conn_request_state(connection, NS(conn, C_DISCONNECTING), CS_VERBOSE);
    break;
    case SS_CW_FAILED_BY_PEER:
    spin_lock_irq(&connection.resource.req_lock);
    cstate = connection.cstate;
    spin_unlock_irq(&connection.resource.req_lock);
    if (cstate <= C_WF_CONNECTION)
    goto repeat;
// The peer probably wants to see us outdated.
    rv = conn_request_state(connection, NS2(conn, C_DISCONNECTING,
    disk, D_OUTDATED), 0);
    if (rv == SS_IS_DISKLESS || rv == SS_LOWER_THAN_OUTDATED) {
    rv = conn_request_state(connection, NS(conn, C_DISCONNECTING),
    CS_HARD);
    }
    break;
    default:;
// no special handling necessary
    }
    if (rv >= SS_SUCCESS) {
    enum drbd_state_rv rv2;
// No one else can reconfigure the network while I am here.
// The state handling only uses drbd_thread_stop_nowait(),
// we want to really wait here until the receiver is no more.
//
    drbd_thread_stop(&connection.receiver);
// Race breaker.  This additional state change request may be
// necessary, if this was a forced disconnect during a receiver
// restart.  We may have "killed" the receiver thread just
// after drbd_receiver() returned.  Typically, we should be
// C_STANDALONE already, now, and this becomes a no-op.
//
    rv2 = conn_request_state(connection, NS(conn, C_STANDALONE),
    CS_VERBOSE | CS_HARD);
    if (rv2 < SS_SUCCESS)
    drbd_err(connection,
    "unexpected rv2=%d in conn_try_disconnect()\n",
    rv2);
// Unlike in DRBD 9, the state engine has generated
// NOTIFY_DESTROY events before clearing connection->net_conf.
    }
    return rv;
    }
#[no_mangle]
pub unsafe extern "C" fn drbd_nl_disconnect_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    int drbd_nl_disconnect_doit(struct sk_buff *skb, struct genl_info *info)
    {
    struct drbd_config_context *adm_ctx = info.user_ptr[0];
    struct disconnect_parms parms;
    struct drbd_connection *connection;
    enum drbd_state_rv rv;
    enum drbd_ret_code retcode;
    int err;
    if (!adm_ctx.reply_skb)
    return 0;
    retcode = adm_ctx.reply_dh.ret_code;
    if (retcode != NO_ERROR)
    goto fail;
    connection = adm_ctx.connection;
    memset(&parms, 0, sizeof(parms));
    if (info.attrs[DRBD_NLA_DISCONNECT_PARMS]) {
    err = disconnect_parms_from_attrs(&parms, info);
    if (err) {
    retcode = ERR_MANDATORY_TAG;
    drbd_msg_put_info(adm_ctx.reply_skb, from_attrs_err_to_txt(err));
    goto fail;
    }
    }
    mutex_lock(&adm_ctx.resource.adm_mutex);
    rv = conn_try_disconnect(connection, parms.force_disconnect);
    mutex_unlock(&adm_ctx.resource.adm_mutex);
    if (rv < SS_SUCCESS) {
    adm_ctx.reply_dh.ret_code = rv;
    return 0;
    }
    retcode = NO_ERROR;
    fail:
    adm_ctx.reply_dh.ret_code = retcode;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn resync_after_online_grow(device: *mut drbd_device) {
    void resync_after_online_grow(struct drbd_device *device)
    {
    int iass; /* I am sync source */
    drbd_info(device, "Resync of new storage after online grow\n");
    if (device.state.role != device.state.peer)
    iass = (device.state.role == R_PRIMARY);
    else
    iass = test_bit(RESOLVE_CONFLICTS, &first_peer_device(device).connection.flags);
    if (iass)
    drbd_start_resync(device, C_SYNC_SOURCE);
    else
    _drbd_request_state(device, NS(conn, C_WF_SYNC_UUID), CS_VERBOSE + CS_SERIALIZE);
    }
#[no_mangle]
pub unsafe extern "C" fn drbd_nl_resize_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    int drbd_nl_resize_doit(struct sk_buff *skb, struct genl_info *info)
    {
    struct drbd_config_context *adm_ctx = info.user_ptr[0];
    struct disk_conf *old_disk_conf, *new_disk_conf = core::ptr::null_mut();
    struct resize_parms rs;
    struct drbd_device *device;
    enum drbd_ret_code retcode;
    enum determine_dev_size dd;
    let mut change_al_layout: bool = false;
    enum dds_flags ddsf;
    sector_t u_size;
    int err;
    if (!adm_ctx.reply_skb)
    return 0;
    retcode = adm_ctx.reply_dh.ret_code;
    if (retcode != NO_ERROR)
    goto finish;
    mutex_lock(&adm_ctx.resource.adm_mutex);
    device = adm_ctx.device;
    if (!get_ldev(device)) {
    retcode = ERR_NO_DISK;
    goto fail;
    }
    memset(&rs, 0, sizeof(struct resize_parms));
    rs.al_stripes = device.ldev.md.al_stripes;
    rs.al_stripe_size = device.ldev.md.al_stripe_size_4k * 4;
    if (info.attrs[DRBD_NLA_RESIZE_PARMS]) {
    err = resize_parms_from_attrs(&rs, info);
    if (err) {
    retcode = ERR_MANDATORY_TAG;
    drbd_msg_put_info(adm_ctx.reply_skb, from_attrs_err_to_txt(err));
    goto fail_ldev;
    }
    }
    if (device.state.conn > C_CONNECTED) {
    retcode = ERR_RESIZE_RESYNC;
    goto fail_ldev;
    }
    if (device.state.role == R_SECONDARY &&
    device.state.peer == R_SECONDARY) {
    retcode = ERR_NO_PRIMARY;
    goto fail_ldev;
    }
    if (rs.no_resync && first_peer_device(device).connection.agreed_pro_version < 93) {
    retcode = ERR_NEED_APV_93;
    goto fail_ldev;
    }
    rcu_read_lock();
    u_size = rcu_dereference(device.ldev.disk_conf).disk_size;
    rcu_read_unlock();
    if (u_size != (sector_t)rs.resize_size) {
    new_disk_conf = kmalloc_obj(struct disk_conf);
    if (!new_disk_conf) {
    retcode = ERR_NOMEM;
    goto fail_ldev;
    }
    }
    if (device.ldev.md.al_stripes != rs.al_stripes ||
    device.ldev.md.al_stripe_size_4k != rs.al_stripe_size / 4) {
    let mut al_size_k: u32 = rs.al_stripes * rs.al_stripe_size;
    if (al_size_k > (16 * 1024 * 1024)) {
    retcode = ERR_MD_LAYOUT_TOO_BIG;
    goto fail_ldev;
    }
    if (al_size_k < MD_32kB_SECT/2) {
    retcode = ERR_MD_LAYOUT_TOO_SMALL;
    goto fail_ldev;
    }
    if (device.state.conn != C_CONNECTED && !rs.resize_force) {
    retcode = ERR_MD_LAYOUT_CONNECTED;
    goto fail_ldev;
    }
    change_al_layout = true;
    }
    if (device.ldev.known_size != drbd_get_capacity(device.ldev.backing_bdev))
    device.ldev.known_size = drbd_get_capacity(device.ldev.backing_bdev);
    if (new_disk_conf) {
    mutex_lock(&device.resource.conf_update);
    old_disk_conf = device.ldev.disk_conf;
// new_disk_conf = *old_disk_conf;
    new_disk_conf.disk_size = (sector_t)rs.resize_size;
    rcu_assign_pointer(device.ldev.disk_conf, new_disk_conf);
    mutex_unlock(&device.resource.conf_update);
    kvfree_rcu_mightsleep(old_disk_conf);
    new_disk_conf = core::ptr::null_mut();
    }
    ddsf = (rs.resize_force ? DDSF_FORCED : 0) | (rs.no_resync ? DDSF_NO_RESYNC : 0);
    dd = drbd_determine_dev_size(device, ddsf, change_al_layout ? &rs : core::ptr::null_mut());
    drbd_md_sync(device);
    put_ldev(device);
    if (dd == DS_ERROR) {
    retcode = ERR_NOMEM_BITMAP;
    goto fail;
    } else if (dd == DS_ERROR_SPACE_MD) {
    retcode = ERR_MD_LAYOUT_NO_FIT;
    goto fail;
    } else if (dd == DS_ERROR_SHRINK) {
    retcode = ERR_IMPLICIT_SHRINK;
    goto fail;
    }
    if (device.state.conn == C_CONNECTED) {
    if (dd == DS_GREW)
    set_bit(RESIZE_PENDING, &device.flags);
    drbd_send_uuids(first_peer_device(device));
    drbd_send_sizes(first_peer_device(device), 1, ddsf);
    }
    fail:
    mutex_unlock(&adm_ctx.resource.adm_mutex);
    finish:
    adm_ctx.reply_dh.ret_code = retcode;
    return 0;
    fail_ldev:
    put_ldev(device);
    kfree(new_disk_conf);
    goto fail;
    }
#[no_mangle]
pub unsafe extern "C" fn drbd_nl_resource_opts_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    int drbd_nl_resource_opts_doit(struct sk_buff *skb, struct genl_info *info)
    {
    struct drbd_config_context *adm_ctx = info.user_ptr[0];
    enum drbd_ret_code retcode;
    struct res_opts res_opts;
    int err;
    if (!adm_ctx.reply_skb)
    return 0;
    retcode = adm_ctx.reply_dh.ret_code;
    if (retcode != NO_ERROR)
    goto fail;
    res_opts = adm_ctx.resource.res_opts;
    if (should_set_defaults(info))
    set_res_opts_defaults(&res_opts);
    err = res_opts_from_attrs(&res_opts, info);
    if (err && err != -ENOMSG) {
    retcode = ERR_MANDATORY_TAG;
    drbd_msg_put_info(adm_ctx.reply_skb, from_attrs_err_to_txt(err));
    goto fail;
    }
    mutex_lock(&adm_ctx.resource.adm_mutex);
    err = set_resource_options(adm_ctx.resource, &res_opts);
    if (err) {
    retcode = ERR_INVALID_REQUEST;
    if (err == -ENOMEM)
    retcode = ERR_NOMEM;
    }
    mutex_unlock(&adm_ctx.resource.adm_mutex);
    fail:
    adm_ctx.reply_dh.ret_code = retcode;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn drbd_nl_invalidate_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    int drbd_nl_invalidate_doit(struct sk_buff *skb, struct genl_info *info)
    {
    struct drbd_config_context *adm_ctx = info.user_ptr[0];
    struct drbd_device *device;
    int retcode; /* enum drbd_ret_code rsp. enum drbd_state_rv */
    if (!adm_ctx.reply_skb)
    return 0;
    retcode = adm_ctx.reply_dh.ret_code;
    if (retcode != NO_ERROR)
    goto out;
    device = adm_ctx.device;
    if (!get_ldev(device)) {
    retcode = ERR_NO_DISK;
    goto out;
    }
    mutex_lock(&adm_ctx.resource.adm_mutex);
// If there is still bitmap IO pending, probably because of a previous
// resync just being finished, wait for it before requesting a new resync.
// Also wait for it's after_state_ch().
    drbd_suspend_io(device);
    wait_event(device.misc_wait, !test_bit(BITMAP_IO, &device.flags));
    drbd_flush_workqueue(&first_peer_device(device).connection.sender_work);
// If we happen to be C_STANDALONE R_SECONDARY, just change to
// D_INCONSISTENT, and set all bits in the bitmap.  Otherwise,
// try to start a resync handshake as sync target for full sync.
//
    if (device.state.conn == C_STANDALONE && device.state.role == R_SECONDARY) {
    retcode = drbd_request_state(device, NS(disk, D_INCONSISTENT));
    if (retcode >= SS_SUCCESS) {
    if (drbd_bitmap_io(device, &drbd_bmio_set_n_write,
    "set_n_write from invalidate", BM_LOCKED_MASK, core::ptr::null_mut()))
    retcode = ERR_IO_MD_DISK;
    }
    } else
    retcode = drbd_request_state(device, NS(conn, C_STARTING_SYNC_T));
    drbd_resume_io(device);
    mutex_unlock(&adm_ctx.resource.adm_mutex);
    put_ldev(device);
    out:
    adm_ctx.reply_dh.ret_code = retcode;
    return 0;
    }
    static int drbd_adm_simple_request_state(struct sk_buff *skb, struct genl_info *info,
    union drbd_state mask, union drbd_state val)
    {
    struct drbd_config_context *adm_ctx = info.user_ptr[0];
    enum drbd_ret_code retcode;
    if (!adm_ctx.reply_skb)
    return 0;
    retcode = adm_ctx.reply_dh.ret_code;
    if (retcode != NO_ERROR)
    goto out;
    mutex_lock(&adm_ctx.resource.adm_mutex);
    retcode = drbd_request_state(adm_ctx.device, mask, val);
    mutex_unlock(&adm_ctx.resource.adm_mutex);
    out:
    adm_ctx.reply_dh.ret_code = retcode;
    return 0;
    }
    static int drbd_bmio_set_susp_al(struct drbd_device *device,
    struct drbd_peer_device *peer_device) __must_hold(local)
    {
    int rv;
    rv = drbd_bmio_set_n_write(device, peer_device);
    drbd_suspend_al(device);
    return rv;
    }
#[no_mangle]
pub unsafe extern "C" fn drbd_nl_inval_peer_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    int drbd_nl_inval_peer_doit(struct sk_buff *skb, struct genl_info *info)
    {
    struct drbd_config_context *adm_ctx = info.user_ptr[0];
    int retcode; /* drbd_ret_code, drbd_state_rv */
    struct drbd_device *device;
    if (!adm_ctx.reply_skb)
    return 0;
    retcode = adm_ctx.reply_dh.ret_code;
    if (retcode != NO_ERROR)
    goto out;
    device = adm_ctx.device;
    if (!get_ldev(device)) {
    retcode = ERR_NO_DISK;
    goto out;
    }
    mutex_lock(&adm_ctx.resource.adm_mutex);
// If there is still bitmap IO pending, probably because of a previous
// resync just being finished, wait for it before requesting a new resync.
// Also wait for it's after_state_ch().
    drbd_suspend_io(device);
    wait_event(device.misc_wait, !test_bit(BITMAP_IO, &device.flags));
    drbd_flush_workqueue(&first_peer_device(device).connection.sender_work);
// If we happen to be C_STANDALONE R_PRIMARY, just set all bits
// in the bitmap.  Otherwise, try to start a resync handshake
// as sync source for full sync.
//
    if (device.state.conn == C_STANDALONE && device.state.role == R_PRIMARY) {
// The peer will get a resync upon connect anyways. Just make that
    into a full resync. */
    retcode = drbd_request_state(device, NS(pdsk, D_INCONSISTENT));
    if (retcode >= SS_SUCCESS) {
    if (drbd_bitmap_io(device, &drbd_bmio_set_susp_al,
    "set_n_write from invalidate_peer",
    BM_LOCKED_SET_ALLOWED, core::ptr::null_mut()))
    retcode = ERR_IO_MD_DISK;
    }
    } else
    retcode = drbd_request_state(device, NS(conn, C_STARTING_SYNC_S));
    drbd_resume_io(device);
    mutex_unlock(&adm_ctx.resource.adm_mutex);
    put_ldev(device);
    out:
    adm_ctx.reply_dh.ret_code = retcode;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn drbd_nl_pause_sync_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    int drbd_nl_pause_sync_doit(struct sk_buff *skb, struct genl_info *info)
    {
    struct drbd_config_context *adm_ctx = info.user_ptr[0];
    enum drbd_ret_code retcode;
    if (!adm_ctx.reply_skb)
    return 0;
    retcode = adm_ctx.reply_dh.ret_code;
    if (retcode != NO_ERROR)
    goto out;
    mutex_lock(&adm_ctx.resource.adm_mutex);
    if (drbd_request_state(adm_ctx.device, NS(user_isp, 1)) == SS_NOTHING_TO_DO)
    retcode = ERR_PAUSE_IS_SET;
    mutex_unlock(&adm_ctx.resource.adm_mutex);
    out:
    adm_ctx.reply_dh.ret_code = retcode;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn drbd_nl_resume_sync_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    int drbd_nl_resume_sync_doit(struct sk_buff *skb, struct genl_info *info)
    {
    struct drbd_config_context *adm_ctx = info.user_ptr[0];
    union drbd_dev_state s;
    enum drbd_ret_code retcode;
    if (!adm_ctx.reply_skb)
    return 0;
    retcode = adm_ctx.reply_dh.ret_code;
    if (retcode != NO_ERROR)
    goto out;
    mutex_lock(&adm_ctx.resource.adm_mutex);
    if (drbd_request_state(adm_ctx.device, NS(user_isp, 0)) == SS_NOTHING_TO_DO) {
    s = adm_ctx.device.state;
    if (s.conn == C_PAUSED_SYNC_S || s.conn == C_PAUSED_SYNC_T) {
    retcode = s.aftr_isp ? ERR_PIC_AFTER_DEP :
    s.peer_isp ? ERR_PIC_PEER_DEP : ERR_PAUSE_IS_CLEAR;
    } else {
    retcode = ERR_PAUSE_IS_CLEAR;
    }
    }
    mutex_unlock(&adm_ctx.resource.adm_mutex);
    out:
    adm_ctx.reply_dh.ret_code = retcode;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn drbd_nl_suspend_io_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    int drbd_nl_suspend_io_doit(struct sk_buff *skb, struct genl_info *info)
    {
    return drbd_adm_simple_request_state(skb, info, NS(susp, 1));
    }
#[no_mangle]
pub unsafe extern "C" fn drbd_nl_resume_io_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    int drbd_nl_resume_io_doit(struct sk_buff *skb, struct genl_info *info)
    {
    struct drbd_config_context *adm_ctx = info.user_ptr[0];
    struct drbd_device *device;
    int retcode; /* enum drbd_ret_code rsp. enum drbd_state_rv */
    if (!adm_ctx.reply_skb)
    return 0;
    retcode = adm_ctx.reply_dh.ret_code;
    if (retcode != NO_ERROR)
    goto out;
    mutex_lock(&adm_ctx.resource.adm_mutex);
    device = adm_ctx.device;
    if (test_bit(NEW_CUR_UUID, &device.flags)) {
    if (get_ldev_if_state(device, D_ATTACHING)) {
    drbd_uuid_new_current(device);
    put_ldev(device);
    } else {
// This is effectively a multi-stage "forced down".
// The NEW_CUR_UUID bit is supposedly only set, if we
// lost the replication connection, and are configured
// to freeze IO and wait for some fence-peer handler.
// So we still don't have a replication connection.
// And now we don't have a local disk either.  After
// resume, we will fail all pending and new IO, because
// we don't have any data anymore.  Which means we will
// eventually be able to terminate all users of this
// device, and then take it down.  By bumping the
// "effective" data uuid, we make sure that you really
// need to tear down before you reconfigure, we will
// the refuse to re-connect or re-attach (because no
// matching real data uuid exists).
//
    u64 val;
    val = get_random_u64();
    drbd_set_ed_uuid(device, val);
    drbd_warn(device, "Resumed without access to data; please tear down before attempting to re-configure.\n");
    }
    clear_bit(NEW_CUR_UUID, &device.flags);
    }
    drbd_suspend_io(device);
    retcode = drbd_request_state(device, NS3(susp, 0, susp_nod, 0, susp_fen, 0));
    if (retcode == SS_SUCCESS) {
    if (device.state.conn < C_CONNECTED)
    tl_clear(first_peer_device(device).connection);
    if (device.state.disk == D_DISKLESS || device.state.disk == D_FAILED)
    tl_restart(first_peer_device(device).connection, FAIL_FROZEN_DISK_IO);
    }
    drbd_resume_io(device);
    mutex_unlock(&adm_ctx.resource.adm_mutex);
    out:
    adm_ctx.reply_dh.ret_code = retcode;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn drbd_nl_outdate_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    int drbd_nl_outdate_doit(struct sk_buff *skb, struct genl_info *info)
    {
    return drbd_adm_simple_request_state(skb, info, NS(disk, D_OUTDATED));
    }
    static int nla_put_drbd_cfg_context(struct sk_buff *skb,
    struct drbd_resource *resource,
    struct drbd_connection *connection,
    struct drbd_device *device)
    {
    struct nlattr *nla;
    nla = nla_nest_start_noflag(skb, DRBD_NLA_CFG_CONTEXT);
    if (!nla)
    goto nla_put_failure;
    if (device &&
    nla_put_u32(skb, DRBD_A_DRBD_CFG_CONTEXT_CTX_VOLUME, device.vnr))
    goto nla_put_failure;
    if (nla_put_string(skb, DRBD_A_DRBD_CFG_CONTEXT_CTX_RESOURCE_NAME, resource.name))
    goto nla_put_failure;
    if (connection) {
    if (connection.my_addr_len &&
    nla_put(skb, DRBD_A_DRBD_CFG_CONTEXT_CTX_MY_ADDR,
    connection.my_addr_len,
    &connection.my_addr))
    goto nla_put_failure;
    if (connection.peer_addr_len &&
    nla_put(skb, DRBD_A_DRBD_CFG_CONTEXT_CTX_PEER_ADDR,
    connection.peer_addr_len,
    &connection.peer_addr))
    goto nla_put_failure;
    }
    nla_nest_end(skb, nla);
    return 0;
    nla_put_failure:
    if (nla)
    nla_nest_cancel(skb, nla);
    return -EMSGSIZE;
    }
//
// net_conf_to_skb() serializes the shared secret verbatim. Any path that can
// answer a request from an unprivileged process must pass exclude_sensitive,
// so the secret is blanked in a private copy before it reaches the skb.
//
    static int net_conf_to_skb_sanitized(struct sk_buff *skb, struct net_conf *nc,
    bool exclude_sensitive)
    {
    struct net_conf nc_clean;
    if (!exclude_sensitive)
    return net_conf_to_skb(skb, nc);
    nc_clean = *nc;
    memset(nc_clean.shared_secret, 0, sizeof(nc_clean.shared_secret));
    nc_clean.shared_secret_len = 0;
    return net_conf_to_skb(skb, &nc_clean);
    }
//
// The generic netlink dump callbacks are called outside the genl_lock(), so
// they cannot use the simple attribute parsing code which uses global
// attribute tables.
//
    static struct nlattr *find_cfg_context_attr(const struct nlmsghdr *nlh, int attr)
    {
    let mut hdrlen: c_uint = GENL_HDRLEN + sizeof(struct drbd_genlmsghdr);
    struct nlattr *nla;
    nla = nla_find(nlmsg_attrdata(nlh, hdrlen), nlmsg_attrlen(nlh, hdrlen),
    DRBD_NLA_CFG_CONTEXT);
    if (!nla)
    return core::ptr::null_mut();
    return nla_find_nested(nla, attr);
    }
    static void resource_to_info(struct resource_info *, struct drbd_resource *);
#[no_mangle]
pub unsafe extern "C" fn drbd_nl_get_resources_dumpit(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int {
    int drbd_nl_get_resources_dumpit(struct sk_buff *skb, struct netlink_callback *cb)
    {
    struct drbd_genlmsghdr *dh;
    struct drbd_resource *resource;
    struct resource_info resource_info;
    struct resource_statistics resource_statistics;
    int err;
    rcu_read_lock();
    if (cb.args[0]) {
    for_each_resource_rcu(resource, &drbd_resources)
    if (resource == (struct drbd_resource *)cb.args[0])
    goto found_resource;
    err = 0;  /* resource was probably deleted */
    goto out;
    }
    resource = list_entry(&drbd_resources,
    struct drbd_resource, resources);
    found_resource:
    list_for_each_entry_continue_rcu(resource, &drbd_resources, resources) {
    goto put_result;
    }
    err = 0;
    goto out;
    put_result:
    dh = genlmsg_put(skb, NETLINK_CB(cb.skb).portid,
    cb.nlh.nlmsg_seq, &drbd_nl_family,
    NLM_F_MULTI, DRBD_ADM_GET_RESOURCES);
    err = -ENOMEM;
    if (!dh)
    goto out;
    dh.minor = -1U;
    dh.ret_code = NO_ERROR;
    err = nla_put_drbd_cfg_context(skb, resource, core::ptr::null_mut(), core::ptr::null_mut());
    if (err)
    goto out;
    err = res_opts_to_skb(skb, &resource.res_opts);
    if (err)
    goto out;
    resource_to_info(&resource_info, resource);
    err = resource_info_to_skb(skb, &resource_info);
    if (err)
    goto out;
    resource_statistics.res_stat_write_ordering = resource.write_ordering;
    err = resource_statistics_to_skb(skb, &resource_statistics);
    if (err)
    goto out;
    cb.args[0] = (long)resource;
    genlmsg_end(skb, dh);
    err = 0;
    out:
    rcu_read_unlock();
    if (err)
    return err;
    return skb.len;
    }
    static void device_to_statistics(struct device_statistics *s,
    struct drbd_device *device)
    {
    memset(s, 0, sizeof(*s));
    s.dev_upper_blocked = !may_inc_ap_bio(device);
    if (get_ldev(device)) {
    struct drbd_md *md = &device.ldev.md;
    u64 *history_uuids = (u64 *)s.history_uuids;
    int n;
    spin_lock_irq(&md.uuid_lock);
    s.dev_current_uuid = md.uuid[UI_CURRENT];
    BUILD_BUG_ON(sizeof(s.history_uuids) < UI_HISTORY_END - UI_HISTORY_START + 1);
    for (n = 0; n < UI_HISTORY_END - UI_HISTORY_START + 1; n++)
    history_uuids[n] = md.uuid[UI_HISTORY_START + n];
    for (; n < HISTORY_UUIDS; n++)
    history_uuids[n] = 0;
    s.history_uuids_len = HISTORY_UUIDS;
    spin_unlock_irq(&md.uuid_lock);
    s.dev_disk_flags = md.flags;
    put_ldev(device);
    }
    s.dev_size = get_capacity(device.vdisk);
    s.dev_read = device.read_cnt;
    s.dev_write = device.writ_cnt;
    s.dev_al_writes = device.al_writ_cnt;
    s.dev_bm_writes = device.bm_writ_cnt;
    s.dev_upper_pending = atomic_read(&device.ap_bio_cnt);
    s.dev_lower_pending = atomic_read(&device.local_cnt);
    s.dev_al_suspended = test_bit(AL_SUSPENDED, &device.flags);
    s.dev_exposed_data_uuid = device.ed_uuid;
    }
#[no_mangle]
unsafe extern "C" fn put_resource_in_arg0(cb: *mut netlink_callback, holder_nr: c_int) -> c_int {
    static int put_resource_in_arg0(struct netlink_callback *cb, int holder_nr)
    {
    if (cb.args[0]) {
    struct drbd_resource *resource =
    (struct drbd_resource *)cb.args[0];
    kref_put(&resource.kref, drbd_destroy_resource);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn drbd_adm_dump_devices_done(cb: *mut netlink_callback) -> c_int {
    return put_resource_in_arg0(cb, 7);
    }
    static void device_to_info(struct device_info *, struct drbd_device *);
#[no_mangle]
pub unsafe extern "C" fn drbd_nl_get_devices_dumpit(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int {
    int drbd_nl_get_devices_dumpit(struct sk_buff *skb, struct netlink_callback *cb)
    {
    struct nlattr *resource_filter;
    struct drbd_resource *resource;
    struct drbd_device *device;
    int minor, err, retcode;
    struct drbd_genlmsghdr *dh;
    struct device_info device_info;
    struct device_statistics device_statistics;
    struct idr *idr_to_search;
    resource = (struct drbd_resource *)cb.args[0];
    if (!cb.args[0] && !cb.args[1]) {
    resource_filter = find_cfg_context_attr(cb.nlh,
    DRBD_A_DRBD_CFG_CONTEXT_CTX_RESOURCE_NAME);
    if (resource_filter) {
    retcode = ERR_RES_NOT_KNOWN;
    resource = drbd_find_resource(nla_data(resource_filter));
    if (!resource) {
    rcu_read_lock();
    goto put_result;
    }
    cb.args[0] = (long)resource;
    }
    }
    rcu_read_lock();
    minor = cb.args[1];
    idr_to_search = resource ? &resource.devices : &drbd_devices;
    device = idr_get_next(idr_to_search, &minor);
    if (!device) {
    err = 0;
    goto out;
    }
    idr_for_each_entry_continue(idr_to_search, device, minor) {
    retcode = NO_ERROR;
    goto put_result;  /* only one iteration */
    }
    err = 0;
    goto out;  /* no more devices */
    put_result:
    dh = genlmsg_put(skb, NETLINK_CB(cb.skb).portid,
    cb.nlh.nlmsg_seq, &drbd_nl_family,
    NLM_F_MULTI, DRBD_ADM_GET_DEVICES);
    err = -ENOMEM;
    if (!dh)
    goto out;
    dh.ret_code = retcode;
    dh.minor = -1U;
    if (retcode == NO_ERROR) {
    dh.minor = device.minor;
    err = nla_put_drbd_cfg_context(skb, device.resource, core::ptr::null_mut(), device);
    if (err)
    goto out;
    if (get_ldev(device)) {
    struct disk_conf *disk_conf =
    rcu_dereference(device.ldev.disk_conf);
    err = disk_conf_to_skb(skb, disk_conf);
    put_ldev(device);
    if (err)
    goto out;
    }
    device_to_info(&device_info, device);
    err = device_info_to_skb(skb, &device_info);
    if (err)
    goto out;
    device_to_statistics(&device_statistics, device);
    err = device_statistics_to_skb(skb, &device_statistics);
    if (err)
    goto out;
    cb.args[1] = minor + 1;
    }
    genlmsg_end(skb, dh);
    err = 0;
    out:
    rcu_read_unlock();
    if (err)
    return err;
    return skb.len;
    }
#[no_mangle]
pub unsafe extern "C" fn drbd_adm_dump_connections_done(cb: *mut netlink_callback) -> c_int {
    int drbd_adm_dump_connections_done(struct netlink_callback *cb)
    {
    return put_resource_in_arg0(cb, 6);
    }
    enum { SINGLE_RESOURCE, ITERATE_RESOURCES };
#[no_mangle]
pub unsafe extern "C" fn drbd_nl_get_connections_dumpit(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int {
    int drbd_nl_get_connections_dumpit(struct sk_buff *skb, struct netlink_callback *cb)
    {
    struct nlattr *resource_filter;
    struct drbd_resource *resource = core::ptr::null_mut(), *next_resource;
    struct drbd_connection *connection;
    let mut err: c_int = 0, retcode;
    struct drbd_genlmsghdr *dh;
    struct connection_info connection_info;
    struct connection_statistics connection_statistics;
    rcu_read_lock();
    resource = (struct drbd_resource *)cb.args[0];
    if (!cb.args[0]) {
    resource_filter = find_cfg_context_attr(cb.nlh,
    DRBD_A_DRBD_CFG_CONTEXT_CTX_RESOURCE_NAME);
    if (resource_filter) {
    retcode = ERR_RES_NOT_KNOWN;
    resource = drbd_find_resource(nla_data(resource_filter));
    if (!resource)
    goto put_result;
    cb.args[0] = (long)resource;
    cb.args[1] = SINGLE_RESOURCE;
    }
    }
    if (!resource) {
    if (list_empty(&drbd_resources))
    goto out;
    resource = list_first_entry(&drbd_resources, struct drbd_resource, resources);
    kref_get(&resource.kref);
    cb.args[0] = (long)resource;
    cb.args[1] = ITERATE_RESOURCES;
    }
    next_resource:
    rcu_read_unlock();
    mutex_lock(&resource.conf_update);
    rcu_read_lock();
    if (cb.args[2]) {
    for_each_connection_rcu(connection, resource)
    if (connection == (struct drbd_connection *)cb.args[2])
    goto found_connection;
// connection was probably deleted
    goto no_more_connections;
    }
    connection = list_entry(&resource.connections, struct drbd_connection, connections);
    found_connection:
    list_for_each_entry_continue_rcu(connection, &resource.connections, connections) {
    if (!has_net_conf(connection))
    continue;
    retcode = NO_ERROR;
    goto put_result;  /* only one iteration */
    }
    no_more_connections:
    if (cb.args[1] == ITERATE_RESOURCES) {
    for_each_resource_rcu(next_resource, &drbd_resources) {
    if (next_resource == resource)
    goto found_resource;
    }
// resource was probably deleted
    }
    goto out;
    found_resource:
    list_for_each_entry_continue_rcu(next_resource, &drbd_resources, resources) {
    mutex_unlock(&resource.conf_update);
    kref_put(&resource.kref, drbd_destroy_resource);
    resource = next_resource;
    kref_get(&resource.kref);
    cb.args[0] = (long)resource;
    cb.args[2] = 0;
    goto next_resource;
    }
    goto out;  /* no more resources */
    put_result:
    dh = genlmsg_put(skb, NETLINK_CB(cb.skb).portid,
    cb.nlh.nlmsg_seq, &drbd_nl_family,
    NLM_F_MULTI, DRBD_ADM_GET_CONNECTIONS);
    err = -ENOMEM;
    if (!dh)
    goto out;
    dh.ret_code = retcode;
    dh.minor = -1U;
    if (retcode == NO_ERROR) {
    struct net_conf *net_conf;
    err = nla_put_drbd_cfg_context(skb, resource, connection, core::ptr::null_mut());
    if (err)
    goto out;
    net_conf = rcu_dereference(connection.net_conf);
    if (net_conf) {
    err = net_conf_to_skb_sanitized(skb, net_conf,
    !capable(CAP_SYS_ADMIN));
    if (err)
    goto out;
    }
    connection_to_info(&connection_info, connection);
    err = connection_info_to_skb(skb, &connection_info);
    if (err)
    goto out;
    connection_statistics.conn_congested = test_bit(NET_CONGESTED, &connection.flags);
    err = connection_statistics_to_skb(skb, &connection_statistics);
    if (err)
    goto out;
    cb.args[2] = (long)connection;
    }
    genlmsg_end(skb, dh);
    err = 0;
    out:
    rcu_read_unlock();
    if (resource)
    mutex_unlock(&resource.conf_update);
    if (err)
    return err;
    return skb.len;
    }
    enum mdf_peer_flag {
    MDF_PEER_CONNECTED =	1 << 0,
    MDF_PEER_OUTDATED =	1 << 1,
    MDF_PEER_FENCING =	1 << 2,
    MDF_PEER_FULL_SYNC =	1 << 3,
    };
    static void peer_device_to_statistics(struct peer_device_statistics *s,
    struct drbd_peer_device *peer_device)
    {
    struct drbd_device *device = peer_device.device;
    memset(s, 0, sizeof(*s));
    s.peer_dev_received = device.recv_cnt;
    s.peer_dev_sent = device.send_cnt;
    s.peer_dev_pending = atomic_read(&device.ap_pending_cnt) +
    atomic_read(&device.rs_pending_cnt);
    s.peer_dev_unacked = atomic_read(&device.unacked_cnt);
    s.peer_dev_out_of_sync = drbd_bm_total_weight(device) << (BM_BLOCK_SHIFT - 9);
    s.peer_dev_resync_failed = device.rs_failed << (BM_BLOCK_SHIFT - 9);
    if (get_ldev(device)) {
    struct drbd_md *md = &device.ldev.md;
    spin_lock_irq(&md.uuid_lock);
    s.peer_dev_bitmap_uuid = md.uuid[UI_BITMAP];
    spin_unlock_irq(&md.uuid_lock);
    s.peer_dev_flags =
    (drbd_md_test_flag(device.ldev, MDF_CONNECTED_IND) ?
    MDF_PEER_CONNECTED : 0) +
    (drbd_md_test_flag(device.ldev, MDF_CONSISTENT) &&
    !drbd_md_test_flag(device.ldev, MDF_WAS_UP_TO_DATE) ?
    MDF_PEER_OUTDATED : 0) +
// FIXME: MDF_PEER_FENCING?
    (drbd_md_test_flag(device.ldev, MDF_FULL_SYNC) ?
    MDF_PEER_FULL_SYNC : 0);
    put_ldev(device);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn drbd_adm_dump_peer_devices_done(cb: *mut netlink_callback) -> c_int {
    int drbd_adm_dump_peer_devices_done(struct netlink_callback *cb)
    {
    return put_resource_in_arg0(cb, 9);
    }
#[no_mangle]
pub unsafe extern "C" fn drbd_nl_get_peer_devices_dumpit(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int {
    int drbd_nl_get_peer_devices_dumpit(struct sk_buff *skb, struct netlink_callback *cb)
    {
    struct nlattr *resource_filter;
    struct drbd_resource *resource;
    struct drbd_device *device;
    struct drbd_peer_device *peer_device = core::ptr::null_mut();
    int minor, err, retcode;
    struct drbd_genlmsghdr *dh;
    struct idr *idr_to_search;
    resource = (struct drbd_resource *)cb.args[0];
    if (!cb.args[0] && !cb.args[1]) {
    resource_filter = find_cfg_context_attr(cb.nlh,
    DRBD_A_DRBD_CFG_CONTEXT_CTX_RESOURCE_NAME);
    if (resource_filter) {
    retcode = ERR_RES_NOT_KNOWN;
    resource = drbd_find_resource(nla_data(resource_filter));
    if (!resource) {
    rcu_read_lock();
    goto put_result;
    }
    }
    cb.args[0] = (long)resource;
    }
    rcu_read_lock();
    minor = cb.args[1];
    idr_to_search = resource ? &resource.devices : &drbd_devices;
    device = idr_find(idr_to_search, minor);
    if (!device) {
    next_device:
    minor++;
    cb.args[2] = 0;
    device = idr_get_next(idr_to_search, &minor);
    if (!device) {
    err = 0;
    goto out;
    }
    }
    if (cb.args[2]) {
    for_each_peer_device(peer_device, device)
    if (peer_device == (struct drbd_peer_device *)cb.args[2])
    goto found_peer_device;
// peer device was probably deleted
    goto next_device;
    }
// Make peer_device point to the list head (not the first entry).
    peer_device = list_entry(&device.peer_devices, struct drbd_peer_device, peer_devices);
    found_peer_device:
    list_for_each_entry_continue_rcu(peer_device, &device.peer_devices, peer_devices) {
    if (!has_net_conf(peer_device.connection))
    continue;
    retcode = NO_ERROR;
    goto put_result;  /* only one iteration */
    }
    goto next_device;
    put_result:
    dh = genlmsg_put(skb, NETLINK_CB(cb.skb).portid,
    cb.nlh.nlmsg_seq, &drbd_nl_family,
    NLM_F_MULTI, DRBD_ADM_GET_PEER_DEVICES);
    err = -ENOMEM;
    if (!dh)
    goto out;
    dh.ret_code = retcode;
    dh.minor = -1U;
    if (retcode == NO_ERROR) {
    struct peer_device_info peer_device_info;
    struct peer_device_statistics peer_device_statistics;
    dh.minor = minor;
    err = nla_put_drbd_cfg_context(skb, device.resource, peer_device.connection, device);
    if (err)
    goto out;
    peer_device_to_info(&peer_device_info, peer_device);
    err = peer_device_info_to_skb(skb, &peer_device_info);
    if (err)
    goto out;
    peer_device_to_statistics(&peer_device_statistics, peer_device);
    err = peer_device_statistics_to_skb(skb, &peer_device_statistics);
    if (err)
    goto out;
    cb.args[1] = minor;
    cb.args[2] = (long)peer_device;
    }
    genlmsg_end(skb, dh);
    err = 0;
    out:
    rcu_read_unlock();
    if (err)
    return err;
    return skb.len;
    }
//
// Return the connection of @resource if @resource has exactly one connection.
//
    static struct drbd_connection *the_only_connection(struct drbd_resource *resource)
    {
    struct list_head *connections = &resource.connections;
    if (list_empty(connections) || connections.next.next != connections)
    return core::ptr::null_mut();
    return list_first_entry(&resource.connections, struct drbd_connection, connections);
    }
    static int nla_put_status_info(struct sk_buff *skb, struct drbd_device *device,
    const struct sib_info *sib)
    {
    struct drbd_resource *resource = device.resource;
    struct state_info *si = core::ptr::null_mut(); /* for sizeof(si.member); */
    struct nlattr *nla;
    int got_ldev;
    let mut err: c_int = 0;
    int exclude_sensitive;
// If sib != NULL, this is drbd_bcast_event, which anyone can listen
// to.  So we better exclude_sensitive information.
//
// If sib == NULL, this is drbd_nl_get_status_doit, executed synchronously
// in the context of the requesting user process. Exclude sensitive
// information, unless current has superuser.
//
// NOTE: for drbd_nl_get_status_dumpit(), this is a netlink dump, and
// relies on the current implementation of netlink_dump(), which
// executes the dump callback successively from netlink_recvmsg(),
// always in the context of the receiving process
    exclude_sensitive = sib || !capable(CAP_SYS_ADMIN);
    got_ldev = get_ldev(device);
// We need to add connection name and volume number information still.
// Minor number is in drbd_genlmsghdr.
    if (nla_put_drbd_cfg_context(skb, resource, the_only_connection(resource), device))
    goto nla_put_failure;
    if (res_opts_to_skb(skb, &device.resource.res_opts))
    goto nla_put_failure;
    rcu_read_lock();
    if (got_ldev) {
    struct disk_conf *disk_conf;
    disk_conf = rcu_dereference(device.ldev.disk_conf);
    err = disk_conf_to_skb(skb, disk_conf);
    }
    if (!err) {
    struct net_conf *nc;
    nc = rcu_dereference(first_peer_device(device).connection.net_conf);
    if (nc)
    err = net_conf_to_skb_sanitized(skb, nc, exclude_sensitive);
    }
    rcu_read_unlock();
    if (err)
    goto nla_put_failure;
    nla = nla_nest_start_noflag(skb, DRBD_NLA_STATE_INFO);
    if (!nla)
    goto nla_put_failure;
    if (nla_put_u32(skb, DRBD_A_STATE_INFO_SIB_REASON,
    sib ? sib.sib_reason : SIB_GET_STATUS_REPLY) ||
    nla_put_u32(skb, DRBD_A_STATE_INFO_CURRENT_STATE,
    device.state.i) ||
    nla_put_u64_64bit(skb, DRBD_A_STATE_INFO_ED_UUID,
    device.ed_uuid, 0) ||
    nla_put_u64_64bit(skb, DRBD_A_STATE_INFO_CAPACITY,
    get_capacity(device.vdisk), 0) ||
    nla_put_u64_64bit(skb, DRBD_A_STATE_INFO_SEND_CNT,
    device.send_cnt, 0) ||
    nla_put_u64_64bit(skb, DRBD_A_STATE_INFO_RECV_CNT,
    device.recv_cnt, 0) ||
    nla_put_u64_64bit(skb, DRBD_A_STATE_INFO_READ_CNT,
    device.read_cnt, 0) ||
    nla_put_u64_64bit(skb, DRBD_A_STATE_INFO_WRIT_CNT,
    device.writ_cnt, 0) ||
    nla_put_u64_64bit(skb, DRBD_A_STATE_INFO_AL_WRIT_CNT,
    device.al_writ_cnt, 0) ||
    nla_put_u64_64bit(skb, DRBD_A_STATE_INFO_BM_WRIT_CNT,
    device.bm_writ_cnt, 0) ||
    nla_put_u32(skb, DRBD_A_STATE_INFO_AP_BIO_CNT,
    atomic_read(&device.ap_bio_cnt)) ||
    nla_put_u32(skb, DRBD_A_STATE_INFO_AP_PENDING_CNT,
    atomic_read(&device.ap_pending_cnt)) ||
    nla_put_u32(skb, DRBD_A_STATE_INFO_RS_PENDING_CNT,
    atomic_read(&device.rs_pending_cnt)))
    goto nla_put_failure;
    if (got_ldev) {
    int err;
    spin_lock_irq(&device.ldev.md.uuid_lock);
    err = nla_put(skb, DRBD_A_STATE_INFO_UUIDS,
    sizeof(si.uuids),
    device.ldev.md.uuid);
    spin_unlock_irq(&device.ldev.md.uuid_lock);
    if (err)
    goto nla_put_failure;
    if (nla_put_u32(skb, DRBD_A_STATE_INFO_DISK_FLAGS, device.ldev.md.flags) ||
    nla_put_u64_64bit(skb, DRBD_A_STATE_INFO_BITS_TOTAL, drbd_bm_bits(device), 0) ||
    nla_put_u64_64bit(skb, DRBD_A_STATE_INFO_BITS_OOS,
    drbd_bm_total_weight(device), 0))
    goto nla_put_failure;
    if (C_SYNC_SOURCE <= device.state.conn &&
    C_PAUSED_SYNC_T >= device.state.conn) {
    if (nla_put_u64_64bit(skb, DRBD_A_STATE_INFO_BITS_RS_TOTAL,
    device.rs_total, 0) ||
    nla_put_u64_64bit(skb, DRBD_A_STATE_INFO_BITS_RS_FAILED,
    device.rs_failed, 0))
    goto nla_put_failure;
    }
    }
    if (sib) {
    switch(sib.sib_reason) {
    case SIB_SYNC_PROGRESS:
    case SIB_GET_STATUS_REPLY:
    break;
    case SIB_STATE_CHANGE:
    if (nla_put_u32(skb, DRBD_A_STATE_INFO_PREV_STATE, sib.os.i) ||
    nla_put_u32(skb, DRBD_A_STATE_INFO_NEW_STATE, sib.ns.i))
    goto nla_put_failure;
    break;
    case SIB_HELPER_POST:
    if (nla_put_u32(skb, DRBD_A_STATE_INFO_HELPER_EXIT_CODE,
    sib.helper_exit_code))
    goto nla_put_failure;
    fallthrough;
    case SIB_HELPER_PRE:
    if (nla_put_string(skb, DRBD_A_STATE_INFO_HELPER, sib.helper_name))
    goto nla_put_failure;
    break;
    }
    }
    nla_nest_end(skb, nla);
    if (0)
    nla_put_failure:
    err = -EMSGSIZE;
    if (got_ldev)
    put_ldev(device);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn drbd_nl_get_status_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    int drbd_nl_get_status_doit(struct sk_buff *skb, struct genl_info *info)
    {
    struct drbd_config_context *adm_ctx = info.user_ptr[0];
    enum drbd_ret_code retcode;
    int err;
    if (!adm_ctx.reply_skb)
    return 0;
    retcode = adm_ctx.reply_dh.ret_code;
    if (retcode != NO_ERROR)
    goto out;
    err = nla_put_status_info(adm_ctx.reply_skb, adm_ctx.device, core::ptr::null_mut());
    if (err) {
    nlmsg_free(adm_ctx.reply_skb);
    adm_ctx.reply_skb = core::ptr::null_mut();
    return err;
    }
    out:
    adm_ctx.reply_dh.ret_code = retcode;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn get_one_status(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int {
    static int get_one_status(struct sk_buff *skb, struct netlink_callback *cb)
    {
    struct drbd_device *device;
    struct drbd_genlmsghdr *dh;
    struct drbd_resource *pos = (struct drbd_resource *)cb.args[0];
    struct drbd_resource *resource = core::ptr::null_mut();
    struct drbd_resource *tmp;
    let mut volume: unsigned = cb.args[1];
// Open coded, deferred, iteration:
// for_each_resource_safe(resource, tmp, &drbd_resources) {
// connection = "first connection of resource or undefined";
// idr_for_each_entry(&resource->devices, device, i) {
// ...
// }
// where resource is cb->args[0];
// and i is cb->args[1];
//
// cb->args[2] indicates if we shall loop over all resources,
// or just dump all volumes of a single resource.
//
// This may miss entries inserted after this dump started,
// or entries deleted before they are reached.
//
// We need to make sure the device won't disappear while
// we are looking at it, and revalidate our iterators
// on each iteration.
//
// synchronize with conn_create()/drbd_destroy_connection()
    rcu_read_lock();
// revalidate iterator position
    for_each_resource_rcu(tmp, &drbd_resources) {
    if (pos == core::ptr::null_mut()) {
// first iteration
    pos = tmp;
    resource = pos;
    break;
    }
    if (tmp == pos) {
    resource = pos;
    break;
    }
    }
    if (resource) {
    next_resource:
    device = idr_get_next(&resource.devices, &volume);
    if (!device) {
// No more volumes to dump on this resource.
// Advance resource iterator.
    pos = list_entry_rcu(resource.resources.next,
    struct drbd_resource, resources);
// Did we dump any volume of this resource yet?
    if (volume != 0) {
// If we reached the end of the list,
// or only a single resource dump was requested,
// we are done.
    if (&pos.resources == &drbd_resources || cb.args[2])
    goto out;
    volume = 0;
    resource = pos;
    goto next_resource;
    }
    }
    dh = genlmsg_put(skb, NETLINK_CB(cb.skb).portid,
    cb.nlh.nlmsg_seq, &drbd_nl_family,
    NLM_F_MULTI, DRBD_ADM_GET_STATUS);
    if (!dh)
    goto out;
    if (!device) {
// This is a connection without a single volume.
// Suprisingly enough, it may have a network
// configuration.
    struct drbd_connection *connection;
    dh.minor = -1U;
    dh.ret_code = NO_ERROR;
    connection = the_only_connection(resource);
    if (nla_put_drbd_cfg_context(skb, resource, connection, core::ptr::null_mut()))
    goto cancel;
    if (connection) {
    struct net_conf *nc;
    nc = rcu_dereference(connection.net_conf);
    if (nc && net_conf_to_skb_sanitized(skb, nc, true) != 0)
    goto cancel;
    }
    goto done;
    }
    D_ASSERT(device, device.vnr == volume);
    D_ASSERT(device, device.resource == resource);
    dh.minor = device_to_minor(device);
    dh.ret_code = NO_ERROR;
    if (nla_put_status_info(skb, device, core::ptr::null_mut())) {
    cancel:
    genlmsg_cancel(skb, dh);
    goto out;
    }
    done:
    genlmsg_end(skb, dh);
    }
    out:
    rcu_read_unlock();
// where to start the next iteration
    cb.args[0] = (long)pos;
    cb.args[1] = (pos == resource) ? volume + 1 : 0;
// No more resources/volumes/minors found results in an empty skb.
// Which will terminate the dump.
    return skb.len;
    }
//
// Request status of all resources, or of all volumes within a single resource.
//
// This is a dump, as the answer may not fit in a single reply skb otherwise.
// Which means we cannot use the family->attrbuf or other such members, because
// dump is NOT protected by the genl_lock().  During dump, we only have access
// to the incoming skb, and need to opencode "parsing" of the nlattr payload.
//
// Once things are setup properly, we call into get_one_status().
//
#[no_mangle]
pub unsafe extern "C" fn drbd_nl_get_status_dumpit(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int {
    int drbd_nl_get_status_dumpit(struct sk_buff *skb, struct netlink_callback *cb)
    {
    let mut hdrlen: c_uint = GENL_HDRLEN + sizeof(struct drbd_genlmsghdr);
    struct nlattr *nla;
    const char *resource_name;
    struct drbd_resource *resource;
// Is this a followup call?
    if (cb.args[0]) {
// ... of a single resource dump,
// and the resource iterator has been advanced already?
    if (cb.args[2] && cb.args[2] != cb.args[0])
    return 0; /* DONE. */
    goto dump;
    }
// First call (from netlink_dump_start).  We need to figure out
// which resource(s) the user wants us to dump.
    nla = nla_find(nlmsg_attrdata(cb.nlh, hdrlen),
    nlmsg_attrlen(cb.nlh, hdrlen),
    DRBD_NLA_CFG_CONTEXT);
// No explicit context given.  Dump all.
    if (!nla)
    goto dump;
    nla = nla_find_nested(nla, DRBD_A_DRBD_CFG_CONTEXT_CTX_RESOURCE_NAME);
// context given, but no name present?
    if (!nla)
    return -EINVAL;
    resource_name = nla_data(nla);
    if (!*resource_name)
    return -ENODEV;
    resource = drbd_find_resource(resource_name);
    if (!resource)
    return -ENODEV;
    kref_put(&resource.kref, drbd_destroy_resource); /* get_one_status() revalidates the resource */
// prime iterators, and set "filter" mode mark:
// only dump this connection.
    cb.args[0] = (long)resource;
// cb->args[1] = 0; passed in this way.
    cb.args[2] = (long)resource;
    dump:
    return get_one_status(skb, cb);
    }
#[no_mangle]
pub unsafe extern "C" fn drbd_nl_get_timeout_type_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    int drbd_nl_get_timeout_type_doit(struct sk_buff *skb, struct genl_info *info)
    {
    struct drbd_config_context *adm_ctx = info.user_ptr[0];
    enum drbd_ret_code retcode;
    struct timeout_parms tp;
    int err;
    if (!adm_ctx.reply_skb)
    return 0;
    retcode = adm_ctx.reply_dh.ret_code;
    if (retcode != NO_ERROR)
    goto out;
    tp.timeout_type =
    adm_ctx.device.state.pdsk == D_OUTDATED ? UT_PEER_OUTDATED :
    test_bit(USE_DEGR_WFC_T, &adm_ctx.device.flags) ? UT_DEGRADED :
    UT_DEFAULT;
    err = timeout_parms_to_skb(adm_ctx.reply_skb, &tp);
    if (err) {
    nlmsg_free(adm_ctx.reply_skb);
    adm_ctx.reply_skb = core::ptr::null_mut();
    return err;
    }
    out:
    adm_ctx.reply_dh.ret_code = retcode;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn drbd_nl_start_ov_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    int drbd_nl_start_ov_doit(struct sk_buff *skb, struct genl_info *info)
    {
    struct drbd_config_context *adm_ctx = info.user_ptr[0];
    struct drbd_device *device;
    enum drbd_ret_code retcode;
    struct start_ov_parms parms;
    if (!adm_ctx.reply_skb)
    return 0;
    retcode = adm_ctx.reply_dh.ret_code;
    if (retcode != NO_ERROR)
    goto out;
    device = adm_ctx.device;
// resume from last known position, if possible
    parms.ov_start_sector = device.ov_start_sector;
    parms.ov_stop_sector = ULLONG_MAX;
    if (info.attrs[DRBD_NLA_START_OV_PARMS]) {
    let mut err: c_int = start_ov_parms_from_attrs(&parms, info);
    if (err) {
    retcode = ERR_MANDATORY_TAG;
    drbd_msg_put_info(adm_ctx.reply_skb, from_attrs_err_to_txt(err));
    goto out;
    }
    }
    mutex_lock(&adm_ctx.resource.adm_mutex);
// w_make_ov_request expects position to be aligned
    device.ov_start_sector = parms.ov_start_sector & ~(BM_SECT_PER_BIT-1);
    device.ov_stop_sector = parms.ov_stop_sector;
// If there is still bitmap IO pending, e.g. previous resync or verify
// just being finished, wait for it before requesting a new resync.
    drbd_suspend_io(device);
    wait_event(device.misc_wait, !test_bit(BITMAP_IO, &device.flags));
    retcode = drbd_request_state(device, NS(conn, C_VERIFY_S));
    drbd_resume_io(device);
    mutex_unlock(&adm_ctx.resource.adm_mutex);
    out:
    adm_ctx.reply_dh.ret_code = retcode;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn drbd_nl_new_c_uuid_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    int drbd_nl_new_c_uuid_doit(struct sk_buff *skb, struct genl_info *info)
    {
    struct drbd_config_context *adm_ctx = info.user_ptr[0];
    struct drbd_device *device;
    enum drbd_ret_code retcode;
    let mut skip_initial_sync: c_int = 0;
    int err;
    struct new_c_uuid_parms args;
    if (!adm_ctx.reply_skb)
    return 0;
    retcode = adm_ctx.reply_dh.ret_code;
    if (retcode != NO_ERROR)
    goto out_nolock;
    device = adm_ctx.device;
    memset(&args, 0, sizeof(args));
    if (info.attrs[DRBD_NLA_NEW_C_UUID_PARMS]) {
    err = new_c_uuid_parms_from_attrs(&args, info);
    if (err) {
    retcode = ERR_MANDATORY_TAG;
    drbd_msg_put_info(adm_ctx.reply_skb, from_attrs_err_to_txt(err));
    goto out_nolock;
    }
    }
    mutex_lock(&adm_ctx.resource.adm_mutex);
    mutex_lock(device.state_mutex); /* Protects us against serialized state changes. */
    if (!get_ldev(device)) {
    retcode = ERR_NO_DISK;
    goto out;
    }
// this is "skip initial sync", assume to be clean
    if (device.state.conn == C_CONNECTED &&
    first_peer_device(device).connection.agreed_pro_version >= 90 &&
    device.ldev.md.uuid[UI_CURRENT] == UUID_JUST_CREATED && args.clear_bm) {
    drbd_info(device, "Preparing to skip initial sync\n");
    skip_initial_sync = 1;
    } else if (device.state.conn != C_STANDALONE) {
    retcode = ERR_CONNECTED;
    goto out_dec;
    }
    drbd_uuid_set(device, UI_BITMAP, 0); /* Rotate UI_BITMAP to History 1, etc... */
    drbd_uuid_new_current(device); /* New current, previous to UI_BITMAP */
    if (args.clear_bm) {
    err = drbd_bitmap_io(device, &drbd_bmio_clear_n_write,
    "clear_n_write from new_c_uuid", BM_LOCKED_MASK, core::ptr::null_mut());
    if (err) {
    drbd_err(device, "Writing bitmap failed with %d\n", err);
    retcode = ERR_IO_MD_DISK;
    }
    if (skip_initial_sync) {
    drbd_send_uuids_skip_initial_sync(first_peer_device(device));
    _drbd_uuid_set(device, UI_BITMAP, 0);
    drbd_print_uuids(device, "cleared bitmap UUID");
    spin_lock_irq(&device.resource.req_lock);
    _drbd_set_state(_NS2(device, disk, D_UP_TO_DATE, pdsk, D_UP_TO_DATE),
    CS_VERBOSE, core::ptr::null_mut());
    spin_unlock_irq(&device.resource.req_lock);
    }
    }
    drbd_md_sync(device);
    out_dec:
    put_ldev(device);
    out:
    mutex_unlock(device.state_mutex);
    mutex_unlock(&adm_ctx.resource.adm_mutex);
    out_nolock:
    adm_ctx.reply_dh.ret_code = retcode;
    return 0;
    }
    static enum drbd_ret_code
    drbd_check_resource_name(struct drbd_config_context *adm_ctx)
    {
    const char *name = adm_ctx.resource_name;
    if (!name || !name[0]) {
    drbd_msg_put_info(adm_ctx.reply_skb, "resource name missing");
    return ERR_MANDATORY_TAG;
    }
// if we want to use these in sysfs/configfs/debugfs some day,
// we must not allow slashes
    if (strchr(name, '/')) {
    drbd_msg_put_info(adm_ctx.reply_skb, "invalid resource name");
    return ERR_INVALID_REQUEST;
    }
    return NO_ERROR;
    }
    static void resource_to_info(struct resource_info *info,
    struct drbd_resource *resource)
    {
    info.res_role = conn_highest_role(first_connection(resource));
    info.res_susp = resource.susp;
    info.res_susp_nod = resource.susp_nod;
    info.res_susp_fen = resource.susp_fen;
    }
#[no_mangle]
pub unsafe extern "C" fn drbd_nl_new_resource_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    int drbd_nl_new_resource_doit(struct sk_buff *skb, struct genl_info *info)
    {
    struct drbd_connection *connection;
    struct drbd_config_context *adm_ctx = info.user_ptr[0];
    enum drbd_ret_code retcode;
    struct res_opts res_opts;
    int err;
    if (!adm_ctx.reply_skb)
    return 0;
    retcode = adm_ctx.reply_dh.ret_code;
    if (retcode != NO_ERROR)
    goto out;
    set_res_opts_defaults(&res_opts);
    err = res_opts_from_attrs(&res_opts, info);
    if (err && err != -ENOMSG) {
    retcode = ERR_MANDATORY_TAG;
    drbd_msg_put_info(adm_ctx.reply_skb, from_attrs_err_to_txt(err));
    goto out;
    }
    retcode = drbd_check_resource_name(adm_ctx);
    if (retcode != NO_ERROR)
    goto out;
    if (adm_ctx.resource) {
    if (info.nlhdr.nlmsg_flags & NLM_F_EXCL) {
    retcode = ERR_INVALID_REQUEST;
    drbd_msg_put_info(adm_ctx.reply_skb, "resource exists");
    }
// else: still NO_ERROR
    goto out;
    }
// not yet safe for genl_family.parallel_ops
    mutex_lock(&resources_mutex);
    connection = conn_create(adm_ctx.resource_name, &res_opts);
    mutex_unlock(&resources_mutex);
    if (connection) {
    struct resource_info resource_info;
    mutex_lock(&notification_mutex);
    resource_to_info(&resource_info, connection.resource);
    notify_resource_state(core::ptr::null_mut(), 0, connection.resource,
    &resource_info, NOTIFY_CREATE);
    mutex_unlock(&notification_mutex);
    } else
    retcode = ERR_NOMEM;
    out:
    adm_ctx.reply_dh.ret_code = retcode;
    return 0;
    }
    static void device_to_info(struct device_info *info,
    struct drbd_device *device)
    {
    info.dev_disk_state = device.state.disk;
    }
#[no_mangle]
pub unsafe extern "C" fn drbd_nl_new_minor_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    int drbd_nl_new_minor_doit(struct sk_buff *skb, struct genl_info *info)
    {
    struct drbd_config_context *adm_ctx = info.user_ptr[0];
    struct drbd_genlmsghdr *dh = genl_info_userhdr(info);
    enum drbd_ret_code retcode;
    if (!adm_ctx.reply_skb)
    return 0;
    retcode = adm_ctx.reply_dh.ret_code;
    if (retcode != NO_ERROR)
    goto out;
    if (dh.minor > MINORMASK) {
    drbd_msg_put_info(adm_ctx.reply_skb, "requested minor out of range");
    retcode = ERR_INVALID_REQUEST;
    goto out;
    }
    if (adm_ctx.volume > DRBD_VOLUME_MAX) {
    drbd_msg_put_info(adm_ctx.reply_skb, "requested volume id out of range");
    retcode = ERR_INVALID_REQUEST;
    goto out;
    }
// drbd_adm_prepare made sure already
// that first_peer_device(device)->connection and device->vnr match the request.
    if (adm_ctx.device) {
    if (info.nlhdr.nlmsg_flags & NLM_F_EXCL)
    retcode = ERR_MINOR_OR_VOLUME_EXISTS;
// else: still NO_ERROR
    goto out;
    }
    mutex_lock(&adm_ctx.resource.adm_mutex);
    retcode = drbd_create_device(adm_ctx, dh.minor);
    if (retcode == NO_ERROR) {
    struct drbd_device *device;
    struct drbd_peer_device *peer_device;
    struct device_info info;
    let mut peer_devices: c_uint = 0;
    enum drbd_notification_type flags;
    device = minor_to_device(dh.minor);
    for_each_peer_device(peer_device, device) {
    if (!has_net_conf(peer_device.connection))
    continue;
    peer_devices++;
    }
    device_to_info(&info, device);
    mutex_lock(&notification_mutex);
    flags = (peer_devices--) ? NOTIFY_CONTINUES : 0;
    notify_device_state(core::ptr::null_mut(), 0, device, &info, NOTIFY_CREATE | flags);
    for_each_peer_device(peer_device, device) {
    struct peer_device_info peer_device_info;
    if (!has_net_conf(peer_device.connection))
    continue;
    peer_device_to_info(&peer_device_info, peer_device);
    flags = (peer_devices--) ? NOTIFY_CONTINUES : 0;
    notify_peer_device_state(core::ptr::null_mut(), 0, peer_device, &peer_device_info,
    NOTIFY_CREATE | flags);
    }
    mutex_unlock(&notification_mutex);
    }
    mutex_unlock(&adm_ctx.resource.adm_mutex);
    out:
    adm_ctx.reply_dh.ret_code = retcode;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adm_del_minor(device: *mut drbd_device) -> enum drbd_ret_code {
    static enum drbd_ret_code adm_del_minor(struct drbd_device *device)
    {
    struct drbd_peer_device *peer_device;
    if (device.state.disk == D_DISKLESS &&
// no need to be device->state.conn == C_STANDALONE &&
// we may want to delete a minor from a live replication group.
//
    device.state.role == R_SECONDARY) {
    struct drbd_connection *connection =
    first_connection(device.resource);
    _drbd_request_state(device, NS(conn, C_WF_REPORT_PARAMS),
    CS_VERBOSE + CS_WAIT_COMPLETE);
// If the state engine hasn't stopped the sender thread yet, we
// need to flush the sender work queue before generating the
// DESTROY events here.
    if (get_t_state(&connection.worker) == RUNNING)
    drbd_flush_workqueue(&connection.sender_work);
    mutex_lock(&notification_mutex);
    for_each_peer_device(peer_device, device) {
    if (!has_net_conf(peer_device.connection))
    continue;
    notify_peer_device_state(core::ptr::null_mut(), 0, peer_device, core::ptr::null_mut(),
    NOTIFY_DESTROY | NOTIFY_CONTINUES);
    }
    notify_device_state(core::ptr::null_mut(), 0, device, core::ptr::null_mut(), NOTIFY_DESTROY);
    mutex_unlock(&notification_mutex);
    drbd_delete_device(device);
    return NO_ERROR;
    } else
    return ERR_MINOR_CONFIGURED;
    }
#[no_mangle]
pub unsafe extern "C" fn drbd_nl_del_minor_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    int drbd_nl_del_minor_doit(struct sk_buff *skb, struct genl_info *info)
    {
    struct drbd_config_context *adm_ctx = info.user_ptr[0];
    enum drbd_ret_code retcode;
    if (!adm_ctx.reply_skb)
    return 0;
    retcode = adm_ctx.reply_dh.ret_code;
    if (retcode != NO_ERROR)
    goto out;
    mutex_lock(&adm_ctx.resource.adm_mutex);
    retcode = adm_del_minor(adm_ctx.device);
    mutex_unlock(&adm_ctx.resource.adm_mutex);
    out:
    adm_ctx.reply_dh.ret_code = retcode;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adm_del_resource(resource: *mut drbd_resource) -> c_int {
    static int adm_del_resource(struct drbd_resource *resource)
    {
    struct drbd_connection *connection;
    for_each_connection(connection, resource) {
    if (connection.cstate > C_STANDALONE)
    return ERR_NET_CONFIGURED;
    }
    if (!idr_is_empty(&resource.devices))
    return ERR_RES_IN_USE;
// The state engine has stopped the sender thread, so we don't
// need to flush the sender work queue before generating the
// DESTROY event here.
    mutex_lock(&notification_mutex);
    notify_resource_state(core::ptr::null_mut(), 0, resource, core::ptr::null_mut(), NOTIFY_DESTROY);
    mutex_unlock(&notification_mutex);
    mutex_lock(&resources_mutex);
    list_del_rcu(&resource.resources);
    mutex_unlock(&resources_mutex);
// Make sure all threads have actually stopped: state handling only
// does drbd_thread_stop_nowait().
    list_for_each_entry(connection, &resource.connections, connections)
    drbd_thread_stop(&connection.worker);
    synchronize_rcu();
    drbd_free_resource(resource);
    return NO_ERROR;
    }
#[no_mangle]
pub unsafe extern "C" fn drbd_nl_down_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    int drbd_nl_down_doit(struct sk_buff *skb, struct genl_info *info)
    {
    struct drbd_config_context *adm_ctx = info.user_ptr[0];
    struct drbd_resource *resource;
    struct drbd_connection *connection;
    struct drbd_device *device;
    int retcode; /* enum drbd_ret_code rsp. enum drbd_state_rv */
    unsigned i;
    if (!adm_ctx.reply_skb)
    return 0;
    retcode = adm_ctx.reply_dh.ret_code;
    if (retcode != NO_ERROR)
    goto finish;
    resource = adm_ctx.resource;
    mutex_lock(&resource.adm_mutex);
// demote
    for_each_connection(connection, resource) {
    struct drbd_peer_device *peer_device;
    idr_for_each_entry(&connection.peer_devices, peer_device, i) {
    retcode = drbd_set_role(peer_device.device, R_SECONDARY, 0);
    if (retcode < SS_SUCCESS) {
    drbd_msg_put_info(adm_ctx.reply_skb, "failed to demote");
    goto out;
    }
    }
    retcode = conn_try_disconnect(connection, 0);
    if (retcode < SS_SUCCESS) {
    drbd_msg_put_info(adm_ctx.reply_skb, "failed to disconnect");
    goto out;
    }
    }
// detach
    idr_for_each_entry(&resource.devices, device, i) {
    retcode = adm_detach(device, 0);
    if (retcode < SS_SUCCESS || retcode > NO_ERROR) {
    drbd_msg_put_info(adm_ctx.reply_skb, "failed to detach");
    goto out;
    }
    }
// delete volumes
    idr_for_each_entry(&resource.devices, device, i) {
    retcode = adm_del_minor(device);
    if (retcode != NO_ERROR) {
// "can not happen"
    drbd_msg_put_info(adm_ctx.reply_skb, "failed to delete volume");
    goto out;
    }
    }
    retcode = adm_del_resource(resource);
    out:
    mutex_unlock(&resource.adm_mutex);
    finish:
    adm_ctx.reply_dh.ret_code = retcode;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn drbd_nl_del_resource_doit(skb: *mut sk_buff, info: *mut genl_info) -> c_int {
    int drbd_nl_del_resource_doit(struct sk_buff *skb, struct genl_info *info)
    {
    struct drbd_config_context *adm_ctx = info.user_ptr[0];
    struct drbd_resource *resource;
    enum drbd_ret_code retcode;
    if (!adm_ctx.reply_skb)
    return 0;
    retcode = adm_ctx.reply_dh.ret_code;
    if (retcode != NO_ERROR)
    goto finish;
    resource = adm_ctx.resource;
    mutex_lock(&resource.adm_mutex);
    retcode = adm_del_resource(resource);
    mutex_unlock(&resource.adm_mutex);
    finish:
    adm_ctx.reply_dh.ret_code = retcode;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn drbd_bcast_event(device: *mut drbd_device, sib: *const sib_info) {
    void drbd_bcast_event(struct drbd_device *device, const struct sib_info *sib)
    {
    struct sk_buff *msg;
    struct drbd_genlmsghdr *d_out;
    unsigned seq;
    let mut err: c_int = -ENOMEM;
    seq = atomic_inc_return(&drbd_genl_seq);
    msg = genlmsg_new(NLMSG_GOODSIZE, GFP_NOIO);
    if (!msg)
    goto failed;
    err = -EMSGSIZE;
    d_out = genlmsg_put(msg, 0, seq, &drbd_nl_family, 0, DRBD_ADM_EVENT);
    if (!d_out) /* cannot happen, but anyways. */
    goto nla_put_failure;
    d_out.minor = device_to_minor(device);
    d_out.ret_code = NO_ERROR;
    if (nla_put_status_info(msg, device, sib))
    goto nla_put_failure;
    genlmsg_end(msg, d_out);
    err = drbd_genl_multicast_events(msg, GFP_NOWAIT);
// msg has been consumed or freed in netlink_broadcast()
    if (err && err != -ESRCH)
    goto failed;
    return;
    nla_put_failure:
    nlmsg_free(msg);
    failed:
    drbd_err(device, "Error %d while broadcasting event. "
    "Event seq:%u sib_reason:%u\n",
    err, seq, sib.sib_reason);
    }
    static int nla_put_notification_header(struct sk_buff *msg,
    enum drbd_notification_type type)
    {
    struct drbd_notification_header nh = {
    .nh_type = type,
    };
    return drbd_notification_header_to_skb(msg, &nh);
    }
    int notify_resource_state(struct sk_buff *skb,
    unsigned int seq,
    struct drbd_resource *resource,
    struct resource_info *resource_info,
    enum drbd_notification_type type)
    {
    struct resource_statistics resource_statistics;
    struct drbd_genlmsghdr *dh;
    let mut multicast: bool = false;
    int err;
    if (!skb) {
    seq = atomic_inc_return(&notify_genl_seq);
    skb = genlmsg_new(NLMSG_GOODSIZE, GFP_NOIO);
    err = -ENOMEM;
    if (!skb)
    goto failed;
    multicast = true;
    }
    err = -EMSGSIZE;
    dh = genlmsg_put(skb, 0, seq, &drbd_nl_family, 0, DRBD_ADM_RESOURCE_STATE);
    if (!dh)
    goto nla_put_failure;
    dh.minor = -1U;
    dh.ret_code = NO_ERROR;
    if (nla_put_drbd_cfg_context(skb, resource, core::ptr::null_mut(), core::ptr::null_mut()) ||
    nla_put_notification_header(skb, type) ||
    ((type & ~NOTIFY_FLAGS) != NOTIFY_DESTROY &&
    resource_info_to_skb(skb, resource_info)))
    goto nla_put_failure;
    resource_statistics.res_stat_write_ordering = resource.write_ordering;
    err = resource_statistics_to_skb(skb, &resource_statistics);
    if (err)
    goto nla_put_failure;
    genlmsg_end(skb, dh);
    if (multicast) {
    err = drbd_genl_multicast_events(skb, GFP_NOWAIT);
// skb has been consumed or freed in netlink_broadcast()
    if (err && err != -ESRCH)
    goto failed;
    }
    return 0;
    nla_put_failure:
    nlmsg_free(skb);
    failed:
    drbd_err(resource, "Error %d while broadcasting event. Event seq:%u\n",
    err, seq);
    return err;
    }
    int notify_device_state(struct sk_buff *skb,
    unsigned int seq,
    struct drbd_device *device,
    struct device_info *device_info,
    enum drbd_notification_type type)
    {
    struct device_statistics device_statistics;
    struct drbd_genlmsghdr *dh;
    let mut multicast: bool = false;
    int err;
    if (!skb) {
    seq = atomic_inc_return(&notify_genl_seq);
    skb = genlmsg_new(NLMSG_GOODSIZE, GFP_NOIO);
    err = -ENOMEM;
    if (!skb)
    goto failed;
    multicast = true;
    }
    err = -EMSGSIZE;
    dh = genlmsg_put(skb, 0, seq, &drbd_nl_family, 0, DRBD_ADM_DEVICE_STATE);
    if (!dh)
    goto nla_put_failure;
    dh.minor = device.minor;
    dh.ret_code = NO_ERROR;
    if (nla_put_drbd_cfg_context(skb, device.resource, core::ptr::null_mut(), device) ||
    nla_put_notification_header(skb, type) ||
    ((type & ~NOTIFY_FLAGS) != NOTIFY_DESTROY &&
    device_info_to_skb(skb, device_info)))
    goto nla_put_failure;
    device_to_statistics(&device_statistics, device);
    device_statistics_to_skb(skb, &device_statistics);
    genlmsg_end(skb, dh);
    if (multicast) {
    err = drbd_genl_multicast_events(skb, GFP_NOWAIT);
// skb has been consumed or freed in netlink_broadcast()
    if (err && err != -ESRCH)
    goto failed;
    }
    return 0;
    nla_put_failure:
    nlmsg_free(skb);
    failed:
    drbd_err(device, "Error %d while broadcasting event. Event seq:%u\n",
    err, seq);
    return err;
    }
    int notify_connection_state(struct sk_buff *skb,
    unsigned int seq,
    struct drbd_connection *connection,
    struct connection_info *connection_info,
    enum drbd_notification_type type)
    {
    struct connection_statistics connection_statistics;
    struct drbd_genlmsghdr *dh;
    let mut multicast: bool = false;
    int err;
    if (!skb) {
    seq = atomic_inc_return(&notify_genl_seq);
    skb = genlmsg_new(NLMSG_GOODSIZE, GFP_NOIO);
    err = -ENOMEM;
    if (!skb)
    goto failed;
    multicast = true;
    }
    err = -EMSGSIZE;
    dh = genlmsg_put(skb, 0, seq, &drbd_nl_family, 0, DRBD_ADM_CONNECTION_STATE);
    if (!dh)
    goto nla_put_failure;
    dh.minor = -1U;
    dh.ret_code = NO_ERROR;
    if (nla_put_drbd_cfg_context(skb, connection.resource, connection, core::ptr::null_mut()) ||
    nla_put_notification_header(skb, type) ||
    ((type & ~NOTIFY_FLAGS) != NOTIFY_DESTROY &&
    connection_info_to_skb(skb, connection_info)))
    goto nla_put_failure;
    connection_statistics.conn_congested = test_bit(NET_CONGESTED, &connection.flags);
    connection_statistics_to_skb(skb, &connection_statistics);
    genlmsg_end(skb, dh);
    if (multicast) {
    err = drbd_genl_multicast_events(skb, GFP_NOWAIT);
// skb has been consumed or freed in netlink_broadcast()
    if (err && err != -ESRCH)
    goto failed;
    }
    return 0;
    nla_put_failure:
    nlmsg_free(skb);
    failed:
    drbd_err(connection, "Error %d while broadcasting event. Event seq:%u\n",
    err, seq);
    return err;
    }
    int notify_peer_device_state(struct sk_buff *skb,
    unsigned int seq,
    struct drbd_peer_device *peer_device,
    struct peer_device_info *peer_device_info,
    enum drbd_notification_type type)
    {
    struct peer_device_statistics peer_device_statistics;
    struct drbd_resource *resource = peer_device.device.resource;
    struct drbd_genlmsghdr *dh;
    let mut multicast: bool = false;
    int err;
    if (!skb) {
    seq = atomic_inc_return(&notify_genl_seq);
    skb = genlmsg_new(NLMSG_GOODSIZE, GFP_NOIO);
    err = -ENOMEM;
    if (!skb)
    goto failed;
    multicast = true;
    }
    err = -EMSGSIZE;
    dh = genlmsg_put(skb, 0, seq, &drbd_nl_family, 0, DRBD_ADM_PEER_DEVICE_STATE);
    if (!dh)
    goto nla_put_failure;
    dh.minor = -1U;
    dh.ret_code = NO_ERROR;
    if (nla_put_drbd_cfg_context(skb, resource, peer_device.connection, peer_device.device) ||
    nla_put_notification_header(skb, type) ||
    ((type & ~NOTIFY_FLAGS) != NOTIFY_DESTROY &&
    peer_device_info_to_skb(skb, peer_device_info)))
    goto nla_put_failure;
    peer_device_to_statistics(&peer_device_statistics, peer_device);
    peer_device_statistics_to_skb(skb, &peer_device_statistics);
    genlmsg_end(skb, dh);
    if (multicast) {
    err = drbd_genl_multicast_events(skb, GFP_NOWAIT);
// skb has been consumed or freed in netlink_broadcast()
    if (err && err != -ESRCH)
    goto failed;
    }
    return 0;
    nla_put_failure:
    nlmsg_free(skb);
    failed:
    drbd_err(peer_device, "Error %d while broadcasting event. Event seq:%u\n",
    err, seq);
    return err;
    }
    void notify_helper(enum drbd_notification_type type,
    struct drbd_device *device, struct drbd_connection *connection,
    const char *name, int status)
    {
    struct drbd_resource *resource = device ? device.resource : connection.resource;
    struct drbd_helper_info helper_info;
    let mut seq: c_uint = atomic_inc_return(&notify_genl_seq);
    struct sk_buff *skb = core::ptr::null_mut();
    struct drbd_genlmsghdr *dh;
    int err;
    strscpy(helper_info.helper_name, name, sizeof(helper_info.helper_name));
    helper_info.helper_name_len = min(strlen(name), sizeof(helper_info.helper_name));
    helper_info.helper_status = status;
    skb = genlmsg_new(NLMSG_GOODSIZE, GFP_NOIO);
    err = -ENOMEM;
    if (!skb)
    goto fail;
    err = -EMSGSIZE;
    dh = genlmsg_put(skb, 0, seq, &drbd_nl_family, 0, DRBD_ADM_HELPER);
    if (!dh)
    goto fail;
    dh.minor = device ? device.minor : -1;
    dh.ret_code = NO_ERROR;
    mutex_lock(&notification_mutex);
    if (nla_put_drbd_cfg_context(skb, resource, connection, device) ||
    nla_put_notification_header(skb, type) ||
    drbd_helper_info_to_skb(skb, &helper_info))
    goto unlock_fail;
    genlmsg_end(skb, dh);
    err = drbd_genl_multicast_events(skb, GFP_NOWAIT);
    skb = core::ptr::null_mut();
// skb has been consumed or freed in netlink_broadcast()
    if (err && err != -ESRCH)
    goto unlock_fail;
    mutex_unlock(&notification_mutex);
    return;
    unlock_fail:
    mutex_unlock(&notification_mutex);
    fail:
    nlmsg_free(skb);
    drbd_err(resource, "Error %d while broadcasting event. Event seq:%u\n",
    err, seq);
    }
#[no_mangle]
unsafe extern "C" fn notify_initial_state_done(skb: *mut sk_buff, seq: c_uint) -> c_int {
    static int notify_initial_state_done(struct sk_buff *skb, unsigned int seq)
    {
    struct drbd_genlmsghdr *dh;
    int err;
    err = -EMSGSIZE;
    dh = genlmsg_put(skb, 0, seq, &drbd_nl_family, 0, DRBD_ADM_INITIAL_STATE_DONE);
    if (!dh)
    goto nla_put_failure;
    dh.minor = -1U;
    dh.ret_code = NO_ERROR;
    if (nla_put_notification_header(skb, NOTIFY_EXISTS))
    goto nla_put_failure;
    genlmsg_end(skb, dh);
    return 0;
    nla_put_failure:
    nlmsg_free(skb);
    pr_err("Error %d sending event. Event seq:%u\n", err, seq);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn free_state_changes(list: *mut list_head) {
    static void free_state_changes(struct list_head *list)
    {
    while (!list_empty(list)) {
    struct drbd_state_change *state_change =
    list_first_entry(list, struct drbd_state_change, list);
    list_del(&state_change.list);
    forget_state_change(state_change);
    }
    }
#[no_mangle]
unsafe extern "C" fn notifications_for_state_change(state_change: *mut drbd_state_change) -> c_uint {
    static unsigned int notifications_for_state_change(struct drbd_state_change *state_change)
    {
    return 1 +
    state_change.n_connections +
    state_change.n_devices +
    state_change.n_devices * state_change.n_connections;
    }
#[no_mangle]
unsafe extern "C" fn get_initial_state(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int {
    static int get_initial_state(struct sk_buff *skb, struct netlink_callback *cb)
    {
    struct drbd_state_change *state_change = (struct drbd_state_change *)cb.args[0];
    let mut seq: c_uint = cb.args[2];
    unsigned int n;
    let mut flags: enum drbd_notification_type = 0;
    let mut err: c_int = 0;
// There is no need for taking notification_mutex here: it doesn't
    matter if the initial state events mix with later state chage
    events; we can always tell the events apart by the NOTIFY_EXISTS
    flag. */
    cb.args[5]--;
    if (cb.args[5] == 1) {
    err = notify_initial_state_done(skb, seq);
    goto out;
    }
    n = cb.args[4]++;
    if (cb.args[4] < cb.args[3])
    flags |= NOTIFY_CONTINUES;
    if (n < 1) {
    err = notify_resource_state_change(skb, seq, state_change.resource,
    NOTIFY_EXISTS | flags);
    goto next;
    }
    n--;
    if (n < state_change.n_connections) {
    err = notify_connection_state_change(skb, seq, &state_change.connections[n],
    NOTIFY_EXISTS | flags);
    goto next;
    }
    n -= state_change.n_connections;
    if (n < state_change.n_devices) {
    err = notify_device_state_change(skb, seq, &state_change.devices[n],
    NOTIFY_EXISTS | flags);
    goto next;
    }
    n -= state_change.n_devices;
    if (n < state_change.n_devices * state_change.n_connections) {
    err = notify_peer_device_state_change(skb, seq, &state_change.peer_devices[n],
    NOTIFY_EXISTS | flags);
    goto next;
    }
    next:
    if (cb.args[4] == cb.args[3]) {
    struct drbd_state_change *next_state_change =
    list_entry(state_change.list.next,
    struct drbd_state_change, list);
    cb.args[0] = (long)next_state_change;
    cb.args[3] = notifications_for_state_change(next_state_change);
    cb.args[4] = 0;
    }
    out:
    if (err)
    return err;
    else
    return skb.len;
    }
#[no_mangle]
pub unsafe extern "C" fn drbd_nl_get_initial_state_dumpit(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int {
    int drbd_nl_get_initial_state_dumpit(struct sk_buff *skb, struct netlink_callback *cb)
    {
    struct drbd_resource *resource;
    LIST_HEAD(head);
    if (cb.args[5] >= 1) {
    if (cb.args[5] > 1)
    return get_initial_state(skb, cb);
    if (cb.args[0]) {
    struct drbd_state_change *state_change =
    (struct drbd_state_change *)cb.args[0];
// connect list to head
    list_add(&head, &state_change.list);
    free_state_changes(&head);
    }
    return 0;
    }
    cb.args[5] = 2;  /* number of iterations */
    mutex_lock(&resources_mutex);
    for_each_resource(resource, &drbd_resources) {
    struct drbd_state_change *state_change;
    state_change = remember_old_state(resource, GFP_KERNEL);
    if (!state_change) {
    if (!list_empty(&head))
    free_state_changes(&head);
    mutex_unlock(&resources_mutex);
    return -ENOMEM;
    }
    copy_old_to_new_state_change(state_change);
    list_add_tail(&state_change.list, &head);
    cb.args[5] += notifications_for_state_change(state_change);
    }
    mutex_unlock(&resources_mutex);
    if (!list_empty(&head)) {
    struct drbd_state_change *state_change =
    list_entry(head.next, struct drbd_state_change, list);
    cb.args[0] = (long)state_change;
    cb.args[3] = notifications_for_state_change(state_change);
    list_del(&head);  /* detach list from head */
    }
    cb.args[2] = cb.nlh.nlmsg_seq;
    return get_initial_state(skb, cb);
    }
    static const struct genl_multicast_group drbd_nl_mcgrps[] = {
    [DRBD_NLGRP_EVENTS] = { .name = "events", },
    };
    struct genl_family drbd_nl_family __ro_after_init = {
    .name		= "drbd",
    .version	= DRBD_FAMILY_VERSION,
    .hdrsize	= NLA_ALIGN(sizeof(struct drbd_genlmsghdr)),
    .split_ops	= drbd_nl_ops,
    .n_split_ops	= ARRAY_SIZE(drbd_nl_ops),
    .mcgrps		= drbd_nl_mcgrps,
    .n_mcgrps	= ARRAY_SIZE(drbd_nl_mcgrps),
    .resv_start_op	= 42,
    .module		= THIS_MODULE,
    .netnsok	= true,
    };
