//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/marvell/libertas_tf/cmd.c
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
// Copyright (C) 2008, cozybit Inc.
// Copyright (C) 2003-2006, Marvell International Ltd.
//

    static const struct channel_range channel_ranges[] = {
    { LBTF_REGDOMAIN_US,		1, 12 },
    { LBTF_REGDOMAIN_CA,		1, 12 },
    { LBTF_REGDOMAIN_EU,		1, 14 },
    { LBTF_REGDOMAIN_JP,		1, 14 },
    { LBTF_REGDOMAIN_SP,		1, 14 },
    { LBTF_REGDOMAIN_FR,		1, 14 },
    };
    static u16 lbtf_region_code_to_index[MRVDRV_MAX_REGION_CODE] =
    {
    LBTF_REGDOMAIN_US, LBTF_REGDOMAIN_CA, LBTF_REGDOMAIN_EU,
    LBTF_REGDOMAIN_SP, LBTF_REGDOMAIN_FR, LBTF_REGDOMAIN_JP,
    };
    static struct cmd_ctrl_node *lbtf_get_cmd_ctrl_node(struct lbtf_private *priv);
//
// lbtf_cmd_copyback - Simple callback that copies response back into command
//
// @priv:	A pointer to struct lbtf_private structure
// @extra:	A pointer to the original command structure for which
// 'resp' is a response
// @resp:	A pointer to the command response
//
// Returns: 0 on success, error on failure
//
    int lbtf_cmd_copyback(struct lbtf_private *priv, unsigned long extra,
    struct cmd_header *resp)
    {
    struct cmd_header *buf = (void *)extra;
    uint16_t copy_len;
    copy_len = min(le16_to_cpu(buf.size), le16_to_cpu(resp.size));
    memcpy(buf, resp, copy_len);
    return 0;
    }
    EXPORT_SYMBOL_GPL(lbtf_cmd_copyback);

