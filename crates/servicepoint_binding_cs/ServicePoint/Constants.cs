using ServicePoint.BindGen;

namespace ServicePoint;

public static class Constants
{
    /// size of a single tile in one dimension
    public const int TileSize = NativeMethods.SP_TILE_SIZE;

    /// tile count in the x-direction
    public const int TileWidth = NativeMethods.SP_TILE_WIDTH;

    /// tile count in the y-direction
    public const int TileHeight = NativeMethods.SP_TILE_SIZE;

    /// screen width in pixels
    public const int PixelWidth = TileWidth * TileSize;

    /// screen height in pixels
    public const int PixelHeight = TileHeight * TileSize;

    /// pixel count on whole screen
    public const int PixelCount = PixelWidth * PixelHeight;
}
