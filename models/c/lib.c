#include <stdio.h>
#include "sdk.h"

// Define the functions
char* port1(char data[]) {
    printf("got data: %c\n", data);
    char myArray[] = { 0x00, 0x11, 0x22 };
    return myArray;
}

char* port2(char data[]) {
    printf("got data: %c\n", data);
    char myArray[] = { 0x00, 0x11, 0x22 };
    return myArray;
}

bool exports_taho_sdk_ports_method_callbacks_invoke_port(exports_taho_sdk_ports_borrow_callbacks_t self, sdk_string_t *port_name, sdk_list_u8_t *data, sdk_list_u8_t *ret) {
    char* (*operations[2])(char[]) = { port1, port2 };
    char myArray[] = { 0x00, 0x11, 0x22 };
   
   if (port_name == "port1") {
    ret = operations[0](myArray);
    return true;
   }
   ret = operations[1](myArray);

   return true;
}


int main() {
    // Create an array of function pointers
    char * (*operations[2])(char[]) = { port1, port2 };
    char myArray[] = { 0x00, 0x11, 0x22 };
    // Call the functions using the array
    printf("port1: %c\n", operations[0](myArray));
    printf("port2: %c\n", operations[1](myArray));

    return 0;
}