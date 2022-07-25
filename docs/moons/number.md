# The number moon.

Provides internal functions for the number primitive type.

```
import moons::number
```

## Functions.

### Table.

| Name                  | Params      | Returns | Description                                                                     |
| --------------------- | ----------- | ------- | ------------------------------------------------------------------------------- |
| [from_b2](#from-b2)   | (x: string) | number  | Get a new number from a string.                                                 |
| [from_b8](#from-b8)   | (x: string) | number  | Get a number from binary number (b₂) formatted as a string.                     |
| [from_b16](#from-b16) | (x: string) | number  | Get a number from octadecimal number (b₈) formatted as a string.                |
| [from_str](#from-str) | (x: string) | number  | Get a number from hexadecimal number (b₁₆) formatted as a string.               |
| [new](#new)           | (x: T)      | number  | Get a new number from any primitive data type.                                  |
| [to_b2](#to-b2)       | (x: number) | string  | Get a new binary number (b₂) formatted as a string from a decimal number.       |
| [to_b8](#to-b8)       | (x: number) | string  | Get a new octadecimal number (b₈) formatted as a string from a decimal number.  |
| [to_b16](#to-b16)     | (x: number) | string  | Get a new hexadecimal number (b₁₆) formatted as a string from a decimal number. |

### Descriptions.

---

#### new

Get a new number from any primitive data type.

```
number.new(x: T)
```

where **T** can be:

- **bool**: it will return 0 or 1, respectively.
- **char**: it will return the ASCII index of the character, respectively.
- **number**: it will return the number passed as param, respectively.

---

#### from_b2

Get a number from binary number (b₂) formatted as a string.

```
number.from_b2(x: str)
```

---

#### from_b8

Get a number from octadecimal number (b₈) formatted as a string.

```
number.from_b8(x: str)
```

---

#### from_b16

Get a number from hexadecimal number (b₁₆) formatted as a string.

```
number.from_b16(x: str)
```

---

#### from_str

Get a new a number from a string.

```
number.from_str(x: str)
```

---

#### to_b2

Get a new binary number (b₂) formatted as a string from a decimal number.

```
number.to_b2(x: number)
```

---

#### to_b8

Get a new octadecimal number (b₈) formatted as a string from a decimal number.

```
number.to_b8(x: number)
```

---

#### to_b16

Get a new hexadecimal number (b₁₆) formatted as a string from a decimal number.

```
number.to_b16(x: number)
```

## Number bases.

Zero can interpret and parse the most common number bases, like binary, octal and hexadecimal.

## State

- ✅ Auto imported.
- ☑️ Stable.
