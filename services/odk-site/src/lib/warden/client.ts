import { AUTHZ_ENABLED, AUTHZ_API_URL } from "@/lib/config/env.config";

/**
 * Check if a user has permission on a resource.
 *
 * Returns true if authorized, false otherwise.
 * Returns true (permissive) when authz is disabled.
 */
export async function checkPermission(
  userId: string,
  resourceType: string,
  resourceId: string,
  permission: string,
): Promise<boolean> {
  // Permissive when disabled
  if (AUTHZ_ENABLED !== "true") return true;
  if (!AUTHZ_API_URL) return true;

  try {
    const response = await fetch(`${AUTHZ_API_URL}/check`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        user: `user:${userId}`,
        relation: permission,
        object: `${resourceType}:${resourceId}`,
      }),
    });

    if (!response.ok) {
      console.error(`AuthZ check failed: ${response.status}`);
      return false;
    }

    const result = (await response.json()) as { allowed: boolean };
    return result.allowed;
  } catch (error) {
    console.error("Authorization check failed:", error);
    // Fail-closed: deny access when PDP is unavailable
    return false;
  }
}

/**
 * Batch check permissions.
 * Returns an array of booleans corresponding to each check.
 */
export async function batchCheckPermissions(
  checks: Array<{
    userId: string;
    resourceType: string;
    resourceId: string;
    permission: string;
  }>,
): Promise<boolean[]> {
  // Permissive when disabled
  if (AUTHZ_ENABLED !== "true") return checks.map(() => true);
  if (!AUTHZ_API_URL) return checks.map(() => true);

  try {
    const response = await fetch(`${AUTHZ_API_URL}/check/batch`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        checks: checks.map((c) => ({
          user: `user:${c.userId}`,
          relation: c.permission,
          object: `${c.resourceType}:${c.resourceId}`,
        })),
      }),
    });

    if (!response.ok) {
      console.error(`AuthZ batch check failed: ${response.status}`);
      return checks.map(() => false);
    }

    const result = (await response.json()) as {
      results: Array<{ allowed: boolean }>;
    };
    return result.results.map((r) => r.allowed);
  } catch (error) {
    console.error("Batch authorization check failed:", error);
    // Fail-closed: deny access when PDP is unavailable
    return checks.map(() => false);
  }
}
