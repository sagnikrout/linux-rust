//! Automatically rewritten from C to Rust
//! Source: drivers/infiniband/core/user_mad.c
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


//
// Copyright (c) 2004 Topspin Communications.  All rights reserved.
// Copyright (c) 2005 Voltaire, Inc. All rights reserved.
// Copyright (c) 2005 Sun Microsystems, Inc. All rights reserved.
// Copyright (c) 2008 Cisco. All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

    MODULE_AUTHOR("Roland Dreier");
    MODULE_DESCRIPTION("InfiniBand userspace MAD packet access");
    MODULE_LICENSE("Dual BSD/GPL");
pub const MAX_UMAD_RECV_LIST_SIZE: c_int = 200000;
    enum {
    IB_UMAD_MAX_PORTS  = RDMA_MAX_PORTS,
    IB_UMAD_MAX_AGENTS = 32,
    IB_UMAD_MAJOR      = 231,
    IB_UMAD_MINOR_BASE = 0,
    IB_UMAD_NUM_FIXED_MINOR = 64,
    IB_UMAD_NUM_DYNAMIC_MINOR = IB_UMAD_MAX_PORTS - IB_UMAD_NUM_FIXED_MINOR,
    IB_ISSM_MINOR_BASE        = IB_UMAD_NUM_FIXED_MINOR,
    };
