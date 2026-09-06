//! Automatically rewritten from C to Rust
//! Source: drivers/infiniband/hw/mlx4/mad.c
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
// Copyright (c) 2007 Cisco Systems, Inc. All rights reserved.
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

    enum {
    MLX4_IB_VENDOR_CLASS1 = 0x9,
    MLX4_IB_VENDOR_CLASS2 = 0xa
    };
pub const MLX4_TUN_SEND_WRID_SHIFT: c_int = 34;
pub const MLX4_TUN_QPN_SHIFT: c_int = 32;

// Port mgmt change event handling

pub const NUM_IDX_IN_PKEY_TBL_BLK: c_int = 32;

pub const GUID_TBL_BLK_NUM_ENTRIES: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_mad_rcv_buf {
    pub grh: ib_grh,
    pub payload: [u8; 256],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_mad_snd_buf {
    pub payload: [u8; 256],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_tunnel_mad {
    pub grh: ib_grh,
    pub hdr: mlx4_ib_tunnel_header,
    pub mad: ib_mad,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_rcv_tunnel_mad {
    pub hdr: mlx4_rcv_tunnel_hdr,
    pub grh: ib_grh,
    pub mad: ib_mad,
    pub __packed: },
    pub port_num): *mut *mut static void handle_client_rereg_event(struct mlx4_ib_dev dev, u32,
    pub port_num): *mut *mut static void handle_lid_change_event(struct mlx4_ib_dev dev, u32,
    static void __propagate_pkey_ev(struct mlx4_ib_dev *dev, int port_num,
    pub change_bitmap): int block, u32,
#[no_mangle]
pub unsafe extern "C" fn mlx4_ib_gen_node_guid() -> __be64 {
    __be64 mlx4_ib_gen_node_guid(void)
    {

    pub get_random_u32()): return cpu_to_be64(NODE_GUID_HI |,
    }
#[no_mangle]
pub unsafe extern "C" fn mlx4_ib_get_new_demux_tid(ctx: *mut mlx4_ib_demux_ctx) -> __be64 {
    __be64 mlx4_ib_get_new_demux_tid(struct mlx4_ib_demux_ctx *ctx)
    {
    return cpu_to_be64(atomic_inc_return(&ctx.tid)) |
    }
    int mlx4_MAD_IFC(struct mlx4_ib_dev *dev, int mad_ifc_flags,
    int port, const struct ib_wc *in_wc,
    const struct ib_grh *in_grh,
    const void *in_mad, void *response_mad)
    {
    pub outmailbox: *mut *mut mlx4_cmd_mailbox inmailbox,,
    pub inbox: *mut c_void,
    pub err: c_int,
    pub port: u32 in_modifier =,
    pub 0: u8 op_modifier =,
    pub mlx4_alloc_cmd_mailbox(dev->dev): inmailbox =,
    if (IS_ERR(inmailbox))
    pub PTR_ERR(inmailbox): return,
    pub inmailbox->buf: inbox =,
    pub mlx4_alloc_cmd_mailbox(dev->dev): outmailbox =,
    if (IS_ERR(outmailbox)) {
    pub inmailbox): mlx4_free_cmd_mailbox(dev->dev,,
    pub PTR_ERR(outmailbox): return,
    }
    pub 256): memcpy(inbox, in_mad,,
//
// Key check traps can't be generated unless we have in_wc to
// tell us where to send the trap.
//
    if ((mad_ifc_flags & MLX4_MAD_IFC_IGNORE_MKEY) || !in_wc)
    pub 0x1: op_modifier |=,
    if ((mad_ifc_flags & MLX4_MAD_IFC_IGNORE_BKEY) || !in_wc)
    pub 0x2: op_modifier |=,
    if (mlx4_is_mfunc(dev.dev) &&
    (mad_ifc_flags & MLX4_MAD_IFC_NET_VIEW || in_wc))
    pub 0x8: op_modifier |=,
    if (in_wc) {
    struct {
    pub my_qpn: __be32,
    pub reserved1: u32,
    pub rqpn: __be32,
    pub sl: u8,
    pub g_path: u8,
    pub reserved2: [u16; 2],
    pub pkey: __be16,
    pub reserved3: [u32; 11],
    pub grh: [u8; 40],
    pub ext_info: *mut },
    pub 256): memset(inbox + 256, 0,,
    pub 256: ext_info = inbox +,
    pub cpu_to_be32(in_wc->qp->qp_num): ext_info->my_qpn =,
    pub cpu_to_be32(in_wc->src_qp): ext_info->rqpn =,
    pub 4: ext_info->sl = in_wc->sl <<,
    ext_info.g_path = in_wc.dlid_path_bits |
    pub 0): (in_wc->wc_flags & IB_WC_GRH ? 0x80 :,
    pub cpu_to_be16(in_wc->pkey_index): ext_info->pkey =,
    if (in_grh)
    pub 40): memcpy(ext_info->grh, in_grh,,
    pub 0x4: op_modifier |=,
    pub 16: in_modifier |= ib_lid_cpu16(in_wc->slid) <<,
    }
    err = mlx4_cmd_box(dev.dev, inmailbox.dma, outmailbox.dma, in_modifier,
    mlx4_is_master(dev.dev) ? (op_modifier & ~0x8) : op_modifier,
    MLX4_CMD_MAD_IFC, MLX4_CMD_TIME_CLASS_C,
    pub MLX4_CMD_WRAPPED): (op_modifier & 0x8) ? MLX4_CMD_NATIVE :,
    if (!err)
    pub 256): memcpy(response_mad, outmailbox->buf,,
    pub inmailbox): mlx4_free_cmd_mailbox(dev->dev,,
    pub outmailbox): mlx4_free_cmd_mailbox(dev->dev,,
    pub err: return,
    }
#[no_mangle]
unsafe extern "C" fn update_sm_ah(dev: *mut mlx4_ib_dev, port_num: u32, lid: u16, sl: u8) {
    static void update_sm_ah(struct mlx4_ib_dev *dev, u32 port_num, u16 lid, u8 sl)
    {
    pub new_ah: *mut ib_ah,
    pub ah_attr: rdma_ah_attr,
    pub flags: c_ulong,
    if (!dev.send_agent[port_num - 1][0])
    pub ah_attr): memset(&ah_attr, 0, sizeof,
    pub port_num): ah_attr.type = rdma_ah_find_type(&dev->ib_dev,,
    pub lid): rdma_ah_set_dlid(&ah_attr,,
    pub sl): rdma_ah_set_sl(&ah_attr,,
    pub port_num): rdma_ah_set_port_num(&ah_attr,,
    new_ah = rdma_create_ah(dev.send_agent[port_num - 1][0].qp.pd,
    pub 0): &ah_attr,,
    if (IS_ERR(new_ah))
    pub flags): spin_lock_irqsave(&dev->sm_lock,,
    if (dev.sm_ah[port_num - 1])
    pub 0): rdma_destroy_ah(dev->sm_ah[port_num - 1],,
    pub new_ah: dev->sm_ah[port_num - 1] =,
    pub flags): spin_unlock_irqrestore(&dev->sm_lock,,
    }
//
// Snoop SM MADs for port info, GUID info, and  P_Key table sets, so we can
// synthesize LID change, Client-Rereg, GID change, and P_Key change events.
//
    static void smp_snoop(struct ib_device *ibdev, u32 port_num,
    const struct ib_mad *mad, u16 prev_lid)
    {
    pub pinfo: *mut ib_port_info,
    pub lid: u16,
    pub base: *mut __be16,
    pub pkey_change_bitmap: u32 bn,,
    pub i: c_int,
    pub to_mdev(ibdev): *mut *mut mlx4_ib_dev dev =,
    if ((mad.mad_hdr.mgmt_class == IB_MGMT_CLASS_SUBN_LID_ROUTED ||
    mad.mad_hdr.mgmt_class == IB_MGMT_CLASS_SUBN_DIRECTED_ROUTE) &&
    mad.mad_hdr.method == IB_MGMT_METHOD_SET)
    switch (mad.mad_hdr.attr_id) {
    case IB_SMP_ATTR_PORT_INFO:
    if (dev.dev.caps.flags & MLX4_DEV_CAP_FLAG_PORT_MNG_CHG_EV)
    pub mad)->data: *mut *mut *mut pinfo = (struct ib_port_info ) ((struct ib_smp ),
    pub be16_to_cpu(pinfo->lid): lid =,
    update_sm_ah(dev, port_num,
    be16_to_cpu(pinfo.sm_lid),
    pub 0xf): pinfo->neighbormtu_mastersmsl &,
    if (pinfo.clientrereg_resv_subnetto & 0x80)
    pub port_num): handle_client_rereg_event(dev,,
    if (prev_lid != lid)
    pub port_num): handle_lid_change_event(dev,,
    case IB_SMP_ATTR_PKEY_TABLE:
    if (dev.dev.caps.flags & MLX4_DEV_CAP_FLAG_PORT_MNG_CHG_EV)
    if (!mlx4_is_mfunc(dev.dev)) {
    mlx4_ib_dispatch_event(dev, port_num,
    }
// at this point, we are running in the master.
// Slaves do not receive SMPs.
//
    pub 0xFFFF: *mut *mut bn = be32_to_cpu(((struct ib_smp )mad)->attr_mod) &,
    pub )mad)->data[0]): *mut *mut base = (__be16 ) &(((struct ib_smp,
    pub 0: pkey_change_bitmap =,
    pub {: for (i = 0; i < 32; i++),
    pr_debug("PKEY[%d] = x%x\n",
    pub be16_to_cpu(base[i])): *mut *mut i + bn32,,
    if (be16_to_cpu(base[i]) !=
    dev.pkeys.phys_pkey_cache[port_num - 1][i + bn*32]) {
    pub i): pkey_change_bitmap |= (1 <<,
    dev.pkeys.phys_pkey_cache[port_num - 1][i + bn*32] =
    }
    }
    pr_debug("PKEY Change event: port=%u, "
    "block=0x%x, change_bitmap=0x%x\n",
    pub pkey_change_bitmap): port_num, bn,,
    if (pkey_change_bitmap) {
    mlx4_ib_dispatch_event(dev, port_num,
    if (!dev.sriov.is_going_down)
    __propagate_pkey_ev(dev, port_num, bn,
    }
    case IB_SMP_ATTR_GUID_INFO:
    if (dev.dev.caps.flags & MLX4_DEV_CAP_FLAG_PORT_MNG_CHG_EV)
// paravirtualized master's guid is guid 0 -- does not change
    if (!mlx4_is_master(dev.dev))
    mlx4_ib_dispatch_event(dev, port_num,
// if master, notify relevant slaves
    if (mlx4_is_master(dev.dev) &&
    !dev.sriov.is_going_down) {
    pub )mad)->attr_mod): *mut bn = be32_to_cpu(((struct ib_smp,
    mlx4_ib_update_cache_on_guid_change(dev, bn, port_num,
    pub )mad)->data)): *mut *mut (u8 )(&((struct ib_smp,
    mlx4_ib_notify_slaves_on_guid_change(dev, bn, port_num,
    pub )mad)->data)): *mut *mut (u8 )(&((struct ib_smp,
    }
    case IB_SMP_ATTR_SL_TO_VL_TABLE:
// cache sl to vl mapping changes for use in
// filling QP1 LRH VL field when sending packets
//
    if (dev.dev.caps.flags & MLX4_DEV_CAP_FLAG_PORT_MNG_CHG_EV &&
    dev.dev.caps.flags2 & MLX4_DEV_CAP_FLAG2_SL_TO_VL_CHANGE_EVENT)
    if (!mlx4_is_slave(dev.dev)) {
    pub sl2vl64: union sl2vl_tbl_to_u64,
    pub jj: c_int,
    pub {: for (jj = 0; jj < 8; jj++),
    pub )mad)->data[jj]: *mut sl2vl64.sl8[jj] = ((struct ib_smp,
    pr_debug("port %u, sl2vl[%d] = %02x\n",
    pub sl2vl64.sl8[jj]): port_num, jj,,
    }
    pub sl2vl64.sl64): atomic64_set(&dev->sl2vl[port_num - 1],,
    }
    default:
    }
    }
    static void __propagate_pkey_ev(struct mlx4_ib_dev *dev, int port_num,
    int block, u32 change_bitmap)
    {
    pub err: int i, ix, slave,,
    pub 0: int have_event =,
    pub {: for (slave = 0; slave < dev->dev->caps.sqp_demux; slave++),
    if (slave == mlx4_master_func_num(dev.dev))
    if (!mlx4_is_slave_active(dev.dev, slave))
    pub 0: have_event =,
    pub {: for (i = 0; i < 32; i++),
    if (!(change_bitmap & (1 << i)))
    pub 0: for (ix =,
    pub {: ix < dev->dev->caps.pkey_table_len[port_num]; ix++),
    if (dev.pkeys.virt2phys_pkey[slave][port_num - 1]
    [ix] == i + 32 * block) {
    pub port_num): err = mlx4_gen_pkey_eqe(dev->dev, slave,,
    pr_debug("propagate_pkey_ev: slave %d,"
    " port %d, ix %d (%d)\n",
    pub err): slave, port_num, ix,,
    pub 1: have_event =,
    }
    }
    if (have_event)
    }
    }
    }
    static void node_desc_override(struct ib_device *dev,
    struct ib_mad *mad)
    {
    pub flags: c_ulong,
    if ((mad.mad_hdr.mgmt_class == IB_MGMT_CLASS_SUBN_LID_ROUTED ||
    mad.mad_hdr.mgmt_class == IB_MGMT_CLASS_SUBN_DIRECTED_ROUTE) &&
    mad.mad_hdr.method == IB_MGMT_METHOD_GET_RESP &&
    mad.mad_hdr.attr_id == IB_SMP_ATTR_NODE_DESC) {
    pub flags): spin_lock_irqsave(&to_mdev(dev)->sm_lock,,
    memcpy(((struct ib_smp *) mad).data, dev.node_desc,
    pub flags): spin_unlock_irqrestore(&to_mdev(dev)->sm_lock,,
    }
    }
    static void forward_trap(struct mlx4_ib_dev *dev, u32 port_num,
    const struct ib_mad *mad)
    {
    pub IB_MGMT_CLASS_SUBN_LID_ROUTED: int qpn = mad->mad_hdr.mgmt_class !=,
    pub send_buf: *mut ib_mad_send_buf,
    pub 1][qpn]: *mut *mut ib_mad_agent agent = dev->send_agent[port_num -,
    pub ret: c_int,
    pub flags: c_ulong,
    if (agent) {
    send_buf = ib_create_send_mad(agent, qpn, 0, 0, IB_MGMT_MAD_HDR,
    IB_MGMT_MAD_DATA, GFP_ATOMIC,
    if (IS_ERR(send_buf))
//
// We rely here on the fact that MLX QPs don't use the
// address handle after the send is posted (this is
// wrong following the IB spec strictly, but we know
// it's OK for our devices).
//
    pub flags): spin_lock_irqsave(&dev->sm_lock,,
    pub mad): *mut memcpy(send_buf->mad, mad, sizeof,
    if ((send_buf.ah = dev.sm_ah[port_num - 1]))
    pub NULL): ret = ib_post_send_mad(send_buf,,
    else
    pub -EINVAL: ret =,
    pub flags): spin_unlock_irqrestore(&dev->sm_lock,,
    if (ret)
    }
    }
    static int mlx4_ib_demux_sa_handler(struct ib_device *ibdev, int port, int slave,
    struct ib_sa_mad *sa_mad)
    {
    pub 0: int ret =,
// dispatch to different sa handlers
    switch (be16_to_cpu(sa_mad.mad_hdr.attr_id)) {
    case IB_SA_ATTR_MC_MEMBER_REC:
    pub sa_mad): ret = mlx4_ib_mcg_demux_handler(ibdev, port, slave,,
    default:
    }
    pub ret: return,
    }
#[no_mangle]
pub unsafe extern "C" fn mlx4_ib_find_real_gid(ibdev: *mut ib_device, port: u32, guid: __be64) -> c_int {
    int mlx4_ib_find_real_gid(struct ib_device *ibdev, u32 port, __be64 guid)
    {
    pub to_mdev(ibdev): *mut *mut mlx4_ib_dev dev =,
    pub i: c_int,
    pub {: for (i = 0; i < dev->dev->caps.sqp_demux; i++),
    if (dev.sriov.demux[port - 1].guid_cache[i] == guid)
    pub i: return,
    }
    pub -1: return,
    }
    static int find_slave_port_pkey_ix(struct mlx4_ib_dev *dev, int slave,
    u32 port, u16 pkey, u16 *ix)
    {
    pub ret: int i,,
    pub 0xFF: u8 unassigned_pkey_ix, pkey_ix, partial_ix =,
    pub slot_pkey: u16,
    if (slave == mlx4_master_func_num(dev.dev))
    pub ix): return ib_find_cached_pkey(&dev->ib_dev, port, pkey,,
    pub 1: unassigned_pkey_ix = dev->dev->phys_caps.pkey_phys_table_len[port] -,
    pub {: for (i = 0; i < dev->dev->caps.pkey_table_len[port]; i++),
    if (dev.pkeys.virt2phys_pkey[slave][port - 1][i] == unassigned_pkey_ix)
    pub 1][i]: pkey_ix = dev->pkeys.virt2phys_pkey[slave][port -,
    pub &slot_pkey): ret = ib_get_cached_pkey(&dev->ib_dev, port, pkey_ix,,
    if (ret)
    if ((slot_pkey & 0x7FFF) == (pkey & 0x7FFF)) {
    if (slot_pkey & 0x8000) {
// ix = (u16) pkey_ix;
    pub 0: return,
    } else {
// take first partial pkey index found
    if (partial_ix == 0xFF)
    pub pkey_ix: partial_ix =,
    }
    }
    }
    if (partial_ix < 0xFF) {
// ix = (u16) partial_ix;
    pub 0: return,
    }
    pub -EINVAL: return,
    }
    static int get_gids_from_l3_hdr(struct ib_grh *grh, union ib_gid *sgid,
    union ib_gid *dgid)
    {
    pub )grh): *const int version = ib_get_rdma_header_version((union rdma_network_hdr,
    pub net_type: enum rdma_network_type,
    if (version == 4)
    pub RDMA_NETWORK_IPV4: net_type =,
#[no_mangle]
pub unsafe extern "C" fn if(6: version ==) -> else {
    else if (version == 6)
    pub RDMA_NETWORK_IPV6: net_type =,
    else
    pub -EINVAL: return,
    return ib_get_gids_from_rdma_hdr((union rdma_network_hdr *)grh, net_type,
    pub dgid): sgid,,
    }
#[no_mangle]
unsafe extern "C" fn is_proxy_qp0(dev: *mut mlx4_ib_dev, qpn: c_int, slave: c_int) -> c_int {
    static int is_proxy_qp0(struct mlx4_ib_dev *dev, int qpn, int slave)
    {
    pub slave: *mut *mut int proxy_start = dev->dev->phys_caps.base_proxy_sqpn + 8,
    pub 1): return (qpn >= proxy_start && qpn <= proxy_start +,
    }
    int mlx4_ib_send_to_slave(struct mlx4_ib_dev *dev, int slave, u32 port,
    enum ib_qp_type dest_qpt, struct ib_wc *wc,
    struct ib_grh *grh, struct ib_mad *mad)
    {
    pub list: ib_sge,
    pub wr: ib_ud_wr,
    pub bad_wr: *const ib_send_wr,
    pub tun_ctx: *mut mlx4_ib_demux_pv_ctx,
    pub tun_qp: *mut mlx4_ib_demux_pv_qp,
    pub tun_mad: *mut mlx4_rcv_tunnel_mad,
    pub attr: rdma_ah_attr,
    pub ah: *mut ib_ah,
    pub NULL: *mut *mut ib_qp src_qp =,
    pub 0: unsigned tun_tx_ix =,
    pub dqpn: c_int,
    pub 0: int ret =,
    pub tun_pkey_ix: u16,
    pub cached_pkey: u16,
    pub MLX4_PORT_TYPE_ETH: u8 is_eth = dev->dev->caps.port_type[port] ==,
    if (dest_qpt > IB_QPT_GSI) {
    pub dest_qpt): pr_debug("dest_qpt (%d) > IB_QPT_GSI\n",,
    pub -EINVAL: return,
    }
    pub dev->sriov.demux[port-1].tun[slave]: tun_ctx =,
// check if proxy qp created
    if (!tun_ctx || tun_ctx.state != DEMUX_PV_STATE_ACTIVE)
    pub -EAGAIN: return,
    if (!dest_qpt)
    pub &tun_ctx->qp[0]: tun_qp =,
    else
    pub &tun_ctx->qp[1]: tun_qp =,
// compute P_Key index to put in tunnel header for slave
    if (dest_qpt) {
    pub pkey_ix: u16,
    pub &cached_pkey): ret = ib_get_cached_pkey(&dev->ib_dev, port, wc->pkey_index,,
    if (ret) {
    pr_debug("unable to get %s cached pkey for index %d, ret %d\n",
    is_proxy_qp0(dev, wc.src_qp, slave) ? "SMI" : "GSI",
    pub ret): wc->pkey_index,,
    pub -EINVAL: return,
    }
    pub &pkey_ix): ret = find_slave_port_pkey_ix(dev, slave, port, cached_pkey,,
    if (ret) {
    pr_debug("unable to get %s pkey ix for pkey 0x%x, ret %d\n",
    is_proxy_qp0(dev, wc.src_qp, slave) ? "SMI" : "GSI",
    pub ret): cached_pkey,,
    pub -EINVAL: return,
    }
    pub pkey_ix: tun_pkey_ix =,
    } else
    pub 1][0]: tun_pkey_ix = dev->pkeys.virt2phys_pkey[slave][port -,
    pub 1: *mut *mut *mut dqpn = dev->dev->phys_caps.base_proxy_sqpn + 8  slave + port + (dest_qpt  2) -,
// get tunnel tx data buf for slave
    pub tun_qp->qp: src_qp =,
// create ah. Just need an empty one with the port num for the post send.
// The driver will set the force loopback bit in post_send
    pub attr): memset(&attr, 0, sizeof,
    pub port): attr.type = rdma_ah_find_type(&dev->ib_dev,,
    pub port): rdma_ah_set_port_num(&attr,,
    if (is_eth) {
    pub sgid: union ib_gid,
    pub dgid: union ib_gid,
    if (get_gids_from_l3_hdr(grh, &sgid, &dgid))
    pub -EINVAL: return,
    pub 0): rdma_ah_set_grh(&attr, &dgid, 0, 0, 0,,
    }
    pub 0): ah = rdma_create_ah(tun_ctx->pd, &attr,,
    if (IS_ERR(ah))
    pub -ENOMEM: return,
// allocate tunnel tx buf after pass failure returns
    if (tun_qp.tx_ix_head - tun_qp.tx_ix_tail >=
    (MLX4_NUM_TUNNEL_BUFS - 1))
    pub -EAGAIN: ret =,
    else
    pub 1): tun_tx_ix = (++tun_qp->tx_ix_head) & (MLX4_NUM_TUNNEL_BUFS -,
    if (ret)
    pub end: goto,
    pub (tun_qp->tx_ring[tun_tx_ix].buf.addr): *mut *mut tun_mad = (struct mlx4_rcv_tunnel_mad ),
    if (tun_qp.tx_ring[tun_tx_ix].ah)
    pub 0): rdma_destroy_ah(tun_qp->tx_ring[tun_tx_ix].ah,,
    pub ah: tun_qp->tx_ring[tun_tx_ix].ah =,
    ib_dma_sync_single_for_cpu(&dev.ib_dev,
    tun_qp.tx_ring[tun_tx_ix].buf.map,
    sizeof (struct mlx4_rcv_tunnel_mad),
// copy over to tunnel buffer
    if (grh)
    pub grh): *mut memcpy(&tun_mad->grh, grh, sizeof,
    pub mad): *mut memcpy(&tun_mad->mad, mad, sizeof,
// adjust tunnel data
    pub cpu_to_be16(tun_pkey_ix): tun_mad->hdr.pkey_index =,
    pub 0xFFFFFF): tun_mad->hdr.flags_src_qp = cpu_to_be32(wc->src_qp &,
    pub 0: tun_mad->hdr.g_ml_path = (grh && (wc->wc_flags & IB_WC_GRH)) ? 0x80 :,
    if (is_eth) {
    pub 0: u16 vlan =,
    if (mlx4_get_slave_default_vlan(dev.dev, port, slave, &vlan,
    core::ptr::null_mut())) {
// VST mode
    if (vlan != wc.vlan_id)
// Packet vlan is not the VST-assigned vlan.
// Drop the packet.
//
    pub out: goto,
    else
// Remove the vlan tag before forwarding
// the packet to the VF.
//
    pub 0xffff: vlan =,
    } else {
    pub wc->vlan_id: vlan =,
    }
    pub cpu_to_be16(vlan): tun_mad->hdr.sl_vid =,
    pub 4): *mut *mut memcpy((char )&tun_mad->hdr.mac_31_0, &(wc->smac[0]),,
    pub 2): *mut *mut memcpy((char )&tun_mad->hdr.slid_mac_47_32, &(wc->smac[4]),,
    } else {
    pub 12): tun_mad->hdr.sl_vid = cpu_to_be16(((u16)(wc->sl)) <<,
    pub ib_lid_be16(wc->slid): tun_mad->hdr.slid_mac_47_32 =,
    }
    ib_dma_sync_single_for_device(&dev.ib_dev,
    tun_qp.tx_ring[tun_tx_ix].buf.map,
    sizeof (struct mlx4_rcv_tunnel_mad),
    pub tun_qp->tx_ring[tun_tx_ix].buf.map: list.addr =,
    pub mlx4_rcv_tunnel_mad): list.length = sizeof (struct,
    pub tun_ctx->pd->local_dma_lkey: list.lkey =,
    pub ah: wr.ah =,
    pub port: wr.port_num =,
    pub IB_QP_SET_QKEY: wr.remote_qkey =,
    pub dqpn: wr.remote_qpn =,
    pub NULL: wr.wr.next =,
    pub MLX4_TUN_SET_WRID_QPN(dest_qpt): wr.wr.wr_id = ((u64) tun_tx_ix) |,
    pub &list: wr.wr.sg_list =,
    pub 1: wr.wr.num_sge =,
    pub IB_WR_SEND: wr.wr.opcode =,
    pub IB_SEND_SIGNALED: wr.wr.send_flags =,
    pub &bad_wr): ret = ib_post_send(src_qp, &wr.wr,,
    if (!ret)
    pub 0: return,
    out:
    pub NULL: tun_qp->tx_ring[tun_tx_ix].ah =,
    end:
    pub 0): rdma_destroy_ah(ah,,
    pub ret: return,
    }
    static int mlx4_ib_demux_mad(struct ib_device *ibdev, u32 port,
    struct ib_wc *wc, struct ib_grh *grh,
    struct ib_mad *mad)
    {
    pub to_mdev(ibdev): *mut *mut mlx4_ib_dev dev =,
    pub other_port: int err,,
    pub -1: int slave =,
    pub slave_id: *mut u8,
    pub 0: int is_eth =,
    if (rdma_port_get_link_layer(ibdev, port) == IB_LINK_LAYER_INFINIBAND)
    pub 0: is_eth =,
    else
    pub 1: is_eth =,
    if (is_eth) {
    pub dgid: union ib_gid,
    pub sgid: union ib_gid,
    if (get_gids_from_l3_hdr(grh, &sgid, &dgid))
    pub -EINVAL: return,
    if (!(wc.wc_flags & IB_WC_GRH)) {
    pub present.\n"): mlx4_ib_warn(ibdev, "RoCE grh not,
    pub -EINVAL: return,
    }
    if (mad.mad_hdr.mgmt_class != IB_MGMT_CLASS_CM) {
    pub CM\n"): mlx4_ib_warn(ibdev, "RoCE mgmt class is not,
    pub -EINVAL: return,
    }
    pub &slave): err = mlx4_get_slave_from_roce_gid(dev->dev, port, dgid.raw,,
    if (err && mlx4_is_mf_bonded(dev.dev)) {
    pub 1: other_port = (port == 1) ? 2 :,
    pub &slave): err = mlx4_get_slave_from_roce_gid(dev->dev, other_port, dgid.raw,,
    if (!err) {
    pub other_port: port =,
    pr_debug("resolved slave %d from gid %pI6 wire port %d other %d\n",
    pub other_port): slave, grh->dgid.raw, port,,
    }
    }
    if (err) {
    pub grh\n"): mlx4_ib_warn(ibdev, "failed matching,
    pub -ENOENT: return,
    }
    if (slave >= dev.dev.caps.sqp_demux) {
    mlx4_ib_warn(ibdev, "slave id: %d is bigger than allowed:%d\n",
    pub dev->dev->caps.sqp_demux): slave,,
    pub -ENOENT: return,
    }
    if (mlx4_ib_demux_cm_handler(ibdev, port, core::ptr::null_mut(), mad))
    pub 0: return,
    pub mad): err = mlx4_ib_send_to_slave(dev, slave, port, wc->qp->qp_type, wc, grh,,
    if (err)
    pr_debug("failed sending %s to slave %d via tunnel qp (%d)\n",
    is_proxy_qp0(dev, wc.src_qp, slave) ? "SMI" : "GSI",
    pub err): slave,,
    pub 0: return,
    }
// Initially assume that this mad is for us
    pub mlx4_master_func_num(dev->dev): slave =,
// See if the slave id is encoded in a response mad
    if (mad.mad_hdr.method & 0x80) {
    pub &mad->mad_hdr.tid: *mut *mut slave_id = (u8 ),
    pub slave_id: *mut slave =,
    if (slave != 255) /*255 indicates the dom0*/
// slave_id = 0; /* remap tid
    }
// If a grh is present, we demux according to it
    if (wc.wc_flags & IB_WC_GRH) {
    if (grh.dgid.global.interface_id ==
    cpu_to_be64(IB_SA_WELL_KNOWN_GUID) &&
    grh.dgid.global.subnet_prefix == cpu_to_be64(
    atomic64_read(&dev.sriov.demux[port - 1].subnet_prefix))) {
    pub 0: slave =,
    } else {
    slave = mlx4_ib_find_real_gid(ibdev, port,
    if (slave < 0) {
    pub grh\n"): mlx4_ib_warn(ibdev, "failed matching,
    pub -ENOENT: return,
    }
    }
    }
// Class-specific handling
    switch (mad.mad_hdr.mgmt_class) {
    case IB_MGMT_CLASS_SUBN_LID_ROUTED:
    case IB_MGMT_CLASS_SUBN_DIRECTED_ROUTE:
// 255 indicates the dom0
    if (slave != 255 && slave != mlx4_master_func_num(dev.dev)) {
    if (!mlx4_vf_smi_enabled(dev.dev, slave, port))
    pub -EPERM: return,
// for a VF. drop unsolicited MADs
    if (!(mad.mad_hdr.method & IB_MGMT_METHOD_RESP)) {
    mlx4_ib_warn(ibdev, "demux QP0. rejecting unsolicited mad for slave %d class 0x%x, method 0x%x\n",
    slave, mad.mad_hdr.mgmt_class,
    pub -EINVAL: return,
    }
    }
    case IB_MGMT_CLASS_SUBN_ADM:
    if (mlx4_ib_demux_sa_handler(ibdev, port, slave,
    (struct ib_sa_mad *) mad))
    pub 0: return,
    case IB_MGMT_CLASS_CM:
    if (mlx4_ib_demux_cm_handler(ibdev, port, &slave, mad))
    pub 0: return,
    case IB_MGMT_CLASS_DEVICE_MGMT:
    if (mad.mad_hdr.method != IB_MGMT_METHOD_GET_RESP)
    pub 0: return,
    default:
// Drop unsupported classes for slaves in tunnel mode
    if (slave != mlx4_master_func_num(dev.dev)) {
    pr_debug("dropping unsupported ingress mad from class:%d "
    pub slave): "for slave:%d\n", mad->mad_hdr.mgmt_class,,
    pub 0: return,
    }
    }
// make sure that no slave==255 was not handled yet.
    if (slave >= dev.dev.caps.sqp_demux) {
    mlx4_ib_warn(ibdev, "slave id: %d is bigger than allowed:%d\n",
    pub dev->dev->caps.sqp_demux): slave,,
    pub -ENOENT: return,
    }
    pub mad): err = mlx4_ib_send_to_slave(dev, slave, port, wc->qp->qp_type, wc, grh,,
    if (err)
    pr_debug("failed sending %s to slave %d via tunnel qp (%d)\n",
    is_proxy_qp0(dev, wc.src_qp, slave) ? "SMI" : "GSI",
    pub err): slave,,
    pub 0: return,
    }
    static int ib_process_mad(struct ib_device *ibdev, int mad_flags, u32 port_num,
    const struct ib_wc *in_wc, const struct ib_grh *in_grh,
    const struct ib_mad *in_mad, struct ib_mad *out_mad)
    {
    pub 0: u16 slid, prev_lid =,
    pub err: c_int,
    pub pattr: ib_port_attr,
    pub be16_to_cpu(IB_LID_PERMISSIVE): slid = in_wc ? ib_lid_cpu16(in_wc->slid) :,
    if (in_mad.mad_hdr.method == IB_MGMT_METHOD_TRAP && slid == 0) {
    pub in_mad): forward_trap(to_mdev(ibdev), port_num,,
    pub IB_MAD_RESULT_CONSUMED: return IB_MAD_RESULT_SUCCESS |,
    }
    if (in_mad.mad_hdr.mgmt_class == IB_MGMT_CLASS_SUBN_LID_ROUTED ||
    in_mad.mad_hdr.mgmt_class == IB_MGMT_CLASS_SUBN_DIRECTED_ROUTE) {
    if (in_mad.mad_hdr.method   != IB_MGMT_METHOD_GET &&
    in_mad.mad_hdr.method   != IB_MGMT_METHOD_SET &&
    in_mad.mad_hdr.method   != IB_MGMT_METHOD_TRAP_REPRESS)
    pub IB_MAD_RESULT_SUCCESS: return,
//
// Don't process SMInfo queries -- the SMA can't handle them.
//
    if (in_mad.mad_hdr.attr_id == IB_SMP_ATTR_SM_INFO)
    pub IB_MAD_RESULT_SUCCESS: return,
    } else if (in_mad.mad_hdr.mgmt_class == IB_MGMT_CLASS_PERF_MGMT ||
    in_mad.mad_hdr.mgmt_class == MLX4_IB_VENDOR_CLASS1   ||
    in_mad.mad_hdr.mgmt_class == MLX4_IB_VENDOR_CLASS2   ||
    in_mad.mad_hdr.mgmt_class == IB_MGMT_CLASS_CONG_MGMT) {
    if (in_mad.mad_hdr.method  != IB_MGMT_METHOD_GET &&
    in_mad.mad_hdr.method  != IB_MGMT_METHOD_SET)
    pub IB_MAD_RESULT_SUCCESS: return,
    } else
    pub IB_MAD_RESULT_SUCCESS: return,
    if ((in_mad.mad_hdr.mgmt_class == IB_MGMT_CLASS_SUBN_LID_ROUTED ||
    in_mad.mad_hdr.mgmt_class == IB_MGMT_CLASS_SUBN_DIRECTED_ROUTE) &&
    in_mad.mad_hdr.method == IB_MGMT_METHOD_SET &&
    in_mad.mad_hdr.attr_id == IB_SMP_ATTR_PORT_INFO &&
    !ib_query_port(ibdev, port_num, &pattr))
    pub ib_lid_cpu16(pattr.lid): prev_lid =,
    err = mlx4_MAD_IFC(to_mdev(ibdev),
    (mad_flags & IB_MAD_IGNORE_MKEY ? MLX4_MAD_IFC_IGNORE_MKEY : 0) |
    (mad_flags & IB_MAD_IGNORE_BKEY ? MLX4_MAD_IFC_IGNORE_BKEY : 0) |
    MLX4_MAD_IFC_NET_VIEW,
    pub out_mad): port_num, in_wc, in_grh, in_mad,,
    if (err)
    pub IB_MAD_RESULT_FAILURE: return,
    if (!out_mad.mad_hdr.status) {
    pub prev_lid): smp_snoop(ibdev, port_num, in_mad,,
// slaves get node desc from FW
    if (!mlx4_is_slave(to_mdev(ibdev).dev))
    pub out_mad): node_desc_override(ibdev,,
    }
// set return bit in status of directed route responses
    if (in_mad.mad_hdr.mgmt_class == IB_MGMT_CLASS_SUBN_DIRECTED_ROUTE)
    pub 15): out_mad->mad_hdr.status |= cpu_to_be16(1 <<,
    if (in_mad.mad_hdr.method == IB_MGMT_METHOD_TRAP_REPRESS)
// no response for trap repress
    pub IB_MAD_RESULT_CONSUMED: return IB_MAD_RESULT_SUCCESS |,
    pub IB_MAD_RESULT_REPLY: return IB_MAD_RESULT_SUCCESS |,
    }
    static void edit_counter(struct mlx4_counter *cnt, void *counters,
    __be16 attr_id)
    {
    switch (attr_id) {
    case IB_PMA_PORT_COUNTERS:
    {
    struct ib_pma_portcounters *pma_cnt =
    pub )counters: *mut (struct ib_pma_portcounters,
    ASSIGN_32BIT_COUNTER(pma_cnt.port_xmit_data,
    pub 2)): (be64_to_cpu(cnt->tx_bytes) >>,
    ASSIGN_32BIT_COUNTER(pma_cnt.port_rcv_data,
    pub 2)): (be64_to_cpu(cnt->rx_bytes) >>,
    ASSIGN_32BIT_COUNTER(pma_cnt.port_xmit_packets,
    ASSIGN_32BIT_COUNTER(pma_cnt.port_rcv_packets,
    }
    case IB_PMA_PORT_COUNTERS_EXT:
    {
    struct ib_pma_portcounters_ext *pma_cnt_ext =
    pub )counters: *mut (struct ib_pma_portcounters_ext,
    pma_cnt_ext.port_xmit_data =
    pub 2): cpu_to_be64(be64_to_cpu(cnt->tx_bytes) >>,
    pma_cnt_ext.port_rcv_data =
    pub 2): cpu_to_be64(be64_to_cpu(cnt->rx_bytes) >>,
    pub cnt->tx_frames: pma_cnt_ext->port_xmit_packets =,
    pub cnt->rx_frames: pma_cnt_ext->port_rcv_packets =,
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn iboe_process_mad_port_info(out_mad: *mut c_void) -> c_int {
    static int iboe_process_mad_port_info(void *out_mad)
    {
    pub {}: ib_class_port_info cpi =,
    pub IB_PMA_CLASS_CAP_EXT_WIDTH: cpi.capability_mask =,
    pub sizeof(cpi)): memcpy(out_mad, &cpi,,
    pub IB_MAD_RESULT_REPLY: return IB_MAD_RESULT_SUCCESS |,
    }
    static int iboe_process_mad(struct ib_device *ibdev, int mad_flags,
    u32 port_num, const struct ib_wc *in_wc,
    const struct ib_grh *in_grh,
    const struct ib_mad *in_mad, struct ib_mad *out_mad)
    {
    pub counter_stats: mlx4_counter,
    pub to_mdev(ibdev): *mut *mut mlx4_ib_dev dev =,
    pub tmp_counter: *mut counter_index,
    pub 0: int err = IB_MAD_RESULT_FAILURE, stats_avail =,
    if (in_mad.mad_hdr.mgmt_class != IB_MGMT_CLASS_PERF_MGMT)
    pub -EINVAL: return,
    if (in_mad.mad_hdr.attr_id == IB_PMA_CLASS_PORT_INFO)
    pub 40)): *mut *mut return iboe_process_mad_port_info((void )(out_mad->data +,
    pub sizeof(counter_stats)): memset(&counter_stats, 0,,
    pub 1].mutex): mutex_lock(&dev->counters_table[port_num -,
    list_for_each_entry(tmp_counter,
    &dev.counters_table[port_num - 1].counters_list,
    list) {
    err = mlx4_get_counter_stats(dev.dev,
    tmp_counter.index,
    pub 0): &counter_stats,,
    if (err) {
    pub IB_MAD_RESULT_FAILURE: err =,
    pub 0: stats_avail =,
    }
    pub 1: stats_avail =,
    }
    pub 1].mutex): mutex_unlock(&dev->counters_table[port_num -,
    if (stats_avail) {
    switch (counter_stats.counter_mode & 0xf) {
    case 0:
    edit_counter(&counter_stats,
    (void *)(out_mad.data + 40),
    pub IB_MAD_RESULT_REPLY: err = IB_MAD_RESULT_SUCCESS |,
    default:
    pub IB_MAD_RESULT_FAILURE: err =,
    }
    }
    pub err: return,
    }
    int mlx4_ib_process_mad(struct ib_device *ibdev, int mad_flags, u32 port_num,
    const struct ib_wc *in_wc, const struct ib_grh *in_grh,
    const struct ib_mad *in, struct ib_mad *out,
    size_t *out_mad_size, u16 *out_mad_pkey_index)
    {
    pub to_mdev(ibdev): *mut *mut mlx4_ib_dev dev =,
    pub port_num): enum rdma_link_layer link = rdma_port_get_link_layer(ibdev,,
// iboe_process_mad() which uses the HCA flow-counters to implement IB PMA
// queries, should be called only by VFs and for that specific purpose
//
    if (link == IB_LINK_LAYER_INFINIBAND) {
    if (mlx4_is_slave(dev.dev) &&
    (in.mad_hdr.mgmt_class == IB_MGMT_CLASS_PERF_MGMT &&
    (in.mad_hdr.attr_id == IB_PMA_PORT_COUNTERS ||
    in.mad_hdr.attr_id == IB_PMA_PORT_COUNTERS_EXT ||
    in.mad_hdr.attr_id == IB_PMA_CLASS_PORT_INFO)))
    return iboe_process_mad(ibdev, mad_flags, port_num,
    pub out): in_wc, in_grh, in,,
    return ib_process_mad(ibdev, mad_flags, port_num, in_wc, in_grh,
    pub out): in,,
    }
    if (link == IB_LINK_LAYER_ETHERNET)
    return iboe_process_mad(ibdev, mad_flags, port_num, in_wc,
    pub out): in_grh, in,,
    pub -EINVAL: return,
    }
    static void send_handler(struct ib_mad_agent *agent,
    struct ib_mad_send_wc *mad_send_wc)
    {
    if (mad_send_wc.send_buf.context[0])
    pub 0): rdma_destroy_ah(mad_send_wc->send_buf->context[0],,
    }
#[no_mangle]
pub unsafe extern "C" fn mlx4_ib_mad_init(dev: *mut mlx4_ib_dev) -> c_int {
    int mlx4_ib_mad_init(struct mlx4_ib_dev *dev)
    {
    pub agent: *mut ib_mad_agent,
    pub q: int p,,
    pub ret: c_int,
    pub ll: enum rdma_link_layer,
    pub {: for (p = 0; p < dev->num_ports; ++p),
    pub 1): ll = rdma_port_get_link_layer(&dev->ib_dev, p +,
    pub {: for (q = 0; q <= 1; ++q),
    if (ll == IB_LINK_LAYER_INFINIBAND) {
    agent = ib_register_mad_agent(&dev.ib_dev, p + 1,
    q ? IB_QPT_GSI : IB_QPT_SMI,
    core::ptr::null_mut(), 0, send_handler,
    pub 0): NULL, NULL,,
    if (IS_ERR(agent)) {
    pub PTR_ERR(agent): ret =,
    pub err: goto,
    }
    pub agent: dev->send_agent[p][q] =,
    } else
    pub NULL: dev->send_agent[p][q] =,
    }
    }
    pub 0: return,
    err:
    pub ++p): for (p = 0; p < dev->num_ports;,
    pub ++q): for (q = 0; q <= 1;,
    if (dev.send_agent[p][q])
    pub ret: return,
    }
#[no_mangle]
pub unsafe extern "C" fn mlx4_ib_mad_cleanup(dev: *mut mlx4_ib_dev) {
    void mlx4_ib_mad_cleanup(struct mlx4_ib_dev *dev)
    {
    pub agent: *mut ib_mad_agent,
    pub q: int p,,
    pub {: for (p = 0; p < dev->num_ports; ++p),
    pub {: for (q = 0; q <= 1; ++q),
    pub dev->send_agent[p][q]: agent =,
    if (agent) {
    pub NULL: dev->send_agent[p][q] =,
    }
    }
    if (dev.sm_ah[p])
    pub 0): rdma_destroy_ah(dev->sm_ah[p],,
    }
    }
#[no_mangle]
unsafe extern "C" fn handle_lid_change_event(dev: *mut mlx4_ib_dev, port_num: u32) {
    static void handle_lid_change_event(struct mlx4_ib_dev *dev, u32 port_num)
    {
    pub IB_EVENT_LID_CHANGE): mlx4_ib_dispatch_event(dev, port_num,,
    if (mlx4_is_master(dev.dev) && !dev.sriov.is_going_down)
    mlx4_gen_slaves_port_mgt_ev(dev.dev, port_num,
    }
#[no_mangle]
unsafe extern "C" fn handle_client_rereg_event(dev: *mut mlx4_ib_dev, port_num: u32) {
    static void handle_client_rereg_event(struct mlx4_ib_dev *dev, u32 port_num)
    {
// re-configure the alias-guid and mcg's
    if (mlx4_is_master(dev.dev)) {
    pub port_num): mlx4_ib_invalidate_all_guid_record(dev,,
    if (!dev.sriov.is_going_down) {
    pub 0): mlx4_ib_mcg_port_cleanup(&dev->sriov.demux[port_num - 1],,
    mlx4_gen_slaves_port_mgt_ev(dev.dev, port_num,
    }
    }
// Update the sl to vl table from inside client rereg
// only if in secure-host mode (snooping is not possible)
// and the sl-to-vl change event is not generated by FW.
//
    if (!mlx4_is_slave(dev.dev) &&
    dev.dev.flags & MLX4_FLAG_SECURE_HOST &&
    !(dev.dev.caps.flags2 & MLX4_DEV_CAP_FLAG2_SL_TO_VL_CHANGE_EVENT)) {
    if (mlx4_is_master(dev.dev))
// already in work queue from mlx4_ib_event queueing
// mlx4_handle_port_mgmt_change_event, which calls
// this procedure. Therefore, call sl2vl_update directly.
//
    pub port_num): mlx4_ib_sl2vl_update(dev,,
    else
    pub port_num): mlx4_sched_ib_sl2vl_update_work(dev,,
    }
    pub IB_EVENT_CLIENT_REREGISTER): mlx4_ib_dispatch_event(dev, port_num,,
    }
    static void propagate_pkey_ev(struct mlx4_ib_dev *dev, int port_num,
    struct mlx4_eqe *eqe)
    {
    __propagate_pkey_ev(dev, port_num, GET_BLK_PTR_FROM_EQE(eqe),
    }
    static void handle_slaves_guid_change(struct mlx4_ib_dev *dev, u32 port_num,
    u32 guid_tbl_blk_num, u32 change_bitmap)
    {
    pub NULL: *mut *mut ib_smp in_mad =,
    pub NULL: *mut *mut ib_smp out_mad =,
    pub i: u16,
    if (!mlx4_is_mfunc(dev.dev) || !mlx4_is_master(dev.dev))
    pub kmalloc_obj(*in_mad): *mut in_mad =,
    pub kmalloc_obj(*out_mad): *mut out_mad =,
    if (!in_mad || !out_mad)
    pub out: goto,
    pub 4: *mut *mut guid_tbl_blk_num =,
    pub {: for (i = 0; i < 4; i++),
    if (change_bitmap && (!((change_bitmap >> (8 * i)) & 0xff)))
    pub in_mad): *mut memset(in_mad, 0, sizeof,
    pub out_mad): *mut memset(out_mad, 0, sizeof,
    pub 1: in_mad->base_version =,
    pub IB_MGMT_CLASS_SUBN_LID_ROUTED: in_mad->mgmt_class =,
    pub 1: in_mad->class_version =,
    pub IB_MGMT_METHOD_GET: in_mad->method =,
    pub IB_SMP_ATTR_GUID_INFO: in_mad->attr_id =,
    pub i): in_mad->attr_mod = cpu_to_be32(guid_tbl_blk_num +,
    if (mlx4_MAD_IFC(dev,
    MLX4_MAD_IFC_IGNORE_KEYS | MLX4_MAD_IFC_NET_VIEW,
    port_num, core::ptr::null_mut(), core::ptr::null_mut(), in_mad, out_mad)) {
    pub MAD_IFC\n"): mlx4_ib_warn(&dev->ib_dev, "Failed in get GUID INFO,
    pub out: goto,
    }
    mlx4_ib_update_cache_on_guid_change(dev, guid_tbl_blk_num + i,
    port_num,
    pub )out_mad)->data)): *mut *mut (u8 )(&((struct ib_smp,
    mlx4_ib_notify_slaves_on_guid_change(dev, guid_tbl_blk_num + i,
    port_num,
    pub )out_mad)->data)): *mut *mut (u8 )(&((struct ib_smp,
    }
    out:
    }
#[no_mangle]
pub unsafe extern "C" fn handle_port_mgmt_change_event(work: *mut work_struct) {
    void handle_port_mgmt_change_event(struct work_struct *work)
    {
    pub work): *mut *mut ib_event_work ew = container_of(work, ib_event_work,,
    pub ew->ib_dev: *mut *mut mlx4_ib_dev dev =,
    pub &(ew->ib_eqe): *mut *mut mlx4_eqe eqe =,
    pub eqe->event.port_mgmt_change.port: u32 port =,
    pub changed_attr: u32,
    pub tbl_block: u32,
    pub change_bitmap: u32,
    switch (eqe.subtype) {
    case MLX4_DEV_PMC_SUBTYPE_PORT_INFO:
    pub be32_to_cpu(eqe->event.port_mgmt_change.params.port_info.changed_attr): changed_attr =,
// Update the SM ah - This should be done before handling
    the other changed attributes so that MADs can be sent to the SM */
    if (changed_attr & MSTR_SM_CHANGE_MASK) {
    pub be16_to_cpu(eqe->event.port_mgmt_change.params.port_info.mstr_sm_lid): u16 lid =,
    pub 0xf: u8 sl = eqe->event.port_mgmt_change.params.port_info.mstr_sm_sl &,
    pub sl): update_sm_ah(dev, port, lid,,
    }
// Check if it is a lid change event
    if (changed_attr & MLX4_EQ_PORT_INFO_LID_CHANGE_MASK)
    pub port): handle_lid_change_event(dev,,
// Generate GUID changed event
    if (changed_attr & MLX4_EQ_PORT_INFO_GID_PFX_CHANGE_MASK) {
    if (mlx4_is_master(dev.dev)) {
    pub gid: union ib_gid,
    pub 0: int err =,
    if (!eqe.event.port_mgmt_change.params.port_info.gid_prefix)
    pub 1): err = __mlx4_ib_query_gid(&dev->ib_dev, port, 0, &gid,,
    else
    gid.global.subnet_prefix =
    if (err) {
    pr_warn("Could not change QP1 subnet prefix for port %d: query_gid error (%d)\n",
    pub err): port,,
    } else {
    pr_debug("Changing QP1 subnet prefix for port %d. old=0x%llx. new=0x%llx\n",
    port,
    (u64)atomic64_read(&dev.sriov.demux[port - 1].subnet_prefix),
    atomic64_set(&dev.sriov.demux[port - 1].subnet_prefix,
    }
    }
    pub IB_EVENT_GID_CHANGE): mlx4_ib_dispatch_event(dev, port,,
// if master, notify all slaves
    if (mlx4_is_master(dev.dev))
    mlx4_gen_slaves_port_mgt_ev(dev.dev, port,
    }
    if (changed_attr & MLX4_EQ_PORT_INFO_CLIENT_REREG_MASK)
    pub port): handle_client_rereg_event(dev,,
    case MLX4_DEV_PMC_SUBTYPE_PKEY_TABLE:
    pub IB_EVENT_PKEY_CHANGE): mlx4_ib_dispatch_event(dev, port,,
    if (mlx4_is_master(dev.dev) && !dev.sriov.is_going_down)
    pub eqe): propagate_pkey_ev(dev, port,,
    case MLX4_DEV_PMC_SUBTYPE_GUID_INFO:
// paravirtualized master's guid is guid 0 -- does not change
    if (!mlx4_is_master(dev.dev))
    pub IB_EVENT_GID_CHANGE): mlx4_ib_dispatch_event(dev, port,,
// if master, notify relevant slaves
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !dev->sriov.is_going_down) -> else {
    pub GET_BLK_PTR_FROM_EQE(eqe): tbl_block =,
    pub GET_MASK_FROM_EQE(eqe): change_bitmap =,
    pub change_bitmap): handle_slaves_guid_change(dev, port, tbl_block,,
    }
    case MLX4_DEV_PMC_SUBTYPE_SL_TO_VL_MAP:
// cache sl to vl mapping changes for use in
// filling QP1 LRH VL field when sending packets
//
    if (!mlx4_is_slave(dev.dev)) {
    pub sl2vl64: union sl2vl_tbl_to_u64,
    pub jj: c_int,
    pub {: for (jj = 0; jj < 8; jj++),
    sl2vl64.sl8[jj] =
    pr_debug("port %u, sl2vl[%d] = %02x\n",
    pub sl2vl64.sl8[jj]): port, jj,,
    }
    pub sl2vl64.sl64): atomic64_set(&dev->sl2vl[port - 1],,
    }
    default:
    pr_warn("Unsupported subtype 0x%x for "
    pub eqe->subtype): "Port Management Change event\n",,
    }
    }
    void mlx4_ib_dispatch_event(struct mlx4_ib_dev *dev, u32 port_num,
    enum ib_event_type type)
    {
    pub event: ib_event,
    pub &dev->ib_dev: event.device =,
    pub port_num: event.element.port_num =,
    pub type: event.event =,
    }
#[no_mangle]
unsafe extern "C" fn mlx4_ib_tunnel_comp_handler(cq: *mut ib_cq, arg: *mut c_void) {
    static void mlx4_ib_tunnel_comp_handler(struct ib_cq *cq, void *arg)
    {
    pub flags: c_ulong,
    pub cq->cq_context: *mut *mut mlx4_ib_demux_pv_ctx ctx =,
    pub to_mdev(ctx->ib_dev): *mut *mut mlx4_ib_dev dev =,
    pub flags): spin_lock_irqsave(&dev->sriov.going_down_lock,,
    if (!dev.sriov.is_going_down && ctx.state == DEMUX_PV_STATE_ACTIVE)
    pub &ctx->work): queue_work(ctx->wq,,
    pub flags): spin_unlock_irqrestore(&dev->sriov.going_down_lock,,
    }
#[no_mangle]
unsafe extern "C" fn mlx4_ib_wire_comp_handler(cq: *mut ib_cq, arg: *mut c_void) {
    static void mlx4_ib_wire_comp_handler(struct ib_cq *cq, void *arg)
    {
    pub flags: c_ulong,
    pub cq->cq_context: *mut *mut mlx4_ib_demux_pv_ctx ctx =,
    pub to_mdev(ctx->ib_dev): *mut *mut mlx4_ib_dev dev =,
    pub flags): spin_lock_irqsave(&dev->sriov.going_down_lock,,
    if (!dev.sriov.is_going_down && ctx.state == DEMUX_PV_STATE_ACTIVE)
    pub &ctx->work): queue_work(ctx->wi_wq,,
    pub flags): spin_unlock_irqrestore(&dev->sriov.going_down_lock,,
    }
    static int mlx4_ib_post_pv_qp_buf(struct mlx4_ib_demux_pv_ctx *ctx,
    struct mlx4_ib_demux_pv_qp *tun_qp,
    int index)
    {
    pub sg_list: ib_sge,
    pub recv_wr: ib_recv_wr,
    pub bad_recv_wr: *const ib_recv_wr,
    pub size: c_int,
    size = (tun_qp.qp.qp_type == IB_QPT_UD) ?
    pub mlx4_mad_rcv_buf): sizeof (struct mlx4_tunnel_mad) : sizeof (struct,
    pub tun_qp->ring[index].map: sg_list.addr =,
    pub size: sg_list.length =,
    pub ctx->pd->local_dma_lkey: sg_list.lkey =,
    pub NULL: recv_wr.next =,
    pub &sg_list: recv_wr.sg_list =,
    pub 1: recv_wr.num_sge =,
    recv_wr.wr_id = (u64) index | MLX4_TUN_WRID_RECV |
    ib_dma_sync_single_for_device(ctx.ib_dev, tun_qp.ring[index].map,
    pub DMA_FROM_DEVICE): size,,
    pub &bad_recv_wr): return ib_post_recv(tun_qp->qp, &recv_wr,,
    }
    static int mlx4_ib_multiplex_sa_handler(struct ib_device *ibdev, int port,
    int slave, struct ib_sa_mad *sa_mad)
    {
    pub 0: int ret =,
// dispatch to different sa handlers
    switch (be16_to_cpu(sa_mad.mad_hdr.attr_id)) {
    case IB_SA_ATTR_MC_MEMBER_REC:
    pub sa_mad): ret = mlx4_ib_mcg_multiplex_handler(ibdev, port, slave,,
    default:
    }
    pub ret: return,
    }
    int mlx4_ib_send_to_wire(struct mlx4_ib_dev *dev, int slave, u32 port,
    enum ib_qp_type dest_qpt, u16 pkey_index,
    u32 remote_qpn, u32 qkey, struct rdma_ah_attr *attr,
    u8 *s_mac, u16 vlan_id, struct ib_mad *mad)
    {
    pub list: ib_sge,
    pub wr: ib_ud_wr,
    pub bad_wr: *const ib_send_wr,
    pub sqp_ctx: *mut mlx4_ib_demux_pv_ctx,
    pub sqp: *mut mlx4_ib_demux_pv_qp,
    pub sqp_mad: *mut mlx4_mad_snd_buf,
    pub ah: *mut ib_ah,
    pub NULL: *mut *mut ib_qp send_qp =,
    pub 0: unsigned wire_tx_ix =,
    pub wire_pkey_ix: u16,
    pub src_qpnum: c_int,
    pub ret: c_int,
    pub dev->sriov.sqps[port-1]: sqp_ctx =,
// check if proxy qp created
    if (!sqp_ctx || sqp_ctx.state != DEMUX_PV_STATE_ACTIVE)
    pub -EAGAIN: return,
    if (dest_qpt == IB_QPT_SMI) {
    pub 0: src_qpnum =,
    pub &sqp_ctx->qp[0]: sqp =,
    pub 1][0]: wire_pkey_ix = dev->pkeys.virt2phys_pkey[slave][port -,
    } else {
    pub 1: src_qpnum =,
    pub &sqp_ctx->qp[1]: sqp =,
    pub 1][pkey_index]: wire_pkey_ix = dev->pkeys.virt2phys_pkey[slave][port -,
    }
    pub sqp->qp: send_qp =,
    pub ib_ah): ah = rdma_zalloc_drv_obj(sqp_ctx->pd->device,,
    if (!ah)
    pub -ENOMEM: return,
    pub sqp_ctx->pd->device: ah->device =,
    pub sqp_ctx->pd: ah->pd =,
// create ah
    ret = mlx4_ib_create_ah_slave(ah, attr,
    rdma_ah_retrieve_grh(attr).sgid_index,
    pub vlan_id): s_mac,,
    if (ret)
    pub out: goto,
    if (sqp.tx_ix_head - sqp.tx_ix_tail >=
    (MLX4_NUM_WIRE_BUFS - 1))
    pub -EAGAIN: ret =,
    else
    pub 1): wire_tx_ix = (++sqp->tx_ix_head) & (MLX4_NUM_WIRE_BUFS -,
    if (ret)
    pub out: goto,
    pub (sqp->tx_ring[wire_tx_ix].buf.addr): *mut *mut sqp_mad = (struct mlx4_mad_snd_buf ),
    pub ah: sqp->tx_ring[wire_tx_ix].ah =,
    ib_dma_sync_single_for_cpu(&dev.ib_dev,
    sqp.tx_ring[wire_tx_ix].buf.map,
    sizeof (struct mlx4_mad_snd_buf),
    pub mad): *mut memcpy(&sqp_mad->payload, mad, sizeof,
    ib_dma_sync_single_for_device(&dev.ib_dev,
    sqp.tx_ring[wire_tx_ix].buf.map,
    sizeof (struct mlx4_mad_snd_buf),
    pub sqp->tx_ring[wire_tx_ix].buf.map: list.addr =,
    pub mlx4_mad_snd_buf): list.length = sizeof (struct,
    pub sqp_ctx->pd->local_dma_lkey: list.lkey =,
    pub ah: wr.ah =,
    pub port: wr.port_num =,
    pub wire_pkey_ix: wr.pkey_index =,
    pub qkey: wr.remote_qkey =,
    pub remote_qpn: wr.remote_qpn =,
    pub NULL: wr.wr.next =,
    pub MLX4_TUN_SET_WRID_QPN(src_qpnum): wr.wr.wr_id = ((u64) wire_tx_ix) |,
    pub &list: wr.wr.sg_list =,
    pub 1: wr.wr.num_sge =,
    pub IB_WR_SEND: wr.wr.opcode =,
    pub IB_SEND_SIGNALED: wr.wr.send_flags =,
    pub &bad_wr): ret = ib_post_send(send_qp, &wr.wr,,
    if (!ret)
    pub 0: return,
    pub NULL: sqp->tx_ring[wire_tx_ix].ah =,
    out:
    pub ret: return,
    }
#[no_mangle]
unsafe extern "C" fn get_slave_base_gid_ix(dev: *mut mlx4_ib_dev, slave: c_int, port: c_int) -> c_int {
    static int get_slave_base_gid_ix(struct mlx4_ib_dev *dev, int slave, int port)
    {
    if (rdma_port_get_link_layer(&dev.ib_dev, port) == IB_LINK_LAYER_INFINIBAND)
    pub slave: return,
    pub port): return mlx4_get_base_gid_ix(dev->dev, slave,,
    }
    static void fill_in_real_sgid_index(struct mlx4_ib_dev *dev, int slave, int port,
    struct rdma_ah_attr *ah_attr)
    {
    pub rdma_ah_retrieve_grh(ah_attr): *mut *mut ib_global_route grh =,
    if (rdma_port_get_link_layer(&dev.ib_dev, port) == IB_LINK_LAYER_INFINIBAND)
    pub slave: grh->sgid_index =,
    else
    pub port): grh->sgid_index += get_slave_base_gid_ix(dev, slave,,
    }
#[no_mangle]
unsafe extern "C" fn mlx4_ib_multiplex_mad(ctx: *mut mlx4_ib_demux_pv_ctx, wc: *mut ib_wc) {
    static void mlx4_ib_multiplex_mad(struct mlx4_ib_demux_pv_ctx *ctx, struct ib_wc *wc)
    {
    pub to_mdev(ctx->ib_dev): *mut *mut mlx4_ib_dev dev =,
    pub &ctx->qp[MLX4_TUN_WRID_QPN(wc->wr_id)]: *mut *mut mlx4_ib_demux_pv_qp tun_qp =,
    pub 1): int wr_ix = wc->wr_id & (MLX4_NUM_TUNNEL_BUFS -,
    pub tun_qp->ring[wr_ix].addr: *mut *mut mlx4_tunnel_mad tunnel =,
    pub ah: mlx4_ib_ah,
    pub ah_attr: rdma_ah_attr,
    pub slave_id: *mut u8,
    pub slave: c_int,
    pub port: c_int,
    pub vlan_id: u16,
    pub qos: u8,
    pub dmac: *mut u8,
    pub sts: c_int,
// Get slave that sent this packet
    if (wc.src_qp < dev.dev.phys_caps.base_proxy_sqpn ||
    wc.src_qp >= dev.dev.phys_caps.base_proxy_sqpn + 8 * MLX4_MFUNC_MAX ||
    (wc.src_qp & 0x1) != ctx.port - 1 ||
    wc.src_qp & 0x4) {
    pub wc->src_qp): mlx4_ib_warn(ctx->ib_dev, "can't multiplex bad sqp:%d\n",,
    }
    pub 8: slave = ((wc->src_qp & ~0x7) - dev->dev->phys_caps.base_proxy_sqpn) /,
    if (slave != ctx.slave) {
    mlx4_ib_warn(ctx.ib_dev, "can't multiplex bad sqp:%d: "
    pub wc->src_qp): "belongs to another slave\n",,
    }
// Map transaction ID
    ib_dma_sync_single_for_cpu(ctx.ib_dev, tun_qp.ring[wr_ix].map,
    sizeof (struct mlx4_tunnel_mad),
    switch (tunnel.mad.mad_hdr.method) {
    case IB_MGMT_METHOD_SET:
    case IB_MGMT_METHOD_GET:
    case IB_MGMT_METHOD_REPORT:
    case IB_SA_METHOD_GET_TABLE:
    case IB_SA_METHOD_DELETE:
    case IB_SA_METHOD_GET_MULTI:
    case IB_SA_METHOD_GET_TRACE_TBL:
    pub &tunnel->mad.mad_hdr.tid: *mut *mut slave_id = (u8 ),
    if (*slave_id) {
    mlx4_ib_warn(ctx.ib_dev, "egress mad has non-null tid msb:%d "
    "class:%d slave:%d\n", *slave_id,
    pub slave): tunnel->mad.mad_hdr.mgmt_class,,
    } else
// slave_id = slave;
    default:
// nothing */;
    }
// Class-specific handling
    switch (tunnel.mad.mad_hdr.mgmt_class) {
    case IB_MGMT_CLASS_SUBN_LID_ROUTED:
    case IB_MGMT_CLASS_SUBN_DIRECTED_ROUTE:
    if (slave != mlx4_master_func_num(dev.dev) &&
    !mlx4_vf_smi_enabled(dev.dev, slave, ctx.port))
    case IB_MGMT_CLASS_SUBN_ADM:
    if (mlx4_ib_multiplex_sa_handler(ctx.ib_dev, ctx.port, slave,
    (struct ib_sa_mad *) &tunnel.mad))
    case IB_MGMT_CLASS_CM:
    if (mlx4_ib_multiplex_cm_handler(ctx.ib_dev, ctx.port, slave,
    (struct ib_mad *) &tunnel.mad))
    case IB_MGMT_CLASS_DEVICE_MGMT:
    if (tunnel.mad.mad_hdr.method != IB_MGMT_METHOD_GET &&
    tunnel.mad.mad_hdr.method != IB_MGMT_METHOD_SET)
    default:
// Drop unsupported classes for slaves in tunnel mode
    if (slave != mlx4_master_func_num(dev.dev)) {
    mlx4_ib_warn(ctx.ib_dev, "dropping unsupported egress mad from class:%d "
    pub slave): "for slave:%d\n", tunnel->mad.mad_hdr.mgmt_class,,
    }
    }
// We are using standard ib_core services to send the mad, so generate a
// stadard address handle by decoding the tunnelled mlx4_ah fields
    pub mlx4_av)): memcpy(&ah.av, &tunnel->hdr.av, sizeof (struct,
    pub ctx->ib_dev: ah.ibah.device =,
    pub 24: port = be32_to_cpu(ah.av.ib.port_pd) >>,
    pub port): port = mlx4_slave_convert_port(dev->dev, slave,,
    if (port < 0)
    pub 0xffffff)): ah.av.ib.port_pd = cpu_to_be32(port << 24 | (be32_to_cpu(ah.av.ib.port_pd) &,
    pub port): ah.ibah.type = rdma_ah_find_type(&dev->ib_dev,,
    pub &ah_attr): mlx4_ib_query_ah(&ah.ibah,,
    if (rdma_ah_get_ah_flags(&ah_attr) & IB_AH_GRH)
    pub &ah_attr): fill_in_real_sgid_index(dev, slave, ctx->port,,
    pub rdma_ah_retrieve_dmac(&ah_attr): dmac =,
    if (dmac)
    pub ETH_ALEN): memcpy(dmac, tunnel->hdr.mac,,
    pub be16_to_cpu(tunnel->hdr.vlan): vlan_id =,
// if slave have default vlan use it
    if (mlx4_get_slave_default_vlan(dev.dev, ctx.port, slave,
    &vlan_id, &qos))
    pub qos): rdma_ah_set_sl(&ah_attr,,
    sts = mlx4_ib_send_to_wire(dev, slave, ctx.port,
    is_proxy_qp0(dev, wc.src_qp, slave) ?
    IB_QPT_SMI : IB_QPT_GSI,
    be16_to_cpu(tunnel.hdr.pkey_index),
    be32_to_cpu(tunnel.hdr.remote_qpn),
    be32_to_cpu(tunnel.hdr.qkey),
    pub &tunnel->mad): &ah_attr, wc->smac, vlan_id,,
    if (sts)
    pr_debug("failed sending %s to wire on behalf of slave %d (%d)\n",
    is_proxy_qp0(dev, wc.src_qp, slave) ? "SMI" : "GSI",
    pub sts): slave,,
    }
    static int mlx4_ib_alloc_pv_bufs(struct mlx4_ib_demux_pv_ctx *ctx,
    enum ib_qp_type qp_type, int is_tun)
    {
    pub i: c_int,
    pub tun_qp: *mut mlx4_ib_demux_pv_qp,
    pub tx_buf_size: int rx_buf_size,,
    pub MLX4_NUM_WIRE_BUFS: int nmbr_bufs = is_tun ? MLX4_NUM_TUNNEL_BUFS :,
    if (qp_type > IB_QPT_GSI)
    pub -EINVAL: return,
    pub &ctx->qp[qp_type]: tun_qp =,
    pub nmbr_bufs): tun_qp->ring = kzalloc_objs(struct mlx4_ib_buf,,
    if (!tun_qp.ring)
    pub -ENOMEM: return,
    pub nmbr_bufs): tun_qp->tx_ring = kzalloc_objs(struct mlx4_ib_tun_tx_buf,,
    if (!tun_qp.tx_ring) {
    pub NULL: tun_qp->ring =,
    pub -ENOMEM: return,
    }
    if (is_tun) {
    pub mlx4_tunnel_mad): rx_buf_size = sizeof (struct,
    pub mlx4_rcv_tunnel_mad): tx_buf_size = sizeof (struct,
    } else {
    pub mlx4_mad_rcv_buf): rx_buf_size = sizeof (struct,
    pub mlx4_mad_snd_buf): tx_buf_size = sizeof (struct,
    }
    pub {: for (i = 0; i < nmbr_bufs; i++),
    pub GFP_KERNEL): tun_qp->ring[i].addr = kmalloc(rx_buf_size,,
    if (!tun_qp.ring[i].addr)
    pub err: goto,
    tun_qp.ring[i].map = ib_dma_map_single(ctx.ib_dev,
    tun_qp.ring[i].addr,
    rx_buf_size,
    if (ib_dma_mapping_error(ctx.ib_dev, tun_qp.ring[i].map)) {
    pub err: goto,
    }
    }
    pub {: for (i = 0; i < nmbr_bufs; i++),
    tun_qp.tx_ring[i].buf.addr =
    pub GFP_KERNEL): kmalloc(tx_buf_size,,
    if (!tun_qp.tx_ring[i].buf.addr)
    pub tx_err: goto,
    tun_qp.tx_ring[i].buf.map =
    ib_dma_map_single(ctx.ib_dev,
    tun_qp.tx_ring[i].buf.addr,
    tx_buf_size,
    if (ib_dma_mapping_error(ctx.ib_dev,
    tun_qp.tx_ring[i].buf.map)) {
    pub tx_err: goto,
    }
    pub NULL: tun_qp->tx_ring[i].ah =,
    }
    pub 0: tun_qp->tx_ix_head =,
    pub 0: tun_qp->tx_ix_tail =,
    pub qp_type: tun_qp->proxy_qpt =,
    pub 0: return,
    tx_err:
    while (i > 0) {
    ib_dma_unmap_single(ctx.ib_dev, tun_qp.tx_ring[i].buf.map,
    pub DMA_TO_DEVICE): tx_buf_size,,
    }
    pub nmbr_bufs: i =,
    err:
    while (i > 0) {
    ib_dma_unmap_single(ctx.ib_dev, tun_qp.ring[i].map,
    pub DMA_FROM_DEVICE): rx_buf_size,,
    }
    pub NULL: tun_qp->tx_ring =,
    pub NULL: tun_qp->ring =,
    pub -ENOMEM: return,
    }
    static void mlx4_ib_free_pv_qp_bufs(struct mlx4_ib_demux_pv_ctx *ctx,
    enum ib_qp_type qp_type, int is_tun)
    {
    pub i: c_int,
    pub tun_qp: *mut mlx4_ib_demux_pv_qp,
    pub tx_buf_size: int rx_buf_size,,
    pub MLX4_NUM_WIRE_BUFS: int nmbr_bufs = is_tun ? MLX4_NUM_TUNNEL_BUFS :,
    if (qp_type > IB_QPT_GSI)
    pub &ctx->qp[qp_type]: tun_qp =,
    if (is_tun) {
    pub mlx4_tunnel_mad): rx_buf_size = sizeof (struct,
    pub mlx4_rcv_tunnel_mad): tx_buf_size = sizeof (struct,
    } else {
    pub mlx4_mad_rcv_buf): rx_buf_size = sizeof (struct,
    pub mlx4_mad_snd_buf): tx_buf_size = sizeof (struct,
    }
    pub {: for (i = 0; i < nmbr_bufs; i++),
    ib_dma_unmap_single(ctx.ib_dev, tun_qp.ring[i].map,
    pub DMA_FROM_DEVICE): rx_buf_size,,
    }
    pub {: for (i = 0; i < nmbr_bufs; i++),
    ib_dma_unmap_single(ctx.ib_dev, tun_qp.tx_ring[i].buf.map,
    pub DMA_TO_DEVICE): tx_buf_size,,
    if (tun_qp.tx_ring[i].ah)
    pub 0): rdma_destroy_ah(tun_qp->tx_ring[i].ah,,
    }
    }
#[no_mangle]
unsafe extern "C" fn mlx4_ib_tunnel_comp_worker(work: *mut work_struct) {
    static void mlx4_ib_tunnel_comp_worker(struct work_struct *work)
    {
    pub ctx: *mut mlx4_ib_demux_pv_ctx,
    pub tun_qp: *mut mlx4_ib_demux_pv_qp,
    pub wc: ib_wc,
    pub ret: c_int,
    pub work): ctx = container_of(work, struct mlx4_ib_demux_pv_ctx,,
    pub IB_CQ_NEXT_COMP): ib_req_notify_cq(ctx->cq,,
    while (ib_poll_cq(ctx.cq, 1, &wc) == 1) {
    pub &ctx->qp[MLX4_TUN_WRID_QPN(wc.wr_id)]: tun_qp =,
    if (wc.status == IB_WC_SUCCESS) {
    switch (wc.opcode) {
    case IB_WC_RECV:
    pub &wc): mlx4_ib_multiplex_mad(ctx,,
    ret = mlx4_ib_post_pv_qp_buf(ctx, tun_qp,
    wc.wr_id &
    pub 1)): (MLX4_NUM_TUNNEL_BUFS -,
    if (ret)
    pr_err("Failed reposting tunnel "
    pub wc.wr_id): "buf:%lld\n",,
    case IB_WC_SEND:
    rdma_destroy_ah(tun_qp.tx_ring[wc.wr_id &
    pub 0): (MLX4_NUM_TUNNEL_BUFS - 1)].ah,,
    tun_qp.tx_ring[wc.wr_id & (MLX4_NUM_TUNNEL_BUFS - 1)].ah
    pub NULL: =,
    default:
    }
    } else  {
    pr_debug("mlx4_ib: completion error in tunnel: %d."
    " status = %d, wrid = 0x%llx\n",
    pub wc.wr_id): ctx->slave, wc.status,,
    if (!MLX4_TUN_IS_RECV(wc.wr_id)) {
    rdma_destroy_ah(tun_qp.tx_ring[wc.wr_id &
    pub 0): (MLX4_NUM_TUNNEL_BUFS - 1)].ah,,
    tun_qp.tx_ring[wc.wr_id & (MLX4_NUM_TUNNEL_BUFS - 1)].ah
    pub NULL: =,
    }
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn pv_qp_event_handler(event: *mut ib_event, qp_context: *mut c_void) {
    static void pv_qp_event_handler(struct ib_event *event, void *qp_context)
    {
    pub qp_context: *mut *mut mlx4_ib_demux_pv_ctx sqp =,
// It's worse than that! He's dead, Jim!
    pr_err("Fatal error (%d) on a MAD QP on port %d\n",
    pub sqp->port): event->event,,
    }
    static int create_pv_sqp(struct mlx4_ib_demux_pv_ctx *ctx,
    enum ib_qp_type qp_type, int create_tun)
    {
    pub ret: int i,,
    pub tun_qp: *mut mlx4_ib_demux_pv_qp,
    pub qp_init_attr: mlx4_ib_qp_tunnel_init_attr,
    pub attr: ib_qp_attr,
    pub qp_attr_mask_INIT: c_int,
    pub MLX4_NUM_WIRE_BUFS: int nmbr_bufs = create_tun ? MLX4_NUM_TUNNEL_BUFS :,
    if (qp_type > IB_QPT_GSI)
    pub -EINVAL: return,
    pub &ctx->qp[qp_type]: tun_qp =,
    pub qp_init_attr): memset(&qp_init_attr, 0, sizeof,
    pub ctx->cq: qp_init_attr.init_attr.send_cq =,
    pub ctx->cq: qp_init_attr.init_attr.recv_cq =,
    pub IB_SIGNAL_ALL_WR: qp_init_attr.init_attr.sq_sig_type =,
    pub nmbr_bufs: qp_init_attr.init_attr.cap.max_send_wr =,
    pub nmbr_bufs: qp_init_attr.init_attr.cap.max_recv_wr =,
    pub 1: qp_init_attr.init_attr.cap.max_send_sge =,
    pub 1: qp_init_attr.init_attr.cap.max_recv_sge =,
    if (create_tun) {
    pub IB_QPT_UD: qp_init_attr.init_attr.qp_type =,
    pub MLX4_IB_SRIOV_TUNNEL_QP: qp_init_attr.init_attr.create_flags =,
    pub ctx->port: qp_init_attr.port =,
    pub ctx->slave: qp_init_attr.slave =,
    pub qp_type: qp_init_attr.proxy_qp_type =,
    qp_attr_mask_INIT = IB_QP_STATE | IB_QP_PKEY_INDEX |
    pub IB_QP_PORT: IB_QP_QKEY |,
    } else {
    pub qp_type: qp_init_attr.init_attr.qp_type =,
    pub MLX4_IB_SRIOV_SQP: qp_init_attr.init_attr.create_flags =,
    pub IB_QP_QKEY: qp_attr_mask_INIT = IB_QP_STATE | IB_QP_PKEY_INDEX |,
    }
    pub ctx->port: qp_init_attr.init_attr.port_num =,
    pub ctx: qp_init_attr.init_attr.qp_context =,
    pub pv_qp_event_handler: qp_init_attr.init_attr.event_handler =,
    pub &qp_init_attr.init_attr): tun_qp->qp = ib_create_qp(ctx->pd,,
    if (IS_ERR(tun_qp.qp)) {
    pub PTR_ERR(tun_qp->qp): ret =,
    pr_err("Couldn't create %s QP (%pe)\n",
    pub tun_qp->qp): create_tun ? "tunnel" : "special",,
    pub NULL: tun_qp->qp =,
    pub ret: return,
    }
    pub attr): memset(&attr, 0, sizeof,
    pub IB_QPS_INIT: attr.qp_state =,
    pub 0: ret =,
    if (create_tun)
    ret = find_slave_port_pkey_ix(to_mdev(ctx.ib_dev), ctx.slave,
    ctx.port, IB_DEFAULT_PKEY_FULL,
    if (ret || !create_tun)
    attr.pkey_index =
    pub 1][0]: to_mdev(ctx->ib_dev)->pkeys.virt2phys_pkey[ctx->slave][ctx->port -,
    pub IB_QP1_QKEY: attr.qkey =,
    pub ctx->port: attr.port_num =,
    pub qp_attr_mask_INIT): ret = ib_modify_qp(tun_qp->qp, &attr,,
    if (ret) {
    pr_err("Couldn't change %s qp state to INIT (%d)\n",
    pub ret): create_tun ? "tunnel" : "special",,
    pub err_qp: goto,
    }
    pub IB_QPS_RTR: attr.qp_state =,
    pub IB_QP_STATE): ret = ib_modify_qp(tun_qp->qp, &attr,,
    if (ret) {
    pr_err("Couldn't change %s qp state to RTR (%d)\n",
    pub ret): create_tun ? "tunnel" : "special",,
    pub err_qp: goto,
    }
    pub IB_QPS_RTS: attr.qp_state =,
    pub 0: attr.sq_psn =,
    pub IB_QP_SQ_PSN): ret = ib_modify_qp(tun_qp->qp, &attr, IB_QP_STATE |,
    if (ret) {
    pr_err("Couldn't change %s qp state to RTS (%d)\n",
    pub ret): create_tun ? "tunnel" : "special",,
    pub err_qp: goto,
    }
    pub {: for (i = 0; i < nmbr_bufs; i++),
    pub i): ret = mlx4_ib_post_pv_qp_buf(ctx, tun_qp,,
    if (ret) {
    pr_err(" mlx4_ib_post_pv_buf error"
    pub i): " (err = %d, i = %d)\n", ret,,
    pub err_qp: goto,
    }
    }
    pub 0: return,
    err_qp:
    pub NULL: tun_qp->qp =,
    pub ret: return,
    }
//
// IB MAD completion callback for real SQPs
//
#[no_mangle]
unsafe extern "C" fn mlx4_ib_sqp_comp_worker(work: *mut work_struct) {
    static void mlx4_ib_sqp_comp_worker(struct work_struct *work)
    {
    pub ctx: *mut mlx4_ib_demux_pv_ctx,
    pub sqp: *mut mlx4_ib_demux_pv_qp,
    pub wc: ib_wc,
    pub grh: *mut ib_grh,
    pub mad: *mut ib_mad,
    pub work): ctx = container_of(work, struct mlx4_ib_demux_pv_ctx,,
    pub IB_CQ_NEXT_COMP): ib_req_notify_cq(ctx->cq,,
    while (mlx4_ib_poll_cq(ctx.cq, 1, &wc) == 1) {
    pub &ctx->qp[MLX4_TUN_WRID_QPN(wc.wr_id)]: sqp =,
    if (wc.status == IB_WC_SUCCESS) {
    switch (wc.opcode) {
    case IB_WC_SEND:
    kfree(sqp.tx_ring[wc.wr_id &
    pub 1)].ah): (MLX4_NUM_WIRE_BUFS -,
    sqp.tx_ring[wc.wr_id & (MLX4_NUM_WIRE_BUFS - 1)].ah
    pub NULL: =,
    case IB_WC_RECV:
    mad = (struct ib_mad *) &(((struct mlx4_mad_rcv_buf *)
    (sqp.ring[wc.wr_id &
    pub 1)].addr))->payload): (MLX4_NUM_WIRE_BUFS -,
    grh = &(((struct mlx4_mad_rcv_buf *)
    (sqp.ring[wc.wr_id &
    pub 1)].addr))->grh): (MLX4_NUM_WIRE_BUFS -,
    pub mad): mlx4_ib_demux_mad(ctx->ib_dev, ctx->port, &wc, grh,,
    if (mlx4_ib_post_pv_qp_buf(ctx, sqp, wc.wr_id &
    (MLX4_NUM_WIRE_BUFS - 1)))
    pr_err("Failed reposting SQP "
    pub wc.wr_id): "buf:%lld\n",,
    default:
    }
    } else  {
    pr_debug("mlx4_ib: completion error in tunnel: %d."
    " status = %d, wrid = 0x%llx\n",
    pub wc.wr_id): ctx->slave, wc.status,,
    if (!MLX4_TUN_IS_RECV(wc.wr_id)) {
    kfree(sqp.tx_ring[wc.wr_id &
    pub 1)].ah): (MLX4_NUM_WIRE_BUFS -,
    sqp.tx_ring[wc.wr_id & (MLX4_NUM_WIRE_BUFS - 1)].ah
    pub NULL: =,
    }
    }
    }
    }
    static int alloc_pv_object(struct mlx4_ib_dev *dev, int slave, int port,
    struct mlx4_ib_demux_pv_ctx **ret_ctx)
    {
    pub ctx: *mut mlx4_ib_demux_pv_ctx,
// ret_ctx = NULL;
    pub mlx4_ib_demux_pv_ctx): ctx = kzalloc_obj(struct,
    if (!ctx)
    pub -ENOMEM: return,
    pub &dev->ib_dev: ctx->ib_dev =,
    pub port: ctx->port =,
    pub slave: ctx->slave =,
// ret_ctx = ctx;
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn free_pv_object(dev: *mut mlx4_ib_dev, slave: c_int, port: c_int) {
    static void free_pv_object(struct mlx4_ib_dev *dev, int slave, int port)
    {
    if (dev.sriov.demux[port - 1].tun[slave]) {
    pub 1].tun[slave]): kfree(dev->sriov.demux[port -,
    pub NULL: dev->sriov.demux[port - 1].tun[slave] =,
    }
    }
    static int create_pv_resources(struct ib_device *ibdev, int slave, int port,
    int create_tun, struct mlx4_ib_demux_pv_ctx *ctx)
    {
    pub cq_size: int ret,,
    pub {}: ib_cq_init_attr cq_attr =,
    pub MLX4_NUM_WIRE_BUFS: int nmbr_bufs = create_tun ? MLX4_NUM_TUNNEL_BUFS :,
    if (ctx.state != DEMUX_PV_STATE_DOWN)
    pub -EEXIST: return,
    pub DEMUX_PV_STATE_STARTING: ctx->state =,
// have QP0 only if link layer is IB
    if (rdma_port_get_link_layer(ibdev, ctx.port) ==
    IB_LINK_LAYER_INFINIBAND)
    pub 1: ctx->has_smi =,
    if (ctx.has_smi) {
    pub create_tun): ret = mlx4_ib_alloc_pv_bufs(ctx, IB_QPT_SMI,,
    if (ret) {
    pub ret): pr_err("Failed allocating qp0 tunnel bufs (%d)\n",,
    pub err_out: goto,
    }
    }
    pub create_tun): ret = mlx4_ib_alloc_pv_bufs(ctx, IB_QPT_GSI,,
    if (ret) {
    pub ret): pr_err("Failed allocating qp1 tunnel bufs (%d)\n",,
    pub err_out_qp0: goto,
    }
    pub nmbr_bufs: *mut *mut cq_size = 2,
    if (ctx.has_smi)
    pub 2: *mut *mut cq_size =,
    pub cq_size: cq_attr.cqe =,
    ctx.cq = ib_create_cq(ctx.ib_dev,
    create_tun ? mlx4_ib_tunnel_comp_handler : mlx4_ib_wire_comp_handler,
    pub &cq_attr): NULL, ctx,,
    if (IS_ERR(ctx.cq)) {
    pub PTR_ERR(ctx->cq): ret =,
    pub ctx->cq): pr_err("Couldn't create tunnel CQ (%pe)\n",,
    pub err_buf: goto,
    }
    pub 0): ctx->pd = ib_alloc_pd(ctx->ib_dev,,
    if (IS_ERR(ctx.pd)) {
    pub PTR_ERR(ctx->pd): ret =,
    pub ctx->pd): pr_err("Couldn't create tunnel PD (%pe)\n",,
    pub err_cq: goto,
    }
    if (ctx.has_smi) {
    pub create_tun): ret = create_pv_sqp(ctx, IB_QPT_SMI,,
    if (ret) {
    pr_err("Couldn't create %s QP0 (%d)\n",
    pub ret): create_tun ? "tunnel for" : "",,
    pub err_pd: goto,
    }
    }
    pub create_tun): ret = create_pv_sqp(ctx, IB_QPT_GSI,,
    if (ret) {
    pr_err("Couldn't create %s QP1 (%d)\n",
    pub ret): create_tun ? "tunnel for" : "",,
    pub err_qp0: goto,
    }
    if (create_tun)
    pub mlx4_ib_tunnel_comp_worker): INIT_WORK(&ctx->work,,
    else
    pub mlx4_ib_sqp_comp_worker): INIT_WORK(&ctx->work,,
    pub 1].wq: ctx->wq = to_mdev(ibdev)->sriov.demux[port -,
    pub 1].wi_wq: ctx->wi_wq = to_mdev(ibdev)->sriov.demux[port -,
    pub IB_CQ_NEXT_COMP): ret = ib_req_notify_cq(ctx->cq,,
    if (ret) {
    pub ret): pr_err("Couldn't arm tunnel cq (%d)\n",,
    pub err_wq: goto,
    }
    pub DEMUX_PV_STATE_ACTIVE: ctx->state =,
    pub 0: return,
    err_wq:
    pub NULL: ctx->wq =,
    pub NULL: ctx->qp[1].qp =,
    err_qp0:
    if (ctx.has_smi)
    pub NULL: ctx->qp[0].qp =,
    err_pd:
    pub NULL: ctx->pd =,
    err_cq:
    pub NULL: ctx->cq =,
    err_buf:
    pub create_tun): mlx4_ib_free_pv_qp_bufs(ctx, IB_QPT_GSI,,
    err_out_qp0:
    if (ctx.has_smi)
    pub create_tun): mlx4_ib_free_pv_qp_bufs(ctx, IB_QPT_SMI,,
    err_out:
    pub DEMUX_PV_STATE_DOWN: ctx->state =,
    pub ret: return,
    }
    static void destroy_pv_resources(struct mlx4_ib_dev *dev, int slave, int port,
    struct mlx4_ib_demux_pv_ctx *ctx, int flush)
    {
    if (!ctx)
    if (ctx.state > DEMUX_PV_STATE_DOWN) {
    pub DEMUX_PV_STATE_DOWNING: ctx->state =,
    if (flush)
    if (ctx.has_smi) {
    pub NULL: ctx->qp[0].qp =,
    pub 1): mlx4_ib_free_pv_qp_bufs(ctx, IB_QPT_SMI,,
    }
    pub NULL: ctx->qp[1].qp =,
    pub 1): mlx4_ib_free_pv_qp_bufs(ctx, IB_QPT_GSI,,
    pub NULL: ctx->pd =,
    pub NULL: ctx->cq =,
    pub DEMUX_PV_STATE_DOWN: ctx->state =,
    }
    }
    static int mlx4_ib_tunnels_update(struct mlx4_ib_dev *dev, int slave,
    int port, int do_init)
    {
    pub 0: int ret =,
    if (!do_init) {
    pub slave): clean_vf_mcast(&dev->sriov.demux[port - 1],,
// for master, destroy real sqp resources
    if (slave == mlx4_master_func_num(dev.dev))
    destroy_pv_resources(dev, slave, port,
    pub 1): dev->sriov.sqps[port - 1],,
// destroy the tunnel qp resources
    destroy_pv_resources(dev, slave, port,
    pub 1): dev->sriov.demux[port - 1].tun[slave],,
    pub 0: return,
    }
// create the tunnel qp resources
    ret = create_pv_resources(&dev.ib_dev, slave, port, 1,
    pub 1].tun[slave]): dev->sriov.demux[port -,
// for master, create the real sqp resources
    if (!ret && slave == mlx4_master_func_num(dev.dev))
    ret = create_pv_resources(&dev.ib_dev, slave, port, 0,
    pub 1]): dev->sriov.sqps[port -,
    pub ret: return,
    }
#[no_mangle]
pub unsafe extern "C" fn mlx4_ib_tunnels_update_work(work: *mut work_struct) {
    void mlx4_ib_tunnels_update_work(struct work_struct *work)
    {
    pub dmxw: *mut mlx4_ib_demux_work,
    pub work): dmxw = container_of(work, struct mlx4_ib_demux_work,,
    mlx4_ib_tunnels_update(dmxw.dev, dmxw.slave, (int) dmxw.port,
    }
    static int mlx4_ib_alloc_demux_ctx(struct mlx4_ib_dev *dev,
    struct mlx4_ib_demux_ctx *ctx,
    int port)
    {
    pub 0: int ret =,
    pub i: c_int,
    ctx.tun = kzalloc_objs(struct mlx4_ib_demux_pv_ctx *,
    if (!ctx.tun)
    pub -ENOMEM: return,
    pub dev: ctx->dev =,
    pub port: ctx->port =,
    pub &dev->ib_dev: ctx->ib_dev =,
    pub 0: for (i =,
    i < min(dev.dev.caps.sqp_demux,
    pub 1)): (u16)(dev->dev->persist->num_vfs +,
    i++) {
    struct mlx4_active_ports actv_ports =
    pub i): mlx4_get_active_ports(dev->dev,,
    if (!test_bit(port - 1, actv_ports.ports))
    pub &ctx->tun[i]): ret = alloc_pv_object(dev, i, port,,
    if (ret) {
    pub -ENOMEM: ret =,
    pub err_mcg: goto,
    }
    }
    pub mlx4_ib_mcg_port_init(ctx): ret =,
    if (ret) {
    pub ret): pr_err("Failed initializing mcg para-virt (%d)\n",,
    pub err_mcg: goto,
    }
    pub port): ctx->wq = alloc_ordered_workqueue("mlx4_ibt%d", WQ_MEM_RECLAIM,,
    if (!ctx.wq) {
    pub port): pr_err("Failed to create tunnelling WQ for port %d\n",,
    pub -ENOMEM: ret =,
    pub err_wq: goto,
    }
    pub port): ctx->wi_wq = alloc_ordered_workqueue("mlx4_ibwi%d", WQ_MEM_RECLAIM,,
    if (!ctx.wi_wq) {
    pub port): pr_err("Failed to create wire WQ for port %d\n",,
    pub -ENOMEM: ret =,
    pub err_wiwq: goto,
    }
    pub port): ctx->ud_wq = alloc_ordered_workqueue("mlx4_ibud%d", WQ_MEM_RECLAIM,,
    if (!ctx.ud_wq) {
    pub port): pr_err("Failed to create up/down WQ for port %d\n",,
    pub -ENOMEM: ret =,
    pub err_udwq: goto,
    }
    pub 0: return,
    err_udwq:
    pub NULL: ctx->wi_wq =,
    err_wiwq:
    pub NULL: ctx->wq =,
    err_wq:
    pub 1): mlx4_ib_mcg_port_cleanup(ctx,,
    err_mcg:
    pub i++): for (i = 0; i < dev->dev->caps.sqp_demux;,
    pub port): free_pv_object(dev, i,,
    pub NULL: ctx->tun =,
    pub ret: return,
    }
#[no_mangle]
unsafe extern "C" fn mlx4_ib_free_sqp_ctx(sqp_ctx: *mut mlx4_ib_demux_pv_ctx) {
    static void mlx4_ib_free_sqp_ctx(struct mlx4_ib_demux_pv_ctx *sqp_ctx)
    {
    if (sqp_ctx.state > DEMUX_PV_STATE_DOWN) {
    pub DEMUX_PV_STATE_DOWNING: sqp_ctx->state =,
    if (sqp_ctx.has_smi) {
    pub NULL: sqp_ctx->qp[0].qp =,
    pub 0): mlx4_ib_free_pv_qp_bufs(sqp_ctx, IB_QPT_SMI,,
    }
    pub NULL: sqp_ctx->qp[1].qp =,
    pub 0): mlx4_ib_free_pv_qp_bufs(sqp_ctx, IB_QPT_GSI,,
    pub NULL: sqp_ctx->pd =,
    pub NULL: sqp_ctx->cq =,
    pub DEMUX_PV_STATE_DOWN: sqp_ctx->state =,
    }
    }
#[no_mangle]
unsafe extern "C" fn mlx4_ib_free_demux_ctx(ctx: *mut mlx4_ib_demux_ctx) {
    static void mlx4_ib_free_demux_ctx(struct mlx4_ib_demux_ctx *ctx)
    {
    pub i: c_int,
    if (ctx) {
    pub to_mdev(ctx->ib_dev): *mut *mut mlx4_ib_dev dev =,
    pub 1): mlx4_ib_mcg_port_cleanup(ctx,,
    pub {: for (i = 0; i < dev->dev->caps.sqp_demux; i++),
    if (!ctx.tun[i])
    if (ctx.tun[i].state > DEMUX_PV_STATE_DOWN)
    pub DEMUX_PV_STATE_DOWNING: ctx->tun[i]->state =,
    }
    pub {: for (i = 0; i < dev->dev->caps.sqp_demux; i++),
    pub 0): destroy_pv_resources(dev, i, ctx->port, ctx->tun[i],,
    pub ctx->port): free_pv_object(dev, i,,
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn mlx4_ib_master_tunnels(dev: *mut mlx4_ib_dev, do_init: c_int) {
    static void mlx4_ib_master_tunnels(struct mlx4_ib_dev *dev, int do_init)
    {
    pub i: c_int,
    if (!mlx4_is_master(dev.dev))
// initialize or tear down tunnel QPs for the master
    pub i++): for (i = 0; i < dev->dev->caps.num_ports;,
    pub do_init): mlx4_ib_tunnels_update(dev, mlx4_master_func_num(dev->dev), i + 1,,
    }
#[no_mangle]
pub unsafe extern "C" fn mlx4_ib_init_sriov(dev: *mut mlx4_ib_dev) -> c_int {
    int mlx4_ib_init_sriov(struct mlx4_ib_dev *dev)
    {
    pub 0: int i =,
    pub err: c_int,
    if (!mlx4_is_mfunc(dev.dev))
    pub 0: return,
    pub 0: dev->sriov.is_going_down =,
    pub enabled\n"): mlx4_ib_warn(&dev->ib_dev, "multi-function,
    if (mlx4_is_slave(dev.dev)) {
    pub mode\n"): mlx4_ib_warn(&dev->ib_dev, "operating in qp1 tunnel,
    pub 0: return,
    }
    pub {: for (i = 0; i < dev->dev->caps.sqp_demux; i++),
    if (i == mlx4_master_func_num(dev.dev))
    pub dev->ib_dev.node_guid): mlx4_put_slave_node_guid(dev->dev, i,,
    else
    pub mlx4_ib_gen_node_guid()): mlx4_put_slave_node_guid(dev->dev, i,,
    }
    pub mlx4_ib_init_alias_guid_service(dev): err =,
    if (err) {
    pub process.\n"): mlx4_ib_warn(&dev->ib_dev, "Failed init alias guid,
    pub paravirt_err: goto,
    }
    pub mlx4_ib_device_register_sysfs(dev): err =,
    if (err) {
    pub sysfs\n"): mlx4_ib_warn(&dev->ib_dev, "Failed to register,
    pub sysfs_err: goto,
    }
    mlx4_ib_warn(&dev.ib_dev, "initializing demux service for %d qp1 clients\n",
    pub {: for (i = 0; i < dev->num_ports; i++),
    pub gid: union ib_gid,
    pub 1): err = __mlx4_ib_query_gid(&dev->ib_dev, i + 1, 0, &gid,,
    if (err)
    pub demux_err: goto,
    pub gid.global.interface_id: dev->sriov.demux[i].guid_cache[0] =,
    atomic64_set(&dev.sriov.demux[i].subnet_prefix,
    err = alloc_pv_object(dev, mlx4_master_func_num(dev.dev), i + 1,
    if (err)
    pub demux_err: goto,
    pub 1): err = mlx4_ib_alloc_demux_ctx(dev, &dev->sriov.demux[i], i +,
    if (err)
    pub free_pv: goto,
    }
    pub 1): mlx4_ib_master_tunnels(dev,,
    pub 0: return,
    free_pv:
    pub 1): free_pv_object(dev, mlx4_master_func_num(dev->dev), i +,
    demux_err:
    while (--i >= 0) {
    pub 1): free_pv_object(dev, mlx4_master_func_num(dev->dev), i +,
    }
    sysfs_err:
    paravirt_err:
    pub -1): mlx4_ib_cm_paravirt_clean(dev,,
    pub err: return,
    }
#[no_mangle]
pub unsafe extern "C" fn mlx4_ib_close_sriov(dev: *mut mlx4_ib_dev) {
    void mlx4_ib_close_sriov(struct mlx4_ib_dev *dev)
    {
    pub i: c_int,
    pub flags: c_ulong,
    if (!mlx4_is_mfunc(dev.dev))
    pub flags): spin_lock_irqsave(&dev->sriov.going_down_lock,,
    pub 1: dev->sriov.is_going_down =,
    pub flags): spin_unlock_irqrestore(&dev->sriov.going_down_lock,,
    if (mlx4_is_master(dev.dev)) {
    pub {: for (i = 0; i < dev->num_ports; i++),
    pub NULL: dev->sriov.sqps[i] =,
    }
    pub -1): mlx4_ib_cm_paravirt_clean(dev,,
    }
    }
