#pragma once

#include <stdexcept>

// clang-format off
/**
 * @brief The body of an unimplemented function
 * @details This is useful to provide a simple implementation for a pure virtual in a class you're making a mock subclass of
 *      If you don't plan for that function to be called in the test.
 * @note The provided curly braces may be relied upon or not depending on whether you think it looks more clear
 *      void foo() UNIMP
 *      void foo() { UNIMP }
 *      both are valid
 */
#define UNIMP { throw std::runtime_error{std::string{__PRETTY_FUNCTION__}+" unimplemented @ "+__FILE__+std::to_string(__LINE__)}; }
// clang-format on
