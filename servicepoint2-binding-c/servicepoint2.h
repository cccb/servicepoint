/* Generated with cbindgen:0.26.0 */

#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * pixel count on whole screen
 */
#define sp2_PIXEL_COUNT ((size_t)sp2_PIXEL_WIDTH * (size_t)sp2_PIXEL_HEIGHT)

/**
 * screen height in pixels
 */
#define sp2_PIXEL_HEIGHT (sp2_TILE_HEIGHT * sp2_TILE_SIZE)

/**
 * screen width in pixels
 */
#define sp2_PIXEL_WIDTH (sp2_TILE_WIDTH * sp2_TILE_SIZE)

/**
 * tile count in the y-direction
 */
#define sp2_TILE_HEIGHT 20

/**
 * size of a single tile in one dimension
 */
#define sp2_TILE_SIZE 8

/**
 * tile count in the x-direction
 */
#define sp2_TILE_WIDTH 56

/**
 * Specifies the kind of compression to use. Availability depends on features.
 */
enum sp2_CompressionCode
#ifdef __cplusplus
  : uint16_t
#endif // __cplusplus
 {
    Uncompressed = 0,
#if defined(SP2_FEATURE_compression_gz)
    Gz = 26490,
#endif
#if defined(SP2_FEATURE_compression_bz)
    Bz = 25210,
#endif
#if defined(SP2_FEATURE_compression_lz)
    Lz = 27770,
#endif
#if defined(SP2_FEATURE_compression_zs)
    Zs = 31347,
#endif
};
#ifndef __cplusplus
typedef uint16_t sp2_CompressionCode;
#endif // __cplusplus

/**
 * A vector of bits
 */
typedef struct sp2_BitVec sp2_BitVec;

/**
 * A 2D grid of bytes
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
 * The raw packet. Should probably not be used directly.
 */
typedef struct sp2_Packet sp2_Packet;

/**
 * A grid of pixels stored in packed bytes.
 */
typedef struct sp2_PixelGrid sp2_PixelGrid;

/**
 * Represents a `&mut [u8]` as a struct usable by C code.
 *
 * Usage of this type is inherently unsafe.
 */
typedef struct sp2_CByteSlice {
    /**
     * The start address of the memory
     */
    uint8_t *start;
    /**
     * The amount of memory in bytes
     */
    size_t length;
} sp2_CByteSlice;

/**
 * Type alias for documenting the meaning of the u16 in enum values
 */
typedef uint16_t sp2_Offset;

/**
 * Type alias for documenting the meaning of the u16 in enum values
 */
typedef uint8_t sp2_Brightness;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * Clones a `BitVec`.
 * The returned instance has to be freed with `bit_vec_dealloc`.
 */
struct sp2_BitVec *sp2_bit_vec_clone(const struct sp2_BitVec *this_);

/**
 * Deallocates a `BitVec`.
 *
 * Note: do not call this if the grid has been consumed in another way, e.g. to create a command.
 */
void sp2_bit_vec_dealloc(struct sp2_BitVec *this_);

/**
 * Sets the value of all bits in the `BitVec`.
 */
void sp2_bit_vec_fill(struct sp2_BitVec *this_, bool value);

/**
 * Gets the value of a bit from the `BitVec`.
 */
bool sp2_bit_vec_get(const struct sp2_BitVec *this_, size_t index);

/**
 * Gets the length of the `BitVec` in bits.
 */
size_t sp2_bit_vec_len(const struct sp2_BitVec *this_);

/**
 * Loads a `BitVec` from the provided data.
 * The returned instance has to be freed with `bit_vec_dealloc`.
 */
struct sp2_BitVec *sp2_bit_vec_load(const uint8_t *data, size_t data_length);

/**
 * Creates a new `BitVec` instance.
 * The returned instance has to be freed with `bit_vec_dealloc`.
 */
struct sp2_BitVec *sp2_bit_vec_new(size_t size);

/**
 * Sets the value of a bit in the `BitVec`.
 */
bool sp2_bit_vec_set(struct sp2_BitVec *this_, size_t index, bool value);

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
struct sp2_CByteSlice sp2_bit_vec_unsafe_data_ref(struct sp2_BitVec *this_);

