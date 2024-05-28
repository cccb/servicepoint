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
    /**
     * no compression
     */
    Uncompressed = 0,
    /**
     * compress using flate2 with zlib header
     */
    Zlib = 26490,
    /**
     * compress using bzip2
     */
    Bzip2 = 25210,
    /**
     * compress using lzma
     */
    Lzma = 27770,
    /**
     * compress using Zstandard
     */
    Zstd = 31347,
};
#ifndef __cplusplus
typedef uint16_t sp_CompressionCode;
#endif // __cplusplus

/**
 * A fixed-size vector of bits
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
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - accesses to the memory pointed to with `start` is never accessed outside `length`
 * - the lifetime of the `CByteSlice` does not outlive the memory it points to, as described in
 *   the function returning this type.
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
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `this` points to a valid `BitVec`
 * - `this` is not written to concurrently
 * - the returned instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_bit_vec_dealloc`.
 */
struct sp_BitVec *sp_bit_vec_clone(const struct sp_BitVec *this_);

/**
 * Deallocates a `BitVec`.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `this` points to a valid `BitVec`
 * - `this` is not used concurrently or after this call
 * - `this` was not passed to another consuming function, e.g. to create a `Command`
 */
void sp_bit_vec_dealloc(struct sp_BitVec *this_);

/**
 * Sets the value of all bits in the `BitVec`.
 *
 * # Arguments
 *
 * * `value`: the value to set all bits to
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `this` points to a valid `BitVec`
 * - `this` is not written to or read from concurrently
 */
void sp_bit_vec_fill(struct sp_BitVec *this_, bool value);

/**
 * Gets the value of a bit from the `BitVec`.
 *
 * # Arguments
 *
 * * `this`: instance to read from
 * * `index`: the bit index to read
 *
 * returns: value of the bit
 *
 * # Panics
 *
 * When accessing `index` out of bounds.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `this` points to a valid `BitVec`
 * - `this` is not written to concurrently
 */
bool sp_bit_vec_get(const struct sp_BitVec *this_, size_t index);

/**
 * Returns true if length is 0.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `this` points to a valid `BitVec`
 */
bool sp_bit_vec_is_empty(const struct sp_BitVec *this_);

/**
 * Gets the length of the `BitVec` in bits.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `this` points to a valid `BitVec`
 */
size_t sp_bit_vec_len(const struct sp_BitVec *this_);

/**
 * Interpret the data as a series of bits and load then into a new `BitVec` instance.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `data` points to a valid memory location of at least `data_length`
 *   bytes in size.
 * - the returned instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_bit_vec_dealloc`.
 */
struct sp_BitVec *sp_bit_vec_load(const uint8_t *data,
                                  size_t data_length);

/**
 * Creates a new `BitVec` instance.
 *
 * # Arguments
 *
 * * `size`: size in bits.
 *
 * returns: `BitVec` with all bits set to false.
 *
 * # Panics
 *
 * When `size` is not divisible by 8.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - the returned instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_bit_vec_dealloc`.
 */
struct sp_BitVec *sp_bit_vec_new(size_t size);

/**
 * Sets the value of a bit in the `BitVec`.
 *
 * # Arguments
 *
 * * `this`: instance to write to
 * * `index`: the bit index to edit
 * * `value`: the value to set the bit to
 *
 * returns: old value of the bit
 *
 * # Panics
 *
 * When accessing `index` out of bounds.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `this` points to a valid `BitVec`
 * - `this` is not written to or read from concurrently
 */
bool sp_bit_vec_set(struct sp_BitVec *this_, size_t index, bool value);

/**
 * Gets an unsafe reference to the data of the `BitVec` instance.
 *
 * ## Safety
 *
 * The caller has to make sure that:
 *
 * - `this` points to a valid `BitVec`
 * - the returned memory range is never accessed after the passed `BitVec` has been freed
 * - the returned memory range is never accessed concurrently, either via the `BitVec` or directly
 */
struct sp_CByteSlice sp_bit_vec_unsafe_data_ref(struct sp_BitVec *this_);

/**
 * Clones a `ByteGrid`.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `this` points to a valid `ByteGrid`
 * - `this` is not written to concurrently
 * - the returned instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_byte_grid_dealloc`.
 */
struct sp_ByteGrid *sp_byte_grid_clone(const struct sp_ByteGrid *this_);

/**
 * Deallocates a `ByteGrid`.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `this` points to a valid `ByteGrid`
 * - `this` is not used concurrently or after this call
 * - `this` was not passed to another consuming function, e.g. to create a `Command`
 */
void sp_byte_grid_dealloc(struct sp_ByteGrid *this_);

