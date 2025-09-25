#include <stdio.h>
#include <stdbool.h>

const char INPUT[]  = "input.txt";

int main() {
	FILE *fptr = fopen(INPUT, "r");

	char ch;

	int floor = 0;
	int counter = 0;
	bool reached_basement = false;

	while ((ch = fgetc(fptr)) != EOF){
		counter += 1;
		if (ch == '(') {
			floor += 1;
		} else if (ch == ')') {
			floor -= 1;
			if (reached_basement == false && floor == -1) {
				printf("First reached basement at position: %d\n", counter); 
				reached_basement = true;
			}
			
		}
	}
	printf("Final floor: %d\n", floor); 
	fclose(fptr);
}
