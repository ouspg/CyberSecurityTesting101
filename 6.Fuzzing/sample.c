#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(int argc, char *argv[]) {
    char buffer[50]; // Fixed-size buffer
    FILE *file;

    if (argc > 1) {
        // Command-line argument provided, assumed to be a file path
        file = fopen(argv[1], "r"); // Open the file for reading
        if (file == NULL) {
            perror("Error opening file");
            return 1;
        }
        fscanf(file, "%s", buffer); // Read one string from file into buffer, still potentially unsafe due to not using length checks
        fclose(file); // Close the file
        printf("File content: %s\n", buffer);
    } else {
        // No command-line argument, read from stdin using scanf() in an insecure way
        printf("Enter your message: ");
        scanf("%s", buffer); // Single string copied to limited-size buffer, vulnerable  
        printf("You entered: %s\n", buffer);
    }

    return 0;
}