/**
 * Sets the value of all cells in the `ByteGrid`.
 *
 * # Arguments
 *
 * * `this`: instance to write to
 * * `value`: the value to set all cells to
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `this` points to a valid `ByteGrid`
 * - `this` is not written to or read from concurrently
 */
void sp_byte_grid_fill(struct sp_ByteGrid *this_, uint8_t value);

/**
 * Gets the current value at the specified position.
 *
 * # Arguments
 *
 * * `this`: instance to read from
 * * `x` and `y`: position of the cell to read
 *
 * # Panics
 *
 * When accessing `x` or `y` out of bounds.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `this` points to a valid `ByteGrid`
 * - `this` is not written to concurrently
 */
uint8_t sp_byte_grid_get(const struct sp_ByteGrid *this_, size_t x, size_t y);

/**
 * Gets the height of the `ByteGrid` instance.
 *
 * # Arguments
 *
 * * `this`: instance to read from
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `this` points to a valid `ByteGrid`
 */
size_t sp_byte_grid_height(const struct sp_ByteGrid *this_);

/**
 * Loads a `ByteGrid` with the specified dimensions from the provided data.
 *
 * # Panics
 *
 * When the provided `data_length` is not sufficient for the `height` and `width`
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `data` points to a valid memory location of at least `data_length`
 *   bytes in size.
 * - the returned instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_byte_grid_dealloc`.
 */
struct sp_ByteGrid *sp_byte_grid_load(size_t width,
                                      size_t height,
                                      const uint8_t *data,
                                      size_t data_length);

/**
 * Creates a new `ByteGrid` with the specified dimensions.
 *
 * returns: `ByteGrid` initialized to 0.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - the returned instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_byte_grid_dealloc`.
 */
struct sp_ByteGrid *sp_byte_grid_new(size_t width,
                                     size_t height);

/**
 * Sets the value of the specified position in the `ByteGrid`.
 *
 * # Arguments
 *
 * * `this`: instance to write to
 * * `x` and `y`: position of the cell
 * * `value`: the value to write to the cell
 *
 * returns: old value of the cell
 *
 * # Panics
 *
 * When accessing `x` or `y` out of bounds.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `this` points to a valid `BitVec`
 * - `this` is not written to or read from concurrently
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
 * The caller has to make sure that:
 *
 * - `this` points to a valid `ByteGrid`
 * - the returned memory range is never accessed after the passed `ByteGrid` has been freed
 * - the returned memory range is never accessed concurrently, either via the `ByteGrid` or directly
 */
struct sp_CByteSlice sp_byte_grid_unsafe_data_ref(struct sp_ByteGrid *this_);

/**
 * Gets the width of the `ByteGrid` instance.
 *
 * # Arguments
 *
 * * `this`: instance to read from
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `this` points to a valid `ByteGrid`
 */
size_t sp_byte_grid_width(const struct sp_ByteGrid *this_);

/**
 * Allocates a new `Command::BitmapLinear` instance.
 * The passed `BitVec` gets consumed.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `bit_vec` points to a valid instance of `BitVec`
 * - `bit_vec` is not used concurrently or after this call
 * - `compression` matches one of the allowed enum values
 * - the returned `Command` instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_command_dealloc`.
 */
struct sp_Command *sp_command_bitmap_linear(sp_Offset offset,
                                            struct sp_BitVec *bit_vec,
                                            sp_CompressionCode compression);

/**
 * Allocates a new `Command::BitmapLinearAnd` instance.
 * The passed `BitVec` gets consumed.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `bit_vec` points to a valid instance of `BitVec`
 * - `bit_vec` is not used concurrently or after this call
 * - `compression` matches one of the allowed enum values
 * - the returned `Command` instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_command_dealloc`.
 */
struct sp_Command *sp_command_bitmap_linear_and(sp_Offset offset,
                                                struct sp_BitVec *bit_vec,
                                                sp_CompressionCode compression);

/**
 * Allocates a new `Command::BitmapLinearOr` instance.
 * The passed `BitVec` gets consumed.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `bit_vec` points to a valid instance of `BitVec`
 * - `bit_vec` is not used concurrently or after this call
 * - `compression` matches one of the allowed enum values
 * - the returned `Command` instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_command_dealloc`.
 */
struct sp_Command *sp_command_bitmap_linear_or(sp_Offset offset,
                                               struct sp_BitVec *bit_vec,
                                               sp_CompressionCode compression);