#[no_mangle]
unsafe extern "C" fn lbtf_geo_init(priv: *mut lbtf_private) {
    static void lbtf_geo_init(struct lbtf_private *priv)
    {
    const struct channel_range *range = channel_ranges;
    u8 ch;
    int i;
    for (i = 0; i < ARRAY_SIZE(channel_ranges); i++)
    if (channel_ranges[i].regdomain == priv.regioncode) {
    range = &channel_ranges[i];
    break;
    }
    for (ch = range.start; ch < range.end; ch++)
    priv.channels[CHAN_TO_IDX(ch)].flags = 0;
    }
//
// lbtf_update_hw_spec: Updates the hardware details.
//
// @priv:    	A pointer to struct lbtf_private structure
//
// Returns: 0 on success, error on failure
//
#[no_mangle]
pub unsafe extern "C" fn lbtf_update_hw_spec(priv: *mut lbtf_private) -> c_int {
    int lbtf_update_hw_spec(struct lbtf_private *priv)
    {
    struct cmd_ds_get_hw_spec cmd;
    let mut ret: c_int = -1;
    u32 i;
    lbtf_deb_enter(LBTF_DEB_CMD);
    memset(&cmd, 0, sizeof(cmd));
    cmd.hdr.size = cpu_to_le16(sizeof(cmd));
    memcpy(cmd.permanentaddr, priv.current_addr, ETH_ALEN);
    ret = lbtf_cmd_with_response(priv, CMD_GET_HW_SPEC, &cmd);
    if (ret)
    goto out;
    priv.fwcapinfo = le32_to_cpu(cmd.fwcapinfo);
// The firmware release is in an interesting format: the patch
// level is in the most significant nibble ... so fix that:
    priv.fwrelease = le32_to_cpu(cmd.fwrelease);
    priv.fwrelease = (priv.fwrelease << 8) |
    (priv.fwrelease >> 24 & 0xff);
    printk(KERN_INFO "libertastf: %pM, fw %u.%u.%up%u, cap 0x%08x\n",
    cmd.permanentaddr,
    priv.fwrelease >> 24 & 0xff,
    priv.fwrelease >> 16 & 0xff,
    priv.fwrelease >>  8 & 0xff,
    priv.fwrelease       & 0xff,
    priv.fwcapinfo);
    lbtf_deb_cmd("GET_HW_SPEC: hardware interface 0x%x, hardware spec 0x%04x\n",
    cmd.hwifversion, cmd.version);
// Clamp region code to 8-bit since FW spec indicates that it should
// only ever be 8-bit, even though the field size is 16-bit.  Some
// firmware returns non-zero high 8 bits here.
//
    priv.regioncode = le16_to_cpu(cmd.regioncode) & 0xFF;
    for (i = 0; i < MRVDRV_MAX_REGION_CODE; i++) {
// use the region code to search for the index
    if (priv.regioncode == lbtf_region_code_to_index[i])
    break;
    }
// if it's unidentified region code, use the default (USA)
    if (i >= MRVDRV_MAX_REGION_CODE) {
    priv.regioncode = 0x10;
    pr_info("unidentified region code; using the default (USA)\n");
    }
    if (priv.current_addr[0] == 0xff)
    memmove(priv.current_addr, cmd.permanentaddr, ETH_ALEN);
    SET_IEEE80211_PERM_ADDR(priv.hw, priv.current_addr);
    lbtf_geo_init(priv);
    out:
    lbtf_deb_leave(LBTF_DEB_CMD);
    return ret;
    }
//
// lbtf_set_channel: Set the radio channel
//
// @priv:	A pointer to struct lbtf_private structure
// @channel:	The desired channel, or 0 to clear a locked channel
//
// Returns: 0 on success, error on failure
//
#[no_mangle]
pub unsafe extern "C" fn lbtf_set_channel(priv: *mut lbtf_private, channel: u8) -> c_int {
    int lbtf_set_channel(struct lbtf_private *priv, u8 channel)
    {
    let mut ret: c_int = 0;
    struct cmd_ds_802_11_rf_channel cmd;
    lbtf_deb_enter(LBTF_DEB_CMD);
    cmd.hdr.size = cpu_to_le16(sizeof(cmd));
    cmd.action = cpu_to_le16(CMD_OPT_802_11_RF_CHANNEL_SET);
    cmd.channel = cpu_to_le16(channel);
    ret = lbtf_cmd_with_response(priv, CMD_802_11_RF_CHANNEL, &cmd);
    lbtf_deb_leave_args(LBTF_DEB_CMD, "ret %d", ret);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn lbtf_beacon_set(priv: *mut lbtf_private, beacon: *mut sk_buff) -> c_int {
    int lbtf_beacon_set(struct lbtf_private *priv, struct sk_buff *beacon)
    {
    struct cmd_ds_802_11_beacon_set cmd;
    int size;
    lbtf_deb_enter(LBTF_DEB_CMD);
    if (beacon.len > MRVL_MAX_BCN_SIZE) {
    lbtf_deb_leave_args(LBTF_DEB_CMD, "ret %d", -1);
    return -1;
    }
    size =  sizeof(cmd) - sizeof(cmd.beacon) + beacon.len;
    cmd.hdr.size = cpu_to_le16(size);
    cmd.len = cpu_to_le16(beacon.len);
    memcpy(cmd.beacon, (u8 *) beacon.data, beacon.len);
    lbtf_cmd_async(priv, CMD_802_11_BEACON_SET, &cmd.hdr, size);
    lbtf_deb_leave_args(LBTF_DEB_CMD, "ret %d", 0);
    return 0;
    }
    int lbtf_beacon_ctrl(struct lbtf_private *priv, bool beacon_enable,
    int beacon_int)
    {
    struct cmd_ds_802_11_beacon_control cmd;
    lbtf_deb_enter(LBTF_DEB_CMD);
    cmd.hdr.size = cpu_to_le16(sizeof(cmd));
    cmd.action = cpu_to_le16(CMD_ACT_SET);
    cmd.beacon_enable = cpu_to_le16(beacon_enable);
    cmd.beacon_period = cpu_to_le16(beacon_int);
    lbtf_cmd_async(priv, CMD_802_11_BEACON_CTRL, &cmd.hdr, sizeof(cmd));
    lbtf_deb_leave(LBTF_DEB_CMD);
    return 0;
    }
    static void lbtf_queue_cmd(struct lbtf_private *priv,
    struct cmd_ctrl_node *cmdnode)
    {
    unsigned long flags;
    lbtf_deb_enter(LBTF_DEB_HOST);
    if (!cmdnode) {
    lbtf_deb_host("QUEUE_CMD: cmdnode is core::ptr::null_mut()\n");
    goto qcmd_done;
    }
    if (!cmdnode.cmdbuf.size) {
    lbtf_deb_host("DNLD_CMD: cmd size is zero\n");
    goto qcmd_done;
    }
    cmdnode.result = 0;
    spin_lock_irqsave(&priv.driver_lock, flags);
    list_add_tail(&cmdnode.list, &priv.cmdpendingq);
    spin_unlock_irqrestore(&priv.driver_lock, flags);
    lbtf_deb_host("QUEUE_CMD: inserted command 0x%04x into cmdpendingq\n",
    le16_to_cpu(cmdnode.cmdbuf.command));
    qcmd_done:
    lbtf_deb_leave(LBTF_DEB_HOST);
    }
    static void lbtf_submit_command(struct lbtf_private *priv,
    struct cmd_ctrl_node *cmdnode)
    {
    unsigned long flags;
    struct cmd_header *cmd;
    uint16_t cmdsize;
    uint16_t command;
    let mut timeo: c_int = 5 * HZ;
    int ret;
    lbtf_deb_enter(LBTF_DEB_HOST);
    cmd = cmdnode.cmdbuf;
    spin_lock_irqsave(&priv.driver_lock, flags);
    priv.cur_cmd = cmdnode;
    cmdsize = le16_to_cpu(cmd.size);
    command = le16_to_cpu(cmd.command);
    lbtf_deb_cmd("DNLD_CMD: command 0x%04x, seq %d, size %d\n",
    command, le16_to_cpu(cmd.seqnum), cmdsize);
    lbtf_deb_hex(LBTF_DEB_CMD, "DNLD_CMD", (void *) cmdnode.cmdbuf, cmdsize);
    ret = priv.ops.hw_host_to_card(priv, MVMS_CMD, (u8 *)cmd, cmdsize);
    spin_unlock_irqrestore(&priv.driver_lock, flags);
    if (ret) {
    pr_info("DNLD_CMD: hw_host_to_card failed: %d\n", ret);
// Let the timer kick in and retry, and potentially reset
    the whole thing if the condition persists */
    timeo = HZ;
    }
// Setup the timer after transmit command
    mod_timer(&priv.command_timer, jiffies + timeo);
    lbtf_deb_leave(LBTF_DEB_HOST);
    }
//
// This function inserts command node to cmdfreeq
// after cleans it. Requires priv->driver_lock held.
//
    static void __lbtf_cleanup_and_insert_cmd(struct lbtf_private *priv,
    struct cmd_ctrl_node *cmdnode)
    {
    lbtf_deb_enter(LBTF_DEB_HOST);
    if (!cmdnode)
    goto cl_ins_out;
    cmdnode.callback = core::ptr::null_mut();
    cmdnode.callback_arg = 0;
    memset(cmdnode.cmdbuf, 0, LBS_CMD_BUFFER_SIZE);
    list_add_tail(&cmdnode.list, &priv.cmdfreeq);
    cl_ins_out:
    lbtf_deb_leave(LBTF_DEB_HOST);
    }
    static void lbtf_cleanup_and_insert_cmd(struct lbtf_private *priv,
    struct cmd_ctrl_node *ptempcmd)
    {
    unsigned long flags;
    spin_lock_irqsave(&priv.driver_lock, flags);
    __lbtf_cleanup_and_insert_cmd(priv, ptempcmd);
    spin_unlock_irqrestore(&priv.driver_lock, flags);
    }
    void lbtf_complete_command(struct lbtf_private *priv, struct cmd_ctrl_node *cmd,
    int result)
    {
    cmd.result = result;
    cmd.cmdwaitqwoken = 1;
    wake_up_interruptible(&cmd.cmdwait_q);
    if (!cmd.callback)
    __lbtf_cleanup_and_insert_cmd(priv, cmd);
    priv.cur_cmd = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn lbtf_cmd_set_mac_multicast_addr(priv: *mut lbtf_private) -> c_int {
    int lbtf_cmd_set_mac_multicast_addr(struct lbtf_private *priv)
    {
    struct cmd_ds_mac_multicast_addr cmd;
    lbtf_deb_enter(LBTF_DEB_CMD);
    cmd.hdr.size = cpu_to_le16(sizeof(cmd));
    cmd.action = cpu_to_le16(CMD_ACT_SET);
    cmd.nr_of_adrs = cpu_to_le16((u16) priv.nr_of_multicastmacaddr);
    lbtf_deb_cmd("MULTICAST_ADR: setting %d addresses\n", cmd.nr_of_adrs);
    memcpy(cmd.maclist, priv.multicastlist,
    priv.nr_of_multicastmacaddr * ETH_ALEN);
    lbtf_cmd_async(priv, CMD_MAC_MULTICAST_ADR, &cmd.hdr, sizeof(cmd));
    lbtf_deb_leave(LBTF_DEB_CMD);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn lbtf_set_mode(priv: *mut lbtf_private, mode: enum lbtf_mode) {
    void lbtf_set_mode(struct lbtf_private *priv, enum lbtf_mode mode)
    {
    struct cmd_ds_set_mode cmd;
    lbtf_deb_enter(LBTF_DEB_WEXT);
    cmd.hdr.size = cpu_to_le16(sizeof(cmd));
    cmd.mode = cpu_to_le16(mode);
    lbtf_deb_wext("Switching to mode: 0x%x\n", mode);
    lbtf_cmd_async(priv, CMD_802_11_SET_MODE, &cmd.hdr, sizeof(cmd));
    lbtf_deb_leave(LBTF_DEB_WEXT);
    }
#[no_mangle]
pub unsafe extern "C" fn lbtf_set_bssid(priv: *mut lbtf_private, activate: bool, bssid: *const u8) {
    void lbtf_set_bssid(struct lbtf_private *priv, bool activate, const u8 *bssid)
    {
    struct cmd_ds_set_bssid cmd;
    lbtf_deb_enter(LBTF_DEB_CMD);
    cmd.hdr.size = cpu_to_le16(sizeof(cmd));
    cmd.activate = activate ? 1 : 0;
    if (activate)
    memcpy(cmd.bssid, bssid, ETH_ALEN);
    lbtf_cmd_async(priv, CMD_802_11_SET_BSSID, &cmd.hdr, sizeof(cmd));
    lbtf_deb_leave(LBTF_DEB_CMD);
    }
#[no_mangle]
pub unsafe extern "C" fn lbtf_set_mac_address(priv: *mut lbtf_private, mac_addr: *mut u8) -> c_int {
    int lbtf_set_mac_address(struct lbtf_private *priv, uint8_t *mac_addr)
    {
    struct cmd_ds_802_11_mac_address cmd;
    lbtf_deb_enter(LBTF_DEB_CMD);
    cmd.hdr.size = cpu_to_le16(sizeof(cmd));
    cmd.action = cpu_to_le16(CMD_ACT_SET);
    memcpy(cmd.macadd, mac_addr, ETH_ALEN);
    lbtf_cmd_async(priv, CMD_802_11_MAC_ADDRESS, &cmd.hdr, sizeof(cmd));
    lbtf_deb_leave(LBTF_DEB_CMD);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn lbtf_set_radio_control(priv: *mut lbtf_private) -> c_int {
    int lbtf_set_radio_control(struct lbtf_private *priv)
    {
    let mut ret: c_int = 0;
    struct cmd_ds_802_11_radio_control cmd;
    lbtf_deb_enter(LBTF_DEB_CMD);
    cmd.hdr.size = cpu_to_le16(sizeof(cmd));
    cmd.action = cpu_to_le16(CMD_ACT_SET);
    switch (priv.preamble) {
    case CMD_TYPE_SHORT_PREAMBLE:
    cmd.control = cpu_to_le16(SET_SHORT_PREAMBLE);
    break;
    case CMD_TYPE_LONG_PREAMBLE:
    cmd.control = cpu_to_le16(SET_LONG_PREAMBLE);
    break;
    case CMD_TYPE_AUTO_PREAMBLE:
    default:
    cmd.control = cpu_to_le16(SET_AUTO_PREAMBLE);
    break;
    }
    if (priv.radioon)
    cmd.control |= cpu_to_le16(TURN_ON_RF);
    else
    cmd.control &= cpu_to_le16(~TURN_ON_RF);
    lbtf_deb_cmd("RADIO_SET: radio %d, preamble %d\n", priv.radioon,
    priv.preamble);
    ret = lbtf_cmd_with_response(priv, CMD_802_11_RADIO_CONTROL, &cmd);
    lbtf_deb_leave_args(LBTF_DEB_CMD, "ret %d", ret);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn lbtf_set_mac_control(priv: *mut lbtf_private) {
    void lbtf_set_mac_control(struct lbtf_private *priv)
    {
    struct cmd_ds_mac_control cmd;
    lbtf_deb_enter(LBTF_DEB_CMD);
    cmd.hdr.size = cpu_to_le16(sizeof(cmd));
    cmd.action = cpu_to_le16(priv.mac_control);
    cmd.reserved = 0;
    lbtf_cmd_async(priv, CMD_MAC_CONTROL,
    &cmd.hdr, sizeof(cmd));
    lbtf_deb_leave(LBTF_DEB_CMD);
    }
//
// lbtf_allocate_cmd_buffer - Allocates cmd buffer, links it to free cmd queue
//
// @priv:	A pointer to struct lbtf_private structure
//
// Returns: 0 on success.
//
#[no_mangle]
pub unsafe extern "C" fn lbtf_allocate_cmd_buffer(priv: *mut lbtf_private) -> c_int {
    int lbtf_allocate_cmd_buffer(struct lbtf_private *priv)
    {
    let mut ret: c_int = 0;
    u32 bufsize;
    u32 i;
    struct cmd_ctrl_node *cmdarray;
    lbtf_deb_enter(LBTF_DEB_HOST);
// Allocate and initialize the command array
    bufsize = sizeof(struct cmd_ctrl_node) * LBS_NUM_CMD_BUFFERS;
    cmdarray = kzalloc(bufsize, GFP_KERNEL);
    if (!cmdarray) {
    lbtf_deb_host("ALLOC_CMD_BUF: tempcmd_array is core::ptr::null_mut()\n");
    ret = -1;
    goto done;
    }
    priv.cmd_array = cmdarray;
// Allocate and initialize each command buffer in the command array
    for (i = 0; i < LBS_NUM_CMD_BUFFERS; i++) {
    cmdarray[i].cmdbuf = kzalloc(LBS_CMD_BUFFER_SIZE, GFP_KERNEL);
    if (!cmdarray[i].cmdbuf) {
    lbtf_deb_host("ALLOC_CMD_BUF: ptempvirtualaddr is core::ptr::null_mut()\n");
    ret = -1;
    goto done;
    }
    }
    for (i = 0; i < LBS_NUM_CMD_BUFFERS; i++) {
    init_waitqueue_head(&cmdarray[i].cmdwait_q);
    lbtf_cleanup_and_insert_cmd(priv, &cmdarray[i]);
    }
    ret = 0;
    done:
    lbtf_deb_leave_args(LBTF_DEB_HOST, "ret %d", ret);
    return ret;
    }
//
// lbtf_free_cmd_buffer - Frees the cmd buffer.
//
// @priv:	A pointer to struct lbtf_private structure
//
// Returns: 0
//
#[no_mangle]
pub unsafe extern "C" fn lbtf_free_cmd_buffer(priv: *mut lbtf_private) -> c_int {
    int lbtf_free_cmd_buffer(struct lbtf_private *priv)
    {
    struct cmd_ctrl_node *cmdarray;
    unsigned int i;
    lbtf_deb_enter(LBTF_DEB_HOST);
// need to check if cmd array is allocated or not
    if (priv.cmd_array == core::ptr::null_mut()) {
    lbtf_deb_host("FREE_CMD_BUF: cmd_array is core::ptr::null_mut()\n");
    goto done;
    }
    cmdarray = priv.cmd_array;
// Release shared memory buffers
    for (i = 0; i < LBS_NUM_CMD_BUFFERS; i++) {
    kfree(cmdarray[i].cmdbuf);
    cmdarray[i].cmdbuf = core::ptr::null_mut();
    }
// Release cmd_ctrl_node
    kfree(priv.cmd_array);
    priv.cmd_array = core::ptr::null_mut();
    done:
    lbtf_deb_leave(LBTF_DEB_HOST);
    return 0;
    }
//
// lbtf_get_cmd_ctrl_node - Gets free cmd node from free cmd queue.
//
// @priv:		A pointer to struct lbtf_private structure
//
// Returns: pointer to a struct cmd_ctrl_node or NULL if none available.
//
    static struct cmd_ctrl_node *lbtf_get_cmd_ctrl_node(struct lbtf_private *priv)
    {
    struct cmd_ctrl_node *tempnode;
    unsigned long flags;
    lbtf_deb_enter(LBTF_DEB_HOST);
    if (!priv)
    return core::ptr::null_mut();
    spin_lock_irqsave(&priv.driver_lock, flags);
    if (!list_empty(&priv.cmdfreeq)) {
    tempnode = list_first_entry(&priv.cmdfreeq,
    struct cmd_ctrl_node, list);
    list_del(&tempnode.list);
    } else {
    lbtf_deb_host("GET_CMD_NODE: cmd_ctrl_node is not available\n");
    tempnode = core::ptr::null_mut();
    }
    spin_unlock_irqrestore(&priv.driver_lock, flags);
    lbtf_deb_leave(LBTF_DEB_HOST);
    return tempnode;
    }
//
// lbtf_execute_next_command: execute next command in cmd pending queue.
//
// @priv:     A pointer to struct lbtf_private structure
//
// Returns: 0 on success.
//
#[no_mangle]
pub unsafe extern "C" fn lbtf_execute_next_command(priv: *mut lbtf_private) -> c_int {
    int lbtf_execute_next_command(struct lbtf_private *priv)
    {
    struct cmd_ctrl_node *cmdnode = core::ptr::null_mut();
    struct cmd_header *cmd;
    unsigned long flags;
    let mut ret: c_int = 0;
// Debug group is lbtf_deb_THREAD and not lbtf_deb_HOST, because the
// only caller to us is lbtf_thread() and we get even when a
// data packet is received
    lbtf_deb_enter(LBTF_DEB_THREAD);
    spin_lock_irqsave(&priv.driver_lock, flags);
    if (priv.cur_cmd) {
    pr_alert("EXEC_NEXT_CMD: already processing command!\n");
    spin_unlock_irqrestore(&priv.driver_lock, flags);
    ret = -1;
    goto done;
    }
    if (!list_empty(&priv.cmdpendingq)) {
    cmdnode = list_first_entry(&priv.cmdpendingq,
    struct cmd_ctrl_node, list);
    }
    if (cmdnode) {
    cmd = cmdnode.cmdbuf;
    list_del(&cmdnode.list);
    lbtf_deb_host("EXEC_NEXT_CMD: sending command 0x%04x\n",
    le16_to_cpu(cmd.command));
    spin_unlock_irqrestore(&priv.driver_lock, flags);
    lbtf_submit_command(priv, cmdnode);
    } else
    spin_unlock_irqrestore(&priv.driver_lock, flags);
    ret = 0;
    done:
    lbtf_deb_leave(LBTF_DEB_THREAD);
    return ret;
    }
    static struct cmd_ctrl_node *__lbtf_cmd_async(struct lbtf_private *priv,
    uint16_t command, struct cmd_header *in_cmd, int in_cmd_size,
    int (*callback)(struct lbtf_private *, unsigned long,
    struct cmd_header *),
    unsigned long callback_arg)
    {
    struct cmd_ctrl_node *cmdnode;
    lbtf_deb_enter(LBTF_DEB_HOST);
    if (priv.surpriseremoved) {
    lbtf_deb_host("PREP_CMD: card removed\n");
    cmdnode = ERR_PTR(-ENOENT);
    goto done;
    }
    cmdnode = lbtf_get_cmd_ctrl_node(priv);
    if (cmdnode == core::ptr::null_mut()) {
    lbtf_deb_host("PREP_CMD: cmdnode is core::ptr::null_mut()\n");
// Wake up main thread to execute next command
    queue_work(lbtf_wq, &priv.cmd_work);
    cmdnode = ERR_PTR(-ENOBUFS);
    goto done;
    }
    cmdnode.callback = callback;
    cmdnode.callback_arg = callback_arg;
// Copy the incoming command to the buffer
    memcpy(cmdnode.cmdbuf, in_cmd, in_cmd_size);
// Set sequence number, clean result, move to buffer
    priv.seqnum++;
    cmdnode.cmdbuf.command = cpu_to_le16(command);
    cmdnode.cmdbuf.size    = cpu_to_le16(in_cmd_size);
    cmdnode.cmdbuf.seqnum  = cpu_to_le16(priv.seqnum);
    cmdnode.cmdbuf.result  = 0;
    lbtf_deb_host("PREP_CMD: command 0x%04x\n", command);
    cmdnode.cmdwaitqwoken = 0;
    lbtf_queue_cmd(priv, cmdnode);
    queue_work(lbtf_wq, &priv.cmd_work);
    done:
    lbtf_deb_leave_args(LBTF_DEB_HOST, "ret %p", cmdnode);
    return cmdnode;
    }
    void lbtf_cmd_async(struct lbtf_private *priv, uint16_t command,
    struct cmd_header *in_cmd, int in_cmd_size)
    {
    lbtf_deb_enter(LBTF_DEB_CMD);
    __lbtf_cmd_async(priv, command, in_cmd, in_cmd_size, core::ptr::null_mut(), 0);
    lbtf_deb_leave(LBTF_DEB_CMD);
    }
    int __lbtf_cmd(struct lbtf_private *priv, uint16_t command,
    struct cmd_header *in_cmd, int in_cmd_size,
    int (*callback)(struct lbtf_private *,
    unsigned long, struct cmd_header *),
    unsigned long callback_arg)
    {
    struct cmd_ctrl_node *cmdnode;
    unsigned long flags;
    let mut ret: c_int = 0;
    lbtf_deb_enter(LBTF_DEB_HOST);
    cmdnode = __lbtf_cmd_async(priv, command, in_cmd, in_cmd_size,
    callback, callback_arg);
    if (IS_ERR(cmdnode)) {
    ret = PTR_ERR(cmdnode);
    goto done;
    }
    might_sleep();
    ret = wait_event_interruptible(cmdnode.cmdwait_q,
    cmdnode.cmdwaitqwoken);
    if (ret) {
    pr_info("PREP_CMD: command 0x%04x interrupted by signal: %d\n",
    command, ret);
    goto done;
    }
    spin_lock_irqsave(&priv.driver_lock, flags);
    ret = cmdnode.result;
    if (ret)
    pr_info("PREP_CMD: command 0x%04x failed: %d\n",
    command, ret);
    __lbtf_cleanup_and_insert_cmd(priv, cmdnode);
    spin_unlock_irqrestore(&priv.driver_lock, flags);
    done:
    lbtf_deb_leave_args(LBTF_DEB_HOST, "ret %d", ret);
    return ret;
    }
    EXPORT_SYMBOL_GPL(__lbtf_cmd);
// Call holding driver_lock
#[no_mangle]
pub unsafe extern "C" fn lbtf_cmd_response_rx(priv: *mut lbtf_private) {
    void lbtf_cmd_response_rx(struct lbtf_private *priv)
    {
    priv.cmd_response_rxed = 1;
    queue_work(lbtf_wq, &priv.cmd_work);
    }
    EXPORT_SYMBOL_GPL(lbtf_cmd_response_rx);
#[no_mangle]
pub unsafe extern "C" fn lbtf_process_rx_command(priv: *mut lbtf_private) -> c_int {
    int lbtf_process_rx_command(struct lbtf_private *priv)
    {
    uint16_t respcmd, curcmd;
    struct cmd_header *resp;
    let mut ret: c_int = 0;
    unsigned long flags;
    uint16_t result;
    lbtf_deb_enter(LBTF_DEB_CMD);
    mutex_lock(&priv.lock);
    spin_lock_irqsave(&priv.driver_lock, flags);
    if (!priv.cur_cmd) {
    ret = -1;
    spin_unlock_irqrestore(&priv.driver_lock, flags);
    goto done;
    }
    resp = (void *)priv.cmd_resp_buff;
    curcmd = le16_to_cpu(priv.cur_cmd.cmdbuf.command);
    respcmd = le16_to_cpu(resp.command);
    result = le16_to_cpu(resp.result);
    lbtf_deb_cmd("libertastf: cmd response 0x%04x, seq %d, size %d\n",
    respcmd, le16_to_cpu(resp.seqnum),
    le16_to_cpu(resp.size));
    if (resp.seqnum != priv.cur_cmd.cmdbuf.seqnum) {
    spin_unlock_irqrestore(&priv.driver_lock, flags);
    ret = -1;
    goto done;
    }
    if (respcmd != CMD_RET(curcmd)) {
    spin_unlock_irqrestore(&priv.driver_lock, flags);
    ret = -1;
    goto done;
    }
    if (resp.result == cpu_to_le16(0x0004)) {
// 0x0004 means -EAGAIN. Drop the response, let it time out
    and be resubmitted */
    spin_unlock_irqrestore(&priv.driver_lock, flags);
    ret = -1;
    goto done;
    }
// Now we got response from FW, cancel the command timer
    timer_delete(&priv.command_timer);
    priv.cmd_timed_out = 0;
    if (priv.nr_retries)
    priv.nr_retries = 0;
// If the command is not successful, cleanup and return failure
    if ((result != 0 || !(respcmd & 0x8000))) {
//
// Handling errors here
//
    switch (respcmd) {
    case CMD_RET(CMD_GET_HW_SPEC):
    case CMD_RET(CMD_802_11_RESET):
    pr_info("libertastf: reset failed\n");
    break;
    }
    lbtf_complete_command(priv, priv.cur_cmd, result);
    spin_unlock_irqrestore(&priv.driver_lock, flags);
    ret = -1;
    goto done;
    }
    spin_unlock_irqrestore(&priv.driver_lock, flags);
    if (priv.cur_cmd && priv.cur_cmd.callback) {
    ret = priv.cur_cmd.callback(priv, priv.cur_cmd.callback_arg,
    resp);
    }
    spin_lock_irqsave(&priv.driver_lock, flags);
    if (priv.cur_cmd) {
// Clean up and Put current command back to cmdfreeq
    lbtf_complete_command(priv, priv.cur_cmd, result);
    }
    spin_unlock_irqrestore(&priv.driver_lock, flags);
    done:
    mutex_unlock(&priv.lock);
    lbtf_deb_leave_args(LBTF_DEB_CMD, "ret %d", ret);
    return ret;
    }
