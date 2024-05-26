/* Generated with cbindgen:0.26.0 */

#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * pixel count on whole screen
 */
#define sp_PIXEL_COUNT (sp_PIXEL_WIDTH * sp_PIXEL_HEIGHT)

/**
 * screen height in pixels
 */
#define sp_PIXEL_HEIGHT (sp_TILE_HEIGHT * sp_TILE_SIZE)

/**
 * screen width in pixels
 */
#define sp_PIXEL_WIDTH (sp_TILE_WIDTH * sp_TILE_SIZE)

/**
 * tile count in the y-direction
 */
#define sp_TILE_HEIGHT 20

/**
 * size of a single tile in one dimension
 */
#define sp_TILE_SIZE 8

/**
 * tile count in the x-direction
 */
#define sp_TILE_WIDTH 56

/**
 * Specifies the kind of compression to use. Availability depends on features.
 */
enum sp_CompressionCode
#ifdef __cplusplus
  : uint16_t
#endif // __cplusplus
 {
    Uncompressed = 0,
    Zlib = 26490,
    Bzip2 = 25210,
    Lzma = 27770,
    Zstd = 31347,
};
#ifndef __cplusplus
typedef uint16_t sp_CompressionCode;
#endif // __cplusplus

/**
 * A vector of bits
 */
typedef struct sp_BitVec sp_BitVec;

/**
 * A 2D grid of bytes
 */
typedef struct sp_ByteGrid sp_ByteGrid;

/**
 * A command to send to the display.
 */
typedef struct sp_Command sp_Command;

/**
 * A connection to the display.
 */
typedef struct sp_Connection sp_Connection;

/**
 * The raw packet. Should probably not be used directly.
 */
typedef struct sp_Packet sp_Packet;

/**
 * A grid of pixels stored in packed bytes.
 */
typedef struct sp_PixelGrid sp_PixelGrid;

/**
 * Represents a span of memory (`&mut [u8]` ) as a struct usable by C code.
 *
 * Usage of this type is inherently unsafe.
 */
typedef struct sp_CByteSlice {
    /**
     * The start address of the memory
     */
    uint8_t *start;
    /**
     * The amount of memory in bytes
     */
    size_t length;
} sp_CByteSlice;

/**
 * Type alias for documenting the meaning of the u16 in enum values
 */
typedef size_t sp_Offset;

/**
 * Type alias for documenting the meaning of the u16 in enum values
 */
typedef uint8_t sp_Brightness;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * Clones a `BitVec`.
 * The returned instance has to be freed with `bit_vec_dealloc`.
 */
struct sp_BitVec *sp_bit_vec_clone(const struct sp_BitVec *this_);

/**
 * Deallocates a `BitVec`.
 *
 * Note: do not call this if the grid has been consumed in another way, e.g. to create a command.
 */
void sp_bit_vec_dealloc(struct sp_BitVec *this_);

/**
 * Sets the value of all bits in the `BitVec`.
 */
void sp_bit_vec_fill(struct sp_BitVec *this_, bool value);

/**
 * Gets the value of a bit from the `BitVec`.
 */
bool sp_bit_vec_get(const struct sp_BitVec *this_, size_t index);

/**
 * Returns true if length is 0.
 */
bool sp_bit_vec_is_empty(const struct sp_BitVec *this_);

/**
 * Gets the length of the `BitVec` in bits.
 */
size_t sp_bit_vec_len(const struct sp_BitVec *this_);

/**
 * Loads a `BitVec` from the provided data.
 * The returned instance has to be freed with `bit_vec_dealloc`.
 */
struct sp_BitVec *sp_bit_vec_load(const uint8_t *data, size_t data_length);

/**
 * Creates a new `BitVec` instance.
 * The returned instance has to be freed with `bit_vec_dealloc`.
 */
struct sp_BitVec *sp_bit_vec_new(size_t size);

/**
 * Sets the value of a bit in the `BitVec`.
 */
bool sp_bit_vec_set(struct sp_BitVec *this_, size_t index, bool value);

/**
 * Gets an unsafe reference to the data of the `BitVec` instance.
 *
 * ## Safety
 *
 * The caller has to make sure to never access the returned memory after the `BitVec`
 * instance has been consumed or manually deallocated.
 *
 * Reading and writing concurrently to either the original instance or the returned data will
 * result in undefined behavior.
 */
struct sp_CByteSlice sp_bit_vec_unsafe_data_ref(struct sp_BitVec *this_);

