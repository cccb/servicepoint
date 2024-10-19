namespace ServicePoint;

public static class Constants
{
    /// size of a single tile in one dimension
    public const nuint TileSize = ConstantsNative.SP_TILE_SIZE;

    /// tile count in the x-direction
    public const nuint TileWidth = ConstantsNative.SP_TILE_WIDTH;

    /// tile count in the y-direction
    public const nuint TileHeight = ConstantsNative.SP_TILE_SIZE;

    /// screen width in pixels
    public const nuint PixelWidth = TileWidth * TileSize;

    /// screen height in pixels
    public const nuint PixelHeight = TileHeight * TileSize;

    /// pixel count on whole screen
    public const nuint PixelCount = PixelWidth * PixelHeight;
}
