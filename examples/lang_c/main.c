#include <stdio.h>
#include "servicepoint2.h"

int main(void) {
    sp2_Connection *connection = sp2_connection_open("localhost:2342");
    if (connection == NULL)
        return 1;

    sp2_Command *command = sp2_command_clear();
    if (command == NULL)
        return 2;
    if (!sp2_connection_send(connection, command))
        return 3;

    sp2_PixelGrid *pixels = sp2_pixel_grid_new(sp2_PIXEL_WIDTH, sp2_PIXEL_HEIGHT);
    sp2_pixel_grid_fill(pixels, true);
    command = sp2_command_bitmap_linear_win(0, 0, pixels);
    if (command == NULL)
        return 4;
    if (!sp2_connection_send(connection, command))
        return 5;

    sp2_connection_dealloc(connection);
    return 0;
}
