#include <stdlib.h>

volatile int do_abort = 1;
int main() {
    if (do_abort) {
        abort();
    }
    return 0;
}
