/* libcnames fixture: stack locals whose address a library call is passed.
 * gcc -O2 -fno-inline -fno-stack-protector -o libcnames_addr_x86_64 libcnames_addr_x86_64.c */
#include <fcntl.h>
#include <stdlib.h>
#include <string.h>
#include <sys/select.h>
#include <sys/stat.h>
#include <sys/wait.h>
#include <time.h>
#include <unistd.h>
#include <wchar.h>

long file_size(const char *path)
{
    struct stat info;
    if (stat(path, &info) != 0)
        return -1;
    return info.st_size;
}

int same_file(const char *a, const char *b)
{
    struct stat sa, sb;
    if (stat(a, &sa) != 0 || stat(b, &sb) != 0)
        return 0;
    return sa.st_ino == sb.st_ino && sa.st_dev == sb.st_dev;
}

long parse(const char *text)
{
    char *end;
    long value = strtol(text, &end, 10);
    return *end ? -1 : value;
}

int hour_now(void)
{
    time_t now;
    struct tm parts;
    time(&now);
    localtime_r(&now, &parts);
    return parts.tm_hour;
}

int spawn(void)
{
    int fds[2], status;
    char c = 0;
    if (pipe(fds) != 0)
        return -1;
    pid_t child = fork();
    if (child == 0) {
        close(fds[0]);
        _exit(write(fds[1], "x", 1) != 1);
    }
    close(fds[1]);
    ssize_t got = read(fds[0], &c, 1);
    close(fds[0]);
    waitpid(child, &status, 0);
    return got + c + WEXITSTATUS(status);
}

void fill(long *out)
{
    *out = 42;
}

long reused(const char *text)
{
    char *end;
    long value = strtol(text, &end, 10);
    fill((long *)&end);
    return value + (long)end;
}

int fresh(const char *s, size_t n)
{
    mbstate_t state;
    memset(&state, 0, sizeof state);
    return mbsinit(&state) + (int)mbrlen(s, n, &state);
}

int ready(int fd)
{
    fd_set set;
    struct timeval wait;
    FD_ZERO(&set);
    FD_SET(fd, &set);
    wait.tv_sec = 1;
    wait.tv_usec = 0;
    return select(fd + 1, &set, NULL, NULL, &wait);
}

struct header {
    unsigned int magic;
    unsigned short version;
    unsigned char width, height;
    unsigned int count;
};

void setup(struct header *h)
{
    h->count = h->width * h->height;
}

int load(const char *path)
{
    struct header h = {0};
    struct stat st;
    int fd = open(path, O_RDONLY);
    if (fd < 0 || fstat(fd, &st) != 0)
        return -1;
    if (read(fd, &h, 8) != 8 || h.magic != 0x52444845 || h.version != 3) {
        close(fd);
        return 1;
    }
    setup(&h);
    close(fd);
    return h.width * h.height + h.count + (int)st.st_size;
}

int main(int argc, char **argv)
{
    if (argc < 3)
        return 2;
    return (int)file_size(argv[1]) + same_file(argv[1], argv[2]) + (int)parse(argv[2]) + hour_now() + spawn()
        + (int)reused(argv[2]) + fresh(argv[1], 4) + ready(argc) + load(argv[1]);
}
