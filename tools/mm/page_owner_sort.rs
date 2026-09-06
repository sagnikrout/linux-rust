//! Automatically rewritten from C to Rust
//! Source: tools/mm/page_owner_sort.c
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
// User-space helper to sort the output of /sys/kernel/debug/page_owner
//
// Example use:
// cat /sys/kernel/debug/page_owner > page_owner_full.txt
// ./page_owner_sort page_owner_full.txt sorted_page_owner.txt
// Or sort by total memory:
// ./page_owner_sort -m page_owner_full.txt sorted_page_owner.txt
//
// See Documentation/mm/page_owner.rst
//

pub const TASK_COMM_LEN: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct block_list {
    pub txt: *mut c_char,
    pub name: *mut *mut char comm; // task command,
    pub stacktrace: *mut c_char,
    pub ts_nsec: __u64,
    pub len: c_int,
    pub num: c_int,
    pub page_num: c_int,
    pub pid: pid_t,
    pub tgid: pid_t,
    pub allocator: c_int,
}

    enum FILTER_BIT {
    FILTER_PID = 1<<1,
    FILTER_TGID = 1<<2,
    FILTER_COMM = 1<<3
    };
    enum FILTER_RESULT {
    FILTER_ERROR,
    FILTER_SKIP,
    FILTER_MATCH
    };
    enum CULL_BIT {
    CULL_PID = 1<<1,
    CULL_TGID = 1<<2,
    CULL_COMM = 1<<3,
    CULL_STACKTRACE = 1<<4,
    CULL_ALLOCATOR = 1<<5
    };
    enum ALLOCATOR_BIT {
    ALLOCATOR_CMA = 1<<1,
    ALLOCATOR_SLAB = 1<<2,
    ALLOCATOR_VMALLOC = 1<<3,
    ALLOCATOR_OTHERS = 1<<4
    };
    enum ARG_TYPE {
    ARG_TXT, ARG_COMM, ARG_STACKTRACE, ARG_ALLOC_TS, ARG_CULL_TIME,
    ARG_PAGE_NUM, ARG_PID, ARG_TGID, ARG_UNKNOWN, ARG_ALLOCATOR
    };
    enum SORT_ORDER {
    SORT_ASC = 1,
    SORT_DESC = -1,
    };
    enum COMP_FLAG {
    COMP_NO_FLAG = 0,
    COMP_ALLOC = 1<<0,
    COMP_PAGE_NUM = 1<<1,
    COMP_PID = 1<<2,
    COMP_STACK = 1<<3,
    COMP_NUM = 1<<4,
    COMP_TGID = 1<<5,
    COMP_COMM = 1<<6
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct filter_condition {
    pub pids: *mut pid_t,
    pub tgids: *mut pid_t,
    pub comms: *mut c_char,
    pub pids_size: c_int,
    pub tgids_size: c_int,
    pub comms_size: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sort_condition {
    pub ): *const *const *const *const int (cmps)(void , void,
    pub signs: *mut c_int,
    pub size: c_int,
}

    static struct filter_condition fc;
    static struct sort_condition sc;
    static regex_t order_pattern;
    static regex_t pid_pattern;
    static regex_t tgid_pattern;
    static regex_t comm_pattern;
    static regex_t ts_nsec_pattern;
    static struct block_list *list;
    static int list_size;
    static int max_size;
    static int cull;
    static int filter;
    static bool debug_on;
    static void set_single_cmp(int (*cmp)(const void *, const void *), int sign);
#[no_mangle]
pub unsafe extern "C" fn read_block(buf: *mut c_char, ext_buf: *mut c_char, buf_size: c_int, fin: *mut FILE) -> c_int {
    int read_block(char *buf, char *ext_buf, int buf_size, FILE *fin)
    {
    char *curr = buf, *const buf_end = buf + buf_size;
    while (buf_end - curr > 1 && fgets(curr, buf_end - curr, fin)) {
    if (*curr == '\n') { /* empty line */
    return curr - buf;
    }
    if (!strncmp(curr, "PFN", 3)) {
    strcpy(ext_buf, curr);
    continue;
    }
    curr += strlen(curr);
    }
    return -1; /* EOF or no space left in buf. */
    }
#[no_mangle]
unsafe extern "C" fn compare_txt(p1: *const c_void, p2: *const c_void) -> c_int {
    static int compare_txt(const void *p1, const void *p2)
    {
    const struct block_list *l1 = p1, *l2 = p2;
    return strcmp(l1.txt, l2.txt);
    }
#[no_mangle]
unsafe extern "C" fn compare_stacktrace(p1: *const c_void, p2: *const c_void) -> c_int {
    static int compare_stacktrace(const void *p1, const void *p2)
    {
    const struct block_list *l1 = p1, *l2 = p2;
    return strcmp(l1.stacktrace, l2.stacktrace);
    }
#[no_mangle]
unsafe extern "C" fn compare_num(p1: *const c_void, p2: *const c_void) -> c_int {
    static int compare_num(const void *p1, const void *p2)
    {
    const struct block_list *l1 = p1, *l2 = p2;
    return l1.num - l2.num;
    }
#[no_mangle]
unsafe extern "C" fn compare_page_num(p1: *const c_void, p2: *const c_void) -> c_int {
    static int compare_page_num(const void *p1, const void *p2)
    {
    const struct block_list *l1 = p1, *l2 = p2;
    return l1.page_num - l2.page_num;
    }
#[no_mangle]
unsafe extern "C" fn compare_pid(p1: *const c_void, p2: *const c_void) -> c_int {
    static int compare_pid(const void *p1, const void *p2)
    {
    const struct block_list *l1 = p1, *l2 = p2;
    return l1.pid - l2.pid;
    }
#[no_mangle]
unsafe extern "C" fn compare_tgid(p1: *const c_void, p2: *const c_void) -> c_int {
    static int compare_tgid(const void *p1, const void *p2)
    {
    const struct block_list *l1 = p1, *l2 = p2;
    return l1.tgid - l2.tgid;
    }
#[no_mangle]
unsafe extern "C" fn compare_allocator(p1: *const c_void, p2: *const c_void) -> c_int {
    static int compare_allocator(const void *p1, const void *p2)
    {
    const struct block_list *l1 = p1, *l2 = p2;
    return l1.allocator - l2.allocator;
    }
#[no_mangle]
unsafe extern "C" fn compare_comm(p1: *const c_void, p2: *const c_void) -> c_int {
    static int compare_comm(const void *p1, const void *p2)
    {
    const struct block_list *l1 = p1, *l2 = p2;
    return strcmp(l1.comm, l2.comm);
    }
#[no_mangle]
unsafe extern "C" fn compare_ts(p1: *const c_void, p2: *const c_void) -> c_int {
    static int compare_ts(const void *p1, const void *p2)
    {
    const struct block_list *l1 = p1, *l2 = p2;
    if (l1.ts_nsec < l2.ts_nsec)
    return -1;
    if (l1.ts_nsec > l2.ts_nsec)
    return 1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn compare_cull_condition(p1: *const c_void, p2: *const c_void) -> c_int {
    static int compare_cull_condition(const void *p1, const void *p2)
    {
    if (cull == 0)
    return compare_txt(p1, p2);
    if ((cull & CULL_STACKTRACE) && compare_stacktrace(p1, p2))
    return compare_stacktrace(p1, p2);
    if ((cull & CULL_PID) && compare_pid(p1, p2))
    return compare_pid(p1, p2);
    if ((cull & CULL_TGID) && compare_tgid(p1, p2))
    return compare_tgid(p1, p2);
    if ((cull & CULL_COMM) && compare_comm(p1, p2))
    return compare_comm(p1, p2);
    if ((cull & CULL_ALLOCATOR) && compare_allocator(p1, p2))
    return compare_allocator(p1, p2);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn compare_sort_condition(p1: *const c_void, p2: *const c_void) -> c_int {
    static int compare_sort_condition(const void *p1, const void *p2)
    {
    let mut cmp: c_int = 0;
    for (int i = 0; i < sc.size; ++i)
    if (cmp == 0)
    cmp = sc.signs[i] * sc.cmps[i](p1, p2);
    return cmp;
    }
#[no_mangle]
unsafe extern "C" fn remove_pattern(pattern: *mut regex_t, buf: *mut c_char, len: c_int) -> c_int {
    static int remove_pattern(regex_t *pattern, char *buf, int len)
    {
    regmatch_t pmatch[2];
    int err;
    err = regexec(pattern, buf, 2, pmatch, REG_NOTBOL);
    if (err != 0 || pmatch[1].rm_so == -1)
    return len;
    memcpy(buf + pmatch[1].rm_so,
    buf + pmatch[1].rm_eo, len - pmatch[1].rm_eo);
    return len - (pmatch[1].rm_eo - pmatch[1].rm_so);
    }
    static int search_pattern(regex_t *pattern, char *pattern_str,
    size_t pattern_str_size, char *buf)
    {
    int err, val_len;
    regmatch_t pmatch[2];
    err = regexec(pattern, buf, 2, pmatch, REG_NOTBOL);
    if (err != 0 || pmatch[1].rm_so == -1) {
    if (debug_on)
    fprintf(stderr, "no matching pattern in %s\n", buf);
    return -1;
    }
    val_len = pmatch[1].rm_eo - pmatch[1].rm_so;
    if ((size_t)val_len >= pattern_str_size) {
    if (debug_on)
    fprintf(stderr, "pattern too long in %s\n", buf);
    return -1;
    }
    memcpy(pattern_str, buf + pmatch[1].rm_so, val_len);
    pattern_str[val_len] = '\0';
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn check_regcomp(pattern: *mut regex_t, regex: *const c_char) -> bool {
    static bool check_regcomp(regex_t *pattern, const char *regex)
    {
    int err;
    err = regcomp(pattern, regex, REG_EXTENDED | REG_NEWLINE);
    if (err != 0 || pattern.re_nsub != 1) {
    fprintf(stderr, "Invalid pattern %s code %d\n", regex, err);
    return false;
    }
    return true;
    }
    static char **explode(char sep, const char *str, int *size)
    {
    let mut count: c_int = 0, len = strlen(str);
    let mut lastindex: c_int = -1, j = 0;
    for (int i = 0; i < len; i++)
    if (str[i] == sep)
    count++;
    char **ret = calloc(++count, sizeof(char *));
    for (int i = 0; i < len; i++) {
    if (str[i] == sep) {
    ret[j] = calloc(i - lastindex, sizeof(char));
    memcpy(ret[j++], str + lastindex + 1, i - lastindex - 1);
    lastindex = i;
    }
    }
    if (lastindex <= len - 1) {
    ret[j] = calloc(len - lastindex, sizeof(char));
    memcpy(ret[j++], str + lastindex + 1, strlen(str) - 1 - lastindex);
    }
// size = j;
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn free_explode(arr: *mut c_char, size: c_int) {
    static void free_explode(char **arr, int size)
    {
    for (int i = 0; i < size; i++)
    free(arr[i]);
    free(arr);
    }

#[no_mangle]
unsafe extern "C" fn get_page_num(buf: *mut c_char) -> c_int {
    static int get_page_num(char *buf)
    {
    int order_val;
    char order_str[FIELD_BUFF] = {0};
    char *endptr;
    if (search_pattern(&order_pattern, order_str, sizeof(order_str), buf) < 0)
    return 0;
    errno = 0;
    order_val = strtol(order_str, &endptr, 10);
    if (order_val > 64 || errno != 0 || endptr == order_str || *endptr != '\0') {
    if (debug_on)
    fprintf(stderr, "wrong order in follow buf:\n%s\n", buf);
    return 0;
    }
    return 1 << order_val;
    }
#[no_mangle]
unsafe extern "C" fn get_pid(buf: *mut c_char) -> pid_t {
    static pid_t get_pid(char *buf)
    {
    pid_t pid;
    char pid_str[FIELD_BUFF] = {0};
    char *endptr;
    if (search_pattern(&pid_pattern, pid_str, sizeof(pid_str), buf) < 0)
    return -1;
    errno = 0;
    pid = strtol(pid_str, &endptr, 10);
    if (errno != 0 || endptr == pid_str || *endptr != '\0') {
    if (debug_on)
    fprintf(stderr, "wrong/invalid pid in follow buf:\n%s\n", buf);
    return -1;
    }
    return pid;
    }
#[no_mangle]
unsafe extern "C" fn get_tgid(buf: *mut c_char) -> pid_t {
    static pid_t get_tgid(char *buf)
    {
    pid_t tgid;
    char tgid_str[FIELD_BUFF] = {0};
    char *endptr;
    if (search_pattern(&tgid_pattern, tgid_str, sizeof(tgid_str), buf) < 0)
    return -1;
    errno = 0;
    tgid = strtol(tgid_str, &endptr, 10);
    if (errno != 0 || endptr == tgid_str || *endptr != '\0') {
    if (debug_on)
    fprintf(stderr, "wrong/invalid tgid in follow buf:\n%s\n", buf);
    return -1;
    }
    return tgid;
    }
#[no_mangle]
unsafe extern "C" fn get_ts_nsec(buf: *mut c_char) -> __u64 {
    static __u64 get_ts_nsec(char *buf)
    {
    __u64 ts_nsec;
    char ts_nsec_str[FIELD_BUFF] = {0};
    char *endptr;
    if (search_pattern(&ts_nsec_pattern, ts_nsec_str,
    sizeof(ts_nsec_str), buf) < 0)
    return -1;
    errno = 0;
    ts_nsec = strtoull(ts_nsec_str, &endptr, 10);
    if (errno != 0 || endptr == ts_nsec_str || *endptr != '\0') {
    if (debug_on)
    fprintf(stderr, "wrong ts_nsec in follow buf:\n%s\n", buf);
    return -1;
    }
    return ts_nsec;
    }
    static char *get_comm(char *buf)
    {
    char *comm_str = malloc(TASK_COMM_LEN);
    if (!comm_str)
    return core::ptr::null_mut();
    memset(comm_str, 0, TASK_COMM_LEN);
    if (search_pattern(&comm_pattern, comm_str, TASK_COMM_LEN, buf) < 0) {
    free(comm_str);
    return core::ptr::null_mut();
    }
    errno = 0;
    if (errno != 0) {
    if (debug_on)
    fprintf(stderr, "wrong comm in follow buf:\n%s\n", buf);
    free(comm_str);
    return core::ptr::null_mut();
    }
    return comm_str;
    }
#[no_mangle]
unsafe extern "C" fn free_block_list(block: *mut block_list) {
    static void free_block_list(struct block_list *block)
    {
    free(block.comm);
    free(block.txt);
    }
#[no_mangle]
unsafe extern "C" fn get_arg_type(arg: *const c_char) -> c_int {
    static int get_arg_type(const char *arg)
    {
    if (!strcmp(arg, "pid") || !strcmp(arg, "p"))
    return ARG_PID;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(arg, !strcmp(arg: "tgid") ||, _arg: "tg")) -> else {
    else if (!strcmp(arg, "tgid") || !strcmp(arg, "tg"))
    return ARG_TGID;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(arg, !strcmp(arg: "name") ||, _arg: "n")) -> else {
    else if (!strcmp(arg, "name") || !strcmp(arg, "n"))
    return  ARG_COMM;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(arg, !strcmp(arg: "stacktrace") ||, _arg: "st")) -> else {
    else if (!strcmp(arg, "stacktrace") || !strcmp(arg, "st"))
    return ARG_STACKTRACE;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(arg, !strcmp(arg: "txt") ||, _arg: "T")) -> else {
    else if (!strcmp(arg, "txt") || !strcmp(arg, "T"))
    return ARG_TXT;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(arg, !strcmp(arg: "alloc_ts") ||, _arg: "at")) -> else {
    else if (!strcmp(arg, "alloc_ts") || !strcmp(arg, "at"))
    return ARG_ALLOC_TS;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(arg, !strcmp(arg: "allocator") ||, _arg: "ator")) -> else {
    else if (!strcmp(arg, "allocator") || !strcmp(arg, "ator"))
    return ARG_ALLOCATOR;
    else {
    return ARG_UNKNOWN;
    }
    }
#[no_mangle]
unsafe extern "C" fn get_allocator(buf: *const c_char, migrate_info: *const c_char) -> c_int {
    static int get_allocator(const char *buf, const char *migrate_info)
    {
    char *tmp, *first_line, *second_line;
    let mut allocator: c_int = 0;
    if (strstr(migrate_info, "CMA"))
    allocator |= ALLOCATOR_CMA;
    if (strstr(migrate_info, "slab"))
    allocator |= ALLOCATOR_SLAB;
    tmp = strstr(buf, "__vmalloc_node_range");
    if (tmp) {
    second_line = tmp;
    while (*tmp != '\n')
    tmp--;
    tmp--;
    while (*tmp != '\n')
    tmp--;
    first_line = ++tmp;
    tmp = strstr(tmp, "alloc_pages");
    if (tmp && first_line <= tmp && tmp < second_line)
    allocator |= ALLOCATOR_VMALLOC;
    }
    if (allocator == 0)
    allocator = ALLOCATOR_OTHERS;
    return allocator;
    }
#[no_mangle]
unsafe extern "C" fn match_num_list(num: c_int, list: *mut c_int, list_size: c_int) -> bool {
    static bool match_num_list(int num, int *list, int list_size)
    {
    for (int i = 0; i < list_size; ++i)
    if (list[i] == num)
    return true;
    return false;
    }
#[no_mangle]
unsafe extern "C" fn match_str_list(str: *const c_char, list: *mut c_char, list_size: c_int) -> bool {
    static bool match_str_list(const char *str, char **list, int list_size)
    {
    for (int i = 0; i < list_size; ++i)
    if (!strcmp(list[i], str))
    return true;
    return false;
    }
#[no_mangle]
unsafe extern "C" fn filter_record(buf: *mut c_char) -> enum FILTER_RESULT {
    static enum FILTER_RESULT filter_record(char *buf)
    {
    char *comm;
    if ((filter & FILTER_PID) && !match_num_list(get_pid(buf), fc.pids, fc.pids_size))
    return FILTER_SKIP;
    if ((filter & FILTER_TGID) &&
    !match_num_list(get_tgid(buf), fc.tgids, fc.tgids_size))
    return FILTER_SKIP;
    if (!(filter & FILTER_COMM))
    return FILTER_MATCH;
    comm = get_comm(buf);
    if (!comm)
    return FILTER_ERROR;
    if (!match_str_list(comm, fc.comms, fc.comms_size)) {
    free(comm);
    return FILTER_SKIP;
    }
    free(comm);
    return FILTER_MATCH;
    }
#[no_mangle]
unsafe extern "C" fn add_list(buf: *mut c_char, len: c_int, ext_buf: *mut c_char) -> bool {
    static bool add_list(char *buf, int len, char *ext_buf)
    {
    enum FILTER_RESULT filter_result;
    if (list_size == max_size) {
    fprintf(stderr, "max_size too small??\n");
    return false;
    }
    filter_result = filter_record(buf);
    if (filter_result == FILTER_ERROR) {
    fprintf(stderr, "Out of memory\n");
    return false;
    }
    if (filter_result == FILTER_SKIP)
    return true;
    list[list_size].pid = get_pid(buf);
    list[list_size].tgid = get_tgid(buf);
    list[list_size].comm = get_comm(buf);
    if (!list[list_size].comm) {
    fprintf(stderr, "Out of memory\n");
    return false;
    }
    list[list_size].txt = malloc(len + 1);
    if (!list[list_size].txt) {
    fprintf(stderr, "Out of memory\n");
    free(list[list_size].comm);
    return false;
    }
    memcpy(list[list_size].txt, buf, len);
    if (sc.cmps[0] != compare_ts) {
    len = remove_pattern(&ts_nsec_pattern, list[list_size].txt, len);
    }
    list[list_size].txt[len] = 0;
    list[list_size].len = len;
    list[list_size].num = 1;
    list[list_size].page_num = get_page_num(buf);
    list[list_size].stacktrace = strchr(list[list_size].txt, '\n') ?: "";
    if (*list[list_size].stacktrace == '\n')
    list[list_size].stacktrace++;
    list[list_size].ts_nsec = get_ts_nsec(buf);
    list[list_size].allocator = get_allocator(buf, ext_buf);
    list_size++;
    if (list_size % 1000 == 0) {
    printf("loaded %d\r", list_size);
    fflush(stdout);
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn parse_cull_args(arg_str: *const c_char) -> bool {
    static bool parse_cull_args(const char *arg_str)
    {
    let mut size: c_int = 0;
    char **args = explode(',', arg_str, &size);
    for (int i = 0; i < size; ++i) {
    let mut arg_type: c_int = get_arg_type(args[i]);
    if (arg_type == ARG_PID)
    cull |= CULL_PID;
#[no_mangle]
pub unsafe extern "C" fn if(ARG_TGID: arg_type ==) -> else {
    else if (arg_type == ARG_TGID)
    cull |= CULL_TGID;
#[no_mangle]
pub unsafe extern "C" fn if(ARG_COMM: arg_type ==) -> else {
    else if (arg_type == ARG_COMM)
    cull |= CULL_COMM;
#[no_mangle]
pub unsafe extern "C" fn if(ARG_STACKTRACE: arg_type ==) -> else {
    else if (arg_type == ARG_STACKTRACE)
    cull |= CULL_STACKTRACE;
#[no_mangle]
pub unsafe extern "C" fn if(ARG_ALLOCATOR: arg_type ==) -> else {
    else if (arg_type == ARG_ALLOCATOR)
    cull |= CULL_ALLOCATOR;
    else {
    free_explode(args, size);
    return false;
    }
    }
    free_explode(args, size);
    if (sc.size == 0)
    set_single_cmp(compare_num, SORT_DESC);
    return true;
    }
#[no_mangle]
unsafe extern "C" fn set_single_cmp(: *const *const int (cmp)(void, ): *const c_void, sign: c_int) {
    static void set_single_cmp(int (*cmp)(const void *, const void *), int sign)
    {
    if (sc.signs == core::ptr::null_mut() || sc.size < 1)
    sc.signs = calloc(1, sizeof(int));
    sc.signs[0] = sign;
    if (sc.cmps == core::ptr::null_mut() || sc.size < 1)
    sc.cmps = calloc(1, sizeof(int *));
    sc.cmps[0] = cmp;
    sc.size = 1;
    }
#[no_mangle]
unsafe extern "C" fn parse_sort_args(arg_str: *const c_char) -> bool {
    static bool parse_sort_args(const char *arg_str)
    {
    let mut size: c_int = 0;
    if (sc.size != 0) { /* reset sort_condition */
    free(sc.signs);
    free(sc.cmps);
    size = 0;
    }
    char **args = explode(',', arg_str, &size);
    sc.signs = calloc(size, sizeof(int));
    sc.cmps = calloc(size, sizeof(int *));
    for (int i = 0; i < size; ++i) {
    let mut offset: c_int = 0;
    sc.signs[i] = SORT_ASC;
    if (args[i][0] == '-' || args[i][0] == '+') {
    if (args[i][0] == '-')
    sc.signs[i] = SORT_DESC;
    offset = 1;
    }
    let mut arg_type: c_int = get_arg_type(args[i]+offset);
    if (arg_type == ARG_PID)
    sc.cmps[i] = compare_pid;
#[no_mangle]
pub unsafe extern "C" fn if(ARG_TGID: arg_type ==) -> else {
    else if (arg_type == ARG_TGID)
    sc.cmps[i] = compare_tgid;
#[no_mangle]
pub unsafe extern "C" fn if(ARG_COMM: arg_type ==) -> else {
    else if (arg_type == ARG_COMM)
    sc.cmps[i] = compare_comm;
#[no_mangle]
pub unsafe extern "C" fn if(ARG_STACKTRACE: arg_type ==) -> else {
    else if (arg_type == ARG_STACKTRACE)
    sc.cmps[i] = compare_stacktrace;
#[no_mangle]
pub unsafe extern "C" fn if(ARG_ALLOC_TS: arg_type ==) -> else {
    else if (arg_type == ARG_ALLOC_TS)
    sc.cmps[i] = compare_ts;
#[no_mangle]
pub unsafe extern "C" fn if(ARG_TXT: arg_type ==) -> else {
    else if (arg_type == ARG_TXT)
    sc.cmps[i] = compare_txt;
#[no_mangle]
pub unsafe extern "C" fn if(ARG_ALLOCATOR: arg_type ==) -> else {
    else if (arg_type == ARG_ALLOCATOR)
    sc.cmps[i] = compare_allocator;
    else {
    free_explode(args, size);
    sc.size = 0;
    return false;
    }
    }
    sc.size = size;
    free_explode(args, size);
    return true;
    }
    static int *parse_nums_list(char *arg_str, int *list_size)
    {
    let mut size: c_int = 0;
    char **args = explode(',', arg_str, &size);
    int *list = calloc(size, sizeof(int));
    errno = 0;
    for (int i = 0; i < size; ++i) {
    char *endptr = core::ptr::null_mut();
    list[i] = strtol(args[i], &endptr, 10);
    if (errno != 0 || endptr == args[i] || *endptr != '\0') {
    free(list);
    return core::ptr::null_mut();
    }
    }
// list_size = size;
    free_explode(args, size);
    return list;
    }
#[no_mangle]
unsafe extern "C" fn print_allocator(out: *mut FILE, allocator: c_int) {
    static void print_allocator(FILE *out, int allocator)
    {
    fprintf(out, "allocated by ");
    if (allocator & ALLOCATOR_CMA)
    fprintf(out, "CMA ");
    if (allocator & ALLOCATOR_SLAB)
    fprintf(out, "SLAB ");
    if (allocator & ALLOCATOR_VMALLOC)
    fprintf(out, "VMALLOC ");
    if (allocator & ALLOCATOR_OTHERS)
    fprintf(out, "OTHERS ");
    }

#[no_mangle]
unsafe extern "C" fn usage() {
    static void usage(void)
    {
    printf("Usage: ./page_owner_sort [OPTIONS] <input> <output>\n"
    "-a\t\t\tSort by memory allocation time.\n"
    "-m\t\t\tSort by total memory.\n"
    "-n\t\t\tSort by task command name.\n"
    "-p\t\t\tSort by pid.\n"
    "-P\t\t\tSort by tgid.\n"
    "-s\t\t\tSort by the stacktrace.\n"
    "-t\t\t\tSort by number of times record is seen (default).\n\n"
    "--pid <pidlist>\t\tSelect by pid. This selects the information"
    " of\n\t\t\tblocks whose process ID numbers appear in <pidlist>.\n"
    "--tgid <tgidlist>\tSelect by tgid. This selects the information"
    " of\n\t\t\tblocks whose Thread Group ID numbers appear in "
    "<tgidlist>.\n"
    "--name <cmdlist>\tSelect by command name. This selects the"
    " information\n\t\t\tof blocks whose command name appears in"
    " <cmdlist>.\n"
    "--cull <rules>\t\tCull by user-defined rules. <rules> is a "
    "single\n\t\t\targument in the form of a comma-separated list "
    "with some\n\t\t\tcommon fields predefined (pid, tgid, comm, "
    "stacktrace, allocator)\n"
    "--sort <order>\t\tSpecify sort order as: [+|-]key[,[+|-]key[,...]]\n"
    );
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    FILE *fin, *fout;
    char *buf, *ext_buf;
    int i, count, compare_flag;
    struct stat st;
    int opt;
    struct option longopts[] = {
    { "pid", required_argument, core::ptr::null_mut(), 1 },
    { "tgid", required_argument, core::ptr::null_mut(), 2 },
    { "name", required_argument, core::ptr::null_mut(), 3 },
    { "cull", required_argument, core::ptr::null_mut(), 4 },
    { "sort", required_argument, core::ptr::null_mut(), 5 },
    { "help", no_argument, core::ptr::null_mut(), 'h' },
    { 0, 0, 0, 0},
    };
    compare_flag = COMP_NO_FLAG;
    while ((opt = getopt_long(argc, argv, "admnpstPh", longopts, core::ptr::null_mut())) != -1)
    switch (opt) {
    case 'a':
    compare_flag |= COMP_ALLOC;
    break;
    case 'd':
    debug_on = true;
    break;
    case 'm':
    compare_flag |= COMP_PAGE_NUM;
    break;
    case 'p':
    compare_flag |= COMP_PID;
    break;
    case 's':
    compare_flag |= COMP_STACK;
    break;
    case 't':
    compare_flag |= COMP_NUM;
    break;
    case 'P':
    compare_flag |= COMP_TGID;
    break;
    case 'n':
    compare_flag |= COMP_COMM;
    break;
    case 'h':
    usage();
    exit(0);
    case 1:
    filter = filter | FILTER_PID;
    fc.pids = parse_nums_list(optarg, &fc.pids_size);
    if (fc.pids == core::ptr::null_mut()) {
    fprintf(stderr, "wrong/invalid pid in from the command line:%s\n",
    optarg);
    exit(1);
    }
    break;
    case 2:
    filter = filter | FILTER_TGID;
    fc.tgids = parse_nums_list(optarg, &fc.tgids_size);
    if (fc.tgids == core::ptr::null_mut()) {
    fprintf(stderr, "wrong/invalid tgid in from the command line:%s\n",
    optarg);
    exit(1);
    }
    break;
    case 3:
    filter = filter | FILTER_COMM;
    fc.comms = explode(',', optarg, &fc.comms_size);
    break;
    case 4:
    if (!parse_cull_args(optarg)) {
    fprintf(stderr, "wrong argument after --cull option:%s\n",
    optarg);
    exit(1);
    }
    break;
    case 5:
    if (!parse_sort_args(optarg)) {
    fprintf(stderr, "wrong argument after --sort option:%s\n",
    optarg);
    exit(1);
    }
    break;
    default:
    usage();
    exit(1);
    }
    if (optind >= (argc - 1)) {
    usage();
    exit(1);
    }
// Only one compare option is allowed, yet we also want handle the
// default case were no option is provided, but we still want to
// match the behavior of the -t option (compare by number of times
// a record is seen
//
    switch (compare_flag) {
    case COMP_ALLOC:
    set_single_cmp(compare_ts, SORT_ASC);
    break;
    case COMP_PAGE_NUM:
    set_single_cmp(compare_page_num, SORT_DESC);
    break;
    case COMP_PID:
    set_single_cmp(compare_pid, SORT_ASC);
    break;
    case COMP_STACK:
    set_single_cmp(compare_stacktrace, SORT_ASC);
    break;
    case COMP_NO_FLAG:
    case COMP_NUM:
    set_single_cmp(compare_num, SORT_DESC);
    break;
    case COMP_TGID:
    set_single_cmp(compare_tgid, SORT_ASC);
    break;
    case COMP_COMM:
    set_single_cmp(compare_comm, SORT_ASC);
    break;
    default:
    usage();
    exit(1);
    }
    fin = fopen(argv[optind], "r");
    if (!fin) {
    usage();
    perror("open: ");
    exit(1);
    }
    if (!check_regcomp(&order_pattern, "order\\s*([0-9]*),"))
    goto out_order;
    if (!check_regcomp(&pid_pattern, "pid\\s*([0-9]*),"))
    goto out_pid;
    if (!check_regcomp(&tgid_pattern, "tgid\\s*([0-9]*) "))
    goto out_tgid;
    if (!check_regcomp(&comm_pattern, "tgid\\s*[0-9]*\\s*\\((.*)\\),\\s*ts"))
    goto out_comm;
    if (!check_regcomp(&ts_nsec_pattern, "ts\\s*([0-9]*)\\s*ns"))
    goto out_ts;
    fstat(fileno(fin), &st);
    max_size = st.st_size / 100; /* hack ... */
    list = malloc(max_size * sizeof(*list));
    buf = malloc(BUF_SIZE);
    ext_buf = malloc(BUF_SIZE);
    if (!list || !buf || !ext_buf) {
    fprintf(stderr, "Out of memory\n");
    goto out_free;
    }
    for ( ; ; ) {
    let mut buf_len: c_int = read_block(buf, ext_buf, BUF_SIZE, fin);
    if (buf_len < 0)
    break;
    if (!add_list(buf, buf_len, ext_buf))
    goto out_free;
    }
    fout = fopen(argv[optind + 1], "w");
    if (!fout) {
    usage();
    perror("open: ");
    exit(1);
    }
    printf("loaded %d\n", list_size);
    printf("sorting ....\n");
    qsort(list, list_size, sizeof(list[0]), compare_cull_condition);
    printf("culling\n");
    for (i = count = 0; i < list_size; i++) {
    if (count == 0 ||
    compare_cull_condition((void *)(&list[count-1]), (void *)(&list[i])) != 0) {
    list[count++] = list[i];
    } else {
    list[count-1].num += list[i].num;
    list[count-1].page_num += list[i].page_num;
    free_block_list(&list[i]);
    }
    }
    list_size = count;
    qsort(list, count, sizeof(list[0]), compare_sort_condition);
    for (i = 0; i < count; i++) {
    if (cull == 0) {
    fprintf(fout, "%d times, %d pages, ", list[i].num, list[i].page_num);
    print_allocator(fout, list[i].allocator);
    fprintf(fout, ":\n%s\n", list[i].txt);
    }
    else {
    fprintf(fout, "%d times, %d pages",
    list[i].num, list[i].page_num);
    if (cull & CULL_PID || filter & FILTER_PID)
    fprintf(fout, ", PID %d", list[i].pid);
    if (cull & CULL_TGID || filter & FILTER_TGID)
    fprintf(fout, ", TGID %d", list[i].tgid);
    if (cull & CULL_COMM || filter & FILTER_COMM)
    fprintf(fout, ", task_comm_name: %s", list[i].comm);
    if (cull & CULL_ALLOCATOR) {
    fprintf(fout, ", ");
    print_allocator(fout, list[i].allocator);
    }
    if (cull & CULL_STACKTRACE)
    fprintf(fout, ":\n%s", list[i].stacktrace);
    fprintf(fout, "\n");
    }
    }
    out_free:
    if (ext_buf)
    free(ext_buf);
    if (buf)
    free(buf);
    if (list) {
    for (i = 0; i < list_size; i++)
    free_block_list(&list[i]);
    free(list);
    }
    out_ts:
    regfree(&ts_nsec_pattern);
    out_comm:
    regfree(&comm_pattern);
    out_tgid:
    regfree(&tgid_pattern);
    out_pid:
    regfree(&pid_pattern);
    out_order:
    regfree(&order_pattern);
    return 0;
    }
