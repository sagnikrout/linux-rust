//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/filesystems/fuse/write_extend_eof_test.c
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
// Regression test for the fuse write-extend partial-EOF-page zeroing bug.
//
// A buffered write that extends i_size past a non-page-aligned EOF must zero
// the tail of the old last page.  If an application has mmap'd that page and
// stored into the post-EOF region (undefined until the file grows), the
// now-in-bounds tail must read back as zero, not as the stale stored bytes.
//
// The bug is exposed on a non-writeback_cache server that keeps the page cache
// across the write (FOPEN_KEEP_CACHE without FOPEN_DIRECT_IO).  This test is a
// raw /dev/fuse server in that mode; the backing data is always zero in the
// hole, so any non-zero byte a read sees is stale page-cache data.
//
// Requires root to mount fuse.
//
// Macro flag: #define _GNU_SOURCE

pub const FUSE_ROOT_ID: c_int = 1;
pub const FILE_INO: c_int = 2;

pub const POLLUTE: c_uint = 0xee;
// Server-side state, shared with the responder thread.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct server {
    pub fd: c_int,
    pub /: *mut *mut unsigned char backing[BACKING_SIZE]; / authoritative bytes,
    pub size: u64,
}

#[no_mangle]
unsafe extern "C" fn reply(fd: c_int, unique: u64, error: c_int, data: *mut c_void, len: usize) {
    static void reply(int fd, uint64_t unique, int error, void *data, size_t len)
    {
    struct fuse_out_header oh = {
    .len = sizeof(oh) + (data ? len : 0),
    .error = error,
    .unique = unique,
    };
    struct iovec iov[2] = { { &oh, sizeof(oh) }, { data, len } };
// Errors here are teardown races (device closed on unmount); ignore.
    if (writev(fd, iov, data ? 2 : 1) < 0)
    return;
    }
    static void fill_attr(struct fuse_attr *a, uint64_t ino, uint32_t mode,
    uint64_t size)
    {
    memset(a, 0, sizeof(*a));
    a.ino = ino;
    a.mode = mode;
    a.nlink = 1;
    a.size = size;
    a.blksize = sysconf(_SC_PAGESIZE);
    }
    static void *server_thread(void *arg)
    {
    struct server *s = arg;
    static char buf[MAX_WRITE + 4096];
    for (;;) {
    let mut n: isize = read(s.fd, buf, sizeof(buf));
    struct fuse_in_header *ih = (void *)buf;
    if (n < 0) {
    if (errno == EINTR || errno == EAGAIN)
    continue;
    return core::ptr::null_mut();	/* device closed on unmount */
    }
    if (n < (ssize_t)sizeof(*ih))
    continue;
    switch (ih.opcode) {
    case FUSE_INIT: {
    struct fuse_init_in *in = (void *)(ih + 1);
    let mut out: fuse_init_out = {0};
// No FUSE_WRITEBACK_CACHE: the exposed configuration.
    out.major = FUSE_KERNEL_VERSION;
    out.minor = FUSE_KERNEL_MINOR_VERSION;
    out.max_readahead = in.max_readahead;
    out.max_write = MAX_WRITE;
    out.max_background = 16;
    out.congestion_threshold = 12;
    out.flags = FUSE_MAX_PAGES;
    out.max_pages = MAX_WRITE / sysconf(_SC_PAGESIZE);
    reply(s.fd, ih.unique, 0, &out, sizeof(out));
    break;
    }
    case FUSE_GETATTR: {
    let mut out: fuse_attr_out = {0};
    let mut root: c_int = ih.nodeid == FUSE_ROOT_ID;
    out.attr_valid = 3600;
    fill_attr(&out.attr, ih.nodeid,
    root ? (S_IFDIR | 0755) : (S_IFREG | 0644),
    root ? 0 : s.size);
    reply(s.fd, ih.unique, 0, &out, sizeof(out));
    break;
    }
    case FUSE_LOOKUP: {
    let mut out: fuse_entry_out = {0};
    out.nodeid = FILE_INO;
    out.attr_valid = 3600;
    out.entry_valid = 3600;
    fill_attr(&out.attr, FILE_INO, S_IFREG | 0644, s.size);
    reply(s.fd, ih.unique, 0, &out, sizeof(out));
    break;
    }
    case FUSE_OPEN:
    case FUSE_OPENDIR: {
    let mut out: fuse_open_out = {0};
// Keep the cache across the write, but not direct I/O.
    out.open_flags = FOPEN_KEEP_CACHE;
    reply(s.fd, ih.unique, 0, &out, sizeof(out));
    break;
    }
    case FUSE_READ: {
    struct fuse_read_in *in = (void *)(ih + 1);
    let mut off: u64 = in.offset;
    let mut size: u32 = in.size;
    if (off >= BACKING_SIZE)
    size = 0;
#[no_mangle]
pub unsafe extern "C" fn if(BACKING_SIZE: off + size >) -> else {
    else if (off + size > BACKING_SIZE)
    size = BACKING_SIZE - off;
    reply(s.fd, ih.unique, 0, s.backing + off, size);
    break;
    }
    case FUSE_WRITE: {
    struct fuse_write_in *in = (void *)(ih + 1);
    let mut out: fuse_write_out = {0};
    let mut off: u64 = in.offset;
    let mut size: u32 = in.size;
    if (off < BACKING_SIZE) {
    let mut c: u32 = size;
    if (off + c > BACKING_SIZE)
    c = BACKING_SIZE - off;
    memcpy(s.backing + off, in + 1, c);
    if (off + c > s.size)
    s.size = off + c;
    }
    out.size = size;
    reply(s.fd, ih.unique, 0, &out, sizeof(out));
    break;
    }
    case FUSE_SETATTR: {
    struct fuse_setattr_in *in = (void *)(ih + 1);
    let mut out: fuse_attr_out = {0};
    if ((in.valid & FATTR_SIZE) && in.size <= BACKING_SIZE) {
    if (in.size > s.size)
    memset(s.backing + s.size, 0,
    in.size - s.size);
    s.size = in.size;
    }
    out.attr_valid = 3600;
    fill_attr(&out.attr, ih.nodeid, S_IFREG | 0644, s.size);
    reply(s.fd, ih.unique, 0, &out, sizeof(out));
    break;
    }
    case FUSE_FALLOCATE: {
    struct fuse_fallocate_in *in = (void *)(ih + 1);
    let mut end: u64 = in.offset + in.length;
// Only plain (size-extending) fallocate is used here.
    if (!(in.mode & FALLOC_FL_KEEP_SIZE) &&
    end <= BACKING_SIZE && end > s.size) {
    memset(s.backing + s.size, 0, end - s.size);
    s.size = end;
    }
    reply(s.fd, ih.unique, 0, core::ptr::null_mut(), 0);
    break;
    }
    case FUSE_FLUSH:
    case FUSE_RELEASE:
    case FUSE_RELEASEDIR:
    case FUSE_FSYNC:
    case FUSE_ACCESS:
    reply(s.fd, ih.unique, 0, core::ptr::null_mut(), 0);
    break;
    case FUSE_FORGET:
    break;
    default:
    reply(s.fd, ih.unique, -EOPNOTSUPP, core::ptr::null_mut(), 0);
    break;
    }
    }
    }
    FIXTURE(fuse)
    {
    struct server *srv;
    pthread_t thread;
    char dir[64];
    long page;		/* runtime page size */
    off_t eof;		/* mid-page EOF, page-relative */
    int fd;			/* open test file */
    char *map;		/* mmap of the EOF page */
    int mounted;
    };
    FIXTURE_SETUP(fuse)
    {
    char opts[128];
    pthread_t t;
    if (geteuid() != 0)
    SKIP(return, "need root to mount fuse");
    self.page = sysconf(_SC_PAGESIZE);
    self.fd = -1;
    self.map = MAP_FAILED;
    self.srv = mmap(core::ptr::null_mut(), sizeof(*self.srv), PROT_READ | PROT_WRITE,
    MAP_SHARED | MAP_ANONYMOUS, -1, 0);
    ASSERT_NE(MAP_FAILED, self.srv);
    self.srv.fd = open("/dev/fuse", O_RDWR);
    ASSERT_GE(self.srv.fd, 0);
    strcpy(self.dir, "/tmp/fuse_weof_XXXXXX");
    ASSERT_NE(core::ptr::null_mut(), mkdtemp(self.dir));
    snprintf(opts, sizeof(opts),
    "fd=%d,rootmode=40000,user_id=0,group_id=0",
    self.srv.fd);
    ASSERT_EQ(0, mount("fuse", self.dir, "fuse", 0, opts));
    self.mounted = 1;
    ASSERT_EQ(0, pthread_create(&t, core::ptr::null_mut(), server_thread, self.srv));
    self.thread = t;
    }
    FIXTURE_TEARDOWN(fuse)
    {
    if (self.map != MAP_FAILED)
    munmap(self.map, self.page);
    if (self.fd >= 0)
    close(self.fd);
    if (self.mounted)
    umount2(self.dir, MNT_DETACH);
    if (self.srv && self.srv != MAP_FAILED) {
    if (self.srv.fd > 0)
    close(self.srv.fd);
    munmap(self.srv, sizeof(*self.srv));
    }
    if (self.dir[0])
    rmdir(self.dir);
    }
//
// Create the test file with a mid-page EOF and mmap-store POLLUTE into its
// post-EOF tail (a legal store, undefined until the file grows).  Leaves the
// file open and the EOF page mapped in the fixture for the caller to extend.
//
    static void pollute_eof_tail(struct __test_metadata *_metadata,
    FIXTURE_DATA(fuse) * self)
    {
    let mut eof: off_t = 2 * self.page + self.page / 4;
    char path[128];
    char *buf;
    snprintf(path, sizeof(path), "%s/file", self.dir);
    self.fd = open(path, O_RDWR | O_CREAT | O_TRUNC, 0644);
    ASSERT_GE(self.fd, 0);
    self.eof = eof;
    buf = malloc(eof);
    ASSERT_NE(core::ptr::null_mut(), buf);
    memset(buf, 'A', eof);
    ASSERT_EQ(eof, pwrite(self.fd, buf, eof, 0));
    free(buf);
    self.map = mmap(core::ptr::null_mut(), self.page, PROT_READ | PROT_WRITE, MAP_SHARED,
    self.fd, eof & ~(self.page - 1));
    ASSERT_NE(MAP_FAILED, self.map);
    memset(self.map + (eof & (self.page - 1)), POLLUTE,
    self.page - (eof & (self.page - 1)));
    }
// Assert the old post-EOF tail [eof, end of its page) now reads back as zero.
    static void assert_tail_zeroed(struct __test_metadata *_metadata,
    FIXTURE_DATA(fuse) * self)
    {
    let mut base: off_t = self.eof & ~(self.page - 1);
    char *tail = malloc(self.page);
    int i;
    ASSERT_NE(core::ptr::null_mut(), tail);
    ASSERT_EQ(self.page, pread(self.fd, tail, self.page, base));
    for (i = self.eof & (self.page - 1); i < self.page; i++)
    ASSERT_EQ(0, tail[i]);
    free(tail);
    }
// Basic: pollute the post-EOF tail, extend past it by a later write.
    TEST_F(fuse, write_extend)
    {
    pollute_eof_tail(_metadata, self);
    ASSERT_EQ(4, pwrite(self.fd, "data", 4, 5 * self.page + self.page / 3));
    assert_tail_zeroed(_metadata, self);
    }
// Extend via ftruncate() rather than a write.
    TEST_F(fuse, ftruncate_extend)
    {
    pollute_eof_tail(_metadata, self);
    ASSERT_EQ(0, ftruncate(self.fd, 8 * self.page));
    assert_tail_zeroed(_metadata, self);
    }
// Extend via fallocate() starting at the old EOF.
    TEST_F(fuse, fallocate_extend)
    {
    pollute_eof_tail(_metadata, self);
    ASSERT_EQ(0, fallocate(self.fd, 0, self.eof, 4 * self.page));
    assert_tail_zeroed(_metadata, self);
    }
// A write landing inside the old EOF page must not clobber its own data.
    TEST_F(fuse, extend_into_eof_page_preserves_data)
    {
    off_t base, wr;
    char *buf, *rd;
    int i;
    pollute_eof_tail(_metadata, self);
    base = self.eof & ~(self.page - 1);
    wr = base + 3 * self.page / 4;		/* starts in the EOF page */
    buf = malloc(2 * self.page);
    ASSERT_NE(core::ptr::null_mut(), buf);
    memset(buf, 'B', 2 * self.page);
    ASSERT_EQ(2 * self.page, pwrite(self.fd, buf, 2 * self.page, wr));
    free(buf);
    rd = malloc(self.page);
    ASSERT_NE(core::ptr::null_mut(), rd);
    ASSERT_EQ(self.page, pread(self.fd, rd, self.page, base));
// [eof, wr) is hole -> zero; [wr, page) is written data -> 'B'.
    for (i = self.eof & (self.page - 1); i < wr - base; i++)
    ASSERT_EQ(0, rd[i]);
    for (i = wr - base; i < self.page; i++)
    ASSERT_EQ('B', rd[i]);
    free(rd);
    }
    TEST_HARNESS_MAIN
