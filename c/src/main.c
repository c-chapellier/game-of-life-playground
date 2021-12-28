
#include "gol.h"

const int height = WIN_HEIGHT + 2;
const int width = WIN_WIDTH + 2;

int scale = 0;
int n_pxl_per_cell;
int n_cells_on_screen;

static void draw_cells(SDL_Renderer *renderer, int cells[height][width]);

static uint64_t get_cpucc()
{
    unsigned int lo, hi;

    __asm__ __volatile__ ("rdtsc" : "=a" (lo), "=d" (hi));
    return ((uint64_t)hi << 32) | lo;
}

static void change_scale(int s)
{
    scale = s;
    n_pxl_per_cell = 1 << -scale;
    n_cells_on_screen = WIN_WIDTH / n_pxl_per_cell;
}

int main()
{
    SDL_Init(SDL_INIT_VIDEO);
    SDL_Window *window = SDL_CreateWindow("Game of life", SDL_WINDOWPOS_UNDEFINED, SDL_WINDOWPOS_UNDEFINED, WIN_WIDTH, WIN_HEIGHT, SDL_WINDOW_SHOWN);
    SDL_Renderer *renderer = SDL_CreateRenderer(window, -1, SDL_RENDERER_ACCELERATED);
    
    int cells[height][width] = { 0 };
    int cells_tmp[height][width] = { 0 };

    int changes[height * width + 1][2] = { 0 };
    int changes_tmp[height * width + 1][2] = { 0 };

    // for (int i = 0; i < height; ++i)
    // {
    //     for (int j = 0; j < width; ++j)
    //     {
    //         cells[i][j] = rand() % 2;
    //     }
    // }

    // cells[1][2] = ALIVE;
    // cells[1][3] = ALIVE;
    // cells[2][1] = ALIVE;
    // cells[2][2] = ALIVE;
    // cells[3][2] = ALIVE;

    cells[200 + 1][200] = ALIVE;
    cells[200 + 3][200 - 1] = ALIVE;
    cells[200][200 - 2] = ALIVE;
    cells[200 + 1][200 - 2] = ALIVE;
    cells[200 + 4][200 - 2] = ALIVE;
    cells[200 + 5][200 - 2] = ALIVE;
    cells[200 + 6][200 - 2] = ALIVE;

#if ALGORITHM == ABRASH_CHANGES
    int ci = 0;
    for (int i = 0; i < height; ++i)
    {
        for (int j = 0; j < width; ++j)
        {
            if (cells[i][j])
            {
                changes[ci][0] = i;
                changes[ci][1] = j;
                changes[ci + 1][0] = -1;
                ++ci;
            }
        }
    }
#endif

#if ALGORITHM == ABRASH || ALGORITHM == ABRASH_CHANGES
    memcpy(cells_tmp, cells, height * width * sizeof(int));
    for (int i = 0; i < height; ++i)
    {
        for (int j = 0; j < width; ++j)
        {
            for (int k = 0; k < 3; ++k)
            {
                for (int l = 0; l < 3; ++l)
                {
                    int m = i - 1 + k;
                    int n = j - 1 + l;

                    if (!(m < 0 || n < 0 || m >= height || n >= width) && (m != i || n != j))
                        cells_tmp[i][j] += cells[m][n] * 2;
                }
            }
        }
    }
    memcpy(cells, cells_tmp, height * width * sizeof(int));
#endif

    change_scale(0);

    draw_cells(renderer, cells);

    char buf;
    read(1, &buf, 1);

    int quit = 0;
    int n = 0;
    int status = STARTED;

    clock_t exec_time = 0;
    uint64_t cpucc = 0;

    while (!quit)
    {
        if (status == STARTED)
        {
            clock_t begin = clock();
            uint64_t begin_cpucc = get_cpucc();

            step(cells, cells_tmp, changes, changes_tmp);

            exec_time += clock() - begin;
            cpucc = get_cpucc() - begin_cpucc;

            draw_cells(renderer, cells);
            ++n;
        }

        SDL_Event event;
        SDL_PollEvent(&event);

        switch (event.type)
        {
        case SDL_QUIT:
            quit = 1;
            break;
        case SDL_KEYDOWN:
            switch (event.key.keysym.sym)
            {
            case SDLK_q:
                quit = 1;
                break;
            case SDLK_SPACE:
                status = !status;
                break;
            case SDLK_EQUALS:
                change_scale(scale - 1);
                break;
            case SDLK_MINUS:
                change_scale(scale + 1);
                break;
            default:
                break;
            }
            break;
        default:
            break;
        }

        if (n > 200)
        {
            break ;
        }
    }

    printf("time %fs\n", (double)exec_time / CLOCKS_PER_SEC);
    printf("cpucc %llukc\n", cpucc / 1000);

    SDL_DestroyRenderer(renderer);
    SDL_DestroyWindow(window);
    SDL_Quit();
}

static void draw_cells(SDL_Renderer *renderer, int cells[height][width])
{
    SDL_SetRenderDrawColor(renderer, 0, 0, 0, 0xFF);
    SDL_RenderClear(renderer);

    SDL_Rect rect;
    rect.w = n_pxl_per_cell;
    rect.h = n_pxl_per_cell;

    for (int i = 0; i < n_cells_on_screen; ++i)
    {
        for (int j = 0; j < n_cells_on_screen; ++j)
        {
            rect.x = n_pxl_per_cell * j;
            rect.y = WIN_HEIGHT - (n_pxl_per_cell * (i + 1));

#if ALGORITHM == ABRASH || ALGORITHM == ABRASH_CHANGES
            if (cells[2 + i][2 + j] % 2)
#else
            if (cells[2 + i][2 + j] == ALIVE)
#endif
            {
                SDL_SetRenderDrawColor(renderer, 255, 0, 0, 0xFF);
            }
            else
            {
                SDL_SetRenderDrawColor(renderer, 0, 0, 0, 0xFF);
            }

            SDL_RenderFillRect(renderer, &rect);
        }
    }
    SDL_RenderPresent(renderer);
}
