//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/drm_debugfs.c
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
// Created: Sun Dec 21 13:08:50 2008 by bgamari@gmail.com
//
// Copyright 2008 Ben Gamari <bgamari@gmail.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice (including the next
// paragraph) shall be included in all copies or substantial portions of the
// Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// VA LINUX SYSTEMS AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//

    static struct dentry *accel_debugfs_root;
    static struct dentry *drm_debugfs_root;
//
// Initialization, etc.
//
#[no_mangle]
unsafe extern "C" fn drm_name_info(m: *mut seq_file, data: *mut c_void) -> c_int {
    static int drm_name_info(struct seq_file *m, void *data)
    {
    struct drm_debugfs_entry *entry = m.private;
    struct drm_device *dev = entry.dev;
    struct drm_master *master;
    mutex_lock(&dev.master_mutex);
    master = dev.master;
    seq_printf(m, "%s", dev.driver.name);
    if (dev.dev)
    seq_printf(m, " dev=%s", dev_name(dev.dev));
    if (master && master.unique)
    seq_printf(m, " master=%s", master.unique);
    if (dev.unique)
    seq_printf(m, " unique=%s", dev.unique);
    seq_printf(m, "\n");
    mutex_unlock(&dev.master_mutex);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn drm_clients_info(m: *mut seq_file, data: *mut c_void) -> c_int {
    static int drm_clients_info(struct seq_file *m, void *data)
    {
    struct drm_debugfs_entry *entry = m.private;
    struct drm_device *dev = entry.dev;
    struct drm_file *priv;
    kuid_t uid;
    seq_printf(m,
    "%20s %5s %3s master a %5s %10s %*s %20s\n",
    "command",
    "tgid",
    "dev",
    "uid",
    "magic",
    DRM_CLIENT_NAME_MAX_LEN,
    "name",
    "id");
// dev->filelist is sorted youngest first, but we want to present
// oldest first (i.e. kernel, servers, clients), so walk backwardss.
//
    mutex_lock(&dev.filelist_mutex);
    list_for_each_entry_reverse(priv, &dev.filelist, lhead) {
    let mut is_current_master: bool = drm_is_current_master(priv);
    struct task_struct *task;
    struct pid *pid;
    mutex_lock(&priv.client_name_lock);
    rcu_read_lock(); /* Locks priv.pid and pid_task().comm! */
    pid = rcu_dereference(priv.pid);
    task = pid_task(pid, PIDTYPE_TGID);
    uid = task ? __task_cred(task).euid : GLOBAL_ROOT_UID;
    seq_printf(m, "%20s %5d %3d   %c    %c %5d %10u %*s %20llu\n",
    task ? task.comm : "<unknown>",
    pid_vnr(pid),
    priv.minor.index,
    is_current_master ? 'y' : 'n',
    priv.authenticated ? 'y' : 'n',
    from_kuid_munged(seq_user_ns(m), uid),
    priv.magic,
    DRM_CLIENT_NAME_MAX_LEN,
    priv.client_name ? priv.client_name : "<unset>",
    priv.client_id);
    rcu_read_unlock();
    mutex_unlock(&priv.client_name_lock);
    }
    mutex_unlock(&dev.filelist_mutex);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn drm_gem_one_name_info(id: c_int, ptr: *mut c_void, data: *mut c_void) -> c_int {
    static int drm_gem_one_name_info(int id, void *ptr, void *data)
    {
    struct drm_gem_object *obj = ptr;
    struct seq_file *m = data;
    seq_printf(m, "%6d %8zd %7d %8d\n",
    obj.name, obj.size,
    obj.handle_count,
    kref_read(&obj.refcount));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn drm_gem_name_info(m: *mut seq_file, data: *mut c_void) -> c_int {
    static int drm_gem_name_info(struct seq_file *m, void *data)
    {
    struct drm_debugfs_entry *entry = m.private;
    struct drm_device *dev = entry.dev;
    seq_printf(m, "  name     size handles refcount\n");
    mutex_lock(&dev.object_name_lock);
    idr_for_each(&dev.object_name_idr, drm_gem_one_name_info, m);
    mutex_unlock(&dev.object_name_lock);
    return 0;
    }
    static const struct drm_debugfs_info drm_debugfs_list[] = {
    {"name", drm_name_info, 0},
    {"clients", drm_clients_info, 0},
    {"gem_names", drm_gem_name_info, DRIVER_GEM},
    };

#[no_mangle]
unsafe extern "C" fn drm_debugfs_open(inode: *mut inode, file: *mut file) -> c_int {
    static int drm_debugfs_open(struct inode *inode, struct file *file)
    {
    struct drm_info_node *node = inode.i_private;
    if (!device_is_registered(node.minor.kdev))
    return -ENODEV;
    return single_open(file, node.info_ent.show, node);
    }
#[no_mangle]
unsafe extern "C" fn drm_debugfs_entry_open(inode: *mut inode, file: *mut file) -> c_int {
    static int drm_debugfs_entry_open(struct inode *inode, struct file *file)
    {
    struct drm_debugfs_entry *entry = inode.i_private;
    struct drm_debugfs_info *node = &entry.file;
    struct drm_minor *minor = entry.dev.primary ?: entry.dev.accel;
    if (!device_is_registered(minor.kdev))
    return -ENODEV;
    return single_open(file, node.show, entry);
    }
    static const struct file_operations drm_debugfs_entry_fops = {
    .owner = THIS_MODULE,
    .open = drm_debugfs_entry_open,
    .read = seq_read,
    .llseek = seq_lseek,
    .release = single_release,
    };
    static const struct file_operations drm_debugfs_fops = {
    .owner = THIS_MODULE,
    .open = drm_debugfs_open,
    .read = seq_read,
    .llseek = seq_lseek,
    .release = single_release,
    };
//
// drm_debugfs_gpuva_info - dump the given DRM GPU VA space
// @m: pointer to the &seq_file to write
// @gpuvm: the &drm_gpuvm representing the GPU VA space
//
// Dumps the GPU VA mappings of a given DRM GPU VA manager.
//
// For each DRM GPU VA space drivers should call this function from their
// &drm_info_list's show callback.
//
// Returns: 0 on success, -ENODEV if the &gpuvm is not initialized
//
    int drm_debugfs_gpuva_info(struct seq_file *m,
    struct drm_gpuvm *gpuvm)
    {
    struct drm_gpuva *va, *kva = &gpuvm.kernel_alloc_node;
    if (!gpuvm.name)
    return -ENODEV;
    seq_printf(m, "DRM GPU VA space (%s) [0x%016llx;0x%016llx]\n",
    gpuvm.name, gpuvm.mm_start, gpuvm.mm_start + gpuvm.mm_range);
    seq_printf(m, "Kernel reserved node [0x%016llx;0x%016llx]\n",
    kva.va.addr, kva.va.addr + kva.va.range);
    seq_puts(m, "\n");
    seq_puts(m, " VAs | start              | range              | end                | object             | object offset\n");
    seq_puts(m, "-------------------------------------------------------------------------------------------------------------\n");
    drm_gpuvm_for_each_va(va, gpuvm) {
    if (unlikely(va == kva))
    continue;
    seq_printf(m, "     | 0x%016llx | 0x%016llx | 0x%016llx | 0x%016llx | 0x%016llx\n",
    va.va.addr, va.va.range, va.va.addr + va.va.range,
    (u64)(uintptr_t)va.gem.obj, va.gem.offset);
    }
    return 0;
    }
    EXPORT_SYMBOL(drm_debugfs_gpuva_info);
//
// drm_debugfs_create_files - Initialize a given set of debugfs files for DRM
// minor
// @files: The array of files to create
// @count: The number of files given
// @root: DRI debugfs dir entry.
// @minor: device minor number
//
// Create a given set of debugfs files represented by an array of
// &struct drm_info_list in the given root directory. These files will be removed
// automatically on drm_debugfs_dev_fini().
//
    void drm_debugfs_create_files(const struct drm_info_list *files, int count,
    struct dentry *root, struct drm_minor *minor)
    {
    struct drm_device *dev = minor.dev;
    struct drm_info_node *tmp;
    int i;
    for (i = 0; i < count; i++) {
    let mut features: u32 = files[i].driver_features;
    if (features && !drm_core_check_all_features(dev, features))
    continue;
    tmp = drmm_kzalloc(dev, sizeof(*tmp), GFP_KERNEL);
    if (tmp == core::ptr::null_mut())
    continue;
    tmp.minor = minor;
    tmp.dent = debugfs_create_file(files[i].name,
    0444, root, tmp,
    &drm_debugfs_fops);
    tmp.info_ent = &files[i];
    }
    }
    EXPORT_SYMBOL(drm_debugfs_create_files);
    int drm_debugfs_remove_files(const struct drm_info_list *files, int count,
    struct dentry *root, struct drm_minor *minor)
    {
    int i;
    for (i = 0; i < count; i++) {
    struct dentry *dent = debugfs_lookup(files[i].name, root);
    if (!dent)
    continue;
    drmm_kfree(minor.dev, d_inode(dent).i_private);
    debugfs_remove(dent);
    }
    return 0;
    }
    EXPORT_SYMBOL(drm_debugfs_remove_files);
#[no_mangle]
pub unsafe extern "C" fn drm_debugfs_bridge_params() {
    void drm_debugfs_bridge_params(void)
    {
    drm_bridge_debugfs_params(drm_debugfs_root);
    }
#[no_mangle]
pub unsafe extern "C" fn drm_debugfs_init_root() {
    void drm_debugfs_init_root(void)
    {
    drm_debugfs_root = debugfs_create_dir("dri", core::ptr::null_mut());

    accel_debugfs_root = debugfs_create_dir("accel", core::ptr::null_mut());

    }
#[no_mangle]
pub unsafe extern "C" fn drm_debugfs_remove_root() {
    void drm_debugfs_remove_root(void)
    {

    debugfs_remove(accel_debugfs_root);

    debugfs_remove(drm_debugfs_root);
    }
#[no_mangle]
unsafe extern "C" fn drm_debugfs_proc_info_show(m: *mut seq_file, unused: *mut c_void) -> c_int {
    static int drm_debugfs_proc_info_show(struct seq_file *m, void *unused)
    {
    struct pid *pid;
    struct task_struct *task;
    struct drm_file *file = m.private;
    if (!file)
    return -EINVAL;
    rcu_read_lock();
    pid = rcu_dereference(file.pid);
    task = pid_task(pid, PIDTYPE_TGID);
    seq_printf(m, "pid: %d\n", task ? task.pid : 0);
    seq_printf(m, "comm: %s\n", task ? task.comm : "Unset");
    rcu_read_unlock();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn drm_debufs_proc_info_open(inode: *mut inode, file: *mut file) -> c_int {
    static int drm_debufs_proc_info_open(struct inode *inode, struct file *file)
    {
    return single_open(file, drm_debugfs_proc_info_show, inode.i_private);
    }
    static const struct file_operations drm_debugfs_proc_info_fops = {
    .owner = THIS_MODULE,
    .open = drm_debufs_proc_info_open,
    .read = seq_read,
    .llseek = seq_lseek,
    .release = single_release,
    };
//
// drm_debugfs_clients_add - Add a per client debugfs directory
// @file: drm_file for a client
//
// Create the debugfs directory for each client. This will be used to populate
// driver specific data for each client.
//
// Also add the process information debugfs file for each client to tag
// which client belongs to which process.
//
#[no_mangle]
pub unsafe extern "C" fn drm_debugfs_clients_add(file: *mut drm_file) {
    void drm_debugfs_clients_add(struct drm_file *file)
    {
    char *client;
    client = kasprintf(GFP_KERNEL, "client-%llu", file.client_id);
    if (!client)
    return;
// Create a debugfs directory for the client in root on drm debugfs
    file.debugfs_client = debugfs_create_dir(client, drm_debugfs_root);
    kfree(client);
    debugfs_create_file("proc_info", 0444, file.debugfs_client, file,
    &drm_debugfs_proc_info_fops);
    client = kasprintf(GFP_KERNEL, "../%s", file.minor.dev.unique);
    if (!client)
    return;
// Create a link from client_id to the drm device this client id belongs to
    debugfs_create_symlink("device", file.debugfs_client, client);
    kfree(client);
    }
//
// drm_debugfs_clients_remove - removes all debugfs directories and files
// @file: drm_file for a client
//
// Removes the debugfs directories recursively from the client directory.
//
// There is also a possibility that debugfs files are open while the drm_file
// is released.
//
#[no_mangle]
pub unsafe extern "C" fn drm_debugfs_clients_remove(file: *mut drm_file) {
    void drm_debugfs_clients_remove(struct drm_file *file)
    {
    debugfs_remove_recursive(file.debugfs_client);
    file.debugfs_client = core::ptr::null_mut();
    }
//
// drm_debugfs_dev_init - create debugfs directory for the device
// @dev: the device which we want to create the directory for
//
// Creates the debugfs directory for the device under the given root directory.
//
#[no_mangle]
pub unsafe extern "C" fn drm_debugfs_dev_init(dev: *mut drm_device) {
    void drm_debugfs_dev_init(struct drm_device *dev)
    {
    if (drm_core_check_feature(dev, DRIVER_COMPUTE_ACCEL))
    dev.debugfs_root = debugfs_create_dir(dev.unique, accel_debugfs_root);
    else
    dev.debugfs_root = debugfs_create_dir(dev.unique, drm_debugfs_root);
    }
//
// drm_debugfs_dev_fini - cleanup debugfs directory
// @dev: the device to cleanup the debugfs stuff
//
// Remove the debugfs directory, might be called multiple times.
//
#[no_mangle]
pub unsafe extern "C" fn drm_debugfs_dev_fini(dev: *mut drm_device) {
    void drm_debugfs_dev_fini(struct drm_device *dev)
    {
    debugfs_remove_recursive(dev.debugfs_root);
    dev.debugfs_root = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn drm_debugfs_dev_register(dev: *mut drm_device) {
    void drm_debugfs_dev_register(struct drm_device *dev)
    {
    drm_debugfs_add_files(dev, drm_debugfs_list, DRM_DEBUGFS_ENTRIES);
    if (drm_core_check_feature(dev, DRIVER_MODESET)) {
    drm_framebuffer_debugfs_init(dev);
    drm_client_debugfs_init(dev);
    }
    if (drm_drv_uses_atomic_modeset(dev))
    drm_atomic_debugfs_init(dev);
    }
#[no_mangle]
pub unsafe extern "C" fn drm_debugfs_register(minor: *mut drm_minor, minor_id: c_int) -> c_int {
    int drm_debugfs_register(struct drm_minor *minor, int minor_id)
    {
    struct drm_device *dev = minor.dev;
    char name[64];
    sprintf(name, "%d", minor_id);
    minor.debugfs_symlink = debugfs_create_symlink(name, drm_debugfs_root,
    dev.unique);
// TODO: Only for compatibility with drivers
    minor.debugfs_root = dev.debugfs_root;
    if (dev.driver.debugfs_init && dev.render != minor)
    dev.driver.debugfs_init(minor);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn drm_debugfs_unregister(minor: *mut drm_minor) {
    void drm_debugfs_unregister(struct drm_minor *minor)
    {
    debugfs_remove(minor.debugfs_symlink);
    minor.debugfs_symlink = core::ptr::null_mut();
    }
//
// drm_debugfs_add_file - Add a given file to the DRM device debugfs file list
// @dev: drm device for the ioctl
// @name: debugfs file name
// @show: show callback
// @data: driver-private data, should not be device-specific
//
// Add a given file entry to the DRM device debugfs file list to be created on
// drm_debugfs_init.
//
    void drm_debugfs_add_file(struct drm_device *dev, const char *name,
    int (*show)(struct seq_file*, void*), void *data)
    {
    struct drm_debugfs_entry *entry = drmm_kzalloc(dev, sizeof(*entry), GFP_KERNEL);
    if (!entry)
    return;
    entry.file.name = name;
    entry.file.show = show;
    entry.file.data = data;
    entry.dev = dev;
    debugfs_create_file(name, 0444, dev.debugfs_root, entry,
    &drm_debugfs_entry_fops);
    }
    EXPORT_SYMBOL(drm_debugfs_add_file);
//
// drm_debugfs_add_files - Add an array of files to the DRM device debugfs file list
// @dev: drm device for the ioctl
// @files: The array of files to create
// @count: The number of files given
//
// Add a given set of debugfs files represented by an array of
// &struct drm_debugfs_info in the DRM device debugfs file list.
//
#[no_mangle]
pub unsafe extern "C" fn drm_debugfs_add_files(dev: *mut drm_device, files: *const drm_debugfs_info, count: c_int) {
    void drm_debugfs_add_files(struct drm_device *dev, const struct drm_debugfs_info *files, int count)
    {
    int i;
    for (i = 0; i < count; i++)
    drm_debugfs_add_file(dev, files[i].name, files[i].show, files[i].data);
    }
    EXPORT_SYMBOL(drm_debugfs_add_files);
#[no_mangle]
unsafe extern "C" fn connector_show(m: *mut seq_file, data: *mut c_void) -> c_int {
    static int connector_show(struct seq_file *m, void *data)
    {
    struct drm_connector *connector = m.private;
    seq_printf(m, "%s\n", drm_get_connector_force_name(connector.force));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn connector_open(inode: *mut inode, file: *mut file) -> c_int {
    static int connector_open(struct inode *inode, struct file *file)
    {
    struct drm_connector *dev = inode.i_private;
    return single_open(file, connector_show, dev);
    }
    static ssize_t connector_write(struct file *file, const char __user *ubuf,
    size_t len, loff_t *offp)
    {
    struct seq_file *m = file.private_data;
    struct drm_connector *connector = m.private;
    char buf[12];
    if (len > sizeof(buf) - 1)
    return -EINVAL;
    if (copy_from_user(buf, ubuf, len))
    return -EFAULT;
    buf[len] = '\0';
    if (sysfs_streq(buf, "on"))
    connector.force = DRM_FORCE_ON;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: sysfs_streq(buf, _arg: "digital")) -> else {
    else if (sysfs_streq(buf, "digital"))
    connector.force = DRM_FORCE_ON_DIGITAL;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: sysfs_streq(buf, _arg: "off")) -> else {
    else if (sysfs_streq(buf, "off"))
    connector.force = DRM_FORCE_OFF;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: sysfs_streq(buf, _arg: "unspecified")) -> else {
    else if (sysfs_streq(buf, "unspecified"))
    connector.force = DRM_FORCE_UNSPECIFIED;
    else
    return -EINVAL;
    return len;
    }
#[no_mangle]
unsafe extern "C" fn edid_show(m: *mut seq_file, data: *mut c_void) -> c_int {
    static int edid_show(struct seq_file *m, void *data)
    {
    return drm_edid_override_show(m.private, m);
    }
#[no_mangle]
unsafe extern "C" fn edid_open(inode: *mut inode, file: *mut file) -> c_int {
    static int edid_open(struct inode *inode, struct file *file)
    {
    struct drm_connector *dev = inode.i_private;
    return single_open(file, edid_show, dev);
    }
    static ssize_t edid_write(struct file *file, const char __user *ubuf,
    size_t len, loff_t *offp)
    {
    struct seq_file *m = file.private_data;
    struct drm_connector *connector = m.private;
    char *buf;
    int ret;
    buf = memdup_user(ubuf, len);
    if (IS_ERR(buf))
    return PTR_ERR(buf);
    if (len == 5 && !strncmp(buf, "reset", 5))
    ret = drm_edid_override_reset(connector);
    else
    ret = drm_edid_override_set(connector, buf, len);
    kfree(buf);
    return ret ? ret : len;
    }
//
// Returns the min and max vrr vfreq through the connector's debugfs file.
// Example usage: cat /sys/kernel/debug/dri/0/DP-1/vrr_range
//
#[no_mangle]
unsafe extern "C" fn vrr_range_show(m: *mut seq_file, data: *mut c_void) -> c_int {
    static int vrr_range_show(struct seq_file *m, void *data)
    {
    struct drm_connector *connector = m.private;
    if (connector.status != connector_status_connected)
    return -ENODEV;
    seq_printf(m, "Min: %u\n", connector.display_info.monitor_range.min_vfreq);
    seq_printf(m, "Max: %u\n", connector.display_info.monitor_range.max_vfreq);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(vrr_range);
//
// Returns Connector's max supported bpc through debugfs file.
// Example usage: cat /sys/kernel/debug/dri/0/DP-1/output_bpc
//
#[no_mangle]
unsafe extern "C" fn output_bpc_show(m: *mut seq_file, data: *mut c_void) -> c_int {
    static int output_bpc_show(struct seq_file *m, void *data)
    {
    struct drm_connector *connector = m.private;
    if (connector.status != connector_status_connected)
    return -ENODEV;
    seq_printf(m, "Maximum: %u\n", connector.display_info.bpc);
    return 0;
    }
    DEFINE_SHOW_ATTRIBUTE(output_bpc);
    static const struct file_operations drm_edid_fops = {
    .owner = THIS_MODULE,
    .open = edid_open,
    .read = seq_read,
    .llseek = seq_lseek,
    .release = single_release,
    .write = edid_write
    };
    static const struct file_operations drm_connector_fops = {
    .owner = THIS_MODULE,
    .open = connector_open,
    .read = seq_read,
    .llseek = seq_lseek,
    .release = single_release,
    .write = connector_write
    };
    static ssize_t
    audio_infoframe_read(struct file *filp, char __user *ubuf, size_t count, loff_t *ppos)
    {
    struct drm_connector_hdmi_infoframe *infoframe;
    struct drm_connector *connector;
    union hdmi_infoframe *frame;
    u8 buf[HDMI_INFOFRAME_SIZE(AUDIO)];
    let mut len: isize = 0;
    connector = filp.private_data;
    mutex_lock(&connector.hdmi.infoframes.lock);
    infoframe = &connector.hdmi.infoframes.audio;
    if (!infoframe.set)
    goto out;
    frame = &infoframe.data;
    len = hdmi_infoframe_pack(frame, buf, sizeof(buf));
    if (len < 0)
    goto out;
    len = simple_read_from_buffer(ubuf, count, ppos, buf, len);
    out:
    mutex_unlock(&connector.hdmi.infoframes.lock);
    return len;
    }
    static const struct file_operations audio_infoframe_fops = {
    .owner   = THIS_MODULE,
    .open    = simple_open,
    .read    = audio_infoframe_read,
    };
    static int create_hdmi_audio_infoframe_file(struct drm_connector *connector,
    struct dentry *parent)
    {
    struct dentry *file;
    if (!connector.hdmi.funcs ||
    !connector.hdmi.funcs.audio.write_infoframe)
    return 0;
    file = debugfs_create_file("audio", 0400, parent, connector, &audio_infoframe_fops);
    if (IS_ERR(file))
    return PTR_ERR(file);
    return 0;
    }

    static ssize_t _f##_read_infoframe(struct file *filp, \
    char __user *ubuf, \
    size_t count,      \
    loff_t *ppos)      \
    { \
    struct drm_connector_hdmi_infoframe *infoframe; \
    struct drm_connector_state *conn_state; \
    struct drm_connector *connector; \
    union hdmi_infoframe *frame; \
    struct drm_device *dev; \
    u8 buf[HDMI_INFOFRAME_SIZE(MAX)]; \
    ssize_t len = 0; \
    \
    connector = filp.private_data; \
    dev = connector.dev; \
    \
    drm_modeset_lock(&dev.mode_config.connection_mutex, core::ptr::null_mut()); \
    \
    conn_state = connector.state; \
    infoframe = &conn_state.hdmi.infoframes._f; \
    if (!infoframe.set) \
    goto out; \
    \
    frame = &infoframe.data; \
    len = hdmi_infoframe_pack(frame, buf, sizeof(buf)); \
    if (len < 0) \
    goto out; \
    \
    len = simple_read_from_buffer(ubuf, count, ppos, buf, len); \
    \
    out: \
    drm_modeset_unlock(&dev.mode_config.connection_mutex); \
    return len; \
    } \
    \
    static const struct file_operations _f##_infoframe_fops = { \
    .owner = THIS_MODULE, \
    .open = simple_open, \
    .read = _f##_read_infoframe, \
    }; \
    \
    static int create_hdmi_## _f ## _infoframe_file(struct drm_connector *connector, \
    struct dentry *parent) \
    { \
    struct dentry *file; \
    \
    if (!connector.hdmi.funcs || \
    !connector.hdmi.funcs._f.write_infoframe) \
    return 0; \
    file = debugfs_create_file(#_f, 0400, parent, connector, &_f ## _infoframe_fops); \
    if (IS_ERR(file)) \
    return PTR_ERR(file); \
    \
    return 0; \
    }
    DEFINE_INFOFRAME_FILE(avi);
    DEFINE_INFOFRAME_FILE(hdmi);
    DEFINE_INFOFRAME_FILE(hdr_drm);
    DEFINE_INFOFRAME_FILE(spd);
    static int create_hdmi_infoframe_files(struct drm_connector *connector,
    struct dentry *parent)
    {
    int ret;
    ret = create_hdmi_audio_infoframe_file(connector, parent);
    if (ret)
    return ret;
    ret = create_hdmi_avi_infoframe_file(connector, parent);
    if (ret)
    return ret;
    ret = create_hdmi_hdmi_infoframe_file(connector, parent);
    if (ret)
    return ret;
    ret = create_hdmi_hdr_drm_infoframe_file(connector, parent);
    if (ret)
    return ret;
    ret = create_hdmi_spd_infoframe_file(connector, parent);
    if (ret)
    return ret;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hdmi_debugfs_add(connector: *mut drm_connector) {
    static void hdmi_debugfs_add(struct drm_connector *connector)
    {
    struct dentry *dir;
    if (!(connector.connector_type == DRM_MODE_CONNECTOR_HDMIA ||
    connector.connector_type == DRM_MODE_CONNECTOR_HDMIB))
    return;
    dir = debugfs_create_dir("infoframes", connector.debugfs_entry);
    if (IS_ERR(dir))
    return;
    create_hdmi_infoframe_files(connector, dir);
    }
#[no_mangle]
pub unsafe extern "C" fn drm_debugfs_connector_add(connector: *mut drm_connector) {
    void drm_debugfs_connector_add(struct drm_connector *connector)
    {
    struct drm_device *dev = connector.dev;
    struct dentry *root;
    if (!dev.debugfs_root)
    return;
    root = debugfs_create_dir(connector.name, dev.debugfs_root);
    connector.debugfs_entry = root;
// force
    debugfs_create_file("force", 0644, root, connector,
    &drm_connector_fops);
// edid
    debugfs_create_file("edid_override", 0644, root, connector,
    &drm_edid_fops);
// vrr range
    debugfs_create_file("vrr_range", 0444, root, connector,
    &vrr_range_fops);
// max bpc
    debugfs_create_file("output_bpc", 0444, root, connector,
    &output_bpc_fops);
    hdmi_debugfs_add(connector);
    if (connector.funcs.debugfs_init)
    connector.funcs.debugfs_init(connector, root);
    }
#[no_mangle]
pub unsafe extern "C" fn drm_debugfs_connector_remove(connector: *mut drm_connector) {
    void drm_debugfs_connector_remove(struct drm_connector *connector)
    {
    if (!connector.debugfs_entry)
    return;
    debugfs_remove_recursive(connector.debugfs_entry);
    connector.debugfs_entry = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn drm_debugfs_crtc_add(crtc: *mut drm_crtc) {
    void drm_debugfs_crtc_add(struct drm_crtc *crtc)
    {
    struct drm_device *dev = crtc.dev;
    struct dentry *root;
    char *name;
    name = kasprintf(GFP_KERNEL, "crtc-%d", crtc.index);
    if (!name)
    return;
    root = debugfs_create_dir(name, dev.debugfs_root);
    kfree(name);
    crtc.debugfs_entry = root;
    drm_debugfs_crtc_crc_add(crtc);
    }
#[no_mangle]
pub unsafe extern "C" fn drm_debugfs_crtc_remove(crtc: *mut drm_crtc) {
    void drm_debugfs_crtc_remove(struct drm_crtc *crtc)
    {
    debugfs_remove_recursive(crtc.debugfs_entry);
    crtc.debugfs_entry = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn drm_debugfs_encoder_add(encoder: *mut drm_encoder) {
    void drm_debugfs_encoder_add(struct drm_encoder *encoder)
    {
    struct drm_minor *minor = encoder.dev.primary;
    struct dentry *root;
    char *name;
    name = kasprintf(GFP_KERNEL, "encoder-%d", encoder.index);
    if (!name)
    return;
    root = debugfs_create_dir(name, minor.debugfs_root);
    kfree(name);
    encoder.debugfs_entry = root;
    drm_bridge_debugfs_encoder_params(root, encoder);
    if (encoder.funcs && encoder.funcs.debugfs_init)
    encoder.funcs.debugfs_init(encoder, root);
    }
#[no_mangle]
pub unsafe extern "C" fn drm_debugfs_encoder_remove(encoder: *mut drm_encoder) {
    void drm_debugfs_encoder_remove(struct drm_encoder *encoder)
    {
    debugfs_remove_recursive(encoder.debugfs_entry);
    encoder.debugfs_entry = core::ptr::null_mut();
    }
