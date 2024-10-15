/* Generated with cbindgen:0.27.0 */

/* Warning, this file is autogenerated by cbindgen. Don't modify this manually. */

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
    SP_COMPRESSION_CODE_UNCOMPRESSED = 0,
    /**
     * compress using flate2 with zlib header
     */
    SP_COMPRESSION_CODE_ZLIB = 26490,
    /**
     * compress using bzip2
     */
    SP_COMPRESSION_CODE_BZIP2 = 25210,
    /**
     * compress using lzma
     */
    SP_COMPRESSION_CODE_LZMA = 27770,
    /**
     * compress using Zstandard
     */
    SP_COMPRESSION_CODE_ZSTD = 31347,
};
#ifndef __cplusplus
typedef uint16_t SPCompressionCode;
#endif // __cplusplus

/**
 * A vector of bits
 *
 * # Examples
 * ```C
 * SPBitVec vec = sp_bitvec_new(8);
 * sp_bitvec_set(vec, 5, true);
 * sp_bitvec_free(vec);
 * ```
 */
typedef struct SPBitVec SPBitVec;

/**
 * A grid of pixels.
 *
 * # Examples
 *
 * ```C
 * Cp437Grid grid = sp_bitmap_new(8, 3);
 * sp_bitmap_fill(grid, true);
 * sp_bitmap_set(grid, 0, 0, false);
 * sp_bitmap_free(grid);
 * ```
 */
typedef struct SPBitmap SPBitmap;

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
 * sp_connection_free(connection);
 * ```
 */
typedef struct SPBrightnessGrid SPBrightnessGrid;

/**
 * A low-level display command.
 *
 * This struct and associated functions implement the UDP protocol for the display.
 *
 * To send a [SPCommand], use a [SPConnection].
 *
 * # Examples
 *
 * ```C
 * sp_connection_send_command(connection, sp_command_clear());
 * sp_connection_send_command(connection, sp_command_brightness(5));
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
 *     sp_connection_send_command(connection, sp_command_clear());
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
 * sp_cp437_grid_free(grid);
 * ```
 */
typedef struct SPCp437Grid SPCp437Grid;

/**
 * The raw packet
 */
typedef struct SPPacket SPPacket;

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
 * Clones a [SPBitmap].
 *
 * Will never return NULL.
 *
 * # Panics
 *
 * - when `bitmap` is NULL
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `bitmap` points to a valid [SPBitmap]
 * - `bitmap` is not written to concurrently
 * - the returned instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_bitmap_free`.
 */
struct SPBitmap *sp_bitmap_clone(const struct SPBitmap *bitmap);

/**
 * Sets the state of all pixels in the [SPBitmap].
 *
 * # Arguments
 *
 * - `bitmap`: instance to write to
 * - `value`: the value to set all pixels to
 *
 * # Panics
 *
 * - when `bitmap` is NULL
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `bitmap` points to a valid [SPBitmap]
 * - `bitmap` is not written to or read from concurrently
 */
void sp_bitmap_fill(struct SPBitmap *bitmap, bool value);

/**
 * Deallocates a [SPBitmap].
 *
 * # Panics
 *
 * - when `bitmap` is NULL
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `bitmap` points to a valid [SPBitmap]
 * - `bitmap` is not used concurrently or after bitmap call
 * - `bitmap` was not passed to another consuming function, e.g. to create a [SPCommand]
 */
void sp_bitmap_free(struct SPBitmap *bitmap);

/**
 * Gets the current value at the specified position in the [SPBitmap].
 *
 * # Arguments
 *
 * - `bitmap`: instance to read from
 * - `x` and `y`: position of the cell to read
 *
 * # Panics
 *
 * - when `bitmap` is NULL
 * - when accessing `x` or `y` out of bounds
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `bitmap` points to a valid [SPBitmap]
 * - `bitmap` is not written to concurrently
 */
bool sp_bitmap_get(const struct SPBitmap *bitmap, size_t x, size_t y);

/**
 * Gets the height in pixels of the [SPBitmap] instance.
 *
 * # Arguments
 *
 * - `bitmap`: instance to read from
 *
 * # Panics
 *
 * - when `bitmap` is NULL
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `bitmap` points to a valid [SPBitmap]
 */
