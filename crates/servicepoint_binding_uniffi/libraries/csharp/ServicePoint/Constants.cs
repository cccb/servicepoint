
using ServicePoint;

public static class ServicePointConstants
{
    private static readonly Constants _instance = ServicepointBindingUniffiMethods.GetConstants();

    public static readonly ulong PixelWidth = _instance.pixelWidth;
    public static readonly ulong PixelHeight = _instance.pixelHeight;
    public static readonly ulong PixelCount = _instance.pixelCount;
    public static readonly ulong TileWidth = _instance.tileWidth;
    public static readonly ulong TileHeight = _instance.tileHeight;
    public static readonly ulong TileSize = _instance.tileSize;
}
