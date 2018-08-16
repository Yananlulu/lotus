cmake_minimum_required(VERSION 3.11)
project(lotus)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_FLAGS_RELEASE "${CMAKE_CXX_FLAGS_RELEASE} -O3 -DNDEBUG")
set(CMAKE_EXE_LINKER_FLAGS_RELEASE "${CMAKE_EXE_LINKER_FLAGS_RELEASE} -s")

set(LOTUS_VERSION_MAJOR 1)
set(LOTUS_VERSION_MINOR 0)
set(LOTUS_VERSION_PATCH 0)

execute_process(COMMAND git rev-parse --short HEAD OUTPUT_STRIP_TRAILING_WHITESPACE OUTPUT_VARIABLE LOTUS_VERSION_GIT)
execute_process(COMMAND date -R OUTPUT_STRIP_TRAILING_WHITESPACE OUTPUT_VARIABLE LOTUS_BUILD_TIME)
execute_process(COMMAND git config --get user.name OUTPUT_STRIP_TRAILING_WHITESPACE OUTPUT_VARIABLE LOTUS_AUTHOR_NAME)
execute_process(COMMAND git config --get user.email OUTPUT_STRIP_TRAILING_WHITESPACE OUTPUT_VARIABLE LOTUS_AUTHOR_EMAIL)
execute_process(COMMAND head -n 1 ${PROJECT_SOURCE_DIR}/LICENSE OUTPUT_STRIP_TRAILING_WHITESPACE OUTPUT_VARIABLE LOTUS_COPYRIGHT)
execute_process(COMMAND sed -n 3p ${PROJECT_SOURCE_DIR}/README.md OUTPUT_STRIP_TRAILING_WHITESPACE OUTPUT_VARIABLE LOTUS_DESCRIPTION)

find_package(Poco REQUIRED Crypto Data DataODBC Encodings Foundation JSON Net Redis Util XML Zip)

include_directories(${CMAKE_CURRENT_LIST_DIR}/src)
aux_source_directory(${CMAKE_CURRENT_LIST_DIR}/src DIR_SRC)
aux_source_directory(${CMAKE_CURRENT_LIST_DIR}/src/plugins DIR_SRC_PLUGINS)
aux_source_directory(${CMAKE_CURRENT_LIST_DIR}/src/plugins/ops DIR_SRC_PLUGINS_OPS)

configure_file(
  "${PROJECT_SOURCE_DIR}/src/env.h.in"
  "${PROJECT_SOURCE_DIR}/src/env.h"
)

add_executable(${PROJECT_NAME} ${DIR_SRC} ${DIR_SRC_PLUGINS} ${DIR_SRC_PLUGINS_OPS})
target_link_libraries(
  ${PROJECT_NAME}
  pqxx odbc odbccr odbcinst
  rabbitmq
  sodium
  Poco::Crypto Poco::Data Poco::DataODBC Poco::Encodings Poco::Foundation Poco::JSON Poco::Net Poco::Redis Poco::Util Poco::XML Poco::Zip
)
