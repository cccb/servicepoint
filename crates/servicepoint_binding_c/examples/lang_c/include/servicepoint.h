/* Generated with cbindgen:0.26.0 */

#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * pixel count on whole screen
 */
#define SP_PIXEL_COUNT (SP_PIXEL_WIDTH * SP_PIXEL_HEIGHT)

/**
 * Display height in pixels
 */
#define SP_PIXEL_HEIGHT (SP_TILE_HEIGHT * SP_TILE_SIZE)

/**
 * Display width in pixels
 */
#define SP_PIXEL_WIDTH (SP_TILE_WIDTH * SP_TILE_SIZE)

/**
 * Display tile count in the y-direction
 */
#define SP_TILE_HEIGHT 20

/**
 * size of a single tile in one dimension
 */
#define SP_TILE_SIZE 8

/**
 * Display tile count in the x-direction
 */
#define SP_TILE_WIDTH 56

/**
 * Specifies the kind of compression to use.
 */
enum SPCompressionCode
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
typedef uint16_t SPCompressionCode;
#endif // __cplusplus

/**
 * A vector of bits
 *
 * # Examples
 * ```C
 * SPBitVec vec = sp_bit_vec_new(8);
 * sp_bit_vec_set(vec, 5, true);
 * sp_bit_vec_dealloc(vec);
 * ```
 */
typedef struct SPBitVec SPBitVec;

/**
 * A grid containing brightness values.
 *
 * # Examples
 * ```C
 * SPConnection connection = sp_connection_open("127.0.0.1:2342");
 * if (connection == NULL)
 *     return 1;
 *
 * SPBrightnessGrid grid = sp_brightness_grid_new(2, 2);
 * sp_brightness_grid_set(grid, 0, 0, 0);
 * sp_brightness_grid_set(grid, 1, 1, 10);
 *
 * SPCommand command = sp_command_char_brightness(grid);
 * sp_connection_dealloc(connection);
 * ```
 */
typedef struct SPBrightnessGrid SPBrightnessGrid;

/**
 * A low-level display command.
 *
 * This struct and associated functions implement the UDP protocol for the display.
 *
 * To send a `SPCommand`, use a `SPConnection`.
 *
 * # Examples
 *
 * ```C
 * sp_connection_send(connection, sp_command_clear());
 * sp_connection_send(connection, sp_command_brightness(5));
 * ```
 */
typedef struct SPCommand SPCommand;

/**
 * A connection to the display.
 *
 * # Examples
 *
 * ```C
 * CConnection connection = sp_connection_open("172.23.42.29:2342");
 * if (connection != NULL)
 *     sp_connection_send(connection, sp_command_clear());
 * ```
 */
typedef struct SPConnection SPConnection;

/**
 * A C-wrapper for grid containing codepage 437 characters.
 *
 * The encoding is currently not enforced.
 *
 * # Examples
 *
 * ```C
 * Cp437Grid grid = sp_cp437_grid_new(4, 3);
 * sp_cp437_grid_fill(grid, '?');
 * sp_cp437_grid_set(grid, 0, 0, '!');
 * sp_cp437_grid_dealloc(grid);
 * ```
 */
typedef struct SPCp437Grid SPCp437Grid;

/**
 * The raw packet
 */
typedef struct SPPacket SPPacket;

/**
 * A grid of pixels.
 *
 * # Examples
 *
 * ```C
 * Cp437Grid grid = sp_pixel_grid_new(8, 3);
 * sp_pixel_grid_fill(grid, true);
 * sp_pixel_grid_set(grid, 0, 0, false);
 * sp_pixel_grid_dealloc(grid);
 * ```
 */
typedef struct SPPixelGrid SPPixelGrid;

/**
 * Represents a span of memory (`&mut [u8]` ) as a struct usable by C code.
 *
 * You should not create an instance of this type in your C code.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - accesses to the memory pointed to with `start` is never accessed outside `length`
 * - the lifetime of the `CByteSlice` does not outlive the memory it points to, as described in
 *   the function returning this type.
 * - an instance of this created from C is never passed to a consuming function, as the rust code
 *   will try to free the memory of a potentially separate allocator.
 */
typedef struct SPByteSlice {
    /**
     * The start address of the memory
     */
    uint8_t *start;
    /**
     * The amount of memory in bytes
     */
    size_t length;
} SPByteSlice;

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
struct SPBitVec *sp_bit_vec_clone(const struct SPBitVec *this_);

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
void sp_bit_vec_dealloc(struct SPBitVec *this_);

