namespace ServicePoint;

public static class Constants
{
    /// size of a single tile in one dimension
    public const int TileSize = 8;

    /// tile count in the x-direction
    public const int TileWidth = 56;

    /// tile count in the y-direction
    public const int TileHeight = 20;

    /// screen width in pixels
    public const int PixelWidth = TileWidth * TileSize;

    /// screen height in pixels
    public const int PixelHeight = TileHeight * TileSize;

    /// pixel count on whole screen
    public const int PixelCount = PixelWidth * PixelHeight;
}
