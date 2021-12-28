
#include "gol.h"

void abrash_step(int cells[height][width], int cells_tmp[height][width])
{
    memcpy(cells_tmp, cells, height * width * sizeof(int));

    for (int i = 1; i < height - 1; ++i)
    {
        for (int j = 1; j < width - 1; ++j)
        {
            if (cells[i][j] == 0)
                continue ;

            int count = (cells[i][j] - (cells[i][j] % 2)) / 2;
            if (cells[i][j] % 2)
            {
                if (count != 2 && count != 3)
                {
                    // Make a live cell come dead.
                    cells_tmp[i - 1][j - 1] -= 2;
                    cells_tmp[i - 1][j] -= 2;
                    cells_tmp[i - 1][j + 1] -= 2;
                    cells_tmp[i][j - 1] -= 2;
                    cells_tmp[i][j] -= 1;
                    cells_tmp[i][j + 1] -= 2;
                    cells_tmp[i + 1][j - 1] -= 2;
                    cells_tmp[i + 1][j] -= 2;
                    cells_tmp[i + 1][j + 1] -= 2;
                }
            }
            else if (count == 3)
            {
                // Make a dead cell come alive.
                cells_tmp[i - 1][j - 1] += 2;
                cells_tmp[i - 1][j] += 2;
                cells_tmp[i - 1][j + 1] += 2;
                cells_tmp[i][j - 1] += 2;
                cells_tmp[i][j] += 1;
                cells_tmp[i][j + 1] += 2;
                cells_tmp[i + 1][j - 1] += 2;
                cells_tmp[i + 1][j] += 2;
                cells_tmp[i + 1][j + 1] += 2;
            }
        }
    }
    memcpy(cells, cells_tmp, height * width * sizeof(int));
}
