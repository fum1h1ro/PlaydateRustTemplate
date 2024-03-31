cmake_minimum_required(VERSION 3.19)
set(CMAKE_C_STANDARD 11)
set(CMAKE_VERBOSE_MAKEFILE TRUE)
set(CMAKE_CONFIGURATION_TYPES "Debug;Release")
set(CMAKE_XCODE_GENERATE_SCHEME TRUE)

if(NOT EXISTS "${CMAKE_SOURCE_DIR}/Source/pdxinfo")
    message(FATAL_ERROR "This project must be in the root of the project")
    return()
else()
    execute_process(
        COMMAND bash -c "egrep '^\\s*name' ${CMAKE_SOURCE_DIR}/Source/pdxinfo"
        COMMAND head -n 1
        COMMAND cut -d= -f2
        COMMAND sed "s/[ \t]/_/g"
        OUTPUT_VARIABLE GAME_NAME
        OUTPUT_STRIP_TRAILING_WHITESPACE
    )
endif()

message(STATUS "GAME_NAME: ${GAME_NAME}")
set(PLAYDATE_GAME_NAME ${GAME_NAME})
set(PLAYDATE_GAME_DEVICE ${GAME_NAME})
message(STATUS "PLAYDATE_GAME_NAME: ${PLAYDATE_GAME_NAME}")
message(STATUS "PLAYDATE_GAME_DEVICE: ${PLAYDATE_GAME_DEVICE}")

project(${GAME_NAME} C ASM)
if(NOT ${TOOLCHAIN} STREQUAL "")
    message(STATUS "TOOLCHAIN: ${TOOLCHAIN}")
else()
    message(STATUS "TOOLCHAIN: clang")
endif()

# SDK path
set(ENVSDK $ENV{PLAYDATE_SDK_PATH})
if(NOT ${ENVSDK} STREQUAL "")
    # Convert path from Windows
    file(TO_CMAKE_PATH ${ENVSDK} SDK)
else()
    execute_process(
        COMMAND bash -c "egrep '^\\s*SDKRoot' $HOME/.Playdate/config"
        COMMAND head -n 1
        COMMAND cut -c9-
        OUTPUT_VARIABLE SDK
        OUTPUT_STRIP_TRAILING_WHITESPACE
    )
endif()
if(NOT EXISTS ${SDK})
    message(FATAL_ERROR "SDK Path not found; set ENV value PLAYDATE_SDK_PATH")
    return()
endif()



# ================================================================================
# main
file(GLOB SOURCE RELATIVE "${CMAKE_CURRENT_SOURCE_DIR}" "src/*.c")
message(STATUS "Main sources: ${SOURCE}")

if(TOOLCHAIN STREQUAL "armgcc")
    add_executable(${GAME_NAME} ${SDK}/C_API/buildsupport/setup.c ${SOURCE})
    target_link_libraries(${GAME_NAME} ${CMAKE_SOURCE_DIR}/target/thumbv7em-none-eabihf/debug/librusttest.a)
else()
    add_library(${GAME_NAME} SHARED ${SOURCE})
    target_link_libraries(${GAME_NAME} ${CMAKE_SOURCE_DIR}/target/debug/librusttest.a)
endif()
target_compile_options(${GAME_NAME} PUBLIC -save-temps=obj -Wall -Wconversion)

include(${SDK}/C_API/buildsupport/playdate_game.cmake)
