#include <stdio.h>

int main(int argc, char **argv)
{
    #ifdef _WIN32
    printf("Windows\n");
    #elif __APPLE__
    printf("Mac OS\n");
    #elif __unix__
    printf("Linux\n");
    #endif

    return 0;
}