/**
 * Sets the value of all bits in the `BitVec`.
 *
 * # Arguments
 *
 * - `value`: the value to set all bits to
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `this` points to a valid `BitVec`
 * - `this` is not written to or read from concurrently
 */
void sp_bit_vec_fill(struct SPBitVec *this_, bool value);

/**
 * Gets the value of a bit from the `BitVec`.
 *
 * # Arguments
 *
 * - `this`: instance to read from
 * - `index`: the bit index to read
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
bool sp_bit_vec_get(const struct SPBitVec *this_, size_t index);

/**
 * Returns true if length is 0.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `this` points to a valid `BitVec`
 */
bool sp_bit_vec_is_empty(const struct SPBitVec *this_);

/**
 * Gets the length of the `BitVec` in bits.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `this` points to a valid `BitVec`
 */
size_t sp_bit_vec_len(const struct SPBitVec *this_);

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
struct SPBitVec *sp_bit_vec_load(const uint8_t *data,
                                 size_t data_length);

/**
 * Creates a new `BitVec` instance.
 *
 * # Arguments
 *
 * - `size`: size in bits.
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
struct SPBitVec *sp_bit_vec_new(size_t size);

/**
 * Sets the value of a bit in the `BitVec`.
 *
 * # Arguments
 *
 * - `this`: instance to write to
 * - `index`: the bit index to edit
 * - `value`: the value to set the bit to
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
void sp_bit_vec_set(struct SPBitVec *this_, size_t index, bool value);

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
struct SPByteSlice sp_bit_vec_unsafe_data_ref(struct SPBitVec *this_);

/**
 * Clones a `BrightnessGrid`.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `this` points to a valid `BrightnessGrid`
 * - `this` is not written to concurrently
 * - the returned instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_brightness_grid_dealloc`.
 */
struct SPBrightnessGrid *sp_brightness_grid_clone(const struct SPBrightnessGrid *this_);

/**
 * Deallocates a `BrightnessGrid`.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `this` points to a valid `BrightnessGrid`
 * - `this` is not used concurrently or after this call
 * - `this` was not passed to another consuming function, e.g. to create a `Command`
 */
void sp_brightness_grid_dealloc(struct SPBrightnessGrid *this_);

/**
 * Sets the value of all cells in the `BrightnessGrid`.
 *
 * # Arguments
 *
 * - `this`: instance to write to
 * - `value`: the value to set all cells to
 *
 * # Panics
 *
 * - When providing an invalid brightness value
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `this` points to a valid `BrightnessGrid`
 * - `this` is not written to or read from concurrently
 */
void sp_brightness_grid_fill(struct SPBrightnessGrid *this_, uint8_t value);

/**
 * Gets the current value at the specified position.
 *
 * # Arguments
 *
 * - `this`: instance to read from
 * - `x` and `y`: position of the cell to read
 *
 * # Panics
 *
 * When accessing `x` or `y` out of bounds.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `this` points to a valid `BrightnessGrid`
 * - `this` is not written to concurrently
 */
uint8_t sp_brightness_grid_get(const struct SPBrightnessGrid *this_,
                               size_t x,
                               size_t y);

/**
 * Gets the height of the `BrightnessGrid` instance.
 *
 * # Arguments
 *
 * - `this`: instance to read from
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `this` points to a valid `BrightnessGrid`
 */
size_t sp_brightness_grid_height(const struct SPBrightnessGrid *this_);

/**
 * Loads a `BrightnessGrid` with the specified dimensions from the provided data.
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
 *   by explicitly calling `sp_brightness_grid_dealloc`.
 */
struct SPBrightnessGrid *sp_brightness_grid_load(size_t width,
                                                 size_t height,
                                                 const uint8_t *data,
                                                 size_t data_length);

/**
 * Creates a new `BrightnessGrid` with the specified dimensions.
 *
 * returns: `BrightnessGrid` initialized to 0.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - the returned instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_brightness_grid_dealloc`.
 */
struct SPBrightnessGrid *sp_brightness_grid_new(size_t width,
                                                size_t height);

/**
 * Sets the value of the specified position in the `BrightnessGrid`.
 *
 * # Arguments
 *
 * - `this`: instance to write to
 * - `x` and `y`: position of the cell
 * - `value`: the value to write to the cell
 *
 * returns: old value of the cell
 *
 * # Panics
 *
 * - When accessing `x` or `y` out of bounds.
 * - When providing an invalid brightness value
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `this` points to a valid `BitVec`
 * - `this` is not written to or read from concurrently
 */
