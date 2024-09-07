#include <stdio.h>
#include "servicepoint.h"

int main(void) {
    SPConnection *connection = sp_connection_open("172.23.42.29:2342");
    if (connection == NULL)
        return 1;

    SPPixelGrid *pixels = sp_pixel_grid_new(SP_PIXEL_WIDTH, SP_PIXEL_HEIGHT);
    sp_pixel_grid_fill(pixels, true);

    SPCommand *command = sp_command_bitmap_linear_win(0, 0, pixels, Uncompressed);
    SPPacket *packet = sp_packet_from_command(command);
    while (sp_connection_send(connection, sp_packet_clone(packet)));

    sp_packet_free(packet);
    sp_connection_free(connection);
    return 0;
}
