//! Automatically rewritten from C to Rust
//! Source: drivers/vdpa/mlx5/core/mr.c
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
// Copyright (c) 2020 Mellanox Technologies Ltd.

// DIV_ROUND_UP where the divider is a power of 2 give by its log base 2 value

    ({ \
    u64 __s = _s; \
    u64 _res; \
    _res = (((_n) + (1 << (__s)) - 1) >> (__s)); \
    _res; \
    })
#[no_mangle]
unsafe extern "C" fn get_octo_len(len: u64, page_shift: c_int) -> c_int {
    static int get_octo_len(u64 len, int page_shift)
    {
    let mut page_size: u64 = 1ULL << page_shift;
    int npages;
    npages = ALIGN(len, page_size) >> page_shift;
    return (npages + 1) / 2;
    }
#[no_mangle]
unsafe extern "C" fn mlx5_set_access_mode(mkc: *mut c_void, mode: c_int) {
    static void mlx5_set_access_mode(void *mkc, int mode)
    {
    MLX5_SET(mkc, mkc, access_mode_1_0, mode & 0x3);
    MLX5_SET(mkc, mkc, access_mode_4_2, mode >> 2);
    }
#[no_mangle]
unsafe extern "C" fn populate_mtts(mr: *mut mlx5_vdpa_direct_mr, mtt: *mut __be64) {
    static void populate_mtts(struct mlx5_vdpa_direct_mr *mr, __be64 *mtt)
    {
    struct scatterlist *sg;
    let mut nsg: c_int = mr.nsg;
    u64 dma_addr;
    u64 dma_len;
    let mut j: c_int = 0;
    int i;
    for_each_sg(mr.sg_head.sgl, sg, mr.nent, i) {
    for (dma_addr = sg_dma_address(sg), dma_len = sg_dma_len(sg);
    nsg && dma_len;
    nsg--, dma_addr += BIT(mr.log_size), dma_len -= BIT(mr.log_size))
    mtt[j++] = cpu_to_be64(dma_addr);
    }
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_create_mkey_mem {
    pub out: [u8; MLX5_ST_SZ_BYTES(create_mkey_out)],
    pub in: [u8; MLX5_ST_SZ_BYTES(create_mkey_in)],
    pub mtt: [__be64; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_destroy_mkey_mem {
    pub out: [u8; MLX5_ST_SZ_BYTES(destroy_mkey_out)],
    pub in: [u8; MLX5_ST_SZ_BYTES(destroy_mkey_in)],
}

    static void fill_create_direct_mr(struct mlx5_vdpa_dev *mvdev,
    struct mlx5_vdpa_direct_mr *mr,
    struct mlx5_create_mkey_mem *mem)
    {
    void *in = &mem.in;
    void *mkc;
    MLX5_SET(create_mkey_in, in, uid, mvdev.res.uid);
    mkc = MLX5_ADDR_OF(create_mkey_in, in, memory_key_mkey_entry);
    MLX5_SET(mkc, mkc, lw, !!(mr.perm & VHOST_MAP_WO));
    MLX5_SET(mkc, mkc, lr, !!(mr.perm & VHOST_MAP_RO));
    mlx5_set_access_mode(mkc, MLX5_MKC_ACCESS_MODE_MTT);
    MLX5_SET(mkc, mkc, qpn, 0xffffff);
    MLX5_SET(mkc, mkc, pd, mvdev.res.pdn);
    MLX5_SET64(mkc, mkc, start_addr, mr.offset);
    MLX5_SET64(mkc, mkc, len, mr.end - mr.start);
    MLX5_SET(mkc, mkc, log_page_size, mr.log_size);
    MLX5_SET(mkc, mkc, translations_octword_size,
    get_octo_len(mr.end - mr.start, mr.log_size));
    MLX5_SET(create_mkey_in, in, translations_octword_actual_size,
    get_octo_len(mr.end - mr.start, mr.log_size));
    populate_mtts(mr, MLX5_ADDR_OF(create_mkey_in, in, klm_pas_mtt));
    MLX5_SET(create_mkey_in, in, opcode, MLX5_CMD_OP_CREATE_MKEY);
    MLX5_SET(create_mkey_in, in, uid, mvdev.res.uid);
    }
    static void create_direct_mr_end(struct mlx5_vdpa_dev *mvdev,
    struct mlx5_vdpa_direct_mr *mr,
    struct mlx5_create_mkey_mem *mem)
    {
    let mut mkey_index: u32 = MLX5_GET(create_mkey_out, mem.out, mkey_index);
    mr.mr = mlx5_idx_to_mkey(mkey_index);
    }
    static void fill_destroy_direct_mr(struct mlx5_vdpa_dev *mvdev,
    struct mlx5_vdpa_direct_mr *mr,
    struct mlx5_destroy_mkey_mem *mem)
    {
    void *in = &mem.in;
    MLX5_SET(destroy_mkey_in, in, uid, mvdev.res.uid);
    MLX5_SET(destroy_mkey_in, in, opcode, MLX5_CMD_OP_DESTROY_MKEY);
    MLX5_SET(destroy_mkey_in, in, mkey_index, mlx5_mkey_to_idx(mr.mr));
    }
#[no_mangle]
unsafe extern "C" fn destroy_direct_mr(mvdev: *mut mlx5_vdpa_dev, mr: *mut mlx5_vdpa_direct_mr) {
    static void destroy_direct_mr(struct mlx5_vdpa_dev *mvdev, struct mlx5_vdpa_direct_mr *mr)
    {
    if (!mr.mr)
    return;
    mlx5_vdpa_destroy_mkey(mvdev, mr.mr);
    }
#[no_mangle]
unsafe extern "C" fn map_start(map: *mut vhost_iotlb_map, mr: *mut mlx5_vdpa_direct_mr) -> u64 {
    static u64 map_start(struct vhost_iotlb_map *map, struct mlx5_vdpa_direct_mr *mr)
    {
    return max_t(u64, map.start, mr.start);
    }
#[no_mangle]
unsafe extern "C" fn map_end(map: *mut vhost_iotlb_map, mr: *mut mlx5_vdpa_direct_mr) -> u64 {
    static u64 map_end(struct vhost_iotlb_map *map, struct mlx5_vdpa_direct_mr *mr)
    {
    return min_t(u64, map.last + 1, mr.end);
    }
#[no_mangle]
unsafe extern "C" fn maplen(map: *mut vhost_iotlb_map, mr: *mut mlx5_vdpa_direct_mr) -> u64 {
    static u64 maplen(struct vhost_iotlb_map *map, struct mlx5_vdpa_direct_mr *mr)
    {
    return map_end(map, mr) - map_start(map, mr);
    }

#[no_mangle]
unsafe extern "C" fn indir_start_addr(mkey: *mut mlx5_vdpa_mr) -> u64 {
    static u64 indir_start_addr(struct mlx5_vdpa_mr *mkey)
    {
    struct mlx5_vdpa_direct_mr *s;
    s = list_first_entry_or_null(&mkey.head, struct mlx5_vdpa_direct_mr, list);
    if (!s)
    return MLX5_VDPA_INVALID_START_ADDR;
    return s.start;
    }
#[no_mangle]
unsafe extern "C" fn indir_len(mkey: *mut mlx5_vdpa_mr) -> u64 {
    static u64 indir_len(struct mlx5_vdpa_mr *mkey)
    {
    struct mlx5_vdpa_direct_mr *s;
    struct mlx5_vdpa_direct_mr *e;
    s = list_first_entry_or_null(&mkey.head, struct mlx5_vdpa_direct_mr, list);
    if (!s)
    return MLX5_VDPA_INVALID_LEN;
    e = list_last_entry(&mkey.head, struct mlx5_vdpa_direct_mr, list);
    return e.end - s.start;
    }
pub const LOG_MAX_KLM_SIZE: c_int = 30;

#[no_mangle]
unsafe extern "C" fn klm_bcount(size: u64) -> u32 {
    static u32 klm_bcount(u64 size)
    {
    return (u32)size;
    }
#[no_mangle]
unsafe extern "C" fn fill_indir(mvdev: *mut mlx5_vdpa_dev, mkey: *mut mlx5_vdpa_mr, in: *mut c_void) {
    static void fill_indir(struct mlx5_vdpa_dev *mvdev, struct mlx5_vdpa_mr *mkey, void *in)
    {
    struct mlx5_vdpa_direct_mr *dmr;
    struct mlx5_klm *klmarr;
    struct mlx5_klm *klm;
    let mut first: bool = true;
    u64 preve;
    int i;
    klmarr = MLX5_ADDR_OF(create_mkey_in, in, klm_pas_mtt);
    i = 0;
    list_for_each_entry(dmr, &mkey.head, list) {
    again:
    klm = &klmarr[i++];
    if (first) {
    preve = dmr.start;
    first = false;
    }
    if (preve == dmr.start) {
    klm.key = cpu_to_be32(dmr.mr);
    klm.bcount = cpu_to_be32(klm_bcount(dmr.end - dmr.start));
    preve = dmr.end;
    } else {
    let mut bcount: u64 = min_t(u64, dmr.start - preve, MAX_KLM_SIZE);
    klm.key = cpu_to_be32(mvdev.res.null_mkey);
    klm.bcount = cpu_to_be32(klm_bcount(bcount));
    preve += bcount;
    goto again;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn klm_byte_size(nklms: c_int) -> c_int {
    static int klm_byte_size(int nklms)
    {
    return 16 * ALIGN(nklms, 4);
    }
pub const MLX5_VDPA_MTT_ALIGN: c_int = 16;
#[no_mangle]
unsafe extern "C" fn create_direct_keys(mvdev: *mut mlx5_vdpa_dev, mr: *mut mlx5_vdpa_mr) -> c_int {
    static int create_direct_keys(struct mlx5_vdpa_dev *mvdev, struct mlx5_vdpa_mr *mr)
    {
    struct mlx5_vdpa_async_cmd *cmds;
    struct mlx5_vdpa_direct_mr *dmr;
    let mut err: c_int = 0;
    let mut i: c_int = 0;
    cmds = kvzalloc_objs(*cmds, mr.num_directs);
    if (!cmds)
    return -ENOMEM;
    list_for_each_entry(dmr, &mr.head, list) {
    struct mlx5_create_mkey_mem *cmd_mem;
    int mttcount;
    mttcount = ALIGN(dmr.nsg, MLX5_VDPA_MTT_ALIGN / sizeof(cmd_mem.mtt[0]));
    cmd_mem = kvzalloc_flex(*cmd_mem, mtt, mttcount);
    if (!cmd_mem) {
    err = -ENOMEM;
    goto done;
    }
    cmds[i].out = cmd_mem.out;
    cmds[i].outlen = sizeof(cmd_mem.out);
    cmds[i].in = cmd_mem.in;
    cmds[i].inlen = struct_size(cmd_mem, mtt, mttcount) -
    offsetof(struct mlx5_create_mkey_mem, in);
    fill_create_direct_mr(mvdev, dmr, cmd_mem);
    i++;
    }
    err = mlx5_vdpa_exec_async_cmds(mvdev, cmds, mr.num_directs);
    if (err) {
    mlx5_vdpa_err(mvdev, "error issuing MTT mkey creation for direct mrs: %d\n", err);
    goto done;
    }
    i = 0;
    list_for_each_entry(dmr, &mr.head, list) {
    struct mlx5_vdpa_async_cmd *cmd = &cmds[i++];
    struct mlx5_create_mkey_mem *cmd_mem;
    cmd_mem = container_of(cmd.out, struct mlx5_create_mkey_mem, out);
    if (!cmd.err) {
    create_direct_mr_end(mvdev, dmr, cmd_mem);
    } else {
    err = err ? err : cmd.err;
    mlx5_vdpa_err(mvdev, "error creating MTT mkey [0x%llx, 0x%llx]: %d\n",
    dmr.start, dmr.end, cmd.err);
    }
    }
    done:
    for (i = i-1; i >= 0; i--) {
    struct mlx5_create_mkey_mem *cmd_mem;
    cmd_mem = container_of(cmds[i].out, struct mlx5_create_mkey_mem, out);
    kvfree(cmd_mem);
    }
    kvfree(cmds);
    return err;
    }
    DEFINE_FREE(free_cmds, struct mlx5_vdpa_async_cmd *, kvfree(_T))
    DEFINE_FREE(free_cmd_mem, struct mlx5_destroy_mkey_mem *, kvfree(_T))
#[no_mangle]
unsafe extern "C" fn destroy_direct_keys(mvdev: *mut mlx5_vdpa_dev, mr: *mut mlx5_vdpa_mr) -> c_int {
    static int destroy_direct_keys(struct mlx5_vdpa_dev *mvdev, struct mlx5_vdpa_mr *mr)
    {
    struct mlx5_destroy_mkey_mem *cmd_mem __free(free_cmd_mem) = core::ptr::null_mut();
    struct mlx5_vdpa_async_cmd *cmds __free(free_cmds) = core::ptr::null_mut();
    struct mlx5_vdpa_direct_mr *dmr;
    let mut err: c_int = 0;
    let mut i: c_int = 0;
    cmds = kvzalloc_objs(*cmds, mr.num_directs);
    cmd_mem = kvzalloc_objs(*cmd_mem, mr.num_directs);
    if (!cmds || !cmd_mem)
    return -ENOMEM;
    list_for_each_entry(dmr, &mr.head, list) {
    cmds[i].out = cmd_mem[i].out;
    cmds[i].outlen = sizeof(cmd_mem[i].out);
    cmds[i].in = cmd_mem[i].in;
    cmds[i].inlen = sizeof(cmd_mem[i].in);
    fill_destroy_direct_mr(mvdev, dmr, &cmd_mem[i]);
    i++;
    }
    err = mlx5_vdpa_exec_async_cmds(mvdev, cmds, mr.num_directs);
    if (err) {
    mlx5_vdpa_err(mvdev, "error issuing MTT mkey deletion for direct mrs: %d\n", err);
    return err;
    }
    i = 0;
    list_for_each_entry(dmr, &mr.head, list) {
    struct mlx5_vdpa_async_cmd *cmd = &cmds[i++];
    dmr.mr = 0;
    if (cmd.err) {
    err = err ? err : cmd.err;
    mlx5_vdpa_err(mvdev, "error deleting MTT mkey [0x%llx, 0x%llx]: %d\n",
    dmr.start, dmr.end, cmd.err);
    }
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn create_indirect_key(mvdev: *mut mlx5_vdpa_dev, mr: *mut mlx5_vdpa_mr) -> c_int {
    static int create_indirect_key(struct mlx5_vdpa_dev *mvdev, struct mlx5_vdpa_mr *mr)
    {
    int inlen;
    void *mkc;
    void *in;
    int err;
    u64 start;
    u64 len;
    start = indir_start_addr(mr);
    len = indir_len(mr);
    if (start == MLX5_VDPA_INVALID_START_ADDR || len == MLX5_VDPA_INVALID_LEN)
    return -EINVAL;
    inlen = MLX5_ST_SZ_BYTES(create_mkey_in) + klm_byte_size(mr.num_klms);
    in = kzalloc(inlen, GFP_KERNEL);
    if (!in)
    return -ENOMEM;
    MLX5_SET(create_mkey_in, in, uid, mvdev.res.uid);
    mkc = MLX5_ADDR_OF(create_mkey_in, in, memory_key_mkey_entry);
    MLX5_SET(mkc, mkc, lw, 1);
    MLX5_SET(mkc, mkc, lr, 1);
    mlx5_set_access_mode(mkc, MLX5_MKC_ACCESS_MODE_KLMS);
    MLX5_SET(mkc, mkc, qpn, 0xffffff);
    MLX5_SET(mkc, mkc, pd, mvdev.res.pdn);
    MLX5_SET64(mkc, mkc, start_addr, start);
    MLX5_SET64(mkc, mkc, len, len);
    MLX5_SET(mkc, mkc, translations_octword_size, klm_byte_size(mr.num_klms) / 16);
    MLX5_SET(create_mkey_in, in, translations_octword_actual_size, mr.num_klms);
    fill_indir(mvdev, mr, in);
    err = mlx5_vdpa_create_mkey(mvdev, &mr.mkey, in, inlen);
    kfree(in);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn destroy_indirect_key(mvdev: *mut mlx5_vdpa_dev, mkey: *mut mlx5_vdpa_mr) {
    static void destroy_indirect_key(struct mlx5_vdpa_dev *mvdev, struct mlx5_vdpa_mr *mkey)
    {
    mlx5_vdpa_destroy_mkey(mvdev, mkey.mkey);
    }
    static int map_direct_mr(struct mlx5_vdpa_dev *mvdev, struct mlx5_vdpa_direct_mr *mr,
    struct vhost_iotlb *iotlb)
    {
    struct vhost_iotlb_map *map;
    let mut lgcd: c_ulong = 0;
    int log_entity_size;
    unsigned long size;
    int err;
    struct page *pg;
    unsigned int nsg;
    int sglen;
    u64 pa, offset;
    u64 paend;
    struct scatterlist *sg;
    struct device *dma = mvdev.vdev.vmap.dma_dev;
    for (map = vhost_iotlb_itree_first(iotlb, mr.start, mr.end - 1);
    map; map = vhost_iotlb_itree_next(map, mr.start, mr.end - 1)) {
    size = maplen(map, mr);
    lgcd = gcd(lgcd, size);
    }
    log_entity_size = ilog2(lgcd);
    sglen = 1 << log_entity_size;
    nsg = MLX5_DIV_ROUND_UP_POW2(mr.end - mr.start, log_entity_size);
    err = sg_alloc_table(&mr.sg_head, nsg, GFP_KERNEL);
    if (err)
    return err;
    sg = mr.sg_head.sgl;
    for (map = vhost_iotlb_itree_first(iotlb, mr.start, mr.end - 1);
    map; map = vhost_iotlb_itree_next(map, mr.start, mr.end - 1)) {
    offset = mr.start > map.start ? mr.start - map.start : 0;
    pa = map.addr + offset;
    paend = map.addr + offset + maplen(map, mr);
    for (; pa < paend; pa += sglen) {
    pg = pfn_to_page(__phys_to_pfn(pa));
    if (!sg) {
    mlx5_vdpa_warn(mvdev, "sg null. start 0x%llx, end 0x%llx\n",
    map.start, map.last + 1);
    err = -ENOMEM;
    goto err_map;
    }
    sg_set_page(sg, pg, sglen, 0);
    sg = sg_next(sg);
    if (!sg)
    goto done;
    }
    }
    done:
    mr.log_size = log_entity_size;
    mr.nsg = nsg;
    mr.nent = dma_map_sg_attrs(dma, mr.sg_head.sgl, mr.nsg, DMA_BIDIRECTIONAL, 0);
    if (!mr.nent) {
    err = -ENOMEM;
    goto err_map;
    }
    return 0;
    err_map:
    sg_free_table(&mr.sg_head);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn unmap_direct_mr(mvdev: *mut mlx5_vdpa_dev, mr: *mut mlx5_vdpa_direct_mr) {
    static void unmap_direct_mr(struct mlx5_vdpa_dev *mvdev, struct mlx5_vdpa_direct_mr *mr)
    {
    struct device *dma = mvdev.vdev.vmap.dma_dev;
    destroy_direct_mr(mvdev, mr);
    dma_unmap_sg_attrs(dma, mr.sg_head.sgl, mr.nsg, DMA_BIDIRECTIONAL, 0);
    sg_free_table(&mr.sg_head);
    }
    static int add_direct_chain(struct mlx5_vdpa_dev *mvdev,
    struct mlx5_vdpa_mr *mr,
    u64 start,
    u64 size,
    u8 perm,
    struct vhost_iotlb *iotlb)
    {
    struct mlx5_vdpa_direct_mr *dmr;
    struct mlx5_vdpa_direct_mr *n;
    LIST_HEAD(tmp);
    u64 st;
    u64 sz;
    int err;
    st = start;
    while (size) {
    sz = (u32)min_t(u64, MAX_KLM_SIZE, size);
    dmr = kzalloc_obj(*dmr);
    if (!dmr) {
    err = -ENOMEM;
    goto err_alloc;
    }
    dmr.start = st;
    dmr.end = st + sz;
    dmr.perm = perm;
    err = map_direct_mr(mvdev, dmr, iotlb);
    if (err) {
    kfree(dmr);
    goto err_alloc;
    }
    list_add_tail(&dmr.list, &tmp);
    size -= sz;
    mr.num_directs++;
    mr.num_klms++;
    st += sz;
    }
    list_splice_tail(&tmp, &mr.head);
    return 0;
    err_alloc:
    list_for_each_entry_safe(dmr, n, &tmp, list) {
    list_del_init(&dmr.list);
    unmap_direct_mr(mvdev, dmr);
    kfree(dmr);
    }
    return err;
    }
// The iotlb pointer contains a list of maps. Go over the maps, possibly
// merging mergeable maps, and create direct memory keys that provide the
// device access to memory. The direct mkeys are then referred to by the
// indirect memory key that provides access to the enitre address space given
// by iotlb.
//
    static int create_user_mr(struct mlx5_vdpa_dev *mvdev,
    struct mlx5_vdpa_mr *mr,
    struct vhost_iotlb *iotlb)
    {
    struct mlx5_vdpa_direct_mr *dmr;
    struct mlx5_vdpa_direct_mr *n;
    struct vhost_iotlb_map *map;
    let mut pperm: u32 = U16_MAX;
    let mut last: u64 = U64_MAX;
    let mut ps: u64 = U64_MAX;
    let mut pe: u64 = U64_MAX;
    let mut start: u64 = 0;
    let mut err: c_int = 0;
    int nnuls;
    INIT_LIST_HEAD(&mr.head);
    for (map = vhost_iotlb_itree_first(iotlb, start, last); map;
    map = vhost_iotlb_itree_next(map, start, last)) {
    start = map.start;
    if (pe == map.start && pperm == map.perm) {
    pe = map.last + 1;
    } else {
    if (ps != U64_MAX) {
    if (pe < map.start) {
// We have a hole in the map. Check how
// many null keys are required to fill it.
//
    nnuls = MLX5_DIV_ROUND_UP_POW2(map.start - pe,
    LOG_MAX_KLM_SIZE);
    mr.num_klms += nnuls;
    }
    err = add_direct_chain(mvdev, mr, ps, pe - ps, pperm, iotlb);
    if (err)
    goto err_chain;
    }
    ps = map.start;
    pe = map.last + 1;
    pperm = map.perm;
    }
    }
    err = add_direct_chain(mvdev, mr, ps, pe - ps, pperm, iotlb);
    if (err)
    goto err_chain;
    err = create_direct_keys(mvdev, mr);
    if (err)
    goto err_chain;
// Create the memory key that defines the guests's address space. This
// memory key refers to the direct keys that contain the MTT
// translations
//
    err = create_indirect_key(mvdev, mr);
    if (err)
    goto err_chain;
    mr.user_mr = true;
    return 0;
    err_chain:
    list_for_each_entry_safe_reverse(dmr, n, &mr.head, list) {
    list_del_init(&dmr.list);
    unmap_direct_mr(mvdev, dmr);
    kfree(dmr);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn create_dma_mr(mvdev: *mut mlx5_vdpa_dev, mr: *mut mlx5_vdpa_mr) -> c_int {
    static int create_dma_mr(struct mlx5_vdpa_dev *mvdev, struct mlx5_vdpa_mr *mr)
    {
    let mut inlen: c_int = MLX5_ST_SZ_BYTES(create_mkey_in);
    void *mkc;
    u32 *in;
    int err;
    in = kzalloc(inlen, GFP_KERNEL);
    if (!in)
    return -ENOMEM;
    mkc = MLX5_ADDR_OF(create_mkey_in, in, memory_key_mkey_entry);
    MLX5_SET(mkc, mkc, access_mode_1_0, MLX5_MKC_ACCESS_MODE_PA);
    MLX5_SET(mkc, mkc, length64, 1);
    MLX5_SET(mkc, mkc, lw, 1);
    MLX5_SET(mkc, mkc, lr, 1);
    MLX5_SET(mkc, mkc, pd, mvdev.res.pdn);
    MLX5_SET(mkc, mkc, qpn, 0xffffff);
    err = mlx5_vdpa_create_mkey(mvdev, &mr.mkey, in, inlen);
    if (!err)
    mr.user_mr = false;
    kfree(in);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn destroy_dma_mr(mvdev: *mut mlx5_vdpa_dev, mr: *mut mlx5_vdpa_mr) {
    static void destroy_dma_mr(struct mlx5_vdpa_dev *mvdev, struct mlx5_vdpa_mr *mr)
    {
    mlx5_vdpa_destroy_mkey(mvdev, mr.mkey);
    }
#[no_mangle]
unsafe extern "C" fn dup_iotlb(dst: *mut vhost_iotlb, src: *mut vhost_iotlb) -> c_int {
    static int dup_iotlb(struct vhost_iotlb *dst, struct vhost_iotlb *src)
    {
    struct vhost_iotlb_map *map;
    let mut start: u64 = 0, last = ULLONG_MAX;
    int err;
    if (dst == src)
    return -EINVAL;
    if (!src) {
    err = vhost_iotlb_add_range(dst, start, last, start, VHOST_ACCESS_RW);
    return err;
    }
    for (map = vhost_iotlb_itree_first(src, start, last); map;
    map = vhost_iotlb_itree_next(map, start, last)) {
    err = vhost_iotlb_add_range(dst, map.start, map.last,
    map.addr, map.perm);
    if (err)
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn prune_iotlb(iotlb: *mut vhost_iotlb) {
    static void prune_iotlb(struct vhost_iotlb *iotlb)
    {
    vhost_iotlb_del_range(iotlb, 0, ULLONG_MAX);
    }
#[no_mangle]
unsafe extern "C" fn destroy_user_mr(mvdev: *mut mlx5_vdpa_dev, mr: *mut mlx5_vdpa_mr) {
    static void destroy_user_mr(struct mlx5_vdpa_dev *mvdev, struct mlx5_vdpa_mr *mr)
    {
    struct mlx5_vdpa_direct_mr *dmr;
    struct mlx5_vdpa_direct_mr *n;
    destroy_indirect_key(mvdev, mr);
    destroy_direct_keys(mvdev, mr);
    list_for_each_entry_safe_reverse(dmr, n, &mr.head, list) {
    list_del_init(&dmr.list);
    unmap_direct_mr(mvdev, dmr);
    kfree(dmr);
    }
    }
#[no_mangle]
unsafe extern "C" fn _mlx5_vdpa_destroy_mr(mvdev: *mut mlx5_vdpa_dev, mr: *mut mlx5_vdpa_mr) {
    static void _mlx5_vdpa_destroy_mr(struct mlx5_vdpa_dev *mvdev, struct mlx5_vdpa_mr *mr)
    {
    if (WARN_ON(!mr))
    return;
    if (mr.user_mr)
    destroy_user_mr(mvdev, mr);
    else
    destroy_dma_mr(mvdev, mr);
    vhost_iotlb_free(mr.iotlb);
    list_del(&mr.mr_list);
    kfree(mr);
    }
// There can be multiple .set_map() operations in quick succession.
// This large delay is a simple way to prevent the MR cleanup from blocking
// .set_map() MR creation in this scenario.
//
pub const MLX5_VDPA_MR_GC_TRIGGER_MS: c_int = 2000;
#[no_mangle]
unsafe extern "C" fn mlx5_vdpa_mr_gc_handler(work: *mut work_struct) {
    static void mlx5_vdpa_mr_gc_handler(struct work_struct *work)
    {
    struct mlx5_vdpa_mr_resources *mres;
    struct mlx5_vdpa_mr *mr, *tmp;
    struct mlx5_vdpa_dev *mvdev;
    mres = container_of(work, struct mlx5_vdpa_mr_resources, gc_dwork_ent.work);
    if (atomic_read(&mres.shutdown)) {
    mutex_lock(&mres.lock);
    } else if (!mutex_trylock(&mres.lock)) {
    queue_delayed_work(mres.wq_gc, &mres.gc_dwork_ent,
    msecs_to_jiffies(MLX5_VDPA_MR_GC_TRIGGER_MS));
    return;
    }
    mvdev = container_of(mres, struct mlx5_vdpa_dev, mres);
    list_for_each_entry_safe(mr, tmp, &mres.mr_gc_list_head, mr_list) {
    _mlx5_vdpa_destroy_mr(mvdev, mr);
    }
    mutex_unlock(&mres.lock);
    }
    static void _mlx5_vdpa_put_mr(struct mlx5_vdpa_dev *mvdev,
    struct mlx5_vdpa_mr *mr)
    {
    struct mlx5_vdpa_mr_resources *mres = &mvdev.mres;
    if (!mr)
    return;
    if (refcount_dec_and_test(&mr.refcount)) {
    list_move_tail(&mr.mr_list, &mres.mr_gc_list_head);
    queue_delayed_work(mres.wq_gc, &mres.gc_dwork_ent,
    msecs_to_jiffies(MLX5_VDPA_MR_GC_TRIGGER_MS));
    }
    }
    void mlx5_vdpa_put_mr(struct mlx5_vdpa_dev *mvdev,
    struct mlx5_vdpa_mr *mr)
    {
    mutex_lock(&mvdev.mres.lock);
    _mlx5_vdpa_put_mr(mvdev, mr);
    mutex_unlock(&mvdev.mres.lock);
    }
    static void _mlx5_vdpa_get_mr(struct mlx5_vdpa_dev *mvdev,
    struct mlx5_vdpa_mr *mr)
    {
    if (!mr)
    return;
    refcount_inc(&mr.refcount);
    }
    void mlx5_vdpa_get_mr(struct mlx5_vdpa_dev *mvdev,
    struct mlx5_vdpa_mr *mr)
    {
    mutex_lock(&mvdev.mres.lock);
    _mlx5_vdpa_get_mr(mvdev, mr);
    mutex_unlock(&mvdev.mres.lock);
    }
    void mlx5_vdpa_update_mr(struct mlx5_vdpa_dev *mvdev,
    struct mlx5_vdpa_mr *new_mr,
    unsigned int asid)
    {
    struct mlx5_vdpa_mr *old_mr = mvdev.mres.mr[asid];
    mutex_lock(&mvdev.mres.lock);
    _mlx5_vdpa_put_mr(mvdev, old_mr);
    mvdev.mres.mr[asid] = new_mr;
    mutex_unlock(&mvdev.mres.lock);
    }
#[no_mangle]
unsafe extern "C" fn mlx5_vdpa_show_mr_leaks(mvdev: *mut mlx5_vdpa_dev) {
    static void mlx5_vdpa_show_mr_leaks(struct mlx5_vdpa_dev *mvdev)
    {
    struct mlx5_vdpa_mr *mr;
    mutex_lock(&mvdev.mres.lock);
    list_for_each_entry(mr, &mvdev.mres.mr_list_head, mr_list) {
    mlx5_vdpa_warn(mvdev, "mkey still alive after resource delete: "
    "mr: %p, mkey: 0x%x, refcount: %u\n",
    mr, mr.mkey, refcount_read(&mr.refcount));
    }
    mutex_unlock(&mvdev.mres.lock);
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5_vdpa_clean_mrs(mvdev: *mut mlx5_vdpa_dev) {
    void mlx5_vdpa_clean_mrs(struct mlx5_vdpa_dev *mvdev)
    {
    if (!mvdev.res.valid)
    return;
    for (int i = 0; i < MLX5_VDPA_NUM_AS; i++)
    mlx5_vdpa_update_mr(mvdev, core::ptr::null_mut(), i);
    prune_iotlb(mvdev.cvq.iotlb);
    mlx5_vdpa_show_mr_leaks(mvdev);
    }
    static int _mlx5_vdpa_create_mr(struct mlx5_vdpa_dev *mvdev,
    struct mlx5_vdpa_mr *mr,
    struct vhost_iotlb *iotlb)
    {
    int err;
    if (mlx5_vdpa_max_iotlb_entries < 2)
    return -EINVAL;
    if (iotlb)
    err = create_user_mr(mvdev, mr, iotlb);
    else
    err = create_dma_mr(mvdev, mr);
    if (err)
    return err;
    mr.iotlb = vhost_iotlb_alloc(mlx5_vdpa_max_iotlb_entries, 0);
    if (!mr.iotlb) {
    err = -ENOMEM;
    goto err_mr;
    }
    err = dup_iotlb(mr.iotlb, iotlb);
    if (err)
    goto err_iotlb;
    list_add_tail(&mr.mr_list, &mvdev.mres.mr_list_head);
    return 0;
    err_iotlb:
    vhost_iotlb_free(mr.iotlb);
    err_mr:
    if (iotlb)
    destroy_user_mr(mvdev, mr);
    else
    destroy_dma_mr(mvdev, mr);
    return err;
    }
    struct mlx5_vdpa_mr *mlx5_vdpa_create_mr(struct mlx5_vdpa_dev *mvdev,
    struct vhost_iotlb *iotlb)
    {
    struct mlx5_vdpa_mr *mr;
    int err;
    mr = kzalloc_obj(*mr);
    if (!mr)
    return ERR_PTR(-ENOMEM);
    mutex_lock(&mvdev.mres.lock);
    err = _mlx5_vdpa_create_mr(mvdev, mr, iotlb);
    mutex_unlock(&mvdev.mres.lock);
    if (err)
    goto out_err;
    refcount_set(&mr.refcount, 1);
    return mr;
    out_err:
    kfree(mr);
    return ERR_PTR(err);
    }
    int mlx5_vdpa_update_cvq_iotlb(struct mlx5_vdpa_dev *mvdev,
    struct vhost_iotlb *iotlb,
    unsigned int asid)
    {
    int err;
    if (mvdev.mres.group2asid[MLX5_VDPA_CVQ_GROUP] != asid)
    return 0;
    spin_lock(&mvdev.cvq.iommu_lock);
    prune_iotlb(mvdev.cvq.iotlb);
    err = dup_iotlb(mvdev.cvq.iotlb, iotlb);
    spin_unlock(&mvdev.cvq.iommu_lock);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5_vdpa_create_dma_mr(mvdev: *mut mlx5_vdpa_dev) -> c_int {
    int mlx5_vdpa_create_dma_mr(struct mlx5_vdpa_dev *mvdev)
    {
    struct mlx5_vdpa_mr *mr;
    mr = mlx5_vdpa_create_mr(mvdev, core::ptr::null_mut());
    if (IS_ERR(mr))
    return PTR_ERR(mr);
    mlx5_vdpa_update_mr(mvdev, mr, 0);
    return mlx5_vdpa_update_cvq_iotlb(mvdev, core::ptr::null_mut(), 0);
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5_vdpa_reset_mr(mvdev: *mut mlx5_vdpa_dev, asid: c_uint) -> c_int {
    int mlx5_vdpa_reset_mr(struct mlx5_vdpa_dev *mvdev, unsigned int asid)
    {
    if (asid >= MLX5_VDPA_NUM_AS)
    return -EINVAL;
    mlx5_vdpa_update_mr(mvdev, core::ptr::null_mut(), asid);
    if (asid == 0 && MLX5_CAP_GEN(mvdev.mdev, umem_uid_0)) {
    if (mlx5_vdpa_create_dma_mr(mvdev))
    mlx5_vdpa_warn(mvdev, "create DMA MR failed\n");
    } else {
    mlx5_vdpa_update_cvq_iotlb(mvdev, core::ptr::null_mut(), asid);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5_vdpa_init_mr_resources(mvdev: *mut mlx5_vdpa_dev) -> c_int {
    int mlx5_vdpa_init_mr_resources(struct mlx5_vdpa_dev *mvdev)
    {
    struct mlx5_vdpa_mr_resources *mres = &mvdev.mres;
    mres.wq_gc = create_singlethread_workqueue("mlx5_vdpa_mr_gc");
    if (!mres.wq_gc)
    return -ENOMEM;
    INIT_DELAYED_WORK(&mres.gc_dwork_ent, mlx5_vdpa_mr_gc_handler);
    mutex_init(&mres.lock);
    INIT_LIST_HEAD(&mres.mr_list_head);
    INIT_LIST_HEAD(&mres.mr_gc_list_head);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5_vdpa_destroy_mr_resources(mvdev: *mut mlx5_vdpa_dev) {
    void mlx5_vdpa_destroy_mr_resources(struct mlx5_vdpa_dev *mvdev)
    {
    struct mlx5_vdpa_mr_resources *mres = &mvdev.mres;
    if (!mres.wq_gc)
    return;
    atomic_set(&mres.shutdown, 1);
    flush_delayed_work(&mres.gc_dwork_ent);
    destroy_workqueue(mres.wq_gc);
    mres.wq_gc = core::ptr::null_mut();
    mutex_destroy(&mres.lock);
    }
