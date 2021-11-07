#include "libwebrtc-sys/include/logging.h"

void set_arcas_log_to_stderr(bool value)
{
    rtc::LogMessage::SetLogToStderr(value);
    RTC_LOG(LS_INFO) << "Set log to stderr";
}

void set_arcas_log_level(rtc::LoggingSeverity level)
{
    rtc::LogMessage::LogToDebug(level);
    RTC_LOG(LS_INFO) << "Logging level set to " << level;
}
