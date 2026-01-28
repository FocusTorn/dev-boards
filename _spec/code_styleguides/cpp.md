# C++ / Arduino / ESP-IDF Style Guide

## 1. Modern C++ Standards
- **Standard:** Use C++11 or newer (ESP-IDF defaults to C++23).
- **Zero-Cost Abstractions:** Use templates and constexpr for compile-time logic.
- **Memory:** Avoid dynamic allocation (
ew/delete) in high-frequency loops. Prefer static or stack allocation.
- **Safety:** Use std::array instead of C-style arrays where possible.

## 2. Arduino (.ino) Specifics
- **File Structure:** Comments -> Headers -> Constants -> Globals -> setup() -> loop() -> Subroutines.
- **Event Loop:** Treat loop() as a non-blocking event loop. Do not use delay(); use non-blocking timers.
- **Naming:** CamelCase for functions (e.g., processInput()), uppercase for constants.

## 3. ESP-IDF Specifics
- **C++ Integration:** Use extern "C" guards in headers. Rename main file to .cpp and use extern "C" void app_main().
- **Error Handling:** Use ESP-IDF's esp_err_t check macros, but prefer C++ exceptions for truly rare, exceptional cases if enabled.
- **Components:** Organize logic into CMake components.

## 4. Documentation
- **Comments:** Explain the "why" behind register manipulation or timing-sensitive code.
- **Naming:** Expressive naming is mandatory. Avoid abbreviations like al or cnt.