/**
 * Clones a `ByteGrid`.
 * The returned instance has to be freed with `byte_grid_dealloc`.
 */
struct sp_ByteGrid *sp_byte_grid_clone(const struct sp_ByteGrid *this_);

/**
 * Deallocates a `ByteGrid`.
 *
 * Note: do not call this if the grid has been consumed in another way, e.g. to create a command.
 */
void sp_byte_grid_dealloc(struct sp_ByteGrid *this_);

/**
 * Fills the whole `ByteGrid` with the specified value
 */
void sp_byte_grid_fill(struct sp_ByteGrid *this_, uint8_t value);

/**
 * Get the current value at the specified position
 */
uint8_t sp_byte_grid_get(const struct sp_ByteGrid *this_, size_t x, size_t y);

/**
 * Gets the height in pixels of the `ByteGrid` instance.
 */
size_t sp_byte_grid_height(const struct sp_ByteGrid *this_);

/**
 * Loads a `ByteGrid` with the specified dimensions from the provided data.
 * The returned instance has to be freed with `byte_grid_dealloc`.
 */
struct sp_ByteGrid *sp_byte_grid_load(size_t width,
                                      size_t height,
                                      const uint8_t *data,
                                      size_t data_length);

/**
 * Creates a new `ByteGrid` instance.
 * The returned instance has to be freed with `byte_grid_dealloc`.
 */
struct sp_ByteGrid *sp_byte_grid_new(size_t width, size_t height);

/**
 * Sets the current value at the specified position
 */
void sp_byte_grid_set(struct sp_ByteGrid *this_,
                      size_t x,
                      size_t y,
                      uint8_t value);

/**
 * Gets an unsafe reference to the data of the `ByteGrid` instance.
 *
 * ## Safety
 *
 * The caller has to make sure to never access the returned memory after the `ByteGrid`
 * instance has been consumed or manually deallocated.
 *
 * Reading and writing concurrently to either the original instance or the returned data will
 * result in undefined behavior.
 */
struct sp_CByteSlice sp_byte_grid_unsafe_data_ref(struct sp_ByteGrid *this_);

/**
 * Gets the width in pixels of the `ByteGrid` instance.
 */
size_t sp_byte_grid_width(const struct sp_ByteGrid *this_);

/**
 * Allocates a new `Command::BitmapLinear` instance.
 * The passed `BitVec` gets deallocated in the process.
 */
struct sp_Command *sp_command_bitmap_linear(sp_Offset offset,
                                            struct sp_BitVec *bit_vec,
                                            sp_CompressionCode compression);

/**
 * Allocates a new `Command::BitmapLinearAnd` instance.
 * The passed `BitVec` gets deallocated in the process.
 */
struct sp_Command *sp_command_bitmap_linear_and(sp_Offset offset,
                                                struct sp_BitVec *bit_vec,
                                                sp_CompressionCode compression);

/**
 * Allocates a new `Command::BitmapLinearOr` instance.
 * The passed `BitVec` gets deallocated in the process.
 */
struct sp_Command *sp_command_bitmap_linear_or(sp_Offset offset,
                                               struct sp_BitVec *bit_vec,
                                               sp_CompressionCode compression);

/**
 * Allocates a new `Command::BitmapLinearWin` instance.
 * The passed `PixelGrid` gets deallocated in the process.
 */
struct sp_Command *sp_command_bitmap_linear_win(size_t x,
                                                size_t y,
                                                struct sp_PixelGrid *byte_grid,
                                                sp_CompressionCode compression_code);

/**
 * Allocates a new `Command::BitmapLinearXor` instance.
 * The passed `BitVec` gets deallocated in the process.
 */
struct sp_Command *sp_command_bitmap_linear_xor(sp_Offset offset,
                                                struct sp_BitVec *bit_vec,
                                                sp_CompressionCode compression);

/**
 * Allocates a new `Command::Brightness` instance
 */
struct sp_Command *sp_command_brightness(sp_Brightness brightness);

/**
 * Allocates a new `Command::CharBrightness` instance.
 * The passed `ByteGrid` gets deallocated in the process.
 */
struct sp_Command *sp_command_char_brightness(size_t x,
                                              size_t y,
                                              struct sp_ByteGrid *byte_grid);

/**
 * Allocates a new `Command::Clear` instance
 */
struct sp_Command *sp_command_clear(void);

/**
 * Clones a `Command` instance
 */
struct sp_Command *sp_command_clone(const struct sp_Command *original);