size_t sp_bitmap_height(const struct SPBitmap *bitmap);

/**
 * Loads a [SPBitmap] with the specified dimensions from the provided data.
 *
 * # Arguments
 *
 * - `width`: size in pixels in x-direction
 * - `height`: size in pixels in y-direction
 *
 * returns: [SPBitmap] that contains a copy of the provided data. Will never return NULL.
 *
 * # Panics
 *
 * - when `data` is NULL
 * - when the dimensions and data size do not match exactly.
 * - when the width is not dividable by 8
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `data` points to a valid memory location of at least `data_length` bytes in size.
 * - the returned instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_bitmap_free`.
 */
struct SPBitmap *sp_bitmap_load(size_t width,
                                size_t height,
                                const uint8_t *data,
                                size_t data_length);

/**
 * Creates a new [SPBitmap] with the specified dimensions.
 *
 * # Arguments
 *
 * - `width`: size in pixels in x-direction
 * - `height`: size in pixels in y-direction
 *
 * returns: [SPBitmap] initialized to all pixels off. Will never return NULL.
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
 *   by explicitly calling `sp_bitmap_free`.
 */
struct SPBitmap *sp_bitmap_new(size_t width,
                               size_t height);

/**
 * Sets the value of the specified position in the [SPBitmap].
 *
 * # Arguments
 *
 * - `bitmap`: instance to write to
 * - `x` and `y`: position of the cell
 * - `value`: the value to write to the cell
 *
 * returns: old value of the cell
 *
 * # Panics
 *
 * - when `bitmap` is NULL
 * - when accessing `x` or `y` out of bounds
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `bitmap` points to a valid [SPBitmap]
 * - `bitmap` is not written to or read from concurrently
 */
void sp_bitmap_set(struct SPBitmap *bitmap, size_t x, size_t y, bool value);

/**
 * Gets an unsafe reference to the data of the [SPBitmap] instance.
 *
 * # Panics
 *
 * - when `bitmap` is NULL
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `bitmap` points to a valid [SPBitmap]
 * - the returned memory range is never accessed after the passed [SPBitmap] has been freed
 * - the returned memory range is never accessed concurrently, either via the [SPBitmap] or directly
 */
struct SPByteSlice sp_bitmap_unsafe_data_ref(struct SPBitmap *bitmap);

/**
 * Gets the width in pixels of the [SPBitmap] instance.
 *
 * # Arguments
 *
 * - `bitmap`: instance to read from
 *
 * # Panics
 *
 * - when `bitmap` is NULL
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `bitmap` points to a valid [SPBitmap]
 */
size_t sp_bitmap_width(const struct SPBitmap *bitmap);

/**
 * Clones a [SPBitVec].
 *
 * returns: new [SPBitVec] instance. Will never return NULL.
 *
 * # Panics
 *
 * - when `bit_vec` is NULL
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `bit_vec` points to a valid [SPBitVec]
 * - `bit_vec` is not written to concurrently
 * - the returned instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_bitvec_free`.
 */
struct SPBitVec *sp_bitvec_clone(const struct SPBitVec *bit_vec);

/**
 * Sets the value of all bits in the [SPBitVec].
 *
 * # Arguments
 *
 * - `bit_vec`: instance to write to
 * - `value`: the value to set all bits to
 *
 * # Panics
 *
 * - when `bit_vec` is NULL
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `bit_vec` points to a valid [SPBitVec]
 * - `bit_vec` is not written to or read from concurrently
 */
void sp_bitvec_fill(struct SPBitVec *bit_vec, bool value);

/**
 * Deallocates a [SPBitVec].
 *
 * # Panics
 *
 * - when `but_vec` is NULL
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `bit_vec` points to a valid [SPBitVec]
 * - `bit_vec` is not used concurrently or after this call
 * - `bit_vec` was not passed to another consuming function, e.g. to create a [SPCommand]
 */
void sp_bitvec_free(struct SPBitVec *bit_vec);

