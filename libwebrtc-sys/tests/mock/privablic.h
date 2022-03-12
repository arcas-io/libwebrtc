// Generate a static data member of type Stub::type in which to store
// the address of a private member.  It is crucial that Stub does not
// depend on the /value/ of the the stored address in any way so that
// we can access it from ordinary code without directly touching
// private data.
template<class Stub>
struct member
{
    static typename Stub::type value;
};
template<class Stub>
typename Stub::type member<Stub>::value;

// Generate a static data member whose constructor initializes
// member<Stub>::value. This type will only be named in an explicit
// instantiation, where it is legal to pass the address of a private
// member.
template<class Stub, typename Stub::type x>
struct private_member
{
    private_member()
    {
        member<Stub>::value = x;
    }
    static private_member instance;
};
template<class Stub, typename Stub::type x>
private_member<Stub, x> private_member<Stub, x>::instance;

template<typename Stub>
struct func
{
    /* export it ... */
    typedef typename Stub::type type;
    static type ptr;
};

template<typename Stub>
typename func<Stub>::type func<Stub>::ptr;

template<typename Stub, typename Stub::type p>
struct private_method : func<Stub>
{
    /* fill it ... */
    struct _private_method
    {
        _private_method()
        {
            func<Stub>::ptr = p;
        }
    };
    static _private_method private_method_obj;
};

template<typename Stub, typename Stub::type p>
typename private_method<Stub, p>::_private_method private_method<Stub, p>::private_method_obj;
