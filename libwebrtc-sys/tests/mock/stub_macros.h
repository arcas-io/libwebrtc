#pragma once

#define STUB_BOX(X)                                                                                                                                  \
    template<>                                                                                                                                       \
    X* rust::cxxbridge1::Box<X>::allocation::alloc() noexcept                                                                                        \
    {                                                                                                                                                \
        return new X{};                                                                                                                              \
    }                                                                                                                                                \
    template<>                                                                                                                                       \
    void rust::cxxbridge1::Box<X>::allocation::dealloc(X* x) noexcept                                                                                \
    {                                                                                                                                                \
        delete x;                                                                                                                                    \
    }                                                                                                                                                \
    template<>                                                                                                                                       \
    void rust::cxxbridge1::Box<X>::drop() noexcept                                                                                                   \
    {                                                                                                                                                \
    }

#define STUB_VEC(X)                                                                                                                                  \
    template<>                                                                                                                                       \
    rust::cxxbridge1::Vec<X>::Vec()                                                                                                                  \
    {                                                                                                                                                \
    }                                                                                                                                                \
    template<>                                                                                                                                       \
    void rust::cxxbridge1::Vec<X>::drop() noexcept                                                                                                   \
    {                                                                                                                                                \
    }                                                                                                                                                \
    template<>                                                                                                                                       \
    std::size_t rust::cxxbridge1::Vec<X>::size() const noexcept                                                                                      \
    {                                                                                                                                                \
        return 0UL;                                                                                                                                  \
    }                                                                                                                                                \
    template<>                                                                                                                                       \
    void rust::cxxbridge1::Vec<X>::reserve_total(std::size_t) noexcept                                                                               \
    {                                                                                                                                                \
        std::abort();                                                                                                                                \
    }                                                                                                                                                \
    template<>                                                                                                                                       \
    void rust::cxxbridge1::Vec<X>::set_len(std::size_t) noexcept                                                                                     \
    {                                                                                                                                                \
        std::abort();                                                                                                                                \
    }                                                                                                                                                \
    template<>                                                                                                                                       \
    X const* rust::cxxbridge1::Vec<X>::data() const noexcept                                                                                         \
    {                                                                                                                                                \
        return nullptr;                                                                                                                              \
    }

#define STUB_STRUCT_BOX(X)                                                                                                                           \
    struct X                                                                                                                                         \
    {                                                                                                                                                \
    };                                                                                                                                               \
    STUB_BOX(X)