/**
 * Gets the value of a bit from the [SPBitVec].
 *
 * # Arguments
 *
 * - `bit_vec`: instance to read from
 * - `index`: the bit index to read
 *
 * returns: value of the bit
 *
 * # Panics
 *
 * - when `bit_vec` is NULL
 * - when accessing `index` out of bounds
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `bit_vec` points to a valid [SPBitVec]
 * - `bit_vec` is not written to concurrently
 */
bool sp_bitvec_get(const struct SPBitVec *bit_vec, size_t index);

/**
 * Returns true if length is 0.
 *
 * # Arguments
 *
 * - `bit_vec`: instance to write to
 *
 * # Panics
 *
 * - when `bit_vec` is NULL
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `bit_vec` points to a valid [SPBitVec]
 */
bool sp_bitvec_is_empty(const struct SPBitVec *bit_vec);

/**
 * Gets the length of the [SPBitVec] in bits.
 *
 * # Arguments
 *
 * - `bit_vec`: instance to write to
 *
 * # Panics
 *
 * - when `bit_vec` is NULL
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `bit_vec` points to a valid [SPBitVec]
 */
size_t sp_bitvec_len(const struct SPBitVec *bit_vec);

/**
 * Interpret the data as a series of bits and load then into a new [SPBitVec] instance.
 *
 * returns: [SPBitVec] instance containing data. Will never return NULL.
 *
 * # Panics
 *
 * - when `data` is NULL
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `data` points to a valid memory location of at least `data_length`
 *   bytes in size.
 * - the returned instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_bitvec_free`.
 */
struct SPBitVec *sp_bitvec_load(const uint8_t *data,
                                size_t data_length);

/**
 * Creates a new [SPBitVec] instance.
 *
 * # Arguments
 *
 * - `size`: size in bits.
 *
 * returns: [SPBitVec] with all bits set to false. Will never return NULL.
 *
 * # Panics
 *
 * - when `size` is not divisible by 8.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - the returned instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_bitvec_free`.
 */
struct SPBitVec *sp_bitvec_new(size_t size);

/**
 * Sets the value of a bit in the [SPBitVec].
 *
 * # Arguments
 *
 * - `bit_vec`: instance to write to
 * - `index`: the bit index to edit
 * - `value`: the value to set the bit to
 *
 * # Panics
 *
 * - when `bit_vec` is NULL
 * - when accessing `index` out of bounds
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `bit_vec` points to a valid [SPBitVec]
 * - `bit_vec` is not written to or read from concurrently
 */
void sp_bitvec_set(struct SPBitVec *bit_vec, size_t index, bool value);

/**
 * Gets an unsafe reference to the data of the [SPBitVec] instance.
 *
 * # Arguments
 *
 * - `bit_vec`: instance to write to
 *
 * # Panics
 *
 * - when `bit_vec` is NULL
 *
 * ## Safety
 *
 * The caller has to make sure that:
 *
 * - `bit_vec` points to a valid [SPBitVec]
 * - the returned memory range is never accessed after the passed [SPBitVec] has been freed
 * - the returned memory range is never accessed concurrently, either via the [SPBitVec] or directly
 */
struct SPByteSlice sp_bitvec_unsafe_data_ref(struct SPBitVec *bit_vec);

/**
 * Clones a [SPBrightnessGrid].
 *
 * # Arguments
 *
 * - `brightness_grid`: instance to read from
 *
 * returns: new [SPBrightnessGrid] instance. Will never return NULL.
 *
 * # Panics
 *
 * - when `brightness_grid` is NULL
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `brightness_grid` points to a valid [SPBrightnessGrid]
 * - `brightness_grid` is not written to concurrently
 * - the returned instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_brightness_grid_free`.
 */
struct SPBrightnessGrid *sp_brightness_grid_clone(const struct SPBrightnessGrid *brightness_grid);

/**
 * Sets the value of all cells in the [SPBrightnessGrid].
 *
 * # Arguments
 *
 * - `brightness_grid`: instance to write to
 * - `value`: the value to set all cells to
 *
 * # Panics
 *
 * - when `brightness_grid` is NULL
 * - When providing an invalid brightness value
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `brightness_grid` points to a valid [SPBrightnessGrid]
 * - `brightness_grid` is not written to or read from concurrently
 */
