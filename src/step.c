
#include "gol.h"

void step(int cells[height][width], int cells_tmp[height][width], int changes[height * width + 1][2], int changes_tmp[height * width + 1][2])
{
    memcpy(cells_tmp, cells, height * width * sizeof(int));

    int changes_tmp_count = 0;

    for (int ci = 0; ci < height * width; ++ci)
    {
        if (changes[ci][0] == -1)
            break ;

        int i = changes[ci][0];
        int j = changes[ci][1];

        for (int k = 0; k < 3; ++k)
        {
            for (int l = 0; l < 3; ++l)
            {
                int m = i - 1 + k;
                int n = j - 1 + l;

                if (!(m < 0 || n < 0 || m >= height || n >= width))
                {
                    int count = (cells[m][n] - (cells[m][n] % 2)) / 2;
                    if (cells[m][n] % 2 && cells_tmp[m][n] % 2)
                    {
                        if (count != 2 && count != 3)
                        {
                            // Make a live cell come dead.
                            cells_tmp[m - 1][n - 1] -= 2;
                            cells_tmp[m - 1][n] -= 2;
                            cells_tmp[m - 1][n + 1] -= 2;
                            cells_tmp[m][n - 1] -= 2;
                            cells_tmp[m][n] -= 1;
                            cells_tmp[m][n + 1] -= 2;
                            cells_tmp[m + 1][n - 1] -= 2;
                            cells_tmp[m + 1][n] -= 2;
                            cells_tmp[m + 1][n + 1] -= 2;
                            
                            changes_tmp[changes_tmp_count][0] = m;
                            changes_tmp[changes_tmp_count][1] = n;
                            changes_tmp[changes_tmp_count + 1][0] = -1;
                            ++changes_tmp_count;
                        }
                    }
                    else if ((cells_tmp[m][n] % 2) == 0 && count == 3)
                    {
                        // Make a dead cell come alive.
                        cells_tmp[m - 1][n - 1] += 2;
                        cells_tmp[m - 1][n] += 2;
                        cells_tmp[m - 1][n + 1] += 2;
                        cells_tmp[m][n - 1] += 2;
                        cells_tmp[m][n] += 1;
                        cells_tmp[m][n + 1] += 2;
                        cells_tmp[m + 1][n - 1] += 2;
                        cells_tmp[m + 1][n] += 2;
                        cells_tmp[m + 1][n + 1] += 2;

                        changes_tmp[changes_tmp_count][0] = m;
                        changes_tmp[changes_tmp_count][1] = n;
                        changes_tmp[changes_tmp_count + 1][0] = -1;
                        ++changes_tmp_count;
                    }
                }
            }
        }
    }
    memcpy(cells, cells_tmp, height * width * sizeof(int));
    memcpy(changes, changes_tmp, (height * width + 1) * 2 * sizeof(int));
}
