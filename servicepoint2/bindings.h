#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * size of a single tile in one dimension
 */
#define sp2_TILE_SIZE 8

/**
 * tile count in the x-direction
 */
#define sp2_TILE_WIDTH 56

/**
 * tile count in the y-direction
 */
#define sp2_TILE_HEIGHT 20

/**
 * screen width in pixels
 */
#define sp2_PIXEL_WIDTH (sp2_TILE_WIDTH * sp2_TILE_SIZE)

/**
 * screen height in pixels
 */
#define sp2_PIXEL_HEIGHT (sp2_TILE_HEIGHT * sp2_TILE_SIZE)

/**
 * pixel count on whole screen
 */
#define sp2_PIXEL_COUNT ((uintptr_t)sp2_PIXEL_WIDTH * (uintptr_t)sp2_PIXEL_HEIGHT)

/**
 * Specifies the kind of compression to use. Availability depends on features.
 */
enum sp2_CompressionCode {
  Uncompressed = 0,
  Gz = 26490,
  Bz = 25210,
  Lz = 27770,
  Zs = 31347,
};
typedef uint16_t sp2_CompressionCode;

/**
 * A vector of bits
 */
typedef struct sp2_BitVec sp2_BitVec;

/**
 * A grid of bytes
 */
typedef struct sp2_ByteGrid sp2_ByteGrid;

/**
 * A command to send to the display.
 */
typedef struct sp2_Command sp2_Command;

/**
 * A connection to the display.
 */
typedef struct sp2_Connection sp2_Connection;

/**
 * A grid of pixels stored in packed bytes.
 */
typedef struct sp2_PixelGrid sp2_PixelGrid;

typedef uint8_t sp2_Brightness;

typedef uint16_t sp2_Offset;

/**
 * Tries to load a command from the passed array with the specified length.
 *
 * returns: NULL in case of an error, pointer to the allocated command otherwise
 */
struct sp2_Command *command_try_load(const uint8_t *data, uintptr_t length);

/**
 * Clones a `Command` instance
 */
struct sp2_Command *command_clone(const struct sp2_Command *original);

/**
 * Allocates a new `Command::Clear` instance
 */
struct sp2_Command *command_clear(void);

/**
 * Allocates a new `Command::HardReset` instance
 */
struct sp2_Command *command_hard_reset(void);

/**
 * Allocates a new `Command::FadeOut` instance
 */
struct sp2_Command *command_fade_out(void);

/**
 * Allocates a new `Command::Brightness` instance
 */
struct sp2_Command *command_brightness(sp2_Brightness brightness);

/**
 * Allocates a new `Command::CharBrightness` instance.
 * The passed `ByteGrid` gets deallocated in the process.
 */
struct sp2_Command *command_char_brightness(uint16_t x, uint16_t y, struct sp2_ByteGrid *byte_grid);

/**
 * Allocates a new `Command::BitmapLinear` instance.
 * The passed `BitVec` gets deallocated in the process.
 */
struct sp2_Command *command_bitmap_linear(sp2_Offset offset,
                                          struct sp2_BitVec *bit_vec,
                                          sp2_CompressionCode compression);

/**
 * Allocates a new `Command::BitmapLinearAnd` instance.
 * The passed `BitVec` gets deallocated in the process.
 */
struct sp2_Command *command_bitmap_linear_and(sp2_Offset offset,
                                              struct sp2_BitVec *bit_vec,
                                              sp2_CompressionCode compression);

/**
 * Allocates a new `Command::BitmapLinearOr` instance.
 * The passed `BitVec` gets deallocated in the process.
 */
struct sp2_Command *command_bitmap_linear_or(sp2_Offset offset,
                                             struct sp2_BitVec *bit_vec,
                                             sp2_CompressionCode compression);

/**
 * Allocates a new `Command::BitmapLinearXor` instance.
 * The passed `BitVec` gets deallocated in the process.
 */
struct sp2_Command *command_bitmap_linear_xor(sp2_Offset offset,
                                              struct sp2_BitVec *bit_vec,
                                              sp2_CompressionCode compression);

/**
 * Allocates a new `Command::Cp437Data` instance.
 * The passed `ByteGrid` gets deallocated in the process.
 */
struct sp2_Command *command_cp437_data(uint16_t x, uint16_t y, struct sp2_ByteGrid *byte_grid);

/**
 * Allocates a new `Command::BitmapLinearWin` instance.
 * The passed `PixelGrid` gets deallocated in the process.
 */
struct sp2_Command *command_bitmap_linear_win(uint16_t x,
                                              uint16_t y,
                                              struct sp2_PixelGrid *byte_grid);

/**
 * Deallocates a command. Note that connection_send does this implicitly, so you only need
 * to do this if you use the library for parsing commands.
 */
void command_dealloc(struct sp2_Command *ptr);

/**
 * Creates a new instance of Connection.
 * The returned instance has to be deallocated with `connection_dealloc`.
 *
 * returns: NULL if connection fails or connected instance
 *
 * Panics: bad string encoding
 */
struct sp2_Connection *connection_open(const char *host);

/**
 * Sends the command instance. The instance is consumed / destroyed and cannot be used after this call.
 */
bool connection_send(const struct sp2_Connection *connection,
                     struct sp2_Command *command_ptr);

/**
 * Closes and deallocates a connection instance
 */
void connection_dealloc(struct sp2_Connection *ptr);