void sp_brightness_grid_fill(struct SPBrightnessGrid *brightness_grid,
                             uint8_t value);

/**
 * Deallocates a [SPBrightnessGrid].
 *
 * # Arguments
 *
 * - `brightness_grid`: instance to read from
 *
 * # Panics
 *
 * - when `brightness_grid` is NULL
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `brightness_grid` points to a valid [SPBrightnessGrid]
 * - `brightness_grid` is not used concurrently or after this call
 * - `brightness_grid` was not passed to another consuming function, e.g. to create a [SPCommand]
 */
void sp_brightness_grid_free(struct SPBrightnessGrid *brightness_grid);

/**
 * Gets the current value at the specified position.
 *
 * # Arguments
 *
 * - `brightness_grid`: instance to read from
 * - `x` and `y`: position of the cell to read
 *
 * returns: value at position
 *
 * # Panics
 *
 * - when `brightness_grid` is NULL
 * - When accessing `x` or `y` out of bounds.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `brightness_grid` points to a valid [SPBrightnessGrid]
 * - `brightness_grid` is not written to concurrently
 */
uint8_t sp_brightness_grid_get(const struct SPBrightnessGrid *brightness_grid,
                               size_t x,
                               size_t y);

/**
 * Gets the height of the [SPBrightnessGrid] instance.
 *
 * # Arguments
 *
 * - `brightness_grid`: instance to read from
 *
 * returns: height
 *
 * # Panics
 *
 * - when `brightness_grid` is NULL
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `brightness_grid` points to a valid [SPBrightnessGrid]
 */
size_t sp_brightness_grid_height(const struct SPBrightnessGrid *brightness_grid);

/**
 * Loads a [SPBrightnessGrid] with the specified dimensions from the provided data.
 *
 * returns: new [SPBrightnessGrid] instance. Will never return NULL.
 *
 * # Panics
 *
 * - when `data` is NULL
 * - when the provided `data_length` does not match `height` and `width`
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `data` points to a valid memory location of at least `data_length`
 *   bytes in size.
 * - the returned instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_brightness_grid_free`.
 */
struct SPBrightnessGrid *sp_brightness_grid_load(size_t width,
                                                 size_t height,
                                                 const uint8_t *data,
                                                 size_t data_length);

/**
 * Creates a new [SPBrightnessGrid] with the specified dimensions.
 *
 * returns: [SPBrightnessGrid] initialized to 0. Will never return NULL.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - the returned instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_brightness_grid_free`.
 */
struct SPBrightnessGrid *sp_brightness_grid_new(size_t width,
                                                size_t height);

/**
 * Sets the value of the specified position in the [SPBrightnessGrid].
 *
 * # Arguments
 *
 * - `brightness_grid`: instance to write to
 * - `x` and `y`: position of the cell
 * - `value`: the value to write to the cell
 *
 * returns: old value of the cell
 *
 * # Panics
 *
 * - when `brightness_grid` is NULL
 * - When accessing `x` or `y` out of bounds.
 * - When providing an invalid brightness value
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `brightness_grid` points to a valid [SPBitVec]
 * - `brightness_grid` is not written to or read from concurrently
 */
void sp_brightness_grid_set(struct SPBrightnessGrid *brightness_grid,
                            size_t x,
                            size_t y,
                            uint8_t value);

/**
 * Gets an unsafe reference to the data of the [SPBrightnessGrid] instance.
 *
 * # Arguments
 *
 * - `brightness_grid`: instance to read from
 *
 * returns: slice of bytes underlying the `brightness_grid`.
 *
 * # Panics
 *
 * - when `brightness_grid` is NULL
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `brightness_grid` points to a valid [SPBrightnessGrid]
 * - the returned memory range is never accessed after the passed [SPBrightnessGrid] has been freed
 * - the returned memory range is never accessed concurrently, either via the [SPBrightnessGrid] or directly
 */
struct SPByteSlice sp_brightness_grid_unsafe_data_ref(struct SPBrightnessGrid *brightness_grid);

/**
 * Gets the width of the [SPBrightnessGrid] instance.
 *
 * # Arguments
 *
 * - `brightness_grid`: instance to read from
 *
 * returns: width
 *
 * # Panics
 *
 * - when `brightness_grid` is NULL
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `brightness_grid` points to a valid [SPBrightnessGrid]
 */
