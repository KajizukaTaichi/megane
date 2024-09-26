# Megane
The array-oriented computing model 

### Running example
```
Megane: array-oriented computing model
Input data to be used
> 1
> +
> 2
> +
> 3
>
[src/main.rs:16:5] &data = [
    "1",
    "+",
    "2",
    "+",
    "3",
]
Input meganes to processing the data
> 1 + 2 : 3
> 3 + 3 : 6
>
[src/main.rs:35:5] &meganes = [
    Megane {
        before: [
            "1",
            "+",
            "2",
        ],
        after: "3",
    },
    Megane {
        before: [
            "3",
            "+",
            "3",
        ],
        after: "6",
    },
]
[src/main.rs:53:9] &data = [
    "1",
    "+",
    "2",
    "+",
    "3",
]
[src/main.rs:53:9] &data = [
    "3",
    "+",
    "3",
]
[src/main.rs:38:5] result = Some(
    "6",
)
```
