
#include "gol.h"

void step(int cells[height][width], int cells_tmp[height][width], int changes[height * width + 1][2], int changes_tmp[height * width + 1][2])
{
#if ALGORITHM == NAIVE
    naive_step(cells, cells_tmp);
#endif
#if ALGORITHM == ARRAYS
    arrays_step(cells);
#endif
#if ALGORITHM == ABRASH
    abrash_step(cells, cells_tmp);
#endif
#if ALGORITHM == ABRASH_CHANGES
    abrash_changes_step(cells, cells_tmp, changes, changes_tmp);
#endif
}
