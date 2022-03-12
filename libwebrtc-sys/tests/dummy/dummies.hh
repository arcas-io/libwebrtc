#pragma once

#include <fcntl.h>
#include <unistd.h>

#include <cerrno>
#include <cstdio>
#include <cstdlib>
#include <cstring>

#include <string_view>

//Lots of paranoia in this file, because you're in a messed-up state and don't know what you're going to be allowed to get away with before you abort on some memory error

template<int N>
void write_literal(char const (&word)[N])
{
    ::write(1, word, N - 1);
    ::write(2, word, N - 1);
}

void write_nulterm(char const* s);
void fix_file(char const* symbol, char const* dummy_file, int line);
void report_errno(char const* context);

#define DUMMY(S)                                                                                                                                     \
    extern "C"                                                                                                                                       \
    {                                                                                                                                                \
        __attribute__((weak)) void S()                                                                                                               \
        {                                                                                                                                            \
            hit(#S, __FILE__, __LINE__);                                                                                                             \
        }                                                                                                                                            \
    }

template<int N, int M>
void hit(char const (&symbol)[N], char const (&dummy_file)[M], int dummy_line)
{
    write_literal(symbol);
    write_literal(" DUMMY HIT !!!\n Dummy was declared in file \n");
    write_literal(dummy_file);
    char buf[32];
    std::sprintf(buf, "\n at line %d \n", dummy_line);
    write_nulterm(buf);
    fix_file(symbol, dummy_file, dummy_line);
    std::sprintf(buf, "pstack %d", ::getpid());
    std::system(buf);
    std::exit(9);//Returning will DEFINITELY mess things up. Can cause you to hit things you wouldn't really.
}

template<int N>
void report_errno(char* buf, char const (&context)[N])
{
    auto e = errno;
    std::perror(context);
    write_literal(context);
    write_literal("\n Dummy functions are declared in \n");
    write_literal(__FILE__);
    std::sprintf(buf, "\n'%s' error was: (%d) %s\n", context, e, std::strerror(e));
    write_nulterm(buf);
}

inline void fix_file(char const* symbol, char const* dummy_file, int /*line*/)
{
    char buf[(1 << 19)];
    int fd = ::open(dummy_file, O_RDONLY);
    if (fd == -1)
    {
        report_errno(buf,
                     "Was unable to open the dummy file (for reading) to update it. You will need "
                     "to do it.\n");
        return;
    }
    std::memset(buf, 0, sizeof(buf));
    auto bytes = ::read(fd, buf, sizeof(buf));
    if (bytes < 0)
    {
        report_errno(buf, "Was unable to read the dummy file to update it. You will need to do it.\n");
        return;
    }
    ::close(fd);
    if (bytes >= static_cast<long>(sizeof(buf)))
    {
        write_literal(
            "\n Your dummy file is too big for me to update automatically. Figure it out "
            "yourself.\n");
        return;
    }
    std::string_view file_contents{+buf, static_cast<unsigned>(bytes)};
    char code[256];
    std::sprintf(code, "DUMMY(%s)", symbol);
    auto pos = file_contents.find(code);
    if (pos == std::string_view::npos)
    {
        write_literal(
            "Can't find the dummy for this symbol in the dummy file. Something odd is up. Perhaps "
            "it was in there, you built it. You removed it manually. And then you ran the outdated "
            "binary?\n");
        return;
    }
    buf[pos - 2] = '/';
    buf[pos - 1] = '/';
    fd = ::open(dummy_file, O_WRONLY | O_DIRECT | O_DSYNC | O_SYNC);
    if (fd == -1)
    {
        report_errno(buf,
                     "Was unable to open the dummy file (for writing) to update it. You will need "
                     "to do it.\n");
        return;
    }
    if (bytes > ::write(fd, buf, bytes))
    {
        report_errno(buf, "Was unable to write to the dummy file to update it. You will need to do it.\n");
        return;
    }
    if (::close(fd))
    {
        report_errno(buf,
                     "Was unable to ## CLOSE ?? ## the dummy file after writing to it??\n Check it "
                     "yourself ! \n");
    }
    else
    {
        write_literal("Updated the dummy file.\n");
    }
}

inline void write_nulterm(char const* s)
{
    auto len = std::strlen(s);
    ::write(1, s, len);
    ::write(2, s, len);
}
