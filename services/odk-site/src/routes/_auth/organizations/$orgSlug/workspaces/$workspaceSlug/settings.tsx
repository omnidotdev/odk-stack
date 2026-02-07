import { createFileRoute } from "@tanstack/react-router";

import { useWorkspace } from "@/lib/context";

export const Route = createFileRoute(
  "/_auth/organizations/$orgSlug/workspaces/$workspaceSlug/settings",
)({
  component: WorkspaceSettingsPage,
});

/**
 * Workspace settings page.
 */
function WorkspaceSettingsPage() {
  const { workspaceSlug } = Route.useParams();
  const { workspaces } = useWorkspace();

  const workspace = workspaces.find((w) => w.slug === workspaceSlug);

  if (!workspace) return null;

  return (
    <div className="container mx-auto py-8">
      <h1 className="mb-6 text-2xl font-bold">Workspace Settings</h1>

      <div className="space-y-6">
        <section className="rounded-lg border p-6">
          <h2 className="mb-4 text-lg font-semibold">General</h2>
          <div className="space-y-4">
            <div>
              <label className="block text-sm font-medium">Workspace Name</label>
              <input
                type="text"
                defaultValue={workspace.name}
                className="mt-1 w-full rounded-md border px-3 py-2"
              />
            </div>
            <div>
              <label className="block text-sm font-medium">Slug</label>
              <input
                type="text"
                defaultValue={workspace.slug}
                className="mt-1 w-full rounded-md border px-3 py-2"
              />
              <p className="mt-1 text-xs text-muted-foreground">
                Used in URLs. Must be unique within the organization.
              </p>
            </div>
          </div>
        </section>

        <section className="rounded-lg border border-destructive/50 p-6">
          <h2 className="mb-4 text-lg font-semibold text-destructive">
            Danger Zone
          </h2>
          <p className="mb-4 text-sm text-muted-foreground">
            Permanently delete this workspace and all its data.
          </p>
          <button
            type="button"
            className="rounded-lg bg-destructive px-4 py-2 text-destructive-foreground hover:bg-destructive/90"
          >
            Delete Workspace
          </button>
        </section>
      </div>
    </div>
  );
}
