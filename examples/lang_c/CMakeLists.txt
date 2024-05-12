cmake_minimum_required(VERSION 3.28)
project(lang_c C)
set(CMAKE_C_STANDARD 17)

include(FetchContent)
FetchContent_Declare(
        Corrosion
        GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
        GIT_TAG v0.5 # Optionally specify a commit hash, version tag or branch here
)
FetchContent_MakeAvailable(Corrosion)

# Import targets defined in a package or workspace manifest `Cargo.toml` file
corrosion_import_crate(
        MANIFEST_PATH ../../servicepoint2/Cargo.toml
        PROFILE release
        FEATURES c-api
        ALL_FEATURES)

add_executable(lang_c main.c)

target_link_libraries(lang_c PRIVATE servicepoint2)