void sp_brightness_grid_set(struct SPBrightnessGrid *this_,
                            size_t x,
                            size_t y,
                            uint8_t value);

/**
 * Gets an unsafe reference to the data of the `BrightnessGrid` instance.
 *
 * ## Safety
 *
 * The caller has to make sure that:
 *
 * - `this` points to a valid `BrightnessGrid`
 * - the returned memory range is never accessed after the passed `BrightnessGrid` has been freed
 * - the returned memory range is never accessed concurrently, either via the `BrightnessGrid` or directly
 */
struct SPByteSlice sp_brightness_grid_unsafe_data_ref(struct SPBrightnessGrid *this_);

/**
 * Gets the width of the `BrightnessGrid` instance.
 *
 * # Arguments
 *
 * - `this`: instance to read from
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `this` points to a valid `BrightnessGrid`
 */
size_t sp_brightness_grid_width(const struct SPBrightnessGrid *this_);

/**
 * Allocates a new `Command::BitmapLinear` instance.
 * The passed `BitVec` gets consumed.
 *
 * Set pixel data starting at the pixel offset on screen.
 *
 * The screen will continuously overwrite more pixel data without regarding the offset, meaning
 * once the starting row is full, overwriting will continue on column 0.
 *
 * The contained `BitVec` is always uncompressed.
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
struct SPCommand *sp_command_bitmap_linear(size_t offset,
                                           struct SPBitVec *bit_vec,
                                           SPCompressionCode compression);

/**
 * Allocates a new `Command::BitmapLinearAnd` instance.
 * The passed `BitVec` gets consumed.
 *
 * Set pixel data according to an and-mask starting at the offset.
 *
 * The screen will continuously overwrite more pixel data without regarding the offset, meaning
 * once the starting row is full, overwriting will continue on column 0.
 *
 * The contained `BitVec` is always uncompressed.
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
struct SPCommand *sp_command_bitmap_linear_and(size_t offset,
                                               struct SPBitVec *bit_vec,
                                               SPCompressionCode compression);

/**
 * Allocates a new `Command::BitmapLinearOr` instance.
 * The passed `BitVec` gets consumed.
 *
 * Set pixel data according to an or-mask starting at the offset.
 *
 * The screen will continuously overwrite more pixel data without regarding the offset, meaning
 * once the starting row is full, overwriting will continue on column 0.
 *
 * The contained `BitVec` is always uncompressed.
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
struct SPCommand *sp_command_bitmap_linear_or(size_t offset,
                                              struct SPBitVec *bit_vec,
                                              SPCompressionCode compression);

/**
 * Allocates a new `Command::BitmapLinearWin` instance.
 * The passed `PixelGrid` gets consumed.
 *
 * Sets a window of pixels to the specified values
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
struct SPCommand *sp_command_bitmap_linear_win(size_t x,
                                               size_t y,
                                               struct SPPixelGrid *pixel_grid,
                                               SPCompressionCode compression_code);

/**
 * Allocates a new `Command::BitmapLinearXor` instance.
 * The passed `BitVec` gets consumed.
 *
 * Set pixel data according to a xor-mask starting at the offset.
 *
 * The screen will continuously overwrite more pixel data without regarding the offset, meaning
 * once the starting row is full, overwriting will continue on column 0.
 *
 * The contained `BitVec` is always uncompressed.
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
struct SPCommand *sp_command_bitmap_linear_xor(size_t offset,
                                               struct SPBitVec *bit_vec,
                                               SPCompressionCode compression);

/**
 * Allocates a new `Command::Brightness` instance for setting the brightness of all tiles to the
 * same value.
 *
 * # Panics
 *
 * - When the provided brightness value is out of range (0-11).
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - the returned `SPCommand` instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_command_dealloc`.
 */
struct SPCommand *sp_command_brightness(uint8_t brightness);

/**
 * Allocates a new `Command::CharBrightness` instance.
 * The passed `SPBrightnessGrid` gets consumed.
 *
 * Set the brightness of individual tiles in a rectangular area of the display.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `grid` points to a valid instance of `SPBrightnessGrid`
 * - `grid` is not used concurrently or after this call
 * - the returned `Command` instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_command_dealloc`.
 */
