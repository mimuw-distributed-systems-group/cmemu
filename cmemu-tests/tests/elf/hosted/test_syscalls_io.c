#include <stdlib.h>
#include <string.h>
#include <stdio.h>

int main() {
    int age;
    printf("What is your age?\n");
    int res = scanf("%d", &age);
    if (res == 0)
        return 1;
    printf("Your age is %d\n", age);

    return 0;
}