/**
 * Clones a `ByteGrid`.
 * The returned instance has to be freed with `byte_grid_dealloc`.
 */
struct sp2_ByteGrid *sp2_byte_grid_clone(const struct sp2_ByteGrid *this_);

/**
 * Deallocates a `ByteGrid`.
 *
 * Note: do not call this if the grid has been consumed in another way, e.g. to create a command.
 */
void sp2_byte_grid_dealloc(struct sp2_ByteGrid *this_);

/**
 * Fills the whole `ByteGrid` with the specified value
 */
void sp2_byte_grid_fill(struct sp2_ByteGrid *this_, uint8_t value);

/**
 * Get the current value at the specified position
 */
uint8_t sp2_byte_grid_get(const struct sp2_ByteGrid *this_, size_t x, size_t y);

/**
 * Gets the height in pixels of the `ByteGrid` instance.
 */
size_t sp2_byte_grid_height(const struct sp2_ByteGrid *this_);

/**
 * Loads a `ByteGrid` with the specified dimensions from the provided data.
 * The returned instance has to be freed with `byte_grid_dealloc`.
 */
struct sp2_ByteGrid *sp2_byte_grid_load(size_t width,
                                        size_t height,
                                        const uint8_t *data,
                                        size_t data_length);

/**
 * Creates a new `ByteGrid` instance.
 * The returned instance has to be freed with `byte_grid_dealloc`.
 */
struct sp2_ByteGrid *sp2_byte_grid_new(size_t width, size_t height);

/**
 * Sets the current value at the specified position
 */
