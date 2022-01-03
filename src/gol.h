
#ifndef GOL_H
#define GOL_H

#include <stdio.h>
#include <unistd.h>
#include <SDL2/SDL.h>
#include <time.h>
#include <assert.h>

#define GUI 1

#define ALIVE 1
#define DEAD 0

#define STARTED 1
#define PAUSED 0

#define WIN_HEIGHT 512
#define WIN_WIDTH 512

extern const int height;
extern const int width;

void step(int cells[height][width], int changes[height * width + 1][2], int changes_tmp[height * width + 1][2]);

#endif
