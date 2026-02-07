import { createFileRoute } from "@tanstack/react-router";

import { useOrganization } from "@/lib/context";

export const Route = createFileRoute("/_auth/organizations/$orgSlug/members")({
  component: OrgMembersPage,
});

/**
 * Organization members page.
 */
function OrgMembersPage() {
  const { orgSlug } = Route.useParams();
  const { organizations } = useOrganization();

  const org = organizations.find((o) => o.slug === orgSlug);

  if (!org) return null;

  const isPersonal = org.type === "personal";

  return (
    <div className="container mx-auto py-8">
      <div className="mb-6 flex items-center justify-between">
        <h1 className="text-2xl font-bold">Members</h1>
        {!isPersonal && (
          <button
            type="button"
            className="rounded-lg bg-primary px-4 py-2 text-primary-foreground hover:bg-primary/90"
          >
            Invite Member
          </button>
        )}
      </div>

      {isPersonal && (
        <div className="mb-6 rounded-lg border border-muted bg-muted/50 p-4">
          <p className="text-sm text-muted-foreground">
            Personal organizations can only have one member (you).
          </p>
        </div>
      )}

      <div className="rounded-lg border">
        <table className="w-full">
          <thead>
            <tr className="border-b bg-muted/50">
              <th className="px-4 py-3 text-left text-sm font-medium">User</th>
              <th className="px-4 py-3 text-left text-sm font-medium">Role</th>
              <th className="px-4 py-3 text-left text-sm font-medium">Teams</th>
              {!isPersonal && (
                <th className="px-4 py-3 text-right text-sm font-medium">
                  Actions
                </th>
              )}
            </tr>
          </thead>
          <tbody>
            <tr className="border-b">
              <td className="px-4 py-3">
                <span className="font-medium">You</span>
              </td>
              <td className="px-4 py-3">
                <span className="rounded bg-primary/10 px-2 py-1 text-xs text-primary">
                  {org.roles[0] || "member"}
                </span>
              </td>
              <td className="px-4 py-3 text-sm text-muted-foreground">
                {org.teams.length > 0
                  ? org.teams.map((t) => t.name).join(", ")
                  : "—"}
              </td>
              {!isPersonal && <td className="px-4 py-3 text-right">—</td>}
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  );
}