size_t sp_brightness_grid_width(const struct SPBrightnessGrid *brightness_grid);

/**
 * Set pixel data starting at the pixel offset on screen.
 *
 * The screen will continuously overwrite more pixel data without regarding the offset, meaning
 * once the starting row is full, overwriting will continue on column 0.
 *
 * The contained [SPBitVec] is always uncompressed.
 *
 * The passed [SPBitVec] gets consumed.
 *
 * Returns: a new [Command::BitmapLinear] instance. Will never return NULL.
 *
 * # Panics
 *
 * - when `bit_vec` is null
 * - when `compression_code` is not a valid value
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `bit_vec` points to a valid instance of [SPBitVec]
 * - `bit_vec` is not used concurrently or after this call
 * - `compression` matches one of the allowed enum values
 * - the returned [SPCommand] instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_command_free`.
 */
struct SPCommand *sp_command_bitmap_linear(size_t offset,
                                           struct SPBitVec *bit_vec,
                                           SPCompressionCode compression);

/**
 * Set pixel data according to an and-mask starting at the offset.
 *
 * The screen will continuously overwrite more pixel data without regarding the offset, meaning
 * once the starting row is full, overwriting will continue on column 0.
 *
 * The contained [SPBitVec] is always uncompressed.
 *
 * The passed [SPBitVec] gets consumed.
 *
 * Returns: a new [Command::BitmapLinearAnd] instance. Will never return NULL.
 *
 * # Panics
 *
 * - when `bit_vec` is null
 * - when `compression_code` is not a valid value
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `bit_vec` points to a valid instance of [SPBitVec]
 * - `bit_vec` is not used concurrently or after this call
 * - `compression` matches one of the allowed enum values
 * - the returned [SPCommand] instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_command_free`.
 */
struct SPCommand *sp_command_bitmap_linear_and(size_t offset,
                                               struct SPBitVec *bit_vec,
                                               SPCompressionCode compression);

/**
 * Set pixel data according to an or-mask starting at the offset.
 *
 * The screen will continuously overwrite more pixel data without regarding the offset, meaning
 * once the starting row is full, overwriting will continue on column 0.
 *
 * The contained [SPBitVec] is always uncompressed.
 *
 * The passed [SPBitVec] gets consumed.
 *
 * Returns: a new [Command::BitmapLinearOr] instance. Will never return NULL.
 *
 * # Panics
 *
 * - when `bit_vec` is null
 * - when `compression_code` is not a valid value
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `bit_vec` points to a valid instance of [SPBitVec]
 * - `bit_vec` is not used concurrently or after this call
 * - `compression` matches one of the allowed enum values
 * - the returned [SPCommand] instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_command_free`.
 */
struct SPCommand *sp_command_bitmap_linear_or(size_t offset,
                                              struct SPBitVec *bit_vec,
                                              SPCompressionCode compression);

/**
 * Sets a window of pixels to the specified values.
 *
 * The passed [SPBitmap] gets consumed.
 *
 * Returns: a new [Command::BitmapLinearWin] instance. Will never return NULL.
 *
 * # Panics
 *
 * - when `bitmap` is null
 * - when `compression_code` is not a valid value
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `bitmap` points to a valid instance of [SPBitmap]
 * - `bitmap` is not used concurrently or after this call
 * - `compression` matches one of the allowed enum values
 * - the returned [SPCommand] instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_command_free`.
 */
struct SPCommand *sp_command_bitmap_linear_win(size_t x,
                                               size_t y,
                                               struct SPBitmap *bitmap,
                                               SPCompressionCode compression_code);

/**
 * Set pixel data according to a xor-mask starting at the offset.
 *
 * The screen will continuously overwrite more pixel data without regarding the offset, meaning
 * once the starting row is full, overwriting will continue on column 0.
 *
 * The contained [SPBitVec] is always uncompressed.
 *
 * The passed [SPBitVec] gets consumed.
 *
 * Returns: a new [Command::BitmapLinearXor] instance. Will never return NULL.
 *
 * # Panics
 *
 * - when `bit_vec` is null
 * - when `compression_code` is not a valid value
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `bit_vec` points to a valid instance of [SPBitVec]
 * - `bit_vec` is not used concurrently or after this call
 * - `compression` matches one of the allowed enum values
 * - the returned [SPCommand] instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_command_free`.
 */