/**
 * Allocates a new `Command::BitmapLinearWin` instance.
 * The passed `PixelGrid` gets consumed.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `pixel_grid` points to a valid instance of `PixelGrid`
 * - `pixel_grid` is not used concurrently or after this call
 * - `compression` matches one of the allowed enum values
 * - the returned `Command` instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_command_dealloc`.
 */
struct sp_Command *sp_command_bitmap_linear_win(size_t x,
                                                size_t y,
                                                struct sp_PixelGrid *pixel_grid,
                                                sp_CompressionCode compression_code);

/**
 * Allocates a new `Command::BitmapLinearXor` instance.
 * The passed `BitVec` gets consumed.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `bit_vec` points to a valid instance of `BitVec`
 * - `bit_vec` is not used concurrently or after this call
 * - `compression` matches one of the allowed enum values
 * - the returned `Command` instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_command_dealloc`.
 */
struct sp_Command *sp_command_bitmap_linear_xor(sp_Offset offset,
                                                struct sp_BitVec *bit_vec,
                                                sp_CompressionCode compression);

/**
 * Allocates a new `Command::Brightness` instance.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - the returned `Command` instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_command_dealloc`.
 */
struct sp_Command *sp_command_brightness(sp_Brightness brightness);

/**
 * Allocates a new `Command::CharBrightness` instance.
 * The passed `ByteGrid` gets consumed.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `byte_grid` points to a valid instance of `ByteGrid`
 * - `byte_grid` is not used concurrently or after this call
 * - the returned `Command` instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_command_dealloc`.
 */
struct sp_Command *sp_command_char_brightness(size_t x,
                                              size_t y,
                                              struct sp_ByteGrid *byte_grid);

/**
 * Allocates a new `Command::Clear` instance.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - the returned `Command` instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_command_dealloc`.
 */
struct sp_Command *sp_command_clear(void);

/**
 * Clones a `Command` instance.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `this` points to a valid instance of `Command`
 * - `this` is not written to concurrently
 * - the returned `Command` instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_command_dealloc`.
 */
struct sp_Command *sp_command_clone(const struct sp_Command *original);

/**
 * Allocates a new `Command::Cp437Data` instance.
 * The passed `ByteGrid` gets consumed.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `byte_grid` points to a valid instance of `ByteGrid`
 * - `byte_grid` is not used concurrently or after this call
 * - the returned `Command` instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_command_dealloc`.
 */
struct sp_Command *sp_command_cp437_data(size_t x,
                                         size_t y,
                                         struct sp_ByteGrid *byte_grid);

/**
 * Deallocates a `Command`.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `this` points to a valid `Command`
 * - `this` is not used concurrently or after this call
 * - `this` was not passed to another consuming function, e.g. to create a `Packet`
 */
void sp_command_dealloc(struct sp_Command *ptr);

/**
 * Allocates a new `Command::FadeOut` instance.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - the returned `Command` instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_command_dealloc`.
 */
struct sp_Command *sp_command_fade_out(void);

/**
 * Allocates a new `Command::HardReset` instance.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - the returned `Command` instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_command_dealloc`.
 */
struct sp_Command *sp_command_hard_reset(void);

/**
 * Tries to turn a `Packet` into a `Command`. The packet is deallocated in the process.
 *
 * Returns: pointer to new `Command` instance or NULL
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `packet` points to a valid instance of `Packet`
 * - `packet` is not used concurrently or after this call
 * - the result is checked for NULL
 * - the returned `Command` instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_command_dealloc`.
 */
struct sp_Command *sp_command_try_from_packet(struct sp_Packet *packet);

/**
 * Closes and deallocates a `Connection`.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `this` points to a valid `Connection`
 * - `this` is not used concurrently or after this call
 */
void sp_connection_dealloc(struct sp_Connection *ptr);

/**
 * Creates a new instance of `Connection`.
 *
 * returns: NULL if connection fails, or connected instance
 *
 * # Panics
 *
 * Bad string encoding
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - the returned instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_connection_dealloc`.
 */
struct sp_Connection *sp_connection_open(const char *host);

/**
 * Sends a `Packet` to the display using the `Connection`.
 * The passed `Packet` gets consumed.
 *
 * returns: true in case of success
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `connection` points to a valid instance of `Connection`
 * - `packet` points to a valid instance of `Packet`
 * - `packet` is not used concurrently or after this call
 */
bool sp_connection_send(const struct sp_Connection *connection,
                        struct sp_Packet *packet);

/**
 * Deallocates a `Packet`.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `this` points to a valid `Packet`
 * - `this` is not used concurrently or after this call
 */
void sp_packet_dealloc(struct sp_Packet *this_);

