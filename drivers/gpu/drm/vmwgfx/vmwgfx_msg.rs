//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/vmwgfx/vmwgfx_msg.c
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
//
// Copyright 2016 VMware, Inc., Palo Alto, CA., USA
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sub license, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice (including the
// next paragraph) shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL
// THE COPYRIGHT HOLDERS, AUTHORS AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM,
// DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
// OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE
// USE OR OTHER DEALINGS IN THE SOFTWARE.
//

pub const MESSAGE_STATUS_SUCCESS: c_uint = 0x0001;
pub const MESSAGE_STATUS_DORECV: c_uint = 0x0002;
pub const MESSAGE_STATUS_CPT: c_uint = 0x0010;
pub const MESSAGE_STATUS_HB: c_uint = 0x0080;
pub const RPCI_PROTOCOL_NUM: c_uint = 0x49435052;
pub const GUESTMSG_FLAG_COOKIE: c_uint = 0x80000000;
pub const RETRIES: c_int = 3;
pub const VMW_PORT_CMD_MSG: c_int = 30;
pub const VMW_PORT_CMD_HB_MSG: c_int = 0;

pub const VMW_PORT_CMD_MKS_GUEST_STATS: c_int = 85;

    let mut vmw_msg_enabled: static u32 = 1;
    enum rpc_msg_type {
    MSG_TYPE_OPEN,
    MSG_TYPE_SENDSIZE,
    MSG_TYPE_SENDPAYLOAD,
    MSG_TYPE_RECVSIZE,
    MSG_TYPE_RECVPAYLOAD,
    MSG_TYPE_RECVSTATUS,
    MSG_TYPE_CLOSE,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rpc_channel {
    pub channel_id: u16,
    pub cookie_high: u32,
    pub cookie_low: u32,
}

// Kernel mksGuestStats counter names and desciptions; same order as enum mksstat_kern_stats_t
    static const char* const mksstat_kern_name_desc[MKSSTAT_KERN_COUNT][2] =
    {
    { "vmw_execbuf_ioctl", "vmw_execbuf_ioctl" },
    { "vmw_cotable_resize", "vmw_cotable_resize" },
    };

//
// vmw_open_channel
//
// @channel: RPC channel
// @protocol:
//
// Returns: 0 on success
//
#[no_mangle]
unsafe extern "C" fn vmw_open_channel(channel: *mut rpc_channel, protocol: c_uint) -> c_int {
    static int vmw_open_channel(struct rpc_channel *channel, unsigned int protocol)
    {
    u32 ecx, edx, esi, edi;
    vmware_hypercall6(VMW_PORT_CMD_OPEN_CHANNEL,
    (protocol | GUESTMSG_FLAG_COOKIE), 0,
    &ecx, &edx, &esi, &edi);
    if ((HIGH_WORD(ecx) & MESSAGE_STATUS_SUCCESS) == 0)
    return -EINVAL;
    channel.channel_id  = HIGH_WORD(edx);
    channel.cookie_high = esi;
    channel.cookie_low  = edi;
    return 0;
    }
//
// vmw_close_channel
//
// @channel: RPC channel
//
// Returns: 0 on success
//
#[no_mangle]
unsafe extern "C" fn vmw_close_channel(channel: *mut rpc_channel) -> c_int {
    static int vmw_close_channel(struct rpc_channel *channel)
    {
    u32 ecx;
    vmware_hypercall5(VMW_PORT_CMD_CLOSE_CHANNEL,
    0, channel.channel_id << 16,
    channel.cookie_high,
    channel.cookie_low,
    &ecx);
    if ((HIGH_WORD(ecx) & MESSAGE_STATUS_SUCCESS) == 0)
    return -EINVAL;
    return 0;
    }
//
// vmw_port_hb_out - Send the message payload either through the
// high-bandwidth port if available, or through the backdoor otherwise.
// @channel: The rpc channel.
// @msg: NULL-terminated message.
// @hb: Whether the high-bandwidth port is available.
//
// Return: The port status.
//
    static unsigned long vmw_port_hb_out(struct rpc_channel *channel,
    const char *msg, bool hb)
    {
    u32 ebx, ecx;
    let mut msg_len: c_ulong = strlen(msg);
// HB port can't access encrypted memory.
    if (hb && !cc_platform_has(CC_ATTR_MEM_ENCRYPT)) {
    vmware_hypercall_hb_out(
    (MESSAGE_STATUS_SUCCESS << 16) | VMW_PORT_CMD_HB_MSG,
    msg_len,
    channel.channel_id << 16,
    (uintptr_t) msg, channel.cookie_low,
    channel.cookie_high,
    &ebx);
    return ebx;
    }
// HB port not available. Send the message 4 bytes at a time.
    ecx = MESSAGE_STATUS_SUCCESS << 16;
    while (msg_len && (HIGH_WORD(ecx) & MESSAGE_STATUS_SUCCESS)) {
    let mut bytes: c_uint = min_t(size_t, msg_len, 4);
    let mut word: c_ulong = 0;
    memcpy(&word, msg, bytes);
    msg_len -= bytes;
    msg += bytes;
    vmware_hypercall5(VMW_PORT_CMD_MSG |
    (MSG_TYPE_SENDPAYLOAD << 16),
    word, channel.channel_id << 16,
    channel.cookie_high,
    channel.cookie_low,
    &ecx);
    }
    return ecx;
    }
//
// vmw_port_hb_in - Receive the message payload either through the
// high-bandwidth port if available, or through the backdoor otherwise.
// @channel: The rpc channel.
// @reply: Pointer to buffer holding reply.
// @reply_len: Length of the reply.
// @hb: Whether the high-bandwidth port is available.
//
// Return: The port status.
//
    static unsigned long vmw_port_hb_in(struct rpc_channel *channel, char *reply,
    unsigned long reply_len, bool hb)
    {
    u32 ebx, ecx, edx;
// HB port can't access encrypted memory
    if (hb && !cc_platform_has(CC_ATTR_MEM_ENCRYPT)) {
    vmware_hypercall_hb_in(
    (MESSAGE_STATUS_SUCCESS << 16) | VMW_PORT_CMD_HB_MSG,
    reply_len,
    channel.channel_id << 16,
    channel.cookie_high,
    (uintptr_t) reply, channel.cookie_low,
    &ebx);
    return ebx;
    }
// HB port not available. Retrieve the message 4 bytes at a time.
    ecx = MESSAGE_STATUS_SUCCESS << 16;
    while (reply_len) {
    let mut bytes: c_uint = min_t(unsigned long, reply_len, 4);
    vmware_hypercall7(VMW_PORT_CMD_MSG |
    (MSG_TYPE_RECVPAYLOAD << 16),
    MESSAGE_STATUS_SUCCESS,
    channel.channel_id << 16,
    channel.cookie_high,
    channel.cookie_low,
    &ebx, &ecx, &edx);
    if ((HIGH_WORD(ecx) & MESSAGE_STATUS_SUCCESS) == 0)
    break;
    memcpy(reply, &ebx, bytes);
    reply_len -= bytes;
    reply += bytes;
    }
    return ecx;
    }
//
// vmw_send_msg: Sends a message to the host
//
// @channel: RPC channel
// @msg: NULL terminated string
//
// Returns: 0 on success
//
#[no_mangle]
unsafe extern "C" fn vmw_send_msg(channel: *mut rpc_channel, msg: *const c_char) -> c_int {
    static int vmw_send_msg(struct rpc_channel *channel, const char *msg)
    {
    u32 ebx, ecx;
    let mut msg_len: usize = strlen(msg);
    let mut retries: c_int = 0;
    while (retries < RETRIES) {
    retries++;
    vmware_hypercall5(VMW_PORT_CMD_SENDSIZE,
    msg_len, channel.channel_id << 16,
    channel.cookie_high,
    channel.cookie_low,
    &ecx);
    if ((HIGH_WORD(ecx) & MESSAGE_STATUS_SUCCESS) == 0) {
// Expected success. Give up.
    return -EINVAL;
    }
// Send msg
    ebx = vmw_port_hb_out(channel, msg,
    !!(HIGH_WORD(ecx) & MESSAGE_STATUS_HB));
    if ((HIGH_WORD(ebx) & MESSAGE_STATUS_SUCCESS) != 0) {
    return 0;
    } else if ((HIGH_WORD(ebx) & MESSAGE_STATUS_CPT) != 0) {
// A checkpoint occurred. Retry.
    continue;
    } else {
    break;
    }
    }
    return -EINVAL;
    }
    STACK_FRAME_NON_STANDARD_FP(vmw_send_msg);
//
// vmw_recv_msg: Receives a message from the host
//
// Note:  It is the caller's responsibility to call kfree() on msg.
//
// @channel:  channel opened by vmw_open_channel
// @msg:  [OUT] message received from the host
// @msg_len: message length
//
    static int vmw_recv_msg(struct rpc_channel *channel, void **msg,
    size_t *msg_len)
    {
    u32 ebx, ecx, edx;
    char *reply;
    size_t reply_len;
    let mut retries: c_int = 0;
// msg_len = 0;
// msg = NULL;
    while (retries < RETRIES) {
    retries++;
    vmware_hypercall7(VMW_PORT_CMD_RECVSIZE,
    0, channel.channel_id << 16,
    channel.cookie_high,
    channel.cookie_low,
    &ebx, &ecx, &edx);
    if ((HIGH_WORD(ecx) & MESSAGE_STATUS_SUCCESS) == 0) {
    DRM_ERROR("Failed to get reply size for host message.\n");
    return -EINVAL;
    }
// No reply available.  This is okay.
    if ((HIGH_WORD(ecx) & MESSAGE_STATUS_DORECV) == 0)
    return 0;
    reply_len = ebx;
    reply     = kzalloc(reply_len + 1, GFP_KERNEL);
    if (!reply) {
    DRM_ERROR("Cannot allocate memory for host message reply.\n");
    return -ENOMEM;
    }
// Receive buffer
    ebx = vmw_port_hb_in(channel, reply, reply_len,
    !!(HIGH_WORD(ecx) & MESSAGE_STATUS_HB));
    if ((HIGH_WORD(ebx) & MESSAGE_STATUS_SUCCESS) == 0) {
    kfree(reply);
    reply = core::ptr::null_mut();
    if ((HIGH_WORD(ebx) & MESSAGE_STATUS_CPT) != 0) {
// A checkpoint occurred. Retry.
    continue;
    }
    return -EINVAL;
    }
    reply[reply_len] = '\0';
    vmware_hypercall5(VMW_PORT_CMD_RECVSTATUS,
    MESSAGE_STATUS_SUCCESS,
    channel.channel_id << 16,
    channel.cookie_high,
    channel.cookie_low,
    &ecx);
    if ((HIGH_WORD(ecx) & MESSAGE_STATUS_SUCCESS) == 0) {
    kfree(reply);
    reply = core::ptr::null_mut();
    if ((HIGH_WORD(ecx) & MESSAGE_STATUS_CPT) != 0) {
// A checkpoint occurred. Retry.
    continue;
    }
    return -EINVAL;
    }
    break;
    }
    if (!reply)
    return -EINVAL;
// msg_len = reply_len;
// msg     = reply;
    return 0;
    }
    STACK_FRAME_NON_STANDARD(vmw_recv_msg);
//
// vmw_host_get_guestinfo: Gets a GuestInfo parameter
//
// Gets the value of a  GuestInfo.* parameter.  The value returned will be in
// a string, and it is up to the caller to post-process.
//
// @guest_info_param:  Parameter to get, e.g. GuestInfo.svga.gl3
// @buffer: if NULL, *reply_len will contain reply size.
// @length: size of the reply_buf.  Set to size of reply upon return
//
// Returns: 0 on success
//
    int vmw_host_get_guestinfo(const char *guest_info_param,
    char *buffer, size_t *length)
    {
    struct rpc_channel channel;
    char *msg, *reply = core::ptr::null_mut();
    let mut reply_len: usize = 0;
    if (!vmw_msg_enabled)
    return -ENODEV;
    if (!guest_info_param || !length)
    return -EINVAL;
    msg = kasprintf(GFP_KERNEL, "info-get %s", guest_info_param);
    if (!msg) {
    DRM_ERROR("Cannot allocate memory to get guest info \"%s\".",
    guest_info_param);
    return -ENOMEM;
    }
    if (vmw_open_channel(&channel, RPCI_PROTOCOL_NUM))
    goto out_open;
    if (vmw_send_msg(&channel, msg) ||
    vmw_recv_msg(&channel, (void *) &reply, &reply_len))
    goto out_msg;
    vmw_close_channel(&channel);
    if (buffer && reply && reply_len > 0) {
// Remove reply code, which are the first 2 characters of
// the reply
//
    reply_len = max(reply_len - 2, (size_t) 0);
    reply_len = min(reply_len, *length);
    if (reply_len > 0)
    memcpy(buffer, reply + 2, reply_len);
    }
// length = reply_len;
    kfree(reply);
    kfree(msg);
    return 0;
    out_msg:
    vmw_close_channel(&channel);
    kfree(reply);
    out_open:
// length = 0;
    kfree(msg);
    DRM_ERROR("Failed to get guest info \"%s\".", guest_info_param);
    return -EINVAL;
    }
//
// vmw_host_printf: Sends a log message to the host
//
// @fmt: Regular printf format string and arguments
//
// Returns: 0 on success
//
    __printf(1, 2)
#[no_mangle]
pub unsafe extern "C" fn vmw_host_printf(fmt: *const c_char, ...) -> c_int {
    int vmw_host_printf(const char *fmt, ...)
    {
    va_list ap;
    struct rpc_channel channel;
    char *msg;
    char *log;
    let mut ret: c_int = 0;
    if (!vmw_msg_enabled)
    return -ENODEV;
    if (!fmt)
    return ret;
    va_start(ap, fmt);
    log = kvasprintf(GFP_KERNEL, fmt, ap);
    va_end(ap);
    if (!log) {
    DRM_ERROR("Cannot allocate memory for the log message.\n");
    return -ENOMEM;
    }
    msg = kasprintf(GFP_KERNEL, "log %s", log);
    if (!msg) {
    DRM_ERROR("Cannot allocate memory for host log message.\n");
    kfree(log);
    return -ENOMEM;
    }
    if (vmw_open_channel(&channel, RPCI_PROTOCOL_NUM))
    goto out_open;
    if (vmw_send_msg(&channel, msg))
    goto out_msg;
    vmw_close_channel(&channel);
    kfree(msg);
    kfree(log);
    return 0;
    out_msg:
    vmw_close_channel(&channel);
    out_open:
    kfree(msg);
    kfree(log);
    DRM_ERROR("Failed to send host log message.\n");
    return -EINVAL;
    }
//
// vmw_msg_ioctl: Sends and receveives a message to/from host from/to user-space
//
// Sends a message from user-space to host.
// Can also receive a result from host and return that to user-space.
//
// @dev: Identifies the drm device.
// @data: Pointer to the ioctl argument.
// @file_priv: Identifies the caller.
// Return: Zero on success, negative error code on error.
//
    int vmw_msg_ioctl(struct drm_device *dev, void *data,
    struct drm_file *file_priv)
    {
    struct drm_vmw_msg_arg *arg =
    (struct drm_vmw_msg_arg *)data;
    struct rpc_channel channel;
    char *msg;
    int length;
    msg = kmalloc(MAX_USER_MSG_LENGTH, GFP_KERNEL);
    if (!msg) {
    DRM_ERROR("Cannot allocate memory for log message.\n");
    return -ENOMEM;
    }
    length = strncpy_from_user(msg, (void __user *)((unsigned long)arg.send),
    MAX_USER_MSG_LENGTH);
    if (length < 0 || length >= MAX_USER_MSG_LENGTH) {
    DRM_ERROR("Userspace message access failure.\n");
    kfree(msg);
    return -EINVAL;
    }
    if (vmw_open_channel(&channel, RPCI_PROTOCOL_NUM)) {
    DRM_ERROR("Failed to open channel.\n");
    goto out_open;
    }
    if (vmw_send_msg(&channel, msg)) {
    DRM_ERROR("Failed to send message to host.\n");
    goto out_msg;
    }
    if (!arg.send_only) {
    char *reply = core::ptr::null_mut();
    let mut reply_len: usize = 0;
    if (vmw_recv_msg(&channel, (void *) &reply, &reply_len)) {
    DRM_ERROR("Failed to receive message from host.\n");
    goto out_msg;
    }
    if (reply && reply_len > 0) {
    if (copy_to_user((void __user *)((unsigned long)arg.receive),
    reply, reply_len)) {
    DRM_ERROR("Failed to copy message to userspace.\n");
    kfree(reply);
    goto out_msg;
    }
    arg.receive_len = (__u32)reply_len;
    }
    kfree(reply);
    }
    vmw_close_channel(&channel);
    kfree(msg);
    return 0;
    out_msg:
    vmw_close_channel(&channel);
    out_open:
    kfree(msg);
    return -EINVAL;
    }
//
// reset_ppn_array: Resets a PPN64 array to INVALID_PPN64 content
//
// @arr: Array to reset.
// @size: Array length.
//
#[no_mangle]
pub unsafe extern "C" fn reset_ppn_array(arr: *mut PPN64, size: usize) {
    static inline void reset_ppn_array(PPN64 *arr, size_t size)
    {
    size_t i;
    BUG_ON(!arr || size == 0);
    for (i = 0; i < size; ++i)
    arr[i] = INVALID_PPN64;
    }
//
// hypervisor_ppn_reset_all: Removes all mksGuestStat instance descriptors from
// the hypervisor. All related pages should be subsequently unpinned or freed.
//
#[no_mangle]
pub unsafe extern "C" fn hypervisor_ppn_reset_all() {
    static inline void hypervisor_ppn_reset_all(void)
    {
    vmware_hypercall1(VMW_PORT_CMD_MKSGS_RESET, 0);
    }
//
// hypervisor_ppn_add: Adds a single mksGuestStat instance descriptor to the
// hypervisor. Any related userspace pages should be pinned in advance.
//
// @pfn: Physical page number of the instance descriptor
//
#[no_mangle]
pub unsafe extern "C" fn hypervisor_ppn_add(pfn: PPN64) {
    static inline void hypervisor_ppn_add(PPN64 pfn)
    {
    vmware_hypercall1(VMW_PORT_CMD_MKSGS_ADD_PPN, (unsigned long)pfn);
    }
//
// hypervisor_ppn_remove: Removes a single mksGuestStat instance descriptor from
// the hypervisor. All related pages should be subsequently unpinned or freed.
//
// @pfn: Physical page number of the instance descriptor
//
#[no_mangle]
pub unsafe extern "C" fn hypervisor_ppn_remove(pfn: PPN64) {
    static inline void hypervisor_ppn_remove(PPN64 pfn)
    {
    vmware_hypercall1(VMW_PORT_CMD_MKSGS_REMOVE_PPN, (unsigned long)pfn);
    }

// Order of the total number of pages used for kernel-internal mksGuestStat; at least 2
pub const MKSSTAT_KERNEL_PAGES_ORDER: c_int = 2;
// Header to the text description of mksGuestStat instance descriptor

//
// mksstat_init_record_time: Initializes an MKSGuestStatCounterTime-based record
// for the respective mksGuestStat index.
//
// @stat_idx: Index of the MKSGuestStatCounterTime-based mksGuestStat record.
// @pstat: Pointer to array of MKSGuestStatCounterTime.
// @pinfo: Pointer to array of MKSGuestStatInfoEntry.
// @pstrs: Pointer to current end of the name/description sequence.
// Return: Pointer to the new end of the names/description sequence.
//
    static inline char *mksstat_init_record_time(mksstat_kern_stats_t stat_idx,
    MKSGuestStatCounterTime *pstat, MKSGuestStatInfoEntry *pinfo, char *pstrs)
    {
    let mut pstrd: *mut char const = pstrs + strlen(mksstat_kern_name_desc[stat_idx][0]) + 1;
    strcpy(pstrs, mksstat_kern_name_desc[stat_idx][0]);
    strcpy(pstrd, mksstat_kern_name_desc[stat_idx][1]);
    pinfo[stat_idx].name.s = pstrs;
    pinfo[stat_idx].description.s = pstrd;
    pinfo[stat_idx].flags = MKS_GUEST_STAT_FLAG_TIME;
    pinfo[stat_idx].stat.counterTime = &pstat[stat_idx];
    return pstrd + strlen(mksstat_kern_name_desc[stat_idx][1]) + 1;
    }
//
// mksstat_init_kern_id: Creates a single mksGuestStat instance descriptor and
// kernel-internal counters. Adds PFN mapping to the hypervisor.
//
// Create a single mksGuestStat instance descriptor and corresponding structures
// for all kernel-internal counters. The corresponding PFNs are mapped with the
// hypervisor.
//
// @ppage: Output pointer to page containing the instance descriptor.
// Return: Zero on success, negative error code on error.
//
#[no_mangle]
unsafe extern "C" fn mksstat_init_kern_id(ppage: *mut page) -> c_int {
    static int mksstat_init_kern_id(struct page **ppage)
    {
    MKSGuestStatInstanceDescriptor *pdesc;
    MKSGuestStatCounterTime *pstat;
    MKSGuestStatInfoEntry *pinfo;
    char *pstrs, *pstrs_acc;
// Allocate pages for the kernel-internal instance descriptor
    struct page *page = alloc_pages(GFP_KERNEL | __GFP_ZERO, MKSSTAT_KERNEL_PAGES_ORDER);
    if (!page)
    return -ENOMEM;
    pdesc = page_address(page);
    pstat = vmw_mksstat_get_kern_pstat(pdesc);
    pinfo = vmw_mksstat_get_kern_pinfo(pdesc);
    pstrs = vmw_mksstat_get_kern_pstrs(pdesc);
// Set up all kernel-internal counters and corresponding structures
    pstrs_acc = pstrs;
    pstrs_acc = mksstat_init_record_time(MKSSTAT_KERN_EXECBUF, pstat, pinfo, pstrs_acc);
    pstrs_acc = mksstat_init_record_time(MKSSTAT_KERN_COTABLE_RESIZE, pstat, pinfo, pstrs_acc);
// Add new counters above, in their order of appearance in mksstat_kern_stats_t
    BUG_ON(pstrs_acc - pstrs > PAGE_SIZE);
// Set up the kernel-internal instance descriptor
    pdesc.reservedMBZ = 0;
    pdesc.statStartVA = (uintptr_t)pstat;
    pdesc.strsStartVA = (uintptr_t)pstrs;
    pdesc.statLength = sizeof(*pstat) * MKSSTAT_KERN_COUNT;
    pdesc.infoLength = sizeof(*pinfo) * MKSSTAT_KERN_COUNT;
    pdesc.strsLength = pstrs_acc - pstrs;
    snprintf(pdesc.description, ARRAY_SIZE(pdesc.description) - 1, "%s pid=%d",
    MKSSTAT_KERNEL_DESCRIPTION, current.pid);
    pdesc.statPPNs[0] = page_to_pfn(virt_to_page(pstat));
    reset_ppn_array(pdesc.statPPNs + 1, ARRAY_SIZE(pdesc.statPPNs) - 1);
    pdesc.infoPPNs[0] = page_to_pfn(virt_to_page(pinfo));
    reset_ppn_array(pdesc.infoPPNs + 1, ARRAY_SIZE(pdesc.infoPPNs) - 1);
    pdesc.strsPPNs[0] = page_to_pfn(virt_to_page(pstrs));
    reset_ppn_array(pdesc.strsPPNs + 1, ARRAY_SIZE(pdesc.strsPPNs) - 1);
// ppage = page;
    hypervisor_ppn_add((PPN64)page_to_pfn(page));
    return 0;
    }
//
// vmw_mksstat_get_kern_slot: Acquires a slot for a single kernel-internal
// mksGuestStat instance descriptor.
//
// Find a slot for a single kernel-internal mksGuestStat instance descriptor.
// In case no such was already present, allocate a new one and set up a kernel-
// internal mksGuestStat instance descriptor for the former.
//
// @pid: Process for which a slot is sought.
// @dev_priv: Identifies the drm private device.
// Return: Non-negative slot on success, negative error code on error.
//
#[no_mangle]
pub unsafe extern "C" fn vmw_mksstat_get_kern_slot(pid: pid_t, dev_priv: *mut vmw_private) -> c_int {
    int vmw_mksstat_get_kern_slot(pid_t pid, struct vmw_private *dev_priv)
    {
    let mut base: usize = (u32)hash_32(pid, MKSSTAT_CAPACITY_LOG2);
    size_t i;
    for (i = 0; i < ARRAY_SIZE(dev_priv.mksstat_kern_pids); ++i) {
    let mut slot: usize = (i + base) % ARRAY_SIZE(dev_priv.mksstat_kern_pids);
// Check if an instance descriptor for this pid is already present
    if (pid == (pid_t)atomic_read(&dev_priv.mksstat_kern_pids[slot]))
    return (int)slot;
// Set up a new instance descriptor for this pid
    if (!atomic_cmpxchg(&dev_priv.mksstat_kern_pids[slot], 0, MKSSTAT_PID_RESERVED)) {
    let mut ret: c_int = mksstat_init_kern_id(&dev_priv.mksstat_kern_pages[slot]);
    if (!ret) {
// Reset top-timer tracking for this slot
    dev_priv.mksstat_kern_top_timer[slot] = MKSSTAT_KERN_COUNT;
    atomic_set(&dev_priv.mksstat_kern_pids[slot], pid);
    return (int)slot;
    }
    atomic_set(&dev_priv.mksstat_kern_pids[slot], 0);
    return ret;
    }
    }
    return -ENOSPC;
    }

//
// vmw_mksstat_cleanup_descriptor: Frees a single userspace-originating
// mksGuestStat instance-descriptor page and unpins all related user pages.
//
// Unpin all user pages realated to this instance descriptor and free
// the instance-descriptor page itself.
//
// @page: Page of the instance descriptor.
//
#[no_mangle]
unsafe extern "C" fn vmw_mksstat_cleanup_descriptor(page: *mut page) {
    static void vmw_mksstat_cleanup_descriptor(struct page *page)
    {
    MKSGuestStatInstanceDescriptor *pdesc = page_address(page);
    size_t i;
    for (i = 0; i < ARRAY_SIZE(pdesc.statPPNs) && pdesc.statPPNs[i] != INVALID_PPN64; ++i)
    unpin_user_page(pfn_to_page(pdesc.statPPNs[i]));
    for (i = 0; i < ARRAY_SIZE(pdesc.infoPPNs) && pdesc.infoPPNs[i] != INVALID_PPN64; ++i)
    unpin_user_page(pfn_to_page(pdesc.infoPPNs[i]));
    for (i = 0; i < ARRAY_SIZE(pdesc.strsPPNs) && pdesc.strsPPNs[i] != INVALID_PPN64; ++i)
    unpin_user_page(pfn_to_page(pdesc.strsPPNs[i]));
    __free_page(page);
    }
//
// vmw_mksstat_remove_all: Resets all mksGuestStat instance descriptors
// from the hypervisor.
//
// Discard all hypervisor PFN mappings, containing active mksGuestState instance
// descriptors, unpin the related userspace pages and free the related kernel pages.
//
// @dev_priv: Identifies the drm private device.
// Return: Zero on success, negative error code on error.
//
#[no_mangle]
pub unsafe extern "C" fn vmw_mksstat_remove_all(dev_priv: *mut vmw_private) -> c_int {
    int vmw_mksstat_remove_all(struct vmw_private *dev_priv)
    {
    let mut ret: c_int = 0;
    size_t i;
// Discard all PFN mappings with the hypervisor
    hypervisor_ppn_reset_all();
// Discard all userspace-originating instance descriptors and unpin all related pages
    for (i = 0; i < ARRAY_SIZE(dev_priv.mksstat_user_pids); ++i) {
    let mut pid0: pid_t = (pid_t)atomic_read(&dev_priv.mksstat_user_pids[i]);
    if (!pid0)
    continue;
    if (pid0 != MKSSTAT_PID_RESERVED) {
    let mut pid1: pid_t = atomic_cmpxchg(&dev_priv.mksstat_user_pids[i], pid0, MKSSTAT_PID_RESERVED);
    if (!pid1)
    continue;
    if (pid1 == pid0) {
    let mut page: *mut page const = dev_priv.mksstat_user_pages[i];
    BUG_ON(!page);
    dev_priv.mksstat_user_pages[i] = core::ptr::null_mut();
    atomic_set(&dev_priv.mksstat_user_pids[i], 0);
    vmw_mksstat_cleanup_descriptor(page);
    continue;
    }
    }
    ret = -EAGAIN;
    }

// Discard all kernel-internal instance descriptors and free all related pages
    for (i = 0; i < ARRAY_SIZE(dev_priv.mksstat_kern_pids); ++i) {
    let mut pid0: pid_t = (pid_t)atomic_read(&dev_priv.mksstat_kern_pids[i]);
    if (!pid0)
    continue;
    if (pid0 != MKSSTAT_PID_RESERVED) {
    let mut pid1: pid_t = atomic_cmpxchg(&dev_priv.mksstat_kern_pids[i], pid0, MKSSTAT_PID_RESERVED);
    if (!pid1)
    continue;
    if (pid1 == pid0) {
    let mut page: *mut page const = dev_priv.mksstat_kern_pages[i];
    BUG_ON(!page);
    dev_priv.mksstat_kern_pages[i] = core::ptr::null_mut();
    atomic_set(&dev_priv.mksstat_kern_pids[i], 0);
    __free_pages(page, MKSSTAT_KERNEL_PAGES_ORDER);
    continue;
    }
    }
    ret = -EAGAIN;
    }

    return ret;
    }
//
// vmw_mksstat_reset_ioctl: Resets all mksGuestStat instance descriptors
// from the hypervisor.
//
// Discard all hypervisor PFN mappings, containing active mksGuestStat instance
// descriptors, unpin the related userspace pages and free the related kernel pages.
//
// @dev: Identifies the drm device.
// @data: Pointer to the ioctl argument.
// @file_priv: Identifies the caller; unused.
// Return: Zero on success, negative error code on error.
//
    int vmw_mksstat_reset_ioctl(struct drm_device *dev, void *data,
    struct drm_file *file_priv)
    {
    let mut dev_priv: *mut vmw_private const = vmw_priv(dev);
    return vmw_mksstat_remove_all(dev_priv);
    }
//
// vmw_mksstat_add_ioctl: Creates a single userspace-originating mksGuestStat
// instance descriptor and registers that with the hypervisor.
//
// Create a hypervisor PFN mapping, containing a single mksGuestStat instance
// descriptor and pin the corresponding userspace pages.
//
// @dev: Identifies the drm device.
// @data: Pointer to the ioctl argument.
// @file_priv: Identifies the caller; unused.
// Return: Zero on success, negative error code on error.
//
    int vmw_mksstat_add_ioctl(struct drm_device *dev, void *data,
    struct drm_file *file_priv)
    {
    struct drm_vmw_mksstat_add_arg *arg =
    (struct drm_vmw_mksstat_add_arg *) data;
    let mut dev_priv: *mut vmw_private const = vmw_priv(dev);
    let mut num_pages_stat: usize = PFN_UP(arg.stat_len);
    let mut num_pages_info: usize = PFN_UP(arg.info_len);
    let mut num_pages_strs: usize = PFN_UP(arg.strs_len);
    long desc_len;
    long nr_pinned_stat;
    long nr_pinned_info;
    long nr_pinned_strs;
    MKSGuestStatInstanceDescriptor *pdesc;
    struct page *page = core::ptr::null_mut();
    struct page **pages_stat = core::ptr::null_mut();
    struct page **pages_info = core::ptr::null_mut();
    struct page **pages_strs = core::ptr::null_mut();
    size_t i, slot;
    let mut ret_err: c_int = -ENOMEM;
    arg.id = -1;
    if (!arg.stat || !arg.info || !arg.strs)
    return -EINVAL;
    if (!arg.stat_len || !arg.info_len || !arg.strs_len)
    return -EINVAL;
    if (!arg.description)
    return -EINVAL;
    if (num_pages_stat > ARRAY_SIZE(pdesc.statPPNs) ||
    num_pages_info > ARRAY_SIZE(pdesc.infoPPNs) ||
    num_pages_strs > ARRAY_SIZE(pdesc.strsPPNs))
    return -EINVAL;
// Find an available slot in the mksGuestStats user array and reserve it
    for (slot = 0; slot < ARRAY_SIZE(dev_priv.mksstat_user_pids); ++slot)
    if (!atomic_cmpxchg(&dev_priv.mksstat_user_pids[slot], 0, MKSSTAT_PID_RESERVED))
    break;
    if (slot == ARRAY_SIZE(dev_priv.mksstat_user_pids))
    return -ENOSPC;
    BUG_ON(dev_priv.mksstat_user_pages[slot]);
// Allocate statically-sized temp arrays for pages -- too big to keep in frame
    pages_stat = (struct page **) kmalloc_objs(*pages_stat,
    ARRAY_SIZE(pdesc.statPPNs) + ARRAY_SIZE(pdesc.infoPPNs) + ARRAY_SIZE(pdesc.strsPPNs));
    if (!pages_stat)
    goto err_nomem;
    pages_info = pages_stat + ARRAY_SIZE(pdesc.statPPNs);
    pages_strs = pages_info + ARRAY_SIZE(pdesc.infoPPNs);
// Allocate a page for the instance descriptor
    page = alloc_page(GFP_KERNEL | __GFP_ZERO);
    if (!page)
    goto err_nomem;
// Set up the instance descriptor
    pdesc = page_address(page);
    pdesc.reservedMBZ = 0;
    pdesc.statStartVA = arg.stat;
    pdesc.strsStartVA = arg.strs;
    pdesc.statLength = arg.stat_len;
    pdesc.infoLength = arg.info_len;
    pdesc.strsLength = arg.strs_len;
    desc_len = strncpy_from_user(pdesc.description, u64_to_user_ptr(arg.description),
    ARRAY_SIZE(pdesc.description) - 1);
    if (desc_len < 0) {
    ret_err = -EFAULT;
    goto err_nomem;
    }
    reset_ppn_array(pdesc.statPPNs, ARRAY_SIZE(pdesc.statPPNs));
    reset_ppn_array(pdesc.infoPPNs, ARRAY_SIZE(pdesc.infoPPNs));
    reset_ppn_array(pdesc.strsPPNs, ARRAY_SIZE(pdesc.strsPPNs));
// Pin mksGuestStat user pages and store those in the instance descriptor
    nr_pinned_stat = pin_user_pages_fast(arg.stat, num_pages_stat, FOLL_LONGTERM, pages_stat);
    if (num_pages_stat != nr_pinned_stat)
    goto err_pin_stat;
    for (i = 0; i < num_pages_stat; ++i)
    pdesc.statPPNs[i] = page_to_pfn(pages_stat[i]);
    nr_pinned_info = pin_user_pages_fast(arg.info, num_pages_info, FOLL_LONGTERM, pages_info);
    if (num_pages_info != nr_pinned_info)
    goto err_pin_info;
    for (i = 0; i < num_pages_info; ++i)
    pdesc.infoPPNs[i] = page_to_pfn(pages_info[i]);
    nr_pinned_strs = pin_user_pages_fast(arg.strs, num_pages_strs, FOLL_LONGTERM, pages_strs);
    if (num_pages_strs != nr_pinned_strs)
    goto err_pin_strs;
    for (i = 0; i < num_pages_strs; ++i)
    pdesc.strsPPNs[i] = page_to_pfn(pages_strs[i]);
// Send the descriptor to the host via a hypervisor call. The mksGuestStat
    pages will remain in use until the user requests a matching remove stats
    or a stats reset occurs. */
    hypervisor_ppn_add((PPN64)page_to_pfn(page));
    dev_priv.mksstat_user_pages[slot] = page;
    atomic_set(&dev_priv.mksstat_user_pids[slot], task_pgrp_vnr(current));
    arg.id = slot;
    DRM_DEV_INFO(dev.dev, "pid=%d arg.description='%.*s' id=%zu\n", current.pid, (int)desc_len, pdesc.description, slot);
    kfree(pages_stat);
    return 0;
    err_pin_strs:
    if (nr_pinned_strs > 0)
    unpin_user_pages(pages_strs, nr_pinned_strs);
    err_pin_info:
    if (nr_pinned_info > 0)
    unpin_user_pages(pages_info, nr_pinned_info);
    err_pin_stat:
    if (nr_pinned_stat > 0)
    unpin_user_pages(pages_stat, nr_pinned_stat);
    err_nomem:
    atomic_set(&dev_priv.mksstat_user_pids[slot], 0);
    if (page)
    __free_page(page);
    kfree(pages_stat);
    return ret_err;
    }
//
// vmw_mksstat_remove_ioctl: Removes a single userspace-originating mksGuestStat
// instance descriptor from the hypervisor.
//
// Discard a hypervisor PFN mapping, containing a single mksGuestStat instance
// descriptor and unpin the corresponding userspace pages.
//
// @dev: Identifies the drm device.
// @data: Pointer to the ioctl argument.
// @file_priv: Identifies the caller; unused.
// Return: Zero on success, negative error code on error.
//
    int vmw_mksstat_remove_ioctl(struct drm_device *dev, void *data,
    struct drm_file *file_priv)
    {
    struct drm_vmw_mksstat_remove_arg *arg =
    (struct drm_vmw_mksstat_remove_arg *) data;
    let mut dev_priv: *mut vmw_private const = vmw_priv(dev);
    let mut slot: usize = arg.id;
    pid_t pgid, pid;
    if (slot >= ARRAY_SIZE(dev_priv.mksstat_user_pids))
    return -EINVAL;
    DRM_DEV_INFO(dev.dev, "pid=%d arg.id=%zu\n", current.pid, slot);
    pgid = task_pgrp_vnr(current);
    pid = atomic_cmpxchg(&dev_priv.mksstat_user_pids[slot], pgid, MKSSTAT_PID_RESERVED);
    if (!pid)
    return 0;
    if (pid == pgid) {
    let mut page: *mut page const = dev_priv.mksstat_user_pages[slot];
    BUG_ON(!page);
    dev_priv.mksstat_user_pages[slot] = core::ptr::null_mut();
    atomic_set(&dev_priv.mksstat_user_pids[slot], 0);
    hypervisor_ppn_remove((PPN64)page_to_pfn(page));
    vmw_mksstat_cleanup_descriptor(page);
    return 0;
    }
    return -EAGAIN;
    }
//
// vmw_disable_backdoor: Disables all backdoor communication
// with the hypervisor.
//
#[no_mangle]
pub unsafe extern "C" fn vmw_disable_backdoor() {
    void vmw_disable_backdoor(void)
    {
    vmw_msg_enabled = 0;
    }