struct SPCommand *sp_command_char_brightness(size_t x,
                                             size_t y,
                                             struct SPBrightnessGrid *grid);

/**
 * Allocates a new `Command::Clear` instance.
 *
 * Set all pixels to the off state. Does not affect brightness.
 *
 * # Examples
 *
 * ```C
 * sp_connection_send(connection, sp_command_clear());
 * ```
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - the returned `Command` instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_command_dealloc`.
 */
struct SPCommand *sp_command_clear(void);

/**
 * Clones a `SPCommand` instance.
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
struct SPCommand *sp_command_clone(const struct SPCommand *original);

/**
 * Allocates a new `Command::Cp437Data` instance.
 * The passed `ByteGrid` gets consumed.
 *
 * Show text on the screen.
 *
 * <div class="warning">
 *     The library does not currently convert between UTF-8 and CP-437.
 *     Because Rust expects UTF-8 strings, it might be necessary to only send ASCII for now.
 * </div>
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
struct SPCommand *sp_command_cp437_data(size_t x,
                                        size_t y,
                                        struct SPCp437Grid *byte_grid);

/**
 * Deallocates a `Command`.
 *
 * # Examples
 *
 * ```C
 * SPCommand c = sp_command_clear();
 * sp_command_dealloc(c);
 * ```
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `this` points to a valid `Command`
 * - `this` is not used concurrently or after this call
 * - `this` was not passed to another consuming function, e.g. to create a `Packet`
 */
void sp_command_dealloc(struct SPCommand *ptr);

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
struct SPCommand *sp_command_fade_out(void);

/**
 * Allocates a new `Command::HardReset` instance.
 *
 * Kills the udp daemon on the display, which usually results in a restart.
 * Please do not send this in your normal program flow.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - the returned `Command` instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_command_dealloc`.
 */
struct SPCommand *sp_command_hard_reset(void);

/**
 * Tries to turn a `SPPacket` into a `SPCommand`. The packet is deallocated in the process.
 *
 * Returns: pointer to new `SPCommand` instance or NULL
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `packet` points to a valid instance of `SPPacket`
 * - `packet` is not used concurrently or after this call
 * - the result is checked for NULL
 * - the returned `SPCommand` instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_command_dealloc`.
 */
struct SPCommand *sp_command_try_from_packet(struct SPPacket *packet);

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
void sp_connection_dealloc(struct SPConnection *ptr);

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
struct SPConnection *sp_connection_open(const char *host);

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
bool sp_connection_send(const struct SPConnection *connection,
                        struct SPPacket *packet);

/**
 * Clones a `Cp437Grid`.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `this` points to a valid `Cp437Grid`
 * - `this` is not written to concurrently
 * - the returned instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_cp437_grid_dealloc`.
 */
struct SPCp437Grid *sp_cp437_grid_clone(const struct SPCp437Grid *this_);

/**
 * Deallocates a `Cp437Grid`.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `this` points to a valid `Cp437Grid`
 * - `this` is not used concurrently or after this call
 * - `this` was not passed to another consuming function, e.g. to create a `Command`
 */
void sp_cp437_grid_dealloc(struct SPCp437Grid *this_);

/**
 * Sets the value of all cells in the `Cp437Grid`.
 *
 * # Arguments
 *
 * - `this`: instance to write to
 * - `value`: the value to set all cells to
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `this` points to a valid `Cp437Grid`
 * - `this` is not written to or read from concurrently
 */
void sp_cp437_grid_fill(struct SPCp437Grid *this_, uint8_t value);

/**
 * Gets the current value at the specified position.
 *
 * # Arguments
 *
 * - `this`: instance to read from
 * - `x` and `y`: position of the cell to read
 *
 * # Panics
 *
 * When accessing `x` or `y` out of bounds.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `this` points to a valid `Cp437Grid`
 * - `this` is not written to concurrently
 */
uint8_t sp_cp437_grid_get(const struct SPCp437Grid *this_, size_t x, size_t y);

/**
 * Gets the height of the `Cp437Grid` instance.
 *
 * # Arguments
 *
 * - `this`: instance to read from
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `this` points to a valid `Cp437Grid`
 */
size_t sp_cp437_grid_height(const struct SPCp437Grid *this_);

/**
 * Loads a `Cp437Grid` with the specified dimensions from the provided data.
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
 *   by explicitly calling `sp_cp437_grid_dealloc`.
 */
