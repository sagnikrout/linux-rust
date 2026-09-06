//! Automatically rewritten from C to Rust
//! Source: drivers/infiniband/hw/mlx5/qpc.c
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
//
// Copyright (c) 2013-2020, Mellanox Technologies inc. All rights reserved.
//

    static int mlx5_core_drain_dct(struct mlx5_ib_dev *dev,
    struct mlx5_core_dct *dct);
    static struct mlx5_core_rsc_common *
    mlx5_get_rsc(struct mlx5_qp_table *table, u32 rsn)
    {
    struct mlx5_core_rsc_common *common;
    unsigned long flags;
    spin_lock_irqsave(&table.lock, flags);
    common = radix_tree_lookup(&table.tree, rsn);
    if (common && !common.invalid)
    refcount_inc(&common.refcount);
    else
    common = core::ptr::null_mut();
    spin_unlock_irqrestore(&table.lock, flags);
    return common;
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5_core_put_rsc(common: *mut mlx5_core_rsc_common) {
    void mlx5_core_put_rsc(struct mlx5_core_rsc_common *common)
    {
    if (refcount_dec_and_test(&common.refcount))
    complete(&common.free);
    }
#[no_mangle]
unsafe extern "C" fn qp_allowed_event_types() -> u64 {
    static u64 qp_allowed_event_types(void)
    {
    u64 mask;
    mask = BIT(MLX5_EVENT_TYPE_PATH_MIG) |
    BIT(MLX5_EVENT_TYPE_COMM_EST) |
    BIT(MLX5_EVENT_TYPE_SQ_DRAINED) |
    BIT(MLX5_EVENT_TYPE_SRQ_LAST_WQE) |
    BIT(MLX5_EVENT_TYPE_WQ_CATAS_ERROR) |
    BIT(MLX5_EVENT_TYPE_PATH_MIG_FAILED) |
    BIT(MLX5_EVENT_TYPE_WQ_INVAL_REQ_ERROR) |
    BIT(MLX5_EVENT_TYPE_WQ_ACCESS_ERROR);
    return mask;
    }
#[no_mangle]
unsafe extern "C" fn rq_allowed_event_types() -> u64 {
    static u64 rq_allowed_event_types(void)
    {
    u64 mask;
    mask = BIT(MLX5_EVENT_TYPE_SRQ_LAST_WQE) |
    BIT(MLX5_EVENT_TYPE_WQ_CATAS_ERROR);
    return mask;
    }
#[no_mangle]
unsafe extern "C" fn sq_allowed_event_types() -> u64 {
    static u64 sq_allowed_event_types(void)
    {
    return BIT(MLX5_EVENT_TYPE_WQ_CATAS_ERROR);
    }
#[no_mangle]
unsafe extern "C" fn dct_allowed_event_types() -> u64 {
    static u64 dct_allowed_event_types(void)
    {
    return BIT(MLX5_EVENT_TYPE_DCT_DRAINED);
    }
#[no_mangle]
unsafe extern "C" fn is_event_type_allowed(rsc_type: c_int, event_type: c_int) -> bool {
    static bool is_event_type_allowed(int rsc_type, int event_type)
    {
    switch (rsc_type) {
    case MLX5_EVENT_QUEUE_TYPE_QP:
    return BIT(event_type) & qp_allowed_event_types();
    case MLX5_EVENT_QUEUE_TYPE_RQ:
    return BIT(event_type) & rq_allowed_event_types();
    case MLX5_EVENT_QUEUE_TYPE_SQ:
    return BIT(event_type) & sq_allowed_event_types();
    case MLX5_EVENT_QUEUE_TYPE_DCT:
    return BIT(event_type) & dct_allowed_event_types();
    default:
    WARN(1, "Event arrived for unknown resource type");
    return false;
    }
    }
#[no_mangle]
unsafe extern "C" fn dct_event_notifier(dev: *mut mlx5_ib_dev, eqe: *mut mlx5_eqe) -> c_int {
    static int dct_event_notifier(struct mlx5_ib_dev *dev, struct mlx5_eqe *eqe)
    {
    struct mlx5_core_dct *dct;
    unsigned long flags;
    u32 qpn;
    qpn = be32_to_cpu(eqe.data.dct.dctn) & 0xFFFFFF;
    xa_lock_irqsave(&dev.qp_table.dct_xa, flags);
    dct = xa_load(&dev.qp_table.dct_xa, qpn);
    if (dct)
    complete(&dct.drained);
    xa_unlock_irqrestore(&dev.qp_table.dct_xa, flags);
    return NOTIFY_OK;
    }
    static int rsc_event_notifier(struct notifier_block *nb,
    unsigned long type, void *data)
    {
    struct mlx5_ib_dev *dev =
    container_of(nb, struct mlx5_ib_dev, qp_table.nb);
    struct mlx5_core_rsc_common *common;
    struct mlx5_eqe *eqe = data;
    let mut event_type: u8 = (u8)type;
    struct mlx5_core_qp *qp;
    u32 rsn;
    switch (event_type) {
    case MLX5_EVENT_TYPE_DCT_DRAINED:
    return dct_event_notifier(dev, eqe);
    case MLX5_EVENT_TYPE_PATH_MIG:
    case MLX5_EVENT_TYPE_COMM_EST:
    case MLX5_EVENT_TYPE_SQ_DRAINED:
    case MLX5_EVENT_TYPE_SRQ_LAST_WQE:
    case MLX5_EVENT_TYPE_WQ_CATAS_ERROR:
    case MLX5_EVENT_TYPE_PATH_MIG_FAILED:
    case MLX5_EVENT_TYPE_WQ_INVAL_REQ_ERROR:
    case MLX5_EVENT_TYPE_WQ_ACCESS_ERROR:
    rsn = be32_to_cpu(eqe.data.qp_srq.qp_srq_n) & 0xffffff;
    rsn |= (eqe.data.qp_srq.type << MLX5_USER_INDEX_LEN);
    break;
    default:
    return NOTIFY_DONE;
    }
    common = mlx5_get_rsc(&dev.qp_table, rsn);
    if (!common)
    return NOTIFY_OK;
    if (!is_event_type_allowed((rsn >> MLX5_USER_INDEX_LEN), event_type))
    goto out;
    switch (common.res) {
    case MLX5_RES_QP:
    case MLX5_RES_RQ:
    case MLX5_RES_SQ:
    qp = (struct mlx5_core_qp *)common;
    qp.event(qp, event_type);
// Need to put resource in event handler
    return NOTIFY_OK;
    default:
    break;
    }
    out:
    mlx5_core_put_rsc(common);
    return NOTIFY_OK;
    }
    static int create_resource_common(struct mlx5_ib_dev *dev,
    struct mlx5_core_qp *qp, int rsc_type)
    {
    struct mlx5_qp_table *table = &dev.qp_table;
    int err;
    qp.common.res = rsc_type;
    spin_lock_irq(&table.lock);
    err = radix_tree_insert(&table.tree,
    qp.qpn | (rsc_type << MLX5_USER_INDEX_LEN),
    qp);
    spin_unlock_irq(&table.lock);
    if (err)
    return err;
    refcount_set(&qp.common.refcount, 1);
    init_completion(&qp.common.free);
    qp.pid = current.pid;
    return 0;
    }
    static void modify_resource_common_state(struct mlx5_ib_dev *dev,
    struct mlx5_core_qp *qp,
    bool invalid)
    {
    struct mlx5_qp_table *table = &dev.qp_table;
    unsigned long flags;
    spin_lock_irqsave(&table.lock, flags);
    qp.common.invalid = invalid;
    spin_unlock_irqrestore(&table.lock, flags);
    }
    static void destroy_resource_common(struct mlx5_ib_dev *dev,
    struct mlx5_core_qp *qp)
    {
    struct mlx5_qp_table *table = &dev.qp_table;
    unsigned long flags;
    spin_lock_irqsave(&table.lock, flags);
    radix_tree_delete(&table.tree,
    qp.qpn | (qp.common.res << MLX5_USER_INDEX_LEN));
    spin_unlock_irqrestore(&table.lock, flags);
    mlx5_core_put_rsc((struct mlx5_core_rsc_common *)qp);
    wait_for_completion(&qp.common.free);
    }
    static int _mlx5_core_destroy_dct(struct mlx5_ib_dev *dev,
    struct mlx5_core_dct *dct)
    {
    u32 in[MLX5_ST_SZ_DW(destroy_dct_in)] = {};
    struct mlx5_core_qp *qp = &dct.mqp;
    MLX5_SET(destroy_dct_in, in, opcode, MLX5_CMD_OP_DESTROY_DCT);
    MLX5_SET(destroy_dct_in, in, dctn, qp.qpn);
    MLX5_SET(destroy_dct_in, in, uid, qp.uid);
    return mlx5_cmd_exec_in(dev.mdev, destroy_dct, in);
    }
    int mlx5_core_create_dct(struct mlx5_ib_dev *dev, struct mlx5_core_dct *dct,
    u32 *in, int inlen, u32 *out, int outlen)
    {
    struct mlx5_core_qp *qp = &dct.mqp;
    int err;
    init_completion(&dct.drained);
    MLX5_SET(create_dct_in, in, opcode, MLX5_CMD_OP_CREATE_DCT);
    err = mlx5_cmd_do(dev.mdev, in, inlen, out, outlen);
    if (err)
    return err;
    qp.qpn = MLX5_GET(create_dct_out, out, dctn);
    qp.uid = MLX5_GET(create_dct_in, in, uid);
    err = xa_err(xa_store_irq(&dev.qp_table.dct_xa, qp.qpn, dct, GFP_KERNEL));
    if (err)
    goto err_cmd;
    return 0;
    err_cmd:
    _mlx5_core_destroy_dct(dev, dct);
    return err;
    }
    int mlx5_qpc_create_qp(struct mlx5_ib_dev *dev, struct mlx5_core_qp *qp,
    u32 *in, int inlen, u32 *out)
    {
    u32 din[MLX5_ST_SZ_DW(destroy_qp_in)] = {};
    int err;
    MLX5_SET(create_qp_in, in, opcode, MLX5_CMD_OP_CREATE_QP);
    err = mlx5_cmd_exec(dev.mdev, in, inlen, out,
    MLX5_ST_SZ_BYTES(create_qp_out));
    if (err)
    return err;
    qp.uid = MLX5_GET(create_qp_in, in, uid);
    qp.qpn = MLX5_GET(create_qp_out, out, qpn);
    err = create_resource_common(dev, qp, MLX5_RES_QP);
    if (err)
    goto err_cmd;
    if (dev.ib_dev.type != RDMA_DEVICE_TYPE_SMI)
    mlx5_debug_qp_add(dev.mdev, qp);
    return 0;
    err_cmd:
    MLX5_SET(destroy_qp_in, din, opcode, MLX5_CMD_OP_DESTROY_QP);
    MLX5_SET(destroy_qp_in, din, qpn, qp.qpn);
    MLX5_SET(destroy_qp_in, din, uid, qp.uid);
    mlx5_cmd_exec_in(dev.mdev, destroy_qp, din);
    return err;
    }
    static int mlx5_core_drain_dct(struct mlx5_ib_dev *dev,
    struct mlx5_core_dct *dct)
    {
    u32 in[MLX5_ST_SZ_DW(drain_dct_in)] = {};
    struct mlx5_core_qp *qp = &dct.mqp;
    MLX5_SET(drain_dct_in, in, opcode, MLX5_CMD_OP_DRAIN_DCT);
    MLX5_SET(drain_dct_in, in, dctn, qp.qpn);
    MLX5_SET(drain_dct_in, in, uid, qp.uid);
    return mlx5_cmd_exec_in(dev.mdev, drain_dct, in);
    }
    int mlx5_core_destroy_dct(struct mlx5_ib_dev *dev,
    struct mlx5_core_dct *dct)
    {
    struct mlx5_qp_table *table = &dev.qp_table;
    struct mlx5_core_dct *tmp;
    int err;
    err = mlx5_core_drain_dct(dev, dct);
    if (err) {
    if (dev.mdev.state == MLX5_DEVICE_STATE_INTERNAL_ERROR)
    goto destroy;
    return err;
    }
    wait_for_completion(&dct.drained);
    destroy:
    tmp = xa_cmpxchg_irq(&table.dct_xa, dct.mqp.qpn, dct, XA_ZERO_ENTRY, GFP_KERNEL);
    if (WARN_ON(tmp != dct))
    return xa_err(tmp) ?: -EINVAL;
    err = _mlx5_core_destroy_dct(dev, dct);
    if (err) {
    xa_cmpxchg_irq(&table.dct_xa, dct.mqp.qpn, XA_ZERO_ENTRY, dct, 0);
    return err;
    }
//
// A race can occur where a concurrent create gets the same dctn
// (after hardware released it) and overwrites XA_ZERO_ENTRY with
// its new DCT before we reach here. In that case, we must not erase
// the entry as it now belongs to the new DCT.
//
    xa_cmpxchg_irq(&table.dct_xa, dct.mqp.qpn, XA_ZERO_ENTRY, core::ptr::null_mut(), 0);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5_core_destroy_qp(dev: *mut mlx5_ib_dev, qp: *mut mlx5_core_qp) -> c_int {
    int mlx5_core_destroy_qp(struct mlx5_ib_dev *dev, struct mlx5_core_qp *qp)
    {
    u32 in[MLX5_ST_SZ_DW(destroy_qp_in)] = {};
    if (dev.ib_dev.type != RDMA_DEVICE_TYPE_SMI)
    mlx5_debug_qp_remove(dev.mdev, qp);
    destroy_resource_common(dev, qp);
    MLX5_SET(destroy_qp_in, in, opcode, MLX5_CMD_OP_DESTROY_QP);
    MLX5_SET(destroy_qp_in, in, qpn, qp.qpn);
    MLX5_SET(destroy_qp_in, in, uid, qp.uid);
    return mlx5_cmd_exec_in(dev.mdev, destroy_qp, in);
    }
    int mlx5_core_set_delay_drop(struct mlx5_ib_dev *dev,
    u32 timeout_usec)
    {
    u32 in[MLX5_ST_SZ_DW(set_delay_drop_params_in)] = {};
    MLX5_SET(set_delay_drop_params_in, in, opcode,
    MLX5_CMD_OP_SET_DELAY_DROP_PARAMS);
    MLX5_SET(set_delay_drop_params_in, in, delay_drop_timeout,
    timeout_usec / 100);
    return mlx5_cmd_exec_in(dev.mdev, set_delay_drop_params, in);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mbox_info {
    pub in: *mut u32,
    pub out: *mut u32,
    pub inlen: c_int,
    pub outlen: c_int,
}

#[no_mangle]
unsafe extern "C" fn mbox_alloc(mbox: *mut mbox_info, inlen: c_int, outlen: c_int) -> c_int {
    static int mbox_alloc(struct mbox_info *mbox, int inlen, int outlen)
    {
    mbox.inlen  = inlen;
    mbox.outlen = outlen;
    mbox.in = kzalloc(mbox.inlen, GFP_KERNEL);
    mbox.out = kzalloc(mbox.outlen, GFP_KERNEL);
    if (!mbox.in || !mbox.out) {
    kfree(mbox.in);
    kfree(mbox.out);
    return -ENOMEM;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mbox_free(mbox: *mut mbox_info) {
    static void mbox_free(struct mbox_info *mbox)
    {
    kfree(mbox.in);
    kfree(mbox.out);
    }
#[no_mangle]
unsafe extern "C" fn get_ece_from_mbox(out: *mut c_void, opcode: u16) -> c_int {
    static int get_ece_from_mbox(void *out, u16 opcode)
    {
    let mut ece: c_int = 0;
    switch (opcode) {
    case MLX5_CMD_OP_INIT2INIT_QP:
    ece = MLX5_GET(init2init_qp_out, out, ece);
    break;
    case MLX5_CMD_OP_INIT2RTR_QP:
    ece = MLX5_GET(init2rtr_qp_out, out, ece);
    break;
    case MLX5_CMD_OP_RTR2RTS_QP:
    ece = MLX5_GET(rtr2rts_qp_out, out, ece);
    break;
    case MLX5_CMD_OP_RTS2RTS_QP:
    ece = MLX5_GET(rts2rts_qp_out, out, ece);
    break;
    case MLX5_CMD_OP_RST2INIT_QP:
    ece = MLX5_GET(rst2init_qp_out, out, ece);
    break;
    default:
    break;
    }
    return ece;
    }
    static int modify_qp_mbox_alloc(struct mlx5_core_dev *dev, u16 opcode, int qpn,
    u32 opt_param_mask, void *qpc,
    struct mbox_info *mbox, u16 uid, u32 ece)
    {
    mbox.out = core::ptr::null_mut();
    mbox.in = core::ptr::null_mut();

    mbox_alloc(mbox, MLX5_ST_SZ_BYTES(typ##_in), MLX5_ST_SZ_BYTES(typ##_out))

    do {                                                                   \
    MLX5_SET(typ##_in, in, opcode, _opcode);                       \
    MLX5_SET(typ##_in, in, qpn, _qpn);                             \
    MLX5_SET(typ##_in, in, uid, _uid);                             \
    } while (0)

    do {                                                                   \
    MOD_QP_IN_SET(typ, in, _opcode, _qpn, _uid);                   \
    MLX5_SET(typ##_in, in, opt_param_mask, _opt_p);                \
    memcpy(MLX5_ADDR_OF(typ##_in, in, qpc), _qpc,                  \
    MLX5_ST_SZ_BYTES(qpc));                                 \
    } while (0)
    switch (opcode) {
// 2RST & 2ERR
    case MLX5_CMD_OP_2RST_QP:
    if (MBOX_ALLOC(mbox, qp_2rst))
    return -ENOMEM;
    MOD_QP_IN_SET(qp_2rst, mbox.in, opcode, qpn, uid);
    break;
    case MLX5_CMD_OP_2ERR_QP:
    if (MBOX_ALLOC(mbox, qp_2err))
    return -ENOMEM;
    MOD_QP_IN_SET(qp_2err, mbox.in, opcode, qpn, uid);
    break;
// MODIFY with QPC
    case MLX5_CMD_OP_RST2INIT_QP:
    if (MBOX_ALLOC(mbox, rst2init_qp))
    return -ENOMEM;
    MOD_QP_IN_SET_QPC(rst2init_qp, mbox.in, opcode, qpn,
    opt_param_mask, qpc, uid);
    MLX5_SET(rst2init_qp_in, mbox.in, ece, ece);
    break;
    case MLX5_CMD_OP_INIT2RTR_QP:
    if (MBOX_ALLOC(mbox, init2rtr_qp))
    return -ENOMEM;
    MOD_QP_IN_SET_QPC(init2rtr_qp, mbox.in, opcode, qpn,
    opt_param_mask, qpc, uid);
    MLX5_SET(init2rtr_qp_in, mbox.in, ece, ece);
    break;
    case MLX5_CMD_OP_RTR2RTS_QP:
    if (MBOX_ALLOC(mbox, rtr2rts_qp))
    return -ENOMEM;
    MOD_QP_IN_SET_QPC(rtr2rts_qp, mbox.in, opcode, qpn,
    opt_param_mask, qpc, uid);
    MLX5_SET(rtr2rts_qp_in, mbox.in, ece, ece);
    break;
    case MLX5_CMD_OP_RTS2RTS_QP:
    if (MBOX_ALLOC(mbox, rts2rts_qp))
    return -ENOMEM;
    MOD_QP_IN_SET_QPC(rts2rts_qp, mbox.in, opcode, qpn,
    opt_param_mask, qpc, uid);
    MLX5_SET(rts2rts_qp_in, mbox.in, ece, ece);
    break;
    case MLX5_CMD_OP_SQERR2RTS_QP:
    if (MBOX_ALLOC(mbox, sqerr2rts_qp))
    return -ENOMEM;
    MOD_QP_IN_SET_QPC(sqerr2rts_qp, mbox.in, opcode, qpn,
    opt_param_mask, qpc, uid);
    break;
    case MLX5_CMD_OP_SQD_RTS_QP:
    if (MBOX_ALLOC(mbox, sqd2rts_qp))
    return -ENOMEM;
    MOD_QP_IN_SET_QPC(sqd2rts_qp, mbox.in, opcode, qpn,
    opt_param_mask, qpc, uid);
    break;
    case MLX5_CMD_OP_INIT2INIT_QP:
    if (MBOX_ALLOC(mbox, init2init_qp))
    return -ENOMEM;
    MOD_QP_IN_SET_QPC(init2init_qp, mbox.in, opcode, qpn,
    opt_param_mask, qpc, uid);
    MLX5_SET(init2init_qp_in, mbox.in, ece, ece);
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
    int mlx5_core_qp_modify(struct mlx5_ib_dev *dev, u16 opcode, u32 opt_param_mask,
    void *qpc, struct mlx5_core_qp *qp, u32 *ece)
    {
    struct mbox_info mbox;
    int err;
    err = modify_qp_mbox_alloc(dev.mdev, opcode, qp.qpn, opt_param_mask,
    qpc, &mbox, qp.uid, (ece) ? *ece : 0);
    if (err)
    return err;
    err = mlx5_cmd_exec(dev.mdev, mbox.in, mbox.inlen, mbox.out,
    mbox.outlen);
    if (ece)
// ece = get_ece_from_mbox(mbox.out, opcode);
    mbox_free(&mbox);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5_init_qp_table(dev: *mut mlx5_ib_dev) -> c_int {
    int mlx5_init_qp_table(struct mlx5_ib_dev *dev)
    {
    struct mlx5_qp_table *table = &dev.qp_table;
    spin_lock_init(&table.lock);
    INIT_RADIX_TREE(&table.tree, GFP_ATOMIC);
    xa_init(&table.dct_xa);
    if (dev.ib_dev.type != RDMA_DEVICE_TYPE_SMI)
    mlx5_qp_debugfs_init(dev.mdev);
    table.nb.notifier_call = rsc_event_notifier;
    mlx5_notifier_register(dev.mdev, &table.nb);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5_cleanup_qp_table(dev: *mut mlx5_ib_dev) {
    void mlx5_cleanup_qp_table(struct mlx5_ib_dev *dev)
    {
    struct mlx5_qp_table *table = &dev.qp_table;
    mlx5_notifier_unregister(dev.mdev, &table.nb);
    if (dev.ib_dev.type != RDMA_DEVICE_TYPE_SMI)
    mlx5_qp_debugfs_cleanup(dev.mdev);
    }
    int mlx5_core_qp_query(struct mlx5_ib_dev *dev, struct mlx5_core_qp *qp,
    u32 *out, int outlen, bool qpc_ext)
    {
    u32 in[MLX5_ST_SZ_DW(query_qp_in)] = {};
    MLX5_SET(query_qp_in, in, opcode, MLX5_CMD_OP_QUERY_QP);
    MLX5_SET(query_qp_in, in, qpn, qp.qpn);
    MLX5_SET(query_qp_in, in, qpc_ext, qpc_ext);
    return mlx5_cmd_exec(dev.mdev, in, sizeof(in), out, outlen);
    }
    int mlx5_core_dct_query(struct mlx5_ib_dev *dev, struct mlx5_core_dct *dct,
    u32 *out, int outlen)
    {
    u32 in[MLX5_ST_SZ_DW(query_dct_in)] = {};
    struct mlx5_core_qp *qp = &dct.mqp;
    MLX5_SET(query_dct_in, in, opcode, MLX5_CMD_OP_QUERY_DCT);
    MLX5_SET(query_dct_in, in, dctn, qp.qpn);
    return mlx5_cmd_exec(dev.mdev, (void *)&in, sizeof(in), (void *)out,
    outlen);
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5_core_xrcd_alloc(dev: *mut mlx5_ib_dev, xrcdn: *mut u32) -> c_int {
    int mlx5_core_xrcd_alloc(struct mlx5_ib_dev *dev, u32 *xrcdn)
    {
    u32 out[MLX5_ST_SZ_DW(alloc_xrcd_out)] = {};
    u32 in[MLX5_ST_SZ_DW(alloc_xrcd_in)] = {};
    int err;
    MLX5_SET(alloc_xrcd_in, in, opcode, MLX5_CMD_OP_ALLOC_XRCD);
    err = mlx5_cmd_exec_inout(dev.mdev, alloc_xrcd, in, out);
    if (!err)
// xrcdn = MLX5_GET(alloc_xrcd_out, out, xrcd);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5_core_xrcd_dealloc(dev: *mut mlx5_ib_dev, xrcdn: u32) -> c_int {
    int mlx5_core_xrcd_dealloc(struct mlx5_ib_dev *dev, u32 xrcdn)
    {
    u32 in[MLX5_ST_SZ_DW(dealloc_xrcd_in)] = {};
    MLX5_SET(dealloc_xrcd_in, in, opcode, MLX5_CMD_OP_DEALLOC_XRCD);
    MLX5_SET(dealloc_xrcd_in, in, xrcd, xrcdn);
    return mlx5_cmd_exec_in(dev.mdev, dealloc_xrcd, in);
    }
#[no_mangle]
unsafe extern "C" fn destroy_rq_tracked(dev: *mut mlx5_ib_dev, rqn: u32, uid: u16) -> c_int {
    static int destroy_rq_tracked(struct mlx5_ib_dev *dev, u32 rqn, u16 uid)
    {
    u32 in[MLX5_ST_SZ_DW(destroy_rq_in)] = {};
    MLX5_SET(destroy_rq_in, in, opcode, MLX5_CMD_OP_DESTROY_RQ);
    MLX5_SET(destroy_rq_in, in, rqn, rqn);
    MLX5_SET(destroy_rq_in, in, uid, uid);
    return mlx5_cmd_exec_in(dev.mdev, destroy_rq, in);
    }
    int mlx5_core_create_rq_tracked(struct mlx5_ib_dev *dev, u32 *in, int inlen,
    struct mlx5_core_qp *rq)
    {
    int err;
    u32 rqn;
    err = mlx5_core_create_rq(dev.mdev, in, inlen, &rqn);
    if (err)
    return err;
    rq.uid = MLX5_GET(create_rq_in, in, uid);
    rq.qpn = rqn;
    err = create_resource_common(dev, rq, MLX5_RES_RQ);
    if (err)
    goto err_destroy_rq;
    return 0;
    err_destroy_rq:
    destroy_rq_tracked(dev, rq.qpn, rq.uid);
    return err;
    }
    int mlx5_core_destroy_rq_tracked(struct mlx5_ib_dev *dev,
    struct mlx5_core_qp *rq)
    {
    int ret;
// The rq destruction can be called again in case it fails, hence we
// mark the common resource as invalid and only once FW destruction
// is completed successfully we actually destroy the resources.
//
    modify_resource_common_state(dev, rq, true);
    ret = destroy_rq_tracked(dev, rq.qpn, rq.uid);
    if (ret) {
    modify_resource_common_state(dev, rq, false);
    return ret;
    }
    destroy_resource_common(dev, rq);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn destroy_sq_tracked(dev: *mut mlx5_ib_dev, sqn: u32, uid: u16) {
    static void destroy_sq_tracked(struct mlx5_ib_dev *dev, u32 sqn, u16 uid)
    {
    u32 in[MLX5_ST_SZ_DW(destroy_sq_in)] = {};
    MLX5_SET(destroy_sq_in, in, opcode, MLX5_CMD_OP_DESTROY_SQ);
    MLX5_SET(destroy_sq_in, in, sqn, sqn);
    MLX5_SET(destroy_sq_in, in, uid, uid);
    mlx5_cmd_exec_in(dev.mdev, destroy_sq, in);
    }
    int mlx5_core_create_sq_tracked(struct mlx5_ib_dev *dev, u32 *in, int inlen,
    struct mlx5_core_qp *sq)
    {
    u32 out[MLX5_ST_SZ_DW(create_sq_out)] = {};
    int err;
    MLX5_SET(create_sq_in, in, opcode, MLX5_CMD_OP_CREATE_SQ);
    err = mlx5_cmd_exec(dev.mdev, in, inlen, out, sizeof(out));
    if (err)
    return err;
    sq.qpn = MLX5_GET(create_sq_out, out, sqn);
    sq.uid = MLX5_GET(create_sq_in, in, uid);
    err = create_resource_common(dev, sq, MLX5_RES_SQ);
    if (err)
    goto err_destroy_sq;
    return 0;
    err_destroy_sq:
    destroy_sq_tracked(dev, sq.qpn, sq.uid);
    return err;
    }
    void mlx5_core_destroy_sq_tracked(struct mlx5_ib_dev *dev,
    struct mlx5_core_qp *sq)
    {
    destroy_resource_common(dev, sq);
    destroy_sq_tracked(dev, sq.qpn, sq.uid);
    }
    struct mlx5_core_rsc_common *mlx5_core_res_hold(struct mlx5_ib_dev *dev,
    int res_num,
    enum mlx5_res_type res_type)
    {
    let mut rsn: u32 = res_num | (res_type << MLX5_USER_INDEX_LEN);
    struct mlx5_qp_table *table = &dev.qp_table;
    return mlx5_get_rsc(table, rsn);
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5_core_res_put(res: *mut mlx5_core_rsc_common) {
    void mlx5_core_res_put(struct mlx5_core_rsc_common *res)
    {
    mlx5_core_put_rsc(res);
    }
