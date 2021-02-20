__declspec(dllexport) void foo(void (*callback)()) {
    callback();
}