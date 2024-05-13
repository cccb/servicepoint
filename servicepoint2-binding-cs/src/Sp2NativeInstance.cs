namespace ServicePoint2;

public abstract class Sp2NativeInstance<T>
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

    private protected unsafe Sp2NativeInstance(T* instance)
    {
        ArgumentNullException.ThrowIfNull(instance);
        _instance = instance;
    }

    protected abstract void Dealloc();

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

    ~Sp2NativeInstance()
    {
        ReleaseUnmanagedResources();
    }
}
