
#include "gol.h"

void naive_step(int cells[height][width], int cells_tmp[height][width])
{
    for (int i = 1; i < height - 1; ++i)
    {
        for (int j = 1; j < width - 1; ++j)
        {
            int n_neighbours_alive = 0;

            n_neighbours_alive += cells[i - 1][j - 1];
            n_neighbours_alive += cells[i - 1][j];
            n_neighbours_alive += cells[i - 1][j + 1];
            n_neighbours_alive += cells[i][j - 1];
            n_neighbours_alive += cells[i][j + 1];
            n_neighbours_alive += cells[i + 1][j - 1];
            n_neighbours_alive += cells[i + 1][j];
            n_neighbours_alive += cells[i + 1][j + 1];

            cells_tmp[i][j] = n_neighbours_alive == 3 || (cells[i][j] && n_neighbours_alive == 2);
        }
    }
    memcpy(cells, cells_tmp, height * width * sizeof(int));
}
