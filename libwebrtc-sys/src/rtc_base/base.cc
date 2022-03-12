#include "libwebrtc-sys/include/rtc_base/base.h"
#include "libwebrtc-sys/src/rtc_base/base.rs.h"
#include <rtc_base/physical_socket_server.h>

bool ArcasQueuedTask::Run()
{
    return _task->run();
}

bool ArcasQueuedTask::operator()()
{
    return _task->run();
}

bool ArcasQueuedTask::operator()() const
{
    return _task->run();
}

std::unique_ptr<rtc::Thread> create_arcas_cxx_thread()
{
    return rtc::Thread::Create();
}
std::unique_ptr<rtc::Thread> create_arcas_cxx_thread_with_socketserver()
{
    return rtc::Thread::CreateWithSocketServer();
}

void arcas_cxx_thread_post_task(rtc::Thread* thread, rust::Box<ArcasRustQueuedTask> task)
{
    auto queued_task = std::make_unique<ArcasQueuedTask>(std::move(task));
    thread->PostTask(RTC_FROM_HERE, [queued_task = std::move(queued_task)] { return queued_task->Run(); });
}

std::unique_ptr<rtc::NetworkManager> create_arcas_cxx_network_manager()
{
    return std::make_unique<rtc::BasicNetworkManager>();
}

void set_thread_name(rtc::Thread& thread, rust::String new_name)
{
    thread.SetName({new_name.data(), new_name.size()}, nullptr);
}
std::unique_ptr<rtc::ByteBufferReader> create_arcas_cxx_byte_buffer_reader(const char* bytes, size_t len)
{
    return std::make_unique<rtc::ByteBufferReader>(bytes, len);
}
