namespace ServicePoint;

public abstract class SpNativeInstance<T>
    : IDisposable
    where T : unmanaged
{
    private unsafe T* _instance;

    internal unsafe T* Instance
    {
        get
        {
            if (_instance == null)
                throw new NullReferenceException("instance is null");
            return _instance;
        }
    }

    private protected unsafe SpNativeInstance(T* instance)
    {
        ArgumentNullException.ThrowIfNull(instance);
        _instance = instance;
    }

    private protected abstract void Dealloc();

    internal unsafe T* Into()
    {
        var instance = _instance;
        _instance = null;
        return instance;
    }

    private unsafe void ReleaseUnmanagedResources()
    {
        if (_instance != null)
            Dealloc();
        _instance = null;
    }

    public void Dispose()
    {
        ReleaseUnmanagedResources();
        GC.SuppressFinalize(this);
    }

    ~SpNativeInstance()
    {
        ReleaseUnmanagedResources();
    }
}
