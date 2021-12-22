#include <stdio.h>
#include <stdlib.h>

int main(int argc, char **argv)
{
    #ifdef _WIN32
    printf("Windows\n");
    #elif __APPLE__
    printf("Mac OS\n");
    #elif __unix__
    printf("Linux\n");
    system("ls");
    #endif

    return 0;
}
