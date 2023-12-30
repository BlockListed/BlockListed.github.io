# Arc<str> vs String, is Arc<str> really faster?
The short (and incorrect) answer is probably,
but that's probably not what we're looking for.

## What does "faster" even mean
In this case we're talking about cloning performance, aka how long
it takes to call the `.clone()` function on the element.

In the case of `String` this means allocating a new string and
then `memcopy`ing the data over.
`Arc<str>` by comparison simply increments the strong reference counter.

## Expectations
You would expect a single increment to always be faster than a call
to `malloc` and then `memcopy`, since, that's supposedly a complex
call which should take much longer than the increment operation.

## Methodology
TODO: FINISH THIS

## Results
![Chart showing performance differences between Arc<str>- and String-cloning](/imgs/arc_str_vs_string_graph.png)
As you can see, our expectations where only fulfilled, when a single
thread accesses the `Arc<str>`. As soon as multiple threads are contending
for the `Arc<str>` we start having issues and the performance of `String`
surpasses the performance of `Arc<str>` significantly. Now the question
remains, why does it have much greater performance?

## Explanation
This information is based on `Jemallocator`, but may also be applicable to other multithreaded allocators.

Firstly, all references of `Arc<str>` refer to the same instance, which
means all clone operations operate on the same atomic integer, which as
I alluded to before causes significant lock contention. `String` by contrast
calls `Jemalloc`s `malloc` implementation, which uses thread-local arenas to
get around this.

Whats still unexplained is why `String` gets *slower* when we *dont* drop
the `String` after each use. This is simply explained by how `Jemalloc` allocates
memory. As you might already know allocating memory from the kernel on Linux
happens using the `mmap` syscall, which takes time. Therefore the allocator
attempts to call `mmap` as little as possible, by putting multiple allocations
into a single `mmap`ed "bucket". Therefore, if we constantly `malloc` then
immediately `free` the allocated memory, we are always using just
a single **thread-local** "bucket" for our allocation, no syscalls involved!
If we instead leak the memory, the allocator now has to constantly allocate
new memory using `mmap`, which, while probably amortized using bigger allocations,
still takes quite a bit of time.

## Conclusion