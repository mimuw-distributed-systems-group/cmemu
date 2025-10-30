#include <time.h>
#include <sys/times.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/unistd.h>
#include <assert.h>

// Test how to reserve high SRAM addresses this from C.
uint8_t  __attribute__((section (".highram_reserve.scratch"), used)) res[256];

// How to put stuff in GPRAM
uint8_t  __attribute__((section (".gpram.data"), used)) gp_data[8] = {1, 7, 13, 33};

int main(int argc, char **argv, char **main_env) {
    fprintf(stderr, "Argc: %d, args:\n", argc);
    if (argv) {
        for (char **argv2 = argv; *argv2; argv2++) {
            fprintf(stderr, "%p: %s\n", *argv2, *argv2);
        }
    }
    if (environ) {
        assert(environ == main_env);
        fprintf(stderr, "Env:\n");
        for (char **environ2 = environ; *environ2; environ2++) {
            fprintf(stderr, "%p: %s\n", *environ2, *environ2);
        }
    }
    fprintf(stderr, "Locale: %s\n", getenv("LC_ALL"));


    // Note: clock() is CYCCNT
    clock_t start = clock();
    printf("Clock now: %ld\n", start);
    time_t res = time(NULL);
    printf("Current time: %s\n", ctime(&res));

    clock_t end = clock();
    printf("Took: %ld units (at %ld)\n", end - start, end);

    // Don't optimize out
    __asm__ volatile ("": : "r" (gp_data));

    return 0;
}