void sp2_byte_grid_set(struct sp2_ByteGrid *this_,
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
struct sp2_CByteSlice sp2_byte_grid_unsafe_data_ref(struct sp2_ByteGrid *this_);

/**
 * Gets the width in pixels of the `ByteGrid` instance.
 */
size_t sp2_byte_grid_width(const struct sp2_ByteGrid *this_);

/**
 * Allocates a new `Command::BitmapLinear` instance.
 * The passed `BitVec` gets deallocated in the process.
 */
struct sp2_Command *sp2_command_bitmap_linear(sp2_Offset offset,
                                              struct sp2_BitVec *bit_vec,
                                              sp2_CompressionCode compression);

/**
 * Allocates a new `Command::BitmapLinearAnd` instance.
 * The passed `BitVec` gets deallocated in the process.
 */
struct sp2_Command *sp2_command_bitmap_linear_and(sp2_Offset offset,
                                                  struct sp2_BitVec *bit_vec,
                                                  sp2_CompressionCode compression);

/**
 * Allocates a new `Command::BitmapLinearOr` instance.
 * The passed `BitVec` gets deallocated in the process.
 */
struct sp2_Command *sp2_command_bitmap_linear_or(sp2_Offset offset,
                                                 struct sp2_BitVec *bit_vec,
                                                 sp2_CompressionCode compression);

/**
 * Allocates a new `Command::BitmapLinearWin` instance.
 * The passed `PixelGrid` gets deallocated in the process.
 */
struct sp2_Command *sp2_command_bitmap_linear_win(uint16_t x,
                                                  uint16_t y,
                                                  struct sp2_PixelGrid *byte_grid);

/**
 * Allocates a new `Command::BitmapLinearXor` instance.
 * The passed `BitVec` gets deallocated in the process.
 */
struct sp2_Command *sp2_command_bitmap_linear_xor(sp2_Offset offset,
                                                  struct sp2_BitVec *bit_vec,
                                                  sp2_CompressionCode compression);

/**
 * Allocates a new `Command::Brightness` instance
 */
struct sp2_Command *sp2_command_brightness(sp2_Brightness brightness);

/**
 * Allocates a new `Command::CharBrightness` instance.
 * The passed `ByteGrid` gets deallocated in the process.
 */
struct sp2_Command *sp2_command_char_brightness(uint16_t x,
                                                uint16_t y,
                                                struct sp2_ByteGrid *byte_grid);

/**
 * Allocates a new `Command::Clear` instance
 */
struct sp2_Command *sp2_command_clear(void);

/**
 * Clones a `Command` instance
 */
struct sp2_Command *sp2_command_clone(const struct sp2_Command *original);

/**
 * Allocates a new `Command::Cp437Data` instance.
 * The passed `ByteGrid` gets deallocated in the process.
 */
struct sp2_Command *sp2_command_cp437_data(uint16_t x,
                                           uint16_t y,
                                           struct sp2_ByteGrid *byte_grid);

/**
 * Deallocates a `Command`. Note that connection_send does this implicitly, so you only need
 * to do this if you use the library for parsing commands.
 */
void sp2_command_dealloc(struct sp2_Command *ptr);

/**
 * Allocates a new `Command::FadeOut` instance
 */
struct sp2_Command *sp2_command_fade_out(void);

/**
 * Allocates a new `Command::HardReset` instance
 */
struct sp2_Command *sp2_command_hard_reset(void);

/**
 * Tries to turn a `Packet` into a `Command`. The packet is gets deallocated in the process.
 *
 * Returns: pointer to command or NULL
 */
struct sp2_Command *sp2_command_try_from_packet(struct sp2_Packet *packet);

/**
 * Tries to load a `Command` from the passed array with the specified length.
 *
 * returns: NULL in case of an error, pointer to the allocated command otherwise
 */
struct sp2_Command *sp2_command_try_load(const uint8_t *data, size_t length);

/**
 * Closes and deallocates a connection instance
 */
void sp2_connection_dealloc(struct sp2_Connection *ptr);

/**
 * Creates a new instance of Connection.
 * The returned instance has to be deallocated with `connection_dealloc`.
 *
 * returns: NULL if connection fails or connected instance
 *
 * Panics: bad string encoding
 */
struct sp2_Connection *sp2_connection_open(const char *host);

/**
 * Sends the command instance. The instance is consumed / destroyed and cannot be used after this call.
 */
bool sp2_connection_send(const struct sp2_Connection *connection,
                         struct sp2_Packet *command_ptr);

/**
 * Turns a `Command` into a `Packet`. The command gets deallocated in the process.
 */
struct sp2_Packet *sp2_packet_from_command(struct sp2_Command *command);

/**
 * Clones a `PixelGrid`.
 * The returned instance has to be freed with `pixel_grid_dealloc`.
 */
struct sp2_PixelGrid *sp2_pixel_grid_clone(const struct sp2_PixelGrid *this_);

/**
 * Deallocates a `PixelGrid`.
 *
 * Note: do not call this if the grid has been consumed in another way, e.g. to create a command.
 */
void sp2_pixel_grid_dealloc(struct sp2_PixelGrid *this_);

/**
 * Fills the whole `PixelGrid` with the specified value
 */
void sp2_pixel_grid_fill(struct sp2_PixelGrid *this_, bool value);

/**
 * Get the current value at the specified position
 */
bool sp2_pixel_grid_get(const struct sp2_PixelGrid *this_, size_t x, size_t y);

/**
 * Gets the height in pixels of the `PixelGrid` instance.
 */
size_t sp2_pixel_grid_height(const struct sp2_PixelGrid *this_);

/**
 * Loads a `PixelGrid` with the specified dimensions from the provided data.
 * The returned instance has to be freed with `pixel_grid_dealloc`.
 */
struct sp2_PixelGrid *sp2_pixel_grid_load(size_t width,
                                          size_t height,
                                          const uint8_t *data,
                                          size_t data_length);

/**
 * Creates a new `PixelGrid` instance.
 * The returned instance has to be freed with `pixel_grid_dealloc`.
 */
struct sp2_PixelGrid *sp2_pixel_grid_new(size_t width, size_t height);

/**
 * Sets the current value at the specified position
 */
void sp2_pixel_grid_set(struct sp2_PixelGrid *this_,
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
struct sp2_CByteSlice sp2_pixel_grid_unsafe_data_ref(struct sp2_PixelGrid *this_);

/**
 * Gets the width in pixels of the `PixelGrid` instance.
 */
size_t sp2_pixel_grid_width(const struct sp2_PixelGrid *this_);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
