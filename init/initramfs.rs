//! Automatically rewritten from C to Rust
//! Source: init/initramfs.c
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

    static __initdata bool csum_present;
    static __initdata u32 io_csum;
    static ssize_t __init xwrite(struct file *file, const unsigned char *p,
    size_t count, loff_t *pos)
    {
    let mut out: isize = 0;
// sys_write only can write MAX_RW_COUNT aka 2G-4K bytes at most
    while (count) {
    let mut rv: isize = kernel_write(file, p, count, pos);
    if (rv < 0) {
    if (rv == -EINTR || rv == -EAGAIN)
    continue;
    return out ? out : rv;
    } else if (rv == 0)
    break;
    if (csum_present) {
    ssize_t i;
    for (i = 0; i < rv; i++)
    io_csum += p[i];
    }
    p += rv;
    out += rv;
    count -= rv;
    }
    return out;
    }
    static __initdata char *message;
#[no_mangle]
unsafe extern "C" fn error(x: *mut c_char) -> void __init {
    static void __init error(char *x)
    {
    if (!message)
    message = x;
    }

    ({ show_mem(); panic(fmt, ##__VA_ARGS__); })
// link hash

    static __initdata struct hash {
    int ino, minor, major;
    umode_t mode;
    struct hash *next;
    char name[N_ALIGN(PATH_MAX)];
    } *head[32];
    static __initdata bool hardlink_seen;
#[no_mangle]
pub unsafe extern "C" fn hash(major: c_int, minor: c_int, ino: c_int) -> c_int {
    static inline int hash(int major, int minor, int ino)
    {
    let mut tmp: c_ulong = ino + minor + (major << 3);
    tmp += tmp >> 5;
    return tmp & 31;
    }
    static char __init *find_link(int major, int minor, int ino,
    umode_t mode, char *name)
    {
    struct hash **p, *q;
    for (p = head + hash(major, minor, ino); *p; p = &(*p).next) {
    if ((*p).ino != ino)
    continue;
    if ((*p).minor != minor)
    continue;
    if ((*p).major != major)
    continue;
    if (((*p).mode ^ mode) & S_IFMT)
    continue;
    return (*p).name;
    }
    q = kmalloc_obj(struct hash);
    if (!q)
    panic_show_mem("can't allocate link hash entry");
    q.major = major;
    q.minor = minor;
    q.ino = ino;
    q.mode = mode;
    strscpy(q.name, name);
    q.next = core::ptr::null_mut();
// p = q;
    hardlink_seen = true;
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn free_hash() -> void __init {
    static void __init free_hash(void)
    {
    struct hash **p, *q;
    for (p = head; hardlink_seen && p < head + 32; p++) {
    while (*p) {
    q = *p;
// p = q->next;
    kfree(q);
    }
    }
    hardlink_seen = false;
    }

#[no_mangle]
unsafe extern "C" fn do_utime(filename: *mut c_char, mtime: time64_t) -> void __init {
    static void __init do_utime(char *filename, time64_t mtime)
    {
    struct timespec64 t[2] = { { .tv_sec = mtime }, { .tv_sec = mtime } };
    init_utimes(filename, t);
    }
#[no_mangle]
unsafe extern "C" fn do_utime_path(path: *const path, mtime: time64_t) -> void __init {
    static void __init do_utime_path(const struct path *path, time64_t mtime)
    {
    struct timespec64 t[2] = { { .tv_sec = mtime }, { .tv_sec = mtime } };
    vfs_utimes(path, t);
    }
    static __initdata LIST_HEAD(dir_list);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dir_entry {
    pub list: list_head,
    pub mtime: time64_t,
    pub name: [c_char; ],
}

#[no_mangle]
unsafe extern "C" fn dir_add(name: *const c_char, nlen: usize, mtime: time64_t) -> void __init {
    static void __init dir_add(const char *name, size_t nlen, time64_t mtime)
    {
    struct dir_entry *de;
    de = kmalloc_flex(*de, name, nlen);
    if (!de)
    panic_show_mem("can't allocate dir_entry buffer");
    INIT_LIST_HEAD(&de.list);
    strscpy(de.name, name, nlen);
    de.mtime = mtime;
    list_add(&de.list, &dir_list);
    }
#[no_mangle]
unsafe extern "C" fn dir_utime() -> void __init {
    static void __init dir_utime(void)
    {
    struct dir_entry *de, *tmp;
    list_for_each_entry_safe(de, tmp, &dir_list, list) {
    list_del(&de.list);
    do_utime(de.name, de.mtime);
    kfree(de);
    }
    }

    static void __init do_utime(char *filename, time64_t mtime) {}
    static void __init do_utime_path(const struct path *path, time64_t mtime) {}
    static void __init dir_add(const char *name, size_t nlen, time64_t mtime) {}
    static void __init dir_utime(void) {}

    static __initdata time64_t mtime;
// cpio header parsing
    static __initdata unsigned long ino, major, minor, nlink;
    static __initdata umode_t mode;
    static __initdata unsigned long body_len, name_len;
    static __initdata uid_t uid;
    static __initdata gid_t gid;
    static __initdata unsigned rdev;
    static __initdata u32 hdr_csum;
#[no_mangle]
unsafe extern "C" fn parse_header(s: *mut c_char) -> int __init {
    static int __init parse_header(char *s)
    {
    __be32 header[13];
    int ret;
    ret = hex2bin((u8 *)header, s + 6, sizeof(header));
    if (ret) {
    error("damaged header");
    return ret;
    }
    ino = be32_to_cpu(header[0]);
    mode = be32_to_cpu(header[1]);
    uid = be32_to_cpu(header[2]);
    gid = be32_to_cpu(header[3]);
    nlink = be32_to_cpu(header[4]);
    mtime = be32_to_cpu(header[5]); /* breaks in y2106 */
    body_len = be32_to_cpu(header[6]);
    major = be32_to_cpu(header[7]);
    minor = be32_to_cpu(header[8]);
    rdev = new_encode_dev(MKDEV(be32_to_cpu(header[9]), be32_to_cpu(header[10])));
    name_len = be32_to_cpu(header[11]);
    hdr_csum = be32_to_cpu(header[12]);
    return 0;
    }
// Finite-state machine
    static __initdata enum state {
    Start,
    Collect,
    GotHeader,
    SkipIt,
    GotName,
    CopyFile,
    GotSymlink,
    Reset
    } state, next_state;
    static __initdata char *victim;
    static unsigned long byte_count __initdata;
    static __initdata loff_t this_header, next_header;
#[no_mangle]
pub unsafe extern "C" fn eat(n: unsigned) -> void __init {
    static inline void __init eat(unsigned n)
    {
    victim += n;
    this_header += n;
    byte_count -= n;
    }
    static __initdata char *collected;
    static long remains __initdata;
    static __initdata char *collect;
#[no_mangle]
unsafe extern "C" fn read_into(buf: *mut c_char, size: unsigned, next: enum state) -> void __init {
    static void __init read_into(char *buf, unsigned size, enum state next)
    {
    if (byte_count >= size) {
    collected = victim;
    eat(size);
    state = next;
    } else {
    collect = collected = buf;
    remains = size;
    next_state = next;
    state = Collect;
    }
    }
    static __initdata char *header_buf, *symlink_buf, *name_buf;
#[no_mangle]
unsafe extern "C" fn do_start() -> int __init {
    static int __init do_start(void)
    {
    read_into(header_buf, CPIO_HDRLEN, GotHeader);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn do_collect() -> int __init {
    static int __init do_collect(void)
    {
    let mut n: c_ulong = remains;
    if (byte_count < n)
    n = byte_count;
    memcpy(collect, victim, n);
    eat(n);
    collect += n;
    if ((remains -= n) != 0)
    return 1;
    state = next_state;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn do_header() -> int __init {
    static int __init do_header(void)
    {
    if (!memcmp(collected, "070701", 6)) {
    csum_present = false;
    } else if (!memcmp(collected, "070702", 6)) {
    csum_present = true;
    } else {
    if (memcmp(collected, "070707", 6) == 0)
    error("incorrect cpio method used: use -H newc option");
    else
    error("no cpio magic");
    return 1;
    }
    if (parse_header(collected))
    return 1;
    next_header = this_header + N_ALIGN(name_len) + body_len;
    next_header = (next_header + 3) & ~3;
    state = SkipIt;
    if (name_len <= 0 || name_len > PATH_MAX)
    return 0;
    if (S_ISLNK(mode)) {
    if (body_len > PATH_MAX)
    return 0;
    collect = collected = symlink_buf;
    remains = N_ALIGN(name_len) + body_len;
    next_state = GotSymlink;
    state = Collect;
    return 0;
    }
    if (S_ISREG(mode) || !body_len)
    read_into(name_buf, N_ALIGN(name_len), GotName);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn do_skip() -> int __init {
    static int __init do_skip(void)
    {
    if (this_header + byte_count < next_header) {
    eat(byte_count);
    return 1;
    } else {
    eat(next_header - this_header);
    state = next_state;
    return 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn do_reset() -> int __init {
    static int __init do_reset(void)
    {
    while (byte_count && *victim == '\0')
    eat(1);
    if (byte_count && (this_header & 3))
    error("broken padding");
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn clean_path(path: *mut c_char, fmode: umode_t) -> void __init {
    static void __init clean_path(char *path, umode_t fmode)
    {
    struct kstat st;
    if (!init_stat(path, &st, AT_SYMLINK_NOFOLLOW) &&
    (st.mode ^ fmode) & S_IFMT) {
    if (S_ISDIR(st.mode))
    init_rmdir(path);
    else
    init_unlink(path);
    }
    }
#[no_mangle]
unsafe extern "C" fn maybe_link() -> int __init {
    static int __init maybe_link(void)
    {
    if (nlink >= 2) {
    char *old = find_link(major, minor, ino, mode, collected);
    if (old) {
    clean_path(collected, 0);
    return (init_link(old, collected) < 0) ? -1 : 1;
    }
    }
    return 0;
    }
    static __initdata struct file *wfile;
    static __initdata loff_t wfile_pos;
#[no_mangle]
unsafe extern "C" fn do_name() -> int __init {
    static int __init do_name(void)
    {
    state = SkipIt;
    next_state = Reset;
// name_len > 0 && name_len <= PATH_MAX checked in do_header
    if (collected[name_len - 1] != '\0') {
    pr_err("initramfs name without nulterm: %.*s\n",
    (int)name_len, collected);
    error("malformed archive");
    return 1;
    }
    if (strcmp(collected, "TRAILER!!!") == 0) {
    free_hash();
    return 0;
    }
    clean_path(collected, mode);
    if (S_ISREG(mode)) {
    let mut ml: c_int = maybe_link();
    if (ml >= 0) {
    let mut openflags: c_int = O_WRONLY|O_CREAT|O_LARGEFILE;
    if (ml != 1)
    openflags |= O_TRUNC;
    wfile = filp_open(collected, openflags, mode);
    if (IS_ERR(wfile))
    return 0;
    wfile_pos = 0;
    io_csum = 0;
    vfs_fchown(wfile, uid, gid);
    vfs_fchmod(wfile, mode);
    if (body_len)
    vfs_truncate(&wfile.f_path, body_len);
    state = CopyFile;
    }
    } else if (S_ISDIR(mode)) {
    init_mkdir(collected, mode);
    init_chown(collected, uid, gid, 0);
    init_chmod(collected, mode);
    dir_add(collected, name_len, mtime);
    } else if (S_ISBLK(mode) || S_ISCHR(mode) ||
    S_ISFIFO(mode) || S_ISSOCK(mode)) {
    if (maybe_link() == 0) {
    init_mknod(collected, mode, rdev);
    init_chown(collected, uid, gid, 0);
    init_chmod(collected, mode);
    do_utime(collected, mtime);
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn do_copy() -> int __init {
    static int __init do_copy(void)
    {
    if (byte_count >= body_len) {
    if (xwrite(wfile, victim, body_len, &wfile_pos) != body_len)
    error("write error");
    do_utime_path(&wfile.f_path, mtime);
    fput(wfile);
    if (csum_present && io_csum != hdr_csum)
    error("bad data checksum");
    eat(body_len);
    state = SkipIt;
    return 0;
    } else {
    if (xwrite(wfile, victim, byte_count, &wfile_pos) != byte_count)
    error("write error");
    body_len -= byte_count;
    eat(byte_count);
    return 1;
    }
    }
#[no_mangle]
unsafe extern "C" fn do_symlink() -> int __init {
    static int __init do_symlink(void)
    {
    if (collected[name_len - 1] != '\0') {
    pr_err("initramfs symlink without nulterm: %.*s\n",
    (int)name_len, collected);
    error("malformed archive");
    return 1;
    }
    collected[N_ALIGN(name_len) + body_len] = '\0';
    clean_path(collected, 0);
    init_symlink(collected + N_ALIGN(name_len), collected);
    init_chown(collected, uid, gid, AT_SYMLINK_NOFOLLOW);
    do_utime(collected, mtime);
    state = SkipIt;
    next_state = Reset;
    return 0;
    }
    static __initdata int (*actions[])(void) = {
    [Start]		= do_start,
    [Collect]	= do_collect,
    [GotHeader]	= do_header,
    [SkipIt]	= do_skip,
    [GotName]	= do_name,
    [CopyFile]	= do_copy,
    [GotSymlink]	= do_symlink,
    [Reset]		= do_reset,
    };
#[no_mangle]
unsafe extern "C" fn write_buffer(buf: *mut c_char, len: c_ulong) -> long __init {
    static long __init write_buffer(char *buf, unsigned long len)
    {
    byte_count = len;
    victim = buf;
    while (!actions[state]())
    ;
    return len - byte_count;
    }
#[no_mangle]
unsafe extern "C" fn flush_buffer(bufv: *mut c_void, len: c_ulong) -> long __init {
    static long __init flush_buffer(void *bufv, unsigned long len)
    {
    char *buf = bufv;
    long written;
    let mut origLen: c_long = len;
    if (message)
    return -1;
    while ((written = write_buffer(buf, len)) < len && !message) {
    let mut c: c_char = buf[written];
    if (c == '0') {
    buf += written;
    len -= written;
    state = Start;
    } else if (c == 0) {
    buf += written;
    len -= written;
    state = Reset;
    } else
    error("junk within compressed archive");
    }
    return origLen;
    }
    static unsigned long my_inptr __initdata; /* index of next byte to be processed in inbuf */

//
// unpack_to_rootfs - decompress and extract an initramfs archive
// @buf: input initramfs archive to extract
// @len: length of initramfs data to process
//
// Returns: NULL for success or an error message string
//
// This symbol shouldn't be used externally. It's available for unit tests.
//
#[no_mangle]
pub unsafe extern "C" fn unpack_to_rootfs(buf: *mut c_char, len: c_ulong) -> *mut char  __init {
    char * __init unpack_to_rootfs(char *buf, unsigned long len)
    {
    long written;
    decompress_fn decompress;
    const char *compress_name;
    struct {
    char header[CPIO_HDRLEN];
    char symlink[PATH_MAX + N_ALIGN(PATH_MAX) + 1];
    char name[N_ALIGN(PATH_MAX)];
    } *bufs = kmalloc_obj(*bufs);
    if (!bufs)
    panic_show_mem("can't allocate buffers");
    header_buf = bufs.header;
    symlink_buf = bufs.symlink;
    name_buf = bufs.name;
    state = Start;
    this_header = 0;
    message = core::ptr::null_mut();
    while (!message && len) {
    let mut saved_offset: loff_t = this_header;
    if (*buf == '0' && !(this_header & 3)) {
    state = Start;
    written = write_buffer(buf, len);
    buf += written;
    len -= written;
    continue;
    }
    if (!*buf) {
    buf++;
    len--;
    this_header++;
    continue;
    }
    this_header = 0;
    decompress = decompress_method(buf, len, &compress_name);
    pr_debug("Detected %s compressed data\n", compress_name);
    if (decompress) {
    int res = decompress(buf, len, core::ptr::null_mut(), flush_buffer, core::ptr::null_mut(),
    &my_inptr, error);
    if (res)
    error("decompressor failed");
    } else if (compress_name) {
    pr_err("compression method %s not configured\n",
    compress_name);
    error("decompressor failed");
    } else
    error("invalid magic at start of compressed archive");
    if (state != Reset)
    error("junk at the end of compressed archive");
    this_header = saved_offset + my_inptr;
    buf += my_inptr;
    len -= my_inptr;
    }
    dir_utime();
// free any hardlink state collected without optional TRAILER!!!
    free_hash();
    kfree(bufs);
    return message;
    }
    static int __initdata do_retain_initrd;
#[no_mangle]
unsafe extern "C" fn retain_initrd_param(str: *mut c_char) -> int __init {
    static int __init retain_initrd_param(char *str)
    {
    if (*str)
    return 0;
    do_retain_initrd = 1;
    return 1;
    }
    __setup("retain_initrd", retain_initrd_param);

#[no_mangle]
unsafe extern "C" fn keepinitrd_setup(__unused: *mut c_char) -> int __init {
    static int __init keepinitrd_setup(char *__unused)
    {
    do_retain_initrd = 1;
    return 1;
    }
    __setup("keepinitrd", keepinitrd_setup);

    let mut initramfs_async: static bool __initdata = true;
#[no_mangle]
unsafe extern "C" fn initramfs_async_setup(str: *mut c_char) -> int __init {
    static int __init initramfs_async_setup(char *str)
    {
    return kstrtobool(str, &initramfs_async) == 0;
    }
    __setup("initramfs_async=", initramfs_async_setup);
    extern char __initramfs_start[];
    extern unsigned long __initramfs_size;

    static BIN_ATTR(initrd, 0440, sysfs_bin_attr_simple_read, core::ptr::null_mut(), 0);
#[no_mangle]
pub unsafe extern "C" fn reserve_initrd_mem() -> void __init {
    void __init reserve_initrd_mem(void)
    {
    phys_addr_t start;
    unsigned long size;
// Ignore the virtual address computed during device tree parsing
    initrd_start = initrd_end = 0;
    if (!phys_initrd_size)
    return;
//
// Round the memory region to page boundaries as per free_initrd_mem()
// This allows us to detect whether the pages overlapping the initrd
// are in use, but more importantly, reserves the entire set of pages
// as we don't want these pages allocated for other purposes.
//
    start = round_down(phys_initrd_start, PAGE_SIZE);
    size = phys_initrd_size + (phys_initrd_start - start);
    size = round_up(size, PAGE_SIZE);
    if (!memblock_is_region_memory(start, size)) {
    pr_err("INITRD: 0x%08llx+0x%08lx is not a memory region",
    (u64)start, size);
    goto disable;
    }
    if (memblock_is_region_reserved(start, size)) {
    pr_err("INITRD: 0x%08llx+0x%08lx overlaps in-use memory region\n",
    (u64)start, size);
    goto disable;
    }
    memblock_reserve(start, size);
// Now convert initrd to virtual addresses
    initrd_start = (unsigned long)__va(phys_initrd_start);
    initrd_end = initrd_start + phys_initrd_size;
    initrd_below_start_ok = 1;
    return;
    disable:
    pr_cont(" - disabling initrd\n");
    initrd_start = 0;
    initrd_end = 0;
    }
#[no_mangle]
pub unsafe extern "C" fn free_initrd_mem(start: c_ulong, end: c_ulong) -> void __weak __init {
    void __weak __init free_initrd_mem(unsigned long start, unsigned long end)
    {
    free_reserved_area((void *)start, (void *)end, POISON_FREE_INITMEM,
    "initrd");
    }

#[no_mangle]
unsafe extern "C" fn kexec_free_initrd() -> bool __init {
    static bool __init kexec_free_initrd(void)
    {
    let mut crashk_start: c_ulong = (unsigned long)__va(crashk_res.start);
    let mut crashk_end: c_ulong = (unsigned long)__va(crashk_res.end);
//
// If the initrd region is overlapped with crashkernel reserved region,
// free only memory that is not part of crashkernel region.
//
    if (initrd_start >= crashk_end || initrd_end <= crashk_start)
    return false;
//
// Initialize initrd memory region since the kexec boot does not do.
//
    memset((void *)initrd_start, 0, initrd_end - initrd_start);
    if (initrd_start < crashk_start)
    free_initrd_mem(initrd_start, crashk_start);
    if (initrd_end > crashk_end)
    free_initrd_mem(crashk_end, initrd_end);
    return true;
    }

#[no_mangle]
pub unsafe extern "C" fn kexec_free_initrd() -> bool {
    static inline bool kexec_free_initrd(void)
    {
    return false;
    }

#[no_mangle]
unsafe extern "C" fn populate_initrd_image(err: *mut c_char) -> void __init {
    static void __init populate_initrd_image(char *err)
    {
    ssize_t written;
    struct file *file;
    let mut pos: loff_t = 0;
    printk(KERN_INFO "rootfs image is not initramfs (%s); looks like an initrd\n",
    err);
    file = filp_open("/initrd.image", O_WRONLY|O_CREAT|O_LARGEFILE, 0700);
    if (IS_ERR(file))
    return;
    written = xwrite(file, (char *)initrd_start, initrd_end - initrd_start,
    &pos);
    if (written != initrd_end - initrd_start)
    pr_err("/initrd.image: incomplete write (%zd != %ld)\n",
    written, initrd_end - initrd_start);
    fput(file);
    }

#[no_mangle]
unsafe extern "C" fn unpack_initramfs(cookie: async_cookie_t) -> void __init {
    static void __init unpack_initramfs(async_cookie_t cookie)
    {
// Load the built in initramfs
    char *err = unpack_to_rootfs(__initramfs_start, __initramfs_size);
    if (err)
    panic_show_mem("%s", err); /* Failed to decompress INTERNAL initramfs */
    if (!initrd_start || IS_ENABLED(CONFIG_INITRAMFS_FORCE))
    return;
    if (IS_ENABLED(CONFIG_BLK_DEV_RAM))
    printk(KERN_INFO "Trying to unpack rootfs image as initramfs...\n");
    else
    printk(KERN_INFO "Unpacking initramfs...\n");
    err = unpack_to_rootfs((char *)initrd_start, initrd_end - initrd_start);
    if (err) {

    populate_initrd_image(err);

    printk(KERN_EMERG "Initramfs unpacking failed: %s\n", err);

    }
    }
#[no_mangle]
unsafe extern "C" fn do_populate_rootfs(unused: *mut c_void, cookie: async_cookie_t) -> void __init {
    static void __init do_populate_rootfs(void *unused, async_cookie_t cookie)
    {
    scoped_with_init_fs() {
    unpack_initramfs(cookie);
    security_initramfs_populated();
    }
//
// If the initrd region is overlapped with crashkernel reserved region,
// free only memory that is not part of crashkernel region.
//
    if (!do_retain_initrd && initrd_start && !kexec_free_initrd()) {
    free_initrd_mem(initrd_start, initrd_end);
    } else if (do_retain_initrd && initrd_start) {
    bin_attr_initrd.size = initrd_end - initrd_start;
    bin_attr_initrd.private = (void *)initrd_start;
    if (sysfs_create_bin_file(firmware_kobj, &bin_attr_initrd))
    pr_err("Failed to create initrd sysfs file");
    }
    initrd_start = 0;
    initrd_end = 0;
    init_flush_fput();
    }
    static ASYNC_DOMAIN_EXCLUSIVE(initramfs_domain);
    static async_cookie_t initramfs_cookie;
#[no_mangle]
pub unsafe extern "C" fn wait_for_initramfs() {
    void wait_for_initramfs(void)
    {
    if (!initramfs_cookie) {
//
// Something before rootfs_initcall wants to access
// the filesystem/initramfs. Probably a bug. Make a
// note, avoid deadlocking the machine, and let the
// caller's access fail as it used to.
//
    pr_warn_once("wait_for_initramfs() called before rootfs_initcalls\n");
    return;
    }
    async_synchronize_cookie_domain(initramfs_cookie + 1, &initramfs_domain);
    }
    EXPORT_SYMBOL_GPL(wait_for_initramfs);
#[no_mangle]
unsafe extern "C" fn populate_rootfs() -> int __init {
    static int __init populate_rootfs(void)
    {
    initramfs_cookie = async_schedule_domain(do_populate_rootfs, core::ptr::null_mut(),
    &initramfs_domain);
    usermodehelper_enable();
    if (!initramfs_async)
    wait_for_initramfs();
    return 0;
    }
    rootfs_initcall(populate_rootfs);
