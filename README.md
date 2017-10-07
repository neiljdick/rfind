# rfind
Simple GNU find alternative written in rust!

# About
GNU find is excellent, but the interface can be clumsy at times. I often find myself typing `find . -name "*foo*"`.
GNU find also has a large amount of extra functionality that I never use. This is a very simple file searching application.

# Examples
Search for foo.c in current directory 

`rfind foo.c`

Search for foo.c in directory 'bar'

`rfind foo.c bar`

Fuzzy Search for files containing 'foo', equivalent to `find . -name "*foo*"`

`rfind -f foo`

`rfind` accepts regular expressions directly. Eg: Search for files ending in .c or .h in the current directory

`rfind "*\.[ch]"`

# Performance
The hard parts are all implemented using the WalkDir and Regex libraries, which are very fast. Performance is at least as good as GNU find in my testing.