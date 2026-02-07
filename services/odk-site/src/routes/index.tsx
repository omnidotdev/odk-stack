import { createFileRoute, Link } from "@tanstack/react-router";
import {
  ArrowRight,
  Database,
  Code,
  Terminal,
  Zap,
  Package,
  GitBranch,
  Layers,
} from "lucide-react";

import { Button } from "@/components/ui/button";
import createMetaTags from "@/lib/util/createMetaTags";

export const Route = createFileRoute("/")({
  head: () => ({
    meta: createMetaTags({
      title: "ODK - Omni Development Kit",
      description:
        "Transform your data schema into a complete API ecosystem - Postgres, REST, GraphQL, and SDKs in minutes.",
    }),
  }),
  component: HomePage,
});

/**
 * Home page / Landing.
 */
function HomePage() {
  return (
    <div className="flex min-h-[calc(100vh-4rem)] flex-col">
      {/* Hero Section */}
      <section className="relative flex flex-1 flex-col items-center justify-center gap-8 overflow-hidden px-4 py-16 text-center">
        {/* Background gradient blobs */}
        <div className="pointer-events-none absolute inset-0 -z-10 opacity-30">
          <div className="absolute right-0 top-0 h-96 w-96 rounded-full bg-primary/20 blur-3xl" />
          <div className="absolute bottom-0 left-0 h-96 w-96 rounded-full bg-secondary/20 blur-3xl" />
        </div>

        <div className="flex max-w-4xl flex-col items-center gap-6">
          <h1 className="text-4xl font-bold tracking-tight sm:text-5xl lg:text-6xl">
            From{" "}
            <span className="bg-gradient-to-r from-primary to-secondary bg-clip-text text-transparent">
              Schema
            </span>{" "}
            to{" "}
            <span className="bg-gradient-to-r from-secondary to-accent bg-clip-text text-transparent">
              APIs
            </span>{" "}
            in Minutes
          </h1>
          <p className="max-w-2xl text-xl text-muted-foreground">
            Omni Development Kit transforms your data schema into a complete API
            ecosystem—Postgres schema, RESTful API, GraphQL, and SDK generation,
            all in one seamless flow.
          </p>
        </div>

        <div className="flex gap-4">
          <Button size="lg" className="group">
            Get Started
            <ArrowRight className="ml-2 h-4 w-4 transition-transform group-hover:translate-x-1" />
          </Button>
          <Button size="lg" variant="outline" asChild>
            <Link to="/pricing">View Documentation</Link>
          </Button>
        </div>

        {/* Code Preview */}
        <div className="mt-8 w-full max-w-4xl overflow-hidden rounded-lg border shadow-2xl">
          <div className="flex items-center justify-between border-b bg-card px-4 py-3">
            <div className="flex space-x-2">
              <div className="h-3 w-3 rounded-full bg-destructive/70" />
              <div className="h-3 w-3 rounded-full bg-yellow-500/70" />
              <div className="h-3 w-3 rounded-full bg-green-500/70" />
            </div>
            <span className="text-sm text-muted-foreground">schema.ts</span>
            <div className="w-16" />
          </div>
          <pre className="overflow-x-auto bg-muted/50 p-4 text-left text-sm">
            <code className="text-foreground">{`// Define your data schema once
type User = {
  id: string;
  name: string;
  email: string;
  role: "admin" | "user";
  createdAt: Date;
};

type Post = {
  id: string;
  title: string;
  content: string;
  authorId: string; // Relation to User
  published: boolean;
};`}</code>
          </pre>
        </div>

        {/* Workflow Icons */}
        <div className="mt-12 flex flex-col gap-8 md:flex-row md:gap-16">
          <WorkflowStep
            icon={<Database className="h-8 w-8 text-primary" />}
            title="Postgres Schema"
            subtitle="via Drizzle"
          />
          <WorkflowStep
            icon={<Code className="h-8 w-8 text-secondary" />}
            title="REST & GraphQL"
            subtitle="OpenAPI compliant"
          />
          <WorkflowStep
            icon={<Terminal className="h-8 w-8 text-accent" />}
            title="SDKs & Plugins"
            subtitle="via Extism"
          />
        </div>
      </section>

      {/* Features Section */}
      <section className="bg-muted/50 px-4 py-16">
        <div className="mx-auto max-w-6xl">
          <h2 className="mb-12 text-center text-3xl font-bold">
            Everything you need to build APIs
          </h2>

          <div className="grid gap-8 md:grid-cols-2 lg:grid-cols-3">
            <FeatureCard
              icon={<Zap className="h-8 w-8" />}
              title="Schema-First Design"
              description="Define your data model once and generate everything else automatically."
            />
            <FeatureCard
              icon={<Layers className="h-8 w-8" />}
              title="Multi-Target Output"
              description="Generate Postgres schemas, REST APIs, GraphQL endpoints, and typed SDKs."
            />
            <FeatureCard
              icon={<Package className="h-8 w-8" />}
              title="Plugin System"
              description="Extend functionality with Extism-powered plugins in any language."
            />
            <FeatureCard
              icon={<GitBranch className="h-8 w-8" />}
              title="Version Control Ready"
              description="Schema changes tracked in git, with migration support built-in."
            />
            <FeatureCard
              icon={<Database className="h-8 w-8" />}
              title="Drizzle Integration"
              description="First-class support for Drizzle ORM with automatic schema generation."
            />
            <FeatureCard
              icon={<Code className="h-8 w-8" />}
              title="Type Safety"
              description="End-to-end TypeScript types from database to client SDK."
            />
          </div>
        </div>
      </section>
    </div>
  );
}

function WorkflowStep({
  icon,
  title,
  subtitle,
}: {
  icon: React.ReactNode;
  title: string;
  subtitle: string;
}) {
  return (
    <div className="flex flex-col items-center text-center">
      <div className="mb-3 rounded-full bg-card p-4 shadow-md">{icon}</div>
      <span className="font-medium">{title}</span>
      <span className="text-sm text-muted-foreground">{subtitle}</span>
    </div>
  );
}

function FeatureCard({
  icon,
  title,
  description,
}: {
  icon: React.ReactNode;
  title: string;
  description: string;
}) {
  return (
    <div className="rounded-lg border bg-card p-6 shadow-sm">
      <div className="mb-4 text-primary">{icon}</div>
      <h3 className="mb-2 text-xl font-semibold">{title}</h3>
      <p className="text-muted-foreground">{description}</p>
    </div>
  );
}
