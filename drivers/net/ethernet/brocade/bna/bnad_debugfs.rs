//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/brocade/bna/bnad_debugfs.c
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
// Linux network driver for QLogic BR-series Converged Network Adapter.
//
// Copyright (c) 2005-2014 Brocade Communications Systems, Inc.
// Copyright (c) 2014-2015 QLogic Corporation
// All rights reserved
// www.qlogic.com
//

//
// BNA debufs interface
//
// To access the interface, debugfs file system should be mounted
// if not already mounted using:
// mount -t debugfs none /sys/kernel/debug
//
// BNA Hierarchy:
// - bna/pci_dev:<pci_name>
// where the pci_name corresponds to the one under /sys/bus/pci/drivers/bna
//
// Debugging service available per pci_dev:
// fwtrc:  To collect current firmware trace.
// fwsave: To collect last saved fw trace as a result of firmware crash.
// regwr:  To write one word to chip register
// regrd:  To read one or more words from chip register.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnad_debug_info {
    pub debug_buffer: *mut c_char,
    pub i_private: *mut c_void,
    pub buffer_len: c_int,
}

    static int
    bnad_debugfs_open_fwtrc(struct inode *inode, struct file *file)
    {
    struct bnad *bnad = inode.i_private;
    struct bnad_debug_info *fw_debug;
    unsigned long flags;
    int rc;
    fw_debug = kzalloc_obj(struct bnad_debug_info);
    if (!fw_debug)
    return -ENOMEM;
    fw_debug.buffer_len = BNA_DBG_FWTRC_LEN;
    fw_debug.debug_buffer = kzalloc(fw_debug.buffer_len, GFP_KERNEL);
    if (!fw_debug.debug_buffer) {
    kfree(fw_debug);
    fw_debug = core::ptr::null_mut();
    return -ENOMEM;
    }
    spin_lock_irqsave(&bnad.bna_lock, flags);
    rc = bfa_nw_ioc_debug_fwtrc(&bnad.bna.ioceth.ioc,
    fw_debug.debug_buffer,
    &fw_debug.buffer_len);
    spin_unlock_irqrestore(&bnad.bna_lock, flags);
    if (rc != BFA_STATUS_OK) {
    kfree(fw_debug.debug_buffer);
    fw_debug.debug_buffer = core::ptr::null_mut();
    kfree(fw_debug);
    fw_debug = core::ptr::null_mut();
    netdev_warn(bnad.netdev, "failed to collect fwtrc\n");
    return -ENOMEM;
    }
    file.private_data = fw_debug;
    return 0;
    }
    static int
    bnad_debugfs_open_fwsave(struct inode *inode, struct file *file)
    {
    struct bnad *bnad = inode.i_private;
    struct bnad_debug_info *fw_debug;
    unsigned long flags;
    int rc;
    fw_debug = kzalloc_obj(struct bnad_debug_info);
    if (!fw_debug)
    return -ENOMEM;
    fw_debug.buffer_len = BNA_DBG_FWTRC_LEN;
    fw_debug.debug_buffer = kzalloc(fw_debug.buffer_len, GFP_KERNEL);
    if (!fw_debug.debug_buffer) {
    kfree(fw_debug);
    fw_debug = core::ptr::null_mut();
    return -ENOMEM;
    }
    spin_lock_irqsave(&bnad.bna_lock, flags);
    rc = bfa_nw_ioc_debug_fwsave(&bnad.bna.ioceth.ioc,
    fw_debug.debug_buffer,
    &fw_debug.buffer_len);
    spin_unlock_irqrestore(&bnad.bna_lock, flags);
    if (rc != BFA_STATUS_OK && rc != BFA_STATUS_ENOFSAVE) {
    kfree(fw_debug.debug_buffer);
    fw_debug.debug_buffer = core::ptr::null_mut();
    kfree(fw_debug);
    fw_debug = core::ptr::null_mut();
    netdev_warn(bnad.netdev, "failed to collect fwsave\n");
    return -ENOMEM;
    }
    file.private_data = fw_debug;
    return 0;
    }
    static int
    bnad_debugfs_open_reg(struct inode *inode, struct file *file)
    {
    struct bnad_debug_info *reg_debug;
    reg_debug = kzalloc_obj(struct bnad_debug_info);
    if (!reg_debug)
    return -ENOMEM;
    reg_debug.i_private = inode.i_private;
    file.private_data = reg_debug;
    return 0;
    }
    static int
    bnad_get_debug_drvinfo(struct bnad *bnad, void *buffer, u32 len)
    {
    struct bnad_drvinfo *drvinfo = (struct bnad_drvinfo *) buffer;
    struct bnad_iocmd_comp fcomp;
    let mut flags: c_ulong = 0;
    let mut ret: c_int = BFA_STATUS_FAILED;
// Get IOC info
    spin_lock_irqsave(&bnad.bna_lock, flags);
    bfa_nw_ioc_get_attr(&bnad.bna.ioceth.ioc, &drvinfo.ioc_attr);
    spin_unlock_irqrestore(&bnad.bna_lock, flags);
// Retrieve CEE related info
    fcomp.bnad = bnad;
    fcomp.comp_status = 0;
    init_completion(&fcomp.comp);
    spin_lock_irqsave(&bnad.bna_lock, flags);
    ret = bfa_nw_cee_get_attr(&bnad.bna.cee, &drvinfo.cee_attr,
    bnad_cb_completion, &fcomp);
    if (ret != BFA_STATUS_OK) {
    spin_unlock_irqrestore(&bnad.bna_lock, flags);
    goto out;
    }
    spin_unlock_irqrestore(&bnad.bna_lock, flags);
    wait_for_completion(&fcomp.comp);
    drvinfo.cee_status = fcomp.comp_status;
// Retrieve flash partition info
    fcomp.comp_status = 0;
    reinit_completion(&fcomp.comp);
    spin_lock_irqsave(&bnad.bna_lock, flags);
    ret = bfa_nw_flash_get_attr(&bnad.bna.flash, &drvinfo.flash_attr,
    bnad_cb_completion, &fcomp);
    if (ret != BFA_STATUS_OK) {
    spin_unlock_irqrestore(&bnad.bna_lock, flags);
    goto out;
    }
    spin_unlock_irqrestore(&bnad.bna_lock, flags);
    wait_for_completion(&fcomp.comp);
    drvinfo.flash_status = fcomp.comp_status;
    out:
    return ret;
    }
    static int
    bnad_debugfs_open_drvinfo(struct inode *inode, struct file *file)
    {
    struct bnad *bnad = inode.i_private;
    struct bnad_debug_info *drv_info;
    int rc;
    drv_info = kzalloc_obj(struct bnad_debug_info);
    if (!drv_info)
    return -ENOMEM;
    drv_info.buffer_len = sizeof(struct bnad_drvinfo);
    drv_info.debug_buffer = kzalloc(drv_info.buffer_len, GFP_KERNEL);
    if (!drv_info.debug_buffer) {
    kfree(drv_info);
    drv_info = core::ptr::null_mut();
    return -ENOMEM;
    }
    mutex_lock(&bnad.conf_mutex);
    rc = bnad_get_debug_drvinfo(bnad, drv_info.debug_buffer,
    drv_info.buffer_len);
    mutex_unlock(&bnad.conf_mutex);
    if (rc != BFA_STATUS_OK) {
    kfree(drv_info.debug_buffer);
    drv_info.debug_buffer = core::ptr::null_mut();
    kfree(drv_info);
    drv_info = core::ptr::null_mut();
    netdev_warn(bnad.netdev, "failed to collect drvinfo\n");
    return -ENOMEM;
    }
    file.private_data = drv_info;
    return 0;
    }
