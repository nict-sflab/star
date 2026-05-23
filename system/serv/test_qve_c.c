#include <stdio.h>
#include <stdlib.h>
#include <dlfcn.h>
#include <stdint.h>

int main() {
    void *handle = dlopen("libsgx_dcap_quoteverify.so", RTLD_NOW | RTLD_GLOBAL);
    if (!handle) {
        printf("dlopen failed: %s\n", dlerror());
        return 1;
    }
    uint32_t (*get_size)(uint32_t *) = dlsym(handle, "sgx_qv_get_quote_supplemental_data_size");
    if (!get_size) {
        printf("dlsym failed: %s\n", dlerror());
        return 1;
    }
    uint32_t size = 0;
    uint32_t res = get_size(&size);
    printf("get_size returned %u, size=%u\n", res, size);
    return 0;
}
