#pragma once
#include "api/task_queue/queued_task.h"
#include "rtc_base/byte_buffer.h"
#include "rtc_base/network.h"
#include "rtc_base/network_constants.h"
#include "rtc_base/ssl_adapter.h"
#include "rtc_base/thread.h"
#include "rust/cxx.h"

using ArcasCxxSSLHandshakeError = rtc::SSLHandshakeError;
using ArcasCxxNetworkManager = rtc::BasicNetworkManager;

struct ArcasRustQueuedTask;

class ArcasQueuedTask : public webrtc::QueuedTask
{
private:
    rust::Box<ArcasRustQueuedTask> _task;

public:
    ArcasQueuedTask(rust::Box<ArcasRustQueuedTask> task)
    : _task(std::move(task))
    {
    }
    ~ArcasQueuedTask() {}

    bool Run() override;

    bool operator()();
    bool operator()() const;
};

std::unique_ptr<rtc::Thread> create_arcas_cxx_thread();
void arcas_cxx_thread_post_task(rtc::Thread* thread, rust::Box<ArcasRustQueuedTask> task);
std::unique_ptr<rtc::NetworkManager> create_arcas_cxx_network_manager();
std::unique_ptr<rtc::ByteBufferReader> create_arcas_cxx_byte_buffer_reader(const char* bytes, size_t len);
std::unique_ptr<rtc::ByteBufferWriter> create_arcas_cxx_byte_buffer_writer(char* bytes, size_t len);
