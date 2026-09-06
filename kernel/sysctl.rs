//! Automatically rewritten from C to Rust
//! Source: kernel/sysctl.c
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



// SPDX-License-Identifier: GPL-2.0-only
//
// sysctl.c: General linux system control interface
//

// shared constants to be used in various sysctls
    const int sysctl_vals[] = { 0, 1, 2, 3, 4, 100, 200, 1000, 3000, INT_MAX, 65535, -1 };
    EXPORT_SYMBOL(sysctl_vals);
    const unsigned long sysctl_long_vals[] = { 0, 1, LONG_MAX };
    EXPORT_SYMBOL_GPL(sysctl_long_vals);

// Constants used for minimum and maximum
    let mut ngroups_max: static int = NGROUPS_MAX;
    let mut cap_last_cap: static int = CAP_LAST_CAP;

//
// enum sysctl_writes_mode - supported sysctl write modes
//
// @SYSCTL_WRITES_LEGACY: each write syscall must fully contain the sysctl value
// to be written, and multiple writes on the same sysctl file descriptor
// will rewrite the sysctl value, regardless of file position. No warning
// is issued when the initial position is not 0.
// @SYSCTL_WRITES_WARN: same as above but warn when the initial file position is
// not 0.
// @SYSCTL_WRITES_STRICT: writes to numeric sysctl entries must always be at
// file position 0 and the value must be fully contained in the buffer
// sent to the write syscall. If dealing with strings respect the file
// position, but restrict this to the max length of the buffer, anything
// passed the max length will be ignored. Multiple writes will append
// to the buffer.
//
// These write modes control how current file position affects the behavior of
// updating internal kernel (SYSCTL_USER_TO_KERN) sysctl values through the proc
// interface on each write.
//
    enum sysctl_writes_mode {
    SYSCTL_WRITES_LEGACY		= -1,
    SYSCTL_WRITES_WARN		= 0,
    SYSCTL_WRITES_STRICT		= 1,
    };
    let mut sysctl_writes_strict: static enum sysctl_writes_mode = SYSCTL_WRITES_STRICT;