//
// Our lifetime rules for these structs are the following:
// device special file is opened, we take a reference on the
// ib_umad_port's struct ib_umad_device. We drop these
// references in the corresponding close().
//
// In addition to references coming from open character devices, there
// is one more reference to each ib_umad_device representing the
// module's reference taken when allocating the ib_umad_device in
// ib_umad_add_one().
//
// When destroying an ib_umad_device, we drop the module's reference.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_umad_port {
    pub cdev: cdev,
    pub dev: device,
    pub sm_cdev: cdev,
    pub sm_dev: device,
    pub sm_sem: semaphore,
    pub file_mutex: mutex,
    pub file_list: list_head,
    pub ib_dev: *mut ib_device,
    pub umad_dev: *mut ib_umad_device,
    pub dev_num: c_int,
    pub port_num: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_umad_device {
    pub kref: kref,
    pub ports: [ib_umad_port; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_umad_file {
    pub mutex: mutex,
    pub port: *mut ib_umad_port,
    pub recv_list: list_head,
    pub recv_list_size: core::sync::atomic::AtomicI32,
    pub send_list: list_head,
    pub port_list: list_head,
    pub send_lock: spinlock_t,
    pub recv_wait: wait_queue_head_t,
    pub agent: [*mut ib_mad_agent; IB_UMAD_MAX_AGENTS],
    pub agents_dead: c_int,
    pub use_pkey_index: u8,
    pub already_used: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_umad_packet {
    pub msg: *mut ib_mad_send_buf,
    pub recv_wc: *mut ib_mad_recv_wc,
    pub list: list_head,
    pub length: c_int,
    pub mad: ib_user_mad,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_rmpp_mad_hdr {
    pub mad_hdr: ib_mad_hdr,
    pub rmpp_hdr: ib_rmpp_hdr,
    pub __packed: },
// Macro flag: #define CREATE_TRACE_POINTS

    pub IB_UMAD_MINOR_BASE): static dev_t base_umad_dev = MKDEV(IB_UMAD_MAJOR,,
    static const dev_t base_issm_dev = MKDEV(IB_UMAD_MAJOR, IB_UMAD_MINOR_BASE) +
    pub dynamic_umad_dev: static dev_t,
    pub dynamic_issm_dev: static dev_t,
    pub DEFINE_IDA(umad_ida): static,
    pub device): *mut static int ib_umad_add_one(struct ib_device,
    pub client_data): *mut *mut static void ib_umad_remove_one(struct ib_device device, void,
#[no_mangle]
unsafe extern "C" fn ib_umad_dev_free(kref: *mut kref) {
    static void ib_umad_dev_free(struct kref *kref)
    {
    struct ib_umad_device *dev =
    pub kref): container_of(kref, struct ib_umad_device,,
    }
#[no_mangle]
unsafe extern "C" fn ib_umad_dev_get(dev: *mut ib_umad_device) {
    static void ib_umad_dev_get(struct ib_umad_device *dev)
    {
    }
#[no_mangle]
unsafe extern "C" fn ib_umad_dev_put(dev: *mut ib_umad_device) {
    static void ib_umad_dev_put(struct ib_umad_device *dev)
    {
    pub ib_umad_dev_free): kref_put(&dev->kref,,
    }
#[no_mangle]
unsafe extern "C" fn hdr_size(file: *mut ib_umad_file) -> c_int {
    static int hdr_size(struct ib_umad_file *file)
    {
    return file.use_pkey_index ? sizeof(struct ib_user_mad_hdr) :
    pub ib_user_mad_hdr_old): sizeof(struct,
    }
// caller must hold file->mutex
    static struct ib_mad_agent *__get_agent(struct ib_umad_file *file, int id)
    {
    pub file->agent[id]: return file->agents_dead ? NULL :,
    }
    static int queue_packet(struct ib_umad_file *file, struct ib_mad_agent *agent,
    struct ib_umad_packet *packet, bool is_recv_mad)
    {
    pub 1: int ret =,
    if (is_recv_mad &&
    atomic_read(&file.recv_list_size) > MAX_UMAD_RECV_LIST_SIZE)
    pub unlock: goto,
    pub 0: for (packet->mad.hdr.id =,
    pub IB_UMAD_MAX_AGENTS: packet->mad.hdr.id <,
    packet.mad.hdr.id++)
    if (agent == __get_agent(file, packet.mad.hdr.id)) {
    pub &file->recv_list): list_add_tail(&packet->list,,
    pub 0: ret =,
    }
    unlock:
    pub ret: return,
    }
    static void dequeue_send(struct ib_umad_file *file,
    struct ib_umad_packet *packet)
    {
    }
    static void send_handler(struct ib_mad_agent *agent,
    struct ib_mad_send_wc *send_wc)
    {
    pub agent->context: *mut *mut ib_umad_file file =,
    pub send_wc->send_buf->context[0]: *mut *mut ib_umad_packet packet =,
    pub packet): dequeue_send(file,,
    pub RDMA_DESTROY_AH_SLEEPABLE): rdma_destroy_ah(packet->msg->ah,,
    if (send_wc.status == IB_WC_RESP_TIMEOUT_ERR) {
    pub IB_MGMT_MAD_HDR: packet->length =,
    pub ETIMEDOUT: packet->mad.hdr.status =,
    if (!queue_packet(file, agent, packet, false))
    }
    }
    static void recv_handler(struct ib_mad_agent *agent,
    struct ib_mad_send_buf *send_buf,
    struct ib_mad_recv_wc *mad_recv_wc)
    {
    pub agent->context: *mut *mut ib_umad_file file =,
    pub packet: *mut ib_umad_packet,
    if (mad_recv_wc.wc.status != IB_WC_SUCCESS)
    pub err1: goto,
    pub kzalloc_obj(*packet): *mut packet =,
    if (!packet)
    pub err1: goto,
    pub mad_recv_wc->mad_len: packet->length =,
    pub mad_recv_wc: packet->recv_wc =,
    pub 0: packet->mad.hdr.status =,
    pub mad_recv_wc->mad_len: packet->mad.hdr.length = hdr_size(file) +,
    pub cpu_to_be32(mad_recv_wc->wc->src_qp): packet->mad.hdr.qpn =,
//
// On OPA devices it is okay to lose the upper 16 bits of LID as this
// information is obtained elsewhere. Mask off the upper 16 bits.
//
    if (rdma_cap_opa_mad(agent.device, agent.port_num))
    packet.mad.hdr.lid = ib_lid_be16(0xFFFF &
    else
    pub ib_lid_be16(mad_recv_wc->wc->slid): packet->mad.hdr.lid =,
    pub mad_recv_wc->wc->sl: packet->mad.hdr.sl =,
    pub mad_recv_wc->wc->dlid_path_bits: packet->mad.hdr.path_bits =,
    pub mad_recv_wc->wc->pkey_index: packet->mad.hdr.pkey_index =,
    pub IB_WC_GRH): packet->mad.hdr.grh_present = !!(mad_recv_wc->wc->wc_flags &,
    if (packet.mad.hdr.grh_present) {
    pub ah_attr: rdma_ah_attr,
    pub grh: *const ib_global_route,
    pub ret: c_int,
    ret = ib_init_ah_attr_from_wc(agent.device, agent.port_num,
    mad_recv_wc.wc,
    mad_recv_wc.recv_buf.grh,
    if (ret)
    pub err2: goto,
    pub rdma_ah_read_grh(&ah_attr): grh =,
    pub grh->sgid_index: packet->mad.hdr.gid_index =,
    pub grh->hop_limit: packet->mad.hdr.hop_limit =,
    pub grh->traffic_class: packet->mad.hdr.traffic_class =,
    pub 16): memcpy(packet->mad.hdr.gid, &grh->dgid,,
    pub cpu_to_be32(grh->flow_label): packet->mad.hdr.flow_label =,
    }
    if (queue_packet(file, agent, packet, true))
    pub err2: goto,
    err2:
    err1:
    }
    static ssize_t copy_recv_mad(struct ib_umad_file *file, char __user *buf,
    struct ib_umad_packet *packet, size_t count)
    {
    pub recv_buf: *mut ib_mad_recv_buf,
    pub max_seg_payload: int left, seg_payload, offset,,
    pub seg_size: usize,
    pub &packet->recv_wc->recv_buf: recv_buf =,
    pub packet->recv_wc->mad_seg_size: seg_size =,
// We need enough room to copy the first (or only) MAD segment.
    if ((packet.length <= seg_size &&
    count < hdr_size(file) + packet.length) ||
    (packet.length > seg_size &&
    count < hdr_size(file) + seg_size))
    pub -EINVAL: return,
    if (copy_to_user(buf, &packet.mad, hdr_size(file)))
    pub -EFAULT: return,
    pub hdr_size(file): buf +=,
    pub seg_size): seg_payload = min_t(int, packet->length,,
    if (copy_to_user(buf, recv_buf.mad, seg_payload))
    pub -EFAULT: return,
    if (seg_payload < packet.length) {
//
// Multipacket RMPP MAD message. Copy remainder of message.
// Note that last segment may have a shorter payload.
//
    if (count < hdr_size(file) + packet.length) {
//
// The buffer is too small, return the first RMPP segment,
// which includes the RMPP message length.
//
    pub -ENOSPC: return,
    }
    pub ib_get_mad_data_offset(recv_buf->mad->mad_hdr.mgmt_class): offset =,
    pub offset: max_seg_payload = seg_size -,
    pub seg_payload: for (left = packet->length - seg_payload, buf +=,
    pub {: left; left -= seg_payload, buf += seg_payload),
    recv_buf = container_of(recv_buf.list.next,
    pub list): ib_mad_recv_buf,,
    pub max_seg_payload): seg_payload = min(left,,
    if (copy_to_user(buf, ((void *) recv_buf.mad) + offset,
    seg_payload))
    pub -EFAULT: return,
    }
    }
    pub &recv_buf->mad->mad_hdr): trace_ib_umad_read_recv(file, &packet->mad.hdr,,
    pub packet->length: return hdr_size(file) +,
    }
    static ssize_t copy_send_mad(struct ib_umad_file *file, char __user *buf,
    struct ib_umad_packet *packet, size_t count)
    {
    pub packet->length: ssize_t size = hdr_size(file) +,
    if (count < size)
    pub -EINVAL: return,
    if (copy_to_user(buf, &packet.mad, hdr_size(file)))
    pub -EFAULT: return,
    pub hdr_size(file): buf +=,
    if (copy_to_user(buf, packet.mad.data, packet.length))
    pub -EFAULT: return,
    trace_ib_umad_read_send(file, &packet.mad.hdr,
    pub )&packet->mad.data): *mut (struct ib_mad_hdr,
    pub size: return,
    }
    static ssize_t ib_umad_read(struct file *filp, char __user *buf,
    size_t count, loff_t *pos)
    {
    pub filp->private_data: *mut *mut ib_umad_file file =,
    pub packet: *mut ib_umad_packet,
    pub ret: isize,
    if (count < hdr_size(file))
    pub -EINVAL: return,
    if (file.agents_dead) {
    pub -EIO: return,
    }
    while (list_empty(&file.recv_list)) {
    if (filp.f_flags & O_NONBLOCK)
    pub -EAGAIN: return,
    if (wait_event_interruptible(file.recv_wait,
    !list_empty(&file.recv_list)))
    pub -ERESTARTSYS: return,
    }
    if (file.agents_dead) {
    pub -EIO: return,
    }
    pub list): packet = list_entry(file->recv_list.next, struct ib_umad_packet,,
    if (packet.recv_wc)
    pub count): ret = copy_recv_mad(file, buf, packet,,
    else
    pub count): ret = copy_send_mad(file, buf, packet,,
    if (ret < 0) {
// Requeue packet
    pub &file->recv_list): list_add(&packet->list,,
    } else {
    if (packet.recv_wc)
    }
    pub ret: return,
    }
#[no_mangle]
unsafe extern "C" fn copy_rmpp_mad(msg: *mut ib_mad_send_buf, buf: *const char __user) -> c_int {
    static int copy_rmpp_mad(struct ib_mad_send_buf *msg, const char __user *buf)
    {
    pub seg: int left,,
// Copy class specific header
    if ((msg.hdr_len > IB_MGMT_RMPP_HDR) &&
    copy_from_user(msg.mad + IB_MGMT_RMPP_HDR, buf + IB_MGMT_RMPP_HDR,
    msg.hdr_len - IB_MGMT_RMPP_HDR))
    pub -EFAULT: return,
// All headers are in place.  Copy data segments.
    pub 0: for (seg = 1, left = msg->data_len, buf += msg->hdr_len; left >,
    seg++, left -= msg.seg_size, buf += msg.seg_size) {
    if (copy_from_user(ib_get_rmpp_segment(msg, seg), buf,
    min(left, msg.seg_size)))
    pub -EFAULT: return,
    }
    pub 0: return,
    }
    static int same_destination(struct ib_user_mad_hdr *hdr1,
    struct ib_user_mad_hdr *hdr2)
    {
    if (!hdr1.grh_present && !hdr2.grh_present)
    pub hdr2->lid): return (hdr1->lid ==,
    if (hdr1.grh_present && hdr2.grh_present)
    pub 16): return !memcmp(hdr1->gid, hdr2->gid,,
    pub 0: return,
    }
    static int is_duplicate(struct ib_umad_file *file,
    struct ib_umad_packet *packet)
    {
    pub sent_packet: *mut ib_umad_packet,
    pub hdr: *mut *mut ib_mad_hdr sent_hdr,,
    pub packet->mad.data: *mut *mut hdr = (struct ib_mad_hdr ),
    list_for_each_entry(sent_packet, &file.send_list, list) {
    pub sent_packet->mad.data: *mut *mut sent_hdr = (struct ib_mad_hdr ),
    if ((hdr.tid != sent_hdr.tid) ||
    (hdr.mgmt_class != sent_hdr.mgmt_class))
//
// No need to be overly clever here.  If two new operations have
// the same TID, reject the second as a duplicate.  This is more
// restrictive than required by the spec.
//
    if (!ib_response_mad(hdr)) {
    if (!ib_response_mad(sent_hdr))
    pub 1: return,
    } else if (!ib_response_mad(sent_hdr))
    if (same_destination(&packet.mad.hdr, &sent_packet.mad.hdr))
    pub 1: return,
    }
    pub 0: return,
    }
    static ssize_t ib_umad_write(struct file *filp, const char __user *buf,
    size_t count, loff_t *pos)
    {
    pub filp->private_data: *mut *mut ib_umad_file file =,
    pub rmpp_mad_hdr: *mut ib_rmpp_mad_hdr,
    pub packet: *mut ib_umad_packet,
    pub agent: *mut ib_mad_agent,
    pub ah_attr: rdma_ah_attr,
    pub ah: *mut ib_ah,
    pub tid: *mut __be64,
    pub rmpp_active: int ret, hdr_len, copy_offset,,
    pub data_len: usize,
    pub base_version: u8,
    if (count < hdr_size(file) + IB_MGMT_RMPP_HDR)
    pub -EINVAL: return,
    pub GFP_KERNEL): *mut *mut packet = kzalloc(sizeof(packet) + IB_MGMT_RMPP_HDR,,
    if (!packet)
    pub -ENOMEM: return,
    if (copy_from_user(&packet.mad, buf, hdr_size(file))) {
    pub -EFAULT: ret =,
    pub err: goto,
    }
    if (packet.mad.hdr.id >= IB_UMAD_MAX_AGENTS) {
    pub -EINVAL: ret =,
    pub err: goto,
    }
    pub hdr_size(file): buf +=,
    if (copy_from_user(packet.mad.data, buf, IB_MGMT_RMPP_HDR)) {
    pub -EFAULT: ret =,
    pub err: goto,
    }
    trace_ib_umad_write(file, &packet.mad.hdr,
    pub )&packet->mad.data): *mut (struct ib_mad_hdr,
    pub packet->mad.hdr.id): agent = __get_agent(file,,
    if (!agent) {
    pub -EIO: ret =,
    pub err_up: goto,
    }
    pub ah_attr): memset(&ah_attr, 0, sizeof,
    ah_attr.type = rdma_ah_find_type(agent.device,
    pub be16_to_cpu(packet->mad.hdr.lid)): rdma_ah_set_dlid(&ah_attr,,
    pub packet->mad.hdr.sl): rdma_ah_set_sl(&ah_attr,,
    pub packet->mad.hdr.path_bits): rdma_ah_set_path_bits(&ah_attr,,
    pub file->port->port_num): rdma_ah_set_port_num(&ah_attr,,
    if (packet.mad.hdr.grh_present) {
    rdma_ah_set_grh(&ah_attr, core::ptr::null_mut(),
    be32_to_cpu(packet.mad.hdr.flow_label),
    packet.mad.hdr.gid_index,
    packet.mad.hdr.hop_limit,
    pub packet->mad.hdr.gid): rdma_ah_set_dgid_raw(&ah_attr,,
    }
    pub NULL): ah = rdma_create_user_ah(agent->qp->pd, &ah_attr,,
    if (IS_ERR(ah)) {
    pub PTR_ERR(ah): ret =,
    pub err_up: goto,
    }
    pub )packet->mad.data: *mut rmpp_mad_hdr = (struct ib_rmpp_mad_hdr,
    pub ib_get_mad_data_offset(rmpp_mad_hdr->mad_hdr.mgmt_class): hdr_len =,
    if (ib_is_mad_class_rmpp(rmpp_mad_hdr.mad_hdr.mgmt_class)
    && ib_mad_kernel_rmpp_agent(agent)) {
    pub IB_MGMT_RMPP_HDR: copy_offset =,
    rmpp_active = ib_get_rmpp_flags(&rmpp_mad_hdr.rmpp_hdr) &
    } else {
    pub IB_MGMT_MAD_HDR: copy_offset =,
    pub 0: rmpp_active =,
    }
    pub )&packet->mad.data)->base_version: *mut base_version = ((struct ib_mad_hdr,
    if (check_sub_overflow(count, hdr_size(file) + hdr_len, &data_len)) {
    pub -EINVAL: ret =,
    pub err_ah: goto,
    }
    packet.msg = ib_create_send_mad(agent,
    be32_to_cpu(packet.mad.hdr.qpn),
    packet.mad.hdr.pkey_index, rmpp_active,
    hdr_len, data_len, GFP_KERNEL,
    if (IS_ERR(packet.msg)) {
    pub PTR_ERR(packet->msg): ret =,
    pub err_ah: goto,
    }
    pub ah: packet->msg->ah =,
    pub packet->mad.hdr.timeout_ms: packet->msg->timeout_ms =,
    pub packet->mad.hdr.retries: packet->msg->retries =,
    pub packet: packet->msg->context[0] =,
// Copy MAD header.  Any RMPP header is already in place.
    pub IB_MGMT_MAD_HDR): memcpy(packet->msg->mad, packet->mad.data,,
    if (!rmpp_active) {
    if (copy_from_user(packet.msg.mad + copy_offset,
    buf + copy_offset,
    hdr_len + data_len - copy_offset)) {
    pub -EFAULT: ret =,
    pub err_msg: goto,
    }
    } else {
    pub buf): ret = copy_rmpp_mad(packet->msg,,
    if (ret)
    pub err_msg: goto,
    }
//
// Set the high-order part of the transaction ID to make MADs from
// different agents unique, and allow routing responses back to the
// original requestor.
//
    if (!ib_response_mad(packet.msg.mad)) {
    pub packet->msg->mad)->tid: *mut *mut tid = &((struct ib_mad_hdr ),
// tid = cpu_to_be64(((u64) agent->hi_tid) << 32 |
    pub 0xffffffff)): (be64_to_cpup(tid) &,
    pub tid: *mut rmpp_mad_hdr->mad_hdr.tid =,
    }
    if (!ib_mad_kernel_rmpp_agent(agent)
    && ib_is_mad_class_rmpp(rmpp_mad_hdr.mad_hdr.mgmt_class)
    && (ib_get_rmpp_flags(&rmpp_mad_hdr.rmpp_hdr) & IB_MGMT_RMPP_FLAG_ACTIVE)) {
    pub &file->send_list): list_add_tail(&packet->list,,
    } else {
    pub packet): ret = is_duplicate(file,,
    if (!ret)
    pub &file->send_list): list_add_tail(&packet->list,,
    if (ret) {
    pub -EINVAL: ret =,
    pub err_msg: goto,
    }
    }
    pub NULL): ret = ib_post_send_mad(packet->msg,,
    if (ret)
    pub err_send: goto,
    pub count: return,
    err_send:
    pub packet): dequeue_send(file,,
    err_msg:
    err_ah:
    pub RDMA_DESTROY_AH_SLEEPABLE): rdma_destroy_ah(ah,,
    err_up:
    err:
    pub ret: return,
    }
#[no_mangle]
unsafe extern "C" fn ib_umad_poll(filp: *mut file, wait: *mut poll_table_struct) -> __poll_t {
    static __poll_t ib_umad_poll(struct file *filp, struct poll_table_struct *wait)
    {
    pub filp->private_data: *mut *mut ib_umad_file file =,
// we will always be able to post a MAD send
    pub EPOLLWRNORM: __poll_t mask = EPOLLOUT |,
    pub wait): poll_wait(filp, &file->recv_wait,,
    if (!list_empty(&file.recv_list))
    pub EPOLLRDNORM: mask |= EPOLLIN |,
    if (file.agents_dead)
    pub EPOLLERR: mask =,
    pub mask: return,
    }
    static int ib_umad_reg_agent(struct ib_umad_file *file, void __user *arg,
    int compat_method_mask)
    {
    pub ureq: ib_user_mad_reg_req,
    pub req: ib_mad_reg_req,
    pub NULL: *mut *mut ib_mad_agent agent =,
    pub agent_id: c_int,
    pub ret: c_int,
    if (!file.port.ib_dev) {
    pub __func__): dev_notice(&file->port->dev, "%s: invalid device\n",,
    pub -EPIPE: ret =,
    pub out: goto,
    }
    if (copy_from_user(&ureq, arg, sizeof ureq)) {
    pub -EFAULT: ret =,
    pub out: goto,
    }
    if (ureq.qpn != 0 && ureq.qpn != 1) {
    dev_notice(&file.port.dev,
    "%s: invalid QPN %u specified\n", __func__,
    pub -EINVAL: ret =,
    pub out: goto,
    }
    pub ++agent_id): for (agent_id = 0; agent_id < IB_UMAD_MAX_AGENTS;,
    if (!__get_agent(file, agent_id))
    pub found: goto,
    dev_notice(&file.port.dev, "%s: Max Agents (%u) reached\n", __func__,
    pub -ENOMEM: ret =,
    pub out: goto,
    found:
    if (ureq.mgmt_class) {
    pub sizeof(req)): memset(&req, 0,,
    pub ureq.mgmt_class: req.mgmt_class =,
    pub ureq.mgmt_class_version: req.mgmt_class_version =,
    pub req.oui): memcpy(req.oui, ureq.oui, sizeof,
    if (compat_method_mask) {
    pub ureq.method_mask: *mut *mut *mut u32 umm = (u32 ),
    pub i: c_int,
    pub ++i): for (i = 0; i < BITS_TO_LONGS(IB_MGMT_MAX_METHODS);,
    req.method_mask[i] =
    pub 32): *mut *mut *mut umm[i  2] | ((u64) umm[i  2 + 1] <<,
    } else
    memcpy(req.method_mask, ureq.method_mask,
    pub req.method_mask): sizeof,
    }
    agent = ib_register_mad_agent(file.port.ib_dev, file.port.port_num,
    ureq.qpn ? IB_QPT_GSI : IB_QPT_SMI,
    ureq.mgmt_class ? &req : core::ptr::null_mut(),
    ureq.rmpp_version,
    pub 0): send_handler, recv_handler, file,,
    if (IS_ERR(agent)) {
    pub PTR_ERR(agent): ret =,
    pub NULL: agent =,
    pub out: goto,
    }
    if (put_user(agent_id,
    (u32 __user *) (arg + offsetof(struct ib_user_mad_reg_req, id)))) {
    pub -EFAULT: ret =,
    pub out: goto,
    }
    if (!file.already_used) {
    pub 1: file->already_used =,
    if (!file.use_pkey_index) {
    dev_warn(&file.port.dev,
    "process %s did not enable P_Key index support.\n",
    dev_warn(&file.port.dev,
    pub ABI.\n"): " Documentation/infiniband/user_mad.rst has info on the new,
    }
    }
    pub agent: file->agent[agent_id] =,
    pub 0: ret =,
    out:
    if (ret && agent)
    pub ret: return,
    }
#[no_mangle]
unsafe extern "C" fn ib_umad_reg_agent2(file: *mut ib_umad_file, arg: *mut void __user) -> c_int {
    static int ib_umad_reg_agent2(struct ib_umad_file *file, void __user *arg)
    {
    pub ureq: ib_user_mad_reg_req2,
    pub req: ib_mad_reg_req,
    pub NULL: *mut *mut ib_mad_agent agent =,
    pub agent_id: c_int,
    pub ret: c_int,
    if (!file.port.ib_dev) {
    pub __func__): dev_notice(&file->port->dev, "%s: invalid device\n",,
    pub -EPIPE: ret =,
    pub out: goto,
    }
    if (copy_from_user(&ureq, arg, sizeof(ureq))) {
    pub -EFAULT: ret =,
    pub out: goto,
    }
    if (ureq.qpn != 0 && ureq.qpn != 1) {
    dev_notice(&file.port.dev, "%s: invalid QPN %u specified\n",
    pub ureq.qpn): __func__,,
    pub -EINVAL: ret =,
    pub out: goto,
    }
    if (ureq.flags & ~IB_USER_MAD_REG_FLAGS_CAP) {
    dev_notice(&file.port.dev,
    pub 0x%x\n",: "%s failed: invalid registration flags specified 0x%x; supported,
    pub IB_USER_MAD_REG_FLAGS_CAP): __func__, ureq.flags,,
    pub -EINVAL: ret =,
    if (put_user((u32)IB_USER_MAD_REG_FLAGS_CAP,
    (u32 __user *) (arg + offsetof(struct
    ib_user_mad_reg_req2, flags))))
    pub -EFAULT: ret =,
    pub out: goto,
    }
    pub ++agent_id): for (agent_id = 0; agent_id < IB_UMAD_MAX_AGENTS;,
    if (!__get_agent(file, agent_id))
    pub found: goto,
    dev_notice(&file.port.dev, "%s: Max Agents (%u) reached\n", __func__,
    pub -ENOMEM: ret =,
    pub out: goto,
    found:
    if (ureq.mgmt_class) {
    pub sizeof(req)): memset(&req, 0,,
    pub ureq.mgmt_class: req.mgmt_class =,
    pub ureq.mgmt_class_version: req.mgmt_class_version =,
    if (ureq.oui & 0xff000000) {
    dev_notice(&file.port.dev,
    "%s failed: oui invalid 0x%08x\n", __func__,
    pub -EINVAL: ret =,
    pub out: goto,
    }
    pub 0x0000ff: req.oui[2] = ureq.oui &,
    pub 8: req.oui[1] = (ureq.oui & 0x00ff00) >>,
    pub 16: req.oui[0] = (ureq.oui & 0xff0000) >>,
    memcpy(req.method_mask, ureq.method_mask,
    }
    agent = ib_register_mad_agent(file.port.ib_dev, file.port.port_num,
    ureq.qpn ? IB_QPT_GSI : IB_QPT_SMI,
    ureq.mgmt_class ? &req : core::ptr::null_mut(),
    ureq.rmpp_version,
    send_handler, recv_handler, file,
    if (IS_ERR(agent)) {
    pub PTR_ERR(agent): ret =,
    pub NULL: agent =,
    pub out: goto,
    }
    if (put_user(agent_id,
    (u32 __user *)(arg +
    offsetof(struct ib_user_mad_reg_req2, id)))) {
    pub -EFAULT: ret =,
    pub out: goto,
    }
    if (!file.already_used) {
    pub 1: file->already_used =,
    pub 1: file->use_pkey_index =,
    }
    pub agent: file->agent[agent_id] =,
    pub 0: ret =,
    out:
    if (ret && agent)
    pub ret: return,
    }
#[no_mangle]
unsafe extern "C" fn ib_umad_unreg_agent(file: *mut ib_umad_file, arg: *mut u32 __user) -> c_int {
    static int ib_umad_unreg_agent(struct ib_umad_file *file, u32 __user *arg)
    {
    pub NULL: *mut *mut ib_mad_agent agent =,
    pub id: u32,
    pub 0: int ret =,
    if (get_user(id, arg))
    pub -EFAULT: return,
    if (id >= IB_UMAD_MAX_AGENTS)
    pub -EINVAL: return,
    pub IB_UMAD_MAX_AGENTS): id = array_index_nospec(id,,
    if (!__get_agent(file, id)) {
    pub -EINVAL: ret =,
    pub out: goto,
    }
    pub file->agent[id]: agent =,
    pub NULL: file->agent[id] =,
    out:
    if (agent)
    pub ret: return,
    }
#[no_mangle]
unsafe extern "C" fn ib_umad_enable_pkey(file: *mut ib_umad_file) -> c_long {
    static long ib_umad_enable_pkey(struct ib_umad_file *file)
    {
    pub 0: int ret =,
    if (file.already_used)
    pub -EINVAL: ret =,
    else
    pub 1: file->use_pkey_index =,
    pub ret: return,
    }
    static long ib_umad_ioctl(struct file *filp, unsigned int cmd,
    unsigned long arg)
    {
    switch (cmd) {
    case IB_USER_MAD_REGISTER_AGENT:
    pub 0): *mut *mut return ib_umad_reg_agent(filp->private_data, (void __user ) arg,,
    case IB_USER_MAD_UNREGISTER_AGENT:
    pub arg): *mut *mut return ib_umad_unreg_agent(filp->private_data, (__u32 __user ),
    case IB_USER_MAD_ENABLE_PKEY:
    pub ib_umad_enable_pkey(filp->private_data): return,
    case IB_USER_MAD_REGISTER_AGENT2:
    pub arg): *mut *mut return ib_umad_reg_agent2(filp->private_data, (void __user ),
    default:
    pub -ENOIOCTLCMD: return,
    }
    }

    static long ib_umad_compat_ioctl(struct file *filp, unsigned int cmd,
    unsigned long arg)
    {
    switch (cmd) {
    case IB_USER_MAD_REGISTER_AGENT:
    pub 1): return ib_umad_reg_agent(filp->private_data, compat_ptr(arg),,
    case IB_USER_MAD_UNREGISTER_AGENT:
    pub compat_ptr(arg)): return ib_umad_unreg_agent(filp->private_data,,
    case IB_USER_MAD_ENABLE_PKEY:
    pub ib_umad_enable_pkey(filp->private_data): return,
    case IB_USER_MAD_REGISTER_AGENT2:
    pub compat_ptr(arg)): return ib_umad_reg_agent2(filp->private_data,,
    default:
    pub -ENOIOCTLCMD: return,
    }
    }

//
// ib_umad_open() does not need the BKL:
//
// - the ib_umad_port structures are properly reference counted, and
// everything else is purely local to the file being created, so
// races against other open calls are not a problem;
// - the ioctl method does not affect any global state outside of the
// file structure being operated on;
//
#[no_mangle]
unsafe extern "C" fn ib_umad_open(inode: *mut inode, filp: *mut file) -> c_int {
    static int ib_umad_open(struct inode *inode, struct file *filp)
    {
    pub port: *mut ib_umad_port,
    pub file: *mut ib_umad_file,
    pub 0: int ret =,
    pub cdev): port = container_of(inode->i_cdev, struct ib_umad_port,,
    if (!port.ib_dev) {
    pub -ENXIO: ret =,
    pub out: goto,
    }
    if (!rdma_dev_access_netns(port.ib_dev, current.nsproxy.net_ns)) {
    pub -EPERM: ret =,
    pub out: goto,
    }
    pub kzalloc_obj(*file): *mut file =,
    if (!file) {
    pub -ENOMEM: ret =,
    pub out: goto,
    }
    pub port: file->port =,
    pub file: filp->private_data =,
    pub &port->file_list): list_add_tail(&file->port_list,,
    pub filp): stream_open(inode,,
    out:
    pub ret: return,
    }
#[no_mangle]
unsafe extern "C" fn ib_umad_close(inode: *mut inode, filp: *mut file) -> c_int {
    static int ib_umad_close(struct inode *inode, struct file *filp)
    {
    pub filp->private_data: *mut *mut ib_umad_file file =,
    pub tmp: *mut *mut ib_umad_packet packet,,
    pub already_dead: c_int,
    pub i: c_int,
    pub file->agents_dead: already_dead =,
    pub 1: file->agents_dead =,
    list_for_each_entry_safe(packet, tmp, &file.recv_list, list) {
    if (packet.recv_wc)
    }
    if (!already_dead)
    pub ++i): for (i = 0; i < IB_UMAD_MAX_AGENTS;,
    if (file.agent[i])
    pub 0: return,
    }
    static const struct file_operations umad_fops = {
    .owner		= THIS_MODULE,
    .read		= ib_umad_read,
    .write		= ib_umad_write,
    .poll		= ib_umad_poll,
    .unlocked_ioctl = ib_umad_ioctl,

    .compat_ioctl	= ib_umad_compat_ioctl,

    .open		= ib_umad_open,
    .release	= ib_umad_close,
}

#[no_mangle]
unsafe extern "C" fn ib_umad_sm_open(inode: *mut inode, filp: *mut file) -> c_int {
    static int ib_umad_sm_open(struct inode *inode, struct file *filp)
    {
    struct ib_umad_port *port;
    struct ib_port_modify props = {
    .set_port_cap_mask = IB_PORT_SM
    };
    int ret;
    port = container_of(inode.i_cdev, struct ib_umad_port, sm_cdev);
    if (filp.f_flags & O_NONBLOCK) {
    if (down_trylock(&port.sm_sem)) {
    ret = -EAGAIN;
    goto fail;
    }
    } else {
    if (down_interruptible(&port.sm_sem)) {
    ret = -ERESTARTSYS;
    goto fail;
    }
    }
    if (!rdma_dev_access_netns(port.ib_dev, current.nsproxy.net_ns)) {
    ret = -EPERM;
    goto err_up_sem;
    }
    ret = ib_modify_port(port.ib_dev, port.port_num, 0, &props);
    if (ret)
    goto err_up_sem;
    filp.private_data = port;
    nonseekable_open(inode, filp);
    return 0;
    err_up_sem:
    up(&port.sm_sem);
    fail:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ib_umad_sm_close(inode: *mut inode, filp: *mut file) -> c_int {
    static int ib_umad_sm_close(struct inode *inode, struct file *filp)
    {
    struct ib_umad_port *port = filp.private_data;
    struct ib_port_modify props = {
    .clr_port_cap_mask = IB_PORT_SM
    };
    let mut ret: c_int = 0;
    mutex_lock(&port.file_mutex);
    if (port.ib_dev)
    ret = ib_modify_port(port.ib_dev, port.port_num, 0, &props);
    mutex_unlock(&port.file_mutex);
    up(&port.sm_sem);
    return ret;
    }
    static const struct file_operations umad_sm_fops = {
    .owner	 = THIS_MODULE,
    .open	 = ib_umad_sm_open,
    .release = ib_umad_sm_close,
    };
    static struct ib_umad_port *get_port(struct ib_device *ibdev,
    struct ib_umad_device *umad_dev,
    u32 port)
    {
    if (!umad_dev)
    return ERR_PTR(-EOPNOTSUPP);
    if (!rdma_is_port_valid(ibdev, port))
    return ERR_PTR(-EINVAL);
    if (!rdma_cap_ib_mad(ibdev, port))
    return ERR_PTR(-EOPNOTSUPP);
    return &umad_dev.ports[port - rdma_start_port(ibdev)];
    }
    static int ib_umad_get_nl_info(struct ib_device *ibdev, void *client_data,
    struct ib_client_nl_info *res)
    {
    struct ib_umad_port *port = get_port(ibdev, client_data, res.port);
    if (IS_ERR(port))
    return PTR_ERR(port);
    res.abi = IB_USER_MAD_ABI_VERSION;
    res.cdev = &port.dev;
    return 0;
    }
    static struct ib_client umad_client = {
    .name   = "umad",
    .add    = ib_umad_add_one,
    .remove = ib_umad_remove_one,
    .get_nl_info = ib_umad_get_nl_info,
    };
    MODULE_ALIAS_RDMA_CLIENT("umad");
    static int ib_issm_get_nl_info(struct ib_device *ibdev, void *client_data,
    struct ib_client_nl_info *res)
    {
    struct ib_umad_port *port = get_port(ibdev, client_data, res.port);
    if (IS_ERR(port))
    return PTR_ERR(port);
    res.abi = IB_USER_MAD_ABI_VERSION;
    res.cdev = &port.sm_dev;
    return 0;
    }
    static struct ib_client issm_client = {
    .name = "issm",
    .get_nl_info = ib_issm_get_nl_info,
    };
    MODULE_ALIAS_RDMA_CLIENT("issm");
    static ssize_t ibdev_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct ib_umad_port *port = dev_get_drvdata(dev);
    if (!port)
    return -ENODEV;
    return sysfs_emit(buf, "%s\n", dev_name(&port.ib_dev.dev));
    }
    static DEVICE_ATTR_RO(ibdev);
    static ssize_t port_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct ib_umad_port *port = dev_get_drvdata(dev);
    if (!port)
    return -ENODEV;
    return sysfs_emit(buf, "%d\n", port.port_num);
    }
    static DEVICE_ATTR_RO(port);
    static struct attribute *umad_class_dev_attrs[] = {
    &dev_attr_ibdev.attr,
    &dev_attr_port.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(umad_class_dev);
    static char *umad_devnode(const struct device *dev, umode_t *mode)
    {
    return kasprintf(GFP_KERNEL, "infiniband/%s", dev_name(dev));
    }
    static ssize_t abi_version_show(const struct class *class,
    const struct class_attribute *attr, char *buf)
    {
    return sysfs_emit(buf, "%d\n", IB_USER_MAD_ABI_VERSION);
    }
    static CLASS_ATTR_RO(abi_version);
    static struct attribute *umad_class_attrs[] = {
    &class_attr_abi_version.attr,
    core::ptr::null_mut(),
    };
    ATTRIBUTE_GROUPS(umad_class);
    static struct class umad_class = {
    .name		= "infiniband_mad",
    .devnode	= umad_devnode,
    .class_groups	= umad_class_groups,
    .dev_groups	= umad_class_dev_groups,
    };
#[no_mangle]
unsafe extern "C" fn ib_umad_release_port(device: *mut device) {
    static void ib_umad_release_port(struct device *device)
    {
    struct ib_umad_port *port = dev_get_drvdata(device);
    struct ib_umad_device *umad_dev = port.umad_dev;
    ib_umad_dev_put(umad_dev);
    }
    static void ib_umad_init_port_dev(struct device *dev,
    struct ib_umad_port *port,
    const struct ib_device *device)
    {
    device_initialize(dev);
    ib_umad_dev_get(port.umad_dev);
    dev.class = &umad_class;
    dev.parent = device.dev.parent;
    dev_set_drvdata(dev, port);
    dev.release = ib_umad_release_port;
    }
    static int ib_umad_init_port(struct ib_device *device, int port_num,
    struct ib_umad_device *umad_dev,
    struct ib_umad_port *port)
    {
    int devnum;
    dev_t base_umad;
    dev_t base_issm;
    int ret;
    devnum = ida_alloc_max(&umad_ida, IB_UMAD_MAX_PORTS - 1, GFP_KERNEL);
    if (devnum < 0)
    return -1;
    port.dev_num = devnum;
    if (devnum >= IB_UMAD_NUM_FIXED_MINOR) {
    base_umad = dynamic_umad_dev + devnum - IB_UMAD_NUM_FIXED_MINOR;
    base_issm = dynamic_issm_dev + devnum - IB_UMAD_NUM_FIXED_MINOR;
    } else {
    base_umad = devnum + base_umad_dev;
    base_issm = devnum + base_issm_dev;
    }
    port.ib_dev   = device;
    port.umad_dev = umad_dev;
    port.port_num = port_num;
    sema_init(&port.sm_sem, 1);
    mutex_init(&port.file_mutex);
    INIT_LIST_HEAD(&port.file_list);
    ib_umad_init_port_dev(&port.dev, port, device);
    port.dev.devt = base_umad;
    dev_set_name(&port.dev, "umad%d", port.dev_num);
    cdev_init(&port.cdev, &umad_fops);
    port.cdev.owner = THIS_MODULE;
    ret = cdev_device_add(&port.cdev, &port.dev);
    if (ret)
    goto err_cdev;
    if (rdma_cap_ib_smi(device, port_num)) {
    ib_umad_init_port_dev(&port.sm_dev, port, device);
    port.sm_dev.devt = base_issm;
    dev_set_name(&port.sm_dev, "issm%d", port.dev_num);
    cdev_init(&port.sm_cdev, &umad_sm_fops);
    port.sm_cdev.owner = THIS_MODULE;
    ret = cdev_device_add(&port.sm_cdev, &port.sm_dev);
    if (ret)
    goto err_dev;
    }
    return 0;
    err_dev:
    put_device(&port.sm_dev);
    cdev_device_del(&port.cdev, &port.dev);
    err_cdev:
    put_device(&port.dev);
    ida_free(&umad_ida, devnum);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ib_umad_kill_port(port: *mut ib_umad_port) {
    static void ib_umad_kill_port(struct ib_umad_port *port)
    {
    struct ib_umad_file *file;
    let mut has_smi: bool = false;
    int id;
    if (rdma_cap_ib_smi(port.ib_dev, port.port_num)) {
    cdev_device_del(&port.sm_cdev, &port.sm_dev);
    has_smi = true;
    }
    cdev_device_del(&port.cdev, &port.dev);
    mutex_lock(&port.file_mutex);
// Mark ib_dev NULL and block ioctl or other file ops to progress
// further.
//
    port.ib_dev = core::ptr::null_mut();
    list_for_each_entry(file, &port.file_list, port_list) {
    mutex_lock(&file.mutex);
    file.agents_dead = 1;
    wake_up_interruptible(&file.recv_wait);
    mutex_unlock(&file.mutex);
    for (id = 0; id < IB_UMAD_MAX_AGENTS; ++id)
    if (file.agent[id])
    ib_unregister_mad_agent(file.agent[id]);
    }
    mutex_unlock(&port.file_mutex);
    ida_free(&umad_ida, port.dev_num);
// balances device_initialize()
    if (has_smi)
    put_device(&port.sm_dev);
    put_device(&port.dev);
    }
#[no_mangle]
unsafe extern "C" fn ib_umad_add_one(device: *mut ib_device) -> c_int {
    static int ib_umad_add_one(struct ib_device *device)
    {
    struct ib_umad_device *umad_dev;
    int s, e, i;
    let mut count: c_int = 0;
    int ret;
    s = rdma_start_port(device);
    e = rdma_end_port(device);
    umad_dev = kzalloc_flex(*umad_dev, ports, size_add(size_sub(e, s), 1));
    if (!umad_dev)
    return -ENOMEM;
    kref_init(&umad_dev.kref);
    for (i = s; i <= e; ++i) {
    if (!rdma_cap_ib_mad(device, i))
    continue;
    ret = ib_umad_init_port(device, i, umad_dev,
    &umad_dev.ports[i - s]);
    if (ret)
    goto err;
    count++;
    }
    if (!count) {
    ret = -EOPNOTSUPP;
    goto free;
    }
    ib_set_client_data(device, &umad_client, umad_dev);
    return 0;
    err:
    while (--i >= s) {
    if (!rdma_cap_ib_mad(device, i))
    continue;
    ib_umad_kill_port(&umad_dev.ports[i - s]);
    }
    free:
// balances kref_init
    ib_umad_dev_put(umad_dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ib_umad_remove_one(device: *mut ib_device, client_data: *mut c_void) {
    static void ib_umad_remove_one(struct ib_device *device, void *client_data)
    {
    struct ib_umad_device *umad_dev = client_data;
    unsigned int i;
    rdma_for_each_port (device, i) {
    if (rdma_cap_ib_mad(device, i))
    ib_umad_kill_port(
    &umad_dev.ports[i - rdma_start_port(device)]);
    }
// balances kref_init()
    ib_umad_dev_put(umad_dev);
    }
#[no_mangle]
unsafe extern "C" fn ib_umad_init() -> int __init {
    static int __init ib_umad_init(void)
    {
    int ret;
    ret = register_chrdev_region(base_umad_dev,
    IB_UMAD_NUM_FIXED_MINOR * 2,
    umad_class.name);
    if (ret) {
    pr_err("couldn't register device number\n");
    goto out;
    }
    ret = alloc_chrdev_region(&dynamic_umad_dev, 0,
    IB_UMAD_NUM_DYNAMIC_MINOR * 2,
    umad_class.name);
    if (ret) {
    pr_err("couldn't register dynamic device number\n");
    goto out_alloc;
    }
    dynamic_issm_dev = dynamic_umad_dev + IB_UMAD_NUM_DYNAMIC_MINOR;
    ret = class_register(&umad_class);
    if (ret) {
    pr_err("couldn't create class infiniband_mad\n");
    goto out_chrdev;
    }
    ret = ib_register_client(&umad_client);
    if (ret)
    goto out_class;
    ret = ib_register_client(&issm_client);
    if (ret)
    goto out_client;
    return 0;
    out_client:
    ib_unregister_client(&umad_client);
    out_class:
    class_unregister(&umad_class);
    out_chrdev:
    unregister_chrdev_region(dynamic_umad_dev,
    IB_UMAD_NUM_DYNAMIC_MINOR * 2);
    out_alloc:
    unregister_chrdev_region(base_umad_dev,
    IB_UMAD_NUM_FIXED_MINOR * 2);
    out:
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn ib_umad_cleanup() -> void __exit {
    static void __exit ib_umad_cleanup(void)
    {
    ib_unregister_client(&issm_client);
    ib_unregister_client(&umad_client);
    class_unregister(&umad_class);
    unregister_chrdev_region(base_umad_dev,
    IB_UMAD_NUM_FIXED_MINOR * 2);
    unregister_chrdev_region(dynamic_umad_dev,
    IB_UMAD_NUM_DYNAMIC_MINOR * 2);
    }
    module_init(ib_umad_init);
    module_exit(ib_umad_cleanup);
