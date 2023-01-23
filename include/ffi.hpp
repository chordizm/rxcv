using namespace std;

template <typename T>
struct FFIResult
{
    T ok;
    char *error;
};

template <typename T, class F>
FFIResult<T> try_execute(F f)
{
    try
    {
        return {f(), nullptr};
    }
    catch (std::exception &e)
    {
        return {nullptr, (char *)(e.what())};
    }
};
