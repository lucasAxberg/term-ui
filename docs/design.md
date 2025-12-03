# Plans for term-gui
## Drop
Create a struct for a terminal context that implements the ```Drop``` trait. This ensures that any initialisation, i.e *raw mode*, *hide cursor* and *alternate screen*, is de-initialized on exit or panic.

## Custom errors
Create custom error an make sure to use the result type if a function can fail in more than one "obvious" way. Preferably using the thiserror crate (it makes it much simpler than implementing them yourself).

## Update
Only change the pixels that has a new value, to save time on "rendering".