//
// /proc/sys support
//

    static int _proc_do_string(char *data, int maxlen, int dir,
    char *buffer, size_t *lenp, loff_t *ppos)
    {
    size_t len;
    char c, *p;
    if (!data || !maxlen || !*lenp) {
// lenp = 0;
    return 0;
    }
    if (SYSCTL_USER_TO_KERN(dir)) {
    if (sysctl_writes_strict == SYSCTL_WRITES_STRICT) {
// Only continue writes not past the end of buffer.
    len = strlen(data);
    if (len > maxlen - 1)
    len = maxlen - 1;
    if (*ppos > len)
    return 0;
    len = *ppos;
    } else {
// Start writing from beginning of buffer.
    len = 0;
    }
// ppos += *lenp;
    p = buffer;
    while ((p - buffer) < *lenp && len < maxlen - 1) {
    c = *(p++);
    if (c == 0 || c == '\n')
    break;
    data[len++] = c;
    }
    data[len] = 0;
    } else {
    len = strlen(data);
    if (len > maxlen)
    len = maxlen;
    if (*ppos > len) {
// lenp = 0;
    return 0;
    }
    data += *ppos;
    len  -= *ppos;
    if (len > *lenp)
    len = *lenp;
    if (len)
    memcpy(buffer, data, len);
    if (len < *lenp) {
    buffer[len] = '\n';
    len++;
    }
// lenp = len;
// ppos += len;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn warn_sysctl_write(table: *const ctl_table) {
    pr_warn_once("%s wrote to %s when file position was not 0!\n"
    "This will not be supported in the future. To silence this\n"
    "warning, set kernel.sysctl_writes_strict = -1\n",
    current.comm, table.procname);
    }
//
// proc_first_pos_non_zero_ignore - check if first position is allowed
// @ppos: file position
// @table: the sysctl table
//
// Returns: true if the first position is non-zero and the sysctl_writes_strict
// mode indicates this is not allowed for numeric input types. String proc
// handlers can ignore the return value.
//
    static bool proc_first_pos_non_zero_ignore(loff_t *ppos,
    const struct ctl_table *table)
    {
    if (!*ppos)
    return false;
    switch (sysctl_writes_strict) {
    case SYSCTL_WRITES_STRICT:
    return true;
    case SYSCTL_WRITES_WARN:
    warn_sysctl_write(table);
    return false;
    default:
    return false;
    }
    }
//
// proc_dostring - read a string sysctl
// @table: the sysctl table
// @dir: %TRUE if this is a write to the sysctl file
// @buffer: the user buffer
// @lenp: the size of the user buffer
// @ppos: file position
//
// Reads/writes a string from/to the user buffer. If the kernel
// buffer provided is not large enough to hold the string, the
// string is truncated. The copied string is %NULL-terminated.
// If the string is being read by the user process, it is copied
// and a newline '\n' is added. It is truncated if the buffer is
// not large enough.
//
// Returns: %0 on success.
//
    int proc_dostring(const struct ctl_table *table, int dir,
    void *buffer, size_t *lenp, loff_t *ppos)
    {
    if (SYSCTL_USER_TO_KERN(dir))
    proc_first_pos_non_zero_ignore(ppos, table);
    return _proc_do_string(table.data, table.maxlen, dir, buffer, lenp,
    ppos);
    }
#[no_mangle]
unsafe extern "C" fn proc_skip_spaces(buf: *mut c_char, size: *mut usize) {
    while (*size) {
    if (!isspace(**buf))
    break;
    (*size)--;
    (*buf)++;
    }
    }
#[no_mangle]
unsafe extern "C" fn proc_skip_char(buf: *mut c_char, size: *mut usize, v: c_char) {
    while (*size) {
    if (**buf != v)
    break;
    (*size)--;
    (*buf)++;
    }
    }
//
// strtoul_lenient - parse an ASCII formatted integer from a buffer and only
// fail on overflow
//
// @cp: kernel buffer containing the string to parse
// @endp: pointer to store the trailing characters
// @base: the base to use
// @res: where the parsed integer will be stored
//
// This function will fail the parse on overflow. If there wasn't an overflow
// the function will defer the decision what characters count as invalid to the
// caller.
//
// Returns:
// * %0 on success and @res will contain the parsed integer,
// @endp will hold any trailing characters.
// * %-ERANGE on overflow.
//
    static int strtoul_lenient(const char *cp, char **endp, unsigned int base,
    unsigned long *res)
    {
    unsigned long long result;
    unsigned int rv;
    cp = _parse_integer_fixup_radix(cp, &base);
    rv = _parse_integer(cp, base, &result);
    if ((rv & KSTRTOX_OVERFLOW) || (result != (unsigned long)result))
    return -ERANGE;
    cp += rv;
    if (endp)
// endp = (char *)cp;
// res = (unsigned long)result;
    return 0;
    }
pub const TMPBUFLEN: c_int = 22;
//
// proc_get_long - reads an ASCII formatted integer from a user buffer
//
// @buf: a kernel buffer
// @size: size of the kernel buffer
// @val: this is where the number will be stored
// @neg: set to %TRUE if number is negative
// @perm_tr: a vector which contains the allowed trailers
// @perm_tr_len: size of the perm_tr vector
// @tr: pointer to store the trailer character
//
// Returns:
// * %0 on success and @buf and @size are updated with
// the amount of bytes read. If @tr is non-NULL and a trailing
// character exists (size is non-zero after returning from this
// function), @tr is updated with the trailing character.
// * %-EINVAL on failure.
//
    static int proc_get_long(char **buf, size_t *size,
    unsigned long *val, bool *neg,
    const char *perm_tr, unsigned perm_tr_len, char *tr)
    {
    char *p, tmp[TMPBUFLEN];
    let mut len: isize = *size;
    if (len <= 0)
    return -EINVAL;
    if (len > TMPBUFLEN - 1)
    len = TMPBUFLEN - 1;
    memcpy(tmp, *buf, len);
    tmp[len] = 0;
    p = tmp;
    if (*p == '-' && *size > 1) {
// neg = true;
    p++;
    } else
// neg = false;
    if (!isdigit(*p))
    return -EINVAL;
    if (strtoul_lenient(p, &p, 0, val))
    return -EINVAL;
    len = p - tmp;
// We don't know if the next char is whitespace thus we may accept
// invalid integers (e.g. 1234...a) or two integers instead of one
// (e.g. 123...1). So lets not allow such large numbers.
    if (len == TMPBUFLEN - 1)
    return -EINVAL;
    if (len < *size && perm_tr_len && !memchr(perm_tr, *p, perm_tr_len))
    return -EINVAL;
    if (tr && (len < *size))
// tr = *p;
// buf += len;
// size -= len;
    return 0;
    }
//
// proc_put_long - converts an integer to a decimal ASCII formatted string
//
// @buf: the user buffer
// @size: the size of the user buffer
// @val: the integer to be converted
// @neg: sign of the number, %TRUE for negative
//
// In case of success @buf and @size are updated with the amount of bytes
// written.
//
#[no_mangle]
unsafe extern "C" fn proc_put_long(buf: *mut c_void, size: *mut usize, val: c_ulong, neg: bool) {
    int len;
    char tmp[TMPBUFLEN], *p = tmp;
    sprintf(p, "%s%lu", neg ? "-" : "", val);
    len = strlen(tmp);
    if (len > *size)
    len = *size;
    memcpy(*buf, tmp, len);
// size -= len;
// buf += len;
    }

#[no_mangle]
unsafe extern "C" fn proc_put_char(buf: *mut c_void, size: *mut usize, c: c_char) {
    if (*size) {
    char **buffer = (char **)buf;
// buffer = c;
    (*size)--;
    (*buffer)++;
// buf = *buffer;
    }
    }
//
// proc_uint_u2k_conv_uop - Assign user value to a kernel pointer
//
// @u_ptr: pointer to user space variable
// @k_ptr: pointer to kernel variable
// @u_ptr_op: execute this function before assigning to k_ptr
//
// Uses WRITE_ONCE to assign value to k_ptr. Executes u_ptr_op if
// not NULL. Check that the values are less than UINT_MAX to avoid
// having to support wrap around from userspace.
//
// Returns: %0 on success.
//
    int proc_uint_u2k_conv_uop(const ulong *u_ptr, uint *k_ptr,
    ulong (*u_ptr_op)(const ulong))
    {
    let mut u: c_ulong = u_ptr_op ? u_ptr_op(*u_ptr) : *u_ptr;
    if (u > UINT_MAX)
    return -EINVAL;
    WRITE_ONCE(*k_ptr, u);
    return 0;
    }
//
// proc_uint_k2u_conv - Assign kernel value to a user space pointer
//
// @u_ptr: pointer to user space variable
// @k_ptr: pointer to kernel variable
//
// Uses READ_ONCE to assign value to u_ptr.
//
// Returns: %0 on success.
//
#[no_mangle]
pub unsafe extern "C" fn proc_uint_k2u_conv(u_ptr: *mut c_ulong, k_ptr: *const c_uint) -> c_int {
    let mut val: c_uint = READ_ONCE(*k_ptr);
// u_ptr = (ulong)val;
    return 0;
    }
//
// proc_uint_conv - Change user or kernel pointer based on direction
//
// @u_ptr: pointer to user variable
// @k_ptr: pointer to kernel variable
// @dir: %TRUE if this is a write to the sysctl file
// @tbl: the sysctl table
// @k_ptr_range_check: Check range for k_ptr when %TRUE
// @user_to_kern: Callback used to assign value from user to kernel var
// @kern_to_user: Callback used to assign value from kernel to user var
//
// When direction is kernel to user, then the u_ptr is modified.
// When direction is user to kernel, then the k_ptr is modified.
//
// Returns: %0 on success
//
    int proc_uint_conv(ulong *u_ptr, uint *k_ptr, int dir,
    const struct ctl_table *tbl, bool k_ptr_range_check,
    int (*user_to_kern)(const ulong *u_ptr, uint *k_ptr),
    int (*kern_to_user)(ulong *u_ptr, const uint *k_ptr))
    {
    if (SYSCTL_KERN_TO_USER(dir))
    return kern_to_user(u_ptr, k_ptr);
    if (k_ptr_range_check) {
    uint tmp_k;
    int ret;
    if (!tbl)
    return -EINVAL;
    ret = user_to_kern(u_ptr, &tmp_k);
    if (ret)
    return ret;
    if ((tbl.extra1 &&
// (uint *)tbl->extra1 > tmp_k) ||
    (tbl.extra2 &&
// (uint *)tbl->extra2 < tmp_k))
    return -ERANGE;
    WRITE_ONCE(*k_ptr, tmp_k);
    } else
    return user_to_kern(u_ptr, k_ptr);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn proc_uint_u2k_conv(u_ptr: *const c_ulong, k_ptr: *mut c_uint) -> c_int {
    return proc_uint_u2k_conv_uop(u_ptr, k_ptr, core::ptr::null_mut());
    }
    static int do_proc_uint_conv(bool *negp, ulong *u_ptr, uint *k_ptr, int dir,
    const struct ctl_table *tbl)
    {
    return proc_uint_conv(u_ptr, k_ptr, dir, tbl, false,
    proc_uint_u2k_conv, proc_uint_k2u_conv);
    }
    static int do_proc_uint_conv_minmax(bool *negp, ulong *u_ptr, uint *k_ptr,
    int dir, const struct ctl_table *tbl)
    {
    return proc_uint_conv(u_ptr, k_ptr, dir, tbl, true,
    proc_uint_u2k_conv, proc_uint_k2u_conv);
    }
//
// proc_int_k2u_conv_kop - Assign kernel value to a user space pointer
// @u_ptr: pointer to user space variable
// @k_ptr: pointer to kernel variable
// @negp: assigned %TRUE if the converted kernel value is negative;
// %FALSE otherweise
// @k_ptr_op: execute this function before assigning to u_ptr
//
// Uses READ_ONCE to get value from k_ptr. Executes k_ptr_op before assigning
// to u_ptr if not NULL. Does **not** check for overflow.
//
// Returns: 0 on success.
//
    int proc_int_k2u_conv_kop(ulong *u_ptr, const int *k_ptr, bool *negp,
    ulong (*k_ptr_op)(const ulong))
    {
    let mut val: c_int = READ_ONCE(*k_ptr);
    if (val < 0) {
// negp = true;
// u_ptr = k_ptr_op ? -k_ptr_op((ulong)val) : -(ulong)val;
    } else {
// negp = false;
// u_ptr = k_ptr_op ? k_ptr_op((ulong)val) : (ulong) val;
    }
    return 0;
    }
//
// proc_int_u2k_conv_uop - Assign user value to a kernel pointer
// @u_ptr: pointer to user space variable
// @k_ptr: pointer to kernel variable
// @negp: If %TRUE, the converted user value is made negative.
// @u_ptr_op: execute this function before assigning to k_ptr
//
// Uses WRITE_ONCE to assign value to k_ptr. Executes u_ptr_op if
// not NULL. Check for overflow with UINT_MAX.
//
// Returns: 0 on success.
//
    int proc_int_u2k_conv_uop(const ulong *u_ptr, int *k_ptr, const bool *negp,
    ulong (*u_ptr_op)(const ulong))
    {
    let mut u: c_ulong = u_ptr_op ? u_ptr_op(*u_ptr) : *u_ptr;
    if (*negp) {
    if (u > (ulong) INT_MAX + 1)
    return -EINVAL;
    WRITE_ONCE(*k_ptr, -u);
    } else {
    if (u > (ulong) INT_MAX)
    return -EINVAL;
    WRITE_ONCE(*k_ptr, u);
    }
    return 0;
    }
//
// proc_int_conv - Change user or kernel pointer based on direction
//
// @negp: will be passed to uni-directional converters
// @u_ptr: pointer to user variable
// @k_ptr: pointer to kernel variable
// @dir: %TRUE if this is a write to the sysctl file
// @tbl: the sysctl table
// @k_ptr_range_check: Check range for k_ptr when %TRUE
// @user_to_kern: Callback used to assign value from user to kernel var
// @kern_to_user: Callback used to assign value from kernel to user var
//
// When direction is kernel to user, then the u_ptr is modified.
// When direction is user to kernel, then the k_ptr is modified.
//
// Returns: 0 on success
//
    int proc_int_conv(bool *negp, ulong *u_ptr, int *k_ptr, int dir,
    const struct ctl_table *tbl, bool k_ptr_range_check,
    int (*user_to_kern)(const bool *negp, const ulong *u_ptr, int *k_ptr),
    int (*kern_to_user)(bool *negp, ulong *u_ptr, const int *k_ptr))
    {
    if (SYSCTL_KERN_TO_USER(dir))
    return kern_to_user(negp, u_ptr, k_ptr);
    if (k_ptr_range_check) {
    int tmp_k, ret;
    if (!tbl)
    return -EINVAL;
    ret = user_to_kern(negp, u_ptr, &tmp_k);
    if (ret)
    return ret;
    if ((tbl.extra1 && *(int *)tbl.extra1 > tmp_k) ||
    (tbl.extra2 && *(int *)tbl.extra2 < tmp_k))
    return -EINVAL;
    WRITE_ONCE(*k_ptr, tmp_k);
    } else
    return user_to_kern(negp, u_ptr, k_ptr);
    return 0;
    }
    static int sysctl_user_to_kern_int_conv(const bool *negp, const ulong *u_ptr,
    int *k_ptr)
    {
    return proc_int_u2k_conv_uop(u_ptr, k_ptr, negp, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn sysctl_kern_to_user_int_conv(negp: *mut bool, u_ptr: *mut c_ulong, k_ptr: *const c_int) -> c_int {
    return proc_int_k2u_conv_kop(u_ptr, k_ptr, negp, core::ptr::null_mut());
    }
    static int do_proc_int_conv(bool *negp, unsigned long *u_ptr, int *k_ptr,
    int dir, const struct ctl_table *tbl)
    {
    return proc_int_conv(negp, u_ptr, k_ptr, dir, tbl, false,
    sysctl_user_to_kern_int_conv,
    sysctl_kern_to_user_int_conv);
    }
    static int do_proc_int_conv_minmax(bool *negp, unsigned long *u_ptr, int *k_ptr,
    int dir, const struct ctl_table *tbl)
    {
    return proc_int_conv(negp, u_ptr, k_ptr, dir, tbl, true,
    sysctl_user_to_kern_int_conv,
    sysctl_kern_to_user_int_conv);
    }
    static const char proc_wspace_sep[] = { ' ', '\t', '\n' };
//
// Element type processed by do_proc_vec(). The tag selects the element size
// and signedness, and it selects which member of union proc_vec_conv is live.
//
    enum proc_vec_type {
    PROC_VEC_INT,
    PROC_VEC_UINT,
    PROC_VEC_ULONG,
    };
//
// Converter passed to do_proc_vec(). Only the member matching the
// enum proc_vec_type tag is ever read, so every dispatch stays fully typed and
// no void * converter pointer is needed.
//
    union proc_vec_conv {
    int (*int_conv)(bool *negp, ulong *u_ptr, int *k_ptr,
    int dir, const struct ctl_table *table);
    int (*uint_conv)(bool *negp, ulong *u_ptr, uint *k_ptr,
    int dir, const struct ctl_table *table);
    int (*ulong_conv)(bool *negp, ulong *u_ptr, ulong *k_ptr,
    int dir, const struct ctl_table *table);
    };
//
// Dispatch to the converter member selected by @type. @k_ptr walks
// table->data as raw bytes and is cast back to the element type here.
//
    static int proc_vec_conv(enum proc_vec_type type, union proc_vec_conv conv,
    bool *negp, ulong *u_ptr, char *k_ptr, int dir,
    const struct ctl_table *table)
    {
    switch (type) {
    case PROC_VEC_INT:
    return conv.int_conv(negp, u_ptr, (int *)k_ptr, dir, table);
    case PROC_VEC_UINT:
    return conv.uint_conv(negp, u_ptr, (uint *)k_ptr, dir, table);
    case PROC_VEC_ULONG:
    return conv.ulong_conv(negp, u_ptr, (ulong *)k_ptr, dir, table);
    }
    return -EINVAL;
    }
//
// Read/write a vector of @type elements. The element size and signedness are
// derived from @type, so a single runtime function replaces the per-type
// variants. table->data is walked as raw bytes (@i) advanced by @size; the
// converter performs the actual typed load/store.
//
    static int do_proc_vec(const struct ctl_table *table, int dir,
    void *buffer, size_t *lenp, loff_t *ppos,
    enum proc_vec_type type, union proc_vec_conv conv)
    {
    int vleft, first = 1, err = 0;
    size_t left, size;
    bool is_unsigned;
    char *i, *p;
    switch (type) {
    case PROC_VEC_INT:
    size = sizeof(int);
    is_unsigned = false;
    break;
    case PROC_VEC_UINT:
    size = sizeof(uint);
    is_unsigned = true;
    break;
    case PROC_VEC_ULONG:
    size = sizeof(ulong);
    is_unsigned = true;
    break;
    default:
    return -EINVAL;
    }
    if (!table.data || !table.maxlen || !*lenp ||
    (*ppos && SYSCTL_KERN_TO_USER(dir))) {
// lenp = 0;
    return 0;
    }
    i = table.data;
    vleft = table.maxlen / size;
    left = *lenp;
// uint arrays are not supported, *Do not* add support for them.
    if (type == PROC_VEC_UINT && vleft != 1)
    return -EINVAL;
    if (SYSCTL_USER_TO_KERN(dir)) {
    if (proc_first_pos_non_zero_ignore(ppos, table))
    goto out;
    if (left > PAGE_SIZE - 1)
    left = PAGE_SIZE - 1;
    p = buffer;
    }
    for (; left && vleft--; i += size, first = 0) {
    unsigned long lval;
    let mut neg: bool = false;
    if (SYSCTL_USER_TO_KERN(dir)) {
    proc_skip_spaces(&p, &left);
    if (!left)
    break;
    err = proc_get_long(&p, &left, &lval, &neg,
    proc_wspace_sep,
    sizeof(proc_wspace_sep), core::ptr::null_mut());
    if (!err && neg && is_unsigned)
    err = -EINVAL;
    if (err)
    break;
    if (proc_vec_conv(type, conv, &neg, &lval, i, dir, table)) {
    err = -EINVAL;
    break;
    }
    } else {
    if (proc_vec_conv(type, conv, &neg, &lval, i, dir, table)) {
    err = -EINVAL;
    break;
    }
    if (!first)
    proc_put_char(&buffer, &left, '\t');
    proc_put_long(&buffer, &left, lval, neg);
    }
    }
    if (SYSCTL_KERN_TO_USER(dir) && !first && left && !err)
    proc_put_char(&buffer, &left, '\n');
    if (SYSCTL_USER_TO_KERN(dir) && !err && left)
    proc_skip_spaces(&p, &left);
    if (SYSCTL_USER_TO_KERN(dir) && first)
    return err ? : -EINVAL;
// lenp -= left;
    out:
// ppos += *lenp;
    return err;
    }
//
// proc_douintvec_conv - read a vector of unsigned ints with a custom converter
//
// @table: the sysctl table
// @dir: %TRUE if this is a write to the sysctl file
// @buffer: the user buffer
// @lenp: the size of the user buffer
// @ppos: file position
// @conv: Custom converter call back
//
// Reads/writes up to table->maxlen/sizeof(unsigned int) unsigned integer
// values from/to the user buffer, treated as an ASCII string. Negative
// strings are not allowed.
//
// Returns: %0 on success
//
    int proc_douintvec_conv(const struct ctl_table *table, int dir, void *buffer,
    size_t *lenp, loff_t *ppos,
    int (*conv)(bool *negp, ulong *u_ptr, uint *k_ptr,
    int dir, const struct ctl_table *table))
    {
    if (!conv)
    conv = do_proc_uint_conv;
    return do_proc_vec(table, dir, buffer, lenp, ppos, PROC_VEC_UINT,
    (union proc_vec_conv){ .uint_conv = conv });
    }
//
// proc_dobool - read/write a bool
// @table: the sysctl table
// @dir: %TRUE if this is a write to the sysctl file
// @buffer: the user buffer
// @lenp: the size of the user buffer
// @ppos: file position
//
// Reads/writes one integer value from/to the user buffer,
// treated as an ASCII string.
//
// table->data must point to a bool variable and table->maxlen must
// be sizeof(bool).
//
// Returns: %0 on success.
//
    int proc_dobool(const struct ctl_table *table, int dir, void *buffer,
    size_t *lenp, loff_t *ppos)
    {
    struct ctl_table tmp;
    bool *data = table.data;
    int res, val;
// Do not support arrays yet.
    if (table.maxlen != sizeof(bool))
    return -EINVAL;
    tmp = *table;
    tmp.maxlen = sizeof(val);
    tmp.data = &val;
    val = READ_ONCE(*data);
    res = proc_dointvec(&tmp, dir, buffer, lenp, ppos);
    if (res)
    return res;
    if (SYSCTL_USER_TO_KERN(dir))
    WRITE_ONCE(*data, val);
    return 0;
    }
//
// proc_dointvec - read a vector of integers
// @table: the sysctl table
// @dir: %TRUE if this is a write to the sysctl file
// @buffer: the user buffer
// @lenp: the size of the user buffer
// @ppos: file position
//
// Reads/writes up to table->maxlen/sizeof(unsigned int) integer
// values from/to the user buffer, treated as an ASCII string.
//
// Returns: %0 on success.
//
    int proc_dointvec(const struct ctl_table *table, int dir, void *buffer,
    size_t *lenp, loff_t *ppos)
    {
    return do_proc_vec(table, dir, buffer, lenp, ppos, PROC_VEC_INT,
    (union proc_vec_conv){ .int_conv = do_proc_int_conv });
    }
//
// proc_douintvec - read a vector of unsigned integers
// @table: the sysctl table
// @dir: %TRUE if this is a write to the sysctl file
// @buffer: the user buffer
// @lenp: the size of the user buffer
// @ppos: file position
//
// Reads/writes up to table->maxlen/sizeof(unsigned int) unsigned integer
// values from/to the user buffer, treated as an ASCII string.
//
// Returns: %0 on success.
//
    int proc_douintvec(const struct ctl_table *table, int dir, void *buffer,
    size_t *lenp, loff_t *ppos)
    {
    return do_proc_vec(table, dir, buffer, lenp, ppos, PROC_VEC_UINT,
    (union proc_vec_conv){ .uint_conv = do_proc_uint_conv });
    }
//
// proc_dointvec_minmax - read a vector of integers with min/max values
// @table: the sysctl table
// @dir: %TRUE if this is a write to the sysctl file
// @buffer: the user buffer
// @lenp: the size of the user buffer
// @ppos: file position
//
// Reads/writes up to table->maxlen/sizeof(unsigned int) integer
// values from/to the user buffer, treated as an ASCII string.
//
// This routine will ensure the values are within the range specified by
// table->extra1 (min) and table->extra2 (max).
//
// Returns: %0 on success or -EINVAL when the range check fails and
// SYSCTL_USER_TO_KERN(dir) == true
//
    int proc_dointvec_minmax(const struct ctl_table *table, int dir,
    void *buffer, size_t *lenp, loff_t *ppos)
    {
    return do_proc_vec(table, dir, buffer, lenp, ppos, PROC_VEC_INT,
    (union proc_vec_conv){ .int_conv = do_proc_int_conv_minmax });
    }
//
// proc_douintvec_minmax - read a vector of unsigned ints with min/max values
// @table: the sysctl table
// @dir: %TRUE if this is a write to the sysctl file
// @buffer: the user buffer
// @lenp: the size of the user buffer
// @ppos: file position
//
// Reads/writes up to table->maxlen/sizeof(unsigned int) unsigned integer
// values from/to the user buffer, treated as an ASCII string. Negative
// strings are not allowed.
//
// When changing the kernel variable, this routine will ensure the values
// are within the range specified by table->extra1 (min) and table->extra2
// (max). And Check that the values are less than UINT_MAX to avoid having to
// support wrap around uses from userspace.
//
// Returns: %0 on success or -ERANGE when range check failes and
// SYSCTL_USER_TO_KERN(dir) == true
//
    int proc_douintvec_minmax(const struct ctl_table *table, int dir,
    void *buffer, size_t *lenp, loff_t *ppos)
    {
    return do_proc_vec(table, dir, buffer, lenp, ppos, PROC_VEC_UINT,
    (union proc_vec_conv){ .uint_conv = do_proc_uint_conv_minmax });
    }
//
// proc_dou8vec_minmax - read a vector of unsigned chars with min/max values
// @table: the sysctl table
// @dir: %TRUE if this is a write to the sysctl file
// @buffer: the user buffer
// @lenp: the size of the user buffer
// @ppos: file position
//
// Reads/writes up to table->maxlen/sizeof(u8) unsigned chars
// values from/to the user buffer, treated as an ASCII string. Negative
// strings are not allowed.
//
// This routine will ensure the values are within the range specified by
// table->extra1 (min) and table->extra2 (max).
//
// Returns: %0 on success or an error on SYSCTL_USER_TO_KERN(dir) == true
// and the range check fails.
//
    int proc_dou8vec_minmax(const struct ctl_table *table, int dir,
    void *buffer, size_t *lenp, loff_t *ppos)
    {
    struct ctl_table tmp;
    let mut min: c_uint = 0, max = 255U, val;
    u8 *data = table.data;
    int res;
// Do not support arrays yet.
    if (table.maxlen != sizeof(u8))
    return -EINVAL;
    tmp = *table;
    tmp.maxlen = sizeof(val);
    tmp.data = &val;
    if (!tmp.extra1)
    tmp.extra1 = (unsigned int *) &min;
    if (!tmp.extra2)
    tmp.extra2 = (unsigned int *) &max;
    val = READ_ONCE(*data);
    res = do_proc_vec(&tmp, dir, buffer, lenp, ppos, PROC_VEC_UINT,
    (union proc_vec_conv){ .uint_conv = do_proc_uint_conv_minmax });
    if (res)
    return res;
    if (SYSCTL_USER_TO_KERN(dir))
    WRITE_ONCE(*data, val);
    return 0;
    }
    EXPORT_SYMBOL_GPL(proc_dou8vec_minmax);
//
// proc_ulong_conv - Change user or kernel pointer based on direction
//
// @u_ptr: pointer to user variable
// @k_ptr: pointer to kernel variable
// @dir: %TRUE if this is a write to the sysctl file
// @tbl: the sysctl table
// @k_ptr_range_check: Check range for k_ptr when %TRUE
// @user_to_kern: Callback used to assign value from user to kernel var
// @kern_to_user: Callback used to assign value from kernel to user var
//
// When direction is kernel to user, then the u_ptr is modified.
// When direction is user to kernel, then the k_ptr is modified.
//
// Returns: 0 on success
//
    int proc_ulong_conv(ulong *u_ptr, ulong *k_ptr, int dir,
    const struct ctl_table *tbl, bool k_ptr_range_check,
    int (*user_to_kern)(const ulong *u_ptr, ulong *k_ptr),
    int (*kern_to_user)(ulong *u_ptr, const ulong *k_ptr))
    {
    if (SYSCTL_KERN_TO_USER(dir))
    return kern_to_user(u_ptr, k_ptr);
    if (k_ptr_range_check) {
    ulong tmp_k;
    int ret;
    if (!tbl)
    return -EINVAL;
    ret = user_to_kern(u_ptr, &tmp_k);
    if (ret)
    return ret;
    if ((tbl.extra1 && *(ulong *)tbl.extra1 > tmp_k) ||
    (tbl.extra2 && *(ulong *)tbl.extra2 < tmp_k))
    return -ERANGE;
    WRITE_ONCE(*k_ptr, tmp_k);
    } else
    return user_to_kern(u_ptr, k_ptr);
    return 0;
    }
//
// proc_ulong_u2k_conv_uop - Assign user value to a kernel pointer
//
// @u_ptr: pointer to user space variable
// @k_ptr: pointer to kernel variable
// @u_ptr_op: execute this function before assigning to k_ptr
//
// Uses WRITE_ONCE to assign value to k_ptr. Executes u_ptr_op if
// not NULL.
//
// Returns: 0 on success.
//
    int proc_ulong_u2k_conv_uop(const ulong *u_ptr, ulong *k_ptr,
    ulong (*u_ptr_op)(const ulong))
    {
    let mut u: c_ulong = u_ptr_op ? u_ptr_op(*u_ptr) : *u_ptr;
    WRITE_ONCE(*k_ptr, u);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn proc_ulong_u2k_conv(u_ptr: *const c_ulong, k_ptr: *mut c_ulong) -> c_int {
    return proc_ulong_u2k_conv_uop(u_ptr, k_ptr, core::ptr::null_mut());
    }
//
// proc_ulong_k2u_conv_kop - Assign kernel value to a user space pointer
//
// @u_ptr: pointer to user space variable
// @k_ptr: pointer to kernel variable
// @k_ptr_op: Operation applied to k_ptr before assignment
//
// Uses READ_ONCE to assign value to u_ptr. Executes k_ptr_op if
// not NULL.
//
// Returns: 0 on success.
//
    int proc_ulong_k2u_conv_kop(ulong *u_ptr, const ulong *k_ptr,
    ulong (*k_ptr_op)(const ulong))
    {
    let mut val: c_ulong = k_ptr_op ? k_ptr_op(READ_ONCE(*k_ptr)) : READ_ONCE(*k_ptr);
// u_ptr = (ulong)val;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn proc_ulong_k2u_conv(u_ptr: *mut c_ulong, k_ptr: *const c_ulong) -> c_int {
    return proc_ulong_k2u_conv_kop(u_ptr, k_ptr, core::ptr::null_mut());
    }
    static int do_proc_ulong_conv(bool *negp, ulong *u_ptr, ulong *k_ptr, int dir,
    const struct ctl_table *tbl)
    {
    return proc_ulong_conv(u_ptr, k_ptr, dir, tbl, true,
    proc_ulong_u2k_conv, proc_ulong_k2u_conv);
    }
//
// proc_doulongvec_conv - read a vector of unsigned longs with a custom converter
//
// @table: the sysctl table
// @dir: %TRUE if this is a write to the sysctl file
// @buffer: the user buffer
// @lenp: the size of the user buffer
// @ppos: file position
// @conv: Custom converter call back
//
// Reads/writes up to table->maxlen/sizeof(unsigned long) unsigned long
// values from/to the user buffer, treated as an ASCII string. Negative
// strings are not allowed.
//
// Returns: 0 on success
//
    int proc_doulongvec_conv(const struct ctl_table *table, int dir,
    void *buffer, size_t *lenp, loff_t *ppos,
    int (*conv)(bool *negp, ulong *u_ptr, ulong *k_ptr,
    int dir, const struct ctl_table *table))
    {
    return do_proc_vec(table, dir, buffer, lenp, ppos, PROC_VEC_ULONG,
    (union proc_vec_conv){ .ulong_conv = conv });
    }
//
// proc_doulongvec_minmax - read a vector of long integers with min/max values
// @table: the sysctl table
// @dir: %TRUE if this is a write to the sysctl file
// @buffer: the user buffer
// @lenp: the size of the user buffer
// @ppos: file position
//
// Reads/writes up to table->maxlen/sizeof(unsigned long) unsigned long
// values from/to the user buffer, treated as an ASCII string.
//
// This routine will ensure the values are within the range specified by
// table->extra1 (min) and table->extra2 (max).
//
// Returns: %0 on success.
//
    int proc_doulongvec_minmax(const struct ctl_table *table, int dir,
    void *buffer, size_t *lenp, loff_t *ppos)
    {
    return do_proc_vec(table, dir, buffer, lenp, ppos, PROC_VEC_ULONG,
    (union proc_vec_conv){ .ulong_conv = do_proc_ulong_conv });
    }
//
// proc_dointvec_conv - read a vector of ints with a custom converter
// @table: the sysctl table
// @dir: %TRUE if this is a write to the sysctl file
// @buffer: the user buffer
// @lenp: the size of the user buffer
// @ppos: file position
// @conv: Custom converter call back. Defaults to do_proc_int_conv
//
// Reads/writes up to table->maxlen/sizeof(int) integer values from/to the
// user buffer, treated as an ASCII string.
//
// Returns: 0 on success
//
    int proc_dointvec_conv(const struct ctl_table *table, int dir, void *buffer,
    size_t *lenp, loff_t *ppos,
    int (*conv)(bool *negp, unsigned long *u_ptr, int *k_ptr,
    int dir, const struct ctl_table *table))
    {
    if (!conv)
    conv = do_proc_int_conv;
    return do_proc_vec(table, dir, buffer, lenp, ppos, PROC_VEC_INT,
    (union proc_vec_conv){ .int_conv = conv });
    }
//
// proc_do_large_bitmap - read/write from/to a large bitmap
// @table: the sysctl table
// @dir: %TRUE if this is a write to the sysctl file
// @buffer: the user buffer
// @lenp: the size of the user buffer
// @ppos: file position
//
// The bitmap is stored at table->data and the bitmap length (in bits)
// in table->maxlen.
//
// We use a range comma separated format (e.g. 1,3-4,10-10) so that
// large bitmaps may be represented in a compact manner. Writing into
// the file will clear the bitmap then update it with the given input.
//
// Returns: %0 on success.
//
    int proc_do_large_bitmap(const struct ctl_table *table, int dir,
    void *buffer, size_t *lenp, loff_t *ppos)
    {
    let mut err: c_int = 0;
    let mut left: usize = *lenp;
    let mut bitmap_len: c_ulong = table.maxlen;
    unsigned long *bitmap = *(unsigned long **) table.data;
    unsigned long *tmp_bitmap = core::ptr::null_mut();
    char tr_a[] = { '-', ',', '\n' }, tr_b[] = { ',', '\n', 0 }, c = 0;
    if (!bitmap || !bitmap_len || !left || (*ppos && SYSCTL_KERN_TO_USER(dir))) {
// lenp = 0;
    return 0;
    }
    if (SYSCTL_USER_TO_KERN(dir)) {
    char *p = buffer;
    let mut skipped: usize = 0;
    if (left > PAGE_SIZE - 1) {
    left = PAGE_SIZE - 1;
// How much of the buffer we'll skip this pass
    skipped = *lenp - left;
    }
    tmp_bitmap = bitmap_zalloc(bitmap_len, GFP_KERNEL);
    if (!tmp_bitmap)
    return -ENOMEM;
    proc_skip_char(&p, &left, '\n');
    while (!err && left) {
    unsigned long val_a, val_b;
    bool neg;
    size_t saved_left;
// In case we stop parsing mid-number, we can reset
    saved_left = left;
    err = proc_get_long(&p, &left, &val_a, &neg, tr_a,
    sizeof(tr_a), &c);
//
// If we consumed the entirety of a truncated buffer or
// only one char is left (may be a "-"), then stop here,
// reset, & come back for more.
//
    if ((left <= 1) && skipped) {
    left = saved_left;
    break;
    }
    if (err)
    break;
    if (val_a >= bitmap_len || neg) {
    err = -EINVAL;
    break;
    }
    val_b = val_a;
    if (left) {
    p++;
    left--;
    }
    if (c == '-') {
    err = proc_get_long(&p, &left, &val_b,
    &neg, tr_b, sizeof(tr_b),
    &c);
//
// If we consumed all of a truncated buffer or
// then stop here, reset, & come back for more.
//
    if (!left && skipped) {
    left = saved_left;
    break;
    }
    if (err)
    break;
    if (val_b >= bitmap_len || neg ||
    val_a > val_b) {
    err = -EINVAL;
    break;
    }
    if (left) {
    p++;
    left--;
    }
    }
    bitmap_set(tmp_bitmap, val_a, val_b - val_a + 1);
    proc_skip_char(&p, &left, '\n');
    }
    left += skipped;
    } else {
    unsigned long bit_a, bit_b = 0;
    let mut first: bool = 1;
    while (left) {
    bit_a = find_next_bit(bitmap, bitmap_len, bit_b);
    if (bit_a >= bitmap_len)
    break;
    bit_b = find_next_zero_bit(bitmap, bitmap_len,
    bit_a + 1) - 1;
    if (!first)
    proc_put_char(&buffer, &left, ',');
    proc_put_long(&buffer, &left, bit_a, false);
    if (bit_a != bit_b) {
    proc_put_char(&buffer, &left, '-');
    proc_put_long(&buffer, &left, bit_b, false);
    }
    first = 0; bit_b++;
    }
    proc_put_char(&buffer, &left, '\n');
    }
    if (!err) {
    if (SYSCTL_USER_TO_KERN(dir)) {
    if (*ppos)
    bitmap_or(bitmap, bitmap, tmp_bitmap, bitmap_len);
    else
    bitmap_copy(bitmap, tmp_bitmap, bitmap_len);
    }
// lenp -= left;
// ppos += *lenp;
    }
    bitmap_free(tmp_bitmap);
    return err;
    }

    int proc_dostring(const struct ctl_table *table, int dir,
    void *buffer, size_t *lenp, loff_t *ppos)
    {
    return -ENOSYS;
    }
    int proc_dobool(const struct ctl_table *table, int dir,
    void *buffer, size_t *lenp, loff_t *ppos)
    {
    return -ENOSYS;
    }
    int proc_dointvec(const struct ctl_table *table, int dir,
    void *buffer, size_t *lenp, loff_t *ppos)
    {
    return -ENOSYS;
    }
    int proc_douintvec(const struct ctl_table *table, int dir,
    void *buffer, size_t *lenp, loff_t *ppos)
    {
    return -ENOSYS;
    }
    int proc_dointvec_minmax(const struct ctl_table *table, int dir,
    void *buffer, size_t *lenp, loff_t *ppos)
    {
    return -ENOSYS;
    }
    int proc_douintvec_minmax(const struct ctl_table *table, int dir,
    void *buffer, size_t *lenp, loff_t *ppos)
    {
    return -ENOSYS;
    }
    int proc_douintvec_conv(const struct ctl_table *table, int write, void *buffer,
    size_t *lenp, loff_t *ppos,
    int (*conv)(bool *negp, ulong *lvalp, uint *valp,
    int write, const struct ctl_table *table))
    {
    return -ENOSYS;
    }
#[no_mangle]
pub unsafe extern "C" fn proc_uint_k2u_conv(u_ptr: *mut c_ulong, k_ptr: *const c_uint) -> c_int {
    return -ENOSYS;
    }
    int proc_uint_u2k_conv_uop(const ulong *u_ptr, uint *k_ptr,
    ulong (*u_ptr_op)(const ulong))
    {
    return -ENOSYS;
    }
    int proc_uint_conv(ulong *u_ptr, uint *k_ptr, int dir,
    const struct ctl_table *tbl, bool k_ptr_range_check,
    int (*user_to_kern)(const ulong *u_ptr, uint *k_ptr),
    int (*kern_to_user)(ulong *u_ptr, const uint *k_ptr))
    {
    return -ENOSYS;
    }
    int proc_dou8vec_minmax(const struct ctl_table *table, int dir,
    void *buffer, size_t *lenp, loff_t *ppos)
    {
    return -ENOSYS;
    }
    int proc_doulongvec_minmax(const struct ctl_table *table, int dir,
    void *buffer, size_t *lenp, loff_t *ppos)
    {
    return -ENOSYS;
    }
    int proc_doulongvec_conv(const struct ctl_table *table, int dir,
    void *buffer, size_t *lenp, loff_t *ppos,
    int (*conv)(bool *negp, ulong *u_ptr, ulong *k_ptr,
    int dir, const struct ctl_table *table))
    {
    return -ENOSYS;
    }
    int proc_dointvec_conv(const struct ctl_table *table, int dir, void *buffer,
    size_t *lenp, loff_t *ppos,
    int (*conv)(bool *negp, unsigned long *u_ptr, int *k_ptr,
    int dir, const struct ctl_table *table))
    {
    return -ENOSYS;
    }
    int proc_do_large_bitmap(const struct ctl_table *table, int dir,
    void *buffer, size_t *lenp, loff_t *ppos)
    {
    return -ENOSYS;
    }

    int proc_do_static_key(const struct ctl_table *table, int dir,
    void *buffer, size_t *lenp, loff_t *ppos)
    {
    struct static_key *key = (struct static_key *)table.data;
// static DEFINE_MUTEX(static_key_mutex);
    int val, ret;
    struct ctl_table tmp = {
    .data   = &val,
    .maxlen = sizeof(val),
    .mode   = table.mode,
    .extra1 = SYSCTL_ZERO,
    .extra2 = SYSCTL_ONE,
    };
    if (SYSCTL_USER_TO_KERN(dir) && !capable(CAP_SYS_ADMIN))
    return -EPERM;
    mutex_lock(&static_key_mutex);
    val = static_key_enabled(key);
    ret = proc_dointvec_minmax(&tmp, dir, buffer, lenp, ppos);
    if (SYSCTL_USER_TO_KERN(dir) && !ret) {
    if (val)
    static_key_enable(key);
    else
    static_key_disable(key);
    }
    mutex_unlock(&static_key_mutex);
    return ret;
    }
    static const struct ctl_table sysctl_subsys_table[] = {

    {
    .procname	= "sysctl_writes_strict",
    .data		= &sysctl_writes_strict,
    .maxlen		= sizeof(int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec_minmax,
    .extra1		= SYSCTL_NEG_ONE,
    .extra2		= SYSCTL_ONE,
    },

    {
    .procname	= "ngroups_max",
    .data		= (void *)&ngroups_max,
    .maxlen		= sizeof (int),
    .mode		= 0444,
    .proc_handler	= proc_dointvec,
    },
    {
    .procname	= "cap_last_cap",
    .data		= (void *)&cap_last_cap,
    .maxlen		= sizeof(int),
    .mode		= 0444,
    .proc_handler	= proc_dointvec,
    },

    {
    .procname	= "unaligned-trap",
    .data		= &unaligned_enabled,
    .maxlen		= sizeof(int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec,
    },

    {
    .procname	= "ignore-unaligned-usertrap",
    .data		= &no_unaligned_warning,
    .maxlen		= sizeof (int),
    .mode		= 0644,
    .proc_handler	= proc_dointvec,
    },

    };
#[no_mangle]
pub unsafe extern "C" fn sysctl_init_bases() -> c_int {
    register_sysctl_init("kernel", sysctl_subsys_table);
    return 0;
    }

//
// No sense putting this after each symbol definition, twice,
// exception granted :-)
//
    EXPORT_SYMBOL(proc_dobool);
    EXPORT_SYMBOL(proc_dointvec);
    EXPORT_SYMBOL(proc_douintvec);
    EXPORT_SYMBOL(proc_dointvec_minmax);
    EXPORT_SYMBOL_GPL(proc_douintvec_minmax);
    EXPORT_SYMBOL(proc_dostring);
    EXPORT_SYMBOL(proc_doulongvec_minmax);
    EXPORT_SYMBOL(proc_do_large_bitmap);
