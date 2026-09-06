//! Automatically rewritten from C to Rust
//! Source: fs/nfs/filelayout/filelayoutdev.c
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
// Device operations for the pnfs nfs4 file layout driver.
//
// Copyright (c) 2002
// The Regents of the University of Michigan
// All Rights Reserved
//
// Dean Hildebrand <dhildebz@umich.edu>
// Garth Goodson   <Garth.Goodson@netapp.com>
//
// Permission is granted to use, copy, create derivative works, and
// redistribute this software and such derivative works for any purpose,
// so long as the name of the University of Michigan is not used in
// any advertising or publicity pertaining to the use or distribution
// of this software without specific, written prior authorization. If
// the above copyright notice or any other identification of the
// University of Michigan is included in any copy of any portion of
// this software, then the disclaimer below must also be included.
//
// This software is provided as is, without representation or warranty
// of any kind either express or implied, including without limitation
// the implied warranties of merchantability, fitness for a particular
// purpose, or noninfringement.  The Regents of the University of
// Michigan shall not be liable for any damages, including special,
// indirect, incidental, or consequential damages, with respect to any
// claim arising out of or in connection with the use of the software,
// even if it has been or is hereafter advised of the possibility of
// such damages.
//

    let mut dataserver_timeo: static unsigned int = NFS4_DEF_DS_TIMEO;
    let mut dataserver_retrans: static unsigned int = NFS4_DEF_DS_RETRANS;
    void
    nfs4_fl_free_deviceid(struct nfs4_file_layout_dsaddr *dsaddr)
    {
    struct nfs4_pnfs_ds *ds;
    int i;
    nfs4_print_deviceid(&dsaddr.id_node.deviceid);
    for (i = 0; i < dsaddr.ds_num; i++) {
    ds = dsaddr.ds_list[i];
    if (ds != core::ptr::null_mut())
    nfs4_pnfs_ds_put(ds);
    }
    kfree(dsaddr.stripe_indices);
    kfree_rcu(dsaddr, id_node.rcu);
    }
// Decode opaque device data and return the result
    struct nfs4_file_layout_dsaddr *
    nfs4_fl_alloc_deviceid_node(struct nfs_server *server, struct pnfs_device *pdev,
    gfp_t gfp_flags)
    {
    int i;
    u32 cnt, num;
    u8 *indexp;
    __be32 *p;
    u8 *stripe_indices;
    u8 max_stripe_index;
    struct nfs4_file_layout_dsaddr *dsaddr = core::ptr::null_mut();
    struct xdr_stream stream;
    struct xdr_buf buf;
    struct folio *scratch;
    struct list_head dsaddrs;
    struct nfs4_pnfs_ds_addr *da;
    struct net *net = server.nfs_client.cl_net;
// set up xdr stream
    scratch = folio_alloc(gfp_flags, 0);
    if (!scratch)
    goto out_err;
    xdr_init_decode_pages(&stream, &buf, pdev.pages, pdev.pglen);
    xdr_set_scratch_folio(&stream, scratch);
// Get the stripe count (number of stripe index)
    p = xdr_inline_decode(&stream, 4);
    if (unlikely(!p))
    goto out_err_free_scratch;
    cnt = be32_to_cpup(p);
    dprintk("%s stripe count  %d\n", __func__, cnt);
    if (cnt > NFS4_PNFS_MAX_STRIPE_CNT) {
    printk(KERN_WARNING "NFS: %s: stripe count %d greater than "
    "supported maximum %d\n", __func__,
    cnt, NFS4_PNFS_MAX_STRIPE_CNT);
    goto out_err_free_scratch;
    }
// read stripe indices
    stripe_indices = kcalloc(cnt, sizeof(u8), gfp_flags);
    if (!stripe_indices)
    goto out_err_free_scratch;
    p = xdr_inline_decode(&stream, cnt << 2);
    if (unlikely(!p))
    goto out_err_free_stripe_indices;
    indexp = &stripe_indices[0];
    max_stripe_index = 0;
    for (i = 0; i < cnt; i++) {
// indexp = be32_to_cpup(p++);
    max_stripe_index = max(max_stripe_index, *indexp);
    indexp++;
    }
// Check the multipath list count
    p = xdr_inline_decode(&stream, 4);
    if (unlikely(!p))
    goto out_err_free_stripe_indices;
    num = be32_to_cpup(p);
    dprintk("%s ds_num %u\n", __func__, num);
    if (num > NFS4_PNFS_MAX_MULTI_CNT) {
    printk(KERN_WARNING "NFS: %s: multipath count %d greater than "
    "supported maximum %d\n", __func__,
    num, NFS4_PNFS_MAX_MULTI_CNT);
    goto out_err_free_stripe_indices;
    }
// validate stripe indices are all < num
    if (max_stripe_index >= num) {
    printk(KERN_WARNING "NFS: %s: stripe index %u >= num ds %u\n",
    __func__, max_stripe_index, num);
    goto out_err_free_stripe_indices;
    }
    dsaddr = kzalloc_flex(*dsaddr, ds_list, num, gfp_flags);
    if (!dsaddr)
    goto out_err_free_stripe_indices;
    dsaddr.stripe_count = cnt;
    dsaddr.stripe_indices = stripe_indices;
    stripe_indices = core::ptr::null_mut();
    dsaddr.ds_num = num;
    nfs4_init_deviceid_node(&dsaddr.id_node, server, &pdev.dev_id);
    INIT_LIST_HEAD(&dsaddrs);
    for (i = 0; i < dsaddr.ds_num; i++) {
    int j;
    u32 mp_count;
    p = xdr_inline_decode(&stream, 4);
    if (unlikely(!p))
    goto out_err_free_deviceid;
    mp_count = be32_to_cpup(p); /* multipath count */
    for (j = 0; j < mp_count; j++) {
    da = nfs4_decode_mp_ds_addr(net, &stream, gfp_flags);
    if (da)
    list_add_tail(&da.da_node, &dsaddrs);
    }
    if (list_empty(&dsaddrs)) {
    dprintk("%s: no suitable DS addresses found\n",
    __func__);
    goto out_err_free_deviceid;
    }
    dsaddr.ds_list[i] = nfs4_pnfs_ds_add(net, &dsaddrs, 4,
    gfp_flags);
    if (!dsaddr.ds_list[i])
    goto out_err_drain_dsaddrs;
    trace_fl_getdevinfo(server, &pdev.dev_id, dsaddr.ds_list[i].ds_remotestr);
// If DS was already in cache, free ds addrs
    while (!list_empty(&dsaddrs)) {
    da = list_first_entry(&dsaddrs,
    struct nfs4_pnfs_ds_addr,
    da_node);
    list_del_init(&da.da_node);
    kfree(da.da_remotestr);
    kfree(da);
    }
    }
    folio_put(scratch);
    return dsaddr;
    out_err_drain_dsaddrs:
    while (!list_empty(&dsaddrs)) {
    da = list_first_entry(&dsaddrs, struct nfs4_pnfs_ds_addr,
    da_node);
    list_del_init(&da.da_node);
    kfree(da.da_remotestr);
    kfree(da);
    }
    out_err_free_deviceid:
    nfs4_fl_free_deviceid(dsaddr);
// stripe_indicies was part of dsaddr
    goto out_err_free_scratch;
    out_err_free_stripe_indices:
    kfree(stripe_indices);
    out_err_free_scratch:
    folio_put(scratch);
    out_err:
    dprintk("%s ERROR: returning core::ptr::null_mut()\n", __func__);
    return core::ptr::null_mut();
    }
    void
    nfs4_fl_put_deviceid(struct nfs4_file_layout_dsaddr *dsaddr)
    {
    nfs4_put_deviceid_node(&dsaddr.id_node);
    }