struct SPCommand *sp_command_bitmap_linear_xor(size_t offset,
                                               struct SPBitVec *bit_vec,
                                               SPCompressionCode compression);

/**
 * Set the brightness of all tiles to the same value.
 *
 * Returns: a new [Command::Brightness] instance. Will never return NULL.
 *
 * # Panics
 *
 * - When the provided brightness value is out of range (0-11).
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - the returned [SPCommand] instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_command_free`.
 */
struct SPCommand *sp_command_brightness(uint8_t brightness);

/**
 * Set the brightness of individual tiles in a rectangular area of the display.
 *
 * The passed [SPBrightnessGrid] gets consumed.
 *
 * Returns: a new [Command::CharBrightness] instance. Will never return NULL.
 *
 * # Panics
 *
 * - when `grid` is NULL
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `grid` points to a valid instance of [SPBrightnessGrid]
 * - `grid` is not used concurrently or after this call
 * - the returned [SPCommand] instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_command_free`.
 */
struct SPCommand *sp_command_char_brightness(size_t x,
                                             size_t y,
                                             struct SPBrightnessGrid *grid);

/**
 * Set all pixels to the off state.
 *
 * Does not affect brightness.
 *
 * Returns: a new [Command::Clear] instance. Will never return NULL.
 *
 * # Examples
 *
 * ```C
 * sp_connection_send_command(connection, sp_command_clear());
 * ```
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - the returned [SPCommand] instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_command_free`.
 */
struct SPCommand *sp_command_clear(void);

/**
 * Clones a [SPCommand] instance.
 *
 * returns: new [SPCommand] instance. Will never return NULL.
 *
 * # Panics
 *
 * - when `command` is NULL
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `command` points to a valid instance of [SPCommand]
 * - `command` is not written to concurrently
 * - the returned [SPCommand] instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_command_free`.
 */
struct SPCommand *sp_command_clone(const struct SPCommand *command);

/**
 * Show text on the screen.
 *
 * The passed [SPCp437Grid] gets consumed.
 *
 * Returns: a new [Command::Cp437Data] instance. Will never return NULL.
 *
 * # Panics
 *
 * - when `grid` is null
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `grid` points to a valid instance of [SPCp437Grid]
 * - `grid` is not used concurrently or after this call
 * - the returned [SPCommand] instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_command_free`.
 */
struct SPCommand *sp_command_cp437_data(size_t x,
                                        size_t y,
                                        struct SPCp437Grid *grid);

/**
 * A yet-to-be-tested command.
 *
 * Returns: a new `Command::FadeOut` instance. Will never return NULL.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - the returned [SPCommand] instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_command_free`.
 */
struct SPCommand *sp_command_fade_out(void);

/**
 * Deallocates a [SPCommand].
 *
 * # Examples
 *
 * ```C
 * SPCommand c = sp_command_clear();
 * sp_command_free(c);
 * ```
 *
 * # Panics
 *
 * - when `command` is NULL
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `command` points to a valid [SPCommand]
 * - `command` is not used concurrently or after this call
 * - `command` was not passed to another consuming function, e.g. to create a [SPPacket]
 */
void sp_command_free(struct SPCommand *command);

/**
 * Kills the udp daemon on the display, which usually results in a restart.
 *
 * Please do not send this in your normal program flow.
 *
 * Returns: a new [Command::HardReset] instance. Will never return NULL.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - the returned [SPCommand] instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_command_free`.
 */
struct SPCommand *sp_command_hard_reset(void);

/**
 * Tries to turn a [SPPacket] into a [SPCommand].
 *
 * The packet is deallocated in the process.
 *
 * Returns: pointer to new [SPCommand] instance or NULL if parsing failed.
 *
 * # Panics
 *
 * - when `packet` is NULL
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - [SPPacket] points to a valid instance of [SPPacket]
 * - [SPPacket] is not used concurrently or after this call
 * - the result is checked for NULL
 * - the returned [SPCommand] instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_command_free`.
 */
