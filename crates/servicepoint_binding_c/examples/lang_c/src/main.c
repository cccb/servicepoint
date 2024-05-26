#include <stdio.h>
#include "servicepoint.h"

int main(void) {
    sp_Connection *connection = sp_connection_open("localhost:2342");
    if (connection == NULL)
        return 1;

    sp_PixelGrid *pixels = sp_pixel_grid_new(sp_PIXEL_WIDTH, sp_PIXEL_HEIGHT);
    sp_pixel_grid_fill(pixels, true);

    sp_Command *command = sp_command_bitmap_linear_win(0, 0, pixels, Uncompressed);
    sp_Packet *packet = sp_packet_from_command(command);
    if (!sp_connection_send(connection, packet))
        return 1;

    sp_connection_dealloc(connection);
    return 0;
}
