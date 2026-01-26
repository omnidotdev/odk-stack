//! Blog API Schema - Rust WASM Plugin
//!
//! This example demonstrates defining a blog API schema as an Extism WASM plugin.
//!
//! Build with: cargo build --target wasm32-unknown-unknown --release
//! Run with: odk compile target/wasm32-unknown-unknown/release/blog_plugin.wasm -o generated/

use extism_pdk::*;
use odk_pdk::*;

#[plugin_fn]
pub fn get_schema() -> FnResult<String> {
    let mut schema = CompiledSchema::new("BlogAPI");

    // Add imports
    schema.add_import("@typespec/http");
    schema.add_import("@omnidotdev/typespec-drizzle");

    // UserRole union
    schema.add_union(CompiledUnion {
        name: "UserRole".into(),
        variants: vec![
            UnionVariant::literal_string("admin"),
            UnionVariant::literal_string("author"),
            UnionVariant::literal_string("reader"),
        ],
        description: Some("User permission levels".into()),
        ..Default::default()
    });

    // PostStatus union
    schema.add_union(CompiledUnion {
        name: "PostStatus".into(),
        variants: vec![
            UnionVariant::literal_string("draft"),
            UnionVariant::literal_string("published"),
            UnionVariant::literal_string("archived"),
        ],
        description: Some("Publication status of a post".into()),
        ..Default::default()
    });

    // User model
    let mut user = CompiledModel::new("User");
    user.description = Some("A blog user account".into());
    user.with_decorator("@table");

    user.add_field(
        CompiledField::new("id", CompiledType::string())
            .with_format("uuid")
            .with_decorator("@id")
            .with_description("Unique user identifier"),
    );
    user.add_field(
        CompiledField::new("email", CompiledType::string())
            .with_format("email")
            .with_description("User's email address"),
    );
    user.add_field(
        CompiledField::new("username", CompiledType::string())
            .with_constraints(FieldConstraints {
                min_length: Some(3),
                max_length: Some(32),
                pattern: Some("^[a-zA-Z0-9_]+$".into()),
                ..Default::default()
            })
            .with_description("Unique username"),
    );
    user.add_field(
        CompiledField::new("displayName", CompiledType::string())
            .optional()
            .with_constraints(FieldConstraints {
                max_length: Some(100),
                ..Default::default()
            })
            .with_description("Display name shown on posts"),
    );
    user.add_field(
        CompiledField::new("role", CompiledType::reference("UserRole"))
            .with_description("User's permission level"),
    );
    user.add_field(
        CompiledField::new("createdAt", CompiledType::scalar("utcDateTime"))
            .with_decorator("@createdAt"),
    );
    user.add_field(
        CompiledField::new("updatedAt", CompiledType::scalar("utcDateTime"))
            .with_decorator("@updatedAt"),
    );

    schema.add_model(user);

    // Post model
    let mut post = CompiledModel::new("Post");
    post.description = Some("A blog post".into());
    post.with_decorator("@table");

    post.add_field(
        CompiledField::new("id", CompiledType::string())
            .with_format("uuid")
            .with_decorator("@id"),
    );
    post.add_field(
        CompiledField::new("title", CompiledType::string())
            .with_constraints(FieldConstraints {
                min_length: Some(1),
                max_length: Some(200),
                ..Default::default()
            })
            .with_description("Post title"),
    );
    post.add_field(
        CompiledField::new("slug", CompiledType::string())
            .with_constraints(FieldConstraints {
                pattern: Some("^[a-z0-9-]+$".into()),
                ..Default::default()
            })
            .with_description("URL-friendly slug"),
    );
    post.add_field(
        CompiledField::new("content", CompiledType::string())
            .with_description("Post content in Markdown"),
    );
    post.add_field(
        CompiledField::new("excerpt", CompiledType::string())
            .optional()
            .with_constraints(FieldConstraints {
                max_length: Some(500),
                ..Default::default()
            })
            .with_description("Short excerpt for previews"),
    );
    post.add_field(
        CompiledField::new("authorId", CompiledType::string())
            .with_format("uuid")
            .with_decorator("@manyToOne(User)"),
    );
    post.add_field(CompiledField::new(
        "status",
        CompiledType::reference("PostStatus"),
    ));
    post.add_field(
        CompiledField::new("publishedAt", CompiledType::scalar("utcDateTime")).optional(),
    );
    post.add_field(
        CompiledField::new("createdAt", CompiledType::scalar("utcDateTime"))
            .with_decorator("@createdAt"),
    );
    post.add_field(
        CompiledField::new("updatedAt", CompiledType::scalar("utcDateTime"))
            .with_decorator("@updatedAt"),
    );

    schema.add_model(post);

    // Comment model
    let mut comment = CompiledModel::new("Comment");
    comment.description = Some("A comment on a post".into());
    comment.with_decorator("@table");

    comment.add_field(
        CompiledField::new("id", CompiledType::string())
            .with_format("uuid")
            .with_decorator("@id"),
    );
    comment.add_field(
        CompiledField::new("content", CompiledType::string()).with_constraints(FieldConstraints {
            min_length: Some(1),
            max_length: Some(2000),
            ..Default::default()
        }),
    );
    comment.add_field(
        CompiledField::new("postId", CompiledType::string())
            .with_format("uuid")
            .with_decorator("@manyToOne(Post)"),
    );
    comment.add_field(
        CompiledField::new("authorId", CompiledType::string())
            .with_format("uuid")
            .with_decorator("@manyToOne(User)"),
    );
    comment.add_field(
        CompiledField::new("parentId", CompiledType::string())
            .with_format("uuid")
            .optional()
            .with_description("Parent comment ID for nested replies"),
    );
    comment.add_field(
        CompiledField::new("createdAt", CompiledType::scalar("utcDateTime"))
            .with_decorator("@createdAt"),
    );

    schema.add_model(comment);

    // Tag model
    let mut tag = CompiledModel::new("Tag");
    tag.description = Some("A tag for categorizing posts".into());
    tag.with_decorator("@table");

    tag.add_field(
        CompiledField::new("id", CompiledType::string())
            .with_format("uuid")
            .with_decorator("@id"),
    );
    tag.add_field(
        CompiledField::new("name", CompiledType::string()).with_constraints(FieldConstraints {
            min_length: Some(1),
            max_length: Some(50),
            ..Default::default()
        }),
    );
    tag.add_field(
        CompiledField::new("slug", CompiledType::string()).with_constraints(FieldConstraints {
            pattern: Some("^[a-z0-9-]+$".into()),
            ..Default::default()
        }),
    );

    schema.add_model(tag);

    // Add raw TypeSpec for API routes
    schema.add_raw(
        r#"
@route("/api")
@tag("Blog API")
namespace BlogApiRoutes {
    @route("/posts")
    interface Posts {
        @get list(@query status?: PostStatus): Post[];
        @get read(@path slug: string): Post;
        @post create(@body post: Post): Post;
        @put update(@path id: string, @body post: Post): Post;
        @delete remove(@path id: string): void;
    }

    @route("/users")
    interface Users {
        @get list(): User[];
        @get read(@path id: string): User;
        @post create(@body user: User): User;
    }

    @route("/posts/{postId}/comments")
    interface Comments {
        @get list(@path postId: string): Comment[];
        @post create(@path postId: string, @body comment: Comment): Comment;
    }
}
"#,
    );

    Ok(schema.to_json()?)
}
