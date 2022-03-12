#pragma once

#ifndef __has_feature
    #define __has_feature(X) 0
#endif

#ifdef __GXX_RTTI
    #define HAS_RTTI 1
#else
    #define HAS_RTTI __has_feature(cxx_rtti)
#endif

#if HAS_RTTI
    //If its safety depends on the type of build you're doing, better to name it unsafe
    //since 'safe' code should be safe in all build variants
    #define unsafe_downcast dynamic_cast
#else
    #define unsafe_downcast reinterpret_cast
#endif
