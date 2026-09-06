//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/tegra/bpmp-debugfs.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2017, NVIDIA CORPORATION.  All rights reserved.
//

    static DEFINE_MUTEX(bpmp_debug_lock);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct seqbuf {
    pub buf: *mut c_char,
    pub pos: usize,
    pub size: usize,
}

#[no_mangle]
unsafe extern "C" fn seqbuf_init(seqbuf: *mut seqbuf, buf: *mut c_void, size: usize) {
    static void seqbuf_init(struct seqbuf *seqbuf, void *buf, size_t size)
    {
    seqbuf.buf = buf;
    seqbuf.size = size;
    seqbuf.pos = 0;
    }
#[no_mangle]
unsafe extern "C" fn seqbuf_avail(seqbuf: *mut seqbuf) -> usize {
    static size_t seqbuf_avail(struct seqbuf *seqbuf)
    {
    return seqbuf.pos < seqbuf.size ? seqbuf.size - seqbuf.pos : 0;
    }
#[no_mangle]
unsafe extern "C" fn seqbuf_status(seqbuf: *mut seqbuf) -> usize {
    static size_t seqbuf_status(struct seqbuf *seqbuf)
    {
    return seqbuf.pos <= seqbuf.size ? 0 : -EOVERFLOW;
    }
#[no_mangle]
unsafe extern "C" fn seqbuf_eof(seqbuf: *mut seqbuf) -> c_int {
    static int seqbuf_eof(struct seqbuf *seqbuf)
    {
    return seqbuf.pos >= seqbuf.size;
    }
#[no_mangle]
unsafe extern "C" fn seqbuf_read(seqbuf: *mut seqbuf, buf: *mut c_void, nbyte: usize) -> c_int {
    static int seqbuf_read(struct seqbuf *seqbuf, void *buf, size_t nbyte)
    {
    nbyte = min(nbyte, seqbuf_avail(seqbuf));
    memcpy(buf, seqbuf.buf + seqbuf.pos, nbyte);
    seqbuf.pos += nbyte;
    return seqbuf_status(seqbuf);
    }
#[no_mangle]
unsafe extern "C" fn seqbuf_read_u32(seqbuf: *mut seqbuf, v: *mut u32) -> c_int {
    static int seqbuf_read_u32(struct seqbuf *seqbuf, u32 *v)
    {
    return seqbuf_read(seqbuf, v, 4);
    }
#[no_mangle]
unsafe extern "C" fn seqbuf_read_str(seqbuf: *mut seqbuf, str: *const c_char) -> c_int {
    static int seqbuf_read_str(struct seqbuf *seqbuf, const char **str)
    {
// str = seqbuf->buf + seqbuf->pos;
    seqbuf.pos += strnlen(*str, seqbuf_avail(seqbuf));
    seqbuf.pos++;
    return seqbuf_status(seqbuf);
    }
#[no_mangle]
unsafe extern "C" fn seqbuf_seek(seqbuf: *mut seqbuf, offset: isize) {
    static void seqbuf_seek(struct seqbuf *seqbuf, ssize_t offset)
    {
    seqbuf.pos += offset;
    }
// map filename in Linux debugfs to corresponding entry in BPMP
    static const char *get_filename(struct tegra_bpmp *bpmp,
    const struct file *file, char *buf, int size)
    {
    const char *root_path, *filename = core::ptr::null_mut();
    char *root_path_buf;
    size_t root_len;
    let mut root_path_buf_len: usize = 512;
    root_path_buf = kzalloc(root_path_buf_len, GFP_KERNEL);
    if (!root_path_buf)
    return core::ptr::null_mut();
    root_path = dentry_path(bpmp.debugfs_mirror, root_path_buf,
    root_path_buf_len);
    if (IS_ERR(root_path))
    goto out;
    root_len = strlen(root_path);
    filename = dentry_path(file.f_path.dentry, buf, size);
    if (IS_ERR(filename)) {
    filename = core::ptr::null_mut();
    goto out;
    }
    if (strlen(filename) < root_len || strncmp(filename, root_path, root_len)) {
    filename = core::ptr::null_mut();
    goto out;
    }
    filename += root_len;
    out:
    kfree(root_path_buf);
    return filename;
    }
    static int mrq_debug_open(struct tegra_bpmp *bpmp, const char *name,
    u32 *fd, u32 *len, bool write)
    {
    struct mrq_debug_request req = {
    .cmd = write ? CMD_DEBUG_OPEN_WO : CMD_DEBUG_OPEN_RO,
    };
    struct mrq_debug_response resp;
    struct tegra_bpmp_message msg = {
    .mrq = MRQ_DEBUG,
    .tx = {
    .data = &req,
    .size = sizeof(req),
    },
    .rx = {
    .data = &resp,
    .size = sizeof(resp),
    },
    };
    ssize_t sz_name;
    let mut err: c_int = 0;
    sz_name = strscpy(req.fop.name, name, sizeof(req.fop.name));
    if (sz_name < 0) {
    pr_err("File name too large: %s\n", name);
    return -EINVAL;
    }
    err = tegra_bpmp_transfer(bpmp, &msg);
    if (err < 0)
    return err;
#[no_mangle]
pub unsafe extern "C" fn if(0: msg.rx.ret <) -> else {
    else if (msg.rx.ret < 0)
    return -EINVAL;
// len = resp.fop.datalen;
// fd = resp.fop.fd;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mrq_debug_close(bpmp: *mut tegra_bpmp, fd: u32) -> c_int {
    static int mrq_debug_close(struct tegra_bpmp *bpmp, u32 fd)
    {
    struct mrq_debug_request req = {
    .cmd = CMD_DEBUG_CLOSE,
    .frd = {
    .fd = fd,
    },
    };
    struct mrq_debug_response resp;
    struct tegra_bpmp_message msg = {
    .mrq = MRQ_DEBUG,
    .tx = {
    .data = &req,
    .size = sizeof(req),
    },
    .rx = {
    .data = &resp,
    .size = sizeof(resp),
    },
    };
    let mut err: c_int = 0;
    err = tegra_bpmp_transfer(bpmp, &msg);
    if (err < 0)
    return err;
#[no_mangle]
pub unsafe extern "C" fn if(0: msg.rx.ret <) -> else {
    else if (msg.rx.ret < 0)
    return -EINVAL;
    return 0;
    }
    static int mrq_debug_read(struct tegra_bpmp *bpmp, const char *name,
    char *data, size_t sz_data, u32 *nbytes)
    {
    struct mrq_debug_request req = {
    .cmd = CMD_DEBUG_READ,
    };
    struct mrq_debug_response resp;
    struct tegra_bpmp_message msg = {
    .mrq = MRQ_DEBUG,
    .tx = {
    .data = &req,
    .size = sizeof(req),
    },
    .rx = {
    .data = &resp,
    .size = sizeof(resp),
    },
    };
    let mut fd: u32 = 0, len = 0;
    int remaining, err, close_err;
    mutex_lock(&bpmp_debug_lock);
    err = mrq_debug_open(bpmp, name, &fd, &len, 0);
    if (err)
    goto out;
    if (len > sz_data) {
    err = -EFBIG;
    goto close;
    }
    req.frd.fd = fd;
    remaining = len;
    while (remaining > 0) {
    err = tegra_bpmp_transfer(bpmp, &msg);
    if (err < 0) {
    goto close;
    } else if (msg.rx.ret < 0) {
    err = -EINVAL;
    goto close;
    }
    if (resp.frd.readlen > remaining) {
    pr_err("%s: read data length invalid\n", __func__);
    err = -EINVAL;
    goto close;
    }
    memcpy(data, resp.frd.data, resp.frd.readlen);
    data += resp.frd.readlen;
    remaining -= resp.frd.readlen;
    }
// nbytes = len;
    close:
    close_err = mrq_debug_close(bpmp, fd);
    if (!err)
    err = close_err;
    out:
    mutex_unlock(&bpmp_debug_lock);
    return err;
    }
    static int mrq_debug_write(struct tegra_bpmp *bpmp, const char *name,
    uint8_t *data, size_t sz_data)
    {
    struct mrq_debug_request req = {
    .cmd = CMD_DEBUG_WRITE
    };
    struct mrq_debug_response resp;
    struct tegra_bpmp_message msg = {
    .mrq = MRQ_DEBUG,
    .tx = {
    .data = &req,
    .size = sizeof(req),
    },
    .rx = {
    .data = &resp,
    .size = sizeof(resp),
    },
    };
    let mut fd: u32 = 0, len = 0;
    size_t remaining;
    int err;
    mutex_lock(&bpmp_debug_lock);
    err = mrq_debug_open(bpmp, name, &fd, &len, 1);
    if (err)
    goto out;
    if (sz_data > len) {
    err = -EINVAL;
    goto close;
    }
    req.fwr.fd = fd;
    remaining = sz_data;
    while (remaining > 0) {
    len = min(remaining, sizeof(req.fwr.data));
    memcpy(req.fwr.data, data, len);
    req.fwr.datalen = len;
    err = tegra_bpmp_transfer(bpmp, &msg);
    if (err < 0) {
    goto close;
    } else if (msg.rx.ret < 0) {
    err = -EINVAL;
    goto close;
    }
    data += req.fwr.datalen;
    remaining -= req.fwr.datalen;
    }
    close:
    err = mrq_debug_close(bpmp, fd);
    out:
    mutex_unlock(&bpmp_debug_lock);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn bpmp_debug_show(m: *mut seq_file, p: *mut c_void) -> c_int {
    static int bpmp_debug_show(struct seq_file *m, void *p)
    {
    struct file *file = m.private;
    struct inode *inode = file_inode(file);
    struct tegra_bpmp *bpmp = inode.i_private;
    char fnamebuf[256];
    const char *filename;
    struct mrq_debug_request req = {
    .cmd = CMD_DEBUG_READ,
    };
    struct mrq_debug_response resp;
    struct tegra_bpmp_message msg = {
    .mrq = MRQ_DEBUG,
    .tx = {
    .data = &req,
    .size = sizeof(req),
    },
    .rx = {
    .data = &resp,
    .size = sizeof(resp),
    },
    };
    let mut fd: u32 = 0, len = 0;
    int remaining, err, close_err;
    filename = get_filename(bpmp, file, fnamebuf, sizeof(fnamebuf));
    if (!filename)
    return -ENOENT;
    mutex_lock(&bpmp_debug_lock);
    err = mrq_debug_open(bpmp, filename, &fd, &len, 0);
    if (err)
    goto out;
    req.frd.fd = fd;
    remaining = len;
    while (remaining > 0) {
    err = tegra_bpmp_transfer(bpmp, &msg);
    if (err < 0) {
    goto close;
    } else if (msg.rx.ret < 0) {
    err = -EINVAL;
    goto close;
    }
    if (resp.frd.readlen > remaining) {
    pr_err("%s: read data length invalid\n", __func__);
    err = -EINVAL;
    goto close;
    }
    seq_write(m, resp.frd.data, resp.frd.readlen);
    remaining -= resp.frd.readlen;
    }
    close:
    close_err = mrq_debug_close(bpmp, fd);
    if (!err)
    err = close_err;
    out:
    mutex_unlock(&bpmp_debug_lock);
    return err;
    }
    static ssize_t bpmp_debug_store(struct file *file, const char __user *buf,
    size_t count, loff_t *f_pos)
    {
    struct inode *inode = file_inode(file);
    struct tegra_bpmp *bpmp = inode.i_private;
    char *databuf = core::ptr::null_mut();
    char fnamebuf[256];
    const char *filename;
    ssize_t err;
    filename = get_filename(bpmp, file, fnamebuf, sizeof(fnamebuf));
    if (!filename)
    return -ENOENT;
    databuf = memdup_user(buf, count);
    if (IS_ERR(databuf))
    return PTR_ERR(databuf);
    err = mrq_debug_write(bpmp, filename, databuf, count);
    kfree(databuf);
    return err ?: count;
    }
#[no_mangle]
unsafe extern "C" fn bpmp_debug_open(inode: *mut inode, file: *mut file) -> c_int {
    static int bpmp_debug_open(struct inode *inode, struct file *file)
    {
    return single_open_size(file, bpmp_debug_show, file, SZ_256K);
    }
    static const struct file_operations bpmp_debug_fops = {
    .open		= bpmp_debug_open,
    .read		= seq_read,
    .llseek		= seq_lseek,
    .write		= bpmp_debug_store,
    .release	= single_release,
    };
    static int bpmp_populate_debugfs_inband(struct tegra_bpmp *bpmp,
    struct dentry *parent,
    char *ppath)
    {
    let mut pathlen: usize = SZ_256;
    let mut bufsize: usize = SZ_16K;
    struct dentry *dentry;
    u32 dsize, attrs = 0;
    struct seqbuf seqbuf;
    char *buf, *pathbuf;
    const char *name;
    let mut err: c_int = 0;
    if (!bpmp || !parent || !ppath)
    return -EINVAL;
    buf = kmalloc(bufsize, GFP_KERNEL);
    if (!buf)
    return -ENOMEM;
    pathbuf = kzalloc(pathlen, GFP_KERNEL);
    if (!pathbuf) {
    kfree(buf);
    return -ENOMEM;
    }
    err = mrq_debug_read(bpmp, ppath, buf, bufsize, &dsize);
    if (err)
    goto out;
    seqbuf_init(&seqbuf, buf, dsize);
    while (!seqbuf_eof(&seqbuf)) {
    err = seqbuf_read_u32(&seqbuf, &attrs);
    if (err)
    goto out;
    err = seqbuf_read_str(&seqbuf, &name);
    if (err < 0)
    goto out;
    if (attrs & DEBUGFS_S_ISDIR) {
    size_t len;
    dentry = debugfs_create_dir(name, parent);
    if (IS_ERR(dentry)) {
    err = PTR_ERR(dentry);
    goto out;
    }
    len = snprintf(pathbuf, pathlen, "%s%s/", ppath, name);
    if (len >= pathlen) {
    err = -EINVAL;
    goto out;
    }
    err = bpmp_populate_debugfs_inband(bpmp, dentry,
    pathbuf);
    if (err < 0)
    goto out;
    } else {
    umode_t mode;
    mode = attrs & DEBUGFS_S_IRUSR ? 0400 : 0;
    mode |= attrs & DEBUGFS_S_IWUSR ? 0200 : 0;
    dentry = debugfs_create_file(name, mode, parent, bpmp,
    &bpmp_debug_fops);
    if (IS_ERR(dentry)) {
    err = PTR_ERR(dentry);
    goto out;
    }
    }
    }
    out:
    kfree(pathbuf);
    kfree(buf);
    return err;
    }
    static int mrq_debugfs_read(struct tegra_bpmp *bpmp,
    dma_addr_t name, size_t sz_name,
    dma_addr_t data, size_t sz_data,
    size_t *nbytes)
    {
    struct mrq_debugfs_request req = {
    .cmd = CMD_DEBUGFS_READ,
    .fop = {
    .fnameaddr = (u32)name,
    .fnamelen = (u32)sz_name,
    .dataaddr = (u32)data,
    .datalen = (u32)sz_data,
    },
    };
    struct mrq_debugfs_response resp;
    struct tegra_bpmp_message msg = {
    .mrq = MRQ_DEBUGFS,
    .tx = {
    .data = &req,
    .size = sizeof(req),
    },
    .rx = {
    .data = &resp,
    .size = sizeof(resp),
    },
    };
    int err;
    err = tegra_bpmp_transfer(bpmp, &msg);
    if (err < 0)
    return err;
#[no_mangle]
pub unsafe extern "C" fn if(0: msg.rx.ret <) -> else {
    else if (msg.rx.ret < 0)
    return -EINVAL;
// nbytes = (size_t)resp.fop.nbytes;
    return 0;
    }
    static int mrq_debugfs_write(struct tegra_bpmp *bpmp,
    dma_addr_t name, size_t sz_name,
    dma_addr_t data, size_t sz_data)
    {
    const struct mrq_debugfs_request req = {
    .cmd = CMD_DEBUGFS_WRITE,
    .fop = {
    .fnameaddr = (u32)name,
    .fnamelen = (u32)sz_name,
    .dataaddr = (u32)data,
    .datalen = (u32)sz_data,
    },
    };
    struct tegra_bpmp_message msg = {
    .mrq = MRQ_DEBUGFS,
    .tx = {
    .data = &req,
    .size = sizeof(req),
    },
    };
    return tegra_bpmp_transfer(bpmp, &msg);
    }
    static int mrq_debugfs_dumpdir(struct tegra_bpmp *bpmp, dma_addr_t addr,
    size_t size, size_t *nbytes)
    {
    const struct mrq_debugfs_request req = {
    .cmd = CMD_DEBUGFS_DUMPDIR,
    .dumpdir = {
    .dataaddr = (u32)addr,
    .datalen = (u32)size,
    },
    };
    struct mrq_debugfs_response resp;
    struct tegra_bpmp_message msg = {
    .mrq = MRQ_DEBUGFS,
    .tx = {
    .data = &req,
    .size = sizeof(req),
    },
    .rx = {
    .data = &resp,
    .size = sizeof(resp),
    },
    };
    int err;
    err = tegra_bpmp_transfer(bpmp, &msg);
    if (err < 0)
    return err;
#[no_mangle]
pub unsafe extern "C" fn if(0: msg.rx.ret <) -> else {
    else if (msg.rx.ret < 0)
    return -EINVAL;
// nbytes = (size_t)resp.dumpdir.nbytes;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn debugfs_show(m: *mut seq_file, p: *mut c_void) -> c_int {
    static int debugfs_show(struct seq_file *m, void *p)
    {
    struct file *file = m.private;
    struct inode *inode = file_inode(file);
    struct tegra_bpmp *bpmp = inode.i_private;
    let mut datasize: usize = m.size;
    let mut namesize: usize = SZ_256;
    void *datavirt, *namevirt;
    dma_addr_t dataphys, namephys;
    char buf[256];
    const char *filename;
    size_t len, nbytes;
    int err;
    filename = get_filename(bpmp, file, buf, sizeof(buf));
    if (!filename)
    return -ENOENT;
    namevirt = dma_alloc_coherent(bpmp.dev, namesize, &namephys,
    GFP_KERNEL | GFP_DMA32);
    if (!namevirt)
    return -ENOMEM;
    datavirt = dma_alloc_coherent(bpmp.dev, datasize, &dataphys,
    GFP_KERNEL | GFP_DMA32);
    if (!datavirt) {
    err = -ENOMEM;
    goto free_namebuf;
    }
    len = strlen(filename);
    strscpy_pad(namevirt, filename, namesize);
    err = mrq_debugfs_read(bpmp, namephys, len, dataphys, datasize,
    &nbytes);
    if (!err)
    seq_write(m, datavirt, nbytes);
    dma_free_coherent(bpmp.dev, datasize, datavirt, dataphys);
    free_namebuf:
    dma_free_coherent(bpmp.dev, namesize, namevirt, namephys);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn debugfs_open(inode: *mut inode, file: *mut file) -> c_int {
    static int debugfs_open(struct inode *inode, struct file *file)
    {
    return single_open_size(file, debugfs_show, file, SZ_128K);
    }
    static ssize_t debugfs_store(struct file *file, const char __user *buf,
    size_t count, loff_t *f_pos)
    {
    struct inode *inode = file_inode(file);
    struct tegra_bpmp *bpmp = inode.i_private;
    let mut datasize: usize = count;
    let mut namesize: usize = SZ_256;
    void *datavirt, *namevirt;
    dma_addr_t dataphys, namephys;
    char fnamebuf[256];
    const char *filename;
    size_t len;
    int err;
    filename = get_filename(bpmp, file, fnamebuf, sizeof(fnamebuf));
    if (!filename)
    return -ENOENT;
    namevirt = dma_alloc_coherent(bpmp.dev, namesize, &namephys,
    GFP_KERNEL | GFP_DMA32);
    if (!namevirt)
    return -ENOMEM;
    datavirt = dma_alloc_coherent(bpmp.dev, datasize, &dataphys,
    GFP_KERNEL | GFP_DMA32);
    if (!datavirt) {
    err = -ENOMEM;
    goto free_namebuf;
    }
    len = strlen(filename);
    strscpy_pad(namevirt, filename, namesize);
    if (copy_from_user(datavirt, buf, count)) {
    err = -EFAULT;
    goto free_databuf;
    }
    err = mrq_debugfs_write(bpmp, namephys, len, dataphys,
    count);
    free_databuf:
    dma_free_coherent(bpmp.dev, datasize, datavirt, dataphys);
    free_namebuf:
    dma_free_coherent(bpmp.dev, namesize, namevirt, namephys);
    return err ?: count;
    }
    static const struct file_operations debugfs_fops = {
    .open		= debugfs_open,
    .read		= seq_read,
    .llseek		= seq_lseek,
    .write		= debugfs_store,
    .release	= single_release,
    };
    static int bpmp_populate_dir(struct tegra_bpmp *bpmp, struct seqbuf *seqbuf,
    struct dentry *parent, u32 depth)
    {
    int err;
    u32 d, t;
    const char *name;
    struct dentry *dentry;
    while (!seqbuf_eof(seqbuf)) {
    err = seqbuf_read_u32(seqbuf, &d);
    if (err < 0)
    return err;
    if (d < depth) {
    seqbuf_seek(seqbuf, -4);
// go up a level
    return 0;
    } else if (d != depth) {
// malformed data received from BPMP
    return -EIO;
    }
    err = seqbuf_read_u32(seqbuf, &t);
    if (err < 0)
    return err;
    err = seqbuf_read_str(seqbuf, &name);
    if (err < 0)
    return err;
    if (t & DEBUGFS_S_ISDIR) {
    dentry = debugfs_create_dir(name, parent);
    if (IS_ERR(dentry))
    return PTR_ERR(dentry);
    err = bpmp_populate_dir(bpmp, seqbuf, dentry, depth+1);
    if (err < 0)
    return err;
    } else {
    umode_t mode;
    mode = t & DEBUGFS_S_IRUSR ? S_IRUSR : 0;
    mode |= t & DEBUGFS_S_IWUSR ? S_IWUSR : 0;
    dentry = debugfs_create_file(name, mode,
    parent, bpmp,
    &debugfs_fops);
    if (IS_ERR(dentry))
    return PTR_ERR(dentry);
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bpmp_populate_debugfs_shmem(bpmp: *mut tegra_bpmp) -> c_int {
    static int bpmp_populate_debugfs_shmem(struct tegra_bpmp *bpmp)
    {
    struct seqbuf seqbuf;
    let mut sz: usize = SZ_512K;
    dma_addr_t phys;
    size_t nbytes;
    void *virt;
    int err;
    virt = dma_alloc_coherent(bpmp.dev, sz, &phys,
    GFP_KERNEL | GFP_DMA32);
    if (!virt)
    return -ENOMEM;
    err = mrq_debugfs_dumpdir(bpmp, phys, sz, &nbytes);
    if (err < 0) {
    goto free;
    } else if (nbytes > sz) {
    err = -EINVAL;
    goto free;
    }
    seqbuf_init(&seqbuf, virt, nbytes);
    err = bpmp_populate_dir(bpmp, &seqbuf, bpmp.debugfs_mirror, 0);
    free:
    dma_free_coherent(bpmp.dev, sz, virt, phys);
    return err;
    }
    static DEFINE_MUTEX(bpmp_debugfs_root_lock);
    static struct dentry *bpmp_debugfs_root;
    static struct dentry *bpmp_debugfs_get_root(void)
    {
    guard(mutex)(&bpmp_debugfs_root_lock);
    if (!bpmp_debugfs_root)
    bpmp_debugfs_root = debugfs_create_dir("bpmp", core::ptr::null_mut());
    return bpmp_debugfs_root;
    }
#[no_mangle]
pub unsafe extern "C" fn tegra_bpmp_init_debugfs(bpmp: *mut tegra_bpmp) -> c_int {
    int tegra_bpmp_init_debugfs(struct tegra_bpmp *bpmp)
    {
    struct dentry *root, *d;
    char name[32];
    bool inband;
    int err;
    inband = tegra_bpmp_mrq_is_supported(bpmp, MRQ_DEBUG);
    if (!inband && !tegra_bpmp_mrq_is_supported(bpmp, MRQ_DEBUGFS))
    return 0;
    root = bpmp_debugfs_get_root();
    if (IS_ERR(root))
    return PTR_ERR(root);
    if (dev_to_node(bpmp.dev) == NUMA_NO_NODE) {
    d = root;
    } else {
    snprintf(name, sizeof(name), "%d-bpmp", dev_to_node(bpmp.dev));
    d = debugfs_create_dir(name, root);
    if (IS_ERR(d)) {
    err = PTR_ERR(d);
    goto out;
    }
    }
    bpmp.debugfs_mirror = debugfs_create_dir("debug", d);
    if (IS_ERR(bpmp.debugfs_mirror)) {
    err = PTR_ERR(bpmp.debugfs_mirror);
    goto out;
    }
    if (inband)
    err = bpmp_populate_debugfs_inband(bpmp, bpmp.debugfs_mirror,
    "/");
    else
    err = bpmp_populate_debugfs_shmem(bpmp);
    out:
    if (err < 0) {
    if (!IS_ERR(d))
    debugfs_remove_recursive(d);
    guard(mutex)(&bpmp_debugfs_root_lock);
    if (root == d) {
    bpmp_debugfs_root = core::ptr::null_mut();
    } else if (simple_empty(root)) {
    debugfs_remove(root);
    bpmp_debugfs_root = core::ptr::null_mut();
    }
    }
    return err;
    }
