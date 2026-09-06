//! Automatically rewritten from C to Rust
//! Source: arch/s390/pci/pci_clp.c
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
//
// Copyright IBM Corp. 2012
//
// Author(s):
// Jan Glauber <jang@linux.vnet.ibm.com>
//

    bool zpci_unique_uid;
#[no_mangle]
pub unsafe extern "C" fn update_uid_checking(new: bool) {
    void update_uid_checking(bool new)
    {
    if (zpci_unique_uid != new)
    zpci_dbg(3, "uid checking:%d\n", new);
    zpci_unique_uid = new;
    }
#[no_mangle]
pub unsafe extern "C" fn zpci_err_clp(rsp: c_uint, rc: c_int) {
    static inline void zpci_err_clp(unsigned int rsp, int rc)
    {
    struct {
    unsigned int rsp;
    int rc;
    } __packed data = {rsp, rc};
    zpci_err_hex(&data, sizeof(data));
    }
//
// Call Logical Processor with c=1, lps=0 and command 1
// to get the bit mask of installed logical processors
//
#[no_mangle]
pub unsafe extern "C" fn clp_get_ilp(ilp: *mut c_ulong) -> c_int {
    static inline int clp_get_ilp(unsigned long *ilp)
    {
    unsigned long mask;
    int cc, exception;
    exception = 1;
    asm_inline volatile (
    "	.insn	rrf,0xb9a00000,%[mask],%[cmd],8,0\n"
    "0:	lhi	%[exc],0\n"
    "1:\n"
    CC_IPM(cc)
    EX_TABLE(0b, 1b)
    : CC_OUT(cc, cc), [mask] "=d" (mask), [exc] "+d" (exception)
    : [cmd] "a" (1)
    : CC_CLOBBER);
// ilp = mask;
    return exception ? 3 : CC_TRANSFORM(cc);
    }
//
// Call Logical Processor with c=0, the give constant lps and an lpcb request.
//
#[no_mangle]
unsafe extern "C" fn clp_req(data: *mut c_void, lps: c_uint) -> __always_inline int {
    static __always_inline int clp_req(void *data, unsigned int lps)
    {
    struct { u8 _[CLP_BLK_SIZE]; } *req = data;
    int cc, exception;
    u64 ignored;
    exception = 1;
    asm_inline volatile (
    "	.insn	rrf,0xb9a00000,%[ign],%[req],0,%[lps]\n"
    "0:	lhi	%[exc],0\n"
    "1:\n"
    CC_IPM(cc)
    EX_TABLE(0b, 1b)
    : CC_OUT(cc, cc), [ign] "=d" (ignored), "+m" (*req), [exc] "+d" (exception)
    : [req] "a" (req), [lps] "i" (lps)
    : CC_CLOBBER);
    return exception ? 3 : CC_TRANSFORM(cc);
    }
    static void *clp_alloc_block(gfp_t gfp_mask)
    {
    return (void *) __get_free_pages(gfp_mask, get_order(CLP_BLK_SIZE));
    }
#[no_mangle]
unsafe extern "C" fn clp_free_block(ptr: *mut c_void) {
    static void clp_free_block(void *ptr)
    {
    free_pages((unsigned long) ptr, get_order(CLP_BLK_SIZE));
    }
    static void clp_store_query_pci_fngrp(struct zpci_dev *zdev,
    struct clp_rsp_query_pci_grp *response)
    {
    zdev.tlb_refresh = response.refresh;
    zdev.dma_mask = response.dasm;
    zdev.msi_addr = response.msia;
    zdev.max_msi = response.noi;
    zdev.fmb_update = response.mui;
    zdev.version = response.version;
    zdev.maxstbl = response.maxstbl;
    zdev.dtsm = response.dtsm;
    zdev.rtr_avail = response.rtr;
    switch (response.version) {
    case 1:
    zdev.max_bus_speed = PCIE_SPEED_5_0GT;
    break;
    default:
    zdev.max_bus_speed = PCI_SPEED_UNKNOWN;
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn clp_query_pci_fngrp(zdev: *mut zpci_dev, pfgid: u8) -> c_int {
    static int clp_query_pci_fngrp(struct zpci_dev *zdev, u8 pfgid)
    {
    struct clp_req_rsp_query_pci_grp *rrb;
    int rc;
    rrb = clp_alloc_block(GFP_KERNEL);
    if (!rrb)
    return -ENOMEM;
    memset(rrb, 0, sizeof(*rrb));
    rrb.request.hdr.len = sizeof(rrb.request);
    rrb.request.hdr.cmd = CLP_QUERY_PCI_FNGRP;
    rrb.response.hdr.len = sizeof(rrb.response);
    rrb.request.pfgid = pfgid;
    rc = clp_req(rrb, CLP_LPS_PCI);
    if (!rc && rrb.response.hdr.rsp == CLP_RC_OK)
    clp_store_query_pci_fngrp(zdev, &rrb.response);
    else {
    zpci_err("Q PCI FGRP:\n");
    zpci_err_clp(rrb.response.hdr.rsp, rc);
    rc = -EIO;
    }
    clp_free_block(rrb);
    return rc;
    }
    static int clp_store_query_pci_fn(struct zpci_dev *zdev,
    struct clp_rsp_query_pci *response)
    {
    int i;
    for (i = 0; i < PCI_STD_NUM_BARS; i++) {
    zdev.bars[i].val = le32_to_cpu(response.bar[i]);
    zdev.bars[i].size = response.bar_size[i];
    }
    zdev.start_dma = response.sdma;
    zdev.end_dma = response.edma;
    zdev.pchid = response.pchid;
    zdev.pfgid = response.pfgid;
    zdev.pft = response.pft;
    zdev.vfn = response.vfn;
    zdev.port = response.port;
    zdev.fidparm = response.fidparm;
    zdev.uid = response.uid;
    zdev.fmb_length = sizeof(u32) * response.fmb_len;
    zdev.is_physfn = response.is_physfn;
    zdev.rid_available = response.rid_avail;
    if (zdev.rid_available)
    zdev.rid = response.rid;
    zdev.tid_avail = response.tid_avail;
    if (zdev.tid_avail)
    zdev.tid = response.tid;
    memcpy(zdev.pfip, response.pfip, sizeof(zdev.pfip));
    if (response.util_str_avail) {
    memcpy(zdev.util_str, response.util_str,
    sizeof(zdev.util_str));
    zdev.util_str_avail = 1;
    }
    zdev.mio_capable = response.mio_addr_avail;
    for (i = 0; i < PCI_STD_NUM_BARS; i++) {
    if (!(response.mio.valid & (1 << (PCI_STD_NUM_BARS - i - 1))))
    continue;
    zdev.bars[i].mio_wb = (void __iomem *) response.mio.addr[i].wb;
    zdev.bars[i].mio_wt = (void __iomem *) response.mio.addr[i].wt;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn clp_query_pci_fn(zdev: *mut zpci_dev) -> c_int {
    int clp_query_pci_fn(struct zpci_dev *zdev)
    {
    struct clp_req_rsp_query_pci *rrb;
    int rc;
    rrb = clp_alloc_block(GFP_KERNEL);
    if (!rrb)
    return -ENOMEM;
    memset(rrb, 0, sizeof(*rrb));
    rrb.request.hdr.len = sizeof(rrb.request);
    rrb.request.hdr.cmd = CLP_QUERY_PCI_FN;
    rrb.response.hdr.len = sizeof(rrb.response);
    rrb.request.fh = zdev.fh;
    rc = clp_req(rrb, CLP_LPS_PCI);
    if (!rc && rrb.response.hdr.rsp == CLP_RC_OK) {
    rc = clp_store_query_pci_fn(zdev, &rrb.response);
    if (rc)
    goto out;
    rc = clp_query_pci_fngrp(zdev, rrb.response.pfgid);
    } else {
    zpci_err("Q PCI FN:\n");
    zpci_err_clp(rrb.response.hdr.rsp, rc);
    rc = -EIO;
    }
    out:
    clp_free_block(rrb);
    return rc;
    }
//
// clp_set_pci_fn() - Execute a command on a PCI function
// @zdev: Function that will be affected
// @fh: Out parameter for updated function handle
// @nr_dma_as: DMA address space number
// @command: The command code to execute
//
// Returns: 0 on success, < 0 for Linux errors (e.g. -ENOMEM), and
// > 0 for non-success platform responses
//
#[no_mangle]
unsafe extern "C" fn clp_set_pci_fn(zdev: *mut zpci_dev, fh: *mut u32, nr_dma_as: u8, command: u8) -> c_int {
    static int clp_set_pci_fn(struct zpci_dev *zdev, u32 *fh, u8 nr_dma_as, u8 command)
    {
    struct clp_req_rsp_set_pci *rrb;
    int rc, retries = 100;
    let mut gisa: u32 = 0;
// fh = 0;
    rrb = clp_alloc_block(GFP_KERNEL);
    if (!rrb)
    return -ENOMEM;
    if (command != CLP_SET_DISABLE_PCI_FN)
    gisa = zdev.gisa;
    do {
    memset(rrb, 0, sizeof(*rrb));
    rrb.request.hdr.len = sizeof(rrb.request);
    rrb.request.hdr.cmd = CLP_SET_PCI_FN;
    rrb.response.hdr.len = sizeof(rrb.response);
    rrb.request.fh = zdev.fh;
    rrb.request.oc = command;
    rrb.request.ndas = nr_dma_as;
    rrb.request.gisa = gisa;
    rc = clp_req(rrb, CLP_LPS_PCI);
    if (rrb.response.hdr.rsp == CLP_RC_SETPCIFN_BUSY) {
    retries--;
    if (retries < 0)
    break;
    msleep(20);
    }
    } while (rrb.response.hdr.rsp == CLP_RC_SETPCIFN_BUSY);
    if (!rc && rrb.response.hdr.rsp == CLP_RC_OK) {
// fh = rrb->response.fh;
    } else {
    zpci_err("Set PCI FN:\n");
    zpci_err_clp(rrb.response.hdr.rsp, rc);
    if (!rc)
    rc = rrb.response.hdr.rsp;
    }
    clp_free_block(rrb);
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn clp_setup_writeback_mio() -> c_int {
    int clp_setup_writeback_mio(void)
    {
    struct clp_req_rsp_slpc_pci *rrb;
    u8  wb_bit_pos;
    int rc;
    rrb = clp_alloc_block(GFP_KERNEL);
    if (!rrb)
    return -ENOMEM;
    memset(rrb, 0, sizeof(*rrb));
    rrb.request.hdr.len = sizeof(rrb.request);
    rrb.request.hdr.cmd = CLP_SLPC;
    rrb.response.hdr.len = sizeof(rrb.response);
    rc = clp_req(rrb, CLP_LPS_PCI);
    if (!rc && rrb.response.hdr.rsp == CLP_RC_OK) {
    if (rrb.response.vwb) {
    wb_bit_pos = rrb.response.mio_wb;
    set_bit_inv(wb_bit_pos, &mio_wb_bit_mask);
    zpci_dbg(3, "wb bit: %d\n", wb_bit_pos);
    } else {
    zpci_dbg(3, "wb bit: n.a.\n");
    }
    } else {
    zpci_err("SLPC PCI:\n");
    zpci_err_clp(rrb.response.hdr.rsp, rc);
    rc = -EIO;
    }
    clp_free_block(rrb);
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn clp_enable_fh(zdev: *mut zpci_dev, fh: *mut u32, nr_dma_as: u8) -> c_int {
    int clp_enable_fh(struct zpci_dev *zdev, u32 *fh, u8 nr_dma_as)
    {
    int rc;
    rc = clp_set_pci_fn(zdev, fh, nr_dma_as, CLP_SET_ENABLE_PCI_FN);
    zpci_dbg(3, "ena fid:%x, fh:%x, rc:%d\n", zdev.fid, *fh, rc);
    if (!rc && zpci_use_mio(zdev)) {
    rc = clp_set_pci_fn(zdev, fh, nr_dma_as, CLP_SET_ENABLE_MIO);
    zpci_dbg(3, "ena mio fid:%x, fh:%x, rc:%d\n",
    zdev.fid, *fh, rc);
    if (rc)
    clp_disable_fh(zdev, fh);
    }
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn clp_disable_fh(zdev: *mut zpci_dev, fh: *mut u32) -> c_int {
    int clp_disable_fh(struct zpci_dev *zdev, u32 *fh)
    {
    int rc;
    if (!zdev_enabled(zdev))
    return 0;
    rc = clp_set_pci_fn(zdev, fh, 0, CLP_SET_DISABLE_PCI_FN);
    zpci_dbg(3, "dis fid:%x, fh:%x, rc:%d\n", zdev.fid, *fh, rc);
    return rc;
    }
    static int clp_list_pci_req(struct clp_req_rsp_list_pci *rrb,
    u64 *resume_token, int *nentries)
    {
    int rc;
    memset(rrb, 0, sizeof(*rrb));
    rrb.request.hdr.len = sizeof(rrb.request);
    rrb.request.hdr.cmd = CLP_LIST_PCI;
// store as many entries as possible
    rrb.response.hdr.len = CLP_BLK_SIZE - LIST_PCI_HDR_LEN;
    rrb.request.resume_token = *resume_token;
// Get PCI function handle list
    rc = clp_req(rrb, CLP_LPS_PCI);
    if (rc || rrb.response.hdr.rsp != CLP_RC_OK) {
    zpci_err("List PCI FN:\n");
    zpci_err_clp(rrb.response.hdr.rsp, rc);
    return -EIO;
    }
    update_uid_checking(rrb.response.uid_checking);
    WARN_ON_ONCE(rrb.response.entry_size !=
    sizeof(struct clp_fh_list_entry));
// nentries = (rrb->response.hdr.len - LIST_PCI_HDR_LEN)
    rrb.response.entry_size;
// resume_token = rrb->response.resume_token;
    return rc;
    }
    static int clp_list_pci(struct clp_req_rsp_list_pci *rrb, void *data,
    void (*cb)(struct clp_fh_list_entry *, void *))
    {
    let mut resume_token: u64 = 0;
    int nentries, i, rc;
    do {
    rc = clp_list_pci_req(rrb, &resume_token, &nentries);
    if (rc)
    return rc;
    for (i = 0; i < nentries; i++)
    cb(&rrb.response.fh_list[i], data);
    } while (resume_token);
    return rc;
    }
    static int clp_find_pci(struct clp_req_rsp_list_pci *rrb, u32 fid,
    struct clp_fh_list_entry *entry)
    {
    struct clp_fh_list_entry *fh_list;
    let mut resume_token: u64 = 0;
    int nentries, i, rc;
    do {
    rc = clp_list_pci_req(rrb, &resume_token, &nentries);
    if (rc)
    return rc;
    fh_list = rrb.response.fh_list;
    for (i = 0; i < nentries; i++) {
    if (fh_list[i].fid == fid) {
// entry = fh_list[i];
    return 0;
    }
    }
    } while (resume_token);
    return -ENODEV;
    }
#[no_mangle]
unsafe extern "C" fn __clp_add(entry: *mut clp_fh_list_entry, data: *mut c_void) {
    static void __clp_add(struct clp_fh_list_entry *entry, void *data)
    {
    struct list_head *scan_list = data;
    struct zpci_dev *zdev;
    if (!entry.vendor_id)
    return;
    zdev = get_zdev_by_fid(entry.fid);
    if (zdev) {
    zpci_zdev_put(zdev);
    return;
    }
    zdev = zpci_create_device(entry.fid, entry.fh, entry.config_state);
    if (IS_ERR(zdev))
    return;
    list_add_tail(&zdev.entry, scan_list);
    }
#[no_mangle]
pub unsafe extern "C" fn clp_scan_pci_devices(scan_list: *mut list_head) -> c_int {
    int clp_scan_pci_devices(struct list_head *scan_list)
    {
    struct clp_req_rsp_list_pci *rrb;
    int rc;
    rrb = clp_alloc_block(GFP_KERNEL);
    if (!rrb)
    return -ENOMEM;
    rc = clp_list_pci(rrb, scan_list, __clp_add);
    clp_free_block(rrb);
    return rc;
    }
//
// Get the current function handle of the function matching @fid
//
#[no_mangle]
pub unsafe extern "C" fn clp_refresh_fh(fid: u32, fh: *mut u32) -> c_int {
    int clp_refresh_fh(u32 fid, u32 *fh)
    {
    struct clp_req_rsp_list_pci *rrb;
    struct clp_fh_list_entry entry;
    int rc;
    rrb = clp_alloc_block(GFP_NOWAIT);
    if (!rrb)
    return -ENOMEM;
    rc = clp_find_pci(rrb, fid, &entry);
    if (!rc)
// fh = entry.fh;
    clp_free_block(rrb);
    return rc;
    }
#[no_mangle]
pub unsafe extern "C" fn clp_get_state(fid: u32, state: *mut enum zpci_state) -> c_int {
    int clp_get_state(u32 fid, enum zpci_state *state)
    {
    struct clp_req_rsp_list_pci *rrb;
    struct clp_fh_list_entry entry;
    int rc;
    rrb = clp_alloc_block(GFP_ATOMIC);
    if (!rrb)
    return -ENOMEM;
    rc = clp_find_pci(rrb, fid, &entry);
    if (!rc) {
// state = entry.config_state;
    } else if (rc == -ENODEV) {
// state = ZPCI_FN_STATE_RESERVED;
    rc = 0;
    }
    clp_free_block(rrb);
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn clp_base_slpc(req: *mut clp_req, lpcb: *mut clp_req_rsp_slpc) -> c_int {
    static int clp_base_slpc(struct clp_req *req, struct clp_req_rsp_slpc *lpcb)
    {
    let mut limit: c_ulong = PAGE_SIZE - sizeof(lpcb.request);
    if (lpcb.request.hdr.len != sizeof(lpcb.request) ||
    lpcb.response.hdr.len > limit)
    return -EINVAL;
    return clp_req(lpcb, CLP_LPS_BASE) ? -EOPNOTSUPP : 0;
    }
#[no_mangle]
unsafe extern "C" fn clp_base_command(req: *mut clp_req, lpcb: *mut clp_req_hdr) -> c_int {
    static int clp_base_command(struct clp_req *req, struct clp_req_hdr *lpcb)
    {
    switch (lpcb.cmd) {
    case 0x0001: /* store logical-processor characteristics */
    return clp_base_slpc(req, (void *) lpcb);
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn clp_pci_slpc(req: *mut clp_req, lpcb: *mut clp_req_rsp_slpc_pci) -> c_int {
    static int clp_pci_slpc(struct clp_req *req, struct clp_req_rsp_slpc_pci *lpcb)
    {
    let mut limit: c_ulong = PAGE_SIZE - sizeof(lpcb.request);
    if (lpcb.request.hdr.len != sizeof(lpcb.request) ||
    lpcb.response.hdr.len > limit)
    return -EINVAL;
    return clp_req(lpcb, CLP_LPS_PCI) ? -EOPNOTSUPP : 0;
    }
#[no_mangle]
unsafe extern "C" fn clp_pci_list(req: *mut clp_req, lpcb: *mut clp_req_rsp_list_pci) -> c_int {
    static int clp_pci_list(struct clp_req *req, struct clp_req_rsp_list_pci *lpcb)
    {
    let mut limit: c_ulong = PAGE_SIZE - sizeof(lpcb.request);
    if (lpcb.request.hdr.len != sizeof(lpcb.request) ||
    lpcb.response.hdr.len > limit)
    return -EINVAL;
    if (lpcb.request.reserved2 != 0)
    return -EINVAL;
    return clp_req(lpcb, CLP_LPS_PCI) ? -EOPNOTSUPP : 0;
    }
    static int clp_pci_query(struct clp_req *req,
    struct clp_req_rsp_query_pci *lpcb)
    {
    let mut limit: c_ulong = PAGE_SIZE - sizeof(lpcb.request);
    if (lpcb.request.hdr.len != sizeof(lpcb.request) ||
    lpcb.response.hdr.len > limit)
    return -EINVAL;
    if (lpcb.request.reserved2 != 0 || lpcb.request.reserved3 != 0)
    return -EINVAL;
    return clp_req(lpcb, CLP_LPS_PCI) ? -EOPNOTSUPP : 0;
    }
    static int clp_pci_query_grp(struct clp_req *req,
    struct clp_req_rsp_query_pci_grp *lpcb)
    {
    let mut limit: c_ulong = PAGE_SIZE - sizeof(lpcb.request);
    if (lpcb.request.hdr.len != sizeof(lpcb.request) ||
    lpcb.response.hdr.len > limit)
    return -EINVAL;
    if (lpcb.request.reserved2 != 0 || lpcb.request.reserved3 != 0 ||
    lpcb.request.reserved4 != 0)
    return -EINVAL;
    return clp_req(lpcb, CLP_LPS_PCI) ? -EOPNOTSUPP : 0;
    }
#[no_mangle]
unsafe extern "C" fn clp_pci_command(req: *mut clp_req, lpcb: *mut clp_req_hdr) -> c_int {
    static int clp_pci_command(struct clp_req *req, struct clp_req_hdr *lpcb)
    {
    switch (lpcb.cmd) {
    case 0x0001: /* store logical-processor characteristics */
    return clp_pci_slpc(req, (void *) lpcb);
    case 0x0002: /* list PCI functions */
    return clp_pci_list(req, (void *) lpcb);
    case 0x0003: /* query PCI function */
    return clp_pci_query(req, (void *) lpcb);
    case 0x0004: /* query PCI function group */
    return clp_pci_query_grp(req, (void *) lpcb);
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn clp_normal_command(req: *mut clp_req) -> c_int {
    static int clp_normal_command(struct clp_req *req)
    {
    struct clp_req_hdr *lpcb;
    void __user *uptr;
    int rc;
    rc = -EINVAL;
    if (req.lps != 0 && req.lps != 2)
    goto out;
    rc = -ENOMEM;
    lpcb = clp_alloc_block(GFP_KERNEL);
    if (!lpcb)
    goto out;
    rc = -EFAULT;
    uptr = (void  __user *)(unsigned long) req.data_p;
    if (copy_from_user(lpcb, uptr, PAGE_SIZE) != 0)
    goto out_free;
    rc = -EINVAL;
    if (lpcb.fmt != 0 || lpcb.reserved1 != 0 || lpcb.reserved2 != 0)
    goto out_free;
    switch (req.lps) {
    case 0:
    rc = clp_base_command(req, lpcb);
    break;
    case 2:
    rc = clp_pci_command(req, lpcb);
    break;
    }
    if (rc)
    goto out_free;
    rc = -EFAULT;
    if (copy_to_user(uptr, lpcb, PAGE_SIZE) != 0)
    goto out_free;
    rc = 0;
    out_free:
    clp_free_block(lpcb);
    out:
    return rc;
    }
#[no_mangle]
unsafe extern "C" fn clp_immediate_command(req: *mut clp_req) -> c_int {
    static int clp_immediate_command(struct clp_req *req)
    {
    void __user *uptr;
    unsigned long ilp;
    int exists;
    if (req.cmd > 1 || clp_get_ilp(&ilp) != 0)
    return -EINVAL;
    uptr = (void  __user *)(unsigned long) req.data_p;
    if (req.cmd == 0) {
// Command code 0: test for a specific processor
    exists = test_bit_inv(req.lps, &ilp);
    return put_user(exists, (int __user *) uptr);
    }
// Command code 1: return bit mask of installed processors
    return put_user(ilp, (unsigned long __user *) uptr);
    }
    static long clp_misc_ioctl(struct file *filp, unsigned int cmd,
    unsigned long arg)
    {
    struct clp_req req;
    void __user *argp;
    if (cmd != CLP_SYNC)
    return -EINVAL;
    argp = (void __user *)arg;
    if (copy_from_user(&req, argp, sizeof(req)))
    return -EFAULT;
    if (req.r != 0)
    return -EINVAL;
    return req.c ? clp_immediate_command(&req) : clp_normal_command(&req);
    }
#[no_mangle]
unsafe extern "C" fn clp_misc_release(inode: *mut inode, filp: *mut file) -> c_int {
    static int clp_misc_release(struct inode *inode, struct file *filp)
    {
    return 0;
    }
    static const struct file_operations clp_misc_fops = {
    .owner = THIS_MODULE,
    .open = nonseekable_open,
    .release = clp_misc_release,
    .unlocked_ioctl = clp_misc_ioctl,
    };
    static struct miscdevice clp_misc_device = {
    .minor = MISC_DYNAMIC_MINOR,
    .name = "clp",
    .fops = &clp_misc_fops,
    };
    builtin_misc_device(clp_misc_device);