struct SPCommand *sp_command_try_from_packet(struct SPPacket *packet);

/**
 * Closes and deallocates a [SPConnection].
 *
 * # Panics
 *
 * - when `connection` is NULL
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `connection` points to a valid [SPConnection]
 * - `connection` is not used concurrently or after this call
 */
void sp_connection_free(struct SPConnection *connection);

/**
 * Creates a new instance of [SPConnection].
 *
 * returns: NULL if connection fails, or connected instance
 *
 * # Panics
 *
 * - when `host` is null or an invalid host
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - the returned instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_connection_free`.
 */
struct SPConnection *sp_connection_open(const char *host);

/**
 * Sends a [SPCommand] to the display using the [SPConnection].
 *
 * The passed `command` gets consumed.
 *
 * returns: true in case of success
 *
 * # Panics
 *
 * - when `connection` is NULL
 * - when `command` is NULL
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `connection` points to a valid instance of [SPConnection]
 * - `command` points to a valid instance of [SPPacket]
 * - `command` is not used concurrently or after this call
 */
bool sp_connection_send_command(const struct SPConnection *connection,
                                struct SPCommand *command);

/**
 * Sends a [SPPacket] to the display using the [SPConnection].
 *
 * The passed `packet` gets consumed.
 *
 * returns: true in case of success
 *
 * # Panics
 *
 * - when `connection` is NULL
 * - when `packet` is NULL
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `connection` points to a valid instance of [SPConnection]
 * - `packet` points to a valid instance of [SPPacket]
 * - `packet` is not used concurrently or after this call
 */
bool sp_connection_send_packet(const struct SPConnection *connection,
                               struct SPPacket *packet);

/**
 * Clones a [SPCp437Grid].
 *
 * Will never return NULL.
 *
 * # Panics
 *
 * - when `cp437_grid` is NULL
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `cp437_grid` points to a valid [SPCp437Grid]
 * - `cp437_grid` is not written to concurrently
 * - the returned instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_cp437_grid_free`.
 */
struct SPCp437Grid *sp_cp437_grid_clone(const struct SPCp437Grid *cp437_grid);

/**
 * Sets the value of all cells in the [SPCp437Grid].
 *
 * # Arguments
 *
 * - `cp437_grid`: instance to write to
 * - `value`: the value to set all cells to
 *
 * # Panics
 *
 * - when `cp437_grid` is NULL
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `cp437_grid` points to a valid [SPCp437Grid]
 * - `cp437_grid` is not written to or read from concurrently
 */
void sp_cp437_grid_fill(struct SPCp437Grid *cp437_grid, uint8_t value);

/**
 * Deallocates a [SPCp437Grid].
 *
 * # Panics
 *
 * - when `cp437_grid` is NULL
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `cp437_grid` points to a valid [SPCp437Grid]
 * - `cp437_grid` is not used concurrently or after cp437_grid call
 * - `cp437_grid` was not passed to another consuming function, e.g. to create a [SPCommand]
 */
void sp_cp437_grid_free(struct SPCp437Grid *cp437_grid);

/**
 * Gets the current value at the specified position.
 *
 * # Arguments
 *
 * - `cp437_grid`: instance to read from
 * - `x` and `y`: position of the cell to read
 *
 * # Panics
 *
 * - when `cp437_grid` is NULL
 * - when accessing `x` or `y` out of bounds
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `cp437_grid` points to a valid [SPCp437Grid]
 * - `cp437_grid` is not written to concurrently
 */
uint8_t sp_cp437_grid_get(const struct SPCp437Grid *cp437_grid,
                          size_t x,
                          size_t y);

/**
 * Gets the height of the [SPCp437Grid] instance.
 *
 * # Arguments
 *
 * - `cp437_grid`: instance to read from
 *
 * # Panics
 *
 * - when `cp437_grid` is NULL
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `cp437_grid` points to a valid [SPCp437Grid]
 */
size_t sp_cp437_grid_height(const struct SPCp437Grid *cp437_grid);

