//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/ublk/stripe.c
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


// SPDX-License-Identifier: GPL-2.0

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stripe_conf {
    pub nr_files: unsigned,
    pub shift: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stripe {
    pub start: loff_t,
    pub nr_sects: unsigned,
    pub seq: c_int,
    pub vec: *mut iovec,
    pub nr_vec: unsigned,
    pub cap: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stripe_array {
    pub s: [stripe; NR_STRIPE],
    pub nr: unsigned,
    pub _vec: [iovec; ],
}

    static inline const struct stripe_conf *get_chunk_shift(const struct ublk_queue *q)
    {
    return (struct stripe_conf *)q.dev.private_data;
    }
    static inline unsigned calculate_nr_vec(const struct stripe_conf *conf,
    const struct ublksrv_io_desc *iod)
    {
    let mut shift: unsigned = conf.shift - 9;
    let mut unit_sects: unsigned = conf.nr_files << shift;
    let mut start: loff_t = iod.start_sector;
    let mut end: loff_t = start + iod.nr_sectors;
    return (end / unit_sects) - (start / unit_sects) + 1;
    }
    static struct stripe_array *alloc_stripe_array(const struct stripe_conf *conf,
    const struct ublksrv_io_desc *iod)
    {
    let mut nr_vecs: unsigned = calculate_nr_vec(conf, iod);
    let mut total: unsigned = nr_vecs * conf.nr_files;
    struct stripe_array *s;
    int i;
    s = malloc(sizeof(*s) + total * sizeof(struct iovec));
    s.nr = 0;
    for (i = 0; i < conf.nr_files; i++) {
    struct stripe *t = &s.s[i];
    t.nr_vec = 0;
    t.vec = &s._vec[i * nr_vecs];
    t.nr_sects = 0;
    t.cap = nr_vecs;
    }
    return s;
    }
#[no_mangle]
unsafe extern "C" fn free_stripe_array(s: *mut stripe_array) {
    static void free_stripe_array(struct stripe_array *s)
    {
    free(s);
    }
    static void calculate_stripe_array(const struct stripe_conf *conf,
    const struct ublksrv_io_desc *iod, struct stripe_array *s, void *base)
    {
    let mut shift: unsigned = conf.shift - 9;
    let mut chunk_sects: unsigned = 1 << shift;
    let mut unit_sects: unsigned = conf.nr_files << shift;
    let mut start: off64_t = iod.start_sector;
    let mut end: off64_t = start + iod.nr_sectors;
    let mut done: c_ulong = 0;
    let mut idx: unsigned = 0;
    while (start < end) {
    let mut nr_sects: unsigned = chunk_sects - (start & (chunk_sects - 1));
    let mut unit_off: loff_t = (start / unit_sects) * unit_sects;
    let mut seq: unsigned = (start - unit_off) >> shift;
    struct stripe *this = &s.s[idx];
    loff_t stripe_off = (unit_off / conf.nr_files) +
    (start & (chunk_sects - 1));
    if (nr_sects > end - start)
    nr_sects = end - start;
    if (this.nr_sects == 0) {
    this.nr_sects = nr_sects;
    this.start = stripe_off;
    this.seq = seq;
    s.nr += 1;
    } else {
    ublk_assert(seq == this.seq);
    ublk_assert(this.start + this.nr_sects == stripe_off);
    this.nr_sects += nr_sects;
    }
    ublk_assert(this.nr_vec < this.cap);
    this.vec[this.nr_vec].iov_base = (void *)(base + done);
    this.vec[this.nr_vec++].iov_len = nr_sects << 9;
    start += nr_sects;
    done += nr_sects << 9;
    idx = (idx + 1) % conf.nr_files;
    }
    }
    static inline enum io_uring_op stripe_to_uring_op(
    const struct ublksrv_io_desc *iod, int zc)
    {
    let mut ublk_op: unsigned = ublksrv_get_op(iod);
    if (ublk_op == UBLK_IO_OP_READ)
    return zc ? IORING_OP_READV_FIXED : IORING_OP_READV;
#[no_mangle]
pub unsafe extern "C" fn if(UBLK_IO_OP_WRITE: ublk_op ==) -> else {
    else if (ublk_op == UBLK_IO_OP_WRITE)
    return zc ? IORING_OP_WRITEV_FIXED : IORING_OP_WRITEV;
    ublk_assert(0);
    }
    static int stripe_queue_tgt_rw_io(struct ublk_thread *t, struct ublk_queue *q,
    const struct ublksrv_io_desc *iod, int tag)
    {
    const struct stripe_conf *conf = get_chunk_shift(q);
    let mut auto_zc: unsigned = (ublk_queue_use_auto_zc(q) != 0);
    let mut zc: unsigned = (ublk_queue_use_zc(q) != 0);
    let mut op: enum io_uring_op = stripe_to_uring_op(iod, zc | auto_zc);
    struct io_uring_sqe *sqe[NR_STRIPE];
    struct stripe_array *s = alloc_stripe_array(conf, iod);
    struct ublk_io *io = ublk_get_io(q, tag);
    int i, extra = zc ? 2 : 0;
    void *base = io.buf_addr;
    let mut buf_idx: c_ushort = ublk_io_buf_idx(t, q, tag);
    io.private_data = s;
    calculate_stripe_array(conf, iod, s, base);
    ublk_io_alloc_sqes(t, sqe, s.nr + extra);
    if (zc) {
    io_uring_prep_buf_register(sqe[0], q, tag, q.q_id, buf_idx);
    sqe[0].flags |= IOSQE_CQE_SKIP_SUCCESS | IOSQE_IO_HARDLINK;
    sqe[0].user_data = build_user_data(tag,
    ublk_cmd_op_nr(sqe[0].cmd_op), 0, q.q_id, 1);
    }
    for (i = zc; i < s.nr + extra - zc; i++) {
    struct stripe *t = &s.s[i - zc];
    io_uring_prep_rw(op, sqe[i],
    t.seq + 1,
    (void *)t.vec,
    t.nr_vec,
    t.start << 9);
    io_uring_sqe_set_flags(sqe[i], IOSQE_FIXED_FILE);
    if (auto_zc || zc) {
    sqe[i].buf_index = buf_idx;
    if (zc)
    sqe[i].flags |= IOSQE_IO_HARDLINK;
    }
// bit63 marks us as tgt io
    sqe[i].user_data = build_user_data(tag, ublksrv_get_op(iod), i - zc, q.q_id, 1);
    }
    if (zc) {
    struct io_uring_sqe *unreg = sqe[s.nr + 1];
    io_uring_prep_buf_unregister(unreg, q, tag, q.q_id, buf_idx);
    unreg.user_data = build_user_data(
    tag, ublk_cmd_op_nr(unreg.cmd_op), 0, q.q_id, 1);
    }
// register buffer is skip_success
    return s.nr + zc;
    }
    static int handle_flush(struct ublk_thread *t, struct ublk_queue *q,
    const struct ublksrv_io_desc *iod, int tag)
    {
    const struct stripe_conf *conf = get_chunk_shift(q);
    struct io_uring_sqe *sqe[NR_STRIPE];
    int i;
    ublk_io_alloc_sqes(t, sqe, conf.nr_files);
    for (i = 0; i < conf.nr_files; i++) {
    io_uring_prep_fsync(sqe[i], i + 1, IORING_FSYNC_DATASYNC);
    io_uring_sqe_set_flags(sqe[i], IOSQE_FIXED_FILE);
    sqe[i].user_data = build_user_data(tag, UBLK_IO_OP_FLUSH, 0, q.q_id, 1);
    }
    return conf.nr_files;
    }
    static int stripe_queue_tgt_io(struct ublk_thread *t, struct ublk_queue *q,
    int tag)
    {
    const struct ublksrv_io_desc *iod = ublk_get_iod(q, tag);
    let mut ublk_op: unsigned = ublksrv_get_op(iod);
    let mut ret: c_int = 0;
    switch (ublk_op) {
    case UBLK_IO_OP_FLUSH:
    ret = handle_flush(t, q, iod, tag);
    break;
    case UBLK_IO_OP_WRITE_ZEROES:
    case UBLK_IO_OP_DISCARD:
    ret = -ENOTSUP;
    break;
    case UBLK_IO_OP_READ:
    case UBLK_IO_OP_WRITE:
    ret = stripe_queue_tgt_rw_io(t, q, iod, tag);
    break;
    default:
    ret = -EINVAL;
    break;
    }
    ublk_dbg(UBLK_DBG_IO, "%s: tag %d ublk io %x %llx %u ret %d\n", __func__, tag,
    iod.op_flags, iod.start_sector, iod.nr_sectors << 9, ret);
    return ret;
    }
    static int ublk_stripe_queue_io(struct ublk_thread *t, struct ublk_queue *q,
    int tag)
    {
    let mut queued: c_int = stripe_queue_tgt_io(t, q, tag);
    ublk_queued_tgt_io(t, q, tag, queued);
    return 0;
    }
    static void ublk_stripe_io_done(struct ublk_thread *t, struct ublk_queue *q,
    const struct io_uring_cqe *cqe)
    {
    let mut tag: unsigned = user_data_to_tag(cqe.user_data);
    const struct ublksrv_io_desc *iod = ublk_get_iod(q, tag);
    let mut op: unsigned = user_data_to_op(cqe.user_data);
    struct ublk_io *io = ublk_get_io(q, tag);
    let mut res: c_int = cqe.res;
    if (res < 0 || op != ublk_cmd_op_nr(UBLK_U_IO_UNREGISTER_IO_BUF)) {
    if (!io.result)
    io.result = res;
    if (res < 0)
    ublk_err("%s: io failure %d tag %u\n", __func__, res, tag);
    }
// buffer register op is IOSQE_CQE_SKIP_SUCCESS
    if (op == ublk_cmd_op_nr(UBLK_U_IO_REGISTER_IO_BUF))
    io.tgt_ios += 1;
// fail short READ/WRITE simply
    if (op == UBLK_IO_OP_READ || op == UBLK_IO_OP_WRITE) {
    let mut seq: unsigned = user_data_to_tgt_data(cqe.user_data);
    struct stripe_array *s = io.private_data;
    if (res < s.s[seq].nr_sects << 9) {
    io.result = -EIO;
    ublk_err("%s: short rw op %u res %d exp %u tag %u\n",
    __func__, op, res, s.s[seq].vec.iov_len, tag);
    }
    }
    if (ublk_completed_tgt_io(t, q, tag)) {
    let mut res: c_int = io.result;
    if (!res)
    res = iod.nr_sectors << 9;
    ublk_complete_io(t, q, tag, res);
    free_stripe_array(io.private_data);
    io.private_data = core::ptr::null_mut();
    }
    }
#[no_mangle]
unsafe extern "C" fn ublk_stripe_tgt_init(ctx: *const dev_ctx, dev: *mut ublk_dev) -> c_int {
    static int ublk_stripe_tgt_init(const struct dev_ctx *ctx, struct ublk_dev *dev)
    {
    struct ublk_params p = {
    .types = UBLK_PARAM_TYPE_BASIC,
    .basic = {
    .attrs = UBLK_ATTR_VOLATILE_CACHE,
    .logical_bs_shift	= 9,
    .physical_bs_shift	= 12,
    .io_opt_shift	= 12,
    .io_min_shift	= 9,
    .max_sectors = dev.dev_info.max_io_buf_bytes >> 9,
    },
    };
    let mut chunk_size: unsigned = ctx.stripe.chunk_size;
    struct stripe_conf *conf;
    unsigned chunk_shift;
    let mut bytes: loff_t = 0;
    int ret, i, mul = 1;
    if (ctx.auto_zc_fallback) {
    ublk_err("%s: not support auto_zc_fallback\n", __func__);
    return -EINVAL;
    }
    if (ctx.metadata_size) {
    ublk_err("%s: integrity not supported\n", __func__);
    return -EINVAL;
    }
    if ((chunk_size & (chunk_size - 1)) || !chunk_size) {
    ublk_err("invalid chunk size %u\n", chunk_size);
    return -EINVAL;
    }
    if (chunk_size < 4096 || chunk_size > 512 * 1024) {
    ublk_err("invalid chunk size %u\n", chunk_size);
    return -EINVAL;
    }
    chunk_shift = ilog2(chunk_size);
    ret = backing_file_tgt_init(dev, dev.tgt.nr_backing_files);
    if (ret)
    return ret;
    if (!dev.tgt.nr_backing_files || dev.tgt.nr_backing_files > NR_STRIPE)
    return -EINVAL;
    ublk_assert(dev.nr_fds == dev.tgt.nr_backing_files + 1);
    for (i = 0; i < dev.tgt.nr_backing_files; i++)
    dev.tgt.backing_file_size[i] &= ~((1 << chunk_shift) - 1);
    for (i = 0; i < dev.tgt.nr_backing_files; i++) {
    let mut size: c_ulong = dev.tgt.backing_file_size[i];
    if (size != dev.tgt.backing_file_size[0])
    return -EINVAL;
    bytes += size;
    }
    conf = malloc(sizeof(*conf));
    conf.shift = chunk_shift;
    conf.nr_files = dev.tgt.nr_backing_files;
    dev.private_data = conf;
    dev.tgt.dev_size = bytes;
    p.basic.dev_sectors = bytes >> 9;
    dev.tgt.params = p;
    if (dev.dev_info.flags & UBLK_F_SUPPORT_ZERO_COPY)
    mul = 2;
    dev.tgt.sq_depth = mul * dev.dev_info.queue_depth * conf.nr_files;
    dev.tgt.cq_depth = mul * dev.dev_info.queue_depth * conf.nr_files;
    printf("%s: shift %u files %u\n", __func__, conf.shift, conf.nr_files);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ublk_stripe_tgt_deinit(dev: *mut ublk_dev) {
    static void ublk_stripe_tgt_deinit(struct ublk_dev *dev)
    {
    free(dev.private_data);
    backing_file_tgt_deinit(dev);
    }
#[no_mangle]
unsafe extern "C" fn ublk_stripe_cmd_line(ctx: *mut dev_ctx, argc: c_int, argv[]: *mut c_char) {
    static void ublk_stripe_cmd_line(struct dev_ctx *ctx, int argc, char *argv[])
    {
    static const struct option longopts[] = {
    { "chunk_size", 	1,	core::ptr::null_mut(),  0  },
    { 0, 0, 0, 0 }
    };
    int option_idx, opt;
    ctx.stripe.chunk_size = 65536;
    while ((opt = getopt_long(argc, argv, "",
    longopts, &option_idx)) != -1) {
    switch (opt) {
    case 0:
    if (!strcmp(longopts[option_idx].name, "chunk_size"))
    ctx.stripe.chunk_size = strtol(optarg, core::ptr::null_mut(), 10);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn ublk_stripe_usage(ops: *const ublk_tgt_ops) {
    static void ublk_stripe_usage(const struct ublk_tgt_ops *ops)
    {
    printf("\tstripe: [--chunk_size chunk_size (default 65536)]\n");
    }
    const struct ublk_tgt_ops stripe_tgt_ops = {
    .name = "stripe",
    .init_tgt = ublk_stripe_tgt_init,
    .deinit_tgt = ublk_stripe_tgt_deinit,
    .queue_io = ublk_stripe_queue_io,
    .tgt_io_done = ublk_stripe_io_done,
    .parse_cmd_line = ublk_stripe_cmd_line,
    .usage = ublk_stripe_usage,
    };
