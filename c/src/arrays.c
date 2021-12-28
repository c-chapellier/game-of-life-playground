
#include "gol.h"

int w[WIN_HEIGHT + 2][WIN_WIDTH + 2];
int e[WIN_HEIGHT + 2][WIN_WIDTH + 2];
int n[WIN_HEIGHT + 2][WIN_WIDTH + 2];
int s[WIN_HEIGHT + 2][WIN_WIDTH + 2];
int nw[WIN_HEIGHT + 2][WIN_WIDTH + 2];
int ne[WIN_HEIGHT + 2][WIN_WIDTH + 2];
int sw[WIN_HEIGHT + 2][WIN_WIDTH + 2];
int se[WIN_HEIGHT + 2][WIN_WIDTH + 2];

int where3[WIN_HEIGHT + 2][WIN_WIDTH + 2];
int where4[WIN_HEIGHT + 2][WIN_WIDTH + 2];
int sum[WIN_HEIGHT + 2][WIN_WIDTH + 2];
int and4[WIN_HEIGHT + 2][WIN_WIDTH + 2];

static void left(int src[height][width], int dst[height][width])
{
    for (int i = 0; i < height; ++i)
    {
        memcpy(dst[i], src[i] + 1, (width - 1) * sizeof(int));
        dst[i][width - 1] = 0;
    }
}

static void right(int src[height][width], int dst[height][width])
{
    for (int i = 0; i < height; ++i)
    {
        memcpy(dst[i] + 1, src[i], (width - 1) * sizeof(int));
        dst[i][0] = 0;
    }
}

static void up(int src[height][width], int dst[height][width])
{
    for (int i = 0; i < height - 1; ++i)
    {
        memcpy(dst[i], src[i + 1], width * sizeof(int));
    }
    memset(dst[height - 1], 0, width * sizeof(int));
}

static void down(int src[height][width], int dst[height][width])
{
    memset(dst[0], 0, width * sizeof(int));
    for (int i = 0; i < height - 1; ++i)
    {
        memcpy(dst[i + 1], src[i], width * sizeof(int));
    }
}

static void add(int cells[height][width], int w[height][width], int e[height][width], int n[height][width], int s[height][width], int nw[height][width], int ne[height][width], int sw[height][width], int se[height][width], int sum[height][width])
{
    for (int i = 0; i < height; ++i)
    {
        for (int j = 0; j < width; ++j)
        {
            sum[i][j] = cells[i][j] + w[i][j] + e[i][j] + n[i][j] + s[i][j] + nw[i][j] + ne[i][j] + sw[i][j] + se[i][j];
        }
    }
}

static void where(int src[height][width], int n, int dst[height][width])
{
    for (int i = 0; i < height; ++i)
    {
        for (int j = 0; j < width; ++j)
        {
            dst[i][j] = src[i][j] == n;
        }
    }
}

static void and(int src1[height][width], int src2[height][width], int dst[height][width])
{
    for (int i = 0; i < height; ++i)
    {
        for (int j = 0; j < width; ++j)
        {
            dst[i][j] = src1[i][j] & src2[i][j];
        }
    }
}

static void or(int src1[height][width], int src2[height][width], int dst[height][width])
{
    for (int i = 0; i < height; ++i)
    {
        for (int j = 0; j < width; ++j)
        {
            dst[i][j] = src1[i][j] | src2[i][j];
        }
    }
}

void arrays_step(int cells[height][width])
{
    left(cells, w);
    right(cells, e);
    up(cells, n);
    down(cells, s);

    up(w, nw);
    up(e, ne);
    down(w, sw);
    down(e, se);
    
    add(cells, w, e, n, s, nw, ne, sw, se, sum);
    where(sum, 4, where4);
    and(where4, cells, and4);
    where(sum, 3, where3);
    
    or(where3, and4, cells);
}