// Changes the current file position
    static loff_t
    bnad_debugfs_lseek(struct file *file, loff_t offset, int orig)
    {
    struct bnad_debug_info *debug = file.private_data;
    if (!debug)
    return -EINVAL;
    return fixed_size_llseek(file, offset, orig, debug.buffer_len);
    }
    static ssize_t
    bnad_debugfs_read(struct file *file, char __user *buf,
    size_t nbytes, loff_t *pos)
    {
    struct bnad_debug_info *debug = file.private_data;
    if (!debug || !debug.debug_buffer)
    return 0;
    return simple_read_from_buffer(buf, nbytes, pos,
    debug.debug_buffer, debug.buffer_len);
    }

    ((u32)(bfa_asic_id_ctc(bfa_ioc_devid(__ioc)) ?  \
    BFA_REG_CT_ADDRSZ : BFA_REG_CB_ADDRSZ))

//
// Function to check if the register offset passed is valid.
//
    static int
    bna_reg_offset_check(struct bfa_ioc *ioc, u32 offset, u32 len)
    {
    u8 area;
// check [16:15]
    area = (offset >> 15) & 0x7;
    if (area == 0) {
// PCIe core register
    if (offset + (len << 2) > 0x8000)	/* 8k dwords or 32KB */
    return BFA_STATUS_EINVAL;
    } else if (area == 0x1) {
// CB 32 KB memory page
    if (offset + (len << 2) > 0x10000)	/* 8k dwords or 32KB */
    return BFA_STATUS_EINVAL;
    } else {
// CB register space 64KB
    if (offset + (len << 2) > BFA_REG_ADDRMSK(ioc))
    return BFA_STATUS_EINVAL;
    }
    return BFA_STATUS_OK;
    }
    static ssize_t
    bnad_debugfs_read_regrd(struct file *file, char __user *buf,
    size_t nbytes, loff_t *pos)
    {
    struct bnad_debug_info *regrd_debug = file.private_data;
    struct bnad *bnad = (struct bnad *)regrd_debug.i_private;
    ssize_t rc;
    if (!bnad.regdata)
    return 0;
    rc = simple_read_from_buffer(buf, nbytes, pos,
    bnad.regdata, bnad.reglen);
    if ((*pos + nbytes) >= bnad.reglen) {
    kfree(bnad.regdata);
    bnad.regdata = core::ptr::null_mut();
    bnad.reglen = 0;
    }
    return rc;
    }
    static ssize_t
    bnad_debugfs_write_regrd(struct file *file, const char __user *buf,
    size_t nbytes, loff_t *ppos)
    {
    struct bnad_debug_info *regrd_debug = file.private_data;
    struct bnad *bnad = (struct bnad *)regrd_debug.i_private;
    struct bfa_ioc *ioc = &bnad.bna.ioceth.ioc;
    int rc, i;
    u32 addr, len;
    u32 *regbuf;
    void __iomem *rb, *reg_addr;
    unsigned long flags;
    void *kern_buf;
// Copy the user space buf
    kern_buf = memdup_user_nul(buf, nbytes);
    if (IS_ERR(kern_buf))
    return PTR_ERR(kern_buf);
    rc = sscanf(kern_buf, "%x:%x", &addr, &len);
    if (rc < 2 || len > UINT_MAX >> 2) {
    netdev_warn(bnad.netdev, "failed to read user buffer\n");
    kfree(kern_buf);
    return -EINVAL;
    }
    kfree(kern_buf);
    kfree(bnad.regdata);
    bnad.reglen = 0;
    bnad.regdata = kzalloc(len << 2, GFP_KERNEL);
    if (!bnad.regdata)
    return -ENOMEM;
    bnad.reglen = len << 2;
    rb = bfa_ioc_bar0(ioc);
    addr &= BFA_REG_ADDRMSK(ioc);
// offset and len sanity check
    rc = bna_reg_offset_check(ioc, addr, len);
    if (rc) {
    netdev_warn(bnad.netdev, "failed reg offset check\n");
    kfree(bnad.regdata);
    bnad.regdata = core::ptr::null_mut();
    bnad.reglen = 0;
    return -EINVAL;
    }
    reg_addr = rb + addr;
    regbuf =  (u32 *)bnad.regdata;
    spin_lock_irqsave(&bnad.bna_lock, flags);
    for (i = 0; i < len; i++) {
// regbuf = readl(reg_addr);
    regbuf++;
    reg_addr += sizeof(u32);
    }
    spin_unlock_irqrestore(&bnad.bna_lock, flags);
    return nbytes;
    }
    static ssize_t
    bnad_debugfs_write_regwr(struct file *file, const char __user *buf,
    size_t nbytes, loff_t *ppos)
    {
    struct bnad_debug_info *debug = file.private_data;
    struct bnad *bnad = (struct bnad *)debug.i_private;
    struct bfa_ioc *ioc = &bnad.bna.ioceth.ioc;
    int rc;
    u32 addr, val;
    void __iomem *reg_addr;
    unsigned long flags;
    void *kern_buf;
// Copy the user space buf
    kern_buf = memdup_user_nul(buf, nbytes);
    if (IS_ERR(kern_buf))
    return PTR_ERR(kern_buf);
    rc = sscanf(kern_buf, "%x:%x", &addr, &val);
    if (rc < 2) {
    netdev_warn(bnad.netdev, "failed to read user buffer\n");
    kfree(kern_buf);
    return -EINVAL;
    }
    kfree(kern_buf);
    addr &= BFA_REG_ADDRMSK(ioc); /* offset only 17 bit and word align */
// offset and len sanity check
    rc = bna_reg_offset_check(ioc, addr, 1);
    if (rc) {
    netdev_warn(bnad.netdev, "failed reg offset check\n");
    return -EINVAL;
    }
    reg_addr = (bfa_ioc_bar0(ioc)) + addr;
    spin_lock_irqsave(&bnad.bna_lock, flags);
    writel(val, reg_addr);
    spin_unlock_irqrestore(&bnad.bna_lock, flags);
    return nbytes;
    }
    static int
    bnad_debugfs_release(struct inode *inode, struct file *file)
    {
    struct bnad_debug_info *debug = file.private_data;
    if (!debug)
    return 0;
    file.private_data = core::ptr::null_mut();
    kfree(debug);
    return 0;
    }
    static int
    bnad_debugfs_buffer_release(struct inode *inode, struct file *file)
    {
    struct bnad_debug_info *debug = file.private_data;
    if (!debug)
    return 0;
    kfree(debug.debug_buffer);
    file.private_data = core::ptr::null_mut();
    kfree(debug);
    debug = core::ptr::null_mut();
    return 0;
    }
    static const struct file_operations bnad_debugfs_op_fwtrc = {
    .owner		=	THIS_MODULE,
    .open		=	bnad_debugfs_open_fwtrc,
    .llseek		=	bnad_debugfs_lseek,
    .read		=	bnad_debugfs_read,
    .release	=	bnad_debugfs_buffer_release,
    };
    static const struct file_operations bnad_debugfs_op_fwsave = {
    .owner		=	THIS_MODULE,
    .open		=	bnad_debugfs_open_fwsave,
    .llseek		=	bnad_debugfs_lseek,
    .read		=	bnad_debugfs_read,
    .release	=	bnad_debugfs_buffer_release,
    };
    static const struct file_operations bnad_debugfs_op_regrd = {
    .owner		=       THIS_MODULE,
    .open		=	bnad_debugfs_open_reg,
    .llseek		=	bnad_debugfs_lseek,
    .read		=	bnad_debugfs_read_regrd,
    .write		=	bnad_debugfs_write_regrd,
    .release	=	bnad_debugfs_release,
    };
    static const struct file_operations bnad_debugfs_op_regwr = {
    .owner		=	THIS_MODULE,
    .open		=	bnad_debugfs_open_reg,
    .llseek		=	bnad_debugfs_lseek,
    .write		=	bnad_debugfs_write_regwr,
    .release	=	bnad_debugfs_release,
    };
    static const struct file_operations bnad_debugfs_op_drvinfo = {
    .owner		=	THIS_MODULE,
    .open		=	bnad_debugfs_open_drvinfo,
    .llseek		=	bnad_debugfs_lseek,
    .read		=	bnad_debugfs_read,
    .release	=	bnad_debugfs_buffer_release,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnad_debugfs_entry {
    pub name: *const c_char,
    pub mode: umode_t,
    pub fops: *const file_operations,
}

    static const struct bnad_debugfs_entry bnad_debugfs_files[] = {
    { "fwtrc",  S_IFREG | 0444, &bnad_debugfs_op_fwtrc, },
    { "fwsave", S_IFREG | 0444, &bnad_debugfs_op_fwsave, },
    { "regrd",  S_IFREG | 0644, &bnad_debugfs_op_regrd, },
    { "regwr",  S_IFREG | 0200, &bnad_debugfs_op_regwr, },
    { "drvinfo", S_IFREG | 0444, &bnad_debugfs_op_drvinfo, },
    };
    static struct dentry *bna_debugfs_root;
    static atomic_t bna_debugfs_port_count;
// Initialize debugfs interface for BNA
    void
    bnad_debugfs_init(struct bnad *bnad)
    {
    const struct bnad_debugfs_entry *file;
    char name[64];
    int i;
// Setup the BNA debugfs root directory
    if (!bna_debugfs_root) {
    bna_debugfs_root = debugfs_create_dir("bna", core::ptr::null_mut());
    atomic_set(&bna_debugfs_port_count, 0);
    }
// Setup the pci_dev debugfs directory for the port
    snprintf(name, sizeof(name), "pci_dev:%s", pci_name(bnad.pcidev));
    if (!bnad.port_debugfs_root) {
    bnad.port_debugfs_root =
    debugfs_create_dir(name, bna_debugfs_root);
    atomic_inc(&bna_debugfs_port_count);
    for (i = 0; i < ARRAY_SIZE(bnad_debugfs_files); i++) {
    file = &bnad_debugfs_files[i];
    debugfs_create_file(file.name,
    file.mode,
    bnad.port_debugfs_root,
    bnad,
    file.fops);
    }
    }
    }
// Uninitialize debugfs interface for BNA
    void
    bnad_debugfs_uninit(struct bnad *bnad)
    {
// Remove the pci_dev debugfs directory for the port
    if (bnad.port_debugfs_root) {
    debugfs_remove(bnad.port_debugfs_root);
    bnad.port_debugfs_root = core::ptr::null_mut();
    atomic_dec(&bna_debugfs_port_count);
    }
// Remove the BNA debugfs root directory
    if (atomic_read(&bna_debugfs_port_count) == 0) {
    debugfs_remove(bna_debugfs_root);
    bna_debugfs_root = core::ptr::null_mut();
    }
    }
