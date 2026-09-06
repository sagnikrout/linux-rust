//! Automatically rewritten from C to Rust
//! Source: kernel/crash_dump_dm_crypt.c
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

macro_rules! EXPORT_SYMBOL { ($($tt:tt)*) => {}; }
macro_rules! EXPORT_SYMBOL_GPL { ($($tt:tt)*) => {}; }
macro_rules! MODULE_LICENSE { ($($tt:tt)*) => {}; }
macro_rules! MODULE_AUTHOR { ($($tt:tt)*) => {}; }
macro_rules! MODULE_DESCRIPTION { ($($tt:tt)*) => {}; }
macro_rules! MODULE_ALIAS { ($($tt:tt)*) => {}; }
macro_rules! module_init { ($($tt:tt)*) => {}; }
macro_rules! module_exit { ($($tt:tt)*) => {}; }
macro_rules! early_initcall { ($($tt:tt)*) => {}; }
macro_rules! core_initcall { ($($tt:tt)*) => {}; }
macro_rules! postcore_initcall { ($($tt:tt)*) => {}; }
macro_rules! arch_initcall { ($($tt:tt)*) => {}; }
macro_rules! subsys_initcall { ($($tt:tt)*) => {}; }
macro_rules! fs_initcall { ($($tt:tt)*) => {}; }
macro_rules! device_initcall { ($($tt:tt)*) => {}; }
macro_rules! late_initcall { ($($tt:tt)*) => {}; }
macro_rules! __setup { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_MUTEX { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_SPINLOCK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DECLARE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DEFINE { ($($tt:tt)*) => {}; }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seq_file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cred { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inode { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct notifier_block { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw_notifier_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_dir_entry { pub _opaque: [u8; 0] }

pub type pid_type = c_int;
pub type cpu_pm_event = c_int;
pub type spinlock_t = u32;
pub type raw_spinlock_t = u32;
pub type kernel_cap_t = u64;
pub type cap_user_header_t = *mut c_void;
pub type cap_user_data_t = *mut c_void;
pub type async_cookie_t = u64;
pub type atomic_long_t = core::sync::atomic::AtomicI64;


























// SPDX-License-Identifier: GPL-2.0-only

    static unsigned int key_count;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_crypt_key {
    pub key_size: c_uint,
    pub key_desc: [c_char; KEY_DESC_MAX_LEN],
    pub data: [u8; KEY_SIZE_MAX],
}

    static struct keys_header {
    let mut total_keys = 0;
    struct dm_crypt_key keys[] __counted_by(total_keys);
    } *keys_header;
#[no_mangle]
unsafe extern "C" fn get_keys_header_size(total_keys: usize) -> usize {
    return struct_size(keys_header, keys, total_keys);
    }
    unsigned long long dm_crypt_keys_addr;
// EXPORT_SYMBOL_GPL;
#[no_mangle]
unsafe extern "C" fn setup_dmcryptkeys(arg: *mut c_char) -> c_int {
    let mut end = core::ptr::null_mut();
    if (!arg) {
    return -EINVAL;
    }
    dm_crypt_keys_addr = memparse(arg, &end);
    if (end > arg) {
    return 0;
    }
    dm_crypt_keys_addr = 0;
    return -EINVAL;
    }
    early_param("dmcryptkeys", setup_dmcryptkeys);
//
// Architectures may override this function to read dm crypt keys
//
#[no_mangle]
pub unsafe extern "C" fn dm_crypt_keys_read(buf: *mut c_char, count: usize, ppos: *mut u64) -> ssize_t __weak {
pub static mut kvec: kvec = { .iov_base = buf, .iov_len = count };
    let mut iter;
    iov_iter_kvec(&iter, READ, &kvec, 1, count);
    return read_from_oldmem(&iter, count, ppos, cc_platform_has(CC_ATTR_MEM_ENCRYPT));
    }
#[no_mangle]
pub unsafe extern "C" fn add_key_to_keyring() {
    let mut key_ref;
    let mut r = 0;
// create or update the requested key and add it to the target keyring
    key_ref = key_create_or_update(keyring_ref, "user", dm_key.key_desc,
    dm_key.data, dm_key.key_size,
    KEY_USR_ALL, KEY_ALLOC_IN_QUOTA);
    if (!IS_ERR(key_ref)) {
    r = key_ref_to_ptr(key_ref).serial;
    key_ref_put(key_ref);
    kexec_dprintk("Success adding key %s", dm_key.key_desc);
    } else {
    r = PTR_ERR(key_ref);
    kexec_dprintk("Error when adding key");
    }
    return r;
    }
#[no_mangle]
unsafe extern "C" fn get_keys_from_kdump_reserved_memory() {
    let mut keys_header_loaded = core::ptr::null_mut();
    arch_kexec_unprotect_crashkres();
    keys_header_loaded = kmap_local_page(pfn_to_page(
    kexec_crash_image.dm_crypt_keys_addr >> PAGE_SHIFT));
    memcpy(keys_header, keys_header_loaded, get_keys_header_size(key_count));
    kunmap_local(keys_header_loaded);
    arch_kexec_protect_crashkres();
    }
#[no_mangle]
unsafe extern "C" fn restore_dm_crypt_keys_to_thread_keyring() -> c_int {
    let mut key = core::ptr::null_mut();
    let mut keys_header_size = 0;
    let mut keyring_ref;
pub static mut ret: c_int = 0;
    let mut addr = 0;
// find the target keyring (which must be writable)
    keyring_ref =
    lookup_user_key(KEY_SPEC_USER_KEYRING, 0x01, KEY_NEED_WRITE);
    if (IS_ERR(keyring_ref)) {
    kexec_dprintk("Failed to get the user keyring\n");
    return PTR_ERR(keyring_ref);
    }
    addr = dm_crypt_keys_addr;
    dm_crypt_keys_read(&key_count, sizeof(key_count), &addr);
    if (key_count > KEY_NUM_MAX) {
    kexec_dprintk("Failed to read the number of dm-crypt keys\n");
    ret = -1;
    goto out;
    }
    kexec_dprintk("There are %u keys\n", key_count);
    addr = dm_crypt_keys_addr;
    keys_header_size = get_keys_header_size(key_count);
    keys_header = kzalloc(keys_header_size, GFP_KERNEL);
    if (!keys_header) {
    ret = -ENOMEM;
    goto out;
    }
    dm_crypt_keys_read(keys_header, keys_header_size, &addr);
    for (int i = 0; i < keys_header.total_keys; i++) {
    key = &keys_header.keys[i];
    kexec_dprintk("Get key (size=%u)\n", key.key_size);
    add_key_to_keyring(key, keyring_ref);
    }
    out:
    key_ref_put(keyring_ref);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn read_key_from_user_keyring(dm_key: *mut dm_crypt_key) -> c_int {
    let mut ukp = core::ptr::null_mut();
    let mut key = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    kexec_dprintk("Requesting logon key %s", dm_key.key_desc);
    key = request_key(&key_type_logon, dm_key.key_desc, core::ptr::null_mut());
    if (IS_ERR(key)) {
    pr_warn("No such logon key %s\n", dm_key.key_desc);
    return PTR_ERR(key);
    }
    down_read(&key.sem);
    ukp = user_key_payload_locked(key);
    if (!ukp) {
    ret = -EKEYREVOKED;
    goto out;
    }
    if (ukp.datalen > KEY_SIZE_MAX) {
    pr_err("Key size %u exceeds maximum (%u)\n", ukp.datalen, KEY_SIZE_MAX);
    ret = -EINVAL;
    goto out;
    }
    memcpy(dm_key.data, ukp.data, ukp.datalen);
    dm_key.key_size = ukp.datalen;
    kexec_dprintk("Get dm crypt key (size=%u) %s\n", dm_key.key_size,
    dm_key.key_desc);
    out:
    up_read(&key.sem);
    key_put(key);
    return ret;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct config_key {
    pub item: config_item,
    pub description: *const c_char,
}

#[no_mangle]
pub unsafe extern "C" fn to_config_key() {
    return container_of(item, struct config_key, item);
    }
#[no_mangle]
unsafe extern "C" fn config_key_description_show(item: *mut config_item, page: *mut c_char) -> isize {
    return sysfs_emit(page, "%s\n", to_config_key(item).description);
    }
#[no_mangle]
pub unsafe extern "C" fn config_key_description_store() {
    struct config_key *config_key = to_config_key(item);
    let mut len = 0;
    let mut ret = 0;
    ret = -EINVAL;
    len = strcspn(page, "\n");
    if (len > KEY_DESC_MAX_LEN) {
    pr_err("The key description shouldn't exceed %u characters", KEY_DESC_MAX_LEN);
    return ret;
    }
    if (!len) {
    return ret;
    }
    kfree(config_key.description);
    ret = -ENOMEM;
    config_key.description = kmemdup_nul(page, len, GFP_KERNEL);
    if (!config_key.description) {
    return ret;
    }
    return count;
    }
// CONFIGFS_ATTR;
    static struct configfs_attribute *config_key_attrs[] = {
    &config_key_attr_description,
    core::ptr::null_mut(),
    };
#[no_mangle]
unsafe extern "C" fn config_key_release(item: *mut config_item) {
    kfree(to_config_key(item));
    key_count--;
    }
pub static mut configfs_item_operations: usize = 0;
pub static mut config_item_type: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn config_keys_make_item() {
    let mut config_key = core::ptr::null_mut();
    if (key_count > KEY_NUM_MAX) {
    pr_err("Only %u keys at maximum to be created\n", KEY_NUM_MAX);
    return ERR_PTR(-EINVAL);
    }
    config_key = kzalloc_obj(struct config_key);
    if (!config_key) {
    return ERR_PTR(-ENOMEM);
    }
    config_item_init_type_name(&config_key.item, name, &config_key_type);
    key_count++;
    return &config_key.item;
    }
#[no_mangle]
unsafe extern "C" fn config_keys_count_show(item: *mut config_item, page: *mut c_char) -> isize {
    return sysfs_emit(page, "%d\n", key_count);
    }
// CONFIGFS_ATTR_RO;
    static bool is_dm_key_reused;
#[no_mangle]
unsafe extern "C" fn config_keys_reuse_show(item: *mut config_item, page: *mut c_char) -> isize {
    return sysfs_emit(page, "%d\n", is_dm_key_reused);
    }
#[no_mangle]
pub unsafe extern "C" fn config_keys_reuse_store() {
    if (!kexec_crash_image || !kexec_crash_image.dm_crypt_keys_addr) {
    kexec_dprintk(
    "dm-crypt keys haven't be saved to crash-reserved memory\n");
    return -EINVAL;
    }
    if (kstrtobool(page, &is_dm_key_reused)) {
    return -EINVAL;
    }
    if (is_dm_key_reused) {
    get_keys_from_kdump_reserved_memory();
    }
    return count;
    }
// CONFIGFS_ATTR;
    static struct configfs_attribute *config_keys_attrs[] = {
    &config_keys_attr_count,
    &config_keys_attr_reuse,
    core::ptr::null_mut(),
    };
//
// Note that, since no extra work is required on ->drop_item(),
// no ->drop_item() is provided.
//
pub static mut configfs_group_operations: usize = 0;
pub static mut config_item_type: usize = 0;
    static bool restore;
#[no_mangle]
unsafe extern "C" fn config_keys_restore_show(item: *mut config_item, page: *mut c_char) -> isize {
    return sysfs_emit(page, "%d\n", restore);
    }
#[no_mangle]
pub unsafe extern "C" fn config_keys_restore_store() {
    if (!restore) {
    restore_dm_crypt_keys_to_thread_keyring();
    }
    if (kstrtobool(page, &restore)) {
    return -EINVAL;
    }
    return count;
    }
// CONFIGFS_ATTR;
    static struct configfs_attribute *kdump_config_keys_attrs[] = {
    &config_keys_attr_restore,
    core::ptr::null_mut(),
    };
pub static mut config_item_type: usize = 0;
pub static mut configfs_subsystem: usize = 0;
#[no_mangle]
unsafe extern "C" fn build_keys_header() -> c_int {
    struct config_item *item = core::ptr::null_mut();
    let mut key = core::ptr::null_mut();
    int i, r;
    if (keys_header != core::ptr::null_mut()) {
    kvfree(keys_header);
    }
    keys_header = kzalloc(get_keys_header_size(key_count), GFP_KERNEL);
    if (!keys_header) {
    return -ENOMEM;
    }
    keys_header.total_keys = key_count;
    i = 0;
    list_for_each_entry(item, &config_keys_subsys.su_group.cg_children,
    ci_entry) {
    if (item.ci_type != &config_key_type) {
    continue;
    }
    key = to_config_key(item);
    if (!key.description) {
    pr_warn("No key description for key %s\n", item.ci_name);
    return -EINVAL;
    }
    strscpy(keys_header.keys[i].key_desc, key.description,
    KEY_DESC_MAX_LEN);
    r = read_key_from_user_keyring(&keys_header.keys[i]);
    if (r != 0) {
    kexec_dprintk("Failed to read key %s\n",
    keys_header.keys[i].key_desc);
    return r;
    }
    i++;
    kexec_dprintk("Found key: %s\n", item.ci_name);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn crash_load_dm_crypt_keys(image: *mut kimage) -> c_int {
    struct kexec_buf kbuf = {
    .image = image,
    .buf_min = 0,
    .buf_max = ULONG_MAX,
    .top_down = false,
    .random = true,
    };
    let mut r = 0;
    if (key_count <= 0) {
    kexec_dprintk("No dm-crypt keys\n");
    return 0;
    }
    if (!is_dm_key_reused) {
    image.dm_crypt_keys_addr = 0;
    r = build_keys_header();
    if (r) {
    pr_err("Failed to build dm-crypt keys header, ret=%d\n", r);
    return r;
    }
    }
    kbuf.buffer = keys_header;
    kbuf.bufsz = get_keys_header_size(key_count);
    kbuf.memsz = kbuf.bufsz;
    kbuf.buf_align = ELF_CORE_HEADER_ALIGN;
    kbuf.mem = KEXEC_BUF_MEM_UNKNOWN;
    r = kexec_add_buffer(&kbuf);
    if (r) {
    pr_err("Failed to call kexec_add_buffer, ret=%d\n", r);
    kvfree(kbuf.buffer);
    return r;
    }
    image.dm_crypt_keys_addr = kbuf.mem;
    image.dm_crypt_keys_sz = kbuf.bufsz;
    kexec_dprintk(
    "Loaded dm crypt keys to kexec_buffer bufsz=0x%lx memsz=0x%lx\n",
    kbuf.bufsz, kbuf.memsz);
    return r;
    }
#[no_mangle]
unsafe extern "C" fn configfs_dmcrypt_keys_init() -> c_int {
    let mut ret = 0;
    if (is_kdump_kernel()) {
    config_keys_subsys.su_group.cg_item.ci_type =
    &kdump_config_keys_type;
    }
    config_group_init(&config_keys_subsys.su_group);
    mutex_init(&config_keys_subsys.su_mutex);
    ret = configfs_register_subsystem(&config_keys_subsys);
    if (ret) {
    pr_err("Error %d while registering subsystem %s\n", ret,
    config_keys_subsys.su_group.cg_item.ci_namebuf);
    goto out_unregister;
    }
    return 0;
    out_unregister:
    configfs_unregister_subsystem(&config_keys_subsys);
    return ret;
    }
// module_init;