//
// Want res = (offset - layout->pattern_offset)/ layout->stripe_unit
// Then: ((res + fsi) % dsaddr->stripe_count)
//
    u32
    nfs4_fl_calc_j_index(struct pnfs_layout_segment *lseg, loff_t offset)
    {
    struct nfs4_filelayout_segment *flseg = FILELAYOUT_LSEG(lseg);
    u64 tmp;
    tmp = offset - flseg.pattern_offset;
    do_div(tmp, flseg.stripe_unit);
    tmp += flseg.first_stripe_index;
    return do_div(tmp, flseg.dsaddr.stripe_count);
    }
    u32
    nfs4_fl_calc_ds_index(struct pnfs_layout_segment *lseg, u32 j)
    {
    return FILELAYOUT_LSEG(lseg).dsaddr.stripe_indices[j];
    }
    struct nfs_fh *
    nfs4_fl_select_ds_fh(struct pnfs_layout_segment *lseg, u32 j)
    {
    struct nfs4_filelayout_segment *flseg = FILELAYOUT_LSEG(lseg);
    u32 i;
    if (flseg.stripe_type == STRIPE_SPARSE) {
    if (flseg.num_fh == 1)
    i = 0;
#[no_mangle]
pub unsafe extern "C" fn if(0: flseg->num_fh ==) -> else {
    else if (flseg.num_fh == 0)
// Use the MDS OPEN fh set in nfs_read_rpcsetup
    return core::ptr::null_mut();
    else
    i = nfs4_fl_calc_ds_index(lseg, j);
    } else
    i = j;
    return flseg.fh_array[i];
    }
// Upon return, either ds is connected, or ds is NULL
    struct nfs4_pnfs_ds *
    nfs4_fl_prepare_ds(struct pnfs_layout_segment *lseg, u32 ds_idx)
    {
    struct nfs4_file_layout_dsaddr *dsaddr = FILELAYOUT_LSEG(lseg).dsaddr;
    struct nfs4_pnfs_ds *ds = dsaddr.ds_list[ds_idx];
    struct nfs4_deviceid_node *devid = FILELAYOUT_DEVID_NODE(lseg);
    struct nfs4_pnfs_ds *ret = ds;
    struct nfs_server *s = NFS_SERVER(lseg.pls_layout.plh_inode);
    int status;
    if (ds == core::ptr::null_mut()) {
    printk(KERN_ERR "NFS: %s: No data server for offset index %d\n",
    __func__, ds_idx);
    pnfs_generic_mark_devid_invalid(devid);
    goto out;
    }
    smp_rmb();
    if (ds.ds_clp)
    goto out_test_devid;
    status = nfs4_pnfs_ds_connect(s, ds, devid, dataserver_timeo,
    dataserver_retrans, 4,
    s.nfs_client.cl_minorversion, true);
    if (status) {
    nfs4_mark_deviceid_unavailable(devid);
    ret = core::ptr::null_mut();
    goto out;
    }
    out_test_devid:
    if (ret.ds_clp == core::ptr::null_mut() ||
    filelayout_test_devid_unavailable(devid))
    ret = core::ptr::null_mut();
    out:
    return ret;
    }
    module_param(dataserver_retrans, uint, 0644);
    MODULE_PARM_DESC(dataserver_retrans, "The  number of times the NFSv4.1 client "
    "retries a request before it attempts further "
    " recovery  action.");
    module_param(dataserver_timeo, uint, 0644);
    MODULE_PARM_DESC(dataserver_timeo, "The time (in tenths of a second) the "
    "NFSv4.1  client  waits for a response from a "
    " data server before it retries an NFS request.");