/**
 * Turns a `Command` into a `Packet`.
 * The `Command` gets consumed.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `command` points to a valid instance of `Command`
 * - `command` is not used concurrently or after this call
 * - the returned `Packet` instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_packet_dealloc`.
 */
struct sp_Packet *sp_packet_from_command(struct sp_Command *command);

/**
 * Tries to load a `Packet` from the passed array with the specified length.
 *
 * returns: NULL in case of an error, pointer to the allocated packet otherwise
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `data` points to a valid memory region of at least `length` bytes
 * - `data` is not written to concurrently
 * - the returned `Packet` instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_packet_dealloc`.
 */
struct sp_Packet *sp_packet_try_load(const uint8_t *data,
                                     size_t length);

/**
 * Clones a `PixelGrid`.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `this` points to a valid `PixelGrid`
 * - `this` is not written to concurrently
 * - the returned instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_pixel_grid_dealloc`.
 */
struct sp_PixelGrid *sp_pixel_grid_clone(const struct sp_PixelGrid *this_);

/**
 * Deallocates a `PixelGrid`.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `this` points to a valid `PixelGrid`
 * - `this` is not used concurrently or after this call
 * - `this` was not passed to another consuming function, e.g. to create a `Command`
 */
void sp_pixel_grid_dealloc(struct sp_PixelGrid *this_);

/**
 * Sets the state of all pixels in the `PixelGrid`.
 *
 * # Arguments
 *
 * * `this`: instance to write to
 * * `value`: the value to set all pixels to
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `this` points to a valid `PixelGrid`
 * - `this` is not written to or read from concurrently
 */
void sp_pixel_grid_fill(struct sp_PixelGrid *this_, bool value);

/**
 * Gets the current value at the specified position in the `PixelGrid`.
 *
 * # Arguments
 *
 * * `this`: instance to read from
 * * `x` and `y`: position of the cell to read
 *
 * # Panics
 *
 * When accessing `x` or `y` out of bounds.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `this` points to a valid `PixelGrid`
 * - `this` is not written to concurrently
 */
bool sp_pixel_grid_get(const struct sp_PixelGrid *this_, size_t x, size_t y);

/**
 * Gets the height in pixels of the `PixelGrid` instance.
 *
 * # Arguments
 *
 * * `this`: instance to read from
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `this` points to a valid `PixelGrid`
 */
size_t sp_pixel_grid_height(const struct sp_PixelGrid *this_);

/**
 * Loads a `PixelGrid` with the specified dimensions from the provided data.
 *
 * # Arguments
 *
 * * `width`: size in pixels in x-direction
 * * `height`: size in pixels in y-direction
 *
 * returns: `PixelGrid` that contains a copy of the provided data
 *
 * # Panics
 *
 * - when the dimensions and data size do not match exactly.
 * - when the width is not dividable by 8
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `data` points to a valid memory location of at least `data_length` bytes in size.
 * - the returned instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_pixel_grid_dealloc`.
 */
struct sp_PixelGrid *sp_pixel_grid_load(size_t width,
                                        size_t height,
                                        const uint8_t *data,
                                        size_t data_length);

/**
 * Creates a new `PixelGrid` with the specified dimensions.
 *
 * # Arguments
 *
 * * `width`: size in pixels in x-direction
 * * `height`: size in pixels in y-direction
 *
 * returns: `PixelGrid` initialized to all pixels off
 *
 * # Panics
 *
 * - when the width is not dividable by 8
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - the returned instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_pixel_grid_dealloc`.
 */
struct sp_PixelGrid *sp_pixel_grid_new(size_t width,
                                       size_t height);

/**
 * Sets the value of the specified position in the `PixelGrid`.
 *
 * # Arguments
 *
 * * `this`: instance to write to
 * * `x` and `y`: position of the cell
 * * `value`: the value to write to the cell
 *
 * returns: old value of the cell
 *
 * # Panics
 *
 * When accessing `x` or `y` out of bounds.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `this` points to a valid `PixelGrid`
 * - `this` is not written to or read from concurrently
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
 * The caller has to make sure that:
 *
 * - `this` points to a valid `PixelGrid`
 * - the returned memory range is never accessed after the passed `PixelGrid` has been freed
 * - the returned memory range is never accessed concurrently, either via the `PixelGrid` or directly
 */
struct sp_CByteSlice sp_pixel_grid_unsafe_data_ref(struct sp_PixelGrid *this_);

/**
 * Gets the width in pixels of the `PixelGrid` instance.
 *
 * # Arguments
 *
 * * `this`: instance to read from
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `this` points to a valid `PixelGrid`
 */
size_t sp_pixel_grid_width(const struct sp_PixelGrid *this_);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