/**
 * Allocates a new `Command::Cp437Data` instance.
 * The passed `ByteGrid` gets deallocated in the process.
 */
struct sp_Command *sp_command_cp437_data(size_t x,
                                         size_t y,
                                         struct sp_ByteGrid *byte_grid);

/**
 * Deallocates a `Command`. Note that connection_send does this implicitly, so you only need
 * to do this if you use the library for parsing commands.
 */
void sp_command_dealloc(struct sp_Command *ptr);

/**
 * Allocates a new `Command::FadeOut` instance
 */
struct sp_Command *sp_command_fade_out(void);

/**
 * Allocates a new `Command::HardReset` instance
 */
struct sp_Command *sp_command_hard_reset(void);

/**
 * Tries to turn a `Packet` into a `Command`. The packet is gets deallocated in the process.
 *
 * Returns: pointer to command or NULL
 */
struct sp_Command *sp_command_try_from_packet(struct sp_Packet *packet);

/**
 * Closes and deallocates a connection instance
 */
void sp_connection_dealloc(struct sp_Connection *ptr);

/**
 * Creates a new instance of Connection.
 * The returned instance has to be deallocated with `connection_dealloc`.
 *
 * returns: NULL if connection fails or connected instance
 *
 * Panics: bad string encoding
 */
struct sp_Connection *sp_connection_open(const char *host);

/**
 * Sends the command instance. The instance is consumed / destroyed and cannot be used after this call.
 */
bool sp_connection_send(const struct sp_Connection *connection,
                        struct sp_Packet *command_ptr);

/**
 * Deallocates a `Packet`.
 *
 * Note: do not call this if the instance has been consumed in another way, e.g. by sending it.
 */
void sp_packet_dealloc(struct sp_Packet *this_);

/**
 * Turns a `Command` into a `Packet`. The command gets deallocated in the process.
 */
struct sp_Packet *sp_packet_from_command(struct sp_Command *command);

/**
 * Tries to load a `Packet` from the passed array with the specified length.
 *
 * returns: NULL in case of an error, pointer to the allocated packet otherwise
 */
struct sp_Packet *sp_packet_try_load(const uint8_t *data, size_t length);

/**
 * Clones a `PixelGrid`.
 * The returned instance has to be freed with `pixel_grid_dealloc`.
 */
struct sp_PixelGrid *sp_pixel_grid_clone(const struct sp_PixelGrid *this_);

/**
 * Deallocates a `PixelGrid`.
 *
 * Note: do not call this if the grid has been consumed in another way, e.g. to create a command.
 */
void sp_pixel_grid_dealloc(struct sp_PixelGrid *this_);

/**
 * Fills the whole `PixelGrid` with the specified value
 */
void sp_pixel_grid_fill(struct sp_PixelGrid *this_, bool value);

/**
 * Get the current value at the specified position
 */
bool sp_pixel_grid_get(const struct sp_PixelGrid *this_, size_t x, size_t y);

/**
 * Gets the height in pixels of the `PixelGrid` instance.
 */
size_t sp_pixel_grid_height(const struct sp_PixelGrid *this_);

/**
 * Loads a `PixelGrid` with the specified dimensions from the provided data.
 * The returned instance has to be freed with `pixel_grid_dealloc`.
 */
struct sp_PixelGrid *sp_pixel_grid_load(size_t width,
                                        size_t height,
                                        const uint8_t *data,
                                        size_t data_length);

/**
 * Creates a new `PixelGrid` instance.
 * The returned instance has to be freed with `pixel_grid_dealloc`.
 */
struct sp_PixelGrid *sp_pixel_grid_new(size_t width, size_t height);

/**
 * Sets the current value at the specified position
 */
void sp_pixel_grid_set(struct sp_PixelGrid *this_,
                       size_t x,
                       size_t y,
                       bool value);

/**
 * Gets an unsafe reference to the data of the `PixelGrid` instance.
 *
 * ## Safety
 *
 * The caller has to make sure to never access the returned memory after the `PixelGrid`
 * instance has been consumed or manually deallocated.
 *
 * Reading and writing concurrently to either the original instance or the returned data will
 * result in undefined behavior.
 */
struct sp_CByteSlice sp_pixel_grid_unsafe_data_ref(struct sp_PixelGrid *this_);

/**
 * Gets the width in pixels of the `PixelGrid` instance.
 */
size_t sp_pixel_grid_width(const struct sp_PixelGrid *this_);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
