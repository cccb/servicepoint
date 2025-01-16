#include <stdio.h>
#include "servicepoint.h"

int main(void) {
    SPConnection *connection = sp_connection_open("localhost:2342");
    if (connection == NULL)
        return 1;

    SPBitmap *pixels = sp_bitmap_new(SP_PIXEL_WIDTH, SP_PIXEL_HEIGHT);
    sp_bitmap_fill(pixels, true);

    SPCommand *command = sp_command_bitmap_linear_win(0, 0, pixels, SP_COMPRESSION_CODE_UNCOMPRESSED);
    sp_connection_send_command(connection, command);

    sp_connection_free(connection);
    return 0;
}