struct SPCp437Grid *sp_cp437_grid_load(size_t width,
                                       size_t height,
                                       const uint8_t *data,
                                       size_t data_length);

/**
 * Creates a new `Cp437Grid` with the specified dimensions.
 *
 * returns: `Cp437Grid` initialized to 0.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - the returned instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_cp437_grid_dealloc`.
 */
struct SPCp437Grid *sp_cp437_grid_new(size_t width,
                                      size_t height);

/**
 * Sets the value of the specified position in the `Cp437Grid`.
 *
 * # Arguments
 *
 * - `this`: instance to write to
 * - `x` and `y`: position of the cell
 * - `value`: the value to write to the cell
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
void sp_cp437_grid_set(struct SPCp437Grid *this_,
                       size_t x,
                       size_t y,
                       uint8_t value);

/**
 * Gets an unsafe reference to the data of the `Cp437Grid` instance.
 *
 * ## Safety
 *
 * The caller has to make sure that:
 *
 * - `this` points to a valid `Cp437Grid`
 * - the returned memory range is never accessed after the passed `Cp437Grid` has been freed
 * - the returned memory range is never accessed concurrently, either via the `Cp437Grid` or directly
 */
struct SPByteSlice sp_cp437_grid_unsafe_data_ref(struct SPCp437Grid *this_);

/**
 * Gets the width of the `Cp437Grid` instance.
 *
 * # Arguments
 *
 * - `this`: instance to read from
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `this` points to a valid `Cp437Grid`
 */
size_t sp_cp437_grid_width(const struct SPCp437Grid *this_);

/**
 * Clones a `Packet`.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `this` points to a valid `Packet`
 * - `this` is not written to concurrently
 * - the returned instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_packet_dealloc`.
 */
struct SPPacket *sp_packet_clone(const struct SPPacket *this_);

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
void sp_packet_dealloc(struct SPPacket *this_);

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
struct SPPacket *sp_packet_from_command(struct SPCommand *command);

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
struct SPPacket *sp_packet_try_load(const uint8_t *data,
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
struct SPPixelGrid *sp_pixel_grid_clone(const struct SPPixelGrid *this_);

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
void sp_pixel_grid_dealloc(struct SPPixelGrid *this_);

/**
 * Sets the state of all pixels in the `PixelGrid`.
 *
 * # Arguments
 *
 * - `this`: instance to write to
 * - `value`: the value to set all pixels to
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `this` points to a valid `PixelGrid`
 * - `this` is not written to or read from concurrently
 */
void sp_pixel_grid_fill(struct SPPixelGrid *this_, bool value);

/**
 * Gets the current value at the specified position in the `PixelGrid`.
 *
 * # Arguments
 *
 * - `this`: instance to read from
 * - `x` and `y`: position of the cell to read
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
bool sp_pixel_grid_get(const struct SPPixelGrid *this_, size_t x, size_t y);

/**
 * Gets the height in pixels of the `PixelGrid` instance.
 *
 * # Arguments
 *
 * - `this`: instance to read from
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `this` points to a valid `PixelGrid`
 */
size_t sp_pixel_grid_height(const struct SPPixelGrid *this_);

/**
 * Loads a `PixelGrid` with the specified dimensions from the provided data.
 *
 * # Arguments
 *
 * - `width`: size in pixels in x-direction
 * - `height`: size in pixels in y-direction
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
struct SPPixelGrid *sp_pixel_grid_load(size_t width,
                                       size_t height,
                                       const uint8_t *data,
                                       size_t data_length);

/**
 * Creates a new `PixelGrid` with the specified dimensions.
 *
 * # Arguments
 *
 * - `width`: size in pixels in x-direction
 * - `height`: size in pixels in y-direction
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
struct SPPixelGrid *sp_pixel_grid_new(size_t width,
                                      size_t height);

/**
 * Sets the value of the specified position in the `PixelGrid`.
 *
 * # Arguments
 *
 * - `this`: instance to write to
 * - `x` and `y`: position of the cell
 * - `value`: the value to write to the cell
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
void sp_pixel_grid_set(struct SPPixelGrid *this_,
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
struct SPByteSlice sp_pixel_grid_unsafe_data_ref(struct SPPixelGrid *this_);

/**
 * Gets the width in pixels of the `PixelGrid` instance.
 *
 * # Arguments
 *
 * - `this`: instance to read from
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `this` points to a valid `PixelGrid`
 */
size_t sp_pixel_grid_width(const struct SPPixelGrid *this_);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
