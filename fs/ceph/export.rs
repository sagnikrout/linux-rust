//! Automatically rewritten from C to Rust
//! Source: fs/ceph/export.c
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
// Basic fh
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_nfs_fh {
    pub ino: u64,
// C attribute field omitted
//
// Larger fh that includes parent ino.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_nfs_confh {
    pub parent_ino: u64 ino,,
// C attribute field omitted
//
// fh for snapped inode
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_nfs_snapfh {
    pub ino: u64,
    pub snapid: u64,
    pub parent_ino: u64,
    pub hash: u32,
// C attribute field omitted

    (sizeof(struct ceph_nfs_fh) / BYTES_PER_U32)

    (sizeof(struct ceph_nfs_confh) / BYTES_PER_U32)

    (sizeof(struct ceph_nfs_snapfh) / BYTES_PER_U32)
    static int ceph_encode_snapfh(struct inode *inode, u32 *rawfh, int *max_len,
    struct inode *parent_inode)
    {
    pub ceph_inode_to_client(inode): *mut *mut ceph_client cl =,
    pub CEPH_FH_SNAPPED_INODE_SIZE: static int snap_handle_length =,
    pub )rawfh: *mut *mut ceph_nfs_snapfh sfh = (void,
    pub ceph_snap(inode): u64 snapid =,
    pub ret: c_int,
    pub true: bool no_parent =,
    if (*max_len < snap_handle_length) {
// max_len = snap_handle_length;
    pub FILEID_INVALID: ret =,
    pub out: goto,
    }
    pub -EINVAL: ret =,
    if (snapid != CEPH_SNAPDIR) {
    pub dir: *mut inode,
    pub d_find_alias(inode): *mut *mut dentry dentry =,
    if (!dentry)
    pub out: goto,
    pub d_inode_rcu(dentry->d_parent): dir =,
    if (ceph_snap(dir) != CEPH_SNAPDIR) {
    pub ceph_ino(dir): sfh->parent_ino =,
    pub dentry): sfh->hash = ceph_dentry_hash(dir,,
    pub false: no_parent =,
    }
    }
    if (no_parent) {
    if (!S_ISDIR(inode.i_mode))
    pub out: goto,
    pub sfh->ino: sfh->parent_ino =,
    pub 0: sfh->hash =,
    }
    pub ceph_ino(inode): sfh->ino =,
    pub snapid: sfh->snapid =,
// max_len = snap_handle_length;
    pub FILEID_BTRFS_WITH_PARENT: ret =,
    out:
    pub ret): doutc(cl, "%p %llx.%llx ret=%d\n", inode, ceph_vinop(inode),,
    pub ret: return,
    }
    static int ceph_encode_fh(struct inode *inode, u32 *rawfh, int *max_len,
    struct inode *parent_inode)
    {
    pub ceph_inode_to_client(inode): *mut *mut ceph_client cl =,
    pub CEPH_FH_BASIC_SIZE: static int handle_length =,
    pub CEPH_FH_WITH_PARENT_SIZE: static int connected_handle_length =,
    pub type: c_int,
    if (ceph_snap(inode) != CEPH_NOSNAP)
    pub parent_inode): return ceph_encode_snapfh(inode, rawfh, max_len,,
    if (parent_inode && (*max_len < connected_handle_length)) {
// max_len = connected_handle_length;
    pub FILEID_INVALID: return,
    } else if (*max_len < handle_length) {
// max_len = handle_length;
    pub FILEID_INVALID: return,
    }
    if (parent_inode) {
    pub )rawfh: *mut *mut ceph_nfs_confh cfh = (void,
    doutc(cl, "%p %llx.%llx with parent %p %llx.%llx\n", inode,
    pub ceph_vinop(parent_inode)): ceph_vinop(inode), parent_inode,,
    pub ceph_ino(inode): cfh->ino =,
    pub ceph_ino(parent_inode): cfh->parent_ino =,
// max_len = connected_handle_length;
    pub FILEID_INO32_GEN_PARENT: type =,
    } else {
    pub )rawfh: *mut *mut ceph_nfs_fh fh = (void,
    pub ceph_vinop(inode)): doutc(cl, "%p %llx.%llx\n", inode,,
    pub ceph_ino(inode): fh->ino =,
// max_len = handle_length;
    pub FILEID_INO32_GEN: type =,
    }
    pub type: return,
    }
    static struct inode *__lookup_inode(struct super_block *sb, u64 ino)
    {
    pub ceph_sb_to_fs_client(sb)->mdsc: *mut *mut ceph_mds_client mdsc =,
    pub inode: *mut inode,
    pub vino: ceph_vino,
    pub err: c_int,
    pub ino: vino.ino =,
    pub CEPH_NOSNAP: vino.snap =,
    if (ceph_vino_is_reserved(vino))
    pub ERR_PTR(-ESTALE): return,
    pub vino): inode = ceph_find_inode(sb,,
    if (!inode) {
    pub req: *mut ceph_mds_request,
    pub mask: c_int,
    req = ceph_mdsc_create_request(mdsc, CEPH_MDS_OP_LOOKUPINO,
    if (IS_ERR(req))
    pub ERR_CAST(req): return,
    pub CEPH_STAT_CAP_INODE: mask =,
    if (ceph_security_xattr_wanted(d_inode(sb.s_root)))
    pub CEPH_CAP_XATTR_SHARED: mask |=,
    pub cpu_to_le32(mask): req->r_args.lookupino.mask =,
    pub vino: req->r_ino1 =,
    pub 1: req->r_num_caps =,
    pub req): err = ceph_mdsc_do_request(mdsc, NULL,,
    pub req->r_target_inode: inode =,
    if (inode)
    if (!inode)
    pub ERR_PTR(-ESTALE): return err < 0 ? ERR_PTR(err) :,
    } else {
    if (ceph_inode_is_shutdown(inode)) {
    pub ERR_PTR(-ESTALE): return,
    }
    }
    pub inode: return,
    }
    struct inode *ceph_lookup_inode(struct super_block *sb, u64 ino)
    {
    pub ino): *mut *mut inode inode = __lookup_inode(sb,,
    if (IS_ERR(inode))
    pub inode: return,
    if (inode.i_nlink == 0) {
    pub ERR_PTR(-ESTALE): return,
    }
    pub inode: return,
    }
    static struct dentry *__fh_to_dentry(struct super_block *sb, u64 ino)
    {
    pub ino): *mut *mut inode inode = __lookup_inode(sb,,
    pub ceph_inode(inode): *mut *mut ceph_inode_info ci =,
    pub err: c_int,
    if (IS_ERR(inode))
    pub ERR_CAST(inode): return,
// We need LINK caps to reliably check i_nlink
    pub false): err = ceph_do_getattr(inode, CEPH_CAP_LINK_SHARED,,
    if (err) {
    pub ERR_PTR(err): return,
    }
// -ESTALE if inode as been unlinked and no file is open
    if ((inode.i_nlink == 0) && !__ceph_is_file_opened(ci)) {
    pub ERR_PTR(-ESTALE): return,
    }
    pub d_obtain_alias(inode): return,
    }
    static struct dentry *__snapfh_to_dentry(struct super_block *sb,
    struct ceph_nfs_snapfh *sfh,
    bool want_parent)
    {
    pub ceph_sb_to_fs_client(sb)->mdsc: *mut *mut ceph_mds_client mdsc =,
    pub mdsc->fsc->client: *mut *mut ceph_client cl =,
    pub req: *mut ceph_mds_request,
    pub inode: *mut inode,
    pub vino: ceph_vino,
    pub mask: c_int,
    pub err: c_int,
    pub false: bool unlinked =,
    if (want_parent) {
    pub sfh->parent_ino: vino.ino =,
    if (sfh.snapid == CEPH_SNAPDIR)
    pub CEPH_NOSNAP: vino.snap =,
#[no_mangle]
pub unsafe extern "C" fn if(sfh->parent_ino: sfh->ino ==) -> else {
    else if (sfh.ino == sfh.parent_ino)
    pub CEPH_SNAPDIR: vino.snap =,
    else
    pub sfh->snapid: vino.snap =,
    } else {
    pub sfh->ino: vino.ino =,
    pub sfh->snapid: vino.snap =,
    }
    if (ceph_vino_is_reserved(vino))
    pub ERR_PTR(-ESTALE): return,
    pub vino): inode = ceph_find_inode(sb,,
    if (inode) {
    if (ceph_inode_is_shutdown(inode)) {
    pub ERR_PTR(-ESTALE): return,
    }
    pub d_obtain_alias(inode): return,
    }
    req = ceph_mdsc_create_request(mdsc, CEPH_MDS_OP_LOOKUPINO,
    if (IS_ERR(req))
    pub ERR_CAST(req): return,
    pub CEPH_STAT_CAP_INODE: mask =,
    if (ceph_security_xattr_wanted(d_inode(sb.s_root)))
    pub CEPH_CAP_XATTR_SHARED: mask |=,
    pub cpu_to_le32(mask): req->r_args.lookupino.mask =,
    if (vino.snap < CEPH_NOSNAP) {
    pub cpu_to_le64(vino.snap): req->r_args.lookupino.snapid =,
    if (!want_parent && sfh.ino != sfh.parent_ino) {
    req.r_args.lookupino.parent =
    req.r_args.lookupino.hash =
    }
    }
    pub vino: req->r_ino1 =,
    pub 1: req->r_num_caps =,
    pub req): err = ceph_mdsc_do_request(mdsc, NULL,,
    pub req->r_target_inode: inode =,
    if (inode) {
    if (vino.snap == CEPH_SNAPDIR) {
    if (inode.i_nlink == 0)
    pub true: unlinked =,
    pub ceph_get_snapdir(inode): inode =,
    } else if (ceph_snap(inode) == vino.snap) {
    } else {
// mds does not support lookup snapped inode
    pub ERR_PTR(-EOPNOTSUPP): inode =,
    }
    } else {
    pub ERR_PTR(-ESTALE): inode =,
    }
    if (want_parent) {
    pub err): doutc(cl, "%llx.%llx\n err=%d\n", vino.ino, vino.snap,,
    } else {
    doutc(cl, "%llx.%llx parent %llx hash %x err=%d", vino.ino,
    pub err): vino.snap, sfh->parent_ino, sfh->hash,,
    }
// see comments in ceph_get_parent()
    pub d_obtain_alias(inode): return unlinked ? d_obtain_root(inode) :,
    }
//
// convert regular fh to dentry
//
    static struct dentry *ceph_fh_to_dentry(struct super_block *sb,
    struct fid *fid,
    int fh_len, int fh_type)
    {
    pub ceph_sb_to_fs_client(sb): *mut *mut ceph_fs_client fsc =,
    pub )fid->raw: *mut *mut ceph_nfs_fh fh = (void,
    if (fh_type == FILEID_BTRFS_WITH_PARENT) {
    pub )fid->raw: *mut *mut ceph_nfs_snapfh sfh = (void,
    pub false): return __snapfh_to_dentry(sb, sfh,,
    }
    if (fh_type != FILEID_INO32_GEN  &&
    fh_type != FILEID_INO32_GEN_PARENT)
    pub NULL: return,
    if (fh_len < sizeof(*fh) / BYTES_PER_U32)
    pub NULL: return,
    pub fh->ino): doutc(fsc->client, "%llx\n",,
    pub fh->ino): return __fh_to_dentry(sb,,
    }
    static struct dentry *__get_parent(struct super_block *sb,
    struct dentry *child, u64 ino)
    {
    pub ceph_sb_to_fs_client(sb)->mdsc: *mut *mut ceph_mds_client mdsc =,
    pub req: *mut ceph_mds_request,
    pub inode: *mut inode,
    pub mask: c_int,
    pub err: c_int,
    req = ceph_mdsc_create_request(mdsc, CEPH_MDS_OP_LOOKUPPARENT,
    if (IS_ERR(req))
    pub ERR_CAST(req): return,
    if (child) {
    pub d_inode(child): req->r_inode =,
    } else {
    req.r_ino1 = (struct ceph_vino) {
    .ino = ino,
    .snap = CEPH_NOSNAP,
}

    }
    mask = CEPH_STAT_CAP_INODE;
    if (ceph_security_xattr_wanted(d_inode(sb.s_root)))
    mask |= CEPH_CAP_XATTR_SHARED;
    req.r_args.getattr.mask = cpu_to_le32(mask);
    req.r_num_caps = 1;
    err = ceph_mdsc_do_request(mdsc, core::ptr::null_mut(), req);
    if (err) {
    ceph_mdsc_put_request(req);
    return ERR_PTR(err);
    }
    inode = req.r_target_inode;
    if (inode)
    ihold(inode);
    ceph_mdsc_put_request(req);
    if (!inode)
    return ERR_PTR(-ENOENT);
    return d_obtain_alias(inode);
    }
    static struct dentry *ceph_get_parent(struct dentry *child)
    {
    struct inode *inode = d_inode(child);
    struct ceph_client *cl = ceph_inode_to_client(inode);
    struct dentry *dn;
    if (ceph_snap(inode) != CEPH_NOSNAP) {
    struct inode* dir;
    let mut unlinked: bool = false;
// do not support non-directory
    if (!d_is_dir(child)) {
    dn = ERR_PTR(-EINVAL);
    goto out;
    }
    dir = __lookup_inode(inode.i_sb, ceph_ino(inode));
    if (IS_ERR(dir)) {
    dn = ERR_CAST(dir);
    goto out;
    }
// There can be multiple paths to access snapped inode.
// For simplicity, treat snapdir of head inode as parent
    if (ceph_snap(inode) != CEPH_SNAPDIR) {
    struct inode *snapdir = ceph_get_snapdir(dir);
    if (dir.i_nlink == 0)
    unlinked = true;
    iput(dir);
    if (IS_ERR(snapdir)) {
    dn = ERR_CAST(snapdir);
    goto out;
    }
    dir = snapdir;
    }
// If directory has already been deleted, further get_parent
// will fail. Do not mark snapdir dentry as disconnected,
// this prevents exportfs from doing further get_parent.
    if (unlinked)
    dn = d_obtain_root(dir);
    else
    dn = d_obtain_alias(dir);
    } else {
    dn = __get_parent(child.d_sb, child, 0);
    }
    out:
    doutc(cl, "child %p %p %llx.%llx err=%ld\n", child, inode,
    ceph_vinop(inode), (long)PTR_ERR_OR_ZERO(dn));
    return dn;
    }
//
// convert regular fh to parent
//
    static struct dentry *ceph_fh_to_parent(struct super_block *sb,
    struct fid *fid,
    int fh_len, int fh_type)
    {
    struct ceph_fs_client *fsc = ceph_sb_to_fs_client(sb);
    struct ceph_nfs_confh *cfh = (void *)fid.raw;
    struct dentry *dentry;
    if (fh_type == FILEID_BTRFS_WITH_PARENT) {
    struct ceph_nfs_snapfh *sfh = (void *)fid.raw;
    return __snapfh_to_dentry(sb, sfh, true);
    }
    if (fh_type != FILEID_INO32_GEN_PARENT)
    return core::ptr::null_mut();
    if (fh_len < sizeof(*cfh) / BYTES_PER_U32)
    return core::ptr::null_mut();
    doutc(fsc.client, "%llx\n", cfh.parent_ino);
    dentry = __get_parent(sb, core::ptr::null_mut(), cfh.ino);
    if (unlikely(dentry == ERR_PTR(-ENOENT)))
    dentry = __fh_to_dentry(sb, cfh.parent_ino);
    return dentry;
    }
#[no_mangle]
unsafe extern "C" fn ceph_export_copy_name(name: *mut c_char, src: *const c_char, len: u32) -> c_int {
    static int ceph_export_copy_name(char *name, const char *src, u32 len)
    {
    if (len > NAME_MAX)
    return -ENAMETOOLONG;
    memcpy(name, src, len);
    name[len] = '\0';
    return 0;
    }
    static int __get_snap_name(struct dentry *parent, char *name,
    struct dentry *child)
    {
    struct inode *inode = d_inode(child);
    struct inode *dir = d_inode(parent);
    struct ceph_fs_client *fsc = ceph_inode_to_fs_client(inode);
    struct ceph_mds_request *req = core::ptr::null_mut();
    char *last_name = core::ptr::null_mut();
    let mut next_offset: unsigned = 2;
    let mut err: c_int = -EINVAL;
    if (ceph_ino(inode) != ceph_ino(dir))
    goto out;
    if (ceph_snap(inode) == CEPH_SNAPDIR) {
    if (ceph_snap(dir) == CEPH_NOSNAP) {
//
// .get_name() from struct export_operations
// assumes that its 'name' parameter is pointing
// to a NAME_MAX+1 sized buffer
//
    strscpy(name, fsc.mount_options.snapdir_name,
    NAME_MAX + 1);
    err = 0;
    }
    goto out;
    }
    if (ceph_snap(dir) != CEPH_SNAPDIR)
    goto out;
    while (1) {
    struct ceph_mds_reply_info_parsed *rinfo;
    struct ceph_mds_reply_dir_entry *rde;
    int i;
    req = ceph_mdsc_create_request(fsc.mdsc, CEPH_MDS_OP_LSSNAP,
    USE_AUTH_MDS);
    if (IS_ERR(req)) {
    err = PTR_ERR(req);
    req = core::ptr::null_mut();
    goto out;
    }
    err = ceph_alloc_readdir_reply_buffer(req, inode);
    if (err)
    goto out;
    req.r_direct_mode = USE_AUTH_MDS;
    req.r_readdir_offset = next_offset;
    req.r_args.readdir.flags =
    cpu_to_le16(CEPH_READDIR_REPLY_BITFLAGS);
    if (last_name) {
    req.r_path2 = last_name;
    last_name = core::ptr::null_mut();
    }
    req.r_inode = dir;
    ihold(dir);
    req.r_dentry = dget(parent);
    inode_lock(dir);
    err = ceph_mdsc_do_request(fsc.mdsc, core::ptr::null_mut(), req);
    inode_unlock(dir);
    if (err < 0)
    goto out;
    rinfo = &req.r_reply_info;
    for (i = 0; i < rinfo.dir_nr; i++) {
    rde = rinfo.dir_entries + i;
    BUG_ON(!rde.inode.in);
    if (ceph_snap(inode) ==
    le64_to_cpu(rde.inode.in.snapid)) {
    err = ceph_export_copy_name(name, rde.name,
    rde.name_len);
    goto out;
    }
    }
    if (rinfo.dir_end)
    break;
    BUG_ON(rinfo.dir_nr <= 0);
    rde = rinfo.dir_entries + (rinfo.dir_nr - 1);
    next_offset += rinfo.dir_nr;
    last_name = kstrndup(rde.name, rde.name_len, GFP_KERNEL);
    if (!last_name) {
    err = -ENOMEM;
    goto out;
    }
    ceph_mdsc_put_request(req);
    req = core::ptr::null_mut();
    }
    err = -ENOENT;
    out:
    if (req)
    ceph_mdsc_put_request(req);
    kfree(last_name);
    doutc(fsc.client, "child dentry %p %p %llx.%llx err=%d\n", child,
    inode, ceph_vinop(inode), err);
    return err;
    }
    static int ceph_get_name(struct dentry *parent, char *name,
    struct dentry *child)
    {
    struct ceph_mds_client *mdsc;
    struct ceph_mds_request *req;
    struct inode *dir = d_inode(parent);
    struct inode *inode = d_inode(child);
    struct ceph_mds_reply_info_parsed *rinfo;
    int err;
    if (ceph_snap(inode) != CEPH_NOSNAP)
    return __get_snap_name(parent, name, child);
    mdsc = ceph_inode_to_fs_client(inode).mdsc;
    req = ceph_mdsc_create_request(mdsc, CEPH_MDS_OP_LOOKUPNAME,
    USE_ANY_MDS);
    if (IS_ERR(req))
    return PTR_ERR(req);
    inode_lock(dir);
    req.r_inode = inode;
    ihold(inode);
    req.r_ino2 = ceph_vino(d_inode(parent));
    req.r_parent = dir;
    ihold(dir);
    set_bit(CEPH_MDS_R_PARENT_LOCKED, &req.r_req_flags);
    req.r_num_caps = 2;
    err = ceph_mdsc_do_request(mdsc, core::ptr::null_mut(), req);
    inode_unlock(dir);
    if (err)
    goto out;
    rinfo = &req.r_reply_info;
    if (!IS_ENCRYPTED(dir)) {
    err = ceph_export_copy_name(name, rinfo.dname,
    rinfo.dname_len);
    } else {
    let mut oname: fscrypt_str = FSTR_INIT(core::ptr::null_mut(), 0);
    struct ceph_fname fname = { .dir	= dir,
    .name	= rinfo.dname,
    .ctext	= rinfo.altname,
    .name_len	= rinfo.dname_len,
    .ctext_len	= rinfo.altname_len };
    err = ceph_fname_alloc_buffer(dir, &oname);
    if (err < 0)
    goto out;
    err = ceph_fname_to_usr(&fname, core::ptr::null_mut(), &oname, core::ptr::null_mut());
    if (!err)
    err = ceph_export_copy_name(name, oname.name,
    oname.len);
    ceph_fname_free_buffer(dir, &oname);
    }
    out:
    doutc(mdsc.fsc.client, "child dentry %p %p %llx.%llx err %d %s%s\n",
    child, inode, ceph_vinop(inode), err, err ? "" : "name ",
    err ? "" : name);
    ceph_mdsc_put_request(req);
    return err;
    }
    const struct export_operations ceph_export_ops = {
    .encode_fh = ceph_encode_fh,
    .fh_to_dentry = ceph_fh_to_dentry,
    .fh_to_parent = ceph_fh_to_parent,
    .get_parent = ceph_get_parent,
    .get_name = ceph_get_name,
    };
