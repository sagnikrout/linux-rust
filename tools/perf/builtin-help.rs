//! Automatically rewritten from C to Rust
//! Source: tools/perf/builtin-help.c
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
// builtin-help.c
//
// Builtin help command
//

    static struct man_viewer_list {
    struct man_viewer_list *next;
    char name[0];
    } *man_viewer_list;
    static struct man_viewer_info_list {
    struct man_viewer_info_list *next;
    const char *info;
    char name[0];
    } *man_viewer_info_list;
    enum help_format {
    HELP_FORMAT_NONE,
    HELP_FORMAT_MAN,
    HELP_FORMAT_INFO,
    HELP_FORMAT_WEB,
    };
#[no_mangle]
unsafe extern "C" fn parse_help_format(format: *const c_char) -> enum help_format {
    static enum help_format parse_help_format(const char *format)
    {
    if (!strcmp(format, "man"))
    return HELP_FORMAT_MAN;
    if (!strcmp(format, "info"))
    return HELP_FORMAT_INFO;
    if (!strcmp(format, "web") || !strcmp(format, "html"))
    return HELP_FORMAT_WEB;
    pr_err("unrecognized help format '%s'", format);
    return HELP_FORMAT_NONE;
    }
    static const char *get_man_viewer_info(const char *name)
    {
    struct man_viewer_info_list *viewer;
    for (viewer = man_viewer_info_list; viewer; viewer = viewer.next) {
    if (!strcasecmp(name, viewer.name))
    return viewer.info;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn check_emacsclient_version() -> c_int {
    static int check_emacsclient_version(void)
    {
    let mut buffer: strbuf = STRBUF_INIT;
    struct child_process ec_process;
    const char *argv_ec[] = { "emacsclient", "--version", core::ptr::null_mut() };
    int version;
    let mut ret: c_int = -1;
// emacsclient prints its version number on stderr
    memset(&ec_process, 0, sizeof(ec_process));
    ec_process.argv = argv_ec;
    ec_process.err = -1;
    ec_process.stdout_to_stderr = 1;
    if (start_command(&ec_process)) {
    fprintf(stderr, "Failed to start emacsclient.\n");
    return -1;
    }
    if (strbuf_read(&buffer, ec_process.err, 20) < 0) {
    fprintf(stderr, "Failed to read emacsclient version\n");
    goto out;
    }
    close(ec_process.err);
//
// Don't bother checking return value, because "emacsclient --version"
// seems to always exits with code 1.
//
    finish_command(&ec_process);
    if (!strstarts(buffer.buf, "emacsclient")) {
    fprintf(stderr, "Failed to parse emacsclient version.\n");
    goto out;
    }
    version = atoi(buffer.buf + strlen("emacsclient"));
    if (version < 22) {
    fprintf(stderr,
    "emacsclient version '%d' too old (< 22).\n",
    version);
    } else
    ret = 0;
    out:
    strbuf_release(&buffer);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn exec_failed(cmd: *const c_char) {
    static void exec_failed(const char *cmd)
    {
    char sbuf[STRERR_BUFSIZE];
    pr_warning("failed to exec '%s': %s", cmd, str_error_r(errno, sbuf, sizeof(sbuf)));
    }
#[no_mangle]
unsafe extern "C" fn exec_woman_emacs(path: *const c_char, page: *const c_char) {
    static void exec_woman_emacs(const char *path, const char *page)
    {
    if (!check_emacsclient_version()) {
// This works only with emacsclient version >= 22.
    char *man_page;
    if (!path)
    path = "emacsclient";
    if (asprintf(&man_page, "(woman \"%s\")", page) > 0) {
    execlp(path, "emacsclient", "-e", man_page, core::ptr::null_mut());
    free(man_page);
    }
    exec_failed(path);
    }
    }
#[no_mangle]
unsafe extern "C" fn exec_man_konqueror(path: *const c_char, page: *const c_char) {
    static void exec_man_konqueror(const char *path, const char *page)
    {
    const char *display = getenv("DISPLAY");
    if (display && *display) {
    char *man_page;
    const char *filename = "kfmclient";
// It's simpler to launch konqueror using kfmclient.
    if (path) {
    const char *file = strrchr(path, '/');
    if (file && !strcmp(file + 1, "konqueror")) {
    char *new = strdup(path);
    char *dest = strrchr(new, '/');
// strlen("konqueror") == strlen("kfmclient")
    strcpy(dest + 1, "kfmclient");
    path = new;
    }
    if (file)
    filename = file;
    } else
    path = "kfmclient";
    if (asprintf(&man_page, "man:%s(1)", page) > 0) {
    execlp(path, filename, "newTab", man_page, core::ptr::null_mut());
    free(man_page);
    }
    exec_failed(path);
    }
    }
#[no_mangle]
unsafe extern "C" fn exec_man_man(path: *const c_char, page: *const c_char) {
    static void exec_man_man(const char *path, const char *page)
    {
    if (!path)
    path = "man";
    execlp(path, "man", page, core::ptr::null_mut());
    exec_failed(path);
    }
#[no_mangle]
unsafe extern "C" fn exec_man_cmd(cmd: *const c_char, page: *const c_char) {
    static void exec_man_cmd(const char *cmd, const char *page)
    {
    char *shell_cmd;
    if (asprintf(&shell_cmd, "%s %s", cmd, page) > 0) {
    execl("/bin/sh", "sh", "-c", shell_cmd, core::ptr::null_mut());
    free(shell_cmd);
    }
    exec_failed(cmd);
    }
#[no_mangle]
unsafe extern "C" fn add_man_viewer(name: *const c_char) {
    static void add_man_viewer(const char *name)
    {
    struct man_viewer_list **p = &man_viewer_list;
    let mut len: usize = strlen(name);
    while (*p)
    p = &((*p).next);
// p = zalloc(sizeof(**p) + len + 1);
    strcpy((*p).name, name);
    }
#[no_mangle]
unsafe extern "C" fn supported_man_viewer(name: *const c_char, len: usize) -> c_int {
    static int supported_man_viewer(const char *name, size_t len)
    {
    return (!strncasecmp("man", name, len) ||
    !strncasecmp("woman", name, len) ||
    !strncasecmp("konqueror", name, len));
    }
    static void do_add_man_viewer_info(const char *name,
    size_t len,
    const char *value)
    {
    struct man_viewer_info_list *new = zalloc(sizeof(*new) + len + 1);
    strncpy(new.name, name, len);
    new.info = strdup(value);
    new.next = man_viewer_info_list;
    man_viewer_info_list = new;
    }
#[no_mangle]
unsafe extern "C" fn unsupported_man_viewer(name: *const c_char, var: *const c_char) {
    static void unsupported_man_viewer(const char *name, const char *var)
    {
    pr_warning("'%s': path for unsupported man viewer.\n"
    "Please consider using 'man.<tool>.%s' instead.", name, var);
    }
    static int add_man_viewer_path(const char *name,
    size_t len,
    const char *value)
    {
    if (supported_man_viewer(name, len))
    do_add_man_viewer_info(name, len, value);
    else
    unsupported_man_viewer(name, "cmd");
    return 0;
    }
    static int add_man_viewer_cmd(const char *name,
    size_t len,
    const char *value)
    {
    if (supported_man_viewer(name, len))
    unsupported_man_viewer(name, "path");
    else
    do_add_man_viewer_info(name, len, value);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn add_man_viewer_info(var: *const c_char, value: *const c_char) -> c_int {
    static int add_man_viewer_info(const char *var, const char *value)
    {
    const char *name = var + 4;
    const char *subkey = strrchr(name, '.');
    if (!subkey) {
    pr_err("Config with no key for man viewer: %s", name);
    return -1;
    }
    if (!strcmp(subkey, ".path")) {
    if (!value)
    return config_error_nonbool(var);
    return add_man_viewer_path(name, subkey - name, value);
    }
    if (!strcmp(subkey, ".cmd")) {
    if (!value)
    return config_error_nonbool(var);
    return add_man_viewer_cmd(name, subkey - name, value);
    }
    pr_warning("'%s': unsupported man viewer sub key.", subkey);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn perf_help_config(var: *const c_char, value: *const c_char, cb: *mut c_void) -> c_int {
    static int perf_help_config(const char *var, const char *value, void *cb)
    {
    enum help_format *help_formatp = cb;
    if (!strcmp(var, "help.format")) {
    if (!value)
    return config_error_nonbool(var);
// help_formatp = parse_help_format(value);
    if (*help_formatp == HELP_FORMAT_NONE)
    return -1;
    return 0;
    }
    if (!strcmp(var, "man.viewer")) {
    if (!value)
    return config_error_nonbool(var);
    add_man_viewer(value);
    return 0;
    }
    if (strstarts(var, "man."))
    return add_man_viewer_info(var, value);
    return 0;
    }
    static struct cmdnames main_cmds, other_cmds;
#[no_mangle]
pub unsafe extern "C" fn list_common_cmds_help() {
    void list_common_cmds_help(void)
    {
    const struct cmdname_help {
    const char *name;
    const char *help;
    } common_cmds[] = {
    {"annotate", "Read perf.data (created by perf record) and display annotated code"},
    {"archive",
    "Create archive with object files with build-ids found in perf.data file"},
    {"bench", "General framework for benchmark suites"},
    {"buildid-cache", "Manage build-id cache."},
    {"buildid-list", "List the buildids in a perf.data file"},
    {"c2c", "Shared Data C2C/HITM Analyzer."},
    {"config", "Get and set variables in a configuration file."},
    {"daemon", "Run record sessions on background"},
    {"data", "Data file related processing"},
    {"diff", "Read perf.data files and display the differential profile"},
    {"evlist", "List the event names in a perf.data file"},
    {"ftrace", "simple wrapper for kernel's ftrace functionality"},
    {"inject", "Filter to augment the events stream with additional information"},
    {"iostat", "Show I/O performance metrics"},
    {"kallsyms", "Searches running kernel for symbols"},
    {"kvm", "Tool to trace/measure kvm guest os"},
    {"list", "List all symbolic event types"},
    {"mem", "Profile memory accesses"},
    {"record", "Run a command and record its profile into perf.data"},
    {"report", "Read perf.data (created by perf record) and display the profile"},
    {"script", "Read perf.data (created by perf record) and display trace output"},
    {"stat", "Run a command and gather performance counter statistics"},
    {"test", "Runs sanity tests."},
    {"top", "System profiling tool."},
    {"version", "display the version of perf binary"},

    {"probe", "Define new dynamic tracepoints"},

    {"trace", "strace inspired tool"},
    {"kmem", "Tool to trace/measure kernel memory properties"},
    {"kwork", "Tool to trace/measure kernel work properties (latencies)"},
    {"lock", "Analyze lock events"},
    {"sched", "Tool to trace/measure scheduler properties (latencies)"},
    {"timechart", "Tool to visualize total system behavior during a workload"},

    };
    let mut longest: usize = 0;
    for (size_t i = 0; i < ARRAY_SIZE(common_cmds); i++) {
    if (longest < strlen(common_cmds[i].name))
    longest = strlen(common_cmds[i].name);
    }
    puts(" The most commonly used perf commands are:");
    for (size_t i = 0; i < ARRAY_SIZE(common_cmds); i++) {
    printf("   %-*s   ", (int)longest, common_cmds[i].name);
    puts(common_cmds[i].help);
    }
    }
    static const char *cmd_to_page(const char *perf_cmd)
    {
    char *s;
    if (!perf_cmd)
    return "perf";
#[no_mangle]
pub unsafe extern "C" fn if(_arg: strstarts(perf_cmd, _arg: "perf")) -> else {
    else if (strstarts(perf_cmd, "perf"))
    return perf_cmd;
    return asprintf(&s, "perf-%s", perf_cmd) < 0 ? core::ptr::null_mut() : s;
    }
#[no_mangle]
unsafe extern "C" fn setup_man_path() {
    static void setup_man_path(void)
    {
    char *new_path;
    const char *old_path = getenv("MANPATH");
// We should always put ':' after our path. If there is no
// old_path, the ':' at the end will let 'man' to try
// system-wide paths after ours to find the manual page. If
// there is old_path, we need ':' as delimiter.
    if (asprintf(&new_path, "%s:%s", system_path(PERF_MAN_PATH), old_path ?: "") > 0) {
    setenv("MANPATH", new_path, 1);
    free(new_path);
    } else {
    pr_err("Unable to setup man path");
    }
    }
#[no_mangle]
unsafe extern "C" fn exec_viewer(name: *const c_char, page: *const c_char) {
    static void exec_viewer(const char *name, const char *page)
    {
    const char *info = get_man_viewer_info(name);
    if (!strcasecmp(name, "man"))
    exec_man_man(info, page);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcasecmp(name, _arg: "woman")) -> else {
    else if (!strcasecmp(name, "woman"))
    exec_woman_emacs(info, page);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcasecmp(name, _arg: "konqueror")) -> else {
    else if (!strcasecmp(name, "konqueror"))
    exec_man_konqueror(info, page);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: info) -> else {
    else if (info)
    exec_man_cmd(info, page);
    else
    pr_warning("'%s': unknown man viewer.", name);
    }
#[no_mangle]
unsafe extern "C" fn show_man_page(perf_cmd: *const c_char) -> c_int {
    static int show_man_page(const char *perf_cmd)
    {
    struct man_viewer_list *viewer;
    const char *page = cmd_to_page(perf_cmd);
    const char *fallback = getenv("PERF_MAN_VIEWER");
    setup_man_path();
    for (viewer = man_viewer_list; viewer; viewer = viewer.next)
    exec_viewer(viewer.name, page); /* will return when unable */
    if (fallback)
    exec_viewer(fallback, page);
    exec_viewer("man", page);
    pr_err("no man viewer handled the request");
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn show_info_page(perf_cmd: *const c_char) -> c_int {
    static int show_info_page(const char *perf_cmd)
    {
    const char *page = cmd_to_page(perf_cmd);
    setenv("INFOPATH", system_path(PERF_INFO_PATH), 1);
    execlp("info", "info", "perfman", page, core::ptr::null_mut());
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn get_html_page_path(page_path: *mut c_char, page: *const c_char) -> c_int {
    static int get_html_page_path(char **page_path, const char *page)
    {
    struct stat st;
    const char *html_path = system_path(PERF_HTML_PATH);
    char path[PATH_MAX];
// Check that we have a perf documentation directory.
    if (stat(mkpath(path, sizeof(path), "%s/perf.html", html_path), &st)
    || !S_ISREG(st.st_mode)) {
    pr_err("'%s': not a documentation directory.", html_path);
    return -1;
    }
    return asprintf(page_path, "%s/%s.html", html_path, page);
    }
//
// If open_html is not defined in a platform-specific way (see for
// example compat/mingw.h), we use the script web--browse to display
// HTML.
//

#[no_mangle]
unsafe extern "C" fn open_html(path: *const c_char) {
    static void open_html(const char *path)
    {
    execl_cmd("web--browse", "-c", "help.browser", path, core::ptr::null_mut());
    }

#[no_mangle]
unsafe extern "C" fn show_html_page(perf_cmd: *const c_char) -> c_int {
    static int show_html_page(const char *perf_cmd)
    {
    const char *page = cmd_to_page(perf_cmd);
    char *page_path; /* it leaks but we exec below */
    if (get_html_page_path(&page_path, page) < 0)
    return -1;
    open_html(page_path);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cmd_help(argc: c_int, argv: *const c_char) -> c_int {
    int cmd_help(int argc, const char **argv)
    {
    let mut show_all: bool = false;
    let mut help_format: enum help_format = HELP_FORMAT_MAN;
    struct option builtin_help_options[] = {
    OPT_BOOLEAN('a', "all", &show_all, "print all available commands"),
    OPT_SET_UINT('m', "man", &help_format, "show man page", HELP_FORMAT_MAN),
    OPT_SET_UINT('w', "web", &help_format, "show manual in web browser",
    HELP_FORMAT_WEB),
    OPT_SET_UINT('i', "info", &help_format, "show info page",
    HELP_FORMAT_INFO),
    OPT_END(),
    };
    const char * const builtin_help_subcommands[] = {
    "buildid-cache", "buildid-list", "diff", "evlist", "help", "list",
    "record", "report", "bench", "stat", "timechart", "top", "annotate",
    "script", "sched", "kallsyms", "kmem", "lock", "kvm", "test", "inject", "mem", "data",

    "probe",

    "trace",
    core::ptr::null_mut() };
    const char *builtin_help_usage[] = {
    "perf help [--all] [--man|--web|--info] [command]",
    core::ptr::null_mut()
    };
    int rc;
    load_command_list("perf-", &main_cmds, &other_cmds);
    rc = perf_config(perf_help_config, &help_format);
    if (rc)
    return rc;
    argc = parse_options_subcommand(argc, argv, builtin_help_options,
    builtin_help_subcommands, builtin_help_usage, 0);
    if (show_all) {
    printf("\n Usage: %s\n\n", perf_usage_string);
    list_commands("perf commands", &main_cmds, &other_cmds);
    printf(" %s\n\n", perf_more_info_string);
    return 0;
    }
    if (!argv[0]) {
    printf("\n usage: %s\n\n", perf_usage_string);
    list_common_cmds_help();
    printf("\n %s\n\n", perf_more_info_string);
    return 0;
    }
    switch (help_format) {
    case HELP_FORMAT_MAN:
    rc = show_man_page(argv[0]);
    break;
    case HELP_FORMAT_INFO:
    rc = show_info_page(argv[0]);
    break;
    case HELP_FORMAT_WEB:
    rc = show_html_page(argv[0]);
    break;
    case HELP_FORMAT_NONE:
// fall-through
    default:
    rc = -1;
    break;
    }
    return rc;
    }
