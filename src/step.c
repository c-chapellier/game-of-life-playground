
#include "gol.h"

void step(int cells[height][width], int changes[height * width + 1][2], int current_changes[height * width + 1][2])
{
    int current_changes_count = 0;

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
                    int count = cells[m][n] / 4;
                    int state = cells[m][n] & 1;
                    int newState = (count == 3) | ((count == 2) & state);
                    
                    if (state & (!newState))
                    {
                        current_changes[current_changes_count][0] = m;
                        current_changes[current_changes_count][1] = n;
                        current_changes[current_changes_count + 1][0] = -1;
                        ++current_changes_count;
                    }
                    else if ((!state) & newState)
                    {
                        cells[m][n] |= 2;

                        current_changes[current_changes_count][0] = m;
                        current_changes[current_changes_count][1] = n;
                        current_changes[current_changes_count + 1][0] = -1;
                        ++current_changes_count;
                    }
                }
            }
        }
    }

    int changes_count = 0;

    for (int ci = 0; ci < height * width; ++ci)
    {
        if (current_changes[ci][0] == -1)
            break ;

        int i = current_changes[ci][0];
        int j = current_changes[ci][1];

        int state = cells[i][j] & 1;
        int newState = cells[i][j] & 2;
                    
        if (newState && state == 0)
        {
            // Make a dead cell come alive.
            cells[i - 1][j - 1] += 4;
            cells[i - 1][j] += 4;
            cells[i - 1][j + 1] += 4;
            cells[i][j - 1] += 4;
            cells[i][j] += 1;
            cells[i][j + 1] += 4;
            cells[i + 1][j - 1] += 4;
            cells[i + 1][j] += 4;
            cells[i + 1][j + 1] += 4;

            changes[changes_count][0] = i;
            changes[changes_count][1] = j;
            changes[changes_count + 1][0] = -1;
            ++changes_count;
        }
        else if (newState == 0 && state)
        {
            // Make a live cell come dead.
            cells[i - 1][j - 1] -= 4;
            cells[i - 1][j] -= 4;
            cells[i - 1][j + 1] -= 4;
            cells[i][j - 1] -= 4;
            cells[i][j] -= 1;
            cells[i][j + 1] -= 4;
            cells[i + 1][j - 1] -= 4;
            cells[i + 1][j] -= 4;
            cells[i + 1][j + 1] -= 4;

            changes[changes_count][0] = i;
            changes[changes_count][1] = j;
            changes[changes_count + 1][0] = -1;
            ++changes_count;
        }
    }

    for (int ci = 0; ci < height * width; ++ci)
    {
        if (current_changes[ci][0] == -1)
            break ;

        int i = current_changes[ci][0];
        int j = current_changes[ci][1];

        cells[i][j] &= ~2;
    }
}
