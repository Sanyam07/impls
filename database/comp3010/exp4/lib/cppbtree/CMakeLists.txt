file(GLOB_RECURSE EXTMEMFS_SOURCES ./*.h)
add_library(cppbtree INTERFACE)
target_sources(cppbtree INTERFACE btree.h btree_map.h btree_set.h)
#set(CMAKE_CXX_FLAGS "-g -O2")
target_include_directories(cppbtree INTERFACE .)