/**
 * Loads a [SPCp437Grid] with the specified dimensions from the provided data.
 *
 * Will never return NULL.
 *
 * # Panics
 *
 * - when `data` is NULL
 * - when the provided `data_length` does not match `height` and `width`
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `data` points to a valid memory location of at least `data_length`
 *   bytes in size.
 * - the returned instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_cp437_grid_free`.
 */
struct SPCp437Grid *sp_cp437_grid_load(size_t width,
                                       size_t height,
                                       const uint8_t *data,
                                       size_t data_length);

/**
 * Creates a new [SPCp437Grid] with the specified dimensions.
 *
 * returns: [SPCp437Grid] initialized to 0. Will never return NULL.
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - the returned instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_cp437_grid_free`.
 */
struct SPCp437Grid *sp_cp437_grid_new(size_t width,
                                      size_t height);

/**
 * Sets the value of the specified position in the [SPCp437Grid].
 *
 * # Arguments
 *
 * - `cp437_grid`: instance to write to
 * - `x` and `y`: position of the cell
 * - `value`: the value to write to the cell
 *
 * returns: old value of the cell
 *
 * # Panics
 *
 * - when `cp437_grid` is NULL
 * - when accessing `x` or `y` out of bounds
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `cp437_grid` points to a valid [SPBitVec]
 * - `cp437_grid` is not written to or read from concurrently
 */
void sp_cp437_grid_set(struct SPCp437Grid *cp437_grid,
                       size_t x,
                       size_t y,
                       uint8_t value);

/**
 * Gets an unsafe reference to the data of the [SPCp437Grid] instance.
 *
 * Will never return NULL.
 *
 * # Panics
 *
 * - when `cp437_grid` is NULL
 *
 * ## Safety
 *
 * The caller has to make sure that:
 *
 * - `cp437_grid` points to a valid [SPCp437Grid]
 * - the returned memory range is never accessed after the passed [SPCp437Grid] has been freed
 * - the returned memory range is never accessed concurrently, either via the [SPCp437Grid] or directly
 */
struct SPByteSlice sp_cp437_grid_unsafe_data_ref(struct SPCp437Grid *cp437_grid);

/**
 * Gets the width of the [SPCp437Grid] instance.
 *
 * # Arguments
 *
 * - `cp437_grid`: instance to read from
 *
 * # Panics
 *
 * - when `cp437_grid` is NULL
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `cp437_grid` points to a valid [SPCp437Grid]
 */
size_t sp_cp437_grid_width(const struct SPCp437Grid *cp437_grid);

/**
 * Clones a [SPPacket].
 *
 * Will never return NULL.
 *
 * # Panics
 *
 * - when `packet` is NULL
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `packet` points to a valid [SPPacket]
 * - `packet` is not written to concurrently
 * - the returned instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_packet_free`.
 */
struct SPPacket *sp_packet_clone(const struct SPPacket *packet);

/**
 * Deallocates a [SPPacket].
 *
 * # Panics
 *
 * - when `sp_packet_free` is NULL
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `packet` points to a valid [SPPacket]
 * - `packet` is not used concurrently or after this call
 */
void sp_packet_free(struct SPPacket *packet);

/**
 * Turns a [SPCommand] into a [SPPacket].
 * The [SPCommand] gets consumed.
 *
 * Will never return NULL.
 *
 * # Panics
 *
 * - when `command` is NULL
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - [SPCommand] points to a valid instance of [SPCommand]
 * - [SPCommand] is not used concurrently or after this call
 * - the returned [SPPacket] instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_packet_free`.
 */
struct SPPacket *sp_packet_from_command(struct SPCommand *command);

/**
 * Tries to load a [SPPacket] from the passed array with the specified length.
 *
 * returns: NULL in case of an error, pointer to the allocated packet otherwise
 *
 * # Panics
 *
 * - when `data` is NULL
 *
 * # Safety
 *
 * The caller has to make sure that:
 *
 * - `data` points to a valid memory region of at least `length` bytes
 * - `data` is not written to concurrently
 * - the returned [SPPacket] instance is freed in some way, either by using a consuming function or
 *   by explicitly calling `sp_packet_free`.
 */
struct SPPacket *sp_packet_try_load(const uint8_t *data,
                                    size_t length);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
