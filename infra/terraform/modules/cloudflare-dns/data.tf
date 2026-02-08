# =============================================================================
# Zone Information
# =============================================================================
# Query zone by name - returns zone ID, status, plan, etc.
# =============================================================================

data "cloudflare_zone" "this" {
  name = var.zone_name
}

# =============================================================================
# Note: DNS Records Query
# =============================================================================
# To list DNS records, use the Cloudflare API or Dashboard.
# The provider v4.x focuses on managing resources, not querying all records.
#
# You can manage individual DNS records using:
#   resource "cloudflare_record" "example" { ... }
#
# To import existing records, use cf-terraforming:
#   cf-terraforming generate --resource-type cloudflare_record --zone <zone_id>
# =============================================================================