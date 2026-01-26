# ODK YAML/JSON Schema Format

This document describes the YAML/JSON format for defining ODK schemas.

## File Extensions

- `.odk.yaml` or `.odk.yml` - YAML format
- `.odk.json` - JSON format

## Schema Structure

```yaml
version: "1.0"              # Schema format version (required)
name: "MyAPI"               # Schema namespace name (required)
imports: []                 # TypeSpec imports (optional)
models: {}                  # Model definitions (optional)
unions: {}                  # Union/enum definitions (optional)
raw: ""                     # Raw TypeSpec code (optional)
```

## Models

Models define structured data types with named fields.

```yaml
models:
  User:
    description: "A user account"
    decorators: ["@table"]
    fields:
      id:
        type: string
        format: uuid
        decorators: ["@id"]
      email:
        type: string
        format: email
      name:
        type: string
        optional: true
```

### Field Properties

| Property | Type | Description |
|----------|------|-------------|
| `type` | string | Field type (see Types below) |
| `optional` | boolean | Whether field is optional (default: false) |
| `description` | string | Field documentation |
| `format` | string | Format hint (uuid, email, uri, etc.) |
| `decorators` | string[] | TypeSpec decorators to apply |
| `constraints` | object | Validation constraints |
| `extensions` | object | Custom extension data |

### Constraints

```yaml
fields:
  username:
    type: string
    constraints:
      minLength: 3
      maxLength: 32
      pattern: "^[a-zA-Z0-9_]+$"
  
  age:
    type: int32
    constraints:
      minValue: 0
      maxValue: 150
  
  score:
    type: float64
    constraints:
      minValue: 0.0
      maxValue: 100.0
      multipleOf: 0.5
```

Available constraints:

| Constraint | Applies To | Description |
|------------|------------|-------------|
| `minLength` | string | Minimum string length |
| `maxLength` | string | Maximum string length |
| `pattern` | string | Regex pattern |
| `minValue` | number | Minimum numeric value |
| `maxValue` | number | Maximum numeric value |
| `multipleOf` | number | Value must be multiple of |

## Types

### Scalar Types

| Type | Description |
|------|-------------|
| `string` | UTF-8 string |
| `boolean` | Boolean value |
| `int8` | 8-bit signed integer |
| `int16` | 16-bit signed integer |
| `int32` | 32-bit signed integer |
| `int64` | 64-bit signed integer |
| `uint8` | 8-bit unsigned integer |
| `uint16` | 16-bit unsigned integer |
| `uint32` | 32-bit unsigned integer |
| `uint64` | 64-bit unsigned integer |
| `float32` | 32-bit floating point |
| `float64` | 64-bit floating point |
| `bytes` | Binary data |
| `plainDate` | Date without time |
| `plainTime` | Time without date |
| `utcDateTime` | UTC datetime |
| `offsetDateTime` | Datetime with timezone offset |
| `duration` | Time duration |
| `url` | URL string |

### Array Types

Use `[]` suffix for arrays:

```yaml
fields:
  tags:
    type: string[]
  posts:
    type: Post[]
```

### References

Reference other models/unions by name:

```yaml
fields:
  author:
    type: User
  status:
    type: PostStatus
```

## Unions

Unions define a type that can be one of several variants.

### String Literal Unions (Enums)

```yaml
unions:
  UserRole:
    description: "User permission levels"
    variants:
      - admin
      - author
      - reader
```

### Type Unions

```yaml
unions:
  SearchResult:
    variants:
      - type: User
        name: UserResult
      - type: Post
        name: PostResult
```

## Decorators

Decorators are TypeSpec annotations that add metadata.

```yaml
models:
  User:
    decorators: ["@table", "@doc('User table')"]
    fields:
      id:
        type: string
        decorators: ["@id", "@key"]
```

Common decorators:

| Decorator | Description |
|-----------|-------------|
| `@table` | Mark as database table |
| `@id` | Primary key field |
| `@key` | Unique key field |
| `@doc("...")` | Documentation |
| `@minLength(n)` | Minimum length |
| `@maxLength(n)` | Maximum length |
| `@pattern("...")` | Regex pattern |
| `@manyToOne(Model)` | Foreign key relation |
| `@oneToMany` | Inverse relation |
| `@createdAt` | Auto-set on create |
| `@updatedAt` | Auto-set on update |

## Raw TypeSpec

The `raw` field allows inserting arbitrary TypeSpec code:

```yaml
raw: |
  @route("/api")
  interface MyApi {
    @get list(): User[];
    @post create(@body user: User): User;
  }
```

Use raw blocks for:
- API route definitions
- Complex type expressions
- TypeSpec features not supported by ODK schema

## Imports

Specify TypeSpec packages to import:

```yaml
imports:
  - "@typespec/http"
  - "@typespec/rest"
  - "@omnidotdev/typespec-drizzle"
```

## Extensions

Store custom metadata that doesn't map to TypeSpec:

```yaml
fields:
  id:
    type: string
    extensions:
      zodTransform: "x => x.toUpperCase()"
      customValidator: "validateId"
```

## Complete Example

```yaml
version: "1.0"
name: TodoAPI
imports:
  - "@typespec/http"

models:
  Todo:
    description: A todo item
    fields:
      id:
        type: string
        format: uuid
        decorators: ["@key"]
      title:
        type: string
        constraints:
          minLength: 1
          maxLength: 200
      completed:
        type: boolean
      priority:
        type: Priority
      createdAt:
        type: utcDateTime

unions:
  Priority:
    variants:
      - low
      - medium
      - high

raw: |
  @route("/todos")
  interface TodosApi {
    @get list(): Todo[];
    @get read(@path id: string): Todo;
    @post create(@body todo: Todo): Todo;
    @patch update(@path id: string, @body todo: Todo): Todo;
    @delete remove(@path id: string): void;
  }
```
