
#ifndef GOL_H
#define GOL_H

#include <stdio.h>
#include <unistd.h>
#include <SDL2/SDL.h>
#include <time.h>

#define ALIVE 1
#define DEAD 0

#define STARTED 1
#define PAUSED 0

#define WIN_HEIGHT 512
#define WIN_WIDTH 512

extern const int height;
extern const int width;

#define NAIVE 0
#define ARRAYS 1
#define ABRASH 2
#define ABRASH_CHANGES 3
#define ALGORITHM ABRASH_CHANGES

void step(int cells[height][width], int cells_tmp[height][width], int changes[height * width + 1][2], int changes_tmp[height * width + 1][2]);
void naive_step(int cells[height][width], int cells_tmp[height][width]);
void arrays_step(int cells[height][width]);
void abrash_step(int cells[height][width], int cells_tmp[height][width]);
void abrash_changes_step(int cells[height][width], int cells_tmp[height][width], int changes[height * width + 1][2], int changes_tmp[height * width + 1][2]);

#endif
