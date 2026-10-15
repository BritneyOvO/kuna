#!/usr/bin/env python3
"""Regenerate the NAMES rows of kuna-analysis/src/analyzers/protos/kuna_libcnames.rs.

usage: python3 gen_names.py > rows.rs   (needs clang and the glibc headers)

Parameter names are the glibc header declarations with leading underscores
stripped: the plain headers first, the _FORTIFY_SOURCE ones for the entry
points only those declare. __isoc99_X, __X_chk and __X_2 take X's names for
the slots they share with it. A parameter pointing to a type in BY_TYPE takes
that name instead: the headers call such an object by its pointer or a generic
buffer (`buf`, `utmp_ptr`), which says nothing about the object a caller
passes the address of. Each row also spells what each parameter points to,
one letter a slot (`kinds`): `m` memory or text (`void *`, `char *`), `p`
another pointer, `o` any other object, `-` not a pointer. Return names are
the RET table below.
"""
import json, os, re, subprocess, sys

HERE = os.path.dirname(os.path.abspath(__file__))
PROTOS = os.path.join(HERE, '../../../decompiler/crates/kuna-analysis/src/analyzers/protos/')
SIG = re.compile(r'\("(\w+)", Sig \{ ret: [^,]+(?:\([^)]*\))?, params: &\[([^\]]*)\], vararg: (-?\d+) \}\)')
C_KEYWORDS = set('auto break case char const continue default do double else enum extern float for goto if inline int '
                 'long register restrict return short signed sizeof static struct switch typedef union unsigned void '
                 'volatile while _Bool _Complex _Imaginary'.split())
BY_TYPE = {'struct stat': 'st', 'struct stat64': 'st', 'struct utmp': 'ut', 'struct utmpx': 'ut'}
RET = {
    'fd': 'open open64 openat openat64 creat creat64 socket accept accept4 dup dup2 dup3 fileno fileno_unlocked '
          'mkstemp mkstemp64 mkostemp mkostemp64 __open_2 __open64_2 __openat_2 __openat64_2 epoll_create '
          'epoll_create1 inotify_init inotify_init1 eventfd timerfd_create signalfd memfd_create posix_openpt dirfd',
    'n': 'read write pread pread64 pwrite pwrite64 recv recvfrom recvmsg send sendto sendmsg readv writev readlink '
         'readlinkat fread fwrite fread_unlocked fwrite_unlocked __read_chk __pread_chk __recv_chk __readlink_chk '
         'getline getdelim __getdelim',
    'len': 'strlen strnlen wcslen wcsnlen',
    'p': 'malloc calloc realloc reallocarray aligned_alloc memalign valloc pvalloc strchr strrchr memchr memrchr '
         'rawmemchr strstr strcasestr strpbrk strchrnul',
    'fp': 'fopen fopen64 fdopen freopen freopen64 popen tmpfile tmpfile64 fmemopen open_memstream setmntent',
    'pid': 'fork vfork getpid getppid waitpid wait wait3 wait4',
    'tid': 'gettid',
    'dirp': 'opendir fdopendir',
    'entry': 'readdir readdir64',
    'c': 'getc fgetc getchar getc_unlocked fgetc_unlocked getchar_unlocked _IO_getc',
    'copy': 'strdup strndup',
    'value': 'getenv secure_getenv',
    'now': 'time',
    'pos': 'ftell ftello ftello64 lseek lseek64',
    'pw': 'getpwnam getpwuid getpwent',
    'gr': 'getgrnam getgrgid getgrent',
    'sp': 'getspnam getspent',
    'tp': 'localtime gmtime',
    'uid': 'getuid geteuid',
    'gid': 'getgid getegid',
    'addr': 'mmap mmap64',
    'status': 'system',
}


def arities():
    out = {}
    for f in ('mod.rs', 'kuna_libcsigs.rs', 'kuna_libctypes.rs'):
        for m in SIG.finditer(open(os.path.join(PROTOS, f)).read()):
            out[m.group(1)] = len(re.findall(r'Ty::', m.group(2)))
    return out


def pointee(qual):
    m = re.fullmatch(r'(?:const )?(struct \w+) \*(?:\s*restrict)?', qual.strip())
    return m.group(1) if m else ''


def kind(qual):
    t = re.sub(r'\b(const|volatile|restrict|__restrict)\b', ' ', qual)
    t = re.sub(r'\s+', ' ', t).strip()
    if not t.endswith('*'):
        return '-'
    inner = t[:-1].strip()
    if inner.endswith('*'):
        return 'p'
    return 'm' if inner in ('void', 'char', 'signed char', 'unsigned char') else 'o'


def declarations(*flags):
    ast = json.loads(subprocess.run(['clang', *flags, '-fsyntax-only', '-Xclang', '-ast-dump=json',
                                     os.path.join(HERE, 'headers.c')], capture_output=True, check=True).stdout)
    best = {}
    for d in ast.get('inner', []):
        if d.get('kind') != 'FunctionDecl':
            continue
        ps, ks = [], []
        for p in d.get('inner', []):
            if p.get('kind') != 'ParmVarDecl':
                continue
            ty = p.get('type', {})
            ps.append(BY_TYPE.get(pointee(ty.get('qualType', '')), p.get('name', '')))
            ks.append(kind(ty.get('desugaredQualType', ty.get('qualType', ''))))
        score = (all(ps), sum(1 for p in ps if p))
        if d['name'] not in best or score > best[d['name']][0]:
            best[d['name']] = (score, ps, ks)
    return {k: (v[1], v[2]) for k, v in best.items()}


def main():
    plain, fortify = declarations(), declarations('-D_FORTIFY_SOURCE=2', '-O2')
    clean = lambda p: '' if not p.lstrip('_') or p.lstrip('_') in C_KEYWORDS else p.lstrip('_')

    def header(fn):
        for src in (plain, fortify):
            if any(src.get(fn, ([], []))[0]):
                return [clean(p) for p in src[fn][0]], src[fn][1]
        return None, None
    ret = {fn: name for name, fns in RET.items() for fn in fns.split()}
    for fn, n in sorted(arities().items()):
        names, kinds = header(fn)
        base = re.fullmatch(r'__isoc99_(\w+)|__(\w+)_chk|__(\w+)_2', fn)
        if names is None and base:
            names, kinds = header(next(g for g in base.groups() if g))
        names = ((names or []) + [''] * n)[:n]
        kinds = ''.join(((kinds or []) + ['-'] * n)[:n])
        if any(names) or fn in ret:
            r = 'Some("%s")' % ret[fn] if fn in ret else 'None'
            print('    ("%s", LibraryNames { params: &[%s], kinds: "%s", ret: %s }),'
                  % (fn, ', '.join('"%s"' % p for p in names), kinds, r))


if __name__ == '__main__':
    main()
