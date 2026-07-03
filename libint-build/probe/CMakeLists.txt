# probe/CMakeLists.txt
cmake_minimum_required(VERSION 3.15)
project(probe)
find_package(Libint2 REQUIRED)

file(GENERATE OUTPUT "${CMAKE_BINARY_DIR}/flags.txt" CONTENT
"INCLUDE_DIRS=$<TARGET_PROPERTY:Libint2::cxx,INTERFACE_INCLUDE_DIRECTORIES>
STATIC_LIB=$<TARGET_LINKER_FILE:Libint2::int-static>
LIBINT2_VERSION=$<TARGET_PROPERTY:Libint2::int2,Libint2_VERSION>
")
